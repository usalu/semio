//! 🛫️ Borrowed GLTF logical records admit every owned field before physical native emission.
use super::*;
use semio_framework_dsl_record::native_encoding::retire_field;
use semio_framework_dsl_record::native_encoding::EncodedRecord;
use semio_framework_dsl_record::FieldValue;
use semio_framework_value::NativeEncodeControl;
use semio_framework_dsl_record::RecordValue;
use semio_framework_value::{ValueError,ValueRefusalKind};


fn emit<T: DslField>(value: &T, control: &mut NativeEncodeControl<'_>) -> Result<FieldValue, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(0)?;
        value.to_value_controlled(control)
    })
}
fn put(record: &mut EncodedRecord, id: u16, value: FieldValue, control: &mut NativeEncodeControl<'_>) -> Result<(), ValueError> {
    record.insert(id, value)?;
    control.step()
}
fn build(count: usize, control: &mut NativeEncodeControl<'_>, construct: impl FnOnce(&mut EncodedRecord, &mut NativeEncodeControl<'_>) -> Result<(), ValueError>) -> Result<FieldValue, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(count)?;
        let mut record = semio_framework_dsl_record::native_encoding::EncodedRecord::new(count, control)?;
        construct(&mut record, control)?;
        Ok(semio_framework_dsl_record::FieldValue::Record(record.take()))
    })
}
fn list<T>(values: &[T], control: &mut NativeEncodeControl<'_>, mut project: impl FnMut(&T, &mut NativeEncodeControl<'_>) -> Result<FieldValue, ValueError>) -> Result<FieldValue, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(values.len())?;
        let mut result = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.allocate_vec(values.len())?, |values: Vec<FieldValue>| {
            for value in values {
                retire_field(value)
            }
        });
        for value in values {
            result.as_mut().push(control.scoped_stage(|control| project(value, control))?);
            control.step()?;
        }
        Ok(semio_framework_dsl_record::FieldValue::List(result.take()))
    })
}
fn absent(control: &mut NativeEncodeControl<'_>) -> Result<FieldValue, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(1)?;
        control.step()?;
        Ok(semio_framework_dsl_record::FieldValue::Absent)
    })
}
fn optional_field<T:DslField>(value:&Option<T>,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{match value{Some(value)=>emit(value,control),None=>absent(control)}}
fn optional_json_value(value: &Option<GltfJson>, control: &mut NativeEncodeControl<'_>) -> Result<FieldValue, ValueError> {
    match value {
        Some(value) => json_value(value, control),
        None => absent(control),
    }
}
fn attributes(values: &[(String, usize)], control: &mut NativeEncodeControl<'_>) -> Result<FieldValue, ValueError> {
    list(values, control, |(semantic, accessor), control| {
        build(2, control, |record, control| {
            put(record, 0, emit(semantic, control)?, control)?;
            put(record, 1, emit(accessor, control)?, control)
        })
    })
}

/// 🌱️ Emits an actual borrowed morph target without constructing a temporary owned target.
fn morph_value(value: &GltfMorphTarget, control: &mut NativeEncodeControl<'_>) -> Result<FieldValue, ValueError> {
    build(1, control, |record, control| put(record, 0, attributes(&value.0, control)?, control))
}
/// 🔺️ Emits every primitive member in its existing zero-based logical record.
fn primitive_value(value: &GltfPrimitive, control: &mut NativeEncodeControl<'_>) -> Result<FieldValue, ValueError> {
    build(7, control, |record, control| {
        put(record, 0, attributes(&value.attributes, control)?, control)?;
        put(record, 1, optional_field(&value.indices, control)?, control)?;
        put(record, 2, optional_field(&value.material, control)?, control)?;
        put(record, 3, optional_field(&value.mode, control)?, control)?;
        put(record, 4, list(&value.targets, control, morph_value)?, control)?;
        put(record, 5, optional_json_value(&value.extensions, control)?, control)?;
        put(record, 6, optional_json_value(&value.extras, control)?, control)
    })
}
/// 🎥️ Preserves the declared exclusive perspective or orthographic projection record.
fn camera_value(value: &GltfCameraProjection, control: &mut NativeEncodeControl<'_>) -> Result<FieldValue, ValueError> {
    build(3, control, |record, control| match value {
        GltfCameraProjection::Perspective(value) => {
            put(record, 0, emit(&ProjectionKind::Perspective, control)?, control)?;
            put(record, 1, emit(value, control)?, control)?;
            put(record, 2, absent(control)?, control)
        }
        GltfCameraProjection::Orthographic(value) => {
            put(record, 0, emit(&ProjectionKind::Orthographic, control)?, control)?;
            put(record, 1, absent(control)?, control)?;
            put(record, 2, emit(value, control)?, control)
        }
    })
}
/// 🖼️ Retains every optional image member and independently owned extras.
fn image_value(value: &GltfImage, control: &mut NativeEncodeControl<'_>) -> Result<FieldValue, ValueError> {
    build(7, control, |record, control| {
        put(record, 0, emit(&ImageKind::Image, control)?, control)?;
        put(record, 1, optional_field(&value.uri, control)?, control)?;
        put(record, 2, optional_field(&value.mime_type, control)?, control)?;
        put(record, 3, optional_field(&value.buffer_view, control)?, control)?;
        put(record, 4, optional_field(&value.name, control)?, control)?;
        put(record, 5, optional_json_value(&value.extensions, control)?, control)?;
        put(record, 6, optional_json_value(&value.extras, control)?, control)
    })
}
/// 🧵️ Retains every optional texture member without native snapshot clones.
fn texture_value(value: &GltfTexture, control: &mut NativeEncodeControl<'_>) -> Result<FieldValue, ValueError> {
    build(6, control, |record, control| {
        put(record, 0, emit(&TextureKind::Texture, control)?, control)?;
        put(record, 1, optional_field(&value.sampler, control)?, control)?;
        put(record, 2, optional_field(&value.source, control)?, control)?;
        put(record, 3, optional_field(&value.name, control)?, control)?;
        put(record, 4, optional_json_value(&value.extensions, control)?, control)?;
        put(record, 5, optional_json_value(&value.extras, control)?, control)
    })
}

fn reserve_frontier<'a>(frontier: &mut Vec<&'a GltfJson>, additional: usize, control: &mut NativeEncodeControl<'_>) -> Result<(), ValueError> {
    let required = frontier.len().checked_add(additional).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"GLTF extras frontier overflow"))?;
    if required > frontier.capacity() {
        let capacity = required.max(frontier.capacity().checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"GLTF extras frontier overflow"))?).max(4);
        let mut replacement = control.allocate_vec(capacity)?;
        replacement.extend(frontier.iter().copied());
        *frontier = replacement;
    }
    Ok(())
}
fn json_frontier<'a>(root: &'a GltfJson, control: &mut NativeEncodeControl<'_>) -> Result<Vec<&'a GltfJson>, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(0)?;
        let mut result = Vec::new();
        reserve_frontier(&mut result, 1, control)?;
        result.push(root);
        let mut index = 0;
        while index < result.len() {
            match result[index] {
                GltfJson::Array(values) => {
                    reserve_frontier(&mut result, values.len(), control)?;
                    for value in values {
                        result.push(value);
                        control.step()?;
                    }
                }
                GltfJson::Object(values) => {
                    reserve_frontier(&mut result, values.len(), control)?;
                    for (_, value) in values {
                        result.push(value);
                        control.step()?;
                    }
                }
                _ => {}
            }
            index += 1;
            control.step()?;
        }
        Ok(result)
    })
}
fn json_member(name: &str, index: u64, control: &mut NativeEncodeControl<'_>) -> Result<FieldValue, ValueError> {
    build(2, control, |record, control| {
        put(record, 0, semio_framework_dsl_record::FieldValue::Text(control.copy_text(name)?), control)?;
        put(record, 1, emit(&index, control)?, control)
    })
}
fn json_node(value: &GltfJson, next: &mut u64, control: &mut NativeEncodeControl<'_>) -> Result<FieldValue, ValueError> {
    build(6, control, |record, control| {
        let kind = match value {
            GltfJson::Null => 0,
            GltfJson::Bool(_) => 1,
            GltfJson::Number(_) => 2,
            GltfJson::String(_) => 3,
            GltfJson::Array(_) => 4,
            GltfJson::Object(_) => 5,
        };
        put(record, 0, semio_framework_dsl_record::FieldValue::Enum(kind), control)?;
        put(
            record,
            1,
            match value {
                GltfJson::Bool(value) => emit(value, control)?,
                _ => absent(control)?,
            },
            control,
        )?;
        put(
            record,
            2,
            match value {
                GltfJson::Number(value) => emit(value, control)?,
                _ => absent(control)?,
            },
            control,
        )?;
        put(
            record,
            3,
            match value {
                GltfJson::String(value) => semio_framework_dsl_record::FieldValue::Text(control.copy_text(value)?),
                _ => absent(control)?,
            },
            control,
        )?;
        let items = match value {
            GltfJson::Array(values) => list(values, control, |_, control| {
                let index = *next;
                *next = next.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"GLTF extras identity overflow"))?;
                emit(&index, control)
            })?,
            _ => semio_framework_dsl_record::FieldValue::List(control.allocate_vec(0)?),
        };
        put(record, 4, items, control)?;
        let members = match value {
            GltfJson::Object(values) => list(values, control, |(name, _), control| {
                let index = *next;
                *next = next.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"GLTF extras identity overflow"))?;
                json_member(name, index, control)
            })?,
            _ => semio_framework_dsl_record::FieldValue::List(control.allocate_vec(0)?),
        };
        put(record, 5, members, control)
    })
}
/// 🧩️ Emits a flat explicit extras graph with paid BFS frontiers and literal ordered duplicate members.
fn json_value(value: &GltfJson, control: &mut NativeEncodeControl<'_>) -> Result<FieldValue, ValueError> {
    let frontier = json_frontier(value, control)?;
    let mut next = 1;
    build(1, control, |record, control| put(record, 0, list(&frontier, control, |value, control| json_node(value, &mut next, control))?, control))
}
/// 📸️ Projects the four actual root fields and resolved buffers without a native mirror clone.
fn snapshot_value(value: &GltfSnapshot, control: &mut NativeEncodeControl<'_>) -> Result<RecordValue, ValueError> {
    let semio_framework_dsl_record::FieldValue::Record(record) = build(4, control, |record, control| {
        put(record, 0, emit(&value.schema, control)?, control)?;
        put(record, 1, emit(&value.document, control)?, control)?;
        put(record, 2, list(&value.buffers, control, |bytes, control| build(1, control, |record, control| put(record, 0, emit(bytes, control)?, control)))?, control)?;
        put(record, 3, emit(&value.source_form, control)?, control)
    })?
    else {
        unreachable!("authored GLTF record")
    };
    Ok(record)
}

