//! 💾️ Binary representation codec surface for `s.stdio.semio.flow` (snapshot) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

use crate::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, FlowNode, FlowParam, SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA};
use semio_framework_job::StepContext;
use std::mem::ManuallyDrop;
use store::{ErasedSnapshotRetirement, MemberOpenAdmissionError, MemberOpenDiagnostic, MemberOpenInputStep, MemberOpenPhase, MemberOpenProgress, MemberOpenRequest, MemberSnapshotOpenOperation, MemberSnapshotOpenStep};

const HEADER: &[u8] = b"\x89SEM\r\n\x1a\n\x18\0\0\0stdio.semio.flow.pack v1";
const MAX_NODES: usize = 256;
const MAX_EDGES: usize = 512;
const MAX_PARAMETERS: usize = 64;
const MAX_STRING_BYTES: usize = 4096;
const MAX_TOTAL_STRING_BYTES: usize = 1024 * 1024;

#[derive(Clone, Copy)]
enum Field {
    Schema,
    NodeId,
    NodeKind,
    NodeLabel,
    ParamKey,
    ParamValue,
    EdgeId,
    FromNode,
    FromPort,
    ToNode,
    ToPort,
    EdgeKind,
}
#[derive(Clone, Copy)]
enum Count {
    Nodes,
    Parameters,
    Edges,
}
#[derive(Clone, Copy)]
enum State {
    Header,
    Format,
    Length(Field),
    Text(Field),
    Count(Count),
    Float(bool),
    Complete,
}

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
pub enum SemioFlowSnapshotDecodeStep {
    Pending { consumed_bytes: usize },
    Ready,
    Rejected(MemberOpenDiagnostic),
}

impl SemioFlowSnapshotDecode {
    pub fn consumed_bytes(&self) -> usize {
        self.offset
    }
    pub fn retained_input_bytes(&self) -> usize {
        self.request.as_ref().map_or(0, MemberOpenRequest::retained_input_bytes)
    }

    fn reject(&mut self, diagnostic: MemberOpenDiagnostic) -> SemioFlowSnapshotDecodeStep {
        self.diagnostic.get_or_insert(diagnostic);
        SemioFlowSnapshotDecodeStep::Rejected(self.diagnostic.unwrap())
    }

    pub fn step(&mut self, cx: &mut StepContext<'_>) -> SemioFlowSnapshotDecodeStep {
        if let Some(diagnostic) = self.diagnostic {
            return SemioFlowSnapshotDecodeStep::Rejected(diagnostic);
        }
        if self.terminal {
            return SemioFlowSnapshotDecodeStep::Rejected(MemberOpenDiagnostic::Stale);
        }
        let frame = match self.request.as_mut().expect("decoder retains input").step_input(cx) {
            MemberOpenInputStep::Framed(frame) => frame,
            MemberOpenInputStep::Pending(_) => return SemioFlowSnapshotDecodeStep::Pending { consumed_bytes: self.offset },
            MemberOpenInputStep::Rejected(diagnostic) => return self.reject(diagnostic),
        };
        let length = frame.snapshot_range().1;
        cx.set_stage("member-open.flow-snapshot");
        while !cx.should_yield() {
            if let Err(diagnostic) = self.request.as_ref().unwrap().check_step_authority(cx) {
                return self.reject(diagnostic);
            }
            if matches!(self.state, State::Complete) {
                if self.offset != length {
                    return self.reject(MemberOpenDiagnostic::Malformed);
                }
                self.verified = true;
                return SemioFlowSnapshotDecodeStep::Ready;
            }
            if self.offset == length {
                return self.reject(MemberOpenDiagnostic::Malformed);
            }
            let mut byte = [0];
            match self.request.as_ref().unwrap().copy_snapshot_chunk(self.offset, &mut byte, cx) {
                Ok(1) => {}
                Ok(_) => return SemioFlowSnapshotDecodeStep::Pending { consumed_bytes: self.offset },
                Err(diagnostic) => return self.reject(diagnostic),
            }
            if let Err(diagnostic) = self.request.as_ref().unwrap().check_step_authority(cx) {
                return self.reject(diagnostic);
            }
            if let Err(diagnostic) = self.accept(byte[0]) {
                return self.reject(diagnostic);
            }
            self.offset += 1;
        }
        SemioFlowSnapshotDecodeStep::Pending { consumed_bytes: self.offset }
    }

    pub fn take_ready(&mut self, cx: &StepContext<'_>) -> Option<(SemioFlowSnapshot, MemberOpenRequest)> {
        if self.terminal || self.diagnostic.is_some() || !self.verified {
            return None;
        }
        if let Err(diagnostic) = self.request.as_ref()?.check_step_authority(cx) {
            self.reject(diagnostic);
            return None;
        }
        self.terminal = true;
        Some((self.snapshot.take()?, self.request.take()?))
    }

