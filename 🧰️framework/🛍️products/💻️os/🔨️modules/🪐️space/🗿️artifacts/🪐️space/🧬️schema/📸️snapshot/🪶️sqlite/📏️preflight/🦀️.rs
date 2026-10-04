//! 🪐️ Preflight borrows every actual space field through fixed-depth typed loops.
use super::*;
#[path="../../../../../🪶️sqlite/📏️preflight/🦀️.rs"]mod census;
pub(super)fn check(value:&SpaceSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 let total=rows(value)?;let mut c=census::Census::new(control,SpaceSnapshot::SQLITE_SCHEMA,total)?;
 c.entity(1)?;c.labels(&["space","schema","name","kind","visibility","users","collections","programs","extensions"])?;
 c.text(&value.schema)?;c.text(&value.name)?;
 c.text(match value.kind{SpaceKind::Atelier=>"atelier",SpaceKind::Studio=>"studio",SpaceKind::Archive=>"archive"})?;
 c.text(match value.visibility{SpaceVisibility::Private=>"private",SpaceVisibility::Public=>"public"})?;
 for row in &value.users{c.entity(3)?;c.labels(&["id","name","avatar","role"])?;c.text(&row.id)?;c.text(&row.name)?;c.optional(row.avatar.as_deref())?;c.text(row.role.as_str())?;}
 for row in &value.collections{c.entity(3)?;c.labels(&["id","name","document-id"])?;c.text(&row.id)?;c.text(&row.name)?;c.text(&row.document_id)?;}
 for row in &value.programs{c.entity(3)?;c.text(row)?;}
 for row in &value.extensions{c.entity(4)?;c.labels(&["extension-id","version","source-uri","package-hash","enabled"])?;c.text(&row.extension_id)?;c.text(&row.version)?;c.text(&row.source_uri)?;c.text(&row.package_hash)?;}
 c.finish(encoding,Some(SpaceSnapshot::__DSL_ENVELOPE_ID))
}

