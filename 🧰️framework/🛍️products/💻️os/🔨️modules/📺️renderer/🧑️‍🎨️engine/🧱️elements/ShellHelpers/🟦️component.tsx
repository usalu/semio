// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/ShellHelpers/component.tsx
/** @emoji 🧰️ `ShellHelpers` — shared plumbing behind the framework OS shell orchestrator
 * ({@link ../ShellHost}): action-history/reserved-id bookkeeping, presence identity, UI history,
 * media-export download helpers, `requestMediaFrames`'s WebCodecs/`<video>` tiered decode pipeline,
 * window-layout-change classification, the utility-tree/command/tool registries, the tutorial UI
 * bridge, reveal-cutoff store, the window action pane, and the plugin UI-refresh cache. No single
 * exported component here — a grab bag of the functions/types `ShellHost` and sibling elements need.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import React, {
  type KeyboardEvent,
  type ReactElement,
  type ReactNode,
  useCallback,
  useEffect,
  useMemo,
  useState,
} from "react";
import {
  isIconName,
} from "@semio-tech/assets";
import {
  type ActionArgControl,
  type ActionArgDef,
  // 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P3-manifest-schema, D6:
  // `ActionArgDef.control` is gone (derived, not stored) — every reader below now calls this instead.
  argControl,
  type ActionDefinition,
  type ActionDescriptor,
  type ActionInvocation,
  type AppDefinition,
  type AppModeDefinition,
  type AppPanelTabDefinition,
  type AppRef,
  type AppRole,
  type AppRouter,
  type AppWindowKindDefinition,
  type ArtifactDialect,
  dialectCoordinate,
  type CommandAddress,
  type CommandDefinition,
  type CommandInvocation,
  type DerivedUtilitySpec,
  deriveUtilityNodes,
  type DialogDefinition,
  DockLayoutStore,
  DockUiStateStore,
  effectiveActionArgs,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
  FRAMEWORK_PANEL_TAB_ARTIFACT_ICON_ID,
  FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
  FRAMEWORK_PANEL_TAB_HISTORY_ID,
  FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID,
  FRAMEWORK_PANEL_TAB_INSPECTION_ID,
  FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID,
  FRAMEWORK_PANEL_TAB_PARAMETERS_ID,
  type Effect,
  type IntroductionDefinition,
  type IntroductionStepDefinition,
  type LocalizedLabel,
  type MergeMode,
  missingRequiredArgs,
  type PanelTabKind,
  panelTabKindId,
  partitionWindowMeasures,
  pendingPanelUiNode,
  type PluginAppLabelsOverlay,
  type PluginCatalog,
  type PluginUiRefreshRequest,
  type PluginUiRefreshResponse,
  type PluginUiRefreshSectionResponse,
  type PluginViewState,
  type Platform,
  RECORD_TUTORIAL_ACTION_ID,
  resolvePluginHostConfig,
  resolveUiDirtyScope,
  resolveWindowActions,
  SET_ACTIVE_TOOL_ACTION_ID,
  SET_ACTIVE_UTILITY_ACTION_ID,
  SHELL_LOCALES,
  START_INTRODUCTION_ACTION_ID,
  START_TUTORIAL_ACTION_ID,
  type ToolDefinition,
  type TutorialUiChange,
  type TutorialUiSnapshot,
  type UiControlNode,
  type UiDirtyScope,
  type UiNode,
  type UiTreeNode,
  type UtilityDefinition,
  type UtilityNode,
  type WindowEngagement,
  type WindowEngagementControl,
  type WindowLayout,
  type WindowLayoutAxisNode,
  type WindowLayoutStackNode,
  type WindowLayoutWindowNode,
  type WindowStackCorner,
  type WindowMeasure,
} from "@semio-tech/framework";
import {
  type ArtifactSyncStatus,
  packValueFromBase64,
  packValueToBase64,
} from "@semio-tech/framework-os";
import {
  decodeWorldProjectionTemplateId,
} from "@semio-tech/infinite-world-r3f";
import {
  type Anchor,
  ANCHORS,
  builtinUiDrivers,
  childElementId,
  ChromeAwareWindowScrollSurface,
  classifyIconSelectorMode,
  createEvenWindowLayout,
  elementIdSegment,
  type ElementsSurfaceAppearance,
  type EngagementControl,
  type EngagementSpec,
  Icon,
  type IconName,
  IconSelector,
  Input,
  type PanelTabNode,
  resolveTranslationLabel,
  RibbonDivider,
  type SearchSpec,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  setUiLocale,
  singleTreeLeaf,
  Slider,
  staticTreePanelDefinition,
  Toggle,
  ToggleGroup,
  Tree,
  TreeCheckbox,
  type TreeDataItem,
  type TreeDataSection,
  type TreePanelConfig,
  UI_RIBBON_PARENT_CATEGORIES,
  UI_TERMINOLOGY_NATIVE,
  type UiChromeLayout,
  type UiChromeTerminologyId,
  uiDataLabel,
  type UiDriver,
  uiI18n,
  type UiLabel,
  type UiLocale,
  type UiRibbonParentCategory,
  type UiTheme,
  type UiTranslationKey,
  useLabel,
  useShellScope,
  type WindowLayoutNode,
  WindowMeasuresTree,
  WindowMeasureTreeGroup,
  WindowMeasureTreeLeaf,
} from "@semio-tech/ui-react";
import {
  declarativeTreeDragController,
  InterpretedUiNode,
  interpretUiNode,
  renderUiControl,
  uiTreeNodeToTreePanelConfig,
  wireLabel,
} from "../Interpreter/🟦️component.tsx";
import {
  type ActionPaneState,
  actionStageKey,
  type ActiveSession,
  EMPTY_SHELL_LOCKS,
  type ExtraWindowInstance,
  type LoadedProgramState,
  type PluginManifest,
  type ResolvedShellLocks,
  type ShellAction,
  ShellFaultBoundary,
  type ShellState,
  type SpacePanelState,
  type SpaceProgramEntry,
  type SpawnedAppEntry,
  type UIHistory,
  type ViewModel,
} from "../Shell/🟦️component.tsx";
import {
  registerPendingWorldProjection,
  type WorldInstanceRecord,
} from "../World3dHost/🟦️component.tsx";
import { groupUtilityNodesByCategory, UTILITY_CATEGORIES, UtilityTree } from "../UtilityTree/🟦️component.tsx";
import { loadPluginModule, type PluginWasmHandle } from "../PluginRuntime/🟦️component.tsx";
// #endregion 🔌️Adapters

//#region ShellHelpers
export function syncDocumentId(session: ActiveSession, panel: SpacePanelState | null, hostMode: boolean): string {
  if (hostMode && panel?.activeSpawnedId) {
    const spawned = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
    if (spawned) return `${spawned.pluginId}-${spawned.instanceId}`;
  }
  return `${session.pluginId}-${session.instanceId}`;
}

/** @emoji ↔ Shared starting width for every panel anchor, one compact step wider than the former 280px Document panel. */
export const DEFAULT_PANEL_WIDTH_PX = 300;

/** @emoji 🌳️ Root category id for the nested dock tab tree — the top row of {@link defaultDock}'s bottom-left (Display) anchor tabs; top-left (Workbench), top-right (Details) and bottom-right (Settings) render their tabs flat instead of under a category branch. */
export const FRAMEWORK_CATEGORY_DISPLAY_ID = "framework.category.display";
/** @emoji 🎛️ Root category id bundling every command-category leaf under one expandable Command toggle on bottom-middle (mirrors Display on bottom-left). */
export const FRAMEWORK_CATEGORY_COMMAND_ID = "framework.category.command";
/** @emoji 🛠️ Root category id bundling every mode-level tool leaf under one expandable Tool toggle on
 * bottom-middle, ordered left of the Command branch (mirrors Command's own bundling on the same anchor). */
export const FRAMEWORK_CATEGORY_TOOL_ID = "framework.category.tool";

/** @emoji 🎛️ Corner/top-middle/bottom-middle anchors park their *folded* root tab row in navbar/footer chrome (via {@link PanelChromeTabBar}); while open, the floating {@link Panel} hosts the full strip on its {@link WindowChrome}. The two side-middle anchors have no navbar/footer slot, so they're absent here and fall back to `"panel"` (see the `?..:"panel"` read site), carrying their own tab bar when folded too. */
export const PANEL_TAB_BAR_HOSTS: Partial<Record<Anchor, "navbar" | "footer">> = {
  "top-left": "navbar",
  "top-middle": "navbar",
  "top-right": "navbar",
  "bottom-left": "footer",
  "bottom-middle": "footer",
  "bottom-right": "footer",
};
const APP_BREADCRUMB_SEPARATOR = " · ";

/** 🧭️ Shell-only action id `World3dHost`'s `WorldOrbitGated.onNavigationGestures` dispatches through the
 * standard `onAction` funnel to report a completed pan/zoom/orbit gesture — intercepted in `onAction`
 * (never forwarded to the program), args `{ windowId: string, gestures: readonly string[] }`. */
export const NOTE_WORLD_NAVIGATION_ACTION_ID = "noteWorldNavigation";

/** 🧭️ Framework-injected action id, dispatched via `noteShellCommand` (see `onAction`'s central funnel) to
 * log a shell-chrome command (theme/appearance/locale/driver/layout change, dock drag, window
 * resize/rearrange/activate/close/split, panel toggle/tab) into the plugin's session-only command-history
 * panel — intercepted by the plugin BEFORE the app ever sees it, args `{ commandId: string, label: string,
 * detail?: unknown }`. Routed through the exact same `handleAction` dispatch path as every other action
 * (unlike {@link NOTE_WORLD_NAVIGATION_ACTION_ID}, which is fully shell-intercepted and never forwarded). */
const NOTE_SHELL_COMMAND_ACTION_ID = "noteShellCommand";

/** 🛡️ Action ids intercepted by `VcsDocumentApp::dispatch_action` before `command_from_action` — undeclared
 * surface verbs (e.g. VFS `selectRows` on Home) must not be forwarded or they hard-error the bridge. */
export const FRAMEWORK_RESERVED_ACTION_IDS: ReadonlySet<string> = new Set([
  "undo",
  "redo",
  "commitCheckpoint",
  "createAlternative",
  "switchAlternative",
  "checkoutCheckpoint",
  "copy",
  "cut",
  "paste",
  "revertToCommand",
  "setHistoryCommandFilter",
  NOTE_SHELL_COMMAND_ACTION_ID,
  "recordTutorial",
  "startIntroduction",
  "startTutorial",
  "setActiveUtility",
  "setActiveTool",
  "suggestionsTick",
  "fillBuildTick",
]);

/** 🧭️ Builds the `noteShellCommand` action descriptor `noteShellCommand` (the component helper) dispatches
 * through the standard `onAction` funnel — pure so it's testable without a session/component. */
export function buildNoteShellCommandAction(controllerId: string, commandId: string, label: string, detail?: Record<string, unknown>): ActionDescriptor {
  return { controllerId, action: NOTE_SHELL_COMMAND_ACTION_ID, args: { commandId, label, ...(detail ? { detail } : {}) } };
}

/** 🧭️ Action ids the tutorial recorder never captures (see `onAction`'s recorder tap) — telemetry/chrome
 * noise a tutorial replay should never literally reproduce, or actions the director/recorder itself just
 * dispatched. Exported so it's independently testable. */
export const TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS: ReadonlySet<string> = new Set([NOTE_WORLD_NAVIGATION_ACTION_ID, NOTE_SHELL_COMMAND_ACTION_ID, START_INTRODUCTION_ACTION_ID, START_TUTORIAL_ACTION_ID, RECORD_TUTORIAL_ACTION_ID]);

export const PRESENCE_CLIENT_STORAGE_KEY = "semio.presence.client";
export const PRESENCE_HEARTBEAT_INTERVAL_MS = 5000;

function presenceIdentityPackBase64(identity: { readonly clientId: string; readonly name: string }): string {
  return packValueToBase64(identity);
}

function presenceIdentityFromPackBase64(encoded: string): { readonly clientId: string; readonly name: string } | null {
  try {
    const decoded = packValueFromBase64(encoded) as { readonly clientId?: string; readonly name?: string };
    if (decoded.clientId && decoded.name) return { clientId: decoded.clientId, name: decoded.name };
  } catch {
    return null;
  }
  return null;
}

/** 🪪️ `real`, when given, is the ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS
 * shell's resolved `Identity` presence pair (`user:{userId}#{shellSessionId}` + `displayName`) —
 * ShellHost's only caller passes it once sign-in resolves, so presence/heartbeat labels show the real
 * signed-in user instead of a random per-tab `Guest ####`. Absent (no hub env, offline, or not yet
 * resolved) falls back to the pre-existing session-storage-cached guest identity unchanged. */
export function presenceClientIdentity(ephemeral = false, real?: { readonly clientId: string; readonly name: string }): { readonly clientId: string; readonly name: string } {
  if (real) return real;
  if (typeof window === "undefined") return { clientId: "server", name: "Server" };
  if (!ephemeral) {
    const stored = window.sessionStorage.getItem(PRESENCE_CLIENT_STORAGE_KEY);
    if (stored) {
      const parsed = presenceIdentityFromPackBase64(stored);
      if (parsed) return parsed;
    }
  }
  const clientId = `client-${Math.random().toString(36).slice(2, 10)}`;
  const identity = { clientId, name: `Guest ${clientId.slice(-4).toUpperCase()}` };
  if (!ephemeral) window.sessionStorage.setItem(PRESENCE_CLIENT_STORAGE_KEY, presenceIdentityPackBase64(identity));
  return identity;
}

function readBrowserUri(): string {
  if (typeof window === "undefined") return "/";
  return `${window.location.pathname}${window.location.search}` || "/";
}

export function useUIHistory(initialUri = "/", syncBrowser = false) {
  const [history, setHistory] = useState<UIHistory>(() => ({
    entries: [{ uri: syncBrowser ? readBrowserUri() : initialUri }],
    index: 0,
  }));
  const uri = history.entries[history.index]?.uri ?? initialUri;
  const canGoBack = history.index > 0;
  const canGoForward = history.index < history.entries.length - 1;
  const segments = uri.split("/").filter(Boolean);
  const canGoUp = segments.length > 0;
  const parentUri = canGoUp ? `/${segments.slice(0, -1).join("/")}` : null;

  const goBack = useCallback(() => {
    setHistory((prev) => (prev.index > 0 ? { ...prev, index: prev.index - 1 } : prev));
  }, []);
  const goForward = useCallback(() => {
    setHistory((prev) => (prev.index < prev.entries.length - 1 ? { ...prev, index: prev.index + 1 } : prev));
  }, []);
  const goUp = useCallback(() => {
    if (!canGoUp || parentUri === null) return;
    setHistory((prev) => {
      const newEntries = prev.entries.slice(0, prev.index + 1);
      return { entries: [...newEntries, { uri: parentUri }], index: newEntries.length };
    });
  }, [canGoUp, parentUri]);
  const navigate = useCallback((targetUri: string) => {
    setHistory((prev) => {
      const existingIndex = prev.entries.findIndex((entry) => entry.uri === targetUri);
      if (existingIndex >= 0) return { ...prev, index: existingIndex };
      const newEntries = prev.entries.slice(0, prev.index + 1);
      return { entries: [...newEntries, { uri: targetUri }], index: newEntries.length };
    });
  }, []);
  const syncUri = useCallback((targetUri: string) => {
    setHistory((prev) => {
      const existingIndex = prev.entries.findIndex((entry) => entry.uri === targetUri);
      if (existingIndex >= 0) return { ...prev, index: existingIndex };
      const newEntries = prev.entries.slice(0, prev.index + 1);
      return { entries: [...newEntries, { uri: targetUri }], index: newEntries.length };
    });
  }, []);

  useEffect(() => {
    if (!syncBrowser || typeof window === "undefined") return;
    const current = `${window.location.pathname}${window.location.search}`;
    if (current !== uri) window.history.pushState(null, "", uri);
  }, [syncBrowser, uri]);

  useEffect(() => {
    if (!syncBrowser || typeof window === "undefined") return;
    const onPopState = () => syncUri(readBrowserUri());
    window.addEventListener("popstate", onPopState);
    return () => window.removeEventListener("popstate", onPopState);
  }, [syncBrowser, syncUri]);

  return { uri, canGoBack, canGoForward, canGoUp, parentUri, goBack, goForward, goUp, navigate, syncUri };
}

export function downloadMediaExport(filename: string, mimeType: string, data: string, encoding?: string): void {
  if (typeof document === "undefined") return;
  const payload = encoding === "base64" ? Uint8Array.from(atob(data), (char) => char.charCodeAt(0)) : data;
  const blob = new Blob([payload], { type: mimeType });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = filename;
  anchor.click();
  URL.revokeObjectURL(url);
}

export function downloadDataUrl(filename: string, dataUrl: string): void {
  if (typeof document === "undefined") return;
  const anchor = document.createElement("a");
  anchor.href = dataUrl;
  anchor.download = filename;
  anchor.click();
}

/** 📤️ Opens the native file picker. Resolves with one entry per selected file, in selection order —
 * always an array (empty on cancel) so single-file callers just read `[0]` and `multiple` callers can
 * fan out over the whole list; single-file behavior (one `<input>`, one resolved entry) is unchanged
 * when `multiple` is false/absent. */
export function requestFileOpen(accept: string, readAs?: string, multiple?: boolean): Promise<readonly { contents: string; name: string }[]> {
  if (typeof document === "undefined") return Promise.resolve([]);
  return new Promise((resolve) => {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = accept;
    if (multiple) input.multiple = true;
    input.onchange = async () => {
      const files = input.files ? Array.from(input.files) : [];
      if (files.length === 0) {
        resolve([]);
        return;
      }
      const opened: { contents: string; name: string }[] = [];
      for (const file of files) {
        if (readAs === "dataUrl") {
          const contents = await new Promise<string | null>((resolveFile) => {
            const reader = new FileReader();
            reader.onload = () => resolveFile(typeof reader.result === "string" ? reader.result : null);
            reader.onerror = () => resolveFile(null);
            reader.readAsDataURL(file);
          });
          if (contents !== null) opened.push({ contents, name: file.name });
          continue;
        }
        opened.push({ contents: await file.text(), name: file.name });
      }
      resolve(opened);
    };
    input.click();
  });
}

/** 🔁️ The one-action-at-a-time callback shared by the `requestFileOpen`/`dispatchAction`/
 * `requestMediaFrames` `applyHostEffects` branches: dispatches `action` against the emitting program
 * instance and feeds its own `requestedEffects` back through `applyHostEffects` recursively. */
type EffectDispatchOne = (action: string, args?: Record<string, unknown>) => Promise<void>;

/** 🎯️ Encodes a host-effect callback as the same fully scoped JSON `ActionInvocation` every other
 * renderer action uses; pack-base64 belongs inside the worker channel, never at `handleAction`'s edge. */
export function encodeEffectActionInvocation(baseSession: ActiveSession, action: string, args?: Record<string, unknown>): string {
  const windowKindId = baseSession.viewState.activeWindowKindId ?? baseSession.app.windowKinds[0]?.id ?? "";
  const windowInstanceId = baseSession.viewState.windowId ?? windowKindId;
  const invocation: ActionInvocation = {
    address: {
      pluginId: baseSession.pluginId,
      appId: baseSession.app.id,
      modeId: baseSession.viewState.activeModeId ?? baseSession.app.defaultModeId ?? baseSession.app.modes[0]?.id ?? baseSession.app.id,
      windowKindId,
      windowInstanceId,
      actionId: action,
    },
    arguments: { ...args, windowId: windowInstanceId },
  };
  return JSON.stringify(invocation);
}

/** 🎯️ Encodes a recursively requested app command with its manifest owner. */
export function encodeEffectCommandInvocation(baseSession: ActiveSession, commandId: string, args?: Record<string, unknown>): string {
  const invocation: CommandInvocation = {
    address: { owner: { app: { pluginId: baseSession.pluginId, appId: baseSession.app.id } }, commandId },
    arguments: { ...args },
  };
  return JSON.stringify(invocation);
}

/** 🔁️ Builds an {@link EffectDispatchOne} bound to one plugin instance + `applyHostEffects` closure;
 * declared app commands re-enter the typed command channel and framework/window actions retain their
 * scoped action channel. */
export function makeEffectDispatchOne(
  pluginEntry: LoadedProgramState,
  baseSession: ActiveSession,
  applyEffects: (effects: readonly Effect[], baseSession: ActiveSession, uiScope?: UiDirtyScope) => Promise<void>,
): EffectDispatchOne {
  return async (action, args) => {
    const isAppCommand = (baseSession.app.commands ?? []).some((command) => command.id === action);
    const response = isAppCommand && pluginEntry.handle.handleCommand
      ? await pluginEntry.handle.handleCommand(baseSession.instanceId, encodeEffectCommandInvocation(baseSession, action, args), baseSession.viewState)
      : await pluginEntry.handle.handleAction(baseSession.instanceId, encodeEffectActionInvocation(baseSession, action, args), baseSession.viewState);
    await applyEffects(response.requestedEffects ?? [], baseSession, resolveUiDirtyScope(response.uiScope));
  };
}

/** 📤️ D3 fan-out: one {@link EffectDispatchOne} call per opened file — single-file behavior (`multiple`
 * absent/false, exactly one call, plain `{payload, name}`) is byte-for-byte what this loop always did
 * before `multiple` existed, since it's just a one-entry `opened` array through the same path. */
export async function dispatchOpenedFiles(
  opened: readonly { readonly contents: string; readonly name: string }[],
  importAction: string,
  multiple: boolean,
  dispatchOne: EffectDispatchOne,
): Promise<void> {
  const total = opened.length;
  for (let index = 0; index < opened.length; index += 1) {
    const file = opened[index]!;
    await dispatchOne(importAction, multiple ? { payload: file.contents, name: file.name, index, total } : { payload: file.contents, name: file.name });
  }
}

/** 🔁️ D2: schedules `action` onto `dispatchOne` after `delayMs` (0 = next tick) via `schedule` (real
 * callers pass `setTimeout`; tests pass `vi.useFakeTimers()`-driven `setTimeout` or a synchronous stub). */
export function scheduleDispatchAction(
  action: string,
  args: Record<string, unknown> | undefined,
  delayMs: number,
  dispatchOne: EffectDispatchOne,
  schedule: (fn: () => void, delayMs: number) => void = (fn, ms) => setTimeout(fn, ms),
): void {
  schedule(() => {
    void dispatchOne(action, args);
  }, delayMs);
}

//#region RequestMediaFrames
//#region Bmff
/** 🧱️ One parsed ISO-BMFF box: `[type, payloadStart, payloadEnd)` — enough to recurse into containers
 * and slice leaf payloads without copying. */
type BmffBox = { readonly type: string; readonly start: number; readonly end: number };

/** 🧱️ Walks sibling boxes in `[start, end)` — handles 64-bit extended sizes (`size===1`) and to-end
 * boxes (`size===0`); malformed/truncated input just stops early rather than throwing, since MP4
 * probing here is best-effort — the Tier-2 `<video>` fallback covers anything this can't parse. */
function walkBmffBoxes(view: DataView, start: number, end: number): BmffBox[] {
  const boxes: BmffBox[] = [];
  let offset = start;
  while (offset + 8 <= end) {
    const size32 = view.getUint32(offset);
    const type = String.fromCharCode(view.getUint8(offset + 4), view.getUint8(offset + 5), view.getUint8(offset + 6), view.getUint8(offset + 7));
    let headerSize = 8;
    let boxSize = size32;
    if (size32 === 1) {
      if (offset + 16 > end) break;
      boxSize = Number(view.getBigUint64(offset + 8));
      headerSize = 16;
    } else if (size32 === 0) {
      boxSize = end - offset;
    }
    if (boxSize < headerSize || offset + boxSize > end) break;
    boxes.push({ type, start: offset + headerSize, end: offset + boxSize });
    offset += boxSize;
  }
  return boxes;
}

