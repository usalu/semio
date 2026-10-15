//! 🔗️ Canonical durable link records, borrowed schema and controlled native codecs.
use super::{ArtifactLink,LinkPin};
use semio_framework_os_store_blob_identity::BlobRef;
use semio_framework_value::ValueError;
use semio_framework_dsl_record::RecordSpecProducer;

// 🚫️async: E4 fn-pointer slot — value stored in `Shape::Record(fn() -> RecordSpec)`
pub(crate) fn link_pin_spec() -> semio_framework_dsl_record::RecordSpec {
    semio_framework_dsl_record::RecordSpec::new(
        None,
        semio_framework_dsl_record::RecordLayout::Inline,
        vec![
            semio_framework_dsl_record::FieldSpec::new(0, "kind", semio_framework_dsl_record::Shape::Enum(vec![("head".to_string(), 0), ("checkpoint".to_string(), 1), ("snapshot".to_string(), 2)])),
            semio_framework_dsl_record::FieldSpec::new(1, "checkpoint_id", semio_framework_dsl_record::Shape::Text).optional(),
            semio_framework_dsl_record::FieldSpec::new(2, "blob_hash", semio_framework_dsl_record::Shape::Text).optional(),
            semio_framework_dsl_record::FieldSpec::new(3, "blob_size", semio_framework_dsl_record::Shape::UInt).optional(),
            semio_framework_dsl_record::FieldSpec::new(4, "blob_media_type", semio_framework_dsl_record::Shape::Text).optional(),
        ],
    )
}

pub(crate) fn link_pin_spec_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::RecordSpec, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(5)?;
        let mut fields = control.allocate_vec(5)?;
        let mut labels = control.allocate_vec(3)?;
        for (key, id) in [("head", 0), ("checkpoint", 1), ("snapshot", 2)] {
            labels.push((control.copy_text(key)?, id));
        }
        fields.push(semio_framework_dsl_record::producer::field(0, "kind", semio_framework_dsl_record::Shape::Enum(labels), control)?);
        control.step()?;
        for (id, key, shape) in [(1, "checkpoint_id", semio_framework_dsl_record::Shape::Text), (2, "blob_hash", semio_framework_dsl_record::Shape::Text), (3, "blob_size", semio_framework_dsl_record::Shape::UInt), (4, "blob_media_type", semio_framework_dsl_record::Shape::Text)] {
            fields.push(semio_framework_dsl_record::producer::field(id, key, shape, control)?.optional());
            control.step()?;
        }
        semio_framework_dsl_record::producer::record(None, semio_framework_dsl_record::RecordLayout::Inline, fields, control)
    })
}

pub(crate) fn link_pin_spec_producer() -> RecordSpecProducer {
    RecordSpecProducer { ordinary: link_pin_spec, decoding: |control| link_pin_spec_controlled(control), encoding: |control| link_pin_spec_controlled(control) }
}

pub(crate) fn link_pin_to_record(pin: &LinkPin) -> semio_framework_dsl_record::RecordValue {
    let mut record = semio_framework_dsl_record::RecordValue::default();
    match pin {
        LinkPin::Head => {
            record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(0));
        }
        LinkPin::Checkpoint { id } => {
            record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(1));
            record.fields.insert(1, semio_framework_dsl_record::FieldValue::Text(id.clone()));
        }
        LinkPin::Snapshot { blob } => {
            record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(2));
            record.fields.insert(2, semio_framework_dsl_record::FieldValue::Text(blob.hash.clone()));
            record.fields.insert(3, semio_framework_dsl_record::FieldValue::UInt(blob.size));
            record.fields.insert(4, semio_framework_dsl_record::FieldValue::Text(blob.media_type.clone()));
        }
    }
    record
}

