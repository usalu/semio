//! 🌊️ Concrete first-party Flow catalogue and example composition laws.
use semio_framework_os_flow::*;
use semio_framework_artifact_flow_flow::*;
use neural_engine::{Atom, Dictionary, Value as NeuralValue};
use std::sync::{Mutex, OnceLock};
static RECTANGLE_EXTRUDE_FIXTURE_TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
const PORT_SIDES_FIXTURE_JSON: &str = include_str!("../../🧫️fixtures/🔌️port-sides/🔣️.json");
fn complete_fixture_registration<T>(future: impl std::future::Future<Output = T>) -> T {
    match std::pin::pin!(future).as_mut().poll(&mut std::task::Context::from_waker(std::task::Waker::noop())) {
        std::task::Poll::Ready(value) => value,
        std::task::Poll::Pending => panic!("fixture registration must not depend on external work"),
    }
}


fn install_first_party_light_flow_extensions_for_tests() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        register_linked_flow_extension_installer("primitive", |registry| { semio_s_plugin_flow_extension_primitive::register(registry); });
        register_linked_flow_extension_installer("math", |registry| { semio_s_plugin_flow_extension_math::register(registry); });
        register_linked_flow_extension_installer("text", |registry| { semio_s_plugin_flow_extension_text::register(registry); });
        register_linked_flow_extension_installer("logic", |registry| { semio_s_plugin_flow_extension_logic::register(registry); });
        register_linked_flow_extension_installer("dictionary", |registry| { semio_s_plugin_flow_extension_dictionary::register(registry); });
        register_linked_flow_extension_installer("list", |registry| { semio_s_plugin_flow_extension_list::register(registry); });
        register_linked_flow_extension_installer("draw", |registry| { semio_s_plugin_flow_extension_draw::register(registry); });
        register_linked_flow_extension_installer("bim", |registry| { semio_s_plugin_flow_extension_bim::register(registry); });
        register_linked_flow_extension_installer("brep", |registry| { semio_s_plugin_flow_extension_brep::register(registry, geometry_session()); });
        for (plugin_id, manifest) in [
            ("flow-extension-primitive", semio_s_plugin_flow_extension_primitive::extension_manifest_json()),
            ("flow-extension-math", semio_s_plugin_flow_extension_math::extension_manifest_json()),
            ("flow-extension-text", semio_s_plugin_flow_extension_text::extension_manifest_json()),
            ("flow-extension-logic", semio_s_plugin_flow_extension_logic::extension_manifest_json()),
            ("flow-extension-dictionary", semio_s_plugin_flow_extension_dictionary::extension_manifest_json()),
            ("flow-extension-list", semio_s_plugin_flow_extension_list::extension_manifest_json()),
            ("flow-extension-draw", semio_s_plugin_flow_extension_draw::extension_manifest_json()),
            ("flow-extension-bim", semio_s_plugin_flow_extension_bim::extension_manifest_json()),
            ("flow-extension-brep", complete_fixture_registration(semio_s_plugin_flow_extension_brep::extension_manifest_json())),
        ] {
            install_flow_extension_manifest(plugin_id, &manifest).expect("fixture extension admission");
        }
    });
}


fn fixture_kind_infos_json() -> String {
    install_first_party_light_flow_extensions_for_tests();
    flow_neuron_kind_infos_json()
}


fn port_sides_fixture() -> serde_json::Value {
    serde_json::from_str(PORT_SIDES_FIXTURE_JSON).expect("port sides fixture")
}


fn catalogue_channel_ids(operator: &serde_json::Value, side: &str) -> Vec<String> {
    operator[side].as_array().map(|channels| channels.iter().filter_map(|channel| channel["name"].as_str().map(str::to_string)).collect()).unwrap_or_default()
}


#[test]
fn no_operator_in_the_catalogue_declares_one_port_id_on_both_sides() {
    let fixture = port_sides_fixture();
    let wildcard = fixture["wildcardMarker"].as_str().expect("wildcardMarker");
    let catalogue: serde_json::Value = serde_json::from_str(&fixture_kind_infos_json()).expect("neuron kind infos json");
    let operators = catalogue.as_array().expect("neuron kind infos array");
    assert!(operators.len() >= 100, "the first-party catalogue must be live, not a stub: {} operators", operators.len());
    let mut checked = 0usize;
    let mut offenders: Vec<String> = Vec::new();
    for operator in operators {
        let id = operator["id"].as_str().unwrap_or_default();
        let inputs = catalogue_channel_ids(operator, "inputs");
        let outputs = catalogue_channel_ids(operator, "outputs");
        let both: Vec<String> = inputs.iter().filter(|input| input.as_str() != wildcard && outputs.contains(input)).cloned().collect();
        if !both.is_empty() {
            offenders.push(format!("{id}: {both:?}"));
        }
        checked += 1;
    }
    assert!(offenders.is_empty(), "these operators name one port id on both sides, so \"{{nodeId}}@{{portId}}\" is ambiguous for them: {offenders:#?}");
}


