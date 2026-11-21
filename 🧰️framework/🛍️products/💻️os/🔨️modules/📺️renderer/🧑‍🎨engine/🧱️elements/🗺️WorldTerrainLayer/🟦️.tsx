// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🗺️WorldTerrainLayer/component.tsx
/** ⛰️ `🗺️WorldTerrainLayer` — chunked-DEM-tile GIS terrain mesh mounted alongside `WorldInstancesLayer` inside the shared `World3d` viewport, driven by a wasm `TerrainSession`. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { publishedPageUrl } from "@semio-tech/framework/assets/published-page";
import { useEffect, useMemo, useRef, useState } from "../⚛️react-port/🟦️.ts";
import { BufferAttribute, BufferGeometry, CanvasTexture, ClampToEdgeWrapping, DoubleSide, MeshStandardMaterial } from "@semio-tech/ui-react";
import { createTerrainSession, type TerrainWasmSession } from "../🪪️WasmSessionLoader/🟦️.tsx";
import { parseTerrainOverlayDescriptor, TerrainMapFeatures, type TerrainOverlayTile } from "./🗺️overlays/🟦️.tsx";
import { TerrainBrushStroke, type TerrainBrushSample } from "./🖌️brush/👆️stroke.ts";
// #endregion 🔌️Adapters

//#region 🔖️WorldTerrainLayerHost
//#region TerrainStyle
export type WorldTerrainStyle = {
  readonly tileUrlTemplate: string;
  readonly elevationGridJson?: string;
  readonly projectOriginLon: number;
  readonly projectOriginLat: number;
  readonly exaggeration: number;
  readonly colorRamp: string;
  readonly minZoom: number;
  readonly maxZoom: number;
};

export function parseWorldTerrainStyle(json: string | undefined): WorldTerrainStyle | null {
  if (!json) return null;
  try {
    const parsed = JSON.parse(json) as Partial<WorldTerrainStyle> & { readonly elevationGrid?: unknown };
    const elevationGridJson = parsed.elevationGrid == null ? undefined : JSON.stringify(parsed.elevationGrid);
    if (!elevationGridJson && typeof parsed.tileUrlTemplate !== "string") return null;
    return {
      tileUrlTemplate: parsed.tileUrlTemplate ?? "",
      elevationGridJson,
      projectOriginLon: parsed.projectOriginLon ?? 0,
      projectOriginLat: parsed.projectOriginLat ?? 0,
      exaggeration: parsed.exaggeration ?? 1,
      colorRamp: parsed.colorRamp ?? "hypsometric",
      minZoom: terrainZoom(parsed.minZoom, 6),
      maxZoom: terrainZoom(parsed.maxZoom, 14),
    };
  } catch {
    return null;
  }
}

function terrainZoom(value: unknown, fallback: number): number {
  return typeof value === "number" && Number.isInteger(value) && value >= 0 ? Math.min(value, 31) : fallback;
}
//#endregion TerrainStyle

//#region TerrainMesh
type TerrainTileMeshPayload = {
  readonly positions: number[];
  readonly normals: number[];
  readonly indices: number[];
  readonly uvs: number[];
};

function geometryFromTerrainMesh(mesh: TerrainTileMeshPayload): BufferGeometry {
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(mesh.positions), 3));
  geometry.setAttribute("normal", new BufferAttribute(new Float32Array(mesh.normals), 3));
  geometry.setAttribute("uv", new BufferAttribute(new Float32Array(mesh.uvs), 2));
  geometry.setIndex(mesh.indices);
  return geometry;
}

/** 🎨️ A vertical hypsometric ramp (low -> green/tan, high -> grey rock, peak -> white) sampled by
 * each terrain vertex's normalized-elevation `uv.y` — generated once client-side rather than
 * round-tripped through Rust, since it's a pure display convenience. */
