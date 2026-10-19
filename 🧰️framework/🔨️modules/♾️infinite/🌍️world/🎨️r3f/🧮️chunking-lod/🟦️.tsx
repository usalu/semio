/** 🧮️ Chunk keys and visibility, chunked scene children, the view radius layer, ref-counted and template pools, and the level-of-detail context, bridge, grid helper and frame runner.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | r3f}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { reactHostPort, useCanvasAppearanceSync } from "@semio-tech/ui-react";
import { resolveThreeColor, themeColorVar } from "@semio-tech/ui-styling";
import {
  Children,
  isValidElement,
  type MutableRefObject,
  type ReactElement,
  type ReactNode
} from "react";
import {
  type Camera,
  DoubleSide,
  Grid,
  Mesh,
  Object3D,
  ThreeOrthographicCamera,
  ThreePerspectiveCamera,
  useFrame,
  useThree,
  Vector3
} from "../🔌️adapters/🟦️.ts";
import { type LodMeshEntry, type Vec3, worldRaycastNone } from "../🧷️r3f-foundation/🟦️.ts";
import {
  adaptiveOrbitCameraFar,
  worldCameraFov,
  worldCameraIsOrthographic,
  worldCameraZoom,
  worldProjectionMatchedPerspectiveDistance
} from "../📷️camera-view/🟦️.tsx";

// #region 🧱️Chunking
/** 🧱️ Stable chunk bucket key for a world-space origin. */
export function chunkKey(origin: Vec3, chunkSize: number): string {
  const ix = Math.floor(origin[0] / chunkSize);
  const iy = Math.floor(origin[1] / chunkSize);
  const iz = Math.floor(origin[2] / chunkSize);
  return `${ix}|${iy}|${iz}`;
}

function setEquals(a: ReadonlySet<string>, b: ReadonlySet<string>): boolean {
  if (a.size !== b.size) return false;
  for (const v of a) if (!b.has(v)) return false;
  return true;
}

/** 📏️ Chunk bounding radius in world units (half-space diagonal of a cube chunk). */
export function chunkBoundsRadius(chunkSize: number): number {
  return chunkSize * 0.866;
}

/** 👁️ Distance-only chunk visibility with enter/exit hysteresis (avoids frustum-edge flicker). */
export function chunkDistanceVisible(args: { readonly camPos: Vector3; readonly chunkCenter: Vector3; readonly chunkSize: number; readonly maxDist: number; readonly wasVisible: boolean }): boolean {
  const boundsR = chunkBoundsRadius(args.chunkSize);
  const dist = args.camPos.distanceTo(args.chunkCenter);
  const enterDist = args.maxDist + boundsR;
  const exitDist = enterDist + args.chunkSize * 0.5;
  if (dist <= enterDist) return true;
  if (args.wasVisible && dist <= exitDist) return true;
  return false;
}

/** 👁️ Tracks which chunk keys are within view radius (per-frame, hysteresis). */
export function useVisibleChunkKeys(chunkKeys: Iterable<string>, chunkSize: number, maxDist: number): ReadonlySet<string> {
  const { camera } = useThree();
  const centerTmp = reactHostPort.useMemo(() => new Vector3(), []);
  const [visible, setVisible] = reactHostPort.useState<ReadonlySet<string>>(() => new Set());
  useFrame(() => {
    const camPos = camera.position;
    setVisible((prev) => {
      const next = new Set(prev);
      for (const key of chunkKeys) {
        const [ix, iy, iz] = key.split("|").map(Number);
        if(ix===undefined||iy===undefined||iz===undefined)throw Error("Complete spatial chunk coordinates required");
        centerTmp.set((ix + 0.5) * chunkSize, (iy + 0.5) * chunkSize, (iz + 0.5) * chunkSize);
        const show = chunkDistanceVisible({
          camPos,
          chunkCenter: centerTmp,
          chunkSize,
          maxDist,
          wasVisible: next.has(key),
        });
        if (show) next.add(key);
        else next.delete(key);
      }
      return setEquals(prev, next) ? prev : next;
    });
  });
  return visible;
}

/** 🧩️ Splits scene children with `origin` into chunked vs unchunked buckets. */
export function splitChunkedSceneChildren(children: ReactNode): { chunked: ReactNode[]; rest: ReactNode[] } {
  const chunked: ReactNode[] = [];
  const rest: ReactNode[] = [];
  Children.forEach(children, (c) => {
    if (isValidElement(c) && (c.props as { origin?: Vec3 }).origin !== undefined) chunked.push(c);
    else rest.push(c);
  });
  return { chunked, rest };
}

