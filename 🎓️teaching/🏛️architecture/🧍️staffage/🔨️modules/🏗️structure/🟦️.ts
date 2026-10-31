/** 🏗️ Structural solver of the staffage archive: one {@link EvaluatedState} in, one {@link StructuralResult} out.
 *
 * Model: the evaluated pose is an instantaneously rigid body loaded by segment weights (and, when `structure.dynamics` is `inertial`, by
 * d'Alembert forces `-m·a` from the evaluated kinematics). Supports are elastic springs of equal normalised stiffness; the 3x3 rigid-body
 * equilibrium is solved for small displacements, unilateral contacts that would pull are dropped, and friction is capped at `μ·N`. Joint
 * loads and member diagrams are exact free-body cuts of the same external force set.
 *
 * Sign conventions: world x right, y up, CCW positive. Axial N > 0 tension; shear V = -(Σ distal Fy in the bone frame); bending M > 0
 * sagging (tension at the local -y face), M = Σ moment of the distal externals about the cut. Deflection points are `(s, w)` in the bone
 * frame, w along local +y.
 *
 * @see ../🔗️contracts/🟦️.ts — the runtime contracts
 * @see ../../🧬️schema/🟦️.ts — the authored masses, sections, anchors and structural model
 * @see ../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/structure-model.md — equations, tolerances, oracle results
 */
import type { Anchor, Bone, Slug, SupportType, Text } from "../../🧬️schema/🟦️.ts";
import type { DiagramStation, DisplayRange, EvaluatedState, ForceQuantity, JointLoad, Legend, MemberDiagram, Point, Reaction, StructuralResult, WorldForce } from "../🔗️contracts/🟦️.ts";

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
//#endregion 🔢️Constants

//#region 🧱️Types
/** 🧱️ A quantity a structural legend can describe. */
export type StructuralQuantity = ForceQuantity | "moment";

/** 🏹️ One arrow of the free-body diagram. */
export type FreeBodyForce = { readonly id: string; readonly kind: "weight" | "inertial" | "applied" | "reaction"; readonly bone: Slug | null; readonly at: Point; readonly fxN: number; readonly fyN: number; readonly momentNm: number; readonly title: Text };

type Piece = { readonly force: WorldForce; readonly span: readonly [number, number] | null };
type Sums = { fx: number; fy: number; m0: number };
type Row = { readonly anchor: string; readonly role: "n" | "t" | "x" | "y" | "m"; readonly d: readonly [number, number, number]; readonly k: number };
type Support = { readonly anchor: Slug; readonly bone: Slug; readonly at: Point; readonly type: SupportType; readonly n: Point; readonly mu: number; readonly def: Anchor };
//#endregion 🧱️Types

//#region 🧰️Helpers
const text = (en: string, de: string): Text => ({ en, de });
const sumOf = (xs: readonly number[]): number => xs.reduce((a, b) => a + b, 0);
const moment0 = (f: { at: Point; fxN: number; fyN: number; momentNm: number }): number => f.at.x * f.fyN - f.at.y * f.fxN + f.momentNm;

function solve3(k: number[][], f: readonly number[], strict = false): number[] | null {
  const scale = Math.max(...k.map((row, i) => Math.abs(row[i]!)), 1);
  const attempt = (reg: number): number[] | null => {
    const a = k.map((row, i) => [...row, f[i]!]);
    for (let i = 0; i < 3; i++) a[i]![i]! += reg;
    for (let c = 0; c < 3; c++) {
      let p = c;
      for (let r = c + 1; r < 3; r++) if (Math.abs(a[r]![c]!) > Math.abs(a[p]![c]!)) p = r;
      [a[c], a[p]] = [a[p]!, a[c]!];
      if (Math.abs(a[c]![c]!) < 1e-12 * scale) return null;
      for (let r = c + 1; r < 3; r++) {
        const q = a[r]![c]! / a[c]![c]!;
        for (let j = c; j < 4; j++) a[r]![j]! -= q * a[c]![j]!;
      }
    }
    const u = [0, 0, 0];
    for (let r = 2; r >= 0; r--) u[r] = (a[r]![3]! - sumOf([0, 1, 2].filter((j) => j > r).map((j) => a[r]![j]! * u[j]!))) / a[r]![r]!;
    return u;
  };
  return strict ? attempt(0) : (attempt(0) ?? attempt(scale * 1e-11) ?? [0, 0, 0]);
}

function localOf(state: EvaluatedState, bone: Slug, p: Point): { s: number; y: number } {
  const e = state.bones[bone]!;
  const dx = p.x - e.origin.x;
  const dy = p.y - e.origin.y;
  const c = Math.cos(e.angleRad);
  const s = Math.sin(e.angleRad);
  return { s: dx * c + dy * s, y: -dx * s + dy * c };
}

