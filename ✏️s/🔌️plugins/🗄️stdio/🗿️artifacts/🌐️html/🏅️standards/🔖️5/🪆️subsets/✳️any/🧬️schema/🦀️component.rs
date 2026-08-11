//! 🧬️ HtmlArtifact schema — full artifact state, mirrors `HtmlSnapshot` field for field (see
//! svg's `SvgArtifact` for the precedent this follows).

use crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::{HtmlNode, HtmlSnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.html")]
pub struct HtmlArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doctype: Option<String>,
    #[state(persistent)]
    pub root: HtmlNode,
}

impl Default for HtmlArtifact {
    fn default() -> Self { Self::from_snapshot(HtmlSnapshot::default()) }
}

impl HtmlArtifact {
    pub fn to_snapshot(&self) -> HtmlSnapshot {
        HtmlSnapshot {
            schema: self.schema.clone(),
            doctype: self.doctype.clone(),
            root: self.root.clone(),
        }
    }
    pub fn from_snapshot(snapshot: HtmlSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            doctype: snapshot.doctype,
            root: snapshot.root,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: HtmlSnapshot) {
        self.schema = snapshot.schema;
        self.doctype = snapshot.doctype;
        self.root = snapshot.root;
    }
}

pub fn html_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.html",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
