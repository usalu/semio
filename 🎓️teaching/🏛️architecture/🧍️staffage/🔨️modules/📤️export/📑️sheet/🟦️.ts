/** 📑️ Multiview sheet: the plan, front and side elevation of ONE evaluated state composed on one sheet in a projection-angle arrangement, as a plain {@link DrawingScene} so SVG, PDF and the raster paths write it unchanged.
 *
 * Arrangement (the cameras of `🎨️depiction/🔭️views`): the side elevation is the main view, seen from the asset's right with ahead on the right of the paper. The plan shares its x with the elevation, the front shares its y (the ground line) with the elevation.
 * Seen from ahead the asset's left is on the paper's right, and in the plan the asset's left is up the page, so the front's paper x runs the same way as the plan's paper up (+z).
 *
 * | projection | plan | front | name |
 * |---|---|---|---|
 * | `third` | above the elevation | right of the elevation | third-angle (ISO 5456-2 method 3, ANSI) |
 * | `first` | below the elevation | left of the elevation | first-angle (ISO 5456-2 method 1, DIN, SN) |
 *
 * An optional fourth cell holds the axonometric view of the same state in the quadrant the three views leave free: above the front in the third-angle arrangement (right of the plan), below the front in the first-angle arrangement (left of the plan), its edge flush with the column of the front and the row of the plan. It is drawn at the same paper scale (the vertical axis true; what else is to scale is stated in `sheet.axonometric.projection`).
 *
 * All three views are drawn at one paper scale (`mmPerMetre`), share the world origins that align them, and carry overall dimensions measured on the drawn figure, the view captions, the scale bar and the projection symbol.
 * The sheet states widthM (ahead), depthM (lateral) and heightM per view in `sheet.extentsM`, measured on the drawn figure without ground.
 *
 * @see ../../🎨️depiction/🔭️views/🟦️.ts — the cameras
 * @see ../🖼️svg/🟦️.ts — `writeSvg` (writes the sheet metadata)
 * @see https://www.iso.org/standard/3925.html — ISO 128-30 views and ISO 5456-2 orthographic representations
 */
import { PLANAR_VIEWPOINTS, type Asset, type Language, type Text, type PlanarViewpoint, type Viewpoint } from "../../../🧬️schema/🟦️.ts";
import { LAYERS, type AxonometricProjection, type AxonometricSpec, type DepictionOptions, type DrawingItem, type DrawingScene, type EvaluatedState, type EvaluationQuery, type LayerId, type ModelLayer, type ModelScene, type Fidelity, type PathCommand, type View } from "../../🔗️contracts/🟦️.ts";
import { resolveViewpoints, type DrawOptions, type DrawingSource, type DrawingUnavailable } from "../../🔭️multiview/🟦️.ts";
import { Canvas } from "../../🎨️depiction/🧱️canvas/🟦️.ts";
import { type Analysis, AXONOMETRIC_NAMES, VIEW_TITLES, depict, modelScene } from "../../🎨️depiction/🟦️.ts";
import { THEMES, tx } from "../../🎨️depiction/🖍️palette/🟦️.ts";
import { viewpointCaption } from "../../🎨️depiction/✂️section/🟦️.ts";
import { scaleBarMetres } from "../../🎨️depiction/🏷️annotations/🟦️.ts";
import { circleCommands, commandsOf } from "../../🎨️depiction/📐️geometry/🟦️.ts";
import { ANNOTATION_PEN_MM, ANNOTATION_TEXT_MM, LABEL_TEXT_MM, formatNumber } from "../../📐️units/🟦️.ts";
import * as portable from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

/** 📐️ The two projection-angle arrangements. */
export const PROJECTIONS = ["first", "third"] as const;

/** 🔭️ One of {@link PROJECTIONS}. */
export type Projection = (typeof PROJECTIONS)[number];

/** 🏷️ The localized name of a projection arrangement. */
export const PROJECTION_NAMES: Readonly<Record<Projection, Text>> = {
  first: tx("First-angle projection (method 1)", "Projektionsmethode 1 (Europa)"),
  third: tx("Third-angle projection (method 3)", "Projektionsmethode 3 (Amerika)"),
};

/** 🗂️ The localized statement of how a drawing relates to the real object. */
export const FIDELITY_NAMES: Readonly<Record<Fidelity, Text>> = {
  physical: tx("Physical drawing, to scale", "Physikalische Zeichnung, maßstäblich"),
  "conventional-symbol": tx("Conventional symbol, not to scale", "Konventionelles Symbol, nicht maßstäblich"),
};

