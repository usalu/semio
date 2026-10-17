/** 🎬️ Tutorial caption, video and ghost-pointer overlay hosts, observed interaction actions, UI snapshot diffing and the tutorial recorder class.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | ShellHost}, which re-exports every public name below so import specifiers stay unchanged.
 */

import React from "react";
// #region 🔌️Adapters
import {
  type ActionDescriptor,
  CLEAR_SELECTION_ACTION_ID,
  INTERACTION_SELECT_ACTION_ID,
  type LocalizedLabel,
  SET_SELECTION_MODE_ACTION_ID,
  SELECT_ALL_ACTION_ID,
  type TutorialAssetSrc,
  type TutorialCameraState,
  type TutorialChapter,
  type TutorialDefinition,
  type TutorialEvent,
  type TutorialGestureCue,
  type TutorialUiChange,
  type TutorialUiSnapshot,
  type TutorialVideoCue
} from "@semio-tech/framework";
import { type LocalInteractionState } from "@semio-tech/framework/replication";
import {
  TutorialCaptions,
  type TutorialClockPort,
  tutorialCuesBetween,
  TutorialGhostPointer,
  TutorialVideoOverlay,
  useTutorialClock
} from "@semio-tech/ui-react";
// #endregion 🔌️Adapters
import { resolveManifestLabel, synthesizeLocalizedLabel } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";

//#region FrameworkOsShell
//#region 🎥️TutorialOverlayHosts
/** 📦️ Resolves a `TutorialAssetSrc` to a value usable as an `<video>`/`<audio>` `src` — `Blob` (a
 * studio `BlobStore` reference) isn't resolvable from this scope (no blob-store bridge here) and returns
 * `null` with a console warning; `Url`/`DataUrl` resolve directly. */
function tutorialAssetSrcToUrl(src: TutorialAssetSrc): string | null {
  if (src.kind === "url") return src.url;
  if (src.kind === "dataUrl") return src.data;
  return null;
}

/** 💬️ Self-subscribes to the tutorial clock (see `useTutorialClock`) so only THIS leaf re-renders every frame — never the whole shell — mirroring `TutorialBar`'s own subscription. */
export const TutorialCaptionsHost: React.FC<{ readonly tutorial: TutorialDefinition; readonly clock: TutorialClockPort; readonly captionsOn: boolean; readonly terminology: string; readonly locale: string }> = ({ tutorial, clock, captionsOn, terminology, locale }) => {
  const timeMs = useTutorialClock(clock);
  const cue = tutorialCuesBetween(tutorial.tracks.narration, timeMs)[0] ?? null;
  return <TutorialCaptions text={cue ? resolveManifestLabel(cue.text, terminology, locale) : null} visible={captionsOn} />;
};

const TUTORIAL_DEFAULT_VIDEO_RECT = { x: 0.72, y: 0.7, width: 0.24, height: 0.24 } as const;

/** 📹️ Self-subscribes to the tutorial clock; resolves the covering `TutorialVideoCue` (if any) and its source-relative local time. */
export const TutorialVideoOverlayHost: React.FC<{ readonly tutorial: TutorialDefinition; readonly clock: TutorialClockPort; readonly muted: boolean; readonly playing: boolean; readonly rate: number }> = ({
  tutorial,
  clock,
  muted,
  playing,
  rate,
}) => {
  const timeMs = useTutorialClock(clock);
  const cue: TutorialVideoCue | null = tutorialCuesBetween(tutorial.tracks.video, timeMs)[0] ?? null;
  const src = cue ? tutorialAssetSrcToUrl(cue.src) : null;
  const localTimeMs = cue ? timeMs - cue.at + cue.sourceOffsetMs : 0;
  return <TutorialVideoOverlay src={src} rect={cue?.rect ?? TUTORIAL_DEFAULT_VIDEO_RECT} muted={muted || (cue?.muted ?? false)} playing={playing} rate={rate} localTimeMs={localTimeMs} />;
};

