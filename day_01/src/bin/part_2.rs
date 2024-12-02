use day_01::{element_count, parse_location_ids};

fn main() {
    let (location_ids_a, location_ids_b) = parse_location_ids(include_str!("input.txt"));

    let frequencies = element_count(&location_ids_a, &location_ids_b);
    let similarity = location_ids_a.iter().fold(0, |sum, location_id| {
        sum + location_id * frequencies.get(&location_id).unwrap()
    });

    println!("{}", similarity);
}