/** 🧱️ Renders origin-tagged children grouped by {@link chunkKey} with view-radius visibility. */
export function WorldChunks(props: { readonly chunkSize: number; readonly maxDistance: number; readonly children: ReactNode }): ReactElement {
  const buckets = reactHostPort.useMemo(() => {
    const map = new Map<string, ReactNode[]>();
    Children.forEach(props.children, (child) => {
      if (!isValidElement(child)) return;
      const p = child.props as { origin?: Vec3 };
      if (!p?.origin) return;
      const k = chunkKey(p.origin, props.chunkSize);
      const arr = map.get(k) ?? [];
      arr.push(child);
      map.set(k, arr);
    });
    return map;
  }, [props.children, props.chunkSize]);

  const visible = useVisibleChunkKeys(buckets.keys(), props.chunkSize, props.maxDistance);
  return (
    <>
      {[...buckets].map(([key, items]) => (
        <group key={key} userData={{ sceneChunk: key }} visible={visible.has(key)}>
          {items}
        </group>
      ))}
    </>
  );
}

/** 🧩️ Chunked scene subtree + unchunked sibling group (view-radius streaming). */
export function WorldChunkedSceneChildren(props: { readonly chunkSize: number; readonly maxDistance: number; readonly children: ReactNode; readonly unchunkedDataAttr?: string }): ReactElement {
  const { chunked, rest } = reactHostPort.useMemo(() => splitChunkedSceneChildren(props.children), [props.children]);
  const attr = props.unchunkedDataAttr ?? "data-world-unchunked";
  return (
    <>
      <WorldChunks chunkSize={props.chunkSize} maxDistance={props.maxDistance}>
        {chunked}
      </WorldChunks>
      <group {...{ [attr]: true }}>{rest}</group>
    </>
  );
}
// #endregion 🧱️Chunking

// #region 👁️ViewRadius
export interface ViewRadiusLayerProps {
  readonly chunkSize: number;
  readonly maxDistance: number;
  readonly children: ReactNode;
}

/** 👁️ View-radius layer: alias for {@link WorldChunkedSceneChildren} in the layer stack. */
export function ViewRadiusLayer(props: ViewRadiusLayerProps): ReactElement {
  return <WorldChunkedSceneChildren chunkSize={props.chunkSize} maxDistance={props.maxDistance} children={props.children} />;
}
// #endregion 👁️ViewRadius

// #region 🏊️Pool
/** 🏊️ Generic refcount pool for asset keys (caller supplies template factory). */
export function createRefCountPool<TKey extends string>(): {
  acquire(key: TKey): void;
  release(key: TKey): void;
  count(key: TKey): number;
  delete(key: TKey): void;
  keys(): IterableIterator<TKey>;
} {
  const counts = new Map<TKey, number>();
  return {
    acquire(key) {
      counts.set(key, (counts.get(key) ?? 0) + 1);
    },
    release(key) {
      const n = (counts.get(key) ?? 1) - 1;
      if (n <= 0) counts.delete(key);
      else counts.set(key, n);
    },
    count(key) {
      return counts.get(key) ?? 0;
    },
    delete(key) {
      counts.delete(key);
    },
    keys() {
      return counts.keys();
    },
  };
}

/** 🏊️ Template cache with refcount (style variants keyed by caller); `TValue` defaults to {@link Object3D} but any pooled disposable (e.g. `BufferGeometry`) works. */
export function createTemplatePool<TKey extends string, TValue = Object3D>(): {
  acquire(key: TKey): void;
  release(key: TKey): void;
  getOrCreate(key: TKey, factory: () => TValue): TValue;
  delete(key: TKey): void;
  deleteByPrefix(prefix: string): void;
  has(key: TKey): boolean;
  peek(key: TKey): TValue | undefined;
} {
  const refCounts = new Map<TKey, number>();
  const templates = new Map<TKey, TValue>();
  return {
    acquire(key) {
      refCounts.set(key, (refCounts.get(key) ?? 0) + 1);
    },
    release(key) {
      const n = (refCounts.get(key) ?? 1) - 1;
      if (n <= 0) {
        refCounts.delete(key);
        templates.delete(key);
      } else {
        refCounts.set(key, n);
      }
    },
    getOrCreate(key, factory) {
      let template = templates.get(key);
      if (!template) {
        template = factory();
        templates.set(key, template);
      }
      return template;
    },
    delete(key) {
      refCounts.delete(key);
      templates.delete(key);
    },
    deleteByPrefix(prefix) {
      for (const key of [...templates.keys()]) {
        if (String(key).startsWith(prefix)) {
          templates.delete(key);
          refCounts.delete(key);
        }
      }
    },
    has(key) {
      return templates.has(key);
    },
    peek(key) {
      return templates.get(key);
    },
  };
}

