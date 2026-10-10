//! 👥️ Domain-authorized presence store closure preserves exact local and peer owner retirement.

use crate::store::{Mutation, PresenceStore, PresenceStoreRetirement};
use crate::app::PluginLifecycleStep;
use crate::{ArtifactOwnedDisposer, Fault};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::sync::{Arc, Weak};

fn yields(grant: RetainedCloneGrant, demand: RetirementDemand) -> bool {
    grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes
}

fn depth_refusal(grant: RetainedCloneGrant, demand: RetirementDemand, scope: &'static str) -> Result<(), ValueError> {
    if grant.maximum_depth < demand.depth { Err(ValueError::literal(ValueRefusalKind::DepthLimit, scope)) } else { Ok(()) }
}

const _: () = assert!(size_of::<crate::NoPresence>() == 0 && !std::mem::needs_drop::<crate::NoPresence>());

/// 🫧️ Explicit ownership for the framework's zero-payload presence type only.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct NoPresenceRetirementFactory;

fn no_presence_is_empty(_: &crate::NoPresence) -> bool {
    true
}

pub fn no_presence_store_disposer() -> Box<dyn ArtifactOwnedDisposer<PresenceStore<crate::NoPresence, crate::NoPresenceMutation>>> {
    Box::new(PresenceStoreOwnedDisposer::new(Arc::new(crate::NoPresence::default()), no_presence_is_empty).expect("NoPresence is statically empty"))
}

/// 🧹️ Bounded close for a presence store of ANY payload type — the presence twin of
/// `bounded_transient_store_disposer`, and the generic form of [`no_presence_store_disposer`],
/// which is this function at `P = NoPresence`.
///
/// 🛂️ It exists so the framework can OWN the presence close lane by default. `VcsArtifactApp`'s
/// close ladder drives lane 3 through `drive_artifact_owned_disposer("presence-store", …)`, whose
/// missing-disposer branch is `interactive-job.close-owned-disposer-missing` — a fail-closed
/// refusal, so an app that declared no presence disposer could never be CLOSED at all, and every
/// caller that constructs an app and closes it (the `codec` resolver builds and closes every app of
/// a bundle) faulted on it. The terminal root a presence close installs is the payload's own
/// `Default`, and the emptiness predicate is equality with it: a closing app holds no live presence
/// by definition. Declaring a disposer explicitly still wins — apps override, they do not opt in.
///
/// 🛂️ The refusal branch is unreachable by construction: the terminal IS `P::default()` and the
/// emptiness predicate is equality with it. It is spelled as a `match` rather than `Result::expect`
/// because `PresenceStoreOwnedDisposer::new` hands the rejected root BACK (`Err(Arc<P>)`), so
/// `expect` would demand `P: Debug` of every presence payload in the repository — which is the whole
/// bound this generic default exists to avoid.
pub fn bounded_presence_store_disposer<P, M>() -> Box<dyn ArtifactOwnedDisposer<PresenceStore<P, M>>>
where
    P: Clone + Default + PartialEq + Send + Sync + 'static,
    M: Mutation<P> + 'static,
{
    match PresenceStoreOwnedDisposer::new(Arc::new(P::default()), presence_root_is_its_own_default::<P>) {
        Ok(disposer) => Box::new(disposer),
        Err(_) => unreachable!("a default presence root is its own empty terminal"),
    }
}

fn presence_root_is_its_own_default<P: Default + PartialEq>(root: &P) -> bool {
    *root == P::default()
}

pub fn no_presence_local_root_retirement_factory() -> Arc<dyn store::SnapshotRetirementFactory<crate::NoPresence>> {
    Arc::new(NoPresenceRetirementFactory)
}

pub fn no_presence_peer_retirement_factory() -> Arc<dyn store::SnapshotRetirementFactory<crate::NoPresence>> {
    Arc::new(NoPresenceRetirementFactory)
}

