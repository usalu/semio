//! 🎬️ Action context: everything one command arm may read or write while the dispatch prologue and epilogue stay in the dispatcher.

use super::*;

/// 🎬️ Everything one `🎮️commands/*` arm may read or write. The prologue/epilogue around the dispatch
/// match (window-option materialization, precompute sync, chrome effects, delta computation, config
/// snapshotting) stays in [`crate::editor::puzzle3d::dispatch::puzzle3d_handle_action`]; an arm only mutates this bundle.
pub struct Puzzle3dActionCtx<'a> {
    /// 🧠️ The app's long-lived precompute session and gumball scratch — every arm reaching them goes
    /// through `borrow_mut()`.
    pub session: &'a Puzzle3dSession,
    /// 🧠️ The document instance's retained operation owner — the brush suggestions link lives there. `None` on
    /// the unstaged adapter paths that run outside an admitted instance.
    pub instance_owner: Option<&'a ArtifactInstanceOperationOwnerHandle>,
    /// ⏯️ The instance's tool run as of command admission (`ArtifactOwnedToolJobContext::tool_run`) — the
    /// identity a run action this arm dispatches targets. `None` on paths admitted without it.
    pub tool_run: Option<&'a semio_framework_plugin::ToolRunView>,
    pub scene: &'a mut Puzzle3dScene,
    /// 🪟️ The window instance this action targets (already defaulted to the main window).
    pub window_id: &'a str,
    /// 🎛️ The pre-action config snapshot, for the few arms that must read state the scene runtime's
    /// materialized copy does not carry.
    pub config: &'a Puzzle3dRuntime,
    pub view_state: Option<&'a semio_framework_plugin::ViewModel>,
    /// 🕹️ Read-only view of the framework-owned `vortex` interaction domain (current selection plus
    /// `"pointer"` hover — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). Retained
    /// selection-acting verbs (delete/duplicate/focus/rotate/scale/translate-selection,
    /// select-same-kind, set-selection-flag, engagement-control-select) read it through the
    /// `selected_*_ids` helpers below instead of the deleted `Puzzle3dConfig` selection fields.
    pub interaction: &'a Puzzle3dInteractionSnapshot,
    pub ui_scope: &'a mut UiDirtyScope,
    pub effects: &'a mut Vec<Effect>,
    /// 🕹️ App-initiated selection writes this arm wants applied once its own mutations land — the
    /// single sanctioned way a reducer selects what it just created or widened to
    /// (`semio_framework_plugin::InteractionWrite`, applied by `VcsArtifactApp` through the same
    /// state machine the reserved `interactionSelect` verb uses). Empty for every arm that does not
    /// move the selection.
    pub interaction_writes: &'a mut Vec<InteractionWrite>,
    /// 🛑️ Set by an arm that must skip the whole epilogue (window save, delta, config snapshot).
    pub abort: bool,
    /// 🧬️ The typed document this action reads as its base — what the transform tool yields against.
    pub base: &'a std::sync::Arc<Puzzle3dSnapshot>,
    /// 🌱️ The admission's authoring seed every tool transaction this action commits is minted from; empty for
    /// a view without command authority, which then publishes the yielded mutations plainly.
    pub authoring_seed: &'a str,
    /// 🛠️ The mutations a committed tool transaction yielded — published after the scene delta.
    pub artifact_mutations: &'a mut Vec<Puzzle3dMutation>,
    /// 🛠️ The tool transaction this action committed — stamped on every op it publishes.
    pub transaction: &'a mut Option<protocol::TransactionRef>,
}

impl<'a> Puzzle3dActionCtx<'a> {
    /// 🔗️ Runs `apply` on this instance's brush suggestions link; `None` without an instance owner or while
    /// the owner is busy.
    pub fn brush_suggestions<R>(&self, apply: impl FnOnce(&mut BrushSuggestionsLink) -> R) -> Option<R> {
        self.instance_owner?.with_mut::<Puzzle3dInstanceOperationOwner, _>(|owner| Ok(apply(&mut owner.brush_suggestions))).ok()
    }

    pub fn selected_object_ids(&self) -> Vec<String> {
        self.interaction.selected_object_ids().to_vec()
    }
    pub fn selected_vortex_ids(&self) -> Vec<String> {
        self.interaction.selected_vortex_ids().to_vec()
    }
    pub fn selected_attraction_ids(&self) -> Vec<String> {
        self.interaction.selected_attraction_ids().to_vec()
    }
    pub fn selected_target_volume_ids(&self) -> Vec<String> {
        self.interaction.selected_target_volume_ids().to_vec()
    }
    pub fn selected_reference_ids(&self) -> Vec<String> {
        self.interaction.selected_reference_ids().to_vec()
    }

    /// 🕹️ Replaces this app's whole `vortex` selection with `ids` at `granularity` once the action's
    /// own mutations have landed — "select the thing I just created/widened to".
    pub fn replace_selection(&mut self, granularity: &str, ids: impl IntoIterator<Item = String>) {
        let write = InteractionWrite::replace(PUZZLE3D_INTERACTION_DOMAIN, granularity, ids);
        if !write.targets.is_empty() {
            self.interaction_writes.push(write);
        }
    }

