/** 🧮️ Gumball and paint gesture state machines, live-preview math, owner poses and the gumball transform preview store.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | World3dHost}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { useEffect, useState } from "../../⚛️react-port/🟦️.ts";
// #region 🔌️Adapters
import {
  gumballHandleKindToTransformMode,
  type GumballConfig,
  type GumballHandleKind,
  type GumballPose,
  Group,
  Object3D,
  Quaternion,
  Vector3
} from "@semio-tech/ui-react";
// #endregion 🔌️Adapters
import { worldProjectionGumballPlane, type WorldProjectionSpec } from "@semio-tech/framework/infinite/world/r3f";
import { type WorldAttractionRecord, type WorldInstanceRecord, type WorldVortexRecord } from "../🔭️scene-schema/🟦️.ts";
import { useThree } from "../🎨️mesh-render/🟦️.tsx";

//#region 🔖️World3dHost
//#region WorldSceneParsing
/** 🎛️ True when `mode` is an explicit transform-gumball utility (`move`/`rotate`/`scale`/`transform`) — never treat a missing/unknown mode as move. */
export function isWorldTransformGumballMode(mode: string | undefined): boolean {
  return mode === "move" || mode === "rotate" || mode === "scale" || mode === "transform";
}

function gumballKindForTransformMode(transformMode: string | undefined, handleKind?: GumballHandleKind): "translate" | "rotate" | "scale" {
  if ((transformMode === "transform" || transformMode === "move") && handleKind != null) {
    return gumballHandleKindToTransformMode(handleKind);
  }
  if (transformMode === "rotate") return "rotate";
  if (transformMode === "scale") return "scale";
  return "translate";
}

export const GUMBALL_TRANSFORM_EPSILON = 1e-6;

/** 🧷️ Copies the plugin's opaque transform targets so a gesture keeps its initial selection. */
export function world3dGumballSelectionArgsV1(selection: {
  readonly ids?: readonly string[];
  readonly gumballSelectionIds?: readonly string[];
  readonly componentIds?: readonly number[];
  readonly selectionMode?: string;
  readonly granularity?: string;
}): { readonly mode: string; readonly ids: readonly string[] } {
  return { mode: selection.selectionMode ?? selection.granularity ?? "object", ids: [...(selection.gumballSelectionIds ?? selection.ids ?? [])] };
}

/** 🎛️ Builds one incremental `translateSelection` / `rotateSelection` / `scaleSelection` dispatch from consecutive gumball poses. */
export function gumballTransformDeltaBetweenPoses(
  transformMode: string | undefined,
  before: GumballPose,
  after: GumballPose,
  base: Record<string, unknown>,
  handleKind?: GumballHandleKind,
): { readonly action: string; readonly args: Record<string, unknown> } | null {
  const kind = gumballKindForTransformMode(transformMode, handleKind);
  if (kind === "translate") {
    const dx = after.position[0] - before.position[0];
    const dy = after.position[1] - before.position[1];
    const dz = after.position[2] - before.position[2];
    if (Math.abs(dx) < GUMBALL_TRANSFORM_EPSILON && Math.abs(dy) < GUMBALL_TRANSFORM_EPSILON && Math.abs(dz) < GUMBALL_TRANSFORM_EPSILON) {
      return null;
    }
    return { action: "translateSelection", args: { ...base, dx, dy, dz } };
  }
  if (kind === "rotate") {
    const beforeQuat = new Quaternion(...before.quaternion);
    const afterQuat = new Quaternion(...after.quaternion);
    const delta = afterQuat.multiply(beforeQuat.invert());
    const angle = 2 * Math.acos(Math.min(1, Math.max(-1, delta.w)));
    if (angle < GUMBALL_TRANSFORM_EPSILON) return null;
    const sinHalfAngle = Math.sqrt(Math.max(0, 1 - delta.w * delta.w));
    const axis = sinHalfAngle < 1e-6 ? { x: 0, y: 0, z: 1 } : { x: delta.x / sinHalfAngle, y: delta.y / sinHalfAngle, z: delta.z / sinHalfAngle };
    return { action: "rotateSelection", args: { ...base, ax: axis.x, ay: axis.y, az: axis.z, angle } };
  }
  const sx = after.scale[0] / Math.max(before.scale[0], GUMBALL_TRANSFORM_EPSILON);
  const sy = after.scale[1] / Math.max(before.scale[1], GUMBALL_TRANSFORM_EPSILON);
  const sz = after.scale[2] / Math.max(before.scale[2], GUMBALL_TRANSFORM_EPSILON);
  if (Math.abs(sx - 1) < GUMBALL_TRANSFORM_EPSILON && Math.abs(sy - 1) < GUMBALL_TRANSFORM_EPSILON && Math.abs(sz - 1) < GUMBALL_TRANSFORM_EPSILON) {
    return null;
  }
  return { action: "scaleSelection", args: { ...base, sx, sy, sz } };
}

