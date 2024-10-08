use crate::input::State::{InCell, InQuotedCell, StartingCell, StartingRow};
use crate::output;

#[derive(PartialEq)]
pub enum State {
    InCell,
    InQuotedCell,
    StartingCell,
    StartingRow,
}

pub fn parse_csv_string(content: &str) -> String {
    let mut parser_state = StartingCell;
    let mut in_headers_row = true;
    let mut index = 0;
    let delimiter = ',';
    let mut buffer = String::from("");
    let mut keys: Vec<String> = vec![];
    let mut current: Vec<String> = vec![];
    let mut rows: Vec<Vec<String>> = vec![];

    while index < content.len() {
        let current_char = content.chars().nth(index).unwrap();
        if parser_state == StartingRow {
            if current_char != '\n' {
                if in_headers_row {
                    in_headers_row = false;
                } else {
                    current = add_to_rows(current, &mut rows);
                }
                parser_state = StartingCell;
            }
        }

        match parser_state {
            StartingCell => {
                parser_state = match current_char {
                    '"' => InQuotedCell,
                    x if x == delimiter => {
                        buffer = commit_string(in_headers_row, &mut keys, &mut current, buffer);
                        StartingCell
                    }
                    _ => {
                        buffer.push(current_char);
                        InCell
                    }
                }
            }
            InCell => {
                if current_char == delimiter {
                    buffer = commit_string(in_headers_row, &mut keys, &mut current, buffer);
                    parser_state = StartingCell;
                } else if current_char == '\n' {
                    parser_state = StartingRow;
                    buffer = commit_string(in_headers_row, &mut keys, &mut current, buffer);
                } else {
                    buffer.push(current_char);
                }
            }
            InQuotedCell => {
                if current_char == '"' {
                    buffer = commit_string(in_headers_row, &mut keys, &mut current, buffer);
                    index = index + 1; // Skip over the delimiter
                    parser_state = match content.chars().nth(index) {
                        Some('\n') => StartingRow,
                        _ => StartingCell,
                    };
                } else {
                    buffer.push(current_char);
                }
            }
            _ => {}
        }
        index = index + 1;
    }
    commit_string(in_headers_row, &mut keys, &mut current, buffer);
    add_to_rows(current, &mut rows);
    output::format_output(keys, rows)
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
