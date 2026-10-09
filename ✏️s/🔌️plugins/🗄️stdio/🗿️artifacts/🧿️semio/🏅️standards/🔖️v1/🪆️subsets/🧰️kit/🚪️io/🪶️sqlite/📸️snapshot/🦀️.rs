//! 🧰️ Kit types, designs, placed pieces, connections, typed child handles and history pins.
use semio_framework_os_kernel::sqlite_snapshot::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow,RowIndex};
use crate::standards::v1::subsets::kit::schema::snapshot::{SemioKitSnapshot,SemioKitType,SemioKitDesign,SemioKitPiece,SemioKitConnection};
use crate::standards::v1::subsets::base::schema::geometry::{SemioTransform};
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use crate::standards::v1::subsets::base::schema::{geometry::{SemioPoint3,SemioQuaternion},child::validate_semio_child_identity};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,reconstruct_text},validate_sqlite_database_schema,SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase}};
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
fn kit_rows<'a>(db:&'a SqliteDatabase,table:&str,columns:usize,control:&mut SqliteSnapshotControl<'_>)->Result<RowIndex<'a>,ValueError>{RowIndex::new(db,table,columns,float_columns(table),control,"invalid Semio kit row identity or columns")}
/// 🔗️ Constructs one consumed descriptor inside its actual first-party retirement guard.
fn kit_reference(references:&mut RowIndex<'_>,id:i64,control:&mut SqliteSnapshotControl<'_>)->Result<Owned<semio_framework_artifact_reference::ArtifactRef>,ValueError>{
 let row=references.take(id,control)?.ok_or_else(||kit_invalid("dangling or multiply owned Semio kit reference"))?;
 let mut target=Owned::new(semio_framework_artifact_reference::ArtifactRef{artifact_id:String::new(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:String::new(),standard:String::new(),subset:String::new()}});
 target.get_mut().artifact_id=reconstruct_text(control,row.text(1)?)?;target.get_mut().dialect.artifact_kind=reconstruct_text(control,row.text(2)?)?;target.get_mut().dialect.standard=reconstruct_text(control,row.text(3)?)?;target.get_mut().dialect.subset=reconstruct_text(control,row.text(4)?)?;Ok(target)
}
/// 🔢️ Accepts only the original canonical unsigned decimal spelling without formatting allocation.
fn kit_blob_size(text:&str)->Result<u64,ValueError>{
 if text.is_empty()||text.len()>20||(text.len()>1&&text.as_bytes()[0]==b'0')||!text.bytes().all(|byte|byte.is_ascii_digit()){return Err(kit_invalid("invalid Semio kit blob identity or size"))}text.parse::<u64>().map_err(|error|kit_invalid(&error.to_string()))
}
/// 📦️ Restores a consumed blob descriptor with guarded partial literal fields.
fn kit_blob(blobs:&mut RowIndex<'_>,id:i64,control:&mut SqliteSnapshotControl<'_>)->Result<Owned<store::BlobRef>,ValueError>{
 let row=blobs.take(id,control)?.ok_or_else(||kit_invalid("dangling or multiply owned Semio kit blob pin"))?;let mut blob=Owned::new(store::BlobRef{hash:String::new(),size:kit_blob_size(row.text(2)?)?,media_type:String::new()});blob.get_mut().hash=reconstruct_text(control,row.text(1)?)?;blob.get_mut().media_type=reconstruct_text(control,row.text(3)?)?;Ok(blob)
}
/// 🪆️ Restores paid ordered handles while protecting each target before later child identity copying.
fn children<S:Send+'static>(database:&SqliteDatabase,table:&str,subset:&str,references:&mut RowIndex<'_>,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<store::ArtifactChild<S>>,ValueError>{
 let rows=kit_rows(database,table,5,control)?;let order=rows.ordered(2,control,"relationship ordinals must be contiguous and unique")?;rows.unique_text(rows.indices(),3,control,"invalid Semio kit child ownership or identity")?;
 let mut result=Owned::new(semio_framework_os_kernel::sqlite_snapshot::transfer::reserve::<store::ArtifactChild<S>>(order.len(),control)?);
 for index in order{let row=rows.row(index)?;if row.integer(1)?!=1{return Err(kit_invalid("invalid Semio kit child ownership or identity"))}let mut target=kit_reference(references,row.integer(4)?,control)?;validate_semio_child_identity(row.text(3)?,target.get_mut(),subset).map_err(|error|kit_invalid(&error))?;let mut child=Owned::new(store::ArtifactChild::<S>::new(String::new(),target.take()));child.get_mut().child_id=reconstruct_text(control,row.text(3)?)?;result.get_mut().push(child.take());}
 control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,result.get_mut().len(),rows.len())?;Ok(result.take())
}
impl ArtifactSqliteSnapshot for SemioKitSnapshot{
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(self));}
fn encode_sqlite_snapshot_native(&self,encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io::IoPayload,ValueError>{crate::standards::v1::subsets::kit::io::sqlite::snapshot::native_encoding::encode(self,encoding,control,native_owner)}
fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{crate::standards::v1::subsets::kit::io::sqlite::snapshot::native_decoding::decode(payload,control,native_control)}
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
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>, declared_schema: &str)->Result<Self,ValueError> {
control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;semio_framework_os_kernel::sqlite_snapshot::validate_sqlite_database_schema_controlled(database,declared_schema,SqliteSnapshotPhase::ReconstructSnapshot,control)?;let document=single_float_row(database,"semio_kit_document")?;identity(document,2)?;if document.rowid!=1{return Err(kit_invalid("invalid Semio kit document identifier"))}
let types=kit_rows(database,"semio_kit_type",6,control)?;let designs=kit_rows(database,"semio_kit_design",5,control)?;let type_order=types.ordered(2,control,"relationship ordinals must be contiguous and unique")?;let design_order=designs.ordered(2,control,"relationship ordinals must be contiguous and unique")?;
for rows in [&types,&designs]{rows.unique_text(rows.indices(),3,control,"invalid Semio kit native ownership or identity")?;for &index in rows.indices(){if rows.row(index)?.integer(1)?!=1{return Err(kit_invalid("invalid Semio kit native ownership or identity"))}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,rows.len())?;}}
let mut pieces=kit_rows(database,"semio_kit_piece",15,control)?;let mut connections=kit_rows(database,"semio_kit_connection",8,control)?;
for rows in [&pieces,&connections]{for &index in rows.indices(){if designs.get(rows.row(index)?.integer(1)?,control)?.is_none(){return Err(kit_invalid("invalid Semio kit relationship owner or identity"))}}}
let piece_order=pieces.grouped_by(2,control,"Semio kit ordinals must be contiguous",|row|Ok((0,Some(row.integer(1)?))))?;let connection_order=connections.grouped_by(2,control,"Semio kit ordinals must be contiguous",|row|Ok((0,Some(row.integer(1)?))))?;
let mut snapshot=Owned::new(Self{schema:String::new(),types:Vec::new(),designs:Vec::new(),objects:Vec::new(),models:Vec::new(),properties:None,representations:Vec::new()});
snapshot.get_mut().types=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(type_order.len(),control)?;
for index in type_order{let row=types.row(index)?;let mut native=Owned::new(SemioKitType{id:String::new(),name:String::new(),category:String::new()});native.get_mut().id=reconstruct_text(control,row.text(3)?)?;native.get_mut().name=reconstruct_text(control,row.text(4)?)?;native.get_mut().category=reconstruct_text(control,row.text(5)?)?;snapshot.get_mut().types.push(native.take());}
snapshot.get_mut().designs=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(design_order.len(),control)?;
for index in design_order{
 let row=designs.row(index)?;let mut design=Owned::new(SemioKitDesign{id:String::new(),name:String::new(),pieces:Vec::new(),connections:Vec::new()});design.get_mut().id=reconstruct_text(control,row.text(3)?)?;design.get_mut().name=reconstruct_text(control,row.text(4)?)?;
 let range=pieces.range_by(&piece_order,(0,Some(row.rowid)),control,|row|Ok((0,Some(row.integer(1)?))))?;pieces.unique_text(&piece_order[range.clone()],3,control,"duplicate Semio kit piece native identifier")?;design.get_mut().pieces=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(range.len(),control)?;
 for ordinal in range{let piece=pieces.take_index(piece_order[ordinal],control)?.ok_or_else(||kit_invalid("duplicate Semio kit piece ownership"))?;let mut native=Owned::new(SemioKitPiece{id:String::new(),type_id:String::new(),transform:SemioTransform::default()});native.get_mut().id=reconstruct_text(control,piece.text(3)?)?;let kind=types.get(piece.integer(4)?,control)?.ok_or_else(||kit_invalid("dangling Semio kit piece type"))?;native.get_mut().type_id=reconstruct_text(control,kind.text(3)?)?;native.get_mut().transform=SemioTransform{translation:SemioPoint3{x:piece.real(5)?,y:piece.real(6)?,z:piece.real(7)?},rotation:SemioQuaternion{x:piece.real(8)?,y:piece.real(9)?,z:piece.real(10)?,w:piece.real(11)?},scale:SemioPoint3{x:piece.real(12)?,y:piece.real(13)?,z:piece.real(14)?}};design.get_mut().pieces.push(native.take());}
 let range=connections.range_by(&connection_order,(0,Some(row.rowid)),control,|row|Ok((0,Some(row.integer(1)?))))?;connections.unique_text(&connection_order[range.clone()],3,control,"duplicate Semio kit connection native identifier")?;design.get_mut().connections=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(range.len(),control)?;
 for ordinal in range{let connection=connections.take_index(connection_order[ordinal],control)?.ok_or_else(||kit_invalid("duplicate Semio kit connection ownership"))?;let connecting=pieces.get(connection.integer(4)?,control)?.ok_or_else(||kit_invalid("dangling Semio kit connecting piece"))?;let connected=pieces.get(connection.integer(6)?,control)?.ok_or_else(||kit_invalid("dangling Semio kit connected piece"))?;if connecting.integer(1)?!=row.rowid||connected.integer(1)?!=row.rowid{return Err(kit_invalid("Semio kit connection crosses its owning design"))}
  let mut native=Owned::new(SemioKitConnection{id:String::new(),connecting_piece_id:String::new(),connecting_port:String::new(),connected_piece_id:String::new(),connected_port:String::new()});native.get_mut().id=reconstruct_text(control,connection.text(3)?)?;native.get_mut().connecting_piece_id=reconstruct_text(control,connecting.text(3)?)?;native.get_mut().connecting_port=reconstruct_text(control,connection.text(5)?)?;native.get_mut().connected_piece_id=reconstruct_text(control,connected.text(3)?)?;native.get_mut().connected_port=reconstruct_text(control,connection.text(7)?)?;design.get_mut().connections.push(native.take());
 }snapshot.get_mut().designs.push(design.take());
}
let mut references=kit_rows(database,"semio_kit_reference",5,control)?;for &index in references.indices(){let row=references.row(index)?;for column in 1..5{row.text(column)?;}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,references.len())?;}
snapshot.get_mut().objects=children::<SemioObjectSnapshot>(database,"semio_kit_object_child","object",&mut references,control)?;snapshot.get_mut().models=children::<SemioModelSnapshot>(database,"semio_kit_model_child","model",&mut references,control)?;
let table=database.table("semio_kit_value_child")?;if !table.rows.is_empty(){let row=SqliteRow::new(table.single_row()?,float_columns("semio_kit_value_child"))?;identity(row,4)?;if row.integer(1)?!=1{return Err(kit_invalid("invalid Semio kit value child owner"))}let mut target=kit_reference(&mut references,row.integer(3)?,control)?;validate_semio_child_identity(row.text(2)?,target.get_mut(),"value").map_err(|error|kit_invalid(&error))?;let mut child=Owned::new(store::ArtifactChild::<SemioValueSnapshot>::new(String::new(),target.take()));child.get_mut().child_id=reconstruct_text(control,row.text(2)?)?;snapshot.get_mut().properties=Some(child.take());}
let mut blobs=kit_rows(database,"semio_kit_blob",4,control)?;for &index in blobs.indices(){let row=blobs.row(index)?;row.text(1)?;kit_blob_size(row.text(2)?)?;row.text(3)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,blobs.len())?;}
let representations=kit_rows(database,"semio_kit_representation",8,control)?;let representation_order=representations.ordered(2,control,"relationship ordinals must be contiguous and unique")?;snapshot.get_mut().representations=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(representation_order.len(),control)?;
for index in representation_order{
 let row=representations.row(index)?;if row.integer(1)?!=1{return Err(kit_invalid("invalid Semio kit representation ownership or identity"))}let checkpoint=row.optional_text(6)?;let blob=if row.is_null(7)?{None}else{Some(row.integer(7)?)};
 let mut pin=Owned::new(store::LinkPin::Head);*pin.get_mut()=match row.text(5)?{
  "head" if checkpoint.is_none()&&blob.is_none()=>store::LinkPin::Head,
  "checkpoint" if blob.is_none()=>store::LinkPin::Checkpoint{id:reconstruct_text(control,checkpoint.ok_or_else(||kit_invalid("missing Semio kit checkpoint pin"))?)?},
  "snapshot" if checkpoint.is_none()=>{let mut blob=kit_blob(&mut blobs,blob.ok_or_else(||kit_invalid("missing Semio kit snapshot pin"))?,control)?;store::LinkPin::Snapshot{blob:blob.take()}},
  _=>return Err(kit_invalid("invalid Semio kit link pin shape"))
 };
 let mut target=kit_reference(&mut references,row.integer(4)?,control)?;let mut link=Owned::new(store::ArtifactLink{target:target.take(),role:String::new(),pin:pin.take()});let kind=types.get(row.integer(3)?,control)?.ok_or_else(||kit_invalid("dangling Semio kit representation type role"))?;link.get_mut().role=reconstruct_text(control,kind.text(3)?)?;snapshot.get_mut().representations.push(link.take());
}
if references.remaining()!=0||blobs.remaining()!=0||pieces.remaining()!=0||connections.remaining()!=0{return Err(kit_invalid("orphan Semio kit reference or blob pin descriptor"))}
snapshot.get_mut().schema=reconstruct_text(control,document.text(1)?)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;Ok(snapshot.take())

    }
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