#[test]
fn every_named_port_side_row_is_the_catalogue_the_extensions_register() {
    let fixture = port_sides_fixture();
    let suffix = fixture["suffix"].as_str().expect("suffix");
    let rows = fixture["rows"].as_array().expect("rows");
    assert!(rows.len() >= 12, "the port-side law needs the collided families AND the controls");
    let catalogue: serde_json::Value = serde_json::from_str(&fixture_kind_infos_json()).expect("neuron kind infos json");
    let operators = catalogue.as_array().expect("neuron kind infos array");
    for row in rows {
        let id = row["operator"].as_str().expect("row operator");
        let live = operators.iter().find(|operator| operator["id"].as_str() == Some(id)).unwrap_or_else(|| panic!("{id} is not in the live catalogue"));
        let expected_inputs: Vec<String> = row["inputs"].as_array().expect("inputs").iter().filter_map(|name| name.as_str().map(str::to_string)).collect();
        let expected_outputs: Vec<String> = row["outputs"].as_array().expect("outputs").iter().filter_map(|name| name.as_str().map(str::to_string)).collect();
        assert_eq!(catalogue_channel_ids(live, "inputs"), expected_inputs, "{id} inputs");
        assert_eq!(catalogue_channel_ids(live, "outputs"), expected_outputs, "{id} outputs");
        for output in &expected_outputs {
            if let Some(plain) = output.strip_suffix(suffix) {
                assert!(expected_inputs.iter().any(|input| input == plain), "{id}: `{output}` carries the suffix, so `{plain}` must be one of its own inputs — the suffix is not decoration");
            }
        }
    }
}


#[test]
fn a_renamed_output_keeps_the_display_name_it_always_had() {
    let catalogue: serde_json::Value = serde_json::from_str(&fixture_kind_infos_json()).expect("neuron kind infos json");
    let operators = catalogue.as_array().expect("neuron kind infos array");
    let vector = operators.iter().find(|operator| operator["id"].as_str() == Some("math.vector")).expect("math.vector");
    let output = vector["outputs"].as_array().expect("outputs").iter().find(|channel| channel["name"].as_str() == Some("vectorOut")).expect("vectorOut");
    assert_eq!(output["fullName"].as_str(), Some("Vector"), "the rename is an identity, not a label");
    assert_eq!(output["abbreviation"].as_str(), Some("vec"));
    assert_eq!(output["code"].as_str(), Some("VE"));
}


#[test]
fn fixture_kind_infos_json_covers_every_first_party_extension() {
    // 🧊️ Untyped JSON on purpose: deserializing into `NeuronKindInfo` (= `neural::OperatorInfo`)
    // reconstructs real cold-tracked `Dictionary` values inside `ChannelSpec::default` and panics
    // on drop outside a cold boundary — plain `serde_json::Value` sidesteps that entirely.
    let catalogue: serde_json::Value = serde_json::from_str(&fixture_kind_infos_json()).expect("neuron kind infos json");
    let ids: Vec<&str> = catalogue.as_array().expect("neuron kind infos array").iter().filter_map(|item| item["id"].as_str()).collect();
    for prefix in ["brep.", "bim.", "dictionary.", "draw.", "list.", "logic.", "math.", "core.", "text."] {
        assert!(ids.iter().any(|id| id.starts_with(prefix)), "expected at least one neuron kind id starting with {prefix:?}, got {ids:?}");
    }
}


#[test]
fn app_catalogue_carries_every_operator_and_palette_section() {
    install_first_party_light_flow_extensions_for_tests();
    let catalogue = flow_app_catalogue();
    assert!(catalogue.operators.iter().any(|info| info.id == "math.add"));
    assert!(catalogue.sections.iter().any(|section| section.id == "math"));
    assert!(catalogue.sections.iter().any(|section| section.id == "inputs"), "static widget sections must merge into the app catalogue");
    let json = flow_app_catalogue_json();
    assert!(json.contains("math.add"));
    println!("[STATS] app catalogue operators={} sections={} json_bytes={}", catalogue.operators.len(), catalogue.sections.len(), json.len());
}