let hypsometricTexture: CanvasTexture | null = null;
function getHypsometricTexture(): CanvasTexture {
  if (hypsometricTexture) return hypsometricTexture;
  const canvas = document.createElement("canvas");
  canvas.width = 2;
  canvas.height = 256;
  const ctx = canvas.getContext("2d");
  if (ctx) {
    const gradient = ctx.createLinearGradient(0, canvas.height, 0, 0);
    gradient.addColorStop(0, "#4b6b3a");
    gradient.addColorStop(0.5, "#a68a5b");
    gradient.addColorStop(0.85, "#8f8f8f");
    gradient.addColorStop(1, "#ffffff");
    ctx.fillStyle = gradient;
    ctx.fillRect(0, 0, canvas.width, canvas.height);
  }
  hypsometricTexture = new CanvasTexture(canvas);
  hypsometricTexture.wrapS = ClampToEdgeWrapping;
  hypsometricTexture.wrapT = ClampToEdgeWrapping;
  hypsometricTexture.needsUpdate = true;
  return hypsometricTexture;
}
//#endregion TerrainMesh

//#region TerrainTileRenderer
const TERRAIN_TILE_REFRESH_DEBOUNCE_MS = 150;
const MAX_CONCURRENT_TERRAIN_TILE_FETCHES = 8;
const TERRAIN_TILE_TIMEOUT_MS = 12000;
const TERRAIN_TILE_RETRY_MS = 1000;
const TERRAIN_TILE_MAX_RETRY_MS = 30000;

type TerrainTileRow = { readonly z: number; readonly x: number; readonly y: number; readonly key: string };
type TerrainTileRetry = { readonly delay: number; readonly readyAt: number };

function parseVisibleTerrainTilesJson(raw: string): TerrainTileRow[] {
  try {
    const rows = JSON.parse(raw) as TerrainTileRow[];
    return Array.isArray(rows) ? rows : [];
  } catch {
    return [];
  }
}

/** 🧵️ Owns a `TerrainSession`, fetches/uploads/evicts DEM tiles as the camera moves, and reports
 * the current set of tile geometries back to React — the 3D analog of `tiled-map-host.tsx`'s
 * `MapRenderer`, except it hands back mesh buffers for three.js instead of driving a canvas. */
class TerrainTileRenderer {
  private disposed = false;
  private session: TerrainWasmSession | null = null;
  private cameraJson: string | null = null;
  private visible = new Map<string, TerrainTileRow>();
  private readonly requests = new Map<string, AbortController>();
  private readonly retries = new Map<string, TerrainTileRetry>();
  private refreshTimer: ReturnType<typeof setTimeout> | null = null;
  private retryTimer: ReturnType<typeof setTimeout> | null = null;
  private readonly geometries = new Map<string, BufferGeometry>();
  private readonly tileUrlTemplate: string;

  private readonly style: WorldTerrainStyle;
  private readonly onGeometriesChanged: (geometries: Map<string, BufferGeometry>) => void;

  constructor(
    style: WorldTerrainStyle,
    onGeometriesChanged: (geometries: Map<string, BufferGeometry>) => void,
  ) {
    this.style = style;
    this.onGeometriesChanged = onGeometriesChanged;
    this.tileUrlTemplate = style.elevationGridJson ? "" : publishedPageUrl(style.tileUrlTemplate);
  }

  async init(): Promise<void> {
    const session = await createTerrainSession();
    if (this.disposed) {
      session.free();
      return;
    }
    session.set_project_origin(this.style.projectOriginLon, this.style.projectOriginLat);
    session.set_exaggeration(this.style.exaggeration);
    if (this.style.elevationGridJson && !session.set_elevation_grid_json(this.style.elevationGridJson)) {
      session.free();
      return;
    }
    this.session = session;
    this.refresh();
  }

  scheduleRefresh(cameraJson: string): void {
    if (this.disposed || this.cameraJson === cameraJson) return;
    this.cameraJson = cameraJson;
    if (this.refreshTimer !== null) clearTimeout(this.refreshTimer);
    this.refreshTimer = setTimeout(() => {
      this.refreshTimer = null;
      this.refresh();
    }, TERRAIN_TILE_REFRESH_DEBOUNCE_MS);
  }

