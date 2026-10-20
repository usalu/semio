/** 🔭️ World-scene record types and the pure parsers behind them: camera state, framing and auto-fit keys, environment, frame and fit records, and pending projection camera seeding.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | World3dHost}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { sunPositionFromAzimuthElevation, type GumballConfig, type SelectionMarqueeMethod } from "@semio-tech/ui-react";
import { parseViewport3dProjectionFramePolicy, type ContextMenuItemSpec, type MergeMode, type Viewport3dProjectionFramePolicy } from "@semio-tech/framework";
// #region 🔌️Adapters
import {
  computeWorldProjectionPose,
  frameWorldProjectionPose,
  worldProjectionFamily,
  worldProjectionModeFov,
  worldSceneContentBounds,
  type WorldCameraState,
  type WorldProjectionSpec
} from "@semio-tech/framework/infinite/world/r3f";
// #endregion 🔌️Adapters

//#region 🔖️World3dHost
//#region WorldSceneParsing
export type WorldMeshData = {
  readonly positions: readonly number[];
  readonly normals: readonly number[];
  readonly indices: readonly number[];
  /** 🎨️ Canonical per-vertex RGBA in linear color space. */
  readonly colors?: readonly number[];
  readonly uvs?: readonly number[];
  readonly faceIds?: readonly number[];
  readonly vertexIds?: readonly number[];
  readonly edgePositions?: readonly number[];
  readonly edgeIds?: readonly number[];
  readonly componentReferences?: Readonly<Record<string, readonly string[]>>;
  readonly paintTextureBase64?: string;
  readonly attributes?: Record<string, WorldMeshAttribute>;
  readonly materials?: Record<string, Record<string, unknown>>;
  readonly textures?: Record<string, { readonly mime: string; readonly bytes: readonly number[] }>;
  readonly edgeUvs?: readonly number[];
  readonly edgeIsSeam?: readonly number[];
  /** 🌡️ Set by the host on the mesh a scalar field paints: its colours are an analysis heatmap and survive hover and selection paint. */
  readonly heatmap?: true;
};

export type WorldMeshAttribute = { readonly domain: "vertex" | "corner" | "face" | "edge"; readonly semantic: "normal" | "uv" | "color" | "material" | "custom"; readonly interpolation: "linear" | "nearest" | "constant"; readonly values: readonly unknown[]; readonly indices?: readonly number[] };

type WorldCameraRecord = {
  readonly position?: readonly [number, number, number];
  readonly target?: readonly [number, number, number];
  readonly fov?: number;
  readonly x?: number;
  readonly y?: number;
  readonly z?: number;
  readonly projectionFrame?: Viewport3dProjectionFramePolicy;
};

export type WorldMeshRecord = {
  readonly id: string;
  readonly data?: WorldMeshData;
  readonly url?: string;
  /** 🥽️ Built-in procedural mesh kind (`box`, `vortex-marker`, `torus`, …) — a REFERENCE this host resolves
   * itself through {@link meshDataFromKind}. Scene payloads never carry a built-in kind's tessellation:
   * `vortex-marker` alone cost ~26 KiB of the 32 KiB fixed surface payload on every refresh. */
  readonly kind?: string;
};

export type WorldInstanceRecord = {
  readonly id: string;
  readonly meshId?: string;
  readonly position?: readonly [number, number, number];
  readonly rotation?: readonly [number, number, number, number];
  readonly scale?: readonly [number, number, number];
  readonly x?: number;
  readonly y?: number;
  readonly z?: number;
  readonly selected?: boolean;
  readonly hovered?: boolean;
  /** 🎨️ Guest-stamped highlight (e.g. an object the open time-travel draft references) — resolves to the secondary "highlighted" mesh style, ORed with the host's own catalog-kind hover highlight. */
  readonly highlighted?: boolean;
  /** 🎨️ Non-interactive/locked state — resolves to the muted "disabled" mesh style at reduced opacity. */
  readonly disabled?: boolean;
  /** 🟩️ Placed by a running tool and not yet finalized — resolves to the "provisional" mesh style and is
   * never pickable. Also derived from `ToolRunProvisionalIdsContext`. */
  readonly provisional?: boolean;
  readonly smoothShading?: boolean;
  readonly objectKind?: string;
  /** 🎯️ The framework interaction target this instance stands for, when it differs from `id`.
   * `id` must stay unique per rendered instance, but an app's interaction TOPOLOGY may only declare
   * a coarser target (procedural3d renders one instance per geometry item, `{node}@{port}#{index}`,
   * while its topology declares the port `{node}@{port}`) — and `validate_state` prunes any
   * hover/selection id absent from the topology. Dispatches on a `domainId`-bound world window use
   * this when set, falling back to `id`. */
  readonly interactionId?: string;
  readonly componentSource?: { readonly handle: string; readonly revision: string };
  /** 🎯️ The granularity the instance's interaction target lives under, when it differs from the
   * scene's `domainGranularityId` — a fem3d scene draws nodes, members, solids, supports and load
   * glyphs as one instance lane under one domain, and each row names its own granularity so a pick
   * on a member selects an `element`, never a `node`. Falls back to the scene's default. */
  readonly interactionGranularityId?: string;
};

