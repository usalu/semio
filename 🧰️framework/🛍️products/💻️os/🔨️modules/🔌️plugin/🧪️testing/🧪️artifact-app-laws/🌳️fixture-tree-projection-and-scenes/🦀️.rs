//! 🌳️ Fixture tree projection and scenes: the fixture tree limits, the node projection and retirement, the document input commit findings, the fixture scene decode and the built surface and carrier text. See [`super`].

use super::*;

pub const FIXTURE_TREE_MAX_DEPTH: usize = 64;

pub const FIXTURE_TREE_MAX_NODES: usize = semio_framework_ui_contract::UI_BUILT_CHILD_RETIRE_SLOTS;

pub const FIXTURE_TREE_RETIRE_STEPS: usize = FIXTURE_TREE_MAX_NODES * (semio_framework_ui_contract::UI_BUILT_CHILDREN_MAX + 1);

/// 🔗️ The action bindings are part of the surface a host receives — a tree item's
/// `activate` route lives in `BuiltNode.bindings`, never inside its `Component`, so a
/// projection that omitted them showed an inert row and every "this window offers action
/// X" assertion read against a tree that had no actions at all
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// ♿️ …and so is the accessibility spec, for exactly the same reason: a `Component::Surface`
/// paints into a canvas with no accessible children, so the record's own name, description,
/// liveness and `aria-keyshortcuts` are ALL an assistive technology ever learns about it. A
/// projection that omitted them made every "this canvas is named / announces / advertises its
/// chord" assertion read against a tree that carried no accessibility at all
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub(super) fn project_fixture_node(node: &semio_framework_ui_contract::BuiltNode, remaining: &mut usize, depth: usize) -> Result<serde_json::Value, &'static str> {
    if depth >= FIXTURE_TREE_MAX_DEPTH || *remaining == 0 {
        return Err("tree-limit");
    }
    *remaining -= 1;
    if !node.rejected_children.is_empty() {
        return Err("rejected-children");
    }
    let component = serde_json::to_value(&node.component).map_err(|_| "component-json")?;
    if component["type"].as_str().is_none() {
        return Err("component-type");
    }
    let mut children = Vec::with_capacity(node.children.len());
    for (index, child) in node.children.iter().enumerate() {
        if node.children.iter().take(index).any(|previous| previous.key == child.key) {
            return Err("duplicate-key");
        }
        children.push(project_fixture_node(child, remaining, depth + 1)?);
    }
    let bindings = serde_json::to_value(&node.bindings).map_err(|_| "bindings-json")?;
    let accessibility = serde_json::to_value(&node.accessibility).map_err(|_| "accessibility-json")?;
    Ok(serde_json::json!({ "key": node.key.as_str(), "component": component, "bindings": bindings, "disabled": node.disabled, "accessibility": accessibility, "children": children }))
}

/// 🖼️ Observes fixture keys/components/children and retires that exact tree before any outcome; not a wire codec.
pub fn project_and_retire_fixture_tree(tree: semio_framework_ui_runtime::ComponentTree) -> Result<String, &'static str> {
    observe_and_retire_fixture_tree(tree, |root| {
        let mut remaining = FIXTURE_TREE_MAX_NODES;
        project_fixture_node(root, &mut remaining, 0).and_then(|value| serde_json::to_string(&value).map_err(|_| "projection-json"))
    })
}

