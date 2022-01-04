pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}


// pub fn reverse_w_unicode_chars(input: &str) -> String {
//     use unicode_segmentation::UnicodeSegmentation;

//     let mut reversed_string: String = String::new();
//     for i in (0..input.len()).rev(){
//         let char_at = input.graphemes(true).nth(i);
//         if let Some(chr) = char_at {
//             reversed_string.push_str(chr);   
//         }
//     }
//     return reversed_string;
// }
