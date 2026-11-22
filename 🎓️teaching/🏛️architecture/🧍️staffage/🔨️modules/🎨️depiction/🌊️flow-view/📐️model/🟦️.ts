/** 📐️ The thermodynamic flow as CAD model-space geometry: the marks of one flow field ({@link flowMarks}, generalised for the drawing scale the file is made for) projected into a viewpoint, in metres at full size, on named layers.
 *
 * | layer | holds | ACI | lineweight (1/100 mm) |
 * |---|---|---|---|
 * | `FLOW-STREAMS-WARM`, `-NEUTRAL`, `-COOL` | the stream lines, cut where the air changes its class | 1, 8, 5 | 18 |
 * | `FLOW-ARROWS-WARM`, `-NEUTRAL`, `-COOL` | the arrow heads of the streams, closed triangles pointing where the air goes | 1, 8, 5 | 18 |
 * | `FLOW-TRACERS-WARM`, `-NEUTRAL`, `-COOL` | the tracer streaks (tail to head) and the head of each as a circle | 1, 8, 5 | 25 |
 * | `FLOW-PATHS-WARM`, `-NEUTRAL`, `-COOL` | the path each parcel travelled (the whole path, or its last `trailS` seconds) | 1, 8, 5 | 13 |
 * | `FLOW-VAPOUR` | the vapour puffs as circles and the moist reaches of the streams | 134 | 30 |
 * | `FLOW-FOG` | the fog puffs as circles | 4 | 30 |
 *
 * The class of air is the one the paint of the paper view ({@link flowPaint}) colours: `NEUTRAL` inside the neutral band around the ambient temperature ({@link NEUTRAL_BAND} of the span), `WARM` above it, `COOL` below. A CAD layer has one colour and one lineweight, so the continuous colour ramp and the speed-dependent width of the paper view become these classes; the sizes of arrow heads, tracer heads and the clamp of the puff radii are the paper sizes of {@link FLOW_STYLE} at the drawing scale.
 * Circles are closed polylines of {@link FLOW_CIRCLE_SEGMENTS} chords with their corners on the circle (a `ModelLayer` holds polylines and texts only). A stream seen along its own direction projects to a point and is left out. Model space keeps every mark, also those the body hides on paper: a CAD user freezes layers, the file hides nothing.
 *
 * @see ../🟦️.ts — the paper view of the same marks
 * @see ../../🟦️.ts — `modelScene`, which adds these layers
 * @see ../../../📤️export/📐️dxf/🟦️.ts — the DXF writer
 */
import type { DisplayRange, FlowField, FlowLayer, FlowMarkDetail, ModelLayer, Point, Vec3 } from "../../../🔗️contracts/🟦️.ts";
import { headOutline } from "../../📐️geometry/🟦️.ts";
import { flowDetailOf, flowMarks } from "../../../💨️transport/🌊️flow/🟦️.ts";
import { FLOW_STYLE, NEUTRAL_BAND, flowPaint } from "../🟦️.ts";
import * as portable from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

/** 🌊️ The flow a CAD model scene carries: the field, the layers to draw, the scale denominator the marks are generalised and sized for, and optionally the mark detail and the display range of the paper view. */
export type ModelFlow = { readonly field: FlowField; readonly layers: readonly FlowLayer[]; readonly denominator: number; readonly detail?: FlowMarkDetail; readonly range?: DisplayRange };

/** 🌡️ The classes of air a CAD layer distinguishes. */
export const FLOW_CLASSES = ["WARM", "NEUTRAL", "COOL"] as const;

/** 🏷️ One of {@link FLOW_CLASSES}. */
export type FlowClass = (typeof FLOW_CLASSES)[number];

/** 🎨️ The ACI colour of each class of air, of vapour and of fog, and the lineweights (1/100 mm) of the mark kinds. */
export const FLOW_CAD = { aci: { WARM: 1, NEUTRAL: 8, COOL: 5, vapour: 134, fog: 4 }, lineweight: { streams: 18, arrows: 18, tracers: 25, paths: 13, vapour: 30, fog: 30 } } as const;

/** ⭕️ The chords of a circle in model space. */
export const FLOW_CIRCLE_SEGMENTS = 32;

/** 🌡️ The class of air of a temperature excess under a span: the neutral band of the paper paint, else its sign. */
export function flowClassOf(excessK: number, excess: { readonly min: number; readonly max: number }): FlowClass {
  const limit = excessK > 0 ? Math.max(excess.max, 1e-9) : -Math.min(excess.min, -1e-9);
  return Math.abs(excessK) / limit < NEUTRAL_BAND ? "NEUTRAL" : excessK > 0 ? "WARM" : "COOL";
}

/** ⭕️ A circle as a closed polyline of {@link FLOW_CIRCLE_SEGMENTS} corners on the circle, the first at angle 0. */
export function circlePolyline(centre: Point, radiusM: number): ModelLayer["polylines"][number] {
  return { closed: true, points: Array.from({ length: FLOW_CIRCLE_SEGMENTS }, (_, i) => ({ x: centre.x + radiusM * portable.cos((2 * Math.PI * i) / FLOW_CIRCLE_SEGMENTS), y: centre.y + radiusM * portable.sin((2 * Math.PI * i) / FLOW_CIRCLE_SEGMENTS) })) };
}

const DISTINCT_M = 1e-6;

