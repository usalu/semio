/** 🏗️ The force and moment views: members coloured by the chosen internal quantity, force and reaction arrows, support markers, centre of mass,
 * free-body inset, joint-moment arcs and member diagrams offset perpendicular to the member.
 *
 * Arrows show the force acting ON the body at its point of application with the head at that point (loads press onto the body, reactions push
 * up into it). Axial N > 0 tension (cool, dotted), N < 0 compression (warm, hatched); M > 0 sagging, drawn on the tension side.
 *
 * @see ../../🏗️structure/🟦️.ts — `solveStructure`, `structuralLegend`, `freeBody`
 * @see ../../🔗️contracts/🟦️.ts — {@link StructuralResult}
 */
import type { Slug, Text } from "../../../🧬️schema/🟦️.ts";
import type { DiagramStation, EvaluatedState, Fill, Legend, MemberDiagram, Point, Reaction, StructuralResult, WorldForce } from "../../🔗️contracts/🟦️.ts";
import { formatNumber, formatQuantity, LABEL_TEXT_MM, ANNOTATION_TEXT_MM, ANNOTATION_PEN_MM } from "../../📐️units/🟦️.ts";
import { freeBody, structuralLegend, type StructuralQuantity } from "../../🏗️structure/🟦️.ts";
import { type Canvas, type Ctx, Labels, labelBudget } from "../🧱️canvas/🟦️.ts";
import { overlayColours, type Scale, divergingColour, legendOf, mix, toneColour, tx, type Theme } from "../🖍️palette/🟦️.ts";
import type { AnalyticView, Prepared, Style } from "../🧩️shapes/🟦️.ts";
import { type Vec, add, arcCommands, arrowOutline, circleCommands, commandsOf, headOutline, lerp, perp, ringsCommands, scale as scaleVec, sectorCommands, sub } from "../📐️geometry/🟦️.ts";

const PATTERN_FRACTION = 0.15;
const DIAGRAM_MM = 6;
const ARROW_MAX_MM = 14;
const ARROW_MIN_MM = 1.5;
const INSET_HEIGHT_MM = 30;
const INSET_GAP_MM = 16;
const MOMENT_MIN_FRACTION = 0.02;

type ArrowKind = "applied" | "reaction" | "weight" | "inertial";
/** 📈️ A member diagram kind. */
export type DiagramKind = "axial" | "shear" | "moment";
type DiagramRun = { base: Vec[]; off: Vec[]; peak: number; peakAt: Vec };

const arrowColour = (theme: Theme, kind: ArrowKind): string => {
  const o = overlayColours(theme);
  return kind === "applied" ? o.applied : kind === "reaction" ? o.reaction : kind === "weight" ? o.weight : o.joint;
};
const DIAGRAM_META: Readonly<Record<DiagramKind, { readonly unit: "N" | "N·m"; readonly title: Text; readonly pick: (s: DiagramStation) => number; readonly side: 1 | -1 }>> = {
  axial: { unit: "N", title: tx("Axial force diagram N", "Normalkraftlinie N"), pick: (s) => s.axialN, side: 1 },
  shear: { unit: "N", title: tx("Shear force diagram V", "Querkraftlinie V"), pick: (s) => s.shearN, side: 1 },
  moment: { unit: "N·m", title: tx("Bending moment diagram M", "Momentenlinie M"), pick: (s) => s.momentNm, side: -1 },
};

