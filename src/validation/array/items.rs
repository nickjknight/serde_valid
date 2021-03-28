/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#length>
pub fn validate_array_items<T>(
    value: &[T],
    min_items: Option<usize>,
    max_items: Option<usize>,
) -> bool {
    let len = value.len();
    if let Some(max) = max_items {
        if max < len {
            return false;
        }
    }

    if let Some(min) = min_items {
        if len < min {
            return false;
        };
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_array_vec_type() {
        assert!(validate_array_items(&vec!['a', 'b', 'c'], Some(3), Some(3)));
    }

    #[test]
    fn test_validate_array_items_array_type() {
        assert!(validate_array_items(&['a', 'b', 'c'], Some(3), Some(3)));
    }

    #[test]
    fn test_validate_array_items_min_is_true() {
        assert!(validate_array_items(&[1, 2, 3], Some(3), None));
    }

    #[test]
    fn test_validate_array_items_min_is_false() {
        assert!(!validate_array_items(&[1, 2, 3], Some(4), None));
    }

    #[test]
    fn test_validate_array_items_max_is_true() {
        assert!(validate_array_items(&[1, 2, 3], None, Some(3)));
    }

    #[test]
    fn test_validate_array_items_max_is_false() {
        assert!(!validate_array_items(&[1, 2, 3], None, Some(2)));
    }
}