function distinct(points: readonly Point[]): Point[] {
  const kept: Point[] = [];
  for (const p of points) {
    const last = kept[kept.length - 1];
    if (last === undefined || Math.abs(p.x - last.x) > DISTINCT_M || Math.abs(p.y - last.y) > DISTINCT_M) kept.push(p);
  }
  return kept;
}

/** 🌊️ The CAD layers of a flow (see the module text) under a projection of world metres into the model plane (x right, y up); empty layers are left out. */
export function flowModelLayers(flow: ModelFlow, project: (p: Vec3) => Point): ModelLayer[] {
  const { field, denominator } = flow;
  const want = new Set(flow.layers);
  const marks = flowMarks(field, field.timeS, flowDetailOf(denominator, flow.detail));
  const paint = flowPaint("light", field, flow.range ?? { mode: "shared" });
  const metres = (paperMm: number): number => (paperMm * denominator) / 1000;
  const radius = (radiusM: number): number => Math.min(metres(FLOW_STYLE.puffMaxMm), Math.max(metres(FLOW_STYLE.puffMinMm), radiusM));
  const lines = new Map<string, ModelLayer["polylines"][number][]>();
  const put = (layer: string, points: readonly Point[], closed: boolean): void => {
    const kept = closed ? [...points] : distinct(points);
    if (kept.length < 2) return;
    const list = lines.get(layer) ?? [];
    list.push({ points: kept, closed });
    lines.set(layer, list);
  };
  if (want.has("moisture")) for (const p of marks.puffs) if (p.vapourExcessKgPerKg * 1000 > FLOW_STYLE.vapourThresholdGKg) put("FLOW-VAPOUR", circlePolyline(project(p.at), radius(p.radiusM)).points, true);
  if (want.has("fog")) for (const p of marks.puffs) if (p.condensateKgPerKg > 0) put("FLOW-FOG", circlePolyline(project(p.at), radius(p.radiusM)).points, true);
  if (want.has("streams"))
    for (const ribbon of marks.ribbons) {
      const points = ribbon.points.map(project);
      const classes = ribbon.excessK.map((e) => flowClassOf(e, paint.excess));
      let start = 0;
      for (let i = 1; i <= points.length; i++) {
        if (i < points.length && classes[i] === classes[start]) continue;
        const end = Math.min(points.length - 1, i);
        if (end > start) put(`FLOW-STREAMS-${classes[start]!}`, points.slice(start, end + 1), false);
        start = end;
        if (i >= points.length - 1) break;
      }
      for (const arrow of ribbon.arrows) {
        const tip = project(arrow.at);
        const ahead = project({ x: arrow.at.x + arrow.direction.x, y: arrow.at.y + arrow.direction.y, z: arrow.at.z + arrow.direction.z });
        const d = { x: ahead.x - tip.x, y: ahead.y - tip.y };
        if (Math.abs(d.x) + Math.abs(d.y) < 0.2) continue;
        put(`FLOW-ARROWS-${classes[Math.max(0, ribbon.points.indexOf(arrow.at))]!}`, headOutline(tip, d, metres(FLOW_STYLE.headLengthMm), metres(FLOW_STYLE.headWidthMm)), true);
      }
    }
  if (want.has("moisture"))
    for (const ribbon of marks.ribbons) {
      let run: Point[] = [];
      const flush = (): void => {
        put("FLOW-VAPOUR", run, false);
        run = [];
      };
      ribbon.points.forEach((q, i) => {
        if (ribbon.vapourExcessKgPerKg[i]! * 1000 > FLOW_STYLE.vapourThresholdGKg) run.push(project(q));
        else flush();
      });
      flush();
    }
  if (want.has("tracers"))
    for (const streak of marks.streaks) {
      if (streak.opacity <= 0.02) continue;
      const points = streak.points.map(project);
      const layer = `FLOW-TRACERS-${flowClassOf(streak.excessK, paint.excess)}`;
      put(layer, points, false);
      put(layer, circlePolyline(points[points.length - 1]!, metres(FLOW_STYLE.tracerHeadMm)).points, true);
    }
  if (want.has("pathlines")) for (const trail of marks.trails) put(`FLOW-PATHS-${flowClassOf(trail.excessK, paint.excess)}`, trail.points.map(project), false);
  const order: readonly (readonly [string, number, number])[] = [
    ...FLOW_CLASSES.map((c) => [`FLOW-STREAMS-${c}`, FLOW_CAD.aci[c], FLOW_CAD.lineweight.streams] as const),
    ...FLOW_CLASSES.map((c) => [`FLOW-ARROWS-${c}`, FLOW_CAD.aci[c], FLOW_CAD.lineweight.arrows] as const),
    ...FLOW_CLASSES.map((c) => [`FLOW-TRACERS-${c}`, FLOW_CAD.aci[c], FLOW_CAD.lineweight.tracers] as const),
    ...FLOW_CLASSES.map((c) => [`FLOW-PATHS-${c}`, FLOW_CAD.aci[c], FLOW_CAD.lineweight.paths] as const),
    ["FLOW-VAPOUR", FLOW_CAD.aci.vapour, FLOW_CAD.lineweight.vapour],
    ["FLOW-FOG", FLOW_CAD.aci.fog, FLOW_CAD.lineweight.fog],
  ];
  return order.flatMap(([name, aci, lineweight]) => {
    const polylines = lines.get(name);
    return polylines?.length ? [{ name, aci, lineweight, polylines, texts: [] }] : [];
  });
}
