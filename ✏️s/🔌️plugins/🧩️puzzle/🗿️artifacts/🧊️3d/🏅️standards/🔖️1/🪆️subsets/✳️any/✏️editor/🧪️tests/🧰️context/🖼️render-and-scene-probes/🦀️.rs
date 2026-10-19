/// 🖼️ Render and scene probes: the body, panel and window render drivers, the world scene carrier census and the scene field probes.

/// 🖼️ The rendered body, as JSON — every panel/window assertion navigates this value.
///
/// 🪟️ The ViewModel is addressed at the very window the body key names (`<body>:<windowInstanceId>`,
/// else the main instance), exactly like `dispatch`: a host never asks "render this pane" without
/// saying which pane, and the window-owned config/transient partitions are captured through
/// `ViewModel::window_id` — a bare `ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)` renders every window-owned field at its
/// type default (no camera, no suggestion popup, no engagement scratch), which is a state no real
/// render can be in.
pub async fn render_body(app: &mut Puzzle3dApp, body_key: &str) -> Value {
    let window_id = body_key.split_once(':').map_or(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID, |(_, window)| window);
    app.ensure_window(window_id);
    let view = app.window_view(window_id);
    rendered_body_value(app, body_key, &view).await
}

/// 📌️ One APP-LEVEL PANEL body rendered exactly the way `plugin_refresh_ui` renders it:
/// `ViewModel::for_panel()` over the live session, i.e. with NO `window_id` at all — the projection a
/// panel actually receives, and the one a `render_body`/`render_window_refresh` call can never
/// reproduce because both address a concrete instance. `focused_window_id` is the shell's last-focused
/// pane, the only per-call carrier of "which window is the user looking at" a panel is given.
pub async fn render_panel_body(app: &mut Puzzle3dApp, body_key: &str, focused_window_id: Option<&str>) -> Value {
    if let Some(window_id) = focused_window_id {
        app.ensure_window(window_id);
    }
    let mut view = app.view.clone();
    view.focused_window_id = focused_window_id.map(str::to_string);
    rendered_body_value(app, body_key, &view.for_panel()).await
}

/// 🪟️ One window INSTANCE rendered exactly the way `plugin_refresh_ui` renders it: the window KIND's
/// own body key — the host sends `windowKinds[].n` for every instance, never a `<body>:<instance>`
/// spelling — with the view narrowed at that one instance through `ViewModel::for_window_instance`.
/// The `<body>:<instance>` form [`render_window`] uses is the OTHER host route
/// (`plugin_render_surface`'s bound surface context), so a law about what a split pane publishes has
/// to state THIS shape: it is the only one where the instance id reaches the guest through the view
/// alone.
pub async fn render_window_refresh(app: &mut Puzzle3dApp, body_key: &str, window_id: &str) -> Value {
    app.ensure_window(window_id);
    let view = app.window_view(window_id);
    rendered_body_value(app, body_key, &view).await
}

/// 🖼️ The world-3d scene a rendered tree publishes, else the projected node JSON — the one projection
/// every render probe in this test context reads. The tree stays owned by the app's stationary render source; the
/// scene is read in place and only the node JSON is a second bounded render.
async fn rendered_body_value(app: &mut Puzzle3dApp, body_key: &str, view: &ViewModel) -> Value {
    let (raw, cx) = app.parts();
    let scene = artifact_app_laws::observe_registered_fixture_app(raw, body_key, view, cx, |root| {
        let mut stack = vec![root];
        let mut rendered_scene = None;
        while let Some(node) = stack.pop() {
            if let semio_framework_ui_contract::Component::Surface(surface) = &node.component {
                if surface.doc_schema.as_str() == <semio_framework_ui_scene::World3dScene as semio_framework_ui_scene::SceneDoc>::SCHEMA {
                    let scene: semio_framework_ui_scene::World3dScene = artifact_app_laws::built_surface_scene(node).expect("assemble world scene");
                    let world3d = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&scene));
                    rendered_scene = Some(object([("schema".to_string(), Value::from(surface.doc_schema.as_str())), ("world3d".to_string(), world3d)]));
                    if scene.interaction_json.is_some() {
                        break;
                    }
                }
            }
            stack.extend(node.children.iter());
        }
        rendered_scene
    })
    .await
    .expect("render");
    if let Some(scene) = scene {
        return scene;
    }
    let (raw, cx) = app.parts();
    let projected = artifact_app_laws::render_registered_fixture_app(raw, body_key, view, cx).await.expect("render projection");
    parse(&projected, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("rendered node JSON")
}

