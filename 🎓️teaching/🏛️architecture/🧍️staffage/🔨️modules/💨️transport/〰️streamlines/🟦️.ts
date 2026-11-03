/** 〰️ Instantaneous streamlines of the transport velocity field at one time, distinct from the pathlines (parcel histories).
 *
 * Velocity field at a point: `u(x) = u_wind(y) + u_jet(x) + u_sink(x)`.
 * - `u_jet`: Gaussian-weighted mean of the parcel velocities relative to the wind, `Σ w_p u_p / Σ w_p · min(1, Σ w_p)` with `w_p = exp(−r²/(2 b_p²))` and `b_p` the parcel radius, so inside a plume or jet it is the local mean flow and it vanishes outside.
 * - `u_sink`: during inhalation a point sink of the inhaled flow at each nostril or mouth, `Q/(4π r²)` towards the opening (r ≥ one diameter); flow visualisation only, parcels do not feel it.
 * All of it is three-dimensional (ahead x, up y, left z); the views only project the traced points. Seeds: around each inhalation sink six points at four diameters (±x, ±y, ±z); per active emitter a rake of 3 stations (0, 2 and 5 diameters along the outflow direction, at most 1.5 m) × 5 transverse points (the axis and ±(0.4 d + 0.15 s) along two perpendicular directions, one in the vertical plane through the outflow and one horizontal, so the spread shows in the plan as well as in the side view); with wind above 0.05 m/s five points on a vertical rake 0.5 m upwind (along the wind bearing) of the most upwind source.
 * Each line is traced both ways by Heun steps of 2 cm arc length (at most 120 per side, 2.4 m), stopping at speed < 5 mm/s, the ground or an inhalation opening.
 *
 * @see ../../🔗️contracts/🟦️.ts — `Streamline`
 * @see https://doi.org/10.1017/CBO9780511800955 — Pope 2000, Turbulent Flows (jet mean flow)
 */
import type { Emitter } from "../../../🧬️schema/🟦️.ts";
import type { EmitterState, Parcel, Streamline, Vec3 } from "../../🔗️contracts/🟦️.ts";
import type { Ambient } from "../🌬️ambient/🟦️.ts";

/** 〰️ Arc-length step of a streamline (m). */
export const STREAMLINE_STEP_M = 0.02;
const MAX_POINTS_PER_SIDE = 120;
const MIN_SPEED_MS = 0.005;

/** 👃️ An inhalation sink: opening position, diameter and inhaled flow. */
export type Sink = { readonly at: Vec3; readonly diameterM: number; readonly flowM3S: number };

const add = (a: Vec3, b: Vec3, s = 1): Vec3 => ({ x: a.x + b.x * s, y: a.y + b.y * s, z: a.z + b.z * s });
const norm = (a: Vec3): number => Math.hypot(a.x, a.y, a.z);

function field(at: Vec3, ambient: Ambient, parcels: readonly Parcel[], sinks: readonly Sink[]): Vec3 {
  const wind = ambient.windAt(Math.max(0, at.y));
  let sw = 0;
  let sx = 0;
  let sy = 0;
  let sz = 0;
  for (const p of parcels) {
    const dx = at.x - p.at.x;
    const dy = at.y - p.at.y;
    const dz = at.z - p.at.z;
    const r2 = dx * dx + dy * dy + dz * dz;
    const b = Math.max(p.radiusM, 1e-3);
    if (r2 > 16 * b * b) continue;
    const w = Math.exp(-r2 / (2 * b * b));
    const pw = ambient.windAt(Math.max(0, p.at.y));
    sw += w;
    sx += w * (p.velocity.x - pw.x);
    sy += w * (p.velocity.y - pw.y);
    sz += w * (p.velocity.z - pw.z);
  }
  let u: Vec3 = wind;
  if (sw > 0) {
    const gain = Math.min(1, sw) / sw;
    u = add(u, { x: sx, y: sy, z: sz }, gain);
  }
  for (const s of sinks) {
    const d = { x: s.at.x - at.x, y: s.at.y - at.y, z: s.at.z - at.z };
    const distance = norm(d);
    const r = Math.max(distance, s.diameterM);
    u = add(u, d, s.flowM3S / (4 * Math.PI * r * r) / (distance || 1));
  }
  return u;
}

