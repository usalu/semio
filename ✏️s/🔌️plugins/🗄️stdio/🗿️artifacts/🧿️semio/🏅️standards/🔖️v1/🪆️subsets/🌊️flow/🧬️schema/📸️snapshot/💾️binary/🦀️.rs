//! 💾️ Binary representation codec surface for `s.stdio.semio.flow` (snapshot) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

use super::{FlowEdge, FlowNode, FlowParam, SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA};
use semio_framework_job::StepContext;
use std::mem::ManuallyDrop;
use store::{ErasedSnapshotRetirement, MemberOpenAdmissionError, MemberOpenDiagnostic, MemberOpenInputStep, MemberOpenPhase, MemberOpenProgress, MemberOpenRequest, MemberSnapshotOpenOperation, MemberSnapshotOpenStep, SnapshotRetirementStep};

const HEADER: &[u8] = b"\x89SEM\r\n\x1a\n\x18\0\0\0stdio.semio.flow.pack v1";
const MAX_NODES: usize = 256;
const MAX_EDGES: usize = 512;
const MAX_PARAMETERS: usize = 64;
const MAX_STRING_BYTES: usize = 4096;
const MAX_TOTAL_STRING_BYTES: usize = 1024 * 1024;

#[derive(Clone, Copy)]
enum Field { Schema, NodeId, NodeKind, NodeLabel, ParamKey, ParamValue, EdgeId, FromNode, FromPort, ToNode, ToPort, EdgeKind }
#[derive(Clone, Copy)]
enum Count { Nodes, Parameters, Edges }
#[derive(Clone, Copy)]
enum State { Header, Format, Length(Field), Text(Field), Count(Count), Float(bool), Complete }

/// 🌊️ Retained decoder of the existing Flow binary format; input and partially hydrated fields remain owned until exact handoff or bounded retirement.
pub struct SemioFlowSnapshotDecode {
    request: ManuallyDrop<Option<MemberOpenRequest>>,
    snapshot: ManuallyDrop<Option<SemioFlowSnapshot>>,
    active: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    state: State,
    offset: usize,
    magnitude: u64,
    varint_bytes: usize,
    text_left: usize,
    string_bytes: usize,
    nodes_left: usize,
    parameters_left: usize,
    edges_left: usize,
    scalar: [u8; 8],
    scalar_bytes: usize,
    utf8_bytes: usize,
    diagnostic: Option<MemberOpenDiagnostic>,
    verified: bool,
    terminal: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemioFlowSnapshotDecodeStep { Pending { consumed_bytes: usize }, Ready, Rejected(MemberOpenDiagnostic) }

impl SemioFlowSnapshotDecode {
    pub fn new(request: MemberOpenRequest) -> Result<Self, MemberOpenAdmissionError> {
        let dialect = match request.admitted_expected() {
            Ok(expected) => &expected.dialect,
            Err(diagnostic) => return Err(MemberOpenAdmissionError { diagnostic, request }),
        };
        if dialect.artifact_kind != "s.stdio.semio" || dialect.standard != "v1" || dialect.subset != "flow" {
            return Err(MemberOpenAdmissionError { diagnostic: MemberOpenDiagnostic::Identity, request });
        }
        Ok(Self { request: ManuallyDrop::new(Some(request)), snapshot: ManuallyDrop::new(Some(SemioFlowSnapshot { schema: String::new(), nodes: Vec::new(), edges: Vec::new() })), active: ManuallyDrop::new(None), state: State::Header, offset: 0, magnitude: 0, varint_bytes: 0, text_left: 0, string_bytes: 0, nodes_left: 0, parameters_left: 0, edges_left: 0, scalar: [0; 8], scalar_bytes: 0, utf8_bytes: 0, diagnostic: None, verified: false, terminal: false })
    }

    pub fn consumed_bytes(&self) -> usize { self.offset }
    pub fn retained_input_bytes(&self) -> usize { self.request.as_ref().map_or(0, MemberOpenRequest::retained_input_bytes) }

    fn reject(&mut self, diagnostic: MemberOpenDiagnostic) -> SemioFlowSnapshotDecodeStep {
        self.diagnostic.get_or_insert(diagnostic);
        SemioFlowSnapshotDecodeStep::Rejected(self.diagnostic.unwrap())
    }

