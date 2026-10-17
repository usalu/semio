//! 🕹️ Interaction of the puzzle 5d editor: copy and paste helpers, the context menu, the `Puzzle5dCommand` vocabulary with its variant macro and the action context every command arm reads and writes. See [`super`].

use crate::editor::puzzle5d::snapshot::Puzzle5dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::editor::puzzle5d::config::Puzzle5dConfigMutation;
use semio_s_artifact_puzzle_3d_editor::Puzzle3dInstanceOperationOwner;
use crate::{editor::puzzle5d::terminology::puzzle5d_labels, editor::puzzle5d::terminology::Puzzle5dLabels};
use semio_framework_plugin::{kernel::ClipboardError, kernel::ClipboardFragment, kernel::Effect, kernel::PasteAnchor, kernel::PastePlacement, kernel::UiDirtyScope};
use semio_framework_plugin::Emit;
use semio_framework_plugin::InteractionTarget;
use semio_framework_plugin::InteractionWrite;
use semio_framework_ui_locale::Label;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::app::InteractionView;
use semio_framework_value::FromValue;
use semio_framework_pack_json::{parse, Value};
use std::{collections::HashMap, collections::HashSet};
use crate::editor::puzzle5d::entities::{PUZZLE5D_GRANULARITY_FASTENER, PUZZLE5D_GRANULARITY_GRIP, PUZZLE5D_GRANULARITY_PART, PUZZLE5D_GRANULARITY_TARGET_VOLUME, PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_PLAY_APP_ID, PUZZLE5D_SCHEMA, Puzzle5dDocument, Puzzle5dFastener, Puzzle5dFreshIds, Puzzle5dPart, mesh_selection_ids, puzzle5d_document_from_snapshot, puzzle5d_editor_projection};
use crate::editor::puzzle5d::view::{Puzzle5dInteractionSnapshot, Puzzle5dScene};
use crate::editor::puzzle5d::config::Puzzle5dConfig;
use crate::editor::puzzle5d::modes::edit;
use crate::editor::puzzle5d::modes::edit::windows::world3d;
use crate::editor::puzzle5d::presence::{Puzzle5dPresence, Puzzle5dPresenceMutation};
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;

//#region 🔖️CopyPaste
/// 🧩️ The part id a `"part_id:grip_id"` full grip reference belongs to.
pub fn owning_part_id_local(grip_ref: &str) -> &str {
    grip_ref.split(':').next().unwrap_or(grip_ref)
}

pub fn rewrite_grip_ref_local(grip_ref: &str, id_map: &HashMap<String, String>) -> String {
    match grip_ref.split_once(':') {
        Some((part_id, grip_id)) => match id_map.get(part_id) {
            Some(fresh_part_id) => format!("{fresh_part_id}:{grip_id}"),
            None => grip_ref.to_string(),
        },
        None => grip_ref.to_string(),
    }
}

/// 🧮️ Closure-selects a copy fragment: expands the part set to include every selected fastener's
/// endpoint parts, then expands the fastener set to include every fastener whose BOTH endpoints are
/// now in the part set — the untyped structural-twin twin of
/// `crate::standards::v1::subsets::any::schema::transfer::copy_selection`.
pub fn copy_selection_local(document: &Puzzle5dDocument, part_ids: &[String], fastener_ids: &[String]) -> (Vec<Puzzle5dPart>, Vec<Puzzle5dFastener>) {
    let mut part_set: HashSet<String> = part_ids.iter().cloned().collect();
    for fastener in &document.fasteners {
        if fastener_ids.contains(&fastener.id) {
            part_set.insert(owning_part_id_local(&fastener.source).to_string());
            part_set.insert(owning_part_id_local(&fastener.target).to_string());
        }
    }
    let mut fastener_set: HashSet<String> = fastener_ids.iter().cloned().collect();
    if !part_set.is_empty() {
        for fastener in &document.fasteners {
            let source_part = owning_part_id_local(&fastener.source);
            let target_part = owning_part_id_local(&fastener.target);
            if part_set.contains(source_part) && part_set.contains(target_part) {
                fastener_set.insert(fastener.id.clone());
            }
        }
    }
    let parts = document.parts.iter().filter(|part| part_set.contains(&part.id)).cloned().collect();
    let fasteners = document.fasteners.iter().filter(|fastener| fastener_set.contains(&fastener.id)).cloned().collect();
    (parts, fasteners)
}

fn centroid_2d_local(parts: &[Puzzle5dPart]) -> Option<(f64, f64)> {
    if parts.is_empty() {
        return None;
    }
    let (mut sum_x, mut sum_y) = (0.0, 0.0);
    for part in parts {
        sum_x += part.part_2d.x;
        sum_y += part.part_2d.y;
    }
    let count = parts.len() as f64;
    Some((sum_x / count, sum_y / count))
}

/// 🧮️ Resolves the 2D paste offset from `placement`: `Original` uses the (optional) position
/// override verbatim; every other anchor uses the target-minus-source centroid delta plus the
/// (optional) position override — mirrors semio_compose_rs's `__pasteCoordinateOffset`
/// (`semio_compose_rs/dev/algorithm/js/index.ts:358`).
pub fn paste_delta_2d(fragment_parts: &[Puzzle5dPart], target_parts: &[Puzzle5dPart], placement: &PastePlacement) -> (f64, f64) {
    let (offset_x, offset_y) = placement.position.map_or((0.0, 0.0), |position| (position[0], position[1]));
    if matches!(placement.anchor, PasteAnchor::Original) {
        return (offset_x, offset_y);
    }
    match (centroid_2d_local(fragment_parts), centroid_2d_local(target_parts)) {
        (Some(source), Some(target)) => (target.0 - source.0 + offset_x, target.1 - source.1 + offset_y),
        _ => (offset_x, offset_y),
    }
}

