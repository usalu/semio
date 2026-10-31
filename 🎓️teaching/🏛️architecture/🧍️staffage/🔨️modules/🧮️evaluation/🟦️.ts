/** 🧮️ Evaluation of an asset at a query: the single deterministic state every view, analysis and export derives from.
 *
 * `evaluate(asset, query, selector)` takes the geometry from the selected representation (the authored drawing for a scale, or a named symbol; without a selector no geometry is
 * placed) and everything physical (bones, anchors, contacts, loads, thermal inputs, respiration, kinematics) from the asset alone, so it is identical for every selector. It samples the pose, clip or transition, solves the rig, flattens every shape to world rings (curves to 0.5 mm), places anchors, converts loads to world forces
 * (distributed loads to eight equivalent resultants per bone segment, scaled by `load:<id>` tracks), binds the thermal inputs and derives segment accelerations and the centre-of-mass
 * velocity by central differences of the evaluated clip with h = 1/240 s. Equal inputs yield bit-identical outputs.
 *
 * Plan shapes are drawn in the plan plane (x, depth); they follow their bone only by the bone's x travel from rest. `bounds` is the extent of the selected elevation geometry except the role `ground` (the drawn extent; the rig extent is {@link rigBounds}), and of the rig when nothing is drawn.
 * Clip queries live on the absolute timeline: the pose wraps, `cycle` counts the loops, planted root advance and respiration continue.
 *
 * @see ../../🧬️schema/🟦️.ts — `Asset`, `Shape`
 * @see ../🔗️contracts/🟦️.ts — `EvaluatedState`, `EvaluationQuery`
 * @see https://www.w3.org/TR/SVG11/paths.html — the path grammar
 */
import type { Anchor, Asset, AppliedLoad, Bone, BoneOffset, Shape, Slug, Viewpoint } from "../../🧬️schema/🟦️.ts";
import type { Affine, EvaluatedBone, EvaluatedShape, EvaluatedState, EvaluationQuery, Point, RepresentationSelector, RespirationState, WorldForce } from "../🔗️contracts/🟦️.ts";
import { PLANT_HZ, clipOf, cycleOf, poseOf, sampleClip, samplePose, transition, wrapTime } from "../🎞️animation/🟦️.ts";
import { respirationOffsets, respirationState } from "../💨️transport/🫁️respiration/🟦️.ts";
import { RepresentationError, pickRepresentation } from "../🖨️representation/🧭️pick/🟦️.ts";
import type { Sample } from "../🎞️animation/🟦️.ts";
import { apply, effectiveOffset, rotationRad, sinCosDeg, solveRig } from "../🦴️rig/🟦️.ts";

export { clipOf, poseOf };

/** 📏️ Flattening tolerance in world metres (0.5 mm). */
export const FLATTEN_TOLERANCE_M = 0.0005;

/** 🧭️ The finite-difference step of kinematics in seconds. */
export const KINEMATIC_STEP_S = 1 / PLANT_HZ;

/** ➗️ The number of equivalent point forces a distributed load becomes. */
export const DISTRIBUTED_SUBPOINTS = 8;

type Ring = Point[];

//#region 〰️Flattening
function arcSteps(radius: number, sweep: number, minimum: number): number {
  const ratio = FLATTEN_TOLERANCE_M / Math.max(radius, FLATTEN_TOLERANCE_M);
  const step = ratio >= 1 ? Math.PI / 2 : 2 * Math.acos(1 - ratio);
  return Math.min(512, Math.max(minimum, Math.ceil(Math.abs(sweep) / step)));
}

function chordDistance(p: Point, a: Point, b: Point): number {
  const dx = b.x - a.x;
  const dy = b.y - a.y;
  const length = Math.hypot(dx, dy);
  return length === 0 ? Math.hypot(p.x - a.x, p.y - a.y) : Math.abs((p.x - a.x) * dy - (p.y - a.y) * dx) / length;
}

