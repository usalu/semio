//! 🧱️ Genuine Block2d Native capability and complete owned-field baselines.
use crate::standards::v1::subsets::any::io::sqlite::snapshot::Block2dSnapshot;
use crate::*;
use store::{ArtifactDsl, ArtifactPack};
fn laws() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()
}
fn words() -> Vec<u64> {
    laws()["binary64Bits"].as_array().unwrap().iter().map(|v| u64::from_str_radix(v.as_str().unwrap(), 16).unwrap()).collect()
}
fn full(word: u64) -> Block2dSnapshot {
    let f = f64::from_bits(word);
    let l = laws();
    let text = l["nativeCase"]["literal"].as_str().unwrap();
    Block2dSnapshot {
        schema: String::new(),
        node_kind: BlockKindIdentity { id: String::new(), name: text.into(), label: String::new(), variant: Some(String::new()), description: text.into(), icon: Some(String::new()), unit: Some(String::new()) },
        presentation: Block2dPresentation { shape: Some(String::new()), radius: Some(f), width: Some(f), height: Some(f), color: Some(String::new()), icon_kind: Some(String::new()) },
        handle_kinds: vec![Block2dHandleKind { id: String::new(), name: text.into(), label: String::new(), color: String::new(), default_wire_kind: text.into() }],
        handles: vec![Block2dHandleTemplate { id: String::new(), handle_kind: l["nativeCase"]["unresolvedHandleKind"].as_str().unwrap().into(), angle: f, radius: f }],
        compatibility: vec![BlockCompatibilityRule { id: String::new(), source: String::new(), target: text.into(), bidirectional: true }],
        attributes: vec![BlockAttribute { key: String::new(), value: text.into(), definition: Some(String::new()) }],
        authors: vec![BlockAuthor { id: String::new(), name: text.into(), email: Some(String::new()) }],
        camera2d: BlockCamera2d { x: f, y: f, zoom: f },
        meta: BlockMeta { description: text.into() },
    }
}
fn assert_words(s: &Block2dSnapshot, w: u64) {
    for f in [s.presentation.radius.unwrap(), s.presentation.width.unwrap(), s.presentation.height.unwrap(), s.handles[0].angle, s.handles[0].radius, s.camera2d.x, s.camera2d.y, s.camera2d.zoom] {
        assert_eq!(f.to_bits(), w);
    }
}
#[test]
fn sqlite_snapshot_block2d_actual_bare_capability_is_present() {
    assert!(<Block2dSnapshot as ArtifactPack>::sqlite_snapshot_codec().is_some());
}
#[test]
fn sqlite_snapshot_block2d_actual_declared_codec_has_owned_sqlite_capability() {
    let d = crate::standards::v1::subsets::any::io::io();
    assert!(d.native.codec.snapshot_sqlite.is_some());
}
#[test]
fn sqlite_snapshot_block2d_all_fields_and_words_survive_actual_text_and_pack() {
    for word in words() {
        let s = full(word);
        let text = s.print_dsl();
        let parsed = Block2dSnapshot::parse_dsl(&text).unwrap();
        assert_words(&parsed, word);
        assert_eq!(parsed.print_dsl(), text);
        let parsed = Block2dSnapshot::decode_pack(&s.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()).unwrap();
        assert_words(&parsed, word);
        assert_eq!(parsed.print_dsl(), text);
    }
}
#[test]
fn sqlite_snapshot_block2d_optional_absence_remains_distinct_from_present_empty() {
    let mut s = full(0);
    s.node_kind.variant = None;
    s.node_kind.icon = None;
    s.node_kind.unit = None;
    s.presentation = Block2dPresentation::default();
    s.attributes[0].definition = None;
    s.authors[0].email = None;
    let text = s.print_dsl();
    for p in [Block2dSnapshot::parse_dsl(&text).unwrap(), Block2dSnapshot::decode_pack(&s.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()).unwrap()] {
        assert_eq!(p.print_dsl(), text);
        assert_eq!(p.presentation, s.presentation);
        assert_eq!(p.node_kind.variant, None);
        assert_eq!(p.authors[0].email, None);
    }
}
#[test]
fn sqlite_snapshot_block2d_controlled_record_constructs_every_owned_field() {
    for word in words() {
        let s = full(word);
        let mut accepted = |_| true;
        let mut c = semio_framework_value::NativeEncodeControl::new(1 << 20, &mut accepted);
        let spec = Block2dSnapshot::__dsl_spec_producer();
        (spec.encoding)(&mut c).unwrap();
        let record = s.__dsl_to_record_controlled(&mut c).unwrap();
        let mut accepted = |_| true;
        let mut c = semio_framework_value::NativeDecodeControl::new(1 << 20, &mut accepted);
        (spec.decoding)(&mut c).unwrap();
        let p = Block2dSnapshot::__dsl_from_record_controlled(&record, &mut c).unwrap();
        assert_words(&p, word);
        assert_eq!(p.print_dsl(), s.print_dsl());
    }
}

