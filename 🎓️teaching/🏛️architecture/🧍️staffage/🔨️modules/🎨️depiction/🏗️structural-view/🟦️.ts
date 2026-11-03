/** 🏗️ The force and moment views: members coloured by the chosen internal quantity, force and reaction arrows, support markers, centre of mass,
 * free-body inset, joint-moment symbols and member diagrams offset perpendicular to the member, in side elevation, front elevation and plan.
 *
 * Every number comes from one three-dimensional {@link StructuralResult}; a view only projects it ({@link ../../🏗️structure/📽️projection/🟦️.ts}). A force component along the line of sight is drawn as a dot (towards the viewer)
 * or a cross (away) with its value in the label; a moment vector in the view plane is a double-headed arrow, a moment about the line of sight a curl (counter-clockwise when positive). Labels carry the invariant total magnitude.
 *
 * Arrows show the force acting ON the body at its point of application with the head at that point (loads press onto the body, reactions push
 * up into it). Axial N > 0 tension (cool, dotted), N < 0 compression (warm, hatched); M > 0 sagging, drawn on the tension side.
 * Colour is never the only code: arrow and moment kinds differ by outline dash, supports by shape.
 *
 * @see ../../🏗️structure/🟦️.ts — `solveStructure`, `structuralLegend`, `freeBody`
 * @see ../../🏗️structure/📽️projection/🟦️.ts — the projection of forces and moments onto the three planes
 * @see ../../🔗️contracts/🟦️.ts — {@link StructuralResult}
 */
import type { Slug, Text, Viewpoint } from "../../../🧬️schema/🟦️.ts";
import type { DiagramStation, EvaluatedState, Fill, Legend, MemberDiagram, Reaction, StructuralResult, Vec3, WorldForce } from "../../🔗️contracts/🟦️.ts";
import { formatNumber, formatQuantity, LABEL_TEXT_MM, ANNOTATION_TEXT_MM, ANNOTATION_PEN_MM } from "../../📐️units/🟦️.ts";
import { bendingOf, freeBody, shearOf, structuralLegend, type StructuralQuantity } from "../../🏗️structure/🟦️.ts";
import { magnitude, planeOf, projectForce, projectMoment, tensionSide, towardViewer } from "../../🏗️structure/📽️projection/🟦️.ts";
import { type Canvas, type Ctx, Labels, labelBudget } from "../🧱️canvas/🟦️.ts";
import { overlayColours, type Scale, divergingColour, legendOf, mix, toneColour, tx, type Theme } from "../🖍️palette/🟦️.ts";
import type { AnalyticView, Prepared, Style } from "../🧩️shapes/🟦️.ts";
import { type Vec, add, arcCommands, arrowOutline, circleCommands, commandsOf, headOutline, lerp, perp, ringsCommands, scale as scaleVec, sectorCommands, sub, unit } from "../📐️geometry/🟦️.ts";

const PATTERN_FRACTION = 0.15;
const DIAGRAM_MM = 6;
const ARROW_MAX_MM = 14;
const ARROW_MIN_MM = 1.5;
const INSET_HEIGHT_MM = 30;
const INSET_WIDTH_MM = 60;
const INSET_GAP_MM = 16;
const MOMENT_MIN_FRACTION = 0.02;
const OUT_OF_PLANE_MIN = 1e-3;

type ArrowKind = "applied" | "reaction" | "weight" | "inertial";
type MomentKind = "joint" | "reaction" | "applied";
/** 📈️ A member diagram kind. */
export type DiagramKind = "axial" | "shear" | "moment";
type DiagramRun = { base: Vec[]; off: Vec[]; peak: number; peakAt: Vec };

/** 🌐️ The texts the force and moment views draw themselves, in both languages. */
const TEXT = { freeBody: tx("Free-body diagram", "Freikörperbild"), residual: tx("Residual", "Restkräfte") } as const;

const arrowColour = (theme: Theme, kind: ArrowKind): string => {
  const o = overlayColours(theme);
  return kind === "applied" ? o.applied : kind === "reaction" ? o.reaction : kind === "weight" ? o.weight : o.joint;
};
const ARROW_DASH: Readonly<Record<ArrowKind, readonly number[] | undefined>> = { applied: undefined, reaction: [1.6, 0.7], weight: [0.5, 0.5], inertial: [0.15, 0.45] };
const MOMENT_DASH: Readonly<Record<MomentKind, readonly number[] | undefined>> = { joint: undefined, reaction: [1.6, 0.7], applied: [0.15, 0.45] };
const MOMENT_COLOUR = (theme: Theme, kind: MomentKind): string => {
  const o = overlayColours(theme);
  return kind === "joint" ? o.joint : kind === "reaction" ? o.reaction : o.applied;
};
const DIAGRAM_META: Readonly<Record<DiagramKind, { readonly unit: "N" | "N·m"; readonly title: Text; readonly side: 1 | -1 }>> = {
  axial: { unit: "N", title: tx("Axial force diagram N", "Normalkraftlinie N"), side: 1 },
  shear: { unit: "N", title: tx("Shear force diagram V", "Querkraftlinie V"), side: 1 },
  moment: { unit: "N·m", title: tx("Bending moment diagram M", "Momentenlinie M"), side: -1 },
};

const at3 = (p: { readonly x: number; readonly y: number }, z: number): Vec3 => ({ x: p.x, y: p.y, z });
const stationAt = (s: DiagramStation): Vec3 => at3(s.at, s.z);
const placeOf = (ctx: Ctx, p: Vec3): Vec => ctx.paper(planeOf(ctx.options.viewpoint, p));
const outLabel = (symbol: "dot" | "cross" | null): string => (symbol === "dot" ? "⊙" : "⊗");

function memberValue(diagram: MemberDiagram | undefined, quantity: StructuralQuantity): number | null {
  if (!diagram) return null;
  const values = diagram.stations.flatMap((s) => (quantity === "axial" ? [s.axialN] : quantity === "shear" ? [shearOf(s)] : quantity === "moment" ? [bendingOf(s)] : s.stressMinPa === null || s.stressMaxPa === null ? [] : [s.stressMinPa, s.stressMaxPa]));
  return values.length === 0 ? null : values.reduce((best, v) => (Math.abs(v) > Math.abs(best) ? v : best), 0);
}

function symmetricIfEmpty(min: number, max: number): Scale {
  return min === 0 && max === 0 ? { min: -1, centre: 0, max: 1 } : { min, centre: 0, max };
}

const MIN_BONE_HEIGHT_FRACTION = 0.05;

/** 🦴️ Which bones are members worth colouring: those with a section, or at least 5 % of the asset height long; foliage masses and other short bones stay neutral. */
export function memberFilter(state: EvaluatedState): (bone: Slug) => boolean {
  const height = Math.max(1e-6, state.bounds.maxY - state.bounds.minY);
  const eligible = new Set(state.asset.rig.bones.filter((b) => b.section !== undefined || b.lengthM >= MIN_BONE_HEIGHT_FRACTION * height).map((b) => b.id));
  return (bone) => eligible.has(bone);
}