function flattenCubic(p0: Point, p1: Point, p2: Point, p3: Point, out: Ring, depth: number): void {
  if (depth >= 18 || 0.75 * Math.max(chordDistance(p1, p0, p3), chordDistance(p2, p0, p3)) <= FLATTEN_TOLERANCE_M * 0.9) {
    out.push(p3);
    return;
  }
  const m = (a: Point, b: Point): Point => ({ x: (a.x + b.x) / 2, y: (a.y + b.y) / 2 });
  const p01 = m(p0, p1);
  const p12 = m(p1, p2);
  const p23 = m(p2, p3);
  const p012 = m(p01, p12);
  const p123 = m(p12, p23);
  const mid = m(p012, p123);
  flattenCubic(p0, p01, p012, mid, out, depth + 1);
  flattenCubic(mid, p123, p23, p3, out, depth + 1);
}

function flattenArc(from: Point, rxIn: number, ryIn: number, rotationDeg: number, largeArc: boolean, sweep: boolean, to: Point, out: Ring): void {
  let rx = Math.abs(rxIn);
  let ry = Math.abs(ryIn);
  if ((from.x === to.x && from.y === to.y) || rx === 0 || ry === 0) {
    out.push(to);
    return;
  }
  const { sin, cos } = sinCosDeg(rotationDeg);
  const dx = (from.x - to.x) / 2;
  const dy = (from.y - to.y) / 2;
  const x1 = cos * dx + sin * dy;
  const y1 = -sin * dx + cos * dy;
  const lambda = (x1 * x1) / (rx * rx) + (y1 * y1) / (ry * ry);
  if (lambda > 1) {
    rx *= Math.sqrt(lambda);
    ry *= Math.sqrt(lambda);
  }
  const numerator = rx * rx * ry * ry - rx * rx * y1 * y1 - ry * ry * x1 * x1;
  const denominator = rx * rx * y1 * y1 + ry * ry * x1 * x1;
  const factor = (largeArc === sweep ? -1 : 1) * Math.sqrt(Math.max(0, numerator / denominator));
  const cxp = (factor * rx * y1) / ry;
  const cyp = (-factor * ry * x1) / rx;
  const cx = cos * cxp - sin * cyp + (from.x + to.x) / 2;
  const cy = sin * cxp + cos * cyp + (from.y + to.y) / 2;
  const theta1 = Math.atan2((y1 - cyp) / ry, (x1 - cxp) / rx);
  const theta2 = Math.atan2((-y1 - cyp) / ry, (-x1 - cxp) / rx);
  let delta = theta2 - theta1;
  if (!sweep && delta > 0) delta -= 2 * Math.PI;
  else if (sweep && delta < 0) delta += 2 * Math.PI;
  const steps = arcSteps(Math.max(rx, ry), delta, 1);
  for (let i = 1; i < steps; i++) {
    const theta = theta1 + (delta * i) / steps;
    const ex = rx * Math.cos(theta);
    const ey = ry * Math.sin(theta);
    out.push({ x: cos * ex - sin * ey + cx, y: sin * ex + cos * ey + cy });
  }
  out.push(to);
}

/** 〰️ The flattened subpaths of an SVG path (`M L H V C S Q T A Z`, absolute and relative, y up); `closed` is true when every subpath ends in `Z`. Curves deviate at most 0.5 mm from the analytic curve. */
export type FlatPath = { readonly rings: readonly (readonly Point[])[]; readonly closed: boolean };

const NUMBER = /[+-]?(?:\d+\.?\d*|\.\d+)(?:[eE][+-]?\d+)?/y;
const PARAMETERS: Readonly<Record<string, number>> = { m: 2, l: 2, h: 1, v: 1, c: 6, s: 4, q: 4, t: 2, a: 7, z: 0 };
const PATHS = new Map<string, FlatPath>();

