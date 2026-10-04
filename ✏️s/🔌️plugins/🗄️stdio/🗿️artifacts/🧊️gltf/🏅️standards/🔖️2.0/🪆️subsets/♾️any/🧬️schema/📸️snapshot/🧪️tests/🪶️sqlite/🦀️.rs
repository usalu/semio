use super::*;
use semio_framework_os_kernel::{
    sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl},
    ArtifactSqliteSnapshot,
};

fn words() -> Vec<u64> {
    #[derive(value_derive::FromValue)]
    #[value(rename_all = "camelCase")]
    struct Words {
        binary64_bits: Vec<String>,
    }
    let fixture: Words = semio_framework_pack_json::from_json_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️words/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    fixture.binary64_bits.iter().map(|word| u64::from_str_radix(word, 16).unwrap()).collect()
}
fn word_snapshot(bits: u64) -> GltfSnapshot {
    let mut snapshot = fixture();
    let value = f64::from_bits(bits);
    snapshot.document.nodes[0].matrix = Some([value; 16]);
    snapshot.document.nodes[0].translation = Some([value; 3]);
    snapshot.document.nodes[0].rotation = Some([value; 4]);
    snapshot.document.nodes[0].scale = Some([value; 3]);
    snapshot.document.nodes[0].weights = vec![value];
    snapshot.document.meshes[0].weights = vec![value];
    snapshot.document.accessors[0].min = Some(vec![value]);
    snapshot.document.accessors[0].max = Some(vec![value]);
    let material = &mut snapshot.document.materials[0];
    material.emissive_factor = [value; 3];
    material.alpha_cutoff = value;
    let pbr = material.pbr_metallic_roughness.as_mut().unwrap();
    pbr.base_color_factor = [value; 4];
    pbr.metallic_factor = value;
    pbr.roughness_factor = value;
    material.normal_texture.as_mut().unwrap().scale = value;
    material.occlusion_texture.as_mut().unwrap().strength = value;
    snapshot.document.extras = Some(GltfJson::Object(vec![("z first".into(), GltfJson::Number(value)), ("a second".into(), GltfJson::Bool(true)), ("duplicate".into(), GltfJson::Number(value)), ("duplicate".into(), GltfJson::Null)]));
    if let GltfCameraProjection::Perspective(camera) = &mut snapshot.document.cameras[0].projection {
        camera.aspect_ratio = Some(value);
        camera.yfov = value;
        camera.zfar = Some(value);
        camera.znear = value;
    }
    if let GltfCameraProjection::Orthographic(camera) = &mut snapshot.document.cameras[1].projection {
        camera.xmag = value;
        camera.ymag = value;
        camera.zfar = value;
        camera.znear = value;
    }
    snapshot.document.nodes[0].mesh = Some(usize::MAX);
    snapshot.document.buffers[0].byte_length = usize::MAX;
    snapshot.document.samplers[0].mag_filter = Some(u64::MAX);
    snapshot
}
fn independent(database: &semio_framework_os_kernel::sqlite_snapshot::SqliteDatabase, sql: &str) -> semio_framework_os_kernel::sqlite_snapshot::SqliteDatabase {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let limits = SqliteDatabaseLimits::default();
    let bytes = export_sqlite_database(database, limits, &mut |_| true).unwrap();
    let script="const {Database}=require('bun:sqlite');const db=Database.deserialize(await Bun.stdin.bytes());if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('invalid semantic database');db.run(process.argv[1]);process.stdout.write(db.serialize());db.close();";
    let mut child = Command::new("bun").args(["-e", script, sql]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    import_sqlite_database(&output.stdout, limits, &mut |_| true).unwrap()
}
#[test]
fn sqlite_snapshot_gltf_independent_sql_reserialization_preserves_ieee_words_unsigned_and_duplicate_extras() {
    for bits in words() {
        let snapshot = word_snapshot(bits);
        let limits = SqliteDatabaseLimits::default();
        let expected = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
        let database = independent(&expected, "SELECT 1");
        let restored = GltfSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
        assert_eq!(restored.document.nodes[0].matrix.unwrap()[0].to_bits(), bits);
        assert_eq!(restored.document.nodes[0].mesh, Some(usize::MAX));
        assert_eq!(restored.document.samplers[0].mag_filter, Some(u64::MAX));
        assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap(), expected);
    }
}
#[test]
fn sqlite_snapshot_gltf_erased_binary_and_text_preserve_every_owned_field() {
    use store::sqlite_snapshot::SnapshotEncoding;
    let codec = <GltfSnapshot as ArtifactSqliteSnapshot>::sqlite_codec();
    let dialect = semio_framework_os_kernel::io_schema::ArtifactDialect { artifact_kind: "s.stdio.gltf".into(), standard: "2.0".into(), subset: "*".into() };
    for bits in words() {
        let snapshot = word_snapshot(bits);
        let limits = SqliteDatabaseLimits::default();
        let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
            let payload = (codec.import)(&snapshot.schema, &dialect, database.clone(), encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap().value;
            let restored = (codec.export)(&snapshot.schema, &dialect, &payload, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap().value;
            assert_eq!(restored, database);
        }
    }
}
#[test]
fn sqlite_snapshot_gltf_rejects_independently_edited_ieee_conflict_and_unowned_relationship() {
    let snapshot = fixture();
    let limits = SqliteDatabaseLimits::default();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    for sql in ["UPDATE gltf_material SET alpha_cutoff=0.75 WHERE id=1", "UPDATE gltf_primitive_attribute SET ordinal=4 WHERE id=1", "INSERT INTO gltf_scene_node VALUES(99,99,0,0,0)"] {
        let database = independent(&database, sql);
        assert!(GltfSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
    }
}

fn fixture() -> GltfSnapshot {
    semio_framework_pack_json::from_json_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()
}

#[test]
fn sqlite_snapshot_gltf_exact_typed_guard_budget_and_cancellation() {
    use semio_framework_os_kernel::{
        io_schema::ArtifactDialect,
        sqlite_snapshot::{SnapshotEncoding, SqliteSnapshotPhase},
    };
    let snapshot = fixture();
    let limits = SqliteDatabaseLimits::default();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    for (artifact_kind, standard, subset) in [("s.stdio.gltf", "2.0", "*"), ("s.stdio.obj", "2.0", "*"), ("s.stdio.gltf", "1.0", "*"), ("s.stdio.gltf", "2.0", "camera")] {
        let dialect = ArtifactDialect { artifact_kind: artifact_kind.into(), standard: standard.into(), subset: subset.into() };
        assert_eq!(snapshot.validate_sqlite_snapshot_subset(&dialect, &database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_ok(), artifact_kind == "s.stdio.gltf" && standard == "2.0" && subset == "*");
    }
    let mut wrong = database.clone();
    wrong.table_mut("gltf_document").unwrap().rows[0].values[1] = semio_framework_os_kernel::sqlite_snapshot::SqliteValue::Text("other".into());
    let dialect = ArtifactDialect { artifact_kind: "s.stdio.gltf".into(), standard: "2.0".into(), subset: "*".into() };
    assert!(snapshot.validate_sqlite_snapshot_subset(&dialect, &wrong, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
    let mut large = snapshot.clone();
    large.document.asset.generator = Some("\"".repeat(65537));
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_ok());
        let small = SqliteDatabaseLimits { max_value_bytes: 4096, ..limits };
        assert!(large.preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, small)).is_err());
        assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut |_| false, limits)).is_err());
    }
    let small = SqliteDatabaseLimits { max_rows: 1, ..limits };
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, small)).is_err());
    assert!(GltfSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| false, limits)).is_err());
    let mut work = snapshot.clone();
    work.document.nodes = vec![GltfNode::default(); 600];
    let mut reached = false;
    let mut progress = |event: semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress| {
        if event.phase == SqliteSnapshotPhase::EncodeNative && event.completed >= 256 {
            reached = true;
            false
        } else {
            true
        }
    };
    assert!(work.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Text, &mut SqliteSnapshotControl::new(&mut progress, limits)).is_err());
    assert!(reached);
}

