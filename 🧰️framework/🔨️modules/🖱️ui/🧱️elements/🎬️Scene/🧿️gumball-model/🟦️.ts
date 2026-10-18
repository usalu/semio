import * as THREE from "three";

/** 🎛️ World-space pose snapshot for gumball drag commits. */
export type GumballPose = {
  readonly position: readonly [number, number, number];
  readonly quaternion: readonly [number, number, number, number];
  readonly scale: readonly [number, number, number];
};

/** 🎛️ Per-handle drag kinds for the unified gumball. */
export type GumballHandleKind = "moveX" | "moveY" | "moveZ" | "moveXY" | "moveYZ" | "moveXZ" | "rotateX" | "rotateY" | "rotateZ" | "scaleX" | "scaleY" | "scaleZ" | "scaleXY" | "scaleYZ" | "scaleXZ" | "scaleUniform";

/** 🎛️ Drafting plane that restricts the gumball to the in-plane handle subset (two axes, view plane, normal rotation). */
export type GumballPlaneId = "xy" | "yz" | "xz";

/** 🎛️ Visibility and snap settings for {@link UnifiedGumball}. */
export interface GumballConfig {
  readonly moveAxes?: boolean;
  readonly movePlanes?: boolean;
  readonly rotate?: boolean;
  readonly scaleAxes?: boolean;
  readonly scalePlanes?: boolean;
  readonly scaleUniform?: boolean;
  /** When set, only the planar subset for this drafting plane is shown (e.g. Top → `xy`). */
  readonly plane?: GumballPlaneId;
  readonly translationSnap?: number;
  readonly rotationSnap?: number;
  readonly scaleSnap?: number;
  readonly shiftTranslationSnap?: number;
  readonly shiftRotationSnap?: number;
  readonly shiftScaleSnap?: number;
  readonly size?: number;
}

/** 🎛️ Handles that remain visible for each drafting plane (in-plane move/scale + normal-axis rotate + uniform). */
export const GUMBALL_PLANE_HANDLES: Readonly<Record<GumballPlaneId, ReadonlySet<GumballHandleKind>>> = {
  xy: new Set(["moveX", "moveY", "moveXY", "rotateZ", "scaleX", "scaleY", "scaleXY", "scaleUniform"]),
  yz: new Set(["moveY", "moveZ", "moveYZ", "rotateX", "scaleY", "scaleZ", "scaleYZ", "scaleUniform"]),
  xz: new Set(["moveX", "moveZ", "moveXZ", "rotateY", "scaleX", "scaleZ", "scaleXZ", "scaleUniform"]),
};

/** 🎛️ Default unified gumball: every handle group visible. */
export const DEFAULT_GUMBALL_CONFIG: Readonly<Required<Pick<GumballConfig, "moveAxes" | "movePlanes" | "rotate" | "scaleAxes" | "scalePlanes" | "scaleUniform">>> = {
  moveAxes: true,
  movePlanes: true,
  rotate: true,
  scaleAxes: true,
  scalePlanes: true,
  scaleUniform: true,
};

export type GumballVec3 = readonly [number, number, number];

