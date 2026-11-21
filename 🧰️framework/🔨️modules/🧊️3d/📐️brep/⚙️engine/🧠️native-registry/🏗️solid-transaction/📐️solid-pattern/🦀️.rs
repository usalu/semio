//! 📐️ Original solid pattern placements, separation proofs and multi-root rollback.

use super::*;

pub use semio_framework_3d_brep_representation::contract::SolidPatternRecipe;

const CONTACT_TOLERANCE: f64 = 1e-6;

fn image(recipe:SolidPatternRecipe,primitive:SolidPrimitive,index:usize)->Result<SolidPrimitive,ValueError>{
    if index == 0 { return Ok(primitive); }
    let map=recipe.transform(index)?;
    let image = primitive.affinely_transformed(&map).ok_or_else(|| unsupported("pattern instance has no certified primitive image"))?;
    image.validate().map_err(|reason| ValueError::literal(ValueRefusalKind::InvalidValue, reason))?;
    Ok(image)
}

fn bounds(recipe:SolidPatternRecipe,primitive:SolidPrimitive,index:usize)->Result<[Pnt3;2],ValueError>{
    let bounds=image(recipe,primitive,index)?.axis_aligned_bounds();
    if bounds.into_iter().flat_map(Pnt3::to_array).all(f64::is_finite) { Ok(bounds) } else { Err(invalid()) }
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

/// 🪪️ Binds a solid group's pending work to at most two original native sources.
pub struct SolidGroupOwnership {
    owner:usize,
    revision:u64,
    root:SolidId,
    pattern_key:Option<[u8;64]>,
    secondary:Option<SolidId>,
}

impl SolidGroupOwnership {
    pub fn new(owner:usize,revision:u64,root:SolidId,pattern_key:Option<&str>,secondary:Option<SolidId>)->Result<Self,ValueError>{
        let pattern_key=pattern_key.map(|key|key.as_bytes().try_into().map_err(|_|ValueError::invariant("invalid pattern source address"))).transpose()?;
        Ok(Self{owner,revision,root,pattern_key,secondary})
    }

    pub fn owner(&self)->usize{self.owner}
    pub fn root(&self)->SolidId{self.root}

    pub fn depth(&self,owner:usize,revision:u64,mut read:impl FnMut(SolidId,Option<&str>)->Result<usize,ValueError>)->Result<usize,ValueError>{
        if self.owner!=owner || self.revision!=revision {return Err(ValueError::invariant("original solid-group source changed"));}
        let key=self.pattern_key.as_ref().map(|key|std::str::from_utf8(key).expect("source is a canonical digest"));
        let depth=read(self.root,key)?;
        if let Some(other)=self.secondary { Ok(depth.max(read(other,None)?)) } else { Ok(depth) }
    }
}

enum GroupRecipe {
    Pattern(SolidPatternRecipe,SolidPrimitive),
    Disconnected(BoxPrismBatch),
}

/// 🏗️ Holds completed and partial solid journals until the compound root claims them together.
pub struct UnpublishedSolidPattern {
    recipe: GroupRecipe,
    count: usize,
    borrowed: usize,
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
        Self::group(GroupRecipe::Pattern(recipe,primitive),recipe.count(),1)
    }

    /// 🧩️ Starts one batch of independently closed certified material pieces.
    pub fn disconnected(batch: BoxPrismBatch) -> Self {
        let count=batch.count();
        Self::group(GroupRecipe::Disconnected(batch),count,0)
    }

    fn group(recipe:GroupRecipe,count:usize,borrowed:usize)->Self {
        Self { recipe, count, borrowed, checked: 0, previous: 0, checking: None, drafts: Vec::new(), members: Vec::new(), committed: false }
    }

    fn needs_proof(&self)->bool { self.borrowed!=0 && self.checked<self.count }
    pub fn committed(&self)->bool { self.committed }
    pub fn channel(&self)->&'static str{if self.borrowed==0 {"solid"} else {"compound"}}

    pub fn ready(&self) -> bool { !self.committed && self.members.len() == self.count }
    pub fn draft_count(&self) -> usize { self.drafts.len() }
    pub fn terminal_is_empty(&self) -> bool { self.drafts.capacity() == 0 && self.members.capacity() == 0 }

    pub fn take_roots(&mut self) -> Vec<SolidId> { take(&mut self.members) }
    pub fn restore_roots(&mut self, members: Vec<SolidId>) { self.members = members; }
    pub fn commit(&mut self) -> usize { self.committed = true; self.draft_count() }

    fn next_draft(&self) -> Option<&SolidPublication> {
        (self.drafts.len()+self.borrowed>self.members.len()).then(|| self.drafts.last()).flatten()
    }

    /// 📏️ Quotes one placement proof, arena action or retained backing reservation.
    pub fn demand(&self, body: &Body) -> Result<RetirementDemand, ValueError> {
        let mut demand = RetirementDemand { depth: 1, ..Default::default() };
        if self.needs_proof() {
            demand.copy_bytes = 4 * size_of::<Pnt3>();
        } else if self.members.capacity() == 0 {
            demand.capacity_bytes = self.count * size_of::<SolidId>();
        } else if self.borrowed!=0 && self.members.is_empty() {
            demand.copy_bytes = size_of::<SolidId>();
        } else if self.drafts.capacity() == 0 && self.count > self.borrowed {
            demand.capacity_bytes = (self.count - self.borrowed) * size_of::<SolidPublication>();
        } else if let Some(draft) = self.next_draft() {
            if !draft.topology_ready() { return draft.topology_demand(body); }
            demand.copy_bytes = size_of::<SolidId>();
        } else {
            demand.copy_bytes = match &self.recipe {GroupRecipe::Disconnected(batch)=>batch.copy_bytes(),GroupRecipe::Pattern(..)=>size_of::<SolidPrimitive>()};
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
        if self.needs_proof() {
            self.prove().map_err(|error| error.with_retained_progress(progress))?;
        } else if self.members.capacity() == 0 {
            self.members.try_reserve_exact(self.count).map_err(|_| allocation().with_retained_progress(progress))?;
            progress.retained_capacity_bytes = self.members.capacity() * size_of::<SolidId>();
        } else if self.borrowed!=0 && self.members.is_empty() {
            self.members.push(source);
        } else if self.drafts.capacity() == 0 {
            self.drafts.try_reserve_exact(self.count - self.borrowed).map_err(|_| allocation().with_retained_progress(progress))?;
            progress.retained_capacity_bytes = self.drafts.capacity() * size_of::<SolidPublication>();
        } else if let Some(draft) = self.next_draft() {
            let (root, _) = draft.product(owner).map_err(|error| error.with_retained_progress(progress))?;
            self.members.push(root);
        } else {
            let boundary=match &self.recipe {
                GroupRecipe::Pattern(recipe,primitive)=>image(*recipe,*primitive,self.members.len()).map(SolidBoundary::Analytic),
                GroupRecipe::Disconnected(batch)=>batch.member(self.members.len()).ok_or_else(||unsupported("Boolean component exceeds the exact solid domain")),
            }.map_err(|error|error.with_retained_progress(progress))?;
            self.drafts.push(SolidPublication::begin(owner, boundary, &mut body.labels, handle)
                .map_err(|error| error.with_retained_progress(progress))?);
        }
        if !progress.fits(grant) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "pattern backing exceeded its grant").with_retained_progress(progress));
        }
        Ok(progress)
    }

    fn prove(&mut self) -> Result<(), ValueError> {
        let GroupRecipe::Pattern(recipe,primitive)=self.recipe else {unreachable!("batch needs no placement proof")};
        if let Some(candidate) = self.checking {
            let previous = bounds(recipe,primitive, self.previous)?;
            if !separated(previous, candidate) { return Err(unsupported("pattern instance bounds overlap or touch")); }
            self.previous += 1;
            if self.previous == self.checked {
                self.checked += 1;
                self.checking = None;
            }
        } else {
            let bounds = bounds(recipe,primitive, self.checked)?;
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
