//! 🧵️ Exact, resumable ownership retirement for persisted DAG snapshots and mutations.

use crate::os_dsl::DslValue;
use crate::os_store::{ArtifactOwnedValueRetirementFactory, ArtifactStoreCursorDisposer, ErasedSnapshotRetirement, MemberStoreOwner, DocumentStoreOwners, SnapshotRetirementFactory, SnapshotRetirementStep};
use crate::{DagFixtureEdge, DagMedia, DagMutation, DagNodeKind, DagNodeSpec, DagPreviewContent, DagSnapshot, IoPortSpec};
use graph::manifest::{PropertyBag, PropertyValue};
use std::collections::{BTreeSet, LinkedList};
use std::mem::ManuallyDrop;
use std::sync::Arc;

enum DagOwner {
    Bytes(Vec<u8>),
    Dsl(DslValue),
    DslValues(Vec<DslValue>),
    DslEntries(Vec<(String, DslValue)>),
    Property(PropertyValue),
    Properties(PropertyBag),
    PropertyValues(Vec<PropertyValue>),
    Port(IoPortSpec),
    Ports(Vec<IoPortSpec>),
    Strings(Vec<String>),
    Expanded(BTreeSet<String>),
    Preview(DagPreviewContent),
    Kind(DagNodeKind),
    Node(DagNodeSpec),
    Nodes(Vec<DagNodeSpec>),
    Edge(DagFixtureEdge),
    Edges(Vec<DagFixtureEdge>),
    Snapshot(DagSnapshot),
    Mutation(DagMutation),
}

#[must_use = "DAG ownership must be retired to a terminal-empty frontier"]
struct DagRetirement {
    owners: ManuallyDrop<LinkedList<DagOwner>>,
}

impl Default for DagRetirement {
    fn default() -> Self {
        Self { owners: ManuallyDrop::new(LinkedList::new()) }
    }
}

impl DagRetirement {
    fn from_snapshot(snapshot: DagSnapshot) -> Self {
        let mut retirement = Self::default();
        retirement.push(DagOwner::Snapshot(snapshot));
        retirement
    }

    fn from_mutation(mutation: DagMutation) -> Self {
        let mut retirement = Self::default();
        retirement.push(DagOwner::Mutation(mutation));
        retirement
    }

    fn push(&mut self, owner: DagOwner) {
        self.owners.push_front(owner);
    }

    fn text(&mut self, value: String) {
        self.push(DagOwner::Bytes(value.into_bytes()));
    }

    fn is_empty(&self) -> bool {
        self.owners.is_empty()
    }

    fn retire_port(&mut self, port: IoPortSpec) {
        let IoPortSpec { id, label, code, abbreviation, full_name, value_type, default, value, connected: _, artifact_kind, cardinality, shape: _, visible: _, resolved: _ } = port;
        self.text(id);
        self.text(label);
        self.text(code);
        self.text(abbreviation);
        self.text(full_name);
        if let Some(value) = value_type {
            self.text(value);
        }
        if let Some(value) = default {
            self.push(DagOwner::Dsl(value));
        }
        if let Some(value) = value {
            self.push(DagOwner::Dsl(value));
        }
        if let Some(value) = artifact_kind {
            self.text(value);
        }
        self.text(cardinality);
    }

    fn retire_kind(&mut self, kind: DagNodeKind) {
        match kind {
            DagNodeKind::Computation { inputs, outputs, variadic_inputs: _, variadic_outputs: _ } | DagNodeKind::Cluster { inputs, outputs } => {
                self.push(DagOwner::Ports(inputs));
                self.push(DagOwner::Ports(outputs));
            }
            DagNodeKind::Slider { min: _, max: _, step: _, value: _, output } => self.push(DagOwner::Port(output)),
            DagNodeKind::Select { options, selected: _, output } => {
                self.push(DagOwner::Strings(options));
                self.push(DagOwner::Port(output));
            }
            DagNodeKind::Screen { media, input } => {
                if let Some(DagMedia { kind: _, src }) = media {
                    self.text(src);
                }
                self.push(DagOwner::Port(input));
            }
            DagNodeKind::Note { text, output } => {
                self.text(text);
                self.push(DagOwner::Port(output));
            }
            DagNodeKind::Image { src, output } => {
                self.text(src);
                self.push(DagOwner::Port(output));
            }
            DagNodeKind::Preview { content, expanded, input } => {
                self.push(DagOwner::Preview(content));
                self.push(DagOwner::Expanded(expanded));
                self.push(DagOwner::Port(input));
            }
            DagNodeKind::Action { label, input } => {
                self.text(label);
                self.push(DagOwner::Port(input));
            }
            DagNodeKind::Export { label, format, input } => {
                self.text(label);
                self.text(format);
                self.push(DagOwner::Port(input));
            }
            DagNodeKind::AppInstance { instance_id, plugin_id, app_id, icon, inputs, outputs } => {
                self.text(instance_id);
                self.text(plugin_id);
                self.text(app_id);
                self.text(icon);
                self.push(DagOwner::Ports(inputs));
                self.push(DagOwner::Ports(outputs));
            }
        }
    }

