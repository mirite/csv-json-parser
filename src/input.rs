use crate::input::Error::MalformedRow;
use crate::input::State::{InCell, InQuotedCell, StartingCell, StartingRow};
use crate::output;

#[derive(Debug)]
pub enum Error {
    MalformedRow,
}

#[derive(PartialEq)]
pub enum State {
    InCell,
    InQuotedCell,
    StartingCell,
    StartingRow,
}

pub fn parse_csv_string(content: &str) -> Result<String, Error> {
    match parse_document(content) {
        Ok((keys, rows)) => Ok(output::format_output(keys, rows)),
        Err(e) => Err(e),
    }
}

pub fn parse_document(content: &str) -> Result<(Vec<String>, Vec<Vec<String>>), Error> {
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
    let mut column_count = 0;
    while let Some(&current_char) = chars.peek() {
        if parser_state == StartingRow {
            if current_char != '\n' {
                if in_headers_row {
                    column_count = keys.len();
                    in_headers_row = false;
                } else {
                    current = match add_to_rows(current, &mut rows, column_count) {
                        Ok(value) => value,
                        Err(e) => return Err(e),
                    }
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
                parser_state = match current_char {
                    '\n' => {
                        buffer = commit_string(in_headers_row, &mut keys, &mut current, buffer);
                        StartingRow
                    }
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
    if parser_state != StartingRow {
        commit_string(in_headers_row, &mut keys, &mut current, buffer);
    }
    if !current.is_empty() {
        match add_to_rows(current, &mut rows, column_count) {
            Err(e) => return Err(e),
            _ => {}
        }
    }
    Ok((keys, rows))
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

fn add_to_rows(
    current: Vec<String>,
    rows: &mut Vec<Vec<String>>,
    column_count: usize,
) -> Result<Vec<String>, Error> {
    let length = current.len();
    if length != column_count {
        return Err(MalformedRow);
    }
    rows.push(current);
    Ok(Vec::with_capacity(length))
}
