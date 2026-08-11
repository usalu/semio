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

use crate::artifacts::bcf::BcfSnapshot;
use crate::artifacts::bcf::schema::snapshot::{BcfComment, BcfComponents, BcfTopic, BcfViewpoint, BcfVisibility};
use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::{ElementClass, PsetValue, RelationKind, SemioModelSnapshot};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

//#region 🔖️Serializer
pub struct SemioModelToBcf;

impl ArtifactSerializer for SemioModelToBcf {
    type From = SemioModelSnapshot;
    type Into = BcfSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("model") };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.bcf", standard: StandardId("2.1"), subset: SubsetId::ANY };

    fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        Ok(bcf_from_model(from))
    }
}

pub fn register() {}
//#endregion 🔖️Serializer

//#region 🔖️Convert
fn text_property<'a>(properties: &'a [crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::Property], key: &str) -> Option<&'a str> {
    properties.iter().find(|p| p.key == key).and_then(|p| match &p.value {
        PsetValue::Text { value } => Some(value.as_str()),
        _ => None,
    })
}

fn number_property(properties: &[crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::Property], key: &str) -> Option<f64> {
    properties.iter().find(|p| p.key == key).and_then(|p| match &p.value {
        PsetValue::Number { value } => Some(*value),
        _ => None,
    })
}

fn topic_from_element(element: &crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelElement, referenced_guids: &[String]) -> BcfTopic {
    let empty: Vec<crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::Property> = Vec::new();
    let topic_props = element.psets.iter().find(|p| p.name == "Pset_BcfTopic").map(|p| p.properties.as_slice()).unwrap_or(&empty);
    let comments_props = element.psets.iter().find(|p| p.name == "Pset_BcfComments").map(|p| p.properties.as_slice()).unwrap_or(&empty);

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
        vec![BcfViewpoint {
            guid: format!("vp-{}", element.id),
            camera: None,
            components: Some(BcfComponents { selection: referenced_guids.to_vec(), visibility: BcfVisibility::default(), coloring: vec![] }),
            snapshot: None,
        }]
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
pub fn bcf_from_model(from: &SemioModelSnapshot) -> BcfSnapshot {
    let topics = from
        .elements
        .iter()
        .filter(|e| e.class == ElementClass::Other { name: "BcfTopic".into() })
        .map(|element| {
            let referenced: Vec<String> = from
                .relations
                .iter()
                .filter(|r| r.from == element.id && r.kind == RelationKind::Other { label: "BcfReferences".into() })
                .map(|r| r.to.clone())
                .collect();
            topic_from_element(element, &referenced)
        })
        .collect();

    BcfSnapshot { schema: crate::artifacts::bcf::STDIO_BCF_DOCUMENT_SCHEMA.into(), version: "2.1".into(), topics, parts: Vec::new() }
}
//#endregion 🔖️Entry

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::model::io::import::deserializers::artifacts::bcf::v2_1::any::model_from_bcf;
    use crate::artifacts::bcf::schema::snapshot::{BcfComponents as BcfComponentsT, BcfTopic as BcfTopicT, BcfViewpoint as BcfViewpointT, BcfVisibility as BcfVisibilityT, BcfComment as BcfCommentT};

    fn fixture() -> BcfSnapshot {
        BcfSnapshot {
            schema: crate::artifacts::bcf::STDIO_BCF_DOCUMENT_SCHEMA.into(),
            version: "2.1".into(),
            topics: vec![BcfTopicT {
                guid: "topic-1".into(),
                title: "Clash between wall and duct".into(),
                description: "Duct penetrates load-bearing wall".into(),
                status: "Open".into(),
                priority: "High".into(),
                labels: vec!["clash".into(), "mep".into()],
                creation_date: "2026-08-10T12:00:00Z".into(),
                creation_author: "ueli@iek.uni-hannover.de".into(),
                comments: vec![BcfCommentT { guid: "comment-1".into(), date: "2026-08-10T12:05:00Z".into(), author: "ueli@iek.uni-hannover.de".into(), text: "Please reroute the duct.".into(), viewpoint_ref: Some("vp-1".into()) }],
                viewpoints: vec![BcfViewpointT {
                    guid: "vp-1".into(),
                    camera: None,
                    components: Some(BcfComponentsT { selection: vec!["wall-guid".into(), "duct-guid".into()], visibility: BcfVisibilityT::default(), coloring: vec![] }),
                    snapshot: None,
                }],
            }],
            parts: vec![],
        }
    }

    /// 🧪️ Required proof: bcf -> model -> bcf -> model round trip preserves everything `model`
    /// can represent (topic metadata, comments, referenced-guid relations, stub elements).
    #[test]
    fn bcf_to_model_to_bcf_to_model_round_trips() {
        let s1 = model_from_bcf(&fixture());
        let bcf_x = bcf_from_model(&s1);
        let s2 = model_from_bcf(&bcf_x);
        assert_eq!(s1, s2, "model-level round trip through the reconstructed BCF must be exact");
    }

    #[test]
    fn non_topic_elements_and_spatial_are_dropped_not_forced() {
        let mut s1 = model_from_bcf(&fixture());
        // hand-add content BCF cannot represent
        s1.spatial.push(crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SpatialNode {
            id: "site-1".into(),
            kind: crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SpatialKind::Site,
            name: "Unrepresentable Site".into(),
            parent_id: None,
            placement: crate::artifacts::semio::standards::v1::engine::geometry::SemioTransform::identity(),
        });
        let bcf_x = bcf_from_model(&s1);
        assert_eq!(bcf_x.topics.len(), 1, "only the BcfTopic-classed element becomes a topic");
        assert_eq!(bcf_x.version, "2.1");
        assert!(bcf_x.parts.is_empty());
    }
}
//#endregion 🧪️Tests
