/** 🖼️ TiledMapHost renderer: the `MapRenderer` that drives the map session, fetches and caches raster and vector tiles and uploads them. */
import { publishedPageUrl } from "@semio-tech/framework/assets/published-page";
import { type MapWasmSession } from "../../🪪️WasmSessionLoader/🟦️.tsx";
import { createDemandFrameScheduler } from "@semio-tech/framework/canvas/react-renderer";
import { type MapCamera, type MapOverlayLayerInfo, type MapRenderMode, MAX_CONCURRENT_TILE_FETCHES, MAX_RASTER_TILE_BYTES_CACHED, MAX_TILE_MISS_ENTRIES, MAX_VECTOR_TILE_BYTES_CACHED, MAX_VECTOR_TILE_MISS_ENTRIES, parseCameraJson, parseMapOverlayIndex, parseVisibleTilesJson, syncMapInteraction, TILE_REFRESH_DEBOUNCE_MS, type VisibleTileRow } from "../📐️map-parsing/🟦️.ts";
import { serializeMapCanvasThemeJson } from "../🎨️map-theme/🟦️.ts";
import { createBoundedSet, createByteLru, createLeadingTrailingDebounce, type LeadingTrailingDebounce } from "../🗃️tile-cache/🟦️.ts";

type TileKind = "raster" | "vector";
type TileRevisions = { raster: number | null; vector: number | null };
type TileRefresh = { revisions: TileRevisions; controller: AbortController; visible: Promise<void> };
const VISIBLE_TILE_RETRY_DELAYS_MS = [250, 1000] as const;
const TILE_FETCH_TIMEOUT_MS = 12_000;

function waitForTileRetry(delayMs: number, signal: AbortSignal): Promise<void> {
  return new Promise((resolve) => {
    const finish = (): void => {
      clearTimeout(timer);
      signal.removeEventListener("abort", finish);
      resolve();
    };
    const timer = setTimeout(finish, delayMs);
    signal.addEventListener("abort", finish, { once: true });
  });
}

//#region 🔖️TiledMapHost
//#region MapRenderer
export class MapRenderer {
  readonly session: MapWasmSession;
  camera: MapCamera = { x: 0, y: 0, zoom: 1 };
  private disposed = false;
  private canvasEl: HTMLCanvasElement | null = null;
  private tileCache = createByteLru(MAX_RASTER_TILE_BYTES_CACHED);
  private vectorTileCache = createByteLru(MAX_VECTOR_TILE_BYTES_CACHED);
  private tileMiss = createBoundedSet(MAX_TILE_MISS_ENTRIES);
  private vectorTileMiss = createBoundedSet(MAX_VECTOR_TILE_MISS_ENTRIES);
  private tileUrlTemplate: string;
  private vectorTileUrlTemplate: string;
  private renderMode: MapRenderMode = "vector";
  private overlayLayers: readonly MapOverlayLayerInfo[] = [];
  private tileRefreshDebounce: LeadingTrailingDebounce;
  private activeRefresh: TileRefresh | null = null;
  private lastRefreshRevisions: TileRevisions = { raster: null, vector: null };
  private lastPolledRevisions: TileRevisions = { raster: null, vector: null };
  private lastMapThemeJson = "";
  private logicalWidth = 1;
  private logicalHeight = 1;
  private dpr = 1;
  private scheduler: ReturnType<typeof createDemandFrameScheduler> | null = null;

  constructor(tileUrlTemplate: string, vectorTileUrlTemplate: string, session: MapWasmSession) {
    this.session = session;
    this.tileUrlTemplate = publishedPageUrl(tileUrlTemplate);
    this.vectorTileUrlTemplate = publishedPageUrl(vectorTileUrlTemplate);
    this.tileRefreshDebounce = createLeadingTrailingDebounce(() => void this.refreshTiles(), TILE_REFRESH_DEBOUNCE_MS);
  }

