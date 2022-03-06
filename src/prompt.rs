use std::io;
use std::io::Write;

pub fn parse_index(num_str: &str) -> Option<usize> {
    let maybe_read_num = num_str.parse();

    if let Ok(read_num) = maybe_read_num {
        match read_num {
            0 => None,
            _ => Some(read_num - 1)
        }
    } else {
        None
    }
}

pub fn prompt_user(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut result = String::new();
    io::stdin()
        .read_line(&mut result)
        .expect("Failed to read line");

    result.trim().to_string()
}
