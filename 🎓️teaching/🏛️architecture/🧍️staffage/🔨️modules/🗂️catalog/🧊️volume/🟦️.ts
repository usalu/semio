/** 🧊️ Semantic rules of the declarative 3D volume (`asset.volume`): what the JSON Schema cannot say about solids bound to bones.
 *
 * Every finding is an {@link Issue} with a stable rule id: `volume-empty`, `volume-duplicate-id`, `volume-bone`, `volume-material`, `volume-region`, `volume-weights-bone`, `volume-weights-count`, `volume-weights-sum`,
 * `volume-size`, `volume-non-finite`, `volume-extrude-range`, `volume-profile-degenerate`, `volume-profile-self-intersecting`, `volume-hole-outside`, `volume-holes-overlap`, `volume-loft-sections`, `volume-repeat`,
 * `volume-transform` and `volume-max-denominator`. Profiles are simple polygons: at least three distinct points, a non-zero area, no edge touching or crossing a non-adjacent edge; holes lie strictly inside the outline and
 * apart from each other. A revolve profile is a simple open polyline of [axial, radial] points with non-negative radial values.
 *
 * THE REFERENCE for the polygon rules is shapely (`Polygon.is_valid`), see `🧪️tests/🧊️volume-schema/🐍️.py`.
 *
 * `exportIssues` judges the `exports` that name `glb`, `fbx` or `model`: `exports-model-volume`, `exports-glb-pose`, `exports-fbx-clip`; `modelFormats` says which of them an asset can produce.
 *
 * @see ../../../🧬️schema/🔣️.json — `Volume`, `VolumeSolid`
 * @see ../🟦️.ts — `validateAsset`, which runs these rules
 */
import { SCALE_DENOMINATORS, VOLUME_REPEAT_MAX, VOLUME_WEIGHTS_MAX } from "../../../🧬️schema/🟦️.ts";
import type { Asset, ExportFormat, Text, Vector2, VolumeSolid } from "../../../🧬️schema/🟦️.ts";
import type { Issue } from "../🟦️.ts";
import * as portable from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

/** ➕️ Allowed deviation of the weights of one solid from a sum of 1. */
export const WEIGHT_SUM_TOLERANCE = 1e-6;

/** 🔄️ Allowed deviation of the length of a rotation quaternion from 1. */
export const QUATERNION_TOLERANCE = 1e-4;

const EPS = 1e-12;

const text = (en: string, de: string): Text => ({ en, de });
const finite = (value: unknown): value is number => typeof value === "number" && Number.isFinite(value);
const positive = (value: unknown): boolean => finite(value) && value > 0;
const vector = (value: readonly unknown[] | undefined, length: number): value is readonly number[] => Array.isArray(value) && value.length === length && value.every(finite);
const norm = (v: readonly number[]): number => portable.hypot(...v);
const distance = (p: Vector2, q: Vector2): number => portable.hypot(p[0] - q[0], p[1] - q[1]);

const cross = (o: Vector2, a: Vector2, b: Vector2): number => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
const within = (a: number, b: number, c: number): boolean => Math.min(a, b) - EPS <= c && c <= Math.max(a, b) + EPS;
const onSegment = (a: Vector2, b: Vector2, p: Vector2): boolean => Math.abs(cross(a, b, p)) <= EPS * Math.max(1, distance(a, b)) && within(a[0], b[0], p[0]) && within(a[1], b[1], p[1]);

/** ✂️ Whether the closed segments ab and cd share a point (crossing, touching or overlapping). */
export function segmentsMeet(a: Vector2, b: Vector2, c: Vector2, d: Vector2): boolean {
  const d1 = cross(a, b, c);
  const d2 = cross(a, b, d);
  const d3 = cross(c, d, a);
  const d4 = cross(c, d, b);
  if (((d1 > EPS && d2 < -EPS) || (d1 < -EPS && d2 > EPS)) && ((d3 > EPS && d4 < -EPS) || (d3 < -EPS && d4 > EPS))) return true;
  return onSegment(a, b, c) || onSegment(a, b, d) || onSegment(c, d, a) || onSegment(c, d, b);
}

