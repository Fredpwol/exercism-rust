// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_word_usage: HashMap<&str, u32> = HashMap::new();
    for word in magazine {
        magazine_word_usage
            .entry(word)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    for word in note {
        if !magazine_word_usage.contains_key(word)
            || *magazine_word_usage.get(word).unwrap() == 0 as u32
        {
            return false;
        }
        magazine_word_usage.entry(word).and_modify(|e| *e -= 1);
    }
    return true;
}
