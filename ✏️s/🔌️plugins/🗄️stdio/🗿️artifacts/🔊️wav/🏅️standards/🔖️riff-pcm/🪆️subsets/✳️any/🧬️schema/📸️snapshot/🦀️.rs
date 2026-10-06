//! 🧬️ WavSnapshot — typed primary `fmt `/`data` chunks plus the complete ordered RIFF
//! chunk sequence. Duplicate canonical chunks remain verbatim auxiliary chunks, so every admitted
//! recording can preserve chunk order and multiplicity through an edit.

/// 📦️ Owned by `wav`: the `fmt ` chunk's fields, typed. `ext` carries the extensible/non-PCM
/// tail (`cbSize` bytes) verbatim when present — `None` for the plain 16-byte PCM form. NO type
/// sharing with `avi` (both are RIFF-based but deliberately distinct vocabularies per the master
/// plan).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct WavFmt {
    pub audio_format: u16,
    pub channels: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<Vec<u8>>,
}

impl Default for WavFmt {
    fn default() -> Self {
        Self { audio_format: 1, channels: 1, sample_rate: 44_100, byte_rate: 88_200, block_align: 2, bits_per_sample: 16, ext: None }
    }
}

/// 📦️ Owned by `wav`: the `data` chunk's samples, typed per `WavFmt`'s
/// `(audio_format, bits_per_sample)` — `Raw` is the honest fallback for anything this codec
/// doesn't interpret sample-by-sample (24-bit PCM, ADPCM, WAVE_FORMAT_EXTENSIBLE payloads, …).
/// 🔢️ JSON float samples retain their complete unsigned IEEE 754 words.
#[derive(Clone, Debug)]
pub enum WavData {
    Pcm16(Vec<i16>),
    Pcm8(Vec<u8>),
    Float32(Vec<f32>),
    Raw(Vec<u8>),
}

impl PartialEq for WavData{
    fn eq(&self,other:&Self)->bool{match(self,other){(Self::Pcm16(a),Self::Pcm16(b))=>a==b,(Self::Pcm8(a),Self::Pcm8(b))|(Self::Raw(a),Self::Raw(b))=>a==b,(Self::Float32(a),Self::Float32(b))=>a.len()==b.len()&&a.iter().zip(b).all(|(a,b)|a.to_bits()==b.to_bits()),_=>false}}
}

impl Default for WavData {
    fn default() -> Self {
        WavData::Raw(Vec::new())
    }
}

fn wav_tagged_fields(value: semio_framework_value::DslValue) -> Result<(String, Option<semio_framework_value::DslValue>), semio_framework_value::ValueError> {
    let semio_framework_value::DslValue::Object(mut fields) = value else { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "WAV variant requires an object")); };
    let index = fields.iter().position(|(key, _)| key == "kind").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "WAV variant requires kind"))?;
    let semio_framework_value::DslValue::String(kind) = fields.remove(index).1 else { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "WAV variant kind requires a string")); };
    let payload = match fields.len() {
        0 => None,
        1 if fields[0].0 == "value" => Some(fields.remove(0).1),
        _ => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "WAV variant has duplicate or unknown fields")),
    };
    Ok((kind, payload))
}

impl semio_framework_value::ToValue for WavData {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let (kind, value) = match self {
            Self::Pcm16(samples) => ("pcm16", semio_framework_value::ToValue::to_value(samples)),
            Self::Pcm8(samples) => ("pcm8", semio_framework_value::ToValue::to_value(samples)),
            Self::Raw(samples) => ("raw", semio_framework_value::ToValue::to_value(samples)),
            Self::Float32(samples) => ("float32", semio_framework_value::DslValue::Array(samples.iter().map(|sample| semio_framework_value::DslValue::Object(vec![("bits".into(), semio_framework_value::ToValue::to_value(&sample.to_bits()))])).collect())),
        };
        semio_framework_value::DslValue::Object(vec![("kind".into(), semio_framework_value::DslValue::String(kind.into())), ("value".into(), value)])
    }
}

