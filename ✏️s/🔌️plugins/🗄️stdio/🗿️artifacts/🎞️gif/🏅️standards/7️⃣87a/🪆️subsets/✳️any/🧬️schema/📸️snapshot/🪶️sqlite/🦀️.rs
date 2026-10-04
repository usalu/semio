//! 🎞️ GIF87a's individually authored indexed-image SQLite projection.
use super::*;
use semio_framework_os_kernel::sqlite_snapshot::{ValueError, ValueRefusalKind};

fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn work(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::WorkLimit,message)}
fn ownership(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::OwnershipLimit,message)}
fn allocation(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::AllocationFailed,message)}

use semio_framework_os_kernel::{sqlite_snapshot::{artifact::{Cell, Projection, reconstruct_text, ordered_row_refs}, validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase}, ArtifactSqliteSnapshot};
use std::collections::{BTreeMap, BTreeSet};

fn unsigned(row: &SqliteRow, column: usize) -> Result<u32,ValueError> { u32::try_from(row.integer(column)?).map_err(|error|invalid(error.to_string())) }
fn byte(row: &SqliteRow, column: usize) -> Result<u8,ValueError> { u8::try_from(row.integer(column)?).map_err(|error|invalid(error.to_string())) }
fn flag(row: &SqliteRow, column: usize) -> Result<bool,ValueError> { match row.integer(column)? {0=>Ok(false),1=>Ok(true),_=>Err(invalid("GIF87 boolean must be zero or one"))} }
fn entities<'a>(database:&'a SqliteDatabase,table:&str,control:&mut SqliteSnapshotControl<'_>)->Result<&'a [SqliteRow],ValueError>{let rows=&database.table(table)?.rows;let mut keys=BTreeSet::new();for(position,row)in rows.iter().enumerate(){if position%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,rows.len())?;}if row.rowid<=0||row.integer(0)?!=row.rowid||!keys.insert(row.rowid){return Err(invalid("GIF87 entities require unique positive aliased identifiers"));}}Ok(rows)}
fn identifiers(rows:impl IntoIterator<Item=i64>,control:&mut SqliteSnapshotControl<'_>)->Result<BTreeSet<i64>,ValueError>{let mut keys=BTreeSet::new();for(position,key)in rows.into_iter().enumerate(){if position%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,0)?;}keys.insert(key);}Ok(keys)}
fn one_to_one<'a>(rows:&'a [SqliteRow],parents:&BTreeSet<i64>,control:&mut SqliteSnapshotControl<'_>)->Result<BTreeMap<i64,&'a SqliteRow>,ValueError>{let mut out=BTreeMap::new();for(position,row)in rows.iter().enumerate(){if position%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,rows.len())?;}if !parents.contains(&row.rowid)||out.insert(row.rowid,row).is_some(){return Err(invalid("GIF87 one-to-one relationship has an unknown or duplicate owner"));}}Ok(out)}
fn groups<'a>(rows:&'a [SqliteRow],parents:&BTreeSet<i64>,ordered:bool,control:&mut SqliteSnapshotControl<'_>)->Result<BTreeMap<i64,Vec<&'a SqliteRow>>,ValueError>{let mut groups=BTreeMap::<i64,Vec<&SqliteRow>>::new();for(position,row)in rows.iter().enumerate(){if position%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,rows.len())?;}let owner=row.integer(1)?;if !parents.contains(&owner){return Err(invalid("GIF87 child has an unknown owner"));}groups.entry(owner).or_default().push(row);}if ordered{for rows in groups.values_mut(){let mut slots=vec![None;rows.len()];for(position,row)in rows.iter().enumerate(){if position%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,rows.len())?;}let ordinal=usize::try_from(row.integer(2)?).map_err(|error|invalid(error.to_string()))?;let slot=slots.get_mut(ordinal).ok_or_else(||invalid("GIF87 child ordinals must be dense"))?;if slot.replace(*row).is_some(){return Err(invalid("GIF87 child ordinals must be unique"));}}let total=slots.len();rows.clear();for(position,row)in slots.into_iter().enumerate(){if position%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,total)?;}rows.push(row.ok_or_else(||invalid("GIF87 child ordinals must be dense"))?);}}}Ok(groups)}
fn palette(out:&mut Projection<'_,'_>,header:&str,entries:&str,owner:i64,palette:&Option<GifColorTable>)->Result<(),ValueError>{if let Some(palette)=palette {out.insert_key(header,owner,&[Cell::Integer(i64::from(palette.sorted))])?;for (ordinal,color)in palette.colors.iter().enumerate(){out.insert(entries,&[Cell::Integer(owner),Cell::Integer(ordinal as i64),Cell::Integer(color.r.into()),Cell::Integer(color.g.into()),Cell::Integer(color.b.into())])?;}}Ok(())}
fn read_palette(header:Option<&&SqliteRow>,colors:Option<&Vec<&SqliteRow>>,control:&mut SqliteSnapshotControl<'_>)->Result<Option<GifColorTable>,ValueError>{let Some(header)=header else{return Ok(None)};let mut entries=Vec::new();for row in colors.into_iter().flatten(){if entries.len()%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,entries.len(),0)?;}entries.push(GifRgb{r:byte(row,3)?,g:byte(row,4)?,b:byte(row,5)?});}Ok(Some(GifColorTable{sorted:flag(header,1)?,colors:entries}))}
fn read_pixels(rows:Option<&Vec<&SqliteRow>>,_width:u32,_height:u32,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<u8>,ValueError>{let rows=rows.map(Vec::as_slice).unwrap_or_default();control.check_value_bytes(rows.len())?;let mut indices=Vec::new();indices.try_reserve_exact(rows.len()).map_err(|_|allocation("GIF index allocation"))?;for(ordinal,row)in rows.iter().enumerate(){if ordinal%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,ordinal,rows.len())?;}if row.integer(2)?!=ordinal as i64{return Err(invalid("GIF index ordinals must be dense"));}indices.push(byte(row,3)?);}Ok(indices)}