/** 🛑️ The pose delta that moves nothing for a gumball handle kind — what a live gesture's `commit` carries when its
 * tail moved nothing, and the verb its `abort` names. */
export function gumballIdentityDelta(transformMode: string | undefined, base: Record<string, unknown>, handleKind?: GumballHandleKind): { readonly action: string; readonly args: Record<string, unknown> } {
  const kind = gumballKindForTransformMode(transformMode, handleKind);
  if (kind === "translate") return { action: "translateSelection", args: { ...base, dx: 0, dy: 0, dz: 0 } };
  if (kind === "rotate") return { action: "rotateSelection", args: { ...base, ax: 0, ay: 0, az: 1, angle: 0 } };
  return { action: "scaleSelection", args: { ...base, sx: 1, sy: 1, sz: 1 } };
}

/** 🖌️ Where a paint dab under the pointer comes from: the press that opens a paint gesture, a drag over a mesh while it is
 * held, or the click that follows a release — a gesture's dabs stream into ONE open transaction the release commits. */
export type WorldPaintOrigin = "press" | "drag" | "click";

/** 🖌️ A World3d paint gesture between events: whether its first dab streamed into the app's open tool transaction, and
 * whether the click that follows its release belongs to it. */
export type WorldPaintGesture = { readonly streamed: boolean; readonly clickConsumed: boolean };

/** 🖌️ What reaches a World3d paint gesture: a dab under the pointer, a host press, the window-level release, or a host
 * cancel (`blur`, `captureLost`). */
export type WorldPaintEvent =
  | { readonly kind: "dab"; readonly origin: WorldPaintOrigin; readonly objectId: string; readonly u: number; readonly v: number }
  | { readonly kind: "press" }
  | { readonly kind: "release" }
  | { readonly kind: "cancel"; readonly reason: "blur" | "captureLost" };

/** 🖌️ The gesture before any press. */
export const WORLD_PAINT_IDLE: WorldPaintGesture = { streamed: false, clickConsumed: false };

/** 🛠️ One step of a World3d paint gesture, pure: a press or drag dab streams into the app's ONE open tool transaction
 * (`paintAt` with `phase: "stream"`), the release commits it (`phase: "commit"`) and consumes the click that follows, a
 * cancel drops it with zero trace (`phase: "abort"` + `reason`); a click with nothing streamed is a one-shot dab (no
 * `phase`). Returns the next gesture and the one `paintAt` to dispatch, if any — the same protocol the wgpu world engine
 * publishes (a press pick, move picks, the release). */
export function worldPaintStep(gesture: WorldPaintGesture, event: WorldPaintEvent): { readonly gesture: WorldPaintGesture; readonly dispatch: Record<string, unknown> | null } {
  switch (event.kind) {
    case "press":
      return { gesture: { ...gesture, clickConsumed: false }, dispatch: null };
    case "dab":
      if (event.origin !== "click") return { gesture: { ...gesture, streamed: true }, dispatch: { objectId: event.objectId, u: event.u, v: event.v, phase: "stream" } };
      if (gesture.clickConsumed) return { gesture: { ...gesture, clickConsumed: false }, dispatch: null };
      return { gesture, dispatch: { objectId: event.objectId, u: event.u, v: event.v } };
    case "release":
      return gesture.streamed ? { gesture: { streamed: false, clickConsumed: true }, dispatch: { phase: "commit" } } : { gesture, dispatch: null };
    case "cancel":
      return gesture.streamed ? { gesture: { streamed: false, clickConsumed: false }, dispatch: { phase: "abort", reason: event.reason } } : { gesture, dispatch: null };
  }
}

