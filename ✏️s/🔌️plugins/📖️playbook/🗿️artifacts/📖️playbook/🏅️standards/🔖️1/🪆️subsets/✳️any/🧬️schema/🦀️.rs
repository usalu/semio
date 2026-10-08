//! 🧬️ Playbook artifact schema — every field of the artifact with its state class.

use crate::{PlaybookFlowChild, PLAYBOOK_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ playbook document artifact state.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[artifact_schema(id = "s.playbook.playbook")]
pub struct PlaybookArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    pub version: String,
    #[state(artifact)]
    pub title: Option<String>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub flow: PlaybookFlowChild,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for PlaybookArtifact {
    fn default() -> Self {
        let snapshot = crate::PlaybookSnapshot::default();
        Self { schema: PLAYBOOK_DOCUMENT_SCHEMA.into(), id: "playbook".into(), version: "1".into(), title: None, flow: snapshot.flow }
    }
}

impl PlaybookArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::PlaybookSnapshot {
        crate::PlaybookSnapshot { schema: self.schema.clone(), id: self.id.clone(), version: self.version.clone(), title: self.title.clone(), flow: self.flow.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: crate::PlaybookSnapshot) -> Self {
        Self { schema: snapshot.schema, id: snapshot.id, version: snapshot.version, title: snapshot.title, flow: snapshot.flow }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::PlaybookSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.version = snapshot.version;
        self.title = snapshot.title;
        self.flow = snapshot.flow;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️ValueCodec
/// 🔀️ Hand-written, not derived: `flow` is a `store::ArtifactChild<S>` composed-artifact
/// handle, which speak `serde` (framework-internal, unaffected by this ticket) rather than
/// `ToValue`/`FromValue` directly — bridged per-field through the pre-existing
/// `to_dsl_value`/`from_dsl_value` seam (`🌱️value/🔀️serde`) instead of widening the derive macro to
/// understand child-slot handles. See the fan-out playbook's "composed artifact fields" trap.
impl semio_framework_value::ToValue for PlaybookArtifact {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object([
            ("schema".to_string(), semio_framework_value::ToValue::to_value(&self.schema)),
            ("id".to_string(), semio_framework_value::ToValue::to_value(&self.id)),
            ("version".to_string(), semio_framework_value::ToValue::to_value(&self.version)),
            ("title".to_string(), semio_framework_value::ToValue::to_value(&self.title)),
            ("flow".to_string(), semio_framework_value::ToValue::to_value(&self.flow)),
        ])
    }
}
impl semio_framework_value::FromValue for PlaybookArtifact {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("missing field `{key}`")));
        Ok(Self {
            schema: semio_framework_value::FromValue::from_value(field("schema")?)?,
            id: semio_framework_value::FromValue::from_value(field("id")?)?,
            version: semio_framework_value::FromValue::from_value(field("version")?)?,
            title: semio_framework_value::FromValue::from_value(field("title")?)?,
            flow: semio_framework_value::FromValue::from_value(field("flow")?)?,
        })
    }
}
//#endregion 🔖️ValueCodec

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.playbook.playbook` — twenty handcrafted schema leaves.
pub fn playbook_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.playbook.playbook",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🔖️DocumentHelpers
/// 🧱️ A blank block of the requested kind — every optional field defaulted, ready to be edited.
/// Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES)
/// — pure over `PlaybookBlock`, no app-runtime parameter.
pub fn default_block(id: String, kind: &str) -> crate::PlaybookBlock {
    crate::PlaybookBlock {
        id,
        label: kind.into(),
        kind: kind.into(),
        description: None,
        required: None,
        placeholder: None,
        default: None,
        min: None,
        max: None,
        step: None,
        unit: None,
        text: None,
        options: None,
        fields: None,
        schema: None,
        src: None,
        accept: None,
        example_id: None,
        params: None,
        condition: None,
    }
}

#[cfg(test)]
#[path = "🧪️tests/🧱️block-defaults/🦀️.rs"]
mod block_defaults_tests;
//#endregion 🔖️BlockDefaults

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
