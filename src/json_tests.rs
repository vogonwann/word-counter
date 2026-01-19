#[cfg(test)]
mod json_tests {
    use crate::*;
    use serde_json::Value;

    #[test]
    fn json_output_is_valid_and_has_expected_shape() {
        // Arrange
        let input = "Rust je super, rust je BRZ!";
        let (total, items) = top_words(input, 3, 1);
        let output = to_output_struct(total, items);

        // Act
        let json = serde_json::to_string(&output).expect("JSON serialization failed");
        let v: Value = serde_json::from_str(&json).expect("JSON is not valid");

        // Assert (shape)
        assert!(v.is_object());
        assert!(v.get("total_words").is_some());
        assert!(v.get("top_words").is_some());
        assert!(v["top_words"].is_array());

        // Assert (values)
        assert_eq!(v["total_words"].as_u64().unwrap(), 6);
        assert_eq!(v["top_words"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn json_output_contains_correct_top_word_entry() {
        // Arrange
        let input = "a ab abc ab abc abc";
        let (total, items) = top_words(input, 10, 3);
        let output = to_output_struct(total, items);

        // Act
        let json = serde_json::to_string(&output).unwrap();
        let v: Value = serde_json::from_str(&json).unwrap();

        // Assert
        assert_eq!(v["total_words"].as_u64().unwrap(), 3);

        let arr = v["top_words"].as_array().unwrap();
        assert_eq!(arr.len(), 1);

        assert_eq!(arr[0]["word"].as_str().unwrap(), "abc");
        assert_eq!(arr[0]["count"].as_u64().unwrap(), 3);
    }

    #[test]
    fn json_output_for_empty_input_is_stable() {
        // Arrange
        let input = "   !!!   ... ??? ";
        let (total, items) = top_words(input, 5, 1);
        let output = to_output_struct(total, items);

        // Act
        let json = serde_json::to_string(&output).unwrap();
        let v: Value = serde_json::from_str(&json).unwrap();

        // Assert
        assert_eq!(v["total_words"].as_u64().unwrap(), 0);
        assert!(v["top_words"].as_array().unwrap().is_empty());
    }
}
