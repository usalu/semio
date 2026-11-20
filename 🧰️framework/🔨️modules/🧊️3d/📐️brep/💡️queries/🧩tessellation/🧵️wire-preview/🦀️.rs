//! 🧵️ Funded line, conic and spline edge previews borrowed from the native topology.

use super::*;
use semio_framework_3d_brep_spline_trim::{SplineCursor, SplineSpan, SplineTrim, SplineVisit};
use semio_framework_3d_brep_representation::topology::history::PersistentLabel;
use semio_framework_mesh_engine::{ComponentReferenceTable, MeshData};
use semio_framework_value::{RetainedCloneGrant as Grant, RetainedCloneProgress as Progress, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use std::mem::{size_of, take};

const DOMAINS: [&str; 2] = ["edge", "vertex"];
const MAX_SEGMENTS: usize = 20_000;

/// 🧭️ Borrows the original selected edge or ordered wire under its stable live identity.
#[derive(Clone, Copy)]
pub enum WirePreviewSource<'a> {
    Vertex(VertexId),
    Edge(EdgeId),
    Wire { label: u64, members: &'a [(EdgeId, bool)] },
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SourceKey {
    Vertex(VertexId),
    Edge(EdgeId),
    Wire { label: u64, length: usize },
}

impl WirePreviewSource<'_> {
    fn key(self) -> SourceKey {
        match self {
            Self::Vertex(id) => SourceKey::Vertex(id),
            Self::Edge(id) => SourceKey::Edge(id),
            Self::Wire { label, members } => SourceKey::Wire { label, length: members.len() },
        }
    }

    fn member(self, index: usize) -> Option<(EdgeId, bool)> {
        match self {
            Self::Vertex(_) => None,
            Self::Edge(id) => (index == 0).then_some((id, true)),
            Self::Wire { members, .. } => members.get(index).copied(),
        }
    }

    fn len(self) -> usize {
        match self {
            Self::Vertex(_) => 0,
            Self::Edge(_) => 1,
            Self::Wire { members, .. } => members.len(),
        }
    }

    fn vertex_extent(self)->usize{
        if matches!(self,Self::Vertex(_)){1}else{2*self.len()}
    }
}

#[derive(Clone, Copy)]
struct VertexSample {
    position: [f32; 3],
    label: u64,
}

#[derive(Clone, Copy)]
struct Member {
    edge: EdgeId,
    label: u64,
    forward: bool,
    ends: [VertexSample; 2],
    segments: usize,
    spline: Option<SplineTrim>,
}

#[derive(Clone, Copy)]
enum Stage {
    Counting,
    CountingSpline,
    Reserve(usize),
    Reading,
    Labeling(usize),
    Sampling,
    SelectingSpline,
    SamplingSpline(SplineSpan),
}

semio_framework_value::artifact_retire_leaf!(SourceKey, VertexSample, Member, Stage);

pub(super) enum WirePreviewStep {
    Working,
    Unsupported,
    Ready(MeshData),
}

pub(super) struct WirePreview {
    source: SourceKey,
    revision: u64,
    stage: Stage,
    member: usize,
    segment: usize,
    total_segments: usize,
    spline_cursor: Option<SplineCursor>,
    current: Option<Member>,
    vertex: Option<VertexSample>,
    previous: [f32; 3],
    labels: HistoryFoldIndex<u64, u32>,
    references: Vec<(String, Vec<String>)>,
    mesh: MeshData,
}

semio_framework_value::artifact_retire_struct!(WirePreview { source, revision, stage, member, segment, total_segments, spline_cursor, current, vertex, previous, labels, references, mesh });

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "wire preview requires valid finite topology")
}

fn vector32(point: Pnt3) -> Result<[f32; 3], ValueError> {
    let axes = [point.x as f32, point.y as f32, point.z as f32];
    if axes.into_iter().all(f32::is_finite) { Ok(axes) } else { Err(invalid()) }
}

