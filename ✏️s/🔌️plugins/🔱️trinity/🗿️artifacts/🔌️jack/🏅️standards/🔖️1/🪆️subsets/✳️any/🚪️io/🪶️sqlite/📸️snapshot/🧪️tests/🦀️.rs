//! 🔌️ Authentic Jack parent Native baselines, before semantic provider opt-in.
use crate::*;
use semio_framework_value::{FromValue, ToValue, ValueType};
use store::{ArtifactDsl, ArtifactPack};
fn laws() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()
}
fn words() -> Vec<u64> {
    laws()["binary64Bits"].as_array().unwrap().iter().map(|v| u64::from_str_radix(v.as_str().unwrap(), 16).unwrap()).collect()
}
fn value_type(v: &serde_json::Value) -> ValueType {
    match v["kind"].as_str().unwrap() {
        "boolean" => ValueType::Boolean,
        "integer" => ValueType::Integer,
        "decimal" => ValueType::Decimal,
        "text" => ValueType::Text,
        "any" => ValueType::Any,
        "schema" => ValueType::Schema(v["of"].as_str().unwrap().into()),
        "list" => ValueType::List(Box::new(value_type(&v["of"]))),
        _ => panic!("neutral type variant"),
    }
}
fn properties() -> Vec<PropertyDef> {
    laws()["nativeCase"]["manifest"]["nodeKinds"][0]["properties"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| PropertyDef {
            name: p["name"].as_str().unwrap().into(),
            kind: match p["kind"].as_str().unwrap() {
                "data" => PropertyKind::Data,
                "derived" => PropertyKind::Derived,
                _ => panic!("neutral property kind"),
            },
            value_type: value_type(&p["valueType"]),
            expr: p.get("expr").map(|v| v.as_str().unwrap().into()),
        })
        .collect()
}
fn full(word: u64, address: usize) -> JackSnapshot {
    let l = laws();
    let c = &l["childAddresses"][address];
    let content = store::ArtifactChild::new(
        c["childId"].as_str().unwrap().into(),
        semio_framework_artifact_reference::ArtifactRef {
            artifact_id: c["artifactId"].as_str().unwrap().into(),
            dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: c["artifactKind"].as_str().unwrap().into(), standard: c["standard"].as_str().unwrap().into(), subset: c["subset"].as_str().unwrap().into() },
        },
    );
    let f = f64::from_bits(word);
    JackSnapshot {
        schema: l["documentSchema"].as_str().unwrap().into(),
        name: l["name"].as_str().unwrap().into(),
        manifest_id: Some(l["manifestId"].as_str().unwrap().into()),
        manifest: Manifest {
            node_kinds: (0..2).map(|_| NodeKindDef { name: String::new(), properties: properties(), port_kinds: vec![String::new(), l["literal"].as_str().unwrap().into(), String::new()] }).collect(),
            edge_kinds: (0..2).map(|_| EdgeKindDef { name: String::new(), properties: properties() }).collect(),
            port_kinds: [PortDirection::In, PortDirection::Out].into_iter().map(|direction| PortKindDef { name: String::new(), direction, properties: properties() }).collect(),
        },
        camera: Camera { x: f, y: f, zoom: f },
        content,
        root_node_id: Some(l["rootNodeId"].as_str().unwrap().into()),
        query: l["query"].as_str().unwrap().into(),
    }
}
struct Owned(Option<JackSnapshot>);
impl Owned {
    fn new(v: JackSnapshot) -> Self {
        Self(Some(v))
    }
}
impl std::ops::Deref for Owned {
    type Target = JackSnapshot;
    fn deref(&self) -> &JackSnapshot {
        self.0.as_ref().unwrap()
    }
}
impl Drop for Owned {
    fn drop(&mut self) {
        if let Some(v) = self.0.take() {
            crate::retire_owned_to_terminal(v).expect("Jack fixture retirement");
        }
    }
}
fn assert_full(a: &JackSnapshot, b: &JackSnapshot, word: u64) {
    assert_eq!(a.schema, b.schema);
    assert_eq!(a.name, b.name);
    assert_eq!(a.manifest_id, b.manifest_id);
    assert_eq!(a.manifest, b.manifest);
    assert_eq!(a.root_node_id, b.root_node_id);
    assert_eq!(a.query, b.query);
    assert_eq!(a.content, b.content);
    for f in [a.camera.x, a.camera.y, a.camera.zoom] {
        assert_eq!(f.to_bits(), word);
    }
}
#[test]
fn sqlite_snapshot_jack_actual_bare_parent_owns_capability() {
    assert!(JackSnapshot::sqlite_snapshot_codec().is_some(), "Jack parent has no owned SQLite capability");
}
#[test]
fn sqlite_snapshot_jack_binary_preserves_inline_manifest_and_literal_content() {
    let expected = Owned::new(full(0, 0));
    let bytes = expected.encode_pack_with(&store::PackEncodeOptions::default()).unwrap();
    let actual = Owned::new(JackSnapshot::decode_pack(&bytes).unwrap());
    assert_full(&actual, &expected, 0);
}
#[test]
fn sqlite_snapshot_jack_text_preserves_inline_manifest_and_literal_content() {
    let expected = Owned::new(full(0, 1));
    let actual = Owned::new(JackSnapshot::parse_dsl(&expected.print_dsl()).unwrap());
    assert_full(&actual, &expected, 0);
}
#[test]
fn sqlite_snapshot_jack_every_native_camera_word_preserves_full_parent() {
    for word in words() {
        let expected = Owned::new(full(word, 0));
        for binary in [false, true] {
            let actual = Owned::new(if binary { JackSnapshot::decode_pack(&expected.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()).unwrap() } else { JackSnapshot::parse_dsl(&expected.print_dsl()).unwrap() });
            assert_full(&actual, &expected, word);
        }
    }
}
#[test]
fn sqlite_snapshot_jack_child_coordinates_survive_attached_and_unresolved_native_owners() {
    for address in 0..2 {
        for attached in [false, true] {
            let mut value = full(0, address);
            if attached {
                crate::materialize_jack_content(&mut value.content, Vec::new(), Vec::new())
            }
            let expected = Owned::new(value);
            for binary in [false, true] {
                let actual = Owned::new(if binary { JackSnapshot::decode_pack(&expected.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()).unwrap() } else { JackSnapshot::parse_dsl(&expected.print_dsl()).unwrap() });
                assert_eq!(actual.content, expected.content);
            }
        }
    }
}
#[test]
fn sqlite_snapshot_jack_actual_controlled_typed_value_construction_preserves_all_fields() {
    let expected = Owned::new(full(0, 0));
    let mut yes = |_| true;
    let mut output = semio_framework_value::NativeEncodeControl::new(32 << 20, &mut yes);
    let v = <JackSnapshot as ToValue>::to_value_controlled(&expected, &mut output).unwrap();
    let mut yes = |_| true;
    let mut input = semio_framework_value::NativeDecodeControl::new(32 << 20, &mut yes);
    let actual = Owned::new(<JackSnapshot as FromValue>::from_value_controlled(&v, &mut input).unwrap());
    assert_full(&actual, &expected, 0);
}
#[test]
fn sqlite_snapshot_jack_declared_json_camera_words_are_closed_and_exact() {
    use crate::standards::v1::subsets::any::io::{export::serializers::artifacts::json::v_rfc8259::any as writer, import::deserializers::artifacts::json::v_rfc8259::any as reader};
    for word in words() {
        let expected = Owned::new(full(word, 0));
        let bytes = writer::serialize_bytes(&expected).unwrap();
        let oracle: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        for field in ["x", "y", "zoom"] {
            assert_eq!(oracle["camera"][field]["bits"].as_str(), Some(format!("{word:016x}").as_str()));
        }
        let actual = Owned::new(reader::deserialize_bytes(&bytes).unwrap());
        assert_full(&actual, &expected, word);
    }
}

