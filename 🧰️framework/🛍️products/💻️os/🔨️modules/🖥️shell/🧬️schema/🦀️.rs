//! 🧬️ `os.shell` schema module — the AUTHORITY for every shell wire type. `🔣️.json` (draft-07,
//! `$id: https://semio.tech/schema/os/shell/component.json`) is the language-neutral contract;
//! [`schema_registry::TYPES`] is its Rust registry and the single input the TypeScript mirror at
//! `../🤖️generated/🟦️.ts` is rendered from (`bun nx run @semio-tech/framework-os-shell-rs:typegen`);
//! `🟦️.ts` is the hand-written TypeScript consumer of the same `🔣️.json`. `../🦀️.rs` owns only the
//! pure [`super::reduce`] transition and is a CONSUMER of this module.
//!
//! Every `$defs` export in `🔣️.json` has exactly one Rust type here and exactly one registry row;
//! `owned_json_schema_defs_match_registry` below is the gate that keeps the three in lockstep, and
//! `bun nx run @semio-tech/framework-os-shell-rs:schema-check` runs it plus the TypeScript-side
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
        SchemaExport { name: "InferencePortPhase", version: 1, typescript: r##"export type InferencePortPhase = "idle" | "submitting" | "running" | "offered" | "approving" | "applied" | "cancelled" | "stale" | "failed";"## },
        SchemaExport { name: "InferencePortStatus", version: 1, typescript: r##"export type InferencePortStatus = { phase: InferencePortPhase, jobId: string | null, cursor: number, completed: number, total: number, proposalHash: string | null, cancelRequested: boolean, code: string | null, };"## },
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
            typescript: r##"export type ShellCommand = { "type": "registerLoadedPlugin", plugin: LoadedPlugin, } | { "type": "unregisterLoadedPlugin", pluginId: string, } | { "type": "setPluginStatus", pluginId: string, status: PluginPanelStatus, } | { "type": "setPluginSupervisorState", pluginId: string, state: PluginSupervisorState, } | { "type": "setActiveSession", session: ActiveSession | null, } | { "type": "setSessionError", error: string | null, } | { "type": "setAppLabelOverride", appId: string, labelKey: string, value: string | null, } | { "type": "setActionPaneFolded", windowId: string, folded: boolean, } | { "type": "setActionPaneExpanded", windowId: string, actionId: string | null, } | { "type": "stageActionArg", windowId: string, actionId: string, argId: string, value: unknown, } | { "type": "resetActionArgs", windowId: string, actionId: string, } | { "type": "setActiveUtility", windowId: string, utilityId: string | null, } | { "type": "setActiveTool", toolId: string | null, } | { "type": "setCommandExpanded", commandId: string | null, } | { "type": "stageCommandArg", commandId: string, argId: string, value: unknown, } | { "type": "resetCommandArgs", commandId: string, } | { "type": "setPanelVisible", anchor: Anchor, visible: boolean, } | { "type": "setPanelSize", anchor: Anchor, size: number, } | { "type": "setPanelPath", anchor: Anchor, path: Array<string>, } | { "type": "setDockOverride", dock: LayoutNode | null, } | { "type": "setPanelPathMemory", panelKey: string, path: string | null, } | { "type": "setTreeOpenState", treeId: string, open: boolean, } | { "type": "hydrateDockUi", dock: DockUiState | null, } | { "type": "resetDock" } | { "type": "focusWindow", windowId: string | null, } | { "type": "setShellLayout", layout: LayoutNode | null, } | { "type": "setActiveExample", exampleId: string, } | { "type": "setMobilePanelPath", path: Array<string>, } | { "type": "setMobilePanelVisible", visible: boolean, } | { "type": "setExtraWindows", windows: Array<ExtraWindowInstance>, } | { "type": "setWindowTitle", windowId: string, title: string, } | { "type": "setWindowIcon", windowId: string, icon: IconName, } | { "type": "setSearchOpen", open: boolean, } | { "type": "setFindOpen", open: boolean, } | { "type": "autoStartIntroduction", key: string, } | { "type": "setIntroductionStep", stepIndex: number | null, } | { "type": "completeIntroductionInteraction", interactionIndex: number, } | { "type": "openDialog", dialogId: string, seedArgs: unknown, } | { "type": "closeDialog", dialogId: string | null, } | { "type": "showTransientNotice", notice: TransientNotice, } | { "type": "dismissTransientNotice" } | { "type": "setOpenWithFocusRole", role: AppRole | null, } | { "type": "setActiveTutorial", tutorialId: string | null, } | { "type": "setUiAppearance", appearance: UiAppearance, } | { "type": "setUiLayout", layout: UiChromeLayout, } | { "type": "setUiDriver", driverId: string, } | { "type": "setUiCustomDriver", driverId: string, driver: UiDriver | null, } | { "type": "setUiDriverDraft", draft: UiDriver | null, } | { "type": "setUiLocale", locale: UiLocale, } | { "type": "setUiTerminology", terminologyId: string, } | { "type": "setUiTheme", themeId: string, } | { "type": "setUiCustomTheme", themeId: string, theme: UiTheme | null, } | { "type": "setUiThemeDraft", draft: UiTheme | null, } | { "type": "setUiKeybindingOverride", controlId: string, keys: string | null, } | { "type": "setSyncBackboneUri", uri: string | null, } | { "type": "setSyncCardKind", kind: SyncCardKind | null, } | { "type": "setSyncDraftPath", path: string, } | { "type": "setDocumentSyncStatus", documentId: string, status: ArtifactSyncStatus, } | { "type": "setDocumentInferencePort", documentId: string, port: InferencePortStatus, } | { "type": "clearDocumentInferencePort", documentId: string, } | { "type": "setMergePolicy", policy: MergePolicy, } | { "type": "setConflicts", conflicts: Array<Conflict>, } | { "type": "selectConflict", conflictId: string | null, } | { "type": "setStorageScope", scope: ShellScope, } | { "type": "setOpeningPreference", role: string, dialectId: string | null, };"##,
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
            typescript: r##"export type ShellState = { revision: number, loadedPlugins: Array<LoadedPlugin>, pluginStatusById: { [key in string]?: PluginPanelStatus }, pluginSupervisorById: { [key in string]?: PluginSupervisorState }, activeSession: ActiveSession | null, sessionError: string | null, appLabelsOverlay: { [key in string]?: { [key in string]?: string } }, actionPaneFoldedByWindow: { [key in string]?: boolean }, actionPaneExpandedByWindow: { [key in string]?: string | null }, stagedActionArgs: Record<string, Record<string, Record<string, unknown>>>, activeUtilityByWindow: { [key in string]?: string | null }, activeToolId: string | null, commandPanelExpanded: string | null, stagedCommandArgs: Record<string, Record<string, unknown>>, panelsVisible: ByAnchor<boolean>, panelsSize: ByAnchor<number>, panelsPath: ByAnchor<Array<string>>, dockOverride: LayoutNode | null, panelPathMemory: { [key in string]?: string }, treeOpenStates: { [key in string]?: boolean }, activeWindowId: string | null, shellLayout: LayoutNode | null, activeExampleId: string, mobilePanelPath: Array<string>, mobilePanelVisible: boolean, extraWindows: Array<ExtraWindowInstance>, windowTitlesById: { [key in string]?: string }, windowIconsById: { [key in string]?: IconName }, searchOpen: boolean, findOpen: boolean, introductionStepIndex: number | null, introductionAutoStartedKeys: Array<string>, introductionCompletedInteractions: Array<number>, dialogStack: Array<DialogState>, transientNotice: TransientNotice | null, openWithFocusRole: AppRole | null, activeTutorialId: string | null, uiAppearance: UiAppearance, uiLayout: UiChromeLayout, uiDriverId: string, uiCustomDrivers: { [key in string]?: UiDriver }, uiDriverDraft: UiDriver | null, uiLocale: UiLocale, uiTerminology: string, uiThemeId: string, uiCustomThemes: { [key in string]?: UiTheme }, uiThemeDraft: UiTheme | null, uiKeybindingOverrides: { [key in string]?: string }, syncBackboneUri: string | null, syncCardKind: SyncCardKind | null, syncDraftPath: string, syncStatusByDocument: { [key in string]?: ArtifactSyncStatus }, inferencePortByDocument: { [key in string]?: InferencePortStatus }, mergePolicy: MergePolicy, conflicts: Array<Conflict>, selectedConflictId: string | null, storageScope: ShellScope, openingPreferences: { [key in string]?: string }, };"##,
        },
        SchemaExport { name: "SplitOrientation", version: 1, typescript: r##"export type SplitOrientation = "horizontal" | "vertical";"## },
        SchemaExport { name: "SyncCardKind", version: 1, typescript: r##"export type SyncCardKind = "file" | "folder" | "remote";"## },
        SchemaExport { name: "TransientNotice", version: 1, typescript: r##"export type TransientNotice = { message: string, kind: NoticeKind, expiresAtMs: number | null, };"## },
        SchemaExport { name: "UiAppearance", version: 1, typescript: r##"export type UiAppearance = "system" | "light" | "dark";"## },
        SchemaExport { name: "UiChromeLayout", version: 1, typescript: r##"export type UiChromeLayout = "default" | "compact";"## },
        SchemaExport { name: "UiDriver", version: 1, typescript: r##"export type UiDriver = { driverId: string, label: string, config: unknown, };"## },
        SchemaExport { name: "UiLocale", version: 1, typescript: r##"export type UiLocale = "en" | "de";"## },
        SchemaExport { name: "UiTheme", version: 1, typescript: r##"export type UiTheme = { themeId: string, label: string, tokens: { [key in string]?: string }, };"## },
    ];

    /// 🧬️ The language-neutral authority this registry projects — `🔣️.json` verbatim.
    pub const OWNED_JSON_SCHEMA: &str = include_str!("🔣️.json");

    /// 🪪️ The `$id` `🔣️.json` MUST declare, and the dialect it MUST be written in.
    pub const OWNED_JSON_SCHEMA_ID: &str = "https://semio.tech/schema/os/shell/component.json";
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
        let mut output = String::from("/** @generated by bun nx run @semio-tech/framework-os-shell-rs:typegen from 🖥️shell/🧬️schema (🔣️.json + its Rust registry). Do not edit. */\n\n");
        for metadata in TYPES {
            output.push_str(metadata.typescript);
            output.push_str("\n\n");
        }
        output
    }
}
//#endregion 🧬️SchemaExport

/// 🧩️ Free-form staged/seed argument payload (action args, command args, dialog seed args).
/// The owned schema metadata projects this deliberately open value as TypeScript `unknown`.
pub type JsonValue = serde_json::Value;

// #region 🌉️ ValueBridges
// 🌉️ `ToValue`/`FromValue` is not implemented for `serde_json::Value` anywhere reachable from this
// crate (implementing it here would be an orphan-rule violation: both the trait, defined in
// `protocol::value`, and the type, defined in `serde_json`, are foreign to this crate) — every
// `JsonValue`/`Option<JsonValue>`/`HashMap<..., JsonValue>` struct field below instead names one of
// these bridges via `#[value(with = "...")]`, built on `DslValue`'s existing bidirectional
// `serde_json::Value` conversions (`🌱️value/🦀️.rs`). ADDITIVE ONLY — round-trips through the exact
// same `DslValue` shape a plain `impl ToValue for serde_json::Value` would produce.
mod json_value_bridge {
    pub fn to_value(value: &super::JsonValue) -> dsl_core::DslValue {
        dsl_core::DslValue::from(value.clone())
    }
    pub fn from_value(value: dsl_core::DslValue) -> Result<super::JsonValue, dsl_core::ValueError> {
        Ok(super::JsonValue::from(value))
    }
    use super::dsl_core;
}

mod json_value_option_bridge {
    pub fn to_value(value: &Option<super::JsonValue>) -> dsl_core::DslValue {
        match value {
            Some(v) => dsl_core::DslValue::from(v.clone()),
            None => dsl_core::DslValue::Null,
        }
    }
    pub fn from_value(value: dsl_core::DslValue) -> Result<Option<super::JsonValue>, dsl_core::ValueError> {
        match value {
            dsl_core::DslValue::Null => Ok(None),
            other => Ok(Some(super::JsonValue::from(other))),
        }
    }
    use super::dsl_core;
}

/// 🌉️ `ShellState::staged_command_args`'s exact shape: `command_id -> arg_id -> value`.
mod staged_command_args_bridge {
    use std::collections::HashMap;
    pub fn to_value(value: &HashMap<String, HashMap<String, super::JsonValue>>) -> dsl_core::DslValue {
        dsl_core::DslValue::object(value.iter().map(|(k, inner)| {
            (k.clone(), dsl_core::DslValue::object(inner.iter().map(|(k2, v)| (k2.clone(), dsl_core::DslValue::from(v.clone())))))
        }))
    }
    pub fn from_value(value: dsl_core::DslValue) -> Result<HashMap<String, HashMap<String, super::JsonValue>>, dsl_core::ValueError> {
        let dsl_core::DslValue::Object(entries) = value else {
            return Err(dsl_core::ValueError::new("expected an object for staged_command_args"));
        };
        entries.into_iter().map(|(k, inner)| {
            let dsl_core::DslValue::Object(inner_entries) = inner else {
                return Err(dsl_core::ValueError::new("expected an object").under(k));
            };
            let inner_map = inner_entries.into_iter().map(|(k2, v)| (k2, super::JsonValue::from(v))).collect();
            Ok((k, inner_map))
        }).collect()
    }
    use super::dsl_core;
}

/// 🌉️ `ShellState::staged_action_args`'s exact shape: `window_id -> action_id -> arg_id -> value`.
mod staged_action_args_bridge {
    use std::collections::HashMap;
    pub fn to_value(value: &HashMap<String, HashMap<String, HashMap<String, super::JsonValue>>>) -> dsl_core::DslValue {
        dsl_core::DslValue::object(value.iter().map(|(k, mid)| {
            (k.clone(), dsl_core::DslValue::object(mid.iter().map(|(k2, inner)| {
                (k2.clone(), dsl_core::DslValue::object(inner.iter().map(|(k3, v)| (k3.clone(), dsl_core::DslValue::from(v.clone())))))
            })))
        }))
    }
    pub fn from_value(value: dsl_core::DslValue) -> Result<HashMap<String, HashMap<String, HashMap<String, super::JsonValue>>>, dsl_core::ValueError> {
        let dsl_core::DslValue::Object(entries) = value else {
            return Err(dsl_core::ValueError::new("expected an object for staged_action_args"));
        };
        entries.into_iter().map(|(k, mid)| {
            let dsl_core::DslValue::Object(mid_entries) = mid else {
                return Err(dsl_core::ValueError::new("expected an object").under(k));
            };
            let mid_map = mid_entries.into_iter().map(|(k2, inner)| {
                let dsl_core::DslValue::Object(inner_entries) = inner else {
                    return Err(dsl_core::ValueError::new("expected an object").under(k2));
                };
                let inner_map = inner_entries.into_iter().map(|(k3, v)| (k3, super::JsonValue::from(v))).collect();
                Ok((k2, inner_map))
            }).collect::<Result<HashMap<_, _>, dsl_core::ValueError>>()?;
            Ok((k, mid_map))
        }).collect()
    }
    use super::dsl_core;
}
// #endregion 🌉️ ValueBridges

//#region 🧭️Anchor
/// 🧭️ The four docking edges a panel can attach to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum Anchor {
    Left,
    Right,
    Top,
    Bottom,
}

impl Anchor {
    pub const ALL: [Anchor; 4] = [Anchor::Left, Anchor::Right, Anchor::Top, Anchor::Bottom];
}

/// 🧭️ One value per [`Anchor`] — `Anchor` is a small closed set (4 edges), so a fixed struct
/// avoids the "enum as HashMap key" JSON-serialization pitfall (serde_json map keys must reduce
/// to strings; a 4-field struct sidesteps the question entirely and is simpler to reason about).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct ByAnchor<T> {
    pub left: T,
    pub right: T,
    pub top: T,
    pub bottom: T,
}

impl<T: Clone> ByAnchor<T> {
    pub fn uniform(value: T) -> Self {
        Self { left: value.clone(), right: value.clone(), top: value.clone(), bottom: value }
    }

    pub fn get(&self, anchor: Anchor) -> &T {
        match anchor {
            Anchor::Left => &self.left,
            Anchor::Right => &self.right,
            Anchor::Top => &self.top,
            Anchor::Bottom => &self.bottom,
        }
    }

    pub fn set(&mut self, anchor: Anchor, value: T) {
        match anchor {
            Anchor::Left => self.left = value,
            Anchor::Right => self.right = value,
            Anchor::Top => self.top = value,
            Anchor::Bottom => self.bottom = value,
        }
    }
}
//#endregion 🧭️Anchor

//#region 🧱️LayoutNode
/// 🧱️ How two child regions of a split are arranged.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum SplitOrientation {
    Horizontal,
    Vertical,
}

/// 🧱️ Minimal local mirror of the real `WindowLayoutNode`/`DockSkeleton` tree types (audit
/// `📓️luna-shellstate-audit.md` §1, `shellLayout`/`dockOverride` rows). The canonical types live
/// in the React/wgpu shells' own modules, which this crate must not depend on (§4 of the packet:
/// "if depending on `semio-framework` drags in a mid-rewrite crate, define the minimal local
/// mirror and say so"). `dockOverride` and `shellLayout` are structurally the same kind of tree
/// (window ids at the leaves, nested splits), so one recursive type serves both; reconciling this
/// with the real tree types is later-packet adoption work.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(crate = "::protocol::value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum LayoutNode {
    Leaf { window_id: String },
    Split { orientation: SplitOrientation, children: Vec<LayoutNode>, sizes: Vec<f32> },
}

/// 🗂️ Restored dock UI state (`HYDRATE_DOCK_UI` row) — a layout tree plus per-anchor visibility,
/// the shape the audit's `DockUiState` payload carries.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct DockUiState {
    pub layout: Option<LayoutNode>,
    pub panels_visible: ByAnchor<bool>,
}
//#endregion 🧱️LayoutNode

//#region 🪟️Windows
/// 🪟️ One spawned/extra window instance (audit `ExtraWindowInstance`). `params` mirrors the same
/// free-form seed shape `OpenWindow`'s kernel effect carries.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct ExtraWindowInstance {
    pub window_id: String,
    pub kind: String,
    #[value(with = "json_value_option_bridge")]
    pub params: Option<JsonValue>,
}

