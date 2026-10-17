/** 🔩️ General shell plumbing: the panel chrome constants, the action-history projection and the UI history hook. The action ids, example switching, leftover interaction views, presence identity, downloads and file requests and host-effect dispatch live in the child folders.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | ShellHelpers}.
 */
// #region 🔌️Adapters
import { type HistoryEntry, historyEntryKey, historyEntryLabelText, type HistoryPatch, type HistoryReprojection, type HistoryTimeTravel, type LocalizedLabel, resolveUiDirtyScope, type UiDirtyScope } from "@semio-tech/framework";
// #endregion 🔌️Adapters
import { type Anchor } from "@semio-tech/ui-react";
import { useCallback, useEffect, useState } from "react";
import { type UIHistory } from "../../🐚️Shell/👥️peer-interaction/🟦️.ts";

//#region ShellHelpers
/** ↔ Shared starting width for every panel anchor, one compact step wider than the former 280px Document panel. */
export const DEFAULT_PANEL_WIDTH_PX = 300;

/** 🌳️ Root category id for the nested dock tab tree — the top row of {@link defaultDock}'s bottom-left (Display) anchor tabs; top-left (Workbench), top-right (Details) and bottom-right (Settings) render their tabs flat instead of under a category branch. */
export const FRAMEWORK_CATEGORY_DISPLAY_ID = "framework.category.display";
/** 🎛️ Root category id bundling every command-category leaf under one expandable Command toggle on bottom-middle (mirrors Display on bottom-left). */
export const FRAMEWORK_CATEGORY_COMMAND_ID = "framework.category.command";
/** 🛠️ Root category id bundling every mode-level tool leaf under one expandable Tool toggle on
 * bottom-middle, ordered left of the Command branch (mirrors Command's own bundling on the same anchor). */
export const FRAMEWORK_CATEGORY_TOOL_ID = "framework.category.tool";

/** 🎛️ Corner/top-middle/bottom-middle anchors park their *folded* root tab row in navbar/footer chrome (via {@link PanelChromeTabBar}); while open, the floating {@link Panel} hosts the full strip on its {@link WindowChrome}. The two side-middle anchors have no navbar/footer slot, so they're absent here and fall back to `"panel"` (see the `?..:"panel"` read site), carrying their own tab bar when folded too. */
export const PANEL_TAB_BAR_HOSTS: Partial<Record<Anchor, "navbar" | "footer">> = {
  "top-left": "navbar",
  "top-middle": "navbar",
  "top-right": "navbar",
  "bottom-left": "footer",
  "bottom-middle": "footer",
  "bottom-right": "footer",
};
export const APP_BREADCRUMB_SEPARATOR = " · ";

/** 🧾️ Equal-cursor patches still apply when they carry upserts; older cursors never clobber. */
export function historyPatchShouldApplyV1(
  currentCursor: number,
  patch: Readonly<{ cursor: number; upserts?: readonly unknown[] }>,
  replace = false,
): boolean {
  if (replace) return true;
  if (patch.cursor > currentCursor) return true;
  return patch.cursor === currentCursor && (patch.upserts?.length ?? 0) > 0;
}

/** 🧾️ One program's history as the shell projects it: the cursor undo/redo act on, the rows folded under
 * {@link historyEntryKey}, the checkpoint the check-in lane watches, the live history-edit session the time-travel
 * band and window indicators show (`null` while none is open — every patch carries the session in full), the history change
 * replaying before adoption the shell's reprojection status announces (`HistoryPatch.reprojection`, `null` while none waits), and
 * how many edits the history holds (`HistoryPatch.editCount`, the `{n}` of the `history.full` notice). */
export type ShellHistoryProjectionV1 = {
  readonly cursor: number;
  readonly entries: Readonly<Record<string, HistoryEntry>>;
  readonly canUndo: boolean;
  readonly canRedo: boolean;
  readonly currentCheckpointId: string | undefined;
  readonly timeTravel: HistoryTimeTravel | null;
  readonly reprojection: HistoryReprojection | null;
  readonly editCount: number;
};

/** 🧾️ What a program whose history has not been read yet projects: no rows, no session. */
export const EMPTY_SHELL_HISTORY_PROJECTION_V1: ShellHistoryProjectionV1 = { cursor: 0, entries: {}, canUndo: false, canRedo: false, currentCheckpointId: undefined, timeTravel: null, reprojection: null, editCount: 0 };

/** 🧾️ `current` after one admitted patch ({@link historyPatchShouldApplyV1}): a snapshot replaces the rows, a delta
 * upserts them, and the session is the patch's own — a patch without one closes it. */
export function shellHistoryProjectionAfterPatchV1(current: ShellHistoryProjectionV1, patch: HistoryPatch, replace: boolean): ShellHistoryProjectionV1 {
  const entries: Record<string, HistoryEntry> = replace ? {} : { ...current.entries };
  for (const entry of patch.upserts ?? []) entries[historyEntryKey(entry)] = entry;
  return { cursor: patch.cursor, entries, canUndo: patch.canUndo ?? false, canRedo: patch.canRedo ?? false, currentCheckpointId: replace ? patch.currentCheckpointId : (patch.currentCheckpointId ?? current.currentCheckpointId), timeTravel: patch.timeTravel ?? null, reprojection: patch.reprojection ?? null, editCount: patch.editCount ?? 0 };
}