/// 🏭️ Uses the actual generated flat extras metadata producer.
pub(super) fn json_shape<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
    json::Json::shape_controlled(control)
}
/// 🏭️ Uses the actual authored morph target metadata producer.
pub(super) fn morph_shape<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
    Target::shape_controlled(control)
}
/// 🏭️ Uses the actual authored primitive metadata producer.
pub(super) fn primitive_shape<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
    Primitive::shape_controlled(control)
}
/// 🏭️ Uses the actual authored camera projection metadata producer.
pub(super) fn camera_shape<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
    Projection::shape_controlled(control)
}
/// 🏭️ Uses the actual authored image metadata producer.
pub(super) fn image_shape<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
    Image::shape_controlled(control)
}
/// 🏭️ Uses the actual authored texture metadata producer.
pub(super) fn texture_shape<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
    Texture::shape_controlled(control)
}
/// 📑️ Retains ordinary, decoding, and encoding metadata factories without constructing metadata.
pub(super) fn spec_producer() -> semio_framework_dsl_record::RecordSpecProducer {
    Snapshot::__dsl_spec_producer()
}

/// 🛫️ Retains typed admission through the declared controlled field boundary.
pub(super) fn morph(value:&GltfMorphTarget,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{morph_value(value,control)}

/// 🛫️ Retains typed admission through the declared controlled field boundary.
pub(super) fn primitive(value:&GltfPrimitive,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{primitive_value(value,control)}

/// 🛫️ Retains typed admission through the declared controlled field boundary.
pub(super) fn camera(value:&GltfCameraProjection,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{camera_value(value,control)}

/// 🛫️ Retains typed admission through the declared controlled field boundary.
pub(super) fn image(value:&GltfImage,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{image_value(value,control)}

/// 🛫️ Retains typed admission through the declared controlled field boundary.
pub(super) fn texture(value:&GltfTexture,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{texture_value(value,control)}

/// 🛫️ Retains typed admission through the declared controlled field boundary.
pub(super) fn json(value:&GltfJson,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{json_value(value,control)}

/// 📸️ Retains canonical refusals through the controlled logical-record boundary.
pub(super) fn snapshot(value:&GltfSnapshot,control:&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>{snapshot_value(value,control)}

/// 🛫️ Uses the declared output terminal with the owner’s controlled borrowed field producer.
pub(super) fn encode_native(value:&GltfSnapshot,encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io::IoPayload,ValueError>{store::encode_sqlite_snapshot_record_native(encoding,"stdio.gltf",spec_producer(),|native|snapshot_value(value,native),control,native_owner)}