impl semio_framework_value::FromValue for WavData {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let (kind, value) = wav_tagged_fields(value)?;
        let value = value.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "WAV samples require value"))?;
        match kind.as_str() {
            "pcm16" => Ok(Self::Pcm16(semio_framework_value::FromValue::from_value(value)?)),
            "pcm8" => Ok(Self::Pcm8(semio_framework_value::FromValue::from_value(value)?)),
            "raw" => Ok(Self::Raw(semio_framework_value::FromValue::from_value(value)?)),
            "float32" => {
                let semio_framework_value::DslValue::Array(samples) = value else { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "WAV float samples require an array")); };
                Ok(Self::Float32(samples.into_iter().map(|sample| {
                    let semio_framework_value::DslValue::Object(mut fields) = sample else { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "WAV float sample requires its unsigned word")); };
                    if fields.len() != 1 || fields[0].0 != "bits" { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "WAV float sample requires exactly bits")); }
                    let bits: u32 = semio_framework_value::FromValue::from_value(fields.remove(0).1)?;
                    Ok(f32::from_bits(bits))
                }).collect::<Result<Vec<_>, semio_framework_value::ValueError>>()?))
            }
            _ => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "WAV samples have an unknown kind")),
        }
    }
    fn edit_value_at_path(&mut self, path: &[&str], edit: semio_framework_value::ValueEdit) -> Result<(), semio_framework_value::ValueError> {
        semio_framework_value::edit_through_value(self, path, edit)
    }
}

fn wav_data_spec() -> semio_framework_dsl_record::RecordSpec {
    semio_framework_dsl_record::RecordSpec::new(
        None,
        semio_framework_dsl_record::RecordLayout::Inline,
        vec![
            semio_framework_dsl_record::FieldSpec::new(1, "kind", semio_framework_dsl_record::Shape::Enum(vec![("pcm16".into(), 0), ("pcm8".into(), 1), ("float32".into(), 2), ("raw".into(), 3)])),
            semio_framework_dsl_record::FieldSpec::new(2, "pcm16", <Vec<i16> as semio_framework_dsl_record::DslField>::shape()).optional(),
            semio_framework_dsl_record::FieldSpec::new(3, "pcm8", <Vec<u8> as semio_framework_dsl_record::DslField>::shape()).optional(),
            semio_framework_dsl_record::FieldSpec::new(4, "float32", <Vec<f32> as semio_framework_dsl_record::DslField>::shape()).optional(),
            semio_framework_dsl_record::FieldSpec::new(5, "raw", <Vec<u8> as semio_framework_dsl_record::DslField>::shape()).optional(),
        ],
    )
}

fn wav_invalid(message:impl Into<String>)->semio_framework_value::ValueError{semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message)}

fn wav_enum_shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(labels:&[(&str,u32)],control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{
    control.scoped_stage(|control|{control.begin_stage(labels.len())?;let mut values=control.allocate_vec::<(String,u32)>(labels.len())?;for(label,ordinal)in labels{values.push((control.copy_text(label)?,*ordinal));control.step()?;}Ok(semio_framework_dsl_record::Shape::Enum(values))})
}

fn wav_data_spec_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::RecordSpec,semio_framework_value::ValueError>{
    control.scoped_stage(|control|{
        control.begin_stage(5)?;let mut fields=control.allocate_vec::<semio_framework_dsl_record::FieldSpec>(5)?;
        let kind=wav_enum_shape_controlled(&[("pcm16",0),("pcm8",1),("float32",2),("raw",3)],control)?;fields.push(semio_framework_dsl_record::producer::field(1,"kind",kind,control)?);control.step()?;
        let pcm16=<Vec<i16>as semio_framework_dsl_record::DslField>::shape_controlled(control)?;fields.push(semio_framework_dsl_record::producer::field(2,"pcm16",pcm16,control)?.optional());control.step()?;
        let pcm8=<Vec<u8>as semio_framework_dsl_record::DslField>::shape_controlled(control)?;fields.push(semio_framework_dsl_record::producer::field(3,"pcm8",pcm8,control)?.optional());control.step()?;
        let float32=<Vec<f32>as semio_framework_dsl_record::DslField>::shape_controlled(control)?;fields.push(semio_framework_dsl_record::producer::field(4,"float32",float32,control)?.optional());control.step()?;
        let raw=<Vec<u8>as semio_framework_dsl_record::DslField>::shape_controlled(control)?;fields.push(semio_framework_dsl_record::producer::field(5,"raw",raw,control)?.optional());control.step()?;
        semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Inline,fields,control)
    })
}