function memberValue(diagram: MemberDiagram | undefined, quantity: StructuralQuantity): number | null {
  if (!diagram) return null;
  const values = diagram.stations.flatMap((s) => (quantity === "axial" ? [s.axialN] : quantity === "shear" ? [s.shearN] : quantity === "moment" ? [s.momentNm] : s.stressMinPa === null || s.stressMaxPa === null ? [] : [s.stressMinPa, s.stressMaxPa]));
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
  const base = structuralLegend(quantity, result, ctx.options.range);
  const eligible = memberFilter(ctx.state);
  const values = quantity === "contact-pressure" ? [] : Object.entries(result.members).flatMap(([id, m]) => (eligible(id) ? [memberValue(m, quantity)] : [])).filter((v): v is number => v !== null);
  const range = ctx.options.range;
  const scale = symmetricIfEmpty(range.mode === "fixed" ? range.min : Math.min(0, ...values), range.mode === "fixed" ? range.max : Math.max(0, ...values));
  const boneTitle = (id: Slug): Text => ctx.state.asset.rig.bones.find((b) => b.id === id)?.title ?? tx(id, id);
  const regionKind = (id: Slug): string | undefined => ctx.state.asset.thermal.regions.find((r) => r.id === id)?.kind;
  const fmt = (v: number, l: "en" | "de"): string => formatNumber(v, Math.max(0, 2 - Math.floor(Math.log10(Math.max(Math.abs(v), 1e-9)))), l);
  const interpretation = tx(
    `${base.title.en} from ${fmt(scale.min, "en")} to ${fmt(scale.max, "en")} ${base.unit}; zero is grey; bones without a section and shorter than 5 % of the object height, foliage, glazing and texture are drawn neutral (ghosted).`,
    `${base.title.de} von ${fmt(scale.min, "de")} bis ${fmt(scale.max, "de")} ${base.unit}; null ist grau; Stäbe ohne Querschnitt und kürzer als 5 % der Objekthöhe sowie Laub, Verglasung und Textur sind neutral (blass) gezeichnet.`,
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
    const pattern: Fill["pattern"] | undefined = patterns && strong && (quantity === "axial" || quantity === "stress") ? (v < 0 ? "compression" : "tension") : undefined;
    return { fill: { color: divergingColour(v, scale, false, ctx.theme), ...(glaze ? { opacity: 0.6 } : {}), ...(pattern ? { pattern } : {}) }, value, outlineMm };
  };
  return { styleOf, legend };
}

type Frame = { readonly origin: Vec; readonly up: Vec; readonly tangent: Vec };

function frameOf(ctx: Ctx, r: Reaction): Frame {
  const anchor = ctx.state.anchors[r.anchor]?.anchor;
  const theta = ((anchor?.normalDeg ?? 90) * Math.PI) / 180;
  const up = { x: Math.cos(theta), y: -Math.sin(theta) };
  return { origin: ctx.paper(r.at), up, tangent: perp(up) };
}

const local = (f: Frame, a: number, b: number): Vec => add(f.origin, add(scaleVec(f.tangent, a), scaleVec(f.up, b)));

