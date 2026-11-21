/** 🧱️ The analytic rules of the volumes of an asset: those that need no tessellation and run inside `validateAsset` as well as inside `checkVolumes`.
 *
 * `volumes-availability` (authored at every scale with a drawing), `volumes-bones` (every bone with drawn 2D shapes has a body solid at that scale), `volumes-feature` (no solid dimension below the
 * smallest feature of the scale, except thin sheets of a closed list of roles), `volumes-thermal-coverage` (every region, layer shell, emitter has a solid; hidden regions never reuse a body solid) and
 * `volumes-structure-coverage` (every eligible bone of the structural view, every 2D member shape and every support anchor has exactly one support solid) and `volumes-copied` (no authored scale is an identical, subset or uniformly scaled copy of a finer one). The thresholds are in {@link VOLUME_LIMITS}; the design is
 * `PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/design.md` (rules 1, 4, 7-9, amendments B1-B4).
 *
 * @see ../../🧊️volume/🟦️.ts — the per-solid rules
 * @see ../🟦️.ts — `checkVolumes`, the superset with geometry */
import type { Asset, AuthoredVolume, Respiration, ScaleDenominator, Slug, StructuralSolid, Text, ThermalSolid, VolumeList, VolumeSolid } from "../../../../🧬️schema/🟦️.ts";
import { polyhedronExtents } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { SCALE_DENOMINATORS } from "../../../../🧬️schema/🟦️.ts";
import { flattenGeometry, solidDrawingProjector } from "../../../🧮️evaluation/🟦️.ts";
import { eligibleBones } from "../../../🎨️depiction/🏗️structural-view/🟦️.ts";
import { authoredVolumes, isReference, solidsOf } from "../🧱️lists/🟦️.ts";
import { centreLineOf } from "../🧵️centreline/🟦️.ts";
import { copyMesh, extentOf, repeatMaps } from "../🌓️silhouette/🟦️.ts";
import { qualityOf } from "../../../🧊️volume/⏱️job/🟦️.ts";

/** 📜️ The rule ids of the volumes. `volumes-solids` is the per-solid family (`volume-*` codes), `volumes-copied` the 3D twin of `representation-copied`, `volumes-slender` the check of the `slender` declaration, `volumes-views-disagree` the agreement of the 2D viewpoints with each other. */
export const VOLUME_RULES = ["volumes-availability", "volumes-silhouette", "volumes-bounds", "volumes-feature", "volumes-monotonic", "volumes-cross-scale", "volumes-thermal-coverage", "volumes-structure-coverage", "volumes-bones", "volumes-solids", "volumes-copied", "volumes-slender", "volumes-views-disagree"] as const;

/** 🏷️ One of {@link VOLUME_RULES}. */
export type VolumeRule = (typeof VOLUME_RULES)[number];

/** 🚦️ The thresholds of the design, never weakened silently: silhouette IoU and Hausdorff (in smallest features), bounds (in features), cross-scale IoU, Hausdorff and union volume change, thermal area tolerance at 1:50 and at coarser scales, the margin of the thermal bounding box (in features), the roles of thin sheets, the decision prefix that declares a scale already minimal, the region kinds that are transparent closures of a cavity and the share of its outer area below which a region counts as lying wholly in a closed cavity (E3). */
export const VOLUME_LIMITS = {
  silhouetteIouElevation: 0.9,
  silhouetteIouOther: 0.88,
  silhouetteP95FeaturesElevation: 2,
  silhouetteP95FeaturesOther: 4,
  softMassExtentFraction: 0.02,
  softRegionKinds: ["foliage", "fur", "feather", "hair"],
  areaBasisRequiredModels: ["leaf"],
  areaBasisRequiredKinds: ["foliage", "fur", "feather", "hair"],
  bodyAreaModels: ["body"],
  smallRegionFeatures: 6,
  boundsFeatures: 1,
  boundsFraction: 0.02,
  viewsDisagreeFeatures: 1,
  crossIou: 0.85,
  crossDistanceFeatures: 2,
  crossDistancePlus2DFeatures: 1,
  crossIouMarginTo2D: 0.05,
  crossToleranceFeatures: 1,
  crossVolumeBandFeatures: 1,
  thermalAreaFine: 0.15,
  thermalAreaCoarse: 0.3,
  thermalBoxFeatures: 1,
  thinRoles: ["glazing", "texture", "ground"],
  minimalDecision: "Minimal:",
  minFeatureMm: 0.15,
  slenderRatio: 20,
  monotonicChordM: 0.002,
  monotonicSegments: 32,
  sectionAngleDeg: 45,
  transparentRegionKinds: ["glazing"],
  enclosedOuterFraction: 0.02,
} as const;

/** 📐️ One measured quantity of a rule against its limit: `bound` "min" means the value must be at least the limit, "max" at most. */
export type VolumeMeasurement = {
  readonly rule: VolumeRule;
  readonly scale?: ScaleDenominator;
  readonly list?: VolumeList;
  readonly name: string;
  readonly value: number;
  readonly limit: number;
  readonly bound: "min" | "max";
  readonly ok: boolean;
};

/** 🛑️ One finding of a rule: an issue with the rule, the scale, the list and the measurements that failed. */
export type VolumeFinding = {
  readonly path: string;
  readonly code: string;
  readonly message: Text;
  readonly data?: Readonly<Record<string, string | number | readonly string[]>>;
  readonly rule: VolumeRule;
  readonly scale?: ScaleDenominator;
  readonly list?: VolumeList;
  readonly measurements: readonly VolumeMeasurement[];
};

/** 🧾️ What a rule produces: every measurement and the findings. */
export type RuleResult = { readonly measurements: VolumeMeasurement[]; readonly findings: VolumeFinding[] };

