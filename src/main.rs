use std::collections::HashMap;
use std::fs::read_to_string;

// Handle case for "one", "two", etc.
//

fn main() {
    let number_conversions: HashMap<&str, i32> = HashMap::from([
        // key, value
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let lines: Vec<&str> = vec!["1eodio2"];
    let mut current_digits = Vec::new();
    let mut sum = 0;

    for line in &lines {
        let mut ptr1 = 0;
        let mut ptr2 = line.len() - 1; // minus 1 due to indexing [0, 1, 2, 3] so len would return 4 (but we need index 3)
      
        let mut running_ptr1 = true;
        let mut running_ptr2 = true;

        let line_chars: Vec<char> = String::from(*line).chars().collect();

        while ptr1 < ptr2 && (running_ptr1 || running_ptr2) {
            // if this fails we know we only have 1 digit (eg. 77)
            if (line_chars[ptr1]).is_numeric() {
                if running_ptr1 {
                    current_digits.push(line_chars[ptr1].to_digit(10).unwrap() as i32);
                    running_ptr1 = false;
                }
            } else {
                ptr1 += 1
            }

            if (line_chars[ptr2]).is_numeric() {
                if running_ptr2 {
                    current_digits.push(line_chars[ptr2].to_digit(10).unwrap() as i32);
                    running_ptr2 = false;
                }
            } else {
                ptr2 -= 1
            }
        }
        if current_digits.len() <= 2 {
            sum += current_digits[0];
            sum += current_digits[1];
        }
    }
}