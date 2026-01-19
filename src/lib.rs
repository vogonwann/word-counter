use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Options {
    pub top_n: usize,
    pub min_length: usize,
    pub json: bool,
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct WordCount {
    pub word: String,
    pub count: usize,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Output {
    pub total_words: usize,
    pub top_words: Vec<WordCount>,
}

/// Count total words and return top N words with their counts
/// # Arguments
/// * `input` - input string to parse
/// * `top_n` - number of top words to return
/// * `min_length` - minimum length of words to consider
/// # Returns
/// * `(usize, Vec<(String, usize)>)` - total word count and vector of top N words with their counts
pub fn top_words(input: &str, top_n: usize, min_length: usize) -> (usize, Vec<(String, usize)>) {
    let mut total_words = 0;
    let mut word_count: HashMap<String, usize> = HashMap::new();

    let cleaned_input: String = input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect();
    // split input into words and populate a hashmap with total count for each word repeated
    for s in cleaned_input.split_whitespace() {
        if s.len() < min_length {
            continue;
        }

        total_words += 1;
        *word_count.entry(s.to_string()).or_insert(0) += 1;
    }

    // sort the hashmap by word count in descending order
    let mut items: Vec<(String, usize)> = word_count.into_iter().collect();
    items.sort_by(|(w1, c1), (w2, c2)| c2.cmp(c1).then_with(|| w1.cmp(w2)));

    (total_words, items.into_iter().take(top_n).collect())
}

/// Convert total words and items into Output struct
/// # Arguments
/// * `total_words` - total word count
/// * `items` - vector of top N words with their counts
/// # Returns
/// * `Output` - output struct containing total words and top words
pub fn to_output_struct(total_words: usize, items: Vec<(String, usize)>) -> Output {
    let top_words = items
        .into_iter()
        .map(|(word, count)| WordCount { word, count })
        .collect();

    Output {
        total_words,
        top_words,
    }
}

/// Parse command line arguments into Options struct
/// # Returns
/// * `Result<Options, String>` - Ok with Options if successful, Err with error message otherwise
/// # Errors
/// * If an unknown argument is provided
/// * If a value is missing after an argument that requires one
/// * If a value is not a positive integer where expected
pub fn parse_args() -> Result<Options, String> {
    let mut args = std::env::args().skip(1);
    let mut options = Options {
        top_n: 3,
        min_length: 1,
        json: false,
        file_path: None,
    };

    while let Some(arg) = args.next() {
        if arg == "--top" {
            let value = args
                .next()
                .ok_or_else(|| "Expected a value after --top".to_string())?;

            let n = value
                .parse::<usize>()
                .map_err(|_| "The value after --top must be a positive integer".to_string())?;
            options.top_n = n;
        } else if arg == "--min-length" {
            let value = args
                .next()
                .ok_or_else(|| "Expected a value after --min-length".to_string())?;

            let n = value.parse::<usize>().map_err(|_| {
                "The value after --min-length must be a positive integer".to_string()
            })?;
            options.min_length = n;
        } else if arg == "--file" {
            let value = args
                .next()
                .ok_or_else(|| "Expected a value after --file".to_string())?;
            options.file_path = Some(value);
        } else if arg == "--json" {
            options.json = true;
        } else if arg == "--help" {
            return Err("Usage: word_counter --top N --min-length M".to_string());
        } else {
            return Err(format!("Unknown argument: {}", arg));
        }
    }

    Ok(options)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_total_words_and_top3_sorted() {
        // Input includes punctuation and mixed case; cleaning should normalize it.
        let input = "Rust je super, rust je BRZ!";
        let (total, top) = top_words(input, 3, 1);

        assert_eq!(total, 6);
        assert_eq!(
            top,
            vec![
                ("je".to_string(), 2),
                ("rust".to_string(), 2),
                ("brz".to_string(), 1),
            ]
        );
    }

    #[test]
    fn ignores_punctuation_and_counts_correctly() {
        let input = "A! a, a... b? B";
        let (total, top) = top_words(input, 10, 1);

        assert_eq!(total, 5);
        assert_eq!(top, vec![("a".to_string(), 3), ("b".to_string(), 2)]);
    }

    #[test]
    fn empty_or_non_alphanumeric_input_gives_empty_results() {
        let input = "   !!!   ... ??? ";
        let (total, top) = top_words(input, 3, 1);

        assert_eq!(total, 0);
        assert!(top.is_empty());
    }

    #[test]
    fn top_n_limits_output_size() {
        let input = "one two three four";
        let (_total, top) = top_words(input, 2, 1);

        assert_eq!(top.len(), 2);
    }

    #[test]
    fn deterministic_tie_break_by_word_ascending() {
        // All words have the same count (=1), so sorting should fall back to word asc.
        let input = "z a b";
        let (_total, top) = top_words(input, 10, 1);

        assert_eq!(
            top,
            vec![
                ("a".to_string(), 1),
                ("b".to_string(), 1),
                ("z".to_string(), 1),
            ]
        );
    }

    #[test]
    fn top_n_zero_returns_empty_but_total_is_correct() {
        let input = "rust rust je";
        let (total, top) = top_words(input, 0, 1);

        assert_eq!(total, 3);
        assert!(top.is_empty());
    }
}

#[cfg(test)]
mod json_tests;
