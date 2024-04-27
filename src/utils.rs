use regex::Regex;

/// Removes the comments from the input String.
/// 3 types of comments can be found:
/// - Inlined comments in the form of //
/// - API comments in the form of a range of lines,
///   starting with "/**" and ending with "*/".
/// - Range comments in the form of a range of lines,
///   starting with "/*" and ending with "*/".
pub(crate) fn remove_comments(input: String) -> String {
    let api_comments_regexp = Regex::new(r"/\*\*[\w\W]*?\*/").expect("failed to compile regexp");
    let range_comments_regexp = Regex::new(r"/\*[\w\W]*?\*/").expect("failed to compile regexp");
    let inline_comments_regexp = Regex::new(r"//.*").expect("failed to compiled regexp");

    let removed_api_comments = api_comments_regexp.replace_all(&input, "");
    let removed_range_comments = range_comments_regexp.replace_all(&removed_api_comments, "");
    inline_comments_regexp
        .replace_all(&removed_range_comments, "")
        .to_string()
}

/// Replace all lines breaks with single space.
pub(crate) fn replace_line_breaks_with_single_space(input: String) -> String {
    let line_breaks_regexp = Regex::new(r"\n").expect("failed to compile regexp");
    line_breaks_regexp.replace_all(&input, " ").to_string()
}

/// Replace all tabs with single space.
pub(crate) fn replace_tabs_with_single_space(input: String) -> String {
    let tabs_regexp = Regex::new(r"\t").expect("failed to compile regexp");
    tabs_regexp.replace_all(&input, " ").to_string()
}

/// Replace all carriage returns with single space.
pub(crate) fn replace_carriage_returns_with_single_space(input: String) -> String {
    let tabs_regexp = Regex::new(r"\r").expect("failed to compile regexp");
    tabs_regexp.replace_all(&input, " ").to_string()
}

/// Replaces multi-spacing with a single space character.
pub(crate) fn replace_multi_spaces_with_single_space(input: String) -> String {
    let multi_spacing_regex = Regex::new(r" +").expect("invalid regexp");
    multi_spacing_regex
        .replace_all(&input, " ")
        .trim()
        .to_string()
}
