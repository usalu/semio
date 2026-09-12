//! 🧬️ DAG diff schema — sparse field delta over the artifact.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `nodes: Option<DagNodesDelta>` /
//! `edges: Option<DagEdgesDelta>` / `set_nodes` / `set_edges` are all gone — the composed child is
//! opaque (a parent's diff never embeds a child diff, per `📓️design-full-plan.md` §1's CHILD/LINK
//! split), so every triad now diffs by minting a whole new `content` handle
//! (`diff_replace_content`, see `🔺️diff/📝️text`) rather than building a structured delta. Single
//! `Option<DagContentChild>` — the slot is never absent, only ever replaced, matching writer's
//! `document`/flow's `content` field shape, not lowpoly's optional-slot `Option<Option<_>>`.
//!
//! `artifact: Option<Box<DagArtifact>>` (a whole-artifact-replace escape hatch) is also gone — it was
//! already dead (never constructed anywhere; `DagPlayApp` never overrides `whole_document_operation`)
//! and is exactly the forbidden whole-document-replace-via-diff shape `📌️important.md`'s vocabulary
//! policy bans. `DagNodesDelta`/`DagEdgesDelta`/`DagNodePatchEntry`/`DagNodeExtraPatch*`/
//! `DagEdgePatchEntry`/`DagNodeSpecList`/`DagFixtureEdgeList` are all dead with it — confirmed zero
//! remaining references after this pass.

use crate::{DagContentChild, DagFixtureEdge, DagNodeSpec, DagSnapshot};
use crate::schema::DagArtifact;
use protocol::MutationDiff;
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the DAG artifact.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, ArtifactSchema)]
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

impl dsl::FromValue for DagDiff {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let mut schema = None;
        let mut content = None;
        for (key, value) in dsl::DslValue::into_object(value)? {
            match key.as_str() {
                "schema" if schema.is_none() => schema = Some(dsl::FromValue::from_value(value)?),
                "content" if content.is_none() => content = Some(dsl::FromValue::from_value(value)?),
                _ => return Err(dsl::ValueError::new(format!("unknown or duplicate Dag field {key}"))),
            }
        }
        let result = Self { schema, content };
        result.validate().map_err(dsl::ValueError::new)?;
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
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct DagStringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️ReplaceContent
/// 🏗️ Every mutation triad's `🔺️diff` builder goes through this: read the current scene off `base`
/// via `crate::dag_working_scene`, apply its own specific semantics to a clone of
/// that scene, then mint+cache a whole new content handle here — the "mint+cache whole handle, never
/// apply-then-capture" pattern flow's `diff_replace_content`/writer's `diff_set_text` established.
pub fn diff_replace_content(nodes: Vec<DagNodeSpec>, edges: Vec<DagFixtureEdge>) -> DagDiff {
    DagDiff { content: Some(crate::dag_content_child_with_owner(nodes, edges)), ..Default::default() }
}
//#endregion 🔖️ReplaceContent

//#region 🔖️Apply
impl DagDiff {
    /// 🧬️ Applies sparse document fields onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &DagArtifact) -> protocol::MutationApplyResult<DagArtifact> {
        self.validate().map_err(|message| protocol::MutationApplyError { code: "mutation.child-identity".into(), message, target: Vec::new() })?;
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
        self.validate().map_err(|message| protocol::MutationApplyError { code: "mutation.child-identity".into(), message, target: Vec::new() })?;
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
