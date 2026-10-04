//! 🧊️ Exact Native capability and controlled Scale laws precede semantic owner mounting.
use super::Puzzle3dSnapshot;
use crate::*;
use store::{ArtifactDsl, ArtifactPack};

fn words() -> Vec<u64> {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    corpus["binary64Bits"].as_array().unwrap().iter().map(|v| u64::from_str_radix(v.as_str().unwrap(), 16).unwrap()).collect()
}
fn specimen(word: u64) -> Puzzle3dSnapshot {
    let f = f64::from_bits(word);
    let mut s = Puzzle3dSnapshot::default();
    s.schema = String::new();
    s.domain = "literal\0😀".into();
    s.objects.push(Puzzle3dObject {
        id: String::new(),
        label: Some(String::new()),
        object_kind: Some("unresolved".into()),
        anchor: Puzzle3dObjectAnchor::Derived,
        origin: [f; 3],
        orientation: Some([f; 4]),
        scale: Some(Puzzle3dScale::Uniform(f)),
        mesh_url: Some(String::new()),
        vortices: vec![Puzzle3dVortex { id: String::new(), vortex_kind: None, label: Some(String::new()), position: [f; 3], direction: Some([f; 3]), radius: Some(f), hidden: false, locked: true }],
        hidden: false,
        locked: true,
    });
    s.target_volumes.push(Puzzle3dTargetVolume { id: String::new(), origin: [f; 3], orientation: Some([f; 4]), scale: Some(Puzzle3dScale::Vec3([f; 3])), hidden: true, locked: false });
    s.references.push(Puzzle3dReference { id: String::new(), source: Puzzle3dReferenceSource { url: "literal\0!@/😀".into(), media_kind: Some(String::new()) }, origin: [f; 3], width_world: f, hidden: false, locked: true });
    s.attractions.push(Puzzle3dAttraction { id: String::new(), attracting: "unresolved\0😀".into(), attracted: String::new(), gap: f, shift: f, rise: f, rotation: f, turn: f, tilt: f, x: f, y: f });
    s.meta.kind_compatibility = [Puzzle3dCompatSpecificity::General, Puzzle3dCompatSpecificity::Object, Puzzle3dCompatSpecificity::Attraction, Puzzle3dCompatSpecificity::Cable, Puzzle3dCompatSpecificity::Vortex]
        .into_iter()
        .map(|specificity| Puzzle3dKindCompatibility { source: String::new(), target: "unresolved".into(), bidirectional: false, important: true, specificity })
        .collect();
    s.meta.kind_catalogs = Some(Puzzle3dKindCatalogs {
        objects: vec![Puzzle3dCatalogObjectKind {
            id: String::new(),
            name: "name\0😀".into(),
            label: String::new(),
            description: "description".into(),
            icon: String::new(),
            image: String::new(),
            unit: String::new(),
            is_abstract: true,
            base_kinds: vec![String::new(), "unresolved".into()],
            representations: vec![Puzzle3dRepresentation { id: String::new(), name: String::new(), url: String::new(), mime: String::new(), tags: vec![String::new(), String::new()], lod: Some(String::new()), description: String::new() }],
            vortices: vec![Puzzle3dCatalogVortexTemplate {
                id: String::new(),
                name: String::new(),
                label: String::new(),
                description: String::new(),
                icon: String::new(),
                vortex_kind: Some(String::new()),
                point: [f; 3],
                direction: [f; 3],
                t: Some(f),
                mandatory: Some(false),
                radius: Some(f),
            }],
            attributes: vec![Puzzle3dAttribute { id: String::new(), key: String::new(), value: "value\0😀".into(), definition: Some(String::new()) }],
            authors: vec![Puzzle3dAuthor { id: String::new(), name: String::new(), email: String::new(), role: Some(String::new()), rank: Some(i32::MIN) }],
        }],
        vortices: vec![Puzzle3dCatalogVortexKind {
            id: String::new(),
            code: Some(String::new()),
            label: Some(String::new()),
            order: Some(i32::MAX),
            compatible_with: vec![String::new(), "unresolved".into()],
            description: String::new(),
            icon: String::new(),
            color: String::new(),
            default_cable_kind: String::new(),
        }],
        cables: vec![Puzzle3dCatalogCableKind { id: String::new(), label: String::new(), name: String::new(), default_attraction_kind: String::new() }],
        attractions: vec![Puzzle3dCatalogAttractionKind { id: String::new(), label: String::new(), name: String::new() }],
    });
    s
}