fn wav_data_spec_producer()->semio_framework_dsl_record::RecordSpecProducer{semio_framework_dsl_record::RecordSpecProducer{ordinary:wav_data_spec,decoding:|control|wav_data_spec_controlled(control),encoding:|control|wav_data_spec_controlled(control)}}

impl semio_framework_dsl_record::DslField for WavData {
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,semio_framework_value::ValueError>{self.to_record_controlled(control).map(semio_framework_dsl_record::FieldValue::Record)}
    fn to_record_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,semio_framework_value::ValueError>{
        control.scoped_depth(64,|control|control.scoped_stage(|control|{
            control.begin_stage(2)?;let mut record=semio_framework_dsl_record::native_encoding::EncodedRecord::new(2,control)?;
            let(kind,id)=match self{Self::Pcm16(_)=>(0,2),Self::Pcm8(_)=>(1,3),Self::Float32(_)=>(2,4),Self::Raw(_)=>(3,5)};
            record.insert(1,semio_framework_dsl_record::FieldValue::Enum(kind))?;control.step()?;
            let values=match self{Self::Pcm16(values)=><Vec<i16>as semio_framework_dsl_record::DslField>::to_value_controlled(values,control)?,Self::Pcm8(values)|Self::Raw(values)=><Vec<u8>as semio_framework_dsl_record::DslField>::to_value_controlled(values,control)?,Self::Float32(values)=><Vec<f32>as semio_framework_dsl_record::DslField>::to_value_controlled(values,control)?};
            record.insert(id,values)?;control.step()?;Ok(record.take())
        }))
    }
    fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{
        let semio_framework_dsl_record::FieldValue::Record(record)=value else{return Err(wav_invalid("WAV samples require a typed record"));};
        Self::from_record_controlled(record,control)
    }
    fn from_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{
        control.step()?;
        if record.fields.len()>5{return Err(wav_invalid("WAV sample record contains excess fields"));}
        let Some(semio_framework_dsl_record::FieldValue::Enum(kind))=record.get(1)else{return Err(wav_invalid("WAV samples require a declared kind"));};
        let id=kind.checked_add(2).filter(|id|*id<=5).ok_or_else(||wav_invalid("WAV samples have an unknown kind"))?as u16;
        for(other,value)in &record.fields{control.step()?;if *other!=1&&*other!=id&&!matches!(value,semio_framework_dsl_record::FieldValue::Absent){return Err(wav_invalid("WAV sample kind selects exactly one typed array"));}}
        let values=record.get(id).ok_or_else(||wav_invalid("WAV selected sample array is absent"))?;
        match kind{
            0=><Vec<i16>as semio_framework_dsl_record::DslField>::from_value_controlled(values,control).map(Self::Pcm16),
            1=><Vec<u8>as semio_framework_dsl_record::DslField>::from_value_controlled(values,control).map(Self::Pcm8),
            2=><Vec<f32>as semio_framework_dsl_record::DslField>::from_value_controlled(values,control).map(Self::Float32),
            3=><Vec<u8>as semio_framework_dsl_record::DslField>::from_value_controlled(values,control).map(Self::Raw),
            _=>Err(wav_invalid("WAV samples have an unknown kind")),
        }
    }
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(wav_data_spec_producer())
    }
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{control.checkpoint()?;Ok(semio_framework_dsl_record::Shape::Record(wav_data_spec_producer()))}
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        let (kind, id, value) = match self {
            Self::Pcm16(values) => (0, 2, semio_framework_dsl_record::DslField::to_value(values)),
            Self::Pcm8(values) => (1, 3, semio_framework_dsl_record::DslField::to_value(values)),
            Self::Float32(values) => (2, 4, semio_framework_dsl_record::DslField::to_value(values)),
            Self::Raw(values) => (3, 5, semio_framework_dsl_record::DslField::to_value(values)),
        };
        let mut record = semio_framework_dsl_record::RecordValue::default();
        record.fields.insert(1, semio_framework_dsl_record::FieldValue::Enum(kind));
        record.fields.insert(id, value);
        semio_framework_dsl_record::FieldValue::Record(record)
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        let semio_framework_dsl_record::FieldValue::Record(record) = value else {
            return Err("WAV samples require a typed record".into());
        };
        let Some(semio_framework_dsl_record::FieldValue::Enum(kind)) = record.get(1) else {
            return Err("WAV samples require a declared kind".into());
        };
        let id = kind.checked_add(2).filter(|id| *id <= 5).ok_or("WAV samples have an unknown kind")? as u16;
        for (other, value) in &record.fields {
            if *other != 1 && *other != id && !matches!(value, semio_framework_dsl_record::FieldValue::Absent) {
                return Err("WAV sample kind selects exactly one typed array".into());
            }
        }
        let values = record.get(id).ok_or("WAV selected sample array is absent")?;
        match kind {
            0 => Ok(Self::Pcm16(semio_framework_dsl_record::DslField::from_value(values)?)),
            1 => Ok(Self::Pcm8(semio_framework_dsl_record::DslField::from_value(values)?)),
            2 => Ok(Self::Float32(semio_framework_dsl_record::DslField::from_value(values)?)),
            3 => Ok(Self::Raw(semio_framework_dsl_record::DslField::from_value(values)?)),
            _ => Err("WAV samples have an unknown kind".into()),
        }
    }
}

