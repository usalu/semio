use super::*;
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


#[test]
fn sqlite_snapshot_tiff_erased_native_preserves_complete_custom_owner() {
    let neutral: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🧾️native-owner/🔣️.json")).expect("closed neutral owner");
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
