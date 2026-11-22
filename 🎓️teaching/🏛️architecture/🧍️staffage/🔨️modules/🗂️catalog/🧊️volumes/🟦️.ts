/** 🧊️ `checkVolumes`: the rules of the per-scale, per-mode volumes of an asset with the measurements behind them, for the catalog check and the authoring tools.
 *
 * Analytic rules (availability, bones, feature, presence of thermal and structural solids, the per-solid `volume-*` rules) come from {@link ../🧱️rules/🟦️.ts} and `../🧊️volume/🟦️.ts`; the geometric rules tessellate with the owned kernel and
 * measure with the owned raster ({@link ../🌓️silhouette/🟦️.ts}), the owned column union and the owned ray parity: `volumes-silhouette` (IoU and Hausdorff of the body against the filled 2D shapes of the same scale in every viewpoint),
 * `volumes-bounds`, `volumes-monotonic` (expanded solids and triangles per list), `volumes-cross-scale` (IoU, Hausdorff and union volume against the finer authored scale), the area and box part of `volumes-thermal-coverage`
 * and the containment and ground part of `volumes-structure-coverage`, and `volumes-connected` ({@link connectedRule}: the body is one object in the default pose and every load path of the structure reaches a support, by the exact distances between the triangles of the parts). A thermal solid that is or references an `envelope: true` solid gives its region coverage but stands for a porous lattice, so its hull surface is left out of the exposed-area sums (C22); a solid tagged `emitter` (a plume, a breath) gives coverage but is no surface either, so it is left out of the sums too, and only its centre has to lie in the body box. The outer area of a region counts the faces the air from outside reaches through the thermal solids of the scale or that a solid of another region covers; the faces of a cavity those solids close off do not count when they are the inside of its closure, and a region that is contents of a closed cavity counts its whole surface, which exchanges with the enclosed air (C31, E3, `outsideOf`, `outerOfShares`; a solid of another region covers a face only when it stands between the face and the air, so a part that passes through a closed shell covers nothing inside it; `thermal.minimumAirPassageM` declares the narrowest real air passage of the asset, the cell of the flood is then at most a third of it at every scale and a flood whose budget cannot keep that cell is a finding (`airPassageCellM`); the cell of the flood is the measurement `info:floodCellM`, the share of the contents `info:cavityAreaM2:<region>`). A region that is not `hidden` and lies wholly in a closed cavity fails unless a glazing region (`VOLUME_LIMITS.transparentRegionKinds`) lets one see it (`seenFromOutside`). Every limit is in {@link VOLUME_LIMITS}. Expensive work reports progress and stops on cancellation.
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
import { SEAM_FEATURES, bandVolumes, exposedArea, exposedShares, fixedTriangleCount, isContents, outerArea, outerOfShares, outsideOf, seenFromOutside, unionVolume, type ContactPlane, type Outside, type Reach } from "./🧮️measures/🟦️.ts";
import { looseGroups, partGap, partsOf, type Loose, type Part } from "./🔗️connected/🟦️.ts";
import { spanGap } from "./🧱️rules/🟦️.ts";
import { insideAny, rememberedReach } from "./🧮️measures/🟦️.ts";
import { GATE, Mask, bodyDimensions, drawnFor, TILE_CELLS, WINDOW_FEATURES, VIEWPOINTS, boundarySpread, distanceField, extentOf, nearWithin, hausdorff, measureBodyView, project, projectedBodyDistance, quantileOf, referenceOf, worldSolids, type Drawn, type Group, type Point, type Viewpoint, type World } from "./🌓️silhouette/🟦️.ts";

export { VOLUME_LIMITS, VOLUME_RULES, analyticRules, compareSolidLists, featureOf };
export { bandVolumes, exposedArea, exposedShares, fixedTriangleCount, isClosed, isContents, outerArea, outerOfShares, outsideOf, seenFromOutside, unionVolume } from "./🧮️measures/🟦️.ts";
export type { ContactPlane, Outside, Reach } from "./🧮️measures/🟦️.ts";
export type { VolumeFinding, VolumeMeasurement, VolumeRule };
export type { SolidSimilarity } from "./🧱️rules/🟦️.ts";
export { authoredVolumes, expandedCount, solidsOf } from "./🧱️lists/🟦️.ts";
export { groupGap, looseGroups, partGap, partsOf, touchingGroups, triangleDistance2 } from "./🔗️connected/🟦️.ts";
export type { ConnectedOptions, Groups, Loose, Part, PartKind } from "./🔗️connected/🟦️.ts";

/** 🎛️ What a volume check is asked: `scale` (only that scale; for the pair rules the pair ending at it), `rules`, the `profile` that sets the smallest feature, the `pose` (default `presentation.defaultPose`). */
export type VolumeOptions = { readonly scale?: ScaleDenominator; readonly rules?: readonly VolumeRule[]; readonly profile?: Pick<PrintProfile, "minFeatureMm">; readonly pose?: Slug };

const PHASE = (en: string, de: string): Text => ({ en, de });
type Vec = readonly [number, number, number];
const world = (asset: Asset, bones: unknown, scale: number, list: VolumeList, body?: readonly World[]): readonly World[] => worldSolids(asset, bones as never, scale, list, body);

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

