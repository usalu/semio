//! 🧭️ Interaction verb arguments, domain topology derivation, selection loss and inline interaction verbs.
use semio_framework_value::DslValue;
use semio_framework::kernel::UiDirtyScope;
use semio_framework::{kernel::Effect, Fault, InteractionDefinition, CLEAR_SELECTION_ACTION_ID, INTERACTION_HOVER_ACTION_ID, INTERACTION_SELECT_ACTION_ID, SELECT_ALL_ACTION_ID, SET_INTERACTION_GRANULARITY_ACTION_ID, SET_SELECTION_MODE_ACTION_ID};
/// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-flip`, 26/08/20): the choke point. Every
/// UI-bearing plugin in the fleet imports its UI vocabulary through THIS block, so flipping it
/// here redirects all 33 at once — `ui_wgpu` leaves the guest dependency graph entirely
/// (registrar-request: drop it from this crate's `Cargo.toml`, add `semio-framework-ui-contract`
/// and `semio-framework-ui-runtime` in its place). `Ui*Node`/`UiPresence`/`UiControlNode`/
/// `ActionDescriptor` have no drop-in replacement of the same name — see
/// `📓️recipe-plugin.md` in this ticket's folder for the variant-by-variant mechanical
/// translation (`UiNode::Text{..}` → `ui::text(..)`, `ActionDescriptor` → `ActionBinding`, …).
/// `ui::*` is the contract crate's own builder DSL (`ui::stack()`, `ui::button()`, …); the bare
/// glob brings every contract type (`Component`, `UiPatch`, `Label`, `ActionId`, …) into scope
/// too, since this module already used every one of the old wgpu-target types unqualified.
use semio_framework_ui_contract::*;
use std::collections::BTreeMap;
use crate::sdk_fault::plugin_sdk_fault;
use crate::window_surface::UiAssemblyResult;
use crate::window_surface::ui_assembly_error;



/// 🕹️ Reads the required `domainId` text arg every interaction verb except `clearSelection`/
/// `selectAll` carries (see `interaction_action_definitions`'s arg declarations).
pub(crate) async fn interaction_domain_id_arg(args: Option<&DslValue>, action: &str) -> Result<String, Fault> {
    args.and_then(|value| value.get("domainId")).and_then(DslValue::as_str).map(str::to_string).ok_or_else(|| plugin_sdk_fault(format!("{action} missing required arg domainId")))
}

/// 🕹️ Decodes the required `targets` arg — a JSON-encoded `Vec<InteractionTarget>`, matching the
/// `ActionArgDef::text` declaration `interaction_action_definitions` gives both `interactionSelect`
/// and `interactionHover` (a renderer JSON-serializes its gathered pick/marquee/hover batch into
/// this one string arg, the same convention other JSON-blob action args in this codebase already use).
pub(crate) async fn parse_interaction_targets(args: Option<&DslValue>, action: &str) -> Result<Vec<protocol::InteractionTarget>, Fault> {
    let raw = args.and_then(|value| value.get("targets")).and_then(DslValue::as_str).ok_or_else(|| plugin_sdk_fault(format!("{action} missing required arg targets")))?;
    serde_json::from_str(raw).map_err(|error| plugin_sdk_fault(format!("{action}: malformed targets ({error})")))
}

/// 🕹️ Decodes `interactionSelect`'s `merge` select-arg — defaults to `Replace` (a plain pick) when
/// absent, matching the un-modified-click case every renderer's modifier→merge policy falls back to.
/// The vocabulary is NOT spelled here: [`protocol::MergeMode::from_wire_label`] is the single codec
/// over this module's own schema enum (`🕹️interaction/🧬️schema/🔣️.json`), pinned by
/// `🕹️interaction/🧫️fixtures/🎯️merge-modes.json`.
pub(crate) async fn parse_merge_mode(args: Option<&DslValue>) -> Result<protocol::MergeMode, Fault> {
    let raw = args.and_then(|value| value.get("merge")).and_then(DslValue::as_str).unwrap_or("replace");
    protocol::MergeMode::from_wire_label(raw).ok_or_else(|| plugin_sdk_fault(format!("interactionSelect: unknown merge '{raw}'")))
}

/// 🕹️ Decodes `setSelectionMode`'s `mode` select-arg.
pub(crate) async fn parse_selection_mode(raw: &str) -> Result<protocol::SelectionMode, Fault> {
    match raw {
        "single" => Ok(protocol::SelectionMode::Single),
        "multiple" => Ok(protocol::SelectionMode::Multiple),
        other => Err(plugin_sdk_fault(format!("setSelectionMode: unknown mode '{other}'"))),
    }
}