    pub fn step(&mut self, cx: &mut StepContext<'_>) -> SemioFlowSnapshotDecodeStep {
        if let Some(diagnostic) = self.diagnostic { return SemioFlowSnapshotDecodeStep::Rejected(diagnostic); }
        if self.terminal { return SemioFlowSnapshotDecodeStep::Rejected(MemberOpenDiagnostic::Stale); }
        let frame = match self.request.as_mut().expect("decoder retains input").step_input(cx) {
            MemberOpenInputStep::Framed(frame) => frame,
            MemberOpenInputStep::Pending(_) => return SemioFlowSnapshotDecodeStep::Pending { consumed_bytes: self.offset },
            MemberOpenInputStep::Rejected(diagnostic) => return self.reject(diagnostic),
        };
        let length = frame.snapshot_range().1;
        cx.set_stage("member-open.flow-snapshot");
        while !cx.should_yield() {
            if let Err(diagnostic) = self.request.as_ref().unwrap().check_step_authority(cx) { return self.reject(diagnostic); }
            if matches!(self.state, State::Complete) {
                if self.offset != length { return self.reject(MemberOpenDiagnostic::Malformed); }
                self.verified = true;
                return SemioFlowSnapshotDecodeStep::Ready;
            }
            if self.offset == length { return self.reject(MemberOpenDiagnostic::Malformed); }
            let mut byte = [0];
            match self.request.as_ref().unwrap().copy_snapshot_chunk(self.offset, &mut byte, cx) {
                Ok(1) => {}
                Ok(_) => return SemioFlowSnapshotDecodeStep::Pending { consumed_bytes: self.offset },
                Err(diagnostic) => return self.reject(diagnostic),
            }
            if let Err(diagnostic) = self.request.as_ref().unwrap().check_step_authority(cx) { return self.reject(diagnostic); }
            if let Err(diagnostic) = self.accept(byte[0]) { return self.reject(diagnostic); }
            self.offset += 1;
        }
        SemioFlowSnapshotDecodeStep::Pending { consumed_bytes: self.offset }
    }

    pub fn take_ready(&mut self, cx: &StepContext<'_>) -> Option<(SemioFlowSnapshot, MemberOpenRequest)> {
        if self.terminal || self.diagnostic.is_some() || !self.verified { return None; }
        if let Err(diagnostic) = self.request.as_ref()?.check_step_authority(cx) { self.reject(diagnostic); return None; }
        self.terminal = true;
        Some((self.snapshot.take()?, self.request.take()?))
    }

    fn uint(&mut self, byte: u8) -> Result<Option<usize>, MemberOpenDiagnostic> {
        if self.varint_bytes == 9 && byte > 1 { return Err(MemberOpenDiagnostic::Malformed); }
        self.magnitude |= u64::from(byte & 127) << (self.varint_bytes * 7);
        self.varint_bytes += 1;
        if byte >= 128 { return Ok(None); }
        if self.varint_bytes > 1 && byte == 0 { return Err(MemberOpenDiagnostic::Malformed); }
        let value = usize::try_from(self.magnitude).map_err(|_| MemberOpenDiagnostic::Capacity)?;
        self.magnitude = 0;
        self.varint_bytes = 0;
        Ok(Some(value))
    }

    fn text(&mut self, field: Field) -> &mut String {
        let snapshot = self.snapshot.as_mut().unwrap();
        match field {
            Field::Schema => &mut snapshot.schema,
            Field::NodeId => &mut snapshot.nodes.last_mut().unwrap().id,
            Field::NodeKind => &mut snapshot.nodes.last_mut().unwrap().kind,
            Field::NodeLabel => &mut snapshot.nodes.last_mut().unwrap().label,
            Field::ParamKey => &mut snapshot.nodes.last_mut().unwrap().params.last_mut().unwrap().key,
            Field::ParamValue => &mut snapshot.nodes.last_mut().unwrap().params.last_mut().unwrap().value,
            Field::EdgeId => &mut snapshot.edges.last_mut().unwrap().id,
            Field::FromNode => &mut snapshot.edges.last_mut().unwrap().from.node,
            Field::FromPort => &mut snapshot.edges.last_mut().unwrap().from.port,
            Field::ToNode => &mut snapshot.edges.last_mut().unwrap().to.node,
            Field::ToPort => &mut snapshot.edges.last_mut().unwrap().to.port,
            Field::EdgeKind => &mut snapshot.edges.last_mut().unwrap().kind,
        }
    }