fn reserve<T>(target: &mut Vec<T>, count: usize) -> Result<usize, ValueError> {
    target.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "wire preview buffer allocation failed"))?;
    Ok(target.capacity() * size_of::<T>())
}

fn text(value: &str) -> Result<String, ValueError> {
    let mut output = String::new();
    output.try_reserve_exact(value.len()).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "wire preview label allocation failed"))?;
    output.push_str(value);
    Ok(output)
}

impl WirePreview {
    pub(super) fn new(source: WirePreviewSource<'_>, revision: u64) -> Self {
        Self {
            source: source.key(), revision, stage: Stage::Counting, member: 0, segment: 0, total_segments: 0, spline_cursor: None,
            current: None, vertex:None, previous: [0.; 3], labels: HistoryFoldIndex::new(), references: Vec::new(), mesh: MeshData::default(),
        }
    }

    fn check_source(&self, source: WirePreviewSource<'_>, revision: u64) -> Result<(), ValueError> {
        if self.source == source.key() && self.revision == revision { Ok(()) }
        else { Err(ValueError::invariant("original wire preview source changed")) }
    }

    fn member(body: &Body, id: EdgeId, forward: bool, deflection: f64) -> Result<Option<Member>, ValueError> {
        let edge = body.edges.get(id).ok_or_else(invalid)?;
        let curve = body.curves3.get(edge.curve).ok_or_else(invalid)?;
        let (start, end) = edge.range;
        let sweep = (end - start).abs();
        if !sweep.is_finite() || sweep == 0. { return Err(invalid()); }
        let mut spline = None;
        let segments = match curve {
            Curve3::Line { .. } => 1,
            Curve3::Circle { radius, .. } => {
                if !radius.is_finite() || *radius == 0. { return Err(invalid()); }
                edge_sampling::segments_for_chord_deviation(radius.abs(), sweep, deflection, DEFAULT_ANGULAR_TOL)
            }
            Curve3::Ellipse { major_radius, minor_radius, .. } => {
                let a = major_radius.abs().max(minor_radius.abs());
                let b = major_radius.abs().min(minor_radius.abs());
                if !a.is_finite() || !b.is_finite() || b == 0. { return Err(invalid()); }
                edge_sampling::segments_for_chord_deviation(a * (a / b), sweep, deflection, DEFAULT_ANGULAR_TOL)
            }
            Curve3::Nurbs { .. } => {
                let Some(trim) = SplineTrim::new(curve, edge.range, forward) else { return Ok(None); };
                spline = Some(trim);
                0
            }
        };
        let vertex = |id| -> Result<VertexSample, ValueError> {
            let vertex = body.vertices.get(id).ok_or_else(invalid)?;
            Ok(VertexSample { position: vector32(vertex.position)?, label: vertex.label.0 })
        };
        let ends = if forward { [vertex(edge.v0)?, vertex(edge.v1)?] } else { [vertex(edge.v1)?, vertex(edge.v0)?] };
        if segments == 1 && ends[0].position == ends[1].position { return Ok(None); }
        Ok(Some(Member { edge: id, label: edge.label.0, forward, ends, segments, spline }))
    }

    fn curve(body: &Body, id: EdgeId) -> Result<&Curve3, ValueError> {
        let edge = body.edges.get(id).ok_or_else(invalid)?;
        body.curves3.get(edge.curve).ok_or_else(invalid)
    }

    fn count_segments(&mut self, count: usize) -> bool {
        let Some(total) = self.total_segments.checked_add(count) else { return false; };
        if total > MAX_SEGMENTS { return false; }
        self.total_segments = total;
        true
    }

    fn finish_member(&mut self, stage: Stage) {
        self.current = None;
        self.spline_cursor = None;
        self.member += 1;
        self.stage = stage;
    }

    fn emit_segment(&mut self, member: Member, a: [f32; 3], b: [f32; 3]) -> Result<(), ValueError> {
        let id = *self.labels.get(&member.label).ok_or_else(invalid)?;
        self.mesh.edge_positions.extend(a);
        self.mesh.edge_positions.extend(b);
        self.mesh.edge_ids.push(id);
        Ok(())
    }

