//! 🎧️ Handcrafted WAV primary chunks, samples, extension octets and chunk relationships.
use super::*;
use semio_framework_os_kernel::{
    sqlite_snapshot::{
        artifact::{reconstruct_text, Cell, Projection},
        validate_sqlite_database_schema, SnapshotEncoding, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue,
    },
    ArtifactSqliteSnapshot,
};
use std::collections::{BTreeMap, BTreeSet};

fn unsigned(row: &SqliteRow, column: usize) -> Result<u32, String> {
    u32::try_from(row.integer(column)?).map_err(|error| error.to_string())
}
fn short(row: &SqliteRow, column: usize) -> Result<u16, String> {
    u16::try_from(row.integer(column)?).map_err(|error| error.to_string())
}
fn octet(row: &SqliteRow, column: usize) -> Result<u8, String> {
    u8::try_from(row.integer(column)?).map_err(|error| error.to_string())
}
fn checkpoint(control: &mut SqliteSnapshotControl<'_>, position: usize, total: usize) -> Result<(), String> {
    if position % 256 == 0 {
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, position, total)?;
    }
    Ok(())
}
fn entities<'a>(database: &'a SqliteDatabase, table: &str, control: &mut SqliteSnapshotControl<'_>) -> Result<&'a [SqliteRow], String> {
    let rows = &database.table(table)?.rows;
    let mut keys = BTreeSet::new();
    for (position, row) in rows.iter().enumerate() {
        checkpoint(control, position, rows.len())?;
        if row.rowid <= 0 || row.integer(0)? != row.rowid || !keys.insert(row.rowid) {
            return Err("WAV entities require unique positive aliased identifiers".into());
        }
    }
    Ok(rows)
}
fn ordered<'a>(database: &'a SqliteDatabase, table: &str, control: &mut SqliteSnapshotControl<'_>) -> Result<Vec<&'a SqliteRow>, String> {
    let rows = entities(database, table, control)?;
    let mut ordered = vec![None; rows.len()];
    for (position, row) in rows.iter().enumerate() {
        checkpoint(control, position, rows.len())?;
        if row.integer(1)? != 1 {
            return Err("WAV primary child has an unknown owner".into());
        }
        let ordinal = usize::try_from(row.integer(2)?).map_err(|error| error.to_string())?;
        let slot = ordered.get_mut(ordinal).ok_or("WAV ordinals must be dense")?;
        if slot.replace(row).is_some() {
            return Err("WAV ordinals must be unique".into());
        }
    }
    ordered.into_iter().map(|row| row.ok_or_else(|| "WAV ordinals must be dense".into())).collect()
}
fn groups<'a>(rows: &'a [SqliteRow], parents: &BTreeSet<i64>, control: &mut SqliteSnapshotControl<'_>) -> Result<BTreeMap<i64, Vec<&'a SqliteRow>>, String> {
    let mut groups = BTreeMap::<i64, Vec<&SqliteRow>>::new();
    for (position, row) in rows.iter().enumerate() {
        checkpoint(control, position, rows.len())?;
        let owner = row.integer(1)?;
        if !parents.contains(&owner) {
            return Err("WAV octet has an unknown chunk owner".into());
        }
        groups.entry(owner).or_default().push(row);
    }
    for rows in groups.values_mut() {
        let mut ordered = vec![None; rows.len()];
        for (position, row) in rows.iter().enumerate() {
            checkpoint(control, position, rows.len())?;
            let ordinal = usize::try_from(row.integer(2)?).map_err(|error| error.to_string())?;
            let slot = ordered.get_mut(ordinal).ok_or("WAV octet ordinals must be dense")?;
            if slot.replace(*row).is_some() {
                return Err("WAV octet ordinals must be unique".into());
            }
        }
        *rows = ordered.into_iter().map(|row| row.ok_or_else(|| "WAV octet ordinals must be dense".into())).collect::<Result<_, String>>()?;
    }
    Ok(groups)
}
fn write_octets(out: &mut Projection<'_, '_>, table: &str, owner: i64, bytes: &[u8]) -> Result<(), String> {
    for (ordinal, byte) in bytes.iter().enumerate() {
        out.insert(table, &[Cell::Integer(owner), Cell::Integer(ordinal as i64), Cell::Integer((*byte).into())])?;
    }
    Ok(())
}
fn read_octets<'a>(rows: impl IntoIterator<Item = &'a SqliteRow>, control: &mut SqliteSnapshotControl<'_>) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
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
fn read_float(row: &SqliteRow) -> Result<f32, String> {
    let sample = f32::from_bits(unsigned(row, 3)?);
    if row.text(4)? != numeric_class(sample) {
        return Err("WAV sample numeric class disagrees with its IEEE754 bits".into());
    }
    if sample.is_finite() {
        let value=f64::from(sample);
        let exact=match row.values.get(5){Some(SqliteValue::Real(query))=>*query==value,Some(SqliteValue::Integer(query))=>value>=i64::MIN as f64&&value < -(i64::MIN as f64)&&value.fract()==0.0&&value as i64==*query,_=>false};
        if !exact {
            return Err("WAV query sample disagrees with its IEEE754 bits".into());
        }
    } else if !matches!(row.values.get(5), Some(SqliteValue::Null)) {
        return Err("WAV non-finite sample requires an explicit class and NULL query value".into());
    }
    Ok(sample)
}

