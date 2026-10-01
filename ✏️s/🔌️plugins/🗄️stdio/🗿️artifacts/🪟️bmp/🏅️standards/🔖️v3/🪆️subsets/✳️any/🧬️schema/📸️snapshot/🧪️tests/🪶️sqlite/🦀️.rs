use super::*;
use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue}, ArtifactSqliteSnapshot};

fn fixture() -> BmpSnapshot { pack::json::from_json_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap() }

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_bmp_owned_io_preserves_all_header_and_rgba_fields() {
    use semio_framework_os_kernel::io::{register_native_snapshot_codec, ArtifactDialect, Dialect, StandardId, SubsetId, io_mechanism::{io_export_sqlite_snapshot, io_import_sqlite_snapshot}};
    use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding, SqliteSnapshotPhase};
    let native = Dialect { artifact_kind: "s.stdio.bmp", standard: StandardId("v3"), subset: SubsetId("*") };
    register_native_snapshot_codec(native, store::ArtifactCodec::of::<BmpSnapshot, crate::BmpMutation>("stdio.bmp")).unwrap();
    let dialect: ArtifactDialect = native.into();
    let mut snapshot = fixture();
    snapshot.schema = "BMP vollständiger Snapshot".into();
    snapshot.header_size = 108;
    snapshot.bits_per_pixel = 32;
    snapshot.image_size = 16;
    snapshot.colors_important = 0;
    snapshot.palette.clear();
    snapshot.pixels[3] = 17;
    let mut phases = Vec::new();
    let bytes = io_export_sqlite_snapshot(&dialect, &snapshot, SnapshotEncoding::Binary, SqliteDatabaseLimits::default(), &mut |progress| { phases.push(progress.phase); true }).await.unwrap().value;
    let restored = io_import_sqlite_snapshot::<BmpSnapshot>(&dialect, &bytes, SqliteDatabaseLimits::default(), &mut |_| true).await.unwrap().value;
    assert_eq!(restored, snapshot);
    assert!(!phases.iter().any(|phase| matches!(phase, SqliteSnapshotPhase::EncodeNative | SqliteSnapshotPhase::DecodeNative)));
    let database = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    assert_eq!(database.table("bmp_document").unwrap().rows[0].text(1).unwrap(), snapshot.schema);
    assert_eq!(database.table("semio_snapshot").unwrap().rows[0].text(1).unwrap(), "s.stdio.bmp");
}

#[test]
fn sqlite_snapshot_bmp_header_palette_and_grid_pixels_are_semantic() {
    let snapshot = fixture();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(database.table("bmp_palette_entry").unwrap().rows.len(), 2);
    assert_eq!(database.table("bmp_pixel").unwrap().rows.len(), 4);
    assert!(database.tables.iter().flat_map(|table| &table.rows).flat_map(|row| &row.values).all(|value| !matches!(value, SqliteValue::Blob(_))));
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let database = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let restored = BmpSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(restored, snapshot);
    let oracle: serde_json::Value = serde_json::from_str(&pack::json::to_json_string(&protocol::ToValue::to_value(&restored))).unwrap();
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    assert_eq!(oracle, expected);
    let native = <BmpSnapshot as store::ArtifactPack>::decode_pack(&<BmpSnapshot as store::ArtifactPack>::encode_pack(&restored)).unwrap();
    assert_eq!(native, restored);
    for alteration in 0..4 {
        let mut broken = database.clone();
        match alteration {
            0 => broken.table_mut("bmp_pixel").unwrap().rows[0].values[1] = SqliteValue::Integer(999),
            1 => broken.table_mut("bmp_pixel").unwrap().rows[0].values[7] = SqliteValue::Integer(256),
            2 => broken.table_mut("bmp_pixel").unwrap().rows[1].values[2] = SqliteValue::Integer(0),
            _ => { broken.table_mut("bmp_pixel").unwrap().rows.pop(); },
        }
        assert!(BmpSnapshot::from_sqlite_database(&broken, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
    }
    let limits = SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() };
    assert!(BmpSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
}

#[test]
fn sqlite_snapshot_bmp_independent_sql_grid_edits_preserve_rgba_channels() {
    use std::{io::Write, process::{Command, Stdio}};
    let mut snapshot = fixture();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let script = "import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const rows=db.query('SELECT d.row_order,p.x,p.y,p.red,p.alpha FROM bmp_document d JOIN bmp_pixel p ON p.document_id=d.id ORDER BY p.y,p.x').all();if(rows.length!==4||rows[0].row_order!=='top_down'||rows[0].red!==255)throw Error('grid query');db.query('UPDATE bmp_pixel SET alpha=128 WHERE x=1 AND y=0').run();await Bun.write(Bun.stdout,db.serialize());db.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let edited = import_sqlite_database(&output.stdout, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let restored = BmpSnapshot::from_sqlite_database(&edited, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    snapshot.pixels[7] = 128;
    assert_eq!(restored, snapshot);
}
