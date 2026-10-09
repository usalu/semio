//! 🧵️ Bounded transient publication plus lease-aware exact-store disposal.

use crate::app::{ArtifactOwnedDisposer, PluginLifecycleStep};
use crate::{protocol, store};
use semio_framework::{Fault, FaultCode, FaultOrigin};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::sync::{Arc, Weak};

fn paged_copy(retained_bytes: &mut usize, grant: RetainedCloneGrant) -> Option<RetainedCloneProgress> {
    let copied_bytes = (*retained_bytes).min(grant.maximum_copy_bytes);
    *retained_bytes -= copied_bytes;
    (copied_bytes != 0).then_some(RetainedCloneProgress { copied_items: 1, copied_bytes, ..Default::default() })
}

fn depth_refusal(grant: RetainedCloneGrant, demand: RetirementDemand, scope: &'static str) -> Result<(), ValueError> {
    if grant.maximum_depth < demand.depth { Err(ValueError::literal(ValueRefusalKind::DepthLimit, scope)) } else { Ok(()) }
}

fn yields(grant: RetainedCloneGrant, demand: RetirementDemand) -> bool {
    grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes
}

struct BoundedTransientPreparation<P, M> {
    request: Option<store::ArtifactEphemeralOneItemPreparationRequest<P, M>>,
    prepared: Option<store::ArtifactEphemeralOneItemPrepared<P>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
}

impl<P, M> BoundedTransientPreparation<P, M> {
    fn demand(&self) -> RetirementDemand {
        RetirementDemand { copy_bytes: usize::from(self.retained_bytes != 0), depth: usize::from(!self.terminal_is_empty()), ..Default::default() }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.request.is_none() && self.prepared.is_none() && self.retained_bytes == 0
    }
}

struct BoundedTransientPreparationFactory<P, M>(std::marker::PhantomData<fn() -> (P, M)>);

impl<P, M> Default for BoundedTransientPreparationFactory<P, M> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<P, M> store::ArtifactEphemeralOneItemPreparationFactory<P, M> for BoundedTransientPreparationFactory<P, M>
where
    P: Clone + Send + Sync + 'static,
    M: protocol::Mutation<P> + protocol::OpBinary + Send + 'static,
{
    fn preflight(&self, mutation: &M) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        let retained_bytes = protocol::OpBinary::encode_op(mutation).map_err(|error| error.to_string())?.len();
        let footprint = store::ArtifactStoreOneItemFootprint::for_ephemeral_item(retained_bytes);
        footprint.is_admissible().then_some(footprint).ok_or_else(|| "transient mutation exceeds the one-item publication bound".into())
    }

    fn begin(&self, request: store::ArtifactEphemeralOneItemPreparationRequest<P, M>) -> Result<Box<dyn store::ArtifactEphemeralOneItemPreparation<P, M>>, store::ArtifactEphemeralOneItemPreparationRequest<P, M>> {
        let retained_bytes = protocol::OpBinary::encode_op(&request.mutation).map_or(store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, |bytes| bytes.len());
        Ok(Box::new(BoundedTransientPreparation { request: Some(request), prepared: None, checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), retained_bytes, cancelled: false, closing: false }))
    }
}

