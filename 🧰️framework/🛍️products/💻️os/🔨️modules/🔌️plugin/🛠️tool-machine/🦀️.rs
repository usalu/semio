//! 🛠️ OS runtime of the framework's continuous tools (design §13 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING), window
//! transient, ephemeral, local-only, never history:
//! - 🎚️ continuous controls (§13.1): every dispatch whose args carry a `gesture` rides the dispatching window's press in the
//!   instance's [`ScrubLedger`]; a tick publishes nothing, the release publishes ONE edit, every host abort leaves zero trace.
//!   Apps supply only the ABSOLUTE leaves of a value: their verb's `Emit::mutations`, and the owned children's share
//!   (`Emit::child_emits`, design §12) — ONE press commits ONE transaction stamped on every member it touched.
//! - ⌨️ typing (§13.2): every live typing delivery (args carry a `typing` buffer) folds its leaves into the window's run in
//!   the [`TypingLedger`] through the app's typing algebra (`ArtifactApp::typing_fold`); the run commits as ONE edit on idle,
//!   a caret jump, blur, page hide, Enter, an explicit apply or any other verb, and aborts only on a conflicting base or a
//!   frozen document.
//!
//! - 🎛️ the config lanes of a press (§20.1): a tick's app-config and window-config mutations ride the same press as its
//!   document leaves ([`PressLeaf`]), rendered over the committed config and never published; the release publishes them
//!   as ONE config edit in the very ledger step that commits the document leaves, a host abort drops them with zero
//!   trace; config edits are never history rows.
//!
//! - 🖐️ gestures (§5, §22.10): every window owns ONE gesture slot in the instance's [`GestureLedger`]. A dispatch drives its
//!   window's streamed tool against a copy of the slot ([`GestureSlot::drive`]); the slot follows only when the dispatch
//!   publishes. Host facts end a window's gesture here, never in an editor: a blur `blur`, a lost pointer capture
//!   `captureLost`, a utility switch or a closing window `retired`, a moved base `baseMoved` (a gesture pinned to a revision),
//!   and an opened history edit `frozen` for every window — zero trace each time.
//!
//! Every render seam reads the committed document (and config) overlaid with every open press's, gesture's and run's
//! provisional leaves; every committed transaction is published stamped with its `TransactionRef`. Domain-neutral machines:
//! `semio_framework_tool_machine` (`ScrubMachine`, `TypingMachine`, `ChartGesture`).

use semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use semio_framework_plugin_decl_app::ArtifactApp;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
#[cfg(test)]
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use std::collections::BTreeMap;
use semio_framework_plugin_window_config::WindowConfigSnapshot;
use semio_framework_plugin_decl_emission::{ChildEmit, Emit, HostEvent};
use semio_framework_value::{DslValue, ValueError};
use semio_framework::{Fault, FaultFrom};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework::kernel::{HybridLogicalTimestamp, InvocationResult};
use store::{ColdMutation, Mutation, MutationDiff};
use semio_framework_plugin_decl_views::{NoConfig, NoConfigMutation};
use protocol::{OpBinary, OpText};
#[cfg(test)]
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::RetainedCloneGrant;
#[cfg(test)]
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
#[cfg(any(test, feature = "artifact-app-testing"))]
use serde_json::Value;
use semio_framework_plugin_window_config::WindowConfigMutation;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_ui_contract::*;
use semio_framework_value_original_aliases::OriginalAliasBatch as ToolOriginalAliasBatch;
use semio_framework_tool_machine::{GestureHostEvent, GestureLedger, ScrubLedger, ScrubPhase, ToolAbortReason, ToolStep, TypingLedger};
use std::sync::Arc;
#[doc(hidden)]
#[path="🖐️gesture/🎟️issuer/🦀️.rs"]
pub mod gesture_issuer;
use crate::tool_jobs_tool_machine::gesture_issuer::GestureOperation;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::retirement::controlled::ControlledRetirement;

/// 🧮️ Folds the provisional `leaf` onto `running` (else `committed`) and answers whether it applied (a leaf its base refuses
/// is skipped); the displaced intermediate joins `displaced`, which the caller retires through its store, never plainly.
fn fold_leaf<P: Clone, M: Mutation<P>+protocol::ColdMutation<P>>(committed: &Arc<P>, running: &mut Option<Arc<P>>, displaced: &mut Vec<Arc<P>>, leaf: &M) -> bool {
    let base: &P = running.as_deref().unwrap_or(committed.as_ref());
    let outcome = leaf.diff(base);
    let applicable = outcome.is_applicable(protocol::MergePolicy::default());
    let (diff, _) = outcome.into_parts();
    let applied = applicable.then(|| protocol::apply_diff_cold(&diff, base));
    MutationDiff::retire_cold(diff);
    match applied {
        Some(Ok(next)) => {
            displaced.extend(running.replace(Arc::new(next)));
            true
        }
        _ => false,
    }
}

