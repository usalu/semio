/** 🌊️ The air flow of a model as geometry: the developed flow field of the static instant, generalised for the scale of its scene by the very marks of the drawings (`flowMarks` with `flowDetailOf(scale)`), as solids of the thermal scene.
 *
 * - A kept stream is a stream tube: one kernel `loft` per run of equal temperature-excess bin, its circular sections at points of the stream (the first and last point of a run always, the points between thinned until the polyline stays within {@link FLOW_MODEL}`.chordMm` of paper of the stream), the radius of a section half the paper width the local speed asks (`flowPaint.streamWidthMm`), so a tube is as wide as the line of the drawing at that scale. Where a stream bends tighter than its tube is wide a loft would cross itself (the kernel's `loftFault`): the tube ends there with a cap and the next one starts at the same station, like an elbow of two pipes ({@link tubesOf}).
 * - The direction of flow is a cone (`revolve`) at every arrow of the ribbon, its tip on the stream point, its axis the direction of flow, with the paper size of the arrow heads of the drawing.
 * - Vapour puffs (more than `FLOW_STYLE.vapourThresholdGKg` of vapour excess) and fog puffs (any condensate) are translucent spheres of the entrained radius of the puff (clamped to the paper sizes of the drawing): `polyhedron` icospheres of {@link FLOW_MODEL}`.puffTriangles` triangles (the finer one from a paper radius of `puffFineMm` on), because a soft cloud has no silhouette to keep round and a kernel ellipsoid at the roundness rule of the models would cost thousands of triangles per puff. A puff is one instance: its solid is the unit icosphere on a node of its own (`flow-vapour-<i>`, `flow-fog-<i>`) whose translation is the centre of the puff and whose uniform scale is its radius, so every puff of one sphere and material has the very same vertices and a file stores them once.
 * Colours are those of `flowPaint`: a material per temperature-excess bin of the streams (opaque, unlit; the speed is the width, not an opacity), per vapour bin and per fog bin (colour and opacity at the middle of the bin). Every base material is fully transparent and only the `thermal` variant binds it to its paint, so the flow shows in the thermal view and in no other.
 * Sizes: paper millimetres times the metres per paper millimetre of the scale, but the widest tube is at most {@link FLOW_MODEL}`.tubeShare` of the largest extent of the asset in radius (the arrow heads shrink with it), so at 1:200 and 1:500 the flow never buries the body.
 * Frame: a point `(x, y, z)` of the field (ahead, up, left) is `(z, y, x)` in the neutral frame of the model, after the `travel` of the figure during the developed history is taken off (a walking clip has moved on by whole cycles while its flow developed; the model shows the figure of the first instant, so the field is carried back onto it; a pose has not travelled). The solids ride on world nodes (`flow`, `flow-stream-<index of the stream in the field>`, `flow-vapour`, `flow-fog` and below the last two one node per puff), not on bones.
 * Tracers are motion and are not part of a file: the tubes, arrows and puffs are the field of one instant.
 *
 * @see ../../💨️transport/🌊️flow/🟦️.ts — `flowMarks`, `flowDetailOf`
 * @see ../../🎨️depiction/🌊️flow-view/🟦️.ts — `flowPaint`, `FLOW_STYLE`
 */
import type { Slug, Text } from "../../../🧬️schema/🟦️.ts";
import { loftFault, type LoftSection, type Node, type Solid } from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import type { DisplayRange, FlowField, FlowPuff, Vec3 } from "../../🔗️contracts/🟦️.ts";
import { FLOW_STYLE, excessBinOf, excessOfBin, flowPaint } from "../../🎨️depiction/🌊️flow-view/🟦️.ts";
import { type Theme, tx } from "../../🎨️depiction/🖍️palette/🟦️.ts";
import { flowDetailOf, flowMarks } from "../../💨️transport/🌊️flow/🟦️.ts";
import { formatNumber } from "../../📐️units/🟦️.ts";
import type { Paint } from "../🎨️looks/🟦️.ts";
import { type Triple, toNeutral } from "../🧭️frame/🟦️.ts";

