/** 🎛️ Shell actions: the closed union of every action the shell reducers accept. */
import { type Anchor, type ElementsSurfaceAppearance, type IconName, type UiChromeLayout, type UiDriver, type UiLocale, type UiTheme, type WindowLayoutNode } from "@semio-tech/ui-react";
import { type AppRole, type Conflict, type ConflictId, type DockSkeleton, type DockUiState, type MergePolicy, type AppCatalogue, type PluginAppLabelsOverlay, type BuiltNode, type WindowEngagement, type WindowMeasure } from "@semio-tech/framework";
import { type ArtifactSyncStatus } from "@semio-tech/framework/backbone";
import { type InstalledServiceStatusV1 } from "@semio-tech/framework/inference/service";
import type { WindowFault } from "../../🏛️ShellHost/🩺️fault/🟦️.ts";
import type { UiDocumentStore } from "../../📃️UiDocumentStore/🟦️.tsx";
import type { ActiveSession, LoadedProgramState, PluginPanelStatus, PluginSupervisorState } from "../📦️program-sessions/🟦️.ts";
import type { SyncCardKind } from "../👥️peer-interaction/🟦️.ts";
import type { ExtraWindowInstance, OverlayState, TransientNotice, TutorialShellUiSnapshot, Updatable } from "../🗃️state-shapes/🟦️.ts";