function memberView(ctx: Ctx, shapes: readonly Prepared[], result: StructuralResult, quantity: StructuralQuantity, patterns: boolean): { readonly styleOf: (p: Prepared) => Style; readonly legend: Legend } {
  const eligible = memberFilter(ctx.state);
  const base = structuralLegend(quantity, result, ctx.options.range, eligible);
  const scale = symmetricIfEmpty(base.min, base.max);
  const boneTitle = (id: Slug): Text => ctx.state.asset.rig.bones.find((b) => b.id === id)?.title ?? tx(id, id);
  const regionKind = (id: Slug): string | undefined => ctx.state.asset.thermal.regions.find((r) => r.id === id)?.kind;
  const fmt = (v: number, l: "en" | "de"): string => formatNumber(v, Math.max(0, 2 - Math.floor(Math.log10(Math.max(Math.abs(v), 1e-9)))), l);
  const interpretation = tx(
    `${base.title.en} from ${fmt(scale.min, "en")} to ${fmt(scale.max, "en")} ${base.unit}; zero is grey; bones without a section and shorter than 5 % of the object height, foliage, glazing and texture are drawn neutral (ghosted). With patterns on, strong values (above 15 % of their side of the range) are also hatched: diagonal hatch = negative (warm), dots = positive (cool).`,
    `${base.title.de} von ${fmt(scale.min, "de")} bis ${fmt(scale.max, "de")} ${base.unit}; null ist grau; Stäbe ohne Querschnitt und kürzer als 5 % der Objekthöhe sowie Laub, Verglasung und Textur sind neutral (blass) gezeichnet. Mit Mustern sind starke Werte (über 15 % ihrer Seite des Bereichs) zusätzlich schraffiert: Schrägschraffur = negativ (warm), Punkte = positiv (kühl).`,
  );
  const legend: Legend = legendOf({ title: base.title, quantity, unit: base.unit, convention: base.convention, interpretation, scale, colourAt: (v) => divergingColour(v, scale, false, ctx.theme) });
  const styleOf = (p: Prepared): Style => {
    const glaze = p.shape.role === "glazing";
    const outlineMm = p.weightMm * 0.5;
    const v = quantity === "contact-pressure" ? null : memberValue(result.members[p.shape.bone], quantity);
    const ghost = glaze || p.shape.role === "texture" || (p.shape.region !== undefined && regionKind(p.shape.region) === "foliage");
    const neutral = { color: toneColour(ctx.theme, p.shape.tone), ...(ghost ? { opacity: 0.4 } : {}) };
    if (v === null) return { fill: neutral, outlineMm };
    const value = { quantity, value: v, unit: base.unit, label: boneTitle(p.shape.bone) };
    if (ghost || !eligible(p.shape.bone)) return { fill: neutral, value, outlineMm };
    const span = v < 0 ? -scale.min : scale.max;
    const strong = span > 0 && Math.abs(v) / span > PATTERN_FRACTION;
    const pattern: Fill["pattern"] | undefined = patterns && strong && quantity !== "contact-pressure" ? (v < 0 ? "compression" : "tension") : undefined;
    return { fill: { color: divergingColour(v, scale, false, ctx.theme), ...(glaze ? { opacity: 0.6 } : {}), ...(pattern ? { pattern } : {}) }, value, outlineMm };
  };
  return { styleOf, legend };
}

type Frame = { readonly origin: Vec; readonly up: Vec; readonly tangent: Vec };

function frameOf(ctx: Ctx, r: Reaction): Frame {
  const anchor = ctx.state.anchors[r.anchor]?.anchor;
  const theta = ((anchor?.normalDeg ?? 90) * Math.PI) / 180;
  const up = { x: Math.cos(theta), y: -Math.sin(theta) };
  return { origin: placeOf(ctx, at3(r.at, r.z)), up, tangent: perp(up) };
}

const local = (f: Frame, a: number, b: number): Vec => add(f.origin, add(scaleVec(f.tangent, a), scaleVec(f.up, b)));

