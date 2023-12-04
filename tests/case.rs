#[test]
fn camel_case() {
    assert_eq!(reword::camel_case("hello world"), "helloWorld");
    assert_eq!(reword::camel_case("this_is_a_test"), "thisIsATest");
    assert_eq!(reword::camel_case("  leading_spaces  "), "leadingSpaces");
    assert_eq!(reword::camel_case("trailing_spaces  "), "trailingSpaces");
    // assert_eq!(
    //     reword::camel_case("iNpUt_wiTh_MiXeD_cAsE"),
    //     "inputWithMixedCase"
    // );
    assert_eq!(reword::camel_case("this-is-an_example"), "thisIsAnExample");
    assert_eq!(reword::camel_case("camel+case*input"), "camelCaseInput");
    assert_eq!(
        reword::camel_case("string_with_123_numbers"),
        "stringWith123Numbers"
    );
    assert_eq!(
        reword::camel_case("string___with_multiple__underscores"),
        "stringWithMultipleUnderscores"
    );
    assert_eq!(
        reword::camel_case("multiple  spaces    here"),
        "multipleSpacesHere"
    );
    assert_eq!(reword::camel_case(""), "");
    assert_eq!(reword::camel_case("a"), "a");
    assert_eq!(reword::camel_case("HTML_parser"), "htmlParser");
    assert_eq!(reword::camel_case("JSON_API_response"), "jsonApiResponse");
    assert_eq!(reword::camel_case("!@#$%^&*()"), "");
}

#[test]
fn pascal_case() {
    assert_eq!(reword::pascal_case("hello world"), "HelloWorld");
    assert_eq!(reword::pascal_case("this_is_a_test"), "ThisIsATest");
    assert_eq!(reword::pascal_case("  leading_spaces  "), "LeadingSpaces");
    assert_eq!(reword::pascal_case("trailing_spaces  "), "TrailingSpaces");
    // assert_eq!(
    //     reword::pascal_case("iNpUt_wiTh_MiXeD_cAsE"),
    //     "InputWithMixedCase"
    // );
    assert_eq!(reword::pascal_case("this-is-an_example"), "ThisIsAnExample");
    assert_eq!(reword::pascal_case("pascal+case*input"), "PascalCaseInput");
    assert_eq!(
        reword::pascal_case("string_with_123_numbers"),
        "StringWith123Numbers"
    );
    assert_eq!(
        reword::pascal_case("string___with_multiple__underscores"),
        "StringWithMultipleUnderscores"
    );
    assert_eq!(
        reword::pascal_case("multiple  spaces    here"),
        "MultipleSpacesHere"
    );
    assert_eq!(reword::pascal_case(""), "");
    assert_eq!(reword::pascal_case("a"), "A");
    assert_eq!(reword::pascal_case("HTML_parser"), "HtmlParser");
    assert_eq!(reword::pascal_case("JSON_API_response"), "JsonApiResponse");
    assert_eq!(reword::pascal_case("!@#$%^&*()"), "");
}

#[test]
fn snake_case() {
    assert_eq!(reword::snake_case("hello world"), "hello_world");
    assert_eq!(reword::snake_case("this_is_a_test"), "this_is_a_test");
    assert_eq!(reword::snake_case("  leading_spaces  "), "leading_spaces");
    assert_eq!(reword::snake_case("trailing_spaces  "), "trailing_spaces");
    assert_eq!(
        reword::snake_case("iNpUt_wiTh_MiXeD_cAsE"),
        "input_with_mixed_case"
    );
    assert_eq!(
        reword::snake_case("this-is-an_example"),
        "this_is_an_example"
    );
    assert_eq!(reword::snake_case("snake+case*input"), "snake_case_input");
    assert_eq!(
        reword::snake_case("string_with_123_numbers"),
        "string_with_123_numbers"
    );
    assert_eq!(
        reword::snake_case("string___with_multiple__underscores"),
        "string_with_multiple_underscores"
    );
    assert_eq!(
        reword::snake_case("multiple  spaces    here"),
        "multiple_spaces_here"
    );
    assert_eq!(reword::snake_case(""), "");
    assert_eq!(reword::snake_case("a"), "a");
    assert_eq!(reword::snake_case("HTML_parser"), "html_parser");
    assert_eq!(reword::snake_case("JSON_API_response"), "json_api_response");
    assert_eq!(reword::snake_case("!@#$%^&*()"), "");
}

