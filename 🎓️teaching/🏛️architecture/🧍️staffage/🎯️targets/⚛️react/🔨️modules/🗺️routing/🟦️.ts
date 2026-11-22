/** 🗺️ The address of the archive: which page is open and every meaningful state of it, as a shareable hash such as `#/humans/walking-commuter?view=thermal&scale=50&clip=walk-cycle&t=1.2`.
 *
 * Route: `#/` overview · `#/all` · `#/<category>` · `#/<category>/<asset>` · `#/collections[/<id>]` · `#/compare` · `#/playground` (the section playground) · `#/playground/plan` (the floor plan playground) · `#/playground/3d` (the 3D playground).
 * Parameters carry only what differs from the default, in a stable order, so `format(parse(x))` is canonical and a link reproduces the state: filters (`q cat an h root done sort dir`),
 * the viewer (`view vp cam vl scale pose|clip t zoom pan bg th tq fq range pat fl dg rate loop xs xf xd`, then the air: `env` the reference environment preset, `air mrt sky gnd rh wind wdir sun dif hpa` the values set by hand (°C, per cent, m/s, the bearing the air moves towards in degrees with 0 ahead and 90 to the asset's left, W/m², hPa), `wp z0 zr ti tf tt life seed` the conditions of the flow (`wp=log` the logarithmic wind profile with roughness length and reference height in m, turbulence intensity in per cent, its floor in m/s and its Lagrangian time in s, parcel life in s, seed) and `fsp fst fpe fex ftr fsw` the detail of the flow marks (spacing and shortest streak in paper mm, tracer period and exposure in s, the fading trail of the parcel paths in s, the width of the widest stream in paper mm); `fl` lists the air flow layers of the thermal view, `s` streams, `t` tracers, `m` moisture, `f` fog, `p` parcel paths, `0` none; `vp` is `elevation`, `front`, `plan`, `axonometric` or `model`, `cam` the camera preset of the 3D model), the comparison (`p` per pane) and the playground (`p` per placement, `len`, `dep`, `tl`, `cam`, `vp` of its section tab).
 * The reader's own preferences — language, theme — are never part of a link.
 *
 * @see ../🔎️search/🟦️.ts — filter parameters
 * @see ../💾️persistence/🟦️.ts — the local state that is not shared
 */
import { useCallback, useMemo, useSyncExternalStore } from "react";
import { CATEGORIES, FORCE_QUANTITIES, THERMAL_QUANTITIES, PLANAR_VIEWPOINTS, VIEWS } from "@teaching/staffage";
import type { CategoryId, DisplayRange, FlowLayer, FlowMarkDetail, ForceQuantity, Slug, Text, ThermalQuantity, View, PlanarViewpoint } from "@teaching/staffage";
import { ENVIRONMENT_QUANTITIES, isPreset, readQuantities, writeQuantities, type EnvironmentOverride } from "../🌤️environments/🟦️.ts";
import { SHOWN_FLOW_LAYERS, FLOW_DEFAULTS, FLOW_QUANTITIES, MARK_DEFAULTS, MARK_QUANTITIES, flowLayersCode, flowLayersFrom, type FlowSettings } from "../🌊️flow/🟦️.ts";
import { CAMERA_PRESETS, DEFAULT_CAMERA, isCameraPreset, type CameraPresetName } from "../🧊️spatial/🟦️.ts";
import { isViewpointKey, type ViewpointKey } from "../🔭️viewpoints/🟦️.ts";
import { BACKGROUNDS, DEFAULT_DEPTH_M, DEFAULT_LENGTH_M, SECTION_VIEWPOINTS, TRACK_DEFAULTS, clampTimeline, clampTrack, type Background, type Placement, type Playground, type SectionViewpoint, type Selection } from "../💾️persistence/🟦️.ts";

/** 🧪️ The three playgrounds: the section (side or front elevation), the floor plan and the 3D model, all over one shared scene. */
export const PLAYGROUND_TABS = ["section", "plan", "model"] as const;

/** 📤️ One of {@link PLAYGROUND_TABS}. */
export type PlaygroundTab = (typeof PLAYGROUND_TABS)[number];

