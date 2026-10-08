// #region 🧲️Header
// 📏️ Domain-neutral world-3d modelling primitives: annotations, scalar field colouring, pick granularity filter, section plane and sub-element highlight tokens.
// The language-neutral contract is 🧬️schema/📏️world3d-modelling/🔣️.json and 🧫️fixtures/📏️world3d-modelling/🔣️.json; the Rust twin is 🦀️.rs beside this file.
// #endregion 🧲️Header

//#region 🔖️Types
/** 📍️ A world-space point or direction. */
export type World3dVec3 = readonly [number, number, number];

/** 🖥️ A screen-space offset in CSS pixels, x right and y down. */
export type World3dVec2 = readonly [number, number];

/** 🌍️ Caller-supplied text in every supported language; the renderer shows the entry for the active locale. */
export type World3dText = { readonly en: string; readonly de: string };

/** 🎨️ A theme colour token: `neutral` is the foreground, the others the palette token of the same name. */
export type World3dTone = "neutral" | "primary" | "secondary" | "tertiary" | "success" | "warning" | "danger" | "info";

export type World3dDimension = { readonly kind: "dimension"; readonly id: string; readonly from: World3dVec3; readonly to: World3dVec3; readonly offset: World3dVec3; readonly text: World3dText; readonly tone: World3dTone };
export type World3dAngle = { readonly kind: "angle"; readonly id: string; readonly vertex: World3dVec3; readonly directionA: World3dVec3; readonly directionB: World3dVec3; readonly radiusPx: number; readonly text: World3dText; readonly tone: World3dTone };
export type World3dMarkerShape = "dot" | "cross" | "ring";
export type World3dMarker = { readonly kind: "marker"; readonly id: string; readonly position: World3dVec3; readonly shape: World3dMarkerShape; readonly text: World3dText; readonly tone: World3dTone };
export type World3dLeader = { readonly kind: "leader"; readonly id: string; readonly anchor: World3dVec3; readonly labelOffsetPx: World3dVec2; readonly text: World3dText; readonly tone: World3dTone };
export type World3dAnnotation = World3dDimension | World3dAngle | World3dMarker | World3dLeader;

/** 📏️ The annotation layer; every item is listed for assistive technology through its text. */
export type World3dAnnotationLayer = { readonly title?: World3dText; readonly items: readonly World3dAnnotation[] };

export type World3dColorRamp = "viridis" | "inferno" | "coolwarm" | "grayscale";
export type World3dScalarDomain = "vertex" | "face";
export type World3dScalarRange = { readonly min: number; readonly max: number };
export type World3dScalarLegend = { readonly title: World3dText; readonly unit?: string; readonly ticks: number };

/** 🌡️ A scalar analysis field painted as a heatmap on every instance of `meshId`: one value per vertex, or one per triangle. `null` is no data. */
export type World3dScalarField = {
  readonly meshId: string;
  readonly domain: World3dScalarDomain;
  readonly values: readonly (number | null)[];
  readonly ramp: World3dColorRamp;
  readonly range: World3dScalarRange;
  readonly legend: World3dScalarLegend;
};

export type World3dPickGranularity = "shape" | "face" | "edge" | "vertex";
export type World3dPickTargets = { readonly mesh: boolean; readonly face: boolean; readonly edge: boolean; readonly vertex: boolean };

/** ✂️ A section plane through `origin`; geometry on the side `normal` points to is removed. */
export type World3dSection = { readonly origin: World3dVec3; readonly normal: World3dVec3; readonly cap?: { readonly tone: World3dTone } };

export type World3dSubElementStyle = { readonly hover?: World3dTone; readonly selected?: World3dTone; readonly widthPx?: number };
export type World3dHighlight = { readonly face?: World3dSubElementStyle; readonly edge?: World3dSubElementStyle; readonly vertex?: World3dSubElementStyle };
export type World3dModellingOptions = { readonly pickFilter?: World3dPickGranularity; readonly section?: World3dSection; readonly highlight?: World3dHighlight };
//#endregion 🔖️Types

