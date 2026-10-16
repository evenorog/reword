use alloc::string::String;
use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase,
};

/// Formats the input string as a kebab case name.
///
/// # Examples
/// ```
/// assert_eq!(reword::kebab_case("hello world"), "hello-world");
/// ```
pub fn kebab_case<T: AsRef<str>>(t: T) -> String {
    t.as_ref().to_kebab_case()
}

/// Formats the input string as a screaming kebab case name.
///
/// # Examples
/// ```
/// assert_eq!(reword::screaming_kebab_case("hello world"), "HELLO-WORLD");
/// ```
pub fn screaming_kebab_case<T: AsRef<str>>(t: T) -> String {
    t.as_ref().to_shouty_kebab_case()
}

/// Formats the input string as a snake case name.
///
/// # Examples
/// ```
/// assert_eq!(reword::snake_case("hello world"), "hello_world");
/// ```
pub fn snake_case<T: AsRef<str>>(t: T) -> String {
    t.as_ref().to_snake_case()
}

/// Formats the input string as a screaming snake case name.
///
/// # Examples
/// ```
/// assert_eq!(reword::screaming_snake_case("hello world"), "HELLO_WORLD");
/// ```
pub fn screaming_snake_case<T: AsRef<str>>(t: T) -> String {
    t.as_ref().to_shouty_snake_case()
}

/// Formats the input string as a camel case name.
///
/// # Examples
/// ```
/// assert_eq!(reword::camel_case("hello world"), "helloWorld");
/// ```
pub fn camel_case<T: AsRef<str>>(t: T) -> String {
    t.as_ref().to_lower_camel_case()
}

/// Formats the input string as a pascal case name.
///
/// # Examples
/// ```
/// assert_eq!(reword::pascal_case("hello world"), "HelloWorld");
/// ```
pub fn pascal_case<T: AsRef<str>>(t: T) -> String {
    t.as_ref().to_pascal_case()
}