/** 📏️ The drawn extent of the figure in one view in metres, without ground: along paper x and along paper up. */
export type ViewExtent = { readonly paperXM: number; readonly paperUpM: number };

/** 🧾️ What a sheet states about its arrangement: where each world origin sits on the sheet and the measured extents. */
export type SheetInfo = {
  readonly projection: Projection;
  readonly viewpoints: readonly PlanarViewpoint[];
  readonly fidelity: Readonly<Record<PlanarViewpoint, Fidelity>>;
  readonly originsMm: Readonly<Record<PlanarViewpoint, { readonly xMm: number; readonly yMm: number }>>;
  readonly extentsM: { readonly elevation: { readonly widthM: number; readonly heightM: number }; readonly front: { readonly depthM: number; readonly heightM: number }; readonly plan: { readonly widthM: number; readonly depthM: number } };
  readonly axonometric?: { readonly fidelity: Fidelity; readonly originMm: { readonly xMm: number; readonly yMm: number }; readonly projection: AxonometricProjection };
};

/** 👁️ A drawing scene that is a multiview sheet. */
export type SheetScene = DrawingScene & { readonly sheet: SheetInfo };

/** 🔎️ Whether a scene is a multiview sheet. */
export const isSheet = (scene: DrawingScene): scene is SheetScene => "sheet" in scene;

const GAP_X_MM = 24;
const GAP_Y_MM = 26;
const DIMENSION_OFFSET_MM = 7;
const TICK_MM = 0.9;
const MARGIN_MM = 6;

type Box = { readonly minX: number; readonly minY: number; readonly maxX: number; readonly maxY: number };

const SHEET_TITLE = tx("Multiview sheet", "Mehrtafelzeichnung");

const DIMENSIONS = {
  width: { quantity: "overall-width", label: tx("Overall width", "Gesamtbreite") },
  depth: { quantity: "overall-depth", label: tx("Overall depth", "Gesamttiefe") },
  height: { quantity: "overall-height", label: tx("Overall height", "Gesamthöhe") },
} as const;

const OVERLAYS: readonly LayerId[] = ["background", "ground", "forces", "diagrams", "annotations", "legend"];

function figureBox(scene: DrawingScene): Box {
  let box: Box | null = null;
  const reach = (x: number, y: number): void => void (box = box === null ? { minX: x, minY: y, maxX: x, maxY: y } : { minX: Math.min(box.minX, x), minY: Math.min(box.minY, y), maxX: Math.max(box.maxX, x), maxY: Math.max(box.maxY, y) });
  for (const layer of scene.layers) {
    if (OVERLAYS.includes(layer.id)) continue;
    for (const item of layer.items) if (item.kind === "path") for (const c of item.commands) if (c.op !== "Z") reach(c.x, c.y);
  }
  if (box === null) throw new Error(`the ${scene.metadata.viewpoint} view of ${scene.metadata.asset} draws no figure`);
  return box;
}

const moved = (c: PathCommand, dx: number, dy: number): PathCommand => (c.op === "Z" ? c : c.op === "C" ? { op: "C", x1: c.x1 + dx, y1: c.y1 + dy, x2: c.x2 + dx, y2: c.y2 + dy, x: c.x + dx, y: c.y + dy } : { op: c.op, x: c.x + dx, y: c.y + dy });

function place(canvas: Canvas, scene: DrawingScene, viewpoint: Viewpoint, dx: number, dy: number): void {
  for (const layer of scene.layers)
    for (const item of layer.items as readonly DrawingItem[]) {
      const id = `${viewpoint}:${item.id}`;
      if (item.kind === "text") canvas.text(layer.id, id, { x: item.x + dx, y: item.y + dy }, item.text, item.sizeMm, item.color, { anchor: item.anchor, ...(item.weight ? { weight: item.weight } : {}) });
      else canvas.path(layer.id, id, item.commands.map((c) => moved(c, dx, dy)), { ...(item.stroke ? { stroke: item.stroke } : {}), ...(item.fill ? { fill: item.fill } : {}), ...(item.value ? { value: item.value } : {}), ...(item.bone ? { bone: item.bone } : {}), ...(item.region ? { region: item.region } : {}) });
    }
}

type Sheeter = { readonly canvas: Canvas; readonly ink: string; readonly language: Language; readonly k: number };

