//! ♿️ The native accessibility bridge replayed from the language-agnostic fixture `🧫️fixtures/♿️native-accessibility-tree`
//! (schema `🧬️schema/♿️native-accessibility-tree`). The third-party oracle is AccessKit's own consumer tree — the model its
//! macOS, Windows and Linux adapters read — which refuses an inconsistent update outright and answers every role, name,
//! state, action and the focus the way a screen reader would be told them.

use super::*;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/♿️native-accessibility-tree/🔣️.json")).expect("native accessibility fixture")
}

fn publication(fixture: &serde_json::Value) -> NativeAccessibilityPublication {
    NativeAccessibilityPublication {
        title: fixture["title"].as_str().expect("title").to_string(),
        windows: fixture["windows"]
            .as_array()
            .expect("windows")
            .iter()
            .map(|window| NativeAccessibilityWindow {
                window_id: window["windowId"].as_str().expect("window id").to_string(),
                window_generation: window["windowGeneration"].as_u64().expect("window generation"),
                nodes: serde_json::from_value(window["nodes"].clone()).expect("projection nodes"),
            })
            .collect(),
    }
}

/// 🔎️ Every node of the consumer tree by its author id, with its parent's author id and its children's.
fn consumer_nodes(tree: &accesskit_consumer::Tree) -> HashMap<String, (Option<String>, Vec<String>, accesskit::Role, Option<String>, accesskit::Node)> {
    fn walk(node: accesskit_consumer::NodeRef<'_>, parent: Option<String>, out: &mut HashMap<String, (Option<String>, Vec<String>, accesskit::Role, Option<String>, accesskit::Node)>) {
        let author = node.data().author_id().map(str::to_string).unwrap_or_else(|| "<root>".to_string());
        let children: Vec<String> = node.children().map(|child| child.data().author_id().unwrap_or_default().to_string()).collect();
        out.insert(author.clone(), (parent, children, node.role(), node.label(), node.data().clone()));
        for child in node.children() {
            walk(child, Some(author.clone()), out);
        }
    }
    let mut out = HashMap::new();
    walk(tree.state().root(), None, &mut out);
    out
}

const ACTIONS: [(&str, accesskit::Action); 4] = [("Click", accesskit::Action::Click), ("Focus", accesskit::Action::Focus), ("SetValue", accesskit::Action::SetValue), ("Blur", accesskit::Action::Blur)];

