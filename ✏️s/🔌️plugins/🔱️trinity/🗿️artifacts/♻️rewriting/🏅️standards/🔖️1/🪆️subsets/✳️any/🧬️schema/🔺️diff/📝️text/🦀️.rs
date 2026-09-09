//! 🔺️ Rewriting artifact — sparse field-delta diff codec and apply/absorb.

use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::RewritingArtifact;
use crate::RewritingSnapshot;
use protocol::MutationDiff;
use replication::MapDelta;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Apply
impl RewritingDiff {
    /// 🧬️ Applies document-owned sparse entries onto the artifact.
    pub fn apply_to_artifact(&self, artifact: &RewritingArtifact) -> protocol::MutationApplyResult<RewritingArtifact> {
        Ok({
            let mut next = artifact.clone();
            if let Some(value) = &self.before_fixture_json {
                next.before_fixture_json = value.clone();
            }
            if let Some(value) = &self.lhs_json {
                next.lhs_json = value.clone();
            }
            if let Some(value) = &self.rhs_json {
                next.rhs_json = value.clone();
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
            if let Some(value) = &self.before_fixture_json {
                next.before_fixture_json = value.clone();
            }
            if let Some(value) = &self.lhs_json {
                next.lhs_json = value.clone();
            }
            if let Some(value) = &self.rhs_json {
                next.rhs_json = value.clone();
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
        take!(before_fixture_json);
        take!(lhs_json);
        take!(rhs_json);
        MapDelta::absorb_optional(&mut self.parameter_bindings, other.parameter_bindings);
        MapDelta::absorb_optional(&mut self.rule_layout, other.rule_layout);
    }
}
//#endregion 🔖️Apply

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type RewritingDiffText = String;
//#endregion 🚚️Carrier
