//! 🧬️ Rewrite artifact schema — every field of the artifact with its state class.

use crate::artifacts::jack::{Camera, PropertyValue};
use crate::artifacts::rewrite::LayoutPoint;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Artifact
/// 🧬️ Full rewrite artifact state across persistent, shared-ui and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.trinity.rewrite")]
pub struct RewriteArtifact {
    #[state(persistent)] pub before_fixture_json: String,
    #[state(persistent)] pub lhs_json: String,
    #[state(persistent)] pub rhs_json: String,
    #[state(persistent)] pub parameter_bindings: BTreeMap<String, PropertyValue>,
    #[state(persistent)] pub rule_layout: BTreeMap<String, LayoutPoint>,
    #[state(shared_ui)] pub selected_node_ids: Vec<String>,
    #[state(shared_ui)] pub active_hover_var: String,
    #[state(shared_ui)] pub active_select_var: String,
    #[state(shared_ui)] pub lod_mode_by_window: BTreeMap<String, String>,
    #[state(local_ui)] pub before_pane_camera: Camera,
    #[state(local_ui)] pub reorganize_epoch: u64,
    #[state(local_ui)] pub hover_epoch: u64,
    #[state(local_ui)] pub select_epoch: u64,
    #[state(local_ui)] pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for RewriteArtifact {
    fn default() -> Self {
        Self {
            before_fixture_json: String::new(),
            lhs_json: String::new(),
            rhs_json: String::new(),
            parameter_bindings: BTreeMap::new(),
            rule_layout: BTreeMap::new(),
            selected_node_ids: Vec::new(),
            active_hover_var: String::new(),
            active_select_var: String::new(),
            lod_mode_by_window: BTreeMap::new(),
            before_pane_camera: Camera::default(),
            reorganize_epoch: 0,
            hover_epoch: 0,
            select_epoch: 0,
            locale: "en-US".into(),
        }
    }
}

impl RewriteArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::rewrite::RewriteSnapshot {
        crate::artifacts::rewrite::RewriteSnapshot {
            before_fixture_json: self.before_fixture_json.clone(),
            lhs_json: self.lhs_json.clone(),
            rhs_json: self.rhs_json.clone(),
            parameter_bindings: self.parameter_bindings.clone(),
            rule_layout: self.rule_layout.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::rewrite::RewriteSnapshot) -> Self {
        Self {
            before_fixture_json: snapshot.before_fixture_json,
            lhs_json: snapshot.lhs_json,
            rhs_json: snapshot.rhs_json,
            parameter_bindings: snapshot.parameter_bindings,
            rule_layout: snapshot.rule_layout,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::rewrite::RewriteSnapshot) {
        self.before_fixture_json = snapshot.before_fixture_json;
        self.lhs_json = snapshot.lhs_json;
        self.rhs_json = snapshot.rhs_json;
        self.parameter_bindings = snapshot.parameter_bindings;
        self.rule_layout = snapshot.rule_layout;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.trinity.rewrite` — fifteen handcrafted schema leaves.
pub fn rewrite_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.trinity.rewrite",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("../📸️snapshot/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../📸️snapshot/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../📸️snapshot/🧬️schema/🔣️component.json"),
            proto: include_str!("../📸️snapshot/🧬️schema/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("../🔺️diff/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../🔺️diff/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../🔺️diff/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../🔺️diff/🧬️schema/🔣️component.json"),
            proto: include_str!("../🔺️diff/🧬️schema/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