#[test]
fn rectangle_extrude_fixture_port_labels_follow_draw_lod() {
    let _guard = RECTANGLE_EXTRUDE_FIXTURE_TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(|error| error.into_inner());
    // 🩹️ Was `include_str!` of procedural's example fixture; procedural migrated that fixture to a
    // handcrafted DSL (`crate::os_store::ArtifactDsl`) — inlined the same flow-fixture JSON this test actually
    // parses (`FlowHost::parse_host_snapshot_json`), decoupled from procedural's document format.
    let json = r#"{
  "schema": "flow.host_snapshot",
  "camera": { "x": 140, "y": -60, "zoom": 2.2 },
  "widgets": [
    { "kind": "inputSlider", "id": "width", "label": "Width", "value": 2, "min": 0.1, "max": 10, "step": 0.1 },
    { "kind": "inputSlider", "id": "height", "label": "Height", "value": 2, "min": 0.1, "max": 10, "step": 0.1 },
    { "kind": "inputSlider", "id": "distance", "label": "Distance", "value": 3, "min": 0.1, "max": 10, "step": 0.1 },
    {
      "kind": "neuron",
      "id": "rect",
      "neuronKind": "brep.curve.rectangle",
      "params": {},
      "input_ports": ["width", "height"],
      "preview": false
    },
    {
      "kind": "neuron",
      "id": "vector",
      "neuronKind": "math.vector",
      "params": {},
      "input_ports": ["x", "y", "z"],
      "preview": false
    },
    {
      "kind": "neuron",
      "id": "extrude",
      "neuronKind": "brep.solid.extrude",
      "params": {},
      "input_ports": ["wire", "vector"],
      "preview": true
    },
    {
      "kind": "neuron",
      "id": "volume",
      "neuronKind": "brep.measure.volume",
      "params": {},
      "input_ports": ["geometry"],
      "preview": false
    }
  ],
  "synapses": [
    { "id": "e1", "from": "width", "to": "rect", "fromPort": "number", "toPort": "width" },
    { "id": "e2", "from": "height", "to": "rect", "fromPort": "number", "toPort": "height" },
    { "id": "e3", "from": "rect", "to": "extrude", "fromPort": "wire", "toPort": "wire" },
    { "id": "e4", "from": "distance", "to": "vector", "fromPort": "number", "toPort": "z" },
    { "id": "e5", "from": "vector", "to": "extrude", "fromPort": "vectorOut", "toPort": "vector" },
    { "id": "e6", "from": "extrude", "to": "volume", "fromPort": "solid", "toPort": "geometry" }
  ],
  "layout": {
    "rect": { "x": 120, "y": -40 },
    "vector": { "x": 200, "y": 20 },
    "extrude": { "x": 280, "y": -40 },
    "volume": { "x": 360, "y": -40 },
    "width": { "x": 40, "y": -60 },
    "height": { "x": 40, "y": -20 },
    "distance": { "x": 120, "y": 20 }
  }
}
"#;
    let mut fixture = FlowHost::parse_host_snapshot_json(json).expect("fixture json");
    fixture.camera.zoom = 1.0;
    let mut host = FlowHost::from_host_snapshot(fixture);
    host.set_neuron_kind_infos_json(&fixture_kind_infos_json());
    host.set_viewport(1280, 800, 1.0);
    let mut port_texts = |lod: &str| -> Vec<String> {
        host.dag.set_automatic_lod(false);
        host.dag.set_forced_draw_lod_label(lod);
        let raw: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();
        raw["labels"].as_array().expect("labels").iter().filter(|row| row["kind"] == "port").filter_map(|row| row["text"].as_str().map(str::to_string)).collect()
    };
    let normal = port_texts("normal");
    assert!(normal.iter().any(|text| text.ends_with("wid")), "normal ports: {normal:?}");
    assert!(normal.iter().any(|text| text.ends_with("wir")), "normal ports: {normal:?}");
    let detail = port_texts("detail");
    assert!(detail.iter().any(|text| text.ends_with("width")), "detail ports: {detail:?}");
    assert!(detail.iter().any(|text| text.ends_with("wire")), "detail ports: {detail:?}");
    let micro = port_texts("micro");
    assert!(micro.iter().any(|text| text.ends_with("RectangleWire")), "micro ports: {micro:?}");
    assert!(micro.iter().any(|text| text.ends_with("ExtrudedSolid")), "micro ports: {micro:?}");
    drop(port_texts);
    host.retire_cold();
}


