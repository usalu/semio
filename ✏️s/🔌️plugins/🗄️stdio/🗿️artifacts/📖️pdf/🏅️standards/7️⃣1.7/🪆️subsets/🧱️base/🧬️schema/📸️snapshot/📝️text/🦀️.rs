//! 📝️ Explicit complete owned snapshot record fields and canonical tagged semantic values.
use super::*;
use pack::value::{ToValue,FromValue,DslValue};
pub const COMPONENT_GRAMMAR_SEMIO:&str=include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH:&str="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio";

const FIELDS:[(u16,&str,dsl::Shape,bool);31]=[
    (1,"schema",dsl::Shape::Text,false),
    (2,"declaredVersion",dsl::Shape::Text,false),
    (3,"pages",dsl::Shape::Value,false),
    (4,"fonts",dsl::Shape::Value,false),
    (5,"images",dsl::Shape::Value,false),
    (6,"forms",dsl::Shape::Value,false),
    (7,"extGStates",dsl::Shape::Value,false),
    (8,"shadings",dsl::Shape::Value,false),
    (9,"patterns",dsl::Shape::Value,false),
    (10,"colorSpaces",dsl::Shape::Value,false),
    (11,"properties",dsl::Shape::Value,false),
    (12,"outlines",dsl::Shape::Value,false),
    (13,"namedDestinations",dsl::Shape::Value,false),
    (14,"pageLabels",dsl::Shape::Value,false),
    (15,"embeddedFiles",dsl::Shape::Value,false),
    (16,"outputIntents",dsl::Shape::Value,false),
    (17,"acroForm",dsl::Shape::Value,true),
    (18,"optionalContent",dsl::Shape::Value,true),
    (19,"pageLayout",dsl::Shape::Value,true),
    (20,"pageMode",dsl::Shape::Value,true),
    (21,"viewerPreferences",dsl::Shape::Value,true),
    (22,"openAction",dsl::Shape::Value,true),
    (23,"language",dsl::Shape::Value,true),
    (24,"markInfo",dsl::Shape::Value,true),
    (25,"metadata",dsl::Shape::Value,true),
    (26,"documentId",dsl::Shape::Value,true),
    (27,"encryption",dsl::Shape::Value,true),
    (28,"info",dsl::Shape::Value,false),
    (29,"catalogExtra",dsl::Shape::Value,false),
    (30,"objects",dsl::Shape::Value,false),
    (31,"trailer",dsl::Shape::Value,false),
];
pub(super) fn spec()->dsl::RecordSpec{
    dsl::RecordSpec::new(None,dsl::RecordLayout::Lines,FIELDS.into_iter().map(|(id,key,shape,optional)|{let mut field=dsl::FieldSpec::new(id,key,shape);field.optional=optional;field}).collect())
}
fn spec_controlled<C:dsl::NativeSchemaControl>(control:&mut C)->Result<dsl::RecordSpec,String>{
    control.scoped_stage(|control|{
        control.begin_stage(FIELDS.len())?;let mut fields=control.allocate_vec::<dsl::FieldSpec>(FIELDS.len())?;
        for(id,key,shape,optional)in FIELDS{control.checkpoint()?;let mut field=dsl::schema::producer::field(id,key,shape,control)?;field.optional=optional;fields.push(field);control.step()?;}
        dsl::schema::producer::record(None,dsl::RecordLayout::Lines,fields,control)
    })
}
/// 🏭️ Owns literal snapshot metadata independently under either native allocation controller.
pub(super) fn spec_producer()->dsl::RecordSpecProducer{dsl::RecordSpecProducer{ordinary:spec,decoding:|control|spec_controlled(control),encoding:|control|spec_controlled(control)}}

