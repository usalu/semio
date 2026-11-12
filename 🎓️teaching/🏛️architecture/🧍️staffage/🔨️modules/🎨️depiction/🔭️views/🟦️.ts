/** 🔭️ The three orthographic cameras of every asset and the error of a view that has no drawing: the one place the projection convention lives in code.
 *
 * World frame (asset-local and scene frame are the same): metres, x ahead, y up, z lateral with the left of the asset at +z, ground at y = 0. With x ahead and y up and z to the left the triple is left-handed.
 * Each camera maps a world point to paper-up coordinates `(x, y)`, y up (the depiction flips y to paper millimetres, y down):
 *
 * | viewpoint | seen from | looking along | paper x | paper up | nearer to the viewer |
 * |---|---|---|---|---|---|
 * | `elevation` (side, the rig plane) | the asset's right, −z | +z | x (ahead is right) | y | smaller z |
 * | `front` | ahead, +x | −x | z (the asset's left is paper right) | y | larger x |
 * | `plan` (top) | above, +y | −y | x | z (the asset's left is paper up) | larger y |
 *
 * The fourth camera is the orthographic axonometric one ({@link axonometricCamera}): the same family with the viewer at an azimuth and elevation, `right = (sin a, 0, cos a)`, `up = (−sin e cos a, cos e, sin e sin a)`, towards the viewer `(cos e cos a, sin e, −cos e sin a)` in the world frame (x ahead, y up, z left); `a = 90°, e = 0°` is the side elevation, `a = 0°, e = 0°` the front elevation, `e = 90°, a = 90°` the plan, so
 * the planar cameras are special cases and the axonometric depiction, the 2D projection helpers and the tests share this one source. The picture is scaled by `1 / cos e`, so the vertical axis is always true to scale (`mm = m · 1000 / denominator`); the horizontal axes keep the factor of their foreshortening: 0.8165 · 1.2247 = 1 for each of the three axes in the isometric projection, 1 and ½ in the ISO 5456-3 dimetric.
 *
 * A viewpoint changes only this basis; pose, time, root, scale, environment and attachments never depend on it.
 *
 * @see ../../🧮️evaluation/🟦️.ts — `projected`, the placement of front and plan shapes
 * @see ../🧊️depth/🟦️.ts — the depth convention (nearer is painted later)
 */
import type { PlanarViewpoint, Viewpoint } from "../../../🧬️schema/🟦️.ts";
import { ModelRequestError } from "../../🧊️volume/🚫️refusal/🟦️.ts";
import type { AxonometricKind, AxonometricProjection, AxonometricSpec, Point, Vec3 } from "../../🔗️contracts/🟦️.ts";
import * as portable from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

/** 🎥️ The orthographic camera of one viewpoint: which world axes become paper x and paper up (with sign), and the world direction the viewer looks along. */
export type Camera = { readonly viewpoint: PlanarViewpoint; readonly seenFrom: string; readonly paperX: { readonly axis: "x" | "y" | "z"; readonly sign: 1 | -1 }; readonly paperUp: { readonly axis: "x" | "y" | "z"; readonly sign: 1 | -1 }; readonly looking: Vec3 };

/** 🦄 The three cameras, keyed by viewpoint. */
export const CAMERAS: Readonly<Record<PlanarViewpoint, Camera>> = {
  elevation: { viewpoint: "elevation", seenFrom: "the right of the asset (-z)", paperX: { axis: "x", sign: 1 }, paperUp: { axis: "y", sign: 1 }, looking: { x: 0, y: 0, z: 1 } },
  front: { viewpoint: "front", seenFrom: "ahead of the asset (+x)", paperX: { axis: "z", sign: 1 }, paperUp: { axis: "y", sign: 1 }, looking: { x: -1, y: 0, z: 0 } },
  plan: { viewpoint: "plan", seenFrom: "above the asset (+y)", paperX: { axis: "x", sign: 1 }, paperUp: { axis: "z", sign: 1 }, looking: { x: 0, y: -1, z: 0 } },
};

/** 🐙 The orthographic camera of a planar viewpoint. */
export const cameraOf = (viewpoint: PlanarViewpoint): Camera => CAMERAS[viewpoint];

const RAD = Math.PI / 180;

/** 🍎 The elevation of the isometric corner view in degrees: `atan(1/√2)`, all three world axes at the same angle to the picture plane. */
export const ISOMETRIC_ELEVATION_DEG = portable.atan(1 / Math.SQRT2) / RAD;
/** 🍋 The azimuth of the isometric view in degrees. */
export const ISOMETRIC_AZIMUTH_DEG = 45;
/** 🍇 The elevation of the dimetric view in degrees: `asin(1/3)`, so that the axes of the 1 : ½ : 1 dimetric are `2√2/3, √2/3, 2√2/3` before the normalisation. */
export const DIMETRIC_ELEVATION_DEG = portable.asin(1 / 3) / RAD;
/** 🍓 The azimuth of the dimetric view in degrees: `90° − asin(1/(2√2))`, the ahead axis at 7.18° and the lateral axis at 41.41° to the horizon of the picture (ISO 5456-3). */
export const DIMETRIC_AZIMUTH_DEG = 90 - portable.asin(1 / (2 * Math.SQRT2)) / RAD;
/** 🦋 The largest elevation a free axonometric camera accepts: beyond it the picture degenerates towards the plan, which has its own viewpoint. */
export const AXONOMETRIC_MAX_ELEVATION_DEG = 75;

/** 🥝 The default axonometric camera. */
export const ISOMETRIC: AxonometricSpec = { kind: "isometric" };

