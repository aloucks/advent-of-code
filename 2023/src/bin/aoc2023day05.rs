use derive_more::{Add, AddAssign, From, Into, Sub, SubAssign};
use std::{
    error::Error,
    fmt,
    io::{BufRead, BufReader, Cursor},
    ops::Range,
    str,
    str::FromStr,
};

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    From,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Into,
)]
pub struct SeedId(pub u64);
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    From,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Into,
)]
pub struct SoilId(pub u64);
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    From,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Into,
)]
pub struct FertilizerId(pub u64);
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    From,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Into,
)]
pub struct WaterId(pub u64);
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    From,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Into,
)]
pub struct LightId(pub u64);
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    From,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Into,
)]
pub struct TemperatureId(pub u64);
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    From,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Into,
)]
pub struct HumidityId(pub u64);
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    From,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Into,
)]
pub struct LocationId(pub u64);

fn main() {
    let input = include_str!("aoc2023day05_input.txt");
    let mut lines = input.lines();

    let seeds: Vec<_> = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .map(str::trim)
        .unwrap()
        .split(" ")
        .map(str::trim)
        .map(|seed_id| SeedId(u64::from_str(seed_id).unwrap()))
        .collect();

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

    let mut locations = Vec::new();

    for seed_id in seeds.iter() {
        let soil_id = find_id(seed_id, &seed_to_soil_ranges);
        let fertilizer_id = find_id(&soil_id, &soil_to_fertilizer_ranges);
        let water_id = find_id(&fertilizer_id, &fertilizer_to_water_ranges);
        let light_id = find_id(&water_id, &water_to_light_ranges);
        let temperature_id = find_id(&light_id, &light_to_temperature_ranges);
        let humidity_id = find_id(&temperature_id, &temperature_to_humidity_ranges);
        let location_id = find_id(&humidity_id, &humidity_to_location_ranges);

        locations.push((location_id, seed_id));
        println!();
    }

    locations.sort_by(|a, b| a.0.cmp(&b.0));

    println!("{:?}", locations[0]);
    println!();
    println!("answer: {:?}", locations[0].0 .0);
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
    println!("{:?}, {:?}", key_id, id);
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
        ranges.sort_by(|a, b| a.0.start.cmp(&b.0.start));
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
