/** 🧊️ `checkVolumes`: the rules of the per-scale, per-mode volumes of an asset with the measurements behind them, for the catalog check and the authoring tools.
 *
 * Analytic rules (availability, bones, feature, presence of thermal and structural solids, the per-solid `volume-*` rules) come from {@link ../🧱️rules/🟦️.ts} and `../🧊️volume/🟦️.ts`; the geometric rules tessellate with the owned kernel and
 * measure with the owned raster ({@link ../🌓️silhouette/🟦️.ts}), the owned column union and the owned ray parity: `volumes-silhouette` (IoU and Hausdorff of the body against the filled 2D shapes of the same scale in every viewpoint),
 * `volumes-bounds`, `volumes-monotonic` (expanded solids and triangles per list), `volumes-cross-scale` (IoU, Hausdorff and union volume against the finer authored scale), the area and box part of `volumes-thermal-coverage`
 * and the containment and ground part of `volumes-structure-coverage`. A thermal solid that is or references an `envelope: true` solid gives its region coverage but stands for a porous lattice, so its hull surface is left out of the exposed-area sums (C22); a solid tagged `emitter` (a plume, a breath) gives coverage but is no surface either, so it is left out of the sums too, and only its centre has to lie in the body box. Every limit is in {@link VOLUME_LIMITS}. Expensive work reports progress and stops on cancellation.
 *
 * THE REFERENCES are shapely and numpy (areas, silhouettes, Hausdorff) and trimesh (volumes) in `🧪️tests/🧊️volumes-rules`.
 *
 * @see ../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/design.md
 * @see ../🧊️volume/🟦️.ts — the per-solid rules */
import type { Asset, AuthoredVolume, PrintProfile, ScaleDenominator, Slug, Text, VolumeList } from "../../../🧬️schema/🟦️.ts";
import type { JobControl } from "../../🔗️contracts/🟦️.ts";
import { evaluate } from "../../🧮️evaluation/🟦️.ts";
import { type Issue } from "../🟦️.ts";
import { volumeIssues } from "../🧊️volume/🟦️.ts";
import { authoredVolumes, expandedCount, solidsOf } from "./🧱️lists/🟦️.ts";
import { VOLUME_LIMITS, VOLUME_RULES, analyticRules, availabilityRule, bonesRule, collector, compareSolidLists, copiedRule, featureOf, featureRule, slenderRule, structureCoverageRule, thermalCoverageRule, type RuleResult, type VolumeFinding, type VolumeMeasurement, type VolumeRule } from "./🧱️rules/🟦️.ts";
import { SEAM_FEATURES, bandVolumes, exposedArea, fixedTriangleCount, outerArea, unionVolume, type ContactPlane } from "./🧮️measures/🟦️.ts";
import { GATE, Mask, drawnFor, TILE_CELLS, WINDOW_FEATURES, VIEWPOINTS, boundarySpread, distanceField, extentOf, nearWithin, hausdorff, measureAsset, project, quantileOf, referenceOf, worldSolids, type Drawn, type Group, type Point, type Viewpoint, type World } from "./🌓️silhouette/🟦️.ts";

export { VOLUME_LIMITS, VOLUME_RULES, analyticRules, compareSolidLists, featureOf };
export { bandVolumes, exposedArea, fixedTriangleCount, isClosed, outerArea, unionVolume } from "./🧮️measures/🟦️.ts";
export type { ContactPlane } from "./🧮️measures/🟦️.ts";
export type { VolumeFinding, VolumeMeasurement, VolumeRule };
export type { SolidSimilarity } from "./🧱️rules/🟦️.ts";
export { authoredVolumes, expandedCount, solidsOf } from "./🧱️lists/🟦️.ts";

/** 🎛️ What a volume check is asked: `scale` (only that scale; for the pair rules the pair ending at it), `rules`, the `profile` that sets the smallest feature, the `pose` (default `presentation.defaultPose`). */
export type VolumeOptions = { readonly scale?: ScaleDenominator; readonly rules?: readonly VolumeRule[]; readonly profile?: Pick<PrintProfile, "minFeatureMm">; readonly pose?: Slug };

const PHASE = (en: string, de: string): Text => ({ en, de });
type Vec = readonly [number, number, number];
const world = (asset: Asset, bones: unknown, scale: number, list: VolumeList): readonly World[] => worldSolids(asset, bones as never, scale, list);

/** 📐️ What two silhouettes agree on: the IoU (with the forgiveness of `toleranceM` when given), the raw IoU, the 95th percentile and the maximum of the boundary distances in both directions, in metres, to the precision of one cell. */
export type Agreement = { readonly iou: number; readonly rawIou: number; readonly p95M: number; readonly hausdorffM: number; readonly cellM: number };