/** 🕹️ What a gumball gesture grabbed — the selection mode and the opaque target ids, pinned at drag start. */
export type WorldGumballTargets = { readonly mode: string; readonly ids: readonly string[] };

/** 🛠️ A World3d gumball gesture between events: whether the selection record asked for live dispatch when it started,
 * the transform mode and handle it drags, its pinned targets, the pose its last delta reached (`sent`) and the newest
 * pose the pointer reported, whether a stream tick is in flight, whether a tick opened the app's tool transaction
 * (`streamed`), and whether a host cancel aborted it (the rest of the drag is swallowed). */
export type WorldGumballGesture = {
  readonly live: boolean;
  readonly transformMode: string | undefined;
  readonly handle: GumballHandleKind | null;
  readonly targets: WorldGumballTargets | null;
  readonly sent: GumballPose | null;
  readonly latest: GumballPose | null;
  readonly inFlight: boolean;
  readonly streamed: boolean;
  readonly aborted: boolean;
};

/** 🎛️ What reaches a World3d gumball gesture: the handle grab, a pointer pose while held, the settle of the stream tick
 * in flight, the release (with the grab pose and the live targets as fallbacks for a release without a grab), or a host
 * cancel (`blur`, `captureLost`). */
export type WorldGumballEvent =
  | { readonly kind: "start"; readonly handle: GumballHandleKind; readonly pose: GumballPose; readonly targets: WorldGumballTargets; readonly live: boolean; readonly transformMode: string | undefined }
  | { readonly kind: "drag"; readonly handle: GumballHandleKind; readonly pose: GumballPose }
  | { readonly kind: "settled" }
  | { readonly kind: "release"; readonly handle: GumballHandleKind; readonly before: GumballPose; readonly after: GumballPose; readonly targets: WorldGumballTargets }
  | { readonly kind: "cancel"; readonly reason: "blur" | "captureLost" };

/** 📨️ One gumball verb dispatch: `translateSelection` / `rotateSelection` / `scaleSelection` with its args. */
export type WorldGumballDispatch = { readonly action: string; readonly args: Record<string, unknown> };

/** 🧾️ One gumball step: the next gesture, the one dispatch it owes (if any), and why a delta it owed was skipped. */
export type WorldGumballStepResult = { readonly gesture: WorldGumballGesture; readonly dispatch: WorldGumballDispatch | null; readonly skipped?: "no-selection-ids" | "no-delta" };

/** 🫥️ The gesture before any grab. */
export const WORLD_GUMBALL_IDLE: WorldGumballGesture = { live: false, transformMode: undefined, handle: null, targets: null, sent: null, latest: null, inFlight: false, streamed: false, aborted: false };

function worldGumballDelta(transformMode: string | undefined, targets: WorldGumballTargets, handle: GumballHandleKind, before: GumballPose, after: GumballPose, phase?: "stream" | "commit"): Omit<WorldGumballStepResult, "gesture"> {
  if (targets.ids.length === 0) return { dispatch: null, skipped: "no-selection-ids" };
  const base = { mode: targets.mode, ids: [...targets.ids] };
  const payload = gumballTransformDeltaBetweenPoses(transformMode, before, after, base, handle) ?? (phase === "commit" ? gumballIdentityDelta(transformMode, base, handle) : null);
  if (!payload) return { dispatch: null, skipped: "no-delta" };
  return { dispatch: phase ? { action: payload.action, args: { ...payload.args, phase } } : payload };
}

function worldGumballPump(gesture: WorldGumballGesture): WorldGumballStepResult {
  const { sent, latest, handle, targets } = gesture;
  if (!gesture.live || gesture.aborted || gesture.inFlight || !sent || !latest || !handle || !targets || gumballPosesEqual(sent, latest)) return { gesture, dispatch: null };
  const { dispatch, skipped } = worldGumballDelta(gesture.transformMode, targets, handle, sent, latest, "stream");
  return { gesture: { ...gesture, sent: latest, inFlight: dispatch !== null, streamed: gesture.streamed || dispatch !== null }, dispatch, skipped };
}

