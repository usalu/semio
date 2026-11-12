/** 🏗️ Structural solver of the staffage archive: one {@link EvaluatedState} in, one {@link StructuralResult} out, solved in physical three-dimensional coordinates.
 *
 * Model: the evaluated pose is an instantaneously rigid body loaded by segment weights (and, when `structure.dynamics` is `inertial`, by
 * d'Alembert forces `-m·a` from the evaluated kinematics) at the true 3D positions of the bone frames. Supports are elastic springs of equal normalised stiffness; the 6x6
 * rigid-body equilibrium (three translations, three rotations) is solved for small displacements with a pseudo-inverse, unilateral contacts that would pull are dropped, and friction is capped at `μ·N`
 * in the whole tangent plane (forward and lateral). Joint loads and member diagrams are exact free-body cuts of the same external force set. Nothing is projected here: the views read the vectors.
 *
 * Sign conventions: world x ahead, y up, z the asset's left; a positive moment about axis k turns i towards j for the cyclic order (i, j, k) (about z it is counter-clockwise in the side view).
 * Axial N > 0 tension; shear V = -(Σ distal F · v) in the rig plane and Vz = -(Σ distal F · w) laterally; bending M = Σ moment of the distal externals about the cut, `momentNm` its component about the bone's lateral axis `w`
 * (M > 0 sagging, tension at the local -y face), `momentLateralNm` about the bone normal `v`, `torsionNm` about the bone axis `u`. Deflection points are `(s, w)` in the bone frame, w along local +y.
 *
 * @see ../🔗️contracts/🟦️.ts — the runtime contracts
 * @see ../../🧬️schema/🟦️.ts — the authored masses, sections, anchors and structural model
 * @see ./📽️projection/🟦️.ts — how the vectors are seen in elevation, plan and front
 * @see ../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/structure-model.md — equations, tolerances, oracle results
 */
import type { Anchor, Bone, Slug, SupportType, Text } from "../../🧬️schema/🟦️.ts";
import type { DiagramStation, DisplayRange, EvaluatedState, ForceQuantity, JointLoad, Legend, MemberDiagram, Point, Reaction, StructuralResult, Vec3, WorldForce } from "../🔗️contracts/🟦️.ts";
import * as portable from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

//#region 🔢️Constants
/** 🔢️ Sub-forces a uniform segment weight is discretised into. */
export const WEIGHT_SUBDIVISIONS = 16;

/** 🔢️ Equidistant stations per member diagram (intervals); load and attachment points add two-sided stations. */
export const DIAGRAM_INTERVALS = 24;

/** 🔢️ Integration intervals of the Euler-Bernoulli deflection line. */
export const DEFLECTION_INTERVALS = 120;

/** 🔢️ Friction coefficient assumed where an anchor authors none. */
export const DEFAULT_FRICTION = 0.6;

/** 🔢️ Normalised normal, tangential and rotational (per m²) spring stiffness of every support. */
export const SUPPORT_STIFFNESS = { normal: 1, tangential: 0.5, translation: 1, rotation: 1 } as const;

/** 🎨️ Colour ramp: compression warm, tension cool, zero grey. */
export const STRUCTURAL_COLORS = { compressionStrong: "#d7301f", compressionWeak: "#fdae61", zero: "#d9d9d9", tensionWeak: "#abd9e9", tensionStrong: "#2c7bb6" } as const;

const POSITION_TOLERANCE = 1e-9;
const MAX_ITERATIONS = 96;
const NULL_SPACE = 1e-12;
//#endregion 🔢️Constants

//#region 🧱️Types
/** 🧱️ A quantity a structural legend can describe. */
export type StructuralQuantity = ForceQuantity | "moment";

/** 🏹️ One arrow of the free-body diagram. */
export type FreeBodyForce = { readonly id: string; readonly kind: "weight" | "inertial" | "applied" | "reaction"; readonly bone: Slug | null; readonly at: Vec3; readonly fxN: number; readonly fyN: number; readonly fzN: number; readonly mxNm: number; readonly myNm: number; readonly mzNm: number; readonly title: Text };

/** 🧮️ A world vector as a triple. */
export type V3 = readonly [number, number, number];

type Piece = { readonly force: WorldForce; readonly span: readonly [number, number] | null };
type Sums = { f: V3; m0: V3 };
type Frame = { readonly o: V3; readonly u: V3; readonly v: V3; readonly w: V3 };
type Frames = Readonly<Record<Slug, Frame>>;
type Row = { readonly anchor: string; readonly role: "n" | "t1" | "t2" | "x" | "y" | "z" | "rx" | "ry" | "rz" | "ml" | "mw"; readonly d: readonly number[]; readonly k: number };
type Support = { readonly anchor: Slug; readonly bone: Slug; readonly at: V3; readonly type: SupportType; readonly n: V3; readonly t: V3; readonly t2: V3; readonly mu: number; readonly def: Anchor };
type Solved = { f: V3; m: V3; normal: number; ml: number; mw: number; active: boolean };
//#endregion 🧱️Types

//#region 🧰️Helpers
const text = (en: string, de: string): Text => ({ en, de });
const sumOf = (xs: readonly number[]): number => xs.reduce((a, b) => a + b, 0);
const ZERO: V3 = [0, 0, 0];
const Z_AXIS: V3 = [0, 0, 1];
const add = (a: V3, b: V3): V3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const scaled = (a: V3, s: number): V3 => [a[0] * s, a[1] * s, a[2] * s];
const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const norm = (a: V3): number => portable.hypot(a[0], a[1], a[2]);
const tuple = (a: Vec3): V3 => [a.x, a.y, a.z];
const vec = (a: V3): Vec3 => ({ x: a[0], y: a[1], z: a[2] });
const posOf = (f: WorldForce): V3 => [f.at.x, f.at.y, f.z];
const forceOf = (f: WorldForce): V3 => [f.fxN, f.fyN, f.fzN];
const coupleOf = (f: WorldForce): V3 => [f.mxNm, f.myNm, f.mzNm];
const moment0 = (f: WorldForce): V3 => add(cross(posOf(f), forceOf(f)), coupleOf(f));

/** 🧲️ A world force at a 3D point with an optional couple. */
export function worldForce(id: Slug, bone: Slug, kind: string, title: Text, at: V3, f: V3, m: V3 = ZERO): WorldForce {
  return { id, at: { x: at[0], y: at[1] }, z: at[2], fxN: f[0], fyN: f[1], fzN: f[2], mxNm: m[0], myNm: m[1], mzNm: m[2], bone, kind, title };
}

function framesOf(state: EvaluatedState): Frames {
  return Object.fromEntries(Object.entries(state.bones).map(([id, b]) => [id, { o: tuple(b.frame.o), u: tuple(b.frame.u), v: tuple(b.frame.v), w: tuple(b.frame.w) }]));
}

function localOf(frames: Frames, bone: Slug, p: V3): { s: number; y: number; z: number } {
  const f = frames[bone]!;
  const d = sub(p, f.o);
  return { s: dot(d, f.u), y: dot(d, f.v), z: dot(d, f.w) };
}

function worldOf(frames: Frames, bone: Slug, s: number, y: number, z = 0): V3 {
  const f = frames[bone]!;
  return add(f.o, add(scaled(f.u, s), add(scaled(f.v, y), scaled(f.w, z))));
}

