/** 🗺️ The address of the archive: which page is open and every meaningful state of it, as a shareable hash such as `#/humans/walking-commuter?view=thermal&scale=50&clip=walk-cycle&t=1.2`.
 *
 * Route: `#/` overview · `#/all` · `#/<category>` · `#/<category>/<asset>` · `#/collections[/<id>]` · `#/compare` · `#/playground`.
 * Parameters carry only what differs from the default, in a stable order, so `format(parse(x))` is canonical and a link reproduces the state: filters (`q cat an h root done sort dir`),
 * the viewer (`view scale pose|clip t zoom pan bg th tq fq range pat dg air mrt rh wind sun`), the comparison (`p` per pane) and the playground (`p` per placement, `len`).
 * The reader's own preferences — language, theme — are never part of a link.
 *
 * @see ../🔎️search/🟦️.ts — filter parameters
 * @see ../💾️persistence/🟦️.ts — the local state that is not shared
 */
import { useCallback, useMemo, useSyncExternalStore } from "react";
import { CATEGORIES, FORCE_QUANTITIES, THERMAL_QUANTITIES, VIEWS } from "@teaching/staffage";
import type { CategoryId, DisplayRange, ForceQuantity, Slug, ThermalQuantity, View } from "@teaching/staffage";
import { BACKGROUNDS, DEFAULT_LENGTH_M, type Background, type Placement, type Playground, type Selection } from "../💾️persistence/🟦️.ts";

/** 📍️ A page of the archive. */
export type Route =
  | { readonly page: "overview" }
  | { readonly page: "all" }
  | { readonly page: "category"; readonly category: CategoryId }
  | { readonly page: "asset"; readonly category: CategoryId; readonly asset: Slug }
  | { readonly page: "collections"; readonly collection: Slug | null }
  | { readonly page: "compare" }
  | { readonly page: "playground" };

/** 🏠️ The overview. */
export const OVERVIEW: Route = { page: "overview" };

/** 🧭️ A route and its query. */
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
  else if (first === "playground") route = { page: "playground" };
  else if (first === "collections") route = { page: "collections", collection: isSlug(second) ? second : null };
  else if (isCategory(first)) route = isSlug(second) ? { page: "asset", category: first, asset: second } : { page: "category", category: first };
  return { route, params };
}

/** 🔗️ The path of a route. */
export function routePath(route: Route): string {
  switch (route.page) {
    case "overview":
      return "/";
    case "all":
    case "compare":
    case "playground":
      return `/${route.page}`;
    case "category":
      return `/${route.category}`;
    case "asset":
      return `/${route.category}/${route.asset}`;
    case "collections":
      return route.collection === null ? "/collections" : `/collections/${route.collection}`;
  }
}

/** 🔗️ The hash of a route and its query. */
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

/** 🧭️ Whether two routes are the same page. */
export const sameRoute = (a: Route, b: Route): boolean => routePath(a) === routePath(b);

//#region 🎛️Viewer
/** 🖼️ The ground of a viewer: by view, or the reader's choice. */
export type ViewerBackground = "auto" | Background;

/** 🌤️ The environment values the reader moved off the asset's reference case (relative humidity as a fraction). */
export type EnvironmentOverride = { readonly airTempC?: number; readonly meanRadiantTempC?: number; readonly relativeHumidity?: number; readonly windMS?: number; readonly solarWM2?: number };

/** 📈️ Which member diagrams the force and moment views draw. */
export type DiagramToggles = { readonly axial: boolean; readonly shear: boolean; readonly moment: boolean; readonly freeBody: boolean };

/** 🎛️ Everything the viewer shows besides the asset: view, scale, what is posed or played, zoom and analytic options. */
export type ViewerState = {
  readonly view: View;
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
  readonly environment: EnvironmentOverride;
  readonly rate: number;
  readonly loop: boolean;
  readonly sheet: "single" | "all-views";
  readonly fps: number | null;
  readonly durationS: number | null;
};

/** 🌱️ The viewer a reader first sees: dark line drawing at 1:100, the default clip playing, fitted. */
export const DEFAULT_VIEWER: ViewerState = {
  view: "dark-line",
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
  patterns: false,
  diagrams: { axial: false, shear: false, moment: true, freeBody: true },
  environment: {},
  rate: 1,
  loop: true,
  sheet: "single",
  fps: null,
  durationS: null,
};

const DIAGRAM_CODES = { n: "axial", v: "shear", m: "moment", fb: "freeBody" } as const;
const isView = (value: string | null): value is View => (VIEWS as readonly string[]).includes(value ?? "");
const number = (value: string | null): number | null => {
  const parsed = value === null || value.trim() === "" ? Number.NaN : Number(value);
  return Number.isFinite(parsed) ? parsed : null;
};
const clamp = (value: number, low: number, high: number): number => Math.min(Math.max(value, low), high);
const round = (value: number, places: number): number => Number(value.toFixed(places));

