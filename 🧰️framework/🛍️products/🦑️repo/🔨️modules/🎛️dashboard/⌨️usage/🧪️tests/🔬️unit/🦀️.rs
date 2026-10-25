use super::{FLAGS, TEXTS, USAGE_DE, USAGE_EN};

const USAGE: &str = USAGE_EN;

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

#[test]
fn the_german_usage_names_the_same_commands_and_flags_in_the_same_order_as_the_english() {
    let commands = |text: &str| -> Vec<String> { text.lines().skip(1).filter(|line| line.trim_start().starts_with("repo dashboard ")).map(|line| line.trim_start().split("   ").next().unwrap_or_default().to_string()).collect() };
    let flags = |text: &str| -> Vec<String> { text.split("--").skip(1).map(|rest| rest.chars().take_while(|character| character.is_ascii_lowercase() || *character == '-').collect::<String>()).collect() };
    assert_eq!(commands(USAGE_EN), commands(USAGE_DE));
    assert_eq!(flags(USAGE_EN), flags(USAGE_DE));
    assert_eq!(TEXTS.iter().map(|(code, _)| *code).collect::<Vec<_>>(), ["en", "de"]);
    assert!(USAGE_DE.contains("Verwendung:") && !USAGE_DE.contains("Usage:") && !USAGE_DE.contains("The dashboard has no other verbs"));
    assert_eq!(super::text("de"), Some(USAGE_DE));
    assert_eq!(super::text("en"), Some(USAGE_EN));
    assert_eq!(super::text("fr"), None);
}
