use super::*;
use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue}, ArtifactSqliteSnapshot};

fn fixture() -> PngSnapshot { pack::json::from_json_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap() }

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_png_owned_io_preserves_source_color_and_chunk_semantics() {
    use semio_framework_os_kernel::io::{register_native_snapshot_codec, ArtifactDialect, Dialect, StandardId, SubsetId, io_mechanism::{io_export_sqlite_snapshot, io_import_sqlite_snapshot}};
    use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding, SqliteSnapshotPhase};
    let native = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    register_native_snapshot_codec(native, store::ArtifactCodec::of::<PngSnapshot, crate::PngMutation>("stdio.png")).unwrap();
    let dialect: ArtifactDialect = native.into();
    let mut snapshot = fixture();
    snapshot.schema = "PNG vollständiger Snapshot".into();
    let mut phases = Vec::new();
    let bytes = io_export_sqlite_snapshot(&dialect, &snapshot, SnapshotEncoding::Binary, SqliteDatabaseLimits::default(), &mut |progress| { phases.push(progress.phase); true }).await.unwrap().value;
    let restored = io_import_sqlite_snapshot::<PngSnapshot>(&dialect, &bytes, SqliteDatabaseLimits::default(), &mut |_| true).await.unwrap().value;
    assert_eq!(restored, snapshot);
    assert!(!phases.iter().any(|phase| matches!(phase, SqliteSnapshotPhase::EncodeNative | SqliteSnapshotPhase::DecodeNative)));
}

#[test]
fn sqlite_snapshot_png_all_semantic_fields_and_typed_variants_roundtrip() {
    let snapshot = fixture();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(database.tables.len(), 16);
    assert!(database.tables.iter().flat_map(|table| &table.rows).flat_map(|row| &row.values).all(|value| !matches!(value, SqliteValue::Blob(_))));
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let database = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let restored = PngSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(restored, snapshot);
    let oracle: serde_json::Value = serde_json::from_str(&pack::json::to_json_string(&protocol::ToValue::to_value(&restored))).unwrap();
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    assert_eq!(oracle, expected);
    for variant in 0..4 {
        let mut value = PngSnapshot::default();
        value.plte = if variant == 0 { Some(Vec::new()) } else { None };
        value.trns = match variant { 0 => Some(PngTransparency::Indexed { alpha: Vec::new() }), 1 => Some(PngTransparency::Grayscale { gray: u16::MAX }), 2 => Some(PngTransparency::Rgb { r: 1, g: 2, b: u16::MAX }), _ => None };
        value.bkgd = match variant { 0 => Some(PngBackground::Indexed { index: u8::MAX }), 1 => Some(PngBackground::Grayscale { gray: u16::MAX }), 2 => Some(PngBackground::Rgb { r: 1, g: 2, b: u16::MAX }), _ => None };
        let database = value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        assert_eq!(PngSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), value);
    }
    for alteration in 0..5 {
        let mut broken = database.clone();
        match alteration {
            0 => broken.table_mut("png_chunk_sequence").unwrap().rows[9].values[4] = SqliteValue::Integer(999),
            1 => broken.table_mut("png_pixel").unwrap().rows[0].values[7] = SqliteValue::Integer(256),
            2 => broken.table_mut("png_pixel").unwrap().rows[1].values[2] = SqliteValue::Integer(0),
            3 => broken.table_mut("png_unknown_chunk_byte").unwrap().rows[0].values[1] = SqliteValue::Integer(999),
            _ => { broken.table_mut("png_pixel").unwrap().rows.pop(); },
        }
        assert!(PngSnapshot::from_sqlite_database(&broken, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
    }
    let limits = SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() };
    assert!(PngSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
}

#[test]
fn sqlite_snapshot_png_independent_sql_pixels_chunks_and_unicode_edits() {
    use std::{io::Write, process::{Command, Stdio}};
    let mut snapshot = fixture();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let script = "import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const rows=db.query('SELECT s.ordinal,t.keyword,t.content FROM png_chunk_sequence s JOIN png_text t ON t.id=s.text_id ORDER BY s.ordinal').all();if(rows.length!==2||rows[0].content!=='second'||rows[1].keyword!=='Titel')throw Error('chunk relationships');db.query('UPDATE png_pixel SET alpha=128 WHERE x=1 AND y=0').run();db.query('UPDATE png_text SET content=? WHERE ordinal=0').run('SQLite Bild');db.query('UPDATE png_unknown_chunk_byte SET value=42 WHERE ordinal=2').run();await Bun.write(Bun.stdout,db.serialize());db.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let edited = import_sqlite_database(&output.stdout, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let restored = PngSnapshot::from_sqlite_database(&edited, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    snapshot.pixels[7] = 128;
    snapshot.text_chunks[0].value = "SQLite Bild".into();
    snapshot.unknown_chunks[0].data[2] = 42;
    assert_eq!(restored, snapshot);
}
