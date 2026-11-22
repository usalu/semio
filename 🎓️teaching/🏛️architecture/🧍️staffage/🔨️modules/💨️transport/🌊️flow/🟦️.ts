/** 🌊️ The thermodynamic flow field of the transport model: an Eulerian velocity, temperature, vapour and condensate field reconstructed from the Lagrangian parcels, the streams traced through it and the tracers advected along them.
 *
 * Field at a point `x` ({@link flowSampler}), three-dimensional (ahead x, up y, left z):
 * - `u(x) = u_wind(y) + Σ_g u_g(x) + u_entrained(x) + u_inhaled(x)`.
 * - Mean flow of a jet or plume `g` (the parcels of one emitter and anchor): every parcel is a slice with top-hat radius `b`, relative velocity `u_p` and thickness `h = max(u_e τ, b/2)`; with `d∥` and `d⊥` the distances of `x` from the parcel along and across its velocity,
 *   `a_p = exp(−d∥²/(2h²))` interpolates along the axis and `G_p = 2·exp(−2 d⊥²/b²)` is the Gaussian profile that carries the same volume and momentum flux as the top hat (centreline twice the top-hat value, `b_G = b/√2`; Morton, Taylor & Turner 1956):
 *   `u_g = min(1, A)/A · Σ a_p G_p u_p`, `A = Σ a_p`. A parcel without momentum (`|u_p| < 1 cm/s`, transpired vapour) is isotropic: `a_p = exp(−2 d²/b²)`, `G_p = 1`. The scalars (temperature excess, vapour excess, condensate) use the same weights.
 * - Entrainment: the slice draws ambient air in at `q_p = 2π α(Ri) b ℓ |u_p|`, `ℓ = u_e τ` (Taylor's entrainment hypothesis, the inflow velocity at the edge is `α |u_p|`), drawn as a point sink regularised inside the plume, `u = −q_p d / (4π max(|d|, b)³)`,
 *   with its mirror image below the ground plane so that no air crosses `y = 0`: induced air glides along the floor into the plume. Seen from farther than {@link FAR_RADII} of its radius, a group of {@link GROUP_SLICES} consecutive slices acts as one sink of their summed strength at their strength-weighted centre. Inhalation openings are sinks of the inhaled flow, mirrored the same way.
 * Streams ({@link flowAt}) are instantaneous streamlines of that field traced both ways by Heun steps of fixed arc length from documented seeds: the nozzle rake of every active emitter, entrainment seeds 2.2 radii beside up to five stations of every plume and interior seeds 0.6 radii beside the first and third station (they show how wide the plume is),
 * six points around every inhalation opening and, when the ambient air moves faster than 5 cm/s, a vertical rake of six points at the upwind face of the domain. A stream ends where the induced speed falls below {@link FLOW_MIN_SPEED_MS} (the air there is undisturbed), at the faces of the domain ({@link flowDomainOf}) or in an inhalation opening; the streams of the wind rake end only at the domain. Each point carries the travel time `∫ ds/|u|`, the speed and the local thermodynamic state.
 * Tracers ({@link flowMarks}) are massless particles advected along the streams of the instant: a tracer released every `T` seconds (`periodS`, longer on slow streams, see {@link TRACER_GAP_STREAKS}) sits where the travel time equals `(t + φ + k·T) mod (N·T)`; its streak covers the last `exposureS` seconds of travel,
 * so its length is the local speed times the exposure and its head leads in the direction of flow. In steady flow streamlines are pathlines and the tracers are true particle paths; in unsteady flow they show the field of the instant.
 * Generalisation: marks are thinned to a world spacing (`spacingM`, the paper spacing times the scale), so 1:500 draws the same field with fewer, bolder marks.
 *
 * @see ../../🔗️contracts/🟦️.ts — `FlowField`, `FlowStream`, `FlowMarks`
 * @see ../🧮️parcels/🟦️.ts — the parcels the field is reconstructed from
 * @see https://doi.org/10.1098/rspa.1956.0011 — Morton, Taylor & Turner 1956, Turbulent gravitational convection from maintained and instantaneous sources
 * @see https://doi.org/10.1017/CBO9780511608827 — Turner 1973, Buoyancy Effects in Fluids
 */
import type { Emitter } from "../../../🧬️schema/🟦️.ts";
import type { EmitterState, FlowField, FlowMarks, FlowPuff, FlowStream, FlowStreak, Parcel, Vec3 } from "../../🔗️contracts/🟦️.ts";
import type { Ambient } from "../🌬️ambient/🟦️.ts";
import { GRAVITY_MS2, dewPointC } from "../🌫️air/🟦️.ts";
import { hash32, uniform } from "../🎲️noise/🟦️.ts";
import { ALPHA_JET, ALPHA_PLUME, RI_PLUME } from "../🧮️parcels/🟦️.ts";
import { STEPS_PER_S, emissionSteps } from "../🔥️sources/🟦️.ts";
import type { Sink } from "../〰️streamlines/🟦️.ts";
import * as portable from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

