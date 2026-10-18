//! 🏠️ Ownership stamp stored on each independently persisted child envelope.
use {semio_framework_artifact_reference::ArtifactRef};
use semio_framework_value::ValueError;
use semio_framework_value_derive::{FromValue, ToValue};

/// 🪪️ Exact owning parent, declared slot and durable child identity.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct OwnerRef {
    pub parent: ArtifactRef,
    pub slot: String,
    pub child_id: String,
}

semio_framework_value::artifact_retire_struct!(OwnerRef { parent, slot, child_id });

//#region 🔖️OwnerRefDslField
// 🚫️async: E4 fn-pointer slot — value stored in `Shape::Record(fn() -> RecordSpec)`
pub fn owner_ref_spec() -> semio_framework_dsl_record::RecordSpec {
    semio_framework_dsl_record::RecordSpec::new(
        None,
        semio_framework_dsl_record::RecordLayout::Inline,
        vec![semio_framework_dsl_record::FieldSpec::new(0, "parent", semio_framework_dsl_record::Shape::Text), semio_framework_dsl_record::FieldSpec::new(1, "slot", semio_framework_dsl_record::Shape::Text), semio_framework_dsl_record::FieldSpec::new(2, "child_id", semio_framework_dsl_record::Shape::Text)],
    )
}

pub fn owner_ref_spec_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::RecordSpec, semio_framework_value::ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(3)?;
        let mut fields = control.allocate_vec(3)?;
        for (id, key) in [(0, "parent"), (1, "slot"), (2, "child_id")] {
            fields.push(semio_framework_dsl_record::producer::field(id, key, semio_framework_dsl_record::Shape::Text, control)?);
            control.step()?;
        }
        semio_framework_dsl_record::producer::record(None, semio_framework_dsl_record::RecordLayout::Inline, fields, control)
    })
}

pub fn owner_ref_spec_producer() -> semio_framework_dsl_record::RecordSpecProducer {
    semio_framework_dsl_record::RecordSpecProducer { ordinary: owner_ref_spec, decoding: |control| owner_ref_spec_controlled(control), encoding: |control| owner_ref_spec_controlled(control) }
}

pub fn owner_ref_to_record(owner: &OwnerRef) -> semio_framework_dsl_record::RecordValue {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    let mut record = semio_framework_dsl_record::RecordValue::default();
    record.fields.insert(0, semio_framework_dsl_record::FieldValue::Text(owner.parent.to_uri()));
    record.fields.insert(1, semio_framework_dsl_record::FieldValue::Text(owner.slot.clone()));
    record.fields.insert(2, semio_framework_dsl_record::FieldValue::Text(owner.child_id.clone()));
    record
}

pub fn owner_ref_from_record(record: &semio_framework_dsl_record::RecordValue) -> Result<OwnerRef, String> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    let parent = match record.get(0) {
        Some(semio_framework_dsl_record::FieldValue::Text(s)) => semio_framework_artifact_reference::ArtifactRef::parse_uri(s)?,
        other => return Err(format!("expected parent, found {other:?}")),
    };
    let slot = match record.get(1) {
        Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
        other => return Err(format!("expected slot, found {other:?}")),
    };
    let child_id = match record.get(2) {
        Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
        other => return Err(format!("expected child_id, found {other:?}")),
    };
    Ok(OwnerRef { parent, slot, child_id })
}

impl semio_framework_dsl_record::DslField for OwnerRef {
    /// 🚫️async: E4 — see `DslField::shape`'s tag on the trait.
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(owner_ref_spec_producer())
    }
    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
        control.checkpoint()?;
        Ok(semio_framework_dsl_record::Shape::Record(owner_ref_spec_producer()))
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Record(owner_ref_to_record(self))
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Record(record) => owner_ref_from_record(record),
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
}
//#endregion 🔖️OwnerRefDslField
