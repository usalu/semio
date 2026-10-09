//! 📸️ Relational owned JPEG raster, density, thumbnail, and opaque metadata.
use crate::standards::v_jfif_1_01::subsets::document::schema::snapshot::*;
use crate::standards::v_jfif_1_01::subsets::document::io::text::snapshot as owned_text;
use semio_framework_os_kernel::sqlite_snapshot::{ValueError, ValueRefusalKind};

fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn work(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::WorkLimit,message)}
fn ownership(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::OwnershipLimit,message)}
fn allocation(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::AllocationFailed,message)}
fn text_error(error:ValueError)->semio_framework_diagnostic::TextError{semio_framework_diagnostic::TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1))}
use std::collections::{BTreeMap,BTreeSet};
use semio_framework_os_kernel::{sqlite_snapshot::{artifact::{Cell,Projection,reconstruct_text},validate_sqlite_database_schema,SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase},ArtifactSqliteSnapshot};

fn octet(row:&SqliteRow,column:usize)->Result<u8,ValueError>{u8::try_from(row.integer(column)?).map_err(|error|invalid(error.to_string()))}
fn short(row:&SqliteRow,column:usize)->Result<u16,ValueError>{u16::try_from(row.integer(column)?).map_err(|error|invalid(error.to_string()))}
fn unsigned(row:&SqliteRow,column:usize)->Result<u32,ValueError>{u32::try_from(row.integer(column)?).map_err(|error|invalid(error.to_string()))}
fn checkpoint(control:&mut SqliteSnapshotControl<'_>,position:usize,total:usize)->Result<(),ValueError>{if position%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,total)?;}Ok(())}
fn identifiers<'a>(rows:impl IntoIterator<Item=&'a SqliteRow>,control:&mut SqliteSnapshotControl<'_>)->Result<BTreeSet<i64>,ValueError>{let mut keys=BTreeSet::new();for(position,row)in rows.into_iter().enumerate(){checkpoint(control,position,0)?;keys.insert(row.rowid);}Ok(keys)}
fn entities<'a>(database:&'a SqliteDatabase,table:&str,control:&mut SqliteSnapshotControl<'_>)->Result<&'a [SqliteRow],ValueError>{let rows=&database.table(table)?.rows;let mut keys=BTreeSet::new();for(position,row)in rows.iter().enumerate(){checkpoint(control,position,rows.len())?;if row.rowid<=0||row.integer(0)?!=row.rowid||!keys.insert(row.rowid){return Err(invalid(format!("{table} requires unique positive aliased identities")));}}Ok(rows)}
fn groups<'a>(database:&'a SqliteDatabase,table:&str,parents:&BTreeSet<i64>,control:&mut SqliteSnapshotControl<'_>)->Result<BTreeMap<i64,Vec<&'a SqliteRow>>,ValueError>{let rows=entities(database,table,control)?;let mut groups=BTreeMap::<i64,Vec<&SqliteRow>>::new();for(position,row)in rows.iter().enumerate(){checkpoint(control,position,rows.len())?;let owner=row.integer(1)?;if !parents.contains(&owner){return Err(invalid(format!("{table} names an unknown owner")));}groups.entry(owner).or_default().push(row);}for rows in groups.values_mut(){let mut ordered=vec![None;rows.len()];for(position,row)in rows.iter().enumerate(){checkpoint(control,position,rows.len())?;let ordinal=usize::try_from(row.integer(2)?).map_err(|error|invalid(error.to_string()))?;let slot=ordered.get_mut(ordinal).ok_or_else(||invalid("JPEG child ordinals must be dense"))?;if slot.replace(*row).is_some(){return Err(invalid("JPEG child ordinals must be unique"));}}let total=ordered.len();rows.clear();for(position,row)in ordered.into_iter().enumerate(){checkpoint(control,position,total)?;rows.push(row.ok_or_else(||invalid("JPEG child ordinals must be dense"))?);}}Ok(groups)}
fn ordered<'a>(database:&'a SqliteDatabase,table:&str,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<&'a SqliteRow>,ValueError>{Ok(groups(database,table,&BTreeSet::from([1]),control)?.remove(&1).unwrap_or_default())}
fn optional<'a>(database:&'a SqliteDatabase,table:&str,control:&mut SqliteSnapshotControl<'_>)->Result<Option<&'a SqliteRow>,ValueError>{let rows=entities(database,table,control)?;if rows.len()>1||rows.first().is_some_and(|row|row.rowid!=1){return Err(invalid(format!("{table} optional identity must be one")));}Ok(rows.first())}
fn channel(pixel:&[u8],index:usize)->Cell<'_>{pixel.get(index).map_or(Cell::Null,|value|Cell::Integer((*value).into()))}
fn write_rgba(out:&mut Projection<'_,'_>,pixels:&[u8])->Result<(),ValueError>{for(ordinal,pixel)in pixels.chunks(4).enumerate(){out.insert("jpg_rgba_pixel",&[Cell::Integer(1),Cell::Integer(i64::try_from(ordinal).map_err(|error|work(error.to_string()))?),channel(pixel,0),channel(pixel,1),channel(pixel,2),channel(pixel,3)])?;}Ok(())}
fn write_rgb(out:&mut Projection<'_,'_>,thumbnail:&JfifThumbnail)->Result<(),ValueError>{for(ordinal,pixel)in thumbnail.rgb_data.chunks(3).enumerate(){out.insert("jpg_thumbnail_rgb_pixel",&[Cell::Integer(1),Cell::Integer(i64::try_from(ordinal).map_err(|error|work(error.to_string()))?),channel(pixel,0),channel(pixel,1),channel(pixel,2)])?;}Ok(())}
fn read_pixels(database:&SqliteDatabase,table:&str,channels:usize,present:bool,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<u8>,ValueError>{
 let rows=ordered(database,table,control)?;if !present&&!rows.is_empty(){return Err(invalid("JPEG absent thumbnail cannot own pixel channels"));}let maximum=rows.len().checked_mul(channels).ok_or_else(||work("JPEG channel count overflow"))?;control.check_value_bytes(maximum)?;let mut pixels=Vec::new();pixels.try_reserve_exact(maximum).map_err(|_|allocation("JPEG channel allocation failed"))?;
 for(position,row)in rows.iter().enumerate(){checkpoint(control,position,rows.len())?;if row.values.len()!=channels+3{return Err(invalid("JPEG pixel channel shape differs from its declared schema"));}let mut absent=false;for channel in 0..channels{if row.values[3+channel]==SqliteValue::Null{if channel==0||position+1!=rows.len(){return Err(invalid("JPEG partial channels require the last pixel entity"));}absent=true;}else{if absent{return Err(invalid("JPEG pixel channels require contiguous ownership"));}pixels.push(octet(row,3+channel)?);}}}Ok(pixels)
}
fn read_octets(rows:&[&SqliteRow],control:&mut SqliteSnapshotControl<'_>)->Result<Vec<u8>,ValueError>{let mut bytes=Vec::new();for(position,row)in rows.iter().enumerate(){checkpoint(control,position,rows.len())?;bytes.push(octet(row,3)?);}Ok(bytes)}

fn native_rows(snapshot:&JpgSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 let mut rows=1usize;let mut add=|count:usize|->Result<(),ValueError>{rows=rows.checked_add(count).ok_or_else(||work("JPEG native entity row count overflow"))?;control.check_rows(rows)};
 add(snapshot.image.pixels.len().div_ceil(4))?;if let Some(thumbnail)=&snapshot.image.jfif_thumbnail{add(1)?;add(thumbnail.rgb_data.len().div_ceil(3))?;}
 for(position,segment)in snapshot.image.other_segments.iter().enumerate(){rows=rows.checked_add(1).and_then(|rows|rows.checked_add(segment.data.len())).ok_or_else(||work("JPEG segment row count overflow"))?;control.check_rows(rows)?;if position%256==0{control.checkpoint(SqliteSnapshotPhase::EncodeNative,position,snapshot.image.other_segments.len())?;}}
 Ok(())
}

impl ArtifactSqliteSnapshot for JpgSnapshot{
 fn encode_sqlite_snapshot_native(&self,encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{
  self.preflight_sqlite_snapshot_encoding(encoding,control)?;native_rows(self,control)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),owned_text::spec_producer(),|native|owned_text::to_record_controlled(self,native),control,native_owner)
 }

 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{let encoding=match payload{store::io_schema::IoPayload::Binary(_)=>semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding::Binary,store::io_schema::IoPayload::Text(_)=>semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding::Text};let snapshot:Self=store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),owned_text::spec_producer(),|record,output,native,_body|{*output=Some(owned_text::from_record_controlled(record,native)?);Ok(())},control,native_control)?;snapshot.preflight_sqlite_snapshot_encoding(encoding,control)?;Ok(snapshot)}
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
  use semio_framework_os_kernel::sqlite_snapshot::artifact::NativeEncodingBound;
  let mut bound=NativeEncodingBound::new(control)?;bound.add(32768)?;bound.repeated(self.schema.len(),32)?;bound.repeated(self.image.pixels.len(),16)?;
  if let Some(thumbnail)=&self.image.jfif_thumbnail{bound.add(4096)?;bound.repeated(thumbnail.rgb_data.len(),16)?;}
  bound.repeated(self.image.other_segments.len(),4096)?;for segment in &self.image.other_segments{bound.repeated(segment.data.len(),16)?;}bound.finish()
 }

 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0).map_err(store::io_schema::IoError::from_value_error)?;if dialect.artifact_kind!="s.stdio.jpg"||dialect.standard!="jfif-1.01"||!matches!(dialect.subset.as_str(),"*"|"baseline"){return Err(store::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::UnsupportedOwner,format!("JPEG does not own semantic subset {}",dialect.to_coordinate()))));}if database.table("jpg_document").map_err(store::io_schema::IoError::from_value_error)?.single_row().map_err(store::io_schema::IoError::from_value_error)?.text(1).map_err(store::io_schema::IoError::from_value_error)?!=self.schema{return Err(store::io_schema::IoError::from_value_error(invalid("JPEG document schema identity disagrees with its owned snapshot")));}
  Ok(store::io_schema::IoOutcome{value:(),diagnostics:Vec::new()})
 }
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
  let mut out=Projection::new(Self::SQLITE_SCHEMA,control)?;let units=match self.image.jfif_density_units{JfifDensityUnits::Aspect=>"aspect",JfifDensityUnits::PixelsPerInch=>"pixelsPerInch",JfifDensityUnits::PixelsPerCm=>"pixelsPerCm"};out.insert("jpg_document",&[Cell::Text(&self.schema),Cell::Integer(self.image.width.into()),Cell::Integer(self.image.height.into()),Cell::Integer(self.image.jfif_version.0.into()),Cell::Integer(self.image.jfif_version.1.into()),Cell::Text(units),Cell::Integer(self.image.jfif_x_density.into()),Cell::Integer(self.image.jfif_y_density.into())])?;write_rgba(&mut out,&self.image.pixels)?;
  if let Some(thumbnail)=&self.image.jfif_thumbnail{out.insert("jpg_thumbnail",&[Cell::Integer(thumbnail.width.into()),Cell::Integer(thumbnail.height.into())])?;write_rgb(&mut out,thumbnail)?;}
  for(ordinal,segment)in self.image.other_segments.iter().enumerate(){let id=out.insert("jpg_segment",&[Cell::Integer(1),Cell::Integer(ordinal as i64),Cell::Integer(segment.marker.into())])?;for(ordinal,octet)in segment.data.iter().enumerate(){out.insert("jpg_segment_octet",&[Cell::Integer(id),Cell::Integer(ordinal as i64),Cell::Integer((*octet).into())])?;}}out.finish()
 }
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits())?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;let document=database.table("jpg_document")?.single_row()?;if document.rowid!=1||document.integer(0)?!=1{return Err(invalid("JPEG document identity must be one"));}let width=unsigned(document,2)?;let height=unsigned(document,3)?;let pixels=read_pixels(database,"jpg_rgba_pixel",4,true,control)?;
  let thumbnail=optional(database,"jpg_thumbnail",control)?;let thumbnail_width=thumbnail.map(|row|octet(row,1)).transpose()?;let thumbnail_height=thumbnail.map(|row|octet(row,2)).transpose()?;let rgb=read_pixels(database,"jpg_thumbnail_rgb_pixel",3,thumbnail.is_some(),control)?;let jfif_thumbnail=thumbnail.map(|_|JfifThumbnail{width:thumbnail_width.unwrap(),height:thumbnail_height.unwrap(),rgb_data:rgb});
  let segment_rows=ordered(database,"jpg_segment",control)?;let keys=identifiers(segment_rows.iter().copied(),control)?;let mut octets=groups(database,"jpg_segment_octet",&keys,control)?;let mut other_segments=Vec::new();for(position,row)in segment_rows.iter().enumerate(){checkpoint(control,position,segment_rows.len())?;other_segments.push(JpgSegment{marker:octet(row,3)?,data:read_octets(&octets.remove(&row.rowid).unwrap_or_default(),control)?});}
  let jfif_density_units=match document.text(6)?{"aspect"=>JfifDensityUnits::Aspect,"pixelsPerInch"=>JfifDensityUnits::PixelsPerInch,"pixelsPerCm"=>JfifDensityUnits::PixelsPerCm,_=>return Err(invalid("JPEG density unit is unknown"))};let result=Self{schema:reconstruct_text(control,document.text(1)?)?,image:crate::JpgImage{width,height,pixels,jfif_version:(octet(document,4)?,octet(document,5)?),jfif_density_units,jfif_x_density:short(document,7)?,jfif_y_density:short(document,8)?,jfif_thumbnail,other_segments}};control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;Ok(result)
 }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
