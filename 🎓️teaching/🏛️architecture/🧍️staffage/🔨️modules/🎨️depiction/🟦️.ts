/** 🎨️ Depiction: one {@link EvaluatedState} becomes any of seven synchronized paper-space views ({@link depict}) or a CAD model-space scene ({@link modelScene}).
 *
 * Paper space is millimetres, y down. The scene size is the drawn extent (ground line, plumes, arrows, diagrams, annotations when shown) × `mmPerMetre`
 * plus margins; `metadata.origin` is where the model origin landed, so a model point `(x, y)` sits at paper `(origin.xMm + x·k, origin.yMm − y·k)` with
 * `k = mmPerMetre`. Every view paints the same prepared rings (identical `fill` item commands per shape id, relative to the origin); line views add
 * role-weighted outlines with hidden lines removed, solid views add tones, analytic views add legends with unit, sign convention and interpretation.
 * Analytic views use `options.theme` (default light).
 *
 * The drawing is the individually authored representation the state was evaluated with: {@link depict} resolves it under `options.profile` and throws a
 * `RepresentationError` carrying the resolution when the requested scale is unavailable or unsuitable for the profile; it never draws an empty or substituted scene.
 * Structural views (forces, moments) draw on bones and are independent of the representation.
 *
 * @see ./🧩️shapes/🟦️.ts — pens, hatches, z order, hidden lines
 * @see ../📤️export/🖼️svg/🟦️.ts — SVG serialisation
 * @see ../🔗️contracts/🟦️.ts — {@link DepictionOptions}, {@link DrawingScene}, {@link ModelScene}
 */
import type { Language, PlanarViewpoint, Text, Viewpoint } from "../../🧬️schema/🟦️.ts";
import { VIEWS, type AxonometricKind, type AxonometricSpec, type DepictionCommon, type DepictionOptions, type TextlessDepiction, type TextualDepiction, type DrawingScene, type EvaluatedState, type JobControl, type Legend, type ModelLayer, type ModelScene, type Point, type RepresentationResolution, type StructuralResult, type ThermalField, type TransportField, type View } from "../🔗️contracts/🟦️.ts";
import { LABEL_TEXT_MM, mmPerMetre } from "../📐️units/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, requireDrawable, resolveRepresentation, resolveSymbol } from "../🖨️representation/🟦️.ts";
import { solveStructure } from "../🏗️structure/🟦️.ts";
import { magnitude, planeOf } from "../🏗️structure/📽️projection/🟦️.ts";
import { solveThermalAt } from "../🌡️thermal/🟦️.ts";
import { solveTransport } from "../💨️transport/🟦️.ts";
import { type DimensionExtent, dimensionLine, legendBlock, scaleBar, titleBlock } from "./🏷️annotations/🟦️.ts";
import { Canvas, type Ctx, Labels, labelBudget } from "./🧱️canvas/🟦️.ts";
import { type DiagramKind, diagramPolygons, forceList, forcesView, momentsView, peakOf } from "./🏗️structural-view/🟦️.ts";
import { type Theme, THEMES, toneColour, tx } from "./🖍️palette/🟦️.ts";
import { type AnalyticView, type Prepared, type Source, baseStyle, drawShapes, outlineRuns, prepare, sourceOf, themeOf } from "./🧩️shapes/🟦️.ts";
import { arrowOutline, commandsOf } from "./📐️geometry/🟦️.ts";
import { ViewMissingError, axonometricCamera, projectionOf } from "./🔭️views/🟦️.ts";
import type { AxonometricFigure } from "./🧊️axonometric/🟦️.ts";
import { type Checkpoint, drive, driveAsync } from "./⏳️steps/🟦️.ts";
import { SECTION_MARKER_MAX_DENOMINATOR, drawSectionMarkers, sectionOf, viewpointCaption } from "./✂️section/🟦️.ts";
import { thermalView } from "./🌡️thermal-view/🟦️.ts";
import { transportView } from "./💨️transport-view/🟦️.ts";
import * as portable from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

const GROUND_OVERHANG_M = 0.3;

type AxonometricModule = typeof import("./🧊️axonometric/🟦️.ts");

let axonometricModule: AxonometricModule | undefined;

let axonometricLoading: Promise<AxonometricModule> | undefined;

