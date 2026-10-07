use crate::standards::v_v3::subsets::any::io::sqlite::snapshot::*;
use semio_framework_os_kernel::{
    sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue},
    ArtifactSqliteSnapshot,
};

fn fixture() -> BmpSnapshot {
    semio_framework_pack_json::from_json_str(include_str!("../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()
}

#[test]
fn sqlite_snapshot_exposes_complete_semantic_layout_without_a_native_carrier() {
    let snapshot = fixture();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).unwrap();
    let table = database.table("bmp_image").unwrap();
    assert_eq!(table.rows.len(), 1);
    assert_eq!(table.rows[0].text(1).unwrap(), snapshot.schema);
    assert_eq!(table.rows[0].text(5).unwrap(), "directRgb24");
    assert_eq!(database.tables.len(), 6);
    assert_eq!(database.tables.iter().flat_map(|table| &table.rows).flat_map(|row| &row.values).filter(|value| matches!(value, SqliteValue::Blob(_))).count(), 0);
    let layout = crate::standards::v_v3::subsets::any::io::bmp_layout(&snapshot).unwrap();
    assert_eq!(database.table("bmp_pixel_sample").unwrap().rows.len(), layout.width as usize * layout.height as usize);
    let file = export_sqlite_database(&database, Default::default(), &mut |_| true).unwrap();
    let database = import_sqlite_database(&file, Default::default(), &mut |_| true).unwrap();
    let restored = BmpSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).unwrap();
    assert_eq!(restored, snapshot);
    assert_eq!(crate::standards::v_v3::subsets::any::io::decode_bmp(&crate::standards::v_v3::subsets::any::io::encode_bmp(&restored).unwrap()).unwrap(), snapshot);
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_typed_file_round_trip_preserves_owned_gap_samples_and_trailer() {
    use {semio_framework_os_kernel::io::io_mechanism::io_export_sqlite_snapshot,semio_framework_os_kernel::io::io_mechanism::io_import_sqlite_snapshot,semio_framework_artifact_reference::ArtifactDialect};
    use store::sqlite_snapshot::SnapshotEncoding;

    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio")
        .label("BMP canonical SQLite")
        .version("0.0.1")
        .package_id("semio:stdio")
        .artifact(crate::declaration(crate::definition().unwrap()).unwrap())
        .try_build()
        .unwrap();
    let dialect = ArtifactDialect { artifact_kind: "s.stdio.bmp".into(), standard: "v3".into(), subset: "*".into() };
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let snapshot = fixture();
        let file = io_export_sqlite_snapshot(&dialect, &snapshot, encoding, Default::default(), &mut |_| true).await.unwrap().value;
        assert_eq!(&file[..16], b"SQLite format 3\0");
        let restored = io_import_sqlite_snapshot::<BmpSnapshot>(&dialect, &file, Default::default(), &mut |_| true).await.unwrap().value;
        assert_eq!(restored, snapshot);
    }
}

#[test]
fn sqlite_snapshot_reconstruction_refuses_incomplete_semantic_grid_atomically() {
    let snapshot = fixture();
    let mut database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).unwrap();
    database.table_mut("bmp_pixel_sample").unwrap().rows.pop().unwrap();
    assert!(BmpSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).is_err());
}

#[test]
fn sqlite_snapshot_projection_and_reconstruction_obey_cancellation_and_ownership_limits() {
    let snapshot = fixture();
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, Default::default())).is_err());
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).unwrap();
    let limits = SqliteDatabaseLimits { max_allocation_bytes: 1, ..Default::default() };
    assert!(BmpSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
}

#[test]
fn sqlite_snapshot_independent_client_edits_one_sample_without_rebuilding_other_native_bytes() {
    use std::{io::Write, process::{Command, Stdio}};

    let snapshot = fixture();
    let layout = crate::standards::v_v3::subsets::any::io::bmp_layout(&snapshot).unwrap();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).unwrap();
    let file = export_sqlite_database(&database, Default::default(), &mut |_| true).unwrap();
    let script = "import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok')throw Error('integrity');if(db.query('PRAGMA foreign_key_check').all().length)throw Error('foreign keys');db.run('UPDATE bmp_pixel_sample SET red=7 WHERE ordinal=1');await Bun.write(Bun.stdout,db.serialize());db.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(&file).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let edited = import_sqlite_database(&output.stdout, Default::default(), &mut |_| true).unwrap();
    let restored = BmpSnapshot::from_sqlite_database(&edited, &mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).unwrap();
    let mut expected = snapshot.clone();
    let crate::schema::snapshot::BmpPixels::Direct { samples } = &mut expected.image.pixels else { panic!("direct fixture"); };
    samples[1].red = 7;
    assert_eq!(restored, expected);
    let expected = crate::standards::v_v3::subsets::any::io::encode_bmp(&expected).unwrap();
    let (_, _, independent) = semio_s_artifact_stdio_bmp_test_oracle::standards::v_v3::subsets::any::oracle_visual_rgba8(&expected).unwrap();
    assert_eq!(crate::schema::operations::bmp_rgba8_preview(&restored).unwrap(), independent);
}

#[path = "🧬️semantic/🦀️.rs"]
mod semantic;