function symmetricEigen(a: number[][]): { values: number[]; vectors: number[][] } {
  const n = a.length;
  const m = a.map((row) => [...row]);
  const v: number[][] = m.map((_, i) => m.map((__, j) => (i === j ? 1 : 0)));
  for (let sweep = 0; sweep < 64; sweep++) {
    let off = 0;
    for (let i = 0; i < n; i++) for (let j = i + 1; j < n; j++) off += m[i]![j]! ** 2;
    if (off < 1e-300) break;
    for (let p = 0; p < n; p++)
      for (let q = p + 1; q < n; q++) {
        if (Math.abs(m[p]![q]!) < 1e-300) continue;
        const theta = (m[q]![q]! - m[p]![p]!) / (2 * m[p]![q]!);
        const t = Math.sign(theta || 1) / (Math.abs(theta) + Math.sqrt(theta * theta + 1));
        const c = 1 / Math.sqrt(t * t + 1);
        const s = t * c;
        for (let k = 0; k < n; k++) {
          const kp = m[k]![p]!;
          const kq = m[k]![q]!;
          m[k]![p] = c * kp - s * kq;
          m[k]![q] = s * kp + c * kq;
        }
        for (let k = 0; k < n; k++) {
          const pk = m[p]![k]!;
          const qk = m[q]![k]!;
          m[p]![k] = c * pk - s * qk;
          m[q]![k] = s * pk + c * qk;
        }
        for (let k = 0; k < n; k++) {
          const kp = v[k]![p]!;
          const kq = v[k]![q]!;
          v[k]![p] = c * kp - s * kq;
          v[k]![q] = s * kp + c * kq;
        }
      }
  }
  return { values: m.map((row, i) => row[i]!), vectors: v };
}

/** 🧮️ Minimum-norm solution `u` of the symmetric semi-definite system `K u = f` and the part of `f` the system cannot balance (its component in the null space of `K`). */
export function solveSemiDefinite(k: number[][], f: readonly number[]): { u: number[]; unbalanced: number } {
  const { values, vectors } = symmetricEigen(k);
  const top = Math.max(...values.map(Math.abs), 0);
  const u = f.map(() => 0);
  let lost = 0;
  values.forEach((lambda, i) => {
    const component = sumOf(f.map((x, r) => x * vectors[r]![i]!));
    if (lambda > NULL_SPACE * top && lambda > 0) for (let r = 0; r < u.length; r++) u[r]! += (component / lambda) * vectors[r]![i]!;
    else lost += component * component;
  });
  return { u, unbalanced: Math.sqrt(lost) };
}
//#endregion 🧰️Helpers

//#region ⚖️Forces
function massPieces(state: EvaluatedState, frames: Frames, inertial: boolean): { weights: Piece[]; inertials: Piece[] } {
  const g = state.asset.structure.gravityMS2;
  const weights: Piece[] = [];
  const inertials: Piece[] = [];
  for (const bone of state.asset.rig.bones) {
    const mass = bone.mass;
    if (!mass || !(mass.kg > 0)) continue;
    const acc = state.kinematics.segmentAccelerations[bone.id] ?? { x: 0, y: 0, z: 0 };
    const uniform = mass.distribution === "uniform" && bone.lengthM > 0;
    const n = uniform ? WEIGHT_SUBDIVISIONS : 1;
    for (let k = 0; k < n; k++) {
      const share = mass.kg / n;
      const span: readonly [number, number] | null = uniform ? [(bone.lengthM * k) / n, (bone.lengthM * (k + 1)) / n] : null;
      const at = uniform ? worldOf(frames, bone.id, (span![0] + span![1]) / 2, mass.comY) : worldOf(frames, bone.id, mass.comX, mass.comY);
      const suffix = uniform ? `-${k}` : "";
      weights.push({ span, force: worldForce(`${bone.id}-weight${suffix}`, bone.id, "weight", text(`Weight: ${bone.title.en}`, `Gewicht: ${bone.title.de}`), at, [0, -share * g, 0]) });
      if (inertial) inertials.push({ span, force: worldForce(`${bone.id}-inertial${suffix}`, bone.id, "inertial", text(`Inertia (d'Alembert): ${bone.title.en}`, `Trägheit (d'Alembert): ${bone.title.de}`), at, [-share * acc.x, -share * acc.y, -share * acc.z]) });
    }
  }
  return { weights, inertials };
}

function supportsOf(state: EvaluatedState, frames: Frames): Support[] {
  const ids = [...new Set([...state.contacts, ...state.asset.structure.supports])];
  const out: Support[] = [];
  for (const id of ids) {
    const entry = state.anchors[id];
    if (!entry) continue;
    const def = entry.anchor;
    if (def.kind !== "contact" && def.kind !== "support" && !state.asset.structure.supports.includes(id)) continue;
    const type: SupportType = def.support ?? (def.kind === "contact" ? "contact" : "pin");
    const rad = ((def.normalDeg ?? 90) * Math.PI) / 180;
    const tilt = ((def.tiltDeg ?? 0) * Math.PI) / 180;
    const n: V3 = [portable.cos(rad) * portable.cos(tilt), portable.sin(rad) * portable.cos(tilt), portable.sin(tilt)];
    const axis: V3 = Math.abs(n[2]) < 0.9 ? Z_AXIS : [1, 0, 0];
    const raw = cross(axis, n);
    const t = scaled(raw, 1 / norm(raw));
    out.push({ anchor: id, bone: def.bone, at: worldOf(frames, def.bone, def.x, def.y, def.z ?? 0), type, n, t, t2: cross(n, t), mu: def.frictionCoefficient ?? DEFAULT_FRICTION, def });
  }
  return out;
}
//#endregion ⚖️Forces

