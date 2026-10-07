//! 📝️ Explicit complete owned snapshot record fields and canonical tagged semantic values.
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use semio_framework_value::{ValueError, ValueRefusalKind};
use pack::value::{ToValue,FromValue,DslValue};
pub const COMPONENT_GRAMMAR_SEMIO:&str=include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH:&str="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/📖️.grammar.semio";

const FIELDS:[(u16,&str,semio_framework_dsl_record::Shape,bool);31]=[
    (1,"schema",semio_framework_dsl_record::Shape::Text,false),
    (2,"declaredVersion",semio_framework_dsl_record::Shape::Text,false),
    (3,"pages",semio_framework_dsl_record::Shape::Value,false),
    (4,"fonts",semio_framework_dsl_record::Shape::Value,false),
    (5,"images",semio_framework_dsl_record::Shape::Value,false),
    (6,"forms",semio_framework_dsl_record::Shape::Value,false),
    (7,"extGStates",semio_framework_dsl_record::Shape::Value,false),
    (8,"shadings",semio_framework_dsl_record::Shape::Value,false),
    (9,"patterns",semio_framework_dsl_record::Shape::Value,false),
    (10,"colorSpaces",semio_framework_dsl_record::Shape::Value,false),
    (11,"properties",semio_framework_dsl_record::Shape::Value,false),
    (12,"outlines",semio_framework_dsl_record::Shape::Value,false),
    (13,"namedDestinations",semio_framework_dsl_record::Shape::Value,false),
    (14,"pageLabels",semio_framework_dsl_record::Shape::Value,false),
    (15,"embeddedFiles",semio_framework_dsl_record::Shape::Value,false),
    (16,"outputIntents",semio_framework_dsl_record::Shape::Value,false),
    (17,"acroForm",semio_framework_dsl_record::Shape::Value,true),
    (18,"optionalContent",semio_framework_dsl_record::Shape::Value,true),
    (19,"pageLayout",semio_framework_dsl_record::Shape::Value,true),
    (20,"pageMode",semio_framework_dsl_record::Shape::Value,true),
    (21,"viewerPreferences",semio_framework_dsl_record::Shape::Value,true),
    (22,"openAction",semio_framework_dsl_record::Shape::Value,true),
    (23,"language",semio_framework_dsl_record::Shape::Value,true),
    (24,"markInfo",semio_framework_dsl_record::Shape::Value,true),
    (25,"metadata",semio_framework_dsl_record::Shape::Value,true),
    (26,"documentId",semio_framework_dsl_record::Shape::Value,true),
    (27,"encryption",semio_framework_dsl_record::Shape::Value,true),
    (28,"info",semio_framework_dsl_record::Shape::Value,false),
    (29,"catalogExtra",semio_framework_dsl_record::Shape::Value,false),
    (30,"objects",semio_framework_dsl_record::Shape::Value,false),
    (31,"trailer",semio_framework_dsl_record::Shape::Value,false),
];
pub(crate) fn spec()->semio_framework_dsl_record::RecordSpec{
    semio_framework_dsl_record::RecordSpec::new(None,semio_framework_dsl_record::RecordLayout::Lines,FIELDS.into_iter().map(|(id,key,shape,optional)|{let mut field=semio_framework_dsl_record::FieldSpec::new(id,key,shape);field.optional=optional;field}).collect())
}
fn spec_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::RecordSpec,ValueError>{
    control.scoped_stage(|control|{
        control.begin_stage(FIELDS.len())?;let mut fields=control.allocate_vec::<semio_framework_dsl_record::FieldSpec>(FIELDS.len())?;
        for(id,key,shape,optional)in FIELDS{control.checkpoint()?;let mut field=semio_framework_dsl_record::producer::field(id,key,shape,control)?;field.optional=optional;fields.push(field);control.step()?;}
        semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Lines,fields,control)
    })
}
/// 🏭️ Owns literal snapshot metadata independently under either native allocation controller.
pub(crate) fn spec_producer()->semio_framework_dsl_record::RecordSpecProducer{semio_framework_dsl_record::RecordSpecProducer{ordinary:spec,decoding:|control|spec_controlled(control),encoding:|control|spec_controlled(control)}}