/** 🪞️ {@link compareGroups} of the projections of two sets of closed meshes in one viewpoint; `hullBones` replaces the solids of `second` on those bones by their convex hull per bone (the lattice an envelope prism of `first` stands for) and the `envelope` solids of `first` on those bones by theirs, so a typified row (fewer, thicker posts for many thin rods) is compared with the row it stands for through their common envelope; a convex prism is its own hull. */
export function silhouetteAgreement(first: readonly World[], second: readonly World[], view: Viewpoint, featureM: number, toleranceM = 0, options: { readonly hullBones?: ReadonlySet<string>; readonly tileCells?: number; readonly limitM?: number; readonly drawnFirst?: Drawn; readonly drawnSecond?: Drawn } = {}): Agreement {
  const hull = options.hullBones ?? new Set<string>();
  const kept = second.filter((solid) => !hull.has(solid.bone ?? ""));
  const typified = first.filter((solid) => solid.envelope === true && hull.has(solid.bone ?? ""));
  const ga = [...referenceOf(view, typified.length === 0 ? first : first.filter((solid) => !typified.includes(solid)), undefined, options.drawnFirst).cut, ...[...hull].map((bone) => hullRing(typified.filter((solid) => solid.bone === bone), view)).filter((ring) => ring.length > 0)] as readonly Group[];
  const gb = [...referenceOf(view, kept, undefined, options.drawnSecond).cut, ...[...hull].map((bone) => hullRing(second.filter((solid) => solid.bone === bone), view)).filter((ring) => ring.length > 0)] as readonly Group[];
  return compareGroups(ga, gb, featureM, toleranceM, options.tileCells, options.limitM);
}