function horizontalDimension(s: Sheeter, id: string, x0: number, x1: number, y: number, from: number, kind: keyof typeof DIMENSIONS, valueM: number): void {
  const stroke = { color: s.ink, widthMm: ANNOTATION_PEN_MM, cap: "butt" as const };
  const { quantity, label } = DIMENSIONS[kind];
  s.canvas.path("annotations", `${id}:extension-0`, commandsOf([{ x: x0, y: from + 1.5 }, { x: x0, y: y + 1 }], false), { stroke });
  s.canvas.path("annotations", `${id}:extension-1`, commandsOf([{ x: x1, y: from + 1.5 }, { x: x1, y: y + 1 }], false), { stroke });
  s.canvas.path("annotations", `${id}:line`, commandsOf([{ x: x0, y }, { x: x1, y }], false), { stroke: { ...stroke, widthMm: ANNOTATION_PEN_MM * 1.4 }, value: { quantity, value: valueM, unit: "m", label } });
  for (const [side, x] of [["0", x0], ["1", x1]] as const) s.canvas.path("annotations", `${id}:tick-${side}`, commandsOf([{ x: x - TICK_MM, y: y + TICK_MM }, { x: x + TICK_MM, y: y - TICK_MM }], false), { stroke: { ...stroke, widthMm: ANNOTATION_PEN_MM * 2 } });
  s.canvas.text("annotations", `${id}:text`, { x: (x0 + x1) / 2, y: y + 1 + ANNOTATION_TEXT_MM }, `${formatNumber(valueM, valueM < 10 ? 2 : 1, s.language)} m`, ANNOTATION_TEXT_MM, s.ink, { anchor: "middle" });
}

function verticalDimension(s: Sheeter, id: string, y0: number, y1: number, x: number, from: number, outward: 1 | -1, kind: keyof typeof DIMENSIONS, valueM: number): void {
  const stroke = { color: s.ink, widthMm: ANNOTATION_PEN_MM, cap: "butt" as const };
  const { quantity, label } = DIMENSIONS[kind];
  s.canvas.path("annotations", `${id}:extension-0`, commandsOf([{ x: from + 1.5 * outward, y: y0 }, { x: x + outward, y: y0 }], false), { stroke });
  s.canvas.path("annotations", `${id}:extension-1`, commandsOf([{ x: from + 1.5 * outward, y: y1 }, { x: x + outward, y: y1 }], false), { stroke });
  s.canvas.path("annotations", `${id}:line`, commandsOf([{ x, y: y0 }, { x, y: y1 }], false), { stroke: { ...stroke, widthMm: ANNOTATION_PEN_MM * 1.4 }, value: { quantity, value: valueM, unit: "m", label } });
  for (const [side, y] of [["0", y0], ["1", y1]] as const) s.canvas.path("annotations", `${id}:tick-${side}`, commandsOf([{ x: x - TICK_MM, y: y + TICK_MM }, { x: x + TICK_MM, y: y - TICK_MM }], false), { stroke: { ...stroke, widthMm: ANNOTATION_PEN_MM * 2 } });
  s.canvas.text("annotations", `${id}:text`, { x: x + 1.6 * outward, y: (y0 + y1) / 2 + ANNOTATION_TEXT_MM * 0.35 }, `${formatNumber(valueM, valueM < 10 ? 2 : 1, s.language)} m`, ANNOTATION_TEXT_MM, s.ink, { anchor: outward === 1 ? "start" : "end" });
}

function scaleBar(s: Sheeter, x: number, y: number, denominator: number): number {
  const metres = scaleBarMetres(denominator);
  const n = Math.abs(metres / 10 ** Math.floor(portable.log10(metres)) - 2) < 1e-9 ? 4 : 5;
  const step = (metres * s.k) / n;
  const height = 1.2;
  const background = THEMES.light.background;
  for (let i = 0; i < n; i++) {
    const pts = [{ x: x + i * step, y }, { x: x + (i + 1) * step, y }, { x: x + (i + 1) * step, y: y + height }, { x: x + i * step, y: y + height }];
    s.canvas.path("annotations", `scale-bar:block-${i}`, commandsOf(pts, true), { stroke: { color: s.ink, widthMm: ANNOTATION_PEN_MM, join: "miter" }, fill: { color: i % 2 === 0 ? s.ink : s.ink === THEMES.light.ink ? background : THEMES.dark.background } });
  }
  s.canvas.text("annotations", "scale-bar:zero", { x, y: y + height + 2.2 }, "0", LABEL_TEXT_MM, s.ink, { anchor: "middle" });
  s.canvas.text("annotations", "scale-bar:end", { x: x + metres * s.k, y: y + height + 2.2 }, `${formatNumber(metres, metres < 1 ? 1 : 0, s.language)} m`, LABEL_TEXT_MM, s.ink, { anchor: "middle" });
  return height + 3.4;
}