/** 🎛️ Options of {@link flowAt}. Defaults: arc-length step `extent/120` within 1.5…10 cm, 120 points downstream and 40 upstream of a seed (8 upstream of a nozzle seed, none upstream of the wind rake), at most 56 seeds, five entrainment stations per plume. */
export type FlowOptions = { readonly stepM?: number; readonly maxPointsPerSide?: number; readonly maxSeeds?: number; readonly stations?: number; readonly domain?: FlowDomain };

/** 📦️ The box the flow is shown in (world metres): streams end at its faces and puffs outside it are left out. */
export type FlowDomain = { readonly min: Vec3; readonly max: Vec3 };

/** 📦️ The domain around a body whose side elevation spans `bounds` and whose lateral half-width is `halfWidthM`: with `e` the largest extent of the body, a margin of `0.75 e` (0.9 to 5 m) on every side and of `e` (1.2 to 6 m) above; the ground closes it below. */
export function flowDomainOf(bounds: { readonly minX: number; readonly minY: number; readonly maxX: number; readonly maxY: number }, halfWidthM: number): FlowDomain {
  const extent = Math.max(bounds.maxX - bounds.minX, bounds.maxY - bounds.minY, 2 * halfWidthM);
  const beside = Math.min(5, Math.max(0.9, 0.75 * extent));
  const above = Math.min(6, Math.max(1.2, extent));
  return { min: { x: bounds.minX - beside, y: 0, z: -halfWidthM - beside }, max: { x: bounds.maxX + beside, y: bounds.maxY + above, z: halfWidthM + beside } };
}

/** 🧪️ What the field is at a point: velocity (m/s), the speed of the part of it that the sources induce (everything but the ambient wind, m/s), temperature excess (K), vapour excess and condensate (kg/kg moist air). */
export type FlowSample = { readonly u: Vec3; readonly inducedMS: number; readonly excessK: number; readonly vapourExcessKgPerKg: number; readonly condensateKgPerKg: number };

/** 🧮️ Induced speed below which a stream ends (m/s): the air there moves as the undisturbed ambient air does. The streams of the wind rake end only at the domain. */
export const FLOW_MIN_SPEED_MS = 0.004;
/** 🧮️ Relative speed below which a parcel is a passive puff without an axis (m/s). */
export const PASSIVE_SPEED_MS = 0.01;
/** 🌱️ Distance of the entrainment seeds from the plume axis in parcel radii. */
export const ENTRAINMENT_SEED_RADII = 2.2;
/** 🌱️ Distance of the interior seeds from the plume axis in parcel radii (first and third station). */
export const INTERIOR_SEED_RADII = 0.6;
/** ⏱️ Seconds between two tracers on one stream. */
export const TRACER_PERIOD_S = 1.5;
/** 📏️ Smallest mean distance between two tracers of one stream in shortest streaks: on slow streams the release period grows to `travel time · TRACER_GAP_STREAKS · minStreakM / length`. */
export const TRACER_GAP_STREAKS = 4;
/** 📸️ Seconds of travel a tracer streak covers. */
export const TRACER_EXPOSURE_S = 0.4;

const STRIDE = 16;
const X = 0, Y = 1, Z = 2, UX = 3, UY = 4, UZ = 5, DX = 6, DY = 7, DZ = 8, B = 9, H = 10, Q = 11, TK = 12, WV = 13, WL = 14, PASSIVE = 15;
type Group = { readonly from: number; readonly to: number; readonly minX: number; readonly minY: number; readonly minZ: number; readonly maxX: number; readonly maxY: number; readonly maxZ: number; readonly q: number; readonly cx: number; readonly cy: number; readonly cz: number; readonly far2: number };
type Plume = { readonly data: Float64Array; readonly groups: readonly Group[] };

const CUT = 3.5;
/** 🧮️ Slices per group of a jet or plume; a group farther away than {@link FAR_RADII} of its own radius acts as one sink at its entrainment-weighted centre. */
export const GROUP_SLICES = 12;
export const FAR_RADII = 6;

const add = (a: Vec3, b: Vec3, s = 1): Vec3 => ({ x: a.x + b.x * s, y: a.y + b.y * s, z: a.z + b.z * s });
const norm = (a: Vec3): number => Math.sqrt(a.x * a.x + a.y * a.y + a.z * a.z);

/** 🏷️ The jet or plume a parcel belongs to: its emitter and anchor (`emitter@anchor#k` or `emitter#k`). */
export const streamOf = (parcelId: string): string => parcelId.slice(0, parcelId.lastIndexOf("#"));

