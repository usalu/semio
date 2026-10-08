//! 🧱️ Genuine Block3d Native owned catalog address and scalar baselines.
use crate::standards::v1::subsets::any::io::sqlite::snapshot::Block3dSnapshot;
use crate::*;
use store::{ArtifactDsl, ArtifactPack};
fn laws() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()
}
fn words() -> Vec<u64> {
    laws()["binary64Bits"].as_array().unwrap().iter().map(|v| u64::from_str_radix(v.as_str().unwrap(), 16).unwrap()).collect()
}
fn full(word: u64) -> Block3dSnapshot {
    let f = f64::from_bits(word);
    let l = laws();
    let text = l["nativeCase"]["literal"].as_str().unwrap();
    Block3dSnapshot {
        schema: String::new(),
        object_kind: BlockKindIdentity { id: String::new(), name: text.into(), label: String::new(), variant: Some(String::new()), description: text.into(), icon: Some(String::new()), unit: Some(String::new()) },
        representations: vec![BlockRepresentation {
            id: String::new(),
            name: text.into(),
            mesh_url: Some(String::new()),
            tags: vec![String::new(), text.into()],
            lod: Some(String::new()),
            description: text.into(),
            attributes: vec![BlockAttribute { key: String::new(), value: text.into(), definition: Some(String::new()) }],
        }],
        catalog: store::ArtifactChild::new(
            "local\0!@/😀".into(),
            semio_framework_artifact_reference::ArtifactRef { artifact_id: "target\0!@/😀".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "kit".into() } },
        ),
        vortex_kind_extra: vec![Block3dVortexKindExtra { id: String::new(), name: text.into(), label: String::new(), color: String::new(), default_cable_kind: text.into() }],
        vortices: vec![Block3dVortexTemplate { id: String::new(), vortex_kind: l["nativeCase"]["unresolvedVortexKind"].as_str().unwrap().into(), position: [f; 3], direction: [f; 3], radius: f, label: Some(String::new()) }],
        compatibility: vec![BlockCompatibilityRule { id: String::new(), source: String::new(), target: text.into(), bidirectional: true }],
        attributes: vec![BlockAttribute { key: String::new(), value: text.into(), definition: Some(String::new()) }],
        authors: vec![BlockAuthor { id: String::new(), name: text.into(), email: Some(String::new()) }],
        camera3d: BlockCamera3d { position: [f; 3], target: [f; 3], zoom: f },
        meta: BlockMeta { description: text.into() },
    }
}
fn assert_words(s: &Block3dSnapshot, w: u64) {
    for f in s.vortices[0].position.into_iter().chain(s.vortices[0].direction).chain([s.vortices[0].radius]).chain(s.camera3d.position).chain(s.camera3d.target).chain([s.camera3d.zoom]) {
        assert_eq!(f.to_bits(), w);
    }
}
#[test]
fn sqlite_snapshot_block3d_actual_bare_capability_is_present() {
    assert!(<Block3dSnapshot as ArtifactPack>::sqlite_snapshot_codec().is_some());
}
#[test]
fn sqlite_snapshot_block3d_actual_declared_codec_has_owned_sqlite_capability() {
    let d = crate::standards::v1::subsets::any::io::io();
    assert!(d.native.codec.snapshot_sqlite.is_some());
}
#[test]
fn sqlite_snapshot_block3d_all_fields_and_words_survive_actual_text_and_pack() {
    for word in words() {
        let s = full(word);
        let text = s.print_dsl();
        let p = Block3dSnapshot::parse_dsl(&text).unwrap();
        assert_words(&p, word);
        assert_eq!(p.print_dsl(), text);
        let p = Block3dSnapshot::decode_pack(&s.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()).unwrap();
        assert_words(&p, word);
        assert_eq!(p.print_dsl(), text);
    }
}
#[test]
fn sqlite_snapshot_block3d_catalog_keeps_all_five_independent_literal_fields() {
    for address in laws()["catalogAddresses"].as_array().unwrap() {
        let mut s = full(0);
        s.catalog = store::ArtifactChild::new(
            address["childId"].as_str().unwrap().into(),
            semio_framework_artifact_reference::ArtifactRef {
                artifact_id: address["artifactId"].as_str().unwrap().into(),
                dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: address["artifactKind"].as_str().unwrap().into(), standard: address["standard"].as_str().unwrap().into(), subset: address["subset"].as_str().unwrap().into() },
            },
        );
        for p in [Block3dSnapshot::parse_dsl(&s.print_dsl()).unwrap(), Block3dSnapshot::decode_pack(&s.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()).unwrap()] {
            assert_eq!(p.catalog, s.catalog);
        }
    }
}
#[test]
fn sqlite_snapshot_block3d_optional_absence_remains_distinct_from_present_empty() {
    let mut s = full(0);
    s.object_kind.variant = None;
    s.object_kind.icon = None;
    s.object_kind.unit = None;
    s.representations[0].mesh_url = None;
    s.representations[0].lod = None;
    s.representations[0].attributes[0].definition = None;
    s.vortices[0].label = None;
    s.attributes[0].definition = None;
    s.authors[0].email = None;
    let text = s.print_dsl();
    for p in [Block3dSnapshot::parse_dsl(&text).unwrap(), Block3dSnapshot::decode_pack(&s.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()).unwrap()] {
        assert_eq!(p.print_dsl(), text);
        assert_eq!(p.vortices[0].label, None);
        assert_eq!(p.representations[0].mesh_url, None);
    }
}
#[test]
fn sqlite_snapshot_block3d_controlled_record_constructs_every_owned_field() {
    for word in words() {
        let s = full(word);
        let mut accepted = |_| true;
        let mut c = semio_framework_value::NativeEncodeControl::new(1 << 20, &mut accepted);
        let spec = Block3dSnapshot::__dsl_spec_producer();
        (spec.encoding)(&mut c).unwrap();
        let record = s.__dsl_to_record_controlled(&mut c).unwrap();
        let mut accepted = |_| true;
        let mut c = semio_framework_value::NativeDecodeControl::new(1 << 20, &mut accepted);
        (spec.decoding)(&mut c).unwrap();
        let p = Block3dSnapshot::__dsl_from_record_controlled(&record, &mut c).unwrap();
        assert_words(&p, word);
        assert_eq!(p.catalog, s.catalog);
        assert_eq!(p.print_dsl(), s.print_dsl());
    }
}