impl<P, M> store::ArtifactEphemeralOneItemPreparation<P, M> for BoundedTransientPreparation<P, M>
where
    P: Clone + Send + Sync + 'static,
    M: protocol::Mutation<P> + Send + 'static,
{
    /// 🎒️ The byte cost is admitted by `preflight` against the one-item bound, so the per-turn grant
    /// PACES the ladder rather than authorising the apply. Gating the apply on
    /// `grant.maximum_bytes >= retained_bytes` deadlocked every transient root larger than one
    /// publication page (`TYPED_OPERATION_RESULT_PAGE_BYTES`, 4 KiB): `advance_publish_one` returned
    /// `Blocked` forever, with no fault and no progress, so the typed operation never retired —
    /// measured on lowpoly, whose transient carries the live mesh workspace (ticket
    /// 26/08/29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS, 2026-09-17).
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if self.cancelled || !grant.permits_one() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_none() {
            let request = self.request.take().ok_or_else(|| "transient preparation lost its request".to_string())?;
            let outcome = protocol::Mutation::diff(&request.mutation, request.base.as_ref());
            if outcome.worst_level().is_some_and(|level| level >= semio_framework_diagnostic::Severity::Error) {
                return Err("transient mutation was rejected against its captured base".into());
            }
            let next_root = protocol::apply_diff(outcome.diff(), request.base.as_ref()).map_err(|error| error.to_string())?;
            self.prepared = Some(store::ArtifactEphemeralOneItemPrepared { next_root: Arc::new(next_root) });
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: [0; 32] };
        }
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactEphemeralOneItemPrepared<P>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactEphemeralOneItemPrepared<P>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    /// 🎒️ Pages its byte accounting on the copy axis instead of demanding the whole root in one grant — the closing
    /// half of the same deadlock `advance` documents above.
    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, ValueError> {
        let grant = grant.retained_grant();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        let demand = self.demand();
        depth_refusal(grant, demand, "transient preparation close exceeds admitted depth")?;
        if !self.closing || yields(grant, demand) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if let Some(progress) = paged_copy(&mut self.retained_bytes, grant) {
            return Ok(RetainedCloneStep::Progress(progress));
        }
        drop(self.prepared.take());
        drop(self.request.take());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.demand().copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, ValueError> {
        Ok(0)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(0)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.demand().depth)
    }

    fn terminal_is_empty(&self) -> bool {
        BoundedTransientPreparation::terminal_is_empty(self)
    }
}

struct BoundedTransientRootRetirement<P> {
    root: Option<Arc<P>>,
    retained_bytes: usize,
}

impl<P> BoundedTransientRootRetirement<P> {
    fn demand(&self) -> RetirementDemand {
        RetirementDemand { copy_bytes: usize::from(self.retained_bytes != 0), depth: usize::from(!self.terminal_is_empty()), ..Default::default() }
    }

    fn terminal_is_empty(&self) -> bool {
        self.root.is_none() && self.retained_bytes == 0
    }
}

impl<P: Send + Sync + 'static> store::ErasedSnapshotRetirement for BoundedTransientRootRetirement<P> {
    /// 🎒️ Pages its byte accounting on the copy axis: a displaced transient root larger than one grant (lowpoly's mesh
    /// workspace) otherwise never retires, and every later publication blocks behind it.
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        let demand = self.demand();
        depth_refusal(grant, demand, "transient root retirement exceeds admitted depth")?;
        if yields(grant, demand) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if let Some(progress) = paged_copy(&mut self.retained_bytes, grant) {
            return Ok(RetainedCloneStep::Progress(progress));
        }
        drop(self.root.take());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    fn terminal_is_empty(&self) -> bool {
        BoundedTransientRootRetirement::terminal_is_empty(self)
    }

    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.demand().copy_bytes)
    }

    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, ValueError> {
        Ok(0)
    }

    fn next_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(0)
    }

    fn next_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.demand().depth)
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct BoundedTransientRootRetirementFactory<P>(std::marker::PhantomData<fn() -> P>);

impl<P> Default for BoundedTransientRootRetirementFactory<P> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<P> store::SnapshotRetirementFactory<P> for BoundedTransientRootRetirementFactory<P>
where
    P: store::ArtifactDsl + Send + Sync + 'static,
{
    fn retirement_birth_bytes(&self, _snapshot: &Arc<P>) -> usize { std::mem::size_of::<BoundedTransientRootRetirement<P>>() }

    fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Arc<P>)> {
        let progress = match (RetainedCloneBirthDemand { capacity_bytes: std::mem::size_of::<BoundedTransientRootRetirement<P>>(), depth: 1 }).admit(grant) {
            Ok(progress) => progress,
            Err(error) => return Err((error, snapshot)),
        };
        let retained_bytes = store::ArtifactDsl::print_dsl(snapshot.as_ref()).len();
        Ok((Box::new(BoundedTransientRootRetirement { root: Some(snapshot), retained_bytes }), progress))
    }
}

struct BoundedTransientStoreDisposer<P, M> {
    retired: Option<store::TransientStore<P, M>>,
    retained_bytes: usize,
    terminal_root: Option<Weak<P>>,
    terminal_generation: u64,
}