    fn uint(&mut self, byte: u8) -> Result<Option<usize>, MemberOpenDiagnostic> {
        if self.varint_bytes == 9 && byte > 1 {
            return Err(MemberOpenDiagnostic::Malformed);
        }
        self.magnitude |= u64::from(byte & 127) << (self.varint_bytes * 7);
        self.varint_bytes += 1;
        if byte >= 128 {
            return Ok(None);
        }
        if self.varint_bytes > 1 && byte == 0 {
            return Err(MemberOpenDiagnostic::Malformed);
        }
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
                if self.snapshot.as_ref().unwrap().schema != STDIO_SEMIOFLOW_DOCUMENT_SCHEMA {
                    return Err(MemberOpenDiagnostic::Identity);
                }
                State::Count(Count::Nodes)
            }
            Field::NodeId => State::Length(Field::NodeKind),
            Field::NodeKind => State::Length(Field::NodeLabel),
            Field::NodeLabel => State::Count(Count::Parameters),
            Field::ParamKey => State::Length(Field::ParamValue),
            Field::ParamValue => {
                self.parameters_left -= 1;
                if self.parameters_left == 0 {
                    State::Float(false)
                } else {
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
                if self.edges_left == 0 {
                    State::Complete
                } else {
                    self.snapshot.as_mut().unwrap().edges.push(FlowEdge::default());
                    State::Length(Field::EdgeId)
                }
            }
        };
        Ok(())
    }

    fn accept(&mut self, byte: u8) -> Result<(), MemberOpenDiagnostic> {
        match self.state {
            State::Header => {
                if HEADER.get(self.offset) != Some(&byte) {
                    return Err(MemberOpenDiagnostic::Malformed);
                }
                if self.offset + 1 == HEADER.len() {
                    self.state = State::Format;
                }
            }
            State::Format => {
                if byte != 1 {
                    return Err(MemberOpenDiagnostic::Malformed);
                }
                self.state = State::Length(Field::Schema);
            }
            State::Length(field) => {
                if let Some(length) = self.uint(byte)? {
                    if length > MAX_STRING_BYTES || self.string_bytes + length > MAX_TOTAL_STRING_BYTES {
                        return Err(MemberOpenDiagnostic::Capacity);
                    }
                    self.text(field).try_reserve_exact(length).map_err(|_| MemberOpenDiagnostic::Capacity)?;
                    self.string_bytes += length;
                    self.text_left = length;
                    if length == 0 {
                        self.finish_text(field)?;
                    } else {
                        self.state = State::Text(field);
                    }
                }
            }
            State::Text(field) => {
                if self.scalar_bytes == 0 {
                    self.utf8_bytes = match byte {
                        0..=127 => 1,
                        194..=223 => 2,
                        224..=239 => 3,
                        240..=244 => 4,
                        _ => return Err(MemberOpenDiagnostic::Malformed),
                    };
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
                    if self.scalar_bytes != 0 {
                        return Err(MemberOpenDiagnostic::Malformed);
                    }
                    self.finish_text(field)?;
                }
            }
            State::Count(kind) => {
                if let Some(count) = self.uint(byte)? {
                    let snapshot = self.snapshot.as_mut().unwrap();
                    self.state = match kind {
                        Count::Nodes => {
                            if count > MAX_NODES {
                                return Err(MemberOpenDiagnostic::Capacity);
                            }
                            snapshot.nodes.try_reserve_exact(count).map_err(|_| MemberOpenDiagnostic::Capacity)?;
                            self.nodes_left = count;
                            if count == 0 {
                                State::Count(Count::Edges)
                            } else {
                                snapshot.nodes.push(FlowNode::default());
                                State::Length(Field::NodeId)
                            }
                        }
                        Count::Parameters => {
                            if count > MAX_PARAMETERS {
                                return Err(MemberOpenDiagnostic::Capacity);
                            }
                            let parameters = &mut snapshot.nodes.last_mut().unwrap().params;
                            parameters.try_reserve_exact(count).map_err(|_| MemberOpenDiagnostic::Capacity)?;
                            self.parameters_left = count;
                            if count == 0 {
                                State::Float(false)
                            } else {
                                parameters.push(FlowParam::default());
                                State::Length(Field::ParamKey)
                            }
                        }
                        Count::Edges => {
                            if count > MAX_EDGES {
                                return Err(MemberOpenDiagnostic::Capacity);
                            }
                            snapshot.edges.try_reserve_exact(count).map_err(|_| MemberOpenDiagnostic::Capacity)?;
                            self.edges_left = count;
                            if count == 0 {
                                State::Complete
                            } else {
                                snapshot.edges.push(FlowEdge::default());
                                State::Length(Field::EdgeId)
                            }
                        }
                    };
                }
            }
            State::Float(y) => {
                self.scalar[self.scalar_bytes] = byte;
                self.scalar_bytes += 1;
                if self.scalar_bytes == 8 {
                    let value = f64::from_le_bytes(self.scalar);
                    self.scalar_bytes = 0;
                    let snapshot = self.snapshot.as_mut().unwrap();
                    if !y {
                        snapshot.nodes.last_mut().unwrap().position.x = value;
                        self.state = State::Float(true);
                    } else {
                        snapshot.nodes.last_mut().unwrap().position.y = value;
                        self.nodes_left -= 1;
                        self.state = if self.nodes_left == 0 {
                            State::Count(Count::Edges)
                        } else {
                            snapshot.nodes.push(FlowNode::default());
                            State::Length(Field::NodeId)
                        };
                    }
                }
            }
            State::Complete => return Err(MemberOpenDiagnostic::Malformed),
        }
        Ok(())
    }
}

