/** 🫁️ Respiration: a continuous breathing phase over the absolute timeline, half-sine flow, lung volume, nose/mouth routing, chest and abdomen motion and respiratory heat.
 *
 * Breaths are consecutive: breath `k` starts at `t_k` (breath 0 at t = 0, a clean inhalation start), lasts `60 / rate` of the pattern requested at `t_k`, and ends at `t_{k+1}`.
 * The phase is `φ(t) = k + (t − t_k)/T_k`, the integral of the rate that was in effect, so it never jumps; a pattern change (pose binding, clip key, `breathing` step track)
 * takes effect at the next breath boundary, where the lung is at functional residual capacity, so lung volume is continuous too and every breath moves exactly its tidal volume.
 * Within a breath: inhalation, pause, exhalation, pause with the pattern's fractions; flow in each active phase is a half-sine `π V_T/(2 T_phase) · sin(π τ/T_phase)` (integral = V_T),
 * following the sinusoidal breathing flow reported by Gupta, Lin & Chen (2010). Lung volume above FRC is the running integral (inhale `V_T/2·(1 − cos)`, exhale `V_T/2·(1 + cos)`).
 *
 * Requested pattern: a pose uses its `thermal.breathing`; a clip uses its `breathing` step track, else the pattern bound by the key pose of the current segment, switching at the segment
 * midpoint; a transition switches from the first to the second pose at half its duration; a pose or key without a binding means the first pattern of the asset (the resting pattern by convention).
 * Looping clips wrap the requested pattern with the clip time while the breathing phase runs on the absolute timeline.
 *
 * Exhaled temperature and relative humidity are the asset's authored `Respiration` values (reference environment); no ambient dependence is modelled (Höppe 1981 is unverified here).
 * Passengers (`ThermalModel.passengers`) breathe independently: every function takes an optional `subject` (the passenger id; omitted = primary body); a passenger's pattern comes from `pose.thermal.passengers[id].breathing` or the clip track `breathing:<id>`.
 * Chest and abdomen motion is `motion.angleDeg`/`dy` times `volumeL` over the tidal volume of the first (resting) pattern.
 *
 * @see ../../../🧬️schema/🟦️.ts — `Respiration`, `BreathingPattern`
 * @see ../../🔗️contracts/🟦️.ts — `RespirationState`
 * @see https://doi.org/10.1111/j.1600-0668.2009.00608.x — Gupta, Lin, Chen 2010, Characterizing exhaled airflow from breathing and talking, Indoor Air 20:31–39
 * @see ../../🎞️animation/🟦️.ts — `wrapTime`, `effectiveKeys`
 */
import type { Asset, BreathingPattern, Environment, Respiration, Slug } from "../../../🧬️schema/🟦️.ts";
import type { EvaluationQuery, RespirationState } from "../../🔗️contracts/🟦️.ts";
import { clipOf, effectiveKeys, poseOf, wrapTime } from "../../🎞️animation/🟦️.ts";
import { enthalpy, humidityRatio, latentHeat, saturationPressurePa, volumePerDryKg } from "../🌫️air/🟦️.ts";

/** 🫁️ Share of the exhaled volume leaving through the nose when the route is `nose-and-mouth` (authored estimate). */
export const NOSE_SHARE_MIXED = 0.5;

type Breath = { readonly startS: number; readonly durationS: number; readonly pattern: BreathingPattern };

/** 🫁️ The breaths of one asset and query from t = 0, extended on demand; independent of the query time. */
type Ledger = { readonly respiration: Respiration; readonly request: (timeS: number) => BreathingPattern; readonly breaths: Breath[] };

const LEDGERS = new WeakMap<Asset, Map<string, Ledger>>();

/** 🧍️ The breathing subject of a pose or clip binding: `undefined` is the primary body of the asset, a slug a passenger (second living body, e.g. a toddler on a hip). */
export type Subject = Slug | undefined;

function respirationOf(asset: Asset, subject: Subject): Respiration | null {
  return subject === undefined ? (asset.thermal.respiration ?? null) : (asset.thermal.passengers?.find((p) => p.id === subject)?.respiration ?? null);
}

/** 🧍️ Every breathing subject of an asset with its respiration: the primary body first, then the passengers that breathe. */
export function respirationSubjects(asset: Asset): readonly { readonly subject: Subject; readonly respiration: Respiration }[] {
  const out: { subject: Subject; respiration: Respiration }[] = [];
  if (asset.thermal.respiration !== undefined) out.push({ subject: undefined, respiration: asset.thermal.respiration });
  for (const p of asset.thermal.passengers ?? []) if (p.respiration !== undefined) out.push({ subject: p.id, respiration: p.respiration });
  return out;
}