/** 🛠️ One step of a World3d gumball gesture, pure — the host side of the gumball tool protocol
 * (`🧫️fixtures/🛠️gumball-live-protocol.json`). Local (no `gumballLiveDispatch`): the host previews the drag itself and the
 * release dispatches ONE net delta with no `phase` (a one-shot tool transaction), nothing when the pose never moved along
 * the handle. Live: every pose reported while no tick is in flight dispatches the delta from the pose the last tick
 * reached as `phase: "stream"` (one in flight, the newest pose wins), the release dispatches the remaining tail as
 * `phase: "commit"` (the identity delta of the handle's verb when the tail moved nothing), and a host cancel after a
 * streamed tick dispatches the verb's identity delta with `phase: "abort"` + `reason` and swallows the rest of the drag.
 * The targets are the ones pinned at the grab, never the selection as it stands later. */
export function worldGumballStep(gesture: WorldGumballGesture, event: WorldGumballEvent): WorldGumballStepResult {
  switch (event.kind) {
    case "start":
      return {
        gesture: { ...WORLD_GUMBALL_IDLE, live: event.live, transformMode: event.transformMode, handle: event.handle, targets: { mode: event.targets.mode, ids: [...event.targets.ids] }, sent: event.pose, latest: event.pose, inFlight: gesture.inFlight },
        dispatch: null,
      };
    case "drag":
      return worldGumballPump({ ...gesture, latest: event.pose, handle: event.handle });
    case "settled":
      return worldGumballPump({ ...gesture, inFlight: false });
    case "release": {
      const idle = { ...WORLD_GUMBALL_IDLE, inFlight: gesture.inFlight };
      if (gesture.aborted) return { gesture: idle, dispatch: null };
      const from = gesture.sent ?? event.before;
      if (!gesture.live && gumballPosesEqual(from, event.after)) return { gesture: idle, dispatch: null };
      return { gesture: idle, ...worldGumballDelta(gesture.transformMode, gesture.targets ?? event.targets, event.handle, from, event.after, gesture.live ? "commit" : undefined) };
    }
    case "cancel": {
      if (!gesture.live || !gesture.streamed || !gesture.handle || !gesture.targets) return { gesture, dispatch: null };
      const verb = gumballIdentityDelta(gesture.transformMode, { mode: gesture.targets.mode, ids: [...gesture.targets.ids] }, gesture.handle);
      return { gesture: { ...WORLD_GUMBALL_IDLE, live: true, aborted: true, inFlight: gesture.inFlight }, dispatch: { action: verb.action, args: { ...verb.args, phase: "abort", reason: event.reason } } };
    }
  }
}

/** ⚡️ Local mid-drag gumball preview delta — applied imperatively to selected instance roots so meshes track the pointer without a WASM/React round-trip (same instant path as catalogue drop ghosts). */
export type WorldGumballLivePreviewDelta =
  | { readonly kind: "translate"; readonly dx: number; readonly dy: number; readonly dz: number }
  | { readonly kind: "rotate"; readonly qx: number; readonly qy: number; readonly qz: number; readonly qw: number }
  | { readonly kind: "scale"; readonly sx: number; readonly sy: number; readonly sz: number };

