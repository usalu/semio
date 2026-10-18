/// 📷️ Fixture round trip, theme, camera and catalogue: the fixture JSON round trip, the shakable document tree, the canvas theme, the minimap, the camera authority law and the catalogue and extras.

#[test]
fn fixture_json_round_trip() {
    let host = FlowHost::default();
    let json = host.host_snapshot_json().unwrap();
    let parsed = FlowHost::parse_host_snapshot_json(&json).unwrap();
    assert_eq!(parsed.schema, "flow.host_snapshot");
    parsed.retire_cold();
    host.retire_cold();
}

#[test]
fn flow_document_tree_is_shakable() {
    let host = host_with_test_fixture();
    let document = host.document();
    assert_eq!(document.schema, "flow.artifact");
    assert!(!document.tree.neurons.is_empty());
    let registry = neural::Registry::new();
    let evaluator = Evaluator::new(&registry);
    let dispatch = |kind: &str, input: &Dictionary| test_math_bridge(kind, input);
    let mut seeds = HashMap::new();
    seeds.insert("slider".into(), channel_output("number", Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(3.0)))));
    let channels = evaluator.evaluate_channels_with(&document.tree, &seeds, &host.kind_infos, &dispatch).unwrap();
    assert_eq!(channels.outputs.get("add").and_then(|d| d.get("sum")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
    channels.retire_cold();
    seeds.retire_cold();
    registry.retire_cold();
    document.retire_cold();
    host.retire_cold();
}

#[test]
fn rebuild_dag_preserves_canvas_theme() {
    use canvas::Color;
    let mut host = FlowHost::default();
    host.dag.viewport.canvas_theme.node_fill = Color::from_rgba8(12, 34, 56, 255);
    host.rebuild_dag();
    assert_eq!(host.dag.viewport.canvas_theme.node_fill.to_rgba8(), Color::from_rgba8(12, 34, 56, 255).to_rgba8());
    host.retire_cold();
}

#[test]
fn set_canvas_theme_dark_applies_board_dark_strokes() {
    let mut host = FlowHost::default();
    host.set_canvas_theme_dark(true);
    let stroke = host.dag.viewport.canvas_theme.node_stroke.to_rgba8();
    assert!(stroke.r > 80 || stroke.g > 80);
    host.set_canvas_theme_dark(false);
    let light_stroke = host.dag.viewport.canvas_theme.node_stroke.to_rgba8();
    assert!(light_stroke.r < 80);
    host.retire_cold();
}

#[test]
fn paint_scene_dark_theme_paints_edges_and_nodes() {
    let mut host = host_with_test_fixture();
    host.set_viewport(1280, 800, 1.0);
    host.set_canvas_theme_dark(true);
    let mut scene = canvas::Scene::new();
    host.paint_scene(&mut scene, 1280, 800, 1.0);
    assert!(scene.path_count() > 8, "populated fixture should paint edges, handles, and node bodies under dark board theme");
    host.retire_cold();
}

#[test]
fn flow_host_enables_minimap_widget_on_dag() {
    let mut host = host_with_test_fixture();
    host.set_viewport(1280, 800, 1.0);
    host.dag.graph.set_camera(200.0, 120.0, 0.65);
    let raw: serde_json::Value = serde_json::from_str(&host.dag.label_overlay_paint_state_json().unwrap()).unwrap();
    assert!(raw.get("minimapWidget").is_some());
    host.retire_cold();
}

//#region 📷️CameraAuthority
/// 📷️ The SURFACE half of the node-graph camera-fit law. The geometry half
/// (`🧰️framework/🔨️modules/🖼️canvas/🧪️tests/📷️camera-fit/🦀️.rs` and the renderer's TypeScript twin) pins WHICH
/// camera a fit computes; these rows pin that the surface actually publishes it. A flow surface
/// carries two camera copies — `FlowHost::fixture.camera`, the authority every projection and every
/// `nodeGraphViewport` publication reads, and `DagHost::fixture.camera`, the derived copy the paint
/// reads — and the wgpu `Fit graph` control published the first while the fit had only landed on the
/// second. Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.
///
/// @see `🧰️framework/🔨️modules/🖼️canvas/🧫️fixtures/📷️camera-fit/🔣️.json` — `surfaceRows`
const CAMERA_FIT_FIXTURE_JSON: &str = include_str!("../../../../../../../../🔨️modules/🖼️canvas/🧪️tests/🧫️fixtures/📷️camera-fit/🔣️.json");

/// 🕸️ The hexagonal mushroom column's own graph, laid out the way the shipped example lays it out —
/// the graph whose nodes sit at negative surface x, which is what made the stale publication visible.
fn camera_law_column_host() -> FlowHost {
    let mut host = host_with_test_fixture();
    let mut layout = protocol::value::ordered::OrderedMap::new();
    for (id, x, y) in [("height", -197.19, -102.70), ("radius", -156.03, -177.33), ("sides", -156.43, -155.28), ("profile", -64.49, -163.40), ("extrusion-axis", -65.26, -116.45), ("extrude", 34.84, -154.18)] {
        layout.insert(id.to_string(), WidgetLayout { x, y });
    }
    host.replace_host_snapshot(FlowHostSnapshot {
        schema: "flow.host_snapshot".into(),
        camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
        widgets: vec![
            Widget::InputSlider { id: "height".into(), label: "Column Height".into(), value: 6.0, min: 0.0, max: 10.0, step: 0.5 },
            Widget::Neuron { id: "extrusion-axis".into(), neuron_kind: "math.vector".into(), params: Dictionary::new(), input_ports: vec![], output_ports: vec![], preview: false },
            Widget::Neuron { id: "extrude".into(), neuron_kind: "math.add".into(), params: Dictionary::new(), input_ports: vec![], output_ports: vec![], preview: true },
        ],
        synapses: vec![],
        layout,
    });
    host
}

fn camera_fit_surface_rows() -> Vec<serde_json::Value> {
    let document: serde_json::Value = serde_json::from_str(CAMERA_FIT_FIXTURE_JSON).expect("camera fit fixture");
    document["surfaceRows"].as_array().expect("surfaceRows").clone()
}

#[test]
fn a_fitted_flow_surface_publishes_the_camera_it_computed() {
    let rows = camera_fit_surface_rows();
    assert!(rows.len() >= 4, "the surface half of the camera-fit law needs more than a happy path");
    for row in &rows {
        let name = row["name"].as_str().expect("row name");
        let width = row["viewport"]["width"].as_u64().expect("width") as u32;
        let height = row["viewport"]["height"].as_u64().expect("height") as u32;
        let mut host = camera_law_column_host();
        host.set_viewport(width, height, 1.0);
        host.set_camera(row["camera"]["x"].as_f64().expect("x"), row["camera"]["y"].as_f64().expect("y"), row["camera"]["zoom"].as_f64().expect("zoom"));
        let before = host.camera();

        let fitted = host.fit_camera_to_content();
        assert_eq!(fitted, row["expect"]["fits"].as_bool().expect("fits"), "{name}: the fit must report whether it framed anything");
        let published = host.camera();
        assert_ne!(published, before, "{name}: a fit that reports success must move the published camera off the pre-fit one");

        let painted = [host.dag.graph.host_snapshot.camera.x, host.dag.graph.host_snapshot.camera.y, host.dag.graph.host_snapshot.camera.zoom];
        if row["expect"]["publishedEqualsPainted"].as_bool().unwrap_or(false) {
            assert_eq!(published, painted, "{name}: the published camera and the painted camera are one camera");
        }
        if row["expect"]["publishedEqualsFitRule"].as_bool().unwrap_or(false) {
            let content = host.dag.graph.content_world_bounds().expect("the column graph has content");
            let rule = canvas::camera::fit_camera(&content, &Viewport { width, height, dpr: 1.0 }, canvas::camera::CONTENT_FIT_PADDING_PX);
            assert_eq!(published, [rule.x, rule.y, rule.zoom], "{name}: the published camera is the shared fit rule's answer, not a neighbouring copy");
        }
        if let Some(expected) = row["expect"]["coverageAfterFit"].as_f64() {
            assert!((camera_content_coverage(&host.dag.viewport, &host.dag.graph) - expected).abs() <= 1e-9, "{name}: a fitted camera frames the whole graph");
        }
        if row["expect"]["idempotent"].as_bool().unwrap_or(false) {
            host.fit_camera_to_content();
            assert_eq!(host.camera(), published, "{name}: fitting twice must not drift the camera");
        }
        host.retire_cold();
    }
}

#[test]
fn a_flow_surface_projects_screen_points_with_the_camera_it_published() {
    let mut host = camera_law_column_host();
    host.set_viewport(483, 814, 1.0);
    host.set_camera(94.75581571737445, -97.50833134679668, 1.7844325616011099);
    assert!(host.fit_camera_to_content());
    let published = host.camera();
    let content = host.dag.graph.content_world_bounds().expect("the column graph has content");
    let rule = canvas::camera::fit_camera(&content, &Viewport { width: 483, height: 814, dpr: 1.0 }, canvas::camera::CONTENT_FIT_PADDING_PX);
    let centre = host.world_from_screen(483.0 / 2.0, 814.0 / 2.0);
    assert!((centre.0 - rule.x).abs() <= 1e-6 && (centre.1 - rule.y).abs() <= 1e-6, "the viewport centre must project through the FITTED camera, not through the one the document shipped with");
    assert_eq!(published, [rule.x, rule.y, rule.zoom]);
    host.retire_cold();
}
//#endregion 📷️CameraAuthority

#[test]
fn replace_fixture_preserves_kind_infos_and_named_input_ports() {
    let mut host = host_with_test_fixture();
    host.replace_host_snapshot(FlowHostSnapshot {
        schema: "flow.host_snapshot".into(),
        camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
        widgets: vec![Widget::Neuron { id: "add".into(), neuron_kind: "math.add".into(), params: Dictionary::new(), input_ports: vec![], output_ports: vec![], preview: true }],
        synapses: vec![],
        layout: protocol::value::ordered::OrderedMap::new(),
    });
    let node = host.dag.graph.host_snapshot.nodes.iter().find(|node| node.id == "add").expect("add node");
    let input_ids: Vec<&str> = node.inputs().iter().map(|port| port.id.as_str()).collect();
    assert_eq!(input_ids, vec!["a", "b"]);
    host.retire_cold();
}

#[test]
fn catalogue_nested_groups_round_trip() {
    let host_json = serde_json::to_string(&[CatalogueSection {
        id: "brep".into(),
        title: "Brep".into(),
        items: vec![],
        groups: vec![CatalogueGroup {
            id: "brep.primitives-3d".into(),
            title: "Primitives 3D".into(),
            items: vec![CatalogueItem {
                kind: "neuron".into(), neuron_kind: Some("brep.prim3d.box".into()), action: None, format: None, name: "Box".into(), abbreviation: "Box".into(), icon: "emoji:📦️".into(), summary: "Axis-aligned box".into()
            }],
            groups: vec![],
        }],
    }])
    .unwrap();
    let sections = merge_catalogue_sections(&host_json).unwrap();
    let brep = sections.iter().find(|section| section.id == "brep").expect("brep section");
    let prim3d = brep.groups.iter().find(|group| group.title == "Primitives 3D").expect("prim3d group");
    assert_eq!(prim3d.items[0].neuron_kind.as_deref(), Some("brep.prim3d.box"));
}

#[test]
fn catalogue_has_module_sections() {
    let host = host_with_test_fixture();
    let json = host.catalogue_json().unwrap();
    assert!(json.contains("math"));
    assert!(json.contains("math.add"));
    assert!(json.contains("Inputs"));
    assert!(json.contains("Outputs"));
    host.retire_cold();
}

#[test]
fn flow_backed_node_graph_extras_include_fixture_and_flow_engine() {
    let host = host_with_test_fixture();
    let extras = flow_backed_node_graph_extras(&host.host_snapshot, FLOW_LOD_MODE_AUTOMATIC, 0.0, true, false, ui_styling::metrics::board::GRID_FACTOR_DEFAULT, None);
    assert!(extras.host_snapshot_json.as_ref().is_some_and(|json| json.contains("flow.host_snapshot")));
    assert!(extras.capabilities_json.as_ref().is_some_and(|json| json.contains(r#""engine":"flow""#)));
    assert!(extras.lod_json.as_ref().is_some_and(|json| json.contains(r#""automatic":true"#)));
    host.retire_cold();
}
