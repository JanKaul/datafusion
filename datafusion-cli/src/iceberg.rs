use regex::Regex;

pub(crate) fn transform_iceberg_input(input: &str) -> String {
    let re = Regex::new(r"(?i)create\s+table").unwrap();
    re.replace_all(input, "create external table").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_iceberg_table_creation() {
        let input = "CREATE TABLE test (id INT, name STRING) stored as ICEBERG;";
        let expected =
            "create external table test (id INT, name STRING) stored as ICEBERG;";
        assert_eq!(transform_iceberg_input(input), expected);
    }

    #[test]
    fn test_mixed_iceberg_and_normal_statements() {
        let input =
            "CREATE TABLE iceberg (id INT) stored as ICEBERG; SELECT * FROM table;";
        let expected = "create external table iceberg (id INT) stored as ICEBERG; SELECT * FROM table;";
        assert_eq!(transform_iceberg_input(input), expected);
    }

    #[test]
    fn test_case_insensitive_iceberg_keyword() {
        let input = "create table test (id INT) stored as ICEBERG;";
        let expected = "create external table test (id INT) stored as ICEBERG;";
        assert_eq!(transform_iceberg_input(input), expected);
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        assert_eq!(transform_iceberg_input(input), input);
    }
    #[test]
    fn test_multiple_spaces_in_create_statement() {
        let input = "CREATE       TABLE test (id INT) stored as ICEBERG;";
        let expected = "create external table test (id INT) stored as ICEBERG;";
        assert_eq!(transform_iceberg_input(input), expected);
    }

    #[test]
    fn test_newlines_and_spaces() {
        let input =
            "CREATE TABLE\n    test (\n        id INT,\n        name STRING\n    ) stored as ICEBERG;";
        let expected = "create external table\n    test (\n        id INT,\n        name STRING\n    ) stored as ICEBERG;";
        assert_eq!(transform_iceberg_input(input), expected);
    }

    #[test]
    fn test_complex_mixed_statements() {
        let input = "
            CREATE TABLE normal (id INT) stored as ICEBERG;
            CREATE TABLE iceberg1 (id INT, name STRING) stored as ICEBERG;
            INSERT INTO normal VALUES (1);
            SELECT * FROM iceberg1 JOIN iceberg2 ON iceberg1.id = iceberg2.id;
        ";
        let expected = "
            create external table normal (id INT) stored as ICEBERG;
            create external table iceberg1 (id INT, name STRING) stored as ICEBERG;
            INSERT INTO normal VALUES (1);
            SELECT * FROM iceberg1 JOIN iceberg2 ON iceberg1.id = iceberg2.id;
        ";
        assert_eq!(transform_iceberg_input(input), expected);
    }
}