    fn retire_node(&mut self, node: DagNodeSpec) {
        let DagNodeSpec { id, name, abbreviation, icon, x: _, y: _, width: _, height: _, operator_kind, properties, kind } = node;
        self.text(id);
        self.text(name);
        self.text(abbreviation);
        self.text(icon);
        if let Some(value) = operator_kind {
            self.text(value);
        }
        self.push(DagOwner::Properties(properties));
        self.push(DagOwner::Kind(kind));
    }

    fn retire_edge(&mut self, edge: DagFixtureEdge) {
        let DagFixtureEdge { id, source, target, route_style: _, properties } = edge;
        self.text(id);
        self.text(source);
        self.text(target);
        self.push(DagOwner::Properties(properties));
    }

    fn retire_mutation(&mut self, mutation: DagMutation) {
        match mutation {
            DagMutation::CreateNode(value) => self.push(DagOwner::Node(value.node)),
            DagMutation::DeleteNode(value) => self.text(value.id),
            DagMutation::RenameNode(value) => {
                self.text(value.id);
                self.text(value.new_id);
            }
            DagMutation::ChangeNodeName(value) => {
                self.text(value.id);
                self.text(value.new_name);
            }
            DagMutation::MoveNode(value) => self.text(value.id),
            DagMutation::ResizeNode(value) => self.text(value.id),
            DagMutation::ChangeNodeIcon(value) => {
                self.text(value.id);
                self.text(value.new_icon);
            }
            DagMutation::ChangeNodeAbbreviation(value) => {
                self.text(value.id);
                self.text(value.new_abbreviation);
            }
            DagMutation::ChangeNodeOperatorKind(value) => {
                self.text(value.id);
                if let Some(value) = value.new_operator_kind {
                    self.text(value);
                }
            }
            DagMutation::ReplaceNodeKind(value) => {
                self.text(value.id);
                self.push(DagOwner::Kind(value.new_kind));
            }
            DagMutation::ReplaceNodeProperties(value) => {
                self.text(value.id);
                self.push(DagOwner::Properties(value.new_properties));
            }
            DagMutation::ReorderNodes(value) => self.push(DagOwner::Strings(value.order)),
            DagMutation::ConnectNodes(value) => {
                self.text(value.id);
                self.text(value.source);
                self.text(value.target);
                self.push(DagOwner::Properties(value.properties));
            }
            DagMutation::DisconnectNodes(value) => self.text(value.id),
        }
    }
}

impl ErasedSnapshotRetirement for DagRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        use SnapshotRetirementStep as Step;
        if self.is_empty() {
            return Ok(Step::Complete);
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return Ok(Step::Blocked);
        }
        let owner = self.owners.pop_front().expect("nonempty DAG retirement");
        let mut released_bytes = 0;
        match owner {
            DagOwner::Bytes(mut value) => {
                released_bytes = maximum_bytes.min(value.len());
                value.truncate(value.len() - released_bytes);
                if !value.is_empty() {
                    self.push(DagOwner::Bytes(value));
                }
            }
            DagOwner::Dsl(value) => match value {
                DslValue::String(value) => self.text(value),
                DslValue::Array(values) => self.push(DagOwner::DslValues(values)),
                DslValue::Object(values) => self.push(DagOwner::DslEntries(values)),
                DslValue::Null | DslValue::Bool(_) | DslValue::Number(_) => {}
            },
            DagOwner::DslValues(mut values) => {
                let next = values.pop();
                if !values.is_empty() {
                    self.push(DagOwner::DslValues(values));
                }
                if let Some(value) = next {
                    self.push(DagOwner::Dsl(value));
                }
            }
            DagOwner::DslEntries(mut values) => {
                let next = values.pop();
                if !values.is_empty() {
                    self.push(DagOwner::DslEntries(values));
                }
                if let Some((key, value)) = next {
                    self.text(key);
                    self.push(DagOwner::Dsl(value));
                }
            }
            DagOwner::Property(value) => match value {
                PropertyValue::String(value) => self.text(value),
                PropertyValue::Array(values) => self.push(DagOwner::PropertyValues(values)),
                PropertyValue::Object(values) => self.push(DagOwner::Properties(values)),
                PropertyValue::Null | PropertyValue::Bool(_) | PropertyValue::Number(_) => {}
            },
            DagOwner::Properties(mut values) => {
                if let Some((key, value)) = values.pop_first() {
                    if !values.is_empty() {
                        self.push(DagOwner::Properties(values));
                    }
                    self.text(key);
                    self.push(DagOwner::Property(value));
                }
            }
            DagOwner::PropertyValues(mut values) => {
                let next = values.pop();
                if !values.is_empty() {
                    self.push(DagOwner::PropertyValues(values));
                }
                if let Some(value) = next {
                    self.push(DagOwner::Property(value));
                }
            }
            DagOwner::Port(value) => self.retire_port(value),
            DagOwner::Ports(mut values) => {
                let next = values.pop();
                if !values.is_empty() {
                    self.push(DagOwner::Ports(values));
                }
                if let Some(value) = next {
                    self.push(DagOwner::Port(value));
                }
            }
            DagOwner::Strings(mut values) => {
                let next = values.pop();
                if !values.is_empty() {
                    self.push(DagOwner::Strings(values));
                }
                if let Some(value) = next {
                    self.text(value);
                }
            }
            DagOwner::Expanded(mut values) => {
                if let Some(value) = values.pop_first() {
                    if !values.is_empty() {
                        self.push(DagOwner::Expanded(values));
                    }
                    self.text(value);
                }
            }
            DagOwner::Preview(value) => match value {
                DagPreviewContent::Empty => {}
                DagPreviewContent::Scalar { text } => self.text(text),
                DagPreviewContent::Image { src } => self.text(src),
                DagPreviewContent::Tree { json } => self.push(DagOwner::Dsl(json)),
            },
            DagOwner::Kind(value) => self.retire_kind(value),
            DagOwner::Node(value) => self.retire_node(value),
            DagOwner::Nodes(mut values) => {
                let next = values.pop();
                if !values.is_empty() {
                    self.push(DagOwner::Nodes(values));
                }
                if let Some(value) = next {
                    self.push(DagOwner::Node(value));
                }
            }
            DagOwner::Edge(value) => self.retire_edge(value),
            DagOwner::Edges(mut values) => {
                let next = values.pop();
                if !values.is_empty() {
                    self.push(DagOwner::Edges(values));
                }
                if let Some(value) = next {
                    self.push(DagOwner::Edge(value));
                }
            }
            DagOwner::Snapshot(value) => {
                self.text(value.schema);
                self.push(DagOwner::Nodes(value.nodes));
                self.push(DagOwner::Edges(value.edges));
            }
            DagOwner::Mutation(value) => self.retire_mutation(value),
        }
        Ok(Step::Pending { released_items: 1, released_bytes })
    }

    fn terminal_is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl Drop for DagRetirement {
    fn drop(&mut self) {
        if !self.is_empty() {
            if !std::thread::panicking() {
                panic!("DAG retirement dropped before terminal-empty");
            }
            return;
        }
        unsafe { ManuallyDrop::drop(&mut self.owners) };
    }
}

