/** 🧊️ The axonometric depiction: the 3D volume of an evaluated state drawn as an orthographic axonometric view in the seven representations, as clean vector items of a {@link DrawingScene} (no raster).
 *
 * Pipeline, one source for every step: the bone frames of `evaluate` pose the tessellated solids of the volume (./🧱️mesh, the kernel tessellator at a chord tolerance of 0.04 paper millimetres, the same glyph placements as `modelOf` in ../../🧊️volume/🧷️frames); the camera is the fourth camera of ../🔭️views (isometric, ISO 5456-3 dimetric or free, the vertical axis true to scale);
 * exact hidden-surface removal (./📐️visibility) leaves, per solid, the visible faces as convex polygons that ./🖇️stitch merges per solid and shading band into one clean outline path; the visible silhouette, crease and interpenetration lines come from ./🧭️edges and the same exact hidden-line removal and are stroked with the pens of the authored representation of the scale.
 * Level of detail is the authored `maxDenominator` of the solids and the role pens of the representation (a role whose pen is `null` at that scale is not drawn), nothing else. The seven views use the colours, ramps, hatch patterns, values and legends of the planar views (`baseStyle`, `thermalView`, `memberView`); the arrows, arcs, centre of mass and emitter cones of the analytic views are the 3D glyph solids of the model.
 *
 * What is to scale: see {@link axonometricCamera}. Deterministic in `(state, options, analysis)`. Expensive: progress and cancellation go through the checkpoints of the generator ({@link Checkpoint}), driven by `depict(..., control)` or `depictAsync`.
 *
 * @see ../🔭️views/🟦️.ts — the camera and its scale
 * @see ./📐️visibility/🟦️.ts — hidden-surface and hidden-line removal
 */
import type { ShapeRole, Slug, Text } from "../../../🧬️schema/🟦️.ts";
import type { Shape } from "../../../🧬️schema/🟦️.ts";
import type { EmitterState, Fill, Legend, StructuralResult, ThermalField, LayerId, PathCommand } from "../../🔗️contracts/🟦️.ts";
import { formatNumber } from "../../📐️units/🟦️.ts";
import { type Analytic, Slots, frameDataOf, glyphPaintOf } from "../../🧊️volume/🧷️frames/🟦️.ts";
import { lineLook, solidLook } from "../../🧊️volume/🎨️looks/🟦️.ts";
import { memberLook } from "../../🧊️volume/🏗️structure/🟦️.ts";
import { thermalLook } from "../../🧊️volume/🌡️thermal/🟦️.ts";
import { eligibleBones, forceList, longestArrowMm, memberView, momentList } from "../🏗️structural-view/🟦️.ts";
import { magnitude } from "../../🏗️structure/📽️projection/🟦️.ts";
import { type Canvas, type Ctx } from "../🧱️canvas/🟦️.ts";
import { commandsOf } from "../📐️geometry/🟦️.ts";
import { legendOf, overlayColours, tx } from "../🖍️palette/🟦️.ts";
import { type AnalyticView, type Prepared, type Source, type Style, baseStyle } from "../🧩️shapes/🟦️.ts";
import { thermalView } from "../🌡️thermal-view/🟦️.ts";
import { ViewMissingError, type AxonometricCamera } from "../🔭️views/🟦️.ts";
import type { Checkpoint } from "../⏳️steps/🟦️.ts";
import { type Soup, type Part, poseSoup, surfacesAt } from "./🧱️mesh/🟦️.ts";
import { type Candidate, edgesOf, joinsOf } from "./🧭️edges/🟦️.ts";
import { type Path, chainOf, unionOf } from "./🖇️stitch/🟦️.ts";
import { Visibility, toleranceOf } from "./📐️visibility/🟦️.ts";
import { bandOf, shaded } from "./🌗️shading/🟦️.ts";