/// 🕹️ `HierarchyProvider::PathDelimited`'s wrapper self-derivation (see `resolve_domain_topology`'s
/// doc for the "known ids only" limitation): every non-empty prefix chain of each id in `known_ids`,
/// split on `delimiter`, becomes one `TopologyNode` (`granularity` from `def.granularities[depth]`,
/// clamped to the last declared granularity for a deeper id than declared granularities), parented
/// to the next-shallower prefix. Dedup'd (and stably ordered) by id via the `BTreeMap` key.
pub(crate) async fn derive_path_delimited_topology(def: &InteractionDefinition, delimiter: &str, known_ids: impl Iterator<Item = String>) -> protocol::DomainTopology {
    let mut nodes: BTreeMap<String, protocol::TopologyNode> = BTreeMap::new();
    for id in known_ids {
        let segments: Vec<&str> = id.split(delimiter).collect();
        let mut prefix = String::new();
        let mut parent: Option<String> = None;
        for (depth, segment) in segments.iter().enumerate() {
            if depth > 0 {
                prefix.push_str(delimiter);
            }
            prefix.push_str(segment);
            let granularity_index = depth.min(def.granularities.len().saturating_sub(1));
            let granularity = def.granularities.get(granularity_index).map(|granularity| granularity.id.clone()).unwrap_or_default();
            nodes.entry(prefix.clone()).or_insert_with(|| protocol::TopologyNode { id: prefix.clone(), granularity, parent: parent.clone() });
            parent = Some(prefix.clone());
        }
    }
    protocol::DomainTopology { ordered: nodes.into_values().collect() }
}

/// 🌳️ `HierarchyProvider::UiTree`'s wrapper self-derivation: pre-order-walks a rendered `Tree`
/// node's `children` — `Component::TreeSection` children hold `Component::TreeItem` rows, and a
/// `TreeItem`'s own `children` may nest further `TreeItem`s (SEMANTIC-UI-CONTRACT §2: sections/
/// items are ordinary children now, not the old `UiTreeSectionNode.items`/`UiTreeItemNode.items`
/// inline fields) — into `TopologyNode`s, parented by tree nesting, keyed by each row's
/// [`crate::BuiltNode::key`]/[`TreeNode::key`] (the old `UiTreeItemNode.id`'s direct successor).
/// `granularity` is applied uniformly to every node — a `TreeItem` carries no per-row granularity
/// of its own, so this assumes (matching every `UiTree`-hierarchy domain in the inventory —
/// writer's AST, note's blocks, …) that a `UiTree`-bound domain's rendered tree is
/// single-granularity; a domain needing per-row granularity declares `HierarchyProvider::Topology`
/// and supplies its own `interaction_topology` instead.
///
/// 🪟️ A windowed tree ([`TreeWindows`]) feeds only its MATERIALISED rows here, so the derived
/// topology covers the visible slice and not the container's full `TreeWindow::total` — harmless
/// today because no app in the fleet declares `HierarchyProvider::UiTree`; a domain that needs the
/// complete hierarchy must declare `HierarchyProvider::Topology`.
pub(crate) async fn ui_tree_domain_topology(sections: &BuiltChildren, granularity: &str) -> UiAssemblyResult<protocol::DomainTopology> {
    struct Frame<'a> {
        children: &'a BuiltChildren,
        next: usize,
        parent: Option<UiText>,
    }
    let mut ordered = Vec::new();
    let mut stack: UiFixedList<Frame<'_>, { semio_framework_ui_runtime::COMPONENT_TREE_PRODUCER_DEPTH }> = UiFixedList::default();
    stack.try_push(Frame { children: sections, next: 0, parent: None }).map_err(|_| ui_assembly_error("interaction-topology.depth"))?;
    while let Some(mut frame) = stack.pop() {
        let Some(node) = frame.children.get(frame.next) else { continue };
        frame.next = frame.next.checked_add(1).ok_or_else(|| ui_assembly_error("interaction-topology.cursor"))?;
        let inherited_parent = frame.parent.clone();
        stack.try_push(frame).map_err(|_| ui_assembly_error("interaction-topology.depth"))?;
        if matches!(node.component, Component::TreeSection(_)) {
            stack.try_push(Frame { children: &node.children, next: 0, parent: inherited_parent }).map_err(|_| ui_assembly_error("interaction-topology.depth"))?;
        } else if matches!(node.component, Component::TreeItem(_)) {
            ordered.push(protocol::TopologyNode { id: node.key.to_string(), granularity: granularity.to_string(), parent: inherited_parent.as_ref().map(ToString::to_string) });
            stack.try_push(Frame { children: &node.children, next: 0, parent: Some(node.key.clone()) }).map_err(|_| ui_assembly_error("interaction-topology.depth"))?;
        }
    }
    Ok(protocol::DomainTopology { ordered })
}