function findBmffBox(boxes: readonly BmffBox[], type: string): BmffBox | undefined {
  return boxes.find((box) => box.type === type);
}
//#endregion Bmff

//#region Tier1
type Mp4Sample = { readonly offset: number; readonly size: number; readonly timestampMs: number; readonly isSync: boolean };
type Mp4Track = {
  readonly width: number;
  readonly height: number;
  readonly codec: "avc1" | "hvc1";
  readonly description: Uint8Array;
  readonly samples: readonly Mp4Sample[];
};

/** 🎞️ Minimal MP4 sample-table extraction — `moov > trak[] > mdia > {mdhd, hdlr, minf > stbl}` for the
 * first video track (`hdlr`'s handler-type `"vide"`), enough to feed `VideoDecoder`: sample byte ranges
 * from `stsc` + `stco`/`co64` + `stsz`, decode timestamps from `stts`, sync flags from `stss` (absent
 * `stss` ⇒ every sample is sync per spec), and the AVC/HEVC decoder config from `stsd`'s `avcC`/`hvcC`.
 * Returns `null` for anything unrecognized (non-AVC/HEVC, missing boxes, malformed tables) so the
 * caller falls back to Tier 2 rather than guessing. */
function probeMp4VideoTrack(bytes: Uint8Array): Mp4Track | null {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const moov = findBmffBox(walkBmffBoxes(view, 0, bytes.byteLength), "moov");
  if (!moov) return null;
  for (const trak of walkBmffBoxes(view, moov.start, moov.end).filter((box) => box.type === "trak")) {
    const mdia = findBmffBox(walkBmffBoxes(view, trak.start, trak.end), "mdia");
    if (!mdia) continue;
    const mdiaBoxes = walkBmffBoxes(view, mdia.start, mdia.end);
    const hdlr = findBmffBox(mdiaBoxes, "hdlr");
    if (!hdlr || hdlr.end - hdlr.start < 12) continue;
    const handlerType = String.fromCharCode(view.getUint8(hdlr.start + 8), view.getUint8(hdlr.start + 9), view.getUint8(hdlr.start + 10), view.getUint8(hdlr.start + 11));
    if (handlerType !== "vide") continue;
    const mdhd = findBmffBox(mdiaBoxes, "mdhd");
    const minf = findBmffBox(mdiaBoxes, "minf");
    if (!mdhd || !minf) continue;
    const timescale = view.getUint8(mdhd.start) === 1 ? view.getUint32(mdhd.start + 20) : view.getUint32(mdhd.start + 12);
    if (timescale <= 0) continue;
    const stbl = findBmffBox(walkBmffBoxes(view, minf.start, minf.end), "stbl");
    if (!stbl) continue;
    const track = probeSampleTable(view, walkBmffBoxes(view, stbl.start, stbl.end), timescale);
    if (track) return track;
  }
  return null;
}

function parseStsd(view: DataView, stsd: BmffBox): { width: number; height: number; codec: "avc1" | "hvc1"; description: Uint8Array } | null {
  if (view.getUint32(stsd.start + 4) < 1) return null;
  const entryOffset = stsd.start + 8;
  const entrySize = view.getUint32(entryOffset);
  const format = String.fromCharCode(
    view.getUint8(entryOffset + 4),
    view.getUint8(entryOffset + 5),
    view.getUint8(entryOffset + 6),
    view.getUint8(entryOffset + 7),
  );
  if (format !== "avc1" && format !== "hvc1" && format !== "hev1") return null;
  const codec = format === "avc1" ? "avc1" : "hvc1";
  const visualEntryStart = entryOffset + 8;
  const width = view.getUint16(visualEntryStart + 24);
  const height = view.getUint16(visualEntryStart + 26);
  const inner = walkBmffBoxes(view, visualEntryStart + 78, entryOffset + entrySize);
  const config = findBmffBox(inner, codec === "avc1" ? "avcC" : "hvcC");
  if (!config) return null;
  return { width, height, codec, description: new Uint8Array(view.buffer.slice(config.start, config.end)) };
}

function parseStsz(view: DataView, box: BmffBox): number[] {
  const uniformSize = view.getUint32(box.start + 4);
  const sampleCount = view.getUint32(box.start + 8);
  if (uniformSize !== 0) return new Array(sampleCount).fill(uniformSize) as number[];
  const sizes: number[] = [];
  for (let i = 0; i < sampleCount; i += 1) sizes.push(view.getUint32(box.start + 12 + i * 4));
  return sizes;
}

function parseChunkOffsets(view: DataView, box: BmffBox, is64: boolean): number[] {
  const count = view.getUint32(box.start + 4);
  const offsets: number[] = [];
  for (let i = 0; i < count; i += 1) {
    offsets.push(is64 ? Number(view.getBigUint64(box.start + 8 + i * 8)) : view.getUint32(box.start + 8 + i * 4));
  }
  return offsets;
}

function parseChunkOfSample(view: DataView, box: BmffBox, sampleCount: number, chunkCount: number): number[] | null {
  const entryCount = view.getUint32(box.start + 4);
  const entries: { firstChunk: number; samplesPerChunk: number }[] = [];
  for (let i = 0; i < entryCount; i += 1) {
    entries.push({ firstChunk: view.getUint32(box.start + 8 + i * 12), samplesPerChunk: view.getUint32(box.start + 12 + i * 12) });
  }
  const chunkOfSample: number[] = [];
  for (let entryIndex = 0; entryIndex < entries.length; entryIndex += 1) {
    const entry = entries[entryIndex]!;
    const nextFirstChunk = entries[entryIndex + 1]?.firstChunk ?? chunkCount + 1;
    for (let chunk = entry.firstChunk; chunk < nextFirstChunk; chunk += 1) {
      for (let inChunk = 0; inChunk < entry.samplesPerChunk; inChunk += 1) chunkOfSample.push(chunk);
    }
  }
  return chunkOfSample.length >= sampleCount ? chunkOfSample : null;
}

function computeSampleOffsets(chunkOfSample: readonly number[], chunkOffsets: readonly number[], sizes: readonly number[]): number[] {
  const offsets: number[] = [];
  const cursorByChunk = new Map<number, number>();
  for (let i = 0; i < sizes.length; i += 1) {
    const chunk = chunkOfSample[i]!;
    const base = cursorByChunk.get(chunk) ?? chunkOffsets[chunk - 1] ?? 0;
    offsets.push(base);
    cursorByChunk.set(chunk, base + sizes[i]!);
  }
  return offsets;
}

function accumulateTimestampsMs(view: DataView, stts: BmffBox, sampleCount: number, timescale: number): number[] {
  const entryCount = view.getUint32(stts.start + 4);
  const timestamps: number[] = [];
  let ticks = 0;
  for (let entryIndex = 0; entryIndex < entryCount && timestamps.length < sampleCount; entryIndex += 1) {
    const count = view.getUint32(stts.start + 8 + entryIndex * 8);
    const delta = view.getUint32(stts.start + 12 + entryIndex * 8);
    for (let i = 0; i < count && timestamps.length < sampleCount; i += 1) {
      timestamps.push((ticks / timescale) * 1000);
      ticks += delta;
    }
  }
  return timestamps;
}

function parseSyncSamples(view: DataView, box: BmffBox): Set<number> {
  const count = view.getUint32(box.start + 4);
  const sync = new Set<number>();
  for (let i = 0; i < count; i += 1) sync.add(view.getUint32(box.start + 8 + i * 4));
  return sync;
}

function probeSampleTable(view: DataView, stblBoxes: readonly BmffBox[], timescale: number): Mp4Track | null {
  const stsd = findBmffBox(stblBoxes, "stsd");
  const stts = findBmffBox(stblBoxes, "stts");
  const stsc = findBmffBox(stblBoxes, "stsc");
  const stsz = findBmffBox(stblBoxes, "stsz");
  const stco = findBmffBox(stblBoxes, "stco") ?? findBmffBox(stblBoxes, "co64");
  if (!stsd || !stts || !stsc || !stsz || !stco) return null;
  const entry = parseStsd(view, stsd);
  if (!entry) return null;
  const sizes = parseStsz(view, stsz);
  const offsets = parseChunkOffsets(view, stco, stco.type === "co64");
  const chunkOfSample = parseChunkOfSample(view, stsc, sizes.length, offsets.length);
  if (!chunkOfSample) return null;
  const sampleOffsets = computeSampleOffsets(chunkOfSample, offsets, sizes);
  const timestampsMs = accumulateTimestampsMs(view, stts, sizes.length, timescale);
  const stss = findBmffBox(stblBoxes, "stss");
  const syncSamples = stss ? parseSyncSamples(view, stss) : null;
  const samples: Mp4Sample[] = sizes.map((size, index) => ({
    offset: sampleOffsets[index]!,
    size,
    timestampMs: timestampsMs[index] ?? 0,
    isSync: syncSamples ? syncSamples.has(index + 1) : true,
  }));
  return { width: entry.width, height: entry.height, codec: entry.codec, description: entry.description, samples };
}

/** 🌐️ Feature-detects the WebCodecs `VideoDecoder`/`EncodedVideoChunk` globals (Tier 1's prerequisite;
 * absent in most JS test environments and in browsers that only support WebM/VP9 without an AVC path). */
function webCodecsAvailable(): boolean {
  const scope = window as unknown as { VideoDecoder?: unknown; EncodedVideoChunk?: unknown };
  return typeof scope.VideoDecoder === "function" && typeof scope.EncodedVideoChunk === "function";
}

/** 🔢️ Derives a WebCodecs `avc1.PPCCLL` codec string from an `avcC` box's profile/compat/level bytes
 * (offsets 1/2/3 — version is byte 0). */
function avcCodecString(description: Uint8Array): string {
  const hex = (byte: number | undefined) => (byte ?? 0).toString(16).padStart(2, "0");
  return `avc1.${hex(description[1])}${hex(description[2])}${hex(description[3])}`;
}

type WebCodecsVideoFrame = { readonly codedWidth: number; readonly codedHeight: number; close: () => void };
type WebCodecsVideoDecoderCtor = new (init: { output: (frame: WebCodecsVideoFrame) => void; error: (error: unknown) => void }) => {
  configure: (config: { codec: string; codedWidth: number; codedHeight: number; description: Uint8Array }) => void;
  decode: (chunk: unknown) => void;
  flush: () => Promise<void>;
  close: () => void;
};
type WebCodecsEncodedVideoChunkCtor = new (init: { type: "key" | "delta"; timestamp: number; data: Uint8Array }) => unknown;

function jpegDataUrlFromFrame(frame: WebCodecsVideoFrame): { readonly dataUrl: string; readonly width: number; readonly height: number } {
  const canvas = document.createElement("canvas");
  canvas.width = frame.codedWidth;
  canvas.height = frame.codedHeight;
  canvas.getContext("2d")?.drawImage(frame as unknown as CanvasImageSource, 0, 0);
  return { dataUrl: canvas.toDataURL("image/jpeg", 0.9), width: frame.codedWidth, height: frame.codedHeight };
}

/** 🎞️ Decodes exactly the samples needed for one target frame — from its nearest preceding sync sample
 * through the target — via a fresh `VideoDecoder`, capturing only the last output frame. Simplification:
 * each target frame re-decodes its GOP prefix from scratch instead of streaming continuously across
 * targets and demuxing outputs by timestamp; acceptable because sampled ingestion (`sampleStride`/
 * `maxFrames`) keeps GOP prefixes short between targets, and Tier 2's `<video>` element is always the
 * correctness fallback if Tier 1 fails or the codec isn't baseline-friendly. */
async function decodeOneMp4Frame(track: Mp4Track, bytes: Uint8Array, targetIndex: number): Promise<{ dataUrl: string; width: number; height: number } | null> {
  const scope = window as unknown as { VideoDecoder: WebCodecsVideoDecoderCtor; EncodedVideoChunk: WebCodecsEncodedVideoChunkCtor };
  let syncIndex = targetIndex;
  while (syncIndex > 0 && !track.samples[syncIndex]!.isSync) syncIndex -= 1;
  let captured: { dataUrl: string; width: number; height: number } | null = null;
  await new Promise<void>((resolve, reject) => {
    const decoder = new scope.VideoDecoder({
      output: (frame) => {
        captured = jpegDataUrlFromFrame(frame);
        frame.close();
      },
      error: reject,
    });
    decoder.configure({ codec: avcCodecString(track.description), codedWidth: track.width, codedHeight: track.height, description: track.description });
    for (let i = syncIndex; i <= targetIndex; i += 1) {
      const sample = track.samples[i]!;
      decoder.decode(
        new scope.EncodedVideoChunk({ type: sample.isSync ? "key" : "delta", timestamp: sample.timestampMs * 1000, data: bytes.subarray(sample.offset, sample.offset + sample.size) }),
      );
    }
    decoder.flush().then(() => {
      decoder.close();
      resolve();
    }, reject);
  });
  return captured;
}

/** 🎞️ Tier 1 orchestration: demuxes `bytes` as MP4/AVC, decodes one frame per sampled timestamp, and
 * dispatches `frameAction` per frame + `doneAction` once. Returns `false` (no dispatch performed at
 * all) when the demux can't find a usable AVC video track, so the caller falls through to Tier 2. */
async function runTier1VideoFrames(bytes: Uint8Array, effect: RequestMediaFramesArgs, name: string, dispatchOne: EffectDispatchOne): Promise<boolean> {
  const track = probeMp4VideoTrack(bytes);
  if (!track || track.samples.length === 0) return false;
  const durationMs = track.samples[track.samples.length - 1]!.timestampMs;
  const timestamps = sampleMediaFrameTimestampsMs(durationMs, effect.sampleStride, effect.maxFrames, effect.fpsHint);
  let sampledCount = 0;
  for (let index = 0; index < timestamps.length; index += 1) {
    const targetMs = timestamps[index]!;
    let targetSampleIndex = 0;
    for (let i = 0; i < track.samples.length; i += 1) if (track.samples[i]!.timestampMs <= targetMs) targetSampleIndex = i;
    const frame = await decodeOneMp4Frame(track, bytes, targetSampleIndex);
    if (!frame) continue;
    sampledCount += 1;
    await dispatchOne(effect.frameAction, {
      payload: frame.dataUrl,
      name,
      frameIndex: index,
      timestampMs: targetMs,
      index,
      total: timestamps.length,
      width: frame.width,
      height: frame.height,
      ...effect.args,
    });
  }
  await dispatchOne(effect.doneAction, {
    name,
    durationMs,
    frameCount: track.samples.length,
    sampledCount,
    width: track.width,
    height: track.height,
    codec: track.codec,
    ...effect.args,
  });
  return true;
}
//#endregion Tier1

//#region Tier2
/** ⏱️ Tier-2 (`<video>` seek-and-capture) target timestamps, ms — one every `sampleStride /
 * (fpsHint || 30)` seconds starting at 0, capped at `maxFrames` (0 ⇒ unlimited, bounded only by
 * `durationMs`). Pure/deterministic so it's unit-testable without any DOM or media APIs. Computes each
 * timestamp as `k * stepMs` rather than an accumulating `ts += stepMs` loop — repeated float addition
 * drifts enough over dozens of steps to occasionally land just under an exact multiple of `durationMs`,
 * sneaking in one extra timestamp; multiplying from the loop index is exact per-step and deterministic. */
export function sampleMediaFrameTimestampsMs(durationMs: number, sampleStride: number, maxFrames: number, fpsHint: number): number[] {
  const stride = sampleStride > 0 ? sampleStride : 1;
  const fps = fpsHint > 0 ? fpsHint : 30;
  const stepMs = (stride / fps) * 1000;
  const timestamps: number[] = [];
  if (durationMs <= 0 || stepMs <= 0) return timestamps;
  for (let k = 0; ; k += 1) {
    if (maxFrames > 0 && timestamps.length >= maxFrames) break;
    const ts = k * stepMs;
    if (ts >= durationMs) break;
    timestamps.push(ts);
  }
  return timestamps;
}

function captureCanvasFrame(video: HTMLVideoElement, maxLongEdgePx: number): { readonly dataUrl: string; readonly width: number; readonly height: number } {
  const sourceWidth = video.videoWidth || 0;
  const sourceHeight = video.videoHeight || 0;
  const scale = maxLongEdgePx > 0 ? Math.min(1, maxLongEdgePx / Math.max(sourceWidth, sourceHeight, 1)) : 1;
  const width = Math.max(1, Math.round(sourceWidth * scale));
  const height = Math.max(1, Math.round(sourceHeight * scale));
  const canvas = document.createElement("canvas");
  canvas.width = width;
  canvas.height = height;
  canvas.getContext("2d")?.drawImage(video, 0, 0, width, height);
  return { dataUrl: canvas.toDataURL("image/jpeg", 0.9), width, height };
}

function waitForVideoEvent(video: HTMLVideoElement, type: string): Promise<void> {
  return new Promise((resolve) => {
    const handler = () => {
      video.removeEventListener(type, handler);
      resolve();
    };
    video.addEventListener(type, handler);
  });
}

/** 🎞️ Tier 2 orchestration: waits for `loadedmetadata` (if not already available), seeks `video`
 * through {@link sampleMediaFrameTimestampsMs}'s schedule, captures each landed frame to a scaled JPEG
 * data URL, dispatches `frameAction` per frame, then `doneAction` once. Used both as the WebM/no-
 * WebCodecs fallback and directly by tests (which inject a real `<video>` element with overridden
 * `duration`/`videoWidth`/`videoHeight`/`readyState` and manually dispatch `loadedmetadata`/`seeked`,
 * since headless test environments have no real media decoder). */
export async function runTier2VideoFrames(video: HTMLVideoElement, effect: RequestMediaFramesArgs, name: string, dispatchOne: EffectDispatchOne): Promise<void> {
  if (video.readyState < 1) await waitForVideoEvent(video, "loadedmetadata");
  const durationMs = Number.isFinite(video.duration) ? video.duration * 1000 : 0;
  const width = video.videoWidth || 0;
  const height = video.videoHeight || 0;
  const timestamps = sampleMediaFrameTimestampsMs(durationMs, effect.sampleStride, effect.maxFrames, effect.fpsHint);
  const total = timestamps.length;
  for (let index = 0; index < total; index += 1) {
    const timestampMs = timestamps[index]!;
    video.currentTime = timestampMs / 1000;
    await waitForVideoEvent(video, "seeked");
    const frame = captureCanvasFrame(video, effect.maxLongEdgePx);
    await dispatchOne(effect.frameAction, {
      payload: frame.dataUrl,
      name,
      frameIndex: index,
      timestampMs,
      index,
      total,
      width: frame.width,
      height: frame.height,
      ...effect.args,
    });
  }
  await dispatchOne(effect.doneAction, { name, durationMs, frameCount: total, sampledCount: total, width, height, codec: "unknown", ...effect.args });
}
//#endregion Tier2

/** 🎞️ D5 `RequestMediaFrames` fields the two decode tiers need, decoupled from the raw `Effect`
 * union member shape so orchestration functions above take a plain, easily-constructed-in-tests object. */
export type RequestMediaFramesArgs = {
  readonly frameAction: string;
  readonly doneAction: string;
  readonly fallbackAction: string;
  readonly sampleStride: number;
  readonly maxFrames: number;
  readonly maxLongEdgePx: number;
  readonly fpsHint: number;
  readonly args?: Record<string, unknown>;
};

function bytesFromDataUrl(dataUrl: string): Uint8Array {
  const binary = atob(dataUrl.slice(dataUrl.indexOf(",") + 1));
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i += 1) bytes[i] = binary.charCodeAt(i);
  return bytes;
}

function bytesToDataUrl(bytes: Uint8Array, mime: string): string {
  let binary = "";
  for (let i = 0; i < bytes.length; i += 1) binary += String.fromCharCode(bytes[i]!);
  return `data:${mime};base64,${btoa(binary)}`;
}

/** 🎞️ D5 top-level: sources video bytes (`payload` data URL, or the native file picker when unset),
 * tries Tier 1 when WebCodecs is available and the demux finds a usable AVC track, otherwise Tier 2's
 * `<video>` seek-and-capture; on total failure (can't demux AND Tier 2 also throws, e.g. a corrupt
 * file) dispatches `fallbackAction` once with the raw original bytes as a data URL. */
export async function runRequestMediaFrames(
  effect: RequestMediaFramesArgs,
  accept: string,
  payload: string | undefined,
  dispatchOne: EffectDispatchOne,
  createVideoElement: () => HTMLVideoElement = () => document.createElement("video"),
): Promise<void> {
  let bytes: Uint8Array;
  let name = "video";
  if (payload) {
    bytes = bytesFromDataUrl(payload);
  } else {
    const opened = await requestFileOpen(accept || "video/*", "dataUrl", false);
    if (opened.length === 0) return;
    bytes = bytesFromDataUrl(opened[0]!.contents);
    name = opened[0]!.name;
  }
  try {
    if (webCodecsAvailable() && (await runTier1VideoFrames(bytes, effect, name, dispatchOne))) return;
    const url = URL.createObjectURL(new Blob([bytes], { type: "video/mp4" }));
    const video = createVideoElement();
    video.muted = true;
    video.playsInline = true;
    video.src = url;
    try {
      await runTier2VideoFrames(video, effect, name, dispatchOne);
    } finally {
      URL.revokeObjectURL(url);
    }
  } catch (error) {
    console.error("[os-shell] requestMediaFrames: decode failed, falling back to raw bytes", error);
    await dispatchOne(effect.fallbackAction, { payload: bytesToDataUrl(bytes, "video/mp4"), name, ...effect.args });
  }
}
//#endregion RequestMediaFrames

function isStudioMode(catalog: PluginCatalog, pluginFilter?: string): boolean {
  return pluginFilter !== undefined && resolvePluginHostConfig(catalog, pluginFilter) !== undefined;
}

export interface SpaceShellPath {
  readonly spaceId: string;
  readonly instanceId?: string;
}

export type ShellRoute = { readonly kind: "landing" } | { readonly kind: "space"; readonly spaceId: string; readonly instanceId?: string } | { readonly kind: "notFound"; readonly path: string };

/** @emoji 🧭️ Classifies shell history paths into landing, studio space, or unknown routes. */
export function parseShellRoute(path: string): ShellRoute {
  const normalized = (path.split("?")[0] ?? "/").trim() || "/";
  if (normalized === "/") return { kind: "landing" };
  const match = /^\/spaces\/([^/]+)(?:\/instances\/([^/]+))?$/.exec(normalized);
  if (match) return { kind: "space", spaceId: match[1]!, instanceId: match[2] };
  return { kind: "notFound", path: normalized };
}

/** @deprecated Use {@link parseShellRoute} instead. */
export function parseSpaceShellPath(path: string): SpaceShellPath | null {
  const route = parseShellRoute(path);
  if (route.kind !== "space") return null;
  return { spaceId: route.spaceId, instanceId: route.instanceId };
}

/**
 * 🗺️ Joins a breadcrumb for display.
 *
 * ⚠️ Accepts `undefined`: `AppDefinition.breadcrumb` is DECLARED OPTIONAL in the manifest, so every
 * unguarded `breadcrumb.join(…)` was a latent crash — and not a cosmetic one. This runs inside
 * `FrameworkOsShellInner`'s render, so a single app whose manifest omits the field took down the
 * WHOLE shell, and in a multi-pane host (the demonstrator) every pane died with it. An app with no
 * breadcrumb should render a nameless title, never destroy its host.
 */
export function appBreadcrumb(breadcrumb: readonly string[] | undefined): string {
  return (breadcrumb ?? []).join(APP_BREADCRUMB_SEPARATOR);
}

/** 🗺️ Resolves the breadcrumb effective under the active terminology; unknown/native ids fall back to `app.breadcrumb`, and an app that declares none at all to the empty breadcrumb (the field is optional — see {@link appBreadcrumb}). */
export function resolveAppBreadcrumb(app: Pick<AppDefinition, "breadcrumb" | "terminologyBreadcrumbs">, terminology: string): readonly string[] {
  return app.terminologyBreadcrumbs?.[terminology] ?? app.breadcrumb ?? [];
}