/// 🧮️ Materializes a copied fragment at 2D delta `delta` (applied verbatim to the 3D origin's x/y
/// too) — document-scoped fresh ids dodge collisions with the live document,
/// and fastener endpoints are remapped onto the fresh part ids.
pub fn paste_selection_local(document: &Puzzle5dDocument, fragment_parts: &[Puzzle5dPart], fragment_fasteners: &[Puzzle5dFastener], delta: (f64, f64)) -> (Vec<Puzzle5dPart>, Vec<Puzzle5dFastener>) {
    let mut fresh_ids = Puzzle5dFreshIds::from_document(document);
    let mut id_map: HashMap<String, String> = HashMap::new();
    let mut fresh_parts = Vec::with_capacity(fragment_parts.len());
    for part in fragment_parts {
        let fresh_id = fresh_ids.next_part();
        id_map.insert(part.id.clone(), fresh_id.clone());
        let mut next = part.clone();
        next.id = fresh_id;
        next.part_2d.x += delta.0;
        next.part_2d.y += delta.1;
        next.part_3d.origin[0] += delta.0;
        next.part_3d.origin[1] += delta.1;
        fresh_parts.push(next);
    }
    let mut fresh_fasteners = Vec::with_capacity(fragment_fasteners.len());
    for fastener in fragment_fasteners {
        let mut next = fastener.clone();
        next.id = fresh_ids.next_fastener();
        next.source = rewrite_grip_ref_local(&fastener.source, &id_map);
        next.target = rewrite_grip_ref_local(&fastener.target, &id_map);
        fresh_fasteners.push(next);
    }
    (fresh_parts, fresh_fasteners)
}
//#endregion 🔖️CopyPaste

//#region 🔖️ContextMenu
/// 🗂️ GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS: `duplicateSelection`/`selectSameKindSelection`/
/// `focusSelection` stay top-level verbs; the hide/lock toggles (bespoke rows — their label/icon flip
/// on selection state, so they can't resolve from a single static `ActionDefinition`) fold into a
/// `settings` group; `deleteSelection` (bespoke label carrying the selection-count phrase) stays the
/// trailing destructive row. `organize_context_menu`, run automatically at the `VcsArtifactApp::context_menu`
/// funnel, handles taxonomy ordering/separator placement — this function only needs to emit the rows.
/// 🕹️ The per-granularity ids one context-menu request carries. `surface.selection` is what the
/// document holds selected; `surface.hits` is the entity the pointer is actually over. A hit OUTSIDE
/// the selection is the menu's whole subject — a right-click on the grip of a selected part opens that
/// grip's menu, never the part's — while a hit inside the selection keeps the whole selection, so
/// "Delete (3 parts)" never silently narrows to the one row under the cursor.
#[derive(Default)]
pub struct Puzzle5dContextSelection {
    pub part_ids: Vec<String>,
    pub grip_ids: Vec<String>,
    pub fastener_ids: Vec<String>,
    /// 🎯️ The pointer's hit alone is the subject, so no selection read may widen it again.
    hit_subject: bool,
}

impl Puzzle5dContextSelection {
    /// 🪣️ The bucket one surface domain name belongs to — the board host's `node`/`handle`/`edge` and the world
    /// host's `object`/`vortex`/`attraction` pick domains for what this app calls a part, grip and fastener.
    fn bucket(&mut self, domain: &str) -> Option<&mut Vec<String>> {
        match domain {
            "node" | "object" | PUZZLE5D_GRANULARITY_PART => Some(&mut self.part_ids),
            "handle" | "vortex" | PUZZLE5D_GRANULARITY_GRIP => Some(&mut self.grip_ids),
            "edge" | "attraction" | PUZZLE5D_GRANULARITY_FASTENER => Some(&mut self.fastener_ids),
            _ => None,
        }
    }

    pub fn from_surface(surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>) -> Self {
        let mut out = Self::default();
        let Some(surface) = surface else { return out };
        for group in &surface.selection {
            let ids = group.ids.clone();
            if let Some(bucket) = out.bucket(group.domain.as_str()) {
                bucket.extend(ids);
            }
        }
        // 🎯️ Hosts list every target under the pointer, most specific first (a grip before its part), and only
        // that one is the menu's subject.
        let mut hits = Self::default();
        if let Some(hit) = surface.hits.iter().find(|hit| hits.bucket(hit.domain.as_str()).is_some()) {
            hits.bucket(hit.domain.as_str()).into_iter().for_each(|bucket| bucket.push(hit.id.clone()));
        }
        let hit_selected = |selected: &[String], hit: &[String]| hit.iter().all(|id| selected.contains(id));
        let inside = hit_selected(&out.part_ids, &hits.part_ids) && hit_selected(&out.grip_ids, &hits.grip_ids) && hit_selected(&out.fastener_ids, &hits.fastener_ids);
        if hits.is_empty() || inside {
            return out;
        }
        hits.hit_subject = true;
        hits
    }

