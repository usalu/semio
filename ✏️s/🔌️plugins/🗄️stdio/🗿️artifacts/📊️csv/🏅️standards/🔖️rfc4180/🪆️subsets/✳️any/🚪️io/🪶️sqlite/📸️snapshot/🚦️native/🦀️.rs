//! 🚦️ Complete logical CSV fields borrow exact forecasts and use the shared paid Record producer.
use crate::standards::v_rfc4180::subsets::any::schema::snapshot::CsvSnapshot;
use crate::store;
use semio_framework_dsl_record::{BorrowedFieldSpec as F,BorrowedRecordSpec as R,BorrowedShape as H,RecordLayout};
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind as K,native_encoding::NativeEncodeProgress};
use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl,SqliteSnapshotPhase};
fn text()->H{H::Text}
static FIELD:[F;2]=[F::new(0,"value",H::Text),F::new(1,"quoted",H::Bool)];
fn field()->R{R{keyword:None,layout:RecordLayout::Inline,fields:&FIELD}}
fn field_shape()->H{H::Record(field)}
static ROW:[F;1]=[F::new(0,"fields",H::List(field_shape))];
fn row()->R{R{keyword:None,layout:RecordLayout::Inline,fields:&ROW}}
fn row_shape()->H{H::Record(row)}
static SNAPSHOT:[F;3]=[F::new(0,"schema",H::Text),F::new(1,"has-header",H::Bool),F::new(2,"records",H::List(row_shape))];
fn spec()->R{R{keyword:None,layout:RecordLayout::Inline,fields:&SNAPSHOT}}
pub(crate)fn decode(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<CsvSnapshot,ValueError>{let limits=control.limits();store::decode_sqlite_snapshot_record_native(payload,"stdio.csv",CsvSnapshot::__dsl_spec_producer(),|record,snapshot_output,native,body|{crate::standards::v_rfc4180::subsets::any::io::sqlite::snapshot::admission::admit(record,native,limits)?;crate::standards::v_rfc4180::subsets::any::io::sqlite::snapshot::admission::bind(record,snapshot_output,native,body)},control,native_control)}
pub(crate) fn preflight(snapshot:&CsvSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 crate::standards::v_rfc4180::subsets::any::io::sqlite::snapshot::semantic(snapshot,control,SqliteSnapshotPhase::EncodeNative)?;let limits=control.limits();let component=match encoding{SnapshotEncoding::Binary=>store::semio_format::Component::Pack,SnapshotEncoding::Text=>store::semio_format::Component::Dsl};let maximum=limits.max_file_bytes.checked_sub(store::semio_format::declared_envelope_prefix_len("stdio.csv",component,1)?).ok_or_else(||ValueError::new(K::OwnershipLimit,"CSV file ceiling cannot contain its declared envelope"))?;
 control.allocation_stage(SqliteSnapshotPhase::EncodeNative,|remaining,checkpoint,allocation|{let mut progress=|event:NativeEncodeProgress|checkpoint(event.completed,event.total);let mut native_allocation=|request:semio_framework_value::native_encoding::NativeEncodeAllocation|allocation(request.bytes);let mut native=NativeEncodeControl::new_forwarded(remaining,&mut progress,&mut native_allocation);let result=match encoding{SnapshotEncoding::Text=>semio_framework_dsl_record::measure_print_borrowed(snapshot,&spec(),usize::MAX,&mut native),SnapshotEncoding::Binary=>{let options=pack::record::EncodeOptions::default();pack::record::measure_document_borrowed(snapshot,&spec(),&options,&mut native)}}.and_then(|length|if length>maximum{Err(ValueError::new(K::OwnershipLimit,"CSV exact native output exceeds file byte ceiling"))}else{Ok(())});(result,native.owned_bytes())})?
}
pub(crate) fn encode(snapshot:&CsvSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{preflight(snapshot,encoding,control)?;store::encode_sqlite_snapshot_record_native(encoding,"stdio.csv",CsvSnapshot::__dsl_spec_producer(),|native|snapshot.__dsl_to_record_controlled(native),control,native_owner)}
