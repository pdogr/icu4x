// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Korean Dangi calendar.
//!
//! ```rust
//! use icu::calendar::{Date, DateTime};
//!
//! // `Date` type
//! let dangi_date = Date::try_new_dangi_date(4356, 6, 6)
//!     .expect("Failed to initialize Dangi Date instance.");
//!
//! // `DateTime` type
//! let dangi_datetime = DateTime::try_new_dangi_datetime(4356, 6, 6, 13, 1, 0)
//!     .expect("Failed to initialize Dangi DateTime instance.");
//!
//! // `Date` checks
//! assert_eq!(dangi_date.year().number, 4356);
//! assert_eq!(dangi_date.year().related_iso, Some(2023));
//! assert_eq!(dangi_date.year().cyclic, Some(40));
//! assert_eq!(dangi_date.month().ordinal, 6);
//! assert_eq!(dangi_date.day_of_month().0, 6);
//!
//! // `DateTime` checks
//! assert_eq!(dangi_datetime.date.year().number, 4356);
//! assert_eq!(dangi_datetime.date.year().related_iso, Some(2023));
//! assert_eq!(dangi_datetime.date.year().cyclic, Some(40));
//! assert_eq!(dangi_datetime.date.month().ordinal, 6);
//! assert_eq!(dangi_datetime.date.day_of_month().0, 6);
//! assert_eq!(dangi_datetime.time.hour.number(), 13);
//! assert_eq!(dangi_datetime.time.minute.number(), 1);
//! assert_eq!(dangi_datetime.time.second.number(), 0);
//! ```

use crate::calendar_arithmetic::CalendarArithmetic;
use crate::chinese_based::chinese_based_ordinal_lunar_month_from_code;
use crate::helpers::div_rem_euclid64;
use crate::{
    astronomy::Location,
    calendar_arithmetic::ArithmeticDate,
    chinese_based::{ChineseBased, ChineseBasedDateInner},
    rata_die::RataDie,
    types::{self, Era, FormattableYear},
    AnyCalendarKind, Calendar, CalendarError, Date, DateTime, Iso,
};
use tinystr::tinystr;

// The first day in the Korean Dangi calendar (based on the founding of Gojoseon)
const KOREAN_EPOCH: RataDie = RataDie::new(-852065); // Lunar new year 2333 BCE (-2332 ISO)

/// The Korean Dangi calendar relies on knowing the current day at the moment of a new moon;
/// however, this can vary depending on location. As such, new moon calculations are based on
/// the time in Seoul. Before 1908, local time was used, represented as UTC+(3809/450 h).
/// This changed multiple times as different standard timezones were adopted in Korea.
/// Currently, UTC+9h is used.
///
/// Offsets are not given in hours, but in partial days (1 hour = 1 / 24 day).
const UTC_OFFSET_ORIGINAL: f64 = (3809.0 / 450.0) / 24.0;
const UTC_OFFSET_1908: f64 = 8.5 / 24.0;
const UTC_OFFSET_1912: f64 = 9.0 / 24.0;
const UTC_OFFSET_1954: f64 = 8.5 / 24.0;
const UTC_OFFSET_1961: f64 = 9.0 / 24.0;

const FIXED_1908: RataDie = RataDie::new(696608); // Apr 1, 1908
const FIXED_1912: RataDie = RataDie::new(697978); // Jan 1, 1912
const FIXED_1954: RataDie = RataDie::new(713398); // Mar 21, 1954
const FIXED_1961: RataDie = RataDie::new(716097); // Aug 10, 1961

const KOREAN_LATITUDE: f64 = 37.0 + (34.0 / 60.0);
const KOREAN_LONGITUDE: f64 = 126.0 + (58.0 / 60.0);
const KOREAN_ELEVATION: f64 = 0.0;

const KOREAN_LOCATION_ORIGINAL: Location = Location::new_unchecked(
    KOREAN_LATITUDE,
    KOREAN_LONGITUDE,
    KOREAN_ELEVATION,
    UTC_OFFSET_ORIGINAL,
);
const KOREAN_LOCATION_1908: Location = Location::new_unchecked(
    KOREAN_LATITUDE,
    KOREAN_LONGITUDE,
    KOREAN_ELEVATION,
    UTC_OFFSET_1908,
);
const KOREAN_LOCATION_1912: Location = Location::new_unchecked(
    KOREAN_LATITUDE,
    KOREAN_LONGITUDE,
    KOREAN_ELEVATION,
    UTC_OFFSET_1912,
);
const KOREAN_LOCATION_1954: Location = Location::new_unchecked(
    KOREAN_LATITUDE,
    KOREAN_LONGITUDE,
    KOREAN_ELEVATION,
    UTC_OFFSET_1954,
);
const KOREAN_LOCATION_1961: Location = Location::new_unchecked(
    KOREAN_LATITUDE,
    KOREAN_LONGITUDE,
    KOREAN_ELEVATION,
    UTC_OFFSET_1961,
);