export type WorldSelectionTargets = {
  readonly mesh?: boolean;
  readonly vertex?: boolean;
  readonly edge?: boolean;
  readonly face?: boolean;
  /** 🎯️ Set by the pick filter: a sub-element miss never falls back to hovering or picking the whole shape. */
  readonly exclusive?: boolean;
};

export type WorldHoverComponent = {
  readonly objectId?: string;
  readonly mode?: string;
  readonly id?: number;
};

type WorldContextMenuItem = ContextMenuItemSpec;

export type WorldSelectionRecord = {
  readonly method?: SelectionMarqueeMethod;
  readonly selectionMergeMode?: MergeMode;
  readonly ids?: readonly string[];
  readonly hoveredId?: string | null;
  readonly referenceSelectedId?: string;
  readonly targetVolumeIds?: readonly string[];
  readonly granularity?: string;
  readonly selectionMode?: string;
  readonly activeObjectId?: string | null;
  readonly componentIds?: readonly number[];
  readonly targets?: WorldSelectionTargets;
  readonly transformMode?: string;
  readonly interactionMode?: "model" | "paint";
  readonly gumballTarget?: readonly [number, number, number];
  readonly gumballActive?: boolean;
  readonly gumballSelectionIds?: readonly string[];
  /** 🎛️ Plugin-authored gumball handle flags (e.g. puzzle3d Move/Rotate). When set, overrides {@link gumballConfigForTransformMode}. */
  readonly gumballConfig?: GumballConfig;
  /** ⚡️ When true the gumball dispatches `translateSelection`/`rotateSelection`/`scaleSelection`
   * INCREMENTALLY while the pointer moves (latest pose wins, one round trip in flight at a time) as
   * `phase: "stream"` ticks of ONE open tool transaction, and the host draws the app's answer instead of
   * a local instance preview — the app previews the open transaction, so every dependent view (a fem3d
   * results window re-solving under the moved node) tracks the gesture live. Drag end sends the remaining
   * delta as `phase: "commit"`; blur or unmount mid-drag sends `phase: "abort"` with its `reason`. Without
   * it the host previews locally and dispatches ONE net delta on release (no `phase`: a one-shot transaction). */
  readonly gumballLiveDispatch?: boolean;
  readonly hoveredComponent?: WorldHoverComponent;
  readonly showEdges?: boolean;
  readonly engagementSessionActive?: boolean;
  /** 🖱️➡️ When true and `targets.face` is set, dragging an already-selected face starts a push/pull gesture (`worldFaceDragEnd` on release) instead of the default marquee/orbit. */
  readonly faceDragActive?: boolean;
  readonly hoveredKindId?: string;
};

type WorldSuggestionCandidateRecord = {
  readonly index: number;
  /** 🔦️ The candidate's key in the suggestion run's `toolRunTrace` lane — hovering its row focuses exactly that record. */
  readonly key?: number;
  readonly objectLabel: string;
  readonly vortexLabel: string;
  readonly icon?: string;
  readonly color?: string;
};

export type WorldSuggestionMenuRecord = {
  readonly open: boolean;
  /** 📂️ The list lives inside the regular context menu as the "suggest" row's submenu instead of floating as its own popup. */
  readonly submenu?: boolean;
  readonly x: number;
  readonly y: number;
  readonly windowId?: string;
  readonly vortexFullId?: string;
  readonly pending: boolean;
  readonly candidates: readonly WorldSuggestionCandidateRecord[];
};

export type WorldInteractionRecord = {
  readonly activeUtility?: string;
  readonly brushCandidateIndex?: number;
  readonly hoveredVortexFullId?: string;
  readonly voxelDims?: readonly [number, number, number];
  readonly gridFactor?: number;
  readonly suggestionMenu?: WorldSuggestionMenuRecord | null;
  /** 🔢️ The guest's monotone brush-mesh install counter. Climbs inside one guest instantiation and
   * starts at zero in a fresh one, so a value below the last one this page read proves the guest was
   * restarted and holds nothing this page uploaded — see {@link Puzzle3dBrushMeshRegistry}. */
  readonly meshResidency?: number;
  /** 🚚️ Mesh ids the guest was announced by id alone and cannot serve; it is asking for the bytes. */
  readonly meshReuploadUrls?: readonly string[];
};

