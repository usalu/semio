use super::*;

#[test]
fn seeded_identifiers_are_reproducible() {
    let mut left = SeededEntropy::new(42);
    let mut right = SeededEntropy::new(42);
    let one = Id::new_from(&mut left).unwrap();
    let two = Id::new_from(&mut right).unwrap();
    assert_eq!(one.to_string(), two.to_string());
    assert_eq!(one.to_string().len(), 36);
    assert_eq!(&one.to_string()[14..15], "4");
}

#[test]
fn relative_time_pluralises() {
    assert_eq!(humanize_seconds(1), "1 second ago");
    assert_eq!(humanize_seconds(120), "2 minutes ago");
    assert_eq!(humanize_seconds(-3600), "1 hour from now");
}

#[test]
fn text_default_emojis_get_the_presentation_selector() {
    assert_eq!(emoji_text("⚙"), "⚙\u{FE0F}");
    assert_eq!(emoji_text("🎯\u{FE0F}"), "🎯");
}

#[test]
fn goal_compose_ids_round_trip() {
    let compose = goal_path_to_compose_id("AI-OPTIMIZED-REPO/REPO-CLI");
    assert_eq!(compose_id_to_goal_segments(&compose), vec!["aioptimizedrepo", "repocli"]);
}

#[test]
fn artifact_refs_are_classified_by_prefix() {
    assert_eq!(parse_artifact_ref("💻️a/b.go").kind, "file");
    assert_eq!(parse_artifact_ref("🔖️a/b.go#Header").section_parts, vec!["Header"]);
    assert_eq!(parse_artifact_ref("a/b/").kind, "folder");
}
