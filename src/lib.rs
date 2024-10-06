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
    let mut keys: Vec<String> = vec![];
    let mut current: Vec<String> = vec![];

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
                } else {
                    parser_state = State::InCell;
                    buffer.push(current_char);
                }
            }
            State::InCell => {
                if current_char == delimiter {
                    buffer = commit_string(in_headers_row, &mut keys, &mut current, buffer);
                    parser_state = State::StartingCell;
                } else if current_char == '\n' {
                    parser_state = State::StartingRow;
                    in_headers_row = false;
                    buffer = commit_string(in_headers_row, &mut keys, &mut current, buffer);
                } else {
                    buffer.push(current_char);
                }
            }
            State::InQuotedCell => {
                if current_char == '"' {
                    buffer = commit_string(in_headers_row, &mut keys, &mut current, buffer);
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
    buffer = commit_string(in_headers_row, &mut keys, &mut current, buffer);
    String::from("")
}

fn commit_string(
    in_headers_row: bool,
    keys: &mut Vec<String>,
    current: &mut Vec<String>,
    buffer: String,
) -> String {
    println!("{}", buffer);
    if in_headers_row == true {
        keys.push(buffer);
    } else {
        current.push(buffer);
    }
    String::from("")
}