/** 🧮️ The entrainment coefficient of a slice: jet value at `Ri = 0`, plume value from `Ri_p` on (Kaminski, Tait & Carazzo 2005), with `g' ≈ g(ΔT/T + 0.608 Δq − q_l)`. */
export function entrainmentOf(parcel: Pick<Parcel, "excessK" | "vapourExcessKgPerKg" | "condensateKgPerKg" | "radiusM">, relativeSpeedMS: number, ambient: Pick<Ambient, "tC" | "sigmaMS">): { readonly alpha: number; readonly ueMS: number } {
  const ue = Math.sqrt(relativeSpeedMS * relativeSpeedMS + ambient.sigmaMS * ambient.sigmaMS);
  const buoyancy = GRAVITY_MS2 * (parcel.excessK / (ambient.tC + 273.15) + 0.608 * parcel.vapourExcessKgPerKg - parcel.condensateKgPerKg);
  const ri = (Math.abs(buoyancy) * parcel.radiusM) / (ue * ue);
  return { alpha: ALPHA_JET + (ALPHA_PLUME - ALPHA_JET) * Math.min(1, ri / RI_PLUME), ueMS: ue };
}

function plumesOf(parcels: readonly Parcel[], definitions: readonly Emitter[], ambient: Ambient): Plume[] {
  const tau = new Map(definitions.map((e) => [e.id, emissionSteps(e) / STEPS_PER_S]));
  const lists = new Map<string, Parcel[]>();
  for (const p of parcels) {
    const key = streamOf(p.id);
    const list = lists.get(key);
    if (list === undefined) lists.set(key, [p]);
    else list.push(p);
  }
  return [...lists.values()].map((list) => {
    const data = new Float64Array(list.length * STRIDE);
    list.forEach((p, i) => {
      const o = i * STRIDE;
      const wind = ambient.windAt(Math.max(0, p.at.y));
      const ux = p.velocity.x - wind.x;
      const uy = p.velocity.y - wind.y;
      const uz = p.velocity.z - wind.z;
      const speed = Math.sqrt(ux * ux + uy * uy + uz * uz);
      const b = Math.max(p.radiusM, 1e-3);
      const { alpha, ueMS } = entrainmentOf(p, speed, ambient);
      const length = ueMS * (tau.get(p.emitter) ?? 1 / 15);
      const axis = norm(p.velocity);
      const still = speed < PASSIVE_SPEED_MS || axis === 0;
      data.set([p.at.x, p.at.y, p.at.z, ux, uy, uz, still ? 0 : p.velocity.x / axis, still ? 1 : p.velocity.y / axis, still ? 0 : p.velocity.z / axis, b, Math.max(length, b / 2), 2 * Math.PI * alpha * b * length * speed, p.excessK, p.vapourExcessKgPerKg, p.condensateKgPerKg, still ? 1 : 0], o);
    });
    const groups: Group[] = [];
    for (let from = 0; from < list.length; from += GROUP_SLICES) {
      const to = Math.min(list.length, from + GROUP_SLICES);
      let minX = Infinity, minY = Infinity, minZ = Infinity, maxX = -Infinity, maxY = -Infinity, maxZ = -Infinity, q = 0, cx = 0, cy = 0, cz = 0;
      for (let i = from; i < to; i++) {
        const o = i * STRIDE;
        const reach = CUT * Math.max(data[o + B]!, data[o + H]!);
        minX = Math.min(minX, data[o + X]! - reach); maxX = Math.max(maxX, data[o + X]! + reach);
        minY = Math.min(minY, data[o + Y]! - reach); maxY = Math.max(maxY, data[o + Y]! + reach);
        minZ = Math.min(minZ, data[o + Z]! - reach); maxZ = Math.max(maxZ, data[o + Z]! + reach);
        q += data[o + Q]!; cx += data[o + Q]! * data[o + X]!; cy += data[o + Q]! * data[o + Y]!; cz += data[o + Q]! * data[o + Z]!;
      }
      if (q > 0) { cx /= q; cy /= q; cz /= q; }
      let radius = 0;
      for (let i = from; i < to; i++) {
        const o = i * STRIDE;
        radius = Math.max(radius, Math.sqrt((data[o + X]! - cx) ** 2 + (data[o + Y]! - cy) ** 2 + (data[o + Z]! - cz) ** 2) + data[o + B]!);
      }
      groups.push({ from, to, minX, minY, minZ, maxX, maxY, maxZ, q, cx, cy, cz, far2: (FAR_RADII * radius) ** 2 });
    }
    return { data, groups };
  });
}

