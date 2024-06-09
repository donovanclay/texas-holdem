use std::cmp::max;
use std::collections::HashSet;

use rand::Rng;

/// Generates a unique ID that is not already present in the provided set of IDs.
///
/// # Parameters
///
/// * `ids`: A reference to a `HashSet` of `i32` values. This set contains the IDs that are already in use.
///
/// # Returns
///
/// An `i32` value that is not present in `ids`. This ID is randomly generated and is in the range from 1 to 999999 (inclusive).
///
/// # Examples
///
/// ```
/// let mut ids = HashSet::new();
/// ids.insert(123456);
/// let new_id = get_unique_id(&ids);
/// assert!(!ids.contains(&new_id));
/// ```
pub fn get_unique_id(ids: &HashSet<i32>) -> i32 {
    let mut rng = rand::thread_rng();
    let mut id = rng.gen_range(1..1000000);

    while ids.contains(&id) {
        id = rng.gen_range(1..1000000);
    }

    id
}

pub fn dashes(num: usize) -> String {
    "-".repeat(num).to_string()
}

/// Returns a string of dashes ("-") with the length equal to the length of the longest string in the provided collection.
///
/// # Parameters
///
/// * `strings`: An iterable collection of strings. The strings can be of any type that implements `AsRef<str>`,
///   such as `String` or `&str`.
///
/// # Returns
///
/// A `String` composed of dashes ("-"). The length of this string is equal to the length of the longest string in `strings`.
///
/// # Panics
///
/// This function will panic if `strings` is empty.
///
/// # Examples
///
/// ```
/// let strings = vec!["short", "medium length", "very long string"];
/// let dashes = get_dashes_for_longest_string(strings);
/// assert_eq!(dashes, "---------------"); // 16 dashes
/// ```
pub fn get_dashes_for_longest_string<I, S>(strings: I) -> String
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
{
    "-".repeat(
        strings
            .into_iter()
            .max_by_key(|x| x.as_ref().len())
            .expect("Error here")
            .as_ref()
            .len(),
    )
}



/// Formats a collection of strings into a single string where each string from the inner collection
/// is printed next to each other, separated by a divider.
///
/// # Parameters
///
/// * `strings_to_print`: An iterable collection of iterable collections of strings. Each inner collection
///   represents a column of strings to be printed next to each other. The outer collection represents
///   the rows of strings.
///
/// # Returns
///
/// A `String` where each string from the inner collections is printed next to each other, separated by a divider.
/// Each inner collection is printed on a new line. If an inner collection is shorter than others, its missing
/// elements are replaced with whitespace.
///
/// # Panics
///
/// This function will panic if `strings_to_print` is empty or if any of the inner collections are empty.
///
/// # Examples
///
/// ```
/// let strings = vec![
///     vec!["Hello", "World"],
///     vec!["Foo", "Bar"]
/// ];
///
/// let result = print_next_to_each_other(strings);
/// assert_eq!(result, "Hello | Foo\nWorld | Bar\n");
/// ```
pub fn print_next_to_each_other<Outer, Inner>(strings_to_print: Outer) -> String
    where
        Outer: IntoIterator<Item = Inner>,
        Inner: IntoIterator<Item = String>
{
    let mut output = String::new();
    let gap = 1;
    let divider = format!("{}{}{}", " ".repeat(gap), "|",  " ".repeat(gap));

    let strings_to_print: Vec<Vec<_>> = strings_to_print.into_iter().map(|x| x.into_iter().collect()).collect();

    let tallest_string_length = strings_to_print
        .iter()
        .map(|x| {x.len()})
        .max()
        .expect("No strings were passed");

    let longest_string_length = strings_to_print
        .iter()
        .map(|x| {
            x.iter().max_by_key(|x| {x.len()}).expect("Something went wrong here").len()
        })
        .collect::<Vec<_>>();

    for line in 0..tallest_string_length {
        let line_str = strings_to_print.iter()
            .enumerate()
            .map(
                |(index, string)| {
                    let whitespace_length = *longest_string_length.get(index).expect("Outside index bounds");
                    let whitespace = " ".repeat(whitespace_length);

                    return (string.get(line).unwrap_or(&whitespace).clone(), whitespace_length);
                }
            )
            .map(|(string, whitespace_length)| {
                let padding_length = whitespace_length.checked_sub(string.len()).unwrap_or(0);
                let padding = " ".repeat(max(0, padding_length));
                let local = format!("{}{}", string, padding);
                local
            })
            .collect::<Vec<_>>()
            .join(&divider);

        output += &(line_str + "\n");
    }

    output
}