/** 📏️ The rules of the flow solids: the paper deviation in millimetres a tube axis may have from its stream, the share of the largest extent of the asset the widest tube radius may be, the bins of the vapour and fog materials, the triangles of a small and of a large puff sphere (an icosahedron subdivided once and twice), the paper radius in millimetres from which a puff is large and the vertices of a tube ring and of an arrow head: the widest tube is 0.15 mm and the head 0.375 mm of paper in radius, so an octagon keeps within 0.012 and 0.029 mm of paper of its circle, within the chord the axis keeps. */
export const FLOW_MODEL = { chordMm: 0.08, tubeShare: 0.012, vapourBins: 4, fogBins: 3, puffTriangles: [80, 320], puffFineMm: 2, tubeSegments: 8 } as const;

/** 🏷️ The tag of every flow node and solid; each is a glyph too (generated, hidden by default, shown by the variant of its view). */
export const FLOW_TAG = "flow";
const FLOW_TAGS = ["glyph", FLOW_TAG] as const;

/** 🎨️ One material of the flow: its base id, title and the paint the thermal variant gives it. */
export type FlowMaterial = { readonly id: Slug; readonly title: Text; readonly paint: Paint };

/** 🧩️ The flow of one scale as model parts: world nodes (the root `flow` first), solids and the materials they use, in order of first use. */
export type FlowParts = { readonly nodes: Node[]; readonly solids: Solid[]; readonly materials: FlowMaterial[] };

/** 📐️ The metres per paper millimetre of the tubes and arrow heads at a scale: that of the scale, capped so the widest tube keeps to its share of the largest extent `extentM` of the asset. */
export const flowUnitOf = (scale: number, extentM: number): number => Math.min(scale / 1000, (FLOW_MODEL.tubeShare * extentM) / (FLOW_STYLE.streamMaxMm / 2));

const distanceToSegment = (p: Vec3, a: Vec3, b: Vec3): number => {
  const ab = { x: b.x - a.x, y: b.y - a.y, z: b.z - a.z };
  const l2 = ab.x * ab.x + ab.y * ab.y + ab.z * ab.z;
  const t = l2 === 0 ? 0 : Math.min(1, Math.max(0, ((p.x - a.x) * ab.x + (p.y - a.y) * ab.y + (p.z - a.z) * ab.z) / l2));
  return Math.sqrt((p.x - a.x - t * ab.x) ** 2 + (p.y - a.y - t * ab.y) ** 2 + (p.z - a.z - t * ab.z) ** 2);
};

/** ✂️ The indices of the points of `points[from … to]` a tube keeps (Ramer–Douglas–Peucker): both ends, and between two kept points the farthest point while it is farther than `toleranceM` from their chord. */
export function tubeStations(points: readonly Vec3[], from: number, to: number, toleranceM: number): number[] {
  const keep = new Set([from, to]);
  const stack: [number, number][] = [[from, to]];
  while (stack.length > 0) {
    const [a, b] = stack.pop()!;
    let worst = -1;
    let reach = toleranceM;
    for (let i = a + 1; i < b; i++) {
      const d = distanceToSegment(points[i]!, points[a]!, points[b]!);
      if (d > reach) {
        reach = d;
        worst = i;
      }
    }
    if (worst < 0) continue;
    keep.add(worst);
    stack.push([a, worst], [worst, b]);
  }
  return [...keep].sort((x, y) => x - y);
}

/** 🪈️ The stations of a tube as the lofts that do not fold: cut at the inner station of the first fold, both pieces sharing it, until no piece folds (two stations never do). */
export function tubesOf<T extends LoftSection>(sections: readonly T[]): (readonly T[])[] {
  const fold = loftFault(sections);
  if (fold === null) return [sections];
  const cut = fold[1] < sections.length - 1 ? fold[1] : fold[0];
  return [...tubesOf(sections.slice(0, cut + 1)), ...tubesOf(sections.slice(cut))];
}