export type WorldLodRecord = {
  readonly gridFactor?: number;
  readonly gridSnapEnabled?: boolean;
  readonly showLodGrid?: boolean;
  readonly automaticLod?: boolean;
  readonly depthVariableLod?: boolean;
  readonly manualLod?: number;
};

export type WorldVortexRecord = World3dMarkerInteractionFields & {
  readonly fullId: string;
  readonly objectId?: string;
  readonly vortexKind?: string;
  readonly position: readonly [number, number, number];
  readonly direction?: readonly [number, number, number];
  readonly displayDirection?: "outwards" | "inwards";
  readonly radius?: number;
  readonly color?: string;
  readonly selected?: boolean;
  readonly hovered?: boolean;
};

export type WorldAttractionRecord = World3dMarkerInteractionFields & {
  readonly id: string;
  readonly from: readonly [number, number, number];
  readonly to: readonly [number, number, number];
  readonly color?: string;
};

export type WorldTargetVolumeRecord = World3dMarkerInteractionFields & {
  readonly id: string;
  readonly origin: readonly [number, number, number];
  readonly orientation?: readonly [number, number, number, number];
  readonly scale?: readonly [number, number, number] | number;
  readonly color?: string;
  readonly hidden?: boolean;
  readonly locked?: boolean;
  readonly selected?: boolean;
};

export type WorldReferenceRecord = World3dMarkerInteractionFields & {
  readonly id: string;
  readonly url: string;
  readonly origin: readonly [number, number, number];
  readonly widthWorld?: number;
  readonly locked?: boolean;
  readonly hidden?: boolean;
  readonly opacity?: number;
};

/** ☁️ One point-cloud rendering layer (`World3dScene.pointsJson` entries) — the cheap path for
 * 10^5-10^6 points, distinct from per-point meshes. `positionsB64` is base64 of little-endian f32 xyz
 * interleaved; `colorsB64` (optional) is base64 of u8 rgb interleaved, one triplet per point. */
export type WorldPointCloudLayerRecord = {
  readonly id: string;
  readonly positionsB64: string;
  readonly colorsB64?: string;
  readonly size: number;
  readonly sizeAttenuation: boolean;
};

type WorldEngagementPreviewPoint = {
  readonly kind: "point";
  readonly role?: string;
  readonly position: readonly [number, number, number];
};

type WorldEngagementPreviewSegment = {
  readonly kind: "segment";
  readonly role?: string;
  readonly from: readonly [number, number, number];
  readonly to: readonly [number, number, number];
};

type WorldEngagementPreviewBox = {
  readonly kind: "box-preview";
  readonly role?: string;
  readonly cornerA?: readonly [number, number, number];
  readonly cornerB?: readonly [number, number, number];
  readonly height?: number;
};

type WorldEngagementPreviewLinearHandle = {
  readonly kind: "linear-handle";
  readonly role?: string;
  readonly axis: readonly [number, number, number];
  readonly origin: readonly [number, number, number];
};

export type WorldEngagementPreviewItem = WorldEngagementPreviewPoint | WorldEngagementPreviewSegment | WorldEngagementPreviewBox | WorldEngagementPreviewLinearHandle;

export type WorldParsedCameraState = WorldCameraState & { readonly fov: number; readonly explicitProjection: boolean; readonly projectionFrame: Viewport3dProjectionFramePolicy };

/** 📐️ A `camera_json.projection` field is the composed mode ⊗ orientation taxonomy object. */
function parseWorldProjectionField(value: unknown): WorldProjectionSpec | undefined {
  if (!value || typeof value !== "object") {
    return undefined;
  }
  const mode = (value as { mode?: { kind?: unknown } }).mode;
  if (typeof mode?.kind !== "string") {
    return undefined;
  }
  return value as WorldProjectionSpec;
}

