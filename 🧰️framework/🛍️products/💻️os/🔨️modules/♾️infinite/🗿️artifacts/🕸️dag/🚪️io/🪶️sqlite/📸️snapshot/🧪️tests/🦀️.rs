//! 🕸️ Framework DAG persisted-owner capability, literal-state and control baselines.
use crate::*;
use semio_framework_value::{DslValue, FromValue, Number, ToValue};

use store::{ArtifactDsl, ArtifactPack};
#[path="./🧮️properties/🦀️.rs"]
pub(crate) mod property_allocation;

fn laws() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()
}
fn words() -> Vec<u64> {
    laws()["binary64Bits"].as_array().unwrap().iter().map(|value| u64::from_str_radix(value.as_str().unwrap(), 16).unwrap()).collect()
}
fn value(word: u64) -> DslValue {
    DslValue::Array(vec![
        DslValue::Null,
        DslValue::Bool(false),
        DslValue::uint(u64::MAX),
        DslValue::int(i64::MIN),
        DslValue::float(f64::from_bits(word)),
        DslValue::String(laws()["literal"].as_str().unwrap().into()),
        DslValue::Bytes(vec![0, 1, 127, 128, 255]),
        DslValue::Array(vec![]),
        DslValue::Object(vec![("same".into(), DslValue::Bool(true)), ("same".into(), DslValue::Null)]),
    ])
}
fn port(word: u64) -> IoPortSpec {
    let literal = laws()["literal"].as_str().unwrap().to_owned();
    IoPortSpec {
        id: literal.clone(),
        label: literal.clone(),
        code: literal.clone(),
        abbreviation: literal.clone(),
        full_name: literal.clone(),
        value_type: Some(String::new()),
        default: Some(value(word)),
        value: Some(DslValue::Null),
        connected: Some(false),
        artifact_kind: Some(literal.clone()),
        cardinality: literal,
        shape: PortShape::Triangle,
        visible: false,
        resolved: Some(true),
    }
}
pub(crate) fn full(word: u64) -> DagSnapshot {
    let text = laws()["literal"].as_str().unwrap().to_owned();
    let f = f64::from_bits(word);
    let mut kinds = vec![
        DagNodeKind::Computation { inputs: vec![port(word)], outputs: vec![port(word)], variadic_inputs: true, variadic_outputs: false },
        DagNodeKind::Slider { min: f, max: f, step: f, value: f, output: port(word) },
        DagNodeKind::Select { options: vec![String::new(), text.clone(), text.clone()], selected: u64::MAX, output: port(word) },
        DagNodeKind::Screen { media: None, input: port(word) },
        DagNodeKind::Note { text: text.clone(), output: port(word) },
        DagNodeKind::Image { src: text.clone(), output: port(word) },
        DagNodeKind::Action { label: text.clone(), input: port(word) },
        DagNodeKind::Export { label: text.clone(), format: text.clone(), input: port(word) },
        DagNodeKind::Cluster { inputs: vec![port(word)], outputs: vec![port(word)] },
        DagNodeKind::AppInstance { instance_id: text.clone(), plugin_id: String::new(), app_id: text.clone(), icon: text.clone(), inputs: vec![port(word)], outputs: vec![port(word)] },
    ];
    for kind in [DagMediaKind::Image, DagMediaKind::Svg, DagMediaKind::Pdf, DagMediaKind::Video] {
        kinds.push(DagNodeKind::Screen { media: Some(DagMedia { kind, src: text.clone() }), input: port(word) });
    }
    for content in [DagPreviewContent::Empty, DagPreviewContent::Scalar { text: String::new() }, DagPreviewContent::Image { src: text.clone() }, DagPreviewContent::Tree { json: value(word) }] {
        kinds.push(DagNodeKind::Preview { content, expanded: DagExpandedPaths::from([String::new(), text.clone()]), input: port(word) });
    }
    let properties = graph::manifest::PropertyBag::from([
        ("null".into(), graph::manifest::PropertyValue::Null),
        ("bool".into(), graph::manifest::PropertyValue::Bool(false)),
        ("number".into(), graph::manifest::PropertyValue::Number(f)),
        ("text".into(), graph::manifest::PropertyValue::String(text.clone())),
        ("array".into(), graph::manifest::PropertyValue::Array(vec![graph::manifest::PropertyValue::Number(f)])),
        ("object".into(), graph::manifest::PropertyValue::Object(graph::manifest::PropertyBag::from([(String::new(), graph::manifest::PropertyValue::String(text.clone()))]))),
    ]);
    let nodes = kinds
        .into_iter()
        .map(|kind| DagNodeSpec { id: text.clone(), name: text.clone(), abbreviation: String::new(), icon: text.clone(), x: f, y: f, width: f, height: f, operator_kind: Some(String::new()), properties: properties.clone(), kind })
        .collect();
    let edges = [EdgeRouteStyle::Bezier, EdgeRouteStyle::SharpSz].into_iter().map(|route_style| DagHostSnapshotEdge { id: String::new(), source: text.clone(), target: String::new(), route_style, properties: properties.clone() }).collect();
    DagSnapshot { schema: String::new(), nodes, edges }
}
struct Owned(Option<DagSnapshot>);
impl Owned {
    fn new(snapshot: DagSnapshot) -> Self {
        Self(Some(snapshot))
    }
}
impl std::ops::Deref for Owned {
    type Target = DagSnapshot;
    fn deref(&self) -> &DagSnapshot {
        self.0.as_ref().unwrap()
    }
}
impl Drop for Owned {
    fn drop(&mut self) {
        if let Some(snapshot) = self.0.take() {
            let mut cursor=semio_framework_value::retirement::controlled::ControlledRetirement::new(snapshot).unwrap();
            for turn in 0..1_000_000{
                let copy=cursor.next_copy_byte_demand().unwrap().max(4096);
                let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:cursor.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:cursor.next_release_byte_demand().unwrap(),maximum_depth:cursor.next_depth_demand().unwrap()};
                let step=cursor.step(grant).unwrap();assert!(step.progress().fits(grant));
                if cursor.terminal_is_empty(){break;}assert!(turn<999_999,"original DAG fixture cleanup stalled");
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
fn assert_full(actual: &DagSnapshot, expected: &DagSnapshot) {
    assert_eq!(observe(&actual.to_value()), observe(&expected.to_value()));
}
fn independent_sqlite_file(bytes: &[u8]) -> Vec<u8> {
    use std::io::Write;
    let script = r#"import{Database}from'bun:sqlite';const d=Database.deserialize(await Bun.stdin.bytes(),{safeIntegers:true});const widths=JSON.parse(process.argv[1]);if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok')throw Error('integrity');if(d.query('PRAGMA foreign_key_check').all().length)throw Error('FK');const names=d.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all().map(r=>r.name);if(JSON.stringify(names)!==JSON.stringify(Object.keys(widths).sort()))throw Error('tables');for(const[name,width]of Object.entries(widths))if(d.query('PRAGMA table_info('+name+')').all().length!==width)throw Error(name);process.stdout.write(d.serialize());d.close();"#;
    let mut child = std::process::Command::new("bun").args(["-e", script, &laws()["tableWidths"].to_string()]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    output.stdout
}
#[test]
fn sqlite_snapshot_framework_dag_bare_persisted_owner_has_semantic_capability() {
    assert!(DagSnapshot::sqlite_snapshot_codec().is_some(), "framework DAG persisted owner lacks semantic SQLite capability");
}
#[test]
fn sqlite_snapshot_framework_dag_public_identity_matches_actual_native_carrier() {
    let expected = laws()["nativeEnvelope"].as_str().unwrap().to_owned();
    assert_eq!(<DagSnapshot as ArtifactDsl>::envelope_id(), expected);
}
#[test]
fn sqlite_snapshot_framework_dag_binary_keeps_all_variant_fields_and_exact_words() {
    for word in words() {
        let expected = Owned::new(full(word));
        let actual = Owned::new(DagSnapshot::decode_pack(&expected.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()).unwrap());
        assert_full(&actual, &expected);
    }
}
#[test]
fn sqlite_snapshot_framework_dag_text_keeps_all_variant_fields_and_exact_words() {
    for word in words() {
        let expected = Owned::new(full(word));
        let actual = Owned::new(DagSnapshot::parse_dsl(&expected.print_dsl()).unwrap());
        assert_full(&actual, &expected);
    }
}
#[test]
fn sqlite_snapshot_framework_dag_controlled_value_keeps_actual_owned_state() {
    let expected = Owned::new(full(words()[6]));
    let mut yes = |_| true;
    let mut encode = semio_framework_value::NativeEncodeControl::new(16 << 20, &mut yes);
    let value = <DagSnapshot as ToValue>::to_value_controlled(&expected, &mut encode).unwrap();
    let mut yes = |_| true;
    let mut decode = semio_framework_value::NativeDecodeControl::new(16 << 20, &mut yes);
    let actual = Owned::new(<DagSnapshot as FromValue>::from_value_controlled(&value, &mut decode).unwrap());
    assert_full(&actual, &expected);
}
#[test]
fn sqlite_snapshot_framework_dag_erased_both_native_formats_have_queryable_entities() {
    use store::sqlite_snapshot::*;
    let capability = store::ArtifactCodec::bare::<DagSnapshot, DagMutation>("dag.host_snapshot").snapshot_sqlite.expect("framework DAG relational owner");
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: laws()["documentSchema"].as_str().unwrap().into(), standard: "1".into(), subset: "*".into() };
    let expected = Owned::new(full(words()[6]));
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let payload = match encoding {
            SnapshotEncoding::Binary => store::io_schema::IoPayload::Binary(expected.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()),
            SnapshotEncoding::Text => store::io_schema::IoPayload::Text(expected.print_dsl()),
        };
        let database = (capability.export)("dag.host_snapshot", &dialect, &payload, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
        assert_eq!(database.tables.len(), 40);
        assert_eq!(database.table("dag_document").unwrap().single_row().unwrap().text(1).unwrap(), "");
        assert_eq!(database.table("dag_node").unwrap().rows.len(), expected.nodes.len());
        let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
        let bytes = independent_sqlite_file(&bytes);
        let database = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
        let payload = (capability.import)("dag.host_snapshot", &dialect, database, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
        let actual = Owned::new(match payload {
            store::io_schema::IoPayload::Binary(bytes) => DagSnapshot::decode_pack(&bytes).unwrap(),
            store::io_schema::IoPayload::Text(text) => DagSnapshot::parse_dsl(&text).unwrap(),
        });
        assert_full(&actual, &expected);
    }
}
#[test]
fn sqlite_snapshot_framework_dag_native_output_cancels_inside_owned_text_copy() {
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
    assert!(<DagSnapshot as ToValue>::to_value_controlled(&snapshot, &mut control).is_err());
    drop(control);
    assert!(seen);
}

#[test]
fn sqlite_snapshot_framework_dag_borrowed_preflight_preserves_zero_owned_admission(){
 use store::ArtifactSqliteSnapshot;
 use store::sqlite_snapshot::*;
 use semio_framework_value::ValueRefusalKind;
 let snapshot=Owned::new(full(words()[6]));
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let mut progress=|_|true;let mut control=SqliteSnapshotControl::new(&mut progress,SqliteDatabaseLimits{max_allocation_bytes:0,..Default::default()});
  snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut control).unwrap();assert_eq!(control.allocation_remaining_bytes(),0);
  let actual=match encoding{SnapshotEncoding::Binary=>snapshot.encode_pack_with(&store::PackEncodeOptions::default()).unwrap().len(),SnapshotEncoding::Text=>snapshot.print_dsl().len()};
  let mut low=0;let mut high=SqliteDatabaseLimits::default().max_file_bytes;while high-low>1{let middle=low+(high-low)/2;let mut yes=|_|true;let mut control=SqliteSnapshotControl::new(&mut yes,SqliteDatabaseLimits{max_file_bytes:middle,max_allocation_bytes:0,..Default::default()});match snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut control){Ok(())=>high=middle,Err(error)=>{assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);low=middle;}}assert_eq!(control.allocation_remaining_bytes(),0);}assert!(high>=actual,"borrowed ceiling {high} must contain actual declared payload {actual}");
  let mut progress=|_|true;let mut control=SqliteSnapshotControl::new(&mut progress,SqliteDatabaseLimits{max_file_bytes:0,max_allocation_bytes:0,..Default::default()});
  assert_eq!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut control).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(control.allocation_remaining_bytes(),0);
  let mut progress=|_|true;let mut control=SqliteSnapshotControl::new(&mut progress,SqliteDatabaseLimits{max_rows:0,max_allocation_bytes:0,..Default::default()});
  assert_eq!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut control).unwrap_err().kind,ValueRefusalKind::WorkLimit);assert_eq!(control.allocation_remaining_bytes(),0);
  let mut reached=false;let mut progress=|event:SqliteSnapshotProgress|{if event.phase==SqliteSnapshotPhase::EncodeNative{reached=true;false}else{true}};let mut control=SqliteSnapshotControl::new(&mut progress,SqliteDatabaseLimits{max_allocation_bytes:17,..Default::default()});
  assert_eq!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(control.allocation_remaining_bytes(),17);drop(control);assert!(reached);
 }
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_framework_dag_normal_owner_public_registration_words_and_edit(){
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteDatabaseLimits};use {semio_framework_artifact_reference::ArtifactDialect,store::io::io_mechanism::io_export_sqlite_snapshot,store::io::io_mechanism::io_import_sqlite_snapshot,store::io::io_mechanism::io_route};use std::{io::Write,process::{Command,Stdio}};
 let registration=<DagSnapshot as ArtifactPack>::native_snapshot_registration();assert!(registration.is_some(),"DagSnapshot must publish normal owning native registration rather than a test-local bare codec");let(owner,codec)=registration.unwrap();assert_eq!(owner.artifact_kind,"dag.host_snapshot");assert_eq!(owner.standard.0,"1");assert_eq!(owner.subset.0,"*");assert!(codec.snapshot_sqlite.is_some());store::io::register_native_document_codec(owner,codec).unwrap();
 let dialect=ArtifactDialect{artifact_kind:"dag.host_snapshot".into(),standard:"1".into(),subset:"*".into()};let sqlite=ArtifactDialect::from(store::io_schema::SQLITE_SNAPSHOT);for route in[io_route(&dialect,&sqlite,1).await.unwrap().value,io_route(&sqlite,&dialect,1).await.unwrap().value]{assert_eq!(route.hops.len(),1);assert_eq!(route.fidelity,store::io_schema::IoFidelity::Exact);}
 for word in words(){let expected=Owned::new(full(word));for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let bytes=io_export_sqlite_snapshot(&dialect,&*expected,encoding,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;let actual=Owned::new(io_import_sqlite_snapshot::<DagSnapshot>(&dialect,&bytes,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value);assert_full(&actual,&expected);assert_eq!(actual.encode_pack(),expected.encode_pack());assert_eq!(actual.print_dsl(),expected.print_dsl());
 let script=r#"import{Database}from'bun:sqlite';const d=Database.deserialize(await Bun.stdin.bytes(),{safeIntegers:true});const widths=JSON.parse(process.argv[1]);widths.semio_snapshot=6;if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const names=d.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all().map(r=>r.name);if(names.length!==41||JSON.stringify(names)!==JSON.stringify(Object.keys(widths).sort()))throw Error('all40DagPlusMetadata');for(const[name,width]of Object.entries(widths))if(d.query('PRAGMA table_info('+name+')').all().length!==width)throw Error(name);if(d.query('SELECT COUNT(*) AS n FROM dag_node').get().n!==18n||d.query('SELECT COUNT(*) AS n FROM dag_edge').get().n!==2n)throw Error('allOriginalNodeVariantsAndEdges');const m=d.query('SELECT artifact_kind,standard,subset,native_encoding FROM semio_snapshot').get();if(m.artifact_kind!=='dag.host_snapshot'||m.standard!=='1'||m.subset!=='*'||m.native_encoding!==process.argv[2])throw Error('metadata');d.query('UPDATE dag_node SET name=? WHERE id=1').run('independent 日本\u0000');await Bun.write(Bun.stdout,d.serialize());d.close();"#;
 let mut child=Command::new("bun").args(["--eval",script,&laws()["tableWidths"].to_string(),encoding.as_str()]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let out=child.wait_with_output().unwrap();assert!(out.status.success(),"{}",String::from_utf8_lossy(&out.stderr));let actual=Owned::new(io_import_sqlite_snapshot::<DagSnapshot>(&dialect,&out.stdout,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value);let mut literal=full(word);literal.nodes[0].name="independent 日本\0".into();let literal=Owned::new(literal);assert_full(&actual,&literal);assert_eq!(actual.encode_pack(),literal.encode_pack());assert_eq!(actual.print_dsl(),literal.print_dsl());
 }}
 eprintln!("[DEBUG] DAG normal native registration preserves public exact owner and independent SQL scalar edit");
}