function worldOf(state: EvaluatedState, bone: Slug, s: number, y: number): Point {
  const [a, b, c, d, e, f] = state.bones[bone]!.world;
  return { x: a * s + c * y + e, y: b * s + d * y + f };
}
//#endregion 🧰️Helpers

//#region ⚖️Forces
function massPieces(state: EvaluatedState, inertial: boolean): { weights: Piece[]; inertials: Piece[] } {
  const g = state.asset.structure.gravityMS2;
  const weights: Piece[] = [];
  const inertials: Piece[] = [];
  for (const bone of state.asset.rig.bones) {
    const mass = bone.mass;
    if (!mass || !(mass.kg > 0)) continue;
    const acc = state.kinematics.segmentAccelerations[bone.id] ?? { x: 0, y: 0 };
    const uniform = mass.distribution === "uniform" && bone.lengthM > 0;
    const n = uniform ? WEIGHT_SUBDIVISIONS : 1;
    for (let k = 0; k < n; k++) {
      const share = mass.kg / n;
      const span: readonly [number, number] | null = uniform ? [(bone.lengthM * k) / n, (bone.lengthM * (k + 1)) / n] : null;
      const at = uniform ? worldOf(state, bone.id, (span![0] + span![1]) / 2, mass.comY) : worldOf(state, bone.id, mass.comX, mass.comY);
      const suffix = uniform ? `-${k}` : "";
      const common = { at, momentNm: 0, bone: bone.id };
      weights.push({ span, force: { ...common, id: `${bone.id}-weight${suffix}`, fxN: 0, fyN: -share * g, kind: "weight", title: text(`Weight: ${bone.title.en}`, `Gewicht: ${bone.title.de}`) } });
      if (inertial) inertials.push({ span, force: { ...common, id: `${bone.id}-inertial${suffix}`, fxN: -share * acc.x, fyN: -share * acc.y, kind: "inertial", title: text(`Inertia (d'Alembert): ${bone.title.en}`, `Trägheit (d'Alembert): ${bone.title.de}`) } });
    }
  }
  return { weights, inertials };
}

function supportsOf(state: EvaluatedState): Support[] {
  const ids = [...new Set([...state.contacts, ...state.asset.structure.supports])];
  const out: Support[] = [];
  for (const id of ids) {
    const entry = state.anchors[id];
    if (!entry) continue;
    const def = entry.anchor;
    if (def.kind !== "contact" && def.kind !== "support" && !state.asset.structure.supports.includes(id)) continue;
    const type: SupportType = def.support ?? (def.kind === "contact" ? "contact" : "pin");
    const rad = ((def.normalDeg ?? 90) * Math.PI) / 180;
    out.push({ anchor: id, bone: def.bone, at: entry.at, type, n: { x: Math.cos(rad), y: Math.sin(rad) }, mu: def.frictionCoefficient ?? DEFAULT_FRICTION, def });
  }
  return out;
}
//#endregion ⚖️Forces

