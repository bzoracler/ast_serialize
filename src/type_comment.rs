//! Parse type comments from Python source code

/// Individual type comment found in a comment line
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeComment {
    /// A type: ignore with optional error codes
    Ignore(Vec<String>),
    /// A type annotation (e.g., `list[int]`)
    TypeAnnotation(String),
}

/// Parse a type comment and extract all parts (type annotation and/or type: ignore).
///
/// # Arguments
///
/// * `comment` - The comment string to parse (should include the leading `#`)
///
/// # Returns
///
/// - `Some(Vec<TypeComment>)` with one or more parts if valid type comment(s) found
/// - `None` if it's not a type comment
///
/// # Examples
///
/// ```
/// use mypy_parser::type_comment::{parse_type_comments, TypeComment};
///
/// // Pure type: ignore
/// let result = parse_type_comments("# type: ignore").unwrap();
/// assert_eq!(result.len(), 1);
///
/// // Type annotation with type: ignore on same line
/// let result = parse_type_comments("# type: int  # type: ignore[arg-type]").unwrap();
/// assert_eq!(result.len(), 2);  // Both annotation and ignore
/// ```
pub fn parse_type_comments(comment: &str) -> Option<Vec<TypeComment>> {
    let mut parts = Vec::new();

    // Remove leading '#' and whitespace
    let trimmed = comment.trim_start_matches('#').trim_start();

    // Check if it starts with "type:"
    if !trimmed.starts_with("type:") {
        return None;
    }

    // Get the part after "type:"
    let after_type = trimmed["type:".len()..].trim_start();

    // Check if it's a type: ignore comment (without type annotation)
    if after_type.starts_with("ignore") {
        // Check if "ignore" is followed by whitespace, '[', or end of string
        let after_ignore = &after_type["ignore".len()..];
        if after_ignore.is_empty()
            || after_ignore.starts_with(|c: char| c.is_whitespace() || c == '[')
        {
            // Parse as type: ignore
            let after_ignore_trimmed = after_ignore.trim_start();

            // Check if there are error codes in brackets
            if after_ignore_trimmed.starts_with('[') {
                // Ensure only whitespace was between 'ignore' and '['
                let whitespace_between =
                    &after_ignore[..after_ignore.len() - after_ignore_trimmed.len()];
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

                    parts.push(TypeComment::Ignore(error_codes));
                    return Some(parts);
                }
            }

            // No error codes specified (just "# type: ignore")
            parts.push(TypeComment::Ignore(Vec::new()));
            return Some(parts);
        }
    }

    // Parse type annotation, stopping at the next '#'
    let (type_annotation, remainder) = if let Some(hash_pos) = after_type.find('#') {
        // There's another comment after the type annotation
        (after_type[..hash_pos].trim_end(), Some(&after_type[hash_pos..]))
    } else {
        // No trailing comment
        (after_type.trim_end(), None)
    };

    if !type_annotation.is_empty() {
        parts.push(TypeComment::TypeAnnotation(type_annotation.to_string()));
    }

    // Check if there's a "# type: ignore" in the remainder
    if let Some(remainder_str) = remainder {
        // Recursively parse the remainder to check for type: ignore
        if let Some(remainder_parts) = parse_type_comments(remainder_str) {
            // Add any ignore parts found
            for part in remainder_parts {
                if matches!(part, TypeComment::Ignore(_)) {
                    parts.push(part);
                }
            }
        }
    }

    if parts.is_empty() {
        None
    } else {
        Some(parts)
    }
}

