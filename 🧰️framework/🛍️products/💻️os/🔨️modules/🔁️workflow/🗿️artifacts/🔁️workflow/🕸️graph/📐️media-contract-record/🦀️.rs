//! 📐️ Media contract record codec of the workflow graph: the controlled spec, record projection, record admission and the field trait impls of the media contract. See [`super`].

use super::*;

//#region 🔖️MediaContractDsl
fn media_contract_spec_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::RecordSpec,semio_framework_value::ValueError>{
    control.scoped_stage(|control|{control.begin_stage(8)?;let mut fields=control.allocate_vec::<semio_framework_dsl_record::FieldSpec>(8)?;
        let shape=semio_framework_dsl_record::Shape::Text;fields.push(semio_framework_dsl_record::producer::field(0,"kind_id",shape,control)?);control.step()?;
        let shape=workflow_enum_shape_controlled(MEDIA_CLASS_LABELS,control)?;fields.push(semio_framework_dsl_record::producer::field(1,"class",shape,control)?);control.step()?;
        let shape=workflow_enum_shape_controlled(MEDIA_FORM_LABELS,control)?;fields.push(semio_framework_dsl_record::producer::field(2,"form",shape,control)?);control.step()?;
        let shape=semio_framework_dsl_record::Shape::Text;fields.push(semio_framework_dsl_record::producer::field(3,"wire_kind",shape,control)?);control.step()?;
        let shape=semio_framework_dsl_record::Shape::Text;fields.push(semio_framework_dsl_record::producer::field(4,"wire_format",shape,control)?.optional());control.step()?;
        let shape=semio_framework_dsl_record::Shape::Text;fields.push(semio_framework_dsl_record::producer::field(5,"wire_schema",shape,control)?.optional());control.step()?;
        let shape=workflow_enum_shape_controlled(MEDIA_FORM_LABELS,control)?;fields.push(semio_framework_dsl_record::producer::field(6,"conversion_from",shape,control)?.optional());control.step()?;
        let shape=workflow_enum_shape_controlled(MEDIA_FORM_LABELS,control)?;fields.push(semio_framework_dsl_record::producer::field(7,"conversion_to",shape,control)?.optional());control.step()?;
        semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Inline,fields,control)
    })
}

fn media_contract_spec_producer()->semio_framework_dsl_record::RecordSpecProducer{semio_framework_dsl_record::RecordSpecProducer{ordinary:media_contract_spec,decoding:|control|media_contract_spec_controlled(control),encoding:|control|media_contract_spec_controlled(control)}}

fn media_contract_spec() -> semio_framework_dsl_record::RecordSpec {
    semio_framework_dsl_record::RecordSpec::new(
        None,
        semio_framework_dsl_record::RecordLayout::Inline,
        vec![
            semio_framework_dsl_record::FieldSpec::new(0, "kind_id", semio_framework_dsl_record::Shape::Text),
            semio_framework_dsl_record::FieldSpec::new(1, "class", semio_framework_dsl_record::Shape::Enum(media_class_variants())),
            semio_framework_dsl_record::FieldSpec::new(2, "form", semio_framework_dsl_record::Shape::Enum(media_form_variants())),
            semio_framework_dsl_record::FieldSpec::new(3, "wire_kind", semio_framework_dsl_record::Shape::Text),
            semio_framework_dsl_record::FieldSpec::new(4, "wire_format", semio_framework_dsl_record::Shape::Text).optional(),
            semio_framework_dsl_record::FieldSpec::new(5, "wire_schema", semio_framework_dsl_record::Shape::Text).optional(),
            semio_framework_dsl_record::FieldSpec::new(6, "conversion_from", semio_framework_dsl_record::Shape::Enum(media_form_variants())).optional(),
            semio_framework_dsl_record::FieldSpec::new(7, "conversion_to", semio_framework_dsl_record::Shape::Enum(media_form_variants())).optional(),
        ],
    )
}

pub fn media_contract_to_record(contract: &MediaContract) -> semio_framework_dsl_record::RecordValue {
    let mut record = semio_framework_dsl_record::RecordValue::default();
    record.fields.insert(0, semio_framework_dsl_record::FieldValue::Text(contract.kind_id.clone()));
    record.fields.insert(1, semio_framework_dsl_record::FieldValue::Enum(media_class_ordinal(contract.media_type.class)));
    record.fields.insert(2, semio_framework_dsl_record::FieldValue::Enum(media_form_ordinal(contract.media_type.form)));
    match &contract.wire {
        MediaWireFormat::Binary { format_kind } => {
            record.fields.insert(3, semio_framework_dsl_record::FieldValue::Text("binary".to_string()));
            record.fields.insert(4, semio_framework_dsl_record::FieldValue::Text(format_kind.clone()));
            record.fields.insert(5, semio_framework_dsl_record::FieldValue::Absent);
        }
        MediaWireFormat::Document { schema } | MediaWireFormat::Intrinsic { schema } => {
            let kind=if matches!(&contract.wire,MediaWireFormat::Intrinsic{..}){"intrinsic"}else{"document"};
            record.fields.insert(3, semio_framework_dsl_record::FieldValue::Text(kind.to_string()));
            record.fields.insert(4, semio_framework_dsl_record::FieldValue::Absent);
            record.fields.insert(5, semio_framework_dsl_record::FieldValue::Text(schema.clone()));
        }
    }
    match contract.conversion {
        Some((from, to)) => {
            record.fields.insert(6, semio_framework_dsl_record::FieldValue::Enum(media_form_ordinal(from)));
            record.fields.insert(7, semio_framework_dsl_record::FieldValue::Enum(media_form_ordinal(to)));
        }
        None => {
            record.fields.insert(6, semio_framework_dsl_record::FieldValue::Absent);
            record.fields.insert(7, semio_framework_dsl_record::FieldValue::Absent);
        }
    }
    record
}

