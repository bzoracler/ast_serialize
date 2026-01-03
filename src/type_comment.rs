//! Parse type comments from Python source code

/// Parse a type comment and extract error codes if it's a type ignore comment.
///
/// # Arguments
///
/// * `comment` - The comment string to parse (should include the leading `#`)
///
/// # Returns
///
/// `Some(Vec<String>)` containing error codes if it's a type ignore comment,
/// `None` if it's not a type ignore comment.
/// Error codes are parsed from brackets like `[code1, code2]`.
/// Only whitespace is allowed between 'ignore' and '['.
///
/// # Examples
///
/// ```
/// use mypy_parser::type_comment::parse_type_comment;
///
/// assert_eq!(parse_type_comment("# type: ignore"), Some(vec![]));
/// assert_eq!(parse_type_comment("# type: ignore[arg-type]"), Some(vec!["arg-type".to_string()]));
/// assert_eq!(parse_type_comment("# type: ignore [override]"), Some(vec!["override".to_string()]));
/// assert_eq!(parse_type_comment("# type: ignore[arg-type, override]"), Some(vec!["arg-type".to_string(), "override".to_string()]));
/// assert_eq!(parse_type_comment("# regular comment"), None);
/// ```
pub fn parse_type_comment(comment: &str) -> Option<Vec<String>> {
    // Remove leading '#' and whitespace
    let trimmed = comment.trim_start_matches('#').trim_start();

    // Check if it starts with "type: ignore"
    if !trimmed.starts_with("type: ignore") {
        return None;
    }

    // Get the part after "type: ignore"
    let after_ignore = &trimmed["type: ignore".len()..];

    // Trim leading whitespace
    let after_ignore_trimmed = after_ignore.trim_start();

    // Check if there are error codes in brackets
    if after_ignore_trimmed.starts_with('[') {
        // Ensure only whitespace was between 'ignore' and '['
        let whitespace_between = &after_ignore[..after_ignore.len() - after_ignore_trimmed.len()];
        if !whitespace_between.chars().all(char::is_whitespace) {
            return None;
        }

        if let Some(bracket_end) = after_ignore_trimmed.find(']') {
            // Extract the content between brackets
            let codes_str = &after_ignore_trimmed[1..bracket_end];

            // Split by comma and collect error codes
            let error_codes: Vec<String> = codes_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            return Some(error_codes);
        }
    }

    // No error codes specified (just "# type: ignore")
    Some(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_ignore_basic() {
        assert_eq!(parse_type_comment("# type: ignore"), Some(vec![]));
    }

    #[test]
    fn test_type_ignore_with_single_code() {
        assert_eq!(
            parse_type_comment("# type: ignore[arg-type]"),
            Some(vec!["arg-type".to_string()])
        );
        assert_eq!(
            parse_type_comment("# type: ignore[override]"),
            Some(vec!["override".to_string()])
        );
    }

    #[test]
    fn test_type_ignore_with_multiple_codes() {
        assert_eq!(
            parse_type_comment("# type: ignore[arg-type, override]"),
            Some(vec!["arg-type".to_string(), "override".to_string()])
        );
        assert_eq!(
            parse_type_comment("# type: ignore[name-defined,no-untyped-def]"),
            Some(vec!["name-defined".to_string(), "no-untyped-def".to_string()])
        );
    }

    #[test]
    fn test_type_ignore_with_whitespace() {
        assert_eq!(parse_type_comment("#type: ignore"), Some(vec![]));
        assert_eq!(parse_type_comment("#  type: ignore"), Some(vec![]));
        assert_eq!(parse_type_comment("# type: ignore "), Some(vec![]));

        // Whitespace before bracket is ok
        assert_eq!(
            parse_type_comment("# type: ignore [arg-type]"),
            Some(vec!["arg-type".to_string()])
        );

        // Whitespace around codes
        assert_eq!(
            parse_type_comment("# type: ignore[ arg-type , override ]"),
            Some(vec!["arg-type".to_string(), "override".to_string()])
        );
    }

    #[test]
    fn test_not_type_ignore() {
        assert_eq!(parse_type_comment("# regular comment"), None);
        assert_eq!(parse_type_comment("# TODO: fix this"), None);
        assert_eq!(parse_type_comment("# type: int"), None);

        // Non-whitespace between 'ignore' and '[' should fail
        assert_eq!(parse_type_comment("# type: ignore-[arg-type]"), None);
        assert_eq!(parse_type_comment("# type: ignorefoo[arg-type]"), None);
    }

    #[test]
    fn test_empty_comment() {
        assert_eq!(parse_type_comment("#"), None);
        assert_eq!(parse_type_comment(""), None);
    }

    #[test]
    fn test_empty_error_codes() {
        // Empty brackets should return empty vec
        assert_eq!(parse_type_comment("# type: ignore[]"), Some(vec![]));
    }
}
