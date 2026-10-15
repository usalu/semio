/** 📐️ World projections: defaults, content bounds and framing, poses, orbit constraints, matrices, the projection rig, applier and curvilinear pass, template ids, labels and icons, and the projection switch panes.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | r3f}, which re-exports every public name below so import specifiers stay unchanged.
 */

import {
  childElementId,
  cn,
  floatingRibbonSurfaceClass,
  LevelProvider,
  reactHostPort,
  sceneHostPort,
  Tree,
  Icon,
  type IconName,
  type GumballPlaneId,
  type TreeDataItem
} from "@semio-tech/ui-react";
import { type ReactElement } from "react";
import {
  type Camera,
  HalfFloatType,
  LinearFilter,
  LinearSRGBColorSpace,
  Matrix4,
  Mesh,
  OrthographicCamera,
  PerspectiveCamera,
  PlaneGeometry,
  RGBAFormat,
  Scene,
  ShaderMaterial,
  ThreeOrthographicCamera,
  ThreePerspectiveCamera,
  useFrame,
  useThree,
  Vector3,
  WebGLRenderTarget
} from "../🔌️adapters/🟦️.ts";
import {
  type ProjectionGizmoHit,
  type WorldAxonometricHemisphere,
  type WorldAxonometricQuadrant,
  type WorldAxonometricVariant,
  type WorldObliqueVariant,
  type WorldOrthographicViewId,
  type WorldProjectionFamily,
  type WorldProjectionKind,
  type WorldProjectionMode,
  type WorldProjectionOrientation,
  type WorldProjectionSpec
} from "../🔭️scene-types/🟦️.ts";
import {
  cadVec3ToThree,
  type OrbitCameraProjection,
  threeVec3ToCad,
  type Vec3,
  type WorldCameraState
} from "../🧷️r3f-foundation/🟦️.ts";
import {
  ORBIT_CAMERA_Z_UP,
  orbitCameraDistance,
  type OrbitCameraViewId,
  orbitCameraZoomForProjection,
  type OrbitControlsTarget,
  useWorldOrbitViewSnapGate,
  vec3Normalize,
  WORLD_ORBIT_CAMERA_MIN_FAR,
  WorldOrbitCameraViewRigSeed,
  worldProjectionMatchedOrthoZoom
} from "../📷️camera-view/🟦️.tsx";
import { WORLD_REFERENCE_DEFAULT_WIDTH } from "../🖼️reference/🟦️.tsx";

// #region 📷️OrbitCameraView
/** 🔎️ Zoom carried through a projection gizmo snap — preserves live parallel zoom; maps perspective's unit zoom onto the orthographic default. */
export function worldProjectionSnapZoom(pendingSpec: WorldProjectionSpec, currentZoom: number, currentIsOrthographic: boolean): number {
  const family = worldProjectionFamily(pendingSpec);
  if (family === "parallel") {
    return currentIsOrthographic ? currentZoom : orbitCameraZoomForProjection("orthographic", currentZoom);
  }
  return orbitCameraZoomForProjection("perspective", currentZoom);
}

/** 🧭️ Maps a CAD viewport gizmo axis click to a named orbit view (Z-up). */
export function resolveOrbitGizmoViewFromDirection(direction: { readonly x: number; readonly y: number; readonly z: number }): OrbitCameraViewId {
  const spec = resolveProjectionGizmoSpec(resolveProjectionGizmoHitFromDirection(direction));
  return worldProjectionSpecToOrbitView(spec) ?? "top";
}

export const WORLD_PROJECTION_KINDS: readonly WorldProjectionKind[] = ["orthographic", "axonometric", "oblique", "onePoint", "twoPoint", "threePoint", "curvilinear"];

export interface WorldProjectionModeOption {
  readonly id: string;
  readonly label: string;
  readonly spec: WorldProjectionSpec;
  readonly active: boolean;
}

/** 🔀️ Non-spatial mode variants that sit beside the template tree (kept for callers that still want a flat list).
 * Prefer {@link createWorldProjectionTemplates} / {@link WorldProjectionKindSwitch} for the canonical taxonomy. */
