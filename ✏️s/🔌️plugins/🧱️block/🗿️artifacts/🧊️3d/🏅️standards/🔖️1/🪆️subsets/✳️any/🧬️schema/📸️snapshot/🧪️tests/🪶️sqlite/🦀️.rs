//! 🧱️ Genuine Block3d Native owned catalog address and scalar baselines.
use super::Block3dSnapshot;
use crate::*;
use store::{ArtifactDsl, ArtifactPack};
fn laws() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()
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
            store::os_io::ArtifactRef { artifact_id: "target\0!@/😀".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "kit".into() } },
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
            store::os_io::ArtifactRef {
                artifact_id: address["artifactId"].as_str().unwrap().into(),
                dialect: store::os_io::ArtifactDialect { artifact_kind: address["artifactKind"].as_str().unwrap().into(), standard: address["standard"].as_str().unwrap().into(), subset: address["subset"].as_str().unwrap().into() },
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
    let sql = include_str!("../../🪶️sqlite/🗄️.sql");
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
            &independent_edit(&bytes, "UPDATE block3_meta SET description='independent';UPDATE block3_vortex SET vortex_kind='unresolved edited';UPDATE block3_catalog_child SET child_id='independent local',artifact_id='independent target'"),
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
    let dialect = store::io_schema::ArtifactDialect { artifact_kind: "s.block.block3d".into(), standard: "1".into(), subset: "*".into() };
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