#[test]
fn sqlite_snapshot_gltf_full_typed_document_and_independent_buffers_reconstruct() {
    let snapshot = fixture();
    let limits = SqliteDatabaseLimits::default();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    assert_eq!(database.table("gltf_primitive").unwrap().rows.len(), 1);
    assert_eq!(database.table("gltf_accessor").unwrap().rows.len(), 3);
    assert_eq!(database.table("gltf_resolved_buffer").unwrap().rows.len(), 3);
    let bytes = export_sqlite_database(&database, limits, &mut |_| true).unwrap();
    let database = import_sqlite_database(&bytes, limits, &mut |_| true).unwrap();
    let restored = GltfSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    assert_eq!(restored, snapshot);
}

#[test]
fn sqlite_snapshot_gltf_independent_sql_mesh_accessor_material_and_buffer_edits() {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let snapshot = fixture();
    let limits = SqliteDatabaseLimits::default();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    let bytes = export_sqlite_database(&database, limits, &mut |_| true).unwrap();
    let script = r#"const {Database}=require('bun:sqlite');const db=Database.deserialize(await Bun.stdin.bytes());if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('invalid semantic database');const row=db.query('SELECT m.name AS mesh,a.semantic,ac.name AS accessor,mat.name AS material FROM gltf_mesh m JOIN gltf_primitive p ON p.mesh_id=m.id JOIN gltf_primitive_attribute a ON a.primitive_id=p.id JOIN gltf_accessor ac ON ac.ordinal=a.accessor_index_low JOIN gltf_material mat ON mat.ordinal=p.material_index_low WHERE a.ordinal=0').get();if(row.mesh!=='Mesh'||row.semantic!=='POSITION'||row.accessor!=='Positions'||row.material!=='Material')throw Error('semantic join mismatch');db.run("UPDATE gltf_material SET name='Edited material' WHERE id=1");db.run('UPDATE gltf_resolved_byte SET value=17 WHERE buffer_id=1 AND ordinal=1');process.stdout.write(db.serialize());db.close();"#;
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let database = import_sqlite_database(&output.stdout, limits, &mut |_| true).unwrap();
    let restored = GltfSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    assert_eq!(restored.document.materials[0].name.as_deref(), Some("Edited material"));
    assert_eq!(restored.buffers[0], vec![0, 17, 1]);
    assert_eq!(restored.document.buffers[0].byte_length, 64);
}