/** ⚡️ Absolute start→current gumball preview delta for local instance transforms. */
export function gumballLivePreviewDeltaBetweenPoses(
  transformMode: string | undefined,
  before: GumballPose,
  after: GumballPose,
  handleKind?: GumballHandleKind,
): WorldGumballLivePreviewDelta | null {
  const kind = gumballKindForTransformMode(transformMode, handleKind);
  if (kind === "translate") {
    const dx = after.position[0] - before.position[0];
    const dy = after.position[1] - before.position[1];
    const dz = after.position[2] - before.position[2];
    if (Math.abs(dx) < GUMBALL_TRANSFORM_EPSILON && Math.abs(dy) < GUMBALL_TRANSFORM_EPSILON && Math.abs(dz) < GUMBALL_TRANSFORM_EPSILON) {
      return null;
    }
    return { kind: "translate", dx, dy, dz };
  }
  if (kind === "rotate") {
    const beforeQuat = new Quaternion(before.quaternion[0], before.quaternion[1], before.quaternion[2], before.quaternion[3]);
    const afterQuat = new Quaternion(after.quaternion[0], after.quaternion[1], after.quaternion[2], after.quaternion[3]);
    const delta = afterQuat.multiply(beforeQuat.invert());
    if (2 * Math.acos(Math.min(1, Math.max(-1, delta.w))) < GUMBALL_TRANSFORM_EPSILON) return null;
    return { kind: "rotate", qx: delta.x, qy: delta.y, qz: delta.z, qw: delta.w };
  }
  const sx = after.scale[0] / Math.max(before.scale[0], GUMBALL_TRANSFORM_EPSILON);
  const sy = after.scale[1] / Math.max(before.scale[1], GUMBALL_TRANSFORM_EPSILON);
  const sz = after.scale[2] / Math.max(before.scale[2], GUMBALL_TRANSFORM_EPSILON);
  if (Math.abs(sx - 1) < GUMBALL_TRANSFORM_EPSILON && Math.abs(sy - 1) < GUMBALL_TRANSFORM_EPSILON && Math.abs(sz - 1) < GUMBALL_TRANSFORM_EPSILON) {
    return null;
  }
  return { kind: "scale", sx, sy, sz };
}

export type WorldGumballLivePose = {
  readonly position: readonly [number, number, number];
  readonly quaternion: readonly [number, number, number, number];
  readonly scale: readonly [number, number, number];
};

/** ⚡️ Applies a local gumball preview delta onto a drag-start instance pose (matches puzzle/lowpoly/CAD scratch translate/rotate/scale semantics). */
export function applyGumballLivePreviewDeltaToPose(base: WorldGumballLivePose, delta: WorldGumballLivePreviewDelta): WorldGumballLivePose {
  if (delta.kind === "translate") {
    return {
      position: [base.position[0] + delta.dx, base.position[1] + delta.dy, base.position[2] + delta.dz],
      quaternion: base.quaternion,
      scale: base.scale,
    };
  }
  if (delta.kind === "rotate") {
    const next = new Quaternion(delta.qx, delta.qy, delta.qz, delta.qw).multiply(
      new Quaternion(base.quaternion[0], base.quaternion[1], base.quaternion[2], base.quaternion[3]),
    );
    return {
      position: base.position,
      quaternion: [next.x, next.y, next.z, next.w],
      scale: base.scale,
    };
  }
  return {
    position: base.position,
    quaternion: base.quaternion,
    scale: [base.scale[0] * delta.sx, base.scale[1] * delta.sy, base.scale[2] * delta.sz],
  };
}

/**
 * 🧿️ Whether the `<group>` ref the reconciler handed back is a real three.js instance root that
 * owns a transform to write. The host mounts under hosts other than the three.js reconciler — jsdom
 * gesture mounts and `renderToStaticMarkup` — where the very same ref yields a DOM node instead, so
 * the imperative gumball preview registry admits only what it can actually pose. `isObject3D` is
 * three's own brand, which survives the realm boundary a bundled `instanceof` does not.
 *
 * @see https://threejs.org/docs/#api/en/core/Object3D.isObject3D
 */
export function isWorldInstanceRootObject3D(candidate: unknown): candidate is Group {
  return typeof candidate === "object" && candidate !== null && (candidate as { isObject3D?: unknown }).isObject3D === true;
}

/** ⚡️ Writes a live gumball preview pose onto a Three.js instance root. */
export function applyGumballLivePreviewPoseToObject3D(target: Object3D, pose: WorldGumballLivePose): void {
  target.position.set(pose.position[0], pose.position[1], pose.position[2]);
  target.quaternion.set(pose.quaternion[0], pose.quaternion[1], pose.quaternion[2], pose.quaternion[3]);
  target.scale.set(pose.scale[0], pose.scale[1], pose.scale[2]);
  target.updateMatrixWorld(true);
}

/** 🧭️ The drag-start pose of the instance a marker belongs to: the origin and orientation it turns and scales about. */
export type WorldGumballOwnerPose = {
  readonly position: readonly [number, number, number];
  readonly rotation: readonly [number, number, number, number];
};

