//! 🧪️ Native SVG admission agrees with the shared owned-value corpus.
use super::*;
#[test]
fn svg_typed_attribute_neutral_corpus_decodes_and_formats_owned_values() {
    use semio_framework_value::FromValue;
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧫️fixtures/🧩️typed-attributes/🔣️.json")).unwrap();
    for row in corpus["cases"].as_array().unwrap() {
        let value=semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(&row["owned"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let expected=crate::schema::snapshot::SvgAttributeValue::from_value(value).unwrap();
        let name=row["name"].as_str().unwrap();let bound=bind_svg_attribute(name,row["native"].as_str().unwrap()).unwrap();
        assert_eq!(bound,expected,"{name}");assert_eq!(bind_svg_attribute(name,&print_svg_attribute(&bound)).unwrap(),expected,"{name}");
    }
    for row in corpus["invalid"].as_array().unwrap(){assert!(bind_svg_attribute(row["name"].as_str().unwrap(),row["native"].as_str().unwrap()).is_err());}
}

#[test]
fn credited_svg_projection_refuses_cancellation_and_small_ownership_before_native_output(){
    use semio_framework_value::{NativeEncodeControl,ValueRefusalKind};
    let doc=bind_svg_document(semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_from_text("<svg viewBox='0 0 20 10'><path d='M0 0L10 10Z'/></svg>").unwrap()).unwrap();
    let mut reject=|_|false;let error=native_svg_document_controlled(&doc,&mut NativeEncodeControl::new(1_000_000,&mut reject)).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::Canceled);
    let mut accept=|_|true;let error=native_svg_document_controlled(&doc,&mut NativeEncodeControl::new(1,&mut accept)).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);
    let mut progress=0;let mut observe=|_|{progress+=1;true};let mut control=NativeEncodeControl::new(1_000_000,&mut observe);let projected=native_svg_document_controlled(&doc,&mut control).unwrap();assert!(control.owned_bytes()>0);drop(control);assert!(progress>0);assert_eq!(bind_svg_document(projected).unwrap(),doc);
}
#[test]
fn credited_svg_projection_retires_deep_partial_native_tree_iteratively(){
    use crate::schema::snapshot::{SvgDocument,SvgNode};
    use semio_framework_value::NativeEncodeControl;
    let mut root=SvgNode::Element{name:"path".into(),attrs:vec![],children:vec![]};for _ in 0..8192{root=SvgNode::Element{name:"g".into(),attrs:vec![],children:vec![root]};}
    let mut owner=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(SvgDocument{root:Some(root),..Default::default()},crate::schema::snapshot::retire_svg_document);
    let mut accept=|_|true;let output=native_svg_document_controlled(owner.as_mut(),&mut NativeEncodeControl::new(32_000_000,&mut accept)).unwrap();semio_s_artifact_stdio_xml::schema::snapshot::ownership::retire_xml_document(output);
}

#[test]
fn svg_owned_attribute_guard_rejects_native_spelling_in_known_positions(){
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧫️fixtures/🧩️typed-attributes/🔣️.json")).unwrap();
    for row in corpus["invalidOwned"].as_array().unwrap(){assert!(semio_framework_pack_json::from_json_str::<crate::schema::snapshot::SvgAttr>(&row.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(),"{row}");}
}