/// ⌨️ The input commit-mode lint (audit F-4, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): every field of a
/// projected body that would publish one document edit per keystroke — an `Input` of any kind but `number` and
/// `color` (continuous controls, the framework scrub, in both hosts) and `file`, bound through `change` to a verb
/// `is_document_verb` names, without the blur commit policy. Answers `"<node key> → <verb>"` per offender, sorted.
/// Fixture `🔌️plugin/🧫️fixtures/🧫️input-commit-lint/🔣️.json`.
pub fn document_input_commit_findings(projection: &serde_json::Value, is_document_verb: &dyn Fn(&str) -> bool) -> Vec<String> {
    let mut findings = Vec::new();
    let mut stack = vec![projection];
    while let Some(node) = stack.pop() {
        let component = &node["component"];
        if component["type"] == "input" && !matches!(component["kind"].as_str(), Some("number" | "color" | "file")) && component["commit"].as_str() != Some("blur") {
            for binding in node["bindings"].as_array().into_iter().flatten().filter(|binding| binding["trigger"] == "change") {
                if let Some(verb) = binding["action"]["name"].as_str().filter(|verb| is_document_verb(verb)) {
                    findings.push(format!("{} → {verb}", node["key"].as_str().unwrap_or_default()));
                }
            }
        }
        stack.extend(node["children"].as_array().into_iter().flatten());
    }
    findings.sort();
    findings
}

/// ⌨️ [`document_input_commit_findings`] of `body_key` rendered under `view`; a document verb is one declared with the
/// `Mutation` kind.
pub async fn document_input_commit_lint<A: ArtifactApp + Default, M: store::SpaceMember + store::MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>, body_key: &str, view: &semio_framework::ViewModel,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Vec<String>, semio_framework_job::OriginalTurnFailure> {
    let projected = render_registered_fixture_app(app, body_key, view,cx).await?;
    let projection: serde_json::Value = serde_json::from_str(&projected).map_err(|error| semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault(error.to_string()))?;
    Ok(document_input_commit_findings(&projection, &|verb| app.tools.registry.get(verb).is_some_and(|definition| definition.kind == semio_framework::ActionKind::Mutation)))
}

/// 🎬️ The projected fixture node carrying `T`'s scene — the root itself when a body IS a bare
/// surface, otherwise the one surface of that schema anywhere beneath it. A window body is free
/// to compose its scene alongside semantic siblings (generation3d's Flow window renders the
/// graph's node/port/wire rows beside the GPU canvas), and a scene assertion may not care which
/// of the two shapes the body under test happens to be.
pub fn fixture_scene_node<T: semio_framework_ui_scene::SceneDoc>(tree: &serde_json::Value) -> Option<&serde_json::Value> {
    let component = &tree["component"];
    if component["type"].as_str() == Some("surface") && component["docSchema"].as_str() == Some(T::SCHEMA) {
        return Some(tree);
    }
    tree["children"].as_array().and_then(|children| children.iter().find_map(fixture_scene_node::<T>))
}

/// 🎬️ Decodes the exact typed scene a projected fixture carries.
pub fn decode_fixture_scene<T: semio_framework_ui_scene::SceneDoc>(projection: &str) -> Result<T, &'static str> {
    let tree: serde_json::Value = serde_json::from_str(projection).map_err(|_| "fixture-scene-json")?;
    let component = &fixture_scene_node::<T>(&tree).ok_or("fixture-scene-schema")?["component"];
    let bytes = component["doc"]["bytes"].as_array().ok_or("fixture-scene-bytes")?.iter().map(|value| value.as_u64().and_then(|value| u8::try_from(value).ok()).ok_or("fixture-scene-byte")).collect::<Result<Vec<_>, _>>()?;
    T::decode_pack(&bytes).map_err(|_| "fixture-scene-pack")
}

/// 🚚️ The ASSEMBLED scene a render host reads, taken straight off ONE built surface node: the
/// spine decoded out of the fixed-capacity `SurfaceProps.doc`, with every out-of-doc payload
/// lane hanging off that same node merged back in. `semio_framework_ui_scene::decode` alone
/// returns the SPINE — since ticket 26/09/02 wave P a world-3d scene's `instancesJson`/
/// `meshesJson`/… do not ride inside the doc at all, so a test that reads one off a bare
/// decode reads an empty field.
pub fn built_surface_scene<T: semio_framework_ui_scene::SceneDoc>(node: &semio_framework_ui_contract::BuiltNode) -> Result<T, &'static str> {
    let semio_framework_ui_contract::Component::Surface(props) = &node.component else { return Err("built-surface-component") };
    let mut scene: T = semio_framework_ui_scene::decode(props).map_err(|_| "built-surface-decode")?;
    for child in node.children.iter() {
        scene.merge_lane(child.key.as_str(), built_carrier_text(child));
    }
    Ok(scene)
}

