//! 🧮️ Retained assembly of a second matrix in the original stiffness equation ordering.

use super::*;
use semio_s_fem_payload_turns::numerical_grant_covers;
use semio_s_fem_sparse::{SPARSE_INDEX_SPACE, reserve_funded_page};

const MAXIMUM_NODES: usize = 8;
const MAXIMUM_DOFS: usize = 6 * MAXIMUM_NODES;

#[derive(Clone, Copy)]
enum Phase {
    Begin,
    Position,
    Equation,
    Cell,
    Convert,
}

/// 🎼️ Collects finite, symmetric element contributions into a second free-equation matrix.
pub struct SecondaryAssembly {
    phase: Phase,
    element: usize,
    probe: usize,
    cursor: usize,
    node_count: usize,
    side: usize,
    positions: [[f64; 3]; MAXIMUM_NODES],
    equations: [Option<u32>; MAXIMUM_DOFS],
    displacement: [f64; MAXIMUM_DOFS],
    triplets: PagedList<SparseTriplet, SPARSE_INDEX_SPACE>,
    csr: Option<CsrTripletBuild>,
}

fn invalid(detail: &'static str) -> ValueError {
    ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, detail)
}

impl Default for SecondaryAssembly {
    fn default() -> Self {
        Self {
            phase: Phase::Begin,
            element: 0,
            probe: 0,
            cursor: 0,
            node_count: 0,
            side: 0,
            positions: [[0.0; 3]; MAXIMUM_NODES],
            equations: [None; MAXIMUM_DOFS],
            displacement: [0.0; MAXIMUM_DOFS],
            triplets: PagedList::empty(),
            csr: None,
        }
    }
}

