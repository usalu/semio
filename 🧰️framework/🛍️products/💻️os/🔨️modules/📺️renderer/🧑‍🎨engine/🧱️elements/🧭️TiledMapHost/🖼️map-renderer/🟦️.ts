/** 🖼️ TiledMapHost renderer: the `MapRenderer` that drives the map session, fetches and caches raster and vector tiles and uploads them. */
import { publishedPageUrl } from "@semio-tech/framework/assets/published-page";
import { type MapWasmSession } from "../../🪪️WasmSessionLoader/🟦️.tsx";
import { EASED_SURFACE_TRAILING_WINDOW_MS, createDemandFrameScheduler } from "@semio-tech/framework/canvas/react-renderer";
import { type MapCamera, type MapOverlayLayerInfo, type MapRenderMode, MAX_CONCURRENT_TILE_FETCHES, MAX_RASTER_TILE_BYTES_CACHED, MAX_TILE_MISS_ENTRIES, MAX_VECTOR_TILE_BYTES_CACHED, MAX_VECTOR_TILE_MISS_ENTRIES, parseCameraJson, parseMapOverlayIndex, parseVisibleTilesJson, syncMapInteraction, TILE_REFRESH_DEBOUNCE_MS, type VisibleTileRow } from "../📐️map-parsing/🟦️.ts";
import { serializeMapCanvasThemeJson } from "../🎨️map-theme/🟦️.ts";
import { createBoundedSet, createByteLru, createLeadingTrailingDebounce, type LeadingTrailingDebounce } from "../🗃️tile-cache/🟦️.ts";

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
  private refreshInFlight: Promise<void> | null = null;
  private refreshGeneration = 0;
  private lastRasterVisibleRevision = Number.NaN;
  private lastVectorVisibleRevision = Number.NaN;
  private lastPolledRasterVisibleRevision = Number.NaN;
  private lastPolledVectorVisibleRevision = Number.NaN;
  private tilesRefreshQueued = false;
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
    this.session.setLodMode(mode);
    this.tileMiss.clear();
    this.vectorTileMiss.clear();
    this.lastRasterVisibleRevision = Number.NaN;
    this.lastVectorVisibleRevision = Number.NaN;
    this.lastPolledRasterVisibleRevision = Number.NaN;
    this.lastPolledVectorVisibleRevision = Number.NaN;
    this.scheduleRefreshTiles();
  }

  async attach(canvas: HTMLCanvasElement, width: number, height: number, dpr: number): Promise<void> {
    this.canvasEl = canvas;
    const lw = Math.max(1, Math.round(width));
    const lh = Math.max(1, Math.round(height));
    const nextDpr = dpr > 0 ? dpr : 1;
    this.logicalWidth = lw;
    this.logicalHeight = lh;
    this.dpr = nextDpr;
    this.applyCanvasPixelSize(lw, lh, nextDpr);
    await this.session.attachCanvas(canvas, lw, lh, nextDpr);
  }

  setSize(width: number, height: number, dpr: number): boolean {
    const lw = Math.max(1, Math.round(width));
    const lh = Math.max(1, Math.round(height));
    const nextDpr = dpr > 0 ? dpr : 1;
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

  /** 🪶️ REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT: schedules one render (plus a brief trailing
   * window — see `createDemandFrameScheduler`) instead of relying on the permanent loop this class
   * used to run. Safe to call before `startLoop()`/after `stopLoop()` — it's a no-op until the
   * scheduler exists. */
  invalidate(): void {
    this.scheduler?.invalidate();
  }

  /** 🪶️ Held down for the duration of a pan/zoom gesture (see `TiledMapHost`'s pointer handlers) so
   * tile visibility keeps getting polled every frame while the viewport is actually moving, instead
   * of only on each discrete `invalidate()`. */
  beginContinuousInteraction(reason: string): void {
    this.scheduler?.beginContinuous(reason);
  }

  endContinuousInteraction(reason: string): void {
    this.scheduler?.endContinuous(reason);
    this.invalidate();
  }

  /** 🪶️ Public wrapper so `TiledMapHost` can drive this from `useCanvasAppearanceSync` (fires on
   * actual theme changes) instead of the old per-frame call inside the render loop. */
  syncTheme(): void {
    this.syncMapThemeFromDocument();
    this.invalidate();
  }

  private needsRasterTiles(): boolean {
    return this.renderMode === "image" || this.renderMode === "combined";
  }

  private needsVectorTiles(): boolean {
    return this.renderMode === "vector" || this.renderMode === "combined";
  }

  scheduleRefreshTiles(): void {
    if (this.disposed || !this.canvasEl) return;
    this.tileRefreshDebounce.call();
  }

  async refreshTiles(): Promise<void> {
    if (this.disposed || !this.canvasEl) return;
    if (this.refreshInFlight) {
      this.tilesRefreshQueued = true;
      return this.refreshInFlight;
    }
    const generation = ++this.refreshGeneration;
    this.refreshInFlight = (async () => {
      const tasks: Promise<void>[] = [];
      if (this.needsRasterTiles()) tasks.push(this.refreshRasterTiles());
      if (this.needsVectorTiles()) tasks.push(this.refreshVectorTiles());
      await Promise.all(tasks);
      this.invalidate();
      await this.prefetchTiles(generation);
    })().finally(() => {
      this.refreshInFlight = null;
      if (this.tilesRefreshQueued) {
        this.tilesRefreshQueued = false;
        void this.refreshTiles();
      }
    });
    return this.refreshInFlight;
  }

  private pollVisibleTilesForRefresh(): void {
    if (!this.canvasEl || !this.session.gpuReady()) return;
    if (this.needsRasterTiles()) {
      const revision = this.session.visibleTilesRevision();
      if (revision !== this.lastPolledRasterVisibleRevision) {
        this.lastPolledRasterVisibleRevision = revision;
        this.scheduleRefreshTiles();
      }
    }
    if (this.needsVectorTiles()) {
      const revision = this.session.visibleVectorTilesRevision();
      if (revision !== this.lastPolledVectorVisibleRevision) {
        this.lastPolledVectorVisibleRevision = revision;
        this.scheduleRefreshTiles();
      }
    }
  }

  /** 🧵️ Fetches (cache-first) then uploads one tile row, skipping the wasm upload entirely when
   * `hasTile`/`hasVectorTile` reports the Rust side already holds it — that call is what used to be
   * unconditional, forcing a full PNG decode / MVT re-parse per tile per refresh even when nothing
   * about the tile had changed. */
  private async uploadTileRow(kind: "raster" | "vector", row: VisibleTileRow, abandoned: () => boolean): Promise<void> {
    const cache = kind === "raster" ? this.tileCache : this.vectorTileCache;
    const miss = kind === "raster" ? this.tileMiss : this.vectorTileMiss;
    const urlTemplate = kind === "raster" ? this.tileUrlTemplate : this.vectorTileUrlTemplate;
    const key = row.key;
    const held = kind === "raster" ? this.session.hasTile(row.z, row.x, row.y) : this.session.hasVectorTile(row.z, row.x, row.y);
    if (held) return;
    let buf = cache.get(key);
    if (!buf) {
      if (miss.has(key)) return;
      const url = urlTemplate.replace("{z}", String(row.z)).replace("{x}", String(row.x)).replace("{y}", String(row.y));
      const res = await fetch(url);
      if (!res.ok) {
        miss.add(key);
        return;
      }
      buf = await res.arrayBuffer();
      cache.set(key, buf);
    }
    if (this.disposed || abandoned()) return;
    const bytes = new Uint8Array(buf);
    if (kind === "raster") this.session.uploadTile(row.z, row.x, row.y, bytes);
    else this.session.uploadVectorTile(row.z, row.x, row.y, bytes);
  }

  private async uploadTileRows(kind: "raster" | "vector", rows: VisibleTileRow[], abandoned: () => boolean): Promise<void> {
    for (let i = 0; i < rows.length; i += MAX_CONCURRENT_TILE_FETCHES) {
      if (abandoned()) return;
      await Promise.all(rows.slice(i, i + MAX_CONCURRENT_TILE_FETCHES).map((row) => this.uploadTileRow(kind, row, abandoned)));
    }
  }

  private async refreshRasterTiles(): Promise<void> {
    if (this.disposed) return;
    const rows = parseVisibleTilesJson(this.session.visibleTilesJson());
    if (rows.length === 0) return;
    const revision = this.session.visibleTilesRevision();
    if (revision !== this.lastRasterVisibleRevision) {
      this.lastRasterVisibleRevision = revision;
      this.tileMiss.clear();
    }
    await this.uploadTileRows("raster", rows, () => this.disposed);
  }

  private async refreshVectorTiles(): Promise<void> {
    if (this.disposed) return;
    const rows = parseVisibleTilesJson(this.session.visibleVectorTilesJson());
    if (rows.length === 0) return;
    const revision = this.session.visibleVectorTilesRevision();
    if (revision !== this.lastVectorVisibleRevision) {
      this.lastVectorVisibleRevision = revision;
      this.vectorTileMiss.clear();
    }
    await this.uploadTileRows("vector", rows, () => this.disposed);
  }

  private async prefetchTiles(generation: number): Promise<void> {
    const abandoned = (): boolean => this.disposed || generation !== this.refreshGeneration;
    if (abandoned()) return;
    const tasks: Promise<void>[] = [];
    if (this.needsRasterTiles()) tasks.push(this.uploadTileRows("raster", parseVisibleTilesJson(this.session.prefetchTilesJson()), abandoned));
    if (this.needsVectorTiles()) tasks.push(this.uploadTileRows("vector", parseVisibleTilesJson(this.session.prefetchVectorTilesJson()), abandoned));
    await Promise.all(tasks);
    if (!abandoned()) this.invalidate();
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

  /** 🪶️ REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT: used to be an unconditional 60fps
   * `requestAnimationFrame` loop for the surface's entire lifetime, no dirty check — see
   * `createDemandFrameScheduler`'s docstring. Theme sync moved off the per-frame path onto
   * `syncTheme()` (driven by `useCanvasAppearanceSync` in `TiledMapHost`, which only fires on actual
   * theme changes); tile-visibility polling still runs every scheduled frame — cheap on its own
   * (two allocation-free wasm revision-number reads, only *schedules* a refresh when a revision
   * actually changed — see `pollVisibleTilesForRefresh`) but now only runs while something has called `invalidate()` or
   * `beginContinuousInteraction()` (pan/zoom gestures — see `TiledMapHost`'s pointer handlers),
   * instead of forever. */
  startLoop(): void {
    if (this.scheduler) return;
    this.scheduler = createDemandFrameScheduler(
      () => {
        if (this.disposed) return;
        this.pollVisibleTilesForRefresh();
        void this.session.renderFrame();
      },
      { trailingWindowMs: EASED_SURFACE_TRAILING_WINDOW_MS },
    );
    this.scheduler.invalidate();
  }

  stopLoop(): void {
    this.scheduler?.dispose();
    this.scheduler = null;
  }

  dispose(): void {
    if (this.disposed) return;
    this.disposed = true;
    this.tileRefreshDebounce.dispose();
    this.stopLoop();
    this.session.free();
    this.canvasEl = null;
  }
}
//#endregion MapRenderer
//#endregion 🔖️TiledMapHost
