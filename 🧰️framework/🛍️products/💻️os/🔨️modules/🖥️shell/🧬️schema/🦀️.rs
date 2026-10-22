//! 🧬️ `os.shell` schema module — the AUTHORITY for every shell wire type. `🔣️.json` (draft-07,
//! `$id: https://json.schemas.assets.semio-tech.com/os/shell/component.json`) is the language-neutral contract;
//! [`schema_registry::TYPES`] is its Rust registry and the single input the TypeScript mirror at
//! `🧰️framework/🔨️modules/🐚️shell/🤖️generated/🟦️.ts` is rendered from (`repo run @semio-tech/framework-os-shell-rs:typegen`);
//! `🟦️.ts` is the hand-written TypeScript consumer of the same `🔣️.json`. `../🦀️.rs` owns only the
//! pure [`super::reduce`] transition and is a CONSUMER of this module.
//!
//! Every `$defs` export in `🔣️.json` has exactly one Rust type here and exactly one registry row;
//! `owned_json_schema_defs_match_registry` below is the gate that keeps the three in lockstep, and
//! `repo run @semio-tech/framework-os-shell-rs:schema-check` runs it plus the TypeScript-side
//! cross-check.
//!
//! Pure: no I/O, no clock, no `wasm_bindgen`/`web_sys`/`winit`/`tokio`/`std::thread`. Compiles for
//! native AND `wasm32-unknown-unknown`.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
// 🧬️ `#[derive(ToValue, FromValue)]`, additive alongside serde
// (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01). This crate is
// kernel-free, so `dsl_core` (mirroring the alias every other framework crate uses for the
// DslValue runtime, e.g. `🕸️graph`'s `extern crate semio_framework_os_kernel as dsl_core`) here
// resolves to `protocol::value` instead — used only by the hand-written `ShellCommand` bridge
// below; every derived container names it explicitly via `#[value(crate = "::protocol::value")]`.
use semio_framework_value_derive::{FromValue, ToValue};
use protocol::value as dsl_core;

//#region 🧬️SchemaRegistry
pub mod schema_registry {
    use std::collections::HashSet;

