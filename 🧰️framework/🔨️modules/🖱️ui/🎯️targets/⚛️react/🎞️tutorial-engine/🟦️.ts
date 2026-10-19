import type { TutorialCameraKeyframe, TutorialCameraState, TutorialDefinition, TutorialUiSnapshot, TutorialUiChange, TutorialEvent, TutorialDocumentEvent } from "../../../../🎓️onboarding/🏷️declaration/🟦️.ts";
import { reactHostPort } from "../../../🧱️elements/🔌️Ports/⚛️react/🟦️.ts";

//#region 🎬️TutorialEngine
/** ⏱️ Formats a millisecond offset as `mm:ss` (floored to the second, never negative). */
export function formatTutorialTime(ms: number): string {
  const totalSeconds = Math.max(0, Math.floor(ms / 1000));
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  return `${minutes}:${String(seconds).padStart(2, "0")}`;
}

/** ✂️ Every cue whose `[at, at + durationMs)` window covers `atMs` — shared by narration/video/gesture track lookups. */
export function tutorialCuesBetween<T extends { readonly at: number; readonly durationMs: number }>(cues: readonly T[], atMs: number): readonly T[] {
  return cues.filter((cue) => atMs >= cue.at && atMs < cue.at + cue.durationMs);
}

function tutorialEaseInOut(t: number): number {
  return t < 0.5 ? 2 * t * t : 1 - (-2 * t + 2) ** 2 / 2;
}

function tutorialLerp3(a: readonly [number, number, number], b: readonly [number, number, number], t: number): [number, number, number] {
  return [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t];
}

/** 🎥️ TS port of Rust `interpolate_tutorial_camera` — same semantics (log-space zoom, ease-in-out/linear/hold curves, mismatched-kind snap). Keep in lockstep with `framework/core/rs/lib.rs`'s `//#region 🔖️TutorialEngine`. */
export function interpolateTutorialCamera(prev: TutorialCameraKeyframe, next: TutorialCameraKeyframe, atMs: number): TutorialCameraState {
  const span = Math.max(next.at - prev.at, 1);
  const raw = Math.min(1, Math.max(0, (atMs - prev.at) / span));
  const t = next.easing === "linear" ? raw : next.easing === "hold" ? (raw >= 1 ? 1 : 0) : tutorialEaseInOut(raw);
  const prevCamera = prev.camera;
  const nextCamera = next.camera;
  if (prevCamera.kind === "orbit" && nextCamera.kind === "orbit") {
    const fov = prevCamera.fov != null && nextCamera.fov != null ? prevCamera.fov + (nextCamera.fov - prevCamera.fov) * t : (prevCamera.fov ?? nextCamera.fov);
    return { kind: "orbit", position: tutorialLerp3(prevCamera.position, nextCamera.position, t), target: tutorialLerp3(prevCamera.target, nextCamera.target, t), up: tutorialLerp3(prevCamera.up, nextCamera.up, t), fov };
  }
  if (prevCamera.kind === "canvas" && nextCamera.kind === "canvas") {
    return {
      kind: "canvas",
      x: prevCamera.x + (nextCamera.x - prevCamera.x) * t,
      y: prevCamera.y + (nextCamera.y - prevCamera.y) * t,
      zoom: Math.exp(Math.log(prevCamera.zoom) + (Math.log(nextCamera.zoom) - Math.log(prevCamera.zoom)) * t),
    };
  }
  return t < 0.5 ? prevCamera : nextCamera;
}

/** 🎥️ TS port of Rust `tutorial_camera_at`. */
export function tutorialCameraAt(def: TutorialDefinition, windowId: string, atMs: number): TutorialCameraState | undefined {
  const keyframes = [...def.base.cameras, ...def.tracks.camera].filter((keyframe) => keyframe.windowId === windowId);
  const first = keyframes[0];
  if (!first) return undefined;
  if (atMs <= first.at) return first.camera;
  for (let index = 0; index < keyframes.length - 1; index++) {
    const prev = keyframes[index];
    const next=keyframes[index+1];if(!prev||!next)throw Error("Complete tutorial camera keyframe sequence required");
    if (atMs <= next.at) return interpolateTutorialCamera(prev, next, atMs);
  }
  return keyframes.at(-1)?.camera;
}