/// 📦️ Owned by `wav`: any auxiliary or duplicate canonical RIFF chunk, retained byte-for-byte
/// together with its word-alignment pad byte.
pub(crate) fn is_zero_byte(value: &u8) -> bool {
    *value == 0
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct RiffChunk {
    pub fourcc: String,
    #[value(default)]
    #[dsl(base64)]
    pub data: Vec<u8>,
    #[value(default, skip_serializing_if = "is_zero_byte")]
    pub pad_byte: u8,
}

/// 🧭️ One position in the top-level RIFF/WAVE chunk sequence. `Format` and `Samples`
/// reference the typed primary chunks; `Other` references `other_chunks[index]`. A duplicate
/// `fmt `/`data` chunk is deliberately an `Other` entry so its original payload survives exactly.
#[derive(Clone, Debug, PartialEq)]
pub enum WavChunkRef {
    Format,
    Samples,
    Other(u64),
}

impl semio_framework_value::ToValue for WavChunkRef {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let kind = match self { Self::Format => "format", Self::Samples => "samples", Self::Other(_) => "other" };
        let mut fields = vec![("kind".into(), semio_framework_value::DslValue::String(kind.into()))];
        if let Self::Other(index) = self { fields.push(("value".into(), semio_framework_value::DslValue::String(index.to_string()))); }
        semio_framework_value::DslValue::Object(fields)
    }
}

impl semio_framework_value::FromValue for WavChunkRef {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        match wav_tagged_fields(value)? {
            (kind, None) if kind == "format" => Ok(Self::Format),
            (kind, None) if kind == "samples" => Ok(Self::Samples),
            (kind, Some(semio_framework_value::DslValue::String(text))) if kind == "other" => {
                let index = text.parse::<u64>().map_err(|_| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "WAV chunk index requires unsigned64 decimal text"))?;
                if index.to_string() != text { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "WAV chunk index requires canonical decimal text")); }
                Ok(Self::Other(index))
            }
            _ => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "WAV chunk reference requires its exact kind and unsigned64 index")),
        }
    }
    fn edit_value_at_path(&mut self, path: &[&str], edit: semio_framework_value::ValueEdit) -> Result<(), semio_framework_value::ValueError> {
        semio_framework_value::edit_through_value(self, path, edit)
    }
}

fn wav_chunk_ref_spec() -> semio_framework_dsl_record::RecordSpec {
    semio_framework_dsl_record::RecordSpec::new(None, semio_framework_dsl_record::RecordLayout::Inline, vec![semio_framework_dsl_record::FieldSpec::new(1, "kind", semio_framework_dsl_record::Shape::Enum(vec![("format".into(), 0), ("samples".into(), 1), ("other".into(), 2)])), semio_framework_dsl_record::FieldSpec::new(2, "index", semio_framework_dsl_record::Shape::UInt).optional()])
}