//#region 🧷️Reactions
function reactionSolution(supports: readonly Support[], external: readonly WorldForce[], comX: number, notes: string[][]): Map<Slug, { fx: number; fy: number; m: number; normal: number; tangent: number; active: boolean }> {
  const result = new Map<Slug, { fx: number; fy: number; m: number; normal: number; tangent: number; active: boolean }>();
  for (const s of supports) result.set(s.anchor, { fx: 0, fy: 0, m: 0, normal: 0, tangent: 0, active: false });
  if (supports.length === 0) return result;
  let c: Point = { x: 0, y: 0 };
  const lever = (p: Point, vx: number, vy: number): number => (p.x - c.x) * vy - (p.y - c.y) * vx;
  const scale = 1 + sumOf(external.map((f) => Math.hypot(f.fxN, f.fyN) + Math.abs(f.momentNm)));
  const dropped = new Set<Slug>();
  const sliding = new Map<Slug, number>();
  const known = new Map<Slug, number>();
  const copCapped = new Map<Slug, number>();
  const knownMoment = new Map<Slug, number>();
  const footprint = (s: Support): number => (s.type === "contact" && s.def.footprintLengthM && s.def.footprintLengthM > 0 ? s.def.footprintLengthM : 0);
  const seen = new Set<string>();
  const forced = new Set<Slug>();

  const rowsOf = (s: Support, moment: boolean): Row[] => {
    const { n, at } = s;
    const t = { x: -n.y, y: n.x };
    switch (s.type) {
      case "contact":
        return [{ anchor: s.anchor, role: "n", d: [n.x, n.y, lever(at, n.x, n.y)], k: SUPPORT_STIFFNESS.normal }, ...(sliding.has(s.anchor) ? [] : [{ anchor: s.anchor, role: "t" as const, d: [t.x, t.y, lever(at, t.x, t.y)] as const, k: SUPPORT_STIFFNESS.tangential }]), ...(moment && footprint(s) > 0 && !copCapped.has(s.anchor) ? [{ anchor: s.anchor, role: "m" as const, d: [0, 0, 1] as const, k: (SUPPORT_STIFFNESS.normal * footprint(s) ** 2) / 12 }] : [])];
      case "roller":
        return [{ anchor: s.anchor, role: "n", d: [n.x, n.y, lever(at, n.x, n.y)], k: SUPPORT_STIFFNESS.translation }];
      case "pin":
        return [{ anchor: s.anchor, role: "x", d: [1, 0, lever(at, 1, 0)], k: SUPPORT_STIFFNESS.translation }, { anchor: s.anchor, role: "y", d: [0, 1, lever(at, 0, 1)], k: SUPPORT_STIFFNESS.translation }];
      case "fixed":
        return [{ anchor: s.anchor, role: "x", d: [1, 0, lever(at, 1, 0)], k: SUPPORT_STIFFNESS.translation }, { anchor: s.anchor, role: "y", d: [0, 1, lever(at, 0, 1)], k: SUPPORT_STIFFNESS.translation }, { anchor: s.anchor, role: "m", d: [0, 0, 1], k: SUPPORT_STIFFNESS.rotation }];
    }
  };

  const solveOnce = (): { u: number[]; rows: Row[] } => {
    const live = supports.filter((s) => !dropped.has(s.anchor));
    c = live.length ? { x: sumOf(live.map((s) => s.at.x)) / live.length, y: sumOf(live.map((s) => s.at.y)) / live.length } : { x: 0, y: 0 };
    const load = [sumOf(external.map((f) => f.fxN)), sumOf(external.map((f) => f.fyN)), sumOf(external.map((f) => (f.at.x - c.x) * f.fyN - (f.at.y - c.y) * f.fxN + f.momentNm))];
    const assemble = (moment: boolean): { rows: Row[]; k: number[][]; f: number[] } => {
      const rows = live.flatMap((s) => rowsOf(s, moment));
      const k = [0, 1, 2].map(() => [0, 0, 0]);
      for (const r of rows) for (let i = 0; i < 3; i++) for (let j = 0; j < 3; j++) k[i]![j]! += r.k * r.d[i]! * r.d[j]!;
      const f = [...load];
      for (const s of live) {
        const mag = known.get(s.anchor);
        if (mag === undefined) continue;
        const fx = mag * -s.n.y;
        const fy = mag * s.n.x;
        f[0]! += fx;
        f[1]! += fy;
        f[2]! += lever(s.at, fx, fy);
      }
      return { rows, k, f };
    };
    const base = assemble(false);
    const extra = live.some((s) => footprint(s) > 0) ? assemble(true) : base;
    const direct = solve3(base.k, base.f.map((v, i) => v + (i === 2 ? sumOf([...knownMoment].filter(([a]) => !dropped.has(a)).map(([, m]) => m)) : 0)), true);
    if (direct) return { u: direct, rows: base.rows };
    const chosen = extra;
    chosen.f[2]! += sumOf([...knownMoment].filter(([a]) => !dropped.has(a)).map(([, m]) => m));
    return { u: solve3(chosen.k, chosen.f)!, rows: chosen.rows };
  };

  let rows: Row[] = [];
  let u: number[] = [0, 0, 0];
  let lastForces = new Map<Slug, { fx: number; fy: number; m: number; normal: number; tangent: number }>();
  const finish = (): void => {
    for (const s of supports) {
      const o = lastForces.get(s.anchor)!;
      const mag = known.get(s.anchor) ?? 0;
      result.set(s.anchor, { fx: o.fx + mag * -s.n.y, fy: o.fy + mag * s.n.x, m: o.m + (knownMoment.get(s.anchor) ?? 0), normal: o.normal, tangent: sliding.has(s.anchor) ? mag : o.tangent, active: !dropped.has(s.anchor) });
    }
    for (const a of dropped) notes.push([`Contact ${a} lifted off (it would have to pull) and carries no load.`, `Kontakt ${a} abgehoben (er müsste ziehen) und trägt keine Last.`]);
    for (const a of sliding.keys()) notes.push([`Contact ${a} slides: friction capped at μ·N.`, `Kontakt ${a} gleitet: Reibung auf μ·N begrenzt.`]);
    for (const a of copCapped.keys()) notes.push([`Contact ${a}: the centre of pressure is clamped at the edge of the footprint (|M| = N·l/2); the foot tips about it.`, `Kontakt ${a}: der Druckmittelpunkt ist am Rand der Aufstandsfläche begrenzt (|M| = N·l/2); der Fuß kippt um diesen Rand.`]);
  };
  for (let iteration = 0; iteration < MAX_ITERATIONS; iteration++) {
    ({ u, rows } = solveOnce());
    const forces = new Map<Slug, { fx: number; fy: number; m: number; normal: number; tangent: number }>();
    for (const s of supports) forces.set(s.anchor, { fx: 0, fy: 0, m: 0, normal: 0, tangent: 0 });
    for (const r of rows) {
      const q = -r.k * (r.d[0] * u[0]! + r.d[1] * u[1]! + r.d[2] * u[2]!);
      const o = forces.get(r.anchor)!;
      if (r.role === "m") o.m += q;
      else {
        o.fx += q * r.d[0];
        o.fy += q * r.d[1];
        if (r.role === "n") o.normal = q;
        if (r.role === "t") o.tangent = q;
      }
    }
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
    const live = supports.filter((s) => !dropped.has(s.anchor));
    const allTensile = worst !== null && live.every((s) => s.type === "contact" && !forced.has(s.anchor) && forces.get(s.anchor)!.normal < -1e-9 * scale);
    if (worst && allTensile) {
      const keep = live.reduce((a, b) => (Math.abs(b.at.x - comX) < Math.abs(a.at.x - comX) ? b : a));
      for (const s of live) if (s !== keep) dropped.add(s.anchor);
      forced.add(keep.anchor);
      notes.push([`Every contact would pull; contact ${keep.anchor}, closest to the centre-of-mass projection, is kept active.`, `Jeder Kontakt müsste ziehen; Kontakt ${keep.anchor}, am nächsten zur Schwerpunktprojektion, bleibt aktiv.`]);
      changed = true;
    } else if (worst) {
      dropped.add(worst);
      sliding.delete(worst);
      known.delete(worst);
      copCapped.delete(worst);
      knownMoment.delete(worst);
      changed = true;
    } else {
      for (const s of supports) {
        if (s.type !== "contact" || dropped.has(s.anchor)) continue;
        const o = forces.get(s.anchor)!;
        const cap = s.mu * Math.max(o.normal, 0);
        if (!sliding.has(s.anchor) && Math.abs(o.tangent) > cap + 1e-9 * scale) {
          sliding.set(s.anchor, Math.sign(o.tangent));
          known.set(s.anchor, Math.sign(o.tangent) * cap);
          changed = true;
        } else if (sliding.has(s.anchor)) {
          const next = sliding.get(s.anchor)! * cap;
          if (Math.abs(next - (known.get(s.anchor) ?? 0)) > 1e-12 * scale) {
            known.set(s.anchor, next);
            changed = true;
          }
        }
      }
    }
    if (!worst) {
      for (const s of supports) {
        const length = footprint(s);
        if (!(length > 0) || dropped.has(s.anchor)) continue;
        const o = forces.get(s.anchor)!;
        const cap = (length / 2) * Math.max(o.normal, 0);
        const sign = copCapped.get(s.anchor);
        if (sign === undefined && Math.abs(o.m) > cap + 1e-9 * scale) {
          copCapped.set(s.anchor, Math.sign(o.m));
          knownMoment.set(s.anchor, Math.sign(o.m) * cap);
          changed = true;
        } else if (sign !== undefined && Math.abs(sign * cap - (knownMoment.get(s.anchor) ?? 0)) > 1e-12 * scale) {
          knownMoment.set(s.anchor, sign * cap);
          changed = true;
        }
      }
    }
    lastForces = forces;
    if (!changed) {
      finish();
      return result;
    }
    const key = [...dropped].sort().join(",") + "|" + [...sliding.keys()].sort().join(",") + "|" + [...copCapped.keys()].sort().join(",");
    if (seen.has(key) && iteration > 8) break;
    seen.add(key);
  }
  finish();
  notes.push(["Support iteration did not converge; the last iterate is reported.", "Auflageriteration nicht konvergiert; die letzte Iteration wird ausgewiesen."]);
  return result;
}
//#endregion 🧷️Reactions

