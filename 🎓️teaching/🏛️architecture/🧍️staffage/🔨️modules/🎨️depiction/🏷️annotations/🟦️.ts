/** 🏷️ Drawing annotations in constant paper size: overall-height dimension, scale bar, title line and legend blocks.
 *
 * The title, scale bar and legends are stacked below the drawing, left-aligned with its extent; the dimension line stands right of it.
 *
 * @see ../../📐️units/🟦️.ts — text sizes and line weights
 * @see ../../🔗️contracts/🟦️.ts — {@link Legend}
 */
import { formatNumber, decimalsFor, LABEL_TEXT_MM, ANNOTATION_TEXT_MM, ANNOTATION_PEN_MM } from "../../📐️units/🟦️.ts";
import type { Legend } from "../../🔗️contracts/🟦️.ts";
import type { Canvas, Ctx } from "../🧱️canvas/🟦️.ts";
import { mix, wrap } from "../🖍️palette/🟦️.ts";
import { commandsOf } from "../📐️geometry/🟦️.ts";
import * as portable from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

const BAR_STEPS = 24;
/** 📐️ The shortest paper span between the extension lines that still holds the value between them; shorter spans lift the value above the upper extension line on a leader. */
export const DIMENSION_FIT_MM = 3;
const DIMENSION_LEADER_MM = 3.2;
/** 📤️ The clear paper distance between the drawn content and the start of the extension lines (ISO 129-1: a visible gap to the object). */
export const DIMENSION_GAP_MM = 1.5;
/** 🦴️ The paper distance from the right edge of the drawn content to the dimension line, so line, ticks and leader stand clear of the drawing at every scale. */
export const DIMENSION_OFFSET_MM = 6;
const MIN_TICK_GAP_MM = 6;
const LEGEND_BAR_MM = 50;
const LEGEND_WRAP = 100;
const SCALE_BAR_METRES: Readonly<Record<number, number>> = { 500: 5, 200: 2, 100: 1, 50: 1 };

/** 📏️ The scale-bar length in metres: the preset table entry, else the 1-2-5 length nearest to 12 mm of paper. */
export function scaleBarMetres(denominator: number): number {
  const preset = SCALE_BAR_METRES[denominator];
  if (preset !== undefined) return preset;
  const k = 1000 / denominator;
  const candidates = [-3, -2, -1, 0, 1, 2, 3].flatMap((e) => [1, 2, 5].map((m) => m * 10 ** e));
  return candidates.reduce((best, m) => (Math.abs(portable.log((m * k) / 12)) < Math.abs(portable.log((best * k) / 12)) ? m : best));
}

function segmentsFor(metres: number): number {
  const mantissa = metres / 10 ** Math.floor(portable.log10(metres));
  return Math.abs(mantissa - 2) < 1e-9 ? 4 : 5;
}

/** 🔭️ The extent in metres a dimension measures, y up. */
export type DimensionExtent = { readonly minX: number; readonly maxX: number; readonly minY: number; readonly maxY: number };

const DIMENSION_QUANTITIES = {
  elevation: { quantity: "height", label: { en: "Overall height", de: "Gesamthöhe" } },
  front: { quantity: "height", label: { en: "Overall height", de: "Gesamthöhe" } },
  plan: { quantity: "depth", label: { en: "Overall depth", de: "Gesamttiefe" } },
  axonometric: { quantity: "height", label: { en: "Overall height", de: "Gesamthöhe" } },
} as const;