pub fn media_contract_from_record(record: &semio_framework_dsl_record::RecordValue) -> Result<MediaContract, semio_framework_diagnostic::TextError> {
    let kind_id = match record.get(0) {
        Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
        other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected kind_id, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
    };
    let class = match record.get(1) {
        Some(semio_framework_dsl_record::FieldValue::Enum(ordinal)) => media_class_from_ordinal(*ordinal).map_err(|message| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))?,
        other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected class, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
    };
    let form = match record.get(2) {
        Some(semio_framework_dsl_record::FieldValue::Enum(ordinal)) => media_form_from_ordinal(*ordinal).map_err(|message| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))?,
        other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected form, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
    };
    let wire_kind = match record.get(3) {
        Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
        other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected wire_kind, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
    };
    let wire = match wire_kind.as_str() {
        "binary" => {
            let format_word = match record.get(4) {
                Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
                other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected wire_format, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
            };
            MediaWireFormat::Binary { format_kind: format_word }
        }
        "document" | "intrinsic" => {
            let schema = match record.get(5) {
                Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
                other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected wire_schema, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
            };
            if wire_kind=="intrinsic"{MediaWireFormat::Intrinsic{schema}}else{MediaWireFormat::Document { schema }}
        }
        other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown wire kind '{other}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
    };
    let conversion = match (record.get(6), record.get(7)) {
        (Some(semio_framework_dsl_record::FieldValue::Enum(from)), Some(semio_framework_dsl_record::FieldValue::Enum(to))) => Some((media_form_from_ordinal(*from).map_err(|message| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))?, media_form_from_ordinal(*to).map_err(|message| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))?)),
        _ => None,
    };
    Ok(MediaContract { kind_id, media_type: MediaType { class, form }, wire, conversion })
}

impl semio_framework_dsl_record::DslField for MediaContract {
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,semio_framework_value::ValueError>{media_contract_to_record_controlled(self,control).map(semio_framework_dsl_record::FieldValue::Record)}
    fn to_record_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,semio_framework_value::ValueError>{media_contract_to_record_controlled(self,control)}
    fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{control.checkpoint()?;match value{semio_framework_dsl_record::FieldValue::Record(record)=>media_contract_from_record_controlled(record,control),_=>Err(workflow_invalid("expected declared record"))}}
    fn from_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{media_contract_from_record_controlled(record,control)}
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(media_contract_spec_producer())
    }
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{control.checkpoint()?;Ok(semio_framework_dsl_record::Shape::Record(media_contract_spec_producer()))}
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Record(media_contract_to_record(self))
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Record(record) => media_contract_from_record(record).map_err(|e| e.message),
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
}

/// 🔀️ Hand-written, not derived: `conversion: Option<(MediaForm, MediaForm)>` is a raw Rust tuple,
/// which has no blanket `ToValue`/`FromValue` (same reasoning as this type's hand-crafted
/// `dsl::DslField` above) — bridged directly as a two-element `DslValue::Array`. `kind_id`/
/// `media_type`/`wire` reuse their own `ToValue`/`FromValue` impls (`MediaType`/`MediaWireFormat`
/// gained them in `🛂️manifest/🦀️.rs`).
impl semio_framework_value::ToValue for MediaContract {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object([
            ("kindId".to_string(), semio_framework_value::ToValue::to_value(&self.kind_id)),
            ("mediaType".to_string(), semio_framework_value::ToValue::to_value(&self.media_type)),
            ("wire".to_string(), semio_framework_value::ToValue::to_value(&self.wire)),
            (
                "conversion".to_string(),
                match &self.conversion {
                    Some((from, to)) => semio_framework_value::DslValue::Array(vec![semio_framework_value::ToValue::to_value(from), semio_framework_value::ToValue::to_value(to)]),
                    None => semio_framework_value::DslValue::Null,
                },
            ),
        ])
    }
}

impl semio_framework_value::FromValue for MediaContract {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for MediaContract, found {value:?}")));
        };
        let mut kind_id = None;
        let mut media_type = None;
        let mut wire = None;
        let mut conversion = None;
        for (key, entry) in fields {
            match key.as_str() {
                "kindId" => kind_id = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("kindId"))?),
                "mediaType" => media_type = Some(<MediaType as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("mediaType"))?),
                "wire" => wire = Some(<MediaWireFormat as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("wire"))?),
                "conversion" => {
                    conversion = Some(match entry {
                        semio_framework_value::DslValue::Null => None,
                        semio_framework_value::DslValue::Array(items) => {
                            let mut iter = items.into_iter();
                            let from = iter.next().ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MediaContract.conversion missing from"))?;
                            let to = iter.next().ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MediaContract.conversion missing to"))?;
                            Some((
                                <MediaForm as semio_framework_value::FromValue>::from_value(from).map_err(|e| e.under("conversion.0"))?,
                                <MediaForm as semio_framework_value::FromValue>::from_value(to).map_err(|e| e.under("conversion.1"))?,
                            ))
                        }
                        other => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected array or null for MediaContract.conversion, found {other:?}"))),
                    });
                }
                _ => {}
            }
        }
        Ok(MediaContract {
            kind_id: kind_id.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MediaContract missing kindId"))?,
            media_type: media_type.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MediaContract missing mediaType"))?,
            wire: wire.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MediaContract missing wire"))?,
            conversion: conversion.unwrap_or(None),
        })
    }
}
//#endregion 🔖️MediaContractDsl