/** 🔣️ The geometry of the projection-method symbol in millimetres relative to its top-left corner (y down), 8 mm high: a truncated cone seen from the side (`frustum`, four corners) and the same cone seen along its axis, two concentric circles (`circles`, centre `cx`, radii outer then inner). Following ISO 5456-2 (method 1 = first-angle, method 3 = third-angle) and the symbol as drawn there: the cone's small end points to the viewer, so the circles are the view from that end, the outer circle is the large end and the inner circle the small end. The frustum is the side view and is the same in both methods, its small end (shortest side) on the right. First-angle puts the circles left and the frustum right, so the shortest side is away from the circles; third-angle puts the frustum left and the circles right, so the shortest side is toward the circles. */
export function projectionSymbolGeometry(projection: Projection): { readonly frustum: readonly { readonly x: number; readonly y: number }[]; readonly circles: { readonly cx: number; readonly cy: number; readonly radii: readonly [number, number] }; readonly axis: { readonly x0: number; readonly x1: number; readonly y: number } } {
  const left = projection === "first" ? 10 : 0;
  const frustum = [{ x: left, y: 0 }, { x: left + 8, y: 2 }, { x: left + 8, y: 6 }, { x: left, y: 8 }];
  const cx = projection === "first" ? 4 : 14;
  return { frustum, circles: { cx, cy: 4, radii: [4, 2] }, axis: { x0: -1.5, x1: 19.5, y: 4 } };
}

function projectionSymbol(s: Sheeter, x: number, y: number, projection: Projection): void {
  const stroke = { color: s.ink, widthMm: ANNOTATION_PEN_MM * 1.5, cap: "butt" as const, join: "miter" as const };
  const geometry = projectionSymbolGeometry(projection);
  s.canvas.path("annotations", "projection-symbol:frustum", commandsOf(geometry.frustum.map((p) => ({ x: x + p.x, y: y + p.y })), true), { stroke });
  s.canvas.path("annotations", "projection-symbol:axis", commandsOf([{ x: x + geometry.axis.x0, y: y + geometry.axis.y }, { x: x + geometry.axis.x1, y: y + geometry.axis.y }], false), { stroke: { color: s.ink, widthMm: ANNOTATION_PEN_MM, dashMm: [2.4, 0.6, 0.3, 0.6] } });
  s.canvas.path("annotations", "projection-symbol:outer", circleCommands({ x: x + geometry.circles.cx, y: y + geometry.circles.cy }, geometry.circles.radii[0]), { stroke });
  s.canvas.path("annotations", "projection-symbol:inner", circleCommands({ x: x + geometry.circles.cx, y: y + geometry.circles.cy }, geometry.circles.radii[1]), { stroke });
}

/** 🚫️ Thrown when a sheet cannot be composed because a view has no drawing: carries the typed {@link DrawingUnavailable} of every such view, so no view is substituted or left blank. */
export class SheetUnavailableError extends Error {
  constructor(readonly unavailable: readonly DrawingUnavailable[]) {
    super(`no sheet: ${unavailable.map((u) => `${u.viewpoint}: ${u.code} (${u.reason.en})`).join("; ")}`);
    this.name = "SheetUnavailableError";
  }
}

/** 🧮️ The result of {@link resolveSheet}: the sheet and the one state and analysis it was composed from, for the CAD sheet of the same instant. */
export type ResolvedSheet = { readonly sheet: SheetScene; readonly state: EvaluatedState; readonly analysis: Analysis };

