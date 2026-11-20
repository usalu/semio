//! 📐️ Original solid pattern placements, separation proofs and multi-root rollback.

use super::*;
use semio_framework_3d_brep_vector::{Vec3, matrix::Affine3};

const MAX_MEMBERS: usize = 4096;
const CONTACT_TOLERANCE: f64 = 1e-6;

#[derive(Clone, Copy)]
enum Layout {
    Grid { row: Vec3, column: Vec3, columns: usize },
    Circular { axis: Vec3 },
}

/// 🧭️ A bounded procedural pattern in authored member order.
#[derive(Clone, Copy)]
pub struct SolidPatternRecipe {
    layout: Layout,
    count: usize,
}

impl SolidPatternRecipe {
    /// 📏️ Translates by successive multiples of the supplied direction and spacing.
    pub fn linear(direction: [f64; 3], spacing: f64, count: usize) -> Result<Self, ValueError> {
        Self::grid(direction, [0.0; 3], spacing, 0.0, count, 1)
    }

    /// 🔄️ Rotates each successive instance around the supplied world axis.
    pub fn circular(axis: [f64; 3], count: usize) -> Result<Self, ValueError> {
        let norm = axis[0].hypot(axis[1]).hypot(axis[2]);
        if !norm.is_finite() || norm <= Tol::DEFAULT.value() { return Err(invalid()); }
        Self::checked(Layout::Circular { axis: Vec3::from_array(axis.map(|v| v / norm)) }, count.max(1))
    }

    /// ▦️ Translates in row-major order along two independent authored directions.
    pub fn grid(x: [f64; 3], y: [f64; 3], sx: f64, sy: f64, nx: usize, ny: usize) -> Result<Self, ValueError> {
        let row = Vec3::from_array(x) * sx;
        let column = Vec3::from_array(y) * sy;
        if !row.to_array().into_iter().chain(column.to_array()).all(f64::is_finite) { return Err(invalid()); }
        let columns = ny.max(1);
        Self::checked(Layout::Grid { row, column, columns }, nx.max(1).saturating_mul(columns))
    }

    fn checked(layout: Layout, count: usize) -> Result<Self, ValueError> {
        if count > MAX_MEMBERS { return Err(invalid()); }
        Ok(Self { layout, count })
    }

    fn image(self, primitive: SolidPrimitive, index: usize) -> Result<SolidPrimitive, ValueError> {
        if index == 0 { return Ok(primitive); }
        let map = match self.layout {
            Layout::Grid { row, column, columns } => {
                let offset = row * (index / columns) as f64 + column * (index % columns) as f64;
                if !offset.to_array().into_iter().all(f64::is_finite) { return Err(invalid()); }
                Affine3::translation(offset)
            }
            Layout::Circular { axis } => Affine3::rotation_axis_angle(axis, std::f64::consts::TAU * index as f64 / self.count as f64),
        };
        let image = primitive.affinely_transformed(&map).ok_or_else(|| unsupported("pattern instance has no certified primitive image"))?;
        image.validate().map_err(|reason| ValueError::literal(ValueRefusalKind::InvalidValue, reason))?;
        Ok(image)
    }

    fn bounds(self, primitive: SolidPrimitive, index: usize) -> Result<[Pnt3; 2], ValueError> {
        let bounds = self.image(primitive, index)?.axis_aligned_bounds();
        if bounds.into_iter().flat_map(Pnt3::to_array).all(f64::is_finite) { Ok(bounds) } else { Err(invalid()) }
    }
}

fn separated([a, b]: [Pnt3; 2], [c, d]: [Pnt3; 2]) -> bool {
    let [a, b, c, d] = [a, b, c, d].map(Pnt3::to_array);
    (0..3).any(|i| {
        let scale = a[i].abs().max(b[i].abs()).max(c[i].abs()).max(d[i].abs());
        (c[i] - b[i]).max(a[i] - d[i]) > CONTACT_TOLERANCE.max(64.0 * f64::EPSILON * scale)
    })
}

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "pattern requires finite parameters and at most 4096 instances")
}

fn unsupported(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, message)
}

/// 🏗️ Holds completed and partial solid journals until the compound root claims them together.
pub struct UnpublishedSolidPattern {
    recipe: SolidPatternRecipe,
    primitive: SolidPrimitive,
    checked: usize,
    previous: usize,
    checking: Option<[Pnt3; 2]>,
    drafts: Vec<SolidPublication>,
    members: Vec<SolidId>,
    committed: bool,
}

impl UnpublishedSolidPattern {
    /// 🎟️ Starts an allocation-free pattern and separation cursor.
    pub fn new(recipe: SolidPatternRecipe, primitive: SolidPrimitive) -> Self {
        Self { recipe, primitive, checked: 0, previous: 0, checking: None, drafts: Vec::new(), members: Vec::new(), committed: false }
    }

    pub fn ready(&self) -> bool { !self.committed && self.members.len() == self.recipe.count }
    pub fn draft_count(&self) -> usize { self.drafts.len() }
    pub fn terminal_is_empty(&self) -> bool { self.drafts.capacity() == 0 && self.members.capacity() == 0 }

    pub fn take_roots(&mut self) -> Vec<SolidId> { take(&mut self.members) }
    pub fn restore_roots(&mut self, members: Vec<SolidId>) { self.members = members; }
    pub fn commit(&mut self) { self.committed = true; }

    fn next_draft(&self) -> Option<&SolidPublication> {
        (self.drafts.len() == self.members.len()).then(|| self.drafts.last()).flatten()
    }