    /// 🕹️ Fills in the granularities the CLIENT surface never sends — puzzle 3d's twin. `World3dHost` only puts
    /// its painted part ids into `ContextMenuSurfaceTarget.selection`, so a selected grip or fastener reaches a
    /// menu only through the framework-owned domain read. Per-granularity additive: whatever the surface DID
    /// supply keeps priority (a right-click on an unselected entity still targets what was clicked).
    pub fn fill_from_interaction(&mut self, interaction: &Puzzle5dInteractionSnapshot) {
        if self.hit_subject {
            return;
        }
        for (bucket, ids) in [(&mut self.part_ids, interaction.selected_part_ids()), (&mut self.grip_ids, interaction.selected_grip_ids()), (&mut self.fastener_ids, interaction.selected_fastener_ids())] {
            if bucket.is_empty() {
                bucket.extend(ids.iter().cloned());
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.part_ids.is_empty() && self.grip_ids.is_empty() && self.fastener_ids.is_empty()
    }
}

/// 🗂️ Every action id a context-menu row may carry that is NOT one of this app's declared actions:
/// the framework-reserved clipboard and interaction verbs, which live on `handle_action`'s reserved
/// branch and therefore never appear in the `AppActionRegistry` a `Menu` resolves against. Named here
/// so the context-menu law can hold every OTHER row to a declared, `Migrated` app action.
pub const PUZZLE5D_CONTEXT_MENU_RESERVED_ACTIONS: [&str; 4] = ["copy", "cut", "paste", "selectAll"];

/// 🗂️ GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS, one branch per selection granularity (puzzle 3d's
/// own shape): an EMPTY surface offers what can be done with no subject (select all, paste, add a
/// part), a PART selection the full selection vocabulary with the hide/lock toggles folded into a
/// `settings` group, a GRIP the brush-suggestions entry point, a FASTENER its own delete. The toggles
/// and the count-carrying delete are bespoke rows because their label/icon flip on state, which a
/// single static `ActionDefinition` cannot resolve; every other row resolves through `Menu::action`
/// against the registry, so a typo'd id is a construction-time panic rather than a dead row.
/// `organize_context_menu`, run at the `VcsArtifactApp::context_menu` funnel, handles taxonomy
/// ordering and separator placement — this function only emits rows.
pub fn puzzle5d_context_menu_items(
    envelope: &Puzzle5dScene,
    selection: &Puzzle5dContextSelection,
    labels: &Puzzle5dLabels,
    view_state: &semio_framework_plugin::ViewModel,
    registry: &semio_framework_plugin::AppActionRegistry,
) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::{selection_count_phrase, ContextMenuItemSpec, Menu, SelectionKind};
    let bespoke = |id: &str, label: String, icon: &str, action: &str, args: Option<Value>, destructive: bool| ContextMenuItemSpec {
        id: id.into(),
        label: Some(label),
        icon: Some(icon.into()),
        action: Some(action.into()),
        args: args.map(|value| semio_framework_pack_json::to_dsl_value(&value)),
        destructive: destructive.then_some(true),
        ..Default::default()
    };
    if selection.is_empty() {
        return Menu::of(registry, view_state)
            .item(bespoke("select-all", labels.select_all.into(), "box-select", "selectAll", None, false))
            .item(bespoke("paste", labels.paste.into(), "clipboard-paste", "paste", None, false))
            .action("openAddPartDialog")
            .build();
    }
    if !selection.part_ids.is_empty() {
        let part_ids = &selection.part_ids;
        let selected: Vec<&Puzzle5dPart> = envelope.document.parts.iter().filter(|part| part_ids.contains(&part.id)).collect();
        let all_hidden = !selected.is_empty() && selected.iter().all(|part| part.part_2d.hidden.unwrap_or(false));
        let all_locked = !selected.is_empty() && selected.iter().all(|part| part.part_2d.locked.unwrap_or(false));
        let phrase = selection_count_phrase(view_state.locale, &[(part_ids.len(), SelectionKind::Part)]).unwrap_or_default();
        return Menu::of(registry, view_state)
            .action("duplicateSelection")
            .item(bespoke("copy", labels.copy.into(), "copy", "copy", None, false))
            .item(bespoke("cut", labels.cut.into(), "scissors", "cut", None, false))
            .action("selectSameKindSelection")
            .action("focusSelection")
            .group("settings", |m| {
                m.item(bespoke("hide-show", if all_hidden { labels.show.into() } else { labels.hide.into() }, if all_hidden { "eye" } else { "eye-off" }, "setSelectionFlag", Some(semio_framework_pack_json::json!({ "flag": "hidden", "value": !all_hidden })), false))
                    .item(bespoke("lock-unlock", if all_locked { labels.unlock.into() } else { labels.lock.into() }, if all_locked { "lock-open" } else { "lock" }, "setSelectionFlag", Some(semio_framework_pack_json::json!({ "flag": "locked", "value": !all_locked })), false))
            })
            .item(bespoke("delete", format!("{} ({phrase})", labels.delete.as_str()), "trash", "deleteSelection", None, true))
            .build();
    }
    if !selection.grip_ids.is_empty() {
        let mut menu = Menu::of(registry, view_state);
        // 🎣️ Only for EXACTLY one grip: the suggestion search points at a single grip. The row carries the
        // world host's `openVortexSuggestions` verb, which the host turns into a live submenu: it starts the
        // search the moment the menu opens and lists the free parts as they are found.
        if let [only] = selection.grip_ids.as_slice() {
            menu = menu.item(bespoke("suggest", labels.suggest_parts.into(), "sparkles", "openVortexSuggestions", Some(semio_framework_pack_json::json!({ "fullId": only.as_str() })), false));
        }
        return menu.action("focusSelection").item(bespoke("delete", labels.delete.into(), "trash", "deleteSelection", None, true)).build();
    }
    // 🔗️ A fastener row carries its own id. `retargetFastener` is deliberately NOT offered here: it
    // needs a REPLACEMENT grip a context menu cannot name, and dispatching it with an id alone is an
    // early return — a visibly dead row. Retargeting lives in the inspector, which has both grips.
    let Some(id) = selection.fastener_ids.first() else { return Vec::new() };
    Menu::of(registry, view_state).item(bespoke("delete", labels.delete.into(), "trash", "deleteFastener", Some(semio_framework_pack_json::json!({ "id": id.as_str() })), true)).build()
}
//#endregion 🔖️ContextMenu

//#region 🔖️Puzzle5dCommand
/// 🎯️ B1: `Puzzle5dPlayApp::Command` — the SOLE dispatch surface, one variant per declared
/// action (mirrors every `.mutation(...)`/`.view_action(...)` id `create_puzzle5d_app` registers,
/// plus the framework-injected `SET_ACTIVE_UTILITY_ACTION_ID`). Each variant carries `window_id` (was
/// host-pushed `view_state.window_id`) plus `args` (the action's original `{...}` JSON payload,
/// unchanged) — `handle` reconstructs the exact `(action, args, window_id)` triple every
/// `🎮️commands/*` arm expects, so each arm's internal `args.get("field")` extraction stays
/// byte-for-byte identical to the pre-migration implementation.
///
/// ⚠️ `OpBinary` is a plain JSON-bytes bridge (NOT `#[derive(dsl::DslOps)]`, and NOT the framework's
/// `app_commands!` macro): a generic `args: Value` field is not representable in the DSL grammar those
/// target, so adopting them would silently rewrite this app's wire format. Keep this macro's variant
/// list, its order and its action-id literals byte-for-byte stable.
macro_rules! puzzle5d_command_variants {
    ($($Variant:ident = $id:tt),* $(,)?) => {
        #[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
        pub enum Puzzle5dCommand {
            $($Variant { window_id: Option<String>, args: Option<semio_framework_pack_json::Value> }),*
        }

        impl Puzzle5dCommand {
            /// 🏷️ The action id this variant was declared under — used both for `command_id()`
            /// (command-log labeling / registry kind-discipline) and to reconstruct the exact
            /// `action: &str` `handle_action_impl` dispatches on.
            fn action_id(&self) -> &'static str {
                match self {
                    $(Puzzle5dCommand::$Variant { .. } => $id),*
                }
            }

            fn window_id(&self) -> Option<&str> {
                match self {
                    $(Puzzle5dCommand::$Variant { window_id, .. } => window_id.as_deref()),*
                }
            }

            fn args(&self) -> Option<&semio_framework_pack_json::Value> {
                match self {
                    $(Puzzle5dCommand::$Variant { args, .. } => args.as_ref()),*
                }
            }

            fn try_from_action(action: &str, args: Option<semio_framework_pack_json::Value>, window_id: Option<String>) -> Option<Self> {
                match action {
                    $($id => Some(Puzzle5dCommand::$Variant { window_id, args })),*,
                    _ => None,
                }
            }

            #[cfg(test)]
            fn from_action(action: &str, args: Option<semio_framework_pack_json::Value>, window_id: Option<String>) -> Self {
                Self::try_from_action(action, args, window_id)
                    .unwrap_or_else(|| panic!("unknown puzzle5d action id in test: {action}"))
            }

            /// 🪶️ Hand-written JSON bridge (see this macro's own doc comment on why `OpBinary` here
            /// is a plain JSON-bytes bridge, not a derive): reproduces serde's default externally
            /// tagged struct-variant shape (`{"VariantName": {"window_id": ..., "args": ...}}`) so
            /// `encode_op`/`decode_op` stay byte-for-byte compatible with the pre-migration wire.
            fn to_json(&self) -> semio_framework_pack_json::Value {
                match self {
                    $(Puzzle5dCommand::$Variant { window_id, args } => semio_framework_pack_json::object([(
                        stringify!($Variant).to_string(),
                        semio_framework_pack_json::object([("window_id".to_string(), semio_framework_pack_json::Value::from(window_id.clone())), ("args".to_string(), args.clone().unwrap_or(semio_framework_pack_json::Value::Null))]),
                    )])),*
                }
            }

            fn from_json(value: &semio_framework_pack_json::Value) -> Option<Self> {
                let entries = value.as_object()?;
                if entries.len() != 1 {
                    return None;
                }
                let (tag, payload) = entries.iter().next()?;
                let window_id = payload.get("window_id").and_then(semio_framework_pack_json::Value::as_str).map(str::to_string);
                let args = payload.get("args").cloned().filter(|value| !value.is_null());
                match tag {
                    $(stringify!($Variant) => Some(Puzzle5dCommand::$Variant { window_id, args }),)*
                    _ => None,
                }
            }
        }
    };
}

