/** 🧮️ The one evaluator every web surface shares: figure, custom element and downloads memoise `evaluate` → `solveThermal` / `solveStructure` → `depict` through it.
 *
 * Results are keyed by asset identity, query and environment, so a figure that re-renders, a second figure on the same instant and an export of that instant solve once.
 * Analyses are solved lazily: only a view that reads them, or a consumer that touches `thermal` / `structure`, pays for them.
 *
 * @see ../🧍️figure/🟦️.tsx — the React consumer
 * @see ../📤️downloads/🟦️.ts — the export consumer
 * @see ./🔤️text/🟦️.ts — the room the texts of a drawing take on the page
 */
import { VIEWS } from "../../../../🔨️modules/🔗️contracts/🟦️.ts";
import { defaultDepictionOptions, depict, resolveDepiction } from "../../../../🔨️modules/🎨️depiction/🟦️.ts";
import { evaluateDrawing } from "../../../../🔨️modules/🧮️evaluation/🟦️.ts";
import { solveStructure } from "../../../../🔨️modules/🏗️structure/🟦️.ts";
import { solveThermal } from "../../../../🔨️modules/🌡️thermal/🟦️.ts";
import { writeSvg } from "../../../../🔨️modules/📤️export/🖼️svg/🟦️.ts";
import { textOverhang } from "./🔤️text/🟦️.ts";
import type { Asset, DepictionOptions, DisplayRange, DrawingScene, DrawingScale, Environment, EvaluatedState, EvaluationQuery, FlowConditions, FlowLayer, FlowMarkDetail, ForceQuantity, Language, PrintProfile, RepresentationSelector, StructuralResult, ThermalField, ThermalQuantity, TransportField, View, Viewpoint } from "@teaching/staffage";

/** 🖥️ The profile of an on-screen figure: the archive is read on a display, so pen and gap floors of a printer do not hide the drawing. A file exported from the site names this profile and does not claim a printer check. */
const SCREEN_PROFILE: PrintProfile = {
  id: "screen",
  title: { en: "Screen", de: "Bildschirm" },
  description: { en: "On-screen reading of the archive. It does not certify that the drawing meets a print profile.", de: "Bildschirmlesung des Archivs. Sie bescheinigt nicht, dass die Zeichnung ein Druckprofil erfüllt." },
  minStrokeMm: 0,
  minGapMm: 0,
  minFeatureMm: 0,
  minHatchSpacingMm: 0,
  source: "authored-estimate",
};

/** 🧾️ One evaluated instant with lazily solved analyses. */
export type StaffageFrame = { readonly state: EvaluatedState; readonly thermal: ThermalField; readonly structure: StructuralResult; readonly scene: DrawingScene };

/** 🎛️ The viewer-level options of a depiction, before they merge into {@link DepictionOptions}. */
export type DepictionRequest = {
  readonly view: View;
  readonly scale: DrawingScale;
  readonly language: Language;
  readonly viewpoint: Viewpoint;
  readonly representation?: RepresentationSelector;
  readonly theme?: "light" | "dark";
  readonly transparent?: boolean;
  readonly annotations?: Partial<DepictionOptions["annotations"]>;
  readonly thermalQuantity?: ThermalQuantity;
  readonly forceQuantity?: ForceQuantity;
  readonly range?: DisplayRange;
  readonly patterns?: boolean;
  readonly diagrams?: Partial<DepictionOptions["diagrams"]>;
  readonly environment?: Environment;
  /** 🌊️ The air flow layers a drawing of the thermal view carries (files and prints; the on-screen figure animates them in its own overlay instead). */
  readonly flow?: readonly FlowLayer[];
  /** 🌪️ The conditions of the air that flow is solved in (turbulence, wind profile, parcel life, seed), the same the screen solves with; the life of a parcel is also how long the drawn flow has developed. */
  readonly flowConditions?: FlowConditions;
  /** 🔎️ The generalisation and timing of the drawn flow marks, the same the screen draws with. */
  readonly flowDetail?: FlowMarkDetail;
};

