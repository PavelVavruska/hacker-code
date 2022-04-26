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

#[test]
fn test_number_of_pairs() {
    let input_array = [1, 2, 1, 2, 1, 3, 2];
    let amount = 7;
    assert_eq!(2, get_number_of_pairs(amount, &input_array));
}