  private applyCanvasPixelSize(lw: number, lh: number, nextDpr: number): void {
    const canvas = this.canvasEl;
    if (!canvas) return;
    const pw = Math.max(1, Math.round(lw * nextDpr));
    const ph = Math.max(1, Math.round(lh * nextDpr));
    if (canvas.width !== pw || canvas.height !== ph) {
      canvas.width = pw;
      canvas.height = ph;
    }
  }

  setRenderMode(mode: MapRenderMode): void {
    if (mode !== this.renderMode) this.cancelRefresh();
    this.renderMode = mode;
    this.session.setRenderMode(mode);
    this.invalidate();
  }

  setVectorStyle(style: string): void {
    this.session.setVectorStyle(style);
    this.invalidate();
  }

  setLayerVisibilityJson(json: string): void {
    this.session.setLayerVisibilityJson(json);
    this.invalidate();
  }

  setLayerStrokeScaleJson(json: string): void {
    this.session.setLayerStrokeScaleJson(json);
    this.invalidate();
  }

  syncInteraction(selectionJson: string, hoverJson: string): void {
    syncMapInteraction(this.session, selectionJson, hoverJson, this.overlayLayers);
    this.invalidate();
  }

  setLodMode(mode: string): void {
    this.cancelRefresh();
    this.session.setLodMode(mode);
    this.tileMiss.clear();
    this.vectorTileMiss.clear();
    this.lastRefreshRevisions = { raster: null, vector: null };
    this.lastPolledRevisions = { raster: null, vector: null };
    this.invalidate();
    this.scheduleRefreshTiles();
  }

  async attach(canvas: HTMLCanvasElement, width: number, height: number, dpr: number, canvas2d = false): Promise<void> {
    this.canvasEl = canvas;
    const lw = Math.max(1, Math.round(width));
    const lh = Math.max(1, Math.round(height));
    const nextDpr = Math.max(1, dpr);
    this.logicalWidth = lw;
    this.logicalHeight = lh;
    this.dpr = nextDpr;
    this.applyCanvasPixelSize(lw, lh, nextDpr);
    if (canvas2d) this.session.attachCanvas2d(canvas, lw, lh, nextDpr);
    else await this.session.attachCanvas(canvas, lw, lh, nextDpr);
  }

  setSize(width: number, height: number, dpr: number): boolean {
    const lw = Math.max(1, Math.round(width));
    const lh = Math.max(1, Math.round(height));
    const nextDpr = Math.max(1, dpr);
    if (!this.canvasEl) return false;
    if (lw === this.logicalWidth && lh === this.logicalHeight && nextDpr === this.dpr) return false;
    this.logicalWidth = lw;
    this.logicalHeight = lh;
    this.dpr = nextDpr;
    this.applyCanvasPixelSize(lw, lh, nextDpr);
    this.session.setSize(lw, lh, nextDpr);
    this.session.reclampCamera();
    const parsed = this.readCameraFromSession();
    if (parsed) this.camera = parsed;
    this.invalidate();
    return true;
  }

  syncDescriptor(json: string): void {
    this.session.syncMapJson(json);
    this.overlayLayers = parseMapOverlayIndex(json).layers;
    this.invalidate();
  }

  readCameraFromSession(): MapCamera | null {
    return parseCameraJson(this.session.cameraJson());
  }

  applyCameraToSession(camera: MapCamera): void {
    this.camera = camera;
    this.session.setCamera(camera.x, camera.y, camera.zoom);
    this.invalidate();
  }

  /** 🪶️ Schedules one paint on the next frame when the scheduler is active. */
  invalidate(): void {
    this.scheduler?.invalidate();
  }

  /** 🪶️ Public wrapper so `TiledMapHost` can drive this from `useCanvasAppearanceSync` (fires on
   * actual theme changes) instead of the old per-frame call inside the render loop. */
  syncTheme(): void {
    this.syncMapThemeFromDocument();
    this.invalidate();
  }

