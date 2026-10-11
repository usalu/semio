//! 🧬️ Playbook snapshot schema — artifact-lane fields only.
//!
//! The steps live in the composed `flow` child (see the artifact root's `🔖️ContentBridge`), never on this struct: the parent
//! carries the child's coordinate, its text and pack carry nothing of the child's content, and every reader composes parent +
//! child on read (design §20.15 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).

use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted playbook document snapshot (persistent fields of the artifact). `#[child(...)]`
/// drives `#[derive(ArtifactSchema, semio_framework_value::RetireOwned)]`'s slot-table emission; never hand-written.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[artifact_schema(id = "s.playbook.playbook")]
pub struct PlaybookSnapshot {
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
    pub flow: crate::PlaybookFlowChild,
}

impl Default for PlaybookSnapshot {
    fn default() -> Self {
        let kernel = crate::playbook::empty_playbook_snapshot();
        Self { schema: kernel.schema, id: kernel.id, version: kernel.version, title: kernel.title, flow: crate::playbook_flow_child(crate::PLAYBOOK_GENESIS_FLOW_ID) }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️ValueCodec
/// 🔀️ Hand-written, not derived — mirrors the sibling `PlaybookArtifact` impl one region up
/// (`../🦀️.rs`'s `🔖️ValueCodec`): `flow` is a `store::ArtifactChild<S>` composed-artifact handle,
/// bridged through the pre-existing `to_dsl_value`/`from_dsl_value` seam instead of widening the
/// derive macro to understand child-slot handles.
impl semio_framework_value::ToValue for PlaybookSnapshot {
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
impl semio_framework_value::FromValue for PlaybookSnapshot {
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

//#region 🔖️PackRecord



//#endregion 🔖️PackRecord

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️ExternalBridges



//#endregion 🔖️ExternalBridges



