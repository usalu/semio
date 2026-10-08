//! 🧬️ Rewriting diff schema — sparse typed delta over the artifact: field patches for the rule's two sides, a sparse
//! working-graph document diff and keyed map deltas for the parameter bindings and the rule layout.

use semio_framework_graph::manifest::PropertyValue;

use crate::standards::v1::subsets::any::schema::{Assignment, Lhs, ParameterSpec, Pattern, Rhs};
use crate::{LayoutPoint, RewritingSnapshot};
use ::semio_framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationDiff};
use replication::{MapDelta, MapEntryOperation};
use semio_s_artifact_trinity_jack::{JackDiff, JackSnapshot};

//#region 🔖️Diff
/// 🔺️ Sparse typed delta for the rewriting artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.trinity.rewriting")]
pub struct RewritingDiff {
    #[state(artifact)]
    pub working_graph: Option<JackDiff>,
    #[state(artifact)]
    pub lhs: Option<LhsPatch>,
    #[state(artifact)]
    pub rhs: Option<RhsPatch>,
    #[state(artifact)]
    pub parameter_bindings: Option<MapDelta<PropertyValue>>,
    #[state(artifact)]
    pub rule_layout: Option<MapDelta<LayoutPoint>>,
}
//#endregion 🔖️Diff

//#region 🔖️Patches
/// 🕳️ Tri-state decode of every `Option<Option<T>>` patch slot: a missing key is the unchanged slot (`None`) and a PRESENT
/// `null` is the clear `Some(None)`, never the unchanged slot the blanket `Option<T>` decode would fold it into.
fn deserialize_double_option<T: semio_framework_value::FromValue>(value: semio_framework_value::DslValue) -> Result<Option<Option<T>>, semio_framework_value::ValueError> {
    <Option<T> as semio_framework_value::FromValue>::from_value(value).map(Some)
}

/// 🩹 Sparse per-field patch over a rule [`Pattern`] — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct PatternPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub left_var: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub left_kind: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub edge_var: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub edge_kind: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub right_var: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub right_kind: Option<Option<String>>,
}

/// 🩹 Sparse per-field patch over the rule's [`Lhs`] — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct LhsPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<PatternPatch>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub where_clause: Option<Option<String>>,
}

/// 🩹 Sparse per-field patch over the rule's [`Rhs`] — each rewrite-program statement list is one ordered unit, so a
/// list is named only when it differs.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct RhsPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub create: Option<Vec<Pattern>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<Vec<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<Vec<Assignment>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub merge: Option<Vec<Pattern>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterSpec>>,
}

impl PatternPatch {
    pub fn is_empty(&self) -> bool {
        self.left_var.is_none() && self.left_kind.is_none() && self.edge_var.is_none() && self.edge_kind.is_none() && self.right_var.is_none() && self.right_kind.is_none()
    }
    fn write_into(&self, item: &mut Pattern) {
        if let Some(value) = &self.left_var {
            item.left_var = value.clone();
        }
        if let Some(value) = &self.left_kind {
            item.left_kind = value.clone();
        }
        if let Some(value) = &self.edge_var {
            item.edge_var = value.clone();
        }
        if let Some(value) = &self.edge_kind {
            item.edge_kind = value.clone();
        }
        if let Some(value) = &self.right_var {
            item.right_var = value.clone();
        }
        if let Some(value) = &self.right_kind {
            item.right_kind = value.clone();
        }
    }
    fn absorb(&mut self, later: Self) {
        if later.left_var.is_some() {
            self.left_var = later.left_var;
        }
        if later.left_kind.is_some() {
            self.left_kind = later.left_kind;
        }
        if later.edge_var.is_some() {
            self.edge_var = later.edge_var;
        }
        if later.edge_kind.is_some() {
            self.edge_kind = later.edge_kind;
        }
        if later.right_var.is_some() {
            self.right_var = later.right_var;
        }
        if later.right_kind.is_some() {
            self.right_kind = later.right_kind;
        }
    }
    fn inverse(&self, base: &Pattern) -> Self {
        Self {
            left_var: self.left_var.as_ref().map(|_| base.left_var.clone()),
            left_kind: self.left_kind.as_ref().map(|_| base.left_kind.clone()),
            edge_var: self.edge_var.as_ref().map(|_| base.edge_var.clone()),
            edge_kind: self.edge_kind.as_ref().map(|_| base.edge_kind.clone()),
            right_var: self.right_var.as_ref().map(|_| base.right_var.clone()),
            right_kind: self.right_kind.as_ref().map(|_| base.right_kind.clone()),
        }
    }
}

impl LhsPatch {
    pub fn is_empty(&self) -> bool {
        self.pattern.as_ref().is_none_or(PatternPatch::is_empty) && self.where_clause.is_none()
    }
    fn write_into(&self, item: &mut Lhs) {
        if let Some(patch) = &self.pattern {
            patch.write_into(&mut item.pattern);
        }
        if let Some(value) = &self.where_clause {
            item.where_clause = value.clone();
        }
    }
    fn absorb(&mut self, later: Self) {
        match (&mut self.pattern, later.pattern) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        if later.where_clause.is_some() {
            self.where_clause = later.where_clause;
        }
    }
    fn inverse(&self, base: &Lhs) -> Self {
        Self { pattern: self.pattern.as_ref().map(|patch| patch.inverse(&base.pattern)), where_clause: self.where_clause.as_ref().map(|_| base.where_clause.clone()) }
    }
}