/** 🗺️ Resolves the breadcrumb for a non-active app (studio spawn palette/spawned entries) by looking up its `AppDefinition` across loaded plugins; falls back to the raw breadcrumb when the app can't be found. */
export function resolveArtifactByAppId(loadedPlugins: readonly LoadedProgramState[], appId: string, breadcrumb: readonly string[], terminology: string): readonly string[] {
  for (const program of loadedPlugins) {
    const app = program.manifest.apps.find((candidate) => candidate.id === appId);
    if (app) return resolveAppBreadcrumb(app, terminology);
  }
  return breadcrumb;
}

export function appWindowLabel(app: AppDefinition, terminology: string, windowLabel: string, locale: string = SHELL_LOCALES[0]): string {
  const trimmed = windowLabel.trim();
  if (trimmed) return trimmed;
  const override = app.terminologyBreadcrumbs?.[terminology];
  return override?.[override.length - 1]?.trim() || resolveManifestLabel(app.label, terminology, locale).trim();
}

export function buildSpacePanelState(programs: readonly SpaceProgramEntry[], spawnedApps: readonly SpawnedAppEntry[], activePanelTab = "s-play-catalogue", activeSpawnedId?: string): SpacePanelState {
  return { activePanelTab, programs, spawnedApps, activeSpawnedId };
}

export function panelJsonFromState(state: SpacePanelState): string {
  return packValueToBase64(state);
}

export function parsePanelState(viewState: ViewModel): SpacePanelState | null {
  if (!viewState.panelJson) return null;
  try {
    return packValueFromBase64(viewState.panelJson) as SpacePanelState;
  } catch {
    return null;
  }
}

/**
 * @emoji 🪟️ Returns a studio panel with `spawned` present and focused as `activeSpawnedId`.
 * Host-effect application must fold this into the in-flight `nextViewState` before the final
 * `SET_SESSION` write — a separate panel dispatch is overwritten by that write and leaves the shell
 * stuck on the studio surface.
 * @see Effect.openPluginInstance
 */
export function studioPanelFocusingSpawned(panel: SpacePanelState, spawned: SpawnedAppEntry): SpacePanelState {
  const spawnedApps = panel.spawnedApps.some((entry) => entry.id === spawned.id)
    ? panel.spawnedApps.map((entry) => (entry.id === spawned.id ? spawned : entry))
    : [...panel.spawnedApps, spawned];
  return buildSpacePanelState(panel.programs, spawnedApps, panel.activePanelTab, spawned.id);
}

/** @emoji 🐚️ Commits a studio panel into a view state's `panelJson` for a single host-effect session write. */
export function viewStateWithSpacePanel(viewState: ViewModel, panel: SpacePanelState): ViewModel {
  return { ...viewState, panelJson: panelJsonFromState(panel) };
}

/** @emoji 🧭️ Default anchor a plugin-declared panel-tab `group` docks into — groups only ever map to the four corners; the four edge-middle anchors start empty and are user-populated via drag-and-drop or a dock skeleton override. */
export function panelAnchorForGroup(group: string): Anchor {
  if (group === "workbench" || group === "document") return "top-left";
  if (group === "details") return "top-right";
  if (group === "display") return "bottom-left";
  if (group === "settings") return "bottom-right";
  return "top-right";
}

/** @emoji 🪟️ One leaf in a framework layout tree, with optional instance/template binding for multi-pane world views. */
type FrameworkLayoutWindowSeed = {
  readonly windowId: string;
  readonly windowKindId: string;
  readonly title?: string;
  readonly templateId?: string;
  readonly size: number;
  readonly corner?: WindowStackCorner;
};

/** @emoji 🪟️ Walks a framework layout and collects every window leaf, preferring `instanceId` as the live pane id. */
function collectFrameworkLayoutWindowSeeds(node: WindowLayoutAxisNode | WindowLayoutStackNode | WindowLayoutWindowNode, parentSize = 100): FrameworkLayoutWindowSeed[] {
  if (node.kind === "window") {
    return [
      {
        windowId: node.instanceId ?? node.windowKindId,
        windowKindId: node.windowKindId,
        title: node.title,
        templateId: node.templateId,
        size: parentSize,
        corner: node.corner,
      },
    ];
  }
  if (node.kind === "stack") {
    const size = node.size ?? parentSize;
    return node.children.map((child) => ({
      windowId: child.instanceId ?? child.windowKindId,
      windowKindId: child.windowKindId,
      title: child.title,
      templateId: child.templateId,
      size,
      corner: child.corner,
    }));
  }
  const childSizes = node.children.map((child) => ("size" in child ? child.size : undefined));
  const explicitTotal = childSizes.reduce<number>((sum, size) => sum + (size ?? 0), 0);
  const unsetCount = childSizes.filter((size) => size === undefined).length;
  const defaultEach = unsetCount > 0 ? Math.max(0, 100 - explicitTotal) / unsetCount : 0;
  return node.children.flatMap((child, index) => {
    const fraction = childSizes[index] ?? defaultEach;
    return collectFrameworkLayoutWindowSeeds(child, parentSize * (fraction / 100));
  });
}

/** 🗣️ For a single, non-instanced window (`instanceId` unset — the common case, one window per kind),
 * the `windowKind`'s own label is the single source of truth: a manifest-baked `WindowLayoutWindowNode.title`
 * (from a plugin's `create_default_layout(..., titles)` call) is a plain, locale-invariant string that
 * predates locale/terminology resolution entirely, so it must never win over a real `windowKinds` lookup —
 * it only survives as a last-resort fallback for a window kind id that isn't declared in the manifest
 * (mirrors {@link retitleWindowLayoutNode}'s already-correct precedence). For a multi-instance window
 * (`instanceId` set — several views sharing one `windowKind`, e.g. "Top"/"Perspective" both backed by a
 * single 3D-viewport kind), the shared kind label can't distinguish instances, so the baked per-instance
 * title is the real title and must win instead. */
function resolveFrameworkWindowTitle(
  windowKindId: string,
  instanceId: string | undefined,
  bakedTitle: string | undefined,
  windowKinds: readonly { readonly id: string; readonly label: LocalizedLabel | string }[],
  terminology: string,
  locale: string,
): string {
  if (instanceId) return bakedTitle ?? windowKindId;
  const kind = windowKinds.find((entry) => entry.id === windowKindId);
  return kind ? resolveManifestLabel(kind.label, terminology, locale) : (bakedTitle ?? windowKindId);
}

function convertFrameworkLayoutNodeToModeLayout(
  node: WindowLayoutAxisNode | WindowLayoutStackNode | WindowLayoutWindowNode,
  appLabelsOverlay: PluginAppLabelsOverlay,
  windowKinds: readonly { readonly id: string; readonly label: LocalizedLabel | string }[],
  terminology: string,
  locale: string,
): WindowLayoutNode {
  if (node.kind === "window") {
    const id = node.instanceId ?? node.windowKindId;
    const title = resolveFrameworkWindowTitle(node.windowKindId, node.instanceId, node.title, windowKinds, terminology, locale);
    return { kind: "window", id, title: wireLabel(resolveAppLabel(appLabelsOverlay, "windowKind", id, title)), corner: node.corner };
  }
  if (node.kind === "stack") {
    return {
      kind: "stack",
      size: node.size,
      children: node.children.map((child) => {
        const id = child.instanceId ?? child.windowKindId;
        const title = resolveFrameworkWindowTitle(child.windowKindId, child.instanceId, child.title, windowKinds, terminology, locale);
        return {
          kind: "window" as const,
          id,
          title: wireLabel(resolveAppLabel(appLabelsOverlay, "windowKind", id, title)),
          corner: child.corner,
        };
      }),
    };
  }
  return {
    kind: node.kind,
    size: node.size,
    children: node.children.map((child) => convertFrameworkLayoutNodeToModeLayout(child, appLabelsOverlay, windowKinds, terminology, locale)),
  };
}

/** @emoji 🗣️ Re-resolves every window's title from the app manifest's windowKinds via resolveManifestLabel in place, preserving the tree's structure/sizes/arrangement — used to react to a locale/terminology switch without discarding the user's live layout. */
export function retitleWindowLayoutNode(
  node: WindowLayoutNode,
  windowKinds: readonly { readonly id: string; readonly label: LocalizedLabel | string }[],
  extraInstances: readonly ExtraWindowInstance[],
  terminology: string,
  locale: string,
): WindowLayoutNode {
  if (node.kind === "window") {
    const extra = extraInstances.find((entry) => entry.id === node.id);
    const windowKindId = extra ? extra.windowKindId : node.id;
    const kind = windowKinds.find((entry) => entry.id === windowKindId);
    const title = kind ? wireLabel(resolveManifestLabel(kind.label, terminology, locale)) : (node.title ?? uiDataLabel(node.id));
    return { ...node, title };
  }
  return {
    ...node,
    children: node.children.map((child) => retitleWindowLayoutNode(child, windowKinds, extraInstances, terminology, locale)),
  } as WindowLayoutNode;
}

/** @emoji 🪟️ Resolves a framework layout into the live mode tree, extra instances, and pending projection templates without inferring window focus (no side effects). */
export function resolveFrameworkLayoutSeed(
  layout: WindowLayout | undefined,
  windowKinds: readonly { readonly id: string; readonly label: LocalizedLabel | string }[],
  appLabelsOverlay: PluginAppLabelsOverlay,
  terminology: string,
  locale: string,
): {
  readonly modeLayout: WindowLayoutNode;
  readonly extraInstances: readonly ExtraWindowInstance[];
  readonly pendingProjections: readonly { readonly windowId: string; readonly templateId: string }[];
} {
  const windowIds = windowKinds.map((kind) => kind.id);
  if (!layout?.root) {
    return {
      modeLayout: createEvenWindowLayout(windowIds.length ? windowIds : ["main"]),
      extraInstances: [],
      pendingProjections: [],
    };
  }
  const seeds = collectFrameworkLayoutWindowSeeds(layout.root);
  const kindById = new Map(windowKinds.map((kind) => [kind.id, kind] as const));
  const extraInstances: ExtraWindowInstance[] = [];
  const pendingProjections: { readonly windowId: string; readonly templateId: string }[] = [];
  for (const seed of seeds) {
    const kind = kindById.get(seed.windowKindId);
    if (!kind) continue;
    if (seed.windowId !== seed.windowKindId) {
      extraInstances.push({
        id: seed.windowId,
        windowKindId: seed.windowKindId,
        title: resolveAppLabel(appLabelsOverlay, "windowKind", seed.windowId, seed.title ?? resolveManifestLabel(kind.label, terminology, locale)),
      });
    }
    if (seed.templateId) pendingProjections.push({ windowId: seed.windowId, templateId: seed.templateId });
  }
  return {
    modeLayout: convertFrameworkLayoutNodeToModeLayout(layout.root, appLabelsOverlay, windowKinds, terminology, locale),
    extraInstances,
    pendingProjections,
  };
}

/** @emoji 🪟️ Applies a resolved framework layout seed: registers one-shot world projections, then returns the live layout payload. */
export function applyFrameworkLayoutSeed(
  layout: WindowLayout | undefined,
  windowKinds: readonly { readonly id: string; readonly label: LocalizedLabel | string }[],
  appLabelsOverlay: PluginAppLabelsOverlay,
  terminology: string,
  locale: string,
): {
  readonly modeLayout: WindowLayoutNode;
  readonly extraInstances: readonly ExtraWindowInstance[];
} {
  const seed = resolveFrameworkLayoutSeed(layout, windowKinds, appLabelsOverlay, terminology, locale);
  for (const pending of seed.pendingProjections) {
    const projectionSpec = decodeWorldProjectionTemplateId(pending.templateId);
    if (projectionSpec) registerPendingWorldProjection(pending.windowId, projectionSpec);
  }
  return { modeLayout: seed.modeLayout, extraInstances: seed.extraInstances };
}

function modeLayoutNodeToFramework(node: WindowLayoutNode, kindByInstanceId: ReadonlyMap<string, string>): WindowLayoutAxisNode | WindowLayoutStackNode | WindowLayoutWindowNode {
  if (node.kind === "window") {
    const windowKindId = kindByInstanceId.get(node.id) ?? node.id;
    const instanceId = kindByInstanceId.has(node.id) ? node.id : undefined;
    return {
      kind: "window",
      windowKindId,
      ...(node.title ? { title: node.title } : {}),
      ...(instanceId ? { instanceId } : {}),
    };
  }
  if (node.kind === "stack") {
    return {
      kind: "stack",
      ...(node.size !== undefined ? { size: node.size } : {}),
      children: node.children.map((child) => {
        const windowKindId = kindByInstanceId.get(child.id) ?? child.id;
        const instanceId = kindByInstanceId.has(child.id) ? child.id : undefined;
        return {
          kind: "window" as const,
          windowKindId,
          ...(child.title ? { title: child.title } : {}),
          ...(instanceId ? { instanceId } : {}),
        };
      }),
    };
  }
  return {
    kind: node.kind,
    ...(node.size !== undefined ? { size: node.size } : {}),
    children: node.children.map((child) => modeLayoutNodeToFramework(child, kindByInstanceId) as WindowLayoutStackNode | WindowLayoutAxisNode),
  };
}

export function captureCurrentFrameworkLayout(shellLayout: WindowLayoutNode | null, extraWindowInstances: readonly ExtraWindowInstance[], fallback?: WindowLayout): WindowLayout | undefined {
  if (!shellLayout) return fallback;
  const kindByInstanceId = new Map(extraWindowInstances.map((entry) => [entry.id, entry.windowKindId] as const));
  const root = modeLayoutNodeToFramework(shellLayout, kindByInstanceId);
  if (root.kind === "window") return { root: { kind: "stack", children: [root] } };
  return { root };
}

//#region WindowLayoutChangeClassify
/** 🪟️ Trailing settle delay for `Mode.onLayoutChange` (fires continuously during a live drag/resize) before
 * noting one `shell.windowResize`/`shell.windowMove` command for the whole gesture — matches Board2dHost's
 * own camera-sync settle debounce (`beginCameraInteraction`), the only precedent for this kind of
 * drag-settle pattern already in this file. */
export const LAYOUT_CHANGE_SETTLE_MS = 350;

/** 🪟️ Recursive skeleton of a {@link WindowLayoutNode} — kind/id/nesting only, stripping `size` (resize) and
 * a stack's `activeId` (mere focus echo) — so two trees compare equal here iff neither differs. */
type WindowLayoutSkeletonNode = { readonly kind: string; readonly id?: string; readonly corner?: string; readonly children?: readonly WindowLayoutSkeletonNode[] };
function windowLayoutSkeleton(node: WindowLayoutNode): WindowLayoutSkeletonNode {
  if (node.kind === "window") return { kind: node.kind, id: node.id, corner: node.corner };
  return { kind: node.kind, children: node.children.map((child) => windowLayoutSkeleton(child as WindowLayoutNode)) };
}

/** 🪟️ Like {@link windowLayoutSkeleton} but keeps each node's `size` (still ignores a stack's `activeId`) —
 * comparing two of these (after their plain skeletons already matched) is how {@link classifyWindowLayoutChange}
 * tells a pure resize apart from no change at all. */
type WindowLayoutSizedSkeletonNode = { readonly kind: string; readonly id?: string; readonly size?: number; readonly children?: readonly WindowLayoutSizedSkeletonNode[] };
function windowLayoutSizedSkeleton(node: WindowLayoutNode): WindowLayoutSizedSkeletonNode {
  if (node.kind === "window") return { kind: node.kind, id: node.id, size: node.size };
  return { kind: node.kind, size: node.size, children: node.children.map((child) => windowLayoutSizedSkeleton(child as WindowLayoutNode)) };
}

/** 🪟️ Classifies a `Mode.onLayoutChange` delta by comparing the previous and next layout tree — `"rearrange"`
 * when window ids/nesting structure differ (drag-to-new-position, split, close), `"resize"` when only pane
 * sizes differ, `null` when neither differs (a pure active-window-flag echo, handled by the dedicated
 * active-window seam instead — never worth its own shell command). */
export function classifyWindowLayoutChange(previous: WindowLayoutNode | null, next: WindowLayoutNode | null): "resize" | "rearrange" | null {
  if (previous === next) return null;
  if (!previous || !next) return "rearrange";
  if (JSON.stringify(windowLayoutSkeleton(previous)) !== JSON.stringify(windowLayoutSkeleton(next))) return "rearrange";
  if (JSON.stringify(windowLayoutSizedSkeleton(previous)) !== JSON.stringify(windowLayoutSizedSkeleton(next))) return "resize";
  return null;
}
//#endregion WindowLayoutChangeClassify

function windowEngagementControlToSpec(control: WindowEngagementControl | undefined, onAction: (action: ActionDescriptor) => void): EngagementControl | undefined {
  if (!control) return undefined;
  if (control.kind === "ring" || control.kind === "toggleGroup") {
    return {
      kind: control.kind,
      id: control.id,
      label: control.label,
      value: control.value,
      disabled: control.disabled,
      options: control.options.map((row) => ({ id: row.id, label: row.label, disabled: row.disabled })),
      onSelect: control.onSelect ? (id: string) => onAction({ ...control.onSelect!, args: { ...(control.onSelect!.args as object | undefined), id } }) : undefined,
    };
  }
  if (control.kind === "select") {
    return {
      kind: "select",
      id: control.id,
      label: control.label,
      value: control.value,
      placeholder: control.placeholder,
      disabled: control.disabled,
      items: control.items.map((row) => ({ id: row.id, value: row.value, label: row.label })),
      onChange: control.onChange ? (value: string) => onAction({ ...control.onChange!, args: { ...(control.onChange!.args as object | undefined), value } }) : undefined,
    };
  }
  const dispatchNumeric = (action: ActionDescriptor | undefined, value: number) => {
    if (!action) return;
    onAction({ ...action, args: { ...(action.args as object | undefined), value } });
  };
  return {
    kind: control.kind,
    id: control.id,
    label: control.label,
    value: control.value,
    min: control.min,
    max: control.max,
    step: control.step,
    unit: control.unit,
    disabled: control.disabled,
    onChange: control.onChange ? (value: number) => dispatchNumeric(control.onChange, value) : undefined,
    onCommit: control.onCommit ? (value: number) => dispatchNumeric(control.onCommit, value) : undefined,
  };
}

const PLUGIN_LOAD_TIMEOUT_MS = 30_000;

/** @emoji 🔌️ Result of {@link installPlugin} — the boot effect must not infer success from
 * `loadedPluginsRef`, which only updates after the next React commit. */
type PluginInstallOutcome = "loaded" | "already-loaded" | "in-flight" | "missing-registry" | "failed";