    fn read_depth(body: &Body, id: EdgeId) -> Result<usize, ValueError> {
        let edge = body.edges.get(id).ok_or_else(invalid)?;
        Ok(body.edges.next_remove_depth_demand(id)?
            .max(body.curves3.next_remove_depth_demand(edge.curve)?)
            .max(body.vertices.next_remove_depth_demand(edge.v0)?)
            .max(body.vertices.next_remove_depth_demand(edge.v1)?))
    }

    fn slot(&self, slot: usize, members: usize, vertices:usize) -> (usize, usize) {
        match slot {
            0 => (0, self.total_segments * 6 * size_of::<f32>()),
            1 => (0, self.total_segments * size_of::<u32>()),
            2 => (0, vertices * 3 * size_of::<f32>()),
            3 => (0, vertices * size_of::<u32>()),
            4 => (0, DOMAINS.len() * size_of::<(String, Vec<String>)>()),
            5 | 6 => { let bytes = DOMAINS[slot - 5].len(); (bytes, bytes) }
            7 => (0, members * size_of::<String>()),
            8 => (0, vertices * size_of::<String>()),
            _ => unreachable!("wire preview has nine original buffers and keys"),
        }
    }

    fn label(&self, index: usize) -> (u64, Option<[f32; 3]>) {
        if let Some(vertex)=self.vertex{return (vertex.label,Some(vertex.position));}
        let member = self.current.expect("wire preview labeling has a current member");
        if index == 0 { (member.label, None) }
        else {
            let vertex = member.ends[index - 1];
            (vertex.label, Some(vertex.position))
        }
    }

    pub(super) fn demand(&self, body: &Body, source: WirePreviewSource<'_>, revision: u64) -> Result<RetirementDemand, ValueError> {
        self.check_source(source, revision)?;
        let mut depth = 1;
        let (copy_bytes, capacity_bytes) = match self.stage {
            Stage::Counting | Stage::Reading => {
                if let WirePreviewSource::Vertex(id)=source{
                    depth=depth.max(body.vertices.next_remove_depth_demand(id)?);
                }
                if let Some((id, _)) = source.member(self.member) {
                    depth = depth.max(Self::read_depth(body, id)?);
                    if matches!(self.stage, Stage::Reading) {
                        depth = depth.max(self.labels.next_contains_depth_demand(&body.edges.get(id).ok_or_else(invalid)?.label.0)?);
                    }
                }
                (size_of::<Member>(), 0)
            }
            Stage::Reserve(slot) => self.slot(slot, source.len(),source.vertex_extent()),
            Stage::Labeling(index) => {
                let (label, position) = self.label(index);
                depth = depth.max(self.labels.next_insert_depth_demand(&label)?);
                if self.labels.contains_key(&label) { (0, 0) }
                else {
                    let backing = self.labels.next_insert_capacity_byte_demand(&label, 0)?;
                    if backing != 0 { (0, backing) }
                    else {
                        let len = PersistentLabel(label).decimal(&mut [0; 20]).len();
                        let coordinates = if position.is_some() { 3 * size_of::<f32>() + size_of::<u32>() } else { 0 };
                        (len + coordinates, len)
                    }
                }
            }
            Stage::CountingSpline | Stage::SelectingSpline => {
                let member = self.current.expect("wire preview span traversal has a current member");
                depth = depth.max(Self::read_depth(body, member.edge)?);
                (size_of::<SplineSpan>(), 0)
            }
            Stage::Sampling | Stage::SamplingSpline(_) => {
                let member = self.current.expect("wire preview sampling has a current member");
                depth = depth.max(Self::read_depth(body, member.edge)?).max(self.labels.next_contains_depth_demand(&member.label)?);
                (6 * size_of::<f32>() + size_of::<u32>(), 0)
            }
        };
        Ok(RetirementDemand { depth, copy_bytes, capacity_bytes, ..Default::default() })
    }