pub(super) fn to_record(value:&PdfSnapshot)->dsl::RecordValue{
    use dsl::FieldValue as V;
    dsl::RecordValue{fields:[
        (1,V::Text(value.schema.clone())),(2,V::Text(value.declared_version.clone())),
        (3,V::Value(value.pages.to_value())),(4,V::Value(value.fonts.to_value())),(5,V::Value(value.images.to_value())),(6,V::Value(value.forms.to_value())),(7,V::Value(value.ext_g_states.to_value())),(8,V::Value(value.shadings.to_value())),(9,V::Value(value.patterns.to_value())),(10,V::Value(value.color_spaces.to_value())),(11,V::Value(value.properties.to_value())),(12,V::Value(value.outlines.to_value())),(13,V::Value(value.named_destinations.to_value())),(14,V::Value(value.page_labels.to_value())),(15,V::Value(value.embedded_files.to_value())),(16,V::Value(value.output_intents.to_value())),
        (17,V::Value(value.acro_form.to_value())),(18,V::Value(value.optional_content.to_value())),(19,V::Value(value.page_layout.to_value())),(20,V::Value(value.page_mode.to_value())),(21,V::Value(value.viewer_preferences.to_value())),(22,V::Value(value.open_action.to_value())),(23,V::Value(value.language.to_value())),(24,V::Value(value.mark_info.to_value())),(25,V::Value(value.metadata.to_value())),(26,V::Value(crate::standards::v1_7::subsets::base::schema::snapshot::octets::document_id_to_value(&value.document_id))),(27,V::Value(value.encryption.to_value())),
        (28,V::Value(value.info.to_value())),(29,V::Value(value.catalog_extra.to_value())),(30,V::Value(value.objects.to_value())),(31,V::Value(value.trailer.to_value())),
    ].into_iter().collect()}
}
/// 🛫️ Projects each literal root slot with bounded allocation and partial-record retirement.
pub(super) fn to_record_controlled(value:&PdfSnapshot,control:&mut dsl::NativeEncodeControl<'_>)->Result<dsl::RecordValue,store::TextError>{
    let error=|message:String|store::TextError::new(message,dsl::TextSpan::at(1,1));
    control.scoped_depth(64,|control|control.scoped_stage(|control|->Result<dsl::RecordValue,String>{
        control.begin_stage(FIELDS.len())?;
        let mut record=dsl::os_dsl::native_encoding::EncodedRecord::new(FIELDS.len(),control)?;
        record.insert(1,dsl::FieldValue::Text(control.copy_text(&value.schema)?));control.step()?;
        record.insert(2,dsl::FieldValue::Text(control.copy_text(&value.declared_version)?));control.step()?;
        macro_rules! field{($id:literal,$key:literal,$output:expr)=>{{let output=$output.map_err(|failure|failure.under($key).to_string())?;record.insert($id,dsl::FieldValue::Value(output));control.step()?;}};}
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
    })).map_err(error)
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

fn borrowed_field<'a>(record:&'a dsl::RecordValue,id:u16,key:&str)->Result<&'a DslValue,store::TextError>{match record.get(id){Some(dsl::FieldValue::Value(value))=>Ok(value),None|Some(dsl::FieldValue::Absent)if(17..=27).contains(&id)=>Ok(&DslValue::Null),_=>Err(store::TextError::new(format!("PDF snapshot field {key} is missing or has a different shape"),dsl::TextSpan::at(1,1)))}}
fn owned<T:FromValue>(record:&dsl::RecordValue,id:u16,key:&str,control:&mut dsl::NativeDecodeControl<'_>)->Result<pack::value::DecodedValue<T>,store::TextError>{let value=T::from_value_controlled(borrowed_field(record,id,key)?,control).map_err(|error|store::TextError::new(error.under(key).to_string(),dsl::TextSpan::at(1,1)))?;let owner=pack::value::DecodedValue::new(value,T::retire_decoded);control.step().map_err(|error|store::TextError::new(error,dsl::TextSpan::at(1,1)))?;Ok(owner)}
pub(super)fn from_record_controlled(record:&dsl::RecordValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<PdfSnapshot,store::TextError>{
    let error=|message:String|store::TextError::new(message,dsl::TextSpan::at(1,1));
    control.scoped_stage(|control|{
        control.begin_stage(31).map_err(error)?;if record.fields.keys().any(|id|!(1..=31).contains(id)){return Err(error("PDF snapshot contains an undeclared root field".into()));}
        let text=|id,key:&str,control:&mut dsl::NativeDecodeControl<'_>|{let Some(dsl::FieldValue::Text(value))=record.get(id)else{return Err(error(format!("PDF snapshot field {key} is missing or has a different shape")))};let value=control.copy_text(value).map_err(error)?;control.step().map_err(error)?;Ok::<_,store::TextError>(value)};
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
        let document_id=octets::document_id_from_value_controlled(borrowed_field(record,26,"documentId")?,control).map_err(|value|error(value.under("documentId").to_string()))?;control.step().map_err(error)?;
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
