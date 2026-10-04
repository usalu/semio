//! 🧬️ DAG diff schema — sparse field delta over the artifact.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `nodes: Option<DagNodesDelta>` /
//! `edges: Option<DagEdgesDelta>` / `set_nodes` / `set_edges` are all gone — the composed child is
//! opaque (a parent's diff never embeds a child diff, per `📓️design-full-plan.md` §1's CHILD/LINK
//! split); content edits are child-lane leaves (design §20.15), so the parent diff only ever swaps the handle. Single
//! `Option<DagContentChild>` — the slot is never absent, only ever replaced, matching writer's
//! `document`/flow's `content` field shape, not lowpoly's optional-slot `Option<Option<_>>`.
//!
//! `artifact: Option<Box<DagArtifact>>` (a whole-artifact-replace escape hatch) is also gone — it was
//! already dead (never constructed anywhere; `DagPlayApp` never overrides `whole_document_operation`)
//! and is exactly the forbidden whole-document-replace-via-diff shape `📌️important.md`'s vocabulary
//! policy bans. `DagNodesDelta`/`DagEdgesDelta`/`DagNodePatchEntry`/`DagNodeExtraPatch*`/
//! `DagEdgePatchEntry`/`DagNodeSpecList`/`DagHostSnapshotEdgeList` are all dead with it — confirmed zero
//! remaining references after this pass.

use crate::{DagContentChild, DagSnapshot};
use crate::schema::DagArtifact;
use protocol::MutationDiff;
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the DAG artifact.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.dag.dag")]
pub struct DagDiff {
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub content: Option<DagContentChild>,
}

impl semio_framework_value::FromValue for DagDiff {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let mut schema = None;
        let mut content = None;
        for (key, value) in semio_framework_value::DslValue::into_object(value)? {
            match key.as_str() {
                "schema" if schema.is_none() => schema = Some(semio_framework_value::FromValue::from_value(value)?),
                "content" if content.is_none() => content = Some(semio_framework_value::FromValue::from_value(value)?),
                _ => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown or duplicate Dag field {key}"))),
            }
        }
        let result = Self { schema, content };
        result.validate().map_err(|message| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message))?;
        Ok(result)
    }
}

impl DagDiff {
    /// 🪆️ Enforces the document marker and exact owned-child coordinates.
    pub fn validate(&self) -> Result<(), String> {
        use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::child::validate_semio_child_identity;
        if self.schema.as_deref().is_some_and(|value| value != "dag.dag") { return Err("invalid Dag document marker".into()); }
        if let Some(child) = &self.content { validate_semio_child_identity(&child.child_id, &child.target, "graph")?; }
        Ok(())
    }
}

//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct DagStringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers


//#region 🔖️Apply
impl DagDiff {
    /// 🧬️ Applies sparse document fields onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &DagArtifact) -> protocol::MutationApplyResult<DagArtifact> {
        self.validate().map_err(|message| protocol::MutationApplyError { code: "mutation.apply.child-identity".into(), message, target: Vec::new() })?;
        Ok({
            let mut next = artifact.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(content) = &self.content {
                next.content = content.clone();
            }
            next
        })
    }
}

impl MutationDiff<DagSnapshot> for DagDiff {
    fn apply(&self, snapshot: &DagSnapshot) -> protocol::MutationApplyResult<DagSnapshot> {
        self.validate().map_err(|message| protocol::MutationApplyError { code: "mutation.apply.child-identity".into(), message, target: Vec::new() })?;
        Ok({
            let mut next = snapshot.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(content) = &self.content {
                next.content = content.clone();
            }
            next
        })
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
        take!(content);
    }
}
//#endregion 🔖️Apply


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