/** 🧭️ Every instance's drag-start pose by id — what a marker of that instance is previewed about. */
export function worldGumballOwnerPoses(instances: readonly WorldInstanceRecord[]): ReadonlyMap<string, WorldGumballOwnerPose> {
  const owners = new Map<string, WorldGumballOwnerPose>();
  for (const instance of instances) owners.set(instance.id, { position: instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0], rotation: instance.rotation ?? [0, 0, 0, 1] });
  return owners;
}

/**
 * 🌀 One marker point (vortex, grip, attraction end) of an instance under the gumball preview, about that instance's OWN
 * origin: a turn rotates it about the owner's origin and a scale stretches it along the owner's local axes — exactly what
 * {@link applyGumballLivePreviewDeltaToPose} does to the owner itself and what the committed selection leaves do to every
 * target (puzzle `rotate-selection`/`scale-selection`, …). The gumball pivot only anchors the widget, so a multi-object
 * turn previews the markers where the release lands them instead of orbiting them about the pivot.
 */
export function gumballPreviewOwnedWorldPoint(
  owner: WorldGumballOwnerPose,
  transformMode: string | undefined,
  before: GumballPose,
  after: GumballPose,
  handleKind: GumballHandleKind | null | undefined,
  worldPoint: readonly [number, number, number],
): readonly [number, number, number] {
  const delta = gumballLivePreviewDeltaBetweenPoses(transformMode, before, after, handleKind ?? undefined);
  if (!delta) return worldPoint;
  if (delta.kind === "translate") return [worldPoint[0] + delta.dx, worldPoint[1] + delta.dy, worldPoint[2] + delta.dz];
  const offset = new Vector3(worldPoint[0] - owner.position[0], worldPoint[1] - owner.position[1], worldPoint[2] - owner.position[2]);
  if (delta.kind === "rotate") {
    offset.applyQuaternion(new Quaternion(delta.qx, delta.qy, delta.qz, delta.qw));
  } else {
    const turn = new Quaternion(owner.rotation[0], owner.rotation[1], owner.rotation[2], owner.rotation[3]);
    offset.applyQuaternion(turn.clone().invert()).multiply(new Vector3(delta.sx, delta.sy, delta.sz)).applyQuaternion(turn);
  }
  return [owner.position[0] + offset.x, owner.position[1] + offset.y, owner.position[2] + offset.z];
}

/** 🌀 One marker direction of an instance under the gumball preview: turned with the owner, or carried through the owner's local stretch and renormalized. */
export function gumballPreviewOwnedWorldDirection(
  owner: WorldGumballOwnerPose,
  transformMode: string | undefined,
  before: GumballPose,
  after: GumballPose,
  handleKind: GumballHandleKind | null | undefined,
  direction: readonly [number, number, number],
): readonly [number, number, number] {
  const delta = gumballLivePreviewDeltaBetweenPoses(transformMode, before, after, handleKind ?? undefined);
  if (!delta || delta.kind === "translate") return direction;
  const vector = new Vector3(direction[0], direction[1], direction[2]);
  if (delta.kind === "rotate") {
    vector.applyQuaternion(new Quaternion(delta.qx, delta.qy, delta.qz, delta.qw));
  } else {
    const turn = new Quaternion(owner.rotation[0], owner.rotation[1], owner.rotation[2], owner.rotation[3]);
    vector.applyQuaternion(turn.clone().invert()).multiply(new Vector3(delta.sx, delta.sy, delta.sz)).applyQuaternion(turn);
  }
  vector.normalize();
  return [vector.x, vector.y, vector.z];
}

export function gumballConfigForTransformMode(mode: string, plane?: GumballConfig["plane"]): GumballConfig {
  const groups =
    mode === "transform"
      ? { moveAxes: true, movePlanes: true, rotate: true, scaleAxes: false, scalePlanes: false, scaleUniform: false }
      : mode === "rotate"
        ? { moveAxes: false, movePlanes: false, rotate: true, scaleAxes: false, scalePlanes: false, scaleUniform: false }
        : mode === "scale"
          ? { moveAxes: false, movePlanes: false, rotate: false, scaleAxes: true, scalePlanes: true, scaleUniform: true }
          : { moveAxes: true, movePlanes: true, rotate: false, scaleAxes: false, scalePlanes: false, scaleUniform: false };
  return plane ? { ...groups, plane } : groups;
}