/** 🧊️ Thrown by {@link depict} and {@link modelScene} for the `axonometric` viewpoint before its module (the volume solids, the kernel tessellation and the hidden-line removal) was loaded; `loadAxonometric` or the asynchronous {@link depictAsync} loads it. */
export class AxonometricNotLoadedError extends Error {
  constructor() {
    super("the axonometric depiction is not loaded: await loadAxonometric() or use depictAsync");
    this.name = "AxonometricNotLoadedError";
  }

  /** 🌍️ The refusal in both languages. */
  get reason(): Text {
    return { en: this.message, de: "Die axonometrische Darstellung ist nicht geladen: loadAxonometric() abwarten oder depictAsync verwenden." };
  }
}

/** 🦥️ Loads the axonometric depiction (volume solids, kernel tessellation, hidden lines) once, as a chunk of its own: a program that draws the planar viewpoints only never pays for it. Resolves when {@link depict} and {@link modelScene} can draw the `axonometric` viewpoint. */
export async function loadAxonometric(): Promise<void> {
  axonometricLoading ??= import("./🧊️axonometric/🟦️.ts");
  axonometricModule = await axonometricLoading;
}

const axonometricOf = (): AxonometricModule => {
  if (axonometricModule === undefined) throw new AxonometricNotLoadedError();
  return axonometricModule;
};

/** 🧍️ The shapes that belong to the figure: everything except role `ground`. Ground shapes are drawn but never count toward the extent a dimension measures or a ground line spans, in any viewpoint. */
const figureOf = (shapes: readonly Prepared[]): readonly Prepared[] => shapes.filter((p) => p.shape.role !== "ground");

/** 📏️ The horizontal extent in metres a ground line spans: the rig bounds in the elevation, the drawn shapes in the front elevation, none in the plan. */
function groundSpan(viewpoint: Viewpoint, bounds: EvaluatedState["bounds"], shapes: readonly Prepared[], k: number): { readonly minX: number; readonly maxX: number } | null {
  if (viewpoint === "elevation") return { minX: bounds.minX, maxX: bounds.maxX };
  if (viewpoint !== "front" || shapes.length === 0) return null;
  return { minX: Math.min(...shapes.map((p) => p.box.minX)) / k, maxX: Math.max(...shapes.map((p) => p.box.maxX)) / k };
}
/** 📐️ The extent in metres the overall dimension measures: the rig bounds in the side elevation (the height), the drawn shapes of the viewpoint itself in front (the height) and plan (the depth along the paper's vertical axis), never the side view's extent; none when nothing is drawn. */
export function dimensionExtent(viewpoint: Viewpoint, bounds: EvaluatedState["bounds"], shapes: readonly Prepared[], k: number): DimensionExtent | null {
  if (viewpoint === "elevation") return bounds;
  if (shapes.length === 0) return null;
  return {
    minX: Math.min(...shapes.map((p) => p.box.minX)) / k,
    maxX: Math.max(...shapes.map((p) => p.box.maxX)) / k,
    minY: -Math.max(...shapes.map((p) => p.box.maxY)) / k,
    maxY: -Math.min(...shapes.map((p) => p.box.minY)) / k,
  };
}
/** 🏷️ The localized title of each of the seven views, the only user-facing spelling of a view id. */
export const VIEW_TITLES = {
  "dark-line": tx("Dark line", "Dunkle Strichzeichnung"),
  "dark-solid": tx("Dark solid", "Dunkler Vollton"),
  "light-line": tx("Light line", "Helle Strichzeichnung"),
  "light-solid": tx("Light solid", "Heller Vollton"),
  thermal: tx("Thermal", "Thermisch"),
  forces: tx("Forces", "Kräfte"),
  moments: tx("Moments", "Momente"),
} as const satisfies Record<View, ReturnType<typeof tx>>;

/** 🔭️ The localized name of each kind of axonometric projection, shown in the caption of the view. */
export const AXONOMETRIC_NAMES = {
  isometric: tx("isometric", "isometrisch"),
  dimetric: tx("dimetric", "dimetrisch"),
  free: tx("free angles", "freie Winkel"),
} as const satisfies Record<AxonometricKind, ReturnType<typeof tx>>;

