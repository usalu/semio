/** 💨️ Air transport of the staffage archive: world-space Lagrangian parcels released by category-specific sources, pathlines, instantaneous streamlines and modelled visible condensation.
 *
 * `solveTransport(asset, query, environment, options)` returns the {@link TransportField} at the query's absolute time. The result is a pure function of that time: parcels are born on a global
 * emission grid at the emitter's world position, direction and speed of the instant they are born (the asset is evaluated at that time), then integrated on a global 1/120 s step grid with
 * ambient wind from the environment timeline, their own decaying jet momentum, buoyancy and entrainment; once emitted a parcel never moves with the emitter again, never resets at a clip loop
 * boundary, and playing at any frame rate or scrubbing yields bit-identical parcels (the cache only skips repeated work). It is an integral-model illustration, not CFD.
 *
 * Models and sources: see `🧮️parcels` (parcel equations), `🔥️sources` (breath, exhaust/vent/flue/fan, plume, transpiration), `🫁️respiration`, `〰️streamlines`, `🌬️ambient`, `🌫️air`.
 *
 * @see ../../🧬️schema/🟦️.ts — `Emitter`, `Respiration`
 * @see ../🔗️contracts/🟦️.ts — `TransportField`
 * @see ./🧮️parcels/🟦️.ts — the parcel model and its literature
 */
import type { Asset, Environment, Text } from "../../🧬️schema/🟦️.ts";
import type { EnvironmentTimeline, EvaluatedState, EvaluationQuery, JobControl, Parcel, ThermalField, TransportField } from "../🔗️contracts/🟦️.ts";
import { evaluate } from "../🧮️evaluation/🟦️.ts";
import { solveThermal } from "../🌡️thermal/🟦️.ts";
import { type BoundaryLayer, type Turbulence } from "./🌬️ambient/🟦️.ts";
import { ambientAt } from "./🌬️ambient/🟦️.ts";
import { type Resolved, type Track, createTrack, parcelsAt } from "./🧮️parcels/🟦️.ts";
import { STEPS_PER_S, emitterStates } from "./🔥️sources/🟦️.ts";
import { breathPatternAt, respirationState, respirationSubjects, routeShares } from "./🫁️respiration/🟦️.ts";
import { type Sink, streamlineSeeds, streamlinesAt } from "./〰️streamlines/🟦️.ts";
import { queryKey } from "./🫁️respiration/🟦️.ts";

export { breathAt, breathPatternAt, exhaledAir, exhaledVolumeM3, respirationOffsets, respirationState, respirationSubjects, respiratoryHeat, routeShares, emitterOwner } from "./🫁️respiration/🟦️.ts";
export { environmentAt } from "./🌬️ambient/🟦️.ts";
export { STEPS_PER_S } from "./🔥️sources/🟦️.ts";

/** 🎛️ Options of {@link solveTransport}. Defaults: parcels live 30 s, seed 1, uniform wind, turbulence intensity 0.1 with a 0.03 m/s floor and T_L = 1 s, pathlines and streamlines on.
 * `evaluate` and `thermal` replace the asset evaluation and the thermal solver (the plume and transpiration strengths come from `thermal`). */
export type TransportOptions = JobControl & {
  readonly maxAgeS?: number;
  readonly seed?: number;
  readonly boundaryLayer?: BoundaryLayer;
  readonly turbulence?: Turbulence;
  readonly pathlines?: boolean;
  readonly streamlines?: boolean;
  readonly evaluate?: (asset: Asset, query: EvaluationQuery) => EvaluatedState;
  readonly thermal?: (state: EvaluatedState, environment: Environment) => ThermalField;
};

/** 🎛️ The default turbulence: σ_u = max(0.1·wind, 0.03 m/s) with a 1 s Lagrangian time scale (illustrative). */
export const DEFAULT_TURBULENCE: Turbulence = { intensity: 0.1, floorMS: 0.03, lagrangianTimeS: 1 };

const TRACKS = new WeakMap<Asset, Map<string, Track>>();
const IDS = new WeakMap<object, number>();
const ENVIRONMENTS = new WeakMap<object, string>();

function identity(f: object | undefined): number {
  if (f === undefined) return 0;
  let id = IDS.get(f);
  if (id === undefined) IDS.set(f, (id = IDS_NEXT++));
  return id;
}
let IDS_NEXT = 1;

function environmentKey(environment: Environment | EnvironmentTimeline): string {
  let key = ENVIRONMENTS.get(environment);
  if (key === undefined) ENVIRONMENTS.set(environment, (key = JSON.stringify(environment)));
  return key;
}