#[test]
fn sqlite_snapshot_puzzle3d_actual_typed_bare_capability_is_present() {
    assert!(<Puzzle3dSnapshot as ArtifactPack>::sqlite_snapshot_codec().is_some());
}
#[test]
fn sqlite_snapshot_puzzle3d_actual_play_bare_capability_is_present() {
    assert!(<Puzzle3dPlaySnapshot as ArtifactPack>::sqlite_snapshot_codec().is_some());
}
#[test]
fn sqlite_snapshot_puzzle3d_native_typed_text_and_pack_preserve_every_word() {
    for word in words() {
        let s = specimen(word);
        let text = s.print_dsl();
        assert_eq!(Puzzle3dSnapshot::parse_dsl(&text).unwrap().print_dsl(), text);
        let pack = s.encode_pack_with(&store::PackEncodeOptions::default()).unwrap();
        assert_eq!(Puzzle3dSnapshot::decode_pack(&pack).unwrap().print_dsl(), text);
    }
}
#[test]
fn sqlite_snapshot_puzzle3d_play_text_preserves_the_actual_typed_snapshot() {
    for word in words() {
        let s = specimen(word);
        let pack = s.encode_pack_with(&store::PackEncodeOptions::default()).unwrap();
        let p = Puzzle3dPlaySnapshot::decode_pack(&pack).unwrap();
        let text = p.print_dsl();
        let out = Puzzle3dPlaySnapshot::parse_dsl(&text).unwrap();
        assert_eq!(out.typed().print_dsl(), s.print_dsl());
    }
}
#[test]
fn sqlite_snapshot_puzzle3d_declared_json_preserves_every_word() {
    for word in words() {
        let s = specimen(word);
        let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&s).unwrap();
        let out = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).unwrap();
        assert_eq!(out.print_dsl(), s.print_dsl());
    }
}
#[test]
fn sqlite_snapshot_puzzle3d_controlled_scale_has_real_metadata_and_both_variants() {
    for word in words() {
        let f = f64::from_bits(word);
        for s in [Puzzle3dScale::Uniform(f), Puzzle3dScale::Vec3([f; 3])] {
            let mut callback = |_| true;
            let mut output = semio_framework_value::NativeEncodeControl::new(4096, &mut callback);
            <Puzzle3dScale as semio_framework_dsl_record::DslField>::shape_controlled(&mut output).unwrap();
            let value = <Puzzle3dScale as semio_framework_dsl_record::DslField>::to_value_controlled(&s, &mut output).unwrap();
            let mut callback = |_| true;
            let mut input = semio_framework_value::NativeDecodeControl::new(4096, &mut callback);
            let out = <Puzzle3dScale as semio_framework_dsl_record::DslField>::from_value_controlled(&value, &mut input).unwrap();
            match (s, out) {
                (Puzzle3dScale::Uniform(a), Puzzle3dScale::Uniform(b)) => assert_eq!(a.to_bits(), b.to_bits()),
                (Puzzle3dScale::Vec3(a), Puzzle3dScale::Vec3(b)) => assert_eq!(a.map(f64::to_bits), b.map(f64::to_bits)),
                _ => panic!("Scale variant changed"),
            }
        }
    }
}