impl SemioFlowSnapshotDecode {
    fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if let Some(active) = self.active.as_ref() { return store::artifact_retirement_box_demands(active, body); }
        if self.snapshot.is_some() { return store::artifact_retirement_owned_birth_demands(&self.snapshot); }
        if self.request.is_some() { return store::artifact_retirement_owner_demands(&self.request, body); }
        Ok(semio_framework_value::RetirementDemand { depth: usize::from(!self.terminal), ..Default::default() })
    }
}

impl ErasedSnapshotRetirement for SemioFlowSnapshotDecode {
    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::{retained_clone::{RetainedCloneProgress, RetainedCloneStep}, ValueError, ValueRefusalKind};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let body = if grant.maximum_copy_bytes == 0 { grant.maximum_release_bytes } else { grant.maximum_copy_bytes };
        let demand = self.retirement_demands(body)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "Flow decoder close exceeds original admitted depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        self.reject(MemberOpenDiagnostic::Cancelled);
        if self.active.is_some() { return store::artifact_retirement_box_close_step(&mut self.active, grant); }
        if self.snapshot.is_some() { return store::artifact_retirement_admit_owned(&mut self.snapshot, &mut self.active, grant); }
        if self.request.is_some() { return store::artifact_retirement_owner_close(&mut self.request, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        self.scalar.fill(0);
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty }))
    }

    fn terminal_is_empty(&self) -> bool { self.terminal && self.request.is_none() && self.snapshot.is_none() && self.active.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { self.retirement_demands(body) }
}
impl MemberSnapshotOpenOperation for SemioFlowSnapshotDecode {
    type Snapshot = SemioFlowSnapshot;
    fn begin_birth_bytes(request: &MemberOpenRequest) -> Result<usize, MemberOpenDiagnostic> {
        let dialect = &request.admitted_expected()?.dialect;
        if dialect.artifact_kind == "s.stdio.semio" && dialect.standard == "v1" && dialect.subset == "flow" { Ok(0) } else { Err(MemberOpenDiagnostic::Identity) }
    }

    fn begin(request: MemberOpenRequest) -> Result<Self, MemberOpenAdmissionError> {
        let dialect = match request.admitted_expected() {
            Ok(expected) => &expected.dialect,
            Err(diagnostic) => return Err(MemberOpenAdmissionError { diagnostic, request }),
        };
        if dialect.artifact_kind != "s.stdio.semio" || dialect.standard != "v1" || dialect.subset != "flow" {
            return Err(MemberOpenAdmissionError { diagnostic: MemberOpenDiagnostic::Identity, request });
        }
        Ok(Self {
            request: ManuallyDrop::new(Some(request)),
            snapshot: ManuallyDrop::new(Some(SemioFlowSnapshot { schema: String::new(), nodes: Vec::new(), edges: Vec::new() })),
            active: ManuallyDrop::new(None),
            state: State::Header,
            offset: 0,
            magnitude: 0,
            varint_bytes: 0,
            text_left: 0,
            string_bytes: 0,
            nodes_left: 0,
            parameters_left: 0,
            edges_left: 0,
            scalar: [0; 8],
            scalar_bytes: 0,
            utf8_bytes: 0,
            diagnostic: None,
            verified: false,
            terminal: false,
        })
    }

