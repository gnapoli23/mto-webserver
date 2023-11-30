use std::collections::HashMap;

pub fn find_mto_numbers(values: Vec<u8>) -> Vec<u8> {
    let mut count_map = HashMap::new();

    // Count occurrences of each number
    values
        .iter()
        .for_each(|v| *count_map.entry(v).or_insert(0) += 1);

    // Find numbers that appear more than once
    let mut mto_values = count_map
        .into_iter()
        .filter_map(|(&num, count)| if count > 1 { Some(num) } else { None })
        .collect::<Vec<u8>>();

    // Sort the result in ascending order
    mto_values.sort();

    mto_values
}

#[cfg(test)]
mod utils_test {
    use crate::utils::find_mto_numbers;

    #[test]
    fn test_mto() {
        let values = vec![3, 2, 5, 1, 5, 7, 2, 1];
        let expected = vec![1, 2, 5];
        assert_eq!(find_mto_numbers(values), expected);

        let values = vec![5, 7, 7];
        let expected = vec![7];
        assert_eq!(find_mto_numbers(values), expected);
    }
}