#[test]
fn sqlite_snapshot_gltf_actual_factory_exposes_owned_provider_and_structural_hash() {
    let codec = (crate::native_codecs()[0].codec)();
    assert!(codec.snapshot_sqlite.is_some());
    let spec = <GltfSnapshot as store::ArtifactPack>::record_spec().expect("owned GLTF record spec");
    assert_eq!(codec.pack_schema_hash, store::schema_hash(&spec));
}

#[test]
fn sqlite_snapshot_gltf_empty_domain_tables_preserve_absence_and_present_empty_lists() {
    let mut snapshot: GltfSnapshot = semio_framework_pack_json::from_json_str(include_str!("../../🧫️fixtures/🪶️sqlite/🌱️minimal/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let limits = SqliteDatabaseLimits::default();
    let before = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    assert_eq!(before.tables.len(), 57);
    assert_eq!(before.table("gltf_accessor").unwrap().rows.len(), 0);
    let database = independent(&before, "SELECT 1");
    assert_eq!(GltfSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap(), snapshot);
    snapshot.document.accessors = vec![GltfAccessor {
        buffer_view: None,
        byte_offset: 0,
        component_type: GltfComponentType::Float,
        normalized: false,
        count: 0,
        kind: GltfAccessorType::Scalar,
        max: None,
        min: Some(Vec::new()),
        sparse: None,
        name: None,
        extensions: None,
        extras: None,
    }];
    snapshot.document.extras = Some(GltfJson::Null);
    snapshot.buffers = vec![Vec::new()];
    let database = independent(&snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap(), "SELECT 1");
    let restored = GltfSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    assert_eq!(restored, snapshot);
    assert_eq!(restored.document.accessors[0].min, Some(Vec::new()));
    assert_eq!(restored.document.accessors[0].max, None);
}

#[test]
fn sqlite_snapshot_gltf_deep_owned_extras_erased_payloads_have_bounded_wire_depth() {
    use store::sqlite_snapshot::SnapshotEncoding;
    let mut snapshot = fixture();
    #[derive(value_derive::FromValue)]
    #[value(rename_all = "camelCase")]
    struct Depth {
        json_depth: usize,
    }
    let depth: Depth = semio_framework_pack_json::from_json_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️words/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mut extras = GltfJson::Number(f64::from_bits(0x7ff8000000000042));
    for _ in 0..depth.json_depth {
        extras = GltfJson::Array(vec![extras])
    }
    snapshot.document.extras = Some(extras);
    let limits = SqliteDatabaseLimits::default();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    let codec = (crate::native_codecs()[0].codec)().snapshot_sqlite.expect("actual GLTF owned provider");
    let dialect = semio_framework_os_kernel::io_schema::ArtifactDialect { artifact_kind: "s.stdio.gltf".into(), standard: "2.0".into(), subset: "*".into() };
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let payload = (codec.import)(&snapshot.schema, &dialect, database.clone(), encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap().value;
        let restored = (codec.export)(&snapshot.schema, &dialect, &payload, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap().value;
        assert_eq!(restored, database);
    }
}

#[test]
fn sqlite_snapshot_gltf_logical_text_rejects_foreign_envelope_and_unowned_suffix() {
    let snapshot = fixture();
    let text = store::ArtifactDsl::print_dsl(&snapshot);
    assert!(<GltfSnapshot as store::ArtifactDsl>::parse_dsl(&text).is_ok());
    for invalid in [text.replace("stdio.gltf.dsl", "stdio.obj.dsl"), text.replace("stdio.gltf.dsl v1", "stdio.gltf.dsl v99"), format!("{text}\nunowned-field=1")] {
        assert!(<GltfSnapshot as store::ArtifactDsl>::parse_dsl(&invalid).is_err());
    }
}

#[test]
fn sqlite_snapshot_gltf_flat_primitive_extras_keep_deep_ordered_members_and_ieee_words() {
    let mut snapshot = fixture();
    let limits = SqliteDatabaseLimits::default();
    let mut extras = GltfJson::Number(f64::from_bits(0x7ff0000000001234));
    for _ in 0..1024 {
        extras = GltfJson::Object(vec![("same".into(), extras), ("same".into(), GltfJson::Null)])
    }
    snapshot.document.meshes[0].primitives[0].extras = Some(extras);
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    for payload in [semio_framework_os_kernel::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot)), semio_framework_os_kernel::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot))] {
        let restored = match payload {
            semio_framework_os_kernel::io_schema::IoPayload::Text(text) => <GltfSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap(),
            semio_framework_os_kernel::io_schema::IoPayload::Binary(bytes) => <GltfSnapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap(),
        };
        assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap(), database);
    }
}

