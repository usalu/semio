//! 🧬️ Playbook diff schema — sparse field delta over the artifact.
//!
//! The steps are edited on the `flow` child's own lane (design §20.15 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING), so this
//! delta carries the parent's scalars and the single-Option whole-handle-replace `flow` coordinate — the slot is never absent,
//! only ever replaced (not `Option<Option<…>>`, the shape for a slot whose PRESENCE itself can change, e.g. lowpoly's `mesh`).

use crate::PlaybookFlowChild;
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the playbook artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[artifact_schema(id = "s.playbook.playbook")]
pub struct PlaybookDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub id: Option<String>,
    #[state(artifact)]
    pub version: Option<String>,
    #[state(artifact)]
    pub title: Option<Option<String>>,
    #[state(artifact)]
    pub flow: Option<PlaybookFlowChild>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct PlaybookStringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️ValueCodec
/// 🔀️ Hand-written, not derived: `flow` is a `store::ArtifactChild<S>` composed-artifact
/// handle bridged through `to_dsl_value`/`from_dsl_value` (see the sibling `🧬️schema/🦀️component.rs`
/// impl for [`crate::schema::PlaybookArtifact`] — same trap, same fix). This is
/// [`crate::mutation::MutationDiff::Diff`]'s own wire shape, so it must implement `ToValue`/
/// `FromValue`, not just the domain types it composes.
impl semio_framework_value::ToValue for PlaybookDiff {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object([
            ("schema".to_string(), semio_framework_value::ToValue::to_value(&self.schema)),
            ("id".to_string(), semio_framework_value::ToValue::to_value(&self.id)),
            ("version".to_string(), semio_framework_value::ToValue::to_value(&self.version)),
            ("title".to_string(), semio_framework_value::ToValue::to_value(&self.title)),
            ("flow".to_string(), self.flow.as_ref().map_or(semio_framework_value::DslValue::Null, |flow| semio_framework_value::ToValue::to_value(flow))),
        ])
    }
}
impl semio_framework_value::FromValue for PlaybookDiff {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let flow_child = |key: &str| -> Result<Option<PlaybookFlowChild>, semio_framework_value::ValueError> {
            match get(key) {
                None | Some(semio_framework_value::DslValue::Null) => Ok(None),
                Some(value) => semio_framework_value::FromValue::from_value(value).map(Some),
            }
        };
        Ok(Self {
            schema: get("schema").map_or(Ok(None), semio_framework_value::FromValue::from_value)?,
            id: get("id").map_or(Ok(None), semio_framework_value::FromValue::from_value)?,
            version: get("version").map_or(Ok(None), semio_framework_value::FromValue::from_value)?,
            title: get("title").map_or(Ok(None), semio_framework_value::FromValue::from_value)?,
            flow: flow_child("flow")?,
        })
    }
}
//#endregion 🔖️ValueCodec

use crate::schema::snapshot::PlaybookSnapshot;
use protocol::MutationDiff;

impl MutationDiff<PlaybookSnapshot> for PlaybookDiff {
    fn apply(&self, snapshot: &PlaybookSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<PlaybookSnapshot> {
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(id) = &self.id {
            next.id = id.clone();
        }
        if let Some(version) = &self.version {
            next.version = version.clone();
        }
        if let Some(title) = &self.title {
            next.title = title.clone();
        }
        if let Some(flow) = &self.flow {
            next.flow = flow.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(id);
        take!(version);
        take!(title);
        take!(flow);
    }
}

impl protocol::DiffAlgebra<PlaybookSnapshot> for PlaybookDiff {
    fn inverse(&self, base: &PlaybookSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            id: self.id.as_ref().map(|_| base.id.clone()),
            version: self.version.as_ref().map(|_| base.version.clone()),
            title: self.title.as_ref().map(|_| base.title.clone()),
            flow: self.flow.as_ref().map(|_| base.flow.clone()),
        }
    }
    fn between(base: &PlaybookSnapshot, other: &PlaybookSnapshot) -> Self {
        Self {
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            id: (base.id != other.id).then(|| other.id.clone()),
            version: (base.version != other.version).then(|| other.version.clone()),
            title: (base.title != other.title).then(|| other.title.clone()),
            flow: (base.flow != other.flow).then(|| other.flow.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.id.is_none() && self.version.is_none() && self.title.is_none() && self.flow.is_none()
    }
}