/** 🦄 Composes the three scenes of one state (and optionally its axonometric scene, drawn at the same scale from the same state) into a sheet in the given projection arrangement and language; the language has no default. */
export function composeSheet(scenes: Readonly<Record<PlanarViewpoint, DrawingScene>>, projection: Projection, language: Language, axonometric?: DrawingScene): SheetScene {
  const { elevation: side, front, plan } = scenes;
  const first = projection === "first";
  const k = side.metadata.mmPerMetre;
  for (const scene of axonometric === undefined ? Object.values(scenes) : [...Object.values(scenes), axonometric]) {
    if (scene.metadata.mmPerMetre !== k) throw new Error(`the views of a sheet share one scale, got ${scene.metadata.mmPerMetre} and ${k} mm per metre`);
    if (JSON.stringify(scene.metadata.query) !== JSON.stringify(side.metadata.query) || scene.metadata.timeS !== side.metadata.timeS || scene.metadata.view !== side.metadata.view) throw new Error("the views of a sheet are projections of one evaluated state: the same query, time and mode");
  }
  const parts = (scene: DrawingScene) => ({ left: scene.metadata.origin.xMm, right: scene.widthMm - scene.metadata.origin.xMm, top: scene.metadata.origin.yMm, bottom: scene.heightMm - scene.metadata.origin.yMm });
  const e = parts(side);
  const f = parts(front);
  const p = parts(plan);
  const columnLeft = Math.max(e.left, p.left);
  const columnRight = Math.max(e.right, p.right);
  const rowTop = Math.max(e.top, f.top);
  const rowBottom = Math.max(e.bottom, f.bottom);
  const origins: Record<PlanarViewpoint, { xMm: number; yMm: number }> = {
    elevation: { xMm: 0, yMm: 0 },
    plan: { xMm: 0, yMm: first ? rowBottom + GAP_Y_MM + p.top : -(rowTop + GAP_Y_MM + p.bottom) },
    front: { xMm: first ? -(columnLeft + GAP_X_MM + f.right) : columnRight + GAP_X_MM + f.left, yMm: 0 },
  };
  const a = axonometric === undefined ? undefined : parts(axonometric);
  const axoOrigin = a === undefined ? undefined : first ? { xMm: -(columnLeft + GAP_X_MM) - a.right, yMm: rowBottom + GAP_Y_MM + a.top } : { xMm: columnRight + GAP_X_MM + a.left, yMm: -(rowTop + GAP_Y_MM) - a.bottom };
  const canvas = new Canvas();
  const background = side.background ?? THEMES.light.background;
  const theme = Object.values(THEMES).find((t) => t.background === background) ?? THEMES.light;
  const s: Sheeter = { canvas, ink: theme.ink, language, k };
  const cells = {} as Record<PlanarViewpoint, Box>;
  const figures = {} as Record<PlanarViewpoint, Box>;
  for (const viewpoint of PLANAR_VIEWPOINTS) {
    const scene = scenes[viewpoint];
    const dx = origins[viewpoint].xMm - scene.metadata.origin.xMm;
    const dy = origins[viewpoint].yMm - scene.metadata.origin.yMm;
    place(canvas, scene, viewpoint, dx, dy);
    cells[viewpoint] = { minX: dx, minY: dy, maxX: dx + scene.widthMm, maxY: dy + scene.heightMm };
    const box = figureBox(scene);
    figures[viewpoint] = { minX: box.minX + dx, minY: box.minY + dy, maxX: box.maxX + dx, maxY: box.maxY + dy };
  }
  let axoCell: Box | undefined;
  if (axonometric !== undefined && axoOrigin !== undefined) {
    const dx = axoOrigin.xMm - axonometric.metadata.origin.xMm;
    const dy = axoOrigin.yMm - axonometric.metadata.origin.yMm;
    place(canvas, axonometric, "axonometric", dx, dy);
    axoCell = { minX: dx, minY: dy, maxX: dx + axonometric.widthMm, maxY: dy + axonometric.heightMm };
  }
  const metres = (a: number, b: number): number => (b - a) / k;
  const extents = {
    elevation: { widthM: metres(figures.elevation.minX, figures.elevation.maxX), heightM: metres(figures.elevation.minY, figures.elevation.maxY) },
    front: { depthM: metres(figures.front.minX, figures.front.maxX), heightM: metres(figures.front.minY, figures.front.maxY) },
    plan: { widthM: metres(figures.plan.minX, figures.plan.maxX), depthM: metres(figures.plan.minY, figures.plan.maxY) },
  };
  const captionOf = (viewpoint: PlanarViewpoint): string => viewpointCaption(viewpoint, undefined, language);
  const fidelity = Object.fromEntries(PLANAR_VIEWPOINTS.map((viewpoint) => [viewpoint, scenes[viewpoint].metadata.fidelity])) as Record<PlanarViewpoint, Fidelity>;
  for (const viewpoint of PLANAR_VIEWPOINTS) {
    canvas.text("annotations", `caption:${viewpoint}`, { x: cells[viewpoint].minX + 2.5, y: cells[viewpoint].minY - 2.5 }, captionOf(viewpoint), ANNOTATION_TEXT_MM, s.ink, { weight: "bold" });
    if (fidelity[viewpoint] === "conventional-symbol") canvas.text("annotations", `fidelity:${viewpoint}`, { x: cells[viewpoint].minX + 2.5, y: cells[viewpoint].minY - 6 }, FIDELITY_NAMES["conventional-symbol"][language], LABEL_TEXT_MM, s.ink);
  }
  if (axoCell !== undefined) canvas.text("annotations", "caption:axonometric", { x: axoCell.minX + 2.5, y: axoCell.minY - 2.5 }, `${viewpointCaption("axonometric", undefined, language)} (${AXONOMETRIC_NAMES[axonometric!.metadata.projection!.kind][language]})`, ANNOTATION_TEXT_MM, s.ink, { weight: "bold" });
  const bottom = Math.max(cells.elevation.maxY, cells.front.maxY);
  horizontalDimension(s, "dimension:width", figures.elevation.minX, figures.elevation.maxX, bottom + DIMENSION_OFFSET_MM, figures.elevation.maxY, "width", extents.elevation.widthM);
  horizontalDimension(s, "dimension:depth", figures.front.minX, figures.front.maxX, bottom + DIMENSION_OFFSET_MM, figures.front.maxY, "depth", extents.front.depthM);
  const outward = first ? -1 : 1;
  verticalDimension(s, "dimension:height", figures.front.minY, figures.front.maxY, (first ? cells.front.minX : cells.front.maxX) + outward * DIMENSION_OFFSET_MM, first ? figures.front.minX : figures.front.maxX, outward, "height", extents.front.heightM);
  verticalDimension(s, "dimension:plan-depth", figures.plan.minY, figures.plan.maxY, (first ? cells.plan.minX : cells.plan.maxX) + outward * DIMENSION_OFFSET_MM, first ? figures.plan.minX : figures.plan.maxX, outward, "depth", extents.plan.depthM);
  const box = canvas.box!;
  let y = box.maxY + 8;
  const meta = side.metadata;
  const q = meta.query;
  const when = q.kind === "pose" ? q.pose : q.kind === "clip" ? `${q.clip} @ ${formatNumber(q.timeS, 2, language)} s` : `${q.from} → ${q.to} @ ${formatNumber(q.timeS, 2, language)} s`;
  canvas.text("annotations", "title:asset", { x: box.minX, y: y + ANNOTATION_TEXT_MM }, `${meta.title[language]} · ${SHEET_TITLE[language]}`, ANNOTATION_TEXT_MM, s.ink, { weight: "bold" });
  canvas.text("annotations", "title:scale", { x: box.minX, y: y + ANNOTATION_TEXT_MM + 3 }, `1:${meta.scale.denominator} · ${VIEW_TITLES[meta.view][language]} · ${when}`, LABEL_TEXT_MM, s.ink);
  canvas.text("annotations", "title:projection", { x: box.minX, y: y + ANNOTATION_TEXT_MM + 5.6 }, PROJECTION_NAMES[projection][language], LABEL_TEXT_MM, s.ink);
  y += ANNOTATION_TEXT_MM + 8.6;
  scaleBar(s, box.minX, y, meta.scale.denominator);
  projectionSymbol(s, Math.max(box.maxX - 26, box.minX + 40), y - 8, projection);
  const done = canvas.finish(MARGIN_MM);
  const shifted = Object.fromEntries(PLANAR_VIEWPOINTS.map((viewpoint) => [viewpoint, { xMm: origins[viewpoint].xMm + done.origin.x, yMm: origins[viewpoint].yMm + done.origin.y }])) as Record<PlanarViewpoint, { xMm: number; yMm: number }>;
  const layers = LAYERS.flatMap((id: LayerId) => done.layers.filter((layer) => layer.id === id));
  return {
    widthMm: done.widthMm,
    heightMm: done.heightMm,
    background: side.background,
    layers,
    legends: side.legends,
    metadata: { ...meta, fidelity: PLANAR_VIEWPOINTS.some((viewpoint) => fidelity[viewpoint] === "conventional-symbol") ? "conventional-symbol" : "physical", origin: { xMm: shifted.elevation.xMm, yMm: shifted.elevation.yMm } },
    sheet: { projection, viewpoints: PLANAR_VIEWPOINTS, fidelity, originsMm: shifted, extentsM: extents, ...(axonometric === undefined || axoOrigin === undefined ? {} : { axonometric: { fidelity: axonometric.metadata.fidelity, originMm: { xMm: axoOrigin.xMm + done.origin.x, yMm: axoOrigin.yMm + done.origin.y }, projection: axonometric.metadata.projection! } }) },
  };
}

