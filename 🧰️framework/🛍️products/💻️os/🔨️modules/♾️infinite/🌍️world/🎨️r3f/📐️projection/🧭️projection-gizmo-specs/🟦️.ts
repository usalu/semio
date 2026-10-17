/** 🧭️ World projection specs of the gizmo: the gizmo spec and hit, the orbit view and its spec, and the kind switch spec. */
import { type ProjectionGizmoHit, type WorldAxonometricHemisphere, type WorldAxonometricQuadrant, type WorldOrthographicViewId, type WorldProjectionKind, type WorldProjectionSpec } from "../../🔭️scene-types/🟦️.ts";
import { type OrbitCameraViewId } from "../../📷️camera-view/🟦️.tsx";
import { worldProjectionDefaults, worldProjectionModeFov } from "../🧮️projection-pose/🟦️.ts";

// #region 📐️WorldProjection
/** 🧭️ Maps a projection-gizmo face to an orthographic view id. */
export function projectionGizmoFaceToOrthographicView(axis: "x" | "y" | "z", sign: 1 | -1): WorldOrthographicViewId {
  if (axis === "x") {
    return sign > 0 ? "right" : "left";
  }
  if (axis === "y") {
    return sign > 0 ? "back" : "front";
  }
  return sign > 0 ? "top" : "bottom";
}

/** 🧭️ Resolves a navigation-cube hit into orientation only — never changes projection mode.
 * Faces → cardinal views; corners → quadrant/hemisphere; center → free 3D. */
export function resolveProjectionGizmoSpec(hit: ProjectionGizmoHit, currentSpec?: WorldProjectionSpec): WorldProjectionSpec {
  const base = currentSpec ?? worldProjectionDefaults("threePoint");
  switch (hit.type) {
    case "face":
      return { mode: base.mode, orientation: { type: "cardinal", view: projectionGizmoFaceToOrthographicView(hit.axis, hit.sign) } };
    case "corner":
      return { mode: base.mode, orientation: { type: "corner", quadrant: hit.quadrant, hemisphere: hit.hemisphere } };
    case "center":
      return { mode: base.mode, orientation: { type: "free" } };
  }
}

/** 🧭️ Maps a direction vector to a navigation-cube hit (face, corner, or center). */
export function resolveProjectionGizmoHitFromDirection(direction: { readonly x: number; readonly y: number; readonly z: number }): ProjectionGizmoHit {
  const absX = Math.abs(direction.x);
  const absY = Math.abs(direction.y);
  const absZ = Math.abs(direction.z);
  if (absX > 0.45 && absY > 0.45 && absZ > 0.45) {
    const sx = direction.x >= 0 ? 1 : -1;
    const sy = direction.y >= 0 ? 1 : -1;
    const quadrant: WorldAxonometricQuadrant = sx > 0 && sy > 0 ? "ne" : sx < 0 && sy > 0 ? "nw" : sx > 0 && sy < 0 ? "se" : "sw";
    const hemisphere: WorldAxonometricHemisphere = direction.z >= 0 ? "upper" : "lower";
    return { type: "corner", quadrant, hemisphere };
  }
  if (absX < 0.2 && absY < 0.2 && absZ < 0.2) {
    return { type: "center" };
  }
  const dominant = [
    { axis: "x" as const, magnitude: absX, sign: (direction.x >= 0 ? 1 : -1) as 1 | -1 },
    { axis: "y" as const, magnitude: absY, sign: (direction.y >= 0 ? 1 : -1) as 1 | -1 },
    { axis: "z" as const, magnitude: absZ, sign: (direction.z >= 0 ? 1 : -1) as 1 | -1 },
  ].sort((a, b) => b.magnitude - a.magnitude)[0] ?? { axis: "z" as const, magnitude: 1, sign: 1 as const };
  return { type: "face", axis: dominant.axis, sign: dominant.sign };
}

/** 🧭️ Converts a legacy {@link OrbitCameraViewId} into a {@link WorldProjectionSpec}. */
export function orbitViewToWorldProjectionSpec(view: OrbitCameraViewId): WorldProjectionSpec {
  switch (view) {
    case "top":
    case "bottom":
    case "front":
    case "back":
    case "left":
    case "right":
      return { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view } };
    case "north":
      return { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "back" } };
    case "south":
      return { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } };
    case "east":
      return { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "right" } };
    case "west":
      return { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "left" } };
    case "isometricNe":
      return { mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" } };
    case "isometricNw":
      return { mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "nw", hemisphere: "upper" } };
    case "isometricSe":
      return { mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "se", hemisphere: "upper" } };
    case "isometricSw":
      return { mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "sw", hemisphere: "upper" } };
    case "twoPointPerspective":
      return worldProjectionDefaults("twoPoint");
    case "perspective":
    default:
      return worldProjectionDefaults("threePoint");
  }
}

/** 🧭️ Best-effort reverse of {@link orbitViewToWorldProjectionSpec} for legacy orbit callbacks. */
export function worldProjectionSpecToOrbitView(spec: WorldProjectionSpec): OrbitCameraViewId | null {
  if (spec.orientation.type === "cardinal") {
    return spec.orientation.view === "plan" ? "top" : spec.orientation.view;
  }
  if (spec.orientation.type === "corner") {
    switch (spec.orientation.quadrant) {
      case "nw":
        return "isometricNw";
      case "se":
        return "isometricSe";
      case "sw":
        return "isometricSw";
      default:
        return "isometricNe";
    }
  }
  if (spec.mode.kind === "twoPoint") return "twoPointPerspective";
  return "perspective";
}

/** 🔀️ Switches projection *mode* while preserving gizmo orientation (and FOV when possible). */
export function worldProjectionKindSwitchSpec(kind: WorldProjectionKind, currentSpec?: WorldProjectionSpec): WorldProjectionSpec {
  const next = worldProjectionDefaults(kind);
  const orientation = currentSpec?.orientation ?? next.orientation;
  if (kind === "onePoint" || kind === "twoPoint" || kind === "threePoint" || kind === "curvilinear") {
    const fov = worldProjectionModeFov(currentSpec ?? next) ?? worldProjectionModeFov(next)!;
    if (kind === "twoPoint") {
      const verticalShift = currentSpec?.mode.kind === "twoPoint" ? currentSpec.mode.verticalShift : 0;
      return { mode: { kind, fov, verticalShift }, orientation };
    }
    if (kind === "curvilinear") {
      const mapping = currentSpec?.mode.kind === "curvilinear" ? currentSpec.mode.mapping : "fisheye";
      const strength = currentSpec?.mode.kind === "curvilinear" ? currentSpec.mode.strength : 1;
      return { mode: { kind, fov: currentSpec?.mode.kind === "curvilinear" ? currentSpec.mode.fov : 120, strength, mapping }, orientation };
    }
    return { mode: { kind, fov }, orientation };
  }
  if (kind === "axonometric") {
    if (currentSpec?.mode.kind === "axonometric") {
      return { mode: currentSpec.mode, orientation };
    }
    return { mode: next.mode, orientation };
  }
  if (kind === "oblique") {
    if (currentSpec?.mode.kind === "oblique") {
      return { mode: currentSpec.mode, orientation };
    }
    return { mode: next.mode, orientation };
  }
  return { mode: next.mode, orientation };
}
// #endregion 📐️WorldProjection