/** 👻️ Self-subscribes to the tutorial clock; resolves the covering `TutorialGestureCue` (if any) and progress (0–1) through it, driving `TutorialGhostPointer` off the PLAYHEAD rather than its own internal clock (unlike the introduction demonstration overlay). */
export const TutorialGhostPointerHost: React.FC<{ readonly tutorial: TutorialDefinition; readonly clock: TutorialClockPort }> = ({ tutorial, clock }) => {
  const timeMs = useTutorialClock(clock);
  const cue: TutorialGestureCue | null = tutorialCuesBetween(tutorial.tracks.gestures, timeMs)[0] ?? null;
  const progress = cue ? Math.min(1, Math.max(0, (timeMs - cue.at) / Math.max(cue.durationMs, 1))) : 0;
  return <TutorialGhostPointer cue={cue} progress={progress} />;
};
//#endregion 🎥️TutorialOverlayHosts

//#region 🎥️TutorialRecorder
export const OBSERVED_INTERACTION_ACTION_IDS = new Set([CLEAR_SELECTION_ACTION_ID, INTERACTION_SELECT_ACTION_ID, SELECT_ALL_ACTION_ID, SET_SELECTION_MODE_ACTION_ID]);

/** 📰️ Projects a tutorial selection restore onto ordinary framework interaction actions. */
export function tutorialInteractionSelectionActions(controllerId: string, selection: LocalInteractionState["selection"]): readonly ActionDescriptor[] {
  const actions: ActionDescriptor[] = [{ controllerId, action: CLEAR_SELECTION_ACTION_ID }];
  for (const domainId of Object.keys(selection).sort()) {
    const current = selection[domainId];
    if (!current || current.ids.length === 0) continue;
    if (current.ids.length > 1) actions.push({ controllerId, action: SET_SELECTION_MODE_ACTION_ID, args: { domainId, mode: "multiple" } });
    actions.push({
      controllerId,
      action: INTERACTION_SELECT_ACTION_ID,
      args: { domainId, merge: "replace", method: "pick", targets: JSON.stringify(current.ids.map((id) => ({ granularity: current.granularity, id }))) },
    });
    if (current.anchorId && current.anchorId !== current.ids[current.ids.length - 1] && current.ids.includes(current.anchorId)) {
      actions.push({
        controllerId,
        action: INTERACTION_SELECT_ACTION_ID,
        args: { domainId, merge: "additive", method: "pick", targets: JSON.stringify([{ granularity: current.granularity, id: current.anchorId }]) },
      });
    }
  }
  return actions;
}

/** ↔ Field-by-field structural diff of two `TutorialUiSnapshot`s into the sparse `TutorialUiChange`
 * alphabet — the recorder's UI-diff effect calls this every `ShellState` change while armed. */
