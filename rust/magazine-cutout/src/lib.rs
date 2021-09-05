// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::hash_map::OccupiedEntry;
use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map: HashMap<&str, u32> = HashMap::new();

    for word in magazine {
        *map.entry(word).or_insert(0) += 1;
    }
    for word in note {
        match map.entry(word) {
            Occupied(entry) => {
                if *entry.get() > 0 {
                    *entry.into_mut() -= 1;
                } else {
                    return false;
                }
            }
            Vacant(_) => return false,
        }
    }
    return true;
}
