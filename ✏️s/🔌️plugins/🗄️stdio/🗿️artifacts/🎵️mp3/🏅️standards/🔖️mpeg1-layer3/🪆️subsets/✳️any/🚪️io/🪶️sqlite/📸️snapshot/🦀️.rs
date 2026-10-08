//! 🎵️ Handcrafted ID3 tag and MPEG frame entities with ordered intrinsic octets.
use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::*;
use std::collections::BTreeMap;
use semio_framework_os_kernel::{sqlite_snapshot::{artifact::{Cell,RowWriter,NativeEncodingBound,reconstruct_text},validate_sqlite_database_schema,SnapshotEncoding,SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind},ArtifactSqliteSnapshot};

type Entities<'a>=BTreeMap<i64,&'a SqliteRow>;
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn work(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::WorkLimit,message)}
fn checkpoint(control:&mut SqliteSnapshotControl<'_>,position:usize,total:usize)->Result<(),ValueError>{if position%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,total)?;}Ok(())}
fn octet(row:&SqliteRow,column:usize)->Result<u8,ValueError>{u8::try_from(row.integer(column)?).map_err(|error|invalid(error.to_string()))}
fn short(row:&SqliteRow,column:usize)->Result<u16,ValueError>{u16::try_from(row.integer(column)?).map_err(|error|invalid(error.to_string()))}
fn flag(row:&SqliteRow,column:usize)->Result<bool,ValueError>{match row.integer(column)?{0=>Ok(false),1=>Ok(true),_=>Err(invalid("MP3 header flag must be boolean"))}}
fn entities<'a>(database:&'a SqliteDatabase,table:&str,columns:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Entities<'a>,ValueError>{let rows=&database.table(table)?.rows;let mut entities=BTreeMap::new();for(position,row)in rows.iter().enumerate(){checkpoint(control,position,rows.len())?;if row.values.len()!=columns||row.rowid<=0||row.integer(0)?!=row.rowid||entities.insert(row.rowid,row).is_some(){return Err(invalid(format!("{table} requires unique positive aliased identities and exact columns")));}}Ok(entities)}
fn optional_tag<'a>(database:&'a SqliteDatabase,table:&str,columns:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Entities<'a>,ValueError>{let rows=entities(database,table,columns,control)?;if rows.len()>1||rows.keys().any(|&id|id!=1){return Err(invalid("MP3 optional tag must have the document identity"));}Ok(rows)}
fn groups<'a>(rows:&Entities<'a>,parents:&Entities<'_>,control:&mut SqliteSnapshotControl<'_>)->Result<BTreeMap<i64,Vec<&'a SqliteRow>>,ValueError>{
 let mut groups=BTreeMap::<i64,Vec<&SqliteRow>>::new();for(position,&row)in rows.values().enumerate(){checkpoint(control,position,rows.len())?;let owner=row.integer(1)?;if !parents.contains_key(&owner){return Err(invalid("MP3 ordered entity has an unknown owner"));}groups.entry(owner).or_default().push(row);}
 for(group,rows)in groups.values_mut().enumerate(){checkpoint(control,group,0)?;let mut slots=vec![None;rows.len()];for(position,&row)in rows.iter().enumerate(){checkpoint(control,position,rows.len())?;let ordinal=usize::try_from(row.integer(2)?).map_err(|error|invalid(error.to_string()))?;if slots.get_mut(ordinal).ok_or_else(||invalid("MP3 ordinals must be dense"))?.replace(row).is_some(){return Err(invalid("MP3 ordinals must be unique"));}}let mut ordered=Vec::new();for(position,row)in slots.into_iter().enumerate(){checkpoint(control,position,rows.len())?;ordered.push(row.ok_or_else(||invalid("MP3 ordinals must be dense"))?);}*rows=ordered;}Ok(groups)
}
fn write_octets(out:&mut RowWriter<'_,'_>,table:&str,owner:i64,bytes:&[u8])->Result<(),ValueError>{for(ordinal,byte)in bytes.iter().enumerate(){out.insert(table,&[Cell::Integer(owner),Cell::Integer(i64::try_from(ordinal).map_err(|error|work(error.to_string()))?),Cell::Integer((*byte).into())])?;}Ok(())}
fn read_octets(rows:Option<&Vec<&SqliteRow>>,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<u8>,ValueError>{let mut bytes=Vec::new();for row in rows.into_iter().flatten(){checkpoint(control,bytes.len(),0)?;bytes.push(octet(row,3)?);}Ok(bytes)}
fn header(row:&SqliteRow)->Result<Mp3FrameHeader,ValueError>{Ok(Mp3FrameHeader{mpeg_version_id:octet(row,3)?,layer:octet(row,4)?,protection_bit:flag(row,5)?,bitrate_index:octet(row,6)?,sample_rate_index:octet(row,7)?,padding:flag(row,8)?,private_bit:flag(row,9)?,channel_mode:octet(row,10)?,mode_extension:octet(row,11)?,copyright:flag(row,12)?,original:flag(row,13)?,emphasis:octet(row,14)?})}


#[path="🧮️semantic/🦀️.rs"]
mod semantic;
fn content_cells(content:&Id3Content)->[Cell<'_>;7]{
 use Id3Content::*;let n=Cell::Null;match content{
  Text{..}=>[Cell::Text("text"),n,n,n,n,n,n],
  UserText{description,..}=>[Cell::Text("userText"),n,Cell::Text(description),n,n,n,n],
  Comment{language,description,text}=>[Cell::Text("comment"),Cell::Text(language),Cell::Text(description),Cell::Text(text),n,n,n],
  Lyrics{language,description,text}=>[Cell::Text("lyrics"),Cell::Text(language),Cell::Text(description),Cell::Text(text),n,n,n],
  Url{url}=>[Cell::Text("url"),n,n,n,Cell::Text(url),n,n],
  UserUrl{description,url}=>[Cell::Text("userUrl"),n,Cell::Text(description),n,Cell::Text(url),n,n],
  Picture{mime,picture_type,description,..}=>[Cell::Text("picture"),n,Cell::Text(description),n,n,Cell::Text(mime),Cell::Integer((*picture_type).into())],
  Opaque{..}=>[Cell::Text("opaque"),n,n,n,n,n,n],
 }
}
fn visit_rows(snapshot:&Mp3Snapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.insert("mp3_document",&[Cell::Text(&snapshot.schema)])?;
 if let Some(tag)=&snapshot.id3v2{out.insert_key("mp3_id3v2_tag",1,&[])?;for(ordinal,frame)in tag.frames.iter().enumerate(){validate_id3_frame(frame).map_err(invalid)?;let cells=content_cells(&frame.content);let id=out.insert("mp3_id3v2_frame",&[Cell::Integer(1),Cell::Integer(i64::try_from(ordinal).map_err(|e|work(e.to_string()))?),Cell::Text(&frame.id),cells[0],cells[1],cells[2],cells[3],cells[4],cells[5],cells[6]])?;match &frame.content{
  Id3Content::Text{values}|Id3Content::UserText{values,..}=>{for(ordinal,value)in values.iter().enumerate(){out.insert("mp3_id3_text_value",&[Cell::Integer(id),Cell::Integer(i64::try_from(ordinal).map_err(|e|work(e.to_string()))?),Cell::Text(value)])?;}},
  Id3Content::Opaque{bytes}=>write_octets(out,"mp3_id3_content_octet",id,bytes)?,Id3Content::Picture{payload,..}=>write_octets(out,"mp3_id3_content_octet",id,payload)?,_=>{}
 }}}
 if let Some(t)=&snapshot.id3v1{validate_id3v1_tag(t).map_err(invalid)?;out.insert_key("mp3_id3v1_tag",1,&[Cell::Text(&t.title),Cell::Text(&t.artist),Cell::Text(&t.album),Cell::Text(&t.year),Cell::Text(&t.comment),t.track.map_or(Cell::Null,|v|Cell::Integer(v.into())),t.genre.map_or(Cell::Null,|v|Cell::Integer(v.into()))])?;}
 for(ordinal,frame)in snapshot.frames.iter().enumerate(){let h=&frame.header;let id=out.insert("mp3_audio_frame",&[Cell::Integer(1),Cell::Integer(i64::try_from(ordinal).map_err(|error|work(error.to_string()))?),Cell::Integer(h.mpeg_version_id.into()),Cell::Integer(h.layer.into()),Cell::Integer(i64::from(h.protection_bit)),Cell::Integer(h.bitrate_index.into()),Cell::Integer(h.sample_rate_index.into()),Cell::Integer(i64::from(h.padding)),Cell::Integer(i64::from(h.private_bit)),Cell::Integer(h.channel_mode.into()),Cell::Integer(h.mode_extension.into()),Cell::Integer(i64::from(h.copyright)),Cell::Integer(i64::from(h.original)),Cell::Integer(h.emphasis.into())])?;write_octets(out,"mp3_audio_payload_octet",id,&frame.payload)?;}Ok(())
}
fn read_content(row:&SqliteRow,values:Option<&Vec<&SqliteRow>>,bytes:Option<&Vec<&SqliteRow>>,control:&mut SqliteSnapshotControl<'_>)->Result<Id3Content,ValueError>{
 let kind=row.text(4)?;let mut used=Vec::new();let mut text=|index|{used.push(index);reconstruct_text(control,row.text(index)?)};
 let content=match kind{
  "text"|"userText"=>{let description=if kind=="userText"{Some(text(6)?)}else{None};let mut strings=Vec::new();for row in values.into_iter().flatten(){checkpoint(control,strings.len(),0)?;strings.push(reconstruct_text(control,row.text(3)?)?);}match description{Some(description)=>Id3Content::UserText{description,values:strings},None=>Id3Content::Text{values:strings}}},
  "comment"|"lyrics"=>{let language=text(5)?;let description=text(6)?;let text=text(7)?;if kind=="comment"{Id3Content::Comment{language,description,text}}else{Id3Content::Lyrics{language,description,text}}},
  "url"=>Id3Content::Url{url:text(8)?},"userUrl"=>Id3Content::UserUrl{description:text(6)?,url:text(8)?},
  "picture"=>{let mime=text(9)?;let description=text(6)?;used.push(10);Id3Content::Picture{mime,description,picture_type:octet(row,10)?,payload:read_octets(bytes,control)?}},
  "opaque"=>Id3Content::Opaque{bytes:read_octets(bytes,control)?},_=>return Err(invalid("MP3 semantic content kind")),
 };
 if !matches!(kind,"text"|"userText")&&values.is_some()||!matches!(kind,"opaque"|"picture")&&bytes.is_some(){return Err(invalid("MP3 content child role mismatch"));}
 for index in 5..=10{if !used.contains(&index)&&!matches!(row.values[index],semio_framework_os_kernel::sqlite_snapshot::SqliteValue::Null){return Err(invalid("MP3 unused semantic content field"));}}Ok(content)
}

impl ArtifactSqliteSnapshot for Mp3Snapshot{
 fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{let limits=control.limits();semantic::extent(limits)?;store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|{semantic::borrowed(record,limits,native)?;Self::__dsl_from_record_controlled(record,native)},control)}
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{
  semantic::typed(self,SqliteSnapshotPhase::EncodeNative,control)?;
  store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)
 }

 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::typed(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut bound=NativeEncodingBound::file_only(control)?;bound.add(16384)?;bound.repeated(self.schema.len(),24)?;if let Some(tag)=&self.id3v2{bound.add(2048)?;for frame in &tag.frames{bound.add(1024)?;bound.repeated(frame.id.len(),24)?;for cell in content_cells(&frame.content){if let Cell::Text(text)=cell{bound.repeated(text.len(),24)?;}}match &frame.content{Id3Content::Text{values}|Id3Content::UserText{values,..}=>for value in values{bound.add(128)?;bound.repeated(value.len(),24)?;},Id3Content::Opaque{bytes}=>bound.repeated(bytes.len(),16)?,Id3Content::Picture{payload,..}=>bound.repeated(payload.len(),16)?,_=>{}}}}if let Some(tag)=&self.id3v1{bound.add(512)?;for text in [&tag.title,&tag.artist,&tag.album,&tag.year,&tag.comment]{bound.repeated(text.len(),24)?;}}for frame in &self.frames{bound.add(4096)?;bound.repeated(frame.payload.len(),16)?;}bound.finish()}
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{semantic::extent(control.limits())?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut out)?;out.finish()}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits())?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;let documents=entities(database,"mp3_document",2,control)?;if documents.len()!=1||!documents.contains_key(&1){return Err(invalid("MP3 requires document identity one"));}let tags=optional_tag(database,"mp3_id3v2_tag",1,control)?;let trailers=optional_tag(database,"mp3_id3v1_tag",8,control)?;
  let id3_frames=entities(database,"mp3_id3v2_frame",11,control)?;let ordered_tags=groups(&id3_frames,&tags,control)?;let tag_values=groups(&entities(database,"mp3_id3_text_value",4,control)?,&id3_frames,control)?;let tag_octets=groups(&entities(database,"mp3_id3_content_octet",4,control)?,&id3_frames,control)?;
  let id3v2=if tags.contains_key(&1){let mut frames=Vec::new();for row in ordered_tags.get(&1).into_iter().flatten(){checkpoint(control,frames.len(),0)?;let frame=Id3Frame{id:reconstruct_text(control,row.text(3)?)?,content:read_content(row,tag_values.get(&row.rowid),tag_octets.get(&row.rowid),control)?};validate_id3_frame(&frame).map_err(invalid)?;frames.push(frame);}Some(Id3v2Tag{frames})}else{None};
  let id3v1=if let Some(row)=trailers.get(&1){let tag=Id3v1Tag{title:reconstruct_text(control,row.text(1)?)?,artist:reconstruct_text(control,row.text(2)?)?,album:reconstruct_text(control,row.text(3)?)?,year:reconstruct_text(control,row.text(4)?)?,comment:reconstruct_text(control,row.text(5)?)?,track:if matches!(row.values[6],semio_framework_os_kernel::sqlite_snapshot::SqliteValue::Null){None}else{Some(octet(row,6)?)},genre:if matches!(row.values[7],semio_framework_os_kernel::sqlite_snapshot::SqliteValue::Null){None}else{Some(octet(row,7)?)}};validate_id3v1_tag(&tag).map_err(invalid)?;Some(tag)}else{None};
  let audio=entities(database,"mp3_audio_frame",15,control)?;let ordered_audio=groups(&audio,&documents,control)?;let audio_octets=groups(&entities(database,"mp3_audio_payload_octet",4,control)?,&audio,control)?;let mut frames=Vec::new();for row in ordered_audio.get(&1).into_iter().flatten(){checkpoint(control,frames.len(),0)?;frames.push(Mp3Frame{header:header(row)?,payload:read_octets(audio_octets.get(&row.rowid),control)?});}let result=Self{schema:reconstruct_text(control,documents[&1].text(1)?)?,id3v2,frames,id3v1};control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;Ok(result)
 }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

