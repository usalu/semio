use super::{FLAGS, USAGE};

#[test]
fn usage_names_every_native_verb_and_states_that_there_are_no_others() {
    for verb in ["commands", "run", "tasks", "logs", "stop|restart|kill", "open", "daemon", "preferences", "catalog", "plugin registry", "command-tree"] {
        assert!(USAGE.contains(&format!("repo dashboard {verb}")), "{verb} is not documented");
    }
    assert!(USAGE.contains("  repo dashboard [--language"));
    assert!(USAGE.contains("repo dashboard --help") && USAGE.contains("The dashboard has no other verbs"));
    assert!(USAGE.contains("repo dashboard run workspace:test") && USAGE.contains("Human developers start tasks through the dashboard command catalog"));
    assert!(!USAGE.contains("<route>") && !USAGE.contains("forwarded"), "nothing is forwarded to another script");
    assert!(!USAGE.contains("launch"), "no launch file exists any more");
}

#[test]
fn every_verb_with_flags_is_listed_once_and_every_flag_is_spelled_the_way_the_readers_take_it() {
    let verbs: Vec<&str> = FLAGS.iter().map(|(verb, _)| *verb).collect();
    let mut unique = verbs.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), verbs.len());
    for (verb, flags) in FLAGS { for flag in *flags { assert!(flag.chars().all(|character| character.is_ascii_lowercase() || character == '-') && !flag.starts_with('-'), "{verb}: {flag}"); } }
}
