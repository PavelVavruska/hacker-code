struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let string_x: String = x.to_string();
        for i in 0..(string_x.len() / 2) {
            if string_x.chars().nth(i).unwrap()
                != string_x.chars().nth(string_x.len() - i - 1).unwrap()
            {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_example_1() {
        let input = 1221;
        assert_eq!(true, Solution::is_palindrome(input));
    }
}
