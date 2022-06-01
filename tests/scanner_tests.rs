
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

    #[test]
    fn test_paragraph() {
        let test_string = "Hallo\n";
        let assert_string = "<p>Hallo</p>\n";

        let buffer = test_string.as_bytes().to_vec();
        let token_list = scanner::scan_token(&buffer);
        let output = htmlout::html_out(&token_list);

        assert_eq!(output, assert_string);
    }

    #[test]
    fn test_unorderedlist() {
        let test_string = "* element\n\n";
        let assert_string = "<ul>\n<li>element</li>\n</ul>\n\n";

        let buffer = test_string.as_bytes().to_vec();
        let token_list = scanner::scan_token(&buffer);
        let output = htmlout::html_out(&token_list);

        assert_eq!(output, assert_string);
    }

    #[test]
    fn test_unorderedlist_fail_without_space() {
        let test_string = "*element\n\n";
        let assert_string = "<p>*element</p>\n\n";

        let buffer = test_string.as_bytes().to_vec();
        let token_list = scanner::scan_token(&buffer);
        let output = htmlout::html_out(&token_list);

        assert_eq!(output, assert_string);
    }

    #[test]
    fn test_orderedlist() {
        let test_string = "1. element 1\n133. element 2\n1. element 3\n\n";
        let assert_string = "<ol>\n<li>element 1</li>\n<li>element 2</li>\n<li>element 3</li>\n</ol>\n\n";

        let buffer = test_string.as_bytes().to_vec();
        let token_list = scanner::scan_token(&buffer);
        let output = htmlout::html_out(&token_list);

        assert_eq!(output, assert_string);
    }

    #[test]
    fn test_orderedlist_fail_without_point() {
        let test_string = "1 element\n\n";
        let assert_string = "<p>1 element</p>\n\n";

        let buffer = test_string.as_bytes().to_vec();
        let token_list = scanner::scan_token(&buffer);
        let output = htmlout::html_out(&token_list);

        assert_eq!(output, assert_string);
    }
}