/** 〰️ Parses and flattens an SVG path string; throws with the offset on malformed input. */
export function flattenPath(d: string): FlatPath {
  const cached = PATHS.get(d);
  if (cached !== undefined) return cached;
  let index = 0;
  const skip = (): void => {
    while (index < d.length && /[\s,]/.test(d[index]!)) index++;
  };
  const fail = (what: string): never => {
    throw new Error(`invalid path data at offset ${index}: ${what} in "${d}"`);
  };
  const number = (): number => {
    skip();
    NUMBER.lastIndex = index;
    const match = NUMBER.exec(d);
    if (match === null) return fail("number expected");
    index = NUMBER.lastIndex;
    return Number(match[0]);
  };
  const flag = (): boolean => {
    skip();
    const c = d[index];
    if (c !== "0" && c !== "1") return fail("arc flag 0 or 1 expected");
    index++;
    return c === "1";
  };
  const rings: Ring[] = [];
  const closedRings: boolean[] = [];
  let ring: Ring = [];
  let current: Point = { x: 0, y: 0 };
  let start: Point = current;
  let previousControl: Point | null = null;
  let previousKind = "";
  let command = "";
  const begin = (at: Point): void => {
    ring = [at];
    rings.push(ring);
    closedRings.push(false);
    start = at;
  };
  skip();
  if (d[index] !== "M" && d[index] !== "m") fail("path must start with M");
  while (true) {
    skip();
    if (index >= d.length) break;
    const c = d[index]!;
    if (/[a-zA-Z]/.test(c)) {
      if (!(c.toLowerCase() in PARAMETERS)) fail(`unknown command ${c}`);
      command = c;
      index++;
      if (c === "Z" || c === "z") {
        closedRings[closedRings.length - 1] = true;
        current = start;
        previousControl = null;
        previousKind = "z";
        continue;
      }
    } else if (command === "" || command === "Z" || command === "z") fail("command expected");
    const relative = command === command.toLowerCase();
    const kind = command.toLowerCase();
    const ox = relative ? current.x : 0;
    const oy = relative ? current.y : 0;
    if (ring.length === 0 || previousKind === "z") if (kind !== "m") begin(current);
    switch (kind) {
      case "m": {
        const p = { x: ox + number(), y: oy + number() };
        begin(p);
        current = p;
        command = relative ? "l" : "L";
        previousControl = null;
        break;
      }
      case "l": {
        current = { x: ox + number(), y: oy + number() };
        ring.push(current);
        previousControl = null;
        break;
      }
      case "h": {
        current = { x: ox + number(), y: current.y };
        ring.push(current);
        previousControl = null;
        break;
      }
      case "v": {
        current = { x: current.x, y: oy + number() };
        ring.push(current);
        previousControl = null;
        break;
      }
      case "c": {
        const p1 = { x: ox + number(), y: oy + number() };
        const p2 = { x: ox + number(), y: oy + number() };
        const p3 = { x: ox + number(), y: oy + number() };
        flattenCubic(current, p1, p2, p3, ring, 0);
        current = p3;
        previousControl = p2;
        break;
      }
      case "s": {
        const p1 = previousControl !== null && (previousKind === "c" || previousKind === "s") ? { x: 2 * current.x - previousControl.x, y: 2 * current.y - previousControl.y } : current;
        const p2 = { x: ox + number(), y: oy + number() };
        const p3 = { x: ox + number(), y: oy + number() };
        flattenCubic(current, p1, p2, p3, ring, 0);
        current = p3;
        previousControl = p2;
        break;
      }
      case "q":
      case "t": {
        const control: Point = kind === "q" ? { x: ox + number(), y: oy + number() } : previousControl !== null && (previousKind === "q" || previousKind === "t") ? { x: 2 * current.x - previousControl.x, y: 2 * current.y - previousControl.y } : current;
        const p3 = { x: ox + number(), y: oy + number() };
        const p1 = { x: current.x + (2 / 3) * (control.x - current.x), y: current.y + (2 / 3) * (control.y - current.y) };
        const p2 = { x: p3.x + (2 / 3) * (control.x - p3.x), y: p3.y + (2 / 3) * (control.y - p3.y) };
        flattenCubic(current, p1, p2, p3, ring, 0);
        current = p3;
        previousControl = control;
        break;
      }
      case "a": {
        const rx = number();
        const ry = number();
        const rotation = number();
        const large = flag();
        const sweep = flag();
        const p = { x: ox + number(), y: oy + number() };
        flattenArc(current, rx, ry, rotation, large, sweep, p, ring);
        current = p;
        previousControl = null;
        break;
      }
    }
    previousKind = kind;
  }
  const result: FlatPath = { rings: rings.filter((r) => r.length > 1), closed: closedRings.length > 0 && closedRings.every(Boolean) };
  if (PATHS.size > 4096) PATHS.clear();
  PATHS.set(d, result);
  return result;
}

function circle(cx: number, cy: number, rx: number, ry: number, rotationDeg: number, steps: number): Ring {
  const { sin, cos } = sinCosDeg(rotationDeg);
  const ring: Ring = [];
  for (let i = 0; i < steps; i++) {
    const theta = (2 * Math.PI * i) / steps;
    const ex = rx * Math.cos(theta);
    const ey = ry * Math.sin(theta);
    ring.push({ x: cx + cos * ex - sin * ey, y: cy + sin * ex + cos * ey });
  }
  return ring;
}

