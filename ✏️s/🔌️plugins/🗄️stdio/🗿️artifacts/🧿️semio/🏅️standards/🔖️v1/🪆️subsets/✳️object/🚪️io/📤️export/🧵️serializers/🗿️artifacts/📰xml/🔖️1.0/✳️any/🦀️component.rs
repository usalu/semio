//! 📤️ `SemioObjectToXml` — mirror of `SemioObjectFromXml`. Expects the ROOT `SemioValue` to be
//! shaped exactly as the deserializer produces it (a `"document"`-tagged map wrapping an optional
//! `"element"`/`"text"`/`"cdata"`/`"comment"`/`"pi"`-tagged node tree) — that convention is real
//! and reversible, but genuinely MORE LOSSY than the object↔json pair as soon as the object graph
//! doesn't already conform to it:
//! - Any `SemioValue` shape that doesn't match the expected tagged-map convention (wrong `kind`,
//!   a `List` where a `Map` was expected, a non-`Str` attribute value, …) is a hard `PackError` —
//!   never silently coerced or dropped, since there is no honest default XML rendering for an
//!   arbitrary object graph.
//! - `Int`/`Float`/`Bool`/`Bytes` values can only ever appear as XML-representable content if a
//!   hand-authored object happens to nest them somewhere this convention doesn't read (e.g. inside
//!   an `attrs` value) — an attribute value or text/cdata/comment/pi field that resolves to
//!   anything other than `Str` is rejected rather than stringified, since XML has no place to
//!   record which original semio TYPE produced a given string.
//! - `Ref{id}` is dereferenced the same way the object↔json serializer does (XML has no graph
//!   either) — dangling refs and cycles are hard errors, never silently truncated.

use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::{ObjectId, SemioObjectEntry, SemioObjectSnapshot, SemioValue};
use crate::artifacts::xml::XmlSnapshot;
use crate::artifacts::xml::STDIO_XML_DOCUMENT_SCHEMA;
use crate::artifacts::xml::schema::snapshot::{XmlAttr, XmlDeclaration, XmlDocument, XmlNode};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use std::collections::{HashMap, HashSet};

//#region 🔖️Serializer
pub struct SemioObjectToXml;

impl ArtifactSerializer for SemioObjectToXml {
    type From = SemioObjectSnapshot;
    type Into = XmlSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("object") };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId::ANY };

    fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let objects: HashMap<&ObjectId, &SemioValue> = from.objects.iter().map(|n| (&n.id, &n.value)).collect();
        let mut visiting: HashSet<ObjectId> = HashSet::new();
        let doc = xml_document_from_semio(&from.root, &objects, &mut visiting)?;
        Ok(XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc })
    }
}

pub fn register() {}
//#endregion 🔖️Serializer

//#region 🔖️Resolve
fn err(msg: impl Into<String>) -> store::PackError {
    store::PackError::Schema(msg.into())
}

fn resolve(v: &SemioValue, objects: &HashMap<&ObjectId, &SemioValue>, visiting: &mut HashSet<ObjectId>) -> Result<SemioValue, store::PackError> {
    match v {
        SemioValue::Ref { id } => {
            if !visiting.insert(id.clone()) {
                return Err(err(format!("object->xml: reference cycle detected at id {:?} (xml has no graph)", id.value)));
            }
            let target = *objects.get(id).ok_or_else(|| err(format!("object->xml: dangling Ref{{id: {:?}}} — not found in `objects`", id.value)))?;
            let result = resolve(target, objects, visiting);
            visiting.remove(id);
            result
        }
        other => Ok(other.clone()),
    }
}

fn expect_entries(v: &SemioValue) -> Result<Vec<SemioObjectEntry>, store::PackError> {
    match v {
        SemioValue::Map { entries } => Ok(entries.clone()),
        other => Err(err(format!("object->xml: expected a Map, got {other:?}"))),
    }
}

fn find(entries: &[SemioObjectEntry], key: &str) -> Option<SemioValue> {
    entries.iter().find(|e| e.key == key).map(|e| e.value.clone())
}

fn expect_str(v: &SemioValue) -> Result<String, store::PackError> {
    match v {
        SemioValue::Str { value } => Ok(value.clone()),
        other => Err(err(format!("object->xml: expected a Str, got {other:?}"))),
    }
}