export function parseCameraState(cameraJson: string): WorldParsedCameraState {
  try {
    const parsed = JSON.parse(cameraJson) as WorldCameraRecord & { target?: readonly [number, number, number]; zoom?: number; up?: readonly [number, number, number]; projection?: string | object };
    const position: [number, number, number] = parsed.position ? [parsed.position[0], parsed.position[1], parsed.position[2]] : [parsed.x ?? 4, parsed.y ?? -4, parsed.z ?? 3];
    const target: [number, number, number] = parsed.target ? [parsed.target[0], parsed.target[1], parsed.target[2]] : [0, 0, 0];
    const projectionSpec = parseWorldProjectionField(parsed.projection);
    const explicitProjection = projectionSpec !== undefined || parsed.projection === "perspective" || parsed.projection === "orthographic";
    const projectionFamily = projectionSpec ? worldProjectionFamily(projectionSpec) : undefined;
    return {
      position,
      target,
      up: parsed.up ? [parsed.up[0], parsed.up[1], parsed.up[2]] : undefined,
      zoom: typeof parsed.zoom === "number" ? parsed.zoom : 1,
      projection: projectionFamily ? (projectionFamily === "parallel" ? "orthographic" : "perspective") : parsed.projection === "orthographic" ? "orthographic" : "perspective",
      projectionSpec,
      fov: parsed.fov ?? (projectionSpec ? worldProjectionModeFov(projectionSpec) : undefined) ?? 45,
      explicitProjection,
      projectionFrame: parsed.projectionFrame === undefined ? "content" : parseViewport3dProjectionFramePolicy(parsed.projectionFrame),
    };
  } catch {
    return { position: [4, -4, 3], target: [0, 0, 0], zoom: 1, projection: "perspective", fov: 45, explicitProjection: false, projectionFrame: "content" };
  }
}

//#region WorldViewportCamera
/** 📷️ Merges an orbit/projection report into the viewport-owned camera without losing FOV, explicit-projection flags, or the full projection spec (orbit reports only ever carry the binary family, never the taxonomy spec). */
export function mergeWorldViewportCamera(base: WorldParsedCameraState, next: WorldCameraState): WorldParsedCameraState {
  return {
    position: next.position,
    target: next.target,
    zoom: next.zoom,
    up: next.up ?? base.up,
    projection: next.projection ?? base.projection,
    projectionSpec: next.projectionSpec ?? base.projectionSpec,
    fov: base.fov,
    explicitProjection: base.explicitProjection || next.projection === "perspective" || next.projection === "orthographic",
    projectionFrame: base.projectionFrame,
  };
}

/** 📷️ Orbit seed: follow `scene.cameraJson` until a programmatic viewport apply bumps `detachEpoch`; orbit-only detach keeps the seed stable. */
export function world3dViewportCameraSeedKey(sceneCameraJson: string, detachEpoch: number): string {
  return detachEpoch === 0 ? sceneCameraJson : `viewport:${detachEpoch}`;
}

/** 📷️ Whether a seeded projection pane may keep auto-fitting {@link worldSceneContentBounds} while a running tool grows the scene. */
export function world3dFitProjectionContent(
  viewportOwned: boolean,
  cameraNavigating: boolean,
  hasProjectionSeed: boolean,
  lockContentFrame: boolean = false,
  projectionFrame: Viewport3dProjectionFramePolicy = "content",
): boolean {
  return projectionFrame === "content" && !viewportOwned && !cameraNavigating && !lockContentFrame && hasProjectionSeed;
}

/** 📷️ Scene instances that may move the projection framing bounds — excludes tool-run provisional placements so fill planning never yanks the camera. */
export function world3dFramingInstances(instances: readonly WorldInstanceRecord[]): readonly WorldInstanceRecord[] {
  return instances.filter((instance) => !instance.provisional);
}

/** 📷️ Whether {@link WorldProjectionContentFrame} should stay mounted — suppressed for the whole orbit/pan/zoom gesture, not only after `end`. */
export function world3dProjectionContentFrameMounted(fitProjectionContent: boolean, projectionFramePending: boolean, cameraNavigating: boolean): boolean {
  return (fitProjectionContent || projectionFramePending) && !cameraNavigating;
}

/** 🎯️ Whether the host may paint the manual frame-visible-instances overlay — always off; framing is
 * {@link WorldAutoFit}, the fit lane, and view commands such as zoom-to-selection. */
export function world3dFrameVisibleOverlayOffered(_fit: Pick<WorldFitRecord, "enabled" | "boundsMin" | "boundsMax"> | null): boolean {
  return false;
}

/** 📷️ Builds the `setCamera` dispatch payload from a viewport camera pose — deliberately omits `projection`
 * (the binary family string, e.g. "orthographic"/"perspective") since the Rust camera struct's `projection`
 * field expects the full taxonomy spec object; forwarding the bare family string there fails deserialization
 * of the whole camera value and silently drops the entire dispatch. */
export function buildWorldCameraDispatchArgs(camera: WorldCameraState): Record<string, unknown> {
  const args: Record<string, unknown> = { position: camera.position, target: camera.target, zoom: camera.zoom };
  if (camera.up) args.up = camera.up;
  return args;
}

