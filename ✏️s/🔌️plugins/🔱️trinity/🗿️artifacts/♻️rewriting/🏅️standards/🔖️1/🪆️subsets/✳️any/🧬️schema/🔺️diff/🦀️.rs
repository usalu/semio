//! 🧬️ Rewriting diff schema — sparse field delta over the artifact.

use semio_framework_graph::manifest::PropertyValue;

use crate::LayoutPoint;
use ::semio_framework_schema::ArtifactSchema;
use replication::MapDelta;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the rewriting artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.trinity.rewriting")]
pub struct RewritingDiff {
    #[state(artifact)]
    pub working_graph: Option<semio_s_artifact_trinity_jack::JackSnapshot>,
    #[state(artifact)]
    pub lhs: Option<crate::standards::v1::subsets::any::schema::Lhs>,
    #[state(artifact)]
    pub rhs: Option<crate::standards::v1::subsets::any::schema::Rhs>,
    #[state(artifact)]
    pub parameter_bindings: Option<MapDelta<PropertyValue>>,
    #[state(artifact)]
    pub rule_layout: Option<MapDelta<LayoutPoint>>,
}
//#endregion 🔖️Diff

#[cfg(test)]
#[path = "🧪️tests/🗂️map-ownership/🦀️.rs"]
mod map_ownership_tests;

use crate::standards::v1::subsets::any::schema::RewritingArtifact;
use crate::RewritingSnapshot;
use protocol::MutationDiff;
use super::*;

impl RewritingDiff {
    /// 🧬️ Applies document-owned sparse entries onto the artifact.
    pub fn apply_to_artifact(&self, artifact: &RewritingArtifact) -> protocol::MutationApplyResult<RewritingArtifact> {
        Ok({
            let mut next = artifact.clone();
            if let Some(value) = &self.working_graph {
                next.working_graph = value.clone();
            }
            if let Some(value) = &self.lhs {
                next.lhs = value.clone();
            }
            if let Some(value) = &self.rhs {
                next.rhs = value.clone();
            }
            if let Some(bindings) = &self.parameter_bindings {
                bindings.apply_to(&mut next.parameter_bindings).map_err(|error| error.under(["parameterBindings"]))?;
            }
            if let Some(layout) = &self.rule_layout {
                layout.apply_to(&mut next.rule_layout).map_err(|error| error.under(["ruleLayout"]))?;
            }
            next
        })
    }
}

impl MutationDiff<RewritingSnapshot> for RewritingDiff {
    fn apply(&self, snapshot: &RewritingSnapshot) -> protocol::MutationApplyResult<RewritingSnapshot> {
        Ok({
            let mut next = snapshot.clone();
            if let Some(value) = &self.working_graph {
                next.working_graph = value.clone();
            }
            if let Some(value) = &self.lhs {
                next.lhs = value.clone();
            }
            if let Some(value) = &self.rhs {
                next.rhs = value.clone();
            }
            if let Some(bindings) = &self.parameter_bindings {
                bindings.apply_to(&mut next.parameter_bindings).map_err(|error| error.under(["parameterBindings"]))?;
            }
            if let Some(layout) = &self.rule_layout {
                layout.apply_to(&mut next.rule_layout).map_err(|error| error.under(["ruleLayout"]))?;
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
        take!(working_graph);
        take!(lhs);
        take!(rhs);
        MapDelta::absorb_optional(&mut self.parameter_bindings, other.parameter_bindings);
        MapDelta::absorb_optional(&mut self.rule_layout, other.rule_layout);
    }
}