/** 🔗️ The viewer of an address query; unknown or out-of-range values fall back to the default. */
export function viewerFromParams(params: URLSearchParams): ViewerState {
  const base = DEFAULT_VIEWER;
  const view = params.get("view");
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
  const diagrams = params.get("dg");
  const t = number(params.get("t"));
  const env = { airTempC: number(params.get("air")), meanRadiantTempC: number(params.get("mrt")), relativeHumidity: number(params.get("rh")), windMS: number(params.get("wind")), solarWM2: number(params.get("sun")) };
  return {
    view: isView(view) ? view : base.view,
    scale: scale !== null && scale >= 1 && scale <= 100000 ? scale : base.scale,
    selection: isSlug(pose ?? undefined) ? { kind: "pose", pose: pose as Slug } : isSlug(clip ?? undefined) ? { kind: "clip", clip: clip as Slug } : null,
    timeS: t !== null && t >= 0 ? t : null,
    zoom: zoomValue !== null && zoomValue > 0 ? clamp(zoomValue, 0.05, 64) : "fit",
    pan: panX !== undefined && panX !== null && panY !== undefined && panY !== null ? { xMm: panX, yMm: panY } : base.pan,
    background: bg === "dark" || bg === "light" || bg === "paper" || bg === "transparent" ? bg : "auto",
    theme: params.get("th") === "light" || params.get("th") === "dark" ? (params.get("th") as "light" | "dark") : "auto",
    thermalQuantity: (THERMAL_QUANTITIES as readonly string[]).includes(tq ?? "") ? (tq as ThermalQuantity) : base.thermalQuantity,
    forceQuantity: (FORCE_QUANTITIES as readonly string[]).includes(fq ?? "") ? (fq as ForceQuantity) : base.forceQuantity,
    range: low !== undefined && low !== null && high !== undefined && high !== null && high > low ? { mode: "fixed", min: low, max: high } : base.range,
    patterns: params.get("pat") === "1",
    diagrams: diagrams === null ? base.diagrams : { axial: false, shear: false, moment: false, freeBody: false, ...Object.fromEntries(diagrams.split(",").flatMap((code) => (code in DIAGRAM_CODES ? [[DIAGRAM_CODES[code as keyof typeof DIAGRAM_CODES], true]] : []))) },
    rate: number(params.get("rate")) !== null && [0.1, 0.25, 0.5, 1, 2].includes(number(params.get("rate"))!) ? number(params.get("rate"))! : 1,
    loop: params.get("loop") !== "0",
    sheet: params.get("xs") === "all" ? "all-views" : "single",
    fps: number(params.get("xf")) !== null ? clamp(number(params.get("xf"))!, 12, 60) : null,
    durationS: number(params.get("xd")) !== null && number(params.get("xd"))! > 0 ? clamp(number(params.get("xd"))!, 0.1, 600) : null,
    environment: {
      ...(env.airTempC === null ? {} : { airTempC: clamp(env.airTempC, -60, 70) }),
      ...(env.meanRadiantTempC === null ? {} : { meanRadiantTempC: clamp(env.meanRadiantTempC, -60, 90) }),
      ...(env.relativeHumidity === null ? {} : { relativeHumidity: clamp(env.relativeHumidity / 100, 0, 1) }),
      ...(env.windMS === null ? {} : { windMS: clamp(env.windMS, 0, 40) }),
      ...(env.solarWM2 === null ? {} : { solarWM2: clamp(env.solarWM2, 0, 1400) }),
    },
  };
}

/** 🔗️ Writes the non-default viewer state into a query, in a stable order. */
export function viewerToParams(viewer: ViewerState, into: URLSearchParams = new URLSearchParams()): URLSearchParams {
  const base = DEFAULT_VIEWER;
  if (viewer.view !== base.view) into.set("view", viewer.view);
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
  if (viewer.patterns) into.set("pat", "1");
  const codes = Object.entries(DIAGRAM_CODES).flatMap(([code, key]) => (viewer.diagrams[key] ? [code] : []));
  if (codes.join() !== "m,fb") into.set("dg", codes.join(","));
  if (viewer.rate !== 1) into.set("rate", String(viewer.rate));
  if (!viewer.loop) into.set("loop", "0");
  if (viewer.sheet === "all-views") into.set("xs", "all");
  if (viewer.fps !== null) into.set("xf", String(round(viewer.fps, 0)));
  if (viewer.durationS !== null) into.set("xd", String(round(viewer.durationS, 2)));
  const { environment } = viewer;
  if (environment.airTempC !== undefined) into.set("air", String(round(environment.airTempC, 1)));
  if (environment.meanRadiantTempC !== undefined) into.set("mrt", String(round(environment.meanRadiantTempC, 1)));
  if (environment.relativeHumidity !== undefined) into.set("rh", String(round(environment.relativeHumidity * 100, 0)));
  if (environment.windMS !== undefined) into.set("wind", String(round(environment.windMS, 1)));
  if (environment.solarWM2 !== undefined) into.set("sun", String(round(environment.solarWM2, 0)));
  return into;
}
//#endregion 🎛️Viewer

