use super::*;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🧫️outcome-code/🔣️.json")).expect("outcome code fixture parses")
}

fn level_of(name: &str) -> crate::diagnostic::Severity {
    match name {
        "info" => crate::diagnostic::Severity::Info,
        "warning" => crate::diagnostic::Severity::Warning,
        "error" => crate::diagnostic::Severity::Error,
        "fatal" => crate::diagnostic::Severity::Fatal,
        other => panic!("unknown fixture level {other}"),
    }
}

/// 📖️ The protocol table walks exactly the language-agnostic vocabulary: same codes, same levels, nothing extra.
#[test]
fn the_vocabulary_table_is_the_fixture() {
    let fixture = fixture();
    assert_eq!(fixture["schema"].as_str(), Some("semio.replication.outcome-code"));
    let declared: Vec<(String, crate::diagnostic::Severity)> = fixture["codes"].as_array().expect("codes").iter().map(|row| (row["code"].as_str().expect("code").to_string(), level_of(row["level"].as_str().expect("level")))).collect();
    let table: Vec<(String, crate::diagnostic::Severity)> = OUTCOME_CODES.iter().map(|(code, level)| (code.to_string(), *level)).collect();
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
    assert_eq!(level_of(fixture["apply"]["level"].as_str().expect("apply level")), crate::diagnostic::Severity::Fatal);
    for code in fixture["apply"]["accepted"].as_array().expect("accepted") {
        let code = code.as_str().expect("accepted code");
        assert_eq!(outcome_code_level(code), Some(crate::diagnostic::Severity::Fatal), "{code}");
    }
    for code in fixture["rejected"].as_array().expect("rejected") {
        let code = code.as_str().expect("rejected code");
        assert_eq!(outcome_code_level(code), None, "{code}");
    }
}

/// 🧭️ A runtime-coded refusal carries the level its code fixes and an empty diff.
#[test]
fn refuse_picks_the_vocabulary_level() {
    for (code, level) in OUTCOME_CODES {
        let outcome = MutationOutcome::<Vec<u8>>::refuse(code, "refused", ["target"]);
        assert!(outcome.diff().is_empty(), "{code}");
        assert_eq!(outcome.messages().iter().map(|message| (message.code.0.as_str(), message.level)).collect::<Vec<_>>(), vec![(code, level)]);
    }
    assert_eq!(MutationOutcome::<Vec<u8>>::refuse("mutation.apply.invalid-base", "refused", ["target"]).worst_level(), Some(crate::diagnostic::Severity::Fatal));
}