//#region 🔖️Limits
export const WORLD3D_ANNOTATIONS_MAX = 512;
export const WORLD3D_SCALAR_VALUES_MAX = 4_000_000;
export const WORLD3D_LEGEND_TICKS_DEFAULT = 5;
export const WORLD3D_ANGLE_RADIUS_PX_DEFAULT = 48;
export const WORLD3D_SCALAR_NO_DATA_HEX = "#808080";
//#endregion 🔖️Limits

//#region 🔖️Parsing
const TONES: ReadonlySet<string> = new Set(["neutral", "primary", "secondary", "tertiary", "success", "warning", "danger", "info"]);
const MARKER_SHAPES: ReadonlySet<string> = new Set(["dot", "cross", "ring"]);
const RAMPS: ReadonlySet<string> = new Set(["viridis", "inferno", "coolwarm", "grayscale"]);
const GRANULARITIES: ReadonlySet<string> = new Set(["shape", "face", "edge", "vertex"]);

function record(value: unknown, allowed: readonly string[]): Record<string, unknown> | null {
  if (value === null || typeof value !== "object" || Array.isArray(value)) return null;
  const entries = value as Record<string, unknown>;
  for (const key of Object.keys(entries)) if (!allowed.includes(key)) return null;
  return entries;
}

function finite(value: unknown): value is number {
  return typeof value === "number" && Number.isFinite(value);
}

function vec(value: unknown, length: 2): World3dVec2 | null;
function vec(value: unknown, length: 3): World3dVec3 | null;
function vec(value: unknown, length: 2 | 3): readonly number[] | null {
  if (!Array.isArray(value) || value.length !== length || !value.every(finite)) return null;
  return value as readonly number[];
}

function nonZero(vector: World3dVec3): boolean {
  return vector[0] !== 0 || vector[1] !== 0 || vector[2] !== 0;
}

function text(value: unknown): World3dText | null {
  const entries = record(value, ["en", "de"]);
  if (!entries || typeof entries.en !== "string" || typeof entries.de !== "string") return null;
  return entries.en.trim() && entries.de.trim() ? { en: entries.en, de: entries.de } : null;
}

function tone(value: unknown, fallback?: World3dTone): World3dTone | null {
  if (value === undefined) return fallback ?? null;
  return typeof value === "string" && TONES.has(value) ? (value as World3dTone) : null;
}

function id(value: unknown): string | null {
  return typeof value === "string" && value.length > 0 ? value : null;
}

function annotation(value: unknown): World3dAnnotation | null {
  if (value === null || typeof value !== "object" || Array.isArray(value)) return null;
  const kind = (value as Record<string, unknown>).kind;
  if (kind === "dimension") {
    const entries = record(value, ["kind", "id", "from", "to", "offset", "text", "tone"]);
    const [identity, from, to, offset, label, color] = entries ? [id(entries.id), vec(entries.from, 3), vec(entries.to, 3), vec(entries.offset, 3), text(entries.text), tone(entries.tone, "neutral")] : [];
    if (!identity || !from || !to || !offset || !label || !color) return null;
    return from[0] === to[0] && from[1] === to[1] && from[2] === to[2] ? null : { kind, id: identity, from, to, offset, text: label, tone: color };
  }
  if (kind === "angle") {
    const entries = record(value, ["kind", "id", "vertex", "directionA", "directionB", "radiusPx", "text", "tone"]);
    if (!entries) return null;
    const radiusPx = entries.radiusPx === undefined ? WORLD3D_ANGLE_RADIUS_PX_DEFAULT : entries.radiusPx;
    const [identity, vertex, directionA, directionB, label, color] = [id(entries.id), vec(entries.vertex, 3), vec(entries.directionA, 3), vec(entries.directionB, 3), text(entries.text), tone(entries.tone, "neutral")];
    if (!identity || !vertex || !directionA || !directionB || !label || !color || !finite(radiusPx) || radiusPx < 16 || radiusPx > 256) return null;
    return nonZero(directionA) && nonZero(directionB) ? { kind, id: identity, vertex, directionA, directionB, radiusPx, text: label, tone: color } : null;
  }
  if (kind === "marker") {
    const entries = record(value, ["kind", "id", "position", "shape", "text", "tone"]);
    if (!entries) return null;
    const shape = entries.shape === undefined ? "dot" : entries.shape;
    const [identity, position, label, color] = [id(entries.id), vec(entries.position, 3), text(entries.text), tone(entries.tone, "neutral")];
    if (!identity || !position || !label || !color || typeof shape !== "string" || !MARKER_SHAPES.has(shape)) return null;
    return { kind, id: identity, position, shape: shape as World3dMarkerShape, text: label, tone: color };
  }
  if (kind === "leader") {
    const entries = record(value, ["kind", "id", "anchor", "labelOffsetPx", "text", "tone"]);
    if (!entries) return null;
    const [identity, anchor, labelOffsetPx, label, color] = [id(entries.id), vec(entries.anchor, 3), vec(entries.labelOffsetPx, 2), text(entries.text), tone(entries.tone, "neutral")];
    return identity && anchor && labelOffsetPx && label && color ? { kind, id: identity, anchor, labelOffsetPx, text: label, tone: color } : null;
  }
  return null;
}