/** 📷️ Full `setCamera` action args for a world-3d viewport gesture/gizmo sync — nests {@link buildWorldCameraDispatchArgs}'s
 * pose under `camera` (never spread flat alongside `windowId`) to match the wgpu renderer's own `setCamera` dispatch
 * (`{surfaceId, camera: {...}}`) and every real plugin app's `setCamera` handler, which reads the pose from
 * `args.get("camera")`, not from top-level `position`/`target`/`zoom` keys. */
export function worldCameraSetCameraDispatchArgs(windowId: string, camera: WorldCameraState): Record<string, unknown> {
  return { windowId, camera: buildWorldCameraDispatchArgs(camera) };
}

/** 📷️ Default float-noise tolerance for {@link worldCameraPoseApproxEqual} — a scene camera echoed back
 * through JSON/f64 serialization never comes back bit-identical to what was sent. */
const WORLD_CAMERA_ECHO_EPSILON = 1e-6;

/** 📷️ Approx-equality of two camera poses (position/target/zoom) within `epsilon`, used by
 * {@link shouldReattachWorldViewportCamera} to recognize the plugin echoing back this component's own
 * just-dispatched pose rather than a genuinely external change. */
export function worldCameraPoseApproxEqual(a: WorldCameraState, b: WorldCameraState, epsilon: number = WORLD_CAMERA_ECHO_EPSILON): boolean {
  const vectorClose = (x?: readonly number[], y?: readonly number[]): boolean => {
    if (!x || !y) return x === y;
    if (x.length !== y.length) return false;
    return x.every((value,index)=>{const other=y[index];return other!==undefined&&Math.abs(value-other)<=epsilon;});
  };
  return vectorClose(a.position, b.position) && vectorClose(a.target, b.target) && Math.abs(a.zoom - b.zoom) <= epsilon;
}

/** 📸️ Compact DOM mirror of one camera pose — rounds every component to
 * {@link WORLD_CAMERA_DOM_PRECISION} decimals so an unchanged pose re-renders to a byte-identical string and a
 * headless probe can diff orbit/pan/zoom without a pixel compare. Deliberately pose-only (no projection spec
 * object) to stay cheap enough for a per-render attribute.
 *
 * 🪟️ Two attributes carry it, and the difference is the whole point (ticket
 * 26/09/02/PUZZLE-3D-END-TO-END wave B12): `data-camera-json` mirrors `sceneCamera` — the pose the
 * PROGRAM published on this window instance's own `WindowConfig` lane — so a reader that watches it
 * across an orbit is watching the lane round-trip, not this component's local state. It used to mirror
 * `viewportCamera ?? sceneCamera`, i.e. the local gesture state, which moves for every drag whether or
 * not `setCamera` ever reached the guest — so it could never tell "the camera works" from "the camera
 * lane works". `data-viewport-camera-json` is that local live pose, kept as its own attribute for
 * anything that genuinely wants the rig's current frame. */
export function world3dCameraDomJson(camera: WorldCameraState & { readonly fov?: number }): string {
  const round = (value: number): number => (Number.isFinite(value) ? Number(value.toFixed(WORLD_CAMERA_DOM_PRECISION)) : 0);
  const vector = (values?: readonly number[]): readonly number[] | null => (values ? values.map(round) : null);
  return JSON.stringify({
    position: vector(camera.position),
    target: vector(camera.target),
    up: vector(camera.up),
    zoom: round(camera.zoom),
    fov: typeof camera.fov === "number" ? round(camera.fov) : null,
    projection: camera.projection ?? null,
  });
}

/** 📸️ Decimal places {@link world3dCameraDomJson} rounds to — coarse enough that float noise never churns the
 * attribute, fine enough that a real orbit/pan/zoom gesture always changes it. */
const WORLD_CAMERA_DOM_PRECISION = 4;

/** 🌞️ The LIGHT this surface is lit with, published on the DOM beside the camera it is seen from —
 * exactly the values handed to the scene's `<directionalLight>`, `position` included, so the sun's
 * own `position` is derived here once and read rather than recomputed by every reader.
 *
 * Without this the sun group's four controls were unobservable from outside React: the measures rail's
 * `data-published-value` proves the guest ACCEPTED an azimuth, and nothing at all proved the scene was
 * then lit with it — a slider wired to a config field no light reads would have passed every check
 * there was (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️window-coverage-audit-2026-09-14.md` §5.4).
 * `enabled:false` still publishes the whole record: what a reader needs is which sun the scene is NOT
 * using, not an absent attribute. */