/** 🫁️ The subject whose respiration owns a breath emitter (its nose or mouth emitter), or `null`. */
export function emitterOwner(asset: Asset, emitterId: Slug): { readonly subject: Subject; readonly respiration: Respiration } | null {
  return respirationSubjects(asset).find((s) => s.respiration.noseEmitter === emitterId || s.respiration.mouthEmitter === emitterId) ?? null;
}

function boundPattern(pose: { readonly thermal: { readonly breathing?: Slug; readonly passengers?: Readonly<Record<Slug, { readonly breathing?: Slug }>> } }, subject: Subject): Slug | undefined {
  return subject === undefined ? pose.thermal.breathing : pose.thermal.passengers?.[subject]?.breathing;
}

function patternOf(respiration: Respiration, id: Slug | undefined): BreathingPattern {
  if (id === undefined) return respiration.patterns[0]!;
  const pattern = respiration.patterns.find((p) => p.id === id);
  if (pattern === undefined) throw new Error(`unknown breathing pattern "${id}"; patterns: ${respiration.patterns.map((p) => p.id).join(", ")}`);
  return pattern;
}

function validate(pattern: BreathingPattern): void {
  const sum = pattern.inhaleFraction + pattern.pauseAfterInhaleFraction + pattern.exhaleFraction + pattern.pauseAfterExhaleFraction;
  if (!(pattern.rateBpm > 0) || !(pattern.tidalVolumeL > 0) || !(pattern.inhaleFraction > 0) || !(pattern.exhaleFraction > 0) || Math.abs(sum - 1) > 1e-6) throw new RangeError(`breathing pattern "${pattern.id}" needs positive rate, volume, inhale and exhale fractions and fractions summing to 1 (sum ${sum})`);
}

function clipRequest(asset: Asset, respiration: Respiration, subject: Subject, clipId: Slug, timeS: number): BreathingPattern {
  const clip = clipOf(asset, clipId);
  const tw = wrapTime(timeS, clip.durationS, clip.loop);
  const track = clip.tracks.find((t) => t.target === (subject === undefined ? "breathing" : `breathing:${subject}`));
  if (track !== undefined && track.keys.length > 0) {
    let v = track.keys[0]!.v;
    for (const key of track.keys) {
      if (key.t > tw) break;
      v = key.v;
    }
    const pattern = respiration.patterns[Math.round(v)];
    if (pattern === undefined) throw new RangeError(`breathing track index ${v} outside the ${respiration.patterns.length} patterns of "${asset.id}"`);
    return pattern;
  }
  const keys = effectiveKeys(clip.keys, clip.loop, clip.durationS);
  const bound = (index: number): BreathingPattern => patternOf(respiration, boundPattern(poseOf(asset, keys[index]!.pose), subject));
  let i = -1;
  keys.forEach((key, j) => {
    if (key.t <= tw) i = j;
  });
  if (!clip.loop) return bound(Math.max(i, 0));
  const last = keys.length - 1;
  const [from, t] = i < 0 ? [last, tw + clip.durationS] : [i, tw];
  const start = keys[from]!.t - (i < 0 ? clip.durationS : 0);
  const end = from === last ? keys[0]!.t + clip.durationS : keys[from + 1]!.t;
  return t < (start + end) / 2 ? bound(from) : bound(from === last ? 0 : from + 1);
}

function requestFor(asset: Asset, respiration: Respiration, subject: Subject, query: EvaluationQuery): (timeS: number) => BreathingPattern {
  if (query.kind === "pose") {
    const pattern = patternOf(respiration, boundPattern(poseOf(asset, query.pose), subject));
    return () => pattern;
  }
  if (query.kind === "clip") return (t) => clipRequest(asset, respiration, subject, query.clip, t);
  const from = patternOf(respiration, boundPattern(poseOf(asset, query.from), subject));
  const to = patternOf(respiration, boundPattern(poseOf(asset, query.to), subject));
  return (t) => (t < query.durationS / 2 ? from : to);
}

/** 🔑️ A stable key of a query without its time. */
export function queryKey(query: EvaluationQuery): string {
  return query.kind === "pose" ? `pose:${query.pose}` : query.kind === "clip" ? `clip:${query.clip}` : `transition:${query.from}>${query.to}:${query.durationS}`;
}

