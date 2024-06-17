use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::{Arc, Mutex, MutexGuard};
use rand::distributions::Standard;

use rand::Rng;
use rand::rngs::OsRng;
use crate::game::Game;

pub async fn get_unique_client_id<T>(ids: &Arc<Mutex<HashSet<T>>>, rng: &Arc<Mutex<OsRng>>) -> T
    where
        T: Eq + Hash + Clone,
        Standard: rand::distributions::Distribution<T>,
{

    let mut rng = rng.lock().expect("Failed to lock RNG");
    let mut set = ids.lock().expect("Failed to lock ID set");
    let mut random_value: T;

    loop {
        random_value = rng.gen();
        if !set.contains(&random_value) {
            break;
        }
    }

    set.insert(random_value.clone());
    random_value
}

pub async fn get_unique_game_id(game_ids: &Arc<Mutex<HashMap<u128, Game>>>, rng: Arc<Mutex<OsRng>>) -> u128
    // where
    //     T: Eq + Hash + Clone,
    //     Standard: rand::distributions::Distribution<u128>,
{

    let mut rng = rng.lock().expect("Failed to lock RNG");
    let mut map = game_ids.lock().expect("Failed to lock ID set");
    let mut random_value: u128;

    loop {
        random_value = rng.gen();
        if !map.contains_key(&random_value) {
            break;
        }
    }

    let big_blind = 2;
    let initial_money = 1000;
    let game: Game = Game::new(random_value.clone(), big_blind, initial_money);

    map.insert(random_value.clone(), game);
    random_value
}

// pub async fn get_unique_key<T>(ids: &MutexGuard<HashMap<u128>>, mut rng: MutexGuard<OsRng>) -> T
//     where
//         T: Eq + Hash + Clone,
//         Standard: rand::distributions::Distribution<T>,
// {
//
//     // let mut rng = rng.lock().expect("Failed to lock RNG");
//     // let mut set = ids.lock().expect("Failed to lock ID set");
//     let mut random_value: T;
//
//     loop {
//         random_value = rng.gen();
//         if !ids.contains(&random_value) {
//             break;
//         }
//     }
//
//     // ids.insert(random_value.clone());
//     random_value
// }

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
pub fn format_next_to_each_other<Outer, Inner>(strings_to_print: Outer) -> String
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