/// 🖼️ Stable icon identifier. Local newtype mirror — the real `IconName` lives in the UI token
/// crate this module must not depend on (§4).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[value(crate = "::protocol::value", transparent)]
pub struct IconName(pub String);
//#endregion 🪟️Windows

//#region 🔌️PluginRuntime
/// 🔌️ One loaded plugin's registry entry (audit `LoadedProgramState` — its exact field shape was
/// never captured by the audit; this is a minimal domain-shaped mirror sufficient for identity +
/// registry semantics, reconciled at adoption time).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct LoadedPlugin {
    pub plugin_id: String,
    pub module_url: String,
    pub label: Option<String>,
}

/// 🪟️ Plugin panel open/collapsed UI state (audit: "plugin panel UI state (open/collapsed)").
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(crate = "::protocol::value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PluginPanelStatus {
    Open,
    Collapsed,
    Errored { message: String },
}

/// 🚑️ Plugin resource/failure monitoring summary (audit: "plugin resource/failure monitoring").
/// A minimal local mirror — the full failure ladder lives in the actor runtime crate this module
/// must not depend on (§4); this is the shell-observable projection of it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct PluginSupervisorState {
    pub healthy: bool,
    pub restart_count: u32,
    /// `u64`, but the wire format is plain JSON (this module has no binary codec), so the TS
    /// mirror is explicitly `number`, since millisecond epoch timestamps never
    /// approach `u64::MAX`, let alone JS's `Number.MAX_SAFE_INTEGER`.
    pub last_signal_ms: Option<u64>,
}

/// 🎯️ The active app instance binding (audit: "active app instance binding").
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct ActiveSession {
    pub plugin_id: String,
    pub app_id: String,
    pub instance_id: u32,
}
//#endregion 🔌️PluginRuntime

//#region 🔔️Overlays
/// 🪟️ One entry on the open-dialog stack. Modeled as a stack (not the audit's flat
/// `Option<dialog>`) so `OpenDialog`/`CloseDialog` can express dialog-over-dialog stacking — one
/// of the tricky paths the packet brief calls out explicitly; a flat `Option` cannot represent a
/// confirm dialog opened on top of a settings dialog.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct DialogState {
    pub dialog_id: String,
    #[value(with = "json_value_option_bridge")]
    pub seed_args: Option<JsonValue>,
}

/// 🔔️ Severity of a [`TransientNotice`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum NoticeKind {
    Info,
    Success,
    Warning,
    Error,
}

/// 🔔️ A non-blocking auto-dismiss notice.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct TransientNotice {
    pub message: String,
    pub kind: NoticeKind,
    /// See [`PluginSupervisorState::last_signal_ms`]'s docstring for why this is `number`, not
    /// a wider integer projection, in the TypeScript mirror.
    pub expires_at_ms: Option<u64>,
}

/// 🎭️ Which role group the Open panel should focus. Local newtype mirror (§4) — the real
/// `AppRole` enumeration lives in the plugin manifest surface this module must not depend on.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[value(crate = "::protocol::value", transparent)]
pub struct AppRole(pub String);
//#endregion 🔔️Overlays

//#region 🎨️UiPreferences
/// 🎨️ Appearance preference.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum UiAppearance {
    System,
    Light,
    Dark,
}

/// 📐️ UI chrome density (matches the `os.setDriver` command's declared select options
/// default/compact — audit §5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum UiChromeLayout {
    Default,
    Compact,
}

/// 🌐️ Interface language. CLAUDE.md requires "no default language" as a *product* policy (a host
/// must not silently prefer a language without an explicit choice reaching the user) — that is a
/// bootstrap-sequencing concern for the host, not something a plain-old-data enum can encode; this
/// type still needs a technical fallback value for [`ShellState::default`], documented there.
/// English first, then German, per CLAUDE.md.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum UiLocale {
    En,
    De,
}

/// 🚗️ A user-defined UI driver (audit: `uiCustomDrivers`/`uiDriverDraft`). Its full shape was
/// never captured by the audit; `config` carries whatever driver-specific data the real type has.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct UiDriver {
    pub driver_id: String,
    pub label: String,
    #[value(with = "json_value_bridge")]
    pub config: JsonValue,
}

/// 🎨️ A user-defined UI theme (audit: `uiCustomThemes`/`uiThemeDraft`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct UiTheme {
    pub theme_id: String,
    pub label: String,
    pub tokens: HashMap<String, String>,
}
//#endregion 🎨️UiPreferences

//#region 🔄️Sync
/// 🗂️ Check-in target type (audit: "check-in file/folder/remote type").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum SyncCardKind {
    File,
    Folder,
    Remote,
}

/// 🩺️ Per-document sync health (audit: `ArtifactSyncStatus`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(crate = "::protocol::value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ArtifactSyncStatus {
    Clean,
    Dirty,
    Syncing,
    Errored { message: String },
}
//#endregion 🔄️Sync

//#region 💡️InferencePort
/// 💡️ The complete rendered lifecycle of one host-owned ephemeral inference port. `Idle` and
/// `Submitting` have no server counterpart at all, `Approving` is the server's `approval-prepared`,
/// and the four terminals are exactly `Applied | Cancelled | Stale | Failed`. This is the neutral
/// twin of the transport-side vocabulary the browser worker and the native turn driver both speak;
/// the ticket's shared fixture is what keeps the two byte-identical.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "kebab-case")]
#[value(crate = "::protocol::value", rename_all = "kebab-case")]
pub enum InferencePortPhase {
    Idle,
    Submitting,
    Running,
    Offered,
    Approving,
    Applied,
    Cancelled,
    Stale,
    Failed,
}

