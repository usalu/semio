//! 🧰️ Kit types, designs, placed pieces, connections, typed child handles and history pins.
use semio_framework_os_kernel::sqlite_snapshot::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;

use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow};
use crate::standards::v1::subsets::kit::schema::snapshot::{SemioKitSnapshot,SemioKitType,SemioKitDesign,SemioKitPiece,SemioKitConnection};
use crate::standards::v1::subsets::base::schema::geometry::{SemioTransform};
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use crate::standards::v1::subsets::base::schema::{geometry::{SemioPoint3,SemioQuaternion},child::validate_semio_child_identity};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter},validate_sqlite_database_schema,SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase}};
#[path="🧮️semantic/🦀️.rs"]
mod semantic;
/// 🔍️ Resolves a retained, paid identity frontier with bounded text comparisons.
fn identifier(ids:&[(&str,i64)],id:&str,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
 let mut lo=0;let mut hi=ids.len();while lo<hi{let mid=lo+(hi-lo)/2;match out.compare_text(ids[mid].0,id)?{std::cmp::Ordering::Less=>lo=mid+1,std::cmp::Ordering::Greater=>hi=mid,std::cmp::Ordering::Equal=>return Ok(ids[mid].1)}}Err(ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio Kit relational identity"))
}
/// 🪪️ Admits literal identities using paid borrowed entries and cancellable comparisons.
fn frontier<'a,T>(values:&'a[T],start:usize,id:impl Fn(&'a T)->&'a str,out:&mut RowWriter<'_,'_>)->Result<Vec<(&'a str,i64)>,ValueError>{
 let phase=out.phase();let mut ids=out.allocate_frontier(values.len())?;for(ordinal,value)in values.iter().enumerate(){out.checkpoint()?;let index=start.checked_add(ordinal).and_then(|value|value.checked_add(1)).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio Kit relational identity overflow"))?;ids.push((id(value),number(index)?));}
 out.sort_frontier(&mut ids,|a,b,control|semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(a.0,b.0,phase,control))?;
 for pair in ids.windows(2){if out.compare_text(pair[0].0,pair[1].0)?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio Kit literal identity"))}}Ok(ids)
}
/// 🧾️ Projects the literal four-text target identity without an opaque carrier.
fn reference(target:&semio_framework_artifact_reference::ArtifactRef,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
 out.insert("semio_kit_reference",&[Cell::Text(&target.artifact_id),Cell::Text(&target.dialect.artifact_kind),Cell::Text(&target.dialect.standard),Cell::Text(&target.dialect.subset)])
}
/// 🧒️ Projects ordered child aliases independently from their persisted target identities.
fn project_children<S>(children:&[store::ArtifactChild<S>],table:&str,subset:&str,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let _ids=frontier(children,0,|child|child.child_id.as_str(),out)?;
 for(ordinal,child)in children.iter().enumerate(){let target=reference(&child.target,out)?;out.insert(table,&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&child.child_id),Cell::Integer(target)])?;validate_semio_child_identity(&child.child_id,&child.target,subset).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))?;}Ok(())
}
/// 🔢️ Borrows the canonical unsigned decimal size from fixed stack storage.
fn decimal(mut value:u64,scratch:&mut[u8;20])->&str{
 let mut at=scratch.len();loop{at-=1;scratch[at]=b'0'+(value%10)as u8;value/=10;if value==0{break}}std::str::from_utf8(&scratch[at..]).expect("ASCII unsigned decimal")
}
/// 🫳️ Visits the actual Kit catalog, placements, child references and history pins through one row writer.
pub(crate)fn visit_rows(snapshot:&SemioKitSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let types=frontier(&snapshot.types,0,|kind|kind.id.as_str(),out)?;
 let _designs=frontier(&snapshot.designs,0,|design|design.id.as_str(),out)?;
 out.insert_key("semio_kit_document",1,&[Cell::Text(&snapshot.schema)])?;
 for(ordinal,kind)in snapshot.types.iter().enumerate(){out.insert("semio_kit_type",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&kind.id),Cell::Text(&kind.name),Cell::Text(&kind.category)])?;}
 let mut piece_start=0usize;
 for(ordinal,design)in snapshot.designs.iter().enumerate(){
  let design_id=out.insert("semio_kit_design",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&design.id),Cell::Text(&design.name)])?;
  let pieces=frontier(&design.pieces,piece_start,|piece|piece.id.as_str(),out)?;
  piece_start=piece_start.checked_add(design.pieces.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio Kit piece extent overflow"))?;
  let _connections=frontier(&design.connections,0,|connection|connection.id.as_str(),out)?;
  for(ordinal,piece)in design.pieces.iter().enumerate(){
   let t=piece.transform;let kind=identifier(&types,&piece.type_id,out)?;
   out.insert_float("semio_kit_piece",&[Cell::Integer(design_id),Cell::Integer(number(ordinal)?),Cell::Text(&piece.id),Cell::Integer(kind),Cell::Real(t.translation.x),Cell::Real(t.translation.y),Cell::Real(t.translation.z),Cell::Real(t.rotation.x),Cell::Real(t.rotation.y),Cell::Real(t.rotation.z),Cell::Real(t.rotation.w),Cell::Real(t.scale.x),Cell::Real(t.scale.y),Cell::Real(t.scale.z)],float_columns("semio_kit_piece"))?;
  }
  for(ordinal,connection)in design.connections.iter().enumerate(){
   let source=identifier(&pieces,&connection.connecting_piece_id,out)?;let target=identifier(&pieces,&connection.connected_piece_id,out)?;
   out.insert("semio_kit_connection",&[Cell::Integer(design_id),Cell::Integer(number(ordinal)?),Cell::Text(&connection.id),Cell::Integer(source),Cell::Text(&connection.connecting_port),Cell::Integer(target),Cell::Text(&connection.connected_port)])?;
  }
 }
 project_children(&snapshot.objects,"semio_kit_object_child","object",out)?;
 project_children(&snapshot.models,"semio_kit_model_child","model",out)?;
 if let Some(child)=&snapshot.properties{let target=reference(&child.target,out)?;out.insert("semio_kit_value_child",&[Cell::Integer(1),Cell::Text(&child.child_id),Cell::Integer(target)])?;validate_semio_child_identity(&child.child_id,&child.target,"value").map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))?;}
 for(ordinal,link)in snapshot.representations.iter().enumerate(){
  let target=reference(&link.target,out)?;let kind=identifier(&types,&link.role,out)?;
  let(pin,checkpoint,blob)=match &link.pin{
   store::LinkPin::Head=>("head",Cell::Null,Cell::Null),
   store::LinkPin::Checkpoint{id}=>("checkpoint",Cell::Text(id),Cell::Null),
   store::LinkPin::Snapshot{blob}=>{let mut scratch=[0u8;20];let size=decimal(blob.size,&mut scratch);let id=out.insert("semio_kit_blob",&[Cell::Text(&blob.hash),Cell::Text(size),Cell::Text(&blob.media_type)])?;("snapshot",Cell::Null,Cell::Integer(id))}
  };
  out.insert("semio_kit_representation",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Integer(kind),Cell::Integer(target),Cell::Text(pin),checkpoint,blob])?;
 }Ok(())
}
/// 🎟️ Admits all typed Kit cells before native forecasting or materialization.
pub(crate)fn admit_values(snapshot:&SemioKitSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}
/// 🏛️ Admits the exact authored table and column layout before native ownership.
pub(crate)fn admit_layout(limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
/// 📦️ Counts the actual native primitive cells before typed ownership.
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
/// 📝️ Counts the actual native primitive cells before typed ownership.
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}

