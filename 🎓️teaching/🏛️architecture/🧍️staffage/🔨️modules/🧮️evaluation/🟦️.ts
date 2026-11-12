/** 🧮️ Evaluation of an asset at a query: the single deterministic state every view, analysis and export derives from.
 *
 * `evaluate(asset, query, selector)` takes the geometry from the selected representation (the authored drawing for a scale, or a named symbol; without a selector no geometry is
 * placed) and everything physical (bones, anchors, contacts, loads, thermal inputs, respiration, kinematics) from the asset alone, so it is identical for every selector. It samples the pose, clip or transition, solves the rig, flattens every shape to world rings (curves to 0.5 mm), places anchors, converts loads to world forces
 * (distributed loads to eight equivalent resultants per bone segment, scaled by `load:<id>` tracks), binds the thermal inputs and derives segment accelerations and the centre-of-mass
 * velocity by central differences of the evaluated clip with h = 1/240 s. Equal inputs yield bit-identical outputs.
 *
 * One three-dimensional chain ({@link solveChain}) carries every view: each bone exposes its 3D `frame`, `tip` and the lateral `z` of its anchors, and the elevation, front (lateral, up) and plan (ahead, lateral) shapes are carried from the rest frame to the current frame of their bone by that one transform ({@link carry}) and projected. Only a `followTip` rect or poly on a bone that lies along the viewing axis grows outwards, and a plan capsule with `followTip` follows the origin and the tip of its bone. `bounds` is the extent of the selected elevation geometry except the role `ground` (the drawn extent; the rig extent is {@link rigBounds}), and of the rig when nothing is drawn.
 * Clip queries live on the absolute timeline: the pose wraps, `cycle` counts the loops, planted root advance and respiration continue.
 *
 * @see ../../🧬️schema/🟦️.ts — `Asset`, `Shape`
 * @see ../🔗️contracts/🟦️.ts — `EvaluatedState`, `EvaluationQuery`
 * @see https://www.w3.org/TR/SVG11/paths.html — the path grammar
 */
import type { Anchor, Asset, AppliedLoad, Bone, BoneOffset, Shape, Slug, Solid, PlanarViewpoint, Text } from "../../🧬️schema/🟦️.ts";
import type { Affine, EvaluatedBone, EvaluatedShape, EvaluatedState, EvaluationQuery, Point, RepresentationSelector, RespirationState, WorldForce } from "../🔗️contracts/🟦️.ts";
import { PLANT_HZ, clipOf, cycleOf, poseOf, sampleClip, samplePose, transition, wrapTime } from "../🎞️animation/🟦️.ts";
import { respirationOffsets, respirationState } from "../💨️transport/🫁️respiration/🟦️.ts";
import { unionPolygons } from "../🔺️polygon/🟦️.ts";
import { RepresentationError, pickRepresentation } from "../🖨️representation/🧭️pick/🟦️.ts";
import { ViewMissingError } from "../🎨️depiction/🔭️views/🟦️.ts";
import type { Sample } from "../🎞️animation/🟦️.ts";
import { apply, carry, effectiveOffset, restFrames, rotationRad, sinCosDeg, solveChain, solveRig } from "../🦴️rig/🟦️.ts";
import type { Frame3, SolvedFrame } from "../🦴️rig/🟦️.ts";
import { nearness, project } from "../🎨️depiction/🔭️views/🟦️.ts";
import * as portable from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

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
  const step = ratio >= 1 ? Math.PI / 2 : 2 * portable.acos(1 - ratio);
  return Math.min(512, Math.max(minimum, Math.ceil(Math.abs(sweep) / step)));
}

function chordDistance(p: Point, a: Point, b: Point): number {
  const dx = b.x - a.x;
  const dy = b.y - a.y;
  const length = portable.hypot(dx, dy);
  return length === 0 ? portable.hypot(p.x - a.x, p.y - a.y) : Math.abs((p.x - a.x) * dy - (p.y - a.y) * dx) / length;
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
  const theta1 = portable.atan2((y1 - cyp) / ry, (x1 - cxp) / rx);
  const theta2 = portable.atan2((-y1 - cyp) / ry, (-x1 - cxp) / rx);
  let delta = theta2 - theta1;
  if (!sweep && delta > 0) delta -= 2 * Math.PI;
  else if (sweep && delta < 0) delta += 2 * Math.PI;
  const steps = arcSteps(Math.max(rx, ry), delta, 1);
  for (let i = 1; i < steps; i++) {
    const theta = theta1 + (delta * i) / steps;
    const ex = rx * portable.cos(theta);
    const ey = ry * portable.sin(theta);
    out.push({ x: cos * ex - sin * ey + cx, y: sin * ex + cos * ey + cy });
  }
  out.push(to);
}

/** 〰️ The flattened subpaths of an SVG path (`M L H V C S Q T A Z`, absolute and relative, y up); `closed` is true when every subpath ends in `Z`. Curves deviate at most 0.5 mm from the analytic curve. */
export type FlatPath = { readonly rings: readonly (readonly Point[])[]; readonly closed: boolean };

const NUMBER = /[+-]?(?:\d+\.?\d*|\.\d+)(?:[eE][+-]?\d+)?/y;
const PARAMETERS: Readonly<Record<string, number>> = { m: 2, l: 2, h: 1, v: 1, c: 6, s: 4, q: 4, t: 2, a: 7, z: 0 };
const PATHS = new Map<string, FlatPath>();

/** 🧵️ Parses and flattens an SVG path string; throws with the offset on malformed input. */
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
    const ex = rx * portable.cos(theta);
    const ey = ry * portable.sin(theta);
    ring.push({ x: cx + cos * ex - sin * ey, y: cy + sin * ex + cos * ey });
  }
  return ring;
}

