//! 📜️ Preflight scans the real checkpoint clocks, authors, members and alternatives.
use super::*;
use crate as store;
#[path="../../../../../../🪐️space/🗿️artifacts/🪶️sqlite/📏️preflight/🦀️.rs"]mod census;
pub(super)fn check(value:&SpaceHistorySnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 let total=rows(value,control,SqliteSnapshotPhase::EncodeNative)?;let mut c=census::Census::new(control,SpaceHistorySnapshot::SQLITE_SCHEMA,total)?;
 c.entity(1)?;c.labels(&["checkpoints","alternatives","activeAlternativeId"])?;c.optional(value.active_alternative_id.as_deref())?;
 for row in &value.checkpoints{
  c.entity(3)?;c.labels(&["id","parentId","message","authors","timestamp","members"])?;c.text(&row.id)?;c.optional(row.parent_id.as_deref())?;c.text(&row.message)?;
  c.entity(7)?;c.labels(&["actor","physical_ms","logical"])?;
  for author in &row.authors{c.entity(3)?;c.labels(&["id","name","avatar"])?;c.text(&author.id)?;c.text(&author.name)?;c.optional(author.avatar.as_deref())?;}
  for pin in &row.members{c.entity(3)?;c.labels(&["documentId","checkpointId","alternativeId"])?;c.text(&pin.document_id)?;c.text(&pin.checkpoint_id)?;c.text(&pin.alternative_id)?;}
 }
 for row in &value.alternatives{
  c.entity(3)?;c.labels(&["id","name","checkpointIds"])?;c.text(&row.id)?;c.text(&row.name)?;
  for pin in &row.checkpoint_ids{c.entity(3)?;c.text(pin)?;}
 }
 c.finish(encoding,None)
}