#[test]
fn sqlite_snapshot_jack_public_json_snapshot_api_preserves_the_same_literal_owned_parent() {
    for word in words() {
        let expected = Owned::new(full(word, 1));
        let json = expected.to_json().unwrap();
        let oracle: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(oracle["content"]["childId"].as_str(), Some(expected.content.child_id.as_str()));
        assert_eq!(oracle["content"]["target"]["artifactId"].as_str(), Some(expected.content.target.artifact_id.as_str()));
        assert!(oracle.get("nodes").is_none());
        assert!(oracle.get("edges").is_none());
        for field in ["x", "y", "zoom"] {
            assert_eq!(oracle["camera"][field]["bits"].as_str(), Some(format!("{word:016x}").as_str()));
        }
        let actual = Owned::new(JackSnapshot::from_json(&json).unwrap());
        assert_full(&actual, &expected, word);
    }
}
#[test]
fn sqlite_snapshot_jack_independent_sqlite_understands_all_authored_entities_and_words() {
    let script = r#"import{Database}from'bun:sqlite';const d=new Database(':memory:',{safeIntegers:true});d.exec(process.argv[1]);const l=JSON.parse(process.argv[2]);const rows=d.query('SELECT name FROM sqlite_schema WHERE type=\'table\'').all();if(rows.length!==13)throw Error('table count');for(const{name}of rows)if(d.query('PRAGMA table_info('+name+')').all().length!==l.tableWidths[name])throw Error(name);d.exec("INSERT INTO jack_document VALUES(1,'trinity.graph','name',NULL,'','query');INSERT INTO jack_content_child VALUES(1,1,'','independent target','s.stdio.semio','v1','graph')");for(const hex of l.binary64Bits){const word=BigInt('0x'+hex),data=new DataView(new ArrayBuffer(8));data.setBigUint64(0,word);const f=data.getFloat64(0),kind=Number.isNaN(f)?'nan':f===Infinity?'positiveInfinity':f===-Infinity?'negativeInfinity':'finite',cells=[1n,1n,...Array(3).fill(Number.isNaN(f)?null:f),...Array.from({length:3},()=>[BigInt.asIntN(64,word),kind]).flat()];d.query('INSERT OR REPLACE INTO jack_camera VALUES('+Array(11).fill('?').join(',')+')').run(...cells);const row=d.query('SELECT x_ieee754_bits,x_numeric_class FROM jack_camera').get();if(BigInt.asUintN(64,row.x_ieee754_bits).toString(16).padStart(16,'0')!==hex||row.x_numeric_class!==kind)throw Error('exact word')}if(d.query('PRAGMA foreign_key_check').all().length)throw Error('foreign keys');d.close();"#;
    let out = std::process::Command::new("bun").args(["-e", script, include_str!("../🗄️.sql"), &laws().to_string()]).output().unwrap();
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
}
#[test]
fn sqlite_snapshot_jack_actual_erased_both_formats_keep_queryable_parent() {
    use store::sqlite_snapshot::*;
    let codec = store::ArtifactCodec::bare::<JackSnapshot, crate::TrinityGraphMutation>(crate::TRINITY_GRAPH_SCHEMA).snapshot_sqlite.expect("Jack absent parent capability");
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.trinity.jack".into(), standard: "1".into(), subset: "*".into() };
    let expected = Owned::new(full(0, 0));
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let native = match encoding {
            SnapshotEncoding::Binary => store::io_schema::IoPayload::Binary(expected.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()),
            SnapshotEncoding::Text => store::io_schema::IoPayload::Text(expected.print_dsl()),
        };
        let db = (codec.export)(crate::TRINITY_GRAPH_SCHEMA, &dialect, &native, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
        assert_eq!(db.tables.len(), 13);
        assert_eq!(db.table("jack_document").unwrap().single_row().unwrap().text(2).unwrap(), expected.name);
        let native = (codec.import)(crate::TRINITY_GRAPH_SCHEMA, &dialect, db, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
        let actual = Owned::new(match native {
            store::io_schema::IoPayload::Text(v) => JackSnapshot::parse_dsl(&v).unwrap(),
            store::io_schema::IoPayload::Binary(v) => JackSnapshot::decode_pack(&v).unwrap(),
        });
        assert_full(&actual, &expected, 0);
    }
}

fn independent_jack_file(bytes: &[u8], edit: &str) -> Vec<u8> {
    use std::io::Write;
    let script = r#"import{Database}from'bun:sqlite';const data=await Bun.stdin.bytes();const d=Database.deserialize(data,{safeIntegers:true});if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok')throw Error('integrity');if(d.query('PRAGMA foreign_key_check').all().length)throw Error('FK');const counts=JSON.parse(process.argv[2]);for(const[name,count]of Object.entries(counts))if(d.query('SELECT count(*) AS n FROM '+name).get().n!==BigInt(count))throw Error(name);d.exec(process.argv[1]);process.stdout.write(d.serialize());d.close();"#;
    let mut child = std::process::Command::new("bun").args(["-e", script, edit, &laws()["tableRowCounts"].to_string()]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(bytes).unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    result.stdout
}
#[test]
fn sqlite_snapshot_jack_all13_tables_and153_rows_survive_independent_edits() {
    use store::sqlite_snapshot::*;
    use store::ArtifactSqliteSnapshot;
    for word in words() {
        let expected = Owned::new(full(word, 0));
        let mut cb = |_| true;
        let mut control = SqliteSnapshotControl::new(&mut cb, Default::default());
        let db = expected.to_sqlite_database(&mut control).unwrap();
        assert_eq!(db.tables.len(), 13);
        assert_eq!(db.tables.iter().map(|t| t.rows.len()).sum::<usize>(), 153);
        let bytes = export_sqlite_database(&db, Default::default(), &mut |_| true).unwrap();
        let edited = independent_jack_file(&bytes, "UPDATE jack_document SET manifest_id='still-independent',query='SQL query'; UPDATE jack_content_child SET child_id='alias!@/',artifact_id='unresolved!@/';");
        let db = import_sqlite_database(&edited, Default::default(), &mut |_| true).unwrap();
        let actual = Owned::new(JackSnapshot::from_sqlite_database(&db, &mut control).unwrap());
        let mut expected = Owned::new(full(word, 0));
        let v = expected.0.as_mut().unwrap();
        v.manifest_id = Some("still-independent".into());
        v.query = "SQL query".into();
        v.content.child_id = "alias!@/".into();
        v.content.target.artifact_id = "unresolved!@/".into();
        assert_full(&actual, &expected, word);
    }
}
#[test]
fn sqlite_snapshot_jack_independently_malformed_semantic_entities_are_refused() {
    use store::sqlite_snapshot::*;
    use store::ArtifactSqliteSnapshot;
    let mut cb = |_| true;
    let mut control = SqliteSnapshotControl::new(&mut cb, Default::default());
    let v = Owned::new(full(0, 0));
    let db = v.to_sqlite_database(&mut control).unwrap();
    let bytes = export_sqlite_database(&db, Default::default(), &mut |_| true).unwrap();
    for edit in laws()["invalidSqlEdits"].as_array().unwrap() {
        let file = independent_jack_file(&bytes, edit.as_str().unwrap());
        match import_sqlite_database(&file, Default::default(), &mut |_| true) {
            Err(_) => {}
            Ok(db) => assert!(JackSnapshot::from_sqlite_database(&db, &mut control).is_err(), "{edit}"),
        }
    }
}
#[test]
fn sqlite_snapshot_jack_exact_row_admission_and_all_four_real_interior_copy_phases() {
    use store::sqlite_snapshot::*;
    use store::ArtifactSqliteSnapshot;
    let value = Owned::new(full(0, 0));
    let mut cb = |_| true;
    let exact = SqliteDatabaseLimits { max_rows: 153, ..Default::default() };
    let short = SqliteDatabaseLimits { max_rows: 152, ..Default::default() };
    assert!(value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut cb, exact)).is_ok());
    assert!(value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut cb, short)).is_err());
    let payload = store::io_schema::IoPayload::Binary(value.encode_pack());
    assert!(JackSnapshot::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut cb, short)).is_err());
    assert!(value.encode_sqlite_snapshot_native(SnapshotEncoding::Binary, &mut SqliteSnapshotControl::new(&mut cb, short)).is_err());
    let mut large = Owned::new(full(0, 0));
    large.0.as_mut().unwrap().manifest.node_kinds[0].properties[0].expr = Some("引用😀".repeat(20000));
    let db = large.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut cb, Default::default())).unwrap();
    let payload = store::io_schema::IoPayload::Binary(large.encode_pack());
    for phase in [SqliteSnapshotPhase::ProjectSnapshot, SqliteSnapshotPhase::ReconstructSnapshot, SqliteSnapshotPhase::DecodeNative, SqliteSnapshotPhase::EncodeNative] {
        let mut hit = false;
        let mut callback = |p: SqliteSnapshotProgress| {
            if p.phase == phase && p.total >= 65536 && p.completed >= 65536 && p.completed < p.total {
                hit = true;
                false
            } else {
                true
            }
        };
        let mut c = SqliteSnapshotControl::new(&mut callback, Default::default());
        let refused = match phase {
            SqliteSnapshotPhase::ProjectSnapshot => large.to_sqlite_database(&mut c).is_err(),
            SqliteSnapshotPhase::ReconstructSnapshot => JackSnapshot::from_sqlite_database(&db, &mut c).is_err(),
            SqliteSnapshotPhase::DecodeNative => JackSnapshot::decode_sqlite_snapshot_native(&payload, &mut c).is_err(),
            SqliteSnapshotPhase::EncodeNative => large.encode_sqlite_snapshot_native(SnapshotEncoding::Binary, &mut c).is_err(),
            _ => unreachable!(),
        };
        assert!(refused);
        assert!(hit, "{:?}", phase);
    }
}

