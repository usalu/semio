//! 🧰️ Retained tool ids and scene builders: the retained, host configuration, job and generic tool ids and the scene builders. See [`super`].

use super::*;

//#region 🧵️RetainedCommands
/// 🧵️ The 2D editor actions that carry an exact app-owned tool proof. Only these may declare
/// [`InteractiveJobClassification::Migrated`] — UI dispatch resolves a controller/owner/factory/tool
/// /schema proof for every migrated verb, and one without a registered factory is refused outright.
pub const PUZZLE2D_RETAINED_TOOL_IDS: &[&str] = &[
    "setActiveExample",
    "forceLayout",
    "addNode",
    "applyBoardEvents",
    "reorganize",
    "closeHandleSuggestions",
    "acceptSuggestion",
    "cycleBrushCandidate",
    "cycleBrushCandidateBack",
    "openHandleSuggestions",
    "hoverSuggestion",
    "targetBrushSuggestions",
    "deleteSelection",
    "duplicateSelection",
    "engagementAbort",
    "engagementControlSelect",
    "engagementInput",
    "engagementSubmit",
    "focusSelection",
    "lodScaleJson",
    "patchInspectorNodes",
    "redrawHandles",
    "selectSameKind",
    "setBrushKindWeights",
    "setBrushNodeSize",
    "setCamera",
    "setFillCount",
    "setGridFactor",
    "setGridSnapEnabled",
    "setGridVisible",
    "setSelectableKind",
    "setBrushPlacementContactTolerance",
    "setBrushPlacementOverlapBudget",
    "engagementRepeatLast",
    "openAddNodeDialog",
    "setLodModeForPane",
    "translateSelection",
    "rotateSelection",
    "scaleSelection",
    "exportSnapshot",
    "importSnapshot",
    "openImportSnapshot",
    "setSelectionFlag",
    "setSelectionHidden",
    "setSelectionLocked",
    "setSuggestionOffset",
    "setTransformGumballFlag",
    "addTargetRegion",
    "deleteTargetRegion",
    "relocateTargetRegion",
    "setTargetRegionFlag",
    "setTargetRegionHidden",
    "setTargetRegionLocked",
    "setAreaBrushSize",
    "createEdge",
    "deleteEdge",
    "proximityConnect",
    "setProximityRadius",
];

/// 🧭️ The two framework-injected host-configuration verbs. They are `InteractiveJobClassification::Migrated`
/// in the manifest and need a generated tool id — without which `validate_tool_job_rows` refuses
/// [`Puzzle2dHostConfigurationProofs`]' generic bounded proofs and `dispatch_action` fails closed.
/// They carry NO app-owned factory, so they stay out of [`PUZZLE2D_RETAINED_TOOL_IDS`], which is what
/// keys [`Puzzle2dRetainedCommandJobFactory`] and what the retained-jobs snapshot pins.
pub const PUZZLE2D_HOST_CONFIGURATION_TOOL_IDS: &[&str] = &["setActiveTool", "setActiveUtility"];

/// 🗂️ Every generated tool id this app declares: the app-owned retained verbs plus the two
/// host-configuration verbs. `OpBinary::TOOL_JOB_IDS` — the list the framework joins against the
/// migrated manifest rows.
pub const PUZZLE2D_TOOL_JOB_IDS: [&str; PUZZLE2D_RETAINED_TOOL_IDS.len() + PUZZLE2D_HOST_CONFIGURATION_TOOL_IDS.len()] = {
    let mut ids = [""; PUZZLE2D_RETAINED_TOOL_IDS.len() + PUZZLE2D_HOST_CONFIGURATION_TOOL_IDS.len()];
    let mut index = 0;
    while index < PUZZLE2D_RETAINED_TOOL_IDS.len() {
        ids[index] = PUZZLE2D_RETAINED_TOOL_IDS[index];
        index += 1;
    }
    let mut host = 0;
    while host < PUZZLE2D_HOST_CONFIGURATION_TOOL_IDS.len() {
        ids[index + host] = PUZZLE2D_HOST_CONFIGURATION_TOOL_IDS[host];
        host += 1;
    }
    ids
};

/// 🎬️ The retained verbs whose whole completion is [`puzzle2d_dispatch_emit`] — one `🎮️commands/*`
/// arm run over a rebuilt scene/board host, then the same document delta and config snapshot
/// `handle` derives. Everything outside this list carries a bespoke `Work` (
/// `forceLayout`/`reorganize`) or an isolated reducer (`addNode`, `setActiveExample`).
pub const PUZZLE2D_GENERIC_TOOL_IDS: &[&str] = &[
    "closeHandleSuggestions",
    "acceptSuggestion",
    "cycleBrushCandidate",
    "cycleBrushCandidateBack",
    "openHandleSuggestions",
    "hoverSuggestion",
    "targetBrushSuggestions",
    "deleteSelection",
    "duplicateSelection",
    "engagementAbort",
    "engagementControlSelect",
    "engagementInput",
    "engagementSubmit",
    "focusSelection",
    "patchInspectorNodes",
    "selectSameKind",
    "setBrushKindWeights",
    "setBrushNodeSize",
    "setCamera",
    "setFillCount",
    "setGridFactor",
    "setGridSnapEnabled",
    "setGridVisible",
    "setSelectableKind",
    "setBrushPlacementContactTolerance",
    "setBrushPlacementOverlapBudget",
    "engagementRepeatLast",
    "openAddNodeDialog",
    "setLodModeForPane",
    "setSelectionFlag",
    "setSelectionHidden",
    "setSelectionLocked",
    "setSuggestionOffset",
    "setTransformGumballFlag",
    "addTargetRegion",
    "deleteTargetRegion",
    "relocateTargetRegion",
    "setTargetRegionFlag",
    "setTargetRegionHidden",
    "setTargetRegionLocked",
    "setAreaBrushSize",
    "translateSelection",
    "rotateSelection",
    "scaleSelection",
    "importSnapshot",
    "openImportSnapshot",
    "createEdge",
    "deleteEdge",
    "proximityConnect",
    "setProximityRadius",
];

pub fn puzzle2d_scene_for(snapshot: Value, runtime: Puzzle2dPlayRuntime, active_utility: &str) -> Puzzle2dScene {
    Puzzle2dScene { board_snapshot: snapshot, runtime, active_utility: active_utility.into(), interaction: Puzzle2dInteractionSnapshot::default() }
}

pub fn puzzle2d_scene_with(snapshot: Value, runtime: Puzzle2dPlayRuntime, active_utility: &str, interaction: Puzzle2dInteractionSnapshot) -> Puzzle2dScene {
    Puzzle2dScene { board_snapshot: snapshot, runtime, active_utility: active_utility.into(), interaction }
}
//#endregion 🧵️RetainedCommands
