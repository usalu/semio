/** 🎨️ Depiction: one {@link EvaluatedState} becomes any of seven synchronized paper-space views ({@link depict}) or a CAD model-space scene ({@link modelScene}).
 *
 * Paper space is millimetres, y down. The scene size is the asset bounds (ground line, plumes, arrows, diagrams, annotations when shown) × `mmPerMetre`
 * plus margins; `metadata.origin` is where the model origin landed, so a model point `(x, y)` sits at paper `(origin.xMm + x·k, origin.yMm − y·k)` with
 * `k = mmPerMetre`. Every view paints the same prepared rings (identical `fill` item commands per shape id, relative to the origin); line views add
 * role-weighted outlines with hidden lines removed, solid views add tones, analytic views add legends with unit, sign convention and interpretation.
 * Analytic views use `options.theme` (default light).
 *
 * @see ./🧩️shapes/🟦️.ts — level of detail, z order, hidden lines
 * @see ../📤️export/🖼️svg/🟦️.ts — SVG serialisation
 * @see ../🔗️contracts/🟦️.ts — {@link DepictionOptions}, {@link DrawingScene}, {@link ModelScene}
 */
import type { Language, Viewpoint } from "../../🧬️schema/🟦️.ts";
import { VIEWS, type DepictionOptions, type DrawingScene, type EvaluatedState, type ModelLayer, type ModelScene, type Point, type StructuralResult, type ThermalField, type View } from "../🔗️contracts/🟦️.ts";
import { LABEL_TEXT_MM, lineWeightMm, lodFor, mmPerMetre } from "../📐️units/🟦️.ts";
import { solveStructure } from "../🏗️structure/🟦️.ts";
import { solveThermal } from "../🌡️thermal/🟦️.ts";
import { dimensionLine, legendBlock, scaleBar, titleBlock } from "./🏷️annotations/🟦️.ts";
import { Canvas, type Ctx, Labels, labelBudget } from "./🧱️canvas/🟦️.ts";
import { type DiagramKind, diagramPolygons, forceList, forcesView, momentsView, peakOf } from "./🏗️structural-view/🟦️.ts";
import { type Theme, THEMES, toneColour, tx } from "./🖍️palette/🟦️.ts";
import { type AnalyticView, type Prepared, type Style, drawShapes, outlineRuns, prepare } from "./🧩️shapes/🟦️.ts";
import { arrowOutline, commandsOf } from "./📐️geometry/🟦️.ts";
import { thermalView } from "./🌡️thermal-view/🟦️.ts";

const GROUND_OVERHANG_M = 0.3;
const CAD_PEN_SCALE = { denominator: 100 } as const;
const VIEW_TITLES = {
  "dark-line": tx("Line drawing, dark", "Strichzeichnung, dunkel"),
  "dark-solid": tx("Solid, dark", "Vollton, dunkel"),
  "light-line": tx("Line drawing, light", "Strichzeichnung, hell"),
  "light-solid": tx("Solid, light", "Vollton, hell"),
  thermal: tx("Thermal", "Thermisch"),
  forces: tx("Forces", "Kräfte"),
  moments: tx("Moments", "Momente"),
} as const satisfies Record<View, ReturnType<typeof tx>>;

/** 🎛️ Sensible options for a view at a scale denominator; the language has no default and must be named. */
export function defaultDepictionOptions(view: View, language: Language, denominator = 100, viewpoint: Viewpoint = "elevation", theme?: Theme): DepictionOptions {
  return {
    view,
    viewpoint,
    scale: { denominator },
    language,
    transparent: false,
    marginMm: 5,
    annotations: { dimensions: true, scaleBar: true, legend: true, labels: true, ground: true },
    thermalQuantity: "surface-temperature",
    forceQuantity: "axial",
    diagrams: { axial: false, shear: false, moment: true, freeBody: true },
    range: { mode: "auto" },
    patterns: false,
    ...(theme ? { theme } : {}),
  };
}

/** 🧮️ The analyses the analytic views read; absent ones are solved on demand. */
export type Analysis = { readonly thermal?: ThermalField; readonly structure?: StructuralResult };

function themeOf(view: View, chosen: Theme | undefined): Theme {
  return view === "dark-line" || view === "dark-solid" ? "dark" : view === "light-line" || view === "light-solid" ? "light" : (chosen ?? "light");
}