fn wav_chunk_ref_spec_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::RecordSpec,semio_framework_value::ValueError>{
    control.scoped_stage(|control|{
        control.begin_stage(2)?;let mut fields=control.allocate_vec::<semio_framework_dsl_record::FieldSpec>(2)?;
        let kind=wav_enum_shape_controlled(&[("format",0),("samples",1),("other",2)],control)?;fields.push(semio_framework_dsl_record::producer::field(1,"kind",kind,control)?);control.step()?;
        fields.push(semio_framework_dsl_record::producer::field(2,"index",semio_framework_dsl_record::Shape::UInt,control)?.optional());control.step()?;
        semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Inline,fields,control)
    })
}

fn wav_chunk_ref_spec_producer()->semio_framework_dsl_record::RecordSpecProducer{semio_framework_dsl_record::RecordSpecProducer{ordinary:wav_chunk_ref_spec,decoding:|control|wav_chunk_ref_spec_controlled(control),encoding:|control|wav_chunk_ref_spec_controlled(control)}}

impl semio_framework_dsl_record::DslField for WavChunkRef {
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,semio_framework_value::ValueError>{self.to_record_controlled(control).map(semio_framework_dsl_record::FieldValue::Record)}
    fn to_record_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,semio_framework_value::ValueError>{
        control.scoped_depth(64,|control|control.scoped_stage(|control|{
            let count=if matches!(self,Self::Other(_)){2}else{1};control.begin_stage(count)?;let mut record=semio_framework_dsl_record::native_encoding::EncodedRecord::new(count,control)?;
            let kind=match self{Self::Format=>0,Self::Samples=>1,Self::Other(_)=>2};record.insert(1,semio_framework_dsl_record::FieldValue::Enum(kind))?;control.step()?;
            if let Self::Other(index)=self{record.insert(2,semio_framework_dsl_record::FieldValue::UInt(*index))?;control.step()?;}Ok(record.take())
        }))
    }
    fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{
        let semio_framework_dsl_record::FieldValue::Record(record)=value else{return Err(wav_invalid("WAV chunk reference requires a typed record"));};
        Self::from_record_controlled(record,control)
    }
    fn from_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{
        control.step()?;
        if record.fields.len()>2||record.fields.keys().any(|id|![1,2].contains(id)){return Err(wav_invalid("WAV chunk reference contains an unknown field"));}
        let index=record.get(2).filter(|value|!matches!(value,semio_framework_dsl_record::FieldValue::Absent));
        match(record.get(1),index){
            (Some(semio_framework_dsl_record::FieldValue::Enum(0)),None)=>Ok(Self::Format),
            (Some(semio_framework_dsl_record::FieldValue::Enum(1)),None)=>Ok(Self::Samples),
            (Some(semio_framework_dsl_record::FieldValue::Enum(2)),Some(semio_framework_dsl_record::FieldValue::UInt(index)))=>Ok(Self::Other(*index)),
            _=>Err(wav_invalid("WAV chunk reference requires its exact declared kind and unsigned64 index")),
        }
    }
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(wav_chunk_ref_spec_producer())
    }
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{control.checkpoint()?;Ok(semio_framework_dsl_record::Shape::Record(wav_chunk_ref_spec_producer()))}
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        let mut record = semio_framework_dsl_record::RecordValue::default();
        let kind = match self {
            Self::Format => 0,
            Self::Samples => 1,
            Self::Other(index) => {
                record.fields.insert(2, semio_framework_dsl_record::FieldValue::UInt(*index));
                2
            }
        };
        record.fields.insert(1, semio_framework_dsl_record::FieldValue::Enum(kind));
        semio_framework_dsl_record::FieldValue::Record(record)
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        let semio_framework_dsl_record::FieldValue::Record(record) = value else {
            return Err("WAV chunk reference requires a typed record".into());
        };
        if record.fields.keys().any(|id| ![1, 2].contains(id)) {
            return Err("WAV chunk reference contains an unknown field".into());
        }
        let index = record.get(2).filter(|value| !matches!(value, semio_framework_dsl_record::FieldValue::Absent));
        match (record.get(1), index) {
            (Some(semio_framework_dsl_record::FieldValue::Enum(0)), None) => Ok(Self::Format),
            (Some(semio_framework_dsl_record::FieldValue::Enum(1)), None) => Ok(Self::Samples),
            (Some(semio_framework_dsl_record::FieldValue::Enum(2)), Some(semio_framework_dsl_record::FieldValue::UInt(index))) => Ok(Self::Other(*index)),
            _ => Err("WAV chunk reference requires its exact declared kind and unsigned64 index".into()),
        }
    }
}

