//! 🔌️ Media port of the workflow graph: the media port record, its spec, record projection, record admission and the field trait impl. See [`super`].

use super::*;

//#region 🔖️WorkflowMediaPort
/// 🔌️ One instance-scoped wire endpoint on a `WorkflowNode` — `id` is unique within the graph
/// (`"{node_id}:{spec.id}:{in|out}"`, see `workflow_media_port`), `spec` is the app-level port
/// declaration it was instantiated from (`semio_framework::MediaPortSpec`).
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct WorkflowMediaPort {
    pub id: String,
    pub spec: MediaPortSpec,
}

// 🚫️async: E4 fn-pointer slot — value goes into `dsl::Shape::Record(fn() -> RecordSpec)`.
fn workflow_media_port_spec_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::RecordSpec,semio_framework_value::ValueError>{
    control.scoped_stage(|control|{control.begin_stage(9)?;let mut fields=control.allocate_vec::<semio_framework_dsl_record::FieldSpec>(9)?;
        let shape=semio_framework_dsl_record::Shape::Text;fields.push(semio_framework_dsl_record::producer::field(0,"id",shape,control)?);control.step()?;
        let shape=semio_framework_dsl_record::Shape::Text;fields.push(semio_framework_dsl_record::producer::field(1,"port_id",shape,control)?);control.step()?;
        let shape=semio_framework_dsl_record::Shape::Text;fields.push(semio_framework_dsl_record::producer::field(2,"label",shape,control)?);control.step()?;
        let shape=workflow_enum_shape_controlled(MEDIA_DIRECTION_LABELS,control)?;fields.push(semio_framework_dsl_record::producer::field(3,"direction",shape,control)?);control.step()?;
        let shape=workflow_enum_shape_controlled(MEDIA_CLASS_LABELS,control)?;fields.push(semio_framework_dsl_record::producer::field(4,"class",shape,control)?);control.step()?;
        let shape=workflow_enum_shape_controlled(MEDIA_FORM_LABELS,control)?;fields.push(semio_framework_dsl_record::producer::field(5,"form",shape,control)?);control.step()?;
        let shape=semio_framework_dsl_record::Shape::Text;fields.push(semio_framework_dsl_record::producer::field(6,"kind_id",shape,control)?.optional());control.step()?;
        let shape=semio_framework_dsl_record::Shape::Bool;fields.push(semio_framework_dsl_record::producer::field(7,"required",shape,control)?);control.step()?;
        let shape=workflow_enum_shape_controlled(MEDIA_MULTIPLICITY_LABELS,control)?;fields.push(semio_framework_dsl_record::producer::field(8,"multiplicity",shape,control)?);control.step()?;
        semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Inline,fields,control)
    })
}

fn workflow_media_port_spec_producer()->semio_framework_dsl_record::RecordSpecProducer{semio_framework_dsl_record::RecordSpecProducer{ordinary:workflow_media_port_spec,decoding:|control|workflow_media_port_spec_controlled(control),encoding:|control|workflow_media_port_spec_controlled(control)}}

fn workflow_media_port_spec() -> semio_framework_dsl_record::RecordSpec {
    semio_framework_dsl_record::RecordSpec::new(
        None,
        semio_framework_dsl_record::RecordLayout::Inline,
        vec![
            semio_framework_dsl_record::FieldSpec::new(0, "id", semio_framework_dsl_record::Shape::Text),
            semio_framework_dsl_record::FieldSpec::new(1, "port_id", semio_framework_dsl_record::Shape::Text),
            semio_framework_dsl_record::FieldSpec::new(2, "label", semio_framework_dsl_record::Shape::Text),
            semio_framework_dsl_record::FieldSpec::new(3, "direction", semio_framework_dsl_record::Shape::Enum(media_port_direction_variants())),
            semio_framework_dsl_record::FieldSpec::new(4, "class", semio_framework_dsl_record::Shape::Enum(media_class_variants())),
            semio_framework_dsl_record::FieldSpec::new(5, "form", semio_framework_dsl_record::Shape::Enum(media_form_variants())),
            semio_framework_dsl_record::FieldSpec::new(6, "kind_id", semio_framework_dsl_record::Shape::Text).optional(),
            semio_framework_dsl_record::FieldSpec::new(7, "required", semio_framework_dsl_record::Shape::Bool),
            semio_framework_dsl_record::FieldSpec::new(8, "multiplicity", semio_framework_dsl_record::Shape::Enum(port_multiplicity_variants())),
        ],
    )
}