export function worldProjectionModeOptions(spec: WorldProjectionSpec): readonly WorldProjectionModeOption[] {
  const mode = spec.mode;
  switch (mode.kind) {
    case "axonometric": {
      const variants: readonly { readonly variant: WorldAxonometricVariant; readonly label: string; readonly angleA: number; readonly angleB: number }[] = [
        { variant: "isometric", label: "Iso", angleA: 30, angleB: 30 },
        { variant: "dimetric", label: "Di", angleA: 15, angleB: 15 },
        { variant: "trimetric", label: "Tri", angleA: 12, angleB: 42 },
      ];
      return variants.map((row) => ({
        id: `axonometric-${row.variant}`,
        label: row.label,
        spec: { mode: { kind: "axonometric", variant: row.variant, angleA: row.angleA, angleB: row.angleB }, orientation: spec.orientation },
        active: mode.variant === row.variant,
      }));
    }
    case "oblique": {
      const variants: readonly { readonly variant: WorldObliqueVariant; readonly label: string; readonly depthScale: number }[] = [
        { variant: "cabinet", label: "Cab", depthScale: 0.5 },
        { variant: "cavalier", label: "Cav", depthScale: 1 },
        { variant: "military", label: "Mil", depthScale: 1 },
      ];
      return variants.map((row) => ({
        id: `oblique-${row.variant}`,
        label: row.label,
        spec: { mode: { kind: "oblique", variant: row.variant, angle: mode.angle, depthScale: row.depthScale }, orientation: spec.orientation },
        active: mode.variant === row.variant,
      }));
    }
    case "curvilinear":
      return (["fisheye", "panini"] as const).map((mapping) => ({
        id: `curvilinear-${mapping}`,
        label: mapping === "fisheye" ? "Fish" : "Pan",
        spec: { mode: { kind: "curvilinear", fov: mode.fov, strength: mode.strength, mapping }, orientation: spec.orientation },
        active: mode.mapping === mapping,
      }));
    default:
      return [];
  }
}

/** 🌲️ Template-tree node id for the active *mode* (orientation is gizmo-owned and does not change selection). */
export function worldProjectionTemplateSelectionId(spec: WorldProjectionSpec): string {
  switch (spec.mode.kind) {
    case "orthographic":
      return "orthographic";
    case "axonometric":
      return `axonometric-${spec.mode.variant}`;
    case "oblique":
      return `oblique-${spec.mode.variant}`;
    case "onePoint":
      return "one-point";
    case "twoPoint":
      return "two-point";
    case "threePoint":
      return "three-point";
    case "curvilinear":
      return "curvilinear";
  }
}

/** 🌲️ Applies a template-tree *mode* while preserving the current gizmo orientation. */
export function worldProjectionTemplateApplySpec(templateSpec: WorldProjectionSpec, currentSpec?: WorldProjectionSpec): WorldProjectionSpec {
  return {
    mode: templateSpec.mode,
    orientation: currentSpec?.orientation ?? templateSpec.orientation,
  };
}

/** 🌲️ Maps {@link createWorldProjectionTemplates} into selectable {@link TreeDataItem}s for the live projection pane.
 * `id` is the OWNING pane's element id: `TreeSection`/`TreeItem` render their item id as the row's DOM `id`, and one
 * world surface is mounted per open window instance, so a bare `template.id` puts `orthographic` (and every sibling)
 * in the document once per pane. Every row id is therefore a {@link childElementId} of the pane. */
export function worldProjectionSwitchTreeItems(id: string, templates: readonly WorldProjectionTemplateDescriptor[], onSelect: (spec: WorldProjectionSpec) => void, currentSpec?: WorldProjectionSpec): TreeDataItem[] {
  return templates.map((template) => ({
    id: childElementId(id, template.id),
    label: template.label,
    icon: <Icon icon={template.iconId} size={12} className="size-tiny shrink-0" />,
    defaultOpen: true,
    onClick: () => onSelect(worldProjectionTemplateApplySpec(template.args.spec, currentSpec)),
    ...(template.children?.length ? { items: worldProjectionSwitchTreeItems(id, template.children, onSelect, currentSpec) } : {}),
  }));
}

export interface WorldProjectionKindSwitchProps {
  /** 🪪️ Element id of the pane this switch is mounted in — qualifies every row id per window instance. */
  readonly id: string;
  readonly spec: WorldProjectionSpec;
  readonly onSpecChange: (spec: WorldProjectionSpec) => void;
  readonly className?: string;
  /** 🪟️ `"pane"` (default) renders a transparent tree inside {@link Pane} body — same silhouette as window options. `"ribbon"` keeps {@link floatingRibbonSurfaceClass} for standalone window-level overlays. */
  readonly surface?: "pane" | "ribbon";
}

/** 🌲️ Projection-mode switcher — same Parallel/Perspective taxonomy as {@link createWorldProjectionTemplates}; cube owns spatial angles. */
export function WorldProjectionKindSwitch(props: WorldProjectionKindSwitchProps): ReactElement {
  const surface = props.surface ?? "pane";
  const shellClass =
    props.className ??
    (surface === "ribbon"
      ? cn("pointer-events-auto min-w-40 text-2xs font-medium", floatingRibbonSurfaceClass)
      : cn("pointer-events-auto w-full min-w-0 text-tiny font-medium"));
  const templates = reactHostPort.useMemo(() => createWorldProjectionTemplates({ controllerId: "projection-switch" }), []);
  const items = reactHostPort.useMemo(() => worldProjectionSwitchTreeItems(props.id, templates, props.onSpecChange, props.spec), [props.id, templates, props.onSpecChange, props.spec]);
  const selectedId = childElementId(props.id, worldProjectionTemplateSelectionId(props.spec));
  const tree = <Tree showLines={surface === "pane"} sortableSections={false} selectionMode="single" selectedIds={[selectedId]} sections={[{ id: childElementId(props.id, "projection-modes"), items }]} />;
  return (
    <div className={shellClass} data-world-projection-kind-switch {...(surface === "ribbon" ? { "data-level": "window" as const } : {})}>
      {surface === "ribbon" ? <LevelProvider level="window">{tree}</LevelProvider> : tree}
    </div>
  );
}

