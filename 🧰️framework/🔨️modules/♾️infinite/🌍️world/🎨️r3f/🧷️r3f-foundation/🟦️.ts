/** 🧷️ Shared r3f foundations: vector types, camera state, navigation gesture classification, entity flags and render modes, the scene event binding controller, CAD to three precision helpers, mesh borders and the raycast-none hook.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | r3f}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { clearColorResolveCache, resolveColorHex, semanticVar } from "@semio-tech/ui-styling";
import {
  BufferGeometry,
  Color,
  EdgesGeometry,
  LineBasicMaterial,
  LineSegments,
  Matrix4,
  Mesh,
  Object3D,
  Quaternion,
  Vector3
} from "../🔌️adapters/🟦️.ts";
import { type WorldProjectionSpec } from "../🔭️scene-types/🟦️.ts";

// #region 🔖️Types
export type Vec3 = readonly [number, number, number];
export type Quat = readonly [number, number, number, number];

export interface LodMeshEntry {
  readonly lod: number;
  readonly url: string;
}

/** 📐️ Orbit view projection mode for display templates. */
export type OrbitCameraProjection = "orthographic" | "perspective";

export interface WorldCameraState {
  readonly position: Vec3;
  readonly target: Vec3;
  readonly zoom: number;
  readonly up?: Vec3;
  readonly projection?: OrbitCameraProjection;
  /** 📐️ Full classical-projection taxonomy spec (Parallel/Perspective families); see {@link WorldProjectionSpec}. */
  readonly projectionSpec?: WorldProjectionSpec;
}

/** 🧭️ A camera-navigation gesture {@link classifyWorldNavigationGestures} can detect on an orbit control. */
export type WorldNavigationGesture = "pan" | "zoom" | "orbit";

/** 📸️ The subset of camera state {@link classifyWorldNavigationGestures} diffs before/after a gesture. */
export interface WorldNavigationSnapshot {
  readonly position: Vec3;
  readonly target: Vec3;
  readonly zoom: number;
  readonly projection: OrbitCameraProjection;
}

/** 📏️ Ratios/angle above which a camera-state delta counts as that gesture, tunable per caller. */
export interface WorldNavigationThresholds {
  readonly panRatio: number;
  readonly zoomRatio: number;
  readonly orbitRadians: number;
}

const WORLD_NAVIGATION_THRESHOLDS_DEFAULT: WorldNavigationThresholds = { panRatio: 0.02, zoomRatio: 0.03, orbitRadians: 0.05 };

function worldNavigationVec3Distance(a: Vec3, b: Vec3): number {
  return Math.hypot(a[0] - b[0], a[1] - b[1], a[2] - b[2]);
}

function worldNavigationVec3Direction(from: Vec3, to: Vec3): Vec3 {
  const dx = to[0] - from[0];
  const dy = to[1] - from[1];
  const dz = to[2] - from[2];
  const length = Math.hypot(dx, dy, dz);
  return length > 1e-9 ? [dx / length, dy / length, dz / length] : [0, 0, 0];
}

/** 🧭️ Classifies which navigation gestures a camera movement performed by diffing before/after
 * snapshots — pan (the orbit target moved), zoom (orthographic zoom factor or perspective dolly distance
 * changed), orbit (the camera's direction around the target rotated). A single drag may perform more than
 * one (e.g. a drag that pans while also slightly orbiting), so this returns a set, not one verdict. */
export function classifyWorldNavigationGestures(
  before: WorldNavigationSnapshot,
  after: WorldNavigationSnapshot,
  thresholds: Partial<WorldNavigationThresholds> = {},
): readonly WorldNavigationGesture[] {
  const { panRatio, zoomRatio, orbitRadians } = { ...WORLD_NAVIGATION_THRESHOLDS_DEFAULT, ...thresholds };
  const gestures: WorldNavigationGesture[] = [];
  const referenceDistance = Math.max(worldNavigationVec3Distance(before.position, before.target), 1);

  if (worldNavigationVec3Distance(before.target, after.target) > panRatio * referenceDistance) gestures.push("pan");

  const zoomDelta =
    before.projection === "orthographic"
      ? Math.abs(after.zoom / (before.zoom || 1) - 1)
      : Math.abs(worldNavigationVec3Distance(after.position, after.target) / referenceDistance - 1);
  if (zoomDelta > zoomRatio) gestures.push("zoom");

  const beforeDirection = worldNavigationVec3Direction(before.target, before.position);
  const afterDirection = worldNavigationVec3Direction(after.target, after.position);
  const dot = Math.min(1, Math.max(-1, beforeDirection[0] * afterDirection[0] + beforeDirection[1] * afterDirection[1] + beforeDirection[2] * afterDirection[2]));
  if (Math.acos(dot) > orbitRadians) gestures.push("orbit");

  return gestures;
}

