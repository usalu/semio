/** 📷️ Orbit camera: perspective or orthographic, fit-to-bounds, the architectural presets, view/projection matrices (column-major, OpenGL clip space) and picking rays.
 *
 * Frame: right-handed, +Y up, +Z ahead of the asset, +X the asset's left. The camera sits at `target + distance * (cos(pitch) sin(yaw), sin(pitch), cos(pitch) cos(yaw))`; `yaw 0, pitch 0` looks at the asset's front.
 * The basis is analytic (`right = (cos yaw, 0, -sin yaw)`), so the plan view (`pitch = pi/2`) is not singular.
 *
 * Presets: `front` from +Z; `elevation` from the asset's right (-X), so "ahead" runs to the right of the picture; `plan` from above with "ahead" to the right and the asset's left at the top; `axonometric` the isometric corner (yaw -45 deg, pitch 35.264 deg, orthographic);
 * `perspective` a three-quarter perspective view.
 * @see ../🧬️contract/🟦️.ts */
import { cross, dot, normalize, type V3 } from "../../🧮️math/🟦️.ts";
import type { Bounds } from "../🧬️contract/🟦️.ts";
import * as portable from "../../🧮️math/🎯️portable/🟦️.ts";

/** 🔲️ Projection kind. */
export type Projection = "perspective" | "orthographic";
/** 🏛️ Named view directions. */
export type CameraPreset = "elevation" | "front" | "plan" | "axonometric" | "perspective";
/** 🎥️ Orbit camera state; `orthoHeight` is the visible world height of an orthographic view, `extent` the model radius used for the clip planes. */
export type CameraState = { projection: Projection; target: V3; yaw: number; pitch: number; distance: number; fovY: number; orthoHeight: number; extent: number };
/** 🧭️ A ray in world space; `direction` is normalised. */
export type Ray = { origin: V3; direction: V3 };

const HALF_PI = Math.PI / 2;
const ISO_PITCH = portable.atan(1 / Math.SQRT2);

/** 🗺️ Yaw, pitch and projection of every preset. */
export const PRESETS: Readonly<Record<CameraPreset, { yaw: number; pitch: number; projection: Projection }>> = {
  front: { yaw: 0, pitch: 0, projection: "orthographic" },
  elevation: { yaw: -HALF_PI, pitch: 0, projection: "orthographic" },
  plan: { yaw: -HALF_PI, pitch: HALF_PI, projection: "orthographic" },
  axonometric: { yaw: -Math.PI / 4, pitch: ISO_PITCH, projection: "orthographic" },
  perspective: { yaw: Math.PI / 6, pitch: 0.35, projection: "perspective" },
};

/** 🔭️ A camera in `preset` looking at the unit box; call {@link fitCamera} to frame real bounds. */
export const cameraOf = (preset: CameraPreset = "perspective"): CameraState => ({ ...PRESETS[preset], target: [0, 0, 0], distance: 4, fovY: 0.6, orthoHeight: 2, extent: 1 });

/** 🔀️ The same camera turned into `preset`, keeping target, distance and extent. */
export const withPreset = (camera: CameraState, preset: CameraPreset): CameraState => ({ ...camera, ...PRESETS[preset] });

/** 🧲️ Camera basis and eye position in world space. */
export function basisOf(camera: CameraState): { right: V3; up: V3; forward: V3; eye: V3 } {
  const cp = portable.cos(camera.pitch), sp = portable.sin(camera.pitch), sy = portable.sin(camera.yaw), cy = portable.cos(camera.yaw);
  const back: V3 = [cp * sy, sp, cp * cy];
  const forward: V3 = [-back[0], -back[1], -back[2]];
  const right: V3 = [cy, 0, -sy];
  const up = cross(right, forward);
  const t = camera.target;
  return { right, up, forward, eye: [t[0] + back[0] * camera.distance, t[1] + back[1] * camera.distance, t[2] + back[2] * camera.distance] };
}

const corners = (b: Bounds): V3[] => {
  const out: V3[] = [];
  for (let i = 0; i < 8; i++) out.push([i & 1 ? b.max[0] : b.min[0], i & 2 ? b.max[1] : b.min[1], i & 4 ? b.max[2] : b.min[2]]);
  return out;
};

/** 🎯️ Frames `bounds` completely (with `margin` > 1 of free space) for `aspect` = width / height, keeping the view direction and projection. Empty or invalid bounds fall back to the unit box. */
export function fitCamera(camera: CameraState, bounds: Bounds, aspect: number, margin = 1.1): CameraState {
  const valid = bounds.min.every((v, i) => Number.isFinite(v) && Number.isFinite(bounds.max[i]!) && bounds.max[i]! >= v);
  const b: Bounds = valid ? bounds : { min: [-0.5, -0.5, -0.5], max: [0.5, 0.5, 0.5] };
  const target: V3 = [(b.min[0] + b.max[0]) / 2, (b.min[1] + b.max[1]) / 2, (b.min[2] + b.max[2]) / 2];
  const extent = Math.max(portable.hypot(b.max[0] - b.min[0], b.max[1] - b.min[1], b.max[2] - b.min[2]) / 2, 1e-6);
  const { right, up, forward } = basisOf(camera);
  const a = Math.max(aspect, 1e-6);
  let halfX = 0, halfY = 0, need = 0;
  const tanY = portable.tan(camera.fovY / 2) / margin, tanX = tanY * a;
  for (const c of corners(b)) {
    const rel = [c[0] - target[0], c[1] - target[1], c[2] - target[2]];
    const x = Math.abs(dot(rel, right)), y = Math.abs(dot(rel, up)), z = dot(rel, forward);
    halfX = Math.max(halfX, x);
    halfY = Math.max(halfY, y);
    need = Math.max(need, x / tanX - z, y / tanY - z);
  }
  if (camera.projection === "orthographic") return { ...camera, target, extent, distance: extent * 2 + 1e-6, orthoHeight: 2 * Math.max(halfY, halfX / a, 1e-6) * margin };
  return { ...camera, target, extent, distance: Math.max(need, extent * 0.05) };
}