/** ⏳️ Seconds of history the air flow of a drawn file has: the same as the flow on screen. */
export const DRAWN_FLOW_DEVELOP_S = 14;

const CAPACITY = 512;

/** ⚖️ The evaluated side-elevation points all kept instants of all assets may hold together, least recently asked first out; the newest instant is always kept. Measured on the 375-asset catalog: a kept instant holds its shapes, scenes and markup at about 370 B per point (walking commuter 875 points and 330 kB, sunflower 2069 points and 600 kB), so this budget is about 150 MB in the page and as much in the drawing worker. */
export const STATE_BUDGET_POINTS = 400_000;

type KeptState = { readonly cache: Map<string, EvaluatedState>; readonly key: string; readonly points: number };
const states = new WeakMap<Asset, Map<string, EvaluatedState>>();
const kept = new Map<EvaluatedState, KeptState>();
let keptPoints = 0;
const thermals = new WeakMap<EvaluatedState, Map<string, ThermalField>>();
const structures = new WeakMap<EvaluatedState, StructuralResult>();
const scenes = new WeakMap<EvaluatedState, Map<string, DrawingScene>>();

const optionKeys = new WeakMap<object, string>();
const keyOf = (options: DepictionOptions): string => {
  let key = optionKeys.get(options);
  if (key === undefined) optionKeys.set(options, (key = JSON.stringify(options)));
  return key;
};

function remember<V>(cache: Map<string, V>, key: string, make: () => V): V {
  const hit = cache.get(key);
  if (hit !== undefined) return hit;
  const value = make();
  cache.set(key, value);
  if (cache.size > CAPACITY) cache.delete(cache.keys().next().value as string);
  return value;
}

/** 🎯️ The query to show when none is given: the asset's default clip at `timeS` when a clock drives it, else its default pose. */
export function defaultQuery(asset: Asset, timeS: number | null): EvaluationQuery {
  return timeS === null ? { kind: "pose", pose: asset.presentation.defaultPose } : { kind: "clip", clip: asset.presentation.defaultClip, timeS };
}

/** ⏱️ The query at an instant: clips and transitions follow the time, poses do not. */
export function queryAt(query: EvaluationQuery, timeS: number | null): EvaluationQuery {
  return timeS === null || query.kind === "pose" ? query : { ...query, timeS };
}

/** 📊️ How many evaluated instants are kept and how many side-elevation points they hold. */
export const evaluatedStateLoad = (): { readonly states: number; readonly points: number } => ({ states: kept.size, points: keptPoints });

/** ⚖️ What an evaluated instant weighs: its side-elevation points, and one for the instant itself. The unit of {@link STATE_BUDGET_POINTS} and of the live budget of the previews.
 * @see ../🖼️previews/🚦️live/🟦️.ts — `PREVIEW_LIVE_BUDGET_POINTS` */
export const statePoints = (state: EvaluatedState): number => state.shapes.elevation.reduce((sum, shape) => sum + shape.rings.reduce((ring, at) => ring + at.length, 0), 1);

/** 🧮️ Evaluates (memoised within {@link STATE_BUDGET_POINTS}) an asset at a query, projecting front and plan views on demand. A selector picks the authored drawing; without one the state carries rig and physics only. */
export function evaluateState(asset: Asset, query: EvaluationQuery, selector?: RepresentationSelector): EvaluatedState {
  let cache = states.get(asset);
  if (cache === undefined) states.set(asset, (cache = new Map()));
  const key = JSON.stringify([query, selector ?? null]);
  const hit = cache.get(key);
  if (hit !== undefined) {
    const entry = kept.get(hit)!;
    kept.delete(hit);
    kept.set(hit, entry);
    return hit;
  }
  const state = evaluateDrawing(asset, query, selector);
  const points = statePoints(state);
  cache.set(key, state);
  kept.set(state, { cache, key, points });
  keptPoints += points;
  for (const [oldest, entry] of kept) {
    if (keptPoints <= STATE_BUDGET_POINTS || oldest === state) break;
    kept.delete(oldest);
    entry.cache.delete(entry.key);
    keptPoints -= entry.points;
  }
  return state;
}

