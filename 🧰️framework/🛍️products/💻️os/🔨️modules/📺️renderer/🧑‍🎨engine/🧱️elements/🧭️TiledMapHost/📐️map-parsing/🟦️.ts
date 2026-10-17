/** 📐️ TiledMapHost parsing: the camera, visible tile, layer id, hovered feature and overlay index shapes, the map constants and their parsers, the feature selection and hover action args and the interaction sync resolution. */
import { type SelectionMarqueeMethod, type IconName } from "@semio-tech/ui-react";
import { type MergeMode } from "@semio-tech/framework";
import { type MapWasmSession } from "../../🪪️WasmSessionLoader/🟦️.tsx";

//#region 🔖️TiledMapHost
//#region Types
export type MapCamera = { x: number; y: number; zoom: number };

export type MapRenderMode = "image" | "vector" | "combined";

/** 🎯️ A hovered or hit overlay feature; `kind` is its overlay layer's pickable granularity. */
export type MapHoveredFeature = { readonly kind: string; readonly id: string };

/** 🗂️ Feature ids per pickable overlay layer id — the hit, marquee and selection wire shape. */
export type MapLayerIds = Readonly<Record<string, readonly string[]>>;

/** 🧩️ One overlay layer the descriptor's `policy` declares; only layers with a `granularity` are pickable. */
export type MapOverlayLayerInfo = { readonly id: string; readonly geometry: string; readonly granularity?: string };

/** 🏷️ The popup facts of one pickable overlay feature. */
type MapOverlayMeta = { readonly label?: string; readonly icon?: IconName; readonly link?: string };

/** 📇️ The descriptor's declared overlay layers plus the popup facts of every pickable feature, by granularity then id. */
type MapOverlayIndex = { readonly layers: readonly MapOverlayLayerInfo[]; readonly meta: ReadonlyMap<string, ReadonlyMap<string, MapOverlayMeta>> };

export type VisibleTileRow = { z: number; x: number; y: number; key: string };

export const DEFAULT_CAMERA_JSON = '{"x":0,"y":0,"zoom":1}';
/** 🕹️ The framework-owned interaction domain every tiled-map app declares, its map granularity and
 * the hover channel — the wire coordinates `interactionSelect`/`interactionHover` are addressed
 * with, matched by the guest app's own interaction-domain constants. */
const MAP_INTERACTION_DOMAIN = "features";
const MAP_FEATURE_GRANULARITY = "feature";
const MAP_HOVER_CHANNEL = "pointer";
export const MAP_MARQUEE_THRESHOLD_PX = 6;
export const MAX_CONCURRENT_TILE_FETCHES = 12;
export const TILE_REFRESH_DEBOUNCE_MS = 120;
/** 🪣️ Byte ceilings for the JS-side raster/vector tile caches (`MapRenderer.tileCache` /
 * `vectorTileCache`) — an entry-count cap would be dishonest here since vector tile payloads vary
 * hugely in size; a byte budget makes the memory ceiling explicit. */
export const MAX_RASTER_TILE_BYTES_CACHED = 64 * 1024 * 1024;
export const MAX_VECTOR_TILE_BYTES_CACHED = 96 * 1024 * 1024;
/** 🪣️ Entry caps for `MapRenderer.tileMiss` / `vectorTileMiss` — these only ever grow while panning
 * across a session, so they need a bound too even though a miss record is a bare key, no bytes. */
export const MAX_TILE_MISS_ENTRIES = 2048;
export const MAX_VECTOR_TILE_MISS_ENTRIES = 2048;

export const MAP_VELLO_THEME_FALLBACK_RGBA = {
  surfaceClear: [12, 28, 33, 255] as [number, number, number, number],
  landFill: [46, 60, 61, 255] as [number, number, number, number],
  landStroke: [51, 64, 65, 107] as [number, number, number, number],
  labelFill: [247, 243, 227, 255] as [number, number, number, number],
  labelHalo: [12, 28, 33, 235] as [number, number, number, number],
  regionFill: [52, 209, 191, 56] as [number, number, number, number],
  regionStroke: [52, 209, 191, 230] as [number, number, number, number],
  routeStroke: [250, 149, 0, 235] as [number, number, number, number],
  positionFill: [255, 52, 79, 255] as [number, number, number, number],
  positionStroke: [247, 243, 227, 255] as [number, number, number, number],
  selectionStroke: [255, 52, 79, 255] as [number, number, number, number],
  hoverStroke: [52, 209, 191, 235] as [number, number, number, number],
};

//#endregion Types
//#region Parsing
export function parseVisibleTilesJson(raw: string): VisibleTileRow[] {
  try {
    const rows = JSON.parse(raw) as VisibleTileRow[];
    return Array.isArray(rows) ? rows : [];
  } catch {
    return [];
  }
}

export function parseCameraJson(raw: string): MapCamera | null {
  try {
    const v = JSON.parse(raw) as { x?: number; y?: number; zoom?: number };
    if (typeof v.x !== "number" || typeof v.y !== "number" || typeof v.zoom !== "number") return null;
    return { x: v.x, y: v.y, zoom: v.zoom };
  } catch {
    return null;
  }
}

export function parseMapLayerIds(raw: string): MapLayerIds {
  try {
    const v = JSON.parse(raw) as unknown;
    if (!v || typeof v !== "object" || Array.isArray(v)) return {};
    return Object.fromEntries(Object.entries(v as Record<string, unknown>).map(([layerId, ids]) => [layerId, Array.isArray(ids) ? ids.filter((id): id is string => typeof id === "string") : []]));
  } catch {
    return {};
  }
}