//#region 🔖️Tag
/// 🏷️ One continuous-control dispatch as its operation carries it to publication: the dispatching window, the tool
/// `<appId>#<verb>` and where the dispatch sits in its press.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub struct ScrubTag {
    pub window: String,
    pub tool: String,
    pub phase: ScrubPhase,
}

/// ⌨️ One live typing delivery as its operation carries it to publication: the typing window, the tool `<appId>#<verb>` and
/// the buffer typed into.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub struct TypingTag {
    pub window: String,
    pub tool: String,
    pub buffer: String,
}

/// 🏷️ What an admitted operation carries to its publication: a press of a continuous control or a typed edit.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub enum ToolTag {
    Scrub(ScrubTag),
    Typing(TypingTag),
}

/// 🚦️ How `dispatch_action` proceeds once it read the tool arguments: a plain dispatch, a press or typed edit whose operation
/// carries the tag, or a dispatch the tool runtime settled itself (a host abort, a commit signal, a frozen edit).
#[doc(hidden)]
pub enum ToolDispatch {
    Plain,
    Tagged(ToolTag),
    Settled(Result<InvocationResult, Fault>),
}

/// 📤️ What one gesture dispatch publishes: its committed transaction as ONE edit stamped with the ref — plainly when the
/// dispatch carries no admission (`authoring_seed` empty: a render or test view without command authority) — or nothing.
pub fn gesture_emit<Mutation, ConfigMutation, DraftMutation>(committed: Option<(protocol::TransactionRef, Vec<Mutation>)>, authoring_seed: &str) -> Emit<Mutation, ConfigMutation, DraftMutation> {
    match committed {
        Some((transaction, mutations)) if !authoring_seed.is_empty() => Emit::commit_transaction(transaction, mutations),
        Some((_, mutations)) => Emit::mutations(mutations),
        None => Emit::default(),
    }
}

/// 📡️ The gesture fact a host event is (law table `hostEvents` of the tool-machine gesture-drive corpus).
#[doc(hidden)]
pub fn gesture_host_event(event: &HostEvent) -> GestureHostEvent {
    match event {
        HostEvent::WindowBlurred { .. } => GestureHostEvent::Blur,
        HostEvent::PointerCaptureLost { .. } => GestureHostEvent::CaptureLost,
        HostEvent::UtilityChanged { .. } => GestureHostEvent::UtilityChanged,
        HostEvent::Retiring { .. } => GestureHostEvent::Retiring,
        HostEvent::TimeTravelFrozen { .. } => GestureHostEvent::TimeTravelFrozen,
        HostEvent::BaseMoved { .. } => GestureHostEvent::BaseMoved,
    }
}
//#endregion 🔖️Tag

//#region 🔖️Runtime
/// 🎚️ One leaf of a press — every lane of its emit (design §13.1, §12, §20.1): an absolute document leaf, an owned child's
/// share, an app-config or a window-config mutation. Every lane rides the window's ONE scrub, so ONE ledger step decides
/// them all: a tick holds them as the press's provisional overlay, the release commits them as ONE transaction, a late input
/// of a closed press and a refused input leave every lane as it was, a host abort drops them all with zero trace.
#[doc(hidden)]
#[derive(Clone, semio_framework_value::RetireOwned)]
pub enum PressLeaf<M, CM> {
    Member(M),
    Child(ChildEmit),
    Config(CM),
    WindowConfig(WindowConfigMutation),
}

impl<M, CM> PressLeaf<M, CM> {
    pub fn member(&self) -> Option<&M> {
        match self {
            Self::Member(leaf) => Some(leaf),
            _ => None,
        }
    }

    pub fn config(&self) -> Option<&CM> {
        match self {
            Self::Config(leaf) => Some(leaf),
            _ => None,
        }
    }

    pub fn window_config(&self) -> Option<&WindowConfigMutation> {
        match self {
            Self::WindowConfig(leaf) => Some(leaf),
            _ => None,
        }
    }
}