export { qualityAt, CHORD_PAPER_MM } from "./🧱️mesh/🟦️.ts";
export { CREASE_RAD } from "./🧭️edges/🟦️.ts";
export { Visibility, toleranceOf } from "./📐️visibility/🟦️.ts";
export { BANDS, LIGHT, SHADE_AMOUNTS, SHADOW, bandOf, shaded } from "./🌗️shading/🟦️.ts";


const PERMILLE = 1000;
const LABELS = {
  pose: tx("Posing the volume", "Volumen wird positioniert"),
  surfaces: tx("Removing hidden surfaces", "Verdeckte Flächen werden entfernt"),
  lines: tx("Finding visible lines", "Sichtbare Linien werden gesucht"),
  paint: tx("Drawing", "Zeichne"),
} as const;
const STEP = 400;
const QUANTUM = 1e-7;
const SEAM_MM = 0.05;
const GLYPH_PEN_MM = 0.18;

/** 📐️ What the depiction of a figure leaves for the shared annotations of `depict`: the camera, the legends of the analytic view, the true height of the posed solids and the number of visible solids. */
export type AxonometricFigure = { readonly camera: AxonometricCamera; readonly legends: readonly Legend[]; readonly heightM: number; readonly visibleSolids: number };

/** 🧮️ The analyses the analytic views read (the emitters belong to the thermal view). */
export type Analysis3d = { readonly thermal?: ThermalField; readonly structure?: StructuralResult; readonly emitters?: readonly EmitterState[] };

/** ⚙️ How the figure is drawn besides the context: whether glyph bodies get outlines (for CAD scenes) and the overhang of the ground outline in metres (`null` draws no ground). */
export type FigureOptions = { readonly glyphEdges?: boolean; readonly groundOverhangM: number | null };

const SHAPE_STUB = { kind: "polygon", points: [] } as const;

const layerOf = (role: ShapeRole): LayerId => role;

function arrowLegend(ctx: Ctx, max: number, longestMm: number): Legend {
  return legendOf({
    title: tx("Force arrows", "Kraftpfeile"),
    quantity: "force",
    unit: "N",
    scale: { min: 0, centre: 0, max },
    colourAt: () => overlayColours(ctx.theme).applied,
    convention: tx(
      "Three-dimensional arrows with the head on the point of application, world axes x ahead, y up, z left. Red = applied load, teal = reaction, grey = weight, orange = inertia; the centre of mass is the dark sphere. An arrow is foreshortened by the projection like any other line.",
      "Dreidimensionale Pfeile mit der Spitze im Angriffspunkt, Weltachsen x voraus, y oben, z links. Rot = äußere Last, Türkis = Reaktion, Grau = Gewicht, Orange = Trägheit; der Massenschwerpunkt ist die dunkle Kugel. Ein Pfeil wird wie jede Linie von der Projektion verkürzt.",
    ),
    interpretation: tx(`Arrow length is proportional to |F| (minimum 1.5 mm of paper): ${formatNumber(longestMm, 0, "en")} mm ≙ ${formatNumber(max, 1, "en")} N before the projection.`, `Die Pfeillänge ist proportional zu |F| (mindestens 1,5 mm Papier): ${formatNumber(longestMm, 0, "de")} mm ≙ ${formatNumber(max, 1, "de")} N vor der Projektion.`),
  });
}

function arcLegend(ctx: Ctx, max: number): Legend {
  return legendOf({
    title: tx("Moment arcs", "Momentbögen"),
    quantity: "moment",
    unit: "N·m",
    scale: { min: 0, centre: 0, max },
    colourAt: () => overlayColours(ctx.theme).joint,
    convention: tx(
      "A 270 degree arc with an arrow head about the axis of the moment vector M = r × F, in the right-handed sense of the picture's frame; orange = joint moment, teal = reaction, red = applied couple.",
      "Ein 270-Grad-Bogen mit Pfeilspitze um die Achse des Momentvektors M = r × F im Rechtssystem des Bildes; Orange = Gelenkmoment, Türkis = Reaktion, Rot = äußeres Kräftepaar.",
    ),
    interpretation: tx(`The arc radius grows with √|M| from 1.6 mm to 4.8 mm of paper (4.8 mm ≙ ${formatNumber(max, 1, "en")} N·m).`, `Der Bogenradius wächst mit √|M| von 1,6 mm auf 4,8 mm Papier (4,8 mm ≙ ${formatNumber(max, 1, "de")} N·m).`),
  });
}