/** 📏️ Total reader of the annotation layer lane: the normalized layer, or `null` for anything the schema or its semantic rules refuse. */
export function parseWorld3dAnnotationLayer(value: unknown): World3dAnnotationLayer | null {
  const entries = record(value, ["title", "items"]);
  if (!entries || !Array.isArray(entries.items) || entries.items.length > WORLD3D_ANNOTATIONS_MAX) return null;
  const title = entries.title === undefined ? undefined : text(entries.title);
  if (title === null) return null;
  const items: World3dAnnotation[] = [];
  const identities = new Set<string>();
  for (const item of entries.items) {
    const parsed = annotation(item);
    if (!parsed || identities.has(parsed.id)) return null;
    identities.add(parsed.id);
    items.push(parsed);
  }
  return title ? { title, items } : { items };
}

/** 🌡️ Total reader of the scalar field lane: the normalized field, or `null`. */
export function parseWorld3dScalarField(value: unknown): World3dScalarField | null {
  const entries = record(value, ["meshId", "domain", "values", "ramp", "range", "legend"]);
  if (!entries) return null;
  const meshId = id(entries.meshId);
  const { domain, ramp, values } = entries;
  if (!meshId || (domain !== "vertex" && domain !== "face") || typeof ramp !== "string" || !RAMPS.has(ramp)) return null;
  if (!Array.isArray(values) || values.length === 0 || values.length > WORLD3D_SCALAR_VALUES_MAX) return null;
  for (const entry of values) if (entry !== null && !finite(entry)) return null;
  const range = record(entries.range, ["min", "max"]);
  if (!range || !finite(range.min) || !finite(range.max) || !(range.min < range.max)) return null;
  const legend = record(entries.legend, ["title", "unit", "ticks"]);
  const title = legend ? text(legend.title) : null;
  if (!legend || !title) return null;
  const ticks = legend.ticks === undefined ? WORLD3D_LEGEND_TICKS_DEFAULT : legend.ticks;
  if (typeof ticks !== "number" || !Number.isInteger(ticks) || ticks < 2 || ticks > 9) return null;
  if (legend.unit !== undefined && (typeof legend.unit !== "string" || legend.unit.length === 0)) return null;
  return {
    meshId,
    domain,
    values: values as readonly (number | null)[],
    ramp: ramp as World3dColorRamp,
    range: { min: range.min, max: range.max },
    legend: typeof legend.unit === "string" ? { title, unit: legend.unit, ticks } : { title, ticks },
  };
}