function capsule(x0: number, y0: number, x1: number, y1: number, r0: number, r1: number): Ring {
  const dx = x1 - x0;
  const dy = y1 - y0;
  const d = Math.hypot(dx, dy);
  if (d <= Math.abs(r0 - r1)) return r0 >= r1 ? circle(x0, y0, r0, r0, 0, arcSteps(r0, 2 * Math.PI, 32)) : circle(x1, y1, r1, r1, 0, arcSteps(r1, 2 * Math.PI, 32));
  const phi = Math.atan2(dy, dx);
  const alpha = Math.acos((r0 - r1) / d);
  const ring: Ring = [];
  const far = arcSteps(r1, 2 * alpha, 12);
  for (let i = 0; i <= far; i++) {
    const theta = phi - alpha + (2 * alpha * i) / far;
    ring.push({ x: x1 + r1 * Math.cos(theta), y: y1 + r1 * Math.sin(theta) });
  }
  const near = arcSteps(r0, 2 * (Math.PI - alpha), 12);
  for (let i = 0; i <= near; i++) {
    const theta = phi + alpha + (2 * (Math.PI - alpha) * i) / near;
    ring.push({ x: x0 + r0 * Math.cos(theta), y: y0 + r0 * Math.sin(theta) });
  }
  return ring;
}

function roundedRect(x: number, y: number, w: number, h: number, rIn: number): Ring {
  const r = Math.min(Math.max(rIn, 0), Math.abs(w) / 2, Math.abs(h) / 2);
  if (r === 0) return [{ x, y }, { x: x + w, y }, { x: x + w, y: y + h }, { x, y: y + h }];
  const steps = arcSteps(r, Math.PI / 2, 4);
  const ring: Ring = [];
  const corners: readonly (readonly [number, number, number])[] = [[x + w - r, y + r, -Math.PI / 2], [x + w - r, y + h - r, 0], [x + r, y + h - r, Math.PI / 2], [x + r, y + r, Math.PI]];
  for (const [cx, cy, from] of corners) for (let i = 0; i <= steps; i++) ring.push({ x: cx + r * Math.cos(from + ((Math.PI / 2) * i) / steps), y: cy + r * Math.sin(from + ((Math.PI / 2) * i) / steps) });
  return ring;
}

/** ✏️ The local-space rings of a shape geometry and whether they are closed. */
export function flattenGeometry(shape: Shape): FlatPath {
  switch (shape.kind) {
    case "capsule": return { rings: [capsule(shape.x0, shape.y0, shape.x1, shape.y1, shape.r0, shape.r1)], closed: true };
    case "ellipse": return { rings: [circle(shape.cx, shape.cy, shape.rx, shape.ry, shape.rotationDeg ?? 0, Math.max(32, arcSteps(Math.max(shape.rx, shape.ry), 2 * Math.PI, 32)))], closed: true };
    case "rect": return { rings: [roundedRect(shape.x, shape.y, shape.w, shape.h, shape.r ?? 0)], closed: true };
    case "poly": return { rings: [shape.points.map(([x, y]) => ({ x, y }))], closed: shape.closed };
    case "path": return flattenPath(shape.d);
  }
}
//#endregion 〰️Flattening

//#region 🌍️Placement
const REST = new WeakMap<Asset, Record<Slug, Affine>>();

function restWorld(asset: Asset): Record<Slug, Affine> {
  let rest = REST.get(asset);
  if (rest === undefined) REST.set(asset, (rest = solveRig(asset.rig, {}, { x: 0, y: 0, angleDeg: 0 })));
  return rest;
}

function placeShape(asset: Asset, shape: Shape, world: Readonly<Record<Slug, Affine>>, viewpoint: Viewpoint, identity: boolean): EvaluatedShape {
  const matrix = world[shape.bone];
  if (matrix === undefined) throw new Error(`shape "${shape.id}" of asset "${asset.id}" is bound to unknown bone "${shape.bone}"`);
  const flat = flattenGeometry(shape);
  if (identity) return { shape, rings: flat.rings, closed: flat.closed };
  const dx = viewpoint === "plan" ? matrix[4] - restWorld(asset)[shape.bone]![4] : 0;
  const rings = flat.rings.map((ring) => ring.map((p) => (viewpoint === "plan" ? { x: p.x + dx, y: p.y } : apply(matrix, p.x, p.y))));
  return { shape, rings, closed: flat.closed };
}