fn expect_kind(entries: &[SemioObjectEntry], objects: &HashMap<&ObjectId, &SemioValue>, visiting: &mut HashSet<ObjectId>) -> Result<String, store::PackError> {
    let raw = find(entries, "kind").ok_or_else(|| err("object->xml: missing required \"kind\" entry"))?;
    expect_str(&resolve(&raw, objects, visiting)?)
}
//#endregion 🔖️Resolve

//#region 🔖️Convert
fn xml_node_from_semio(v: &SemioValue, objects: &HashMap<&ObjectId, &SemioValue>, visiting: &mut HashSet<ObjectId>) -> Result<XmlNode, store::PackError> {
    let resolved = resolve(v, objects, visiting)?;
    let entries = expect_entries(&resolved)?;
    let kind = expect_kind(&entries, objects, visiting)?;
    match kind.as_str() {
        "element" => {
            let tag_raw = find(&entries, "tag").ok_or_else(|| err("object->xml: element missing \"tag\""))?;
            let name = expect_str(&resolve(&tag_raw, objects, visiting)?)?;
            let attrs_raw = find(&entries, "attrs").ok_or_else(|| err("object->xml: element missing \"attrs\""))?;
            let attrs_entries = expect_entries(&resolve(&attrs_raw, objects, visiting)?)?;
            let attrs = attrs_entries
                .iter()
                .map(|e| Ok(XmlAttr { name: e.key.clone(), value: expect_str(&resolve(&e.value, objects, visiting)?)? }))
                .collect::<Result<Vec<_>, store::PackError>>()?;
            let children_raw = find(&entries, "children").ok_or_else(|| err("object->xml: element missing \"children\""))?;
            let children_resolved = resolve(&children_raw, objects, visiting)?;
            let items = match children_resolved {
                SemioValue::List { items } => items,
                other => return Err(err(format!("object->xml: \"children\" must be a List, got {other:?}"))),
            };
            let children = items.iter().map(|c| xml_node_from_semio(c, objects, visiting)).collect::<Result<Vec<_>, store::PackError>>()?;
            Ok(XmlNode::Element { name, attrs, children })
        }
        "text" => Ok(XmlNode::Text { text: expect_str(&resolve(&find(&entries, "text").ok_or_else(|| err("object->xml: text node missing \"text\""))?, objects, visiting)?)? }),
        "cdata" => Ok(XmlNode::CData { text: expect_str(&resolve(&find(&entries, "text").ok_or_else(|| err("object->xml: cdata node missing \"text\""))?, objects, visiting)?)? }),
        "comment" => Ok(XmlNode::Comment { text: expect_str(&resolve(&find(&entries, "text").ok_or_else(|| err("object->xml: comment node missing \"text\""))?, objects, visiting)?)? }),
        "pi" => {
            let target = expect_str(&resolve(&find(&entries, "target").ok_or_else(|| err("object->xml: pi node missing \"target\""))?, objects, visiting)?)?;
            let data = expect_str(&resolve(&find(&entries, "data").ok_or_else(|| err("object->xml: pi node missing \"data\""))?, objects, visiting)?)?;
            Ok(XmlNode::ProcessingInstruction { target, data })
        }
        other => Err(err(format!("object->xml: unknown node kind {other:?}"))),
    }
}

fn xml_declaration_from_semio(v: &SemioValue, objects: &HashMap<&ObjectId, &SemioValue>, visiting: &mut HashSet<ObjectId>) -> Result<XmlDeclaration, store::PackError> {
    let entries = expect_entries(v)?;
    let version = expect_str(&resolve(&find(&entries, "version").ok_or_else(|| err("object->xml: declaration missing \"version\""))?, objects, visiting)?)?;
    let encoding = match find(&entries, "encoding") {
        Some(raw) => match resolve(&raw, objects, visiting)? {
            SemioValue::Null => None,
            SemioValue::Str { value } => Some(value),
            other => return Err(err(format!("object->xml: declaration \"encoding\" must be Str or Null, got {other:?}"))),
        },
        None => None,
    };
    let standalone = match find(&entries, "standalone") {
        Some(raw) => match resolve(&raw, objects, visiting)? {
            SemioValue::Null => None,
            SemioValue::Bool { value } => Some(value),
            other => return Err(err(format!("object->xml: declaration \"standalone\" must be Bool or Null, got {other:?}"))),
        },
        None => None,
    };
    Ok(XmlDeclaration { version, encoding, standalone })
}

