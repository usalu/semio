//! 🫴️ Original typed Binary fields before native schema or octet backing is born.
use super::BinarySnapshot;
use semio_framework_os_kernel::NativeSnapshotBodyWallet;
use semio_framework_value::{NativeDecodeControl,RetainedCloneGrant,retained_clone::RetainedCloneProgress,ValueError,ValueRefusalKind};
#[derive(Clone,Copy)]
pub(super)enum Carrier<'a>{Raw(&'a[u8]),Hex(&'a str)}
fn overflow()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"Binary original receiving extent overflow")}
pub(super)fn digit(ch:char)->Result<u8,ValueError>{ch.to_digit(16).and_then(|value|u8::try_from(value).ok()).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"invalid native Binary hex digit"))}
/// 📨️ Borrows the authored Binary envelope without creating an owned parser refusal.
pub(super)fn body<'a>(carrier:Carrier<'a>,native:&mut NativeDecodeControl<'_>)->Result<Carrier<'a>,ValueError>{
 let Carrier::Hex(text)=carrier else{return Ok(carrier)};if !text.starts_with("semio "){return Ok(carrier)}native.checkpoint()?;
 const TOKEN:&[u8]=b"stdio.binary.dsl v1";let end=6+TOKEN.len();let bytes=text.as_bytes();if bytes.len()<end{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"invalid semio preamble: truncated envelope token"))}
 let matches=native.scoped_stage(|native|{native.begin_stage(TOKEN.len())?;let mut position=6;for piece in[b"stdio.binary".as_slice(),b".",b"dsl",b" v",b"1"]{if bytes[position..position+piece.len()]!=*piece{return Ok(false)}position+=piece.len();native.advance(piece.len())?;}Ok::<bool,ValueError>(true)})?;
 if !matches{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"invalid semio preamble: declared envelope identity mismatch"))}
 let offset=if end==bytes.len(){end}else if bytes[end]==b'\n'{end+1}else if bytes.get(end..end+2)==Some(b"\r\n"){end+2}else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"invalid semio preamble: canonical preamble requires a line boundary"))};Ok(Carrier::Hex(&text[offset..]))
}
/// 🔍️ Validates borrowed hex before the original recipient can create any physical frame.
pub(super)fn count(carrier:Carrier<'_>,native:&mut NativeDecodeControl<'_>)->Result<usize,ValueError>{match carrier{Carrier::Raw(bytes)=>Ok(bytes.len()),Carrier::Hex(text)=>native.scoped_stage(|native|{native.begin_stage(text.len())?;let mut digits=0usize;for ch in text.chars(){if !ch.is_whitespace(){digit(ch)?;digits=digits.checked_add(1).ok_or_else(overflow)?;}native.advance(ch.len_utf8())?;}if digits%2!=0{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"odd hex length"))}Ok(digits/2)})}}
/// 🧾️ Admits every independently supplied body currency before publishing even an empty typed slot.
pub(super)fn bind(carrier:Carrier<'_>,schema:&str,slot:&mut Option<BinarySnapshot>,native:&mut NativeDecodeControl<'_>,body:&mut NativeSnapshotBodyWallet)->Result<(),ValueError>{
 if slot.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"Binary original receiving slot must be absent"))}
 let carrier=body(carrier,native)?;let count=count(carrier,native)?;let vector_header=if matches!(carrier,Carrier::Hex(_)){std::mem::size_of::<Vec<u8>>()}else{0};let header=std::mem::size_of::<BinarySnapshot>();let copy=header.checked_add(schema.len()).and_then(|value|value.checked_add(count)).and_then(|value|value.checked_add(vector_header)).ok_or_else(overflow)?;let capacity=schema.len().checked_add(count).ok_or_else(overflow)?;
 body.admit_frontier(RetainedCloneGrant{maximum_items:3,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:0,maximum_depth:1})?;
 native.checkpoint()?;body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:header,retained_capacity_bytes:0,released_bytes:0})?;
 *slot=Some(BinarySnapshot{schema:String::new(),bytes:Vec::new()});let output=slot.as_mut().unwrap();body.copy_text_into(native,schema,&mut output.schema,1)?;
 match carrier{
  Carrier::Raw(bytes)=>body.copy_bytes_into(native,bytes,&mut output.bytes,1),
  Carrier::Hex(text)=>{body.allocate_vec_into(native,count,&mut output.bytes,1)?;native.scoped_stage(|native|{native.begin_stage(text.len())?;let mut high=None;for ch in text.chars(){if !ch.is_whitespace(){let value=digit(ch)?;if let Some(first)=high.take(){output.bytes.push(first*16+value);body.record_progress(RetainedCloneProgress{copied_items:0,copied_bytes:1,retained_capacity_bytes:0,released_bytes:0})?;}else{high=Some(value);}}native.advance(ch.len_utf8())?;}Ok(())})}
 }
}