puzzle5d_command_variants! {
    ExportSnapshot = "exportSnapshot",
    ImportSnapshot = "importSnapshot",
    OpenImportSnapshot = "openImportSnapshot",
    OpenAddPartDialog = "openAddPartDialog",
    SetActiveExample = "setActiveExample",
    AddNode = "addNode",
    AddPartKind = "addPartKind",
    AddBrushPart = "addBrushPart",
    DeleteSelection = "deleteSelection",
    DuplicateSelection = "duplicateSelection",
    SetSelectionFlag = "setSelectionFlag",
    SetSelectionHidden = "setSelectionHidden",
    SetSelectionLocked = "setSelectionLocked",
    FocusSelection = "focusSelection",
    EngagementSubmit = "engagementSubmit",
    EngagementRepeatLast = "engagementRepeatLast",
    SetFillCount = "setFillCount",
    PatchPart = "patchPart",
    PatchGrip = "patchGrip",
    PatchFastener = "patchFastener",
    CreateFastener = "createFastener",
    DeleteFastener = "deleteFastener",
    RetargetFastener = "retargetFastener",
    EditFastener = "editFastener",
    ProximityConnect = "proximityConnect",
    TranslateSelection = "translateSelection",
    RotateSelection = "rotateSelection",
    ScaleSelection = "scaleSelection",
    WorldRelocate = "worldRelocate",
    ApplyBoardEvents = "applyBoardEvents",
    SetCamera = "setCamera",
    SetCamera2d = "setCamera2d",
    SetCamera3d = "setCamera3d",
    SelectSameKindSelection = "selectSameKindSelection",
    ToggleSun = "toggleSun",
    SetSunAzimuth = "setSunAzimuth",
    SetSunElevation = "setSunElevation",
    SetSunIntensity = "setSunIntensity",
    EngagementInput = "engagementInput",
    EngagementAbort = "engagementAbort",
    EngagementControlSelect = "engagementControlSelect",
    CycleBrushCandidate = "cycleBrushCandidate",
    CycleBrushCandidateBack = "cycleBrushCandidateBack",
    TargetBrushSuggestions = "targetBrushSuggestions",
    OpenVortexSuggestions = "openVortexSuggestions",
    CloseVortexSuggestions = "closeVortexSuggestions",
    HoverSuggestion = "hoverSuggestion",
    AcceptSuggestion = "acceptSuggestion",
    RegisterBrushMesh = "registerBrushMesh",
    SetBrushPlacementContactTolerance = "setBrushPlacementContactTolerance",
    SetProximityRadius = "setProximityRadius",
    SetChunkSize = "setChunkSize",
    SetPartKindWeight = "setPartKindWeight",
    SetGripKindWeight = "setGripKindWeight",
    SetLodMode = "setLodMode",
    SetSuggestionOffset = "setSuggestionOffset",
    SetGridSnapEnabled = "setGridSnapEnabled",
    SetGridFactor = "setGridFactor",
    SetGridVisible = "setGridVisible",
    SetGridSpacing = "setGridSpacing",
    SetProjection = "setProjection",
    SetProjectionParam = "setProjectionParam",
    SetGripShow = "setGripShow",
    SetGripDirection = "setGripDirection",
    SetSelectableKind = "setSelectableKind",
    SetLodAutomatic = "setLodAutomatic",
    SetLodDepthVariable = "setLodDepthVariable",
    SetLodManual = "setLodManual",
    SetTransformGumballFlag = "setTransformGumballFlag",
    WorldPointerDown = "worldPointerDown",
    CanvasPointerDown = "canvasPointerDown",
    AddTargetVolume = "addTargetVolume",
    DeleteTargetVolume = "deleteTargetVolume",
    RelocateTargetVolume = "relocateTargetVolume",
    SetTargetVolumeFlag = "setTargetVolumeFlag",
    SetTargetVolumeHidden = "setTargetVolumeHidden",
    SetTargetVolumeLocked = "setTargetVolumeLocked",
    SetVoxelDims = "setVoxelDims",
}

