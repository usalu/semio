// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/World3dHost/component.tsx
/** @emoji 🌐️ `World3dHost` — the 3D world viewport scene host: mesh/instance parsing, point-cloud and vortex-marker
 * layers, catalogue-drop and selection-preview stores, instance chrome, the transform gumball, and the full R3F
 * `World3dHost` component mounted inside a Mode window. The largest scene host in this package. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import React, { createContext, useCallback, useContext, useEffect, useLayoutEffect, useMemo, useRef, useState, Suspense, useSyncExternalStore, type ComponentProps, type DragEvent, type MouseEvent } from "react";
import { Box3, BufferAttribute, BufferGeometry, Color, DoubleSide, EdgesGeometry, Group, LineBasicMaterial, LineSegments, Mesh, MeshStandardMaterial, Object3D, OrthographicCamera, PointsMaterial, Quaternion, ShaderMaterial, TextureLoader, Vector3, type ThreeEvent } from "three";
import { useFrame, useLoader, useThree } from "@react-three/fiber";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
import { clearColorResolveCache, resolveColorHex, semanticVar, themeColorVar, tokenVar } from "@semio-tech/ui-styling";
import {
  CATALOGUE_DRAG_MIME,
  CELEBRATE_STAMP_DURATION_MS,
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
  type SelectionMergeMode,
  type TutorialCameraDriver,
  type UiLabel,
} from "@semio-tech/ui-react";
import { windowElementId, type ComponentSceneHostProps, type ContextMenuItemSpec, type PluginContextMenuSurfaceTarget } from "@semio-tech/framework";
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
import { CAMERA_SYNC_DEBOUNCE_MS } from "../Canvas2dHost/🟦️component.tsx";
import { openSurfaceContextMenu, useShellContextMenuFallback, wireLabel } from "../Interpreter/🟦️component.tsx";
import { WorldTerrainLayer } from "../WorldTerrainLayer/🟦️component.tsx";
import { base64ToBytes } from "../Paint2dHost/🟦️component.tsx";
import { createCoalescingActionDispatcher, createInFlightSkippingInterval, isRevealCutoffHidden, registeredPuzzle3dBrushMeshes, NOTE_WORLD_NAVIGATION_ACTION_ID, PUZZLE3D_FILL_REVEAL_GROUP_ID, reconcileCommittedRevealCutoffs, worldRevealCutoffStore, shellLabel } from "../ShellHelpers/🟦️component.tsx";
import { SetWindowIconContext, SetWindowTitleContext, useMapContextMenuSpecs } from "../ShellHost/🟦️component.tsx";
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
  readonly selectionMergeMode?: SelectionMergeMode;
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
  readonly voxelDims?: readonly [number, number, number];
  readonly gridFactor?: number;
  readonly suggestionMenu?: WorldSuggestionMenuRecord | null;
  readonly fillBuild?: WorldFillBuildRecord;
  /** 🪣️ Committed reveal cutoff per reveal group id (see `WindowMeasure.Slider.reveal`) — instances
   * tagged `revealIndex` below this value are shown. Seeds `RevealCutoffStore` when no drag is live. */
  readonly revealCutoffs?: Readonly<Record<string, number>>;
};

type WorldLodRecord = {
  readonly gridFactor?: number;
  readonly gridSnapEnabled?: boolean;
  readonly showLodGrid?: boolean;
  readonly automaticLod?: boolean;
  readonly depthVariableLod?: boolean;
  readonly manualLod?: number;
};