export const gumballV3 = (x: number, y: number, z: number): GumballVec3 => [x, y, z];
const gumballDot = (a: GumballVec3, b: GumballVec3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
export const gumballSub = (a: GumballVec3, b: GumballVec3): GumballVec3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const gumballAdd = (a: GumballVec3, b: GumballVec3): GumballVec3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
const gumballScale = (a: GumballVec3, s: number): GumballVec3 => [a[0] * s, a[1] * s, a[2] * s];
const gumballCross = (a: GumballVec3, b: GumballVec3): GumballVec3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
export const gumballLen = (a: GumballVec3): number => Math.hypot(a[0], a[1], a[2]);
const gumballNorm = (a: GumballVec3): GumballVec3 => {
  const l = gumballLen(a) || 1;
  return [a[0] / l, a[1] / l, a[2] / l];
};

/** 🎛️ Resolves partial {@link GumballConfig} with defaults. */
export function resolveGumballConfig(config?: GumballConfig): Required<Pick<GumballConfig, "moveAxes" | "movePlanes" | "rotate" | "scaleAxes" | "scalePlanes" | "scaleUniform">> & GumballConfig {
  return {
    ...DEFAULT_GUMBALL_CONFIG,
    ...config,
    moveAxes: config?.moveAxes !== false,
    movePlanes: config?.movePlanes !== false,
    rotate: config?.rotate !== false,
    scaleAxes: config?.scaleAxes !== false,
    scalePlanes: config?.scalePlanes !== false,
    scaleUniform: config?.scaleUniform !== false,
    plane: config?.plane,
  };
}

/** 🎛️ True when a handle is allowed by an optional drafting-plane subset. */
export function gumballHandleAllowedByPlane(kind: GumballHandleKind, plane: GumballPlaneId | undefined): boolean {
  if (!plane) return true;
  return GUMBALL_PLANE_HANDLES[plane].has(kind);
}

/** 🎛️ True when a handle's visibility group is enabled in the resolved config. */
export function gumballHandleGroupEnabled(kind: GumballHandleKind, config: ReturnType<typeof resolveGumballConfig>): boolean {
  if (kind === "moveX" || kind === "moveY" || kind === "moveZ") return config.moveAxes;
  if (kind === "moveXY" || kind === "moveYZ" || kind === "moveXZ") return config.movePlanes;
  if (kind === "rotateX" || kind === "rotateY" || kind === "rotateZ") return config.rotate;
  if (kind === "scaleX" || kind === "scaleY" || kind === "scaleZ") return config.scaleAxes;
  if (kind === "scaleXY" || kind === "scaleYZ" || kind === "scaleXZ") return config.scalePlanes;
  return config.scaleUniform;
}

/** 🎛️ True when a handle should render for the resolved config (group flags ∩ plane subset). */
export function gumballHandleEnabled(kind: GumballHandleKind, config: ReturnType<typeof resolveGumballConfig>): boolean {
  return gumballHandleGroupEnabled(kind, config) && gumballHandleAllowedByPlane(kind, config.plane);
}

/** 🎛️ True when at least one gumball handle group is enabled. */
export function gumballConfigVisible(config?: GumballConfig): boolean {
  const resolved = resolveGumballConfig(config);
  if (!(resolved.moveAxes || resolved.movePlanes || resolved.rotate || resolved.scaleAxes || resolved.scalePlanes || resolved.scaleUniform)) return false;
  if (!resolved.plane) return true;
  const kinds: readonly GumballHandleKind[] = ["moveX", "moveY", "moveZ", "moveXY", "moveYZ", "moveXZ", "rotateX", "rotateY", "rotateZ", "scaleX", "scaleY", "scaleZ", "scaleXY", "scaleYZ", "scaleXZ", "scaleUniform"];
  return kinds.some((kind) => gumballHandleEnabled(kind, resolved));
}

/** 🎛️ Maps a handle drag to translate / rotate / scale for host commit payloads. */
export function gumballHandleKindToTransformMode(kind: GumballHandleKind): "translate" | "rotate" | "scale" {
  if (kind.startsWith("move")) return "translate";
  if (kind.startsWith("rotate")) return "rotate";
  return "scale";
}

/** 🎛️ Reads pose from a three.js object. */
export function gumballPoseFromObject3D(object: THREE.Object3D): GumballPose {
  return {
    position: [object.position.x, object.position.y, object.position.z],
    quaternion: [object.quaternion.x, object.quaternion.y, object.quaternion.z, object.quaternion.w],
    scale: [object.scale.x, object.scale.y, object.scale.z],
  };
}

/** 🎛️ Writes pose to a three.js object. */
export function applyGumballPose(object: THREE.Object3D, pose: GumballPose): void {
  object.position.set(pose.position[0], pose.position[1], pose.position[2]);
  object.quaternion.set(pose.quaternion[0], pose.quaternion[1], pose.quaternion[2], pose.quaternion[3]);
  object.scale.set(pose.scale[0], pose.scale[1], pose.scale[2]);
}

/** 🎛️ Closest-axis parameter for a world ray (axis translate drag). */
export function gumballRayAxisParameter(rayOrigin: GumballVec3, rayDir: GumballVec3, axisPoint: GumballVec3, axisDir: GumballVec3): number {
  const w0 = gumballSub(rayOrigin, axisPoint);
  const a = gumballDot(rayDir, rayDir);
  const b = gumballDot(rayDir, axisDir);
  const c = gumballDot(axisDir, axisDir);
  const d = gumballDot(rayDir, w0);
  const e = gumballDot(axisDir, w0);
  const denom = a * c - b * b;
  if (Math.abs(denom) < 1e-10) return 0;
  return (b * d - a * e) / denom;
}

/** 👁️ Unit vector from a pivot toward the active camera (TransformControls-style eye). */
export function gumballEyeFromPivot(camera: THREE.Camera, pivot: GumballVec3): GumballVec3 {
  const cam = new THREE.Vector3();
  camera.getWorldPosition(cam);
  return gumballNorm(gumballSub([cam.x, cam.y, cam.z], pivot));
}

/** 🎛️ Drag plane normal for constrained axis move/scale (matches three.js TransformControls). */
export function gumballAxisDragPlaneNormal(axisDir: GumballVec3, eye: GumballVec3): GumballVec3 {
  const axis = gumballNorm(axisDir);
  const align = gumballCross(eye, axis);
  if (gumballLen(align) > 1e-6) {
    return gumballNorm(gumballCross(axis, align));
  }
  return eye;
}

/** 🎛️ Projects a screen ray onto an axis using a camera-aligned plane through the pivot. */
export function gumballProjectRayOntoAxis(rayOrigin: GumballVec3, rayDir: GumballVec3, pivot: GumballVec3, axisDir: GumballVec3, eye: GumballVec3): number | null {
  const planeNormal = gumballAxisDragPlaneNormal(axisDir, eye);
  const hit = gumballRayPlanePoint(rayOrigin, rayDir, pivot, planeNormal);
  if (!hit) return null;
  return gumballDot(gumballSub(hit, pivot), gumballNorm(axisDir));
}

/** 🎛️ Ray-plane intersection for plane translate drags. */
export function gumballRayPlanePoint(rayOrigin: GumballVec3, rayDir: GumballVec3, planePoint: GumballVec3, planeNormal: GumballVec3): GumballVec3 | null {
  const denom = gumballDot(planeNormal, rayDir);
  if (Math.abs(denom) < 1e-10) return null;
  const t = gumballDot(planeNormal, gumballSub(planePoint, rayOrigin)) / denom;
  return gumballAdd(rayOrigin, gumballScale(rayDir, t));
}

/** 🎛️ Signed rotation angle between two vectors around an axis. */
export function gumballAxisRotateAngle(startVec: GumballVec3, currentVec: GumballVec3, axisDir: GumballVec3): number {
  const axis = gumballNorm(axisDir);
  const project = (v: GumballVec3): GumballVec3 => gumballSub(v, gumballScale(axis, gumballDot(v, axis)));
  const a = gumballNorm(project(startVec));
  const b = gumballNorm(project(currentVec));
  let angle = Math.acos(Math.max(-1, Math.min(1, gumballDot(a, b))));
  if (gumballDot(gumballCross(a, b), axis) < 0) angle = -angle;
  return angle;
}

/** 🎛️ Scale factor from projected distances along an axis. */
export function gumballAxisScaleFactor(startProj: number, currentProj: number): number {
  if (Math.abs(startProj) < 1e-10) return 1;
  return currentProj / startProj;
}

/** 🎛️ Snaps a scalar when snap step is positive. */
export function gumballSnapScalar(value: number, snap: number | undefined): number {
  if (!snap || snap <= 0) return value;
  return Math.round(value / snap) * snap;
}

/** 🎛️ Default rotation snap while Shift is held (15°). */
export const GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP = Math.PI / 12;

/** 🎛️ Default uniform scale snap while Shift is held (10%). */
export const GUMBALL_DEFAULT_SHIFT_SCALE_SNAP = 0.1;

/** 🎛️ Resolves an active snap step from config and Shift modifier. */
export function gumballEffectiveSnapValue(configSnap: number | undefined, shiftKey: boolean, shiftFallback: number): number | undefined {
  if (configSnap != null && configSnap > 0) return configSnap;
  if (shiftKey && shiftFallback > 0) return shiftFallback;
  return undefined;
}

/** 🎛️ Active gumball drag snap steps for translate / rotate / scale. */
export function gumballResolveDragSnaps(config: GumballConfig, shiftKey: boolean): { readonly translationSnap: number | undefined; readonly rotationSnap: number | undefined; readonly scaleSnap: number | undefined } {
  return {
    translationSnap: gumballEffectiveSnapValue(config.translationSnap, shiftKey, config.shiftTranslationSnap ?? 0),
    rotationSnap: gumballEffectiveSnapValue(config.rotationSnap, shiftKey, config.shiftRotationSnap ?? GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP),
    scaleSnap: gumballEffectiveSnapValue(config.scaleSnap, shiftKey, config.shiftScaleSnap ?? GUMBALL_DEFAULT_SHIFT_SCALE_SNAP),
  };
}