/** 🎛️ Sensible options for a view at a scale denominator under the default print profile, drawing the representation authored for that scale; the language has no default and must be named. */
export function defaultDepictionOptions(view: View, language: Language, denominator = 100, viewpoint: Viewpoint = "elevation", theme?: Theme): DepictionCommon & TextualDepiction {
  return {
    ...commonOptions(view, denominator, viewpoint, theme),
    language,
    annotations: { dimensions: true, scaleBar: true, legend: true, labels: true, ground: true },
    diagrams: { axial: false, shear: false, moment: true, freeBody: true },
  };
}

/** 🤐️ Sensible options for a depiction that renders no text, such as a cast of figures: it names no language because it needs none, and only the ground line remains as annotation. */
export function textlessDepictionOptions(view: View, denominator = 100, viewpoint: Viewpoint = "elevation", theme?: Theme): DepictionCommon & TextlessDepiction {
  return {
    ...commonOptions(view, denominator, viewpoint, theme),
    annotations: { dimensions: false, scaleBar: false, legend: false, labels: false, ground: true },
    diagrams: { axial: false, shear: false, moment: true, freeBody: false },
  };
}

function commonOptions(view: View, denominator: number, viewpoint: Viewpoint, theme: Theme | undefined): DepictionCommon {
  return {
    view,
    viewpoint,
    scale: { denominator },
    profile: DEFAULT_PRINT_PROFILE,
    representation: { kind: "scale", scale: { denominator } },
    transparent: false,
    marginMm: 5,
    thermalQuantity: "surface-temperature",
    forceQuantity: "axial",
    range: { mode: "shared" },
    patterns: true,
    transport: [],
    ...(theme ? { theme } : {}),
  };
}

type WithoutView<T> = T extends unknown ? Omit<T, "view"> : never;

/** 🔤️ The language of a depiction that renders text; throws when a textless depiction reaches text, since no language exists to fall back on. */
function requireLanguage(options: DepictionOptions): Language {
  if (options.language === undefined) throw new Error("a textless depiction has no language; text needs options.language");
  return options.language;
}

/** 🧮️ The analyses the analytic views read; absent ones are solved on demand. */
export type Analysis = { readonly thermal?: ThermalField; readonly structure?: StructuralResult; readonly transport?: TransportField };

function checkState(state: EvaluatedState, options: DepictionOptions, resolution: Extract<RepresentationResolution, { readonly kind: "authored" | "symbol" }>): void {
  const mine = state.representation;
  const fail = (): never => {
    throw new Error(`the state of "${state.asset.id}" was evaluated with a different representation than the options ask for; evaluate(asset, query, options.representation)`);
  };
  if (resolution.kind === "symbol") {
    if (mine.kind !== "symbol" || mine.symbol !== resolution.symbol.id) fail();
  } else if (mine.kind !== "scale" || mine.denominator !== resolution.denominator || mine.requested !== resolution.requested) fail();
}

/** 🧭️ The representation a depiction of the state under the options draws; throws `RepresentationError` when the scale is unavailable or unsuitable for the profile. */
export function resolveDepiction(asset: EvaluatedState["asset"], options: DepictionOptions): Extract<RepresentationResolution, { readonly kind: "authored" | "symbol" }> {
  const resolution = options.representation.kind === "symbol" ? resolveSymbol(asset, options.representation.symbol, options.scale.denominator) : resolveRepresentation(asset, options.scale, options.profile);
  return requireDrawable(resolution, asset.id);
}

/** 🖌️ Depicts a state in one of the seven views at the option's scale. Deterministic in `(state, options, analysis)`; throws `RepresentationError` for an unavailable or unsuitable scale and `ViewMissingError` for a viewpoint the asset has no drawing of. The `axonometric` viewpoint draws the 3D volume (see ./🧊️axonometric) and is the one expensive depiction: `control` gives it progress and cancellation (the signal is checked between steps, a cancelled run throws the signal's reason), {@link depictAsync} yields to the event loop between steps. */
export function depict(state: EvaluatedState, options: DepictionOptions, analysis: Analysis = {}, control?: JobControl): DrawingScene {
  return drive(depictSteps(state, options, analysis), control);
}