#[test]
fn the_platform_tree_is_the_fixture_tree_as_accesskit_reads_it() {
    let fixture = fixture();
    let tree = native_accessibility_tree(&publication(&fixture));
    let consumer = accesskit_consumer::Tree::new(tree.update.clone(), true);
    let nodes = consumer_nodes(&consumer);
    let expected = &fixture["expected"];
    let (_, root_children, root_role, root_name, _) = &nodes["<root>"];
    assert_eq!((format!("{root_role:?}"), root_name.as_deref()), (expected["root"]["role"].as_str().unwrap().to_string(), expected["root"]["name"].as_str()));
    assert_eq!(serde_json::json!(root_children), expected["root"]["children"]);
    for pane in expected["panes"].as_array().unwrap() {
        let (_, children, role, _, _) = &nodes[pane["authorId"].as_str().unwrap()];
        assert_eq!((format!("{role:?}"), serde_json::json!(children)), (pane["role"].as_str().unwrap().to_string(), pane["children"].clone()), "{pane}");
    }
    for want in expected["nodes"].as_array().unwrap() {
        let id = want["authorId"].as_str().unwrap();
        let (_, children, role, name, data) = nodes.get(id).unwrap_or_else(|| panic!("{id} is not in the platform tree"));
        assert_eq!(format!("{role:?}"), want["role"].as_str().unwrap(), "{id} role");
        assert_eq!(name.as_deref(), want["name"].as_str(), "{id} name");
        assert_eq!(serde_json::json!(children), want["children"], "{id} children");
        let actions: Vec<&str> = ACTIONS.iter().filter(|(_, action)| data.supports_action(*action)).map(|(name, _)| *name).collect();
        let mut wanted: Vec<&str> = want["actions"].as_array().unwrap().iter().map(|action| action.as_str().unwrap()).collect();
        let mut got = actions.clone();
        wanted.sort_unstable();
        got.sort_unstable();
        assert_eq!(got, wanted, "{id} actions");
        assert_eq!(data.toggled().map(|toggled| format!("{toggled:?}")), want["toggled"].as_str().map(str::to_string), "{id} toggled");
        assert_eq!(data.is_selected(), want["selected"].as_bool(), "{id} selected");
        assert_eq!(data.is_expanded(), want["expanded"].as_bool(), "{id} expanded");
        assert_eq!(data.is_disabled(), want["disabled"].as_bool().unwrap_or(false), "{id} disabled");
        assert_eq!(data.is_read_only(), want["readOnly"].as_bool().unwrap_or(false), "{id} read-only");
        assert_eq!(data.level(), want["level"].as_u64().map(|level| level as usize), "{id} level");
        assert_eq!(data.value(), want["value"].as_str(), "{id} value");
        assert_eq!(data.keyboard_shortcut(), want["shortcut"].as_str(), "{id} shortcut");
        assert_eq!(data.live().map(|live| format!("{live:?}")), want["live"].as_str().map(str::to_string), "{id} live");
        let numeric = [data.min_numeric_value(), data.max_numeric_value(), data.numeric_value()];
        match want["numeric"].as_array() {
            Some(values) => assert_eq!(numeric.map(|value| value.expect("numeric value")).to_vec(), values.iter().map(|value| value.as_f64().unwrap()).collect::<Vec<_>>(), "{id} numeric"),
            None => assert_eq!(numeric, [None, None, None], "{id} numeric"),
        }
    }
    let focus = consumer.state().focus().expect("the platform tree has a focus");
    assert_eq!(focus.data().author_id(), expected["focus"].as_str(), "the focused projection node is the platform focus");
}

#[test]
fn every_projected_role_has_the_platform_role_the_fixture_names() {
    for row in fixture()["roles"].as_array().unwrap() {
        assert_eq!(format!("{:?}", platform_role(row["role"].as_str().unwrap(), false)), row["platform"].as_str().unwrap(), "{row}");
    }
    assert_eq!(platform_role("textbox", true), accesskit::Role::MultilineTextInput);
    assert_eq!(platform_role("marquee", false), accesskit::Role::Unknown, "a role outside the vocabulary is the platform's own unknown");
}

#[test]
fn every_platform_action_is_the_shell_event_the_fixture_names() {
    let fixture = fixture();
    let tree = native_accessibility_tree(&publication(&fixture));
    let node_of = |author: &str| tree.update.nodes.iter().find(|(_, node)| node.author_id() == Some(author)).map(|(id, _)| *id).unwrap_or_else(|| panic!("{author} has no platform node"));
    for case in fixture["actions"].as_array().unwrap() {
        let target_node = case["authorId"].as_str().map_or(ROOT_NODE, node_of);
        let action = match case["action"].as_str().unwrap() {
            "Click" => accesskit::Action::Click,
            "Focus" => accesskit::Action::Focus,
            "Blur" => accesskit::Action::Blur,
            "SetValue" => accesskit::Action::SetValue,
            _ => accesskit::Action::ScrollIntoView,
        };
        let data = case["value"].as_str().map(|value| accesskit::ActionData::Value(value.into()));
        let dispatched = action_dispatch(&tree, &accesskit::ActionRequest { action, target_tree: accesskit::TreeId::ROOT, target_node, data });
        let want = &case["dispatch"];
        match dispatched {
            None => assert!(want.is_null(), "{}: nothing dispatched", case["id"]),
            Some(ui_render::DispatchEvent::Accessibility { target, event }) => {
                let event = match event {
                    ui_render::AccessibilityEvent::Activate => serde_json::json!({ "event": "activate" }),
                    ui_render::AccessibilityEvent::Focus => serde_json::json!({ "event": "focus" }),
                    ui_render::AccessibilityEvent::Blur => serde_json::json!({ "event": "blur" }),
                    ui_render::AccessibilityEvent::Value(value) => serde_json::json!({ "event": "value", "value": value }),
                };
                let mut got = event;
                got["windowId"] = serde_json::json!(target.window_id);
                got["windowGeneration"] = serde_json::json!(target.window_generation);
                got["nodeId"] = serde_json::json!(target.node_id);
                got["nodeKey"] = serde_json::json!(target.node_key);
                assert_eq!(&got, want, "{}", case["id"]);
            }
            Some(other) => panic!("{}: dispatched {other:?}", case["id"]),
        }
    }
}

