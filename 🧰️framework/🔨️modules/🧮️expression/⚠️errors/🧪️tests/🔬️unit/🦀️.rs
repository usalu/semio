use super::*;

fn every_kind() -> Vec<ErrorKind> {
    vec![
        ErrorKind::UnknownParam { name: "w".into() },
        ErrorKind::OperandKind { site: "+", position: 1, expected: Kind::NUMERIC, found: Kind::Bool },
        ErrorKind::MixedKinds { site: "+", left: Kind::Length, right: Kind::Angle },
        ErrorKind::Arity { site: "sqrt", expected: 1, found: 2 },
        ErrorKind::Exponent { base: Kind::Length, exponent: Some(4.0) },
        ErrorKind::Exponent { base: Kind::Length, exponent: None },
        ErrorKind::Branches { then: Kind::Length, otherwise: Kind::Number },
        ErrorKind::DivisionByZero,
        ErrorKind::Domain { site: "sqrt" },
        ErrorKind::Overflow,
        ErrorKind::Cycle { members: vec!["a".into(), "b".into()] },
        ErrorKind::FailedDependency { name: "a".into() },
        ErrorKind::UnknownOverride,
        ErrorKind::DeclaredKind { declared: Kind::Length, found: Kind::Number },
    ]
}

#[test]
fn every_error_has_a_distinct_code_family_and_a_message() {
    let kinds = every_kind();
    let codes: std::collections::BTreeSet<&str> = kinds.iter().map(ErrorKind::code).collect();
    assert_eq!(codes.len(), 13);
    for kind in &kinds {
        assert!(!kind.message().is_empty());
        assert!(kind.code().chars().all(|c| c.is_ascii_lowercase() || c == '-'));
    }
}

#[test]
fn messages_name_the_operands() {
    let mixed = ErrorKind::MixedKinds { site: "+", left: Kind::Length, right: Kind::Angle };
    assert_eq!(mixed.message(), "`+` cannot combine length with angle");
    let operand = ErrorKind::OperandKind { site: "and", position: 0, expected: &[Kind::Bool], found: Kind::Number };
    assert_eq!(operand.message(), "operand 1 of `and` must be bool but is number");
    assert_eq!(ErrorKind::Cycle { members: vec!["a".into(), "b".into()] }.message(), "circular reference between a, b");
}

#[test]
fn located_and_parameter_errors_carry_their_path() {
    assert_eq!(ExprError::at(&[1, 0], ErrorKind::Overflow).path, vec![1, 0]);
    assert!(ExprError::parameter(ErrorKind::UnknownOverride).path.is_empty());
    assert_eq!(ExprError::parameter(ErrorKind::DivisionByZero).code(), "division-by-zero");
    assert_eq!(ExprError::parameter(ErrorKind::DivisionByZero).to_string(), "division by zero");
}
