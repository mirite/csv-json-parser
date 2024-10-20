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
    let (keys, rows) = parse_document(content);
    output::format_output(keys, rows)
}

pub fn parse_document(content: &str) -> (Vec<String>, Vec<Vec<String>>) {
    let mut parser_state = StartingCell;
    let mut in_headers_row = true;
    let approximate_rows = content.matches('\n').count();
    let content_cleaned = content.replace('\r', "");
    let delimiter = ',';
    let mut buffer = String::with_capacity(128);
    let mut keys: Vec<String> = Vec::with_capacity(10);
    let mut current: Vec<String> = Vec::with_capacity(10);
    let mut rows: Vec<Vec<String>> = Vec::with_capacity(approximate_rows);
    let mut chars = content_cleaned.chars().peekable();

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
                        buffer = commit_string(in_headers_row, &mut keys, &mut current, buffer);
                        StartingCell
                    }
                    '\n' => {
                        buffer = commit_string(in_headers_row, &mut keys, &mut current, buffer);
                        StartingRow
                    }
                    _ => {
                        buffer.push(current_char);
                        InCell
                    }
                }
            }
            InCell => {
                if current_char == '\n' {
                    parser_state = StartingRow;
                    buffer = commit_string(in_headers_row, &mut keys, &mut current, buffer);
                } else if current_char == delimiter {
                    buffer = commit_string(in_headers_row, &mut keys, &mut current, buffer);
                    parser_state = StartingCell;
                } else {
                    buffer.push(current_char);
                }
            }
            InQuotedCell => {
                if current_char == '"' {
                    chars.next(); // Consume the quote
                    if let Some(&next_char) = chars.peek() {
                        if next_char == '"' {
                            // It's an escaped quote, consume it and add to buffer
                            buffer.push('"');
                        } else {
                            buffer = commit_string(in_headers_row, &mut keys, &mut current, buffer);

                            parser_state = match chars.peek() {
                                Some('\n') => StartingRow,
                                _ => StartingCell,
                            };
                        }
                    }
                } else {
                    buffer.push(current_char);
                }
            }
            _ => {}
        }
        chars.next();
    }
    commit_string(in_headers_row, &mut keys, &mut current, buffer);
    if !current.is_empty() {
        add_to_rows(current, &mut rows);
    }
    (keys, rows)
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
    let length = current.len();
    rows.push(current);
    Vec::with_capacity(length)
}