/// 🔑️ A node keeps its platform id while it exists: re-projecting the same publication, or one that gained a node elsewhere,
/// leaves every id in place — an assistive technology's cursor never jumps because a sibling appeared.
#[test]
fn a_node_keeps_its_platform_id_across_publications() {
    let fixture = fixture();
    let first = publication(&fixture);
    let ids = |tree: &NativeAccessibilityTree| -> HashMap<String, u64> { tree.update.nodes.iter().filter_map(|(id, node)| node.author_id().map(|author| (author.to_string(), id.0))).collect() };
    let before = ids(&native_accessibility_tree(&first));
    assert_eq!(before, ids(&native_accessibility_tree(&first)), "the same publication projects the same ids");
    let mut grown = first.clone();
    let mut added = grown.windows[1].nodes[0].clone();
    added.key = "new-sibling".into();
    added.node_id = 99;
    grown.windows[1].nodes.insert(0, added);
    let after = ids(&native_accessibility_tree(&grown));
    for (author, id) in &before {
        assert_eq!(after.get(author), Some(id), "{author} kept its id");
    }
    assert_eq!(before.len() + 1, after.len());
}

/// 📣️ A publication equal to the last one is no change: the platform tree is pushed again only when the projection moved.
#[test]
fn only_a_changed_publication_advances_the_published_version() {
    let fixture = fixture();
    publish_native_accessibility(publication(&fixture));
    let (version, _) = published_snapshot();
    publish_native_accessibility(publication(&fixture));
    assert_eq!(published_snapshot().0, version, "an equal publication is not a change");
    let mut changed = publication(&fixture);
    changed.title = "Semio (de)".into();
    publish_native_accessibility(changed);
    assert_eq!(published_snapshot().0, version.wrapping_add(1));
}

/// 🧱️ AccessKit stays behind the bridge (AGENTS: an external library behind an interface, no exported third-party type):
/// no renderer source but the bridge module and this law names the crate, so no public signature can carry one of its types.
#[test]
fn only_the_bridge_module_names_accesskit() {
    fn walk(directory: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        for entry in std::fs::read_dir(directory).expect("renderer source directory").flatten() {
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().is_some_and(|name| name != "node_modules") {
                    walk(&path, out);
                }
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                out.push(path);
            }
        }
    }
    let engine = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..").canonicalize().expect("engine root");
    let mut sources = Vec::new();
    walk(&engine, &mut sources);
    let naming: Vec<String> = sources
        .iter()
        .filter(|path| std::fs::read_to_string(path).is_ok_and(|source| source.contains("accesskit")))
        .map(|path| path.strip_prefix(&engine).unwrap_or(path).display().to_string())
        .filter(|path| !path.ends_with("♿️native-accessibility/🦀️.rs"))
        .collect();
    assert!(sources.len() > 100, "the walk reached the renderer sources ({} files)", sources.len());
    assert!(naming.is_empty(), "only the bridge names accesskit: {naming:?}");
}
