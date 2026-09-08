//! 📤️ `SemioModelToBcf` — mirror of `SemioModelFromBcf`. Reconstructs one `BcfTopic` per
//! `model` element of class `Other{"BcfTopic"}` from its `Pset_BcfTopic`/`Pset_BcfComments`
//! property sets, and ONE synthesized `BcfViewpoint` per topic (guid `vp-<topic-guid>`, no camera,
//! `components.selection` = every `to`-guid of that topic's outgoing `Other{"BcfReferences"}`
//! relations, in relation order) — see the deserializer's own doc comment for the full mapping and
//! its documented gaps.
//!
//! This bridge is intentionally NARROW: it only round-trips BCF-SHAPED `model` content.
//! `model.spatial` and any element that is NOT `class: Other{"BcfTopic"}` (real geometric
//! elements, stub `Other{"BcfReferencedComponent"}` placeholders, anything else) are silently
//! dropped here — BCF has no representation for a spatial tree or a freestanding geometric
//! element, only topic-scoped guid REFERENCES to elements defined elsewhere (typically an IFC
//! file). A relation of any kind other than `Other{"BcfReferences"}` has no BCF counterpart either
//! and is dropped. `version` is always emitted as the fixed literal `"2.1"` (never captured on
//! decode, so there is nothing to round-trip it from); `parts` is always empty.

use semio_s_artifact_stdio_bcf::schema::snapshot::{BcfComment, BcfComponents, BcfTopic, BcfViewpoint, BcfVisibility};
use semio_s_artifact_stdio_bcf::BcfSnapshot;
use crate::standards::v1::subsets::model::schema::snapshot::{ElementClass, PsetValue, RelationKind, SemioModelSnapshot};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

//#region 🔖️Serializer
pub struct SemioModelToBcf;

impl ArtifactSerializer for SemioModelToBcf {
    type From = SemioModelSnapshot;
    type Into = BcfSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("model") };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.bcf", standard: StandardId("2.1"), subset: SubsetId::ANY };

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        Ok(bcf_from_model(from))
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
//#endregion 🔖️Serializer

//#region 🔖️Convert
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn text_property<'a>(properties: &'a [crate::standards::v1::subsets::model::schema::snapshot::Property], key: &str) -> Option<&'a str> {
    properties.iter().find(|p| p.key == key).and_then(|p| match &p.value {
        PsetValue::Text { value } => Some(value.as_str()),
        _ => None,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn number_property(properties: &[crate::standards::v1::subsets::model::schema::snapshot::Property], key: &str) -> Option<f64> {
    properties.iter().find(|p| p.key == key).and_then(|p| match &p.value {
        PsetValue::Number { value } => Some(*value),
        _ => None,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn topic_from_element(element: &crate::standards::v1::subsets::model::schema::snapshot::SemioModelElement, referenced_guids: &[String]) -> BcfTopic {
    let empty: Vec<crate::standards::v1::subsets::model::schema::snapshot::Property> = Vec::new();
    let topic_props = element.psets.iter().find(|p| p.name == "Pset_BcfTopic").map_or(empty.as_slice(), |p| p.properties.as_slice());
    let comments_props = element.psets.iter().find(|p| p.name == "Pset_BcfComments").map_or(empty.as_slice(), |p| p.properties.as_slice());

    let count = number_property(comments_props, "count").unwrap_or(0.0) as usize;
    let comments = (0..count)
        .map(|i| BcfComment {
            guid: text_property(comments_props, &format!("comment_{i}_guid")).unwrap_or_default().to_string(),
            date: text_property(comments_props, &format!("comment_{i}_date")).unwrap_or_default().to_string(),
            author: text_property(comments_props, &format!("comment_{i}_author")).unwrap_or_default().to_string(),
            text: text_property(comments_props, &format!("comment_{i}_text")).unwrap_or_default().to_string(),
            viewpoint_ref: text_property(comments_props, &format!("comment_{i}_viewpointRef")).map(str::to_string),
        })
        .collect();

    let viewpoints = if referenced_guids.is_empty() {
        Vec::new()
    } else {
        vec![BcfViewpoint { guid: format!("vp-{}", element.id), camera: None, components: Some(BcfComponents { selection: referenced_guids.to_vec(), visibility: BcfVisibility::default(), coloring: vec![] }), snapshot: None }]
    };

    BcfTopic {
        guid: element.id.clone(),
        title: text_property(topic_props, "title").unwrap_or_default().to_string(),
        description: text_property(topic_props, "description").unwrap_or_default().to_string(),
        status: text_property(topic_props, "status").unwrap_or_default().to_string(),
        priority: text_property(topic_props, "priority").unwrap_or_default().to_string(),
        labels: text_property(topic_props, "labels").map(|s| s.split('|').filter(|l| !l.is_empty()).map(str::to_string).collect()).unwrap_or_default(),
        creation_date: text_property(topic_props, "creationDate").unwrap_or_default().to_string(),
        creation_author: text_property(topic_props, "creationAuthor").unwrap_or_default().to_string(),
        comments,
        viewpoints,
    }
}
//#endregion 🔖️Convert

//#region 🔖️Entry
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn bcf_from_model(from: &SemioModelSnapshot) -> BcfSnapshot {
    let topics = from
        .elements
        .iter()
        .filter(|e| e.class == ElementClass::Other { name: "BcfTopic".into() })
        .map(|element| {
            let referenced: Vec<String> = from.relations.iter().filter(|r| r.from == element.id && r.kind == RelationKind::Other { label: "BcfReferences".into() }).map(|r| r.to.clone()).collect();
            topic_from_element(element, &referenced)
        })
        .collect();

    BcfSnapshot { schema: semio_s_artifact_stdio_bcf::STDIO_BCF_DOCUMENT_SCHEMA.into(), version: "2.1".into(), topics, parts: Vec::new() }
}
//#endregion 🔖️Entry

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
