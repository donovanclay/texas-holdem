use rand::Rng;
use std::collections::HashSet;

pub fn get_unique_id(ids: &HashSet<i32>) -> i32 {
    let mut rng = rand::thread_rng();
    let mut id = rng.gen_range(1..1000000);

    while ids.contains(&id) {
        id = rng.gen_range(1..1000000);
    }

    id
}