/// 🚚️ Depth-first concatenation of every `Component::Text` leaf under one built lane carrier.
pub fn built_carrier_text(node: &semio_framework_ui_contract::BuiltNode) -> String {
    let mut payload = String::new();
    let mut order = Vec::new();
    let mut frontier = vec![node];
    while let Some(current) = frontier.pop() {
        order.push(current);
        frontier.extend(current.children.iter().rev());
    }
    for current in order {
        if let semio_framework_ui_contract::Component::Text(text) = &current.component {
            payload.push_str(&text.packed_payload());
        }
    }
    payload
}

/// 🚚️ Depth-first concatenation of every `text` leaf under a projected fixture node — the exact
/// inverse of [`crate::app::paged_text_carrier`], and the Rust twin of the React Interpreter's
/// own `surfaceSceneLaneText`.
pub fn fixture_carrier_text(node: &serde_json::Value) -> String {
    let mut payload = String::new();
    let mut frontier = vec![node];
    let mut order = Vec::new();
    while let Some(current) = frontier.pop() {
        order.push(current);
        if let Some(children) = current["children"].as_array() {
            for child in children.iter().rev() {
                frontier.push(child);
            }
        }
    }
    for current in order {
        if current["component"]["type"].as_str() == Some("text") {
            let value = current["component"]["value"].as_str().unwrap_or("");
            let mut attributes = Vec::new();
            if let Some(map) = current["component"]["dataAttributes"].as_object() {
                let mut keys: Vec<&String> = map.keys().collect();
                keys.sort();
                for key in keys {
                    if let Some(slice) = map[key].as_str() {
                        attributes.push(slice);
                    }
                }
            }
            payload.push_str(&semio_framework_ui_contract::packed_text_leaf(value, attributes));
        }
    }
    payload
}

/// 🚚️ Decodes a projected surface's typed scene AND re-attaches every out-of-doc payload lane
/// hanging off it — the Rust twin of the React Interpreter's `world3dSurfaceLaneTexts` +
/// `world3dSceneFromLanes`. A child whose key is not a lane carrier is ignored.
pub fn decode_fixture_scene_with_lanes<T: semio_framework_ui_scene::SceneDoc>(projection: &str) -> Result<T, &'static str> {
    let tree: serde_json::Value = serde_json::from_str(projection).map_err(|_| "fixture-scene-json")?;
    let mut scene = decode_fixture_scene::<T>(projection)?;
    for child in fixture_scene_node::<T>(&tree).ok_or("fixture-scene-schema")?["children"].as_array().ok_or("fixture-scene-children")? {
        let key = child["key"].as_str().ok_or("fixture-scene-lane-key")?;
        scene.merge_lane(key, fixture_carrier_text(child));
    }
    Ok(scene)
}

#[doc(hidden)]
pub fn observe_and_retire_fixture_tree<R>(tree: semio_framework_ui_runtime::ComponentTree, project: impl FnOnce(&semio_framework_ui_contract::BuiltNode) -> R) -> R {
    let projected = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| project(&tree.root)));
    let mut retirement = semio_framework_ui_contract::BuiltTreeRetirement::new(tree.root);
    while !retirement.terminal_is_empty() {
        let step = retirement.close_step(1, 4096).expect("fixture's exact built-tree retirement remains valid");
        if !step.progressed {
            std::thread::yield_now();
        }
    }
    match projected {
        Ok(result) => result,
        Err(panic) => std::panic::resume_unwind(panic),
    }
}