/** 🖨️ The same instant drawn with the representation the depiction asks for. */
function represented(state: EvaluatedState, options: DepictionOptions): EvaluatedState {
  const resolution = resolveDepiction(state.asset, options);
  const mine = state.representation;
  const aligned = resolution.kind === "symbol" ? mine.kind === "symbol" && mine.symbol === resolution.symbol.id : mine.kind === "scale" && mine.denominator === resolution.denominator && mine.requested === resolution.requested;
  return aligned ? state : evaluateState(state.asset, state.query, options.representation);
}

/** 🌡️ Solves (memoised) the thermal field of a state under an environment. */
export function thermalOf(state: EvaluatedState, environment?: Environment): ThermalField {
  let cache = thermals.get(state);
  if (cache === undefined) thermals.set(state, (cache = new Map()));
  return remember(cache, JSON.stringify(environment ?? null), () => solveThermal(state, environment));
}

/** 🏗️ Solves (memoised) the structural result of a state. */
export function structureOf(state: EvaluatedState): StructuralResult {
  let result = structures.get(state);
  if (result === undefined) structures.set(state, (result = solveStructure(state)));
  return result;
}

/** 🎛️ Merges viewer options over {@link defaultDepictionOptions}. */
export function depictionOptions(request: DepictionRequest): DepictionOptions {
  const base = defaultDepictionOptions(request.view, request.language, request.scale.denominator, request.viewpoint, request.theme);
  return {
    ...base,
    profile: SCREEN_PROFILE,
    scale: request.scale,
    ...(request.representation === undefined ? {} : { representation: request.representation }),
    transparent: request.transparent ?? base.transparent,
    annotations: { ...base.annotations, ...request.annotations },
    diagrams: { ...base.diagrams, ...request.diagrams },
    thermalQuantity: request.thermalQuantity ?? base.thermalQuantity,
    forceQuantity: request.forceQuantity ?? base.forceQuantity,
    range: request.range ?? base.range,
    patterns: request.patterns ?? base.patterns,
    ...(request.environment === undefined ? {} : { environment: request.environment }),
    ...(request.flow === undefined || request.flow.length === 0 || request.view !== "thermal" ? {} : { flow: request.flow, flowDevelopS: request.flowConditions?.maxAgeS ?? DRAWN_FLOW_DEVELOP_S, ...(request.flowConditions === undefined ? {} : { flowConditions: request.flowConditions }), ...(request.flowDetail === undefined ? {} : { flowDetail: request.flowDetail }) }),
  };
}

/** 🖨️ Depicts a state; only the analyses the view reads are solved. The state is re-evaluated when it was solved without the depiction's representation. */
export function sceneOf(state: EvaluatedState, options: DepictionOptions): DrawingScene {
  let cache = scenes.get(state);
  if (cache === undefined) scenes.set(state, (cache = new Map()));
  return remember(cache, keyOf(options), () => {
    const drawn = represented(state, options);
    const thermal = options.view === "thermal" ? thermalOf(drawn, options.environment) : undefined;
    const structure = options.view === "forces" || options.view === "moments" ? structureOf(drawn) : undefined;
    return depict(drawn, options, { ...(thermal === undefined ? {} : { thermal }), ...(structure === undefined ? {} : { structure }) });
  });
}

/** 🌬️ Depicts a state with the air of another instant: the figure and its thermal field stay those of `state`, the flow layers are drawn from `transport`. This is how a still pose plays its air flow frame by frame; nothing is memoised, every frame is drawn once. */
export function sceneWithAir(state: EvaluatedState, options: DepictionOptions, transport: TransportField): DrawingScene {
  const drawn = represented(state, options);
  return depict(drawn, options, { ...(options.view === "thermal" ? { thermal: thermalOf(drawn, options.environment) } : {}), transport });
}

