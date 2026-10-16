use super::*;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🧫️outcome-code/🔣️.json")).expect("outcome code fixture parses")
}

fn level_of(name: &str) -> semio_framework_diagnostic::Severity {
    match name {
        "info" => semio_framework_diagnostic::Severity::Info,
        "warning" => semio_framework_diagnostic::Severity::Warning,
        "error" => semio_framework_diagnostic::Severity::Error,
        "fatal" => semio_framework_diagnostic::Severity::Fatal,
        other => panic!("unknown fixture level {other}"),
    }
}

/// 📖️ The protocol table walks exactly the language-agnostic vocabulary: same codes, same levels, nothing extra.
#[test]
fn the_vocabulary_table_is_the_fixture() {
    let fixture = fixture();
    assert_eq!(fixture["schema"].as_str(), Some("semio.replication.outcome-code"));
    let declared: Vec<(String, semio_framework_diagnostic::Severity)> = fixture["codes"].as_array().expect("codes").iter().map(|row| (row["code"].as_str().expect("code").to_string(), level_of(row["level"].as_str().expect("level")))).collect();
    let table: Vec<(String, semio_framework_diagnostic::Severity)> = OUTCOME_CODES.iter().map(|(code, level)| (code.to_string(), *level)).collect();
    assert_eq!(table, declared);
    for (code, level) in OUTCOME_CODES {
        assert_eq!(outcome_code_level(code), Some(level), "{code}");
    }
}

/// 🧱️ The apply family admits every kebab detail at `Fatal`; everything the fixture rejects stays outside.
#[test]
fn apply_family_and_rejected_codes_follow_the_fixture() {
    let fixture = fixture();
    assert_eq!(fixture["apply"]["prefix"].as_str(), Some(APPLY_OUTCOME_CODE_PREFIX));
    assert_eq!(level_of(fixture["apply"]["level"].as_str().expect("apply level")), semio_framework_diagnostic::Severity::Fatal);
    for code in fixture["apply"]["accepted"].as_array().expect("accepted") {
        let code = code.as_str().expect("accepted code");
        assert_eq!(outcome_code_level(code), Some(semio_framework_diagnostic::Severity::Fatal), "{code}");
    }
    for code in fixture["rejected"].as_array().expect("rejected") {
        let code = code.as_str().expect("rejected code");
        assert_eq!(outcome_code_level(code), None, "{code}");
    }
}

/// 🏷️ The typed vocabulary is the table: every [`OutcomeCode`] spells a distinct fixture code at the fixture's level.
#[test]
fn the_typed_codes_are_the_vocabulary() {
    let fixture = fixture();
    let declared: Vec<(String, semio_framework_diagnostic::Severity)> = fixture["codes"].as_array().expect("codes").iter().map(|row| (row["code"].as_str().expect("code").to_string(), level_of(row["level"].as_str().expect("level")))).collect();
    let typed: Vec<(String, semio_framework_diagnostic::Severity)> = OutcomeCode::ALL.iter().map(|code| (code.as_str().to_string(), code.level())).collect();
    assert_eq!(typed, declared);
    assert_eq!(OutcomeCode::ALL.iter().collect::<std::collections::HashSet<_>>().len(), OutcomeCode::ALL.len());
}

/// 🧭️ A runtime-coded refusal carries exactly its typed code at the level the code fixes, with an empty diff.
#[test]
fn refuse_picks_the_vocabulary_level() {
    for code in OutcomeCode::ALL {
        let outcome = MutationOutcome::<Vec<u8>>::refuse(code, "refused", ["target"]);
        assert!(outcome.diff().is_empty(), "{code:?}");
        assert_eq!(outcome.messages().iter().map(|message| (message.code.0.as_str(), message.level, message.target.clone())).collect::<Vec<_>>(), vec![(code.as_str(), code.level(), vec!["target".to_string()])]);
        assert_eq!(outcome_code_level(code.as_str()), Some(code.level()), "{code:?}");
    }
}

/// 🔤️ The level has one builder spelling, the wire's: `warning` builds `Warning` on both the message and the outcome.
#[test]
fn the_warning_builders_build_the_warning_level() {
    assert_eq!(MutationMessage::warning(OutcomeCode::NoOp.as_str(), "m").level, semio_framework_diagnostic::Severity::Warning);
    assert_eq!(MutationOutcome::<Vec<u8>>::new(Vec::new()).warning(OutcomeCode::NoOp.as_str(), "m").worst_level(), Some(semio_framework_diagnostic::Severity::Warning));
}