#[test]
fn sqlite_snapshot_block2d_actual_declared_json_preserves_every_owned_word() {
    for word in words() {
        let s = full(word);
        let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any::json_text(&s);
        let independent: serde_json::Value = serde_json::from_str(&bytes).unwrap();
        for value in [&independent["presentation"]["radius"], &independent["presentation"]["width"], &independent["presentation"]["height"], &independent["handles"][0]["angle"], &independent["handles"][0]["radius"], &independent["camera2d"]["x"], &independent["camera2d"]["y"], &independent["camera2d"]["zoom"]] {
            assert_eq!(value.as_object().unwrap().len(), 1);
            assert_eq!(u64::from_str_radix(value["bits"].as_str().unwrap(), 16).unwrap(), word);
        }
        let p = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::from_json_text(&bytes).unwrap();
        assert_words(&p, word);
        assert_eq!(p.print_dsl(), s.print_dsl());
    }
}

#[test]
fn sqlite_snapshot_block2d_independent_sqlite_reads_the_literal_table_and_word_contract() {
    use std::process::Command;
    let sql = include_str!("../🗄️.sql");
    let script = r#"import{Database}from'bun:sqlite';const d=new Database(':memory:',{safeIntegers:true});d.exec(process.argv[1]);const expected=JSON.parse(process.argv[2]);const tables=d.query('SELECT name FROM sqlite_schema WHERE type=\'table\'').all();if(tables.length!==Object.keys(expected).length)throw Error('tablecount');for(const{name}of tables){if(d.query('PRAGMA table_info('+name+')').all().length!==expected[name])throw Error('width '+name)}d.exec('CREATE TABLE independent_words(ordinal INTEGER PRIMARY KEY,bits INTEGER NOT NULL)');const words=JSON.parse(process.argv[3]);const insert=d.query('INSERT INTO independent_words VALUES(?,?)');for(let i=0;i<words.length;i++)insert.run(i,BigInt.asIntN(64,BigInt('0x'+words[i])));const rows=d.query('SELECT bits FROM independent_words ORDER BY ordinal').all();for(let i=0;i<rows.length;i++)if(BigInt.asUintN(64,rows[i].bits).toString(16).padStart(16,'0')!==words[i])throw Error('word');d.close();"#;
    let widths = r#"{"block2_document":2,"block2_kind":9,"block2_presentation":14,"block2_handle_kind":8,"block2_handle":11,"block2_compatibility":7,"block2_attribute":6,"block2_author":6,"block2_camera2d":11,"block2_meta":3}"#;
    let output = Command::new("bun").args(["-e", script, sql, widths, &laws()["binary64Bits"].to_string()]).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}