function baseStyle(view: View, theme: Theme, scaleDenominator: number): (p: Prepared) => Style {
  const line = view.endsWith("-line");
  const background = THEMES[theme].background;
  return (p) => {
    const glaze = p.shape.role === "glazing";
    if (line) return { fill: glaze ? null : { color: background }, outlineMm: null };
    return { fill: { color: toneColour(theme, p.shape.tone), ...(glaze ? { opacity: 0.45 } : {}) }, outlineMm: scaleDenominator <= 100 ? p.weightMm * 0.5 : null };
  };
}

/** 🎨️ Depicts a state in one of the seven views at the option's scale. Deterministic in `(state, options, analysis)`. */
export function depict(state: EvaluatedState, options: DepictionOptions, analysis: Analysis = {}): DrawingScene {
  const k = mmPerMetre(options.scale);
  const lod = lodFor(options.scale);
  const theme = themeOf(options.view, options.theme);
  const colours = THEMES[theme];
  const ctx: Ctx = { state, options, language: options.language, k, theme, colours, paper: (p: Point) => ({ x: p.x * k, y: -p.y * k }), labels: new Labels() };
  const canvas = new Canvas();
  const shapes = prepare(state, options.viewpoint, options.scale, lod, k);
  const b = state.bounds;
  for (const corner of [{ x: b.minX, y: b.minY }, { x: b.maxX, y: b.maxY }]) canvas.reach(ctx.paper(corner));

  if (options.annotations.ground && options.viewpoint === "elevation") {
    const weight = lineWeightMm(options.scale, "ground") ?? 0.35;
    canvas.path("ground", "ground:line", commandsOf([ctx.paper({ x: b.minX - GROUND_OVERHANG_M, y: 0 }), ctx.paper({ x: b.maxX + GROUND_OVERHANG_M, y: 0 })], false), { stroke: { color: colours.ink, widthMm: weight, cap: "butt" } });
  }

  const analytic: AnalyticView | null =
    options.view === "thermal" ? thermalView(ctx, shapes, analysis.thermal ?? solveThermal(state, options.environment))
    : options.view === "forces" ? forcesView(ctx, shapes, analysis.structure ?? solveStructure(state))
    : options.view === "moments" ? momentsView(ctx, shapes, analysis.structure ?? solveStructure(state))
    : null;

  drawShapes(canvas, ctx, shapes, analytic ? analytic.styleOf : baseStyle(options.view, theme, options.scale.denominator), options.view.endsWith("-line"));
  analytic?.overlay(canvas);
  if (options.annotations.dimensions) dimensionLine(canvas, ctx);
  const column = (canvas.box?.maxX ?? 0) + 4;
  const budget = labelBudget(options.scale.denominator);
  const omitted = ctx.labels.flush(canvas, "annotations", column, LABEL_TEXT_MM, colours.ink, 0.1, budget) + (analytic?.inset?.(canvas) ?? 0);
  const legends = (analytic?.legends ?? []).map((l, i) => (i === 0 && omitted > 0 ? { ...l, interpretation: { en: `${l.interpretation.en} ${omitted} further values in the inspector.`, de: `${l.interpretation.de} ${omitted} weitere Werte im Inspektor.` } } : l));
  const box = canvas.box;
  if (box !== null) {
    let y = box.maxY + 5;
    const x = box.minX;
    if (options.annotations.labels) y += titleBlock(canvas, ctx, x, y, VIEW_TITLES[options.view][options.language]) + 2;
    if (options.annotations.scaleBar) y += scaleBar(canvas, ctx, x, y) + 2;
    if (options.annotations.legend) legends.forEach((legend, i) => (y += legendBlock(canvas, ctx, legend, `legend:${i}:${legend.quantity}`, x, y) + 1));
  }

  const done = canvas.finish(options.marginMm);
  return {
    widthMm: done.widthMm,
    heightMm: done.heightMm,
    background: options.transparent ? null : colours.background,
    layers: done.layers,
    legends,
    metadata: { asset: state.asset.id, title: state.asset.title, query: state.query, timeS: state.timeS, view: options.view, viewpoint: options.viewpoint, scale: options.scale, paperUnits: "mm", modelUnits: "m", mmPerMetre: k, origin: { xMm: done.origin.x, yMm: done.origin.y } },
  };
}

