#[cfg(test)]
mod tests {
    use crate::input::{parse_csv_string, parse_document};
    use std::fs::read_to_string;
    #[test]
    fn read_basic() {
        let result = parse_csv_string("A,B,C\n1,D,3");
        assert_eq!(result, "[{\"A\":1,\"B\":\"D\",\"C\":3}]");
    }

    #[test]
    fn read_carriage_return() {
        let result = parse_csv_string("A,B,C\r\n1,D,3");
        assert_eq!(result, "[{\"A\":1,\"B\":\"D\",\"C\":3}]");
    }

    #[test]
    fn read_carriage_return_after_blank_cell() {
        let result = parse_csv_string("A,B,C\r\n1,D,\r\nA,B,C");
        assert_eq!(
            result,
            "[{\"A\":1,\"B\":\"D\",\"C\":\"\"},{\"A\":\"A\",\"B\":\"B\",\"C\":\"C\"}]"
        );
    }

    #[test]
    fn read_carriage_return_after_escaped() {
        let result = parse_csv_string("A,B,\"C,D\"\r\n1,D,3");
        assert_eq!(result, "[{\"A\":1,\"B\":\"D\",\"C,D\":3}]");
    }

    #[test]
    fn read_escaped() {
        let result = parse_csv_string("A,\"B, E\",C\n1,D,3");
        assert_eq!(result, "[{\"A\":1,\"B, E\":\"D\",\"C\":3}]");
    }

    #[test]
    fn read_with_breaks() {
        let result = parse_csv_string("A,B,C\n1,\"D\nE\",3");
        assert_eq!(result, "[{\"A\":1,\"B\":\"D\nE\",\"C\":3}]");
    }

    #[test]
    fn read_escaped_at_end() {
        let result = parse_csv_string("A,B,\"C E\"\n1,D,3");
        assert_eq!(result, "[{\"A\":1,\"B\":\"D\",\"C E\":3}]");
    }

    #[test]
    fn ignore_trailing_whitespace() {
        let result = parse_csv_string("A,B,C\n1,D,3\n\n");
        assert_eq!(result, "[{\"A\":1,\"B\":\"D\",\"C\":3}]");
    }

    #[test]
    fn read_blank_line() {
        let result = parse_csv_string("A,B,C\n,,\n,,");
        assert_eq!(
            result,
            "[{\"A\":\"\",\"B\":\"\",\"C\":\"\"},{\"A\":\"\",\"B\":\"\",\"C\":\"\"}]"
        );
    }

    #[test]
    fn read_blank_line_then_value() {
        let result = parse_csv_string("A,B,C\n,,\na,b,c");
        assert_eq!(
            result,
            "[{\"A\":\"\",\"B\":\"\",\"C\":\"\"},{\"A\":\"a\",\"B\":\"b\",\"C\":\"c\"}]"
        );
    }

    #[test]
    fn read_escaped_quotes() {
        let result = parse_csv_string("A,B,C\n,\"Hi\"\"\",");
        assert_eq!(result, "[{\"A\":\"\",\"B\":\"Hi\\\"\",\"C\":\"\"}]");
    }

    #[test]
    fn read_file() {
        let content = read_to_string("All_People.csv");
        let column_count = 162;
        let (keys, rows) = match content {
            Ok(f) => parse_document(f.as_str()),
            Err(e) => panic!("{}", e),
        };
        assert_eq!(keys.len(), column_count);
        assert_eq!(rows.len(), 12316);
        for row in rows {
            let columns_present = row.len();
            if row[34] == "lindsaysleightholm@gmail.com" {
                println!("{}", row[34]);
            }
            assert_eq!(
                columns_present, column_count,
                "Incorrect column count for {}. {} present, {} expected.",
                row[34], columns_present, column_count
            );
        }
    }
}
