//! 🧫️ Individually authored Count relational provider under the same caller control.
use super::Snapshot;
use semio_framework_os_kernel as store;
use semio_framework_os_kernel::os_pack as pack;
use store::sqlite_snapshot::{*, artifact::{Cell, Projection}};
use semio_framework_value::{NativeDecodeControl, NativeEncodeControl};
const SQL: &str = include_str!("🗄️.sql");
fn invalid(message: &'static str) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, message) }
fn decimal(value: i32, bytes: &mut [u8;11]) -> &str {
    let mut magnitude = value.unsigned_abs(); let mut start = bytes.len();
    loop { start -= 1; bytes[start] = b'0' + (magnitude % 10) as u8; magnitude /= 10; if magnitude == 0 { break; } }
    if value < 0 { start -= 1; bytes[start] = b'-'; }
    std::str::from_utf8(&bytes[start..]).expect("decimal i32 contains only ASCII")
}
fn fixed_deflate_bound(bytes: usize) -> usize { (bytes * 9 + 10).div_ceil(8) }
fn compressed_segment_bound(raw: usize) -> usize { 2 + 20 + fixed_deflate_bound(raw) + 4 }
fn binary_bound() -> usize {
    let manifest = 1 + 32 + 20 + 10 + 20 + 20 + 20 + 10 + 10 + 10 + 10;
    pack::format::HEADER_SIZE + compressed_segment_bound(1) + compressed_segment_bound(10)
        + compressed_segment_bound(manifest) + 2 + 10 + 4 + pack::format::FOOTER_SIZE
}
fn bound(source: &Snapshot, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
    control.checkpoint(SqliteSnapshotPhase::EncodeNative, 0, 1)?;
    if SQL.len()>control.limits().max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Count native forecast exceeds the authored schema allowance"));}
    control.check_rows(1)?;
    let mut digits = [0;11];
    let bytes = match encoding { SnapshotEncoding::Text => 10 + decimal(source.count, &mut digits).len(), SnapshotEncoding::Binary => binary_bound() };
    if bytes > control.limits().max_file_bytes { return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "Count native forecast exceeds the file ceiling")); }
    if bytes > control.allocation_remaining_bytes() { return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "Count prospective native output exceeds the caller allowance")); }
    control.check_value_bytes(8)?;
    control.checkpoint(SqliteSnapshotPhase::EncodeNative, 1, 1)
}
impl store::ArtifactSqliteSnapshot for Snapshot {
    const SQLITE_SCHEMA: &'static str = SQL;
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase,ValueError> {
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?; control.check_rows(1)?; control.check_value_bytes(16)?;
        let mut projection=Projection::new(SQL,control)?;
        projection.insert_key("fixture_counter",1,&[Cell::Integer(i64::from(self.count))])?;
        projection.checkpoint_total(1)?; projection.finish()
    }
    fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,1)?;
        control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
        validate_sqlite_database_schema_controlled(database,SQL,SqliteSnapshotPhase::ReconstructSnapshot,control)?;
        let row=database.table("fixture_counter")?.single_row()?;
        if row.rowid!=1 || row.values.len()!=2 || row.integer(0)?!=1 { return Err(invalid("Count requires the literal singleton entity identity")); }
        let count=i32::try_from(row.integer(1)?).map_err(|_|invalid("Count is outside the authored i32 domain"))?;
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?; Ok(Self{count})
    }
    fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,ValueError> {
 let original=native_owner.native();
        control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,1)?; control.check_rows(1)?; control.check_value_bytes(8)?;
        let length=match payload{store::io::IoPayload::Text(text)=>text.len(),store::io::IoPayload::Binary(bytes)=>bytes.len()};
        if length>control.limits().max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Count native input exceeds the file ceiling"));}
        control.allocation_stage_native(SqliteSnapshotPhase::DecodeNative,|remaining,checkpoint|{
            let mut progress=|event:semio_framework_value::native_decoding::NativeDecodeProgress|checkpoint(event.completed,event.total);
            let before=original.owned_bytes();let maximum_owned=before.checked_add(remaining).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"fixture native cumulative allowance overflow"));
            let result=maximum_owned.and_then(|maximum|original.scoped_maximum(maximum,|native|native.scoped_observer(&mut progress,|native|{
                match payload {
                    store::io::IoPayload::Text(text)=>{
                        let empty=native.scoped_stage(|native|{
                            native.begin_stage(text.len())?;let mut empty=true;
                            for character in text.chars(){empty&=character.is_whitespace();native.advance(character.len_utf8())?;}
                            Ok::<_,ValueError>(empty)
                        })?;
                        if empty { Ok(Self::default()) }
                        else { semio_framework_pack_json::from_json_str_controlled(text,semio_framework_pack_json::JsonMemberPolicy::Reject,native) }
                    }
                    store::io::IoPayload::Binary(bytes)=>{
                        let spec=Self::__dsl_spec_producer().decode(native)?;
                        let (record,_)=pack::decode_document_controlled(bytes,&spec,&store::PackDecodeOptions::default(),native).map_err(store::PackRefusal::into_value_error)?;
                        Self::__dsl_from_record_controlled(&record,native)
                    }
                }
            })));
            (result,original.owned_bytes()-before)
        })?
    }
    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,owner:&mut store::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io::IoPayload,ValueError>{
        bound(self,encoding,control)?;
        let original=owner.native();
        control.allocation_stage_native(SqliteSnapshotPhase::EncodeNative,|remaining,checkpoint|{
            let mut progress=|event:semio_framework_value::native_encoding::NativeEncodeProgress|checkpoint(event.completed,event.total);
            let before=original.owned_bytes();let maximum_owned=before.checked_add(remaining).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"fixture native cumulative allowance overflow"));
            let result=maximum_owned.and_then(|maximum|original.scoped_maximum(maximum,|native|native.scoped_observer(&mut progress,|native|{
                match encoding{
                    SnapshotEncoding::Text=>{
                        let mut digits=[0;11];let digits=decimal(self.count,&mut digits);let length=10+digits.len();
                        native.begin_stage(length)?;native.charge(length)?;let mut output=String::new();
                        output.try_reserve_exact(length).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"Count JSON output allocation failed"))?;
                        for text in ["{\"count\":",digits,"}"]{output.push_str(text);native.advance(text.len())?;}
                        Ok(store::io::IoPayload::Text(output))
                    }
                    SnapshotEncoding::Binary=>{
                        let spec=Self::__dsl_spec_producer().encode(native)?;
                        let record=semio_framework_dsl_record::native_encoding::EncodedRecord::from_record(self.__dsl_to_record_controlled(native)?);
                        pack::encode_document_controlled(&spec,record.as_record(),&store::PackEncodeOptions::default(),native).map(store::io::IoPayload::Binary).map_err(store::PackRefusal::into_value_error)
                    }
                }
            })));
            (result,original.owned_bytes()-before)
        })?
    }
    fn preflight_sqlite_snapshot_encoding(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{bound(self,encoding,control)}
}