export function world3dSunDomJson(environment: { readonly sun?: { readonly enabled?: boolean; readonly azimuth?: number; readonly elevation?: number; readonly intensity?: number; readonly color?: string } } | null | undefined): string {
  const sun = environment?.sun;
  const azimuth = sun?.azimuth ?? 45;
  const elevation = sun?.elevation ?? 35;
  return JSON.stringify({
    enabled: sun?.enabled === true,
    azimuth,
    elevation,
    intensity: sun?.intensity ?? 0.85,
    color: sun?.color ?? "#ffffff",
    position: sunPositionFromAzimuthElevation(azimuth, elevation).map((value) => Number(value.toFixed(WORLD_CAMERA_DOM_PRECISION))),
  });
}

/** 📷️ True when `scene.cameraJson` changed from outside this viewport (view preset, focus, example load) —
 * false both for a byte-identical string and for one that merely echoes `lastDispatchedCamera` (this
 * component's own just-sent `setCamera` pose, within {@link worldCameraPoseApproxEqual} float-noise
 * tolerance) so a self-echo never visually snaps the orbit controls back to where they already are. */
export function shouldReattachWorldViewportCamera(previousSceneCameraJson: string, nextSceneCameraJson: string, lastDispatchedCamera?: WorldCameraState | null): boolean {
  if (previousSceneCameraJson === nextSceneCameraJson) return false;
  if (lastDispatchedCamera && worldCameraPoseApproxEqual(parseCameraState(nextSceneCameraJson), lastDispatchedCamera)) return false;
  return true;
}
//#endregion WorldViewportCamera

export type WorldEnvironmentMaterialRecord = {
  readonly color?: string;
  readonly metalness?: number;
  readonly roughness?: number;
  readonly emissive?: string;
  readonly emissiveIntensity?: number;
  readonly stroke?: string;
};

export type WorldEnvironmentRecord = {
  readonly background?: string;
  readonly ambient?: { readonly intensity?: number; readonly color?: string };
  readonly sun?: { readonly enabled?: boolean; readonly azimuth?: number; readonly elevation?: number; readonly intensity?: number; readonly color?: string };
  readonly shadow?: { readonly enabled?: boolean; readonly opacity?: number; readonly softness?: number };
  readonly material?: WorldEnvironmentMaterialRecord;
};

type WorldFrameRecord = {
  readonly width: number;
  readonly height: number;
  readonly shape?: string;
  readonly badge?: boolean;
  readonly background?: string;
};

type WorldFitRecord = {
  readonly enabled?: boolean;
  readonly revision?: number;
  readonly padding?: number;
  /** 📦️ The producer's OWN delivered extent (`world3d_fit_json`'s `bounds`). Preferred over the scene
   * graph's `Box3`, which can only measure what this renderer has already uploaded — and which the wgpu
   * renderer has no equivalent of at all, so without it the two renderers frame differently. */
  readonly boundsMin?: readonly number[];
  readonly boundsMax?: readonly number[];
};

function parseJsonRecord<T>(json?: string): T | null {
  if (!json) return null;
  try {
    const parsed = JSON.parse(json) as T | null;
    return typeof parsed === "object" ? parsed : null;
  } catch {
    return null;
  }
}

export const parseEnvironment = (json?: string) => parseJsonRecord<WorldEnvironmentRecord>(json);
export const parseFrame = (json?: string) => parseJsonRecord<WorldFrameRecord>(json);
export const parseFit = (json?: string) => parseJsonRecord<WorldFitRecord>(json);

export function isTransparentWorldBackground(background?: string): boolean {
  return !background || background === "transparent";
}

/** 🎯️ The headroom a framing leaves around the bounds, as a multiple of the EXACT fitting distance —
 * the twin of `WORLD_FRAME_BOUNDS_MARGIN` (`🖱️ui/🎬️scene/📐️math/🎥️camera/🦀️.rs`). */
export const WORLD3D_FRAME_BOUNDS_MARGIN = 1.12;

/** 📐️ How far a perspective eye must stand off a bounding sphere of `radius` for the WHOLE sphere to
 * project inside a `fovDeg`/`aspect` frustum, times `margin`.
 *
 * ⚖️ The binding half-angle is the SMALLER of the vertical and horizontal ones: a wide viewport is
 * limited by its vertical field, a tall one by its horizontal field. Fitting by `radius * constant` —
 * what every ad-hoc rule here used to do — is short by `1 / sin(halfAngle)` ≈ 2.6 at the default 45°
 * field, which is exactly how a converged example arrived clipped at the corner of the preview
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️boot-camera-framing-2026-09-15.md`).
 *
 * @see ../../../../../../🔨️modules/🖱️ui/🎬️scene/📐️math/🎥️camera/🦀️.rs — `frame_distance_for_radius`, the Rust twin. */
