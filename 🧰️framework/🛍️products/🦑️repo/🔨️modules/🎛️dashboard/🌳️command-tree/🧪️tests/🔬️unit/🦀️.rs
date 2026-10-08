use super::*;
use std::fs;

fn temp_root(name: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let dir = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map_or_else(std::env::temp_dir, PathBuf::from).join(format!("semio-command-tree-{name}-{nanos}"));
    fs::create_dir_all(&dir).expect("create temp root");
    dir
}

#[test]
fn segment_key_strips_emoji_prefix() {
    assert_eq!(segment_key("🌊️flow"), "flow");
    assert_eq!(segment_key("📦️packages"), "packages");
}

#[test]
fn playground_renderers_and_examples_select_actual_nx_targets() {
    let root = temp_root("playgrounds");
    fs::create_dir_all(crate::catalog::generated_dir(&root)).unwrap();
    fs::write(crate::catalog::generated_dir(&root).join("🎠️playgrounds.json"), "[]\n").unwrap();
    fs::write(crate::catalog::generated_dir(&root).join("🚀️playgrounds.json"), r#"[{"variant":"demo","pluginId":"plugin","cratePath":"plugin/rs","aliases":[],"ports":{"react":3100,"wgpu":3200},"examples":["🎬️first","🎬️second"]}]"#).unwrap();
    let tree = discover(&root);
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🗣️launch-axes/🔣️.json")).unwrap();
    for renderer in fixture["renderers"].as_array().unwrap() {
        for example in ["all", fixture["example"].as_str().unwrap()] {
            let mut selection = &tree;
            for key in ["dev", "plugin", "demo", renderer["id"].as_str().unwrap()] { selection = selection.children.iter().find(|child| child.key == key).unwrap(); }
            if example != "all" { selection = selection.children.iter().find(|child| child.key == "examples").unwrap(); }
            selection = selection.children.iter().find(|child| child.key == example).unwrap();
            assert!(selection.children.is_empty(), "presentation must not add command questions");
            assert!(selection.leaf.is_some());
            for locale in fixture["locales"].as_array().unwrap() {
                for terminology in fixture["terminologies"].as_array().unwrap() {
                    let Some(CommandLeaf::Process(mut spec)) = selection.leaf.clone() else { panic!("missing process") };
                    let preferences = crate::preferences::Preferences { language: locale.as_str().unwrap().into(), terminology: terminology.as_str().unwrap().into(), ..Default::default() };
                    preferences.bind(&mut spec);
                    let mut args = vec!["nx".to_string(), "run".into(), format!("@semio-tech/framework-os-dev:{}", renderer["target"].as_str().unwrap())];
                    if renderer["id"] == "wgpu-native" && example != "all" { args.extend(["--".into(), "--example".into(), example.into()]); }
                    assert_eq!(spec.args, args);
                    assert!(spec.env.contains(&("SEMIO_LOCKED_LOCALE".into(), locale.as_str().unwrap().into())));
                    assert!(spec.env.contains(&("SEMIO_LOCKED_TERMINOLOGY".into(), terminology.as_str().unwrap().into())));
                    assert!(spec.env.contains(&("S_OS_PORT".into(), renderer["port"].as_str().unwrap().into())));
                    assert_eq!(spec.env.iter().find(|(key,_)| key=="PLAYGROUND_LOCKED_EXAMPLE_ID").map(|(_,value)|value.as_str()), (example!="all").then_some(example));
                }
            }
        }
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn workspace_targets_and_parameterized_scripts_are_all_reachable() {
    let root = temp_root("workspace");
    fs::write(root.join("📋️project.json"), r#"{"name":"workspace","targets":{"build":{},"test":{}}}"#).unwrap();
    fs::write(root.join("package.json"), r#"{"scripts":{"nx":"bun bootstrap nx","test":"bun nx run workspace:test","test:quiz":"bun nx run quiz:test","test:quiz:e2e":"bun nx run quiz:test-e2e","dashboard":"bun nx run repo:run"}}"#).unwrap();
    let tree = discover(&root);
    fn visit(node: &CommandNode, commands: &mut Vec<Vec<String>>) {
        if let Some(CommandLeaf::Process(spec)) = &node.leaf {
            assert!(node.children.is_empty(), "a runnable command must remain selectable");
            assert!(spec.env.contains(&("NX_NATIVE_COMMAND_RUNNER".into(), "false".into())));
            assert!(spec.env.contains(&("NX_TUI".into(), "false".into())));
            commands.push(spec.args.clone());
        }
        for child in &node.children { visit(child, commands); }
    }
    let mut commands = Vec::new();
    visit(&tree, &mut commands);
    assert!(commands.contains(&vec!["nx".into(), "run".into(), "workspace:build".into()]));
    for name in ["test", "test:quiz", "test:quiz:e2e"] { assert!(commands.contains(&vec!["run".into(), name.into()])); }
    assert!(!commands.contains(&vec!["run".into(), "dashboard".into()]));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn discover_builds_verb_first_level() {
    let tmp = temp_root("verbs");
    let pkg = tmp.join("🔨️modules/🧪️demo/📦️packages/🦀️rust");
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
        leaf => panic!("expected a process leaf, got {leaf:?}"),
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

#[test]
fn nx_inferred_targets_are_selectable_without_authored_project_rows() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🌳️inferred-targets/🔣️.json")).unwrap();
    let root = temp_root("inferred");
    fs::create_dir_all(root.join("owner/📦️packages/🦀️rust")).unwrap();
    fs::create_dir_all(root.join(".nx/workspace-data")).unwrap();
    fs::write(root.join(".nx/workspace-data/project-graph.json"), fixture["graph"].to_string()).unwrap();
    let tree = discover(&root);
    fn commands(node: &CommandNode, targets: &mut Vec<String>) {
        if let Some(CommandLeaf::Process(spec)) = &node.leaf { if spec.args.starts_with(&["nx".into(), "run".into()]) { targets.push(spec.args[2].clone()); } }
        for child in &node.children { commands(child, targets); }
    }
    let mut targets = Vec::new(); commands(&tree, &mut targets); targets.sort();
    assert_eq!(targets, fixture["expected"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect::<Vec<_>>());
    fs::remove_dir_all(root).unwrap();
}


#[test]
fn nx_inferred_targets_survive_an_in_progress_graph_publication() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🌳️inferred-targets/🔣️.json")).unwrap();
    let root = temp_root("inferred-publication");
    fs::create_dir_all(root.join("owner/📦️packages/🦀️rust")).unwrap();
    fs::create_dir_all(root.join(".nx/workspace-data")).unwrap();
    let path = root.join(".nx/workspace-data/project-graph.json");
    fs::write(&path, "{").unwrap();
    let writer = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(75));
        fs::write(path, fixture["graph"].to_string()).unwrap();
    });
    let mut trie = TrieNode::default(); collect_inferred_targets(&root, &mut trie, &std::sync::atomic::AtomicBool::new(false));
    writer.join().unwrap();
    let node = trie.into_command_node("root", "semio");
    assert_eq!(node.children.iter().map(|child| child.key.as_str()).collect::<Vec<_>>(), ["build", "task"]);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn launch_configurations_targets_and_scripts_are_filed_under_their_verb() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🚀️launch-configurations/🔣️.json")).unwrap();
    let root = temp_root("launch");
    fs::create_dir_all(root.join(".vscode")).unwrap();
    fs::write(root.join(LAUNCH_CONFIGURATIONS), fixture["launch"].as_str().unwrap()).unwrap();
    let manifest = root.join(fixture["project"]["path"].as_str().unwrap());
    fs::create_dir_all(manifest.parent().unwrap()).unwrap();
    let targets: serde_json::Map<String, serde_json::Value> = fixture["project"]["targets"].as_array().unwrap().iter().map(|target| (target.as_str().unwrap().to_string(), serde_json::json!({}))).collect();
    fs::write(&manifest, serde_json::json!({ "name": fixture["project"]["name"], "targets": targets }).to_string()).unwrap();
    fs::write(root.join("package.json"), serde_json::json!({ "scripts": fixture["scripts"] }).to_string()).unwrap();
    let tree = discover(&root);
    let commands = crate::inventory::commands(&tree);
    let verbs: Vec<&str> = fixture["verbs"].as_array().unwrap().iter().map(|verb| verb.as_str().unwrap()).collect();
    assert_eq!(tree.children.iter().take(verbs.len()).map(|child| child.key.as_str()).collect::<Vec<_>>(), verbs);
    let prefix = root.display().to_string();
    let relative = |value: &str| value.replace(&format!("{prefix}/"), "").replace(&prefix, "");
    let project = |spec: &CommandSpec| {
        let env: serde_json::Map<String, serde_json::Value> = spec.env.iter().filter(|pair| !nx_env().contains(pair)).map(|(key, value)| (key.clone(), relative(value).into())).collect();
        let cwd = relative(&spec.cwd.display().to_string());
        if spec.cmd == "sh" || spec.cmd == "cmd.exe" { serde_json::json!({ "shell": relative(spec.args.last().unwrap()), "cwd": cwd, "env": env }) } else { serde_json::json!({ "cmd": spec.cmd, "args": spec.args.iter().map(|arg| relative(arg)).collect::<Vec<_>>(), "cwd": cwd, "env": env }) }
    };
    for expected in fixture["launched"].as_array().unwrap() {
        let (_, leaf) = commands.iter().find(|(label, _)| label == expected["label"].as_str().unwrap()).unwrap_or_else(|| panic!("missing {}", expected["label"]));
        let processes: Vec<_> = match leaf { CommandLeaf::Process(spec) => vec![project(spec)], CommandLeaf::Compound(specs) => specs.iter().map(project).collect(), CommandLeaf::Repo(action) => panic!("unexpected {action:?}") };
        assert_eq!(serde_json::Value::Array(processes), expected["processes"], "{}", expected["label"]);
        if let CommandLeaf::Process(spec) = leaf { assert!(nx_env().iter().all(|pair| spec.env.contains(pair))); }
    }
    for omitted in fixture["omitted"].as_array().unwrap() { assert!(commands.iter().all(|(label, _)| !label.contains(omitted.as_str().unwrap())), "{omitted} must not be offered"); }
    for search in fixture["searches"].as_array().unwrap() {
        let state = ui_tui::tui::widget::WizardState { filter: search["filter"].as_str().unwrap().into(), ..ui_tui::tui::widget::WizardState::new(commands.iter().map(|(label, _)| label.clone()).collect()) };
        let labels: Vec<&str> = state.visible_indices().into_iter().map(|index| state.options[index].as_str()).collect();
        assert_eq!(labels, search["labels"].as_array().unwrap().iter().map(|label| label.as_str().unwrap()).collect::<Vec<_>>(), "{}", search["filter"]);
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn launch_documents_tolerate_comments_and_trailing_commas() {
    assert_eq!(strip_jsonc("{ // note\n \"a\": \"// kept, \\\" */\", /* gone */ \"b\": [1, 2, ], }"), "{ \n \"a\": \"// kept, \\\" */\",  \"b\": [1, 2]}");
}