function section(value: unknown): World3dSection | null {
  const entries = record(value, ["origin", "normal", "cap"]);
  const origin = entries ? vec(entries.origin, 3) : null;
  const normal = entries ? vec(entries.normal, 3) : null;
  if (!entries || !origin || !normal || !nonZero(normal)) return null;
  if (entries.cap === undefined) return { origin, normal };
  const cap = record(entries.cap, ["tone"]);
  const color = cap ? tone(cap.tone, "neutral") : null;
  return color ? { origin, normal, cap: { tone: color } } : null;
}

function subElementStyle(value: unknown): World3dSubElementStyle | null {
  const entries = record(value, ["hover", "selected", "widthPx"]);
  if (!entries) return null;
  const hover = entries.hover === undefined ? undefined : tone(entries.hover);
  const selected = entries.selected === undefined ? undefined : tone(entries.selected);
  if (hover === null || selected === null) return null;
  const widthPx = entries.widthPx;
  if (widthPx !== undefined && (!finite(widthPx) || widthPx < 1 || widthPx > 8)) return null;
  return { ...(hover ? { hover } : {}), ...(selected ? { selected } : {}), ...(widthPx === undefined ? {} : { widthPx }) };
}

/** ⚙️ Total reader of the modelling options lane (pick filter, section plane, highlight tokens): the normalized options, or `null`. */
export function parseWorld3dModellingOptions(value: unknown): World3dModellingOptions | null {
  const entries = record(value, ["pickFilter", "section", "highlight"]);
  if (!entries) return null;
  const options: { pickFilter?: World3dPickGranularity; section?: World3dSection; highlight?: World3dHighlight } = {};
  if (entries.pickFilter !== undefined) {
    if (typeof entries.pickFilter !== "string" || !GRANULARITIES.has(entries.pickFilter)) return null;
    options.pickFilter = entries.pickFilter as World3dPickGranularity;
  }
  if (entries.section !== undefined) {
    const parsed = section(entries.section);
    if (!parsed) return null;
    options.section = parsed;
  }
  if (entries.highlight !== undefined) {
    const highlight = record(entries.highlight, ["face", "edge", "vertex"]);
    if (!highlight) return null;
    const styles: { face?: World3dSubElementStyle; edge?: World3dSubElementStyle; vertex?: World3dSubElementStyle } = {};
    for (const key of ["face", "edge", "vertex"] as const) {
      if (highlight[key] === undefined) continue;
      const style = subElementStyle(highlight[key]);
      if (!style) return null;
      styles[key] = style;
    }
    options.highlight = styles;
  }
  return options;
}
//#endregion 🔖️Parsing

//#region 🔖️Text
/** 🌍️ The entry for the active locale (`de`, `de-CH` → German; everything else → English, the first language). */
export function resolveWorld3dText(value: World3dText, locale: string): string {
  return locale.toLowerCase().split("-")[0] === "de" ? value.de : value.en;
}
//#endregion 🔖️Text

//#region 🔖️Ramps
/** 🎨️ Equally spaced sRGB stops of each named ramp; colours interpolate linearly per channel. */
export const WORLD3D_COLOR_RAMPS: Readonly<Record<World3dColorRamp, readonly string[]>> = {
  viridis: ["#440154", "#3b528b", "#21918c", "#5ec962", "#fde725"],
  inferno: ["#000004", "#57106e", "#bc3754", "#f98e09", "#fcffa4"],
  coolwarm: ["#3b4cc0", "#8db0fe", "#dddddd", "#f4987a", "#b40426"],
  grayscale: ["#000000", "#ffffff"],
};

const RAMP_CHANNELS: Readonly<Record<World3dColorRamp, readonly (readonly [number, number, number])[]>> = Object.fromEntries(
  Object.entries(WORLD3D_COLOR_RAMPS).map(([name, stops]) => [name, stops.map((hex) => [1, 3, 5].map((offset) => Number.parseInt(hex.slice(offset, offset + 2), 16) / 255) as unknown as readonly [number, number, number])]),
) as never;

function toByte(channel: number): number {
  return Math.min(255, Math.max(0, Math.floor(channel * 255 + 0.5)));
}