impl<P, M> Default for BoundedTransientStoreDisposer<P, M> {
    fn default() -> Self {
        Self { retired: None, retained_bytes: 0, terminal_root: None, terminal_generation: 0 }
    }
}

impl<P, M> BoundedTransientStoreDisposer<P, M>
where
    P: Clone + Default,
    M: protocol::Mutation<P>,
{
    fn owns_terminal(&self, owner: &store::TransientStore<P, M>) -> bool {
        owner.generation_now() == self.terminal_generation && self.terminal_root.as_ref().and_then(Weak::upgrade).is_some_and(|root| Arc::ptr_eq(&root, &owner.current_root()))
    }
}

impl<P, M> ArtifactOwnedDisposer<store::TransientStore<P, M>> for BoundedTransientStoreDisposer<P, M>
where
    P: Clone + Default + store::ArtifactDsl + Send + Sync + 'static,
    M: protocol::Mutation<P> + Send + 'static,
{
    /// 🎒️ Same paging as the root retirement above.
    fn close_step(&mut self, owner: &mut store::TransientStore<P, M>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        let demand = self.retirement_demands(owner, grant.maximum_copy_bytes).map_err(|error| Fault::from(error.into_message()))?;
        if grant.maximum_depth < demand.depth {
            return Err(Fault::from("transient disposal exceeds admitted depth"));
        }
        if self.terminal_root.is_some() && self.retired.is_none() {
            return self.owns_terminal(owner).then(|| PluginLifecycleStep::Complete(Default::default())).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("transient.disposer-authority"), "transient terminal owner changed during disposal"));
        }
        if yields(grant, demand) {
            return Ok(PluginLifecycleStep::Progress(Default::default()));
        }
        if self.terminal_root.is_none() {
            self.retained_bytes = store::ArtifactDsl::print_dsl(owner.current_root().as_ref()).len();
            self.retired = Some(std::mem::replace(owner, store::TransientStore::new(P::default())));
            self.terminal_root = Some(Arc::downgrade(&owner.current_root()));
            self.terminal_generation = owner.generation_now();
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if let Some(progress) = paged_copy(&mut self.retained_bytes, grant) {
            return Ok(PluginLifecycleStep::Progress(progress));
        }
        self.retired = None;
        let progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        Ok(if self.owns_terminal(owner) { PluginLifecycleStep::Complete(progress) } else { PluginLifecycleStep::Progress(progress) })
    }

    fn retirement_demands(&self, _owner: &store::TransientStore<P, M>, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(if self.terminal_root.is_none() {
            RetirementDemand { depth: 1, ..Default::default() }
        } else if self.retired.is_some() {
            RetirementDemand { copy_bytes: usize::from(self.retained_bytes != 0), depth: 1, ..Default::default() }
        } else {
            Default::default()
        })
    }

    fn terminal_is_empty(&self, owner: &store::TransientStore<P, M>) -> bool {
        self.retired.is_none() && self.owns_terminal(owner)
    }
}

pub fn bounded_transient_preparation_factory<P, M>() -> Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<P, M>>
where
    P: Clone + Send + Sync + 'static,
    M: protocol::Mutation<P> + protocol::OpBinary + Send + 'static,
{
    Arc::new(BoundedTransientPreparationFactory::<P, M>::default())
}

pub fn bounded_transient_root_retirement_factory<P>() -> Arc<dyn store::SnapshotRetirementFactory<P>>
where
    P: store::ArtifactDsl + Send + Sync + 'static,
{
    Arc::new(BoundedTransientRootRetirementFactory::<P>::default())
}

pub fn bounded_transient_store_disposer<P, M>() -> Box<dyn ArtifactOwnedDisposer<store::TransientStore<P, M>>>
where
    P: Clone + Default + store::ArtifactDsl + Send + Sync + 'static,
    M: protocol::Mutation<P> + Send + 'static,
{
    Box::new(BoundedTransientStoreDisposer::<P, M>::default())
}

pub struct TransientStoreDisposer<P, M> {
    retirement: Option<store::TransientStoreRetirement<P>>,
    terminal_root: Option<Weak<P>>,
    terminal_generation: u64,
    factory: Arc<dyn store::ArtifactOwnedValueRetirementFactory<P>>,
    marker: std::marker::PhantomData<fn() -> M>,
}