#[cfg(feature="component-app-assembly")]
mod populated_public_owner {
use crate::standards::v1::subsets::any::io::sqlite::snapshot::tests::*;use semio_framework_plugin::__semio_dispatch_PluginApp;use semio_framework_plugin::plugin_app_close_prelude::*;use store::sqlite_snapshot::{SnapshotEncoding,SqliteDatabaseLimits};
semio_framework_dispatch_macros::dyn_enum_close! {
/// 🗃️ Actual owner editor/viewer app wrappers for public I/O registration.
pub(crate) enum PublicApps: PluginApp {Editor(VcsArtifactApp<EditorApp<crate::editor::jack::TrinityJackPlayApp>,semio_s_artifact_stdio_semio::SemioMembers>),Viewer(VcsArtifactApp<ViewerApp<crate::viewer::jack::TrinityJackViewer>,semio_s_artifact_stdio_semio::SemioMembers>),}
}

fn register(){semio_framework_plugin::Plugin::<PublicApps>::builder("trinity").label("Populated public SQLite").version("0.0.1").package_id("semio:trinity").declare_artifact(crate::artifact::<PublicApps>()).try_build().unwrap();}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_jack_populated_actual_app_public_both_forms(){
use {semio_framework_artifact_reference::ArtifactDialect,store::io::io_mechanism::io_export_sqlite_snapshot,store::io::io_mechanism::io_import_sqlite_snapshot,store::io::io_mechanism::io_route};use store::io_schema::{IoFidelity,SQLITE_SNAPSHOT};
register();let dialect=ArtifactDialect{artifact_kind:"s.trinity.jack".into(),standard:"1".into(),subset:"*".into()};let sqlite=ArtifactDialect::from(SQLITE_SNAPSHOT);for route in[io_route(&dialect,&sqlite,1).await.unwrap().value,io_route(&sqlite,&dialect,1).await.unwrap().value]{assert_eq!(route.hops.len(),1);assert_eq!(route.fidelity,IoFidelity::Exact);}
for word in words(){let expected=Owned::new(full(word,1));for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let bytes=io_export_sqlite_snapshot(&dialect,&*expected,encoding,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;let actual=Owned::new(io_import_sqlite_snapshot::<JackSnapshot>(&dialect,&bytes,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value);assert_full(&actual,&expected,word);assert_eq!((&*actual).encode_pack(),(&*expected).encode_pack());assert_eq!((&*actual).print_dsl(),(&*expected).print_dsl());}}
eprintln!("[DEBUG] jack actual app public both forms preserve complete owner and words");
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_jack_populated_public_independent_edit_retirement(){
use {semio_framework_artifact_reference::ArtifactDialect,store::io::io_mechanism::io_export_sqlite_snapshot,store::io::io_mechanism::io_import_sqlite_snapshot};use std::{io::Write,process::{Command,Stdio}};
register();let dialect=ArtifactDialect{artifact_kind:"s.trinity.jack".into(),standard:"1".into(),subset:"*".into()};
for word in words(){let expected=Owned::new(full(word,1));for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let bytes=io_export_sqlite_snapshot(&dialect,&*expected,encoding,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;
let script=r#"import{Database}from'bun:sqlite';const d=Database.deserialize(await Bun.stdin.bytes(),{safeIntegers:true});if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');if(d.query('SELECT COUNT(*) AS n FROM sqlite_schema WHERE type=\'table\'').get().n!==14n)throw Error('table census');const m=d.query('SELECT * FROM semio_snapshot').get();if(JSON.stringify(Object.keys(m))!==JSON.stringify(['id','artifact_kind','standard','subset','schema_version','native_encoding'])||m.id!==1n||m.schema_version!==1n)throw Error('completePublicMetadata');if(m.artifact_kind!=='s.trinity.jack'||m.standard!=='1'||m.subset!=='*'||m.native_encoding!==process.argv[1])throw Error('metadata');d.query('UPDATE jack_document SET query=? WHERE id=1').run('independent 日本\u0000');await Bun.write(Bun.stdout,d.serialize());d.close();"#;
let mut child=Command::new("bun").args(["--eval",script,encoding.as_str(),&(laws()).to_string()]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let out=child.wait_with_output().unwrap();assert!(out.status.success(),"{}",String::from_utf8_lossy(&out.stderr));let actual=Owned::new(io_import_sqlite_snapshot::<JackSnapshot>(&dialect,&out.stdout,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value);let mut literal=full(word,1);literal.query="independent 日本\0".into();let literal=Owned::new(literal);assert_full(&actual,&literal,word);assert_eq!((&*actual).encode_pack(),(&*literal).encode_pack());assert_eq!((&*actual).print_dsl(),(&*literal).print_dsl());}}
eprintln!("[DEBUG] jack independent public SQL edit complete owner and retirement");
}
}