/** 📐️ The signed area of a ring (counter-clockwise positive). */
export const ringArea = (ring: readonly Vector2[]): number => ring.reduce((sum, p, i) => sum + (p[0] * ring[(i + 1) % ring.length]![1] - ring[(i + 1) % ring.length]![0] * p[1]), 0) / 2;

/** 🔎️ Whether a point lies strictly inside a ring (not on its boundary). */
export function strictlyInside(ring: readonly Vector2[], p: Vector2): boolean {
  let hit = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const a = ring[i]!;
    const b = ring[j]!;
    if (onSegment(a, b, p)) return false;
    if (a[1] > p[1] !== b[1] > p[1] && p[0] < ((b[0] - a[0]) * (p[1] - a[1])) / (b[1] - a[1]) + a[0]) hit = !hit;
  }
  return hit;
}

/** 🧩️ The verdict on a profile: `degenerate` (too few distinct points, a zero-length edge, or a ring on one line), `self-intersecting` (an edge meets a non-adjacent edge, or an adjacent one folds back), or `simple`. */
export function profileVerdict(points: readonly Vector2[], closed: boolean): "degenerate" | "self-intersecting" | "simple" {
  const n = points.length;
  if (n < (closed ? 3 : 2) || !points.every((p) => vector(p, 2))) return "degenerate";
  const edges = closed ? n : n - 1;
  for (let i = 0; i < edges; i++) if (distance(points[i]!, points[(i + 1) % n]!) <= EPS) return "degenerate";
  if (closed && points.every((p) => Math.abs(cross(points[0]!, points[1]!, p)) <= EPS * Math.max(1, distance(points[0]!, points[1]!)))) return "degenerate";
  if (!closed && points.every((p) => p[1] <= EPS)) return "degenerate";
  for (let i = 0; i < edges; i++)
    for (let j = i + 1; j < edges; j++) {
      const a = points[i]!;
      const b = points[(i + 1) % n]!;
      const c = points[j]!;
      const d = points[(j + 1) % n]!;
      const adjacent = j === i + 1 || (closed && i === 0 && j === edges - 1);
      if (adjacent) {
        const shared = j === i + 1 ? b : a;
        const far1 = j === i + 1 ? a : b;
        const far2 = j === i + 1 ? d : c;
        if (Math.abs(cross(shared, far1, far2)) <= EPS * Math.max(1, distance(shared, far1) * distance(shared, far2)) && (far1[0] - shared[0]) * (far2[0] - shared[0]) + (far1[1] - shared[1]) * (far2[1] - shared[1]) > 0) return "self-intersecting";
      } else if (segmentsMeet(a, b, c, d)) return "self-intersecting";
    }
  return closed && Math.abs(ringArea(points)) <= EPS ? "degenerate" : "simple";
}

const ringsMeet = (p: readonly Vector2[], q: readonly Vector2[]): boolean => p.some((a, i) => q.some((c, j) => segmentsMeet(a, p[(i + 1) % p.length]!, c, q[(j + 1) % q.length]!)));

/** 🕳️ Whether a hole lies strictly inside the outline: no edge of the hole meets an edge of the outline and one vertex of the hole is inside. */
export const holeInside = (outline: readonly Vector2[], hole: readonly Vector2[]): boolean => !ringsMeet(outline, hole) && strictlyInside(outline, hole[0]!);

/** 📤️ Whether two holes overlap or touch: an edge of one meets an edge of the other, or one contains the other. */
export const holesOverlap = (p: readonly Vector2[], q: readonly Vector2[]): boolean => ringsMeet(p, q) || strictlyInside(p, q[0]!) || strictlyInside(q, p[0]!);

/** 🔺️ The 3D model formats an asset can produce: `model` (the neutral JSON) and `model-zip` (every pose and clip) from its volume, `glb` (one per named pose) from its volume and one named pose, `fbx` (one per clip) from its volume and at least one clip; none without a volume. */
export function modelFormats(asset: Asset): readonly ExportFormat[] {
  if (asset.volume === undefined) return [];
  return [...(asset.poses.some((pose) => pose.key !== true) ? (["glb"] as const) : []), ...(asset.clips.length > 0 ? (["fbx"] as const) : []), "model", "model-zip"];
}