struct DagSnapshotRetirement {
    snapshot: Option<Arc<DagSnapshot>>,
    retirement: Option<DagRetirement>,
}

impl ErasedSnapshotRetirement for DagSnapshotRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if self.snapshot.is_some() && maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Blocked);
        }
        if let Some(snapshot) = self.snapshot.take() {
            if let Some(snapshot) = Arc::into_inner(snapshot) {
                self.retirement = Some(DagRetirement::from_snapshot(snapshot));
            }
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        let Some(retirement) = self.retirement.as_mut() else {
            return Ok(SnapshotRetirementStep::Complete);
        };
        let step = retirement.close_step(maximum_items, maximum_bytes)?;
        if matches!(step, SnapshotRetirementStep::Complete) {
            if !retirement.terminal_is_empty() {
                return Err("DAG snapshot retirement reported Complete before terminal-empty".into());
            }
            self.retirement = None;
        }
        Ok(step)
    }

    fn terminal_is_empty(&self) -> bool {
        self.snapshot.is_none() && self.retirement.is_none()
    }
}

impl Drop for DagSnapshotRetirement {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "DAG snapshot retirement dropped before terminal-empty");
    }
}

pub struct DagSnapshotRetirementFactory;

impl SnapshotRetirementFactory<DagSnapshot> for DagSnapshotRetirementFactory {
    fn retire(&self, snapshot: Arc<DagSnapshot>) -> Box<dyn ErasedSnapshotRetirement> {
        Box::new(DagSnapshotRetirement { snapshot: Some(snapshot), retirement: None })
    }
}

struct DagOwnedSnapshotRetirementFactory;

impl ArtifactOwnedValueRetirementFactory<DagSnapshot> for DagOwnedSnapshotRetirementFactory {
    fn retire_owned(&self, snapshot: DagSnapshot) -> Box<dyn ErasedSnapshotRetirement> {
        Box::new(DagRetirement::from_snapshot(snapshot))
    }
}

struct DagMutationRetirementFactory;

impl ArtifactOwnedValueRetirementFactory<DagMutation> for DagMutationRetirementFactory {
    fn retire_owned(&self, mutation: DagMutation) -> Box<dyn ErasedSnapshotRetirement> {
        Box::new(DagRetirement::from_mutation(mutation))
    }
}

impl MemberStoreOwner<DagMutation> for DagSnapshot {
    type SnapshotOpen = crate::os_store::UnsupportedMemberSnapshotOpen<Self>;

    fn member_store_owners() -> DocumentStoreOwners<Self, DagMutation> {
        DocumentStoreOwners::new(Arc::new(DagSnapshotRetirementFactory), Arc::new(DagOwnedSnapshotRetirementFactory), Arc::new(DagMutationRetirementFactory), Box::new(ArtifactStoreCursorDisposer::<DagSnapshot, DagMutation>::new()))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