impl InferencePortPhase {
    /// 🏁️ A terminal phase accepts no further server answer — only an explicit clear.
    pub const fn terminal(self) -> bool {
        matches!(self, Self::Applied | Self::Cancelled | Self::Stale | Self::Failed)
    }

    /// 🔊️ Work in flight announces politely; every terminal asserts.
    pub const fn aria_role(self) -> &'static str {
        if self.terminal() {
            "alert"
        } else {
            "status"
        }
    }

    /// 🗣️ Explicit English and German text; there is no default language.
    pub const fn text(self, locale: UiLocale) -> &'static str {
        match (self, locale) {
            (Self::Idle, UiLocale::En) => "No proposal requested.",
            (Self::Idle, UiLocale::De) => "Kein Vorschlag angefordert.",
            (Self::Submitting, UiLocale::En) => "Requesting a bounds proposal…",
            (Self::Submitting, UiLocale::De) => "Begrenzungsvorschlag wird angefordert…",
            (Self::Running, UiLocale::En) => "Computing the bounds proposal…",
            (Self::Running, UiLocale::De) => "Begrenzungsvorschlag wird berechnet…",
            (Self::Offered, UiLocale::En) => "A bounds proposal is ready for review.",
            (Self::Offered, UiLocale::De) => "Ein Begrenzungsvorschlag liegt zur Prüfung bereit.",
            (Self::Approving, UiLocale::En) => "Waiting for the server to commit the approved proposal…",
            (Self::Approving, UiLocale::De) => "Warten auf die Freigabe des Vorschlags durch den Server…",
            (Self::Applied, UiLocale::En) => "The approved proposal was committed to the document.",
            (Self::Applied, UiLocale::De) => "Der freigegebene Vorschlag wurde im Dokument übernommen.",
            (Self::Cancelled, UiLocale::En) => "The proposal was cancelled.",
            (Self::Cancelled, UiLocale::De) => "Der Vorschlag wurde abgebrochen.",
            (Self::Stale, UiLocale::En) => "The document changed while the proposal ran. Request a new one.",
            (Self::Stale, UiLocale::De) => "Das Dokument hat sich während des Vorschlags geändert. Fordern Sie einen neuen an.",
            (Self::Failed, UiLocale::En) => "The proposal did not complete.",
            (Self::Failed, UiLocale::De) => "Der Vorschlag wurde nicht abgeschlossen.",
        }
    }
}

/// 💡️ Ephemeral per-document inference-port state. It is never persisted into a document and never
/// carries a receipt, bearer, origin, path, base pack, proposal body or user identity — only the
/// phase, the server's own job id, its bounded progress cursor, the hash the server published, and
/// whether a cancel has been requested.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct InferencePortStatus {
    pub phase: InferencePortPhase,
    pub job_id: Option<String>,
    pub cursor: u64,
    pub completed: u64,
    pub total: u64,
    pub proposal_hash: Option<String>,
    pub cancel_requested: bool,
    pub code: Option<String>,
}
//#endregion 💡️InferencePort

//#region 🤝️Merge
/// 🤝️ Conflict resolution strategy (audit: `MergePolicy`, persisted).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum MergePolicy {
    PreferLocal,
    PreferRemote,
    Manual,
}

/// ⚠️ One open conflict on the roster (audit: `Conflict`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct Conflict {
    pub conflict_id: String,
    pub document_id: String,
    pub description: String,
}
//#endregion 🤝️Merge

//#region 💾️Host
/// 💾️ Storage backend for shell state persistence (ShellHost `scope` useState, audit §2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum ShellScope {
    LocalStorage,
    Memory,
}
//#endregion 💾️Host

//#region 📋️ShellState
/// 📋️ The shell's entire semantic UI state — the single source of truth. Render caches (UiNode
/// trees, engagements, measure overlays, tutorial playback transport state) are deliberately
/// absent; see `📓️terra-P9-report.md` "Rows excluded" for the row-by-row justification. `revision`
/// increments on every successfully applied [`ShellCommand`]; a rejected command never changes it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct ShellState {
    /// See [`PluginSupervisorState::last_signal_ms`]'s docstring for why this is `number`, not
    /// a wider integer projection, in the TypeScript mirror.
    pub revision: u64,

    //#region 🔌️PluginRuntime
    pub loaded_plugins: Vec<LoadedPlugin>,
    pub plugin_status_by_id: HashMap<String, PluginPanelStatus>,
    pub plugin_supervisor_by_id: HashMap<String, PluginSupervisorState>,
    pub active_session: Option<ActiveSession>,
    pub session_error: Option<String>,
    //#endregion 🔌️PluginRuntime

    //#region 🏷️AppLabels
    /// app_id -> (label_key -> override text). Ephemeral per shell instance (not persisted) —
    /// audit classifies this SEMANTIC but never states a persistence scope; treating it as
    /// session-local is the conservative reading (it is `PluginAppLabelsOverlay`, not a document
    /// or account-level preference).
    pub app_labels_overlay: HashMap<String, HashMap<String, String>>,
    //#endregion 🏷️AppLabels

    //#region 🎛️ActionRail
    pub action_pane_folded_by_window: HashMap<String, bool>,
    pub action_pane_expanded_by_window: HashMap<String, Option<String>>,
    /// window_id -> action_id -> arg_id -> value.
    #[value(with = "staged_action_args_bridge")]
    pub staged_action_args: HashMap<String, HashMap<String, HashMap<String, JsonValue>>>,
    pub active_utility_by_window: HashMap<String, Option<String>>,
    pub active_tool_id: Option<String>,
    //#endregion 🎛️ActionRail

    //#region 🎮️CommandPalette
    pub command_panel_expanded: Option<String>,
    /// command_id -> arg_id -> value.
    #[value(with = "staged_command_args_bridge")]
    pub staged_command_args: HashMap<String, HashMap<String, JsonValue>>,
    //#endregion 🎮️CommandPalette

    //#region 🗂️PanelLayout
    pub panels_visible: ByAnchor<bool>,
    pub panels_size: ByAnchor<f32>,
    pub panels_path: ByAnchor<Vec<String>>,
    pub dock_override: Option<LayoutNode>,
    pub panel_path_memory: HashMap<String, String>,
    pub tree_open_states: HashMap<String, bool>,
    pub active_window_id: Option<String>,
    pub shell_layout: Option<LayoutNode>,
    pub active_example_id: String,
    pub mobile_panel_path: Vec<String>,
    pub mobile_panel_visible: bool,
    pub extra_windows: Vec<ExtraWindowInstance>,
    pub window_titles_by_id: HashMap<String, String>,
    pub window_icons_by_id: HashMap<String, IconName>,
    //#endregion 🗂️PanelLayout

    //#region 🔔️Overlays
    pub search_open: bool,
    pub find_open: bool,
    pub introduction_step_index: Option<u32>,
    pub introduction_auto_started_keys: Vec<String>,
    pub introduction_completed_interactions: Vec<u32>,
    /// Open dialog stack, top-of-stack last. See [`DialogState`] for why this is a stack.
    pub dialog_stack: Vec<DialogState>,
    pub transient_notice: Option<TransientNotice>,
    pub open_with_focus_role: Option<AppRole>,
    //#endregion 🔔️Overlays

    //#region 🎓️Tutorial
    /// Only the persisted/semantic subset survives here — playback transport state
    /// (playing/rate/muted/captions/recording/deviated) is TRANSIENT per the audit and stays in
    /// the renderer.
    pub active_tutorial_id: Option<String>,
    //#endregion 🎓️Tutorial

    //#region 🎨️UiPreferences
    pub ui_appearance: UiAppearance,
    pub ui_layout: UiChromeLayout,
    pub ui_driver_id: String,
    pub ui_custom_drivers: HashMap<String, UiDriver>,
    pub ui_driver_draft: Option<UiDriver>,
    pub ui_locale: UiLocale,
    pub ui_terminology: String,
    pub ui_theme_id: String,
    pub ui_custom_themes: HashMap<String, UiTheme>,
    pub ui_theme_draft: Option<UiTheme>,
    pub ui_keybinding_overrides: HashMap<String, String>,
    //#endregion 🎨️UiPreferences

    //#region 🔄️Sync
    pub sync_backbone_uri: Option<String>,
    pub sync_card_kind: Option<SyncCardKind>,
    pub sync_draft_path: String,
    pub sync_status_by_document: HashMap<String, ArtifactSyncStatus>,
    //#endregion 🔄️Sync

    //#region 💡️InferencePort
    /// Ephemeral per-document inference-port state, keyed by document id. Never persisted.
    pub inference_port_by_document: HashMap<String, InferencePortStatus>,
    //#endregion 💡️InferencePort

    //#region 🤝️Merge
    pub merge_policy: MergePolicy,
    pub conflicts: Vec<Conflict>,
    pub selected_conflict_id: Option<String>,
    //#endregion 🤝️Merge

    //#region 💾️Host
    pub storage_scope: ShellScope,
    /// role -> default dialect/app id (ShellHost `openingPreferences`, audit §2).
    pub opening_preferences: HashMap<String, String>,
    //#endregion 💾️Host
}

impl Default for ShellState {
    /// 🧪️ A purely technical starting point for tests/fixtures/bootstrap wiring — NOT a product
    /// choice of language, theme, or driver. CLAUDE.md requires "no default language"; a host
    /// integrating this module MUST issue an explicit `SetUiLocale`/`SetUiAppearance`/… during its
    /// own bootstrap sequence before presenting UI, exactly as it must do today for the React
    /// reducer's own initial state.
    fn default() -> Self {
        ShellState {
            revision: 0,
            loaded_plugins: Vec::new(),
            plugin_status_by_id: HashMap::new(),
            plugin_supervisor_by_id: HashMap::new(),
            active_session: None,
            session_error: None,
            app_labels_overlay: HashMap::new(),
            action_pane_folded_by_window: HashMap::new(),
            action_pane_expanded_by_window: HashMap::new(),
            staged_action_args: HashMap::new(),
            active_utility_by_window: HashMap::new(),
            active_tool_id: None,
            command_panel_expanded: None,
            staged_command_args: HashMap::new(),
            panels_visible: ByAnchor::uniform(false),
            panels_size: ByAnchor::uniform(280.0),
            panels_path: ByAnchor::uniform(Vec::new()),
            dock_override: None,
            panel_path_memory: HashMap::new(),
            tree_open_states: HashMap::new(),
            active_window_id: None,
            shell_layout: None,
            active_example_id: String::new(),
            mobile_panel_path: Vec::new(),
            mobile_panel_visible: false,
            extra_windows: Vec::new(),
            window_titles_by_id: HashMap::new(),
            window_icons_by_id: HashMap::new(),
            search_open: false,
            find_open: false,
            introduction_step_index: None,
            introduction_auto_started_keys: Vec::new(),
            introduction_completed_interactions: Vec::new(),
            dialog_stack: Vec::new(),
            transient_notice: None,
            open_with_focus_role: None,
            active_tutorial_id: None,
            ui_appearance: UiAppearance::System,
            ui_layout: UiChromeLayout::Default,
            ui_driver_id: String::new(),
            ui_custom_drivers: HashMap::new(),
            ui_driver_draft: None,
            ui_locale: UiLocale::En,
            ui_terminology: String::new(),
            ui_theme_id: String::new(),
            ui_custom_themes: HashMap::new(),
            ui_theme_draft: None,
            ui_keybinding_overrides: HashMap::new(),
            sync_backbone_uri: None,
            sync_card_kind: None,
            sync_draft_path: String::new(),
            sync_status_by_document: HashMap::new(),
            inference_port_by_document: HashMap::new(),
            merge_policy: MergePolicy::Manual,
            conflicts: Vec::new(),
            selected_conflict_id: None,
            storage_scope: ShellScope::Memory,
            opening_preferences: HashMap::new(),
        }
    }
}
//#endregion 📋️ShellState