export async function loadPluginModuleResilient(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle | null> {
  try {
    return await Promise.race([
      loadPluginModule(pluginId, moduleUrl),
      new Promise<never>((_, reject) => {
        window.setTimeout(() => reject(new Error(`timeout loading ${pluginId}`)), PLUGIN_LOAD_TIMEOUT_MS);
      }),
    ]);
  } catch (error) {
    console.error("[DEBUG] program load failed", pluginId, error);
    return null;
  }
}

function isViewportSurface(surfaceKind: string | undefined): boolean {
  return surfaceKind === "world-3d" || surfaceKind === "node-graph" || surfaceKind === "canvas-2d";
}

function defaultViewportEngagement(): WindowEngagement {
  return {
    sessionActive: true,
    status: [{ id: "framework.viewport.status", text: shellLabel("ui.engagement.viewport") }],
  };
}

export function resolveWindowEngagement(kind: AppDefinition["windowKinds"][number], windowId: string, byWindowId: Readonly<Record<string, WindowEngagement>>): WindowEngagement | undefined {
  const surfaceKind = (kind as { surfaceKind?: string }).surfaceKind;
  const declaredEngagement = kind.options.engagement.kind === "some" ? kind.options.engagement.value : undefined;
  return byWindowId[windowId] ?? declaredEngagement ?? (isViewportSurface(surfaceKind) ? defaultViewportEngagement() : undefined);
}

export function windowEngagementToSpec(engagement: WindowEngagement | undefined, onAction: (action: ActionDescriptor) => void): EngagementSpec | undefined {
  if (!engagement) return undefined;
  const options = engagement.options?.map((option) => ({
    id: option.id,
    label: option.label,
    icon: option.iconId ? <Icon icon={option.iconId as IconName} size="small" /> : undefined,
    pressed: option.pressed,
    disabled: option.disabled,
    onPress: option.action ? () => onAction(option.action!) : undefined,
  }));
  const status = engagement.status?.map((row) => ({ id: row.id, content: row.text }));
  const control = windowEngagementControlToSpec(engagement.control, onAction);
  const controls = engagement.controls?.map((row) => windowEngagementControlToSpec(row, onAction)).filter((row): row is EngagementControl => row !== undefined);
  const hasContent = (options?.length ?? 0) > 0 || Boolean(control) || (controls?.length ?? 0) > 0 || (status?.length ?? 0) > 0;
  if (!hasContent) return undefined;
  return { sessionActive: engagement.sessionActive, options, control, controls, status };
}

/** @emoji 🔎️ Builds the top-middle window {@link SearchSpec} from the same Rust engagement payload: typed action input and autocomplete possibles. */
export function windowEngagementToSearchSpec(engagement: WindowEngagement | undefined, onAction: (action: ActionDescriptor) => void): SearchSpec | undefined {
  if (!engagement) return undefined;
  const input = engagement.input
    ? {
        id: engagement.input.id,
        value: engagement.input.value,
        placeholder: engagement.input.placeholder,
        disabled: engagement.input.disabled,
        onChange: engagement.input.onChange ? (value: string) => onAction({ ...engagement.input!.onChange!, args: { ...(engagement.input!.onChange!.args as object | undefined), value } }) : undefined,
        onSubmit: engagement.input.onSubmit ? (value: string) => onAction({ ...engagement.input!.onSubmit!, args: { ...(engagement.input!.onSubmit!.args as object | undefined), value } }) : undefined,
        onRepeatLast: engagement.input.onRepeatLast ? () => onAction(engagement.input!.onRepeatLast!) : undefined,
        onAbort: engagement.input.onAbort ? () => onAction(engagement.input!.onAbort!) : undefined,
      }
    : undefined;
  const possibles = engagement.possibleEngagements?.map((row) => ({
    id: row.id,
    label: row.label,
    detail: row.detail,
    onSelect: row.action ? () => onAction(row.action!) : undefined,
  }));
  const hasContent = Boolean(input) || (possibles?.length ?? 0) > 0;
  if (!hasContent) return undefined;
  return { sessionActive: engagement.sessionActive, input, possibles };
}

function panelTabIcon(tabId: string, group: string): React.FC<{ size?: number }> {
  // 🌱️ `group === "workbench"` already covers every host-app catalogue tab (each such app declares its
  // catalogue tab under `PanelGroup::Workbench` — see `s/plugin/rs`'s `App::builder(...).panel_tab(...)`)
  // so no separate app-specific tab-id literal is needed here.
  if (group === "workbench") return shellTabIcon(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID);
  if (tabId.includes("parameters")) return shellTabIcon(FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID);
  if (tabId.includes("inspector")) return shellTabIcon(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID);
  if (tabId === FRAMEWORK_PANEL_TAB_HISTORY_ID) return shellTabIcon("undo");
  return shellTabIcon(tabId);
}

/** @emoji 🌳️ Category-row icon: the first child's icon, or `fallback` when the category has no tabs yet. */
export function categoryTabIcon(tabs: readonly PanelTabNode[], fallback: IconName): React.FC<{ size?: number }> {
  const FirstIcon = tabs[0]?.icon;
  return function CategoryTabIcon({ size = 16 }: { size?: number }) {
    return FirstIcon ? <FirstIcon size={size} /> : <Icon icon={fallback} size="small" />;
  };
}

/** @emoji 🌳️ Depth-first leaves of a recursive panel-tab tree — the nodes that actually carry a `bodyKey` to render. */
export function flattenPanelTabLeaves<T extends { readonly children?: readonly T[] }>(tabs: readonly T[]): T[] {
  return tabs.flatMap((tab) => (tab.children && tab.children.length > 0 ? flattenPanelTabLeaves(tab.children) : [tab]));
}

/** @emoji 🌳️ Converts one plugin-declared {@link AppPanelTabDefinition} (recursively) into a {@link PanelTabNode}. */
export function panelTabDefinitionToNode(
  tab: AppPanelTabDefinition,
  group: string,
  panelUiByKey: Readonly<Record<string, UiNode>>,
  onAction: (action: ActionDescriptor) => void,
  order: number,
  appLabelsOverlay: PluginAppLabelsOverlay,
  terminology: string = UI_TERMINOLOGY_NATIVE,
  locale: string = SHELL_LOCALES[0],
): PanelTabNode {
  const tabId = panelTabKindId(tab.kind);
  const label = resolvePanelTabLabel(appLabelsOverlay, tabId, resolveManifestLabel(tab.label, terminology, locale));
  if (tab.children && tab.children.length > 0) {
    return {
      kind: "branch",
      id: tabId,
      icon: panelTabIcon(tabId, group),
      name: label,
      order,
      children: tab.children.map((child, childOrder) => panelTabDefinitionToNode(child, group, panelUiByKey, onAction, childOrder, appLabelsOverlay, terminology, locale)),
    };
  }
  return singleTreeLeaf({
    id: tabId,
    icon: panelTabIcon(tabId, group),
    name: label,
    order,
    tree: staticTreePanelDefinition(uiNodeToTreePanelConfig(panelUiByKey[tabId] ?? pendingPanelUiNode(), onAction)),
  });
}

export function resolveCanvasBodyKey(app: AppDefinition): string {
  const windowKind = app.windowKinds[0];
  if (!windowKind) return "main";
  if (windowKind.bodyKey.includes("composite")) {
    const workflow = app.windowKinds.find((kind) => kind.bodyKey.includes("workflow"));
    return workflow?.bodyKey ?? windowKind.bodyKey;
  }
  return windowKind.bodyKey;
}

//#region 🧰️UtilityRegistry
/**
 * 🧰️ Resolves the `UtilityDefinition`s in scope for one window kind against the app's utility registry:
 * the window kind's own `utilities` refs when non-empty, otherwise every utility the app declares (the
 * scoping fallback, mirroring `resolveWindowActions`' intent for utilities). Unresolvable refs are dropped.
 */
export function resolveUtilities(app: Pick<AppDefinition, "utilities">, windowKind: Pick<AppWindowKindDefinition, "utilities">): UtilityDefinition[] {
  const registry = app.utilities ?? [];
  const refs = windowKind.utilities ?? [];
  if (refs.length === 0) return [...registry];
  const resolved: UtilityDefinition[] = [];
  for (const ref of refs) {
    const utility = registry.find((entry) => entry.id === ref);
    if (utility) resolved.push(utility);
  }
  return resolved;
}

/** 🧰️ Chrome-known ribbon-group ids that already have a `ui.ribbon.parent.*` translation key — the fallback tier for plugin-declared utility groups not covered by that plugin's own `groupLabels` overlay. */
const CHROME_KNOWN_RIBBON_PARENT_CATEGORIES = new Set(UI_RIBBON_PARENT_CATEGORIES);

/** 🧰️ Resolves a `UtilityDefinition.group` id's display label: the app's own `groupLabels` overlay first, then the shared `ui.ribbon.parent.*` chrome vocabulary for known category ids, else the raw id. */
function resolveUtilityGroupLabel(group: string, appLabelsOverlay: PluginAppLabelsOverlay): string {
  const fallback = CHROME_KNOWN_RIBBON_PARENT_CATEGORIES.has(group) ? shellLabel(`ui.ribbon.parent.${group as UiRibbonParentCategory}`) : group;
  return resolveAppLabel(appLabelsOverlay, "group", group, fallback);
}

/** 🧰️ One `UtilityDefinition` → the lean `DerivedUtilitySpec` consumed by {@link deriveUtilityNodes}, resolving the label (and, for grouped utilities, the group label) through the app's locale/terminology overlay. `UtilityDefinition.label` is a manifest `LocalizedLabel` field. */
function utilityDefinitionToSpec(utility: UtilityDefinition, appLabelsOverlay: PluginAppLabelsOverlay, terminology: string, locale: string): DerivedUtilitySpec {
  return {
    id: utility.id,
    label: resolveAppLabel(appLabelsOverlay, "utility", utility.id, resolveManifestLabel(utility.label, terminology, locale)),
    iconId: utility.iconId,
    group: utility.group ?? undefined,
    groupLabel: utility.group ? resolveUtilityGroupLabel(utility.group, appLabelsOverlay) : undefined,
    category: utility.category ?? "utilities",
  };
}

/** 🧰️ Stamps the owning `windowId` onto every `setActiveUtility` descriptor in a derived utility tree so the shell's `onAction` interceptor targets the right window regardless of which window is globally active. */
function tagSetActiveUtilityWindow(nodes: readonly UtilityNode[], windowId: string): UtilityNode[] {
  return nodes.map((node) => {
    if (node.kind === "collection") return { ...node, children: tagSetActiveUtilityWindow(node.children, windowId) };
    if (node.kind === "toggle" && "onChange" in node && node.onChange.action === SET_ACTIVE_UTILITY_ACTION_ID) {
      return { ...node, onChange: { ...node.onChange, args: { ...(node.onChange.args as object | undefined), windowId } } };
    }
    return node;
  });
}

/**
 * 🧰️ Builds the window utility bar `UtilityNode[]` for one window purely from the static utility registry plus
 * the host-owned active utility id — the replacement for the deleted program `list-tools` sourcing. Each
 * `setActiveUtility` descriptor is tagged with `windowId` so activation is scoped to this exact window.
 */
export function resolveUtilityNodes(
  app: Pick<AppDefinition, "utilities" | "controllerId">,
  windowKind: Pick<AppWindowKindDefinition, "utilities">,
  activeUtilityId: string | null | undefined,
  windowId: string,
  appLabelsOverlay: PluginAppLabelsOverlay = EMPTY_APP_LABELS_OVERLAY,
  terminology: string = UI_TERMINOLOGY_NATIVE,
  locale: string = SHELL_LOCALES[0],
): UtilityNode[] {
  const utilities = resolveUtilities(app, windowKind);
  if (utilities.length === 0) return [];
  return tagSetActiveUtilityWindow(
    deriveUtilityNodes(
      app.controllerId,
      utilities.map((utility) => utilityDefinitionToSpec(utility, appLabelsOverlay, terminology, locale)),
      activeUtilityId ?? undefined,
    ),
    windowId,
  );
}
//#endregion 🧰️UtilityRegistry

/** @emoji 💬️ Builds spawned-window engagement, search, measures, and utility-options chrome for one window instance. */
export function spawnedWindowChromeForKind(
  kind: AppDefinition["windowKinds"][number],
  windowId: string,
  engagementsByWindowId: Readonly<Record<string, WindowEngagement>>,
  measuresByWindowId: Readonly<Record<string, readonly WindowMeasure[]>>,
  activeUtilityId: string | undefined,
  onAction: (action: ActionDescriptor) => void,
): { readonly engagement?: EngagementSpec; readonly search?: SearchSpec; readonly measures: ReactNode; readonly utilityOptions: ReactNode } {
  const { measures, utilityOptions } = windowMeasuresChrome(measuresByWindowId[windowId] ?? kind.options.measures, activeUtilityId, windowId, onAction);
  const resolvedEngagement = resolveWindowEngagement(kind, windowId, engagementsByWindowId);
  return {
    engagement: windowEngagementToSpec(resolvedEngagement, onAction),
    search: windowEngagementToSearchSpec(resolvedEngagement, onAction),
    measures,
    utilityOptions,
  };
}

function isTreeNode(node: UiNode): node is UiTreeNode {
  return node.type === "tree";
}

export function uiNodeToTreePanelConfig(node: UiNode, onAction: (action: ActionDescriptor) => void): TreePanelConfig {
  const treeHasDrag = node.type === "tree" && node.sections.some((s) => s.items.some((i) => i.draggable || i.dragData));
  if (isTreeNode(node)) {
    return {
      ...uiTreeNodeToTreePanelConfig(node, onAction),
      dragAndDropController: node.dropAction || treeHasDrag ? declarativeTreeDragController(node, onAction) : undefined,
    };
  }
  return declarativeUiNodeToTreePanelConfig(node, onAction);
}

/** @emoji 🌲️ Maps non-tree declarative UI (stack/section/field/controls) to the same TreePanel shape Settings/Theme use — never an empty-label wrapper host (that rendered as a lone document icon above nested content). */
function declarativeUiNodeToTreePanelConfig(node: UiNode, onAction: (action: ActionDescriptor) => void): TreePanelConfig {
  if (node.type === "stack") {
    const emphasized = node.children.find((child) => child.type === "text" && child.emphasize);
    const bodyChildren = node.children.filter((child) => !(child.type === "text" && child.emphasize));
    const sectionNodes = bodyChildren.filter((child) => child.type === "section");
    if (sectionNodes.length > 0 && sectionNodes.length === bodyChildren.length) {
      return {
        sections: sectionNodes.map((section) => ({
          id: section.id,
          label: section.label ?? "",
          defaultOpen: section.defaultOpen,
          items: section.children.flatMap((child, index) => declarativeUiChildToTreeItems(child, `${section.id}.${index}`, onAction)),
        })),
        sortableSections: false,
      };
    }
    return {
      sections: [
        {
          id: node.id ?? "panel.body",
          label: emphasized && emphasized.type === "text" ? emphasized.value : "",
          defaultOpen: true,
          items: bodyChildren.flatMap((child, index) => declarativeUiChildToTreeItems(child, `${node.id ?? "panel.body"}.${index}`, onAction)),
        },
      ],
      sortableSections: false,
    };
  }
  if (node.type === "section") {
    return {
      sections: [
        {
          id: node.id,
          label: node.label ?? "",
          defaultOpen: node.defaultOpen,
          items: node.children.flatMap((child, index) => declarativeUiChildToTreeItems(child, `${node.id}.${index}`, onAction)),
        },
      ],
      sortableSections: false,
    };
  }
  return {
    sections: [
      {
        id: "panel.body",
        label: "",
        defaultOpen: true,
        items: declarativeUiChildToTreeItems(node, "panel.body.0", onAction),
      },
    ],
    sortableSections: false,
  };
}

function isUiControlNode(node: UiNode): node is UiControlNode {
  switch (node.type) {
    case "button":
    case "input":
    case "select":
    case "toggle":
    case "slider":
    case "numberStepper":
    case "ring":
    case "iconSelect":
    case "keyValue":
      return true;
    default:
      return false;
  }
}

function declarativeUiChildToTreeItems(node: UiNode, fallbackId: string, onAction: (action: ActionDescriptor) => void): TreeDataItem[] {
  switch (node.type) {
    case "field": {
      const control = isUiControlNode(node.child) ? renderUiControl(node.child, onAction) : <InterpretedUiNode node={node.child} onAction={onAction} />;
      return [{ id: node.id, label: node.label, description: node.description, control }];
    }
    case "text":
      return [{ id: `${fallbackId}.text`, label: node.value }];
    case "button":
      return [{ id: node.id ?? fallbackId, label: node.label, control: renderUiControl(node, onAction) }];
    case "input":
    case "select":
    case "toggle":
    case "slider":
    case "numberStepper":
    case "ring":
    case "iconSelect":
    case "keyValue":
      return [{ id: node.id, label: node.placeholder ?? node.id, control: renderUiControl(node, onAction) }];
    case "stack":
      return node.children.flatMap((child, index) => declarativeUiChildToTreeItems(child, `${fallbackId}.${index}`, onAction));
    case "group":
      return [
        {
          id: node.id,
          label: node.label,
          defaultOpen: node.defaultOpen,
          items: node.children.flatMap((child, index) => declarativeUiChildToTreeItems(child, `${node.id}.${index}`, onAction)),
        },
      ];
    case "tree":
      return uiTreeNodeToTreePanelConfig(node, onAction).sections.flatMap((section) => section.items);
    case "separator":
      return [{ id: `${fallbackId}.sep`, label: "—" }];
    default:
      return [
        {
          id: fallbackId,
          label: node.type,
          control: (
            <ShellFaultBoundary boundaryId={`panel-${fallbackId}`} fallbackLabel={shellLabel("ui.common.renderError")}>
              <ChromeAwareWindowScrollSurface className="min-h-0 flex-1">{interpretUiNode(node, { onAction })}</ChromeAwareWindowScrollSurface>
            </ShellFaultBoundary>
          ),
        },
      ];
  }
}

export function shellTabIcon(iconId: IconName | string): React.FC<{ size?: number }> {
  return function ShellTabIcon({ size = 16 }: { size?: number }) {
    const iconName: IconName =
      iconId === FRAMEWORK_PANEL_TAB_ARTIFACT_ICON_ID
        ? "file-text"
        : iconId === FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID
          ? "panel-catalogue"
          : iconId === FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID
            ? "panel-inspection"
            : iconId === FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID
              ? "panel-parameters"
              : isIconName(iconId)
                ? iconId
                : "circle-dot";
    return <Icon icon={iconName} size={size} />;
  };
}

/** @emoji 🌐️ Resolves a chrome translation key outside hook context (tree builders run there). Both bundles
 * are guaranteed complete for every key via `satisfies UiTranslationSchema`, so `?? key` is unreachable in
 * practice — kept only as a last-resort literal rather than a thrown error. `options` supports i18next
 * interpolation for keys with `{{placeholders}}`. */
export function shellLabel(key: UiTranslationKey, options?: Record<string, unknown>): UiLabel {
  return wireLabel(resolveTranslationLabel(uiI18n.t(key, options)) ?? key);
}

//#region 👁️✏️SurfaceRoleLabels
/** 👁️✏️ Frozen bilingual text pair (contract freeze §5) — English first, no default language. These
 * strings are net-new chrome vocabulary and the domain-neutral `uiChromeTranslationBundles` dictionary
 * they'd normally register into lives outside this lease (`🖱️ui/📦️packages/🟦️typescript/🎯️targets/
 * ⚛️react/📦️component.tsx`), so they resolve directly off `uiLocale` here — the same
 * `{ native: { en, de }, reuse: { en, de } }`-shaped idiom `resolveManifestLabel` already gives a
 * plugin's own `LocalizedLabel`, just constructed locally instead of decoded off the wire. */
type FrozenLabel = { readonly en: string; readonly de: string };

function frozenLabelText(pair: FrozenLabel, locale: string): string {
  return locale === "de" ? pair.de : pair.en;
}

/** 👁️✏️ Window title chip / read-only badge text — contract freeze §5. */
const SURFACE_ROLE_CHIP_LABEL: Readonly<Record<AppRole, FrozenLabel>> = {
  viewer: { en: "Viewer", de: "Betrachter" },
  editor: { en: "Editor", de: "Editor" },
};
export function surfaceRoleChipText(role: AppRole, locale: string): string {
  return frozenLabelText(SURFACE_ROLE_CHIP_LABEL[role], locale);
}

const OPEN_ARTIFACT_WITH_LABEL: FrozenLabel = { en: "Open with…", de: "Öffnen mit…" };
export function openArtifactWithText(locale: string): string {
  return frozenLabelText(OPEN_ARTIFACT_WITH_LABEL, locale);
}

const SET_AS_DEFAULT_LABEL: FrozenLabel = { en: "Set as default", de: "Als Standard festlegen" };
export function setAsDefaultText(locale: string): string {
  return frozenLabelText(SET_AS_DEFAULT_LABEL, locale);
}

const DEFAULT_APPS_SETTINGS_TAB_LABEL: FrozenLabel = { en: "Default apps", de: "Standard-Apps" };
export function defaultAppsSettingsTabText(locale: string): string {
  return frozenLabelText(DEFAULT_APPS_SETTINGS_TAB_LABEL, locale);
}
/** 👁️✏️ {@link defaultAppsSettingsTabText} as a {@link UiLabel} — for `PanelTabNode.name`, which
 * (unlike a plain tree-item `label`) always takes the `wireLabel`-wrapped shape `shellLabel` returns. */
export function defaultAppsSettingsTabLabel(locale: string): UiLabel {
  return wireLabel(defaultAppsSettingsTabText(locale));
}

/** 👁️✏️ Not itself a contract-frozen string (the freeze pins the chip/"Open with…"/"Set as
 * default"/"Default apps" vocabulary, not this one) — the text for the non-blocking notice contract
 * freeze §2.3/§5 requires when a `"viewer.read-only"` fault surfaces, or a viewer-role dispatch is
 * blocked client-side before it ever reaches the host. */
const VIEWER_READ_ONLY_NOTICE_LABEL: FrozenLabel = { en: "This is a read-only viewer — editing is disabled.", de: "Dies ist ein schreibgeschützter Betrachter – Bearbeiten ist deaktiviert." };
export function viewerReadOnlyNoticeText(locale: string): string {
  return frozenLabelText(VIEWER_READ_ONLY_NOTICE_LABEL, locale);
}

/** 👁️✏️ The `SettingsDefaultApps` table's "no pin" option — no existing `ui.common.*` chrome key
 * covers a bare "None" (checked: `ui.common.close`/`.none` are not registered), so this follows the
 * same local-resolution idiom as the rest of this region rather than adding one to the out-of-lease
 * chrome dictionary for a single call site. */
const NONE_OPTION_LABEL: FrozenLabel = { en: "None", de: "Keine" };
export function noneOptionText(locale: string): string {
  return frozenLabelText(NONE_OPTION_LABEL, locale);
}

/** 👁️✏️ Palette command ids, frozen (contract freeze §5) — `owner: "os"`, no `os.` prefix (unlike the
 * wire `AppCommand`s in `💻️os/🎮️commands/`, which these are NOT the same thing as: selecting either
 * one opens the shell's own "Open with…" picker scoped to that role, it doesn't itself send
 * `os.open-artifact-with` over the app channel — the picker's own row click does that). */
export const OPEN_ARTIFACT_WITH_VIEWER_COMMAND_ID = "open-artifact-with-viewer";
export const OPEN_ARTIFACT_WITH_EDITOR_COMMAND_ID = "open-artifact-with-editor";

/** 👁️✏️ `true` for a `mutation`-kind action/command — the one predicate every viewer-chrome hiding
 * rule in this lease (context menu, command palette, dispatch guard) shares, so "what counts as an
 * editing verb" has exactly one definition. */
export function isMutationKindDefinition(definition: Pick<ActionDefinition, "kind"> | Pick<CommandDefinition, "kind">): boolean {
  return definition.kind === "mutation";
}

/** 👁️✏️ Filters `Mutation`-kind entries out of an action/command list for a `"viewer"` role — a no-op
 * for `"editor"` (and for `undefined`, since an app with no resolved session has no role to gate on
 * yet). Shared by the shell context menu and the command palette so both hide the exact same set. */
export function filterDefinitionsForRole<T extends Pick<ActionDefinition, "kind"> | Pick<CommandDefinition, "kind">>(definitions: readonly T[], role: AppRole | undefined): readonly T[] {
  if (role !== "viewer") return definitions;
  return definitions.filter((definition) => !isMutationKindDefinition(definition));
}

/** 👁️✏️ One `AppRouter` entry resolved for display in the "Open with…" surfaces (Document panel,
 * context menu, command palette) — plugin-labelled, flagged as the current session's own app and/or
 * the pinned default so the picker can render both without a second lookup. */
export type OpenWithEntry = {
  readonly app: AppRef;
  readonly pluginLabel: string;
  readonly current: boolean;
  readonly isDefault: boolean;
};

/** 👁️✏️ Groups one dialect's `AppRouter` entries by role for the "Open with…" surfaces — owner-plugin
 * first within each role group (the router's own build-time ordering, contract freeze §3), annotated
 * with `current`/`isDefault` against the live session and pinned prefs. `pluginLabel` falls back to
 * the raw `pluginId` when the plugin's own manifest label isn't available (e.g. not loaded yet). */
export function groupOpenWithEntries(
  router: AppRouter,
  dialect: ArtifactDialect,
  currentApp: AppRef | undefined,
  pinnedApp: (role: AppRole) => AppRef | undefined,
  pluginLabelById: ReadonlyMap<string, string>,
): Readonly<Record<AppRole, readonly OpenWithEntry[]>> {
  const forRole = (role: AppRole): readonly OpenWithEntry[] => {
    const pinned = pinnedApp(role);
    return router.entriesFor(dialect, role).map((app) => ({
      app,
      pluginLabel: pluginLabelById.get(app.pluginId) ?? app.pluginId,
      current: currentApp?.pluginId === app.pluginId && currentApp?.appId === app.appId,
      isDefault: pinned?.pluginId === app.pluginId && pinned?.appId === app.appId,
    }));
  };
  return { viewer: forRole("viewer"), editor: forRole("editor") };
}
//#endregion 👁️✏️SurfaceRoleLabels

//#region 🔖️CheckInAndSyncStatus
/** 📌️ ticket `26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS` §C5 — auto check-in
 * policy constants: an open editor session with uncommitted edits checkpoints once it has been idle
 * (no new edit) for this long, or immediately once this many uncommitted edits have piled up (never
 * waiting for an idle period once the threshold is crossed). */
export const AUTO_CHECKIN_IDLE_MS = 20_000;
export const AUTO_CHECKIN_EDIT_THRESHOLD = 200;

/** 📌️ A tiny, framework-free debounce scheduler for the auto check-in policy: call {@link notify}
 * every time the uncommitted-edit count changes for the currently open editor session; it fires
 * `onCheckpoint` at most once per idle period — immediately once `threshold` is reached (never
 * waiting out the idle period once crossed), otherwise `idleMs` after the LAST `notify` call, and
 * never again until `notify(0)` (a landed checkpoint) clears the `pending` latch — the "never a
 * storm" guard: a `notify` that arrives while a checkpoint is already pending is a no-op. Deliberately
 * framework-free (`setTimeout`/`clearTimeout` only, no React) so it is unit-testable with vitest's
 * fake timers without mounting a component tree — `ShellHost`'s own effect is a thin wrapper around
 * one instance per open editor session, `cancel`ed on unmount/session-switch. */
export class AutoCheckinScheduler {
  private timer: ReturnType<typeof setTimeout> | null = null;
  private pending = false;

  constructor(
    private readonly onCheckpoint: () => void,
    private readonly idleMs: number = AUTO_CHECKIN_IDLE_MS,
    private readonly threshold: number = AUTO_CHECKIN_EDIT_THRESHOLD,
  ) {}

  notify(uncommittedEditCount: number): void {
    if (uncommittedEditCount === 0) {
      this.cancel();
      this.pending = false;
      return;
    }
    if (this.pending) return;
    if (uncommittedEditCount >= this.threshold) {
      this.cancel();
      this.pending = true;
      this.onCheckpoint();
      return;
    }
    this.cancel();
    this.timer = setTimeout(() => {
      this.timer = null;
      this.pending = true;
      this.onCheckpoint();
    }, this.idleMs);
  }

  cancel(): void {
    if (this.timer != null) {
      clearTimeout(this.timer);
      this.timer = null;
    }
  }
}

const CHECKIN_ACTION_LABEL: FrozenLabel = { en: "Check In", de: "Einchecken" };
/** 📌️ `#s-checkin`'s own label — mirrors `⚛️react/📦️index.tsx`'s `ui.checkin.action` bilingual pair
 * (that barrel sits downstream of `ShellHost`, so it cannot be imported here without a cycle — see
 * `SurfaceRoleLabels`'s header doc for the identical constraint). */
export function checkinActionText(locale: string): string {
  return frozenLabelText(CHECKIN_ACTION_LABEL, locale);
}

const CHECKIN_MESSAGE_PLACEHOLDER_LABEL: FrozenLabel = { en: "Check-in message", de: "Check-in-Nachricht" };
export function checkinMessagePlaceholderText(locale: string): string {
  return frozenLabelText(CHECKIN_MESSAGE_PLACEHOLDER_LABEL, locale);
}

const CHECKIN_SUBMIT_LABEL: FrozenLabel = { en: "Commit", de: "Übernehmen" };
export function checkinSubmitText(locale: string): string {
  return frozenLabelText(CHECKIN_SUBMIT_LABEL, locale);
}

const CHECKIN_CANCEL_LABEL: FrozenLabel = { en: "Cancel", de: "Abbrechen" };
export function checkinCancelText(locale: string): string {
  return frozenLabelText(CHECKIN_CANCEL_LABEL, locale);
}

/** 👁️✏️ ticket §C5 item 5 — "viewers never checkpoint": the one predicate gating BOTH the
 * `#s-checkin` affordance's presence and the auto check-in timer's arming, mirroring
 * `isMutationKindDefinition`'s "one definition of what counts as an editing verb" precedent —
 * checkpoint is exactly such a verb. `VcsArtifactApp`'s own host-side guard already rejects a
 * viewer's `CommitCheckpoint` dispatch regardless; this is the client-side mirror that keeps the
 * affordance from ever reaching the wire in the first place. */
export function canCheckIn(role: AppRole | undefined): boolean {
  return role === "editor";
}

/** 🚦️ ticket §C5 — the sync status pill's three-way vocabulary, derived from `ArtifactSyncStatus`. A
 * non-live remote takes priority over a pending-mutation count (the user needs to know the
 * connection itself is degraded before anything about local pending edits); `null` (no status event
 * observed yet, e.g. before the document's first `open`) reads as `remote: "detached"`. */
export type SyncPillState = { readonly kind: "persisted" } | { readonly kind: "pending"; readonly count: number } | { readonly kind: "remote"; readonly remote: "connected" | "connecting" | "backoff" | "detached" };

export function computeSyncPillState(status: ArtifactSyncStatus | null): SyncPillState {
  if (!status || status.remote.kind !== "live") {
    return { kind: "remote", remote: !status ? "detached" : status.remote.kind === "live" ? "connected" : status.remote.kind };
  }
  if (status.pendingMutations > 0) return { kind: "pending", count: status.pendingMutations };
  return { kind: "persisted" };
}

const SYNC_STATUS_PERSISTED_LABEL: FrozenLabel = { en: "Persisted", de: "Gespeichert" };
const SYNC_STATUS_PENDING_LABEL: FrozenLabel = { en: "Pending", de: "Ausstehend" };
const SYNC_STATUS_REMOTE_LABEL: FrozenLabel = { en: "Remote", de: "Remote" };
const SYNC_STATUS_REMOTE_STATE_LABEL: Readonly<Record<"connected" | "connecting" | "backoff" | "detached", FrozenLabel>> = {
  connected: { en: "connected", de: "verbunden" },
  connecting: { en: "connecting", de: "verbindet" },
  backoff: { en: "backoff", de: "erneuter Versuch" },
  detached: { en: "detached", de: "getrennt" },
};

/** 🚦️ Localized pill text for `#s-sync-status` — matches contract §C5's own vocabulary (`persisted |
 * pending(n) | remote(connected|connecting|backoff|detached)`) closely enough that a state can be
 * read back off the rendered string in either locale, not just English. */
export function syncPillText(state: SyncPillState, locale: string): string {
  if (state.kind === "persisted") return frozenLabelText(SYNC_STATUS_PERSISTED_LABEL, locale);
  if (state.kind === "pending") return `${frozenLabelText(SYNC_STATUS_PENDING_LABEL, locale)} (${state.count})`;
  return `${frozenLabelText(SYNC_STATUS_REMOTE_LABEL, locale)}: ${frozenLabelText(SYNC_STATUS_REMOTE_STATE_LABEL[state.remote], locale)}`;
}
//#endregion 🔖️CheckInAndSyncStatus

/** @emoji 🧭️ The five panel tabs the framework itself owns (never app-supplied) — routed through the typed chrome schema instead of the plugin overlay so a locale-locked shell can never show their English manifest label. */
const FRAMEWORK_PANEL_TAB_LABEL_KEYS: Readonly<Record<string, UiTranslationKey>> = {
  [FRAMEWORK_PANEL_TAB_ARTIFACT_ID]: "ui.panel.artifact",
  [FRAMEWORK_PANEL_TAB_CATALOGUE_ID]: "ui.panel.catalogue",
  [FRAMEWORK_PANEL_TAB_INSPECTION_ID]: "ui.panel.inspection",
  [FRAMEWORK_PANEL_TAB_PARAMETERS_ID]: "ui.panel.parameters",
  [FRAMEWORK_PANEL_TAB_HISTORY_ID]: "ui.panel.history",
};

/** @emoji 🧭️ Framework-owned panel tabs resolve through the chrome schema (`shellLabel`); every other app-declared tab still resolves through the plugin overlay (`resolveAppLabel`). */
export function resolvePanelTabLabel(overlay: PluginAppLabelsOverlay, tabId: string, fallback: string): string {
  const chromeKey = FRAMEWORK_PANEL_TAB_LABEL_KEYS[tabId];
  return chromeKey ? shellLabel(chromeKey) : resolveAppLabel(overlay, "panelTab", tabId, fallback);
}

/** @emoji 🗣️ Stable empty overlay reference so components depending on it don't re-render before the first `appLabels` fetch resolves. */
export const EMPTY_APP_LABELS_OVERLAY: PluginAppLabelsOverlay = {
  windowKindLabels: {},
  panelTabLabels: {},
  modeLabels: {},
  actionLabels: {},
  utilityLabels: {},
  exampleLabels: {},
  actionArgLabels: {},
  dialogLabels: {},
  introductionLabels: {},
  groupLabels: {},
};

/** 🗺️ Synthesizes a full `LocalizedLabel` matrix from a user-authored string by broadcasting it across all cells (native/reuse × en/de), matching Rust's `LocalizedLabel::data(...)`. Also accepts an existing `LocalizedLabel` idempotently. */
export function synthesizeLocalizedLabel(label: string | LocalizedLabel): LocalizedLabel {
  if (typeof label !== "string") return label;
  return {
    native: { en: label, de: label },
    reuse: { en: label, de: label },
  };
}

/** 🗺️ Resolves a manifest label field for the active terminology/locale. Every app-manifest struct's
 * `label`/`title`/`body`/`submitLabel`/`cancelLabel`/`description`/`text` field is now Rust's
 * `LocalizedLabel` on the wire — a `{ native: { en, de }, reuse: { en, de } }` matrix — instead of the
 * plain string these fields used to be. Falls back gracefully (reuse→native, missing locale→en, missing
 * entirely→"") so a stale/partial payload never throws; also tolerates a bare `string` defensively since
 * the ts-rs mirror for these fields is still `unknown`/stale (see `framework/core/rs/lib.rs`'s
 * `LocalizedLabel` follow-up notes) — some call sites may still see the pre-migration shape until that
 * typegen lands. */
export function resolveManifestLabel(label: LocalizedLabel | string | undefined, terminology: string, locale: string): string {
  if (label === undefined) return "";
  if (typeof label === "string") return label;
  const byTerminology = label[terminology as keyof LocalizedLabel] ?? label.native ?? label.reuse;
  if (!byTerminology) return "";
  return byTerminology[locale as keyof typeof byTerminology] ?? byTerminology.en ?? Object.values(byTerminology)[0] ?? "";
}

/** @emoji 🗣️ Resolves a window-kind/panel-tab/mode/action/utility/example/actionArg/dialog/introduction/group id's locale-aware label from the active app's overlay, falling back to the static manifest label. */
export function resolveAppLabel(overlay: PluginAppLabelsOverlay, kind: "windowKind" | "panelTab" | "mode" | "action" | "utility" | "example" | "actionArg" | "dialog" | "introduction" | "group", id: string, fallback: string): string {
  const map =
    kind === "windowKind"
      ? overlay.windowKindLabels
      : kind === "panelTab"
        ? overlay.panelTabLabels
        : kind === "mode"
          ? overlay.modeLabels
          : kind === "action"
            ? overlay.actionLabels
            : kind === "utility"
              ? overlay.utilityLabels
              : kind === "example"
                ? overlay.exampleLabels
                : kind === "actionArg"
                  ? overlay.actionArgLabels
                  : kind === "dialog"
                    ? overlay.dialogLabels
                    : kind === "introduction"
                      ? overlay.introductionLabels
                      : overlay.groupLabels;
  return map[id] ?? fallback;
}

/** @emoji 🗣️ Resolves one action-arg's label + (for `select` controls) its options' labels from the overlay's `actionArgLabels` map, keyed `"{scopeId}.{argId}"` / `"{scopeId}.{argId}.option.{value}"`. `scopeId` is an action id for staged/palette forms, a dialog id for dialog args, or a command id for command args. `ActionArgDef.label`/`ActionArgOption.label` are manifest `LocalizedLabel` fields, resolved for `terminology`/`locale` before the overlay's (always-empty, see the `AppLabelsOverlay` deletion note) fallback lookup even applies. */
function resolveActionArgDef(def: ActionArgDef, scopeId: string, overlay: PluginAppLabelsOverlay, terminology: string, locale: string): ActionArgDef {
  const label = resolveAppLabel(overlay, "actionArg", `${scopeId}.${def.id}`, resolveManifestLabel(def.label, terminology, locale));
  // 🎫️ D6: `argControl(def).kind !== "select"` replaces the old `def.control.kind` check — a
  // `select` control is now derived from `def.schema` (a `string` schema with non-empty `options`),
  // so the options being resolved below live at `def.schema.options`, not on a stored `control`.
  if (argControl(def).kind !== "select" || def.schema.kind !== "string") return label === def.label ? def : { ...def, label };
  const options = def.schema.options.map((option) => ({ ...option, label: resolveAppLabel(overlay, "actionArg", `${scopeId}.${def.id}.option.${option.value}`, resolveManifestLabel(option.label, terminology, locale)) }));
  return { ...def, label, schema: { ...def.schema, options } };
}

/** @emoji 🗣️ Resolves a `DialogDefinition`'s title/body/submitLabel/cancelLabel/args from the overlay's `dialogLabels`/`actionArgLabels` maps, keyed by the dialog's own id. `title`/`body`/`submitLabel`/`cancelLabel` are all manifest `LocalizedLabel` fields. */
export function resolveDialogDefinition(dialog: DialogDefinition, overlay: PluginAppLabelsOverlay, terminology: string, locale: string): DialogDefinition {
  return {
    ...dialog,
    title: resolveAppLabel(overlay, "dialog", `${dialog.id}.title`, resolveManifestLabel(dialog.title, terminology, locale)),
    body: dialog.body ? resolveAppLabel(overlay, "dialog", `${dialog.id}.body`, resolveManifestLabel(dialog.body, terminology, locale)) : dialog.body,
    submitLabel: resolveAppLabel(overlay, "dialog", `${dialog.id}.submit`, resolveManifestLabel(dialog.submitLabel, terminology, locale)),
    cancelLabel: dialog.cancelLabel ? resolveAppLabel(overlay, "dialog", `${dialog.id}.cancel`, resolveManifestLabel(dialog.cancelLabel, terminology, locale)) : dialog.cancelLabel,
    args: dialog.args.map((def) => resolveActionArgDef(def, dialog.id, overlay, terminology, locale)),
  };
}

/** @emoji 🗣️ Resolves an `IntroductionDefinition`'s title and every step's title/body labels from the
 * overlay's `introductionLabels` map. `title`/`body` are manifest `LocalizedLabel` fields;
 * `IntroductionInteraction.label` is a short checklist caption that is still a plain `String` on the Rust
 * side (not part of the `LocalizedLabel` migration), so it is left as-is. */
export function resolveIntroductionDefinition(introduction: IntroductionDefinition, overlay: PluginAppLabelsOverlay, terminology: string, locale: string): IntroductionDefinition {
  return {
    title: resolveAppLabel(overlay, "introduction", "intro.title", resolveManifestLabel(introduction.title, terminology, locale)),
    steps: introduction.steps.map(
      (step): IntroductionStepDefinition => ({
        ...step,
        title: resolveAppLabel(overlay, "introduction", `intro.step.${step.id}.title`, resolveManifestLabel(step.title, terminology, locale)),
        body: resolveAppLabel(overlay, "introduction", `intro.step.${step.id}.body`, resolveManifestLabel(step.body, terminology, locale)),
        interactions: (step.interactions ?? []).map((interaction, index) => ({
          ...interaction,
          label: resolveAppLabel(overlay, "introduction", `intro.step.${step.id}.interaction.${index}.label`, interaction.label),
        })),
        ordered: step.ordered ?? false,
      }),
    ),
  };
}

//#region 🎥️TutorialUiBridge
/** @emoji 🎥️ Captures the shell's current `ShellState` (+ active session) as a renderer-neutral `TutorialUiSnapshot` — the recorder's periodic full-snapshot keyframes and the `TutorialBar`'s "record" path both call this. See the Rust doc comment on `TutorialUiSnapshot` for why this is deliberately NOT a serialization of `ShellState` itself. */
export function captureTutorialUiSnapshot(state: ShellState, session: ActiveSession | null): TutorialUiSnapshot {
  const activeUtilityByWindowId: Record<string, string> = {};
  for (const [windowId, utilityId] of Object.entries(state.actionPane.activeUtilityByWindowId)) {
    if (utilityId) activeUtilityByWindowId[windowId] = utilityId;
  }
  const activePanelTabByGroup: Record<string, string> = {};
  for (const anchor of ANCHORS) {
    const panelState = state.layout.panels[anchor];
    const tabId = panelState.path[panelState.path.length - 1];
    if (panelState.visible && tabId) activePanelTabByGroup[anchor] = tabId;
  }
  return {
    activeModeId: session?.viewState.activeModeId,
    focusedWindowId: state.layout.activeWindowId ?? undefined,
    activeUtilityByWindowId,
    activeToolId: state.actionPane.activeToolId ?? undefined,
    layout: captureCurrentFrameworkLayout(state.layout.shellLayout, state.layout.extraWindowInstances),
    activePanelTabByGroup,
    panelJson: session?.viewState.panelJson,
    selectionJson: session?.viewState.selectionJson,
    openDialogId: state.overlays.dialog?.dialogId,
    expandedTreeIds: Object.entries(state.layout.treeOpenStates).filter(([, open]) => open).map(([id]) => id),
    commandPanelOpen: state.overlays.searchOpen,
  };
}

/** @emoji 🎥️ Context every `applyTutorialUiSnapshotToShell`/`applyTutorialUiChangeToShell` call needs beyond `dispatch` itself — resolved once per render by the caller (the director/seek/deviation-converge paths all share it). */
type TutorialUiBridgeContext = {
  readonly session: ActiveSession | null;
  readonly appLabelsOverlay: PluginAppLabelsOverlay;
  readonly terminology: string;
  readonly locale: string;
};

/** @emoji 🎥️ Applies a full `TutorialUiSnapshot` (a `TutorialUiSample::Snapshot`, or the composed target of a seek/deviation-converge) onto the live `ShellState` — snaps every field instantly (camera is the only interpolated track, applied separately by the director). Dispatches the atomic `APPLY_TUTORIAL_UI_SNAPSHOT` for everything resolvable purely from `ShellState`, plus one `SET_SESSION` for the fields that live on `ActiveSession.viewState` (`activeModeId`/`panelJson`/`selectionJson`). */
export function applyTutorialUiSnapshotToShell(dispatch: (action: ShellAction) => void, snapshot: TutorialUiSnapshot, ctx: TutorialUiBridgeContext): void {
  const windowKinds = ctx.session?.app.windowKinds.map((kind) => ({ id: kind.id, label: kind.label })) ?? [];
  const seed = applyFrameworkLayoutSeed(snapshot.layout, windowKinds, ctx.appLabelsOverlay, ctx.terminology, ctx.locale);
  const panelPatches: Partial<Record<Anchor, { readonly visible: boolean; readonly path: readonly string[] }>> = {};
  for (const anchor of ANCHORS) {
    const tabId = snapshot.activePanelTabByGroup[anchor];
    panelPatches[anchor] = tabId ? { visible: true, path: [tabId] } : { visible: false, path: [] };
  }
  const treeOpenStates: Record<string, boolean> = {};
  for (const id of snapshot.expandedTreeIds) treeOpenStates[id] = true;
  dispatch({
    type: "APPLY_TUTORIAL_UI_SNAPSHOT",
    snapshot: {
      activeWindowId: snapshot.focusedWindowId ?? null,
      shellLayout: seed.modeLayout,
      extraWindowInstances: seed.extraInstances,
      panelPatches,
      treeOpenStates,
      activeUtilityByWindowId: snapshot.activeUtilityByWindowId,
      activeToolId: snapshot.activeToolId ?? null,
      openDialogId: snapshot.openDialogId ?? null,
      commandPanelOpen: snapshot.commandPanelOpen,
    },
  });
  if (ctx.session) {
    dispatch({
      type: "SET_SESSION",
      value: (current) =>
        current
          ? {
              ...current,
              viewState: {
                ...current.viewState,
                activeModeId: snapshot.activeModeId ?? current.viewState.activeModeId,
                panelJson: snapshot.panelJson ?? current.viewState.panelJson,
                selectionJson: snapshot.selectionJson ?? current.viewState.selectionJson,
              },
            }
          : current,
    });
  }
}

/** @emoji 🎥️ Applies one sparse `TutorialUiChange` (a `TutorialUiSample::Delta` entry, replayed by the director's per-tick `tutorialSlice`) onto the live `ShellState` by dispatching the SAME existing, targeted `ShellAction`s the real UI's own interactions use — never a bespoke tutorial-only mutation channel. */
export function applyTutorialUiChangeToShell(dispatch: (action: ShellAction) => void, change: TutorialUiChange, ctx: TutorialUiBridgeContext): void {
  switch (change.kind) {
    case "activeMode":
      if (!ctx.session) return;
      dispatch({ type: "SET_SESSION", value: (current) => (current ? { ...current, viewState: { ...current.viewState, activeModeId: change.id } } : current) });
      return;
    case "focusedWindow":
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: change.id ?? null });
      return;
    case "activeUtility":
      dispatch({ type: "SET_ACTIVE_UTILITY", windowId: change.windowId, utilityId: change.utilityId ?? null });
      return;
    case "activeTool":
      dispatch({ type: "SET_ACTIVE_TOOL", toolId: change.id ?? null });
      return;
    case "layout": {
      const windowKinds = ctx.session?.app.windowKinds.map((kind) => ({ id: kind.id, label: kind.label })) ?? [];
      const seed = applyFrameworkLayoutSeed(change.layout, windowKinds, ctx.appLabelsOverlay, ctx.terminology, ctx.locale);
      dispatch({ type: "SET_SHELL_LAYOUT", value: seed.modeLayout });
      dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seed.extraInstances });
      return;
    }
    case "panelTab": {
      const anchor = change.group as Anchor;
      if (!(ANCHORS as readonly string[]).includes(anchor)) return;
      dispatch({ type: "SET_PANEL_VISIBLE", anchor, value: change.tabId != null });
      dispatch({ type: "SET_PANEL_PATH", anchor, value: change.tabId ? [change.tabId] : [] });
      return;
    }
    case "panelState":
      if (!ctx.session) return;
      dispatch({ type: "SET_SESSION", value: (current) => (current ? { ...current, viewState: { ...current.viewState, panelJson: change.panelJson } } : current) });
      return;
    case "selection":
      if (!ctx.session) return;
      dispatch({ type: "SET_SESSION", value: (current) => (current ? { ...current, viewState: { ...current.viewState, selectionJson: change.selectionJson } } : current) });
      return;
    case "dialog":
      dispatch({ type: "SET_DIALOG", value: change.id ? { dialogId: change.id, seedArgs: change.args as Record<string, unknown> | undefined } : null });
      return;
    case "treeExpansion":
      dispatch({ type: "SET_TREE_OPEN_STATE", id: change.id, open: change.expanded });
      return;
    case "commandPanel":
      dispatch({ type: "SET_SEARCH_OPEN", value: change.open });
      return;
    default:
      return;
  }
}
//#endregion 🎥️TutorialUiBridge