/** 🚛️ One program's projection while its document loads in steps: the host's own polled progress as the `load`
 * reprojection, every other field kept — the shell status outside the History panel then shows a load like any other
 * replay (`HistoryReprojectionStatus`). `null` ends it: a settled, failed or cancelled load leaves no `load` reprojection
 * behind, and a replay of another kind is never touched. The same object when nothing changes. */
export function shellHistoryProjectionWithLoadV1(current: ShellHistoryProjectionV1, load: { readonly completed: number; readonly total: number } | null): ShellHistoryProjectionV1 {
  const shown = current.reprojection;
  if (load === null) return shown?.kind === "load" ? { ...current, reprojection: null } : current;
  if (shown?.kind === "load" && shown.done === load.completed && shown.total === load.total) return current;
  return { ...current, reprojection: { kind: "load", done: load.completed, total: load.total } };
}

/** 🧳️ Every program's projection after one polled step of `key`'s stepped document load
 * ({@link shellHistoryProjectionWithLoadV1}) — the same map when nothing changes, so the history store wakes no reader. */
export function programHistoryProjectionsWithLoadV1(projections: Readonly<Record<string, ShellHistoryProjectionV1>>, key: string, load: { readonly completed: number; readonly total: number } | null): Readonly<Record<string, ShellHistoryProjectionV1>> {
  if (key === "") return projections;
  const current = projections[key] ?? EMPTY_SHELL_HISTORY_PROJECTION_V1;
  const next = shellHistoryProjectionWithLoadV1(current, load);
  return next === current ? projections : { ...projections, [key]: next };
}

/** 🎞️ One unsolicited mid-operation frame (`AppFrame::Invocation`, `in_reply_to` 0): the dirty scope a running
 * operation asks the shell to refresh and the history patch a running replay publishes. */
export type OperationProgressV1 = { readonly uiScope?: UiDirtyScope; readonly historyPatch?: HistoryPatch };

/** 🎞️ What the shell does with one progress frame: refresh its scope (nothing when it carried none — a bare frame is
 * never read as a full refresh) and fold its history patch into the program's projection. */
export function operationProgressPartsV1(progress: OperationProgressV1): { readonly scope: UiDirtyScope; readonly historyPatch: HistoryPatch | undefined } {
  return { scope: progress.uiScope === undefined ? { kind: "none" } : resolveUiDirtyScope(progress.uiScope), historyPatch: progress.historyPatch };
}

/** 🧾️ The framework history cursor as the shell publishes it on `data-history-json` — the ONE readable
 * statement of what undo and redo would actually do right now.
 *
 * `historyProjection` is the shell's own reduction of every `HistoryPatch` the guest sent, and it is what
 * gates the `framework.history.undo`/`.redo` controls. Until this existed nothing carried it into the
 * document, so an undo/redo assertion could only watch WHICH ACTION a chord minted and never whether the
 * history it acted on had anything to undo — measured on generation3d 6018, `historyJsonPublished: false`
 * with zero `[data-history-json]` carriers in the page (`📓️react-current-tree-battery-2026-09-14.md` §6).
 * The newest few labels ride along in cursor order so a reader can name the entry a press would revert
 * without a second round trip; the list is bounded because this is chrome state on a DOM attribute, not
 * the History panel. */
export const SHELL_HISTORY_DOM_LABELS = 8;

export function shellHistoryCursorDomV1(
  projection: {
    readonly cursor: number;
    readonly entries: Readonly<Record<string, { readonly seq: number; readonly label: LocalizedLabel; readonly actionId: string }>>;
    readonly canUndo: boolean;
    readonly canRedo: boolean;
    readonly currentCheckpointId?: string;
  },
  axes: { readonly terminology: string; readonly locale: string },
): Record<string, unknown> {
  const ordered = Object.values(projection.entries ?? {}).sort((left, right) => left.seq - right.seq);
  const recent = ordered.slice(-SHELL_HISTORY_DOM_LABELS);
  const text = (entry: { readonly label: LocalizedLabel } | undefined): string | null => (entry ? historyEntryLabelText(entry.label, axes.terminology, axes.locale) : null);
  return {
    cursor: projection.cursor,
    canUndo: Boolean(projection.canUndo),
    canRedo: Boolean(projection.canRedo),
    entries: ordered.length,
    currentCheckpointId: projection.currentCheckpointId ?? null,
    labels: recent.map((entry) => historyEntryLabelText(entry.label, axes.terminology, axes.locale)),
    actionIds: recent.map((entry) => entry.actionId),
    undoLabel: projection.canUndo ? text(ordered.findLast((entry) => entry.seq <= projection.cursor)) : null,
    redoLabel: projection.canRedo ? text(ordered.find((entry) => entry.seq > projection.cursor)) : null,
  };
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
//#endregion ShellHelpers