function diffTutorialUiSnapshot(prev: TutorialUiSnapshot, next: TutorialUiSnapshot): TutorialUiChange[] {
  const changes: TutorialUiChange[] = [];
  if (prev.activeModeId !== next.activeModeId && next.activeModeId != null) changes.push({ kind: "activeMode", id: next.activeModeId });
  if (prev.focusedWindowId !== next.focusedWindowId) changes.push({ kind: "focusedWindow", id: next.focusedWindowId });
  const utilityWindowIds = new Set([...Object.keys(prev.activeUtilityByWindowId), ...Object.keys(next.activeUtilityByWindowId)]);
  for (const windowId of utilityWindowIds) {
    if (prev.activeUtilityByWindowId[windowId] !== next.activeUtilityByWindowId[windowId]) changes.push({ kind: "activeUtility", windowId, utilityId: next.activeUtilityByWindowId[windowId] });
  }
  if (prev.activeToolId !== next.activeToolId) changes.push({ kind: "activeTool", id: next.activeToolId });
  if (next.layout && JSON.stringify(prev.layout) !== JSON.stringify(next.layout)) changes.push({ kind: "layout", layout: next.layout });
  const groups = new Set([...Object.keys(prev.activePanelTabByGroup), ...Object.keys(next.activePanelTabByGroup)]);
  for (const group of groups) {
    if (prev.activePanelTabByGroup[group] !== next.activePanelTabByGroup[group]) changes.push({ kind: "panelTab", group, tabId: next.activePanelTabByGroup[group] });
  }
  if (next.panelJson != null && prev.panelJson !== next.panelJson) changes.push({ kind: "panelState", panelJson: next.panelJson });
  // 🕹️ `TutorialUiSnapshot.selectionJson` is gone — replaced by `interactionSelection`, the framework-
  // owned per-domain selection map. Diffed one domain at a time, matching the granularity every other
  // diff in this function already operates at (`activeUtilityByWindowId`/`activePanelTabByGroup`).
  const selectionDomainIds = new Set([...Object.keys(prev.interactionSelection), ...Object.keys(next.interactionSelection)]);
  for (const domainId of selectionDomainIds) {
    const prevSelection = prev.interactionSelection[domainId];
    const nextSelection = next.interactionSelection[domainId];
    const idsMatch = prevSelection !== undefined && nextSelection !== undefined && prevSelection.ids.length === nextSelection.ids.length && prevSelection.ids.every((id, index) => id === nextSelection.ids[index]);
    if (prevSelection?.granularity === nextSelection?.granularity && idsMatch) continue;
    const granularity = nextSelection?.granularity ?? prevSelection?.granularity;
    if (granularity !== undefined) changes.push({ kind: "selection", domainId, granularity, ids: nextSelection ? [...nextSelection.ids] : [] });
  }
  if (prev.openDialogId !== next.openDialogId) changes.push({ kind: "dialog", id: next.openDialogId });
  const prevTree = new Set(prev.expandedTreeIds);
  const nextTree = new Set(next.expandedTreeIds);
  for (const id of nextTree) if (!prevTree.has(id)) changes.push({ kind: "treeExpansion", id, expanded: true });
  for (const id of prevTree) if (!nextTree.has(id)) changes.push({ kind: "treeExpansion", id, expanded: false });
  if (prev.commandPanelOpen !== next.commandPanelOpen) changes.push({ kind: "commandPanel", open: next.commandPanelOpen });
  return changes;
}

/** 🎥️ Epsilon-equality for two camera poses — the recorder's 10Hz camera sampler skips writing a
 * new keyframe when the live pose hasn't meaningfully moved since the last sample. */
function tutorialCameraPoseEquals(a: TutorialCameraState, b: TutorialCameraState): boolean {
  if (a.kind !== b.kind) return false;
  if (a.kind === "orbit" && b.kind === "orbit") return a.position.every((value,index)=>{const other=b.position[index];return other!==undefined&&Math.abs(value-other)<1e-4;}) && a.target.every((value,index)=>{const other=b.target[index];return other!==undefined&&Math.abs(value-other)<1e-4;});
  if (a.kind === "canvas" && b.kind === "canvas") return Math.abs(a.x - b.x) < 1e-4 && Math.abs(a.y - b.y) < 1e-4 && Math.abs(a.zoom - b.zoom) < 1e-4;
  return false;
}

/** 🎥️ Captures a live session into a `TutorialDefinition` — a recording IS a `TutorialDefinition`,
 * so this class simply accumulates a densely-sampled one (see the Rust core doc comment on
 * `TutorialDefinition`). Deliberately produces events/UI/camera/document tracks only: webcam/mic capture
 * (`MediaRecorder`) is an explicit, reported scope cut — see the ticket close-out summary — a text-only
 * recording is still a fully valid, useful `TutorialDefinition` per the Rust model's own optionality
 * (narration/video tracks default to empty). Document `Edit` operations are NOT captured (that would
 * require intercepting the plugin's internal vcs operation stream in per-op form, which isn't exposed to
 * this shell) — also a reported scope cut; UI/camera/events still replay faithfully. */
