use std::iter::FromIterator;

pub fn jumble(input_string: &str) -> String {
    let mut letter_count = 0i32;

    let mut result = "".to_owned();

    for c in input_string.chars() {
        match c {
            'a'...'z' | 'A'...'Z' => {
                match letter_count % 2 {
                    0 => result.push_str(&String::from_iter(c.to_uppercase())),
                    _ => result.push_str(&String::from_iter(c.to_lowercase()))
                }
                letter_count += 1
            }
            _ => result.push(c)
        };
    };

    result
}