/** 🧭️ The field of the instant as a function of the position: see the module text for the terms. Parcels of one emitter and anchor form one jet or plume; `definitions` give the emission interval of each emitter. */
export function flowSampler(parcels: readonly Parcel[], definitions: readonly Emitter[], ambient: Ambient, sinks: readonly Sink[]): (at: Vec3) => FlowSample {
  const plumes = plumesOf(parcels, definitions, ambient);
  const FOUR_PI = 4 * Math.PI;
  return (at) => {
    const wind = ambient.windAt(Math.max(0, at.y));
    const px = at.x, py = at.y, pz = at.z;
    let vx = wind.x, vy = wind.y, vz = wind.z, excessK = 0, vapour = 0, condensate = 0;
    for (const { data, groups } of plumes) {
      let a = 0, sx = 0, sy = 0, sz = 0, st = 0, sv = 0, sc = 0;
      for (const g of groups) {
        if (g.q > 0) {
          const gx = px - g.cx, gz = pz - g.cz;
          const gy = py - g.cy, my = py + g.cy;
          const r2 = gx * gx + gy * gy + gz * gz;
          if (r2 > g.far2) {
            const m2 = gx * gx + my * my + gz * gz;
            const k = -g.q / (FOUR_PI * r2 * Math.sqrt(r2));
            const km = -g.q / (FOUR_PI * m2 * Math.sqrt(m2));
            vx += (k + km) * gx; vy += k * gy + km * my; vz += (k + km) * gz;
          } else
            for (let i = g.from; i < g.to; i++) {
              const o = i * STRIDE;
              const q = data[o + Q]!;
              if (q === 0) continue;
              const b = data[o + B]!;
              const dx = px - data[o + X]!, dz = pz - data[o + Z]!;
              const dy = py - data[o + Y]!, ey = py + data[o + Y]!;
              const r = Math.max(Math.sqrt(dx * dx + dy * dy + dz * dz), b);
              const m = Math.max(Math.sqrt(dx * dx + ey * ey + dz * dz), b);
              const k = -q / (FOUR_PI * r * r * r);
              const km = -q / (FOUR_PI * m * m * m);
              vx += (k + km) * dx; vy += k * dy + km * ey; vz += (k + km) * dz;
            }
        }
        if (px < g.minX || px > g.maxX || py < g.minY || py > g.maxY || pz < g.minZ || pz > g.maxZ) continue;
        for (let i = g.from; i < g.to; i++) {
          const o = i * STRIDE;
          const b = data[o + B]!;
          const dx = px - data[o + X]!, dy = py - data[o + Y]!, dz = pz - data[o + Z]!;
          const r2 = dx * dx + dy * dy + dz * dz;
          let w: number, profile: number;
          if (data[o + PASSIVE] === 1) {
            if (r2 > CUT * CUT * b * b) continue;
            w = portable.fastExp((-2 * r2) / (b * b));
            profile = 1;
          } else {
            const h = data[o + H]!;
            const along = dx * data[o + DX]! + dy * data[o + DY]! + dz * data[o + DZ]!;
            const across2 = Math.max(0, r2 - along * along);
            if (Math.abs(along) > CUT * h || across2 > CUT * CUT * b * b) continue;
            w = portable.fastExp((-along * along) / (2 * h * h));
            profile = 2 * portable.fastExp((-2 * across2) / (b * b));
          }
          a += w;
          const k = w * profile;
          sx += k * data[o + UX]!; sy += k * data[o + UY]!; sz += k * data[o + UZ]!;
          st += k * data[o + TK]!; sv += k * data[o + WV]!; sc += k * data[o + WL]!;
        }
      }
      if (a <= 0) continue;
      const gain = Math.min(1, a) / a;
      vx += gain * sx; vy += gain * sy; vz += gain * sz;
      excessK += gain * st; vapour += gain * sv; condensate += gain * sc;
    }
    for (const s of sinks) {
      const dx = px - s.at.x, dz = pz - s.at.z;
      const dy = py - s.at.y, ey = py + s.at.y;
      const r = Math.max(Math.sqrt(dx * dx + dy * dy + dz * dz), s.diameterM);
      const m = Math.max(Math.sqrt(dx * dx + ey * ey + dz * dz), s.diameterM);
      const k = -s.flowM3S / (FOUR_PI * r * r * r);
      const km = -s.flowM3S / (FOUR_PI * m * m * m);
      vx += (k + km) * dx; vy += k * dy + km * ey; vz += (k + km) * dz;
    }
    const ix = vx - wind.x, iy = vy - wind.y, iz = vz - wind.z;
    return { u: { x: vx, y: vy, z: vz }, inducedMS: Math.sqrt(ix * ix + iy * iy + iz * iz), excessK, vapourExcessKgPerKg: vapour, condensateKgPerKg: condensate };
  };
}

type Side = { readonly points: Vec3[]; readonly samples: FlowSample[]; readonly speeds: number[] };

/** 〰️ One side of a stream from `seed`: Heun steps of arc length `stepM` along (`sign` 1) or against (−1) the field (the mean of the unit directions at the point and at the predicted point), at most `maxPoints`, ending where the induced speed falls below `minInducedMS`, outside `box` or inside an inhalation opening; the ground keeps the stream at `y ≥ 0`. */
export function traceStream(sample: (at: Vec3) => FlowSample, seed: Vec3, sign: 1 | -1, stepM: number, maxPoints: number, box: FlowDomain, sinks: readonly Sink[] = [], first: FlowSample = sample(seed), minInducedMS = FLOW_MIN_SPEED_MS): Side {
  const points: Vec3[] = [];
  const samples: FlowSample[] = [];
  const speeds: number[] = [];
  let p = seed;
  let here = first;
  for (let i = 0; i < maxPoints; i++) {
    const a = here.u;
    const sa = norm(a);
    if (sa < 1e-6 || here.inducedMS < minInducedMS) break;
    const predicted = add(p, a, (sign * stepM) / sa);
    const b = sample({ x: predicted.x, y: Math.max(0, predicted.y), z: predicted.z }).u;
    const sb = norm(b) || 1;
    const mean = { x: (a.x / sa + b.x / sb) / 2, y: (a.y / sa + b.y / sb) / 2, z: (a.z / sa + b.z / sb) / 2 };
    const sm = norm(mean);
    if (sm < 1e-6) break;
    const next = add(p, mean, (sign * stepM) / sm);
    p = { x: next.x, y: Math.max(0, next.y), z: next.z };
    if (p.x < box.min.x || p.x > box.max.x || p.y > box.max.y || p.z < box.min.z || p.z > box.max.z) break;
    if (sinks.some((s) => norm({ x: s.at.x - p.x, y: s.at.y - p.y, z: s.at.z - p.z }) < Math.max(s.diameterM, stepM))) break;
    here = sample(p);
    const speed = norm(here.u);
    if (speed < 1e-6 || here.inducedMS < minInducedMS) break;
    points.push(p);
    samples.push(here);
    speeds.push(speed);
  }
  return { points, samples, speeds };
}