//#region 🎮️ShellCommand
/// 🎮️ The vocabulary the LLM (and every other shell client) speaks. Every variant is plain
/// serializable data — no `Updatable<T>`/functional-updater payload (a React idiom that cannot
/// cross a wire); every Record/HashMap-shaped mutation from the TS reducer is decomposed into a
/// single-key upsert/remove command (`value: None` removes) rather than a whole-map replacement,
/// which is both wire-safe and small-diff-friendly for an LLM caller.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "type", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ShellCommand {
    // ── Plugin runtime — audit UPSERT_LOADED_PLUGIN/REMOVE_LOADED_PLUGIN/SET_PLUGIN_STATUS/SET_PLUGIN_SUPERVISOR/SET_SESSION/SET_ERROR
    RegisterLoadedPlugin { plugin: LoadedPlugin },
    UnregisterLoadedPlugin { plugin_id: String },
    SetPluginStatus { plugin_id: String, status: PluginPanelStatus },
    SetPluginSupervisorState { plugin_id: String, state: PluginSupervisorState },
    SetActiveSession { session: Option<ActiveSession> },
    SetSessionError { error: Option<String> },

    // ── App labels — audit SET_APP_LABELS_OVERLAY
    SetAppLabelOverride { app_id: String, label_key: String, value: Option<String> },

    // ── Action rail — audit SET_ACTION_PANE_FOLDED/SET_ACTION_PANE_EXPANDED/STAGE_ACTION_ARG/RESET_ACTION_ARGS/SET_ACTIVE_UTILITY/SET_ACTIVE_TOOL
    SetActionPaneFolded { window_id: String, folded: bool },
    SetActionPaneExpanded { window_id: String, action_id: Option<String> },
    StageActionArg { window_id: String, action_id: String, arg_id: String, value: JsonValue },
    ResetActionArgs { window_id: String, action_id: String },
    SetActiveUtility { window_id: String, utility_id: Option<String> },
    SetActiveTool { tool_id: Option<String> },

    // ── Command palette — audit SET_COMMAND_EXPANDED/STAGE_COMMAND_ARG/RESET_COMMAND_ARGS
    SetCommandExpanded { command_id: Option<String> },
    StageCommandArg { command_id: String, arg_id: String, value: JsonValue },
    ResetCommandArgs { command_id: String },

    // ── Panel layout — audit SET_PANEL_VISIBLE/SET_PANEL_SIZE/SET_PANEL_PATH/SET_DOCK_OVERRIDE/SET_PANEL_PATH_MEMORY/SET_TREE_OPEN_STATE/HYDRATE_DOCK_UI/RESET_DOCK/SET_ACTIVE_WINDOW_ID/SET_SHELL_LAYOUT/SET_ACTIVE_EXAMPLE_ID/SET_MOBILE_PANEL_PATH/SET_MOBILE_PANEL_VISIBLE/SET_EXTRA_WINDOW_INSTANCES/SET_WINDOW_TITLE/SET_WINDOW_ICON
    SetPanelVisible { anchor: Anchor, visible: bool },
    SetPanelSize { anchor: Anchor, size: f32 },
    SetPanelPath { anchor: Anchor, path: Vec<String> },
    SetDockOverride { dock: Option<LayoutNode> },
    SetPanelPathMemory { panel_key: String, path: Option<String> },
    SetTreeOpenState { tree_id: String, open: bool },
    HydrateDockUi { dock: Option<DockUiState> },
    ResetDock,
    FocusWindow { window_id: Option<String> },
    SetShellLayout { layout: Option<LayoutNode> },
    SetActiveExample { example_id: String },
    SetMobilePanelPath { path: Vec<String> },
    SetMobilePanelVisible { visible: bool },
    SetExtraWindows { windows: Vec<ExtraWindowInstance> },
    SetWindowTitle { window_id: String, title: String },
    SetWindowIcon { window_id: String, icon: IconName },

    // ── Overlays / dialogs — audit SET_SEARCH_OPEN/SET_FIND_OPEN/AUTO_START_INTRODUCTION/SET_INTRODUCTION_STEP/COMPLETE_INTRODUCTION_INTERACTION/SET_DIALOG/SET_TRANSIENT_NOTICE/SET_OPEN_WITH_FOCUS_ROLE
    SetSearchOpen { open: bool },
    SetFindOpen { open: bool },
    AutoStartIntroduction { key: String },
    SetIntroductionStep { step_index: Option<u32> },
    CompleteIntroductionInteraction { interaction_index: u32 },
    OpenDialog { dialog_id: String, seed_args: Option<JsonValue> },
    CloseDialog { dialog_id: Option<String> },
    ShowTransientNotice { notice: TransientNotice },
    DismissTransientNotice,
    SetOpenWithFocusRole { role: Option<AppRole> },

    // ── Tutorial (semantic subset) — audit SET_TUTORIAL
    SetActiveTutorial { tutorial_id: Option<String> },

    // ── UI preferences — audit SET_UI_APPEARANCE/SET_UI_LAYOUT/SET_UI_DRIVER_ID/SET_UI_CUSTOM_DRIVERS/SET_UI_DRIVER_DRAFT/SET_UI_LOCALE/SET_UI_TERMINOLOGY/SET_UI_THEME_ID/SET_UI_CUSTOM_THEMES/SET_UI_THEME_DRAFT/SET_UI_KEYBINDING_OVERRIDES
    SetUiAppearance { appearance: UiAppearance },
    SetUiLayout { layout: UiChromeLayout },
    SetUiDriver { driver_id: String },
    SetUiCustomDriver { driver_id: String, driver: Option<UiDriver> },
    SetUiDriverDraft { draft: Option<UiDriver> },
    SetUiLocale { locale: UiLocale },
    SetUiTerminology { terminology_id: String },
    SetUiTheme { theme_id: String },
    SetUiCustomTheme { theme_id: String, theme: Option<UiTheme> },
    SetUiThemeDraft { draft: Option<UiTheme> },
    SetUiKeybindingOverride { control_id: String, keys: Option<String> },

    // ── Sync — audit SET_SYNC_BACKBONE_URI/SET_SYNC_CARD_KIND/SET_SYNC_DRAFT_PATH/SET_SYNC_STATUS_FOR_DOCUMENT
    SetSyncBackboneUri { uri: Option<String> },
    SetSyncCardKind { kind: Option<SyncCardKind> },
    SetSyncDraftPath { path: String },
    SetDocumentSyncStatus { document_id: String, status: ArtifactSyncStatus },

    // ── Inference port — host-owned ephemeral proposal lifecycle, never a document mutation
    SetDocumentInferencePort { document_id: String, port: InferencePortStatus },
    ClearDocumentInferencePort { document_id: String },

    // ── Merge / conflicts — audit SET_MERGE_POLICY/SET_CONFLICTS/SET_SELECTED_CONFLICT_ID
    SetMergePolicy { policy: MergePolicy },
    SetConflicts { conflicts: Vec<Conflict> },
    SelectConflict { conflict_id: Option<String> },

    // ── Host prefs — ShellHost `scope`/`openingPreferences` useState (audit §2)
    SetStorageScope { scope: ShellScope },
    SetOpeningPreference { role: String, dialect_id: Option<String> },
}
//#endregion 🎮️ShellCommand