//#region 🧷️Reactions
function reactionSolution(supports: readonly Support[], external: readonly WorldForce[], com: V3, notes: string[][]): Map<Slug, Solved> {
  const result = new Map<Slug, Solved>();
  for (const s of supports) result.set(s.anchor, { f: ZERO, m: ZERO, normal: 0, ml: 0, mw: 0, active: false });
  if (supports.length === 0) return result;
  let c: V3 = ZERO;
  const scale = 1 + sumOf(external.map((f) => norm(forceOf(f)) + norm(coupleOf(f))));
  const dropped = new Set<Slug>();
  const sliding = new Map<Slug, readonly [number, number]>();
  const known = new Map<Slug, readonly [number, number]>();
  const capped = new Map<string, number>();
  const knownMoment = new Map<string, V3>();
  const forced = new Set<Slug>();
  const seen = new Set<string>();
  let lastKey = "";
  const length = (s: Support): number => (s.type === "contact" && s.def.footprintLengthM && s.def.footprintLengthM > 0 ? s.def.footprintLengthM : 0);
  const width = (s: Support): number => (s.type === "contact" && s.def.footprintWidthM && s.def.footprintWidthM > 0 ? s.def.footprintWidthM : 0);
  const tangential = (s: Support, [a, b]: readonly [number, number]): V3 => add(scaled(s.t, a), scaled(s.t2, b));
  const lever = (p: V3, d: V3): V3 => cross(sub(p, c), d);
  const axisOf = (s: Support, which: "l" | "w"): V3 => (which === "l" ? s.t2 : s.t);
  const live = (): Support[] => supports.filter((s) => !dropped.has(s.anchor));

  const rowsOf = (s: Support, moment: boolean): Row[] => {
    const force = (role: Row["role"], d: V3, k: number): Row => ({ anchor: s.anchor, role, d: [...d, ...lever(s.at, d)], k });
    const couple = (role: Row["role"], a: V3, k: number): Row => ({ anchor: s.anchor, role, d: [0, 0, 0, ...a], k });
    switch (s.type) {
      case "contact":
        return [
          force("n", s.n, SUPPORT_STIFFNESS.normal),
          ...(sliding.has(s.anchor) ? [] : [force("t1", s.t, SUPPORT_STIFFNESS.tangential), force("t2", s.t2, SUPPORT_STIFFNESS.tangential)]),
          ...(moment && length(s) > 0 && !capped.has(`${s.anchor}|l`) ? [couple("ml", s.t2, (SUPPORT_STIFFNESS.normal * length(s) ** 2) / 12)] : []),
          ...(moment && width(s) > 0 && !capped.has(`${s.anchor}|w`) ? [couple("mw", s.t, (SUPPORT_STIFFNESS.normal * width(s) ** 2) / 12)] : []),
        ];
      case "roller":
        return [force("n", s.n, SUPPORT_STIFFNESS.translation)];
      case "pin":
        return [force("x", [1, 0, 0], SUPPORT_STIFFNESS.translation), force("y", [0, 1, 0], SUPPORT_STIFFNESS.translation), force("z", Z_AXIS, SUPPORT_STIFFNESS.translation)];
      case "fixed":
        return [force("x", [1, 0, 0], SUPPORT_STIFFNESS.translation), force("y", [0, 1, 0], SUPPORT_STIFFNESS.translation), force("z", Z_AXIS, SUPPORT_STIFFNESS.translation), couple("rx", [1, 0, 0], SUPPORT_STIFFNESS.rotation), couple("ry", [0, 1, 0], SUPPORT_STIFFNESS.rotation), couple("rz", Z_AXIS, SUPPORT_STIFFNESS.rotation)];
    }
  };

  const solveOnce = (): { u: number[]; rows: Row[] } => {
    const active = live();
    c = active.length ? scaled(active.reduce<V3>((a, s) => add(a, s.at), ZERO), 1 / active.length) : ZERO;
    const total = external.reduce<{ f: V3; m: V3 }>((a, f) => ({ f: add(a.f, forceOf(f)), m: add(a.m, add(cross(sub(posOf(f), c), forceOf(f)), coupleOf(f))) }), { f: ZERO, m: ZERO });
    const assemble = (moment: boolean): { rows: Row[]; k: number[][]; f: number[] } => {
      const rows = active.flatMap((s) => rowsOf(s, moment));
      const k = Array.from({ length: 6 }, () => new Array<number>(6).fill(0));
      for (const r of rows) for (let i = 0; i < 6; i++) for (let j = 0; j < 6; j++) k[i]![j]! += r.k * r.d[i]! * r.d[j]!;
      let f = add(total.f, ZERO);
      let m = total.m;
      for (const s of active) {
        const mag = known.get(s.anchor);
        if (mag === undefined) continue;
        const force = tangential(s, mag);
        f = add(f, force);
        m = add(m, lever(s.at, force));
      }
      for (const [key, couple] of knownMoment) if (!dropped.has(key.split("|")[0]!)) m = add(m, couple);
      return { rows, k, f: [...f, ...m] };
    };
    const base = assemble(false);
    const first = solveSemiDefinite(base.k, base.f);
    if (first.unbalanced <= 1e-9 * scale || !active.some((s) => length(s) > 0 || width(s) > 0)) return { u: first.u, rows: base.rows };
    const extra = assemble(true);
    return { u: solveSemiDefinite(extra.k, extra.f).u, rows: extra.rows };
  };

  let lastForces = new Map<Slug, Solved & { tangent: readonly [number, number] }>();
  const finish = (): void => {
    for (const s of supports) {
      const o = lastForces.get(s.anchor)!;
      const mag = known.get(s.anchor);
      const moments = (["l", "w"] as const).reduce<V3>((a, which) => add(a, knownMoment.get(`${s.anchor}|${which}`) ?? ZERO), ZERO);
      result.set(s.anchor, { f: mag ? add(o.f, tangential(s, mag)) : o.f, m: add(o.m, moments), normal: o.normal, ml: o.ml + dot(knownMoment.get(`${s.anchor}|l`) ?? ZERO, s.t2), mw: o.mw + dot(knownMoment.get(`${s.anchor}|w`) ?? ZERO, s.t), active: !dropped.has(s.anchor) });
    }
    for (const a of dropped) notes.push([`Contact ${a} lifted off (it would have to pull) and carries no load.`, `Kontakt ${a} abgehoben (er müsste ziehen) und trägt keine Last.`]);
    for (const a of sliding.keys()) notes.push([`Contact ${a} slides: friction capped at μ·N.`, `Kontakt ${a} gleitet: Reibung auf μ·N begrenzt.`]);
    for (const key of capped.keys()) {
      const [a, which] = key.split("|");
      notes.push([`Contact ${a}: the centre of pressure is clamped at the edge of the footprint (${which === "l" ? "length" : "width"}, |M| = N·${which === "l" ? "l" : "b"}/2); the foot tips about it.`, `Kontakt ${a}: der Druckmittelpunkt ist am Rand der Aufstandsfläche (${which === "l" ? "Länge" : "Breite"}, |M| = N·${which === "l" ? "l" : "b"}/2) begrenzt; der Fuß kippt um diesen Rand.`]);
    }
  };

  for (let iteration = 0; iteration < MAX_ITERATIONS; iteration++) {
    const { u, rows } = solveOnce();
    const forces = new Map<Slug, Solved & { tangent: readonly [number, number] }>();
    for (const s of supports) forces.set(s.anchor, { f: ZERO, m: ZERO, normal: 0, ml: 0, mw: 0, active: true, tangent: [0, 0] });
    const lateral = new Map<Slug, [number, number]>();
    for (const r of rows) {
      const q = -r.k * sumOf(r.d.map((x, i) => x * u[i]!));
      const o = forces.get(r.anchor)!;
      const direction: V3 = [r.d[0]!, r.d[1]!, r.d[2]!];
      const axis: V3 = [r.d[3]!, r.d[4]!, r.d[5]!];
      if (r.role === "ml" || r.role === "mw" || r.role === "rx" || r.role === "ry" || r.role === "rz") {
        o.m = add(o.m, scaled(axis, q));
        if (r.role === "ml") o.ml = q;
        if (r.role === "mw") o.mw = q;
      } else {
        o.f = add(o.f, scaled(direction, q));
        if (r.role === "n") o.normal = q;
        if (r.role === "t1") lateral.set(r.anchor, [q, lateral.get(r.anchor)?.[1] ?? 0]);
        if (r.role === "t2") lateral.set(r.anchor, [lateral.get(r.anchor)?.[0] ?? 0, q]);
      }
    }
    for (const [anchor, tangent] of lateral) forces.get(anchor)!.tangent = tangent;
    let changed = false;
    let worst: Slug | null = null;
    let worstValue = -1e-9 * scale;
    for (const s of supports) {
      if (s.type !== "contact" || dropped.has(s.anchor) || forced.has(s.anchor)) continue;
      const o = forces.get(s.anchor)!;
      if (o.normal < worstValue) {
        worst = s.anchor;
        worstValue = o.normal;
      }
    }
    const active = live();
    const allTensile = worst !== null && active.every((s) => s.type === "contact" && !forced.has(s.anchor) && forces.get(s.anchor)!.normal < -1e-9 * scale);
    if (worst && allTensile) {
      const keep = active.reduce((a, b) => (portable.hypot(b.at[0] - com[0], b.at[2] - com[2]) < portable.hypot(a.at[0] - com[0], a.at[2] - com[2]) ? b : a));
      for (const s of active) if (s !== keep) dropped.add(s.anchor);
      forced.add(keep.anchor);
      notes.push([`Every contact would pull; contact ${keep.anchor}, closest to the centre-of-mass projection, is kept active.`, `Jeder Kontakt müsste ziehen; Kontakt ${keep.anchor}, am nächsten zur Schwerpunktprojektion, bleibt aktiv.`]);
      changed = true;
    } else if (worst) {
      dropped.add(worst);
      sliding.delete(worst);
      known.delete(worst);
      for (const which of ["l", "w"]) {
        capped.delete(`${worst}|${which}`);
        knownMoment.delete(`${worst}|${which}`);
      }
      changed = true;
    } else {
      for (const s of supports) {
        if (s.type !== "contact" || dropped.has(s.anchor)) continue;
        const o = forces.get(s.anchor)!;
        const cap = s.mu * Math.max(o.normal, 0);
        const magnitude = portable.hypot(o.tangent[0], o.tangent[1]);
        if (!sliding.has(s.anchor) && magnitude > cap + 1e-9 * scale) {
          const direction: readonly [number, number] = [o.tangent[0] / magnitude, o.tangent[1] / magnitude];
          sliding.set(s.anchor, direction);
          known.set(s.anchor, [direction[0] * cap, direction[1] * cap]);
          changed = true;
        } else if (sliding.has(s.anchor)) {
          const direction = sliding.get(s.anchor)!;
          const next: readonly [number, number] = [direction[0] * cap, direction[1] * cap];
          const previous = known.get(s.anchor) ?? [0, 0];
          if (portable.hypot(next[0] - previous[0], next[1] - previous[1]) > 1e-9 * scale) {
            known.set(s.anchor, next);
            changed = true;
          }
        }
      }
    }
    if (!worst) {
      for (const s of supports) {
        if (dropped.has(s.anchor)) continue;
        const o = forces.get(s.anchor)!;
        for (const which of ["l", "w"] as const) {
          const extent = which === "l" ? length(s) : width(s);
          if (!(extent > 0)) continue;
          const key = `${s.anchor}|${which}`;
          const value = which === "l" ? o.ml : o.mw;
          const cap = (extent / 2) * Math.max(o.normal, 0);
          const sign = capped.get(key);
          if (sign === undefined && Math.abs(value) > cap + 1e-9 * scale) {
            capped.set(key, Math.sign(value));
            knownMoment.set(key, scaled(axisOf(s, which), Math.sign(value) * cap));
            changed = true;
          } else if (sign !== undefined && norm(sub(knownMoment.get(key) ?? ZERO, scaled(axisOf(s, which), sign * cap))) > 1e-9 * scale) {
            knownMoment.set(key, scaled(axisOf(s, which), sign * cap));
            changed = true;
          }
        }
      }
    }
    lastForces = forces;
    if (!changed) {
      finish();
      return result;
    }
    const key = [...dropped].sort().join(",") + "|" + [...sliding.keys()].sort().join(",") + "|" + [...capped.keys()].sort().join(",");
    if (key !== lastKey) {
      if (seen.has(key) && iteration > 8) break;
      seen.add(key);
      lastKey = key;
    }
  }
  finish();
  notes.push(["Support iteration did not converge; the last iterate is reported.", "Auflageriteration nicht konvergiert; die letzte Iteration wird ausgewiesen."]);
  return result;
}
//#endregion 🧷️Reactions