/// 🧭️ Every generated tool id this app's wire may address: the retained command catalog plus the two
/// framework-injected host-configuration verbs. They are not app-owned retained tools, but
/// `validate_tool_job_rows` only admits `Puzzle5dHostConfigurationProofs`' generic bounded proofs for a
/// generated id — without them `dispatch_action` fails closed with `interactive-job.missing-factory` before
/// the host-configuration branch is ever consulted, and the fill TOOL cannot be armed.
const PUZZLE5D_TOOL_JOB_IDS: [&str; PUZZLE5D_RETAINED_TOOL_IDS.len() + 2] = {
    let mut ids = [""; PUZZLE5D_RETAINED_TOOL_IDS.len() + 2];
    let mut index = 0;
    while index < PUZZLE5D_RETAINED_TOOL_IDS.len() {
        ids[index] = PUZZLE5D_RETAINED_TOOL_IDS[index];
        index += 1;
    }
    ids[index] = semio_framework_plugin::SET_ACTIVE_TOOL_ACTION_ID;
    ids[index + 1] = semio_framework_plugin::SET_ACTIVE_UTILITY_ACTION_ID;
    ids
};

impl protocol::OpBinary for Puzzle5dCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = &PUZZLE5D_TOOL_JOB_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_string(&self.to_json()).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?;
        let value = parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error.into_value_error())))?;
        Self::from_json(&value).ok_or_else(|| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unrecognized Puzzle5dCommand tag"))))
    }
}
//#endregion 🔖️Puzzle5dCommand

//#region 🔖️ActionContext
/// 🎬️ Everything one `🎮️commands/*` arm may read or write. The prologue/epilogue around the dispatch
/// match (scene materialization, delta computation, host-effect emission, config snapshotting) stays
/// in [`Puzzle5dPlayApp::handle_action_impl`]; an arm only mutates this bundle.
pub struct Puzzle5dActionCtx<'a> {
    pub scene: &'a mut Puzzle5dScene,
    /// 📸️ The committed play snapshot the scene was materialized from (authored kind catalogs included).
    pub snapshot: &'a Puzzle5dPlaySnapshot,
    /// 🧠️ The document instance's retained operation owner (the brush suggestions link), when the route bound one.
    pub instance_owner: Option<&'a semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
    /// 🪟️ The window this action targets (already defaulted to the 3D window).
    pub window_id: &'a str,
    /// 🧭️ The registered kind of the exact target window.
    pub window_kind: &'a str,
    /// 🕹️ Read-only view of the framework-owned `vortex` interaction domain (ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — retained selection-acting verbs read
    /// `.selected_part_ids()?`/etc. here instead of the deleted `Puzzle5dConfig` selection fields.
    pub selection: &'a protocol::DomainSelection,
    /// 🌍️ The host's projected locale×terminology axes — the only source a notice may resolve its
    /// sentence from (`Puzzle5dActionCtx::notice`).
    pub view_state: Option<&'a semio_framework_plugin::ViewModel>,
    /// ⏯️ The instance's tool run as of command admission (`ArtifactOwnedToolJobContext::tool_run`) — the
    /// identity Escape's `toolRunAbort` is bound to while a fill run is live.
    pub tool_run: Option<&'a semio_framework_plugin::ToolRunView>,
    /// 🧯️ At most ONE bounded, localized transient notice this arm wants shown, plus whatever host
    /// effects it raised. An arm that REFUSES pushes a notice here instead of completing silently.
    pub effects: Vec<Effect>,
    /// 🕹️ App-initiated selection writes this arm wants applied once its own mutations land — the
    /// single sanctioned way a reducer selects what it just created, or widens/clears the selection
    /// (`semio_framework_plugin::InteractionWrite`, applied by `VcsArtifactApp` through the same state
    /// machine the reserved `interactionSelect` verb uses). Empty for every arm that does not move it.
    pub interaction_writes: Vec<InteractionWrite>,
    /// 🛑️ Set by an arm that must skip the whole epilogue (delta, effects, config snapshot) — the
    /// direct replacement for the pre-migration `return Emit::default()` early exits.
    pub abort: bool,
    /// 🌱️ The admission's authoring seed every tool transaction this action commits is minted from; empty for
    /// a reduction no admission seeded, whose transaction then stays unstamped.
    pub authoring_seed: &'a str,
    /// 🛠️ The parametric mutations the transform tool yielded for this action, published ahead of the scene delta.
    pub artifact_mutations: Vec<Puzzle5dMutation>,
    /// 🛠️ The tool transaction this action committed — stamped on the ONE edit it publishes.
    pub transaction: Option<protocol::TransactionRef>,
}