pub(crate) fn to_record(value:&PdfSnapshot)->semio_framework_dsl_record::RecordValue{
    use semio_framework_dsl_record::FieldValue as V;
    semio_framework_dsl_record::RecordValue{fields:[
        (1,V::Text(value.schema.clone())),(2,V::Text(value.declared_version.clone())),
        (3,V::Value(value.pages.to_value())),(4,V::Value(value.fonts.to_value())),(5,V::Value(value.images.to_value())),(6,V::Value(value.forms.to_value())),(7,V::Value(value.ext_g_states.to_value())),(8,V::Value(value.shadings.to_value())),(9,V::Value(value.patterns.to_value())),(10,V::Value(value.color_spaces.to_value())),(11,V::Value(value.properties.to_value())),(12,V::Value(value.outlines.to_value())),(13,V::Value(value.named_destinations.to_value())),(14,V::Value(value.page_labels.to_value())),(15,V::Value(value.embedded_files.to_value())),(16,V::Value(value.output_intents.to_value())),
        (17,V::Value(value.acro_form.to_value())),(18,V::Value(value.optional_content.to_value())),(19,V::Value(value.page_layout.to_value())),(20,V::Value(value.page_mode.to_value())),(21,V::Value(value.viewer_preferences.to_value())),(22,V::Value(value.open_action.to_value())),(23,V::Value(value.language.to_value())),(24,V::Value(value.mark_info.to_value())),(25,V::Value(value.metadata.to_value())),(26,V::Value(crate::standards::v1_7::subsets::base::schema::snapshot::octets::document_id_to_value(&value.document_id))),(27,V::Value(value.encryption.to_value())),
        (28,V::Value(value.info.to_value())),(29,V::Value(value.catalog_extra.to_value())),(30,V::Value(value.objects.to_value())),(31,V::Value(value.trailer.to_value())),
    ].into_iter().collect()}
}
/// 🛫️ Projects each literal root slot with bounded allocation and partial-record retirement.
pub(crate) fn to_record_controlled(value:&PdfSnapshot,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,ValueError>{
    control.scoped_depth(64,|control|control.scoped_stage(|control|->Result<semio_framework_dsl_record::RecordValue,ValueError>{
        control.begin_stage(FIELDS.len())?;
        let mut record=semio_framework_dsl_record::native_encoding::EncodedRecord::new(FIELDS.len(),control)?;
        record.insert(1,semio_framework_dsl_record::FieldValue::Text(control.copy_text(&value.schema)?))?;control.step()?;
        record.insert(2,semio_framework_dsl_record::FieldValue::Text(control.copy_text(&value.declared_version)?))?;control.step()?;
        macro_rules! field{($id:literal,$key:literal,$output:expr)=>{{let output=$output.map_err(|failure|failure.under($key))?;record.insert($id,semio_framework_dsl_record::FieldValue::Value(output))?;control.step()?;}};}
        field!(3,"pages",value.pages.to_value_controlled(control));
        field!(4,"fonts",value.fonts.to_value_controlled(control));
        field!(5,"images",value.images.to_value_controlled(control));
        field!(6,"forms",value.forms.to_value_controlled(control));
        field!(7,"extGStates",value.ext_g_states.to_value_controlled(control));
        field!(8,"shadings",value.shadings.to_value_controlled(control));
        field!(9,"patterns",value.patterns.to_value_controlled(control));
        field!(10,"colorSpaces",value.color_spaces.to_value_controlled(control));
        field!(11,"properties",value.properties.to_value_controlled(control));
        field!(12,"outlines",value.outlines.to_value_controlled(control));
        field!(13,"namedDestinations",value.named_destinations.to_value_controlled(control));
        field!(14,"pageLabels",value.page_labels.to_value_controlled(control));
        field!(15,"embeddedFiles",value.embedded_files.to_value_controlled(control));
        field!(16,"outputIntents",value.output_intents.to_value_controlled(control));
        field!(17,"acroForm",value.acro_form.to_value_controlled(control));
        field!(18,"optionalContent",value.optional_content.to_value_controlled(control));
        field!(19,"pageLayout",value.page_layout.to_value_controlled(control));
        field!(20,"pageMode",value.page_mode.to_value_controlled(control));
        field!(21,"viewerPreferences",value.viewer_preferences.to_value_controlled(control));
        field!(22,"openAction",value.open_action.to_value_controlled(control));
        field!(23,"language",value.language.to_value_controlled(control));
        field!(24,"markInfo",value.mark_info.to_value_controlled(control));
        field!(25,"metadata",value.metadata.to_value_controlled(control));
        field!(26,"documentId",octets::document_id_to_value_controlled(&value.document_id,control));
        field!(27,"encryption",value.encryption.to_value_controlled(control));
        field!(28,"info",value.info.to_value_controlled(control));
        field!(29,"catalogExtra",value.catalog_extra.to_value_controlled(control));
        field!(30,"objects",value.objects.to_value_controlled(control));
        field!(31,"trailer",value.trailer.to_value_controlled(control));
        Ok(record.take())
    }))
}
fn field(record:&semio_framework_dsl_record::RecordValue,id:u16,key:&str)->Result<DslValue,semio_framework_diagnostic::TextError>{match record.get(id){Some(semio_framework_dsl_record::FieldValue::Text(value))if id<=2=>Ok(DslValue::String(value.clone())),Some(semio_framework_dsl_record::FieldValue::Value(value))if id>2=>Ok(value.clone()),None|Some(semio_framework_dsl_record::FieldValue::Absent)if(17..=27).contains(&id)=>Ok(DslValue::Null),_=>Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("PDF snapshot field {key} is missing or has a different shape"),semio_framework_diagnostic::TextSpan::at(1,1)))}}
pub(crate) fn from_record(record:&semio_framework_dsl_record::RecordValue)->Result<PdfSnapshot,semio_framework_diagnostic::TextError>{
    if record.fields.keys().any(|id|!(1..=31).contains(id)){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PDF snapshot contains an undeclared root field",semio_framework_diagnostic::TextSpan::at(1,1)));}
    PdfSnapshot::from_value(DslValue::object([
        ("schema".into(),field(record,1,"schema")?),("declaredVersion".into(),field(record,2,"declaredVersion")?),
        ("pages".into(),field(record,3,"pages")?),("fonts".into(),field(record,4,"fonts")?),("images".into(),field(record,5,"images")?),("forms".into(),field(record,6,"forms")?),("extGStates".into(),field(record,7,"extGStates")?),("shadings".into(),field(record,8,"shadings")?),("patterns".into(),field(record,9,"patterns")?),("colorSpaces".into(),field(record,10,"colorSpaces")?),("properties".into(),field(record,11,"properties")?),("outlines".into(),field(record,12,"outlines")?),("namedDestinations".into(),field(record,13,"namedDestinations")?),("pageLabels".into(),field(record,14,"pageLabels")?),("embeddedFiles".into(),field(record,15,"embeddedFiles")?),("outputIntents".into(),field(record,16,"outputIntents")?),
        ("acroForm".into(),field(record,17,"acroForm")?),("optionalContent".into(),field(record,18,"optionalContent")?),("pageLayout".into(),field(record,19,"pageLayout")?),("pageMode".into(),field(record,20,"pageMode")?),("viewerPreferences".into(),field(record,21,"viewerPreferences")?),("openAction".into(),field(record,22,"openAction")?),("language".into(),field(record,23,"language")?),("markInfo".into(),field(record,24,"markInfo")?),("metadata".into(),field(record,25,"metadata")?),("documentId".into(),field(record,26,"documentId")?),("encryption".into(),field(record,27,"encryption")?),
        ("info".into(),field(record,28,"info")?),("catalogExtra".into(),field(record,29,"catalogExtra")?),("objects".into(),field(record,30,"objects")?),("trailer".into(),field(record,31,"trailer")?),
    ])).map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1,1)))
}