function transverse(d: Vec3): { readonly up: Vec3; readonly side: Vec3 } {
  const horizontal = Math.sqrt(d.x * d.x + d.z * d.z);
  if (horizontal < 1e-6) return { up: { x: 1, y: 0, z: 0 }, side: { x: 0, y: 0, z: 1 } };
  const side = { x: -d.z / horizontal, y: 0, z: d.x / horizontal };
  const up = { x: (-d.y * d.x) / horizontal, y: horizontal, z: (-d.y * d.z) / horizontal };
  const l = norm(up);
  return { up: { x: up.x / l, y: up.y / l, z: up.z / l }, side };
}

type Seed = { readonly at: Vec3; readonly role: FlowStream["role"]; readonly emitter: string | null };

/** 🌱️ The seeds of the streams (see the module text), most important first. */
export function flowSeeds(parcels: readonly Parcel[], emitters: readonly EmitterState[], definitions: readonly Emitter[], ambient: Ambient, sinks: readonly Sink[], stations = 5, domain?: FlowDomain): readonly Seed[] {
  const core: Seed[] = [];
  const entrained: Seed[] = [];
  const chains = new Map<string, Parcel[]>();
  for (const p of parcels) {
    const key = streamOf(p.id);
    const list = chains.get(key);
    if (list === undefined) chains.set(key, [p]);
    else list.push(p);
  }
  for (const state of emitters) {
    if (!state.active || state.kind === "transpiration") continue;
    const d = definitions.find((e) => e.id === state.emitter)?.diameterM ?? 0.02;
    const { up, side } = transverse(state.direction);
    const lateral = 0.3 * d;
    const centre = add(state.at, state.direction, 0.5 * d);
    core.push({ at: centre, role: "source", emitter: state.emitter });
    for (const [axis, sign] of [[up, 1], [up, -1], [side, 1], [side, -1]] as const) core.push({ at: add(centre, axis, sign * lateral), role: "source", emitter: state.emitter });
  }
  for (const [, chain] of chains) {
    const moving = chain.filter((p) => {
      const wind = ambient.windAt(Math.max(0, p.at.y));
      return norm({ x: p.velocity.x - wind.x, y: p.velocity.y - wind.y, z: p.velocity.z - wind.z }) >= 2 * PASSIVE_SPEED_MS;
    }).sort((a, b) => a.ageS - b.ageS);
    if (moving.length < 2) continue;
    const count = Math.min(stations, moving.length - 1);
    for (let i = 1; i <= count; i++) {
      const p = moving[Math.min(moving.length - 1, Math.round((i * (moving.length - 1)) / (count + 1)))]!;
      const speed = norm(p.velocity) || 1;
      const { up, side } = transverse({ x: p.velocity.x / speed, y: p.velocity.y / speed, z: p.velocity.z / speed });
      const reach = ENTRAINMENT_SEED_RADII * p.radiusM;
      for (const [axis, sign] of (i % 2 === 1 ? [[up, 1], [up, -1], [side, 1], [side, -1]] : [[up, 1], [up, -1]]) as readonly (readonly [Vec3, number])[]) {
        const at = add(p.at, axis, sign * reach);
        entrained.push({ at: { x: at.x, y: Math.max(0.01, at.y), z: at.z }, role: "entrained", emitter: p.emitter });
      }
      if (i === 1 || i === 3)
        for (const [axis, sign] of [[up, 1], [up, -1], [side, 1], [side, -1]] as const) {
          const at = add(p.at, axis, sign * INTERIOR_SEED_RADII * p.radiusM);
          core.push({ at: { x: at.x, y: Math.max(0.01, at.y), z: at.z }, role: "source", emitter: p.emitter });
        }
    }
  }
  const inhaled: Seed[] = sinks.flatMap((sink) => [{ x: 1, y: 0, z: 0 }, { x: -1, y: 0, z: 0 }, { x: 0, y: 1, z: 0 }, { x: 0, y: -1, z: 0 }, { x: 0, y: 0, z: 1 }, { x: 0, y: 0, z: -1 }].map((axis) => ({ at: add(sink.at, axis, 4 * sink.diameterM), role: "inhaled" as const, emitter: null })));
  const windy: Seed[] = [];
  if (ambient.environment.windMS > 0.05 && emitters.length > 0) {
    const phi = (ambient.environment.windDirectionDeg * Math.PI) / 180;
    const h = { x: portable.cos(phi), y: 0, z: portable.sin(phi) };
    const along = (at: Vec3): number => at.x * h.x + at.z * h.z;
    const places = [...emitters.map((e) => e.at), ...parcels.map((p) => p.at)];
    const first = places.reduce((best, at) => (along(at) < along(best) ? at : best));
    const centre = emitters.reduce((sum, e) => ({ x: sum.x + e.at.x / emitters.length, y: 0, z: sum.z + e.at.z / emitters.length }), { x: 0, y: 0, z: 0 });
    const back = domain === undefined ? along(centre) - along(first) + 0.4 : Math.min(...[domain.min.x, domain.max.x].flatMap((x) => [domain.min.z, domain.max.z].map((z) => Math.abs(h.x) > 1e-9 || Math.abs(h.z) > 1e-9 ? Math.max(Math.abs(h.x) > 1e-9 ? (centre.x - x) / h.x : -Infinity, Math.abs(h.z) > 1e-9 ? (centre.z - z) / h.z : -Infinity) : 0)).filter((d) => d > 0)) - 0.02;
    const base = add(centre, h, -back);
    const top = Math.min(domain?.max.y ?? Infinity, Math.max(...places.map((at) => at.y)) + 0.2);
    for (let i = 0; i < 6; i++) windy.push({ at: { x: base.x, y: 0.08 + ((top - 0.08) * i) / 5, z: base.z }, role: "ambient", emitter: null });
  }
  const out: Seed[] = [];
  for (let i = 0; i < Math.max(core.length, entrained.length, inhaled.length, windy.length); i++) for (const list of [core, entrained, windy, inhaled]) if (list[i] !== undefined) out.push(list[i]!);
  return out;
}