impl<P, M> TransientStoreDisposer<P, M> {
    pub fn new(factory: Arc<dyn store::ArtifactOwnedValueRetirementFactory<P>>) -> Self {
        Self { retirement: None, terminal_root: None, terminal_generation: 0, factory, marker: std::marker::PhantomData }
    }
}

impl<P, M> TransientStoreDisposer<P, M>
where
    P: Clone + Default,
    M: protocol::Mutation<P>,
{
    fn owns_terminal(&self, owner: &store::TransientStore<P, M>) -> bool {
        owner.generation_now() == self.terminal_generation && self.terminal_root.as_ref().is_some_and(|root| owner.current_matches(root))
    }
}

impl<P, M> ArtifactOwnedDisposer<store::TransientStore<P, M>> for TransientStoreDisposer<P, M>
where
    P: Clone + Default + Send + Sync + 'static,
    M: protocol::Mutation<P> + Send + 'static,
{
    /// 🧹️ The retained transient's own retirement gets the caller's WHOLE grant: the close ladder
    /// prices a step per axis, and clamping it to one item here is what made a mesh-scale transient
    /// answer an empty receipt — eight of those in a row and the structural accountant kills the
    /// close with `plugin.internal.zero-progress` (ticket 26/09/09).
    fn close_step(&mut self, owner: &mut store::TransientStore<P, M>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        let demand = self.retirement_demands(owner, grant.maximum_copy_bytes).map_err(|error| Fault::from(error.into_message()))?;
        if self.retirement.is_none() && self.terminal_root.is_some() {
            return self.owns_terminal(owner).then(|| PluginLifecycleStep::Complete(Default::default())).ok_or_else(|| Fault::from("transient terminal owner or generation changed"));
        }
        if grant.maximum_depth < demand.depth {
            return Err(Fault::from("transient disposal exceeds admitted depth"));
        }
        if yields(grant, demand) {
            return Ok(PluginLifecycleStep::Progress(Default::default()));
        }
        let Some(retirement) = self.retirement.as_mut() else {
            self.retirement = Some(owner.begin_retirement(P::default(), self.factory.clone()));
            self.terminal_root = Some(owner.current_weak());
            self.terminal_generation = owner.generation_now();
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        };
        if retirement.terminal_is_empty() {
            self.retirement = None;
            let progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
            return Ok(if self.owns_terminal(owner) { PluginLifecycleStep::Complete(progress) } else { PluginLifecycleStep::Progress(progress) });
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        let step = retirement.close_step(child).map_err(|error| Fault::from(error.into_message()))?;
        let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, retirement.terminal_is_empty(), "transient store retirement").map_err(|error| Fault::from(error.into_message()))?;
        if retirement.terminal_is_empty() {
            self.retirement = None;
        }
        Ok(PluginLifecycleStep::retained(step, self.retirement.is_none() && self.owns_terminal(owner)))
    }

    fn retirement_demands(&self, _owner: &store::TransientStore<P, M>, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(retirement) = self.retirement.as_ref() {
            if retirement.terminal_is_empty() {
                return Ok(RetirementDemand { depth: 1, ..Default::default() });
            }
            let mut demand = retirement.retirement_demands(body)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "transient disposal depth overflow"))?;
            return Ok(demand);
        }
        Ok(if self.terminal_root.is_none() { RetirementDemand { copy_bytes: std::mem::size_of::<store::TransientStoreRetirement<P>>(), depth: 1, ..Default::default() } } else { Default::default() })
    }

    fn terminal_is_empty(&self, owner: &store::TransientStore<P, M>) -> bool {
        self.retirement.is_none() && self.owns_terminal(owner)
    }
}

pub fn transient_store_disposer<P, M>(factory: Arc<dyn store::ArtifactOwnedValueRetirementFactory<P>>) -> Box<dyn ArtifactOwnedDisposer<store::TransientStore<P, M>>>
where
    P: Clone + Default + Send + Sync + 'static,
    M: protocol::Mutation<P> + Send + 'static,
{
    Box::new(TransientStoreDisposer::<P, M>::new(factory))
}
