use super::*;
pub(crate) fn fixture()->PptxSnapshot{
 let neutral:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("closed neutral outline fixture");
 semio_framework_pack_json::from_json_str(&neutral["snapshot"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("actual OPC and XML fixture")
}
#[semio_framework_async_macros::async_test]
async fn counts_slides_shapes_and_words(){
 let snapshot=fixture();let outline=PptxOutline::compute(&snapshot).expect("retained presentation");
 assert_eq!(outline.slide_count,1);assert_eq!(outline.shape_count,2);assert_eq!(outline.word_count,2);
}
#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic(){
 let snapshot=fixture();assert_eq!(PptxOutline::compute(&snapshot).unwrap(),PptxOutline::compute(&snapshot).unwrap());
}
#[semio_framework_async_macros::async_test]
async fn outline_refuses_missing_retained_authority(){
 let mut relationship=fixture();relationship.opc.relationships = semio_s_artifact_stdio_zip::opc::OpcRelationshipOwners::new();
 let mut part=fixture();part.xml_parts.pop();
 let mut namespace=fixture();
 if let Some(semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element{attrs,..})=namespace.xml_parts[0].document.root.as_mut(){attrs[0].value="foreign".into();}
 for snapshot in[relationship,part,namespace,PptxSnapshot::default()]{assert_eq!(PptxOutline::compute(&snapshot).unwrap_err().kind,ValueRefusalKind::InvalidValue);}
}