fn number(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string()))}
fn identity<'a>(row:impl std::borrow::Borrow<SqliteRow<'a>>,columns:usize)->Result<(),ValueError>{let row=*row.borrow();if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio kit row identity or columns"))}else{Ok(())}}
/// 🚫️ Reports an authored Kit relationship refusal.
fn kit_invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
/// 🗃️ Binds declared Kit logical and IEEE columns to paid borrowed source positions.
/// 🔢️ Accepts only the original canonical unsigned decimal spelling without formatting allocation.
fn kit_blob_size(text:&str)->Result<u64,ValueError>{
 if text.is_empty()||text.len()>20||(text.len()>1&&text.as_bytes()[0]==b'0')||!text.bytes().all(|byte|byte.is_ascii_digit()){return Err(kit_invalid("invalid Semio kit blob identity or size"))}text.parse::<u64>().map_err(|error|kit_invalid(&error.to_string()))
}
impl ArtifactSqliteSnapshot for SemioKitSnapshot{
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(self));}
fn encode_sqlite_snapshot_native(&self,encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io::IoPayload,ValueError>{crate::standards::v1::subsets::kit::io::sqlite::snapshot::native_encoding::encode(self,encoding,control,native_owner)}
fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{crate::standards::v1::subsets::kit::io::sqlite::snapshot::native_decoding::decode(payload,control,native_control.native())}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{(|| -> Result<(),ValueError>{admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut b=Bound::file_only("",control)?;self.native_fields(&mut b)?;b.finish()})()}

fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{(|| -> Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if !SemioKitSnapshot::admits_dialect_parts(&dialect.artifact_kind,&dialect.standard,&dialect.subset){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_kit_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})().map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {semantic::layout(control.limits())?;Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA)}
}