export class TutorialRecorder {
  private readonly startedAtMs: number;
  private readonly baseUiSnapshot: TutorialUiSnapshot;
  private readonly baseDocumentJson: string | null;
  private readonly events: TutorialEvent[] = [];
  private readonly uiKeyframes: { readonly at: number; readonly sample: { readonly kind: "snapshot"; readonly state: TutorialUiSnapshot } | { readonly kind: "delta"; readonly changes: TutorialUiChange[] } }[] = [];
  private readonly cameraKeyframes: { readonly at: number; readonly windowId: string; readonly camera: TutorialCameraState; readonly easing: "easeInOut" }[] = [];
  private readonly chapters: TutorialChapter[] = [];
  private lastUiSnapshot: TutorialUiSnapshot;
  private readonly lastCameraByWindow = new Map<string, TutorialCameraState>();

  constructor(baseUiSnapshot: TutorialUiSnapshot, baseDocumentJson: string | null) {
    this.startedAtMs = performance.now();
    this.baseUiSnapshot = baseUiSnapshot;
    this.lastUiSnapshot = baseUiSnapshot;
    this.baseDocumentJson = baseDocumentJson;
  }

  private nowMs(): number {
    return Math.max(0, Math.round(performance.now() - this.startedAtMs));
  }

  recordEvent(kind: TutorialEvent["kind"]): void {
    this.events.push({ at: this.nowMs(), kind });
  }

  recordUiDiff(next: TutorialUiSnapshot): void {
    const changes = diffTutorialUiSnapshot(this.lastUiSnapshot, next);
    if (changes.length > 0) this.uiKeyframes.push({ at: this.nowMs(), sample: { kind: "delta", changes } });
    this.lastUiSnapshot = next;
  }

  recordSnapshot(state: TutorialUiSnapshot): void {
    this.uiKeyframes.push({ at: this.nowMs(), sample: { kind: "snapshot", state } });
    this.lastUiSnapshot = state;
  }

  sampleCamera(windowId: string, camera: TutorialCameraState): void {
    const prev = this.lastCameraByWindow.get(windowId);
    if (prev && tutorialCameraPoseEquals(prev, camera)) return;
    this.lastCameraByWindow.set(windowId, camera);
    this.cameraKeyframes.push({ at: this.nowMs(), windowId, camera, easing: "easeInOut" });
  }

  /** 📖️ `ui.tutorial.addChapter` — marks the current elapsed time as a scrub-bar chapter with an
   * auto-numbered title (no naming-prompt UI in this scope; a recorded tutorial's authored titles can
   * always be hand-edited in the downloaded JSON afterward). Synthesizes a `LocalizedLabel` matrix. */
  addChapter(title?: string | LocalizedLabel): void {
    const index = this.chapters.length + 1;
    const rawTitle = title ?? `Chapter ${index}`;
    this.chapters.push({ id: `chapter-${index}`, at: this.nowMs(), title: synthesizeLocalizedLabel(rawTitle) });
  }

  build(id: string, title: string | LocalizedLabel, exampleId?: string): TutorialDefinition {
    const durationMs = Math.max(1000, this.nowMs());
    return {
      id,
      title: synthesizeLocalizedLabel(title),
      durationMs,
      chapters: this.chapters,
      base: { documentDsl: this.baseDocumentJson ?? undefined, exampleId, ui: this.baseUiSnapshot, cameras: [] },
      tracks: { narration: [], video: [], events: this.events, ui: this.uiKeyframes, document: [], camera: this.cameraKeyframes, gestures: [] },
      recordedAt: new Date().toISOString(),
    };
  }
}
//#endregion 🎥️TutorialRecorder
//#endregion FrameworkOsShell