    fn finish_text(&mut self, field: Field) -> Result<(), MemberOpenDiagnostic> {
        self.state = match field {
            Field::Schema => {
                if self.snapshot.as_ref().unwrap().schema != STDIO_SEMIOFLOW_DOCUMENT_SCHEMA { return Err(MemberOpenDiagnostic::Identity); }
                State::Count(Count::Nodes)
            }
            Field::NodeId => State::Length(Field::NodeKind),
            Field::NodeKind => State::Length(Field::NodeLabel),
            Field::NodeLabel => State::Count(Count::Parameters),
            Field::ParamKey => State::Length(Field::ParamValue),
            Field::ParamValue => {
                self.parameters_left -= 1;
                if self.parameters_left == 0 { State::Float(false) } else {
                    self.snapshot.as_mut().unwrap().nodes.last_mut().unwrap().params.push(FlowParam::default());
                    State::Length(Field::ParamKey)
                }
            }
            Field::EdgeId => State::Length(Field::FromNode),
            Field::FromNode => State::Length(Field::FromPort),
            Field::FromPort => State::Length(Field::ToNode),
            Field::ToNode => State::Length(Field::ToPort),
            Field::ToPort => State::Length(Field::EdgeKind),
            Field::EdgeKind => {
                self.edges_left -= 1;
                if self.edges_left == 0 { State::Complete } else { self.snapshot.as_mut().unwrap().edges.push(FlowEdge::default()); State::Length(Field::EdgeId) }
            }
        };
        Ok(())
    }