    fn step(&mut self, cx: &mut StepContext<'_>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> MemberSnapshotOpenStep {
        match SemioFlowSnapshotDecode::step(self, cx) {
            SemioFlowSnapshotDecodeStep::Pending { consumed_bytes } => MemberSnapshotOpenStep::Pending(store::MemberSnapshotOpenProgress {
                opening: MemberOpenProgress { phase: MemberOpenPhase::Snapshot, completed: consumed_bytes as u64, total: self.retained_input_bytes() as u64 },
                retained_progress: semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: usize::from(consumed_bytes > 0 && grant.maximum_items > 0), copied_bytes: consumed_bytes.min(grant.maximum_copy_bytes), ..Default::default() },
            }),
            SemioFlowSnapshotDecodeStep::Ready => MemberSnapshotOpenStep::Ready,
            SemioFlowSnapshotDecodeStep::Rejected(diagnostic) => MemberSnapshotOpenStep::Rejected(diagnostic),
        }
    }

    fn take_ready(&mut self, cx: &mut StepContext<'_>) -> Option<(Self::Snapshot, MemberOpenRequest)> {
        SemioFlowSnapshotDecode::take_ready(self, cx)
    }
}

impl Drop for SemioFlowSnapshotDecode {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Flow decoder dropped before exact handoff or bounded retirement");
    }
}

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::flow::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use framework_schema::ArtifactSchema;

/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers `stdio.json`'s upgraded `OpBinary`/`DiffCodec` reuse) backing
/// the real `ArtifactPack` below — replaces the old `serde_json::to_vec`-in-envelope shortcut.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_flow_snapshot_binary(s: &SemioFlowSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    store::pack_rt::write_varint_u64(&mut out, s.nodes.len() as u64);
    for n in &s.nodes {
        write_str_lp(&mut out, &n.id);
        write_str_lp(&mut out, &n.kind);
        write_str_lp(&mut out, &n.label);
        store::pack_rt::write_varint_u64(&mut out, n.params.len() as u64);
        for p in &n.params {
            write_str_lp(&mut out, &p.key);
            write_str_lp(&mut out, &p.value);
        }
        out.extend_from_slice(&n.position.x.to_le_bytes());
        out.extend_from_slice(&n.position.y.to_le_bytes());
    }
    store::pack_rt::write_varint_u64(&mut out, s.edges.len() as u64);
    for e in &s.edges {
        write_str_lp(&mut out, &e.id);
        write_str_lp(&mut out, &e.from.node);
        write_str_lp(&mut out, &e.from.port);
        write_str_lp(&mut out, &e.to.node);
        write_str_lp(&mut out, &e.to.port);
        write_str_lp(&mut out, &e.kind);
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_flow_snapshot_binary(bytes: &[u8]) -> Result<SemioFlowSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let node_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut nodes = Vec::with_capacity(node_count as usize);
    for _ in 0..node_count {
        let id = read_str_lp(&mut reader)?;
        let kind = read_str_lp(&mut reader)?;
        let label = read_str_lp(&mut reader)?;
        let param_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
        let mut params = Vec::with_capacity(param_count as usize);
        for _ in 0..param_count {
            let key = read_str_lp(&mut reader)?;
            let value = read_str_lp(&mut reader)?;
            params.push(FlowParam { key, value });
        }
        let x = reader.read_f64_le().map_err(|e| e.to_string())?;
        let y = reader.read_f64_le().map_err(|e| e.to_string())?;
        nodes.push(FlowNode { id, kind, label, params, position: SemioPoint2 { x, y } });
    }
    let edge_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut edges = Vec::with_capacity(edge_count as usize);
    for _ in 0..edge_count {
        let id = read_str_lp(&mut reader)?;
        let from_node = read_str_lp(&mut reader)?;
        let from_port = read_str_lp(&mut reader)?;
        let to_node = read_str_lp(&mut reader)?;
        let to_port = read_str_lp(&mut reader)?;
        let kind = read_str_lp(&mut reader)?;
        edges.push(FlowEdge { id, from: PortRef { node: from_node, port: from_port }, to: PortRef { node: to_node, port: to_port }, kind });
    }
    Ok(SemioFlowSnapshot { schema, nodes, edges })
}

impl store::ArtifactPack for SemioFlowSnapshot {

    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_flow_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        decode_flow_snapshot_binary(&inner).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v1::subsets::flow::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::flow::io::text::snapshot::*;
/// 📥️ Decodes this subset's own committed `.pack.semio` bytes into a real [`SemioFlowSnapshot`] — the
/// binary half of the same bridge, so a caller outside this crate can check the two codecs against
/// each other on the two real committed artifacts instead of against itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_flow_pack(bytes: &[u8]) -> Result<SemioFlowSnapshot, String> {
    <SemioFlowSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
/// 📤️ The `store::ArtifactPack::encode_pack` inverse of [`decode_semio_flow_pack`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_flow_pack(snapshot: &SemioFlowSnapshot) -> Vec<u8> {
    <SemioFlowSnapshot as store::ArtifactPack>::encode_pack(snapshot)
}
}
pub use native_snapshot_codec::*;