/// 🈳️ Lazily-initialized empty fallbacks `InteractionView`'s accessors return for an undeclared or
/// never-touched domain — one static pair shared across every instance, so the accessor methods
/// below can return a plain `&DomainSelection`/`&DomainHover` (no `Option`, no per-call allocation)
/// exactly like the ticket's specified signature.
// 🚫️async: E1-adjacent pure lazily-initialized static accessor consumed by `unwrap_or_else`/
// `match`'s sync arms in `InteractionView::selection`/`hover` — see R9.
pub(crate) fn empty_domain_selection() -> &'static protocol::DomainSelection {
    static EMPTY: std::sync::OnceLock<protocol::DomainSelection> = std::sync::OnceLock::new();
    EMPTY.get_or_init(protocol::DomainSelection::default)
}

// 🚫️async: E1-adjacent pure lazily-initialized static accessor consumed by `unwrap_or_else`/
// `match`'s sync arms in `InteractionView::selection`/`hover` — see R9.
pub(crate) fn empty_domain_hover() -> &'static protocol::DomainHover {
    static EMPTY: std::sync::OnceLock<protocol::DomainHover> = std::sync::OnceLock::new();
    EMPTY.get_or_init(protocol::DomainHover::default)
}

/// 🈳️ The draft references of a view no time-travel draft feeds — a dispatch, presence stamping, a viewer law.
pub(crate) fn empty_draft_references() -> &'static BTreeMap<String, Vec<String>> {
    static EMPTY: BTreeMap<String, Vec<String>> = BTreeMap::new();
    &EMPTY
}

/// 🧮️ The ONE flat projection of an `InteractionState`'s selection half: every domain's ids as a
/// SET — each topology id at most once, in first-seen domain order. A selection is a set of
/// topology ids, so the flattened publication must be one too: `overlay_leftover_ids_into_vortex`
/// deliberately republishes another domain's ids under a second `vortex` key, so a plain flatten
/// reports a single pick once per mirroring domain. Measured live on 2026-09-12 (ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️selection-dedupe-2026-09-12.md`): one click on the
/// generation3d preview published `selectedIds: ["extrude@solid","extrude@solid"]` while `locked`
/// — an object, so keyed by construction — carried the id exactly once.
// 🚫️async: E1 pure fold over an owned map, consumed by `InteractionView::leftover_selected_ids`
// and `leftover_interaction_view_from`'s sync bodies — see R9.
pub(crate) fn leftover_selected_ids_of(state: &protocol::InteractionState) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    for selection in state.selection.values() {
        for id in &selection.ids {
            if !ids.iter().any(|existing| existing == id) {
                ids.push(id.clone());
            }
        }
    }
    ids
}

/// 🧲️ Leftover `interactionView.gumball.active` — always false. Selection leftovers exist so
/// Inspection/outliner/gumball-target consumers see ids before the guest lane repaints; arming the
/// world gumball is app-authored on the guest `selectionJson` lane (`gumballActive` +
/// `transformMode` keyed off the armed utility), never inferred from bare selected ids (puzzle3d
/// ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub(crate) fn leftover_gumball_active_v1() -> bool {
    false
}

/// 🕳️ Which revalidation pass this is, because the same pruning is a feature in one and a defect
/// in the other: `DocumentChange` runs after every artifact dispatch precisely to drop ids the
/// document no longer has, `Pick` carries ids a user click or an app's own `InteractionWrite`
/// just named and owes them to the render.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum InteractionRevalidateOrigin {
    Pick,
    DocumentChange,
}