/** 🎛️ Transform-mode gumball config intersected with the planar subset implied by a window projection. */
export function worldGumballConfigForProjection(mode: string, projectionSpec?: WorldProjectionSpec): GumballConfig {
  return gumballConfigForTransformMode(mode, worldProjectionGumballPlane(projectionSpec));
}

/** 🫥️ True while the canvas is hovered, or always true when `enabled` is false (driver gumball reveal is `"always"`). Listens directly on the R3F canvas DOM element — no prop drilling into the scene graph. */
export function useUiCanvasHovered(enabled: boolean): boolean {
  const gl = useThree((state) => state.gl);
  const [hovered, setHovered] = useState(!enabled);
  useEffect(() => {
    if (!enabled) {
      setHovered(true);
      return;
    }
    setHovered(false);
    const canvas = gl.domElement;
    const onEnter = () => setHovered(true);
    const onLeave = () => setHovered(false);
    canvas.addEventListener("pointerenter", onEnter);
    canvas.addEventListener("pointerleave", onLeave);
    return () => {
      canvas.removeEventListener("pointerenter", onEnter);
      canvas.removeEventListener("pointerleave", onLeave);
    };
  }, [enabled, gl]);
  return hovered;
}
//#endregion WorldSceneParsing

//#region WorldGumballTransformPreviewStore
export type WorldGumballTransformPreview = {
  readonly sourceId: string;
  readonly transformMode: string | undefined;
  readonly handleKind: GumballHandleKind | null;
  readonly before: GumballPose;
  readonly after: GumballPose;
  readonly instanceIds: readonly string[];
  readonly pivot: readonly [number, number, number];
};

/** 🧲️ Live gumball transform previews keyed by controller so every sibling World3d pane paints the same in-progress move/rotate before the plugin round-trip commits it. */
const worldGumballTransformPreviewByController = new Map<string, WorldGumballTransformPreview>();
const worldGumballTransformPreviewListeners = new Set<() => void>();

function gumballPosesEqual(a: GumballPose, b: GumballPose): boolean {
  return (
    a.position.every((value, index) => Math.abs(value - b.position[index]!) < GUMBALL_TRANSFORM_EPSILON) &&
    a.quaternion.every((value, index) => Math.abs(value - b.quaternion[index]!) < GUMBALL_TRANSFORM_EPSILON) &&
    a.scale.every((value, index) => Math.abs(value - b.scale[index]!) < GUMBALL_TRANSFORM_EPSILON)
  );
}

function worldGumballTransformPreviewsEqual(a: WorldGumballTransformPreview | null | undefined, b: WorldGumballTransformPreview | null | undefined): boolean {
  if (a === b) return true;
  if (!a || !b || a.sourceId !== b.sourceId || a.transformMode !== b.transformMode || a.handleKind !== b.handleKind) return false;
  if (a.instanceIds.length !== b.instanceIds.length || a.instanceIds.some((id, index) => id !== b.instanceIds[index])) return false;
  if (a.pivot.some((value, index) => Math.abs(value - b.pivot[index]!) > GUMBALL_TRANSFORM_EPSILON)) return false;
  return gumballPosesEqual(a.before, b.before) && gumballPosesEqual(a.after, b.after);
}

function notifyWorldGumballTransformPreviewListeners(): void {
  for (const listener of worldGumballTransformPreviewListeners) listener();
}

/** 🧲️ Subscribes a World3d pane to live gumball previews from sibling panes. */
export function subscribeWorldGumballTransformPreview(listener: () => void): () => void {
  worldGumballTransformPreviewListeners.add(listener);
  return () => {
    worldGumballTransformPreviewListeners.delete(listener);
  };
}

/** 🧲️ Returns the live gumball preview for `controllerId`, or `null` outside an in-progress gesture. */
export function getWorldGumballTransformPreview(controllerId: string): WorldGumballTransformPreview | null {
  return worldGumballTransformPreviewByController.get(controllerId) ?? null;
}

/** 🧲️ SSR snapshot for {@link useSyncExternalStore}; gumball gestures never hydrate in progress. */
export function getWorldGumballTransformPreviewServerSnapshot(_controllerId: string): WorldGumballTransformPreview | null {
  return null;
}