/** 〰️ The stream through a seed: traced upstream (a third of `maxPoints`; 8 points for a nozzle seed, none for the wind rake) and downstream (`maxPoints`; three times as many for the wind rake), with the travel time from its first point by the trapezoid rule on the slowness, `Δt = 2Δs/(|u₀| + |u₁|)`. `null` where the air does not move (induced speed below {@link FLOW_MIN_SPEED_MS}; the wind rake only needs moving air) or the stream has fewer than three points. */
export function streamFrom(sample: (at: Vec3) => FlowSample, seed: { readonly at: Vec3; readonly role: FlowStream["role"]; readonly emitter: string | null }, stepM: number, maxPoints: number, domain: FlowDomain, sinks: readonly Sink[] = []): FlowStream | null {
  const here = sample(seed.at);
  const speedHere = norm(here.u);
  const floor = seed.role === "ambient" ? 0 : FLOW_MIN_SPEED_MS;
  if (speedHere < 1e-6 || here.inducedMS < floor) return null;
  const back = seed.role === "ambient" ? { points: [], samples: [], speeds: [] } : traceStream(sample, seed.at, -1, stepM, seed.role === "source" ? 8 : Math.round(maxPoints / 3), domain, sinks, here, floor);
  const forward = traceStream(sample, seed.at, 1, stepM, seed.role === "ambient" ? 3 * maxPoints : maxPoints, domain, sinks, here, floor);
  if (back.points.length + forward.points.length < 2) return null;
  const points = [...back.points.reverse(), seed.at, ...forward.points];
  const samples = [...back.samples.reverse(), here, ...forward.samples];
  const speedMS = [...back.speeds.reverse(), speedHere, ...forward.speeds];
  const travelS = [0];
  for (let i = 1; i < points.length; i++) {
    const ds = norm({ x: points[i]!.x - points[i - 1]!.x, y: points[i]!.y - points[i - 1]!.y, z: points[i]!.z - points[i - 1]!.z });
    travelS.push(travelS[i - 1]! + (2 * ds) / (speedMS[i]! + speedMS[i - 1]!));
  }
  return { seed: seed.at, role: seed.role, emitter: seed.emitter, points, travelS, speedMS, excessK: samples.map((s) => s.excessK), vapourExcessKgPerKg: samples.map((s) => s.vapourExcessKgPerKg), condensateKgPerKg: samples.map((s) => s.condensateKgPerKg) };
}