function memberProjection(first: readonly World[], second: readonly World[], view: Viewpoint, featureM: number): { bodyMask: Mask; faces: readonly Group[]; members: readonly Group[] } {
  const [ga, gb] = [referenceOf(view, first, undefined, undefined, "mesh").cut, referenceOf(view, second, undefined, undefined, "mesh").cut];
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
  const bodyMask = new Mask(frame, cell);
  for (const g of ga) bodyMask.fill(g, true);
  bodyMask.seal();
  return { bodyMask, faces: ga, members: gb };
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

/** 🪟️ `volumes-views-disagree` (amendments C8a, C11): the elevation, the front and the plan of one scale must give the whole asset the same extent along a shared axis (height: elevation and front, length: elevation and plan, width: front and plan) and the same place (both ends of the interval it covers along that axis, `positionDifferenceM:<axis>`: a view drawn 50 mm aside has the right extent and the wrong place) within max(one smallest feature, 2 % of the larger extent); a finding names the axis and the two extents or intervals, and the 2D drawing is to be repaired, no 3D model can agree with both. Open strokes and unfilled outlines take part widened by half their pen (as in the silhouette comparison). The per-bone differences are measurements named `info:extentDifferenceM:<bone>:<axis>` and `info:positionDifferenceM:<bone>:<axis>` that never fail: a bone is legitimately drawn with other parts in another view (a tail in one shape of the last tail bone in the side view, the stair treads on the column), measured on twelve catalog assets (2026-11-22: 7 of 12 would fail per bone, 4 of 12 per shape id, none on the whole asset). */
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
    if (asset.representations[`${d}`]?.status !== "authored") continue;
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
      const shift = c.measure({ scale: d, list: "body", name: `positionDifferenceM:${axis}`, value: Math.max(Math.abs(p.lo - q.lo), Math.abs(p.hi - q.hi)), limit, bound: "max" });
      if (m.ok && !shift.ok) c.fail(`representations.${d}`, `The ${first} and the ${second} of 1:${d} place the asset differently along its ${names[axis]}: ${(p.lo * 1000).toFixed(0)} to ${(p.hi * 1000).toFixed(0)} mm against ${(q.lo * 1000).toFixed(0)} to ${(q.hi * 1000).toFixed(0)} mm (an edge ${mm(shift.value)} apart, more than ${mm(shift.limit)}); repair the 2D drawing, no 3D model can agree with both.`, `Die ${first} und die ${second} von 1:${d} legen das Asset entlang ${axis === "y" ? "der Höhe" : axis === "x" ? "der Länge" : "der Breite"} verschieden: ${(p.lo * 1000).toFixed(0)} bis ${(p.hi * 1000).toFixed(0)} mm gegen ${(q.lo * 1000).toFixed(0)} bis ${(q.hi * 1000).toFixed(0)} mm (eine Kante ${mm(shift.value)} auseinander, mehr als ${mm(shift.limit)}); die 2D-Zeichnung ist zu korrigieren, kein 3D-Modell kann beiden folgen.`, { scale: d, list: "body", code: "volumes-views-disagree", measurements: [shift], data: { axis, first, second, firstM: [p.lo, p.hi], secondM: [q.lo, q.hi] } });
    }
    for (const bone of bones)
      for (const [axis, first, second] of pairs) {
        const [p, q] = [perBone.get(`${bone}/${first}/${axis}`), perBone.get(`${bone}/${second}/${axis}`)];
        if (p === undefined || q === undefined) continue;
        c.measure({ scale: d, list: "body", name: `info:extentDifferenceM:${bone}:${axis}`, value: Math.abs(p.hi - p.lo - (q.hi - q.lo)), limit: Number.MAX_VALUE, bound: "max" });
        c.measure({ scale: d, list: "body", name: `info:positionDifferenceM:${bone}:${axis}`, value: Math.max(Math.abs(p.lo - q.lo), Math.abs(p.hi - q.hi)), limit: Number.MAX_VALUE, bound: "max" });
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

/** 🦴️ The pairs of bones that a movable joint of the rig holds together, for the bones that carry solids: a bone and its nearest ancestor with solids when a joint on the way between them is not `fixed` (a hinge, a slide, a ball: a mechanical connection that the solids must show; a `fixed` joint only places a bone, so a second object standing beside the first is fixed to the root and is not tied). */
export function jointTies(asset: Asset, carrying: ReadonlySet<string>): readonly (readonly [string, string])[] {
  const bones = new Map(asset.rig.bones.map((bone) => [bone.id, bone as { readonly id: string; readonly parent?: string; readonly joint?: { readonly type?: string } }] as const));
  const ties: (readonly [string, string])[] = [];
  for (const id of carrying) {
    let bone = bones.get(id);
    let movable = false;
    for (let guard = 0; bone !== undefined && guard < bones.size; guard++) {
      movable ||= bone.joint?.type !== undefined && bone.joint.type !== "fixed";
      const parent = bone.parent === undefined || bone.parent === null ? undefined : bones.get(bone.parent);
      if (parent === undefined) break;
      if (carrying.has(parent.id)) {
        if (movable) ties.push([id, parent.id]);
        break;
      }
      bone = parent;
    }
  }
  return ties;
}

/** 🔗️ `volumes-connected` for one list of one scale in the evaluated pose. Parts are the copies of the solids ({@link partsOf}); two touch when their surfaces come within {@link VOLUME_LIMITS}.connectedToleranceFeatures smallest features of each other or one lies inside the other. Body: every group of touching parts apart from the largest must rest on something outside the list, the ground plane (y = 0) or a `contact` or `support` anchor of the pose, else it floats; a group that rests on its own is a separate object only when no movable joint of the rig ties its bones to bones whose solids all lie in other groups ({@link jointTies}: a wheel on its axle, a shoe on its shank; `bonesOf` gives the bones of a part) and when it clears every other group by more than {@link VOLUME_LIMITS}.connectedApartFeatures features, because no drawing of the scale shows a narrower gap, so it is a part that missed its joint. Structure: every group that is not only `mass` markers (they sit at centres of mass) must contain a `support` solid or rest on the ground or at such an anchor (many assets declare no `structure.supports` and stand on contact anchors), directly or through one solid of `body` that itself stands there and that the part touches or lies in (the sole under a foot bone, the tyre around a rim, the glide under a leg; one solid, never the body as a whole), else its load path is open; in a list without any `support` solid (an idealised skeleton in the sagittal plane) the largest group is the structure itself and every other group must join it; supported groups keep the same clearance. Nothing an author declares switches a part off: there is no flag, role or tag that the rule reads. The measurements are `looseGroups:<reason>` (limit 0) and the information `info:groups`; each finding names the worst groups by their first solid, the number of further solids, the gap and the nearest solid, which in the structure list is never a `mass` marker (a mass is no load path, so it is neither measured to nor named). */
export function connectedRule(c: ReturnType<typeof collector>, d: ScaleDenominator, list: "body" | "structure", parts: readonly Part[], feature: number, anchors: readonly Vec[], kinds: ReadonlyMap<string, string>, tick: () => void = () => {}, ties: readonly (readonly [string, string])[] = [], bonesOf: (part: Part) => readonly string[] = () => [], body: readonly Part[] = []): void {
  const tolerance = VOLUME_LIMITS.connectedToleranceFeatures * feature;
  const [apart, cap] = [VOLUME_LIMITS.connectedApartFeatures * feature, VOLUME_LIMITS.connectedGapCapFeatures * feature];
  const direct = (part: Part): boolean => part.min[1] <= tolerance || anchors.some((a) => [0, 1, 2].every((k) => a[k]! >= part.min[k]! - feature && a[k]! <= part.max[k]! + feature));
  const carriers = list === "structure" ? body.filter(direct) : [];
  const rests = (part: Part): boolean => direct(part) || carriers.some((carrier) => partGap(part, carrier, tolerance, true) <= tolerance);
  const tie = list !== "body" || ties.length === 0 ? undefined : (own: readonly Part[], rest: readonly Part[], groupOf: (part: Part) => number): string | undefined => {
    const [mine, theirs] = [new Set(own.flatMap(bonesOf)), new Map<string, Set<number>>()];
    for (const part of rest) for (const bone of bonesOf(part)) theirs.set(bone, (theirs.get(bone) ?? new Set()).add(groupOf(part)));
    const g = groupOf(own[0]!);
    for (const [child, parent] of ties)
      for (const [here, there] of [[child, parent], [parent, child]] as const) {
        if (!mine.has(here) || mine.has(there) || !theirs.has(there)) continue;
        const shared = [...(theirs.get(here) ?? [])].some((group) => theirs.get(there)!.has(group));
        if (!shared && g !== 0) return `${child} — ${parent}`;
      }
    return undefined;
  };
  const { groups, loose } = looseGroups(parts, { toleranceM: tolerance, apartM: apart, capM: cap, rests, structure: list === "structure", kind: (part) => (kinds.get(part.id) === "mass" ? "mass" : kinds.get(part.id) === "support" ? "support" : undefined), tick, ...(tie === undefined ? {} : { tie }) });
  c.measure({ scale: d, list, name: "info:groups", value: groups, limit: Number.MAX_VALUE, bound: "max" });
  const named = (items: readonly Loose[], lang: "en" | "de"): string => items.slice(0, 4).map((x) => `"${x.ids[0]}"${x.ids.length > 1 ? ` (+${x.ids.length - 1})` : x.parts > 1 ? ` (${x.parts}×)` : ""} ${x.gapM >= cap ? (lang === "en" ? "at least " : "mindestens ") : ""}${mm(x.gapM)}${x.nearest === undefined ? "" : `${lang === "en" ? " from" : " von"} "${x.nearest}"`}${x.tie === undefined ? "" : `${lang === "en" ? ", joint" : ", Gelenk"} ${x.tie}`}`).join(", ") + (items.length > 4 ? (lang === "en" ? ` and ${items.length - 4} more` : ` und ${items.length - 4} weitere`) : "");
  const s = (n: number): string => (n === 1 ? "" : "s");
  const texts: Record<Loose["reason"], (n: number, items: readonly Loose[]) => readonly [string, string]> = {
    floating: (n, items) => [`The body of 1:${d} is not one connected object: ${n} group${s(n)} of solids touch${n === 1 ? "es" : ""} neither the rest of the body nor the ground nor a contact or support anchor (contact tolerance ${mm(tolerance)}); worst: ${named(items, "en")}. Join every part to what carries it.`, `Der Body von 1:${d} ist kein zusammenhängender Gegenstand: ${n} Gruppe${n === 1 ? "" : "n"} von Körpern berühr${n === 1 ? "t" : "en"} weder den übrigen Body noch den Boden noch einen Kontakt- oder Auflageranker (Berührtoleranz ${mm(tolerance)}); am schlimmsten: ${named(items, "de")}. Jedes Teil ist mit dem zu verbinden, was es trägt.`],
    unsupported: (n, items) => [`The structure of 1:${d} has an open load path: ${n} group${s(n)} of members and joints reach${n === 1 ? "es" : ""} neither a support solid nor the ground nor a contact or support anchor (directly or through one body solid that stands there) through touching solids, nor, in a list without support solids, the largest group (contact tolerance ${mm(tolerance)}); worst: ${named(items, "en")}. Members meet at their joints and the path ends in a support.`, `Das Tragwerk von 1:${d} hat einen offenen Lastpfad: ${n} Gruppe${n === 1 ? "" : "n"} von Stäben und Gelenken erreich${n === 1 ? "t" : "en"} über sich berührende Körper weder einen Auflagerkörper noch den Boden noch einen Kontakt- oder Auflageranker (direkt oder über einen dort stehenden Bodykörper) und, in einer Liste ohne Auflagerkörper, auch nicht die größte Gruppe (Berührtoleranz ${mm(tolerance)}); am schlimmsten: ${named(items, "de")}. Stäbe treffen sich an ihren Gelenken und der Pfad endet in einem Auflager.`],
    detached: (n, items) => [`${n} group${s(n)} of body solids of 1:${d} stand${n === 1 ? "s" : ""} on the ground or at an anchor but do${n === 1 ? "es" : ""} not touch the solids of the bone that a movable joint of the rig ties ${n === 1 ? "it" : "them"} to (contact tolerance ${mm(tolerance)}): ${named(items, "en")}. What a joint holds together meets at that joint.`, `${n} Gruppe${n === 1 ? "" : "n"} von Bodykörpern von 1:${d} steh${n === 1 ? "t" : "en"} auf dem Boden oder an einem Anker, berühr${n === 1 ? "t" : "en"} aber nicht die Körper des Knochens, mit dem ein bewegliches Gelenk des Rigs sie verbindet (Berührtoleranz ${mm(tolerance)}): ${named(items, "de")}. Was ein Gelenk zusammenhält, trifft sich an diesem Gelenk.`],
    near: (n, items) => list === "body"
      ? [`${n} group${s(n)} of body solids of 1:${d} stand${n === 1 ? "s" : ""} on ${n === 1 ? "its" : "their"} own (on the ground or at an anchor) but within ${mm(apart)} (one smallest feature) of the rest without touching it (contact tolerance ${mm(tolerance)}), a gap no drawing of this scale shows: ${named(items, "en")}. Join them or set them clearly apart.`, `${n} Gruppe${n === 1 ? "" : "n"} von Bodykörpern von 1:${d} steh${n === 1 ? "t" : "en"} für sich (auf dem Boden oder an einem Anker), aber näher als ${mm(apart)} (ein kleinstes Merkmal) am Rest, ohne ihn zu berühren (Berührtoleranz ${mm(tolerance)}), ein Spalt, den keine Zeichnung dieses Maßstabs zeigt: ${named(items, "de")}. Sie sind zu verbinden oder deutlich abzusetzen.`]
      : [`${n} group${s(n)} of members and joints of the structure of 1:${d} ${n === 1 ? "has its" : "have their"} own load path to a support solid, the ground or an anchor but pass${n === 1 ? "es" : ""} within ${mm(apart)} (one smallest feature) of the members, joints or supports of the rest of the structure without touching them (contact tolerance ${mm(tolerance)}), a gap no drawing of this scale shows: ${named(items, "en")}. Members that meet at a joint touch there; parts of the structure that do not meet stay clearly apart.`, `${n} Gruppe${n === 1 ? "" : "n"} von Stäben und Gelenken des Tragwerks von 1:${d} ha${n === 1 ? "t" : "ben"} einen eigenen Lastpfad zu einem Auflagerkörper, zum Boden oder zu einem Anker, komm${n === 1 ? "t" : "en"} aber den Stäben, Gelenken oder Auflagern des übrigen Tragwerks näher als ${mm(apart)} (ein kleinstes Merkmal), ohne sie zu berühren (Berührtoleranz ${mm(tolerance)}), ein Spalt, den keine Zeichnung dieses Maßstabs zeigt: ${named(items, "de")}. Stäbe, die sich an einem Gelenk treffen, berühren sich dort; Tragwerksteile, die sich nicht treffen, bleiben deutlich getrennt.`],
  };
  for (const reason of (list === "body" ? ["floating", "detached", "near"] : ["unsupported", "near"]) as readonly Loose["reason"][]) {
    const items = loose.filter((x) => x.reason === reason);
    const m = c.measure({ scale: d, list, name: `looseGroups:${reason}`, value: items.length, limit: 0, bound: "max" });
    if (m.ok) continue;
    const [en, de] = texts[reason](items.length, items);
    c.fail(`volumes.${d}.${list}`, en, de, { scale: d, list, measurements: [m], data: { reason, solids: items.flatMap((x) => x.ids), gapsM: items.map((x) => x.gapM.toFixed(4)) } });
  }
}

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
  if (rules.has("volumes-slender")) results.push(slenderRule(asset, only, minFeatureMm));
  if (rules.has("volumes-views-disagree")) results.push(viewsDisagreeRule(asset, authoredVolumes(asset).filter((v) => options.scale === undefined || v.denominator === options.scale), (d) => evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: d } }), minFeatureMm));
  const issues = volumeIssues(asset) as readonly Issue[];
  for (const issue of issues) {
    const scale = Number(/^volumes\.(\d+)/.exec(issue.path)?.[1]) as ScaleDenominator;
    if (options.scale !== undefined && scale !== options.scale) continue;
    const list = /\.(body|thermal|structure)\b/.exec(issue.path)?.[1] as VolumeList | undefined;
    const mapped: VolumeRule = issue.code === "volume-region" || issue.code === "volume-layer" ? "volumes-thermal-coverage" : issue.code === "volume-member" || issue.code === "volume-anchor" ? "volumes-structure-coverage" : issue.code === "volume-duplicate-id" || issue.code === "volume-of" ? "volumes-bones" : "volumes-solids";
    acc.findings.push({ ...issue, rule: mapped, ...(Number.isFinite(scale) ? { scale } : {}), ...(list === undefined ? {} : { list }), measurements: [] });
  }
  const geometric = ["volumes-silhouette", "volumes-bounds", "volumes-monotonic", "volumes-cross-scale", "volumes-thermal-coverage", "volumes-structure-coverage", "volumes-connected"].some((rule) => rules.has(rule as VolumeRule));
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
    const connected = collector("volumes-connected");
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
        hit = evaluate(asset, { kind: "pose", pose }, asset.representations[`${denominator}`]?.status === "authored" ? { kind: "scale", scale: { denominator } } : undefined);
        stateAt.set(denominator, hit);
      }
      return hit;
    };
    const worldAt = (volume: AuthoredVolume, list: VolumeList): readonly World[] => {
      const key = `${volume.denominator}:${list}`;
      let hit = cache.get(key);
      if (hit === undefined) {
        hit = world(asset, stateOf(volume.denominator).bones, volume.denominator, list, list === "body" ? undefined : worldAt(volume, "body"));
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
        const body = worldAt(volume, "body");
        const bbox = bodyDimensions(body);
        if (rules.has("volumes-bounds"))
          (["widthM", "heightM", "depthM"] as const).forEach((name, k) => {
            const want = asset.dimensions[name];
            const got = bbox[k]!;
            const limit = Math.max(VOLUME_LIMITS.boundsFeatures * feature, VOLUME_LIMITS.boundsFraction * want);
            const measured = bounds.measure({ scale: d, list: "body", name: `bboxDeviationM:${name}`, value: Math.abs(got - want), limit, bound: "max" });
            if (!measured.ok) bounds.fail(`volumes.${d}.body`, `The body of 1:${d} is ${got.toFixed(3)} m along ${name} but dimensions say ${want} m (limit ${mm(measured.limit)}).`, `Der Body von 1:${d} misst ${got.toFixed(3)} m entlang ${name}, dimensions nennen ${want} m (Grenze ${mm(measured.limit)}).`, { scale: d, list: "body", measurements: [measured] });
          });
        if (asset.representations[`${d}`]?.status === "authored" && rules.has("volumes-silhouette")) {
          const softIds = softIdsOf(asset, volume);
          for (const view of VIEWPOINTS) {
            const v = measureBodyView(asset, stateOf(d), body, view, d, feature, softIds);
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
        const probe = Math.max(1e-4, 0.02 * feature);
        const seam = SEAM_FEATURES * feature;
        let flooded: Outside | undefined;
        const walls = worlds.filter((w) => !porous.has(w.id) && w.envelope !== true && w.emitter !== true);
        const air = (): Outside => (flooded ??= outsideOf(walls, feature, probe, () => operation.signal?.throwIfAborted(), asset.thermal.minimumAirPassageM));
        const transparent = new Set(volume.thermal.filter((s) => (VOLUME_LIMITS.transparentRegionKinds as readonly string[]).includes(asset.thermal.regions.find((r) => r.id === (s as { region: Slug }).region)?.kind ?? "")).map((s) => s.id));
        let lit: ((base: readonly [number, number, number], normal: readonly [number, number, number]) => Reach | undefined) | undefined;
        const light = (): NonNullable<typeof lit> => (lit ??= seenFromOutside(walls, (w) => transparent.has(w.id), feature, probe, () => operation.signal?.throwIfAborted(), asset.thermal.minimumAirPassageM));
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
          const reached = hidden ? undefined : rememberedReach((base, normal) => air().reach(base, normal, (solid) => ownWorlds.has(solid)));
          const shares = exposedShares(picked, hidden ? [] : picked, [], () => false, probe, seam, reached);
          const area = outerOfShares(shares);
          const contents = isContents(shares);
          if (!hidden && contents && shares.air + shares.cover < VOLUME_LIMITS.enclosedOuterFraction * area) {
            const seen = transparent.size === 0 ? 0 : exposedArea(picked, picked, [], () => false, probe, seam, light());
            if (seen < VOLUME_LIMITS.enclosedOuterFraction * area)
              thermal.fail(`volumes.${d}.thermal`, `Thermal region "${region.id}" at 1:${d} lies wholly in a closed cavity that no glazing lets one see into: declare it \`hidden\` (B4, C31, E3).`, `Die thermische Region "${region.id}" liegt bei 1:${d} ganz in einem geschlossenen Hohlraum, in den keine Verglasung blicken lässt: sie ist als \`hidden\` zu deklarieren (B4, C31, E3).`, { scale: d, list: "thermal", measurements: [], data: { region: region.id } });
          }
          const exposed = exposedArea(picked, against, planes, (other) => ownWorlds.has(other), probe, seam, reached === undefined ? undefined : (base, normal, sample) => {
            const why = reached(base, normal, sample);
            return why === "cavity" && !contents ? undefined : why;
          });
          const binding = (asset.poses as readonly { readonly id: Slug; readonly thermal: { readonly regions: Readonly<Record<string, { readonly exposedFraction: number; readonly contact?: { readonly fraction: number } }>> } }[]).find((p) => p.id === pose)?.thermal.regions[region.id];
          const target = binding === undefined ? 1 : binding.exposedFraction * (1 - (binding.contact?.fraction ?? 0));
          const info = (name: string, value: number): void => void thermal.measure({ scale: d, list: "thermal", name: `info:${name}:${region.id}`, value, limit: Number.MAX_VALUE, bound: "max" });
          const declaredSurface = region.areaBasis === "total" ? exposedArea(picked, picked, [], () => false, probe, seam) : area;
          if (region.areaBasis === "total") info("totalAreaM2", declaredSurface);
          const small = region.areaM2 < VOLUME_LIMITS.smallRegionFeatures * feature * feature;
          if (!small) sumArea += declaredSurface;
          info("outerAreaM2", area);
          info("exposedAreaM2", exposed);
          if (contents) info("cavityAreaM2", shares.cavity);
          info("exposureMeasured", declaredSurface > 0 ? exposed / declaredSurface : 0);
          info("exposureTarget", target);
          info("exposureDeviation", declaredSurface > 0 ? Math.abs(exposed / declaredSurface - target) : 0);
          if (!small) sumDeclared += region.areaM2;
          const deviation = thermal.measure({ scale: d, list: "thermal", name: `areaDeviation:${region.id}`, value: Math.abs(declaredSurface / region.areaM2 - 1), limit: perRegion || small ? Number.MAX_VALUE : (VOLUME_LIMITS.bodyAreaModels as readonly string[]).includes(region.model) ? VOLUME_LIMITS.thermalAreaCoarse : tolerance, bound: "max" });
          if (!deviation.ok) thermal.fail(`volumes.${d}.thermal`, `The ${region.areaBasis === "total" ? "total component surface" : "outer surface"} of region "${region.id}" at 1:${d} is ${declaredSurface.toFixed(3)} m² against areaM2 ${region.areaM2} m² (deviation ${(deviation.value * 100).toFixed(0)} %, limit ${(deviation.limit * 100).toFixed(0)} %).`, `Die ${region.areaBasis === "total" ? "gesamte Bauteiloberfläche" : "Außenfläche"} der Region "${region.id}" bei 1:${d} beträgt ${declaredSurface.toFixed(3)} m² gegen areaM2 ${region.areaM2} m² (Abweichung ${(deviation.value * 100).toFixed(0)} %, Grenze ${(deviation.limit * 100).toFixed(0)} %).`, { scale: d, list: "thermal", measurements: [deviation], data: { region: region.id } });
        }
        if (flooded !== undefined) thermal.measure({ scale: d, list: "thermal", name: "info:floodCellM", value: flooded.cellM, limit: Number.MAX_VALUE, bound: "max" });
        const passage = asset.thermal.minimumAirPassageM;
        if (flooded !== undefined && passage !== undefined && passage > 0) {
          const resolved = thermal.measure({ scale: d, list: "thermal", name: "airPassageCellM", value: flooded.cellM, limit: (passage / 3) * (1 + 1e-9), bound: "max" });
          if (!resolved.ok) thermal.fail(`volumes.${d}.thermal`, `The flood of 1:${d} cannot resolve the declared thermal.minimumAirPassageM of ${mm(passage)}: its cell is ${mm(flooded.cellM)} (at most a third of the passage) because the asset is too large for a passage this narrow, so the outer areas of this scale are measured with that passage closed.`, `Die Flutung von 1:${d} kann die deklarierte thermal.minimumAirPassageM von ${mm(passage)} nicht auflösen: ihre Zelle misst ${mm(flooded.cellM)} (höchstens ein Drittel des Durchlasses), weil das Asset für einen so engen Durchlass zu groß ist; die Außenflächen dieses Maßstabs sind daher mit geschlossenem Durchlass gemessen.`, { scale: d, list: "thermal", measurements: [resolved] });
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
          const { bodyMask: a, faces, members: projectedMembers } = memberProjection(body, needBody, view, feature);
          let exact: ReturnType<typeof projectedBodyDistance> | undefined;
          const measured = new WeakMap<Point, number>();
          const field = distanceField(a.cells, a.width, a.height);
          const reach = (groups: readonly Group[]): number => {
            let worst = 0;
            for (const g of groups) for (const r of g) for (const p of r) {
              const [i, j] = [Math.floor((p.x - a.box.minX) / a.cell), Math.floor((p.y - a.box.minY) / a.cell)];
              if (i < 0 || j < 0 || i >= a.width || j >= a.height) continue;
              let outside = Math.max(0, (Math.sqrt(field[j * a.width + i]!) - 1) * a.cell);
              if (outside > feature) {
                exact ??= projectedBodyDistance(faces, a.box);
                outside = measured.get(p) ?? exact(p, outside);
                measured.set(p, outside);
              }
              worst = Math.max(worst, outside);
            }
            return worst;
          };
          const worst = reach(projectedMembers);
          const m = structure.measure({ scale: d, list: "structure", name: `outsideBodyM:${view}`, value: worst, limit: feature, bound: "max" });
          if (!m.ok) {
            const culprits = needBody.map((w) => ({ id: w.id, outsideM: reach(referenceOf(view, [w], undefined, undefined, "mesh").cut) })).filter((x) => x.outsideM > feature).sort((p, q) => q.outsideM - p.outsideM).slice(0, 3);
            const named = culprits.map((x) => `"${x.id}" ${mm(x.outsideM)}`).join(", ");
            structure.fail(`volumes.${d}.structure`, `Member solids of 1:${d} lie ${mm(m.value)} outside the body silhouette in the ${view} (limit ${mm(feature)}); the worst are ${named}.`, `Stabkörper von 1:${d} liegen ${mm(m.value)} außerhalb der Body-Silhouette in der ${view} (Grenze ${mm(feature)}); am weitesten außen liegen ${named}.`, { scale: d, list: "structure", measurements: [m], data: { view, solids: culprits.map((x) => x.id) } });
          }
        }
        void members;
        for (const solid of volume.structure) {
          const spanned = (solid as { members?: readonly Slug[] }).members;
          const w = worlds.find((x) => x.id === solid.id);
          if ((solid as { part: string }).part !== "member" || !Array.isArray(spanned) || w === undefined) continue;
          for (const bone of spanned) {
            const b = state.bones[bone];
            if (b === undefined) continue;
            const m = structure.measure({ scale: d, list: "structure", name: `spanGapM:${solid.id}:${bone}`, value: spanGap(w, [b.frame.o.x, b.frame.o.y, b.frame.o.z], [b.tip.x, b.tip.y, b.tip.z], (point) => insideAny([w], point)), limit: VOLUME_LIMITS.memberSpanFeatures * feature, bound: "max" });
            if (!m.ok) structure.fail(`volumes.${d}.structure[${volume.structure.indexOf(solid)}].members`, `Member solid "${solid.id}" of 1:${d} names bone "${bone}" in \`members\` but does not hold it: the bone runs up to ${mm(m.value)} away from the solid (limit ${mm(m.limit)}, one smallest feature). A member solid spans only bones that lie in it or beside it from joint to tip; give this bone its own member solid or let the solid run along it.`, `Stabkörper "${solid.id}" von 1:${d} nennt den Knochen "${bone}" in \`members\`, fasst ihn aber nicht: der Knochen verläuft bis zu ${mm(m.value)} vom Körper entfernt (Grenze ${mm(m.limit)}, ein kleinstes Merkmal). Ein Stabkörper überspannt nur Knochen, die vom Gelenk bis zur Spitze in ihm oder an ihm liegen; dieser Knochen braucht einen eigenen Stabkörper, oder der Körper läuft an ihm entlang.`, { scale: d, list: "structure", measurements: [m], data: { solid: solid.id, bone } });
          }
        }
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
      if (rules.has("volumes-connected")) {
        const anchors = Object.values(stateOf(d).anchors).filter((entry) => entry.anchor.kind === "contact" || entry.anchor.kind === "support").map((entry) => [entry.at.x, entry.at.y, entry.z] as const);
        const kinds = new Map(volume.structure.map((s) => [s.id, (s as { part: string }).part] as const));
        let bodyParts: readonly Part[] = [];
        for (const list of ["body", "structure"] as const) {
          const solids = solidsOf(volume, list);
          const bonesOf = (part: Part): readonly string[] => [solids[part.solid]!.bone, ...(solids[part.solid]!.weights ?? []).map((w) => w.bone)];
          const ties = list === "body" ? jointTies(asset, new Set(solids.flatMap((solid) => [solid.bone, ...(solid.weights ?? []).map((w) => w.bone)]))) : [];
          const parts = partsOf(worldAt(volume, list), solids.map((solid) => solid.repeat?.count ?? 1));
          if (list === "body") bodyParts = parts;
          connectedRule(connected, d, list, parts, feature, anchors, kinds, () => operation.signal?.throwIfAborted(), ties, bonesOf, bodyParts);
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
        const hullBones = new Set(coarseAll.filter((w) => w.envelope === true).map((w) => w.bone ?? ""));
        const compareDrawings = asset.representations[`${d}`]?.status === "authored" && asset.representations[`${finer.denominator}`]?.status === "authored";
        const physicalProjection = (solids: readonly World[]) => compareDrawings ? solids : solids.map(solid => solid.slender ? { ...solid, slender: false } : solid);
        const [projectedCoarse, projectedFine] = [physicalProjection(coarseAll), physicalProjection(fineAll)];
        const [stateCoarse, stateFine] = [stateOf(d), stateOf(finer.denominator)];
        const drawnGroups = (state: ReturnType<typeof evaluate>, view: Viewpoint): readonly Group[] =>
          (state.shapes[view] as readonly { readonly shape: { readonly role: string; readonly filled?: boolean }; readonly rings: readonly (readonly { readonly x: number; readonly y: number }[])[]; readonly closed: boolean }[]).filter((item) => item.closed && item.shape.filled !== false && ["body", "detail", "glazing"].includes(item.shape.role)).map((item) => item.rings as Group);
        for (const view of VIEWPOINTS) {
          operation.signal?.throwIfAborted();
          const pair = compareDrawings ? compareGroups(drawnGroups(stateCoarse, view), drawnGroups(stateFine, view), feature) : undefined;
          const limitM = Math.max(VOLUME_LIMITS.crossDistanceFeatures * feature, (pair?.p95M ?? 0) + VOLUME_LIMITS.crossDistancePlus2DFeatures * feature);
          const agreement = silhouetteAgreement(projectedCoarse, projectedFine, view, feature, VOLUME_LIMITS.crossToleranceFeatures * feature, { hullBones, limitM, ...(compareDrawings ? { drawnFirst: drawnFor(asset, stateCoarse, view, d), drawnSecond: drawnFor(asset, stateFine, view, finer.denominator) } : {}) });
          const iou = cross.measure({ scale: d, list: "body", name: `iou:${view}`, value: agreement.iou, limit: pair ? Math.min(VOLUME_LIMITS.crossIou, pair.rawIou - VOLUME_LIMITS.crossIouMarginTo2D) : VOLUME_LIMITS.crossIou, bound: "min" });
          const dist = cross.measure({ scale: d, list: "body", name: `p95M:${view}`, value: agreement.p95M, limit: limitM, bound: "max" });
          cross.measure({ scale: d, list: "body", name: `hausdorffM:${view}`, value: agreement.hausdorffM, limit: Number.MAX_VALUE, bound: "max" });
          if (pair) cross.measure({ scale: d, list: "body", name: `iou2D:${view}`, value: pair.rawIou, limit: 0, bound: "min" });
          if (pair) cross.measure({ scale: d, list: "body", name: `p952DM:${view}`, value: pair.p95M, limit: Number.MAX_VALUE, bound: "max" });
          if (!iou.ok || !dist.ok) cross.fail(`volumes.${d}.body`, `The body of 1:${d} disagrees with 1:${finer.denominator} in the ${view}: IoU ${iou.value.toFixed(3)} (at least ${iou.limit.toFixed(3)}), boundary distance p95 ${mm(dist.value)} (at most ${mm(dist.limit)}).`, `Der Body von 1:${d} weicht von 1:${finer.denominator} in der ${view} ab: IoU ${iou.value.toFixed(3)} (mindestens ${iou.limit.toFixed(3)}), Randabstand p95 ${mm(dist.value)} (höchstens ${mm(dist.limit)}).`, { scale: d, list: "body", measurements: [iou, dist].filter((x) => !x.ok) });
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
    for (const result of [sil, bounds, mono, cross, thermal, structure, connected]) results.push(result);
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