function ledgerOf(asset: Asset, query: EvaluationQuery, subject: Subject): Ledger | null {
  const respiration = respirationOf(asset, subject);
  if (respiration === null || respiration.patterns.length === 0) return null;
  let byQuery = LEDGERS.get(asset);
  if (byQuery === undefined) LEDGERS.set(asset, (byQuery = new Map()));
  const key = `${queryKey(query)}#${subject ?? ""}`;
  let ledger = byQuery.get(key);
  if (ledger === undefined) {
    if (byQuery.size > 32) byQuery.clear();
    ledger = { respiration, request: requestFor(asset, respiration, subject, query), breaths: [] };
    byQuery.set(key, ledger);
  }
  return ledger;
}

function extend(ledger: Ledger, timeS: number): void {
  const b = ledger.breaths;
  while (b.length === 0 || b[b.length - 1]!.startS + b[b.length - 1]!.durationS <= timeS) {
    const startS = b.length === 0 ? 0 : b[b.length - 1]!.startS + b[b.length - 1]!.durationS;
    const pattern = ledger.request(startS);
    validate(pattern);
    b.push({ startS, durationS: 60 / pattern.rateBpm, pattern });
  }
}

function indexAt(ledger: Ledger, timeS: number): number {
  extend(ledger, timeS);
  let lo = 0;
  let hi = ledger.breaths.length - 1;
  while (lo < hi) {
    const mid = (lo + hi + 1) >> 1;
    if (ledger.breaths[mid]!.startS <= timeS) lo = mid;
    else hi = mid - 1;
  }
  return lo;
}

/** 🫁️ The flow (m³/s, + = exhaling) and lung volume above FRC (L) of one breath at the fraction `tau` of its cycle. */
export function breathAt(pattern: BreathingPattern, durationS: number, tau: number): { readonly phase: RespirationState["phase"]; readonly flowM3S: number; readonly volumeL: number } {
  const v = pattern.tidalVolumeL;
  const c1 = pattern.inhaleFraction;
  const c2 = c1 + pattern.pauseAfterInhaleFraction;
  const c3 = c2 + pattern.exhaleFraction;
  if (tau < c1) {
    const x = tau / c1;
    return { phase: "inhale", flowM3S: -((Math.PI * v) / 1000 / (2 * c1 * durationS)) * Math.sin(Math.PI * x), volumeL: (v / 2) * (1 - Math.cos(Math.PI * x)) };
  }
  if (tau < c2) return { phase: "pause-after-inhale", flowM3S: 0, volumeL: v };
  if (tau < c3) {
    const x = (tau - c2) / pattern.exhaleFraction;
    return { phase: "exhale", flowM3S: ((Math.PI * v) / 1000 / (2 * pattern.exhaleFraction * durationS)) * Math.sin(Math.PI * x), volumeL: (v / 2) * (1 + Math.cos(Math.PI * x)) };
  }
  return { phase: "pause-after-exhale", flowM3S: 0, volumeL: 0 };
}

/** 🫁️ The respiratory state at the query's absolute time (`timeS`, default 0 for poses); `null` for assets without respiration. */
export function respirationState(asset: Asset, query: EvaluationQuery, subject?: Slug): RespirationState | null {
  const ledger = ledgerOf(asset, query, subject);
  if (ledger === null) return null;
  const t = Math.max(0, query.timeS ?? 0);
  const k = indexAt(ledger, t);
  const breath = ledger.breaths[k]!;
  const tau = (t - breath.startS) / breath.durationS;
  const now = breathAt(breath.pattern, breath.durationS, tau);
  return { pattern: breath.pattern.id, cycles: k + tau, phase: now.phase, flowM3S: now.flowM3S, volumeL: now.volumeL, rateBpm: breath.pattern.rateBpm, tidalVolumeL: breath.pattern.tidalVolumeL };
}