/** 🖼️ Depicts a state in all seven views, solving each analysis at most once. */
export function depictAll(state: EvaluatedState, options: Omit<DepictionOptions, "view">, analysis: Analysis = {}): Record<View, DrawingScene> {
  const solved: Analysis = { thermal: analysis.thermal ?? solveThermal(state, options.environment), structure: analysis.structure ?? solveStructure(state) };
  return Object.fromEntries(VIEWS.map((view) => [view, depict(state, { ...options, view }, solved)])) as Record<View, DrawingScene>;
}

const CAD_LAYERS = {
  body: { name: "STAFFAGE-BODY", aci: 7 },
  detail: { name: "STAFFAGE-DETAIL", aci: 8 },
  texture: { name: "STAFFAGE-TEXTURE", aci: 9 },
  glazing: { name: "STAFFAGE-GLAZING", aci: 4 },
  ground: { name: "GROUND", aci: 3 },
} as const;

/** 📐️ The CAD model-space scene in metres: hidden-line-cleaned outlines on STAFFAGE-BODY/-DETAIL/-TEXTURE/-GLAZING and GROUND with ACI colours and the lineweights (1/100 mm) of the 1:100 pen table; optional ANALYSIS-ARROWS and ANALYSIS-DIAGRAM layers from a structural result. */
export function modelScene(state: EvaluatedState, options: { readonly viewpoint: Viewpoint; readonly analysis?: Analysis; readonly diagrams?: readonly DiagramKind[] }): ModelScene {
  const millimetres = 1000;
  const shapes = prepare(state, options.viewpoint, CAD_PEN_SCALE, 3, millimetres);
  const toModel = (p: { readonly x: number; readonly y: number }): Point => ({ x: p.x / millimetres, y: -p.y / millimetres });
  const polylines = new Map<keyof typeof CAD_LAYERS, ModelLayer["polylines"][number][]>();
  for (const { p, run } of outlineRuns(shapes, true)) {
    const list = polylines.get(p.shape.role) ?? [];
    list.push({ points: run.points.map(toModel), closed: run.closed });
    polylines.set(p.shape.role, list);
  }
  const b = state.bounds;
  if (options.viewpoint === "elevation") polylines.set("ground", [...(polylines.get("ground") ?? []), { points: [{ x: b.minX - GROUND_OVERHANG_M, y: 0 }, { x: b.maxX + GROUND_OVERHANG_M, y: 0 }], closed: false }]);
  const layers: ModelLayer[] = (Object.keys(CAD_LAYERS) as (keyof typeof CAD_LAYERS)[]).flatMap((role) => {
    const list = polylines.get(role);
    if (!list?.length) return [];
    return [{ name: CAD_LAYERS[role].name, aci: CAD_LAYERS[role].aci, lineweight: Math.round((lineWeightMm(CAD_PEN_SCALE, role) ?? 0.13) * 100), polylines: list, texts: [] }];
  });
  const result = options.analysis?.structure;
  if (result) {
    const height = Math.max(1e-6, b.maxY - b.minY);
    const forces = forceList(state, result);
    const max = Math.max(0, ...forces.map((f) => Math.hypot(f.force.x, f.force.y)));
    const arrows = forces.filter((f) => f.drawn).flatMap((f) => {
      const mag = Math.hypot(f.force.x, f.force.y);
      if (mag < 1e-9 || max <= 0) return [];
      const dir = { x: f.force.x / mag, y: f.force.y / mag };
      const length = Math.max(0.05 * height, (mag / max) * 0.5 * height);
      const tail = { x: f.at.x - dir.x * length, y: f.at.y - dir.y * length };
      return [{ points: arrowOutline(tail, f.at, 0.004 * height, 0.06 * height, 0.04 * height), closed: true }];
    });
    if (arrows.length) layers.push({ name: "ANALYSIS-ARROWS", aci: 1, lineweight: 25, polylines: arrows, texts: [] });
    const diagrams = (options.diagrams ?? ["moment"]).flatMap((kind) => {
      const peak = peakOf(result, kind);
      return peak > 1e-9 ? diagramPolygons(state, result, kind, peak, (p) => p, 0.1 * height, false).map((d) => ({ points: d.points, closed: true })) : [];
    });
    if (diagrams.length) layers.push({ name: "ANALYSIS-DIAGRAM", aci: 5, lineweight: 18, polylines: diagrams, texts: [] });
  }
  return { units: "m", asset: state.asset.id, timeS: state.timeS, layers };
}