const note = (en: string, de: string): Text => ({ en, de });

function trackOf(asset: Asset, query: EvaluationQuery, environment: Environment | EnvironmentTimeline, options: TransportOptions, settings: Resolved): Track {
  const key = [queryKey(query), environmentKey(environment), JSON.stringify(settings), identity(options.evaluate), identity(options.thermal)].join("|");
  let tracks = TRACKS.get(asset);
  if (tracks === undefined) TRACKS.set(asset, (tracks = new Map()));
  let track = tracks.get(key);
  if (track === undefined) {
    if (tracks.size >= 6) tracks.delete(tracks.keys().next().value as string);
    track = createTrack(asset, query, environment, settings, options.evaluate ?? evaluate, options.thermal ?? solveThermal);
    tracks.set(key, track);
  }
  return track;
}

/** 💨️ The transport field at the query's absolute time (`timeS`, 0 for poses without one) in the environment (constant or timeline). Cancellable and progress-reporting through `options`. */
export function solveTransport(asset: Asset, query: EvaluationQuery, environment: Environment | EnvironmentTimeline, options: TransportOptions = {}): TransportField {
  const timeS = Math.max(0, query.timeS ?? 0);
  const settings: Resolved = { maxAgeS: options.maxAgeS ?? 30, seed: options.seed ?? 1, layer: options.boundaryLayer ?? null, turbulence: options.turbulence ?? DEFAULT_TURBULENCE };
  const track = trackOf(asset, query, environment, options, settings);
  const label = note("Solving air transport", "Luftbewegung wird berechnet");
  const { parcels, pathlines } = parcelsAt(track, timeS, options.pathlines !== false, options.signal, (done, total) => options.progress?.(done, total, label));
  const emitters = emitterStates(track.world, timeS);
  const ambient = ambientAt(environment, timeS, settings.layer, settings.turbulence);
  const respiration = respirationState(asset, track.world.queryAt(timeS));
  const sinks: Sink[] = [];
  const now = track.world.queryAt(timeS);
  for (const { subject, respiration: resp } of respirationSubjects(asset)) {
    const rs = respirationState(asset, now, subject);
    const pattern = breathPatternAt(asset, now, subject);
    if (rs === null || pattern === null || rs.flowM3S >= 0) continue;
    const shares = routeShares(resp, pattern);
    emitters.forEach((state, i) => {
      const share = resp.noseEmitter === state.emitter ? shares.nose : resp.mouthEmitter === state.emitter ? shares.mouth : 0;
      if (share > 0) sinks.push({ at: state.at, diameterM: asset.thermal.emitters[i]!.diameterM, flowM3S: -rs.flowM3S * share });
    });
  }
  const streamlines = options.streamlines === false ? [] : streamlinesAt(streamlineSeeds(emitters, asset.thermal.emitters, ambient, sinks), ambient, parcels, sinks);
  const condensation = parcels.filter((p: Parcel) => p.condensateKgPerKg > 0).map((p) => ({ at: p.at, radiusM: p.radiusM, condensateKgPerKg: p.condensateKgPerKg }));
  return {
    timeS,
    emitters,
    parcels,
    pathlines,
    streamlines,
    condensation,
    respiration,
    model: { stepS: 1 / STEPS_PER_S, maxAgeS: settings.maxAgeS, seed: settings.seed },
    notes: [
      note(
        "Integral Lagrangian model: parcels are slices of jets and plumes entraining ambient air (Morton–Taylor–Turner), advected by the wind timeline and their own momentum and buoyancy; parcel positions are a pure function of absolute time. Not CFD, not validated against measurements.",
        "Integrales Lagrange-Modell: Teilchen sind Scheiben von Strahlen und Fahnen, die Umgebungsluft einmischen (Morton–Taylor–Turner), vom Windverlauf sowie eigenem Impuls und Auftrieb getragen; die Teilchenorte sind eine reine Funktion der absoluten Zeit. Keine CFD, nicht an Messungen validiert.",
      ),
      note(
        "Condensation is modelled: parcels whose mixed state is supersaturated (mixing line of the exhaled air with the ambient air); it is not the analytical parcel field.",
        "Kondensation ist modelliert: Teilchen, deren Mischzustand übersättigt ist (Mischgerade der Ausatemluft mit der Umgebungsluft); sie ist nicht das analytische Teilchenfeld.",
      ),
    ],
  };
}
