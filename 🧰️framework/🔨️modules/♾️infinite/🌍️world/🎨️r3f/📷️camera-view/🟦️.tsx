/** 📷️ The orbit camera view model: view ids, directions, templates and layouts, camera state accessors and application, the snap gate context, the orbit far-plane constants and the camera view rig.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | r3f}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { reactHostPort } from "@semio-tech/ui-react";
import { type MutableRefObject, type ReactElement, type ReactNode } from "react";
import {
  type Camera,
  Matrix4,
  OrthographicCamera,
  PerspectiveCamera,
  useThree,
  Vector3
} from "../🔌️adapters/🟦️.ts";
import { type WorldProjectionSpec } from "../🔭️scene-types/🟦️.ts";
import {
  cadVec3ToThree,
  type OrbitCameraProjection,
  threeVec3ToCad,
  type Vec3,
  type WorldCameraState
} from "../🧷️r3f-foundation/🟦️.ts";

// #region 📶️Lod
export const WORLD_ORBIT_CAMERA_MIN_FAR = 524_288;
export const WORLD_ORBIT_CAMERA_FAR_DISTANCE_FACTOR = 1024;

/** 📷️ Quantized far clipping distance that follows arbitrarily large orbit distances. */
export function adaptiveOrbitCameraFar(distance: number): number {
  const target = Math.max(WORLD_ORBIT_CAMERA_MIN_FAR, Math.max(0, distance) * WORLD_ORBIT_CAMERA_FAR_DISTANCE_FACTOR);
  return 2 ** Math.ceil(Math.log2(target));
}
// #endregion 📶️Lod

// #region 📷️OrbitCameraView
/** 🧭️ Standard orbit orthographic-style view ids for Z-up CAD scenes. */
export type OrbitCameraViewId = "top" | "bottom" | "front" | "back" | "right" | "left" | "north" | "south" | "east" | "west" | "isometricNe" | "isometricNw" | "isometricSe" | "isometricSw" | "perspective" | "twoPointPerspective";

/** 📡️ Command name products should handle to apply {@link OrbitCameraViewId} presets. */
export const ORBIT_CAMERA_VIEW_COMMAND = "setOrbitCameraView";

const ORBIT_CAMERA_VIEW_LABELS: Record<OrbitCameraViewId, string> = {
  top: "Top",
  bottom: "Below",
  front: "Front",
  back: "Back",
  right: "Right",
  left: "Left",
  north: "North",
  south: "South",
  east: "East",
  west: "West",
  isometricNe: "NE",
  isometricNw: "NW",
  isometricSe: "SE",
  isometricSw: "SW",
  perspective: "Perspective",
  twoPointPerspective: "Two Point Perspective",
};

const ORBIT_CAMERA_VIEW_DIRECTION: Record<OrbitCameraViewId, Vec3> = {
  top: [0, 0, 1],
  bottom: [0, 0, -1],
  front: [0, -1, 0],
  back: [0, 1, 0],
  right: [1, 0, 0],
  left: [-1, 0, 0],
  north: [0, 1, 0],
  south: [0, -1, 0],
  east: [1, 0, 0],
  west: [-1, 0, 0],
  isometricNe: [1, 1, 1],
  isometricNw: [-1, 1, 1],
  isometricSe: [1, -1, 1],
  isometricSw: [-1, -1, 1],
  perspective: [0.75, -0.75, 0.55],
  twoPointPerspective: [1, -1, 0],
};

const ORBIT_CAMERA_VIEW_EPSILON = 1e-3;

function vec3Len(v: Vec3): number {
  return Math.hypot(v[0], v[1], v[2]);
}

function vec3Scale(v: Vec3, scale: number): Vec3 {
  return [v[0] * scale, v[1] * scale, v[2] * scale];
}

export function vec3Normalize(v: Vec3): Vec3 {
  const len = vec3Len(v);
  if (len < ORBIT_CAMERA_VIEW_EPSILON) {
    return [0, 0, 1];
  }
  return vec3Scale(v, 1 / len);
}