pub(crate) fn link_pin_to_record_controlled(pin: &LinkPin, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::RecordValue, ValueError> {
    
    control.scoped_depth(64, |control| {
        control.scoped_stage(|control| {
            let count = match pin {
                LinkPin::Head => 1,
                LinkPin::Checkpoint { .. } => 2,
                LinkPin::Snapshot { .. } => 4,
            };
            control.begin_stage(count)?;
            let mut output = semio_framework_dsl_record::native_encoding::EncodedRecord::new(count, control)?;
            let ordinal = match pin {
                LinkPin::Head => 0,
                LinkPin::Checkpoint { .. } => 1,
                LinkPin::Snapshot { .. } => 2,
            };
            output.insert(0, semio_framework_dsl_record::FieldValue::Enum(ordinal))?;
            control.step()?;
            match pin {
                LinkPin::Head => {}
                LinkPin::Checkpoint { id } => {
                    output.insert(1, semio_framework_dsl_record::FieldValue::Text(control.copy_text(id)?))?;
                    control.step()?;
                }
                LinkPin::Snapshot { blob } => {
                    output.insert(2, semio_framework_dsl_record::FieldValue::Text(control.copy_text(&blob.hash)?))?;
                    control.step()?;
                    output.insert(3, semio_framework_dsl_record::FieldValue::UInt(blob.size))?;
                    control.step()?;
                    output.insert(4, semio_framework_dsl_record::FieldValue::Text(control.copy_text(&blob.media_type)?))?;
                    control.step()?;
                }
            }
            Ok(output.take())
        })
    })
}

pub(crate) fn validate_link_pin_record(record: &semio_framework_dsl_record::RecordValue) -> Result<u32, String> {
    let Some(semio_framework_dsl_record::FieldValue::Enum(ordinal)) = record.get(0) else {
        return Err("expected link pin kind Enum".into());
    };
    let fields: &[u16] = match ordinal {
        0 => &[0],
        1 => &[0, 1],
        2 => &[0, 2, 3, 4],
        _ => return Err("unknown link pin kind ordinal".into()),
    };
    if record.fields.keys().any(|id| *id > 4)
        || fields.iter().any(|id| matches!(record.get(*id), None | Some(semio_framework_dsl_record::FieldValue::Absent)))
        || record.fields.iter().any(|(id, value)| !matches!(value, semio_framework_dsl_record::FieldValue::Absent) && !fields.contains(id))
    {
        return Err("link pin fields differ from its declared branch".into());
    }
    Ok(*ordinal)
}

pub(crate) fn link_pin_from_record_controlled(record: &semio_framework_dsl_record::RecordValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<LinkPin, ValueError> {
    control.scoped_stage(|control| {
        control.checkpoint()?;
        let ordinal = validate_link_pin_record(record).map_err(|message| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message))?;
        control.begin_stage(match ordinal {
            0 => 1,
            1 => 2,
            2 => 4,
            _ => unreachable!(),
        })?;
        control.step()?;
        let text = |id| match record.get(id) {
            Some(semio_framework_dsl_record::FieldValue::Text(value)) => Ok(value.as_str()),
            _ => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "link pin field must be Text")),
        };
        match ordinal {
            0 => Ok(LinkPin::Head),
            1 => {
                let id = control.copy_text(text(1)?)?;
                control.step()?;
                Ok(LinkPin::Checkpoint { id })
            }
            2 => {
                let hash = text(2)?;
                let media_type = text(4)?;
                let Some(semio_framework_dsl_record::FieldValue::UInt(size)) = record.get(3) else {
                    return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "link pin blob size must be UInt"));
                };
                let hash = control.copy_text(hash)?;
                control.step()?;
                let size = *size;
                control.step()?;
                let media_type = control.copy_text(media_type)?;
                control.step()?;
                Ok(LinkPin::Snapshot { blob: BlobRef { hash, size, media_type } })
            }
            _ => unreachable!(),
        }
    })
}

pub(crate) fn link_pin_from_record(record: &semio_framework_dsl_record::RecordValue) -> Result<LinkPin, String> {
    validate_link_pin_record(record)?;
    let ordinal = match record.get(0) {
        Some(semio_framework_dsl_record::FieldValue::Enum(n)) => *n,
        other => return Err(format!("expected kind, found {other:?}")),
    };
    match ordinal {
        0 => Ok(LinkPin::Head),
        1 => {
            let id = match record.get(1) {
                Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
                other => return Err(format!("expected checkpoint_id, found {other:?}")),
            };
            Ok(LinkPin::Checkpoint { id })
        }
        2 => {
            let hash = match record.get(2) {
                Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
                other => return Err(format!("expected blob_hash, found {other:?}")),
            };
            let size = match record.get(3) {
                Some(semio_framework_dsl_record::FieldValue::UInt(n)) => *n,
                other => return Err(format!("expected blob_size, found {other:?}")),
            };
            let media_type = match record.get(4) {
                Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
                other => return Err(format!("expected blob_media_type, found {other:?}")),
            };
            Ok(LinkPin::Snapshot { blob: BlobRef { hash, size, media_type } })
        }
        other => Err(format!("unknown link pin kind ordinal {other}")),
    }
}

