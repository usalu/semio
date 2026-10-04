//! 🎧️ Handcrafted WAV primary chunks, samples, extension octets and chunk relationships.
use super::*;
use semio_framework_os_kernel::sqlite_snapshot::{ValueError, ValueRefusalKind};

fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn work(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::WorkLimit,message)}
fn ownership(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::OwnershipLimit,message)}
use semio_framework_os_kernel::{
    sqlite_snapshot::{
        artifact::{reconstruct_text, Cell, Projection},
        transfer::{heap_sort,reserve}, validate_sqlite_database_schema_controlled, SnapshotEncoding, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue,
    },
    ArtifactSqliteSnapshot,
};

struct UnsignedWord{bytes:[u8;20],start:usize}
impl UnsignedWord{
    fn new(mut value:u64)->Self{let mut word=Self{bytes:[0;20],start:20};loop{word.start-=1;word.bytes[word.start]=b'0'+(value%10)as u8;value/=10;if value==0{return word;}}}
    fn text(&self)->&str{std::str::from_utf8(&self.bytes[self.start..]).expect("decimal digits are UTF-8")}
}

fn unsigned(row: &SqliteRow, column: usize) -> Result<u32, ValueError> {
    u32::try_from(row.integer(column)?).map_err(|error|invalid(error.to_string()))
}
fn short(row: &SqliteRow, column: usize) -> Result<u16, ValueError> {
    u16::try_from(row.integer(column)?).map_err(|error|invalid(error.to_string()))
}
fn octet(row: &SqliteRow, column: usize) -> Result<u8, ValueError> {
    u8::try_from(row.integer(column)?).map_err(|error|invalid(error.to_string()))
}
fn checkpoint(control: &mut SqliteSnapshotControl<'_>, position: usize, total: usize) -> Result<(), ValueError> {
    if position % 256 == 0 {
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, position, total)?;
    }
    Ok(())
}
fn entities<'a>(database: &'a SqliteDatabase, table: &str, control: &mut SqliteSnapshotControl<'_>) -> Result<Vec<&'a SqliteRow>, ValueError> {
    let rows = &database.table(table)?.rows;
    let mut keys = reserve(rows.len(),control)?;
    for (position, row) in rows.iter().enumerate() {
        checkpoint(control, position, rows.len())?;
        if row.rowid <= 0 || row.integer(0)? != row.rowid {
            return Err(invalid("WAV entities require unique positive aliased identifiers"));
        }
        keys.push(row);
    }
    heap_sort(&mut keys,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,_|Ok(a.rowid.cmp(&b.rowid)))?;
    for(position,pair)in keys.windows(2).enumerate(){checkpoint(control,position,keys.len())?;if pair[0].rowid==pair[1].rowid{return Err(invalid("WAV entities require unique positive aliased identifiers"));}}
    Ok(keys)
}
fn ordered<'a>(database: &'a SqliteDatabase, table: &str, control: &mut SqliteSnapshotControl<'_>) -> Result<Vec<&'a SqliteRow>, ValueError> {
    let mut rows = entities(database, table, control)?;
    heap_sort(&mut rows,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,_|Ok(a.integer(2)?.cmp(&b.integer(2)?)))?;
    for (position, row) in rows.iter().enumerate() {
        checkpoint(control, position, rows.len())?;
        if row.integer(1)? != 1 {
            return Err(invalid("WAV primary child has an unknown owner"));
        }
        let ordinal = usize::try_from(row.integer(2)?).map_err(|error|invalid(error.to_string()))?;
        if ordinal!=position{return Err(invalid("WAV ordinals must be contiguous and unique"));}
    }
    Ok(rows)
}
fn groups<'a>(mut rows:Vec<&'a SqliteRow>, parents:&[i64], control:&mut SqliteSnapshotControl<'_>)->Result<Vec<&'a SqliteRow>,ValueError>{
    heap_sort(&mut rows,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,_|Ok(a.integer(1)?.cmp(&b.integer(1)?).then(a.integer(2)?.cmp(&b.integer(2)?))))?;
    let mut previous=None;let mut ordinal=0usize;
    for (position, row) in rows.iter().enumerate() {
        checkpoint(control, position, rows.len())?;
        let owner = row.integer(1)?;
        if parents.binary_search(&owner).is_err() {
            return Err(invalid("WAV octet has an unknown chunk owner"));
        }
        if previous!=Some(owner){previous=Some(owner);ordinal=0;}
        if usize::try_from(row.integer(2)?).ok()!=Some(ordinal){return Err(invalid("WAV octet ordinals must be contiguous and unique"));}
        ordinal=ordinal.checked_add(1).ok_or_else(||work("WAV octet ordinal overflow"))?;
    }
    Ok(rows)
}
fn owner_rows<'a,'b>(rows:&'b[&'a SqliteRow],owner:i64)->&'b[&'a SqliteRow]{let start=rows.partition_point(|row|matches!(row.values.get(1),Some(SqliteValue::Integer(value))if *value<owner));let end=rows.partition_point(|row|matches!(row.values.get(1),Some(SqliteValue::Integer(value))if *value<=owner));&rows[start..end]}
fn write_octets(out: &mut Projection<'_, '_>, table: &str, owner: i64, bytes: &[u8]) -> Result<(), ValueError> {
    for (ordinal, byte) in bytes.iter().enumerate() {
        out.insert(table, &[Cell::Integer(owner), Cell::Integer(ordinal as i64), Cell::Integer((*byte).into())])?;
    }
    Ok(())
}
fn read_octets(rows:&[&SqliteRow], control: &mut SqliteSnapshotControl<'_>) -> Result<Vec<u8>, ValueError> {
    let mut bytes = reserve(rows.len(),control)?;
    for row in rows {
        checkpoint(control, bytes.len(), 0)?;
        bytes.push(octet(row, 3)?);
    }
    Ok(bytes)
}
fn numeric_class(value: f32) -> &'static str {
    if value.is_nan() {
        "nan"
    } else if value == f32::INFINITY {
        "positive_infinity"
    } else if value == f32::NEG_INFINITY {
        "negative_infinity"
    } else if value == 0.0 && value.is_sign_negative() {
        "negative_zero"
    } else {
        "finite"
    }
}
fn read_float(row: &SqliteRow) -> Result<f32, ValueError> {
    let sample = f32::from_bits(unsigned(row, 3)?);
    if row.text(4)? != numeric_class(sample) {
        return Err(invalid("WAV sample numeric class disagrees with its IEEE754 bits"));
    }
    if sample.is_finite() {
        let value=f64::from(sample);
        let exact=match row.values.get(5){Some(SqliteValue::Real(query))=>*query==value,Some(SqliteValue::Integer(query))=>value>=i64::MIN as f64&&value < -(i64::MIN as f64)&&value.fract()==0.0&&value as i64==*query,_=>false};
        if !exact {
            return Err(invalid("WAV query sample disagrees with its IEEE754 bits"));
        }
    } else if !matches!(row.values.get(5), Some(SqliteValue::Null)) {
        return Err(invalid("WAV non-finite sample requires an explicit class and NULL query value"));
    }
    Ok(sample)
}