//#region 🧮️ShellStore
//#region actions
export type ShellAction =
  | { readonly type: "UPSERT_LOADED_PLUGIN"; readonly value: LoadedProgramState }
  | { readonly type: "REMOVE_LOADED_PLUGIN"; readonly pluginId: string }
  | { readonly type: "SET_PLUGIN_STATUS"; readonly pluginId: string; readonly value: PluginPanelStatus }
  | { readonly type: "SET_PLUGIN_SUPERVISOR"; readonly pluginId: string; readonly value: PluginSupervisorState }
  | { readonly type: "SET_SESSION"; readonly value: Updatable<ActiveSession | null> }
  | { readonly type: "SET_ERROR"; readonly value: Updatable<string | null>; readonly fault?: WindowFault | null }
  | { readonly type: "SET_INSTANCE_FAULT"; readonly value: WindowFault | null }
  | { readonly type: "SET_WINDOW_UI_BY_WINDOW_ID"; readonly value: Updatable<Readonly<Record<string, BuiltNode>>> }
  | { readonly type: "SET_WINDOW_ENGAGEMENTS_BY_WINDOW_ID"; readonly value: Updatable<Readonly<Record<string, WindowEngagement>>> }
  | { readonly type: "SET_WINDOW_MEASURES_BY_WINDOW_ID"; readonly value: Updatable<Readonly<Record<string, readonly WindowMeasure[]>>> }
  | { readonly type: "SET_PANEL_BODY_STORE_BY_KEY"; readonly value: Updatable<Readonly<Record<string, UiDocumentStore>>> }
  | { readonly type: "SET_APP_LABELS_OVERLAY"; readonly value: Updatable<PluginAppLabelsOverlay> }
  | { readonly type: "SET_APP_CATALOGUE"; readonly value: Updatable<AppCatalogue> }
  | { readonly type: "SET_SPAWNED_WINDOW_ACTIVITY"; readonly value: Updatable<Readonly<Record<string, BuiltNode["activity"]>>>; readonly fault?: WindowFault | null }
  | { readonly type: "SET_SPAWNED_WINDOW_ENGAGEMENTS"; readonly value: Updatable<Readonly<Record<string, WindowEngagement>>> }
  | { readonly type: "SET_SPAWNED_WINDOW_MEASURES"; readonly value: Updatable<Readonly<Record<string, readonly WindowMeasure[]>>> }
  | { readonly type: "SET_ACTION_PANE_FOLDED"; readonly windowId: string; readonly value: boolean }
  | { readonly type: "SET_ACTION_PANE_EXPANDED"; readonly windowId: string; readonly value: string | null }
  | { readonly type: "STAGE_ACTION_ARG"; readonly windowId: string; readonly actionId: string; readonly argId: string; readonly value: unknown }
  | { readonly type: "RESET_ACTION_ARGS"; readonly windowId: string; readonly actionId: string }
  | { readonly type: "SET_ACTIVE_UTILITY"; readonly windowId: string; readonly utilityId: string | null }
  | { readonly type: "SET_ACTIVE_TOOL"; readonly toolId: string | null }
  | { readonly type: "SET_TOOL_MEASURES_BY_TOOL_ID"; readonly value: Updatable<Readonly<Record<string, readonly WindowMeasure[]>>> }
  | { readonly type: "SET_COMMAND_EXPANDED"; readonly value: string | null }
  | { readonly type: "STAGE_COMMAND_ARG"; readonly commandId: string; readonly argId: string; readonly value: unknown }
  | { readonly type: "RESET_COMMAND_ARGS"; readonly commandId: string }
  | { readonly type: "SET_PANEL_VISIBLE"; readonly anchor: Anchor; readonly value: Updatable<boolean> }
  | { readonly type: "SET_PANEL_SIZE"; readonly anchor: Anchor; readonly value: Updatable<number> }
  | { readonly type: "SET_PANEL_PATH"; readonly anchor: Anchor; readonly value: Updatable<readonly string[]> }
  | { readonly type: "SET_DOCK_OVERRIDE"; readonly value: DockSkeleton | null }
  | { readonly type: "SET_PANEL_PATH_MEMORY"; readonly value: Updatable<Readonly<Record<string, string>>> }
  | { readonly type: "SET_TREE_OPEN_STATE"; readonly id: string; readonly open: boolean }
  | { readonly type: "HYDRATE_DOCK_UI"; readonly value: DockUiState | null }
  | { readonly type: "RESET_DOCK" }
  | { readonly type: "SET_ACTIVE_WINDOW_ID"; readonly value: Updatable<string | null> }
  | { readonly type: "SET_SHELL_LAYOUT"; readonly value: Updatable<WindowLayoutNode | null> }
  | { readonly type: "SET_ACTIVE_EXAMPLE_ID"; readonly value: Updatable<string> }
  | { readonly type: "SET_MOBILE_PANEL_PATH"; readonly value: Updatable<readonly string[]> }
  | { readonly type: "SET_MOBILE_PANEL_VISIBLE"; readonly value: Updatable<boolean> }
  | { readonly type: "SET_EXTRA_WINDOW_INSTANCES"; readonly value: Updatable<readonly ExtraWindowInstance[]> }
  | { readonly type: "SET_WINDOW_TITLE"; readonly windowId: string; readonly title: string }
  | { readonly type: "SET_WINDOW_ICON"; readonly windowId: string; readonly iconId: IconName }
  | { readonly type: "SET_SEARCH_OPEN"; readonly value: Updatable<boolean> }
  | { readonly type: "SET_FIND_OPEN"; readonly value: Updatable<boolean> }
  | { readonly type: "AUTO_START_INTRODUCTION"; readonly key: string }
  | { readonly type: "SET_INTRODUCTION_STEP"; readonly value: Updatable<number | null> }
  | { readonly type: "COMPLETE_INTRODUCTION_INTERACTION"; readonly index: number }
  | { readonly type: "SET_DIALOG"; readonly value: OverlayState["dialog"] }
  | { readonly type: "CLOSE_DIALOG"; readonly openingId: number }
  | { readonly type: "SET_TRANSIENT_NOTICE"; readonly value: TransientNotice | null }
  | { readonly type: "SET_OPEN_WITH_FOCUS_ROLE"; readonly value: AppRole | null }
  | { readonly type: "SET_TUTORIAL"; readonly value: string | null }
  | { readonly type: "SET_TUTORIAL_PLAYING"; readonly value: Updatable<boolean> }
  | { readonly type: "SET_TUTORIAL_RATE"; readonly value: number }
  | { readonly type: "SET_TUTORIAL_MUTED"; readonly value: Updatable<boolean> }
  | { readonly type: "SET_TUTORIAL_CAPTIONS"; readonly value: Updatable<boolean> }
  | { readonly type: "SET_TUTORIAL_RECORDING"; readonly value: boolean }
  | { readonly type: "SET_TUTORIAL_DEVIATED"; readonly value: boolean }
  | { readonly type: "APPLY_TUTORIAL_UI_SNAPSHOT"; readonly snapshot: TutorialShellUiSnapshot }
  | { readonly type: "SET_UI_APPEARANCE"; readonly value: Updatable<ElementsSurfaceAppearance> }
  | { readonly type: "SET_UI_LAYOUT"; readonly value: Updatable<UiChromeLayout> }
  | { readonly type: "SET_UI_DRIVER_ID"; readonly value: Updatable<string> }
  | { readonly type: "SET_UI_CUSTOM_DRIVERS"; readonly value: Updatable<Record<string, UiDriver>> }
  | { readonly type: "SET_UI_DRIVER_DRAFT"; readonly value: Updatable<UiDriver | null> }
  | { readonly type: "SET_UI_LOCALE"; readonly value: Updatable<UiLocale> }
  | { readonly type: "SET_UI_TERMINOLOGY"; readonly value: Updatable<string> }
  | { readonly type: "SET_UI_THEME_ID"; readonly value: Updatable<string> }
  | { readonly type: "SET_UI_CUSTOM_THEMES"; readonly value: Updatable<Record<string, UiTheme>> }
  | { readonly type: "SET_UI_THEME_DRAFT"; readonly value: Updatable<UiTheme | null> }
  | { readonly type: "SET_UI_KEYBINDING_OVERRIDES"; readonly value: Updatable<Record<string, string>> }
  | { readonly type: "SET_SYNC_BACKBONE_URI"; readonly value: Updatable<string | null> }
  | { readonly type: "SET_SYNC_CARD_KIND"; readonly value: Updatable<SyncCardKind | null> }
  | { readonly type: "SET_SYNC_DRAFT_PATH"; readonly value: Updatable<string> }
  | { readonly type: "SET_SYNC_STATUS_FOR_DOCUMENT"; readonly documentId: string; readonly status: ArtifactSyncStatus }
  | { readonly type: "OPEN_INFERENCE_PORT"; readonly runtimeKey: string; readonly operationEpoch: number }
  | { readonly type: "SET_INFERENCE_PORT_FOR_DOCUMENT"; readonly runtimeKey: string; readonly status: InstalledServiceStatusV1 }
  | { readonly type: "CLEAR_INFERENCE_PORT_FOR_DOCUMENT"; readonly runtimeKey: string }
  | { readonly type: "SET_MERGE_POLICY"; readonly value: MergePolicy }
  | { readonly type: "SET_CONFLICTS"; readonly value: Updatable<readonly Conflict[]> }
  | { readonly type: "SET_SELECTED_CONFLICT_ID"; readonly value: ConflictId | null };
//#endregion actions
//#endregion 🧮️ShellStore
