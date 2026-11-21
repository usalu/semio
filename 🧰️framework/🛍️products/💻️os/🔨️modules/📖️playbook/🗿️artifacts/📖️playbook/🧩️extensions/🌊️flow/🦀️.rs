//! 📖️ Actual Playbook document construction from original Flow field loans.
use crate::{PlaybookBlock,PlaybookVectorField,PlaybookSpec,PlaybookStep,PLAYBOOK_DOCUMENT_SCHEMA};
use semio_framework_value::DslValue;
use semio_os_flow::forms::{FlowFormProducer, FlowVariableAnswerSchema};
    fn humanize_widget_label(id: &str) -> String {
        let mut words = Vec::new();
        let mut current = String::new();
        for ch in id.chars() {
            if ch == '_' || ch == '-' || ch == ' ' {
                if !current.is_empty() {
                    words.push(current.clone());
                    current.clear();
                }
                continue;
            }
            if ch.is_uppercase() && !current.is_empty() {
                words.push(current.clone());
                current.clear();
            }
            current.push(if ch.is_uppercase() { ch } else { ch.to_ascii_uppercase() });
        }
        if !current.is_empty() {
            words.push(current);
        }
        if words.is_empty() {
            return id.to_string();
        }
        words.join(" ")
    }

/// 🏭️ Selected child-owned producer of the actual generation form document.
pub struct PlaybookFlowFormProducer;
impl FlowFormProducer for PlaybookFlowFormProducer {
 type Field=PlaybookBlock;
 type Form=PlaybookSpec;
fn slider(&self,id:&str,label:&str,value:f64,min:f64,max:f64,step:f64)->Self::Field {
PlaybookBlock {
                id: id.to_owned(),
                label: label.to_owned(),
                kind: "slider".into(),
                description: None,
                required: None,
                placeholder: None,
                default: Some(DslValue::float(value)),
                min: Some(min),
                max: Some(max),
                step: Some(step),
                unit: None,
                text: None,
                options: None,
                fields: None,
                schema: None,
                src: None,
                accept: None,
                example_id: None,
                params: None,
                condition: None,
            }
}
fn note(&self,id:&str,text:&str)->Self::Field {
PlaybookBlock {
                id: id.to_owned(),
                label: humanize_widget_label(id),
                kind: "note".into(),
                description: None,
                required: None,
                placeholder: None,
                default: None,
                min: None,
                max: None,
                step: None,
                unit: None,
                text: Some(text.to_owned()),
                options: None,
                fields: None,
                schema: None,
                src: None,
                accept: None,
                example_id: None,
                params: None,
                condition: None,
            }
}
fn image(&self,id:&str,src:&str)->Self::Field {
PlaybookBlock {
                id: id.to_owned(),
                label: humanize_widget_label(id),
                kind: "image".into(),
                description: None,
                required: None,
                placeholder: None,
                default: None,
                min: None,
                max: None,
                step: None,
                unit: None,
                text: None,
                options: None,
                fields: None,
                schema: None,
                src: Some(src.to_owned()),
                accept: None,
                example_id: None,
                params: None,
                condition: None,
            }
}
    fn variable(&self, id: &str, name: &str, schema: &str) -> Option<Self::Field> {
        let answer_schema = FlowVariableAnswerSchema::from_schema_id(schema)?;
        let answer = answer_schema.default_answer();
        let (kind, default, fields) = match answer_schema {
            FlowVariableAnswerSchema::Text => ("text", Some(answer), None),
            FlowVariableAnswerSchema::Number => ("number", Some(answer), None),
            FlowVariableAnswerSchema::Boolean => ("boolean", Some(answer), None),
            FlowVariableAnswerSchema::Point | FlowVariableAnswerSchema::Vector => {
                let axes = FlowVariableAnswerSchema::AXES.into_iter().zip(answer.as_array()?);
                let fields = axes.map(|(axis, value)| PlaybookVectorField {
                    key: axis.into(), label: Some(axis.to_ascii_uppercase()), value: value.as_f64(),
                }).collect();
                ("vector", None, Some(fields))
            }
        };
        Some(PlaybookBlock {
            id: id.into(),
            label: humanize_widget_label(name),
            kind: kind.into(),
            description: None,
            required: None,
            placeholder: None,
            default,
            min: None,
            max: None,
            step: None,
            unit: None,
            text: None,
            options: None,
            fields,
            schema: Some(schema.into()),
            src: None,
            accept: None,
            example_id: None,
            params: None,
            condition: None,
        })
    }
 fn finish(&self,blocks:Vec<Self::Field>)->Self::Form {
  PlaybookSpec {schema:PLAYBOOK_DOCUMENT_SCHEMA.into(),id:"flow-generate".into(),version:"1".into(),title:Some("Generate".into()),steps:vec![PlaybookStep {id:"inputs".into(),title:"Inputs".into(),description:None,blocks}]}
 }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
