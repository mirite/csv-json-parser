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
    let mut rows: Vec<Vec<String>> = vec![];

    while index < content.len() {
        if parser_state == State::StartingRow {
            if in_headers_row {
                in_headers_row = false;
            } else {
                current = add_to_rows(current, &mut rows);
            }
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
    commit_string(in_headers_row, &mut keys, &mut current, buffer);
    add_to_rows(current, &mut rows);
    format_output(keys, rows)
}

fn format_output(keys: Vec<String>, rows: Vec<Vec<String>>) -> String {
    let mut output = String::from("[");
    let mut rows_ittr = rows.iter().peekable();
    let column_count = keys.len();
    while let Some(row) = rows_ittr.next() {
        output.push_str("{");
        for i in 0..column_count {
            let value_escape = match row[i].parse::<i32>() {
                Ok(_t) => "",
                Err(_t) => "\"",
            };
            output.push_str(&format!(
                "\"{}\":{}{}{}",
                keys[i], value_escape, row[i], value_escape
            ));
            if i != column_count - 1 {
                output.push_str(",");
            }
        }
        output.push_str("}");
        if rows_ittr.peek().is_some() {
            output.push_str(",");
        }
    }
    output.push_str("]");
    output
}

fn commit_string(
    in_headers_row: bool,
    keys: &mut Vec<String>,
    current: &mut Vec<String>,
    buffer: String,
) -> String {
    if in_headers_row == true {
        keys.push(buffer);
    } else {
        current.push(buffer);
    }
    String::from("")
}

fn add_to_rows(current: Vec<String>, rows: &mut Vec<Vec<String>>) -> Vec<String> {
    rows.push(current);
    vec![]
}
