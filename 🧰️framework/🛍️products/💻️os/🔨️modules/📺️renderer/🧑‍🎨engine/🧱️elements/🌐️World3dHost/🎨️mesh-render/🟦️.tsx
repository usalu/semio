/** 🎨️ Mesh paint styles, instance celebration, the auto-fit and frame-diagnostics components, the projection content frame, mesh residency and geometry, appearance and GLB bounds.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | World3dHost}, which re-exports every public name below so import specifiers stay unchanged.
 */

import React, {
  useCallback,
  useEffect,
  useLayoutEffect,
  useMemo,
  useRef,
  useState,
  useSyncExternalStore,
  type ComponentProps
} from "react";
import {
  clearColorResolveCache,
  resolveColorHex,
  semanticVar,
  themeColorVar,
  tokenVar
} from "@semio-tech/ui-styling";
// #region 🔌️Adapters
import {
  CELEBRATE_STAMP_DURATION_MS,
  sceneHostPort,
  useCanvasAppearanceSync,
  useShellScopeOptional,
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
  ShaderMaterial,
  SphereGeometry,
  Texture,
  ClampToEdgeWrapping,
  RepeatWrapping,
  MirroredRepeatWrapping,
  NearestFilter,
  LinearFilter,
  NearestMipmapNearestFilter,
  LinearMipmapNearestFilter,
  NearestMipmapLinearFilter,
  LinearMipmapLinearFilter,
  TextureLoader,
  FrontSide,
  SRGBColorSpace,
  NoColorSpace,
  TorusGeometry,
  Vector3
} from "@semio-tech/ui-react";
import { TOOL_RUN_PROVISIONAL_PAINT } from "../⏯️tool-run-trace/🟦️.tsx";
import {
  DEFAULT_LOD_GRID_FACTOR,
  DEFAULT_MANUAL_LOD,
  frameWorldProjectionPose,
  GLB_MESH_FRAME_ROTATION_X,
  lodFromCameraDistance,
  lodGridStepWorld,
  lodOrbitDistanceForCamera,
  WORLD_MESH_OUTLINE_USER_DATA_KEY,
  WORLD_REFERENCE_DEFAULT_WIDTH,
  worldProjectionModeFov,
  worldSceneContentBoundsKey,
  type WorldCameraState,
  type WorldProjectionSpec,
  type WorldSceneContentBounds
} from "@semio-tech/infinite-world-r3f";
// #endregion 🔌️Adapters
import { runtimeDiagnosticsEnabled } from "../../🏛️ShellHost/🔌️extension-runtime/🟦️.ts";
import {
  fitCameraFromBounds,
  world3dAutoFitOwed,
  world3dBoundsRadius,
  type WorldInstanceRecord,
  type WorldLodRecord,
  type WorldMeshAttribute,
  type WorldMeshData,
  type WorldMeshRecord,
  type WorldParsedCameraState,
  type WorldReferenceRecord
} from "../🔭️scene-schema/🟦️.ts";

export const { useFrame, useLoader, useThree } = sceneHostPort.fiber;

//#region 🔖️World3dHost
//#region WorldSceneParsing
//#region WorldMeshPaint
/** 🎨️ Mesh style kinds, in {@link resolveMeshStyle} priority order (highest first). */
export type MeshStyleKind = "disabled" | "provisional" | "celebrated" | "selected" | "highlighted" | "hovered" | "neutral";

type MeshStyleColors = {
  readonly meshColor: string;
  readonly lineColor: string;
  readonly emissiveIntensity: number;
  readonly opacity: number;
};

export type MeshStylePalette = Readonly<Record<MeshStyleKind, MeshStyleColors>>;

