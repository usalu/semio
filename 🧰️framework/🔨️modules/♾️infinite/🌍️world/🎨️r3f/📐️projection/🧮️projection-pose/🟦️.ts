/** 🧮️ World projection poses: the command, mode kind and field of view, the family, defaults, content bounds and their key, the cardinal, corner and orientation looks, half extent, ortho zoom, framing, the pose, orbit constraints and the gumball plane. */
import { type GumballPlaneId } from "@semio-tech/ui-react";
import { type WorldAxonometricHemisphere, type WorldAxonometricQuadrant, type WorldOrthographicViewId, type WorldProjectionFamily, type WorldProjectionKind, type WorldProjectionOrientation, type WorldProjectionSpec } from "../../🔭️scene-types/🟦️.ts";
import { type Vec3, type WorldCameraState } from "../../🧷️r3f-foundation/🟦️.ts";
import { type OrbitCameraViewId, orbitCameraZoomForProjection, vec3Normalize } from "../../📷️camera-view/🟦️.tsx";
import { WORLD_REFERENCE_DEFAULT_WIDTH } from "../../🖼️reference/🟦️.tsx";

// #region 📷️OrbitCameraView
/** 🔎️ Zoom carried through a projection gizmo snap — preserves live parallel zoom; maps perspective's unit zoom onto the orthographic default. */
export function worldProjectionSnapZoom(pendingSpec: WorldProjectionSpec, currentZoom: number, currentIsOrthographic: boolean): number {
  const family = worldProjectionFamily(pendingSpec);
  if (family === "parallel") {
    return currentIsOrthographic ? currentZoom : orbitCameraZoomForProjection("orthographic", currentZoom);
  }
  return orbitCameraZoomForProjection("perspective", currentZoom);
}

// #endregion 📷️OrbitCameraView
// #region 📐️WorldProjection
/** 📡️ Command name products handle to apply a {@link WorldProjectionSpec}. */
export const WORLD_PROJECTION_COMMAND = "setProjection";

/** 📐️ Kind discriminator for a composed {@link WorldProjectionSpec}. */
export function worldProjectionModeKind(spec: WorldProjectionSpec): WorldProjectionKind {
  return spec.mode.kind;
}

/** 📐️ Perspective FOV when the active mode carries one. */
export function worldProjectionModeFov(spec: WorldProjectionSpec): number | undefined {
  const mode = spec.mode;
  return mode.kind === "onePoint" || mode.kind === "twoPoint" || mode.kind === "threePoint" || mode.kind === "curvilinear" ? mode.fov : undefined;
}

/** 📐️ Effective `PerspectiveCamera.fov` for a spec — curvilinear is capped at 160° (matches {@link WorldCurvilinearPass}'s capture), others default 50°. */
export function worldProjectionPerspectiveFov(spec: WorldProjectionSpec): number {
  const fov = worldProjectionModeFov(spec);
  return spec.mode.kind === "curvilinear" ? Math.min(fov ?? 120, 160) : (fov ?? 50);
}

/** 📐️ Parallel family = orthographic camera (Orthographic/Axonometric/Oblique); everything else is perspective. */
export function worldProjectionFamily(spec: WorldProjectionSpec | undefined): WorldProjectionFamily {
  if (!spec) {
    return "perspective";
  }
  const kind = spec.mode.kind;
  return kind === "orthographic" || kind === "axonometric" || kind === "oblique" ? "parallel" : "perspective";
}