/// Parse a type comment and extract error codes if it contains a type ignore comment.
///
/// # Arguments
///
/// * `comment` - The comment string to parse (should include the leading `#`)
///
/// # Returns
///
/// `Some(Vec<String>)` containing error codes if it contains a type ignore comment,
/// `None` if it doesn't contain a type ignore comment.
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
/// assert_eq!(parse_type_comment("# type: int  # type: ignore[override]"), Some(vec!["override".to_string()]));
/// assert_eq!(parse_type_comment("# regular comment"), None);
/// ```
pub fn parse_type_comment(comment: &str) -> Option<Vec<String>> {
    let parts = parse_type_comments(comment)?;
    // Find the first Ignore part
    for part in parts {
        if let TypeComment::Ignore(codes) = part {
            return Some(codes);
        }
    }
    None
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
            Some(vec![
                "name-defined".to_string(),
                "no-untyped-def".to_string()
            ])
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

    #[test]
    fn test_type_comment_kind_ignore() {
        let result = parse_type_comments("# type: ignore").unwrap();
        assert_eq!(result.len(), 1);
        assert!(matches!(&result[0], TypeComment::Ignore(codes) if codes.is_empty()));

        let result = parse_type_comments("# type: ignore[arg-type]").unwrap();
        assert_eq!(result.len(), 1);
        assert!(matches!(&result[0], TypeComment::Ignore(codes) if codes == &vec!["arg-type".to_string()]));
    }

    #[test]
    fn test_type_comment_kind_annotation() {
        let result = parse_type_comments("# type: int").unwrap();
        assert_eq!(result.len(), 1);
        assert!(matches!(&result[0], TypeComment::TypeAnnotation(s) if s == "int"));

        let result = parse_type_comments("# type: list[int]").unwrap();
        assert_eq!(result.len(), 1);
        assert!(matches!(&result[0], TypeComment::TypeAnnotation(s) if s == "list[int]"));

        let result = parse_type_comments("# type: Dict[str, int]").unwrap();
        assert_eq!(result.len(), 1);
        assert!(matches!(&result[0], TypeComment::TypeAnnotation(s) if s == "Dict[str, int]"));
    }

    #[test]
    fn test_type_comment_kind_annotation_with_trailing_comment() {
        let result = parse_type_comments("# type: int  # This is a comment").unwrap();
        assert_eq!(result.len(), 1);
        assert!(matches!(&result[0], TypeComment::TypeAnnotation(s) if s == "int"));

        let result = parse_type_comments("# type: list[int] # comment").unwrap();
        assert_eq!(result.len(), 1);
        assert!(matches!(&result[0], TypeComment::TypeAnnotation(s) if s == "list[int]"));
    }

    #[test]
    fn test_type_comment_kind_annotation_with_type_ignore() {
        // Type annotation followed by type: ignore on the same line
        let result = parse_type_comments("# type: str # type: ignore").unwrap();
        assert_eq!(result.len(), 2);
        assert!(matches!(&result[0], TypeComment::TypeAnnotation(s) if s == "str"));
        assert!(matches!(&result[1], TypeComment::Ignore(codes) if codes.is_empty()));

        let result = parse_type_comments("# type: list[int] # type: ignore[arg-type]").unwrap();
        assert_eq!(result.len(), 2);
        assert!(matches!(&result[0], TypeComment::TypeAnnotation(s) if s == "list[int]"));
        assert!(matches!(&result[1], TypeComment::Ignore(codes) if codes == &vec!["arg-type".to_string()]));
    }

    #[test]
    fn test_type_comment_kind_not_type_comment() {
        assert_eq!(parse_type_comments("# regular comment"), None);
        assert_eq!(parse_type_comments("# TODO: fix this"), None);
        assert_eq!(parse_type_comments("# type:"), None); // Empty annotation
    }

    #[test]
    fn test_type_comment_kind_whitespace_handling() {
        let result = parse_type_comments("#type: int").unwrap();
        assert_eq!(result.len(), 1);
        assert!(matches!(&result[0], TypeComment::TypeAnnotation(s) if s == "int"));

        let result = parse_type_comments("#  type:  int  ").unwrap();
        assert_eq!(result.len(), 1);
        assert!(matches!(&result[0], TypeComment::TypeAnnotation(s) if s == "int"));
    }
}
