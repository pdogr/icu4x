// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod fixtures;

use criterion::{
    black_box, criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
};
use icu_calendar::{Calendar, Date};

fn bench_calendar<C: Copy + Calendar>(
    group: &mut BenchmarkGroup<WallTime>,
    name: &str,
    calendar: C,
) {
    let iso = Date::try_new_iso_date(1970, 1, 2).unwrap();
    group.bench_function(name, |b| {
        b.iter(|| {
            let converted = black_box(iso).to_calendar(calendar);
            let year = black_box(converted.year().number);
            let month = black_box(converted.month().ordinal);
            let day = black_box(converted.day_of_month().0);
            black_box((year, month, day))
        })
    });
}

fn convert_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("convert");

    bench_calendar(&mut group, "calendar/iso", icu::calendar::iso::Iso);

    #[cfg(feature = "bench")]
    bench_calendar(
        &mut group,
        "calendar/buddhist",
        icu::calendar::buddhist::Buddhist,
    );

    #[cfg(feature = "bench")]
    bench_calendar(&mut group, "calendar/coptic", icu::calendar::coptic::Coptic);

    #[cfg(feature = "bench")]
    bench_calendar(
        &mut group,
        "calendar/ethiopic",
        icu::calendar::ethiopian::Ethiopian::new(),
    );

    #[cfg(feature = "bench")]
    bench_calendar(&mut group, "calendar/indian", icu::calendar::indian::Indian);

    #[cfg(feature = "bench")]
    bench_calendar(&mut group, "calendar/julian", icu::calendar::julian::Julian);

    #[cfg(feature = "bench")]
    bench_calendar(
        &mut group,
        "calendar/chinese",
        icu::calendar::chinese::Chinese,
    );

    #[cfg(feature = "bench")]
    bench_calendar(
        &mut group,
        "calendar/gregorian",
        icu::calendar::gregorian::Gregorian,
    );

    group.finish();
}

criterion_group!(benches, convert_benches);
criterion_main!(benches);