/** 📦️ The `exports` of an asset that name a 3D model format it cannot produce: `exports-model-volume` for `glb`, `fbx`, `model` and `model-zip` without a volume, `exports-glb-pose` for `glb` without a named pose, `exports-fbx-clip` for `fbx` without a clip. Other formats are not judged here. */
export function exportIssues(asset: Asset): readonly Issue[] {
  const producible = new Set<ExportFormat>(modelFormats(asset));
  const out: Issue[] = [];
  asset.exports.forEach((format, index) => {
    if (format !== "glb" && format !== "fbx" && format !== "model" && format !== "model-zip") return;
    if (producible.has(format)) return;
    const path = `exports[${index}]`;
    if (asset.volume === undefined) out.push({ path, code: "exports-model-volume", message: text(`The export "${format}" needs an authored volume, which this asset does not have.`, `Der Export "${format}" braucht ein erstelltes Volumen, das dieses Asset nicht hat.`) });
    else if (format === "glb") out.push({ path, code: "exports-glb-pose", message: text('The export "glb" is one file per named pose and the asset has none.', 'Der Export "glb" ist eine Datei je benannter Pose und das Asset hat keine.') });
    else out.push({ path, code: "exports-fbx-clip", message: text('The export "fbx" is one file per clip and the asset has no clip.', 'Der Export "fbx" ist eine Datei je Clip und das Asset hat keinen Clip.') });
  });
  return out;
}

/** 🧱️ Every semantic problem of the volume of an asset; empty for a valid volume and for an asset without one. */
export function volumeIssues(asset: Asset): readonly Issue[] {
  const volume = asset.volume;
  if (volume === undefined) return [];
  const out: Issue[] = [];
  const add = (path: string, code: string, en: string, de: string): void => void out.push({ path, code, message: text(en, de) });
  const bones = new Set(asset.rig.bones.map((bone) => bone.id));
  const materials = new Set(asset.materials.map((material) => material.id));
  const regions = new Set(asset.thermal.regions.map((region) => region.id));
  const seen = new Set<string>();
  if (volume.solids.length === 0) add("volume.solids", "volume-empty", "The volume needs at least one solid.", "Das Volumen braucht mindestens einen Körper.");
  volume.solids.forEach((solid, index) => {
    const p = `volume.solids[${index}]`;
    const label = `Solid "${solid.id}"`;
    const bez = `Körper "${solid.id}"`;
    if (seen.has(solid.id)) add(`${p}.id`, "volume-duplicate-id", `${label} repeats an id already used in the volume.`, `${bez} wiederholt eine im Volumen schon verwendete Id.`);
    seen.add(solid.id);
    if (!bones.has(solid.bone)) add(`${p}.bone`, "volume-bone", `${label} is bound to bone "${solid.bone}", which does not exist in the rig.`, `${bez} ist an den Knochen "${solid.bone}" gebunden, der im Rig nicht existiert.`);
    if (!materials.has(solid.material)) add(`${p}.material`, "volume-material", `${label} uses material "${solid.material}", which is not in the asset's materials.`, `${bez} verwendet das Material "${solid.material}", das nicht in den Materialien des Assets steht.`);
    for (const region of [...(solid.region === undefined ? [] : [solid.region]), ...(solid.regions ?? [])])
      if (!regions.has(region)) add(`${p}.${solid.region === region ? "region" : "regions"}`, "volume-region", `${label} names thermal region "${region}", which does not exist in thermal.regions.`, `${bez} nennt die thermische Region "${region}", die in thermal.regions nicht existiert.`);
    if (solid.maxDenominator !== undefined && !(SCALE_DENOMINATORS as readonly number[]).includes(solid.maxDenominator))
      add(`${p}.maxDenominator`, "volume-max-denominator", `${label} has maxDenominator ${solid.maxDenominator}, which is not a preset scale (${SCALE_DENOMINATORS.join(", ")}).`, `${bez} hat maxDenominator ${solid.maxDenominator}, das keine voreingestellte Maßstabszahl ist (${SCALE_DENOMINATORS.join(", ")}).`);
    weightIssues(solid, p, bones, add);
    transformIssues(solid, p, add);
    if (solid.repeat !== undefined) repeatIssues(solid, p, add);
    geometryIssues(solid, p, add);
  });
  return out;
}

type Add = (path: string, code: string, en: string, de: string) => void;