pub fn workflow_media_port_to_record(port: &WorkflowMediaPort) -> semio_framework_dsl_record::RecordValue {
    let mut record = semio_framework_dsl_record::RecordValue::default();
    record.fields.insert(0, semio_framework_dsl_record::FieldValue::Text(port.id.clone()));
    record.fields.insert(1, semio_framework_dsl_record::FieldValue::Text(port.spec.id.clone()));
    record.fields.insert(2, semio_framework_dsl_record::FieldValue::Text(port.spec.label.clone()));
    record.fields.insert(3, semio_framework_dsl_record::FieldValue::Enum(media_port_direction_ordinal(port.spec.direction)));
    record.fields.insert(4, semio_framework_dsl_record::FieldValue::Enum(media_class_ordinal(port.spec.media_type.class)));
    record.fields.insert(5, semio_framework_dsl_record::FieldValue::Enum(media_form_ordinal(port.spec.media_type.form)));
    match &port.spec.kind_id {
        Some(kind_id) => record.fields.insert(6, semio_framework_dsl_record::FieldValue::Text(kind_id.clone())),
        None => record.fields.insert(6, semio_framework_dsl_record::FieldValue::Absent),
    };
    record.fields.insert(7, semio_framework_dsl_record::FieldValue::Bool(port.spec.required));
    record.fields.insert(8, semio_framework_dsl_record::FieldValue::Enum(port_multiplicity_ordinal(port.spec.multiplicity)));
    record
}

pub fn workflow_media_port_from_record(record: &semio_framework_dsl_record::RecordValue) -> Result<WorkflowMediaPort, semio_framework_diagnostic::TextError> {
    let id = match record.get(0) {
        Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
        other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected id, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
    };
    let port_id = match record.get(1) {
        Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
        other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected port_id, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
    };
    let label = match record.get(2) {
        Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
        other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected label, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
    };
    let direction = match record.get(3) {
        Some(semio_framework_dsl_record::FieldValue::Enum(ordinal)) => media_port_direction_from_ordinal(*ordinal).map_err(|message| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))?,
        other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected direction, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
    };
    let class = match record.get(4) {
        Some(semio_framework_dsl_record::FieldValue::Enum(ordinal)) => media_class_from_ordinal(*ordinal).map_err(|message| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))?,
        other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected class, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
    };
    let form = match record.get(5) {
        Some(semio_framework_dsl_record::FieldValue::Enum(ordinal)) => media_form_from_ordinal(*ordinal).map_err(|message| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))?,
        other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected form, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
    };
    let kind_id = match record.get(6) {
        Some(semio_framework_dsl_record::FieldValue::Text(s)) => Some(s.clone()),
        _ => None,
    };
    let required = match record.get(7) {
        Some(semio_framework_dsl_record::FieldValue::Bool(b)) => *b,
        other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected required, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
    };
    let multiplicity = match record.get(8) {
        Some(semio_framework_dsl_record::FieldValue::Enum(ordinal)) => port_multiplicity_from_ordinal(*ordinal).map_err(|message| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))?,
        other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected multiplicity, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
    };
    Ok(WorkflowMediaPort { id, spec: MediaPortSpec { id: port_id, label, direction, media_type: MediaType { class, form }, kind_id, required, multiplicity } })
}

impl semio_framework_dsl_record::DslField for WorkflowMediaPort {
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,semio_framework_value::ValueError>{workflow_media_port_to_record_controlled(self,control).map(semio_framework_dsl_record::FieldValue::Record)}
    fn to_record_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,semio_framework_value::ValueError>{workflow_media_port_to_record_controlled(self,control)}
    fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{control.checkpoint()?;match value{semio_framework_dsl_record::FieldValue::Record(record)=>workflow_media_port_from_record_controlled(record,control),_=>Err(workflow_invalid("expected declared record"))}}
    fn from_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{workflow_media_port_from_record_controlled(record,control)}
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(workflow_media_port_spec_producer())
    }
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{control.checkpoint()?;Ok(semio_framework_dsl_record::Shape::Record(workflow_media_port_spec_producer()))}
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Record(workflow_media_port_to_record(self))
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Record(record) => workflow_media_port_from_record(record).map_err(|e| e.message),
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
}
//#endregion 🔖️WorkflowMediaPort
