//! 🛬 Source-owned semantic receiving measures and admits nonrecursive coordinates one original event at a time.
use super::{metadata, overlay::*, retained::*};
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{
    retained_clone::{RetainedCloneClose, RetainedCloneGrant, RetainedCloneStep},
    RetirementDemand, ValueError,
};
use std::mem::size_of;

const MAXIMUM_DEPTH: usize = 256;
#[derive(Clone, Copy, Default, Debug, semio_framework_value::RetireOwned)]
pub struct SourceRoot {
    pub row: usize,
    pub component: usize,
    pub argument: usize,
}
#[derive(Clone, Copy, Debug, semio_framework_value::RetireOwned)]
pub enum ReceivedKind {
    Unset,
    Derived,
    Integer(i64),
    Real(f64),
    Text,
    Binary,
    Enumeration,
    Reference(usize),
    Aggregate,
    Typed,
}
#[derive(Debug, semio_framework_value::RetireOwned)]
pub struct ReceivedNode {
    pub root: SourceRoot,
    pub parent: Option<usize>,
    pub child: usize,
    pub kind: ReceivedKind,
    pub first_edge: Option<usize>,
    pub last_edge: Option<usize>,
    pub children: usize,
}
#[derive(Clone, Copy, Debug, semio_framework_value::RetireOwned)]
pub struct ReceivedEdge {
    pub value: usize,
    pub next: Option<usize>,
}
#[derive(Clone, Copy, Debug, semio_framework_value::RetireOwned)]
pub struct ReceivedField {
    pub root: SourceRoot,
    pub value: usize,
}
#[derive(Clone, Copy, Debug, semio_framework_value::RetireOwned)]
pub struct ReceivedEntity {
    pub row: usize,
    pub first_field: usize,
    pub fields: usize,
}
#[derive(Default, semio_framework_value::RetireOwned)]
pub struct ReceivedPopulation {
    pub entities: Vec<ReceivedEntity>,
    pub fields: Vec<ReceivedField>,
    pub nodes: Vec<ReceivedNode>,
    pub edges: Vec<ReceivedEdge>,
}
#[derive(Clone, Copy, Default)]
struct Walk {
    child: usize,
    next: usize,
    node: usize,
    entered: bool,
}
#[derive(Clone, Copy, Default)]
enum Phase {
    #[default]
    Header,
    Fields,
    Walk,
    Allocate,
    Ready,
}

