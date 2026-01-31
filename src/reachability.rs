/// Inferred truth value of an expression during reachability analysis.
///
/// These values match the constants in mypy.reachability:
/// - ALWAYS_TRUE: Expression is always true
/// - MYPY_TRUE: True in mypy, False at runtime
/// - ALWAYS_FALSE: Expression is always false
/// - MYPY_FALSE: False in mypy, True at runtime
/// - TRUTH_VALUE_UNKNOWN: Truth value cannot be determined
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TruthValue {
    AlwaysTrue = 1,
    MypyTrue = 2,
    AlwaysFalse = 3,
    MypyFalse = 4,
    TruthValueUnknown = 5,
}

impl TruthValue {
    /// Returns the inverted truth value (for handling `not` expressions).
    pub fn invert(self) -> Self {
        match self {
            TruthValue::AlwaysTrue => TruthValue::AlwaysFalse,
            TruthValue::AlwaysFalse => TruthValue::AlwaysTrue,
            TruthValue::MypyTrue => TruthValue::MypyFalse,
            TruthValue::MypyFalse => TruthValue::MypyTrue,
            TruthValue::TruthValueUnknown => TruthValue::TruthValueUnknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_values() {
        // Verify the enum values match the Python constants
        assert_eq!(TruthValue::AlwaysTrue as u8, 1);
        assert_eq!(TruthValue::MypyTrue as u8, 2);
        assert_eq!(TruthValue::AlwaysFalse as u8, 3);
        assert_eq!(TruthValue::MypyFalse as u8, 4);
        assert_eq!(TruthValue::TruthValueUnknown as u8, 5);
    }

    #[test]
    fn test_invert() {
        assert_eq!(TruthValue::AlwaysTrue.invert(), TruthValue::AlwaysFalse);
        assert_eq!(TruthValue::AlwaysFalse.invert(), TruthValue::AlwaysTrue);
        assert_eq!(TruthValue::MypyTrue.invert(), TruthValue::MypyFalse);
        assert_eq!(TruthValue::MypyFalse.invert(), TruthValue::MypyTrue);
        assert_eq!(
            TruthValue::TruthValueUnknown.invert(),
            TruthValue::TruthValueUnknown
        );
    }
}
