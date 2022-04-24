use std::collections::{HashSet, HashMap};

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();
    let word_hashmap = get_word_count_hashmap(&word);
    for possible_anagram in possible_anagrams{
        if *possible_anagram.to_lowercase().to_string() == word.to_lowercase().to_string() {continue};
        let pos_anagram_hashmap = get_word_count_hashmap(possible_anagram);
        if pos_anagram_hashmap == word_hashmap{
            anagrams.insert(*possible_anagram);
        } 
    }
    anagrams
}


fn get_word_count_hashmap<'a>(word: &'a str) -> HashMap<String, i32>{
    let mut word_hashmap: HashMap<String, i32> = HashMap::new();
    for chr in word.chars(){
        let val = chr.clone().to_lowercase().to_string();
        let  count = &word_hashmap.get(&val).unwrap_or(&0).clone();
        word_hashmap.insert(val, count + 1);
    }
    return word_hashmap;
}