/// 🕹️ What one applied interaction verb hands back to whichever dispatch carried it
/// ([`VcsArtifactApp::apply_interaction_verb`]): the app-declared refresh scope for the verb and
/// the domains it touched, and the leftover `InteractionView` publication the host peels from
/// `Invocation.output.interactionView` (`interactionViewFromLeftoverOutput`, `🏛️ShellHost`).
pub(crate) struct AppliedInteractionVerb {
    pub(crate) scope: UiDirtyScope,
    pub(crate) leftover: DslValue,
}

/// 🔀️ The outcome of folding a guest emit's interaction verbs inline
/// ([`VcsArtifactApp::fold_inline_interaction_verbs`]): the carrying emit's `ui_scope` widened by
/// every applied verb's scope, the LAST applied verb's leftover view (the host publishes one
/// leftover per invocation, and a later verb's snapshot already includes the earlier ones), and
/// one diagnostic per verb that faulted instead of applying.
pub(crate) struct InlineInteractionFold {
    pub(crate) scope: UiDirtyScope,
    pub(crate) leftover: Option<DslValue>,
    pub(crate) diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
}

/// 🩹 Diagnostic code of a guest-emitted interaction verb the reactor could not apply inline —
/// attached to the carrying command's `InvocationResult.diagnostics`, never raised as its fault.
pub(crate) const INLINE_INTERACTION_VERB_DIAGNOSTIC_CODE: &str = "interaction.inline-verb-not-applied";

/// 🔀️ Whether this effect is a guest asking the host to (re)dispatch one of the six framework
/// interaction verbs (`INTERACTION_ACTION_IDS`), in EITHER of the two shapes apps emit for that
/// one intent: `Effect::ReplayShellCommand { action_id ∈ … }` (what `interaction_select_effect`
/// / `interaction_hover_effect` build in most apps) and `Effect::DispatchAction { action ∈ … }`
/// (what `📏️layout`'s `layout_select_effect`/`layout_hover_effect`/`layout_clear_selection_effect`
/// and `💡️reasoning`'s `wires_select_effect` build). Every other `ReplayShellCommand` (an
/// undo/redo inverse replay, an `os.*` shell relay) and every other `DispatchAction` (a
/// self-dispatch chain, a `probeChildContinuation`, a traced `canvasPointerDown` redispatch) is
/// NOT one and keeps reaching the host untouched.
///
/// ⏱️ A `DispatchAction` interaction verb with a positive `delay_ms` still folds inline: the delay
/// was only ever a host-scheduling artefact (a later `onAction` tick) for a state write that is
/// synchronous in the reactor — deferring a pick behind pointer input issued meanwhile is exactly
/// the ordering hazard §2 C removes, so no delay is honoured for these six verbs.
///
/// 🔮️ The `req: RequestId` such an effect carries is never a parked `RequestFuture`: a registry-
/// minted `DispatchAction` (`Host::dispatch_action` → `RequestRegistry::request`) queues on the
/// registry's own outbound lane and never enters `Emit.effects`, so every one peeled here is an
/// app struct literal with a placeholder id nothing awaits — dropping it completes nothing late.
pub(crate) fn is_inline_interaction_verb(effect: &Effect) -> bool {
    match effect {
        Effect::ReplayShellCommand { action_id, .. } => INTERACTION_ACTION_IDS.contains(&action_id.as_str()),
        Effect::DispatchAction { action, .. } => INTERACTION_ACTION_IDS.contains(&action.as_str()),
        _ => false,
    }
}

/// 🔀️ Peels every guest-emitted interaction verb out of an emit's effect list — both shapes
/// [`is_inline_interaction_verb`] admits, in one pass so emission order is kept ACROSS the two
/// shapes — leaving every other effect in place. The host never sees a peeled verb; the reactor
/// applies it inline instead ([`VcsArtifactApp::fold_inline_interaction_verbs`]).
pub(crate) fn take_inline_interaction_verbs(effects: &mut Vec<Effect>) -> Vec<(String, Option<DslValue>)> {
    if !effects.iter().any(is_inline_interaction_verb) {
        return Vec::new();
    }
    let mut folded = Vec::new();
    effects.retain_mut(|effect| {
        if !is_inline_interaction_verb(effect) {
            return true;
        }
        match effect {
            Effect::ReplayShellCommand { action_id, args } => folded.push((std::mem::take(action_id), args.take())),
            Effect::DispatchAction { action, args, .. } => folded.push((std::mem::take(action), args.take())),
            _ => unreachable!("is_inline_interaction_verb admits exactly the two arms above"),
        }
        false
    });
    folded
}