//#region 📈️Members
type Cut = { f: V3; m: V3 };

function distalCut(frames: Frames, bone: Slug, own: Map<Slug, Piece[]>, attach: { s: number; sums: Sums }[], s: number, inclusive: boolean): Cut {
  const q = worldOf(frames, bone, s, 0);
  const u = frames[bone]!.u;
  let f: V3 = ZERO;
  let m: V3 = ZERO;
  const take = (p: number): boolean => (inclusive ? p >= s - POSITION_TOLERANCE : p > s + POSITION_TOLERANCE);
  for (const piece of own.get(bone) ?? []) {
    const force = piece.force;
    if (piece.span) {
      const [s0, s1] = piece.span;
      const from = Math.max(s, s0);
      if (from >= s1 - POSITION_TOLERANCE) continue;
      const share = (s1 - from) / (s1 - s0);
      const centre = (from + s1) / 2 - (s0 + s1) / 2;
      const at = add(posOf(force), scaled(u, centre));
      f = add(f, scaled(forceOf(force), share));
      m = add(m, scaled(cross(sub(at, q), forceOf(force)), share));
    } else if (take(localOf(frames, bone, posOf(force)).s)) {
      f = add(f, forceOf(force));
      m = add(m, add(cross(sub(posOf(force), q), forceOf(force)), coupleOf(force)));
    }
  }
  for (const a of attach) {
    if (!take(a.s)) continue;
    f = add(f, a.sums.f);
    m = add(m, sub(a.sums.m0, cross(q, a.sums.f)));
  }
  return { f, m };
}

function deflectionLine(state: EvaluatedState, bone: Bone, momentAt: (s: number) => number, restraints: readonly { s: number; fixed: boolean }[], hasParent: boolean): Point[] | null {
  const section = bone.section;
  const modulus = state.asset.materials.find((x) => x.id === section?.material)?.elasticModulusPa;
  if (!section || !modulus || !(section.secondMomentM4 > 0)) return null;
  const ei = modulus * section.secondMomentM4;
  const parentJoint = hasParent ? bone.joint.type : null;
  const points = restraints.map((r) => ({ ...r }));
  let clamp: number | null = null;
  let pins: number[] = [];
  const fixedAt = points.filter((p) => p.fixed);
  if (parentJoint === "fixed" && points.length === 0) clamp = 0;
  else if (parentJoint === "fixed") return null;
  else if (fixedAt.length === 1 && points.length === 1 && !hasParent) clamp = fixedAt[0]!.s;
  else if (parentJoint !== null) pins = [0, ...points.map((p) => p.s)];
  else pins = points.map((p) => p.s);
  if (clamp === null && (pins.length !== 2 || fixedAt.length > 0)) return null;
  const n = DEFLECTION_INTERVALS;
  const h = bone.lengthM / n;
  const theta = [0];
  const w = [0];
  let previous = momentAt(0) / ei;
  for (let i = 1; i <= n; i++) {
    const k = momentAt(i * h) / ei;
    theta.push(theta[i - 1]! + ((previous + k) / 2) * h);
    w.push(w[i - 1]! + ((theta[i - 1]! + theta[i]!) / 2) * h);
    previous = k;
  }
  const interpolate = (arr: number[], s: number): number => {
    const x = Math.min(Math.max(s / h, 0), n);
    const i = Math.min(Math.floor(x), n - 1);
    return arr[i]! + (arr[i + 1]! - arr[i]!) * (x - i);
  };
  let a: number;
  let b: number;
  if (clamp !== null) {
    b = -interpolate(theta, clamp);
    a = -interpolate(w, clamp) - b * clamp;
  } else {
    const [s0, s1] = [Math.min(...pins), Math.max(...pins)] as [number, number];
    if (s1 - s0 < POSITION_TOLERANCE) return null;
    b = -(interpolate(w, s1) - interpolate(w, s0)) / (s1 - s0);
    a = -interpolate(w, s0) - b * s0;
  }
  return w.map((value, i) => ({ x: i * h, y: value + a + b * i * h }));
}