    /// 🧬️ One versioned shell wire type and its owned TypeScript projection.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct SchemaExport {
        pub name: &'static str,
        pub version: u16,
        pub typescript: &'static str,
    }

    pub const TYPES: &[SchemaExport] = &[
        SchemaExport { name: "ActiveSession", version: 1, typescript: r##"export type ActiveSession = { pluginId: string, appId: string, instanceId: number, };"## },
        SchemaExport { name: "Anchor", version: 1, typescript: r##"export type Anchor = "left" | "right" | "top" | "bottom";"## },
        SchemaExport { name: "AppRole", version: 1, typescript: r##"export type AppRole = string;"## },
        SchemaExport { name: "ArtifactSyncStatus", version: 1, typescript: r##"export type ArtifactSyncStatus = { "kind": "clean" } | { "kind": "dirty" } | { "kind": "syncing" } | { "kind": "errored", message: string, };"## },
        SchemaExport { name: "ByAnchor", version: 1, typescript: r##"export type ByAnchor<T> = { left: T, right: T, top: T, bottom: T, };"## },
        SchemaExport { name: "Conflict", version: 1, typescript: r##"export type Conflict = { conflictId: string, documentId: string, description: string, };"## },
        SchemaExport { name: "DialogState", version: 1, typescript: r##"export type DialogState = { dialogId: string, seedArgs: unknown, };"## },
        SchemaExport { name: "DockUiState", version: 1, typescript: r##"export type DockUiState = { layout: LayoutNode | null, panelsVisible: ByAnchor<boolean>, };"## },
        SchemaExport { name: "ExtraWindowInstance", version: 1, typescript: r##"export type ExtraWindowInstance = { windowId: string, kind: string, params: unknown, };"## },
        SchemaExport { name: "IconName", version: 1, typescript: r##"export type IconName = string;"## },
        SchemaExport { name: "InferencePortStatus", version: 1, typescript: r##"export type InferencePortStatus = { owner:string, serviceId:string, payload:unknown, };"## },
        SchemaExport { name: "LayoutNode", version: 1, typescript: r##"export type LayoutNode = { "kind": "leaf", windowId: string, } | { "kind": "split", orientation: SplitOrientation, children: Array<LayoutNode>, sizes: Array<number>, };"## },
        SchemaExport { name: "LoadedPlugin", version: 1, typescript: r##"export type LoadedPlugin = { pluginId: string, moduleUrl: string, label: string | null, };"## },
        SchemaExport { name: "MergePolicy", version: 1, typescript: r##"export type MergePolicy = "preferLocal" | "preferRemote" | "manual";"## },
        SchemaExport { name: "NoticeKind", version: 1, typescript: r##"export type NoticeKind = "info" | "success" | "warning" | "error";"## },
        SchemaExport { name: "PluginPanelStatus", version: 1, typescript: r##"export type PluginPanelStatus = { "kind": "open" } | { "kind": "collapsed" } | { "kind": "errored", message: string, };"## },
        SchemaExport { name: "PluginSupervisorState", version: 1, typescript: r##"export type PluginSupervisorState = { healthy: boolean, restartCount: number, lastSignalMs: number | null, };"## },
        SchemaExport { name: "ShellCapability", version: 1, typescript: r##"export type ShellCapability = { id: string, title: string, description: string, schema: unknown, observableOnly: boolean, };"## },
        SchemaExport {
            name: "ShellCommand",
            version: 1,
            typescript: r##"export type ShellCommand = { "type": "registerLoadedPlugin", plugin: LoadedPlugin, } | { "type": "unregisterLoadedPlugin", pluginId: string, } | { "type": "setPluginStatus", pluginId: string, status: PluginPanelStatus, } | { "type": "setPluginSupervisorState", pluginId: string, state: PluginSupervisorState, } | { "type": "setActiveSession", session: ActiveSession | null, } | { "type": "setSessionError", error: string | null, } | { "type": "setAppLabelOverride", appId: string, labelKey: string, value: string | null, } | { "type": "setActionPaneFolded", windowId: string, folded: boolean, } | { "type": "setActionPaneExpanded", windowId: string, actionId: string | null, } | { "type": "stageActionArg", windowId: string, actionId: string, argId: string, value: unknown, } | { "type": "resetActionArgs", windowId: string, actionId: string, } | { "type": "setActiveUtility", windowId: string, utilityId: string | null, } | { "type": "setActiveTool", toolId: string | null, } | { "type": "setCommandExpanded", commandId: string | null, } | { "type": "stageCommandArg", commandId: string, argId: string, value: unknown, } | { "type": "resetCommandArgs", commandId: string, } | { "type": "setPanelVisible", anchor: Anchor, visible: boolean, } | { "type": "setPanelSize", anchor: Anchor, size: number, } | { "type": "setPanelPath", anchor: Anchor, path: Array<string>, } | { "type": "setDockOverride", dock: LayoutNode | null, } | { "type": "setPanelPathMemory", panelKey: string, path: string | null, } | { "type": "setTreeOpenState", treeId: string, open: boolean, } | { "type": "hydrateDockUi", dock: DockUiState | null, } | { "type": "resetDock" } | { "type": "focusWindow", windowId: string | null, } | { "type": "setShellLayout", layout: LayoutNode | null, } | { "type": "setActiveExample", exampleId: string, } | { "type": "setMobilePanelPath", path: Array<string>, } | { "type": "setMobilePanelVisible", visible: boolean, } | { "type": "setExtraWindows", windows: Array<ExtraWindowInstance>, } | { "type": "setWindowTitle", windowId: string, title: string, } | { "type": "setWindowIcon", windowId: string, icon: IconName, } | { "type": "setSearchOpen", open: boolean, } | { "type": "setFindOpen", open: boolean, } | { "type": "autoStartIntroduction", key: string, } | { "type": "setIntroductionStep", stepIndex: number | null, } | { "type": "completeIntroductionInteraction", interactionIndex: number, } | { "type": "openDialog", dialogId: string, seedArgs: unknown, } | { "type": "closeDialog", dialogId: string | null, } | { "type": "showTransientNotice", notice: TransientNotice, } | { "type": "dismissTransientNotice" } | { "type": "setOpenWithFocusRole", role: AppRole | null, } | { "type": "setActiveTutorial", tutorialId: string | null, } | { "type": "setUiDriverDraft", draft: UiDriver | null, } | { "type": "setUiThemeDraft", draft: UiTheme | null, } | { "type": "setSyncBackboneUri", uri: string | null, } | { "type": "setSyncCardKind", kind: SyncCardKind | null, } | { "type": "setSyncDraftPath", path: string, } | { "type": "setDocumentSyncStatus", documentId: string, status: ArtifactSyncStatus, } | { "type": "setDocumentInferencePort", documentId: string, port: InferencePortStatus, } | { "type": "clearDocumentInferencePort", documentId: string, } | { "type": "setMergePolicy", policy: MergePolicy, } | { "type": "setConflicts", conflicts: Array<Conflict>, } | { "type": "selectConflict", conflictId: string | null, } | { "type": "setStorageScope", scope: ShellScope, } | { "type": "setOpeningPreference", role: string, dialectId: string | null, };"##,
        },
        SchemaExport {
            name: "ShellError",
            version: 1,
            typescript: r##"export type ShellError = { "kind": "emptyIdentifier", field: string, } | { "kind": "unknownPlugin", pluginId: string, } | { "kind": "unknownDialog", dialogId: string, } | { "kind": "unknownConflict", conflictId: string, } | { "kind": "invalidPanelSize", anchor: Anchor, size: number, };"##,
        },
        SchemaExport {
            name: "ShellEvent",
            version: 1,
            typescript: r##"export type ShellEvent = { "type": "applied", capabilityId: string, revision: number, } | { "type": "windowFocusChanged", previous: string | null, current: string | null, } | { "type": "activeUtilityChanged", windowId: string, previous: string | null, current: string | null, } | { "type": "activeToolChanged", previous: string | null, current: string | null, } | { "type": "dockReset" } | { "type": "dialogOpened", dialogId: string, } | { "type": "dialogClosed", dialogId: string, };"##,
        },
        SchemaExport { name: "ShellScope", version: 1, typescript: r##"export type ShellScope = "localStorage" | "memory";"## },
        SchemaExport {
            name: "ShellState",
            version: 1,
            typescript: r##"export type ShellState = { revision: number, loadedPlugins: Array<LoadedPlugin>, pluginStatusById: { [key in string]?: PluginPanelStatus }, pluginSupervisorById: { [key in string]?: PluginSupervisorState }, activeSession: ActiveSession | null, sessionError: string | null, appLabelsOverlay: { [key in string]?: { [key in string]?: string } }, actionPaneFoldedByWindow: { [key in string]?: boolean }, actionPaneExpandedByWindow: { [key in string]?: string | null }, stagedActionArgs: Record<string, Record<string, Record<string, unknown>>>, activeUtilityByWindow: { [key in string]?: string | null }, activeToolId: string | null, commandPanelExpanded: string | null, stagedCommandArgs: Record<string, Record<string, unknown>>, panelsVisible: ByAnchor<boolean>, panelsSize: ByAnchor<number>, panelsPath: ByAnchor<Array<string>>, dockOverride: LayoutNode | null, panelPathMemory: { [key in string]?: string }, treeOpenStates: { [key in string]?: boolean }, activeWindowId: string | null, shellLayout: LayoutNode | null, activeExampleId: string, mobilePanelPath: Array<string>, mobilePanelVisible: boolean, extraWindows: Array<ExtraWindowInstance>, windowTitlesById: { [key in string]?: string }, windowIconsById: { [key in string]?: IconName }, searchOpen: boolean, findOpen: boolean, introductionStepIndex: number | null, introductionAutoStartedKeys: Array<string>, introductionCompletedInteractions: Array<number>, dialogStack: Array<DialogState>, transientNotice: TransientNotice | null, openWithFocusRole: AppRole | null, activeTutorialId: string | null, uiDriverDraft: UiDriver | null, uiThemeDraft: UiTheme | null, syncBackboneUri: string | null, syncCardKind: SyncCardKind | null, syncDraftPath: string, syncStatusByDocument: { [key in string]?: ArtifactSyncStatus }, inferencePortByDocument: { [key in string]?: InferencePortStatus }, mergePolicy: MergePolicy, conflicts: Array<Conflict>, selectedConflictId: string | null, storageScope: ShellScope, openingPreferences: { [key in string]?: string }, };"##,
        },
        SchemaExport { name: "SplitOrientation", version: 1, typescript: r##"export type SplitOrientation = "horizontal" | "vertical";"## },
        SchemaExport { name: "SyncCardKind", version: 1, typescript: r##"export type SyncCardKind = "file" | "folder" | "remote";"## },
        SchemaExport { name: "TransientNotice", version: 1, typescript: r##"export type TransientNotice = { message: string, kind: NoticeKind, expiresAtMs: number | null, };"## },
        SchemaExport { name: "UiDriver", version: 1, typescript: r##"export type UiDriver = { driverId: string, label: string, config: unknown, };"## },
        SchemaExport { name: "UiTheme", version: 1, typescript: r##"export type UiTheme = { themeId: string, label: string, config: unknown, };"## },
    ];

    /// 🧬️ The language-neutral authority this registry projects — `🔣️.json` verbatim.
    pub const OWNED_JSON_SCHEMA: &str = include_str!("🔣️.json");

    /// 🪪️ The `$id` `🔣️.json` MUST declare, and the dialect it MUST be written in.
    pub const OWNED_JSON_SCHEMA_ID: &str = "https://json.schemas.assets.semio-tech.com/os/shell/component.json";
    pub const OWNED_JSON_SCHEMA_DIALECT: &str = "http://json-schema.org/draft-07/schema#";

    /// 🧾️ Every export id this registry declares, in declaration order.
    pub fn export_ids() -> Vec<&'static str> {
        TYPES.iter().map(|export| export.name).collect()
    }

    /// 🧬️ Every `$defs` key `🔣️.json` declares, in document order, after checking its dialect/`$id`.
    pub fn owned_json_schema_def_ids() -> Result<Vec<String>, String> {
        let document: serde_json::Value = serde_json::from_str(OWNED_JSON_SCHEMA).map_err(|error| format!("🔣️.json is not valid JSON: {error}"))?;
        let dialect = document.get("$schema").and_then(serde_json::Value::as_str).unwrap_or_default();
        if dialect != OWNED_JSON_SCHEMA_DIALECT {
            return Err(format!("🔣️.json declares dialect '{dialect}', expected '{OWNED_JSON_SCHEMA_DIALECT}'"));
        }
        let id = document.get("$id").and_then(serde_json::Value::as_str).unwrap_or_default();
        if id != OWNED_JSON_SCHEMA_ID {
            return Err(format!("🔣️.json declares $id '{id}', expected '{OWNED_JSON_SCHEMA_ID}'"));
        }
        let defs = document.get("$defs").and_then(serde_json::Value::as_object).ok_or_else(|| "🔣️.json has no `$defs` object".to_string())?;
        Ok(defs.keys().cloned().collect())
    }

    /// 🔍️ Rejects unversioned, duplicate, or name-mismatched schema rows before generation.
    pub fn validate() -> Result<(), String> {
        let mut names = HashSet::with_capacity(TYPES.len());
        for metadata in TYPES {
            if metadata.version == 0 {
                return Err(format!("schema '{}' has version zero", metadata.name));
            }
            if !names.insert(metadata.name) {
                return Err(format!("duplicate schema '{}'", metadata.name));
            }
            let prefix = format!("export type {}", metadata.name);
            if !metadata.typescript.starts_with(&prefix) {
                return Err(format!("schema '{}' declaration has a mismatched name", metadata.name));
            }
        }
        Ok(())
    }

    /// 🟦️ Renders the stable language projection consumed by every shell host.
    pub fn render_typescript() -> String {
        let mut output = String::from("/** @generated by repo run @semio-tech/framework-os-shell-rs:typegen from 🖥️shell/🧬️schema (🔣️.json + its Rust registry). Do not edit. */\n\n");
        for metadata in TYPES {
            output.push_str(metadata.typescript);
            output.push_str("\n\n");
        }
        output
    }
}
//#endregion 🧬️SchemaExport

#[path = "🌉️json-value-bridges/🦀️.rs"]
mod json_value_bridges;
pub use json_value_bridges::*;

#[path = "🧭️anchors-and-layout/🦀️.rs"]
mod anchors_and_layout;
pub use anchors_and_layout::*;

#[path = "🔌️plugin-runtime-and-overlays/🦀️.rs"]
mod plugin_runtime_and_overlays;
pub use plugin_runtime_and_overlays::*;

#[path = "🔄️sync-merge-and-scope/🦀️.rs"]
mod sync_merge_and_scope;
pub use sync_merge_and_scope::*;

#[path = "📋️shell-state/🦀️.rs"]
mod shell_state;
pub use shell_state::*;

#[path = "🎮️shell-command/🦀️.rs"]
mod shell_command;
pub use shell_command::*;

#[path = "🪢️shell-command-value-bridge/🦀️.rs"]
mod shell_command_value_bridge;

#[path = "📣️events-and-errors/🦀️.rs"]
mod events_and_errors;
pub use events_and_errors::*;

#[path = "🛰️capabilities/🦀️.rs"]
mod capabilities;
pub use capabilities::*;

//#region 🎨️UiPreferences
pub use semio_framework_os_config::opening_config::{UiAppearance, UiChromeLayout, UiDriver, UiLocale, UiPreferences, UiTheme};
//#endregion 🎨️UiPreferences

//#region 🧪️tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️tests