    /// 🧹️ Empties this app's whole `vortex` selection through the same sanctioned reducer channel
    /// [`Self::replace_selection`] uses. It is expressed as a `Subtractive` write naming exactly what
    /// is selected right now, NOT an empty `Replace`: the framework's state machine returns the
    /// current selection unchanged when a write names no target at all (`protocol::next_selection`),
    /// so an empty `Replace` is a silent no-op. Nothing selected means nothing to clear.
    pub fn clear_selection(&mut self) {
        let targets: Vec<InteractionTarget> = [
            (PUZZLE3D_GRANULARITY_OBJECT, self.selected_object_ids()),
            (PUZZLE3D_GRANULARITY_VORTEX, self.selected_vortex_ids()),
            (PUZZLE3D_GRANULARITY_ATTRACTION, self.selected_attraction_ids()),
            (PUZZLE3D_GRANULARITY_TARGET_VOLUME, self.selected_target_volume_ids()),
            (PUZZLE3D_GRANULARITY_REFERENCE, self.selected_reference_ids()),
        ]
        .into_iter()
        .flat_map(|(granularity, ids)| ids.into_iter().map(move |id| InteractionTarget { granularity: granularity.to_string(), id }))
        .collect();
        if targets.is_empty() {
            return;
        }
        self.interaction_writes.push(InteractionWrite { domain: PUZZLE3D_INTERACTION_DOMAIN.into(), targets, merge: MergeMode::Subtractive });
    }

    /// 🧯️ Surfaces ONE bounded, localized notice through the shell's transient-notice channel
    /// (`Effect::Notify` → `ShellHost`'s `showTransientNotice`) — how an arm whose engine work was
    /// REJECTED tells the user, instead of the silence
    /// `addBrushObject`/`acceptSuggestion` used to fall through (`📓️2026-09-09-user-feature-
    /// checklist.md` §9/§13). At most one notice per action: a second call replaces nothing and adds
    /// nothing, so the effect list stays fixed-width. An unauthored locale×terminology axis carries
    /// the app's own `ui.localization.unsupported` code rather than an English sentence — this UI has
    /// no default language, and a silent drop would restore exactly the defect being fixed.
    /// 🎯️ Refuses a selection-scoped command that has nothing to act on: ONE localized notice, then the
    /// same `abort` `select_same_kind` already uses, so no empty document edit and no empty interaction
    /// write is emitted. Answers whether it refused, so an arm reads
    /// `if ctx.refuse_without_selection(&ids) { return }`.
    ///
    /// 🕹️ Why this exists: every one of these arms used to run to completion over an EMPTY id list —
    /// `duplicate_selection` cloned nothing and then called `replace_selection` with no targets,
    /// `translate/rotate/scale_selection` moved nothing, `delete_selection` retained everything. The
    /// emitted delta was empty, so the framework logged no command row and the outliner never changed:
    /// in the browser "Duplicate Selection" was indistinguishable from a dead menu row (measured
    /// 2026-09-09 21:05, no notice of any kind). A refusal must be visible.
    pub fn refuse_without_selection(&mut self, ids: &[String]) -> bool {
        if !ids.is_empty() {
            return false;
        }
        self.notice(|labels| labels.nothing_selected.as_str());
        self.abort = true;
        true
    }

    pub fn refuse_when_locked(&mut self) -> bool {
        self.notice(|labels| labels.selection_locked.as_str());
        self.abort = true;
        true
    }

    /// 🧲️ One gumball pose delta (`translateSelection`/`rotateSelection`/`scaleSelection`) as ONE tool
    /// transaction over the gesture's own object ids (else the selected objects) and the selected target volumes.
    pub fn commit_gumball(&mut self, verb: &str, args: Option<&Value>) {
        let targets = [mesh_selection_ids(args, &self.selected_object_ids()), self.selected_target_volume_ids()].concat();
        if let Some(record) = semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dSelectionRecord::from_gumball(verb, args, targets) {
            self.commit_selection(verb, vec![record]);
        }
    }

    /// 🛠️ Commits `records` through the transform tool machine as ONE tool transaction of this action — the
    /// parametric selection leaves plus the attractions their drops land, yielded as `verb`. Nothing named is
    /// the `nothing_selected` refusal; a request whose every target is locked is the `selection_locked` one;
    /// a request with a movable target is yielded whole, and its leaf reports the locked rest as
    /// `mutation.partial`. A motionless request leaves zero trace.
    pub fn commit_selection(&mut self, verb: &str, records: Vec<semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dSelectionRecord>) {
        if records.iter().all(|record| record.targets.is_empty()) {
            self.refuse_without_selection(&[]);
            return;
        }
        if records.iter().any(|record| record.refused_as_locked(self.base)) && !records.iter().any(|record| record.applies_to(self.base)) {
            self.refuse_when_locked();
            return;
        }
        let request = semio_s_artifact_puzzle_3d_editor_transform::TransformToolRequest { base: std::sync::Arc::clone(self.base), records };
        if let Some((transaction, mutations)) = semio_s_artifact_puzzle_3d_editor_transform::puzzle3d_transform_tool_commit(verb, self.authoring_seed, request) {
            *self.transaction = (!self.authoring_seed.is_empty()).then_some(transaction);
            self.artifact_mutations.extend(mutations);
        }
    }

    pub fn notice(&mut self, message: impl Fn(&Puzzle3dLabels) -> &'static str) {
        if self.effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })) {
            return;
        }
        let text = self.view_state.and_then(puzzle3d_labels).map_or_else(|| PUZZLE3D_LOCALIZATION_UNSUPPORTED.to_string(), |labels| message(labels).to_string());
        self.effects.push(Effect::Notify { message: text });
    }
}