/// The Dangi Calendar
///
/// The Dangi Calendar is a lunisolar calendar used traditionally in North and South Korea.
/// It is often used today to track important cultural events and holidays like Seollal
/// (Korean lunar new year). It is similar to the Chinese lunar calendar (see `Chinese`),
/// except that observations are based in Korea (currently UTC+9) rather than China (UTC+8).
/// This can cause some differences; for example, 2012 was a leap year, but in the Dangi
/// calendar the leap month was 3, while in the Chinese calendar the leap month was 4.
///
/// ```rust
/// use icu::calendar::{Date, dangi::Dangi, chinese::Chinese};
/// use tinystr::tinystr;
///
/// let iso_a = Date::try_new_iso_date(2012, 4, 23).unwrap();
/// let dangi_a = iso_a.to_calendar(Dangi);
/// let chinese_a = iso_a.to_calendar(Chinese);
///
/// assert_eq!(dangi_a.month().code.0, tinystr!(4, "M03L"));
/// assert_eq!(chinese_a.month().code.0, tinystr!(4, "M04"));
///
/// let iso_b = Date::try_new_iso_date(2012, 5, 23).unwrap();
/// let dangi_b = iso_b.to_calendar(Dangi);
/// let chinese_b = iso_b.to_calendar(Chinese);
///
/// assert_eq!(dangi_b.month().code.0, tinystr!(4, "M04"));
/// assert_eq!(chinese_b.month().code.0, tinystr!(4, "M04L"));
/// ```
/// # Era codes
///
/// This Calendar supports a single era code "dangi" based on the year -2332 ISO (2333 BCE) as year 1.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct Dangi;

/// The inner date type used for representing [`Date`]s of [`Dangi`]. See [`Date`] and [`Dangi`] for more detail.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct DangiDateInner(ChineseBasedDateInner<Dangi>);

type Inner = ChineseBasedDateInner<Dangi>;

impl Calendar for Dangi {
    type DateInner = DangiDateInner;

    fn date_from_codes(
        &self,
        era: crate::types::Era,
        year: i32,
        month_code: crate::types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, crate::Error> {
        let month = if let Some(ordinal) =
            chinese_based_ordinal_lunar_month_from_code::<Dangi>(year, month_code)
        {
            ordinal
        } else {
            return Err(CalendarError::UnknownMonthCode(
                month_code.0,
                self.debug_name(),
            ));
        };

        if month > Self::months_for_every_year(year) {
            return Err(CalendarError::UnknownMonthCode(
                month_code.0,
                self.debug_name(),
            ));
        }

        let max_day = Self::month_days(year, month);
        if day > max_day {
            return Err(CalendarError::Overflow {
                field: "day",
                max: max_day as usize,
            });
        }

        if era.0 != tinystr!(16, "dangi") {
            return Err(CalendarError::UnknownEra(era.0, self.debug_name()));
        }

        Ok(ArithmeticDate::new_unchecked(year, month, day))
            .map(ChineseBasedDateInner)
            .map(DangiDateInner)
    }

    fn date_from_iso(&self, iso: Date<crate::Iso>) -> Self::DateInner {
        let fixed = Iso::fixed_from_iso(iso.inner);
        DangiDateInner(Inner::chinese_based_date_from_fixed(fixed))
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<crate::Iso> {
        let fixed = Inner::fixed_from_chinese_based_date_inner(date.0);
        Iso::iso_from_fixed(fixed)
    }

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        Self::months_for_every_year(date.0 .0.year)
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u16 {
        Self::days_in_provided_year(date.0 .0.year)
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        Self::month_days(date.0 .0.year, date.0 .0.month)
    }

    fn offset_date(&self, date: &mut Self::DateInner, offset: crate::DateDuration<Self>) {
        date.0 .0.offset_date(offset)
    }

    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        _calendar2: &Self,
        largest_unit: crate::DateDurationUnit,
        smallest_unit: crate::DateDurationUnit,
    ) -> crate::DateDuration<Self> {
        date1.0 .0.until(date2.0 .0, largest_unit, smallest_unit)
    }

    fn debug_name(&self) -> &'static str {
        "Dangi"
    }

    fn year(&self, date: &Self::DateInner) -> crate::types::FormattableYear {
        Self::format_dangi_year(date.0 .0.year)
    }