    /// 📏️ Quotes one placement proof, arena action or retained backing reservation.
    pub fn demand(&self, body: &Body) -> Result<RetirementDemand, ValueError> {
        let mut demand = RetirementDemand { depth: 1, ..Default::default() };
        if self.checked < self.recipe.count {
            demand.copy_bytes = 4 * size_of::<Pnt3>();
        } else if self.members.capacity() == 0 {
            demand.capacity_bytes = self.recipe.count * size_of::<SolidId>();
        } else if self.members.is_empty() {
            demand.copy_bytes = size_of::<SolidId>();
        } else if self.drafts.capacity() == 0 && self.recipe.count > 1 {
            demand.capacity_bytes = (self.recipe.count - 1) * size_of::<SolidPublication>();
        } else if let Some(draft) = self.next_draft() {
            if !draft.topology_ready() { return draft.topology_demand(body); }
            demand.copy_bytes = size_of::<SolidId>();
        } else {
            demand.copy_bytes = size_of::<SolidPrimitive>();
        }
        Ok(demand)
    }

    /// ⏱️ Advances at most one pairwise proof, unpublished topology action, or member append.
    pub fn advance(&mut self, body: &mut Body, owner: usize, source: SolidId, demand: RetirementDemand,
                   handle: impl FnOnce(u64) -> [u8; 64], effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        let grant = effect.grant();
        if self.ready() || self.committed || grant.maximum_items == 0 || !retained_clone_grant_admits(grant, demand) {
            return Ok(Progress::default());
        }
        if self.next_draft().is_some_and(|draft| !draft.topology_ready()) {
            return self.drafts.last_mut().expect("active draft").advance(body, effect);
        }
        if !effect.performed(1)? { return Ok(Progress::default()); }
        let mut progress = Progress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        if self.checked < self.recipe.count {
            self.prove().map_err(|error| error.with_retained_progress(progress))?;
        } else if self.members.capacity() == 0 {
            self.members.try_reserve_exact(self.recipe.count).map_err(|_| allocation().with_retained_progress(progress))?;
            progress.retained_capacity_bytes = self.members.capacity() * size_of::<SolidId>();
        } else if self.members.is_empty() {
            self.members.push(source);
        } else if self.drafts.capacity() == 0 {
            self.drafts.try_reserve_exact(self.recipe.count - 1).map_err(|_| allocation().with_retained_progress(progress))?;
            progress.retained_capacity_bytes = self.drafts.capacity() * size_of::<SolidPublication>();
        } else if let Some(draft) = self.next_draft() {
            let (root, _) = draft.product(owner).map_err(|error| error.with_retained_progress(progress))?;
            self.members.push(root);
        } else {
            let primitive = self.recipe.image(self.primitive, self.members.len()).map_err(|error| error.with_retained_progress(progress))?;
            self.drafts.push(SolidPublication::begin(owner, SolidBoundary::Analytic(primitive), &mut body.labels, handle)
                .map_err(|error| error.with_retained_progress(progress))?);
        }
        if !progress.fits(grant) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "pattern backing exceeded its grant").with_retained_progress(progress));
        }
        Ok(progress)
    }

    fn prove(&mut self) -> Result<(), ValueError> {
        if let Some(candidate) = self.checking {
            let previous = self.recipe.bounds(self.primitive, self.previous)?;
            if !separated(previous, candidate) { return Err(unsupported("pattern instance bounds overlap or touch")); }
            self.previous += 1;
            if self.previous == self.checked {
                self.checked += 1;
                self.checking = None;
            }
        } else {
            let bounds = self.recipe.bounds(self.primitive, self.checked)?;
            if self.checked == 0 {
                self.checked = 1;
            } else {
                self.checking = Some(bounds);
                self.previous = 0;
            }
        }
        Ok(())
    }

    /// 🧹️ Quotes the next private rollback or public journal retirement.
    pub fn close_demand(&self, body: &Body) -> Result<RetirementDemand, ValueError> {
        if let Some(draft) = self.drafts.last() {
            if !self.committed { return draft.close_demand(body); }
            return Ok(RetirementDemand { release_bytes: draft.inserted.capacity() * size_of::<EntityRef>(), depth: 1, ..Default::default() });
        }
        let release_bytes = if self.drafts.capacity() != 0 { self.drafts.capacity() * size_of::<SolidPublication>() }
            else { self.members.capacity() * size_of::<SolidId>() };
        Ok(RetirementDemand { release_bytes, depth: 1, ..Default::default() })
    }

    /// ♻️ Releases one journal or rolls back one private topology action.
    pub fn close(&mut self, body: &mut Body, effect: &mut RetainedAllocationEffect<'_>) -> Result<Step, ValueError> {
        let grant = effect.grant();
        let demand = self.close_demand(body)?;
        if grant.maximum_items == 0 || !retained_clone_grant_admits(grant, demand) { return Ok(Step::Progress(Progress::default())); }
        let receipt = if let Some(draft) = self.drafts.last_mut() {
            if self.committed { draft.mark_published(); }
            let receipt = draft.close(body, effect)?.progress();
            if draft.terminal_is_empty() { self.drafts.pop(); }
            receipt
        } else {
            if !effect.performed(1)? { return Ok(Step::Progress(Progress::default())); }
            if self.drafts.capacity() != 0 { drop(take(&mut self.drafts)); }
            else { drop(take(&mut self.members)); }
            Progress { released_bytes: demand.release_bytes, ..item() }
        };
        Ok(if self.terminal_is_empty() { Step::Complete(receipt) } else { Step::Progress(receipt) })
    }
}

fn allocation() -> ValueError {
    ValueError::literal(ValueRefusalKind::AllocationFailed, "pattern backing allocation failed")
}