fn add_rows(total:usize,count:usize)->Result<usize,String>{total.checked_add(count).ok_or_else(||"WAV semantic row count overflow".into())}
fn admit_schema(control:&SqliteSnapshotControl<'_>)->Result<(),String>{if WavSnapshot::SQLITE_SCHEMA.len()>control.limits().max_schema_bytes{Err("WAV authored SQLite schema exceeds schema byte limit".into())}else{Ok(())}}
fn forecast(snapshot:&WavSnapshot,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<usize,String>{
    admit_schema(control)?;control.checkpoint(phase,0,snapshot.other_chunks.len())?;
    let samples=match &snapshot.data{WavData::Pcm16(values)=>values.len(),WavData::Pcm8(values)|WavData::Raw(values)=>values.len(),WavData::Float32(values)=>values.len()};
    let mut count=add_rows(add_rows(add_rows(3,samples)?,snapshot.other_chunks.len())?,snapshot.chunk_order.len())?;
    if let Some(extension)=&snapshot.fmt.ext{count=add_rows(add_rows(count,1)?,extension.len())?;}control.check_rows(count)?;
    for(position,chunk)in snapshot.other_chunks.iter().enumerate(){count=add_rows(count,chunk.data.len())?;control.check_rows(count)?;if(position+1)%256==0{control.checkpoint(phase,position+1,snapshot.other_chunks.len())?;}}
    control.checkpoint(phase,snapshot.other_chunks.len(),snapshot.other_chunks.len())?;Ok(count)
}
fn native_list(value:Option<&dsl::FieldValue>)->Result<&[dsl::FieldValue],String>{match value{Some(dsl::FieldValue::List(values))=>Ok(values),None|Some(dsl::FieldValue::Absent)=>Ok(&[]),_=>Err("WAV native collection requires a literal list".into())}}
fn native_record(value:Option<&dsl::FieldValue>)->Result<&dsl::RecordValue,String>{match value{Some(dsl::FieldValue::Record(record))=>Ok(record),_=>Err("WAV native entity requires its literal record".into())}}
fn admit_native_rows(record:&dsl::RecordValue,maximum:usize,native:&mut dsl::NativeDecodeControl<'_>)->Result<(),dsl::TextError>{
    native.scoped_stage(|native|->Result<_,String>{
        let format=native_record(record.get(1))?;let data=native_record(record.get(2))?;
        let Some(dsl::FieldValue::Enum(kind))=data.get(1)else{return Err("WAV samples require a declared kind".into());};
        let id=kind.checked_add(2).filter(|id|*id<=5).ok_or("WAV sample kind is undeclared")?as u16;
        let chunks=native_list(record.get(5))?;let mut count=add_rows(add_rows(add_rows(3,native_list(data.get(id))?.len())?,chunks.len())?,native_list(record.get(6))?.len())?;
        if let Some(value)=format.get(6).filter(|value|!matches!(value,dsl::FieldValue::Absent)){count=add_rows(add_rows(count,1)?,native_list(Some(value))?.len())?;}
        if count>maximum{return Err("WAV native snapshot exceeds semantic row limit".into());}
        native.begin_stage(chunks.len())?;
        for chunk in chunks{let chunk=native_record(Some(chunk))?;let size=match chunk.get(1){Some(dsl::FieldValue::Bytes64(bytes))=>bytes.len(),None|Some(dsl::FieldValue::Absent)=>0,_=>return Err("WAV chunk requires literal octets".into())};count=add_rows(count,size)?;if count>maximum{return Err("WAV native snapshot exceeds semantic row limit".into());}native.step()?;}
        Ok(())
    }).map_err(dsl::__rt::field_error)
}

impl ArtifactSqliteSnapshot for WavSnapshot {
    fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{admit_schema(control)?;control.check_rows(3)?;let maximum=control.limits().max_rows;store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|{admit_native_rows(record,maximum,native)?;Self::__dsl_from_record_controlled(record,native)},control)}
    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io::IoPayload,String>{forecast(self,control,SqliteSnapshotPhase::EncodeNative)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)}
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, String> {
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
        let mut chunks = Vec::new();
        for (ordinal, chunk) in self.other_chunks.iter().enumerate() {
            let id = out.insert("wav_other_chunk", &[Cell::Integer(1), Cell::Integer(ordinal as i64), Cell::Text(&chunk.fourcc), Cell::Integer(chunk.pad_byte.into())])?;
            chunks.push(id);
            write_octets(&mut out, "wav_other_chunk_byte", id, &chunk.data)?;
        }
        for (ordinal, reference) in self.chunk_order.iter().enumerate() {
            let index = match reference {
                WavChunkRef::Other(index) => Some(index.to_string()),
                _ => None,
            };
            let (kind, format, data, other) = match reference {
                WavChunkRef::Format => ("format", Cell::Integer(1), Cell::Null, Cell::Null),
                WavChunkRef::Samples => ("samples", Cell::Null, Cell::Integer(1), Cell::Null),
                WavChunkRef::Other(index) => ("other", Cell::Null, Cell::Null, usize::try_from(*index).ok().and_then(|index| chunks.get(index)).map_or(Cell::Null, |id| Cell::Integer(*id))),
            };
            out.insert("wav_chunk_order", &[Cell::Integer(1), Cell::Integer(ordinal as i64), Cell::Text(kind), format, data, other, index.as_deref().map_or(Cell::Null, Cell::Text)])?;
        }
        out.finish()
    }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> {
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits()).map_err(|error| error.to_string())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let document = database.table("wav_document")?.single_row()?;
        let fmt = database.table("wav_format")?.single_row()?;
        let data = database.table("wav_data")?.single_row()?;
        for row in [document, fmt, data] {
            if row.rowid != 1 || row.integer(0)? != 1 {
                return Err("WAV primary chunk identity must be one".into());
            }
        }
        let extension_rows = entities(database, "wav_format_extension", control)?;
        if extension_rows.len() > 1 || extension_rows.first().is_some_and(|row| row.rowid != 1) {
            return Err("WAV format has an unknown extension owner".into());
        }
        let ext_rows = ordered(database, "wav_format_extension_byte", control)?;
        if extension_rows.is_empty() && !ext_rows.is_empty() {
            return Err("WAV extension octets lack their optional extension".into());
        }
        let ext = if extension_rows.is_empty() { None } else { Some(read_octets(ext_rows, control)?) };
        let kind = data.text(1)?;
        for (table, selected) in [("wav_pcm16_sample", "pcm16"), ("wav_pcm8_sample", "pcm8"), ("wav_float32_sample", "float32"), ("wav_raw_data_byte", "raw")] {
            if selected != kind && !database.table(table)?.rows.is_empty() {
                return Err("WAV data contains rows of another sample kind".into());
            }
        }
        let samples = match kind {
            "pcm16" => {
                let rows = ordered(database, "wav_pcm16_sample", control)?;
                let mut samples = Vec::new();
                for row in rows {
                    checkpoint(control, samples.len(), 0)?;
                    samples.push(i16::try_from(row.integer(3)?).map_err(|error| error.to_string())?);
                }
                WavData::Pcm16(samples)
            }
            "pcm8" => WavData::Pcm8(read_octets(ordered(database, "wav_pcm8_sample", control)?, control)?),
            "raw" => WavData::Raw(read_octets(ordered(database, "wav_raw_data_byte", control)?, control)?),
            "float32" => {
                let rows = ordered(database, "wav_float32_sample", control)?;
                let mut samples = Vec::new();
                for row in rows {
                    checkpoint(control, samples.len(), 0)?;
                    samples.push(read_float(row)?);
                }
                WavData::Float32(samples)
            }
            _ => return Err("WAV data has an unknown sample kind".into()),
        };
        let chunks = ordered(database, "wav_other_chunk", control)?;
        let chunk_ids = chunks.iter().map(|row| row.rowid).collect();
        let bytes = groups(entities(database, "wav_other_chunk_byte", control)?, &chunk_ids, control)?;
        let mut other_chunks = Vec::new();
        let mut identities = Vec::new();
        for row in chunks {
            checkpoint(control, other_chunks.len(), 0)?;
            identities.push(row.rowid);
            other_chunks.push(RiffChunk { fourcc: reconstruct_text(control, row.text(3)?)?, data: read_octets(bytes.get(&row.rowid).into_iter().flatten().copied(), control)?, pad_byte: octet(row, 4)? });
        }
        let mut chunk_order = Vec::new();
        for row in ordered(database, "wav_chunk_order", control)? {
            checkpoint(control, chunk_order.len(), 0)?;
            let null = |column| matches!(row.values.get(column), Some(SqliteValue::Null));
            let reference = match row.text(3)? {
                "format" if row.integer(4)? == 1 && null(5) && null(6) && null(7) => WavChunkRef::Format,
                "samples" if null(4) && row.integer(5)? == 1 && null(6) && null(7) => WavChunkRef::Samples,
                "other" if null(4) && null(5) => {
                    let text = row.text(7)?;
                    let index = text.parse::<u64>().map_err(|error| error.to_string())?;
                    if index.to_string() != text {
                        return Err("WAV auxiliary chunk index requires a canonical unsigned64 decimal".into());
                    }
                    let expected = usize::try_from(index).ok().and_then(|index| identities.get(index)).copied();
                    let actual = if null(6) { None } else { Some(row.integer(6)?) };
                    if actual != expected {
                        return Err("WAV auxiliary index disagrees with its resolved chunk relationship".into());
                    }
                    WavChunkRef::Other(index)
                }
                _ => return Err("WAV chunk reference does not select exactly one declared chunk".into()),
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