function drawSupports(canvas: Canvas, ctx: Ctx, result: StructuralResult, pressure: { readonly colour: (v: number) => string } | null): void {
  const { colours, options } = ctx;
  const w = ANNOTATION_PEN_MM;
  const stroke = { color: colours.ink, widthMm: w * 1.6, join: "miter" as const };
  for (const r of result.reactions) {
    const anchor = ctx.state.anchors[r.anchor]?.anchor;
    if (!anchor) continue;
    const f = frameOf(ctx, r);
    const type = anchor.support ?? "contact";
    const id = `support:${r.anchor}`;
    const opacity = r.active ? 1 : 0.35;
    const dim = { ...stroke, opacity };
    const ticks = (b: number, xs: readonly number[]): void =>
      xs.forEach((x, i) => canvas.path("forces", `${id}:hatch-${i}`, commandsOf([local(f, x, b), local(f, x - 0.7, b - 0.8)], false), { stroke: { ...dim, widthMm: w } }));
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
}

function arrowLengthMm(mag: number, max: number, longest: number): number {
  return max <= 0 ? 0 : Math.max(ARROW_MIN_MM, (mag / max) * longest);
}

type ArrowSpec = { readonly id: string; readonly kind: ArrowKind; readonly head: Vec; readonly force: Point; readonly title: Text; readonly longest: number; readonly max: number; readonly drawn: boolean; readonly label: boolean; readonly required: boolean };

function drawArrow(canvas: Canvas, ctx: Ctx, spec: ArrowSpec, labels: Labels): void {
  const mag = Math.hypot(spec.force.x, spec.force.y);
  if (mag < 1e-9) return;
  const dir = { x: spec.force.x / mag, y: -spec.force.y / mag };
  const length = arrowLengthMm(mag, spec.max, spec.longest);
  const tail = sub(spec.head, scaleVec(dir, length));
  const color = arrowColour(ctx.theme, spec.kind);
  if (spec.drawn)
    canvas.path("forces", spec.id, commandsOf(arrowOutline(tail, spec.head, 0.3, Math.min(1.8, length * 0.6), 1.3), true), {
      fill: { color },
      value: { quantity: "force", value: mag, unit: "N", label: spec.title },
    });
  if (spec.label) labels.add(`${spec.id}:label`, spec.drawn ? tail : spec.head, `${spec.title[ctx.language]} ${formatQuantity(mag, "N", ctx.language)}`, { required: spec.required, magnitude: mag });
}

/** 🏹️ One arrow request: `drawn` arrows are painted, `labelled` ones get a value label, `required` labels survive the label budget. */
export type ForceItem = { readonly id: string; readonly kind: ArrowKind; readonly at: Point; readonly force: Point; readonly title: Text; readonly drawn: boolean; readonly labelled: boolean; readonly required: boolean };

const loadGroup = (id: string): string | null => (/#\d+$/u.test(id) ? id.replace(/#\d+$/u, "") : null);

/** ➕️ Sub-forces of one distributed load (`<load>#<n>`) stay as small unlabelled arrows plus one undrawn resultant carrying the single label. */
export function withResultants(items: readonly ForceItem[]): ForceItem[] {
  const groups = new Map<string, ForceItem[]>();
  for (const item of items) {
    const g = loadGroup(item.id);
    if (g !== null) groups.set(g, [...(groups.get(g) ?? []), item]);
  }
  const out: ForceItem[] = items.map((i) => (loadGroup(i.id) === null ? i : { ...i, labelled: false }));
  for (const [g, list] of groups)
    out.push({
      id: `${g}:resultant`,
      kind: list[0]!.kind,
      at: { x: list.reduce((s, f) => s + f.at.x, 0) / list.length, y: list.reduce((s, f) => s + f.at.y, 0) / list.length },
      force: { x: list.reduce((s, f) => s + f.force.x, 0), y: list.reduce((s, f) => s + f.force.y, 0) },
      title: tx(`${list[0]!.title.en}, distributed`, `${list[0]!.title.de}, verteilt`),
      drawn: false,
      labelled: true,
      required: false,
    });
  return out;
}

/** 🏹️ The arrows of the force view: applied loads (distributed ones with one resultant), active reactions, total weight and, for inertial states, total inertia. */
export function forceList(state: EvaluatedState, result: StructuralResult): ForceItem[] {
  const total = (list: readonly WorldForce[]): Point => ({ x: list.reduce((s, f) => s + f.fxN, 0), y: list.reduce((s, f) => s + f.fyN, 0) });
  const weight = total(result.segmentWeights);
  const inertial = total(result.inertialForces);
  const reactionTitle = (r: Reaction): Text => state.anchors[r.anchor]?.anchor.title ?? tx(r.anchor, r.anchor);
  const plain = { drawn: true, labelled: true, required: false };
  return withResultants([
    ...result.applied.map((f) => ({ id: `force:applied:${f.id}`, kind: "applied" as const, at: f.at, force: { x: f.fxN, y: f.fyN }, title: f.title, ...plain })),
    ...result.reactions.filter((r) => r.active).map((r) => ({ id: `force:reaction:${r.anchor}`, kind: "reaction" as const, at: r.at, force: { x: r.fxN, y: r.fyN }, title: reactionTitle(r), ...plain, required: true })),
    { id: "force:weight", kind: "weight" as const, at: result.centreOfMass, force: weight, title: tx("Weight", "Gewicht"), ...plain },
    ...(Math.hypot(inertial.x, inertial.y) > 1e-9 ? [{ id: "force:inertial", kind: "inertial" as const, at: result.centreOfMass, force: inertial, title: tx("Inertia", "Trägheit"), ...plain }] : []),
  ]);
}

function drawCentreOfMass(canvas: Canvas, ctx: Ctx, at: Vec, mass: number): void {
  const r = 1.3;
  const ink = ctx.colours.ink;
  canvas.path("forces", "com:disc", circleCommands(at, r), { stroke: { color: ink, widthMm: 0.25 }, fill: { color: ctx.colours.background }, value: { quantity: "total-mass", value: mass, unit: "kg", label: tx("Centre of mass", "Massenschwerpunkt") } });
  canvas.path("forces", "com:quadrant-a", sectorCommands(at, r, 0, Math.PI / 2), { fill: { color: ink } });
  canvas.path("forces", "com:quadrant-b", sectorCommands(at, r, Math.PI, (3 * Math.PI) / 2), { fill: { color: ink } });
}

/** 📈️ The largest absolute ordinate of a diagram kind over all members. */
export function peakOf(result: StructuralResult, kind: DiagramKind, include: (bone: Slug) => boolean = () => true): number {
  const pick = DIAGRAM_META[kind].pick;
  return Math.max(0, ...Object.entries(result.members).flatMap(([id, m]) => (include(id) ? m.stations.map((s) => Math.abs(pick(s))) : [])));
}

/** 📈️ One filled region of a member diagram: the baseline followed by the reversed ordinate curve, and its peak. */
export type DiagramPolygon = { readonly bone: Slug; readonly index: number; readonly points: readonly Vec[]; readonly peak: number; readonly peakAt: Vec };

/** 📈️ The sign-split polygons of one diagram kind for every member, ordinates offset perpendicular to the bone by `ordinate · value / peak` on the kind's side; `yDown` selects the paper (true) or model (false) handedness. */
export function diagramPolygons(state: EvaluatedState, result: StructuralResult, kind: DiagramKind, peak: number, map: (p: Point) => Vec, ordinate: number, yDown: boolean, include: (bone: Slug) => boolean = () => true): DiagramPolygon[] {
  const meta = DIAGRAM_META[kind];
  const out: DiagramPolygon[] = [];
  for (const [bone, member] of Object.entries(result.members)) {
    const evaluated = state.bones[bone];
    if (!evaluated || member.stations.length < 2 || !include(bone)) continue;
    const left = { x: -Math.sin(evaluated.angleRad), y: (yDown ? -1 : 1) * Math.cos(evaluated.angleRad) };
    const base = member.stations.map((s) => map(s.at));
    const values = member.stations.map(meta.pick);
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

function drawDiagram(canvas: Canvas, ctx: Ctx, result: StructuralResult, kind: DiagramKind, peak: number): void {
  const meta = DIAGRAM_META[kind];
  const sc: Scale = { min: -peak, centre: 0, max: peak };
  for (const r of diagramPolygons(ctx.state, result, kind, peak, ctx.paper, DIAGRAM_MM, true, memberFilter(ctx.state))) {
    const color = divergingColour(r.peak, sc, false, ctx.theme);
    const label = ctx.state.asset.rig.bones.find((b) => b.id === r.bone)?.title ?? tx(r.bone, r.bone);
    const id = `diagram:${kind}:${r.bone}:${r.index}`;
    canvas.path("diagrams", id, commandsOf(r.points, true), {
      fill: { color, opacity: 0.4 },
      stroke: { color, widthMm: 0.2, join: "round" },
      value: { quantity: kind === "moment" ? "bending-moment" : kind === "axial" ? "axial-force" : "shear-force", value: r.peak, unit: meta.unit, label },
      bone: r.bone,
    });
    if (ctx.options.annotations.labels) ctx.labels.add(`${id}:label`, r.peakAt, `${label[ctx.language]} ${formatQuantity(r.peak, meta.unit, ctx.language)}`, { required: false, magnitude: Math.abs(r.peak) });
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
      ? tx("M > 0 sagging (cool), M < 0 hogging (warm); the ordinate is drawn on the tension side of the member (M > 0 on the bone's local −y face).", "M > 0 Feldmoment (kühl), M < 0 Stützmoment (warm); die Ordinate liegt auf der Zugseite des Stabes (M > 0 auf der lokalen −y-Seite des Stabes).")
      : tx("Positive values (tension for N) are drawn on the bone's local +y side (left of the bone direction), negative values on the other side.", "Positive Werte (Zug bei N) liegen auf der lokalen +y-Seite des Stabes (links der Stabrichtung), negative Werte auf der anderen Seite."),
    interpretation: tx(`Ordinate scale: ${DIAGRAM_MM} mm ≙ ${formatNumber(peak, 2, "en")} ${meta.unit}, the same for all members.`, `Ordinatenmaßstab: ${DIAGRAM_MM} mm ≙ ${formatNumber(peak, 2, "de")} ${meta.unit}, für alle Stäbe gleich.`),
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
    convention: tx("The arrow is the force acting on the body, its head at the point of application; world axes x right, y up. Red = applied load, teal = support reaction, grey = weight, orange = inertia; quartered disc = centre of mass.", "Der Pfeil ist die auf den Körper wirkende Kraft, die Spitze liegt im Angriffspunkt; Weltachsen x nach rechts, y nach oben. Rot = äußere Last, Türkis = Auflagerreaktion, Grau = Gewicht, Orange = Trägheit; geviertelte Scheibe = Massenschwerpunkt."),
    interpretation: tx(`Arrow length is proportional to |F| (minimum ${ARROW_MIN_MM} mm): ${longest} mm ≙ ${formatNumber(max, 1, "en")} N.`, `Die Pfeillänge ist proportional zu |F| (mindestens ${ARROW_MIN_MM} mm): ${longest} mm ≙ ${formatNumber(max, 1, "de")} N.`),
  });
}

/** 🏗️ Builds the forces view: members coloured by the chosen quantity, supports, load and reaction arrows, centre of mass, optional N/V diagrams and free-body inset. */
export function forcesView(ctx: Ctx, shapes: readonly Prepared[], result: StructuralResult): AnalyticView {
  const { options, state } = ctx;
  const quantity = options.forceQuantity;
  const members = memberView(ctx, shapes, result, quantity, options.patterns);
  const forces = forceList(state, result);
  const max = Math.max(0, ...forces.map((f) => Math.hypot(f.force.x, f.force.y)));
  const heightMm = (state.bounds.maxY - state.bounds.minY) * ctx.k;
  const longest = Math.min(ARROW_MAX_MM, Math.max(7, heightMm * 0.9));
  const pressures = result.reactions.flatMap((r) => (r.pressurePa === null ? [] : [r.pressurePa]));
  const pmax = Math.max(1, ...pressures);
  const pScale: Scale = { min: -pmax, centre: 0, max: pmax };
  const pressureColour = { colour: (v: number): string => divergingColour(v, pScale, true, ctx.theme) };
  const showPressure = quantity === "contact-pressure";
  const pressureLegend = (): Legend => {
    const base = structuralLegend("contact-pressure", result, options.range);
    return legendOf({ title: base.title, quantity: "contact-pressure", unit: "Pa", scale: { min: 0, centre: 0, max: pmax }, colourAt: pressureColour.colour, convention: tx("Compression only: normal reaction divided by the footprint area; warm = pressure.", "Nur Druck: Normalreaktion geteilt durch die Aufstandsfläche; warm = Pressung."), interpretation: base.interpretation });
  };
  const diagrams = (["axial", "shear"] as const).filter((k) => options.diagrams[k]).map((k) => ({ kind: k, peak: peakOf(result, k, memberFilter(state)) })).filter((d) => d.peak > 1e-9);

  const overlay = (canvas: Canvas): void => {
    drawSupports(canvas, ctx, result, showPressure ? pressureColour : null);
    for (const f of forces) drawArrow(canvas, ctx, { id: f.id, kind: f.kind, head: ctx.paper(f.at), force: f.force, title: f.title, longest, max, drawn: f.drawn, label: options.annotations.labels && f.labelled, required: f.required }, ctx.labels);
    drawCentreOfMass(canvas, ctx, ctx.paper(result.centreOfMass), result.totalMassKg);
    for (const d of diagrams) drawDiagram(canvas, ctx, result, d.kind, d.peak);
  };

  const inset = (canvas: Canvas): number => {
    const box = canvas.box;
    if (box === null) return 0;
    const hm = Math.max(1e-6, state.bounds.maxY - state.bounds.minY);
    const fi = Math.max(ctx.k, INSET_HEIGHT_MM / hm);
    const x0 = box.maxX + INSET_GAP_MM;
    const y0 = box.minY + 6;
    const map = (p: Point): Vec => ({ x: x0 + (p.x - state.bounds.minX) * fi, y: y0 + (state.bounds.maxY - p.y) * fi });
    canvas.text("diagrams", "freebody:title", { x: x0, y: box.minY + ANNOTATION_TEXT_MM }, ctx.language === "de" ? "Freikörperbild" : "Free-body diagram", ANNOTATION_TEXT_MM, ctx.colours.ink, { weight: "bold" });
    for (const p of shapes) {
      if (!p.closed || !p.shape.filled || p.shape.role === "glazing" || p.shape.role === "texture") continue;
      const rings = p.rings.map((r) => r.map((q) => map({ x: q.x / ctx.k, y: -q.y / ctx.k })));
      canvas.path("diagrams", `freebody:outline:${p.shape.id}`, ringsCommands(rings, true), { stroke: { color: ctx.colours.ink, widthMm: 0.18, opacity: 0.55, dashMm: [0.8, 0.5] }, fill: { color: mix(ctx.colours.background, ctx.colours.ink, 0.06) } });
    }
    const longestInset = Math.min(10, Math.max(5, hm * fi * 0.35));
    const fb: ForceItem[] = withResultants(freeBody(result).map((f) => ({ id: `freebody:${f.kind}:${f.id}`, kind: f.kind, at: f.at, force: { x: f.fxN, y: f.fyN }, title: f.title, drawn: true, labelled: f.kind === "applied" || f.kind === "reaction", required: f.kind === "reaction" }))).filter((f) => !f.drawn || loadGroup(f.id) === null).map((f) => (f.drawn ? f : { ...f, drawn: true }));
    const maxInset = Math.max(0, ...fb.map((f) => Math.hypot(f.force.x, f.force.y)));
    const insetLabels = new Labels();
    for (const f of fb) drawArrow(canvas, ctx, { id: f.id, kind: f.kind, head: map(f.at), force: f.force, title: f.title, longest: longestInset, max: maxInset, drawn: true, label: options.annotations.labels && f.labelled, required: f.required }, insetLabels);
    drawCentreOfMass(canvas, ctx, map(result.centreOfMass), result.totalMassKg);
    const omitted = insetLabels.flush(canvas, "diagrams", (canvas.box?.maxX ?? x0) + 4, LABEL_TEXT_MM, ctx.colours.ink, 0.1, labelBudget(options.scale.denominator));
    const r = result.residual;
    const weightN = Math.abs(result.segmentWeights.reduce((sum, f) => sum + f.fyN, 0));
    const tiny = (v: number, scaleN: number): number => (Math.abs(v) < 1e-6 * Math.max(1, scaleN) ? 0 : v);
    const lang = ctx.language;
    const sums = `ΣFx ${formatQuantity(tiny(r.fxN, weightN), "N", lang)}, ΣFy ${formatQuantity(tiny(r.fyN, weightN), "N", lang)}, ΣM ${formatQuantity(tiny(r.momentNm, weightN), "N·m", lang)}`;
    const labelY = (canvas.box?.maxY ?? y0 + hm * fi) + 4;
    canvas.text("diagrams", "freebody:residual", { x: x0, y: labelY }, `${lang === "de" ? "Restkräfte" : "Residual"}: ${sums}`, LABEL_TEXT_MM, ctx.colours.ink);
    return omitted;
  };

  const legends: Legend[] = [showPressure ? pressureLegend() : members.legend, ...(max > 0 ? [arrowLegend(ctx.theme, max, longest)] : []), ...diagrams.map((d) => diagramLegend(ctx, d.kind, d.peak))];
  return { styleOf: members.styleOf, overlay, legends, ...(options.diagrams.freeBody ? { inset } : {}) };
}

function momentArc(canvas: Canvas, ctx: Ctx, id: string, at: Vec, moment: number, radius: number, color: string, title: Text, required: boolean): void {
  const from = -Math.PI / 2;
  const sweep = moment > 0 ? (-3 * Math.PI) / 2 : (3 * Math.PI) / 2;
  const end = from + sweep;
  const tangent = { x: -Math.sin(end) * Math.sign(sweep), y: Math.cos(end) * Math.sign(sweep) };
  const tip = add({ x: at.x + radius * Math.cos(end), y: at.y + radius * Math.sin(end) }, scaleVec(tangent, 0.9));
  canvas.path("forces", id, arcCommands(at, radius, from, sweep), { stroke: { color, widthMm: 0.3, cap: "round" }, value: { quantity: "moment", value: moment, unit: "N·m", label: title } });
  canvas.path("forces", `${id}:head`, commandsOf(headOutline(tip, tangent, 1.4, 1.1), true), { fill: { color } });
  if (ctx.options.annotations.labels) ctx.labels.add(`${id}:label`, { x: at.x + radius, y: at.y }, `${title[ctx.language]} ${formatQuantity(moment, "N·m", ctx.language)}`, { required, magnitude: Math.abs(moment) });
}

/** 🔄️ Builds the moments view: members coloured by bending moment, joint-moment arcs sized by |M|, reaction and applied moments, and the bending-moment diagram. */
export function momentsView(ctx: Ctx, shapes: readonly Prepared[], result: StructuralResult): AnalyticView {
  const { options, state } = ctx;
  const members = memberView(ctx, shapes, result, "moment", false);
  const boneTitle = (id: Slug): Text => state.asset.rig.bones.find((b) => b.id === id)?.title ?? tx(id, id);
  const arcs = [
    ...Object.values(result.joints).map((j) => ({ id: `moment:joint:${j.bone}`, at: j.at, m: j.momentNm, color: overlayColours(ctx.theme).joint, title: boneTitle(j.bone) })),
    ...result.reactions.filter((r) => r.active).map((r) => ({ id: `moment:reaction:${r.anchor}`, at: r.at, m: r.momentNm, color: overlayColours(ctx.theme).reaction, title: state.anchors[r.anchor]?.anchor.title ?? tx(r.anchor, r.anchor) })),
    ...result.applied.map((f) => ({ id: `moment:applied:${f.id}`, at: f.at, m: f.momentNm, color: overlayColours(ctx.theme).applied, title: f.title })),
  ];
  const max = Math.max(0, ...arcs.map((a) => Math.abs(a.m)));
  const largest = arcs.reduce<(typeof arcs)[number] | null>((best, a) => (best === null || Math.abs(a.m) > Math.abs(best.m) ? a : best), null);
  const peak = peakOf(result, "moment", memberFilter(state));
  const overlay = (canvas: Canvas): void => {
    drawSupports(canvas, ctx, result, null);
    for (const a of arcs) if (Math.abs(a.m) >= Math.max(1e-9, max * MOMENT_MIN_FRACTION)) momentArc(canvas, ctx, a.id, ctx.paper(a.at), a.m, 1.6 + 3.2 * Math.sqrt(Math.abs(a.m) / max), a.color, a.title, a.id.startsWith("moment:reaction:") || a === largest);
    if (options.diagrams.moment && peak > 1e-9) drawDiagram(canvas, ctx, result, "moment", peak);
  };
  const legends: Legend[] = [members.legend];
  if (max > 0)
    legends.push(
      legendOf({
        title: tx("Moment arcs", "Momentenbögen"),
        quantity: "moment",
        unit: "N·m",
        scale: { min: 0, centre: 0, max },
        colourAt: () => overlayColours(ctx.theme).joint,
        convention: tx("Joint moment = moment of all external loads on the child subtree about the joint, counter-clockwise positive; the arrow shows its sense. Orange = joint, teal = support reaction, red = applied moment.", "Gelenkmoment = Moment aller äußeren Lasten des Kind-Teilbaums um das Gelenk, gegen den Uhrzeigersinn positiv; der Pfeil zeigt den Drehsinn. Orange = Gelenk, Türkis = Auflagerreaktion, Rot = äußeres Moment."),
        interpretation: tx(`Arc radius grows with √|M|: 1.6 mm ≙ 0 N·m, 4.8 mm ≙ ${formatNumber(max, 1, "en")} N·m; arcs below ${MOMENT_MIN_FRACTION * 100} % of the maximum are not drawn.`, `Der Bogenradius wächst mit √|M|: 1,6 mm ≙ 0 N·m, 4,8 mm ≙ ${formatNumber(max, 1, "de")} N·m; Bögen unter ${MOMENT_MIN_FRACTION * 100} % des Maximums werden nicht gezeichnet.`),
      }),
    );
  if (options.diagrams.moment && peak > 1e-9) legends.push(diagramLegend(ctx, "moment", peak));
  return { styleOf: members.styleOf, overlay, legends };
}