// 🚫️async: E4 fn-pointer slot — value stored in `Shape::Record(fn() -> RecordSpec)`
pub(crate) fn link_pin_borrowed_spec()->semio_framework_dsl_record::BorrowedRecordSpec{
 use semio_framework_dsl_record::{BorrowedFieldSpec as F,BorrowedShape as H};
 const fn optional(mut field:F)->F{field.optional=true;field}
 static KINDS:[(&str,u32);3]=[("head",0),("checkpoint",1),("snapshot",2)];
 static FIELDS:[F;5]=[F::new(0,"kind",H::Enum(&KINDS)),optional(F::new(1,"checkpoint_id",H::Text)),optional(F::new(2,"blob_hash",H::Text)),optional(F::new(3,"blob_size",H::UInt)),optional(F::new(4,"blob_media_type",H::Text))];
 semio_framework_dsl_record::BorrowedRecordSpec{keyword:None,layout:semio_framework_dsl_record::RecordLayout::Inline,fields:&FIELDS}
}

pub(crate) fn artifact_link_spec() -> semio_framework_dsl_record::RecordSpec {
    semio_framework_dsl_record::RecordSpec::new(
        None,
        semio_framework_dsl_record::RecordLayout::Inline,
        vec![
            semio_framework_dsl_record::FieldSpec::new(0, "target", <semio_framework_artifact_reference::ArtifactRef as semio_framework_dsl_record::DslField>::shape()),
            semio_framework_dsl_record::FieldSpec::new(1, "pin", semio_framework_dsl_record::Shape::Record(link_pin_spec_producer())),
            semio_framework_dsl_record::FieldSpec::new(2, "role", semio_framework_dsl_record::Shape::Text),
        ],
    )
}

pub(crate) fn artifact_link_spec_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::RecordSpec, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(3)?;
        let mut fields = control.allocate_vec(3)?;
        let target = <semio_framework_artifact_reference::ArtifactRef as semio_framework_dsl_record::DslField>::shape_controlled(control)?;
        for (id, key, shape) in [(0, "target", target), (1, "pin", semio_framework_dsl_record::Shape::Record(link_pin_spec_producer())), (2, "role", semio_framework_dsl_record::Shape::Text)] {
            fields.push(semio_framework_dsl_record::producer::field(id, key, shape, control)?);
            control.step()?;
        }
        semio_framework_dsl_record::producer::record(None, semio_framework_dsl_record::RecordLayout::Inline, fields, control)
    })
}

pub(crate) fn artifact_link_spec_producer() -> RecordSpecProducer {
    RecordSpecProducer { ordinary: artifact_link_spec, decoding: |control| artifact_link_spec_controlled(control), encoding: |control| artifact_link_spec_controlled(control) }
}

pub(crate) fn artifact_link_to_record(link: &ArtifactLink) -> semio_framework_dsl_record::RecordValue {
    let mut record = semio_framework_dsl_record::RecordValue::default();
    record.fields.insert(0, <semio_framework_artifact_reference::ArtifactRef as semio_framework_dsl_record::DslField>::to_value(&link.target));
    record.fields.insert(1, semio_framework_dsl_record::FieldValue::Record(link_pin_to_record(&link.pin)));
    record.fields.insert(2, semio_framework_dsl_record::FieldValue::Text(link.role.clone()));
    record
}

