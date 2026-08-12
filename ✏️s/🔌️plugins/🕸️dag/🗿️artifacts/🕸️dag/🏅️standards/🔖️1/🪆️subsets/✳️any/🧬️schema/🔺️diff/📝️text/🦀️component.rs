//! 🔺️ DAG artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::dag::schema::DagArtifact;
use crate::artifacts::dag::{DagFixtureEdge, DagNodeSpec, DagSnapshot};
use protocol::{MutationDiff, Patchable};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::artifacts::dag::schema::diff::*;

//#region 🔖️Apply
pub fn apply_nodes_delta(items: &[DagNodeSpec], delta: &DagNodesDelta) -> Vec<DagNodeSpec> {
    let mut next = apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &DagNodePatchEntry| {
        (&entry.id, &entry.patch)
    });
    for entry in &delta.extra_patched {
        if let Some(item) = next.iter_mut().find(|item| item.id == entry.id) {
            if let Some(new_id) = &entry.patch.new_id {
                item.id = new_id.clone();
            }
            if let Some(icon) = &entry.patch.icon {
                item.icon = icon.clone();
            }
            if let Some(abbreviation) = &entry.patch.abbreviation {
                item.abbreviation = abbreviation.clone();
            }
            if let Some(operator_kind) = &entry.patch.operator_kind {
                item.operator_kind = operator_kind.clone();
            }
            if let Some(properties) = &entry.patch.properties {
                item.properties = properties.clone();
            }
        }
    }
    next
}

pub fn apply_edges_delta(items: &[DagFixtureEdge], delta: &DagEdgesDelta) -> Vec<DagFixtureEdge> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &DagEdgePatchEntry| {
        (&entry.id, &entry.patch)
    })
}

fn apply_identified_delta<T, P, E, F>(
    items: &[T],
    removed: &[String],
    added: &[T],
    patched: &[E],
    reordered: Option<&Vec<String>>,
    entry_parts: F,
) -> Vec<T>
where
    T: Clone + protocol::Identified<String> + Patchable<P>,
    P: Clone,
    F: Fn(&E) -> (&String, &P),
{
    let mut next = items.to_vec();
    for id in removed {
        next.retain(|item| item.id() != id);
    }
    for item in added {
        next.push(item.clone());
    }
    for entry in patched {
        let (id, patch) = entry_parts(entry);
        if let Some(item) = next.iter_mut().find(|item| item.id() == id) {
            item.apply_patch(patch);
        }
    }
    if let Some(order) = reordered {
        let mut by_id: std::collections::BTreeMap<_, _> = next.into_iter().map(|item| (item.id().clone(), item)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(item) = by_id.remove(id) {
                ordered.push(item);
            }
        }
        ordered.extend(by_id.into_values());
        next = ordered;
    }
    next
}

fn absorb_nodes_delta(target: &mut Option<DagNodesDelta>, incoming: Option<DagNodesDelta>) {
    if let Some(src) = incoming {
        match target {
            Some(dst) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                dst.extra_patched.extend(src.extra_patched);
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            None => *target = Some(src),
        }
    }
}

fn absorb_edges_delta(target: &mut Option<DagEdgesDelta>, incoming: Option<DagEdgesDelta>) {
    if let Some(src) = incoming {
        match target {
            Some(dst) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            None => *target = Some(src),
        }
    }
}

impl DagDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &DagArtifact) -> DagArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(delta) = &self.nodes {
            next.nodes = apply_nodes_delta(&next.nodes, delta);
        }
        if let Some(delta) = &self.edges {
            next.edges = apply_edges_delta(&next.edges, delta);
        }
        if let Some(list) = &self.set_nodes {
            next.nodes = list.values.clone();
        }
        if let Some(list) = &self.set_edges {
            next.edges = list.values.clone();
        }
        if let Some(list) = &self.selected_node_ids {
            next.selected_node_ids = list.values.clone();
        }
        if let Some(value) = &self.camera {
            next.camera = value.clone();
        }
        if let Some(value) = &self.locale {
            next.locale = value.clone();
        }
        next
    }
}

impl MutationDiff<DagSnapshot> for DagDiff {
    fn apply(&self, snapshot: &DagSnapshot) -> DagSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(delta) = &self.nodes {
            next.nodes = apply_nodes_delta(&next.nodes, delta);
        }
        if let Some(delta) = &self.edges {
            next.edges = apply_edges_delta(&next.edges, delta);
        }
        if let Some(list) = &self.set_nodes {
            next.nodes = list.values.clone();
        }
        if let Some(list) = &self.set_edges {
            next.edges = list.values.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        absorb_nodes_delta(&mut self.nodes, other.nodes);
        absorb_edges_delta(&mut self.edges, other.edges);
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(set_nodes);
        take!(set_edges);
        take!(selected_node_ids);
        take!(camera);
        take!(locale);
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::dag::default_snapshot;
    use crate::artifacts::dag::schema::mutations::delete_node;
    use protocol::Mutation;

    #[test]
    fn dag_diff_default_has_no_pending_writes() {
        let diff = DagDiff::default();
        assert!(diff.artifact.is_none());
        assert!(diff.nodes.is_none());
    }

    #[test]
    fn delete_node_diff_removes_the_node() {
        let base = default_snapshot();
        let id = base.nodes.first().expect("fixture has a node").id.clone();
        let mutation = delete_node(id.clone());
        let diff = mutation.diff(&base);
        assert!(diff.apply(&base).nodes.iter().all(|node| node.id != id));
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod semio_grammar_conformance {
    use super::*;

    #[test]
    fn component_grammar_semio_is_grammar_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_GRAMMAR_SEMIO).expect("parse grammar.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Grammar);
        assert!(!COMPONENT_GRAMMAR_SEMIO.is_empty());
        let _ = COMPONENT_GRAMMAR_PATH;
    }
}