fn borrowed_field<'a>(record:&'a semio_framework_dsl_record::RecordValue,id:u16,key:&str)->Result<&'a DslValue,ValueError>{match record.get(id){Some(semio_framework_dsl_record::FieldValue::Value(value))=>Ok(value),None|Some(semio_framework_dsl_record::FieldValue::Absent)if(17..=27).contains(&id)=>Ok(&DslValue::Null),_=>Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("PDF snapshot field {key} is missing or has a different shape")))}}
fn owned<T:FromValue>(record:&semio_framework_dsl_record::RecordValue,id:u16,key:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<pack::value::DecodedValue<T>,ValueError>{let value=T::from_value_controlled(borrowed_field(record,id,key)?,control).map_err(|error|error.under(key))?;let owner=pack::value::DecodedValue::new(value,T::retire_decoded);control.step()?;Ok(owner)}
pub(crate) fn from_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<PdfSnapshot,ValueError>{
    control.scoped_stage(|control|->Result<_,ValueError>{
        control.begin_stage(31)?;if record.fields.keys().any(|id|!(1..=31).contains(id)){return Err(semio_framework_value::ValueError::new(ValueRefusalKind::InvalidValue, "PDF snapshot contains an undeclared root field"));}
        let text=|id,key:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>|{let Some(semio_framework_dsl_record::FieldValue::Text(value))=record.get(id)else{return Err(semio_framework_value::ValueError::new(ValueRefusalKind::InvalidValue, format!("PDF snapshot field {key} is missing or has a different shape")))};let value=control.copy_text(value)?;control.step()?;Ok::<_,ValueError>(value)};
        let schema=text(1,"schema",control)?;let declared_version=text(2,"declaredVersion",control)?;
        let pages=owned::<Vec<PdfPage>>(record,3,"pages",control)?;
        let fonts=owned::<Vec<PdfFont>>(record,4,"fonts",control)?;
        let images=owned::<Vec<PdfImage>>(record,5,"images",control)?;
        let forms=owned::<Vec<PdfFormXObject>>(record,6,"forms",control)?;
        let ext_g_states=owned::<Vec<PdfExtGState>>(record,7,"extGStates",control)?;
        let shadings=owned::<Vec<PdfShading>>(record,8,"shadings",control)?;
        let patterns=owned::<Vec<PdfPattern>>(record,9,"patterns",control)?;
        let color_spaces=owned::<Vec<PdfNamedColorSpace>>(record,10,"colorSpaces",control)?;
        let properties=owned::<Vec<PdfNamedProperties>>(record,11,"properties",control)?;
        let outlines=owned::<Vec<PdfOutlineItem>>(record,12,"outlines",control)?;
        let named_destinations=owned::<Vec<PdfNamedDestination>>(record,13,"namedDestinations",control)?;
        let page_labels=owned::<Vec<PdfPageLabelRange>>(record,14,"pageLabels",control)?;
        let embedded_files=owned::<Vec<PdfEmbeddedFile>>(record,15,"embeddedFiles",control)?;
        let output_intents=owned::<Vec<PdfOutputIntent>>(record,16,"outputIntents",control)?;
        let acro_form=owned::<Option<PdfAcroForm>>(record,17,"acroForm",control)?;
        let optional_content=owned::<Option<PdfOptionalContent>>(record,18,"optionalContent",control)?;
        let page_layout=owned::<Option<PdfPageLayout>>(record,19,"pageLayout",control)?;
        let page_mode=owned::<Option<PdfPageMode>>(record,20,"pageMode",control)?;
        let viewer_preferences=owned::<Option<PdfViewerPreferences>>(record,21,"viewerPreferences",control)?;
        let open_action=owned::<Option<PdfOpenAction>>(record,22,"openAction",control)?;
        let language=owned::<Option<String>>(record,23,"language",control)?;
        let mark_info=owned::<Option<PdfMarkInfo>>(record,24,"markInfo",control)?;
        let metadata=owned::<Option<String>>(record,25,"metadata",control)?;
        let document_id=octets::document_id_from_value_controlled(borrowed_field(record,26,"documentId")?,control).map_err(|value|value.under("documentId"))?;control.step()?;
        let encryption=owned::<Option<PdfEncryption>>(record,27,"encryption",control)?;
        let info=owned::<PdfInfo>(record,28,"info",control)?;
        let catalog_extra=owned::<Vec<PdfDictEntry>>(record,29,"catalogExtra",control)?;
        let objects=owned::<Vec<PdfIndirectObject>>(record,30,"objects",control)?;
        let trailer=owned::<Vec<PdfDictEntry>>(record,31,"trailer",control)?;
        Ok(PdfSnapshot{schema,declared_version,pages:pages.take(),fonts:fonts.take(),images:images.take(),forms:forms.take(),ext_g_states:ext_g_states.take(),shadings:shadings.take(),patterns:patterns.take(),color_spaces:color_spaces.take(),properties:properties.take(),outlines:outlines.take(),named_destinations:named_destinations.take(),page_labels:page_labels.take(),embedded_files:embedded_files.take(),output_intents:output_intents.take(),acro_form:acro_form.take(),optional_content:optional_content.take(),page_layout:page_layout.take(),page_mode:page_mode.take(),viewer_preferences:viewer_preferences.take(),open_action:open_action.take(),language:language.take(),mark_info:mark_info.take(),metadata:metadata.take(),document_id,encryption:encryption.take(),info:info.take(),catalog_extra:catalog_extra.take(),objects:objects.take(),trailer:trailer.take()})
    })
}

#[cfg(test)]
#[path="🧪️tests/🏭️producer/🦀️.rs"]
mod producer_tests;

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use framework_schema::ArtifactSchema;
use std::fmt;
use crate::standards::v1_7::subsets::base::io::text::snapshot as snapshot_text;

impl store::ArtifactDsl for PdfSnapshot {
    const EXTENSION: &'static str = "pdf";
    fn envelope_id() -> &'static str {
        STDIO_PDF17_DOCUMENT_SCHEMA
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((envelope, rest)) => {
                if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1) { return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PDF snapshot text envelope mismatch", semio_framework_diagnostic::TextSpan::at(1, 1))); }
                rest
            }
            Err(_) => text,
        };
        let record=semio_framework_dsl_record::parse(body,&snapshot_text::spec(),&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits{max_bytes:272*1024*1024,max_tokens:4_000_000,max_nodes:2_000_000,..semio_framework_diagnostic::Limits::default()},mode:semio_framework_dsl_record::SourceMode::Document})?;
        snapshot_text::from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body=semio_framework_dsl_record::print(&snapshot_text::to_record(self),&snapshot_text::spec(),semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;

mod source_fixtures {
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
/// 🆕️ A new pdf document: the empty 1.7 document as the real codec round-trips it — its retained object graph
/// (`objects`/`trailer`) is what a fresh write produced, read back, exactly like [`demo_pdf17_snapshot`]; the empty
/// `Default` carries no graph, so it saved as a document that reopened as a different one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn blank_pdf_snapshot() -> PdfSnapshot {
    use crate::standards::v1_7::subsets::base::io::{decode_pdf, encode_pdf};
    encode_pdf(&PdfSnapshot::default()).and_then(|bytes| decode_pdf(&bytes)).expect("blank_pdf_snapshot: the empty document round-trips through the real codec")
}
/// 📄️ The demo `stdio.pdf.1.7` document -- the single source of truth for `🏅️standards/7️⃣1.7/
/// 📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` (both are literally this
/// snapshot's `print_dsl`/`encode_pack` output, asserted equal by `fixture_honesty_law`).
///
/// Deliberately the real `decode_pdf(encode_pdf(seed))` FIXED POINT: the typed lanes survive the
/// round trip by the `lift(lower(t)) == t` law, and `objects`/`trailer` are whatever the fresh
/// write produced, read back — a hand-built snapshot with empty `objects` would not equal its
/// own decoded print.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_pdf17_snapshot() -> PdfSnapshot {
    let seed = crate::standards::v1_7::subsets::base::io::text_document(&[(200.0, 300.0, "Semio")]);
    let bytes = crate::standards::v1_7::subsets::base::io::encode_pdf(&seed).expect("encode_pdf(seed) must succeed");
    crate::standards::v1_7::subsets::base::io::decode_pdf(&bytes).expect("decode_pdf(encode_pdf(seed)) must succeed")
}
}
pub use source_fixtures::*;

#[path="📅️date/🦀️.rs"]
pub mod date;

#[path="🪪️retained-text/🦀️.rs"]
pub mod retained_text;