/** 🔌️ Port for leaf bundles that load assets (e.g. GLTF via drei). */
export interface AssetPoolPort {
  clear(url: string): void;
}
// #endregion 🏊️Pool

// #region 📶️Lod
export const DEFAULT_LOD_RANGE = { min: 0.01, max: 100_000 } as const;
export const DEFAULT_MANUAL_LOD = 100;
export const DEFAULT_GRID_PLANE_ANCHOR_CAD: Vec3 = [0, 0, 0];
export const DEFAULT_LOD_GRID_FACTOR = 10;
export const WORLD_LOD_SLIDER_MIN = 0;
export const WORLD_LOD_SLIDER_MAX = 1000;
export const WORLD_LOD_EPSILON = 0.01;
export const WORLD_LOD_GRID_BASE_LOD = 2;
export const WORLD_LOD_GRID_MIN_FADE_CELLS = 24;
export const WORLD_LOD_GRID_FADE_HEIGHT_FACTOR = 32;
export const WORLD_LOD_GRID_FADE_STRENGTH = 1.5;
/** 🌫️ Multiplier on the live frustum radius so the procedural grid plane always overfills the viewport (including corners) with near-uniform opacity under {@link WORLD_LOD_GRID_FADE_STRENGTH}. */
export const WORLD_LOD_GRID_COVERAGE_MARGIN = 32;
/** 📶️ Reference perspective FOV (°) used to map orthographic zoom onto an equivalent orbit distance for automatic LOD / grid banding — matches the default three-point FOV. */
export const WORLD_LOD_REFERENCE_FOV_DEG = 50;

/** 📶️ Maps orbit camera distance to scene LOD (`distance / reference`). */
export function lodFromCameraDistance(distance: number, reference: number): number {
  const d = Math.max(distance, 1e-6);
  const ref = Math.max(reference, 1e-6);
  return d / ref;
}

/** 📶️ Orbit distance that drives automatic LOD — perspective uses eye→target; orthographic maps `zoom` through {@link worldProjectionMatchedPerspectiveDistance} so scroll (which changes zoom, not dolly) still retunes the grid. */
export function lodOrbitDistanceForCamera(camera: Camera, orbitDistance: number, viewportHeight: number): number {
  if (!worldCameraIsOrthographic(camera)) return Math.max(orbitDistance, 1e-6);
  return worldProjectionMatchedPerspectiveDistance(WORLD_LOD_REFERENCE_FOV_DEG, worldCameraZoom(camera), viewportHeight);
}

/** 📶️ Picks the closest available LOD; on log-distance ties prefers the smaller (more detailed) LOD. */
export function pickClosestLod(available: readonly number[], desired: number): number | null {
  if (!available.length || !Number.isFinite(desired) || desired <= 0) return null;
  let best = available[0]!;
  let bestDist = Math.abs(Math.log(best) - Math.log(desired));
  for (let i = 1; i < available.length; i++) {
    const rep = available[i]!;
    if (!Number.isFinite(rep) || rep <= 0) continue;
    const dist = Math.abs(Math.log(rep) - Math.log(desired));
    if (dist < bestDist - 1e-12 || (Math.abs(dist - bestDist) <= 1e-12 && rep < best)) {
      best = rep;
      bestDist = dist;
    }
  }
  return best;
}

/** 🎨️ Resolves a mesh URL from per-LOD entries with {@link pickClosestLod}. */
export function pickClosestMeshUrl(entries: readonly LodMeshEntry[] | undefined, desired: number, fallback?: string): string | undefined {
  if (!entries?.length) return fallback;
  const lods = entries.map((e) => e.lod).filter((lod) => Number.isFinite(lod) && lod > 0);
  const picked = pickClosestLod(lods, desired);
  if (picked == null) return fallback;
  const match = entries.find((e) => e.lod === picked);
  return match?.url ?? fallback;
}

/** 📶️ Formats scene LOD for host readouts. */
export function formatLod(lod: number): string {
  return Number.isFinite(lod) ? lod.toFixed(2) : "—";
}