type Segment = readonly [number, number, number, number];

/** 🖌️ Draws the figure of the context's state into the canvas: the fills per solid, the visible lines and the ground outline. A generator that yields a {@link Checkpoint} between its steps (every few hundred triangles). Throws `ViewMissingError` for an asset without volume. */
export function* drawAxonometric(canvas: Canvas, ctx: Ctx, source: Source, analytic: Analysis3d, figure: FigureOptions): Generator<Checkpoint, AxonometricFigure> {
  const { state, options, k, theme, colours } = ctx;
  const asset = state.asset;
  const camera = ctx.camera;
  if (camera === undefined) throw new Error("an axonometric depiction needs ctx.camera");
  if (asset.volume === undefined) throw new ViewMissingError(asset.id, "axonometric");
  const view = options.view;
  const denominator = options.scale.denominator;
  const mm = denominator / 1000;
  const heightM = asset.dimensions.heightM;
  const surfaces = surfacesAt(asset, denominator);
  const slots = new Slots();
  const eligible = eligibleBones(asset, heightM);
  const range = options.range;
  const fixed = view === "dark-solid" || view === "light-solid" ? { [view]: solidLook(view === "dark-solid" ? "dark" : "light", view, surfaces) } : view === "dark-line" || view === "light-line" ? { [view]: lineLook(view === "dark-line" ? "dark" : "light", view) } : {};
  const frame = frameDataOf(
    {
      asset,
      views: [view],
      theme,
      mm,
      longestMm: longestArrowMm(heightM / mm),
      looks: {
        thermal: view === "thermal" ? thermalLook(asset, { quantity: options.thermalQuantity, range, theme }, surfaces) : undefined,
        forces: view === "forces" ? memberLook(asset, { view: "forces", quantity: options.forceQuantity, range, theme, eligible }, surfaces) : undefined,
        moments: view === "moments" ? memberLook(asset, { view: "moments", quantity: "moment", range, theme, eligible }, surfaces) : undefined,
        fixed,
      },
      slots,
    },
    state,
    analytic as Analytic,
  );

  yield { done: 0, total: PERMILLE, label: LABELS.pose };
  const soup: Soup = poseSoup(asset, frame, slots.map, denominator);
  const parts = soup.parts;
  const pens = source.pens;
  const drawn = parts.map((p) => p.kind === "glyph" || pens[p.role] !== null);
  const glazing = parts.map((p) => p.kind === "solid" && p.role === "glazing");

  const nrm = camera.normalisation;
  const { right, up, toward } = camera;
  const count = soup.vertices.length / 3;
  const vertices = new Float64Array(soup.vertices.length);
  let ox = Infinity, oy = Infinity, oz = Infinity, px = -Infinity, py = -Infinity, pz = -Infinity;
  for (let i = 0; i < count; i++) {
    const X = soup.vertices[3 * i]!, Y = soup.vertices[3 * i + 1]!, Z = soup.vertices[3 * i + 2]!;
    const x = nrm * (right.x * Z + right.y * Y + right.z * X);
    const y = nrm * (up.x * Z + up.y * Y + up.z * X);
    const z = nrm * (toward.x * Z + toward.y * Y + toward.z * X);
    vertices[3 * i] = x; vertices[3 * i + 1] = y; vertices[3 * i + 2] = z;
    ox = Math.min(ox, x); oy = Math.min(oy, y); oz = Math.min(oz, z); px = Math.max(px, x); py = Math.max(py, y); pz = Math.max(pz, z);
  }
  const extent = count === 0 ? 1 : Math.max(px - ox, py - oy, pz - oz);
  const triangleCount = soup.indices.length / 3;
  const opaque = new Uint8Array(triangleCount);
  for (let t = 0; t < triangleCount; t++) {
    const part = soup.part[t]!;
    opaque[t] = drawn[part] === true && !glazing[part] ? 1 : 0;
  }
  const vis = new Visibility({ vertices, indices: soup.indices, opaque }, toleranceOf(extent));
  const toPaper = (x: number, y: number): [number, number] => [x * k, -y * k];

  let bottom = Infinity, top = -Infinity;
  for (const p of parts) {
    if (p.kind !== "solid" || !drawn[parts.indexOf(p)]) continue;
    for (let t = p.first; t < p.first + p.count; t++) for (let c = 0; c < 3; c++) {
      const y = soup.vertices[3 * soup.indices[3 * t + c]! + 1]!;
      bottom = Math.min(bottom, y);
      top = Math.max(top, y);
    }
  }

  const banded = view.endsWith("-solid") && options.shading !== "flat";
  const pieces: Map<number, Map<number, number[][]>> = new Map();
  for (let start = 0; start < triangleCount; start += STEP) {
    yield { done: Math.round(100 + (500 * start) / Math.max(1, triangleCount)), total: PERMILLE, label: LABELS.surfaces };
    for (let t = start; t < Math.min(triangleCount, start + STEP); t++) {
      const part = soup.part[t]!;
      if (!drawn[part] || !vis.isLive(t)) continue;
      const fragments = vis.fragments(t);
      if (fragments.length === 0) continue;
      let band = BANDS_TOP;
      if (banded && parts[part]!.kind === "solid" && !glazing[part]) {
        const a = soup.indices[3 * t]! * 3, b = soup.indices[3 * t + 1]! * 3, c = soup.indices[3 * t + 2]! * 3;
        const ux = vertices[b]! - vertices[a]!, uy = vertices[b + 1]! - vertices[a + 1]!, uz = vertices[b + 2]! - vertices[a + 2]!, wx = vertices[c]! - vertices[a]!, wy = vertices[c + 1]! - vertices[a + 1]!, wz = vertices[c + 2]! - vertices[a + 2]!;
        band = bandOf(uy * wz - uz * wy, uz * wx - ux * wz, ux * wy - uy * wx);
      }
      const byBand = pieces.get(part) ?? pieces.set(part, new Map()).get(part)!;
      const list = byBand.get(band) ?? byBand.set(band, []).get(band)!;
      list.push(...fragments);
    }
  }

  const prepared = new Map<number, Prepared>();
  const parents = new Map(asset.rig.bones.map((b) => [b.id, b.parent]));
  [...pieces.keys()].sort((a, b) => a - b).forEach((part, order) => {
    const p = parts[part]!;
    if (p.kind !== "solid") return;
    const regions = p.regions;
    const shape = { ...SHAPE_STUB, id: p.id, bone: p.bone!, role: p.role, z: order, filled: true, tone: p.tone, ...(regions.length === 1 ? { region: regions[0]! } : regions.length > 1 ? { regions } : {}) } as unknown as Shape;
    prepared.set(part, { shape, rings: [], closed: true, box: { minX: 0, minY: 0, maxX: 0, maxY: 0 }, order, occluder: null, weightMm: pens[p.role] ?? 0, hatch: null, parent: parents.get(p.bone!) ?? null });
  });
  const visibleSolids = [...prepared.values()];

  const legends: Legend[] = [];
  let analyticView: AnalyticView | null = null;
  if (view === "thermal") {
    const field = analytic.thermal;
    if (field === undefined) throw new Error("the thermal view needs a solved thermal field");
    analyticView = thermalView(ctx, visibleSolids, field);
    legends.push(...analyticView.legends);
  } else if (view === "forces" || view === "moments") {
    const result = analytic.structure;
    if (result === undefined) throw new Error(`the ${view} view needs a solved structural result`);
    const members = memberView(ctx, visibleSolids, result, view === "forces" ? options.forceQuantity : "moment", options.patterns);
    analyticView = { styleOf: members.styleOf, overlay: () => {}, legends: [members.legend] };
    legends.push(members.legend);
    const longestMm = longestArrowMm(heightM / mm);
    if (view === "forces") {
      const max = Math.max(0, ...forceList(state, result).map((f) => magnitude(f.force)));
      if (max > 0) legends.push(arrowLegend(ctx, max, longestMm));
    } else {
      const max = Math.max(0, ...momentList(state, result).map((a) => magnitude(a.m)));
      if (max > 0) legends.push(arcLegend(ctx, max));
    }
  }
  const styleOf: (p: Prepared) => Style = analyticView ? analyticView.styleOf : baseStyle(view, theme, denominator <= 100);

  yield { done: 600, total: PERMILLE, label: LABELS.paint };
  const order = [...pieces.keys()].sort((a, b) => Number(glazing[a]) - Number(glazing[b]) || a - b);
  const slotOf = (p: Part): { readonly kind: "arrow" | "arc" | "mass" | "emitter"; readonly material: Slug; readonly detail: string } | undefined => [...slots.map.values()].find((s) => p.material === s.material);
  for (const part of order) {
    const p = parts[part]!;
    const byBand = pieces.get(part)!;
    let base: Fill | null;
    let value: Style["value"];
    if (p.kind === "solid") {
      const style = styleOf(prepared.get(part)!);
      base = style.fill;
      value = style.value;
    } else {
      const slot = slotOf(p);
      const paint = slot === undefined ? undefined : glyphPaintOf(slot, theme, frame.glyphPaints);
      base = paint === undefined ? null : { color: paint.hex, ...(paint.alpha < 1 ? { opacity: paint.alpha } : {}) };
      value = undefined;
    }
    if (base === null) continue;
    for (const band of [...byBand.keys()].sort((a, b) => a - b)) {
      const rings = unionOf(byBand.get(band)!, QUANTUM);
      if (rings.length === 0) continue;
      const fill: Fill = banded && p.kind === "solid" && !glazing[part] ? { ...base, color: shaded(base.color, band) } : base;
      const commands: PathCommand[] = rings.flatMap((ring) => commandsOf(pointsOf(ring, toPaper), true));
      const seam = view.endsWith("-solid") && fill.opacity === undefined && fill.pattern === undefined;
      canvas.path("fill", banded && byBand.size > 1 ? `shape:${p.id}:band-${band}` : `shape:${p.id}`, commands, {
        fill,
        ...(seam ? { stroke: { color: fill.color, widthMm: SEAM_MM, cap: "round" as const, join: "round" as const } } : {}),
        ...(value ? { value } : {}),
        ...(p.bone === null ? {} : { bone: p.bone }),
        ...(p.regions.length === 1 ? { region: p.regions[0]! } : {}),
      });
    }
  }

  yield { done: 600, total: PERMILLE, label: LABELS.lines };
  const multiplier = view.endsWith("-line") ? 1 : view.endsWith("-solid") ? (denominator <= 100 ? 0.5 : 0) : 0.5;
  const include = (part: number): boolean => drawn[part] === true && (parts[part]!.kind === "solid" || figure.glyphEdges === true);
  const candidates: Candidate[] = multiplier === 0 ? [] : [...edgesOf({ vertices, indices: soup.indices, part: soup.part }, include), ...joinsOf({ vertices, indices: soup.indices, part: soup.part }, include, parts.length)];
  const groups = new Map<string, { id: string; layer: LayerId; widthMm: number; segments: Segment[]; bone: Slug | null }>();
  const penOf = (part: number): number => (parts[part]!.kind === "glyph" ? GLYPH_PEN_MM : (pens[parts[part]!.role] ?? 0));
  const group = (key: string, id: string, layer: LayerId, widthMm: number, bone: Slug | null): Segment[] => (groups.get(key) ?? groups.set(key, { id, layer, widthMm, segments: [], bone }).get(key)!).segments;
  for (let i = 0; i < candidates.length; i++) {
    if (i % STEP === 0) yield { done: Math.round(600 + (300 * i) / Math.max(1, candidates.length)), total: PERMILLE, label: LABELS.lines };
    const c = candidates[i]!;
    const intervals = vis.visible(c.a, c.b, new Set(c.triangles));
    if (intervals.length === 0) continue;
    let owner = c.part;
    if (c.kind === "join") owner = penOf(c.part) <= penOf(c.other) ? c.part : c.other;
    const p = parts[owner]!;
    const key = c.kind === "join" ? `join:${parts[c.part]!.id}:${parts[c.other]!.id}` : `edge:${p.id}`;
    const target = group(key, key, p.kind === "glyph" ? "forces" : layerOf(p.role), penOf(owner) * multiplier, p.bone);
    for (const [t0, t1] of intervals) target.push([c.a[0] + (c.b[0] - c.a[0]) * t0, c.a[1] + (c.b[1] - c.a[1]) * t0, c.a[0] + (c.b[0] - c.a[0]) * t1, c.a[1] + (c.b[1] - c.a[1]) * t1]);
  }
  for (const key of [...groups.keys()].sort()) {
    const g = groups.get(key)!;
    if (!(g.widthMm > 0)) continue;
    const commands = chainOf(g.segments, QUANTUM).flatMap((path) => commandsOf(pointsOf(path, toPaper), path.closed));
    if (commands.length === 0) continue;
    canvas.path(g.layer, g.id, commands, { stroke: { color: colours.ink, widthMm: g.widthMm, cap: "round", join: "round" }, ...(g.bone === null ? {} : { bone: g.bone }) });
  }

  if (figure.groundOverhangM !== null && count > 0) {
    let x0 = Infinity, x1 = -Infinity, z0 = Infinity, z1 = -Infinity;
    parts.forEach((p, index) => {
      if (p.kind !== "solid" || !drawn[index]) return;
      for (let t = p.first; t < p.first + p.count; t++) for (let c = 0; c < 3; c++) {
        const v = soup.indices[3 * t + c]!;
        x0 = Math.min(x0, soup.vertices[3 * v]!); x1 = Math.max(x1, soup.vertices[3 * v]!);
        z0 = Math.min(z0, soup.vertices[3 * v + 2]!); z1 = Math.max(z1, soup.vertices[3 * v + 2]!);
      }
    });
    if (Number.isFinite(x0)) {
      const o = figure.groundOverhangM;
      const corner = (X: number, Z: number): [number, number, number] => [nrm * (right.x * Z + right.z * X), nrm * (up.x * Z + up.z * X), nrm * (toward.x * Z + toward.z * X)];
      const ring = [corner(x0 - o, z0 - o), corner(x1 + o, z0 - o), corner(x1 + o, z1 + o), corner(x0 - o, z1 + o)];
      const segments: Segment[] = [];
      for (let e = 0; e < 4; e++) {
        const a = ring[e]!, b = ring[(e + 1) % 4]!;
        for (const [t0, t1] of vis.visible(a, b)) segments.push([a[0] + (b[0] - a[0]) * t0, a[1] + (b[1] - a[1]) * t0, a[0] + (b[0] - a[0]) * t1, a[1] + (b[1] - a[1]) * t1]);
      }
      const commands = chainOf(segments, QUANTUM).flatMap((path) => commandsOf(pointsOf(path, toPaper), path.closed));
      if (commands.length > 0) canvas.path("ground", "ground:outline", commands, { stroke: { color: colours.ink, widthMm: pens.ground, cap: "butt" } });
    }
  }
  yield { done: PERMILLE, total: PERMILLE, label: LABELS.paint };
  return { camera, legends, heightM: Number.isFinite(top) ? top - bottom : 0, visibleSolids: visibleSolids.length };
}

const BANDS_TOP = 3;

function pointsOf(path: Path, toPaper: (x: number, y: number) => [number, number]): { x: number; y: number }[] {
  const out: { x: number; y: number }[] = [];
  for (let i = 0; i < path.points.length; i += 2) {
    const [x, y] = toPaper(path.points[i]!, path.points[i + 1]!);
    out.push({ x, y });
  }
  return out;
}