  private visibleRasterRevision(): number | null {
    return this.renderMode === "image" || this.renderMode === "combined" ? this.session.visibleTilesRevision() : null;
  }

  private visibleVectorRevision(): number | null {
    return this.renderMode === "vector" || this.renderMode === "combined" ? this.session.visibleVectorTilesRevision() : null;
  }

  scheduleRefreshTiles(): void {
    if (this.disposed || !this.canvasEl) return;
    this.tileRefreshDebounce.call();
  }

  async refreshTiles(): Promise<void> {
    if (this.disposed || !this.canvasEl) return;
    const revisions = { raster: this.visibleRasterRevision(), vector: this.visibleVectorRevision() };
    const active = this.activeRefresh;
    if (active?.revisions.raster === revisions.raster && active.revisions.vector === revisions.vector) return active.visible;
    this.cancelRefresh();
    this.lastPolledRevisions = revisions;
    if (revisions.raster !== null && revisions.raster !== this.lastRefreshRevisions.raster) {
      this.tileMiss.clear();
      this.lastRefreshRevisions.raster = revisions.raster;
    }
    if (revisions.vector !== null && revisions.vector !== this.lastRefreshRevisions.vector) {
      this.vectorTileMiss.clear();
      this.lastRefreshRevisions.vector = revisions.vector;
    }

    const controller = new AbortController();
    const tasks: Promise<void>[] = [];
    if (revisions.raster !== null) tasks.push(this.refreshVisibleRows("raster", parseVisibleTilesJson(this.session.visibleTilesJson()), controller.signal));
    if (revisions.vector !== null) tasks.push(this.refreshVisibleRows("vector", parseVisibleTilesJson(this.session.visibleVectorTilesJson()), controller.signal));
    const visible = Promise.all(tasks).then(() => undefined);
    const refresh = { revisions, controller, visible };
    this.activeRefresh = refresh;
    void visible.then(() => this.prefetchTiles(revisions, controller.signal)).finally(() => {
      if (this.activeRefresh === refresh) this.activeRefresh = null;
    });
    return visible;
  }

  private cancelRefresh(): void {
    this.activeRefresh?.controller.abort();
    this.activeRefresh = null;
  }

  private pollVisibleTilesForRefresh(): void {
    if (!this.canvasEl || !this.session.rendererReady()) return;
    const raster = this.visibleRasterRevision();
    const vector = this.visibleVectorRevision();
    if (raster === this.lastPolledRevisions.raster && vector === this.lastPolledRevisions.vector) return;
    this.lastPolledRevisions = { raster, vector };
    this.scheduleRefreshTiles();
  }

  /** 🧵️ Fetches a tile only when the Wasm session needs it. Returns whether the request can be retried. */
  private async uploadTileRow(kind: TileKind, row: VisibleTileRow, signal: AbortSignal, retry: boolean): Promise<boolean> {
    if (signal.aborted) return false;
    const cache = kind === "raster" ? this.tileCache : this.vectorTileCache;
    const miss = kind === "raster" ? this.tileMiss : this.vectorTileMiss;
    const urlTemplate = kind === "raster" ? this.tileUrlTemplate : this.vectorTileUrlTemplate;
    const key = row.key;
    const held = kind === "raster" ? this.session.hasTile(row.z, row.x, row.y) : this.session.hasVectorTile(row.z, row.x, row.y);
    if (held || miss.has(key)) return false;
    const cached = cache.get(key);
    let buf = cached;
    try {
      if (!buf) {
        const url = urlTemplate.replace("{z}", String(row.z)).replace("{x}", String(row.x)).replace("{y}", String(row.y));
        const request = new AbortController();
        const abortRequest = (): void => {
          clearTimeout(deadline);
          request.abort();
        };
        const deadline = setTimeout(abortRequest, TILE_FETCH_TIMEOUT_MS);
        signal.addEventListener("abort", abortRequest, { once: true });
        try {
          const res = await fetch(url, { signal: request.signal, cache: retry ? "reload" : "default" });
          if (request.signal.aborted) return !signal.aborted;
          if (!res.ok) {
            const status = res.status;
            if (status === 404 || status === 410) miss.add(key);
            return status === 408 || status === 425 || status === 429 || status >= 500;
          }
          buf = await res.arrayBuffer();
          if (request.signal.aborted) return !signal.aborted;
        } finally {
          clearTimeout(deadline);
          signal.removeEventListener("abort", abortRequest);
        }
      }
      if (signal.aborted) return false;
      const bytes = new Uint8Array(buf);
      if (kind === "raster") this.session.uploadTile(row.z, row.x, row.y, bytes);
      else this.session.uploadVectorTile(row.z, row.x, row.y, bytes);
      if (!cached) cache.set(key, buf);
      this.invalidate();
      return false;
    } catch {
      if (signal.aborted) return false;
      cache.delete(key);
      return true;
    }
  }

