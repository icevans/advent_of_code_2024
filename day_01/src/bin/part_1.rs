use day_01::{parse_location_ids, sum_of_differences};

fn main() {
    let (mut list_1, mut list_2) = parse_location_ids(include_str!("input.txt"));

    list_1.sort();
    list_2.sort();

    let sum = sum_of_differences(&list_1, &list_2);

    println!("{}", sum);
}