  private refresh(): void {
    const session = this.session;
    if (!session || this.cameraJson === null) return;
    const rows = parseVisibleTerrainTilesJson(session.visible_terrain_tiles_json(this.cameraJson));
    const previouslyVisible = this.visible;
    this.visible = new Map(rows.map((row) => [row.key, row]));
    let removedGeometry = false;
    for (const [key, row] of previouslyVisible) {
      if (this.visible.has(key)) continue;
      this.requests.get(key)?.abort();
      this.retries.delete(key);
      const geometry = this.geometries.get(key);
      if (!geometry) continue;
      geometry.dispose();
      this.geometries.delete(key);
      session.evict_terrain_tile(row.z, row.x, row.y);
      removedGeometry = true;
    }
    if (removedGeometry) this.onGeometriesChanged(new Map(this.geometries));
    this.pump();
  }

  getTerrainOverlayTile(key: string, overlaysJson: string): string {
    const row = this.visible.get(key);
    return row && this.session ? this.session.terrain_overlay_tile_json(row.z, row.x, row.y, overlaysJson) : "null";
  }

  getOriginElevation(key: string): number | undefined {
    const row = this.visible.get(key);
    return row ? this.session?.terrain_origin_elevation(row.z, row.x, row.y) : undefined;
  }

  pickElevationSample(origin: readonly number[], direction: readonly number[]): readonly [number, number] | undefined {
    if (!this.style.elevationGridJson || !this.session) return undefined;
    const hit = JSON.parse(this.session.pickElevationGridJson(JSON.stringify({ origin, direction }))) as { readonly sample: readonly [number, number] } | null;
    return hit?.sample;
  }

  private pump(): void {
    const session = this.session;
    if (!session) return;
    if (this.retryTimer !== null) clearTimeout(this.retryTimer);
    this.retryTimer = null;
    const now = performance.now();
    let nextRetry = Infinity;
    for (const row of this.visible.values()) {
      if (this.requests.size >= MAX_CONCURRENT_TERRAIN_TILE_FETCHES) break;
      if (this.geometries.has(row.key) || this.requests.has(row.key)) continue;
      if (this.style.elevationGridJson) {
        this.publishMesh(session, row);
        continue;
      }
      const readyAt = this.retries.get(row.key)?.readyAt ?? now;
      if (readyAt > now) {
        nextRetry = Math.min(nextRetry, readyAt);
        continue;
      }
      const request = new AbortController();
      this.requests.set(row.key, request);
      void this.uploadOne(session, row, request).finally(() => {
        this.requests.delete(row.key);
        this.pump();
      });
    }
    if (Number.isFinite(nextRetry)) this.retryTimer = setTimeout(() => this.pump(), Math.max(1, nextRetry - performance.now()));
  }

  private deferTile(key: string, retry: boolean): void {
    const delay = Math.min(TERRAIN_TILE_MAX_RETRY_MS, (this.retries.get(key)?.delay ?? TERRAIN_TILE_RETRY_MS / 2) * 2);
    this.retries.set(key, { delay, readyAt: retry ? performance.now() + delay : Infinity });
  }

  private publishMesh(session: TerrainWasmSession, { z, x, y, key }: TerrainTileRow): boolean {
    const mesh = JSON.parse(session.terrain_tile_mesh_json(z, x, y)) as TerrainTileMeshPayload | null;
    if (!mesh) return false;
    this.geometries.set(key, geometryFromTerrainMesh(mesh));
    this.onGeometriesChanged(new Map(this.geometries));
    return true;
  }

  private async uploadOne(session: TerrainWasmSession, row: TerrainTileRow, request: AbortController): Promise<void> {
    const { z, x, y, key } = row;
    const url = this.tileUrlTemplate.replaceAll("{z}", String(z)).replaceAll("{x}", String(x)).replaceAll("{y}", String(y));
    const signal = AbortSignal.any([request.signal, AbortSignal.timeout(TERRAIN_TILE_TIMEOUT_MS)]);
    try {
      const response = await fetch(url, { signal });
      if (request.signal.aborted) return;
      if (!response.ok) return this.deferTile(key, response.status >= 500 || response.status === 408 || response.status === 429);
      const bytes = new Uint8Array(await response.arrayBuffer());
      if (request.signal.aborted) return;
      if (!session.upload_elevation_tile(z, x, y, bytes)) return this.deferTile(key, false);
      if (!this.publishMesh(session, row)) {
        session.evict_terrain_tile(z, x, y);
        return this.deferTile(key, false);
      }
      this.retries.delete(key);
    } catch {
      if (!request.signal.aborted) {
        session.evict_terrain_tile(z, x, y);
        this.deferTile(key, true);
      }
    }
  }