  private async uploadTileRows(kind: TileKind, rows: VisibleTileRow[], signal: AbortSignal, retry = false): Promise<VisibleTileRow[]> {
    let next = 0;
    const failed = new Array<boolean>(rows.length);
    const worker = async (): Promise<void> => {
      while (next < rows.length && !signal.aborted) {
        const index = next++;
        failed[index] = await this.uploadTileRow(kind, rows[index], signal, retry);
      }
    };
    await Promise.all(Array.from({ length: Math.min(rows.length, MAX_CONCURRENT_TILE_FETCHES) }, worker));
    return rows.filter((_, index) => failed[index]);
  }

  private async refreshVisibleRows(kind: TileKind, rows: VisibleTileRow[], signal: AbortSignal): Promise<void> {
    let failed = await this.uploadTileRows(kind, rows, signal);
    for (const delay of VISIBLE_TILE_RETRY_DELAYS_MS) {
      if (!failed.length || signal.aborted) return;
      await waitForTileRetry(delay, signal);
      failed = await this.uploadTileRows(kind, failed, signal, true);
    }
  }

  private async prefetchTiles(revisions: TileRevisions, signal: AbortSignal): Promise<void> {
    if (signal.aborted) return;
    const tasks: Promise<VisibleTileRow[]>[] = [];
    if (revisions.raster !== null) tasks.push(this.uploadTileRows("raster", parseVisibleTilesJson(this.session.prefetchTilesJson()), signal));
    if (revisions.vector !== null) tasks.push(this.uploadTileRows("vector", parseVisibleTilesJson(this.session.prefetchVectorTilesJson()), signal));
    await Promise.all(tasks);
  }

  private syncMapThemeFromDocument(): void {
    if (typeof document === "undefined") return;
    try {
      const json = serializeMapCanvasThemeJson();
      if (json !== this.lastMapThemeJson) {
        this.lastMapThemeJson = json;
        this.session.setMapThemeJson(json);
      }
    } catch {
      this.lastMapThemeJson = "";
    }
  }

  /** 🕰️ Starts painting in response to map changes. The map session applies camera changes synchronously. */
  startLoop(onFailure?: () => void): void {
    if (this.scheduler) return;
    this.scheduler = createDemandFrameScheduler(() => {
      if (this.disposed) return;
      try {
        this.pollVisibleTilesForRefresh();
        this.session.renderFrame();
      } catch (error) {
        if (!onFailure) throw error;
        onFailure();
      }
    });
    this.scheduler.invalidate();
  }

  stopLoop(): void {
    this.scheduler?.dispose();
    this.scheduler = null;
  }

  dispose(): void {
    if (this.disposed) return;
    this.disposed = true;
    this.cancelRefresh();
    this.tileRefreshDebounce.dispose();
    this.stopLoop();
    this.session.free();
    this.canvasEl = null;
  }
}
//#endregion MapRenderer
//#endregion 🔖️TiledMapHost
