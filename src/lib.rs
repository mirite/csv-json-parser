mod tests;

#[derive(PartialEq)]
enum State {
    InCell,
    InQuotedCell,
    StartingCell,
    StartingRow,
}

fn main() {
    println!("Hello, world!");
}

pub fn parse_csv_string(content: &str) -> String {
    let mut parser_state = State::StartingCell;
    let mut in_headers_row = true;
    let mut index = 0;
    let delimiter = ',';
    let mut buffer = String::from("");

    while index < content.len() {
        if parser_state == State::StartingRow {
            // Add a new object to our output
            parser_state = State::StartingCell;
        }
        let current_char = content.chars().nth(index).unwrap();

        match parser_state {
            State::StartingCell => {
                if current_char == '"' {
                    parser_state = State::InQuotedCell;
                    // Add the char to the string being built
                    buffer.push(current_char);
                } else if current_char == '\n' {
                    in_headers_row = false;
                    parser_state = State::StartingRow;
                } else {
                    parser_state = State::InCell;
                    // Add the char to the string being built
                    buffer.push(current_char);
                }
            }
            State::InCell => {
                if current_char == delimiter {
                    // Commit the string
                    println!("{}", buffer);
                    buffer = String::from("");
                    parser_state = State::StartingCell;
                } else {
                    // Add the char to the string being built
                    buffer.push(current_char);
                }
            }
            State::InQuotedCell => {
                if current_char == '"' {
                    // Commit the string
                    println!("{}", buffer);
                    buffer = String::from("");
                    index = index + 1; // Skip over the delimiter
                    parser_state = State::StartingCell;
                } else {
                    buffer.push(current_char);
                }
            }
            _ => panic!("Something really strange happened."),
        }
        index = index + 1;
    }
    // Commit the string
    println!("{}", buffer);
    String::from("")
}