#[test]
fn sqlite_snapshot_gltf_empty_image_and_texture_entities_survive_logical_native_lists() {
    let snapshot: GltfSnapshot = semio_framework_pack_json::from_json_str(include_str!("../../🧫️fixtures/🪶️sqlite/🪹️empty-entities/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let limits = SqliteDatabaseLimits::default();
    let database = independent(&snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap(), "SELECT 1");
    assert_eq!(database.table("gltf_image").unwrap().rows.len(), 2);
    assert_eq!(database.table("gltf_texture").unwrap().rows.len(), 1);
    for payload in [semio_framework_os_kernel::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot)), semio_framework_os_kernel::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot))] {
        let restored = match payload {
            semio_framework_os_kernel::io_schema::IoPayload::Text(text) => <GltfSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap(),
            semio_framework_os_kernel::io_schema::IoPayload::Binary(bytes) => <GltfSnapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap(),
        };
        assert_eq!(restored, snapshot);
        assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap(), database);
    }
}

#[test]
fn sqlite_snapshot_gltf_authored_grammar_recognizes_each_owned_named_domain() {
    let grammar = semio_framework_dsl::grammar::parse_grammar(include_str!("../../📝️text/📖️.grammar.semio")).unwrap();
    let recognizer = semio_framework_dsl::grammar::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros())
        .expect("selected grammar fragments");
    for snapshot in [fixture(), semio_framework_pack_json::from_json_str::<GltfSnapshot>(include_str!("../../🧫️fixtures/🪶️sqlite/🪹️empty-entities/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()] {
        let text = store::ArtifactDsl::print_dsl(&snapshot);
        assert!(recognizer.recognize(&text).unwrap(), "{text}");
    }
}

#[test]
fn sqlite_snapshot_gltf_controlled_binding_retires_deep_completed_fields_after_failure_and_cancellation() {
    use semio_framework_dsl_record::DslField;
    #[derive(value_derive::FromValue)]
    #[value(rename_all = "camelCase")]
    struct Depth {
        controlled_retirement_depth: usize,
    }
    let depth_fixture: Depth = semio_framework_pack_json::from_json_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️words/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(move || {
            #[derive(semio_framework_dsl_record_derive::DslRecord)]
            struct Retained {
                primitive: GltfPrimitive,
                required: u64,
            }
            let depth = depth_fixture.controlled_retirement_depth;
            let mut json = GltfJson::Number(f64::from_bits(0x7ff0000000001234));
            for _ in 0..depth {
                json = GltfJson::Array(vec![json]);
            }
            let mut primitive = fixture().document.meshes.remove(0).primitives.remove(0);
            primitive.extensions = Some(json);
            primitive.extras = None;
            let value = Retained { primitive, required: 7 };
            let mut fields = match value.to_value() {
                semio_framework_dsl_record::FieldValue::Record(record) => record,
                _ => unreachable!(),
            };
            Retained::retire_decoded(value);
            let required = fields.fields.remove(&1).unwrap();
            let mut callback = |_: semio_framework_value::native_decoding::NativeDecodeProgress| true;
            let mut control = semio_framework_value::NativeDecodeControl::new(64 * 1024 * 1024, &mut callback);
            let error = Retained::from_record_controlled(&fields, &mut control).err().expect("missing later field must reject");
            assert_eq!(error.kind, semio_framework_value::ValueRefusalKind::InvalidValue);
            assert!(error.message.contains("required"), "{error}");
            fields.fields.insert(1, required);
            let mut reached = false;
            let mut callback = |progress: semio_framework_value::native_decoding::NativeDecodeProgress| {
                if progress.total == depth * 2 + 1 && progress.completed >= 256 {
                    reached = true;
                    false
                } else {
                    true
                }
            };
            let mut control = semio_framework_value::NativeDecodeControl::new(64 * 1024 * 1024, &mut callback);
            assert!(Retained::from_record_controlled(&fields, &mut control).is_err());
            assert!(reached, "cancel inside actual extras reconstruction");
            let mut callback = |_: semio_framework_value::native_decoding::NativeDecodeProgress| true;
            let mut control = semio_framework_value::NativeDecodeControl::new(64 * 1024 * 1024, &mut callback);
            let value = Retained::from_record_controlled(&fields, &mut control).unwrap();
            let mut current = value.primitive.extensions.as_ref().unwrap();
            for _ in 0..depth {
                match current {
                    GltfJson::Array(values) => current = &values[0],
                    _ => panic!("lost typed array"),
                }
            }
            assert!(matches!(current,GltfJson::Number(value)if value.to_bits()==0x7ff0000000001234));
            Retained::retire_decoded(value);
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn sqlite_snapshot_gltf_independent_bad_later_json_owner_retires_deep_completed_sibling() {
    std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(|| {
            let mut snapshot = fixture();
            let mut value = GltfJson::Null;
            for _ in 0..2048 {
                value = GltfJson::Array(vec![value]);
            }
            snapshot.document.extras = Some(GltfJson::Object(vec![("completed deep sibling".into(), value), ("bad later scalar".into(), GltfJson::Bool(true))]));
            let limits = SqliteDatabaseLimits::default();
            let expected = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
            let database = independent(&expected, "UPDATE gltf_json_object_member SET value_id=(SELECT value_id FROM gltf_json_object_member WHERE name='completed deep sibling') WHERE name='bad later scalar'");
            assert!(GltfSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
            GltfSnapshot::retire_sqlite_snapshot(snapshot);
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn sqlite_snapshot_gltf_independent_later_root_owner_failure_retires_completed_extensions() {
    std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(|| {
            let mut snapshot = fixture();
            let mut value = GltfJson::Null;
            for _ in 0..2048 {
                value = GltfJson::Array(vec![value]);
            }
            snapshot.document.extensions = Some(value);
            snapshot.document.extras = None;
            let limits = SqliteDatabaseLimits::default();
            let expected = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
            let database = independent(&expected, "UPDATE gltf_document SET extras_id=extensions_id");
            assert!(GltfSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
            GltfSnapshot::retire_sqlite_snapshot(snapshot);
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn sqlite_snapshot_gltf_curated_demo_uses_actual_owned_flat_native_records() {
    let demo = crate::engine::demo_gltf_snapshot();
    let text = include_str!("../../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
    let bytes = include_bytes!("../../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio");
    assert_eq!(store::ArtifactDsl::print_dsl(&demo), text);
    assert_eq!(store::ArtifactPack::encode_pack(&demo), bytes);
    assert_eq!(<GltfSnapshot as store::ArtifactDsl>::parse_dsl(text).unwrap(), demo);
    assert_eq!(<GltfSnapshot as store::ArtifactPack>::decode_pack(bytes).unwrap(), demo);
}

#[test]
fn sqlite_snapshot_gltf_curated_metabolism_pack_is_current_owned_logical_state() {
    let snapshot = crate::engine::decode_glb(include_bytes!("../../../../🖼️assets/🌱️metabolism/🏙️base/🧊️.glb")).unwrap();
    let bytes = include_bytes!("../../../../📚️examples/🌱️metabolism/🖼️assets/🎒️.pack.semio");
    assert!(store::ArtifactPack::encode_pack(&snapshot).as_slice() == bytes, "curated metabolism Pack must match the actual current owned native record graph");
    assert_eq!(<GltfSnapshot as store::ArtifactPack>::decode_pack(bytes).unwrap(), snapshot);
}

#[test]
fn sqlite_snapshot_gltf_typed_guard_rejects_independently_changed_owned_state() {
    let snapshot = fixture();
    let limits = SqliteDatabaseLimits::default();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    let edited = independent(&database, "UPDATE gltf_material SET name='Different valid state' WHERE id=1");
    let dialect = semio_framework_os_kernel::io_schema::ArtifactDialect { artifact_kind: "s.stdio.gltf".into(), standard: "2.0".into(), subset: "*".into() };
    assert!(snapshot.validate_sqlite_snapshot_subset(&dialect, &edited, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err(), "typed guard must compare all owned state");
    let restored = GltfSnapshot::from_sqlite_database(&edited, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    assert!(restored.validate_sqlite_snapshot_subset(&dialect, &edited, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_ok());
}

#[test]
fn sqlite_snapshot_gltf_typed_guard_accepts_independently_renumbered_surrogate() {
    let snapshot = fixture();
    let limits = SqliteDatabaseLimits::default();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    let edited = independent(&database, "UPDATE gltf_scene_node SET id=99 WHERE id=1");
    let dialect = semio_framework_os_kernel::io_schema::ArtifactDialect { artifact_kind: "s.stdio.gltf".into(), standard: "2.0".into(), subset: "*".into() };
    assert!(snapshot.validate_sqlite_snapshot_subset(&dialect, &edited, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_ok(), "surrogate row identity must not change typed state");
}

#[test]
fn sqlite_snapshot_gltf_native_owned_decode_controls_physical_and_typed_materialization() {
    let mut value = word_snapshot(0x7ff0000000001234);
    value.schema = "nondefault controlled GLTF schema".into();
    value.document.extras = Some(GltfJson::Array((0..600).map(|index| GltfJson::String(if index == 0 { "long semantic literal 世界".repeat(4096) } else { "child".into() })).collect()));
    let limits = SqliteDatabaseLimits { max_value_bytes: 128 * 1024 * 1024, ..SqliteDatabaseLimits::default() };
    let expected = value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    for payload in [store::os_io::IoPayload::Text(store::ArtifactDsl::print_dsl(&value)), store::os_io::IoPayload::Binary(store::ArtifactPack::encode_pack(&value))] {
        let restored = GltfSnapshot::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
        assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap(), expected);
        GltfSnapshot::retire_sqlite_snapshot(restored);
        assert!(GltfSnapshot::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_value_bytes: 4096, ..limits })).is_err());
        let mut interior = false;
        let mut progress = |event: store::sqlite_snapshot::SqliteSnapshotProgress| {
            if event.phase == store::sqlite_snapshot::SqliteSnapshotPhase::DecodeNative && event.completed >= 256 && event.completed < event.total {
                interior = true;
                false
            } else {
                true
            }
        };
        assert!(GltfSnapshot::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut progress, limits)).is_err());
        assert!(interior, "must cancel inside known actual native work");
    }
    GltfSnapshot::retire_sqlite_snapshot(value);
}

#[test]
fn sqlite_snapshot_gltf_genuine_native_output_preserves_paid_full_word_state() {
    use store::sqlite_snapshot::{SnapshotEncoding, SqliteSnapshotPhase};
    let limits = SqliteDatabaseLimits::default();
    for word in words() {
        let snapshot = word_snapshot(word);
        let expected = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
            let mut events = Vec::new();
            let mut accept = |p| {
                events.push(p);
                true
            };
            let payload = snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut accept, limits)).unwrap();
            assert!(events.iter().any(|p| p.phase == SqliteSnapshotPhase::EncodeNative && p.total > 0 && p.completed == p.total));
            let actual = GltfSnapshot::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
            assert_eq!(actual.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap(), expected);
            GltfSnapshot::retire_sqlite_snapshot(actual);
        }
        GltfSnapshot::retire_sqlite_snapshot(snapshot);
    }
}
#[test]
fn sqlite_snapshot_gltf_genuine_native_output_rejects_limits_and_cancels_during_paid_copy() {
    use store::sqlite_snapshot::{SnapshotEncoding, SqliteSnapshotPhase};
    let contract: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️words/🔣️.json")).unwrap();
    let work = &contract["controlledOutput"];
    let mut snapshot = fixture();
    snapshot.document.asset.generator = Some(work["textUnit"].as_str().unwrap().repeat(work["repeat"].as_u64().unwrap() as usize));
    let limits = SqliteDatabaseLimits::default();
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let payload = snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
        assert!(match payload {
            store::os_io::IoPayload::Binary(v) => !v.is_empty(),
            store::os_io::IoPayload::Text(v) => !v.is_empty(),
        });
        for limits in [SqliteDatabaseLimits { max_value_bytes: work["refusedBytes"].as_u64().unwrap() as usize, ..limits }, SqliteDatabaseLimits { max_file_bytes: 1, ..limits }] {
            assert!(snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
        }
        assert!(snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| false, limits)).is_err());
        let mut reached = false;
        let mut progress = |p: store::sqlite_snapshot::SqliteSnapshotProgress| {
            if p.phase == SqliteSnapshotPhase::EncodeNative && p.total >= 65536 && p.completed >= 65536 && p.completed < p.total {
                reached = true;
                false
            } else {
                true
            }
        };
        assert!(snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut progress, limits)).is_err());
        assert!(reached);
    }
    GltfSnapshot::retire_sqlite_snapshot(snapshot);
}