fn independent_edit(bytes: &[u8], sql: &str) -> Vec<u8> {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let script = r#"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});try{if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');if(db.query('SELECT name FROM sqlite_schema WHERE type=\'table\'').all().length!==10)throw Error('tables');db.exec(process.argv[1]);process.stdout.write(db.serialize())}finally{db.close()}"#;
    let mut child = Command::new("bun").args(["-e", script, sql]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    output.stdout
}
#[test]
fn sqlite_snapshot_block2d_independent_edited_physical_database_preserves_every_word() {
    use store::{sqlite_snapshot::*, ArtifactSqliteSnapshot};
    for word in words() {
        let expected = full(word);
        let database = expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        assert_eq!(database.tables.len(), 10);
        assert_eq!(database.tables.iter().map(|t| t.rows.len()).sum::<usize>(), 10);
        let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
        let edited = import_sqlite_database(&independent_edit(&bytes, "UPDATE block2_meta SET description='independent';UPDATE block2_handle SET handle_kind='unresolved edited'"), SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
        let actual = Block2dSnapshot::from_sqlite_database(&edited, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        assert_words(&actual, word);
        assert_eq!(actual.meta.description, "independent");
        assert_eq!(actual.handles[0].handle_kind, "unresolved edited");
        let mut expected = expected;
        expected.meta.description = "independent".into();
        expected.handles[0].handle_kind = "unresolved edited".into();
        assert_eq!(actual.print_dsl(), expected.print_dsl());
    }
}
#[test]
fn sqlite_snapshot_block2d_independent_malformed_entities_refuse() {
    use store::{sqlite_snapshot::*, ArtifactSqliteSnapshot};
    let database = full(0).to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    for edit in [
        "UPDATE block2_handle SET ordinal=2",
        "UPDATE block2_camera2d SET x_ieee754_bits=1",
        "UPDATE block2_camera2d SET x_numeric_class='nan'",
        "DELETE FROM block2_kind",
        "UPDATE block2_compatibility SET bidirectional=3",
        "INSERT INTO block2_meta VALUES(99,1,'duplicate')",
        "UPDATE block2_attribute SET document_id=999",
    ] {
        let malformed = import_sqlite_database(&independent_edit(&bytes, edit), SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
        assert!(Block2dSnapshot::from_sqlite_database(&malformed, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err(), "{edit}");
    }
}
#[test]
fn sqlite_snapshot_block2d_row_frontier_and_real_four_phase_cancellation() {
    use store::{sqlite_snapshot::*, ArtifactSqliteSnapshot};
    let mut expected = full(0);
    expected.node_kind.description = "😀".repeat(32768);
    let limits = SqliteDatabaseLimits { max_rows: 10, ..SqliteDatabaseLimits::default() };
    let database = expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    let short = SqliteDatabaseLimits { max_rows: 9, ..limits };
    assert!(expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, short)).is_err());
    let native = store::io_schema::IoPayload::Text(expected.print_dsl());
    assert!(Block2dSnapshot::decode_sqlite_snapshot_native(&native, &mut SqliteSnapshotControl::new(&mut |_| true, short)).is_err());
    assert!(expected.encode_sqlite_snapshot_native(SnapshotEncoding::Text, &mut SqliteSnapshotControl::new(&mut |_| true, short)).is_err());
    for phase in [SqliteSnapshotPhase::ProjectSnapshot, SqliteSnapshotPhase::ReconstructSnapshot, SqliteSnapshotPhase::DecodeNative, SqliteSnapshotPhase::EncodeNative] {
        let mut saw = false;
        let mut callback = |e: SqliteSnapshotProgress| {
            if e.phase == phase && e.total >= 65536 && e.completed >= 65536 && e.completed < e.total {
                saw = true;
                false
            } else {
                true
            }
        };
        let mut control = SqliteSnapshotControl::new(&mut callback, limits);
        let refused = match phase {
            SqliteSnapshotPhase::ProjectSnapshot => expected.to_sqlite_database(&mut control).is_err(),
            SqliteSnapshotPhase::ReconstructSnapshot => Block2dSnapshot::from_sqlite_database(&database, &mut control).is_err(),
            SqliteSnapshotPhase::DecodeNative => Block2dSnapshot::decode_sqlite_snapshot_native(&native, &mut control).is_err(),
            SqliteSnapshotPhase::EncodeNative => expected.encode_sqlite_snapshot_native(SnapshotEncoding::Text, &mut control).is_err(),
            _ => unreachable!(),
        };
        drop(control);
        assert!(refused && saw, "{phase:?}");
    }
}
#[test]
fn sqlite_snapshot_block2d_actual_erased_declared_binary_text_retains_full_domain() {
    use store::sqlite_snapshot::*;
    let codec = crate::standards::v1::subsets::any::io::io().native.codec.snapshot_sqlite.unwrap();
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.block.block2d".into(), standard: "1".into(), subset: "*".into() };
    for word in words() {
        let expected = full(word);
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
            let native = match encoding {
                SnapshotEncoding::Binary => store::io_schema::IoPayload::Binary(expected.encode_pack()),
                SnapshotEncoding::Text => store::io_schema::IoPayload::Text(expected.print_dsl()),
            };
            let database = (codec.export)(BLOCK_2D_SCHEMA, &dialect, &native, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
            assert_eq!(database.tables.len(), 10);
            assert_eq!(database.table("block2_document").unwrap().single_row().unwrap().text(1).unwrap(), "");
            let native = (codec.import)(BLOCK_2D_SCHEMA, &dialect, database, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
            let actual = match native {
                store::io_schema::IoPayload::Binary(v) => Block2dSnapshot::decode_pack(&v).unwrap(),
                store::io_schema::IoPayload::Text(v) => Block2dSnapshot::parse_dsl(&v).unwrap(),
            };
            assert_words(&actual, word);
            assert_eq!(actual.print_dsl(), expected.print_dsl());
        }
    }
}

#[cfg(feature="component-app-assembly")]
mod populated_public_owner {
use crate::standards::v1::subsets::any::io::sqlite::snapshot::tests::*;use semio_framework_plugin::__semio_dispatch_PluginApp;use semio_framework_plugin::plugin_app_close_prelude::*;use store::sqlite_snapshot::{SnapshotEncoding,SqliteDatabaseLimits};
semio_framework_dispatch_macros::dyn_enum_close! {
/// 🗃️ Actual owner editor/viewer app wrappers for public I/O registration.
pub(crate) enum PublicApps: PluginApp {Editor(VcsArtifactApp<EditorApp<crate::editor::block2d::Block2dPlayApp>>),Viewer(VcsArtifactApp<ViewerApp<crate::viewer::block2d::Block2dViewer>>),}
}
struct PublicOwned(Option<Block2dSnapshot>);impl PublicOwned{fn new(value:Block2dSnapshot)->Self{Self(Some(value))}fn get(&self)->&Block2dSnapshot{self.0.as_ref().unwrap()}}impl Drop for PublicOwned{fn drop(&mut self){if let Some(value)=self.0.take(){<Block2dSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(value);}}}
fn exact(a:&Block2dSnapshot,b:&Block2dSnapshot){use semio_framework_value::{DslValue,Number,ToValue};fn same(a:&DslValue,b:&DslValue){match(a,b){(DslValue::Number(Number::Float(a)),DslValue::Number(Number::Float(b)))=>assert_eq!(a.to_bits(),b.to_bits()),(DslValue::Array(a),DslValue::Array(b))=>{assert_eq!(a.len(),b.len());for(a,b)in a.iter().zip(b){same(a,b);}},(DslValue::Object(a),DslValue::Object(b))=>{assert_eq!(a.len(),b.len());for((ka,a),(kb,b))in a.iter().zip(b){assert_eq!(ka,kb);same(a,b);}},_=>assert_eq!(a,b)}}same(&a.to_value(),&b.to_value());}

fn register(){semio_framework_plugin::Plugin::<PublicApps>::builder("block").label("Populated public SQLite").version("0.0.1").package_id("semio:block").declare_artifact(crate::artifact::<PublicApps>()).try_build().unwrap();}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_block2d_populated_actual_app_public_both_forms(){
use {semio_framework_artifact_reference::ArtifactDialect,store::io::io_mechanism::io_export_sqlite_snapshot,store::io::io_mechanism::io_import_sqlite_snapshot,store::io::io_mechanism::io_route};use store::io_schema::{IoFidelity,SQLITE_SNAPSHOT};
register();let dialect=ArtifactDialect{artifact_kind:"s.block.block2d".into(),standard:"1".into(),subset:"*".into()};let sqlite=ArtifactDialect::from(SQLITE_SNAPSHOT);for route in[io_route(&dialect,&sqlite,1).await.unwrap().value,io_route(&sqlite,&dialect,1).await.unwrap().value]{assert_eq!(route.hops.len(),1);assert_eq!(route.fidelity,IoFidelity::Exact);}
for word in words(){let expected=PublicOwned::new(full(word));for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let bytes=io_export_sqlite_snapshot(&dialect,expected.get(),encoding,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;let actual=PublicOwned::new(io_import_sqlite_snapshot::<Block2dSnapshot>(&dialect,&bytes,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value);exact(actual.get(),expected.get());assert_eq!((actual.get()).encode_pack(),(expected.get()).encode_pack());assert_eq!((actual.get()).print_dsl(),(expected.get()).print_dsl());}}
eprintln!("[DEBUG] block2d actual app public both forms preserve complete owner and words");
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_block2d_populated_public_independent_edit_retirement(){
use {semio_framework_artifact_reference::ArtifactDialect,store::io::io_mechanism::io_export_sqlite_snapshot,store::io::io_mechanism::io_import_sqlite_snapshot};use std::{io::Write,process::{Command,Stdio}};
register();let dialect=ArtifactDialect{artifact_kind:"s.block.block2d".into(),standard:"1".into(),subset:"*".into()};
for word in words(){let expected=PublicOwned::new(full(word));for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let bytes=io_export_sqlite_snapshot(&dialect,expected.get(),encoding,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;
let script=r#"import{Database}from'bun:sqlite';const d=Database.deserialize(await Bun.stdin.bytes(),{safeIntegers:true});if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');if(d.query('SELECT COUNT(*) AS n FROM sqlite_schema WHERE type=\'table\'').get().n!==11n)throw Error('table census');if(d.query('SELECT COUNT(*) AS n FROM block2_document').get().n!==1n)throw Error('literalRows:block2_document');if(d.query('SELECT COUNT(*) AS n FROM block2_kind').get().n!==1n)throw Error('literalRows:block2_kind');if(d.query('SELECT COUNT(*) AS n FROM block2_presentation').get().n!==1n)throw Error('literalRows:block2_presentation');if(d.query('SELECT COUNT(*) AS n FROM block2_handle_kind').get().n!==1n)throw Error('literalRows:block2_handle_kind');if(d.query('SELECT COUNT(*) AS n FROM block2_handle').get().n!==1n)throw Error('literalRows:block2_handle');if(d.query('SELECT COUNT(*) AS n FROM block2_compatibility').get().n!==1n)throw Error('literalRows:block2_compatibility');if(d.query('SELECT COUNT(*) AS n FROM block2_attribute').get().n!==1n)throw Error('literalRows:block2_attribute');if(d.query('SELECT COUNT(*) AS n FROM block2_author').get().n!==1n)throw Error('literalRows:block2_author');if(d.query('SELECT COUNT(*) AS n FROM block2_camera2d').get().n!==1n)throw Error('literalRows:block2_camera2d');if(d.query('SELECT COUNT(*) AS n FROM block2_meta').get().n!==1n)throw Error('literalRows:block2_meta');const m=d.query('SELECT * FROM semio_snapshot').get();if(JSON.stringify(Object.keys(m))!==JSON.stringify(['id','artifact_kind','standard','subset','schema_version','native_encoding'])||m.id!==1n||m.schema_version!==1n)throw Error('completePublicMetadata');if(m.artifact_kind!=='s.block.block2d'||m.standard!=='1'||m.subset!=='*'||m.native_encoding!==process.argv[1])throw Error('metadata');d.query('UPDATE block2_kind SET description=? WHERE id=1').run('independent 日本\u0000');await Bun.write(Bun.stdout,d.serialize());d.close();"#;
let mut child=Command::new("bun").args(["--eval",script,encoding.as_str(),&(laws()).to_string()]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let out=child.wait_with_output().unwrap();assert!(out.status.success(),"{}",String::from_utf8_lossy(&out.stderr));let actual=PublicOwned::new(io_import_sqlite_snapshot::<Block2dSnapshot>(&dialect,&out.stdout,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value);let mut literal=full(word);literal.node_kind.description="independent 日本\0".into();let literal=PublicOwned::new(literal);exact(actual.get(),literal.get());assert_eq!((actual.get()).encode_pack(),(literal.get()).encode_pack());assert_eq!((actual.get()).print_dsl(),(literal.get()).print_dsl());}}
eprintln!("[DEBUG] block2d independent public SQL edit complete owner and retirement");
}
}