/** 🎨️ The ramp colour at `t` (clamped to 0..1) as `[r, g, b]` bytes. */
export function world3dRampRgb(ramp: World3dColorRamp, t: number): readonly [number, number, number] {
  const stops = RAMP_CHANNELS[ramp];
  const segments = stops.length - 1;
  const scaled = Math.min(Math.max(t, 0), 1) * segments;
  const index = Math.min(Math.floor(scaled), segments - 1);
  const fraction = scaled - index;
  const from = stops[index]!;
  const to = stops[index + 1]!;
  return [toByte(from[0] + (to[0] - from[0]) * fraction), toByte(from[1] + (to[1] - from[1]) * fraction), toByte(from[2] + (to[2] - from[2]) * fraction)];
}

function hexOf(rgb: readonly number[]): string {
  return `#${rgb.map((channel) => channel.toString(16).padStart(2, "0")).join("")}`;
}

/** 🎨️ {@link world3dRampRgb} as `#rrggbb`. */
export function world3dRampHex(ramp: World3dColorRamp, t: number): string {
  return hexOf(world3dRampRgb(ramp, t));
}

function scalarT(field: World3dScalarField, value: number): number {
  return (value - field.range.min) / (field.range.max - field.range.min);
}

/** 🌡️ The heatmap colour of one value as `#rrggbb`; `null` is the no-data grey. */
export function world3dScalarColorHex(field: World3dScalarField, value: number | null): string {
  return value === null ? WORLD3D_SCALAR_NO_DATA_HEX : world3dRampHex(field.ramp, scalarT(field, value));
}

/** 🌡️ Every value of the field as packed `r, g, b` bytes, ready for a vertex or triangle colour attribute. */
export function world3dScalarFieldColorBytes(field: World3dScalarField): Uint8Array {
  const bytes = new Uint8Array(field.values.length * 3);
  const noData = [0x80, 0x80, 0x80];
  for (let index = 0; index < field.values.length; index += 1) {
    const value = field.values[index]!;
    const rgb = value === null ? noData : world3dRampRgb(field.ramp, scalarT(field, value));
    bytes[index * 3] = rgb[0]!;
    bytes[index * 3 + 1] = rgb[1]!;
    bytes[index * 3 + 2] = rgb[2]!;
  }
  return bytes;
}

/** 🏷️ The legend: `ticks` evenly spaced values from range min to max, each with its ramp colour. */
export function world3dScalarLegend(field: World3dScalarField): readonly { readonly value: number; readonly hex: string }[] {
  const { min, max } = field.range;
  const count = field.legend.ticks;
  return Array.from({ length: count }, (_, index) => {
    const value = min + (index * (max - min)) / (count - 1);
    return { value, hex: world3dRampHex(field.ramp, scalarT(field, value)) };
  });
}
//#endregion 🔖️Ramps

//#region 🔖️Interaction
/** 🎯️ The sub-elements hover and selection may reach; no filter reaches all of them. */
export function world3dPickTargets(filter: World3dPickGranularity | undefined): World3dPickTargets {
  if (filter === undefined) return { mesh: true, face: true, edge: true, vertex: true };
  return { mesh: filter === "shape", face: filter === "face", edge: filter === "edge", vertex: filter === "vertex" };
}

/** ✂️ The plane `[nx, ny, nz, constant]` in three.js clipping convention (negative signed distance is clipped), for geometry on the side `section.normal` points to. */
export function world3dSectionClipPlane(value: World3dSection): readonly [number, number, number, number] {
  const length = Math.hypot(value.normal[0], value.normal[1], value.normal[2]);
  const normal = [value.normal[0] / length, value.normal[1] / length, value.normal[2] / length] as const;
  const constant = normal[0] * value.origin[0] + normal[1] * value.origin[1] + normal[2] * value.origin[2];
  return [-normal[0], -normal[1], -normal[2], constant];
}
//#endregion 🔖️Interaction
