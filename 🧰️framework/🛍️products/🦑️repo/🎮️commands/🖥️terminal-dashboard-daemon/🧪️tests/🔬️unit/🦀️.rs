
use super::*;
use std::collections::HashMap;

#[test]
fn unknown_subcommand_returns_usage_without_side_effects() {
    let parsed = ParsedArgs { verb: "daemon".into(), segments: vec!["unknown".into()], flags: HashMap::new() };
    assert_eq!(run(Path::new("."), &parsed), 1);
}
