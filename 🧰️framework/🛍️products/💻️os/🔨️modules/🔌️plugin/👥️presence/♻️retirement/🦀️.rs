//! 👥️ Domain-authorized presence store closure preserves exact local and peer owner retirement.

use crate::store::{Mutation, PresenceStore, PresenceStoreRetirement, SnapshotRetirementStep};
use crate::{ArtifactOwnedDisposer, Fault, PluginCloseStep};
use std::sync::{Arc, Weak};

const _: () = assert!(size_of::<crate::NoPresence>() == 0 && !std::mem::needs_drop::<crate::NoPresence>());

/// 🫧️ Explicit ownership for the framework's zero-payload presence type only.
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

struct BoundedPresenceRootRetirementFactory<P>(std::marker::PhantomData<fn() -> P>);

impl<P> store::SnapshotRetirementFactory<P> for BoundedPresenceRootRetirementFactory<P>
where
    P: store::ArtifactDsl + Send + Sync + 'static,
{
    fn retire(&self, snapshot: Arc<P>) -> Box<dyn store::ErasedSnapshotRetirement> {
        let retained_bytes = store::ArtifactDsl::print_dsl(snapshot.as_ref()).len();
        Box::new(BoundedPresenceRootRetirement { root: Some(snapshot), retained_bytes })
    }
}

struct BoundedPresenceRootRetirement<P> {
    root: Option<Arc<P>>,
    retained_bytes: usize,
}

impl<P: Send + Sync + 'static> store::ErasedSnapshotRetirement for BoundedPresenceRootRetirement<P> {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.retained_bytes > maximum_bytes {
            self.retained_bytes -= maximum_bytes;
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: maximum_bytes });
        }
        if self.root.take().is_some() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.root.is_none()
    }
}

impl store::SnapshotRetirementFactory<crate::NoPresence> for NoPresenceRetirementFactory {
    fn retire(&self, root: Arc<crate::NoPresence>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(NoPresenceRetirement(std::mem::ManuallyDrop::new(Some(root))))
    }
}

struct NoPresenceRetirement(std::mem::ManuallyDrop<Option<Arc<crate::NoPresence>>>);

impl store::ErasedSnapshotRetirement for NoPresenceRetirement {
    fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if self.0.is_none() {
            return Ok(SnapshotRetirementStep::Complete);
        }
        if items == 0 || bytes == 0 {
            return Ok(SnapshotRetirementStep::Blocked);
        }
        drop(self.0.take());
        Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
    }

    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }
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
    fn close_step(&mut self, owner: &mut PresenceStore<P, M>, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if self.retirement.is_some() && !self.owns_terminal(owner) {
            return Err(Fault::from("presence close terminal root or generation changed"));
        }
        if self.retirement.as_ref().is_some_and(PresenceStoreRetirement::terminal_is_empty) {
            return Ok(PluginCloseStep::Complete);
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(1, maximum_bytes).map_err(Fault::from)? {
                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= maximum_bytes => Ok(PluginCloseStep::Pending { released_items, released_bytes }),
                SnapshotRetirementStep::Pending { .. } => Err(Fault::from("presence retirement exceeded its exact grant")),
                SnapshotRetirementStep::Blocked => Ok(PluginCloseStep::Blocked { reason: "presence retains a captured local or peer reader" }),
                SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => Ok(PluginCloseStep::Complete),
                SnapshotRetirementStep::Complete => Err(Fault::from("presence retirement completed with retained owners")),
            };
        }
        let terminal = self.terminal.take().ok_or_else(|| Fault::from("presence close lost its exact terminal root"))?;
        match owner.begin_retirement(terminal, self.terminal_is_empty) {
            Ok(retirement) => {
                self.terminal_generation = Some(owner.generation_now());
                self.retirement = Some(retirement);
                Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
            }
            Err((reason, terminal)) => {
                self.terminal = Some(terminal);
                Err(Fault::from(reason))
            }
        }
    }

    fn terminal_is_empty(&self, owner: &PresenceStore<P, M>) -> bool {
        self.terminal.is_none() && self.owns_terminal(owner) && self.retirement.as_ref().is_some_and(PresenceStoreRetirement::terminal_is_empty)
    }
}