function capsule(x0: number, y0: number, x1: number, y1: number, r0: number, r1: number): Ring {
  const dx = x1 - x0;
  const dy = y1 - y0;
  const d = portable.hypot(dx, dy);
  if (d <= Math.abs(r0 - r1)) return r0 >= r1 ? circle(x0, y0, r0, r0, 0, arcSteps(r0, 2 * Math.PI, 32)) : circle(x1, y1, r1, r1, 0, arcSteps(r1, 2 * Math.PI, 32));
  const phi = portable.atan2(dy, dx);
  const alpha = portable.acos((r0 - r1) / d);
  const ring: Ring = [];
  const far = arcSteps(r1, 2 * alpha, 12);
  for (let i = 0; i <= far; i++) {
    const theta = phi - alpha + (2 * alpha * i) / far;
    ring.push({ x: x1 + r1 * portable.cos(theta), y: y1 + r1 * portable.sin(theta) });
  }
  const near = arcSteps(r0, 2 * (Math.PI - alpha), 12);
  for (let i = 0; i <= near; i++) {
    const theta = phi + alpha + (2 * (Math.PI - alpha) * i) / near;
    ring.push({ x: x0 + r0 * portable.cos(theta), y: y0 + r0 * portable.sin(theta) });
  }
  return ring;
}