//#region 🌉️ Hand-written ToValue/FromValue bridge for ShellCommand
// ShellCommand cannot use #[derive(ToValue, FromValue)]: three variants (StageActionArg.value,
// StageCommandArg.value, OpenDialog.seed_args) carry a `JsonValue` (= serde_json::Value) field on an
// ENUM VARIANT, and #[value(with = "...")] is deliberately unsupported there (semio-framework-value-derive
// docs, `expand`) — this file hand-writes the identical tag="type"/rename_all="camelCase"/
// rename_all_fields="camelCase" wire shape `#[serde(...)]` produces instead, byte-for-byte. Builds
// DslValue directly (never dispatches back through ToValue/FromValue on ShellCommand itself).
impl dsl_core::ToValue for ShellCommand {
    fn to_value(&self) -> dsl_core::DslValue {
        match self {
            Self::RegisterLoadedPlugin { plugin } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("registerLoadedPlugin".to_string())));
                entries.push(("plugin".to_string(), dsl_core::ToValue::to_value(plugin)));
                dsl_core::DslValue::Object(entries)
            }
            Self::UnregisterLoadedPlugin { plugin_id } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("unregisterLoadedPlugin".to_string())));
                entries.push(("pluginId".to_string(), dsl_core::ToValue::to_value(plugin_id)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetPluginStatus { plugin_id, status } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setPluginStatus".to_string())));
                entries.push(("pluginId".to_string(), dsl_core::ToValue::to_value(plugin_id)));
                entries.push(("status".to_string(), dsl_core::ToValue::to_value(status)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetPluginSupervisorState { plugin_id, state } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setPluginSupervisorState".to_string())));
                entries.push(("pluginId".to_string(), dsl_core::ToValue::to_value(plugin_id)));
                entries.push(("state".to_string(), dsl_core::ToValue::to_value(state)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetActiveSession { session } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setActiveSession".to_string())));
                entries.push(("session".to_string(), dsl_core::ToValue::to_value(session)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetSessionError { error } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setSessionError".to_string())));
                entries.push(("error".to_string(), dsl_core::ToValue::to_value(error)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetAppLabelOverride { app_id, label_key, value } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setAppLabelOverride".to_string())));
                entries.push(("appId".to_string(), dsl_core::ToValue::to_value(app_id)));
                entries.push(("labelKey".to_string(), dsl_core::ToValue::to_value(label_key)));
                entries.push(("value".to_string(), dsl_core::ToValue::to_value(value)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetActionPaneFolded { window_id, folded } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setActionPaneFolded".to_string())));
                entries.push(("windowId".to_string(), dsl_core::ToValue::to_value(window_id)));
                entries.push(("folded".to_string(), dsl_core::ToValue::to_value(folded)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetActionPaneExpanded { window_id, action_id } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setActionPaneExpanded".to_string())));
                entries.push(("windowId".to_string(), dsl_core::ToValue::to_value(window_id)));
                entries.push(("actionId".to_string(), dsl_core::ToValue::to_value(action_id)));
                dsl_core::DslValue::Object(entries)
            }
            Self::StageActionArg { window_id, action_id, arg_id, value } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("stageActionArg".to_string())));
                entries.push(("windowId".to_string(), dsl_core::ToValue::to_value(window_id)));
                entries.push(("actionId".to_string(), dsl_core::ToValue::to_value(action_id)));
                entries.push(("argId".to_string(), dsl_core::ToValue::to_value(arg_id)));
                entries.push(("value".to_string(), dsl_core::DslValue::from(value.clone())));
                dsl_core::DslValue::Object(entries)
            }
            Self::ResetActionArgs { window_id, action_id } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("resetActionArgs".to_string())));
                entries.push(("windowId".to_string(), dsl_core::ToValue::to_value(window_id)));
                entries.push(("actionId".to_string(), dsl_core::ToValue::to_value(action_id)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetActiveUtility { window_id, utility_id } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setActiveUtility".to_string())));
                entries.push(("windowId".to_string(), dsl_core::ToValue::to_value(window_id)));
                entries.push(("utilityId".to_string(), dsl_core::ToValue::to_value(utility_id)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetActiveTool { tool_id } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setActiveTool".to_string())));
                entries.push(("toolId".to_string(), dsl_core::ToValue::to_value(tool_id)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetCommandExpanded { command_id } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setCommandExpanded".to_string())));
                entries.push(("commandId".to_string(), dsl_core::ToValue::to_value(command_id)));
                dsl_core::DslValue::Object(entries)
            }
            Self::StageCommandArg { command_id, arg_id, value } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("stageCommandArg".to_string())));
                entries.push(("commandId".to_string(), dsl_core::ToValue::to_value(command_id)));
                entries.push(("argId".to_string(), dsl_core::ToValue::to_value(arg_id)));
                entries.push(("value".to_string(), dsl_core::DslValue::from(value.clone())));
                dsl_core::DslValue::Object(entries)
            }
            Self::ResetCommandArgs { command_id } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("resetCommandArgs".to_string())));
                entries.push(("commandId".to_string(), dsl_core::ToValue::to_value(command_id)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetPanelVisible { anchor, visible } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setPanelVisible".to_string())));
                entries.push(("anchor".to_string(), dsl_core::ToValue::to_value(anchor)));
                entries.push(("visible".to_string(), dsl_core::ToValue::to_value(visible)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetPanelSize { anchor, size } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setPanelSize".to_string())));
                entries.push(("anchor".to_string(), dsl_core::ToValue::to_value(anchor)));
                entries.push(("size".to_string(), dsl_core::ToValue::to_value(size)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetPanelPath { anchor, path } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setPanelPath".to_string())));
                entries.push(("anchor".to_string(), dsl_core::ToValue::to_value(anchor)));
                entries.push(("path".to_string(), dsl_core::ToValue::to_value(path)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetDockOverride { dock } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setDockOverride".to_string())));
                entries.push(("dock".to_string(), dsl_core::ToValue::to_value(dock)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetPanelPathMemory { panel_key, path } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setPanelPathMemory".to_string())));
                entries.push(("panelKey".to_string(), dsl_core::ToValue::to_value(panel_key)));
                entries.push(("path".to_string(), dsl_core::ToValue::to_value(path)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetTreeOpenState { tree_id, open } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setTreeOpenState".to_string())));
                entries.push(("treeId".to_string(), dsl_core::ToValue::to_value(tree_id)));
                entries.push(("open".to_string(), dsl_core::ToValue::to_value(open)));
                dsl_core::DslValue::Object(entries)
            }
            Self::HydrateDockUi { dock } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("hydrateDockUi".to_string())));
                entries.push(("dock".to_string(), dsl_core::ToValue::to_value(dock)));
                dsl_core::DslValue::Object(entries)
            }
            Self::ResetDock => dsl_core::DslValue::Object(vec![("type".to_string(), dsl_core::DslValue::String("resetDock".to_string()))]),
            Self::FocusWindow { window_id } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("focusWindow".to_string())));
                entries.push(("windowId".to_string(), dsl_core::ToValue::to_value(window_id)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetShellLayout { layout } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setShellLayout".to_string())));
                entries.push(("layout".to_string(), dsl_core::ToValue::to_value(layout)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetActiveExample { example_id } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setActiveExample".to_string())));
                entries.push(("exampleId".to_string(), dsl_core::ToValue::to_value(example_id)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetMobilePanelPath { path } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setMobilePanelPath".to_string())));
                entries.push(("path".to_string(), dsl_core::ToValue::to_value(path)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetMobilePanelVisible { visible } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setMobilePanelVisible".to_string())));
                entries.push(("visible".to_string(), dsl_core::ToValue::to_value(visible)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetExtraWindows { windows } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setExtraWindows".to_string())));
                entries.push(("windows".to_string(), dsl_core::ToValue::to_value(windows)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetWindowTitle { window_id, title } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setWindowTitle".to_string())));
                entries.push(("windowId".to_string(), dsl_core::ToValue::to_value(window_id)));
                entries.push(("title".to_string(), dsl_core::ToValue::to_value(title)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetWindowIcon { window_id, icon } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setWindowIcon".to_string())));
                entries.push(("windowId".to_string(), dsl_core::ToValue::to_value(window_id)));
                entries.push(("icon".to_string(), dsl_core::ToValue::to_value(icon)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetSearchOpen { open } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setSearchOpen".to_string())));
                entries.push(("open".to_string(), dsl_core::ToValue::to_value(open)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetFindOpen { open } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setFindOpen".to_string())));
                entries.push(("open".to_string(), dsl_core::ToValue::to_value(open)));
                dsl_core::DslValue::Object(entries)
            }
            Self::AutoStartIntroduction { key } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("autoStartIntroduction".to_string())));
                entries.push(("key".to_string(), dsl_core::ToValue::to_value(key)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetIntroductionStep { step_index } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setIntroductionStep".to_string())));
                entries.push(("stepIndex".to_string(), dsl_core::ToValue::to_value(step_index)));
                dsl_core::DslValue::Object(entries)
            }
            Self::CompleteIntroductionInteraction { interaction_index } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("completeIntroductionInteraction".to_string())));
                entries.push(("interactionIndex".to_string(), dsl_core::ToValue::to_value(interaction_index)));
                dsl_core::DslValue::Object(entries)
            }
            Self::OpenDialog { dialog_id, seed_args } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("openDialog".to_string())));
                entries.push(("dialogId".to_string(), dsl_core::ToValue::to_value(dialog_id)));
                entries.push(("seedArgs".to_string(), match seed_args { Some(__v) => dsl_core::DslValue::from(__v.clone()), None => dsl_core::DslValue::Null }));
                dsl_core::DslValue::Object(entries)
            }
            Self::CloseDialog { dialog_id } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("closeDialog".to_string())));
                entries.push(("dialogId".to_string(), dsl_core::ToValue::to_value(dialog_id)));
                dsl_core::DslValue::Object(entries)
            }
            Self::ShowTransientNotice { notice } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("showTransientNotice".to_string())));
                entries.push(("notice".to_string(), dsl_core::ToValue::to_value(notice)));
                dsl_core::DslValue::Object(entries)
            }
            Self::DismissTransientNotice => dsl_core::DslValue::Object(vec![("type".to_string(), dsl_core::DslValue::String("dismissTransientNotice".to_string()))]),
            Self::SetOpenWithFocusRole { role } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setOpenWithFocusRole".to_string())));
                entries.push(("role".to_string(), dsl_core::ToValue::to_value(role)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetActiveTutorial { tutorial_id } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setActiveTutorial".to_string())));
                entries.push(("tutorialId".to_string(), dsl_core::ToValue::to_value(tutorial_id)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetUiAppearance { appearance } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setUiAppearance".to_string())));
                entries.push(("appearance".to_string(), dsl_core::ToValue::to_value(appearance)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetUiLayout { layout } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setUiLayout".to_string())));
                entries.push(("layout".to_string(), dsl_core::ToValue::to_value(layout)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetUiDriver { driver_id } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setUiDriver".to_string())));
                entries.push(("driverId".to_string(), dsl_core::ToValue::to_value(driver_id)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetUiCustomDriver { driver_id, driver } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setUiCustomDriver".to_string())));
                entries.push(("driverId".to_string(), dsl_core::ToValue::to_value(driver_id)));
                entries.push(("driver".to_string(), dsl_core::ToValue::to_value(driver)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetUiDriverDraft { draft } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setUiDriverDraft".to_string())));
                entries.push(("draft".to_string(), dsl_core::ToValue::to_value(draft)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetUiLocale { locale } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setUiLocale".to_string())));
                entries.push(("locale".to_string(), dsl_core::ToValue::to_value(locale)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetUiTerminology { terminology_id } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setUiTerminology".to_string())));
                entries.push(("terminologyId".to_string(), dsl_core::ToValue::to_value(terminology_id)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetUiTheme { theme_id } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setUiTheme".to_string())));
                entries.push(("themeId".to_string(), dsl_core::ToValue::to_value(theme_id)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetUiCustomTheme { theme_id, theme } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setUiCustomTheme".to_string())));
                entries.push(("themeId".to_string(), dsl_core::ToValue::to_value(theme_id)));
                entries.push(("theme".to_string(), dsl_core::ToValue::to_value(theme)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetUiThemeDraft { draft } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setUiThemeDraft".to_string())));
                entries.push(("draft".to_string(), dsl_core::ToValue::to_value(draft)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetUiKeybindingOverride { control_id, keys } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setUiKeybindingOverride".to_string())));
                entries.push(("controlId".to_string(), dsl_core::ToValue::to_value(control_id)));
                entries.push(("keys".to_string(), dsl_core::ToValue::to_value(keys)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetSyncBackboneUri { uri } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setSyncBackboneUri".to_string())));
                entries.push(("uri".to_string(), dsl_core::ToValue::to_value(uri)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetSyncCardKind { kind } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setSyncCardKind".to_string())));
                entries.push(("kind".to_string(), dsl_core::ToValue::to_value(kind)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetSyncDraftPath { path } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setSyncDraftPath".to_string())));
                entries.push(("path".to_string(), dsl_core::ToValue::to_value(path)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetDocumentSyncStatus { document_id, status } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setDocumentSyncStatus".to_string())));
                entries.push(("documentId".to_string(), dsl_core::ToValue::to_value(document_id)));
                entries.push(("status".to_string(), dsl_core::ToValue::to_value(status)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetDocumentInferencePort { document_id, port } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setDocumentInferencePort".to_string())));
                entries.push(("documentId".to_string(), dsl_core::ToValue::to_value(document_id)));
                entries.push(("port".to_string(), dsl_core::ToValue::to_value(port)));
                dsl_core::DslValue::Object(entries)
            }
            Self::ClearDocumentInferencePort { document_id } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("clearDocumentInferencePort".to_string())));
                entries.push(("documentId".to_string(), dsl_core::ToValue::to_value(document_id)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetMergePolicy { policy } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setMergePolicy".to_string())));
                entries.push(("policy".to_string(), dsl_core::ToValue::to_value(policy)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetConflicts { conflicts } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setConflicts".to_string())));
                entries.push(("conflicts".to_string(), dsl_core::ToValue::to_value(conflicts)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SelectConflict { conflict_id } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("selectConflict".to_string())));
                entries.push(("conflictId".to_string(), dsl_core::ToValue::to_value(conflict_id)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetStorageScope { scope } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setStorageScope".to_string())));
                entries.push(("scope".to_string(), dsl_core::ToValue::to_value(scope)));
                dsl_core::DslValue::Object(entries)
            }
            Self::SetOpeningPreference { role, dialect_id } => {
                let mut entries: Vec<(String, dsl_core::DslValue)> = Vec::new();
                entries.push(("type".to_string(), dsl_core::DslValue::String("setOpeningPreference".to_string())));
                entries.push(("role".to_string(), dsl_core::ToValue::to_value(role)));
                entries.push(("dialectId".to_string(), dsl_core::ToValue::to_value(dialect_id)));
                dsl_core::DslValue::Object(entries)
            }
        }
    }
}

impl dsl_core::FromValue for ShellCommand {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::Object(__entries) = value else {
            return Err(dsl_core::ValueError::new("expected an object for ShellCommand"));
        };
        let __tag = match __entries.iter().find(|(k, _)| k == "type") {
            Some((_, dsl_core::DslValue::String(s))) => s.clone(),
            _ => return Err(dsl_core::ValueError::new("missing string field `type`")),
        };
        match __tag.as_str() {
                "registerLoadedPlugin" => {
                let plugin = match __entries.iter().find(|(k, _)| k == "plugin") { Some((_, v)) => <LoadedPlugin as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("plugin"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `plugin`"))) };
                    Ok(Self::RegisterLoadedPlugin { plugin })
                }
                "unregisterLoadedPlugin" => {
                let plugin_id = match __entries.iter().find(|(k, _)| k == "pluginId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("pluginId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `pluginId`"))) };
                    Ok(Self::UnregisterLoadedPlugin { plugin_id })
                }
                "setPluginStatus" => {
                let plugin_id = match __entries.iter().find(|(k, _)| k == "pluginId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("pluginId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `pluginId`"))) };
                let status = match __entries.iter().find(|(k, _)| k == "status") { Some((_, v)) => <PluginPanelStatus as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("status"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `status`"))) };
                    Ok(Self::SetPluginStatus { plugin_id, status })
                }
                "setPluginSupervisorState" => {
                let plugin_id = match __entries.iter().find(|(k, _)| k == "pluginId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("pluginId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `pluginId`"))) };
                let state = match __entries.iter().find(|(k, _)| k == "state") { Some((_, v)) => <PluginSupervisorState as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("state"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `state`"))) };
                    Ok(Self::SetPluginSupervisorState { plugin_id, state })
                }
                "setActiveSession" => {
                let session = match __entries.iter().find(|(k, _)| k == "session") { Some((_, v)) => <Option<ActiveSession> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("session"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `session`"))) };
                    Ok(Self::SetActiveSession { session })
                }
                "setSessionError" => {
                let error = match __entries.iter().find(|(k, _)| k == "error") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("error"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `error`"))) };
                    Ok(Self::SetSessionError { error })
                }
                "setAppLabelOverride" => {
                let app_id = match __entries.iter().find(|(k, _)| k == "appId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("appId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `appId`"))) };
                let label_key = match __entries.iter().find(|(k, _)| k == "labelKey") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("labelKey"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `labelKey`"))) };
                let value = match __entries.iter().find(|(k, _)| k == "value") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("value"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `value`"))) };
                    Ok(Self::SetAppLabelOverride { app_id, label_key, value })
                }
                "setActionPaneFolded" => {
                let window_id = match __entries.iter().find(|(k, _)| k == "windowId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("windowId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `windowId`"))) };
                let folded = match __entries.iter().find(|(k, _)| k == "folded") { Some((_, v)) => <bool as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("folded"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `folded`"))) };
                    Ok(Self::SetActionPaneFolded { window_id, folded })
                }
                "setActionPaneExpanded" => {
                let window_id = match __entries.iter().find(|(k, _)| k == "windowId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("windowId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `windowId`"))) };
                let action_id = match __entries.iter().find(|(k, _)| k == "actionId") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("actionId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `actionId`"))) };
                    Ok(Self::SetActionPaneExpanded { window_id, action_id })
                }
                "stageActionArg" => {
                let window_id = match __entries.iter().find(|(k, _)| k == "windowId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("windowId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `windowId`"))) };
                let action_id = match __entries.iter().find(|(k, _)| k == "actionId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("actionId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `actionId`"))) };
                let arg_id = match __entries.iter().find(|(k, _)| k == "argId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("argId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `argId`"))) };
                let value = match __entries.iter().find(|(k, _)| k == "value") { Some((_, v)) => JsonValue::from(v.clone()), None => return Err(dsl_core::ValueError::new(format!("missing field `value`"))) };
                    Ok(Self::StageActionArg { window_id, action_id, arg_id, value })
                }
                "resetActionArgs" => {
                let window_id = match __entries.iter().find(|(k, _)| k == "windowId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("windowId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `windowId`"))) };
                let action_id = match __entries.iter().find(|(k, _)| k == "actionId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("actionId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `actionId`"))) };
                    Ok(Self::ResetActionArgs { window_id, action_id })
                }
                "setActiveUtility" => {
                let window_id = match __entries.iter().find(|(k, _)| k == "windowId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("windowId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `windowId`"))) };
                let utility_id = match __entries.iter().find(|(k, _)| k == "utilityId") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("utilityId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `utilityId`"))) };
                    Ok(Self::SetActiveUtility { window_id, utility_id })
                }
                "setActiveTool" => {
                let tool_id = match __entries.iter().find(|(k, _)| k == "toolId") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("toolId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `toolId`"))) };
                    Ok(Self::SetActiveTool { tool_id })
                }
                "setCommandExpanded" => {
                let command_id = match __entries.iter().find(|(k, _)| k == "commandId") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("commandId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `commandId`"))) };
                    Ok(Self::SetCommandExpanded { command_id })
                }
                "stageCommandArg" => {
                let command_id = match __entries.iter().find(|(k, _)| k == "commandId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("commandId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `commandId`"))) };
                let arg_id = match __entries.iter().find(|(k, _)| k == "argId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("argId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `argId`"))) };
                let value = match __entries.iter().find(|(k, _)| k == "value") { Some((_, v)) => JsonValue::from(v.clone()), None => return Err(dsl_core::ValueError::new(format!("missing field `value`"))) };
                    Ok(Self::StageCommandArg { command_id, arg_id, value })
                }
                "resetCommandArgs" => {
                let command_id = match __entries.iter().find(|(k, _)| k == "commandId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("commandId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `commandId`"))) };
                    Ok(Self::ResetCommandArgs { command_id })
                }
                "setPanelVisible" => {
                let anchor = match __entries.iter().find(|(k, _)| k == "anchor") { Some((_, v)) => <Anchor as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("anchor"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `anchor`"))) };
                let visible = match __entries.iter().find(|(k, _)| k == "visible") { Some((_, v)) => <bool as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("visible"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `visible`"))) };
                    Ok(Self::SetPanelVisible { anchor, visible })
                }
                "setPanelSize" => {
                let anchor = match __entries.iter().find(|(k, _)| k == "anchor") { Some((_, v)) => <Anchor as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("anchor"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `anchor`"))) };
                let size = match __entries.iter().find(|(k, _)| k == "size") { Some((_, v)) => <f32 as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("size"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `size`"))) };
                    Ok(Self::SetPanelSize { anchor, size })
                }
                "setPanelPath" => {
                let anchor = match __entries.iter().find(|(k, _)| k == "anchor") { Some((_, v)) => <Anchor as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("anchor"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `anchor`"))) };
                let path = match __entries.iter().find(|(k, _)| k == "path") { Some((_, v)) => <Vec<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("path"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `path`"))) };
                    Ok(Self::SetPanelPath { anchor, path })
                }
                "setDockOverride" => {
                let dock = match __entries.iter().find(|(k, _)| k == "dock") { Some((_, v)) => <Option<LayoutNode> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("dock"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `dock`"))) };
                    Ok(Self::SetDockOverride { dock })
                }
                "setPanelPathMemory" => {
                let panel_key = match __entries.iter().find(|(k, _)| k == "panelKey") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("panelKey"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `panelKey`"))) };
                let path = match __entries.iter().find(|(k, _)| k == "path") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("path"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `path`"))) };
                    Ok(Self::SetPanelPathMemory { panel_key, path })
                }
                "setTreeOpenState" => {
                let tree_id = match __entries.iter().find(|(k, _)| k == "treeId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("treeId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `treeId`"))) };
                let open = match __entries.iter().find(|(k, _)| k == "open") { Some((_, v)) => <bool as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("open"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `open`"))) };
                    Ok(Self::SetTreeOpenState { tree_id, open })
                }
                "hydrateDockUi" => {
                let dock = match __entries.iter().find(|(k, _)| k == "dock") { Some((_, v)) => <Option<DockUiState> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("dock"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `dock`"))) };
                    Ok(Self::HydrateDockUi { dock })
                }
                "resetDock" => Ok(Self::ResetDock),
                "focusWindow" => {
                let window_id = match __entries.iter().find(|(k, _)| k == "windowId") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("windowId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `windowId`"))) };
                    Ok(Self::FocusWindow { window_id })
                }
                "setShellLayout" => {
                let layout = match __entries.iter().find(|(k, _)| k == "layout") { Some((_, v)) => <Option<LayoutNode> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("layout"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `layout`"))) };
                    Ok(Self::SetShellLayout { layout })
                }
                "setActiveExample" => {
                let example_id = match __entries.iter().find(|(k, _)| k == "exampleId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("exampleId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `exampleId`"))) };
                    Ok(Self::SetActiveExample { example_id })
                }
                "setMobilePanelPath" => {
                let path = match __entries.iter().find(|(k, _)| k == "path") { Some((_, v)) => <Vec<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("path"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `path`"))) };
                    Ok(Self::SetMobilePanelPath { path })
                }
                "setMobilePanelVisible" => {
                let visible = match __entries.iter().find(|(k, _)| k == "visible") { Some((_, v)) => <bool as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("visible"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `visible`"))) };
                    Ok(Self::SetMobilePanelVisible { visible })
                }
                "setExtraWindows" => {
                let windows = match __entries.iter().find(|(k, _)| k == "windows") { Some((_, v)) => <Vec<ExtraWindowInstance> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("windows"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `windows`"))) };
                    Ok(Self::SetExtraWindows { windows })
                }
                "setWindowTitle" => {
                let window_id = match __entries.iter().find(|(k, _)| k == "windowId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("windowId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `windowId`"))) };
                let title = match __entries.iter().find(|(k, _)| k == "title") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("title"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `title`"))) };
                    Ok(Self::SetWindowTitle { window_id, title })
                }
                "setWindowIcon" => {
                let window_id = match __entries.iter().find(|(k, _)| k == "windowId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("windowId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `windowId`"))) };
                let icon = match __entries.iter().find(|(k, _)| k == "icon") { Some((_, v)) => <IconName as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("icon"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `icon`"))) };
                    Ok(Self::SetWindowIcon { window_id, icon })
                }
                "setSearchOpen" => {
                let open = match __entries.iter().find(|(k, _)| k == "open") { Some((_, v)) => <bool as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("open"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `open`"))) };
                    Ok(Self::SetSearchOpen { open })
                }
                "setFindOpen" => {
                let open = match __entries.iter().find(|(k, _)| k == "open") { Some((_, v)) => <bool as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("open"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `open`"))) };
                    Ok(Self::SetFindOpen { open })
                }
                "autoStartIntroduction" => {
                let key = match __entries.iter().find(|(k, _)| k == "key") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("key"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `key`"))) };
                    Ok(Self::AutoStartIntroduction { key })
                }
                "setIntroductionStep" => {
                let step_index = match __entries.iter().find(|(k, _)| k == "stepIndex") { Some((_, v)) => <Option<u32> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("stepIndex"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `stepIndex`"))) };
                    Ok(Self::SetIntroductionStep { step_index })
                }
                "completeIntroductionInteraction" => {
                let interaction_index = match __entries.iter().find(|(k, _)| k == "interactionIndex") { Some((_, v)) => <u32 as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("interactionIndex"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `interactionIndex`"))) };
                    Ok(Self::CompleteIntroductionInteraction { interaction_index })
                }
                "openDialog" => {
                let dialog_id = match __entries.iter().find(|(k, _)| k == "dialogId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("dialogId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `dialogId`"))) };
                let seed_args = match __entries.iter().find(|(k, _)| k == "seedArgs") { Some((_, dsl_core::DslValue::Null)) | None => None, Some((_, v)) => Some(JsonValue::from(v.clone())) };
                    Ok(Self::OpenDialog { dialog_id, seed_args })
                }
                "closeDialog" => {
                let dialog_id = match __entries.iter().find(|(k, _)| k == "dialogId") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("dialogId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `dialogId`"))) };
                    Ok(Self::CloseDialog { dialog_id })
                }
                "showTransientNotice" => {
                let notice = match __entries.iter().find(|(k, _)| k == "notice") { Some((_, v)) => <TransientNotice as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("notice"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `notice`"))) };
                    Ok(Self::ShowTransientNotice { notice })
                }
                "dismissTransientNotice" => Ok(Self::DismissTransientNotice),
                "setOpenWithFocusRole" => {
                let role = match __entries.iter().find(|(k, _)| k == "role") { Some((_, v)) => <Option<AppRole> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("role"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `role`"))) };
                    Ok(Self::SetOpenWithFocusRole { role })
                }
                "setActiveTutorial" => {
                let tutorial_id = match __entries.iter().find(|(k, _)| k == "tutorialId") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("tutorialId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `tutorialId`"))) };
                    Ok(Self::SetActiveTutorial { tutorial_id })
                }
                "setUiAppearance" => {
                let appearance = match __entries.iter().find(|(k, _)| k == "appearance") { Some((_, v)) => <UiAppearance as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("appearance"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `appearance`"))) };
                    Ok(Self::SetUiAppearance { appearance })
                }
                "setUiLayout" => {
                let layout = match __entries.iter().find(|(k, _)| k == "layout") { Some((_, v)) => <UiChromeLayout as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("layout"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `layout`"))) };
                    Ok(Self::SetUiLayout { layout })
                }
                "setUiDriver" => {
                let driver_id = match __entries.iter().find(|(k, _)| k == "driverId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("driverId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `driverId`"))) };
                    Ok(Self::SetUiDriver { driver_id })
                }
                "setUiCustomDriver" => {
                let driver_id = match __entries.iter().find(|(k, _)| k == "driverId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("driverId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `driverId`"))) };
                let driver = match __entries.iter().find(|(k, _)| k == "driver") { Some((_, v)) => <Option<UiDriver> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("driver"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `driver`"))) };
                    Ok(Self::SetUiCustomDriver { driver_id, driver })
                }
                "setUiDriverDraft" => {
                let draft = match __entries.iter().find(|(k, _)| k == "draft") { Some((_, v)) => <Option<UiDriver> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("draft"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `draft`"))) };
                    Ok(Self::SetUiDriverDraft { draft })
                }
                "setUiLocale" => {
                let locale = match __entries.iter().find(|(k, _)| k == "locale") { Some((_, v)) => <UiLocale as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("locale"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `locale`"))) };
                    Ok(Self::SetUiLocale { locale })
                }
                "setUiTerminology" => {
                let terminology_id = match __entries.iter().find(|(k, _)| k == "terminologyId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("terminologyId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `terminologyId`"))) };
                    Ok(Self::SetUiTerminology { terminology_id })
                }
                "setUiTheme" => {
                let theme_id = match __entries.iter().find(|(k, _)| k == "themeId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("themeId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `themeId`"))) };
                    Ok(Self::SetUiTheme { theme_id })
                }
                "setUiCustomTheme" => {
                let theme_id = match __entries.iter().find(|(k, _)| k == "themeId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("themeId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `themeId`"))) };
                let theme = match __entries.iter().find(|(k, _)| k == "theme") { Some((_, v)) => <Option<UiTheme> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("theme"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `theme`"))) };
                    Ok(Self::SetUiCustomTheme { theme_id, theme })
                }
                "setUiThemeDraft" => {
                let draft = match __entries.iter().find(|(k, _)| k == "draft") { Some((_, v)) => <Option<UiTheme> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("draft"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `draft`"))) };
                    Ok(Self::SetUiThemeDraft { draft })
                }
                "setUiKeybindingOverride" => {
                let control_id = match __entries.iter().find(|(k, _)| k == "controlId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("controlId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `controlId`"))) };
                let keys = match __entries.iter().find(|(k, _)| k == "keys") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("keys"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `keys`"))) };
                    Ok(Self::SetUiKeybindingOverride { control_id, keys })
                }
                "setSyncBackboneUri" => {
                let uri = match __entries.iter().find(|(k, _)| k == "uri") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("uri"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `uri`"))) };
                    Ok(Self::SetSyncBackboneUri { uri })
                }
                "setSyncCardKind" => {
                let kind = match __entries.iter().find(|(k, _)| k == "kind") { Some((_, v)) => <Option<SyncCardKind> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("kind"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `kind`"))) };
                    Ok(Self::SetSyncCardKind { kind })
                }
                "setSyncDraftPath" => {
                let path = match __entries.iter().find(|(k, _)| k == "path") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("path"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `path`"))) };
                    Ok(Self::SetSyncDraftPath { path })
                }
                "setDocumentSyncStatus" => {
                let document_id = match __entries.iter().find(|(k, _)| k == "documentId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("documentId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `documentId`"))) };
                let status = match __entries.iter().find(|(k, _)| k == "status") { Some((_, v)) => <ArtifactSyncStatus as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("status"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `status`"))) };
                    Ok(Self::SetDocumentSyncStatus { document_id, status })
                }
                "setDocumentInferencePort" => {
                let document_id = match __entries.iter().find(|(k, _)| k == "documentId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("documentId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `documentId`"))) };
                let port = match __entries.iter().find(|(k, _)| k == "port") { Some((_, v)) => <InferencePortStatus as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("port"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `port`"))) };
                    Ok(Self::SetDocumentInferencePort { document_id, port })
                }
                "clearDocumentInferencePort" => {
                let document_id = match __entries.iter().find(|(k, _)| k == "documentId") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("documentId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `documentId`"))) };
                    Ok(Self::ClearDocumentInferencePort { document_id })
                }
                "setMergePolicy" => {
                let policy = match __entries.iter().find(|(k, _)| k == "policy") { Some((_, v)) => <MergePolicy as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("policy"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `policy`"))) };
                    Ok(Self::SetMergePolicy { policy })
                }
                "setConflicts" => {
                let conflicts = match __entries.iter().find(|(k, _)| k == "conflicts") { Some((_, v)) => <Vec<Conflict> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("conflicts"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `conflicts`"))) };
                    Ok(Self::SetConflicts { conflicts })
                }
                "selectConflict" => {
                let conflict_id = match __entries.iter().find(|(k, _)| k == "conflictId") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("conflictId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `conflictId`"))) };
                    Ok(Self::SelectConflict { conflict_id })
                }
                "setStorageScope" => {
                let scope = match __entries.iter().find(|(k, _)| k == "scope") { Some((_, v)) => <ShellScope as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("scope"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `scope`"))) };
                    Ok(Self::SetStorageScope { scope })
                }
                "setOpeningPreference" => {
                let role = match __entries.iter().find(|(k, _)| k == "role") { Some((_, v)) => <String as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("role"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `role`"))) };
                let dialect_id = match __entries.iter().find(|(k, _)| k == "dialectId") { Some((_, v)) => <Option<String> as dsl_core::FromValue>::from_value(v.clone()).map_err(|e| e.under("dialectId"))?, None => return Err(dsl_core::ValueError::new(format!("missing field `dialectId`"))) };
                    Ok(Self::SetOpeningPreference { role, dialect_id })
                }
            other => Err(dsl_core::ValueError::new(format!("unknown ShellCommand type `{other}`"))),
        }
    }
}
//#endregion


//#region 📣️ShellEvent
/// 📣️ What [`reduce`] reports happened. Every accepted command always emits [`ShellEvent::Applied`]
/// (a deterministic, always-present baseline every fixture can assert on); commands whose
/// acceptance also triggers an automatic side effect (focus reassignment, mutual-exclusion
/// clearing, a dock reset) additionally emit the matching specific variant below. This is
/// deliberately NOT a 1:1 mirror of `ShellCommand` — most setters have no side effect beyond the
/// field they set, so a second event carrying the same payload as the command would be pure
/// duplication; the specific variants exist only where `reduce` does something beyond the literal
/// field write the command names.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(tag = "type", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(crate = "::protocol::value", tag = "type", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ShellEvent {
    /// Always emitted, exactly once, as the last event of every accepted command. `revision` is
    /// `number` — see [`PluginSupervisorState::last_signal_ms`].
    Applied {
        capability_id: String,
        revision: u64,
    },
    /// Focus moved — either directly (`FocusWindow`) or automatically (the previously-focused
    /// window disappeared from `SetExtraWindows`'s new list).
    WindowFocusChanged {
        previous: Option<String>,
        current: Option<String>,
    },
    /// `SetActiveTool` cleared a window's active utility for mode↔tool mutual exclusion.
    ActiveUtilityChanged {
        window_id: String,
        previous: Option<String>,
        current: Option<String>,
    },
    /// `SetActiveUtility` cleared the active tool for mode↔tool mutual exclusion.
    ActiveToolChanged {
        previous: Option<String>,
        current: Option<String>,
    },
    DockReset,
    DialogOpened {
        dialog_id: String,
    },
    DialogClosed {
        dialog_id: String,
    },
}
//#endregion 📣️ShellEvent

//#region 🚨️ShellError
/// 🚨️ Why a command was rejected. `reduce` never panics; every invalid transition returns one of
/// these instead of silently no-oping (packet §4: "invalid transitions return `ShellError` rather
/// than silently no-oping").
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(crate = "::protocol::value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ShellError {
    EmptyIdentifier { field: String },
    UnknownPlugin { plugin_id: String },
    UnknownDialog { dialog_id: String },
    UnknownConflict { conflict_id: String },
    InvalidPanelSize { anchor: Anchor, size: f32 },
}

impl std::fmt::Display for ShellError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => write!(formatter, "empty identifier for {field}"),
            Self::UnknownPlugin { plugin_id } => write!(formatter, "unknown plugin: {plugin_id}"),
            Self::UnknownDialog { dialog_id } => write!(formatter, "unknown dialog: {dialog_id}"),
            Self::UnknownConflict { conflict_id } => write!(formatter, "unknown conflict: {conflict_id}"),
            Self::InvalidPanelSize { anchor, size } => write!(formatter, "invalid panel size for {anchor:?}: {size}"),
        }
    }
}

impl std::error::Error for ShellError {}
//#endregion 🚨️ShellError

//#region 🛰️ShellCapability
/// 🛰️ One machine-readable descriptor per [`ShellCommand`] variant — what an MCP gateway (a later
/// packet) compiles into its tool catalog. Defined locally (packet §3: "do NOT depend on the
/// gateway crate or on `🛂️manifest`").
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct ShellCapability {
    /// Stable dotted id, e.g. `ui.window.focus`. Reuses the wgpu shell's existing informal
    /// `shell.*` verb strings where one already names this exact mutation (see
    /// `📓️terra-P9-report.md` "existing-verb → variant mapping" for the full table); coined fresh,
    /// domain-shaped, where no verb existed yet.
    pub id: String,
    pub title: String,
    pub description: String,
    /// JSON Schema for this variant's payload (schemars-derived from [`ShellCommand`]).
    #[value(with = "json_value_bridge")]
    pub schema: JsonValue,
    /// True for commands that only stage/observe data (no mutation with externally-visible
    /// consequence beyond the field itself) — none of today's commands are observable-only, this
    /// flag exists for the gateway to distinguish future read-modeling commands if any are added.
    pub observable_only: bool,
}

struct CommandMeta {
    id: &'static str,
    title: &'static str,
    description: &'static str,
    observable_only: bool,
}

/// 🆔 Returns the [`ShellCapability::id`] for a command instance — used by [`reduce`] to stamp
/// [`ShellEvent::Applied`]. Declaration order here MUST match [`ShellCommand`]'s declaration
/// order and [`SHELL_COMMAND_CATALOG`]'s order; `shell_capabilities_declaration_order_matches_enum`
/// guards this.
pub(crate) fn capability_id_for(command: &ShellCommand) -> String {
    let index = match command {
        ShellCommand::RegisterLoadedPlugin { .. } => 0,
        ShellCommand::UnregisterLoadedPlugin { .. } => 1,
        ShellCommand::SetPluginStatus { .. } => 2,
        ShellCommand::SetPluginSupervisorState { .. } => 3,
        ShellCommand::SetActiveSession { .. } => 4,
        ShellCommand::SetSessionError { .. } => 5,
        ShellCommand::SetAppLabelOverride { .. } => 6,
        ShellCommand::SetActionPaneFolded { .. } => 7,
        ShellCommand::SetActionPaneExpanded { .. } => 8,
        ShellCommand::StageActionArg { .. } => 9,
        ShellCommand::ResetActionArgs { .. } => 10,
        ShellCommand::SetActiveUtility { .. } => 11,
        ShellCommand::SetActiveTool { .. } => 12,
        ShellCommand::SetCommandExpanded { .. } => 13,
        ShellCommand::StageCommandArg { .. } => 14,
        ShellCommand::ResetCommandArgs { .. } => 15,
        ShellCommand::SetPanelVisible { .. } => 16,
        ShellCommand::SetPanelSize { .. } => 17,
        ShellCommand::SetPanelPath { .. } => 18,
        ShellCommand::SetDockOverride { .. } => 19,
        ShellCommand::SetPanelPathMemory { .. } => 20,
        ShellCommand::SetTreeOpenState { .. } => 21,
        ShellCommand::HydrateDockUi { .. } => 22,
        ShellCommand::ResetDock => 23,
        ShellCommand::FocusWindow { .. } => 24,
        ShellCommand::SetShellLayout { .. } => 25,
        ShellCommand::SetActiveExample { .. } => 26,
        ShellCommand::SetMobilePanelPath { .. } => 27,
        ShellCommand::SetMobilePanelVisible { .. } => 28,
        ShellCommand::SetExtraWindows { .. } => 29,
        ShellCommand::SetWindowTitle { .. } => 30,
        ShellCommand::SetWindowIcon { .. } => 31,
        ShellCommand::SetSearchOpen { .. } => 32,
        ShellCommand::SetFindOpen { .. } => 33,
        ShellCommand::AutoStartIntroduction { .. } => 34,
        ShellCommand::SetIntroductionStep { .. } => 35,
        ShellCommand::CompleteIntroductionInteraction { .. } => 36,
        ShellCommand::OpenDialog { .. } => 37,
        ShellCommand::CloseDialog { .. } => 38,
        ShellCommand::ShowTransientNotice { .. } => 39,
        ShellCommand::DismissTransientNotice => 40,
        ShellCommand::SetOpenWithFocusRole { .. } => 41,
        ShellCommand::SetActiveTutorial { .. } => 42,
        ShellCommand::SetUiAppearance { .. } => 43,
        ShellCommand::SetUiLayout { .. } => 44,
        ShellCommand::SetUiDriver { .. } => 45,
        ShellCommand::SetUiCustomDriver { .. } => 46,
        ShellCommand::SetUiDriverDraft { .. } => 47,
        ShellCommand::SetUiLocale { .. } => 48,
        ShellCommand::SetUiTerminology { .. } => 49,
        ShellCommand::SetUiTheme { .. } => 50,
        ShellCommand::SetUiCustomTheme { .. } => 51,
        ShellCommand::SetUiThemeDraft { .. } => 52,
        ShellCommand::SetUiKeybindingOverride { .. } => 53,
        ShellCommand::SetSyncBackboneUri { .. } => 54,
        ShellCommand::SetSyncCardKind { .. } => 55,
        ShellCommand::SetSyncDraftPath { .. } => 56,
        ShellCommand::SetDocumentSyncStatus { .. } => 57,
        ShellCommand::SetDocumentInferencePort { .. } => 58,
        ShellCommand::ClearDocumentInferencePort { .. } => 59,
        ShellCommand::SetMergePolicy { .. } => 60,
        ShellCommand::SetConflicts { .. } => 61,
        ShellCommand::SelectConflict { .. } => 62,
        ShellCommand::SetStorageScope { .. } => 63,
        ShellCommand::SetOpeningPreference { .. } => 64,
    };
    SHELL_COMMAND_CATALOG[index].id.to_string()
}

/// 🗂️ One entry per [`ShellCommand`] variant, in declaration order. `id`s reuse the wgpu shell's
/// existing `shell.*` verb strings where one already exists for this exact mutation (see
/// `📓️terra-P9-report.md`), and are coined fresh in the same dotted-noun style otherwise.
const SHELL_COMMAND_CATALOG: [CommandMeta; 65] = [
    CommandMeta { id: "plugin.register", title: "Register Loaded Plugin", description: "Add or replace a plugin's registry entry.", observable_only: false },
    CommandMeta { id: "plugin.unregister", title: "Unregister Loaded Plugin", description: "Remove a plugin's registry entry.", observable_only: false },
    CommandMeta { id: "plugin.setStatus", title: "Set Plugin Status", description: "Set a plugin panel's open/collapsed/error status.", observable_only: false },
    CommandMeta { id: "plugin.setSupervisorState", title: "Set Plugin Supervisor State", description: "Update a plugin's resource/failure monitoring summary.", observable_only: false },
    CommandMeta { id: "plugin.setActiveSession", title: "Set Active Session", description: "Switch the active app instance binding.", observable_only: false },
    CommandMeta { id: "plugin.setSessionError", title: "Set Session Error", description: "Set or clear the top-level shell error message.", observable_only: false },
    CommandMeta { id: "ui.app.setLabelOverride", title: "Set App Label Override", description: "Set or clear one app-specific label customization.", observable_only: false },
    CommandMeta { id: "shell.action.fold", title: "Set Action Pane Folded", description: "Collapse or expand a window's action rail.", observable_only: false },
    CommandMeta { id: "shell.action.expand", title: "Set Action Pane Expanded", description: "Open or close a window's action arg form.", observable_only: false },
    CommandMeta { id: "shell.action.stageArg", title: "Stage Action Arg", description: "Buffer one action argument value.", observable_only: false },
    CommandMeta { id: "shell.action.reset", title: "Reset Action Args", description: "Clear all staged args for one action.", observable_only: false },
    CommandMeta { id: "ui.window.setActiveUtility", title: "Set Active Utility", description: "Switch a window's host-owned active utility.", observable_only: false },
    CommandMeta { id: "ui.tool.setActive", title: "Set Active Tool", description: "Switch the host-owned active mode tool.", observable_only: false },
    CommandMeta { id: "ui.command.expand", title: "Set Command Expanded", description: "Open or close a command palette entry's arg form.", observable_only: false },
    CommandMeta { id: "ui.command.stageArg", title: "Stage Command Arg", description: "Buffer one command palette argument value.", observable_only: false },
    CommandMeta { id: "ui.command.reset", title: "Reset Command Args", description: "Clear all staged args for one command.", observable_only: false },
    CommandMeta { id: "shell.panelToggle", title: "Set Panel Visible", description: "Show or hide a docked panel.", observable_only: false },
    CommandMeta { id: "ui.panel.setSize", title: "Set Panel Size", description: "Resize a docked panel.", observable_only: false },
    CommandMeta { id: "shell.panel.tab", title: "Set Panel Path", description: "Navigate a docked panel's breadcrumb.", observable_only: false },
    CommandMeta { id: "ui.dock.setOverride", title: "Set Dock Override", description: "Persist a user dock rearrangement.", observable_only: false },
    CommandMeta { id: "ui.panel.setPathMemory", title: "Set Panel Path Memory", description: "Remember (or forget) a drill-down tab per panel.", observable_only: false },
    CommandMeta { id: "ui.tree.setOpenState", title: "Set Tree Open State", description: "Expand or collapse a tree section.", observable_only: false },
    CommandMeta { id: "ui.dock.hydrate", title: "Hydrate Dock UI", description: "Restore dock layout and panel visibility from a snapshot.", observable_only: false },
    CommandMeta { id: "ui.dock.reset", title: "Reset Dock", description: "Clear the dock override, returning to the default layout.", observable_only: false },
    CommandMeta { id: "ui.window.focus", title: "Focus Window", description: "Change the focused window instance.", observable_only: false },
    CommandMeta { id: "ui.shell.setLayout", title: "Set Shell Layout", description: "Change the window split/stack arrangement.", observable_only: false },
    CommandMeta { id: "ui.example.setActive", title: "Set Active Example", description: "Switch the active catalog example.", observable_only: false },
    CommandMeta { id: "ui.mobile.setPanelPath", title: "Set Mobile Panel Path", description: "Change the mobile panel breadcrumb.", observable_only: false },
    CommandMeta { id: "ui.mobile.setPanelVisible", title: "Set Mobile Panel Visible", description: "Show or hide the mobile panel.", observable_only: false },
    CommandMeta { id: "ui.window.setExtraWindows", title: "Set Extra Windows", description: "Replace the spawned extra window list.", observable_only: false },
    CommandMeta { id: "ui.window.setTitle", title: "Set Window Title", description: "Set a live window title override.", observable_only: false },
    CommandMeta { id: "ui.window.setIcon", title: "Set Window Icon", description: "Set a live window icon override.", observable_only: false },
    CommandMeta { id: "ui.search.setOpen", title: "Set Search Open", description: "Show or hide the global search panel.", observable_only: false },
    CommandMeta { id: "ui.find.setOpen", title: "Set Find Open", description: "Show or hide the find-in-window panel.", observable_only: false },
    CommandMeta { id: "ui.introduction.autoStart", title: "Auto-Start Introduction", description: "Mark a walkthrough as auto-started this session.", observable_only: false },
    CommandMeta { id: "ui.introduction.setStep", title: "Set Introduction Step", description: "Change the walkthrough step index.", observable_only: false },
    CommandMeta { id: "ui.introduction.completeInteraction", title: "Complete Introduction Interaction", description: "Mark one walkthrough step interaction done.", observable_only: false },
    CommandMeta { id: "ui.dialog.open", title: "Open Dialog", description: "Push a dialog onto the open-dialog stack.", observable_only: false },
    CommandMeta { id: "ui.dialog.close", title: "Close Dialog", description: "Pop a dialog off the open-dialog stack (top, or by id).", observable_only: false },
    CommandMeta { id: "ui.notice.show", title: "Show Transient Notice", description: "Show a non-blocking auto-dismiss notice.", observable_only: false },
    CommandMeta { id: "ui.notice.dismiss", title: "Dismiss Transient Notice", description: "Dismiss the current transient notice.", observable_only: false },
    CommandMeta { id: "ui.open.setFocusRole", title: "Set Open-With Focus Role", description: "Set which role group the Open panel focuses.", observable_only: false },
    CommandMeta { id: "ui.tutorial.setActive", title: "Set Active Tutorial", description: "Switch the active video tutorial.", observable_only: false },
    CommandMeta { id: "os.setAppearance", title: "Set Appearance", description: "Set light/dark/system appearance.", observable_only: false },
    CommandMeta { id: "os.setDriver", title: "Set UI Layout", description: "Set default/compact UI chrome density.", observable_only: false },
    CommandMeta { id: "ui.driver.setActive", title: "Set UI Driver", description: "Select the active UI driver.", observable_only: false },
    CommandMeta { id: "ui.driver.setCustom", title: "Set Custom UI Driver", description: "Add, replace, or remove a user-defined UI driver.", observable_only: false },
    CommandMeta { id: "ui.driver.setDraft", title: "Set UI Driver Draft", description: "Set or clear the in-progress driver editor draft.", observable_only: false },
    CommandMeta { id: "os.setLocale", title: "Set UI Locale", description: "Set the interface language.", observable_only: false },
    CommandMeta { id: "os.setTerminology", title: "Set UI Terminology", description: "Set the app-specific terminology id.", observable_only: false },
    CommandMeta { id: "os.setThemeId", title: "Set UI Theme", description: "Select the active theme.", observable_only: false },
    CommandMeta { id: "ui.theme.setCustom", title: "Set Custom UI Theme", description: "Add, replace, or remove a user-defined theme.", observable_only: false },
    CommandMeta { id: "ui.theme.setDraft", title: "Set UI Theme Draft", description: "Set or clear the in-progress theme editor draft.", observable_only: false },
    CommandMeta { id: "ui.keybinding.setOverride", title: "Set Keybinding Override", description: "Set or clear a user keybinding customization.", observable_only: false },
    CommandMeta { id: "sync.setBackboneUri", title: "Set Sync Backbone URI", description: "Set or clear the hub document sync backbone URI.", observable_only: false },
    CommandMeta { id: "sync.setCardKind", title: "Set Sync Card Kind", description: "Set or clear the check-in target type.", observable_only: false },
    CommandMeta { id: "sync.setDraftPath", title: "Set Sync Draft Path", description: "Set the work-in-progress check-in path.", observable_only: false },
    CommandMeta { id: "sync.setDocumentStatus", title: "Set Document Sync Status", description: "Set one document's sync health.", observable_only: false },
    CommandMeta { id: "inference.setDocumentPort", title: "Set Document Inference Port", description: "Replace one document's ephemeral inference-port state.", observable_only: false },
    CommandMeta { id: "inference.clearDocumentPort", title: "Clear Document Inference Port", description: "Retire one document's ephemeral inference-port state.", observable_only: false },
    CommandMeta { id: "merge.setPolicy", title: "Set Merge Policy", description: "Set the conflict resolution strategy.", observable_only: false },
    CommandMeta { id: "merge.setConflicts", title: "Set Conflicts", description: "Replace the open conflict roster.", observable_only: false },
    CommandMeta { id: "merge.selectConflict", title: "Select Conflict", description: "Select a conflict for preview.", observable_only: false },
    CommandMeta { id: "host.setStorageScope", title: "Set Storage Scope", description: "Select the shell state storage backend.", observable_only: false },
    CommandMeta { id: "host.setOpeningPreference", title: "Set Opening Preference", description: "Set or clear the default app/dialect for a role.", observable_only: false },
];

/// 🛰️ Every [`ShellCommand`] variant's machine-readable descriptor, in declaration order.
pub fn shell_capabilities() -> Vec<ShellCapability> {
    let root_schema = schemars::schema_for!(ShellCommand);
    let root_value = serde_json::to_value(&root_schema).unwrap_or(serde_json::Value::Null);
    let one_of = root_value.get("oneOf").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    SHELL_COMMAND_CATALOG
        .iter()
        .enumerate()
        .map(|(index, meta)| ShellCapability {
            id: meta.id.to_string(),
            title: meta.title.to_string(),
            description: meta.description.to_string(),
            schema: one_of.get(index).cloned().unwrap_or(serde_json::Value::Null),
            observable_only: meta.observable_only,
        })
        .collect()
}
//#endregion 🛰️ShellCapability

//#region 🧪️tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️tests
