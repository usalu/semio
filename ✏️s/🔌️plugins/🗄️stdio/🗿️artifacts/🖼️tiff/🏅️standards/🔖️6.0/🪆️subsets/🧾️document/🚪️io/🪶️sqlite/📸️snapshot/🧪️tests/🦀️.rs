use crate::standards::v6_0::subsets::document::io::sqlite::snapshot::*;
use semio_framework_os_kernel::{sqlite_snapshot::*, ArtifactSqliteSnapshot};

#[path = "🚦️cohort/🦀️.rs"]
mod cohort;

fn fixture() -> TiffSnapshot {
    let values = vec![
        TiffValues::Byte(vec![0, 255]),
        TiffValues::Ascii(b"exact\0octets\0".to_vec()),
        TiffValues::Short(vec![0, u16::MAX]),
        TiffValues::Long(vec![0, u32::MAX]),
        TiffValues::Rational(vec![(u32::MAX, 0)]),
        TiffValues::SByte(vec![i8::MIN, i8::MAX]),
        TiffValues::Undefined(vec![0, 255]),
        TiffValues::SShort(vec![i16::MIN, i16::MAX]),
        TiffValues::SLong(vec![i32::MIN, i32::MAX]),
        TiffValues::SRational(vec![(i32::MIN, -1)]),
        TiffValues::Float(vec![TiffBinary32 { bits: 0x7fc0_0042 }]),
        TiffValues::Double(vec![TiffBinary64 { bits: 0x7ff8_0000_0000_0042 }]),
    ];
    TiffSnapshot {
        byte_order: TiffByteOrder::BigEndian,
        ifds: vec![TiffIfd {
            entries: values.into_iter().enumerate().map(|(index, values)| TiffTag { tag: 60000 + index as u16, values }).collect(),
            storage: TiffStorage { kind: TiffStorageKind::Strips, offsets_kind: TiffFieldType::Short, byte_counts_kind: TiffFieldType::Long, chunks: vec![vec![1, 2], vec![3, 4, 5]] },
        }],
        ..TiffSnapshot::default()
    }
}

fn project(snapshot: &TiffSnapshot) -> SqliteDatabase {
    snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).expect("project")
}

fn restore(database: &SqliteDatabase) -> Result<TiffSnapshot, ValueError> {
    TiffSnapshot::from_sqlite_database(database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default()))
}

fn bun_oracle(bytes: &[u8], script: &str) -> Vec<u8> {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("bun");
    child.stdin.take().expect("stdin").write_all(bytes).expect("write");
    let output = child.wait_with_output().expect("wait");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    output.stdout
}

#[test]
fn sqlite_preserves_storage_ascii_octets_and_ieee_bits() {
    let snapshot = fixture();
    assert_eq!(restore(&project(&snapshot)).expect("restore"), snapshot);
}

#[test]
fn independent_sqlite_oracle_reads_and_edits_owned_chunk_bytes() {
    let snapshot = fixture();
    let bytes = export_sqlite_database(&project(&snapshot), SqliteDatabaseLimits::default(), &mut |_| true).expect("SQLite bytes");
    let bytes = bun_oracle(&bytes, "import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(d.query('SELECT hex(payload) AS h FROM tiff_chunk ORDER BY ordinal').get().h!=='0102')throw Error('chunk');d.query('UPDATE tiff_chunk SET payload=? WHERE ordinal=0').run(new Uint8Array([9,8]));await Bun.write(Bun.stdout,d.serialize());d.close();");
    let database = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).expect("import");
    let mut expected = snapshot;
    expected.ifds[0].storage.chunks[0] = vec![9, 8];
    assert_eq!(restore(&database).expect("restore edited"), expected);
}

#[test]
fn relational_and_ownership_limits_are_enforced() {
    let snapshot = fixture();
    let database = project(&snapshot);
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_rows: 1, ..Default::default() })).is_err());
    assert!(TiffSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_value_bytes: 1, ..Default::default() })).is_err());
    let mut malformed = database;
    malformed.table_mut("tiff_chunk").expect("table").rows[0].values[1] = SqliteValue::Integer(9);
    assert!(restore(&malformed).is_err());
}