function roundedRect(x: number, y: number, w: number, h: number, rIn: number): Ring {
  const r = Math.min(Math.max(rIn, 0), Math.abs(w) / 2, Math.abs(h) / 2);
  if (r === 0) return [{ x, y }, { x: x + w, y }, { x: x + w, y: y + h }, { x, y: y + h }];
  const steps = arcSteps(r, Math.PI / 2, 4);
  const ring: Ring = [];
  const corners: readonly (readonly [number, number, number])[] = [[x + w - r, y + r, -Math.PI / 2], [x + w - r, y + h - r, 0], [x + r, y + h - r, Math.PI / 2], [x + r, y + r, Math.PI]];
  for (const [cx, cy, from] of corners) for (let i = 0; i <= steps; i++) ring.push({ x: cx + r * portable.cos(from + ((Math.PI / 2) * i) / steps), y: cy + r * portable.sin(from + ((Math.PI / 2) * i) / steps) });
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
    case "member": return { rings: [], closed: true };
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

const CIRCLE_SEGMENTS = 64;
const UNION_CIRCLE_SEGMENTS = 32;

/** 🧊️ The current and the rest 3D frames of the bones of a spread or twisted rig. */
type Chain3 = { readonly now: Readonly<Record<Slug, SolvedFrame>>; readonly rest: Readonly<Record<Slug, SolvedFrame>> };

const REST3 = new WeakMap<Asset, Record<Slug, SolvedFrame>>();

function restOf(asset: Asset): Record<Slug, SolvedFrame> {
  let rest = REST3.get(asset);
  if (rest === undefined) REST3.set(asset, (rest = restFrames(asset.rig)));
  return rest;
}

/** ➡️ A front or plan point of a bone carried by the 3D transform from its rest frame to its current frame and projected to the view: a plan point `(x, y)` is the rest point (x, height of the bone origin, lateral y), a front point `(x, y)` the rest point (forward position of the bone origin, height y, lateral x). */
function carryPoint(kin: Chain3, bone: Slug, viewpoint: PlanarViewpoint, p: Point, depth?: number): Point {
  const rest = kin.rest[bone]!.frame;
  const now = kin.now[bone]!.frame;
  if (viewpoint === "plan") {
    const q = carry(rest, now, [p.x, depth ?? rest.o[1], p.y]);
    return { x: q[0], y: q[2] };
  }
  const q = carry(rest, now, [depth ?? rest.o[0], p.y, p.x]);
  return { x: q[2], y: q[1] };
}

/** 👁️ Whether a bone has at least half of its length as extent in the view at rest (height in front, forward position in plan); a bone along the viewing axis has none and its `followTip` shapes grow outwards instead. */
function inView(asset: Asset, bone: Slug, viewpoint: PlanarViewpoint): boolean {
  const length = boneOf(asset, bone).lengthM;
  const rest = restWorld(asset)[bone]!;
  const tip = apply(rest, length, 0);
  return length > 0 && Math.abs(viewpoint === "front" ? tip.y - rest[5] : tip.x - rest[4]) >= 0.5 * length;
}

type Vec3 = readonly [number, number, number];

const vec3 = (v: Vec3): { readonly x: number; readonly y: number; readonly z: number } => ({ x: v[0], y: v[1], z: v[2] });

const cross = (a: Vec3, b: Vec3): Vec3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const dot = (a: Vec3, b: Vec3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const norm = (a: Vec3): Vec3 => {
  const l = portable.hypot(a[0], a[1], a[2]);
  return [a[0] / l, a[1] / l, a[2] / l];
};

/** 🔺️ The convex hull of points (monotone chain), counter-clockwise. */
function hull(points: readonly Point[]): Point[] {
  const sorted = [...points].sort((a, b) => a.x - b.x || a.y - b.y);
  if (sorted.length < 3) return sorted;
  const turn = (o: Point, a: Point, b: Point): number => (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
  const build = (input: readonly Point[]): Point[] => {
    const out: Point[] = [];
    for (const p of input) {
      while (out.length >= 2 && turn(out[out.length - 2]!, out[out.length - 1]!, p) <= 0) out.pop();
      out.push(p);
    }
    out.pop();
    return out;
  };
  return [...build(sorted), ...build([...sorted].reverse())];
}

/** ✂️ Douglas-Peucker simplification of a ring to a tolerance in metres (the first and the farthest point are kept). */
function simplifyRing(ring: readonly Point[], tolerance: number): Point[] {
  if (ring.length <= 8) return [...ring];
  const keep = new Array<boolean>(ring.length).fill(false);
  let far = 0;
  let best = -1;
  ring.forEach((p, i) => {
    const d = portable.hypot(p.x - ring[0]!.x, p.y - ring[0]!.y);
    if (d > best) [far, best] = [i, d];
  });
  keep[0] = keep[far] = true;
  const visit = (from: number, to: number): void => {
    const a = ring[from % ring.length]!;
    const b = ring[to % ring.length]!;
    const length = portable.hypot(b.x - a.x, b.y - a.y) || 1;
    let at = -1;
    let worst = tolerance;
    for (let i = from + 1; i < to; i++) {
      const p = ring[i % ring.length]!;
      const d = Math.abs((p.x - a.x) * (b.y - a.y) - (p.y - a.y) * (b.x - a.x)) / length;
      if (d > worst) [at, worst] = [i, d];
    }
    if (at >= 0) {
      keep[at % ring.length] = true;
      visit(from, at);
      visit(at, to);
    }
  };
  visit(0, far);
  visit(far, ring.length);
  return ring.filter((_, i) => keep[i]);
}

const projected = (viewpoint: PlanarViewpoint, q: Vec3): Point => project(viewpoint, vec3(q));

type MemberShape = Extract<Shape, { readonly kind: "member" }>;

const MEMBER_SEGMENTS = 32;

/** 🧱️ The corners of the two end sections of a `member` solid in world metres: the end points are carried by the frames of their bones (`from` by the shape's own bone, `to` by `to.bone`); a rectangular section has the width `widthM` along the part of the normal `v` of the start bone perpendicular to the bar and the depth `depthM` along the remaining perpendicular axis (the frame completes the orthonormal triple `bar`, width, depth), a circular section is `MEMBER_SEGMENTS` points of the diameter `diameterM` about the bar. */
export function memberSections(member: MemberShape, frameOf: (id: Slug) => Frame3): readonly [readonly Vec3[], readonly Vec3[]] {
  const at = (id: Slug, p: { readonly x: number; readonly y: number; readonly z: number }): Vec3 => {
    const f = frameOf(id);
    return [f.o[0] + p.x * f.u[0] + p.y * f.v[0] + p.z * f.w[0], f.o[1] + p.x * f.u[1] + p.y * f.v[1] + p.z * f.w[1], f.o[2] + p.x * f.u[2] + p.y * f.v[2] + p.z * f.w[2]];
  };
  const bone = member.bone;
  const a = at(member.from.bone ?? bone, member.from);
  const b = at(member.to.bone ?? bone, member.to);
  const bar = norm([b[0] - a[0], b[1] - a[1], b[2] - a[2]]);
  const start = frameOf(member.from.bone ?? bone);
  const across = (reference: Vec3): Vec3 => {
    const k = dot(reference, bar);
    return [reference[0] - k * bar[0], reference[1] - k * bar[1], reference[2] - k * bar[2]];
  };
  const normal = across(start.v);
  const width = norm(portable.hypot(...normal) > 1e-9 ? normal : across(start.w));
  const depth = cross(bar, width);
  const { section } = member;
  const offsets: readonly (readonly [number, number])[] =
    section.kind === "circle"
      ? Array.from({ length: MEMBER_SEGMENTS }, (_, i) => [(section.d / 2) * portable.cos((2 * Math.PI * i) / MEMBER_SEGMENTS), (section.d / 2) * portable.sin((2 * Math.PI * i) / MEMBER_SEGMENTS)] as const)
      : ([[-1, -1], [1, -1], [1, 1], [-1, 1]] as const).map(([s, t]) => [(s * section.w) / 2, (t * section.h) / 2] as const);
  const corner = (centre: Vec3, [s, t]: readonly [number, number]): Vec3 => [centre[0] + s * width[0] + t * depth[0], centre[1] + s * width[1] + t * depth[1], centre[2] + s * width[2] + t * depth[2]];
  return [offsets.map((o) => corner(a, o)), offsets.map((o) => corner(b, o))];
}

/** 🔭️ A {@link Solid} placed by the full 3D frames of its bone and projected to the view (`elevation` is the side view): an extruded part is the profile swept from `z0` to `z1` along the lateral axis of the frame (`sweep` `w`: the profile in the (u, v) plane, `z` measured from the rest origin of the bone in world z) or along the bone normal (`sweep` `v`: the profile in the (u, w) plane, `z` measured from the bone origin along v), the span tapering linearly along u to `zTip` at the bone length (clamped to the bone), as ONE outline, the union of the profile at both ends and the sweep of every edge; a revolved part is revolved about the longitudinal axis of its axis bone (its own rest direction, whatever its rest tilt, carried to the current frame), one outline, the union of the hulls of the projected circles of the two ends of every profile edge; without `mirror` the half profile itself is drawn: each profile point keeps its signed distance along the forward direction of the axis bone and its height along the axis, moved into the lateral direction of the view (front) or left in the sagittal plane (elevation). */
function solid3(solid: Solid, profile: readonly (readonly Point[])[], kin: Chain3, bone: Slug, viewpoint: PlanarViewpoint, length: number): Point[][] {
  const rings = profile.map((ring) => simplifyRing(ring, 0.0003));
  const now = kin.now[bone]!.frame;
  const rest = kin.rest[bone]!.frame;
  const place = (a: number, b: number, c: number): Vec3 => [now.o[0] + a * now.u[0] + b * now.v[0] + c * now.w[0], now.o[1] + a * now.u[1] + b * now.v[1] + c * now.w[1], now.o[2] + a * now.u[2] + b * now.v[2] + c * now.w[2]];
  if (solid.kind === "extruded") {
    const lateral = solid.sweep === "w";
    const base = lateral ? rest.o[2] : 0;
    const span = (a: number): readonly [number, number] => {
      if (solid.zTip === undefined) return [solid.z0 - base, solid.z1 - base];
      const t = length > 0 ? Math.min(Math.max(a / length, 0), 1) : 0;
      return [solid.z0 + (solid.zTip[0] - solid.z0) * t - base, solid.z1 + (solid.zTip[1] - solid.z1) * t - base];
    };
    const at = (p: Point, c: number): Point => projected(viewpoint, lateral ? place(p.x, p.y, c) : place(p.x, c, p.y));
    return rings.flatMap((ring) => {
      const low = ring.map((p) => at(p, span(p.x)[0]));
      const high = ring.map((p) => at(p, span(p.x)[1]));
      const sweeps = low.map((p, i) => [p, low[(i + 1) % low.length]!, high[(i + 1) % high.length]!, high[i]!]);
      return unionPolygons([low, high, ...sweeps]);
    });
  }
  const axisRest = kin.rest[solid.axisBone]!.frame;
  const axisNow = kin.now[solid.axisBone]!.frame;
  const axisAt: Vec3 = [axisRest.o[0] + solid.axisX, axisRest.o[1], axisRest.o[2] + (solid.axisZ ?? 0)];
  const origin = carry(axisRest, axisNow, axisAt);
  const shift: Vec3 = [0, 0, 0];
  const profileZ = axisAt[2] - rest.o[2];
  const axis = norm(axisNow[solid.axis]);
  const e1 = norm(portable.hypot(...cross(axis, [0, 0, 1])) > 1e-9 ? cross(axis, [0, 0, 1]) : cross(axis, [1, 0, 0]));
  const e2 = cross(axis, e1);
  const circle = (p: Point): Point[] => {
    const q = place(p.x, p.y, profileZ);
    const d: Vec3 = [q[0] - origin[0], q[1] - origin[1], q[2] - origin[2]];
    const h = dot(d, axis);
    const radial: Vec3 = [d[0] - h * axis[0], d[1] - h * axis[1], d[2] - h * axis[2]];
    const r = portable.hypot(...radial);
    return Array.from({ length: UNION_CIRCLE_SEGMENTS }, (_, i) => {
      const t = (2 * Math.PI * i) / UNION_CIRCLE_SEGMENTS;
      return projected(viewpoint, [origin[0] + shift[0] + h * axis[0] + r * (portable.cos(t) * e1[0] + portable.sin(t) * e2[0]), origin[1] + shift[1] + h * axis[1] + r * (portable.cos(t) * e1[1] + portable.sin(t) * e2[1]), origin[2] + shift[2] + h * axis[2] + r * (portable.cos(t) * e1[2] + portable.sin(t) * e2[2])]);
    });
  };
  if (solid.mirror !== true && viewpoint !== "plan") {
    const forward = carry(axisRest, axisNow, [axisAt[0] + 1, axisAt[1], axisAt[2]]);
    const f0: Vec3 = [forward[0] - origin[0], forward[1] - origin[1], forward[2] - origin[2]];
    const lateral: Vec3 = (() => {
      const k = dot([0, 0, 1], axis);
      return norm([0 - k * axis[0], 0 - k * axis[1], 1 - k * axis[2]]);
    })();
    const along: Vec3 = [f0[0] - dot(f0, axis) * axis[0], f0[1] - dot(f0, axis) * axis[1], f0[2] - dot(f0, axis) * axis[2]];
    const meridian = portable.hypot(...along) > 1e-6 ? norm(along) : norm(solid.axis === "v" ? axisNow.u : axisNow.v);
    return rings.map((ring) =>
      ring.map((p) => {
        const q = place(p.x, p.y, profileZ);
        const d: Vec3 = [q[0] - origin[0], q[1] - origin[1], q[2] - origin[2]];
        const h = dot(d, axis);
        const rho = dot(d, meridian);
        const direction = viewpoint === "elevation" ? meridian : lateral;
        return projected(viewpoint, [origin[0] + shift[0] + h * axis[0] + rho * direction[0], origin[1] + shift[1] + h * axis[1] + rho * direction[1], origin[2] + shift[2] + h * axis[2] + rho * direction[2]]);
      }),
    );
  }
  return rings.flatMap((ring) => unionPolygons(ring.map((p, i) => hull([...circle(p), ...circle(ring[(i + 1) % ring.length]!)]))));
}

/** 🦿️ The projected displacement of the origin and of the tip of a bone from the rest pose, `[ahead, up, left]`. */
function displacement(kin: Chain3, bone: Slug, length: number): { readonly origin: readonly [number, number, number]; readonly tip: readonly [number, number, number] } {
  const now = kin.now[bone]!.frame;
  const rest = kin.rest[bone]!.frame;
  const at = (f: Frame3, s: number): [number, number, number] => [f.o[0] + f.u[0] * s, f.o[1] + f.u[1] * s, f.o[2] + f.u[2] * s];
  const [o1, o0, t1, t0] = [at(now, 0), at(rest, 0), at(now, length), at(rest, length)];
  return { origin: [o1[0] - o0[0], o1[1] - o0[1], o1[2] - o0[2]], tip: [t1[0] - t0[0], t1[1] - t0[1], t1[2] - t0[2]] };
}

/** 🌟️ The depth a flat front or plan shape is carried at: the height (plan) or the forward position (front) of the middle of its elevation twin (same id, same bone) at rest, so a rigid part far from the bone origin pivots about its true place; without a twin the rest position of the bone origin (author such a part as a solid instead). */
type DepthProfile = (key: number) => number;

const DEPTH_SAMPLES = 48;
const DEPTHS = new WeakMap<Shape, DepthProfile | null>();

function depthProfile(asset: Asset, shape: Shape, twins: ReadonlyMap<string, readonly Shape[]> | undefined, viewpoint: PlanarViewpoint): DepthProfile | undefined {
  const cached = DEPTHS.get(shape);
  if (cached !== undefined) return cached ?? undefined;
  const sources = twins?.get(`${shape.id} ${shape.bone}`) ?? twins?.get(`* ${shape.bone}`);
  const rest = restWorld(asset)[shape.bone]!;
  const plan = viewpoint === "plan";
  const rings = (sources ?? []).flatMap((source) => flattenGeometry(source).rings.map((ring) => ring.map((p) => apply(rest, p.x, p.y))));
  const keys = rings.flat().map((p) => (plan ? p.x : p.y));
  if (keys.length === 0) {
    DEPTHS.set(shape, null);
    return undefined;
  }
  if (twins?.has(`${shape.id} ${shape.bone}`) === true) {
    const own = rings.flat().map((p) => (plan ? p.y : p.x));
    const middle = (Math.min(...own) + Math.max(...own)) / 2;
    const constant: DepthProfile = () => middle;
    DEPTHS.set(shape, constant);
    return constant;
  }
  const lo = Math.min(...keys);
  const hi = Math.max(...keys);
  const depths: number[] = [];
  for (let i = 0; i < DEPTH_SAMPLES; i++) {
    const key = lo + ((hi - lo) * i) / (DEPTH_SAMPLES - 1);
    let low = Number.POSITIVE_INFINITY;
    let high = Number.NEGATIVE_INFINITY;
    for (const ring of rings)
      for (let j = 0; j < ring.length; j++) {
        const a = ring[j]!;
        const b = ring[(j + 1) % ring.length]!;
        const [ka, kb, da, db] = plan ? [a.x, b.x, a.y, b.y] : [a.y, b.y, a.x, b.x];
        if (Math.min(ka, kb) - 1e-12 > key || Math.max(ka, kb) + 1e-12 < key) continue;
        for (const v of kb === ka ? [da, db] : [da + ((db - da) * (key - ka)) / (kb - ka)]) {
          low = Math.min(low, v);
          high = Math.max(high, v);
        }
      }
    depths.push(Number.isFinite(low) ? (low + high) / 2 : Number.NaN);
  }
  const across = rings.flat().map((p) => (plan ? p.y : p.x));
  const fallback = (Math.min(...across) + Math.max(...across)) / 2;
  const filled = depths.map((d) => (Number.isNaN(d) ? fallback : d));
  const profile: DepthProfile = (key) => {
    if (hi === lo) return filled[0]!;
    const f = Math.min(Math.max((key - lo) / (hi - lo), 0), 1) * (DEPTH_SAMPLES - 1);
    const i = Math.min(Math.floor(f), DEPTH_SAMPLES - 2);
    return filled[i]! + (filled[i + 1]! - filled[i]!) * (f - i);
  };
  DEPTHS.set(shape, profile);
  return profile;
}

/** 📐️ A front or plan shape placed by the single 3D path: every shape is carried by the transform from the rest frame to the current frame of its bone and projected to the view (rotations about any axis, spread, twist and the sagittal swing alike); a revolved or extruded solid is built in 3D from its profile ({@link solid3}); a plan capsule with `followTip` has its first end follow the origin and its second the tip of the bone; a `followTip` rect or poly on a bone that has no extent in the view (a bone along the viewing axis) additionally grows from the displacement of the origin to that of the tip, by the lateral position across the shape from its inner end to its outer end. */
function placeProjected(asset: Asset, shape: Shape, flat: { readonly rings: readonly (readonly Point[])[]; readonly closed: boolean }, kin: Chain3, viewpoint: Exclude<PlanarViewpoint, "elevation">, twins: ReadonlyMap<string, readonly Shape[]> | undefined): EvaluatedShape {
  const bone = boneOf(asset, shape.bone);
  const plan = viewpoint === "plan";
  const moved = displacement(kin, shape.bone, bone.lengthM);
  if (plan && shape.kind === "capsule" && shape.followTip === true) return { shape, rings: [capsule(shape.x0 + moved.origin[0], shape.y0 + moved.origin[2], shape.x1 + moved.tip[0], shape.y1 + moved.tip[2], shape.r0, shape.r1)], closed: true };
  const grows = (shape.kind === "rect" || shape.kind === "poly") && shape.followTip === true && !inView(asset, shape.bone, viewpoint);
  const profile = grows ? undefined : depthProfile(asset, shape, twins, viewpoint);
  const carried = flat.rings.map((ring) => ring.map((p) => carryPoint(kin, shape.bone, viewpoint, p, profile?.(plan ? p.x : p.y))));
  const outward = Math.sign(moved.tip[2] - moved.origin[2]);
  if (!grows || outward === 0) return { shape, rings: carried, closed: flat.closed };
  const across = flat.rings.flat().map((p) => (plan ? p.y : p.x));
  const lo = Math.min(...across);
  const hi = Math.max(...across);
  const dx = moved.tip[0] - moved.origin[0];
  const dy = moved.tip[1] - moved.origin[1];
  const dz = moved.tip[2] - moved.origin[2];
  const rings = carried.map((ring, i) =>
    ring.map((p, j) => {
      const c = across[0] === undefined ? 0 : plan ? flat.rings[i]![j]!.y : flat.rings[i]![j]!.x;
      const u = hi === lo ? 1 : outward > 0 ? (c - lo) / (hi - lo) : (hi - c) / (hi - lo);
      return plan ? { x: p.x + dx * u, y: p.y + dz * u } : { x: p.x + dz * u, y: p.y + dy * u };
    }),
  );
  return { shape, rings, closed: flat.closed };
}

/** 📷️ How near a shape is to the viewer along the view direction, larger is nearer: the nearest point of the shape carried by the 3D frames of its bone (the same carry that places it) — the highest point in the plan, the foremost in the front, the point of the smallest lateral position in the elevation (the right of the asset faces the viewer) — so a broad or rotated shape is ranked by its extent, not by its bone origin and tip. A `solid` is carried at the plane of its profile; a flat shape at the depth profile it was placed with. */
export function shapeDepth(state: EvaluatedState, shape: Shape, viewpoint: PlanarViewpoint): number {
  const vec = (v: { readonly x: number; readonly y: number; readonly z: number }): Vec3 => [v.x, v.y, v.z];
  const frameOf = (id: Slug): Frame3 => {
    const f = state.bones[id]!.frame;
    return { o: vec(f.o), u: vec(f.u), v: vec(f.v), w: vec(f.w) };
  };
  return state.bones[shape.bone] === undefined ? 0 : shapeDepthIn(state.asset, frameOf, shape, viewpoint);
}

/** 📤️ The rest-pose counterpart of {@link shapeDepth}: how near the shape is when every bone is at rest. */
export function restShapeDepth(asset: Asset, shape: Shape, viewpoint: PlanarViewpoint): number {
  const rest = restOf(asset);
  return rest[shape.bone] === undefined ? 0 : shapeDepthIn(asset, (id) => rest[id]!.frame, shape, viewpoint);
}

function shapeDepthIn(asset: Asset, frameOf: (id: Slug) => Frame3, shape: Shape, viewpoint: PlanarViewpoint): number {
  const now = frameOf(shape.bone);
  if (shape.kind === "member") {
    const [start, end] = memberSections(shape, frameOf);
    return Math.max(...[...start, ...end].map((q) => nearness(viewpoint, vec3(q))));
  }
  const rest = restOf(asset)[shape.bone];
  if (rest === undefined) return 0;
  const points = flattenGeometry(shape).rings.flat();
  if (points.length === 0) return 0;
  const plan = viewpoint === "plan";
  const profile = DEPTHS.get(shape) ?? undefined;
  const local = (p: Point, c: number): Vec3 => [now.o[0] + p.x * now.u[0] + p.y * now.v[0] + c * now.w[0], now.o[1] + p.x * now.u[1] + p.y * now.v[1] + c * now.w[1], now.o[2] + p.x * now.u[2] + p.y * now.v[2] + c * now.w[2]];
  const world = (p: Point): Vec3 => {
    if (viewpoint === "elevation") return local(p, 0);
    if (shape.solid !== undefined) return local(p, shape.solid.kind === "extruded" ? (shape.solid.z0 + shape.solid.z1) / 2 - rest.frame.o[2] : 0);
    return plan ? carry(rest.frame, now, [p.x, profile?.(p.x) ?? rest.frame.o[1], p.y]) : carry(rest.frame, now, [profile?.(p.y) ?? rest.frame.o[0], p.y, p.x]);
  };
  return Math.max(...points.map((p) => nearness(viewpoint, vec3(world(p)))));
}

/** 🚫️ Thrown when a `member` is drawn in paper units (a nominal symbol): a bar has no paper-millimetre outline, so the symbol must draw it with planar shapes and the member is never silently dropped. */
export class MemberInPaperUnitsError extends Error {
  readonly asset: string;
  readonly shape: string;

  constructor(asset: string, shape: string) {
    super(`member "${shape}" of asset "${asset}" cannot be drawn in paper units: a nominal symbol draws planar shapes only`);
    this.name = "MemberInPaperUnitsError";
    this.asset = asset;
    this.shape = shape;
  }
}

/** 🚨️ Thrown when an `extruded` solid hint has no `sweep` (`v` or `w`): the axis the profile is swept along is required and never defaulted (D47), so a part is never drawn transposed in the front and plan views. The validator reports the same defect as `solid-sweep`. */
export class SolidSweepError extends Error {
  readonly asset: string;
  readonly shape: string;
  readonly viewpoint: PlanarViewpoint;

  constructor(asset: string, shape: string, viewpoint: PlanarViewpoint) {
    super(`extruded solid of shape "${shape}" (${viewpoint}) of asset "${asset}" has no sweep: state "w" (lateral) or "v" (along the bone normal), there is no default`);
    this.name = "SolidSweepError";
    this.asset = asset;
    this.shape = shape;
    this.viewpoint = viewpoint;
  }

  /** 🌍️ The refusal in both languages. */
  get reason(): Text {
    return { en: this.message, de: `Der extrudierte Körper der Form "${this.shape}" (${this.viewpoint}) des Assets "${this.asset}" hat keinen sweep: "w" (seitlich) oder "v" (entlang der Knochennormalen) angeben, es gibt keinen Standardwert.` };
  }
}

function placeShape(asset: Asset, shape: Shape, world: Readonly<Record<Slug, Affine>>, viewpoint: PlanarViewpoint, identity: boolean, kin: Chain3 | undefined = undefined, twins: ReadonlyMap<string, readonly Shape[]> | undefined = undefined): EvaluatedShape {
  const matrix = world[shape.bone];
  if (matrix === undefined) throw new Error(`shape "${shape.id}" of asset "${asset.id}" is bound to unknown bone "${shape.bone}"`);
  const flat = flattenGeometry(shape);
  if (identity && shape.kind === "member") throw new MemberInPaperUnitsError(asset.id, shape.id);
  if (identity) return { shape, rings: flat.rings, closed: flat.closed };
  if (shape.kind === "member" && kin !== undefined) {
    const [start, end] = memberSections(shape, (id) => kin.now[id]!.frame);
    return { shape, rings: [hull([...start, ...end].map((q) => projected(viewpoint, q)))], closed: true };
  }
  if (kin !== undefined && shape.solid !== undefined) {
    const sweep = shape.solid.kind === "extruded" ? (shape.solid as { sweep?: string }).sweep : "w";
    if (sweep !== "v" && sweep !== "w") throw new SolidSweepError(asset.id, shape.id, viewpoint);
  }
  if (kin !== undefined && shape.solid !== undefined) return { shape, rings: solid3(shape.solid, flat.rings, kin, shape.bone, viewpoint, boneOf(asset, shape.bone).lengthM), closed: true };
  if (viewpoint === "elevation" || kin === undefined) return { shape, rings: flat.rings.map((ring) => ring.map((p) => apply(matrix, p.x, p.y))), closed: flat.closed };
  return placeProjected(asset, shape, flat, kin, viewpoint, twins);
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

/** 🧲️ The forces of one authored load at its 3D position: anchor, bone point or bone segment, placed by the bone frames (so a lateral `z`, `fzN` and the `x`, `y` moments are kept) and scaled by the load track. */
function worldForces(asset: Asset, load: AppliedLoad, scale: number, frames: Readonly<Record<Slug, SolvedFrame>>): WorldForce[] {
  const base = { kind: load.kind, title: load.title };
  const forces: WorldForce[] = [];
  const place = (bone: Slug, a: number, b: number, c: number): { readonly x: number; readonly y: number; readonly z: number } => {
    const f = frames[bone]!.frame;
    return { x: f.o[0] + a * f.u[0] + b * f.v[0] + c * f.w[0], y: f.o[1] + a * f.u[1] + b * f.v[1] + c * f.w[1], z: f.o[2] + a * f.u[2] + b * f.v[2] + c * f.w[2] };
  };
  const placeAt = (): { bone: Slug; at: { readonly x: number; readonly y: number; readonly z: number } } => {
    if (load.anchor !== undefined) {
      const anchor = anchorOf(asset, load.anchor);
      return { bone: anchor.bone, at: place(anchor.bone, anchor.x, anchor.y, anchor.z ?? 0) };
    }
    if (load.bone !== undefined) return { bone: load.bone, at: place(boneOf(asset, load.bone).id, load.x ?? 0, load.y ?? 0, load.z ?? 0) };
    throw new Error(`load "${load.id}" of asset "${asset.id}" has neither anchor nor bone`);
  };
  const hasPoint = [load.fxN, load.fyN, load.fzN ?? 0, load.momentXNm ?? 0, load.momentYNm ?? 0, load.momentNm ?? 0].some((v) => v !== 0);
  if (hasPoint || load.distributed === undefined) {
    const { bone, at } = placeAt();
    forces.push({ id: load.id, at: { x: at.x, y: at.y }, z: at.z, fxN: load.fxN * scale, fyN: load.fyN * scale, fzN: (load.fzN ?? 0) * scale, mxNm: (load.momentXNm ?? 0) * scale, myNm: (load.momentYNm ?? 0) * scale, mzNm: (load.momentNm ?? 0) * scale, bone, ...base });
  }
  const d = load.distributed;
  if (d !== undefined) {
    const length = (d.toM - d.fromM) / DISTRIBUTED_SUBPOINTS;
    for (let i = 0; i < DISTRIBUTED_SUBPOINTS; i++) {
      const at = place(boneOf(asset, d.bone).id, d.fromM + (i + 0.5) * length, 0, 0);
      forces.push({ id: `${load.id}#${i + 1}`, at: { x: at.x, y: at.y }, z: at.z, fxN: d.wxNPerM * length * scale, fyN: d.wyNPerM * length * scale, fzN: (d.wzNPerM ?? 0) * length * scale, mxNm: 0, myNm: 0, mzNm: 0, bone: d.bone, ...base });
    }
  }
  return forces;
}
//#endregion 🌍️Placement

//#region ⚙️Kinematics
type Mass3 = { readonly at: { readonly x: number; readonly y: number; readonly z: number }; readonly kg: number };

function massCentres(asset: Asset, frames: Readonly<Record<Slug, SolvedFrame>>): Record<Slug, Mass3> {
  const result: Record<Slug, Mass3> = {};
  for (const bone of asset.rig.bones)
    if (bone.mass !== undefined && bone.mass.kg > 0) {
      const f = frames[bone.id]!.frame;
      const { comX, comY } = bone.mass;
      result[bone.id] = { at: { x: f.o[0] + comX * f.u[0] + comY * f.v[0], y: f.o[1] + comX * f.u[1] + comY * f.v[1], z: f.o[2] + comX * f.u[2] + comY * f.v[2] }, kg: bone.mass.kg };
    }
  return result;
}

function centreOf(masses: Readonly<Record<Slug, Mass3>>): { x: number; y: number; z: number } {
  let m = 0;
  let [x, y, z] = [0, 0, 0];
  for (const { at, kg } of Object.values(masses)) {
    m += kg;
    x += at.x * kg;
    y += at.y * kg;
    z += at.z * kg;
  }
  return m === 0 ? { x: 0, y: 0, z: 0 } : { x: x / m, y: y / m, z: z / m };
}

type Timeline = { readonly sampleAt: (timeS: number) => Sample; readonly loop: boolean; readonly durationS: number };

function kinematics(asset: Asset, timeline: Timeline | null, timeS: number): EvaluatedState["kinematics"] {
  const massed = asset.rig.bones.filter((bone) => bone.mass !== undefined && bone.mass.kg > 0).map((bone) => bone.id);
  const zero: Record<Slug, { x: number; y: number; z: number }> = {};
  for (const id of massed) zero[id] = { x: 0, y: 0, z: 0 };
  if (timeline === null || massed.length === 0) return { comVelocity: { x: 0, y: 0, z: 0 }, segmentAccelerations: zero };
  const h = KINEMATIC_STEP_S;
  const mid = timeline.loop ? timeS : wrapTime(timeS, timeline.durationS, false);
  const lo = timeline.loop ? mid - h : Math.max(mid - h, 0);
  const hi = timeline.loop ? mid + h : Math.min(mid + h, timeline.durationS);
  const at = (t: number) => {
    const sample = timeline.sampleAt(t);
    return massCentres(asset, solveChain(asset.rig, sample.offsets, sample.root));
  };
  const before = at(lo);
  const now = at(mid);
  const after = at(hi);
  const symmetric = timeline.loop || (mid - h >= 0 && mid + h <= timeline.durationS);
  const held = !timeline.loop && (timeS < 0 || timeS > timeline.durationS);
  const direction = mid - h < 0 ? 1 : -1;
  const reach = [1, 2, 3].map((k) => mid + direction * k * h);
  const oneSided = !symmetric && !held && reach.every((t) => t >= 0 && t <= timeline.durationS);
  const side = oneSided ? reach.map(at) : [];
  const span = hi - lo;
  const accelerations: Record<Slug, { x: number; y: number; z: number }> = {};
  const axes = ["x", "y", "z"] as const;
  for (const id of massed) {
    const a = before[id]!.at;
    const b = now[id]!.at;
    const c = after[id]!.at;
    if (symmetric) accelerations[id] = { x: (c.x - 2 * b.x + a.x) / (h * h), y: (c.y - 2 * b.y + a.y) / (h * h), z: (c.z - 2 * b.z + a.z) / (h * h) };
    else if (oneSided) {
      const [p1, p2, p3] = side.map((s) => s[id]!.at) as [Mass3["at"], Mass3["at"], Mass3["at"]];
      const d = (axis: (typeof axes)[number]): number => (2 * b[axis] - 5 * p1[axis] + 4 * p2[axis] - p3[axis]) / (h * h);
      accelerations[id] = { x: d("x"), y: d("y"), z: d("z") };
    } else accelerations[id] = { x: 0, y: 0, z: 0 };
  }
  const p0 = centreOf(before);
  const p1 = centreOf(after);
  return { comVelocity: span > 0 ? { x: (p1.x - p0.x) / span, y: (p1.y - p0.y) / span, z: (p1.z - p0.z) / span } : { x: 0, y: 0, z: 0 }, segmentAccelerations: accelerations };
}
//#endregion ⚙️Kinematics

type Geometry = { readonly representation: EvaluatedState["representation"]; readonly elevation: readonly Shape[]; readonly front: readonly Shape[]; readonly plan: readonly Shape[]; readonly paper: boolean; readonly paperUnits?: boolean };

function viewOf(asset: Asset, shapes: readonly Shape[] | undefined, viewpoint: PlanarViewpoint): readonly Shape[] {
  if (shapes === undefined) throw new ViewMissingError(asset.id, viewpoint);
  return shapes;
}

function geometryOf(asset: Asset, selector: RepresentationSelector | undefined): Geometry {
  if (selector === undefined) return { representation: { kind: "none" }, elevation: [], front: [], plan: [], paper: false };
  if (selector.kind === "symbol") {
    const symbol = asset.symbols.find((candidate) => candidate.id === selector.symbol);
    if (symbol === undefined) throw new Error(`asset "${asset.id}" has no symbol "${selector.symbol}"; symbols: ${asset.symbols.map((s) => s.id).join(", ")}`);
    return { representation: { kind: "symbol", symbol: symbol.id }, elevation: symbol.geometry.elevation, front: viewOf(asset, symbol.geometry.front, "front"), plan: viewOf(asset, symbol.geometry.plan, "plan"), paper: true, paperUnits: symbol.units === "paper-mm" };
  }
  const picked = pickRepresentation(asset, selector.scale.denominator);
  if (picked.kind === "unavailable") throw new RepresentationError(picked, asset.id);
  const { representation } = picked;
  return { representation: { kind: "scale", denominator: picked.denominator, requested: picked.requested, exact: picked.exact }, elevation: representation.geometry.elevation, front: viewOf(asset, representation.geometry.front, "front"), plan: viewOf(asset, representation.geometry.plan, "plan"), paper: false };
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
    if (!asset.rig.bones.some((bone) => bone.id === id)) throw new Error(`respiration of asset "${asset.id}" drives unknown bone "${id}"`);
    const base = offsets[id] ?? { angleDeg: 0, abductionDeg: 0, twistDeg: 0, dx: 0, dy: 0 };
    const add = extra[id]!;
    offsets[id] = { angleDeg: base.angleDeg + (add.angleDeg ?? 0), abductionDeg: base.abductionDeg + (add.abductionDeg ?? 0), twistDeg: base.twistDeg + (add.twistDeg ?? 0), dx: base.dx + (add.dx ?? 0), dy: base.dy + (add.dy ?? 0) };
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

/** 🎬️ The sample every view of a query is placed from: pose, clip or transition offsets and root with the respiration offsets added. */
export function sampleOf(asset: Asset, query: EvaluationQuery): Sample {
  return withRespiration(asset, query, sampleQuery(asset, query).sample).sample;
}

/** ▶️ The state of the asset at the query on the absolute timeline: the pose, the clip at a time (the pose wraps for looping clips, `cycle` counts the loops, planted root advance and respiration continue) or the transition between two poses, with the geometry of the selected representation. `frame = floor(t · fps + 1e-9)` with the clip's fps; `timeS` is the absolute query time (never wrapped, 0 for a pose without time) and `clipTimeS` the wrapped or clamped clip phase (0 for poses). Throws {@link RepresentationError} when the selected scale is unavailable and clear errors for unknown poses, clips, symbols, anchors, bones or loads without placement. */
export function evaluate(asset: Asset, query: EvaluationQuery, selector?: RepresentationSelector): EvaluatedState {
  const geometry = geometryOf(asset, selector);
  const { sample: pure, timeline } = sampleQuery(asset, query);
  const { sample, respiration } = withRespiration(asset, query, pure);
  const world = solveRig(asset.rig, sample.offsets, sample.root);
  const kin: Chain3 = { now: solveChain(asset.rig, sample.offsets, sample.root), rest: restOf(asset) };
  const bones: Record<Slug, EvaluatedBone> = {};
  for (const bone of asset.rig.bones) {
    const matrix = world[bone.id]!;
    const effective = effectiveOffset(bone, sample.offsets[bone.id]);
    const f = kin.now[bone.id]!.frame;
    const tip = vec3([f.o[0] + f.u[0] * bone.lengthM, f.o[1] + f.u[1] * bone.lengthM, f.o[2] + f.u[2] * bone.lengthM]);
    bones[bone.id] = { id: bone.id, world: matrix, origin: apply(matrix, 0, 0), tip, frame: { o: vec3(f.o), u: vec3(f.u), v: vec3(f.v), w: vec3(f.w) }, angleRad: rotationRad(matrix), offset: effective };
  }
  const twins = new Map<string, Shape[]>();
  for (const shape of geometry.elevation) {
    if (shape.solid !== undefined) continue;
    twins.set(`${shape.id} ${shape.bone}`, [shape]);
    if (shape.role !== "ground") twins.set(`* ${shape.bone}`, [...(twins.get(`* ${shape.bone}`) ?? []), shape]);
  }
  const shapes: Record<PlanarViewpoint, EvaluatedShape[]> = {
    elevation: geometry.elevation.map((shape) => placeShape(asset, shape, world, "elevation", geometry.paper, kin)),
    front: geometry.front.map((shape) => placeShape(asset, shape, world, "front", geometry.paper, kin, twins)),
    plan: geometry.plan.map((shape) => placeShape(asset, shape, world, "plan", geometry.paper, kin, twins)),
  };
  const anchors: Record<Slug, { anchor: Anchor; at: Point; z: number }> = {};
  for (const anchor of asset.anchors) {
    const f = kin.now[anchor.bone]!.frame;
    const side = anchor.z ?? 0;
    const flat = apply(world[anchor.bone]!, anchor.x, anchor.y);
    anchors[anchor.id] = { anchor, at: { x: flat.x + side * f.w[0], y: flat.y + side * f.w[1] }, z: f.o[2] + anchor.x * f.u[2] + anchor.y * f.v[2] + side * f.w[2] };
  }
  const loads: WorldForce[] = [];
  for (const load of sample.loads) {
    const scale = sample.loadScales[load.id] ?? 1;
    if (scale !== 0) loads.push(...worldForces(asset, load, scale, kin.now));
  }
  const drawn = geometry.paperUnits === true ? [] : shapes.elevation.filter((evaluated) => evaluated.shape.role !== "ground").flatMap((evaluated) => evaluated.rings.flat());
  const extent = extentOf(drawn);
  const base = { bones, anchors };
  const bounds = Number.isFinite(extent.minX) ? extent : rigBounds(base);
  return {
    asset,
    query,
    timeS: query.timeS ?? 0,
    clipTimeS: query.kind === "pose" ? 0 : sample.clipTimeS,
    frame: sample.frame,
    phase: sample.phase,
    root: { x: sample.root.x, y: sample.root.y, z: 0, angleDeg: sample.root.angleDeg },
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