#[test]
fn sqlite_snapshot_block3d_actual_declared_json_preserves_every_owned_word() {
    for word in words() {
        let s = full(word);
        let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any::json_text(&s);
        let p = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::from_json_text(&bytes).unwrap();
        assert_words(&p, word);
        assert_eq!(p.print_dsl(), s.print_dsl());
    }
}

#[test]
fn sqlite_snapshot_block3d_independent_sqlite_reads_the_literal_table_and_word_contract() {
    use std::process::Command;
    let sql = include_str!("../🗄️.sql");
    let script = r#"import{Database}from'bun:sqlite';const d=new Database(':memory:',{safeIntegers:true});d.exec(process.argv[1]);const expected=JSON.parse(process.argv[2]);const tables=d.query('SELECT name FROM sqlite_schema WHERE type=\'table\'').all();if(tables.length!==Object.keys(expected).length)throw Error('tablecount');for(const{name}of tables){if(d.query('PRAGMA table_info('+name+')').all().length!==expected[name])throw Error('width '+name)}d.exec('CREATE TABLE independent_words(ordinal INTEGER PRIMARY KEY,bits INTEGER NOT NULL)');const words=JSON.parse(process.argv[3]);const insert=d.query('INSERT INTO independent_words VALUES(?,?)');for(let i=0;i<words.length;i++)insert.run(i,BigInt.asIntN(64,BigInt('0x'+words[i])));const rows=d.query('SELECT bits FROM independent_words ORDER BY ordinal').all();for(let i=0;i<rows.length;i++)if(BigInt.asUintN(64,rows[i].bits).toString(16).padStart(16,'0')!==words[i])throw Error('word');d.close();"#;
    let widths = r#"{"block3_document":2,"block3_kind":9,"block3_catalog_child":7,"block3_representation":8,"block3_representation_tag":4,"block3_representation_attribute":6,"block3_vortex_kind_extra":8,"block3_vortex":27,"block3_compatibility":7,"block3_attribute":6,"block3_author":6,"block3_camera3d":23,"block3_meta":3}"#;
    let output = Command::new("bun").args(["-e", script, sql, widths, &laws()["binary64Bits"].to_string()]).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}