//#region ⚖️Compare
/** ⚖️ One pane of the comparison: an asset in a view, optionally at its own pose or clip. */
export type ComparePane = { readonly asset: Slug; readonly view: View; readonly selection: Selection | null };

/** ⚖️ The comparison: two to four panes on one clock at one scale. */
export type CompareState = { readonly panes: readonly ComparePane[]; readonly scale: number; readonly timeS: number | null; readonly rate?: number; readonly loop?: boolean };

/** 📏️ The fewest and most panes. */
export const COMPARE_LIMITS = { min: 2, max: 4 } as const;

const selectionToken = (selection: Selection | null): string => (selection === null ? "" : selection.kind === "pose" ? `pose.${selection.pose}` : `clip.${selection.clip}`);
function selectionFrom(token: string | undefined): Selection | null {
  const [kind, slug] = (token ?? "").split(".");
  return isSlug(slug) ? (kind === "pose" ? { kind: "pose", pose: slug } : kind === "clip" ? { kind: "clip", clip: slug } : null) : null;
}

/** 🔗️ The comparison of an address query. */
export function compareFromParams(params: URLSearchParams): CompareState {
  const panes = params
    .getAll("p")
    .flatMap((token): ComparePane[] => {
      const [asset, view, selection] = token.split("~");
      return isSlug(asset) ? [{ asset, view: isView(view ?? null) ? (view as View) : DEFAULT_VIEWER.view, selection: selectionFrom(selection) }] : [];
    })
    .slice(0, COMPARE_LIMITS.max);
  const scale = number(params.get("scale"));
  const t = number(params.get("t"));
  const rate = number(params.get("rate"));
  return { panes, scale: scale !== null && scale >= 1 ? scale : DEFAULT_VIEWER.scale, timeS: t !== null && t >= 0 ? t : null, ...(rate !== null && [0.1, 0.25, 0.5, 2].includes(rate) ? { rate } : {}), ...(params.get("loop") === "0" ? { loop: false } : {}) };
}

/** 🔗️ Writes a comparison into a query. */
export function compareToParams(compare: CompareState, into: URLSearchParams = new URLSearchParams()): URLSearchParams {
  for (const pane of compare.panes) into.append("p", [pane.asset, pane.view, selectionToken(pane.selection)].join("~").replace(/~+$/u, ""));
  if (compare.scale !== DEFAULT_VIEWER.scale) into.set("scale", String(round(compare.scale, 2)));
  if (compare.timeS !== null) into.set("t", String(round(compare.timeS, 3)));
  if (compare.rate !== undefined && compare.rate !== 1) into.set("rate", String(compare.rate));
  if (compare.loop === false) into.set("loop", "0");
  return into;
}
//#endregion ⚖️Compare

//#region 🧪️Playground
/** 🔗️ The playground of an address query, or null when the address carries none (the local one then stands). */
export function playgroundFromParams(params: URLSearchParams): Playground | null {
  const tokens = params.getAll("p");
  if (tokens.length === 0 && !params.has("len") && !params.has("scale") && !params.has("view")) return null;
  const lengthM = clamp(number(params.get("len")) ?? DEFAULT_LENGTH_M, 4, 200);
  const view = params.get("view");
  const scale = number(params.get("scale"));
  const bg = params.get("bg");
  const placements = tokens.flatMap((token, index): Placement[] => {
    const [asset, x, pick, offset, mirrored] = token.split("~");
    return isSlug(asset) ? [{ id: `url-${index}`, asset, xM: clamp(number(x ?? null) ?? 0, 0, lengthM), pick: selectionFrom(pick), offsetS: number(offset ?? null) ?? 0, mirrored: mirrored === "m" }] : [];
  });
  return { scale: scale !== null && scale >= 1 ? scale : 100, view: isView(view) ? view : "dark-line", background: (BACKGROUNDS as readonly string[]).includes(bg ?? "") ? (bg as Background) : "paper", lengthM, placements };
}

/** 🔗️ Writes a playground into a query. */
export function playgroundToParams(playground: Playground, into: URLSearchParams = new URLSearchParams()): URLSearchParams {
  if (playground.scale !== 100) into.set("scale", String(round(playground.scale, 2)));
  if (playground.view !== "dark-line") into.set("view", playground.view);
  if (playground.background !== "paper") into.set("bg", playground.background);
  if (playground.lengthM !== DEFAULT_LENGTH_M) into.set("len", String(round(playground.lengthM, 1)));
  for (const placement of playground.placements) into.append("p", [placement.asset, round(placement.xM, 2), selectionToken(placement.pick), placement.offsetS === 0 ? "" : round(placement.offsetS, 2), placement.mirrored ? "m" : ""].join("~").replace(/~+$/u, ""));
  return into;
}
//#endregion 🧪️Playground

//#region 🧭️Navigation
const remembered = new Map<string, string>();

/** 🧠️ Remembers the query a page was left with, so going back up to it restores its filters. */
export function rememberParams(route: Route, params: URLSearchParams): void {
  remembered.set(routePath(route), params.toString());
}

/** 🧠️ The query a page was last left with. */
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