/// 🧹️ Bounded retirement for a displaced presence root of ANY payload type — the presence twin of
/// `bounded_transient_root_retirement_factory`, whose lane has the identical shape (one displaced
/// local root, retired in grant-sized steps, priced by the payload's own DSL print).
///
/// It exists so the framework can OWN the presence lane by default. `PresenceStore::local_read`
/// fails closed without a live local retirement owner (`🏪️store/🦀️.rs:4716`) and the `ArtifactApp`
/// default returned `None`, so every app that declares a presence type and no owner refuses the
/// first command whose ephemeral leg reads local presence — 95 editors, of which four were found one
/// at a time by four different slices in two days (🏠️home, 📖️playbook, ⚙️playbook-module-procedural,
/// 🪐️space-index; ticket 26/09/18 S10 §2.9a, S11 §3.2/§3.3b). The viewer wrapper already defaulted
/// into the framework's own owners (`🔌️plugin/🦀️.rs:8003`); the editor wrapper could not, because
/// its `Self::Presence` is `V::Presence` and the only owner on offer was `NoPresence`-typed.
/// Declaring an owner explicitly still wins — this is `or_else`, not a replacement.
pub fn bounded_presence_root_retirement_factory<P>() -> Arc<dyn store::SnapshotRetirementFactory<P>>
where
    P: store::ArtifactDsl + Send + Sync + 'static,
{
    Arc::new(BoundedPresenceRootRetirementFactory::<P>(std::marker::PhantomData))
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct BoundedPresenceRootRetirementFactory<P>(std::marker::PhantomData<fn() -> P>);

impl<P> store::SnapshotRetirementFactory<P> for BoundedPresenceRootRetirementFactory<P>
where
    P: store::ArtifactDsl + Send + Sync + 'static,
{
    fn retirement_birth_bytes(&self, _snapshot: &Arc<P>) -> usize { std::mem::size_of::<BoundedPresenceRootRetirement<P>>() }

    fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Arc<P>)> {
        let progress = match (RetainedCloneBirthDemand { capacity_bytes: std::mem::size_of::<BoundedPresenceRootRetirement<P>>(), depth: 1 }).admit(grant) {
            Ok(progress) => progress,
            Err(error) => return Err((error, snapshot)),
        };
        let retained_bytes = store::ArtifactDsl::print_dsl(snapshot.as_ref()).len();
        Ok((Box::new(BoundedPresenceRootRetirement { root: Some(snapshot), retained_bytes }), progress))
    }
}

struct BoundedPresenceRootRetirement<P> {
    root: Option<Arc<P>>,
    retained_bytes: usize,
}

impl<P> BoundedPresenceRootRetirement<P> {
    fn demand(&self) -> RetirementDemand {
        RetirementDemand { copy_bytes: usize::from(self.retained_bytes != 0), depth: usize::from(!self.terminal_is_empty()), ..Default::default() }
    }

    fn terminal_is_empty(&self) -> bool {
        self.root.is_none() && self.retained_bytes == 0
    }
}

impl<P: Send + Sync + 'static> store::ErasedSnapshotRetirement for BoundedPresenceRootRetirement<P> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        let demand = self.demand();
        depth_refusal(grant, demand, "presence root retirement exceeds admitted depth")?;
        if yields(grant, demand) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let copied_bytes = self.retained_bytes.min(grant.maximum_copy_bytes);
        if copied_bytes != 0 {
            self.retained_bytes -= copied_bytes;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes, ..Default::default() }));
        }
        drop(self.root.take());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    fn terminal_is_empty(&self) -> bool {
        BoundedPresenceRootRetirement::terminal_is_empty(self)
    }

    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demand().copy_bytes) }

    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, ValueError> { Ok(0) }

    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }

    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demand().depth) }
}

impl store::SnapshotRetirementFactory<crate::NoPresence> for NoPresenceRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &Arc<crate::NoPresence>) -> usize { std::mem::size_of::<NoPresenceRetirement>() }

    fn retire(&self, root: Arc<crate::NoPresence>, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Arc<crate::NoPresence>)> {
        match (RetainedCloneBirthDemand { capacity_bytes: std::mem::size_of::<NoPresenceRetirement>(), depth: 1 }).admit(grant) {
            Ok(progress) => Ok((Box::new(NoPresenceRetirement(std::mem::ManuallyDrop::new(Some(root)))), progress)),
            Err(error) => Err((error, root)),
        }
    }
}

struct NoPresenceRetirement(std::mem::ManuallyDrop<Option<Arc<crate::NoPresence>>>);

impl store::ErasedSnapshotRetirement for NoPresenceRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.0.is_none() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        if grant.maximum_depth < 1 {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "empty presence retirement exceeds admitted depth"));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        drop(self.0.take());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }

    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }

    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, ValueError> { Ok(0) }

    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }

    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(usize::from(self.0.is_some())) }
}