fn add_rows(total:usize,count:usize)->Result<usize,ValueError>{total.checked_add(count).ok_or_else(||work("WAV semantic row count overflow"))}
fn admit_schema(control:&SqliteSnapshotControl<'_>)->Result<(),ValueError>{if WavSnapshot::SQLITE_SCHEMA.len()>control.limits().max_schema_bytes{Err(ownership("WAV authored SQLite schema exceeds schema byte limit"))}else{Ok(())}}
fn forecast(snapshot:&WavSnapshot,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<usize,ValueError>{
    admit_schema(control)?;control.checkpoint(phase,0,snapshot.other_chunks.len())?;
    let samples=match &snapshot.data{WavData::Pcm16(values)=>values.len(),WavData::Pcm8(values)|WavData::Raw(values)=>values.len(),WavData::Float32(values)=>values.len()};
    let mut count=add_rows(add_rows(add_rows(3,samples)?,snapshot.other_chunks.len())?,snapshot.chunk_order.len())?;
    if let Some(extension)=&snapshot.fmt.ext{count=add_rows(add_rows(count,1)?,extension.len())?;}control.check_rows(count)?;
    for(position,chunk)in snapshot.other_chunks.iter().enumerate(){count=add_rows(count,chunk.data.len())?;control.check_rows(count)?;if(position+1)%256==0{control.checkpoint(phase,position+1,snapshot.other_chunks.len())?;}}
    control.checkpoint(phase,snapshot.other_chunks.len(),snapshot.other_chunks.len())?;Ok(count)
}
fn native_value_bytes(snapshot:&WavSnapshot)->Result<usize,ValueError>{
 fn add(total:usize,count:usize)->Result<usize,ValueError>{total.checked_add(count).ok_or_else(||ownership("WAV native value byte count overflow"))}
 let mut total=add(snapshot.schema.len(),32)?;if let Some(extension)=&snapshot.fmt.ext{total=add(total,extension.len())?;}total=add(total,match &snapshot.data{WavData::Pcm16(values)=>values.len().checked_mul(2),WavData::Pcm8(values)|WavData::Raw(values)=>Some(values.len()),WavData::Float32(values)=>values.len().checked_mul(4)}.ok_or_else(||ownership("WAV native sample byte count overflow"))?)?;
 for chunk in &snapshot.other_chunks{total=add(add(total,chunk.fourcc.len())?,chunk.data.len())?;}add(total,snapshot.chunk_order.len().checked_mul(8).ok_or_else(||ownership("WAV native relationship byte count overflow"))?)
}
fn native_list(value:Option<&semio_framework_dsl_record::FieldValue>)->Result<&[semio_framework_dsl_record::FieldValue],ValueError>{match value{Some(semio_framework_dsl_record::FieldValue::List(values))=>Ok(values),None|Some(semio_framework_dsl_record::FieldValue::Absent)=>Ok(&[]),_=>Err(invalid("WAV native collection requires a literal list"))}}
fn native_record(value:Option<&semio_framework_dsl_record::FieldValue>)->Result<&semio_framework_dsl_record::RecordValue,ValueError>{match value{Some(semio_framework_dsl_record::FieldValue::Record(record))=>Ok(record),_=>Err(invalid("WAV native entity requires its literal record"))}}
fn admit_native_rows(record:&semio_framework_dsl_record::RecordValue,maximum:usize,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{
    native.scoped_stage(|native|->Result<_,ValueError>{
        let format=native_record(record.get(1))?;let data=native_record(record.get(2))?;
        let Some(semio_framework_dsl_record::FieldValue::Enum(kind))=data.get(1)else{return Err(invalid("WAV samples require a declared kind"));};
        let id=kind.checked_add(2).filter(|id|*id<=5).ok_or_else(||invalid("WAV sample kind is undeclared"))?as u16;
        let chunks=native_list(record.get(5))?;let mut count=add_rows(add_rows(add_rows(3,native_list(data.get(id))?.len())?,chunks.len())?,native_list(record.get(6))?.len())?;
        if let Some(value)=format.get(6).filter(|value|!matches!(value,semio_framework_dsl_record::FieldValue::Absent)){count=add_rows(add_rows(count,1)?,native_list(Some(value))?.len())?;}
        if count>maximum{return Err(work("WAV native snapshot exceeds semantic row limit"));}
        native.begin_stage(chunks.len())?;
        for chunk in chunks{let chunk=native_record(Some(chunk))?;let size=match chunk.get(1){Some(semio_framework_dsl_record::FieldValue::Bytes64(bytes))=>bytes.len(),None|Some(semio_framework_dsl_record::FieldValue::Absent)=>0,_=>return Err(invalid("WAV chunk requires literal octets"))};count=add_rows(count,size)?;if count>maximum{return Err(work("WAV native snapshot exceeds semantic row limit"));}native.step()?;}
        Ok(())
    })
}

impl ArtifactSqliteSnapshot for WavSnapshot {
    fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{admit_schema(control)?;control.check_rows(3)?;let maximum=control.limits().max_rows;let snapshot=store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|{admit_native_rows(record,maximum,native)?;Self::__dsl_from_record_controlled(record,native)},control)?;control.check_value_bytes(native_value_bytes(&snapshot)?)?;Ok(snapshot)}
    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io::IoPayload,ValueError>{admit_schema(control)?;control.check_value_bytes(native_value_bytes(self)?)?;forecast(self,control,SqliteSnapshotPhase::EncodeNative)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)}
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        forecast(self,control,SqliteSnapshotPhase::ProjectSnapshot)?;
        let mut out = Projection::new(Self::SQLITE_SCHEMA, control)?;
        out.insert("wav_document", &[Cell::Text(&self.schema)])?;
        out.insert(
            "wav_format",
            &[
                Cell::Integer(self.fmt.audio_format.into()),
                Cell::Integer(self.fmt.channels.into()),
                Cell::Integer(self.fmt.sample_rate.into()),
                Cell::Integer(self.fmt.byte_rate.into()),
                Cell::Integer(self.fmt.block_align.into()),
                Cell::Integer(self.fmt.bits_per_sample.into()),
                Cell::Integer(self.fmt_pad_byte.into()),
            ],
        )?;
        if let Some(ext) = &self.fmt.ext {
            out.insert_key("wav_format_extension", 1, &[])?;
            write_octets(&mut out, "wav_format_extension_byte", 1, ext)?;
        }
        let kind = match &self.data {
            WavData::Pcm16(_) => "pcm16",
            WavData::Pcm8(_) => "pcm8",
            WavData::Float32(_) => "float32",
            WavData::Raw(_) => "raw",
        };
        out.insert("wav_data", &[Cell::Text(kind), Cell::Integer(self.data_pad_byte.into())])?;
        match &self.data {
            WavData::Pcm16(samples) => {
                for (ordinal, sample) in samples.iter().enumerate() {
                    out.insert("wav_pcm16_sample", &[Cell::Integer(1), Cell::Integer(ordinal as i64), Cell::Integer((*sample).into())])?;
                }
            }
            WavData::Pcm8(samples) => write_octets(&mut out, "wav_pcm8_sample", 1, samples)?,
            WavData::Raw(bytes) => write_octets(&mut out, "wav_raw_data_byte", 1, bytes)?,
            WavData::Float32(samples) => {
                for (ordinal, sample) in samples.iter().enumerate() {
                    out.insert(
                        "wav_float32_sample",
                        &[Cell::Integer(1), Cell::Integer(ordinal as i64), Cell::Integer(sample.to_bits().into()), Cell::Text(numeric_class(*sample)), if sample.is_finite() { Cell::Real(f64::from(*sample)) } else { Cell::Null }],
                    )?;
                }
            }
        }
        for (ordinal, chunk) in self.other_chunks.iter().enumerate() {
            let id = out.insert("wav_other_chunk", &[Cell::Integer(1), Cell::Integer(ordinal as i64), Cell::Text(&chunk.fourcc), Cell::Integer(chunk.pad_byte.into())])?;
            let expected=i64::try_from(ordinal).ok().and_then(|value|value.checked_add(1)).ok_or_else(||work("WAV auxiliary chunk identity exceeds signed64"))?;
            if id!=expected{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"WAV auxiliary identity differs from its canonical ordinal"));}
            write_octets(&mut out, "wav_other_chunk_byte", id, &chunk.data)?;
        }
        for (ordinal, reference) in self.chunk_order.iter().enumerate() {
            let index = match reference {
                WavChunkRef::Other(index) => Some(UnsignedWord::new(*index)),
                _ => None,
            };
            let (kind, format, data, other) = match reference {
                WavChunkRef::Format => ("format", Cell::Integer(1), Cell::Null, Cell::Null),
                WavChunkRef::Samples => ("samples", Cell::Null, Cell::Integer(1), Cell::Null),
                WavChunkRef::Other(index) => ("other", Cell::Null, Cell::Null, usize::try_from(*index).ok().filter(|index|*index<self.other_chunks.len()).and_then(|index|i64::try_from(index).ok()?.checked_add(1)).map_or(Cell::Null,Cell::Integer)),
            };
            out.insert("wav_chunk_order", &[Cell::Integer(1), Cell::Integer(ordinal as i64), Cell::Text(kind), format, data, other, index.as_ref().map_or(Cell::Null, |word|Cell::Text(word.text()))])?;
        }
        out.finish()
    }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        validate_sqlite_database_schema_controlled(database, Self::SQLITE_SCHEMA,SqliteSnapshotPhase::ReconstructSnapshot,control)?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let document = database.table("wav_document")?.single_row()?;
        let fmt = database.table("wav_format")?.single_row()?;
        let data = database.table("wav_data")?.single_row()?;
        for row in [document, fmt, data] {
            if row.rowid != 1 || row.integer(0)? != 1 {
                return Err(invalid("WAV primary chunk identity must be one"));
            }
        }
        let extension_rows = entities(database, "wav_format_extension", control)?;
        if extension_rows.len() > 1 || extension_rows.first().is_some_and(|row| row.rowid != 1) {
            return Err(invalid("WAV format has an unknown extension owner"));
        }
        let ext_rows = ordered(database, "wav_format_extension_byte", control)?;
        if extension_rows.is_empty() && !ext_rows.is_empty() {
            return Err(invalid("WAV extension octets lack their optional extension"));
        }
        let ext = if extension_rows.is_empty() { None } else { Some(read_octets(&ext_rows, control)?) };
        let kind = data.text(1)?;
        for (table, selected) in [("wav_pcm16_sample", "pcm16"), ("wav_pcm8_sample", "pcm8"), ("wav_float32_sample", "float32"), ("wav_raw_data_byte", "raw")] {
            if selected != kind && !database.table(table)?.rows.is_empty() {
                return Err(invalid("WAV data contains rows of another sample kind"));
            }
        }
        let samples = match kind {
            "pcm16" => {
                let rows = ordered(database, "wav_pcm16_sample", control)?;
                let mut samples = reserve(rows.len(),control)?;
                for row in rows {
                    checkpoint(control, samples.len(), 0)?;
                    samples.push(i16::try_from(row.integer(3)?).map_err(|error|invalid(error.to_string()))?);
                }
                WavData::Pcm16(samples)
            }
            "pcm8" => WavData::Pcm8(read_octets(&ordered(database, "wav_pcm8_sample", control)?, control)?),
            "raw" => WavData::Raw(read_octets(&ordered(database, "wav_raw_data_byte", control)?, control)?),
            "float32" => {
                let rows = ordered(database, "wav_float32_sample", control)?;
                let mut samples = reserve(rows.len(),control)?;
                for row in rows {
                    checkpoint(control, samples.len(), 0)?;
                    samples.push(read_float(row)?);
                }
                WavData::Float32(samples)
            }
            _ => return Err(invalid("WAV data has an unknown sample kind")),
        };
        let chunks = ordered(database, "wav_other_chunk", control)?;
        let mut chunk_ids=reserve(chunks.len(),control)?;
        for(position,row)in chunks.iter().enumerate(){checkpoint(control,position,chunks.len())?;chunk_ids.push(row.rowid);}
        heap_sort(&mut chunk_ids,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,_|Ok(a.cmp(b)))?;
        let bytes = groups(entities(database, "wav_other_chunk_byte", control)?, &chunk_ids, control)?;
        let mut other_chunks = reserve(chunks.len(),control)?;
        for row in &chunks {
            checkpoint(control, other_chunks.len(), 0)?;
            other_chunks.push(RiffChunk { fourcc: reconstruct_text(control, row.text(3)?)?, data: read_octets(owner_rows(&bytes,row.rowid), control)?, pad_byte: octet(row, 4)? });
        }
        let order=ordered(database,"wav_chunk_order",control)?;
        let mut chunk_order = reserve(order.len(),control)?;
        for row in order {
            checkpoint(control, chunk_order.len(), 0)?;
            let null = |column| matches!(row.values.get(column), Some(SqliteValue::Null));
            let reference = match row.text(3)? {
                "format" if row.integer(4)? == 1 && null(5) && null(6) && null(7) => WavChunkRef::Format,
                "samples" if null(4) && row.integer(5)? == 1 && null(6) && null(7) => WavChunkRef::Samples,
                "other" if null(4) && null(5) => {
                    let text = row.text(7)?;
                    let index = text.parse::<u64>().map_err(|error|invalid(error.to_string()))?;
                    if UnsignedWord::new(index).text() != text {
                        return Err(invalid("WAV auxiliary chunk index requires a canonical unsigned64 decimal"));
                    }
                    let expected = usize::try_from(index).ok().and_then(|index| chunks.get(index)).map(|row|row.rowid);
                    let actual = if null(6) { None } else { Some(row.integer(6)?) };
                    if actual != expected {
                        return Err(invalid("WAV auxiliary index disagrees with its resolved chunk relationship"));
                    }
                    WavChunkRef::Other(index)
                }
                _ => return Err(invalid("WAV chunk reference does not select exactly one declared chunk")),
            };
            chunk_order.push(reference);
        }
        let result = Self {
            schema: reconstruct_text(control, document.text(1)?)?,
            fmt: WavFmt { audio_format: short(fmt, 1)?, channels: short(fmt, 2)?, sample_rate: unsigned(fmt, 3)?, byte_rate: unsigned(fmt, 4)?, block_align: short(fmt, 5)?, bits_per_sample: short(fmt, 6)?, ext },
            data: samples,
            fmt_pad_byte: octet(fmt, 7)?,
            data_pad_byte: octet(data, 2)?,
            other_chunks,
            chunk_order,
        };
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 1, 1)?;
        Ok(result)
    }
}