/// 🗂️ The instance's continuous-tool runtime: the per-window press ledger (every lane of a press as [`PressLeaf`]s),
/// [`GestureLedger`] (each window's ONE gesture slot, the slots admitted dispatches hold until they publish, and whether a
/// host fact ended a gesture since the last repaint) and [`TypingLedger`], the committed ⊕ provisional overlays every render seam reads while a press or run is open — the
/// document, the app config and each window's config —, the tag of the dispatch being admitted, the tags of admitted
/// operations until their completion publishes, and the logical tick that makes every typing clock unique.
pub struct ToolMachineRuntime<P, M:RetireOwned+Send+'static, C = NoConfig, CM = NoConfigMutation> {
    pub presses: ScrubLedger<PressLeaf<M, CM>>,
    pub gestures: GestureLedger<M>,
    pub gesture_operations: [Option<GestureOperation<M>>;ARTIFACT_LIVE_OUTPUT_SLOTS],
    pub gesture_retirement:Option<ControlledRetirement<GestureOperation<M>>>,
    pub gesture_ended: bool,
    pub typing: TypingLedger<M>,
    pub overlay: Option<Arc<P>>,
    overlay_generation: u64,
    pub provisional_generation: u64,
    pub config_overlay: Option<Arc<C>>,
    config_overlay_generation: u64,
    pub pending_overlay_aliases: Option<Vec<Arc<P>>>,
    pub overlay_alias_retirement: Option<ToolOriginalAliasBatch<P>>,
    pub pending_config_aliases: Option<Vec<Arc<C>>>,
    pub config_alias_retirement: Option<ToolOriginalAliasBatch<C>>,
    pub refold_owed: bool,
    pub refold_request:u64,
    pub window_overlays: BTreeMap<String, WindowConfigSnapshot>,
    pub ingress: Option<ToolTag>,
    pub operations: Vec<(u64, ToolTag)>,
    pub tick: u64,
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub now_ms: Option<u64>,
}

impl<P, M:RetireOwned+Send+'static, C, CM> Default for ToolMachineRuntime<P, M, C, CM> {
    fn default() -> Self {
        Self {
            presses: ScrubLedger::default(),
            gestures: GestureLedger::default(),
            gesture_operations:std::array::from_fn(|_|None),
            gesture_retirement:None,
            gesture_ended: false,
            typing: TypingLedger::default(),
            overlay: None,
            overlay_generation: 0,
            provisional_generation: 0,
            config_overlay: None,
            config_overlay_generation: 0,
            pending_overlay_aliases: None,
            overlay_alias_retirement: None,
            pending_config_aliases: None,
            config_alias_retirement: None,
            refold_owed: false,
            refold_request:0,
            window_overlays: BTreeMap::new(),
            ingress: None,
            operations: Vec::new(),
            tick: 0,
            #[cfg(any(test, feature = "artifact-app-testing"))]
            now_ms: None,
        }
    }
}