function memberDiagram(state: EvaluatedState, frames: Frames, bone: Bone, own: Map<Slug, Piece[]>, attach: { s: number; sums: Sums }[], active: readonly Support[]): MemberDiagram {
  const L = bone.lengthM;
  const { u, v, w } = frames[bone.id]!;
  const special: number[] = [];
  const plain: number[] = [];
  for (const piece of own.get(bone.id) ?? []) {
    if (piece.span) plain.push(piece.span[0], piece.span[1]);
    else special.push(localOf(frames, bone.id, posOf(piece.force)).s);
  }
  for (const a of attach) special.push(a.s);
  for (let i = 0; i <= DIAGRAM_INTERVALS; i++) plain.push((L * i) / DIAGRAM_INTERVALS);
  const entries: { s: number; pair: boolean }[] = [];
  const addEntry = (s: number, pair: boolean): void => {
    if (s < -POSITION_TOLERANCE || s > L + POSITION_TOLERANCE) return;
    const clamped = Math.min(Math.max(s, 0), L);
    const hit = entries.find((x) => Math.abs(x.s - clamped) < POSITION_TOLERANCE);
    if (hit) hit.pair ||= pair;
    else entries.push({ s: clamped, pair });
  };
  special.forEach((s) => addEntry(s, true));
  plain.forEach((s) => addEntry(s, false));
  entries.sort((x, y) => x.s - y.s);
  const section = bone.section;
  const stations: DiagramStation[] = [];
  const push = (s: number, inclusive: boolean): void => {
    const cut = distalCut(frames, bone.id, own, attach, s, inclusive);
    const axialN = dot(cut.f, u);
    const shear = scaled(sub(cut.f, scaled(u, axialN)), -1);
    const momentNm = dot(cut.m, w);
    const momentLateralNm = dot(cut.m, v);
    const sigma = section ? axialN / section.areaM2 : null;
    const bending = section ? (portable.hypot(momentNm, momentLateralNm) * section.extremeFibreM) / section.secondMomentM4 : null;
    const at = worldOf(frames, bone.id, s, 0);
    stations.push({ s, at: { x: at[0], y: at[1] }, z: at[2], axialN, shearN: -dot(cut.f, v), shearZN: -dot(cut.f, w), torsionNm: dot(cut.m, u), momentNm, momentLateralNm, shearVector: vec(shear), momentVector: vec(cut.m), stressMinPa: sigma === null ? null : sigma - bending!, stressMaxPa: sigma === null ? null : sigma + bending! });
  };
  for (const entry of entries) {
    push(entry.s, true);
    if (entry.pair) push(entry.s, false);
  }
  const restraints: { s: number; fixed: boolean }[] = [];
  for (const s of active) {
    if (s.bone !== bone.id) continue;
    const local = localOf(frames, bone.id, s.at).s;
    if (s.type === "fixed") restraints.push({ s: local, fixed: true });
    else if (s.type === "pin") restraints.push({ s: local, fixed: false });
    else if (Math.abs(dot(s.n, v)) > 1e-3) restraints.push({ s: local, fixed: false });
  }
  const moment = (s: number): number => dot(distalCut(frames, bone.id, own, attach, s, false).m, w);
  return { bone: bone.id, stations, deflection: deflectionLine(state, bone, moment, restraints, bone.parent !== null) };
}
//#endregion 📈️Members

/** 📐️ The unit direction of the line through all points when they are two or more and collinear within a millimetre in plan (x, z), else `null`. */
function planLine(points: readonly (readonly [number, number])[]): readonly [number, number] | null {
  if (points.length < 2) return null;
  const first = points[0]!;
  const far = points.reduce((a, p) => (portable.hypot(p[0] - first[0], p[1] - first[1]) > portable.hypot(a[0] - first[0], a[1] - first[1]) ? p : a));
  const length = portable.hypot(far[0] - first[0], far[1] - first[1]);
  if (length < 1e-6) return null;
  const d: readonly [number, number] = [(far[0] - first[0]) / length, (far[1] - first[1]) / length];
  return points.every((p) => Math.abs((p[0] - first[0]) * d[1] - (p[1] - first[1]) * d[0]) < 1e-3) ? d : null;
}

//#region 🏗️Solver
/** 🏗️ Solves reactions, joint loads and member diagrams of one evaluated state in three dimensions.
 *
 * Statically determinate supports (one fixed, pin plus roller) reproduce exact statics; indeterminate ones follow the equal-stiffness
 * elastic-support assumption stated in the result notes.
 *
 * @see ../🔗️contracts/🟦️.ts — {@link StructuralResult}
 */
