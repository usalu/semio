/** 〰️ Instantaneous streamlines of the transport velocity field at one time, distinct from the pathlines (parcel histories).
 *
 * Velocity field at a point: `u(x) = u_wind(y) + u_jet(x) + u_sink(x)`.
 * - `u_jet`: Gaussian-weighted mean of the parcel velocities relative to the wind, `Σ w_p u_p / Σ w_p · min(1, Σ w_p)` with `w_p = exp(−r²/(2 b_p²))` and `b_p` the parcel radius, so inside a plume or jet it is the local mean flow and it vanishes outside.
 * - `u_sink`: during inhalation a point sink of the inhaled flow at each nostril or mouth, `Q/(4π r²)` towards the opening (r ≥ one diameter); flow visualisation only, parcels do not feel it.
 * Seeds: around each inhalation sink four points at four diameters; per active emitter a rake of 3 stations (0, 2 and 5 diameters, at most 1.5 m) × 3 transverse points (0, ±(0.4 d + 0.15 s)); with wind above 0.05 m/s five points on a vertical rake 0.5 m upwind of the leftmost or rightmost source.
 * Each line is traced both ways by Heun steps of 2 cm arc length (at most 120 per side, 2.4 m), stopping at speed < 5 mm/s, the ground or an inhalation opening.
 *
 * @see ../../🔗️contracts/🟦️.ts — `Streamline`
 * @see https://doi.org/10.1017/CBO9780511800955 — Pope 2000, Turbulent Flows (jet mean flow)
 */
import type { Emitter } from "../../../🧬️schema/🟦️.ts";
import type { EmitterState, Parcel, Point, Streamline } from "../../🔗️contracts/🟦️.ts";
import type { Ambient } from "../🌬️ambient/🟦️.ts";

/** 〰️ Arc-length step of a streamline (m). */
export const STREAMLINE_STEP_M = 0.02;
const MAX_POINTS_PER_SIDE = 120;
const MIN_SPEED_MS = 0.005;

/** 👃️ An inhalation sink: opening position, diameter and inhaled flow. */
export type Sink = { readonly at: Point; readonly diameterM: number; readonly flowM3S: number };

function field(at: Point, ambient: Ambient, parcels: readonly Parcel[], sinks: readonly Sink[]): Point {
  const wind = ambient.windAt(Math.max(0, at.y));
  let sw = 0;
  let sx = 0;
  let sy = 0;
  for (const p of parcels) {
    const dx = at.x - p.at.x;
    const dy = at.y - p.at.y;
    const r2 = dx * dx + dy * dy;
    const b = Math.max(p.radiusM, 1e-3);
    if (r2 > 16 * b * b) continue;
    const w = Math.exp(-r2 / (2 * b * b));
    const pw = ambient.windAt(Math.max(0, p.at.y));
    sw += w;
    sx += w * (p.velocity.x - pw.x);
    sy += w * (p.velocity.y - pw.y);
  }
  let x = wind.x;
  let y = wind.y;
  if (sw > 0) {
    const gain = Math.min(1, sw) / sw;
    x += sx * gain;
    y += sy * gain;
  }
  for (const s of sinks) {
    const dx = s.at.x - at.x;
    const dy = s.at.y - at.y;
    const r = Math.max(Math.hypot(dx, dy), s.diameterM);
    const speed = s.flowM3S / (4 * Math.PI * r * r);
    const l = Math.hypot(dx, dy) || 1;
    x += (speed * dx) / l;
    y += (speed * dy) / l;
  }
  return { x, y };
}

function trace(seed: Point, sign: 1 | -1, ambient: Ambient, parcels: readonly Parcel[], sinks: readonly Sink[]): { readonly points: Point[]; readonly speeds: number[] } {
  const points: Point[] = [];
  const speeds: number[] = [];
  let p = seed;
  for (let i = 0; i < MAX_POINTS_PER_SIDE; i++) {
    const a = field(p, ambient, parcels, sinks);
    const sa = Math.hypot(a.x, a.y);
    if (sa < MIN_SPEED_MS) break;
    const mid = { x: p.x + (sign * a.x * STREAMLINE_STEP_M) / (2 * sa), y: p.y + (sign * a.y * STREAMLINE_STEP_M) / (2 * sa) };
    const b = field(mid, ambient, parcels, sinks);
    const sb = Math.hypot(b.x, b.y);
    if (sb < MIN_SPEED_MS) break;
    p = { x: p.x + (sign * b.x * STREAMLINE_STEP_M) / sb, y: p.y + (sign * b.y * STREAMLINE_STEP_M) / sb };
    if (p.y < 0 || sinks.some((s) => Math.hypot(s.at.x - p.x, s.at.y - p.y) < Math.max(s.diameterM, STREAMLINE_STEP_M))) break;
    points.push(p);
    speeds.push(sb);
  }
  return { points, speeds };
}

/** 🌱️ The documented seed points near the active emitters and upwind. */
export function streamlineSeeds(emitters: readonly EmitterState[], definitions: readonly Emitter[], ambient: Ambient, sinks: readonly Sink[]): readonly Point[] {
  const seeds: Point[] = [];
  const active = emitters.filter((e) => e.active);
  active.forEach((state) => {
    const d = definitions.find((e) => e.id === state.emitter)?.diameterM ?? 0.02;
    const c = Math.cos(state.directionRad);
    const s = Math.sin(state.directionRad);
    for (const station of [0, 2 * d, 5 * d]) {
      const distance = Math.min(station, 1.5);
      const lateral = 0.4 * d + 0.15 * distance;
      for (const offset of [0, lateral, -lateral]) seeds.push({ x: state.at.x + c * distance - s * offset, y: state.at.y + s * distance + c * offset });
    }
  });
  for (const sink of sinks) for (let i = 0; i < 4; i++) seeds.push({ x: sink.at.x + 4 * sink.diameterM * Math.cos((i * Math.PI) / 2), y: sink.at.y + 4 * sink.diameterM * Math.sin((i * Math.PI) / 2) });
  if (ambient.environment.windMS > 0.05 && emitters.length > 0) {
    const phi = (ambient.environment.windDirectionDeg * Math.PI) / 180;
    const upwindLeft = Math.cos(phi) >= 0;
    const xs = emitters.map((e) => e.at.x);
    const x = (upwindLeft ? Math.min(...xs) : Math.max(...xs)) - Math.sign(Math.cos(phi) || 1) * 0.5;
    const top = Math.max(...emitters.map((e) => e.at.y)) + 0.3;
    for (let i = 0; i < 5; i++) seeds.push({ x, y: 0.1 + ((top - 0.1) * i) / 4 });
  }
  return seeds;
}

/** 〰️ The streamlines through the seeds at the instant: backward trace reversed, the seed, forward trace. */
export function streamlinesAt(seeds: readonly Point[], ambient: Ambient, parcels: readonly Parcel[], sinks: readonly Sink[]): readonly Streamline[] {
  const lines: Streamline[] = [];
  for (const seed of seeds) {
    const back = trace(seed, -1, ambient, parcels, sinks);
    const forward = trace(seed, 1, ambient, parcels, sinks);
    const at = field(seed, ambient, parcels, sinks);
    const points = [...back.points.reverse(), seed, ...forward.points];
    if (points.length < 2) continue;
    lines.push({ seed, points, speedMS: [...back.speeds.reverse(), Math.hypot(at.x, at.y), ...forward.speeds] });
  }
  return lines;
}