export function world3dFrameDistanceForRadius(radius: number, fovDeg: number, aspect: number, margin: number = WORLD3D_FRAME_BOUNDS_MARGIN): number {
  const vertical = Math.min(Math.max((((Number.isFinite(fovDeg) ? fovDeg : 45) * Math.PI) / 180) * 0.5, 0.02), 1.5);
  const horizontal = Math.min(Math.max(Math.atan(Math.tan(vertical) * Math.max(Number.isFinite(aspect) ? aspect : 1, 0.05)), 0.02), 1.5);
  const half = Math.min(vertical, horizontal);
  return Math.max((Math.max(radius, 1e-4) / Math.sin(half)) * Math.max(margin, 1), 0.5);
}

/** 📦️ Bounding-sphere radius of an axis-aligned box — half its diagonal, never half its longest edge:
 * a cube framed by `max(size) / 2` leaves its own corners outside the frustum. */
export function world3dBoundsRadius(minimum: readonly number[], maximum: readonly number[]): number {
  return Math.hypot((maximum[0] ?? 0) - (minimum[0] ?? 0), (maximum[1] ?? 0) - (minimum[1] ?? 0), (maximum[2] ?? 0) - (minimum[2] ?? 0)) * 0.5;
}

export function fitCameraFromBounds(center: readonly [number, number, number], radius: number, camera: WorldParsedCameraState, padding: number, aspect = 1): { position: [number, number, number]; target: [number, number, number]; zoom: number } {
  const distance = world3dFrameDistanceForRadius(radius, camera.fov ?? 45, aspect, padding);
  const dx = camera.position[0] - camera.target[0];
  const dy = camera.position[1] - camera.target[1];
  const dz = camera.position[2] - camera.target[2];
  const length = Math.hypot(dx, dy, dz);
  const nx = length > 1e-6 ? dx / length : 1;
  const ny = length > 1e-6 ? dy / length : -1;
  const nz = length > 1e-6 ? dz / length : 0.85;
  const norm = Math.hypot(nx, ny, nz) || 1;
  return {
    position: [center[0] + (nx / norm) * distance, center[1] + (ny / norm) * distance, center[2] + (nz / norm) * distance],
    target: [center[0], center[1], center[2]],
    zoom: camera.zoom,
  };
}

/** 🎯️ Whether a fit lane owes the viewport a framing right now: the producer's document identity
 * (`revision`) must have moved since the last one applied, OR nothing has been framed for this
 * document yet — and in the latter case a camera the USER has moved is never taken back.
 *
 * ⚖️ `userMoved` is cleared by the caller on a revision change, which is the whole "frame on example
 * switch, never yank a moved camera on re-evaluation" rule in one predicate: a re-delivery of the same
 * document answers the same revision, so only the first delivery of it frames
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️boot-camera-framing-2026-09-15.md`). */
export function world3dAutoFitOwed(appliedKey: string, fitKey: string, userMoved: boolean): boolean {
  if (appliedKey === fitKey) return false;
  return !userMoved;
}

/** 📷️ Key for {@link WorldAutoFit} — document revision and external camera seed, plus producer-published
 * bounds when present. Deliberately omits the scene mesh roster: puzzle 3d's fit revision already tracks
 * document identity, and mesh-list churn (catalogue drop) must not re-arm auto-fit; producers that publish
 * `boundsMin`/`boundsMax` still refit when that extent grows (generation3d preview). */
export function world3dAutoFitKey(
  revision: number,
  sceneCameraAttachJson: string,
  autoFitBounds: readonly [readonly number[], readonly number[]] | null,
): string {
  if (!autoFitBounds) return `${revision}:${sceneCameraAttachJson}`;
  const [minimum, maximum] = autoFitBounds;
  const round = (value: number) => Math.round(value * 1e3) / 1e3;
  return `${revision}:${sceneCameraAttachJson}:${minimum.map(round).join(",")}:${maximum.map(round).join(",")}`;
}

/** 📷️ Frames the orbit camera on a live world AABB while keeping the current look direction.
 * `radius` is the BOUNDING-SPHERE radius ({@link world3dBoundsRadius}), not half the longest edge. */
export function world3dFrameCameraFromBounds(
  center: readonly [number, number, number],
  radius: number,
  camera: WorldParsedCameraState,
  padding = WORLD3D_FRAME_BOUNDS_MARGIN,
  aspect = 1,
): WorldParsedCameraState {
  const fitted = fitCameraFromBounds(center, Math.max(radius, 0.5), camera, padding, aspect);
  return { ...camera, position: fitted.position, target: fitted.target, zoom: fitted.zoom };
}