#[test]
fn rectangle_extrude_fixture_evaluates_solid_output() {
    let _guard = RECTANGLE_EXTRUDE_FIXTURE_TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(|error| error.into_inner());
    // 🩹️ Was `include_str!` of procedural's example fixture; procedural migrated that fixture to a
    // handcrafted DSL (`crate::os_store::ArtifactDsl`) — inlined the same flow-fixture JSON this test actually
    // parses (`FlowHost::parse_host_snapshot_json`), decoupled from procedural's document format.
    let json = r#"{
  "schema": "flow.host_snapshot",
  "camera": { "x": 140, "y": -60, "zoom": 2.2 },
  "widgets": [
    { "kind": "inputSlider", "id": "width", "label": "Width", "value": 2, "min": 0.1, "max": 10, "step": 0.1 },
    { "kind": "inputSlider", "id": "height", "label": "Height", "value": 2, "min": 0.1, "max": 10, "step": 0.1 },
    { "kind": "inputSlider", "id": "distance", "label": "Distance", "value": 3, "min": 0.1, "max": 10, "step": 0.1 },
    {
      "kind": "neuron",
      "id": "rect",
      "neuronKind": "brep.curve.rectangle",
      "params": {},
      "input_ports": ["width", "height"],
      "preview": false
    },
    {
      "kind": "neuron",
      "id": "vector",
      "neuronKind": "math.vector",
      "params": {},
      "input_ports": ["x", "y", "z"],
      "preview": false
    },
    {
      "kind": "neuron",
      "id": "extrude",
      "neuronKind": "brep.solid.extrude",
      "params": {},
      "input_ports": ["wire", "vector"],
      "preview": true
    },
    {
      "kind": "neuron",
      "id": "volume",
      "neuronKind": "brep.measure.volume",
      "params": {},
      "input_ports": ["geometry"],
      "preview": false
    }
  ],
  "synapses": [
    { "id": "e1", "from": "width", "to": "rect", "fromPort": "number", "toPort": "width" },
    { "id": "e2", "from": "height", "to": "rect", "fromPort": "number", "toPort": "height" },
    { "id": "e3", "from": "rect", "to": "extrude", "fromPort": "wire", "toPort": "wire" },
    { "id": "e4", "from": "distance", "to": "vector", "fromPort": "number", "toPort": "z" },
    { "id": "e5", "from": "vector", "to": "extrude", "fromPort": "vectorOut", "toPort": "vector" },
    { "id": "e6", "from": "extrude", "to": "volume", "fromPort": "solid", "toPort": "geometry" }
  ],
  "layout": {
    "rect": { "x": 120, "y": -40 },
    "vector": { "x": 200, "y": 20 },
    "extrude": { "x": 280, "y": -40 },
    "volume": { "x": 360, "y": -40 },
    "width": { "x": 40, "y": -60 },
    "height": { "x": 40, "y": -20 },
    "distance": { "x": 120, "y": 20 }
  }
}
"#;
    let fixture = FlowHost::parse_host_snapshot_json(json).expect("fixture json");
    let mut host = FlowHost::from_host_snapshot(fixture);
    host.set_neuron_kind_infos_json(&fixture_kind_infos_json());
    let eval_json = host.evaluate().expect("evaluate");
    let parsed: serde_json::Value = serde_json::from_str(&eval_json).expect("eval json");
    let solid = parsed.get("extrude").and_then(|entry| entry.get("out")).and_then(|out| out.get("solid").or_else(|| out.get("S"))).expect("extrude solid output");
    assert_eq!(solid.get("$schema").and_then(|v| v.as_str()), Some("geometry"));
    assert_eq!(solid.get("kind").and_then(|v| v.as_str()), Some("solid"));
    host.retire_cold();
}