/** @emoji 🗣️ Resolves a terminology id's display name; chrome-known ids get a translated label, app-declared ids fall back to their raw id. */
export function shellTerminologyLabel(id: string): string {
  const isChromeKnown = id === "native" || id === "reuse";
  return isChromeKnown ? shellLabel(`ui.settings.terminology.${id as UiChromeTerminologyId}`) : id;
}

/** @emoji 🎚️ Serializes async updates while retaining only the newest value requested during an in-flight update. */
export function createLatestAsyncDispatcher<T>(dispatchValue: (value: T) => unknown): (value: T) => void {
  let running = false;
  let queued: T | undefined;
  let hasQueued = false;
  const dispatchLatest = (value: T) => {
    if (running) {
      queued = value;
      hasQueued = true;
      return;
    }
    running = true;
    void Promise.resolve(dispatchValue(value)).finally(() => {
      running = false;
      if (!hasQueued) return;
      const next = queued as T;
      queued = undefined;
      hasQueued = false;
      dispatchLatest(next);
    });
  };
  return dispatchLatest;
}

/** @emoji ↕️ Serializes numeric slider updates while retaining every direction change and coalescing movement within one direction. */
export function createDirectionalAsyncDispatcher(dispatchValue: (value: number) => unknown): (value: number) => void {
  let running = false;
  let active = 0;
  const queued: number[] = [];
  const dispatchNext = (value: number) => {
    running = true;
    active = value;
    void Promise.resolve(dispatchValue(value)).finally(() => {
      const next = queued.shift();
      if (next === undefined) {
        running = false;
        return;
      }
      dispatchNext(next);
    });
  };
  return (value) => {
    if (!running) {
      dispatchNext(value);
      return;
    }
    const previous = queued.at(-1);
    if (previous === undefined) {
      if (value !== active) queued.push(value);
      return;
    }
    const anchor = queued.at(-2) ?? active;
    const direction = Math.sign(previous - anchor);
    const nextDirection = Math.sign(value - previous);
    if (nextDirection === 0) return;
    if (direction === 0 || nextDirection === direction) queued[queued.length - 1] = value;
    else queued.push(value);
    // 🔁️ A jittery drag (rapid direction reversals while a round trip is in flight) would otherwise grow
    // `queued` by one entry per reversal; only the last two are ever needed (the pending value and the
    // anchor used to detect the next reversal), so cap it there.
    if (queued.length > 2) queued.splice(0, queued.length - 2);
  };
}

//#region RevealCutoffStore
/**
 * @emoji 🪣️ Live per-gesture visibility cutoff for reveal-tagged instances (`WorldInstanceRecord.revealIndex`,
 * set by a `WindowMeasure.Slider.reveal` group). Main-thread-only and never dispatched: a slider drag writes
 * here directly, `WorldInstancesLayer` subscribes and imperatively toggles `Object3D.visible` — zero React
 * re-render, zero WASM round trip. Reconciled from the plugin's committed `WorldInteractionRecord.revealCutoffs`
 * whenever that value changes (a no-operation during a live drag, since the committed value only changes on commit).
 */
export type RevealCutoffStore = {
  get(groupId: string): number | undefined;
  set(groupId: string, value: number): void;
  subscribe(groupId: string, listener: (value: number | undefined) => void): () => void;
};

export function createRevealCutoffStore(): RevealCutoffStore {
  const values = new Map<string, number>();
  const listeners = new Map<string, Set<(value: number | undefined) => void>>();
  return {
    get: (groupId) => values.get(groupId),
    set: (groupId, value) => {
      values.set(groupId, value);
      for (const listener of listeners.get(groupId) ?? []) listener(value);
    },
    subscribe: (groupId, listener) => {
      let group = listeners.get(groupId);
      if (!group) {
        group = new Set();
        listeners.set(groupId, group);
      }
      group.add(listener);
      return () => {
        group!.delete(listener);
      };
    },
  };
}

/** Shared instance — a reveal group id is app-instance-global in v1; namespace by app instance id if a second concurrent document instance ever needs independent cutoffs. */
export const worldRevealCutoffStore = createRevealCutoffStore();