function hullOf(points: readonly Vec[]): Vec[] {
  const sorted = [...points].sort((a, b) => a.x - b.x || a.y - b.y);
  if (sorted.length < 3) return sorted;
  const turn = (o: Vec, a: Vec, b: Vec): number => (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
  const build = (input: readonly Vec[]): Vec[] => {
    const out: Vec[] = [];
    for (const p of input) {
      while (out.length >= 2 && turn(out[out.length - 2]!, out[out.length - 1]!, p) <= 0) out.pop();
      out.push(p);
    }
    out.pop();
    return out;
  };
  return [...build(sorted), ...build([...sorted].reverse())];
}

function drawSupportsSide(canvas: Canvas, ctx: Ctx, r: Reaction, pressure: { readonly colour: (v: number) => string } | null): void {
  const { colours, options } = ctx;
  const anchor = ctx.state.anchors[r.anchor]!.anchor;
  const w = ANNOTATION_PEN_MM;
  const stroke = { color: colours.ink, widthMm: w * 1.6, join: "miter" as const };
  const f = frameOf(ctx, r);
  const type = anchor.support ?? "contact";
  const id = `support:${r.anchor}`;
  const opacity = r.active ? 1 : 0.35;
  const dim = { ...stroke, opacity };
  const ticks = (b: number, xs: readonly number[]): void => xs.forEach((x, i) => canvas.path("forces", `${id}:hatch-${i}`, commandsOf([local(f, x, b), local(f, x - 0.7, b - 0.8)], false), { stroke: { ...dim, widthMm: w } }));
  if (type === "pin" || type === "roller") {
    const depth = type === "pin" ? 2.3 : 1.7;
    canvas.path("forces", id, commandsOf([local(f, 0, 0), local(f, -1.3, -depth), local(f, 1.3, -depth)], true), { stroke: dim, fill: { color: colours.background, opacity } });
    if (type === "roller") for (const x of [-0.65, 0.65]) canvas.path("forces", `${id}:wheel`, circleCommands(local(f, x, -depth - 0.5), 0.5), { stroke: { ...dim, widthMm: w }, fill: { color: colours.background, opacity } });
    const base = type === "pin" ? -depth : -depth - 1;
    canvas.path("forces", `${id}:base`, commandsOf([local(f, -1.9, base), local(f, 1.9, base)], false), { stroke: dim });
    ticks(base, [-1.4, 0, 1.4]);
  } else if (type === "fixed") {
    canvas.path("forces", id, commandsOf([local(f, -2.2, 0), local(f, 2.2, 0)], false), { stroke: { ...dim, widthMm: w * 2.4 } });
    ticks(0, [-1.8, -0.9, 0, 0.9, 1.8, 2.7].map((x) => x - 0.4));
  } else {
    const half = Math.max(0.5, ((anchor.footprintLengthM ?? 0.1) * ctx.k) / 2);
    const p = r.pressurePa;
    const color = pressure !== null && p !== null ? pressure.colour(p) : mix(colours.background, colours.ink, 0.35);
    canvas.path("forces", id, commandsOf([local(f, -half, 0), local(f, half, 0), local(f, half, -0.7), local(f, -half, -0.7)], true), {
      stroke: { ...dim, widthMm: w },
      fill: { color, opacity: r.active ? 1 : 0.35, ...(pressure !== null && options.patterns && p !== null && p > 0 ? { pattern: "compression" as const } : {}) },
      ...(p === null ? {} : { value: { quantity: "contact-pressure", value: p, unit: "Pa", label: anchor.title } }),
    });
  }
}

function drawSupportsProjected(canvas: Canvas, ctx: Ctx, r: Reaction, pressure: { readonly colour: (v: number) => string } | null): void {
  const { colours, options } = ctx;
  const anchor = ctx.state.anchors[r.anchor]!.anchor;
  const w = ANNOTATION_PEN_MM;
  const type = anchor.support ?? "contact";
  const id = `support:${r.anchor}`;
  const opacity = r.active ? 1 : 0.35;
  const dim = { color: colours.ink, widthMm: w * 1.6, join: "miter" as const, opacity };
  const o = placeOf(ctx, at3(r.at, r.z));
  const bg = { color: colours.background, opacity };
  if (type === "pin" || type === "roller") {
    canvas.path("forces", id, circleCommands(o, 1.4), { stroke: dim, fill: bg });
    if (type === "pin") for (const s of [1, -1]) canvas.path("forces", `${id}:cross-${s > 0 ? "a" : "b"}`, commandsOf([add(o, { x: -1, y: -s }), add(o, { x: 1, y: s })], false), { stroke: { ...dim, widthMm: w } });
    else canvas.path("forces", `${id}:slot`, commandsOf([add(o, { x: -1.4, y: 0 }), add(o, { x: 1.4, y: 0 })], false), { stroke: { ...dim, widthMm: w } });
  } else if (type === "fixed") {
    const h = 1.8;
    canvas.path("forces", id, commandsOf([add(o, { x: -h, y: -h }), add(o, { x: h, y: -h }), add(o, { x: h, y: h }), add(o, { x: -h, y: h })], true), { stroke: { ...dim, widthMm: w * 2.4 }, fill: bg });
    for (const k of [-1.2, 0, 1.2]) canvas.path("forces", `${id}:hatch-${k}`, commandsOf([add(o, { x: -h, y: k + h * 0.4 }), add(o, { x: h * 0.4 - 0.2, y: -h })], false), { stroke: { ...dim, widthMm: w } });
  } else {
    const theta = ((anchor.normalDeg ?? 90) * Math.PI) / 180;
    const tilt = ((anchor.tiltDeg ?? 0) * Math.PI) / 180;
    const n: Vec3 = { x: Math.cos(theta) * Math.cos(tilt), y: Math.sin(theta) * Math.cos(tilt), z: Math.sin(tilt) };
    const axis: Vec3 = Math.abs(n.z) < 0.9 ? { x: 0, y: 0, z: 1 } : { x: 1, y: 0, z: 0 };
    const raw: Vec3 = { x: axis.y * n.z - axis.z * n.y, y: axis.z * n.x - axis.x * n.z, z: axis.x * n.y - axis.y * n.x };
    const rawLength = Math.hypot(raw.x, raw.y, raw.z);
    const t: Vec3 = { x: raw.x / rawLength, y: raw.y / rawLength, z: raw.z / rawLength };
    const t2: Vec3 = { x: n.y * t.z - n.z * t.y, y: n.z * t.x - n.x * t.z, z: n.x * t.y - n.y * t.x };
    const length = anchor.footprintLengthM ?? 0.1;
    const width = anchor.footprintWidthM ?? length * 0.5;
    const thick = 0.7 / ctx.k;
    const corners = [-1, 1].flatMap((a) => [-1, 1].flatMap((b) => [0, 1].map((c) => placeOf(ctx, { x: r.at.x + (t.x * a * length + t2.x * b * width) / 2 - n.x * c * thick, y: r.at.y + (t.y * a * length + t2.y * b * width) / 2 - n.y * c * thick, z: r.z + (t.z * a * length + t2.z * b * width) / 2 - n.z * c * thick }))));
    const p = r.pressurePa;
    const color = pressure !== null && p !== null ? pressure.colour(p) : mix(colours.background, colours.ink, 0.35);
    canvas.path("forces", id, commandsOf(hullOf(corners), true), {
      stroke: { ...dim, widthMm: w },
      fill: { color, opacity: r.active ? 1 : 0.35, ...(pressure !== null && options.patterns && p !== null && p > 0 ? { pattern: "compression" as const } : {}) },
      ...(p === null ? {} : { value: { quantity: "contact-pressure", value: p, unit: "Pa", label: anchor.title } }),
    });
  }
}

function drawSupports(canvas: Canvas, ctx: Ctx, result: StructuralResult, pressure: { readonly colour: (v: number) => string } | null): void {
  for (const r of result.reactions) {
    if (!ctx.state.anchors[r.anchor]) continue;
    if (ctx.options.viewpoint === "elevation" && Math.abs(ctx.state.anchors[r.anchor]!.anchor.tiltDeg ?? 0) < 45) drawSupportsSide(canvas, ctx, r, pressure);
    else drawSupportsProjected(canvas, ctx, r, pressure);
  }
}

function arrowLengthMm(mag: number, max: number, longest: number): number {
  return max <= 0 ? 0 : Math.max(ARROW_MIN_MM, (mag / max) * longest);
}

type ArrowSpec = { readonly id: string; readonly kind: ArrowKind; readonly head: Vec; readonly force: Vec3; readonly title: Text; readonly longest: number; readonly max: number; readonly drawn: boolean; readonly label: boolean; readonly required: boolean };

function crossSymbol(canvas: Canvas, id: string, centre: Vec, radius: number, symbol: "dot" | "cross", color: string, dash: readonly number[] | undefined, value: { readonly quantity: string; readonly value: number; readonly unit: string; readonly label: Text }): void {
  canvas.path("forces", id, circleCommands(centre, radius), { stroke: { color, widthMm: 0.3, ...(dash ? { dashMm: dash } : {}) }, fill: { color, opacity: 0.12 }, value });
  if (symbol === "dot") canvas.path("forces", `${id}:dot`, circleCommands(centre, Math.max(0.3, radius * 0.35)), { fill: { color } });
  else for (const s of [1, -1]) canvas.path("forces", `${id}:cross-${s > 0 ? "a" : "b"}`, commandsOf([add(centre, { x: -radius * 0.7, y: -radius * 0.7 * s }), add(centre, { x: radius * 0.7, y: radius * 0.7 * s })], false), { stroke: { color, widthMm: 0.3, cap: "round" } });
}

function drawArrow(canvas: Canvas, ctx: Ctx, spec: ArrowSpec, labels: Labels): void {
  const projection = projectForce(ctx.options.viewpoint, spec.force);
  if (projection.total < 1e-9) return;
  const inPlane = Math.hypot(projection.arrow.x, projection.arrow.y);
  const color = arrowColour(ctx.theme, spec.kind);
  const dash = ARROW_DASH[spec.kind];
  const value = { quantity: "force", value: projection.total, unit: "N", label: spec.title };
  let tail = spec.head;
  if (inPlane > OUT_OF_PLANE_MIN * projection.total) {
    const dir = { x: projection.arrow.x / inPlane, y: -projection.arrow.y / inPlane };
    const length = arrowLengthMm(inPlane, spec.max, spec.longest);
    tail = sub(spec.head, scaleVec(dir, length));
    if (spec.drawn) canvas.path("forces", spec.id, commandsOf(arrowOutline(tail, spec.head, 0.3, Math.min(1.8, length * 0.6), 1.3), true), { fill: { color }, stroke: { color, widthMm: 0.12, ...(dash ? { dashMm: dash } : {}) }, value });
  }
  if (spec.drawn && projection.symbol !== null) crossSymbol(canvas, `${spec.id}:out`, spec.head, 0.9 + 1.6 * Math.sqrt(Math.abs(projection.outOfPlane) / Math.max(spec.max, 1e-9)), projection.symbol, color, dash, { ...value, value: Math.abs(projection.outOfPlane) });
  if (spec.label) {
    const extra = projection.symbol === null ? "" : ` (${outLabel(projection.symbol)} ${formatQuantity(Math.abs(projection.outOfPlane), "N", ctx.language)})`;
    labels.add(`${spec.id}:label`, spec.drawn ? tail : spec.head, `${spec.title[ctx.language]} ${formatQuantity(projection.total, "N", ctx.language)}${extra}`, { required: spec.required, magnitude: projection.total });
  }
}

/** 🏹️ One arrow request: `drawn` arrows are painted, `labelled` ones get a value label, `required` labels survive the label budget. The force and its point are world vectors; the view projects them. */
export type ForceItem = { readonly id: string; readonly kind: ArrowKind; readonly at: Vec3; readonly force: Vec3; readonly title: Text; readonly drawn: boolean; readonly labelled: boolean; readonly required: boolean };

const loadGroup = (id: string): string | null => (/#\d+$/u.test(id) ? id.replace(/#\d+$/u, "") : null);
const mean = (list: readonly Vec3[]): Vec3 => ({ x: list.reduce((s, p) => s + p.x, 0) / list.length, y: list.reduce((s, p) => s + p.y, 0) / list.length, z: list.reduce((s, p) => s + p.z, 0) / list.length });
const sumOf = (list: readonly Vec3[]): Vec3 => ({ x: list.reduce((s, p) => s + p.x, 0), y: list.reduce((s, p) => s + p.y, 0), z: list.reduce((s, p) => s + p.z, 0) });
const forceVec = (f: { readonly fxN: number; readonly fyN: number; readonly fzN: number }): Vec3 => ({ x: f.fxN, y: f.fyN, z: f.fzN });

/** ➕️ Sub-forces of one distributed load (`<load>#<n>`) stay as small unlabelled arrows plus one undrawn resultant carrying the single label. */
export function withResultants(items: readonly ForceItem[]): ForceItem[] {
  const groups = new Map<string, ForceItem[]>();
  for (const item of items) {
    const g = loadGroup(item.id);
    if (g !== null) groups.set(g, [...(groups.get(g) ?? []), item]);
  }
  const out: ForceItem[] = items.map((i) => (loadGroup(i.id) === null ? i : { ...i, labelled: false }));
  for (const [g, list] of groups)
    out.push({ id: `${g}:resultant`, kind: list[0]!.kind, at: mean(list.map((f) => f.at)), force: sumOf(list.map((f) => f.force)), title: tx(`${list[0]!.title.en}, distributed`, `${list[0]!.title.de}, verteilt`), drawn: false, labelled: true, required: false });
  return out;
}

const worldPoint = (f: WorldForce): Vec3 => at3(f.at, f.z);

/** 🏹️ The arrows of the force view: applied loads (distributed ones with one resultant), active reactions, total weight and, for inertial states, total inertia, all as world vectors. */
export function forceList(state: EvaluatedState, result: StructuralResult): ForceItem[] {
  const total = (list: readonly WorldForce[]): Vec3 => sumOf(list.map(forceVec));
  const weight = total(result.segmentWeights);
  const inertial = total(result.inertialForces);
  const reactionTitle = (r: Reaction): Text => state.anchors[r.anchor]?.anchor.title ?? tx(r.anchor, r.anchor);
  const plain = { drawn: true, labelled: true, required: false };
  return withResultants([
    ...result.applied.map((f) => ({ id: `force:applied:${f.id}`, kind: "applied" as const, at: worldPoint(f), force: forceVec(f), title: f.title, ...plain })),
    ...result.reactions.filter((r) => r.active).map((r) => ({ id: `force:reaction:${r.anchor}`, kind: "reaction" as const, at: at3(r.at, r.z), force: forceVec(r), title: reactionTitle(r), ...plain, required: true })),
    { id: "force:weight", kind: "weight" as const, at: result.centreOfMass, force: weight, title: tx("Weight", "Gewicht"), ...plain },
    ...(magnitude(inertial) > 1e-9 ? [{ id: "force:inertial", kind: "inertial" as const, at: result.centreOfMass, force: inertial, title: tx("Inertia", "Trägheit"), ...plain }] : []),
  ]);
}

function drawCentreOfMass(canvas: Canvas, ctx: Ctx, at: Vec, mass: number): void {
  const r = 1.3;
  const ink = ctx.colours.ink;
  canvas.path("forces", "com:disc", circleCommands(at, r), { stroke: { color: ink, widthMm: 0.25 }, fill: { color: ctx.colours.background }, value: { quantity: "total-mass", value: mass, unit: "kg", label: tx("Centre of mass", "Massenschwerpunkt") } });
  canvas.path("forces", "com:quadrant-a", sectorCommands(at, r, 0, Math.PI / 2), { fill: { color: ink } });
  canvas.path("forces", "com:quadrant-b", sectorCommands(at, r, Math.PI, (3 * Math.PI) / 2), { fill: { color: ink } });
}

const totalOf = (kind: DiagramKind, s: DiagramStation): number => (kind === "axial" ? Math.abs(s.axialN) : kind === "shear" ? Math.hypot(s.shearN, s.shearZN) : Math.hypot(s.momentNm, s.momentLateralNm));

/** 📈️ The largest invariant magnitude of a diagram kind over all members (axial |N|, shear |V| of both components, bending the resultant of both components), the same in every view. */
export function peakOf(result: StructuralResult, kind: DiagramKind, include: (bone: Slug) => boolean = () => true): number {
  return Math.max(0, ...Object.entries(result.members).flatMap(([id, m]) => (include(id) ? m.stations.map((s) => totalOf(kind, s)) : [])));
}

/** 📈️ One filled region of a member diagram: the baseline followed by the reversed ordinate curve, and its peak (the signed in-plane value). */
export type DiagramPolygon = { readonly bone: Slug; readonly index: number; readonly points: readonly Vec[]; readonly peak: number; readonly peakAt: Vec };

/** 📈️ A member whose diagram has a significant component along the line of sight: the symbol replaces the missing ordinate at the peak station. */
export type DiagramOutOfPlane = { readonly bone: Slug; readonly at: Vec3; readonly total: number; readonly outOfPlane: number; readonly symbol: "dot" | "cross" };

function directionOf(state: EvaluatedState, viewpoint: Viewpoint, bone: Slug): Vec | null {
  const u = planeOf(viewpoint, state.bones[bone]!.frame.u);
  const len = Math.hypot(u.x, u.y);
  return len < 1e-6 ? null : { x: u.x / len, y: u.y / len };
}

/** 📈️ The vector one station draws in a view: the shear vector, the tension-side direction of the bending moment, or the axial force along the member's left normal; model coordinates of the viewpoint. */
function stationVector(state: EvaluatedState, viewpoint: Viewpoint, kind: DiagramKind, bone: Slug, s: DiagramStation, left: Vec): { readonly scalar: number; readonly out: number } {
  if (kind === "axial") return { scalar: s.axialN, out: 0 };
  const vector = kind === "shear" ? s.shearVector : tensionSide(s.momentVector, state.bones[bone]!.frame.u);
  const flat = planeOf(viewpoint, vector);
  const along = flat.x * left.x + flat.y * left.y;
  return { scalar: kind === "moment" ? -along : along, out: towardViewer(viewpoint, vector) };
}

/** 📈️ The sign-split polygons of one diagram kind for every member in one view, ordinates offset perpendicular to the projected bone by `ordinate · value / peak` on the kind's side; `yDown` selects the paper (true) or model (false) handedness. `value` is the in-plane part of the 3D vector, `peak` the invariant maximum of {@link peakOf}. */
export function diagramPolygons(state: EvaluatedState, result: StructuralResult, kind: DiagramKind, viewpoint: Viewpoint, peak: number, map: (p: { readonly x: number; readonly y: number }) => Vec, ordinate: number, yDown: boolean, include: (bone: Slug) => boolean = () => true): DiagramPolygon[] {
  const meta = DIAGRAM_META[kind];
  const out: DiagramPolygon[] = [];
  for (const [bone, member] of Object.entries(result.members)) {
    const direction = state.bones[bone] ? directionOf(state, viewpoint, bone) : null;
    if (direction === null || member.stations.length < 2 || !include(bone)) continue;
    const leftModel = { x: -direction.y, y: direction.x };
    const left = yDown ? { x: leftModel.x, y: -leftModel.y } : leftModel;
    const base = member.stations.map((s) => map(planeOf(viewpoint, stationAt(s))));
    const values = member.stations.map((s) => stationVector(state, viewpoint, kind, bone, s, leftModel).scalar);
    const offset = (i: number, v: number): Vec => add(base[i]!, scaleVec(left, (meta.side * v * ordinate) / peak));
    let run: DiagramRun = { base: [], off: [], peak: 0, peakAt: base[0]! };
    let index = 0;
    const push = (b: Vec, o: Vec, v: number): void => {
      run.base.push(b);
      run.off.push(o);
      if (Math.abs(v) > Math.abs(run.peak)) {
        run.peak = v;
        run.peakAt = o;
      }
    };
    const close = (): void => {
      if (run.base.length > 1 && Math.abs(run.peak) > 1e-9) out.push({ bone, index: index++, points: [...run.base, ...run.off.slice().reverse()], peak: run.peak, peakAt: run.peakAt });
      run = { base: [], off: [], peak: 0, peakAt: base[0]! };
    };
    for (let i = 0; i < base.length; i++) {
      push(base[i]!, offset(i, values[i]!), values[i]!);
      const next = values[i + 1];
      if (next !== undefined && values[i]! * next < 0) {
        const t = values[i]! / (values[i]! - next);
        const crossing = lerp(base[i]!, base[i + 1]!, t);
        push(crossing, crossing, 0);
        close();
        push(crossing, crossing, 0);
      }
    }
    close();
  }
  return out;
}

/** 🎯️ The stations of every member where the vector of a diagram kind points along the line of sight with at least 1 ‰ of the invariant peak: one entry per member, at its largest such station. */
export function diagramOutOfPlane(state: EvaluatedState, result: StructuralResult, kind: DiagramKind, viewpoint: Viewpoint, peak: number, include: (bone: Slug) => boolean = () => true): DiagramOutOfPlane[] {
  const out: DiagramOutOfPlane[] = [];
  if (kind === "axial") return out;
  for (const [bone, member] of Object.entries(result.members)) {
    if (!include(bone) || !state.bones[bone]) continue;
    let best: DiagramOutOfPlane | null = null;
    for (const s of member.stations) {
      const { out: depth } = stationVector(state, viewpoint, kind, bone, s, { x: 0, y: 1 });
      if (Math.abs(depth) > Math.max(OUT_OF_PLANE_MIN * peak, 1e-9) && (best === null || Math.abs(depth) > Math.abs(best.outOfPlane))) best = { bone, at: stationAt(s), total: totalOf(kind, s), outOfPlane: depth, symbol: depth > 0 ? "dot" : "cross" };
    }
    if (best !== null) out.push(best);
  }
  return out;
}

function drawDiagram(canvas: Canvas, ctx: Ctx, result: StructuralResult, kind: DiagramKind, peak: number): void {
  const meta = DIAGRAM_META[kind];
  const sc: Scale = { min: -peak, centre: 0, max: peak };
  const viewpoint = ctx.options.viewpoint;
  const include = memberFilter(ctx.state);
  const titleOf = (bone: Slug): Text => ctx.state.asset.rig.bones.find((b) => b.id === bone)?.title ?? tx(bone, bone);
  for (const r of diagramPolygons(ctx.state, result, kind, viewpoint, peak, ctx.paper, DIAGRAM_MM, true, include)) {
    const color = divergingColour(r.peak, sc, false, ctx.theme);
    const label = titleOf(r.bone);
    const id = `diagram:${kind}:${r.bone}:${r.index}`;
    canvas.path("diagrams", id, commandsOf(r.points, true), {
      fill: { color, opacity: 0.4 },
      stroke: { color, widthMm: 0.2, join: "round" },
      value: { quantity: kind === "moment" ? "bending-moment" : kind === "axial" ? "axial-force" : "shear-force", value: r.peak, unit: meta.unit, label },
      bone: r.bone,
    });
    if (ctx.options.annotations.labels) ctx.labels.add(`${id}:label`, r.peakAt, `${label[ctx.language]} ${formatQuantity(r.peak, meta.unit, ctx.language)}`, { required: false, magnitude: Math.abs(r.peak) });
  }
  for (const o of diagramOutOfPlane(ctx.state, result, kind, viewpoint, peak, include)) {
    const color = divergingColour(o.outOfPlane, sc, false, ctx.theme);
    const label = titleOf(o.bone);
    const centre = placeOf(ctx, o.at);
    crossSymbol(canvas, `diagram:${kind}:${o.bone}:out`, centre, 0.9 + 1.6 * Math.sqrt(Math.abs(o.outOfPlane) / peak), o.symbol, color, undefined, { quantity: kind === "moment" ? "bending-moment" : "shear-force", value: Math.abs(o.outOfPlane), unit: meta.unit, label });
    if (ctx.options.annotations.labels) ctx.labels.add(`diagram:${kind}:${o.bone}:out:label`, centre, `${label[ctx.language]} ${formatQuantity(o.total, meta.unit, ctx.language)} (${outLabel(o.symbol)} ${formatQuantity(Math.abs(o.outOfPlane), meta.unit, ctx.language)})`, { required: false, magnitude: o.total });
  }
}

function diagramLegend(ctx: Ctx, kind: DiagramKind, peak: number): Legend {
  const meta = DIAGRAM_META[kind];
  const sc: Scale = { min: 0, centre: 0, max: peak };
  const side = kind === "moment";
  return legendOf({
    title: meta.title,
    quantity: `${kind}-diagram`,
    unit: meta.unit,
    scale: sc,
    colourAt: (v) => divergingColour(v, { min: -peak, centre: 0, max: peak }, false, ctx.theme),
    convention: side
      ? tx("M > 0 sagging (cool), M < 0 hogging (warm); the ordinate is drawn on the tension side of the member, projected into the view (M > 0 about the lateral axis on the bone's local −y face); at the peak a ringed dot (towards the viewer) or cross (away) marks the component along the line of sight of the tension-side direction of the bending moment (the direction in which the ordinate is drawn).", "M > 0 Feldmoment (kühl), M < 0 Stützmoment (warm); die Ordinate liegt auf der Zugseite des Stabes, in die Ansicht projiziert (M > 0 um die Querachse auf der lokalen −y-Seite des Stabes); am Maximum markiert ein Punkt im Kreis (zum Betrachter) oder Kreuz im Kreis (vom Betrachter weg) die Komponente in Blickrichtung der Zugseitenrichtung des Biegemoments (die Richtung, in der die Ordinate gezeichnet wird).")
      : tx("Positive values (tension for N) are drawn on the bone's local +y side (left of the bone direction), negative values on the other side; the vector is projected into the view, at its peak a ringed dot (towards the viewer) or cross (away) marks the component along the line of sight of the shear force vector V = −Σ distal transverse force (for N there is none).", "Positive Werte (Zug bei N) liegen auf der lokalen +y-Seite des Stabes (links der Stabrichtung), negative Werte auf der anderen Seite; der Vektor wird in die Ansicht projiziert, an ihrem Maximum markiert ein Punkt im Kreis (zum Betrachter) oder Kreuz im Kreis (vom Betrachter weg) die Komponente in Blickrichtung des Querkraftvektors V = −Σ distale Querkraft (bei N gibt es keine)."),
    interpretation: tx(`Ordinate scale: ${DIAGRAM_MM} mm ≙ ${formatNumber(peak, 2, "en")} ${meta.unit} (the invariant maximum of the three-dimensional vector), the same for all members and all views.`, `Ordinatenmaßstab: ${DIAGRAM_MM} mm ≙ ${formatNumber(peak, 2, "de")} ${meta.unit} (das unveränderliche Maximum des räumlichen Vektors), für alle Stäbe und alle Ansichten gleich.`),
  });
}

function arrowLegend(theme: Theme, max: number, longest: number): Legend {
  const sc: Scale = { min: 0, centre: 0, max };
  return legendOf({
    title: tx("Force arrows", "Kraftpfeile"),
    quantity: "force",
    unit: "N",
    scale: sc,
    colourAt: () => arrowColour(theme, "applied"),
    convention: tx(
      "The arrow is the in-plane part of the force acting on the body, its head at the point of application; world axes x ahead, y up, z left. A component along the line of sight is a ringed dot (towards the viewer) or ringed cross (away) with its value in the label; labels give the full magnitude. Red = applied load (solid outline), teal = support reaction (long-dash outline), grey = weight (short-dash outline), orange = inertia (dotted outline); quartered disc = centre of mass.",
      "Der Pfeil ist der in der Ebene liegende Anteil der auf den Körper wirkenden Kraft, die Spitze liegt im Angriffspunkt; Weltachsen x voraus, y oben, z links. Eine Komponente in Blickrichtung erscheint als Punkt im Kreis (zum Betrachter) oder Kreuz im Kreis (vom Betrachter weg) mit ihrem Wert im Label; die Labels nennen den vollen Betrag. Rot = äußere Last (durchgezogene Kontur), Türkis = Auflagerreaktion (langgestrichelte Kontur), Grau = Gewicht (kurzgestrichelte Kontur), Orange = Trägheit (gepunktete Kontur); geviertelte Scheibe = Massenschwerpunkt.",
    ),
    interpretation: tx(`Arrow length is proportional to the in-plane part of |F| (minimum ${ARROW_MIN_MM} mm): ${longest} mm ≙ ${formatNumber(max, 1, "en")} N, the same scale in every view.`, `Die Pfeillänge ist proportional zum ebenen Anteil von |F| (mindestens ${ARROW_MIN_MM} mm): ${longest} mm ≙ ${formatNumber(max, 1, "de")} N, in jeder Ansicht derselbe Maßstab.`),
  });
}

function modelBox(shapes: readonly Prepared[], k: number): { readonly minX: number; readonly minY: number; readonly maxX: number; readonly maxY: number } | null {
  const points = shapes.filter((p) => p.closed && p.shape.filled && p.shape.role !== "glazing" && p.shape.role !== "texture").flatMap((p) => p.rings.flat());
  if (points.length === 0) return null;
  const xs = points.map((p) => p.x / k);
  const ys = points.map((p) => -p.y / k);
  return { minX: Math.min(...xs), maxX: Math.max(...xs), minY: Math.min(...ys), maxY: Math.max(...ys) };
}

/** 🏗️ Builds the forces view: members coloured by the chosen quantity, supports, load and reaction arrows, centre of mass, optional N/V diagrams and free-body inset. */
export function forcesView(ctx: Ctx, shapes: readonly Prepared[], result: StructuralResult): AnalyticView {
  const { options, state } = ctx;
  const quantity = options.forceQuantity;
  const members = memberView(ctx, shapes, result, quantity, options.patterns);
  const forces = forceList(state, result);
  const max = Math.max(0, ...forces.map((f) => magnitude(f.force)));
  const heightMm = (state.bounds.maxY - state.bounds.minY) * ctx.k;
  const longest = Math.min(ARROW_MAX_MM, Math.max(7, heightMm * 0.9));
  const pressureBase = structuralLegend("contact-pressure", result, options.range);
  const pmax = pressureBase.max > 0 ? pressureBase.max : 1;
  const pScale: Scale = { min: -pmax, centre: 0, max: pmax };
  const pressureColour = { colour: (v: number): string => divergingColour(v, pScale, true, ctx.theme) };
  const showPressure = quantity === "contact-pressure";
  const pressureLegend = (): Legend => {
    const base = pressureBase;
    return legendOf({ title: base.title, quantity: "contact-pressure", unit: "Pa", scale: { min: 0, centre: 0, max: base.max }, colourAt: pressureColour.colour, convention: tx("Compression only: normal reaction divided by the footprint area; warm = pressure; with patterns on, every footprint under pressure is also diagonally hatched.", "Nur Druck: Normalreaktion geteilt durch die Aufstandsfläche; warm = Pressung; mit Mustern ist jede belastete Aufstandsfläche zusätzlich schräg schraffiert."), interpretation: base.interpretation });
  };
  const diagrams = (["axial", "shear"] as const).filter((k) => options.diagrams[k]).map((k) => ({ kind: k, peak: peakOf(result, k, memberFilter(state)) })).filter((d) => d.peak > 1e-9);

  const overlay = (canvas: Canvas): void => {
    drawSupports(canvas, ctx, result, showPressure ? pressureColour : null);
    for (const f of forces) drawArrow(canvas, ctx, { id: f.id, kind: f.kind, head: placeOf(ctx, f.at), force: f.force, title: f.title, longest, max, drawn: f.drawn, label: options.annotations.labels && f.labelled, required: f.required }, ctx.labels);
    drawCentreOfMass(canvas, ctx, placeOf(ctx, result.centreOfMass), result.totalMassKg);
    for (const d of diagrams) drawDiagram(canvas, ctx, result, d.kind, d.peak);
  };

  const inset = (canvas: Canvas): number => {
    const box = canvas.box;
    const frame = modelBox(shapes, ctx.k);
    if (box === null || frame === null) return 0;
    const wm = Math.max(1e-6, frame.maxX - frame.minX);
    const hm = Math.max(1e-6, frame.maxY - frame.minY);
    const fi = Math.max(ctx.k, Math.min(INSET_HEIGHT_MM / hm, INSET_WIDTH_MM / wm));
    const x0 = box.maxX + INSET_GAP_MM;
    const y0 = box.minY + 6;
    const viewpoint = options.viewpoint;
    const map = (p: Vec3): Vec => {
      const q = planeOf(viewpoint, p);
      return { x: x0 + (q.x - frame.minX) * fi, y: y0 + (frame.maxY - q.y) * fi };
    };
    canvas.text("diagrams", "freebody:title", { x: x0, y: box.minY + ANNOTATION_TEXT_MM }, TEXT.freeBody[ctx.language], ANNOTATION_TEXT_MM, ctx.colours.ink, { weight: "bold" });
    for (const p of shapes) {
      if (!p.closed || !p.shape.filled || p.shape.role === "glazing" || p.shape.role === "texture") continue;
      const rings = p.rings.map((r) => r.map((q) => ({ x: x0 + (q.x / ctx.k - frame.minX) * fi, y: y0 + (frame.maxY + q.y / ctx.k) * fi })));
      canvas.path("diagrams", `freebody:outline:${p.shape.id}`, ringsCommands(rings, true), { stroke: { color: ctx.colours.ink, widthMm: 0.18, opacity: 0.55, dashMm: [0.8, 0.5] }, fill: { color: mix(ctx.colours.background, ctx.colours.ink, 0.06) } });
    }
    const longestInset = Math.min(10, Math.max(5, hm * fi * 0.35));
    const fb: ForceItem[] = withResultants(freeBody(result).map((f) => ({ id: `freebody:${f.kind}:${f.id}`, kind: f.kind, at: f.at, force: forceVec(f), title: f.title, drawn: true, labelled: f.kind === "applied" || f.kind === "reaction", required: f.kind === "reaction" }))).filter((f) => !f.drawn || loadGroup(f.id) === null).map((f) => (f.drawn ? f : { ...f, drawn: true }));
    const maxInset = Math.max(0, ...fb.map((f) => magnitude(f.force)));
    const insetLabels = new Labels();
    for (const f of fb) drawArrow(canvas, ctx, { id: f.id, kind: f.kind, head: map(f.at), force: f.force, title: f.title, longest: longestInset, max: maxInset, drawn: true, label: options.annotations.labels && f.labelled, required: f.required }, insetLabels);
    drawCentreOfMass(canvas, ctx, map(result.centreOfMass), result.totalMassKg);
    const omitted = insetLabels.flush(canvas, "diagrams", (canvas.box?.maxX ?? x0) + 4, LABEL_TEXT_MM, ctx.colours.ink, 0.1, labelBudget(options.scale.denominator));
    const r = result.residual;
    const weightN = Math.abs(result.segmentWeights.reduce((sum, f) => sum + f.fyN, 0));
    const tiny = (v: number, scaleN: number): number => (Math.abs(v) < 1e-6 * Math.max(1, scaleN) ? 0 : v);
    const lang = ctx.language;
    const labelY = (canvas.box?.maxY ?? y0 + hm * fi) + 4;
    const line = (id: string, y: number, parts: readonly (readonly [string, number, "N" | "N·m"])[]): void => {
      canvas.text("diagrams", id, { x: x0, y }, parts.map(([name, v, unitName]) => `${name} ${formatQuantity(tiny(v, weightN), unitName, lang)}`).join(", "), LABEL_TEXT_MM, ctx.colours.ink);
    };
    canvas.text("diagrams", "freebody:residual", { x: x0, y: labelY }, `${TEXT.residual[lang]}:`, LABEL_TEXT_MM, ctx.colours.ink);
    line("freebody:residual-forces", labelY + LABEL_TEXT_MM * 1.4, [["ΣFx", r.fxN, "N"], ["ΣFy", r.fyN, "N"], ["ΣFz", r.fzN, "N"]]);
    line("freebody:residual-moments", labelY + LABEL_TEXT_MM * 2.8, [["ΣMx", r.mxNm, "N·m"], ["ΣMy", r.myNm, "N·m"], ["ΣMz", r.mzNm, "N·m"]]);
    return omitted;
  };

  const legends: Legend[] = [showPressure ? pressureLegend() : members.legend, ...(max > 0 ? [arrowLegend(ctx.theme, max, longest)] : []), ...diagrams.map((d) => diagramLegend(ctx, d.kind, d.peak))];
  return { styleOf: members.styleOf, overlay, legends, ...(options.diagrams.freeBody ? { inset } : {}) };
}

function momentCurl(canvas: Canvas, ctx: Ctx, id: string, at: Vec, curl: number, radius: number, color: string, dash: readonly number[] | undefined, title: Text, total: number): void {
  const from = -Math.PI / 2;
  const sweep = curl > 0 ? (-3 * Math.PI) / 2 : (3 * Math.PI) / 2;
  const end = from + sweep;
  const tangent = { x: -Math.sin(end) * Math.sign(sweep), y: Math.cos(end) * Math.sign(sweep) };
  const tip = add({ x: at.x + radius * Math.cos(end), y: at.y + radius * Math.sin(end) }, scaleVec(tangent, 0.9));
  canvas.path("forces", id, arcCommands(at, radius, from, sweep), { stroke: { color, widthMm: 0.3, cap: "round", ...(dash ? { dashMm: dash } : {}) }, value: { quantity: "moment", value: curl, unit: "N·m", label: title } });
  canvas.path("forces", `${id}:head`, commandsOf(headOutline(tip, tangent, 1.4, 1.1), true), { fill: { color } });
  void ctx;
  void total;
}

function momentVector(canvas: Canvas, id: string, at: Vec, vector: { readonly x: number; readonly y: number }, half: number, color: string, dash: readonly number[] | undefined, title: Text, value: number): void {
  const len = Math.hypot(vector.x, vector.y);
  const dir = unit({ x: vector.x / len, y: -vector.y / len });
  const tail = sub(at, scaleVec(dir, half));
  const head = add(at, scaleVec(dir, half));
  canvas.path("forces", id, commandsOf([tail, head], false), { stroke: { color, widthMm: 0.3, cap: "round", ...(dash ? { dashMm: dash } : {}) }, value: { quantity: "moment", value, unit: "N·m", label: title } });
  for (const back of [0, 1.1]) canvas.path("forces", `${id}:head-${back}`, commandsOf(headOutline(sub(head, scaleVec(dir, back)), dir, 1.4, 1.1), true), { fill: { color } });
}

type MomentItem = { readonly id: string; readonly at: Vec3; readonly m: Vec3; readonly kind: MomentKind; readonly title: Text };

/** 🔄️ The moment symbols of the moment view as world vectors: joints, active reactions and applied couples. */
export function momentList(state: EvaluatedState, result: StructuralResult): MomentItem[] {
  const boneTitle = (id: Slug): Text => state.asset.rig.bones.find((b) => b.id === id)?.title ?? tx(id, id);
  return [
    ...Object.values(result.joints).map((j) => ({ id: `moment:joint:${j.bone}`, at: at3(j.at, j.z), m: { x: j.mxNm, y: j.myNm, z: j.mzNm }, kind: "joint" as const, title: boneTitle(j.bone) })),
    ...result.reactions.filter((r) => r.active).map((r) => ({ id: `moment:reaction:${r.anchor}`, at: at3(r.at, r.z), m: { x: r.mxNm, y: r.myNm, z: r.mzNm }, kind: "reaction" as const, title: state.anchors[r.anchor]?.anchor.title ?? tx(r.anchor, r.anchor) })),
    ...result.applied.map((f) => ({ id: `moment:applied:${f.id}`, at: worldPoint(f), m: { x: f.mxNm, y: f.myNm, z: f.mzNm }, kind: "applied" as const, title: f.title })),
  ];
}

/** 🔄️ Builds the moments view: members coloured by bending moment, joint, reaction and applied moments as curls (about the line of sight) and double-headed arrows (in the view plane), and the bending-moment diagram. */
export function momentsView(ctx: Ctx, shapes: readonly Prepared[], result: StructuralResult): AnalyticView {
  const { options, state } = ctx;
  void shapes;
  const viewpoint = options.viewpoint;
  const members = memberView(ctx, shapes, result, "moment", options.patterns);
  const arcs = momentList(state, result);
  const max = Math.max(0, ...arcs.map((a) => magnitude(a.m)));
  const largest = arcs.reduce<MomentItem | null>((best, a) => (best === null || magnitude(a.m) > magnitude(best.m) ? a : best), null);
  const peak = peakOf(result, "moment", memberFilter(state));
  const overlay = (canvas: Canvas): void => {
    drawSupports(canvas, ctx, result, null);
    for (const a of arcs) {
      const total = magnitude(a.m);
      if (total < Math.max(1e-9, max * MOMENT_MIN_FRACTION)) continue;
      const projection = projectMoment(viewpoint, a.m);
      const at = placeOf(ctx, a.at);
      const color = MOMENT_COLOUR(ctx.theme, a.kind);
      const dash = MOMENT_DASH[a.kind];
      const radius = (part: number): number => 1.6 + 3.2 * Math.sqrt(part / max);
      const required = a.kind === "reaction" || a === largest;
      const inPlane = Math.hypot(projection.vector.x, projection.vector.y);
      const curl = Math.abs(projection.curl);
      if (curl > OUT_OF_PLANE_MIN * total) momentCurl(canvas, ctx, a.id, at, projection.curl, radius(curl), color, dash, a.title, total);
      if (inPlane > OUT_OF_PLANE_MIN * total) momentVector(canvas, `${a.id}:vector`, at, projection.vector, radius(inPlane), color, dash, a.title, inPlane);
      if (options.annotations.labels) {
        const parts = [...(curl > OUT_OF_PLANE_MIN * total ? [`${projection.curl > 0 ? "↺" : "↻"} ${formatQuantity(curl, "N·m", ctx.language)}`] : []), ...(inPlane > OUT_OF_PLANE_MIN * total ? [`⟷ ${formatQuantity(inPlane, "N·m", ctx.language)}`] : [])];
        ctx.labels.add(`${a.id}:label`, { x: at.x + radius(Math.max(curl, inPlane)), y: at.y }, `${a.title[ctx.language]} ${formatQuantity(total, "N·m", ctx.language)}${parts.length > 1 ? ` (${parts.join(", ")})` : ""}`, { required, magnitude: total });
      }
    }
    if (options.diagrams.moment && peak > 1e-9) drawDiagram(canvas, ctx, result, "moment", peak);
  };
  const legends: Legend[] = [members.legend];
  if (max > 0)
    legends.push(
      legendOf({
        title: tx("Moment symbols", "Momentensymbole"),
        quantity: "moment",
        unit: "N·m",
        scale: { min: 0, centre: 0, max },
        colourAt: () => overlayColours(ctx.theme).joint,
        convention: tx(
          "Joint moment = moment of all external loads on the child subtree about the joint; reaction moments act about the support anchor, applied couples at their point. The components (mx, my, mz) are those of M = r × F in the world axes x ahead, y up, z left. This frame is left-handed, so the double-headed arrow of the in-plane part follows the LEFT-hand rule. A moment about the line of sight is a curl (↺ counter-clockwise, ↻ clockwise as seen). Positive mz is counter-clockwise in the side elevation, positive mx is clockwise in the front elevation and positive my is clockwise in the plan. Orange = joint (solid), teal = support reaction (long dash), red = applied moment (dotted).",
          "Gelenkmoment = Moment aller äußeren Lasten des Kind-Teilbaums um das Gelenk; Auflagermomente wirken um den Auflagerpunkt, äußere Momente an ihrem Angriffspunkt. Die Komponenten (mx, my, mz) sind die von M = r × F in den Weltachsen x voraus, y oben, z links. Dieses Koordinatensystem ist linkshändig, daher folgt der Doppelpfeil des ebenen Anteils der LINKEN-Hand-Regel. Ein Moment um die Blickrichtung ist ein Bogen (↺ gegen den Uhrzeigersinn, ↻ im Uhrzeigersinn, wie gesehen). Ein positives mz dreht in der Seitenansicht gegen den Uhrzeigersinn, ein positives mx in der Vorderansicht im Uhrzeigersinn und ein positives my in der Draufsicht im Uhrzeigersinn. Orange = Gelenk (durchgezogen), Türkis = Auflagerreaktion (langgestrichelt), Rot = äußeres Moment (gepunktet).",
        ),
        interpretation: tx(`Symbol size grows with √|M| of its drawn part: 1.6 mm ≙ 0 N·m, 4.8 mm ≙ ${formatNumber(max, 1, "en")} N·m (the largest invariant total); symbols below ${MOMENT_MIN_FRACTION * 100} % of the maximum are not drawn; labels give the total and its in-plane and line-of-sight parts.`, `Die Symbolgröße wächst mit √|M| des gezeichneten Anteils: 1,6 mm ≙ 0 N·m, 4,8 mm ≙ ${formatNumber(max, 1, "de")} N·m (die größte unveränderliche Gesamtgröße); Symbole unter ${MOMENT_MIN_FRACTION * 100} % des Maximums werden nicht gezeichnet; die Labels nennen die Gesamtgröße und ihre ebenen und Blickrichtungsanteile.`),
      }),
    );
  if (options.diagrams.moment && peak > 1e-9) legends.push(diagramLegend(ctx, "moment", peak));
  return { styleOf: members.styleOf, overlay, legends };
}
