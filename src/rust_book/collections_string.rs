/// Converts strings to pig latin.
/// The first consonant of each word is moved to the end of the word and “ay” is added,
/// so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added
/// to the end instead (“apple” becomes “apple-hay”).
pub fn get_pig_latin(input_str: String) -> String {
    let mut ending = String::new();
    let mut start = String::new();
    for (index, char) in input_str.chars().enumerate() {
        if index == 0 {
            ending = match char {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    start.push(char);
                    String::from("-hay")
                }
                other => format!("-{}ay", other),
            }
        } else {
            start.push(char);
        }
    }
    format!("{}{}", start, ending)
}

#[cfg(test)]
mod tests {
    use super::get_pig_latin;

    #[test]
    fn test_get_consonant() {
        let input = String::from("first");
        let result = String::from("irst-fay");

        assert_eq!(result, get_pig_latin(input));
    }

    #[test]
    fn test_get_vowel() {
        let input = String::from("apple");
        let result = String::from("apple-hay");

        assert_eq!(result, get_pig_latin(input));
    }
}