/** The only reveal group that exists today — puzzle3d's fill-plan slider. */
export const PUZZLE3D_FILL_REVEAL_GROUP_ID = "puzzle3d-fill";

/**
 * @emoji 🪣️ Writes committed reveal cutoffs into `store` only when the numeric value for a group changes.
 * Ignores object-identity churn from `fillBuildTick` refreshes so a live slider drag is not reset mid-gesture.
 */
export function reconcileCommittedRevealCutoffs(
  store: RevealCutoffStore,
  committedRef: { current: Readonly<Record<string, number>> },
  revealCutoffs: Readonly<Record<string, number>>,
): void {
  for (const [groupId, value] of Object.entries(revealCutoffs)) {
    if (committedRef.current[groupId] === value) continue;
    committedRef.current = { ...committedRef.current, [groupId]: value };
    store.set(groupId, value);
  }
}

/** @emoji 🙈️ True for a reveal-tagged instance beyond the live cutoff — `WorldInstancesLayer` already
 * hides its root imperatively, but pure functions that read `instances` data directly (marquee hit
 * testing) don't see three.js `Object3D.visible` and need this check instead. Untagged instances are
 * never cutoff-hidden: the nullish guard also rejects a JSON `null`, which would otherwise compare as `0`
 * and hide every ordinary object while the cutoff sits at its boot value. */
export function isRevealCutoffHidden(instance: Pick<WorldInstanceRecord, "revealIndex">): boolean {
  if (instance.revealIndex == null) return false;
  const cutoff = worldRevealCutoffStore.get(PUZZLE3D_FILL_REVEAL_GROUP_ID);
  return cutoff !== undefined && instance.revealIndex >= cutoff;
}
//#endregion RevealCutoffStore

/**
 * @emoji 🚦️ Fires `run` at most once at a time — interval ticks that arrive while a previous run is still
 * in flight are dropped (not queued). Used by World3dHost's `suggestionsTick`/`fillBuildTick` loops so a
 * slow program tick cannot unbounded-queue into the serialized WASM handle and starve the fill utility.
 */
export function createInFlightSkippingInterval(run: () => unknown, delayMs: number, setIntervalFn: typeof setInterval = setInterval, clearIntervalFn: typeof clearInterval = clearInterval): () => void {
  let cancelled = false;
  let inFlight = false;
  const tick = () => {
    if (cancelled || inFlight) return;
    inFlight = true;
    void Promise.resolve(run()).finally(() => {
      inFlight = false;
    });
  };
  const timer = setIntervalFn(tick, delayMs);
  return () => {
    cancelled = true;
    clearIntervalFn(timer);
  };
}

/**
 * @emoji 🎯️ Coalesces rapid dispatches to the latest value — skips when unchanged and keeps at most one
 * in-flight round trip (used by World3dHost hover so pointermove cannot flood the WASM handle).
 */
export function createCoalescingActionDispatcher<T>(dispatch: (value: T) => unknown, isEqual: (a: T, b: T) => boolean = (a, b) => Object.is(a, b)): (value: T) => void {
  let inFlight = false;
  let pending: T | undefined;
  let lastSent: T | undefined;
  const flush = () => {
    if (inFlight || pending === undefined) return;
    const next = pending;
    pending = undefined;
    if (lastSent !== undefined && isEqual(lastSent, next)) return;
    lastSent = next;
    inFlight = true;
    void Promise.resolve(dispatch(next)).finally(() => {
      inFlight = false;
      flush();
    });
  };
  return (value: T) => {
    if (pending === undefined && lastSent !== undefined && isEqual(lastSent, value)) return;
    pending = value;
    flush();
  };
}

export const registeredPuzzle3dBrushMeshes = new Set<string>();

/** @emoji 🎚️ Whether any measure (including nested group children) declares `id`. */
export function windowMeasureTreeContainsId(measures: readonly WindowMeasure[], id: string): boolean {
  for (const measure of measures) {
    if (measure.id === id) return true;
    if (measure.kind === "group" && windowMeasureTreeContainsId(measure.children, id)) return true;
  }
  return false;
}

/** @emoji 📊️ Probability weights (0–1 simplex sliders) read out as whole-percent labels, not raw fractions. */
function windowMeasureUsesProbabilityReadout(measure: Extract<WindowMeasure, { kind: "slider" }>): boolean {
  const step = measure.step ?? 1;
  return measure.min === 0 && measure.max <= 1 && step < 1;
}

function windowMeasureProbabilityReadout(value: number): string {
  return `${Math.round(value * 100)}%`;
}

/** @emoji 🎚️ Keeps a measure slider live without accumulating stale document actions behind the pointer. */
function WindowMeasureSlider({ measure, onAction }: { readonly measure: Extract<WindowMeasure, { kind: "slider" }>; readonly onAction: (action: ActionDescriptor) => unknown }) {
  const dispatchValue = useMemo(
    () => createDirectionalAsyncDispatcher((value) => onAction({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), value } })),
    [measure.onChange, onAction],
  );
  const formatDisplayValue = windowMeasureUsesProbabilityReadout(measure) ? windowMeasureProbabilityReadout : undefined;
  const disabled = measure.disabled === true;
  // 🪣️ A reveal-group measure (e.g. puzzle3d's fill-count slider) must not round-trip through WASM on
  // every drag value — the plugin already rendered every planned piece tagged with its reveal index, so
  // dragging only needs to move a main-thread visibility cutoff. Only the final value round-trips, once,
  // on gesture release.
  const revealGroupId = measure.reveal;

  return (
    <Slider
      id={measure.id}
      value={[measure.value]}
      min={measure.min}
      max={measure.max}
      ready={measure.ready}
      loading={measure.loading === true}
      waiting={measure.waiting === true}
      step={measure.step}
      disabled={disabled}
      clampToReady={Boolean(revealGroupId)}
      formatDisplayValue={formatDisplayValue}
      onValueChange={(values) => {
        if (disabled) return;
        const value = values[0] ?? measure.value;
        if (revealGroupId) {
          worldRevealCutoffStore.set(revealGroupId, value);
          return;
        }
        dispatchValue(value);
      }}
      onValueCommit={
        revealGroupId
          ? (values) => {
              if (disabled) return;
              const value = values[0] ?? measure.value;
              worldRevealCutoffStore.set(revealGroupId, value);
              onAction({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), value } });
            }
          : undefined
      }
      onPointerCancel={revealGroupId ? () => worldRevealCutoffStore.set(revealGroupId, measure.value) : undefined}
    />
  );
}

function windowMeasureGroupHeaderSlider(measure: Extract<WindowMeasure, { kind: "group" }>, onAction: (action: ActionDescriptor) => unknown): ReactNode | undefined {
  if (measure.value === undefined || measure.onChange === undefined) return undefined;
  const sliderMeasure: Extract<WindowMeasure, { kind: "slider" }> = {
    kind: "slider",
    id: `${measure.id}.header-slider`,
    label: undefined,
    value: measure.value,
    min: measure.min ?? 0,
    max: measure.max ?? 1,
    step: measure.step,
    ready: measure.ready,
    loading: measure.loading,
    waiting: measure.waiting,
    onChange: measure.onChange,
  };
  return <WindowMeasureSlider measure={sliderMeasure} onAction={onAction} />;
}

function windowMeasureSelectControl(measure: Extract<WindowMeasure, { kind: "select" }>, onAction: (action: ActionDescriptor) => unknown): ReactNode {
  return (
    <Select value={measure.value} onValueChange={(value) => onAction({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), value } })}>
      <SelectTrigger id={measure.id} className="h-small w-full min-w-0" size="sm">
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        {measure.items.map((item) => (
          <SelectItem key={item.id} value={item.value}>
            {item.label}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}

function windowMeasureToggleControl(measure: Extract<WindowMeasure, { kind: "toggle" }>, onAction: (action: ActionDescriptor) => unknown): ReactNode {
  const label = measure.label ?? measure.text ?? measure.id;
  return (
    <TreeCheckbox
      id={measure.id}
      checked={measure.pressed}
      title={label}
      ariaLabel={label}
      onCheckedChange={(pressed) => onAction({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), pressed } })}
    />
  );
}

function windowMeasureToggleIcon(measure: Extract<WindowMeasure, { kind: "toggle" }>): ReactNode {
  return <Icon icon={measure.iconId as IconName} size={12} />;
}

/**
 * 🌲️ Maps window measures to native panel-tree rows — same chrome as left-corner trees (label left, control right, guide lines).
 * Pre-reverses top-level measures so bottom-anchored `direction="up"` panels read Count at the bottom, Distribution above.
 */
function windowMeasuresToTreeItems(measures: readonly WindowMeasure[], onAction: (action: ActionDescriptor) => unknown, reverseForUpPanel = true): TreeDataItem[] {
  const ordered = reverseForUpPanel ? [...measures].reverse() : [...measures];
  const mapMeasure = (measure: WindowMeasure): TreeDataItem => {
    if (measure.kind === "group") {
      return {
        id: measure.id,
        label: measure.label,
        defaultOpen: measure.defaultOpen,
        control: windowMeasureGroupHeaderSlider(measure, onAction),
        items: measure.children.length > 0 ? windowMeasuresToTreeItems(measure.children, onAction, false) : undefined,
      };
    }
    if (measure.kind === "slider") {
      return {
        id: measure.id,
        label: measure.label ?? "",
        control: <WindowMeasureSlider measure={measure} onAction={onAction} />,
        loading: measure.loading,
        waiting: measure.waiting,
      };
    }
    if (measure.kind === "select") {
      return {
        id: measure.id,
        label: measure.label ?? "",
        control: windowMeasureSelectControl(measure, onAction),
      };
    }
    return {
      id: measure.id,
      label: measure.label ?? measure.text ?? "",
      icon: windowMeasureToggleIcon(measure),
      control: windowMeasureToggleControl(measure, onAction),
    };
  };
  return ordered.map(mapMeasure);
}

function renderWindowMeasure(measure: WindowMeasure, onAction: (action: ActionDescriptor) => unknown): ReactNode {
  if (measure.kind === "group") {
    const headerSlider = windowMeasureGroupHeaderSlider(measure, onAction);
    return (
      <WindowMeasureTreeGroup key={measure.id} id={measure.id} label={measure.label} defaultOpen={measure.defaultOpen} headerControl={headerSlider}>
        {measure.children.map((child) => renderWindowMeasure(child, onAction))}
      </WindowMeasureTreeGroup>
    );
  }
  if (measure.kind === "select") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label}>
        {windowMeasureSelectControl(measure, onAction)}
      </WindowMeasureTreeLeaf>
    );
  }
  if (measure.kind === "slider") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label}>
        <WindowMeasureSlider measure={measure} onAction={onAction} />
      </WindowMeasureTreeLeaf>
    );
  }
  if (measure.kind === "toggle") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label ?? measure.text} icon={windowMeasureToggleIcon(measure)}>
        {windowMeasureToggleControl(measure, onAction)}
      </WindowMeasureTreeLeaf>
    );
  }
  return null;
}

function windowMeasuresOverlay(measures: readonly WindowMeasure[] | undefined, onAction: (action: ActionDescriptor) => unknown, direction: "up" | "down" = "down"): ReactNode | undefined {
  if (!measures || measures.length === 0) return undefined;
  return <WindowMeasuresTree direction={direction}>{measures.map((measure) => renderWindowMeasure(measure, onAction))}</WindowMeasuresTree>;
}

/** @emoji 🪟️ Public window-options tree for measures rails and tests — icon before label, checkbox for toggles. */
export function renderWindowMeasuresTree(measures: readonly WindowMeasure[], onAction: (action: ActionDescriptor) => unknown, direction: "up" | "down" = "down"): ReactNode | undefined {
  return windowMeasuresOverlay(measures, onAction, direction);
}

export function SelectionUtilityOptions({ activeUtilityId, windowId, onAction }: { readonly activeUtilityId: string | undefined; readonly windowId: string; readonly onAction: (action: ActionDescriptor) => void }) {
  const methodLabel = useLabel("ui.selection.method");
  const modeLabel = useLabel("ui.selection.mode");
  const rectangleLabel = useLabel("ui.selection.rectangle");
  const lassoLabel = useLabel("ui.selection.lasso");
  const selectiveLabel = useLabel("ui.selection.selective");
  const additiveLabel = useLabel("ui.selection.additive");
  const subtractiveLabel = useLabel("ui.selection.subtractive");
  const invertiveLabel = useLabel("ui.selection.invertive");
  const selectionMethod = activeUtilityId === "selectLasso" ? "lasso" : "rectangle";
  // 🐚️ Replaces the old `(globalThis).__selectionMode` + `window` `"semio:selectionOptionsChanged"`
  // broadcast — this shell's own store, so its selection-mode toggle never reconfigures another mounted
  // shell's marquee gestures.
  const selectionStore = useShellScope().selection;

  const [selectionMode, setSelectionMode] = useState<MergeMode>(() => selectionStore.get());

  const handleModeChange = (mode: MergeMode) => {
    selectionStore.set(mode);
    setSelectionMode(mode);
  };

  const handleMethodChange = (method: "rectangle" | "lasso") => {
    onAction({
      controllerId: "window",
      action: SET_ACTIVE_UTILITY_ACTION_ID,
      args: { windowId, utilityId: method === "lasso" ? "selectLasso" : "selectMarquee" },
    });
  };

  return (
    <div className="flex items-center gap-double">
      <div className="flex items-center gap-single">
        <span className="text-tiny text-muted-foreground uppercase tracking-wider font-semibold">{methodLabel}</span>
        <ToggleGroup
          kind="single"
          value={selectionMethod}
          onValueChange={(val) => {
            if (val === "rectangle" || val === "lasso") {
              handleMethodChange(val);
            }
          }}
          items={[
            { value: "rectangle", icon: <Icon icon="square-dashed" size="small" />, text: rectangleLabel },
            { value: "lasso", icon: <Icon icon="lasso" size="small" />, text: lassoLabel },
          ]}
        />
      </div>
      <RibbonDivider />
      <div className="flex items-center gap-single">
        <span className="text-tiny text-muted-foreground uppercase tracking-wider font-semibold">{modeLabel}</span>
        <ToggleGroup
          kind="single"
          value={selectionMode}
          onValueChange={(val) => {
            if (val === "replace" || val === "additive" || val === "subtractive" || val === "invertive") {
              handleModeChange(val);
            }
          }}
          items={[
            { value: "replace", text: selectiveLabel },
            { value: "additive", text: additiveLabel },
            { value: "subtractive", text: subtractiveLabel },
            { value: "invertive", text: invertiveLabel },
          ]}
        />
      </div>
    </div>
  );
}

export function windowMeasuresChrome(
  measures: readonly WindowMeasure[] | undefined,
  activeUtilityId: string | undefined,
  windowId: string,
  onAction: (action: ActionDescriptor) => unknown,
): { readonly measures: ReactNode | undefined; readonly utilityOptions: ReactNode | undefined } {
  const { general, utilityOptions } = partitionWindowMeasures(measures ?? [], activeUtilityId);
  // 🪟️ Stamps this chrome's owning `windowId` onto every measure action, mirroring `tagSetActiveUtilityWindow`
  // for the utility bar — the generic `onAction` dispatch path reads it back out to target the plugin call's
  // `view_state.windowId`, so a grid/LOD/selection toggle only ever mutates ITS OWN window's options.
  const taggedOnAction = (action: ActionDescriptor) => onAction({ ...action, args: { ...(action.args as object | undefined), windowId } });
  return {
    measures: windowMeasuresOverlay(general, taggedOnAction),
    utilityOptions: windowMeasuresOverlay(utilityOptions, taggedOnAction, "up"),
  };
}

/** @emoji 🎓️ Whether a utility node tree has a node (leaf or group) with the given id anywhere in it — used
 * to decide if this window's utility bar is the one an introduction step's `Utility` anchor targets. */
export function utilityNodeTreeContainsId(nodes: readonly UtilityNode[], targetId: string): boolean {
  return nodes.some((node) => node.id === targetId || (node.kind === "collection" && utilityNodeTreeContainsId(node.children, targetId)));
}

export function utilityBarNode(utilities: readonly UtilityNode[] | undefined, windowId: string, onAction: (action: ActionDescriptor) => void, revealUtilityId?: string | null, utilityOptions?: ReactNode): ReactNode {
  if (!utilities?.length && !utilityOptions) return undefined;
  const categories = groupUtilityNodesByCategory(utilities ?? [], UTILITY_CATEGORIES);
  if (!categories.length && !utilityOptions) return undefined;
  const grouped: UtilityNode[] = [];
  for (const node of categories) {
    if (node.kind === "collection" && (node.category === "utilities" || node.category === "selection")) {
      if (node.id === "group:Select" || node.id === "group:selection" || node.label === "Select" || node.text === "Select") {
        grouped.push(...node.children);
      } else {
        for (const child of node.children) {
          if (child.kind === "collection" && (child.id === "group:Select" || child.id === "group:selection" || child.label === "Select" || child.text === "Select")) {
            grouped.push(...child.children);
          } else {
            grouped.push(child);
          }
        }
      }
    } else {
      grouped.push(node);
    }
  }
  return <UtilityTree id={`ui.utilities.${windowId}`} utilities={grouped} onAction={onAction} direction="up" revealUtilityId={revealUtilityId} utilityOptions={utilityOptions} />;
}

//#region 🧰️WindowActionPane
/**
 * 🎛️ Renders one {@link ActionArgControl} into a STAGED form field — the crucial difference from
 * `renderUiControl` in `ui-interpreter.tsx` is that this dispatches NOTHING globally; `onChange` only
 * writes to the caller's local staged buffer. `value` is the already-resolved effective value
 * (staged ?? default ?? unset).
 */
