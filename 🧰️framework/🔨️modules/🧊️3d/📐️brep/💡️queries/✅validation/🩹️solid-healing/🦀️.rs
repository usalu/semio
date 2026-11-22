//! 🩹️ Staged repair of selected vertices supported by their incident curve endpoints.

use std::mem::size_of;
use semio_framework_3d_brep_representation::arena::{SolidId, VertexId};
use semio_framework_3d_brep_representation::topology::{Body, EntityRef, ReachabilityJob, ReachabilityStep};
use semio_framework_3d_brep_vector::Pnt3;
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress as Progress, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;

use super::BodyValidationJob;

const MAXIMUM_REPAIRS: usize = 16;

#[derive(Clone, Copy)]
struct Repair { vertex: VertexId, position: Pnt3 }
semio_framework_value::artifact_retire_leaf!(Repair);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Stage { Selected, Vertices, OtherSolids, SharedVertices, Validate, Ready }
semio_framework_value::artifact_retire_leaf!(Stage);

/// 🩹️ Keeps prospective coordinates private until selected topology and ownership validate.
pub struct SolidHealing {
    source: usize,
    revision: (u64, u64, u64),
    solid: SolidId,
    tolerance: f64,
    certified: bool,
    selected: ReachabilityJob,
    other: ReachabilityJob,
    validation: BodyValidationJob,
    stage: Stage,
    vertex_rank: usize,
    edge_rank: usize,
    endpoint: usize,
    other_slot: usize,
    repair_count: usize,
    candidate: Option<Pnt3>,
    needs_repair: bool,
    repairs: [Option<Repair>; MAXIMUM_REPAIRS],
}
semio_framework_value::artifact_retire_struct!(SolidHealing {
    source, revision, solid, tolerance, certified, selected, other, validation, stage,
    vertex_rank, edge_rank, endpoint, other_slot, repair_count, candidate, needs_repair, repairs
});

impl SolidHealing {
    /// 🎟️ Opens a repair cursor over one original solid.
    pub fn new(body: &Body, solid: SolidId, revision: (u64, u64, u64), certified: bool, tolerance: f64) -> Result<Self, ValueError> {
        if !tolerance.is_finite() || tolerance <= 0.0 {
            return Err(invalid("healing tolerance must be finite and positive"));
        }
        let mut selected = ReachabilityJob::new();
        selected.begin_root(EntityRef::Solid(solid)).map_err(|_| changed())?;
        Ok(Self {
            source: std::ptr::from_ref(body) as usize, solid, revision, tolerance, certified,
            selected, other: ReachabilityJob::new(), validation: BodyValidationJob::new(body), stage: Stage::Selected,
            vertex_rank: 0, edge_rank: 0, endpoint: 0, other_slot: 0, repair_count: 0, candidate: None, needs_repair: false, repairs: [None; MAXIMUM_REPAIRS],
        })
    }

