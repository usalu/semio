//! 🌊️ Framework Flow persisted-owner capability and complete state baselines.
use crate::*;
use semio_framework_value::{DslValue, FromValue, Number, ToValue};
use store::{ArtifactDsl, ArtifactPack};
fn laws() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()
}
fn words() -> Vec<u64> {
    laws()["binary64Bits"].as_array().unwrap().iter().map(|value| u64::from_str_radix(value.as_str().unwrap(), 16).unwrap()).collect()
}
fn dictionary(word: u64) -> neural::Dictionary {
    use neural::{Atom, Dictionary, Value};
    Dictionary::new()
        .insert("null", Value::Atom(Atom::Null))
        .insert("bool", Value::Atom(Atom::Boolean(false)))
        .insert("integer", Value::Atom(Atom::Integer(i64::MIN)))
        .insert("decimal", Value::Atom(Atom::Decimal(f64::from_bits(word))))
        .insert("text", Value::Atom(Atom::String(laws()["literal"].as_str().unwrap().into())))
        .insert("nested", Value::Dictionary(Dictionary::new().insert("", Value::Atom(Atom::Integer(i64::MAX)))))
}
fn expanded() -> OrderedSet {
    let mut set = OrderedSet::new();
    set.insert(laws()["literal"].as_str().unwrap().into());
    set.insert(String::new());
    set
}
fn full(word: u64) -> FlowHostSnapshot {
    let f = f64::from_bits(word);
    let text = laws()["literal"].as_str().unwrap().to_owned();
    let camera = CameraJson { x: f, y: f, zoom: f };
    let mut layout = OrderedMap::new();
    layout.insert(text.clone(), WidgetLayout { x: f, y: f });
    layout.insert(String::new(), WidgetLayout { x: f, y: f });
    let mut nodes = OrderedMap::new();
    for (index, chrome) in [
        NodeChrome::Plain { preview: false },
        NodeChrome::Slider { label: text.clone(), min: f, max: f, step: f, value: f },
        NodeChrome::Note { text: String::new() },
        NodeChrome::Image { src: text.clone() },
        NodeChrome::Variable { name: text.clone(), schema: String::new() },
    ]
    .into_iter()
    .enumerate()
    {
        nodes.insert(index.to_string(), FlowNodeGui { layout: WidgetLayout { x: f, y: f }, chrome });
    }
    let previews = vec![
        FlowPreviewGui { id: text.clone(), source: None, mode: text.clone(), preview: dictionary(word), expanded: expanded(), layout: None },
        FlowPreviewGui { id: String::new(), source: Some(FlowChannelRef { neuron: String::new(), channel: text.clone() }), mode: String::new(), preview: dictionary(word), expanded: expanded(), layout: Some(WidgetLayout { x: f, y: f }) },
    ];
    let synapse = || neural::Synapse { id: text.clone(), from: String::new(), to: text.clone(), from_port: String::new(), to_port: text.clone() };
    let tree = neural::Tree {
        neurons: vec![neural::Neuron {
            id: text.clone(),
            kind: String::new(),
            params: dictionary(word),
            tree: Some(Box::new(neural::Tree { neurons: vec![neural::Neuron { id: text.clone(), kind: text.clone(), params: dictionary(word), tree: None }], synapses: vec![synapse()] })),
        }],
        synapses: vec![synapse()],
    };
    let widgets = vec![
        Widget::Neuron { id: text.clone(), neuron_kind: String::new(), params: dictionary(word), input_ports: vec![String::new(), text.clone(), text.clone()], output_ports: vec![text.clone(), String::new()], preview: false },
        Widget::InputSlider { id: text.clone(), label: text.clone(), value: f, min: f, max: f, step: f },
        Widget::InputNote { id: String::new(), text: text.clone() },
        Widget::InputImage { id: text.clone(), src: String::new() },
        Widget::Variable { id: text.clone(), name: String::new(), schema: text.clone() },
        Widget::OutputPreview { id: text.clone(), preview: dictionary(word), expanded: expanded() },
        Widget::OutputAction { id: text.clone(), action: String::new() },
        Widget::OutputExport { id: text.clone(), format: text.clone() },
        Widget::Cluster { id: text.clone(), name: text.clone(), tree, flow: FlowUi { camera: camera.clone(), nodes, previews } },
    ];
    FlowHostSnapshot { schema: String::new(), camera, widgets, synapses: vec![SynapseSpec { id: String::new(), from: text.clone(), to: String::new(), from_port: text.clone(), to_port: String::new() }], layout }
}
struct Owned(Option<FlowHostSnapshot>);
impl Owned {
    fn new(snapshot: FlowHostSnapshot) -> Self {
        Self(Some(snapshot))
    }
}
impl std::ops::Deref for Owned {
    type Target = FlowHostSnapshot;
    fn deref(&self) -> &FlowHostSnapshot {
        self.0.as_ref().unwrap()
    }
}
impl Drop for Owned {
    fn drop(&mut self) {
        if let Some(snapshot) = self.0.take() {
            let mut cursor = semio_framework_value::retirement::owned_retirement(snapshot);
            loop {
                match cursor.close_step(256, usize::MAX).unwrap() {
                    store::SnapshotRetirementStep::Complete => break,
                    store::SnapshotRetirementStep::Pending { .. } => {}
                    store::SnapshotRetirementStep::Blocked => panic!("framework Flow retirement blocked"),
                }
            }
            assert!(cursor.terminal_is_empty());
        }
    }
}
fn observe(value: &DslValue) -> serde_json::Value {
    match value {
        DslValue::Null => serde_json::json!(["null"]),
        DslValue::Bool(v) => serde_json::json!(["bool", v]),
        DslValue::Number(Number::UInt(v)) => serde_json::json!(["uint", v.to_string()]),
        DslValue::Number(Number::Int(v)) => serde_json::json!(["int", v.to_string()]),
        DslValue::Number(Number::Float(v)) => serde_json::json!(["float", format!("{:016x}", v.to_bits())]),
        DslValue::String(v) => serde_json::json!(["string", v]),
        DslValue::Bytes(v) => serde_json::json!(["bytes", v]),
        DslValue::Array(v) => serde_json::json!(["array", v.iter().map(observe).collect::<Vec<_>>()]),
        DslValue::Object(v) => serde_json::json!(["object", v.iter().map(|(k, v)| serde_json::json!([k, observe(v)])).collect::<Vec<_>>()]),
    }
}
fn assert_full(actual: &FlowHostSnapshot, expected: &FlowHostSnapshot) {
    assert_eq!(observe(&actual.to_value()), observe(&expected.to_value()));
}
fn independent_sqlite_file(bytes: &[u8]) -> Vec<u8> {
    independent_sqlite_file_with_metadata(bytes, None)
}
fn independent_sqlite_file_with_metadata(bytes: &[u8], encoding: Option<store::sqlite_snapshot::SnapshotEncoding>) -> Vec<u8> {
    use std::io::Write;
    let script = r#"import{Database}from'bun:sqlite';const d=Database.deserialize(await Bun.stdin.bytes(),{safeIntegers:true});const widths=JSON.parse(process.argv[1]);if(process.argv[2]){widths.semio_snapshot=6;const m=d.query('SELECT artifact_kind,standard,subset,native_encoding FROM semio_snapshot').get();if(m.artifact_kind!=='flow.host_snapshot'||m.standard!=='1'||m.subset!=='*'||m.native_encoding!==process.argv[2])throw Error('metadata');}if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok')throw Error('integrity');if(d.query('PRAGMA foreign_key_check').all().length)throw Error('FK');const names=d.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all().map(r=>r.name);if(JSON.stringify(names)!==JSON.stringify(Object.keys(widths).sort()))throw Error('tables');for(const[name,width]of Object.entries(widths))if(d.query('PRAGMA table_info('+name+')').all().length!==width)throw Error(name);process.stdout.write(d.serialize());d.close();"#;
    let mut child = std::process::Command::new("bun").args(["-e", script, &laws()["tableWidths"].to_string(), encoding.map_or("", |value| value.as_str())]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    output.stdout
}
#[test]
fn sqlite_snapshot_framework_flow_bare_persisted_owner_has_semantic_capability() {
    assert!(FlowHostSnapshot::sqlite_snapshot_codec().is_some(), "framework Flow persisted owner lacks semantic SQLite capability");
}
#[test]
fn sqlite_snapshot_framework_flow_public_identity_matches_actual_native_carrier() {
    assert_eq!(<FlowHostSnapshot as ArtifactDsl>::envelope_id(), laws()["nativeEnvelope"].as_str().unwrap());
}
#[test]
fn sqlite_snapshot_framework_flow_binary_keeps_every_widget_cluster_gui_and_word() {
    for word in words() {
        let expected = Owned::new(full(word));
        let actual = Owned::new(FlowHostSnapshot::decode_pack(&expected.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()).unwrap());
        assert_full(&actual, &expected);
    }
}
#[test]
fn sqlite_snapshot_framework_flow_text_keeps_every_widget_cluster_gui_and_word() {
    for word in words() {
        let expected = Owned::new(full(word));
        let text = expected.print_dsl();
        let result = FlowHostSnapshot::parse_dsl(&text);
        let actual = Owned::new(result.unwrap());
        assert_full(&actual, &expected);
    }
}

fn family_laws() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🚦️owned-families.json")).unwrap()
}
fn retire_layouts(value: OrderedMap<WidgetLayout>) {
    crate::retained::FlowRetirement::from_owner(crate::retained::FlowOwner::Layouts(value)).retire_cold();
}
fn retire_tree(value: neural::Tree) {
    crate::retained::FlowRetirement::from_owner(crate::retained::FlowOwner::Tree(value)).retire_cold();
}
fn retire_neuron(value: neural::Neuron) {
    crate::retained::FlowRetirement::from_owner(crate::retained::FlowOwner::Neurons(vec![value])).retire_cold();
}
fn retire_synapse(value: neural::Synapse) {
    crate::retained::FlowRetirement::from_owner(crate::retained::FlowOwner::Synapses(vec![value])).retire_cold();
}
fn assert_family<T: ToValue + FromValue>(expected: &T, name: &str, retire: fn(T)) {
    use semio_framework_value::{NativeDecodeControl, NativeEncodeControl, ValueRefusalKind};
    let fixture = family_laws();
    let maximum = fixture["limits"]["successBytes"].as_u64().unwrap() as usize;
    let mut yes = |_| true;
    let mut encode = NativeEncodeControl::new(maximum, &mut yes);
    let value = expected.to_value_controlled(&mut encode).unwrap().guard_decoded();
    assert!(fixture["families"].as_array().unwrap().iter().any(|family| family.as_str() == Some(name)));
    if let Some(fields) = fixture["recordFields"][name].as_array() {
        let DslValue::Object(entries) = value.get() else { panic!("neural record witness must be an object"); };
        assert_eq!(entries.iter().map(|(key, _)| key.as_str()).collect::<Vec<_>>(), fields.iter().map(|field| field.as_str().unwrap()).collect::<Vec<_>>());
    }
    let encoded = encode.owned_bytes();
    assert!(encoded > 0);
    assert_eq!(observe(value.get()), observe(&expected.to_value()));
    for limit in [encoded, encoded - 1, 0] {
        let mut yes = |_| true;
        let mut control = NativeEncodeControl::new(limit, &mut yes);
        let result = expected.to_value_controlled(&mut control);
        if limit == encoded {
            let actual = result.unwrap().guard_decoded();
            assert_eq!(observe(actual.get()), observe(value.get()));
            assert_eq!(control.owned_bytes(), encoded);
        } else {
            assert_eq!(result.err().expect("owned family must refuse incomplete backing").kind, ValueRefusalKind::OwnershipLimit);
            assert!(control.owned_bytes() <= limit);
        }
    }
    let mut yes = |_| true;
    let mut decode = NativeDecodeControl::new(maximum, &mut yes);
    let actual = semio_framework_value::DecodedValue::new(T::from_value_controlled(value.get(), &mut decode).unwrap(), retire);
    let decoded = decode.owned_bytes();
    assert!(decoded > 0);
    assert_eq!(observe(&actual.get().to_value()), observe(value.get()));
    for limit in [decoded, decoded - 1, 0] {
        let mut yes = |_| true;
        let mut control = NativeDecodeControl::new(limit, &mut yes);
        let result = T::from_value_controlled(value.get(), &mut control);
        if limit == decoded {
            let actual = semio_framework_value::DecodedValue::new(result.unwrap(), retire);
            assert_eq!(observe(&actual.get().to_value()), observe(value.get()));
            assert_eq!(control.owned_bytes(), decoded);
        } else {
            assert_eq!(result.err().expect("owned family must refuse incomplete backing").kind, ValueRefusalKind::OwnershipLimit);
            assert!(control.owned_bytes() <= limit);
        }
    }
    let mut no = |_| false;
    let mut control = NativeEncodeControl::new(maximum, &mut no);
    assert_eq!(expected.to_value_controlled(&mut control).err().unwrap().kind, ValueRefusalKind::Canceled);
    assert_eq!(control.owned_bytes(), 0);
    let mut no = |_| false;
    let mut control = NativeDecodeControl::new(maximum, &mut no);
    assert_eq!(T::from_value_controlled(value.get(), &mut control).err().unwrap().kind, ValueRefusalKind::Canceled);
    assert_eq!(control.owned_bytes(), 0);
}
fn assert_family_interior_stop<T: ToValue + FromValue>(expected: &T) {
    use semio_framework_value::{NativeDecodeControl, NativeEncodeControl, ValueRefusalKind};
    let fixture = family_laws();
    let maximum = fixture["limits"]["successBytes"].as_u64().unwrap() as usize;
    let interior = fixture["limits"]["interiorBytes"].as_u64().unwrap() as usize;
    let mut seen = false;
    let mut progress = |p: semio_framework_value::native_encoding::NativeEncodeProgress| {
        if p.total > interior && p.completed >= interior && p.completed < p.total { seen = true; false } else { true }
    };
    let mut control = NativeEncodeControl::new(maximum, &mut progress);
    assert_eq!(expected.to_value_controlled(&mut control).err().unwrap().kind, ValueRefusalKind::Canceled);
    assert!(control.owned_bytes() >= interior);
    drop(control);
    assert!(seen);
    let value = expected.to_value().guard_decoded();
    let mut seen = false;
    let mut progress = |p: semio_framework_value::native_decoding::NativeDecodeProgress| {
        if p.total > interior && p.completed >= interior && p.completed < p.total { seen = true; false } else { true }
    };
    let mut control = NativeDecodeControl::new(maximum, &mut progress);
    assert_eq!(T::from_value_controlled(value.get(), &mut control).err().unwrap().kind, ValueRefusalKind::Canceled);
    assert!(control.owned_bytes() >= interior);
    drop(control);
    assert!(seen);
}

