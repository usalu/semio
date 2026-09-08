
use super::*;

#[test]
fn preserves_root_script_verb_and_positional_segments() {
    let parsed = ParsedArgs { verb: "verify".into(), segments: vec!["taxonomy".into(), "report".into()], ..Default::default() };
    assert_eq!(forwarded_segments(&parsed), ["./📜️script.ts", "verify", "taxonomy", "report"]);
}