/// 🕳️ Whether a `;`-joined [`interaction_selection_witness`] names at least one id — an empty
/// selection and a present-but-empty domain (`vortex=object:`) are both "nothing was carried".
fn interaction_witness_names_ids(witness: &str) -> bool {
    witness.split(';').any(|entry| entry.split_once(':').is_some_and(|(_, ids)| !ids.is_empty()))
}

/// 🕳️ The ONE predicate that names how a pick disappeared between the ids a dispatch carried and
/// the ids the app will actually render from, instead of leaving the two silently unequal:
/// `validate-state-pruned` when [`protocol::validate_state`] dropped them against the topology it
/// was handed, `persist-skipped` when the validated half compared equal to the stored one so no
/// edit was ever minted, `store-readback-lost` when the store took the edit and then did not
/// answer with it. `None` is the healthy case — the render source carries exactly what was
/// dispatched, whether through the store or through the leftover overlay
/// [`VcsArtifactApp::interaction_selection_snapshot`] lays over it.
///
/// 🔬️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B23: every browser symptom of the
/// selection-scoped family (`inspection-object-fields id=null`, `locked-flag-row`,
/// `gumball-scene-delta`, `relocate-pose-delta`) is this predicate answering `Some` where nothing
/// ever said so.
pub(crate) fn interaction_selection_loss_v1(dispatched: &str, validated: &str, readback: &str, minted: bool) -> Option<&'static str> {
    if !interaction_witness_names_ids(dispatched) || readback == dispatched {
        return None;
    }
    if validated != dispatched {
        return Some("validate-state-pruned");
    }
    if !minted {
        return Some("persist-skipped");
    }
    Some("store-readback-lost")
}

/// 🧹️ Retires the leftover selection overlay for the domains an APP-authored selection write just
/// moved, and answers the pair `(overlay, leftover_ids)` that must replace the live ones.
///
/// The overlay exists to paper over one specific hole: a pick whose ids the interaction store has
/// not answered with yet, or that `validate_state` pruned, still has to reach the render
/// ([`VcsArtifactApp::interaction_selection_snapshot`] lays it over an EMPTY store selection). It
/// is written only by the framework-reserved pick route, so before this rule existed it also
/// papered over a deliberate emptying: an app that empties its own selection through
/// `Emit.interaction_writes` (`MergeMode::Subtractive` — every `ctx.clear_selection()` in every
/// plugin) had the store answer `[]` correctly and then read the PRE-write ids straight back off
/// the overlay, forever. Measured 2026-09-12 in-process (ticket 26/09/02/PUZZLE-3D-END-TO-END wave
/// B31): `write merge=Subtractive current=[object] next=[]`, `persisted=Some([])`, then
/// `snapshot store=Some([]) overlay=Some([object])` — puzzle3d's `deleteSelection` kept its own
/// victim selected and its `clear` verb could not clear.
///
/// A write names the newest truth for its domain, so that domain's overlay entry BECOMES the
/// freshly computed selection (empty included) and the flat leftover ids keep only what is still
/// selected somewhere. Domains the write never named are untouched, so an unrelated pick in flight
/// keeps its cover.
pub(crate) fn leftover_after_app_selection_write_v1(overlay: Option<&protocol::InteractionState>, leftover_ids: &[String], next: &protocol::InteractionState, domains: &[String]) -> (Option<protocol::InteractionState>, Vec<String>) {
    let retired = overlay.map(|overlay| {
        let mut retired = overlay.clone();
        for domain in domains {
            match next.selection.get(domain) {
                Some(selection) => retired.selection.insert(domain.clone(), selection.clone()),
                None => retired.selection.remove(domain),
            };
        }
        retired
    });
    let live: Vec<&String> = next.selection.values().flat_map(|selection| selection.ids.iter()).collect();
    (retired, leftover_ids.iter().filter(|id| live.contains(id)).cloned().collect())
}