use framework_schema::ArtifactSchema;

//#region 🔖️Ids
pub const STDIO_WAV_DOCUMENT_SCHEMA: &str = "stdio.wav";
pub const MAXIMUM_FMT_EXTENSION_BYTES: usize = u16::MAX as usize;
//#endregion 🔖️Ids

/// 🚫️ One WAV snapshot field that cannot be represented exactly on the RIFF wire.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WavSerializationIssue {
    pub code: &'static str,
    pub message: String,
    pub target: Vec<String>,
}

fn serialization_issue(code: &'static str, message: impl Into<String>, target: impl IntoIterator<Item = impl Into<String>>) -> WavSerializationIssue {
    WavSerializationIssue { code, message: message.into(), target: target.into_iter().map(Into::into).collect() }
}

fn pad_is_representable(pad_byte: u8, payload_is_odd: bool, target: &'static str) -> Result<(), WavSerializationIssue> {
    if pad_byte != 0 && !payload_is_odd {
        return Err(serialization_issue("stdio.wav.serialization.invalid-pad-byte", format!("{target} is nonzero but its RIFF chunk payload has even length and carries no pad byte"), [target]));
    }
    Ok(())
}

/// 🧭️ Refuses snapshot states that cannot survive one exact RIFF/WAVE save and reopen.
pub fn validate_wav_serialization(snapshot: &WavSnapshot) -> Result<(), WavSerializationIssue> {
    let ext_len = snapshot.fmt.ext.as_ref().map_or(0, Vec::len);
    if ext_len > MAXIMUM_FMT_EXTENSION_BYTES {
        return Err(serialization_issue("stdio.wav.serialization.fmt-extension-too-large", format!("fmt.ext contains {ext_len} bytes; RIFF/WAVE cbSize can declare at most {MAXIMUM_FMT_EXTENSION_BYTES}"), ["fmt", "ext"]));
    }
    let fmt_payload_is_odd = snapshot.fmt.ext.as_ref().is_some_and(|ext| !ext.len().is_multiple_of(2));
    pad_is_representable(snapshot.fmt_pad_byte, fmt_payload_is_odd, "fmtPadByte")?;
    let data_payload_is_odd = match &snapshot.data {
        WavData::Pcm8(bytes) | WavData::Raw(bytes) => !bytes.len().is_multiple_of(2),
        WavData::Pcm16(_) | WavData::Float32(_) => false,
    };
    pad_is_representable(snapshot.data_pad_byte, data_payload_is_odd, "dataPadByte")?;
    for (index, chunk) in snapshot.other_chunks.iter().enumerate() {
        let bytes = chunk.fourcc.as_bytes();
        if bytes.len() != 4 || !bytes.iter().all(|byte| byte.is_ascii_graphic() || *byte == b' ') {
            return Err(serialization_issue("stdio.wav.serialization.invalid-fourcc", format!("otherChunks[{index}].fourcc must contain exactly four printable ASCII wire bytes"), ["otherChunks".to_string(), index.to_string(), "fourcc".to_string()]));
        }
        pad_is_representable(chunk.pad_byte, !chunk.data.len().is_multiple_of(2), "padByte").map_err(|mut issue| {
            issue.message = format!("otherChunks[{index}].padByte is nonzero but its RIFF chunk payload has even length and carries no pad byte");
            issue.target = vec!["otherChunks".into(), index.to_string(), "padByte".into()];
            issue
        })?;
    }
    Ok(())
}

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.wav")]
pub struct WavSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub fmt: WavFmt,
    #[state(artifact)]
    pub data: WavData,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "is_zero_byte")]
    pub fmt_pad_byte: u8,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "is_zero_byte")]
    pub data_pad_byte: u8,
    #[state(artifact)]
    #[value(default)]
    pub other_chunks: Vec<RiffChunk>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub chunk_order: Vec<WavChunkRef>,
}

impl Default for WavSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_WAV_DOCUMENT_SCHEMA.into(), fmt: WavFmt::default(), data: WavData::Pcm16(Vec::new()), fmt_pad_byte: 0, data_pad_byte: 0, other_chunks: Vec::new(), chunk_order: vec![WavChunkRef::Format, WavChunkRef::Samples] }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
