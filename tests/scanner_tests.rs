
#[cfg(test)]
mod tests {
    use markdowncompiler::{scanner, htmlout};

    #[test]
    fn test_header() {
        let test_string = 
        "# Hallo\n## Hallo\n### Hallo\n#### Hallo\n##### Hallo\n###### Hallo\n";
        let assert_string = 
        "<h1>Hallo</h1>\n<h2>Hallo</h2>\n<h3>Hallo</h3>\n<h4>Hallo</h4>\n<h5>Hallo</h5>\n<h6>Hallo</h6>\n";

        let buffer = test_string.as_bytes().to_vec();
        let token_list = scanner::scan_token(&buffer);
        let output = htmlout::html_out(&token_list);

        assert_eq!(output, assert_string);
    }

    #[test]
    fn test_header_fails_with_missing_space() {
        let test_string = "##Hallo\n";
        let assert_string = "<p>##Hallo</p>\n";

        let buffer = test_string.as_bytes().to_vec();
        let token_list = scanner::scan_token(&buffer);
        let output = htmlout::html_out(&token_list);

        assert_eq!(output, assert_string);
    }

    #[test]
    fn test_header_fails_with_7_number_signs() {
        let test_string = "####### Hallo\n";
        let assert_string = "<p>#######Hallo</p>\n";

        let buffer = test_string.as_bytes().to_vec();
        let token_list = scanner::scan_token(&buffer);
        let output = htmlout::html_out(&token_list);

        assert_eq!(output, assert_string);
    }
}