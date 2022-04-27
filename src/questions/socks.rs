use std::collections::HashMap;

pub fn get_number_of_pairs(_number: i8, color_array: &[i8]) -> i8 {
    let mut hashmap = HashMap::new();
    for color in color_array {
        *hashmap.entry(color).or_insert(0) += 1;
    }
    let mut count = 0;
    for (&_key, &val) in hashmap.iter() {
        if val < 2 {
            continue;
        }
        count += val / 2;
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::get_number_of_pairs;

    #[test]
    fn test_number_of_pairs_1() {
        let input_array = [1, 2, 1, 2, 1, 3, 2];
        let amount = 7;
        assert_eq!(2, get_number_of_pairs(amount, &input_array));
    }

    #[test]
    fn test_number_of_pairs_2() {
        let input_array = [10, 20, 20, 10, 10, 30, 50, 10, 20];
        let amount = 9;
        assert_eq!(3, get_number_of_pairs(amount, &input_array));
    }

    #[test]
    fn test_number_of_pairs_3() {
        let input_array = [1, 1, 3, 1, 2, 1, 3, 3, 3, 3];
        let amount = 10;
        assert_eq!(4, get_number_of_pairs(amount, &input_array));
    }
}
