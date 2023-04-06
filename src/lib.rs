mod my_tests;
mod tests;

#[derive(Eq, PartialEq, Debug)]
struct Square {
    rows: usize,
    cols: usize,
}

pub fn encrypt(input: &str) -> String {
    let normalized_input: String = normalize(input);
    match normalized_input.len() {
        0..=2 => normalized_input.to_string(),
        n => {
            let Square { cols, rows } = find_nearest_factor(n).unwrap();
            padded_flat_chunks(encoded_flat(normalized_input, cols), rows)
        }
    }
}

// TODO: make functions methods of Square

/// Produce a lowercase alphanumeric version of an input string.
fn normalize(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect()
}

/// Finds the factor of the nearest number to the input.
/// Returns the factor and the distance.
fn find_nearest_factor(mut input: usize) -> Option<Square> {
    let mut candidate: Option<Square> = factor(input);
    while !candidate.is_some() {
        input += 1;
        candidate = factor(input);
    }
    candidate.map(|c| (c))
}

/// Find a factor pair for the given number with the smallest distance, either 0 or 1.
fn factor(input: usize) -> Option<Square> {
    (1..=((input as f64).sqrt().floor()) as usize)
        .rev()
        .find_map(|c| {
            if input % c == 0 && input / c - c <= 1_usize {
                Some(Square {
                    rows: c,
                    cols: input / c,
                })
            } else {
                None
            }
        })
}

fn encoded_flat(input: String, cols: usize) -> String {
    (0..cols)
        .map(|i| input.chars().skip(i).step_by(cols).collect::<String>())
        .collect()
}

/// Splits input text into equal-sized chunks, joins chunks into a space-separated string.
/// Trailing whitepace padding is evenly distributed between final rows.
fn padded_flat_chunks(input: String, cols: usize) -> String {
    let has_short_rows = input.len() % cols != 0;
    let normal_row_count = (input.len() / cols) + {
        if has_short_rows {
            1
        } else {
            0
        }
    } - if has_short_rows {
        cols - (input.len() % cols)
    } else {
        0
    };
    (0..normal_row_count)
        .map(|i| String::from_iter(input.chars().skip(cols * i).take(cols)))
        .chain(
            input
                .chars()
                .collect::<Vec<char>>()
                .into_iter()
                .skip(normal_row_count * cols)
                .collect::<Vec<char>>()
                .chunks(cols - 1)
                .map(|c| String::from_iter(c) + " "),
        )
        .collect::<Vec<String>>()
        .join(" ")
}