pub(crate) fn artifact_link_to_record_controlled(link: &ArtifactLink, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::RecordValue, ValueError> {
    use semio_framework_dsl_record::DslField;
    control.scoped_depth(64, |control| {
        control.scoped_stage(|control| {
            control.begin_stage(3)?;
            let mut output = semio_framework_dsl_record::native_encoding::EncodedRecord::new(3, control)?;
            output.insert(0, <semio_framework_artifact_reference::ArtifactRef as DslField>::to_value_controlled(&link.target, control)?)?;
            control.step()?;
            output.insert(1, semio_framework_dsl_record::FieldValue::Record(link_pin_to_record_controlled(&link.pin, control)?))?;
            control.step()?;
            output.insert(2, semio_framework_dsl_record::FieldValue::Text(control.copy_text(&link.role)?))?;
            control.step()?;
            Ok(output.take())
        })
    })
}

pub(crate) fn artifact_link_from_record(record: &semio_framework_dsl_record::RecordValue) -> Result<ArtifactLink, String> {
    if record.fields.len() != 3 || record.fields.keys().any(|id| *id > 2) {
        return Err("artifact link requires its three declared fields".into());
    }
    let target = <semio_framework_artifact_reference::ArtifactRef as semio_framework_dsl_record::DslField>::from_value(record.get(0).ok_or("artifact link requires target")?)?;
    let pin = match record.get(1) {
        Some(semio_framework_dsl_record::FieldValue::Record(record)) => link_pin_from_record(record)?,
        other => return Err(format!("expected pin, found {other:?}")),
    };
    let role = match record.get(2) {
        Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
        other => return Err(format!("expected role, found {other:?}")),
    };
    Ok(ArtifactLink { target, pin, role })
}

pub(crate) fn artifact_link_from_record_controlled(record: &semio_framework_dsl_record::RecordValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<ArtifactLink, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(3)?;
        if record.fields.len() != 3 || record.fields.keys().any(|id| *id > 2) {
            return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "artifact link requires its three declared fields"));
        }
        let target = record.get(0).ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "artifact link requires target"))?;
        let Some(semio_framework_dsl_record::FieldValue::Record(pin)) = record.get(1) else {
            return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "artifact link pin must be Record"));
        };
        let Some(semio_framework_dsl_record::FieldValue::Text(role)) = record.get(2) else {
            return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "artifact link role must be Text"));
        };
        validate_link_pin_record(pin).map_err(|message| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message))?;
        let target = <semio_framework_artifact_reference::ArtifactRef as semio_framework_dsl_record::DslField>::from_value_controlled(target, control)?;
        control.step()?;
        let pin = link_pin_from_record_controlled(pin, control)?;
        control.step()?;
        let role = control.copy_text(role)?;
        control.step()?;
        Ok(ArtifactLink { target, pin, role })
    })
}

impl semio_framework_dsl_record::DslField for LinkPin {
    /// 🚫️async: E4 — see `DslField::shape`'s tag on the trait.
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(link_pin_spec_producer())
    }
    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
        control.checkpoint()?;
        Ok(semio_framework_dsl_record::Shape::Record(link_pin_spec_producer()))
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Record(link_pin_to_record(self))
    }
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::FieldValue, ValueError> {
        link_pin_to_record_controlled(self, control).map(semio_framework_dsl_record::FieldValue::Record)
    }
    fn to_record_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::RecordValue, ValueError> {
        link_pin_to_record_controlled(self, control)
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Record(record) => link_pin_from_record(record),
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &semio_framework_dsl_record::FieldValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        let semio_framework_dsl_record::FieldValue::Record(record) = value else {
            return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected link pin Record"));
        };
        link_pin_from_record_controlled(record, control)
    }
    fn from_record_controlled(record: &semio_framework_dsl_record::RecordValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        link_pin_from_record_controlled(record, control)
    }
}
impl semio_framework_dsl_record::BorrowedDslRecord for ArtifactLink{
 const RECORD:semio_framework_dsl_record::BorrowedRecordSpec=semio_framework_dsl_record::BorrowedRecordSpec{keyword:None,layout:semio_framework_dsl_record::RecordLayout::Inline,fields:&[
 semio_framework_dsl_record::BorrowedFieldSpec::new(0,"target",<semio_framework_artifact_reference::ArtifactRef as semio_framework_dsl_record::BorrowedDslField>::SHAPE),
 semio_framework_dsl_record::BorrowedFieldSpec::new(1,"pin",semio_framework_dsl_record::BorrowedShape::Record(link_pin_borrowed_spec)),
 semio_framework_dsl_record::BorrowedFieldSpec::new(2,"role",semio_framework_dsl_record::BorrowedShape::Text)]};
}
impl semio_framework_dsl_record::BorrowedDslField for ArtifactLink{const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>);}

impl semio_framework_dsl_record::DslField for ArtifactLink {
    /// 🚫️async: E4 — see `DslField::shape`'s tag on the trait.
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(artifact_link_spec_producer())
    }
    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
        control.checkpoint()?;
        Ok(semio_framework_dsl_record::Shape::Record(artifact_link_spec_producer()))
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Record(artifact_link_to_record(self))
    }
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::FieldValue, ValueError> {
        artifact_link_to_record_controlled(self, control).map(semio_framework_dsl_record::FieldValue::Record)
    }
    fn to_record_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::RecordValue, ValueError> {
        artifact_link_to_record_controlled(self, control)
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Record(record) => artifact_link_from_record(record),
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &semio_framework_dsl_record::FieldValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        let semio_framework_dsl_record::FieldValue::Record(record) = value else {
            return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected artifact link Record"));
        };
        artifact_link_from_record_controlled(record, control)
    }
    fn from_record_controlled(record: &semio_framework_dsl_record::RecordValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        artifact_link_from_record_controlled(record, control)
    }
}