/** 📶️ Maps a linear slider position to log-spaced scene LOD. */
export function lodFromSliderValue(slider: number, range: { readonly min: number; readonly max: number } = DEFAULT_LOD_RANGE): number {
  const t = Math.max(0, Math.min(1, (slider - WORLD_LOD_SLIDER_MIN) / (WORLD_LOD_SLIDER_MAX - WORLD_LOD_SLIDER_MIN)));
  const logMin = Math.log(range.min);
  const logMax = Math.log(range.max);
  return Math.exp(logMin + t * (logMax - logMin));
}

/** 📶️ Maps scene LOD to a linear slider position. */
export function sliderValueFromLod(lod: number, range: { readonly min: number; readonly max: number } = DEFAULT_LOD_RANGE): number {
  const clamped = Math.max(range.min, Math.min(range.max, lod));
  const logMin = Math.log(range.min);
  const logMax = Math.log(range.max);
  const t = (Math.log(clamped) - logMin) / (logMax - logMin);
  return Math.round(WORLD_LOD_SLIDER_MIN + t * (WORLD_LOD_SLIDER_MAX - WORLD_LOD_SLIDER_MIN));
}

/** 📐️ Sparse, unbounded LOD grid step using the configured spacing and regular 1–2.5–5 multipliers. */
export function lodGridStepWorld(lod: number, gridFactor: number): number | null {
  if (!Number.isFinite(lod) || lod <= 0 || !Number.isFinite(gridFactor) || gridFactor <= 0) return null;
  const targetMultiplier = Math.max(1, lod / WORLD_LOD_GRID_BASE_LOD);
  const magnitude = 10 ** Math.floor(Math.log10(targetMultiplier));
  const normalized = targetMultiplier / magnitude;
  const quantum = normalized <= 1 ? 1 : normalized <= 2.5 ? 2.5 : normalized <= 5 ? 5 : 10;
  return gridFactor * quantum * magnitude;
}

/** 🌫️ World-space radius of the camera frustum on the grid plane — orthographic uses zoomed pixel extent; perspective uses FOV × height above the plane. */
export function cameraGridVisibleRadius(camera: Camera, planeZ: number, viewport: { readonly width: number; readonly height: number }): number {
  const width = Math.max(viewport.width, 1);
  const height = Math.max(viewport.height, 1);
  if (worldCameraIsOrthographic(camera)) {
    const zoom = Math.max(worldCameraZoom(camera), 1e-6);
    return Math.hypot(width * 0.5 / zoom, height * 0.5 / zoom);
  }
  const distance = Math.max(Math.abs(camera.position.z - planeZ), 1e-3);
  const fov = worldCameraFov(camera) ?? WORLD_LOD_REFERENCE_FOV_DEG;
  const halfHeight = distance * Math.tan((fov * Math.PI) / 360);
  const halfWidth = halfHeight * (width / height);
  return Math.hypot(halfWidth, halfHeight);
}

/** 🌫️ Stable world-space fade radius for the procedural grid that always overfills the live viewport so ortho/persp zoom never reveals a hard plane edge. */
export function cameraGridFadeDistance(camera: Camera, planeZ: number, stepWorld: number, viewport?: { readonly width: number; readonly height: number }): number {
  const coverage =
    viewport != null
      ? cameraGridVisibleRadius(camera, planeZ, viewport) * WORLD_LOD_GRID_COVERAGE_MARGIN
      : Math.abs(camera.position.z - planeZ) * WORLD_LOD_GRID_FADE_HEIGHT_FACTOR;
  const target = Math.max(stepWorld * WORLD_LOD_GRID_MIN_FADE_CELLS, coverage);
  const cells = 2 ** Math.ceil(Math.log2(Math.max(target / stepWorld, 1)));
  const fade = stepWorld * cells;
  const far = "far" in camera && typeof camera.far === "number" ? camera.far : undefined;
  const farCap = far !== undefined && Number.isFinite(far) && far > 0 ? far * 0.25 : fade;
  return Math.max(coverage, Math.min(fade, Math.max(farCap, coverage)));
}

export interface LodContextValue {
  readonly lod: number;
  readonly depthVariable: boolean;
  readonly lodForWorldPosition: (position: Vec3) => number;
  readonly gridStepWorld: number | null;
  readonly gridFactor: number;
  readonly gridSnapEnabled: boolean;
}

const LodContext = reactHostPort.createContext<LodContextValue | null>(null);