fn independent_puzzle3_edit(bytes: &[u8], edit: &str) -> Vec<u8> {
    use std::io::Write;
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    let script = r#"import{Database}from'bun:sqlite';const db=Database.deserialize(await Bun.stdin.bytes());if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('SQLite integrity');const counts=JSON.parse(process.argv[1]);for(const[name,count]of Object.entries(counts)){if(db.query('SELECT count(*) AS n FROM '+name).get().n!==count)throw Error(name+' count');}db.exec(process.argv[2]);process.stdout.write(db.serialize());"#;
    let mut child =
        std::process::Command::new("bun").args(["--eval", script, &corpus["nativeTableRowCounts"].to_string(), edit]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    output.stdout
}
#[test]
fn sqlite_snapshot_puzzle3d_all23_tables_physical_oracle_and_edit_reconstruct_all_words() {
    use store::sqlite_snapshot::*;
    use store::ArtifactSqliteSnapshot;
    for word in words() {
        let expected = specimen(word);
        let mut cb = |_| true;
        let mut control = SqliteSnapshotControl::new(&mut cb, SqliteDatabaseLimits::default());
        let database = expected.to_sqlite_database(&mut control).unwrap();
        assert_eq!(database.tables.len(), 23);
        assert_eq!(database.tables.iter().map(|t| t.rows.len()).sum::<usize>(), 30);
        let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
        let edited = independent_puzzle3_edit(&bytes, "UPDATE puzzle3_document SET domain='independent'; UPDATE puzzle3_reference_source SET url='unresolved!@/';");
        let database = import_sqlite_database(&edited, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
        let actual = Puzzle3dSnapshot::from_sqlite_database(&database, &mut control).unwrap();
        let mut expected = expected;
        expected.domain = "independent".into();
        expected.references[0].source.url = "unresolved!@/".into();
        assert_eq!(actual.print_dsl(), expected.print_dsl());
    }
}
#[test]
fn sqlite_snapshot_puzzle3d_independently_edited_malformed_graphs_are_rejected() {
    use store::sqlite_snapshot::*;
    use store::ArtifactSqliteSnapshot;
    let mut cb = |_| true;
    let mut control = SqliteSnapshotControl::new(&mut cb, SqliteDatabaseLimits::default());
    let database = specimen(0).to_sqlite_database(&mut control).unwrap();
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    for edit in corpus["nativeMalformedSql"].as_array().unwrap() {
        let file = independent_puzzle3_edit(&bytes, edit.as_str().unwrap());
        match import_sqlite_database(&file, SqliteDatabaseLimits::default(), &mut |_| true) {
            Err(_) => {}
            Ok(database) => assert!(Puzzle3dSnapshot::from_sqlite_database(&database, &mut control).is_err(), "{}", edit),
        }
    }
}
#[test]
fn sqlite_snapshot_puzzle3d_actual_typed_and_play_erased_codecs_keep_all23_domain_tables() {
    use store::sqlite_snapshot::*;
    let dialect = store::io_schema::ArtifactDialect { artifact_kind: "s.puzzle.puzzle3d".into(), standard: "1".into(), subset: "*".into() };
    for word in words() {
        let value = specimen(word);
        for codec in [<Puzzle3dSnapshot as ArtifactPack>::sqlite_snapshot_codec().unwrap(), <Puzzle3dPlaySnapshot as ArtifactPack>::sqlite_snapshot_codec().unwrap()] {
            for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
                let payload = match encoding {
                    SnapshotEncoding::Binary => store::io_schema::IoPayload::Binary(value.encode_pack()),
                    SnapshotEncoding::Text => store::io_schema::IoPayload::Text(value.print_dsl()),
                };
                let database = (codec.export)(PUZZLE_3D_SCHEMA, &dialect, &payload, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
                assert_eq!(database.tables.len(), 23);
                let output = (codec.import)(PUZZLE_3D_SCHEMA, &dialect, database, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
                let actual = match output {
                    store::io_schema::IoPayload::Binary(bytes) => Puzzle3dSnapshot::decode_pack(&bytes).unwrap(),
                    store::io_schema::IoPayload::Text(text) => Puzzle3dSnapshot::parse_dsl(&text).unwrap(),
                };
                assert_eq!(actual.print_dsl(), value.print_dsl());
            }
        }
    }
}

#[test]
fn sqlite_snapshot_puzzle3d_exact_rows_and_all_four_interior_copy_phases() {
    use store::sqlite_snapshot::*;
    use store::ArtifactSqliteSnapshot;
    let small = specimen(0);
    let exact = SqliteDatabaseLimits { max_rows: 30, ..Default::default() };
    let short = SqliteDatabaseLimits { max_rows: 29, ..Default::default() };
    let mut cb = |_| true;
    assert!(small.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut cb, exact)).is_ok());
    assert!(small.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut cb, short)).is_err());
    let payload = store::io_schema::IoPayload::Binary(small.encode_pack());
    assert!(Puzzle3dSnapshot::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut cb, short)).is_err());
    assert!(small.encode_sqlite_snapshot_native(SnapshotEncoding::Binary, &mut SqliteSnapshotControl::new(&mut cb, short)).is_err());
    let mut large = small;
    large.meta.kind_catalogs.as_mut().unwrap().objects[0].description = "引用😀".repeat(20000);
    let database = large.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut cb, Default::default())).unwrap();
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
        let mut control = SqliteSnapshotControl::new(&mut callback, Default::default());
        let rejected = match phase {
            SqliteSnapshotPhase::ProjectSnapshot => large.to_sqlite_database(&mut control).is_err(),
            SqliteSnapshotPhase::ReconstructSnapshot => Puzzle3dSnapshot::from_sqlite_database(&database, &mut control).is_err(),
            SqliteSnapshotPhase::DecodeNative => Puzzle3dSnapshot::decode_sqlite_snapshot_native(&payload, &mut control).is_err(),
            SqliteSnapshotPhase::EncodeNative => large.encode_sqlite_snapshot_native(SnapshotEncoding::Binary, &mut control).is_err(),
            _ => unreachable!(),
        };
        assert!(rejected);
        assert!(hit, "{:?}", phase);
    }
}