type WorldVortexRecord = {
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

type WorldAttractionRecord = {
  readonly id: string;
  readonly from: readonly [number, number, number];
  readonly to: readonly [number, number, number];
  readonly color?: string;
};

type WorldTargetVolumeRecord = {
  readonly id: string;
  readonly origin: readonly [number, number, number];
  readonly orientation?: readonly [number, number, number, number];
  readonly scale?: readonly [number, number, number] | number;
  readonly color?: string;
  readonly hidden?: boolean;
  readonly locked?: boolean;
  readonly selected?: boolean;
};

type WorldReferenceRecord = {
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
};

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
let celebratingWorldInstanceTimer: ReturnType<typeof setTimeout> | null = null;

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
  const timeMs = typeof document !== "undefined" ? (document.timeline?.currentTime ?? performance.now()) : 0;
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
  return { ...pose, fov: "fov" in pendingSpec ? pendingSpec.fov : sceneCamera.fov, explicitProjection: true };
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
    const framed: WorldParsedCameraState = { ...pose, fov: "fov" in props.spec ? props.spec.fov : props.fov, explicitProjection: true };
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

function parseMeshes(meshesJson: string): WorldMeshRecord[] {
  try {
    const parsed = JSON.parse(meshesJson);
    return Array.isArray(parsed) ? (parsed as WorldMeshRecord[]) : [];
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

function parseSelection(selectionJson: string): WorldSelectionRecord {
  try {
    return JSON.parse(selectionJson) as WorldSelectionRecord;
  } catch {
    return { method: "rectangle", ids: [] };
  }
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
  const groups: NonNullable<PluginContextMenuSurfaceTarget["selection"]> = [];
  if (domains.nodes.length > 0) groups.push({ domain: "node", ids: domains.nodes });
  if (domains.edges.length > 0) groups.push({ domain: "edge", ids: domains.edges });
  if (domains.handles.length > 0) groups.push({ domain: "handle", ids: domains.handles });
  return groups;
}

/** @emoji 🖱️ Maps plugin-authored {@link ContextMenuItemSpec} rows onto UI {@link ContextMenuItem} rows, binding select/hover to host `dispatch`. */
export function mapContextMenuSpecs(
  specs: readonly ContextMenuItemSpec[] | null | undefined,
  dispatch: (action: string, args?: Record<string, unknown>) => void,
  keysByActionId?: ReadonlyMap<string, string>,
): ContextMenuItem[] {
  return (specs ?? []).map((spec) => {
    const boundKeys = spec.action ? keysByActionId?.get(spec.action) : undefined;
    const shortcut = spec.shortcut ?? (boundKeys ? formatKeybindingShortcut(boundKeys) : undefined);
    return {
      id: spec.id,
      label: wireLabel(spec.label),
      icon: spec.icon,
      color: spec.color,
      shortcut,
      disabled: spec.disabled,
      separator: spec.separator,
      checked: spec.checked,
      destructive: spec.destructive,
      onSelect: spec.action
        ? (event) => {
            const clientX = event && "clientX" in event && typeof (event as MouseEvent).clientX === "number" ? (event as MouseEvent).clientX : undefined;
            const clientY = event && "clientY" in event && typeof (event as MouseEvent).clientY === "number" ? (event as MouseEvent).clientY : undefined;
            const pointArgs = spec.action === "openVortexSuggestions" && clientX != null && clientY != null ? { x: clientX, y: clientY } : undefined;
            dispatch(spec.action!, { ...spec.args, ...pointArgs });
          }
        : undefined,
      onHover: spec.hoverAction
        ? () => {
            if (spec.hoverAction === "hoverSuggestion") {
              console.log(`[DEBUG] hoverSuggestion`, { index: spec.hoverArgs?.index, color: spec.color, icon: spec.icon });
            }
            dispatch(spec.hoverAction!, spec.hoverArgs);
          }
        : undefined,
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

function parseBrushPreview(brushPreviewJson: string | undefined): WorldBrushPreviewRecord | null {
  if (!brushPreviewJson) return null;
  try {
    return JSON.parse(brushPreviewJson) as WorldBrushPreviewRecord;
  } catch {
    return null;
  }
}

type WorldContextMenuTarget = { readonly kind: "vortex" | "object" | "reference"; readonly id: string };

/** @emoji 🖱️ Resolves which entity a plain right-click should select-then-open a menu for, by priority: hovered vortex, then hovered object component, then hovered reference. */
export function resolveWorldContextMenuTarget(interaction: WorldInteractionRecord, selection: WorldSelectionRecord): WorldContextMenuTarget | null {
  if (interaction.hoveredVortexFullId) return { kind: "vortex", id: interaction.hoveredVortexFullId };
  if (selection.hoveredComponent?.objectId) return { kind: "object", id: selection.hoveredComponent.objectId };
  const hoveredId = selection.hoveredId;
  if (hoveredId?.startsWith("reference:")) return { kind: "reference", id: hoveredId.slice("reference:".length) };
  return null;
}

/** @emoji 🚫️ Instance-mesh picking must be disabled for fill/brush engagements — otherwise a click meant for a vortex marker or a fill/voxel gesture falls through and selects/gumballs the underlying object instead. */
export function worldInstancePickBlocked(activeUtility: string | undefined): boolean {
  return activeUtility === "fill" || activeUtility === "brush" || activeUtility === "volumeBrush" || activeUtility === "surfaceBrush";
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
}) {
  const gltf = useLoader(GLTFLoader, url);
  const invalidate = useThree((state) => state.invalidate);
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

function BrushMeshRegistrar({ url, onRegister }: { readonly url: string; readonly onRegister: (url: string, positions: number[], indices: number[]) => void }) {
  const gltf = useLoader(GLTFLoader, url);
  useEffect(() => {
    const mesh = extractGlbCollisionMesh(gltf);
    if (mesh.positions.length === 0 || mesh.indices.length === 0) return;
    onRegister(url, mesh.positions, mesh.indices);
  }, [gltf, onRegister, url]);
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
  readonly paintFromHit: (objectId: string, mesh: WorldMeshData, event: ThreeEvent<PointerEvent> & { faceIndex?: number | null; uv?: { x: number; y: number } }) => void;
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
          {hasShadedMesh ? (
            <PaintTexturedMesh
            geometry={geometry}
            style={style}
            styleKind={styleKind}
            textureBase64={paintTextureBase64}
            flatShading={flatShading}
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
            />
          </Suspense>
        </group>
      ) : (
        <mesh
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
  /** 🐚️ This shell's own `SelectionModeStore` value — see `resolveWorldSelectionMergeMode`'s doc. */
  readonly persistentSelectionMode: SelectionMergeMode;
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

  const mergeMode = (event: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => componentMergeArg(resolveWorldSelectionMergeMode(selection.selectionMergeMode, event, persistentSelectionMode));

  const paintFromHit = (objectId: string, mesh: WorldMeshData, event: ThreeEvent<PointerEvent> & { faceIndex?: number | null; uv?: { x: number; y: number } }) => {
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
 * alongside `WorldTerrainLayer` in the `World3dHost` scene tree. */
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
            <mesh position={vortex.position as [number, number, number]} visible={false} {...pointerHandlers}>
              <sphereGeometry args={[radius, 16, 16]} />
              <meshBasicMaterial transparent opacity={0} depthWrite={false} />
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

/** @emoji 🧊️ Invisible ground plane (Z-up XY plane, matching this world's up axis) that tracks the grid-snapped cursor while voxel-editing target volumes; Alt+click commits a volume there. */
function WorldVoxelGroundPlane({ gridFactor, onHover, onPlace }: { readonly gridFactor: number; readonly onHover: (origin: readonly [number, number, number] | null) => void; readonly onPlace: (origin: readonly [number, number, number]) => void }) {
  const snap = (value: number) => Math.round(value / gridFactor) * gridFactor;
  return (
    <mesh
      onPointerMove={(event) => {
        event.stopPropagation();
        onHover([snap(event.point.x), snap(event.point.y), snap(event.point.z)]);
      }}
      onPointerOut={(event) => {
        event.stopPropagation();
        onHover(null);
      }}
      onClick={(event) => {
        event.stopPropagation();
        if (!event.nativeEvent.altKey) return;
        onPlace([snap(event.point.x), snap(event.point.y), snap(event.point.z)]);
      }}
    >
      <planeGeometry args={[10000, 10000]} />
      <meshBasicMaterial visible={false} />
    </mesh>
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

const MARQUEE_DRAG_THRESHOLD_PX = 4;

/** @emoji 🎯️ Generic add/remove/toggle/replace merge, mirrors `selectionMergeIds` from `@semio-tech/ui-react` for non-string id sets. */
function mergeIdSet<T>(mode: ReturnType<typeof marqueeModeFromModifiers>, current: readonly T[], incoming: readonly T[]): T[] {
  const currentSet = new Set(current);
  const incomingSet = new Set(incoming);
  if (mode === "default") return [...incomingSet];
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

/** @emoji 🖱️ additive→add, subtractive→remove, invertive→toggle, default→replace (whole-instance picks/marquee). */
function instanceMergeArg(mode: ReturnType<typeof marqueeModeFromModifiers>): string {
  if (mode === "additive") return "add";
  if (mode === "subtractive") return "remove";
  if (mode === "invertive") return "toggle";
  return "replace";
}

/** @emoji 🎯️ Resolves a world surface's declared selection mode before falling back to the shared selection toolbar. */
export function resolveWorldSelectionMergeMode(
  configuredMode: SelectionMergeMode | undefined,
  modifiers: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean },
  persistentMode?: SelectionMergeMode,
): SelectionMergeMode {
  if (configuredMode === undefined) return marqueeModeFromModifiers(modifiers, persistentMode);
  if (configuredMode !== "default") return configuredMode;
  const shift = modifiers.shiftKey === true;
  const ctrl = modifiers.ctrlKey === true || modifiers.metaKey === true;
  if (shift && ctrl) return "invertive";
  if (shift) return "additive";
  if (ctrl) return "subtractive";
  return "default";
}

/** @emoji 🖱️ Same as {@link instanceMergeArg} but a bare click (no modifiers) defaults to invertive. */
function componentMergeArg(mode: ReturnType<typeof marqueeModeFromModifiers>): string {
  if (mode === "additive") return "add";
  if (mode === "subtractive") return "remove";
  return "toggle";
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
    const localCorners = meshData ? meshBoundsCorners(meshData) : [[0, 0, 0] as const];
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

function CameraRefBridge({ cameraRef }: { readonly cameraRef: React.MutableRefObject<import("three").Camera | null> }) {
  const camera = useThree((state) => state.camera);
  useEffect(() => {
    cameraRef.current = camera;
  }, [camera, cameraRef]);
  return null;
}

/** @emoji 🧭️ Registers this window's live camera and entity data as the introduction demonstration
 * engine's resolver for `windowElementId(windowInstanceId)` — the same element id `windowElementId(kind.id)`
 * builds for authoring (see `mit-bestand/aggregator/🟦️brand.ts`). Only the base (non-split) window instance
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

/** @emoji 🔀️ Portals the world's projection-kind switch into the enclosing window's pane host (see `usePaneSlot`), defaulting to bottom-right under the navigation cube — the cube sits above the folded chrome and the unfolded pane grows over it. Falls back to a local overlay when no pane host is mounted yet (or outside one). */
function WorldOrbitProjectionSwitchPane({ spec, onSpecChange }: { readonly spec: WorldProjectionSpec; readonly onSpecChange: (spec: WorldProjectionSpec) => void }) {
  const [anchor, setAnchor] = useState<Anchor>("bottom-right");
  const [folded, setFolded] = useState(true);
  const projectionLabel = useLabel("ui.host.projection");
  const pane = (
    <Pane id="framework.worldOrbit.projection" anchor={anchor} onAnchorChange={setAnchor} folded={folded} onFoldToggle={() => setFolded((value) => !value)} icon={worldProjectionSpecIconId(spec) as IconName} label={projectionLabel}>
      <WorldProjectionKindSwitch spec={spec} onSpecChange={onSpecChange} />
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

//#region World3dHost
export function World3dHost({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.world3d;
  // 🐚️ Optional — this host is also unit-tested standalone, outside any `ShellScopeProvider`.
  const shellScope = useShellScopeOptional();
  // 🐚️ Read as reactive state (not `shellScope.selection.get()` inline at each use) because this value
  // is consumed inside `<Canvas>` r3f subtrees (`WorldInstancesLayer`) via a prop, not context — R3F
  // primitives aren't guaranteed to re-render just because an *outer* DOM component's context changed,
  // so the toolbar's mode toggle needs an explicit subscription to actually propagate.
  const [persistentSelectionMode, setPersistentSelectionMode] = useState<SelectionMergeMode>(() => shellScope?.selection.get() ?? "default");
  useEffect(() => {
    if (!shellScope) return;
    return shellScope.selection.subscribe(() => setPersistentSelectionMode(shellScope.selection.get()));
  }, [shellScope]);
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const setWindowTitle = useContext(SetWindowTitleContext);
  const setWindowIcon = useContext(SetWindowIconContext);
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const contextMenuTitleLabel = useLabel("ui.surfaceContextMenu.scene");
  const meshStylePalette = useMeshStylePalette();
  const colors = useMemo(() => semanticColorsFromPalette(meshStylePalette), [meshStylePalette]);
  const sceneCameraJson = scene?.cameraJson ?? "{}";
  const parsedCamera = useMemo(() => parseCameraState(sceneCameraJson), [sceneCameraJson]);
  const instances = useMemo(() => parseInstances(scene?.instancesJson ?? "[]"), [scene?.instancesJson]);
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
    console.log("[DEBUG] world3d viewport reattached to scene camera", { surfaceId: node.surfaceId, sceneCameraJson });
  }, [node.surfaceId, sceneCameraJson]);
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
        return { kind: "orbit", position: live.position, target: live.target, up: live.up ?? [0, 0, 1], fov: live.fov };
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
  const selection = useMemo(() => parseSelection(scene?.selectionJson ?? "{}"), [scene?.selectionJson]);
  const vortices = useMemo(() => parseJsonArray<WorldVortexRecord>(scene?.vorticesJson), [scene?.vorticesJson]);
  const attractions = useMemo(() => parseJsonArray<WorldAttractionRecord>(scene?.attractionsJson), [scene?.attractionsJson]);
  const targetVolumes = useMemo(() => parseJsonArray<WorldTargetVolumeRecord>(scene?.targetVolumesJson), [scene?.targetVolumesJson]);
  const interaction = useMemo(() => parseInteraction(scene?.interactionJson), [scene?.interactionJson]);
  const lod = useMemo(() => parseLod(scene?.lodJson), [scene?.lodJson]);
  const engagementPreview = useMemo(() => parseEngagementPreview(scene?.engagementPreviewJson), [scene?.engagementPreviewJson]);
  const brushPreview = useMemo(() => parseBrushPreview(scene?.brushPreviewJson), [scene?.brushPreviewJson]);
  const environment = useMemo(() => parseEnvironment(scene?.environmentJson), [scene?.environmentJson]);
  const frame = useMemo(() => parseFrame(scene?.frameJson), [scene?.frameJson]);
  const fit = useMemo(() => parseFit(scene?.fitJson), [scene?.fitJson]);
  // 🧵️ Off-main-thread compute status (see `World3dScene.statusJson`) — the meshes above stay the
  // last-known-good (stale) cache while a plugin worker's `flowEvalTick` chain is still resolving.
  const computing = useMemo(() => {
    try {
      return (JSON.parse(scene?.statusJson ?? "{}") as { readonly computing?: boolean; readonly label?: string }).computing === true;
    } catch {
      return false;
    }
  }, [scene?.statusJson]);
  const activeUtility = interaction.activeUtility ?? "select";
  const fillMode = activeUtility === "fill";
  const brushMode = activeUtility === "brush";
  const volumeBrushMode = activeUtility === "volumeBrush";
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
  const [contextMenu, setContextMenu] = useState<{ readonly x: number; readonly y: number; readonly items: readonly ContextMenuItem[] } | null>(null);
  const cameraRef = useRef<import("three").Camera | null>(null);
  const catalogueDragDepthRef = useRef(0);
  const catalogueDragEncodedRef = useRef<string | null>(null);
  /** 🎉️ Pre-drop instance ids — when the scene gains new ids after {@link commitCatalogueDropAt}, those objects celebrate. */
  const pendingCelebrateCatalogueDropIdsRef = useRef<ReadonlySet<string> | null>(null);
  const instancesRef = useRef(instances);
  instancesRef.current = instances;
  const wasMarqueeDragRef = useRef(false);
  const [marqueeCommitHold, setMarqueeCommitHold] = useState<{
    readonly mergedComponentIds: readonly number[] | null;
    readonly mergedInstanceIds: readonly string[] | null;
  } | null>(null);
  const connectDropConsumedRef = useRef(false);
  const engagementPointerMoveInFlightRef = useRef(false);
  const engagementPointerMoveLastPointRef = useRef<readonly [number, number, number] | null>(null);
  const gumballDragStartPoseRef = useRef<GumballPose | null>(null);
  /** 🧲️ Serialized WASM begin/end chain — mid-drag is local-only; one absolute start→end delta commits on drag end. */
  const gumballDragChainRef = useRef(Promise.resolve());
  const gumballDragDebugTickRef = useRef(0);
  const selectionMode = selection.selectionMode ?? selection.granularity ?? "mesh";
  const gridSnapEnabled = lod.gridSnapEnabled ?? false;
  const suggestionMenuOpen = Boolean(interaction.suggestionMenu?.open);
  const suggestionMenuOwnsThisWindow = worldSuggestionMenuOwnsWindow(interaction.suggestionMenu, windowInstanceId);
  const suggestionMenuCheckingPlacementLabel = useLabel("ui.host.checkingPlacement");
  const suggestionMenuNoPlacementLabel = useLabel("ui.host.noPlacement");
  const suggestionMenuTitleLabel = useLabel("ui.surfaceContextMenu.placementSuggestions");
  useEffect(() => {
    if (!brushPreview) return;
    console.log(`[DEBUG] brushPreview`, { color: brushPreview.color, objectKindId: brushPreview.objectKindId, meshUrl: brushPreview.meshUrl });
  }, [brushPreview]);
  const gridFactor = lod.gridFactor ?? interaction.gridFactor ?? DEFAULT_LOD_GRID_FACTOR;
  const marqueeDown = marqueePath.length > 0;
  const method = selection.method ?? "rectangle";
  const marqueeStart = marqueePath[0];
  const marqueeEnd = marqueePath[marqueePath.length - 1];
  const marqueeDragActive = marqueeDown && marqueePath.length > 1 && marqueeStart != null && marqueeEnd != null && Math.hypot(marqueeEnd.x - marqueeStart.x, marqueeEnd.y - marqueeStart.y) > MARQUEE_DRAG_THRESHOLD_PX;
  const marqueeMergeMode = useMemo(() => resolveWorldSelectionMergeMode(selection.selectionMergeMode, marqueeModifiers, persistentSelectionMode), [marqueeModifiers, selection.selectionMergeMode, persistentSelectionMode]);
  const marqueeCoverage: SelectionMarqueeCoverage = useMemo(() => {
    if (!marqueeDragActive || !marqueeStart || !marqueeEnd) return "full";
    return marqueeCoverageFromGesture({ method, startX: marqueeStart.x, endX: marqueeEnd.x, path: marqueePath });
  }, [marqueeDragActive, marqueeEnd, marqueePath, marqueeStart, method]);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) =>
      onAction({
        controllerId: node.controllerId,
        action,
        args: { surfaceId: node.surfaceId, ...args },
      }),
    [node.controllerId, node.surfaceId, onAction],
  );

  const adoptViewportCamera = useCallback(
    (next: WorldCameraState, applyToRig: boolean) => {
      setViewportCamera((prev) => mergeWorldViewportCamera(prev ?? sceneCamera, next));
      setViewportOwned((owned) => {
        if (!owned) console.log("[DEBUG] world3d viewport detached from shared scene camera", { surfaceId: node.surfaceId });
        return true;
      });
      if (applyToRig) setDetachEpoch((epoch) => epoch + 1);
    },
    [node.surfaceId, sceneCamera],
  );

  // 🧭️ Syncs a completed user-driven camera gesture (orbit/pan/zoom end, gizmo view snap) to the plugin so
  // the shell-side command-history panel has something to show — trailing-debounced like the 2D canvas'
  // own camera sync (`CAMERA_SYNC_DEBOUNCE_MS`) so a scroll/wheel burst doesn't spam one dispatch per tick.
  // Never wired into a programmatic camera change (auto-fit, scene-camera echo reattach) — only genuine
  // user gestures call this.
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
    [dispatch, node.surfaceId, references, selectionMode],
  );

  const handleReferenceHover = useCallback(
    (id: string | null) => {
      if (!id) {
        dispatch("referenceHover", {});
        return;
      }
      dispatch("referenceHover", { referenceId: id });
    },
    [dispatch],
  );

  const handleTargetVolumeSelect = useCallback(
    (id: string) => {
      const volume = targetVolumes.find((entry) => entry.id === id);
      if (volume?.locked) {
        dispatch("worldPick", { granularity: selectionMode, id: null, merge: "replace" });
        return;
      }
      dispatch("setSelection", {
        selection: { objectIds: [], vortexIds: [], attractionIds: [], targetVolumeIds: [id], referenceIds: [] },
      });
    },
    [dispatch, selectionMode, targetVolumes],
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

  const registeredBrushMeshesRef = useRef(registeredPuzzle3dBrushMeshes);
  const handleRegisterBrushMesh = useCallback(
    (url: string, positions: number[], indices: number[]) => {
      if (registeredBrushMeshesRef.current.has(url)) return;
      registeredBrushMeshesRef.current.add(url);
      dispatch("registerBrushMesh", { url, positions, indices });
    },
    [dispatch],
  );

  // 👻️ Include the live brush/suggestion ghost URL so collision precompute can register kinds that are
  // not yet placed in the scene — otherwise suggestions stay pending and never emit a 3D preview.
  const brushMeshUrls = useMemo(() => [...new Set([...meshes.map((mesh) => mesh.url).filter((url): url is string => Boolean(url)), ...(brushPreview?.meshUrl ? [brushPreview.meshUrl] : [])])], [brushPreview?.meshUrl, meshes]);

  const handleZoomToSelection = useCallback(() => {
    const selectedIds = new Set(selection.ids ?? []);
    if (selectedIds.size === 0) return;
    const selected = instances.filter((instance) => selectedIds.has(instance.id));
    if (selected.length === 0) return;
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
  }, [adoptViewportCamera, cameraState.projection, cameraState.up, cameraState.zoom, instances, selection.ids]);

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
  useEffect(() => {
    hoveredVortexFullIdRef.current = interaction.hoveredVortexFullId ?? null;
  }, [interaction.hoveredVortexFullId]);

  const handleWorldOrbitRightPointerDown = useCallback(
    (event: PointerEvent) => {
      if (event.altKey && hoveredVortexFullIdRef.current) {
        setVortexPointerArm(null);
        setConnectDragSource(null);
        setConnectDragHoverPosition(null);
        dispatch("openVortexSuggestions", { fullId: hoveredVortexFullIdRef.current, x: event.clientX, y: event.clientY, windowId: windowInstanceId ?? undefined });
        return false;
      }
      return true;
    },
    [dispatch, windowInstanceId],
  );

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
  useEffect(() => {
    if (!(interaction.suggestionMenu?.open && interaction.suggestionMenu.pending)) return;
    return createInFlightSkippingInterval(() => {
      if (interactivePluginActionInFlight()) return;
      dispatch("suggestionsTick");
    }, 120);
  }, [dispatch, interaction.suggestionMenu?.open, interaction.suggestionMenu?.pending]);

  const fillBuildPending = Boolean(interaction.fillBuild && !interaction.fillBuild.done);
  useEffect(() => {
    if (!(activeUtility === "fill" && fillBuildPending)) return;
    return createInFlightSkippingInterval(() => {
      if (interactivePluginActionInFlight()) return;
      dispatch("fillBuildTick");
    }, 120);
  }, [activeUtility, dispatch, fillBuildPending]);

  const selectionArgs = useCallback(
    () => ({
      mode: selection.selectionMode ?? selection.granularity ?? "mesh",
      ids: selection.componentIds ?? [],
    }),
    [selection.componentIds, selection.granularity, selection.selectionMode],
  );

  const handleInstancePointerDown = useCallback(
    (id: string, index: number, event: { shiftKey: boolean; ctrlKey: boolean; metaKey: boolean }) => {
      const merge = instanceMergeArg(resolveWorldSelectionMergeMode(selection.selectionMergeMode, event, persistentSelectionMode));
      const record = instances.find((entry) => entry.id === id);
      if (record?.disabled) {
        dispatch("worldPick", { granularity: selectionMode, id: null, merge });
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
    [dispatch, instances, selection.selectionMergeMode, selectionMode],
  );

  const dispatchInstanceHover = useMemo(
    () =>
      createCoalescingActionDispatcher<string | null>((id) => {
        if (id == null) dispatch("setHover", {});
        else dispatch("setHover", { objectId: id, mode: "mesh", id: 0 });
      }),
    [dispatch],
  );

  const dispatchVortexHover = useMemo(
    () =>
      createCoalescingActionDispatcher<string | null>((fullId) => {
        if (!fullId) dispatch("worldVortexHover", {});
        else dispatch("worldVortexHover", { fullId });
      }),
    [dispatch],
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
      dispatchVortexHover(fullId);
    },
    [dispatchVortexHover],
  );

  const handleVortexSelect = useCallback(
    (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
      const merge = instanceMergeArg(resolveWorldSelectionMergeMode(selection.selectionMergeMode, event ?? {}, persistentSelectionMode));
      dispatch("worldVortexSelect", { fullId, merge });
    },
    [dispatch, selection.selectionMergeMode],
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

  const handleBrushPlace = useCallback(() => {
    const args = brushObjectPlacementArgs(brushPreview);
    if (!args) return;
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
      if (!payload) return Promise.resolve();
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
      gumballDragStartPoseRef.current = before;
      gumballDragDebugTickRef.current = 0;
      console.log("[DEBUG] gumball drag begin", { position: before.position });
      void enqueueGumballDispatch(() => Promise.resolve(dispatch("transformBegin")));
    },
    [dispatch, enqueueGumballDispatch],
  );

  const handleGumballDrag = useCallback((_kind: GumballHandleKind, _pose: GumballPose) => {
    // ⚡️ Mid-drag stays local (WorldInstancesLayer imperative preview) — no WASM/React composite rebuild.
  }, []);

  const handleGumballDragEnd = useCallback(
    (kind: GumballHandleKind, before: GumballPose, after: GumballPose) => {
      const startPose = gumballDragStartPoseRef.current ?? before;
      gumballDragStartPoseRef.current = null;
      const tick = ++gumballDragDebugTickRef.current;
      console.log("[DEBUG] gumball drag end", {
        kind,
        tick,
        delta: [after.position[0] - startPose.position[0], after.position[1] - startPose.position[1], after.position[2] - startPose.position[2]],
      });
      void enqueueGumballDispatch(async () => {
        // One absolute start→end tick onto the transform scratch, then a single commit.
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

  const handlePointerDown = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (event.button !== 0) return;
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
      setMarqueePath([toLocalPoint(event)]);
      event.currentTarget.setPointerCapture?.(event.pointerId);
    },
    [dispatch, node.surfaceId, paintMode, selection.engagementSessionActive, toLocalPoint],
  );

  const handlePointerMove = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
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
      setMarqueeModifiers({ shiftKey: event.shiftKey, ctrlKey: event.ctrlKey, metaKey: event.metaKey });
      setMarqueePath((path) => [...path, toLocalPoint(event)]);
    },
    [dispatch, marqueeDown, node.surfaceId, selection.engagementSessionActive, toLocalPoint],
  );

  const finalizeMarqueeSelection = useCallback(() => {
    if (!marqueeDragActiveRef.current || marqueeFinalizeOnceRef.current) return;
    const preview = marqueePreviewRef.current;
    if (!preview.mergedInstanceIds?.length && !preview.mergedComponentIds?.length) return;
    marqueeFinalizeOnceRef.current = true;
    if (preview.mergedInstanceIds?.length) {
      setMarqueeCommitHold({ mergedComponentIds: null, mergedInstanceIds: preview.mergedInstanceIds });
      void Promise.resolve(dispatch("worldSelect", { ids: preview.mergedInstanceIds, merge: "replace" })).finally(() => {
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
  }, [dispatch, selection.activeObjectId, selectionMode]);

  const handlePointerUp = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (event.currentTarget.hasPointerCapture?.(event.pointerId)) {
        event.currentTarget.releasePointerCapture(event.pointerId);
      }
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
      if (paintStrokeActive) {
        dispatch("paintStrokeEnd");
        setPaintStrokeActive(false);
      }
      setMarqueePath([]);
      setVortexPointerArm(null);
      if (connectDropConsumedRef.current) {
        connectDropConsumedRef.current = false;
      } else {
        handleConnectDragCancel();
      }
    },
    [faceDragSession, finalizeMarqueeSelection, handleConnectDragCancel, node.surfaceId, paintStrokeActive],
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
    (event: MouseEvent) => {
      // 🧹️ Consume the post-marquee suppress flag so a stale `true` cannot permanently no-operation background deselect.
      if (wasMarqueeDragRef.current) {
        wasMarqueeDragRef.current = false;
        return;
      }
      if (selection.engagementSessionActive || paintMode) return;
      if (interaction.suggestionMenu?.open) {
        handleSuggestionClose();
      }
      dispatch("worldPick", { granularity: selectionMode, id: null, merge: instanceMergeArg(resolveWorldSelectionMergeMode(selection.selectionMergeMode, event, persistentSelectionMode)) });
    },
    [dispatch, handleSuggestionClose, interaction.suggestionMenu?.open, paintMode, selection.engagementSessionActive, selection.selectionMergeMode, selectionMode],
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
      className="semio-world-3d-host relative h-full min-h-[24rem] w-full"
      data-surface-id={node.surfaceId}
      data-orbit-view-gizmo=""
      data-puzzle3d-fixture-drag-active={catalogueDropPreview ? "" : undefined}
      onContextMenu={(event) => {
        if (event.altKey || !requestContextMenu) return;
        const target = resolveWorldContextMenuTarget(interaction, selection);
        event.preventDefault();
        event.stopPropagation();
        void (async () => {
          if (target) dispatch("contextMenuAt", { kind: target.kind, id: target.id });
          const selectionGroups = [];
          if ((selection.ids?.length ?? 0) > 0) selectionGroups.push({ domain: "object", ids: [...(selection.ids ?? [])] });
          if ((selection.componentIds?.length ?? 0) > 0) selectionGroups.push({ domain: "feature", ids: (selection.componentIds ?? []).map(String) });
          const hits = target ? [{ domain: target.kind, id: target.id }] : [];
          const items = await openSurfaceContextMenu(
            requestContextMenu,
            {
              menu: { id: "world3d" },
              surface: { surfaceId: node.surfaceId, kind: "world3d", hits, selection: selectionGroups },
              windowInstanceId: windowInstanceId ?? undefined,
              point: { x: event.clientX, y: event.clientY },
            },
            mapWorldContextMenuSpecs,
            shellContextMenuFallback,
          );
          setContextMenu({ x: event.clientX, y: event.clientY, items });
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
            <WorldOrbitProjectionSwitchPane spec={worldProjectionSpec} onSpecChange={handleProjectionKindChange} />
            {computing ? (
              <div className={cn("pointer-events-none absolute right-3 top-3 flex items-center gap-2 rounded px-2 py-1 text-xs shadow-sm", glassClass)} data-level="pane" role="status" aria-busy="true">
                <Spinner size="small" />
                <span>{shellLabel("ui.common.loading")}</span>
              </div>
            ) : null}
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
            {windowInstanceId ? (
              <IntroductionWorldResolverBridge windowInstanceId={windowInstanceId} vortices={vortices} instances={instances} attractions={attractions} />
            ) : null}
            {brushMeshUrls.map((url) => (
              <Suspense key={url} fallback={null}>
                <BrushMeshRegistrar url={url} onRegister={handleRegisterBrushMesh} />
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
            {brushPreview ? <BrushPreviewGhost preview={brushPreview} meshes={meshes} palette={meshStylePalette} /> : null}
            {!brushPreview && catalogueDropPreview ? <CatalogueDropGhost preview={catalogueDropPreview} meshes={meshes} palette={meshStylePalette} /> : null}
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
            {volumeBrushMode ? <WorldVoxelGroundPlane gridFactor={interaction.gridFactor ?? DEFAULT_LOD_GRID_FACTOR} onHover={setVoxelHoverOrigin} onPlace={handleVoxelPlace} /> : null}
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
      {marqueeDragActive && marqueeStart && marqueeEnd ? (
        method === "lasso" ? (
          <SelectionMarquee coverage={marqueeCoverage} shape="polygon" points={marqueePath} />
        ) : (
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
        )
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