impl<P, M: RetireOwned + protocol::StoreMutationFor<P>, C, CM: Mutation<C> + ColdMutation<C> + Clone + 'static> ToolMachineRuntime<P, M, C, CM> {
    /// 🔎️ The open typing runs.
    pub fn typing(&self) -> &TypingLedger<M> {
        &self.typing
    }

    /// 🖐️ Every window's open gesture.
    pub fn gestures(&self) -> &GestureLedger<M> {
        &self.gestures
    }

    /// 🪞️ The document every render seam reads: `committed` with every open press's, gesture's and run's provisional leaves, else
    /// `committed`. A command, a poll and a context menu keep deciding over what landed.
    pub fn overlay_or<'a>(&'a self, committed: &'a Arc<P>) -> &'a Arc<P> {
        self.overlay.as_ref().unwrap_or(committed)
    }

    /// 🧾️ Every open press's and run's provisional leaves in the overlay's fold order, in value form — what a derived view
    /// (a preview evaluated by a retained job) folds onto the committed document it reads; empty while nothing is open.
    pub fn provisional_values(&self) -> Vec<DslValue> {
        let typing = self.typing.windows().filter_map(|window| self.typing.open(window)).flat_map(|state| state.net.iter());
        self.presses.provisional().filter_map(PressLeaf::member).chain(self.gestures.provisional()).chain(typing).map(semio_framework_value::ToValue::to_value).collect()
    }

    /// 🔢️ Bumped whenever the overlay is refolded or dropped (a tick, a release, a host abort, a moved base), so a derived
    /// view knows its provisional input changed even when no edit landed.
    pub fn provisional_generation(&self) -> u64 {
        self.provisional_generation
    }

    /// ⏰️ The clock of one typing input: the wall clock with a logical tick unique to this instance, so two runs opened in
    /// the same millisecond never mint the same transaction id.
    pub fn clock(&mut self) -> HybridLogicalTimestamp {
        self.tick = self.tick.wrapping_add(1);
        #[cfg(any(test, feature = "artifact-app-testing"))]
        if let Some(physical_ms) = self.now_ms {
            return HybridLogicalTimestamp { actor: 0, physical_ms, logical: self.tick };
        }
        semio_framework_tool_machine::authoring_clock(self.tick)
    }

    /// 🪞️ Refolds the overlay on `committed` (store generation `generation`) when a press or run changed or the document
    /// moved under one. Scrub leaves are absolute and fold on any base (a leaf the base refuses is skipped); a typing run
    /// whose leaves the moved base refuses is a conflict and aborts with zero trace (`baseMoved`). Answers every snapshot
    /// alias the refold displaced (the caller retires them through the store, never plainly) and every aborted run.
    pub fn follow(&mut self, committed: &Arc<P>, generation: u64, changed: bool) -> (Vec<Arc<P>>, Vec<ToolStep<M>>) where P: Clone,M: protocol::StoreMutationFor<P> {
        if self.presses.provisional().all(|leaf| leaf.member().is_none()) && self.gestures.is_empty() && self.typing.is_empty() {
            let dropped: Vec<Arc<P>> = self.overlay.take().into_iter().collect();
            self.provisional_generation = self.provisional_generation.wrapping_add(u64::from(!dropped.is_empty()));
            return (dropped, Vec::new());
        }
        if !changed && self.overlay.is_some() && self.overlay_generation == generation {
            return (Vec::new(), Vec::new());
        }
        let mut displaced = Vec::new();
        let mut running: Option<Arc<P>> = None;
        for leaf in self.presses.provisional().filter_map(PressLeaf::member).chain(self.gestures.provisional()) {
            fold_leaf(committed, &mut running, &mut displaced, leaf);
        }
        let mut conflicts = Vec::new();
        for window in self.typing.windows() {
            let state = self.typing.open(window).expect("a listed run is open");
            let checkpoint = running.clone();
            if !state.net.iter().all(|leaf| fold_leaf(committed, &mut running, &mut displaced, leaf)) {
                displaced.extend(std::mem::replace(&mut running, checkpoint));
                conflicts.push(window.to_string());
            }
        }
        let aborted = conflicts.iter().map(|window| self.typing.abort(window, ToolAbortReason::BaseMoved)).collect();
        displaced.extend(self.overlay.replace(running.unwrap_or_else(|| Arc::clone(committed))));
        self.overlay_generation = generation;
        self.provisional_generation = self.provisional_generation.wrapping_add(1);
        (displaced, aborted)
    }

    /// 🪞️ The app config every render seam reads: committed with every held press's config mutations, else `committed`.
    pub fn config_overlay_or<'a>(&'a self, committed: &'a Arc<C>) -> &'a Arc<C> {
        self.config_overlay.as_ref().unwrap_or(committed)
    }

    /// 🪞️ Refolds the app-config overlay on `committed` (config generation `generation`) when a held press changed or the
    /// config moved under one; answers the displaced aliases (retired by the caller through the config store).
    pub fn follow_config(&mut self, committed: &Arc<C>, generation: u64, changed: bool) -> Vec<Arc<C>> where C: Clone,CM:protocol::ColdMutation<C> {
        if self.presses.provisional().all(|leaf| leaf.config().is_none()) {
            return self.config_overlay.take().into_iter().collect();
        }
        if !changed && self.config_overlay.is_some() && self.config_overlay_generation == generation {
            return Vec::new();
        }
        let (mut running, mut displaced) = (None, Vec::new());
        for leaf in self.presses.provisional().filter_map(PressLeaf::config) {
            fold_leaf(committed, &mut running, &mut displaced, leaf);
        }
        displaced.extend(self.config_overlay.replace(running.unwrap_or_else(|| Arc::clone(committed))));
        self.config_overlay_generation = generation;
        self.provisional_generation = self.provisional_generation.wrapping_add(1);
        displaced
    }
}
//#endregion 🔖️Runtime

//#region 🧪️Tests

//#endregion 🧪️Tests

/// 🔁️ Continuous overlays use the same original semantic producer as retained run operations.

#[path="🔁️follow/🦀️.rs"]
mod original_follow;
pub use original_follow::OriginalToolMachineFollow;

#[path="🧵️settlement/🦀️.rs"]
mod original_press_settlement;
pub use original_press_settlement::OriginalPressSettlement;

#[path="🏷️tag/🧵️original/🦀️.rs"]
mod original_tool_tag;
pub use original_tool_tag::{OriginalToolTagCapture,OriginalToolTagSource,OriginalToolTagKind,OriginalToolWindow};

impl<P,M:RetireOwned+Send+'static,C,CM> ToolMachineRuntime<P,M,C,CM>{
    pub fn operation_tag(&self,operation:u64)->Option<&ToolTag>{self.operations.iter().find(|(owner,_)|*owner==operation).map(|(_,tag)|tag)}

    pub fn take_operation(&mut self, operation: u64) -> Option<ToolTag> {
        let index = self.operations.iter().position(|(owner, _)| *owner == operation)?;
        Some(self.operations.remove(index).1)
    }

}