function icosphere(levels: number): { readonly vertices: readonly Triple[]; readonly triangles: readonly (readonly [number, number, number])[] } {
  const t = (1 + Math.sqrt(5)) / 2;
  const unit = (v: Triple): Triple => {
    const l = Math.sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
    return [v[0] / l, v[1] / l, v[2] / l];
  };
  const vertices: Triple[] = ([[-1, t, 0], [1, t, 0], [-1, -t, 0], [1, -t, 0], [0, -1, t], [0, 1, t], [0, -1, -t], [0, 1, -t], [t, 0, -1], [t, 0, 1], [-t, 0, -1], [-t, 0, 1]] as Triple[]).map(unit);
  let triangles: [number, number, number][] = [[0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11], [1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8], [3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9], [4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1]];
  for (let level = 0; level < levels; level++) {
    const middle = new Map<string, number>();
    const mid = (a: number, b: number): number => {
      const key = a < b ? `${a}:${b}` : `${b}:${a}`;
      let index = middle.get(key);
      if (index === undefined) {
        const p = vertices[a]!, q = vertices[b]!;
        index = vertices.push(unit([(p[0] + q[0]) / 2, (p[1] + q[1]) / 2, (p[2] + q[2]) / 2])) - 1;
        middle.set(key, index);
      }
      return index;
    };
    triangles = triangles.flatMap(([a, b, c]) => {
      const ab = mid(a, b), bc = mid(b, c), ca = mid(c, a);
      return [[a, ab, ca], [b, bc, ab], [c, ca, bc], [ab, bc, ca]] as [number, number, number][];
    });
  }
  return { vertices, triangles };
}

const SPHERES = [icosphere(1), icosphere(2)] as const;

const flowNode = (id: Slug, parent: Slug | null): Node => ({ id, parent, translation: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1], tags: FLOW_TAGS });

