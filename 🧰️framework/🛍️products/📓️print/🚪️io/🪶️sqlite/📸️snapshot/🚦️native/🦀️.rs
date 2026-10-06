//! 🚦️ Exact Chart Record envelopes and same-caller controlled native ownership.
use crate::ChartSnapshot;
use protocol as store;
use semio_framework_dsl_record::{BorrowedFieldSpec as F,BorrowedRecordSpec as R,BorrowedShape as H,RecordLayout,__rt::DecodedFieldOwner};
use semio_framework_value::{FromValue,NativeEncodeControl,ValueError,ValueRefusalKind as K,native_encoding::NativeEncodeProgress};
use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl,SqliteSnapshotPhase};
use semio_framework_diagnostic::{TextError,TextSpan};
static FIELD:[F;1]=[F::new(0,"chart",H::Value)];
fn spec()->R{R{keyword:None,layout:RecordLayout::Inline,fields:&FIELD}}
pub(super) fn decode(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<ChartSnapshot,ValueError>{let mut owner=DecodedFieldOwner::new(store::decode_sqlite_snapshot_record_native(payload,"print.chart",ChartSnapshot::__dsl_spec_producer(),|record,native|ChartSnapshot::__dsl_from_record_controlled(record,native),control)?,ChartSnapshot::retire_decoded);super::forecast(owner.as_mut(),control,SqliteSnapshotPhase::DecodeNative)?;Ok(owner.take())}
pub(super) fn preflight(snapshot:&ChartSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{super::forecast(snapshot,control,SqliteSnapshotPhase::EncodeNative)?;let component=match encoding{SnapshotEncoding::Binary=>store::semio_format::Component::Pack,SnapshotEncoding::Text=>store::semio_format::Component::Dsl};let maximum=control.limits().max_file_bytes.checked_sub(store::semio_format::declared_envelope_prefix_len("print.chart",component,1)?).ok_or_else(||ValueError::new(K::OwnershipLimit,"Chart file ceiling cannot contain its declared envelope"))?;
 control.allocation_stage(SqliteSnapshotPhase::EncodeNative,|remaining,checkpoint|{let mut progress=|event:NativeEncodeProgress|checkpoint(event.completed,event.total);let mut native=NativeEncodeControl::new(remaining,&mut progress);let result=match encoding{SnapshotEncoding::Text=>semio_framework_dsl_record::measure_print_borrowed(snapshot,&spec(),maximum,&mut native),SnapshotEncoding::Binary=>pack::record::measure_document_borrowed(snapshot,&spec(),&Default::default(),&mut native)}.and_then(|length|if length>maximum{Err(ValueError::new(K::OwnershipLimit,"Chart native output exceeds file byte ceiling"))}else{Ok(())});(result,native.owned_bytes())})?
}
pub(super) fn encode(snapshot:&ChartSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{preflight(snapshot,encoding,control)?;store::encode_sqlite_snapshot_record_native(encoding,"print.chart",ChartSnapshot::__dsl_spec_producer(),|native|snapshot.__dsl_to_record_controlled(native),control)}