/** 🩹️ TS port of Rust `apply_tutorial_ui_change` — immutable (returns a new snapshot) rather than in-place, since TS callers never hold a live `&mut`. */
export function applyTutorialUiChange(state: TutorialUiSnapshot, change: TutorialUiChange): TutorialUiSnapshot {
  switch (change.kind) {
    case "activeMode":
      return { ...state, activeModeId: change.id };
    case "focusedWindow":
      return { ...state, focusedWindowId: change.id };
    case "activeUtility": {
      const next = { ...state.activeUtilityByWindowId };
      if (change.utilityId) next[change.windowId] = change.utilityId;
      else delete next[change.windowId];
      return { ...state, activeUtilityByWindowId: next };
    }
    case "activeTool":
      return { ...state, activeToolId: change.id };
    case "layout":
      return { ...state, layout: change.layout };
    case "panelTab": {
      const next = { ...state.activePanelTabByGroup };
      if (change.tabId) next[change.group] = change.tabId;
      else delete next[change.group];
      return { ...state, activePanelTabByGroup: next };
    }
    case "panelState":
      return { ...state, panelJson: change.panelJson };
    case "selection": {
      const next = { ...state.interactionSelection };
      if (change.ids.length === 0) delete next[change.domainId];
      else next[change.domainId] = { granularity: change.granularity, ids: [...change.ids] };
      return { ...state, interactionSelection: next };
    }
    case "dialog":
      return { ...state, openDialogId: change.id };
    case "treeExpansion": {
      const has = state.expandedTreeIds.includes(change.id);
      if (change.expanded) return has ? state : { ...state, expandedTreeIds: [...state.expandedTreeIds, change.id] };
      return has ? { ...state, expandedTreeIds: state.expandedTreeIds.filter((id) => id !== change.id) } : state;
    }
    case "commandPanel":
      return { ...state, commandPanelOpen: change.open };
    default:
      return state;
  }
}

/** 🧮️ TS port of Rust `compose_tutorial_ui`. */
export function composeTutorialUi(def: TutorialDefinition, atMs: number): TutorialUiSnapshot {
  let state = def.base.ui;
  let deltas: TutorialUiChange[] = [];
  for (const keyframe of def.tracks.ui) {
    if (keyframe.at > atMs) break;
    if (keyframe.sample.kind === "snapshot") {
      state = keyframe.sample.state;
      deltas = [];
    } else {
      deltas = [...deltas, ...keyframe.sample.changes];
    }
  }
  for (const change of deltas) state = applyTutorialUiChange(state, change);
  return state;
}

/** ✂️ TS mirror of Rust `TutorialSlice` — see `tutorialSlice` below. */
export type TutorialSlice = {
  readonly forward: boolean;
  readonly events: readonly TutorialEvent[];
  readonly document: readonly TutorialDocumentEvent[];
  readonly uiChanges: readonly TutorialUiChange[];
};

/** ✂️ TS port of Rust `tutorial_slice` — same directionality contract (forward: oldest→newest;
 * backward: newest→oldest so `Edit.backwards` unwind in the right order). Never spans a
 * `TutorialUiSample::Snapshot` boundary correctly on its own — a seek/scrub must call
 * {@link composeTutorialUi} wholesale instead, exactly like the Rust doc comment warns. */
export function tutorialSlice(def: TutorialDefinition, fromMs: number, toMs: number): TutorialSlice {
  const forward = toMs >= fromMs;
  const lo = Math.min(fromMs, toMs);
  const hi = Math.max(fromMs, toMs);
  const inRange = (at: number) => at > lo && at <= hi;
  let events = def.tracks.events.filter((event) => inRange(event.at));
  let document = def.tracks.document.filter((event) => inRange(event.at));
  let uiChanges: TutorialUiChange[] = [];
  for (const keyframe of def.tracks.ui) {
    if (!inRange(keyframe.at)) continue;
    if (keyframe.sample.kind === "delta") uiChanges = [...uiChanges, ...keyframe.sample.changes];
  }
  if (!forward) {
    events = [...events].reverse();
    document = [...document].reverse();
    uiChanges = [...uiChanges].reverse();
  }
  return { forward, events, document, uiChanges };
}