/** 🔄️ Orbits by yaw and pitch deltas (rad); pitch is clamped to [-pi/2, pi/2]. */
export const orbit = (camera: CameraState, dYaw: number, dPitch: number): CameraState => ({ ...camera, yaw: camera.yaw + dYaw, pitch: Math.min(HALF_PI, Math.max(-HALF_PI, camera.pitch + dPitch)) });

/** ✋️ Pans the target in the picture plane by fractions of the visible height (`dx` right, `dy` up). */
export function pan(camera: CameraState, dx: number, dy: number, aspect: number): CameraState {
  const { right, up } = basisOf(camera);
  const h = visibleHeight(camera);
  const sx = dx * h * aspect, sy = dy * h;
  return { ...camera, target: [camera.target[0] + right[0] * sx + up[0] * sy, camera.target[1] + right[1] * sx + up[1] * sy, camera.target[2] + right[2] * sx + up[2] * sy] };
}

/** 🔎️ Zooms by a factor above 1 to go closer (perspective: distance, orthographic: visible height). */
export const zoom = (camera: CameraState, factor: number): CameraState =>
  camera.projection === "orthographic" ? { ...camera, orthoHeight: camera.orthoHeight / factor } : { ...camera, distance: camera.distance / factor };

/** 📏️ World height visible at the target plane. */
export const visibleHeight = (camera: CameraState): number => (camera.projection === "orthographic" ? camera.orthoHeight : 2 * camera.distance * portable.tan(camera.fovY / 2));

/** ✂️ Near and far clip distances along the view direction. */
export function clipPlanes(camera: CameraState): { near: number; far: number } {
  const far = camera.distance + camera.extent * 2;
  const near = camera.distance - camera.extent * 2;
  return camera.projection === "orthographic" ? { near, far } : { near: Math.max(near, camera.extent * 0.01, 1e-4), far };
}

/** 🧮️ Writes the view matrix. */
export function viewMatrix(camera: CameraState, out: number[] | Float32Array = new Array<number>(16)): number[] | Float32Array {
  const { right, up, forward, eye } = basisOf(camera);
  const m = out;
  m[0] = right[0]; m[1] = up[0]; m[2] = -forward[0]; m[3] = 0;
  m[4] = right[1]; m[5] = up[1]; m[6] = -forward[1]; m[7] = 0;
  m[8] = right[2]; m[9] = up[2]; m[10] = -forward[2]; m[11] = 0;
  m[12] = -dot(right, eye); m[13] = -dot(up, eye); m[14] = dot(forward, eye); m[15] = 1;
  return out;
}

/** 📽️ Writes the projection matrix for `aspect` = width / height. */
export function projectionMatrix(camera: CameraState, aspect: number, out: number[] | Float32Array = new Array<number>(16)): number[] | Float32Array {
  const { near, far } = clipPlanes(camera);
  const m = out;
  for (let i = 0; i < 16; i++) m[i] = 0;
  if (camera.projection === "orthographic") {
    const halfH = camera.orthoHeight / 2, halfW = halfH * aspect;
    m[0] = 1 / halfW; m[5] = 1 / halfH; m[10] = -2 / (far - near); m[14] = -(far + near) / (far - near); m[15] = 1;
  } else {
    const f = 1 / portable.tan(camera.fovY / 2);
    m[0] = f / aspect; m[5] = f; m[10] = (far + near) / (near - far); m[11] = -1; m[14] = (2 * far * near) / (near - far);
  }
  return out;
}

/** 🏹️ The world ray through normalised device coordinates (`x` right, `y` up, both in [-1, 1]). */
export function rayOf(camera: CameraState, aspect: number, ndcX: number, ndcY: number): Ray {
  const { right, up, forward, eye } = basisOf(camera);
  if (camera.projection === "orthographic") {
    const halfH = camera.orthoHeight / 2, halfW = halfH * aspect;
    const o: V3 = [eye[0] + right[0] * ndcX * halfW + up[0] * ndcY * halfH, eye[1] + right[1] * ndcX * halfW + up[1] * ndcY * halfH, eye[2] + right[2] * ndcX * halfW + up[2] * ndcY * halfH];
    return { origin: o, direction: forward };
  }
  const t = portable.tan(camera.fovY / 2);
  const d = normalize([forward[0] + right[0] * ndcX * t * aspect + up[0] * ndcY * t, forward[1] + right[1] * ndcX * t * aspect + up[1] * ndcY * t, forward[2] + right[2] * ndcX * t * aspect + up[2] * ndcY * t]);
  return { origin: eye, direction: d };
}