function weightIssues(solid: VolumeSolid, p: string, bones: ReadonlySet<string>, add: Add): void {
  const weights = solid.weights;
  if (weights === undefined) return;
  const label = `Solid "${solid.id}"`;
  const bez = `Körper "${solid.id}"`;
  if (weights.length === 0 || weights.length > VOLUME_WEIGHTS_MAX) add(`${p}.weights`, "volume-weights-count", `${label} blends over ${weights.length} bones; one to ${VOLUME_WEIGHTS_MAX} are allowed.`, `${bez} mischt über ${weights.length} Knochen; erlaubt sind einer bis ${VOLUME_WEIGHTS_MAX}.`);
  const named = new Set<string>();
  weights.forEach((entry, i) => {
    if (!bones.has(entry.bone)) add(`${p}.weights[${i}].bone`, "volume-weights-bone", `${label} weights bone "${entry.bone}", which does not exist in the rig.`, `${bez} gewichtet den Knochen "${entry.bone}", der im Rig nicht existiert.`);
    else if (named.has(entry.bone)) add(`${p}.weights[${i}].bone`, "volume-weights-bone", `${label} weights bone "${entry.bone}" more than once.`, `${bez} gewichtet den Knochen "${entry.bone}" mehrfach.`);
    named.add(entry.bone);
  });
  const sum = weights.reduce((total, entry) => total + entry.weight, 0);
  if (!weights.every((entry) => finite(entry.weight) && entry.weight >= 0) || !(Math.abs(sum - 1) <= WEIGHT_SUM_TOLERANCE)) add(`${p}.weights`, "volume-weights-sum", `The weights of ${label.toLowerCase()} must be non-negative and sum to 1, they sum to ${sum}.`, `Die Gewichte von ${bez} müssen nicht negativ sein und sich zu 1 summieren, sie summieren sich zu ${sum}.`);
}

function unit(q: readonly number[] | undefined): boolean {
  return q === undefined || (vector(q, 4) && Math.abs(norm(q) - 1) <= QUATERNION_TOLERANCE);
}

function transformIssues(solid: VolumeSolid, p: string, add: Add): void {
  const transform = solid.transform;
  if (transform === undefined) return;
  const bad = (field: string, what: string, was: string): void => add(`${p}.transform.${field}`, "volume-transform", `Solid "${solid.id}" has ${what}.`, `Körper "${solid.id}" hat ${was}.`);
  if (!unit(transform.rotation)) bad("rotation", "a rotation that is not a unit quaternion", "eine Drehung, die kein Einheitsquaternion ist");
  if (transform.scale !== undefined && !(vector(transform.scale, 3) && transform.scale.every((factor) => factor > 0))) bad("scale", "a scale with a non-positive factor", "einen Maßstab mit nicht positivem Faktor");
  if (transform.translation !== undefined && !vector(transform.translation, 3)) bad("translation", "a translation that is not three finite numbers", "eine Verschiebung, die nicht aus drei endlichen Zahlen besteht");
}

function repeatIssues(solid: VolumeSolid, p: string, add: Add): void {
  const repeat = solid.repeat!;
  const bad = (en: string, de: string): void => add(`${p}.repeat`, "volume-repeat", `Solid "${solid.id}" ${en}`, `Körper "${solid.id}" ${de}`);
  if (!Number.isInteger(repeat.count) || repeat.count < 1) bad(`repeats ${repeat.count} times; the count is an integer of at least 1.`, `wird ${repeat.count}-mal wiederholt; die Anzahl ist eine ganze Zahl von mindestens 1.`);
  else if (repeat.count > VOLUME_REPEAT_MAX) bad(`repeats ${repeat.count} times; at most ${VOLUME_REPEAT_MAX} copies are allowed.`, `wird ${repeat.count}-mal wiederholt; höchstens ${VOLUME_REPEAT_MAX} Kopien sind erlaubt.`);
  if (repeat.rotationRad !== undefined && repeat.rotationRad !== 0 && (repeat.rotationAxis === undefined || !vector(repeat.rotationAxis, 3) || !(norm(repeat.rotationAxis) > EPS))) bad("rotates its copies without a non-zero rotationAxis.", "dreht seine Kopien ohne eine rotationAxis ungleich null.");
  for (const field of ["translation", "rotationAxis", "origin"] as const) if (repeat[field] !== undefined && !vector(repeat[field], 3)) bad(`has a ${field} that is not three finite numbers.`, `hat eine ${field}, die nicht aus drei endlichen Zahlen besteht.`);
}

