#[cfg(test)]
mod tests {
    use crate::input::parse_csv_string;
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
        let result = parse_csv_string("A,B,C\n,,");
        assert_eq!(result, "[{\"A\":\"\",\"B\":\"\",\"C\":\"\"}]");
    }

    // #[test]
    // fn read_file() {
    //     let content = read_to_string("All_People.csv");
    //     match content {
    //         Ok(f) => parse_csv_string(f.as_str()),
    //         Err(e) => panic!("{}", e),
    //     };
    // }
}