function anchorOf(asset: Asset, id: Slug): Anchor {
  const anchor = asset.anchors.find((candidate) => candidate.id === id);
  if (anchor === undefined) throw new Error(`asset "${asset.id}" has no anchor "${id}"`);
  return anchor;
}

function boneOf(asset: Asset, id: Slug): Bone {
  const bone = asset.rig.bones.find((candidate) => candidate.id === id);
  if (bone === undefined) throw new Error(`asset "${asset.id}" has no bone "${id}"`);
  return bone;
}

function worldForces(asset: Asset, load: AppliedLoad, scale: number, world: Readonly<Record<Slug, Affine>>): WorldForce[] {
  const base = { kind: load.kind, title: load.title };
  const forces: WorldForce[] = [];
  const placeAt = (): { bone: Slug; at: Point } => {
    if (load.anchor !== undefined) {
      const anchor = anchorOf(asset, load.anchor);
      return { bone: anchor.bone, at: apply(world[anchor.bone]!, anchor.x, anchor.y) };
    }
    if (load.bone !== undefined) return { bone: load.bone, at: apply(world[boneOf(asset, load.bone).id]!, load.x ?? 0, load.y ?? 0) };
    throw new Error(`load "${load.id}" of asset "${asset.id}" has neither anchor nor bone`);
  };
  const hasPoint = load.fxN !== 0 || load.fyN !== 0 || (load.momentNm ?? 0) !== 0;
  if (hasPoint || load.distributed === undefined) {
    const { bone, at } = placeAt();
    forces.push({ id: load.id, at, fxN: load.fxN * scale, fyN: load.fyN * scale, momentNm: (load.momentNm ?? 0) * scale, bone, ...base });
  }
  const d = load.distributed;
  if (d !== undefined) {
    const matrix = world[boneOf(asset, d.bone).id]!;
    const length = (d.toM - d.fromM) / DISTRIBUTED_SUBPOINTS;
    for (let i = 0; i < DISTRIBUTED_SUBPOINTS; i++) {
      const s = d.fromM + (i + 0.5) * length;
      forces.push({ id: `${load.id}#${i + 1}`, at: apply(matrix, s, 0), fxN: d.wxNPerM * length * scale, fyN: d.wyNPerM * length * scale, momentNm: 0, bone: d.bone, ...base });
    }
  }
  return forces;
}
//#endregion 🌍️Placement

//#region ⚙️Kinematics
function massCentres(asset: Asset, world: Readonly<Record<Slug, Affine>>): Record<Slug, { readonly at: Point; readonly kg: number }> {
  const result: Record<Slug, { at: Point; kg: number }> = {};
  for (const bone of asset.rig.bones) if (bone.mass !== undefined && bone.mass.kg > 0) result[bone.id] = { at: apply(world[bone.id]!, bone.mass.comX, bone.mass.comY), kg: bone.mass.kg };
  return result;
}

function centreOf(masses: Readonly<Record<Slug, { readonly at: Point; readonly kg: number }>>): Point {
  let m = 0;
  let x = 0;
  let y = 0;
  for (const { at, kg } of Object.values(masses)) {
    m += kg;
    x += at.x * kg;
    y += at.y * kg;
  }
  return m === 0 ? { x: 0, y: 0 } : { x: x / m, y: y / m };
}

type Timeline = { readonly sampleAt: (timeS: number) => Sample; readonly loop: boolean; readonly durationS: number };