impl SecondaryAssembly {
    /// 🎛️ Performs one lookup, contribution, or sparse conversion action.
    pub fn step(
        &mut self,
        stiffness: &AssemblyCsrBuild,
        reference: Option<&dyn Fn(usize) -> Option<f64>>,
        coefficient: impl FnOnce(&Elements, &[[f64; 3]], &[f64], usize, usize) -> Option<f64>,
        cx: &mut StepContext<'_>,
    ) -> Result<bool, OriginalTurnFailure> {
        if cx.should_yield() {
            return Ok(false);
        }
        if let Some(csr) = self.csr.as_mut() {
            let grant = cx.retained_grant();
            let Some((step, work)) = cx.with_original_effect(grant, |_, effect| csr.step_one_in(effect))? else {
                return Ok(false);
            };
            let (complete, progress) = step.map_err(|_| invalid("secondary matrix CSR conversion refused"))?;
            semio_framework_job::receive_original_retained_work(cx, semio_framework_value::RetainedWorkOutcome {
                result: Ok::<_, ValueError>(semio_framework_value::RetainedCloneStep::Progress(progress)),
                work,
            })?;
            return Ok(complete);
        }
        let primary = stiffness.assembly.as_ref().ok_or_else(|| invalid("secondary matrix requires the original assembly"))?;
        let grant = cx.retained_grant();
        let demand = RetirementDemand {
            copy_bytes: size_of::<Self>() + size_of::<SparseTriplet>(),
            depth: 1,
            ..Default::default()
        };
        if !numerical_grant_covers(grant, demand) {
            return Ok(false);
        }
        let matrix_order = primary.visual_free_order();
        let Some(element) = primary.model_element(self.element) else {
            if self.element != primary.model.elements_len() {
                return Err(invalid("secondary matrix lost an admitted element").into());
            }
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
            cx.consume_fuel(1);
            self.csr = Some(CsrTripletBuild::new(matrix_order, std::mem::take(&mut self.triplets)));
            self.phase = Phase::Convert;
            return Ok(false);
        };
        match self.phase {
            Phase::Begin => {
                let count = element.mounted_node_id_count()
                    .filter(|count| (1..=MAXIMUM_NODES).contains(count))
                    .ok_or_else(|| invalid("secondary matrix element node count"))?;
                let side = count * element.dofs_per_node().len();
                if !(1..=MAXIMUM_DOFS).contains(&side) {
                    return Err(invalid("secondary matrix element equation count").into());
                }
                self.node_count = count;
                self.side = side;
                self.phase = Phase::Position;
            }
            Phase::Position => {
                let node_id = element.mounted_node_id(self.cursor).ok_or_else(|| invalid("secondary matrix element node"))?;
                let plan = &primary.plan;
                let found = if plan.mounted_node_slots.is_empty() {
                    AssemblyIndexProbe::Found(*plan.node_slot.get(node_id).ok_or_else(|| invalid("secondary matrix missing node"))?)
                } else {
                    assembly_index_probe(&plan.mounted_node_slots, DofMap::hash(node_id, Dof::Tx), self.probe, grant.maximum_depth, |row| {
                        if primary.model.node_depth(row)? > grant.maximum_depth {
                            return Ok(None);
                        }
                        Ok(Some(primary.model_node(row).ok_or(FemError::Singular)?.id == node_id))
                    }).map_err(|_| invalid("secondary matrix node lookup refused"))?
                };
                match found {
                    AssemblyIndexProbe::Found(row) => {
                        let position = primary.model_node(row).ok_or_else(|| invalid("secondary matrix missing position"))?.pos;
                        self.positions[self.cursor] = position;
                        self.cursor += 1;
                        self.probe = 0;
                        if self.cursor == self.node_count {
                            self.cursor = 0;
                            self.phase = Phase::Equation;
                        }
                    }
                    AssemblyIndexProbe::Collision => self.probe += 1,
                    AssemblyIndexProbe::Pending => return Ok(false),
                    AssemblyIndexProbe::Vacant(_) => return Err(invalid("secondary matrix missing node").into()),
                }
            }
            Phase::Equation => {
                let dofs = element.dofs_per_node();
                let node_id = element.mounted_node_id(self.cursor / dofs.len()).ok_or_else(|| invalid("secondary matrix element node"))?;
                let dof = dofs[self.cursor % dofs.len()];
                let found = assembly_dof_probe(&primary.plan.dof_map, node_id, dof, self.probe, grant.maximum_depth)
                    .map_err(|_| invalid("secondary matrix equation lookup refused"))?;
                match found {
                    AssemblyIndexProbe::Found(old) => {
                        let plan = &primary.plan;
                        if plan.inv_perm.next_get_depth_demand(old).map_err(|_| invalid("secondary matrix permutation"))? > grant.maximum_depth {
                            return Ok(false);
                        }
                        let new = *plan.inv_perm.get(old).ok_or_else(|| invalid("secondary matrix missing permutation"))?;
                        if plan.compact_of_new.next_get_depth_demand(new).map_err(|_| invalid("secondary matrix compact equation"))? > grant.maximum_depth {
                            return Ok(false);
                        }
                        let free = *plan.compact_of_new.get(new).ok_or_else(|| invalid("secondary matrix missing compact equation"))?;
                        let equation = free.map(u32::try_from).transpose().map_err(|_| invalid("secondary matrix equation extent"))?;
                        let displacement = match (reference, free) {
                            (Some(reference), Some(index)) => reference(index).filter(|value| value.is_finite()).ok_or_else(|| invalid("secondary matrix reference displacement unavailable"))?,
                            _ => 0.0,
                        };
                        self.equations[self.cursor] = equation;
                        self.displacement[self.cursor] = displacement;
                        self.cursor += 1;
                        self.probe = 0;
                        if self.cursor == self.side {
                            self.cursor = 0;
                            self.phase = Phase::Cell;
                        }
                    }
                    AssemblyIndexProbe::Collision => self.probe += 1,
                    AssemblyIndexProbe::Pending => return Ok(false),
                    AssemblyIndexProbe::Vacant(_) => return Err(invalid("secondary matrix missing equation").into()),
                }
            }
            Phase::Cell => {
                let row = self.cursor / self.side;
                let column = self.cursor % self.side;
                if let (Some(global_row), Some(global_column)) = (self.equations[row], self.equations[column]) {
                    let target = self.triplets.len() + 1;
                    if self.triplets.capacity() < target {
                        let target = self.triplets.whole_page_capacity(target);
                        reserve_funded_page(&mut self.triplets, target, cx).map_err(|_| invalid("secondary matrix capacity"))?;
                        return Ok(false);
                    }
                    if self.triplets.next_push_depth_demand().map_err(|_| invalid("secondary matrix entry depth"))? > grant.maximum_depth {
                        return Ok(false);
                    }
                    let value = coefficient(element, &self.positions[..self.node_count], &self.displacement[..self.side], row.min(column), row.max(column))
                        .filter(|value| value.is_finite()).ok_or_else(|| invalid("secondary matrix element coefficient refused"))?;
                    if value != 0.0 {
                        self.triplets.push_reserved(SparseTriplet {
                            sequence: self.triplets.len() as u64, row: global_row, col: global_column, value,
                        }).map_err(|_| invalid("secondary matrix entry refused"))?;
                    }
                }
                self.cursor += 1;
                if self.cursor == self.side * self.side {
                    self.cursor = 0;
                    self.element += 1;
                    self.phase = Phase::Begin;
                }
            }
            Phase::Convert => return Err(invalid("secondary matrix conversion owner disappeared").into()),
        }
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(false)
    }

    /// 🧾️ Transfers the completed matrix under its original retained grant.
    pub fn take_complete(&mut self, grant: RetainedCloneGrant) -> Option<(Csr, RetainedCloneProgress)> {
        self.csr.as_mut()?.take_complete(grant)
    }

    /// ♻️ Retires one item or backing page from an incomplete or transferred assembly.
    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if let Some(csr) = self.csr.as_mut() {
            return csr.close_step(maximum_bytes);
        }
        match semio_s_fem_sparse::close_paged_owner_step(&mut self.triplets, maximum_bytes) {
            Ok(Some((items, bytes))) => (false, items, bytes),
            Ok(None) => (true, 0, 0),
            Err(()) => (false, 0, 0),
        }
    }
}