/** 🪜️ The vertical overall dimension right of the content: extension lines, oblique ticks, the value in metres (between the extension lines, or on a leader above them when the span is shorter than {@link DIMENSION_FIT_MM}). It measures the drawn vertical extent of the viewpoint (height in elevations, depth in the plan; in the axonometric view the true height along the vertical axis, which is true to scale). */
export function dimensionLine(canvas: Canvas, ctx: Ctx, extent: DimensionExtent, contentMaxX: number = extent.maxX * ctx.k): void {
  const { k, language, colours, options } = ctx;
  const box = canvas.box;
  if (box === null) return;
  const { quantity, label } = DIMENSION_QUANTITIES[options.viewpoint];
  const w = ANNOTATION_PEN_MM;
  const stroke = { color: colours.ink, widthMm: w, cap: "butt" as const };
  const right = Math.max(box.maxX, contentMaxX, extent.maxX * k);
  const x = right + DIMENSION_OFFSET_MM;
  const top = -extent.maxY * k;
  const bottom = -extent.minY * k;
  const edge = Math.max(contentMaxX, extent.maxX * k) + DIMENSION_GAP_MM;
  canvas.path("annotations", "dimension:extension-top", commandsOf([{ x: edge, y: top }, { x: x + 1, y: top }], false), { stroke });
  canvas.path("annotations", "dimension:extension-bottom", commandsOf([{ x: edge, y: bottom }, { x: x + 1, y: bottom }], false), { stroke });
  canvas.path("annotations", "dimension:line", commandsOf([{ x, y: top }, { x, y: bottom }], false), {
    stroke: { ...stroke, widthMm: w * 1.4 },
    value: { quantity, value: extent.maxY - extent.minY, unit: "m", label },
  });
  for (const [id, y] of [["top", top], ["bottom", bottom]] as const) canvas.path("annotations", `dimension:tick-${id}`, commandsOf([{ x: x - 0.9, y: y + 0.9 }, { x: x + 0.9, y: y - 0.9 }], false), { stroke: { ...stroke, widthMm: w * 2 } });
  const length = extent.maxY - extent.minY;
  const value = `${formatNumber(length, length < 10 ? 2 : 1, language)} m`;
  if (bottom - top >= DIMENSION_FIT_MM) {
    canvas.text("annotations", "dimension:text", { x: x + 1.6, y: (top + bottom) / 2 + ANNOTATION_TEXT_MM * 0.35 }, value, ANNOTATION_TEXT_MM, colours.ink);
    return;
  }
  const lead = top - DIMENSION_LEADER_MM;
  canvas.path("annotations", "dimension:leader", commandsOf([{ x, y: top }, { x, y: lead }], false), { stroke });
  canvas.text("annotations", "dimension:text", { x: x + 1.6, y: lead + ANNOTATION_TEXT_MM * 0.35 }, value, ANNOTATION_TEXT_MM, colours.ink);
}

/** 🔄️ The alternating-block scale bar with `0` and length labels; returns the height it used. */
export function scaleBar(canvas: Canvas, ctx: Ctx, x: number, y: number): number {
  const { k, language, colours, options } = ctx;
  const metres = scaleBarMetres(options.scale.denominator);
  const n = segmentsFor(metres);
  const step = (metres * k) / n;
  const w = ANNOTATION_PEN_MM;
  const height = 1.2;
  for (let i = 0; i < n; i++) {
    const pts = [{ x: x + i * step, y }, { x: x + (i + 1) * step, y }, { x: x + (i + 1) * step, y: y + height }, { x: x + i * step, y: y + height }];
    canvas.path("annotations", `scale-bar:block-${i}`, commandsOf(pts, true), { stroke: { color: colours.ink, widthMm: w, join: "miter" }, fill: { color: i % 2 === 0 ? colours.ink : colours.background } });
  }
  const decimals = metres < 1 ? 1 : 0;
  canvas.text("annotations", "scale-bar:zero", { x, y: y + height + 2.2 }, "0", LABEL_TEXT_MM, colours.ink, { anchor: "middle" });
  canvas.text("annotations", "scale-bar:end", { x: x + metres * k, y: y + height + 2.2 }, `${formatNumber(metres, decimals, language)} m`, LABEL_TEXT_MM, colours.ink, { anchor: "middle" });
  return height + 3.4;
}

/** 🪧️ The title line (asset, view) and the scale / query line; returns the height used. */
export function titleBlock(canvas: Canvas, ctx: Ctx, x: number, y: number, viewTitle: string): number {
  const { state, language, colours, options } = ctx;
  const q = state.query;
  const when = q.kind === "pose" ? q.pose : q.kind === "clip" ? `${q.clip} @ ${formatNumber(q.timeS, 2, language)} s` : `${q.from} → ${q.to} @ ${formatNumber(q.timeS, 2, language)} s`;
  canvas.text("annotations", "title:asset", { x, y: y + ANNOTATION_TEXT_MM }, state.asset.title[language], ANNOTATION_TEXT_MM, colours.ink, { weight: "bold" });
  canvas.text("annotations", "title:scale", { x, y: y + ANNOTATION_TEXT_MM + 3 }, `1:${options.scale.denominator} · ${viewTitle} · ${when}`, LABEL_TEXT_MM, colours.ink);
  return ANNOTATION_TEXT_MM + 4.6;
}