/** 🏭️ The flow parts of one scale: `flow` the developed field, `scale` the denominator of the scene, `extentM` the largest extent of the asset, `theme` and `range` those of the thermal view, `travel` what the figure moved during the developed history (world metres, default none). A field without a kept stream or puff gives no part at all. */
export function flowParts(flow: FlowField, scale: number, extentM: number, theme: Theme, range: DisplayRange, travel: Vec3 = { x: 0, y: 0, z: 0 }): FlowParts {
  const mm = scale / 1000;
  const unit = flowUnitOf(scale, extentM);
  const marks = flowMarks(flow, flow.timeS, flowDetailOf(scale));
  const paint = flowPaint(theme, flow, range);
  const nodes: Node[] = [];
  const solids: Solid[] = [];
  const materials = new Map<Slug, FlowMaterial>();
  const n = (value: number, digits: number, language: "en" | "de"): string => formatNumber(value, digits, language);
  const excessMaterial = (bin: number): Slug => {
    const id = `flow-excess-${bin}`;
    const k = excessOfBin(paint, bin);
    if (!materials.has(id)) materials.set(id, { id, title: tx(`Air flow, temperature excess ${n(k, 1, "en")} K`, `Luftströmung, Übertemperatur ${n(k, 1, "de")} K`), paint: { hex: paint.excessColour(k), alpha: 1 } });
    return id;
  };
  const binned = (kind: "vapour" | "fog", gKg: number): Slug => {
    const [bins, max] = kind === "vapour" ? [FLOW_MODEL.vapourBins, paint.vapourMaxGKg] : [FLOW_MODEL.fogBins, paint.fogMaxGKg];
    const bin = Math.min(bins - 1, Math.max(0, Math.floor((bins * gKg) / max)));
    const id = `flow-${kind}-${bin}`;
    const g = ((bin + 0.5) * max) / bins;
    if (!materials.has(id))
      materials.set(id, kind === "vapour" ? { id, title: tx(`Water vapour excess ${n(g, 1, "en")} g/kg`, `Wasserdampfüberschuss ${n(g, 1, "de")} g/kg`), paint: { hex: paint.vapourColour(g), alpha: paint.vapourOpacity(g) } } : { id, title: tx(`Fog, condensed water ${n(g, 2, "en")} g/kg`, `Nebel, kondensiertes Wasser ${n(g, 2, "de")} g/kg`), paint: { hex: paint.fogColour, alpha: paint.fogOpacity(g) } });
    return id;
  };
  const neutral = (p: Vec3): Triple => toNeutral([p.x, p.y, p.z]);
  const placed = (p: Vec3): Triple => toNeutral([p.x - travel.x, p.y - travel.y, p.z - travel.z]);

  for (const ribbon of marks.ribbons) {
    const node = `flow-stream-${ribbon.stream}`;
    const points = ribbon.points;
    const bins = ribbon.excessK.map((k) => excessBinOf(paint, k));
    const made: Solid[] = [];
    let start = 0;
    for (let i = 1; i <= points.length; i++) {
      if (i < points.length && bins[i] === bins[start]) continue;
      const end = Math.min(points.length - 1, i);
      if (end > start) {
        const stations = tubeStations(points, start, end, FLOW_MODEL.chordMm * mm).map((index) => {
          const radius = (paint.streamWidthMm(ribbon.speedMS[index]!) / 2) * unit;
          return { index, at: placed(points[index]!), rx: radius, ry: radius };
        });
        for (const tube of tubesOf(stations)) made.push({ id: `${node}-${tube[0]!.index}`, node, material: excessMaterial(bins[start]!), tags: FLOW_TAGS, kind: "loft", sections: tube.map(({ at, rx, ry }) => ({ at, rx, ry })), capStart: true, capEnd: true, segments: FLOW_MODEL.tubeSegments });
      }
      start = end;
      if (i >= points.length - 1) break;
    }
    ribbon.arrows.forEach((arrow, j) => {
      const length = FLOW_STYLE.headLengthMm * unit;
      made.push({ id: `flow-arrow-${ribbon.stream}-${j}`, node, material: excessMaterial(bins[Math.max(0, points.indexOf(arrow.at))]!), tags: FLOW_TAGS, kind: "revolve", profile: [[-length, 0], [-length, (FLOW_STYLE.headWidthMm / 2) * unit], [0, 0]], axis: neutral(arrow.direction), origin: placed(arrow.at), segments: FLOW_MODEL.tubeSegments });
    });
    if (made.length === 0) continue;
    nodes.push(flowNode(node, "flow"));
    solids.push(...made);
  }

  const sphere = (kind: "vapour" | "fog", i: number, material: Slug, puff: FlowPuff): void => {
    const id = `flow-${kind}-${i}`;
    const radius = Math.min(FLOW_STYLE.puffMaxMm * mm, Math.max(FLOW_STYLE.puffMinMm * mm, puff.radiusM));
    const mesh = SPHERES[radius < FLOW_MODEL.puffFineMm * mm ? 0 : 1];
    nodes.push({ id, parent: `flow-${kind}`, translation: placed(puff.at), rotation: [0, 0, 0, 1], scale: [radius, radius, radius], tags: FLOW_TAGS });
    solids.push({ id, node: id, material, tags: FLOW_TAGS, kind: "polyhedron", vertices: mesh.vertices, triangles: mesh.triangles });
  };
  const vapour = marks.puffs.filter((p) => p.vapourExcessKgPerKg * 1000 > FLOW_STYLE.vapourThresholdGKg);
  if (vapour.length > 0) nodes.push(flowNode("flow-vapour", "flow"));
  vapour.forEach((p, i) => sphere("vapour", i, binned("vapour", p.vapourExcessKgPerKg * 1000), p));
  const fog = marks.puffs.filter((p) => p.condensateKgPerKg > 0);
  if (fog.length > 0) nodes.push(flowNode("flow-fog", "flow"));
  fog.forEach((p, i) => sphere("fog", i, binned("fog", p.condensateKgPerKg * 1000), p));

  return solids.length === 0 ? { nodes: [], solids: [], materials: [] } : { nodes: [flowNode("flow", null), ...nodes], solids, materials: [...materials.values()] };
}