function kinematics(asset: Asset, timeline: Timeline | null, timeS: number): EvaluatedState["kinematics"] {
  const massed = asset.rig.bones.filter((bone) => bone.mass !== undefined && bone.mass.kg > 0).map((bone) => bone.id);
  const zero: Record<Slug, Point> = {};
  for (const id of massed) zero[id] = { x: 0, y: 0 };
  if (timeline === null || massed.length === 0) return { comVelocity: { x: 0, y: 0 }, segmentAccelerations: zero };
  const h = KINEMATIC_STEP_S;
  const mid = timeline.loop ? timeS : wrapTime(timeS, timeline.durationS, false);
  const lo = timeline.loop ? mid - h : Math.max(mid - h, 0);
  const hi = timeline.loop ? mid + h : Math.min(mid + h, timeline.durationS);
  const at = (t: number) => {
    const sample = timeline.sampleAt(t);
    return massCentres(asset, solveRig(asset.rig, sample.offsets, sample.root));
  };
  const before = at(lo);
  const now = at(mid);
  const after = at(hi);
  const symmetric = timeline.loop || (mid - h >= 0 && mid + h <= timeline.durationS);
  const span = hi - lo;
  const accelerations: Record<Slug, Point> = {};
  for (const id of massed) {
    const a = before[id]!.at;
    const b = now[id]!.at;
    const c = after[id]!.at;
    accelerations[id] = symmetric ? { x: (c.x - 2 * b.x + a.x) / (h * h), y: (c.y - 2 * b.y + a.y) / (h * h) } : { x: 0, y: 0 };
  }
  const p0 = centreOf(before);
  const p1 = centreOf(after);
  return { comVelocity: span > 0 ? { x: (p1.x - p0.x) / span, y: (p1.y - p0.y) / span } : { x: 0, y: 0 }, segmentAccelerations: accelerations };
}
//#endregion ⚙️Kinematics

type Geometry = { readonly representation: EvaluatedState["representation"]; readonly elevation: readonly Shape[]; readonly plan: readonly Shape[]; readonly paper: boolean; readonly paperUnits?: boolean };

function geometryOf(asset: Asset, selector: RepresentationSelector | undefined): Geometry {
  if (selector === undefined) return { representation: { kind: "none" }, elevation: [], plan: [], paper: false };
  if (selector.kind === "symbol") {
    const symbol = asset.symbols.find((candidate) => candidate.id === selector.symbol);
    if (symbol === undefined) throw new Error(`asset "${asset.id}" has no symbol "${selector.symbol}"; symbols: ${asset.symbols.map((s) => s.id).join(", ")}`);
    return { representation: { kind: "symbol", symbol: symbol.id }, elevation: symbol.geometry.elevation, plan: symbol.geometry.plan ?? [], paper: true, paperUnits: symbol.units === "paper-mm" };
  }
  const picked = pickRepresentation(asset, selector.scale.denominator);
  if (picked.kind === "unavailable") throw new RepresentationError(picked, asset.id);
  const { representation } = picked;
  return { representation: { kind: "scale", denominator: picked.denominator, requested: picked.requested, exact: picked.exact }, elevation: representation.geometry.elevation, plan: representation.geometry.plan ?? [], paper: false };
}

type Extent = { minX: number; minY: number; maxX: number; maxY: number };

function extentOf(points: readonly Point[]): Extent {
  const e: Extent = { minX: Number.POSITIVE_INFINITY, minY: Number.POSITIVE_INFINITY, maxX: Number.NEGATIVE_INFINITY, maxY: Number.NEGATIVE_INFINITY };
  for (const p of points) {
    if (p.x < e.minX) e.minX = p.x;
    if (p.x > e.maxX) e.maxX = p.x;
    if (p.y < e.minY) e.minY = p.y;
    if (p.y > e.maxY) e.maxY = p.y;
  }
  return e;
}

/** 🦴️ The extent of the rig itself (bone origins, tips and anchors): physical, identical for every representation. */
export function rigBounds(state: Pick<EvaluatedState, "bones" | "anchors">): EvaluatedState["bounds"] {
  const e = extentOf([...Object.values(state.bones).flatMap((bone) => [bone.origin, bone.tip]), ...Object.values(state.anchors).map((anchor) => anchor.at)]);
  return Number.isFinite(e.minX) ? e : { minX: 0, minY: 0, maxX: 0, maxY: 0 };
}

function withRespiration(asset: Asset, query: EvaluationQuery, sample: Sample): { readonly sample: Sample; readonly respiration: RespirationState | null } {
  const respiration = asset.thermal.respiration === undefined ? null : respirationState(asset, query);
  const extra: Readonly<Record<Slug, BoneOffset>> = respiration === null ? {} : respirationOffsets(asset, respiration);
  const ids = Object.keys(extra);
  if (ids.length === 0) return { sample, respiration };
  const offsets = { ...sample.offsets };
  for (const id of ids) {
    const base = offsets[id];
    const add = extra[id]!;
    if (base !== undefined) offsets[id] = { angleDeg: base.angleDeg + (add.angleDeg ?? 0), dx: base.dx + (add.dx ?? 0), dy: base.dy + (add.dy ?? 0) };
  }
  return { sample: { ...sample, offsets }, respiration };
}