#[test]
fn sqlite_snapshot_framework_flow_controlled_value_keeps_actual_owned_state() {
    for word in words() {
        let expected = Owned::new(full(word));
        let mut layout = OrderedMap::new();
        let mut set = OrderedSet::new();
        for key in family_laws()["keys"].as_array().unwrap() {
            layout.insert(key.as_str().unwrap().into(), WidgetLayout { x: f64::from_bits(word), y: f64::from_bits(word) });
            set.insert(key.as_str().unwrap().into());
        }
        let layout = semio_framework_value::DecodedValue::new(layout, retire_layouts);
        let set = semio_framework_value::DecodedValue::new(set, OrderedSet::retire_cold);
        let keys = family_laws()["orderedKeys"].as_array().unwrap().iter().map(|key| key.as_str().unwrap().to_owned()).collect::<Vec<_>>();
        assert_eq!(layout.get().iter().map(|(key, _)| key.clone()).collect::<Vec<_>>(), keys);
        assert_eq!(set.get().iter().cloned().collect::<Vec<_>>(), keys);
        assert_family(layout.get(), "orderedMap", retire_layouts);
        assert_family(set.get(), "orderedSet", OrderedSet::retire_cold);
        assert_family(&expected.layout, "orderedMap", retire_layouts);
        let Widget::Cluster { tree, .. } = &expected.widgets[8] else { panic!("neutral cluster witness"); };
        assert_family(tree, "tree", retire_tree);
        assert_family(&tree.neurons[0], "neuron", retire_neuron);
        assert_family(&tree.synapses[0], "synapse", retire_synapse);
        let Widget::OutputPreview { expanded, .. } = &expected.widgets[5] else { panic!("neutral preview witness"); };
        assert_family(expanded, "orderedSet", OrderedSet::retire_cold);
        let mut yes = |_| true;
        let mut encode = semio_framework_value::NativeEncodeControl::new(16 << 20, &mut yes);
        let value = <FlowHostSnapshot as ToValue>::to_value_controlled(&expected, &mut encode).unwrap().guard_decoded();
        let mut yes = |_| true;
        let mut decode = semio_framework_value::NativeDecodeControl::new(16 << 20, &mut yes);
        let actual = Owned::new(<FlowHostSnapshot as FromValue>::from_value_controlled(value.get(), &mut decode).unwrap());
        assert_full(&actual, &expected);
    }

    let depth = family_laws()["limits"]["recursiveDepth"].as_u64().unwrap() as usize;
    let mut tree = neural::Tree { neurons: Vec::new(), synapses: Vec::new() };
    for _ in 0..depth {
        tree = neural::Tree { neurons: vec![neural::Neuron { id: String::new(), kind: String::new(), params: neural::Dictionary::new(), tree: Some(Box::new(tree)) }], synapses: Vec::new() };
    }
    let tree = semio_framework_value::DecodedValue::new(tree, retire_tree);
    let mut yes = |_| true;
    let mut encode = semio_framework_value::NativeEncodeControl::new(16 << 20, &mut yes);
    assert_eq!(tree.get().to_value_controlled(&mut encode).err().unwrap().kind, semio_framework_value::ValueRefusalKind::DepthLimit);
    let value = tree.get().to_value().guard_decoded();
    let mut yes = |_| true;
    let mut decode = semio_framework_value::NativeDecodeControl::new(16 << 20, &mut yes);
    assert_eq!(neural::Tree::from_value_controlled(value.get(), &mut decode).err().unwrap().kind, semio_framework_value::ValueRefusalKind::DepthLimit);

}
#[test]
fn sqlite_snapshot_framework_flow_erased_both_formats_expose_persisted_entities() {
    use store::sqlite_snapshot::*;
    let capability = store::ArtifactCodec::bare::<FlowHostSnapshot, FlowMutation>("flow.host_snapshot").snapshot_sqlite.expect("framework Flow relational owner");
    let dialect = store::io_schema::ArtifactDialect { artifact_kind: laws()["documentSchema"].as_str().unwrap().into(), standard: "1".into(), subset: "*".into() };
    let expected = Owned::new(full(words()[6]));
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let payload = match encoding {
            SnapshotEncoding::Binary => store::io_schema::IoPayload::Binary(expected.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()),
            SnapshotEncoding::Text => store::io_schema::IoPayload::Text(expected.print_dsl()),
        };
        let result = (capability.export)("flow.host_snapshot", &dialect, &payload, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default()));
        let database = result.unwrap().value;
        assert_eq!(database.tables.len(), 37);
        assert_eq!(database.table("flow_document").unwrap().single_row().unwrap().text(1).unwrap(), "");
        assert_eq!(database.table("flow_widget").unwrap().rows.len(), 9);
        let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
        let bytes = independent_sqlite_file(&bytes);
        let database = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
        let payload = (capability.import)("flow.host_snapshot", &dialect, database, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
        let actual = Owned::new(match payload {
            store::io_schema::IoPayload::Binary(bytes) => FlowHostSnapshot::decode_pack(&bytes).unwrap(),
            store::io_schema::IoPayload::Text(text) => FlowHostSnapshot::parse_dsl(&text).unwrap(),
        });
        assert_full(&actual, &expected);
    }
}
#[test]
fn sqlite_snapshot_framework_flow_native_output_cancels_inside_owned_text_copy() {
    let mut snapshot = full(0);
    snapshot.schema = laws()["literal"].as_str().unwrap().repeat(laws()["work"]["repeat"].as_u64().unwrap() as usize);
    let snapshot = Owned::new(snapshot);
    let mut seen = false;
    let mut progress = |p: semio_framework_value::native_encoding::NativeEncodeProgress| {
        if p.total >= 65536 && p.completed >= 65536 && p.completed < p.total {
            seen = true;
            false
        } else {
            true
        }
    };
    let mut control = semio_framework_value::NativeEncodeControl::new(16 << 20, &mut progress);
    assert!(<FlowHostSnapshot as ToValue>::to_value_controlled(&snapshot, &mut control).is_err());
    drop(control);
    assert!(seen);

    let literal = laws()["literal"].as_str().unwrap().repeat(family_laws()["limits"]["repeat"].as_u64().unwrap() as usize);
    let mut snapshot = full(0);
    snapshot.layout.insert(literal.clone(), WidgetLayout { x: -0.0, y: f64::from_bits(words()[6]) });
    if let Widget::OutputPreview { expanded, .. } = &mut snapshot.widgets[5] { expanded.insert(literal.clone()); }
    if let Widget::Cluster { tree, .. } = &mut snapshot.widgets[8] {
        tree.neurons[0].id = literal.clone();
        tree.synapses[0].id = literal;
    }
    let snapshot = Owned::new(snapshot);
    assert_family_interior_stop(&snapshot.layout);
    let Widget::OutputPreview { expanded, .. } = &snapshot.widgets[5] else { panic!("neutral preview witness"); };
    assert_family_interior_stop(expanded);
    let Widget::Cluster { tree, .. } = &snapshot.widgets[8] else { panic!("neutral cluster witness"); };
    assert_family_interior_stop(tree);
    assert_family_interior_stop(&tree.neurons[0]);
    assert_family_interior_stop(&tree.synapses[0]);
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_framework_flow_public_io_reaches_actual_persisted_owner_in_both_formats() {
    use store::io::{ArtifactDialect, io_mechanism::{io_route, io_run_with_snapshot_control}};
    use store::io_schema::{Dialect, StandardId, SubsetId, IoPayload, IoFidelity, SQLITE_SNAPSHOT};
    use store::sqlite_snapshot::{SnapshotEncoding, SqliteDatabaseLimits, SqliteSnapshotPhase};
    let codec = store::ArtifactCodec::bare::<FlowHostSnapshot, FlowMutation>(FLOW_DOCUMENT_SCHEMA);
    store::io::register_native_document_codec(Dialect { artifact_kind: "flow.host_snapshot", standard: StandardId("1"), subset: SubsetId("*") }, codec).unwrap();
    let dialect = ArtifactDialect { artifact_kind: laws()["documentSchema"].as_str().unwrap().into(), standard: "1".into(), subset: "*".into() };
    let sqlite = ArtifactDialect::from(SQLITE_SNAPSHOT);
    let export = io_route(&dialect, &sqlite, 1).await.expect("actual framework Flow persisted owner must publish public SQLite export").value;
    let import = io_route(&sqlite, &dialect, 1).await.expect("actual framework Flow persisted owner must publish public SQLite import").value;
    for route in [&export, &import] { assert_eq!(route.hops.len(), 1); assert_eq!(route.fidelity, IoFidelity::Exact); }
    let expected = Owned::new(full(words()[6]));
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let payload = match encoding {
            SnapshotEncoding::Binary => IoPayload::Binary(expected.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()),
            SnapshotEncoding::Text => IoPayload::Text(expected.print_dsl()),
        };
        let mut phases = Vec::new();
        let output = io_run_with_snapshot_control(&export, payload, SqliteDatabaseLimits::default(), &mut |event| { phases.push(event.phase); true }).await.expect("framework Flow native state must reach public physical SQLite").value;
        let IoPayload::Binary(bytes) = output else { panic!("SQLite endpoint must return binary"); };
        for phase in [SqliteSnapshotPhase::DecodeNative, SqliteSnapshotPhase::ProjectSnapshot, SqliteSnapshotPhase::WritePages] { assert!(phases.contains(&phase)); }
        let bytes = independent_sqlite_file_with_metadata(&bytes, Some(encoding));
        phases.clear();
        let payload = io_run_with_snapshot_control(&import, IoPayload::Binary(bytes), SqliteDatabaseLimits::default(), &mut |event| { phases.push(event.phase); true }).await.expect("independently serialized SQLite must reach framework Flow native state").value;
        for phase in [SqliteSnapshotPhase::ReadPages, SqliteSnapshotPhase::ReconstructSnapshot, SqliteSnapshotPhase::EncodeNative] { assert!(phases.contains(&phase)); }
        let actual = Owned::new(match payload {
            IoPayload::Binary(bytes) => FlowHostSnapshot::decode_pack(&bytes).unwrap(),
            IoPayload::Text(text) => FlowHostSnapshot::parse_dsl(&text).unwrap(),
        });
        assert_full(&actual, &expected);
    }
}