/** 🪞️ Compares two sets of filled regions (groups of rings) on a raster whose cell is a quarter of `featureM`, tiled for large objects so that no fixed pixel count limits the resolution; with `toleranceM` the mismatch cells within that distance of the other region do not count against the IoU. */
export function compareGroups(ga: readonly Group[], gb: readonly Group[], featureM: number, toleranceM = 0, tileCells: number = TILE_CELLS, limitM = 0): Agreement {
  const frame = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  for (const g of [...ga, ...gb]) for (const r of g) for (const p of r) {
    frame.minX = Math.min(frame.minX, p.x);
    frame.maxX = Math.max(frame.maxX, p.x);
    frame.minY = Math.min(frame.minY, p.y);
    frame.maxY = Math.max(frame.maxY, p.y);
  }
  if (!Number.isFinite(frame.minX)) return { iou: 1, rawIou: 1, p95M: 0, hausdorffM: 0, cellM: featureM / 4 };
  const cell = Math.max(1e-4, featureM / 4);
  const side = tileCells * cell;
  const margin = Math.ceil(Math.max(WINDOW_FEATURES * featureM, toleranceM + cell, limitM + 2 * featureM) / cell) * cell;
  const bins = Math.ceil(margin / cell) + 2;
  const [nx, ny] = [Math.max(1, Math.ceil((frame.maxX - frame.minX) / side)), Math.max(1, Math.ceil((frame.maxY - frame.minY) / side))];
  let [both, either, unforgiven] = [0, 0, 0];
  let worst = 0;
  let hist: number[] | undefined;
  const limit = (toleranceM / cell) ** 2;
  for (let j = 0; j < ny; j++)
    for (let i = 0; i < nx; i++) {
      const [minX, minY] = [frame.minX - 2 * cell + i * side, frame.minY - 2 * cell + j * side];
      const core = { minX, minY, maxX: minX + Math.ceil((Math.min(frame.maxX + 2 * cell, minX + side) - minX) / cell) * cell, maxY: minY + Math.ceil((Math.min(frame.maxY + 2 * cell, minY + side) - minY) / cell) * cell };
      if (core.maxX <= core.minX || core.maxY <= core.minY) continue;
      const box = { minX: core.minX - margin, minY: core.minY - margin, maxX: core.maxX + margin, maxY: core.maxY + margin };
      const [a, b] = [new Mask(box, cell), new Mask(box, cell)];
      for (const g of ga) a.fill(g, true);
      for (const g of gb) b.fill(g, true);
      a.seal();
      b.seal();
      const range = { i0: Math.round(margin / cell), i1: Math.round((margin + core.maxX - core.minX) / cell), j0: Math.round(margin / cell), j1: Math.round((margin + core.maxY - core.minY) / cell) };
      for (let y = range.j0; y < range.j1; y++)
        for (let x = range.i0; x < range.i1; x++) {
          const k = y * a.width + x;
          const [p, q] = [a.cells[k]!, b.cells[k]!];
          if (p & q) both++;
          if (p | q) either++;
          if (toleranceM > 0) {
            if (p && !q && nearWithin(b.cells, b.width, b.height, k, limit) === Infinity) unforgiven++;
            if (q && !p && nearWithin(a.cells, a.width, a.height, k, limit) === Infinity) unforgiven++;
          }
        }
      const spread = boundarySpread(a.cells, b.cells, undefined, a.width, a.height, cell, range, bins, (4 * featureM / cell) ** 2);
      worst = Math.max(worst, spread.maxM);
      hist = hist === undefined ? [...spread.hard] : hist.map((x, k) => x + (spread.hard[k] ?? 0));
    }
  const rawIou = either === 0 ? 1 : both / either;
  return { iou: toleranceM > 0 ? (either === 0 ? 1 : 1 - unforgiven / either) : rawIou, rawIou, p95M: quantileOf(hist ?? [], 0.95, cell), hausdorffM: worst, cellM: cell };
}