/** 🆕️ An empty {@link RuleResult} with the helpers every rule uses. */
export function collector(rule: VolumeRule): RuleResult & {
  measure: (m: Omit<VolumeMeasurement, "rule" | "ok">) => VolumeMeasurement;
  fail: (path: string, en: string, de: string, extra?: { scale?: ScaleDenominator; list?: VolumeList; code?: string; measurements?: readonly VolumeMeasurement[]; data?: VolumeFinding["data"] }) => void;
} {
  const measurements: VolumeMeasurement[] = [];
  const findings: VolumeFinding[] = [];
  return {
    measurements,
    findings,
    measure(m) {
      const ok = m.bound === "min" ? m.value >= m.limit : m.value <= m.limit;
      const full = { rule, ...m, ok } as VolumeMeasurement;
      measurements.push(full);
      return full;
    },
    fail(path, en, de, extra = {}) {
      const { scale, list, code, measurements: failed, data } = extra;
      findings.push({ path, code: code ?? rule, message: { en, de }, rule, ...(scale === undefined ? {} : { scale }), ...(list === undefined ? {} : { list }), ...(data === undefined ? {} : { data }), measurements: failed ?? [] });
    },
  };
}

/** 📏️ The smallest feature in metres at a denominator: the printable feature in paper millimetres times the denominator. */
export const featureOf = (denominator: number, minFeatureMm: number = VOLUME_LIMITS.minFeatureMm): number => (minFeatureMm * denominator) / 1000;

type Extent = readonly number[];
const span = (values: readonly number[]): number => (values.length === 0 ? 0 : Math.max(...values) - Math.min(...values));

/** 🧮️ The three dimensions of a solid in metres, using the section path length for a swept loft, sorted ascending, scaled by the smallest factor of its `transform`. */
export function sortedExtents(solid: VolumeSolid): readonly [number, number, number] {
  let extents: Extent;
  switch (solid.kind) {
    case "box":
      extents = solid.size;
      break;
    case "ellipsoid":
      extents = solid.radii.map((r) => 2 * r);
      break;
    case "capsule": {
      const axis = Math.hypot(solid.p1[0] - solid.p0[0], solid.p1[1] - solid.p0[1], solid.p1[2] - solid.p0[2]);
      const wide = 2 * Math.max(solid.r0, solid.r1);
      extents = [axis + solid.r0 + solid.r1, wide, wide * (solid.lateralScale ?? 1)];
      break;
    }
    case "revolve": {
      const wide = 2 * Math.max(0, ...solid.profile.map((q) => q[1]));
      extents = [span(solid.profile.map((q) => q[0])), wide, wide * (solid.lateralScale ?? 1)];
      break;
    }
    case "extrude": {
      const taper = Math.max(solid.taperTo?.[0] ?? 1, solid.taperTo?.[1] ?? 1);
      extents = [solid.to - solid.from, span(solid.profile.map((q) => q[0])) * taper, span(solid.profile.map((q) => q[1])) * taper];
      break;
    }
    case "polyhedron":
      extents = polyhedronExtents(solid.vertices, solid.triangles[0]);
      break;
    case "loft": {
      let pathLength = 0;
      for (let i = 1; i < solid.sections.length; i++) {
        const [x, y, z] = solid.sections[i]!.at;
        const previous = solid.sections[i - 1]!.at;
        pathLength += Math.hypot(x - previous[0], y - previous[1], z - previous[2]);
      }
      extents = [pathLength, 2 * Math.max(...solid.sections.map((s) => s.rx)), 2 * Math.max(...solid.sections.map((s) => s.ry))];
      break;
    }
  }
  const factor = solid.transform?.scale === undefined ? 1 : Math.min(...solid.transform.scale);
  return extents.map((e) => e * factor).sort((p, q) => p - q) as unknown as readonly [number, number, number];
}

/** 🧮️ The smallest dimension of the shape of a solid in metres (the shortest of its bone-local extents, scaled by the smallest factor of its `transform`). */
export const smallestDimension = (solid: VolumeSolid): number => sortedExtents(solid)[0];

const scaleKeys = (denominators?: readonly ScaleDenominator[]): readonly ScaleDenominator[] => denominators ?? [...SCALE_DENOMINATORS].reverse();
const bonesOf = (solids: readonly VolumeSolid[]): Set<Slug> => new Set(solids.flatMap((solid) => [solid.bone, ...(solid.weights ?? []).map((w) => w.bone)]));

/** 🪞️ `volumes-availability`: authored at every scale with a drawing, and the stored `denominator` equals the key. */
export function availabilityRule(asset: Asset, scales?: readonly ScaleDenominator[]): RuleResult {
  const c = collector("volumes-availability");
  for (const denominator of scaleKeys(scales)) {
    const rep = asset.representations[`${denominator}`];
    const volume = (asset.volumes as Partial<Record<string, { status: string; denominator: number }>> | undefined)?.[`${denominator}`];
    const path = `volumes.${denominator}`;
    const status = c.measure({ scale: denominator, name: "statusMatches", value: volume !== undefined && (volume.status === "authored" || rep?.status === "unavailable") ? 1 : 0, limit: 1, bound: "min" });
    if (volume === undefined) c.fail(path, `The scale 1:${denominator} has no volume; every scale is decided.`, `Der Maßstab 1:${denominator} hat kein Volumen; jeder Maßstab ist entschieden.`, { scale: denominator, measurements: [status] });
    else if (rep?.status === "authored" && volume.status !== "authored") c.fail(`${path}.status`, `The volume of 1:${denominator} is ${volume.status} but its representation is ${rep.status}.`, `Das Volumen von 1:${denominator} ist ${volume.status === "authored" ? "erstellt" : "nicht verfügbar"}, seine Darstellung aber erstellt.`, { scale: denominator, measurements: [status] });
    else if (volume.denominator !== denominator) c.fail(`${path}.denominator`, `The volume under key ${denominator} states denominator ${volume.denominator}.`, `Das Volumen unter dem Schlüssel ${denominator} nennt den Nenner ${volume.denominator}.`, { scale: denominator });
  }
  return c;
}

