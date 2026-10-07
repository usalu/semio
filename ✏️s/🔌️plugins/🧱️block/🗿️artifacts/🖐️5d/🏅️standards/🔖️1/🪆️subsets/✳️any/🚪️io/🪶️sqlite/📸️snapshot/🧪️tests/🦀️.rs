//! 🧱️ Genuine Block5d Native full-field words and real declaration baselines.
use crate::standards::v1::subsets::any::io::sqlite::snapshot::Block5dSnapshot;
use crate::*;
use store::{ArtifactDsl, ArtifactPack};
fn laws() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()
}
fn words() -> Vec<u64> {
    laws()["binary64Bits"].as_array().unwrap().iter().map(|v| u64::from_str_radix(v.as_str().unwrap(), 16).unwrap()).collect()
}
fn full(word: u64) -> Block5dSnapshot {
    let f = f64::from_bits(word);
    let l = laws();
    let text = l["nativeCase"]["literal"].as_str().unwrap();
    Block5dSnapshot {
        schema: String::new(),
        part_kind: BlockKindIdentity { id: String::new(), name: text.into(), label: String::new(), variant: Some(String::new()), description: text.into(), icon: Some(String::new()), unit: Some(String::new()) },
        part_2d: Block5dPart2d { shape: Some(String::new()), radius: Some(f), width: Some(f), height: Some(f), color: Some(String::new()), icon_kind: Some(String::new()) },
        part_3d: Block5dPart3d { orientation: Some([f; 4]), scale: Some([f; 3]) },
        representations: vec![BlockRepresentation {
            id: String::new(),
            name: text.into(),
            mesh_url: Some(String::new()),
            tags: vec![String::new(), text.into()],
            lod: Some(String::new()),
            description: text.into(),
            attributes: vec![BlockAttribute { key: String::new(), value: text.into(), definition: Some(String::new()) }],
        }],
        grip_kinds: vec![Block5dGripKind { id: String::new(), name: text.into(), label: String::new(), color: String::new(), default_rope_kind: text.into() }],
        grips: vec![Block5dGripTemplate { id: String::new(), grip_kind: l["nativeCase"]["unresolvedGripKind"].as_str().unwrap().into(), angle: f, radius_2d: f, position: [f; 3], direction: [f; 3], radius_3d: f }],
        compatibility: vec![BlockCompatibilityRule { id: String::new(), source: String::new(), target: text.into(), bidirectional: true }],
        attributes: vec![BlockAttribute { key: String::new(), value: text.into(), definition: Some(String::new()) }],
        authors: vec![BlockAuthor { id: String::new(), name: text.into(), email: Some(String::new()) }],
        camera2d: BlockCamera2d { x: f, y: f, zoom: f },
        camera3d: BlockCamera3d { position: [f; 3], target: [f; 3], zoom: f },
        meta: BlockMeta { description: text.into() },
    }
}
fn assert_words(s: &Block5dSnapshot, w: u64) {
    for f in [s.part_2d.radius.unwrap(), s.part_2d.width.unwrap(), s.part_2d.height.unwrap()]
        .into_iter()
        .chain(s.part_3d.orientation.unwrap())
        .chain(s.part_3d.scale.unwrap())
        .chain([s.grips[0].angle, s.grips[0].radius_2d, s.grips[0].radius_3d])
        .chain(s.grips[0].position)
        .chain(s.grips[0].direction)
        .chain([s.camera2d.x, s.camera2d.y, s.camera2d.zoom])
        .chain(s.camera3d.position)
        .chain(s.camera3d.target)
        .chain([s.camera3d.zoom])
    {
        assert_eq!(f.to_bits(), w);
    }
}
#[test]
fn sqlite_snapshot_block5d_actual_bare_capability_is_present() {
    assert!(<Block5dSnapshot as ArtifactPack>::sqlite_snapshot_codec().is_some());
}
#[test]
fn sqlite_snapshot_block5d_actual_declared_codec_has_owned_sqlite_capability() {
    let d = crate::standards::v1::subsets::any::io::io();
    assert!(d.native.codec.snapshot_sqlite.is_some());
}
#[test]
fn sqlite_snapshot_block5d_all_fields_and_words_survive_actual_text_and_pack() {
    for word in words() {
        let s = full(word);
        let text = s.print_dsl();
        let p = Block5dSnapshot::parse_dsl(&text).unwrap();
        assert_words(&p, word);
        assert_eq!(p.print_dsl(), text);
        let p = Block5dSnapshot::decode_pack(&s.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()).unwrap();
        assert_words(&p, word);
        assert_eq!(p.print_dsl(), text);
    }
}
#[test]
fn sqlite_snapshot_block5d_optional_absence_remains_distinct_from_present_empty() {
    let mut s = full(0);
    s.part_kind.variant = None;
    s.part_kind.icon = None;
    s.part_kind.unit = None;
    s.part_2d = Block5dPart2d::default();
    s.part_3d = Block5dPart3d::default();
    s.representations[0].mesh_url = None;
    s.representations[0].lod = None;
    s.representations[0].attributes[0].definition = None;
    s.attributes[0].definition = None;
    s.authors[0].email = None;
    let text = s.print_dsl();
    for p in [Block5dSnapshot::parse_dsl(&text).unwrap(), Block5dSnapshot::decode_pack(&s.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()).unwrap()] {
        assert_eq!(p.print_dsl(), text);
        assert_eq!(p.part_2d, s.part_2d);
        assert_eq!(p.part_3d, s.part_3d);
        assert_eq!(p.authors[0].email, None);
    }
}
#[test]
fn sqlite_snapshot_block5d_controlled_record_constructs_every_owned_field() {
    for word in words() {
        let s = full(word);
        let mut accepted = |_| true;
        let mut c = semio_framework_value::NativeEncodeControl::new(1 << 20, &mut accepted);
        let spec = Block5dSnapshot::__dsl_spec_producer();
        (spec.encoding)(&mut c).unwrap();
        let record = s.__dsl_to_record_controlled(&mut c).unwrap();
        let mut accepted = |_| true;
        let mut c = semio_framework_value::NativeDecodeControl::new(1 << 20, &mut accepted);
        (spec.decoding)(&mut c).unwrap();
        let p = Block5dSnapshot::__dsl_from_record_controlled(&record, &mut c).unwrap();
        assert_words(&p, word);
        assert_eq!(p.print_dsl(), s.print_dsl());
    }
}
#[test]
fn sqlite_snapshot_block5d_independent_sqlite_reads_the_real_fifteen_table_contract() {
    use std::process::Command;
    let sql = include_str!("../🗄️.sql");
    let script = r#"import{Database}from'bun:sqlite';const d=new Database(':memory:',{safeIntegers:true});d.exec(process.argv[1]);const expected=JSON.parse(process.argv[2]);const tables=d.query('SELECT name FROM sqlite_schema WHERE type=\'table\'').all();if(tables.length!==15)throw Error('tablecount');for(const{name}of tables){const columns=d.query('PRAGMA table_info('+name+')').all();if(columns.length!==expected[name])throw Error('width '+name)}const words=JSON.parse(process.argv[3]);d.exec('CREATE TABLE independent_words(ordinal INTEGER PRIMARY KEY,bits INTEGER NOT NULL)');const insert=d.query('INSERT INTO independent_words VALUES(?,?)');for(let i=0;i<words.length;i++)insert.run(i,BigInt.asIntN(64,BigInt('0x'+words[i])));const rows=d.query('SELECT bits FROM independent_words ORDER BY ordinal').all();for(let i=0;i<rows.length;i++)if(BigInt.asUintN(64,rows[i].bits).toString(16).padStart(16,'0')!==words[i])throw Error('word');d.close();"#;
    let widths = r#"{"block5_document":2,"block5_kind":9,"block5_part2d":14,"block5_part3d":23,"block5_representation":8,"block5_representation_tag":4,"block5_representation_attribute":6,"block5_grip_kind":8,"block5_grip":32,"block5_compatibility":7,"block5_attribute":6,"block5_author":6,"block5_camera2d":11,"block5_camera3d":23,"block5_meta":3}"#;
    let output = Command::new("bun").args(["-e", script, sql, widths, &laws()["binary64Bits"].to_string()]).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn sqlite_snapshot_block5d_actual_declared_json_preserves_every_owned_word() {
    for word in words() {
        let s = full(word);
        let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any::json_text(&s);
        let p = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::from_json_text(&bytes).unwrap();
        assert_words(&p, word);
        assert_eq!(p.print_dsl(), s.print_dsl());
    }
}

fn independent_edit(bytes: &[u8], sql: &str) -> Vec<u8> {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let script = r#"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});try{if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok')throw Error('integrity');if(db.query('PRAGMA foreign_key_check').all().length)throw Error('foreign key');if(db.query('SELECT name FROM sqlite_schema WHERE type=\'table\'').all().length!==15)throw Error('table count');db.exec(process.argv[1]);process.stdout.write(db.serialize())}finally{db.close()}"#;
    let mut child = Command::new("bun").args(["-e", script, sql]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    output.stdout
}
#[test]
fn sqlite_snapshot_block5d_full_relational_words_and_independent_edited_file_reconstruct() {
    use store::{ArtifactSqliteSnapshot, sqlite_snapshot::*};
    for word in words() {
        let expected = full(word);
        let database = expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        assert_eq!(database.tables.len(), 15);
        assert_eq!(database.tables.iter().map(|v| v.rows.len()).sum::<usize>(), 16);
        let actual = Block5dSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        assert_words(&actual, word);
        assert_eq!(actual.print_dsl(), expected.print_dsl());
        let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
        let bytes = independent_edit(&bytes, "UPDATE block5_meta SET description='independent edited metadata';UPDATE block5_grip SET grip_kind='independent unresolved semantic kind'");
        let edited = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
        let actual = Block5dSnapshot::from_sqlite_database(&edited, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        assert_words(&actual, word);
        assert_eq!(actual.meta.description, "independent edited metadata");
        assert_eq!(actual.grips[0].grip_kind, "independent unresolved semantic kind");
        assert_eq!(actual.schema, "");
        assert_eq!(actual.part_kind.description, expected.part_kind.description);
    }
}
#[test]
fn sqlite_snapshot_block5d_malformed_relational_ownership_and_optional_words_refuse() {
    use store::{ArtifactSqliteSnapshot, sqlite_snapshot::*};
    let expected = full(0);
    let database = expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    for edit in [
        "UPDATE block5_grip SET ordinal=2",
        "UPDATE block5_camera2d SET x_ieee754_bits=1",
        "UPDATE block5_camera2d SET x_numeric_class='nan'",
        "UPDATE block5_part3d SET orientation_y=NULL,orientation_y_ieee754_bits=NULL,orientation_y_numeric_class=NULL",
        "DELETE FROM block5_kind",
        "UPDATE block5_compatibility SET bidirectional=3",
        "UPDATE block5_representation_tag SET representation_id=99999",
        "INSERT INTO block5_meta VALUES(99,1,'duplicate singleton')",
    ] {
        let malformed = import_sqlite_database(&independent_edit(&bytes, edit), SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
        assert!(Block5dSnapshot::from_sqlite_database(&malformed, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err(), "{edit}");
    }
}
#[test]
fn sqlite_snapshot_block5d_exact_row_admission_and_four_real_copy_phase_cancellations() {
    use store::{ArtifactSqliteSnapshot, sqlite_snapshot::*};
    let mut expected = full(0);
    expected.part_kind.description = "😀".repeat(32768);
    let limits = SqliteDatabaseLimits { max_rows: 16, ..SqliteDatabaseLimits::default() };
    let database = expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    let short = SqliteDatabaseLimits { max_rows: 15, ..limits };
    assert!(expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, short)).is_err());
    let native = store::io_schema::IoPayload::Text(expected.print_dsl());
    assert!(Block5dSnapshot::decode_sqlite_snapshot_native(&native, &mut SqliteSnapshotControl::new(&mut |_| true, short)).is_err());
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
            SqliteSnapshotPhase::ReconstructSnapshot => Block5dSnapshot::from_sqlite_database(&database, &mut control).is_err(),
            SqliteSnapshotPhase::DecodeNative => Block5dSnapshot::decode_sqlite_snapshot_native(&native, &mut control).is_err(),
            SqliteSnapshotPhase::EncodeNative => expected.encode_sqlite_snapshot_native(SnapshotEncoding::Text, &mut control).is_err(),
            _ => unreachable!(),
        };
        drop(control);
        assert!(refused && saw, "phase {phase:?} failed to cancel real interior copy");
    }
}
#[test]
fn sqlite_snapshot_block5d_actual_declared_erased_binary_text_queries_every_domain_table() {
    use store::sqlite_snapshot::*;
    let codec = crate::standards::v1::subsets::any::io::io().native.codec.snapshot_sqlite.unwrap();
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.block.block5d".into(), standard: "1".into(), subset: "*".into() };
    for word in words() {
        let expected = full(word);
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
            let native = match encoding {
                SnapshotEncoding::Binary => store::io_schema::IoPayload::Binary(expected.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()),
                SnapshotEncoding::Text => store::io_schema::IoPayload::Text(expected.print_dsl()),
            };
            let database = (codec.export)(BLOCK_5D_SCHEMA, &dialect, &native, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
            assert_eq!(database.tables.len(), 15);
            assert_eq!(database.table("block5_document").unwrap().single_row().unwrap().text(1).unwrap(), "");
            assert_eq!(database.table("block5_grip").unwrap().single_row().unwrap().text(4).unwrap(), expected.grips[0].grip_kind);
            let native = (codec.import)(BLOCK_5D_SCHEMA, &dialect, database, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
            let actual = match native {
                store::io_schema::IoPayload::Binary(v) => Block5dSnapshot::decode_pack(&v).unwrap(),
                store::io_schema::IoPayload::Text(v) => Block5dSnapshot::parse_dsl(&v).unwrap(),
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
pub(crate) enum PublicApps: PluginApp {Editor(VcsArtifactApp<EditorApp<crate::editor::block5d::Block5dPlayApp>>),Viewer(VcsArtifactApp<ViewerApp<crate::viewer::block5d::Block5dViewer>>),}
}
struct PublicOwned(Option<Block5dSnapshot>);impl PublicOwned{fn new(value:Block5dSnapshot)->Self{Self(Some(value))}fn get(&self)->&Block5dSnapshot{self.0.as_ref().unwrap()}}impl Drop for PublicOwned{fn drop(&mut self){if let Some(value)=self.0.take(){<Block5dSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(value);}}}
fn exact(a:&Block5dSnapshot,b:&Block5dSnapshot){use semio_framework_value::{DslValue,Number,ToValue};fn same(a:&DslValue,b:&DslValue){match(a,b){(DslValue::Number(Number::Float(a)),DslValue::Number(Number::Float(b)))=>assert_eq!(a.to_bits(),b.to_bits()),(DslValue::Array(a),DslValue::Array(b))=>{assert_eq!(a.len(),b.len());for(a,b)in a.iter().zip(b){same(a,b);}},(DslValue::Object(a),DslValue::Object(b))=>{assert_eq!(a.len(),b.len());for((ka,a),(kb,b))in a.iter().zip(b){assert_eq!(ka,kb);same(a,b);}},_=>assert_eq!(a,b)}}same(&a.to_value(),&b.to_value());}

fn register(){semio_framework_plugin::Plugin::<PublicApps>::builder("block").label("Populated public SQLite").version("0.0.1").package_id("semio:block").declare_artifact(crate::artifact::<PublicApps>()).try_build().unwrap();}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_block5d_populated_actual_app_public_both_forms(){
use {semio_framework_artifact_reference::ArtifactDialect,store::io::io_mechanism::io_export_sqlite_snapshot,store::io::io_mechanism::io_import_sqlite_snapshot,store::io::io_mechanism::io_route};use store::io_schema::{IoFidelity,SQLITE_SNAPSHOT};
register();let dialect=ArtifactDialect{artifact_kind:"s.block.block5d".into(),standard:"1".into(),subset:"*".into()};let sqlite=ArtifactDialect::from(SQLITE_SNAPSHOT);for route in[io_route(&dialect,&sqlite,1).await.unwrap().value,io_route(&sqlite,&dialect,1).await.unwrap().value]{assert_eq!(route.hops.len(),1);assert_eq!(route.fidelity,IoFidelity::Exact);}
for word in words(){let expected=PublicOwned::new(full(word));for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let bytes=io_export_sqlite_snapshot(&dialect,expected.get(),encoding,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;let actual=PublicOwned::new(io_import_sqlite_snapshot::<Block5dSnapshot>(&dialect,&bytes,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value);exact(actual.get(),expected.get());assert_eq!((actual.get()).encode_pack(),(expected.get()).encode_pack());assert_eq!((actual.get()).print_dsl(),(expected.get()).print_dsl());}}
eprintln!("[DEBUG] block5d actual app public both forms preserve complete owner and words");
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_block5d_populated_public_independent_edit_retirement(){
use {semio_framework_artifact_reference::ArtifactDialect,store::io::io_mechanism::io_export_sqlite_snapshot,store::io::io_mechanism::io_import_sqlite_snapshot};use std::{io::Write,process::{Command,Stdio}};
register();let dialect=ArtifactDialect{artifact_kind:"s.block.block5d".into(),standard:"1".into(),subset:"*".into()};
for word in words(){let expected=PublicOwned::new(full(word));for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let bytes=io_export_sqlite_snapshot(&dialect,expected.get(),encoding,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;
let script=r#"import{Database}from'bun:sqlite';const d=Database.deserialize(await Bun.stdin.bytes(),{safeIntegers:true});if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');if(d.query('SELECT COUNT(*) AS n FROM sqlite_schema WHERE type=\'table\'').get().n!==16n)throw Error('table census');if(d.query('SELECT COUNT(*) AS n FROM block5_document').get().n!==1n)throw Error('literalRows:block5_document');if(d.query('SELECT COUNT(*) AS n FROM block5_kind').get().n!==1n)throw Error('literalRows:block5_kind');if(d.query('SELECT COUNT(*) AS n FROM block5_part2d').get().n!==1n)throw Error('literalRows:block5_part2d');if(d.query('SELECT COUNT(*) AS n FROM block5_part3d').get().n!==1n)throw Error('literalRows:block5_part3d');if(d.query('SELECT COUNT(*) AS n FROM block5_representation').get().n!==1n)throw Error('literalRows:block5_representation');if(d.query('SELECT COUNT(*) AS n FROM block5_representation_tag').get().n!==2n)throw Error('literalRows:block5_representation_tag');if(d.query('SELECT COUNT(*) AS n FROM block5_representation_attribute').get().n!==1n)throw Error('literalRows:block5_representation_attribute');if(d.query('SELECT COUNT(*) AS n FROM block5_grip_kind').get().n!==1n)throw Error('literalRows:block5_grip_kind');if(d.query('SELECT COUNT(*) AS n FROM block5_grip').get().n!==1n)throw Error('literalRows:block5_grip');if(d.query('SELECT COUNT(*) AS n FROM block5_compatibility').get().n!==1n)throw Error('literalRows:block5_compatibility');if(d.query('SELECT COUNT(*) AS n FROM block5_attribute').get().n!==1n)throw Error('literalRows:block5_attribute');if(d.query('SELECT COUNT(*) AS n FROM block5_author').get().n!==1n)throw Error('literalRows:block5_author');if(d.query('SELECT COUNT(*) AS n FROM block5_camera2d').get().n!==1n)throw Error('literalRows:block5_camera2d');if(d.query('SELECT COUNT(*) AS n FROM block5_camera3d').get().n!==1n)throw Error('literalRows:block5_camera3d');if(d.query('SELECT COUNT(*) AS n FROM block5_meta').get().n!==1n)throw Error('literalRows:block5_meta');const m=d.query('SELECT * FROM semio_snapshot').get();if(JSON.stringify(Object.keys(m))!==JSON.stringify(['id','artifact_kind','standard','subset','schema_version','native_encoding'])||m.id!==1n||m.schema_version!==1n)throw Error('completePublicMetadata');if(m.artifact_kind!=='s.block.block5d'||m.standard!=='1'||m.subset!=='*'||m.native_encoding!==process.argv[1])throw Error('metadata');d.query('UPDATE block5_kind SET description=? WHERE id=1').run('independent 日本\u0000');await Bun.write(Bun.stdout,d.serialize());d.close();"#;
let mut child=Command::new("bun").args(["--eval",script,encoding.as_str(),&(laws()).to_string()]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let out=child.wait_with_output().unwrap();assert!(out.status.success(),"{}",String::from_utf8_lossy(&out.stderr));let actual=PublicOwned::new(io_import_sqlite_snapshot::<Block5dSnapshot>(&dialect,&out.stdout,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value);let mut literal=full(word);literal.part_kind.description="independent 日本\0".into();let literal=PublicOwned::new(literal);exact(actual.get(),literal.get());assert_eq!((actual.get()).encode_pack(),(literal.get()).encode_pack());assert_eq!((actual.get()).print_dsl(),(literal.get()).print_dsl());}}
eprintln!("[DEBUG] block5d independent public SQL edit complete owner and retirement");
}
}