/// 🧹️ What a DOCUMENT CHANGE leaves of the just-validated interaction state, of the leftover
/// cover laid over it, and of the flat leftover ids — the ONE rule that makes a whole-document
/// replacement (loading another bundled example, importing a file, reverting) drop the ids the
/// new document does not have.
///
/// Two holes, both measured on the generation3d React playground 2026-09-14 (ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️selection-prune-interact-2026-09-14.md`), let a pick
/// made in one example outlive it into every later one:
///
/// 1. [`protocol::validate_state`] can only prune a domain the topology has an entry for, and
///    [`VcsArtifactApp::build_full_interaction_topology`] builds entries for DECLARED domains
///    only. The `vortex` mirror [`VcsArtifactApp::overlay_leftover_ids_into_vortex`] writes for
///    the panels that read that name is not a declared domain of an app like generation3d, so it
///    was never checked against anything and carried `shell@solid` through all eight examples.
///    A mirror is a PROJECTION of the real domains, so it may only name what a declared domain
///    still names after validation.
/// 2. The leftover cover exists to carry a pick the interaction store has not answered with yet
///    ([`VcsArtifactApp::interaction_selection_snapshot`] lays it over an EMPTY store selection).
///    A document change empties that store selection legitimately — so the cover papered straight
///    over the prune and put the stale ids back on the very next read. A cover may not outlive
///    the ids it was covering.
///
/// A document edit that touches nothing selected leaves all three untouched, so an in-flight pick
/// during an unrelated edit keeps its cover.
pub(crate) fn interaction_after_document_change_v1(
    validated: &protocol::InteractionState,
    declared_domains: &[String],
    overlay: Option<&protocol::InteractionState>,
    leftover_ids: &[String],
) -> (protocol::InteractionState, Option<protocol::InteractionState>, Vec<String>) {
    let declared_live: Vec<String> = validated.selection.iter().filter(|(domain, _)| declared_domains.contains(domain)).flat_map(|(_, selection)| selection.ids.iter().cloned()).collect();
    let mut pruned = validated.clone();
    for (domain, selection) in pruned.selection.iter_mut() {
        if declared_domains.contains(domain) {
            continue;
        }
        selection.ids.retain(|id| declared_live.contains(id));
        if selection.anchor_id.as_ref().is_some_and(|anchor| !selection.ids.contains(anchor)) {
            selection.anchor_id = None;
        }
    }
    for (domain, hover) in pruned.hover.iter_mut() {
        if declared_domains.contains(domain) {
            continue;
        }
        hover.ids.retain(|id| declared_live.contains(id));
    }
    let retired = overlay.map(|overlay| {
        let mut retired = overlay.clone();
        for selection in retired.selection.values_mut() {
            selection.ids.retain(|id| declared_live.contains(id));
            if selection.anchor_id.as_ref().is_some_and(|anchor| !selection.ids.contains(anchor)) {
                selection.anchor_id = None;
            }
        }
        retired
    });
    (pruned, retired, leftover_ids.iter().filter(|id| declared_live.contains(id)).cloned().collect())
}

/// 🕳️ Records one selection loss on a channel the BROWSER actually shows.
///
/// Deliberately not [`crate::plugin_runtime::debug_runtime_line`]: that channel is gated on
/// `semio_framework_trace::runtime_diagnostics_enabled()`, which reads an environment variable a
/// jco-transpiled component in a browser tab does not have, so it is off in exactly the place this
/// defect lives — measured on wasm #48 (`🗑️generated/probe-2026-09-11T19-11-07.md`: zero
/// `debug_runtime_line` records in 4 000 console lines, against thousands of plain `eprintln!`
/// ones from the same guest). A lost pick is a defect report, not a trace, and is emitted only
/// when one actually happened — the same rule the renderer's own permanent
/// `refreshUi dropped requested body` record follows.
pub(crate) fn report_interaction_selection_loss(reason: &str, dispatched: &str, validated: &str, readback: &str, topology: &protocol::InteractionTopology) {
    let domains = topology.domains.iter().map(|(domain, topo)| format!("{domain}:{}", topo.ordered.len())).collect::<Vec<_>>().join(",");
    eprintln!("interaction selection lost reason={reason} dispatched={dispatched} validated={validated} readback={readback} domains={domains}");
}

#[cfg(test)]
#[path = "../../🧪️tests/🕹️interaction-selection-laws/🦀️.rs"]
mod interaction_selection_laws;

/// 🕹️ The six framework-owned interaction actions `dispatch_action` intercepts before any of the
/// other framework-reserved branches — see `dispatch_interaction_action`'s own doc comment.
pub(crate) const INTERACTION_ACTION_IDS: [&str; 6] = [INTERACTION_SELECT_ACTION_ID, INTERACTION_HOVER_ACTION_ID, CLEAR_SELECTION_ACTION_ID, SELECT_ALL_ACTION_ID, SET_SELECTION_MODE_ACTION_ID, SET_INTERACTION_GRANULARITY_ACTION_ID];
