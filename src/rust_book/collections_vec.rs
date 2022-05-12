/// Returns middle value from vector,
/// if even lenght of vector returns average of two values in the middle
pub fn get_median(int_list: &mut Vec<i32>) -> f32 {
    int_list.sort();
    if let Some(x) = int_list.get(int_list.len() / 2) {
        if int_list.len() % 2 != 0 {
            return *x as f32;
        } else {
            if let Some(y) = int_list.get(int_list.len() / 2 + 1) {
                return (*x + *y) as f32 / 2.0;
            }
        }
    }
    panic!("Incorrect vector values.");
}

#[cfg(test)]
mod tests {
    use super::get_median;

    #[test]
    fn test_get_median_even() {
        let result = 6.5;
        let mut input_vec = [9, 8, 7, 6, 5, 4, 3, 2].to_vec();
        assert_eq!(result, get_median(&mut input_vec));
    }

    #[test]
    fn test_get_median_odd() {
        let result = 6.0;
        let mut input_vec = [9, 8, 7, 6, 5, 4, 3].to_vec();
        assert_eq!(result, get_median(&mut input_vec));
    }
}