/** 🦄 {@link depict} as a promise that yields to the event loop between the steps of an axonometric depiction, so a long one neither blocks the thread nor a cancellation through `control.signal`; the planar viewpoints finish in one step. */
export async function depictAsync(state: EvaluatedState, options: DepictionOptions, analysis: Analysis = {}, control?: JobControl): Promise<DrawingScene> {
  if (options.viewpoint === "axonometric") await loadAxonometric();
  return driveAsync(depictSteps(state, options, analysis), control);
}

function* depictSteps(state: EvaluatedState, options: DepictionOptions, analysis: Analysis): Generator<Checkpoint, DrawingScene> {
  const resolution = resolveDepiction(state.asset, options);
  checkState(state, options, resolution);
  const k = mmPerMetre(options.scale);
  const theme = themeOf(options.view, options.theme);
  const colours = THEMES[theme];
  const camera = options.viewpoint === "axonometric" ? axonometricCamera(options.axonometric) : undefined;
  const ctx: Ctx = { state, options, get language(): Language { return requireLanguage(options); }, k, theme, colours, paper: (p: Point) => ({ x: p.x * k, y: -p.y * k }), labels: new Labels(), ...(camera ? { camera } : {}) };
  const canvas = new Canvas();
  const source = sourceOf(state);
  const thermal = options.view === "thermal" ? (analysis.thermal ?? solveThermalAt(state.asset, state.query, options.environmentTimeline ?? options.environment)) : undefined;
  const structure = options.view === "forces" || options.view === "moments" ? (analysis.structure ?? solveStructure(state)) : undefined;
  const b = state.bounds;
  const metric = !source.paperUnits;
  let shapes: readonly Prepared[] = [];
  let analytic: AnalyticView | null = null;
  let figure: AxonometricFigure | null = null;
  if (camera !== undefined) {
    if (resolution.kind === "symbol") throw new ViewMissingError(state.asset.id, "axonometric");
    const emitters = thermal !== undefined && state.asset.thermal.emitters.length > 0 ? (analysis.transport ?? solveTransport(state.asset, state.query, thermal.environment, { maxAgeS: 0.5, pathlines: false, streamlines: false })).emitters : undefined;
    figure = yield* axonometricOf().drawAxonometric(canvas, ctx, source, { ...(thermal ? { thermal } : {}), ...(structure ? { structure } : {}), ...(emitters ? { emitters } : {}) }, { groundOverhangM: options.annotations.ground ? GROUND_OVERHANG_M : null });
  } else {
    const viewpoint = options.viewpoint as PlanarViewpoint;
    if (state.shapes[viewpoint].length === 0) throw new ViewMissingError(state.asset.id, viewpoint);
    shapes = prepare(state, viewpoint, source, k);
    if (metric && options.viewpoint === "elevation") for (const corner of [{ x: b.minX, y: b.minY }, { x: b.maxX, y: b.maxY }]) canvas.reach(ctx.paper(corner));
    for (const p of shapes) {
      canvas.reach({ x: p.box.minX, y: p.box.minY });
      canvas.reach({ x: p.box.maxX, y: p.box.maxY });
    }

    const span = groundSpan(options.viewpoint, b, figureOf(shapes), k);
    if (options.annotations.ground && span !== null && metric) {
      canvas.path("ground", "ground:line", commandsOf([ctx.paper({ x: span.minX - GROUND_OVERHANG_M, y: 0 }), ctx.paper({ x: span.maxX + GROUND_OVERHANG_M, y: 0 })], false), { stroke: { color: colours.ink, widthMm: source.pens.ground, cap: "butt" } });
    }

    analytic =
      options.view === "thermal" ? thermalView(ctx, shapes, thermal!)
      : options.view === "forces" ? forcesView(ctx, shapes, structure!)
      : options.view === "moments" ? momentsView(ctx, shapes, structure!)
      : null;

    drawShapes(canvas, ctx, shapes, analytic ? analytic.styleOf : baseStyle(options.view, theme, options.scale.denominator <= 100), options.view.endsWith("-line"));
    analytic?.overlay(canvas);
  }
  if ((options.viewpoint === "elevation" || options.viewpoint === "plan") && metric && options.annotations.labels && options.scale.denominator <= SECTION_MARKER_MAX_DENOMINATOR && resolution.kind === "authored" && resolution.representation.sections !== undefined) drawSectionMarkers(canvas, ctx, resolution.representation.sections, b, options.viewpoint);
  const legendsOfTransport: Legend[] = [];
  if (options.transport.length > 0) {
    const field = analysis.transport ?? solveTransport(state.asset, state.query, options.environmentTimeline ?? options.environment ?? state.asset.thermal.environment);
    const overlay = transportView(ctx, field, options.transport, options.range);
    overlay.overlay(canvas);
    legendsOfTransport.push(...overlay.legends);
  }
  if (figure !== null && options.annotations.dimensions && canvas.box !== null) {
    const content = canvas.box;
    const base = -content.maxY / k;
    dimensionLine(canvas, ctx, { minX: content.minX / k, maxX: content.maxX / k, minY: base, maxY: base + figure.heightM }, content.maxX);
  }
  const extent = figure === null && options.annotations.dimensions && metric ? dimensionExtent(options.viewpoint, b, figureOf(shapes), k) : null;
  if (extent !== null) dimensionLine(canvas, ctx, extent, figureOf(shapes).length === 0 ? extent.maxX * k : Math.max(...figureOf(shapes).map((p) => p.box.maxX)));
  const column = (canvas.box?.maxX ?? 0) + 4;
  const budget = labelBudget(options.scale.denominator);
  const omitted = ctx.labels.flush(canvas, "annotations", column, LABEL_TEXT_MM, colours.ink, 0.1, budget) + (analytic?.inset?.(canvas) ?? 0);
  const legends = [...(analytic?.legends ?? []), ...(figure?.legends ?? []), ...legendsOfTransport];
  const box = canvas.box;
  if (box !== null) {
    let y = box.maxY + 5;
    const x = box.minX;
    if (options.annotations.labels) {
      const viewTitle = VIEW_TITLES[options.view][ctx.language];
      const caption = `${viewTitle} · ${viewpointCaption(options.viewpoint, resolution.kind === "authored" ? sectionOf(resolution.representation, options.viewpoint) : undefined, ctx.language)}${camera === undefined ? "" : ` (${AXONOMETRIC_NAMES[camera.kind][ctx.language]})`}`;
      y += titleBlock(canvas, ctx, x, y, caption) + 2;
    }
    if (options.annotations.scaleBar) y += scaleBar(canvas, ctx, x, y) + 2;
    if (options.annotations.legend) legends.forEach((legend, i) => (y += legendBlock(canvas, ctx, legend, `legend:${i}:${legend.quantity}`, x, y) + 1));
    if (options.annotations.labels && omitted > 0) canvas.text("annotations", "note:omitted-values", { x, y: y + LABEL_TEXT_MM }, ctx.language === "de" ? `${omitted} weitere Werte im Inspektor.` : `${omitted} further values in the inspector.`, LABEL_TEXT_MM, colours.ink);
  }

  const done = canvas.finish(options.marginMm);
  return {
    widthMm: done.widthMm,
    heightMm: done.heightMm,
    background: options.transparent ? null : colours.background,
    layers: done.layers,
    legends,
    metadata: { asset: state.asset.id, title: state.asset.title, query: state.query, timeS: state.timeS, view: options.view, viewpoint: options.viewpoint, scale: options.scale, resolution: resolution.kind, omittedValues: omitted, fidelity: resolution.kind === "authored" || resolution.symbol.semantics === "true-extent" ? "physical" : "conventional-symbol", representation: state.representation, profile: options.profile.id, paperUnits: "mm", modelUnits: "m", mmPerMetre: k, origin: { xMm: done.origin.x, yMm: done.origin.y }, ...(camera ? { projection: projectionOf(camera) } : {}) },
  };
}

