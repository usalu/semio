//! 🚪️ S installs exact foreign carriers for the complete original native Jack document archive.
use semio_framework_artifact_reference::{Dialect,StandardId,SubsetId};
use semio_framework_value::{FromValue,RetireOwned,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
use store::io::io_mechanism::{IoEntry,IoEntryDirection,IoRunControl};
use store::io_schema::{IoError,IoFidelity,IoOutcome,IoPayload,IoResult};
pub const JSON:Dialect=Dialect{artifact_kind:"s.stdio.json",standard:StandardId("rfc8259"),subset:SubsetId::ANY};
pub const TXT:Dialect=store::io_schema::CARRIER_TEXT;
const SCHEMA:&str="s.dev.trinity.jack.archive.v1";
const JSON_PREFIX:&str="{\"schema\":\"s.dev.trinity.jack.archive.v1\",\"archive\":\"";
const TXT_PREFIX:&str="s.dev.trinity.jack.archive.v1\n";
const HEX:&[u8;16]=b"0123456789abcdef";
const MAXIMUM_BYTES:usize=store::channel::DOCUMENT_ARCHIVE_MAXIMUM_BYTES;
#[derive(FromValue,RetireOwned)]
#[value(deny_unknown_fields)]
struct Carrier{schema:String,archive:String}
#[derive(RetireOwned)]
struct Receiving{carrier:Option<Carrier>,output:Option<IoPayload>}
#[derive(Clone,Copy)]
enum Format{Json,Text}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn allocation()->ValueError{ValueError::literal(ValueRefusalKind::AllocationFailed,"Jack archive carrier allocation failed")}
fn nibble(value:u8)->Result<u8,ValueError>{match value{b'0'..=b'9'=>Ok(value-b'0'),b'a'..=b'f'=>Ok(value-b'a'+10),_=>Err(invalid("Jack archive requires lowercase hexadecimal octets"))}}
fn extent(bytes:&[u8])->Result<(),ValueError>{if bytes.is_empty()||bytes.len()>MAXIMUM_BYTES||bytes[0]!=store::channel::DOCUMENT_ARCHIVE_VERSION{return Err(invalid("Jack requires the original versioned native document archive"))}Ok(())}
fn export(payload:&IoPayload,format:Format,control:&mut IoRunControl<'_,'_>)->IoResult<IoPayload>{
 let IoPayload::Binary(bytes)=payload else{return Err(invalid("Jack archive export requires native binary archive").into())};extent(bytes).map_err(IoError::from_value_error)?;
 let(prefix,suffix)=match format{Format::Json=>(JSON_PREFIX,"\"}"),Format::Text=>(TXT_PREFIX,"\n")};let length=bytes.len()*2+prefix.len()+suffix.len();
 control.snapshot_encode().map_err(IoError::from_value_error)?.receive::<IoPayload,IoPayload>(|slot,native,body|{
  body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:length,maximum_capacity_bytes:length,maximum_release_bytes:0,maximum_depth:2})?;
  *slot=Some(IoPayload::Text(String::new()));let Some(IoPayload::Text(output))=slot else{unreachable!()};
  native.begin_stage(bytes.len())?;native.charge(length)?;output.try_reserve_exact(length).map_err(|_|allocation())?;output.push_str(prefix);
  let result=(||{for chunk in bytes.chunks(256){for byte in chunk{output.push(HEX[usize::from(byte>>4)]as char);output.push(HEX[usize::from(byte&15)]as char);}native.advance(chunk.len())?;}output.push_str(suffix);Ok(())})();
  body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:output.len(),retained_capacity_bytes:output.capacity(),released_bytes:0})?;result?;Ok(slot.take().unwrap())
 }).map(IoOutcome::clean).map_err(IoError::from_value_error)
}
fn import(payload:&IoPayload,format:Format,control:&mut IoRunControl<'_,'_>)->IoResult<IoPayload>{
 let text=match payload{IoPayload::Text(text)=>text.as_str(),IoPayload::Binary(bytes)=>{if bytes.len()>MAXIMUM_BYTES*2+128{return Err(invalid("Jack archive text extent exceeds authored ceiling").into())}std::str::from_utf8(bytes).map_err(|_|IoError::from_value_error(invalid("Jack archive carrier requires UTF8")))?}};
 if text.len()>MAXIMUM_BYTES*2+128{return Err(invalid("Jack archive text extent exceeds authored ceiling").into())}
 control.snapshot_decode().map_err(IoError::from_value_error)?.receive::<Receiving,IoPayload>(|slot,native,body|{
  *slot=Some(Receiving{carrier:None,output:None});let frame=slot.as_mut().unwrap();
  let encoded=match format{Format::Json=>{
   body.admit_frontier(RetainedCloneGrant{maximum_items:3,maximum_copy_bytes:text.len()*8,maximum_capacity_bytes:text.len()*8,maximum_release_bytes:text.len()*8,maximum_depth:8})?;
   frame.carrier=Some(semio_framework_pack_json::from_json_str_controlled::<Carrier>(text,semio_framework_pack_json::JsonMemberPolicy::Reject,native)?);
   let carrier=frame.carrier.as_ref().unwrap();body.record_progress(RetainedCloneProgress{copied_items:3,copied_bytes:carrier.schema.len()+carrier.archive.len(),retained_capacity_bytes:carrier.schema.capacity()+carrier.archive.capacity(),released_bytes:0})?;
   let carrier=frame.carrier.as_ref().unwrap();if carrier.schema!=SCHEMA{return Err(invalid("Jack archive schema identifier refused"))}carrier.archive.as_str()
  },Format::Text=>text.strip_prefix(TXT_PREFIX).and_then(|text|text.strip_suffix('\n')).ok_or_else(||invalid("Jack archive text framing refused"))?};
  if encoded.is_empty()||encoded.len()%2!=0||encoded.len()>MAXIMUM_BYTES*2{return Err(invalid("Jack archive hexadecimal extent refused"))}
  let count=encoded.len()/2;body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:count,maximum_capacity_bytes:count,maximum_release_bytes:0,maximum_depth:2})?;
  frame.output=Some(IoPayload::Binary(Vec::new()));let Some(IoPayload::Binary(output))=&mut frame.output else{unreachable!()};
  native.begin_stage(encoded.len())?;native.charge(count)?;output.try_reserve_exact(count).map_err(|_|allocation())?;
  let result=(||{for chunk in encoded.as_bytes().chunks(512){for pair in chunk.chunks_exact(2){output.push((nibble(pair[0])?<<4)|nibble(pair[1])?);}native.advance(chunk.len())?;}extent(output)})();
  body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:output.len(),retained_capacity_bytes:output.capacity(),released_bytes:0})?;result?;Ok(frame.output.take().unwrap())
 }).map(IoOutcome::clean).map_err(IoError::from_value_error)
}
#[path="📤️export/🔣️json/🦀️.rs"]mod into_json;
#[path="📥️import/🔣️json/🦀️.rs"]mod from_json;
#[path="📤️export/🔤️txt/🦀️.rs"]mod into_txt;
#[path="📥️import/🔤️txt/🦀️.rs"]mod from_txt;
pub fn entries()->&'static[IoEntry]{static ENTRIES:[IoEntry;4]=[
 IoEntry{owned_serializer:None,from:super::jack::TRINITY_JACK_DIALECT,into:JSON,fidelity:IoFidelity::Exact,direction:IoEntryDirection::Export,sniff:None,run:into_json::run},
 IoEntry{owned_serializer:None,from:JSON,into:super::jack::TRINITY_JACK_DIALECT,fidelity:IoFidelity::Exact,direction:IoEntryDirection::Import,sniff:None,run:from_json::run},
 IoEntry{owned_serializer:None,from:super::jack::TRINITY_JACK_DIALECT,into:TXT,fidelity:IoFidelity::Exact,direction:IoEntryDirection::Export,sniff:None,run:into_txt::run},
 IoEntry{owned_serializer:None,from:TXT,into:super::jack::TRINITY_JACK_DIALECT,fidelity:IoFidelity::Exact,direction:IoEntryDirection::Import,sniff:None,run:from_txt::run}];&ENTRIES}