#[test]
fn hexagonal_mushroom_fixture_reports_extruded_solid_output() {
    // 🩹️ Was `include_str!` of procedural's example fixture; procedural migrated that fixture to a
    // handcrafted DSL (`crate::os_store::ArtifactDsl`) — inlined the same flow-fixture JSON this test actually
    // parses (`FlowHost::parse_host_snapshot_json`), decoupled from procedural's document format.
    let json = r#"{
  "schema": "flow.host_snapshot",
  "camera": { "x": 94.75581571737445, "y": -97.50833134679668, "zoom": 1.7844325616011099 },
  "widgets": [
    { "kind": "inputSlider", "id": "height", "label": "Column Height", "value": 6.0, "min": 0.0, "max": 10.0, "step": 0.5, "unit": "m" },
    { "kind": "inputSlider", "id": "radius", "label": "Profile Radius", "value": 0.5, "min": 0.1, "max": 2.0, "step": 0.05, "unit": "m" },
    { "kind": "inputSlider", "id": "sides", "label": "Side Count", "value": 6.0, "min": 3.0, "max": 12.0, "step": 1.0 },
    { "kind": "neuron", "id": "profile", "neuronKind": "brep.curve.polygon", "params": {}, "input_ports": ["radius", "sides"], "preview": false },
    { "kind": "neuron", "id": "extrusion-axis", "neuronKind": "math.vector", "params": {}, "input_ports": ["x", "y", "z"], "preview": false },
    { "kind": "neuron", "id": "extrude", "neuronKind": "brep.solid.extrude", "params": {}, "input_ports": ["wire", "vector"], "preview": true },
    { "kind": "outputPreview", "id": "column-preview", "preview": {}, "expanded": [] }
  ],
  "synapses": [
    { "id": "e1", "from": "height", "to": "extrusion-axis", "fromPort": "number", "toPort": "z" },
    { "id": "e2", "from": "radius", "to": "profile", "fromPort": "number", "toPort": "radius" },
    { "id": "e3", "from": "sides", "to": "profile", "fromPort": "number", "toPort": "sides" },
    { "id": "e4", "from": "profile", "to": "extrude", "fromPort": "wire", "toPort": "wire" },
    { "id": "e5", "from": "extrusion-axis", "to": "extrude", "fromPort": "vectorOut", "toPort": "vector" },
    { "id": "e6", "from": "extrude", "to": "column-preview", "fromPort": "solid", "toPort": "" }
  ],
  "layout": {
    "height": { "x": -197.1913555449187, "y": -102.70789997839545 },
    "radius": { "x": -156.03796288966, "y": -177.3373596163105 },
    "sides": { "x": -156.43467044109153, "y": -155.28679730672846 },
    "profile": { "x": -64.49671116929301, "y": -163.40310309861746 },
    "extrusion-axis": { "x": -65.26327021036892, "y": -116.45687403531778 },
    "extrude": { "x": 34.842068675720895, "y": -154.18083645790136 },
    "column-preview": { "x": 237.4197774877085, "y": -103.14518978933415 }
  }
}
"#;
    let fixture = FlowHost::parse_host_snapshot_json(json).expect("fixture json");
    let mut host = FlowHost::from_host_snapshot(fixture);
    host.set_neuron_kind_infos_json(&fixture_kind_infos_json());
    let eval_json = host.evaluate().expect("evaluate");
    let parsed: serde_json::Value = serde_json::from_str(&eval_json).expect("eval json");
    let solid = parsed.get("extrude").and_then(|entry| entry.get("out")).and_then(|out| out.get("solid").or_else(|| out.get("S"))).expect("extrude solid output");
    assert_eq!(solid.get("$schema").and_then(serde_json::Value::as_str), Some("geometry"));
    assert_eq!(solid.get("kind").and_then(serde_json::Value::as_str), Some("solid"));
    let handle = solid.get("handle").and_then(serde_json::Value::as_str).expect("solid handle");
    assert!(handle.len() == 64 && handle.bytes().all(|byte| byte.is_ascii_hexdigit()), "a solid handle is the kernel's Blake3 content digest: {handle}");
    // 🧊️ The solid lives in the kernel of the extension that minted it (a packaged extension links its own copy of
    // this crate), so its mesh is asked of that extension's own operator through the registry the evaluation used.
    let geometry = Dictionary::with_schema("geometry").insert("handle", NeuralValue::Atom(Atom::String(handle.into()))).insert("kind", NeuralValue::Atom(Atom::String("solid".into())));
    let input = Dictionary::new().insert("geometry", NeuralValue::Dictionary(geometry)).insert("deflection", NeuralValue::Dictionary(Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(0.05)))));
    let exported = flow_operator_registry().dispatch("brep.io.exportStl", &input).expect("the evaluated solid is live in its extension's kernel");
    let stl = semio_framework_os_flow::mesh::decode_base64(exported.get("stl").and_then(NeuralValue::as_dictionary).and_then(|text| text.get("value")).and_then(NeuralValue::as_atom).and_then(Atom::as_str).expect("stl text")).expect("stl base64");
    let triangles = u32::from_le_bytes(stl[80..84].try_into().expect("binary stl count")) as usize;
    assert!(triangles > 0 && stl.len() == 84 + 50 * triangles, "a closed solid tessellates to a non-empty binary STL ({} bytes, {triangles} triangles)", stl.len());
    neural::ColdRetire::retire_cold(exported);
    neural::ColdRetire::retire_cold(input);
    host.retire_cold();
}