fn palette_bound(table:Option<&GifColorTable>,bound:&mut semio_framework_os_kernel::sqlite_snapshot::artifact::NativeEncodingBound<'_,'_>)->Result<(),ValueError>{if let Some(table)=table{bound.add(4096)?;bound.repeated(table.colors.len(),4096)?;for _ in &table.colors{bound.add(64)?;}}Ok(())}

impl ArtifactSqliteSnapshot for GifSnapshot {
 fn encode_sqlite_snapshot_native(&self,encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{
  self.preflight_sqlite_snapshot_encoding(encoding,control)?;
  let mut rows=1usize;if let Some(palette)=&self.gct{rows=rows.checked_add(1).and_then(|value|value.checked_add(palette.colors.len())).ok_or_else(||work("GIF entity count overflow"))?;}control.check_rows(rows)?;
  for image in &self.images{rows=rows.checked_add(1).and_then(|value|value.checked_add(image.indices.len())).ok_or_else(||work("GIF entity count overflow"))?;if let Some(palette)=&image.lct{rows=rows.checked_add(1).and_then(|value|value.checked_add(palette.colors.len())).ok_or_else(||work("GIF entity count overflow"))?;}control.check_rows(rows)?;control.checkpoint(SqliteSnapshotPhase::EncodeNative,rows,0)?;}
  control.check_rows(rows)?;
  store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)
 }
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{let encoding=match payload{store::io_schema::IoPayload::Binary(_)=>semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding::Binary,store::io_schema::IoPayload::Text(_)=>semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding::Text};let snapshot=store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|Self::__dsl_from_record_controlled(record,native),control)?;snapshot.preflight_sqlite_snapshot_encoding(encoding,control)?;Ok(snapshot)}
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
  let mut bound=semio_framework_os_kernel::sqlite_snapshot::artifact::NativeEncodingBound::new(control)?;bound.add(32768)?;bound.repeated(self.schema.len(),32)?;palette_bound(self.gct.as_ref(),&mut bound)?;
  bound.repeated(self.images.len(),16384)?;for image in &self.images{bound.repeated(image.indices.len(),16)?;palette_bound(image.lct.as_ref(),&mut bound)?;}
  bound.finish()
 }
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn validate_sqlite_snapshot_subset(&self,dialect:&store::io_schema::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{
  (||->Result<_,ValueError>{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.gif"||dialect.standard!="87a"||dialect.subset!="*"{return Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,format!("GIF87a does not own semantic subset {}",dialect.to_coordinate())));}if database.table("gif87_document")?.single_row()?.text(1)?!=self.schema{return Err(invalid("GIF87a semantic document identity disagrees with its snapshot"));}Ok(store::io_schema::IoOutcome::clean(()))})().map_err(store::io_schema::IoError::from_value_error)
 }
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
  let mut out=Projection::new(Self::SQLITE_SCHEMA,control)?;
  out.insert("gif87_document",&[Cell::Text(&self.schema),Cell::Integer(self.width.into()),Cell::Integer(self.height.into()),Cell::Integer(self.background_color_index.into()),Cell::Integer(self.pixel_aspect_ratio.into())])?;
  palette(&mut out,"gif87_global_palette","gif87_global_color",1,&self.gct)?;
  for (ordinal,image)in self.images.iter().enumerate(){
   let id=out.insert("gif87_image",&[Cell::Integer(1),Cell::Integer(ordinal as i64),Cell::Integer(image.left.into()),Cell::Integer(image.top.into()),Cell::Integer(image.width.into()),Cell::Integer(image.height.into()),Cell::Integer(i64::from(image.interlace))])?;
   palette(&mut out,"gif87_local_palette","gif87_local_color",id,&image.lct)?;
   for (position,index)in image.indices.iter().enumerate(){out.insert("gif87_pixel",&[Cell::Integer(id),Cell::Integer(position as i64),Cell::Integer((*index).into())])?;}
  }out.finish()
 }
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits())?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
  let document=database.table("gif87_document")?.single_row()?;if document.rowid!=1||document.integer(0)?!=1{return Err(invalid("GIF87 document identity must be one"));}
  let image_rows=entities(database,"gif87_image",control)?;let image_ids=identifiers(image_rows.iter().map(|row|row.rowid),control)?;let document_ids=BTreeSet::from([1]);
  let global=one_to_one(entities(database,"gif87_global_palette",control)?,&document_ids,control)?;let global_ids=identifiers(global.keys().copied(),control)?;let global_colors=groups(entities(database,"gif87_global_color",control)?,&global_ids,true,control)?;
  let local=one_to_one(entities(database,"gif87_local_palette",control)?,&image_ids,control)?;let local_ids=identifiers(local.keys().copied(),control)?;let local_colors=groups(entities(database,"gif87_local_color",control)?,&local_ids,true,control)?;
  let pixels=groups(entities(database,"gif87_pixel",control)?,&image_ids,true,control)?;let mut images=Vec::new();
  for row in ordered_row_refs(database.table("gif87_image")?,2,control)?{if row.integer(1)?!=1{return Err(invalid("GIF87 image has an unknown document"));}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,images.len(),image_rows.len())?;let width=unsigned(row,5)?;let height=unsigned(row,6)?;images.push(GifImage{left:unsigned(row,3)?,top:unsigned(row,4)?,width,height,interlace:flag(row,7)?,lct:read_palette(local.get(&row.rowid),local_colors.get(&row.rowid),control)?,indices:read_pixels(pixels.get(&row.rowid),width,height,control)?});}
  let result=Self{schema:reconstruct_text(control,document.text(1)?)?,width:unsigned(document,2)?,height:unsigned(document,3)?,gct:read_palette(global.get(&1),global_colors.get(&1),control)?,background_color_index:byte(document,4)?,pixel_aspect_ratio:byte(document,5)?,images};control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,image_rows.len(),image_rows.len())?;Ok(result)
 }
}
