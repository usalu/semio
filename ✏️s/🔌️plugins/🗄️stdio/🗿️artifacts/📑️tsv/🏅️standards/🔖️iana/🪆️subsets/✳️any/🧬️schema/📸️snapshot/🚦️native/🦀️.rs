//! 🚦️ Complete logical TSV fields borrow exact forecasts and use the shared paid Record producer.
use super::TsvSnapshot;
use crate::store;
use semio_framework_dsl_record::{BorrowedFieldSpec as F,BorrowedRecordSpec as R,BorrowedShape as H,RecordLayout,__rt::DecodedFieldOwner};
use semio_framework_value::{FromValue,NativeEncodeControl,ValueError,ValueRefusalKind as K,native_encoding::NativeEncodeProgress};
use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl,SqliteSnapshotPhase};
fn text()->H{H::Text}
fn row_shape()->H{H::List(text)}
static SNAPSHOT:[F;4]=[F::new(0,"schema",H::Text),F::new(1,"records",H::List(row_shape)),F::new(2,"trailing-newline",H::Bool),F::new(3,"line-ending",H::Enum(&[("lf",0),("crlf",1)]))];
fn spec()->R{R{keyword:None,layout:RecordLayout::Inline,fields:&SNAPSHOT}}
pub(super) fn decode(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<TsvSnapshot,ValueError>{let mut owner=DecodedFieldOwner::new(store::decode_sqlite_snapshot_record_native(payload,"stdio.tsv",TsvSnapshot::__dsl_spec_producer(),|record,native|TsvSnapshot::__dsl_from_record_controlled(record,native),control)?,<TsvSnapshot as FromValue>::retire_decoded);super::sqlite::forecast(owner.as_mut(),control,SqliteSnapshotPhase::DecodeNative)?;Ok(owner.take())}
pub(super) fn preflight(snapshot:&TsvSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 super::sqlite::forecast(snapshot,control,SqliteSnapshotPhase::EncodeNative)?;let limits=control.limits();let component=match encoding{SnapshotEncoding::Binary=>store::semio_format::Component::Pack,SnapshotEncoding::Text=>store::semio_format::Component::Dsl};let maximum=limits.max_file_bytes.checked_sub(store::semio_format::declared_envelope_prefix_len("stdio.tsv",component,1)?).ok_or_else(||ValueError::new(K::OwnershipLimit,"TSV file ceiling cannot contain its declared envelope"))?;
 control.allocation_stage(SqliteSnapshotPhase::EncodeNative,|remaining,checkpoint|{let mut progress=|event:NativeEncodeProgress|checkpoint(event.completed,event.total);let mut native=NativeEncodeControl::new(remaining,&mut progress);let result=match encoding{SnapshotEncoding::Text=>semio_framework_dsl_record::measure_print_borrowed(snapshot,&spec(),usize::MAX,&mut native),SnapshotEncoding::Binary=>{let options=pack::record::EncodeOptions::default();pack::record::measure_document_borrowed(snapshot,&spec(),&options,&mut native)}}.and_then(|length|if length>maximum{Err(ValueError::new(K::OwnershipLimit,"TSV exact native output exceeds file byte ceiling"))}else{Ok(())});(result,native.owned_bytes())})?
}
pub(super) fn encode(snapshot:&TsvSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{preflight(snapshot,encoding,control)?;store::encode_sqlite_snapshot_record_native(encoding,"stdio.tsv",TsvSnapshot::__dsl_spec_producer(),|native|snapshot.__dsl_to_record_controlled(native),control)}