fn geometry_session() -> &'static semio_s_spatial_kernel_semio_session::Session {
    static SESSION: OnceLock<semio_s_spatial_kernel_semio_session::Session> = OnceLock::new();
    SESSION.get_or_init(semio_s_spatial_kernel_semio_session::Session::new)
}

#[test]
fn supplied_operator_registries_and_geometry_ports_keep_host_authorities_independent() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🌐️geometry-composition/🔣️.json")).unwrap();
    let sessions = [semio_s_spatial_kernel_semio_session::Session::new(), semio_s_spatial_kernel_semio_session::Session::new()];
    let mut handles = Vec::new();
    for (session, row) in sessions.iter().zip(fixture["boxes"].as_array().unwrap()) {
        let mut registry = semio_s_plugin_flow_extension_brep::module_registry(session);
        semio_s_plugin_flow_extension_primitive::register(&mut registry);
        let infos = std::sync::Arc::new(registry.operator_infos().map(|info| (info.id.clone(),info.clone())).collect());
        let (registry, mut retirement) = neural_engine::SharedRegistry::new(registry);
        let mut host = FlowHost::default().with_operator_registry(registry).with_geometry_port(session.port());
        host.set_neuron_kind_info_map(infos);
        let widget = serde_json::json!({ "kind":"neuron", "id":"box", "neuronKind":fixture["operator"], "params":{}, "input_ports":["width","depth","height"], "preview":true });
        host.add_widget(&widget.to_string(),0.0,0.0).unwrap();
        for axis in ["width","depth","height"] {
            let slider = serde_json::json!({"kind":"inputSlider","id":axis,"label":axis,"value":row[axis],"min":0.1,"max":10.0,"step":0.1});
            host.add_widget(&slider.to_string(),0.0,0.0).unwrap();
            host.connect_ports(axis,"number","box",axis).unwrap();
        }
        let eval: serde_json::Value = serde_json::from_str(&host.evaluate().unwrap()).unwrap();
        let handle = eval["box"]["out"]["solid"]["handle"].as_str().expect("the supplied local operator answers").to_owned();
        let mut steps = 0;
        let mut units = 0;
        loop {
            steps += 1;
            assert!(steps <= fixture["maximumTessellationSteps"].as_u64().unwrap());
            match host.geometry_port().unwrap().tessellate_step(&handle,0.05,fixture["tessellationGrant"].as_u64().unwrap() as usize) {
                semio_framework_os_flow::geometry::GeometryStep::Ready(mesh) => { assert!(!mesh.indices.is_empty()); break; },
                semio_framework_os_flow::geometry::GeometryStep::Working { units_done, .. } => { assert!(units_done > units); units = units_done; },
                _ => panic!("owned geometry port failed before terminal mesh"),
            }
        }
        let volume: serde_json::Value = serde_json::from_str(&session.brep_invoke_json("volume",&serde_json::json!({ "shape":handle }).to_string())).unwrap();
        assert_eq!(volume["value"].as_f64(),row["volume"].as_f64());
        handles.push(handle);
        host.retire_cold();
        for _ in 0..100_000 { if retirement.terminal_is_empty() { break; } retirement.close_step(1,4096).unwrap(); }
        assert!(retirement.terminal_is_empty());
    }
    sessions[0].close();
    let volume: serde_json::Value = serde_json::from_str(&sessions[1].brep_invoke_json("volume",&serde_json::json!({ "shape":handles[1] }).to_string())).unwrap();
    assert_eq!(volume["value"].as_f64(),fixture["boxes"][1]["volume"].as_f64());
    sessions[1].close();
}