/** 🌊️ The flow field of an instant: streams with travel time, speed and thermodynamic state, the puffs (parcels that carry heat, vapour or condensate) and the ambient state the legend states. */
export function flowAt(timeS: number, parcels: readonly Parcel[], emitters: readonly EmitterState[], definitions: readonly Emitter[], ambient: Ambient, sinks: readonly Sink[], options: FlowOptions = {}): FlowField {
  const sample = flowSampler(parcels, definitions, ambient, sinks);
  const places = [...emitters.map((e) => e.at), ...parcels.map((p) => p.at)];
  const reach = Math.max(0.15, ...parcels.map((p) => p.radiusM));
  const domain: FlowDomain = options.domain ?? { min: { x: Math.min(...places.map((p) => p.x)) - 3 * reach - 0.3, y: 0, z: Math.min(...places.map((p) => p.z)) - 3 * reach - 0.3 }, max: { x: Math.max(...places.map((p) => p.x)) + 3 * reach + 0.3, y: Math.max(...places.map((p) => p.y)) + 2 * reach + 0.3, z: Math.max(...places.map((p) => p.z)) + 3 * reach + 0.3 } };
  const inside = (at: Vec3): boolean => at.x >= domain.min.x && at.x <= domain.max.x && at.y <= domain.max.y && at.z >= domain.min.z && at.z <= domain.max.z;
  const extent = places.length === 0 ? 1 : norm({ x: domain.max.x - domain.min.x, y: domain.max.y - domain.min.y, z: domain.max.z - domain.min.z });
  const stepM = options.stepM ?? Math.min(0.1, Math.max(0.015, extent / 120));
  const maxPoints = options.maxPointsPerSide ?? 120;
  const streams: FlowStream[] = [];
  if (places.length > 0)
    for (const seed of flowSeeds(parcels.filter((p) => inside(p.at)), emitters, definitions, ambient, sinks, options.stations ?? 5, domain).slice(0, options.maxSeeds ?? 56)) {
      const stream = inside(seed.at) ? streamFrom(sample, seed, stepM, maxPoints, domain, sinks) : null;
      if (stream !== null) streams.push(stream);
    }
  const puffs: FlowPuff[] = parcels.filter((p) => inside(p.at)).map((p) => ({ at: p.at, radiusM: p.radiusM, excessK: p.excessK, vapourExcessKgPerKg: p.vapourExcessKgPerKg, condensateKgPerKg: p.condensateKgPerKg, carries: p.carries }));
  const e = ambient.environment;
  return { timeS, streams, puffs, ambient: { airTempC: e.airTempC, relativeHumidity: e.relativeHumidity, dewPointC: dewPointC(ambient.wv, ambient.pressurePa), windMS: e.windMS, windDirectionDeg: e.windDirectionDeg, pressurePa: e.pressurePa }, model: { stepM, periodS: TRACER_PERIOD_S, exposureS: TRACER_EXPOSURE_S } };
}

/** 📍️ The point of a stream at a travel time (clamped to the stream) with the interpolated state. */
export function streamAt(stream: FlowStream, travelS: number): { readonly at: Vec3; readonly speedMS: number; readonly excessK: number; readonly vapourExcessKgPerKg: number; readonly condensateKgPerKg: number; readonly index: number; readonly fraction: number } {
  const t = stream.travelS;
  const last = t.length - 1;
  const clamped = Math.min(t[last]!, Math.max(0, travelS));
  let lo = 0;
  let hi = last;
  while (hi - lo > 1) {
    const mid = (lo + hi) >> 1;
    if (t[mid]! <= clamped) lo = mid;
    else hi = mid;
  }
  const span = t[hi]! - t[lo]!;
  const f = span > 0 ? (clamped - t[lo]!) / span : 0;
  const mix = (values: readonly number[]): number => values[lo]! + (values[hi]! - values[lo]!) * f;
  const a = stream.points[lo]!;
  const b = stream.points[hi]!;
  return { at: { x: a.x + (b.x - a.x) * f, y: a.y + (b.y - a.y) * f, z: a.z + (b.z - a.z) * f }, speedMS: mix(stream.speedMS), excessK: mix(stream.excessK), vapourExcessKgPerKg: mix(stream.vapourExcessKgPerKg), condensateKgPerKg: mix(stream.condensateKgPerKg), index: lo, fraction: f };
}

/** 🎚️ How the marks of a flow field are generalised: `spacingM` is the smallest world distance between two drawn streams or puffs (paper spacing times the scale), `minStreakM` the shortest streak that still reads as a direction; `periodS` and `exposureS` default to the field's own. */
export type FlowDetail = { readonly spacingM: number; readonly minStreakM: number; readonly periodS?: number; readonly exposureS?: number };

/** 📐️ The generalisation of a drawing scale: streams and puffs at least 2.2 mm of paper apart, streaks at least 0.9 mm long. */
export const flowDetailOf = (denominator: number): FlowDetail => ({ spacingM: (2.2 * denominator) / 1000, minStreakM: (0.9 * denominator) / 1000 });

const distance2 = (a: Vec3, b: Vec3): number => (a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y) + (a.z - b.z) * (a.z - b.z);

/** 🧹️ The streams that stay at a spacing: in their given order a stream is kept when fewer than 60 % of its points lie within `spacingM` of a point of a stream already kept. */
export function thinStreams(streams: readonly FlowStream[], spacingM: number): readonly number[] {
  const kept: number[] = [];
  const cloud: Vec3[] = [];
  const s2 = spacingM * spacingM;
  streams.forEach((stream, index) => {
    const stride = Math.max(1, Math.floor(stream.points.length / 24));
    const probes = stream.points.filter((_, i) => i % stride === 0);
    const crowded = probes.filter((p) => cloud.some((c) => distance2(p, c) < s2)).length;
    if (crowded > 0.6 * probes.length) return;
    kept.push(index);
    cloud.push(...probes);
  });
  return kept;
}