/** 🐝 The orthographic axonometric camera: the unit axes of the picture and the viewing direction in the world frame (x ahead, y up, z left), the normalisation `1 / cos(elevation)` that makes the vertical axis true to scale and the paper scale of every world axis. */
export type AxonometricCamera = {
  readonly viewpoint: "axonometric";
  readonly kind: AxonometricKind;
  readonly azimuthDeg: number;
  readonly elevationDeg: number;
  readonly right: Vec3;
  readonly up: Vec3;
  readonly toward: Vec3;
  readonly looking: Vec3;
  readonly normalisation: number;
  readonly axisScale: { readonly x: number; readonly y: number; readonly z: number };
};

/** 🦉 The camera of an axonometric spec; throws the bilingual `ModelRequestError` (`camera-azimuth`, `camera-elevation`) for an azimuth outside −180°…180° or an elevation outside 0°…{@link AXONOMETRIC_MAX_ELEVATION_DEG}. */
export function axonometricCamera(spec: AxonometricSpec = ISOMETRIC): AxonometricCamera {
  const azimuthDeg = spec.kind === "isometric" ? ISOMETRIC_AZIMUTH_DEG : spec.kind === "dimetric" ? DIMETRIC_AZIMUTH_DEG : spec.azimuthDeg;
  const elevationDeg = spec.kind === "isometric" ? ISOMETRIC_ELEVATION_DEG : spec.kind === "dimetric" ? DIMETRIC_ELEVATION_DEG : spec.elevationDeg;
  if (!Number.isFinite(azimuthDeg) || azimuthDeg < -180 || azimuthDeg > 180) throw new ModelRequestError("camera-azimuth", String(azimuthDeg));
  if (!Number.isFinite(elevationDeg) || elevationDeg < 0 || elevationDeg > AXONOMETRIC_MAX_ELEVATION_DEG) throw new ModelRequestError("camera-elevation", String(elevationDeg));
  const a = azimuthDeg * RAD;
  const e = elevationDeg * RAD;
  const right: Vec3 = { x: portable.sin(a), y: 0, z: portable.cos(a) };
  const up: Vec3 = { x: -portable.sin(e) * portable.cos(a), y: portable.cos(e), z: portable.sin(e) * portable.sin(a) };
  const toward: Vec3 = { x: portable.cos(e) * portable.cos(a), y: portable.sin(e), z: -portable.cos(e) * portable.sin(a) };
  const normalisation = 1 / portable.cos(e);
  const along = (axis: Vec3): number => normalisation * portable.hypot(right.x * axis.x + right.y * axis.y + right.z * axis.z, up.x * axis.x + up.y * axis.y + up.z * axis.z);
  return { viewpoint: "axonometric", kind: spec.kind, azimuthDeg, elevationDeg, right, up, toward, looking: { x: -toward.x, y: -toward.y, z: -toward.z }, normalisation, axisScale: { x: along({ x: 1, y: 0, z: 0 }), y: along({ x: 0, y: 1, z: 0 }), z: along({ x: 0, y: 0, z: 1 }) } };
}

/** 🦊 What the axonometric camera is to scale, as the scene metadata states it. */
export const projectionOf = (camera: AxonometricCamera): AxonometricProjection => ({ kind: camera.kind, azimuthDeg: camera.azimuthDeg, elevationDeg: camera.elevationDeg, axisScale: camera.axisScale });

const dot = (a: Vec3, b: Vec3): number => a.x * b.x + a.y * b.y + a.z * b.z;

/** 🐢 The paper-up coordinates `(x, y)` in metres of a world point through an axonometric camera (the vertical axis true) and its depth towards the viewer in the same unit (larger is nearer). */
export function projectAxonometric(camera: AxonometricCamera, p: Vec3): { readonly x: number; readonly y: number; readonly depth: number } {
  const n = camera.normalisation;
  return { x: n * dot(camera.right, p), y: n * dot(camera.up, p), depth: n * dot(camera.toward, p) };
}

/** 🦀 The paper-up coordinates `(x, y)`, metres, of a world point in a viewpoint; the axonometric viewpoint uses `camera` (default the isometric one). */
export function project(viewpoint: Viewpoint, p: Vec3, camera?: AxonometricCamera): Point {
  if (viewpoint === "axonometric") {
    const q = projectAxonometric(camera ?? axonometricCamera(), p);
    return { x: q.x, y: q.y };
  }
  const planar = CAMERAS[viewpoint];
  return { x: planar.paperX.sign * p[planar.paperX.axis], y: planar.paperUp.sign * p[planar.paperUp.axis] };
}

/** 🧭️ The direction the viewer looks along in a viewpoint. */
export const lookingOf = (viewpoint: Viewpoint, camera?: AxonometricCamera): Vec3 => (viewpoint === "axonometric" ? (camera ?? axonometricCamera()).looking : CAMERAS[viewpoint].looking);

/** 🧊️ The distance of a world point toward the viewer, up to a constant: larger is nearer and is painted later. */
export function nearness(viewpoint: Viewpoint, p: Vec3, camera?: AxonometricCamera): number {
  const looking = lookingOf(viewpoint, camera);
  return -(p.x * looking.x + p.y * looking.y + p.z * looking.z);
}

/** 🚫️ Thrown when the drawing selected for a viewpoint has no shapes there: the view is missing and nothing is substituted, flattened or left blank. */
export class ViewMissingError extends Error {
  readonly viewpoint: Viewpoint;
  readonly asset: string;

  constructor(asset: string, viewpoint: Viewpoint) {
    super(`asset "${asset}" has no drawn shapes in the ${viewpoint} view of the selected representation; no other view is substituted`);
    this.name = "ViewMissingError";
    this.viewpoint = viewpoint;
    this.asset = asset;
  }
}