/** 🖼️ Depicts a state in all seven views, solving each analysis at most once. */
export function depictAll(state: EvaluatedState, options: WithoutView<DepictionOptions>, analysis: Analysis = {}): Record<View, DrawingScene> {
  const transport = analysis.transport ?? (options.transport.length > 0 ? solveTransport(state.asset, state.query, options.environmentTimeline ?? options.environment ?? state.asset.thermal.environment) : undefined);
  const solved: Analysis = { thermal: analysis.thermal ?? solveThermalAt(state.asset, state.query, options.environmentTimeline ?? options.environment), structure: analysis.structure ?? solveStructure(state), ...(transport ? { transport } : {}) };
  return Object.fromEntries(VIEWS.map((view) => [view, depict(state, { ...options, view }, solved)])) as Record<View, DrawingScene>;
}

const CAD_LAYERS = {
  body: { name: "STAFFAGE-BODY", aci: 7 },
  detail: { name: "STAFFAGE-DETAIL", aci: 8 },
  texture: { name: "STAFFAGE-TEXTURE", aci: 9 },
  glazing: { name: "STAFFAGE-GLAZING", aci: 4 },
  ground: { name: "GROUND", aci: 3 },
} as const;

/** 🏗️ The CAD model-space scene in metres: hidden-line-cleaned outlines on STAFFAGE-BODY/-DETAIL/-TEXTURE/-GLAZING and GROUND with ACI colours and the lineweights (1/100 mm) of the pens of the representation the state was evaluated with; optional ANALYSIS-ARROWS and ANALYSIS-DIAGRAM layers from a structural result. Paper-unit symbols have no model extent and yield no outlines.
 * For the `axonometric` viewpoint the polylines are the visible silhouette, crease and interpenetration lines of the 3D volume projected through `options.axonometric` (default isometric) into the picture plane, in metres of that plane (the vertical axis true to scale); a structural result adds the outlines of its 3D arrows to ANALYSIS-ARROWS and no diagrams. Throws `ViewMissingError` for an asset without volume or a symbol state. */