function geometryIssues(solid: VolumeSolid, p: string, add: Add): void {
  const label = `Solid "${solid.id}"`;
  const bez = `Körper "${solid.id}"`;
  const size = (field: string, en: string, de: string): void => add(`${p}.${field}`, "volume-size", `${label} ${en}`, `${bez} ${de}`);
  const point = (field: string, value: readonly number[] | undefined): void => {
    if (!vector(value, 3)) add(`${p}.${field}`, "volume-non-finite", `${label} has a ${field} that is not three finite numbers.`, `${bez} hat ein ${field}, das nicht aus drei endlichen Zahlen besteht.`);
  };
  const lateral = (): void => {
    if ("lateralScale" in solid && solid.lateralScale !== undefined && !positive(solid.lateralScale)) size("lateralScale", `has lateralScale ${solid.lateralScale}; it must be positive.`, `hat lateralScale ${solid.lateralScale}; es muss positiv sein.`);
  };
  switch (solid.kind) {
    case "box":
      if (!(vector(solid.size, 3) && solid.size.every(positive))) size("size", `has a box size ${JSON.stringify(solid.size)} with a non-positive edge.`, `hat die Quadergröße ${JSON.stringify(solid.size)} mit einer nicht positiven Kante.`);
      else if (solid.cornerRadius !== undefined && !(finite(solid.cornerRadius) && solid.cornerRadius >= 0 && solid.cornerRadius <= Math.min(...solid.size) / 2 + EPS)) size("cornerRadius", `has cornerRadius ${solid.cornerRadius}; it is at least 0 and at most half the smallest edge.`, `hat cornerRadius ${solid.cornerRadius}; er ist mindestens 0 und höchstens die halbe kleinste Kante.`);
      break;
    case "ellipsoid":
      if (!(vector(solid.radii, 3) && solid.radii.every(positive))) size("radii", `has the radii ${JSON.stringify(solid.radii)} with a non-positive radius.`, `hat die Radien ${JSON.stringify(solid.radii)} mit einem nicht positiven Radius.`);
      break;
    case "capsule":
      point("p0", solid.p0);
      point("p1", solid.p1);
      if (!(finite(solid.r0) && finite(solid.r1) && solid.r0 >= 0 && solid.r1 >= 0 && solid.r0 + solid.r1 > 0)) size("r0", `has the radii ${solid.r0} and ${solid.r1}; both are at least 0 and not both 0.`, `hat die Radien ${solid.r0} und ${solid.r1}; beide sind mindestens 0 und nicht beide 0.`);
      lateral();
      break;
    case "revolve": {
      point("axis", solid.axis);
      point("origin", solid.origin);
      if (vector(solid.axis, 3) && !(norm(solid.axis) > EPS)) size("axis", "has the zero vector as axis.", "hat den Nullvektor als Achse.");
      if (solid.sweepRad !== undefined && !(finite(solid.sweepRad) && solid.sweepRad > 0 && solid.sweepRad <= 2 * Math.PI + EPS)) size("sweepRad", `has sweepRad ${solid.sweepRad}; it is in (0, 2 pi].`, `hat sweepRad ${solid.sweepRad}; es liegt in (0, 2 pi].`);
      if (solid.profile.some((q) => vector(q, 2) && q[1] < 0)) size("profile", "has a profile point with a negative radial value.", "hat einen Profilpunkt mit negativem Radialwert.");
      lateral();
      const loop = solid.profile.length >= 4 && vector(solid.profile[0], 2) && vector(solid.profile[solid.profile.length - 1], 2) && distance(solid.profile[0]!, solid.profile[solid.profile.length - 1]!) <= EPS;
      profileIssues(loop ? solid.profile.slice(0, -1) : solid.profile, `${p}.profile`, loop, solid.id, add);
      break;
    }
    case "extrude": {
      if (!(finite(solid.from) && finite(solid.to) && solid.from < solid.to)) add(`${p}.from`, "volume-extrude-range", `${label} needs from < to, got ${solid.from} and ${solid.to}.`, `${bez} braucht from < to, erhielt ${solid.from} und ${solid.to}.`);
      if (solid.taperTo !== undefined && !(vector(solid.taperTo, 2) && solid.taperTo.every((factor) => factor >= 0))) size("taperTo", `has taperTo ${JSON.stringify(solid.taperTo)}; both factors are at least 0.`, `hat taperTo ${JSON.stringify(solid.taperTo)}; beide Faktoren sind mindestens 0.`);
      const outline = profileIssues(solid.profile, `${p}.profile`, true, solid.id, add);
      const holes = solid.holes ?? [];
      const simple = holes.map((hole, k) => profileIssues(hole, `${p}.holes[${k}]`, true, solid.id, add));
      holes.forEach((hole, k) => {
        if (outline && simple[k] && !holeInside(solid.profile, hole)) add(`${p}.holes[${k}]`, "volume-hole-outside", `Hole ${k} of ${label.toLowerCase()} is not strictly inside the outline.`, `Loch ${k} von ${bez} liegt nicht echt innerhalb der Kontur.`);
        for (let m = k + 1; m < holes.length; m++)
          if (simple[k] && simple[m] && holesOverlap(hole, holes[m]!)) add(`${p}.holes[${m}]`, "volume-holes-overlap", `Holes ${k} and ${m} of ${label.toLowerCase()} overlap or touch.`, `Löcher ${k} und ${m} von ${bez} überlappen oder berühren sich.`);
      });
      break;
    }
    case "loft": {
      if (solid.sections.length < 2) add(`${p}.sections`, "volume-loft-sections", `${label} needs at least two loft sections, got ${solid.sections.length}.`, `${bez} braucht mindestens zwei Loft-Schnitte, erhielt ${solid.sections.length}.`);
      solid.sections.forEach((section, k) => {
        point(`sections[${k}].at`, section.at);
        if (!(positive(section.rx) && positive(section.ry))) size(`sections[${k}]`, `has section ${k} with rx ${section.rx} and ry ${section.ry}; both are positive.`, `hat Schnitt ${k} mit rx ${section.rx} und ry ${section.ry}; beide sind positiv.`);
        if (section.exponent !== undefined && !positive(section.exponent)) size(`sections[${k}].exponent`, `has section ${k} with exponent ${section.exponent}; it is positive.`, `hat Schnitt ${k} mit Exponent ${section.exponent}; er ist positiv.`);
        if (!unit(section.rotation)) add(`${p}.sections[${k}].rotation`, "volume-transform", `${label} has section ${k} with a rotation that is not a unit quaternion.`, `${bez} hat Schnitt ${k} mit einer Drehung, die kein Einheitsquaternion ist.`);
        const next = solid.sections[k + 1];
        if (next !== undefined && vector(section.at, 3) && vector(next.at, 3) && !(norm(section.at.map((value, i) => value - next.at[i]!)) > EPS)) add(`${p}.sections[${k + 1}].at`, "volume-loft-sections", `${label} has sections ${k} and ${k + 1} at the same centre; the path has no direction there.`, `${bez} hat die Schnitte ${k} und ${k + 1} am selben Mittelpunkt; der Pfad hat dort keine Richtung.`);
      });
      break;
    }
  }
}

function profileIssues(points: readonly Vector2[], path: string, closed: boolean, id: string, add: Add): boolean {
  const verdict = profileVerdict(points, closed);
  if (verdict === "degenerate") add(path, "volume-profile-degenerate", `The profile of solid "${id}" is degenerate: it needs ${closed ? "three" : "two"} distinct points, no zero-length edge${closed ? " and a non-zero area" : " and a positive radial value"}.`, `Das Profil von Körper "${id}" ist entartet: es braucht ${closed ? "drei" : "zwei"} verschiedene Punkte, keine Kante der Länge null${closed ? " und eine Fläche ungleich null" : " und einen positiven Radialwert"}.`);
  if (verdict === "self-intersecting") add(path, "volume-profile-self-intersecting", `The profile of solid "${id}" is self-intersecting: an edge crosses, touches or folds back onto another edge.`, `Das Profil von Körper "${id}" überschneidet sich selbst: eine Kante kreuzt, berührt oder faltet sich auf eine andere Kante zurück.`);
  return verdict === "simple";
}
