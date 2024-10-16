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
    let approximate_rows = content.matches('\n').count();
    let delimiter = ',';
    let mut buffer = String::with_capacity(128);
    let mut keys: Vec<String> = Vec::with_capacity(10);
    let mut current: Vec<String> = Vec::with_capacity(10);
    let mut rows: Vec<Vec<String>> = Vec::with_capacity(approximate_rows);
    let mut chars = content.chars().peekable();

    while let Some(&current_char) = chars.peek() {
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
                        commit_string(in_headers_row, &mut keys, &mut current, &mut buffer);
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
                    commit_string(in_headers_row, &mut keys, &mut current, &mut buffer);
                    parser_state = StartingCell;
                } else if current_char == '\n' {
                    parser_state = StartingRow;
                    commit_string(in_headers_row, &mut keys, &mut current, &mut buffer);
                } else if current_char != '\r' {
                    buffer.push(current_char);
                }
            }
            InQuotedCell => {
                if current_char == '"' {
                    commit_string(in_headers_row, &mut keys, &mut current, &mut buffer);
                    chars.next(); // Skip over the delimiter
                    parser_state = match chars.peek() {
                        Some('\n') => StartingRow,
                        Some('\r') => {
                            chars.next();
                            StartingRow
                        }
                        _ => StartingCell,
                    };
                } else {
                    buffer.push(current_char);
                }
            }
            _ => {}
        }
        chars.next();
    }
    commit_string(in_headers_row, &mut keys, &mut current, &mut buffer);
    add_to_rows(current, &mut rows);
    output::format_output(keys, rows)
}

fn commit_string(
    in_headers_row: bool,
    keys: &mut Vec<String>,
    current: &mut Vec<String>,
    buffer: &mut String,
) {
    if in_headers_row == true {
        keys.push(buffer.clone());
    } else {
        current.push(buffer.clone());
    }
    buffer.clear();
}

fn add_to_rows(current: Vec<String>, rows: &mut Vec<Vec<String>>) -> Vec<String> {
    let length = current.len();
    rows.push(current);
    Vec::with_capacity(length)
}