/// 🏭️ Matches the published TIFF binding to its genuine codec and independent protocol SHA256.
#[test]
fn sqlite_snapshot_tiff_declared_native_factory_matches_live_protocol() {
    let factories = crate::native_codecs();
    let factory = &factories[0];
    let codec = (factory.codec)();
    let kind = (factory.kind)();
    let declared: serde_json::Value = serde_json::from_str(crate::ARTIFACT_DEFINITION_SCHEMA).expect("TIFF definition");
    let binding = &declared["codecs"][0]["native_factory"];
    let hash: String = codec.pack_schema_hash.iter().map(|byte| format!("{byte:02x}")).collect();
    eprintln!("[DEBUG] TIFF actual native factory={} artifact={} kind={} schema={} extension={} pack_hash={} declared_binding={}", factory.id, factory.artifact, kind.id, codec.schema, codec.extension, hash, binding);
    let protocol = include_bytes!("../../../💾️binary/📸️snapshot/📡️.protocol.semio");
    let independent = bun_oracle(protocol, r#"import{createHash}from'node:crypto';await Bun.write(Bun.stdout,createHash('sha256').update(new Uint8Array(await Bun.stdin.arrayBuffer())).digest('hex'));"#);
    assert_eq!(hash, String::from_utf8(independent).expect("independent protocol hash"));
    crate::definition().expect("published TIFF native factory exactly matches its live protocol owner");
}

#[test]
fn sqlite_snapshot_tiff_erased_native_preserves_complete_custom_owner() {
    let neutral: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧾️native-owner/🔣️.json")).expect("closed neutral owner");
    let snapshot = TiffSnapshot {
        schema: neutral["schema"].as_str().expect("schema").to_owned(),
        byte_order: TiffByteOrder::LittleEndian,
        ifds: vec![TiffIfd {
            entries: vec![TiffTag { tag: 0, values: TiffValues::Ascii(Vec::new()) }, TiffTag { tag: 1, values: TiffValues::Double(Vec::new()) }],
            storage: TiffStorage { kind: TiffStorageKind::None, offsets_kind: TiffFieldType::Short, byte_counts_kind: TiffFieldType::Long, chunks: Vec::new() },
        }],
    };
    let dialect = semio_framework_os_kernel::io_schema::ArtifactDialect { artifact_kind: "s.stdio.tiff".into(), standard: "6.0".into(), subset: "*".into() };
    let codec = TiffSnapshot::sqlite_codec();
    for encoding in [SnapshotEncoding::Text, SnapshotEncoding::Binary] {
        let imported = (codec.import)(&snapshot.schema, &dialect, project(&snapshot), encoding, &mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).expect("erased native import");
        let decoded = TiffSnapshot::decode_sqlite_snapshot_native(&imported.value, &mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).expect("decode imported full owner");
        assert_eq!(decoded, snapshot, "erased import retains every declared field");
        let exported = (codec.export)(&snapshot.schema, &dialect, &imported.value, &mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).expect("erased native export");
        assert_eq!(restore(&exported.value).expect("restore exported full owner"), snapshot, "erased export retains every declared field");
    }
}

#[test]
fn ordinary_and_controlled_initial_record_pack_body_diagnostic() {
    use pack::record as pack_rt;
    let owner = <crate::editor::tiff_any::TiffAnyEditor as semio_framework_plugin::ArtifactEditor>::initial_snapshot();
    let original_spec = crate::standards::v6_0::subsets::document::schema::snapshot::text::spec();
    let original_record = crate::standards::v6_0::subsets::document::schema::snapshot::text::to_record(&owner);
    let maximum = semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits::default().max_allocation_bytes;
    let mut observer = |_| true;
    let mut native = semio_framework_value::NativeEncodeControl::new(maximum, &mut observer);
    let paid_spec = crate::standards::v6_0::subsets::document::schema::snapshot::text::spec_producer().encode(&mut native).unwrap();
    let paid_record = crate::standards::v6_0::subsets::document::schema::snapshot::text::to_record_controlled(&owner, &mut native).unwrap();
    let options = pack_rt::EncodeOptions::default();
    let original = pack_rt::encode_document(&original_spec, &original_record, &options).unwrap();
    let record_join = pack_rt::encode_document(&original_spec, &paid_record, &options).unwrap();
    let spec_join = pack_rt::encode_document(&paid_spec, &original_record, &options).unwrap();
    let controlled = pack_rt::encode_document_controlled(&paid_spec, &paid_record, &options, &mut native).unwrap();
    for (label,bytes) in [("record", &record_join),("spec", &spec_join),("controlled", &controlled)] {
        let first = original.iter().zip(bytes.iter()).position(|(left,right)|left != right);
        eprintln!("[DEBUG] tiff body {label} original={} candidate={} first={first:?}",original.len(),bytes.len());
    }
    let original_decoded = pack_rt::decode_document(&original,&original_spec,&pack_rt::DecodeOptions::default()).unwrap().0;
    let controlled_decoded = pack_rt::decode_document(&controlled,&original_spec,&pack_rt::DecodeOptions::default()).unwrap().0;
    eprintln!("[DEBUG] tiff record equal={} decoded equal={} original={:?} controlled={:?}",original_record == paid_record,original_decoded == controlled_decoded,original_record,paid_record);
    assert_eq!(original_record,paid_record,"real initial owner RecordValue changed");
    assert_eq!(original,record_join,"paid RecordValue changes ordinary body");
    assert_eq!(original,spec_join,"paid RecordSpec changes ordinary body");
    assert_eq!(original,controlled,"controlled encoder changes exact body");
    let shipped = semio_framework_os_kernel::pack_rt::encode_document(&original_spec,&original_record,&semio_framework_os_kernel::PackEncodeOptions::default()).unwrap();
    eprintln!("[DEBUG] actual shipped Record authority original={} core={} first={:?}",shipped.len(),original.len(),shipped.iter().zip(&original).position(|(left,right)|left!=right));
    assert_eq!(shipped,original,"actual shipped ArtifactPack runtime must use the same intrinsic Record authority as SQLite");
}

fn norm_complete_source(case:&serde_json::Value)->TiffSnapshot{if case["id"]=="originalNativeOneIfd"{return fixture()}if case["id"]=="emptyIfds"{let mut source=fixture();source.ifds.clear();return source}let json:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let octets=|name:&str|->Vec<u8>{json[name].as_array().unwrap().iter().map(|value|u8::try_from(value.as_u64().unwrap()).unwrap()).collect()};let chunks=|name:&str|->Vec<Vec<u8>>{json[name].as_array().unwrap().iter().map(|values|values.as_array().unwrap().iter().map(|value|u8::try_from(value.as_u64().unwrap()).unwrap()).collect()).collect()};let values=vec![TiffValues::Byte(vec![0,255]),TiffValues::Ascii(octets("asciiBytes")),TiffValues::Short(vec![0,u16::MAX]),TiffValues::Long(vec![0,u32::MAX]),TiffValues::Rational(vec![(u32::MAX,0),(1,u32::MAX)]),TiffValues::SByte(vec![i8::MIN,i8::MAX]),TiffValues::Undefined(vec![0,255]),TiffValues::SShort(vec![i16::MIN,i16::MAX]),TiffValues::SLong(vec![i32::MIN,i32::MAX]),TiffValues::SRational(vec![(i32::MIN,0),(i32::MAX,-1)]),TiffValues::Float(json["float32Bits"].as_array().unwrap().iter().map(|word|TiffBinary32{bits:u32::from_str_radix(word.as_str().unwrap(),16).unwrap()}).collect()),TiffValues::Double(json["float64Bits"].as_array().unwrap().iter().map(|word|TiffBinary64{bits:u64::from_str_radix(word.as_str().unwrap(),16).unwrap()}).collect())];TiffSnapshot{schema:json["schema"].as_str().unwrap().into(),byte_order:TiffByteOrder::BigEndian,ifds:vec![TiffIfd{entries:values.into_iter().map(|values|TiffTag{tag:u16::MAX,values}).collect(),storage:TiffStorage{kind:TiffStorageKind::Strips,offsets_kind:TiffFieldType::Short,byte_counts_kind:TiffFieldType::Long,chunks:chunks("primaryChunks")}},TiffIfd{entries:vec![TiffTag{tag:0,values:TiffValues::Byte(Vec::new())}],storage:TiffStorage{kind:TiffStorageKind::Tiles,offsets_kind:TiffFieldType::Long,byte_counts_kind:TiffFieldType::Short,chunks:chunks("secondaryChunks")}}]}}
 fn norm_independent_complete_extent(source:&TiffSnapshot)->serde_json::Value{
 use store::{ArtifactSqliteSnapshot as _,sqlite_snapshot::*};use std::{io::Write,process::{Command,Stdio}};let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let script=concat!(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔮️semantic-extent/🟦️.ts"),"\nawait Bun.write(Bun.stdout,JSON.stringify(independentSqliteExtent(new Uint8Array(await Bun.stdin.arrayBuffer()))));");let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));serde_json::from_slice(&output.stdout).unwrap()
}
#[test]
fn sqlite_snapshot_tiff_complete_independent_native_semantic_limits(){
 use store::{ArtifactSqliteSnapshot as _,sqlite_snapshot::*};let contract:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎛️semantic.json")).unwrap();
 for case in contract["cases"].as_array().unwrap(){let source=norm_complete_source(case);let extent=norm_independent_complete_extent(&source);assert_eq!(extent["rows"],case["rows"]);assert_eq!(extent["valueBytes"],case["valueBytes"]);assert_eq!(extent["schemaBytes"],contract["schemaBytes"]);assert_eq!(extent["tableWidths"],contract["tableWidths"]);let limits=SqliteDatabaseLimits{max_rows:case["rows"].as_u64().unwrap()as usize,max_value_bytes:case["valueBytes"].as_u64().unwrap()as usize,max_schema_bytes:contract["schemaBytes"].as_u64().unwrap()as usize,max_tables:16,max_columns:6,..SqliteDatabaseLimits::default()};let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(TiffSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),source);
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let payload=source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(TiffSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),source);
   for short in[SqliteDatabaseLimits{max_rows:limits.max_rows-1,..limits},SqliteDatabaseLimits{max_value_bytes:limits.max_value_bytes-1,..limits},SqliteDatabaseLimits{max_schema_bytes:limits.max_schema_bytes-1,..limits},SqliteDatabaseLimits{max_tables:15,..limits},SqliteDatabaseLimits{max_columns:5,..limits}]{assert!(source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"relational copied limits {short:?}");assert!(TiffSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"reconstruct copied limits {short:?}");assert!(source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"preflight copied limits {encoding:?} {short:?}");assert!(source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"encoder copied limits {encoding:?} {short:?}");assert!(TiffSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"decoder copied limits {encoding:?} {short:?}");}
  }
 }eprintln!("[DEBUG] TIFF independently measured complete copied native semantic limits");
}
#[test]
fn sqlite_snapshot_tiff_complete_independent_copied_columns_admission(){
 use store::{ArtifactSqliteSnapshot as _,sqlite_snapshot::*};let source=fixture();let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let limits=SqliteDatabaseLimits{max_columns:5,..SqliteDatabaseLimits::default()};assert!(source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());assert!(TiffSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert!(source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"preflight copied columns {encoding:?}");assert!(source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"encoder copied columns {encoding:?}");assert!(TiffSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"decoder copied columns {encoding:?}");}
}
