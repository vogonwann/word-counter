use word_counter::{Options, parse_args, top_words};
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        eprintln!();
        eprintln!("Usage: word_counter --top N --min-length M [--file FILE_PATH] [--json]");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args = parse_args()?;

    if let Some(file_path) = &args.file_path {
        // read input from file
        let input = std::fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {}: {}", file_path, e))?;

        parse_input(&input, &args)?;

        return Ok(());
    }

    let mut input = String::new();

    // read input from stdin
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    parse_input(&input, &args)?;

    Ok(())
}

/// Parse input and print results
/// # Arguments
/// * `input` - input string to parse
/// * `args` - command line arguments
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Ok if successful, Err otherwise
fn parse_input(input: &str, args: &Options) -> Result<(), String> {
    let (total_words, items) = top_words(&input, args.top_n, args.min_length);

    if args.json {
        // output as json
        let output = word_counter::to_output_struct(total_words, items);
        let json_output =
            serde_json::to_string_pretty(&output).map_err(|e| format!("JSON error: {}", e))?;
        println!("{}", json_output);
    } else {
        // print total words and word count
        println!("Total words: {}", total_words);
        for (word, count) in items.into_iter() {
            println!("{}: {}", word, count);
        }
    }

    Ok(())
}
