//! 🧵️ Bounded transient publication plus lease-aware exact-store disposal.

use crate::app::{ArtifactOwnedDisposer, PluginCloseStep};
use crate::{protocol, store};
use semio_framework::{Fault, FaultCode, FaultOrigin};
use std::sync::{Arc, Weak};

struct BoundedTransientPreparation<P, M> {
    request: Option<store::ArtifactEphemeralOneItemPreparationRequest<P, M>>,
    prepared: Option<store::ArtifactEphemeralOneItemPrepared<P>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
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
        let footprint = store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes };
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
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if self.cancelled || !grant.permits_one() || grant.maximum_bytes < self.retained_bytes {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_none() {
            let request = self.request.take().ok_or_else(|| "transient preparation lost its request".to_string())?;
            let outcome = protocol::Mutation::diff(&request.mutation, request.base.as_ref());
            if outcome.worst_level().is_some_and(|level| level >= protocol::Severity::Error) {
                return Err("transient mutation was rejected against its captured base".into());
            }
            let next_root = protocol::MutationDiff::apply(outcome.diff(), request.base.as_ref()).map_err(|error| error.to_string())?;
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

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 || grant.maximum_bytes < self.retained_bytes {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.request.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.request.is_none() && self.prepared.is_none()
    }
}

struct BoundedTransientRootRetirement<P> {
    root: Option<Arc<P>>,
    retained_bytes: usize,
}

impl<P: Send + Sync + 'static> store::ErasedSnapshotRetirement for BoundedTransientRootRetirement<P> {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 || maximum_bytes < self.retained_bytes {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.root.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.root.is_none()
    }
}

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
    fn retire(&self, snapshot: Arc<P>) -> Box<dyn store::ErasedSnapshotRetirement> {
        let retained_bytes = store::ArtifactDsl::print_dsl(snapshot.as_ref()).len();
        Box::new(BoundedTransientRootRetirement { root: Some(snapshot), retained_bytes })
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
    fn close_step(&mut self, owner: &mut store::TransientStore<P, M>, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if self.terminal_root.is_none() {
            if maximum_items == 0 {
                return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.retained_bytes = store::ArtifactDsl::print_dsl(owner.current_root().as_ref()).len();
            self.retired = Some(std::mem::replace(owner, store::TransientStore::new(P::default())));
            self.terminal_root = Some(Arc::downgrade(&owner.current_root()));
            self.terminal_generation = owner.generation_now();
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.retired.is_some() {
            if maximum_items == 0 || maximum_bytes < self.retained_bytes {
                return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.retired = None;
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        self.owns_terminal(owner).then_some(PluginCloseStep::Complete).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("transient.disposer-authority"), "transient terminal owner changed during disposal"))
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
    fn close_step(&mut self, owner: &mut store::TransientStore<P, M>, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if self.retirement.is_none() {
            if self.terminal_root.is_some() {
                return self.owns_terminal(owner).then_some(PluginCloseStep::Complete).ok_or_else(|| Fault::from("transient terminal owner or generation changed"));
            }
            if maximum_items == 0 {
                return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.retirement = Some(owner.begin_retirement(P::default(), self.factory.clone()));
            self.terminal_root = Some(owner.current_weak());
            self.terminal_generation = owner.generation_now();
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        let retirement = self.retirement.as_mut().expect("checked transient retirement remains present");
        // 🧹️ The retained transient's own retirement gets the caller's WHOLE grant: the close ladder
        // prices a step in PAGES, and clamping it to one item here is what made a mesh-scale transient
        // answer `Pending { 0, 0 }` — eight of those in a row and the structural accountant kills the
        // close with `plugin.internal.zero-progress` (ticket 26/09/09).
        match retirement.close_step(maximum_items, maximum_bytes).map_err(Fault::from)? {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(PluginCloseStep::Pending { released_items, released_bytes }),
            store::SnapshotRetirementStep::Blocked => Ok(PluginCloseStep::Blocked { reason: "transient read remains live" }),
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                self.retirement = None;
                Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
            }
            store::SnapshotRetirementStep::Complete => Err(Fault::from("transient retirement completed without terminal-empty ownership")),
        }
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