export type SceneListenerTarget = Pick<EventTarget, "addEventListener" | "removeEventListener">;

/** 👁️ Persisted per-entity hide/lock flags shared by CAD and puzzle 3d. */
export interface WorldEntityFlags {
  readonly hidden?: boolean;
  readonly locked?: boolean;
}

/** 🔒️ Opacity scale applied when an entity is locked but still rendered. */
export const WORLD_LOCKED_OPACITY_SCALE = 0.35;

/** 🔒️ Desaturation factor applied when an entity is locked but still rendered. */
export const WORLD_LOCKED_DESATURATION = 0.55;

export interface WorldEntityRenderModeInput {
  readonly hovered?: boolean;
  readonly selected?: boolean;
  readonly revealed?: boolean;
}

export interface WorldEntityRenderMode {
  readonly visible: boolean;
  readonly asHover: boolean;
  readonly dim: boolean;
  readonly showSelectedOutline: boolean;
}

/** 👁️ Whether an entity participates in canvas pick, hover, and edit interactions.
 * Locked entities must not absorb raycasts — a click is equivalent to clicking the background
 * (deselect / pass-through to whatever is behind). */
export function worldEntitySelectable(flags: WorldEntityFlags | undefined): boolean {
  return flags?.hidden !== true && flags?.locked !== true;
}

/** 🔎️ Whether an entity can appear in details/tree selection while locked.
 * Canvas pointer picking still uses {@link worldEntitySelectable} so locked clicks deselect like background. */
export function worldEntityInspectable(flags: WorldEntityFlags | undefined): boolean {
  return flags?.hidden !== true;
}

/** 👁️ Whether an entity should be drawn in the 3d scene. */
export function worldEntityRendered(flags: WorldEntityFlags | undefined, revealed = false): boolean {
  return flags?.hidden !== true || revealed;
}

/** 👁️ Resolves hover/dim/outline behavior for hidden-reveal and locked entities. */
export function worldEntityRenderMode(flags: WorldEntityFlags | undefined, input: WorldEntityRenderModeInput = {}): WorldEntityRenderMode {
  const revealed = input.revealed === true;
  const hovered = input.hovered === true;
  const selected = input.selected === true;
  const hidden = flags?.hidden === true;
  const locked = flags?.locked === true;
  const visible = worldEntityRendered(flags, revealed);
  const asHover = (hidden && revealed) || (hovered && !locked);
  const dim = locked && !hidden;
  const showSelectedOutline = selected && !locked && !hidden;
  return { visible, asHover, dim, showSelectedOutline };
}
// #endregion 🔖️Types

// #region 🔖️EventBinding
/** 🎧️ Tracks DOM listeners for deterministic teardown on world unmount. */
export class WorldEventBindingController {
  private readonly cleanups: Array<() => void> = [];

  listen(target: SceneListenerTarget | null | undefined, kind: string, listener: EventListenerOrEventListenerObject, options?: boolean | AddEventListenerOptions): void {
    if (!target) return;
    target.addEventListener(kind, listener, options);
    this.cleanups.push(() => target.removeEventListener(kind, listener, options));
  }

  dispose(): void {
    while (this.cleanups.length > 0) {
      this.cleanups.pop()?.();
    }
  }
}
// #endregion 🔖️EventBinding

// #region 🧭️Precision
const _mCadToThree = new Matrix4();

/** 🧭️ Identity matrix: CAD Z-up and Three.js scene share the same frame. */
export function cadToThreeMatrix(): Matrix4 {
  return _mCadToThree.identity();
}

/** 🧭️ Maps a CAD fixture point to Three.js world coordinates (identity in z-up scenes). */
export function cadVec3ToThree(v: Vec3): Vec3 {
  return [v[0], v[1], v[2]];
}

/** 🧭️ Maps a Three.js point back to CAD fixture coordinates (identity in z-up scenes). */
export function threeVec3ToCad(v: { readonly x: number; readonly y: number; readonly z: number }): Vec3 {
  return [v.x, v.y, v.z];
}

/** 🧭️ Maps a CAD fixture quaternion to Three.js (identity in z-up scenes). */
export function cadQuatToThree(q: Quat): Quat {
  return [q[0], q[1], q[2], q[3]];
}

/** 🧭️ Maps a Three.js quaternion back to CAD fixture coordinates (identity in z-up scenes). */
export function threeQuatToCad(q: Quaternion): Quat {
  return [q.x, q.y, q.z, q.w];
}

function quatRotateVec(q: Quat, v: Vec3): Vec3 {
  const out = new Vector3(v[0], v[1], v[2]).applyQuaternion(new Quaternion(q[0], q[1], q[2], q[3]));
  return [out.x, out.y, out.z];
}

