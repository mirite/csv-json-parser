#[cfg(test)]
mod tests {
    use crate::parse_csv_string;
    #[test]
    fn read_basic() {
        let result = parse_csv_string("A,B,C\n1,D,3");
        assert_eq!(result, "[{\"A\":1,\"B\":\"D\",\"C\":3}]");
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
        let result = parse_csv_string("A,B,\"C \"E\"\n1,D,3");
        assert_eq!(result, "[{\"A\":1,\"B\":\"D\",\"C E\":3}]");
    }
}