    fn accept(&mut self, byte: u8) -> Result<(), MemberOpenDiagnostic> {
        match self.state {
            State::Header => {
                if HEADER.get(self.offset) != Some(&byte) { return Err(MemberOpenDiagnostic::Malformed); }
                if self.offset + 1 == HEADER.len() { self.state = State::Format; }
            }
            State::Format => { if byte != 1 { return Err(MemberOpenDiagnostic::Malformed); } self.state = State::Length(Field::Schema); }
            State::Length(field) => if let Some(length) = self.uint(byte)? {
                if length > MAX_STRING_BYTES || self.string_bytes + length > MAX_TOTAL_STRING_BYTES { return Err(MemberOpenDiagnostic::Capacity); }
                self.text(field).try_reserve_exact(length).map_err(|_| MemberOpenDiagnostic::Capacity)?;
                self.string_bytes += length;
                self.text_left = length;
                if length == 0 { self.finish_text(field)?; } else { self.state = State::Text(field); }
            },
            State::Text(field) => {
                if self.scalar_bytes == 0 {
                    self.utf8_bytes = match byte { 0..=127 => 1, 194..=223 => 2, 224..=239 => 3, 240..=244 => 4, _ => return Err(MemberOpenDiagnostic::Malformed) };
                }
                self.scalar[self.scalar_bytes] = byte;
                self.scalar_bytes += 1;
                self.text_left -= 1;
                if self.scalar_bytes == self.utf8_bytes {
                    let character = std::str::from_utf8(&self.scalar[..self.scalar_bytes]).map_err(|_| MemberOpenDiagnostic::Malformed)?.chars().next().unwrap();
                    self.text(field).push(character);
                    self.scalar_bytes = 0;
                }
                if self.text_left == 0 {
                    if self.scalar_bytes != 0 { return Err(MemberOpenDiagnostic::Malformed); }
                    self.finish_text(field)?;
                }
            }
            State::Count(kind) => if let Some(count) = self.uint(byte)? {
                let snapshot = self.snapshot.as_mut().unwrap();
                self.state = match kind {
                    Count::Nodes => {
                        if count > MAX_NODES { return Err(MemberOpenDiagnostic::Capacity); }
                        snapshot.nodes.try_reserve_exact(count).map_err(|_| MemberOpenDiagnostic::Capacity)?;
                        self.nodes_left = count;
                        if count == 0 { State::Count(Count::Edges) } else { snapshot.nodes.push(FlowNode::default()); State::Length(Field::NodeId) }
                    }
                    Count::Parameters => {
                        if count > MAX_PARAMETERS { return Err(MemberOpenDiagnostic::Capacity); }
                        let parameters = &mut snapshot.nodes.last_mut().unwrap().params;
                        parameters.try_reserve_exact(count).map_err(|_| MemberOpenDiagnostic::Capacity)?;
                        self.parameters_left = count;
                        if count == 0 { State::Float(false) } else { parameters.push(FlowParam::default()); State::Length(Field::ParamKey) }
                    }
                    Count::Edges => {
                        if count > MAX_EDGES { return Err(MemberOpenDiagnostic::Capacity); }
                        snapshot.edges.try_reserve_exact(count).map_err(|_| MemberOpenDiagnostic::Capacity)?;
                        self.edges_left = count;
                        if count == 0 { State::Complete } else { snapshot.edges.push(FlowEdge::default()); State::Length(Field::EdgeId) }
                    }
                };
            },
            State::Float(y) => {
                self.scalar[self.scalar_bytes] = byte;
                self.scalar_bytes += 1;
                if self.scalar_bytes == 8 {
                    let value = f64::from_le_bytes(self.scalar);
                    self.scalar_bytes = 0;
                    let snapshot = self.snapshot.as_mut().unwrap();
                    if !y { snapshot.nodes.last_mut().unwrap().position.x = value; self.state = State::Float(true); }
                    else {
                        snapshot.nodes.last_mut().unwrap().position.y = value;
                        self.nodes_left -= 1;
                        self.state = if self.nodes_left == 0 { State::Count(Count::Edges) } else { snapshot.nodes.push(FlowNode::default()); State::Length(Field::NodeId) };
                    }
                }
            }
            State::Complete => return Err(MemberOpenDiagnostic::Malformed),
        }
        Ok(())
    }
}

impl ErasedSnapshotRetirement for SemioFlowSnapshotDecode {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if self.terminal { return Ok(SnapshotRetirementStep::Complete); }
        if maximum_items == 0 { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        self.reject(MemberOpenDiagnostic::Cancelled);
        if let Some(active) = self.active.as_mut() {
            return match active.close_step(1, maximum_bytes)? {
                SnapshotRetirementStep::Complete if active.terminal_is_empty() => { self.active.take(); Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }) }
                SnapshotRetirementStep::Complete => Err("Flow decoder retirement reported false terminal".into()),
                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > maximum_bytes => Err("Flow decoder retirement exceeded grant".into()),
                step => Ok(step),
            };
        }
        if let Some(snapshot) = self.snapshot.take() {
            *self.active = Some(store::retirement::owned_retirement(snapshot));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(request) = self.request.as_mut() {
            return match request.close_step(1, maximum_bytes)? {
                SnapshotRetirementStep::Complete if request.terminal_is_empty() => { self.request.take(); Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }) }
                SnapshotRetirementStep::Complete => Err("Flow decoder input reported false terminal".into()),
                step => Ok(step),
            };
        }
        self.scalar.fill(0);
        self.terminal = true;
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool { self.terminal && self.request.is_none() && self.snapshot.is_none() && self.active.is_none() }
}

impl MemberSnapshotOpenOperation for SemioFlowSnapshotDecode {
    type Snapshot = SemioFlowSnapshot;

    fn begin(request: MemberOpenRequest) -> Result<Self, MemberOpenAdmissionError> {
        Self::new(request)
    }

    fn step(&mut self, cx: &mut StepContext<'_>) -> MemberSnapshotOpenStep {
        match SemioFlowSnapshotDecode::step(self, cx) {
            SemioFlowSnapshotDecodeStep::Pending { consumed_bytes } => MemberSnapshotOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Snapshot, completed: consumed_bytes as u64, total: self.retained_input_bytes() as u64 }),
            SemioFlowSnapshotDecodeStep::Ready => MemberSnapshotOpenStep::Ready,
            SemioFlowSnapshotDecodeStep::Rejected(diagnostic) => MemberSnapshotOpenStep::Rejected(diagnostic),
        }
    }

    fn take_ready(&mut self, cx: &mut StepContext<'_>) -> Option<(Self::Snapshot, MemberOpenRequest)> {
        SemioFlowSnapshotDecode::take_ready(self, cx)
    }
}

impl Drop for SemioFlowSnapshotDecode {
    fn drop(&mut self) { assert!(self.terminal_is_empty(), "Flow decoder dropped before exact handoff or bounded retirement"); }
}
