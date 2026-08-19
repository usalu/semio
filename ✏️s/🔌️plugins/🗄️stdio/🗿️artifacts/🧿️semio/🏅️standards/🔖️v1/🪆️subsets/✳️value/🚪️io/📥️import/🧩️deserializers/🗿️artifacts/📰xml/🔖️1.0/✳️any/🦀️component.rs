//! 📥️ `SemioValueFromXml` — element/attribute tree -> typed value graph. More lossy than the
//! json pair (see the serializer's own doc comment for the honest asymmetry), but the STRUCTURE
//! mapping itself is a real, reversible convention (used consistently by both directions):
//!
//! The whole `XmlDocument` becomes ONE `"document"`-tagged `SemioValue::Map`:
//! `{kind:"document", declaration: <decl-map|Null>, doctype: <Str|Null>, prolog: <nodes>, root: <node|Null>}`.
//! Each `XmlNode` becomes a `kind`-tagged map: `element` carries `tag`/`attrs`/`children`; `text`/
//! `cdata`/`comment` carry `text`; `pi` carries `target`/`data`. This is a lossless, information-
//! preserving encoding of the STRUCTURE — every `XmlNode` variant, every attribute, sibling order,
//! and the declaration/doctype both round-trip exactly (proven by the serializer's own round-trip
//! test). `nodes` always decodes empty — XML has no id-graph/reference concept for `Ref` to come
//! from.

use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueSnapshot, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};
use crate::artifacts::xml::schema::snapshot::{XmlDeclaration, XmlDoctype, XmlDocument, XmlDtdDeclaration, XmlExternalId, XmlNode};
use crate::artifacts::xml::XmlSnapshot;
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

//#region 🔖️Deserializer
pub struct SemioValueFromXml;

impl ArtifactDeserializer for SemioValueFromXml {
    type From = XmlSnapshot;
    type Into = SemioValueSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId::ANY };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("value") };

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        Ok(SemioValueSnapshot { schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root: semio_value_from_xml_document(&from.doc), nodes: Vec::new() })
    }
}

pub fn register() {}
//#endregion 🔖️Deserializer

//#region 🔖️Convert
fn entry(key: &str, value: SemioValue) -> SemioValueEntry {
    SemioValueEntry { key: key.into(), value }
}

fn str_value(s: &str) -> SemioValue {
    SemioValue::Str { value: s.to_string() }
}

fn kind_tagged(kind: &str, text: &str) -> SemioValue {
    SemioValue::Map { entries: vec![entry("kind", str_value(kind)), entry("text", str_value(text))] }
}

pub fn semio_value_of_node(node: &XmlNode) -> SemioValue {
    match node {
        XmlNode::Element { name, attrs, children } => SemioValue::Map {
            entries: vec![
                entry("kind", str_value("element")),
                entry("tag", str_value(name)),
                entry("attrs", SemioValue::Map { entries: attrs.iter().map(|a| entry(&a.name, str_value(&a.value))).collect() }),
                entry("children", SemioValue::List { items: children.iter().map(semio_value_of_node).collect() }),
            ],
        },
        XmlNode::Text { text } => kind_tagged("text", text),
        XmlNode::CData { text } => kind_tagged("cdata", text),
        XmlNode::Comment { text } => kind_tagged("comment", text),
        XmlNode::ProcessingInstruction { target, data } => SemioValue::Map { entries: vec![entry("kind", str_value("pi")), entry("target", str_value(target)), entry("data", str_value(data))] },
    }
}

fn semio_value_of_declaration(d: &XmlDeclaration) -> SemioValue {
    SemioValue::Map {
        entries: vec![entry("version", str_value(&d.version)), entry("encoding", d.encoding.as_deref().map(str_value).unwrap_or(SemioValue::Null)), entry("standalone", d.standalone.map(|b| SemioValue::Bool { value: b }).unwrap_or(SemioValue::Null))],
    }
}

fn semio_value_of_doctype(doctype: &XmlDoctype) -> SemioValue {
    let external_id = match &doctype.external_id {
        None => SemioValue::Null,
        Some(XmlExternalId::System { system_id }) => SemioValue::Map { entries: vec![entry("kind", str_value("system")), entry("systemId", str_value(system_id))] },
        Some(XmlExternalId::Public { public_id, system_id }) => SemioValue::Map { entries: vec![entry("kind", str_value("public")), entry("publicId", str_value(public_id)), entry("systemId", str_value(system_id))] },
    };
    let declarations = doctype
        .declarations
        .iter()
        .map(|declaration| match declaration {
            XmlDtdDeclaration::Entity { parameter, name, value } => {
                SemioValue::Map { entries: vec![entry("kind", str_value("entity")), entry("parameter", SemioValue::Bool { value: *parameter }), entry("name", str_value(name)), entry("value", str_value(value))] }
            }
        })
        .collect();
    SemioValue::Map { entries: vec![entry("name", str_value(&doctype.name)), entry("externalId", external_id), entry("declarations", SemioValue::List { items: declarations })] }
}

pub fn semio_value_from_xml_document(doc: &XmlDocument) -> SemioValue {
    SemioValue::Map {
        entries: vec![
            entry("kind", str_value("document")),
            entry("declaration", doc.declaration.as_ref().map(semio_value_of_declaration).unwrap_or(SemioValue::Null)),
            entry("doctype", doc.doctype.as_ref().map(semio_value_of_doctype).unwrap_or(SemioValue::Null)),
            entry("prolog", SemioValue::List { items: doc.prolog.iter().map(semio_value_of_node).collect() }),
            entry("root", doc.root.as_ref().map(semio_value_of_node).unwrap_or(SemioValue::Null)),
        ],
    }
}
//#endregion 🔖️Convert

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::xml::schema::snapshot::XmlAttr;

    #[test]
    fn element_with_attrs_and_children_maps_to_a_kind_tagged_structure() {
        let doc = XmlDocument {
            root: Some(XmlNode::Element { name: "svg".into(), attrs: vec![XmlAttr { name: "viewBox".into(), value: "0 0 10 10".into() }], children: vec![XmlNode::Text { text: "hi".into() }, XmlNode::Comment { text: "note".into() }] }),
            doctype: None,
            declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: Some(true) }),
            prolog: vec![XmlNode::Comment { text: "generated".into() }],
        };
        let value = semio_value_from_xml_document(&doc);
        match &value {
            SemioValue::Map { entries } => {
                assert_eq!(entries[0], SemioValueEntry { key: "kind".into(), value: str_value("document") });
                let root = entries.iter().find(|e| e.key == "root").expect("root entry").value.clone();
                match root {
                    SemioValue::Map { entries } => {
                        assert!(entries.contains(&SemioValueEntry { key: "tag".into(), value: str_value("svg") }));
                        let children = entries.iter().find(|e| e.key == "children").unwrap().value.clone();
                        match children {
                            SemioValue::List { items } => assert_eq!(items.len(), 2),
                            other => panic!("expected list, got {other:?}"),
                        }
                    }
                    other => panic!("expected map, got {other:?}"),
                }
            }
            other => panic!("expected map, got {other:?}"),
        }
    }

    #[test]
    fn empty_document_still_produces_a_document_shaped_map() {
        let value = semio_value_from_xml_document(&XmlDocument::default());
        match value {
            SemioValue::Map { entries } => {
                assert_eq!(entries.iter().find(|e| e.key == "root").unwrap().value, SemioValue::Null);
                assert_eq!(entries.iter().find(|e| e.key == "doctype").unwrap().value, SemioValue::Null);
            }
            other => panic!("expected map, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
