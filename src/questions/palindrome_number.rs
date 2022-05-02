struct Solution;

impl Solution {
    /// Using only number structs
    /// Runtime: 12 ms
    /// Memory Usage: 2 MB
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let xx: i64 = x as i64;
        let mut devider: i64 = 10;
        let mut vec_numbers: Vec<i64> = Vec::new();
        loop {
            let value_on_index = xx % devider / (devider / 10);
            if xx / devider > 0 {
                vec_numbers.push(value_on_index);
                if devider < i32::MAX as i64 {
                    devider *= 10;
                }
            } else {
                vec_numbers.push(value_on_index);
                break;
            }
        }

        for i in 0..(vec_numbers.len() / 2) {
            let left_side = vec_numbers[i];
            let right_side = vec_numbers[vec_numbers.len() - i - 1];
            if left_side != right_side {
                return false;
            }
        }
        true
    }

    /// Runtime: 8 ms
    /// Memory Usage: 2.1 MB
    pub fn is_palindrome_string_version(x: i32) -> bool {
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

    #[test]
    fn test_example_2() {
        let input = 10;
        assert_eq!(false, Solution::is_palindrome(input));
    }

    #[test]
    fn test_example_3() {
        let input = 1234567899;
        assert_eq!(false, Solution::is_palindrome(input));
    }
}
