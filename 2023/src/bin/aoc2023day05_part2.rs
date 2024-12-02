use derive_more::{From, Into};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{fmt, ops::Range, str, str::FromStr};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, From, Into)]
pub struct SeedId(pub u64);
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, From, Into)]
pub struct SoilId(pub u64);
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, From, Into)]
pub struct FertilizerId(pub u64);
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, From, Into)]
pub struct WaterId(pub u64);
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, From, Into)]
pub struct LightId(pub u64);
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, From, Into)]
pub struct TemperatureId(pub u64);
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, From, Into)]
pub struct HumidityId(pub u64);
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, From, Into)]
pub struct LocationId(pub u64);

fn main() {
    let input = include_str!("aoc2023day05_input.txt");
    let mut lines = input.lines();

    let seeds_data: Vec<_> = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .map(str::trim)
        .unwrap()
        .split(" ")
        .map(str::trim)
        .map(|seed_id| u64::from_str(seed_id).unwrap())
        .collect();

    println!("seeds_data: {:?}", seeds_data);
    let seed_ranges: Vec<_> = seeds_data
        .chunks(2)
        .map(|val| Range {
            start: SeedId(val[0]),
            end: SeedId(val[0] + val[1]),
        })
        .collect();

    println!("{:?}", seed_ranges);

    assert_eq!(Some(""), lines.next());

    assert_eq!(Some("seed-to-soil map:"), lines.next());
    let seed_to_soil_ranges = parse_ranges::<SeedId, SoilId>(&mut lines);

    assert_eq!(Some("soil-to-fertilizer map:"), lines.next());
    let soil_to_fertilizer_ranges = parse_ranges::<SoilId, FertilizerId>(&mut lines);

    assert_eq!(Some("fertilizer-to-water map:"), lines.next());
    let fertilizer_to_water_ranges = parse_ranges::<FertilizerId, WaterId>(&mut lines);

    assert_eq!(Some("water-to-light map:"), lines.next());
    let water_to_light_ranges = parse_ranges::<WaterId, LightId>(&mut lines);

    assert_eq!(Some("light-to-temperature map:"), lines.next());
    let light_to_temperature_ranges = parse_ranges::<LightId, TemperatureId>(&mut lines);

    assert_eq!(Some("temperature-to-humidity map:"), lines.next());
    let temperature_to_humidity_ranges = parse_ranges::<TemperatureId, HumidityId>(&mut lines);

    assert_eq!(Some("humidity-to-location map:"), lines.next());
    let humidity_to_location_ranges = parse_ranges::<HumidityId, LocationId>(&mut lines);

    let seed_count = seed_ranges
        .iter()
        .flat_map(|range| Range {
            start: range.start.0,
            end: range.end.0,
        })
        .count();
    println!("seed_count: {}", seed_count);

    let bar = ProgressBar::new(seed_count as u64);

    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] [estimate: {duration_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} [{percent}%] {msg}")
    .unwrap()
    .progress_chars("##-"));

    let result = seed_ranges
        .par_iter()
        .flat_map(|range| Range {
            start: range.start.0,
            end: range.end.0,
        })
        .map(|seed_id| {
            let seed_id = SeedId(seed_id);
            let soil_id = find_id(&seed_id, &seed_to_soil_ranges);
            let fertilizer_id = find_id(&soil_id, &soil_to_fertilizer_ranges);
            let water_id = find_id(&fertilizer_id, &fertilizer_to_water_ranges);
            let light_id = find_id(&water_id, &water_to_light_ranges);
            let temperature_id = find_id(&light_id, &light_to_temperature_ranges);
            let humidity_id = find_id(&temperature_id, &temperature_to_humidity_ranges);
            let location_id = find_id(&humidity_id, &humidity_to_location_ranges);

            Some((location_id, seed_id))
        })
        .reduce(
            || Option::<(LocationId, SeedId)>::None,
            |acc, item| {
                bar.inc(1);

                if acc.is_none() {
                    item
                } else {
                    let acc = acc.unwrap();
                    let item = item.unwrap();
                    if acc.0 .0 < item.0 .0 {
                        Some(acc)
                    } else {
                        Some(item)
                    }
                }
            },
        );

    // single threaded
    // let result = seed_ranges
    //     .iter()
    //     .flat_map(|range| Range {
    //         start: range.start.0,
    //         end: range.end.0,
    //     })
    //     .map(|seed_id| {
    //         let seed_id = SeedId(seed_id);
    //         let soil_id = find_id(&seed_id, &seed_to_soil_ranges);
    //         let fertilizer_id = find_id(&soil_id, &soil_to_fertilizer_ranges);
    //         let water_id = find_id(&fertilizer_id, &fertilizer_to_water_ranges);
    //         let light_id = find_id(&water_id, &water_to_light_ranges);
    //         let temperature_id = find_id(&light_id, &light_to_temperature_ranges);
    //         let humidity_id = find_id(&temperature_id, &temperature_to_humidity_ranges);
    //         let location_id = find_id(&humidity_id, &humidity_to_location_ranges);

    //         Some((location_id, seed_id))
    //     })
    //     .fold(
    //         Option::<(LocationId, SeedId)>::None,
    //         |acc, item| {
    //             bar.inc(1);

    //             if acc.is_none() {
    //                 item
    //             } else {
    //                 let other = acc.unwrap();
    //                 let item = item.unwrap();
    //                 if other.0 .0 < item.0 .0 {
    //                     Some(other)
    //                 } else {
    //                     Some(item)
    //                 }
    //             }
    //         },
    //     );

    bar.finish();

    println!("result: {:?}", result);
}

fn find_id<
    K: Copy + Into<u64> + From<u64> + PartialOrd + PartialEq + fmt::Debug,
    V: Copy + Into<u64> + From<u64> + fmt::Debug,
>(
    key_id: &K,
    ranges: &[(Range<K>, Range<V>)],
) -> V {
    let mut mapped_id = None;
    let key_id_raw: u64 = (*key_id).into();
    for (k_range, v_range) in ranges.iter() {
        if k_range.contains(&key_id) {
            let offset = key_id_raw - k_range.start.into();
            mapped_id = Some(V::from(v_range.start.into() + offset));
        }
    }
    if mapped_id.is_none() {
        mapped_id = Some(V::from(key_id_raw))
    }
    let id = mapped_id.unwrap();
    id
}

fn parse_ranges<K: From<u64> + Ord, V: From<u64> + Ord>(
    lines: &mut str::Lines<'_>,
) -> Vec<(Range<K>, Range<V>)> {
    let mut ranges = Vec::new();
    loop {
        let line = lines.next().unwrap_or("");
        if "" == line.trim() {
            break;
        }
        let (k, v) = map_ranges::<K, V>(line);
        ranges.push((k, v));
        ranges.sort_by(|a, b| a.1.start.cmp(&b.1.start));
    }
    ranges
}

fn map_ranges<K: From<u64>, V: From<u64>>(line: &str) -> (Range<K>, Range<V>) {
    let mut components = line.split(" ");
    let b = u64::from_str(components.next().unwrap()).unwrap();
    let a = u64::from_str(components.next().unwrap()).unwrap();
    let len = u64::from_str(components.next().unwrap()).unwrap();

    let k = Range {
        start: K::from(a),
        end: K::from(a + len),
    };

    let v = Range {
        start: V::from(b),
        end: V::from(b + len),
    };

    (k, v)
}