export function solveStructure(state: EvaluatedState): StructuralResult {
  const asset = state.asset;
  const model = asset.structure;
  const frames = framesOf(state);
  const inertial = model.dynamics === "inertial";
  const { weights, inertials } = massPieces(state, frames, inertial);
  const applied: WorldForce[] = [...state.loads];
  const nonReaction: WorldForce[] = [...weights, ...inertials].map((p) => p.force).concat(applied);
  const supports = supportsOf(state, frames);
  const collected: string[][] = [];
  const weightSumN = sumOf(weights.map((p) => -p.force.fyN));
  const comProjection: V3 = weightSumN > 0 ? [sumOf(weights.map((p) => -p.force.fyN * p.force.at.x)) / weightSumN, 0, sumOf(weights.map((p) => -p.force.fyN * p.force.z)) / weightSumN] : ZERO;
  const solution = reactionSolution(supports, nonReaction, comProjection, collected);
  if (![...solution.values()].some((r) => r.active)) collected.push(["Unsupported: no active support (flight phase / ballistic motion or no contact); reactions are zero and the residual is the unbalanced weight plus inertia.", "Ungestützt: kein aktives Auflager (Flugphase / ballistische Bewegung oder kein Kontakt); Reaktionen sind null und das Residuum ist Gewicht plus Trägheit ohne Gegenkraft."]);
  const reactions: Reaction[] = supports.map((s) => {
    const r = solution.get(s.anchor)!;
    const area = s.def.footprintLengthM && s.def.footprintWidthM ? s.def.footprintLengthM * s.def.footprintWidthM : null;
    const along = dot(r.f, s.n);
    const length = s.type === "contact" ? (s.def.footprintLengthM ?? 0) : 0;
    const breadth = s.type === "contact" ? (s.def.footprintWidthM ?? 0) : 0;
    const usable = r.active && r.normal > 1e-9;
    const offsetL = usable && length > 0 ? Math.max(-length / 2, Math.min(length / 2, -r.ml / r.normal)) : 0;
    const offsetW = usable && breadth > 0 ? Math.max(-breadth / 2, Math.min(breadth / 2, r.mw / r.normal)) : 0;
    const hasCop = usable && (length > 0 || breadth > 0);
    return {
      anchor: s.anchor,
      at: { x: s.at[0], y: s.at[1] },
      z: s.at[2],
      fxN: r.f[0],
      fyN: r.f[1],
      fzN: r.f[2],
      mxNm: r.m[0],
      myNm: r.m[1],
      mzNm: r.m[2],
      pressurePa: area === null ? null : r.active ? Math.max(along, 0) / area : 0,
      active: r.active,
      ...(hasCop ? { cop: vec(add(add(s.at, scaled(s.t, offsetL)), scaled(s.t2, offsetW))) } : {}),
    };
  });
  const reactionForces: WorldForce[] = reactions.map((r) => {
    const def = state.anchors[r.anchor]!.anchor;
    return worldForce(r.anchor, def.bone, "reaction", text(`Reaction: ${def.title.en}`, `Reaktion: ${def.title.de}`), [r.at.x, r.at.y, r.z], [r.fxN, r.fyN, r.fzN], [r.mxNm, r.myNm, r.mzNm]);
  });
  const pieces: Piece[] = [...weights, ...inertials, ...applied.map((force) => ({ force, span: null })), ...reactionForces.map((force) => ({ force, span: null }))];
  const own = new Map<Slug, Piece[]>();
  for (const p of pieces) own.set(p.force.bone, [...(own.get(p.force.bone) ?? []), p]);

  const children = new Map<Slug, Bone[]>();
  for (const b of asset.rig.bones) if (b.parent) children.set(b.parent, [...(children.get(b.parent) ?? []), b]);
  const sums = new Map<Slug, Sums>();
  const subtree = (id: Slug): Sums => {
    const cached = sums.get(id);
    if (cached) return cached;
    let f: V3 = ZERO;
    let m0: V3 = ZERO;
    for (const p of own.get(id) ?? []) {
      f = add(f, forceOf(p.force));
      m0 = add(m0, moment0(p.force));
    }
    for (const c of children.get(id) ?? []) {
      const s = subtree(c.id);
      f = add(f, s.f);
      m0 = add(m0, s.m0);
    }
    const total: Sums = { f, m0 };
    sums.set(id, total);
    return total;
  };
  for (const b of asset.rig.bones) subtree(b.id);

  const joints: Record<Slug, JointLoad> = {};
  const members: Record<Slug, MemberDiagram> = {};
  const activeSupports = supports.filter((s) => solution.get(s.anchor)?.active);
  for (const b of asset.rig.bones) {
    const frame = frames[b.id];
    if (!frame) continue;
    const total = subtree(b.id);
    if (b.parent !== null) {
      const m = sub(total.m0, cross(frame.o, total.f));
      joints[b.id] = {
        bone: b.id,
        at: { x: frame.o[0], y: frame.o[1] },
        z: frame.o[2],
        fxN: -total.f[0],
        fyN: -total.f[1],
        fzN: -total.f[2],
        axialN: dot(total.f, frame.u),
        shearN: -dot(total.f, frame.v),
        shearZN: -dot(total.f, frame.w),
        mxNm: m[0],
        myNm: m[1],
        mzNm: m[2],
        torsionNm: dot(m, frame.u),
        momentNm: dot(m, frame.w),
        momentLateralNm: dot(m, frame.v),
      };
    }
    if (b.lengthM > 0) {
      const attach = (children.get(b.id) ?? []).map((c) => ({ s: localOf(frames, b.id, frames[c.id]!.o).s, sums: subtree(c.id) }));
      members[b.id] = memberDiagram(state, frames, b, own, attach, activeSupports);
    }
  }

  const everything = pieces.map((p) => p.force);
  const residualMoment = everything.reduce<V3>((a, f) => add(a, moment0(f)), ZERO);
  const residual = { fxN: sumOf(everything.map((f) => f.fxN)), fyN: sumOf(everything.map((f) => f.fyN)), fzN: sumOf(everything.map((f) => f.fzN)), mxNm: residualMoment[0], myNm: residualMoment[1], mzNm: residualMoment[2] };
  const massed = weights.map((p) => p.force);
  const totalMassKg = sumOf(asset.rig.bones.map((b) => b.mass?.kg ?? 0));
  const weightSum = sumOf(massed.map((f) => -f.fyN));
  const centreOfMass: Vec3 = weightSum > 0 ? { x: sumOf(massed.map((f) => -f.fyN * f.at.x)) / weightSum, y: sumOf(massed.map((f) => -f.fyN * f.at.y)) / weightSum, z: sumOf(massed.map((f) => -f.fyN * f.z)) / weightSum } : { x: 0, y: 0, z: 0 };

  const notes: Text[] = [
    text("Three-dimensional statics: forces, supports, reactions, joint loads and member cuts are solved in world metres and newtons (x ahead, y up, z the asset's left) from the bone frames of the evaluated state; side elevation, plan (top view) and front elevation only project these vectors, so every magnitude is the same in all views and a component along the line of sight is stated explicitly.", "Räumliche Statik: Kräfte, Auflager, Reaktionen, Gelenklasten und Stabschnitte werden in Weltmetern und Newton (x voraus, y oben, z links des Objekts) aus den Stabsystemen des ausgewerteten Zustands gelöst; Seitenansicht, Draufsicht und Vorderansicht projizieren diese Vektoren nur, jeder Betrag ist daher in allen Ansichten gleich, und eine Komponente in Blickrichtung wird ausdrücklich genannt."),
    text("Contacts with a footprint length l and width b transmit a moment |M| ≤ N·l/2 about the lateral axis and |M| ≤ N·b/2 about the tangent axis in the side elevation plane (the centre of pressure moves within the sole, tyre patch or hoof); rotational stiffness k·l²/12 and k·b²/12, engaged only when the supports cannot balance the moment otherwise, capped at the limit like friction; `cop` is the resulting centre of pressure. A contact transmits no torque about its normal.", "Kontakte mit Aufstandslänge l und -breite b übertragen ein Moment |M| ≤ N·l/2 um die Querachse und |M| ≤ N·b/2 um die Tangentenachse in der Ebene der Seitenansicht (der Druckmittelpunkt wandert in der Sohle, Reifenaufstandsfläche oder dem Huf); Drehsteifigkeit k·l²/12 und k·b²/12, nur aktiv, wenn die Auflager das Moment sonst nicht aufnehmen können, wie die Reibung auf die Grenze begrenzt; `cop` ist der resultierende Druckmittelpunkt. Ein Kontakt überträgt kein Moment um seine Normale."),
    text("Model: instantaneously rigid body on elastic supports of equal normalised stiffness (contact: normal 1, friction 0.5 in both tangent directions; pin, roller, fixed: 1); small-displacement 6x6 equilibrium by pseudo-inverse; contacts that would pull are dropped; friction is capped at μ·N in the tangent plane.", "Modell: momentan starrer Körper auf elastischen Auflagern gleicher normierter Steifigkeit (Kontakt: normal 1, Reibung 0,5 in beiden Tangentenrichtungen; Gelenk, Rollenlager, Einspannung: 1); 6x6-Gleichgewicht kleiner Verschiebungen per Pseudoinverse; ziehende Kontakte entfallen; Reibung in der Tangentialebene auf μ·N begrenzt."),
    inertial
      ? text("Dynamics: inertial (d'Alembert from evaluated kinematics): forces -m·a at the segment centres; rotational inertia of segments is neglected. The residual includes the d'Alembert forces and is therefore also zero.", "Dynamik: inertial (d'Alembert aus den ausgewerteten Kinematiken): Kräfte -m·a in den Segmentschwerpunkten; Drehträgheit der Segmente vernachlässigt. Das Residuum enthält die d'Alembert-Kräfte und ist daher ebenfalls null.")
      : text("Dynamics: quasi-static (weights and applied loads only); the residual is zero when the supports can balance the load.", "Dynamik: quasi-statisch (nur Gewichte und aufgebrachte Lasten); das Residuum ist null, wenn die Auflager die Last aufnehmen können."),
    text(`Gravity ${model.gravityMS2} m/s²; uniform masses are discretised into ${WEIGHT_SUBDIVISIONS} sub-forces; contact friction defaults to μ = ${DEFAULT_FRICTION} where an anchor authors none.`, `Erdbeschleunigung ${model.gravityMS2} m/s²; gleichmäßige Massen in ${WEIGHT_SUBDIVISIONS} Teilkräfte zerlegt; Kontaktreibung μ = ${DEFAULT_FRICTION}, wo ein Anker keine angibt.`),
    text("Sign conventions: N > 0 tension; shear V = -Σ distal F·v in the rig plane and Vz = -Σ distal F·w laterally; bending M > 0 sagging about the bone's lateral axis w, lateral bending about the bone normal v, torsion about the bone axis; joint force is what the parent transmits to the child, the joint moment vector is the moment of the child subtree's external loads about the joint; a positive moment about axis k turns i towards j for the cyclic order (i, j, k); the components are those of M = r × F and the frame is left-handed, so a moment arrow follows the left-hand rule (positive mz is counter-clockwise in the side elevation, positive mx clockwise in the front elevation, positive my clockwise in the plan).", "Vorzeichen: N > 0 Zug; Querkraft V = -Σ distale F·v in der Skelettebene und Vz = -Σ distale F·w seitlich; Biegemoment M > 0 Feldmoment um die Stabquerachse w, seitliche Biegung um die Stabnormale v, Torsion um die Stabachse; Gelenkkraft ist die vom Elternteil auf das Kind übertragene Kraft, der Gelenkmomentvektor ist das Moment der äußeren Lasten des Kind-Teilbaums um das Gelenk; ein positives Moment um die Achse k dreht i nach j in der zyklischen Reihenfolge (i, j, k); die Komponenten sind die von M = r × F und das Koordinatensystem ist linkshändig, ein Momentenpfeil folgt daher der Linke-Hand-Regel (positives mz dreht in der Seitenansicht gegen den Uhrzeigersinn, positives mx in der Vorderansicht und positives my in der Draufsicht im Uhrzeigersinn)."),
    text("Deflection: Euler-Bernoulli double integration of M/EI (bending about the lateral axis w, in the rig plane) for members with section and elastic modulus that are clamped, or supported at two points; shear, axial and lateral deformation are neglected; the section is treated as isotropic in bending, the extreme-fibre stress uses the resultant of both bending components.", "Durchbiegung: Euler-Bernoulli-Doppelintegration von M/EI (Biegung um die Querachse w, in der Skelettebene) für Stäbe mit Querschnitt und Elastizitätsmodul, die eingespannt oder an zwei Punkten gelagert sind; Schub-, Längs- und Seitenverformung vernachlässigt; der Querschnitt gilt als biegeisotrop, die Randfaserspannung nutzt die Resultierende beider Biegekomponenten."),
    ...(asset.rig.kind === "anatomical" ? [text("Anatomical rig: joint values are simplified rigid-segment resultants, not clinically validated bone, muscle or joint forces.", "Anatomisches Skelett: Gelenkwerte sind vereinfachte Starrkörper-Resultierende, keine klinisch validierten Knochen-, Muskel- oder Gelenkkräfte.")] : []),
    ...model.assumptions,
    ...collected.map(([en, de]) => text(en!, de!)),
  ];
  const scale = 1 + sumOf(everything.map((f) => norm(forceOf(f))));
  const line = planLine(activeSupports.map((s) => [s.at[0], s.at[2]] as const));
  const tipping = line !== null && Math.abs(residual.mxNm * line[0] + residual.mzNm * line[1]) > 1e-6 * scale;
  if (tipping) notes.push(text("Collinear supports: every active support lies on one line in plan, so a moment about that line cannot be carried and the body tips about it (castors, wheels or feet on one axis need lateral positions `z` to stand).", "Kollineare Auflager: alle aktiven Auflager liegen im Grundriss auf einer Geraden, ein Moment um diese Gerade kann nicht aufgenommen werden und der Körper kippt um sie (Rollen, Räder oder Füße auf einer Achse brauchen seitliche Positionen `z`, um zu stehen)."));
  if (portable.hypot(residual.fxN, residual.fyN, residual.fzN) > 1e-6 * scale || portable.hypot(residual.mxNm, residual.myNm, residual.mzNm) > 1e-6 * scale) notes.push(text("Unbalanced: the active supports cannot equilibrate the loads (tipping, sliding or no support); the residual is reported unchanged.", "Nicht im Gleichgewicht: die aktiven Auflager können die Lasten nicht aufnehmen (Kippen, Gleiten oder kein Auflager); das Residuum wird unverändert ausgewiesen."));

  return { gravityMS2: model.gravityMS2, dynamics: model.dynamics, centreOfMass, totalMassKg, segmentWeights: massed, inertialForces: inertials.map((p) => p.force), applied, reactions, joints, members, residual, notes };
}
//#endregion 🏗️Solver

