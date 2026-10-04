//! ⏱️ Actual row-tail consumption remains cancellable after its paid frontier initialization.
use super::*;
#[test]
fn sqlite_snapshot_semantic_row_tail_processing_cancels_inside_owned_frontier(){
 let neutral:serde_json::Value=serde_json::from_str(include_str!("../../../../🧫️fixtures/🪶️sqlite/🧬️semantic/🔣️.json")).unwrap();
 let demand=&neutral["rowTailProgress"];let height=demand["height"].as_u64().unwrap() as usize;let cadence=demand["checkpointCadence"].as_u64().unwrap() as usize;
 assert_eq!(height,257);assert_eq!(cadence,256);assert_eq!(demand["width"],1);assert_eq!(demand["bitsPerPixel"],1);
 let mut bytes=vec![0u8;62+height*4];bytes[..2].copy_from_slice(b"BM");let length=bytes.len() as u32;bytes[2..6].copy_from_slice(&length.to_le_bytes());bytes[10..14].copy_from_slice(&62u32.to_le_bytes());bytes[14..18].copy_from_slice(&40u32.to_le_bytes());bytes[18..22].copy_from_slice(&1i32.to_le_bytes());bytes[22..26].copy_from_slice(&(-(height as i32)).to_le_bytes());bytes[26..28].copy_from_slice(&1u16.to_le_bytes());bytes[28..30].copy_from_slice(&1u16.to_le_bytes());bytes[34..38].copy_from_slice(&((height*4)as u32).to_le_bytes());bytes[54..62].copy_from_slice(&[0,0,0,0,255,255,255,0]);
 for y in 0..height{bytes[62+y*4]=(y%128)as u8;}
 let snapshot=BmpSnapshot{schema:"stdio.bmp".into(),bytes};let database=project(&snapshot);let tails=&database.table("bmp_row_tail_bits").unwrap().rows;
 assert_eq!(tails.len(),height);for(y,row)in tails.iter().enumerate(){assert_eq!(row.integer(2).unwrap(),y as i64);assert_eq!(row.integer(3).unwrap(),(y%128)as i64);}
 assert_eq!(restore(&database),snapshot);
 let mut completed_frontiers=0usize;let mut canceled_at=None;
 let mut progress=|event:semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress|{
  if event.phase==semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase::ReconstructSnapshot&&event.total==height{
   if event.completed==height{completed_frontiers+=1;}
   if completed_frontiers>=2&&event.completed==cadence{canceled_at=Some(event);return false;}
  }true
 };
 let error=BmpSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut progress,Default::default())).unwrap_err();
 assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::Canceled);assert!(canceled_at.is_some());assert_eq!(completed_frontiers,2);
}