function trace(seed: Vec3, sign: 1 | -1, ambient: Ambient, parcels: readonly Parcel[], sinks: readonly Sink[]): { readonly points: Vec3[]; readonly speeds: number[] } {
  const points: Vec3[] = [];
  const speeds: number[] = [];
  let p = seed;
  for (let i = 0; i < MAX_POINTS_PER_SIDE; i++) {
    const a = field(p, ambient, parcels, sinks);
    const sa = norm(a);
    if (sa < MIN_SPEED_MS) break;
    const mid = add(p, a, (sign * STREAMLINE_STEP_M) / (2 * sa));
    const b = field(mid, ambient, parcels, sinks);
    const sb = norm(b);
    if (sb < MIN_SPEED_MS) break;
    p = add(p, b, (sign * STREAMLINE_STEP_M) / sb);
    if (p.y < 0 || sinks.some((s) => norm({ x: s.at.x - p.x, y: s.at.y - p.y, z: s.at.z - p.z }) < Math.max(s.diameterM, STREAMLINE_STEP_M))) break;
    points.push(p);
    speeds.push(sb);
  }
  return { points, speeds };
}

/** 🧭️ The two unit directions perpendicular to an outflow direction: one in the vertical plane through it (`up`), one horizontal (`side`); a vertical outflow takes the lateral axis and the forward axis. */
function transverse(d: Vec3): { readonly up: Vec3; readonly side: Vec3 } {
  const horizontal = Math.hypot(d.x, d.z);
  if (horizontal < 1e-9) return { up: { x: 1, y: 0, z: 0 }, side: { x: 0, y: 0, z: 1 } };
  const side = { x: -d.z / horizontal, y: 0, z: d.x / horizontal };
  const up = { x: d.y * (d.x / horizontal) * -1, y: horizontal, z: d.y * (d.z / horizontal) * -1 };
  const l = norm(up);
  return { up: { x: up.x / l, y: up.y / l, z: up.z / l }, side };
}

/** 🌱️ The documented seed points near the active emitters and upwind. */
export function streamlineSeeds(emitters: readonly EmitterState[], definitions: readonly Emitter[], ambient: Ambient, sinks: readonly Sink[]): readonly Vec3[] {
  const seeds: Vec3[] = [];
  const active = emitters.filter((e) => e.active);
  active.forEach((state) => {
    const d = definitions.find((e) => e.id === state.emitter)?.diameterM ?? 0.02;
    const { up, side } = transverse(state.direction);
    for (const station of [0, 2 * d, 5 * d]) {
      const distance = Math.min(station, 1.5);
      const lateral = 0.4 * d + 0.15 * distance;
      const centre = add(state.at, state.direction, distance);
      seeds.push(centre, add(centre, up, lateral), add(centre, up, -lateral), add(centre, side, lateral), add(centre, side, -lateral));
    }
  });
  for (const sink of sinks) for (const axis of [{ x: 1, y: 0, z: 0 }, { x: -1, y: 0, z: 0 }, { x: 0, y: 1, z: 0 }, { x: 0, y: -1, z: 0 }, { x: 0, y: 0, z: 1 }, { x: 0, y: 0, z: -1 }]) seeds.push(add(sink.at, axis, 4 * sink.diameterM));
  if (ambient.environment.windMS > 0.05 && emitters.length > 0) {
    const phi = (ambient.environment.windDirectionDeg * Math.PI) / 180;
    const h = { x: Math.cos(phi), y: 0, z: Math.sin(phi) };
    const along = (e: EmitterState): number => e.at.x * h.x + e.at.z * h.z;
    const source = emitters.reduce((best, e) => (along(e) < along(best) ? e : best));
    const base = add(source.at, h, -0.5);
    const top = Math.max(...emitters.map((e) => e.at.y)) + 0.3;
    for (let i = 0; i < 5; i++) seeds.push({ x: base.x, y: 0.1 + ((top - 0.1) * i) / 4, z: base.z });
  }
  return seeds;
}

/** 〰️ The streamlines through the seeds at the instant: backward trace reversed, the seed, forward trace. */
export function streamlinesAt(seeds: readonly Vec3[], ambient: Ambient, parcels: readonly Parcel[], sinks: readonly Sink[]): readonly Streamline[] {
  const lines: Streamline[] = [];
  for (const seed of seeds) {
    const back = trace(seed, -1, ambient, parcels, sinks);
    const forward = trace(seed, 1, ambient, parcels, sinks);
    const at = field(seed, ambient, parcels, sinks);
    const points = [...back.points.reverse(), seed, ...forward.points];
    if (points.length < 2) continue;
    lines.push({ seed, points, speedMS: [...back.speeds.reverse(), norm(at), ...forward.speeds] });
  }
  return lines;
}