impl SemioKitSnapshot {
    /// 🧩️ Projects owned semantic fields with typed relational refusals.
    pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
semantic::layout(control.limits())?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut out)?;out.finish()
}

    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{reconstruction::reconstruct(database,control,declared_schema)}
}

fn float_columns(table:&str)->&'static [FloatColumn]{match table{"semio_kit_piece"=>&[FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10),FloatColumn::Binary64(11),FloatColumn::Binary64(12),FloatColumn::Binary64(13),FloatColumn::Binary64(14)],_=>&[]}}
fn single_float_row<'a>(db:&'a SqliteDatabase,table:&str)->Result<SqliteRow<'a>,ValueError>{SqliteRow::new(db.table(table)?.single_row()?,float_columns(table))}

impl SemioKitSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.entities(self.types.len())?;for ty in &self.types{b.text(&ty.id)?;b.text(&ty.name)?;b.text(&ty.category)?;}b.entities(self.designs.len())?;for design in &self.designs{b.text(&design.id)?;b.text(&design.name)?;b.entities(design.pieces.len())?;for piece in &design.pieces{b.text(&piece.id)?;b.text(&piece.type_id)?;b.scalars(10)?;}b.entities(design.connections.len())?;for connection in &design.connections{b.text(&connection.id)?;b.text(&connection.connecting_piece_id)?;b.text(&connection.connecting_port)?;b.text(&connection.connected_piece_id)?;b.text(&connection.connected_port)?;}}for child in &self.objects{crate::standards::v1::subsets::object::io::sqlite::snapshot::native_child(child,b)?;}for child in &self.models{crate::standards::v1::subsets::object::io::sqlite::snapshot::native_child(child,b)?;}if let Some(child)=&self.properties{crate::standards::v1::subsets::object::io::sqlite::snapshot::native_child(child,b)?;}b.entities(self.representations.len())?;for link in &self.representations{b.text(&link.role)?;crate::standards::v1::subsets::object::io::sqlite::snapshot::native_reference(&link.target,b)?;match &link.pin{store::LinkPin::Head=>{},store::LinkPin::Checkpoint{id}=>b.text(id)?,store::LinkPin::Snapshot{blob}=>{b.text(&blob.hash)?;b.text(&blob.media_type)?;b.scalars(1)?;}}}Ok(())}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;

#[path="💰️reconstruction/🦀️.rs"]mod reconstruction;
