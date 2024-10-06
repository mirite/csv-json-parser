mod tests;

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
    String::from("")
}
