//! 📤️ `SemioValueToXml` — mirror of `SemioValueFromXml`. Expects the ROOT `SemioValue` to be
//! shaped exactly as the deserializer produces it (a `"document"`-tagged map wrapping an optional
//! `"element"`/`"text"`/`"cdata"`/`"comment"`/`"pi"`-tagged node tree) — that convention is real
//! and reversible, but genuinely MORE LOSSY than the value↔json pair as soon as the value graph
//! doesn't already conform to it:
//! - Any `SemioValue` shape that doesn't match the expected tagged-map convention (wrong `kind`,
//!   a `List` where a `Map` was expected, a non-`Str` attribute value, …) is a hard `PackError` —
//!   never silently coerced or dropped, since there is no honest default XML rendering for an
//!   arbitrary value graph.
//! - `Int`/`Float`/`Bool`/`Bytes` values can only ever appear as XML-representable content if a
//!   hand-authored value happens to nest them somewhere this convention doesn't read (e.g. inside
//!   an `attrs` value) — an attribute value or text/cdata/comment/pi field that resolves to
//!   anything other than `Str` is rejected rather than stringified, since XML has no place to
//!   record which original semio TYPE produced a given string.
//! - `Ref{id}` is dereferenced the same way the value↔json serializer does (XML has no graph
//!   either) — dangling refs and cycles are hard errors, never silently truncated.

use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueSnapshot, ValueId};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlDeclaration, XmlDoctype, XmlDocument, XmlDtdDeclaration, XmlExternalId, XmlNode};
use semio_s_artifact_stdio_xml::XmlSnapshot;
use semio_s_artifact_stdio_xml::STDIO_XML_DOCUMENT_SCHEMA;
use std::collections::{HashMap, HashSet};

//#region 🔖️Serializer
pub struct SemioValueToXml;