  dispose(): void {
    this.disposed = true;
    if (this.refreshTimer !== null) clearTimeout(this.refreshTimer);
    if (this.retryTimer !== null) clearTimeout(this.retryTimer);
    for (const request of this.requests.values()) request.abort();
    for (const geometry of this.geometries.values()) geometry.dispose();
    this.geometries.clear();
    this.session?.free();
    this.session = null;
  }
}
//#endregion TerrainTileRenderer

//#region WorldTerrainLayer
/** ⛰️ Renders GIS 3D terrain as chunked DEM-tile meshes inside the shared `World3d` viewport —
 * mounted alongside `WorldInstancesLayer` when `scene.terrainJson` is present. */
export function WorldTerrainLayer({ terrainJson, cameraPosition, cameraTarget, onFeatureHover, onOriginElevation, brush, onElevationStroke }: {
  readonly terrainJson: string | undefined;
  readonly cameraPosition: readonly [number, number, number];
  readonly cameraTarget: readonly [number, number, number];
  readonly onFeatureHover?: (name: string | null) => void;
  readonly onOriginElevation?: (elevation: number) => void;
  readonly brush?: { readonly radius: number; readonly key: string };
  readonly onElevationStroke?: (samples: readonly TerrainBrushSample[]) => void;
}) {
  const style = useMemo(() => parseWorldTerrainStyle(terrainJson), [terrainJson]);
  const overlaysJson = useMemo(() => JSON.stringify(parseTerrainOverlayDescriptor(terrainJson).overlays), [terrainJson]);
  const rendererRef = useRef<TerrainTileRenderer | null>(null);
  const stroke = useMemo(() => new TerrainBrushStroke(), []);
  const [geometries, setGeometries] = useState<Map<string, BufferGeometry>>(new Map());
  const material = useMemo(() => new MeshStandardMaterial({ map: getHypsometricTexture(), side: DoubleSide, roughness: 1, metalness: 0 }), []);

  useEffect(() => {
    setGeometries(new Map());
    if (!style) return undefined;
    const renderer = new TerrainTileRenderer(style, setGeometries);
    rendererRef.current = renderer;
    void renderer.init();
    return () => {
      renderer.dispose();
      if (rendererRef.current === renderer) rendererRef.current = null;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps -- the camera and style zoom limits only change tile visibility.
  }, [style?.tileUrlTemplate, style?.elevationGridJson, style?.projectOriginLon, style?.projectOriginLat, style?.exaggeration]);

  useEffect(() => {
    if (!style) return;
    rendererRef.current?.scheduleRefresh(JSON.stringify({ position: cameraPosition, target: cameraTarget, minZoom: style.minZoom, maxZoom: style.maxZoom }));
  }, [style, cameraPosition, cameraTarget]);

  useEffect(() => {
    if (!onOriginElevation) return;
    for (const key of geometries.keys()) {
      const elevation = rendererRef.current?.getOriginElevation(key);
      if (elevation === undefined) continue;
      onOriginElevation(elevation);
      break;
    }
  }, [geometries, onOriginElevation]);

  useEffect(() => {
    const cancel = () => stroke.cancel();
    if (!brush?.key || !style?.elevationGridJson) return cancel;
    const keyDown = (event: KeyboardEvent) => { if (event.key === "Escape") cancel(); };
    const pointerCancel = (event: PointerEvent) => stroke.cancel(event.pointerId);
    const captureLost = (event: PointerEvent) => stroke.captureLost(event.pointerId, window.requestAnimationFrame.bind(window));
    window.addEventListener("blur", cancel);
    window.addEventListener("keydown", keyDown);
    window.addEventListener("pointercancel", pointerCancel, true);
    window.addEventListener("lostpointercapture", captureLost, true);
    return () => {
      window.removeEventListener("blur", cancel);
      window.removeEventListener("keydown", keyDown);
      window.removeEventListener("pointercancel", pointerCancel, true);
      window.removeEventListener("lostpointercapture", captureLost, true);
      cancel();
    };
  }, [stroke, brush?.key, style?.elevationGridJson, style?.projectOriginLon, style?.projectOriginLat, style?.exaggeration]);

  if (!style) return null;

  return (
    <group>
      {[...geometries.entries()].map(([key, geometry]) => (
        <TerrainTileVisual key={key} tileKey={key} geometry={geometry} material={material} renderer={rendererRef.current} overlaysJson={overlaysJson} onFeatureHover={onFeatureHover} stroke={stroke} radius={style.elevationGridJson && onElevationStroke ? brush?.radius : undefined} onElevationStroke={onElevationStroke} />
      ))}
    </group>
  );
}

function TerrainTileVisual({ tileKey, geometry, material, renderer, overlaysJson, onFeatureHover, stroke, radius, onElevationStroke }: {
  readonly tileKey: string;
  readonly geometry: BufferGeometry;
  readonly material: MeshStandardMaterial;
  readonly renderer: TerrainTileRenderer | null;
  readonly overlaysJson: string | undefined;
  readonly onFeatureHover?: (name: string | null) => void;
  readonly stroke: TerrainBrushStroke;
  readonly radius: number | undefined;
  readonly onElevationStroke?: (samples: readonly TerrainBrushSample[]) => void;
}) {
  const overlays = useMemo(() => overlaysJson && renderer ? JSON.parse(renderer.getTerrainOverlayTile(tileKey, overlaysJson)) as TerrainOverlayTile | null : null, [renderer, tileKey, geometry, overlaysJson]);
  const positions = geometry.getAttribute("position");
  const width = positions.getX(positions.count - 1) - positions.getX(0);
  return (
    <group>
      <mesh
        geometry={geometry}
        material={material}
        receiveShadow
        onPointerDown={radius === undefined ? undefined : (event) => {
          if (event.button !== 0 || event.isPrimary === false) return;
          const sample = renderer?.pickElevationSample(event.ray.origin.toArray(), event.ray.direction.toArray());
          if (!sample) return;
          const { pointerId } = event;
          const target = event.target as unknown as Pick<Element, "hasPointerCapture" | "setPointerCapture" | "releasePointerCapture"> | null;
          const source = event.nativeEvent.target;
          if (!target || !(source instanceof HTMLElement)) return;
          const otherTouch = (touch: PointerEvent) => {
            if (touch.pointerType === "touch" && touch.pointerId !== pointerId) stroke.cancel(pointerId);
          };
          if (!stroke.begin(pointerId, radius, sample, () => {
            source.removeEventListener("pointerdown", otherTouch);
            if (target.hasPointerCapture(pointerId)) target.releasePointerCapture(pointerId);
          })) return;
          try {
            target.setPointerCapture(pointerId);
            if (event.pointerType === "touch") source.addEventListener("pointerdown", otherTouch);
            event.stopPropagation();
          } catch {
            stroke.cancel(pointerId);
          }
        }}
        onPointerMove={radius === undefined ? undefined : (event) => {
          if (!stroke.owns(event.pointerId)) return;
          event.stopPropagation();
          stroke.move(event.pointerId, renderer?.pickElevationSample(event.ray.origin.toArray(), event.ray.direction.toArray()));
        }}
        onPointerUp={radius === undefined ? undefined : (event) => {
          if (!stroke.owns(event.pointerId)) return;
          event.stopPropagation();
          if (event.button !== 0) return;
          const samples = stroke.finish(event.pointerId, renderer?.pickElevationSample(event.ray.origin.toArray(), event.ray.direction.toArray()));
          if (samples) onElevationStroke?.(samples);
        }}
      />
      {overlays && width > 0 ? <TerrainMapFeatures tile={overlays} tileWidth={width} onFeatureHover={onFeatureHover} /> : null}
    </group>
  );
}
//#endregion WorldTerrainLayer
//#endregion 🔖️WorldTerrainLayerHost