//#region 📈️Members
type Cut = { fx: number; fy: number; m: number };

function distalCut(state: EvaluatedState, bone: Slug, own: Map<Slug, Piece[]>, attach: { s: number; sums: Sums }[], s: number, inclusive: boolean): Cut {
  const q = worldOf(state, bone, s, 0);
  const e = state.bones[bone]!;
  const ex = { x: Math.cos(e.angleRad), y: Math.sin(e.angleRad) };
  let fx = 0;
  let fy = 0;
  let m = 0;
  const take = (p: number): boolean => (inclusive ? p >= s - POSITION_TOLERANCE : p > s + POSITION_TOLERANCE);
  for (const piece of own.get(bone) ?? []) {
    const f = piece.force;
    if (piece.span) {
      const [s0, s1] = piece.span;
      const from = Math.max(s, s0);
      if (from >= s1 - POSITION_TOLERANCE) continue;
      const share = (s1 - from) / (s1 - s0);
      const centre = (from + s1) / 2 - (s0 + s1) / 2;
      const at = { x: f.at.x + ex.x * centre, y: f.at.y + ex.y * centre };
      fx += f.fxN * share;
      fy += f.fyN * share;
      m += (at.x - q.x) * f.fyN * share - (at.y - q.y) * f.fxN * share;
    } else if (take(localOf(state, bone, f.at).s)) {
      fx += f.fxN;
      fy += f.fyN;
      m += (f.at.x - q.x) * f.fyN - (f.at.y - q.y) * f.fxN + f.momentNm;
    }
  }
  for (const a of attach) {
    if (!take(a.s)) continue;
    fx += a.sums.fx;
    fy += a.sums.fy;
    m += a.sums.m0 - (q.x * a.sums.fy - q.y * a.sums.fx);
  }
  return { fx, fy, m };
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

function memberDiagram(state: EvaluatedState, bone: Bone, own: Map<Slug, Piece[]>, attach: { s: number; sums: Sums }[], active: readonly Support[]): MemberDiagram {
  const e = state.bones[bone.id]!;
  const L = bone.lengthM;
  const ex = { x: Math.cos(e.angleRad), y: Math.sin(e.angleRad) };
  const ey = { x: -ex.y, y: ex.x };
  const special: number[] = [];
  const plain: number[] = [];
  for (const piece of own.get(bone.id) ?? []) {
    if (piece.span) plain.push(piece.span[0], piece.span[1]);
    else special.push(localOf(state, bone.id, piece.force.at).s);
  }
  for (const a of attach) special.push(a.s);
  for (let i = 0; i <= DIAGRAM_INTERVALS; i++) plain.push((L * i) / DIAGRAM_INTERVALS);
  const entries: { s: number; pair: boolean }[] = [];
  const add = (s: number, pair: boolean): void => {
    if (s < -POSITION_TOLERANCE || s > L + POSITION_TOLERANCE) return;
    const clamped = Math.min(Math.max(s, 0), L);
    const hit = entries.find((x) => Math.abs(x.s - clamped) < POSITION_TOLERANCE);
    if (hit) hit.pair ||= pair;
    else entries.push({ s: clamped, pair });
  };
  special.forEach((s) => add(s, true));
  plain.forEach((s) => add(s, false));
  entries.sort((x, y) => x.s - y.s);
  const section = bone.section;
  const stations: DiagramStation[] = [];
  const push = (s: number, inclusive: boolean): void => {
    const cut = distalCut(state, bone.id, own, attach, s, inclusive);
    const axialN = cut.fx * ex.x + cut.fy * ex.y;
    const shearN = -(cut.fx * ey.x + cut.fy * ey.y);
    const sigma = section ? axialN / section.areaM2 : null;
    const bending = section ? (Math.abs(cut.m) * section.extremeFibreM) / section.secondMomentM4 : null;
    stations.push({ s, at: worldOf(state, bone.id, s, 0), axialN, shearN, momentNm: cut.m, stressMinPa: sigma === null ? null : sigma - bending!, stressMaxPa: sigma === null ? null : sigma + bending! });
  };
  for (const entry of entries) {
    push(entry.s, true);
    if (entry.pair) push(entry.s, false);
  }
  const restraints: { s: number; fixed: boolean }[] = [];
  for (const s of active) {
    if (s.bone !== bone.id) continue;
    const local = localOf(state, bone.id, s.at).s;
    if (s.type === "fixed") restraints.push({ s: local, fixed: true });
    else if (s.type === "pin") restraints.push({ s: local, fixed: false });
    else if (Math.abs(s.n.x * ey.x + s.n.y * ey.y) > 1e-3) restraints.push({ s: local, fixed: false });
  }
  const moment = (s: number): number => distalCut(state, bone.id, own, attach, s, false).m;
  return { bone: bone.id, stations, deflection: deflectionLine(state, bone, moment, restraints, bone.parent !== null) };
}
//#endregion 📈️Members

//#region 🏗️Solver
/** 🏗️ Solves reactions, joint loads and member diagrams of one evaluated state.
 *
 * Statically determinate supports (one fixed, pin plus roller) reproduce exact statics; indeterminate ones follow the equal-stiffness
 * elastic-support assumption stated in the result notes.
 *
 * @see ../🔗️contracts/🟦️.ts — {@link StructuralResult}
 */
export function solveStructure(state: EvaluatedState): StructuralResult {
  const asset = state.asset;
  const model = asset.structure;
  const inertial = model.dynamics === "inertial";
  const { weights, inertials } = massPieces(state, inertial);
  const applied: WorldForce[] = [...state.loads];
  const nonReaction: WorldForce[] = [...weights, ...inertials].map((p) => p.force).concat(applied);
  const supports = supportsOf(state);
  const collected: string[][] = [];
  const weightSumN = sumOf(weights.map((p) => -p.force.fyN));
  const comProjectionX = weightSumN > 0 ? sumOf(weights.map((p) => -p.force.fyN * p.force.at.x)) / weightSumN : 0;
  const solution = reactionSolution(supports, nonReaction, comProjectionX, collected);
  if (![...solution.values()].some((r) => r.active)) collected.push(["Unsupported: no active support (flight phase / ballistic motion or no contact); reactions are zero and the residual is the unbalanced weight plus inertia.", "Ungestützt: kein aktives Auflager (Flugphase / ballistische Bewegung oder kein Kontakt); Reaktionen sind null und das Residuum ist Gewicht plus Trägheit ohne Gegenkraft."]);
  const reactions: Reaction[] = supports.map((s) => {
    const r = solution.get(s.anchor)!;
    const area = s.def.footprintLengthM && s.def.footprintWidthM ? s.def.footprintLengthM * s.def.footprintWidthM : null;
    const along = r.fx * s.n.x + r.fy * s.n.y;
    const length = s.type === "contact" ? (s.def.footprintLengthM ?? 0) : 0;
    const offset = r.active && length > 0 && r.normal > 1e-9 ? Math.max(-length / 2, Math.min(length / 2, -r.m / r.normal)) : null;
    return { anchor: s.anchor, at: s.at, fxN: r.fx, fyN: r.fy, momentNm: r.m, pressurePa: area === null ? null : r.active ? Math.max(along, 0) / area : 0, active: r.active, ...(offset === null ? {} : { cop: { x: s.at.x - s.n.y * offset, y: s.at.y + s.n.x * offset } }) };
  });
  const reactionForces: WorldForce[] = reactions.map((r) => {
    const def = state.anchors[r.anchor]!.anchor;
    return { id: r.anchor, at: r.at, fxN: r.fxN, fyN: r.fyN, momentNm: r.momentNm, bone: def.bone, kind: "reaction", title: text(`Reaction: ${def.title.en}`, `Reaktion: ${def.title.de}`) };
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
    const total: Sums = { fx: 0, fy: 0, m0: 0 };
    for (const p of own.get(id) ?? []) {
      total.fx += p.force.fxN;
      total.fy += p.force.fyN;
      total.m0 += moment0(p.force);
    }
    for (const c of children.get(id) ?? []) {
      const s = subtree(c.id);
      total.fx += s.fx;
      total.fy += s.fy;
      total.m0 += s.m0;
    }
    sums.set(id, total);
    return total;
  };
  for (const b of asset.rig.bones) subtree(b.id);

  const joints: Record<Slug, JointLoad> = {};
  const members: Record<Slug, MemberDiagram> = {};
  const activeSupports = supports.filter((s) => solution.get(s.anchor)?.active);
  for (const b of asset.rig.bones) {
    const e = state.bones[b.id];
    if (!e) continue;
    const total = subtree(b.id);
    if (b.parent !== null) {
      const ex = { x: Math.cos(e.angleRad), y: Math.sin(e.angleRad) };
      joints[b.id] = {
        bone: b.id,
        at: e.origin,
        fxN: -total.fx,
        fyN: -total.fy,
        axialN: total.fx * ex.x + total.fy * ex.y,
        shearN: -(-total.fx * ex.y + total.fy * ex.x),
        momentNm: total.m0 - (e.origin.x * total.fy - e.origin.y * total.fx),
      };
    }
    if (b.lengthM > 0) {
      const attach = (children.get(b.id) ?? []).map((c) => ({ s: localOf(state, b.id, state.bones[c.id]!.origin).s, sums: subtree(c.id) }));
      members[b.id] = memberDiagram(state, b, own, attach, activeSupports);
    }
  }

  const everything = pieces.map((p) => p.force);
  const residual = { fxN: sumOf(everything.map((f) => f.fxN)), fyN: sumOf(everything.map((f) => f.fyN)), momentNm: sumOf(everything.map(moment0)) };
  const massed = weights.map((p) => p.force);
  const totalMassKg = sumOf(asset.rig.bones.map((b) => b.mass?.kg ?? 0));
  const weightSum = sumOf(massed.map((f) => -f.fyN));
  const centreOfMass: Point = weightSum > 0 ? { x: sumOf(massed.map((f) => (-f.fyN * f.at.x))) / weightSum, y: sumOf(massed.map((f) => -f.fyN * f.at.y)) / weightSum } : { x: 0, y: 0 };

  const notes: Text[] = [
    text("Contacts with a footprint length l transmit a moment |M| ≤ N·l/2 about the anchor (the centre of pressure moves within the sole, tyre patch or hoof, in the elevation plane); rotational stiffness k·l²/12, engaged only when the supports cannot balance the moment otherwise, capped at the limit like friction; `cop` is the resulting centre of pressure.", "Kontakte mit Aufstandslänge l übertragen ein Moment |M| ≤ N·l/2 um den Anker (der Druckmittelpunkt wandert in der Sohle, Reifenaufstandsfläche oder dem Huf, in der Aufrissebene); Drehsteifigkeit k·l²/12, nur aktiv, wenn die Auflager das Moment sonst nicht aufnehmen können, wie die Reibung auf die Grenze begrenzt; `cop` ist der resultierende Druckmittelpunkt."),
    text("Model: instantaneously rigid body on elastic supports of equal normalised stiffness (contact: normal 1, friction 0.5; pin, roller, fixed: 1); small-displacement 3x3 equilibrium; contacts that would pull are dropped; friction is capped at μ·N.", "Modell: momentan starrer Körper auf elastischen Auflagern gleicher normierter Steifigkeit (Kontakt: normal 1, Reibung 0,5; Gelenk, Rollenlager, Einspannung: 1); 3x3-Gleichgewicht kleiner Verschiebungen; ziehende Kontakte entfallen; Reibung auf μ·N begrenzt."),
    inertial
      ? text("Dynamics: inertial (d'Alembert from evaluated kinematics): forces -m·a at the segment centres; rotational inertia of segments is neglected. The residual includes the d'Alembert forces and is therefore also zero.", "Dynamik: inertial (d'Alembert aus den ausgewerteten Kinematiken): Kräfte -m·a in den Segmentschwerpunkten; Drehträgheit der Segmente vernachlässigt. Das Residuum enthält die d'Alembert-Kräfte und ist daher ebenfalls null.")
      : text("Dynamics: quasi-static (weights and applied loads only); the residual is zero when the supports can balance the load.", "Dynamik: quasi-statisch (nur Gewichte und aufgebrachte Lasten); das Residuum ist null, wenn die Auflager die Last aufnehmen können."),
    text(`Gravity ${model.gravityMS2} m/s²; uniform masses are discretised into ${WEIGHT_SUBDIVISIONS} sub-forces; contact friction defaults to μ = ${DEFAULT_FRICTION} where an anchor authors none.`, `Erdbeschleunigung ${model.gravityMS2} m/s²; gleichmäßige Massen in ${WEIGHT_SUBDIVISIONS} Teilkräfte zerlegt; Kontaktreibung μ = ${DEFAULT_FRICTION}, wo ein Anker keine angibt.`),
    text("Sign conventions: N > 0 tension; shear V = -Σ distal F⊥; bending M > 0 sagging; joint force is what the parent transmits to the child, joint moment is the moment of the child subtree's external loads about the joint (CCW +).", "Vorzeichen: N > 0 Zug; Querkraft V = -Σ distale F⊥; Moment M > 0 Feldmoment; Gelenkkraft ist die vom Elternteil auf das Kind übertragene Kraft, Gelenkmoment ist das Moment der äußeren Lasten des Kind-Teilbaums um das Gelenk (gegen den Uhrzeigersinn +)."),
    text("Deflection: Euler-Bernoulli double integration of M/EI for members with section and elastic modulus that are clamped, or supported at two points; shear and axial deformation are neglected.", "Durchbiegung: Euler-Bernoulli-Doppelintegration von M/EI für Stäbe mit Querschnitt und Elastizitätsmodul, die eingespannt oder an zwei Punkten gelagert sind; Schub- und Längsverformung vernachlässigt."),
    ...(asset.rig.kind === "anatomical" ? [text("Anatomical rig: joint values are simplified rigid-segment resultants, not clinically validated bone, muscle or joint forces.", "Anatomisches Skelett: Gelenkwerte sind vereinfachte Starrkörper-Resultierende, keine klinisch validierten Knochen-, Muskel- oder Gelenkkräfte.")] : []),
    ...model.assumptions,
    ...collected.map(([en, de]) => text(en!, de!)),
  ];
  const scale = 1 + sumOf(everything.map((f) => Math.hypot(f.fxN, f.fyN)));
  if (Math.hypot(residual.fxN, residual.fyN) > 1e-6 * scale || Math.abs(residual.momentNm) > 1e-6 * scale) notes.push(text("Unbalanced: the active supports cannot equilibrate the loads (tipping, sliding or no support); the residual is reported unchanged.", "Nicht im Gleichgewicht: die aktiven Auflager können die Lasten nicht aufnehmen (Kippen, Gleiten oder kein Auflager); das Residuum wird unverändert ausgewiesen."));

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
  shear: text("V = -Σ distal transverse force in the bone frame; sign shows direction, not tension or compression.", "V = -Σ distale Querkraft im Stabsystem; das Vorzeichen zeigt die Richtung, nicht Zug oder Druck."),
  stress: text("Tension positive (cool), compression negative (warm); σ = N/A ∓ |M|·c/I.", "Zug positiv (kühl), Druck negativ (warm); σ = N/A ∓ |M|·c/I."),
  "contact-pressure": text("Compression only: normal reaction divided by the footprint area.", "Nur Druck: Normalreaktion geteilt durch die Aufstandsfläche."),
  moment: text("Sagging positive (cool), hogging negative (warm); CCW moment of the distal loads about the cut.", "Feldmoment positiv (kühl), Stützmoment negativ (warm); Moment der distalen Lasten um den Schnitt gegen den Uhrzeigersinn."),
};

function quantityValues(quantity: StructuralQuantity, result: StructuralResult): number[] {
  if (quantity === "contact-pressure") return result.reactions.flatMap((r) => (r.pressurePa === null ? [] : [r.pressurePa]));
  const stations = Object.values(result.members).flatMap((m) => m.stations);
  switch (quantity) {
    case "axial":
      return stations.map((s) => s.axialN);
    case "shear":
      return stations.map((s) => s.shearN);
    case "moment":
      return stations.map((s) => s.momentNm);
    case "stress":
      return stations.flatMap((s) => (s.stressMinPa === null || s.stressMaxPa === null ? [] : [s.stressMinPa, s.stressMaxPa]));
  }
}

/** 🎨️ The legend of one structural quantity: unit, sign convention, range and colour stops (compression warm, tension cool, zero grey).
 *
 * @see ../🔗️contracts/🟦️.ts — {@link Legend}
 */
export function structuralLegend(quantity: StructuralQuantity, result: StructuralResult, range: DisplayRange): Legend {
  const values = quantityValues(quantity, result);
  const auto = values.length ? { min: Math.min(0, ...values), max: Math.max(0, ...values) } : { min: 0, max: 0 };
  const { min, max } = range.mode === "fixed" ? range : auto;
  const span = max - min;
  const c = STRUCTURAL_COLORS;
  const norm = (v: number): number => (span > 0 ? (v - min) / span : 0);
  const eps = 1e-6;
  const stops: { at: number; color: string }[] = [];
  if (quantity === "contact-pressure") stops.push({ at: norm(0), color: c.zero }, ...(span > 0 ? [{ at: norm(0) + eps, color: c.compressionWeak }, { at: 1, color: c.compressionStrong }] : []));
  else {
    const z = norm(0);
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
      const fx = sumOf(group.map((f) => f.fxN));
      const fy = sumOf(group.map((f) => f.fyN));
      const m0 = sumOf(group.map(moment0));
      const mag = Math.hypot(fx, fy);
      const at = mag > 0 ? centroidOf(group, fx, fy, m0) : group[0]!.at;
      return { id: `${bone}-${kind}`, kind, bone, at, fxN: fx, fyN: fy, momentNm: 0, title: group[0]!.title };
    });
  };
  return [
    ...merged(result.segmentWeights, "weight"),
    ...merged(result.inertialForces, "inertial"),
    ...result.applied.map((f): FreeBodyForce => ({ id: f.id, kind: "applied", bone: f.bone, at: f.at, fxN: f.fxN, fyN: f.fyN, momentNm: f.momentNm, title: f.title })),
    ...result.reactions.filter((r) => r.active).map((r): FreeBodyForce => ({ id: r.anchor, kind: "reaction", bone: null, at: r.at, fxN: r.fxN, fyN: r.fyN, momentNm: r.momentNm, title: text(`Reaction at ${r.anchor}`, `Reaktion bei ${r.anchor}`) })),
  ];
}

function centroidOf(group: readonly WorldForce[], fx: number, fy: number, m0: number): Point {
  const mag2 = fx * fx + fy * fy;
  const mean = { x: sumOf(group.map((f) => f.at.x)) / group.length, y: sumOf(group.map((f) => f.at.y)) / group.length };
  const along = (mean.x * fy - mean.y * fx - m0) / mag2;
  return { x: mean.x + along * -fy, y: mean.y + along * fx };
}
//#endregion 🎨️Depiction helpers
