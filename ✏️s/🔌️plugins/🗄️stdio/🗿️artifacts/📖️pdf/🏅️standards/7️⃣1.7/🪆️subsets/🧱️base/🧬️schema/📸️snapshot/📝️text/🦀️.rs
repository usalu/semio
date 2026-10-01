//! 📝️ Explicit complete owned snapshot record fields and canonical tagged semantic values.
use super::*;
use pack::value::{ToValue,FromValue,DslValue};
pub const COMPONENT_GRAMMAR_SEMIO:&str=include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH:&str="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio";

pub(super) fn spec()->dsl::RecordSpec{
    use dsl::{FieldSpec as F,Shape as S};
    dsl::RecordSpec::new(None,dsl::RecordLayout::Lines,vec![
        F::new(1,"schema",S::Text),F::new(2,"declaredVersion",S::Text),
        F::new(3,"pages",S::Value),F::new(4,"fonts",S::Value),F::new(5,"images",S::Value),F::new(6,"forms",S::Value),F::new(7,"extGStates",S::Value),F::new(8,"shadings",S::Value),F::new(9,"patterns",S::Value),F::new(10,"colorSpaces",S::Value),F::new(11,"properties",S::Value),F::new(12,"outlines",S::Value),F::new(13,"namedDestinations",S::Value),F::new(14,"pageLabels",S::Value),F::new(15,"embeddedFiles",S::Value),F::new(16,"outputIntents",S::Value),
        F::new(17,"acroForm",S::Value).optional(),F::new(18,"optionalContent",S::Value).optional(),F::new(19,"pageLayout",S::Value).optional(),F::new(20,"pageMode",S::Value).optional(),F::new(21,"viewerPreferences",S::Value).optional(),F::new(22,"openAction",S::Value).optional(),F::new(23,"language",S::Value).optional(),F::new(24,"markInfo",S::Value).optional(),F::new(25,"metadata",S::Value).optional(),F::new(26,"documentId",S::Value).optional(),F::new(27,"encryption",S::Value).optional(),
        F::new(28,"info",S::Value),F::new(29,"catalogExtra",S::Value),F::new(30,"objects",S::Value),F::new(31,"trailer",S::Value),
    ])
}
pub(super) fn to_record(value:&PdfSnapshot)->dsl::RecordValue{
    use dsl::FieldValue as V;
    dsl::RecordValue{fields:[
        (1,V::Text(value.schema.clone())),(2,V::Text(value.declared_version.clone())),
        (3,V::Value(value.pages.to_value())),(4,V::Value(value.fonts.to_value())),(5,V::Value(value.images.to_value())),(6,V::Value(value.forms.to_value())),(7,V::Value(value.ext_g_states.to_value())),(8,V::Value(value.shadings.to_value())),(9,V::Value(value.patterns.to_value())),(10,V::Value(value.color_spaces.to_value())),(11,V::Value(value.properties.to_value())),(12,V::Value(value.outlines.to_value())),(13,V::Value(value.named_destinations.to_value())),(14,V::Value(value.page_labels.to_value())),(15,V::Value(value.embedded_files.to_value())),(16,V::Value(value.output_intents.to_value())),
        (17,V::Value(value.acro_form.to_value())),(18,V::Value(value.optional_content.to_value())),(19,V::Value(value.page_layout.to_value())),(20,V::Value(value.page_mode.to_value())),(21,V::Value(value.viewer_preferences.to_value())),(22,V::Value(value.open_action.to_value())),(23,V::Value(value.language.to_value())),(24,V::Value(value.mark_info.to_value())),(25,V::Value(value.metadata.to_value())),(26,V::Value(value.document_id.to_value())),(27,V::Value(value.encryption.to_value())),
        (28,V::Value(value.info.to_value())),(29,V::Value(value.catalog_extra.to_value())),(30,V::Value(value.objects.to_value())),(31,V::Value(value.trailer.to_value())),
    ].into_iter().collect()}
}
fn field(record:&dsl::RecordValue,id:u16,key:&str)->Result<DslValue,store::TextError>{match record.get(id){Some(dsl::FieldValue::Text(value))if id<=2=>Ok(DslValue::String(value.clone())),Some(dsl::FieldValue::Value(value))if id>2=>Ok(value.clone()),None|Some(dsl::FieldValue::Absent)if(17..=27).contains(&id)=>Ok(DslValue::Null),_=>Err(store::TextError::new(format!("PDF snapshot field {key} is missing or has a different shape"),dsl::TextSpan::at(1,1)))}}
pub(super) fn from_record(record:&dsl::RecordValue)->Result<PdfSnapshot,store::TextError>{
    if record.fields.keys().any(|id|!(1..=31).contains(id)){return Err(store::TextError::new("PDF snapshot contains an undeclared root field",dsl::TextSpan::at(1,1)));}
    PdfSnapshot::from_value(DslValue::object([
        ("schema".into(),field(record,1,"schema")?),("declaredVersion".into(),field(record,2,"declaredVersion")?),
        ("pages".into(),field(record,3,"pages")?),("fonts".into(),field(record,4,"fonts")?),("images".into(),field(record,5,"images")?),("forms".into(),field(record,6,"forms")?),("extGStates".into(),field(record,7,"extGStates")?),("shadings".into(),field(record,8,"shadings")?),("patterns".into(),field(record,9,"patterns")?),("colorSpaces".into(),field(record,10,"colorSpaces")?),("properties".into(),field(record,11,"properties")?),("outlines".into(),field(record,12,"outlines")?),("namedDestinations".into(),field(record,13,"namedDestinations")?),("pageLabels".into(),field(record,14,"pageLabels")?),("embeddedFiles".into(),field(record,15,"embeddedFiles")?),("outputIntents".into(),field(record,16,"outputIntents")?),
        ("acroForm".into(),field(record,17,"acroForm")?),("optionalContent".into(),field(record,18,"optionalContent")?),("pageLayout".into(),field(record,19,"pageLayout")?),("pageMode".into(),field(record,20,"pageMode")?),("viewerPreferences".into(),field(record,21,"viewerPreferences")?),("openAction".into(),field(record,22,"openAction")?),("language".into(),field(record,23,"language")?),("markInfo".into(),field(record,24,"markInfo")?),("metadata".into(),field(record,25,"metadata")?),("documentId".into(),field(record,26,"documentId")?),("encryption".into(),field(record,27,"encryption")?),
        ("info".into(),field(record,28,"info")?),("catalogExtra".into(),field(record,29,"catalogExtra")?),("objects".into(),field(record,30,"objects")?),("trailer".into(),field(record,31,"trailer")?),
    ])).map_err(|error|store::TextError::new(error.to_string(),dsl::TextSpan::at(1,1)))
}