/// 📏️ The packed byte length of the world-3d surface payload this body actually admits, plus the
/// fixed capacity it is admitted against — the two numbers `scene-surface.encode` compares. The
/// payload is read back off the built `Surface` node rather than re-encoded, so the law measures the
/// same bytes the host shipped.
pub async fn world_surface_payload_bytes(app: &mut Puzzle3dApp, body_key: &str) -> (usize, usize) {
    let window_id = body_key.split_once(':').map_or(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID, |(_, window)| window);
    app.ensure_window(window_id);
    let view = app.window_view(window_id);
    let (raw, cx) = app.parts();
    let widest = artifact_app_laws::observe_registered_fixture_app(raw, body_key, &view, cx, |root| {
        let mut stack = vec![root];
        let mut widest = 0;
        while let Some(node) = stack.pop() {
            if let semio_framework_ui_contract::Component::Surface(surface) = &node.component {
                if surface.doc_schema.as_str() == <semio_framework_ui_scene::World3dScene as semio_framework_ui_scene::SceneDoc>::SCHEMA {
                    widest = widest.max(surface.doc.bytes.as_slice().len());
                }
            }
            stack.extend(node.children.iter());
        }
        widest
    })
    .await
    .expect("render");
    assert!(widest > 0, "{body_key} rendered no world-3d surface");
    (widest, semio_framework_ui_contract::UI_FIXED_BYTES)
}

fn count_built_nodes(node: &semio_framework_ui_contract::BuiltNode) -> usize {
    1 + node.children.iter().map(count_built_nodes).sum::<usize>()
}

//#region 🚚️WorldSceneCarrierCensus
/// 🚚️ One out-of-doc payload lane as it was actually published: the carrier's measured shape next to
/// what the spine declared about it.
pub struct WorldSceneLaneCensus {
    pub key: String,
    pub bytes: usize,
    pub leaves: usize,
    pub leaf_depth: usize,
    pub widest_children: usize,
    pub widest_leaf: usize,
    pub declared_bytes: u32,
    pub declared_hash: String,
}

/// 🚚️ Everything the paged-carrier laws measure about one rendered world body: the spine that rode
/// inside the fixed-capacity surface doc, every lane carrier beside it, and the scene a render host
/// reassembles from the two.
pub struct WorldSceneCarrierCensus {
    pub doc_bytes: usize,
    pub capacity: usize,
    pub nodes: usize,
    pub lanes: Vec<WorldSceneLaneCensus>,
    pub assembled: semio_framework_ui_scene::World3dScene,
}

impl WorldSceneCarrierCensus {
    pub fn payload_bytes(&self) -> usize {
        self.lanes.iter().map(|lane| lane.bytes).sum()
    }

    pub fn lane(&self, key: &str) -> Option<&WorldSceneLaneCensus> {
        self.lanes.iter().find(|lane| lane.key == key)
    }

    pub fn report(&self) -> String {
        self.lanes.iter().map(|lane| format!("{}={}B/{}leaves", lane.key.trim_start_matches(semio_framework_ui_scene::WORLD3D_SCENE_LANE_KEY_PREFIX), lane.bytes, lane.leaves)).collect::<Vec<_>>().join(" ")
    }
}

/// 🚚️ Renders `body_key` and censuses the world-3d surface it publishes. Reads the built tree
/// directly — never a re-encode — so the law measures exactly the bytes the host shipped.
pub async fn world_surface_carrier_census(app: &mut Puzzle3dApp, body_key: &str) -> WorldSceneCarrierCensus {
    let window_id = body_key.split_once(':').map_or(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID, |(_, window)| window);
    app.ensure_window(window_id);
    let view = app.window_view(window_id);
    let (raw, cx) = app.parts();
    let census = artifact_app_laws::observe_registered_fixture_app(raw, body_key, &view, cx, |root| {
    let mut stack = vec![root];
    let mut census = None;
    while let Some(node) = stack.pop() {
        if let semio_framework_ui_contract::Component::Surface(surface) = &node.component {
            if surface.doc_schema.as_str() == <semio_framework_ui_scene::World3dScene as semio_framework_ui_scene::SceneDoc>::SCHEMA {
                let spine: semio_framework_ui_scene::World3dScene = semio_framework_ui_scene::decode(surface).expect("decode world spine");
                let mut lanes = Vec::new();
                for child in node.children.iter() {
                    let payload = artifact_app_laws::built_carrier_text(child);
                    let (mut leaves, mut leaf_depth, mut widest_children, mut widest_leaf) = (0usize, 0usize, 0usize, 0usize);
                    let mut frontier = vec![(child, 0usize)];
                    while let Some((current, level)) = frontier.pop() {
                        widest_children = widest_children.max(current.children.len());
                        if let semio_framework_ui_contract::Component::Text(text) = &current.component {
                            leaves += 1;
                            leaf_depth = leaf_depth.max(level);
                            widest_leaf = widest_leaf.max(text.value.0.as_str().len());
                        }
                        frontier.extend(current.children.iter().map(|grandchild| (grandchild, level + 1)));
                    }
                    let reference = spine.lanes.iter().find(|reference| semio_framework_ui_scene::World3dSceneLane::from_name(&reference.lane).is_some_and(|lane| lane.body_key() == child.key.as_str()));
                    lanes.push(WorldSceneLaneCensus {
                        key: child.key.as_str().to_string(),
                        bytes: payload.len(),
                        leaves,
                        leaf_depth,
                        widest_children,
                        widest_leaf,
                        declared_bytes: reference.map_or(0, |reference| reference.bytes),
                        declared_hash: reference.map_or_else(String::new, |reference| reference.hash.clone()),
                    });
                }
                census = Some(WorldSceneCarrierCensus { doc_bytes: surface.doc.bytes.as_slice().len(), capacity: semio_framework_ui_contract::UI_FIXED_BYTES, nodes: count_built_nodes(node), lanes, assembled: artifact_app_laws::built_surface_scene(node).expect("assemble world scene") });
                break;
            }
        }
        stack.extend(node.children.iter());
    }
    census
    })
    .await
    .expect("render");
    census.unwrap_or_else(|| panic!("{body_key} rendered no world-3d surface"))
}
//#endregion 🚚️WorldSceneCarrierCensus