impl RhsPatch {
    pub fn is_empty(&self) -> bool {
        self.create.is_none() && self.delete.is_none() && self.set.is_none() && self.merge.is_none() && self.parameters.is_none()
    }
    fn write_into(&self, item: &mut Rhs) {
        if let Some(value) = &self.create {
            item.create = value.clone();
        }
        if let Some(value) = &self.delete {
            item.delete = value.clone();
        }
        if let Some(value) = &self.set {
            item.set = value.clone();
        }
        if let Some(value) = &self.merge {
            item.merge = value.clone();
        }
        if let Some(value) = &self.parameters {
            item.parameters = value.clone();
        }
    }
    fn absorb(&mut self, later: Self) {
        if later.create.is_some() {
            self.create = later.create;
        }
        if later.delete.is_some() {
            self.delete = later.delete;
        }
        if later.set.is_some() {
            self.set = later.set;
        }
        if later.merge.is_some() {
            self.merge = later.merge;
        }
        if later.parameters.is_some() {
            self.parameters = later.parameters;
        }
    }
    fn inverse(&self, base: &Rhs) -> Self {
        Self {
            create: self.create.as_ref().map(|_| base.create.clone()),
            delete: self.delete.as_ref().map(|_| base.delete.clone()),
            set: self.set.as_ref().map(|_| base.set.clone()),
            merge: self.merge.as_ref().map(|_| base.merge.clone()),
            parameters: self.parameters.as_ref().map(|_| base.parameters.clone()),
        }
    }
}
//#endregion 🔖️Patches

//#region 🔖️MapAlgebra
/// 🗂️ Read access to a sorted keyed map a [`MapDelta`] edits.
trait MapView<V> {
    fn view_get(&self, key: &str) -> Option<&V>;
    fn view_keys(&self) -> Vec<&String>;
}

impl MapView<PropertyValue> for semio_framework_graph::manifest::PropertyBag {
    fn view_get(&self, key: &str) -> Option<&PropertyValue> {
        self.get(key)
    }
    fn view_keys(&self) -> Vec<&String> {
        self.keys().collect()
    }
}

impl MapView<LayoutPoint> for crate::standards::v1::subsets::any::schema::RuleLayout {
    fn view_get(&self, key: &str) -> Option<&LayoutPoint> {
        self.get(key)
    }
    fn view_keys(&self) -> Vec<&String> {
        self.keys().collect()
    }
}

/// 🔁️ The negative map delta: every key the delta touches goes back to the value `base` held there, or is removed when
/// `base` held none.
fn invert_map<V: Clone, M: MapView<V>>(delta: &MapDelta<V>, base: &M) -> MapDelta<V> {
    let mut inverse = MapDelta::default();
    for (key, entry) in delta.entries() {
        if matches!(entry.operation(), MapEntryOperation::Reject) {
            continue;
        }
        inverse.absorb(match base.view_get(key) {
            Some(old) => MapDelta::set(key.clone(), old.clone()),
            None => MapDelta::remove(key.clone()),
        });
    }
    inverse
}

//#endregion 🔖️MapAlgebra

impl MutationDiff<RewritingSnapshot> for RewritingDiff {
    fn apply(&self, snapshot: &RewritingSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<RewritingSnapshot> {
        let mut next = snapshot.clone();
        if let Some(graph) = &self.working_graph {
            next.working_graph = MutationDiff::<JackSnapshot>::apply(graph, &snapshot.working_graph, capability).map_err(|error| error.under(["workingGraph"]))?;
        }
        if let Some(patch) = &self.lhs {
            patch.write_into(&mut next.lhs);
        }
        if let Some(patch) = &self.rhs {
            patch.write_into(&mut next.rhs);
        }
        if let Some(bindings) = &self.parameter_bindings {
            bindings.write_into(&mut next.parameter_bindings).map_err(|error| error.under(["parameterBindings"]))?;
        }
        if let Some(layout) = &self.rule_layout {
            layout.write_into(&mut next.rule_layout).map_err(|error| error.under(["ruleLayout"]))?;
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        match (&mut self.working_graph, other.working_graph) {
            (Some(earlier), Some(next)) => MutationDiff::<JackSnapshot>::absorb(earlier, next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.lhs, other.lhs) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.rhs, other.rhs) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        MapDelta::absorb_optional(&mut self.parameter_bindings, other.parameter_bindings);
        MapDelta::absorb_optional(&mut self.rule_layout, other.rule_layout);
    }
}

impl DiffAlgebra<RewritingSnapshot> for RewritingDiff {
    fn inverse(&self, base: &RewritingSnapshot) -> Self {
        Self {
            working_graph: self.working_graph.as_ref().map(|graph| DiffAlgebra::<JackSnapshot>::inverse(graph, &base.working_graph)),
            lhs: self.lhs.as_ref().map(|patch| patch.inverse(&base.lhs)),
            rhs: self.rhs.as_ref().map(|patch| patch.inverse(&base.rhs)),
            parameter_bindings: self.parameter_bindings.as_ref().map(|delta| invert_map(delta, &base.parameter_bindings)),
            rule_layout: self.rule_layout.as_ref().map(|delta| invert_map(delta, &base.rule_layout)),
        }
    }
    fn is_empty(&self) -> bool {
        self.working_graph.as_ref().is_none_or(|graph| DiffAlgebra::<JackSnapshot>::is_empty(graph)) && self.lhs.as_ref().is_none_or(LhsPatch::is_empty) && self.rhs.as_ref().is_none_or(RhsPatch::is_empty) && self.parameter_bindings.as_ref().is_none_or(MapDelta::is_empty) && self.rule_layout.as_ref().is_none_or(MapDelta::is_empty)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🗂️map-ownership/🦀️.rs"]
mod map_ownership_tests;