export function modelScene(state: EvaluatedState, options: { readonly viewpoint: Viewpoint; readonly analysis?: Analysis; readonly diagrams?: readonly DiagramKind[]; readonly axonometric?: AxonometricSpec }): ModelScene {
  if (options.viewpoint === "axonometric") return axonometricModelScene(state, options);
  const viewpoint = options.viewpoint;
  const millimetres = 1000;
  const source: Source = sourceOf(state);
  const shapes = source.paperUnits ? [] : prepare(state, viewpoint, source, millimetres);
  const toModel = (p: { readonly x: number; readonly y: number }): Point => ({ x: p.x / millimetres, y: -p.y / millimetres });
  const polylines = new Map<keyof typeof CAD_LAYERS, ModelLayer["polylines"][number][]>();
  for (const { p, run } of outlineRuns(shapes, true)) {
    const list = polylines.get(p.shape.role) ?? [];
    list.push({ points: run.points.map(toModel), closed: run.closed });
    polylines.set(p.shape.role, list);
  }
  const b = state.bounds;
  const span = groundSpan(options.viewpoint, b, figureOf(shapes), millimetres);
  if (span !== null && !source.paperUnits) polylines.set("ground", [...(polylines.get("ground") ?? []), { points: [{ x: span.minX - GROUND_OVERHANG_M, y: 0 }, { x: span.maxX + GROUND_OVERHANG_M, y: 0 }], closed: false }]);
  const layers: ModelLayer[] = (Object.keys(CAD_LAYERS) as (keyof typeof CAD_LAYERS)[]).flatMap((role) => {
    const list = polylines.get(role);
    if (!list?.length) return [];
    return [{ name: CAD_LAYERS[role].name, aci: CAD_LAYERS[role].aci, lineweight: Math.round((source.pens[role] ?? source.pens.body) * 100), polylines: list, texts: [] }];
  });
  const result = options.analysis?.structure;
  if (result) {
    const height = Math.max(1e-6, b.maxY - b.minY);
    const forces = forceList(state, result).map((f) => ({ ...f, flat: planeOf(options.viewpoint, f.force), where: planeOf(options.viewpoint, f.at) }));
    const max = Math.max(0, ...forces.map((f) => magnitude(f.force)));
    const arrows = forces.filter((f) => f.drawn).flatMap((f) => {
      const mag = portable.hypot(f.flat.x, f.flat.y);
      if (mag < 1e-9 || max <= 0) return [];
      const dir = { x: f.flat.x / mag, y: f.flat.y / mag };
      const length = Math.max(0.05 * height, (mag / max) * 0.5 * height);
      const tail = { x: f.where.x - dir.x * length, y: f.where.y - dir.y * length };
      return [{ points: arrowOutline(tail, f.where, 0.004 * height, 0.06 * height, 0.04 * height), closed: true }];
    });
    if (arrows.length) layers.push({ name: "ANALYSIS-ARROWS", aci: 1, lineweight: 25, polylines: arrows, texts: [] });
    const diagrams = (options.diagrams ?? ["moment"]).flatMap((kind) => {
      const peak = peakOf(result, kind);
      return peak > 1e-9 ? diagramPolygons(state, result, kind, options.viewpoint, peak, (p) => p, 0.1 * height, false).map((d) => ({ points: d.points, closed: true })) : [];
    });
    if (diagrams.length) layers.push({ name: "ANALYSIS-DIAGRAM", aci: 5, lineweight: 18, polylines: diagrams, texts: [] });
  }
  return { units: "m", asset: state.asset.id, timeS: state.timeS, representation: state.representation, layers };
}