    fn month(&self, date: &Self::DateInner) -> crate::types::FormattableMonth {
        let ordinal = date.0 .0.month;
        let leap_month = if Self::is_leap_year(date.0 .0.year) {
            Inner::get_leap_month_in_year(Inner::fixed_mid_year_from_year(date.0 .0.year))
        } else {
            14
        };
        let code_inner = if leap_month == ordinal {
            // Month cannot be 1 because a year cannot have a leap month before the first actual month,
            // and the maximum num of months ina leap year is 13.
            debug_assert!((2..=13).contains(&ordinal));
            match ordinal {
                2 => tinystr!(4, "M01L"),
                3 => tinystr!(4, "M02L"),
                4 => tinystr!(4, "M03L"),
                5 => tinystr!(4, "M04L"),
                6 => tinystr!(4, "M05L"),
                7 => tinystr!(4, "M06L"),
                8 => tinystr!(4, "M07L"),
                9 => tinystr!(4, "M08L"),
                10 => tinystr!(4, "M09L"),
                11 => tinystr!(4, "M10L"),
                12 => tinystr!(4, "M11L"),
                13 => tinystr!(4, "M12L"),
                _ => tinystr!(4, "und"),
            }
        } else {
            let mut adjusted_ordinal = ordinal;
            if ordinal > leap_month {
                // Before adjusting for leap month, if ordinal > leap_month,
                // the month cannot be 1 because this implies the leap month is < 1, which is impossible;
                // cannot be 2 because that implies the leap month is = 1, which is impossible,
                // and cannot be more than 13 because max number of months in a year is 13.
                debug_assert!((2..=13).contains(&ordinal));
                adjusted_ordinal -= 1;
            }
            debug_assert!((1..=12).contains(&adjusted_ordinal));
            match adjusted_ordinal {
                1 => tinystr!(4, "M01"),
                2 => tinystr!(4, "M02"),
                3 => tinystr!(4, "M03"),
                4 => tinystr!(4, "M04"),
                5 => tinystr!(4, "M05"),
                6 => tinystr!(4, "M06"),
                7 => tinystr!(4, "M07"),
                8 => tinystr!(4, "M08"),
                9 => tinystr!(4, "M09"),
                10 => tinystr!(4, "M10"),
                11 => tinystr!(4, "M11"),
                12 => tinystr!(4, "M12"),
                _ => tinystr!(4, "und"),
            }
        };
        let code = types::MonthCode(code_inner);
        types::FormattableMonth {
            ordinal: ordinal as u32,
            code,
        }
    }

    fn day_of_month(&self, date: &Self::DateInner) -> crate::types::DayOfMonth {
        types::DayOfMonth(date.0 .0.day as u32)
    }

    fn day_of_year_info(&self, date: &Self::DateInner) -> crate::types::DayOfYearInfo {
        let prev_year = date.0 .0.year.saturating_sub(1);
        let next_year = date.0 .0.year.saturating_add(1);
        types::DayOfYearInfo {
            day_of_year: date.0 .0.day_of_year(),
            days_in_year: date.0 .0.days_in_year(),
            prev_year: Self::format_dangi_year(prev_year),
            days_in_prev_year: Self::days_in_provided_year(prev_year),
            next_year: Self::format_dangi_year(next_year),
        }
    }

    fn day_of_week(&self, date: &Self::DateInner) -> crate::types::IsoWeekday {
        self.date_to_iso(date).day_of_week()
    }

    fn any_calendar_kind(&self) -> Option<crate::AnyCalendarKind> {
        Some(AnyCalendarKind::Dangi)
    }
}

impl Date<Dangi> {
    /// Construct a new Dangi date from a `year`, `month`, and `day`.
    /// `year` represents the Chinese year counted infinitely with -2332 (2333 BCE) as year 1;
    /// `month` represents the month of the year ordinally (ex. if it is a leap year, the last month will be 13, not 12);
    /// `day` indicates day of month.
    ///
    /// ```rust
    /// use icu::calendar::Date;
    ///
    /// let date_dangi = Date::try_new_dangi_date(4356, 6, 18)
    ///     .expect("Failed to initialize Dangi Date instance.");
    ///
    /// assert_eq!(date_dangi.year().number, 4356);
    /// assert_eq!(date_dangi.year().cyclic, Some(40));
    /// assert_eq!(date_dangi.year().related_iso, Some(2023));
    /// assert_eq!(date_dangi.month().ordinal, 6);
    /// assert_eq!(date_dangi.day_of_month().0, 18);
    /// ```
    pub fn try_new_dangi_date(year: i32, month: u8, day: u8) -> Result<Date<Dangi>, CalendarError> {
        ArithmeticDate::new_from_lunar_ordinals(year, month, day)
            .map(ChineseBasedDateInner)
            .map(DangiDateInner)
            .map(|inner| Date::from_raw(inner, Dangi))
    }
}

