mod my_tests;
mod tests;

#[derive(Eq, PartialEq, Debug)]
struct Factor(usize, usize);

pub fn encrypt(input: &str) -> String {
    let normalized_input: String = normalize(input);
    match normalized_input.len() {
        0 => String::new(),
        1 => input.to_string(),
        n => {
            let Factor(cols,_) = find_nearest_factor(n).unwrap();
            let input_chunks = flat_chunks(normalized_input, cols);
            // TODO: Remove
            input_chunks
            // TODO: Read input chunks top to bottom, left to right
            // TODO: Produce output chunks with missing chars spread out
            // TODO: Join chunks with space buffers
        }
    }
}

/// Splits input text into equal-sized chunks, joins chunks into a space-separated string.
fn padded_flat_chunks(input: String, cols: usize) -> String {
    unimplemented!("oh no")
}

/// Splits input text into equal-sized chunks, joins chunks into a space-separated string.
fn flat_chunks(input: String, cols: usize) -> String {
    // TODO: Write me
    String::new()
}

/// Finds the factor of the nearest number to the input.
/// Returns the factor and the distance.
fn find_nearest_factor(mut input: usize) -> Option<Factor> {
    let mut candidate: Option<Factor> = factor(input);
    while !candidate.is_some() && input > 0 {
        input -= 1;
        candidate = factor(input);
    }
    candidate.map(|c| (c))
}

/// Produce a lowercase alphanumeric version of an input string.
fn normalize(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect()
}

/// Find a factor pair for the given number with the smallest distance, either 0 or 1.
fn factor(input: usize) -> Option<Factor> {
    match input {
        2 => Some(Factor(2,2)),
        n => (1..=((input as f64).sqrt().floor()) as usize)
            .filter_map(|c| {
                if input % c == 0 && (input / c) - c <= 1_usize {
                    Some(Factor(c, input / c))
                } else {
                    None
                }
            })
            .min_by_key(|f| f.1 - f.0)
    }

}