pub struct PopulationReceiver {
    arena: Option<ReceivedPopulation>,
    retirement: RetainedCloneClose,
    phase: Phase,
    receiving: bool,
    root: SourceRoot,
    stack: [Walk; MAXIMUM_DEPTH],
    depth: usize,
    binary: usize,
    measured_nodes: usize,
    measured_edges: usize,
    measured_fields: usize,
    entity_start: usize,
    closing: bool,
}
impl Default for PopulationReceiver {
    fn default() -> Self {
        Self {
            arena: None,
            retirement: Default::default(),
            phase: Phase::Header,
            receiving: false,
            root: Default::default(),
            stack: [Walk::default(); MAXIMUM_DEPTH],
            depth: 0,
            binary: 0,
            measured_nodes: 0,
            measured_edges: 0,
            measured_fields: 0,
            entity_start: 0,
            closing: false,
        }
    }
}
fn children(value: &IfcValue) -> &[IfcValue] {
    match value {
        IfcValue::Aggregate(items) => items,
        IfcValue::TypedValue(typed) => &typed.items,
        _ => &[],
    }
}
fn canonical_name(name: &str) -> bool {
    name.len() <= 128 && !name.is_empty() && name.bytes().enumerate().all(|(index, byte)| if index == 0 { byte.is_ascii_uppercase() } else { byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_' })
}
fn root_value<'a>(overlay: &PopulationOverlay, source: &'a IfcSnapshot, mutations: &'a [IfcMutation], root: SourceRoot) -> Result<&'a IfcValue, ValueError> {
    if root.component == 0 {
        overlay.argument(source, mutations, root.row, root.argument)
    } else {
        overlay.entity(source, mutations, root.row)?.complex.get(root.component - 1).and_then(|part| part.args.get(root.argument)).ok_or_else(|| refusal("express.receive-component-argument"))
    }
}
impl PopulationReceiver {
    /// 📐 Measure an exact arena birth or one fixed depth-frontier/leaf transition before touching an owner.
    pub fn demands(&self, overlay: &PopulationOverlay) -> Result<RetirementDemand, ValueError> {
        let mut demand = fixed(size_of::<ReceivedNode>() + size_of::<ReceivedField>() + size_of::<ReceivedEntity>() + size_of::<ReceivedEdge>() + 4 * size_of::<Walk>() + 128);
        demand.depth = self.depth + 1;
        if matches!(self.phase, Phase::Allocate) {
            demand.capacity_bytes = capacity(overlay.len(), size_of::<ReceivedEntity>())?
                .checked_add(capacity(self.measured_fields, size_of::<ReceivedField>())?)
                .and_then(|bytes| bytes.checked_add(capacity(self.measured_nodes, size_of::<ReceivedNode>()).ok()?))
                .and_then(|bytes| bytes.checked_add(capacity(self.measured_edges, size_of::<ReceivedEdge>()).ok()?))
                .ok_or_else(|| refusal("express.receive-capacity-overflow"))?;
        }
        Ok(demand)
    }
    fn current<'a>(&self, overlay: &PopulationOverlay, source: &'a IfcSnapshot, mutations: &'a [IfcMutation]) -> Result<&'a IfcValue, ValueError> {
        let mut value = root_value(overlay, source, mutations, self.root)?;
        for frame in &self.stack[1..=self.depth] {
            value = children(value).get(frame.child).ok_or_else(|| refusal("express.receive-source-path"))?;
        }
        Ok(value)
    }
    fn kind(&self, overlay: &PopulationOverlay, value: &IfcValue) -> Result<ReceivedKind, ValueError> {
        Ok(match value {
            IfcValue::Unset => ReceivedKind::Unset,
            IfcValue::Derived => ReceivedKind::Derived,
            IfcValue::Integer(value) => ReceivedKind::Integer(*value),
            IfcValue::Real(value) if value.is_finite() => ReceivedKind::Real(*value),
            IfcValue::Real(_) => return Err(refusal("express.receive-nonfinite-real")),
            IfcValue::String(_) => ReceivedKind::Text,
            IfcValue::Binary(_) => ReceivedKind::Binary,
            IfcValue::Enum(name) if canonical_name(name) => ReceivedKind::Enumeration,
            IfcValue::Enum(_) => return Err(refusal("express.receive-enumeration")),
            IfcValue::Reference(id) => ReceivedKind::Reference(overlay.find(*id).ok_or_else(|| refusal("express.receive-dangling-reference"))?),
            IfcValue::Aggregate(_) => ReceivedKind::Aggregate,
            IfcValue::TypedValue(typed) if canonical_name(&typed.name) && (metadata::defined_type(&typed.name).is_some() || metadata::entity(&typed.name).is_some()) => ReceivedKind::Typed,
            IfcValue::TypedValue(_) => return Err(refusal("express.receive-typed-name")),
        })
    }
    fn fields(&mut self, overlay: &PopulationOverlay, source: &IfcSnapshot, mutations: &[IfcMutation]) -> Result<(), ValueError> {
        if self.root.row == overlay.len() {
            self.phase = if self.receiving { Phase::Ready } else { Phase::Allocate };
            return Ok(());
        }
        let entity = overlay.entity(source, mutations, self.root.row)?;
        if self.root.component > entity.complex.len() {
            if self.receiving {
                let fields = self.arena.as_ref().unwrap().fields.len() - self.entity_start;
                self.arena.as_mut().unwrap().entities.push(ReceivedEntity { row: self.root.row, first_field: self.entity_start, fields });
                self.entity_start = self.arena.as_ref().unwrap().fields.len();
            }
            self.root.row += 1;
            self.root.component = 0;
            self.root.argument = 0;
            return Ok(());
        }
        let (name, arguments) = if self.root.component == 0 {
            (entity.name.as_str(), entity.args.as_slice())
        } else {
            let part = &entity.complex[self.root.component - 1];
            (part.name.as_str(), part.args.as_slice())
        };
        if self.root.argument == 0 {
            if !canonical_name(name) {
                return Err(refusal("express.receive-entity-name"));
            }
            let declaration = metadata::entity(name).ok_or_else(|| refusal("express.receive-unknown-entity"))?;
            let attributes = if entity.complex.is_empty() { declaration.all_attributes } else { declaration.attributes };
            if arguments.len() != attributes.len() {
                return Err(refusal("express.receive-attribute-count"));
            }
        }
        if self.root.argument == arguments.len() {
            self.root.component += 1;
            self.root.argument = 0;
            return Ok(());
        }
        self.depth = 0;
        self.stack[0] = Walk::default();
        self.binary = 0;
        self.phase = Phase::Walk;
        Ok(())
    }
    fn walk(&mut self, overlay: &PopulationOverlay, source: &IfcSnapshot, mutations: &[IfcMutation]) -> Result<(), ValueError> {
        let value = self.current(overlay, source, mutations)?;
        let count = children(value).len();
        if !self.stack[self.depth].entered {
            let kind = self.kind(overlay, value)?;
            if let IfcValue::Binary(text) = value {
                if text.is_empty() {
                    return Err(refusal("express.receive-binary"));
                }
                let end = self.binary.saturating_add(64).min(text.len());
                for (offset, byte) in text.as_bytes()[self.binary..end].iter().copied().enumerate() {
                    let index = self.binary + offset;
                    if index == 0 && !matches!(byte, b'0'..=b'3') || index > 0 && !matches!(byte, b'0'..=b'9' | b'A'..=b'F') {
                        return Err(refusal("express.receive-binary"));
                    }
                }
                self.binary = end;
                if end < text.len() {
                    return Ok(());
                }
                if text.len() == 1 && text != "0" || text.len() > 1 && u8::from_str_radix(&text[1..2], 16).unwrap_or(255) >= 1u8 << (4 - (text.as_bytes()[0] - b'0')) {
                    return Err(refusal("express.receive-binary"));
                }
            }
            if self.receiving {
                let node = self.arena.as_ref().unwrap().nodes.len();
                let parent = (self.depth > 0).then(|| self.stack[self.depth - 1].node);
                let value = ReceivedNode { root: self.root, parent, child: self.stack[self.depth].child, kind, first_edge: None, last_edge: None, children: count };
                let arena = self.arena.as_mut().unwrap();
                if arena.nodes.len() == arena.nodes.capacity() {
                    return Err(refusal("express.receive-node-measure-changed"));
                }
                arena.nodes.push(value);
                if let Some(parent) = parent {
                    if arena.edges.len() == arena.edges.capacity() {
                        return Err(refusal("express.receive-edge-measure-changed"));
                    }
                    let edge = arena.edges.len();
                    arena.edges.push(ReceivedEdge { value: node, next: None });
                    if let Some(previous) = arena.nodes[parent].last_edge {
                        arena.edges[previous].next = Some(edge);
                    } else {
                        arena.nodes[parent].first_edge = Some(edge);
                    }
                    arena.nodes[parent].last_edge = Some(edge);
                } else {
                    if arena.fields.len() == arena.fields.capacity() {
                        return Err(refusal("express.receive-field-measure-changed"));
                    }
                    arena.fields.push(ReceivedField { root: self.root, value: node });
                }
                self.stack[self.depth].node = node;
            } else {
                self.measured_nodes = self.measured_nodes.checked_add(1).ok_or_else(|| refusal("express.receive-node-overflow"))?;
                if self.depth == 0 {
                    self.measured_fields = self.measured_fields.checked_add(1).ok_or_else(|| refusal("express.receive-field-overflow"))?;
                } else {
                    self.measured_edges = self.measured_edges.checked_add(1).ok_or_else(|| refusal("express.receive-edge-overflow"))?;
                }
            }
            self.stack[self.depth].entered = true;
            self.binary = 0;
        } else if self.stack[self.depth].next < count {
            if self.depth + 1 >= MAXIMUM_DEPTH {
                return Err(refusal("express.receive-depth"));
            }
            let child = self.stack[self.depth].next;
            self.stack[self.depth].next += 1;
            self.depth += 1;
            self.stack[self.depth] = Walk { child, ..Default::default() };
            self.binary = 0;
        } else if self.depth > 0 {
            self.depth -= 1;
        } else {
            self.root.argument += 1;
            self.phase = Phase::Fields;
        }
        Ok(())
    }
    /// 🛬 Receive one real source frontier while preserving its original source and mutation owners.
    pub fn advance(&mut self, overlay: &PopulationOverlay, source: &IfcSnapshot, mutations: &[IfcMutation], cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if self.closing {
            return Err(refusal("express.receive-closing").into());
        }
        if !overlay.is_ready() {
            return Err(refusal("express.receive-overlay-incomplete").into());
        }
        overlay.check_authority(source, mutations)?;
        let demand = self.demands(overlay)?;
        if !admitted(cx, demand)? {
            return Ok(false);
        }
        match self.phase {
            Phase::Header => {
                let schema = &source.header.file_schema;
                if schema.len() != 1 || !matches!(&schema[0], IfcValue::Aggregate(values) if values.len() == 1 && matches!(&values[0], IfcValue::String(name) if name == "IFC4X3_ADD2")) {
                    return Err(refusal("express.receive-file-schema").into());
                }
                self.phase = Phase::Fields;
            }
            Phase::Fields => self.fields(overlay, source, mutations)?,
            Phase::Walk => self.walk(overlay, source, mutations)?,
            Phase::Allocate => {
                self.arena = Some(ReceivedPopulation { entities: Vec::with_capacity(overlay.len()), fields: Vec::with_capacity(self.measured_fields), nodes: Vec::with_capacity(self.measured_nodes), edges: Vec::with_capacity(self.measured_edges) });
                self.receiving = true;
                self.root = SourceRoot::default();
                self.phase = Phase::Fields;
            }
            Phase::Ready => {}
        }
        debit(cx, demand)?;
        Ok(matches!(self.phase, Phase::Ready))
    }
    pub fn arena(&self) -> Option<&ReceivedPopulation> {
        matches!(self.phase, Phase::Ready).then(|| self.arena.as_ref()).flatten()
    }
    /// 👁 Resolve an admitted arena handle back to a live original value without cloning text or aggregates.
    pub fn source_value<'a>(&self, overlay: &PopulationOverlay, source: &'a IfcSnapshot, mutations: &'a [IfcMutation], node: usize) -> Result<&'a IfcValue, ValueError> {
        let arena = self.arena.as_ref().ok_or_else(|| refusal("express.receive-arena"))?;
        let mut node = arena.nodes.get(node).ok_or_else(|| refusal("express.receive-node"))?;
        let root = node.root;
        let mut path = [0usize; MAXIMUM_DEPTH];
        let mut length = 0;
        while let Some(parent) = node.parent {
            if length == MAXIMUM_DEPTH {
                return Err(refusal("express.receive-path-depth"));
            }
            path[length] = node.child;
            length += 1;
            node = arena.nodes.get(parent).ok_or_else(|| refusal("express.receive-parent"))?;
        }
        let mut value = root_value(overlay, source, mutations, root)?;
        for index in path[..length].iter().rev() {
            value = children(value).get(*index).ok_or_else(|| refusal("express.receive-child"))?;
        }
        Ok(value)
    }
    pub fn begin_close(&mut self) {
        self.closing = true;
    }
    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.retirement.next_owner_demands::<ReceivedPopulation>(self.arena.is_some(), &None, body)
    }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing {
            return Err(refusal("express.receive-not-closing"));
        }
        if self.arena.is_some() {
            self.retirement.begin_granted(&mut self.arena, grant)?.ok_or_else(|| refusal("express.receive-close-owner"))
        } else if !self.retirement.is_empty() {
            self.retirement.step_granted(grant)
        } else {
            Ok(RetainedCloneStep::Complete(Default::default()))
        }
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.arena.is_none() && self.retirement.is_empty()
    }
}
