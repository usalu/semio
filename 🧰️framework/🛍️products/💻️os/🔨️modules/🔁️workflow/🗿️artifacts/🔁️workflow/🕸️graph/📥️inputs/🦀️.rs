//! 📥️ Inputs of the workflow graph: the declared collection-level input slot with its spec and field trait impl. See [`super`].

use super::*;

/// 🔌️ One declared collection-level input slot a workflow's nodes can bind an in-port to — `selector`
/// is a glob matched against collection entry paths at run time (W5's `SpaceRunner` job); this crate
/// only carries the declaration + validates bindings resolve (`validate_workflow_snapshot`).
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct WorkflowInput {
    pub id: String,
    pub kind_id: String,
    pub selector: String,
    pub required: bool,
    pub multiplicity: PortMultiplicity,
}

// 🚫️async: E4 fn-pointer slot — value goes into `dsl::Shape::Record(fn() -> RecordSpec)`.
fn workflow_input_spec_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::RecordSpec,semio_framework_value::ValueError>{
    control.scoped_stage(|control|{control.begin_stage(5)?;let mut fields=control.allocate_vec::<semio_framework_dsl_record::FieldSpec>(5)?;
        let shape=semio_framework_dsl_record::Shape::Text;fields.push(semio_framework_dsl_record::producer::field(0,"id",shape,control)?);control.step()?;
        let shape=semio_framework_dsl_record::Shape::Text;fields.push(semio_framework_dsl_record::producer::field(1,"kind_id",shape,control)?);control.step()?;
        let shape=semio_framework_dsl_record::Shape::Text;fields.push(semio_framework_dsl_record::producer::field(2,"selector",shape,control)?);control.step()?;
        let shape=semio_framework_dsl_record::Shape::Bool;fields.push(semio_framework_dsl_record::producer::field(3,"required",shape,control)?);control.step()?;
        let shape=workflow_enum_shape_controlled(MEDIA_MULTIPLICITY_LABELS,control)?;fields.push(semio_framework_dsl_record::producer::field(4,"multiplicity",shape,control)?);control.step()?;
        semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Inline,fields,control)
    })
}

fn workflow_input_spec_producer()->semio_framework_dsl_record::RecordSpecProducer{semio_framework_dsl_record::RecordSpecProducer{ordinary:workflow_input_spec,decoding:|control|workflow_input_spec_controlled(control),encoding:|control|workflow_input_spec_controlled(control)}}

fn workflow_input_spec() -> semio_framework_dsl_record::RecordSpec {
    semio_framework_dsl_record::RecordSpec::new(
        None,
        semio_framework_dsl_record::RecordLayout::Inline,
        vec![
            semio_framework_dsl_record::FieldSpec::new(0, "id", semio_framework_dsl_record::Shape::Text),
            semio_framework_dsl_record::FieldSpec::new(1, "kind_id", semio_framework_dsl_record::Shape::Text),
            semio_framework_dsl_record::FieldSpec::new(2, "selector", semio_framework_dsl_record::Shape::Text),
            semio_framework_dsl_record::FieldSpec::new(3, "required", semio_framework_dsl_record::Shape::Bool),
            semio_framework_dsl_record::FieldSpec::new(4, "multiplicity", semio_framework_dsl_record::Shape::Enum(port_multiplicity_variants())),
        ],
    )
}

/// 🧬️ Hand-crafted `dsl::DslField` (not `#[derive(dsl::DslRecord)]`) — `multiplicity: PortMultiplicity`
/// is a foreign type this crate can't derive `DslField` for under the orphan rule, same reasoning as
/// `WorkflowMediaPort`/`MediaContract` above; reuses their `port_multiplicity_ordinal`/`_from_ordinal`/
/// `_variants` helpers directly.
impl semio_framework_dsl_record::DslField for WorkflowInput {
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,semio_framework_value::ValueError>{workflow_input_to_record_controlled(self,control).map(semio_framework_dsl_record::FieldValue::Record)}
    fn to_record_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,semio_framework_value::ValueError>{workflow_input_to_record_controlled(self,control)}
    fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{control.checkpoint()?;match value{semio_framework_dsl_record::FieldValue::Record(record)=>workflow_input_from_record_controlled(record,control),_=>Err(workflow_invalid("expected declared record"))}}
    fn from_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{workflow_input_from_record_controlled(record,control)}
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(workflow_input_spec_producer())
    }
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{control.checkpoint()?;Ok(semio_framework_dsl_record::Shape::Record(workflow_input_spec_producer()))}
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        let mut record = semio_framework_dsl_record::RecordValue::default();
        record.fields.insert(0, semio_framework_dsl_record::FieldValue::Text(self.id.clone()));
        record.fields.insert(1, semio_framework_dsl_record::FieldValue::Text(self.kind_id.clone()));
        record.fields.insert(2, semio_framework_dsl_record::FieldValue::Text(self.selector.clone()));
        record.fields.insert(3, semio_framework_dsl_record::FieldValue::Bool(self.required));
        record.fields.insert(4, semio_framework_dsl_record::FieldValue::Enum(port_multiplicity_ordinal(self.multiplicity)));
        semio_framework_dsl_record::FieldValue::Record(record)
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        let semio_framework_dsl_record::FieldValue::Record(record) = value else { return Err(format!("expected Record, found {value:?}")) };
        let id = match record.get(0) {
            Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
            other => return Err(format!("expected id, found {other:?}")),
        };
        let kind_id = match record.get(1) {
            Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
            other => return Err(format!("expected kind_id, found {other:?}")),
        };
        let selector = match record.get(2) {
            Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
            other => return Err(format!("expected selector, found {other:?}")),
        };
        let required = match record.get(3) {
            Some(semio_framework_dsl_record::FieldValue::Bool(b)) => *b,
            other => return Err(format!("expected required, found {other:?}")),
        };
        let multiplicity = match record.get(4) {
            Some(semio_framework_dsl_record::FieldValue::Enum(ordinal)) => port_multiplicity_from_ordinal(*ordinal)?,
            other => return Err(format!("expected multiplicity, found {other:?}")),
        };
        Ok(WorkflowInput { id, kind_id, selector, required, multiplicity })
    }
}