/** 🧹️ The puffs that stay at a spacing: strongest first (condensate, then vapour, then heat), a puff is kept when no kept puff lies within `spacingM` or within 0.4 of that puff's radius. */
export function thinPuffs(puffs: readonly FlowPuff[], spacingM: number): readonly FlowPuff[] {
  const order = [...puffs].sort((a, b) => b.condensateKgPerKg - a.condensateKgPerKg || b.vapourExcessKgPerKg - a.vapourExcessKgPerKg || Math.abs(b.excessK) - Math.abs(a.excessK));
  const kept: FlowPuff[] = [];
  const s2 = spacingM * spacingM;
  for (const puff of order) if (!kept.some((k) => distance2(k.at, puff.at) < Math.max(s2, 0.16 * k.radiusM * k.radiusM))) kept.push(puff);
  return kept;
}

const THINNED = new WeakMap<FlowField, Map<number, { readonly kept: readonly number[]; readonly puffs: readonly FlowPuff[] }>>();

function thinned(flow: FlowField, spacingM: number): { readonly kept: readonly number[]; readonly puffs: readonly FlowPuff[] } {
  let cache = THINNED.get(flow);
  if (cache === undefined) THINNED.set(flow, (cache = new Map()));
  let entry = cache.get(spacingM);
  if (entry === undefined) cache.set(spacingM, (entry = { kept: thinStreams(flow.streams, spacingM), puffs: thinPuffs(flow.puffs, spacingM) }));
  return entry;
}

/** ✨️ The marks of a flow field at a time (seconds on the same timeline as `flow.timeS`): the kept streams as ribbons with direction arrows, the tracers of the instant as streaks (tail to head, fading in over the first tenth and out over the last fifth of the stream) and the kept puffs. A pure function of `(flow, timeS, detail)`. */
export function flowMarks(flow: FlowField, timeS: number, detail: FlowDetail): FlowMarks {
  const period = detail.periodS ?? flow.model.periodS;
  const exposure = detail.exposureS ?? flow.model.exposureS;
  const { kept, puffs } = thinned(flow, detail.spacingM);
  const streaks: FlowStreak[] = [];
  const ribbons = kept.map((index) => {
    const stream = flow.streams[index]!;
    const total = stream.travelS[stream.travelS.length - 1]!;
    const arrows: { at: Vec3; direction: Vec3 }[] = [];
    const lengthM = stream.points.reduce((sum, p, i) => (i === 0 ? 0 : sum + Math.sqrt(distance2(p, stream.points[i - 1]!))), 0);
    const count = Math.max(1, Math.min(4, Math.floor(lengthM / (12 * detail.spacingM))));
    for (let k = 1; k <= count; k++) {
      const i = Math.min(stream.points.length - 1, Math.max(1, Math.round((k * (stream.points.length - 1)) / (count + 1))));
      const a = stream.points[i - 1]!;
      const b = stream.points[i]!;
      const l = Math.sqrt(distance2(a, b)) || 1;
      arrows.push({ at: b, direction: { x: (b.x - a.x) / l, y: (b.y - a.y) / l, z: (b.z - a.z) / l } });
    }
    const every = Math.max(period, (total * TRACER_GAP_STREAKS * detail.minStreakM) / Math.max(lengthM, 1e-9));
    const n = Math.max(1, Math.ceil(total / every));
    const cycle = n * every;
    const phase = uniform(hash32(index, Math.round(stream.seed.x * 1000), Math.round(stream.seed.y * 1000), Math.round(stream.seed.z * 1000))) * every;
    for (let k = 0; k < n; k++) {
      const head = (((timeS + phase + k * every) % cycle) + cycle) % cycle;
      if (head > total) continue;
      const at = streamAt(stream, head);
      const needed = Math.max(exposure, detail.minStreakM / Math.max(at.speedMS, FLOW_MIN_SPEED_MS));
      const tail = streamAt(stream, Math.max(0, head - needed));
      const points: Vec3[] = [tail.at];
      for (let i = tail.index + 1; i <= at.index; i++) points.push(stream.points[i]!);
      points.push(at.at);
      const fade = Math.min(1, head / (0.1 * total + 1e-9), (total - head) / (0.2 * total + 1e-9));
      streaks.push({ stream: index, role: stream.role, points, speedMS: at.speedMS, excessK: at.excessK, vapourExcessKgPerKg: at.vapourExcessKgPerKg, condensateKgPerKg: at.condensateKgPerKg, opacity: Math.max(0, fade) });
    }
    return { stream: index, role: stream.role, emitter: stream.emitter, points: stream.points, speedMS: stream.speedMS, excessK: stream.excessK, vapourExcessKgPerKg: stream.vapourExcessKgPerKg, condensateKgPerKg: stream.condensateKgPerKg, arrows };
  });
  return { timeS, ribbons, streaks, puffs };
}