/** 🐙 One call through the multiview resolver: one `evaluate`, one analysis solve, three viewpoints (no margins, no per-view annotations; the legend of an analytic mode is drawn under the elevation), composed in the projection arrangement. `axonometric` (`false` or absent: none) adds the axonometric view of the same state as the fourth cell, through that camera. Throws {@link SheetUnavailableError} naming every view without a drawing. */
export function resolveSheet(asset: Asset, query: EvaluationQuery, source: DrawingSource, mode: View, options: DrawOptions, projection: Projection, language: Language, axonometric: AxonometricSpec | false = false): ResolvedSheet {
  const quiet = (legend: boolean): DrawOptions => ({ ...options, marginMm: 0, transparent: false, annotations: { ...options.annotations, dimensions: false, scaleBar: false, labels: false, legend } }) as DrawOptions;
  const viewpoints: readonly Viewpoint[] = axonometric === false ? PLANAR_VIEWPOINTS : [...PLANAR_VIEWPOINTS, "axonometric"];
  const multi = resolveViewpoints(asset, query, viewpoints, source, mode, axonometric === false ? quiet(false) : ({ ...quiet(false), axonometric } as DrawOptions));
  const missing = viewpoints.flatMap((viewpoint) => (multi.views[viewpoint].kind === "unavailable" ? [multi.views[viewpoint] as DrawingUnavailable] : []));
  if (missing.length > 0 || multi.state === null) throw new SheetUnavailableError(missing);
  const scenes = Object.fromEntries(PLANAR_VIEWPOINTS.map((viewpoint) => [viewpoint, (multi.views[viewpoint] as { scene: DrawingScene }).scene])) as Record<PlanarViewpoint, DrawingScene>;
  if ((mode === "thermal" || mode === "forces" || mode === "moments") && options.annotations.legend) {
    const representation = source.kind === "symbol" ? ({ kind: "symbol", symbol: source.symbol } as const) : ({ kind: "scale", scale: { denominator: source.denominator } } as const);
    scenes.elevation = depict(multi.state, { ...quiet(true), view: mode, viewpoint: "elevation", scale: { denominator: source.denominator }, representation } as DepictionOptions, multi.analysis);
  }
  const axonometricScene = axonometric === false ? undefined : (multi.views.axonometric as { scene: DrawingScene }).scene;
  return { sheet: composeSheet(scenes, projection, language, axonometricScene), state: multi.state, analysis: multi.analysis };
}

