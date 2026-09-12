// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/World3dHost/component.tsx
/** @emoji 🌐️ `World3dHost` — the 3D world viewport scene host: mesh/instance parsing, point-cloud and vortex-marker
 * layers, catalogue-drop and selection-preview stores, instance chrome, the transform gumball, and the full R3F
 * `World3dHost` component mounted inside a Mode window. The largest scene host in this package. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import React, { createContext, useCallback, useContext, useEffect, useLayoutEffect, useMemo, useRef, useState, Suspense, useSyncExternalStore, type ComponentProps, type DragEvent, type MouseEvent } from "react";
import {
  Box3,
  BoxGeometry,
  BufferAttribute,
  BufferGeometry,
  Color,
  ConeGeometry,
  CylinderGeometry,
  DoubleSide,
  EdgesGeometry,
  Group,
  IcosahedronGeometry,
  LineBasicMaterial,
  LineSegments,
  Mesh,
  MeshStandardMaterial,
  Object3D,
  OrthographicCamera,
  PlaneGeometry,
  PointsMaterial,
  Quaternion,
  ShaderMaterial,
  SphereGeometry,
  TextureLoader,
  TorusGeometry,
  Vector3,
} from "three";
import type { ThreeEvent } from "@semio-tech/ui-react";
import { useFrame, useLoader, useThree } from "@react-three/fiber";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
import { meshAssetTransportUrl } from "../../../../../../../🔨️modules/🖼️assets/🥽️mesh/🟦️.ts";
import { clearColorResolveCache, resolveColorHex, semanticVar, themeColorVar, tokenVar } from "@semio-tech/ui-styling";
import {
  CATALOGUE_DRAG_MIME,
  CELEBRATE_STAMP_DURATION_MS,
  childElementId,
  cn,
  ContextMenuController,
  formatKeybindingShortcut,
  getActiveCatalogueDragPayload,
  glassClass,
  gumballHandleKindToTransformMode,
  IconShotFrame,
  isContextMenuPointerTarget,
  marqueeCoverageFromGesture,
  marqueeModeFromModifiers,
  ndcToViewportPoint,
  Pane,
  reactHostPort,
  registerIntroductionSurfaceResolver,
  registerTutorialCameraDriver,
  SelectionMarquee,
  Spinner,
  sunPositionFromAzimuthElevation,
  UnifiedGumball,
  useCanvasAppearanceSync,
  useLabel,
  usePaneSlot,
  useShellScopeOptional,
  useUiDriver,
  windowChromeClearedTopOffset,
  type Anchor,
  type ContextMenuItem,
  type GumballConfig,
  type GumballHandleKind,
  type GumballPose,
  type IconName,
  type IntroductionResolvedGeometry,
  type SelectionMarqueeCoverage,
  type SelectionMarqueeMethod,
  type SelectionMarqueePoint,
  type TutorialCameraDriver,
  type UiLabel,
} from "@semio-tech/ui-react";
import { isIconName } from "@semio-tech/assets";
import { windowElementId, world3dComputeStatusV1, type ComponentSceneHostProps, type ContextMenuItemSpec, type MergeMode, type PluginContextMenuSurfaceTarget } from "@semio-tech/framework";
import {
  cadVec3ToThree,
  computeWorldProjectionPose,
  DEFAULT_LOD_GRID_FACTOR,
  DEFAULT_MANUAL_LOD,
  frameWorldProjectionPose,
  GLB_MESH_FRAME_ROTATION_X,
  WORLD_MESH_OUTLINE_USER_DATA_KEY,
  worldProjectionDefaults,
  worldProjectionFamily,
  worldProjectionGumballPlane,
  worldProjectionModeFov,
  worldProjectionOrbitConstraints,
  worldProjectionSpecIconId,
  worldProjectionSpecLabel,
  worldSceneContentBounds,
  worldSceneContentBoundsKey,
  WorldCanvas,
  WorldLayerStack,
  WorldLodBridge,
  WorldOrbitGated,
  WorldOrbitViewControls,
  WorldOrbitViewSnapGateProvider,
  WorldProjectionKindSwitch,
  WorldProjectionRig,
  WorldReferenceLayer,
  WorldVolumeLayer,
  type WorldCameraState,
  type WorldProjectionSpec,
  type WorldSceneContentBounds,
  type WorldVolumeRelocatePayload,
} from "@semio-tech/infinite-world-r3f";
import { CAMERA_SYNC_DEBOUNCE_MS } from "../📐️Canvas2dHost/🟦️.tsx";
import { openSurfaceContextMenu, useShellContextMenuFallback, wireLabel, type SurfaceContextMenuResult } from "../🗣️Interpreter/🟦️.tsx";
import { WorldTerrainLayer } from "../🗺️WorldTerrainLayer/🟦️.tsx";
import { base64ToBytes } from "../🖌️Paint2dHost/🟦️.tsx";
import { contextMenuGroupLabel, createCoalescingActionDispatcher, declareSurfaceCancelAction, createInFlightSkippingInterval, isolatedJobDriveIsActive, takeIsolatedJobUiPoll, isRevealCutoffHidden, world3dMarqueeOverlayShape, type Puzzle3dBrushMeshPage, puzzle3dBrushMeshDigest, puzzle3dBrushMeshPages, drainPuzzle3dBrushMeshQueue, PUZZLE3D_MESH_UPLOAD_QUEUE_PAGES, puzzle3dBrushMeshRegistry, NOTE_WORLD_NAVIGATION_ACTION_ID, PUZZLE3D_FILL_REVEAL_GROUP_ID, reconcileCommittedRevealCutoffs, worldRevealCutoffStore, shellLabel, leftoverWorldGumballPoseV1 } from "../🛠️ShellHelpers/🟦️.tsx";
import { SetWindowIconContext, SetWindowTitleContext, useMapContextMenuSpecs } from "../🏛️ShellHost/🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🔖️World3dHost
//#region WorldSceneParsing
type WorldMeshData = {
  readonly positions: readonly number[];
  readonly normals: readonly number[];
  readonly indices: readonly number[];
  /** Per-vertex RGB (0..1, 3 floats per vertex) — e.g. FEM stress contours. Native wgpu renderer has no
   * per-vertex color pipeline yet, so this is a react-renderer-only capability for now. */
  readonly colors?: readonly number[];
  readonly uvs?: readonly number[];
  readonly faceIds?: readonly number[];
  readonly vertexIds?: readonly number[];
  readonly edgePositions?: readonly number[];
  readonly edgeIds?: readonly number[];
  readonly paintTextureBase64?: string;
};

type WorldCameraRecord = {
  readonly position?: readonly [number, number, number];
  readonly target?: readonly [number, number, number];
  readonly fov?: number;
  readonly x?: number;
  readonly y?: number;
  readonly z?: number;
};

type WorldMeshRecord = {
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
  /** 🎨️ Compatible/suggested state (e.g. catalog-kind hover in puzzle) — resolves to the secondary "highlighted" mesh style. */
  readonly highlighted?: boolean;
  /** 🎨️ Non-interactive/locked state — resolves to the muted "disabled" mesh style at reduced opacity. */
  readonly disabled?: boolean;
  readonly smoothShading?: boolean;
  /** 🪣️ 0-based position in a background-planned sequence (e.g. puzzle3d's fill plan) — see `RevealCutoffStore`. Absent for ordinary (non-planned) instances. */
  readonly revealIndex?: number;
  readonly objectKind?: string;
  /** 🎯️ The framework interaction target this instance stands for, when it differs from `id`.
   * `id` must stay unique per rendered instance, but an app's interaction TOPOLOGY may only declare
   * a coarser target (procedural3d renders one instance per geometry item, `{node}@{port}#{index}`,
   * while its topology declares the port `{node}@{port}`) — and `validate_state` prunes any
   * hover/selection id absent from the topology. Dispatches on a `domainId`-bound world window use
   * this when set, falling back to `id`. */
  readonly interactionId?: string;
};

type WorldSelectionTargets = {
  readonly mesh?: boolean;
  readonly vertex?: boolean;
  readonly edge?: boolean;
  readonly face?: boolean;
};

type WorldHoverComponent = {
  readonly objectId?: string;
  readonly mode?: string;
  readonly id?: number;
};

type WorldContextMenuItem = ContextMenuItemSpec;

type WorldSelectionRecord = {
  readonly method?: SelectionMarqueeMethod;
  readonly selectionMergeMode?: MergeMode;
  readonly ids?: readonly string[];
  readonly hoveredId?: string | null;
  readonly referenceSelectedId?: string;
  readonly targetVolumeIds?: readonly string[];
  readonly granularity?: string;
  readonly selectionMode?: string;
  readonly activeObjectId?: string;
  readonly componentIds?: readonly number[];
  readonly targets?: WorldSelectionTargets;
  readonly transformMode?: string;
  readonly interactionMode?: "model" | "paint";
  readonly gumballTarget?: readonly [number, number, number];
  readonly gumballActive?: boolean;
  /** 🎛️ Plugin-authored gumball handle flags (e.g. puzzle3d Move/Rotate). When set, overrides {@link gumballConfigForTransformMode}. */
  readonly gumballConfig?: GumballConfig;
  readonly hoveredComponent?: WorldHoverComponent;
  readonly showEdges?: boolean;
  readonly engagementSessionActive?: boolean;
  /** 🖱️➡️ When true and `targets.face` is set, dragging an already-selected face starts a push/pull gesture (`worldFaceDragEnd` on release) instead of the default marquee/orbit. */
  readonly faceDragActive?: boolean;
  readonly hoveredKindId?: string;
};

type WorldSuggestionCandidateRecord = {
  readonly index: number;
  readonly objectLabel: string;
  readonly vortexLabel: string;
  readonly icon?: string;
  readonly color?: string;
};

type WorldSuggestionMenuRecord = {
  readonly open: boolean;
  readonly x: number;
  readonly y: number;
  readonly windowId?: string;
  readonly vortexFullId?: string;
  readonly pending: boolean;
  readonly candidates: readonly WorldSuggestionCandidateRecord[];
};

type WorldFillBuildRecord = {
  readonly count: number;
  readonly appliedCount: number;
  readonly maxCount: number;
  readonly done: boolean;
};

type WorldInteractionRecord = {
  readonly activeUtility?: string;
  readonly brushCandidateIndex?: number;
  readonly hoveredVortexFullId?: string;
  readonly brushPreviewJson?: string;
  readonly voxelDims?: readonly [number, number, number];
  readonly gridFactor?: number;
  readonly suggestionMenu?: WorldSuggestionMenuRecord | null;
  readonly fillBuild?: WorldFillBuildRecord;
  /** 🪣️ Committed reveal cutoff per reveal group id (see `WindowMeasure.Slider.reveal`) — instances
   * tagged `revealIndex` below this value are shown. Seeds `RevealCutoffStore` when no drag is live. */
  readonly revealCutoffs?: Readonly<Record<string, number>>;
  /** 🔢️ The guest's monotone brush-mesh install counter. Climbs inside one guest instantiation and
   * starts at zero in a fresh one, so a value below the last one this page read proves the guest was
   * restarted and holds nothing this page uploaded — see {@link Puzzle3dBrushMeshRegistry}. */
  readonly meshResidency?: number;
  /** 🚚️ Mesh ids the guest was announced by id alone and cannot serve; it is asking for the bytes. */
  readonly meshReuploadUrls?: readonly string[];
};

type WorldLodRecord = {
  readonly gridFactor?: number;
  readonly gridSnapEnabled?: boolean;
  readonly showLodGrid?: boolean;
  readonly automaticLod?: boolean;
  readonly depthVariableLod?: boolean;
  readonly manualLod?: number;
};

type WorldVortexRecord = World3dMarkerInteractionFields & {
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

type WorldAttractionRecord = World3dMarkerInteractionFields & {
  readonly id: string;
  readonly from: readonly [number, number, number];
  readonly to: readonly [number, number, number];
  readonly color?: string;
};

type WorldTargetVolumeRecord = World3dMarkerInteractionFields & {
  readonly id: string;
  readonly origin: readonly [number, number, number];
  readonly orientation?: readonly [number, number, number, number];
  readonly scale?: readonly [number, number, number] | number;
  readonly color?: string;
  readonly hidden?: boolean;
  readonly locked?: boolean;
  readonly selected?: boolean;
};

type WorldReferenceRecord = World3dMarkerInteractionFields & {
  readonly id: string;
  readonly url: string;
  readonly origin: readonly [number, number, number];
  readonly widthWorld?: number;
  readonly locked?: boolean;
  readonly hidden?: boolean;
  readonly opacity?: number;
};

type WorldBrushPreviewRecord = {
  readonly targetVortexFullId?: string;
  readonly objectKindId?: string;
  readonly sourceVortexIndex?: number;
  readonly meshUrl?: string;
  readonly origin?: readonly [number, number, number];
  readonly orientation?: readonly [number, number, number, number];
  readonly scale?: readonly [number, number, number] | number;
  readonly color?: string;
  readonly opacity?: number;
  readonly fillBuildPreview?: WorldFillDiagnosticRecord;
};

type WorldFillDiagnosticRecord = {
  readonly operation: number;
  readonly baseRevision: number;
  readonly registryGeneration: number;
  readonly sequence: number;
  readonly generation: number;
  readonly stage: string;
  readonly statusLabel: string;
  readonly targetVortexFullId: string | null;
  readonly candidateObjectKindId: string | null;
  readonly candidateGhost: WorldBrushPreviewRecord | null;
  readonly currentPairObjectId: string | null;
  readonly collisionCount: number;
  readonly sampleCursor: number;
  readonly insideBoth: number;
  readonly lastSample: readonly [number, number, number] | null;
  readonly candidatePage: readonly (string | null)[];
  readonly truncated: boolean;
  readonly rejectionReason: string | null;
  readonly targetCursor: number;
  readonly candidateCursor: number;
  readonly acceptedCount: number;
  readonly totalCount: number;
  readonly searchCount: number;
  readonly rejectedCount: number;
};

const WORLD_FILL_STATUS_LABEL_MAX_BYTES = 256;
const WORLD_FILL_COLOR_MAX_BYTES = 128;
const WORLD_FILL_PREVIEW_JSON_MAX_BYTES = 4 * 1024;
const WORLD_FILL_ROOT_KEYS: ReadonlySet<string> = new Set(["targetVortexFullId", "objectKindId", "sourceVortexIndex", "meshUrl", "origin", "orientation", "color", "opacity", "fillBuildPreview"]);
const WORLD_FILL_DIAGNOSTIC_KEYS: ReadonlySet<string> = new Set([
  "operation",
  "baseRevision",
  "registryGeneration",
  "sequence",
  "generation",
  "stage",
  "statusLabel",
  "targetVortexFullId",
  "candidateObjectKindId",
  "candidateGhost",
  "currentPairObjectId",
  "collisionCount",
  "sampleCursor",
  "insideBoth",
  "lastSample",
  "candidatePage",
  "truncated",
  "rejectionReason",
  "targetCursor",
  "candidateCursor",
  "acceptedCount",
  "totalCount",
  "searchCount",
  "rejectedCount",
]);
const WORLD_FILL_GHOST_KEYS: ReadonlySet<string> = new Set(["targetVortexFullId", "objectKindId", "sourceVortexIndex", "meshUrl", "origin", "orientation"]);

function censusAllowedOwnKeys(value: object, allowed: ReadonlySet<string>): number {
  let count = 0;
  for (const key in value) {
    if (!Object.prototype.hasOwnProperty.call(value, key)) continue;
    if (!allowed.has(key)) return -1;
    count += 1;
  }
  return count;
}

/** ☁️ One point-cloud rendering layer (`World3dScene.pointsJson` entries) — the cheap path for
 * 10^5-10^6 points, distinct from per-point meshes. `positionsB64` is base64 of little-endian f32 xyz
 * interleaved; `colorsB64` (optional) is base64 of u8 rgb interleaved, one triplet per point. */
type WorldPointCloudLayerRecord = {
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

type WorldEngagementPreviewItem = WorldEngagementPreviewPoint | WorldEngagementPreviewSegment | WorldEngagementPreviewBox | WorldEngagementPreviewLinearHandle;

//#region WorldMeshPaint
/** 🎨️ Mesh style kinds, in {@link resolveMeshStyle} priority order (highest first). */
type MeshStyleKind = "disabled" | "celebrated" | "selected" | "highlighted" | "hovered" | "neutral";

type MeshStyleColors = {
  readonly meshColor: string;
  readonly lineColor: string;
  readonly emissiveIntensity: number;
  readonly opacity: number;
};

type MeshStylePalette = Readonly<Record<MeshStyleKind, MeshStyleColors>>;

/** 🎨️ CSS-expression paint spec per style kind, ported from the premigration puzzle 3d paint table. */
const MESH_STYLE_PAINT: Readonly<Record<MeshStyleKind, { readonly fill: string; readonly line: string; readonly emissiveIntensity: number; readonly opacity: number }>> = {
  neutral: { fill: "var(--panel)", line: semanticVar("border-normal-color"), emissiveIntensity: 0, opacity: 1 },
  hovered: { fill: semanticVar("hover-interactive-fill"), line: semanticVar("border-emphasized-color"), emissiveIntensity: 0.08, opacity: 1 },
  selected: { fill: tokenVar("primary"), line: tokenVar("primary"), emissiveIntensity: 0.35, opacity: 1 },
  highlighted: { fill: tokenVar("secondary"), line: tokenVar("secondary"), emissiveIntensity: 0.2, opacity: 1 },
  // 🎉️ Transient drop/completion paint — solid fallback for lines; shaded meshes use {@link CelebratingConicMaterial}.
  celebrated: { fill: tokenVar("primary"), line: tokenVar("primary"), emissiveIntensity: 0.55, opacity: 1 },
  disabled: { fill: "color-mix(in oklab, var(--color-muted-foreground) 55%, var(--panel))", line: themeColorVar("muted-foreground"), emissiveIntensity: 0, opacity: 0.45 },
};

/** 🎨️ Resolves the full {@link MeshStylePalette} from live CSS custom properties (theme/dark-mode aware). */
function resolveMeshStylePalette(): MeshStylePalette {
  const resolved = {} as Record<MeshStyleKind, MeshStyleColors>;
  for (const kind of Object.keys(MESH_STYLE_PAINT) as MeshStyleKind[]) {
    const spec = MESH_STYLE_PAINT[kind];
    resolved[kind] = {
      meshColor: resolveColorHex(spec.fill),
      lineColor: resolveColorHex(spec.line),
      emissiveIntensity: spec.emissiveIntensity,
      opacity: spec.opacity,
    };
  }
  return resolved;
}

function useMeshStylePalette(): MeshStylePalette {
  const [palette, setPalette] = useState(resolveMeshStylePalette);
  const shellScope = useShellScopeOptional();
  useCanvasAppearanceSync(
    useCallback(() => {
      // 🎨️ resolveColorHex caches by CSS-expression string only (no theme key), so a theme flip must bust it before re-resolving or every kind keeps its stale color.
      clearColorResolveCache();
      setPalette(resolveMeshStylePalette());
    }, []),
    true,
    shellScope?.rootRef.current ?? undefined,
  );
  return palette;
}

/** 🎨️ Remount key for world mesh materials — R3F/three often keep the previous selected emissive until a later hover forces an update. */
export function worldMeshMaterialRevision(kind: MeshStyleKind): MeshStyleKind {
  return kind;
}

/** 🎨️ Resolves the effective style kind for an instance/component, priority: disabled → celebrated → selected → highlighted → hovered → neutral. */
export function resolveMeshStyle(state: {
  readonly disabled?: boolean;
  readonly celebrating?: boolean;
  readonly selected?: boolean;
  readonly highlighted?: boolean;
  readonly hovered?: boolean;
}): MeshStyleKind {
  if (state.disabled) return "disabled";
  if (state.celebrating) return "celebrated";
  if (state.selected) return "selected";
  if (state.highlighted) return "highlighted";
  if (state.hovered) return "hovered";
  return "neutral";
}

/** 🎨️ Resolves live group-selection preview paint: the new selection is active, while only objects exiting the old selection are highlighted. */
export function resolveMeshSelectionPreviewStyle(
  instance: Pick<WorldInstanceRecord, "disabled" | "selected" | "highlighted" | "hovered"> & { readonly celebrating?: boolean },
  previewSelected?: boolean,
): MeshStyleKind {
  const selectionExited = previewSelected === false && instance.selected === true;
  return resolveMeshStyle({
    disabled: instance.disabled,
    celebrating: instance.celebrating,
    selected: previewSelected ?? instance.selected,
    highlighted: selectionExited || instance.highlighted,
    hovered: instance.hovered,
  });
}

/** 🎨️ Slim alias over {@link MeshStylePalette} for call sites that only need the four legacy semantic colors (face/edge/vertex component overlays, markers). */
export type SemanticColors = {
  readonly mesh: string;
  readonly edge: string;
  readonly select: string;
  readonly hover: string;
  readonly edgeHover: string;
};

/** 🎨️ Maps mesh style palette fills/lines onto World3d semantic overlay colors — edge hover uses line paint so coplanar edges stay distinct from face hover fill. */
export function semanticColorsFromPalette(palette: MeshStylePalette): SemanticColors {
  return {
    mesh: palette.neutral.meshColor,
    edge: palette.neutral.lineColor,
    select: palette.selected.lineColor,
    hover: palette.hovered.meshColor,
    edgeHover: palette.hovered.lineColor,
  };
}

//#region 🎉️WorldInstanceCelebrate
/** 🎉️ Expiry timestamps keyed by world instance id — transient completion paint after catalogue drop (mirrors DOM `celebrateElements`). */
const celebratingWorldInstanceUntil = new Map<string, number>();
const celebratingWorldInstanceListeners = new Set<() => void>();
let celebratingWorldInstanceVersion = 0;
let celebratingWorldInstanceTimer: number | null = null;

function notifyCelebratingWorldInstances(): void {
  celebratingWorldInstanceVersion += 1;
  celebratingWorldInstanceListeners.forEach((listener) => listener());
}

function pruneCelebratingWorldInstances(now = performance.now()): void {
  let changed = false;
  for (const [id, until] of celebratingWorldInstanceUntil) {
    if (until <= now) {
      celebratingWorldInstanceUntil.delete(id);
      changed = true;
    }
  }
  if (changed) notifyCelebratingWorldInstances();
}

function scheduleCelebratingWorldInstancePrune(): void {
  if (celebratingWorldInstanceTimer != null) window.clearTimeout(celebratingWorldInstanceTimer);
  let next = Infinity;
  const now = performance.now();
  for (const until of celebratingWorldInstanceUntil.values()) {
    if (until > now && until < next) next = until;
  }
  if (!Number.isFinite(next)) {
    celebratingWorldInstanceTimer = null;
    return;
  }
  celebratingWorldInstanceTimer = window.setTimeout(() => {
    celebratingWorldInstanceTimer = null;
    pruneCelebratingWorldInstances();
    scheduleCelebratingWorldInstancePrune();
  }, Math.max(0, next - now + 1));
}

/** 🎉️ Stamps world instance ids as celebrating for `durationMs` so their mesh paint uses the spinning conic brand gradient instead of selected. */
export function celebrateWorldInstances(ids: readonly string[], durationMs = CELEBRATE_STAMP_DURATION_MS): () => void {
  const until = performance.now() + durationMs;
  for (const id of ids) celebratingWorldInstanceUntil.set(id, until);
  notifyCelebratingWorldInstances();
  scheduleCelebratingWorldInstancePrune();
  return () => {
    let changed = false;
    for (const id of ids) {
      if (celebratingWorldInstanceUntil.delete(id)) changed = true;
    }
    if (changed) notifyCelebratingWorldInstances();
  };
}

/** 🎉️ Whether `id` is still inside its transient celebration window. */
export function isWorldInstanceCelebrating(id: string, now = performance.now()): boolean {
  const until = celebratingWorldInstanceUntil.get(id);
  return until != null && until > now;
}

function subscribeCelebratingWorldInstances(listener: () => void): () => void {
  celebratingWorldInstanceListeners.add(listener);
  return () => {
    celebratingWorldInstanceListeners.delete(listener);
  };
}

function celebratingWorldInstanceSnapshot(): number {
  return celebratingWorldInstanceVersion;
}

/** 🎉️ React subscription to the transient celebrating-instance set — re-renders when stamps are added or expire. */
function useCelebratingWorldInstanceIds(): ReadonlySet<string> {
  const version = useSyncExternalStore(subscribeCelebratingWorldInstances, celebratingWorldInstanceSnapshot, celebratingWorldInstanceSnapshot);
  return useMemo(() => {
    const now = performance.now();
    const ids = new Set<string>();
    for (const [id, until] of celebratingWorldInstanceUntil) {
      if (until > now) ids.add(id);
    }
    return ids;
  }, [version]);
}

/** 🎉️ One spin period of the celebrate conic — matches `--celebrate-border-duration` (1.2s). */
const CELEBRATE_CONIC_SPIN_SECONDS = 1.2;

/** 🎉️ Document-timeline celebrate spin angle — phase-locked to the CSS `:root` celebrate-border-spin clock. */
function celebrateConicAngleRadians(): number {
  const timelineTime = typeof document !== "undefined" ? document.timeline?.currentTime : null;
  const timeMs = typeof timelineTime === "number" ? timelineTime : performance.now();
  return (((timeMs / 1000) % CELEBRATE_CONIC_SPIN_SECONDS) / CELEBRATE_CONIC_SPIN_SECONDS) * Math.PI * 2;
}

const CELEBRATE_CONIC_VERTEX_SHADER = /* glsl */ `
varying vec3 vObjectPosition;
void main() {
  vObjectPosition = position;
  gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
}
`;

const CELEBRATE_CONIC_FRAGMENT_SHADER = /* glsl */ `
uniform vec3 uColorA;
uniform vec3 uColorB;
uniform vec3 uColorC;
uniform float uAngle;
uniform float uOpacity;
varying vec3 vObjectPosition;
void main() {
  float a = atan(vObjectPosition.y, vObjectPosition.x) + uAngle;
  float t = fract(a / 6.28318530718);
  vec3 color;
  if (t < 0.333333) {
    color = mix(uColorA, uColorB, t / 0.333333);
  } else if (t < 0.666667) {
    color = mix(uColorB, uColorC, (t - 0.333333) / 0.333333);
  } else {
    color = mix(uColorC, uColorA, (t - 0.666667) / 0.333333);
  }
  gl_FragColor = vec4(color, uOpacity);
}
`;

function hexToRgb01(hex: string): [number, number, number] {
  const color = new Color(hex);
  return [color.r, color.g, color.b];
}

/** 🎉️ Spinning primary/secondary/tertiary conic fill — the 3D counterpart of `[data-celebrated="true"]`'s CSS ring. */
function CelebratingConicMaterial({ opacity = 1 }: { readonly opacity?: number }) {
  const invalidate = useThree((state) => state.invalidate);
  const colors = useMemo(() => {
    clearColorResolveCache();
    return {
      a: hexToRgb01(resolveColorHex(tokenVar("primary"))),
      b: hexToRgb01(resolveColorHex(tokenVar("secondary"))),
      c: hexToRgb01(resolveColorHex(tokenVar("tertiary"))),
    };
  }, []);
  const material = useMemo(
    () =>
      new ShaderMaterial({
        vertexShader: CELEBRATE_CONIC_VERTEX_SHADER,
        fragmentShader: CELEBRATE_CONIC_FRAGMENT_SHADER,
        uniforms: {
          uColorA: { value: colors.a },
          uColorB: { value: colors.b },
          uColorC: { value: colors.c },
          uAngle: { value: 0 },
          uOpacity: { value: opacity },
        },
        transparent: opacity < 1,
        side: DoubleSide,
        depthWrite: opacity >= 1,
      }),
    [colors, opacity],
  );
  useEffect(() => () => material.dispose(), [material]);
  useFrame(() => {
    material.uniforms.uAngle.value = celebrateConicAngleRadians();
    invalidate();
  });
  return <primitive object={material} attach="material" />;
}

function createCelebratingConicMaterial(opacity = 1): ShaderMaterial {
  clearColorResolveCache();
  const a = hexToRgb01(resolveColorHex(tokenVar("primary")));
  const b = hexToRgb01(resolveColorHex(tokenVar("secondary")));
  const c = hexToRgb01(resolveColorHex(tokenVar("tertiary")));
  return new ShaderMaterial({
    vertexShader: CELEBRATE_CONIC_VERTEX_SHADER,
    fragmentShader: CELEBRATE_CONIC_FRAGMENT_SHADER,
    uniforms: {
      uColorA: { value: a },
      uColorB: { value: b },
      uColorC: { value: c },
      uAngle: { value: 0 },
      uOpacity: { value: opacity },
    },
    transparent: opacity < 1,
    side: DoubleSide,
    depthWrite: opacity >= 1,
  });
}
//#endregion 🎉️WorldInstanceCelebrate
//#endregion WorldMeshPaint

type WorldParsedCameraState = WorldCameraState & { readonly fov: number; readonly explicitProjection: boolean };

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

function parseCameraState(cameraJson: string): WorldParsedCameraState {
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
    };
  } catch {
    return { position: [4, -4, 3], target: [0, 0, 0], zoom: 1, projection: "perspective", fov: 45, explicitProjection: false };
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
  };
}

/** 📷️ Orbit seed: follow `scene.cameraJson` until a programmatic viewport apply bumps `detachEpoch`; orbit-only detach keeps the seed stable. */
export function world3dViewportCameraSeedKey(sceneCameraJson: string, detachEpoch: number): string {
  return detachEpoch === 0 ? sceneCameraJson : `viewport:${detachEpoch}`;
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
    return x.every((value, index) => Math.abs(value - y[index]) <= epsilon);
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

type WorldEnvironmentMaterialRecord = {
  readonly color?: string;
  readonly metalness?: number;
  readonly roughness?: number;
  readonly emissive?: string;
  readonly emissiveIntensity?: number;
};

type WorldEnvironmentRecord = {
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

const parseEnvironment = (json?: string) => parseJsonRecord<WorldEnvironmentRecord>(json);
const parseFrame = (json?: string) => parseJsonRecord<WorldFrameRecord>(json);
const parseFit = (json?: string) => parseJsonRecord<WorldFitRecord>(json);

function isTransparentWorldBackground(background?: string): boolean {
  return !background || background === "transparent";
}

function fitCameraFromBounds(center: readonly [number, number, number], radius: number, camera: WorldParsedCameraState, padding: number): { position: [number, number, number]; target: [number, number, number]; zoom: number } {
  const distance = Math.max(radius * padding, 2);
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

/** @emoji 🎯️ Fits the orbit camera to the bounds of a scene group once per fit key, preserving the view direction. */
function WorldAutoFit({
  groupRef,
  fitKey,
  padding,
  camera,
  onFitted,
}: {
  readonly groupRef: React.RefObject<Group | null>;
  readonly fitKey: string;
  readonly padding: number;
  readonly camera: WorldParsedCameraState;
  readonly onFitted: (state: WorldCameraState) => void;
}): null {
  const { camera: sceneCamera, controls, invalidate } = useThree();
  const appliedKeyRef = useRef("");
  const targetScratch = useMemo(() => new Vector3(), []);
  useFrame(() => {
    if (!sceneCamera) return;
    const group = groupRef.current;
    if (!group) return;
    if (appliedKeyRef.current === fitKey) return;
    const box = new Box3().setFromObject(group);
    if (box.isEmpty()) return;
    const center = box.getCenter(new Vector3());
    const size = box.getSize(new Vector3());
    const radius = Math.max(size.x, size.y, size.z) * 0.5;
    if (radius <= 0) return;
    appliedKeyRef.current = fitKey;
    const fitted = fitCameraFromBounds([center.x, center.y, center.z], radius, camera, padding);
    const orbit = controls as { target: Vector3; update?: () => void } | null;
    const target = orbit?.target ?? targetScratch;
    target.set(fitted.target[0], fitted.target[1], fitted.target[2]);
    sceneCamera.position.set(fitted.position[0], fitted.position[1], fitted.position[2]);
    if ("zoom" in sceneCamera) sceneCamera.zoom = fitted.zoom;
    sceneCamera.updateProjectionMatrix();
    if (orbit) orbit.update?.();
    else sceneCamera.lookAt(target);
    invalidate();
    onFitted({ ...camera, position: fitted.position, target: fitted.target, zoom: fitted.zoom });
  });
  return null;
}

/** @emoji 📷️ Frames the orbit camera on a live world AABB while keeping the current look direction. */
export function world3dFrameCameraFromBounds(
  center: readonly [number, number, number],
  radius: number,
  camera: WorldParsedCameraState,
  padding = 1.8,
): WorldParsedCameraState {
  const fitted = fitCameraFromBounds(center, Math.max(radius, 0.5), camera, padding);
  return { ...camera, position: fitted.position, target: fitted.target, zoom: fitted.zoom };
}

/** @emoji 📷️ Frames the orbit camera on instance centroids so table-scale vortex markers stay hittable. */
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

function autofitCameraFromInstances(instances: readonly WorldInstanceRecord[]): WorldParsedCameraState {
  if (instances.length === 0) {
    return { position: [4, -4, 3], target: [0, 0, 0], zoom: 1, projection: "perspective", fov: 45, explicitProjection: false };
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
  };
}

/** @emoji 📷️ Seeds a pending display-template projection, framing visible references/instances when present. */
function seedPendingWorldProjectionCamera(
  pendingSpec: WorldProjectionSpec,
  sceneCamera: WorldParsedCameraState,
  instances: readonly WorldInstanceRecord[],
  references: readonly WorldReferenceRecord[],
  viewport?: { readonly width: number; readonly height: number },
): WorldParsedCameraState {
  const bounds = worldSceneContentBounds(instances, references);
  const pose = bounds
    ? frameWorldProjectionPose(pendingSpec, bounds, {
        viewportWidth: viewport?.width,
        viewportHeight: viewport?.height,
      })
    : computeWorldProjectionPose(pendingSpec, {
        target: sceneCamera.target,
        distance: Math.hypot(sceneCamera.position[0] - sceneCamera.target[0], sceneCamera.position[1] - sceneCamera.target[1], sceneCamera.position[2] - sceneCamera.target[2]) || 600,
      });
  return { ...pose, fov: worldProjectionModeFov(pendingSpec) ?? sceneCamera.fov, explicitProjection: true };
}

/** @emoji 📷️ Viewport-aware reframe for a projection seed so orthographic panes fit content — re-runs whenever
 * {@link worldSceneContentBoundsKey} changes (fill planning grows the scene) until the host stops enabling it
 * (user-owned orbit/pan/zoom). */
function WorldProjectionContentFrame(props: {
  readonly enabled: boolean;
  readonly spec: WorldProjectionSpec | undefined;
  readonly bounds: WorldSceneContentBounds | null;
  readonly fov: number;
  readonly onFramed: (state: WorldParsedCameraState) => void;
}): null {
  const size = useThree((state) => state.size);
  const getThree = useThree((state) => state.get);
  const invalidate = useThree((state) => state.invalidate);
  const framedBoundsKeyRef = useRef<string | null>(null);
  const onFramedRef = useRef(props.onFramed);
  onFramedRef.current = props.onFramed;
  const boundsKey = worldSceneContentBoundsKey(props.bounds);
  useLayoutEffect(() => {
    if (!props.enabled || !props.spec || !props.bounds || !boundsKey) return;
    if (size.width < 1 || size.height < 1) return;
    if (framedBoundsKeyRef.current === boundsKey) return;
    framedBoundsKeyRef.current = boundsKey;
    // 📷️ Read the live store camera — render-time `useThree(s => s.camera)` is still the Canvas default
    // PerspectiveCamera while sibling `OrthographicCamera makeDefault` runs in an earlier layout effect.
    const { camera, controls: rawControls } = getThree();
    const controls = rawControls as { target: Vector3; update?: () => void } | null;
    const pose = frameWorldProjectionPose(props.spec, props.bounds, { viewportWidth: size.width, viewportHeight: size.height });
    const framed: WorldParsedCameraState = { ...pose, fov: worldProjectionModeFov(props.spec) ?? props.fov, explicitProjection: true };
    const ortho = camera as OrthographicCamera & { readonly isOrthographicCamera?: boolean };
    if (ortho.isOrthographicCamera) {
      ortho.left = size.width / -2;
      ortho.right = size.width / 2;
      ortho.top = size.height / 2;
      ortho.bottom = size.height / -2;
      ortho.zoom = framed.zoom;
      ortho.position.set(framed.position[0], framed.position[1], framed.position[2]);
      ortho.up.set(framed.up?.[0] ?? 0, framed.up?.[1] ?? 1, framed.up?.[2] ?? 0);
      const target = controls?.target;
      if (target) {
        target.set(framed.target[0], framed.target[1], framed.target[2]);
        controls?.update?.();
      } else {
        ortho.lookAt(framed.target[0], framed.target[1], framed.target[2]);
      }
      ortho.updateProjectionMatrix();
    } else if (camera) {
      camera.position.set(framed.position[0], framed.position[1], framed.position[2]);
      camera.up.set(framed.up?.[0] ?? 0, framed.up?.[1] ?? 1, framed.up?.[2] ?? 0);
      const target = controls?.target;
      if (target) {
        target.set(framed.target[0], framed.target[1], framed.target[2]);
        controls?.update?.();
      } else {
        camera.lookAt(framed.target[0], framed.target[1], framed.target[2]);
      }
      if ("zoom" in camera) (camera as OrthographicCamera).zoom = framed.zoom;
      if ("updateProjectionMatrix" in camera) (camera as OrthographicCamera).updateProjectionMatrix();
    }
    invalidate();
    onFramedRef.current(framed);
  }, [boundsKey, getThree, invalidate, props.bounds, props.enabled, props.fov, props.spec, size.height, size.width]);
  return null;
}

/** @emoji 🥽️ One built-in mesh kind's geometry, built from the engine's own primitives. The placement and
 * extent are pinned against `🧰️framework/🔨️modules/🏗️mesh-engine/🧫️fixtures/🥽️scene-mesh-kinds/🔣️.json`,
 * the same fixture Rust's `mesh_from_kind` answers to — the two tessellate differently on purpose, so the
 * local bounding box, not the triangle list, is the contract. `cone` and `plane`/`torus` are re-placed
 * because the engine's primitives are centred / lie in a different plane than the scene convention. */
function worldMeshKindGeometry(kind: string): BufferGeometry {
  switch (kind) {
    case "plane":
      return new PlaneGeometry(1, 1).rotateX(-Math.PI / 2);
    case "sphere":
    case "uvSphere":
      return new SphereGeometry(0.5, 16, 12);
    case "icoSphere":
      return new IcosahedronGeometry(0.5, 1);
    case "vortex-marker":
      return new IcosahedronGeometry(0.12, 1);
    case "vertex-marker":
      return new IcosahedronGeometry(1, 1);
    case "cylinder":
      return new CylinderGeometry(0.5, 0.5, 1, 16);
    case "cone":
      return new ConeGeometry(0.5, 1, 16).translate(0, 0.5, 0);
    case "torus":
      return new TorusGeometry(0.5, 0.15, 16, 12).rotateX(-Math.PI / 2);
    default:
      return new BoxGeometry(1, 1, 1);
  }
}

const worldMeshKindData = new Map<string, WorldMeshData>();

/** @emoji 🥽️ Resolves a `{ id, kind }` scene mesh reference into the buffers every downstream path
 * (shading, edge outlines, marquee bounds, component overlays) already expects. Memoized per kind: the
 * set is closed and tiny, and a scene refresh must not re-tessellate. */
export function meshDataFromKind(kind: string): WorldMeshData {
  const cached = worldMeshKindData.get(kind);
  if (cached) return cached;
  const geometry = worldMeshKindGeometry(kind);
  if (!geometry.getAttribute("normal")) geometry.computeVertexNormals();
  const index = geometry.getIndex();
  const positions = Array.from(geometry.getAttribute("position").array as ArrayLike<number>);
  const data: WorldMeshData = {
    positions,
    normals: Array.from(geometry.getAttribute("normal").array as ArrayLike<number>),
    // 🔺️ A non-indexed engine primitive (the polyhedra) still owes this record a triangle list: every
    // downstream reader gates shaded rendering, face overlays and pick on `indices.length > 0`, and the
    // Rust generator emits the same trivial sequential run for its own non-indexed triangles.
    indices: index ? Array.from(index.array as ArrayLike<number>) : Array.from({ length: positions.length / 3 }, (_unused, vertex) => vertex),
  };
  geometry.dispose();
  worldMeshKindData.set(kind, data);
  return data;
}

function parseMeshes(meshesJson: string): WorldMeshRecord[] {
  try {
    const parsed = JSON.parse(meshesJson);
    if (!Array.isArray(parsed)) return [];
    return (parsed as WorldMeshRecord[]).map((record) => (record.data || !record.kind ? record : { ...record, data: meshDataFromKind(record.kind) }));
  } catch {
    return [];
  }
}

function parseInstances(instancesJson: string): WorldInstanceRecord[] {
  try {
    const parsed = JSON.parse(instancesJson);
    return Array.isArray(parsed) ? (parsed as WorldInstanceRecord[]) : [];
  } catch {
    return [];
  }
}

/** 🚚️ The `instancesDeltaJson` lane's declared shape — see the Rust `World3dScene::instances_delta_json`. */
export type WorldInstanceDeltaV1 = {
  readonly base: number;
  readonly revision: number;
  readonly count: number;
  readonly changed: readonly WorldInstanceRecord[];
  readonly removed: readonly string[];
};

/** 🧾️ One consumer's retained instance set and the delta revision it stands at. */
export type WorldInstanceResidencyV1 = {
  readonly revision: number;
  readonly records: readonly WorldInstanceRecord[];
};

export function parseWorldInstanceDelta(deltaJson: string | null | undefined): WorldInstanceDeltaV1 | null {
  if (!deltaJson) return null;
  try {
    const parsed = JSON.parse(deltaJson) as Partial<WorldInstanceDeltaV1>;
    if (typeof parsed?.revision !== "number" || typeof parsed?.base !== "number" || !Array.isArray(parsed.changed)) return null;
    return { base: parsed.base, revision: parsed.revision, count: typeof parsed.count === "number" ? parsed.count : parsed.changed.length, changed: parsed.changed, removed: Array.isArray(parsed.removed) ? parsed.removed : [] };
  } catch {
    return null;
  }
}

/**
 * @emoji 🚚️ Advances one consumer's retained instance set by the publication it just received.
 *
 * ⏱️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B44. A pose edit on a large document republishes the
 * whole `instancesJson` lane — 55 KiB / 180 records on Nakagin — and every consumer used to re-parse
 * all of it to learn that ONE instance moved. When the producer also publishes
 * `instancesDeltaJson` and this consumer's retained set is exactly at that delta's `base`, the
 * changed records are substituted BY ID and every untouched record keeps its object identity, so the
 * downstream instanced-mesh memos see only the instances that actually moved.
 *
 * 🧯️ In-place application is deliberately restricted to pure UPDATES of ids already retained.
 * `instancesJson` is index-addressed (`worldPick` resolves a hit by array position, which is why a
 * hidden instance stays in the array at zero scale), so an addition or a removal — anything that can
 * reorder — falls back to the authoritative full parse. Same for a delta whose `base` does not match,
 * a `count` that disagrees with the result, or a producer that publishes no delta at all: the full
 * lane is always correct, so ignoring the delta can only cost time, never correctness.
 */
export function advanceWorldInstanceResidency(previous: WorldInstanceResidencyV1 | null, instancesJson: string, deltaJson: string | null | undefined): WorldInstanceResidencyV1 {
  const delta = parseWorldInstanceDelta(deltaJson);
  const retained = previous?.records ?? null;
  const applicable =
    delta !== null &&
    retained !== null &&
    previous?.revision === delta.base &&
    delta.removed.length === 0 &&
    delta.count === retained.length &&
    delta.changed.every((record) => retained.some((existing) => existing.id === record.id));
  if (!applicable) return { revision: delta?.revision ?? -1, records: parseInstances(instancesJson) };
  if (delta.changed.length === 0) return { revision: delta.revision, records: retained };
  const replacements = new Map(delta.changed.map((record) => [record.id, record]));
  return { revision: delta.revision, records: retained.map((existing) => replacements.get(existing.id) ?? existing) };
}

function parseSelection(selectionJson: string): WorldSelectionRecord {
  try {
    return JSON.parse(selectionJson) as WorldSelectionRecord;
  } catch {
    return { method: "rectangle", ids: [] };
  }
}

export type LeftoverWorldSelectionOverlayV1 = {
  readonly ids: readonly string[];
  readonly hoveredId: string | null;
  readonly hoveredDomain?: string | null;
  readonly gumballActive: boolean;
  readonly gumballAnchorId: string | null;
  readonly activeUtility?: string | null;
  readonly activeToolId?: string | null;
  readonly brushPreviewJson?: string | null;
};

/** 🪟️ The overlay fields ONE pane owns. Everything else in the overlay is the document's, shared by
 * every pane of it: `ids`/`gumball*` are a document selection (B20) and `activeToolId` is a
 * mode-level tool. Hover, the armed window utility and its brush preview belong to the window
 * INSTANCE that resolved them (wave B9), so one pane arming Brush must never read back in another. */
const LEFTOVER_WORLD_WINDOW_FIELDS = ["hoveredId", "hoveredDomain", "activeUtility", "brushPreviewJson"] as const;

/** 🪟️ Where ONE leftover publication has authority: a single pane, the document's shared fields only,
 * or every pane at once — the last is what a mode-level tool activation is (it clears every window's
 * utility, `🏛️ShellHost`'s `clearAllWindowUtilities`). */
export type LeftoverWorldOverlayScopeV1 = { readonly kind: "window"; readonly windowId: string } | { readonly kind: "document" } | { readonly kind: "allWindows" };

const leftoverWorldOverlayByWindow = new Map<string, LeftoverWorldSelectionOverlayV1>();
let leftoverWorldDocumentOverlay: LeftoverWorldSelectionOverlayV1 | null = null;
const leftoverWorldSelectionListeners = new Set<() => void>();

/** 🪪️ `document`'s own shared fields with `window`'s per-pane fields laid over them. */
function leftoverWorldOverlayForWindowFieldsV1(document: LeftoverWorldSelectionOverlayV1, window: LeftoverWorldSelectionOverlayV1): LeftoverWorldSelectionOverlayV1 {
  const merged: Record<string, unknown> = { ...document };
  for (const field of LEFTOVER_WORLD_WINDOW_FIELDS) merged[field] = window[field];
  return merged as unknown as LeftoverWorldSelectionOverlayV1;
}

/** 🕹️ Host leftover InteractionView overlay — vite-live until guest scene.selectionJson republishes.
 *
 * A `window` publication owns only that pane's window fields; the document slot takes the
 * publication's shared fields and KEEPS its own window fields, so arming Brush in one pane leaves
 * every other pane exactly as it was (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B39). */
export function publishLeftoverWorldSelectionV1(overlay: LeftoverWorldSelectionOverlayV1 | null, scope: LeftoverWorldOverlayScopeV1): void {
  if (scope.kind === "allWindows") leftoverWorldOverlayByWindow.clear();
  if (scope.kind === "window") {
    if (overlay) leftoverWorldOverlayByWindow.set(scope.windowId, overlay);
    else leftoverWorldOverlayByWindow.delete(scope.windowId);
  }
  if (!overlay) leftoverWorldDocumentOverlay = null;
  else if (scope.kind === "allWindows" || !leftoverWorldDocumentOverlay) leftoverWorldDocumentOverlay = overlay;
  else leftoverWorldDocumentOverlay = leftoverWorldOverlayForWindowFieldsV1(overlay, leftoverWorldDocumentOverlay);
  for (const listener of leftoverWorldSelectionListeners) listener();
}

/** 🗂️ The document-scoped overlay: the shared selection plus the window fields of the last
 * document-wide publication. Readers that are about the DOCUMENT (the Outliner/Inspection tree, the
 * armed mode-level tool) read this; a pane reads {@link leftoverWorldWindowOverlayV1}. */
export function leftoverWorldSelectionOverlayV1(): LeftoverWorldSelectionOverlayV1 | null {
  return leftoverWorldDocumentOverlay;
}

/** 🪟️ What ONE pane's record must be overlaid with: the document's shared fields under this pane's
 * own window fields, falling back to the document-wide ones for a pane that has published none. */
export function leftoverWorldWindowOverlayV1(windowId: string | null | undefined): LeftoverWorldSelectionOverlayV1 | null {
  const own = windowId ? leftoverWorldOverlayByWindow.get(windowId) : undefined;
  if (!own) return leftoverWorldDocumentOverlay;
  return leftoverWorldOverlayForWindowFieldsV1(leftoverWorldDocumentOverlay ?? own, own);
}

/** 🖌️ The pane whose own overlay is armed with a brush, for the host's brush-preview refresh lane —
 * an arm lives in exactly one pane now, so "is anything armed" is a search, not a global read. */
export function leftoverWorldArmedWindowOverlayV1(): LeftoverWorldSelectionOverlayV1 | null {
  for (const [windowId, overlay] of leftoverWorldOverlayByWindow) {
    if (leftoverOverlayArmedBrushUtilityV1(overlay.activeUtility)) return leftoverWorldWindowOverlayV1(windowId);
  }
  return leftoverWorldDocumentOverlay;
}

/** 🖌️ The window instance id whose own overlay is armed with a brush, or `undefined`. */
export function leftoverWorldArmedWindowIdV1(): string | undefined {
  for (const [windowId, overlay] of leftoverWorldOverlayByWindow) {
    if (leftoverOverlayArmedBrushUtilityV1(overlay.activeUtility)) return windowId;
  }
  return undefined;
}

/**
 * 🧰️ The armed window utility is host session state owned by `SET_ACTIVE_UTILITY_ACTION_ID`, NOT part of
 * the guest's `InteractionView` leftover — so an `interactionSelect`/`interactionHover`/`setCamera`
 * leftover republish, which carries no `activeUtility` at all, must CARRY the armed one forward instead of
 * replacing the overlay with an utility-less one (that dropped the lane back to the guest's `select` and
 * disarmed Brush on the first pointer move after arming it).
 */
export function leftoverOverlayArmedBrushUtilityV1(utility: string | null | undefined): utility is "brush" | "volumeBrush" {
  return utility === "brush" || utility === "volumeBrush";
}

export function leftoverOverlayCarryingUtilityV1(next: LeftoverWorldSelectionOverlayV1, prior: LeftoverWorldSelectionOverlayV1 | null): LeftoverWorldSelectionOverlayV1 {
  const nextUtility = next.activeUtility === undefined ? prior?.activeUtility ?? null : next.activeUtility;
  const activeUtility = nextUtility === "select" && leftoverOverlayArmedBrushUtilityV1(prior?.activeUtility) ? prior?.activeUtility : nextUtility;
  const carried = nextUtility === next.activeUtility && activeUtility === next.activeUtility ? next : { ...next, activeUtility };
  const withTool = carried.activeToolId === undefined ? { ...carried, activeToolId: prior?.activeToolId ?? null } : carried;
  const preview = typeof withTool.brushPreviewJson === "string" && withTool.brushPreviewJson.length > 0 ? withTool.brushPreviewJson : prior?.brushPreviewJson ?? null;
  return preview === withTool.brushPreviewJson ? withTool : { ...withTool, brushPreviewJson: preview };
}

/** 🛠️ Fill is a mode-level tool, not a window utility — leftover `select` must not mask an armed fill tab. */
export function leftoverOverlayArmedUtilityV1(leftover: LeftoverWorldSelectionOverlayV1 | null | undefined): string | null | undefined {
  return leftover?.activeToolId === "fill" ? "fill" : leftover?.activeUtility;
}

/** 🕹️ Hover leftover publishes empty `ids` — a first pick must keep leftover.ids, never fall back to hoveredId. */
export function leftoverOverlayCarryingSelectionV1(next: LeftoverWorldSelectionOverlayV1, prior: LeftoverWorldSelectionOverlayV1 | null): LeftoverWorldSelectionOverlayV1 {
  const carried = leftoverOverlayCarryingUtilityV1(next, prior);
  const utility = leftoverOverlayArmedBrushUtilityV1(prior?.activeUtility) && carried.hoveredId && carried.activeUtility === "select"
    ? { ...carried, activeUtility: prior?.activeUtility }
    : carried;
  if (utility.ids.length > 0 || !prior?.ids.length) return utility;
  return {
    ...utility,
    ids: prior.ids,
    gumballActive: utility.gumballActive || prior.gumballActive,
    gumballAnchorId: utility.gumballAnchorId ?? prior.gumballAnchorId ?? prior.ids[0] ?? null,
  };
}

/** 📄️ Outliner/Inspection tree keys are the entity id; leftover.ids never use hoveredId. */
export function leftoverTreeItemSelectedV1(itemId: string, leftoverIds: readonly string[] | undefined): boolean {
  if (!leftoverIds?.length) return false;
  return leftoverIds.some((id) => id === itemId || itemId.endsWith(`/${id}`) || itemId.endsWith(`.${id}`));
}

/** 🕹️ Wave B15: leftover after interactionSelect must include hoverTarget.id in selectedIds. */
export function leftoverSelectIdsMustNameHoverPickV1(selectedIds: readonly string[] | undefined, hoverId: string | null | undefined): boolean {
  return Boolean(hoverId) && (selectedIds ?? []).includes(hoverId);
}

export function subscribeLeftoverWorldSelectionV1(listener: () => void): () => void {
  leftoverWorldSelectionListeners.add(listener);
  return () => leftoverWorldSelectionListeners.delete(listener);
}

/** Hover-only leftover still overlays — selectedIds empty is the interactionHover leftover shape. */
export function leftoverWorldOverlayAppliesV1(leftover: LeftoverWorldSelectionOverlayV1 | null | undefined): boolean {
  return Boolean(leftover && (leftover.ids.length > 0 || leftover.hoveredId || leftover.activeUtility || leftover.activeToolId));
}

/** Vortex-domain leftover hover id (`objectId:vortexId`) for hoveredVortexFullId. */
export function leftoverHoveredVortexFullIdV1(leftover: Pick<LeftoverWorldSelectionOverlayV1, "hoveredId" | "hoveredDomain"> | null | undefined): string | undefined {
  const id = leftover?.hoveredId;
  if (!id) return undefined;
  if (leftover.hoveredDomain && leftover.hoveredDomain !== "vortex") return undefined;
  return id.includes(":") ? id : undefined;
}

/** 🖌️ Armed leftover brush must not publish an empty interactionHover — that clears the guest hover
 * map and the next SurfaceVisible / refreshUi re-projects preview=0 over the 252–313 byte lane. */
export function leftoverBrushRetainGuestHoverV1(activeUtility: string | null | undefined, leftover: LeftoverWorldSelectionOverlayV1 | null | undefined): string | undefined {
  return leftoverOverlayArmedBrushUtilityV1(activeUtility ?? leftover?.activeUtility) ? leftoverHoveredVortexFullIdV1(leftover) : undefined;
}

export function mergeWorldSelectionWithLeftoverV1(base: WorldSelectionRecord, leftover: LeftoverWorldSelectionOverlayV1 | null, instances: readonly WorldInstanceRecord[] = []): WorldSelectionRecord {
  if (!leftoverWorldOverlayAppliesV1(leftover) || !leftover) return base;
  const pose = leftoverWorldGumballPoseV1(leftover, instances);
  return {
    ...base,
    // 🎯️ `activeObjectId` travels WITH the ids it belongs to. The overlay exists to make a pick visible
    // before the guest's own lane answers, and `ids` alone only covers the readers that take a list:
    // the gumball target, Inspection's focus and `data-selection-json`'s own `activeObjectId` all read
    // this single field, so an overlay that replaced the list and left this null published
    // `{selectedIds:[picked], activeObjectId:null}` for the whole window it was covering — measured on
    // the 180-object Nakagin document, every sample of a 12-minute run
    // (26/09/02/PUZZLE-3D-END-TO-END wave B46). The base's own active id is kept when the overlay still
    // names it, so a re-pick of the same object does not move the active one.
    ...(leftover.ids.length > 0 ? { ids: leftover.ids, activeObjectId: base.activeObjectId && leftover.ids.includes(base.activeObjectId) ? base.activeObjectId : leftover.ids[0] } : {}),
    hoveredId: leftover.hoveredId ?? base.hoveredId,
    gumballActive: leftover.gumballActive || Boolean(base.gumballActive),
    gumballTarget: pose.gumballTarget ?? base.gumballTarget,
    transformMode: pose.transformMode ?? base.transformMode,
  };
}

export function mergeWorldInteractionWithLeftoverV1(base: WorldInteractionRecord, leftover: LeftoverWorldSelectionOverlayV1 | null): WorldInteractionRecord {
  const hoveredVortexFullId = leftoverHoveredVortexFullIdV1(leftover);
  const activeUtility = leftoverOverlayArmedUtilityV1(leftover);
  if (!hoveredVortexFullId && !activeUtility) return base;
  return {
    ...base,
    ...(hoveredVortexFullId ? { hoveredVortexFullId } : {}),
    ...(activeUtility ? { activeUtility } : {}),
  };
}

function mergeWorldSelectionWithLeftover(base: WorldSelectionRecord, windowId: string | null | undefined, instances: readonly WorldInstanceRecord[] = []): WorldSelectionRecord {
  return mergeWorldSelectionWithLeftoverV1(base, leftoverWorldWindowOverlayV1(windowId), instances);
}

/** 🔦️ What ONE pane publishes as `data-selection-json` — the state the pane actually PAINTS, after
 * `mergeWorldSelectionWithLeftoverV1` has laid the host's leftover overlay over the guest's own
 * `selectionJson` lane. Until this attribute existed the pane published its geometry, its camera and
 * its interaction record but never its selection: `WorldInteractionRecord` carries neither
 * `selectedIds` nor a hover target (it is the utility/brush/fill record), `data-instances-json` is the
 * guest's geometry cache which deliberately never bakes selection into an instance
 * (`world_instances_geometry_json`), and `data-status-json` is the off-thread COMPUTE status
 * (`{computing,label}`) — so every outside reader of "what is selected in this pane" was reading
 * fields that never held it (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B20 defect 1).
 *
 * 🪟️ Selection is per DOCUMENT and hover is per WINDOW, so `selectedIds` is the same list in every
 * pane of one document while `hoverTarget` is whatever THIS pane resolved; `activeUtility` rides along
 * because it is per window instance too (wave B9). */
export function worldSurfaceSelectionDomV1(selection: WorldSelectionRecord, interaction: WorldInteractionRecord): Record<string, unknown> {
  const hoveredId = selection.hoveredId ?? null;
  return {
    selectedIds: selection.ids ?? [],
    activeObjectId: selection.activeObjectId ?? null,
    targetVolumeIds: selection.targetVolumeIds ?? [],
    referenceSelectedId: selection.referenceSelectedId ?? null,
    hoverTarget: hoveredId ? { domain: hoveredId.includes(":") ? "vortex" : "object", id: hoveredId } : null,
    hoveredVortexFullId: interaction.hoveredVortexFullId ?? null,
    hoveredKindId: selection.hoveredKindId ?? null,
    gumballActive: Boolean(selection.gumballActive),
    gumballTarget: selection.gumballTarget ?? null,
    transformMode: selection.transformMode ?? null,
    activeUtility: interaction.activeUtility ?? "select",
  };
}

/** 🛰️ What the GUEST itself sent on this pane's own selection lane, BEFORE
 * {@link mergeWorldSelectionWithLeftoverV1} lays the host's leftover overlay over it.
 *
 * `data-selection-json` publishes the merged result — the state the pane PAINTS — so a guest that
 * renders from an empty interaction and a guest that renders from the picked one are byte-identical
 * from outside as soon as the host's own leftover overlay carries the ids. That is precisely the
 * divergence the selection-scoped defect family lives in (ticket 26/09/02/PUZZLE-3D-END-TO-END wave
 * B23): on wasm #48 the panes painted `selectedIds:["seed-left-001"]` while the guest's Inspection
 * body rendered the empty-document summary in the same window of time, and no attribute anywhere
 * separated "the guest has the pick" from "the host is painting it for the guest".
 *
 * Kept deliberately minimal — ids, hover, gumball — because its ONE job is to be comparable against
 * `data-selection-json`'s same three fields. */
export function worldSurfaceGuestSelectionDomV1(selectionJson: string | undefined): Record<string, unknown> {
  const guest = parseSelection(selectionJson ?? "{}");
  return {
    selectedIds: guest.ids ?? [],
    activeObjectId: guest.activeObjectId ?? null,
    hoveredId: guest.hoveredId ?? null,
    gumballActive: Boolean(guest.gumballActive),
  };
}

export function parseJsonArray<T>(json: string | undefined): readonly T[] {
  if (!json) return [];
  try {
    const parsed = JSON.parse(json);
    return Array.isArray(parsed) ? (parsed as T[]) : [];
  } catch {
    return [];
  }
}

export function parseSelectionDomainsFromSession(json: string): { readonly nodes: string[]; readonly edges: string[]; readonly handles: string[] } {
  try {
    const parsed = JSON.parse(json) as unknown;
    if (Array.isArray(parsed)) {
      return { nodes: parsed as string[], edges: [], handles: [] };
    }
    if (parsed && typeof parsed === "object") {
      const record = parsed as { nodes?: unknown; edges?: unknown; handles?: unknown; edgeIds?: unknown; handleIds?: unknown };
      return {
        nodes: Array.isArray(record.nodes) ? (record.nodes as string[]) : [],
        edges: Array.isArray(record.edges) ? (record.edges as string[]) : Array.isArray(record.edgeIds) ? (record.edgeIds as string[]) : [],
        handles: Array.isArray(record.handles) ? (record.handles as string[]) : Array.isArray(record.handleIds) ? (record.handleIds as string[]) : [],
      };
    }
  } catch {
    /* invalid json */
  }
  return { nodes: [], edges: [], handles: [] };
}

export function selectionGroupsFromDomains(domains: { readonly nodes: string[]; readonly edges: string[]; readonly handles: string[] }): NonNullable<PluginContextMenuSurfaceTarget["selection"]> {
  const groups: NonNullable<PluginContextMenuSurfaceTarget["selection"]>[number][] = [];
  if (domains.nodes.length > 0) groups.push({ domain: "node", ids: domains.nodes });
  if (domains.edges.length > 0) groups.push({ domain: "edge", ids: domains.edges });
  if (domains.handles.length > 0) groups.push({ domain: "handle", ids: domains.handles });
  return groups;
}

/** @emoji 🖱️ Maps plugin-authored {@link ContextMenuItemSpec} rows onto UI {@link ContextMenuItem} rows, binding select/hover to host `dispatch`.
 *
 * 🗂️ A `menu.group.<category>` row travels from the guest with `label: undefined` by contract — the
 * taxonomy is chrome vocabulary the SHELL owns, not app vocabulary (`plugin/🦀️.rs`'s `Menu::group`,
 * asserted by `plugin-runtime-plugin-builder-contract`'s "group rows travel with no label"). So the
 * label is resolved here from the same EN/DE `ui.ribbon.parent.*` bundle the ribbon reads, exactly
 * like the wgpu target's `shell_context_menu_item_from_spec` does through `ribbon_parent_label` —
 * including its default folder icon. Without this the React shell rendered the raw ids
 * (`menu.group.history`, `menu.group.selection`, …), measured 2026-09-09 21:05. */
export function mapContextMenuSpecs(
  specs: readonly ContextMenuItemSpec[] | null | undefined,
  dispatch: (action: string, args?: Record<string, unknown>) => void,
  keysByActionId?: ReadonlyMap<string, string>,
): ContextMenuItem[] {
  return (specs ?? []).map((spec) => {
    const boundKeys = spec.action ? keysByActionId?.get(spec.action) : undefined;
    const shortcut = spec.shortcut ?? (boundKeys ? formatKeybindingShortcut(boundKeys) : undefined);
    const groupLabel = spec.label === undefined ? contextMenuGroupLabel(spec.id) : undefined;
    return {
      id: spec.id,
      action: spec.action,
      label: spec.label === undefined ? groupLabel : wireLabel(spec.label),
      icon: spec.icon && isIconName(spec.icon) ? spec.icon : groupLabel === undefined ? undefined : "folder",
      color: spec.color,
      shortcut,
      disabled: spec.disabled,
      separator: spec.separator,
      checked: spec.checked,
      destructive: spec.destructive,
      onSelect: spec.action
        ? (event) => {
            const clientX = event && "clientX" in event && typeof event.clientX === "number" ? event.clientX : undefined;
            const clientY = event && "clientY" in event && typeof event.clientY === "number" ? event.clientY : undefined;
            const pointArgs = spec.action === "openVortexSuggestions" && clientX != null && clientY != null ? { x: clientX, y: clientY } : undefined;
            dispatch(spec.action!, { ...spec.args, ...pointArgs });
          }
        : undefined,
      onHover: spec.hoverAction ? () => dispatch(spec.hoverAction!, spec.hoverArgs) : undefined,
      children: spec.children?.length ? mapContextMenuSpecs(spec.children, dispatch, keysByActionId) : undefined,
    };
  });
}

function parseInteraction(interactionJson: string | undefined): WorldInteractionRecord {
  if (!interactionJson) return {};
  try {
    return JSON.parse(interactionJson) as WorldInteractionRecord;
  } catch {
    return {};
  }
}

function parseLod(lodJson: string | undefined): WorldLodRecord {
  if (!lodJson) return {};
  try {
    return JSON.parse(lodJson) as WorldLodRecord;
  } catch {
    return {};
  }
}

export function parseWorldBrushPreview(brushPreviewJson: string | undefined): WorldBrushPreviewRecord | null {
  if (!brushPreviewJson) return null;
  try {
    const parsed = JSON.parse(brushPreviewJson) as WorldBrushPreviewRecord;
    if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) return null;
    const hasFillDiagnostic = Object.prototype.hasOwnProperty.call(parsed, "fillBuildPreview");
    const diagnostic = parsed.fillBuildPreview;
    if (!hasFillDiagnostic) return parsed;
    if (typeof diagnostic !== "object" || diagnostic === null || Array.isArray(diagnostic)) return null;
    if (censusAllowedOwnKeys(parsed, WORLD_FILL_ROOT_KEYS) < 0 || censusAllowedOwnKeys(diagnostic, WORLD_FILL_DIAGNOSTIC_KEYS) !== WORLD_FILL_DIAGNOSTIC_KEYS.size) return null;
    const nullableString = (value: unknown) => value === null || typeof value === "string";
    const nonnegativeInteger = (value: unknown) => Number.isSafeInteger(value) && (value as number) >= 0;
    const finiteTuple = (value: unknown, length: number) => Array.isArray(value) && value.length === length && value.every((item) => typeof item === "number" && Number.isFinite(item));
    const equalTuple = (left: unknown, right: unknown) => Array.isArray(left) && Array.isArray(right) && left.length === right.length && left.every((item, index) => item === right[index]);
    const boundedUtf8 = (value: string, maximumBytes: number) => {
      let bytes = 0;
      for (const scalar of value) {
        const point = scalar.codePointAt(0) ?? 0;
        bytes += point <= 0x7f ? 1 : point <= 0x7ff ? 2 : point <= 0xffff ? 3 : 4;
        if (bytes > maximumBytes) return false;
      }
      return true;
    };
    if (!boundedUtf8(brushPreviewJson, WORLD_FILL_PREVIEW_JSON_MAX_BYTES)) return null;
    if (
      (Object.prototype.hasOwnProperty.call(parsed, "targetVortexFullId") && typeof parsed.targetVortexFullId !== "string") ||
      (Object.prototype.hasOwnProperty.call(parsed, "objectKindId") && typeof parsed.objectKindId !== "string") ||
      (Object.prototype.hasOwnProperty.call(parsed, "sourceVortexIndex") && !nonnegativeInteger(parsed.sourceVortexIndex)) ||
      (Object.prototype.hasOwnProperty.call(parsed, "meshUrl") && typeof parsed.meshUrl !== "string") ||
      (Object.prototype.hasOwnProperty.call(parsed, "origin") && !finiteTuple(parsed.origin, 3)) ||
      (Object.prototype.hasOwnProperty.call(parsed, "orientation") && !finiteTuple(parsed.orientation, 4)) ||
      (Object.prototype.hasOwnProperty.call(parsed, "color") && (typeof parsed.color !== "string" || !boundedUtf8(parsed.color, WORLD_FILL_COLOR_MAX_BYTES))) ||
      (Object.prototype.hasOwnProperty.call(parsed, "opacity") && parsed.opacity !== 0.35)
    ) {
      return null;
    }
    const candidateGhost = diagnostic?.candidateGhost;
    const candidateGhostRecord =
      candidateGhost === null ||
      (typeof candidateGhost === "object" &&
        !Array.isArray(candidateGhost) &&
        censusAllowedOwnKeys(candidateGhost, WORLD_FILL_GHOST_KEYS) === WORLD_FILL_GHOST_KEYS.size &&
        typeof candidateGhost.targetVortexFullId === "string" &&
        typeof candidateGhost.objectKindId === "string" &&
        nonnegativeInteger(candidateGhost.sourceVortexIndex) &&
        typeof candidateGhost.meshUrl === "string" &&
        finiteTuple(candidateGhost.origin, 3) &&
        finiteTuple(candidateGhost.orientation, 4));
    if (
      !Number.isSafeInteger(diagnostic.operation) ||
      diagnostic.operation <= 0 ||
      !Number.isSafeInteger(diagnostic.baseRevision) ||
      diagnostic.baseRevision <= 0 ||
      !Number.isSafeInteger(diagnostic.registryGeneration) ||
      diagnostic.registryGeneration <= 0 ||
      !Number.isSafeInteger(diagnostic.generation) ||
      diagnostic.generation <= 0 ||
      !Number.isSafeInteger(diagnostic.sequence) ||
      diagnostic.sequence < 0 ||
      typeof diagnostic.stage !== "string" ||
      typeof diagnostic.statusLabel !== "string" ||
      diagnostic.statusLabel.length === 0 ||
      !boundedUtf8(diagnostic.statusLabel, WORLD_FILL_STATUS_LABEL_MAX_BYTES) ||
      !nullableString(diagnostic.targetVortexFullId) ||
      !nullableString(diagnostic.candidateObjectKindId) ||
      !nullableString(diagnostic.currentPairObjectId) ||
      !nullableString(diagnostic.rejectionReason) ||
      !nonnegativeInteger(diagnostic.collisionCount) ||
      !nonnegativeInteger(diagnostic.sampleCursor) ||
      !nonnegativeInteger(diagnostic.insideBoth) ||
      !nonnegativeInteger(diagnostic.targetCursor) ||
      !nonnegativeInteger(diagnostic.candidateCursor) ||
      !nonnegativeInteger(diagnostic.acceptedCount) ||
      !nonnegativeInteger(diagnostic.totalCount) ||
      !nonnegativeInteger(diagnostic.searchCount) ||
      !nonnegativeInteger(diagnostic.rejectedCount) ||
      typeof diagnostic.truncated !== "boolean" ||
      !Array.isArray(diagnostic.candidatePage) ||
      diagnostic.candidatePage.length !== 8 ||
      !diagnostic.candidatePage.every(nullableString) ||
      (diagnostic.lastSample !== null && (!Array.isArray(diagnostic.lastSample) || diagnostic.lastSample.length !== 3 || !diagnostic.lastSample.every(Number.isFinite))) ||
      !candidateGhostRecord ||
      (diagnostic.candidateGhost !== null &&
        (diagnostic.candidateGhost.targetVortexFullId !== parsed.targetVortexFullId ||
          diagnostic.candidateGhost.objectKindId !== parsed.objectKindId ||
          diagnostic.candidateGhost.sourceVortexIndex !== parsed.sourceVortexIndex ||
          diagnostic.candidateGhost.meshUrl !== parsed.meshUrl ||
          !equalTuple(diagnostic.candidateGhost.origin, parsed.origin) ||
          !equalTuple(diagnostic.candidateGhost.orientation, parsed.orientation)))
    ) {
      return null;
    }
    return parsed;
  } catch {
    return null;
  }
}

type WorldContextMenuTarget = { readonly kind: "vortex" | "object" | "reference"; readonly id: string };

/** @emoji 🖱️ Resolves which entity a plain right-click should select-then-open a menu for, by priority: hovered vortex, object component, reference, then object. */
export function resolveWorldContextMenuTarget(interaction: WorldInteractionRecord, selection: WorldSelectionRecord): WorldContextMenuTarget | null {
  if (interaction.hoveredVortexFullId) return { kind: "vortex", id: interaction.hoveredVortexFullId };
  if (selection.hoveredComponent?.objectId) return { kind: "object", id: selection.hoveredComponent.objectId };
  const hoveredId = selection.hoveredId;
  if (hoveredId?.startsWith("reference:")) return { kind: "reference", id: hoveredId.slice("reference:".length) };
  if (hoveredId) return { kind: "object", id: hoveredId };
  return null;
}

/** @emoji 🪧️ The `surface` half of one world right-click's {@link PluginContextMenuRequest}: the entity under
 * the pointer as the single `hits` row, and the painted selection as per-domain `selection` groups. The hit
 * target rides THIS request — an editor reads it straight off `ContextMenuSurfaceTarget.hits` — so no
 * separate target-recording dispatch precedes the menu. `contextMenuAt`, the action that used to carry it,
 * no longer exists in any world-3d app (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM moved the
 * target onto the request); dispatching it anyway only produced an `undeclaredActionDiagnostic` drop on
 * every right-click. */
export function world3dContextMenuSurfaceV1(
  target: WorldContextMenuTarget | null,
  selection: Pick<WorldSelectionRecord, "ids" | "componentIds">,
): { readonly hits: readonly { readonly domain: string; readonly id: string }[]; readonly selection: readonly { readonly domain: string; readonly ids: readonly string[] }[] } {
  const groups: { readonly domain: string; readonly ids: readonly string[] }[] = [];
  if ((selection.ids?.length ?? 0) > 0) groups.push({ domain: "object", ids: [...(selection.ids ?? [])] });
  if ((selection.componentIds?.length ?? 0) > 0) groups.push({ domain: "feature", ids: (selection.componentIds ?? []).map(String) });
  return { hits: target ? [{ domain: target.kind, id: target.id }] : [], selection: groups };
}

/** @emoji 🚫️ Instance-mesh picking must be disabled for fill/brush engagements — otherwise a click meant for a vortex marker or a fill/voxel gesture falls through and selects/gumballs the underlying object instead. */
export function worldInstancePickBlocked(activeUtility: string | undefined): boolean {
  return activeUtility === "fill" || activeUtility === "brush" || activeUtility === "volumeBrush" || activeUtility === "surfaceBrush";
}

/** @emoji 🚫️ `undefined` keeps the default mesh raycast; a no-op replaces it so a blocked instance cannot steal sibling vortex hits. */
export function worldInstanceMeshRaycast(pickEnabled: boolean): (() => null) | undefined {
  return pickEnabled ? undefined : () => null;
}

/** @emoji 🚫️ Walks a GLB/instance root and disables mesh raycasts when pick is blocked. */
export function applyWorldInstanceMeshRaycast(
  root: { readonly traverse: (fn: (object: { readonly isMesh?: boolean; raycast: unknown }) => void) => void },
  pickEnabled: boolean,
  meshRaycast: unknown,
): void {
  const raycast = pickEnabled ? meshRaycast : () => null;
  root.traverse((object) => {
    if (!object.isMesh) return;
    object.raycast = raycast;
  });
}

const WORLD_VORTEX_DEFAULT_RADIUS = 0.36;

/** @emoji 🎯 Hit-proxy sphere stays `visible` so Three's raycaster does not skip it; radius is at least the published marker radius. */
export function worldVortexHitProxy(radius?: number): { readonly visible: true; readonly radius: number } {
  const published = radius ?? WORLD_VORTEX_DEFAULT_RADIUS;
  return { visible: true, radius: Math.max(published, WORLD_VORTEX_DEFAULT_RADIUS) };
}

/** 📜️ Guest fill progress before the first `fillBuildTick` is `{ done: true, count: 0 }` — requiring
 * `!done` starved the interval forever. Tick while Fill is the published interaction (or the host
 * tool) until a completed plan exists (`done` and `count > 0`). */
export function worldFillBuildShouldTick(activeUtility: string | undefined, fillBuild: { readonly done?: boolean; readonly count?: number } | undefined, activeToolId?: string | null): boolean {
  if (activeUtility !== "fill" && activeToolId !== "fill") return false;
  if (fillBuild == null) return true;
  return !fillBuild.done || (fillBuild.count ?? 0) === 0;
}

/** ⏱️ While an Isolated fill job holds the actor lock, only a queued UI poll may dispatch fillBuildTick. */
export function worldFillBuildHostTickAllowed(shouldTick: boolean, driving: boolean, pollDue: boolean): boolean {
  if (!shouldTick) return false;
  if (!driving) return true;
  return pollDue;
}

/** @emoji 🖱️ In brush mode or vertex selection mode, pointer-down on a vortex selects immediately; otherwise a click selects and a drag starts connect. */
export function resolveVortexPointerDownIntent(brushMode: boolean, selectionMode?: string): "select" | "click-or-drag" {
  return brushMode || selectionMode === "vertex" ? "select" : "click-or-drag";
}

/** @emoji 🧱️ Builds the `addBrushObject` action args from a parsed brush preview, or `null` if there is nothing to place yet. */
export function brushObjectPlacementArgs(preview: WorldBrushPreviewRecord | null): Record<string, unknown> | null {
  if (!preview) return null;
  return {
    targetVortexFullId: preview.targetVortexFullId,
    objectKindId: preview.objectKindId,
    sourceVortexIndex: preview.sourceVortexIndex ?? 0,
    origin: preview.origin,
    orientation: preview.orientation,
    scale: preview.scale,
  };
}

/** 🖼️ Guest may publish `brushPreviewJson` then wipe it on a hover-empty body (`gate reason=no-target`). Keep the last JSON per vortex while leftover/local hover still names that target. */
export function retainWorldBrushPreviewJsonV1(published: string | undefined, hoverId: string | null | undefined, retained: Readonly<Record<string, string>>): { readonly json: string; readonly retained: Record<string, string> } {
  const next: Record<string, string> = { ...retained };
  const parsed = parseWorldBrushPreview(published);
  if (parsed?.targetVortexFullId && published) next[parsed.targetVortexFullId] = published;
  if (parsed && published) return { json: published, retained: next };
  if (hoverId && next[hoverId]) return { json: next[hoverId], retained: next };
  return { json: "", retained: next };
}


function parseEngagementPreview(engagementPreviewJson: string | undefined): readonly WorldEngagementPreviewItem[] {
  return parseJsonArray<WorldEngagementPreviewItem>(engagementPreviewJson);
}

function scaleTuple(scale: WorldBrushPreviewRecord["scale"]): [number, number, number] {
  if (typeof scale === "number") return [scale, scale, scale];
  if (Array.isArray(scale) && scale.length >= 3) return [scale[0]!, scale[1]!, scale[2]!];
  return [1, 1, 1];
}

function geometryFromMesh(mesh: WorldMeshData) {
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(mesh.positions), 3));
  geometry.setAttribute("normal", new BufferAttribute(new Float32Array(mesh.normals), 3));
  if (mesh.uvs?.length) geometry.setAttribute("uv", new BufferAttribute(new Float32Array(mesh.uvs), 2));
  if (mesh.colors?.length) geometry.setAttribute("color", new BufferAttribute(new Float32Array(mesh.colors), 3));
  if (mesh.indices.length > 0) geometry.setIndex([...mesh.indices]);
  return geometry;
}

type VertexPickData = {
  readonly geometry: BufferGeometry;
  readonly vertexIds: readonly number[];
};

function buildVertexPickData(mesh: WorldMeshData): VertexPickData | null {
  if (!mesh.vertexIds?.length) return null;
  const positions: number[] = [];
  const vertexIds: number[] = [];
  const emitted = new Set<number>();
  for (let index = 0; index < mesh.vertexIds.length; index += 1) {
    const id = mesh.vertexIds[index]!;
    if (emitted.has(id)) continue;
    emitted.add(id);
    vertexIds.push(id);
    positions.push(mesh.positions[index * 3]!, mesh.positions[index * 3 + 1]!, mesh.positions[index * 3 + 2]!);
  }
  if (!positions.length) return null;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(positions), 3));
  return { geometry, vertexIds };
}

function buildEdgeGeometry(mesh: WorldMeshData): BufferGeometry | null {
  if (!mesh.edgePositions?.length) return null;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(mesh.edgePositions), 3));
  return geometry;
}

/** @emoji 🧵️ Curve/centerline meshes have edge samples but no shaded triangles — pick/hover must treat them as whole instances. */
export function isCurveOnlyWorldMesh(mesh: Pick<WorldMeshData, "indices" | "edgePositions">): boolean {
  return Boolean(mesh.edgePositions?.length) && !(mesh.indices?.length > 0);
}

function buildFaceOverlayGeometry(mesh: WorldMeshData, faceIds: ReadonlySet<number>): BufferGeometry | null {
  if (!mesh.faceIds?.length || !mesh.indices.length || faceIds.size === 0) return null;
  const positions: number[] = [];
  const normals: number[] = [];
  for (let faceIndex = 0; faceIndex < mesh.faceIds.length; faceIndex += 1) {
    const faceId = mesh.faceIds[faceIndex]!;
    if (!faceIds.has(faceId)) continue;
    const i0 = mesh.indices[faceIndex * 3] ?? 0;
    const i1 = mesh.indices[faceIndex * 3 + 1] ?? 0;
    const i2 = mesh.indices[faceIndex * 3 + 2] ?? 0;
    for (const index of [i0, i1, i2]) {
      positions.push(mesh.positions[index * 3]!, mesh.positions[index * 3 + 1]!, mesh.positions[index * 3 + 2]!);
      normals.push(mesh.normals[index * 3]!, mesh.normals[index * 3 + 1]!, mesh.normals[index * 3 + 2]!);
    }
  }
  if (!positions.length) return null;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(positions), 3));
  geometry.setAttribute("normal", new BufferAttribute(new Float32Array(normals), 3));
  return geometry;
}

/** @emoji 🖱️➡️ Approximates a picked face's in-plane size from its triangles' local bounding box, dropping the
 * smallest axis (roughly the one aligned with the face normal for axis-aligned primitive faces) — good
 * enough to size a push/pull tool's footprint without needing a true tangent-plane projection. */
function faceExtentFromMesh(mesh: WorldMeshData, faceId: number): readonly [number, number] | undefined {
  if (!mesh.faceIds?.length || !mesh.indices.length) return undefined;
  let minX = Infinity;
  let maxX = -Infinity;
  let minY = Infinity;
  let maxY = -Infinity;
  let minZ = Infinity;
  let maxZ = -Infinity;
  let found = false;
  for (let faceIndex = 0; faceIndex < mesh.faceIds.length; faceIndex += 1) {
    if (mesh.faceIds[faceIndex] !== faceId) continue;
    found = true;
    for (const corner of [0, 1, 2]) {
      const vertexIndex = mesh.indices[faceIndex * 3 + corner];
      if (vertexIndex == null) continue;
      const x = mesh.positions[vertexIndex * 3] ?? 0;
      const y = mesh.positions[vertexIndex * 3 + 1] ?? 0;
      const z = mesh.positions[vertexIndex * 3 + 2] ?? 0;
      minX = Math.min(minX, x);
      maxX = Math.max(maxX, x);
      minY = Math.min(minY, y);
      maxY = Math.max(maxY, y);
      minZ = Math.min(minZ, z);
      maxZ = Math.max(maxZ, z);
    }
  }
  if (!found) return undefined;
  const extents = [maxX - minX, maxY - minY, maxZ - minZ].sort((a, b) => b - a);
  return [extents[0] ?? 0.2, extents[1] ?? 0.2];
}

function buildEdgeOverlayGeometry(mesh: WorldMeshData, edgeIds: ReadonlySet<number>): BufferGeometry | null {
  if (!mesh.edgeIds?.length || !mesh.edgePositions?.length || edgeIds.size === 0) return null;
  const positions: number[] = [];
  for (let edgeIndex = 0; edgeIndex < mesh.edgeIds.length; edgeIndex += 1) {
    if (!edgeIds.has(mesh.edgeIds[edgeIndex]!)) continue;
    const base = edgeIndex * 6;
    positions.push(mesh.edgePositions[base]!, mesh.edgePositions[base + 1]!, mesh.edgePositions[base + 2]!, mesh.edgePositions[base + 3]!, mesh.edgePositions[base + 4]!, mesh.edgePositions[base + 5]!);
  }
  if (!positions.length) return null;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(positions), 3));
  return geometry;
}

function buildVertexOverlayGeometry(mesh: WorldMeshData, vertexIds: ReadonlySet<number>): BufferGeometry | null {
  const pick = buildVertexPickData(mesh);
  if (!pick) return null;
  const positions: number[] = [];
  for (let index = 0; index < pick.vertexIds.length; index += 1) {
    if (!vertexIds.has(pick.vertexIds[index]!)) continue;
    positions.push(pick.geometry.attributes.position!.getX(index), pick.geometry.attributes.position!.getY(index), pick.geometry.attributes.position!.getZ(index));
  }
  if (!positions.length) return null;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(positions), 3));
  return geometry;
}

function paintTextureUrl(base64: string): string {
  return `data:image/png;base64,${base64}`;
}

function PaintTexturedMesh({
  geometry,
  style,
  styleKind,
  textureBase64,
  flatShading,
  children,
  ...meshProps
}: {
  readonly geometry: BufferGeometry;
  readonly style: MeshStyleColors;
  readonly styleKind: MeshStyleKind;
  readonly textureBase64?: string;
  readonly flatShading?: boolean;
  readonly children?: React.ReactNode;
} & ComponentProps<"mesh">) {
  const paintMap = textureBase64 ? useLoader(TextureLoader, paintTextureUrl(textureBase64)) : null;
  // Per-vertex colors (e.g. FEM stress contours) multiply against the material's own `color` in
  // three.js, so white lets them show through unmodified — `style.meshColor` would otherwise tint them.
  const hasVertexColors = geometry.hasAttribute("color");
  const celebrating = styleKind === "celebrated" && !hasVertexColors;
  return (
    <mesh geometry={geometry} {...meshProps}>
      {celebrating ? (
        <CelebratingConicMaterial opacity={style.opacity} />
      ) : (
        <meshStandardMaterial
          key={worldMeshMaterialRevision(styleKind)}
          color={hasVertexColors ? "#ffffff" : style.meshColor}
          vertexColors={hasVertexColors}
          map={paintMap ?? undefined}
          side={DoubleSide}
          flatShading={flatShading}
          metalness={0}
          roughness={1}
          emissive={hasVertexColors ? "#000000" : style.meshColor}
          emissiveIntensity={hasVertexColors ? 0 : style.emissiveIntensity}
          transparent={style.opacity < 1}
          opacity={style.opacity}
        />
      )}
      {children}
    </mesh>
  );
}

//#region GlbMeshBounds
/** @emoji 📦️ Local-space extents of every GLB the scene has loaded, keyed by the mesh record's own
 * `url` and already carrying the {@link GLB_MESH_FRAME_ROTATION_X} frame rotation the instance group
 * applies — so an entry is exactly the eight local corners a pick or a marquee has to project.
 *
 * 🎯️ A URL-backed mesh record carries NO inline `data` (the guest publishes `{id, url}` for every
 * catalogue representation), so every screen-space hit test over it used to fall back to a unit cube —
 * or, in the marquee's case, to the single point `[0,0,0]`. Measured on the 180-object Nakagin tower,
 * fully rendered: `candidates=180 containing=0 hit=none boxes=["28x30@427,394", …]` on 20 of 20 clicks
 * spread over the whole pane, while R3F's own raycast against the same GLB geometry resolved an
 * instance on 14 of them (26/09/02/PUZZLE-3D-END-TO-END wave B46). The loader is the only place that
 * knows a GLB's real size, so it is the place that records it. */
const GLB_MESH_LOCAL_BOUNDS = new Map<string, readonly (readonly [number, number, number])[]>();

/** 📦️ The eight corners of one loaded GLB's frame-rotated AABB, in the instance group's local space. */
function glbMeshFrameCorners(scene: Object3D): readonly (readonly [number, number, number])[] {
  const frame = new Object3D();
  frame.rotation.x = GLB_MESH_FRAME_ROTATION_X;
  frame.updateMatrixWorld(true);
  scene.updateMatrixWorld(true);
  const box = new Box3().setFromObject(scene);
  if (box.isEmpty()) return [];
  const scratch = new Vector3();
  const rotated = new Box3().makeEmpty();
  for (const corner of [
    [box.min.x, box.min.y, box.min.z],
    [box.max.x, box.min.y, box.min.z],
    [box.min.x, box.max.y, box.min.z],
    [box.max.x, box.max.y, box.min.z],
    [box.min.x, box.min.y, box.max.z],
    [box.max.x, box.min.y, box.max.z],
    [box.min.x, box.max.y, box.max.z],
    [box.max.x, box.max.y, box.max.z],
  ] as const) {
    rotated.expandByPoint(scratch.set(corner[0], corner[1], corner[2]).applyMatrix4(frame.matrixWorld));
  }
  return [
    [rotated.min.x, rotated.min.y, rotated.min.z],
    [rotated.max.x, rotated.min.y, rotated.min.z],
    [rotated.min.x, rotated.max.y, rotated.min.z],
    [rotated.max.x, rotated.max.y, rotated.min.z],
    [rotated.min.x, rotated.min.y, rotated.max.z],
    [rotated.max.x, rotated.min.y, rotated.max.z],
    [rotated.min.x, rotated.max.y, rotated.max.z],
    [rotated.max.x, rotated.max.y, rotated.max.z],
  ];
}

/** 📦️ The local corners a screen-space hit test must use for one instance's mesh: the mesh's own inline
 * geometry when it has some, else the loaded GLB's recorded extents, else — only while a GLB is still
 * loading — the unit cube. */
export function world3dInstanceLocalCorners(
  meshData: WorldMeshData | undefined,
  meshUrl: string | undefined,
  bounds: ReadonlyMap<string, readonly (readonly [number, number, number])[]> = GLB_MESH_LOCAL_BOUNDS,
): readonly (readonly [number, number, number])[] {
  if (meshData && (meshData.positions.length >= 3 || (meshData.edgePositions?.length ?? 0) >= 3)) return meshBoundsCorners(meshData);
  const recorded = meshUrl ? bounds.get(meshUrl) : undefined;
  if (recorded && recorded.length === 8) return recorded;
  return [
    [-0.5, -0.5, -0.5],
    [0.5, -0.5, -0.5],
    [-0.5, 0.5, -0.5],
    [0.5, 0.5, -0.5],
    [-0.5, -0.5, 0.5],
    [0.5, -0.5, 0.5],
    [-0.5, 0.5, 0.5],
    [0.5, 0.5, 0.5],
  ];
}
//#endregion GlbMeshBounds

//#region GlbMeshStyling
/** 🎨️ EdgesGeometry cache keyed by source BufferGeometry — `gltf.scene.clone(true)` shares geometries across every per-instance clone of the same GLB, so this dedupes edge computation across instances. */
const GLB_EDGE_GEOMETRY_CACHE = new WeakMap<BufferGeometry, EdgesGeometry>();

/** 🎨️ Adds a border-color {@link EdgesGeometry} outline to every mesh under `root` (idempotent), using the shared {@link GLB_EDGE_GEOMETRY_CACHE}. */
function applyGlbMeshEdgeBorders(root: Object3D, borderColor: string): void {
  // 🧵️ Collect targets before mutating: `object.add(...)` during `traverse()` would splice the new
  // (itself a Mesh) child into the live `children` array traverse is still walking, so it gets visited
  // and outlined again — and again — recursing until the stack overflows.
  const targets: Mesh[] = [];
  root.traverse((object) => {
    if (!(object instanceof Mesh)) return;
    const geometry = object.geometry;
    if (!geometry || object.children.some((child) => child.userData[WORLD_MESH_OUTLINE_USER_DATA_KEY])) return;
    targets.push(object);
  });
  for (const object of targets) {
    let edges = GLB_EDGE_GEOMETRY_CACHE.get(object.geometry);
    if (!edges) {
      edges = new EdgesGeometry(object.geometry);
      GLB_EDGE_GEOMETRY_CACHE.set(object.geometry, edges);
    }
    const outline = new LineSegments(edges, new LineBasicMaterial({ color: new Color(borderColor) }));
    outline.userData[WORLD_MESH_OUTLINE_USER_DATA_KEY] = true;
    outline.scale.setScalar(1.001);
    object.add(outline);
  }
}

//#endregion GlbMeshStyling

function GlbInstanceMesh({
  url,
  color,
  emissive,
  emissiveIntensity,
  opacity,
  borderColor,
  material,
  shadowEnabled,
  revision,
  pickEnabled,
}: {
  readonly url: string;
  readonly color: string;
  readonly emissive: string;
  readonly emissiveIntensity: number;
  readonly opacity: number;
  readonly borderColor: string;
  readonly material?: WorldEnvironmentMaterialRecord;
  readonly shadowEnabled?: boolean;
  readonly revision: MeshStyleKind;
  readonly pickEnabled: boolean;
}) {
  const gltf = useLoader(GLTFLoader, meshAssetTransportUrl(url));
  const invalidate = useThree((state) => state.invalidate);
  if (!GLB_MESH_LOCAL_BOUNDS.has(url)) {
    const corners = glbMeshFrameCorners(gltf.scene);
    if (corners.length === 8) GLB_MESH_LOCAL_BOUNDS.set(url, corners);
  }
  const celebrating = revision === "celebrated";
  // 🎨️ Bake selection/hover paint into the clone itself. Imperative `color.set` after deselect was leaving
  // the previous selected tint until a later hover remounted materials — style deps must recreate the tree.
  const scene = useMemo(() => {
    const cloned = gltf.scene.clone(true);
    cloned.traverse((child) => {
      if (!(child instanceof Mesh)) return;
      if (celebrating) {
        child.material = createCelebratingConicMaterial(opacity);
      } else {
        child.material = new MeshStandardMaterial({
          color: new Color(color),
          emissive: new Color(emissive),
          emissiveIntensity,
          metalness: material?.metalness ?? 0,
          roughness: material?.roughness ?? 1,
          transparent: opacity < 1,
          opacity,
        });
      }
      child.castShadow = shadowEnabled === true;
      child.receiveShadow = shadowEnabled === true;
    });
    applyGlbMeshEdgeBorders(cloned, borderColor);
    return cloned;
  }, [gltf.scene, material?.metalness, material?.roughness, shadowEnabled, color, emissive, emissiveIntensity, opacity, borderColor, revision, celebrating]);
  useFrame(() => {
    if (!celebrating) return;
    const angle = celebrateConicAngleRadians();
    scene.traverse((child) => {
      if (!(child instanceof Mesh)) return;
      const mat = child.material;
      if (!(mat instanceof ShaderMaterial) || !mat.uniforms?.uAngle) return;
      mat.uniforms.uAngle.value = angle;
    });
    invalidate();
  });
  // 🎞️ Demand frameloop: useLoader / style remounts after the mount kick would otherwise leave transparent panes.
  useLayoutEffect(() => {
    invalidate();
  }, [invalidate, scene]);
  useLayoutEffect(() => {
    applyWorldInstanceMeshRaycast(scene, pickEnabled, Mesh.prototype.raycast);
  }, [scene, pickEnabled]);

  return (
    <group rotation={[GLB_MESH_FRAME_ROTATION_X, 0, 0]}>
      <primitive object={scene} />
    </group>
  );
}

function extractGlbCollisionMesh(gltf: Awaited<ReturnType<GLTFLoader["loadAsync"]>>): {
  readonly positions: number[];
  readonly indices: number[];
} {
  const frame = new Object3D();
  frame.rotation.x = GLB_MESH_FRAME_ROTATION_X;
  frame.updateMatrixWorld(true);
  const positions: number[] = [];
  const indices: number[] = [];
  let vertexOffset = 0;
  const scratch = new Vector3();
  gltf.scene.updateMatrixWorld(true);
  gltf.scene.traverse((child) => {
    if (!(child instanceof Mesh)) return;
    const geometry = child.geometry;
    const positionAttr = geometry.getAttribute("position");
    if (!positionAttr) return;
    const worldMatrix = frame.matrixWorld.clone().multiply(child.matrixWorld);
    for (let index = 0; index < positionAttr.count; index += 1) {
      scratch.fromBufferAttribute(positionAttr, index).applyMatrix4(worldMatrix);
      positions.push(scratch.x, scratch.y, scratch.z);
    }
    const indexAttr = geometry.index;
    if (indexAttr) {
      for (let index = 0; index < indexAttr.count; index += 1) {
        indices.push(indexAttr.getX(index) + vertexOffset);
      }
    } else {
      for (let index = 0; index < positionAttr.count; index += 3) {
        indices.push(vertexOffset + index, vertexOffset + index + 1, vertexOffset + index + 2);
      }
    }
    vertexOffset += positionAttr.count;
  });
  return { positions, indices };
}

/** 🥽️ Announces one loaded GLB's collision geometry to the guest. `revision` is the ONLY thing that
 * re-announces an already-loaded mesh: the GLB is `useLoader`-cached for the page's lifetime, so without
 * it the announcement happens exactly once per mount and a guest that lost the geometry (a restored
 * actor, whose mesh store is not part of any checkpoint) could never be told again — see
 * {@link Puzzle3dBrushMeshRegistry}. */
function BrushMeshRegistrar({ url, revision, onRegister }: { readonly url: string; readonly revision: number; readonly onRegister: (url: string, positions: number[], indices: number[]) => void }) {
  const gltf = useLoader(GLTFLoader, meshAssetTransportUrl(url));
  useEffect(() => {
    const mesh = extractGlbCollisionMesh(gltf);
    if (mesh.positions.length === 0 || mesh.indices.length === 0) return;
    onRegister(url, mesh.positions, mesh.indices);
  }, [gltf, onRegister, revision, url]);
  return null;
}

/** 🎛️ True when `mode` is an explicit transform-gumball utility (`move`/`rotate`/`scale`/`transform`) — never treat a missing/unknown mode as move. */
export function isWorldTransformGumballMode(mode: string | undefined): boolean {
  return mode === "move" || mode === "rotate" || mode === "scale" || mode === "transform";
}

function gumballKindForTransformMode(transformMode: string | undefined, handleKind?: GumballHandleKind): "translate" | "rotate" | "scale" {
  if (transformMode === "transform" && handleKind != null) {
    return gumballHandleKindToTransformMode(handleKind);
  }
  if (transformMode === "rotate") return "rotate";
  if (transformMode === "scale") return "scale";
  return "translate";
}

const GUMBALL_TRANSFORM_EPSILON = 1e-6;

/** 🕹️ Leftover/object ids for gumball `translateSelection` — never component face ids. */
export function world3dGumballSelectionArgsV1(selection: {
  readonly ids?: readonly string[];
  readonly componentIds?: readonly number[];
  readonly selectionMode?: string;
  readonly granularity?: string;
}): { readonly mode: string; readonly ids: readonly string[] } {
  const leftoverIds = selection.ids ?? [];
  return {
    mode: selection.selectionMode ?? selection.granularity ?? "object",
    ids: leftoverIds.length > 0 ? leftoverIds : (selection.componentIds ?? []).map(String),
  };
}

/** @emoji 🎛️ Builds one incremental `translateSelection` / `rotateSelection` / `scaleSelection` dispatch from consecutive gumball poses. */
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

/** @emoji ⚡️ Local mid-drag gumball preview delta — applied imperatively to selected instance roots so meshes track the pointer without a WASM/React round-trip (same instant path as catalogue drop ghosts). */
export type WorldGumballLivePreviewDelta =
  | { readonly kind: "translate"; readonly dx: number; readonly dy: number; readonly dz: number }
  | { readonly kind: "rotate"; readonly qx: number; readonly qy: number; readonly qz: number; readonly qw: number }
  | { readonly kind: "scale"; readonly sx: number; readonly sy: number; readonly sz: number };

/** @emoji ⚡️ Absolute start→current gumball preview delta for local instance transforms. */
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

type WorldGumballLivePose = {
  readonly position: readonly [number, number, number];
  readonly quaternion: readonly [number, number, number, number];
  readonly scale: readonly [number, number, number];
};

/** @emoji ⚡️ Applies a local gumball preview delta onto a drag-start instance pose (matches puzzle/lowpoly/CAD scratch translate/rotate/scale semantics). */
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

/** @emoji ⚡️ Writes a live gumball preview pose onto a Three.js instance root. */
export function applyGumballLivePreviewPoseToObject3D(target: Object3D, pose: WorldGumballLivePose): void {
  target.position.set(pose.position[0], pose.position[1], pose.position[2]);
  target.quaternion.set(pose.quaternion[0], pose.quaternion[1], pose.quaternion[2], pose.quaternion[3]);
  target.scale.set(pose.scale[0], pose.scale[1], pose.scale[2]);
  target.updateMatrixWorld(true);
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

/** @emoji 🎛️ Transform-mode gumball config intersected with the planar subset implied by a window projection. */
export function worldGumballConfigForProjection(mode: string, projectionSpec?: WorldProjectionSpec): GumballConfig {
  return gumballConfigForTransformMode(mode, worldProjectionGumballPlane(projectionSpec));
}

/** @emoji 🫥️ True while the canvas is hovered, or always true when `enabled` is false (driver gumball reveal is `"always"`). Listens directly on the R3F canvas DOM element — no prop drilling into the scene graph. */
function useUiCanvasHovered(enabled: boolean): boolean {
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

function SceneGumball({
  target,
  config,
  active,
  onDraggingChanged,
  onDragStart,
  onDrag,
  onDragEnd,
}: {
  readonly target?: readonly [number, number, number];
  readonly config: GumballConfig;
  readonly active: boolean;
  readonly onDraggingChanged: (dragging: boolean) => void;
  readonly onDragStart?: (kind: GumballHandleKind, before: GumballPose) => void;
  readonly onDrag?: (kind: GumballHandleKind, pose: GumballPose) => void;
  readonly onDragEnd: (kind: GumballHandleKind, before: GumballPose, after: GumballPose) => void;
}) {
  const pivotRef = useRef<Object3D>(new Object3D());
  const draggingRef = useRef(false);
  const [ready, setReady] = useState(false);
  const driver = useUiDriver();
  const hovered = useUiCanvasHovered(driver.gumball === "hover");
  useEffect(() => {
    if (!target || draggingRef.current) return;
    pivotRef.current.position.set(target[0], target[1], target[2]);
    pivotRef.current.quaternion.set(0, 0, 0, 1);
    pivotRef.current.scale.set(1, 1, 1);
    pivotRef.current.updateMatrixWorld(true);
    setReady(true);
  }, [target]);
  if (!active || !target || !ready || !hovered) return null;
  return (
    <>
      <primitive object={pivotRef.current} />
      <UnifiedGumball
        target={pivotRef.current}
        config={config}
        onDraggingChanged={(dragging) => {
          draggingRef.current = dragging;
          onDraggingChanged(dragging);
        }}
        onDragStart={onDragStart}
        onDrag={onDrag}
        onDragEnd={(kind, before, after) => {
          onDragEnd(kind, before, after);
          pivotRef.current.position.set(after.position[0], after.position[1], after.position[2]);
          pivotRef.current.quaternion.set(after.quaternion[0], after.quaternion[1], after.quaternion[2], after.quaternion[3]);
          pivotRef.current.scale.set(after.scale[0], after.scale[1], after.scale[2]);
          pivotRef.current.updateMatrixWorld(true);
          draggingRef.current = false;
        }}
      />
    </>
  );
}

const WorldInstanceNode = reactHostPort.memo(function WorldInstanceNode({
  instance,
  index,
  meshRecord,
  meshData,
  geometry,
  borderGeometry,
  palette,
  vertexPick,
  edgeGeometry,
  paintTextureBase64,
  position,
  scale,
  quaternion,
  targets,
  activeObjectId,
  selectionMode,
  selectedComponentIds,
  previewComponentIds,
  hoveredComponent,
  showEdges,
  pickEnabled,
  onPaintAt,
  paintFromHit,
  flatShading,
  onInstancePointerDown,
  onInstancePointerMove,
  onWorldPick,
  onComponentHover,
  mergeMode,
  previewInstanceSelected,
  environmentMaterial,
  environmentShadowEnabled,
  faceDragActive,
  onFaceDragStart,
  onRootRef,
}: {
  readonly instance: WorldInstanceRecord;
  readonly index: number;
  readonly meshRecord?: WorldMeshRecord;
  readonly meshData?: WorldMeshData;
  readonly geometry?: BufferGeometry;
  /** 🎨️ Shared per-meshId edge outline geometry (see {@link WorldInstancesLayer}'s `geometries` memo); never rebuilt per instance. */
  readonly borderGeometry?: EdgesGeometry;
  readonly palette: MeshStylePalette;
  readonly vertexPick: VertexPickData | null;
  readonly edgeGeometry: BufferGeometry | null;
  readonly paintTextureBase64?: string;
  readonly position: readonly [number, number, number];
  readonly scale: readonly [number, number, number];
  readonly quaternion?: Quaternion;
  readonly targets: WorldSelectionTargets;
  readonly activeObjectId?: string;
  readonly selectionMode: string;
  readonly selectedComponentIds: ReadonlySet<number>;
  readonly previewComponentIds: ReadonlySet<number>;
  readonly hoveredComponent?: WorldHoverComponent;
  readonly showEdges?: boolean;
  readonly pickEnabled: boolean;
  readonly onPaintAt?: (objectId: string, u: number, v: number) => void;
  readonly paintFromHit: (objectId: string, mesh: WorldMeshData, event: { faceIndex?: number | null; uv?: { x: number; y: number } }) => void;
  readonly flatShading?: boolean;
  readonly onInstancePointerDown: (id: string, index: number, event: { shiftKey: boolean; ctrlKey: boolean; metaKey: boolean }) => void;
  readonly onInstancePointerMove: (id: string | null) => void;
  readonly onWorldPick: (args: { granularity: string; id: number; merge: string; objectId?: string }) => void;
  readonly onComponentHover: (args: { objectId: string; mode: string; id: number } | null) => void;
  readonly mergeMode: (event: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => string;
  /** 🖱️➡️ When true, pointer-down on an already-selected face starts a push/pull drag instead of falling through to selection/orbit. */
  readonly faceDragActive?: boolean;
  readonly onFaceDragStart?: (args: { objectId: string; faceId: number; normal: readonly [number, number, number]; point: readonly [number, number, number]; faceExtent?: readonly [number, number] }) => void;
  /** Live marquee-drag merged selection state for this instance; undefined when no drag is in progress. */
  readonly previewInstanceSelected?: boolean;
  readonly environmentMaterial?: WorldEnvironmentMaterialRecord;
  readonly environmentShadowEnabled?: boolean;
  /** ⚡️ Registers the instance root group for imperative mid-drag gumball live preview. */
  readonly onRootRef?: (id: string, group: Group | null) => void;
}) {
  const rootRef = useCallback(
    (group: Group | null) => {
      onRootRef?.(instance.id, group);
    },
    [instance.id, onRootRef],
  );
  const chrome = useWorldInstanceChrome(instance.id, instance.objectKind);
  const isActiveObject = instance.id === activeObjectId;
  const colors = semanticColorsFromPalette(palette);
  const celebratingIds = useCelebratingWorldInstanceIds();
  const styleKind = resolveMeshSelectionPreviewStyle(
    { ...instance, selected: chrome.selected, hovered: chrome.hovered, highlighted: chrome.highlighted, celebrating: celebratingIds.has(instance.id) },
    chrome.previewSelected ?? previewInstanceSelected,
  );
  const style = palette[styleKind];
  const glbUsesEnvironmentColor = styleKind === "neutral" && environmentMaterial?.color != null;
  const glbColor = glbUsesEnvironmentColor ? environmentMaterial!.color! : style.meshColor;
  const glbEmissive = glbUsesEnvironmentColor && environmentMaterial?.emissive ? environmentMaterial.emissive : style.meshColor;
  const glbEmissiveIntensity = glbUsesEnvironmentColor && environmentMaterial?.emissive ? (environmentMaterial.emissiveIntensity ?? 1) : style.emissiveIntensity;
  const instancePickEnabled = pickEnabled && !instance.disabled;
  const lockedClickClears = instance.disabled === true;
  const hoveredFaceId = hoveredComponent?.mode === "face" && hoveredComponent.objectId === instance.id ? hoveredComponent.id : undefined;
  const hoveredVertexId = hoveredComponent?.mode === "vertex" && hoveredComponent.objectId === instance.id ? hoveredComponent.id : undefined;
  const hoveredEdgeId = hoveredComponent?.mode === "edge" && hoveredComponent.objectId === instance.id ? hoveredComponent.id : undefined;
  const selectedFaceIds = isActiveObject && selectionMode === "face" ? selectedComponentIds : new Set<number>();
  const selectedVertexIds = isActiveObject && selectionMode === "vertex" ? selectedComponentIds : new Set<number>();
  const selectedEdgeIds = isActiveObject && selectionMode === "edge" ? selectedComponentIds : new Set<number>();
  const previewFaceIds = isActiveObject && selectionMode === "face" ? previewComponentIds : new Set<number>();
  const previewVertexIds = isActiveObject && selectionMode === "vertex" ? previewComponentIds : new Set<number>();
  const previewEdgeIds = isActiveObject && selectionMode === "edge" ? previewComponentIds : new Set<number>();
  const facePreviewOverlay = meshData && previewFaceIds.size > 0 ? buildFaceOverlayGeometry(meshData, previewFaceIds) : null;
  const edgePreviewOverlay = meshData && previewEdgeIds.size > 0 ? buildEdgeOverlayGeometry(meshData, previewEdgeIds) : null;
  const vertexPreviewOverlay = meshData && previewVertexIds.size > 0 ? buildVertexOverlayGeometry(meshData, previewVertexIds) : null;
  const faceSelectedOverlay = meshData ? buildFaceOverlayGeometry(meshData, selectedFaceIds) : null;
  const faceHoveredOverlay = meshData && hoveredFaceId != null ? buildFaceOverlayGeometry(meshData, new Set([hoveredFaceId])) : null;
  const edgeSelectedOverlay = meshData ? buildEdgeOverlayGeometry(meshData, selectedEdgeIds) : null;
  const edgeHoveredOverlay = meshData && hoveredEdgeId != null ? buildEdgeOverlayGeometry(meshData, new Set([hoveredEdgeId])) : null;
  const vertexSelectedOverlay = meshData ? buildVertexOverlayGeometry(meshData, selectedVertexIds) : null;
  const vertexHoveredOverlay = meshData && hoveredVertexId != null ? buildVertexOverlayGeometry(meshData, new Set([hoveredVertexId])) : null;

  const hasShadedMesh = Boolean(meshData && geometry && meshData.indices.length > 0);
  const isCurveOnly = Boolean(meshData && isCurveOnlyWorldMesh(meshData));
  const curveLineWidth = styleKind === "neutral" ? 2 : 4;

  return (
    <group ref={rootRef} position={position as [number, number, number]} scale={scale as [number, number, number]} quaternion={quaternion}>
      {meshData ? (
        <>
          {hasShadedMesh && geometry ? (
            <PaintTexturedMesh
            geometry={geometry}
            style={style}
            styleKind={styleKind}
            textureBase64={paintTextureBase64}
            flatShading={flatShading}
            raycast={worldInstanceMeshRaycast(instancePickEnabled)}
            onPointerDown={(event) => {
              if (onPaintAt || !faceDragActive || !onFaceDragStart || !event.face) return;
              if (!(targets.face && event.faceIndex != null && meshData.faceIds?.[event.faceIndex] != null)) return;
              const faceId = meshData.faceIds[event.faceIndex]!;
              if (!(isActiveObject && selectionMode === "face" && selectedComponentIds.has(faceId))) return;
              event.stopPropagation();
              const normal = event.face.normal.clone().transformDirection(event.object.matrixWorld).normalize();
              onFaceDragStart({
                objectId: instance.id,
                faceId,
                normal: [normal.x, normal.y, normal.z],
                point: [event.point.x, event.point.y, event.point.z],
                faceExtent: faceExtentFromMesh(meshData, faceId),
              });
            }}
            onClick={(event) => {
              if (onPaintAt) {
                paintFromHit(instance.id, meshData, event);
                return;
              }
              if (lockedClickClears) {
                event.stopPropagation();
                onInstancePointerDown(instance.id, index, event);
                return;
              }
              if (!instancePickEnabled) return;
              event.stopPropagation();
              if (targets.face && event.faceIndex != null && meshData.faceIds?.[event.faceIndex] != null) {
                onWorldPick({
                  granularity: "face",
                  id: meshData.faceIds[event.faceIndex]!,
                  merge: mergeMode(event),
                  objectId: instance.id,
                });
              } else if (targets.mesh) {
                onInstancePointerDown(instance.id, index, event);
              }
            }}
            onPointerMove={(event) => {
              if (onPaintAt) {
                if ((event.buttons & 1) !== 0) paintFromHit(instance.id, meshData, event);
                return;
              }
              if (!instancePickEnabled) return;
              event.stopPropagation();
              if (targets.face && event.faceIndex != null && meshData.faceIds?.[event.faceIndex] != null) {
                onComponentHover({
                  objectId: instance.id,
                  mode: "face",
                  id: meshData.faceIds[event.faceIndex]!,
                });
              } else {
                onInstancePointerMove(instance.id);
              }
            }}
            onPointerOut={() => {
              onInstancePointerMove(null);
              onComponentHover(null);
            }}
          ></PaintTexturedMesh>
          ) : null}
          {hasShadedMesh && borderGeometry && (showEdges ?? true) && !edgeGeometry ? (
            <lineSegments geometry={borderGeometry} scale={1.001} raycast={() => null} renderOrder={2}>
              <lineBasicMaterial color={style.lineColor} depthTest={false} />
            </lineSegments>
          ) : null}
          {(targets.edge || isCurveOnly || (showEdges ?? true) || (selectionMode === "mesh" && selectedComponentIds.size > 0)) && edgeGeometry ? (
            <lineSegments
              geometry={edgeGeometry}
              renderOrder={2}
              onClick={(event) => {
                if (lockedClickClears) {
                  event.stopPropagation();
                  onInstancePointerDown(instance.id, index, event);
                  return;
                }
                if (!instancePickEnabled) return;
                event.stopPropagation();
                // 🧵️ Centerline/curve objects are the model-definition instances — never decompose into edge components.
                if (isCurveOnly && targets.mesh) {
                  onInstancePointerDown(instance.id, index, event);
                  return;
                }
                if (meshData.edgeIds?.length) {
                  const edgeIndex = Math.floor((event.index ?? 0) / 2);
                  const edgeId = meshData.edgeIds[edgeIndex];
                  if (edgeId != null) {
                    onWorldPick({ granularity: "edge", id: edgeId, merge: mergeMode(event), objectId: instance.id });
                    return;
                  }
                }
                if (targets.mesh) {
                  onInstancePointerDown(instance.id, index, event);
                }
              }}
              onPointerMove={(event) => {
                if (!instancePickEnabled) return;
                event.stopPropagation();
                if (isCurveOnly) {
                  onInstancePointerMove(instance.id);
                  return;
                }
                if (!meshData?.edgeIds?.length) return;
                const edgeIndex = Math.floor((event.index ?? 0) / 2);
                const edgeId = meshData.edgeIds[edgeIndex];
                if (edgeId == null) return;
                onComponentHover({ objectId: instance.id, mode: "edge", id: edgeId });
              }}
              onPointerOut={() => {
                if (isCurveOnly) onInstancePointerMove(null);
                onComponentHover(null);
              }}
            >
              <lineBasicMaterial color={style.lineColor} linewidth={isCurveOnly ? curveLineWidth : 1} depthTest={false} />
            </lineSegments>
          ) : null}
          {targets.vertex && vertexPick ? (
            <points
              geometry={vertexPick.geometry}
              onClick={(event) => {
                if (lockedClickClears) {
                  event.stopPropagation();
                  onInstancePointerDown(instance.id, index, event);
                  return;
                }
                if (!instancePickEnabled) return;
                event.stopPropagation();
                const idx = event.index ?? 0;
                const vertexId = vertexPick.vertexIds[idx];
                if (vertexId == null) return;
                onWorldPick({ granularity: "vertex", id: vertexId, merge: mergeMode(event), objectId: instance.id });
              }}
              onPointerMove={(event) => {
                if (!instancePickEnabled) return;
                event.stopPropagation();
                const idx = event.index ?? 0;
                const vertexId = vertexPick.vertexIds[idx];
                if (vertexId == null) return;
                onComponentHover({ objectId: instance.id, mode: "vertex", id: vertexId });
              }}
              onPointerOut={() => onComponentHover(null)}
            >
              <pointsMaterial color={colors.edge} size={0.05} sizeAttenuation />
            </points>
          ) : null}
          {faceSelectedOverlay ? (
            <mesh geometry={faceSelectedOverlay} raycast={() => null}>
              <meshBasicMaterial color={colors.select} transparent opacity={0.62} side={DoubleSide} depthWrite={false} polygonOffset polygonOffsetFactor={-2} />
            </mesh>
          ) : null}
          {faceHoveredOverlay ? (
            <mesh geometry={faceHoveredOverlay} raycast={() => null}>
              <meshBasicMaterial color={colors.hover} transparent opacity={0.48} side={DoubleSide} depthWrite={false} polygonOffset polygonOffsetFactor={-3} />
            </mesh>
          ) : null}
          {facePreviewOverlay ? (
            <mesh geometry={facePreviewOverlay} raycast={() => null}>
              <meshBasicMaterial color={colors.hover} transparent opacity={0.36} side={DoubleSide} depthWrite={false} polygonOffset polygonOffsetFactor={-4} />
            </mesh>
          ) : null}
          {edgeSelectedOverlay ? (
            <lineSegments geometry={edgeSelectedOverlay} raycast={() => null} renderOrder={3}>
              <lineBasicMaterial color={colors.select} linewidth={3} depthTest={false} />
            </lineSegments>
          ) : null}
          {edgeHoveredOverlay ? (
            <lineSegments geometry={edgeHoveredOverlay} raycast={() => null} renderOrder={4}>
              <lineBasicMaterial color={colors.edgeHover} linewidth={3} depthTest={false} />
            </lineSegments>
          ) : null}
          {edgePreviewOverlay ? (
            <lineSegments geometry={edgePreviewOverlay} raycast={() => null} renderOrder={3}>
              <lineBasicMaterial color={colors.edgeHover} linewidth={2} depthTest={false} />
            </lineSegments>
          ) : null}
          {vertexSelectedOverlay ? (
            <points geometry={vertexSelectedOverlay} raycast={() => null}>
              <pointsMaterial color={colors.select} size={0.09} sizeAttenuation depthTest={false} />
            </points>
          ) : null}
          {vertexHoveredOverlay ? (
            <points geometry={vertexHoveredOverlay} raycast={() => null}>
              <pointsMaterial color={colors.hover} size={0.09} sizeAttenuation depthTest={false} />
            </points>
          ) : null}
          {vertexPreviewOverlay ? (
            <points geometry={vertexPreviewOverlay} raycast={() => null}>
              <pointsMaterial color={colors.hover} size={0.09} sizeAttenuation depthTest={false} />
            </points>
          ) : null}
        </>
      ) : meshRecord?.url ? (
        <group
          onPointerDown={(event) => {
            if (!instancePickEnabled && !lockedClickClears) return;
            event.stopPropagation();
            onInstancePointerDown(instance.id, index, event);
          }}
          onPointerMove={(event) => {
            if (!instancePickEnabled) return;
            event.stopPropagation();
            onInstancePointerMove(instance.id);
          }}
          onPointerOut={() => onInstancePointerMove(null)}
        >
          <Suspense fallback={null}>
            <GlbInstanceMesh
              key={worldMeshMaterialRevision(styleKind)}
              url={meshRecord.url}
              color={glbColor}
              emissive={glbEmissive}
              emissiveIntensity={glbEmissiveIntensity}
              opacity={style.opacity}
              borderColor={palette.neutral.lineColor}
              material={environmentMaterial}
              shadowEnabled={environmentShadowEnabled}
              revision={styleKind}
              pickEnabled={instancePickEnabled}
            />
          </Suspense>
        </group>
      ) : (
        <mesh
          raycast={worldInstanceMeshRaycast(instancePickEnabled)}
          onPointerDown={(event) => {
            if (!instancePickEnabled && !lockedClickClears) return;
            event.stopPropagation();
            onInstancePointerDown(instance.id, index, event);
          }}
        >
          <boxGeometry args={[1, 1, 1]} />
          <meshStandardMaterial key={worldMeshMaterialRevision(styleKind)} color={style.meshColor} metalness={0} roughness={1} emissive={style.meshColor} emissiveIntensity={style.emissiveIntensity} transparent={style.opacity < 1} opacity={style.opacity} />
        </mesh>
      )}
    </group>
  );
});
//#endregion WorldSceneParsing

//#region WorldInstancesLayer
function WorldInstancesLayer({
  instances,
  meshes,
  selection,
  persistentSelectionMode,
  palette,
  projectionSpec,
  onInstancePointerDown,
  onInstancePointerMove,
  onWorldPick,
  onComponentHover,
  onPaintAt,
  gumballDragActive,
  onGumballDraggingChanged,
  onGumballDragStart,
  onGumballDrag,
  onGumballDragEnd,
  onFaceDragStart,
  mergedComponentIds,
  mergedInstanceIds,
  blockPick,
  environment,
  revealCutoffs,
}: {
  readonly instances: readonly WorldInstanceRecord[];
  readonly meshes: readonly WorldMeshRecord[];
  readonly selection: WorldSelectionRecord;
  /** 🐚️ This shell's own `SelectionModeStore` value — see `resolveWorldMergeMode`'s doc. */
  readonly persistentSelectionMode: MergeMode;
  readonly palette: MeshStylePalette;
  readonly projectionSpec?: WorldProjectionSpec;
  readonly onInstancePointerDown: (id: string, index: number, event: { shiftKey: boolean; ctrlKey: boolean; metaKey: boolean }) => void;
  readonly onInstancePointerMove: (id: string | null) => void;
  readonly onWorldPick: (args: { granularity: string; id: number; merge: string; objectId?: string }) => void;
  readonly onComponentHover: (args: { objectId: string; mode: string; id: number } | null) => void;
  readonly onPaintAt?: (objectId: string, u: number, v: number) => void;
  readonly gumballDragActive: boolean;
  readonly onGumballDraggingChanged: (dragging: boolean) => void;
  readonly onGumballDragStart?: (kind: GumballHandleKind, before: GumballPose) => void;
  readonly onGumballDrag?: (kind: GumballHandleKind, pose: GumballPose) => void;
  readonly onGumballDragEnd: (kind: GumballHandleKind, before: GumballPose, after: GumballPose) => void;
  readonly onFaceDragStart?: (args: { objectId: string; faceId: number; normal: readonly [number, number, number]; point: readonly [number, number, number]; faceExtent?: readonly [number, number] }) => void;
  /** Live drag-preview merged component id set (null when no marquee drag is in progress). */
  readonly mergedComponentIds?: readonly number[] | null;
  /** Live drag-preview merged whole-instance id set (null when no marquee drag is in progress). */
  readonly mergedInstanceIds?: readonly string[] | null;
  /** Disables instance picking; passed for fill and brush engagements so a click meant for a vortex marker can't fall through and select/gumball the underlying object instead. */
  readonly blockPick?: boolean;
  readonly environment?: WorldEnvironmentRecord | null;
  /** 🪣️ Committed reveal cutoffs (`WorldInteractionRecord.revealCutoffs`) — reconciles `worldRevealCutoffStore` whenever the committed value changes; a live drag already wrote the store directly and this is then a same-value no-operation. */
  readonly revealCutoffs?: Readonly<Record<string, number>>;
}) {
  const meshById = useMemo(() => new Map(meshes.map((mesh) => [mesh.id, mesh])), [meshes]);
  const geometries = useMemo(() => {
    const map = new Map<string, BufferGeometry>();
    for (const mesh of meshes) {
      if (mesh.data) map.set(mesh.id, geometryFromMesh(mesh.data));
    }
    return map;
  }, [meshes]);
  /** 🎨️ Per-meshId border outline geometry, shared by every instance of that mesh — never rebuilt per instance. */
  const borderGeometries = useMemo(() => {
    const map = new Map<string, EdgesGeometry>();
    for (const [meshId, geometry] of geometries) map.set(meshId, new EdgesGeometry(geometry));
    return map;
  }, [geometries]);
  const vertexPickByMeshId = useMemo(() => {
    const map = new Map<string, VertexPickData | null>();
    for (const mesh of meshes) {
      if (mesh.data) map.set(mesh.id, buildVertexPickData(mesh.data));
    }
    return map;
  }, [meshes]);
  const edgeGeometryByMeshId = useMemo(() => {
    const map = new Map<string, BufferGeometry | null>();
    for (const mesh of meshes) {
      if (mesh.data) map.set(mesh.id, buildEdgeGeometry(mesh.data));
    }
    return map;
  }, [meshes]);
  const targets = useMemo(() => selection.targets ?? { mesh: true, vertex: false, edge: false, face: false }, [selection.targets]);
  const selectionMode = selection.selectionMode ?? selection.granularity ?? "mesh";
  const currentComponentIds = new Set(selection.componentIds ?? []);
  const mergedComponentIdsSet = mergedComponentIds ? new Set(mergedComponentIds) : null;
  // Still-selected (solid) = current ∩ merged when dragging; newly-added (preview tint) = merged − current.
  const selectedComponentIds = useMemo(
    () => (mergedComponentIdsSet ? new Set([...currentComponentIds].filter((id) => mergedComponentIdsSet.has(id))) : currentComponentIds),
    [currentComponentIds, mergedComponentIdsSet],
  );
  const previewComponentIds = useMemo(
    () => (mergedComponentIdsSet ? new Set([...mergedComponentIdsSet].filter((id) => !currentComponentIds.has(id))) : new Set<number>()),
    [currentComponentIds, mergedComponentIdsSet],
  );
  const mergedInstanceIdsSet = mergedInstanceIds ? new Set(mergedInstanceIds) : null;
  const selectedIds = selection.ids ?? [];
  const instanceChromeStore = useMemo(() => createWorldInstanceChromeStore(), []);
  reactHostPort.useLayoutEffect(() => {
    instanceChromeStore.setSnapshot({
      selectedIds: new Set(selection.ids ?? []),
      hoveredId: selection.hoveredId ?? null,
      hoveredKindId: selection.hoveredKindId ?? null,
      previewInstanceIds: mergedInstanceIdsSet,
    });
  }, [instanceChromeStore, mergedInstanceIdsSet, selection.hoveredId, selection.hoveredKindId, selection.ids]);
  const pickEnabled = !gumballDragActive && !onPaintAt && !blockPick && !mergedComponentIdsSet && !mergedInstanceIdsSet;
  const transformMode = selection.transformMode;
  const transformGumballMode = isWorldTransformGumballMode(transformMode);
  const gumballConfig = useMemo(() => {
    if (selection.gumballConfig) {
      const plane = worldProjectionGumballPlane(projectionSpec);
      return plane ? { ...selection.gumballConfig, plane } : selection.gumballConfig;
    }
    return worldGumballConfigForProjection(transformMode ?? "move", projectionSpec);
  }, [selection.gumballConfig, transformMode, projectionSpec]);
  const paintMode = selection.interactionMode === "paint";
  const gumballVisible = Boolean(selection.gumballActive) && transformGumballMode && !paintMode;
  const invalidate = useThree((state) => state.invalidate);
  const instanceRootsRef = useRef(new Map<string, Group>());
  const gumballLiveBasesRef = useRef(new Map<string, WorldGumballLivePose>());
  /** ⚡️ Final local poses held across the post-drag React frame(s) until `instancesJson` catches up — prevents a one-frame snap-back to the pre-drag pose. */
  const gumballCommitHoldRef = useRef(new Map<string, WorldGumballLivePose>());
  const gumballLiveStartPoseRef = useRef<GumballPose | null>(null);
  const gumballLivePoseRef = useRef<GumballPose | null>(null);
  const gumballLiveKindRef = useRef<GumballHandleKind | null>(null);

  const registerInstanceRoot = useCallback((id: string, group: Group | null) => {
    if (group) instanceRootsRef.current.set(id, group);
    else instanceRootsRef.current.delete(id);
  }, []);

  /** 🪣️ Imperatively shows/hides reveal-tagged instance roots per the live cutoff — zero React re-render,
   * zero WASM round trip. Re-runs on every instance-list change (new roots to tag) and on every live
   * cutoff update from `worldRevealCutoffStore` (a slider drag, or the commit reconciliation below). */
  const applyRevealCutoff = useCallback(() => {
    const cutoff = worldRevealCutoffStore.get(PUZZLE3D_FILL_REVEAL_GROUP_ID) ?? revealCutoffs?.[PUZZLE3D_FILL_REVEAL_GROUP_ID];
    let changed = false;
    for (const instance of instances) {
      if (instance.revealIndex == null) continue;
      const root = instanceRootsRef.current.get(instance.id);
      if (!root) continue;
      const visible = cutoff === undefined || instance.revealIndex < cutoff;
      if (root.visible !== visible) {
        root.visible = visible;
        changed = true;
      }
    }
    if (changed) invalidate();
  }, [instances, revealCutoffs, invalidate]);

  useLayoutEffect(() => {
    applyRevealCutoff();
    return worldRevealCutoffStore.subscribe(PUZZLE3D_FILL_REVEAL_GROUP_ID, applyRevealCutoff);
  }, [applyRevealCutoff]);

  /** 🪣️ Reconciles the shared store from the plugin's committed cutoff — only when the *committed*
   * value itself changes. A live slider drag already wrote the store directly; fillBuildTick refreshes
   * rewrite `interactionJson` (and a new `revealCutoffs` object identity) with the same committed count,
   * and must not clobber the in-progress drag back to that stale value (which hid fill objects mid-gesture). */
  const committedRevealCutoffsRef = useRef<Readonly<Record<string, number>>>({});
  useEffect(() => {
    if (!revealCutoffs) return;
    reconcileCommittedRevealCutoffs(worldRevealCutoffStore, committedRevealCutoffsRef, revealCutoffs);
  }, [revealCutoffs]);

  const writeGumballPreviewPoses = useCallback(
    (poses: ReadonlyMap<string, WorldGumballLivePose>) => {
      for (const [id, pose] of poses) {
        const root = instanceRootsRef.current.get(id);
        if (!root) continue;
        applyGumballLivePreviewPoseToObject3D(root, pose);
      }
      invalidate();
    },
    [invalidate],
  );

  const applyGumballLivePreview = useCallback(
    (before: GumballPose, after: GumballPose, handleKind: GumballHandleKind | null) => {
      const delta = gumballLivePreviewDeltaBetweenPoses(transformMode, before, after, handleKind ?? undefined);
      const poses = new Map<string, WorldGumballLivePose>();
      for (const [id, base] of gumballLiveBasesRef.current) {
        poses.set(id, delta ? applyGumballLivePreviewDeltaToPose(base, delta) : base);
      }
      writeGumballPreviewPoses(poses);
      return poses;
    },
    [transformMode, writeGumballPreviewPoses],
  );

  useLayoutEffect(() => {
    const before = gumballLiveStartPoseRef.current;
    const after = gumballLivePoseRef.current;
    if (gumballDragActive && before && after) {
      applyGumballLivePreview(before, after, gumballLiveKindRef.current);
      return;
    }
    const hold = gumballCommitHoldRef.current;
    if (hold.size === 0) return;
    writeGumballPreviewPoses(hold);
    let allMatch = true;
    for (const [id, pose] of hold) {
      const instance = instances.find((entry) => entry.id === id);
      if (!instance) {
        allMatch = false;
        continue;
      }
      const position = instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0];
      const scale = instance.scale ?? [1, 1, 1];
      const rotation = instance.rotation ?? [0, 0, 0, 1];
      if (
        Math.abs(position[0] - pose.position[0]) > GUMBALL_TRANSFORM_EPSILON ||
        Math.abs(position[1] - pose.position[1]) > GUMBALL_TRANSFORM_EPSILON ||
        Math.abs(position[2] - pose.position[2]) > GUMBALL_TRANSFORM_EPSILON ||
        Math.abs(scale[0] - pose.scale[0]) > GUMBALL_TRANSFORM_EPSILON ||
        Math.abs(scale[1] - pose.scale[1]) > GUMBALL_TRANSFORM_EPSILON ||
        Math.abs(scale[2] - pose.scale[2]) > GUMBALL_TRANSFORM_EPSILON ||
        Math.abs(rotation[0] - pose.quaternion[0]) > GUMBALL_TRANSFORM_EPSILON ||
        Math.abs(rotation[1] - pose.quaternion[1]) > GUMBALL_TRANSFORM_EPSILON ||
        Math.abs(rotation[2] - pose.quaternion[2]) > GUMBALL_TRANSFORM_EPSILON ||
        Math.abs(rotation[3] - pose.quaternion[3]) > GUMBALL_TRANSFORM_EPSILON
      ) {
        allMatch = false;
      }
    }
    if (allMatch) hold.clear();
  }, [applyGumballLivePreview, gumballDragActive, instances, writeGumballPreviewPoses]);

  const handleGumballDraggingChanged = useCallback(
    (dragging: boolean) => {
      if (dragging) gumballCommitHoldRef.current.clear();
      onGumballDraggingChanged(dragging);
    },
    [onGumballDraggingChanged],
  );

  const handleGumballDragStart = useCallback(
    (kind: GumballHandleKind, before: GumballPose) => {
      const bases = new Map<string, WorldGumballLivePose>();
      for (const id of selectedIds) {
        const root = instanceRootsRef.current.get(id);
        if (!root) continue;
        bases.set(id, {
          position: [root.position.x, root.position.y, root.position.z],
          quaternion: [root.quaternion.x, root.quaternion.y, root.quaternion.z, root.quaternion.w],
          scale: [root.scale.x, root.scale.y, root.scale.z],
        });
      }
      gumballLiveBasesRef.current = bases;
      gumballCommitHoldRef.current.clear();
      gumballLiveStartPoseRef.current = before;
      gumballLivePoseRef.current = before;
      gumballLiveKindRef.current = kind;
      onGumballDragStart?.(kind, before);
    },
    [onGumballDragStart, selectedIds],
  );

  const handleGumballDrag = useCallback(
    (kind: GumballHandleKind, pose: GumballPose) => {
      const before = gumballLiveStartPoseRef.current ?? pose;
      gumballLiveStartPoseRef.current = before;
      gumballLivePoseRef.current = pose;
      gumballLiveKindRef.current = kind;
      applyGumballLivePreview(before, pose, kind);
      onGumballDrag?.(kind, pose);
    },
    [applyGumballLivePreview, onGumballDrag],
  );

  const handleGumballDragEnd = useCallback(
    (kind: GumballHandleKind, before: GumballPose, after: GumballPose) => {
      const start = gumballLiveStartPoseRef.current ?? before;
      const finals = applyGumballLivePreview(start, after, kind);
      gumballCommitHoldRef.current = finals;
      gumballLiveBasesRef.current.clear();
      gumballLiveStartPoseRef.current = null;
      gumballLivePoseRef.current = null;
      gumballLiveKindRef.current = null;
      onGumballDragEnd(kind, before, after);
    },
    [applyGumballLivePreview, onGumballDragEnd],
  );

  const mergeMode = (event: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => componentPickMergeMode(resolveWorldMergeMode(selection.selectionMergeMode, event, persistentSelectionMode));

  const paintFromHit = (objectId: string, mesh: WorldMeshData, event: { faceIndex?: number | null; uv?: { x: number; y: number } }) => {
    if (!onPaintAt) return;
    let u = event.uv?.x;
    let v = event.uv?.y;
    if (u == null || v == null) {
      if (event.faceIndex == null || !mesh.indices.length) return;
      const i0 = mesh.indices[event.faceIndex * 3] ?? 0;
      const i1 = mesh.indices[event.faceIndex * 3 + 1] ?? 0;
      const i2 = mesh.indices[event.faceIndex * 3 + 2] ?? 0;
      if (!mesh.uvs || mesh.uvs.length < 6) return;
      u = (mesh.uvs[i0 * 2]! + mesh.uvs[i1 * 2]! + mesh.uvs[i2 * 2]!) / 3;
      v = (mesh.uvs[i0 * 2 + 1]! + mesh.uvs[i1 * 2 + 1]! + mesh.uvs[i2 * 2 + 1]!) / 3;
    }
    onPaintAt(objectId, u, v);
  };

  return (
    <WorldInstanceChromeContext.Provider value={instanceChromeStore}>
    <WorldLayerStack>
      <group>
        {instances.map((instance, index) => {
          const meshId = instance.meshId ?? instance.id;
          const meshRecord = meshById.get(meshId);
          const meshData = meshRecord?.data;
          const geometry = geometries.get(meshId);
          const position = instance.position ?? [instance.x ?? index, instance.y ?? 0, instance.z ?? 0];
          const scale = instance.scale ?? [1, 1, 1];
          const rotation = instance.rotation;
          const quaternion = rotation ? new Quaternion(rotation[0], rotation[1], rotation[2], rotation[3]) : undefined;
          return (
            <WorldInstanceNode
              key={instance.id}
              instance={instance}
              previewInstanceSelected={undefined}
              index={index}
              meshRecord={meshRecord}
              meshData={meshData}
              geometry={geometry}
              borderGeometry={borderGeometries.get(meshId)}
              palette={palette}
              vertexPick={vertexPickByMeshId.get(meshId) ?? null}
              edgeGeometry={edgeGeometryByMeshId.get(meshId) ?? null}
              paintTextureBase64={meshData?.paintTextureBase64}
              position={position as [number, number, number]}
              scale={scale as [number, number, number]}
              quaternion={quaternion}
              targets={targets}
              activeObjectId={selection.activeObjectId}
              selectionMode={selectionMode}
              selectedComponentIds={selectedComponentIds}
              previewComponentIds={previewComponentIds}
              hoveredComponent={selection.hoveredComponent}
              showEdges={selection.showEdges}
              pickEnabled={pickEnabled}
              onPaintAt={onPaintAt}
              paintFromHit={paintFromHit}
              flatShading={instance.smoothShading === false}
              onInstancePointerDown={onInstancePointerDown}
              onInstancePointerMove={onInstancePointerMove}
              onWorldPick={onWorldPick}
              onComponentHover={onComponentHover}
              mergeMode={mergeMode}
              faceDragActive={selection.faceDragActive === true}
              onFaceDragStart={onFaceDragStart}
              environmentMaterial={environment?.material}
              environmentShadowEnabled={environment?.shadow?.enabled === true}
              onRootRef={registerInstanceRoot}
            />
          );
        })}
      </group>
      <SceneGumball
        target={selection.gumballTarget}
        config={gumballConfig}
        active={gumballVisible}
        onDraggingChanged={handleGumballDraggingChanged}
        onDragStart={handleGumballDragStart}
        onDrag={handleGumballDrag}
        onDragEnd={handleGumballDragEnd}
      />
    </WorldLayerStack>
    </WorldInstanceChromeContext.Provider>
  );
}
//#endregion WorldInstancesLayer

//#region WorldPointCloudLayer
/** ☁️ Renders `World3dScene.pointsJson` layers as GPU point sprites — decodes each layer's base64
 * position/color buffers into a `BufferGeometry` and draws it with a `PointsMaterial`, mounted
 * alongside `🗺️WorldTerrainLayer` in the `World3dHost` scene tree. */
type WorldPointCloudLayerVisual = { readonly geometry: BufferGeometry; readonly material: PointsMaterial };

function pointCloudLayerVisual(layer: WorldPointCloudLayerRecord): WorldPointCloudLayerVisual {
  const geometry = new BufferGeometry();
  const positionBytes = base64ToBytes(layer.positionsB64);
  const positions = new Float32Array(positionBytes.buffer, positionBytes.byteOffset, positionBytes.byteLength / Float32Array.BYTES_PER_ELEMENT);
  geometry.setAttribute("position", new BufferAttribute(positions, 3));
  const hasColors = Boolean(layer.colorsB64);
  if (layer.colorsB64) geometry.setAttribute("color", new BufferAttribute(base64ToBytes(layer.colorsB64), 3, true));
  const material = new PointsMaterial({ size: layer.size, sizeAttenuation: layer.sizeAttenuation, vertexColors: hasColors });
  return { geometry, material };
}

function WorldPointCloudLayer({ pointsJson }: { readonly pointsJson: string | undefined }) {
  const layers = useMemo(() => parseJsonArray<WorldPointCloudLayerRecord>(pointsJson), [pointsJson]);
  const visuals = useMemo(() => {
    const map = new Map<string, WorldPointCloudLayerVisual>();
    for (const layer of layers) map.set(layer.id, pointCloudLayerVisual(layer));
    return map;
  }, [layers]);

  useEffect(() => {
    return () => {
      for (const visual of visuals.values()) {
        visual.geometry.dispose();
        visual.material.dispose();
      }
    };
  }, [visuals]);

  if (layers.length === 0) return null;

  return (
    <group>
      {layers.map((layer) => {
        const visual = visuals.get(layer.id);
        if (!visual) return null;
        return <points key={layer.id} geometry={visual.geometry} material={visual.material} />;
      })}
    </group>
  );
}
//#endregion WorldPointCloudLayer

//#region WorldVortexMarkers
export function worldVortexMaterialRevision(selected?: boolean, hovered?: boolean): "selected" | "hovered" | "neutral" {
  return selected ? "selected" : hovered ? "hovered" : "neutral";
}

const WORLD_VORTEX_Y_AXIS = new Vector3(0, 1, 0);
const WORLD_VORTEX_DIRECTION_FALLBACK: readonly [number, number, number] = [0, 0, -1];

function worldVortexUnitDirection(direction?: readonly [number, number, number]): Vector3 {
  const vector = new Vector3(...(direction ?? WORLD_VORTEX_DIRECTION_FALLBACK));
  if (vector.lengthSq() < 1e-12) {
    return new Vector3(...WORLD_VORTEX_DIRECTION_FALLBACK);
  }
  return vector.normalize();
}

function worldVortexArrowLayout(
  position: readonly [number, number, number],
  direction: readonly [number, number, number] | undefined,
  radius: number,
  displayDirection: "outwards" | "inwards",
): {
  readonly pointRadius: number;
  readonly shaftRadius: number;
  readonly shaftLength: number;
  readonly headLength: number;
  readonly shaftCenter: [number, number, number];
  readonly headCenter: [number, number, number];
  readonly quaternion: Quaternion;
} {
  const dir = worldVortexUnitDirection(direction);
  const arrowLength = radius;
  const headLength = radius * 0.28;
  const shaftLength = Math.max(arrowLength - headLength, radius * 0.2);
  const shaftRadius = radius * 0.055;
  const pointRadius = radius * 0.18;
  const outward = displayDirection !== "inwards";
  const shaftCenter = new Vector3(...position).addScaledVector(dir, outward ? shaftLength * 0.5 : -(headLength + shaftLength * 0.5));
  const headCenter = new Vector3(...position).addScaledVector(dir, outward ? arrowLength - headLength * 0.5 : -headLength * 0.5);
  return {
    pointRadius,
    shaftRadius,
    shaftLength,
    headLength,
    shaftCenter: shaftCenter.toArray() as [number, number, number],
    headCenter: headCenter.toArray() as [number, number, number],
    quaternion: new Quaternion().setFromUnitVectors(WORLD_VORTEX_Y_AXIS, dir),
  };
}

function WorldVortexMarkers({
  vortices,
  palette,
  brushMode,
  selectionMode,
  connectSourceFullId,
  onHover,
  onVortexSelect,
  onBrushPlace,
  onVortexPointerArm,
  onVortexPointerMove,
  onVortexPointerUp,
  onConnectDragHover,
  onConnectDragDrop,
}: {
  readonly vortices: readonly WorldVortexRecord[];
  readonly palette: MeshStylePalette;
  readonly brushMode: boolean;
  readonly selectionMode?: string;
  readonly connectSourceFullId?: string;
  readonly onHover: (fullId: string | null) => void;
  readonly onVortexSelect: (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => void;
  readonly onBrushPlace: () => void;
  readonly onVortexPointerArm: (args: {
    readonly fullId: string;
    readonly position: readonly [number, number, number];
    readonly clientX: number;
    readonly clientY: number;
    readonly event: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean };
  }) => void;
  readonly onVortexPointerMove: (fullId: string, clientX: number, clientY: number) => void;
  readonly onVortexPointerUp: (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => void;
  readonly onConnectDragHover: (position: readonly [number, number, number]) => void;
  readonly onConnectDragDrop: (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => void;
}) {
  if (!vortices.length) return null;
  return (
    <group>
      {vortices.map((vortex) => {
        const radius = vortex.radius ?? 0.36;
        const displayDirection = vortex.displayDirection ?? "outwards";
        const layout = worldVortexArrowLayout(vortex.position, vortex.direction, radius, displayDirection);
        const isConnectSource = connectSourceFullId === vortex.fullId;
        const style = vortex.selected ? palette.selected : vortex.hovered ? palette.hovered : null;
        const color = isConnectSource ? "#f59e0b" : (style?.meshColor ?? vortex.color ?? "#38bdf8");
        const materialKey = worldVortexMaterialRevision(vortex.selected, vortex.hovered);
        const materialProps = {
          color,
          emissive: style?.meshColor ?? "#000000",
          emissiveIntensity: style?.emissiveIntensity ?? 0,
          transparent: true,
          opacity: 0.88,
        };
        const pointerHandlers = {
          onPointerOver: (event: { stopPropagation: () => void }) => {
            event.stopPropagation();
            onHover(vortex.fullId);
            if (connectSourceFullId) onConnectDragHover(vortex.position);
          },
          onPointerOut: (event: { stopPropagation: () => void }) => {
            event.stopPropagation();
            onHover(null);
          },
          onPointerDown: (event: { stopPropagation: () => void; button?: number; clientX: number; clientY: number; shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
            // 🖱️ Ignore right/middle — otherwise opening a context/suggestion menu arms connect-drag
            // and the portaled menu click never delivers host pointer-up to cancel it.
            if (event.button != null && event.button !== 0) return;
            event.stopPropagation();
            if (resolveVortexPointerDownIntent(brushMode, selectionMode) === "select") {
              onVortexSelect(vortex.fullId, event);
              if (brushMode) onBrushPlace();
              return;
            }
            onVortexPointerArm({
              fullId: vortex.fullId,
              position: vortex.position,
              clientX: event.clientX,
              clientY: event.clientY,
              event,
            });
          },
          onPointerMove: (event: { stopPropagation: () => void; clientX: number; clientY: number }) => {
            if (brushMode) return;
            event.stopPropagation();
            onVortexPointerMove(vortex.fullId, event.clientX, event.clientY);
            if (connectSourceFullId) onConnectDragHover(vortex.position);
          },
          onPointerUp: (event: { stopPropagation: () => void; shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
            if (brushMode) return;
            if (connectSourceFullId) {
              event.stopPropagation();
              onConnectDragDrop(vortex.fullId, event);
              return;
            }
            event.stopPropagation();
            onVortexPointerUp(vortex.fullId, event);
          },
          onClick: (event: { stopPropagation: () => void }) => {
            event.stopPropagation();
            if (brushMode) onBrushPlace();
          },
        };
        return (
          <group key={vortex.fullId}>
            <mesh position={vortex.position as [number, number, number]} visible={worldVortexHitProxy(radius).visible} {...pointerHandlers}>
              <sphereGeometry args={[worldVortexHitProxy(radius).radius, 16, 16]} />
              <meshBasicMaterial transparent opacity={0} depthWrite={false} depthTest={false} />
            </mesh>
            <mesh position={vortex.position as [number, number, number]}>
              <sphereGeometry args={[layout.pointRadius, 12, 12]} />
              <meshStandardMaterial key={materialKey} {...materialProps} />
            </mesh>
            <mesh position={layout.shaftCenter} quaternion={layout.quaternion}>
              <cylinderGeometry args={[layout.shaftRadius, layout.shaftRadius, layout.shaftLength, 10]} />
              <meshStandardMaterial key={materialKey} {...materialProps} />
            </mesh>
            <mesh position={layout.headCenter} quaternion={layout.quaternion}>
              <coneGeometry args={[layout.shaftRadius * 1.8, layout.headLength, 12]} />
              <meshStandardMaterial key={materialKey} {...materialProps} />
            </mesh>
          </group>
        );
      })}
    </group>
  );
}
//#endregion WorldVortexMarkers

/** @emoji 🧲️ Rubber-band line drawn from the drag-connect source vortex to the currently hovered vortex (or itself, if hovering nothing). */
function WorldConnectRubberBand({ from, to }: { readonly from: readonly [number, number, number]; readonly to: readonly [number, number, number] }) {
  const geometry = useMemo(() => {
    const positions = new Float32Array([from[0], from[1], from[2], to[0], to[1], to[2]]);
    const geom = new BufferGeometry();
    geom.setAttribute("position", new BufferAttribute(positions, 3));
    return geom;
  }, [from, to]);
  return (
    <lineSegments geometry={geometry} raycast={() => null}>
      <lineBasicMaterial color="#f59e0b" linewidth={2} />
    </lineSegments>
  );
}

/** @emoji 🧊️ Cursor-follow ghost box previewing the target volume that Alt+click would place, sized by the engagement's W/D/H steppers. */
function WorldVoxelPreviewBox({ origin, dims, gridFactor }: { readonly origin: readonly [number, number, number]; readonly dims: readonly [number, number, number]; readonly gridFactor: number }) {
  return (
    <mesh position={origin as [number, number, number]} raycast={() => null}>
      <boxGeometry args={[dims[0] * gridFactor, dims[1] * gridFactor, dims[2] * gridFactor]} />
      <meshStandardMaterial color="#38bdf8" transparent opacity={0.48} />
    </mesh>
  );
}

function WorldAttractionLines({ attractions }: { readonly attractions: readonly WorldAttractionRecord[] }) {
  if (!attractions.length) return null;
  return (
    <group>
      {attractions.map((attraction) => {
        const positions = new Float32Array([attraction.from[0], attraction.from[1], attraction.from[2], attraction.to[0], attraction.to[1], attraction.to[2]]);
        const geometry = new BufferGeometry();
        geometry.setAttribute("position", new BufferAttribute(positions, 3));
        return (
          <lineSegments key={attraction.id} geometry={geometry} raycast={() => null}>
            <lineBasicMaterial color={attraction.color ?? "#60a5fa"} linewidth={2} />
          </lineSegments>
        );
      })}
    </group>
  );
}

/** @emoji 👻️ GLB URL for a brush/suggestion ghost — scene mesh match when present, else the preview's own `meshUrl` (catalogue-drop parity so one-shot suggestions still render kinds not yet placed). */
export function brushPreviewGhostMeshUrl(preview: Pick<WorldBrushPreviewRecord, "meshUrl">, meshes: readonly Pick<WorldMeshRecord, "url">[]): string | undefined {
  const meshUrl = preview.meshUrl;
  if (!meshUrl) return undefined;
  return meshes.find((mesh) => mesh.url === meshUrl)?.url ?? meshUrl;
}

/** @emoji 🎞️ Demand-frameloop kick when a token changes — box fallbacks and already-cached GLBs otherwise leave the suggestion ghost invisible until the next orbit tick. */
function DemandInvalidateOnToken({ token }: { readonly token: string }) {
  const invalidate = useThree((state) => state.invalidate);
  useLayoutEffect(() => {
    invalidate();
  }, [invalidate, token]);
  return null;
}

function BrushPreviewGhost({ preview, meshes, palette }: { readonly preview: WorldBrushPreviewRecord; readonly meshes: readonly WorldMeshRecord[]; readonly palette: MeshStylePalette }) {
  if (!preview.origin) return null;
  const style = palette.highlighted;
  const meshColor = preview.color ?? style.meshColor;
  const url = brushPreviewGhostMeshUrl(preview, meshes);
  const position = preview.origin as [number, number, number];
  const rotation = preview.orientation as [number, number, number, number] | undefined;
  const scale = scaleTuple(preview.scale);
  const quaternion = rotation ? new Quaternion(rotation[0], rotation[1], rotation[2], rotation[3]) : undefined;
  const invalidateToken = `${preview.objectKindId ?? ""}:${preview.targetVortexFullId ?? ""}:${preview.sourceVortexIndex ?? 0}:${url ?? ""}:${position.join(",")}`;
  return (
    <group position={position} scale={scale} quaternion={quaternion} raycast={() => null}>
      <DemandInvalidateOnToken token={invalidateToken} />
      {url ? (
        <Suspense fallback={null}>
          <GlbInstanceMesh url={url} color={meshColor} emissive={meshColor} emissiveIntensity={0.6} opacity={0.72} borderColor={palette.neutral.lineColor} revision="highlighted" />
        </Suspense>
      ) : (
        <mesh raycast={() => null}>
          <boxGeometry args={[1, 1, 1]} />
          <meshBasicMaterial color={meshColor} transparent opacity={0.42} depthWrite={false} />
        </mesh>
      )}
    </group>
  );
}

/** @emoji 🪣️ Bounded fill progress/rejection readout; it is deliberately independent from the optional placement ghost. */
function FillDiagnosticOverlay({ diagnostic }: { readonly diagnostic: WorldFillDiagnosticRecord }) {
  const target = diagnostic.targetVortexFullId ?? "—";
  const candidate = diagnostic.candidateObjectKindId ?? "—";
  const rejection = diagnostic.rejectionReason ?? "—";
  const page = diagnostic.candidatePage.filter((value): value is string => typeof value === "string").join(", ");
  const sample = diagnostic.lastSample?.join(",") ?? "—";
  const label = [
    diagnostic.statusLabel,
    diagnostic.stage,
    String(diagnostic.acceptedCount) + "/" + String(diagnostic.totalCount),
    target,
    candidate,
    String(diagnostic.collisionCount),
    rejection,
    page || "—",
    sample,
  ].join("; ");
  return (
    <div
      className={cn("pointer-events-none absolute bottom-3 left-3 max-w-[28rem] rounded px-2 py-1 text-xs shadow-sm", glassClass)}
      data-fill-operation={diagnostic.operation}
      data-fill-base-revision={diagnostic.baseRevision}
      data-fill-registry-generation={diagnostic.registryGeneration}
      data-fill-generation={diagnostic.generation}
      data-fill-sequence={diagnostic.sequence}
      data-fill-stage={diagnostic.stage}
      data-fill-target-cursor={diagnostic.targetCursor}
      data-fill-candidate-cursor={diagnostic.candidateCursor}
      data-fill-search-count={diagnostic.searchCount}
      data-fill-rejected-count={diagnostic.rejectedCount}
      data-fill-sample-cursor={diagnostic.sampleCursor}
      data-fill-inside-both={diagnostic.insideBoth}
      data-fill-current-pair={diagnostic.currentPairObjectId ?? undefined}
      data-fill-has-ghost={diagnostic.candidateGhost != null}
      data-fill-truncated={diagnostic.truncated}
      role="status"
      aria-label={label}
    >
      <span>{diagnostic.statusLabel}</span>
      <span className="ml-2">{diagnostic.stage}</span>
      <span className="ml-2">
        {diagnostic.acceptedCount}/{diagnostic.totalCount}
      </span>
      <span className="ml-2">{diagnostic.rejectionReason ?? String(diagnostic.collisionCount)}</span>
      {diagnostic.truncated ? <span className="ml-2">…</span> : null}
    </div>
  );
}

/**
 * @emoji ⏳️ The compute-status pane: what the producer says it is doing, how far it has got, and —
 * while it says so — a real button that stops it.
 *
 * 🛑️ The affordance is a `<button>`, not a clickable div, so it is in the tab order, answers Enter
 * and Space for free and reads as a control to a screen reader. Its label comes from the shell's own
 * `ui.common.cancel` bundle (both languages, no default) while the PHASE label comes from the
 * producer's `{en, de}` pair — the surface never invents either. Interaction-friendliness is a
 * CLAUDE.md law, and an expensive operation with progress but no stop is only half of it
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
 */
function WorldComputeStatusPane({
  status,
  glassClass,
  locale,
  onCancel,
}: {
  readonly status: ReturnType<typeof world3dComputeStatusV1>;
  readonly glassClass: string;
  readonly locale: string | undefined;
  readonly onCancel: () => void;
}) {
  if (!status.computing && !status.cancellable && status.phase !== "cancelled") return null;
  const german = (locale ?? "en").toLowerCase().startsWith("de");
  const phaseText = status.phaseLabel ? (german ? status.phaseLabel.de : status.phaseLabel.en) : null;
  const percent = status.unitsTotal > 0 ? Math.round(status.ratio * 100) : null;
  return (
    <div
      className={cn("pointer-events-auto flex items-center gap-single rounded px-single py-half text-xs shadow-sm", glassClass)}
      data-level="pane"
      data-slot="world-compute-status"
      data-compute-phase={status.phase}
      data-compute-cancellable={status.cancellable ? "" : undefined}
      data-compute-ratio={status.unitsTotal > 0 ? String(status.ratio) : undefined}
      role="status"
      aria-busy={status.computing ? "true" : undefined}
    >
      {status.computing ? <Spinner size="small" /> : null}
      <span data-slot="world-compute-phase">{phaseText ?? shellLabel("ui.common.loading")}</span>
      {percent === null ? null : (
        <span data-slot="world-compute-progress">
          {status.unitsDone}/{status.unitsTotal} ({percent}%)
        </span>
      )}
      {status.cancellable ? (
        <button type="button" data-slot="world-compute-cancel" data-cancel-action={status.cancelAction} className="rounded border px-half py-0 text-xs" onClick={onCancel}>
          {shellLabel("ui.common.cancel")}
        </button>
      ) : null}
    </div>
  );
}

function EngagementPreviewLayer({ items, color }: { readonly items: readonly WorldEngagementPreviewItem[]; readonly color: string }) {
  if (!items.length) return null;
  return (
    <group>
      {items.map((item, index) => {
        if (item.kind === "point") {
          return (
            <mesh key={`preview-point-${index}`} position={item.position as [number, number, number]} raycast={() => null}>
              <sphereGeometry args={[0.08, 12, 12]} />
              <meshStandardMaterial color={color} />
            </mesh>
          );
        }
        if (item.kind === "segment") {
          const positions = new Float32Array([item.from[0], item.from[1], item.from[2], item.to[0], item.to[1], item.to[2]]);
          const geometry = new BufferGeometry();
          geometry.setAttribute("position", new BufferAttribute(positions, 3));
          return (
            <lineSegments key={`preview-segment-${index}`} geometry={geometry} raycast={() => null}>
              <lineBasicMaterial color={color} linewidth={2} />
            </lineSegments>
          );
        }
        if (item.kind === "box-preview" && item.cornerA && item.cornerB) {
          const [ax, ay, az] = item.cornerA;
          const [bx, by] = item.cornerB;
          const width = Math.max(Math.abs(bx - ax), 0.05);
          const depth = Math.max(Math.abs(by - ay), 0.05);
          // `height` is a separate vertical extrusion from the footprint plane (az), not derived
          // from cornerB's z — the interaction specs author cornerA/cornerB as ground-plane points.
          const height = Math.max(Math.abs(item.height ?? 0.05), 0.05);
          return (
            <mesh key={`preview-box-${index}`} position={[(ax + bx) * 0.5, (ay + by) * 0.5, az + height * 0.5]} raycast={() => null}>
              <boxGeometry args={[width, depth, height]} />
              <meshBasicMaterial color={color} transparent opacity={0.35} depthWrite={false} wireframe />
            </mesh>
          );
        }
        if (item.kind === "linear-handle") {
          const [ox, oy, oz] = item.origin;
          const [dx, dy, dz] = item.axis;
          const length = Math.max(Math.hypot(dx, dy, dz), 0.05);
          const direction = new Vector3(dx, dy, dz).normalize();
          const quaternion = new Quaternion().setFromUnitVectors(new Vector3(0, 1, 0), direction);
          return (
            <mesh key={`preview-handle-${index}`} position={[ox + dx * 0.5, oy + dy * 0.5, oz + dz * 0.5]} quaternion={quaternion} raycast={() => null}>
              <cylinderGeometry args={[0.02, 0.02, length, 8]} />
              <meshBasicMaterial color={color} transparent opacity={0.6} depthWrite={false} />
            </mesh>
          );
        }
        return null;
      })}
    </group>
  );
}

/** @emoji 🪟️ True when this world host pane owns the open one-shot suggestion popup (by `windowId`). */
export function worldSuggestionMenuOwnsWindow(
  menu: { readonly open?: boolean; readonly windowId?: string } | null | undefined,
  windowInstanceId: string | undefined,
): boolean {
  if (!menu?.open) return false;
  return !menu.windowId || menu.windowId === windowInstanceId;
}

/** @emoji 🧭️ Floating per-vortex candidate popup opened by Alt+right-click or the context menu's "Suggest objects" — a one-shot placement picker that does not switch the active utility into brush mode; hovering a row previews the ghost, clicking places it. Icon + active highlight only (no color swatch — object-kind color stays on the 3D ghost). */
export function suggestionMenuItems(
  menu: WorldSuggestionMenuRecord,
  activeIndex: number,
  labels: { readonly checkingPlacement: UiLabel; readonly noPlacement: UiLabel },
): ContextMenuItemSpec[] {
  if (menu.pending) {
    return [{ id: "pending", label: labels.checkingPlacement, disabled: true }];
  }
  if (menu.candidates.length === 0) {
    return [{ id: "empty", label: labels.noPlacement, disabled: true }];
  }
  return menu.candidates.map((candidate) => ({
    id: `suggestion-${candidate.index}`,
    label: `${candidate.objectLabel} · ${candidate.vortexLabel}`,
    icon: candidate.icon ?? "box",
    checked: candidate.index === activeIndex,
    action: "acceptSuggestion",
    args: { index: candidate.index, ...(menu.vortexFullId ? { fullId: menu.vortexFullId } : {}) },
    hoverAction: "hoverSuggestion",
    hoverArgs: { index: candidate.index },
  }));
}

export const WORLD3D_MARQUEE_DRAG_THRESHOLD_PX = 4;
const MARQUEE_DRAG_THRESHOLD_PX = WORLD3D_MARQUEE_DRAG_THRESHOLD_PX;

/** @emoji 🖱️ Marquee may steal the pointer only after the click slop — otherwise r3f never sees the click. */
export function world3dMarqueePointerCaptureArmed(distancePx: number): boolean {
  return distancePx > MARQUEE_DRAG_THRESHOLD_PX;
}

/** @emoji 🎯️ The five-word `MergeMode` set algebra for non-string id sets — mirrors `selectionMergeIds`
 * from `@semio-tech/ui-react`, which is string-only. `range` has no ordered topology in a world
 * viewport, so it replaces (see `🕹️interaction/🧫️fixtures/🎯️merge-modes.json`, `range.unorderedDomains`). */
function mergeIdSet<T>(mode: MergeMode, current: readonly T[], incoming: readonly T[]): T[] {
  const currentSet = new Set(current);
  const incomingSet = new Set(incoming);
  if (mode === "replace" || mode === "range") return [...incomingSet];
  if (mode === "additive") {
    for (const id of incomingSet) currentSet.add(id);
    return [...currentSet];
  }
  if (mode === "subtractive") {
    for (const id of incomingSet) currentSet.delete(id);
    return [...currentSet];
  }
  for (const id of incomingSet) {
    if (currentSet.has(id)) currentSet.delete(id);
    else currentSet.add(id);
  }
  return [...currentSet];
}

/** @emoji 🎯️ Resolves a world surface's declared selection mode before falling back to the shared selection toolbar. */
export function resolveWorldMergeMode(
  configuredMode: MergeMode | undefined,
  modifiers: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean },
  persistentMode?: MergeMode,
): MergeMode {
  if (configuredMode === undefined) return marqueeModeFromModifiers(modifiers, persistentMode);
  if (configuredMode !== "replace") return configuredMode;
  const shift = modifiers.shiftKey === true;
  const ctrl = modifiers.ctrlKey === true || modifiers.metaKey === true;
  if (shift && ctrl) return "invertive";
  if (shift) return "additive";
  if (ctrl) return "subtractive";
  return "replace";
}

/** @emoji 🖱️ The component (vertex/edge/face) pick's one deviation from the whole-instance pick: a BARE
 * click toggles instead of replacing, because component picking is an accumulate gesture. Every other
 * chord resolves exactly as {@link resolveWorldMergeMode} says. The result is a `MergeMode` — the ONE
 * merge vocabulary of `🕹️interaction/🧬️schema/🔣️.json`, pinned by
 * `🕹️interaction/🧫️fixtures/🎯️merge-modes.json`. Ticket 26/09/09/PROCEDURAL-3D-END-TO-END deleted the
 * `instanceMergeArg`/`componentMergeArg` translation into a private `add`/`remove`/`toggle` spelling:
 * the framework's `parse_merge_mode` rejects those words, so every modifier-click on a domain-bound
 * world scene faulted with `interactionSelect: unknown merge '…'`. */
export function componentPickMergeMode(mode: MergeMode): MergeMode {
  return mode === "replace" ? "invertive" : mode;
}

function pointInMarqueeRect(sx: number, sy: number, marquee: readonly SelectionMarqueePoint[]): boolean {
  if (marquee.length < 2) return false;
  const start = marquee[0]!;
  const end = marquee[marquee.length - 1]!;
  const minX = Math.min(start.x, end.x);
  const maxX = Math.max(start.x, end.x);
  const minY = Math.min(start.y, end.y);
  const maxY = Math.max(start.y, end.y);
  return sx >= minX && sx <= maxX && sy >= minY && sy <= maxY;
}

/** @emoji 🎯️ Even-odd point-in-polygon test for lasso selection. */
function pointInPolygon(sx: number, sy: number, polygon: readonly SelectionMarqueePoint[]): boolean {
  let inside = false;
  for (let i = 0, j = polygon.length - 1; i < polygon.length; j = i++) {
    const a = polygon[i]!;
    const b = polygon[j]!;
    const intersects = a.y > sy !== b.y > sy && sx < ((b.x - a.x) * (sy - a.y)) / (b.y - a.y) + a.x;
    if (intersects) inside = !inside;
  }
  return inside;
}

function pointInMarqueeRegion(sx: number, sy: number, method: SelectionMarqueeMethod, marquee: readonly SelectionMarqueePoint[]): boolean {
  if (method === "lasso" && marquee.length >= 3) return pointInPolygon(sx, sy, marquee);
  return pointInMarqueeRect(sx, sy, marquee);
}

/** @emoji 🎯️ Window (full containment, all points) vs crossing (partial, any point) semantics for multi-point elements. */
function pointsSatisfyMarquee(points: readonly (readonly [number, number])[], method: SelectionMarqueeMethod, marquee: readonly SelectionMarqueePoint[], coverage: SelectionMarqueeCoverage): boolean {
  if (points.length === 0) return false;
  const test = (point: readonly [number, number]) => pointInMarqueeRegion(point[0], point[1], method, marquee);
  return coverage === "full" ? points.every(test) : points.some(test);
}

function projectWorldPoint(point: readonly [number, number, number], offset: readonly [number, number, number], camera: import("three").Camera, rect: DOMRect): { readonly x: number; readonly y: number } {
  const projected = new Vector3(point[0] + offset[0], point[1] + offset[1], point[2] + offset[2]).project(camera);
  return {
    x: ((projected.x + 1) / 2) * rect.width,
    y: ((-projected.y + 1) / 2) * rect.height,
  };
}

function resolveMarqueeComponentIds(
  instances: readonly WorldInstanceRecord[],
  meshes: readonly WorldMeshRecord[],
  selectionMode: string,
  activeObjectId: string | undefined,
  marquee: readonly SelectionMarqueePoint[],
  rect: DOMRect,
  camera: import("three").Camera,
  method: SelectionMarqueeMethod,
  coverage: SelectionMarqueeCoverage,
): readonly number[] {
  const active = instances.find((instance) => instance.id === activeObjectId);
  if (!active) return [];
  const meshId = active.meshId ?? active.id;
  const meshData = meshes.find((mesh) => mesh.id === meshId)?.data;
  if (!meshData) return [];
  const offset = (active.position ?? [0, 0, 0]) as [number, number, number];
  const project = (point: readonly [number, number, number]): readonly [number, number] => {
    const screen = projectWorldPoint(point, offset, camera, rect);
    return [screen.x, screen.y];
  };
  const hits = new Set<number>();
  if (selectionMode === "vertex") {
    const pick = buildVertexPickData(meshData);
    if (!pick) return [];
    const positions = pick.geometry.attributes.position!;
    for (let index = 0; index < pick.vertexIds.length; index += 1) {
      const point = project([positions.getX(index), positions.getY(index), positions.getZ(index)]);
      if (pointsSatisfyMarquee([point], method, marquee, coverage)) hits.add(pick.vertexIds[index]!);
    }
  } else if (selectionMode === "edge" && meshData.edgeIds && meshData.edgePositions) {
    for (let edgeIndex = 0; edgeIndex < meshData.edgeIds.length; edgeIndex += 1) {
      const base = edgeIndex * 6;
      const a = project([meshData.edgePositions[base]!, meshData.edgePositions[base + 1]!, meshData.edgePositions[base + 2]!]);
      const b = project([meshData.edgePositions[base + 3]!, meshData.edgePositions[base + 4]!, meshData.edgePositions[base + 5]!]);
      if (pointsSatisfyMarquee([a, b], method, marquee, coverage)) hits.add(meshData.edgeIds[edgeIndex]!);
    }
  } else if (selectionMode === "face" && meshData.faceIds && meshData.indices.length) {
    for (let faceIndex = 0; faceIndex < meshData.faceIds.length; faceIndex += 1) {
      const i0 = meshData.indices[faceIndex * 3] ?? 0;
      const i1 = meshData.indices[faceIndex * 3 + 1] ?? 0;
      const i2 = meshData.indices[faceIndex * 3 + 2] ?? 0;
      const p0 = project([meshData.positions[i0 * 3]!, meshData.positions[i0 * 3 + 1]!, meshData.positions[i0 * 3 + 2]!]);
      const p1 = project([meshData.positions[i1 * 3]!, meshData.positions[i1 * 3 + 1]!, meshData.positions[i1 * 3 + 2]!]);
      const p2 = project([meshData.positions[i2 * 3]!, meshData.positions[i2 * 3 + 1]!, meshData.positions[i2 * 3 + 2]!]);
      if (pointsSatisfyMarquee([p0, p1, p2], method, marquee, coverage)) hits.add(meshData.faceIds[faceIndex]!);
    }
  }
  return [...hits];
}

/** @emoji 📦️ Local-space AABB corners of a mesh's vertex positions (or edge samples for curve-only meshes; fallback: origin). */
export function meshBoundsCorners(meshData: WorldMeshData): readonly (readonly [number, number, number])[] {
  let minX = Infinity;
  let minY = Infinity;
  let minZ = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;
  let maxZ = -Infinity;
  const source = meshData.positions.length >= 3 ? meshData.positions : (meshData.edgePositions ?? []);
  for (let index = 0; index < source.length; index += 3) {
    const x = source[index]!;
    const y = source[index + 1]!;
    const z = source[index + 2]!;
    if (x < minX) minX = x;
    if (x > maxX) maxX = x;
    if (y < minY) minY = y;
    if (y > maxY) maxY = y;
    if (z < minZ) minZ = z;
    if (z > maxZ) maxZ = z;
  }
  if (!Number.isFinite(minX)) return [[0, 0, 0]];
  return [
    [minX, minY, minZ],
    [maxX, minY, minZ],
    [minX, maxY, minZ],
    [maxX, maxY, minZ],
    [minX, minY, maxZ],
    [maxX, minY, maxZ],
    [minX, maxY, maxZ],
    [maxX, maxY, maxZ],
  ];
}

function resolveMarqueeInstanceIds(
  instances: readonly WorldInstanceRecord[],
  meshes: readonly WorldMeshRecord[],
  marquee: readonly SelectionMarqueePoint[],
  rect: DOMRect,
  camera: import("three").Camera,
  method: SelectionMarqueeMethod,
  coverage: SelectionMarqueeCoverage,
): readonly string[] {
  const meshById = new Map(meshes.map((mesh) => [mesh.id, mesh]));
  const hits: string[] = [];
  instances.forEach((instance, index) => {
    if (isRevealCutoffHidden(instance)) return;
    const meshId = instance.meshId ?? instance.id;
    const meshData = meshById.get(meshId)?.data;
    const position = (instance.position ?? [instance.x ?? index, instance.y ?? 0, instance.z ?? 0]) as [number, number, number];
    const scale = (instance.scale ?? [1, 1, 1]) as [number, number, number];
    const rotation = instance.rotation;
    const quaternion = rotation ? new Quaternion(rotation[0], rotation[1], rotation[2], rotation[3]) : undefined;
    const localCorners = world3dInstanceLocalCorners(meshData, meshById.get(meshId)?.url);
    const worldCorners = localCorners.map((corner) => {
      const v = new Vector3(corner[0] * scale[0], corner[1] * scale[1], corner[2] * scale[2]);
      if (quaternion) v.applyQuaternion(quaternion);
      v.add(new Vector3(position[0], position[1], position[2]));
      return [v.x, v.y, v.z] as const;
    });
    const points = worldCorners.map((corner) => {
      const screen = projectWorldPoint(corner, [0, 0, 0], camera, rect);
      return [screen.x, screen.y] as const;
    });
    if (pointsSatisfyMarquee(points, method, marquee, coverage)) hits.push(instance.id);
  });
  return hits;
}

/** @emoji 🖱️ Whether a host-local click sits inside the axis-aligned screen box of projected mesh corners. */
export function world3dProjectedAabbContainsClick(
  click: { readonly x: number; readonly y: number },
  corners: readonly (readonly [number, number])[],
): boolean {
  if (corners.length === 0) return false;
  let minX = Infinity;
  let minY = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;
  for (const [x, y] of corners) {
    if (x < minX) minX = x;
    if (y < minY) minY = y;
    if (x > maxX) maxX = x;
    if (y > maxY) maxY = y;
  }
  return click.x >= minX && click.x <= maxX && click.y >= minY && click.y <= maxY;
}

/** @emoji 🖱️ Nearest projected AABB that contains the click; empty space returns null. */
export function resolveClickInstanceIdFromProjected(
  click: { readonly x: number; readonly y: number },
  candidates: readonly { readonly id: string; readonly corners: readonly (readonly [number, number])[]; readonly depth: number }[],
): string | null {
  let bestId: string | null = null;
  let bestDepth = Infinity;
  for (const candidate of candidates) {
    if (!world3dProjectedAabbContainsClick(click, candidate.corners)) continue;
    if (candidate.depth < bestDepth) {
      bestDepth = candidate.depth;
      bestId = candidate.id;
    }
  }
  return bestId;
}

function resolveClickInstanceId(
  instances: readonly WorldInstanceRecord[],
  meshes: readonly WorldMeshRecord[],
  click: SelectionMarqueePoint,
  rect: DOMRect,
  camera: import("three").Camera,
): string | null {
  const meshById = new Map(meshes.map((mesh) => [mesh.id, mesh]));
  const cam = camera.position;
  const candidates: { id: string; corners: readonly (readonly [number, number])[]; depth: number }[] = [];
  instances.forEach((instance, index) => {
    if (isRevealCutoffHidden(instance)) return;
    const meshId = instance.meshId ?? instance.id;
    const meshData = meshById.get(meshId)?.data;
    const position = (instance.position ?? [instance.x ?? index, instance.y ?? 0, instance.z ?? 0]) as [number, number, number];
    const scale = (instance.scale ?? [1, 1, 1]) as [number, number, number];
    const rotation = instance.rotation;
    const quaternion = rotation ? new Quaternion(rotation[0], rotation[1], rotation[2], rotation[3]) : undefined;
    const localCorners = world3dInstanceLocalCorners(meshData, meshById.get(meshId)?.url);
    const worldCorners = localCorners.map((corner) => {
      const v = new Vector3(corner[0] * scale[0], corner[1] * scale[1], corner[2] * scale[2]);
      if (quaternion) v.applyQuaternion(quaternion);
      v.add(new Vector3(position[0], position[1], position[2]));
      return [v.x, v.y, v.z] as const;
    });
    const points = worldCorners.map((corner) => {
      const screen = projectWorldPoint(corner, [0, 0, 0], camera, rect);
      return [screen.x, screen.y] as const;
    });
    candidates.push({
      id: instance.id,
      corners: points,
      depth: Math.hypot(position[0] - cam.x, position[1] - cam.y, position[2] - cam.z),
    });
  });
  return resolveClickInstanceIdFromProjected(click, candidates);
}

function CameraRefBridge({ cameraRef }: { readonly cameraRef: React.MutableRefObject<import("three").Camera | null> }) {
  const camera = useThree((state) => state.camera);
  useEffect(() => {
    cameraRef.current = camera;
  }, [camera, cameraRef]);
  return null;
}

/** @emoji 🧭️ Registers this window's live camera and entity data as the introduction demonstration
 * engine's resolver for `windowElementId(windowInstanceId)` — the same element id `windowElementId(kind.id)`
 * builds for authoring (see `mit-bestand/demonstrator/🪧️brand.ts`). Only the base (non-split) window instance
 * registers under its exact kind id this way, since `windowInstanceId` equals `kind.id` verbatim for it;
 * a split/spawned extra instance registers under its own instance id instead (last-write-wins if multiple
 * instances of a kind are open — acceptable for a single demonstration target, not a general
 * multi-instance broadcast). Supports `scenePoint` (raw 3D → screen), `canvasPoint` (ground-plane 2D →
 * screen, z = 0), and `entity` for domains `"vortex"` (`WorldVortexRecord.fullId`), `"object"`
 * (`WorldInstanceRecord.id`, reveal-cutoff-hidden instances resolve to null), and `"attraction"`
 * (`WorldAttractionRecord.id`, exposes a `polyline` of its projected endpoints for `Curve` targeting) —
 * `entity: "*"` picks whichever visible record projects nearest the window's own viewport center. Reads
 * the latest arrays via a ref so the registration effect (keyed on camera/gl/window id, not on scene data)
 * never has to re-run just because the scene updated. */
function IntroductionWorldResolverBridge({
  windowInstanceId,
  vortices,
  instances,
  attractions,
}: {
  readonly windowInstanceId: string;
  readonly vortices: readonly WorldVortexRecord[];
  readonly instances: readonly WorldInstanceRecord[];
  readonly attractions: readonly WorldAttractionRecord[];
}) {
  const camera = useThree((state) => state.camera);
  const gl = useThree((state) => state.gl);
  const dataRef = useRef({ vortices, instances, attractions });
  dataRef.current = { vortices, instances, attractions };

  useEffect(() => {
    const windowId = windowElementId(windowInstanceId);
    const project = (position: readonly [number, number, number]): { readonly x: number; readonly y: number; readonly visible: boolean } => {
      const projected = new Vector3(...cadVec3ToThree(position)).project(camera);
      if (projected.z >= 1 || Math.abs(projected.x) > 1.05 || Math.abs(projected.y) > 1.05) return { x: 0, y: 0, visible: false };
      const rect = gl.domElement.getBoundingClientRect();
      const { x, y } = ndcToViewportPoint(projected, rect);
      return { x, y, visible: true };
    };
    const nearestToCenter = <T,>(
      records: readonly T[],
      positionOf: (record: T) => readonly [number, number, number],
    ): { readonly record: T; readonly projected: { readonly x: number; readonly y: number } } | null => {
      const rect = gl.domElement.getBoundingClientRect();
      const centerX = rect.left + rect.width / 2;
      const centerY = rect.top + rect.height / 2;
      let best: { record: T; projected: { x: number; y: number } } | null = null;
      let bestDistance = Infinity;
      for (const record of records) {
        const projected = project(positionOf(record));
        if (!projected.visible) continue;
        const distance = Math.hypot(projected.x - centerX, projected.y - centerY);
        if (distance < bestDistance) {
          bestDistance = distance;
          best = { record, projected };
        }
      }
      return best;
    };
    const instancePosition = (instance: WorldInstanceRecord): readonly [number, number, number] => instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0];
    const attractionMidpoint = (attraction: WorldAttractionRecord): readonly [number, number, number] => [
      (attraction.from[0] + attraction.to[0]) / 2,
      (attraction.from[1] + attraction.to[1]) / 2,
      (attraction.from[2] + attraction.to[2]) / 2,
    ];

    return registerIntroductionSurfaceResolver(windowId, {
      scenePoint: project,
      canvasPoint: (x, y) => project([x, y, 0]),
      entity: (domain, entity): IntroductionResolvedGeometry | null => {
        const { vortices: liveVortices, instances: liveInstances, attractions: liveAttractions } = dataRef.current;
        if (domain === "vortex") {
          if (entity === "*") {
            const nearest = nearestToCenter(liveVortices, (vortex) => vortex.position);
            return nearest ? { point: nearest.projected, visible: true } : null;
          }
          const vortex = liveVortices.find((candidate) => candidate.fullId === entity);
          if (!vortex) return null;
          const projected = project(vortex.position);
          return projected.visible ? { point: projected, visible: true } : null;
        }
        if (domain === "object") {
          const visibleInstances = liveInstances.filter((instance) => !isRevealCutoffHidden(instance));
          if (entity === "*") {
            const nearest = nearestToCenter(visibleInstances, instancePosition);
            return nearest ? { point: nearest.projected, visible: true } : null;
          }
          const instance = visibleInstances.find((candidate) => candidate.id === entity);
          if (!instance) return null;
          const projected = project(instancePosition(instance));
          return projected.visible ? { point: projected, visible: true } : null;
        }
        if (domain === "attraction") {
          const attraction = entity === "*" ? nearestToCenter(liveAttractions, attractionMidpoint)?.record : liveAttractions.find((candidate) => candidate.id === entity);
          if (!attraction) return null;
          const from = project(attraction.from);
          const to = project(attraction.to);
          if (!from.visible || !to.visible) return null;
          return { point: { x: (from.x + to.x) / 2, y: (from.y + to.y) / 2 }, polyline: [from, to], visible: true };
        }
        return null;
      },
    });
  }, [camera, gl, windowInstanceId]);
  return null;
}

/** @emoji 🎯️ Widens Line/Points raycast hit area so thin edge/vertex geometry is reliably pickable without stealing face clicks. */
function RaycasterPickTuning() {
  const raycaster = useThree((state) => state.raycaster);
  useEffect(() => {
    raycaster.params.Line = { threshold: 0.12 };
    raycaster.params.Points = { threshold: 0.08 };
  }, [raycaster]);
  return null;
}


function WorldGumballHitStamp({
  target,
  active,
  hostRef,
}: {
  readonly target?: readonly [number, number, number];
  readonly active: boolean;
  readonly hostRef: React.RefObject<HTMLDivElement | null>;
}) {
  const camera = useThree((state) => state.camera);
  const size = useThree((state) => state.size);
  useFrame(() => {
    const el = hostRef.current;
    if (!el) return;
    if (!active || !target) {
      if (el.getAttribute("data-gumball-hits") !== "[]") el.setAttribute("data-gumball-hits", "[]");
      return;
    }
    const origin = new Vector3(target[0], target[1], target[2]);
    const scale = Math.max(camera.position.distanceTo(origin) / 8, 1e-4);
    const tip = 0.85;
    const handles = [
      { kind: "moveX", x: target[0] + scale * tip, y: target[1], z: target[2] },
      { kind: "moveY", x: target[0], y: target[1] + scale * tip, z: target[2] },
      { kind: "moveZ", x: target[0], y: target[1], z: target[2] + scale * tip },
      { kind: "origin", x: target[0], y: target[1], z: target[2] },
    ];
    const hits = handles.map((handle) => {
      const projected = new Vector3(handle.x, handle.y, handle.z).project(camera);
      return {
        kind: handle.kind,
        sx: Math.round((projected.x * 0.5 + 0.5) * size.width),
        sy: Math.round((-projected.y * 0.5 + 0.5) * size.height),
        ndcZ: Number(projected.z.toFixed(3)),
      };
    });
    const next = JSON.stringify(hits);
    if (el.getAttribute("data-gumball-hits") !== next) el.setAttribute("data-gumball-hits", next);
  });
  return null;
}

function WorldVortexHitStamp({
  vortices,
  hostRef,
}: {
  readonly vortices: readonly WorldVortexRecord[];
  readonly hostRef: React.RefObject<HTMLDivElement | null>;
}) {
  const camera = useThree((state) => state.camera);
  const size = useThree((state) => state.size);
  useFrame(() => {
    const el = hostRef.current;
    if (!el) return;
    const hits = vortices.map((vortex) => {
      const projected = new Vector3(vortex.position[0], vortex.position[1], vortex.position[2]).project(camera);
      return {
        fullId: vortex.fullId,
        x: vortex.position[0],
        y: vortex.position[1],
        z: vortex.position[2],
        sx: Math.round((projected.x * 0.5 + 0.5) * size.width),
        sy: Math.round((-projected.y * 0.5 + 0.5) * size.height),
        ndcZ: Number(projected.z.toFixed(3)),
      };
    });
    const next = JSON.stringify(hits);
    if (el.getAttribute("data-vortex-hits") !== next) el.setAttribute("data-vortex-hits", next);
  });
  return null;
}

function paneSuffixFromSurfaceId(surfaceId?: string): string | undefined {
  if (!surfaceId) return undefined;
  const slash = surfaceId.lastIndexOf("/");
  return slash >= 0 ? surfaceId.slice(slash + 1) : surfaceId;
}

/** @emoji 📡️ World-space camera ray through an NDC point — orthographic uses parallel near→far unproject rays;
 * perspective uses the pinhole from `camera.position`. Duck-types `isOrthographicCamera` (not `instanceof`) so
 * R3F-swapped cameras stay correct. */
function worldRayFromNdc(ndcX: number, ndcY: number, camera: import("three").Camera): { origin: Vector3; direction: Vector3 } | null {
  const ortho = camera as OrthographicCamera & { readonly isOrthographicCamera?: boolean };
  if (ortho.isOrthographicCamera) {
    const origin = new Vector3(ndcX, ndcY, -1).unproject(camera);
    const direction = new Vector3(ndcX, ndcY, 1).unproject(camera).sub(origin);
    if (direction.lengthSq() < 1e-12) return null;
    return { origin, direction: direction.normalize() };
  }
  const origin = camera.position.clone();
  const direction = new Vector3(ndcX, ndcY, 0.5).unproject(camera).sub(origin);
  if (direction.lengthSq() < 1e-12) return null;
  return { origin, direction: direction.normalize() };
}

/** @emoji 🎯️ Intersects the camera ray through a client point with the world Z=0 ground plane (catalogue drop + face drag). */
export function raycastGroundPoint(clientX: number, clientY: number, hostRect: DOMRect, camera: import("three").Camera): [number, number, number] | null {
  const ndcX = ((clientX - hostRect.left) / hostRect.width) * 2 - 1;
  const ndcY = -(((clientY - hostRect.top) / hostRect.height) * 2 - 1);
  const ray = worldRayFromNdc(ndcX, ndcY, camera);
  if (!ray) return null;
  if (Math.abs(ray.direction.z) < 1e-6) return null;
  const t = -ray.origin.z / ray.direction.z;
  if (t < 0) return null;
  const hit = ray.origin.add(ray.direction.multiplyScalar(t));
  return [hit.x, hit.y, hit.z];
}

//#region CatalogueDrop
type Puzzle3dCatalogueDropPayload = {
  readonly objectKind: string;
  readonly meshUrl?: string;
};

type Puzzle3dCatalogueDropPreview = Puzzle3dCatalogueDropPayload & {
  readonly origin: readonly [number, number, number];
};

export function parsePuzzle3dCatalogueDragPayload(encoded: string | null | undefined): Puzzle3dCatalogueDropPayload | null {
  if (!encoded) return null;
  try {
    const parsed = JSON.parse(encoded) as Partial<Puzzle3dCatalogueDropPayload>;
    if (typeof parsed.objectKind !== "string" || !parsed.objectKind) return null;
    return {
      objectKind: parsed.objectKind,
      meshUrl: typeof parsed.meshUrl === "string" && parsed.meshUrl ? parsed.meshUrl : undefined,
    };
  } catch {
    return null;
  }
}

export function snapWorldPointToGrid(point: readonly [number, number, number], gridSnapEnabled: boolean, gridFactor: number): [number, number, number] {
  if (!gridSnapEnabled || gridFactor <= 0) return [point[0], point[1], point[2]];
  const snap = (value: number) => Math.round(value / gridFactor) * gridFactor;
  return [snap(point[0]), snap(point[1]), snap(point[2])];
}

//#region WorldRelocateGesture
/** @emoji 🚚️ Which object a Relocate-utility press grabs. A press with NOTHING selected grabs the object
 * under the pointer outright (direct manipulation, no select-then-drag ceremony); a press on a DIFFERENT
 * object than the selected one grabs nothing and falls through to the ordinary marquee/pick path, so the
 * user can re-select without the scene jumping under the cursor.
 * `selectedIds` is the leftover/object id list {@link world3dGumballSelectionArgsV1} already resolves.
 *
 * 🎯️ A press on EMPTY GROUND while a selection exists grabs the selection's anchor and treats the press
 * as the gesture's BASE POINT — the two-point move ("pick a base point, pick a target point") every CAD
 * tool spells this way, and exactly what {@link world3dRelocateDispatchArgsV1} already computes:
 * `origin + (to - from)`, a travel DELTA, never "put the object under the cursor". The base point
 * therefore never had to be on the object, and requiring it there made the utility unusable whenever the
 * grab point was occluded, off-screen or simply small on screen — measured on the live `:6013` shell
 * (wasm #53, ticket 26/09/02 wave B34, `📓️2026-09-12-wave-B34-interaction-scope.md` §2): a press at
 * `local={x:739,y:342}` resolved the ground point `[9.76,42.24,0]` with `ids=["seed-left-001"]` live and
 * nothing under the pointer, i.e. refused only because the press sat 265 px from the one object's
 * projected origin.
 * The cost is deliberate: inside this utility an empty-ground press is a base point rather than a
 * marquee, and a press without travel commits nothing ({@link GUMBALL_TRANSFORM_EPSILON}), so Escape and
 * `clearSelection` stay the way to drop a selection. */
export function world3dRelocateDragTargetV1(pressedId: string | null | undefined, selectedIds: readonly string[]): string | null {
  if (!pressedId) return selectedIds[0] ?? null;
  if (selectedIds.length === 0) return pressedId;
  return selectedIds.includes(pressedId) ? pressedId : null;
}

/** @emoji 🚚️ The ONE `worldRelocate` payload a finished Relocate drag commits — the absolute world origin
 * the guest's `world_relocate` arm decodes as `{objectId, position}`, never an incremental pose delta
 * (`gumballTransformDeltaBetweenPoses`'s `translateSelection` shape, which this verb deliberately is not).
 * The ground-plane travel `from → to` is added to the grabbed object's own origin and snapped exactly like
 * a catalogue drop; a drag that lands inside {@link GUMBALL_TRANSFORM_EPSILON} of where it started returns
 * `null` so a click-without-travel never writes a document edit. */
export function world3dRelocateDispatchArgsV1(
  objectId: string,
  origin: readonly [number, number, number],
  from: readonly [number, number, number],
  to: readonly [number, number, number],
  snap?: { readonly gridSnapEnabled: boolean; readonly gridFactor: number },
): { readonly objectId: string; readonly position: readonly [number, number, number] } | null {
  const moved: [number, number, number] = [origin[0] + (to[0] - from[0]), origin[1] + (to[1] - from[1]), origin[2] + (to[2] - from[2])];
  const position = snap ? snapWorldPointToGrid(moved, snap.gridSnapEnabled, snap.gridFactor) : moved;
  if (Math.abs(position[0] - origin[0]) < GUMBALL_TRANSFORM_EPSILON && Math.abs(position[1] - origin[1]) < GUMBALL_TRANSFORM_EPSILON && Math.abs(position[2] - origin[2]) < GUMBALL_TRANSFORM_EPSILON) {
    return null;
  }
  return { objectId, position };
}

type World3dRelocateSession = {
  readonly objectId: string;
  readonly origin: readonly [number, number, number];
  readonly from: readonly [number, number, number];
  readonly objectKind: string;
  readonly meshUrl?: string;
};
//#endregion WorldRelocateGesture

//#region WorldVolumeBrushGesture
/** @emoji 🧊️ Grid-snapped ground origin a Volume Brush hover previews and an Alt+click commits, from the
 * raw `raycastGroundPoint` hit. Always snaps (the guest re-snaps by its own `gridSpacing` anyway), so a
 * placed target volume, a dropped catalogue object and a relocated object all land on one lattice.
 * `null` when the pointer ray misses the ground plane, which is also what keeps the ghost box off screen. */
export function world3dVolumeBrushOriginV1(ground: readonly [number, number, number] | null, gridFactor: number): [number, number, number] | null {
  return ground ? snapWorldPointToGrid(ground, true, gridFactor) : null;
}

/** @emoji 🧊️ True when a Volume Brush pointer press must COMMIT a target volume instead of opening a
 * marquee — the utility armed and Alt held. The gesture reads the ground through the host's own pointer
 * handlers (like relocate and catalogue drop) rather than an invisible r3f plane inside the canvas, which
 * the instance layer occluded on every click that landed on an object
 * (`📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 12). */
export function world3dVolumeBrushCommits(volumeBrushMode: boolean, altKey: boolean): boolean {
  return volumeBrushMode && altKey;
}
//#endregion WorldVolumeBrushGesture

function resolveCatalogueDropOrigin(clientX: number, clientY: number, hostRect: DOMRect, camera: import("three").Camera | null, gridSnapEnabled: boolean, gridFactor: number): [number, number, number] | null {
  if (!camera) return null;
  const hit = raycastGroundPoint(clientX, clientY, hostRect, camera);
  if (!hit) return null;
  return snapWorldPointToGrid(hit, gridSnapEnabled, gridFactor);
}

function clientPointOverHost(clientX: number, clientY: number, hostRect: DOMRect): boolean {
  return clientX >= hostRect.left && clientX <= hostRect.right && clientY >= hostRect.top && clientY <= hostRect.bottom;
}

//#region WorldCatalogueDropPreviewStore
/** @emoji 👻️ Shared world-space catalogue-drop ghosts keyed by controller — every {@link World3dHost} pane of that controller subscribes so the preview is never clipped to the hovered window. */
const worldCatalogueDropPreviewByController = new Map<string, Puzzle3dCatalogueDropPreview>();
const worldCatalogueDropPreviewListeners = new Set<() => void>();
const worldCatalogueDropHostHitTests = new Map<string, { readonly controllerId: string; readonly hitTest: (clientX: number, clientY: number) => boolean }>();

function worldCatalogueDropPreviewsEqual(a: Puzzle3dCatalogueDropPreview | null | undefined, b: Puzzle3dCatalogueDropPreview | null | undefined): boolean {
  if (a === b) return true;
  if (!a || !b) return false;
  return a.objectKind === b.objectKind && a.meshUrl === b.meshUrl && a.origin[0] === b.origin[0] && a.origin[1] === b.origin[1] && a.origin[2] === b.origin[2];
}

function notifyWorldCatalogueDropPreviewListeners(): void {
  for (const listener of worldCatalogueDropPreviewListeners) listener();
}

/** @emoji 👻️ Subscribe to shared world-space catalogue-drop previews (all open World3d panes). */
export function subscribeWorldCatalogueDropPreview(listener: () => void): () => void {
  worldCatalogueDropPreviewListeners.add(listener);
  return () => {
    worldCatalogueDropPreviewListeners.delete(listener);
  };
}

/** @emoji 👻️ Current shared world-space catalogue-drop preview for `controllerId`, or `null` when no catalogue drag is live. */
export function getWorldCatalogueDropPreview(controllerId: string): Puzzle3dCatalogueDropPreview | null {
  return worldCatalogueDropPreviewByController.get(controllerId) ?? null;
}

/** @emoji 👻️ SSR snapshot for {@link useSyncExternalStore} — catalogue drops never hydrate with a live ghost. */
export function getWorldCatalogueDropPreviewServerSnapshot(_controllerId: string): Puzzle3dCatalogueDropPreview | null {
  return null;
}

/** @emoji 👻️ Publish a world-space catalogue-drop ghost visible in every World3d pane of `controllerId`. */
export function setWorldCatalogueDropPreview(controllerId: string, preview: Puzzle3dCatalogueDropPreview | null): void {
  const previous = worldCatalogueDropPreviewByController.get(controllerId) ?? null;
  if (worldCatalogueDropPreviewsEqual(previous, preview)) return;
  if (preview) worldCatalogueDropPreviewByController.set(controllerId, preview);
  else worldCatalogueDropPreviewByController.delete(controllerId);
  notifyWorldCatalogueDropPreviewListeners();
}

/** @emoji 👻️ Clears the shared catalogue-drop ghost across all World3d panes of `controllerId`. */
export function clearWorldCatalogueDropPreview(controllerId: string): void {
  setWorldCatalogueDropPreview(controllerId, null);
}

/** @emoji 🎯️ Registers a World3d pane's hit-test so shared clear logic can tell whether the pointer is over *any* pane of that controller. */
export function registerWorldCatalogueDropHost(controllerId: string, hostId: string, hitTest: (clientX: number, clientY: number) => boolean): () => void {
  const key = `${controllerId}\0${hostId}`;
  worldCatalogueDropHostHitTests.set(key, { controllerId, hitTest });
  return () => {
    worldCatalogueDropHostHitTests.delete(key);
  };
}

/** @emoji 🎯️ True when any registered World3d pane of `controllerId` contains the client point. */
export function worldCatalogueDropHostContainsPoint(controllerId: string, clientX: number, clientY: number): boolean {
  for (const entry of worldCatalogueDropHostHitTests.values()) {
    if (entry.controllerId !== controllerId) continue;
    if (entry.hitTest(clientX, clientY)) return true;
  }
  return false;
}
//#endregion WorldCatalogueDropPreviewStore

function CatalogueDropGhost({ preview, meshes, palette }: { readonly preview: Puzzle3dCatalogueDropPreview; readonly meshes: readonly WorldMeshRecord[]; readonly palette: MeshStylePalette }) {
  const style = palette.highlighted;
  const meshRecord = preview.meshUrl ? meshes.find((mesh) => mesh.url === preview.meshUrl) : undefined;
  const url = meshRecord?.url ?? preview.meshUrl;
  return (
    <group position={preview.origin as [number, number, number]} raycast={() => null}>
      {url ? (
        <Suspense fallback={null}>
          <GlbInstanceMesh url={url} color={style.meshColor} emissive={style.meshColor} emissiveIntensity={0.6} opacity={0.88} borderColor={palette.neutral.lineColor} revision="highlighted" />
        </Suspense>
      ) : (
        <mesh raycast={() => null}>
          <boxGeometry args={[1, 1, 1]} />
          <meshBasicMaterial color={style.meshColor} transparent opacity={0.42} depthWrite={false} />
        </mesh>
      )}
    </group>
  );
}
//#endregion CatalogueDrop

//#region WorldSelectionPreviewStore
export type WorldSelectionPreview = {
  readonly sourceId: string;
  readonly mergedComponentIds: readonly number[] | null;
  readonly mergedInstanceIds: readonly string[] | null;
};

/** @emoji 🎯️ Live selection previews keyed by controller so every sibling World3d pane paints the same in-progress marquee selection before the plugin round-trip commits it. */
const worldSelectionPreviewByController = new Map<string, WorldSelectionPreview>();
const worldSelectionPreviewListeners = new Set<() => void>();

function worldSelectionPreviewsEqual(a: WorldSelectionPreview | null | undefined, b: WorldSelectionPreview | null | undefined): boolean {
  if (a === b) return true;
  if (!a || !b || a.sourceId !== b.sourceId) return false;
  const idsEqual = <T,>(left: readonly T[] | null, right: readonly T[] | null) => left === right || (left !== null && right !== null && left.length === right.length && left.every((id, index) => id === right[index]));
  return idsEqual(a.mergedComponentIds, b.mergedComponentIds) && idsEqual(a.mergedInstanceIds, b.mergedInstanceIds);
}

function notifyWorldSelectionPreviewListeners(): void {
  for (const listener of worldSelectionPreviewListeners) listener();
}

/** @emoji 🎯️ Subscribes a World3d pane to live selection previews from sibling panes. */
export function subscribeWorldSelectionPreview(listener: () => void): () => void {
  worldSelectionPreviewListeners.add(listener);
  return () => {
    worldSelectionPreviewListeners.delete(listener);
  };
}

/** @emoji 🎯️ Returns the live selection preview for `controllerId`, or `null` outside an in-progress gesture. */
export function getWorldSelectionPreview(controllerId: string): WorldSelectionPreview | null {
  return worldSelectionPreviewByController.get(controllerId) ?? null;
}

/** @emoji 🎯️ SSR snapshot for {@link useSyncExternalStore}; selection gestures never hydrate in progress. */
export function getWorldSelectionPreviewServerSnapshot(_controllerId: string): WorldSelectionPreview | null {
  return null;
}

/** @emoji 🎯️ Publishes an in-progress marquee result to every World3d pane using the same controller. */
export function setWorldSelectionPreview(controllerId: string, preview: WorldSelectionPreview | null): void {
  const previous = worldSelectionPreviewByController.get(controllerId) ?? null;
  if (worldSelectionPreviewsEqual(previous, preview)) return;
  if (preview) worldSelectionPreviewByController.set(controllerId, preview);
  else worldSelectionPreviewByController.delete(controllerId);
  notifyWorldSelectionPreviewListeners();
}

/** @emoji 🎯️ Clears a preview only when `sourceId` still owns it, preventing an idle sibling pane from cancelling the active pane's gesture. */
export function clearWorldSelectionPreview(controllerId: string, sourceId?: string): void {
  const current = worldSelectionPreviewByController.get(controllerId);
  if (!current || (sourceId && current.sourceId !== sourceId)) return;
  setWorldSelectionPreview(controllerId, null);
}
//#endregion WorldSelectionPreviewStore

//#region WorldInstanceChromeStore
type WorldInstanceChromeSnapshot = {
  readonly selectedIds: ReadonlySet<string>;
  readonly hoveredId: string | null;
  readonly hoveredKindId: string | null;
  readonly previewInstanceIds: ReadonlySet<string> | null;
};

interface WorldInstanceChromeStore {
  subscribe: (listener: () => void) => () => void;
  getSnapshot: () => WorldInstanceChromeSnapshot;
  setSnapshot: (next: WorldInstanceChromeSnapshot) => void;
  isSelected: (instanceId: string) => boolean;
  isHovered: (instanceId: string) => boolean;
  isHighlighted: (objectKind: string | undefined) => boolean;
  previewSelected: (instanceId: string) => boolean | undefined;
}

function chromeSnapshotsEqual(a: WorldInstanceChromeSnapshot, b: WorldInstanceChromeSnapshot): boolean {
  if (a.hoveredId !== b.hoveredId || a.hoveredKindId !== b.hoveredKindId) return false;
  if (a.previewInstanceIds !== b.previewInstanceIds) {
    if (!a.previewInstanceIds || !b.previewInstanceIds || a.previewInstanceIds.size !== b.previewInstanceIds.size) return false;
    for (const id of a.previewInstanceIds) {
      if (!b.previewInstanceIds.has(id)) return false;
    }
  }
  if (a.selectedIds.size !== b.selectedIds.size) return false;
  for (const id of a.selectedIds) {
    if (!b.selectedIds.has(id)) return false;
  }
  return true;
}

function createWorldInstanceChromeStore(): WorldInstanceChromeStore {
  let snapshot: WorldInstanceChromeSnapshot = {
    selectedIds: new Set(),
    hoveredId: null,
    hoveredKindId: null,
    previewInstanceIds: null,
  };
  const listeners = new Set<() => void>();
  const notify = (): void => {
    for (const listener of listeners) listener();
  };
  return {
    subscribe(listener) {
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },
    getSnapshot() {
      return snapshot;
    },
    setSnapshot(next) {
      if (chromeSnapshotsEqual(snapshot, next)) return;
      snapshot = next;
      notify();
    },
    isSelected(instanceId) {
      return snapshot.selectedIds.has(instanceId);
    },
    isHovered(instanceId) {
      return snapshot.hoveredId === instanceId;
    },
    isHighlighted(objectKind) {
      return objectKind != null && snapshot.hoveredKindId != null && objectKind === snapshot.hoveredKindId;
    },
    previewSelected(instanceId) {
      if (!snapshot.previewInstanceIds) return undefined;
      return snapshot.previewInstanceIds.has(instanceId);
    },
  };
}

const WorldInstanceChromeContext = reactHostPort.createContext<WorldInstanceChromeStore | null>(null);

function useWorldInstanceChromeStore(): WorldInstanceChromeStore {
  const store = reactHostPort.useContext(WorldInstanceChromeContext);
  if (!store) throw new Error("World instance chrome hooks must render inside WorldInstancesLayer");
  return store;
}

function useWorldInstanceChrome(
  instanceId: string,
  objectKind: string | undefined,
): {
  readonly selected: boolean;
  readonly hovered: boolean;
  readonly highlighted: boolean;
  readonly previewSelected: boolean | undefined;
} {
  const store = useWorldInstanceChromeStore();
  const selected = reactHostPort.useSyncExternalStore(store.subscribe, () => store.isSelected(instanceId), () => false);
  const hovered = reactHostPort.useSyncExternalStore(store.subscribe, () => store.isHovered(instanceId), () => false);
  const highlighted = reactHostPort.useSyncExternalStore(store.subscribe, () => store.isHighlighted(objectKind), () => false);
  const previewSelected = reactHostPort.useSyncExternalStore(store.subscribe, () => store.previewSelected(instanceId), () => undefined);
  return { selected, hovered, highlighted, previewSelected };
}

let interactivePluginActionsInFlight = 0;

/** @emoji 🖱️ Marks the start of a user-driven plugin action so background ticks can yield the WASM queue. */
export function beginInteractivePluginAction(): void {
  interactivePluginActionsInFlight += 1;
}

/** @emoji 🖱️ Marks the end of a user-driven plugin action so background ticks can resume. */
export function endInteractivePluginAction(): void {
  interactivePluginActionsInFlight = Math.max(0, interactivePluginActionsInFlight - 1);
}

function interactivePluginActionInFlight(): boolean {
  return interactivePluginActionsInFlight > 0;
}
//#endregion WorldInstanceChromeStore

/** @emoji 🖱️➡️ Signed distance along `axis` (unit vector) from `origin` to the point on that line closest to the
 * camera ray through the current pointer position — the standard closest-point-between-two-lines
 * construction, used so a face-normal drag tracks naturally instead of needing a ground/tangent-plane
 * intersection (which is undefined for motion parallel to the plane, i.e. exactly along the normal). */
function axisDragParam(clientX: number, clientY: number, hostRect: DOMRect, camera: import("three").Camera, origin: readonly [number, number, number], axis: readonly [number, number, number]): number | null {
  const ndcX = ((clientX - hostRect.left) / hostRect.width) * 2 - 1;
  const ndcY = -(((clientY - hostRect.top) / hostRect.height) * 2 - 1);
  const ray = worldRayFromNdc(ndcX, ndcY, camera);
  if (!ray) return null;
  const axisOrigin = new Vector3(origin[0], origin[1], origin[2]);
  const axisDirection = new Vector3(axis[0], axis[1], axis[2]).normalize();
  const originDelta = ray.origin.clone().sub(axisOrigin);
  const a = ray.direction.dot(ray.direction);
  const b = ray.direction.dot(axisDirection);
  const c = axisDirection.dot(axisDirection);
  const d = ray.direction.dot(originDelta);
  const e = axisDirection.dot(originDelta);
  const denominator = a * c - b * b;
  if (Math.abs(denominator) < 1e-9) return null;
  return (a * e - b * d) / denominator;
}

/** @emoji 🪪️ Element id of one world surface's projection pane. A world surface is mounted once per OPEN WINDOW
 * INSTANCE of its kind (`WindowInstanceIdContext`), and {@link Pane} renders its `id` on the pane root plus two
 * derived control ids (`…pane.fold`, `…pane.foldControl`), so the bare `framework.worldOrbit.projection` it used
 * to carry put three DUPLICATE DOM ids in the document per extra pane — invalid HTML, and it makes the pane
 * unaddressable through `<label for>`, `aria-labelledby` and any automation. The instance segment is the same
 * camelCase {@link childElementId} convention the window element ids use (`…projection.puzzle3dMainPerspective`). */
export function world3dProjectionPaneElementId(windowElementSegment: string): string {
  return childElementId("framework.worldOrbit.projection", windowElementSegment);
}

/** @emoji 🔀️ Portals the world's projection-kind switch into the enclosing window's pane host (see `usePaneSlot`), defaulting to bottom-right under the navigation cube — the cube sits above the folded chrome and the unfolded pane grows over it. Falls back to a local overlay when no pane host is mounted yet (or outside one). */
export function WorldOrbitProjectionSwitchPane({ spec, onSpecChange, windowElementSegment }: { readonly spec: WorldProjectionSpec; readonly onSpecChange: (spec: WorldProjectionSpec) => void; readonly windowElementSegment: string }) {
  const [anchor, setAnchor] = useState<Anchor>("bottom-right");
  const [folded, setFolded] = useState(true);
  const projectionLabel = useLabel("ui.host.projection");
  const paneId = world3dProjectionPaneElementId(windowElementSegment);
  const pane = (
    <Pane id={paneId} anchor={anchor} onAnchorChange={setAnchor} folded={folded} onFoldToggle={() => setFolded((value) => !value)} icon={worldProjectionSpecIconId(spec) as IconName} label={projectionLabel}>
      <WorldProjectionKindSwitch id={paneId} spec={spec} onSpecChange={onSpecChange} />
    </Pane>
  );
  const portaled = usePaneSlot(pane);
  if (portaled) return portaled;
  return (
    <div className="pointer-events-none absolute inset-0" data-slot="world-orbit-projection-switch-fallback" data-world-projection-kind-switch-host="">
      {pane}
    </div>
  );
}

//#region WorldWindowInstance
/** @emoji 🪪️ Identifies which window *pane* (not just which window *kind*) a `ComponentSceneHost` is
 * mounted for — every window kind's `UiNode` is shared verbatim across all of its open instances, so a
 * host can't otherwise tell which pane it is; provided per-pane around each `<InterpretedUiNode>` call. */
export const WindowInstanceIdContext = createContext<string | null>(null);

/** @emoji 🪟️ One-shot initial camera pose, keyed by window instance id, consumed by {@link World3dHost} on
 * mount and then discarded — the side-channel a Display "Windows" drag-and-drop template uses to seed a
 * freshly-opened pane's projection (dragging "Top" opens a pane that starts in the Top view, etc.). */
const pendingWorldProjectionByWindowId = new Map<string, WorldProjectionSpec>();

/** @emoji 🪟️ Registers `spec` to be consumed once by the next {@link World3dHost} mounted for `windowId`. */
export function registerPendingWorldProjection(windowId: string, spec: WorldProjectionSpec): void {
  pendingWorldProjectionByWindowId.set(windowId, spec);
}

/** @emoji 🪟️ Peeks the sticky initial projection for `windowId` — kept until the pane is closed so React
 * Strict Mode's mount→unmount→remount pass still seeds both Top and Perspective (take-on-read / clear-on-apply
 * left the second mount on the shared scene camera and empty transparent canvases). */
function peekPendingWorldProjection(windowId: string | null): WorldProjectionSpec | null {
  if (!windowId) return null;
  return pendingWorldProjectionByWindowId.get(windowId) ?? null;
}

/** @emoji 🪟️ Drops the initial projection seed when a pane is closed (not after first apply). */
export function clearPendingWorldProjection(windowId: string | null): void {
  if (!windowId) return;
  pendingWorldProjectionByWindowId.delete(windowId);
}
//#endregion WorldWindowInstance

//#region 🎯️WorldInteractionDomain
/** 🎯️ Granularity a plain world instance hit reports when the scene declares no
 * `domainGranularityId` — matches the node graph's own port granularity, which is what an
 * instance standing for one output channel resolves to. */
export const WORLD3D_DEFAULT_INTERACTION_GRANULARITY = "handle";

/** 🖱️ Mirrors `nodeGraphHoverActionArgs` (`NodeGraph/🟦️.tsx`) for a world window bound to a
 * framework interaction domain (`scene.domainId`), reporting the scene's own
 * `domainGranularityId` — the same shape a `domain_id`-aware `interactionHover` handler expects. */
export function world3dHoverActionArgs(domainId: string, granularity: string, id: string | null | undefined) {
  const targets = id ? [{ granularity, id }] : [];
  return { domainId, channel: "pointer", targets: JSON.stringify(targets) };
}

/** @emoji ✨ Alt+right-click opens suggestions only when a vortex marker is already hovered. */
export function world3dSuggestionsGestureArmed(altKey: boolean, hoveredVortexFullId: string | null | undefined): boolean {
  return Boolean(altKey && hoveredVortexFullId);
}

/** @emoji ✨ Playwright/macOS often omit `event.altKey` on reconstructed pointer events — track the key. */
export function world3dSuggestionsAltHeld(eventAltKey: boolean, altHeld: boolean): boolean {
  return Boolean(eventAltKey || altHeld);
}

/** @emoji ✨ Guest `interactionHover` may stay null after a spawn-admit miss — keep the local marker hover for Alt+right-click. */
export function world3dRetainLocalVortexHover(local: string | null | undefined, guest: string | null | undefined): string | null {
  return guest || local || null;
}

/** @emoji ✨ Alt+right-click is the suggestions gesture — consume contextmenu so workspace chrome cannot steal it. */
export function world3dSuggestionsGestureConsumesContextMenu(altKey: boolean): boolean {
  return altKey;
}

/** @emoji ✨ Chrome overlays steal bubbling button=2 before orbit's canvas listener — window capture over the host is the route. */
export function world3dSuggestionsRightDownRoutesOnWindowCapture(button: number, overHost: boolean): boolean {
  return button === 2 && overHost;
}

type World3dSuggestionsRightDownRoute = {
  readonly host: () => HTMLElement | null;
  readonly handle: (event: PointerEvent) => boolean;
};
const world3dSuggestionsRightDownRoutes = new Set<World3dSuggestionsRightDownRoute>();

function world3dSuggestionsEventOverHost(node: HTMLElement | null, clientX: number, clientY: number): boolean {
  if (!node) return false;
  const rects = [node.getBoundingClientRect(), ...Array.from(node.querySelectorAll("canvas")).map((canvas) => canvas.getBoundingClientRect())];
  return rects.some((rect) => rect.width > 1 && rect.height > 1 && clientPointOverHost(clientX, clientY, rect));
}

function world3dSuggestionsDispatchWindowRightDown(event: PointerEvent): void {
  if (event.button !== 2) return;
  console.warn(`[DEBUG] suggestions-rightdown capture button=2 routes=${world3dSuggestionsRightDownRoutes.size} x=${event.clientX} y=${event.clientY}`);
  for (const route of world3dSuggestionsRightDownRoutes) {
    const overHost = world3dSuggestionsEventOverHost(route.host(), event.clientX, event.clientY);
    if (!world3dSuggestionsRightDownRoutesOnWindowCapture(event.button, overHost)) continue;
    if (route.handle(event) !== false) continue;
    event.preventDefault();
    event.stopPropagation();
    return;
  }
}

function world3dSuggestionsDispatchWindowContextMenu(event: MouseEvent): void {
  console.warn(`[DEBUG] suggestions-contextmenu capture routes=${world3dSuggestionsRightDownRoutes.size} x=${event.clientX} y=${event.clientY}`);
  for (const route of world3dSuggestionsRightDownRoutes) {
    if (!world3dSuggestionsEventOverHost(route.host(), event.clientX, event.clientY)) continue;
    const asPointer = event as unknown as PointerEvent;
    if (route.handle(asPointer) !== false) continue;
    event.preventDefault();
    event.stopPropagation();
    return;
  }
}

if (typeof window !== "undefined") {
  window.addEventListener("pointerdown", world3dSuggestionsDispatchWindowRightDown, true);
  window.addEventListener("contextmenu", world3dSuggestionsDispatchWindowContextMenu, true);
}

/** 🖱️ Mirrors `nodeGraphSelectionActionArgs` for a world window bound to a framework interaction
 * domain — `merge` is passed through as already resolved at the call site (marquee/click modifier
 * state), not recomputed here. */

/** @emoji 🖱️ Instance picks join a scene interaction domain only when the instance names a topology id. */
export function world3dInstancePickUsesInteractionDomain(record: { readonly interactionId?: string } | undefined): boolean {
  return typeof record?.interactionId === "string" && record.interactionId.length > 0;
}

/** @emoji 🖱️ A selection is a SET of topology ids, so the target list this builds is one: the same id
 * can be handed in twice (several rendered instances of one channel, a pick batch the caller did not
 * run through {@link interactionTargetsForInstances}), and a `Select` must be idempotent per id —
 * the host never emits a duplicate target. Live defect 2026-09-12 (ticket
 * 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️selection-dedupe-2026-09-12.md`): the guest republished one
 * picked id twice, and only a set on BOTH sides makes that unrepresentable. */
/** 🎯️ `merge` is a `MergeMode` — the ONE vocabulary of `🕹️interaction/🧬️schema/🔣️.json`, put on the
 * wire verbatim. A host must NOT translate it: the framework's `parse_merge_mode` accepts exactly
 * those five words and faults on anything else. */
export function world3dSelectionActionArgs(domainId: string, granularity: string, ids: readonly string[], merge: MergeMode) {
  const targets = [...new Set(ids)].map((id) => ({ granularity, id }));
  return { domainId, targets: JSON.stringify(targets), merge, method: "pick" };
}

/** 🎯️ Resolves rendered instance ids onto the interaction targets they stand for, deduplicated and
 * order-preserving — several instances can share one target (procedural3d renders one instance per
 * geometry item of a channel, all of which resolve to that channel's port). */
export function interactionTargetsForInstances(instances: readonly WorldInstanceRecord[], ids: readonly string[]): readonly string[] {
  const seen = new Set<string>();
  const targets: string[] = [];
  for (const id of ids) {
    const target = instances.find((entry) => entry.id === id)?.interactionId ?? id;
    if (seen.has(target)) continue;
    seen.add(target);
    targets.push(target);
  }
  return targets;
}

/** 🧿️ The pickable non-instance layers of a `World3dScene`, each named after the scene field it is
 * parsed from (`vorticesJson`, `attractionsJson`, `targetVolumesJson`, `referencesJson`). */
export type World3dMarkerLayer = "vortex" | "attraction" | "targetVolume" | "reference";

/** 🧿️ Granularity a marker hit reports when its scene record declares no `interactionGranularityId`
 * — the layer's own name, the same way {@link WORLD3D_DEFAULT_INTERACTION_GRANULARITY} names the
 * instance layer's default. Keeps the host free of any per-app granularity vocabulary: an app that
 * calls its marker rows something else overrides it per record from the scene JSON. */
export const WORLD3D_DEFAULT_MARKER_GRANULARITY: Readonly<Record<World3dMarkerLayer, string>> = {
  vortex: "vortex",
  attraction: "attraction",
  targetVolume: "targetVolume",
  reference: "reference",
};

/** 🧿️ What every pickable marker record may carry to redirect its own hit — both read straight off
 * the scene JSON, so which interaction target a marker stands for is the plugin's statement, never
 * the host's guess. */
export type World3dMarkerInteractionFields = {
  readonly interactionId?: string;
  readonly interactionGranularityId?: string;
};

/** 🧿️ Resolves the `{ granularity, id }` interaction target one marker hit stands for — the record's
 * own scene-JSON fields win, else the record's own id at its layer's default granularity. */
export function world3dMarkerInteractionTarget(layer: World3dMarkerLayer, id: string, record?: World3dMarkerInteractionFields): { readonly granularity: string; readonly id: string } {
  return { granularity: record?.interactionGranularityId ?? WORLD3D_DEFAULT_MARKER_GRANULARITY[layer], id: record?.interactionId ?? id };
}
//#endregion 🎯️WorldInteractionDomain

//#region World3dHost
/** 🚚️ `node.world3d` arrives ASSEMBLED. Its payload fields (`instancesJson`, `vorticesJson`, … — see
 * `WORLD3D_SCENE_LANES`) no longer ride inside the surface doc's 32 KiB `UiFixedBytes` blob, which
 * cannot page and refused a 57 KB Nakagin world outright; each is published as its own retained,
 * individually paged text carrier beside the surface node and reattached by the Interpreter's
 * `PagedSurfaceView` before this host ever sees it (ticket 26/09/02 wave P). Every `*Json` field is
 * therefore still a plain string here, and the per-lane `useMemo`s below stay the incremental seam:
 * a lane whose content did not change keeps its identical string, so its parse never re-runs. */
export function World3dHost({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.world3d;
  // 🪟️ Non-empty only — an empty-string `domainId` (never emitted by the Rust side, but defensive
  // against a hand-authored fixture) must fall back to the plugin-private action pathway below.
  const interactionDomainId = scene?.domainId ? scene.domainId : undefined;
  const interactionGranularity = scene?.domainGranularityId ? scene.domainGranularityId : WORLD3D_DEFAULT_INTERACTION_GRANULARITY;
  // 🐚️ Optional — this host is also unit-tested standalone, outside any `ShellScopeProvider`.
  const shellScope = useShellScopeOptional();
  // 🐚️ Read as reactive state (not `shellScope.selection.get()` inline at each use) because this value
  // is consumed inside `<Canvas>` r3f subtrees (`WorldInstancesLayer`) via a prop, not context — R3F
  // primitives aren't guaranteed to re-render just because an *outer* DOM component's context changed,
  // so the toolbar's mode toggle needs an explicit subscription to actually propagate.
  const [persistentSelectionMode, setPersistentSelectionMode] = useState<MergeMode>(() => shellScope?.selection.get() ?? "replace");
  useEffect(() => {
    if (!shellScope) return;
    return shellScope.selection.subscribe(() => setPersistentSelectionMode(shellScope.selection.get()));
  }, [shellScope]);
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const setWindowTitle = useContext(SetWindowTitleContext);
  const setWindowIcon = useContext(SetWindowIconContext);
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const meshStylePalette = useMeshStylePalette();
  const colors = useMemo(() => semanticColorsFromPalette(meshStylePalette), [meshStylePalette]);
  const sceneCameraJson = scene?.cameraJson ?? "{}";
  const parsedCamera = useMemo(() => parseCameraState(sceneCameraJson), [sceneCameraJson]);
  // 🚚️ B44: the retained instance set advances by the producer's own per-object delta when it can, so a
  // pose edit on a 180-object document substitutes the moved records by id instead of re-parsing 55 KiB.
  const instanceResidencyRef = useRef<WorldInstanceResidencyV1 | null>(null);
  const instances = useMemo(() => {
    const advanced = advanceWorldInstanceResidency(instanceResidencyRef.current, scene?.instancesJson ?? "[]", scene?.instancesDeltaJson ?? null);
    instanceResidencyRef.current = advanced;
    return advanced.records as WorldInstanceRecord[];
  }, [scene?.instancesJson, scene?.instancesDeltaJson]);
  const references = useMemo(() => parseJsonArray<WorldReferenceRecord>(scene?.referencesJson), [scene?.referencesJson]);
  const sceneCamera = useMemo(() => {
    if (sceneCameraJson.includes('"position"')) return parsedCamera;
    return instances.length > 0 ? autofitCameraFromInstances(instances) : parsedCamera;
  }, [instances, parsedCamera, sceneCameraJson]);
  const contentBounds = useMemo(() => worldSceneContentBounds(instances, references), [instances, references]);
  const pendingProjectionSpecRef = useRef<WorldProjectionSpec | null>(null);
  const [viewportCamera, setViewportCamera] = useState<WorldParsedCameraState | null>(() => {
    const pendingSpec = peekPendingWorldProjection(windowInstanceId);
    if (!pendingSpec) return null;
    pendingProjectionSpecRef.current = pendingSpec;
    return seedPendingWorldProjectionCamera(pendingSpec, sceneCamera, instances, references);
  });
  const [projectionFramePending, setProjectionFramePending] = useState(() => pendingProjectionSpecRef.current !== null);
  const [viewportOwned, setViewportOwned] = useState(false);
  const [detachEpoch, setDetachEpoch] = useState(0);
  /** 📷️ First content-frame remounts orbit controls; later fill-driven bound expansions only soft-update the camera. */
  const projectionContentFrameSeededRef = useRef(false);
  const previousSceneCameraJsonRef = useRef(sceneCameraJson);
  /** 🧭️ The last camera pose this component itself dispatched via debounced `setCamera` — lets the reattach
   * effect below recognize the plugin echoing it straight back (see `shouldReattachWorldViewportCamera`). */
  const lastDispatchedWorldCameraRef = useRef<WorldCameraState | null>(null);
  useEffect(() => {
    // 🧭️ Always advance the tracking ref, even when we're about to suppress a reattach below — otherwise the
    // NEXT comparison would still diff against this stale value instead of the pose we just saw.
    const previousSceneCameraJson = previousSceneCameraJsonRef.current;
    previousSceneCameraJsonRef.current = sceneCameraJson;
    if (!shouldReattachWorldViewportCamera(previousSceneCameraJson, sceneCameraJson, lastDispatchedWorldCameraRef.current)) return;
    setViewportCamera(null);
    setViewportOwned(false);
    setDetachEpoch(0);
    setProjectionFramePending(Boolean(pendingProjectionSpecRef.current));
    projectionContentFrameSeededRef.current = false;
  }, [sceneCameraJson]);
  const cameraState = viewportCamera ?? sceneCamera;
  const cameraSeedKey = world3dViewportCameraSeedKey(sceneCameraJson, detachEpoch);

  // 🎥️ Registers this window's live camera get/set for tutorial playback/recording (see
  // `registerTutorialCameraDriver` — modeled on `registerIntroductionSurfaceResolver`). `get` reads the
  // CURRENT pose via a ref (never a stale closure) for deviation-then-play convergence; `set` writes a
  // viewport-owned override and bumps `detachEpoch` so `OrbitControls` reseeds from the new pose instead
  // of fighting it, exactly like a programmatic view-preset apply already does.
  const cameraStateRef = useRef(cameraState);
  cameraStateRef.current = cameraState;
  useEffect(() => {
    if (!windowInstanceId) return undefined;
    const driver: TutorialCameraDriver = {
      get: () => {
        const live = cameraStateRef.current;
        return {
          kind: "orbit",
          position: [live.position[0], live.position[1], live.position[2]],
          target: [live.target[0], live.target[1], live.target[2]],
          up: live.up ? [live.up[0], live.up[1], live.up[2]] : [0, 0, 1],
          fov: live.fov,
        };
      },
      set: (pose) => {
        if (pose.kind !== "orbit") return;
        const live = cameraStateRef.current;
        setViewportCamera({ ...live, position: pose.position, target: pose.target, up: pose.up ?? live.up, fov: pose.fov ?? live.fov });
        setDetachEpoch((epoch) => epoch + 1);
      },
    };
    return registerTutorialCameraDriver(windowInstanceId, driver);
  }, [windowInstanceId]);
  const meshes = useMemo(() => parseMeshes(scene?.meshesJson ?? "[]"), [scene?.meshesJson]);
  const leftoverSelectionEpoch = useSyncExternalStore((listener) => {
    leftoverWorldSelectionListeners.add(listener);
    return () => leftoverWorldSelectionListeners.delete(listener);
  }, leftoverWorldSelectionOverlayV1, leftoverWorldSelectionOverlayV1);
  const selection = useMemo(() => mergeWorldSelectionWithLeftover(parseSelection(scene?.selectionJson ?? "{}"), windowInstanceId, instances), [leftoverSelectionEpoch, scene?.selectionJson, instances, windowInstanceId]);
  const vortices = useMemo(() => parseJsonArray<WorldVortexRecord>(scene?.vorticesJson), [scene?.vorticesJson]);
  const attractions = useMemo(() => parseJsonArray<WorldAttractionRecord>(scene?.attractionsJson), [scene?.attractionsJson]);
  const targetVolumes = useMemo(() => parseJsonArray<WorldTargetVolumeRecord>(scene?.targetVolumesJson), [scene?.targetVolumesJson]);
  const interaction = useMemo(() => mergeWorldInteractionWithLeftoverV1(parseInteraction(scene?.interactionJson), leftoverWorldWindowOverlayV1(windowInstanceId)), [leftoverSelectionEpoch, scene?.interactionJson, windowInstanceId]);
  const lod = useMemo(() => parseLod(scene?.lodJson), [scene?.lodJson]);
  const engagementPreview = useMemo(() => parseEngagementPreview(scene?.engagementPreviewJson), [scene?.engagementPreviewJson]);
  const retainedBrushPreviewByVortexRef = useRef<Record<string, string>>({});
  const brushPreviewJson = useMemo(() => {
    const interaction = parseInteraction(scene?.interactionJson);
    const paneLeftover = leftoverWorldWindowOverlayV1(windowInstanceId);
    const hover = leftoverHoveredVortexFullIdV1(paneLeftover) ?? interaction.hoveredVortexFullId;
    const leftoverPreview = paneLeftover?.brushPreviewJson;
    const published = leftoverPreview || scene?.brushPreviewJson || interaction.brushPreviewJson;
    const decided = retainWorldBrushPreviewJsonV1(published, hover, retainedBrushPreviewByVortexRef.current);
    retainedBrushPreviewByVortexRef.current = decided.retained;
    return decided.json;
  }, [leftoverSelectionEpoch, scene?.brushPreviewJson, scene?.interactionJson, windowInstanceId]);
  const brushPreview = useMemo(() => parseWorldBrushPreview(brushPreviewJson || undefined), [brushPreviewJson]);
  const latestFillIdentityRef = useRef<readonly [number, number, number, number, number] | null>(null);
  const suppliedFillDiagnostic = brushPreview?.fillBuildPreview ?? null;
  let fillDiagnostic: WorldFillDiagnosticRecord | null = null;
  if (suppliedFillDiagnostic) {
    const identity = [suppliedFillDiagnostic.operation, suppliedFillDiagnostic.baseRevision, suppliedFillDiagnostic.registryGeneration, suppliedFillDiagnostic.generation, suppliedFillDiagnostic.sequence] as const;
    const latest = latestFillIdentityRef.current;
    const fresh =
      !latest ||
      identity[0] > latest[0] ||
      (identity[0] === latest[0] && identity[1] > latest[1]) ||
      (identity[0] === latest[0] && identity[1] === latest[1] && identity[2] > latest[2]) ||
      (identity[0] === latest[0] && identity[1] === latest[1] && identity[2] === latest[2] && identity[3] > latest[3]) ||
      (identity[0] === latest[0] && identity[1] === latest[1] && identity[2] === latest[2] && identity[3] === latest[3] && identity[4] >= latest[4]);
    if (fresh) {
      latestFillIdentityRef.current = identity;
      fillDiagnostic = suppliedFillDiagnostic;
    }
  }
  const environment = useMemo(() => parseEnvironment(scene?.environmentJson), [scene?.environmentJson]);
  const frame = useMemo(() => parseFrame(scene?.frameJson), [scene?.frameJson]);
  const fit = useMemo(() => parseFit(scene?.fitJson), [scene?.fitJson]);
  // 🧵️ Off-main-thread compute status (see `World3dScene.statusJson`) — the meshes above stay the
  // last-known-good (stale) cache while a plugin worker's `flowEvalTick` chain is still resolving.
  const computeStatus = useMemo(() => world3dComputeStatusV1(scene?.statusJson), [scene?.statusJson]);
  const computing = computeStatus.computing;
  // 🛑️ While this surface offers a cancel affordance it DECLARES the action id to the shell, so the
  // shell's one action funnel can retire the requesting instance's in-flight extension requests
  // before forwarding the gesture — without the shell ever learning a domain verb from code
  // (`isDeclaredSurfaceCancelAction`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
  const cancelActionId = computeStatus.cancellable ? computeStatus.cancelAction : "";
  useEffect(() => declareSurfaceCancelAction(cancelActionId), [cancelActionId]);
  const activeUtility = interaction.activeUtility ?? "select";
  const fillMode = activeUtility === "fill";
  const visibleBrushPreview = fillMode ? (fillDiagnostic?.candidateGhost ? brushPreview : null) : brushPreview;
  const brushMode = activeUtility === "brush";
  const volumeBrushMode = activeUtility === "volumeBrush";
  const relocateMode = activeUtility === "worldRelocate";
  const volumeLayersInteractive = !brushMode && !fillMode && !volumeBrushMode;
  const hostRef = useRef<HTMLDivElement | null>(null);
  const instancesGroupRef = useRef<Group | null>(null);
  const lodRef = useRef(DEFAULT_MANUAL_LOD);
  const [marqueePath, setMarqueePath] = useState<readonly SelectionMarqueePoint[]>([]);
  const [marqueeModifiers, setMarqueeModifiers] = useState<{ readonly shiftKey: boolean; readonly ctrlKey: boolean; readonly metaKey: boolean }>({ shiftKey: false, ctrlKey: false, metaKey: false });
  const [gumballDragActive, setGumballDragActive] = useState(false);
  const [faceDragSession, setFaceDragSession] = useState<{
    readonly objectId: string;
    readonly faceId: number;
    readonly normal: readonly [number, number, number];
    readonly startPoint: readonly [number, number, number];
    readonly faceExtent?: readonly [number, number];
  } | null>(null);
  const [connectDragSource, setConnectDragSource] = useState<{
    readonly fullId: string;
    readonly position: readonly [number, number, number];
    readonly record?: WorldVortexRecord;
  } | null>(null);
  const [connectDragHoverPosition, setConnectDragHoverPosition] = useState<readonly [number, number, number] | null>(null);
  const displayVortices = useMemo(() => {
    if (!connectDragSource) return vortices;
    if (vortices.some((vortex) => vortex.fullId === connectDragSource.fullId)) return vortices;
    const retained =
      connectDragSource.record ??
      ({
        fullId: connectDragSource.fullId,
        position: connectDragSource.position,
      } satisfies WorldVortexRecord);
    return [...vortices, retained];
  }, [connectDragSource, vortices]);
  const [vortexPointerArm, setVortexPointerArm] = useState<{
    readonly fullId: string;
    readonly position: readonly [number, number, number];
    readonly clientX: number;
    readonly clientY: number;
    readonly event: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean };
  } | null>(null);
  const [voxelHoverOrigin, setVoxelHoverOrigin] = useState<readonly [number, number, number] | null>(null);
  const [paintStrokeActive, setPaintStrokeActive] = useState(false);
  const catalogueDropPreview = useSyncExternalStore(
    subscribeWorldCatalogueDropPreview,
    () => getWorldCatalogueDropPreview(node.controllerId),
    () => getWorldCatalogueDropPreviewServerSnapshot(node.controllerId),
  );
  const sharedSelectionPreview = useSyncExternalStore(
    subscribeWorldSelectionPreview,
    () => getWorldSelectionPreview(node.controllerId),
    () => getWorldSelectionPreviewServerSnapshot(node.controllerId),
  );
  const [contextMenu, setContextMenu] = useState<(SurfaceContextMenuResult & { readonly x: number; readonly y: number }) | null>(null);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.scene");
  const cameraRef = useRef<import("three").Camera | null>(null);
  const catalogueDragDepthRef = useRef(0);
  const catalogueDragEncodedRef = useRef<string | null>(null);
  /** 🎉️ Pre-drop instance ids — when the scene gains new ids after {@link commitCatalogueDropAt}, those objects celebrate. */
  const pendingCelebrateCatalogueDropIdsRef = useRef<ReadonlySet<string> | null>(null);
  const instancesRef = useRef(instances);
  instancesRef.current = instances;
  const meshesRef = useRef(meshes);
  meshesRef.current = meshes;
  const marqueeStartRef = useRef<SelectionMarqueePoint | null>(null);
  const vorticesRef = useRef(vortices);
  vorticesRef.current = vortices;
  const referencesRef = useRef(references);
  referencesRef.current = references;
  const wasMarqueeDragRef = useRef(false);
  const [marqueeCommitHold, setMarqueeCommitHold] = useState<{
    readonly mergedComponentIds: readonly number[] | null;
    readonly mergedInstanceIds: readonly string[] | null;
  } | null>(null);
  const connectDropConsumedRef = useRef(false);
  const engagementPointerMoveInFlightRef = useRef(false);
  const engagementPointerMoveLastPointRef = useRef<readonly [number, number, number] | null>(null);
  /** 🚚️ Open Relocate-utility drag, or `null`. A ref, not state: mid-drag the only thing that changes is
   * the shared ghost origin, so the gesture never re-renders this host by itself (same discipline as the
   * gumball's local preview). */
  const relocateSessionRef = useRef<World3dRelocateSession | null>(null);
  const gumballDragStartPoseRef = useRef<GumballPose | null>(null);
  /** 🧲️ Serialized WASM begin/end chain — mid-drag is local-only; one absolute start→end delta commits on drag end. */
  const gumballDragChainRef = useRef(Promise.resolve());
  const selectionMode = selection.selectionMode ?? selection.granularity ?? "mesh";
  const gridSnapEnabled = lod.gridSnapEnabled ?? false;
  const suggestionMenuOpen = Boolean(interaction.suggestionMenu?.open);
  const suggestionMenuOwnsThisWindow = worldSuggestionMenuOwnsWindow(interaction.suggestionMenu, windowInstanceId ?? undefined);
  const suggestionMenuCheckingPlacementLabel = useLabel("ui.host.checkingPlacement");
  const suggestionMenuNoPlacementLabel = useLabel("ui.host.noPlacement");
  const suggestionMenuTitleLabel = useLabel("ui.surfaceContextMenu.placementSuggestions");
  const gridFactor = lod.gridFactor ?? interaction.gridFactor ?? DEFAULT_LOD_GRID_FACTOR;
  const marqueeDown = marqueePath.length > 0;
  const method = selection.method ?? "rectangle";
  const marqueeStart = marqueePath[0];
  const marqueeEnd = marqueePath[marqueePath.length - 1];
  const marqueeDragActive = marqueeDown && marqueePath.length > 1 && marqueeStart != null && marqueeEnd != null && Math.hypot(marqueeEnd.x - marqueeStart.x, marqueeEnd.y - marqueeStart.y) > MARQUEE_DRAG_THRESHOLD_PX;
  const marqueeMergeMode = useMemo(() => resolveWorldMergeMode(selection.selectionMergeMode, marqueeModifiers, persistentSelectionMode), [marqueeModifiers, selection.selectionMergeMode, persistentSelectionMode]);
  const marqueeCoverage: SelectionMarqueeCoverage = useMemo(() => {
    if (!marqueeDragActive || !marqueeStart || !marqueeEnd) return "full";
    return marqueeCoverageFromGesture({ method, startX: marqueeStart.x, endX: marqueeEnd.x, path: marqueePath });
  }, [marqueeDragActive, marqueeEnd, marqueePath, marqueeStart, method]);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({
        controllerId: node.controllerId,
        action,
        args: { surfaceId: node.surfaceId, windowId: windowInstanceId ?? node.surfaceId, ...args },
      });
    },
    [node.controllerId, node.surfaceId, onAction, windowInstanceId],
  );

  /** 🏁️ `dispatch`'s awaitable twin for every SELF-GATING lane — the coalescing hover dispatchers and the
   * two background tick intervals. `dispatch` discards `onAction`'s promise, and `onAction` is what settles
   * on the guest's `OperationCompleted` frame, so each gate built on "at most one round trip outstanding"
   * (`createCoalescingActionDispatcher`, `createInFlightSkippingInterval`) cleared itself on the next
   * microtask and gated NOTHING. Wave B33 measured one 70-move brush hover storm enqueuing 72
   * `interactionHover` + 85 `suggestionsTick` turns with 11/10 of them settled, after which the next user
   * action (`addTargetVolume`) waited behind the backlog past its 30 s budget
   * (`📓️2026-09-12-wave-B33-full-run-vs-fresh-lane.md` §3). Same envelope as `dispatch`, awaitable — the
   * shape `dispatchBrushMesh` already uses for the mesh page lane. */
  const dispatchSettled = useCallback(
    (action: string, args?: Record<string, unknown>) =>
      Promise.resolve(
        onAction({
          controllerId: node.controllerId,
          action,
          args: { surfaceId: node.surfaceId, windowId: windowInstanceId ?? node.surfaceId, ...args },
        }),
      ),
    [node.controllerId, node.surfaceId, onAction, windowInstanceId],
  );

  const adoptViewportCamera = useCallback(
    (next: WorldCameraState, applyToRig: boolean) => {
      setViewportCamera((prev) => mergeWorldViewportCamera(prev ?? sceneCamera, next));
      setViewportOwned(true);
      if (applyToRig) setDetachEpoch((epoch) => epoch + 1);
    },
    [sceneCamera],
  );

  // 🧭️ Syncs a completed user-driven camera gesture (orbit/pan/zoom end, gizmo view snap) to the plugin so
  // the guest owns one authoritative pose per window instance — the plugin re-derives every camera-dependent
  // projection (LOD, vortex markers, sun) from it, and a reattaching pane adopts it instead of snapping back.
  // `setCamera` is `ActionKind::View` publishing on the WindowConfig lane ONLY: it is per-window session view
  // state, never a document edit and never an artifact-history row (the plugin runtime's `dispatch_emit`
  // skips the command log for exactly this shape). Trailing-debounced like the 2D canvas' own camera sync
  // (`CAMERA_SYNC_DEBOUNCE_MS`) so a scroll/wheel burst doesn't spam one dispatch per tick. Never wired into
  // a programmatic camera change (auto-fit, scene-camera echo reattach) — only genuine user gestures call this.
  const cameraDispatchTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const dispatchWorldCameraDebounced = useCallback(
    (state: WorldCameraState) => {
      if (cameraDispatchTimeoutRef.current) clearTimeout(cameraDispatchTimeoutRef.current);
      cameraDispatchTimeoutRef.current = setTimeout(() => {
        cameraDispatchTimeoutRef.current = null;
        lastDispatchedWorldCameraRef.current = state;
        dispatch("setCamera", worldCameraSetCameraDispatchArgs(windowInstanceId ?? node.surfaceId, state));
      }, CAMERA_SYNC_DEBOUNCE_MS);
    },
    [dispatch, node.surfaceId, windowInstanceId],
  );
  useEffect(
    () => () => {
      if (cameraDispatchTimeoutRef.current) clearTimeout(cameraDispatchTimeoutRef.current);
    },
    [],
  );

  const referenceSelectedIds = useMemo(() => {
    if (!selection.referenceSelectedId) return new Set<string>();
    return new Set([selection.referenceSelectedId]);
  }, [selection.referenceSelectedId]);

  const targetVolumeSelectedIds = useMemo(() => new Set(selection.targetVolumeIds ?? []), [selection.targetVolumeIds]);

  const volumeGumballConfig = useMemo(() => {
    if (activeUtility !== "transform") return undefined;
    return selection.gumballConfig ?? gumballConfigForTransformMode("transform");
  }, [activeUtility, selection.gumballConfig]);

  const referenceHoveredId = useMemo(() => {
    const hovered = selection.hoveredId;
    if (!hovered?.startsWith("reference:")) return null;
    return hovered.slice("reference:".length);
  }, [selection.hoveredId]);

  const handleReferenceSelect = useCallback(
    (id: string) => {
      const reference = references.find((entry) => entry.id === id);
      if (interactionDomainId) {
        const target = world3dMarkerInteractionTarget("reference", id, reference);
        dispatch("interactionSelect", world3dSelectionActionArgs(interactionDomainId, target.granularity, reference?.locked ? [] : [target.id], "replace"));
        return;
      }
      if (reference?.locked) {
        dispatch("worldPick", {
          granularity: selectionMode,
          id: null,
          merge: "replace",
        });
        return;
      }
      dispatch("setReferenceSelection", {
        pane: paneSuffixFromSurfaceId(node.surfaceId),
        referenceId: id,
      });
    },
    [dispatch, interactionDomainId, node.surfaceId, references, selectionMode],
  );

  /** 🏁️ Reference-marker hover is a POINTERMOVE lane like the instance and vortex ones, so it coalesces onto
   * the latest marker and keeps one round trip outstanding — it used to dispatch one guest turn per move with
   * no gate at all, the same unbounded shape wave B33 measured on the vortex lane (§3). The record lookup
   * reads `referencesRef` so a republished scene cannot rebuild the dispatcher and drop its in-flight state. */
  const handleReferenceHover = useMemo(
    () =>
      createCoalescingActionDispatcher<string | null>((id) => {
        if (interactionDomainId) {
          const target = id ? world3dMarkerInteractionTarget("reference", id, referencesRef.current.find((entry) => entry.id === id)) : null;
          return dispatchSettled("interactionHover", world3dHoverActionArgs(interactionDomainId, target?.granularity ?? WORLD3D_DEFAULT_MARKER_GRANULARITY.reference, target?.id));
        }
        if (!id) return dispatchSettled("referenceHover", {});
        return dispatchSettled("referenceHover", { referenceId: id });
      }),
    [dispatchSettled, interactionDomainId],
  );

  const handleTargetVolumeSelect = useCallback(
    (id: string) => {
      const volume = targetVolumes.find((entry) => entry.id === id);
      if (interactionDomainId) {
        const target = world3dMarkerInteractionTarget("targetVolume", id, volume);
        dispatch("interactionSelect", world3dSelectionActionArgs(interactionDomainId, target.granularity, volume?.locked ? [] : [target.id], "replace"));
        return;
      }
      if (volume?.locked) {
        dispatch("worldPick", { granularity: selectionMode, id: null, merge: "replace" });
        return;
      }
      dispatch("setSelection", {
        selection: { objectIds: [], vortexIds: [], attractionIds: [], targetVolumeIds: [id], referenceIds: [] },
      });
    },
    [dispatch, interactionDomainId, selectionMode, targetVolumes],
  );

  const handleTargetVolumeRelocate = useCallback(
    (payload: WorldVolumeRelocatePayload) => {
      dispatch("relocateTargetVolume", {
        volumeId: payload.volumeId,
        mode: payload.mode,
        before: payload.before,
        after: payload.after,
      });
    },
    [dispatch],
  );

  // 🥽️ One GLB's collision geometry never fitted a single retained command: the shared puzzle contract
  // admits 8 192 raw JSON bytes and the Concrete Forest mesh alone encoded to 64 KB, so every upload is
  // a page run (`puzzle3dBrushMeshPages`) — the plugin stages the pages
  // under `(url, digest)` and installs the mesh when the last one lands. A mesh THIS GUEST already holds
  // is re-announced by `{url, digest}` alone, which it serves from its own content-addressed store
  // instead of taking the bytes again.
  //
  // 🚚️ Which meshes that is is `puzzle3dBrushMeshRegistry`'s to say, not this component's: the claim is
  // scoped to the guest instantiation that justified it, confirmed only when a run's LAST page goes out,
  // and voided by a fallen `meshResidency`. The claim used to live in a bare page-lifetime `Map` written
  // at enqueue time, so every activation after a guest restart re-announced seven identities the guest
  // held nothing for and the brush utility silently kept no collision geometry until a full reload.
  //
  // ⏳️ The run is a BACK-PRESSURED stream, never a burst. A macrotask drain (`setTimeout(drain, 0)`) put
  // every page of every mesh into the actor's one command queue within a couple of hundred milliseconds
  // and then left them there: browser-measured 2026-09-12 on wasm #47, 202 pages enqueued in 21 s, 40 of
  // them settled over the next 420 s, and ONE user click that landed mid-run waited 44.3 s behind 15
  // pages (`📓️2026-09-12-wave-B22-brush-mesh-upload.md`). Awaiting each page's own settle before the
  // next is queued bounds what a user action can ever queue behind at ONE page, whatever the run's
  // length — `onAction` settles on the dispatched action's typed-operation completion, which is exactly
  // the "this page is in" signal the old fire-and-forget shape threw away.
  //
  // 🪢️ And a run is only ever paged ONCE PER DIGEST. Every `dist/mesh/*.glb` in this repo is the same
  // 771 728-byte capsule, so a scene placing several object kinds paged byte-identical geometry once per
  // mesh id — 72 commands each. A digest this page already put into THIS guest is announced by
  // `{url, digest}` alone and the guest aliases its own derived page onto the new id
  // (`adopt_brush_mesh_by_digest`, `✏️editor/⏳️precompute/🦀️.rs`); a guest that cannot serve it answers
  // on `meshReuploadUrls` and the claim is re-driven, so an optimistic announcement is self-healing.
  const brushMeshQueueRef = useRef<Puzzle3dBrushMeshPage[]>([]);
  const brushMeshRunsRef = useRef(new Map<string, string>());
  const brushMeshDrainingRef = useRef(false);
  const brushMeshLiveRef = useRef(true);
  const [brushMeshRevisions, setBrushMeshRevisions] = useState<{ readonly generation: number; readonly urls: Readonly<Record<string, number>> }>({ generation: 0, urls: {} });
  /** 🥽️ `dispatch`'s awaitable twin for the one lane that needs back pressure — same envelope, but the
   * caller can wait for the page to be IN before it queues the next one. */
  const dispatchBrushMesh = useCallback(
    (args: Record<string, unknown>) =>
      Promise.resolve(
        onAction({
          controllerId: node.controllerId,
          action: "registerBrushMesh",
          args: { surfaceId: node.surfaceId, windowId: windowInstanceId ?? node.surfaceId, ...args },
        }),
      ),
    [node.controllerId, node.surfaceId, onAction, windowInstanceId],
  );
  const drainBrushMeshQueue = useCallback(async () => {
    if (brushMeshDrainingRef.current) return;
    brushMeshDrainingRef.current = true;
    try {
      await drainPuzzle3dBrushMeshQueue(
        brushMeshQueueRef.current,
        {
          holdsDigest: (digest) => puzzle3dBrushMeshRegistry.holdsDigest(digest),
          mayAlias: (url) => puzzle3dBrushMeshRegistry.mayAlias(url),
          alias: (url, digest) => {
            brushMeshRunsRef.current.delete(url);
            puzzle3dBrushMeshRegistry.alias(url, digest);
          },
          confirm: (url, digest) => {
            brushMeshRunsRef.current.delete(url);
            puzzle3dBrushMeshRegistry.confirm(url, digest);
          },
        },
        dispatchBrushMesh,
        () => brushMeshLiveRef.current,
      );
    } finally {
      brushMeshDrainingRef.current = false;
    }
  }, [dispatchBrushMesh]);
  const handleRegisterBrushMesh = useCallback(
    (url: string, positions: number[], indices: number[]) => {
      const digest = puzzle3dBrushMeshDigest(positions, indices);
      if (puzzle3dBrushMeshRegistry.holds(url, digest)) {
        void dispatchBrushMesh({ url, digest });
        return;
      }
      if (puzzle3dBrushMeshRegistry.mayAlias(url) && puzzle3dBrushMeshRegistry.holdsDigest(digest)) {
        puzzle3dBrushMeshRegistry.alias(url, digest);
        void dispatchBrushMesh({ url, digest });
        return;
      }
      if (brushMeshRunsRef.current.get(url) === digest) return;
      const pages = puzzle3dBrushMeshPages(url, node.surfaceId, positions, indices);
      if (pages.length === 0 || brushMeshQueueRef.current.length + pages.length > PUZZLE3D_MESH_UPLOAD_QUEUE_PAGES) return;
      brushMeshRunsRef.current.set(url, digest);
      brushMeshQueueRef.current.push(...pages);
      void drainBrushMeshQueue();
    },
    [dispatchBrushMesh, drainBrushMeshQueue, node.surfaceId],
  );
  useEffect(
    () => () => {
      brushMeshLiveRef.current = false;
      for (const page of brushMeshQueueRef.current.splice(0)) {
        brushMeshRunsRef.current.delete(page.url);
        puzzle3dBrushMeshRegistry.forget(page.url);
      }
    },
    [],
  );
  // 🚚️ The guest's two published facts about what it actually holds, folded in before anything is
  // announced: a residency that fell proves a restarted guest (its mesh store is not part of any
  // checkpoint), which voids EVERY claim this page holds, and `meshReuploadUrls` names identities an
  // announcement already sent is waiting on bytes for. Either way the answer is the same — bump the
  // revision the mounted `BrushMeshRegistrar`s carry, so they re-announce their already-loaded GLB
  // and, with the claim gone, take the page path instead of the id-only one.
  const meshResidency = interaction.meshResidency;
  const meshReuploadUrls = interaction.meshReuploadUrls;
  useEffect(() => {
    if (meshResidency === undefined) return;
    const restarted = puzzle3dBrushMeshRegistry.observeResidency(meshResidency);
    const claimed = (meshReuploadUrls ?? []).filter((url) => puzzle3dBrushMeshRegistry.claimReupload(url));
    if (!restarted && claimed.length === 0) return;
    setBrushMeshRevisions((previous) => {
      const urls: Record<string, number> = { ...previous.urls };
      for (const url of claimed) urls[url] = (previous.urls[url] ?? 0) + 1;
      return { generation: previous.generation + (restarted ? 1 : 0), urls };
    });
  }, [meshResidency, meshReuploadUrls]);

  // 👻️ Include the live brush/suggestion ghost URL so collision precompute can register kinds that are
  // not yet placed in the scene — otherwise suggestions stay pending and never emit a 3D preview.
  const brushMeshUrls = useMemo(() => [...new Set([...meshes.map((mesh) => mesh.url).filter((url): url is string => Boolean(url)), ...(brushPreview?.meshUrl ? [brushPreview.meshUrl] : [])])], [brushPreview?.meshUrl, meshes]);

  const handleFrameVisibleInstances = useCallback(() => {
    const group = instancesGroupRef.current;
    if (group) {
      const box = new Box3().setFromObject(group);
      if (!box.isEmpty()) {
        const center = box.getCenter(new Vector3());
        const size = box.getSize(new Vector3());
        const radius = Math.max(size.x, size.y, size.z) * 0.5;
        adoptViewportCamera(world3dFrameCameraFromBounds([center.x, center.y, center.z], radius, cameraState), true);
        return;
      }
    }
    adoptViewportCamera(world3dFrameCameraFromInstances(instances, cameraState), true);
  }, [adoptViewportCamera, cameraState, instances]);

  const handleZoomToSelection = useCallback(() => {
    const selectedIds = new Set(selection.ids ?? []);
    if (selectedIds.size === 0) {
      handleFrameVisibleInstances();
      return;
    }
    const selected = instances.filter((instance) => selectedIds.has(instance.id));
    if (selected.length === 0) {
      handleFrameVisibleInstances();
      return;
    }
    let centerX = 0;
    let centerY = 0;
    let centerZ = 0;
    for (const instance of selected) {
      const position = instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0];
      centerX += position[0];
      centerY += position[1];
      centerZ += position[2];
    }
    const count = selected.length;
    centerX /= count;
    centerY /= count;
    centerZ /= count;
    let maxDistance = 1;
    for (const instance of selected) {
      const position = instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0];
      const dx = position[0] - centerX;
      const dy = position[1] - centerY;
      const dz = position[2] - centerZ;
      maxDistance = Math.max(maxDistance, Math.hypot(dx, dy, dz));
    }
    const distance = maxDistance * 3 + 2;
    adoptViewportCamera(
      {
        position: [centerX + distance * 0.6, centerY - distance * 0.6, centerZ + distance * 0.5],
        target: [centerX, centerY, centerZ],
        zoom: cameraState.zoom,
        up: cameraState.up,
        projection: cameraState.projection,
      },
      true,
    );
  }, [adoptViewportCamera, cameraState.projection, cameraState.up, cameraState.zoom, handleFrameVisibleInstances, instances, selection.ids]);

  const handleWorldMenuDispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      if (action === "zoomToSelection") {
        handleZoomToSelection();
        return;
      }
      if (action === "openVortexSuggestions") {
        setVortexPointerArm(null);
        setConnectDragSource(null);
        setConnectDragHoverPosition(null);
        dispatch(action, {
          windowId: windowInstanceId ?? undefined,
          x: contextMenu?.x ?? 0,
          y: contextMenu?.y ?? 0,
          ...args,
        });
        return;
      }
      dispatch(action, args);
    },
    [contextMenu, dispatch, handleZoomToSelection, windowInstanceId],
  );

  const mapWorldContextMenuSpecs = useMapContextMenuSpecs(handleWorldMenuDispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();
  const mapSuggestionContextMenuSpecs = useMapContextMenuSpecs((action, args) => {
    if (action === "acceptSuggestion") {
      setVortexPointerArm(null);
      setConnectDragSource(null);
      setConnectDragHoverPosition(null);
    }
    dispatch(action, args);
  });

  const hoveredVortexFullIdRef = useRef<string | null>(null);
  const altHeldRef = useRef(false);
  useEffect(() => {
    hoveredVortexFullIdRef.current = world3dRetainLocalVortexHover(hoveredVortexFullIdRef.current, interaction.hoveredVortexFullId);
  }, [interaction.hoveredVortexFullId]);
  /** 🕰️ Every per-gesture brush-preview tick travels ONE single-flight lane: at most one `suggestionsTick`
   * outstanding, and however many gestures ask while it is crossing, exactly one follow-up tick. A hover, its
   * reply and a refused brush place each used to enqueue their own tick straight into the serialized guest
   * queue, so one 70-move hover storm left 85 of them pending and the next user command (`addTargetVolume`)
   * waited behind the whole backlog past its 30 s budget (wave B33 §3). The sequence number is what makes
   * two successive requests distinct values for the coalescer — a constant would dedupe to the FIRST tick and
   * never ask again. Preview latency is unchanged: the next tick still leaves the moment the guest answers. */
  const suggestionsTickSeqRef = useRef(0);
  const sendSuggestionsTick = useMemo(() => createCoalescingActionDispatcher<number>(() => dispatchSettled("suggestionsTick")), [dispatchSettled]);
  const requestSuggestionsTick = useCallback(() => {
    suggestionsTickSeqRef.current += 1;
    sendSuggestionsTick(suggestionsTickSeqRef.current);
  }, [sendSuggestionsTick]);
  useEffect(() => {
    if (!brushMode || !interaction.hoveredVortexFullId) return;
    requestSuggestionsTick();
  }, [brushMode, interaction.hoveredVortexFullId, requestSuggestionsTick]);
  useEffect(() => {
    const onKey = (event: KeyboardEvent) => {
      if (event.key === "Alt" || event.key === "AltGraph") altHeldRef.current = event.type === "keydown";
    };
    window.addEventListener("keydown", onKey, true);
    window.addEventListener("keyup", onKey, true);
    window.addEventListener("blur", () => {
      altHeldRef.current = false;
    });
    return () => {
      window.removeEventListener("keydown", onKey, true);
      window.removeEventListener("keyup", onKey, true);
    };
  }, []);

  const handleWorldOrbitRightPointerDown = useCallback(
    (event: PointerEvent) => {
      const alt = world3dSuggestionsAltHeld(event.altKey, altHeldRef.current);
      const brushArmed = Boolean(brushMode && hoveredVortexFullIdRef.current);
      console.warn(`[DEBUG] suggestions-rightdown hop alt=${event.altKey} held=${altHeldRef.current} brush=${brushMode} hover=${hoveredVortexFullIdRef.current ?? "null"}`);
      if (world3dSuggestionsGestureArmed(alt, hoveredVortexFullIdRef.current) || brushArmed) {
        setVortexPointerArm(null);
        setConnectDragSource(null);
        setConnectDragHoverPosition(null);
        dispatch("openVortexSuggestions", { fullId: hoveredVortexFullIdRef.current, x: event.clientX, y: event.clientY, windowId: windowInstanceId ?? undefined });
        return false;
      }
      return true;
    },
    [brushMode, dispatch, windowInstanceId],
  );
  const suggestionsRightDownDispatchedAtRef = useRef(0);
  useEffect(() => {
    const route: World3dSuggestionsRightDownRoute = {
      host: () => hostRef.current,
      handle: (event) => {
        const allowOrbit = handleWorldOrbitRightPointerDown(event);
        if (allowOrbit === false) suggestionsRightDownDispatchedAtRef.current = performance.now();
        return allowOrbit;
      },
    };
    world3dSuggestionsRightDownRoutes.add(route);
    return () => {
      world3dSuggestionsRightDownRoutes.delete(route);
    };
  }, [handleWorldOrbitRightPointerDown]);

  const handleSuggestionClose = useCallback(() => {
    setVortexPointerArm(null);
    setConnectDragSource(null);
    setConnectDragHoverPosition(null);
    dispatch("closeVortexSuggestions");
  }, [dispatch]);

  // 🪟️ Sibling split panes do not mount the suggestion ContextMenuController, so they need their own
  // Escape / outside-dismiss path — otherwise `suggestionMenu.open` gates them with no way to clear it.
  useEffect(() => {
    if (!suggestionMenuOpen || suggestionMenuOwnsThisWindow) return undefined;
    const handlePointerDown = (event: globalThis.PointerEvent): void => {
      if (isContextMenuPointerTarget(event.target)) return;
      handleSuggestionClose();
    };
    const handleKeyDown = (event: globalThis.KeyboardEvent): void => {
      if (event.key === "Escape") handleSuggestionClose();
    };
    window.addEventListener("pointerdown", handlePointerDown);
    window.addEventListener("keydown", handleKeyDown);
    return () => {
      window.removeEventListener("pointerdown", handlePointerDown);
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [handleSuggestionClose, suggestionMenuOpen, suggestionMenuOwnsThisWindow]);

  // 🐢️ Background suggestion/fill planning ticks must not pile into the serialized plugin WASM queue —
  // a blind `setInterval` every 120ms while each tick+refresh still runs turns ~15s of idle fill into an
  // unbounded backlog that starves every other utility action (the fill utility appears to "die").
  //
  // 🏁️ `createInFlightSkippingInterval` gates on the promise `run` RETURNS, so the tick must return the
  // dispatch — a `run` that swallowed it cleared the in-flight flag on the same microtask and the gate
  // did nothing (measured 2026-09-09 20:55: 252 `fillBuildTick`s enqueued in 35 s, 38 rejected with
  // `serializePerActor: queue is full (>256 pending turns)`). `onAction` settles on the dispatched
  // action's own `OperationCompleted` frame (`ComponentSceneHostProps.onAction`), so exactly one tick is
  // ever outstanding and the cadence degrades to the guest's real turn time instead of overflowing.
  useEffect(() => {
    const menuPending = Boolean(interaction.suggestionMenu?.open && interaction.suggestionMenu.pending);
    if (!(menuPending || brushMode)) return;
    return createInFlightSkippingInterval(() => {
      if (interactivePluginActionInFlight()) return undefined;
      return dispatchSettled("suggestionsTick");
    }, 120);
  }, [brushMode, dispatchSettled, interaction.suggestionMenu?.open, interaction.suggestionMenu?.pending]);

  const leftoverArmedToolId = leftoverWorldSelectionOverlayV1()?.activeToolId;
  const fillBuildShouldTick = worldFillBuildShouldTick(activeUtility, interaction.fillBuild, leftoverArmedToolId);
  useEffect(() => {
    if (!fillBuildShouldTick) return;
    return createInFlightSkippingInterval(() => {
      if (interactivePluginActionInFlight()) return undefined;
      if (!worldFillBuildHostTickAllowed(true, isolatedJobDriveIsActive(), takeIsolatedJobUiPoll())) return undefined;
      return dispatchSettled("fillBuildTick");
    }, 120);
  }, [activeUtility, dispatchSettled, fillBuildShouldTick, interaction.fillBuild, leftoverArmedToolId]);

  const selectionArgs = useCallback(() => world3dGumballSelectionArgsV1(selection), [selection]);

  const handleInstancePointerDown = useCallback(
    (id: string, index: number, event: { shiftKey: boolean; ctrlKey: boolean; metaKey: boolean }) => {
      const merge = resolveWorldMergeMode(selection.selectionMergeMode, event, persistentSelectionMode);
      const record = instances.find((entry) => entry.id === id);
      if (record?.disabled) {
        dispatch("worldPick", { granularity: selectionMode, id: null, merge });
        return;
      }
      if (interactionDomainId) {
        dispatch("interactionSelect", world3dSelectionActionArgs(interactionDomainId, "object", [record?.interactionId ?? id], merge));
        return;
      }
      if (selectionMode === "mesh" || selectionMode === "object") {
        dispatch("worldPick", { granularity: "mesh", id: index, merge });
        return;
      }
      dispatch("worldSelect", {
        ids: [id],
        merge,
      });
    },
    [dispatch, instances, interactionDomainId, interactionGranularity, persistentSelectionMode, selection.selectionMergeMode, selectionMode],
  );

  // 🏁️ Both hover dispatchers RETURN their round trip: `createCoalescingActionDispatcher` keeps at most one
  // outstanding and coalesces the rest onto the latest target, and it can only do that for a `dispatch`
  // whose promise it can see — `dispatch` discards `onAction`'s, so the awaitable twin is the only shape
  // that arms the gate (wave B33 §3: 72 hover turns enqueued by one 70-move storm, 11 settled).
  const dispatchInstanceHover = useMemo(
    () =>
      createCoalescingActionDispatcher<string | null>((id) => {
        if (interactionDomainId) {
          if (id == null && leftoverBrushRetainGuestHoverV1(activeUtility, leftoverWorldWindowOverlayV1(windowInstanceId))) return undefined;
          const target = id == null ? null : (instancesRef.current.find((entry) => entry.id === id)?.interactionId ?? id);
          return dispatchSettled("interactionHover", world3dHoverActionArgs(interactionDomainId, interactionGranularity, target));
        }
        if (id == null) return dispatchSettled("setHover", {});
        return dispatchSettled("setHover", { objectId: id, mode: "mesh", id: 0 });
      }),
    [activeUtility, dispatchSettled, interactionDomainId, interactionGranularity, windowInstanceId],
  );

  const dispatchVortexHover = useMemo(
    () =>
      createCoalescingActionDispatcher<string | null>((fullId) => {
        if (interactionDomainId) {
          if (!fullId && leftoverBrushRetainGuestHoverV1(activeUtility, leftoverWorldWindowOverlayV1(windowInstanceId))) return undefined;
          const target = fullId ? world3dMarkerInteractionTarget("vortex", fullId, vorticesRef.current.find((entry) => entry.fullId === fullId)) : null;
          const args = world3dHoverActionArgs(interactionDomainId, target?.granularity ?? WORLD3D_DEFAULT_MARKER_GRANULARITY.vortex, target?.id);
          console.warn(`[DEBUG] interactionHover dispatch domain=${interactionDomainId} gran=${target?.granularity ?? WORLD3D_DEFAULT_MARKER_GRANULARITY.vortex} id=${target?.id ?? "null"}`);
          return dispatchSettled("interactionHover", args);
        }
        if (!fullId) return dispatchSettled("worldVortexHover", {});
        return dispatchSettled("worldVortexHover", { fullId });
      }),
    [activeUtility, dispatchSettled, interactionDomainId, windowInstanceId],
  );

  const handleInstancePointerMove = useCallback(
    (id: string | null) => {
      dispatchInstanceHover(id);
    },
    [dispatchInstanceHover],
  );

  const handleComponentHover = useCallback(
    (args: { objectId: string; mode: string; id: number } | null) => {
      if (!args) {
        dispatch("setHover", {});
        return;
      }
      dispatch("setHover", args);
    },
    [dispatch],
  );

  const handleVortexHover = useCallback(
    (fullId: string | null) => {
      if (fullId) hoveredVortexFullIdRef.current = fullId;
      console.warn(`[DEBUG] vortex-hover hop fullId=${fullId ?? "null"} keep=${hoveredVortexFullIdRef.current ?? "null"} brush=${brushMode}`);
      if (fullId) {
        dispatchVortexHover(fullId);
        if (brushMode) requestSuggestionsTick();
      }
    },
    [brushMode, dispatchVortexHover, requestSuggestionsTick],
  );

  const handleVortexSelect = useCallback(
    (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
      const merge = resolveWorldMergeMode(selection.selectionMergeMode, event ?? {}, persistentSelectionMode);
      if (interactionDomainId) {
        const target = world3dMarkerInteractionTarget("vortex", fullId, vortices.find((entry) => entry.fullId === fullId));
        dispatch("interactionSelect", world3dSelectionActionArgs(interactionDomainId, target.granularity, [target.id], merge));
        return;
      }
      dispatch("worldVortexSelect", { fullId, merge });
    },
    [dispatch, interactionDomainId, persistentSelectionMode, selection.selectionMergeMode, vortices],
  );

  const handleConnectDragStart = useCallback(
    (fullId: string, position: readonly [number, number, number]) => {
      setVortexPointerArm(null);
      const record = vortices.find((vortex) => vortex.fullId === fullId);
      setConnectDragSource({ fullId, position, record });
      setConnectDragHoverPosition(position);
    },
    [vortices],
  );

  const handleVortexPointerArm = useCallback(
    (arm: {
      readonly fullId: string;
      readonly position: readonly [number, number, number];
      readonly clientX: number;
      readonly clientY: number;
      readonly event: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean };
    }) => {
      setVortexPointerArm(arm);
    },
    [],
  );

  const handleVortexPointerMove = useCallback(
    (fullId: string, clientX: number, clientY: number) => {
      setVortexPointerArm((arm) => {
        if (!arm || arm.fullId !== fullId) return arm;
        const distance = Math.hypot(clientX - arm.clientX, clientY - arm.clientY);
        if (distance > MARQUEE_DRAG_THRESHOLD_PX) {
          handleConnectDragStart(arm.fullId, arm.position);
          return null;
        }
        return arm;
      });
    },
    [handleConnectDragStart],
  );

  const handleVortexPointerUp = useCallback(
    (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
      setVortexPointerArm((arm) => {
        if (arm && arm.fullId === fullId) {
          handleVortexSelect(fullId, arm.event ?? event);
        }
        return null;
      });
    },
    [handleVortexSelect],
  );

  const handleConnectDragHover = useCallback((position: readonly [number, number, number]) => {
    setConnectDragHoverPosition(position);
  }, []);

  const handleConnectDragDrop = useCallback(
    (targetFullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
      connectDropConsumedRef.current = true;
      setConnectDragSource((source) => {
        if (source) {
          if (source.fullId === targetFullId) {
            handleVortexSelect(targetFullId, event);
          } else {
            dispatch("createAttraction", { attracting: source.fullId, attracted: targetFullId });
          }
        }
        return null;
      });
      setConnectDragHoverPosition(null);
    },
    [dispatch, handleVortexSelect],
  );

  const handleConnectDragCancel = useCallback(() => {
    setVortexPointerArm(null);
    setConnectDragSource(null);
    setConnectDragHoverPosition(null);
  }, []);

  const handleVoxelPlace = useCallback(
    (origin: readonly [number, number, number]) => {
      dispatch("addTargetVolume", { origin });
    },
    [dispatch],
  );

  /** 🧊️ The Volume Brush's ground read, from the host's own pointer coordinates — one raycast shared by
   * the hover ghost and the Alt+click commit, so the box the user sees is exactly the volume that lands. */
  const voxelGroundOriginAt = useCallback(
    (clientX: number, clientY: number): [number, number, number] | null => {
      const host = hostRef.current;
      const camera = cameraRef.current;
      if (!host || !camera) return null;
      return world3dVolumeBrushOriginV1(raycastGroundPoint(clientX, clientY, host.getBoundingClientRect(), camera), gridFactor);
    },
    [gridFactor],
  );

  useEffect(() => {
    if (!volumeBrushMode) setVoxelHoverOrigin(null);
  }, [volumeBrushMode]);

  const pendingBrushPlaceRef = useRef(false);
  const handleBrushPlace = useCallback(() => {
    const args = brushObjectPlacementArgs(brushPreview);
    console.warn(`[DEBUG] brush-place hop preview=${args ? args.targetVortexFullId : "null"} utility=${interaction.activeUtility ?? "none"} hover=${hoveredVortexFullIdRef.current ?? "null"}`);
    if (!args) {
      pendingBrushPlaceRef.current = true;
      const keep = hoveredVortexFullIdRef.current;
      if (keep) dispatchVortexHover(keep);
      if (brushMode) requestSuggestionsTick();
      return;
    }
    pendingBrushPlaceRef.current = false;
    dispatch("addBrushObject", args);
  }, [brushMode, brushPreview, dispatch, dispatchVortexHover, interaction.activeUtility, requestSuggestionsTick]);
  useEffect(() => {
    if (!pendingBrushPlaceRef.current) return;
    const args = brushObjectPlacementArgs(brushPreview);
    if (!args) return;
    pendingBrushPlaceRef.current = false;
    console.warn(`[DEBUG] brush-place deferred addBrushObject ${args.targetVortexFullId}`);
    dispatch("addBrushObject", args);
  }, [brushPreview, dispatch]);

  const handleWorldPick = useCallback(
    (args: { granularity: string; id: number; merge: string; objectId?: string }) => {
      dispatch("worldPick", args);
    },
    [dispatch],
  );

  const paintMode = selection.interactionMode === "paint";
  const handlePaintAt = useCallback(
    (objectId: string, u: number, v: number) => {
      dispatch("paintAt", { objectId, u, v });
    },
    [dispatch],
  );

  // 🧭️ Completed user orbit/pan/zoom gesture (`WorldOrbitGated.onCamera`, fired on gesture end) — the only
  // camera-change handler wired to a real user interaction, so it's the only one that also dispatches.
  const handleCameraChange = useCallback(
    (state: WorldCameraState) => {
      adoptViewportCamera(state, false);
      dispatchWorldCameraDebounced(state);
    },
    [adoptViewportCamera, dispatchWorldCameraDebounced],
  );

  // 🧭️ Programmatic auto-fit-to-content camera change (`WorldAutoFit.onFitted`, runs after a document/scene
  // loads or content changes) — deliberately split from `handleCameraChange` so this path never dispatches;
  // only a genuine user gesture should sync a camera to the plugin.
  const handleAutoFitCameraChange = useCallback(
    (state: WorldCameraState) => {
      adoptViewportCamera(state, false);
    },
    [adoptViewportCamera],
  );

  const syncProjectionWindowChrome = useCallback(
    (spec: WorldProjectionSpec) => {
      if (!windowInstanceId) return;
      setWindowTitle?.(windowInstanceId, worldProjectionSpecLabel(spec));
      setWindowIcon?.(windowInstanceId, worldProjectionSpecIconId(spec) as IconName);
    },
    [setWindowIcon, setWindowTitle, windowInstanceId],
  );

  useEffect(() => {
    const pending = peekPendingWorldProjection(windowInstanceId);
    if (pending) syncProjectionWindowChrome(pending);
  }, [syncProjectionWindowChrome, windowInstanceId]);

  // 🧭️ Gizmo/axis-indicator camera snap — a discrete view change the user clicked, so (like
  // `handleCameraChange`) it also dispatches, debounced identically.
  const handleGizmoCameraChange = useCallback(
    (state: WorldCameraState) => {
      adoptViewportCamera(state, true);
      if (state.projectionSpec) syncProjectionWindowChrome(state.projectionSpec);
      dispatchWorldCameraDebounced(state);
    },
    [adoptViewportCamera, syncProjectionWindowChrome, dispatchWorldCameraDebounced],
  );

  const [externalPendingProjectionSpec, setExternalPendingProjectionSpec] = useState<WorldProjectionSpec | null>(null);
  const [pendingProjectionSpec, setPendingProjectionSpec] = useState<WorldProjectionSpec | null>(null);

  // 🧭️ User-driven projection-kind switch (`WorldOrbitProjectionSwitchPane`) — view-state only, never
  // dispatched: the only existing `setProjection` consumer (`apply_world3d_projection_action` in the
  // framework plugin's Rust) takes granular `{field, value}` pairs from window-measure Select/Slider
  // controls (e.g. `perspectiveKind`/`orthographicView`/`axonometricVariant`), a fundamentally different
  // contract than this pane's whole-`WorldProjectionSpec` (mode ⊗ orientation) template selection — there is
  // no lossless mapping from one to the other, so dispatching here would silently no-op against every real
  // handler instead of doing anything.
  const handleProjectionKindChange = useCallback(
    (spec: WorldProjectionSpec) => {
      setExternalPendingProjectionSpec(spec);
      syncProjectionWindowChrome(spec);
    },
    [syncProjectionWindowChrome],
  );

  const handleProjectionContentFrame = useCallback((state: WorldParsedCameraState) => {
    setViewportCamera(state);
    if (!projectionContentFrameSeededRef.current) {
      projectionContentFrameSeededRef.current = true;
      setDetachEpoch((epoch) => epoch + 1);
    }
    setProjectionFramePending(false);
  }, []);

  const worldProjectionSpec: WorldProjectionSpec = cameraState.projectionSpec ?? (cameraState.projection === "orthographic" ? worldProjectionDefaults("orthographic") : worldProjectionDefaults("threePoint"));
  /** 📷️ Keep fitting the live content bounds into seeded projection panes (esp. orthographic Top) until the
   * user takes ownership — otherwise fill-planned objects that expand the scene fall outside the one-shot
   * initial frustum and only remain visible in the wider perspective pane. */
  const fitProjectionContent = !viewportOwned && Boolean(pendingProjectionSpecRef.current ?? cameraState.projectionSpec);
  const worldOrbitConstraints = useMemo(() => worldProjectionOrbitConstraints(cameraState.projectionSpec), [cameraState.projectionSpec]);

  const marqueePreview = useMemo<{ readonly mergedComponentIds: readonly number[] | null; readonly mergedInstanceIds: readonly string[] | null }>(() => {
    if (!marqueeDragActive || !hostRef.current || !cameraRef.current) return { mergedComponentIds: null, mergedInstanceIds: null };
    const rect = hostRef.current.getBoundingClientRect();
    const camera = cameraRef.current;
    if (selectionMode === "mesh" || selectionMode === "object") {
      const hits = resolveMarqueeInstanceIds(instances, meshes, marqueePath, rect, camera, method, marqueeCoverage);
      return { mergedComponentIds: null, mergedInstanceIds: mergeIdSet(marqueeMergeMode, selection.ids ?? [], hits) };
    }
    const hits = resolveMarqueeComponentIds(instances, meshes, selectionMode, selection.activeObjectId, marqueePath, rect, camera, method, marqueeCoverage);
    return { mergedComponentIds: mergeIdSet(marqueeMergeMode, selection.componentIds ?? [], hits), mergedInstanceIds: null };
  }, [instances, marqueeCoverage, marqueeDragActive, marqueeMergeMode, marqueePath, meshes, method, selection.activeObjectId, selection.componentIds, selection.ids, selectionMode]);
  const selectionPreviewSourceId = windowInstanceId ?? node.surfaceId;
  const localSelectionPreviewActive = marqueePreview.mergedComponentIds !== null || marqueePreview.mergedInstanceIds !== null;
  const heldSelectionPreviewActive = marqueeCommitHold !== null && (marqueeCommitHold.mergedComponentIds !== null || marqueeCommitHold.mergedInstanceIds !== null);
  useEffect(() => {
    if (!marqueeCommitHold) return;
    const selectionIdSet = new Set(selection.ids ?? []);
    const selectionComponentSet = new Set(selection.componentIds ?? []);
    const instanceMatch =
      marqueeCommitHold.mergedInstanceIds != null &&
      marqueeCommitHold.mergedInstanceIds.length > 0 &&
      marqueeCommitHold.mergedInstanceIds.length === selectionIdSet.size &&
      marqueeCommitHold.mergedInstanceIds.every((id) => selectionIdSet.has(id));
    const componentMatch =
      marqueeCommitHold.mergedComponentIds != null &&
      marqueeCommitHold.mergedComponentIds.length > 0 &&
      marqueeCommitHold.mergedComponentIds.length === selectionComponentSet.size &&
      marqueeCommitHold.mergedComponentIds.every((id) => selectionComponentSet.has(id));
    if (instanceMatch || componentMatch) setMarqueeCommitHold(null);
  }, [marqueeCommitHold, selection.componentIds, selection.ids]);
  useLayoutEffect(() => {
    if (localSelectionPreviewActive) {
      setWorldSelectionPreview(node.controllerId, { sourceId: selectionPreviewSourceId, ...marqueePreview });
    } else if (heldSelectionPreviewActive && marqueeCommitHold) {
      setWorldSelectionPreview(node.controllerId, { sourceId: selectionPreviewSourceId, ...marqueeCommitHold });
    } else {
      clearWorldSelectionPreview(node.controllerId, selectionPreviewSourceId);
    }
  }, [heldSelectionPreviewActive, localSelectionPreviewActive, marqueeCommitHold, marqueePreview, node.controllerId, selectionPreviewSourceId]);
  const marqueePreviewRef = useRef(marqueePreview);
  marqueePreviewRef.current = marqueePreview;
  const marqueeDragActiveRef = useRef(marqueeDragActive);
  marqueeDragActiveRef.current = marqueeDragActive;
  const marqueeFinalizeOnceRef = useRef(false);
  useEffect(
    () => () => {
      clearWorldSelectionPreview(node.controllerId, selectionPreviewSourceId);
    },
    [node.controllerId, selectionPreviewSourceId],
  );
  const visibleSelectionPreview = localSelectionPreviewActive
    ? marqueePreview
    : heldSelectionPreviewActive && marqueeCommitHold
      ? marqueeCommitHold
      : (sharedSelectionPreview ?? marqueePreview);

  const dispatchGumballPoseDelta = useCallback(
    (kind: GumballHandleKind, before: GumballPose, after: GumballPose) => {
      const payload = gumballTransformDeltaBetweenPoses(selection.transformMode, before, after, selectionArgs(), kind);
      if (!payload) {
        // 🧯️ A drag whose pose did not move commits NOTHING. It used to synthesize a fixed 0.5 translate
        // along the handle's axis instead — a document edit the user never made, minted precisely when the
        // gesture failed to say anything (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B31). The record below
        // is PERMANENT, not a `[DEBUG]` trace: it is what a zero-delta drag owes, so the cause is read off
        // the gesture rather than covered by a fabricated move.
        console.info("gumball pose delta skipped", {
          transformMode: selection.transformMode,
          kind,
          dx: after.position[0] - before.position[0],
          dy: after.position[1] - before.position[1],
          dz: after.position[2] - before.position[2],
          before: [...before.position],
          after: [...after.position],
          args: selectionArgs(),
        });
        return Promise.resolve();
      }
      console.info("[DEBUG] gumball pose delta", { action: payload.action, ids: payload.args.ids, mode: payload.args.mode });
      return Promise.resolve(dispatch(payload.action, payload.args));
    },
    [dispatch, selection.transformMode, selectionArgs],
  );

  const enqueueGumballDispatch = useCallback((task: () => void | Promise<void>) => {
    gumballDragChainRef.current = gumballDragChainRef.current.then(task).catch(() => undefined);
    return gumballDragChainRef.current;
  }, []);

  const handleGumballDragStart = useCallback(
    (_kind: GumballHandleKind, before: GumballPose) => {
      (globalThis as { __gumballDragEntered?: boolean }).__gumballDragEntered = true;
      console.info("[DEBUG] gumball drag entered", { kind: _kind, ids: selectionArgs().ids });
      gumballDragStartPoseRef.current = before;
      void enqueueGumballDispatch(() => Promise.resolve(dispatch("transformBegin")));
    },
    [dispatch, enqueueGumballDispatch, selectionArgs],
  );

  const handleGumballDrag = useCallback((_kind: GumballHandleKind, _pose: GumballPose) => {
    // ⚡️ Mid-drag stays local (WorldInstancesLayer imperative preview) — no WASM/React composite rebuild.
  }, []);

  const handleGumballDragEnd = useCallback(
    (kind: GumballHandleKind, before: GumballPose, after: GumballPose) => {
      const startPose = gumballDragStartPoseRef.current ?? before;
      gumballDragStartPoseRef.current = null;
      void enqueueGumballDispatch(async () => {
        // One absolute start→end delta — the app commits it directly; `transformEnd` only closes the host bracket.
        await dispatchGumballPoseDelta(kind, startPose, after);
        await Promise.resolve(dispatch("transformEnd"));
      });
    },
    [dispatch, dispatchGumballPoseDelta, enqueueGumballDispatch],
  );

  const handleFaceDragStart = useCallback((args: { objectId: string; faceId: number; normal: readonly [number, number, number]; point: readonly [number, number, number]; faceExtent?: readonly [number, number] }) => {
    setFaceDragSession({ objectId: args.objectId, faceId: args.faceId, normal: args.normal, startPoint: args.point, faceExtent: args.faceExtent });
  }, []);

  const toLocalPoint = useCallback((event: React.PointerEvent<HTMLDivElement>): SelectionMarqueePoint => {
    const rect = hostRef.current?.getBoundingClientRect();
    if (!rect) return { x: event.clientX, y: event.clientY };
    return { x: event.clientX - rect.left, y: event.clientY - rect.top };
  }, []);

  /** 🚚️ Takes a Relocate-utility press: grabs the object under it (gated by the selection, see
   * {@link world3dRelocateDragTargetV1}), pins the ground point the drag starts from and paints the shared
   * world ghost at the object's own origin. Answers whether it took the press, so the marquee never opens
   * underneath an in-progress relocate. A LOCKED object is grabbed on purpose — the guest answers the
   * commit with its `selection_locked` notice, which is a visible refusal instead of a dead gesture.
   * A press on empty ground with a live selection is the gesture's BASE POINT, not a miss — see
   * {@link world3dRelocateDragTargetV1} for why the base point never had to be on the object. */
  const beginRelocateDrag = useCallback(
    (event: React.PointerEvent<HTMLDivElement>): boolean => {
      const host = hostRef.current;
      const camera = cameraRef.current;
      if (!host || !camera) return false;
      const rect = host.getBoundingClientRect();
      const from = raycastGroundPoint(event.clientX, event.clientY, rect, camera);
      if (!from) return false;
      const pressed = resolveClickInstanceId(instancesRef.current, meshesRef.current, toLocalPoint(event), rect, camera);
      const objectId = world3dRelocateDragTargetV1(pressed, selectionArgs().ids);
      const instance = objectId ? instancesRef.current.find((entry) => entry.id === objectId) : undefined;
      if (!objectId || !instance) return false;
      const origin = instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0];
      const objectKind = instance.objectKind ?? "";
      const meshUrl = meshesRef.current.find((mesh) => mesh.id === (instance.meshId ?? instance.id))?.url;
      relocateSessionRef.current = { objectId, origin, from, objectKind, meshUrl };
      event.currentTarget.setPointerCapture?.(event.pointerId);
      setWorldCatalogueDropPreview(node.controllerId, { objectKind, meshUrl, origin });
      return true;
    },
    [node.controllerId, selectionArgs, toLocalPoint],
  );

  /** 🚚️ Moves the live relocate ghost to the grabbed object's would-be origin — grid-snapped exactly like a
   * catalogue drop, zero dispatches until release. */
  const updateRelocateDrag = useCallback(
    (event: React.PointerEvent<HTMLDivElement>): boolean => {
      const session = relocateSessionRef.current;
      const host = hostRef.current;
      const camera = cameraRef.current;
      if (!session || !host || !camera) return false;
      const to = raycastGroundPoint(event.clientX, event.clientY, host.getBoundingClientRect(), camera);
      if (!to) return true;
      const args = world3dRelocateDispatchArgsV1(session.objectId, session.origin, session.from, to, { gridSnapEnabled, gridFactor });
      setWorldCatalogueDropPreview(node.controllerId, { objectKind: session.objectKind, meshUrl: session.meshUrl, origin: args?.position ?? session.origin });
      return true;
    },
    [gridFactor, gridSnapEnabled, node.controllerId],
  );

  /** 🚚️ Closes the relocate drag. A pointer-UP event commits ONE absolute `worldRelocate`; `null` (Escape) and
   * a `pointercancel` both drop the ghost and dispatch nothing — the host binds `onPointerCancel` to the same
   * pointer-up handler, so the event kind is the only thing separating a commit from an abort. */
  const endRelocateDrag = useCallback(
    (event: React.PointerEvent<HTMLDivElement> | null): boolean => {
      const session = relocateSessionRef.current;
      if (!session) return false;
      relocateSessionRef.current = null;
      clearWorldCatalogueDropPreview(node.controllerId);
      const host = hostRef.current;
      const camera = cameraRef.current;
      if (!event || event.type === "pointercancel" || !host || !camera) return true;
      const to = raycastGroundPoint(event.clientX, event.clientY, host.getBoundingClientRect(), camera);
      const args = to ? world3dRelocateDispatchArgsV1(session.objectId, session.origin, session.from, to, { gridSnapEnabled, gridFactor }) : null;
      if (args) void Promise.resolve(dispatch("worldRelocate", args));
      return true;
    },
    [dispatch, gridFactor, gridSnapEnabled, node.controllerId],
  );

  useEffect(() => {
    if (!relocateMode) return undefined;
    const onKeyDown = (event: KeyboardEvent): void => {
      if (event.key !== "Escape") return;
      if (endRelocateDrag(null)) event.stopPropagation();
    };
    window.addEventListener("keydown", onKeyDown, true);
    return () => window.removeEventListener("keydown", onKeyDown, true);
  }, [endRelocateDrag, relocateMode]);

  const handlePointerDown = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (event.button !== 0) return;
      if (relocateMode && beginRelocateDrag(event)) return;
      if (world3dVolumeBrushCommits(volumeBrushMode, event.altKey)) {
        const origin = voxelGroundOriginAt(event.clientX, event.clientY);
        if (origin) {
          setVoxelHoverOrigin(origin);
          handleVoxelPlace(origin);
          return;
        }
      }
      if (selection.engagementSessionActive && hostRef.current && cameraRef.current) {
        const rect = hostRef.current.getBoundingClientRect();
        const point = raycastGroundPoint(event.clientX, event.clientY, rect, cameraRef.current);
        if (point) {
          dispatch("worldPointerDown", {
            pane: paneSuffixFromSurfaceId(node.surfaceId),
            position: point,
            shiftKey: event.shiftKey,
            ctrlKey: event.ctrlKey,
            metaKey: event.metaKey,
          });
          return;
        }
      }
      if (paintMode) {
        setPaintStrokeActive(true);
        dispatch("paintStrokeBegin");
      }
      setMarqueeModifiers({ shiftKey: event.shiftKey, ctrlKey: event.ctrlKey, metaKey: event.metaKey });
      marqueeFinalizeOnceRef.current = false;
      const start = toLocalPoint(event);
      marqueeStartRef.current = start;
      setMarqueePath([start]);
    },
    [beginRelocateDrag, dispatch, handleVoxelPlace, node.surfaceId, paintMode, relocateMode, selection.engagementSessionActive, toLocalPoint, volumeBrushMode, voxelGroundOriginAt],
  );

  const handlePointerMove = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (updateRelocateDrag(event)) return;
      if (volumeBrushMode) setVoxelHoverOrigin(voxelGroundOriginAt(event.clientX, event.clientY));
      if (selection.engagementSessionActive && hostRef.current && cameraRef.current) {
        const rect = hostRef.current.getBoundingClientRect();
        const point = raycastGroundPoint(event.clientX, event.clientY, rect, cameraRef.current);
        const last = engagementPointerMoveLastPointRef.current;
        const unchanged = point && last && point[0] === last[0] && point[1] === last[1] && point[2] === last[2];
        if (point && !unchanged && !engagementPointerMoveInFlightRef.current) {
          engagementPointerMoveInFlightRef.current = true;
          engagementPointerMoveLastPointRef.current = point;
          requestAnimationFrame(() => {
            engagementPointerMoveInFlightRef.current = false;
            dispatch("worldPointerMove", { pane: paneSuffixFromSurfaceId(node.surfaceId), position: point });
          });
        }
        return;
      }
      if (!marqueeDown) return;
      const local = toLocalPoint(event);
      const start = marqueeStartRef.current;
      if (start && world3dMarqueePointerCaptureArmed(Math.hypot(local.x - start.x, local.y - start.y))) {
        if (!event.currentTarget.hasPointerCapture?.(event.pointerId)) {
          event.currentTarget.setPointerCapture?.(event.pointerId);
        }
      }
      setMarqueeModifiers({ shiftKey: event.shiftKey, ctrlKey: event.ctrlKey, metaKey: event.metaKey });
      setMarqueePath((path) => [...path, local]);
    },
    [dispatch, marqueeDown, node.surfaceId, selection.engagementSessionActive, toLocalPoint, updateRelocateDrag, volumeBrushMode, voxelGroundOriginAt],
  );

  const finalizeMarqueeSelection = useCallback(() => {
    if (!marqueeDragActiveRef.current || marqueeFinalizeOnceRef.current) return;
    const preview = marqueePreviewRef.current;
    if (!preview.mergedInstanceIds?.length && !preview.mergedComponentIds?.length) return;
    marqueeFinalizeOnceRef.current = true;
    if (preview.mergedInstanceIds?.length) {
      setMarqueeCommitHold({ mergedComponentIds: null, mergedInstanceIds: preview.mergedInstanceIds });
      const domainTargets = interactionTargetsForInstances(instancesRef.current, preview.mergedInstanceIds);
      const marqueeSelect = interactionDomainId
        ? dispatch("interactionSelect", world3dSelectionActionArgs(interactionDomainId, "object", domainTargets, "replace"))
        : dispatch("worldSelect", { ids: preview.mergedInstanceIds, merge: "replace" });
      void Promise.resolve(marqueeSelect).finally(() => {
        window.setTimeout(() => setMarqueeCommitHold((hold) => (hold?.mergedInstanceIds === preview.mergedInstanceIds ? null : hold)), 250);
      });
    } else if (preview.mergedComponentIds?.length) {
      setMarqueeCommitHold({ mergedComponentIds: preview.mergedComponentIds, mergedInstanceIds: null });
      void Promise.resolve(
        dispatch("setSelection", {
          mode: selectionMode,
          ids: preview.mergedComponentIds,
          objectId: selection.activeObjectId,
          merge: "replace",
        }),
      ).finally(() => {
        window.setTimeout(() => setMarqueeCommitHold((hold) => (hold?.mergedComponentIds === preview.mergedComponentIds ? null : hold)), 250);
      });
    }
    wasMarqueeDragRef.current = true;
    setMarqueePath([]);
  }, [dispatch, interactionDomainId, interactionGranularity, selection.activeObjectId, selectionMode]);

  const handlePointerUp = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (event.currentTarget.hasPointerCapture?.(event.pointerId)) {
        event.currentTarget.releasePointerCapture(event.pointerId);
      }
      if (endRelocateDrag(event)) return;
      if (faceDragSession) {
        const session = faceDragSession;
        setFaceDragSession(null);
        if (hostRef.current && cameraRef.current) {
          const rect = hostRef.current.getBoundingClientRect();
          const distance = axisDragParam(event.clientX, event.clientY, rect, cameraRef.current, session.startPoint, session.normal);
          if (distance != null && Math.abs(distance) > 1e-4) {
            dispatch("worldFaceDragEnd", {
              pane: paneSuffixFromSurfaceId(node.surfaceId),
              objectId: session.objectId,
              faceId: session.faceId,
              normal: session.normal,
              startPoint: session.startPoint,
              distance,
              faceExtent: session.faceExtent,
            });
          }
        }
        return;
      }
      finalizeMarqueeSelection();
      if (!marqueeDragActiveRef.current && !paintMode && !selection.engagementSessionActive && !worldInstancePickBlocked(activeUtility)) {
        const host = hostRef.current;
        const camera = cameraRef.current;
        if (host && camera) {
          const rect = host.getBoundingClientRect();
          const local = toLocalPoint(event);
          const id = resolveClickInstanceId(instancesRef.current, meshesRef.current, local, rect, camera);
          if (id) {
            const index = instancesRef.current.findIndex((entry) => entry.id === id);
            handleInstancePointerDown(id, index < 0 ? 0 : index, event);
          }
        }
      }
      if (paintStrokeActive) {
        dispatch("paintStrokeEnd");
        setPaintStrokeActive(false);
      }
      setMarqueePath([]);
      marqueeStartRef.current = null;
      setVortexPointerArm(null);
      if (connectDropConsumedRef.current) {
        connectDropConsumedRef.current = false;
      } else {
        handleConnectDragCancel();
      }
    },
    [activeUtility, dispatch, endRelocateDrag, faceDragSession, finalizeMarqueeSelection, handleConnectDragCancel, handleInstancePointerDown, interactionDomainId, interactionGranularity, node.surfaceId, paintMode, paintStrokeActive, persistentSelectionMode, selection.engagementSessionActive, selection.selectionMergeMode, selectionMode, toLocalPoint],
  );

  useEffect(() => {
    if (!marqueeDown || selection.engagementSessionActive || paintMode) return undefined;
    const onWindowPointerUp = (event: PointerEvent): void => {
      if (event.button !== 0) return;
      finalizeMarqueeSelection();
    };
    window.addEventListener("pointerup", onWindowPointerUp, true);
    window.addEventListener("pointercancel", onWindowPointerUp, true);
    return () => {
      window.removeEventListener("pointerup", onWindowPointerUp, true);
      window.removeEventListener("pointercancel", onWindowPointerUp, true);
    };
  }, [finalizeMarqueeSelection, marqueeDown, paintMode, selection.engagementSessionActive]);

  const handleEmptyClick = useCallback(
    (event: globalThis.MouseEvent) => {
      // 🧹️ Consume the post-marquee suppress flag so a stale `true` cannot permanently no-operation background deselect.
      if (wasMarqueeDragRef.current) {
        wasMarqueeDragRef.current = false;
        return;
      }
      if (selection.engagementSessionActive || paintMode) return;
      if (interaction.suggestionMenu?.open) {
        handleSuggestionClose();
      }
      const merge = resolveWorldMergeMode(selection.selectionMergeMode, event, persistentSelectionMode);
      if (interactionDomainId) {
        dispatch("interactionSelect", world3dSelectionActionArgs(interactionDomainId, interactionGranularity, [], merge));
        return;
      }
      dispatch("worldPick", { granularity: selectionMode, id: null, merge });
    },
    [dispatch, handleSuggestionClose, interaction.suggestionMenu?.open, interactionDomainId, interactionGranularity, paintMode, persistentSelectionMode, selection.engagementSessionActive, selection.selectionMergeMode, selectionMode],
  );

  const clearCatalogueDrop = useCallback(() => {
    catalogueDragEncodedRef.current = null;
    catalogueDragDepthRef.current = 0;
    clearWorldCatalogueDropPreview(node.controllerId);
  }, [node.controllerId]);

  const readCatalogueDragEncoded = useCallback((): string | null => {
    return getActiveCatalogueDragPayload() ?? catalogueDragEncodedRef.current;
  }, []);

  const updateCatalogueDropPreviewAt = useCallback(
    (clientX: number, clientY: number) => {
      const encoded = readCatalogueDragEncoded();
      const payload = parsePuzzle3dCatalogueDragPayload(encoded);
      if (!payload || !hostRef.current || !cameraRef.current) return;
      const rect = hostRef.current.getBoundingClientRect();
      if (!clientPointOverHost(clientX, clientY, rect)) {
        if (!worldCatalogueDropHostContainsPoint(node.controllerId, clientX, clientY)) clearWorldCatalogueDropPreview(node.controllerId);
        return;
      }
      const origin = resolveCatalogueDropOrigin(clientX, clientY, rect, cameraRef.current, gridSnapEnabled, gridFactor);
      if (!origin) return;
      if (encoded) catalogueDragEncodedRef.current = encoded;
      setWorldCatalogueDropPreview(node.controllerId, { ...payload, origin });
    },
    [gridFactor, gridSnapEnabled, node.controllerId, readCatalogueDragEncoded],
  );

  const commitCatalogueDropAt = useCallback(
    (clientX: number, clientY: number, encoded?: string | null) => {
      const payload = parsePuzzle3dCatalogueDragPayload(encoded ?? readCatalogueDragEncoded());
      if (!payload || !hostRef.current || !cameraRef.current) return;
      const rect = hostRef.current.getBoundingClientRect();
      const origin = resolveCatalogueDropOrigin(clientX, clientY, rect, cameraRef.current, gridSnapEnabled, gridFactor);
      if (!origin) return;
      // 🎉️ Stamp the pre-drop instance set so the next scene update can celebrate the newly placed object(s).
      pendingCelebrateCatalogueDropIdsRef.current = new Set(instancesRef.current.map((instance) => instance.id));
      dispatch("addObjectKind", { objectKind: payload.objectKind, origin });
    },
    [dispatch, gridFactor, gridSnapEnabled, readCatalogueDragEncoded],
  );

  useEffect(() => {
    const baseline = pendingCelebrateCatalogueDropIdsRef.current;
    if (!baseline) return;
    const added = instances.filter((instance) => !baseline.has(instance.id)).map((instance) => instance.id);
    if (added.length === 0) return;
    pendingCelebrateCatalogueDropIdsRef.current = null;
    celebrateWorldInstances(added);
  }, [instances]);

  const onCatalogueDragEnter = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!scene) return;
      if (!event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME) && !getActiveCatalogueDragPayload()) return;
      event.preventDefault();
      catalogueDragDepthRef.current += 1;
    },
    [scene],
  );

  const onCatalogueDragLeave = useCallback(
    (_event: DragEvent<HTMLDivElement>) => {
      if (!scene) return;
      catalogueDragDepthRef.current = Math.max(0, catalogueDragDepthRef.current - 1);
    },
    [scene],
  );

  const onCatalogueDragOver = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!scene) return;
      if (!event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME) && !getActiveCatalogueDragPayload()) return;
      const encoded = getActiveCatalogueDragPayload();
      if (!parsePuzzle3dCatalogueDragPayload(encoded) && !event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME)) return;
      event.preventDefault();
      event.dataTransfer.dropEffect = "copy";
      if (encoded) catalogueDragEncodedRef.current = encoded;
      updateCatalogueDropPreviewAt(event.clientX, event.clientY);
    },
    [scene, updateCatalogueDropPreviewAt],
  );

  const onCatalogueDrop = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!scene) return;
      event.preventDefault();
      const encoded = event.dataTransfer.getData(CATALOGUE_DRAG_MIME) || getActiveCatalogueDragPayload() || catalogueDragEncodedRef.current;
      commitCatalogueDropAt(event.clientX, event.clientY, encoded);
      clearCatalogueDrop();
    },
    [clearCatalogueDrop, commitCatalogueDropAt, scene],
  );

  useEffect(() => {
    const hostId = windowInstanceId ?? node.surfaceId;
    return registerWorldCatalogueDropHost(node.controllerId, hostId, (clientX, clientY) => {
      const host = hostRef.current;
      if (!host) return false;
      return clientPointOverHost(clientX, clientY, host.getBoundingClientRect());
    });
  }, [node.controllerId, node.surfaceId, windowInstanceId]);

  useEffect(() => {
    const onPointerMove = (event: PointerEvent) => {
      const encoded = getActiveCatalogueDragPayload();
      if (!encoded) {
        if (getWorldCatalogueDropPreview(node.controllerId)) clearWorldCatalogueDropPreview(node.controllerId);
        return;
      }
      if (!parsePuzzle3dCatalogueDragPayload(encoded)) return;
      catalogueDragEncodedRef.current = encoded;
      updateCatalogueDropPreviewAt(event.clientX, event.clientY);
    };

    const onPointerUp = (event: PointerEvent) => {
      const encoded = getActiveCatalogueDragPayload() ?? catalogueDragEncodedRef.current;
      const payload = parsePuzzle3dCatalogueDragPayload(encoded);
      if (!payload) return;
      const host = hostRef.current;
      if (!host) return;
      const rect = host.getBoundingClientRect();
      if (clientPointOverHost(event.clientX, event.clientY, rect)) {
        commitCatalogueDropAt(event.clientX, event.clientY, encoded);
        clearCatalogueDrop();
        return;
      }
      if (!worldCatalogueDropHostContainsPoint(node.controllerId, event.clientX, event.clientY)) clearCatalogueDrop();
    };

    const onDragEnd = () => {
      queueMicrotask(() => {
        if (!getActiveCatalogueDragPayload()) clearCatalogueDrop();
      });
    };

    window.addEventListener("pointermove", onPointerMove);
    window.addEventListener("pointerup", onPointerUp, true);
    window.addEventListener("dragend", onDragEnd);
    return () => {
      window.removeEventListener("pointermove", onPointerMove);
      window.removeEventListener("pointerup", onPointerUp, true);
      window.removeEventListener("dragend", onDragEnd);
    };
  }, [clearCatalogueDrop, commitCatalogueDropAt, node.controllerId, updateCatalogueDropPreviewAt]);

  if (!scene) return <div className="semio-world-3d-empty">{emptySceneLabel}</div>;

  return (
    <div
      ref={hostRef}
      className="semio-world-3d-host relative h-full min-h-0 w-full overflow-hidden"
      data-surface-id={node.surfaceId}
      data-window-instance-id={windowInstanceId || undefined}
      data-selection-json={JSON.stringify(worldSurfaceSelectionDomV1(selection, interaction))}
      data-guest-selection-json={JSON.stringify(worldSurfaceGuestSelectionDomV1(scene.selectionJson))}
      data-orbit-view-gizmo=""
      data-puzzle3d-fixture-drag-active={catalogueDropPreview ? "" : undefined}
      data-meshes-json={scene.meshesJson ?? undefined}
      data-instances-json={scene.instancesJson ?? undefined}
      data-target-volumes-json={scene.targetVolumesJson ?? undefined}
      data-camera-json={world3dCameraDomJson(sceneCamera)}
      data-viewport-camera-json={world3dCameraDomJson(cameraState)}
      data-vortices-json={scene.vorticesJson ?? undefined}
      data-brush-preview-json={brushPreviewJson}
      data-suggestion-menu-json={interaction.suggestionMenu ? JSON.stringify(interaction.suggestionMenu) : ""}
      data-interaction-json={JSON.stringify(interaction)}
      data-status-json={scene.statusJson ?? undefined}
      onContextMenu={(event) => {
        const alt = world3dSuggestionsAltHeld(event.altKey, altHeldRef.current);
        const brushArmed = Boolean(brushMode && hoveredVortexFullIdRef.current);
        if (world3dSuggestionsGestureConsumesContextMenu(alt) || brushArmed) {
          event.preventDefault();
          event.stopPropagation();
          console.warn(`[DEBUG] suggestions-contextmenu hop alt=${event.altKey} held=${altHeldRef.current} brush=${brushMode} hover=${hoveredVortexFullIdRef.current ?? "null"}`);
          if (performance.now() - suggestionsRightDownDispatchedAtRef.current < 80) return;
          if (world3dSuggestionsGestureArmed(alt, hoveredVortexFullIdRef.current) || brushArmed) {
            dispatch("openVortexSuggestions", { fullId: hoveredVortexFullIdRef.current, x: event.clientX, y: event.clientY, windowId: windowInstanceId ?? undefined });
          }
          return;
        }
        if (!requestContextMenu) return;
        const target = resolveWorldContextMenuTarget(interaction, selection);
        event.preventDefault();
        event.stopPropagation();
        void (async () => {
          const surface = world3dContextMenuSurfaceV1(target, selection);
          const menu = await openSurfaceContextMenu(
            requestContextMenu,
            {
              menu: { id: "world3d", args: null },
              surface: { surfaceId: node.surfaceId, kind: "world3d", hits: [...surface.hits], selection: surface.selection.map((group) => ({ domain: group.domain, ids: [...group.ids] })) },
              windowInstanceId: windowInstanceId ?? undefined,
              point: { x: event.clientX, y: event.clientY },
            },
            mapWorldContextMenuSpecs,
            shellContextMenuFallback,
          );
          setContextMenu({ x: event.clientX, y: event.clientY, ...menu });
        })();
      }}
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
      onPointerUp={handlePointerUp}
      onPointerCancel={handlePointerUp}
      onDragEnter={onCatalogueDragEnter}
      onDragLeave={onCatalogueDragLeave}
      onDragOver={onCatalogueDragOver}
      onDrop={onCatalogueDrop}
    >
      <WorldCanvas
        className="h-full w-full"
        cameraUp={(cameraState.up as [number, number, number] | undefined) ?? [0, 0, 1]}
        cameraFov={cameraState.fov}
        background={environment && !isTransparentWorldBackground(environment.background) ? environment.background : undefined}
        gl={environment && isTransparentWorldBackground(environment.background) ? { antialias: true, alpha: true } : undefined}
        shadows={environment?.shadow?.enabled === true ? true : undefined}
        onPointerMissed={handleEmptyClick}
        overlay={
          <>
            {frame ? <IconShotFrame width={frame.width} height={frame.height} shape={frame.shape === "ellipse" ? "ellipse" : "rectangle"} badge={frame.badge !== false} background={frame.background} /> : null}
            <WorldOrbitProjectionSwitchPane spec={worldProjectionSpec} onSpecChange={handleProjectionKindChange} windowElementSegment={windowInstanceId ?? node.surfaceId} />
            {/* 🚧️ Scene overlays are window CONTENT, so they start below the window's own floating chrome
                control row ({@link windowChromeClearedTopOffset}) and stay out of the top-left corner the
                folded engagement's quick-action rail owns — a control painted into that band covers the
                pane toggles and, at `z-40`, swallows their pointer events outright. */}
            <div
              data-slot="world-view-overlay-rail"
              className="pointer-events-none absolute z-40 flex flex-col items-end gap-single"
              style={{ top: windowChromeClearedTopOffset, right: "var(--spacing-single)" }}
            >
              <button
                id={`world3d-frame-instances-${windowInstanceId ?? node.surfaceId}`}
                type="button"
                data-slot="world-frame-instances"
                className={cn("pointer-events-auto rounded px-single py-half text-xs shadow-sm", glassClass)}
                data-level="pane"
                onClick={handleFrameVisibleInstances}
              >
                {shellLabel("ui.host.frameVisible")}
              </button>
              <WorldComputeStatusPane status={computeStatus} glassClass={glassClass} locale={shellScope?.i18n.language} onCancel={() => dispatch(computeStatus.cancelAction)} />
            </div>
            {fillDiagnostic ? <FillDiagnosticOverlay diagnostic={fillDiagnostic} /> : null}
          </>
        }
      >
        <WorldOrbitViewSnapGateProvider>
          <WorldProjectionRig spec={worldProjectionSpec} state={cameraState} seedKey={cameraSeedKey} pendingSpec={pendingProjectionSpec} />
          {fitProjectionContent || projectionFramePending ? (
            <WorldProjectionContentFrame enabled spec={pendingProjectionSpecRef.current ?? worldProjectionSpec} bounds={contentBounds} fov={cameraState.fov} onFramed={handleProjectionContentFrame} />
          ) : null}
          <WorldOrbitGated
            controlsGate={marqueeDown || gumballDragActive || connectDragSource !== null || faceDragSession !== null}
            onCamera={handleCameraChange}
            zoom={cameraState.zoom}
            projection={cameraState.explicitProjection ? cameraState.projection : undefined}
            constraints={worldOrbitConstraints}
            onRightPointerDown={handleWorldOrbitRightPointerDown}
            onNavigationGestures={(gestures) => dispatch(NOTE_WORLD_NAVIGATION_ACTION_ID, { windowId: windowInstanceId ?? node.surfaceId, gestures })}
          />
          <WorldOrbitViewControls
            projectionSpec={worldProjectionSpec}
            externalPendingSpec={externalPendingProjectionSpec}
            onExternalPendingSpecClear={() => setExternalPendingProjectionSpec(null)}
            onCameraChange={handleGizmoCameraChange}
            onPendingSpecChange={setPendingProjectionSpec}
          />
          <WorldLodBridge
            lodRef={lodRef}
            distanceReference={100}
            gridFactor={lod.gridFactor ?? DEFAULT_LOD_GRID_FACTOR}
            gridSnapEnabled={lod.gridSnapEnabled ?? false}
            showLodGrid={lod.showLodGrid ?? true}
            automaticLod={lod.automaticLod ?? true}
            depthVariableLod={lod.depthVariableLod ?? false}
            manualLod={lod.manualLod ?? DEFAULT_MANUAL_LOD}
            gridDatum={[0, 0, 0]}
          >
            <ambientLight color={environment?.ambient?.color ?? "#ffffff"} intensity={environment?.ambient?.intensity ?? 1.15} />
            {environment?.sun?.enabled === true ? (
              <directionalLight
                color={environment.sun?.color ?? "#ffffff"}
                intensity={environment.sun?.intensity ?? 0.85}
                position={sunPositionFromAzimuthElevation(environment.sun?.azimuth ?? 45, environment.sun?.elevation ?? 35)}
                castShadow={environment.shadow?.enabled === true}
              />
            ) : (
              <>
                <hemisphereLight color="#ffffff" groundColor="#9aa0ab" intensity={1.35} position={[0, 0, 1]} />
                <directionalLight position={[12, 18, 10]} intensity={2.4} />
                <directionalLight position={[-14, -10, 6]} intensity={1.2} />
                <directionalLight position={[0, 0, -16]} intensity={0.75} />
              </>
            )}
            {fit?.enabled ? <WorldAutoFit groupRef={instancesGroupRef} fitKey={`${fit.revision ?? 0}:${meshes.map((mesh) => mesh.url ?? mesh.id).join(",")}`} padding={fit.padding ?? 1.25} camera={cameraState} onFitted={handleAutoFitCameraChange} /> : null}
            <CameraRefBridge cameraRef={cameraRef} />
            <RaycasterPickTuning />
            <WorldVortexHitStamp vortices={displayVortices} hostRef={hostRef} />
            <WorldGumballHitStamp target={selection.gumballTarget} active={Boolean(selection.gumballActive) && isWorldTransformGumballMode(selection.transformMode)} hostRef={hostRef} />
            {windowInstanceId ? (
              <IntroductionWorldResolverBridge windowInstanceId={windowInstanceId} vortices={vortices} instances={instances} attractions={attractions} />
            ) : null}
            {brushMeshUrls.map((url) => (
              <Suspense key={url} fallback={null}>
                <BrushMeshRegistrar url={url} revision={brushMeshRevisions.generation + (brushMeshRevisions.urls[url] ?? 0)} onRegister={handleRegisterBrushMesh} />
              </Suspense>
            ))}
            <WorldTerrainLayer terrainJson={scene?.terrainJson} cameraPosition={cameraState.position} cameraTarget={cameraState.target} />
            <WorldPointCloudLayer pointsJson={scene?.pointsJson} />
            <group ref={instancesGroupRef}>
              <WorldInstancesLayer
                instances={instances}
                meshes={meshes}
                selection={selection}
                persistentSelectionMode={persistentSelectionMode}
                palette={meshStylePalette}
                projectionSpec={worldProjectionSpec}
                onInstancePointerDown={handleInstancePointerDown}
                onInstancePointerMove={handleInstancePointerMove}
                onWorldPick={handleWorldPick}
                onComponentHover={handleComponentHover}
                onPaintAt={paintMode ? handlePaintAt : undefined}
                gumballDragActive={gumballDragActive}
                onGumballDraggingChanged={setGumballDragActive}
                onGumballDragStart={handleGumballDragStart}
                onGumballDrag={handleGumballDrag}
                onGumballDragEnd={handleGumballDragEnd}
                onFaceDragStart={handleFaceDragStart}
                mergedComponentIds={visibleSelectionPreview.mergedComponentIds}
                mergedInstanceIds={visibleSelectionPreview.mergedInstanceIds}
                blockPick={worldInstancePickBlocked(activeUtility)}
                environment={environment}
                revealCutoffs={interaction.revealCutoffs}
              />
            </group>
            <WorldVortexMarkers
              vortices={displayVortices}
              palette={meshStylePalette}
              brushMode={brushMode}
              selectionMode={selectionMode}
              connectSourceFullId={connectDragSource?.fullId}
              onHover={handleVortexHover}
              onVortexSelect={handleVortexSelect}
              onBrushPlace={handleBrushPlace}
              onVortexPointerArm={handleVortexPointerArm}
              onVortexPointerMove={handleVortexPointerMove}
              onVortexPointerUp={handleVortexPointerUp}
              onConnectDragHover={handleConnectDragHover}
              onConnectDragDrop={handleConnectDragDrop}
            />
            {connectDragSource && connectDragHoverPosition ? <WorldConnectRubberBand from={connectDragSource.position} to={connectDragHoverPosition} /> : null}
            <WorldAttractionLines attractions={attractions} />
            {visibleBrushPreview ? <BrushPreviewGhost preview={visibleBrushPreview} meshes={meshes} palette={meshStylePalette} /> : null}
            {!visibleBrushPreview && catalogueDropPreview ? <CatalogueDropGhost preview={catalogueDropPreview} meshes={meshes} palette={meshStylePalette} /> : null}
            {engagementPreview.length > 0 ? <EngagementPreviewLayer items={engagementPreview} color={colors.hover} /> : null}
            <WorldVolumeLayer
              volumes={targetVolumes
                .filter((volume) => !volume.hidden)
                .map((volume) => ({
                  id: volume.id,
                  origin: volume.origin as [number, number, number],
                  orientation: volume.orientation as [number, number, number, number] | undefined,
                  scale: volume.scale,
                  color: volume.color,
                  hidden: volume.hidden,
                  locked: volume.locked,
                }))}
              selectedIds={targetVolumeSelectedIds}
              interactive={volumeLayersInteractive}
              gumballConfig={volumeGumballConfig}
              relocateActive={activeUtility === "transform"}
              translationSnap={gridSnapEnabled ? gridFactor : undefined}
              onSelect={handleTargetVolumeSelect}
              onRelocate={handleTargetVolumeRelocate}
            />
            {volumeBrushMode && voxelHoverOrigin ? <WorldVoxelPreviewBox origin={voxelHoverOrigin} dims={interaction.voxelDims ?? [1, 1, 1]} gridFactor={interaction.gridFactor ?? DEFAULT_LOD_GRID_FACTOR} /> : null}
            <WorldReferenceLayer
              references={references
                .filter((reference) => !reference.hidden)
                .map((reference) => ({
                  id: reference.id,
                  source: { url: reference.url, mediaKind: "image" as const },
                  origin: reference.origin as [number, number, number],
                  widthWorld: reference.widthWorld,
                  locked: reference.locked,
                  opacity: reference.opacity,
                }))}
              selectedIds={referenceSelectedIds}
              hoveredId={referenceHoveredId}
              onSelect={(id) => handleReferenceSelect(id)}
              onHover={handleReferenceHover}
            />
          </WorldLodBridge>
        </WorldOrbitViewSnapGateProvider>
      </WorldCanvas>
      {marqueeDragActive && marqueeStart && marqueeEnd && world3dMarqueeOverlayShape(method) === "polygon" ? (
        <SelectionMarquee coverage={marqueeCoverage} shape="polygon" points={marqueePath} />
      ) : marqueeDragActive && marqueeStart && marqueeEnd && world3dMarqueeOverlayShape(method) === "rect" ? (
          <SelectionMarquee
            coverage={marqueeCoverage}
            shape="rect"
            rect={{
              x: Math.min(marqueeStart.x, marqueeEnd.x),
              y: Math.min(marqueeStart.y, marqueeEnd.y),
              width: Math.abs(marqueeEnd.x - marqueeStart.x),
              height: Math.abs(marqueeEnd.y - marqueeStart.y),
            }}
          />
      ) : null}
      <ContextMenuController
        title={contextMenuTitleLabel}
        open={contextMenu != null && (contextMenu.items?.length ?? 0) > 0 && !suggestionMenuOwnsThisWindow}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenu?.items ?? []}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
      {suggestionMenuOwnsThisWindow ? (
        <ContextMenuController
          title={suggestionMenuTitleLabel}
          open
          closeOnSelect={false}
          position={{ x: interaction.suggestionMenu!.x, y: interaction.suggestionMenu!.y }}
          items={mapSuggestionContextMenuSpecs(
            suggestionMenuItems(interaction.suggestionMenu!, interaction.brushCandidateIndex ?? 0, {
              checkingPlacement: suggestionMenuCheckingPlacementLabel,
              noPlacement: suggestionMenuNoPlacementLabel,
            }),
          )}
          onOpenChange={(open) => {
            if (!open) handleSuggestionClose();
          }}
        />
      ) : null}
    </div>
  );
}
//#endregion World3dHost
//#endregion 🔖️World3dHost