function vec3Add(a: Vec3, b: Vec3): Vec3 {
  return [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
}

/** 🧭️ Maps object-local CAD offset to Three.js parent-group coordinates. */
export function cadObjectLocalToThreeGroupLocal(local: Vec3, originCad: Vec3, orientationCad: Quat | undefined): Vec3 {
  const q = orientationCad ?? ([0, 0, 0, 1] as Quat);
  const worldCad = vec3Add(originCad, quatRotateVec(q, local));
  const worldThree = new Vector3(...cadVec3ToThree(worldCad));
  const originThree = new Vector3(...cadVec3ToThree(originCad));
  const qThree = new Quaternion(...cadQuatToThree(q));
  const out = worldThree.sub(originThree).applyQuaternion(qThree.invert());
  return [out.x, out.y, out.z];
}

/** 🧭️ Maps object-local CAD direction to a unit vector in the parent Three.js group. */
export function cadObjectLocalDirectionToThreeGroupLocal(localDir: Vec3, originCad: Vec3, orientationCad: Quat | undefined): Vec3 {
  const tip = cadObjectLocalToThreeGroupLocal(localDir, originCad, orientationCad);
  const base = cadObjectLocalToThreeGroupLocal([0, 0, 0], originCad, orientationCad);
  const d = new Vector3(tip[0] - base[0], tip[1] - base[1], tip[2] - base[2]);
  const len = d.length();
  if (len < 1e-9) {
    return [0, 0, 1];
  }
  d.divideScalar(len);
  return [d.x, d.y, d.z];
}

/** 🧭️ +90° X: glTF Y-up mesh → CAD object-local Z-up inside the pose group. */
export const GLB_MESH_FRAME_ROTATION_X = Math.PI / 2;

/** 🎯️ Rebases world positions near the camera to reduce floating-point jitter at large coordinates. */
export function floatingOriginRebase(worldCad: Vec3, anchorCad: Vec3): Vec3 {
  return [worldCad[0] - anchorCad[0], worldCad[1] - anchorCad[1], worldCad[2] - anchorCad[2]];
}
// #endregion 🧭️Precision

// #region 🎨️MeshBorder
/** 📏️ UI normal border token for infinite-world mesh edge strokes ({@link borderNormalClass}). */
export const WORLD_MESH_BORDER_CSS = semanticVar("border-normal-color");

/** 📏️ Marks {@link LineSegments} added by {@link applyWorldMeshEdgeBorders}. */
export const WORLD_MESH_OUTLINE_USER_DATA_KEY = "__worldMeshBorderOutline";

let worldMeshBorderColorCache: string | null = null;

/** 📏️ Resolves {@link WORLD_MESH_BORDER_CSS} to an sRGB color string for Three.js materials. */
export function worldMeshBorderColor(): string {
  if (worldMeshBorderColorCache) {
    return worldMeshBorderColorCache;
  }
  worldMeshBorderColorCache = resolveColorHex(WORLD_MESH_BORDER_CSS, "gray");
  return worldMeshBorderColorCache;
}

/** 🔄️ Clears cached mesh border color (tests or theme switches). */
export function resetWorldMeshBorderColorCache(): void {
  worldMeshBorderColorCache = null;
  clearColorResolveCache();
}

/** 📏️ Edge-segment outline for one mesh geometry using the UI normal border color. */
export function createWorldMeshEdgeOutline(geometry: BufferGeometry, borderColor?: string): LineSegments {
  const color = borderColor ?? worldMeshBorderColor();
  const outline = new LineSegments(new EdgesGeometry(geometry), new LineBasicMaterial({ color: new Color(resolveColorHex(color, "gray")) }));
  outline.userData[WORLD_MESH_OUTLINE_USER_DATA_KEY] = true;
  outline.scale.setScalar(1.001);
  return outline;
}

/** 📏️ Adds normal-border edge outlines to every mesh under `root` (idempotent per mesh). */
export function applyWorldMeshEdgeBorders(root: Object3D, borderColor?: string): void {
  const color = borderColor ?? worldMeshBorderColor();
  root.traverse((object) => {
    if (!(object instanceof Mesh)) {
      return;
    }
    const geometry = object.geometry;
    if (!geometry || object.children.some((child) => child.userData[WORLD_MESH_OUTLINE_USER_DATA_KEY])) {
      return;
    }
    object.add(createWorldMeshEdgeOutline(geometry, color));
  });
}
// #endregion 🎨️MeshBorder

// #region 📶️Lod
export const worldRaycastNone: Object3D["raycast"] = () => undefined;
// #endregion 📶️Lod