impl<'a> Puzzle5dActionCtx<'a> {
    /// 🧲️ One gumball pose delta (`translateSelection`/`rotateSelection`/`scaleSelection`) as ONE tool
    /// transaction over the gesture's own part ids (else the selected parts) and the selected target volumes.
    pub fn commit_gumball(&mut self, verb: &str, args: Option<&Value>) {
        let targets = [mesh_selection_ids(args, &self.selected_part_ids()), self.selected_ids(PUZZLE5D_GRANULARITY_TARGET_VOLUME)].concat();
        if let Some(record) = semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dSelectionRecord::from_gumball(verb, args, targets) {
            self.commit_selection(verb, vec![crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::Puzzle5dSelectionRecord::world(record)]);
        }
    }

    /// 🛠️ Commits `records` through the transform tool machine as ONE tool transaction of this action — the
    /// parametric selection leaves plus the fasteners their drops land, yielded as `verb`. Nothing named, or
    /// nothing named that exists, is the `nothing_selected` refusal; a request whose every target is locked is
    /// the `selection_locked` one; a request with a movable target is yielded whole, and its leaf reports the
    /// locked rest as `mutation.partial`. A motionless request leaves zero trace.
    pub fn commit_selection(&mut self, verb: &str, records: Vec<crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::Puzzle5dSelectionRecord>) {
        let base = self.snapshot.typed_arc();
        if records.iter().all(|record| !record.names_any(&base)) {
            self.refuse_without_selection(&[]);
            return;
        }
        if records.iter().any(|record| record.refused_as_locked(&base)) && !records.iter().any(|record| record.applies_to(&base)) {
            self.notice(|labels| labels.selection_locked.as_str());
            self.abort = true;
            return;
        }
        let request = crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::TransformToolRequest { base, records };
        if let Some((transaction, mutations)) = crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::puzzle5d_transform_tool_commit(verb, self.authoring_seed, request) {
            self.transaction = (!self.authoring_seed.is_empty()).then_some(transaction);
            self.artifact_mutations.extend(mutations);
        }
    }

    /// 🔗️ Runs `apply` on this instance's brush suggestions link; `None` without an instance owner or while the
    /// owner is busy.
    pub fn brush_suggestions<R>(&self, apply: impl FnOnce(&mut semio_s_artifact_puzzle_3d_precompute_brush::BrushSuggestionsLink) -> R) -> Option<R> {
        self.instance_owner?.with_mut::<Puzzle3dInstanceOperationOwner, _>(|owner| Ok(apply(&mut owner.brush_suggestions))).ok()
    }

    pub fn selected_ids(&self, granularity_id: &str) -> Vec<String> {
        if self.selection.granularity == granularity_id {
            self.selection.ids.clone()
        } else {
            Vec::new()
        }
    }
    pub fn selected_part_ids(&self) -> Vec<String> {
        self.selected_ids(PUZZLE5D_GRANULARITY_PART)
    }
    pub fn selected_grip_ids(&self) -> Vec<String> {
        self.selected_ids(PUZZLE5D_GRANULARITY_GRIP)
    }
    pub fn selected_fastener_ids(&self) -> Vec<String> {
        self.selected_ids(PUZZLE5D_GRANULARITY_FASTENER)
    }

    /// 🕹️ Replaces this app's whole `vortex` selection with `ids` at `granularity` once the action's own
    /// mutations have landed — "select the thing I just created/widened to".
    pub fn replace_selection(&mut self, granularity: &str, ids: impl IntoIterator<Item = String>) {
        let write = InteractionWrite::replace(PUZZLE5D_INTERACTION_DOMAIN, granularity, ids);
        if !write.targets.is_empty() {
            self.interaction_writes.push(write);
        }
    }

    /// 🧹️ Empties this app's whole `vortex` selection through the same sanctioned reducer channel
    /// [`Self::replace_selection`] uses. It is a `Subtractive` write naming exactly what is selected right
    /// now, NOT an empty `Replace`: the framework's state machine returns the current selection unchanged
    /// when a write names no target at all (`protocol::next_selection`), so an empty `Replace` is a silent
    /// no-op. Nothing selected means nothing to clear.
    pub fn clear_selection(&mut self) {
        let targets: Vec<InteractionTarget> = [
            (PUZZLE5D_GRANULARITY_PART, self.selected_part_ids()),
            (PUZZLE5D_GRANULARITY_GRIP, self.selected_grip_ids()),
            (PUZZLE5D_GRANULARITY_FASTENER, self.selected_fastener_ids()),
        ]
        .into_iter()
        .flat_map(|(granularity, ids)| ids.into_iter().map(move |id| InteractionTarget { granularity: granularity.to_string(), id }))
        .collect();
        if targets.is_empty() {
            return;
        }
        self.interaction_writes.push(InteractionWrite { domain: PUZZLE5D_INTERACTION_DOMAIN.into(), targets, merge: MergeMode::Subtractive });
    }