/** 📷️ Frames the orbit camera on instance centroids so table-scale vortex markers stay hittable. */
function world3dTableScaleInstances(instances: readonly WorldInstanceRecord[]): readonly WorldInstanceRecord[] {
  if (instances.length <= 1) return instances;
  const points = instances.map((instance) => instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0]);
  const mid = (values: number[]) => {
    const sorted = [...values].sort((a, b) => a - b);
    return sorted[Math.floor(sorted.length / 2)] ?? 0;
  };
  const cx = mid(points.map((p) => p[0]));
  const cy = mid(points.map((p) => p[1]));
  const cz = mid(points.map((p) => p[2]));
  const near = instances.filter((_, i) => {
    const p = points[i]!;
    return Math.hypot(p[0] - cx, p[1] - cy, p[2] - cz) <= 3.5;
  });
  return near.length > 0 ? near : instances;
}

export function world3dFrameCameraFromInstances(
  instances: readonly WorldInstanceRecord[],
  camera: WorldParsedCameraState,
  padding = 1.35,
): WorldParsedCameraState {
  const seed = autofitCameraFromInstances(world3dTableScaleInstances(instances));
  const dx = seed.position[0] - seed.target[0];
  const dy = seed.position[1] - seed.target[1];
  const dz = seed.position[2] - seed.target[2];
  const length = Math.hypot(dx, dy, dz) || 1;
  const distance = Math.max(length * (padding / 2.5), 1.4);
  return {
    ...camera,
    position: [seed.target[0] + (dx / length) * distance, seed.target[1] + (dy / length) * distance, seed.target[2] + (dz / length) * distance],
    target: seed.target,
    zoom: camera.zoom,
  };
}

export function autofitCameraFromInstances(instances: readonly WorldInstanceRecord[]): WorldParsedCameraState {
  if (instances.length === 0) {
    return { position: [4, -4, 3], target: [0, 0, 0], zoom: 1, projection: "perspective", fov: 45, explicitProjection: false, projectionFrame: "content" };
  }
  let minX = Number.POSITIVE_INFINITY;
  let minY = Number.POSITIVE_INFINITY;
  let minZ = Number.POSITIVE_INFINITY;
  let maxX = Number.NEGATIVE_INFINITY;
  let maxY = Number.NEGATIVE_INFINITY;
  let maxZ = Number.NEGATIVE_INFINITY;
  for (const instance of instances) {
    const position = instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0];
    minX = Math.min(minX, position[0]);
    minY = Math.min(minY, position[1]);
    minZ = Math.min(minZ, position[2]);
    maxX = Math.max(maxX, position[0]);
    maxY = Math.max(maxY, position[1]);
    maxZ = Math.max(maxZ, position[2]);
  }
  const center: [number, number, number] = [(minX + maxX) / 2, (minY + maxY) / 2, (minZ + maxZ) / 2];
  const span = Math.max(maxX - minX, maxY - minY, maxZ - minZ, 1);
  const distance = span * 2.5;
  return {
    position: [center[0] + distance * 0.7, center[1] - distance * 0.7, center[2] + distance * 0.45],
    target: center,
    zoom: 1,
    projection: "perspective",
    fov: 45,
    explicitProjection: false,
    projectionFrame: "content",
  };
}

/** 📷️ Seeds a pending display-template projection, framing visible references/instances when present. */
export function seedPendingWorldProjectionCamera(
  pendingSpec: WorldProjectionSpec,
  sceneCamera: WorldParsedCameraState,
  instances: readonly WorldInstanceRecord[],
  references: readonly WorldReferenceRecord[],
  viewport?: { readonly width: number; readonly height: number },
): WorldParsedCameraState {
  const bounds = worldSceneContentBounds(world3dFramingInstances(instances), references);
  const pose = bounds
    ? frameWorldProjectionPose(pendingSpec, bounds, {
        viewportWidth: viewport?.width,
        viewportHeight: viewport?.height,
      })
    : computeWorldProjectionPose(pendingSpec, {
        target: sceneCamera.target,
        distance: Math.hypot(sceneCamera.position[0] - sceneCamera.target[0], sceneCamera.position[1] - sceneCamera.target[1], sceneCamera.position[2] - sceneCamera.target[2]) || 600,
      });
  return { ...pose, fov: worldProjectionModeFov(pendingSpec) ?? sceneCamera.fov, explicitProjection: true, projectionFrame: sceneCamera.projectionFrame };
}
//#endregion WorldSceneParsing

//#region 🎯️WorldInteractionDomain
/** 🧿️ What every pickable marker record may carry to redirect its own hit — both read straight off
 * the scene JSON, so which interaction target a marker stands for is the plugin's statement, never
 * the host's guess. */
export type World3dMarkerInteractionFields = {
  readonly interactionId?: string;
  readonly interactionGranularityId?: string;
};
//#endregion 🎯️WorldInteractionDomain
//#endregion 🔖️World3dHost