/** 🧾️ One instant as a frame: scene now, analyses on first read. */
export function figureFrame(asset: Asset, query: EvaluationQuery, request: DepictionRequest): StaffageFrame {
  const options = depictionOptions(request);
  const state = evaluateState(asset, query, options.representation);
  const scene = sceneOf(state, options);
  return {
    state,
    scene,
    get thermal() {
      return thermalOf(state, options.environment);
    },
    get structure() {
      return structureOf(state);
    },
  };
}

/** 🖼️ Depicts one state in all seven views, solving each analysis once. */
export function allViews(state: EvaluatedState, request: Omit<DepictionRequest, "view">): DrawingScene[] {
  return VIEWS.map((view) => sceneOf(state, depictionOptions({ ...request, view })));
}

const inlined = new WeakMap<DrawingScene, string>();
const ROOT_TAG = /<svg\b[^>]*>/u;
const OWNED_ATTRIBUTES = /\s(?:width|height|viewBox|preserveAspectRatio|style|role|aria-[\w-]+|focusable)="[^"]*"/gu;

/** 🪟️ A paper-space window in millimetres relative to a scene's `metadata.origin` (x right, y down). */
export type SceneFrame = { readonly minX: number; readonly minY: number; readonly maxX: number; readonly maxY: number };

/** 🪟️ The extent of one scene relative to its origin: its paper, widened by what its texts take past their reserved room in the faces of the page ({@link textOverhang}). */
export function sceneExtent(scene: DrawingScene): SceneFrame {
  const { left, right } = textOverhang(scene);
  return { minX: -scene.metadata.origin.xMm - left, minY: -scene.metadata.origin.yMm, maxX: scene.widthMm - scene.metadata.origin.xMm + right, maxY: scene.heightMm - scene.metadata.origin.yMm };
}

/** 🪟️ The window a scene shown alone needs instead of its own paper: none while every text fits its room. */
export function sceneWindow(scene: DrawingScene): SceneFrame | undefined {
  const { left, right } = textOverhang(scene);
  return left === 0 && right === 0 ? undefined : sceneExtent(scene);
}

/** 🪟️ The union frame around the shared origin of several scenes, so figures of different views keep the model fixed when they share it. */
export function frameOf(scenes: readonly DrawingScene[]): SceneFrame {
  const extents = scenes.map(sceneExtent);
  return { minX: Math.min(...extents.map((e) => e.minX)), minY: Math.min(...extents.map((e) => e.minY)), maxX: Math.max(...extents.map((e) => e.maxX)), maxY: Math.max(...extents.map((e) => e.maxY)) };
}

/** 🖼️ The scene as inline-embeddable SVG: the root sizes to its box and keeps the paper aspect, the figure wrapper owns role and label; a `frame` pads the viewBox around the origin. */
export function inlineSvg(scene: DrawingScene, frame?: SceneFrame): string {
  const cached = frame === undefined ? inlined.get(scene) : undefined;
  if (cached !== undefined) return cached;
  const markup = writeSvg(scene);
  const root = (ROOT_TAG.exec(markup)?.[0] ?? "<svg>").replace(OWNED_ATTRIBUTES, "").replace(/\/?>$/u, "");
  const view = frame === undefined ? { x: 0, y: 0, w: scene.widthMm, h: scene.heightMm } : { x: scene.metadata.origin.xMm + frame.minX, y: scene.metadata.origin.yMm + frame.minY, w: frame.maxX - frame.minX, h: frame.maxY - frame.minY };
  const own = ` viewBox="${view.x} ${view.y} ${view.w} ${view.h}" preserveAspectRatio="xMidYMid meet" style="display:block;width:100%;height:100%" aria-hidden="true" focusable="false">`;
  const result = markup.replace(ROOT_TAG, root + own);
  if (frame === undefined) inlined.set(scene, result);
  return result;
}
