/** 🧮️ The one evaluator every web surface shares: figure, custom element and downloads memoise `evaluate` → `solveThermal` / `solveStructure` → `depict` through it.
 *
 * Results are keyed by asset identity, query and environment, so a figure that re-renders, a second figure on the same instant and an export of that instant solve once.
 * Analyses are solved lazily: only a view that reads them, or a consumer that touches `thermal` / `structure`, pays for them.
 *
 * @see ../🧍️figure/🟦️.tsx — the React consumer
 * @see ../📤️downloads/🟦️.ts — the export consumer
 */
import { VIEWS, defaultDepictionOptions, depict, evaluate, resolveDepiction, solveStructure, solveThermal, writeSvg } from "@teaching/staffage";
import type { Asset, DepictionOptions, DisplayRange, DrawingScene, DrawingScale, Environment, EvaluatedState, EvaluationQuery, ForceQuantity, Language, PrintProfile, RepresentationSelector, StructuralResult, ThermalField, ThermalQuantity, View, Viewpoint } from "@teaching/staffage";

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
  readonly viewpoint?: Viewpoint;
  readonly theme?: "light" | "dark";
  readonly transparent?: boolean;
  readonly annotations?: Partial<DepictionOptions["annotations"]>;
  readonly thermalQuantity?: ThermalQuantity;
  readonly forceQuantity?: ForceQuantity;
  readonly range?: DisplayRange;
  readonly patterns?: boolean;
  readonly diagrams?: Partial<DepictionOptions["diagrams"]>;
  readonly environment?: Environment;
};

const CAPACITY = 512;
const states = new WeakMap<Asset, Map<string, EvaluatedState>>();
const thermals = new WeakMap<EvaluatedState, Map<string, ThermalField>>();
const structures = new WeakMap<EvaluatedState, StructuralResult>();
const scenes = new WeakMap<EvaluatedState, Map<string, DrawingScene>>();

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

/** 🧮️ Evaluates (memoised) an asset at a query. A selector picks the authored drawing; without one the state carries rig and physics only. */
export function evaluateState(asset: Asset, query: EvaluationQuery, selector?: RepresentationSelector): EvaluatedState {
  let cache = states.get(asset);
  if (cache === undefined) states.set(asset, (cache = new Map()));
  return remember(cache, JSON.stringify([query, selector ?? null]), () => evaluate(asset, query, selector));
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
  const base = defaultDepictionOptions(request.view, request.language, request.scale.denominator, request.viewpoint ?? "elevation", request.theme);
  return {
    ...base,
    profile: SCREEN_PROFILE,
    scale: request.scale,
    transparent: request.transparent ?? base.transparent,
    annotations: { ...base.annotations, ...request.annotations },
    diagrams: { ...base.diagrams, ...request.diagrams },
    thermalQuantity: request.thermalQuantity ?? base.thermalQuantity,
    forceQuantity: request.forceQuantity ?? base.forceQuantity,
    range: request.range ?? base.range,
    patterns: request.patterns ?? base.patterns,
    ...(request.environment === undefined ? {} : { environment: request.environment }),
  };
}

/** 🖨️ Depicts a state; only the analyses the view reads are solved. The state is re-evaluated when it was solved without the depiction's representation. */
export function sceneOf(state: EvaluatedState, options: DepictionOptions): DrawingScene {
  let cache = scenes.get(state);
  if (cache === undefined) scenes.set(state, (cache = new Map()));
  return remember(cache, JSON.stringify(options), () => {
    const drawn = represented(state, options);
    const thermal = options.view === "thermal" ? thermalOf(drawn, options.environment) : undefined;
    const structure = options.view === "forces" || options.view === "moments" ? structureOf(drawn) : undefined;
    return depict(drawn, options, { ...(thermal === undefined ? {} : { thermal }), ...(structure === undefined ? {} : { structure }) });
  });
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

/** 🪟️ The extent of one scene relative to its origin. */
export const sceneExtent = (scene: DrawingScene): SceneFrame => ({ minX: -scene.metadata.origin.xMm, minY: -scene.metadata.origin.yMm, maxX: scene.widthMm - scene.metadata.origin.xMm, maxY: scene.heightMm - scene.metadata.origin.yMm });

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
