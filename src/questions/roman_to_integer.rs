/// Roman to integers solution
/// 1 <= s.length <= 15
// 's' contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
// It is guaranteed that s is a valid roman numeral in the range [1, 3999].
struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut number = 0;
        let mut last_num = 0;

        let mut is_prenumber_loaded = false;
        for c in s.chars() {
            let current_num = match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0, // panic in production code
            };

            match current_num {
                1 | 10 | 100 => {
                    if is_prenumber_loaded {
                        if current_num > last_num {
                            // suibstracting smaller number from current bigger number
                            number += current_num - last_num;
                            is_prenumber_loaded = false;
                        } else if current_num == last_num {
                            // last number is not used for substracting, add both current and last
                            number += last_num + current_num;
                            is_prenumber_loaded = false;
                        } else {
                            // current_num < last_num
                            // (current might be used for substracting in the next iteration)
                            number += last_num;
                            last_num = current_num;
                            is_prenumber_loaded = true;
                        }
                    } else {
                        // this number might be used for substracting in the next iteration
                        last_num = current_num;
                        is_prenumber_loaded = true;
                    }
                }
                _ => {
                    if is_prenumber_loaded {
                        if current_num > last_num {
                            // substracting smaller number from current bigger number
                            number += current_num - last_num;
                        } else {
                            // last number is not used for substracting, add both current and last
                            number += current_num + last_num;
                        }
                        is_prenumber_loaded = false;
                    } else {
                        number += current_num;
                    }
                }
            }
        }
        // handle number ending with potentialy substracting value
        if is_prenumber_loaded {
            number += last_num;
        }
        number
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_example_1() {
        let input = String::from("III");
        let expected_result: i32 = 3;
        assert_eq!(expected_result, Solution::roman_to_int(input));
    }

    #[test]
    fn test_example_2() {
        let input = String::from("LVIII");
        let expected_result: i32 = 58;
        assert_eq!(expected_result, Solution::roman_to_int(input));
    }

    #[test]
    fn test_example_3() {
        let input = String::from("MCMXCIV");
        let expected_result: i32 = 1994;
        assert_eq!(expected_result, Solution::roman_to_int(input));
    }

    #[test]
    fn test_example_4() {
        let input = String::from("MDCCCLXXXIV");
        let expected_result: i32 = 1884;
        assert_eq!(expected_result, Solution::roman_to_int(input));
    }
}