impl DateTime<Dangi> {
    /// Construct a new Dangi DateTime from integers. See `try_new_dangi_date`.
    ///
    /// ```rust
    /// use icu::calendar::DateTime;
    ///
    /// let dangi_datetime = DateTime::try_new_dangi_datetime(4356, 6, 6, 13, 1, 0)
    ///     .expect("Failed to initialize Dangi DateTime instance.");
    ///
    /// assert_eq!(dangi_datetime.date.year().number, 4356);
    /// assert_eq!(dangi_datetime.date.year().related_iso, Some(2023));
    /// assert_eq!(dangi_datetime.date.year().cyclic, Some(40));
    /// assert_eq!(dangi_datetime.date.month().ordinal, 6);
    /// assert_eq!(dangi_datetime.date.day_of_month().0, 6);
    /// assert_eq!(dangi_datetime.time.hour.number(), 13);
    /// assert_eq!(dangi_datetime.time.minute.number(), 1);
    /// assert_eq!(dangi_datetime.time.second.number(), 0);
    /// ```
    pub fn try_new_dangi_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<Dangi>, CalendarError> {
        Ok(DateTime {
            date: Date::try_new_dangi_date(year, month, day)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }
}

impl ChineseBased for Dangi {
    fn location(fixed: RataDie) -> Location {
        if fixed < FIXED_1908 {
            KOREAN_LOCATION_ORIGINAL
        } else if fixed < FIXED_1912 {
            KOREAN_LOCATION_1908
        } else if fixed < FIXED_1954 {
            KOREAN_LOCATION_1912
        } else if fixed < FIXED_1961 {
            KOREAN_LOCATION_1954
        } else {
            KOREAN_LOCATION_1961
        }
    }

    const EPOCH: RataDie = KOREAN_EPOCH;

    // Unchecked can be used since this function is only ever called when generating dates from
    // a valid year, month, and day.
    fn new_chinese_based_date(year: i32, month: u8, day: u8) -> ChineseBasedDateInner<Dangi> {
        ChineseBasedDateInner(ArithmeticDate::new_unchecked(year, month, day))
    }
}

impl Dangi {
    /// Get the ISO Date of Seollal (Korean Lunar New Year) on or before the given ISO Date `iso`.
    ///
    /// ```rust
    /// use icu::calendar::Date;
    /// use icu::calendar::dangi::Dangi;
    ///
    /// let iso = Date::try_new_iso_date(2023, 6, 18)
    ///     .expect("Failed to initialize ISO Date instance.");
    /// let seollal = Dangi::seollal_on_or_before_iso(iso);
    ///
    /// assert_eq!(seollal.year().number, 2023);
    /// assert_eq!(seollal.month().ordinal, 1);
    /// assert_eq!(seollal.day_of_month().0, 22);
    /// ```
    pub fn seollal_on_or_before_iso(iso: Date<Iso>) -> Date<Iso> {
        let iso_inner = iso.inner;
        let fixed = Iso::fixed_from_iso(iso_inner);
        let result_fixed = Inner::new_year_on_or_before_fixed_date(fixed, None).0;
        Iso::iso_from_fixed(result_fixed)
    }

    fn format_dangi_year(year: i32) -> FormattableYear {
        let era = Era(tinystr!(16, "dangi"));
        let number = year;
        let cyclic = Some(div_rem_euclid64(number as i64 - 1 + 364, 60).1 as i32 + 1);
        let mid_year = Inner::fixed_mid_year_from_year(number);
        let iso_formattable_year = Iso::iso_from_fixed(mid_year).year();
        let related_iso = Some(iso_formattable_year.number);
        types::FormattableYear {
            era,
            number,
            cyclic,
            related_iso,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::chinese::Chinese;

    #[test]
    fn test_seollal() {
        #[derive(Debug)]
        struct TestCase {
            iso_year: i32,
            iso_month: u8,
            iso_day: u8,
            expected_year: i32,
            expected_month: u32,
            expected_day: u32,
        }

        let cases = [
            TestCase {
                iso_year: 2024,
                iso_month: 6,
                iso_day: 6,
                expected_year: 2024,
                expected_month: 2,
                expected_day: 10,
            },
            TestCase {
                iso_year: 2024,
                iso_month: 2,
                iso_day: 9,
                expected_year: 2023,
                expected_month: 1,
                expected_day: 22,
            },
            TestCase {
                iso_year: 2023,
                iso_month: 1,
                iso_day: 22,
                expected_year: 2023,
                expected_month: 1,
                expected_day: 22,
            },
            TestCase {
                iso_year: 2023,
                iso_month: 1,
                iso_day: 21,
                expected_year: 2022,
                expected_month: 2,
                expected_day: 1,
            },
            TestCase {
                iso_year: 2022,
                iso_month: 6,
                iso_day: 6,
                expected_year: 2022,
                expected_month: 2,
                expected_day: 1,
            },
            TestCase {
                iso_year: 2021,
                iso_month: 6,
                iso_day: 6,
                expected_year: 2021,
                expected_month: 2,
                expected_day: 12,
            },
            TestCase {
                iso_year: 2020,
                iso_month: 6,
                iso_day: 6,
                expected_year: 2020,
                expected_month: 1,
                expected_day: 25,
            },
            TestCase {
                iso_year: 2019,
                iso_month: 6,
                iso_day: 6,
                expected_year: 2019,
                expected_month: 2,
                expected_day: 5,
            },
            TestCase {
                iso_year: 2018,
                iso_month: 6,
                iso_day: 6,
                expected_year: 2018,
                expected_month: 2,
                expected_day: 16,
            },
            TestCase {
                iso_year: 2025,
                iso_month: 6,
                iso_day: 6,
                expected_year: 2025,
                expected_month: 1,
                expected_day: 29,
            },
            TestCase {
                iso_year: 2026,
                iso_month: 8,
                iso_day: 8,
                expected_year: 2026,
                expected_month: 2,
                expected_day: 17,
            },
            TestCase {
                iso_year: 2027,
                iso_month: 4,
                iso_day: 4,
                expected_year: 2027,
                expected_month: 2,
                expected_day: 7,
            },
            TestCase {
                iso_year: 2028,
                iso_month: 9,
                iso_day: 21,
                expected_year: 2028,
                expected_month: 1,
                expected_day: 27,
            },
        ];

        for case in cases {
            let iso = Date::try_new_iso_date(case.iso_year, case.iso_month, case.iso_day).unwrap();
            let seollal = Dangi::seollal_on_or_before_iso(iso);
            assert_eq!(
                seollal.year().number,
                case.expected_year,
                "Year check failed for case: {case:?}"
            );
            assert_eq!(
                seollal.month().ordinal,
                case.expected_month,
                "Month check failed for case: {case:?}"
            );
            assert_eq!(
                seollal.day_of_month().0,
                case.expected_day,
                "Day check failed for case: {case:?}"
            );
        }
    }

    fn check_cyclic_and_rel_iso(year: i32) {
        let iso = Date::try_new_iso_date(year, 6, 6).unwrap();
        let chinese = iso.to_calendar(Chinese);
        let dangi = iso.to_calendar(Dangi);
        let chinese_year = chinese.year().cyclic;
        let korean_year = dangi.year().cyclic;
        assert_eq!(
            chinese_year, korean_year,
            "Cyclic year failed for year: {year}"
        );
        let chinese_rel_iso = chinese.year().related_iso;
        let korean_rel_iso = dangi.year().related_iso;
        assert_eq!(
            chinese_rel_iso, korean_rel_iso,
            "Rel. ISO year equality failed for year: {year}"
        );
        assert_eq!(korean_rel_iso, Some(year), "Dangi Rel. ISO failed!");
    }

    #[test]
    fn test_cyclic_same_as_chinese_near_present_day() {
        for year in 1923..=2123 {
            check_cyclic_and_rel_iso(year);
        }
    }

    #[test]
    fn test_cyclic_same_as_chinese_near_rd_zero() {
        for year in -100..=100 {
            check_cyclic_and_rel_iso(year);
        }
    }

    #[test]
    fn test_fixed_chinese_roundtrip() {
        let mut fixed = -1963020;
        let max_fixed = 1963020;
        let mut iters = 0;
        let max_iters = 560;
        while fixed < max_fixed && iters < max_iters {
            let rata_die = RataDie::new(fixed);
            let chinese = Inner::chinese_based_date_from_fixed(rata_die);
            let result = Inner::fixed_from_chinese_based_date_inner(chinese);
            let result_debug = result.to_i64_date();
            assert_eq!(result, rata_die, "Failed roundtrip fixed -> Chinese -> fixed for fixed: {fixed}, with calculated: {result_debug} from Chinese date:\n{chinese:?}");
            fixed += 7043;
            iters += 1;
        }
    }

    #[test]
    fn test_iso_to_dangi_roundtrip() {
        let mut fixed = -1963020;
        let max_fixed = 1963020;
        let mut iters = 0;
        let max_iters = 560;
        while fixed < max_fixed && iters < max_iters {
            let rata_die = RataDie::new(fixed);
            let iso = Iso::iso_from_fixed(rata_die);
            let korean = iso.to_calendar(Dangi);
            let result = korean.to_calendar(Iso);
            assert_eq!(
                iso, result,
                "Failed roundtrip ISO -> Dangi -> ISO for fixed: {fixed}"
            );
            fixed += 7043;
            iters += 1;
        }
    }

    #[test]
    fn test_dangi_consistent_with_icu() {
        // Test cases for this test are derived from existing ICU Intl.DateTimeFormat. If there is a bug in ICU,
        // these test cases may be affected, and this calendar's output may not be entirely valid.

        // There are a number of test cases which do not match ICU for dates very far in the past or future,
        // see #3709.

        #[derive(Debug)]
        struct TestCase {
            iso_year: i32,
            iso_month: u8,
            iso_day: u8,
            expected_rel_iso: i32,
            expected_cyclic: i32,
            expected_month: u32,
            expected_day: u32,
        }

        let cases = [
            TestCase {
                // #3709: This test case fails to match ICU
                iso_year: 4321,
                iso_month: 1,
                iso_day: 23,
                expected_rel_iso: 4320,
                expected_cyclic: 57,
                expected_month: 13,
                expected_day: 12,
            },
            TestCase {
                iso_year: 3649,
                iso_month: 9,
                iso_day: 20,
                expected_rel_iso: 3649,
                expected_cyclic: 46,
                expected_month: 9,
                expected_day: 1,
            },
            TestCase {
                iso_year: 3333,
                iso_month: 3,
                iso_day: 3,
                expected_rel_iso: 3333,
                expected_cyclic: 30,
                expected_month: 1,
                expected_day: 25,
            },
            TestCase {
                iso_year: 3000,
                iso_month: 3,
                iso_day: 30,
                expected_rel_iso: 3000,
                expected_cyclic: 57,
                expected_month: 3,
                expected_day: 3,
            },
            TestCase {
                iso_year: 2772,
                iso_month: 7,
                iso_day: 27,
                expected_rel_iso: 2772,
                expected_cyclic: 9,
                expected_month: 7,
                expected_day: 5,
            },
            TestCase {
                iso_year: 2525,
                iso_month: 2,
                iso_day: 25,
                expected_rel_iso: 2525,
                expected_cyclic: 2,
                expected_month: 2,
                expected_day: 3,
            },
            TestCase {
                iso_year: 2345,
                iso_month: 3,
                iso_day: 21,
                expected_rel_iso: 2345,
                expected_cyclic: 2,
                expected_month: 2,
                expected_day: 17,
            },
            TestCase {
                iso_year: 2222,
                iso_month: 2,
                iso_day: 22,
                expected_rel_iso: 2222,
                expected_cyclic: 59,
                expected_month: 1,
                expected_day: 11,
            },
            TestCase {
                iso_year: 2167,
                iso_month: 6,
                iso_day: 22,
                expected_rel_iso: 2167,
                expected_cyclic: 4,
                expected_month: 5,
                expected_day: 6,
            },
            TestCase {
                iso_year: 2121,
                iso_month: 2,
                iso_day: 12,
                expected_rel_iso: 2120,
                expected_cyclic: 17,
                expected_month: 13,
                expected_day: 25,
            },
            TestCase {
                iso_year: 2080,
                iso_month: 12,
                iso_day: 31,
                expected_rel_iso: 2080,
                expected_cyclic: 37,
                expected_month: 12,
                expected_day: 21,
            },
            TestCase {
                iso_year: 2030,
                iso_month: 3,
                iso_day: 20,
                expected_rel_iso: 2030,
                expected_cyclic: 47,
                expected_month: 2,
                expected_day: 17,
            },
            TestCase {
                iso_year: 2027,
                iso_month: 2,
                iso_day: 7,
                expected_rel_iso: 2027,
                expected_cyclic: 44,
                expected_month: 1,
                expected_day: 1,
            },
            TestCase {
                iso_year: 2023,
                iso_month: 7,
                iso_day: 1,
                expected_rel_iso: 2023,
                expected_cyclic: 40,
                expected_month: 6,
                expected_day: 14,
            },
            TestCase {
                iso_year: 2022,
                iso_month: 3,
                iso_day: 1,
                expected_rel_iso: 2022,
                expected_cyclic: 39,
                expected_month: 1,
                expected_day: 29,
            },
            TestCase {
                iso_year: 2021,
                iso_month: 2,
                iso_day: 1,
                expected_rel_iso: 2020,
                expected_cyclic: 37,
                expected_month: 13,
                expected_day: 20,
            },
            TestCase {
                iso_year: 2016,
                iso_month: 3,
                iso_day: 30,
                expected_rel_iso: 2016,
                expected_cyclic: 33,
                expected_month: 2,
                expected_day: 22,
            },
            TestCase {
                iso_year: 2016,
                iso_month: 7,
                iso_day: 30,
                expected_rel_iso: 2016,
                expected_cyclic: 33,
                expected_month: 6,
                expected_day: 27,
            },
            TestCase {
                iso_year: 2015,
                iso_month: 9,
                iso_day: 22,
                expected_rel_iso: 2015,
                expected_cyclic: 32,
                expected_month: 8,
                expected_day: 10,
            },
            TestCase {
                iso_year: 2013,
                iso_month: 10,
                iso_day: 1,
                expected_rel_iso: 2013,
                expected_cyclic: 30,
                expected_month: 8,
                expected_day: 27,
            },
            TestCase {
                iso_year: 2010,
                iso_month: 2,
                iso_day: 1,
                expected_rel_iso: 2009,
                expected_cyclic: 26,
                expected_month: 13,
                expected_day: 18,
            },
            TestCase {
                iso_year: 2000,
                iso_month: 8,
                iso_day: 30,
                expected_rel_iso: 2000,
                expected_cyclic: 17,
                expected_month: 8,
                expected_day: 2,
            },
            TestCase {
                iso_year: 1990,
                iso_month: 11,
                iso_day: 11,
                expected_rel_iso: 1990,
                expected_cyclic: 7,
                expected_month: 10,
                expected_day: 24,
            },
            TestCase {
                iso_year: 1970,
                iso_month: 6,
                iso_day: 10,
                expected_rel_iso: 1970,
                expected_cyclic: 47,
                expected_month: 5,
                expected_day: 7,
            },
            TestCase {
                iso_year: 1970,
                iso_month: 1,
                iso_day: 1,
                expected_rel_iso: 1969,
                expected_cyclic: 46,
                expected_month: 11,
                expected_day: 24,
            },
            TestCase {
                iso_year: 1941,
                iso_month: 12,
                iso_day: 7,
                expected_rel_iso: 1941,
                expected_cyclic: 18,
                expected_month: 11,
                expected_day: 19,
            },
            TestCase {
                iso_year: 1812,
                iso_month: 5,
                iso_day: 4,
                expected_rel_iso: 1812,
                expected_cyclic: 9,
                expected_month: 3,
                expected_day: 24,
            },
            TestCase {
                iso_year: 1655,
                iso_month: 6,
                iso_day: 15,
                expected_rel_iso: 1655,
                expected_cyclic: 32,
                expected_month: 5,
                expected_day: 12,
            },
            TestCase {
                iso_year: 1333,
                iso_month: 3,
                iso_day: 10,
                expected_rel_iso: 1333,
                expected_cyclic: 10,
                expected_month: 2,
                expected_day: 16,
            },
            TestCase {
                iso_year: 1000,
                iso_month: 10,
                iso_day: 10,
                expected_rel_iso: 1000,
                expected_cyclic: 37,
                expected_month: 9,
                expected_day: 5,
            },
            TestCase {
                iso_year: 842,
                iso_month: 2,
                iso_day: 15,
                expected_rel_iso: 841,
                expected_cyclic: 58,
                expected_month: 13,
                expected_day: 28,
            },
            TestCase {
                iso_year: 101,
                iso_month: 1,
                iso_day: 10,
                expected_rel_iso: 100,
                expected_cyclic: 37,
                expected_month: 12,
                expected_day: 24,
            },
            TestCase {
                iso_year: -1,
                iso_month: 3,
                iso_day: 28,
                expected_rel_iso: -1,
                expected_cyclic: 56,
                expected_month: 2,
                expected_day: 25,
            },
            TestCase {
                iso_year: -3,
                iso_month: 2,
                iso_day: 28,
                expected_rel_iso: -3,
                expected_cyclic: 54,
                expected_month: 2,
                expected_day: 5,
            },
            TestCase {
                iso_year: -365,
                iso_month: 7,
                iso_day: 24,
                expected_rel_iso: -365,
                expected_cyclic: 52,
                expected_month: 6,
                expected_day: 24,
            },
            TestCase {
                iso_year: -999,
                iso_month: 9,
                iso_day: 9,
                expected_rel_iso: -999,
                expected_cyclic: 18,
                expected_month: 7,
                expected_day: 27,
            },
            TestCase {
                iso_year: -1500,
                iso_month: 1,
                iso_day: 5,
                expected_rel_iso: -1501,
                expected_cyclic: 56,
                expected_month: 12,
                expected_day: 2,
            },
            TestCase {
                iso_year: -2332,
                iso_month: 3,
                iso_day: 1,
                expected_rel_iso: -2332,
                expected_cyclic: 5,
                expected_month: 1,
                expected_day: 16,
            },
            TestCase {
                iso_year: -2332,
                iso_month: 2,
                iso_day: 15,
                expected_rel_iso: -2332,
                expected_cyclic: 5,
                expected_month: 1,
                expected_day: 1,
            },
            TestCase {
                // #3709: This test case fails to match ICU
                iso_year: -2332,
                iso_month: 2,
                iso_day: 14,
                expected_rel_iso: -2333,
                expected_cyclic: 4,
                expected_month: 13,
                expected_day: 30,
            },
            TestCase {
                // #3709: This test case fails to match ICU
                iso_year: -2332,
                iso_month: 1,
                iso_day: 17,
                expected_rel_iso: -2333,
                expected_cyclic: 4,
                expected_month: 13,
                expected_day: 2,
            },
            TestCase {
                // #3709: This test case fails to match ICU
                iso_year: -2332,
                iso_month: 1,
                iso_day: 16,
                expected_rel_iso: -2333,
                expected_cyclic: 4,
                expected_month: 13,
                expected_day: 1,
            },
            TestCase {
                iso_year: -2332,
                iso_month: 1,
                iso_day: 15,
                expected_rel_iso: -2333,
                expected_cyclic: 4,
                expected_month: 12,
                expected_day: 29,
            },
            TestCase {
                iso_year: -2332,
                iso_month: 1,
                iso_day: 1,
                expected_rel_iso: -2333,
                expected_cyclic: 4,
                expected_month: 12,
                expected_day: 15,
            },
            TestCase {
                iso_year: -2333,
                iso_month: 1,
                iso_day: 16,
                expected_rel_iso: -2334,
                expected_cyclic: 3,
                expected_month: 12,
                expected_day: 19,
            },
            TestCase {
                iso_year: -2333,
                iso_month: 1,
                iso_day: 27,
                expected_rel_iso: -2333,
                expected_cyclic: 4,
                expected_month: 1,
                expected_day: 1,
            },
            TestCase {
                iso_year: -2333,
                iso_month: 1,
                iso_day: 26,
                expected_rel_iso: -2334,
                expected_cyclic: 3,
                expected_month: 12,
                expected_day: 29,
            },
            TestCase {
                iso_year: -2600,
                iso_month: 9,
                iso_day: 16,
                expected_rel_iso: -2600,
                expected_cyclic: 37,
                expected_month: 8,
                expected_day: 16,
            },
            TestCase {
                iso_year: -2855,
                iso_month: 2,
                iso_day: 3,
                expected_rel_iso: -2856,
                expected_cyclic: 21,
                expected_month: 12,
                expected_day: 30,
            },
            TestCase {
                // #3709: This test case fails to match ICU
                iso_year: -3000,
                iso_month: 5,
                iso_day: 15,
                expected_rel_iso: -3000,
                expected_cyclic: 57,
                expected_month: 4,
                expected_day: 1,
            },
            TestCase {
                // #3709: This test case fails to match ICU
                iso_year: -3649,
                iso_month: 9,
                iso_day: 20,
                expected_rel_iso: -3649,
                expected_cyclic: 8,
                expected_month: 8,
                expected_day: 10,
            },
            TestCase {
                // #3709: This test case fails to match ICU
                iso_year: -3649,
                iso_month: 3,
                iso_day: 30,
                expected_rel_iso: -3649,
                expected_cyclic: 8,
                expected_month: 2,
                expected_day: 14,
            },
            TestCase {
                // #3709: This test case fails to match ICU
                iso_year: -3650,
                iso_month: 3,
                iso_day: 30,
                expected_rel_iso: -3650,
                expected_cyclic: 7,
                expected_month: 3,
                expected_day: 3,
            },
        ];

        for case in cases {
            let iso = Date::try_new_iso_date(case.iso_year, case.iso_month, case.iso_day).unwrap();
            let dangi = iso.to_calendar(Dangi);
            let dangi_rel_iso = dangi.year().related_iso;
            let dangi_cyclic = dangi.year().cyclic;
            let dangi_month = dangi.month().ordinal;
            let dangi_day = dangi.day_of_month().0;

            assert_eq!(
                dangi_rel_iso,
                Some(case.expected_rel_iso),
                "Related ISO failed for test case: {case:?}"
            );
            assert_eq!(
                dangi_cyclic,
                Some(case.expected_cyclic),
                "Cyclic year failed for test case: {case:?}"
            );
            assert_eq!(
                dangi_month, case.expected_month,
                "Month failed for test case: {case:?}"
            );
            assert_eq!(
                dangi_day, case.expected_day,
                "Day failed for test case: {case:?}"
            );
        }
    }
}