/** 📶️ Reads the live scene LOD band and grid snap step from world context. */
export function useLod(): LodContextValue {
  const v = reactHostPort.useContext(LodContext);
  if (!v) throw new Error("World LOD context missing");
  return v;
}

interface LodRuntimeCells {
  sceneLod: number;
  depthVariable: boolean;
  distanceReference: number;
  camera: Camera | null;
  tmpWorld: Vector3;
}

/** 📐️ Sparse procedural world grid that follows the camera and fades without a finite edge. */
export function WorldLodGridHelper(props: { readonly gridDatum?: Vec3 }): ReactElement | null {
  const lod = useLod();
  const camera = useThree((s) => s.camera);
  const size = useThree((s) => s.size);
  const invalidate = useThree((s) => s.invalidate);
  const datum = props.gridDatum ?? DEFAULT_GRID_PLANE_ANCHOR_CAD;
  const stepWorld = lodGridStepWorld(lod.lod, lod.gridFactor);
  const [fadeDistance, setFadeDistance] = reactHostPort.useState(() =>
    stepWorld == null ? 1 : cameraGridFadeDistance(camera, datum[2], stepWorld, size),
  );
  const gridRef = reactHostPort.useRef<Mesh | null>(null);
  useFrame(() => {
    if (stepWorld == null) return;
    const next = cameraGridFadeDistance(camera, datum[2], stepWorld, size);
    setFadeDistance((previous) => (previous === next ? previous : next));
  });
  const [gridColor, setGridColor] = reactHostPort.useState<number>(() => resolveThreeColor(themeColorVar("element"), "gray"));
  useCanvasAppearanceSync(reactHostPort.useCallback(() => setGridColor(resolveThreeColor(themeColorVar("element"), "gray")), []));
  reactHostPort.useLayoutEffect(() => {
    const grid = gridRef.current;
    if (!grid) return;
    const materials = Array.isArray(grid.material) ? grid.material : [grid.material];
    for (const material of materials) {
      material.depthTest = true;
      material.depthWrite = false;
    }
    grid.renderOrder = -5;
    grid.raycast = worldRaycastNone;
    invalidate();
  }, [gridColor, invalidate]);
  if (stepWorld == null) return null;
  return (
    <Grid
      ref={gridRef}
      args={[2, 2]}
      position={[0, 0, datum[2] + 0.001]}
      rotation={[Math.PI / 2, 0, 0]}
      cellSize={stepWorld}
      cellThickness={0.6}
      cellColor={gridColor}
      sectionSize={stepWorld}
      sectionThickness={0}
      sectionColor={gridColor}
      fadeDistance={fadeDistance}
      fadeStrength={WORLD_LOD_GRID_FADE_STRENGTH}
      followCamera
      infiniteGrid
      side={DoubleSide}
    />
  );
}

function LodFrameRunner(props: {
  readonly lodRef: MutableRefObject<number>;
  readonly lodRuntimeRef: MutableRefObject<LodRuntimeCells>;
  readonly distanceReference: number;
  readonly gridFactor: number;
  readonly gridSnapEnabled: boolean;
  readonly automaticLod: boolean;
  readonly depthVariableLod: boolean;
  readonly manualLod: number;
  readonly onLod: (patch: { readonly sceneLod: number; readonly depthVariable: boolean; readonly gridStepWorld: number | null }) => void;
  readonly onLodChange?: (lod: number) => void;
}): null {
  const cam = useThree((s) => s.camera);
  const controls = useThree((s) => s.controls as { target?: Vector3 } | null);
  const size = useThree((s) => s.size);
  const invalidate = useThree((s) => s.invalidate);
  const tmpT = reactHostPort.useMemo(() => new Vector3(), []);
  const ctxSig = reactHostPort.useRef("");
  useFrame(() => {
    const tgt = controls?.target ?? tmpT.set(0, 0, 0);
    const dist = cam.position.distanceTo(tgt);
    if (cam instanceof ThreePerspectiveCamera || cam instanceof ThreeOrthographicCamera) {
      const far = adaptiveOrbitCameraFar(dist);
      if (cam.far !== far) {
        cam.far = far;
        cam.updateProjectionMatrix();
        invalidate();
      }
    }
    const lodDist = lodOrbitDistanceForCamera(cam, dist, size.height);
    const autoLod = lodFromCameraDistance(lodDist, props.distanceReference);
    const sceneLod = props.automaticLod ? autoLod : props.depthVariableLod ? autoLod : props.manualLod;
    props.lodRef.current = sceneLod;
    const runtime = props.lodRuntimeRef.current;
    runtime.sceneLod = sceneLod;
    runtime.depthVariable = props.depthVariableLod;
    runtime.distanceReference = props.distanceReference;
    runtime.camera = cam;
    const gridStep = lodGridStepWorld(sceneLod, props.gridFactor);
    const sig = `${props.depthVariableLod ? 1 : 0}|${gridStep ?? "x"}|${props.gridFactor}|${props.gridSnapEnabled}`;
    if (ctxSig.current !== sig) {
      ctxSig.current = sig;
      props.onLod({ sceneLod, depthVariable: props.depthVariableLod, gridStepWorld: gridStep });
      props.onLodChange?.(sceneLod);
      invalidate();
    }
  });
  return null;
}