//#region 🎨️Depiction helpers
const QUANTITY_UNITS: Readonly<Record<StructuralQuantity, string>> = { axial: "N", shear: "N", stress: "Pa", "contact-pressure": "Pa", moment: "N·m" };
const QUANTITY_TITLES: Readonly<Record<StructuralQuantity, Text>> = {
  axial: text("Axial force", "Normalkraft"),
  shear: text("Shear force", "Querkraft"),
  stress: text("Extreme-fibre stress", "Randfaserspannung"),
  "contact-pressure": text("Contact pressure", "Kontaktpressung"),
  moment: text("Bending moment", "Biegemoment"),
};
const QUANTITY_CONVENTIONS: Readonly<Record<StructuralQuantity, Text>> = {
  axial: text("Tension positive (cool), compression negative (warm).", "Zug positiv (kühl), Druck negativ (warm)."),
  shear: text("V = -Σ distal transverse force; the larger of the rig-plane component (along v) and the lateral component (along w) is shown, its sign shows direction, not tension or compression.", "V = -Σ distale Querkraft; gezeigt wird die größere der Komponente in der Skelettebene (entlang v) und der seitlichen Komponente (entlang w), das Vorzeichen zeigt die Richtung, nicht Zug oder Druck."),
  stress: text("Tension positive (cool), compression negative (warm); σ = N/A ∓ |M|·c/I with |M| the resultant of both bending components.", "Zug positiv (kühl), Druck negativ (warm); σ = N/A ∓ |M|·c/I mit |M| als Resultierende beider Biegekomponenten."),
  "contact-pressure": text("Compression only: normal reaction divided by the footprint area.", "Nur Druck: Normalreaktion geteilt durch die Aufstandsfläche."),
  moment: text("Sagging positive (cool), hogging negative (warm); moment of the distal loads about the cut, the larger of the bending about the lateral axis w and about the bone normal v.", "Feldmoment positiv (kühl), Stützmoment negativ (warm); Moment der distalen Lasten um den Schnitt, das größere der Biegemomente um die Querachse w und um die Stabnormale v."),
};

