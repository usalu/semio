use super::*;
use crate::registry::{Request, RunPolicy};
use std::fs;

fn temp_root(name: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let dir = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map_or_else(std::env::temp_dir, PathBuf::from).join(format!("semio-command-tree-{name}-{nanos}"));
    fs::create_dir_all(&dir).expect("create temp root");
    dir
}

fn leaves(node: &CommandNode, found: &mut Vec<Leaf>) {
    if let Some(leaf) = &node.leaf { found.push(leaf.clone()); }
    for child in &node.children { leaves(child, found); }
}

#[test]
fn playground_renderers_and_examples_select_actual_nx_targets() {
    let root = temp_root("playgrounds");
    fs::create_dir_all(crate::catalog::generated_dir(&root)).unwrap();
    fs::write(crate::catalog::generated_dir(&root).join("🚀️playgrounds.json"), r#"[{"variant":"demo","pluginId":"plugin","cratePath":"plugin/rs","aliases":[],"ports":{"react":3100,"wgpu":3200},"examples":["🎬️first","🎬️second"]}]"#).unwrap();
    let registry = crate::inventory::discover(&root, &std::sync::atomic::AtomicBool::new(false));
    let tree = tree(&registry, true);
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🗣️launch-axes/🔣️.json")).unwrap();
    for renderer in fixture["renderers"].as_array().unwrap() {
        let mut selection = &tree;
        for key in ["dev", "plugin", "demo", renderer["id"].as_str().unwrap()] { selection = selection.children.iter().find(|child| child.key == key).unwrap_or_else(|| panic!("{key} below {}", selection.key)); }
        assert!(selection.children.is_empty(), "presentation must not add command questions");
        let leaf = selection.leaf.clone().expect("a renderer is a leaf");
        assert_eq!(leaf, Leaf { id: "playground:demo".into(), parameters: vec![("renderer".into(), renderer["id"].as_str().unwrap().into())] });
        for example in ["all", fixture["example"].as_str().unwrap()] {
            for locale in fixture["locales"].as_array().unwrap() {
                for terminology in fixture["terminologies"].as_array().unwrap() {
                    let mut parameters = leaf.parameters.clone();
                    parameters.extend([("language".to_string(), locale.as_str().unwrap().to_string()), ("terminology".into(), terminology.as_str().unwrap().into())]);
                    if example != "all" { parameters.push(("example".into(), example.into())); }
                    let launch = registry.resolve_with(&leaf.id, &Request::with_parameters(parameters), RunPolicy::default()).unwrap();
                    let process = &launch.processes[0];
                    let mut args = vec!["nx".to_string(), "run".into(), format!("@semio-tech/framework-os-dev:{}", renderer["target"].as_str().unwrap())];
                    if renderer["id"] == "wgpu-native" && example != "all" { args.extend(["--".into(), "--example".into(), example.into()]); }
                    assert_eq!(process.args, args);
                    let env = |key: &str| process.env.iter().find(|(name, _)| name == key).map(|(_, value)| value.as_str());
                    assert_eq!((env("SEMIO_LOCKED_LOCALE"), env("SEMIO_LOCKED_TERMINOLOGY")), (Some(locale.as_str().unwrap()), Some(terminology.as_str().unwrap())));
                    assert_eq!(env("S_OS_PORT"), Some(renderer["port"].as_str().unwrap()));
                    assert_eq!(env("PLAYGROUND_LOCKED_EXAMPLE_ID"), (example != "all").then_some(example));
                }
            }
        }
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn every_listed_command_is_a_leaf_of_the_tree_and_a_row_of_the_launcher_and_names_a_registry_command() {
    let root = temp_root("workspace");
    fs::write(root.join("📋️project.json"), r#"{"name":"workspace","targets":{"build":{},"test":{}}}"#).unwrap();
    fs::write(root.join("package.json"), r#"{"scripts":{"nx":"bun bootstrap nx","test:quiz":"bun nx run quiz:test","dashboard":"bun nx run repo:run"}}"#).unwrap();
    let registry = crate::inventory::discover(&root, &std::sync::atomic::AtomicBool::new(false));
    let (mut in_tree, rows) = (Vec::new(), launcher(&registry));
    leaves(&tree(&registry, false), &mut in_tree);
    let listed: Vec<&str> = registry.entries().iter().filter(|entry| entry.listed).map(|entry| entry.id.as_str()).collect();
    for id in &listed { assert!(in_tree.iter().any(|leaf| &leaf.id == id) && rows.iter().any(|row| &row.leaf.id == id), "{id}"); }
    for leaf in &in_tree { assert!(registry.find(&leaf.id).is_some(), "{} names no command", leaf.id); }
    assert!(in_tree.iter().any(|leaf| leaf.id == "workspace:build"));
    assert!(in_tree.iter().all(|leaf| !leaf.id.starts_with("script:")), "root scripts are not commands");
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn the_tree_starts_with_the_verbs_in_launcher_order() {
    let tmp = temp_root("verbs");
    let pkg = tmp.join("🔨️modules/🧪️demo/📦️packages/🦀️rust");
    fs::create_dir_all(&pkg).unwrap();
    fs::write(pkg.join("📋️project.json"), r#"{"name":"@semio-tech/demo","targets":{"test":{"executor":"nx:run-commands"},"dev":{"continuous":true},"odd-thing":{}}}"#).unwrap();
    let tree = discover(&tmp);
    let verbs: Vec<&str> = tree.children.iter().map(|child| child.key.as_str()).collect();
    let position = |verb: &str| verbs.iter().position(|known| *known == verb).unwrap_or_else(|| panic!("{verb} in {verbs:?}"));
    assert!(position("dev") < position("test") && position("test") < position("task") && position("task") < position("goals"));
    let demo = tree.children.iter().find(|child| child.key == "test").unwrap();
    assert_eq!(demo.children.iter().map(|child| child.key.as_str()).collect::<Vec<_>>(), ["demo"], "taxonomy noise segments are dropped");
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn the_repo_domain_branches_carry_repo_leaves() {
    let tmp = temp_root("repo-domain");
    let tree = discover(&tmp);
    let verbs: Vec<&str> = tree.children.iter().map(|child| child.key.as_str()).collect();
    for expected in ["goals", "analyze", "tree", "statutes"] { assert!(verbs.contains(&expected), "missing {expected} in {verbs:?}"); }
    let analyze = tree.children.iter().find(|child| child.key == "analyze").expect("analyze branch");
    assert_eq!(analyze.children.len(), crate::repo_domain::ANALYZE_SCOPES.len());
    assert!(analyze.children.iter().all(|child| child.leaf.as_ref().is_some_and(|leaf| leaf.id.starts_with("repo:analyze:"))));
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn tree_json_states_every_leaf_kind_and_the_go_argv_of_repo_leaves() {
    let tmp = temp_root("json");
    let registry = crate::inventory::discover(&tmp, &std::sync::atomic::AtomicBool::new(false));
    let node = CommandNode {
        key: "root".into(),
        label: "semio".into(),
        leaf: None,
        children: vec![
            CommandNode { key: "build".into(), label: "build".into(), children: Vec::new(), leaf: Some(Leaf { id: "workspace:build".into(), parameters: vec![("cache".into(), "skip-all".into())] }) },
            CommandNode { key: "statutes".into(), label: "statutes".into(), children: Vec::new(), leaf: Some(Leaf { id: "repo:statutes.catalog".into(), parameters: Vec::new() }) },
        ],
    };
    let json = tree_json(&registry, &node);
    assert_eq!(json["children"][0]["leaf"], serde_json::json!({ "kind": "command", "id": "workspace:build", "parameters": [["cache", "skip-all"]] }));
    assert_eq!(json["children"][1]["leaf"], serde_json::json!({ "kind": "repo", "action": "statutes.catalog", "goArgv": ["statute", "list"] }));
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn nx_inferred_targets_are_selectable_without_authored_project_rows() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🌳️inferred-targets/🔣️.json")).unwrap();
    let root = temp_root("inferred");
    fs::create_dir_all(root.join("owner/📦️packages/🦀️rust")).unwrap();
    fs::create_dir_all(root.join(".nx/workspace-data")).unwrap();
    fs::write(root.join(".nx/workspace-data/project-graph.json"), fixture["graph"].to_string()).unwrap();
    let mut found = Vec::new();
    leaves(&discover(&root), &mut found);
    let mut targets: Vec<String> = found.into_iter().map(|leaf| leaf.id).filter(|id| !id.starts_with("repo:")).collect();
    targets.sort();
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
    let node = discover(&root);
    writer.join().unwrap();
    assert_eq!(node.children.iter().map(|child| child.key.as_str()).take(2).collect::<Vec<_>>(), ["build", "task"]);
    fs::remove_dir_all(root).unwrap();
}