/** 🧊️ The projected convex hull of the vertices of the solids of one bone, as one ring (Andrew's monotone chain). */
function hullRing(solids: readonly World[], view: Viewpoint): Group {
  const points = solids.flatMap((s) => Array.from({ length: s.positions.length / 3 }, (_, i) => project(view, s.positions[i * 3]!, s.positions[i * 3 + 1]!, s.positions[i * 3 + 2]!))).sort((p, q) => p.x - q.x || p.y - q.y);
  const cross = (o: Point, a: Point, b: Point): number => (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
  const build = (list: readonly Point[]): Point[] => {
    const out: Point[] = [];
    for (const p of list) {
      while (out.length >= 2 && cross(out[out.length - 2]!, out[out.length - 1]!, p) <= 0) out.pop();
      out.push(p);
    }
    out.pop();
    return out;
  };
  return points.length < 3 ? [] : [[...build(points), ...build([...points].reverse())]];
}

/** 🪞️ {@link compareGroups} of the projections of two sets of closed meshes in one viewpoint; `hullBones` replaces the solids of `second` on those bones by their convex hull per bone (the lattice an envelope prism of `first` stands for). */
export function silhouetteAgreement(first: readonly World[], second: readonly World[], view: Viewpoint, featureM: number, toleranceM = 0, options: { readonly hullBones?: ReadonlySet<string>; readonly tileCells?: number; readonly limitM?: number; readonly drawnFirst?: Drawn; readonly drawnSecond?: Drawn } = {}): Agreement {
  const hull = options.hullBones ?? new Set<string>();
  const kept = second.filter((solid) => !hull.has(solid.bone ?? ""));
  const ga = referenceOf(view, first, undefined, options.drawnFirst).cut as readonly Group[];
  const gb = [...referenceOf(view, kept, undefined, options.drawnSecond).cut, ...[...hull].map((bone) => hullRing(second.filter((solid) => solid.bone === bone), view)).filter((ring) => ring.length > 0)] as readonly Group[];
  return compareGroups(ga, gb, featureM, toleranceM, options.tileCells, options.limitM);
}

function viewMasks(first: readonly World[], second: readonly World[], view: Viewpoint, featureM: number): { a: Mask; b: Mask } {
  const [ga, gb] = [referenceOf(view, first, undefined).cut, referenceOf(view, second, undefined).cut];
  const box = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  for (const g of [...ga, ...gb]) for (const r of g) for (const p of r) {
    box.minX = Math.min(box.minX, p.x);
    box.maxX = Math.max(box.maxX, p.x);
    box.minY = Math.min(box.minY, p.y);
    box.maxY = Math.max(box.maxY, p.y);
  }
  const long = Math.max(box.maxX - box.minX, box.maxY - box.minY, 1e-6);
  const cell = Math.max(long / GATE.cellsMax, Math.min(featureM / 4, long / 200));
  const frame = { minX: box.minX - 2 * cell, minY: box.minY - 2 * cell, maxX: box.maxX + 2 * cell, maxY: box.maxY + 2 * cell };
  const [a, b] = [new Mask(frame, cell), new Mask(frame, cell)];
  for (const g of ga as readonly Group[]) a.fill(g, true);
  for (const g of gb as readonly Group[]) b.fill(g, true);
  a.seal();
  b.seal();
  return { a, b };
}

const iouOf = (a: Mask, b: Mask): number => {
  let both = 0;
  let either = 0;
  for (let i = 0; i < a.cells.length; i++) {
    both += a.cells[i]! & b.cells[i]!;
    either += a.cells[i]! | b.cells[i]!;
  }
  return either === 0 ? 1 : both / either;
};

/** 🪟️ `volumes-views-disagree` (amendments C8a, C11): the elevation, the front and the plan of one scale must give the whole asset the same extent along a shared axis (height: elevation and front, length: elevation and plan, width: front and plan) within max(one smallest feature, 2 % of the larger extent); a finding names the axis and the two extents, and the 2D drawing is to be repaired, no 3D model can agree with both. Open strokes and unfilled outlines take part widened by half their pen (as in the silhouette comparison). The per-bone differences are measurements named `info:extentDifferenceM:<bone>:<axis>` that never fail. */
function viewsDisagreeRule(asset: Asset, volumes: readonly AuthoredVolume[], state: (d: ScaleDenominator) => ReturnType<typeof evaluate>, minFeatureMm: number): RuleResult {
  const c = collector("volumes-views-disagree");
  type Box = { lo: number; hi: number };
  type Axis = "x" | "y" | "z";
  const axes: Record<Viewpoint, readonly [Axis, Axis]> = { elevation: ["x", "y"], front: ["z", "y"], plan: ["x", "z"] };
  const names: Record<Axis, string> = { x: "length", y: "height", z: "width" };
  const pairs: readonly (readonly [Axis, Viewpoint, Viewpoint])[] = [["y", "elevation", "front"], ["x", "elevation", "plan"], ["z", "front", "plan"]];
  for (const volume of volumes) {
    const d = volume.denominator;
    const feature = featureOf(d, minFeatureMm);
    const evaluated = state(d);
    const whole = new Map<string, Box>();
    const perBone = new Map<string, Box>();
    const grow = (map: Map<string, Box>, key: string, values: readonly number[]): void => {
      const box = map.get(key);
      map.set(key, { lo: Math.min(box?.lo ?? Infinity, ...values), hi: Math.max(box?.hi ?? -Infinity, ...values) });
    };
    const bones = new Set<string>();
    for (const view of VIEWPOINTS)
      for (const item of evaluated.shapes[view] as readonly { readonly shape: { readonly bone: string; readonly role: string; readonly filled?: boolean }; readonly rings: readonly (readonly { readonly x: number; readonly y: number }[])[]; readonly closed: boolean }[]) {
        if (!["body", "detail", "glazing"].includes(item.shape.role)) continue;
        const points = item.rings.flat();
        if (points.length === 0) continue;
        const stroke = !item.closed || item.shape.filled === false;
        const pen = stroke ? (((item.shape as { penMm?: number }).penMm ?? (asset.representations[`${d}`] as { pens?: Record<string, number | null> }).pens?.[item.shape.role] ?? 0) * d) / 2000 : 0;
        bones.add(item.shape.bone);
        for (const [index, axis] of axes[view].entries()) {
          const values = points.flatMap((q) => (index === 0 ? [q.x - pen, q.x + pen] : [q.y - pen, q.y + pen]));
          grow(whole, `${view}/${axis}`, values);
          grow(perBone, `${item.shape.bone}/${view}/${axis}`, values);
        }
      }
    for (const [axis, first, second] of pairs) {
      const [p, q] = [whole.get(`${first}/${axis}`), whole.get(`${second}/${axis}`)];
      if (p === undefined || q === undefined) continue;
      const [e1, e2] = [p.hi - p.lo, q.hi - q.lo];
      const limit = Math.max(VOLUME_LIMITS.viewsDisagreeFeatures * feature, VOLUME_LIMITS.boundsFraction * Math.max(e1, e2));
      const m = c.measure({ scale: d, list: "body", name: `extentDifferenceM:${axis}`, value: Math.abs(e1 - e2), limit, bound: "max" });
      if (!m.ok) c.fail(`representations.${d}`, `The ${first} and the ${second} of 1:${d} disagree on the ${names[axis]} of the asset: ${(e1 * 1000).toFixed(0)} mm against ${(e2 * 1000).toFixed(0)} mm (more than ${mm(m.limit)}); repair the 2D drawing, no 3D model can agree with both.`, `Die ${first} und die ${second} von 1:${d} widersprechen sich bei ${axis === "y" ? "der Höhe" : axis === "x" ? "der Länge" : "der Breite"} des Assets: ${(e1 * 1000).toFixed(0)} mm gegen ${(e2 * 1000).toFixed(0)} mm (mehr als ${mm(m.limit)}); die 2D-Zeichnung ist zu korrigieren, kein 3D-Modell kann beiden folgen.`, { scale: d, list: "body", code: "volumes-views-disagree", measurements: [m], data: { axis, first, second, firstExtentM: e1, secondExtentM: e2 } });
    }
    for (const bone of bones)
      for (const [axis, first, second] of pairs) {
        const [p, q] = [perBone.get(`${bone}/${first}/${axis}`), perBone.get(`${bone}/${second}/${axis}`)];
        if (p !== undefined && q !== undefined) c.measure({ scale: d, list: "body", name: `info:extentDifferenceM:${bone}:${axis}`, value: Math.abs(p.hi - p.lo - (q.hi - q.lo)), limit: Number.MAX_VALUE, bound: "max" });
      }
  }
  return c;
}

/** 🌿️ The ids of the body solids of a volume that model a soft mass (foliage, fur, feather, hair): those that a thermal reference of a soft region reuses. */
function softIdsOf(asset: Asset, volume: AuthoredVolume): ReadonlySet<string> {
  const soft = new Set(asset.thermal.regions.filter((region) => region.areaBasis === "exchange" || (VOLUME_LIMITS.softRegionKinds as readonly string[]).includes(region.kind)).map((region) => region.id));
  return new Set(volume.thermal.flatMap((solid) => ((solid as { of?: string }).of !== undefined && soft.has((solid as { region: string }).region) ? [(solid as { of: string }).of] : [])));
}

/** 🧱️ The contact planes of the `contact` and `support` anchors of an evaluated pose: through the anchor, with the normal `normalDeg` in the sagittal plane (default 90, vertical) lifted by `tiltDeg` towards +z, within 0.02 smallest features (at least 0.1 mm, at most 1 mm): a face 2 mm above the plane is exposed. */
export function contactPlanes(state: ReturnType<typeof evaluate>, featureM: number): readonly ContactPlane[] {
  return Object.values(state.anchors)
    .filter((entry) => entry.anchor.kind === "contact" || entry.anchor.kind === "support")
    .map((entry) => {
      const [n, t] = [((entry.anchor.normalDeg ?? 90) * Math.PI) / 180, ((entry.anchor.tiltDeg ?? 0) * Math.PI) / 180];
      return { point: [entry.at.x, entry.at.y, entry.z] as const, normal: [Math.cos(n) * Math.cos(t), Math.sin(n) * Math.cos(t), Math.sin(t)] as const, toleranceM: Math.min(0.001, Math.max(1e-4, 0.02 * featureM)) };
    });
}

/** 📍️ A plume or another emitter reduced to the centre of its box: an emitter leaves the surface it comes from, so only where it starts has to lie in the body box. */
const atCentre = (w: World): World => {
  const box = extentOf([w]);
  return { id: w.id, positions: Float64Array.from([0, 1, 2].map((k) => (box.min[k]! + box.max[k]!) / 2)), indices: new Uint32Array(0) };
};

const mm = (metres: number): string => `${(metres * 1000).toFixed(1)} mm`;

/** 📊️ Every measurement and finding of the requested rules at the requested scales. */
function run(asset: Asset, options: VolumeOptions, operation: JobControl): { readonly measurements: readonly VolumeMeasurement[]; readonly findings: readonly VolumeFinding[] } {
  const rules = new Set<VolumeRule>(options.rules ?? VOLUME_RULES);
  const minFeatureMm = options.profile?.minFeatureMm ?? VOLUME_LIMITS.minFeatureMm;
  const pose = options.pose ?? asset.presentation.defaultPose;
  const acc: RuleResult = { measurements: [], findings: [] };
  const results = { push: (r: RuleResult): void => void (acc.measurements.push(...r.measurements), acc.findings.push(...r.findings)) };
  const only = options.scale === undefined ? undefined : ([options.scale] as readonly ScaleDenominator[]);
  if (rules.has("volumes-availability")) results.push(availabilityRule(asset, only));
  if (rules.has("volumes-bones")) results.push(bonesRule(asset, only));
  if (rules.has("volumes-feature")) results.push(featureRule(asset, only, minFeatureMm));
  if (rules.has("volumes-thermal-coverage")) results.push(thermalCoverageRule(asset, only));
  if (rules.has("volumes-structure-coverage")) results.push(structureCoverageRule(asset, only));
  if (rules.has("volumes-copied")) results.push(copiedRule(asset, only));
  if (rules.has("volumes-slender")) results.push(slenderRule(asset, only));
  if (rules.has("volumes-views-disagree")) results.push(viewsDisagreeRule(asset, authoredVolumes(asset).filter((v) => options.scale === undefined || v.denominator === options.scale), (d) => evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: d } }), minFeatureMm));
  const issues = volumeIssues(asset) as readonly Issue[];
  for (const issue of issues) {
    const scale = Number(/^volumes\.(\d+)/.exec(issue.path)?.[1]) as ScaleDenominator;
    if (options.scale !== undefined && scale !== options.scale) continue;
    const list = /\.(body|thermal|structure)\b/.exec(issue.path)?.[1] as VolumeList | undefined;
    const mapped: VolumeRule = issue.code === "volume-region" || issue.code === "volume-layer" ? "volumes-thermal-coverage" : issue.code === "volume-member" || issue.code === "volume-anchor" ? "volumes-structure-coverage" : issue.code === "volume-duplicate-id" || issue.code === "volume-of" ? "volumes-bones" : "volumes-solids";
    acc.findings.push({ ...issue, rule: mapped, ...(Number.isFinite(scale) ? { scale } : {}), ...(list === undefined ? {} : { list }), measurements: [] });
  }
  const geometric = ["volumes-silhouette", "volumes-bounds", "volumes-monotonic", "volumes-cross-scale", "volumes-thermal-coverage", "volumes-structure-coverage"].some((rule) => rules.has(rule as VolumeRule));
  const volumes = authoredVolumes(asset);
  const targets = volumes.filter((volume) => options.scale === undefined || volume.denominator === options.scale);
  const soft = new Set(["volume-region", "volume-layer", "volume-member", "volume-anchor", "volume-of", "volume-duplicate-id", "volume-material", "volume-envelope"]);
  const blocked = new Set<number>();
  for (const issue of issues) if (!soft.has(issue.code)) blocked.add(Number(/^volumes\.(\d+)/.exec(issue.path)?.[1] ?? 0));
  const usable = (d: ScaleDenominator): boolean => !blocked.has(0) && !blocked.has(d);
  if (geometric && volumes.length > 0) {
    const sil = collector("volumes-silhouette");
    const bounds = collector("volumes-bounds");
    const mono = collector("volumes-monotonic");
    const cross = collector("volumes-cross-scale");
    const thermal = collector("volumes-thermal-coverage");
    const structure = collector("volumes-structure-coverage");
    const total = targets.length * 4;
    let done = 0;
    const step = (label: Text): void => {
      operation.signal?.throwIfAborted();
      operation.progress?.(done++, total, label);
    };
    const cache = new Map<string, readonly World[]>();
    const stateAt = new Map<number, ReturnType<typeof evaluate>>();
    const stateOf = (denominator: ScaleDenominator): ReturnType<typeof evaluate> => {
      let hit = stateAt.get(denominator);
      if (hit === undefined) {
        hit = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator } });
        stateAt.set(denominator, hit);
      }
      return hit;
    };
    const worldAt = (volume: AuthoredVolume, list: VolumeList): readonly World[] => {
      const key = `${volume.denominator}:${list}`;
      let hit = cache.get(key);
      if (hit === undefined) {
        hit = world(asset, stateOf(volume.denominator).bones, volume.denominator, list);
        cache.set(key, hit);
      }
      return hit;
    };
    for (const volume of targets) {
      const d = volume.denominator;
      if (!usable(d)) continue;
      const feature = featureOf(d, minFeatureMm);
      step(PHASE(`1:${d} silhouette`, `1:${d} Silhouette`));
      if (rules.has("volumes-silhouette") || rules.has("volumes-bounds")) {
        const m = measureAsset(asset, d, pose, undefined, feature, softIdsOf(asset, volume));
        if (rules.has("volumes-bounds"))
          (["widthM", "heightM", "depthM"] as const).forEach((name, k) => {
            const want = asset.dimensions[name];
            const got = m.bbox[k]!;
            const limit = Math.max(VOLUME_LIMITS.boundsFeatures * feature, VOLUME_LIMITS.boundsFraction * want);
            const measured = bounds.measure({ scale: d, list: "body", name: `bboxDeviationM:${name}`, value: Math.abs(got - want), limit, bound: "max" });
            if (!measured.ok) bounds.fail(`volumes.${d}.body`, `The body of 1:${d} is ${got.toFixed(3)} m along ${name} but dimensions say ${want} m (limit ${mm(measured.limit)}).`, `Der Body von 1:${d} misst ${got.toFixed(3)} m entlang ${name}, dimensions nennen ${want} m (Grenze ${mm(measured.limit)}).`, { scale: d, list: "body", measurements: [measured] });
          });
        if (rules.has("volumes-silhouette"))
          for (const view of VIEWPOINTS) {
            const v = m.views[view];
            const elevation = view === "elevation";
            const hardLimit = (elevation ? VOLUME_LIMITS.silhouetteP95FeaturesElevation : VOLUME_LIMITS.silhouetteP95FeaturesOther) * feature;
            const iou = sil.measure({ scale: d, list: "body", name: `iou:${view}`, value: v.iou, limit: elevation ? VOLUME_LIMITS.silhouetteIouElevation : VOLUME_LIMITS.silhouetteIouOther, bound: "min" });
            const p95 = sil.measure({ scale: d, list: "body", name: `p95M:${view}`, value: v.p95M, limit: hardLimit, bound: "max" });
            const softLimit = Math.max(hardLimit, VOLUME_LIMITS.softMassExtentFraction * v.softExtentM);
            const soft = v.softExtentM > 0 ? sil.measure({ scale: d, list: "body", name: `p95SoftM:${view}`, value: v.p95SoftM, limit: softLimit, bound: "max" }) : undefined;
            sil.measure({ scale: d, list: "body", name: `hausdorffM:${view}`, value: v.hausdorffM, limit: Number.MAX_VALUE, bound: "max" });
            const failing = [iou, p95, ...(soft === undefined ? [] : [soft])].filter((x) => !x.ok);
            if (failing.length > 0) sil.fail(`volumes.${d}.body`, `The body of 1:${d} disagrees with the ${view} drawing: ${failing.map((x) => `${x.name.split(":")[0]} ${x.name.startsWith("iou") ? x.value.toFixed(3) : mm(x.value)} (${x.bound === "min" ? "at least" : "at most"} ${x.name.startsWith("iou") ? x.limit : mm(x.limit)})`).join(", ")}; the largest boundary distance is ${mm(v.hausdorffM)}.`, `Der Body von 1:${d} weicht von der Zeichnung ${view} ab: ${failing.map((x) => `${x.name.split(":")[0]} ${x.name.startsWith("iou") ? x.value.toFixed(3) : mm(x.value)} (${x.bound === "min" ? "mindestens" : "höchstens"} ${x.name.startsWith("iou") ? x.limit : mm(x.limit)})`).join(", ")}; der größte Randabstand beträgt ${mm(v.hausdorffM)}.`, { scale: d, list: "body", measurements: failing });
          }
      }
      step(PHASE(`1:${d} thermal`, `1:${d} Thermik`));
      if (rules.has("volumes-thermal-coverage")) {
        const body = worldAt(volume, "body");
        const worlds = worldAt(volume, "thermal");
        const bodyBox = extentOf(body);
        const tolerance = volume.denominator === 50 ? VOLUME_LIMITS.thermalAreaFine : VOLUME_LIMITS.thermalAreaCoarse;
        const byId = new Map(worlds.map((w) => [w.id, w] as const));
        const planes = contactPlanes(stateOf(d), feature);
        const exchangeRegions = new Set(asset.thermal.regions.filter((r) => r.areaBasis === "exchange").map((r) => r.id));
        const ofExchange = new Set(volume.thermal.filter((s) => exchangeRegions.has((s as { region: Slug }).region)).flatMap((s) => [s.id, ...((s as { of?: string }).of === undefined ? [] : [(s as { of: string }).of])]));
        const ofOther = new Set(volume.thermal.filter((s) => !exchangeRegions.has((s as { region: Slug }).region) && (s as { of?: string }).of !== undefined).map((s) => (s as { of: string }).of));
        const porous = new Set([...ofExchange].filter((id) => !ofOther.has(id)));
        let outside = 0;
        const perRegion = d >= 200;
        let [sumArea, sumDeclared] = [0, 0];
        for (const region of asset.thermal.regions) {
          const own = volume.thermal.filter((s) => (s as { region: Slug }).region === region.id);
          if (own.length === 0) continue;
          const outer = region.layers.length > 1 && d === 50 ? region.layers[region.layers.length - 1]!.id : undefined;
          const shell = outer === undefined ? own : own.filter((s) => (s as { layer?: Slug }).layer === outer);
          const picked = (shell.length > 0 ? shell : own).map((s) => byId.get(s.id)).filter((w): w is World => w !== undefined && w.envelope !== true && w.emitter !== true);
          if (own.every((solid) => byId.get(solid.id)?.emitter === true) && region.areaM2 > 0) {
            thermal.fail(`volumes.${d}.thermal`, `Thermal region "${region.id}" at 1:${d} has only emitter solids, which are no surface; it needs a solid of its own surface.`, `Die thermische Region "${region.id}" hat bei 1:${d} nur Emitterkörper, die keine Oberfläche sind; sie braucht einen Körper ihrer eigenen Oberfläche.`, { scale: d, list: "thermal", measurements: [], data: { region: region.id } });
            continue;
          }
          if (picked.length === 0 || region.areaM2 <= 0) continue;
          const exchange = region.areaBasis === "exchange";
          if (exchange) continue;
          const hidden = region.hidden !== undefined;
          const ownWorlds = new Set(own.map((s) => byId.get(s.id)).filter((w): w is World => w !== undefined));
          const against = hidden ? [] : (["body", "thermal", "structure"] as const).flatMap((list) => worldAt(volume, list)).filter((w) => !porous.has(w.id));
          const probe = Math.max(1e-4, 0.02 * feature);
          const seam = SEAM_FEATURES * feature;
          const area = exposedArea(picked, hidden ? [] : picked, [], () => false, probe, seam);
          const exposed = exposedArea(picked, against, planes, (other) => ownWorlds.has(other), probe, seam);
          const binding = (asset.poses as readonly { readonly id: Slug; readonly thermal: { readonly regions: Readonly<Record<string, { readonly exposedFraction: number; readonly contact?: { readonly fraction: number } }>> } }[]).find((p) => p.id === pose)?.thermal.regions[region.id];
          const target = binding === undefined ? 1 : binding.exposedFraction * (1 - (binding.contact?.fraction ?? 0));
          const info = (name: string, value: number): void => void thermal.measure({ scale: d, list: "thermal", name: `info:${name}:${region.id}`, value, limit: Number.MAX_VALUE, bound: "max" });
          const small = region.areaM2 < VOLUME_LIMITS.smallRegionFeatures * feature * feature;
          if (!small) sumArea += area;
          info("outerAreaM2", area);
          info("exposedAreaM2", exposed);
          info("exposureMeasured", area > 0 ? exposed / area : 0);
          info("exposureTarget", target);
          info("exposureDeviation", area > 0 ? Math.abs(exposed / area - target) : 0);
          if (!small) sumDeclared += region.areaM2;
          const deviation = thermal.measure({ scale: d, list: "thermal", name: `areaDeviation:${region.id}`, value: Math.abs(area / region.areaM2 - 1), limit: perRegion || small ? Number.MAX_VALUE : (VOLUME_LIMITS.bodyAreaModels as readonly string[]).includes(region.model) ? VOLUME_LIMITS.thermalAreaCoarse : tolerance, bound: "max" });
          if (!deviation.ok) thermal.fail(`volumes.${d}.thermal`, `The outer surface of region "${region.id}" at 1:${d} is ${area.toFixed(3)} m² against areaM2 ${region.areaM2} m² (deviation ${(deviation.value * 100).toFixed(0)} %, limit ${(tolerance * 100).toFixed(0)} %).`, `Die Außenfläche der Region "${region.id}" bei 1:${d} beträgt ${area.toFixed(3)} m² gegen areaM2 ${region.areaM2} m² (Abweichung ${(deviation.value * 100).toFixed(0)} %, Grenze ${(tolerance * 100).toFixed(0)} %).`, { scale: d, list: "thermal", measurements: [deviation], data: { region: region.id } });
        }
        if (perRegion && sumDeclared > 0) {
          const total = thermal.measure({ scale: d, list: "thermal", name: "areaDeviationTotal", value: Math.abs(sumArea / sumDeclared - 1), limit: VOLUME_LIMITS.thermalAreaCoarse, bound: "max" });
          if (!total.ok) thermal.fail(`volumes.${d}.thermal`, `The outer surface of all regions at 1:${d} is ${sumArea.toFixed(3)} m² against the sum of areaM2 ${sumDeclared.toFixed(3)} m² (deviation ${(total.value * 100).toFixed(0)} %, limit ${(total.limit * 100).toFixed(0)} %).`, `Die Außenfläche aller Regionen bei 1:${d} beträgt ${sumArea.toFixed(3)} m² gegen die Summe der areaM2 ${sumDeclared.toFixed(3)} m² (Abweichung ${(total.value * 100).toFixed(0)} %, Grenze ${(total.limit * 100).toFixed(0)} %).`, { scale: d, list: "thermal", measurements: [total] });
        }
        const box = extentOf(worlds.map((w) => (w.emitter === true ? atCentre(w) : w)));
        const stray = [0, 1, 2].reduce((m, k) => Math.max(m, bodyBox.min[k]! - box.min[k]!, box.max[k]! - bodyBox.max[k]!), 0);
        const strayM = thermal.measure({ scale: d, list: "thermal", name: "outsideBodyBoxM", value: Math.max(0, stray), limit: VOLUME_LIMITS.thermalBoxFeatures * feature, bound: "max" });
        if (!strayM.ok) {
          outside++;
          thermal.fail(`volumes.${d}.thermal`, `Thermal solids of 1:${d} protrude ${mm(stray)} out of the body bounding box (limit ${mm(strayM.limit)}).`, `Thermische Körper von 1:${d} ragen ${mm(stray)} aus der Body-Box (Grenze ${mm(strayM.limit)}).`, { scale: d, list: "thermal", measurements: [strayM] });
        }
        void outside;
      }
      step(PHASE(`1:${d} structure`, `1:${d} Tragwerk`));
      if (rules.has("volumes-structure-coverage")) {
        const body = worldAt(volume, "body");
        const members = solidsOf(volume, "structure");
        const worlds = worldAt(volume, "structure");
        const state = stateOf(d);
        const parts = new Map(volume.structure.map((s) => [s.id, (s as { part: string }).part] as const));
        const needBody = worlds.filter((w) => parts.get(w.id) !== "support");
        for (const view of VIEWPOINTS) {
          const { a, b } = viewMasks(body, needBody, view, feature);
          const field = distanceField(a.cells, a.width, a.height);
          const reach = (groups: readonly Group[]): number => {
            let worst = 0;
            for (const g of groups) for (const r of g) for (const p of r) {
              const [i, j] = [Math.floor((p.x - a.box.minX) / a.cell), Math.floor((p.y - a.box.minY) / a.cell)];
              if (i >= 0 && j >= 0 && i < a.width && j < a.height) worst = Math.max(worst, Math.sqrt(field[j * a.width + i]!) * a.cell);
            }
            return worst;
          };
          const worst = reach(referenceOf(view, needBody, undefined).cut as readonly Group[]);
          void b;
          const m = structure.measure({ scale: d, list: "structure", name: `outsideBodyM:${view}`, value: Math.max(0, worst - a.cell), limit: feature, bound: "max" });
          if (!m.ok) {
            const culprits = needBody.map((w) => ({ id: w.id, outsideM: Math.max(0, reach(referenceOf(view, [w], undefined).cut as readonly Group[]) - a.cell) })).filter((x) => x.outsideM > feature).sort((p, q) => q.outsideM - p.outsideM).slice(0, 3);
            const named = culprits.map((x) => `"${x.id}" ${mm(x.outsideM)}`).join(", ");
            structure.fail(`volumes.${d}.structure`, `Member solids of 1:${d} lie ${mm(m.value)} outside the body silhouette in the ${view} (limit ${mm(feature)}); the worst are ${named}.`, `Stabkörper von 1:${d} liegen ${mm(m.value)} außerhalb der Body-Silhouette in der ${view} (Grenze ${mm(feature)}); am weitesten außen liegen ${named}.`, { scale: d, list: "structure", measurements: [m], data: { view, solids: culprits.map((x) => x.id) } });
          }
        }
        void members;
        for (const solid of volume.structure) {
          if ((solid as { part: string }).part !== "support") continue;
          const w = worlds.find((x) => x.id === solid.id);
          if (w === undefined) continue;
          const box = extentOf([w]);
          const anchor = state.anchors[(solid as { anchor?: Slug }).anchor ?? ""];
          const nearGround = Math.abs(box.min[1]!) <= feature;
          const atAnchor = anchor !== undefined && anchor.at.x >= box.min[0]! - feature && anchor.at.x <= box.max[0]! + feature && anchor.at.y >= box.min[1]! - feature && anchor.at.y <= box.max[1]! + feature && anchor.z >= box.min[2]! - feature && anchor.z <= box.max[2]! + feature;
          const m = structure.measure({ scale: d, list: "structure", name: `supportOffM:${solid.id}`, value: nearGround || atAnchor ? 0 : Math.abs(box.min[1]!), limit: feature, bound: "max" });
          if (!m.ok) structure.fail(`volumes.${d}.structure`, `Support solid "${solid.id}" of 1:${d} sits neither on the ground plane nor at its anchor (${mm(m.value)} off).`, `Auflagerkörper "${solid.id}" von 1:${d} liegt weder auf der Bodenebene noch an seinem Anker (${mm(m.value)} entfernt).`, { scale: d, list: "structure", measurements: [m] });
        }
      }
      step(PHASE(`1:${d} counts`, `1:${d} Zählung`));
    }
    const fine = (denominator: ScaleDenominator): AuthoredVolume | undefined => {
      const index = volumes.findIndex((v) => v.denominator === denominator);
      return index > 0 ? volumes[index - 1] : undefined;
    };
    for (const volume of targets) {
      const finer = fine(volume.denominator);
      if (finer === undefined || !usable(volume.denominator) || !usable(finer.denominator)) continue;
      const d = volume.denominator;
      const feature = featureOf(d, minFeatureMm);
      operation.signal?.throwIfAborted();
      if (rules.has("volumes-monotonic"))
        for (const list of ["body", "thermal", "structure"] as const) {
          const [coarseCount, fineCount] = [expandedCount(solidsOf(volume, list)), expandedCount(solidsOf(finer, list))];
          const [coarseTri, fineTri] = [fixedTriangleCount(solidsOf(volume, list)), fixedTriangleCount(solidsOf(finer, list))];
          const count = mono.measure({ scale: d, list, name: "solidCountDelta", value: coarseCount - fineCount, limit: 0, bound: "max" });
          const tri = mono.measure({ scale: d, list, name: "triangleCountDelta", value: coarseTri - fineTri, limit: 0, bound: "max" });
          const minimal = volume.decisions.some((text) => text.en.startsWith(VOLUME_LIMITS.minimalDecision));
          const drop = mono.measure({ scale: d, list, name: "strictDrop", value: coarseCount < fineCount || coarseTri < fineTri || minimal ? 1 : 0, limit: 1, bound: "min" });
          const failing = [count, tri, drop].filter((x) => !x.ok);
          if (failing.length > 0)
            mono.fail(`volumes.${d}.${list}`, `The ${list} of 1:${d} (${coarseCount} solids, ${coarseTri} triangles) is not simpler than 1:${finer.denominator} (${fineCount}, ${fineTri}); a coarser scale never grows and one count must drop unless a decision starts with "${VOLUME_LIMITS.minimalDecision}".`, `Der ${list} von 1:${d} (${coarseCount} Körper, ${coarseTri} Dreiecke) ist nicht einfacher als 1:${finer.denominator} (${fineCount}, ${fineTri}); ein gröberer Maßstab wächst nie und eine Zahl muss sinken, außer eine Entscheidung beginnt mit "${VOLUME_LIMITS.minimalDecision}".`, { scale: d, list, measurements: failing });
        }
      if (rules.has("volumes-cross-scale")) {
        const [coarseAll, fineAll] = [worldAt(volume, "body").filter((w) => w.texture !== true), worldAt(finer, "body").filter((w) => w.texture !== true)];
        const [coarseBody, fineBody] = [coarseAll.filter((w) => w.slender !== true), fineAll.filter((w) => w.slender !== true)];
        const hullBones = new Set(coarseBody.filter((w) => w.envelope === true).map((w) => w.bone ?? ""));
        const [stateCoarse, stateFine] = [stateOf(d), stateOf(finer.denominator)];
        const drawnGroups = (state: ReturnType<typeof evaluate>, view: Viewpoint): readonly Group[] =>
          (state.shapes[view] as readonly { readonly shape: { readonly role: string; readonly filled?: boolean }; readonly rings: readonly (readonly { readonly x: number; readonly y: number }[])[]; readonly closed: boolean }[]).filter((item) => item.closed && item.shape.filled !== false && ["body", "detail", "glazing"].includes(item.shape.role)).map((item) => item.rings as Group);
        for (const view of VIEWPOINTS) {
          operation.signal?.throwIfAborted();
          const pair = compareGroups(drawnGroups(stateCoarse, view), drawnGroups(stateFine, view), feature);
          const agreement = silhouetteAgreement(coarseAll, fineAll, view, feature, VOLUME_LIMITS.crossToleranceFeatures * feature, { hullBones, limitM: Math.max(VOLUME_LIMITS.crossDistanceFeatures * feature, pair.p95M + VOLUME_LIMITS.crossDistancePlus2DFeatures * feature), drawnFirst: drawnFor(asset, stateCoarse, view, d), drawnSecond: drawnFor(asset, stateFine, view, finer.denominator) });
          const iou = cross.measure({ scale: d, list: "body", name: `iou:${view}`, value: agreement.iou, limit: Math.min(VOLUME_LIMITS.crossIou, pair.rawIou - VOLUME_LIMITS.crossIouMarginTo2D), bound: "min" });
          const dist = cross.measure({ scale: d, list: "body", name: `p95M:${view}`, value: agreement.p95M, limit: Math.max(VOLUME_LIMITS.crossDistanceFeatures * feature, pair.p95M + VOLUME_LIMITS.crossDistancePlus2DFeatures * feature), bound: "max" });
          cross.measure({ scale: d, list: "body", name: `hausdorffM:${view}`, value: agreement.hausdorffM, limit: Number.MAX_VALUE, bound: "max" });
          cross.measure({ scale: d, list: "body", name: `iou2D:${view}`, value: pair.rawIou, limit: 0, bound: "min" });
          cross.measure({ scale: d, list: "body", name: `p952DM:${view}`, value: pair.p95M, limit: Number.MAX_VALUE, bound: "max" });
          if (!iou.ok || !dist.ok) cross.fail(`volumes.${d}.body`, `The body of 1:${d} disagrees with 1:${finer.denominator} in the ${view}: IoU ${iou.value.toFixed(3)} (at least ${iou.limit.toFixed(3)}), boundary distance p95 ${mm(dist.value)} (at most ${mm(dist.limit)}); the 2D drawings of the two scales themselves agree with IoU ${pair.rawIou.toFixed(3)} and p95 ${mm(pair.p95M)}.`, `Der Body von 1:${d} weicht von 1:${finer.denominator} in der ${view} ab: IoU ${iou.value.toFixed(3)} (mindestens ${iou.limit.toFixed(3)}), Randabstand p95 ${mm(dist.value)} (höchstens ${mm(dist.limit)}); die 2D-Zeichnungen der beiden Maßstäbe stimmen selbst mit IoU ${pair.rawIou.toFixed(3)} und p95 ${mm(pair.p95M)} überein.`, { scale: d, list: "body", measurements: [iou, dist].filter((x) => !x.ok) });
        }
        const [softCoarse, softFine] = [softIdsOf(asset, volume), softIdsOf(asset, finer)];
        const fineKept = fineBody.filter((w) => !hullBones.has(w.bone ?? ""));
        const coarseKept = coarseBody.filter((w) => w.envelope !== true);
        const bandM = VOLUME_LIMITS.crossVolumeBandFeatures * feature;
        const softM = Math.max(bandM, VOLUME_LIMITS.softMassExtentFraction * Math.max(...[0, 1, 2].map((k) => { const e = extentOf([...fineKept.filter((w) => softFine.has(w.id)), ...coarseKept.filter((w) => softCoarse.has(w.id))]); return Number.isFinite(e.min[k]) ? e.max[k]! - e.min[k]! : 0; })));
        const [hard, soft] = [bandVolumes(fineKept.filter((w) => !softFine.has(w.id)), coarseKept.filter((w) => !softCoarse.has(w.id)), bandM), bandVolumes(fineKept.filter((w) => softFine.has(w.id)), coarseKept.filter((w) => softCoarse.has(w.id)), softM)];
        const band = { fine: hard.fine + soft.fine, coarse: hard.coarse + soft.coarse, eroded: hard.eroded + soft.eroded, dilated: hard.dilated + soft.dilated };
        const excess = band.fine === 0 ? 0 : Math.max(0, band.eroded - band.coarse, band.coarse - band.dilated) / band.fine;
        const change = cross.measure({ scale: d, list: "body", name: "unionVolumeBandExcess", value: excess, limit: 0, bound: "max" });
        if (!change.ok) cross.fail(`volumes.${d}.body`, `The union volume of 1:${d} (${band.coarse.toFixed(4)} m³) lies outside the band ${band.eroded.toFixed(4)} to ${band.dilated.toFixed(4)} m³ of 1:${finer.denominator} eroded and dilated by ${mm(VOLUME_LIMITS.crossVolumeBandFeatures * feature)}.`, `Das Vereinigungsvolumen von 1:${d} (${band.coarse.toFixed(4)} m³) liegt außerhalb des Bandes ${band.eroded.toFixed(4)} bis ${band.dilated.toFixed(4)} m³ von 1:${finer.denominator}, erodiert und gedehnt um ${mm(VOLUME_LIMITS.crossVolumeBandFeatures * feature)}.`, { scale: d, list: "body", measurements: [change] });
      }
    }
    for (const result of [sil, bounds, mono, cross, thermal, structure]) results.push(result);
    operation.progress?.(total, total, PHASE("Volumes checked", "Volumen geprüft"));
  }
  return { measurements: acc.measurements.filter((m) => rules.has(m.rule)), findings: acc.findings.filter((f) => rules.has(f.rule)) };
}

/** 🩺️ The findings (empty = valid) of the volumes of an asset; each carries the failing measurements. Expensive rules report progress through `operation.progress` and stop through `operation.signal`. */
export function checkVolumes(asset: Asset, options: VolumeOptions = {}, operation: JobControl = {}): readonly VolumeFinding[] {
  return run(asset, options, operation).findings;
}

/** 📐️ Every measurement of the volumes of an asset, passing or not, with its limit, so that authors see how far off they are. */
export function measureVolumes(asset: Asset, options: VolumeOptions = {}, operation: JobControl = {}): readonly VolumeMeasurement[] {
  return run(asset, options, operation).measurements;
}