/** 📐️ Baseline mode + default orientation for a freshly-selected projection kind. */
export function worldProjectionDefaults(kind: WorldProjectionKind): WorldProjectionSpec {
  switch (kind) {
    case "orthographic":
      return { mode: { kind }, orientation: { type: "cardinal", view: "plan" } };
    case "axonometric":
      return { mode: { kind, variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" } };
    case "oblique":
      return { mode: { kind, variant: "cavalier", angle: 45, depthScale: 1 }, orientation: { type: "cardinal", view: "front" } };
    case "onePoint":
      return { mode: { kind, fov: 50 }, orientation: { type: "cardinal", view: "front" } };
    case "twoPoint":
      return { mode: { kind, fov: 50, verticalShift: 0 }, orientation: { type: "free" } };
    case "curvilinear":
      return { mode: { kind, fov: 120, strength: 1, mapping: "fisheye" }, orientation: { type: "free" } };
    default:
      return { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } };
  }
}

export const WORLD_PROJECTION_DEFAULT_DISTANCE = 600;
/** 📷️ Fallback canvas size when framing a projection before the live viewport is known. */
const WORLD_PROJECTION_FRAME_FALLBACK_VIEWPORT = 640;
/** 📷️ Default padding around content when framing a projection pane. */
const WORLD_PROJECTION_FRAME_PADDING = 1.35;

export type WorldSceneContentBounds = {
  readonly center: Vec3;
  readonly halfExtent: Vec3;
};

/** 📷️ Axis-aligned bounds of instances and visible reference planes, for framing projection panes. */
export function worldSceneContentBounds(
  instances: readonly { readonly position?: Vec3; readonly x?: number; readonly y?: number; readonly z?: number }[],
  references: readonly { readonly origin: Vec3; readonly widthWorld?: number; readonly hidden?: boolean }[] = [],
): WorldSceneContentBounds | null {
  let minX = Number.POSITIVE_INFINITY;
  let minY = Number.POSITIVE_INFINITY;
  let minZ = Number.POSITIVE_INFINITY;
  let maxX = Number.NEGATIVE_INFINITY;
  let maxY = Number.NEGATIVE_INFINITY;
  let maxZ = Number.NEGATIVE_INFINITY;
  let any = false;
  const expand = (x: number, y: number, z: number) => {
    minX = Math.min(minX, x);
    minY = Math.min(minY, y);
    minZ = Math.min(minZ, z);
    maxX = Math.max(maxX, x);
    maxY = Math.max(maxY, y);
    maxZ = Math.max(maxZ, z);
    any = true;
  };
  for (const instance of instances) {
    const position = instance.position ?? ([instance.x ?? 0, instance.y ?? 0, instance.z ?? 0] as Vec3);
    expand(position[0], position[1], position[2]);
  }
  for (const reference of references) {
    if (reference.hidden) continue;
    const half = Math.max(reference.widthWorld ?? WORLD_REFERENCE_DEFAULT_WIDTH, 1e-3) * 0.5;
    const [x, y, z] = reference.origin;
    expand(x - half, y - half, z);
    expand(x + half, y + half, z);
  }
  if (!any) return null;
  return {
    center: [(minX + maxX) * 0.5, (minY + maxY) * 0.5, (minZ + maxZ) * 0.5],
    halfExtent: [Math.max((maxX - minX) * 0.5, 0.5), Math.max((maxY - minY) * 0.5, 0.5), Math.max((maxZ - minZ) * 0.5, 0.5)],
  };
}

/** 📷️ Stable key for {@link WorldSceneContentBounds} — used to re-frame projection panes when fill (or any edit) expands the scene. */
export function worldSceneContentBoundsKey(bounds: WorldSceneContentBounds | null | undefined): string | null {
  if (!bounds) return null;
  const round = (value: number) => Math.round(value * 1e3) / 1e3;
  return `${round(bounds.center[0])},${round(bounds.center[1])},${round(bounds.center[2])}:${round(bounds.halfExtent[0])},${round(bounds.halfExtent[1])},${round(bounds.halfExtent[2])}`;
}

/** 📷️ Cardinal face look direction (Z-up CAD). */
export function worldProjectionCardinalLook(view: WorldOrthographicViewId): { readonly dir: Vec3; readonly up: Vec3 } {
  switch (view) {
    case "bottom":
      return { dir: [0, 0, -1], up: [0, -1, 0] };
    case "front":
      return { dir: [0, -1, 0], up: [0, 0, 1] };
    case "back":
      return { dir: [0, 1, 0], up: [0, 0, 1] };
    case "left":
      return { dir: [-1, 0, 0], up: [0, 0, 1] };
    case "right":
      return { dir: [1, 0, 0], up: [0, 0, 1] };
    default:
      return { dir: [0, 0, 1], up: [0, 1, 0] };
  }
}

/** 📷️ Corner look from axonometric elevation/azimuth (also used as spatial corner for non-axo modes). */
export function worldProjectionCornerLook(
  quadrant: WorldAxonometricQuadrant,
  hemisphere: WorldAxonometricHemisphere | undefined,
  angleADeg: number,
  angleBDeg: number,
): { readonly dir: Vec3; readonly up: Vec3 } {
  const angleA = angleADeg * (Math.PI / 180);
  const angleB = angleBDeg * (Math.PI / 180);
  const elevation = Math.asin(Math.sqrt(Math.tan(angleA) * Math.tan(angleB)));
  const azimuth = Math.atan(Math.sqrt(Math.tan(angleA) / Math.tan(angleB)));
  const signX = quadrant === "nw" || quadrant === "sw" ? -1 : 1;
  const signY = quadrant === "se" || quadrant === "sw" ? -1 : 1;
  const hemi = hemisphere ?? "upper";
  const dir: Vec3 = [signX * Math.cos(elevation) * Math.sin(azimuth), signY * Math.cos(elevation) * Math.cos(azimuth), Math.sin(elevation) * (hemi === "lower" ? -1 : 1)];
  return { dir, up: hemi === "lower" ? [0, 0, -1] : [0, 0, 1] };
}

/** 📷️ Resolves look direction from mode ⊗ orientation — every orientation works with every mode. */
export function worldProjectionOrientationLook(spec: WorldProjectionSpec): { readonly dir: Vec3; readonly up: Vec3 } {
  const { mode, orientation } = spec;
  switch (orientation.type) {
    case "cardinal":
      if (mode.kind === "oblique" && mode.variant === "military" && (orientation.view === "plan" || orientation.view === "top")) {
        const rotation = mode.angle * (Math.PI / 180);
        return { dir: [0, 0, 1], up: [Math.sin(rotation), Math.cos(rotation), 0] };
      }
      return worldProjectionCardinalLook(orientation.view);
    case "corner": {
      const angleA = mode.kind === "axonometric" ? (mode.variant === "isometric" ? 30 : mode.angleA) : 30;
      const angleB = mode.kind === "axonometric" ? (mode.variant === "isometric" ? 30 : mode.variant === "dimetric" ? mode.angleA : mode.angleB) : 30;
      return worldProjectionCornerLook(orientation.quadrant, orientation.hemisphere, angleA, angleB);
    }
    case "free":
      if (mode.kind === "twoPoint") {
        return { dir: [Math.SQRT1_2, -Math.SQRT1_2, 0], up: [0, 0, 1] };
      }
      if (mode.kind === "oblique") {
        if (mode.variant === "military") {
          const rotation = mode.angle * (Math.PI / 180);
          return { dir: [0, 0, 1], up: [Math.sin(rotation), Math.cos(rotation), 0] };
        }
        return { dir: [0, -1, 0], up: [0, 0, 1] };
      }
      return { dir: vec3Normalize([0.75, -0.75, 0.55]), up: [0, 0, 1] };
  }
}

/** 📷️ Half-width/height of `bounds` in the projection's view plane (CAD axes). */
export function worldProjectionViewHalfExtent(spec: WorldProjectionSpec, bounds: WorldSceneContentBounds): { readonly halfWidth: number; readonly halfHeight: number } {
  const [hx, hy, hz] = bounds.halfExtent;
  if (spec.orientation.type === "cardinal") {
    switch (spec.orientation.view) {
      case "front":
      case "back":
        return { halfWidth: hx, halfHeight: hz };
      case "left":
      case "right":
        return { halfWidth: hy, halfHeight: hz };
      default:
        return { halfWidth: hx, halfHeight: hy };
    }
  }
  if (spec.mode.kind === "oblique" && spec.mode.variant !== "military" && spec.orientation.type === "free") {
    return { halfWidth: hx, halfHeight: hz };
  }
  const span = Math.max(hx, hy, hz);
  return { halfWidth: span, halfHeight: span };
}

/** 📷️ Orthographic zoom that fits content in a drei/R3F pixel-sized frustum (`left = -width/2`, …). */
export function worldProjectionOrthoZoom(halfWidth: number, halfHeight: number, viewportWidth: number, viewportHeight: number, padding = WORLD_PROJECTION_FRAME_PADDING): number {
  const paddedHalfW = Math.max(halfWidth * padding, 0.5);
  const paddedHalfH = Math.max(halfHeight * padding, 0.5);
  const zoomX = viewportWidth * 0.5 / paddedHalfW;
  const zoomY = viewportHeight * 0.5 / paddedHalfH;
  return Math.max(Math.min(zoomX, zoomY), 1e-3);
}

/** 📷️ Frames a projection to scene content — centers the target and sets viewport-aware orthographic zoom. */
export function frameWorldProjectionPose(
  spec: WorldProjectionSpec,
  bounds: WorldSceneContentBounds,
  options?: { readonly padding?: number; readonly viewportWidth?: number; readonly viewportHeight?: number; readonly distance?: number },
): WorldCameraState {
  const padding = options?.padding ?? WORLD_PROJECTION_FRAME_PADDING;
  const { halfWidth, halfHeight } = worldProjectionViewHalfExtent(spec, bounds);
  const span = Math.max(halfWidth, halfHeight) * 2 * padding;
  const distance = options?.distance ?? Math.max(span * 1.5, 2);
  const family = worldProjectionFamily(spec);
  const zoom =
    family === "parallel"
      ? worldProjectionOrthoZoom(halfWidth, halfHeight, options?.viewportWidth ?? WORLD_PROJECTION_FRAME_FALLBACK_VIEWPORT, options?.viewportHeight ?? WORLD_PROJECTION_FRAME_FALLBACK_VIEWPORT, padding)
      : 1;
  return computeWorldProjectionPose(spec, { target: bounds.center, distance, zoom });
}

/** 📷️ Computes a Z-up camera pose for mode ⊗ orientation — look from orientation, camera family from mode. */
export function computeWorldProjectionPose(spec: WorldProjectionSpec, options: { readonly target: Vec3; readonly distance?: number; readonly zoom?: number }): WorldCameraState {
  const distance = options.distance ?? WORLD_PROJECTION_DEFAULT_DISTANCE;
  const target = options.target;
  const family = worldProjectionFamily(spec);
  const zoom = family === "parallel" ? (options.zoom ?? 50) : (options.zoom ?? 1);
  const { dir, up } = worldProjectionOrientationLook(spec);
  return {
    position: [target[0] + dir[0] * distance, target[1] + dir[1] * distance, target[2] + dir[2] * distance],
    target,
    up,
    zoom,
    projection: family === "parallel" ? "orthographic" : "perspective",
    projectionSpec: spec,
  };
}

/** 🔒️ Orbit-interaction constraints implied by a projection — drafting-locked orientations and modes disable rotation; two-point keeps the horizon level when free. */
export function worldProjectionOrbitConstraints(spec: WorldProjectionSpec | undefined): { readonly rotate: boolean; readonly minPolar?: number; readonly maxPolar?: number } {
  if (!spec) {
    return { rotate: true };
  }
  if (spec.orientation.type === "cardinal" && spec.orientation.view === "plan" && (spec.mode.kind === "orthographic" || spec.mode.kind === "oblique")) {
    return { rotate: false };
  }
  if (spec.mode.kind === "oblique" || spec.mode.kind === "onePoint") {
    return { rotate: false };
  }
  if (spec.mode.kind === "twoPoint" && spec.orientation.type === "free") {
    return { rotate: true, minPolar: Math.PI / 2, maxPolar: Math.PI / 2 };
  }
  return { rotate: true };
}

/** 🎛️ Drafting plane from cardinal orientation — `undefined` for corner/free 3D. */
export function worldProjectionGumballPlane(spec: WorldProjectionSpec | undefined): GumballPlaneId | undefined {
  if (!spec || spec.orientation.type !== "cardinal") return undefined;
  switch (spec.orientation.view) {
    case "front":
    case "back":
      return "xz";
    case "left":
    case "right":
      return "yz";
    default:
      return "xy";
  }
}

/** 🎛️ Drafting plane for a legacy {@link OrbitCameraViewId} preset (CAD panes that still seed via orbit views). */
export function orbitCameraViewGumballPlane(view: OrbitCameraViewId | undefined): GumballPlaneId | undefined {
  if (!view) return undefined;
  switch (view) {
    case "top":
    case "bottom":
      return "xy";
    case "front":
    case "back":
    case "north":
    case "south":
      return "xz";
    case "left":
    case "right":
    case "east":
    case "west":
      return "yz";
    default:
      return undefined;
  }
}

/** 📷️ Applies a projection kind/variant switch, keeping the current pose when the caller supplies one
 * (pure parameter tweaks like angle/depth/fov never move the camera — only the matrix driver re-shears). */
export function applyWorldProjectionToCameraState(state: WorldCameraState, spec: WorldProjectionSpec): WorldCameraState {
  const family = worldProjectionFamily(spec);
  return { ...state, projection: family === "parallel" ? "orthographic" : "perspective", projectionSpec: spec, zoom: family === "parallel" ? (state.zoom === 1 ? 50 : state.zoom) : state.zoom };
}

/** 🧭️ Structural equality for {@link WorldProjectionOrientation}. */
export function worldProjectionOrientationsEqual(a: WorldProjectionOrientation, b: WorldProjectionOrientation): boolean {
  return JSON.stringify(a) === JSON.stringify(b);
}
// #endregion 📐️WorldProjection