/** ✅️ TS port of Rust `validate_tutorial` — light structural sanity check shared by the recorder before download; does not validate referenced action/command/element ids (no `AppDefinition` in scope here). Returns the first error found, or `null`. */
export function validateTutorial(def: TutorialDefinition): string | null {
  const sortedByAt = <T,>(label: string, items: readonly T[], at: (item: T) => number): string | null => {
    let last: number | null = null;
    for (const item of items) {
      const value = at(item);
      if (value > def.durationMs) return `tutorial track \`${label}\` has an entry at ${value}ms beyond durationMs ${def.durationMs}`;
      if (last != null && value < last) return `tutorial track \`${label}\` is not sorted ascending by \`at\` (${last}ms then ${value}ms)`;
      last = value;
    }
    return null;
  };
  const checks: readonly (() => string | null)[] = [
    () => sortedByAt("chapters", def.chapters, (c) => c.at),
    () => sortedByAt("narration", def.tracks.narration, (c) => c.at),
    () => sortedByAt("video", def.tracks.video, (c) => c.at),
    () => sortedByAt("events", def.tracks.events, (e) => e.at),
    () => sortedByAt("ui", def.tracks.ui, (k) => k.at),
    () => sortedByAt("document", def.tracks.document, (e) => e.at),
    () => sortedByAt("camera", def.tracks.camera, (k) => k.at),
    () => sortedByAt("gestures", def.tracks.gestures, (c) => c.at),
  ];
  for (const check of checks) {
    const error = check();
    if (error) return error;
  }
  const chapterIds = new Set<string>();
  for (const chapter of def.chapters) {
    if (chapterIds.has(chapter.id)) return `duplicate tutorial chapter id \`${chapter.id}\``;
    chapterIds.add(chapter.id);
  }
  const cueIds = new Set<string>();
  for (const cue of def.tracks.narration) {
    if (cueIds.has(cue.id)) return `duplicate tutorial narration cue id \`${cue.id}\``;
    cueIds.add(cue.id);
  }
  for (const camera of def.base.cameras) {
    if (camera.at !== 0) return `tutorial base camera keyframe for window \`${camera.windowId}\` must have at == 0`;
  }
  return null;
}
//#endregion 🎬️TutorialEngine

/** ⏱️ The minimal read+subscribe surface {@link TutorialBar}/{@link TutorialCaptions}/{@link TutorialVideoOverlay}/{@link TutorialGhostPointer} need — lets every per-frame time consumer self-subscribe via `useSyncExternalStore` instead of the whole shell re-rendering on every tick. */
export type TutorialClockPort = {
  readonly getTimeMs: () => number;
  readonly subscribe: (callback: () => void) => () => void;
};

/** ⏱️ Full imperative control surface for a `TutorialClockPort` — owned by the shell orchestration (director), read by the UI kit. */
export type TutorialClock = TutorialClockPort & {
  readonly play: () => void;
  readonly pause: () => void;
  readonly seek: (ms: number) => void;
  readonly setRate: (rate: number) => void;
  readonly setDurationMs: (durationMs: number) => void;
  readonly isPlaying: () => boolean;
  readonly getRate: () => number;
  readonly dispose: () => void;
};

/** ⏱️ Creates a tiny external store driven by `requestAnimationFrame`: `t += dtWallClock * rate` while playing, auto-pausing at `durationMs`. Not a React hook itself — subscribe via {@link useTutorialClock} (or any `useSyncExternalStore`) for the reactive read. */
export function createTutorialClock(durationMs: number): TutorialClock {
  let tMs = 0;
  let rate = 1;
  let playing = false;
  let duration = Math.max(0, durationMs);
  let lastFrameAt: number | null = null;
  let rafHandle: number | null = null;
  const subscribers = new Set<() => void>();
  const notify = () => subscribers.forEach((callback) => callback());
  const tick = (now: number) => {
    if (!playing) {
      rafHandle = null;
      return;
    }
    const dt = lastFrameAt == null ? 0 : now - lastFrameAt;
    lastFrameAt = now;
    tMs = Math.min(duration, Math.max(0, tMs + dt * rate));
    if (tMs >= duration) playing = false;
    notify();
    rafHandle = playing ? requestAnimationFrame(tick) : null;
  };
  return {
    getTimeMs: () => tMs,
    subscribe: (callback) => {
      subscribers.add(callback);
      return () => subscribers.delete(callback);
    },
    play: () => {
      if (playing || duration <= 0) return;
      playing = true;
      lastFrameAt = null;
      if (rafHandle == null) rafHandle = requestAnimationFrame(tick);
      notify();
    },
    pause: () => {
      if (!playing) return;
      playing = false;
      if (rafHandle != null) {
        cancelAnimationFrame(rafHandle);
        rafHandle = null;
      }
      notify();
    },
    seek: (ms) => {
      tMs = Math.min(duration, Math.max(0, ms));
      lastFrameAt = playing ? performance.now() : null;
      notify();
    },
    setRate: (nextRate) => {
      rate = nextRate;
      notify();
    },
    setDurationMs: (nextDuration) => {
      duration = Math.max(0, nextDuration);
      tMs = Math.min(tMs, duration);
      notify();
    },
    isPlaying: () => playing,
    getRate: () => rate,
    dispose: () => {
      if (rafHandle != null) cancelAnimationFrame(rafHandle);
      subscribers.clear();
    },
  };
}

/** ⏱️ Subscribes the calling component to a {@link TutorialClockPort}'s per-frame time — only this component re-renders on tick, never the whole shell. */
export function useTutorialClock(clock: TutorialClockPort): number {
  return reactHostPort.useSyncExternalStore(clock.subscribe, clock.getTimeMs, clock.getTimeMs);
}
