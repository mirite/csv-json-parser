pub fn format_output(keys: Vec<String>, rows: Vec<Vec<String>>) -> String {
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
                keys[i],
                value_escape,
                row[i].replace('"', "\\\"").replace('\n', "\\n"),
                value_escape
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