/** 🫁️ The volume (m³) exhaled between two absolute times: the exact integral of the positive half-sine flow, so the volumes of consecutive intervals add up to the tidal volumes. */
export function exhaledVolumeM3(asset: Asset, query: EvaluationQuery, fromS: number, toS: number, subject?: Slug): number {
  const ledger = ledgerOf(asset, query, subject);
  if (ledger === null || toS <= fromS) return 0;
  const a = Math.max(0, fromS);
  let k = indexAt(ledger, a);
  indexAt(ledger, toS);
  let total = 0;
  for (; k < ledger.breaths.length && ledger.breaths[k]!.startS < toS; k++) {
    const { startS, durationS, pattern } = ledger.breaths[k]!;
    const begin = startS + durationS * (pattern.inhaleFraction + pattern.pauseAfterInhaleFraction);
    const length = durationS * pattern.exhaleFraction;
    const cumulative = (t: number): number => {
      const x = Math.min(1, Math.max(0, (t - begin) / length));
      return (pattern.tidalVolumeL / 1000 / 2) * (1 - Math.cos(Math.PI * x));
    };
    total += cumulative(Math.min(toS, begin + length)) - cumulative(Math.max(a, begin));
  }
  return Math.max(0, total);
}

/** 🫁️ The shares of the exhaled volume through the nose and the mouth for a pattern; a route whose emitter the asset lacks is carried by the other. */
export function routeShares(respiration: Respiration, pattern: BreathingPattern): { readonly nose: number; readonly mouth: number } {
  const hasNose = respiration.noseEmitter !== undefined;
  const hasMouth = respiration.mouthEmitter !== undefined;
  const wanted = pattern.route === "nose" ? 1 : pattern.route === "mouth" ? 0 : NOSE_SHARE_MIXED;
  if (hasNose && hasMouth) return { nose: wanted, mouth: 1 - wanted };
  return hasNose ? { nose: 1, mouth: 0 } : hasMouth ? { nose: 0, mouth: 1 } : { nose: 0, mouth: 0 };
}

/** 🫁️ The pattern of the breath in effect at the query's time, for routing. */
export function breathPatternAt(asset: Asset, query: EvaluationQuery, subject?: Slug): BreathingPattern | null {
  const ledger = ledgerOf(asset, query, subject);
  return ledger === null ? null : ledger.breaths[indexAt(ledger, Math.max(0, query.timeS ?? 0))]!.pattern;
}

/** 🫁️ Additive bone offsets (`angleDeg`, `dy` in the parent frame) of the chest and abdomen for a respiratory state: the authored motion scaled by the lung volume over the tidal volume of the first pattern. */
export function respirationOffsets(asset: Asset, state: RespirationState, subject?: Slug): Readonly<Record<Slug, { readonly angleDeg: number; readonly dy: number }>> {
  const respiration = respirationOf(asset, subject);
  const out: Record<Slug, { angleDeg: number; dy: number }> = {};
  if (respiration === null || respiration.patterns.length === 0) return out;
  const scale = state.volumeL / respiration.patterns[0]!.tidalVolumeL;
  for (const m of respiration.motion) {
    const previous = out[m.bone] ?? { angleDeg: 0, dy: 0 };
    out[m.bone] = { angleDeg: previous.angleDeg + m.angleDeg * scale, dy: previous.dy + m.dy * scale };
  }
  return out;
}

/** 🫁️ The exhaled air of an asset: temperature and humidity ratio (kg/kg dry air) at a pressure. */
export function exhaledAir(respiration: Respiration, pressurePa: number): { readonly tC: number; readonly wv: number } {
  return { tC: respiration.exhaledTempC, wv: humidityRatio(respiration.exhaledRelativeHumidity * saturationPressurePa(respiration.exhaledTempC), pressurePa) };
}

/** 🔥️ The cycle-mean respiratory heat release of a pattern into an environment: the enthalpy that warms and humidifies the inhaled air to the exhaled state, split into latent (`Δw·h_fg(T_ex)`) and sensible; consistent in magnitude with the ASHRAE regression `0.0014 M (34 − T_a)` and `0.0173 M (5.87 − p_a)` that the thermal solver reports. */
export function respiratoryHeat(respiration: Respiration, pattern: BreathingPattern, environment: Environment): { readonly dryAirKgS: number; readonly sensibleW: number; readonly latentW: number } {
  const exhaled = exhaledAir(respiration, environment.pressurePa);
  const wa = humidityRatio(environment.relativeHumidity * saturationPressurePa(environment.airTempC), environment.pressurePa);
  const dryAirKgS = ((pattern.rateBpm / 60) * (pattern.tidalVolumeL / 1000)) / volumePerDryKg(exhaled.tC, exhaled.wv, environment.pressurePa);
  const total = dryAirKgS * (enthalpy(exhaled.tC, exhaled.wv, 0) - enthalpy(environment.airTempC, wa, 0));
  const latentW = dryAirKgS * (exhaled.wv - wa) * latentHeat(exhaled.tC);
  return { dryAirKgS, sensibleW: total - latentW, latentW };
}