    pub fn phase(&self) -> &'static str {
        match self.stage {
            Stage::Selected => "healing-selected-scope",
            Stage::Vertices => "healing-vertices",
            Stage::OtherSolids | Stage::SharedVertices => "healing-shared-ownership",
            Stage::Validate => "healing-validation",
            Stage::Ready => "healing-publication",
        }
    }

    pub fn ready(&self) -> bool { self.stage == Stage::Ready }
    pub fn targets(&self, solid: SolidId) -> bool { self.solid == solid }

    fn check(&self, body: &Body, revision: (u64, u64, u64)) -> Result<(), ValueError> {
        if self.source != std::ptr::from_ref(body) as usize || self.revision != revision || body.solids.get(self.solid).is_none() {
            return Err(changed());
        }
        Ok(())
    }

    /// 🧮️ Quotes the next repair event.
    pub fn demand(&self, body: &Body, revision: (u64, u64, u64), copy: usize) -> Result<RetirementDemand, ValueError> {
        self.check(body, revision)?;
        let scope = self.selected.keep();
        let depth = match self.stage {
            Stage::Selected => return walk_demand(&self.selected, copy),
            Stage::Vertices => {
                let Some((&id, ())) = scope.vertices.0.ranked(self.vertex_rank) else { return Ok(metadata(1)); };
                let mut demand = RetirementDemand {
                    depth: scope.vertices.0.next_rank_depth_demand(self.vertex_rank)?.max(body.vertices.next_remove_depth_demand(id)?),
                    ..Default::default()
                };
                if let Some((&edge, ())) = scope.edges.0.ranked(self.edge_rank) {
                    demand.depth = demand.depth.max(scope.edges.0.next_rank_depth_demand(self.edge_rank)?).max(body.edges.next_remove_depth_demand(edge)?);
                    let edge = body.edges.get(edge).ok_or_else(changed)?;
                    if [edge.v0, edge.v1][self.endpoint] == id {
                        demand.depth = demand.depth.max(body.curves3.next_remove_depth_demand(edge.curve)?);
                        if self.candidate.is_none() { demand.copy_bytes = size_of::<Pnt3>(); }
                    }
                } else if self.needs_repair {
                    demand.copy_bytes = size_of::<Repair>();
                }
                return Ok(demand);
            }
            Stage::OtherSolids if !self.other.walk_is_complete() => return walk_demand(&self.other, copy),
            Stage::OtherSolids => 1,
            Stage::SharedVertices => self.repairs.get(self.endpoint).copied().flatten()
                .map(|repair| self.other.keep().vertices.0.next_contains_depth_demand(&repair.vertex)).transpose()?.unwrap_or(1),
            Stage::Validate => return self.validation.funded_demands_for(body, Some(scope)),
            Stage::Ready => {
                let depth = self.repairs.iter().flatten().try_fold(1, |depth, repair|
                    Ok::<_, ValueError>(depth.max(body.vertices.next_remove_depth_demand(repair.vertex)?)))?;
                return Ok(RetirementDemand { depth, copy_bytes: self.repair_count * size_of::<Pnt3>(), ..Default::default() });
            }
        };
        Ok(metadata(depth))
    }

    /// ⏱️ Advances one repair event without changing the source.
    pub fn step(&mut self, body: &Body, revision: (u64, u64, u64), effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        let grant = effect.grant();
        let demand = self.demand(body, revision, grant.maximum_copy_bytes)?;
        if grant.maximum_items == 0 || self.ready() || !retained_clone_grant_admits(grant, demand) {
            return Ok(Progress::default());
        }
        if self.stage == Stage::Validate {
            let repairs = &self.repairs;
            let positions = |vertex, original| {
                repairs.iter().flatten().find(|repair| repair.vertex == vertex).map_or(original, |repair| repair.position)
            };
            let (receipt, _) = self.validation.step_funded_with_positions(body, Some(self.selected.keep()), 1, effect, &positions)?;
            if self.validation.issue_count() > 0 {
                return Err(invalid("healing could not validate the selected solid").with_retained_progress(receipt));
            }
            if self.validation.passed() { self.stage = Stage::Ready; }
            return Ok(receipt);
        }
        if !effect.performed(1)? { return Ok(Progress::default()); }
        match self.stage {
            Stage::Selected => {
                let receipt = walk_step(&mut self.selected, body, grant)?;
                if self.selected.walk_is_complete() {
                    let scope = self.selected.keep();
                    if scope.vertices.0.is_empty() || scope.edges.0.is_empty() || scope.faces.0.is_empty() {
                        return Err(invalid("selected solid has incomplete topology").with_retained_progress(receipt));
                    }
                    self.stage = Stage::Vertices;
                }
                return Ok(receipt);
            }
            Stage::Vertices => return self.probe_vertex(body),
            Stage::OtherSolids if !self.other.walk_is_complete() => return walk_step(&mut self.other, body, grant),
            Stage::OtherSolids => {
                if self.other_slot == body.solids.slot_count() {
                    self.stage = Stage::SharedVertices;
                    self.endpoint = 0;
                } else {
                    if let Some((id, _)) = body.solids.slot_at(self.other_slot).filter(|(id, _)| *id != self.solid) {
                        self.other.begin_root(EntityRef::Solid(id)).map_err(|_| changed())?;
                    }
                    self.other_slot += 1;
                }
            }
            Stage::SharedVertices => {
                if let Some(repair) = self.repairs.get(self.endpoint).copied().flatten() {
                    if self.other.keep().vertices.contains(&repair.vertex) {
                        return Err(invalid("healing would move a vertex owned by another solid"));
                    }
                    self.endpoint += 1;
                } else { self.stage = Stage::Validate; }
            }
            Stage::Ready | Stage::Validate => unreachable!(),
        }
        Ok(item(0))
    }

    fn probe_vertex(&mut self, body: &Body) -> Result<Progress, ValueError> {
        let scope = self.selected.keep();
        let Some((&id, ())) = scope.vertices.0.ranked(self.vertex_rank) else {
            self.stage = if self.repair_count != 0 { Stage::OtherSolids }
                else if self.certified { Stage::Ready } else { Stage::Validate };
            return Ok(item(0));
        };
        let vertex = body.vertices.get(id).ok_or_else(changed)?;
        let Some((&edge_id, ())) = scope.edges.0.ranked(self.edge_rank) else {
            let mut receipt = item(0);
            let candidate = self.candidate.take().ok_or_else(|| invalid("selected vertex has no incident curve endpoint"))?;
            if std::mem::take(&mut self.needs_repair) {
                let position = candidate;
                if vertex.position.distance(position) > self.tolerance {
                    return Err(invalid("vertex lies outside the selected healing tolerance"));
                }
                let repair = self.repairs.get_mut(self.repair_count).ok_or_else(|| ValueError::literal(ValueRefusalKind::WorkLimit, "too many vertex repairs for one atomic healing publication"))?;
                *repair = Some(Repair { vertex: id, position });
                self.repair_count += 1;
                receipt.copied_bytes = size_of::<Repair>();
            }
            self.vertex_rank += 1;
            self.edge_rank = 0;
            return Ok(receipt);
        };
        let edge = body.edges.get(edge_id).ok_or_else(changed)?;
        let endpoint = self.endpoint;
        self.endpoint = (endpoint + 1) % 2;
        if self.endpoint == 0 { self.edge_rank += 1; }
        if [edge.v0, edge.v1][endpoint] != id { return Ok(item(0)); }
        let parameter = [edge.range.0, edge.range.1][endpoint];
        let curve = body.curves3.get(edge.curve).ok_or_else(changed)?;
        let (position, _, _) = curve.jet_original(parameter, 0).ok_or_else(|| ValueError::literal(ValueRefusalKind::UnsupportedOwner, "healing cannot evaluate this curve endpoint under the native work bound"))?;
        let deviation = vertex.position.distance(position);
        if !deviation.is_finite() { return Err(invalid("selected vertex position is nonfinite")); }
        self.needs_repair |= deviation > vertex.tol.value();
        if let Some(candidate) = self.candidate {
            if candidate.distance(position) > vertex.tol.value() {
                return Err(invalid("incident curve endpoints disagree at the selected vertex"));
            }
            Ok(item(0))
        } else {
            self.candidate = Some(position);
            Ok(item(size_of::<Pnt3>()))
        }
    }

    /// 📤️ Publishes validated coordinates together in the original solid arena.
    pub fn commit(&mut self, body: &mut Body, revision: (u64, u64, u64), effect: &mut RetainedAllocationEffect<'_>) -> Result<(bool, bool, Progress), ValueError> {
        let grant = effect.grant();
        let demand = self.demand(body, revision, grant.maximum_copy_bytes)?;
        if !self.ready() || grant.maximum_items == 0 || !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? {
            return Ok((false, false, Progress::default()));
        }
        for repair in self.repairs.iter().flatten() {
            if body.vertices.get(repair.vertex).is_none() { return Err(changed()); }
        }
        for repair in self.repairs.iter().flatten() {
            body.vertices.get_mut(repair.vertex).expect("validated repair vertex").position = repair.position;
        }
        Ok((true, self.repair_count != 0, item(demand.copy_bytes)))
    }
}

fn changed() -> ValueError { ValueError::invariant("selected healing source changed") }
fn invalid(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, message) }
fn metadata(depth: usize) -> RetirementDemand { RetirementDemand { depth, ..Default::default() } }
fn item(copied_bytes: usize) -> Progress { Progress { copied_items: 1, copied_bytes, ..Default::default() } }

fn walk_demand(walk: &ReachabilityJob, copy: usize) -> Result<RetirementDemand, ValueError> {
    Ok(RetirementDemand {
        copy_bytes: walk.next_copy_byte_demand()?, capacity_bytes: walk.next_capacity_byte_demand(copy)?,
        release_bytes: walk.next_release_byte_demand()?, depth: walk.next_depth_demand()?,
    })
}

fn walk_step(walk: &mut ReachabilityJob, body: &Body, grant: semio_framework_value::RetainedCloneGrant) -> Result<Progress, ValueError> {
    match walk.step(body, grant)? {
        ReachabilityStep::Working(receipt) | ReachabilityStep::Ready(receipt) => Ok(receipt),
        ReachabilityStep::Failed { error, progress } => Err(error.with_retained_progress(progress)),
        ReachabilityStep::Cancelled(receipt) => Err(changed().with_retained_progress(receipt)),
    }
}