export interface WorldOrbitProjectionSwitchProps {
  /** 🪪️ Element id of the pane this switch is mounted in — see {@link WorldProjectionKindSwitchProps.id}. */
  readonly id: string;
  readonly projection: OrbitCameraProjection;
  readonly onProjectionChange: (projection: OrbitCameraProjection) => void;
  readonly className?: string;
  readonly spec?: WorldProjectionSpec;
  readonly onSpecChange?: (spec: WorldProjectionSpec) => void;
}

/** 🔀️ Orthographic / perspective toggle, or full {@link WorldProjectionKindSwitch} when `spec` is provided. */
export function WorldOrbitProjectionSwitch(props: WorldOrbitProjectionSwitchProps): ReactElement {
  if (props.spec && props.onSpecChange) {
    return <WorldProjectionKindSwitch id={props.id} spec={props.spec} onSpecChange={props.onSpecChange} className={props.className} />;
  }
  const shellClass = props.className ?? cn("pointer-events-auto flex text-2xs font-medium", floatingRibbonSurfaceClass);
  const buttonClass = (active: boolean) => cn("px-2 py-1 transition-colors text-muted-foreground hover:text-emphasized", active && "text-emphasized");
  return (
    <div className={shellClass} data-world-projection-switch data-level="window">
      <button type="button" className={buttonClass(props.projection === "orthographic")} aria-pressed={props.projection === "orthographic"} onClick={() => props.onProjectionChange("orthographic")}>
        Ortho
      </button>
      <button type="button" className={buttonClass(props.projection === "perspective")} aria-pressed={props.projection === "perspective"} onClick={() => props.onProjectionChange("perspective")}>
        Persp
      </button>
    </div>
  );
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

const WORLD_PROJECTION_DEFAULT_DISTANCE = 600;
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
    default: // "plan" | "top"
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

/** 🪞️ Oblique receding-axis shear matrix — extracted from {@link WorldProjectionMatrixDriver} so
 * {@link worldProjectionGoalMatrix} and the live per-frame shear share one formula; `strength` ramps the shear 0→1. */
export function worldObliqueShearMatrix(mode: Extract<WorldProjectionMode, { kind: "oblique" }>, strength = 1): Matrix4 {
  const angle = mode.angle * (Math.PI / 180);
  const l = mode.variant === "military" ? 1 : mode.depthScale;
  const alpha = mode.variant === "military" ? Math.PI / 2 : angle;
  const shear = new Matrix4().identity();
  shear.elements[8] = -l * Math.cos(alpha) * strength;
  shear.elements[9] = -l * Math.sin(alpha) * strength;
  return shear;
}

/** 📐️ Destination projection matrix for `spec` at the live viewport size — the "goal" a projection-matrix morph
 * lerps toward each frame. Built from real three.js camera classes so it matches exactly what mounting that camera would
 * produce (drei's `OrthographicCamera`/`PerspectiveCamera` seed left/right/top/bottom or aspect the same way). */
export function worldProjectionGoalMatrix(spec: WorldProjectionSpec, options: { readonly zoom: number; readonly fov?: number; readonly viewport: { readonly width: number; readonly height: number }; readonly near?: number; readonly far?: number }): Matrix4 {
  const near = options.near ?? 0.2;
  const far = options.far ?? WORLD_ORBIT_CAMERA_MIN_FAR;
  const { width, height } = options.viewport;
  if (worldProjectionFamily(spec) === "parallel") {
    const camera = new ThreeOrthographicCamera(width / -2, width / 2, height / 2, height / -2, near, far);
    camera.zoom = options.zoom;
    camera.updateProjectionMatrix();
    const matrix = camera.projectionMatrix.clone();
    return spec.mode.kind === "oblique" ? matrix.multiply(worldObliqueShearMatrix(spec.mode)) : matrix;
  }
  const fov = options.fov ?? worldProjectionPerspectiveFov(spec);
  const camera = new ThreePerspectiveCamera(fov, width / Math.max(1, height), near, far);
  camera.zoom = options.zoom;
  camera.updateProjectionMatrix();
  const matrix = camera.projectionMatrix.clone();
  if (spec.mode.kind === "twoPoint") {
    matrix.elements[9] += spec.mode.verticalShift;
  }
  return matrix;
}

/** 🎞️ Element-wise projection-matrix lerp — valid across every pair this engine morphs between: persp↔ortho
 * lerps the w-row (`-z ↔ 1`, the standard projective blend), persp↔persp is cot-space FOV interpolation, and
 * oblique-shear/two-point-shift elements ramp linearly since they're already linear terms of the base matrix. */
export function worldProjectionMorphMatrix(from: Matrix4, to: Matrix4, t: number): Matrix4 {
  const result = new Matrix4();
  for (let i = 0; i < 16; i++) {
    result.elements[i] = from.elements[i] + (to.elements[i] - from.elements[i]) * t;
  }
  return result;
}

/** 🎞️ Destination pose for a projection snap — preserves eye exactly when only mode changes (matches
 * {@link applyWorldProjectionToCameraState}'s "pure parameter tweaks never move the camera" invariant); re-looks only
 * when orientation changes. When `live.viewport` is supplied, a persp→ortho mode-only switch picks a zoom that matches
 * the live apparent scale via {@link worldProjectionMatchedOrthoZoom} instead of the legacy zoom-50 default — this only
 * changes *which* zoom value is chosen, never the camera position, so it cannot reposition the camera. */
export function worldProjectionTransitionPose(
  pendingSpec: WorldProjectionSpec,
  live: {
    readonly position: Vec3;
    readonly target: Vec3;
    readonly up?: Vec3;
    readonly zoom: number;
    readonly isOrthographic: boolean;
    readonly projectionSpec?: WorldProjectionSpec;
    readonly viewport?: { readonly width: number; readonly height: number };
    readonly fov?: number;
  },
): WorldCameraState {
  const previousSpec = live.projectionSpec;
  const orientationUnchanged = previousSpec !== undefined && worldProjectionOrientationsEqual(previousSpec.orientation, pendingSpec.orientation);
  if (orientationUnchanged && previousSpec && live.viewport && worldProjectionFamily(previousSpec) === "perspective" && worldProjectionFamily(pendingSpec) === "parallel") {
    const fromFov = live.fov ?? worldProjectionPerspectiveFov(previousSpec);
    const distance = orbitCameraDistance({ position: live.position, target: live.target, zoom: live.zoom });
    const zoom = worldProjectionMatchedOrthoZoom(fromFov, distance, live.viewport.height);
    return applyWorldProjectionToCameraState({ position: live.position, target: live.target, up: live.up ?? ORBIT_CAMERA_Z_UP, zoom, projection: "orthographic", projectionSpec: pendingSpec }, pendingSpec);
  }
  const zoom = worldProjectionSnapZoom(pendingSpec, live.zoom, live.isOrthographic);
  if (orientationUnchanged || !previousSpec) {
    return applyWorldProjectionToCameraState(
      {
        position: live.position,
        target: live.target,
        up: live.up ?? ORBIT_CAMERA_Z_UP,
        zoom,
        projection: live.isOrthographic ? "orthographic" : "perspective",
        projectionSpec: pendingSpec,
      },
      pendingSpec,
    );
  }
  const distance = orbitCameraDistance({ position: live.position, target: live.target, zoom: live.zoom });
  return computeWorldProjectionPose(pendingSpec, { target: live.target, distance, zoom });
}

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

/** 📷️ Mounts the camera for a {@link WorldProjectionSpec} and the matrix/curvilinear post-processing it needs. */
export function WorldProjectionRig(props: { readonly spec: WorldProjectionSpec; readonly state: WorldCameraState; readonly seedKey: string | number; readonly onCamera?: (camera: Camera | null) => void; readonly pendingSpec?: WorldProjectionSpec | null }): ReactElement {
  const family = worldProjectionFamily(props.spec);
  const up = cadVec3ToThree(props.state.up ?? ORBIT_CAMERA_Z_UP);
  const cameraKey = `${props.spec.mode.kind}:${props.seedKey}`;
  const perspectiveFov = worldProjectionPerspectiveFov(props.spec);
  // 🐟️ Pre-mount the curvilinear pass while only the *pending* spec is curvilinear so its render target is warm and
  // its strength can ramp in via `morphRef` before the camera remount, instead of popping in at full strength.
  const curvilinearMode = props.spec.mode.kind === "curvilinear" ? props.spec.mode : props.pendingSpec?.mode.kind === "curvilinear" ? props.pendingSpec.mode : null;
  return (
    <>
      {family === "parallel" ? (
        <OrthographicCamera key={cameraKey} ref={props.onCamera} makeDefault up={up} near={0.2} far={WORLD_ORBIT_CAMERA_MIN_FAR} zoom={props.state.zoom} />
      ) : (
        <PerspectiveCamera key={cameraKey} ref={props.onCamera} makeDefault up={up} near={0.2} far={WORLD_ORBIT_CAMERA_MIN_FAR} fov={perspectiveFov} zoom={props.state.zoom} />
      )}
      <WorldOrbitCameraViewRigSeed state={props.state} seedKey={`${cameraKey}`} />
      {props.spec.mode.kind === "oblique" || props.spec.mode.kind === "twoPoint" ? <WorldProjectionMatrixDriver mode={props.spec.mode} /> : null}
      {curvilinearMode ? <WorldCurvilinearPass mode={curvilinearMode} /> : null}
    </>
  );
}

/** 📷️ Applies a projection preset when `seedKey` changes (owned-camera canvases) — mirrors {@link WorldOrbitCameraViewApplier}. */
export function WorldProjectionApplier(props: { readonly spec: WorldProjectionSpec; readonly seedKey: string | number }): ReactElement {
  const { camera } = useThree();
  const controls = useThree((s) => s.controls as OrbitControlsTarget | null);
  const targetScratch = reactHostPort.useMemo(() => new Vector3(), []);
  const state = reactHostPort.useMemo(() => {
    const target = controls?.target ?? targetScratch.set(0, 0, 0);
    const targetCad = threeVec3ToCad(target);
    const distance = camera ? Math.hypot(camera.position.x - target.x, camera.position.y - target.y, camera.position.z - target.z) || WORLD_PROJECTION_DEFAULT_DISTANCE : WORLD_PROJECTION_DEFAULT_DISTANCE;
    return computeWorldProjectionPose(props.spec, { target: targetCad, distance });
  }, [camera, controls, props.seedKey, props.spec, targetScratch]);
  return <WorldProjectionRig spec={props.spec} state={state} seedKey={props.seedKey} />;
}

/** 🪞️ Post-multiplies the oblique receding-axis shear or two-point vertical lens-shift onto the active
 * camera's projection matrix every frame (after recomputing the pristine matrix, so resize/zoom never compounds
 * the shear) and refreshes `projectionMatrixInverse` — required because r3f raycasting/picking reads the inverse.
 * Defers to {@link WorldProjectionSnapDriver} while a projection morph is in flight (`morphRef.current`), since the
 * morph's goal matrix already bakes in the destination shear/shift via {@link worldObliqueShearMatrix} — running both
 * would double-apply the transform. */
function WorldProjectionMatrixDriver(props: { readonly mode: Extract<WorldProjectionMode, { kind: "oblique" | "twoPoint" }> }): null {
  const { camera, invalidate } = useThree();
  const { morphRef } = useWorldOrbitViewSnapGate();
  useFrame(() => {
    if (morphRef.current) {
      return;
    }
    camera.updateProjectionMatrix();
    if (props.mode.kind === "oblique" && camera instanceof ThreeOrthographicCamera) {
      camera.projectionMatrix.multiply(worldObliqueShearMatrix(props.mode));
    } else if (props.mode.kind === "twoPoint" && camera instanceof ThreePerspectiveCamera) {
      camera.projectionMatrix.elements[9] += props.mode.verticalShift;
    }
    camera.projectionMatrixInverse.copy(camera.projectionMatrix).invert();
    invalidate();
  });
  return null;
}

/** 🐟️ Remaps a pointer NDC coordinate through the inverse curvilinear radius mapping so raycasting against
 * the exact (undistorted) capture-space render stays pixel-accurate under fisheye/panini distortion. */
export function worldCurvilinearUnproject(ndc: readonly [number, number], mode: Extract<WorldProjectionMode, { kind: "curvilinear" }>, aspect = 1): readonly [number, number] {
  const halfFov = (Math.min(mode.fov, 160) * (Math.PI / 180)) / 2;
  const scaled: [number, number] = [ndc[0] * aspect, ndc[1]];
  const r = Math.hypot(scaled[0], scaled[1]);
  if (r < 1e-6) {
    return ndc;
  }
  const theta = r * halfFov;
  const rectilinearR = Math.tan(theta) / Math.tan(halfFov);
  const sourceR = r + (rectilinearR - r) * mode.strength;
  const scale = sourceR / r;
  return [(scaled[0] * scale) / aspect, scaled[1] * scale];
}

const WORLD_CURVILINEAR_VERTEX_SHADER = `
  varying vec2 vUv;
  void main() {
    vUv = uv;
    gl_Position = vec4(position.xy, 0.0, 1.0);
  }
`;

/** 🐟️ Fisheye/panini remap blit — samples the linear capture and applies Three's output color-space convert. */
export const WORLD_CURVILINEAR_FRAGMENT_SHADER = `
  uniform sampler2D tCapture;
  uniform float uFov;
  uniform float uStrength;
  uniform float uAspect;
  uniform float uMapping; // 0 = fisheye, 1 = panini
  varying vec2 vUv;
  void main() {
    vec2 ndc = vUv * 2.0 - 1.0;
    vec2 scaled = vec2(ndc.x * uAspect, ndc.y);
    float halfFov = uFov * 0.5;
    float r = length(scaled);
    vec2 sourceNdc = ndc;
    if (r > 1e-5) {
      float theta = r * halfFov;
      float rectR = tan(theta) / tan(halfFov);
      float sourceR = mix(r, rectR, uStrength);
      float scale = sourceR / r;
      sourceNdc = vec2((scaled.x * scale) / uAspect, scaled.y * scale);
    }
    vec2 sourceUv = sourceNdc * 0.5 + 0.5;
    if (sourceUv.x < 0.0 || sourceUv.x > 1.0 || sourceUv.y < 0.0 || sourceUv.y > 1.0) {
      gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
      #include <colorspace_fragment>
      return;
    }
    gl_FragColor = texture2D(tCapture, sourceUv);
    #include <colorspace_fragment>
  }
`;

/** 🐟️ Capture-target defaults for the curvilinear blit: linear working-space storage, linear filtering
 * (fisheye UV warp must interpolate), half-float precision (same as pmndrs/drei `RenderCubeTexture`), sized by the
 * caller at drawing-buffer resolution. Not drei `<Fisheye>` — that component portals children into a cubemap sphere
 * and has no FOV/strength/panini; our taxonomy needs a sibling planar remap pass instead. */
export const WORLD_CURVILINEAR_CAPTURE_TARGET_OPTIONS = {
  minFilter: LinearFilter,
  magFilter: LinearFilter,
  format: RGBAFormat,
  type: HalfFloatType,
  colorSpace: LinearSRGBColorSpace,
} as const;

/** 🐟️ Wide-FOV planar capture (capped at 160°, exact and material-agnostic below full 180°) remapped through
 * a fullscreen fisheye/panini shader — chosen over pmndrs/drei `<Fisheye>` / a 6-face cubemap unwrap because it needs
 * one extra render target instead of six, keeps orbit/gizmo chrome undistorted as siblings, supports strength+panini,
 * and the taxonomy never promises >=180° coverage. */
function WorldCurvilinearPass(props: { readonly mode: Extract<WorldProjectionMode, { kind: "curvilinear" }> }): null {
  const { gl, camera, scene, size, invalidate } = useThree();
  const { morphRef } = useWorldOrbitViewSnapGate();
  const targetRef = reactHostPort.useRef<InstanceType<typeof WebGLRenderTarget<InstanceType<typeof sceneHostPort.three.Texture>>> | null>(null);
  const quadRef = reactHostPort.useRef<{ readonly scene: InstanceType<typeof Scene>; readonly camera: InstanceType<typeof ThreeOrthographicCamera>; readonly material: InstanceType<typeof ShaderMaterial> } | null>(null);

  useFrame(() => {
    if (!(camera instanceof ThreePerspectiveCamera)) {
      return;
    }
    const pixelRatio = gl.getPixelRatio();
    const width = Math.max(1, Math.floor(size.width * pixelRatio));
    const height = Math.max(1, Math.floor(size.height * pixelRatio));
    if (!targetRef.current || targetRef.current.width !== width || targetRef.current.height !== height) {
      targetRef.current?.dispose();
      targetRef.current = new WebGLRenderTarget(width, height, { ...WORLD_CURVILINEAR_CAPTURE_TARGET_OPTIONS });
    }
    if (!quadRef.current) {
      const material = new ShaderMaterial({
        vertexShader: WORLD_CURVILINEAR_VERTEX_SHADER,
        fragmentShader: WORLD_CURVILINEAR_FRAGMENT_SHADER,
        uniforms: { tCapture: { value: targetRef.current.texture }, uFov: { value: 1 }, uStrength: { value: 1 }, uAspect: { value: 1 }, uMapping: { value: 0 } },
        depthTest: false,
        depthWrite: false,
        toneMapped: false,
      });
      const quadScene = new Scene();
      quadScene.add(new Mesh(new PlaneGeometry(2, 2), material));
      const quadCamera = new ThreeOrthographicCamera(-1, 1, 1, -1, 0, 1);
      quadRef.current = { scene: quadScene, camera: quadCamera, material };
    }
    const target = targetRef.current;
    const { material } = quadRef.current;
    // 🐟️ Fade `uStrength`/`uFov` with the live projection morph — entering curvilinear ramps 0→target strength,
    // leaving ramps target→0; `uStrength=0` is an exact identity blit so the fade can never pop.
    const morph = morphRef.current;
    const enteringCurvilinear = morph?.toSpec.mode.kind === "curvilinear";
    const leavingCurvilinear = morph?.fromSpec?.mode.kind === "curvilinear";
    let ramp = 1;
    let fov = props.mode.fov;
    if (morph && enteringCurvilinear && !leavingCurvilinear) {
      ramp = morph.eased;
      fov = morph.fromFov + (morph.toFov - morph.fromFov) * morph.eased;
    } else if (morph && leavingCurvilinear && !enteringCurvilinear) {
      ramp = 1 - morph.eased;
      fov = morph.fromFov + (morph.toFov - morph.fromFov) * morph.eased;
    } else if (morph && !enteringCurvilinear && !leavingCurvilinear) {
      ramp = 0;
    }
    material.uniforms.tCapture.value = target.texture;
    material.uniforms.uFov.value = Math.min(fov, 160) * (Math.PI / 180);
    material.uniforms.uStrength.value = props.mode.strength * ramp;
    material.uniforms.uAspect.value = size.width / Math.max(1, size.height);
    material.uniforms.uMapping.value = props.mode.mapping === "panini" ? 1 : 0;

    gl.setRenderTarget(target);
    gl.render(scene, camera);
    gl.setRenderTarget(null);
    gl.render(quadRef.current.scene, quadRef.current.camera);
    invalidate();
  }, 1);

  reactHostPort.useEffect(() => {
    return () => {
      targetRef.current?.dispose();
      quadRef.current?.material.dispose();
    };
  }, []);
  return null;
}

export interface WorldProjectionTemplateDescriptor {
  readonly id: string;
  readonly label: string;
  readonly iconId: IconName;
  readonly controllerId: string;
  readonly command: string;
  readonly args: { readonly spec: WorldProjectionSpec };
  readonly children?: readonly WorldProjectionTemplateDescriptor[];
}

export interface CreateWorldProjectionTemplatesConfig {
  readonly controllerId: string;
  readonly command?: string;
}

function worldProjectionTemplateLeaf(controllerId: string, command: string, id: string, label: string, iconId: IconName, spec: WorldProjectionSpec): WorldProjectionTemplateDescriptor {
  return { id, label, iconId, controllerId, command, args: { spec } };
}

function worldProjectionTemplateBranch(controllerId: string, command: string, id: string, label: string, iconId: IconName, spec: WorldProjectionSpec, children: readonly WorldProjectionTemplateDescriptor[]): WorldProjectionTemplateDescriptor {
  return { id, label, iconId, controllerId, command, args: { spec }, children };
}

/** 🪟️ Builds the projection-mode taxonomy tree (no Top/Front — those are gizmo orientations):
 * `Parallel > Orthographic | Axonometric (Isometric/Dimetric/Trimetric) | Oblique (Cabinet/Cavalier/Military)`
 * and `Perspective > 1-Point/2-Point/3-Point/Curvilinear`. */
export function createWorldProjectionTemplates(config: CreateWorldProjectionTemplatesConfig): readonly WorldProjectionTemplateDescriptor[] {
  const { controllerId } = config;
  const command = config.command ?? WORLD_PROJECTION_COMMAND;
  const leaf = (id: string, label: string, iconId: IconName, spec: WorldProjectionSpec) => worldProjectionTemplateLeaf(controllerId, command, id, label, iconId, spec);
  const branch = (id: string, label: string, iconId: IconName, spec: WorldProjectionSpec, children: readonly WorldProjectionTemplateDescriptor[]) => worldProjectionTemplateBranch(controllerId, command, id, label, iconId, spec, children);

  const orthographic = leaf("orthographic", "Orthographic", "projection-orthographic", worldProjectionDefaults("orthographic"));

  const axonometricVariants: readonly [WorldAxonometricVariant, string, IconName][] = [
    ["isometric", "Isometric", "projection-isometric"],
    ["dimetric", "Dimetric", "projection-dimetric"],
    ["trimetric", "Trimetric", "projection-trimetric"],
  ];
  const axonometric = branch(
    "axonometric",
    "Axonometric",
    "projection-axonometric",
    worldProjectionDefaults("axonometric"),
    axonometricVariants.map(([variant, label, iconId]) =>
      leaf(`axonometric-${variant}`, label, iconId, {
        mode: { kind: "axonometric", variant, angleA: variant === "trimetric" ? 12 : 15, angleB: variant === "trimetric" ? 42 : variant === "isometric" ? 30 : 15 },
        orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" },
      }),
    ),
  );

  const obliqueVariants: readonly [WorldObliqueVariant, string, IconName][] = [
    ["cabinet", "Cabinet", "projection-oblique-cabinet"],
    ["cavalier", "Cavalier", "projection-oblique-cavalier"],
    ["military", "Military", "projection-oblique-military"],
  ];
  const oblique = branch(
    "oblique",
    "Oblique",
    "projection-oblique",
    worldProjectionDefaults("oblique"),
    obliqueVariants.map(([variant, label, iconId]) =>
      leaf(`oblique-${variant}`, label, iconId, {
        mode: { kind: "oblique", variant, angle: 45, depthScale: variant === "cabinet" ? 0.5 : 1 },
        orientation: { type: "cardinal", view: variant === "military" ? "plan" : "front" },
      }),
    ),
  );

  const parallel = branch("parallel", "Parallel", "projection-parallel", worldProjectionDefaults("orthographic"), [orthographic, axonometric, oblique]);

  const perspective = branch("perspective", "Perspective", "projection-perspective", worldProjectionDefaults("threePoint"), [
    leaf("one-point", "1-Point", "projection-one-point", worldProjectionDefaults("onePoint")),
    leaf("two-point", "2-Point", "projection-two-point", worldProjectionDefaults("twoPoint")),
    leaf("three-point", "3-Point", "projection-three-point", worldProjectionDefaults("threePoint")),
    leaf("curvilinear", "Curvilinear", "projection-curvilinear", worldProjectionDefaults("curvilinear")),
  ]);

  return [parallel, perspective];
}

const WORLD_PROJECTION_TEMPLATE_PREFIX = "world-projection:";

/** 🪟️ Encodes a {@link WorldProjectionSpec} into a `WindowTemplateDropPayload.templateId` string,
 * for the Display "Windows" drag palette to seed a freshly-opened pane's initial camera. */
export function encodeWorldProjectionTemplateId(spec: WorldProjectionSpec): string {
  return `${WORLD_PROJECTION_TEMPLATE_PREFIX}${JSON.stringify(spec)}`;
}

/** 🪟️ Inverse of {@link encodeWorldProjectionTemplateId}; `null` for anything else (including `undefined`). */
export function decodeWorldProjectionTemplateId(templateId: string | undefined): WorldProjectionSpec | null {
  if (!templateId || !templateId.startsWith(WORLD_PROJECTION_TEMPLATE_PREFIX)) {
    return null;
  }
  try {
    const parsed = JSON.parse(templateId.slice(WORLD_PROJECTION_TEMPLATE_PREFIX.length)) as { readonly mode?: { readonly kind?: unknown }; readonly orientation?: unknown };
    return typeof parsed.mode?.kind === "string" && parsed.orientation ? (parsed as WorldProjectionSpec) : null;
  } catch {
    return null;
  }
}

const WORLD_ORTHOGRAPHIC_VIEW_LABELS: Record<WorldOrthographicViewId, string> = { plan: "Plan", top: "Top", bottom: "Bottom", front: "Front", back: "Back", left: "Left", right: "Right" };
const WORLD_AXONOMETRIC_VARIANT_LABELS: Record<WorldAxonometricVariant, string> = { isometric: "Isometric", dimetric: "Dimetric", trimetric: "Trimetric" };
const WORLD_OBLIQUE_VARIANT_LABELS: Record<WorldObliqueVariant, string> = { cabinet: "Cabinet", cavalier: "Cavalier", military: "Military" };

/** 🧭️ Short label for a gizmo orientation. */
export function worldProjectionOrientationLabel(orientation: WorldProjectionOrientation): string {
  switch (orientation.type) {
    case "cardinal":
      return WORLD_ORTHOGRAPHIC_VIEW_LABELS[orientation.view];
    case "corner": {
      const q = orientation.quadrant.toUpperCase();
      return (orientation.hemisphere ?? "upper") === "lower" ? `${q}↓` : q;
    }
    case "free":
      return "3D";
  }
}

/** 📐️ Mode label matching {@link createWorldProjectionTemplates} leaves. */
export function worldProjectionModeLabel(mode: WorldProjectionMode): string {
  switch (mode.kind) {
    case "orthographic":
      return "Orthographic";
    case "axonometric":
      return WORLD_AXONOMETRIC_VARIANT_LABELS[mode.variant];
    case "oblique":
      return WORLD_OBLIQUE_VARIANT_LABELS[mode.variant];
    case "onePoint":
      return "1-Point";
    case "twoPoint":
      return "2-Point";
    case "threePoint":
      return "3-Point";
    case "curvilinear":
      return "Curvilinear";
  }
}

/** 🪟️ Window title for a live projection — matches {@link createWorldProjectionTemplates} mode labels. */
export function worldProjectionSpecLabel(spec: WorldProjectionSpec): string {
  return worldProjectionModeLabel(spec.mode);
}

/** 🖼️ Catalog icon for a live projection — matches {@link createWorldProjectionTemplates} icons. */
export function worldProjectionSpecIconId(spec: WorldProjectionSpec): string {
  switch (spec.mode.kind) {
    case "orthographic":
      return "projection-orthographic";
    case "axonometric":
      switch (spec.mode.variant) {
        case "isometric":
          return "projection-isometric";
        case "dimetric":
          return "projection-dimetric";
        case "trimetric":
          return "projection-trimetric";
      }
      break;
    case "oblique":
      switch (spec.mode.variant) {
        case "cabinet":
          return "projection-oblique-cabinet";
        case "cavalier":
          return "projection-oblique-cavalier";
        case "military":
          return "projection-oblique-military";
      }
      break;
    case "onePoint":
      return "projection-one-point";
    case "twoPoint":
      return "projection-two-point";
    case "threePoint":
      return "projection-three-point";
    case "curvilinear":
      return "projection-curvilinear";
  }
}
// #endregion 📐️WorldProjection
