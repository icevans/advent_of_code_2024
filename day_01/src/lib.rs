use std::collections::HashMap;
use std::hash::Hash;

/// Parses two lists of location IDs from a single input string. Input
/// is expected to be a series of lines, where each line is a location
/// ID (parseable as a u32), followed by some number of spaces, and then
/// another location ID.
///
/// # Examples
/// ```
/// use day_01::parse_location_id_lists;
///
/// let lists = parse_location_id_lists("111   123\n345   789");
/// assert_eq!(lists, (vec![111, 345], vec![123, 789]));
pub fn parse_location_ids(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut list_1 = vec![];
    let mut list_2 = vec![];

    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        list_1.push(
            parts[0]
                .parse()
                .expect("expected something parseable as u32"),
        );

        list_2.push(
            parts[1]
                .parse()
                .expect("list 2: expected something parseable as u32"),
        );
    });

    (list_1, list_2)
}

/// Counts number of times each element of a appears in b and returns
/// this information as a HashMap
///
/// # Example
/// ```
/// use day_01::element_count;
///
/// let count = element_count(&["a", "b"], &["a", "a", "b", "b", "b", "c"]);
/// assert_eq!(count.get(&"a"), Some(&2_u32));
/// assert_eq!(count.get(&"b"), Some(&3_u32));
/// ```
pub fn element_count<'a, T: Eq + Hash>(a: &'a [T], b: &'a [T]) -> HashMap<&'a T, u32> {
    let mut count = HashMap::new();

    for element in a {
        count.insert(element, 0);
    }

    for element in b {
        if count.contains_key(element) {
            count.entry(element).and_modify(|v| *v += 1);
        }
    }

    count
}

/// Calculates the difference of each pair of elements from two lists of numbers
/// and returns the sum.
///
/// # Examples
///
/// ```
/// use day_01::sum_of_differences;
///
/// let sum = sum_of_differences(&[0, 2], &[3, 4]);
/// assert_eq!(sum, 5);
/// ```
pub fn sum_of_differences(list_1: &[u32], list_2: &[u32]) -> u32 {
    list_1
        .into_iter()
        .zip(list_2)
        .fold(0, |sum: u32, items: (&u32, &u32)| {
            let difference = items.0.abs_diff(*items.1);
            sum + difference
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