#[test]
fn screaming_snake_case() {
    assert_eq!(reword::screaming_snake_case("hello world"), "HELLO_WORLD");
    assert_eq!(
        reword::screaming_snake_case("this_is_a_test"),
        "THIS_IS_A_TEST"
    );
    assert_eq!(
        reword::screaming_snake_case("  leading_spaces  "),
        "LEADING_SPACES"
    );
    assert_eq!(
        reword::screaming_snake_case("trailing_spaces  "),
        "TRAILING_SPACES"
    );
    assert_eq!(
        reword::screaming_snake_case("iNpUt_wiTh_MiXeD_cAsE"),
        "INPUT_WITH_MIXED_CASE"
    );
    assert_eq!(
        reword::screaming_snake_case("this-is-an_example"),
        "THIS_IS_AN_EXAMPLE"
    );
    assert_eq!(
        reword::screaming_snake_case("snake+case*input"),
        "SNAKE_CASE_INPUT"
    );
    assert_eq!(
        reword::screaming_snake_case("string_with_123_numbers"),
        "STRING_WITH_123_NUMBERS"
    );
    assert_eq!(
        reword::screaming_snake_case("string___with_multiple__underscores"),
        "STRING_WITH_MULTIPLE_UNDERSCORES"
    );
    assert_eq!(
        reword::screaming_snake_case("multiple  spaces    here"),
        "MULTIPLE_SPACES_HERE"
    );
    assert_eq!(reword::screaming_snake_case(""), "");
    assert_eq!(reword::screaming_snake_case("a"), "A");
    assert_eq!(reword::screaming_snake_case("HTML_parser"), "HTML_PARSER");
    assert_eq!(
        reword::screaming_snake_case("JSON_API_response"),
        "JSON_API_RESPONSE"
    );
    assert_eq!(reword::screaming_snake_case("!@#$%^&*()"), "");
}

#[test]
fn kebab_case() {
    assert_eq!(reword::kebab_case("hello world"), "hello-world");
    assert_eq!(reword::kebab_case("this_is_a_test"), "this-is-a-test");
    assert_eq!(reword::kebab_case("  leading_spaces  "), "leading-spaces");
    assert_eq!(reword::kebab_case("trailing_spaces  "), "trailing-spaces");
    assert_eq!(
        reword::kebab_case("iNpUt_wiTh_MiXeD_cAsE"),
        "input-with-mixed-case"
    );
    assert_eq!(
        reword::kebab_case("this-is-an_example"),
        "this-is-an-example"
    );
    assert_eq!(reword::kebab_case("kebab+case*input"), "kebab-case-input");
    assert_eq!(
        reword::kebab_case("string_with_123_numbers"),
        "string-with-123-numbers"
    );
    assert_eq!(
        reword::kebab_case("string___with_multiple__underscores"),
        "string-with-multiple-underscores"
    );
    assert_eq!(
        reword::kebab_case("multiple  spaces    here"),
        "multiple-spaces-here"
    );
    assert_eq!(reword::kebab_case(""), "");
    assert_eq!(reword::kebab_case("a"), "a");
    assert_eq!(reword::kebab_case("HTML_parser"), "html-parser");
    assert_eq!(reword::kebab_case("JSON_API_response"), "json-api-response");
    assert_eq!(reword::kebab_case("!@#$%^&*()"), "");
}

#[test]
fn screaming_kebab_case() {
    assert_eq!(reword::screaming_kebab_case("hello world"), "HELLO-WORLD");
    assert_eq!(
        reword::screaming_kebab_case("this_is_a_test"),
        "THIS-IS-A-TEST"
    );
    assert_eq!(
        reword::screaming_kebab_case("  leading_spaces  "),
        "LEADING-SPACES"
    );
    assert_eq!(
        reword::screaming_kebab_case("trailing_spaces  "),
        "TRAILING-SPACES"
    );
    assert_eq!(
        reword::screaming_kebab_case("iNpUt_wiTh_MiXeD_cAsE"),
        "INPUT-WITH-MIXED-CASE"
    );
    assert_eq!(
        reword::screaming_kebab_case("this-is-an_example"),
        "THIS-IS-AN-EXAMPLE"
    );
    assert_eq!(
        reword::screaming_kebab_case("kebab+case*input"),
        "KEBAB-CASE-INPUT"
    );
    assert_eq!(
        reword::screaming_kebab_case("string_with_123_numbers"),
        "STRING-WITH-123-NUMBERS"
    );
    assert_eq!(
        reword::screaming_kebab_case("string___with_multiple__underscores"),
        "STRING-WITH-MULTIPLE-UNDERSCORES"
    );
    assert_eq!(
        reword::screaming_kebab_case("multiple  spaces    here"),
        "MULTIPLE-SPACES-HERE"
    );
    assert_eq!(reword::screaming_kebab_case(""), "");
    assert_eq!(reword::screaming_kebab_case("a"), "A");
    assert_eq!(reword::screaming_kebab_case("HTML_parser"), "HTML-PARSER");
    assert_eq!(
        reword::screaming_kebab_case("JSON_API_response"),
        "JSON-API-RESPONSE"
    );
    assert_eq!(reword::screaming_kebab_case("!@#$%^&*()"), "");
}
