use super::*;
use std::fs;

fn temp_root(name: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let dir = std::env::temp_dir().join(format!("semio-command-tree-{name}-{nanos}"));
    fs::create_dir_all(&dir).expect("create temp root");
    dir
}

#[test]
fn segment_key_strips_emoji_prefix() {
    assert_eq!(segment_key("🌊️flow"), "flow");
    assert_eq!(segment_key("📦️packages"), "packages");
}

#[test]
fn discover_builds_verb_first_level() {
    let tmp = temp_root("verbs");
    let pkg = tmp.join("✏️s/🔌️plugins/demo/📦️packages/🦀️rust");
    fs::create_dir_all(&pkg).unwrap();
    fs::write(pkg.join("📋️project.json"), r#"{"name":"@semio-tech/demo","targets":{"test":{"executor":"nx:run-commands"}}}"#).unwrap();
    let tree = discover(&tmp);
    let verbs: Vec<&str> = tree.children.iter().map(|c| c.key.as_str()).collect();
    assert!(verbs.contains(&"test"));
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn discover_carries_the_repo_domain_branches() {
    let tmp = temp_root("repo-domain");
    let tree = discover(&tmp);
    let verbs: Vec<&str> = tree.children.iter().map(|c| c.key.as_str()).collect();
    for expected in ["goals", "analyze", "tree", "statutes"] {
        assert!(verbs.contains(&expected), "missing {expected} in {verbs:?}");
    }
    let analyze = tree.children.iter().find(|child| child.key == "analyze").expect("analyze branch");
    assert_eq!(analyze.children.len(), ANALYZE_SCOPES.len());
    assert!(matches!(analyze.children[0].leaf, Some(CommandLeaf::Repo(_))));
    fs::remove_dir_all(&tmp).ok();
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
fn go_implementation_projects_process_leaves_at_the_go_binary() {
    let tmp = temp_root("go-leaves");
    let mut trie = TrieNode::default();
    inject_repo_domain(&tmp, &mut trie, RepoImplementation::Go);
    let node = trie.into_command_node("root", "semio");
    let statutes = node.children.iter().find(|child| child.key == "statutes").expect("statutes branch");
    let leaf = statutes.children[0].leaf.clone().expect("leaf");
    match leaf {
        CommandLeaf::Process(spec) => {
            assert_eq!(spec.cmd, go_binary_path(&tmp).display().to_string());
            assert_eq!(spec.args, vec!["statute", "list"]);
        }
        CommandLeaf::Repo(action) => panic!("expected a process leaf, got {action:?}"),
    }
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn repo_action_carries_its_go_argv() {
    assert_eq!(RepoAction::GoalsTree.go_argv(), vec!["goal", "tree"]);
    assert_eq!(RepoAction::TreeMonorepo.go_argv(), vec!["tree", "monorepo"]);
    assert_eq!(RepoAction::Analyze { scope: "rust".into() }.go_argv(), vec!["analyze", "rust"]);
    assert_eq!(RepoAction::TicketShow { id: "26/09/06/X".into() }.go_argv(), vec!["ticket", "show", "26/09/06/X"]);
    assert_eq!(
        RepoAction::TicketClose { id: "26/09/06/X".into() }.go_argv(),
        vec!["ticket", "close", "26/09/06/X", "--summary", DASHBOARD_CLOSE_SUMMARY, "--no-management"]
    );
}

#[test]
fn tree_json_states_every_leaf_kind() {
    let root = Path::new("/repo");
    let node = CommandNode {
        key: "root".into(),
        label: "semio".into(),
        leaf: None,
        children: vec![
            CommandNode { key: "build".into(), label: "build".into(), children: Vec::new(), leaf: Some(CommandLeaf::Process(CommandSpec { cmd: "bun".into(), args: vec!["nx".into()], cwd: PathBuf::from("/repo"), env: Vec::new() })) },
            CommandNode { key: "statutes".into(), label: "statutes".into(), children: Vec::new(), leaf: Some(CommandLeaf::Repo(RepoAction::StatutesCatalog)) },
        ],
    };
    let json = tree_json(root, &node);
    assert_eq!(json["children"][0]["leaf"]["kind"], "process");
    assert_eq!(json["children"][1]["leaf"]["kind"], "repo");
    assert_eq!(json["children"][1]["leaf"]["action"], "statutes.catalog");
}