/** 🦴️ `volumes-bones`: every bone with drawn 2D shapes at a scale has a body solid at that scale (rigid or through its weights). Unique ids and `of` references are the `volume-duplicate-id` and `volume-of` issues of the solid rules, reported under this rule by `checkVolumes`. */
export function bonesRule(asset: Asset, scales?: readonly ScaleDenominator[]): RuleResult {
  const c = collector("volumes-bones");
  for (const volume of authoredVolumes(asset)) {
    if (scales !== undefined && !scales.includes(volume.denominator)) continue;
    const rep = asset.representations[`${volume.denominator}`];
    if (rep === undefined || rep.status !== "authored") continue;
    const drawn = new Set([...rep.geometry.elevation, ...rep.geometry.front, ...rep.geometry.plan].map((shape) => shape.bone));
    const have = bonesOf(volume.body);
    const missing = [...drawn].filter((bone) => !have.has(bone));
    const m = c.measure({ scale: volume.denominator, list: "body", name: "bonesWithoutSolid", value: missing.length, limit: 0, bound: "max" });
    for (const bone of missing) c.fail(`volumes.${volume.denominator}.body`, `Bone "${bone}" has drawn 2D shapes at 1:${volume.denominator} but no body solid.`, `Der Knochen "${bone}" hat bei 1:${volume.denominator} gezeichnete 2D-Formen, aber keinen Körper.`, { scale: volume.denominator, list: "body", measurements: [m], data: { bone } });
  }
  return c;
}