impl ArtifactSerializer for SemioValueToXml {
    type From = SemioValueSnapshot;
    type Into = XmlSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("value") };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId::ANY };

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let nodes: HashMap<&ValueId, &SemioValue> = from.nodes.iter().map(|n| (&n.id, &n.value)).collect();
        let mut visiting: HashSet<ValueId> = HashSet::new();
        let doc = xml_document_from_semio(&from.root, &nodes, &mut visiting)?;
        Ok(XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
//#endregion 🔖️Serializer

//#region 🔖️Resolve
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn err(msg: impl Into<String>) -> store::PackError {
    store::PackError::Schema(msg.into())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn resolve(v: &SemioValue, nodes: &HashMap<&ValueId, &SemioValue>, visiting: &mut HashSet<ValueId>) -> Result<SemioValue, store::PackError> {
    match v {
        SemioValue::Ref { id } => {
            if !visiting.insert(id.clone()) {
                return Err(err(format!("value->xml: reference cycle detected at id {:?} (xml has no graph)", id.value)));
            }
            let target = *nodes.get(id).ok_or_else(|| err(format!("value->xml: dangling Ref{{id: {:?}}} — not found in `nodes`", id.value)))?;
            let result = resolve(target, nodes, visiting);
            visiting.remove(id);
            result
        }
        other => Ok(other.clone()),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn expect_entries(v: &SemioValue) -> Result<Vec<SemioValueEntry>, store::PackError> {
    match v {
        SemioValue::Map { entries } => Ok(entries.clone()),
        other => Err(err(format!("value->xml: expected a Map, got {other:?}"))),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn find(entries: &[SemioValueEntry], key: &str) -> Option<SemioValue> {
    entries.iter().find(|e| e.key == key).map(|e| e.value.clone())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn expect_str(v: &SemioValue) -> Result<String, store::PackError> {
    match v {
        SemioValue::Str { value } => Ok(value.clone()),
        other => Err(err(format!("value->xml: expected a Str, got {other:?}"))),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn expect_kind(entries: &[SemioValueEntry], nodes: &HashMap<&ValueId, &SemioValue>, visiting: &mut HashSet<ValueId>) -> Result<String, store::PackError> {
    let raw = find(entries, "kind").ok_or_else(|| err("value->xml: missing required \"kind\" entry"))?;
    expect_str(&resolve(&raw, nodes, visiting)?)
}
//#endregion 🔖️Resolve

//#region 🔖️Convert
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_node_from_semio(v: &SemioValue, nodes: &HashMap<&ValueId, &SemioValue>, visiting: &mut HashSet<ValueId>) -> Result<XmlNode, store::PackError> {
    let resolved = resolve(v, nodes, visiting)?;
    let entries = expect_entries(&resolved)?;
    let kind = expect_kind(&entries, nodes, visiting)?;
    match kind.as_str() {
        "element" => {
            let tag_raw = find(&entries, "tag").ok_or_else(|| err("value->xml: element missing \"tag\""))?;
            let name = expect_str(&resolve(&tag_raw, nodes, visiting)?)?;
            let attrs_raw = find(&entries, "attrs").ok_or_else(|| err("value->xml: element missing \"attrs\""))?;
            let attrs_entries = expect_entries(&resolve(&attrs_raw, nodes, visiting)?)?;
            let attrs = attrs_entries.iter().map(|e| Ok(XmlAttr { name: e.key.clone(), value: expect_str(&resolve(&e.value, nodes, visiting)?)? })).collect::<Result<Vec<_>, store::PackError>>()?;
            let children_raw = find(&entries, "children").ok_or_else(|| err("value->xml: element missing \"children\""))?;
            let children_resolved = resolve(&children_raw, nodes, visiting)?;
            let items = match children_resolved {
                SemioValue::List { items } => items,
                other => return Err(err(format!("value->xml: \"children\" must be a List, got {other:?}"))),
            };
            let children = items.iter().map(|c| xml_node_from_semio(c, nodes, visiting)).collect::<Result<Vec<_>, store::PackError>>()?;
            Ok(XmlNode::Element { name, attrs, children })
        }
        "text" => Ok(XmlNode::Text { text: expect_str(&resolve(&find(&entries, "text").ok_or_else(|| err("value->xml: text node missing \"text\""))?, nodes, visiting)?)? }),
        "cdata" => Ok(XmlNode::CData { text: expect_str(&resolve(&find(&entries, "text").ok_or_else(|| err("value->xml: cdata node missing \"text\""))?, nodes, visiting)?)? }),
        "comment" => Ok(XmlNode::Comment { text: expect_str(&resolve(&find(&entries, "text").ok_or_else(|| err("value->xml: comment node missing \"text\""))?, nodes, visiting)?)? }),
        "pi" => {
            let target = expect_str(&resolve(&find(&entries, "target").ok_or_else(|| err("value->xml: pi node missing \"target\""))?, nodes, visiting)?)?;
            let data = expect_str(&resolve(&find(&entries, "data").ok_or_else(|| err("value->xml: pi node missing \"data\""))?, nodes, visiting)?)?;
            Ok(XmlNode::ProcessingInstruction { target, data })
        }
        other => Err(err(format!("value->xml: unknown node kind {other:?}"))),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_declaration_from_semio(v: &SemioValue, nodes: &HashMap<&ValueId, &SemioValue>, visiting: &mut HashSet<ValueId>) -> Result<XmlDeclaration, store::PackError> {
    let entries = expect_entries(v)?;
    let version = expect_str(&resolve(&find(&entries, "version").ok_or_else(|| err("value->xml: declaration missing \"version\""))?, nodes, visiting)?)?;
    let encoding = match find(&entries, "encoding") {
        Some(raw) => match resolve(&raw, nodes, visiting)? {
            SemioValue::Null => None,
            SemioValue::Str { value } => Some(value),
            other => return Err(err(format!("value->xml: declaration \"encoding\" must be Str or Null, got {other:?}"))),
        },
        None => None,
    };
    let standalone = match find(&entries, "standalone") {
        Some(raw) => match resolve(&raw, nodes, visiting)? {
            SemioValue::Null => None,
            SemioValue::Bool { value } => Some(value),
            other => return Err(err(format!("value->xml: declaration \"standalone\" must be Bool or Null, got {other:?}"))),
        },
        None => None,
    };
    Ok(XmlDeclaration { version, encoding, standalone })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_doctype_from_semio(v: &SemioValue, nodes: &HashMap<&ValueId, &SemioValue>, visiting: &mut HashSet<ValueId>) -> Result<XmlDoctype, store::PackError> {
    let entries = expect_entries(v)?;
    let name = expect_str(&resolve(&find(&entries, "name").ok_or_else(|| err("value->xml: doctype missing name"))?, nodes, visiting)?)?;
    let external_id = match find(&entries, "externalId") {
        None => None,
        Some(value) => match resolve(&value, nodes, visiting)? {
            SemioValue::Null => None,
            SemioValue::Map { entries } => {
                let kind = expect_str(&find(&entries, "kind").ok_or_else(|| err("value->xml: externalId missing kind"))?)?;
                let system_id = expect_str(&find(&entries, "systemId").ok_or_else(|| err("value->xml: externalId missing systemId"))?)?;
                Some(match kind.as_str() {
                    "system" => XmlExternalId::System { system_id },
                    "public" => XmlExternalId::Public { public_id: expect_str(&find(&entries, "publicId").ok_or_else(|| err("value->xml: public externalId missing publicId"))?)?, system_id },
                    _ => return Err(err(format!("value->xml: unknown externalId kind {kind}"))),
                })
            }
            other => return Err(err(format!("value->xml: externalId must be Map or Null, got {other:?}"))),
        },
    };
    let declarations = match find(&entries, "declarations") {
        None => Vec::new(),
        Some(SemioValue::List { items }) => items
            .into_iter()
            .map(|item| {
                let fields = expect_entries(&item)?;
                let kind = expect_str(&find(&fields, "kind").ok_or_else(|| err("value->xml: DTD declaration missing kind"))?)?;
                if kind != "entity" {
                    return Err(err(format!("value->xml: unsupported DTD declaration kind {kind}")));
                }
                let parameter = match find(&fields, "parameter") {
                    Some(SemioValue::Bool { value }) => value,
                    _ => false,
                };
                Ok(XmlDtdDeclaration::Entity {
                    parameter,
                    name: expect_str(&find(&fields, "name").ok_or_else(|| err("value->xml: entity missing name"))?)?,
                    value: expect_str(&find(&fields, "value").ok_or_else(|| err("value->xml: entity missing value"))?)?,
                })
            })
            .collect::<Result<Vec<_>, store::PackError>>()?,
        Some(other) => return Err(err(format!("value->xml: declarations must be List, got {other:?}"))),
    };
    Ok(XmlDoctype { name, external_id, declarations })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn xml_document_from_semio(v: &SemioValue, nodes: &HashMap<&ValueId, &SemioValue>, visiting: &mut HashSet<ValueId>) -> Result<XmlDocument, store::PackError> {
    let resolved = resolve(v, nodes, visiting)?;
    let entries = expect_entries(&resolved)?;
    let kind = expect_kind(&entries, nodes, visiting)?;
    if kind != "document" {
        return Err(err(format!("value->xml: expected kind \"document\" at the snapshot root, got {kind:?}")));
    }
    let declaration = match find(&entries, "declaration") {
        Some(raw) => match resolve(&raw, nodes, visiting)? {
            SemioValue::Null => None,
            other @ SemioValue::Map { .. } => Some(xml_declaration_from_semio(&other, nodes, visiting)?),
            other => return Err(err(format!("value->xml: \"declaration\" must be Map or Null, got {other:?}"))),
        },
        None => None,
    };
    let doctype = match find(&entries, "doctype") {
        Some(raw) => match resolve(&raw, nodes, visiting)? {
            SemioValue::Null => None,
            other @ SemioValue::Map { .. } => Some(xml_doctype_from_semio(&other, nodes, visiting)?),
            other => return Err(err(format!("value->xml: \"doctype\" must be Map or Null, got {other:?}"))),
        },
        None => None,
    };
    let root = match find(&entries, "root") {
        Some(raw) => match resolve(&raw, nodes, visiting)? {
            SemioValue::Null => None,
            other => Some(xml_node_from_semio(&other, nodes, visiting)?),
        },
        None => None,
    };
    let prolog = match find(&entries, "prolog") {
        Some(raw) => match resolve(&raw, nodes, visiting)? {
            SemioValue::List { items } => items.iter().map(|node| xml_node_from_semio(node, nodes, visiting)).collect::<Result<Vec<_>, store::PackError>>()?,
            other => return Err(err(format!("value->xml: \"prolog\" must be a List, got {other:?}"))),
        },
        None => Vec::new(),
    };
    Ok(XmlDocument { root, doctype, declaration, prolog })
}
//#endregion 🔖️Convert

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