/** 🌈️ The colour of a value on a legend: piecewise linear between the normalised stops, clamped at both ends. */
export function legendColour(legend: Legend, value: number): string {
  const stops = legend.stops;
  if (stops.length === 0) return "#d9d9d9";
  const x = legend.max === legend.min ? 0 : (value - legend.min) / (legend.max - legend.min);
  if (x <= stops[0]!.at) return stops[0]!.color;
  for (let i = 1; i < stops.length; i++) {
    const a = stops[i - 1]!;
    const b = stops[i]!;
    if (x <= b.at) return b.at === a.at ? b.color : mix(a.color, b.color, (x - a.at) / (b.at - a.at));
  }
  return stops[stops.length - 1]!.color;
}

/** 🧾️ A legend block: title with unit, colour bar with min / centre / max ticks, sign convention and interpretation; returns the height used. */
export function legendBlock(canvas: Canvas, ctx: Ctx, legend: Legend, id: string, x: number, y: number): number {
  const { language, colours, options } = ctx;
  const w = ANNOTATION_PEN_MM;
  canvas.text("legend", `${id}:title`, { x, y: y + ANNOTATION_TEXT_MM }, `${legend.title[language]} [${legend.unit}]`, ANNOTATION_TEXT_MM, colours.ink, { weight: "bold" });
  const barY = y + ANNOTATION_TEXT_MM + 1.6;
  const barH = 2.4;
  const span = legend.max - legend.min || 1;
  const stepW = LEGEND_BAR_MM / BAR_STEPS;
  for (let i = 0; i < BAR_STEPS; i++) {
    const value = legend.min + (span * (i + 0.5)) / BAR_STEPS;
    const pts = [{ x: x + i * stepW, y: barY }, { x: x + (i + 1) * stepW, y: barY }, { x: x + (i + 1) * stepW, y: barY + barH }, { x: x + i * stepW, y: barY + barH }];
    canvas.path("legend", `${id}:bar-${i}`, commandsOf(pts, true), { fill: { color: legendColour(legend, value) }, value: { quantity: legend.quantity, value, unit: legend.unit, label: legend.title } });
  }
  const outline = [{ x, y: barY }, { x: x + LEGEND_BAR_MM, y: barY }, { x: x + LEGEND_BAR_MM, y: barY + barH }, { x, y: barY + barH }];
  canvas.path("legend", `${id}:frame`, commandsOf(outline, true), { stroke: { color: colours.ink, widthMm: w, join: "miter" } });
  const decimals = decimalsFor(Math.max(Math.abs(legend.min), Math.abs(legend.max), 1e-9), 3);
  const ticks = [legend.min, ...(legend.min < 0 && legend.max > 0 && ((0 - legend.min) / span) * LEGEND_BAR_MM >= MIN_TICK_GAP_MM && (legend.max / span) * LEGEND_BAR_MM >= MIN_TICK_GAP_MM ? [0] : []), legend.max];
  ticks.forEach((value, i) => {
    const tx = x + ((value - legend.min) / span) * LEGEND_BAR_MM;
    canvas.path("legend", `${id}:tick-${i}`, commandsOf([{ x: tx, y: barY + barH }, { x: tx, y: barY + barH + 0.6 }], false), { stroke: { color: colours.ink, widthMm: w } });
    canvas.text("legend", `${id}:tick-label-${i}`, { x: tx + (i === 0 ? 0.4 : i === ticks.length - 1 ? -0.4 : 0), y: barY + barH + 2.6 }, formatNumber(value, Math.min(decimals, 4), language), LABEL_TEXT_MM, colours.ink, { anchor: i === 0 ? "start" : i === ticks.length - 1 ? "end" : "middle" });
  });
  let cursor = barY + barH + 2.6 + 1.2;
  const de = language === "de";
  const lines = [
    ...wrap(legend.convention[language], LEGEND_WRAP),
    ...wrap(legend.interpretation[language], LEGEND_WRAP),
    ...(legend.conditions === undefined ? [] : wrap(`${de ? "Bedingungen" : "Conditions"}: ${legend.conditions[language]}`, LEGEND_WRAP)),
    ...(legend.assumptions ?? []).flatMap((assumption) => wrap(`${de ? "Annahme" : "Assumption"}: ${assumption[language]}`, LEGEND_WRAP)),
  ];
  lines.forEach((line, i) => {
    cursor += 2.6;
    canvas.text("legend", `${id}:line-${i}`, { x, y: cursor }, line, LABEL_TEXT_MM, colours.ink);
  });
  return cursor + 3 - y;
}