export function mapLayerIdsFlat(hits: MapLayerIds): string[] {
  return Object.values(hits).flat();
}

export function parseMapHoveredFeature(raw: string): MapHoveredFeature | null {
  if (raw === "null") return null;
  try {
    const v = JSON.parse(raw) as { kind?: string; id?: string };
    if (typeof v.kind === "string" && v.kind.length > 0 && typeof v.id === "string") {
      return { kind: v.kind, id: v.id };
    }
  } catch {
    return null;
  }
  return null;
}

export function parseMapScreenPoint(raw: string): { x: number; y: number } | null {
  if (raw === "null") return null;
  try {
    const v = JSON.parse(raw) as { x?: number; y?: number };
    if (typeof v.x !== "number" || typeof v.y !== "number") return null;
    return { x: v.x, y: v.y };
  } catch {
    return null;
  }
}

export const EMPTY_MAP_OVERLAY_INDEX: MapOverlayIndex = { layers: [], meta: new Map() };

/** 📇️ Reads the overlay layers the descriptor's `policy.overlays` declares and the `label`/`icon`/`link` of every feature of a pickable one under `overlays`. */
export function parseMapOverlayIndex(mapDescriptorJson: string): MapOverlayIndex {
  try {
    const descriptor = JSON.parse(mapDescriptorJson) as {
      overlays?: Record<string, Array<{ id?: unknown; label?: unknown; icon?: unknown; link?: unknown }>>;
      policy?: { overlays?: Array<{ id?: unknown; geometry?: unknown; granularity?: unknown }> };
    };
    const text = (value: unknown): string | undefined => (typeof value === "string" ? value : undefined);
    const layers: MapOverlayLayerInfo[] = (descriptor.policy?.overlays ?? []).flatMap((row) => (typeof row.id === "string" ? [{ id: row.id, geometry: text(row.geometry) ?? "point", ...(typeof row.granularity === "string" ? { granularity: row.granularity } : {}) }] : []));
    const meta = new Map<string, Map<string, MapOverlayMeta>>();
    for (const layer of layers) {
      if (!layer.granularity) continue;
      const rows = meta.get(layer.granularity) ?? new Map<string, MapOverlayMeta>();
      for (const row of descriptor.overlays?.[layer.id] ?? []) {
        if (typeof row.id === "string") rows.set(row.id, { label: text(row.label), icon: text(row.icon) as IconName | undefined, link: text(row.link) });
      }
      meta.set(layer.granularity, rows);
    }
    return { layers, meta };
  } catch {
    return EMPTY_MAP_OVERLAY_INDEX;
  }
}

/** 🕹️ `interactionSelect` args for a map feature pick/marquee against the framework-owned
 * `"features"` domain's `"feature"` granularity — the tiled-map twin of `World3dHost`'s
 * `world3dSelectionActionArgs`, and the wire shape a map app's context menu selects with. */
export function mapFeatureSelectionActionArgs(ids: readonly string[], merge: MergeMode, method: "pick" | SelectionMarqueeMethod) {
  const targets = [...new Set(ids)].map((id) => ({ granularity: MAP_FEATURE_GRANULARITY, id }));
  return { domainId: MAP_INTERACTION_DOMAIN, targets: JSON.stringify(targets), merge, method };
}

/** 🐁️ `interactionHover` args on the `"pointer"` channel — the tiled-map twin of `World3dHost`'s
 * `world3dHoverActionArgs`; a null id publishes an empty `targets`, which clears the guest hover. */
export function mapFeatureHoverActionArgs(id: string | null | undefined) {
  const targets = id ? [{ granularity: MAP_FEATURE_GRANULARITY, id }] : [];
  return { domainId: MAP_INTERACTION_DOMAIN, channel: MAP_HOVER_CHANNEL, targets: JSON.stringify(targets) };
}

/** 🕹️ The one granularity the session highlights: the first declared pickable layer with selected ids, else the hovered feature's granularity, else the first pickable one. */
export function resolveMapInteractionSync(selectionJson: string, hoverJson: string, layers: readonly MapOverlayLayerInfo[]): { granularity: string; selectedIdsJson: string; hoveredId?: string } {
  const selection = parseMapLayerIds(selectionJson);
  const hover = parseMapHoveredFeature(hoverJson);
  const pickable = layers.filter((layer) => layer.granularity !== undefined);
  const selected = pickable.find((layer) => (selection[layer.id]?.length ?? 0) > 0);
  const granularity = selected?.granularity ?? hover?.kind ?? pickable[0]?.granularity ?? MAP_FEATURE_GRANULARITY;
  const selectedIds = selected ? selection[selected.id] : [];
  return { granularity, selectedIdsJson: JSON.stringify(selectedIds), ...(hover?.kind === granularity ? { hoveredId: hover.id } : {}) };
}

export function syncMapInteraction(session: MapWasmSession, selectionJson: string, hoverJson: string, layers: readonly MapOverlayLayerInfo[]): void {
  const interaction = resolveMapInteractionSync(selectionJson, hoverJson, layers);
  session.syncInteraction(interaction.granularity, interaction.selectedIdsJson, interaction.hoveredId);
}

export function getTiledMapCameraLimits(session?: MapWasmSession): { min: number; max: number } {
  if (session) {
    return JSON.parse(session.cameraLimitsJson()) as { min: number; max: number };
  }
  return { min: 0.05, max: 64 };
}
//#endregion Parsing
//#endregion 🔖️TiledMapHost