function vec3Dot(a: Vec3, b: Vec3): number {
  return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

/** 🧭️ Nudges axis-aligned view directions off the orbit up vector to avoid gimbal lock. */
export function stabilizeOrbitViewDirection(direction: Vec3, up: Vec3 = [0, 0, 1]): Vec3 {
  const unit = vec3Normalize(direction);
  if (Math.abs(vec3Dot(unit, up)) < 0.995) {
    return unit;
  }
  return vec3Normalize([unit[0] + ORBIT_CAMERA_VIEW_EPSILON, unit[1] + 0.02, unit[2]]);
}

/** 📏️ Distance from orbit position to target; falls back when degenerate. */
export function orbitCameraDistance(state: WorldCameraState, fallback = 600): number {
  const dx = state.position[0] - state.target[0];
  const dy = state.position[1] - state.target[1];
  const dz = state.position[2] - state.target[2];
  const distance = Math.hypot(dx, dy, dz);
  return distance > ORBIT_CAMERA_VIEW_EPSILON ? distance : fallback;
}

export interface ComputeOrbitCameraViewOptions {
  readonly target: Vec3;
  readonly distance?: number;
  readonly up?: Vec3;
  readonly zoom?: number;
}

export const ORBIT_CAMERA_Z_UP: Vec3 = [0, 0, 1];

/** 📐️ Maps a view id to orthographic vs perspective projection. */
export function orbitCameraProjectionForView(view: OrbitCameraViewId): OrbitCameraProjection {
  return view === "perspective" || view === "twoPointPerspective" ? "perspective" : "orthographic";
}

const ORBIT_CAMERA_VIEW_IDS: readonly OrbitCameraViewId[] = ["top", "bottom", "front", "back", "right", "left", "north", "south", "east", "west", "isometricNe", "isometricNw", "isometricSe", "isometricSw", "perspective", "twoPointPerspective"];

const ORBIT_CAMERA_TEMPLATE_BRANCH_VIEWS: Record<string, OrbitCameraViewId> = {
  orthographic: "top",
  "orthographic-2d": "top",
  "orthographic-3d": "isometricNe",
  isometry: "isometricNe",
};

/** 🪟️ Maps a display-tree window template id to an {@link OrbitCameraViewId}. */
export function resolveOrbitCameraViewFromTemplateId(templateId: string): OrbitCameraViewId | null {
  if ((ORBIT_CAMERA_VIEW_IDS as readonly string[]).includes(templateId)) {
    return templateId as OrbitCameraViewId;
  }
  return ORBIT_CAMERA_TEMPLATE_BRANCH_VIEWS[templateId] ?? null;
}

export function orbitCameraZoomForProjection(projection: OrbitCameraProjection, zoom?: number): number {
  if (projection === "orthographic") {
    return zoom !== undefined && zoom !== 1 ? zoom : 50;
  }
  return zoom ?? 1;
}

/** 🔎️ Duck-typed camera zoom — `instanceof Three*Camera` fails across duplicate `three` package copies. */
export function worldCameraZoom(camera: Camera): number {
  const zoomable = camera as Camera & { readonly isPerspectiveCamera?: boolean; readonly isOrthographicCamera?: boolean; zoom?: number };
  if ((zoomable.isPerspectiveCamera || zoomable.isOrthographicCamera) && typeof zoomable.zoom === "number") {
    return zoomable.zoom;
  }
  return 1;
}

/** 🔎️ Duck-typed orthographic check — same duplicate-`three` constraint as {@link worldCameraZoom}. */
export function worldCameraIsOrthographic(camera: Camera): boolean {
  return (camera as Camera & { readonly isOrthographicCamera?: boolean }).isOrthographicCamera === true;
}

/** 🔎️ Duck-typed live perspective FOV — `undefined` for orthographic cameras, same duplicate-`three` constraint as {@link worldCameraZoom}. */
export function worldCameraFov(camera: Camera): number | undefined {
  const perspective = camera as Camera & { readonly isPerspectiveCamera?: boolean; readonly fov?: number };
  return perspective.isPerspectiveCamera && typeof perspective.fov === "number" ? perspective.fov : undefined;
}

/** 📷️ Computes a Z-up orbit camera state for a named view around `target`. */
export function computeOrbitCameraViewState(view: OrbitCameraViewId, options: ComputeOrbitCameraViewOptions): WorldCameraState {
  const distance = options.distance ?? 600;
  const target = options.target;
  const projection = orbitCameraProjectionForView(view);
  const zoom = orbitCameraZoomForProjection(projection, options.zoom);
  switch (view) {
    case "top":
      return { position: [target[0], target[1], target[2] + distance], target, up: [0, 1, 0], zoom, projection };
    case "bottom":
      return { position: [target[0], target[1], target[2] - distance], target, up: [0, -1, 0], zoom, projection };
    case "front":
    case "south":
      return { position: [target[0], target[1] - distance, target[2]], target, up: ORBIT_CAMERA_Z_UP, zoom, projection };
    case "back":
    case "north":
      return { position: [target[0], target[1] + distance, target[2]], target, up: ORBIT_CAMERA_Z_UP, zoom, projection };
    case "right":
    case "east":
      return { position: [target[0] + distance, target[1], target[2]], target, up: ORBIT_CAMERA_Z_UP, zoom, projection };
    case "left":
    case "west":
      return { position: [target[0] - distance, target[1], target[2]], target, up: ORBIT_CAMERA_Z_UP, zoom, projection };
    default: {
      const direction = vec3Normalize(ORBIT_CAMERA_VIEW_DIRECTION[view]);
      return {
        position: [target[0] + direction[0] * distance, target[1] + direction[1] * distance, target[2] + direction[2] * distance],
        target,
        up: ORBIT_CAMERA_Z_UP,
        zoom,
        projection,
      };
    }
  }
}

export type OrbitControlsTarget = { readonly target: Vector3; update: () => void };

function applyWorldCameraState(camera: Camera, state: WorldCameraState, controls: OrbitControlsTarget | null): void {
  const position = cadVec3ToThree(state.position);
  const target = cadVec3ToThree(state.target);
  camera.position.set(position[0], position[1], position[2]);
  const up = cadVec3ToThree(state.up ?? ORBIT_CAMERA_Z_UP);
  camera.up.set(up[0], up[1], up[2]);
  camera.lookAt(target[0], target[1], target[2]);
  if (controls?.target) {
    controls.target.set(target[0], target[1], target[2]);
    controls.update();
  }
  const zoomable = camera as Camera & { readonly isPerspectiveCamera?: boolean; readonly isOrthographicCamera?: boolean; zoom: number; updateProjectionMatrix: () => void };
  if (zoomable.isPerspectiveCamera || zoomable.isOrthographicCamera) {
    zoomable.zoom = state.zoom;
    zoomable.updateProjectionMatrix();
  }
}

/** 📐️ Keeps orbit pose while swapping orthographic vs perspective zoom defaults. */
export function applyOrbitProjectionToCameraState(state: WorldCameraState, projection: OrbitCameraProjection): WorldCameraState {
  return { ...state, projection, zoom: orbitCameraZoomForProjection(projection, state.zoom) };
}

/** 🎞️ Live projection-matrix morph driven by {@link WorldProjectionSnapDriver} — read by {@link WorldProjectionMatrixDriver} and
 * {@link WorldCurvilinearPass} so their per-frame effects ramp with the same tween instead of popping on remount. */
export interface WorldProjectionMorphState {
  readonly fromSpec: WorldProjectionSpec | undefined;
  readonly toSpec: WorldProjectionSpec;
  readonly fromMatrix: Matrix4;
  readonly fromFov: number;
  readonly toFov: number;
  eased: number;
  holding: boolean;
}

const WORLD_PROJECTION_MORPH_FALLBACK_REF: MutableRefObject<WorldProjectionMorphState | null> = { current: null };

export interface WorldOrbitViewSnapGateContextValue {
  readonly snapGate: boolean;
  readonly setSnapGate: (active: boolean) => void;
  readonly morphRef: MutableRefObject<WorldProjectionMorphState | null>;
}

const WorldOrbitViewSnapGateContext = reactHostPort.createContext<WorldOrbitViewSnapGateContextValue | null>(null);

/** 🧭️ Enables {@link useWorldOrbitViewSnapGate} for orbit controls during gizmo view snaps. */
export function WorldOrbitViewSnapGateProvider(props: { readonly children?: ReactNode }): ReactElement {
  const [snapGate, setSnapGate] = reactHostPort.useState(false);
  const morphRef = reactHostPort.useRef<WorldProjectionMorphState | null>(null);
  const value = reactHostPort.useMemo(() => ({ snapGate, setSnapGate, morphRef }), [snapGate, morphRef]);
  return <WorldOrbitViewSnapGateContext.Provider value={value}>{props.children}</WorldOrbitViewSnapGateContext.Provider>;
}

/** 🧭️ Reads the active gizmo view-snap gate for {@link WorldOrbitGated}. */
export function useWorldOrbitViewSnapGate(): WorldOrbitViewSnapGateContextValue {
  return reactHostPort.useContext(WorldOrbitViewSnapGateContext) ?? { snapGate: false, setSnapGate: () => {}, morphRef: WORLD_PROJECTION_MORPH_FALLBACK_REF };
}

export interface OrbitCameraViewTemplateDescriptor {
  readonly id: string;
  readonly label: string;
  readonly controllerId: string;
  readonly command: string;
  readonly args: { readonly view: OrbitCameraViewId };
  readonly children?: readonly OrbitCameraViewTemplateDescriptor[];
}

export interface CreateOrbitCameraViewTemplatesConfig {
  readonly controllerId: string;
  readonly command?: string;
  readonly views?: readonly OrbitCameraViewId[];
}

function orbitCameraViewTemplateLeaf(controllerId: string, command: string, view: OrbitCameraViewId, id = view, label = ORBIT_CAMERA_VIEW_LABELS[view]): OrbitCameraViewTemplateDescriptor {
  return { id, label, controllerId, command, args: { view } };
}

function orbitCameraViewTemplateBranch(controllerId: string, command: string, id: string, label: string, view: OrbitCameraViewId, children: readonly OrbitCameraViewTemplateDescriptor[]): OrbitCameraViewTemplateDescriptor {
  return { id, label, controllerId, command, args: { view }, children };
}

/** 🪟️ Builds the orthographic/perspective window-template tree for the display panel. */
export function createOrbitCameraViewTemplates(config: CreateOrbitCameraViewTemplatesConfig): readonly OrbitCameraViewTemplateDescriptor[] {
  const command = config.command ?? ORBIT_CAMERA_VIEW_COMMAND;
  if (config.views) {
    return config.views.map((view) => orbitCameraViewTemplateLeaf(config.controllerId, command, view));
  }
  const orthographic2d: OrbitCameraViewTemplateDescriptor[] = (["top", "bottom", "front", "back", "right", "left"] as const).map((view) => orbitCameraViewTemplateLeaf(config.controllerId, command, view));
  const isometryCorners: OrbitCameraViewTemplateDescriptor[] = (["isometricNe", "isometricNw", "isometricSe", "isometricSw"] as const).map((view) => orbitCameraViewTemplateLeaf(config.controllerId, command, view));
  return [
    orbitCameraViewTemplateBranch(config.controllerId, command, "orthographic", "Orthographic", "top", [
      orbitCameraViewTemplateBranch(config.controllerId, command, "orthographic-2d", "2D", "top", orthographic2d),
      orbitCameraViewTemplateBranch(config.controllerId, command, "orthographic-3d", "3D", "isometricNe", [orbitCameraViewTemplateBranch(config.controllerId, command, "isometry", "Isometry", "isometricNe", isometryCorners)]),
    ]),
    orbitCameraViewTemplateBranch(config.controllerId, command, "perspective", "Perspective", "perspective", [orbitCameraViewTemplateLeaf(config.controllerId, command, "twoPointPerspective")]),
  ];
}

export interface OrbitCameraViewLayoutPane {
  readonly view: OrbitCameraViewId;
  readonly title?: string;
  readonly size?: number;
}

export type OrbitCameraViewLayoutArrangement =
  | { readonly kind: "stack"; readonly panes: readonly [OrbitCameraViewLayoutPane] }
  | { readonly kind: "row"; readonly panes: readonly OrbitCameraViewLayoutPane[] }
  | { readonly kind: "column"; readonly panes: readonly OrbitCameraViewLayoutPane[] }
  | { readonly kind: "grid"; readonly rows: readonly { readonly size?: number; readonly panes: readonly OrbitCameraViewLayoutPane[] }[] };

export interface OrbitCameraViewLayoutDescriptor {
  readonly id: string;
  readonly label: string;
  readonly groupPath: readonly string[];
  readonly arrangement: OrbitCameraViewLayoutArrangement;
}

function orbitLayoutPane(view: OrbitCameraViewId, title?: string, size?: number): OrbitCameraViewLayoutPane {
  return { view, ...(title ? { title } : {}), ...(size !== undefined ? { size } : {}) };
}

function orbitLayoutSingle(view: OrbitCameraViewId, group: string): OrbitCameraViewLayoutDescriptor {
  return {
    id: `view-single-${view}`,
    label: ORBIT_CAMERA_VIEW_LABELS[view],
    groupPath: ["Single", group],
    arrangement: { kind: "stack", panes: [orbitLayoutPane(view)] },
  };
}

/** 🧭️ Catalog of reusable orbit-view window arrangements for named layouts. */
export function createOrbitCameraViewLayoutDescriptors(): readonly OrbitCameraViewLayoutDescriptor[] {
  const singles2d = (["top", "bottom", "front", "back", "right", "left"] as const).map((view) => orbitLayoutSingle(view, "2D"));
  const singles3d = (["isometricNe", "isometricNw", "isometricSe", "isometricSw", "perspective", "twoPointPerspective"] as const).map((view) => orbitLayoutSingle(view, "3D"));
  const dualRow = (id: string, label: string, group: string, panes: readonly OrbitCameraViewLayoutPane[]): OrbitCameraViewLayoutDescriptor => ({
    id,
    label,
    groupPath: ["Dual", group],
    arrangement: { kind: "row", panes },
  });
  return [
    ...singles2d,
    ...singles3d,
    dualRow("view-dual-top-front", "Top | Front", "Plan + Elevation", [orbitLayoutPane("top"), orbitLayoutPane("front")]),
    dualRow("view-dual-front-right", "Front | Right", "Elevation", [orbitLayoutPane("front"), orbitLayoutPane("right")]),
    dualRow("view-dual-top-ne", "Top | NE", "Plan + Isometry", [orbitLayoutPane("top"), orbitLayoutPane("isometricNe")]),
    dualRow("view-dual-plan-perspective", "Top | Perspective", "Plan + Perspective", [orbitLayoutPane("top", undefined, 100 / 3), orbitLayoutPane("perspective", undefined, 200 / 3)]),
    dualRow("view-dual-front-back", "Front | Back", "Opposite Elevations", [orbitLayoutPane("front"), orbitLayoutPane("back")]),
    dualRow("view-dual-right-left", "Right | Left", "Side Elevations", [orbitLayoutPane("right"), orbitLayoutPane("left")]),
    {
      id: "view-triple-top-front-right",
      label: "Top / Front / Right",
      groupPath: ["Triple", "Standard"],
      arrangement: {
        kind: "column",
        panes: [orbitLayoutPane("top", undefined, 34), orbitLayoutPane("front", undefined, 33), orbitLayoutPane("right", undefined, 33)],
      },
    },
    {
      id: "view-quad-standard",
      label: "Standard",
      groupPath: ["Quad", "Mixed"],
      arrangement: {
        kind: "grid",
        rows: [
          { size: 50, panes: [orbitLayoutPane("top"), orbitLayoutPane("front")] },
          { size: 50, panes: [orbitLayoutPane("right"), orbitLayoutPane("isometricNe")] },
        ],
      },
    },
    {
      id: "view-quad-ortho-faces",
      label: "Ortho Faces",
      groupPath: ["Quad", "2D"],
      arrangement: {
        kind: "grid",
        rows: [
          { size: 50, panes: [orbitLayoutPane("top"), orbitLayoutPane("front")] },
          { size: 50, panes: [orbitLayoutPane("right"), orbitLayoutPane("left")] },
        ],
      },
    },
    {
      id: "view-quad-isometry",
      label: "Isometry Corners",
      groupPath: ["Quad", "3D"],
      arrangement: {
        kind: "grid",
        rows: [
          { size: 50, panes: [orbitLayoutPane("isometricNe"), orbitLayoutPane("isometricNw")] },
          { size: 50, panes: [orbitLayoutPane("isometricSe"), orbitLayoutPane("isometricSw")] },
        ],
      },
    },
    {
      id: "view-2d-six",
      label: "Six Ortho",
      groupPath: ["2D", "Full"],
      arrangement: {
        kind: "grid",
        rows: [
          { size: 34, panes: [orbitLayoutPane("top"), orbitLayoutPane("bottom")] },
          { size: 33, panes: [orbitLayoutPane("front"), orbitLayoutPane("back")] },
          { size: 33, panes: [orbitLayoutPane("right"), orbitLayoutPane("left")] },
        ],
      },
    },
    {
      id: "view-3d-isometry",
      label: "Four Isometry",
      groupPath: ["3D", "Isometry"],
      arrangement: {
        kind: "grid",
        rows: [
          { size: 50, panes: [orbitLayoutPane("isometricNe"), orbitLayoutPane("isometricNw")] },
          { size: 50, panes: [orbitLayoutPane("isometricSe"), orbitLayoutPane("isometricSw")] },
        ],
      },
    },
  ];
}

/** 📷️ Applies an orbit view preset when `seedKey` changes (owned-camera canvases). */
export function WorldOrbitCameraViewApplier(props: { readonly view: OrbitCameraViewId; readonly seedKey: string | number; readonly projectionOverride?: OrbitCameraProjection }): ReactElement {
  const { camera } = useThree();
  const controls = useThree((s) => s.controls as OrbitControlsTarget | null);
  const targetScratch = reactHostPort.useMemo(() => new Vector3(), []);
  const state = reactHostPort.useMemo(() => {
    const target = controls?.target ?? targetScratch.set(0, 0, 0);
    const targetCad = threeVec3ToCad(target);
    const distance = camera ? Math.hypot(camera.position.x - target.x, camera.position.y - target.y, camera.position.z - target.z) || 600 : 600;
    const base = computeOrbitCameraViewState(props.view, { target: targetCad, distance });
    return props.projectionOverride ? applyOrbitProjectionToCameraState(base, props.projectionOverride) : base;
  }, [camera, controls, props.projectionOverride, props.seedKey, props.view, targetScratch]);
  return <WorldOrbitCameraViewRig state={state} seedKey={props.seedKey} />;
}

/** 📷️ Seeds orbit camera + target when `seedKey` changes (fixture presets, display templates). */
export function OrbitCameraViewSeed(props: { readonly camera: Camera | null; readonly state: WorldCameraState; readonly seedKey: string | number }): null {
  const controls = useThree((s) => s.controls as OrbitControlsTarget | null);
  const lastSeedKey = reactHostPort.useRef<string | number | null>(null);
  reactHostPort.useLayoutEffect(() => {
    const camera = props.camera;
    if (!camera || lastSeedKey.current === props.seedKey) {
      return;
    }
    lastSeedKey.current = props.seedKey;
    applyWorldCameraState(camera, props.state, controls);
  }, [controls, props.camera, props.seedKey, props.state.position, props.state.target, props.state.up, props.state.zoom, props.state.projection]);
  return null;
}

/** 📷️ Mounts perspective/orthographic camera + seeds orbit state for display templates. */
export function WorldOrbitCameraViewRig(props: { readonly state: WorldCameraState; readonly seedKey: string | number; readonly onCamera?: (camera: Camera | null) => void; readonly perspectiveFov?: number }): ReactElement {
  const projection = props.state.projection ?? "perspective";
  const up = cadVec3ToThree(props.state.up ?? ORBIT_CAMERA_Z_UP);
  const cameraKey = `${projection}:${props.seedKey}`;
  return (
    <>
      {projection === "orthographic" ? (
        <OrthographicCamera key={cameraKey} ref={props.onCamera} makeDefault up={up} near={0.2} far={WORLD_ORBIT_CAMERA_MIN_FAR} zoom={props.state.zoom} />
      ) : (
        <PerspectiveCamera key={cameraKey} ref={props.onCamera} makeDefault up={up} near={0.2} far={WORLD_ORBIT_CAMERA_MIN_FAR} fov={props.perspectiveFov ?? 50} zoom={props.state.zoom} />
      )}
      <WorldOrbitCameraViewRigSeed state={props.state} seedKey={props.seedKey} />
    </>
  );
}

/** 🔑️ Stable apply token for {@link WorldOrbitCameraViewRigSeed}; keyed by seed + projection, not camera uuid remounts. */
export function orbitCameraViewRigApplyToken(seedKey: string | number, projection: OrbitCameraProjection = "perspective"): string {
  return `${seedKey}:${projection}`;
}

/** 🔑️ Whether a seed, camera, or controls identity needs the owned pose applied. */
export function shouldApplyOrbitCameraViewRigSeed(
  lastToken: string | null,
  nextToken: string,
  lastCamera: object | null,
  nextCamera: object | null,
  lastControls: object | null,
  nextControls: object | null,
): boolean {
  return lastToken !== nextToken || lastCamera !== nextCamera || lastControls !== nextControls;
}

export function WorldOrbitCameraViewRigSeedApply(props: {
  readonly camera: Camera;
  readonly controls: OrbitControlsTarget | null;
  readonly invalidate: () => void;
  readonly state: WorldCameraState;
  readonly seedKey: string | number;
}): null {
  const lastApplyToken = reactHostPort.useRef<string | null>(null);
  const lastCamera = reactHostPort.useRef<Camera | null>(null);
  const lastControls = reactHostPort.useRef<OrbitControlsTarget | null>(null);
  const stateRef = reactHostPort.useRef(props.state);
  stateRef.current = props.state;
  const projection = props.state.projection ?? "perspective";
  reactHostPort.useLayoutEffect(() => {
    const token = orbitCameraViewRigApplyToken(props.seedKey, projection);
    if (!shouldApplyOrbitCameraViewRigSeed(lastApplyToken.current, token, lastCamera.current, props.camera, lastControls.current, props.controls)) {
      return;
    }
    lastApplyToken.current = token;
    lastCamera.current = props.camera;
    lastControls.current = props.controls;
    applyWorldCameraState(props.camera, stateRef.current, props.controls);
    props.invalidate();
  }, [projection, props.camera, props.controls, props.invalidate, props.seedKey]);
  return null;
}

export function WorldOrbitCameraViewRigSeed(props: { readonly state: WorldCameraState; readonly seedKey: string | number }): ReactElement {
  const camera = useThree((s) => s.camera);
  const controls = useThree((s) => s.controls as OrbitControlsTarget | null);
  const invalidate = useThree((s) => s.invalidate);
  return <WorldOrbitCameraViewRigSeedApply camera={camera} controls={controls} invalidate={invalidate} state={props.state} seedKey={props.seedKey} />;
}
// #endregion 📷️OrbitCameraView

// #region 📐️WorldProjection
function halfFovTan(fovDeg: number): number {
  return Math.tan((fovDeg * Math.PI) / 360);
}

/** 📐️ Orthographic zoom (pixels-per-world-unit in a drei pixel frustum) reproducing a perspective camera's apparent
 * scale at `distance` from the target, so a persp→ortho snap doesn't jump to the legacy zoom-50 default. */
export function worldProjectionMatchedOrthoZoom(fovDeg: number, distance: number, viewportHeight: number): number {
  return viewportHeight / (2 * Math.max(distance, ORBIT_CAMERA_VIEW_EPSILON) * halfFovTan(fovDeg));
}

/** 📐️ Perspective orbit distance reproducing an orthographic zoom's apparent scale — inverse of {@link worldProjectionMatchedOrthoZoom}; used by automatic LOD so ortho scroll retunes the grid. */
export function worldProjectionMatchedPerspectiveDistance(fovDeg: number, zoom: number, viewportHeight: number): number {
  return Math.max(viewportHeight, 1) / (2 * Math.max(zoom, 1e-6) * halfFovTan(fovDeg));
}
// #endregion 📐️WorldProjection