fn independent_edit(bytes: &[u8], sql: &str) -> Vec<u8> {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let script = r#"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});try{if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');if(db.query('SELECT name FROM sqlite_schema WHERE type=\'table\'').all().length!==13)throw Error('tables');db.exec(process.argv[1]);process.stdout.write(db.serialize())}finally{db.close()}"#;
    let mut child = Command::new("bun").args(["-e", script, sql]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    output.stdout
}
#[test]
fn sqlite_snapshot_block3d_independent_edited_physical_database_preserves_every_word() {
    use store::{sqlite_snapshot::*, ArtifactSqliteSnapshot};
    for word in words() {
        let expected = full(word);
        let database = expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        assert_eq!(database.tables.len(), 13);
        assert_eq!(database.tables.iter().map(|t| t.rows.len()).sum::<usize>(), 14);
        let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
        let edited = import_sqlite_database(
            &independent_edit(&bytes, "UPDATE block3_meta SET description='independent';UPDATE block3_vortex SET vortex_kind='unresolved edited';UPDATE block3_catalog_child SET child_id='independent local',target_artifact_id='independent target'"),
            SqliteDatabaseLimits::default(),
            &mut |_| true,
        )
        .unwrap();
        let actual = Block3dSnapshot::from_sqlite_database(&edited, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        assert_words(&actual, word);
        assert_eq!(actual.meta.description, "independent");
        assert_eq!(actual.vortices[0].vortex_kind, "unresolved edited");
        let mut expected = expected;
        expected.meta.description = "independent".into();
        expected.vortices[0].vortex_kind = "unresolved edited".into();
        expected.catalog.child_id = "independent local".into();
        expected.catalog.target.artifact_id = "independent target".into();
        assert_eq!(actual.print_dsl(), expected.print_dsl());
    }
}
#[test]
fn sqlite_snapshot_block3d_independent_malformed_entities_refuse() {
    use store::{sqlite_snapshot::*, ArtifactSqliteSnapshot};
    let database = full(0).to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    for edit in [
        "UPDATE block3_vortex SET ordinal=2",
        "UPDATE block3_camera3d SET zoom_ieee754_bits=1",
        "UPDATE block3_camera3d SET zoom_numeric_class='nan'",
        "DELETE FROM block3_kind",
        "UPDATE block3_compatibility SET bidirectional=3",
        "INSERT INTO block3_meta VALUES(99,1,'duplicate')",
        "UPDATE block3_representation_tag SET representation_id=999",
        "UPDATE block3_catalog_child SET subset='graph'",
    ] {
        let malformed = import_sqlite_database(&independent_edit(&bytes, edit), SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
        assert!(Block3dSnapshot::from_sqlite_database(&malformed, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err(), "{edit}");
    }
}
#[test]
fn sqlite_snapshot_block3d_row_frontier_and_real_four_phase_cancellation() {
    use store::{sqlite_snapshot::*, ArtifactSqliteSnapshot};
    let mut expected = full(0);
    expected.object_kind.description = "😀".repeat(32768);
    let limits = SqliteDatabaseLimits { max_rows: 14, ..SqliteDatabaseLimits::default() };
    let database = expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    let short = SqliteDatabaseLimits { max_rows: 13, ..limits };
    assert!(expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, short)).is_err());
    let native = store::io_schema::IoPayload::Text(expected.print_dsl());
    assert!(Block3dSnapshot::decode_sqlite_snapshot_native(&native, &mut SqliteSnapshotControl::new(&mut |_| true, short)).is_err());
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
            SqliteSnapshotPhase::ReconstructSnapshot => Block3dSnapshot::from_sqlite_database(&database, &mut control).is_err(),
            SqliteSnapshotPhase::DecodeNative => Block3dSnapshot::decode_sqlite_snapshot_native(&native, &mut control).is_err(),
            SqliteSnapshotPhase::EncodeNative => expected.encode_sqlite_snapshot_native(SnapshotEncoding::Text, &mut control).is_err(),
            _ => unreachable!(),
        };
        drop(control);
        assert!(refused && saw, "{phase:?}");
    }
}
#[test]
fn sqlite_snapshot_block3d_actual_erased_declared_binary_text_retains_full_domain() {
    use store::sqlite_snapshot::*;
    let codec = crate::standards::v1::subsets::any::io::io().native.codec.snapshot_sqlite.unwrap();
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.block.block3d".into(), standard: "1".into(), subset: "*".into() };
    for word in words() {
        let expected = full(word);
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
            let native = match encoding {
                SnapshotEncoding::Binary => store::io_schema::IoPayload::Binary(expected.encode_pack()),
                SnapshotEncoding::Text => store::io_schema::IoPayload::Text(expected.print_dsl()),
            };
            let database = (codec.export)(BLOCK_3D_SCHEMA, &dialect, &native, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
            assert_eq!(database.tables.len(), 13);
            assert_eq!(database.table("block3_document").unwrap().single_row().unwrap().text(1).unwrap(), "");
            let child = database.table("block3_catalog_child").unwrap().single_row().unwrap();
            assert_eq!(child.text(2).unwrap(), expected.catalog.child_id);
            assert_eq!(child.text(3).unwrap(), expected.catalog.target.artifact_id);
            let native = (codec.import)(BLOCK_3D_SCHEMA, &dialect, database, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
            let actual = match native {
                store::io_schema::IoPayload::Binary(v) => Block3dSnapshot::decode_pack(&v).unwrap(),
                store::io_schema::IoPayload::Text(v) => Block3dSnapshot::parse_dsl(&v).unwrap(),
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
pub(crate) enum PublicApps: PluginApp {Editor(VcsArtifactApp<EditorApp<crate::editor::block3d::Block3dPlayApp>>),Viewer(VcsArtifactApp<ViewerApp<crate::viewer::block3d::Block3dViewer>>),}
}
struct PublicOwned(Option<Block3dSnapshot>);impl PublicOwned{fn new(value:Block3dSnapshot)->Self{Self(Some(value))}fn get(&self)->&Block3dSnapshot{self.0.as_ref().unwrap()}}impl Drop for PublicOwned{fn drop(&mut self){if let Some(value)=self.0.take(){<Block3dSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(value);}}}
fn exact(a:&Block3dSnapshot,b:&Block3dSnapshot){use semio_framework_value::{DslValue,Number,ToValue};fn same(a:&DslValue,b:&DslValue){match(a,b){(DslValue::Number(Number::Float(a)),DslValue::Number(Number::Float(b)))=>assert_eq!(a.to_bits(),b.to_bits()),(DslValue::Array(a),DslValue::Array(b))=>{assert_eq!(a.len(),b.len());for(a,b)in a.iter().zip(b){same(a,b);}},(DslValue::Object(a),DslValue::Object(b))=>{assert_eq!(a.len(),b.len());for((ka,a),(kb,b))in a.iter().zip(b){assert_eq!(ka,kb);same(a,b);}},_=>assert_eq!(a,b)}}same(&a.to_value(),&b.to_value());}

fn register(){semio_framework_plugin::Plugin::<PublicApps>::builder("block").label("Populated public SQLite").version("0.0.1").package_id("semio:block").declare_artifact(crate::artifact::<PublicApps>()).try_build().unwrap();}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_block3d_populated_actual_app_public_both_forms(){
use {semio_framework_artifact_reference::ArtifactDialect,store::io::io_mechanism::io_export_sqlite_snapshot,store::io::io_mechanism::io_import_sqlite_snapshot,store::io::io_mechanism::io_route};use store::io_schema::{IoFidelity,SQLITE_SNAPSHOT};
register();let dialect=ArtifactDialect{artifact_kind:"s.block.block3d".into(),standard:"1".into(),subset:"*".into()};let sqlite=ArtifactDialect::from(SQLITE_SNAPSHOT);for route in[io_route(&dialect,&sqlite,1).await.unwrap().value,io_route(&sqlite,&dialect,1).await.unwrap().value]{assert_eq!(route.hops.len(),1);assert_eq!(route.fidelity,IoFidelity::Exact);}
for word in words(){let expected=PublicOwned::new(full(word));for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let bytes=io_export_sqlite_snapshot(&dialect,expected.get(),encoding,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;let actual=PublicOwned::new(io_import_sqlite_snapshot::<Block3dSnapshot>(&dialect,&bytes,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value);exact(actual.get(),expected.get());assert_eq!((actual.get()).encode_pack(),(expected.get()).encode_pack());assert_eq!((actual.get()).print_dsl(),(expected.get()).print_dsl());}}
eprintln!("[DEBUG] block3d actual app public both forms preserve complete owner and words");
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_block3d_populated_public_independent_edit_retirement(){
use {semio_framework_artifact_reference::ArtifactDialect,store::io::io_mechanism::io_export_sqlite_snapshot,store::io::io_mechanism::io_import_sqlite_snapshot};use std::{io::Write,process::{Command,Stdio}};
register();let dialect=ArtifactDialect{artifact_kind:"s.block.block3d".into(),standard:"1".into(),subset:"*".into()};
for word in words(){let expected=PublicOwned::new(full(word));for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let bytes=io_export_sqlite_snapshot(&dialect,expected.get(),encoding,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;
let script=r#"import{Database}from'bun:sqlite';const d=Database.deserialize(await Bun.stdin.bytes(),{safeIntegers:true});if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');if(d.query('SELECT COUNT(*) AS n FROM sqlite_schema WHERE type=\'table\'').get().n!==14n)throw Error('table census');if(d.query('SELECT COUNT(*) AS n FROM block3_document').get().n!==1n)throw Error('literalRows:block3_document');if(d.query('SELECT COUNT(*) AS n FROM block3_kind').get().n!==1n)throw Error('literalRows:block3_kind');if(d.query('SELECT COUNT(*) AS n FROM block3_catalog_child').get().n!==1n)throw Error('literalRows:block3_catalog_child');if(d.query('SELECT COUNT(*) AS n FROM block3_representation').get().n!==1n)throw Error('literalRows:block3_representation');if(d.query('SELECT COUNT(*) AS n FROM block3_representation_tag').get().n!==2n)throw Error('literalRows:block3_representation_tag');if(d.query('SELECT COUNT(*) AS n FROM block3_representation_attribute').get().n!==1n)throw Error('literalRows:block3_representation_attribute');if(d.query('SELECT COUNT(*) AS n FROM block3_vortex_kind_extra').get().n!==1n)throw Error('literalRows:block3_vortex_kind_extra');if(d.query('SELECT COUNT(*) AS n FROM block3_vortex').get().n!==1n)throw Error('literalRows:block3_vortex');if(d.query('SELECT COUNT(*) AS n FROM block3_compatibility').get().n!==1n)throw Error('literalRows:block3_compatibility');if(d.query('SELECT COUNT(*) AS n FROM block3_attribute').get().n!==1n)throw Error('literalRows:block3_attribute');if(d.query('SELECT COUNT(*) AS n FROM block3_author').get().n!==1n)throw Error('literalRows:block3_author');if(d.query('SELECT COUNT(*) AS n FROM block3_camera3d').get().n!==1n)throw Error('literalRows:block3_camera3d');if(d.query('SELECT COUNT(*) AS n FROM block3_meta').get().n!==1n)throw Error('literalRows:block3_meta');const m=d.query('SELECT * FROM semio_snapshot').get();if(JSON.stringify(Object.keys(m))!==JSON.stringify(['id','artifact_kind','standard','subset','schema_version','native_encoding'])||m.id!==1n||m.schema_version!==1n)throw Error('completePublicMetadata');if(m.artifact_kind!=='s.block.block3d'||m.standard!=='1'||m.subset!=='*'||m.native_encoding!==process.argv[1])throw Error('metadata');d.query('UPDATE block3_kind SET description=? WHERE id=1').run('independent 日本\u0000');await Bun.write(Bun.stdout,d.serialize());d.close();"#;
let mut child=Command::new("bun").args(["--eval",script,encoding.as_str(),&(laws()).to_string()]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let out=child.wait_with_output().unwrap();assert!(out.status.success(),"{}",String::from_utf8_lossy(&out.stderr));let actual=PublicOwned::new(io_import_sqlite_snapshot::<Block3dSnapshot>(&dialect,&out.stdout,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value);let mut literal=full(word);literal.object_kind.description="independent 日本\0".into();let literal=PublicOwned::new(literal);exact(actual.get(),literal.get());assert_eq!((actual.get()).encode_pack(),(literal.get()).encode_pack());assert_eq!((actual.get()).print_dsl(),(literal.get()).print_dsl());}}
eprintln!("[DEBUG] block3d independent public SQL edit complete owner and retirement");
}
}

#[test]
fn sqlite_snapshot_block3d_json_transport_corpus_is_closed_and_third_party_measured() {
    use std::{io::Write,process::{Command,Stdio}};
    let corpus=include_str!("../🧫️fixtures/🔢️transport/🔣️.json");
    let script=r#"import Ajv from'ajv';const c=JSON.parse(process.argv[1]),word=process.argv[2],v=JSON.parse(await Bun.stdin.text());const a=new Ajv({strict:true});const valid=a.compile(JSON.parse(process.argv[3]).$defs.Binary64);for(const path of c.roles){const cell=path.reduce((v,k)=>v[k],v);if(!valid(cell)||cell.bits!==word)throw Error('literal IEEE role');const bytes=Buffer.from(cell.bits,'hex');if(new DataView(bytes.buffer,bytes.byteOffset,8).getBigUint64(0)!==BigInt('0x'+word))throw Error('independent bytes');}for(const invalid of c.invalidWords)if(valid(invalid))throw Error('invalid word');"#;
    for word in words(){let snapshot=full(word);let text=crate::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any::json_text(&snapshot);let mut child=Command::new("bun").args(["-e",script,corpus,&format!("{word:016x}"),include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🔣️.json")]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(text.as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let restored=crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::from_json_text(&text).unwrap();assert_words(&restored,word);
    let fixture:serde_json::Value=serde_json::from_str(corpus).unwrap();for bad in fixture["invalidWords"].as_array().unwrap(){let mut value:serde_json::Value=serde_json::from_str(&text).unwrap();value["vortices"][0]["position"][0]=bad.clone();assert!(crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::from_json_text(&value.to_string()).is_err());}}
}
#[test]
fn sqlite_snapshot_block3d_typed_catalog_dialect_is_required_by_every_native_form() {
 use store::{ArtifactSqliteSnapshot,sqlite_snapshot::*};
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔢️transport/🔣️.json")).unwrap();
 for row in corpus["childDialects"]["refused"].as_array().unwrap(){let mut s=full(0);s.catalog.target.dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:row["artifactKind"].as_str().unwrap().into(),standard:row["standard"].as_str().unwrap().into(),subset:row["subset"].as_str().unwrap().into()};assert!(s.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());for encoding in[SnapshotEncoding::Text,SnapshotEncoding::Binary]{let payload=match encoding{SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(s.print_dsl()),SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(s.encode_pack_with(&Default::default()).unwrap())};assert!(Block3dSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());assert!(s.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());}}
}

#[test]
fn sqlite_snapshot_block3d_complete_native_limits_follow_independent_all_cell_corpus(){
 use store::{ArtifactSqliteSnapshot,sqlite_snapshot::*};use std::{io::Write,process::{Command,Stdio}};
 let corpus=include_str!("../🧫️fixtures/🛂️semantic/🔣️.json");let plan:serde_json::Value=serde_json::from_str(corpus).unwrap();
 let oracle=r#"import{Database}from'bun:sqlite';const p=JSON.parse(process.argv[1]),sample=JSON.parse(process.argv[2]);const db=Database.deserialize(await Bun.stdin.bytes());try{if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('SQL integrity');let rows=0,bytes=0;for(const[table,width]of Object.entries(p.tableWidths)){const fields=db.query('PRAGMA table_info('+table+')').all().map(v=>v.name);if(fields.length!==width)throw Error('columns');const cells=fields.map(name=>"CASE typeof("+name+") WHEN 'integer' THEN 8 WHEN 'real' THEN 8 WHEN 'text' THEN length(CAST("+name+" AS BLOB)) WHEN 'blob' THEN length("+name+") ELSE 0 END").join('+');const extent=db.query('SELECT COUNT(*) AS rows,COALESCE(SUM('+cells+'),0) AS bytes FROM '+table).get();rows+=extent.rows;bytes+=extent.bytes;}if(rows!==sample.rows||bytes!==sample.bytes)throw Error('all cells '+JSON.stringify({rows,bytes,sample}));}finally{db.close()}"#;
 for sample in plan["cases"].as_array().unwrap(){let word=u64::from_str_radix(sample["word"].as_str().unwrap(),16).unwrap();let snapshot=full(word);let defaults=SqliteDatabaseLimits::default();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,defaults)).unwrap();let file=export_sqlite_database(&database,defaults,&mut |_|true).unwrap();let mut child=Command::new("bun").args(["-e",oracle,corpus,&sample.to_string()]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&file).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
 let rows=sample["rows"].as_u64().unwrap()as usize;let bytes=sample["bytes"].as_u64().unwrap()as usize;
 for encoding in[SnapshotEncoding::Text,SnapshotEncoding::Binary]{let input=match encoding{SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(snapshot.print_dsl()),SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(snapshot.encode_pack_with(&Default::default()).unwrap())};let retained=Block3dSnapshot::decode_sqlite_snapshot_native(&input,&mut SqliteSnapshotControl::new(&mut |_|true,defaults)).unwrap();assert_words(&retained,word);retained.retire_sqlite_snapshot();
 for(field,max)in[("columns",27),("tables",13),("schema",Block3dSnapshot::SQLITE_SCHEMA.len()),("rows",rows),("bytes",bytes)]{let limits=match field{"columns"=>SqliteDatabaseLimits{max_columns:max-1,..defaults},"tables"=>SqliteDatabaseLimits{max_tables:max-1,..defaults},"schema"=>SqliteDatabaseLimits{max_schema_bytes:max-1,..defaults},"rows"=>SqliteDatabaseLimits{max_rows:max-1,..defaults},"bytes"=>SqliteDatabaseLimits{max_value_bytes:max-1,..defaults},_=>unreachable!()};assert!(Block3dSnapshot::decode_sqlite_snapshot_native(&input,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"{word:016x} direct native {field}");for success in[false,true]{let limits=if success{match field{"columns"=>SqliteDatabaseLimits{max_columns:max,..defaults},"tables"=>SqliteDatabaseLimits{max_tables:max,..defaults},"schema"=>SqliteDatabaseLimits{max_schema_bytes:max,..defaults},"rows"=>SqliteDatabaseLimits{max_rows:max,..defaults},"bytes"=>SqliteDatabaseLimits{max_value_bytes:max,..defaults},_=>unreachable!()}}else{limits};assert_eq!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_ok(),success,"{word:016x} typed output {field}");assert_eq!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_ok(),success,"{word:016x} borrowed preflight {field}");}}
 }snapshot.retire_sqlite_snapshot();}
}

#[path="../📏️preflight/🫳️borrowed/🦀️.rs"]mod exact_borrowed_wire;
#[test]
fn sqlite_snapshot_block3d_actual_borrowed_wire_matches_all_words_and_owned_child_fields(){
 use semio_framework_value::NativeEncodeControl;use store::sqlite_snapshot::*;
 for word in words(){let source=full(word);for encoding in[SnapshotEncoding::Text,SnapshotEncoding::Binary]{
 let length=match encoding{SnapshotEncoding::Text=>source.print_dsl().len(),SnapshotEncoding::Binary=>source.encode_pack_with(&Default::default()).unwrap().len()};
 let component=match encoding{SnapshotEncoding::Text=>store::semio_format::Component::Dsl,SnapshotEncoding::Binary=>store::semio_format::Component::Pack};
 let prefix=store::semio_format::declared_envelope_prefix_len("block.block3d",component,1).unwrap();let body=length-prefix;
 for maximum in[body,body-1]{let mut progress=|_|true;let mut native=NativeEncodeControl::new(SqliteDatabaseLimits::default().max_allocation_bytes,&mut progress);
 let result=match encoding{SnapshotEncoding::Text=>semio_framework_dsl_record::measure_print_borrowed(&source,&exact_borrowed_wire::spec(),maximum,&mut native),SnapshotEncoding::Binary=>{let mut options=store::os_pack::record::EncodeOptions::default();options.limits.max_file_len=maximum as u64;store::os_pack::record::measure_document_borrowed(&source,&exact_borrowed_wire::spec(),&options,&mut native)}};
 if maximum==body{assert_eq!(result.expect("actual retained child and target must be borrowed without Record mirrors"),body);use semio_framework_dsl_record::{DslField,native_encoding::FieldProjectionView as V};for(path,expected)in[(&[0][..],source.catalog.child_id.as_str()),(&[1,0][..],source.catalog.target.artifact_id.as_str()),(&[1,1][..],source.catalog.target.dialect.artifact_kind.as_str()),(&[1,2][..],source.catalog.target.dialect.standard.as_str()),(&[1,3][..],source.catalog.target.dialect.subset.as_str())]{let V::Text(actual)=DslField::projection_view(&source.catalog,path).unwrap()else{panic!("literal borrowed child field")};assert_eq!(actual.as_ptr(),expected.as_ptr());assert_eq!(actual.len(),expected.len());}assert!(DslField::projection_view(&source.catalog,&[1,4]).is_err());assert!(DslField::projection_view(&source.catalog,&[0,0]).is_err());assert!(DslField::projection_key(&source.catalog,&[],0).is_err())}else{assert!(result.is_err(),"one actual wire octet short");}
 }
 for maximum in[length,length-1]{let limits=SqliteDatabaseLimits{max_file_bytes:maximum,..Default::default()};assert_eq!(<Block3dSnapshot as store::ArtifactSqliteSnapshot>::preflight_sqlite_snapshot_encoding(&source,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_ok(),maximum==length,"actual envelope and borrowed wire file ceiling");}
 }
 <Block3dSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(source);
 }
}

#[test]
fn sqlite_snapshot_block3d_complete_borrowed_semantic_gate_copied_limits_exact_and_one_short(){
 use store::{ArtifactSqliteSnapshot,sqlite_snapshot::*};
 let plan:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛂️semantic/🔣️.json")).unwrap();
 let defaults=SqliteDatabaseLimits::default();
 for sample in plan["cases"].as_array().unwrap(){let word=u64::from_str_radix(sample["word"].as_str().unwrap(),16).unwrap();let s=full(word);let rows=sample["rows"].as_u64().unwrap()as usize;let bytes=sample["bytes"].as_u64().unwrap()as usize;
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let input=match encoding{SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(s.encode_pack_with(&Default::default()).unwrap()),SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(s.print_dsl())};
 for(field,max)in[("columns",27),("tables",13),("schema",Block3dSnapshot::SQLITE_SCHEMA.len()),("rows",rows),("bytes",bytes)]{for success in[true,false]{let maximum=max-usize::from(!success);let limits=match field{"columns"=>SqliteDatabaseLimits{max_columns:maximum,..defaults},"tables"=>SqliteDatabaseLimits{max_tables:maximum,..defaults},"schema"=>SqliteDatabaseLimits{max_schema_bytes:maximum,..defaults},"rows"=>SqliteDatabaseLimits{max_rows:maximum,..defaults},"bytes"=>SqliteDatabaseLimits{max_value_bytes:maximum,..defaults},_=>unreachable!()};let result=store::decode_sqlite_snapshot_record_native(&input,<Block3dSnapshot as ArtifactDsl>::envelope_id(),Block3dSnapshot::__dsl_spec_producer(),|record,native|crate::standards::v1::subsets::any::io::sqlite::snapshot::admission::admit(record,native,limits),&mut SqliteSnapshotControl::new(&mut |_|true,defaults));assert_eq!(result.is_ok(),success,"{word:016x} isolated copied {field} semantic limit before typed binding");}}
 }s.retire_sqlite_snapshot();}
}