    pub(super) fn step(&mut self, body: &Body, source: WirePreviewSource<'_>, revision: u64, deflection: f64, grant: Grant) -> Result<(WirePreviewStep, Progress), ValueError> {
        let demand = self.demand(body, source, revision)?;
        if grant.maximum_items == 0 || !retained_clone_grant_admits(grant, demand) { return Ok((WirePreviewStep::Working, Default::default())); }
        let mut progress = Progress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        match self.stage {
            Stage::Counting => {
                if let WirePreviewSource::Vertex(id)=source{
                    let row=body.vertices.get(id).ok_or_else(invalid)?;
                    self.vertex=Some(VertexSample{position:vector32(row.position)?,label:row.label.0});
                    self.stage=Stage::Reserve(0);
                }else if let Some((id, forward)) = source.member(self.member) {
                    let Some(member) = Self::member(body, id, forward, deflection)? else { return Ok((WirePreviewStep::Unsupported, progress)); };
                    if let Some(trim) = member.spline {
                        self.current = Some(member);
                        self.spline_cursor = Some(trim.cursor());
                        self.segment = 0;
                        self.stage = Stage::CountingSpline;
                    } else {
                        if !self.count_segments(member.segments) { return Ok((WirePreviewStep::Unsupported, progress)); }
                        self.member += 1;
                    }
                } else {
                    if self.total_segments == 0 { return Ok((WirePreviewStep::Unsupported, progress)); }
                    self.member = 0;
                    self.stage = Stage::Reserve(0);
                }
            }
            Stage::CountingSpline => {
                let member = self.current.ok_or_else(invalid)?;
                let curve = Self::curve(body, member.edge)?;
                match self.spline_cursor.as_mut().ok_or_else(invalid)?.next(curve, deflection) {
                    Some(SplineVisit::Span(span)) => {
                        if !self.count_segments(span.segments) { return Ok((WirePreviewStep::Unsupported, progress)); }
                        self.segment += span.segments;
                    }
                    Some(SplineVisit::Empty) => {}
                    Some(SplineVisit::End) if self.segment > 0 => self.finish_member(Stage::Counting),
                    _ => return Ok((WirePreviewStep::Unsupported, progress)),
                }
            }
            Stage::Reserve(slot) => {
                let count = source.len();
                let vertices=source.vertex_extent();
                progress.retained_capacity_bytes = match slot {
                    0 => reserve(&mut self.mesh.edge_positions, self.total_segments * 6)?,
                    1 => reserve(&mut self.mesh.edge_ids, self.total_segments)?,
                    2 => reserve(&mut self.mesh.positions, vertices * 3)?,
                    3 => reserve(&mut self.mesh.vertex_ids, vertices)?,
                    4 => reserve(&mut self.references, DOMAINS.len())?,
                    5 | 6 => {
                        let domain = text(DOMAINS[slot - 5])?;
                        let cap = domain.capacity();
                        self.references.push((domain, Vec::new()));
                        cap
                    }
                    7 => reserve(&mut self.references[0].1, count)?,
                    8 => reserve(&mut self.references[1].1, vertices)?,
                    _ => unreachable!("wire preview backing slot"),
                };
                self.stage = if slot == 8 { Stage::Reading } else { Stage::Reserve(slot + 1) };
            }
            Stage::Reading => {
                if self.vertex.is_some(){
                    self.stage=Stage::Labeling(0);
                    return Ok((WirePreviewStep::Working,progress));
                }
                let Some((edge, forward)) = source.member(self.member) else {
                    self.mesh.component_references = ComponentReferenceTable::from_entries(take(&mut self.references));
                    return Ok((WirePreviewStep::Ready(take(&mut self.mesh)), progress));
                };
                let member = Self::member(body, edge, forward, deflection)?.ok_or_else(|| ValueError::invariant("original wire member changed after admission"))?;
                if self.labels.contains_key(&member.label) {
                    self.member += 1;
                } else {
                    self.current = Some(member);
                    self.previous = member.ends[0].position;
                    self.segment = 0;
                    self.stage = Stage::Labeling(0);
                }
            }
            Stage::Labeling(index) => {
                let (label, position) = self.label(index);
                if !self.labels.contains_key(&label) {
                    if demand.copy_bytes == 0 {
                        let progress = self.labels.reserve_insert_step(&label, grant).map_err(|(error, progress)| error.with_retained_progress(progress))?;
                        return Ok((WirePreviewStep::Working, progress));
                    }
                    let domain = usize::from(position.is_some());
                    let names = &mut self.references[domain].1;
                    let id = names.len() as u32;
                    let name = text(PersistentLabel(label).decimal(&mut [0; 20]))?;
                    progress.retained_capacity_bytes = name.capacity();
                    names.push(name);
                    if !progress.fits(grant) {
                        return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "wire preview label allocation exceeded its grant").with_retained_progress(progress));
                    }
                    let (old, _) = self.labels.insert_reserved(label, id, grant).map_err(|(error, _, _)| error.with_retained_progress(progress))?;
                    if old.is_some() { return Err(ValueError::invariant("original wire preview label was already indexed").with_retained_progress(progress)); }
                    if let Some(position) = position {
                        self.mesh.positions.extend(position);
                        self.mesh.vertex_ids.push(id);
                    }
                }
                self.stage = if self.vertex.take().is_some() { Stage::Reading }
                    else if index < 2 { Stage::Labeling(index + 1) }
                    else if let Some(trim) = self.current.and_then(|member| member.spline) {
                        self.spline_cursor = Some(trim.cursor());
                        Stage::SelectingSpline
                    } else { Stage::Sampling };
            }
            Stage::Sampling => {
                let member = self.current.ok_or_else(invalid)?;
                let end = if self.segment + 1 == member.segments { member.ends[1].position }
                else {
                    let edge = body.edges.get(member.edge).ok_or_else(invalid)?;
                    let curve = body.curves3.get(edge.curve).ok_or_else(invalid)?;
                    let (a, b) = if member.forward { edge.range } else { (edge.range.1, edge.range.0) };
                    let t = (self.segment + 1) as f64 / member.segments as f64;
                    vector32(curve.eval(a + (b - a) * t))?
                };
                self.emit_segment(member, self.previous, end)?;
                self.segment += 1;
                self.previous = end;
                if self.segment == member.segments { self.finish_member(Stage::Reading); }
            }
            Stage::SelectingSpline => {
                let member = self.current.ok_or_else(invalid)?;
                let curve = Self::curve(body, member.edge)?;
                match self.spline_cursor.as_mut().ok_or_else(invalid)?.next(curve, deflection) {
                    Some(SplineVisit::Span(span)) => {
                        self.segment = 0;
                        self.stage = Stage::SamplingSpline(span);
                    }
                    Some(SplineVisit::Empty) => {}
                    Some(SplineVisit::End) => self.finish_member(Stage::Reading),
                    None => return Err(ValueError::invariant("wire preview spline span changed after admission")),
                }
            }
            Stage::SamplingSpline(span) => {
                let member = self.current.ok_or_else(invalid)?;
                let curve = Self::curve(body, member.edge)?;
                let point = |step| vector32(span.point(curve, step).ok_or_else(invalid)?);
                let a = if self.segment == 0 { point(0)? } else { self.previous };
                let b = point(self.segment + 1)?;
                self.emit_segment(member, a, b)?;
                self.previous = b;
                self.segment += 1;
                if self.segment == span.segments { self.stage = Stage::SelectingSpline; }
            }
        }
        if !progress.fits(grant) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "wire preview allocation exceeded its grant").with_retained_progress(progress));
        }
        Ok((WirePreviewStep::Working, progress))
    }
}