/** 🧲️ Publishes an in-progress gumball transform to every World3d pane using the same controller. */
export function setWorldGumballTransformPreview(controllerId: string, preview: WorldGumballTransformPreview | null): void {
  const previous = worldGumballTransformPreviewByController.get(controllerId) ?? null;
  if (worldGumballTransformPreviewsEqual(previous, preview)) return;
  if (preview) worldGumballTransformPreviewByController.set(controllerId, preview);
  else worldGumballTransformPreviewByController.delete(controllerId);
  notifyWorldGumballTransformPreviewListeners();
}

/** 🧲️ Clears a preview only when `sourceId` still owns it, preventing an idle sibling pane from cancelling the active pane's gesture. */
export function clearWorldGumballTransformPreview(controllerId: string, sourceId?: string): void {
  const current = worldGumballTransformPreviewByController.get(controllerId);
  if (!current || (sourceId && current.sourceId !== sourceId)) return;
  setWorldGumballTransformPreview(controllerId, null);
}

/** 🌀 Maps committed vortex markers through an active gumball preview when they belong to a transformed instance — each about
 * its OWN instance's drag-start pose (`owners`, see {@link gumballPreviewOwnedWorldPoint}), so a multi-object turn or scale
 * previews every marker where the release lands it. */
export function worldVorticesWithGumballPreview(
  vortices: readonly WorldVortexRecord[],
  preview: WorldGumballTransformPreview | null,
  owners: ReadonlyMap<string, WorldGumballOwnerPose>,
): readonly WorldVortexRecord[] {
  if (!preview || preview.instanceIds.length === 0) return vortices;
  const selected = new Set(preview.instanceIds);
  return vortices.map((vortex) => {
    const objectId = vortex.objectId ?? vortex.fullId.split(":")[0];
    const owner = objectId ? owners.get(objectId) : undefined;
    if (!objectId || !owner || !selected.has(objectId)) return vortex;
    const position = gumballPreviewOwnedWorldPoint(owner, preview.transformMode, preview.before, preview.after, preview.handleKind, vortex.position);
    const direction = vortex.direction
      ? gumballPreviewOwnedWorldDirection(owner, preview.transformMode, preview.before, preview.after, preview.handleKind, vortex.direction)
      : vortex.direction;
    return direction === vortex.direction && position.every((value, index) => Math.abs(value - vortex.position[index]!) < GUMBALL_TRANSFORM_EPSILON)
      ? vortex
      : { ...vortex, position, direction };
  });
}

/** 🌀 Maps committed attraction endpoints through an active gumball preview when they touch a transformed instance's vortices,
 * each about its own instance's drag-start pose (`owners`) like {@link worldVorticesWithGumballPreview}. */
export function worldAttractionsWithGumballPreview(
  attractions: readonly WorldAttractionRecord[],
  vortices: readonly WorldVortexRecord[],
  preview: WorldGumballTransformPreview | null,
  owners: ReadonlyMap<string, WorldGumballOwnerPose>,
): readonly WorldAttractionRecord[] {
  if (!preview || preview.instanceIds.length === 0) return attractions;
  const selected = new Set(preview.instanceIds);
  const endpointMap = new Map<string, readonly [number, number, number]>();
  for (const vortex of vortices) {
    const objectId = vortex.objectId ?? vortex.fullId.split(":")[0];
    const owner = objectId ? owners.get(objectId) : undefined;
    if (!objectId || !owner || !selected.has(objectId)) continue;
    const key = vortex.position.join(",");
    endpointMap.set(key, gumballPreviewOwnedWorldPoint(owner, preview.transformMode, preview.before, preview.after, preview.handleKind, vortex.position));
  }
  const remap = (point: readonly [number, number, number]) => endpointMap.get(point.join(",")) ?? point;
  return attractions.map((attraction) => {
    const from = remap(attraction.from);
    const to = remap(attraction.to);
    return from === attraction.from && to === attraction.to ? attraction : { ...attraction, from, to };
  });
}
//#endregion WorldGumballTransformPreviewStore
//#endregion 🔖️World3dHost