/** 📐️ The smallest width of a point set over all directions (rotating calipers over its convex hull: the least over the hull edges of the extent across that edge), so a rotated capsule reads its true cross extent and not the box around it. */
export function minimumWidth(points: readonly { readonly x: number; readonly y: number }[]): number {
  const sorted = [...points].sort((p, q) => p.x - q.x || p.y - q.y);
  const turn = (o: { x: number; y: number }, a: { x: number; y: number }, b: { x: number; y: number }): number => (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
  const chain = (list: readonly { x: number; y: number }[]): { x: number; y: number }[] => {
    const out: { x: number; y: number }[] = [];
    for (const p of list) {
      while (out.length >= 2 && turn(out[out.length - 2]!, out[out.length - 1]!, p) <= 0) out.pop();
      out.push(p);
    }
    out.pop();
    return out;
  };
  const hull = sorted.length < 3 ? sorted : [...chain(sorted), ...chain([...sorted].reverse())];
  if (hull.length < 3) return 0;
  let best = Infinity;
  for (let i = 0; i < hull.length; i++) {
    const [a, b] = [hull[i]!, hull[(i + 1) % hull.length]!];
    const l = Math.hypot(b.x - a.x, b.y - a.y);
    if (l === 0) continue;
    let across = 0;
    for (const q of hull) across = Math.max(across, Math.abs((b.x - a.x) * (q.y - a.y) - (b.y - a.y) * (q.x - a.x)) / l);
    best = Math.min(best, across);
  }
  return best === Infinity ? 0 : best;
}

/** 🖋️ The thinnest filled 2D shape extent in metres that of the authored section or its paper projection for each bone (the least of the shorter side of the bounding box and the smallest width over all directions, {@link minimumWidth}, of every filled closed body, detail or glazing shape); bones without such a shape are absent. */
export function thinnestDrawn(asset: Asset, denominator: ScaleDenominator): ReadonlyMap<Slug, number> {
  const rep = asset.representations[`${denominator}`];
  const out = new Map<Slug, number>();
  if (rep === undefined || rep.status !== "authored") return out;
  let project: ReturnType<typeof solidDrawingProjector> | undefined;
  for (const view of ["elevation", "front", "plan"] as const) {
    for (const shape of rep.geometry[view]) {
      if (shape.kind === "member" || !shape.filled || !["body", "detail", "glazing"].includes(shape.role)) continue;
      const flat = flattenGeometry(shape);
      if (!flat.closed) continue;
      if (shape.solid) project ??= solidDrawingProjector(asset, { kind:"pose", pose:asset.presentation.defaultPose });
      const sectionWidth = minimumWidth(flat.rings.flat());
      const projectedWidth = shape.solid ? minimumWidth(project!(shape, flat.rings, view).flat()) : sectionWidth;
      const thin = projectedWidth > 1e-9 ? Math.min(sectionWidth, projectedWidth) : sectionWidth;
      if (thin > 1e-9) out.set(shape.bone, Math.min(out.get(shape.bone) ?? Infinity, thin));
    }
  }
  return out;
}

type Point3 = readonly [number, number, number];
const offset3 = (a: Point3, b: Point3): Point3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const dot3 = (a: Point3, b: Point3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];

/** 📍️ Whether a triangle intersects the sphere around an outlet centre, including its three edges. */
function nearTriangle(p: Point3, a: Point3, b: Point3, c: Point3, radius2: number): boolean {
  for (const [start, end] of [[a, b], [b, c], [c, a]] as const) {
    const edge = offset3(end, start), delta = offset3(p, start);
    const length2 = dot3(edge, edge), along = dot3(delta, edge);
    const t = Math.min(1, Math.max(0, along / (length2 || 1)));
    if (dot3(delta, delta) - 2 * t * along + t * t * length2 <= radius2) return true;
  }
  const ab = offset3(b, a), ac = offset3(c, a), ap = offset3(p, a);
  const xx = dot3(ab, ab), xy = dot3(ab, ac), yy = dot3(ac, ac), px = dot3(ap, ab), py = dot3(ap, ac);
  const det = xx * yy - xy * xy;
  if (det <= 0) return false;
  const u = (yy * px - xy * py) / det, v = (xx * py - xy * px) / det;
  return u >= 0 && v >= 0 && u + v <= 1 && dot3(ap, ap) - u * px - v * py <= radius2;
}

/** 👃️ A located source lies within one declared aperture diameter of its physical tissue boundary in the same bone frame. */
function onTissue(source: VolumeSolid, surfaces: readonly VolumeSolid[], diameter: number, quality: ReturnType<typeof qualityOf>): boolean {
  const bounds = extentOf([{id:source.id,...copyMesh(source, quality)}]);
  const mid: Point3 = [(bounds.min[0]+bounds.max[0])/2, (bounds.min[1]+bounds.max[1])/2, (bounds.min[2]+bounds.max[2])/2];
  const centres: Point3[] = source.repeat ? repeatMaps(source.repeat).map(({a,b}) =>
    [a[0]!*mid[0]+a[1]!*mid[1]+a[2]!*mid[2]+b[0]!, a[3]!*mid[0]+a[4]!*mid[1]+a[5]!*mid[2]+b[1]!, a[6]!*mid[0]+a[7]!*mid[1]+a[8]!*mid[2]+b[2]!]) : [mid];
  return centres.every(centre => surfaces.some(surface => {
    const mesh = copyMesh(surface, quality);
    const points: Point3[] = surface.repeat ? repeatMaps(surface.repeat).map(({a,b}) => {
      const [x,y,z] = offset3(centre, b as Point3);
      return [a[0]!*x+a[3]!*y+a[6]!*z, a[1]!*x+a[4]!*y+a[7]!*z, a[2]!*x+a[5]!*y+a[8]!*z];
    }) : [centre];
    const vertex = (i: number): Point3 => [mesh.positions[3*i]!,mesh.positions[3*i+1]!,mesh.positions[3*i+2]!];
    return points.some(p => {
      for (let i = 0; i < mesh.indices.length; i += 3) {
        if (nearTriangle(p, vertex(mesh.indices[i]!), vertex(mesh.indices[i + 1]!), vertex(mesh.indices[i + 2]!), diameter * diameter)) return true;
      }
      return false;
    });
  }));
}

/** 🫁️ True when a thin thermal outlet has a face spanning the declared breath diameter. The two face dimensions exceed its normal thickness. */
function breathFace(solid: VolumeSolid, diameter: number): boolean {
  const scale = solid.transform?.scale ?? [1, 1, 1];
  let thickness: number, width: number, length: number;
  if (solid.kind === "ellipsoid") {
    [thickness, width, length] = solid.radii.map((r, i) => 2 * r * scale[i]!).sort((a, b) => a - b) as [number, number, number];
  } else if (solid.kind === "extrude" && solid.taperTo === undefined) {
    const [u, v, normal]: [number, number, number] = solid.plane === "xy" ? [0, 1, 2] : solid.plane === "yz" ? [1, 2, 0] : [2, 0, 1];
    const face = solid.profile.map(([x, y]) => ({ x: x * scale[u]!, y: y * scale[v]! }));
    thickness = (solid.to - solid.from) * scale[normal]!;
    width = minimumWidth(face);
    length = 0;
    for (let i = 0; i < face.length; i++) for (let j = i + 1; j < face.length; j++) {
      length = Math.max(length, Math.hypot(face[i]!.x - face[j]!.x, face[i]!.y - face[j]!.y));
    }
  } else {
    return false;
  }
  return thickness > 0 && thickness < width && Math.abs(length - diameter) <= 1e-6;
}

/** 🧵 Physical stock gauges associated with a material and bone by that scale's thermal surfaces. A conflicting gauge makes the association ambiguous. */
function sheetGauges(asset: Asset, volume: AuthoredVolume): ReadonlyMap<string, number> {
  const layers = new Map(asset.thermal.regions.flatMap(region => region.layers.map(layer => [`${region.id}/${layer.id}`, layer] as const)));
  const body = new Map(volume.body.map(solid => [solid.id, solid]));
  const gauges = new Map<string, number>();
  for (const surface of volume.thermal) {
    if (surface.layer === undefined) continue;
    const layer = layers.get(`${surface.region}/${surface.layer}`);
    const solid = "of" in surface ? body.get(surface.of) : surface;
    if (solid === undefined || layer?.thicknessM === undefined || layer.material !== solid.material) continue;
    const key = `${solid.bone}/${solid.material}`;
    const previous = gauges.get(key);
    gauges.set(key, previous === undefined || previous === layer.thicknessM ? layer.thicknessM : NaN);
  }
  return gauges;
}

/** 📐 A planar sheet keeps its declared material gauge through the normal and must still meet the feature width within its plane. */
function matchesSheet(solid: VolumeSolid, gauge: number | undefined, feature: number): boolean {
  if (solid.kind !== "extrude" || !(gauge !== undefined && gauge > 0) || solid.taperTo !== undefined) return false;
  const [u, v, normal] = solid.plane === "xy" ? [0,1,2] as const : solid.plane === "yz" ? [1,2,0] as const : [2,0,1] as const;
  const scale = solid.transform?.scale ?? [1,1,1];
  const thickness = Math.abs((solid.to - solid.from) * scale[normal]!);
  if (Math.abs(thickness - gauge) > Math.max(1e-9, gauge * 1e-5)) return false;
  const width = minimumWidth(solid.profile.map(([a,b]) => ({ x:a * scale[u]!, y:b * scale[v]! })));
  return width >= Math.max(feature, VOLUME_LIMITS.slenderRatio * thickness) - 1e-9;
}

/** 🪡 Exact physical correspondence keeps material gauges and anatomical dimensions independent of paper scale. */
function sameValue(a: unknown, b: unknown): boolean {
  if (a === b) return true;
  if (!a || !b || typeof a !== "object" || typeof b !== "object" || Array.isArray(a) !== Array.isArray(b)) return false;
  const left = a as Record<string,unknown>, right = b as Record<string,unknown>;
  return Object.keys(left).length === Object.keys(right).length && Object.keys(left).every(key => Object.hasOwn(right,key) && sameValue(left[key],right[key]));
}

function retainedStations(coarse: readonly unknown[], fine: readonly unknown[]): boolean {
  if (coarse.length < 2 || coarse.length > fine.length || !sameValue(coarse[0],fine[0]) || !sameValue(coarse.at(-1),fine.at(-1))) return false;
  let at = 1;
  for (let i = 1; i < coarse.length - 1; i++) {
    while (at < fine.length - 1 && !sameValue(coarse[i],fine[at])) at++;
    if (at >= fine.length - 1) return false;
    at++;
  }
  return true;
}

type PhysicalFields = Readonly<Record<string,unknown>>;

const displayFields = new Set(["id","of","tone","sectionFaces"]);

function physicalMatch(coarse: PhysicalFields, fine: PhysicalFields): boolean {
  const keys = Object.keys(coarse).filter(key => !displayFields.has(key));
  if (keys.length !== Object.keys(fine).filter(key => !displayFields.has(key)).length) return false;
  return keys.every(key => {
    if (!Object.hasOwn(fine,key)) return false;
    if (key === "sections" && coarse.kind === "loft") return retainedStations(coarse[key] as readonly unknown[],fine[key] as readonly unknown[]);
    return sameValue(coarse[key],fine[key]);
  });
}

function physicalSources(fine: AuthoredVolume, list: VolumeList): (solid: VolumeSolid) => boolean {
  const groups = new Map<string,PhysicalFields[]>();
  const group = (s: PhysicalFields) => JSON.stringify([s.bone,s.kind,s.material,s.region,s.layer,s.member,s.part,s.anchor]);
  const body = new Map(fine.body.map(s => [s.id,s]));
  for (const entry of fine[list]) {
    const solid = "of" in entry ? { ...body.get(entry.of), ...entry } : entry;
    const key = group(solid), groupSolids = groups.get(key);
    if (groupSolids) groupSolids.push(solid);
    else groups.set(key,[solid]);
  }
  return solid => groups.get(group(solid))?.some(source => physicalMatch(solid,source)) ?? false;
}

/** 🔬️ `volumes-feature` (amendments C13, C20; the aggregate `minDimensionM` is information, the per-solid findings judge): no solid of any list has a dimension below min(the smallest feature of the scale, the thinnest filled 2D shape extent drawn for its bone at that scale), except `role` in {@link VOLUME_LIMITS}.thinRoles (declared thin sheets), `slender` solids (C2), `texture` solids (C10) and thermal shells with a `layer` and physiological surfaces without an individual filled region on their bone in print. Broad flat extrusions matching the thermal material gauge retain their sheet thickness. A tagged breath outlet on the anchor bone retains its physical thickness when its face spans the declared diameter above a separate tissue surface. A coarser solid may keep the exact dimensions of its 1:50 counterpart on the same bone and material, including a loft with unchanged end sections and a subset of its original interior stations. */
export function featureRule(asset: Asset, scales?: readonly ScaleDenominator[], minFeatureMm: number = VOLUME_LIMITS.minFeatureMm): RuleResult {
  const c = collector("volumes-feature");
  const finest = asset.volumes?.["50"];
  const sources = new Map<VolumeList,(solid: VolumeSolid) => boolean>();
  const tissueRegions = new Set(asset.thermal.regions.filter((region) => region.model === "body").map((region) => region.id));
  const respiration = new Map<Slug | undefined, Respiration | undefined>();
  respiration.set(undefined, asset.thermal.respiration);
  for (const passenger of asset.thermal.passengers) respiration.set(passenger.id, passenger.respiration);
  const breathSurfaces = new Map(asset.thermal.regions.filter(region => region.model === "body").map(region => {
    const source = respiration.get(region.body?.subject);
    return [region.id, [source?.noseEmitter, source?.mouthEmitter]] as const;
  }));
  const breaths = asset.thermal.emitters.flatMap(emitter => {
    const anchor = emitter.kind === "breath" ? asset.anchors.find(a => a.id === emitter.anchor && a.kind === "emitter") : undefined;
    return anchor === undefined ? [] : [{ emitter, anchor }];
  });
  for (const volume of authoredVolumes(asset)) {
    if (scales !== undefined && !scales.includes(volume.denominator)) continue;
    const drawing = asset.representations[`${volume.denominator}`];
    if (drawing?.status !== "authored") continue;
    const printedTissues = new Set<string>();
    for (const view of Object.values(drawing.geometry)) {
      for (const shape of view) {
        const region = shape.region ?? (shape.regions?.length === 1 ? shape.regions[0] : undefined);
        if (shape.filled && region !== undefined && tissueRegions.has(region)) printedTissues.add(`${shape.bone}/${region}`);
      }
    }
    const gauges = sheetGauges(asset, volume);
    const tissueSurfaces = new Map<string, VolumeSolid[]>();
    if (breaths.length) {
      const surfaces = new Map(solidsOf(volume, "thermal").map(s => [s.id, s]));
      for (const { id, region } of volume.thermal) {
        const surface = surfaces.get(id);
        if (surface === undefined || surface.tags?.includes("emitter") || !tissueRegions.has(region)) continue;
        const key = `${surface.bone}/${region}`;
        const entries = tissueSurfaces.get(key) ?? [];
        entries.push(surface);
        tissueSurfaces.set(key, entries);
      }
    }
    const feature = featureOf(volume.denominator, minFeatureMm);
    const drawn = thinnestDrawn(asset, volume.denominator);
    for (const list of ["body", "thermal", "structure"] as const) {
      const solids = (volume[list] as readonly (VolumeSolid | ThermalSolid | StructuralSolid)[]).filter((solid) => !isReference(solid)) as readonly VolumeSolid[];
      let smallest = Infinity;
      solids.forEach((solid) => {
        const region = list === "thermal" ? (solid as ThermalSolid).region : undefined;
        if (region !== undefined && tissueRegions.has(region)) {
          const key = `${solid.bone}/${region}`;
          if (!solid.tags?.includes("emitter") && !printedTissues.has(key)) return;
          if (solid.tags?.includes("emitter") && tissueSurfaces.has(key) && breaths.some(({ emitter, anchor }) =>
            anchor.bone === solid.bone && (emitter.region === undefined || emitter.region === region) &&
            breathSurfaces.get(region)?.includes(emitter.id) && solid.tags!.includes(emitter.id) && breathFace(solid, emitter.diameterM) &&
            onTissue(solid, tissueSurfaces.get(key)!, emitter.diameterM, qualityOf(asset, volume.denominator)))) return;
        }
        const dimension = smallestDimension(solid);
        const limit = Math.min(feature, drawn.get(solid.bone) ?? Infinity);
        const sheet = dimension < limit && list === "body" && matchesSheet(solid, gauges.get(`${solid.bone}/${solid.material}`), limit);
        const thin = sheet || (VOLUME_LIMITS.thinRoles as readonly string[]).includes(solid.role) || solid.slender === true || solid.texture === true || (list === "thermal" && (solid as { layer?: string }).layer !== undefined);
        if (!thin && dimension < smallest) smallest = dimension;
        if (!thin && dimension < limit - 1e-9) {
          if (volume.denominator !== 50 && finest?.status === "authored") {
            let matches = sources.get(list);
            if (!matches) { matches = physicalSources(finest,list); sources.set(list,matches); }
            if (matches(solid)) return;
          }
          const m = c.measure({ scale: volume.denominator, list, name: `minDimensionM:${solid.id}`, value: dimension, limit, bound: "min" });
          c.fail(`volumes.${volume.denominator}.${list}[${(volume[list] as readonly { id: string }[]).findIndex((s) => s.id === solid.id)}]`, `Solid "${solid.id}" of ${list} is ${(dimension * 1000).toFixed(1)} mm thin, below ${(limit * 1000).toFixed(1)} mm (the smallest feature of 1:${volume.denominator}${limit < feature ? ` or the thinnest drawing of bone "${solid.bone}"` : ""}).`, `Körper "${solid.id}" von ${list} ist ${(dimension * 1000).toFixed(1)} mm dünn, unter ${(limit * 1000).toFixed(1)} mm (dem kleinsten Merkmal von 1:${volume.denominator}${limit < feature ? ` oder der dünnsten Zeichnung des Knochens "${solid.bone}"` : ""}).`, { scale: volume.denominator, list, measurements: [m] });
        }
      });
      c.measure({ scale: volume.denominator, list, name: "minDimensionM", value: smallest === Infinity ? feature : smallest, limit: 0, bound: "min" });
    }
  }
  return c;
}

const regionOf = (solid: ThermalSolid): Slug => (solid as { region: Slug }).region;
const tagsOf = (volume: AuthoredVolume): ReadonlyMap<string, readonly string[]> => new Map(solidsOf(volume, "thermal").map((solid) => [solid.id, solid.tags ?? []] as const));

/** 🌡️ `volumes-thermal-coverage` (presence part, amendments B2-B4): every thermal region has a solid at every authored scale; at 1:50 every layer of a multi-layer region that has a `thicknessM` is a shell with that `layer`; every emitter has a thermal solid tagged with its id; a hidden region is a real solid of its own, never a reference to a body solid. The area and bounding-box checks are geometric (`checkVolumes`). */
export function thermalCoverageRule(asset: Asset, scales?: readonly ScaleDenominator[]): RuleResult {
  const c = collector("volumes-thermal-coverage");
  for (const volume of authoredVolumes(asset)) {
    if (scales !== undefined && !scales.includes(volume.denominator)) continue;
    const d = volume.denominator;
    const base = `volumes.${d}.thermal`;
    const solids = volume.thermal;
    const tags = tagsOf(volume);
    let missingRegions = 0;
    for (const region of asset.thermal.regions) {
      const own = solids.filter((solid) => regionOf(solid) === region.id);
      if (own.length === 0) {
        missingRegions++;
        c.fail(base, `Thermal region "${region.id}" has no solid at 1:${d}.`, `Die thermische Region "${region.id}" hat bei 1:${d} keinen Körper.`, { scale: d, list: "thermal", data: { region: region.id } });
        continue;
      }
      if (d === 50 && region.layers.length > 1)
        for (const layer of region.layers)
          if (layer.thicknessM !== undefined && !own.some((solid) => (solid as { layer?: Slug }).layer === layer.id)) {
            missingRegions++;
            c.fail(base, `Layer "${layer.id}" of region "${region.id}" has no shell at 1:50.`, `Die Schicht "${layer.id}" der Region "${region.id}" hat bei 1:50 keine Schale.`, { scale: d, list: "thermal", data: { region: region.id, layer: layer.id } });
          }
      if (region.hidden !== undefined && own.some((solid) => isReference(solid))) {
        missingRegions++;
        c.fail(base, `Hidden region "${region.id}" reuses a body solid at 1:${d}; hidden regions are real solids inside the body, never body surfaces.`, `Die verdeckte Region "${region.id}" verwendet bei 1:${d} einen Körper des Bodys; verdeckte Regionen sind eigene Körper im Inneren.`, { scale: d, list: "thermal", data: { region: region.id } });
      }
    }
    for (const emitter of asset.thermal.emitters)
      if (![...tags.values()].some((list) => list.includes(emitter.id))) {
        missingRegions++;
        c.fail(base, `Emitter "${emitter.id}" has no thermal solid tagged with its id at 1:${d}.`, `Der Emitter "${emitter.id}" hat bei 1:${d} keinen mit seiner Id markierten thermischen Körper.`, { scale: d, list: "thermal", data: { emitter: emitter.id } });
      }
    c.measure({ scale: d, list: "thermal", name: "uncoveredItems", value: missingRegions, limit: 0, bound: "max" });
  }
  return c;
}

/** 🏗️ `volumes-structure-coverage` (presence part, amendment B1, B3): every bone `eligibleBones` of the structural view selects has a `member` solid, every 2D `member` shape at the scale has one more solid along its bone, and every `structure.supports` anchor has exactly one `support` solid naming it in `anchor`. Containment in the body and the ground contact are geometric (`checkVolumes`). */
export function structureCoverageRule(asset: Asset, scales?: readonly ScaleDenominator[]): RuleResult {
  const c = collector("volumes-structure-coverage");
  const eligible = eligibleBones(asset, asset.dimensions.heightM);
  const loadBearing = asset.rig.bones.filter((bone) => eligible(bone.id)).map((bone) => bone.id);
  const anchors = new Map(asset.anchors.map((anchor) => [anchor.id, anchor] as const));
  for (const volume of authoredVolumes(asset)) {
    if (scales !== undefined && !scales.includes(volume.denominator)) continue;
    const d = volume.denominator;
    const base = `volumes.${d}.structure`;
    const rep = asset.representations[`${d}`];
    const parts = (part: string, bone: Slug): number => volume.structure.filter((solid) => (solid as { part: string }).part === part && (solid as { member: Slug }).member === bone).length;
    const memberShapes = new Map<Slug, number>();
    if (rep !== undefined && rep.status === "authored")
      for (const shapes of [rep.geometry.elevation, rep.geometry.front, rep.geometry.plan]) {
        const perBone = new Map<Slug, number>();
        for (const shape of shapes) if (shape.kind === "member") perBone.set(shape.bone, (perBone.get(shape.bone) ?? 0) + 1);
        for (const [bone, count] of perBone) memberShapes.set(bone, Math.max(memberShapes.get(bone) ?? 0, count));
      }
    let uncovered = 0;
    for (const bone of loadBearing)
      if (parts("member", bone) < Math.max(1, memberShapes.get(bone) ?? 0)) {
        uncovered++;
        c.fail(base, `Load-bearing bone "${bone}" has no member solid at 1:${d}${memberShapes.has(bone) ? ` (its drawing has ${memberShapes.get(bone)} member shapes)` : ""}.`, `Der tragende Knochen "${bone}" hat bei 1:${d} keinen Stabkörper${memberShapes.has(bone) ? ` (seine Zeichnung hat ${memberShapes.get(bone)} Stabformen)` : ""}.`, { scale: d, list: "structure", data: { bone } });
      }
    for (const [bone, count] of memberShapes) if (!loadBearing.includes(bone) && parts("member", bone) < count) {
      uncovered++;
      c.fail(base, `Bone "${bone}" draws ${count} member shapes at 1:${d} but has fewer member solids.`, `Der Knochen "${bone}" zeichnet bei 1:${d} ${count} Stabformen, hat aber weniger Stabkörper.`, { scale: d, list: "structure", data: { bone } });
    }
    for (const id of asset.structure.supports) {
      const own = volume.structure.filter((solid) => (solid as { part: string }).part === "support" && (solid as { anchor?: Slug }).anchor === id).length;
      if (own !== 1) {
        uncovered++;
        c.fail(base, own === 0 ? `Support anchor "${id}" has no support solid at 1:${d}.` : `Support anchor "${id}" has ${own} support solids at 1:${d}; exactly one is needed.`, own === 0 ? `Der Auflageranker "${id}" hat bei 1:${d} keinen Auflagerkörper.` : `Der Auflageranker "${id}" hat bei 1:${d} ${own} Auflagerkörper; genau einer wird gebraucht.`, { scale: d, list: "structure", data: { anchor: id } });
      }
    }
    c.measure({ scale: d, list: "structure", name: "uncoveredItems", value: uncovered, limit: 0, bound: "max" });
  }
  return c;
}


/** 🔗️ How two lists of solids relate: `identical`, `subset` (every coarse solid occurs identically in the fine list), `scaled` (equal up to a common factor of all lengths) or `distinct`. */
export type SolidSimilarity = { readonly kind: "identical" | "subset" | "scaled" | "distinct"; readonly sharedSolids: number; readonly coarseSolids: number; readonly fineSolids: number; readonly factor?: number };

const ROUND_M = 1e-4;
const LENGTH_KEYS = new Set(["size", "radii", "cornerRadius", "p0", "p1", "r0", "r1", "origin", "from", "to", "at", "rx", "ry", "translation", "profile", "holes", "vertices"]);
const IDENTITY_KEYS = new Set(["id", "of", "region", "layer", "member", "part", "anchor", "tags"]);

function normal(value: unknown, factor: number, lengths: boolean): unknown {
  if (typeof value === "number") return Math.round((lengths ? value * factor : value) / ROUND_M);
  if (Array.isArray(value)) return value.map((item) => normal(item, factor, lengths));
  if (value === null || typeof value !== "object") return value;
  const out: Record<string, unknown> = {};
  for (const key of Object.keys(value).sort()) if (!IDENTITY_KEYS.has(key)) out[key] = normal((value as Record<string, unknown>)[key], factor, LENGTH_KEYS.has(key));
  return out;
}

const signature = (solid: VolumeSolid, factor: number): string => JSON.stringify(normal(solid, factor, false));

function reach(solids: readonly VolumeSolid[]): number {
  let max = 0;
  const walk = (value: unknown, lengths: boolean): void => {
    if (typeof value === "number") {
      if (lengths) max = Math.max(max, Math.abs(value));
    } else if (Array.isArray(value)) value.forEach((item) => walk(item, lengths));
    else if (value !== null && typeof value === "object") for (const [key, child] of Object.entries(value)) walk(child, LENGTH_KEYS.has(key));
  };
  solids.forEach((solid) => walk(solid, false));
  return max;
}

function multiset(items: readonly string[]): Map<string, number> {
  const counts = new Map<string, number>();
  for (const item of items) counts.set(item, (counts.get(item) ?? 0) + 1);
  return counts;
}

const sameMultiset = (a: readonly string[], b: readonly string[]): boolean => {
  if (a.length !== b.length) return false;
  const counts = multiset(a);
  for (const item of b) {
    const left = counts.get(item) ?? 0;
    if (left === 0) return false;
    counts.set(item, left - 1);
  }
  return true;
};

/** 🪞️ Compares two lists of real solids (no `{ of }` references) of two scales: signatures are kind, bone, material, role, tone, weights, transform, repeat and the geometry rounded to 0.1 mm, ids and region, layer, member and part labels ignored; the factors tried are the denominator ratio and the ratio of the drawn reaches. */
export function compareSolidLists(coarse: readonly VolumeSolid[], fine: readonly VolumeSolid[], coarseDenominator: number, fineDenominator: number): SolidSimilarity {
  const sa = coarse.map((solid) => signature(solid, 1));
  const sb = fine.map((solid) => signature(solid, 1));
  const pool = multiset(sb);
  let shared = 0;
  for (const item of sa) {
    const left = pool.get(item) ?? 0;
    if (left > 0) {
      shared++;
      pool.set(item, left - 1);
    }
  }
  const base = { sharedSolids: shared, coarseSolids: coarse.length, fineSolids: fine.length };
  if (sameMultiset(sa, sb)) return { kind: "identical", ...base };
  if (coarse.length > 0 && shared === coarse.length) return { kind: "subset", ...base };
  const [ea, eb] = [reach(coarse), reach(fine)];
  const factors = new Set<number>([coarseDenominator / fineDenominator, fineDenominator / coarseDenominator]);
  if (ea > 0 && eb > 0) {
    factors.add(eb / ea);
    factors.add(ea / eb);
  }
  for (const factor of factors) {
    if (Math.abs(factor - 1) < 1e-9) continue;
    if (sameMultiset(coarse.map((solid) => signature(solid, factor)), sb)) return { kind: "scaled", factor, ...base };
  }
  return { kind: "distinct", ...base };
}

/** 🪞️ `volumes-copied`: no authored scale has a `body`, `thermal` or `structure` list of real solids that is identical to, a subset of, or a uniform scaling of the same list of a finer authored scale; every scale is modelled anew. `{ of }` references are excluded, so a thermal or structural list made only of references is not judged. */
export function copiedRule(asset: Asset, scales?: readonly ScaleDenominator[]): RuleResult {
  const c = collector("volumes-copied");
  const volumes = authoredVolumes(asset);
  const real = (volume: AuthoredVolume, list: VolumeList): readonly VolumeSolid[] => (volume[list] as readonly object[]).filter((solid) => !isReference(solid)) as unknown as readonly VolumeSolid[];
  volumes.forEach((coarse, i) => {
    if (scales !== undefined && !scales.includes(coarse.denominator)) return;
    for (const list of ["body", "thermal", "structure"] as const) {
      const own = real(coarse, list);
      if (own.length === 0) continue;
      for (const fine of volumes.slice(0, i)) {
        const similarity = compareSolidLists(own, real(fine, list), coarse.denominator, fine.denominator);
        const share = c.measure({ scale: coarse.denominator, list, name: `copied:${fine.denominator}`, value: similarity.kind === "distinct" ? 0 : 1, limit: 0, bound: "max" });
        if (similarity.kind !== "distinct")
          c.fail(`volumes.${coarse.denominator}.${list}`, `The ${list} of 1:${coarse.denominator} is a copy of 1:${fine.denominator} (${similarity.kind}: ${similarity.sharedSolids} of ${similarity.coarseSolids} solids identical); every scale must be modelled anew.`, `Der ${list} von 1:${coarse.denominator} ist eine Kopie von 1:${fine.denominator} (${similarity.kind}: ${similarity.sharedSolids} von ${similarity.coarseSolids} Körpern identisch); jeder Maßstab muss neu modelliert werden.`, { scale: coarse.denominator, list, code: "volumes-copied", measurements: [share], data: { finer: fine.denominator, kind: similarity.kind, ...(similarity.factor === undefined ? {} : { factor: similarity.factor }) } });
      }
    }
  });
  return c;
}

/** 🪢️ The slenderness of a solid: the larger of its longest extent over the middle one and the length of its centre line over its largest cross extent, so that a hoop, a ring or a bent hose qualifies by the path it follows (C24 refined). The centre line is only found when the extents alone do not decide. */
export function slendernessOf(asset: Asset, solid: VolumeSolid): number {
  const [, middle, longest] = sortedExtents(solid);
  const boxes = middle > 0 ? longest / middle : Infinity;
  if (boxes >= VOLUME_LIMITS.slenderRatio) return boxes;
  const mesh = copyMesh(solid, qualityOf(asset));
  const line = centreLineOf(mesh.positions, mesh.positions, mesh.indices);
  return line === undefined || !(line.thickness > 0) ? boxes : Math.max(boxes, line.length / line.thickness);
}

/** 🪢️ `volumes-slender`: a solid that declares `slender: true` is at least {@link VOLUME_LIMITS}.slenderRatio times as long as its largest cross extent, measured along its centre line ({@link slendernessOf}), in every list of every authored scale. */
export function slenderRule(asset: Asset, scales?: readonly ScaleDenominator[]): RuleResult {
  const c = collector("volumes-slender");
  for (const volume of authoredVolumes(asset)) {
    if (scales !== undefined && !scales.includes(volume.denominator)) continue;
    for (const list of ["body", "thermal", "structure"] as const)
      (volume[list] as readonly object[]).forEach((entry, index) => {
        const solid = entry as VolumeSolid;
        if (solid.slender !== true || isReference(solid)) return;
        const m = c.measure({ scale: volume.denominator, list, name: `slenderRatio:${solid.id}`, value: slendernessOf(asset, solid), limit: VOLUME_LIMITS.slenderRatio, bound: "min" });
        if (!m.ok) c.fail(`volumes.${volume.denominator}.${list}[${index}]`, `Solid "${solid.id}" is declared slender but is only ${m.value.toFixed(1)} times as long as thick (at least ${VOLUME_LIMITS.slenderRatio}).`, `Körper "${solid.id}" ist als schlank erklärt, aber nur ${m.value.toFixed(1)}-mal so lang wie dick (mindestens ${VOLUME_LIMITS.slenderRatio}).`, { scale: volume.denominator, list, measurements: [m] });
      });
  }
  return c;
}

/** 🧰️ The analytic rules in order, for the callers that run them together. */
export function analyticRules(asset: Asset, scales?: readonly ScaleDenominator[], minFeatureMm: number = VOLUME_LIMITS.minFeatureMm): readonly VolumeFinding[] {
  return [availabilityRule(asset, scales), bonesRule(asset, scales), featureRule(asset, scales, minFeatureMm), thermalCoverageRule(asset, scales), structureCoverageRule(asset, scales), copiedRule(asset, scales), slenderRule(asset, scales)].flatMap((result) => result.findings);
}