/** 🎯️ The signed one of two components with the larger magnitude (the first wins a tie): the single value a legend colours a member with when it carries two. */
export const dominantOf = (a: number, b: number): number => (Math.abs(b) > Math.abs(a) ? b : a);

/** 🎯️ The shear a member is coloured by: the larger of the rig-plane and the lateral component. */
export const shearOf = (s: Pick<DiagramStation, "shearN" | "shearZN">): number => dominantOf(s.shearN, s.shearZN);

/** 🎯️ The bending moment a member is coloured by: the larger of the bending about the lateral axis and about the bone normal. */
export const bendingOf = (s: Pick<DiagramStation, "momentNm" | "momentLateralNm">): number => dominantOf(s.momentNm, s.momentLateralNm);

function quantityValues(quantity: StructuralQuantity, result: StructuralResult, include: (bone: Slug) => boolean): number[] {
  if (quantity === "contact-pressure") return result.reactions.flatMap((r) => (r.pressurePa === null ? [] : [r.pressurePa]));
  const stations = Object.entries(result.members).flatMap(([bone, m]) => (include(bone) ? m.stations : []));
  switch (quantity) {
    case "axial":
      return stations.map((s) => s.axialN);
    case "shear":
      return stations.map(shearOf);
    case "moment":
      return stations.map(bendingOf);
    case "stress":
      return stations.flatMap((s) => (s.stressMinPa === null || s.stressMaxPa === null ? [] : [s.stressMinPa, s.stressMaxPa]));
  }
}

/** 🎨️ The legend of one structural quantity: unit, sign convention, range and colour stops (compression warm, tension cool, zero grey).
 *
 * `include` selects the members the range is taken from (the views pass the members they colour, so the drawn legend and this one are the same function over the same member set); a `fixed` range wins for every quantity including the contact pressure.
 *
 * @see ../🔗️contracts/🟦️.ts — {@link Legend}
 */
export function structuralLegend(quantity: StructuralQuantity, result: StructuralResult, range: DisplayRange, include: (bone: Slug) => boolean = () => true): Legend {
  const values = quantityValues(quantity, result, include);
  const auto = values.length ? { min: Math.min(0, ...values), max: Math.max(0, ...values) } : { min: 0, max: 0 };
  const { min, max } = range.mode === "fixed" ? range : auto;
  const span = max - min;
  const c = STRUCTURAL_COLORS;
  const norm01 = (v: number): number => (span > 0 ? (v - min) / span : 0);
  const eps = 1e-6;
  const stops: { at: number; color: string }[] = [];
  if (quantity === "contact-pressure") stops.push({ at: norm01(0), color: c.zero }, ...(span > 0 ? [{ at: norm01(0) + eps, color: c.compressionWeak }, { at: 1, color: c.compressionStrong }] : []));
  else {
    const z = norm01(0);
    if (min < 0) stops.push({ at: 0, color: c.compressionStrong }, { at: Math.max(0, z - eps), color: c.compressionWeak });
    stops.push({ at: z, color: c.zero });
    if (max > 0) stops.push({ at: Math.min(1, z + eps), color: c.tensionWeak }, { at: 1, color: c.tensionStrong });
  }
  const unit = QUANTITY_UNITS[quantity];
  return {
    title: QUANTITY_TITLES[quantity],
    quantity,
    unit,
    convention: QUANTITY_CONVENTIONS[quantity],
    min,
    max,
    stops,
    interpretation: text(`${QUANTITY_TITLES[quantity].en} from ${min.toPrecision(4)} to ${max.toPrecision(4)} ${unit}; zero is grey.`, `${QUANTITY_TITLES[quantity].de} von ${min.toPrecision(4)} bis ${max.toPrecision(4)} ${unit}; null ist grau.`),
    range: range.mode,
  };
}

/** 🏹️ Every force of the free-body diagram with its application point: one resultant weight and inertia arrow per segment, each applied load and each reaction.
 *
 * @see ../🔗️contracts/🟦️.ts — {@link StructuralResult}
 */
export function freeBody(result: StructuralResult): FreeBodyForce[] {
  const merged = (forces: readonly WorldForce[], kind: "weight" | "inertial"): FreeBodyForce[] => {
    const byBone = new Map<Slug, WorldForce[]>();
    for (const f of forces) byBone.set(f.bone, [...(byBone.get(f.bone) ?? []), f]);
    return [...byBone].map(([bone, group]) => {
      const total = group.reduce<V3>((a, f) => add(a, forceOf(f)), ZERO);
      const at = norm(total) > 0 ? centroidOf(group, total) : posOf(group[0]!);
      return { id: `${bone}-${kind}`, kind, bone, at: vec(at), fxN: total[0], fyN: total[1], fzN: total[2], mxNm: 0, myNm: 0, mzNm: 0, title: group[0]!.title };
    });
  };
  const direct = (f: WorldForce, kind: FreeBodyForce["kind"], title: Text, bone: Slug | null): FreeBodyForce => ({ id: f.id, kind, bone, at: vec(posOf(f)), fxN: f.fxN, fyN: f.fyN, fzN: f.fzN, mxNm: f.mxNm, myNm: f.myNm, mzNm: f.mzNm, title });
  return [
    ...merged(result.segmentWeights, "weight"),
    ...merged(result.inertialForces, "inertial"),
    ...result.applied.map((f) => direct(f, "applied", f.title, f.bone)),
    ...result.reactions.filter((r) => r.active).map((r): FreeBodyForce => ({ id: r.anchor, kind: "reaction", bone: null, at: { x: r.at.x, y: r.at.y, z: r.z }, fxN: r.fxN, fyN: r.fyN, fzN: r.fzN, mxNm: r.mxNm, myNm: r.myNm, mzNm: r.mzNm, title: text(`Reaction at ${r.anchor}`, `Reaktion bei ${r.anchor}`) })),
  ];
}

function centroidOf(group: readonly WorldForce[], total: V3): V3 {
  const mean = scaled(group.reduce<V3>((a, f) => add(a, posOf(f)), ZERO), 1 / group.length);
  const m0 = group.reduce<V3>((a, f) => add(a, moment0(f)), ZERO);
  const deficit = sub(m0, cross(mean, total));
  return add(mean, scaled(cross(total, deficit), 1 / dot(total, total)));
}
//#endregion 🎨️Depiction helpers
