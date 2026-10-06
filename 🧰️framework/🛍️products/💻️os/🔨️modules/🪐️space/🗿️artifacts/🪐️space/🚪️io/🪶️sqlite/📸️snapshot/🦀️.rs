//! 🪐️ Exact typed space state through handwritten relational entities and controlled native records.
#[path="./📏️preflight/🦀️.rs"]mod preflight;
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::{SpaceSnapshot,SpaceKind,SpaceVisibility,SpaceRole,SpaceUser,CollectionRef,InstalledExtension,S_SPACE_SCHEMA};
use semio_framework_os_kernel as store;
use store::ArtifactSqliteSnapshot;
use store::sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,validate_sqlite_database_schema,artifact::{RowWriter,Cell}};
#[path="../../../../../🪶️sqlite/🦀️.rs"]mod fields;
/// 🚪️ The explicitly authored native envelope-version coordinate for this builtin owner.
pub const SQLITE_SNAPSHOT_DIALECT:store::os_io::Dialect=store::os_io::Dialect{artifact_kind:S_SPACE_SCHEMA,standard:store::os_io::StandardId("1"),subset:store::os_io::SubsetId("*")};
/// 📣️ Registers the real bare native factory and its owned SQLite capability atomically.
pub fn register_sqlite_snapshot()->Result<(),store::os_io::ArtifactAssemblyRegistryError>{store::os_io::register_native_snapshot_codec(SQLITE_SNAPSHOT_DIALECT,store::ArtifactCodec::bare::<SpaceSnapshot,crate::SpaceMutation>(S_SPACE_SCHEMA))}
fn rows(value:&SpaceSnapshot)->Result<usize,ValueError>{let mut count=1;for length in[value.users.len(),value.collections.len(),value.programs.len(),value.extensions.len()]{count=fields::add(count,length)?;}Ok(count)}
fn schema(control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{if SpaceSnapshot::SQLITE_SCHEMA.len()>control.limits().max_schema_bytes{Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "space authored schema byte limit exceeded"))}else{Ok(())}}
#[path="./🛂️admission/🦀️.rs"]pub(crate)mod admission;
fn write_rows(value:&SpaceSnapshot,total:usize,p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{  let kind=match value.kind{SpaceKind::Atelier=>"atelier",SpaceKind::Studio=>"studio",SpaceKind::Archive=>"archive"};let visibility=match value.visibility{SpaceVisibility::Private=>"private",SpaceVisibility::Public=>"public"};
  let document=p.insert("space_document",&[Cell::Text(&value.schema),Cell::Text(&value.name),Cell::Text(kind),Cell::Text(visibility)])?;p.checkpoint_total(total)?;
  for(index,row)in value.users.iter().enumerate(){p.insert("space_user",&[Cell::Integer(document),Cell::Integer(fields::ordinal(index)?),Cell::Text(&row.id),Cell::Text(&row.name),row.avatar.as_deref().map(Cell::Text).unwrap_or(Cell::Null),Cell::Text(row.role.as_str())])?;p.checkpoint_total(total)?;}
  for(index,row)in value.collections.iter().enumerate(){p.insert("space_collection",&[Cell::Integer(document),Cell::Integer(fields::ordinal(index)?),Cell::Text(&row.id),Cell::Text(&row.name),Cell::Text(&row.document_id)])?;p.checkpoint_total(total)?;}
  for(index,row)in value.programs.iter().enumerate(){p.insert("space_program",&[Cell::Integer(document),Cell::Integer(fields::ordinal(index)?),Cell::Text(row)])?;p.checkpoint_total(total)?;}
  for(index,row)in value.extensions.iter().enumerate(){p.insert("space_extension",&[Cell::Integer(document),Cell::Integer(fields::ordinal(index)?),Cell::Text(&row.extension_id),Cell::Text(&row.version),Cell::Text(&row.source_uri),Cell::Text(&row.package_hash),Cell::Integer(i64::from(row.enabled))])?;p.checkpoint_total(total)?;}Ok(())}
fn semantic(value:&SpaceSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{schema(control)?;let total=rows(value)?;control.check_rows(total)?;let mut p=RowWriter::borrowed(control,phase)?;write_rows(value,total,&mut p)?;p.finish_borrowed()}
impl store::ArtifactSqliteSnapshot for SpaceSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn preflight_sqlite_snapshot_encoding(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{preflight::check(self,encoding,control)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{
  semantic(self,SqliteSnapshotPhase::EncodeNative,control)?;
  store::encode_sqlite_snapshot_record_native(encoding,Self::__DSL_ENVELOPE_ID,Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)
 }
 fn decode_sqlite_snapshot_native(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,0)?;schema(control)?;let limits=control.limits();
  let maximum=control.reconstruction_remaining_bytes()?.min(control.allocation_remaining_bytes());
  let(result,owned)={
   let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|control.checkpoint(event.phase,event.completed,event.total).is_ok();
   let mut child=SqliteSnapshotControl::new(&mut callback,store::sqlite_snapshot::SqliteDatabaseLimits{max_allocation_bytes:maximum,..limits});
   let result=store::decode_sqlite_snapshot_record_native(payload,Self::__DSL_ENVELOPE_ID,Self::__dsl_spec_producer(),|record,native|{
    admission::record(record,native,limits)?;Self::__dsl_from_record_controlled(record,native)
   },&mut child);
   (result,maximum-child.allocation_remaining_bytes())
  };
  control.admit_allocation_bytes(owned)?;control.admit_reconstruction_bytes(owned)?;result
 }
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{let total=rows(self)?;control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,total)?;schema(control)?;control.check_rows(total)?;let mut p=RowWriter::new(Self::SQLITE_SCHEMA,control)?;write_rows(self,total,&mut p)?;p.finish()}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  schema(control)?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;store::sqlite_snapshot::validate_sqlite_database_schema_controlled(database,Self::SQLITE_SCHEMA,SqliteSnapshotPhase::ReconstructSnapshot,control)?;
  let doc=database.table("space_document")?.single_row()?;if doc.values.len()!=5||doc.integer(0)?!=doc.rowid{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "space document identity differs"))}
  let kind=match doc.text(3)?{"atelier"=>SpaceKind::Atelier,"studio"=>SpaceKind::Studio,"archive"=>SpaceKind::Archive,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "space kind is not declared"))};let visibility=match doc.text(4)?{"private"=>SpaceVisibility::Private,"public"=>SpaceVisibility::Public,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "space visibility is not declared"))};
  fields::reconstruct(control,|mut native|{
  let schema=fields::text(doc,1,&mut native)?;let name=fields::text(doc,2,&mut native)?;
  let ordered=fields::ordered(database.table("space_user")?,doc.rowid,7,&mut native)?;let mut users=native.allocate_vec(ordered.len())?;native.begin_stage(ordered.len())?;for row in ordered{users.push(SpaceUser{id:fields::text(row,3,&mut native)?,name:fields::text(row,4,&mut native)?,avatar:fields::optional(row,5,&mut native)?,role:SpaceRole::parse(row.text(6)?).ok_or_else(||fields::invalid("space role is not declared"))?});native.step()?;}
  let ordered=fields::ordered(database.table("space_collection")?,doc.rowid,6,&mut native)?;let mut collections=native.allocate_vec(ordered.len())?;native.begin_stage(ordered.len())?;for row in ordered{collections.push(CollectionRef{id:fields::text(row,3,&mut native)?,name:fields::text(row,4,&mut native)?,document_id:fields::text(row,5,&mut native)?});native.step()?;}
  let ordered=fields::ordered(database.table("space_program")?,doc.rowid,4,&mut native)?;let mut programs=native.allocate_vec(ordered.len())?;native.begin_stage(ordered.len())?;for row in ordered{programs.push(fields::text(row,3,&mut native)?);native.step()?;}
  let ordered=fields::ordered(database.table("space_extension")?,doc.rowid,8,&mut native)?;let mut extensions=native.allocate_vec(ordered.len())?;native.begin_stage(ordered.len())?;for row in ordered{extensions.push(InstalledExtension{extension_id:fields::text(row,3,&mut native)?,version:fields::text(row,4,&mut native)?,source_uri:fields::text(row,5,&mut native)?,package_hash:fields::text(row,6,&mut native)?,enabled:match row.integer(7)?{0=>false,1=>true,_=>return Err(fields::invalid("space enabled must be boolean"))}});native.step()?;}native.checkpoint()?;Ok(Self{schema,name,kind,visibility,users,collections,programs,extensions})})
 }
 fn validate_sqlite_snapshot_subset(&self,dialect:&store::os_io::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{
  (|| -> Result<store::io_schema::IoOutcome<()>,ValueError> {
  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!=S_SPACE_SCHEMA||dialect.standard!="1"||dialect.subset!="*"{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "space does not own this SQLite coordinate"))}let candidate=Self::from_sqlite_database(database,control)?;if self!=&candidate{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "space semantic state differs"))}Ok(store::io_schema::IoOutcome::clean(()))
 
  })().map_err(store::io_schema::IoError::from_value_error)
 }
}