pub fn xml_document_from_semio(v: &SemioValue, objects: &HashMap<&ObjectId, &SemioValue>, visiting: &mut HashSet<ObjectId>) -> Result<XmlDocument, store::PackError> {
    let resolved = resolve(v, objects, visiting)?;
    let entries = expect_entries(&resolved)?;
    let kind = expect_kind(&entries, objects, visiting)?;
    if kind != "document" {
        return Err(err(format!("object->xml: expected kind \"document\" at the snapshot root, got {kind:?}")));
    }
    let declaration = match find(&entries, "declaration") {
        Some(raw) => match resolve(&raw, objects, visiting)? {
            SemioValue::Null => None,
            other @ SemioValue::Map { .. } => Some(xml_declaration_from_semio(&other, objects, visiting)?),
            other => return Err(err(format!("object->xml: \"declaration\" must be Map or Null, got {other:?}"))),
        },
        None => None,
    };
    let doctype = match find(&entries, "doctype") {
        Some(raw) => match resolve(&raw, objects, visiting)? {
            SemioValue::Null => None,
            SemioValue::Str { value } => Some(value),
            other => return Err(err(format!("object->xml: \"doctype\" must be Str or Null, got {other:?}"))),
        },
        None => None,
    };
    let root = match find(&entries, "root") {
        Some(raw) => match resolve(&raw, objects, visiting)? {
            SemioValue::Null => None,
            other => Some(xml_node_from_semio(&other, objects, visiting)?),
        },
        None => None,
    };
    Ok(XmlDocument { root, doctype, declaration })
}
//#endregion 🔖️Convert

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::object::io::import::deserializers::artifacts::xml::v1_0::any::semio_value_from_xml_document;

    fn round_trip(doc: XmlDocument) -> XmlDocument {
        let value = semio_value_from_xml_document(&doc);
        let objects = HashMap::new();
        let mut visiting = HashSet::new();
        xml_document_from_semio(&value, &objects, &mut visiting).expect("object->xml")
    }

    /// 🧪️ Required proof: xml -> object -> xml -> object round trip preserves everything the
    /// object subset can represent (the whole document, structurally — nothing is lossy in THIS
    /// direction since the fixture already conforms to the tagged-map convention).
    #[test]
    fn xml_to_object_to_xml_round_trips_structurally() {
        let doc = XmlDocument {
            root: Some(XmlNode::Element {
                name: "svg".into(),
                attrs: vec![XmlAttr { name: "viewBox".into(), value: "0 0 10 10".into() }, XmlAttr { name: "xmlns".into(), value: "http://www.w3.org/2000/svg".into() }],
                children: vec![
                    XmlNode::Element { name: "rect".into(), attrs: vec![XmlAttr { name: "width".into(), value: "5".into() }], children: vec![] },
                    XmlNode::Comment { text: "a note".into() },
                    XmlNode::CData { text: "raw <stuff>".into() },
                    XmlNode::Text { text: "hello & goodbye".into() },
                ],
            }),
            doctype: Some("<!DOCTYPE svg>".into()),
            declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: Some(false) }),
        };
        assert_eq!(round_trip(doc.clone()), doc);
    }

    #[test]
    fn empty_document_round_trips() {
        assert_eq!(round_trip(XmlDocument::default()), XmlDocument::default());
    }

    #[test]
    fn non_conforming_shape_is_a_hard_error_not_a_silent_default() {
        let objects = HashMap::new();
        let mut visiting = HashSet::new();
        let bogus = SemioValue::Int { lexeme: "1".into() };
        assert!(xml_document_from_semio(&bogus, &objects, &mut visiting).is_err());
    }

    #[test]
    fn ref_is_dereferenced_and_cycles_error() {
        let id = ObjectId::new("self");
        let cyclic = SemioValue::Ref { id: id.clone() };
        let mut objects_owned: HashMap<&ObjectId, &SemioValue> = HashMap::new();
        objects_owned.insert(&id, &cyclic);
        let mut visiting = HashSet::new();
        assert!(xml_document_from_semio(&cyclic, &objects_owned, &mut visiting).is_err());
    }
}
//#endregion 🧪️Tests