    /// 🧯️ Surfaces ONE bounded, localized notice through the shell's transient-notice channel
    /// (`Effect::Notify` → `ShellHost`'s `showTransientNotice`) — how an arm whose work was REFUSED tells
    /// the user, instead of the `Ok(Fixture(_))`-only silence every 5d placement arm fell through with. At
    /// most one notice per action, so the effect list stays fixed-width. An unauthored locale×terminology
    /// axis carries this app's own `ui.localization.unsupported` code rather than an English sentence.
    pub fn notice(&mut self, message: impl Fn(&Puzzle5dLabels) -> &'static str) {
        if self.effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })) {
            return;
        }
        let text = self.view_state.and_then(puzzle5d_labels).map_or_else(|| PUZZLE5D_LOCALIZATION_UNSUPPORTED.to_string(), |labels| message(labels).to_string());
        self.effects.push(Effect::Notify { message: text });
    }

    /// 🎯️ Refuses a selection-scoped command that has nothing to act on: ONE localized notice, then the
    /// same `abort` the early `return` used, so neither an empty document edit nor an empty interaction
    /// write is emitted. Answers whether it refused.
    pub fn refuse_without_selection(&mut self, ids: &[String]) -> bool {
        if !ids.is_empty() {
            return false;
        }
        self.notice(|labels| labels.nothing_selected.as_str());
        self.abort = true;
        true
    }

    /// 🔒️ Refuses a gesture whose every addressed part carries `part_2d.locked` — a locked part must not
    /// move, and a silent no-op is indistinguishable from a dead control.
    pub fn refuse_when_locked(&mut self, ids: &[String]) -> bool {
        if ids.is_empty() || ids.iter().any(|id| self.scene.document.parts.iter().any(|part| &part.id == id && !part.part_2d.locked.unwrap_or(false))) {
            return false;
        }
        self.notice(|labels| labels.selection_locked.as_str());
        self.abort = true;
        true
    }
}

/// 🌍️ The code an unauthored locale×terminology axis carries instead of an English sentence — this UI has
/// no default language, so a hard-coded fallback would be a silent localization regression.
pub const PUZZLE5D_LOCALIZATION_UNSUPPORTED: &str = "ui.localization.unsupported";

/// 🧯️ ONE bounded, localized notice as a terminal `Emit` — how a retained work whose gesture produced
/// nothing tells the user WHY, instead of the bare `Emit::default()` every refusal path used to complete
/// with. The scope is [`UiDirtyScope::None`]: a work that refused published nothing, so there is nothing to
/// repaint — the notice itself travels on the effect lane, which the host applies before it consults scope.
pub fn puzzle5d_notice_emit(view_state: Option<&semio_framework_plugin::ViewModel>, message: impl Fn(&Puzzle5dLabels) -> &'static str) -> Emit<Puzzle5dMutation, Puzzle5dConfigMutation> {
    let text = view_state.and_then(puzzle5d_labels).map_or_else(|| PUZZLE5D_LOCALIZATION_UNSUPPORTED.to_string(), |labels| message(labels).to_string());
    Emit { effects: vec![Effect::Notify { message: text }], ui_scope: UiDirtyScope::None, ..Default::default() }
}

/// 📋️ The clipboard fragment of the selected parts and fasteners (the fasteners among them only).
pub fn puzzle5d_copy_fragment(snapshot: &Puzzle5dPlaySnapshot, part_ids: &[String], fastener_ids: &[String]) -> Result<ClipboardFragment, ClipboardError> {
    let document: Puzzle5dDocument = puzzle5d_document_from_snapshot(snapshot.typed()).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
    let (parts, fasteners) = copy_selection_local(&document, part_ids, fastener_ids);
    if parts.is_empty() {
        return Err(ClipboardError::EmptySelection);
    }
    let fragment_value = semio_framework_pack_json::json!({ "schema": PUZZLE5D_SCHEMA, "parts": parts, "fasteners": fasteners });
    Ok(ClipboardFragment {
        schema: PUZZLE5D_SCHEMA.to_string(),
        media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Design },
        dsl_text: semio_framework_pack_json::to_string_pretty(&fragment_value),
        pack_bytes: None,
        source_app: PUZZLE5D_PLAY_APP_ID.to_string(),
        label: format!("{} part(s)", parts.len()),
    })
}

/// ✂️ The document removal of the selected parts and fasteners.
pub fn puzzle5d_cut_operations(snapshot: &Puzzle5dPlaySnapshot, part_ids: &[String], fastener_ids: &[String]) -> Vec<Puzzle5dMutation> {
    let before = puzzle5d_editor_projection(snapshot);
    let Ok(document) = <Puzzle5dDocument as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&before)) else {
        return Vec::new();
    };
    let (parts, fasteners) = copy_selection_local(&document, part_ids, fastener_ids);
    if parts.is_empty() {
        return Vec::new();
    }
    // 🔒️ A LOCKED part is copied but never removed — a lock exists precisely to refuse a destructive
    // gesture — and the fasteners of a surviving part survive with it, or the document is left half-cut.
    // `Puzzle5dClipboardJob` applies the identical rule on the retained route; the two must never disagree.
    let locked: HashSet<&str> = parts.iter().filter(|part| part.part_2d.locked.unwrap_or(false)).map(|part| part.id.as_str()).collect();
    fasteners
        .iter()
        .filter(|fastener| !locked.contains(owning_part_id_local(&fastener.source)) && !locked.contains(owning_part_id_local(&fastener.target)))
        .map(|fastener| crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(fastener.id.clone()))
        .chain(parts.iter().filter(|part| !locked.contains(part.id.as_str())).map(|part| crate::standards::v1::subsets::any::schema::mutations::delete_part(part.id.clone())))
        .collect()
}

