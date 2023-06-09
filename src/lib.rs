mod my_tests;
mod tests;

#[derive(Eq, PartialEq, Debug)]
/// A crypto square.
struct Square(String);

impl Square {
    /// Creates a new crypto square with the given plaintext.
    fn new(input: &str) -> Square {
        Square(
            // Normalize plaintext to a lowercase alphanumeric string
            input
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect(),
        )
    }

    /// Returns the square's ciphertext.
    pub fn encrypt(&self) -> String {
        match self.0.len() {
            0..=2 => self.0.to_string(),
            _ => {
                let (output_cols, encryption_cols) = Square::nearest_factor(self.0.len()).unwrap();
                let encrypted_input = self.encrypted_input(encryption_cols);
                // Format encrypted input into correctly-sized rows for output
                let extra_chars = self.0.len() % output_cols;
                let mut normal_row_count = self.0.len() / output_cols;
                if extra_chars > 0 {
                    // Subtract the number of truncated rows
                    // (the number of missing characters in the final row)
                    normal_row_count -= output_cols - extra_chars - 1;
                }
                // Build full-length rows
                (0..normal_row_count)
                    .map(|i| {
                        String::from_iter(
                            encrypted_input
                                .chars()
                                .skip(output_cols * i)
                                .take(output_cols),
                        )
                    })
                    // Build truncated rows
                    .chain(
                        encrypted_input
                            .chars()
                            .skip(normal_row_count * output_cols)
                            .collect::<Vec<char>>()
                            .chunks(output_cols - 1)
                            .map(|c| String::from_iter(c) + " "),
                    )
                    .collect::<Vec<String>>()
                    .join(" ")
            }
        }
    }

    /// Finds the factors of the plaintext length with the smallest distance (0 or 1).
    /// If no such factors exist for the plaintext length, returns factors of the
    /// next largest number that has such factors.
    fn nearest_factor(mut input_len: usize) -> Option<(usize, usize)> {
        let mut candidate: Option<(usize, usize)> = Square::factor(input_len);
        while !candidate.is_some() {
            input_len += 1;
            candidate = Square::factor(input_len);
        }
        candidate
    }

    /// Returns the factor pair with the smallest distance (0 or 1) for the given number, if any.
    /// Factors are listed in increasing order.
    fn factor(input_len: usize) -> Option<(usize, usize)> {
        (1..=((input_len as f64).sqrt().floor()) as usize)
            .rev()
            .find_map(|c| {
                if input_len % c == 0 && input_len / c - c <= 1_usize {
                    Some((c, input_len / c))
                } else {
                    None
                }
            })
    }

    /// Applies the square encryption method to the plaintext and returns the resulting ciphertext.
    fn encrypted_input(&self, cols: usize) -> String {
        (0..cols)
            .map(|i| self.0.chars().skip(i).step_by(cols).collect::<String>())
            .collect()
    }
}

pub fn encrypt(input: &str) -> String {
    Square::new(input).encrypt()
}
