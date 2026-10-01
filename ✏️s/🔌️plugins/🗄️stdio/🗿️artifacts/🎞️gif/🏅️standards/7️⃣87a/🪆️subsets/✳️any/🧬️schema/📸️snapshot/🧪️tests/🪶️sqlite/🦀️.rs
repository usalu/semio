use super::*;
use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue}, ArtifactSqliteSnapshot};

fn fixture() -> GifSnapshot { pack::json::from_json_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap() }

#[test]
fn sqlite_snapshot_gif87_entity_validation_honors_bounded_cancel_before_palette_ownership(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🧫️fixtures/🪶️sqlite/🚦️control.json")).unwrap();let count=cases["paletteColors"].as_u64().unwrap() as usize;let snapshot=GifSnapshot{gct:Some(GifColorTable{sorted:false,colors:vec![GifRgb{r:1,g:2,b:3};count]}),..GifSnapshot::default()};let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let mut reached=false;
 let result=GifSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |event|{if event.completed==cases["cancelCompleted"].as_u64().unwrap() as usize&&event.total==count{reached=true;false}else{true}},SqliteDatabaseLimits::default()));assert!(reached,"entity scan must offer cancellation before palette ownership");assert!(result.is_err());
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_gif87_owned_io_preserves_full_source_metadata() {
 use semio_framework_os_kernel::io::{ArtifactDialect,io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot}};
 use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 crate::register_sqlite_test_declaration();
 let dialect:ArtifactDialect=crate::GIF_87A_DIALECT.into();let mut snapshot=fixture();snapshot.schema="GIF87 vollständiger Snapshot 世界".into();let mut phases=Vec::new();
 let bytes=io_export_sqlite_snapshot(&dialect,&snapshot,SnapshotEncoding::Binary,SqliteDatabaseLimits::default(),&mut |event|{phases.push(event.phase);true}).await.unwrap().value;
 assert_eq!(io_import_sqlite_snapshot::<GifSnapshot>(&dialect,&bytes,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value,snapshot);
 assert!(!phases.iter().any(|phase|matches!(phase,SqliteSnapshotPhase::EncodeNative|SqliteSnapshotPhase::DecodeNative)));
 let database=import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();assert_eq!(database.table("semio_snapshot").unwrap().rows[0].text(2).unwrap(),"87a");
}

#[test]
fn sqlite_snapshot_gif87_preserves_indexed_images_optional_palettes_and_full_widths() {
 let snapshot = fixture();
 let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
 let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
 let database = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
 assert_eq!(GifSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), snapshot);
 let oracle: serde_json::Value = serde_json::from_str(&pack::json::to_json_string(&protocol::ToValue::to_value(&snapshot))).unwrap();
 assert_eq!(oracle, serde_json::from_str::<serde_json::Value>(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap());
 assert!(database.tables.iter().flat_map(|table| &table.rows).flat_map(|row| &row.values).all(|value| !matches!(value, SqliteValue::Blob(_))));
 for value in [GifSnapshot::default(), GifSnapshot {gct: Some(GifColorTable::default()), ..GifSnapshot::default()}] { let database=value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(); assert_eq!(GifSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(),value); }
 let mut invalid=database.clone(); invalid.table_mut("gif87_pixel").unwrap().rows[0].values[1]=SqliteValue::Integer(999);
 assert!(GifSnapshot::from_sqlite_database(&invalid,&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).is_err());
 let limits=SqliteDatabaseLimits {max_rows:1,..SqliteDatabaseLimits::default()}; assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true,limits)).is_err());
 assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false,SqliteDatabaseLimits::default())).is_err());
}

#[test]
fn sqlite_snapshot_gif87_independent_palette_and_pixel_edits() {
 use std::{io::Write,process::{Command,Stdio}};
 let mut snapshot=fixture(); let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).unwrap();
 let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_| true).unwrap();
 let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const p=db.query('SELECT p.color_index,c.red FROM gif87_pixel p JOIN gif87_image i ON i.id=p.image_id JOIN gif87_global_color c ON c.ordinal=p.color_index WHERE i.ordinal=0 AND p.x=0').get();if(p.color_index!==0||p.red!==255)throw Error('semantic join');db.query('UPDATE gif87_global_color SET red=42 WHERE ordinal=0').run();db.query('UPDATE gif87_pixel SET color_index=1 WHERE image_id=1 AND x=0').run();await Bun.write(Bun.stdout,db.serialize());db.close();";
 let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap(); child.stdin.take().unwrap().write_all(&bytes).unwrap(); let output=child.wait_with_output().unwrap(); assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
 let database=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_| true).unwrap(); snapshot.gct.as_mut().unwrap().colors[0].r=42;snapshot.images[0].indices[0]=1;
 assert_eq!(GifSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).unwrap(),snapshot);
}