function sampleQuery(asset: Asset, query: EvaluationQuery): { readonly sample: Sample; readonly timeline: Timeline | null } {
  if (query.kind === "pose") return { sample: samplePose(asset, poseOf(asset, query.pose)), timeline: null };
  if (query.kind === "clip") {
    const clip = clipOf(asset, query.clip);
    return { sample: sampleClip(asset, clip, query.timeS), timeline: { sampleAt: (t) => sampleClip(asset, clip, t), loop: clip.loop, durationS: clip.durationS } };
  }
  const clip = transition(asset, query.from, query.to, query.durationS);
  return { sample: sampleClip(asset, clip, query.timeS), timeline: { sampleAt: (t) => sampleClip(asset, clip, t), loop: false, durationS: clip.durationS } };
}

/** 🧮️ The state of the asset at the query on the absolute timeline: the pose, the clip at a time (the pose wraps for looping clips, `cycle` counts the loops, planted root advance and respiration continue) or the transition between two poses, with the geometry of the selected representation. `frame = floor(t · fps + 1e-9)` with the clip's fps; `timeS` is the wrapped or clamped clip time (the query's own time for poses). Throws {@link RepresentationError} when the selected scale is unavailable and clear errors for unknown poses, clips, symbols, anchors, bones or loads without placement. */
export function evaluate(asset: Asset, query: EvaluationQuery, selector?: RepresentationSelector): EvaluatedState {
  const geometry = geometryOf(asset, selector);
  const { sample: pure, timeline } = sampleQuery(asset, query);
  const { sample, respiration } = withRespiration(asset, query, pure);
  const world = solveRig(asset.rig, sample.offsets, sample.root);
  const bones: Record<Slug, EvaluatedBone> = {};
  for (const bone of asset.rig.bones) {
    const matrix = world[bone.id]!;
    const effective = effectiveOffset(bone, sample.offsets[bone.id]);
    bones[bone.id] = { id: bone.id, world: matrix, origin: apply(matrix, 0, 0), tip: apply(matrix, bone.lengthM, 0), angleRad: rotationRad(matrix), offset: effective };
  }
  const shapes: Record<Viewpoint, EvaluatedShape[]> = {
    elevation: geometry.elevation.map((shape) => placeShape(asset, shape, world, "elevation", geometry.paper)),
    plan: geometry.plan.map((shape) => placeShape(asset, shape, world, "plan", geometry.paper)),
  };
  const anchors: Record<Slug, { anchor: Anchor; at: Point }> = {};
  for (const anchor of asset.anchors) anchors[anchor.id] = { anchor, at: apply(world[anchor.bone]!, anchor.x, anchor.y) };
  const loads: WorldForce[] = [];
  for (const load of sample.loads) {
    const scale = sample.loadScales[load.id] ?? 1;
    if (scale !== 0) loads.push(...worldForces(asset, load, scale, world));
  }
  const drawn = geometry.paperUnits === true ? [] : shapes.elevation.filter((evaluated) => evaluated.shape.role !== "ground").flatMap((evaluated) => evaluated.rings.flat());
  const extent = extentOf(drawn);
  const base = { bones, anchors };
  const bounds = Number.isFinite(extent.minX) ? extent : rigBounds(base);
  return {
    asset,
    query,
    timeS: query.kind === "pose" ? (query.timeS ?? 0) : sample.timeS,
    frame: sample.frame,
    phase: sample.phase,
    bones,
    shapes,
    anchors,
    contacts: sample.contacts,
    loads,
    thermal: sample.thermal,
    respiration,
    cycle: query.kind === "clip" ? cycleOf(query.timeS, clipOf(asset, query.clip).durationS, clipOf(asset, query.clip).loop) : 0,
    representation: geometry.representation,
    bounds,
    kinematics: kinematics(asset, timeline, query.kind === "pose" ? (query.timeS ?? 0) : query.timeS),
  };
}
