//! 🚪️ Canonical foreign-format routes use the original native snapshot receiver.
use semio_os_kernel::io::io_mechanism::{ArchiveChildren,Serializer,Deserializer,IoEntry,IoRunControl,serializer_entry,deserializer_entry};
use semio_framework_artifact_reference::{Dialect,StandardId,SubsetId};
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework::io_schema::{IoPayload,IoOutcome,IoError,IoFidelity,IoResult};
use std::sync::OnceLock;
use crate::Puzzle2dSnapshot as Snapshot;

const NATIVE:Dialect=Dialect{artifact_kind:"s.puzzle.puzzle2d",standard:StandardId("1"),subset:SubsetId::ANY};
fn failure(error:semio_framework_diagnostic::TextError)->IoError{IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))}
fn text(bytes:Vec<u8>)->IoResult<IoPayload>{String::from_utf8(bytes).map(|text|IoOutcome::clean(IoPayload::Text(text))).map_err(|_|IoError::from_value_error(ValueError::literal(ValueRefusalKind::InvalidValue,"declared text format must produce UTF8")))}
fn bytes(payload:&IoPayload)->&[u8]{match payload{IoPayload::Binary(bytes)=>bytes,IoPayload::Text(text)=>text.as_bytes()}}

const DWG:Dialect=Dialect{artifact_kind:"s.stdio.dwg",standard:StandardId("ac1024"),subset:SubsetId::ANY};
pub struct DwgFormat;
impl Serializer<Snapshot> for DwgFormat{
 const INTO:Dialect=DWG;
 const FIDELITY:IoFidelity=IoFidelity::Lossy;
 async fn serialize(from:&Snapshot,_:&ArchiveChildren,control:&mut IoRunControl<'_,'_>)->IoResult<IoPayload>{control.checkpoint().map_err(IoError::from_value_error)?;crate::standards::v1::subsets::any::io::export::serializers::artifacts::dwg::v_ac1024::any::serialize_bytes(from).map(|bytes|IoOutcome::clean(IoPayload::Binary(bytes))).map_err(failure)}
}
const DXF:Dialect=Dialect{artifact_kind:"s.stdio.dxf",standard:StandardId("r12"),subset:SubsetId::ANY};
pub struct DxfFormat;
impl Serializer<Snapshot> for DxfFormat{
 const INTO:Dialect=DXF;
 const FIDELITY:IoFidelity=IoFidelity::Lossy;
 async fn serialize(from:&Snapshot,_:&ArchiveChildren,control:&mut IoRunControl<'_,'_>)->IoResult<IoPayload>{control.checkpoint().map_err(IoError::from_value_error)?;crate::standards::v1::subsets::any::io::export::serializers::artifacts::dxf::v_r12::any::serialize_bytes(from).map_err(failure).and_then(text)}
}
const JSON:Dialect=Dialect{artifact_kind:"s.stdio.json",standard:StandardId("rfc8259"),subset:SubsetId::ANY};
pub struct JsonFormat;
impl Serializer<Snapshot> for JsonFormat{
 const INTO:Dialect=JSON;
 const FIDELITY:IoFidelity=IoFidelity::Exact;
 async fn serialize(from:&Snapshot,_:&ArchiveChildren,control:&mut IoRunControl<'_,'_>)->IoResult<IoPayload>{control.checkpoint().map_err(IoError::from_value_error)?;crate::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(from).map_err(failure).and_then(text)}
}
impl Deserializer<Snapshot> for JsonFormat{
 const FROM:Dialect=JSON;
 const FIDELITY:IoFidelity=IoFidelity::Exact;
 async fn deserialize(payload:&IoPayload,control:&mut IoRunControl<'_,'_>)->IoResult<Snapshot>{control.checkpoint().map_err(IoError::from_value_error)?;crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(bytes(payload)).map(IoOutcome::clean).map_err(failure)}
}
const PDF:Dialect=Dialect{artifact_kind:"s.stdio.pdf",standard:StandardId("1.4"),subset:SubsetId::ANY};
pub struct PdfFormat;
impl Serializer<Snapshot> for PdfFormat{
 const INTO:Dialect=PDF;
 const FIDELITY:IoFidelity=IoFidelity::Lossy;
 async fn serialize(from:&Snapshot,_:&ArchiveChildren,control:&mut IoRunControl<'_,'_>)->IoResult<IoPayload>{control.checkpoint().map_err(IoError::from_value_error)?;crate::standards::v1::subsets::any::io::export::serializers::artifacts::pdf::v1_4::any::serialize_bytes(from).map(|bytes|IoOutcome::clean(IoPayload::Binary(bytes))).map_err(failure)}
}
const PNG:Dialect=Dialect{artifact_kind:"s.stdio.png",standard:StandardId("1.2"),subset:SubsetId::ANY};
pub struct PngFormat;
impl Serializer<Snapshot> for PngFormat{
 const INTO:Dialect=PNG;
 const FIDELITY:IoFidelity=IoFidelity::Lossy;
 async fn serialize(from:&Snapshot,_:&ArchiveChildren,control:&mut IoRunControl<'_,'_>)->IoResult<IoPayload>{control.checkpoint().map_err(IoError::from_value_error)?;crate::standards::v1::subsets::any::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(from).map(|bytes|IoOutcome::clean(IoPayload::Binary(bytes))).map_err(failure)}
}
const SVG:Dialect=Dialect{artifact_kind:"s.stdio.svg",standard:StandardId("1.1"),subset:SubsetId::ANY};
pub struct SvgFormat;
impl Serializer<Snapshot> for SvgFormat{
 const INTO:Dialect=SVG;
 const FIDELITY:IoFidelity=IoFidelity::Lossy;
 async fn serialize(from:&Snapshot,_:&ArchiveChildren,control:&mut IoRunControl<'_,'_>)->IoResult<IoPayload>{control.checkpoint().map_err(IoError::from_value_error)?;crate::standards::v1::subsets::any::io::export::serializers::artifacts::svg::v1_1::any::serialize_bytes(from).map_err(failure).and_then(text)}
}
const TXT:Dialect=Dialect{artifact_kind:"s.stdio.txt",standard:StandardId("utf-8"),subset:SubsetId::ANY};
pub struct TxtFormat;
impl Serializer<Snapshot> for TxtFormat{
 const INTO:Dialect=TXT;
 const FIDELITY:IoFidelity=IoFidelity::Exact;
 async fn serialize(from:&Snapshot,_:&ArchiveChildren,control:&mut IoRunControl<'_,'_>)->IoResult<IoPayload>{control.checkpoint().map_err(IoError::from_value_error)?;crate::standards::v1::subsets::any::io::export::serializers::artifacts::txt::v_utf_8::any::serialize_bytes(from).map_err(failure).and_then(text)}
}
impl Deserializer<Snapshot> for TxtFormat{
 const FROM:Dialect=TXT;
 const FIDELITY:IoFidelity=IoFidelity::Exact;
 async fn deserialize(payload:&IoPayload,control:&mut IoRunControl<'_,'_>)->IoResult<Snapshot>{control.checkpoint().map_err(IoError::from_value_error)?;crate::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(bytes(payload)).map(IoOutcome::clean).map_err(failure)}
}

pub fn entries()->&'static [IoEntry]{
 static ENTRIES:OnceLock<Vec<IoEntry>>=OnceLock::new();
 ENTRIES.get_or_init(||vec![
  deserializer_entry::<Snapshot,JsonFormat>(NATIVE),
  deserializer_entry::<Snapshot,TxtFormat>(NATIVE),
  serializer_entry::<Snapshot,DwgFormat>(NATIVE),
  serializer_entry::<Snapshot,DxfFormat>(NATIVE),
  serializer_entry::<Snapshot,JsonFormat>(NATIVE),
  serializer_entry::<Snapshot,PdfFormat>(NATIVE),
  serializer_entry::<Snapshot,PngFormat>(NATIVE),
  serializer_entry::<Snapshot,SvgFormat>(NATIVE),
  serializer_entry::<Snapshot,TxtFormat>(NATIVE),
]).as_slice()
}