const TAB_SEGMENTS: Readonly<Record<PlaygroundTab, string>> = { section: "", plan: "plan", model: "3d" };

/** 📍️ A page of the archive. */
export type Route =
  | { readonly page: "overview" }
  | { readonly page: "all" }
  | { readonly page: "category"; readonly category: CategoryId }
  | { readonly page: "asset"; readonly category: CategoryId; readonly asset: Slug }
  | { readonly page: "collections"; readonly collection: Slug | null }
  | { readonly page: "compare" }
  | { readonly page: "playground"; readonly tab?: PlaygroundTab };

/** 🏠️ The overview. */
export const OVERVIEW: Route = { page: "overview" };

/** 📮️ A route and its query. */
export type Address = { readonly route: Route; readonly params: URLSearchParams };

const SLUG = /^[a-z0-9]+(?:-[a-z0-9]+)*$/u;
const isSlug = (value: string | undefined): value is Slug => value !== undefined && SLUG.test(value);
const isCategory = (value: string | undefined): value is CategoryId => (CATEGORIES as readonly string[]).includes(value ?? "");

/** 🔗️ Parses a hash (with or without `#`); anything unreadable is the overview. */
export function parseAddress(hash: string): Address {
  const text = hash.replace(/^#/u, "");
  const at = text.indexOf("?");
  const path = (at < 0 ? text : text.slice(0, at)).split("/").filter(Boolean);
  const params = new URLSearchParams(at < 0 ? "" : text.slice(at + 1));
  const [first, second] = path;
  let route: Route = OVERVIEW;
  if (first === "all") route = { page: "all" };
  else if (first === "compare") route = { page: "compare" };
  else if (first === "playground") route = second === "plan" ? { page: "playground", tab: "plan" } : second === "3d" ? { page: "playground", tab: "model" } : { page: "playground" };
  else if (first === "collections") route = { page: "collections", collection: isSlug(second) ? second : null };
  else if (isCategory(first)) route = isSlug(second) ? { page: "asset", category: first, asset: second } : { page: "category", category: first };
  return { route, params };
}

/** 🛤️ The path of a route. */
export function routePath(route: Route): string {
  switch (route.page) {
    case "overview":
      return "/";
    case "all":
    case "compare":
      return `/${route.page}`;
    case "playground": {
      const segment = TAB_SEGMENTS[route.tab ?? "section"];
      return segment === "" ? "/playground" : `/playground/${segment}`;
    }
    case "category":
      return `/${route.category}`;
    case "asset":
      return `/${route.category}/${route.asset}`;
    case "collections":
      return route.collection === null ? "/collections" : `/collections/${route.collection}`;
  }
}

/** 🧾️ The hash of a route and its query. */
export function formatAddress(route: Route, params: URLSearchParams = new URLSearchParams()): string {
  const query = params.toString();
  return `#${routePath(route)}${query === "" ? "" : `?${query}`}`;
}

/** 🌳️ The nested pages a route opens, outermost first: `asset` opens its category, which opens the root page. */
export function openedChain(route: Route): readonly string[] {
  switch (route.page) {
    case "overview":
      return [];
    case "category":
      return [route.category];
    case "asset":
      return [route.category, route.asset];
    case "collections":
      return route.collection === null ? ["collections"] : ["collections", route.collection];
    default:
      return [route.page];
  }
}

/** ⬆️ The route one level up. */
export function parentRoute(route: Route): Route | null {
  switch (route.page) {
    case "overview":
      return null;
    case "asset":
      return { page: "category", category: route.category };
    case "collections":
      return route.collection === null ? OVERVIEW : { page: "collections", collection: null };
    default:
      return OVERVIEW;
  }
}

/** 🟰️ Whether two routes are the same page. */
export const sameRoute = (a: Route, b: Route): boolean => routePath(a) === routePath(b);

//#region 🎛️Viewer
/** 🖼️ The ground of a viewer: by view, or the reader's choice. */
export type ViewerBackground = "auto" | Background;

export type { EnvironmentOverride };

/** 📈️ Which member diagrams the force and moment views draw. */
export type DiagramToggles = { readonly axial: boolean; readonly shear: boolean; readonly moment: boolean; readonly freeBody: boolean };

/** 🎛️ Everything the viewer shows besides the asset: view, scale, what is posed or played, zoom, analytic options and the air: the reference environment preset (`null`: the asset's own), the environment values set by hand on top of it, the flow layers, the conditions of the flow and the detail of its marks (each only where it differs from its default). */
export type ViewerState = {
  readonly view: View;
  readonly viewpoint: ViewpointKey;
  readonly camera: CameraPresetName;
  readonly layout: "single" | "all";
  readonly scale: number;
  readonly selection: Selection | null;
  readonly timeS: number | null;
  readonly zoom: "fit" | number;
  readonly pan: { readonly xMm: number; readonly yMm: number };
  readonly background: ViewerBackground;
  readonly theme: "auto" | "light" | "dark";
  readonly thermalQuantity: ThermalQuantity;
  readonly forceQuantity: ForceQuantity;
  readonly range: DisplayRange;
  readonly patterns: boolean;
  readonly diagrams: DiagramToggles;
  readonly preset: Slug | null;
  readonly environment: EnvironmentOverride;
  readonly flow: readonly FlowLayer[];
  readonly flowSettings: FlowSettings;
  readonly flowDetail: FlowMarkDetail;
  readonly rate: number;
  readonly loop: boolean;
  readonly sheet: "single" | "all-views";
  readonly fps: number | null;
  readonly durationS: number | null;
};

/** 🌱️ The viewer a reader first sees: dark line drawing at 1:100, the default clip playing, fitted. */
export const DEFAULT_VIEWER: ViewerState = {
  view: "dark-line",
  viewpoint: "elevation",
  camera: DEFAULT_CAMERA,
  layout: "single",
  scale: 100,
  selection: null,
  timeS: null,
  zoom: "fit",
  pan: { xMm: 0, yMm: 0 },
  background: "auto",
  theme: "auto",
  thermalQuantity: "surface-temperature",
  forceQuantity: "axial",
  range: { mode: "auto" },
  patterns: true,
  diagrams: { axial: false, shear: false, moment: true, freeBody: true },
  preset: null,
  environment: {},
  flow: SHOWN_FLOW_LAYERS,
  flowSettings: {},
  flowDetail: {},
  rate: 1,
  loop: true,
  sheet: "single",
  fps: null,
  durationS: null,
};

/** 📐️ The viewpoint of the surfaces that draw planar viewpoints only (comparison, playground): the side elevation. */
const PLANAR_DEFAULT: PlanarViewpoint = "elevation";
const DIAGRAM_CODES = { n: "axial", v: "shear", m: "moment", fb: "freeBody" } as const;
const isViewpoint = (value: string | null): value is PlanarViewpoint => (PLANAR_VIEWPOINTS as readonly string[]).includes(value ?? "");
const isView = (value: string | null): value is View => (VIEWS as readonly string[]).includes(value ?? "");
const number = (value: string | null): number | null => {
  const parsed = value === null || value.trim() === "" ? Number.NaN : Number(value);
  return Number.isFinite(parsed) ? parsed : null;
};
const clamp = (value: number, low: number, high: number): number => Math.min(Math.max(value, low), high);
const round = (value: number, places: number): number => Number(value.toFixed(places));
const offDefault = <K extends string>(values: Partial<Record<K, number>>, defaults: Readonly<Record<K, number>>): Partial<Record<K, number>> => Object.fromEntries(Object.entries(values).filter(([key, value]) => value !== defaults[key as K])) as Partial<Record<K, number>>;

/** 📥️ The viewer of an address query; unknown or out-of-range values fall back to the default. */
export function viewerFromParams(params: URLSearchParams): ViewerState {
  const base = DEFAULT_VIEWER;
  const view = params.get("view");
  const viewpoint = params.get("vp");
  const camera = params.get("cam");
  const scale = number(params.get("scale"));
  const pose = params.get("pose");
  const clip = params.get("clip");
  const zoom = params.get("zoom");
  const zoomValue = number(zoom);
  const [panX, panY] = (params.get("pan") ?? "").split(",").map(number);
  const bg = params.get("bg");
  const tq = params.get("tq");
  const fq = params.get("fq");
  const [low, high] = (params.get("range") ?? "").split(":").map(number);
  const spanK = number(params.get("span"));
  const diagrams = params.get("dg");
  const t =number(params.get("t"));
  const get = (name: string): string | null => params.get(name);
  const preset = params.get("env");
  return {
    view: isView(view) ? view : base.view,
    viewpoint: isViewpointKey(viewpoint) ? viewpoint : base.viewpoint,
    camera: isCameraPreset(camera) ? camera : base.camera,
    layout: params.get("vl") === "all" ? "all" : base.layout,
    scale: scale !== null && scale >= 1 && scale <= 100000 ? scale : base.scale,
    selection: isSlug(pose ?? undefined) ? { kind: "pose", pose: pose as Slug } : isSlug(clip ?? undefined) ? { kind: "clip", clip: clip as Slug } : null,
    timeS: t !== null && t >= 0 ? t : null,
    zoom: zoomValue !== null && zoomValue > 0 ? clamp(zoomValue, 0.05, 64) : "fit",
    pan: panX !== undefined && panX !== null && panY !== undefined && panY !== null ? { xMm: panX, yMm: panY } : base.pan,
    background: bg === "dark" || bg === "light" || bg === "paper" || bg === "transparent" ? bg : "auto",
    theme: params.get("th") === "light" || params.get("th") === "dark" ? (params.get("th") as "light" | "dark") : "auto",
    thermalQuantity: (THERMAL_QUANTITIES as readonly string[]).includes(tq ?? "") ? (tq as ThermalQuantity) : base.thermalQuantity,
    forceQuantity: (FORCE_QUANTITIES as readonly string[]).includes(fq ?? "") ? (fq as ForceQuantity) : base.forceQuantity,
    range: low !== undefined && low !== null && high !== undefined && high !== null && high > low ? { mode: "fixed", min: low, max: high } : spanK !== null && spanK > 0 ? { mode: "span", spanK } : base.range,
    patterns: params.get("pat") !== "0",
    flow: flowLayersFrom(params.get("fl")),
    diagrams: diagrams === null ? base.diagrams : { axial: false, shear: false, moment: false, freeBody: false, ...Object.fromEntries(diagrams.split(",").flatMap((code) => (code in DIAGRAM_CODES ? [[DIAGRAM_CODES[code as keyof typeof DIAGRAM_CODES], true]] : []))) },
    rate: number(params.get("rate")) !== null && [0.1, 0.25, 0.5, 1, 2].includes(number(params.get("rate"))!) ? number(params.get("rate"))! : 1,
    loop: params.get("loop") !== "0",
    sheet: params.get("xs") === "all" ? "all-views" : "single",
    fps: number(params.get("xf")) !== null ? clamp(number(params.get("xf"))!, 12, 60) : null,
    durationS: number(params.get("xd")) !== null && number(params.get("xd"))! > 0 ? clamp(number(params.get("xd"))!, 0.1, 600) : null,
    preset: isPreset(preset) ? preset : null,
    environment: readQuantities(ENVIRONMENT_QUANTITIES, get, "param"),
    flowSettings: { ...offDefault(readQuantities(FLOW_QUANTITIES, get, "param"), FLOW_DEFAULTS), ...(params.get("wp") === "log" ? { profile: "logarithmic" as const } : {}) },
    flowDetail: offDefault(readQuantities(MARK_QUANTITIES, get, "param"), MARK_DEFAULTS),
  };
}

/** 📤️ Writes the non-default viewer state into a query, in a stable order. */
export function viewerToParams(viewer: ViewerState, into: URLSearchParams = new URLSearchParams()): URLSearchParams {
  const base = DEFAULT_VIEWER;
  if (viewer.view !== base.view) into.set("view", viewer.view);
  if (viewer.viewpoint !== base.viewpoint) into.set("vp", viewer.viewpoint);
  if (viewer.camera !== base.camera) into.set("cam", viewer.camera);
  if (viewer.layout !== base.layout) into.set("vl", viewer.layout);
  if (viewer.scale !== base.scale) into.set("scale", String(round(viewer.scale, 2)));
  if (viewer.selection?.kind === "pose") into.set("pose", viewer.selection.pose);
  if (viewer.selection?.kind === "clip") into.set("clip", viewer.selection.clip);
  if (viewer.timeS !== null) into.set("t", String(round(viewer.timeS, 3)));
  if (viewer.zoom !== "fit") into.set("zoom", String(round(viewer.zoom, 3)));
  if (viewer.pan.xMm !== 0 || viewer.pan.yMm !== 0) into.set("pan", `${round(viewer.pan.xMm, 1)},${round(viewer.pan.yMm, 1)}`);
  if (viewer.background !== "auto") into.set("bg", viewer.background);
  if (viewer.theme !== "auto") into.set("th", viewer.theme);
  if (viewer.thermalQuantity !== base.thermalQuantity) into.set("tq", viewer.thermalQuantity);
  if (viewer.forceQuantity !== base.forceQuantity) into.set("fq", viewer.forceQuantity);
  if (viewer.range.mode === "fixed") into.set("range", `${round(viewer.range.min, 3)}:${round(viewer.range.max, 3)}`);
  if (viewer.range.mode === "span") into.set("span", String(round(viewer.range.spanK, 3)));
  if (!viewer.patterns) into.set("pat", "0");
  const flow = flowLayersCode(viewer.flow);
  if (flow !== null) into.set("fl", flow);
  const codes = Object.entries(DIAGRAM_CODES).flatMap(([code, key]) => (viewer.diagrams[key] ? [code] : []));
  if (codes.join() !== "m,fb") into.set("dg", codes.join(","));
  if (viewer.rate !== 1) into.set("rate", String(viewer.rate));
  if (!viewer.loop) into.set("loop", "0");
  if (viewer.sheet === "all-views") into.set("xs", "all");
  if (viewer.fps !== null) into.set("xf", String(round(viewer.fps, 0)));
  if (viewer.durationS !== null) into.set("xd", String(round(viewer.durationS, 2)));
  const set = (name: string, value: string): void => into.set(name, value);
  if (viewer.preset !== null) into.set("env", viewer.preset);
  writeQuantities(ENVIRONMENT_QUANTITIES, viewer.environment, set);
  const { profile, ...settings } = viewer.flowSettings;
  if (profile === "logarithmic") into.set("wp", "log");
  writeQuantities(FLOW_QUANTITIES, settings, set);
  writeQuantities(MARK_QUANTITIES, viewer.flowDetail, set);
  return into;
}
//#endregion 🎛️Viewer

//#region ⚖️Compare
/** ⚖️ One pane of the comparison: an asset in a representation and a viewpoint, optionally at its own pose or clip. */
export type ComparePane = { readonly asset: Slug; readonly view: View; readonly viewpoint: PlanarViewpoint; readonly selection: Selection | null };

/** 🆚️ The comparison: two to four panes on one clock at one scale. */
export type CompareState = { readonly panes: readonly ComparePane[]; readonly scale: number; readonly timeS: number | null; readonly rate?: number; readonly loop?: boolean };

/** 📏️ The fewest and most panes. */
export const COMPARE_LIMITS = { min: 2, max: 4 } as const;

const selectionToken = (selection: Selection | null): string => (selection === null ? "" : selection.kind === "pose" ? `pose.${selection.pose}` : `clip.${selection.clip}`);
function selectionFrom(token: string | undefined): Selection | null {
  const [kind, slug] = (token ?? "").split(".");
  return isSlug(slug) ? (kind === "pose" ? { kind: "pose", pose: slug } : kind === "clip" ? { kind: "clip", clip: slug } : null) : null;
}

/** 🔍️ The comparison of an address query. */
export function compareFromParams(params: URLSearchParams): CompareState {
  const panes = params
    .getAll("p")
    .flatMap((token): ComparePane[] => {
      const [asset, view, selection, viewpoint] = token.split("~");
      return isSlug(asset) ? [{ asset, view: isView(view ?? null) ? (view as View) : DEFAULT_VIEWER.view, viewpoint: isViewpoint(viewpoint ?? null) ? (viewpoint as PlanarViewpoint) : PLANAR_DEFAULT, selection: selectionFrom(selection) }] : [];
    })
    .slice(0, COMPARE_LIMITS.max);
  const scale = number(params.get("scale"));
  const t = number(params.get("t"));
  const rate = number(params.get("rate"));
  return { panes, scale: scale !== null && scale >= 1 ? scale : DEFAULT_VIEWER.scale, timeS: t !== null && t >= 0 ? t : null, ...(rate !== null && [0.1, 0.25, 0.5, 2].includes(rate) ? { rate } : {}), ...(params.get("loop") === "0" ? { loop: false } : {}) };
}

/** 📝️ Writes a comparison into a query. */
export function compareToParams(compare: CompareState, into: URLSearchParams = new URLSearchParams()): URLSearchParams {
  for (const pane of compare.panes) into.append("p", [pane.asset, pane.view, selectionToken(pane.selection), pane.viewpoint === PLANAR_DEFAULT ? "" : pane.viewpoint].join("~").replace(/~+$/u, ""));
  if (compare.scale !== DEFAULT_VIEWER.scale) into.set("scale", String(round(compare.scale, 2)));
  if (compare.timeS !== null) into.set("t", String(round(compare.timeS, 3)));
  if (compare.rate !== undefined && compare.rate !== 1) into.set("rate", String(compare.rate));
  if (compare.loop === false) into.set("loop", "0");
  return into;
}
//#endregion ⚖️Compare

//#region 🧪️Playground
const isSectionViewpoint = (value: string | null): value is SectionViewpoint => (SECTION_VIEWPOINTS as readonly string[]).includes(value ?? "");
const flag = (value: string | undefined): boolean => value === undefined || value !== "n";

/** 🛝️ The playground of an address query, or null when the address carries none (the local one then stands). */
export function playgroundFromParams(params: URLSearchParams): Playground | null {
  const tokens = params.getAll("p");
  if (tokens.length === 0 && !["len", "dep", "tl", "cam", "scale", "view", "vp"].some((name) => params.has(name))) return null;
  const lengthM = clamp(number(params.get("len")) ?? DEFAULT_LENGTH_M, 4, 200);
  const depthM = clamp(number(params.get("dep")) ?? DEFAULT_DEPTH_M, 2, 200);
  const view = params.get("view");
  const viewpoint = params.get("vp");
  const camera = params.get("cam");
  const scale = number(params.get("scale"));
  const bg = params.get("bg");
  const placements = tokens.flatMap((token, index): Placement[] => {
    const [asset, x, pick, offset, mirrored, z, yaw, start, duration, speed, loop, hold] = token.split("~");
    return isSlug(asset)
      ? [{ id: `url-${index}`, asset, xM: clamp(number(x ?? null) ?? 0, 0, lengthM), zM: clamp(number(z ?? null) ?? 0, -depthM / 2, depthM / 2), yawDeg: clamp(number(yaw ?? null) ?? 0, -180, 180), pick: selectionFrom(pick), offsetS: number(offset ?? null) ?? 0, mirrored: mirrored === "m", ...clampTrack({ startS: number(start ?? null) ?? TRACK_DEFAULTS.startS, durationS: number(duration ?? null), speed: number(speed ?? null) ?? TRACK_DEFAULTS.speed, loop: flag(loop), holdS: number(hold ?? null) }) }]
      : [];
  });
  return { scale: scale !== null && scale >= 1 ? scale : 100, view: isView(view) ? view : "dark-line", viewpoint: isSectionViewpoint(viewpoint) ? viewpoint : "elevation", camera: isCameraPreset(camera) ? camera : DEFAULT_CAMERA, background: (BACKGROUNDS as readonly string[]).includes(bg ?? "") ? (bg as Background) : "paper", lengthM, depthM, timelineS: clampTimeline(number(params.get("tl"))), placements };
}

/** 🧩️ Writes a playground into a query. */
export function playgroundToParams(playground: Playground, into: URLSearchParams = new URLSearchParams()): URLSearchParams {
  if (playground.scale !== 100) into.set("scale", String(round(playground.scale, 2)));
  if (playground.view !== "dark-line") into.set("view", playground.view);
  if (playground.viewpoint !== "elevation") into.set("vp", playground.viewpoint);
  if (playground.camera !== DEFAULT_CAMERA) into.set("cam", playground.camera);
  if (playground.background !== "paper") into.set("bg", playground.background);
  if (playground.lengthM !== DEFAULT_LENGTH_M) into.set("len", String(round(playground.lengthM, 1)));
  if (playground.depthM !== DEFAULT_DEPTH_M) into.set("dep", String(round(playground.depthM, 1)));
  if (playground.timelineS !== null) into.set("tl", String(round(playground.timelineS, 2)));
  for (const placement of playground.placements)
    into.append("p", [placement.asset, round(placement.xM, 2), selectionToken(placement.pick), placement.offsetS === 0 ? "" : round(placement.offsetS, 2), placement.mirrored ? "m" : "", placement.zM === 0 ? "" : round(placement.zM, 2), placement.yawDeg === 0 ? "" : round(placement.yawDeg, 1), placement.startS === 0 ? "" : round(placement.startS, 2), placement.durationS === null ? "" : round(placement.durationS, 2), placement.speed === 1 ? "" : round(placement.speed, 3), placement.loop ? "" : "n", placement.holdS === null ? "" : round(placement.holdS, 3)].join("~").replace(/~+$/u, ""));
  return into;
}
//#endregion 🧪️Playground

//#region 🚨️Problems
const PROBLEM_WORDS = {
  planarOnly: (value: string): Text => ({ en: `The link names the viewpoint "${value}", which this page cannot show; the side elevation is shown.`, de: `Der Link nennt die Blickrichtung "${value}", die diese Seite nicht zeigen kann; die Seitenansicht wird gezeigt.` }),
  camera: (value: string): Text => ({ en: `The link names an unknown camera "${value}"; the perspective view is shown. Use ${CAMERA_PRESETS.join(", ")}.`, de: `Der Link nennt eine unbekannte Kamera "${value}"; die Perspektive wird gezeigt. Gültig sind ${CAMERA_PRESETS.join(", ")}.` }),
  viewpoint: (value: string): Text => ({ en: `The link names an unknown viewpoint "${value}"; the side elevation is shown. Use elevation, front or plan.`, de: `Der Link nennt eine unbekannte Blickrichtung "${value}"; die Seitenansicht wird gezeigt. Gültig sind elevation, front und plan.` }),
  sectionOnly: (value: string): Text => ({ en: `The link names the viewpoint "${value}", which the section playground cannot show; it shows the side elevation or the front elevation. The floor plan and the 3D model have playgrounds of their own.`, de: `Der Link nennt die Blickrichtung "${value}", die der Schnitt-Spielplatz nicht zeigen kann; er zeigt die Seitenansicht oder die Vorderansicht. Grundriss und 3D-Modell haben eigene Spielplätze.` }),
  mirror: { en: "The link mirrors placements, but mirroring applies to the side elevation only; they are drawn unmirrored.", de: "Der Link spiegelt Platzierungen, Spiegeln gilt aber nur in der Seitenansicht; sie werden ungespiegelt gezeichnet." } as Text,
  camera3d: (value: string): Text => ({ en: `The link names an unknown camera "${value}"; the perspective view is shown. Use ${CAMERA_PRESETS.join(", ")}.`, de: `Der Link nennt eine unbekannte Kamera "${value}"; die Perspektive wird gezeigt. Gültig sind ${CAMERA_PRESETS.join(", ")}.` }),
  layout: (value: string): Text => ({ en: `The link names an unknown layout "${value}"; one viewpoint is shown. Use all for three views.`, de: `Der Link nennt ein unbekanntes Layout "${value}"; eine Blickrichtung wird gezeigt. Gültig ist all für drei Ansichten.` }),
  view: (value: string): Text => ({ en: `The link names an unknown representation "${value}"; the dark line drawing is shown.`, de: `Der Link nennt eine unbekannte Darstellung "${value}"; die dunkle Strichzeichnung wird gezeigt.` }),
} as const;

/** 🚨️ What the address query of a page says that the archive cannot read, in words: an unknown viewpoint, layout or representation is reported, never silently replaced. */
export function addressProblems(params: URLSearchParams, page: "asset" | "compare" | "playground", tab: PlaygroundTab = "section"): readonly Text[] {
  const problems: Text[] = [];
  const viewpoint = params.get("vp");
  const layout = params.get("vl");
  const view = params.get("view");
  const camera = params.get("cam");
  const known = page === "asset" ? isViewpointKey(viewpoint) : page === "playground" ? isSectionViewpoint(viewpoint) : isViewpoint(viewpoint);
  if (viewpoint !== null && !known) problems.push(page === "playground" ? (isViewpointKey(viewpoint) ? PROBLEM_WORDS.sectionOnly(viewpoint) : PROBLEM_WORDS.viewpoint(viewpoint)) : isViewpointKey(viewpoint) ? PROBLEM_WORDS.planarOnly(viewpoint) : PROBLEM_WORDS.viewpoint(viewpoint));
  if (page === "asset" && camera !== null && !isCameraPreset(camera)) problems.push(PROBLEM_WORDS.camera(camera));
  if (page === "playground" && camera !== null && !isCameraPreset(camera)) problems.push(PROBLEM_WORDS.camera3d(camera));
  if (page === "asset" && layout !== null && layout !== "all") problems.push(PROBLEM_WORDS.layout(layout));
  if (view !== null && !isView(view)) problems.push(PROBLEM_WORDS.view(view));
  if (page === "playground" && (tab !== "section" || viewpoint === "front") && params.getAll("p").some((token) => token.split("~")[4] === "m")) problems.push(PROBLEM_WORDS.mirror);
  for (const token of page === "compare" ? params.getAll("p") : []) {
    const named = token.split("~")[3];
    if (named !== undefined && named !== "" && !isViewpoint(named)) problems.push(isViewpointKey(named) ? PROBLEM_WORDS.planarOnly(named) : PROBLEM_WORDS.viewpoint(named));
  }
  return problems;
}
//#endregion 🚨️Problems

//#region 🧭️Navigation
const remembered = new Map<string, string>();

/** 🧠️ Remembers the query a page was left with, so going back up to it restores its filters. */
export function rememberParams(route: Route, params: URLSearchParams): void {
  remembered.set(routePath(route), params.toString());
}

/** 🔙️ The query a page was last left with. */
export function recallParams(route: Route): URLSearchParams {
  return new URLSearchParams(remembered.get(routePath(route)) ?? "");
}

const CHANGE = "staffage:address";

function subscribe(listener: () => void): () => void {
  window.addEventListener("hashchange", listener);
  window.addEventListener(CHANGE, listener);
  return () => {
    window.removeEventListener("hashchange", listener);
    window.removeEventListener(CHANGE, listener);
  };
}

/** 🧭️ Moves to an address: a new history entry, or with `replace` the current one (for continuous state such as time or pan). */
export function navigate(route: Route, params: URLSearchParams = new URLSearchParams(), replace = false): void {
  const hash = formatAddress(route, params);
  if (hash === (window.location.hash || "#/")) return;
  if (replace) {
    window.history.replaceState(window.history.state, "", hash);
    window.dispatchEvent(new Event(CHANGE));
  } else window.location.hash = hash;
}

/** 🪝️ The current address and the way to change it. */
export function useAddress(): Address & { readonly navigate: typeof navigate; readonly hash: string } {
  const hash = useSyncExternalStore(subscribe, () => window.location.hash || "#/", () => "#/");
  const address = useMemo(() => parseAddress(hash), [hash]);
  const go = useCallback(navigate, []);
  return { ...address, hash, navigate: go };
}
//#endregion 🧭️Navigation