impl Drop for NoPresenceRetirement {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.0.is_none(), "empty presence must return its exact root before drop");
        }
    }
}

/// 🛂️ A domain supplies its exact empty terminal root and predicate; the store supplies its installed factories.
pub struct PresenceStoreOwnedDisposer<P> {
    terminal: Option<Arc<P>>,
    terminal_root: Weak<P>,
    terminal_generation: Option<u64>,
    terminal_is_empty: fn(&P) -> bool,
    retirement: Option<PresenceStoreRetirement<P>>,
}

impl<P> PresenceStoreOwnedDisposer<P> {
    pub fn new(terminal: Arc<P>, terminal_is_empty: fn(&P) -> bool) -> Result<Self, Arc<P>> {
        if !terminal_is_empty(&terminal) {
            return Err(terminal);
        }
        Ok(Self { terminal_root: Arc::downgrade(&terminal), terminal: Some(terminal), terminal_generation: None, terminal_is_empty, retirement: None })
    }
}

impl<P: Clone + Send + Sync + 'static> PresenceStoreOwnedDisposer<P> {
    fn owns_terminal<M: Mutation<P>>(&self, owner: &PresenceStore<P, M>) -> bool {
        self.terminal_generation == Some(owner.generation_now())
            && owner.retirement_started()
            && self.terminal_root.upgrade().is_some_and(|terminal| std::ptr::eq(terminal.as_ref(), owner.local()))
            && (self.terminal_is_empty)(owner.local())
            && owner.peers_root().is_empty()
    }
}

impl<P: Clone + Send + Sync + 'static, M: Mutation<P>> ArtifactOwnedDisposer<PresenceStore<P, M>> for PresenceStoreOwnedDisposer<P> {
    fn close_step(&mut self, owner: &mut PresenceStore<P, M>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        if self.retirement.is_some() && !self.owns_terminal(owner) {
            return Err(Fault::from("presence close terminal root or generation changed"));
        }
        if self.retirement.as_ref().is_some_and(PresenceStoreRetirement::terminal_is_empty) {
            return Ok(PluginLifecycleStep::Complete(Default::default()));
        }
        let demand = self.retirement_demands(owner, grant.maximum_copy_bytes).map_err(|error| Fault::from(error.into_message()))?;
        depth_refusal(grant, demand, "presence close exceeds admitted depth").map_err(|error| Fault::from(error.into_message()))?;
        if yields(grant, demand) {
            return Ok(PluginLifecycleStep::Progress(Default::default()));
        }
        if let Some(retirement) = self.retirement.as_mut() {
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = retirement.close_step(child).map_err(|error| Fault::from(error.into_message()))?;
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, retirement.terminal_is_empty(), "presence store retirement").map_err(|error| Fault::from(error.into_message()))?;
            return Ok(PluginLifecycleStep::retained(step, retirement.terminal_is_empty()));
        }
        let terminal = self.terminal.take().ok_or_else(|| Fault::from("presence close lost its exact terminal root"))?;
        match owner.begin_retirement(terminal, self.terminal_is_empty) {
            Ok(retirement) => {
                self.terminal_generation = Some(owner.generation_now());
                self.retirement = Some(retirement);
                Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
            }
            Err((reason, terminal)) => {
                self.terminal = Some(terminal);
                Err(Fault::from(reason))
            }
        }
    }

    fn retirement_demands(&self, _owner: &PresenceStore<P, M>, body: usize) -> Result<RetirementDemand, ValueError> {
        match self.retirement.as_ref() {
            Some(retirement) if retirement.terminal_is_empty() => Ok(Default::default()),
            Some(retirement) => {
                let mut demand = retirement.retirement_demands(body)?;
                demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "presence disposal depth overflow"))?;
                Ok(demand)
            }
            None => Ok(RetirementDemand { copy_bytes: 0, depth: 1, ..Default::default() }),
        }
    }

    fn terminal_is_empty(&self, owner: &PresenceStore<P, M>) -> bool {
        self.terminal.is_none() && self.owns_terminal(owner) && self.retirement.as_ref().is_some_and(PresenceStoreRetirement::terminal_is_empty)
    }
}