/** 🪁️ {@link modelScene} as a promise that loads the axonometric depiction first when the viewpoint asks for it. */
export async function modelSceneAsync(state: EvaluatedState, options: { readonly viewpoint: Viewpoint; readonly analysis?: Analysis; readonly diagrams?: readonly DiagramKind[]; readonly axonometric?: AxonometricSpec }): Promise<ModelScene> {
  if (options.viewpoint === "axonometric") await loadAxonometric();
  return modelScene(state, options);
}

const MODEL_MM = 1000;

function axonometricModelScene(state: EvaluatedState, options: { readonly analysis?: Analysis; readonly axonometric?: AxonometricSpec }): ModelScene {
  const representation = state.representation;
  if (representation.kind !== "scale") throw new ViewMissingError(state.asset.id, "axonometric");
  const result = options.analysis?.structure;
  const base = defaultDepictionOptions(result ? "forces" : "light-line", "en", representation.requested, "axonometric", "light");
  const drawing: DepictionOptions = { ...base, ...(options.axonometric ? { axonometric: options.axonometric } : {}), representation: { kind: "scale", scale: { denominator: representation.requested } }, annotations: { ...base.annotations, ground: true } };
  const camera = axonometricCamera(options.axonometric);
  const source = sourceOf(state);
  const ctx: Ctx = { state, options: drawing, get language(): Language { return requireLanguage(drawing); }, k: MODEL_MM, theme: "light", colours: THEMES.light, paper: (p: Point) => ({ x: p.x * MODEL_MM, y: -p.y * MODEL_MM }), labels: new Labels(), camera };
  const canvas = new Canvas();
  const emitters = undefined;
  drive(axonometricOf().drawAxonometric(canvas, ctx, source, { ...(result ? { structure: result } : {}), ...(emitters ? { emitters } : {}) }, { glyphEdges: true, groundOverhangM: GROUND_OVERHANG_M }));
  const done = canvas.finish(0);
  const toModel = (c: { readonly x: number; readonly y: number }): Point => ({ x: (c.x - done.origin.x) / MODEL_MM, y: -(c.y - done.origin.y) / MODEL_MM });
  const lines = new Map<string, ModelLayer["polylines"][number][]>();
  for (const layer of done.layers) {
    if (layer.id === "fill" || layer.id === "legend" || layer.id === "annotations" || layer.id === "diagrams" || layer.id === "field" || layer.id === "background") continue;
    for (const item of layer.items) {
      if (item.kind !== "path" || item.stroke === undefined) continue;
      const list = lines.get(layer.id) ?? [];
      let current: Point[] = [];
      const flush = (closed: boolean): void => {
        if (current.length > 1) list.push({ points: current, closed });
        current = [];
      };
      for (const c of item.commands) {
        if (c.op === "M") {
          flush(false);
          current.push(toModel(c));
        } else if (c.op === "L") current.push(toModel(c));
        else if (c.op === "Z") flush(true);
      }
      flush(false);
      lines.set(layer.id, list);
    }
  }
  const layers: ModelLayer[] = (Object.keys(CAD_LAYERS) as (keyof typeof CAD_LAYERS)[]).flatMap((role) => {
    const list = lines.get(role);
    return list?.length ? [{ name: CAD_LAYERS[role].name, aci: CAD_LAYERS[role].aci, lineweight: Math.round((source.pens[role] ?? source.pens.body) * 100), polylines: list, texts: [] }] : [];
  });
  const arrows = lines.get("forces");
  if (arrows?.length) layers.push({ name: "ANALYSIS-ARROWS", aci: 1, lineweight: 25, polylines: arrows, texts: [] });
  return { units: "m", asset: state.asset.id, timeS: state.timeS, representation: state.representation, layers };
}