/** 🎨️ CSS-expression paint spec per style kind, ported from the premigration puzzle 3d paint table. */
const MESH_STYLE_PAINT: Readonly<Record<MeshStyleKind, { readonly fill: string; readonly line: string; readonly emissiveIntensity: number; readonly opacity: number }>> = {
  neutral: { fill: "var(--panel)", line: semanticVar("border-normal-color"), emissiveIntensity: 0, opacity: 1 },
  hovered: { fill: semanticVar("hover-interactive-fill"), line: semanticVar("border-emphasized-color"), emissiveIntensity: 0.08, opacity: 1 },
  selected: { fill: tokenVar("primary"), line: tokenVar("primary"), emissiveIntensity: 0.35, opacity: 1 },
  highlighted: { fill: tokenVar("secondary"), line: tokenVar("secondary"), emissiveIntensity: 0.2, opacity: 1 },
  // 🎉️ Transient drop/completion paint — solid fallback for lines; shaded meshes use {@link CelebratingConicMaterial}.
  // 🟩️ The `provisional` style token: a running tool's not-yet-finalized placement (dashed outline via `ToolRunProvisionalOutline`).
  provisional: { fill: TOOL_RUN_PROVISIONAL_PAINT.fill, line: TOOL_RUN_PROVISIONAL_PAINT.line, emissiveIntensity: 0.2, opacity: TOOL_RUN_PROVISIONAL_PAINT.opacity },
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

export function useMeshStylePalette(): MeshStylePalette {
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

/** 🎨️ Resolves the effective style kind for an instance/component, priority: disabled → provisional → celebrated → selected → highlighted → hovered → neutral. */
export function resolveMeshStyle(state: {
  readonly disabled?: boolean;
  readonly provisional?: boolean;
  readonly celebrating?: boolean;
  readonly selected?: boolean;
  readonly highlighted?: boolean;
  readonly hovered?: boolean;
}): MeshStyleKind {
  if (state.disabled) return "disabled";
  if (state.provisional) return "provisional";
  if (state.celebrating) return "celebrated";
  if (state.selected) return "selected";
  if (state.highlighted) return "highlighted";
  if (state.hovered) return "hovered";
  return "neutral";
}

/** 🎨️ Resolves live group-selection preview paint: the new selection is active, while only objects exiting the old selection are highlighted. */
export function resolveMeshSelectionPreviewStyle(
  instance: Pick<WorldInstanceRecord, "disabled" | "provisional" | "selected" | "highlighted" | "hovered"> & { readonly celebrating?: boolean },
  previewSelected?: boolean,
): MeshStyleKind {
  const selectionExited = previewSelected === false && instance.selected === true;
  return resolveMeshStyle({
    disabled: instance.disabled,
    provisional: instance.provisional,
    celebrating: instance.celebrating,
    selected: previewSelected ?? instance.selected,
    highlighted: selectionExited || instance.highlighted,
    hovered: instance.hovered,
  });
}

/** 🔗️ An instance's effective highlight: the host's own catalog-kind hover OR the flag the guest stamped on the record
 * (an object the open time-travel draft references) — the host's hover never erases what the guest asked to show. */
export function worldInstanceHighlighted(hostHighlighted: boolean, instance: Pick<WorldInstanceRecord, "highlighted">): boolean {
  return hostHighlighted || instance.highlighted === true;
}

/** 🎨️ Slim alias over {@link MeshStylePalette} for call sites that only need the four legacy semantic colors (face/edge/vertex component overlays, markers). */
export type SemanticColors = {
  readonly mesh: string;
  readonly edge: string;
  readonly select: string;
  readonly hover: string;
  readonly edgeHover: string;
};

/** 🎨️ Maps mesh style palette fills/lines onto World3d semantic overlay colors — edge hover uses line paint so coplanar edges stay distinct from face hover fill.
 *
 * 🖱️ Component hover (a face/edge/vertex under the pointer) paints with the `highlighted` style — the
 * theme's SECONDARY token — not the `hovered` instance style: that one resolves to
 * `--hover-interactive-fill` = `--border-normal-color`, a grey a half-transparent overlay over the grey
 * mesh could not be told from the mesh at all (lowpoly face/vertex hover, 2026-09-18). Primary stays
 * selection, secondary is hover, and both are theme-owned. */
export function semanticColorsFromPalette(palette: MeshStylePalette): SemanticColors {
  return {
    mesh: palette.neutral.meshColor,
    edge: palette.neutral.lineColor,
    select: palette.selected.lineColor,
    hover: palette.highlighted.meshColor,
    edgeHover: palette.highlighted.lineColor,
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
export function useCelebratingWorldInstanceIds(): ReadonlySet<string> {
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
export function celebrateConicAngleRadians(): number {
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

export function createCelebratingConicMaterial(opacity = 1): ShaderMaterial {
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

/** 🎯️ Fits the orbit camera to the producer's published bounds (or, absent those, the scene
 * group's own AABB) once per fit key, preserving the view direction. */
export function WorldAutoFit({
  groupRef,
  fitKey,
  padding,
  camera,
  bounds,
  userMoved,
  onFitted,
}: {
  readonly groupRef: React.RefObject<Group | null>;
  readonly fitKey: string;
  readonly padding: number;
  readonly camera: WorldParsedCameraState;
  readonly bounds: readonly [readonly number[], readonly number[]] | null;
  readonly userMoved: boolean;
  readonly onFitted: (state: WorldCameraState) => void;
}): null {
  const { camera: sceneCamera, controls, invalidate } = useThree();
  const appliedKeyRef = useRef("");
  const targetScratch = useMemo(() => new Vector3(), []);
  useFrame(() => {
    if (!sceneCamera) return;
    const group = groupRef.current;
    if (!group) return;
    if (!world3dAutoFitOwed(appliedKeyRef.current, fitKey, userMoved)) return;
    let center: readonly [number, number, number];
    let radius: number;
    if (bounds) {
      const [minimum, maximum] = bounds;
      center = [((minimum[0] ?? 0) + (maximum[0] ?? 0)) * 0.5, ((minimum[1] ?? 0) + (maximum[1] ?? 0)) * 0.5, ((minimum[2] ?? 0) + (maximum[2] ?? 0)) * 0.5];
      radius = world3dBoundsRadius(minimum, maximum);
    } else {
      const box = new Box3().setFromObject(group);
      if (box.isEmpty()) return;
      const boxCenter = box.getCenter(new Vector3());
      const boxMinimum = box.min;
      const boxMaximum = box.max;
      center = [boxCenter.x, boxCenter.y, boxCenter.z];
      radius = world3dBoundsRadius([boxMinimum.x, boxMinimum.y, boxMinimum.z], [boxMaximum.x, boxMaximum.y, boxMaximum.z]);
    }
    if (radius <= 0) return;
    appliedKeyRef.current = fitKey;
    const aspect = (sceneCamera as { aspect?: number }).aspect ?? 1;
    const fitted = fitCameraFromBounds(center, radius, camera, padding, aspect);
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

type WorldAcceptedFrameDiagnosticsProps = {
  readonly surfaceId: string;
  readonly windowInstanceId?: string | null;
  readonly projection: WorldProjectionSpec;
  readonly cameraTarget: readonly [number, number, number];
  readonly contentBounds: WorldSceneContentBounds | null;
  readonly references: readonly WorldReferenceRecord[];
  readonly lod: WorldLodRecord;
};

/** 🩺️ Emits one deduplicated, diagnostics-gated receipt from the actual R3F camera after
 * projection/framing and LOD frame runners have accepted their inputs. */
export function WorldAcceptedFrameDiagnostics(props: WorldAcceptedFrameDiagnosticsProps): null {
  const { camera, controls, size } = useThree();
  const fallbackTarget = useMemo(() => new Vector3(), []);
  const lastReceiptRef = useRef("");
  useFrame(() => {
    if (!runtimeDiagnosticsEnabled()) return;
    const target = (controls as { readonly target?: Vector3 } | null)?.target ?? fallbackTarget.set(...props.cameraTarget);
    const orbitDistance = camera.position.distanceTo(target);
    const lodDistance = lodOrbitDistanceForCamera(camera, orbitDistance, size.height);
    const automaticLod = props.lod.automaticLod ?? true;
    const depthVariableLod = props.lod.depthVariableLod ?? false;
    const manualLod = props.lod.manualLod ?? DEFAULT_MANUAL_LOD;
    const sceneLod = automaticLod || depthVariableLod ? lodFromCameraDistance(lodDistance, 100) : manualLod;
    const gridFactor = props.lod.gridFactor ?? DEFAULT_LOD_GRID_FACTOR;
    const receipt = {
      surfaceId: props.surfaceId,
      windowInstanceId: props.windowInstanceId ?? null,
      projection: props.projection,
      camera: {
        kind: camera.type,
        position: camera.position.toArray(),
        target: target.toArray(),
        up: camera.up.toArray(),
        quaternion: camera.quaternion.toArray(),
        fov: "fov" in camera && typeof camera.fov === "number" ? camera.fov : null,
        zoom: "zoom" in camera && typeof camera.zoom === "number" ? camera.zoom : 1,
        near: "near" in camera && typeof camera.near === "number" ? camera.near : null,
        far: "far" in camera && typeof camera.far === "number" ? camera.far : null,
      },
      viewport: { width: size.width, height: size.height, aspect: size.height > 0 ? size.width / size.height : null },
      contentBounds: props.contentBounds,
      references: props.references.filter((reference) => !reference.hidden).map((reference) => ({ id: reference.id, origin: reference.origin, widthWorld: reference.widthWorld ?? WORLD_REFERENCE_DEFAULT_WIDTH })),
      grid: { automaticLod, depthVariableLod, distanceReference: 100, orbitDistance, lodDistance, sceneLod, gridFactor, gridStepWorld: lodGridStepWorld(sceneLod, gridFactor) },
    };
    const encoded = JSON.stringify(receipt);
    if (encoded === lastReceiptRef.current) return;
    lastReceiptRef.current = encoded;
    console.info(`[TRACE] react-world-frame ${encoded}`);
  });
  return null;
}

/** 📷️ Viewport-aware reframe for a projection seed so orthographic panes fit content — re-runs whenever
 * {@link worldSceneContentBoundsKey} changes (a running tool grows the scene) until the host stops enabling it
 * (user-owned orbit/pan/zoom). */
export function WorldProjectionContentFrame(props: {
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
    const framed: WorldParsedCameraState = { ...pose, fov: worldProjectionModeFov(props.spec) ?? props.fov, explicitProjection: true, projectionFrame: "content" };
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

/** 🥽️ One built-in mesh kind's geometry, built from the engine's own primitives. The placement and
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

/** 🥽️ Resolves a `{ id, kind }` scene mesh reference into the buffers every downstream path
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

function resolveMeshRecord(record: WorldMeshRecord): WorldMeshRecord {
  return record.data || !record.kind ? record : { ...record, data: meshDataFromKind(record.kind) };
}

function parseMeshes(meshesJson: string): WorldMeshRecord[] {
  try {
    const parsed = JSON.parse(meshesJson);
    if (!Array.isArray(parsed)) return [];
    return (parsed as WorldMeshRecord[]).map(resolveMeshRecord);
  } catch {
    return [];
  }
}

/** 🔪️ Splits a JSON array's TEXT into its element texts without parsing any of them — a
 * balanced scan over strings/escapes/nesting. `null` means "this is not a flat JSON array I can
 * account for", which every caller answers by falling back to a whole-document parse.
 *
 * 🪪️ This exists so mesh identity can be decided on the wire text: `meshesJson` is one string, and
 * re-parsing it hands back brand-new record objects for meshes whose bytes never moved. */
export function splitJsonArrayElements(json: string): readonly string[] | null {
  const text = json.trim();
  if (!text.startsWith("[") || !text.endsWith("]")) return null;
  const elements: string[] = [];
  let depth = 0;
  let inString = false;
  let escaped = false;
  let start = -1;
  for (let index = 1; index < text.length - 1; index += 1) {
    const character = text[index]!;
    if (inString) {
      if (escaped) escaped = false;
      else if (character === "\\") escaped = true;
      else if (character === '"') inString = false;
      continue;
    }
    if (character === '"') {
      if (start < 0) start = index;
      inString = true;
      continue;
    }
    if (character === "{" || character === "[") {
      if (start < 0) start = index;
      depth += 1;
      continue;
    }
    if (character === "}" || character === "]") {
      depth -= 1;
      if (depth < 0) return null;
      continue;
    }
    if (character === "," && depth === 0) {
      if (start < 0) return null;
      elements.push(text.slice(start, index).trim());
      start = -1;
      continue;
    }
    if (start < 0 && character.trim() !== "") start = index;
  }
  if (inString || depth !== 0) return null;
  if (start >= 0) elements.push(text.slice(start, text.length - 1).trim());
  else if (elements.length > 0) return null;
  return elements;
}

/** 🧾️ One consumer's retained mesh set and the element texts it was decoded from — the texts are the
 * identity key, so a refresh that republishes an unchanged mesh hands back the SAME record object. */
export type WorldMeshResidencyV1 = {
  readonly sources: readonly string[];
  readonly records: readonly WorldMeshRecord[];
};

/** 🧊️ Advances a retained mesh set to `meshesJson`, keeping the object IDENTITY of every mesh
 * whose wire text is unchanged. That identity is what lets the instanced-mesh layer rebuild the
 * `BufferGeometry` of exactly the meshes that moved instead of all of them: before this, a 3-mesh
 * surface re-tessellated all three whenever one upstream node re-evaluated, because the memo boundary
 * was the whole `meshesJson` string (`📓️react-perf-ceilings-audit-2026-09-14.md` §2c).
 *
 * 🪃️ A payload this cannot split is not an error — it falls back to the whole-document parse, which
 * is always correct and merely rebuilds everything, exactly as the instance-delta lane's own
 * `advanceWorldInstanceResidency` fallback does. */
export function advanceWorldMeshResidency(previous: WorldMeshResidencyV1 | null, meshesJson: string): WorldMeshResidencyV1 {
  const sources = splitJsonArrayElements(meshesJson);
  if (sources === null) return { sources: [], records: parseMeshes(meshesJson) };
  const retained = new Map<string, WorldMeshRecord>();
  const previousSources = previous?.sources ?? [];
  for (let index = 0; index < previousSources.length; index += 1) {
    const source = previousSources[index]!;
    const record = previous?.records[index];
    if (record && !retained.has(source)) retained.set(source, record);
  }
  try {
    return { sources, records: sources.map((source) => retained.get(source) ?? resolveMeshRecord(JSON.parse(source) as WorldMeshRecord)) };
  } catch {
    return { sources: [], records: parseMeshes(meshesJson) };
  }
}

function surfaceSample(attribute: WorldMeshAttribute, mesh: WorldMeshData, corner: number): unknown {
  const vertex = mesh.indices[corner]!;
  const domain = attribute.domain === "vertex" ? vertex : attribute.domain === "corner" ? corner : attribute.domain === "face" ? Math.floor(corner / 3) : -1;
  if (domain < 0) throw new Error("a directed edge attribute cannot shade a triangle");
  const sample = attribute.indices?.[domain] ?? (attribute.indices ? -1 : domain);
  if (!Number.isInteger(sample) || sample < 0 || sample >= attribute.values.length) throw new Error("surface attribute references an invalid sample");
  return attribute.values[sample];
}

function surfaceTuple(value: unknown, width: number, name: string): number[] {
  if (!Array.isArray(value) || value.length !== width || value.some(value => typeof value !== "number" || !Number.isFinite(value))) throw new Error(`${name} requires a finite ${width}-component tuple`);
  return value as number[];
}

function surfaceAttributes(mesh: WorldMeshData, semantic: WorldMeshAttribute["semantic"]): [string, WorldMeshAttribute][] {
  return Object.entries(mesh.attributes ?? {}).filter(([, attribute]) => attribute.semantic === semantic).sort(([a],[b]) => a < b ? -1 : a > b ? 1 : 0);
}

/** 🎨️ Builds authored draw samples while retaining triangle order and original picking identities. */
export function geometryFromMesh(mesh: WorldMeshData) {
  if (mesh.positions.length % 3 || mesh.indices.length % 3 || mesh.positions.some(value => !Number.isFinite(value)) || mesh.indices.some(value => !Number.isInteger(value) || value < 0 || value >= mesh.positions.length / 3)) throw new Error("surface mesh has invalid positions or triangle indices");
  const normal = surfaceAttributes(mesh,"normal")[0]?.[1];
  const color = surfaceAttributes(mesh,"color")[0]?.[1];
  const tangent=mesh.attributes?.tangent;
  const uv = surfaceAttributes(mesh,"uv");
  const channels = [normal,color,tangent,...uv.map(([,attribute])=>attribute)].filter((value):value is WorldMeshAttribute=>Boolean(value));
  for(const attribute of channels){
    const count=attribute.domain === "vertex" ? mesh.positions.length/3 : attribute.domain === "face" ? mesh.indices.length/3 : mesh.indices.length;
    if(!["vertex","corner","face"].includes(attribute.domain)||(attribute.indices?.length ?? attribute.values.length)!==count||attribute.indices?.some(index=>!Number.isInteger(index)||index<0||index>=attribute.values.length))throw new Error("surface attribute cardinality differs from its domain");
  }
  const geometry = new BufferGeometry();
  const expanded = channels.some(attribute=>attribute.domain !== "vertex");
  const corners = expanded ? mesh.indices.length : mesh.positions.length / 3;
  const positions = expanded ? mesh.indices.flatMap(vertex=>mesh.positions.slice(vertex*3,vertex*3+3)) : mesh.positions;
  geometry.setAttribute("position",new BufferAttribute(new Float32Array(positions),3));
  if (!expanded && mesh.indices.length) geometry.setIndex([...mesh.indices]);
  const bind = (name:string,attribute:WorldMeshAttribute|undefined,width:number,fallback:readonly number[]|undefined)=>{
    if (attribute) {
      const values:number[]=[];
      for(let index=0;index<corners;index++){
        const value=expanded ? surfaceSample(attribute,mesh,index) : attribute.values[attribute.indices?.[index] ?? index];
        const tuple=surfaceTuple(value,width,name);if(name === "normal" && tuple.every(value=>value === 0))throw new Error("surface normal cannot be zero");values.push(...tuple);
      }
      geometry.setAttribute(name,new BufferAttribute(new Float32Array(values),width));
    } else if (fallback?.length) {
      if (fallback.length !== mesh.positions.length/3*width) throw new Error(`${name} buffer cardinality differs from the vertex count`);
      const values=expanded ? mesh.indices.flatMap(vertex=>fallback.slice(vertex*width,vertex*width+width)) : fallback;
      geometry.setAttribute(name,new BufferAttribute(new Float32Array(values),width));
    }
  };
  try {
    bind("normal",normal,3,mesh.normals);
    bind("color",color,4,mesh.colors);
    bind("tangent",tangent,4,undefined);
    if(tangent){const values=geometry.getAttribute("tangent");for(let i=0;i<values.count;i++){if(values.getW(i)!==1 && values.getW(i)!==-1 || values.getX(i)===0 && values.getY(i)===0 && values.getZ(i)===0)throw new Error("surface tangent requires a nonzero direction and signed handedness");}}
    if (uv.length) uv.forEach(([name,attribute],index)=>{const declared=/^uv(\d+)?$/.exec(name);const set=declared ? Number(declared[1] ?? 0) : index;bind(set ? `uv${set}` : "uv",attribute,2,undefined);});
    else bind("uv",undefined,2,mesh.uvs);
    if (geometry.hasAttribute("uv") && !geometry.hasAttribute("uv1")) geometry.setAttribute("uv1",geometry.getAttribute("uv").clone());
    if (!geometry.hasAttribute("normal")) geometry.computeVertexNormals();
    return geometry;
  } catch(error) {geometry.dispose();throw error;}
}

export type VertexPickData = {
  readonly geometry: BufferGeometry;
  readonly vertexIds: readonly number[];
};

export function buildVertexPickData(mesh: WorldMeshData): VertexPickData | null {
  if (!mesh.vertexIds?.length) return null;
  const positions: number[] = [];
  const vertexIds: number[] = [];
  const emitted = new Set<number>();
  for (let index = 0; index < mesh.vertexIds.length; index += 1) {
    const id = mesh.vertexIds[index]!;
    if (id === 0xffff_ffff || emitted.has(id)) continue;
    emitted.add(id);
    vertexIds.push(id);
    positions.push(mesh.positions[index * 3]!, mesh.positions[index * 3 + 1]!, mesh.positions[index * 3 + 2]!);
  }
  if (!positions.length) return null;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(positions), 3));
  return { geometry, vertexIds };
}

export type WorldMeshAppearance = {
  readonly materials: MeshStandardMaterial[];
  readonly textures: Map<string, Texture>;
  readonly urls: Set<string>;
  readonly listeners: Set<()=>void>;
  pending: number;
  fault?: string;
  closed: boolean;
};

/** 🖍️ Applies owned material coefficients using glTF alpha and color-space conventions. */
export function worldSurfaceMaterial(name:string,source:Record<string,unknown>,vertexColors:boolean):MeshStandardMaterial {
  const base=surfaceTuple(source.baseColor ?? [1,1,1,1],4,"baseColor");
  const emissive=surfaceTuple(source.emissive ?? [0,0,0],3,"emissive");
  const alpha=source.alphaMode ?? "OPAQUE";
  if (!["OPAQUE","MASK","BLEND"].includes(String(alpha))) throw new Error("surface material has an invalid alpha mode");
  const coefficient=(name:string,defaultValue:number,maximum=1)=>{const value=source[name]??defaultValue;if(typeof value!=="number"||!Number.isFinite(Math.fround(value))||value<0||value>maximum)throw new Error(`surface material ${name} requires a coefficient in [0,${maximum}]`);return value;};
  if(base.some(value=>value<0||value>1)||emissive.some(value=>value<0))throw new Error("surface color coefficients are outside their domain");
  const normalScale=surfaceTuple(Array.isArray(source.normalScale)?source.normalScale:[source.normalScale ?? 1,source.normalScale ?? 1],2,"normalScale");if(normalScale.some(value=>Math.abs(value)>3.4028234663852886e38))throw new Error("surface material normalScale requires finite coefficients");
  const occlusionStrength=coefficient("occlusionStrength",1);
  const coordinates=source.textureCoordinates;if(coordinates!==undefined && (typeof coordinates!=="object"||coordinates===null||Array.isArray(coordinates)||Object.entries(coordinates).some(([key,value])=>!["baseColorTexture","metallicRoughnessTexture","normalTexture","occlusionTexture","emissiveTexture"].includes(key)||!Number.isInteger(value)||(value as number)<0||(value as number)>63)))throw new Error("surface material has invalid UV selectors");
  const material=new MeshStandardMaterial({name,vertexColors,metalness:coefficient("metallic",1),roughness:coefficient("roughness",1),opacity:alpha === "OPAQUE" ? 1 : base[3],transparent:alpha === "BLEND",alphaTest:alpha === "MASK" ? coefficient("alphaCutoff",.5,3.4028234663852886e38) : 0,depthWrite:alpha !== "BLEND",side:source.doubleSided === true ? DoubleSide : FrontSide});
  material.color.setRGB(base[0]!,base[1]!,base[2]!);
  material.emissive.setRGB(emissive[0]!,emissive[1]!,emissive[2]!);
  material.normalScale.set(normalScale[0]!,normalScale[1]!);material.aoMapIntensity=occlusionStrength;
  const extra=[...new Set(Object.values((coordinates ?? {}) as Record<string,number>).filter(set=>set>3))].sort((a,b)=>a-b);
  if(extra.length){material.onBeforeCompile=shader=>{shader.vertexShader=extra.map(set=>`attribute vec2 uv${set};`).join("\n")+"\n"+shader.vertexShader;};material.customProgramCacheKey=()=>`surface-uv:${extra.join(",")}`;}
  return material;
}

function buildWorldMeshAppearance(mesh:WorldMeshData,geometry:BufferGeometry):WorldMeshAppearance|null {
  const assignments=surfaceAttributes(mesh,"material")[0]?.[1];
  if (!assignments && !Object.keys(mesh.materials ?? {}).length) return null;
  const appearance:WorldMeshAppearance={materials:[],textures:new Map(),urls:new Set(),listeners:new Set(),pending:0,closed:false};
  const names:string[]=[];
  try {
    for(let corner=0;corner<mesh.indices.length;corner+=3){
      const name=assignments ? surfaceSample(assignments,mesh,corner) : Object.keys(mesh.materials!)[0];
      if (typeof name !== "string" || !mesh.materials?.[name]) throw new Error("face material references an unowned material");
      let index=names.indexOf(name);
      if(index<0){index=names.length;names.push(name);appearance.materials.push(worldSurfaceMaterial(name,mesh.materials[name]!,geometry.hasAttribute("color")));}
      const previous=geometry.groups.at(-1);
      if(previous?.materialIndex===index) previous.count+=3;
      else geometry.addGroup(corner,3,index);
    }
    for(const [index,name] of names.entries()){
      const source=mesh.materials![name]!;const material=appearance.materials[index]!;
      for(const [field,roles,srgb] of [["baseColorTexture",["map"],true],["metallicRoughnessTexture",["metalnessMap","roughnessMap"],false],["normalTexture",["normalMap"],false],["occlusionTexture",["aoMap"],false],["emissiveTexture",["emissiveMap"],true]] as const){
        const id=source[field];if(id===undefined)continue;
        if(typeof id!=="string"||!mesh.textures?.[id])throw new Error(`${field} references an unowned texture`);
        const coordinates=source.textureCoordinates as Record<string,number>|undefined;const set=coordinates?.[field] ?? 0;
        if(!geometry.hasAttribute(set ? `uv${set}` : "uv"))throw new Error(`${field} references an absent UV set`);
        const sampler=(source.textureSamplers as Record<string,Record<string,number>>|undefined)?.[field] ?? {};
        if(Object.entries(sampler).some(([field,value])=>!({wrapS:[33071,33648,10497],wrapT:[33071,33648,10497],magFilter:[9728,9729],minFilter:[9728,9729,9984,9985,9986,9987]} as Record<string,number[]>)[field]?.includes(value)))throw new Error("surface material texture sampler is invalid");
        const owned=mesh.textures[id]!;const key=`${id}:${srgb ? "srgb" : "linear"}:${set}:${sampler.wrapS ?? 10497}:${sampler.wrapT ?? 10497}:${sampler.magFilter ?? 9729}:${sampler.minFilter ?? 9729}`;
        let texture=appearance.textures.get(key);
        if(!texture){
          if(!["image/png","image/jpeg"].includes(owned.mime)||owned.bytes.length>16_000_000||owned.bytes.some(value=>!Number.isInteger(value)||value<0||value>255))throw new Error("surface texture requires bounded owned PNG/JPEG bytes");
          texture=new Texture();texture.colorSpace=srgb ? SRGBColorSpace : NoColorSpace;texture.channel=set;texture.flipY=false;
          const wraps={33071:ClampToEdgeWrapping,33648:MirroredRepeatWrapping,10497:RepeatWrapping};const filters={9728:NearestFilter,9729:LinearFilter,9984:NearestMipmapNearestFilter,9985:LinearMipmapNearestFilter,9986:NearestMipmapLinearFilter,9987:LinearMipmapLinearFilter};
          texture.wrapS=wraps[(sampler.wrapS ?? 10497) as keyof typeof wraps];texture.wrapT=wraps[(sampler.wrapT ?? 10497) as keyof typeof wraps];texture.magFilter=filters[(sampler.magFilter ?? 9729) as 9728|9729];texture.minFilter=filters[(sampler.minFilter ?? 9729) as keyof typeof filters];appearance.textures.set(key,texture);
          if(typeof URL.createObjectURL==="function"){
            const url=URL.createObjectURL(new Blob([Uint8Array.from(owned.bytes)],{type:owned.mime}));appearance.urls.add(url);appearance.pending++;
            const target=texture;
            new TextureLoader().load(url,loaded=>{URL.revokeObjectURL(url);appearance.urls.delete(url);if(!appearance.closed){target.image=loaded.image;target.needsUpdate=true;appearance.pending--;for(const listener of appearance.listeners)listener();}loaded.dispose();},undefined,()=>{URL.revokeObjectURL(url);appearance.urls.delete(url);if(!appearance.closed){appearance.pending--;appearance.fault=`texture ${id} could not be decoded`;for(const listener of appearance.listeners)listener();}});
          }
        }
        for(const role of roles)material[role]=texture;
      }
    }
    return appearance;
  }catch(error){disposeWorldMeshAppearance(appearance);throw error;}
}

export function disposeWorldMeshAppearance(appearance:WorldMeshAppearance):void {
  if(appearance.closed)return;appearance.closed=true;for(const listener of appearance.listeners)listener();appearance.listeners.clear();
  for(const material of appearance.materials)material.dispose();
  for(const texture of appearance.textures.values())texture.dispose();
  for(const url of appearance.urls)URL.revokeObjectURL(url);appearance.urls.clear();
}

/** 🧊️ One existing mesh visual lease owns every draw buffer, material and decoded texture. */
export type MeshVisuals = { readonly record:WorldMeshRecord;readonly geometry:BufferGeometry|null;readonly border:EdgesGeometry|null;readonly vertexPick:VertexPickData|null;readonly edge:BufferGeometry|null;readonly appearance:WorldMeshAppearance|null;readonly fault?:string;closed:boolean; };

/** 🧊️ Builds authored visuals at the existing retained record boundary. */
export function buildMeshVisuals(record:WorldMeshRecord):MeshVisuals {
  if(!record.data)return {record,geometry:null,border:null,vertexPick:null,edge:null,appearance:null,closed:false};
  const geometry=geometryFromMesh(record.data);
  if(record.data.heatmap)geometry.userData.world3dHeatmap=true;
  let border:EdgesGeometry|null=null;let vertexPick:VertexPickData|null=null;let edge:BufferGeometry|null=null;let appearance:WorldMeshAppearance|null=null;
  try {appearance=buildWorldMeshAppearance(record.data,geometry);border=new EdgesGeometry(geometry);vertexPick=buildVertexPickData(record.data);edge=buildEdgeGeometry(record.data);return {record,geometry,border,vertexPick,edge,appearance,closed:false};}
  catch(error){geometry.dispose();border?.dispose();vertexPick?.geometry.dispose();edge?.dispose();if(appearance)disposeWorldMeshAppearance(appearance);throw error;}
}

/** 🧹️ Retires each visual-owned resource exactly once; loader-shared GLB assets remain owned by their loader. */
export function disposeMeshVisuals(visuals:MeshVisuals):void {
  if(visuals.closed)return;visuals.closed=true;
  visuals.geometry?.dispose();visuals.border?.dispose();visuals.vertexPick?.geometry.dispose();visuals.edge?.dispose();
  if(visuals.appearance)disposeWorldMeshAppearance(visuals.appearance);
}

function buildEdgeGeometry(mesh: WorldMeshData): BufferGeometry | null {
  if (!mesh.edgePositions?.length) return null;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(mesh.edgePositions), 3));
  return geometry;
}

/** 🧵️ Curve/centerline meshes have edge samples but no shaded triangles — pick/hover must treat them as whole instances. */
export function isCurveOnlyWorldMesh(mesh: Pick<WorldMeshData, "indices" | "edgePositions">): boolean {
  return Boolean(mesh.edgePositions?.length) && !(mesh.indices?.length > 0);
}

export function buildFaceOverlayGeometry(mesh: WorldMeshData, faceIds: ReadonlySet<number>): BufferGeometry | null {
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

/** 🖱️➡️ Approximates a picked face's in-plane size from its triangles' local bounding box, dropping the
 * smallest axis (roughly the one aligned with the face normal for axis-aligned primitive faces) — good
 * enough to size a push/pull tool's footprint without needing a true tangent-plane projection. */
export function faceExtentFromMesh(mesh: WorldMeshData, faceId: number): readonly [number, number] | undefined {
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

export function buildEdgeOverlayGeometry(mesh: WorldMeshData, edgeIds: ReadonlySet<number>): BufferGeometry | null {
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

export function buildVertexOverlayGeometry(mesh: WorldMeshData, vertexIds: ReadonlySet<number>): BufferGeometry | null {
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

/** 🌡️ How much of the hover or selection emissive tint a heatmap keeps, so the analysis colours stay readable under it. */
const WORLD_HEATMAP_TINT_SHARE = 0.5;
/** ✂️ Marks the shaded solid of an instance: the section cap stencils exactly these meshes. */
const WORLD_SOLID_MESH_USER_DATA = { world3dSolid: true } as const;

export function PaintTexturedMesh({
  geometry,
  style,
  styleKind,
  textureBase64,
  appearance,
  flatShading,
  children,
  ...meshProps
}: {
  readonly geometry: BufferGeometry;
  readonly style: MeshStyleColors;
  readonly styleKind: MeshStyleKind;
  readonly textureBase64?: string;
  readonly appearance?: WorldMeshAppearance | null;
  readonly flatShading?: boolean;
  readonly children?: React.ReactNode;
} & ComponentProps<"mesh">) {
  const paintMap = textureBase64 ? useLoader(TextureLoader, paintTextureUrl(textureBase64)) : null;
  const invalidate=useThree(state=>state.invalidate);
  useSyncExternalStore(useCallback(listener=>{appearance?.listeners.add(listener);return ()=>appearance?.listeners.delete(listener);},[appearance]),()=>`${appearance?.pending ?? 0}:${appearance?.fault ?? ""}:${appearance?.closed ?? false}`,()=>"0::false");
  useEffect(()=>{if(!appearance)return;const listener=()=>invalidate();appearance.listeners.add(listener);return ()=>{appearance.listeners.delete(listener);};},[appearance,invalidate]);
  const authored=appearance && !appearance.closed && (styleKind === "neutral" || styleKind === "disabled");
  // Per-vertex colours (energy class swatches, FEM contours) multiply against the material `color` in
  // three.js — white lets them show through in the neutral/disabled styles only. Selected/hovered paint
  // must match url-backed meshes (puzzle 3d, cad): solid token fill + emissive, not a guest-side bake.
  const hasVertexColors = geometry.hasAttribute("color");
  const heatmap = geometry.userData.world3dHeatmap === true && styleKind !== "provisional" && styleKind !== "celebrated";
  const preserveVertexColors = hasVertexColors && (styleKind === "neutral" || styleKind === "disabled" || heatmap);
  const heatmapTint = preserveVertexColors && styleKind !== "neutral" && styleKind !== "disabled";
  const celebrating = styleKind === "celebrated" && !preserveVertexColors;
  return (
    <mesh geometry={geometry} userData={WORLD_SOLID_MESH_USER_DATA} {...meshProps}>
      {authored ? <primitive object={appearance.materials} attach="material" dispose={null} /> : celebrating ? (
        <CelebratingConicMaterial opacity={style.opacity} />
      ) : (
        <meshStandardMaterial
          key={worldMeshMaterialRevision(styleKind)}
          color={preserveVertexColors ? "#ffffff" : style.meshColor}
          vertexColors={preserveVertexColors}
          map={paintMap ?? undefined}
          side={DoubleSide}
          flatShading={flatShading}
          metalness={0}
          roughness={1}
          emissive={preserveVertexColors && !heatmapTint ? "#000000" : style.meshColor}
          emissiveIntensity={preserveVertexColors && !heatmapTint ? 0 : style.emissiveIntensity * (heatmapTint ? WORLD_HEATMAP_TINT_SHARE : 1)}
          transparent={style.opacity < 1}
          opacity={style.opacity}
        />
      )}
      {children}
    </mesh>
  );
}

//#region GlbMeshBounds
/** 📦️ Local-space extents of every GLB the scene has loaded, keyed by the mesh record's own
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
export const GLB_MESH_LOCAL_BOUNDS = new Map<string, readonly (readonly [number, number, number])[]>();

/** 📦️ The eight corners of one loaded GLB's frame-rotated AABB, in the instance group's local space. */
export function glbMeshFrameCorners(scene: Object3D): readonly (readonly [number, number, number])[] {
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
export function applyGlbMeshEdgeBorders(root: Object3D, borderColor: string): void {
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

/** 📍️ Vertex-granularity pick dots and their hover/selection marks, in CSS pixels (screen-space). */
export const WORLD_VERTEX_DOT_PX = 6;
//#endregion WorldSceneParsing
export const WORLD_VERTEX_MARK_PX = 11;

/** 📦️ Local-space AABB corners of a mesh's vertex positions (or edge samples for curve-only meshes; fallback: origin). */
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
//#endregion 🔖️World3dHost
