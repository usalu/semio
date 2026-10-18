//! 🧮️ Buffers of the remodeling snapshot: the float sample buffer and the byte buffer with their DSL field bindings. See [`super`].

use super::asset_handles_and_durable_content::{remodeling_durable_chunk, RemodelingDurableArtifactStore};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Buffers
/// ☁️ Literal samples or an explicit reference to snapshot-owned raw content.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Float32Buffer {
    #[dsl(key = "inline")]
    Inline { values: Vec<f32> },
    #[dsl(key = "content")]
    Content { content_id: String, chunk_count: u64 },
}

impl Default for Float32Buffer {
    fn default() -> Self { Self::Inline { values: Vec::new() } }
}

impl Float32Buffer {
    pub fn from_f32_slice(values: &[f32]) -> Self { Self::Inline { values: values.to_vec() } }
    pub fn to_f32_vec(&self) -> Vec<f32> { match self { Self::Inline { values } => values.clone(), Self::Content { .. } => Vec::new() } }
    pub fn content_reference(&self) -> Option<(&str, u64)> { match self { Self::Content { content_id, chunk_count } => Some((content_id, *chunk_count)), Self::Inline { .. } => None } }
    pub fn to_f32_vec_from(&self, store: &RemodelingDurableArtifactStore) -> Vec<f32> {
        let Some((content_id, chunk_count)) = self.content_reference() else { return self.to_f32_vec() };
        let Some(artifact) = store.get(content_id).filter(|artifact| artifact.kind == "sparse" && u64::try_from(artifact.chunks.len()).ok() == Some(chunk_count)) else { return Vec::new() };
        let mut values = Vec::new();
        for leaf in &artifact.chunks {
            let Some(bytes) = remodeling_durable_chunk(leaf) else { return Vec::new() };
            let (chunks, remainder) = bytes.as_chunks::<4>();
            if !remainder.is_empty() { return Vec::new(); }
            values.extend(chunks.iter().map(|chunk| f32::from_le_bytes(*chunk)));
        }
        values
    }
    pub fn is_empty(&self) -> bool { match self { Self::Inline { values } => values.is_empty(), Self::Content { chunk_count, .. } => *chunk_count == 0 } }
}

impl semio_framework_dsl_record::BorrowedDslField for Float32Buffer { const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Value; }

impl semio_framework_dsl_record::DslField for Float32Buffer {
    fn shape() -> semio_framework_dsl_record::Shape { semio_framework_dsl_record::Shape::Block(Box::new(semio_framework_dsl_record::Shape::Statements(<Self as semio_framework_dsl_record::DslVariants>::variants()))) }
    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, semio_framework_value::ValueError> {
        let shape = semio_framework_dsl_record::Shape::Statements(<Self as semio_framework_dsl_record::DslVariants>::variants_controlled(control)?);
        Ok(semio_framework_dsl_record::Shape::Block(semio_framework_dsl_record::producer::boxed(shape, control)?))
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue { semio_framework_dsl_record::FieldValue::Block(Box::new(semio_framework_dsl_record::FieldValue::Statements(vec![<Self as semio_framework_dsl_record::DslVariants>::to_named_record(self)]))) }
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::FieldValue, semio_framework_value::ValueError> {
        let mut statements = control.allocate_vec(1)?;
        control.charge(size_of::<semio_framework_dsl_record::FieldValue>())?;
        statements.push(<Self as semio_framework_dsl_record::DslVariants>::to_named_record_controlled(self, control)?);
        Ok(semio_framework_dsl_record::FieldValue::Block(Box::new(semio_framework_dsl_record::FieldValue::Statements(statements))))
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        let semio_framework_dsl_record::FieldValue::Block(value) = value else { return Err("expected a float-buffer block".into()) };
        let semio_framework_dsl_record::FieldValue::Statements(statements) = value.as_ref() else { return Err("expected a float-buffer variant".into()) };
        let [(keyword, record)] = statements.as_slice() else { return Err("expected one float-buffer variant".into()) };
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, record).map_err(|error| error.to_string())
    }
    fn from_value_controlled(value: &semio_framework_dsl_record::FieldValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        control.checkpoint()?;
        let semio_framework_dsl_record::FieldValue::Block(value) = value else { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected a float-buffer block")) };
        let semio_framework_dsl_record::FieldValue::Statements(statements) = value.as_ref() else { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected a float-buffer variant")) };
        let [(keyword, record)] = statements.as_slice() else { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected one float-buffer variant")) };
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record_controlled(keyword, record, control)
    }
}

/// 🧱 Literal octets owned by a snapshot or a typed reconstruction operation.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(transparent)]
#[serde(transparent)]
pub struct ByteBuffer(pub Vec<u8>);

impl ByteBuffer {
    pub fn from_u8_slice(values: &[u8]) -> Self { Self(values.to_vec()) }
    pub fn to_u8_vec(&self) -> Vec<u8> { self.0.clone() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

impl semio_framework_dsl_record::BorrowedDslField for ByteBuffer { const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Bytes64; }

impl semio_framework_dsl_record::DslField for ByteBuffer {
    fn shape() -> semio_framework_dsl_record::Shape { semio_framework_dsl_record::Shape::Bytes64 }
    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, semio_framework_value::ValueError> { control.checkpoint()?; Ok(semio_framework_dsl_record::Shape::Bytes64) }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue { semio_framework_dsl_record::FieldValue::Bytes64(self.0.clone()) }
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::FieldValue, semio_framework_value::ValueError> { control.copy_bytes(&self.0).map(semio_framework_dsl_record::FieldValue::Bytes64) }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> { match value { semio_framework_dsl_record::FieldValue::Bytes64(bytes) => Ok(Self(bytes.clone())), _ => Err("expected literal octets".into()) } }
    fn from_value_controlled(value: &semio_framework_dsl_record::FieldValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> { match value { semio_framework_dsl_record::FieldValue::Bytes64(bytes) => control.copy_bytes(bytes).map(Self), _ => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected literal octets")) } }
}
//#endregion 🔖️Buffers