/// 🪟️ The world composite body for one window INSTANCE — the `<body>:<windowInstanceId>` form is
/// how a split pane asks for its own materialized options (see `ArtifactApp::render`).
pub async fn render_window(app: &mut Puzzle3dApp, window_id: &str) -> Value {
    render_body(app, &format!("{}:{window_id}", main::BODY_KEY)).await
}

pub async fn render_composite(app: &mut Puzzle3dApp) -> Value {
    render_body(app, main::BODY_KEY).await
}

pub fn projection_of(app: &Puzzle3dApp) -> Value {
    semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(app.snapshot().expect("projection").typed()))
}

pub fn object_count(app: &Puzzle3dApp) -> usize {
    projection_of(app).get("objects").and_then(|value| value.as_array()).map(Vec::len).unwrap_or(0)
}

pub fn first_object_id(app: &Puzzle3dApp) -> String {
    projection_of(app).get("objects").and_then(Value::as_array).and_then(|objects| objects.first()).and_then(|object| object.get("id")).and_then(Value::as_str).expect("first object id").to_string()
}

pub fn vortex_full_ids(app: &Puzzle3dApp) -> Vec<String> {
    let projection = projection_of(app);
    let mut ids = Vec::new();
    for object in projection.get("objects").and_then(Value::as_array).into_iter().flatten() {
        let object_id = object.get("id").and_then(Value::as_str).unwrap_or_default();
        for vortex in object.get("vortices").and_then(Value::as_array).into_iter().flatten() {
            if let Some(vortex_id) = vortex.get("id").and_then(Value::as_str) {
                ids.push(puzzle3d_vortex_full_id(object_id, vortex_id));
            }
        }
    }
    ids
}

pub fn first_vortex_full_id(app: &Puzzle3dApp) -> String {
    vortex_full_ids(app).into_iter().next().expect("seed vortex")
}

//#region 🔖️SceneProbes
fn scene_field(node: &Value, field: &str) -> Value {
    node.pointer(&format!("/world3d/{field}")).and_then(Value::as_str).and_then(|raw| parse(raw, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()).unwrap_or(Value::Null)
}

pub fn instances_of(node: &Value) -> Vec<Value> {
    scene_field(node, "instancesJson").as_array().cloned().unwrap_or_default()
}

pub fn instance_count(node: &Value) -> usize {
    instances_of(node).len()
}

pub fn scene_meshes_of(node: &Value) -> Vec<Value> {
    scene_field(node, "meshesJson").as_array().cloned().unwrap_or_default()
}

pub fn vortices_of(node: &Value) -> Vec<Value> {
    scene_field(node, "vorticesJson").as_array().cloned().unwrap_or_default()
}

pub fn interaction_of(node: &Value) -> Value {
    scene_field(node, "interactionJson")
}

pub fn selection_of(node: &Value) -> Value {
    scene_field(node, "selectionJson")
}

pub fn lod_of(node: &Value) -> Value {
    scene_field(node, "lodJson")
}

pub fn camera_of(node: &Value) -> Value {
    scene_field(node, "cameraJson")
}

//#endregion 🔖️SceneProbes