export function renderStagedArgControl(def: ActionArgDef, value: unknown, onChange: (value: unknown) => void, disabled?: boolean): ReactElement {
  // 🎫️ D6: `def.control` is gone — derive it fresh via `argControl(def)` (mirrors Rust `ActionArgDef::control()`).
  const control: ActionArgControl = argControl(def);
  switch (control.kind) {
    case "text":
      return <Input id={def.id} type="text" className="h-medium w-full min-w-0" value={typeof value === "string" ? value : ""} placeholder={control.placeholder} disabled={disabled} onChange={(event) => onChange(event.target.value)} />;
    case "number":
      return (
        <Input
          id={def.id}
          type="number"
          className="h-medium w-full min-w-0"
          value={value === undefined || value === null || value === "" ? "" : String(value)}
          min={control.min}
          max={control.max}
          step={control.step}
          disabled={disabled}
          onChange={(event) => onChange(event.target.value === "" ? undefined : Number(event.target.value))}
        />
      );
    case "slider": {
      const numeric = typeof value === "number" && Number.isFinite(value) ? value : control.min;
      const slider = <Slider id={def.id} className="w-full min-w-0" min={control.min} max={control.max} step={control.step ?? 1} value={[numeric]} disabled={disabled} onValueChange={(values) => onChange(values[0] ?? numeric)} />;
      if (!control.unit) return slider;
      return (
        <div className="flex w-full min-w-0 items-center gap-single">
          {slider}
          <span className="shrink-0 text-xs tabular-nums text-muted-foreground">
            {numeric} {control.unit}
          </span>
        </div>
      );
    }
    case "toggle":
      return <Toggle id={def.id} pressed={value === true} text={def.label} disabled={disabled} onPressedChange={(pressed) => onChange(pressed)} />;
    case "select":
      return (
        <Select value={typeof value === "string" && value ? value : undefined} disabled={disabled} onValueChange={(next) => onChange(next)}>
          <SelectTrigger id={def.id} className="h-medium w-full min-w-0" size="sm">
            <SelectValue placeholder={def.label} />
          </SelectTrigger>
          <SelectContent>
            {control.options.map((option, index) => (
              <SelectItem key={`${def.id}:${index}:${option.value}`} value={option.value}>
                {option.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      );
    case "vec3": {
      const tuple = Array.isArray(value) && value.length >= 3 ? (value as readonly number[]) : null;
      const axes = ["x", "y", "z"] as const;
      return (
        <div className="grid grid-cols-3 gap-single">
          {axes.map((axis, index) => (
            <Input
              key={`${def.id}.${axis}`}
              id={`${def.id}.${axis}`}
              type="number"
              className="h-medium w-full min-w-0"
              value={tuple ? String(tuple[index] ?? 0) : ""}
              placeholder={axis}
              disabled={disabled}
              onChange={(event) => {
                const parsed = Number(event.target.value);
                if (!Number.isFinite(parsed)) return;
                const next: [number, number, number] = tuple ? [tuple[0] ?? 0, tuple[1] ?? 0, tuple[2] ?? 0] : [0, 0, 0];
                next[index] = parsed;
                onChange(next);
              }}
            />
          ))}
        </div>
      );
    }
    case "iconSelect":
      return <IconSelector id={def.id} classifyIconSelectorMode={undefined} value={typeof value === "string" ? value : ""} uniform onChange={(next) => onChange(next)} />;
    // 🎫️ D6: `artifactKind`/`surfaceApp` are HOST-resolved — the host substitutes them with a plain
    // `select` before a staged form ever renders (see `artifact_kind_choices`/`🔖️HostResolvedArgs`
    // in the Rust manifest), so neither reaches here in practice; no `ActionArgDef` builder produces
    // them today either (`artifact_kind`/`surface_app` have zero call sites repo-wide). Kept as an
    // explicit fallback purely so this switch stays exhaustive over `ActionArgControl.kind`.
    case "artifactKind":
    case "surfaceApp":
      return <Input id={def.id} type="text" className="h-medium w-full min-w-0" value={typeof value === "string" ? value : ""} disabled={disabled} onChange={(event) => onChange(event.target.value)} />;
  }
}

/** 🧰️ True when an action carries arguments and therefore stages a form instead of firing immediately (P1–P4). */
export function actionRequiresStagedForm(action: Pick<ActionDefinition, "args">): boolean {
  return (action.args?.length ?? 0) > 0;
}

/** 🧰️ The decision a bound hotkey makes for one action (P4). */
/** ⌨️ True when a keydown's target is a text-editing surface (input/textarea/select/contenteditable) — hotkeys never fire while the user is typing. */
export function isEditableEventTarget(target: EventTarget | null): boolean {
  if (!(target instanceof HTMLElement)) return false;
  const tag = target.tagName;
  if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return true;
  if (target.isContentEditable) return true;
  return target.closest("[contenteditable='true'], [role='textbox']") != null;
}

/** ⌨️ True when a keydown event matches one `+`-joined chord (e.g. `"mod+shift+z"`), where `mod` accepts either ctrl or meta. */
export function keyboardEventMatchesChord(event: KeyboardEvent, chord: string): boolean {
  const parts = chord.split("+").map((part) => part.trim());
  const key = parts[parts.length - 1] ?? "";
  const needsCtrl = parts.includes("ctrl") || parts.includes("meta") || parts.includes("mod");
  const needsShift = parts.includes("shift");
  const needsAlt = parts.includes("alt");
  const hasCtrl = event.ctrlKey || event.metaKey;
  if (needsCtrl !== hasCtrl) return false;
  if (needsShift !== event.shiftKey) return false;
  if (needsAlt !== event.altKey) return false;
  return event.key.toLowerCase() === key;
}

export type KeybindingIntent = { readonly kind: "fire" } | { readonly kind: "open"; readonly actionId: string } | { readonly kind: "execute"; readonly actionId: string; readonly args: Record<string, unknown> };

/**
 * ✍️ Pure P4 decision: an arg-less action fires directly; an arg-carrying action opens its staged form,
 * unless that form is already the expanded one in the active window AND validation passes, in which case
 * the hotkey executes with the merged effective args. An already-open-but-invalid form stays open.
 */
export function resolveKeybindingIntent(definition: Pick<ActionDefinition, "id" | "args"> | undefined, expandedActionId: string | null, stagedArgs: Readonly<Record<string, unknown>>): KeybindingIntent {
  if (!definition || !actionRequiresStagedForm(definition)) return { kind: "fire" };
  if (expandedActionId === definition.id) {
    const effective = effectiveActionArgs(definition.args, stagedArgs);
    if (missingRequiredArgs(definition.args, effective).length === 0) return { kind: "execute", actionId: definition.id, args: effective };
  }
  return { kind: "open", actionId: definition.id };
}

/** 🧰️ Pure P5 activation decision: an empty request, or re-requesting the already-active utility, deactivates (null); otherwise the requested utility becomes active. */
export function resolveUtilityActivation(current: string | null | undefined, requested: string): string | null {
  return requested === "" || (current ?? null) === requested ? null : requested;
}

/** 🗂️ Category id for one action: declared category, else `"history"` for history actions, else `"actions"` (mirrors the command-palette fallback at {@link resolveCommands}'s sibling `searchItems` builder). */
export function actionCategoryId(action: Pick<ActionDefinition, "category" | "kind">): string {
  return action.category ?? (action.kind === "history" ? "history" : "actions");
}

/** 🗂️ Resolves an action category's display label: the app's own group-label overlay first, then the shared `ui.ribbon.parent.*` chrome vocabulary for known category ids, else the raw id (mirrors {@link resolveUtilityGroupLabel}). */
function actionCategoryLabel(category: string, appLabelsOverlay: PluginAppLabelsOverlay): string {
  const fallback = CHROME_KNOWN_RIBBON_PARENT_CATEGORIES.has(category) ? shellLabel(`ui.ribbon.parent.${category as UiRibbonParentCategory}`) : category;
  return resolveAppLabel(appLabelsOverlay, "group", category, fallback);
}

/** 🗂️ Ordered, deduped categories from resolved actions (sibling of {@link commandCategories}). */
export function actionCategories(actions: readonly ActionDefinition[], appLabelsOverlay: PluginAppLabelsOverlay = EMPTY_APP_LABELS_OVERLAY): { readonly id: string; readonly label: string }[] {
  const seen = new Set<string>();
  const categories: { readonly id: string; readonly label: string }[] = [];
  for (const action of actions) {
    const id = actionCategoryId(action);
    if (seen.has(id)) continue;
    seen.add(id);
    categories.push({ id, label: actionCategoryLabel(id, appLabelsOverlay) });
  }
  return categories;
}

/**
 * 🎛️ Category sections of one window's Actions rail (Tree twin of {@link buildCommandCategoryTree}):
 * one section per category, zero-arg actions fire directly, arg-carrying actions toggle a sibling form
 * section — exactly {@link buildCommandCategoryTree}'s list/form split, localized per category so
 * multiple categories can render side by side. Only one action (across all categories) is expanded at a
 * time, per `expandedActionId`.
 */
export function buildActionCategoryTree(
  windowId: string,
  controllerId: string,
  actions: readonly ActionDefinition[],
  expandedActionId: string | null,
  stagedArgsByKey: Readonly<Record<string, Readonly<Record<string, unknown>>>>,
  disabled: boolean,
  onExpandedChange: (actionId: string | null) => void,
  onStageArg: (actionId: string, argId: string, value: unknown) => void,
  onResetArgs: (actionId: string) => void,
  onExecute: (descriptor: ActionDescriptor) => void,
  appLabelsOverlay: PluginAppLabelsOverlay = EMPTY_APP_LABELS_OVERLAY,
): TreeDataSection[] {
  const categories = actionCategories(actions, appLabelsOverlay);
  const expandedAction = expandedActionId ? actions.find((action) => action.id === expandedActionId) : undefined;
  const sections: TreeDataSection[] = [];
  for (const category of categories) {
    const categoryActions = actions.filter((action) => actionCategoryId(action) === category.id);
    sections.push({
      id: `action.category.${category.id}`,
      label: category.label,
      defaultOpen: true,
      items: categoryActions.map((action): TreeDataItem => {
        const icon = action.iconId ? <Icon icon={action.iconId as IconName} size="small" /> : undefined;
        const rowClassName = disabled ? "pointer-events-none opacity-50" : undefined;
        if (!actionRequiresStagedForm(action)) {
          return { id: `action.${action.id}`, label: action.label, icon, className: rowClassName, onClick: () => !disabled && onExecute({ controllerId, action: action.id }) };
        }
        const expanded = expandedActionId === action.id;
        return {
          id: `action.${action.id}`,
          label: `${action.label}…`,
          icon: icon ?? <Icon icon={expanded ? "chevron-down" : "chevron-right"} size="small" />,
          className: rowClassName,
          onClick: () => !disabled && onExpandedChange(expanded ? null : action.id),
        };
      }),
    });
    if (expandedAction && actionCategoryId(expandedAction) === category.id) {
      const staged = stagedArgsByKey[actionStageKey(windowId, expandedAction.id)] ?? {};
      const effective = effectiveActionArgs(expandedAction.args, staged);
      const missing = missingRequiredArgs(expandedAction.args, effective);
      sections.push({
        id: `action.category.${category.id}.form`,
        defaultOpen: true,
        items: expandedAction.args.map(
          (def): TreeDataItem => ({
            id: `action.${expandedAction.id}.arg.${def.id}`,
            label: def.label,
            description: def.description,
            control: renderStagedArgControl(def, effective[def.id], (value) => onStageArg(expandedAction.id, def.id, value), disabled),
          }),
        ),
        actions: [
          {
            id: childElementId("framework.window", windowId, "action", expandedAction.id, "execute"),
            icon: <Icon icon="check" size="small" />,
            text: shellLabel("ui.common.execute"),
            disabled: disabled || missing.length > 0,
            onClick: () => onExecute({ controllerId, action: expandedAction.id, args: effective }),
          },
          {
            id: childElementId("framework.window", windowId, "action", expandedAction.id, "reset"),
            icon: <Icon icon="undo" size="small" />,
            text: shellLabel("ui.common.reset"),
            disabled,
            onClick: () => onResetArgs(expandedAction.id),
          },
        ],
      });
    }
  }
  return sections;
}

/** 🎛️ Props for the per-window Action rail body (P1/P2). */
export type WindowActionPaneProps = {
  readonly windowId: string;
  readonly controllerId: string;
  readonly actions: readonly ActionDefinition[];
  readonly expandedActionId: string | null;
  readonly stagedArgsByKey: Readonly<Record<string, Readonly<Record<string, unknown>>>>;
  readonly disabled: boolean;
  readonly onExpandedChange: (actionId: string | null) => void;
  readonly onStageArg: (actionId: string, argId: string, value: unknown) => void;
  readonly onResetArgs: (actionId: string) => void;
  readonly onExecute: (descriptor: ActionDescriptor) => void;
  readonly appLabelsOverlay?: PluginAppLabelsOverlay;
};

/**
 * 🎛️ The per-window Actions rail body (P1/P2), grouped into categories like the command panel. Zero-arg
 * actions fire directly; arg-carrying actions expand a locally-buffered staged form (same inline
 * property-row controls as utility measures) — nothing dispatches on edit, effective value is
 * `staged ?? default ?? unset`, Execute is enabled only when every required arg has an effective value,
 * fires exactly ONE `ActionDescriptor` with the merged args, and keeps the staged values afterward.
 * When `disabled` (an active utility with `allowsActionsWhileActive === false`), every row renders disabled.
 */
export function WindowActionPane(props: WindowActionPaneProps): ReactElement {
  const { windowId, controllerId, actions, expandedActionId, stagedArgsByKey, disabled, onExpandedChange, onStageArg, onResetArgs, onExecute, appLabelsOverlay } = props;
  const sections = buildActionCategoryTree(windowId, controllerId, actions, expandedActionId, stagedArgsByKey, disabled, onExpandedChange, onStageArg, onResetArgs, onExecute, appLabelsOverlay);
  return (
    <div data-slot="window-action-pane" className="flex min-w-0 flex-col">
      <Tree sections={sections} showLines={false} sortableSections={false} />
    </div>
  );
}

/** 🧰️ Slice of the {@link ActionPaneState} the {@link windowActionPaneNode} builder reads. */
type ActionPaneSlice = Pick<ActionPaneState, "expandedByWindowId" | "stagedArgsByKey" | "activeUtilityByWindowId">;

/**
 * 🧰️ Sibling of {@link utilityBarNode}: resolves a window kind's panel-eligible actions and returns a
 * bound {@link WindowActionPane}, or `undefined` when the window has no resolved actions (so the rail
 * chip never renders). Rows render disabled while an active utility gates actions
 * (`allowsActionsWhileActive === false`).
 */
export function windowActionPaneNode(
  app: AppDefinition,
  windowKind: AppWindowKindDefinition,
  windowId: string,
  actionPane: ActionPaneSlice,
  onAction: (action: ActionDescriptor) => void,
  dispatch: (action: ShellAction) => void,
  appLabelsOverlay: PluginAppLabelsOverlay = EMPTY_APP_LABELS_OVERLAY,
  terminology: string = UI_TERMINOLOGY_NATIVE,
  locale: string = SHELL_LOCALES[0],
): ReactNode {
  const resolvedActions = resolveWindowActions(app, windowKind);
  if (resolvedActions.length === 0) return undefined;
  const actions = resolvedActions.map((action) => ({
    ...action,
    label: resolveAppLabel(appLabelsOverlay, "action", action.id, resolveManifestLabel(action.label, terminology, locale)),
    args: action.args.map((def) => resolveActionArgDef(def, action.id, appLabelsOverlay, terminology, locale)),
  }));
  const activeUtilityId = actionPane.activeUtilityByWindowId[windowId] ?? null;
  const activeUtility = activeUtilityId ? (app.utilities ?? []).find((utility) => utility.id === activeUtilityId) : undefined;
  const disabled = Boolean(activeUtility && activeUtility.allowsActionsWhileActive === false);
  return (
    <WindowActionPane
      windowId={windowId}
      controllerId={app.controllerId}
      actions={actions}
      expandedActionId={actionPane.expandedByWindowId[windowId] ?? null}
      stagedArgsByKey={actionPane.stagedArgsByKey}
      disabled={disabled}
      onExpandedChange={(actionId) => dispatch({ type: "SET_ACTION_PANE_EXPANDED", windowId, value: actionId })}
      onStageArg={(actionId, argId, value) => dispatch({ type: "STAGE_ACTION_ARG", windowId, actionId, argId, value })}
      onResetArgs={(actionId) => dispatch({ type: "RESET_ACTION_ARGS", windowId, actionId })}
      onExecute={onAction}
      appLabelsOverlay={appLabelsOverlay}
    />
  );
}
//#endregion 🧰️WindowActionPane

//#region 🎛️CommandRegistry
/** 🎛️ One command definition paired with its shared, fully qualified address. */
export type ResolvedCommand = {
  readonly definition: CommandDefinition;
  readonly address: CommandAddress;
};

/** 📍️ Stable key for maps whose command ids may overlap across owners. */
export function commandAddressKey(address: CommandAddress): string {
  const owner = address.owner;
  if (owner === "os") return `os:${address.commandId}`;
  if ("plugin" in owner) return `plugin:${owner.plugin.pluginId}:${address.commandId}`;
  if ("app" in owner) return `app:${owner.app.pluginId}:${owner.app.appId}:${address.commandId}`;
  return `mode:${owner.mode.pluginId}:${owner.mode.appId}:${owner.mode.modeId}:${address.commandId}`;
}

/** 🪪️ Element-id-safe projection of the complete command address. */
function commandElementKey(address: CommandAddress): string {
  return commandAddressKey(address).replaceAll(":", ".");
}

/** 📍️ Whether an addressed command belongs to the operating-system catalog. */
export function isOsCommandAddress(address: CommandAddress): boolean {
  return address.owner === "os";
}

/** 📍️ Plugin segment of a non-OS command owner. */
export function commandOwnerPluginId(owner: CommandAddress["owner"]): string | null {
  if (owner === "os") return null;
  if ("plugin" in owner) return owner.plugin.pluginId;
  if ("app" in owner) return owner.app.pluginId;
  return owner.mode.pluginId;
}

/** ⌨️ Detects the command-keybinding platform from a browser/host platform description. */
export function detectCommandPlatform(value: string): Platform | undefined {
  if (/mac|iphone|ipad/i.test(value)) return "macOs";
  if (/win/i.test(value)) return "windows";
  if (/linux|x11/i.test(value)) return "linux";
  return undefined;
}

/** ⌨️ Resolves a command's portable chords for one host platform. */
export function commandKeybindingChords(definition: Pick<CommandDefinition, "keybindings">, platform?: Platform): string[] {
  return definition.keybindings.filter((binding) => binding.platform === undefined || binding.platform === platform).map((binding) => binding.chord);
}

/**
 * 🎛️ Aggregates every command visible for the current session: os built-ins, the active session's
 * plugin-owned commands, app-owned commands, and commands owned by the active mode. Window-local
 * verbs are actions and therefore never enter this global command resolver — unlike
 * `resolveWindowActions`/`resolveUtilities`, this never takes a window kind.
 */
export function resolveCommands(
  osCommands: readonly CommandDefinition[],
  activePluginManifest: Pick<PluginManifest, "pluginId" | "commands"> | null | undefined,
  app: Pick<AppDefinition, "id" | "commands" | "modes"> | null | undefined,
  activeModeId: string,
  overlay: PluginAppLabelsOverlay = EMPTY_APP_LABELS_OVERLAY,
  terminology: string = UI_TERMINOLOGY_NATIVE,
  locale: string = SHELL_LOCALES[0],
): ResolvedCommand[] {
  // 🗺️ `CommandDefinition.label`/`.args[].label` are manifest `LocalizedLabel` fields — there is no
  // "command"/"commandArg" overlay category (commands never went through `AppLabelsOverlay`), so this is
  // the single choke point that resolves them to plain strings for every downstream consumer (the
  // footer command panel, the command palette, `noteShellCommand`'s history label); `osCommands` are
  // already plain strings (built by `buildOsCommands` via `shellLabel`) and pass through unchanged.
  const resolveDefinition = (definition: CommandDefinition): CommandDefinition => ({
    ...definition,
    label: resolveManifestLabel(definition.label, terminology, locale),
    args: definition.args.map((def) => resolveActionArgDef(def, definition.id, overlay, terminology, locale)),
  });
  const resolved: ResolvedCommand[] = osCommands.map((definition) => ({ definition: resolveDefinition(definition), address: { owner: "os", commandId: definition.id } }));
  for (const definition of activePluginManifest?.commands ?? []) {
    resolved.push({ definition: resolveDefinition(definition), address: { owner: { plugin: { pluginId: activePluginManifest!.pluginId } }, commandId: definition.id } });
  }
  if (!app) return resolved;
  const pluginId = activePluginManifest?.pluginId ?? "";
  for (const definition of app.commands ?? []) {
    resolved.push({ definition: resolveDefinition(definition), address: { owner: { app: { pluginId, appId: app.id } }, commandId: definition.id } });
  }
  const activeMode = (app.modes as readonly AppModeDefinition[] | undefined)?.find((mode) => mode.id === activeModeId);
  for (const definition of activeMode?.commands ?? []) {
    resolved.push({ definition: resolveDefinition(definition), address: { owner: { mode: { pluginId, appId: app.id, modeId: activeModeId } }, commandId: definition.id } });
  }
  return resolved;
}

/** 🎛️ Chrome-known command category ids that already have a `ui.settings.tab.*` translation key. */
const CHROME_KNOWN_COMMAND_CATEGORIES = new Set(["general", "driver", "app", "appearance", "layout", "language", "terminology", "theme"]);

/** 🎛️ Loose title-case for an open-set command category id (e.g. "appearance" -> "Appearance"). Falls back to this for app/plugin-invented categories that have no fixed framework vocabulary entry. */
function titleizeCommandCategory(category: string): string {
  return category.replace(/[-_]+/g, " ").replace(/\b\w/g, (char) => char.toUpperCase());
}

/** 🎛️ Resolves a command category's display label, reusing the existing `ui.settings.tab.*` keys for chrome-known ids and falling back to a loose title-case for open-set app/plugin categories. */
export function commandCategoryLabel(category: string): string {
  return CHROME_KNOWN_COMMAND_CATEGORIES.has(category) ? shellLabel(`ui.settings.tab.${category as "general" | "driver" | "app" | "appearance" | "layout" | "language" | "terminology" | "theme"}`) : titleizeCommandCategory(category);
}

/** 🎛️ Ordered, deduped category tabs for the footer command panel, derived from whatever commands actually resolved. */
export function commandCategories(commands: readonly ResolvedCommand[]): { readonly id: string; readonly label: string }[] {
  const seen = new Set<string>();
  const categories: { readonly id: string; readonly label: string }[] = [];
  for (const { definition } of commands) {
    if (!definition.inPalette) continue;
    if (seen.has(definition.category)) continue;
    seen.add(definition.category);
    categories.push({ id: definition.category, label: commandCategoryLabel(definition.category) });
  }
  return categories;
}

function selectCommandArg(id: string, label: string, options: readonly { readonly value: string; readonly label: string }[]): ActionArgDef {
  return { id, label, control: { kind: "select", options: options.map((option) => ({ ...option })) }, required: true };
}

/** @emoji 🚗️ Translated display name for a built-in driver id; a custom (user-authored) driver has no
 * translation key, so its own {@link UiDriver.label} (genuine runtime data) is the correct fallback. */
export function driverDisplayLabel(driver: UiDriver): string {
  if (driver.id === "default") return shellLabel("settings.driver.default");
  if (driver.id === "compact") return shellLabel("settings.driver.compact");
  return driver.label || driver.id;
}

/**
 * 🎛️ Os-owned built-in commands — app introduction/theme/layout/locale/appearance/driver,
 * handled locally by the shell (never routed to a program). Rebuilt via `useMemo` since the theme and
 * terminology option lists are live state.
 */
export function buildOsCommands(
  themeList: readonly UiTheme[],
  terminologies: readonly string[],
  hasIntroduction: boolean,
  locks: ResolvedShellLocks = EMPTY_SHELL_LOCKS,
  driverList: readonly UiDriver[] = builtinUiDrivers(),
  tutorials: readonly { readonly id: string; readonly title: LocalizedLabel | string }[] = [],
  tutorialRecorderAvailable = false,
  terminology: string = UI_TERMINOLOGY_NATIVE,
  locale: string = SHELL_LOCALES[0],
  /** 👁️✏️ Whether the active session's dialect has at least one `AppRouter` entry for either role —
   * gates `open-artifact-with-viewer`/`open-artifact-with-editor` (contract freeze §5) the same way
   * `hasIntroduction`/`tutorialRecorderAvailable` gate their own optional commands above. */
  hasOpenArtifactSurfaces = false,
): CommandDefinition[] {
  const lockedCommandIds = new Set<string>([...(locks.appearance ? ["os.setAppearance"] : []), ...(locks.themeId ? ["os.setThemeId"] : []), ...(locks.locale ? ["os.setLocale"] : []), ...(locks.terminology ? ["os.setTerminology"] : [])]);
  const commands: CommandDefinition[] = [
    {
      id: "os.toggleFullscreen",
      label: shellLabel("ui.fullscreen.toggle"),
      category: "layout",
      iconId: "maximize-2",
      kind: "shell",
      inPalette: true,
      args: [],
      keybindings: [
        { chord: "f11", platform: "windows" },
        { chord: "f11", platform: "linux" },
        { chord: "control+meta+f", platform: "macOs" },
      ],
    },
    ...(hasIntroduction ? [{ id: "os.introduceApp", label: shellLabel("ui.command.introduceApp"), category: "app", iconId: "graduation-cap", kind: "shell" as const, inPalette: true, args: [], keybindings: [] }] : []),
    // 🎥️ `os.playTutorial` only appears once at least one tutorial is declared (app-own or brand-own);
    // `os.recordTutorial` is dev/studio-only (see `isTutorialRecorderAvailable`) and needs no declared
    // tutorial at all — recording an app IS the authoring path for one.
    ...(tutorials.length > 0
      ? [{ id: "os.playTutorial", label: shellLabel("ui.command.playTutorial"), category: "app", iconId: "play", kind: "shell" as const, inPalette: true, keybindings: [], args: [selectCommandArg("tutorialId", shellLabel("tutorial.chapter"), tutorials.map((tutorial) => ({ value: tutorial.id, label: resolveManifestLabel(tutorial.title, terminology, locale) })))] }]
      : []),
    ...(tutorialRecorderAvailable ? [{ id: "os.recordTutorial", label: shellLabel("ui.command.recordTutorial"), category: "app", iconId: "circle", kind: "shell" as const, inPalette: true, args: [], keybindings: [] }] : []),
    {
      id: "os.setAppearance",
      label: shellLabel("ui.command.setAppearance"),
      category: "appearance",
      iconId: "sun-moon",
      kind: "shell",
      keybindings: [],
      inPalette: true,
      args: [
        selectCommandArg("appearance", shellLabel("ui.settings.tab.appearance"), [
          { value: "system", label: shellLabel("ui.settings.appearance.system") },
          { value: "light", label: shellLabel("ui.settings.appearance.light") },
          { value: "dark", label: shellLabel("ui.settings.appearance.dark") },
        ]),
      ],
    },
    {
      id: "os.setThemeId",
      label: shellLabel("ui.command.setTheme"),
      category: "appearance",
      iconId: "palette",
      kind: "shell",
      keybindings: [],
      inPalette: true,
      args: [
        selectCommandArg(
          "themeId",
          shellLabel("ui.settings.tab.theme"),
          themeList.map((theme) => ({ value: theme.id, label: theme.label || theme.id })),
        ),
      ],
    },
    {
      id: "os.setLayout",
      label: shellLabel("ui.command.setLayout"),
      category: "layout",
      iconId: "layout-template",
      kind: "shell",
      keybindings: [],
      inPalette: true,
      args: [
        selectCommandArg("layout", shellLabel("ui.settings.tab.layout"), [
          { value: "desktop", label: shellLabel("settings.layout.desktop") },
          { value: "tablet", label: shellLabel("settings.layout.tablet") },
        ]),
      ],
    },
    { id: "os.resetDock", label: shellLabel("ui.settings.resetDock"), category: "layout", iconId: "undo", kind: "shell", inPalette: true, args: [], keybindings: [] },
    {
      id: "os.setLocale",
      label: shellLabel("ui.command.setLocale"),
      category: "language",
      iconId: "languages",
      kind: "shell",
      keybindings: [],
      inPalette: true,
      args: [
        selectCommandArg("locale", shellLabel("ui.settings.tab.language"), [
          { value: "en", label: shellLabel("ui.settings.language.en") },
          { value: "de", label: shellLabel("ui.settings.language.de") },
        ]),
      ],
    },
    {
      id: "os.setTerminology",
      label: shellLabel("ui.command.setTerminology"),
      category: "language",
      iconId: "languages",
      kind: "shell",
      keybindings: [],
      inPalette: true,
      args: [
        selectCommandArg(
          "terminology",
          shellLabel("ui.settings.tab.terminology"),
          terminologies.map((id) => ({ value: id, label: shellTerminologyLabel(id) })),
        ),
      ],
    },
    {
      id: "os.setDriver",
      label: shellLabel("ui.command.setDriver"),
      category: "layout",
      iconId: "settings",
      kind: "shell",
      keybindings: [],
      inPalette: true,
      args: [
        selectCommandArg(
          "driver",
          shellLabel("ui.settings.tab.driver"),
          driverList.map((driver) => ({ value: driver.id, label: driverDisplayLabel(driver) })),
        ),
      ],
    },
    // 👁️✏️ Both share the frozen "Open with…" label (contract freeze §5) — the role is which picker
    // group they focus, not a different label; `dispatchOsCommand` opens the Document panel's
    // "Open with…" section pre-scoped to that role rather than sending a wire command itself.
    ...(hasOpenArtifactSurfaces
      ? [
          { id: OPEN_ARTIFACT_WITH_VIEWER_COMMAND_ID, label: openArtifactWithText(locale), category: "artifact", iconId: "eye" as IconName, kind: "shell" as const, inPalette: true, args: [], keybindings: [] },
          { id: OPEN_ARTIFACT_WITH_EDITOR_COMMAND_ID, label: openArtifactWithText(locale), category: "artifact", iconId: "pencil" as IconName, kind: "shell" as const, inPalette: true, args: [], keybindings: [] },
        ]
      : []),
  ];
  return commands.filter((command) => !lockedCommandIds.has(command.id));
}

/** 🎛️ Os-scope command ids that are handled locally by the shell — mirrors {@link buildOsCommands}. */
export function dispatchOsCommand(
  commandId: string,
  args: Record<string, unknown> | undefined,
  dispatch: (action: ShellAction) => void,
  dockLayoutStore: DockLayoutStore,
  dockUiStateStore: DockUiStateStore,
  locks: ResolvedShellLocks = EMPTY_SHELL_LOCKS,
): void {
  switch (commandId) {
    case "os.introduceApp":
      dispatch({ type: "SET_INTRODUCTION_STEP", value: 0 });
      return;
    case "os.setAppearance":
      if (locks.appearance) return;
      dispatch({ type: "SET_UI_APPEARANCE", value: (args?.appearance as ElementsSurfaceAppearance) ?? "system" });
      return;
    case "os.setThemeId":
      if (locks.themeId) return;
      if (typeof args?.themeId === "string") dispatch({ type: "SET_UI_THEME_ID", value: args.themeId });
      return;
    case "os.setLayout":
      dispatch({ type: "SET_UI_LAYOUT", value: (args?.layout as UiChromeLayout) ?? "desktop" });
      return;
    case "os.resetDock":
      dispatch({ type: "RESET_DOCK" });
      dockLayoutStore.reset();
      dockUiStateStore.reset();
      return;
    case "os.setLocale":
      if (locks.locale) return;
      if (typeof args?.locale === "string") {
        setUiLocale(args.locale as UiLocale);
        dispatch({ type: "SET_UI_LOCALE", value: args.locale as UiLocale });
      }
      return;
    case "os.setTerminology":
      if (locks.terminology) return;
      if (typeof args?.terminology === "string") dispatch({ type: "SET_UI_TERMINOLOGY", value: args.terminology });
      return;
    case "os.setDriver":
      if (typeof args?.driver === "string") dispatch({ type: "SET_UI_DRIVER_ID", value: args.driver });
      return;
    // 👁️✏️ Neither sends a wire command itself (contract freeze §5) — both just focus the Document
    // panel's "Open with…" section, pre-expanded to the role the palette entry named.
    case OPEN_ARTIFACT_WITH_VIEWER_COMMAND_ID:
    case OPEN_ARTIFACT_WITH_EDITOR_COMMAND_ID:
      dispatch({ type: "SET_OPEN_WITH_FOCUS_ROLE", value: commandId === OPEN_ARTIFACT_WITH_VIEWER_COMMAND_ID ? "viewer" : "editor" });
      dispatch({ type: "SET_PANEL_PATH", anchor: "top-left", value: [FRAMEWORK_PANEL_TAB_ARTIFACT_ID] });
      dispatch({ type: "SET_PANEL_VISIBLE", anchor: "top-left", value: true });
      return;
    default:
      return;
  }
}

/** @emoji 🎛️ Fallback icon for every command-category leaf — categories are open-set strings any plugin/app/mode author can invent, so there's no per-category icon metadata to key off (unlike the framework's own Workbench/Details/Display/Settings categories). */
const COMMAND_CATEGORY_ICON = shellTabIcon("wrench");

/**
 * 🎛️ One category's command list (and, if a command is expanded, its staged arg form) as a `TreePanelConfig`
 * — the content a category `PanelTabLeaf` resolves to. A zero-arg command's row fires immediately on click
 * (a plain fire-and-forget tree row, same pattern as {@link groupNamedLayoutsToTreeItems}'s layout rows —
 * no `selectedIds`/`onSelectionChange` override, so it takes `Tree`'s default single-select highlight after
 * firing, same as clicking a Display→Layout row does). An arg-carrying command's row toggles `expandedCommandId`
 * itself (kept as its own exclusive, bespoke state — not `Tree`'s per-row `openStates`, which isn't naturally
 * exclusive across sibling rows) and, when expanded, a synthetic form section (one row per arg, `control`
 * holding the staged input, replacing the old `Field` wrapper since `TreeDataItem` already renders label +
 * description + control in the same two-column layout) is prepended so it renders above the command list —
 * `Tree` reverses top-level `sections` for `direction="up"` (bottom anchors), threaded here via `flowFromAnchor`/
 * `FlowProvider`/`useFlow` down from the hosting `Panel`, not any manual reversal in this function.
 */
export function buildCommandCategoryTree(
  commands: readonly ResolvedCommand[],
  expandedCommandId: string | null,
  stagedArgsByCommandId: Readonly<Record<string, Readonly<Record<string, unknown>>>>,
  onExecute: (entry: ResolvedCommand, executeArgs?: Record<string, unknown>) => void,
  onToggleExpanded: (commandId: string | null) => void,
  onStageArg: (commandId: string, argId: string, value: unknown) => void,
  onResetArgs: (commandId: string) => void,
): TreePanelConfig {
  const argCarryingCommands = commands.filter((entry) => entry.definition.args.length > 0);
  const autoExpandedSingleton = argCarryingCommands.length === 1 ? argCarryingCommands[0] : undefined;
  const expanded = (expandedCommandId ? commands.find((entry) => commandAddressKey(entry.address) === expandedCommandId) : undefined) ?? autoExpandedSingleton;
  const effectiveExpandedId = expanded ? commandAddressKey(expanded.address) : null;
  const sections: TreeDataSection[] = [];
  if (expanded && expanded.definition.args.length > 0) {
    const expandedKey = commandAddressKey(expanded.address);
    const expandedElementKey = commandElementKey(expanded.address);
    const staged = stagedArgsByCommandId[expandedKey] ?? {};
    const effective = effectiveActionArgs(expanded.definition.args, staged);
    const missing = missingRequiredArgs(expanded.definition.args, effective);
    sections.push({
      id: `command.category.${expanded.definition.category}.form`,
      items: expanded.definition.args.map(
        (def): TreeDataItem => ({
          id: `command.${expandedElementKey}.arg.${def.id}`,
          label: def.label,
          description: def.description,
          control: renderStagedArgControl(def, effective[def.id], (value) => onStageArg(expandedKey, def.id, value)),
        }),
      ),
      actions: [
        {
          id: `command-${expandedElementKey}-execute`,
          icon: <Icon icon="check" size="small" />,
          text: shellLabel("ui.common.execute"),
          disabled: missing.length > 0,
          onClick: () => onExecute(expanded, effective),
        },
        {
          id: `command-${expandedElementKey}-reset`,
          icon: <Icon icon="undo" size="small" />,
          text: shellLabel("ui.common.reset"),
          onClick: () => onResetArgs(expandedKey),
        },
      ],
    });
  }
  const listCommands = commands.filter((entry) => commandAddressKey(entry.address) !== effectiveExpandedId);
  if (listCommands.length > 0) {
    sections.push({
      id: "command.category.list",
      items: listCommands.map((entry): TreeDataItem => {
        const argCarrying = entry.definition.args.length > 0;
        const entryKey = commandAddressKey(entry.address);
        const elementKey = commandElementKey(entry.address);
        const icon = entry.definition.iconId ? <Icon icon={entry.definition.iconId as IconName} size="small" /> : undefined;
        if (!argCarrying) return { id: `command.${elementKey}`, label: entry.definition.label, icon, onClick: () => onExecute(entry) };
        return {
          id: `command.${elementKey}`,
          label: `${entry.definition.label}…`,
          icon: <Icon icon={expandedCommandId === entryKey ? "chevron-down" : "chevron-up"} size="small" />,
          onClick: () => onToggleExpanded(expandedCommandId === entryKey ? null : entryKey),
        };
      }),
    });
  }
  return { sections };
}

/**
 * 🎛️ One `PanelTabLeaf` per resolved command category — consumers wrap these under the Command branch
 * (`FRAMEWORK_CATEGORY_COMMAND_ID`) on `defaultDock.anchors["bottom-middle"]` so the folded chrome shows
 * a single expandable Command toggle. The command palette's fold/active-category/size/persistence is the
 * generic per-anchor `Panel` state (see `buildPanelProps`); this only builds the category tab leaves.
 * Content is a *lazy* `resolveTree` (mirrors {@link createFrameworkDisplayPanelTabs}'s windows tab) so
 * this array — and therefore `defaultDock`'s own memo — never depends on `expandedCommandId`/
 * `stagedArgsByCommandId`, which change on every keystroke while staging a command argument; `resolveTree`
 * reads those fresh off refs at render time instead.
 */
export function buildCommandCategoryTabs(
  resolvedCommands: readonly ResolvedCommand[],
  categories: readonly { readonly id: string; readonly label: string }[],
  expandedCommandIdRef: React.RefObject<string | null>,
  stagedArgsByCommandIdRef: React.RefObject<Readonly<Record<string, Readonly<Record<string, unknown>>>>>,
  onCommand: (address: CommandAddress, args?: Record<string, unknown>) => void,
  dispatch: (action: ShellAction) => void,
): PanelTabNode[] {
  return categories.map((category) => {
    const categoryCommands = resolvedCommands.filter((entry) => entry.definition.inPalette && entry.definition.category === category.id);
    return singleTreeLeaf({
      id: `command.category.${category.id}`,
      icon: COMMAND_CATEGORY_ICON,
      name: category.label,
      tree: {
        resolveTree: () =>
          buildCommandCategoryTree(
            categoryCommands,
            expandedCommandIdRef.current,
            stagedArgsByCommandIdRef.current,
            (entry, executeArgs) => onCommand(entry.address, executeArgs),
            (commandId) => dispatch({ type: "SET_COMMAND_EXPANDED", value: commandId }),
            (commandId, argId, value) => dispatch({ type: "STAGE_COMMAND_ARG", commandId, argId, value }),
            (commandId) => dispatch({ type: "RESET_COMMAND_ARGS", commandId }),
          ),
      },
    });
  });
}
//#endregion 🎛️CommandRegistry

//#region 🛠️ToolRegistry
/**
 * 🛠️ One tool's measure-tree content. Selecting the tool tab activates it (see `buildToolTabs` /
 * panel path change); the tree itself is a single headerless section mapped to native `TreeDataItem`s
 * so Fill opens directly onto count + distribution with the same chrome as left-corner panel trees.
 */
function buildToolTree(tool: ToolDefinition, controllerId: string, isActive: boolean, measures: readonly WindowMeasure[] | undefined, onAction: (action: ActionDescriptor) => unknown): { readonly sections: TreeDataSection[]; readonly sortableSections: false } {
  const iconName: IconName = tool.iconId as IconName;
  if (isActive && measures && measures.length > 0) {
    return {
      sortableSections: false,
      sections: [
        {
          id: `tool.${tool.id}.options`,
          label: "",
          defaultOpen: true,
          items: windowMeasuresToTreeItems(measures, onAction),
        },
      ],
    };
  }
  return {
    sortableSections: false,
    sections: [
      {
        id: `tool.${tool.id}.activate`,
        label: "",
        defaultOpen: true,
        items: [
          {
            id: `tool.${tool.id}.activate.toggle`,
            label: "",
            control: (
              <Toggle
                id={`tool.${tool.id}`}
                pressed={isActive}
                text={tool.label}
                icon={<Icon icon={iconName} size="small" />}
                onPressedChange={(pressed) => onAction({ controllerId, action: SET_ACTIVE_TOOL_ACTION_ID, args: { toolId: pressed ? tool.id : "" } })}
              />
            ),
          },
        ],
      },
    ],
  };
}

/**
 * 🛠️ One `PanelTabLeaf` per resolved mode tool — consumers wrap these under the Tool branch
 * (`FRAMEWORK_CATEGORY_TOOL_ID`) on `defaultDock.anchors["bottom-middle"]`, ordered left of the Command
 * branch, so the folded chrome shows a single Tool toggle. Content is a *lazy* `resolveTree` (mirrors
 * `buildCommandCategoryTabs`'s windows tab) so this array — and therefore `defaultDock`'s own memo —
 * never depends on `activeToolId`/`toolMeasuresByToolId`, which change on every activation/slider tick;
 * `resolveTree` reads those fresh off refs at render time instead.
 */
export function buildToolTabs(
  tools: readonly ToolDefinition[],
  controllerId: string,
  activeToolIdRef: React.RefObject<string | null>,
  toolMeasuresByToolIdRef: React.RefObject<Readonly<Record<string, readonly WindowMeasure[]>>>,
  onAction: (action: ActionDescriptor) => unknown,
): PanelTabNode[] {
  return tools.map((tool) =>
    singleTreeLeaf({
      id: `tool.${tool.id}`,
      icon: shellTabIcon(tool.iconId),
      name: tool.label,
      tree: {
        resolveTree: () => {
          const tree = buildToolTree(tool, controllerId, activeToolIdRef.current === tool.id, toolMeasuresByToolIdRef.current[tool.id], onAction);
          return { sections: tree.sections, sortableSections: tree.sortableSections };
        },
      },
    }),
  );
}

/** 🛠️ Activates the mode tool whose footer tab was just selected (`tool.<id>`), mirroring utility-bar press → options. */
export function toolIdFromPanelTabId(tabId: string | undefined): string | null {
  if (!tabId?.startsWith("tool.")) return null;
  const toolId = tabId.slice("tool.".length);
  return toolId.length > 0 ? toolId : null;
}
//#endregion 🛠️ToolRegistry

/** @emoji 🐢️ Structural equality over plain JSON-shaped values (the shape every `UiNode`/`WindowEngagement`/`WindowMeasure` program payload takes) — no cycles, no non-JSON types. */
function uiJsonDeepEqual(a: unknown, b: unknown): boolean {
  if (a === b) return true;
  if (typeof a !== "object" || typeof b !== "object" || a === null || b === null) return false;
  if (Array.isArray(a) !== Array.isArray(b)) return false;
  if (Array.isArray(a) && Array.isArray(b)) {
    if (a.length !== b.length) return false;
    for (let index = 0; index < a.length; index += 1) {
      if (!uiJsonDeepEqual(a[index], b[index])) return false;
    }
    return true;
  }
  const aRecord = a as Record<string, unknown>;
  const bRecord = b as Record<string, unknown>;
  const aKeys = Object.keys(aRecord);
  const bKeys = Object.keys(bRecord);
  if (aKeys.length !== bKeys.length) return false;
  for (const key of aKeys) {
    if (!Object.prototype.hasOwnProperty.call(bRecord, key)) return false;
    if (!uiJsonDeepEqual(aRecord[key], bRecord[key])) return false;
  }
  return true;
}

/**
 * @emoji 🐢️ Reuses `previous`'s object identity when it's structurally equal to `next` — every program
 * `render()`/`utilities()`/`windowEngagements()`/`windowMeasures()` call re-parses a fresh JSON payload
 * every time, even when nothing about that body actually changed (e.g. a camera-only or selection-only
 * action still returns byte-identical panel/utility JSON). Without this, every downstream `React.memo`
 * (see `InterpretedUiNode`) sees a new prop reference every render and can never bail.
 */
export function preserveJsonIdentity<T>(previous: T | undefined, next: T): T {
  return previous !== undefined && uiJsonDeepEqual(previous, next) ? previous : next;
}

/**
 * @emoji 🐢️ Builds a `Record<string, V>` from `entries`, reusing `prev`'s per-key value reference where
 * `preserveJsonIdentity` finds no structural change, and reusing `prev` itself (the whole record) when
 * no key actually changed — so a no-operation action's `dispatch` doesn't hand `windowUiByWindowId`/etc. a new
 * object reference and cascade an unmemoizable re-render through every downstream consumer.
 */
export function mergeRecordPreservingIdentity<V>(prev: Readonly<Record<string, V>>, entries: readonly (readonly [string, V])[]): Readonly<Record<string, V>> {
  const next: Record<string, V> = {};
  let changed = Object.keys(prev).length !== entries.length;
  for (const [key, value] of entries) {
    const preserved = preserveJsonIdentity(prev[key], value);
    next[key] = preserved;
    if (preserved !== prev[key]) changed = true;
  }
  return changed ? next : prev;
}

/** @emoji 🎯️ Merges selection chrome into an existing world-3d component scene without touching instance geometry. */
export function patchWorld3dChromeOntoNode(node: UiNode, patch: { readonly selectionJson: string; readonly vorticesJson?: string }): UiNode {
  if (node.type !== "component" || !node.world3d) return node;
  const next: UiNode = {
    ...node,
    world3d: {
      ...node.world3d,
      selectionJson: patch.selectionJson,
      ...(patch.vorticesJson !== undefined ? { vorticesJson: patch.vorticesJson } : {}),
    },
  };
  return preserveJsonIdentity(node, next);
}

/** @emoji 🌲️ Updates tree-level selection highlights without rebuilding structural sections. */
export function patchDocumentTreeSelectedIds(node: UiNode, selectedIds: readonly string[], highlightedIds?: readonly string[]): UiNode {
  if (node.type !== "tree") return node;
  const next: UiNode = {
    ...node,
    selectedIds: [...selectedIds],
    ...(highlightedIds ? { highlightedIds: [...highlightedIds] } : {}),
  };
  return preserveJsonIdentity(node, next);
}

//#region UiRefresh
/** @emoji 🐢️ One cached section value keyed by `${section}:${key}` (e.g. `window:2d-overview`, `engagements`) — the hash is what gets sent back to the plugin next time so it can skip re-serializing unchanged content. */
export type UiRefreshCache = Map<string, { readonly hash: string; readonly value: unknown }>;

function uiRefreshWantsWindow(scope: UiDirtyScope, bodyKey: string): boolean {
  return scope.kind === "full" || (scope.kind === "partial" && (scope.windowBodies ?? []).includes(bodyKey));
}
function uiRefreshWantsPanel(scope: UiDirtyScope, bodyKey: string): boolean {
  return scope.kind === "full" || (scope.kind === "partial" && (scope.panelBodies ?? []).includes(bodyKey));
}
function uiRefreshWantsFlag(scope: UiDirtyScope, flag: "engagements" | "measures" | "tools" | "labels"): boolean {
  return scope.kind === "full" || (scope.kind === "partial" && scope[flag] === true);
}

/**
 * 🪟️ Every live window instance for a session — one per base `AppDefinition.windowKinds` entry (id ==
 * kind id) plus one per split/spawned extra — so `refreshUi` fetches and the plugin returns state for
 * every actual window, never collapsing two same-kind instances (e.g. split top/perspective panes) onto
 * one shared entry.
 */
export function sessionWindowInstances(
  app: { readonly windowKinds: readonly { readonly id: string; readonly bodyKey: string }[] },
  extraWindowInstances: readonly ExtraWindowInstance[],
): readonly { readonly id: string; readonly bodyKey: string; readonly windowKindId: string }[] {
  const kindById = new Map(app.windowKinds.map((kind) => [kind.id, kind] as const));
  const base = app.windowKinds.map((kind) => ({ id: kind.id, bodyKey: kind.bodyKey, windowKindId: kind.id }));
  const extra = extraWindowInstances.flatMap((instance) => {
    const kind = kindById.get(instance.windowKindId);
    return kind ? [{ id: instance.id, bodyKey: kind.bodyKey, windowKindId: instance.windowKindId }] : [];
  });
  return [...base, ...extra];
}

/** 🎓️ Kind-level introduction targets must also match live window *instances* of that kind
 * (`puzzle3d-main-top` / `puzzle3d-main-perspective` for kind `puzzle3d-main`) — otherwise force-unfold
 * of the utility bar / Actions rail never reaches the panes the user actually sees. `targetKindId` is a
 * raw window-kind id; `targetSegment` is an already-normalized `elementIdSegment` (e.g. from a
 * `framework.window.{segment}.action.*` introduce id). */
export function introductionTargetsWindow(
  windowId: string,
  windowKindId: string,
  targetKindId: string | null,
  targetSegment: string | null = null,
): boolean {
  if (targetKindId && (elementIdSegment(windowId) === elementIdSegment(targetKindId) || elementIdSegment(windowKindId) === elementIdSegment(targetKindId))) return true;
  if (targetSegment && (elementIdSegment(windowId) === targetSegment || elementIdSegment(windowKindId) === targetSegment)) return true;
  return false;
}

/** @emoji 🧰️ Materializes the shell's per-window utility map for batched `refresh-ui` — omits null entries. */
export function buildActiveUtilityByWindowId(activeUtilityByWindowId: Readonly<Record<string, string | null>>): Record<string, string> {
  return Object.fromEntries(Object.entries(activeUtilityByWindowId).flatMap(([windowId, utilityId]) => (utilityId ? [[windowId, utilityId]] : [])));
}

/**
 * @emoji 🐢️ Builds one batched `refresh-ui` request restricted to `scope` — `null` when the scope
 * resolves to nothing worth fetching (`none`, or a `partial` whose fields all miss this app's actual
 * bodies/instances). Every requested entry carries the host's cached hash so the plugin can omit payloads
 * for sections that didn't change. `windowInstances` is keyed by window INSTANCE id (base windows plus any
 * split/spawned extras) — never by window kind — so two instances of the same kind get independent
 * cache entries and independent rendered bodies.
 */
export function buildUiRefreshRequest(
  scope: UiDirtyScope,
  windowInstances: readonly { readonly id: string; readonly bodyKey: string }[],
  panelTabLeaves: readonly { readonly kind: PanelTabKind; readonly bodyKey?: string }[],
  viewState: PluginViewState,
  cache: UiRefreshCache,
): PluginUiRefreshRequest | null {
  if (scope.kind === "none") return null;
  const windows = windowInstances.filter((instance) => uiRefreshWantsWindow(scope, instance.bodyKey)).map((instance) => ({ key: instance.id, bodyKey: instance.bodyKey, hash: cache.get(`window:${instance.id}`)?.hash }));
  const panels = panelTabLeaves
    .filter((tab): tab is { readonly kind: string; readonly bodyKey: string } => Boolean(tab.bodyKey) && uiRefreshWantsPanel(scope, tab.bodyKey!))
    .map((tab) => ({ key: panelTabKindId(tab.kind), bodyKey: tab.bodyKey, hash: cache.get(`panel:${panelTabKindId(tab.kind)}`)?.hash }));
  const engagements = uiRefreshWantsFlag(scope, "engagements") ? { hash: cache.get("engagements")?.hash } : undefined;
  const measures = uiRefreshWantsFlag(scope, "measures") ? { hash: cache.get("measures")?.hash } : undefined;
  const tools = uiRefreshWantsFlag(scope, "tools") ? { hash: cache.get("tools")?.hash } : undefined;
  const labels = uiRefreshWantsFlag(scope, "labels") ? { hash: cache.get("labels")?.hash } : undefined;
  if (windows.length === 0 && panels.length === 0 && !engagements && !measures && !tools && !labels) return null;
  return { viewState, windows, panels, engagements, measures, tools, labels };
}

/** @emoji 🐢️ Writes every changed section (`value !== undefined`) from a `refresh-ui` response into `cache`; unchanged sections are left as-is since the cached value is still current. */
function applyUiRefreshSectionsToCache(cache: UiRefreshCache, prefix: string, entries: readonly PluginUiRefreshSectionResponse[] | undefined): void {
  for (const entry of entries ?? []) {
    if (entry.value !== undefined) cache.set(`${prefix}:${entry.key}`, { hash: entry.hash, value: entry.value });
  }
}

export function applyUiRefreshResponseToCache(cache: UiRefreshCache, response: PluginUiRefreshResponse): void {
  applyUiRefreshSectionsToCache(cache, "window", response.windows);
  applyUiRefreshSectionsToCache(cache, "panel", response.panels);
  if (response.engagements?.value !== undefined) cache.set("engagements", { hash: response.engagements.hash, value: response.engagements.value });
  if (response.measures?.value !== undefined) cache.set("measures", { hash: response.measures.hash, value: response.measures.value });
  if (response.tools?.value !== undefined) cache.set("tools", { hash: response.tools.hash, value: response.tools.value });
  if (response.labels?.value !== undefined) cache.set("labels", { hash: response.labels.hash, value: response.labels.value });
}
//#endregion UiRefresh
//#endregion ShellHelpers