/// 🕹️ `copy_fragment`/`cut_operations` have no `Puzzle5dActionCtx` (only `doc`/`cfg`/`interaction`
/// per `ArtifactApp`'s signature) — a free-function twin of `Puzzle5dActionCtx::selected_part_ids`/
/// `selected_fastener_ids` for those two call sites.
pub fn puzzle5d_interaction_part_and_fastener_ids(interaction: &InteractionView<'_>) -> (Vec<String>, Vec<String>) {
    let selection = interaction.selection(PUZZLE5D_INTERACTION_DOMAIN);
    match selection.granularity.as_str() {
        PUZZLE5D_GRANULARITY_PART => (selection.ids.clone(), Vec::new()),
        PUZZLE5D_GRANULARITY_FASTENER => (Vec::new(), selection.ids.clone()),
        _ => (Vec::new(), Vec::new()),
    }
}
/// 🏷️ Admits dynamic puzzle labels into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref().to_string()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle5d label admission failed"))
}
//#endregion 🔖️ActionContext

pub const PUZZLE5D_RETAINED_TOOL_IDS: &[&str] = &[
    "addTargetVolume",
    "deleteTargetVolume",
    "relocateTargetVolume",
    "setTargetVolumeFlag",
    "setTargetVolumeHidden",
    "setTargetVolumeLocked",
    "setVoxelDims",
    "canvasPointerDown",
    "cycleBrushCandidate",
    "cycleBrushCandidateBack",
    "worldPointerDown",
    "deleteSelection",
    "duplicateSelection",
    "engagementAbort",
    "engagementControlSelect",
    "engagementInput",
    "engagementRepeatLast",
    "engagementSubmit",
    "exportSnapshot",
    "importSnapshot",
    "openAddPartDialog",
    "openImportSnapshot",
    "selectSameKindSelection",
    "setFillCount",
    "setSelectionFlag",
    "setSelectionHidden",
    "setSelectionLocked",
    "targetBrushSuggestions",
    "openVortexSuggestions",
    "closeVortexSuggestions",
    "hoverSuggestion",
    "acceptSuggestion",
    "focusSelection",
    "addBrushPart",
    "addNode",
    "addPartKind",
    "applyBoardEvents",
    "createFastener",
    "deleteFastener",
    "editFastener",
    "patchFastener",
    "patchGrip",
    "patchPart",
    "proximityConnect",
    "registerBrushMesh",
    "retargetFastener",
    "rotateSelection",
    "scaleSelection",
    "setActiveExample",
    "translateSelection",
    "worldRelocate",
    "setCamera",
    "setCamera2d",
    "setCamera3d",
    "setGridFactor",
    "setGridSnapEnabled",
    "setLodMode",
    "setSuggestionOffset",
    "toggleSun",
    "setSunAzimuth",
    "setSunElevation",
    "setSunIntensity",
    "setBrushPlacementContactTolerance",
    "setProximityRadius",
    "setChunkSize",
    "setPartKindWeight",
    "setGripKindWeight",
    "setGridVisible",
    "setGridSpacing",
    "setProjection",
    "setProjectionParam",
    "setGripShow",
    "setGripDirection",
    "setSelectableKind",
    "setLodAutomatic",
    "setLodDepthVariable",
    "setLodManual",
    "setTransformGumballFlag",
];

//#region 🧩️EditorContract
/// 🧩️ The puzzle5d editor contract: every editor whose lanes are puzzle5d's play snapshot, mutation union, config,
/// presence and command vocabulary. The action dispatch, the retained works and the tool run builders are written
/// against it, so they sit below the one concrete editor that implements it.
pub trait Puzzle5dEditor: semio_framework_plugin::ArtifactEditor<Snapshot = Puzzle5dPlaySnapshot, Mutation = Puzzle5dMutation, Config = crate::editor::puzzle5d::config::Puzzle5dConfig, ConfigMutation = Puzzle5dConfigMutation, Draft = semio_framework_plugin::NoDraft, DraftMutation = semio_framework_plugin::NoDraftMutation, Presence = crate::editor::puzzle5d::presence::Puzzle5dPresence, PresenceMutation = crate::editor::puzzle5d::presence::Puzzle5dPresenceMutation, Transient = semio_framework_plugin::NoTransient, TransientMutation = semio_framework_plugin::NoTransientMutation, Command = Puzzle5dCommand> {}

impl<A> Puzzle5dEditor for A where A: semio_framework_plugin::ArtifactEditor<Snapshot = Puzzle5dPlaySnapshot, Mutation = Puzzle5dMutation, Config = crate::editor::puzzle5d::config::Puzzle5dConfig, ConfigMutation = Puzzle5dConfigMutation, Draft = semio_framework_plugin::NoDraft, DraftMutation = semio_framework_plugin::NoDraftMutation, Presence = crate::editor::puzzle5d::presence::Puzzle5dPresence, PresenceMutation = crate::editor::puzzle5d::presence::Puzzle5dPresenceMutation, Transient = semio_framework_plugin::NoTransient, TransientMutation = semio_framework_plugin::NoTransientMutation, Command = Puzzle5dCommand> {}
//#endregion 🧩️EditorContract