use semio_framework_os_kernel::sqlite_snapshot::{SqliteDatabase,SqliteTable,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,artifact::receiving::{Direction,Projection,Scalar,Port as OriginalPort,cell,text_cell,row,push_row,sort,settle,validate_schema,AuthoredTable}};
type Port<'a,'b,'c>=OriginalPort<'a,'b,'c,NativeSnapshotBodyWallet>;
const SQL:&str=include_str!("../🗄️.sql");
const NAMES:[&str;2]=["binary_document","binary_byte"];
fn definition(index:usize)->&'static str{SQL.split(';').filter(|part|!part.trim().is_empty()).nth(index).unwrap().trim()}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn ordinal(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"Binary ordinal exceeds signed SQL extent"))}
fn empty()->BinarySnapshot{BinarySnapshot{schema:String::new(),bytes:Vec::new()}}
fn integer(row:&SqliteRow,column:usize)->Result<i64,ValueError>{match row.values.get(column){Some(SqliteValue::Integer(value))=>Ok(*value),_=>Err(invalid("Binary authored integer cell is invalid"))}}
fn table<'a>(database:&'a SqliteDatabase,name:&str)->Result<&'a SqliteTable,ValueError>{database.tables.iter().find(|table|table.name.eq_ignore_ascii_case(name)).ok_or_else(||invalid("Binary authored table is missing"))}
fn project(source:&BinarySnapshot,frame:&mut Projection,port:&mut Port<'_,'_,'_>)->Result<(),ValueError>{
 let total=source.bytes.len().checked_add(1).ok_or_else(overflow)?;port.control.check_rows(total)?;let limits=port.control.limits();if limits.max_tables<2||limits.max_columns<4||(0..2).map(|index|NAMES[index].len()+definition(index).len()).sum::<usize>()>limits.max_schema_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Binary authored schema exceeds original ceiling"))}
 port.vector(2,&mut frame.database.tables,1)?;for(index,count)in[1,source.bytes.len()].into_iter().enumerate(){port.work(2,std::mem::size_of::<SqliteTable>())?;frame.database.tables.push(SqliteTable{name:String::new(),sql:String::new(),rows:Vec::new()});let table=&mut frame.database.tables[index];port.text(NAMES[index],&mut table.name,2)?;port.text(definition(index),&mut table.sql,2)?;port.vector(count,&mut table.rows,2)?;}
 row(frame,1,2,port)?;text_cell(frame,&source.schema,port)?;push_row(frame,0,port)?;
 for(index,byte)in source.bytes.iter().copied().enumerate(){row(frame,ordinal(index+1)?,4,port)?;cell(frame,Scalar::Integer(1),port)?;cell(frame,Scalar::Integer(ordinal(index)?),port)?;cell(frame,Scalar::Integer(i64::from(byte)),port)?;push_row(frame,1,port)?;}
 port.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,total,total)
}
#[derive(semio_framework_value::RetireOwned)]
struct Reconstruction{snapshot:BinarySnapshot,ordinals:Vec<usize>,identities:Vec<usize>}
fn reconstruct(database:&SqliteDatabase,frame:&mut Reconstruction,port:&mut Port<'_,'_,'_>)->Result<(),ValueError>{
 let authored=[0,1].map(|index|AuthoredTable{name:NAMES[index],declaration:definition(index),columns:[2,4][index]});validate_schema(database,&authored,port)?;port.control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;let documents=&table(database,NAMES[0])?.rows;if documents.len()!=1||documents[0].rowid!=1||documents[0].values.len()!=2||integer(&documents[0],0)?!=1{return Err(invalid("binary document identity or width"))}let document=&documents[0];let schema=match document.values.get(1){Some(SqliteValue::Text(value))=>value.as_str(),_=>return Err(invalid("Binary authored schema cell is invalid"))};let octets=&table(database,NAMES[1])?.rows;
 port.vector(octets.len(),&mut frame.ordinals,1)?;port.vector(octets.len(),&mut frame.identities,1)?;for(index,row)in octets.iter().enumerate(){port.work(1,0)?;if row.rowid<=0||row.values.len()!=4||integer(row,0)?!=row.rowid||integer(row,1)?!=1{return Err(invalid("binary octet identity or document ownership"))}port.work(1,2*std::mem::size_of::<usize>())?;frame.ordinals.push(index);frame.identities.push(index);}
 sort(&mut frame.ordinals,port,|a,b|Ok(integer(&octets[a],2)?.cmp(&integer(&octets[b],2)?)))?;sort(&mut frame.identities,port,|a,b|Ok(octets[a].rowid.cmp(&octets[b].rowid)))?;for pair in frame.identities.windows(2){port.work(1,0)?;if octets[pair[0]].rowid==octets[pair[1]].rowid{return Err(invalid("Binary octet identity is duplicated"))}}
 port.control.admit_reconstruction_bytes(schema.len())?;port.text(schema,&mut frame.snapshot.schema,1)?;port.vector(octets.len(),&mut frame.snapshot.bytes,1)?;for(position,index)in frame.ordinals.iter().copied().enumerate(){port.work(1,0)?;let row=&octets[index];if integer(row,2)?!=ordinal(position)?{return Err(invalid("relationship ordinals must be contiguous and unique"))}let value=u8::try_from(integer(row,3)?).map_err(|_|invalid("binary octet must fit u8"))?;port.control.admit_reconstruction_bytes(8)?;port.work(1,1)?;frame.snapshot.bytes.push(value);}
 let total=octets.len().checked_add(1).ok_or_else(overflow)?;port.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,total,total)
}
pub(super)fn project_receiving(source:&BinarySnapshot,control:&mut SqliteSnapshotControl<'_>,owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>)->Result<SqliteDatabase,ValueError>{let remaining=control.allocation_remaining_bytes();if remaining==0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Binary projection has no original backing allowance"))}let before=owner.native().owned_bytes();let maximum=before.checked_add(remaining).ok_or_else(overflow)?;let result=owner.scoped_native(maximum,&mut |_|true,|owner|owner.receive::<Projection,SqliteDatabase>(|slot,native,body|{let mut port=Port{control,native:Direction::Decode(native),body};port.work(1,std::mem::size_of::<Projection>())?;*slot=Some(Projection::new());let frame=slot.as_mut().unwrap();project(source,frame,&mut port)?;port.work(1,std::mem::size_of::<SqliteDatabase>())?;Ok(std::mem::replace(&mut frame.database,SqliteDatabase{tables:Vec::new()}))}));settle(control,remaining,before,owner.native().owned_bytes())?;result.map_err(|error|error.with_retained_progress(owner.progress()))}
pub(super)fn reconstruct_receiving(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut store::NativeSnapshotEncodeOwner<'_,'_>)->Result<BinarySnapshot,ValueError>{let remaining=control.allocation_remaining_bytes();if remaining==0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Binary reconstruction has no original backing allowance"))}let before=owner.native().owned_bytes();let maximum=before.checked_add(remaining).ok_or_else(overflow)?;let result=owner.scoped_native(maximum,&mut |_|true,|owner|owner.receive::<Reconstruction,BinarySnapshot>(|slot,native,body|{let mut port=Port{control,native:Direction::Encode(native),body};port.work(1,std::mem::size_of::<Reconstruction>())?;*slot=Some(Reconstruction{snapshot:empty(),ordinals:Vec::new(),identities:Vec::new()});let frame=slot.as_mut().unwrap();reconstruct(database,frame,&mut port)?;port.work(1,std::mem::size_of::<BinarySnapshot>())?;Ok(std::mem::replace(&mut frame.snapshot,empty()))}));settle(control,remaining,before,owner.native().owned_bytes())?;result.map_err(|error|error.with_retained_progress(owner.progress()))}
pub(super)fn validate_subset(dialect:&semio_framework_artifact_reference::ArtifactDialect,control:&mut SqliteSnapshotControl<'_>,checkpoint:impl FnOnce()->Result<(),ValueError>)->store::io_schema::IoResult<()>{checkpoint().map_err(store::io_schema::IoError::from_value_error)?;control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0).map_err(store::io_schema::IoError::from_value_error)?;if dialect.subset=="*"{return Ok(store::io_schema::IoOutcome::clean(()))}Err(store::io_schema::IoError::from_value_error(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"Binary subset has no authored semantic validator")))}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]mod tests;
