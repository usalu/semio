use super::*;
use std::fs;

fn temp_root(name: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let dir = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map_or_else(std::env::temp_dir, PathBuf::from).join(format!("semio-repo-domain-{name}-{nanos}"));
    fs::create_dir_all(&dir).expect("create temp root");
    dir
}

#[test]
fn repo_actions_run_in_process_against_the_domain_crates() {
    let tmp = temp_root("in-process");
    let catalog = RepoAction::StatutesCatalog.execute(&tmp);
    assert!(catalog.lines().count() > 1, "statute catalog is empty: {catalog}");
    assert!(RepoAction::TreeStatute.execute(&tmp).contains('\n'));
    assert!(RepoAction::TreeTerritory.execute(&tmp).contains('\n'));
    assert_eq!(RepoAction::GoalsList.execute(&tmp), "");
    assert!(RepoAction::TicketShow { id: "26/09/06/ABSENT".into() }.execute(&tmp).contains("ticket not found"));
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn repo_action_carries_its_go_argv() {
    assert_eq!(RepoAction::GoalsTree.go_argv(), vec!["goal", "tree"]);
    assert_eq!(RepoAction::TreeMonorepo.go_argv(), vec!["tree", "monorepo"]);
    assert_eq!(RepoAction::Analyze { scope: "rust".into() }.go_argv(), vec!["analyze", "rust"]);
    assert_eq!(RepoAction::TicketShow { id: "26/09/06/X".into() }.go_argv(), vec!["ticket", "show", "26/09/06/X"]);
    assert_eq!(RepoAction::TicketClose { id: "26/09/06/X".into() }.go_argv(), vec!["ticket", "close", "26/09/06/X", "--summary", DASHBOARD_CLOSE_SUMMARY, "--no-management"]);
}

#[test]
fn every_action_has_a_distinct_key_the_registry_grammar_accepts() {
    let actions = [RepoAction::GoalsList, RepoAction::GoalsTree, RepoAction::TreeMonorepo, RepoAction::TreeGoal, RepoAction::TreeStatute, RepoAction::TreeTerritory, RepoAction::StatutesCatalog, RepoAction::Analyze { scope: "rust".into() }, RepoAction::TicketShow { id: "26/09/06/X".into() }, RepoAction::TicketFiles { id: "26/09/06/X".into() }, RepoAction::TicketClose { id: "26/09/06/X".into() }, RepoAction::TicketReopen { id: "26/09/06/X".into() }];
    let keys: std::collections::BTreeSet<String> = actions.iter().map(action_key).collect();
    assert_eq!(keys.len(), actions.len());
    for key in keys { assert!(crate::registry::CommandId::parse(&format!("repo:{key}")).is_ok_and(|id| matches!(id, crate::registry::CommandId::Repo { .. })), "{key}"); }
}

#[test]
fn the_go_binary_lives_in_the_marked_cache() {
    let root = Path::new("/workspace");
    assert!(go_binary_path(root).starts_with(root.join(REPO_BIN_CACHE_DIR)));
}

#[test]
fn every_scenario_of_the_repo_domain_feature_is_proved_by_a_test() {
    crate::tests::assert_proved(include_str!("../../../🧪️tests/🏛️repo-domain/🥒️.feature"), &[include_str!("🦀️.rs")], &[
        ("Repo actions run in process against the domain crates", &["repo_actions_run_in_process_against_the_domain_crates"]),
        ("A repo action carries the argument list of the Go implementation", &["repo_action_carries_its_go_argv"]),
        ("Every action has a distinct key the registry grammar accepts", &["every_action_has_a_distinct_key_the_registry_grammar_accepts"]),
        ("The Go binary lives in the marked cache", &["the_go_binary_lives_in_the_marked_cache"]),
    ]);
}