const CAD_PREFIX: Readonly<Record<PlanarViewpoint, string>> = { elevation: "ELEVATION", front: "FRONT", plan: "PLAN" };

/** 🪜️ The CAD model-space sheet in metres at full size: the model scenes of the three viewpoints of one state, translated into the projection arrangement with a gap of the paper gaps of the printed sheet at `denominator`, every layer prefixed by its view (`PLAN-STAFFAGE-BODY`) and one `SHEET-LABELS` layer with the localized captions at the text height of 2.5 mm paper. The ground layers stay in their views. */
export function modelSheet(state: EvaluatedState, options: { readonly projection: Projection; readonly denominator: number; readonly language: Language; readonly analysis?: Analysis; readonly diagrams?: Parameters<typeof modelScene>[1]["diagrams"]; readonly axonometric?: AxonometricSpec | false }): ModelScene {
  const first = options.projection === "first";
  const scenes = Object.fromEntries(PLANAR_VIEWPOINTS.map((viewpoint) => [viewpoint, modelScene(state, { viewpoint, ...(options.analysis ? { analysis: options.analysis } : {}), ...(options.diagrams ? { diagrams: options.diagrams } : {}) })])) as Record<PlanarViewpoint, ModelScene>;
  const boxOf = (scene: ModelScene): Box => {
    const points = scene.layers.flatMap((layer) => layer.polylines.flatMap((line) => line.points));
    if (points.length === 0) throw new Error(`the ${scene.asset} model scene of a sheet has no outline`);
    return { minX: Math.min(...points.map((q) => q.x)), minY: Math.min(...points.map((q) => q.y)), maxX: Math.max(...points.map((q) => q.x)), maxY: Math.max(...points.map((q) => q.y)) };
  };
  const boxes = Object.fromEntries(PLANAR_VIEWPOINTS.map((viewpoint) => [viewpoint, boxOf(scenes[viewpoint])])) as Record<PlanarViewpoint, Box>;
  const gapX = (GAP_X_MM * options.denominator) / 1000;
  const gapY = (GAP_Y_MM * options.denominator) / 1000;
  const e = boxes.elevation;
  const f = boxes.front;
  const p = boxes.plan;
  const shift: Record<PlanarViewpoint, { x: number; y: number }> = {
    elevation: { x: 0, y: 0 },
    front: { x: first ? Math.min(e.minX, p.minX) - gapX - f.maxX : Math.max(e.maxX, p.maxX) + gapX - f.minX, y: 0 },
    plan: { x: 0, y: first ? Math.min(e.minY, f.minY) - gapY - p.maxY : Math.max(e.maxY, f.maxY) + gapY - p.minY },
  };
  const layers: ModelLayer[] = PLANAR_VIEWPOINTS.flatMap((viewpoint) => scenes[viewpoint].layers.map((layer) => ({ ...layer, name: `${CAD_PREFIX[viewpoint]}-${layer.name}`, polylines: layer.polylines.map((line) => ({ closed: line.closed, points: line.points.map((q) => ({ x: q.x + shift[viewpoint].x, y: q.y + shift[viewpoint].y })) })) })));
  const axo = options.axonometric === undefined || options.axonometric === false ? undefined : modelScene(state, { viewpoint: "axonometric", ...(options.analysis ? { analysis: options.analysis } : {}), axonometric: options.axonometric });
  const axoBox = axo === undefined ? undefined : boxOf(axo);
  const axoShift = axoBox === undefined ? undefined : first ? { x: Math.min(e.minX, p.minX) - gapX - axoBox.maxX, y: Math.min(e.minY, f.minY) - gapY - axoBox.maxY } : { x: Math.max(e.maxX, p.maxX) + gapX - axoBox.minX, y: Math.max(e.maxY, f.maxY) + gapY - axoBox.minY };
  if (axo !== undefined && axoShift !== undefined) layers.push(...axo.layers.map((layer) => ({ ...layer, name: `AXONOMETRIC-${layer.name}`, polylines: layer.polylines.map((line) => ({ closed: line.closed, points: line.points.map((q) => ({ x: q.x + axoShift.x, y: q.y + axoShift.y })) })) })));
  const height = (ANNOTATION_TEXT_MM * options.denominator) / 1000;
  const texts = PLANAR_VIEWPOINTS.map((viewpoint) => ({ at: { x: boxes[viewpoint].minX + shift[viewpoint].x, y: boxes[viewpoint].maxY + shift[viewpoint].y + height }, heightM: height, text: viewpointCaption(viewpoint, undefined, options.language) }));
  if (axoBox !== undefined && axoShift !== undefined) texts.push({ at: { x: axoBox.minX + axoShift.x, y: axoBox.maxY + axoShift.y + height }, heightM: height, text: viewpointCaption("axonometric", undefined, options.language) });
  layers.push({ name: "SHEET-LABELS", aci: 7, lineweight: 18, polylines: [], texts });
  const base = scenes.elevation;
  return { units: "m", asset: base.asset, timeS: base.timeS, representation: base.representation, layers };
}

/** 📝️ Adds the one-line note of a model scene (asset, instant, representation, viewpoint, scale, fidelity) as a TEXT on layer `STAFFAGE-NOTE` below the drawing, so a DXF or DWG states its view in its content and not only in a header. The text is 2.5 mm paper high at `denominator`. */
export function withNote(scene: ModelScene, note: string, denominator: number): ModelScene {
  const points = scene.layers.flatMap((layer) => layer.polylines.flatMap((line) => line.points));
  const height = (ANNOTATION_TEXT_MM * denominator) / 1000;
  const at = { x: points.length === 0 ? 0 : Math.min(...points.map((p) => p.x)), y: (points.length === 0 ? 0 : Math.min(...points.map((p) => p.y))) - 3 * height };
  return { ...scene, layers: [...scene.layers, { name: "STAFFAGE-NOTE", aci: 7, lineweight: 18, polylines: [], texts: [{ at, heightM: height, text: note }] }] };
}