export interface WorldLodBridgeProps {
  readonly children: ReactNode;
  readonly lodRef: MutableRefObject<number>;
  readonly distanceReference: number;
  readonly gridFactor: number;
  readonly gridSnapEnabled: boolean;
  readonly showLodGrid: boolean;
  readonly automaticLod: boolean;
  readonly depthVariableLod: boolean;
  readonly manualLod: number;
  readonly gridDatum?: Vec3;
  readonly onLodChange?: (lod: number) => void;
}

/** 📶️ LOD runtime bridge: context + optional {@link WorldLodGridHelper}. */
export function WorldLodBridge(props: WorldLodBridgeProps): ReactElement {
  const tmpWorld = reactHostPort.useMemo(() => new Vector3(), []);
  const lodRuntimeRef = reactHostPort.useRef<LodRuntimeCells>({
    sceneLod: DEFAULT_MANUAL_LOD,
    depthVariable: false,
    distanceReference: props.distanceReference,
    camera: null,
    tmpWorld,
  });
  const lodForWorldPosition = reactHostPort.useCallback((position: Vec3) => {
    const r = lodRuntimeRef.current;
    if (!r.depthVariable || !r.camera) return r.sceneLod;
    if (worldCameraIsOrthographic(r.camera)) return r.sceneLod;
    r.tmpWorld.set(position[0], position[1], position[2]);
    return lodFromCameraDistance(r.camera.position.distanceTo(r.tmpWorld), r.distanceReference);
  }, []);
  const [sceneLod, setLod] = reactHostPort.useState(DEFAULT_MANUAL_LOD);
  const [depthVariable, setDepthVariable] = reactHostPort.useState(false);
  const [gridStepWorld, setGridStepWorld] = reactHostPort.useState<number | null>(() => lodGridStepWorld(DEFAULT_MANUAL_LOD, props.gridFactor));
  const onLod = reactHostPort.useCallback((patch: { readonly sceneLod: number; readonly depthVariable: boolean; readonly gridStepWorld: number | null }) => {
    setLod((prev) => (Math.abs(prev - patch.sceneLod) > WORLD_LOD_EPSILON ? patch.sceneLod : prev));
    setDepthVariable((prev) => (prev === patch.depthVariable ? prev : patch.depthVariable));
    setGridStepWorld((prev) => (prev === patch.gridStepWorld ? prev : patch.gridStepWorld));
  }, []);
  const lodCtx = reactHostPort.useMemo<LodContextValue>(
    () => ({
      lod: sceneLod,
      depthVariable,
      lodForWorldPosition,
      gridStepWorld,
      gridFactor: props.gridFactor,
      gridSnapEnabled: props.gridSnapEnabled,
    }),
    [sceneLod, depthVariable, lodForWorldPosition, gridStepWorld, props.gridFactor, props.gridSnapEnabled],
  );
  return (
    <LodContext.Provider value={lodCtx}>
      <LodFrameRunner
        lodRef={props.lodRef}
        lodRuntimeRef={lodRuntimeRef}
        distanceReference={props.distanceReference}
        gridFactor={props.gridFactor}
        gridSnapEnabled={props.gridSnapEnabled}
        automaticLod={props.automaticLod}
        depthVariableLod={props.depthVariableLod}
        manualLod={props.manualLod}
        onLod={onLod}
        onLodChange={props.onLodChange}
      />
      {props.showLodGrid ? <WorldLodGridHelper gridDatum={props.gridDatum} /> : null}
      {props.children}
    </LodContext.Provider>
  );
}

/** 📐️ Grid layer slot for {@link WorldLayerStack}. */
export function GridLayer(props: { readonly gridDatum?: Vec3 }): ReactElement | null {
  return <WorldLodGridHelper gridDatum={props.gridDatum} />;
}
// #endregion 📶️Lod
