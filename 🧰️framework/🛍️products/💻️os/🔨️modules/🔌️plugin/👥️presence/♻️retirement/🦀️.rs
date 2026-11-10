//! 👥️ Domain-authorized presence store closure preserves exact local and peer owner retirement.

use crate::store::{Mutation, PresenceStore, PresenceStoreRetirement};
use semio_framework_value::{RetirementPollExit, SharedValueRetirementFactory};
use semio_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework::Fault;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::sync::Arc;

fn yields(grant: RetainedCloneGrant, demand: RetirementDemand) -> bool {
    grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes
}

fn depth_refusal(grant: RetainedCloneGrant, demand: RetirementDemand, scope: &'static str) -> Result<(), ValueError> {
    if grant.maximum_depth < demand.depth { Err(ValueError::literal(ValueRefusalKind::DepthLimit, scope)) } else { Ok(()) }
}

const _: () = assert!(size_of::<crate::NoPresence>() == 0 && !std::mem::needs_drop::<crate::NoPresence>());

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
    Arc::new(SharedValueRetirementFactory::<crate::NoPresence>::default())
}

pub fn no_presence_peer_retirement_factory() -> Arc<dyn store::SnapshotRetirementFactory<crate::NoPresence>> {
    Arc::new(SharedValueRetirementFactory::<crate::NoPresence>::default())
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
    P: semio_framework_value::retirement::RetireOwned + Sync,
{
    Arc::new(SharedValueRetirementFactory::<P>::default())
}

/// 🛂️ A domain supplies its exact empty terminal root and predicate; the store supplies its installed factories.
pub struct PresenceStoreOwnedDisposer<P> {
    terminal: Option<Arc<P>>,
    terminal_generation: Option<u64>,
    terminal_is_empty: fn(&P) -> bool,
    retirement: Option<Box<PresenceStoreRetirement<P>>>,
}

impl<P> PresenceStoreOwnedDisposer<P> {
    pub fn new(terminal: Arc<P>, terminal_is_empty: fn(&P) -> bool) -> Result<Self, Arc<P>> {
        if !terminal_is_empty(&terminal) {
            return Err(terminal);
        }
        Ok(Self { terminal: Some(terminal), terminal_generation: None, terminal_is_empty, retirement: None })
    }
}

impl<P: Clone + Send + Sync + 'static> PresenceStoreOwnedDisposer<P> {
    fn owns_terminal<M: Mutation<P> + 'static>(&self, owner: &PresenceStore<P, M>) -> bool {
        self.terminal_generation == Some(owner.generation_now()) && owner.detached_terminal_is_empty()
    }
}

impl<P: Clone + Send + Sync + 'static, M: Mutation<P> + 'static> ArtifactOwnedDisposer<PresenceStore<P, M>> for PresenceStoreOwnedDisposer<P> {
    fn close_step(&mut self, owner: &mut PresenceStore<P, M>, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> Result<PluginLifecycleStep, Fault> {
        let grant = effect.grant();
        if !effect.performed(1).map_err(|error| Fault::from(error.into_message()))? { return Ok(PluginLifecycleStep::Progress(Default::default())); }
        if self.retirement.is_some() && !self.owns_terminal(owner) {
            return Err(Fault::from("presence close terminal root or generation changed"));
        }
        if self.retirement.is_none() && self.terminal.is_none() && self.owns_terminal(owner) {
            return Ok(PluginLifecycleStep::Complete(Default::default()));
        }
        let demand = self.retirement_demands(owner, grant.maximum_copy_bytes).map_err(|error| Fault::from(error.into_message()))?;
        depth_refusal(grant, demand, "presence close exceeds admitted depth").map_err(|error| Fault::from(error.into_message()))?;
        if yields(grant, demand) {
            return Ok(PluginLifecycleStep::Progress(Default::default()));
        }
        if self.retirement.as_ref().is_some_and(|retirement| retirement.terminal_is_empty()) {
            self.retirement = None;
            return Ok(PluginLifecycleStep::retained(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: size_of::<PresenceStoreRetirement<P>>(), ..Default::default() }), self.owns_terminal(owner)));
        }
        if let Some(retirement) = self.retirement.as_mut() {
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = match retirement.close_step(&mut effect.nested(child)) {
                Ok(step) => step,
                Err(RetirementPollExit::Pending(_)) => RetainedCloneStep::Progress(Default::default()),
                Err(RetirementPollExit::Failed(failure)) => return Err(Fault::from(semio_plugin_ownership::bounded_ownership::plugin_failure_error(failure).into_message())),
            };
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, retirement.terminal_is_empty(), "presence store retirement").map_err(|error| Fault::from(error.into_message()))?;
            return Ok(PluginLifecycleStep::retained(step, false));
        }
        let terminal = self.terminal.take().ok_or_else(|| Fault::from("presence close lost its exact terminal root"))?;
        match owner.begin_retirement(terminal, self.terminal_is_empty) {
            Ok(retirement) => {
                self.terminal_generation = Some(owner.generation_now());
                self.retirement = Some(Box::new(retirement));
                Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() }))
            }
            Err((reason, terminal)) => {
                self.terminal = Some(terminal);
                Err(Fault::from(reason))
            }
        }
    }

    fn retirement_demands(&self, _owner: &PresenceStore<P, M>, body: usize) -> Result<RetirementDemand, ValueError> {
        match self.retirement.as_ref() {
            Some(retirement) if retirement.terminal_is_empty() => Ok(RetirementDemand { release_bytes: size_of::<PresenceStoreRetirement<P>>(), depth: 1, ..Default::default() }),
            Some(retirement) => {
                let mut demand = retirement.retirement_demands(body)?;
                demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "presence disposal depth overflow"))?;
                Ok(demand)
            }
            None if self.terminal.is_some() => Ok(RetirementDemand { capacity_bytes: size_of::<PresenceStoreRetirement<P>>(), depth: 1, ..Default::default() }),
            None => Ok(Default::default()),
        }
    }

    fn terminal_is_empty(&self, owner: &PresenceStore<P, M>) -> bool {
        self.terminal.is_none() && self.owns_terminal(owner) && self.retirement.is_none()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        (self.terminal.is_none() && self.retirement.is_none()).then_some(size_of::<Self>())
    }
}
