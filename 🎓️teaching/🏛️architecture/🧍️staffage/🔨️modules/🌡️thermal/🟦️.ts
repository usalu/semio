/** 🌡️ Thermal solver of the staffage archive: explicit physical quantities per region (surface and skin temperature, emitted and net radiation, sensible, latent
 * and moisture flux) from energy balances, before any colour.
 *
 * - Outer surface: radiation against sky, ground and surroundings, absorbed solar by sunlit share, convection by geometry (natural and forced, relative wind incl. body
 *   motion), evaporation, contact conduction, layers in series from the skin, the interior or the heat source (🧱️balance, 🔬️physics).
 * - Bodies: Gagge-type two-node thermoregulation distributed over the regions, shivering, sweating, panting, skin blood flow, respiration (🫀️physiology).
 * - Leaves: Penman–Monteith balance with light and vapour-pressure-deficit controlled stomata, CAM at night (🍃️canopy).
 * - Machines and building elements: released heat `heatW × level`, interior-to-exterior conduction through layers.
 * - Water-bearing surfaces: a `hydronic` region (radiator plate) conducts from the circuit water at its place along the flow (supply falling to return at the operating level), never above the supply; a `coupled` region (jug or pot wall)
 *   conducts from the water of its source region over the wetted share of its area, and the heat it draws is taken from that region's balance.
 * - Transient: first-order response with `C/G` per region, driven by the input history (⏳️transient); core temperature with the body's heat capacity.
 * - Phase change: a region whose material or own `phaseChange` declares a point (water: boiling at 100 °C, 2.257 MJ/kg) never exceeds it; the surplus power `inflow − outflow` there becomes converted mass (`phaseChangeKgS`, `evaporationKgS`) that vapour emitters follow.
 *
 * Documentation, sources, parameter classes and validation: ticket file `thermal-v2.md`. Deterministic in (state, environment, history); invariant under drawing scale.
 *
 * @see ../🔗️contracts/🟦️.ts — EvaluatedState, ThermalField
 * @see ../../🧬️schema/🟦️.ts — ThermalModel, ThermalRegion, Environment
 */
import { type Asset, type Environment, type Slug, type Text, type ThermalRegion, operatingKeyOf } from "../../🧬️schema/🟦️.ts";
import type { BodyField, EnvironmentTimeline, EvaluatedState, JobControl, RegionField, ThermalField, ThermalInputs, ThermalQuantity, Vec3 } from "../🔗️contracts/🟦️.ts";
import { type EnvironmentInput, environmentAt, isTimeline } from "./🌤️environment/🟦️.ts";
import { type BodySetup, type BodyState, bodySetup, evaluateBody, neutralState, skinConductances, steadyBody, stepBody } from "./🫀️physiology/🟦️.ts";
import { type Ambient, ENCLOSED_EXPOSURE, NO_DRAW, type RadiationSplit, type RegionFrame, type RegionSetup, ambientOf, circuitWaterC, emittedOf, envOf, exchange, inflowOf, phaseChangeWM2, radiationSplit as splitOf, regionSetup, residualWM2, restoringConductance, solarOf, solveSteady, stomataOf } from "./🧱️balance/🟦️.ts";
import { MET_WM2, latentHeatJkg, radiantEnvironmentC } from "./🔬️physics/🟦️.ts";
import { QUANTITY_SPECS } from "./📏️quantities/🟦️.ts";
import { historyPlan, relax } from "./⏳️transient/🟦️.ts";

export { environmentAt, isTimeline, type EnvironmentInput } from "./🌤️environment/🟦️.ts";
export { absorbedSolar, convection, emittedFlux, radiationCoefficient, radiationFlux, saturationPressurePa, specificHumidity, vapourPressurePa, humidityRatio, latentHeatJkg, airDensity, SIGMA } from "./🔬️physics/🟦️.ts";
export { QUANTITY_SPECS, type QuantitySpec, valueOf } from "./📏️quantities/🟦️.ts";
export { respiratoryHeat, type RespiratoryHeat } from "./🫀️physiology/🟦️.ts";
export { type PenmanMonteithInput, penmanMonteith, perGroundArea, stomatalConductance } from "./🍃️canopy/🟦️.ts";
export { type RadiationSplit } from "./🧱️balance/🟦️.ts";
export { radiantEnvironmentC, radiantTemperatures, viewFactors } from "./🔬️physics/🟦️.ts";
export { type HistoryPlan, historyPlan, relax } from "./⏳️transient/🟦️.ts";
export { type TimelineOptions, solveThermalAt } from "./🕰️timeline/🟦️.ts";
export { CLO_M2KW, MET_WM2_ISO, type Comfort, type ComfortInputs, type Sensation, bodyComfort, clothingInsulationClo, cloToM2KW, comfortViolations, m2KWToClo, metToWM2, pmvPpd, relativeAirSpeedMS, sensationOf, vapourPressureIsoPa } from "./🛋️comfort/🟦️.ts";

/** 🧍️ The inputs of the passengers (second living bodies) at one instant; `ThermalInputs` of the contract does not carry them yet, the evaluator adds them. */
export type PassengerInputs = { readonly passengers?: Readonly<Record<Slug, { readonly activityMet: number; readonly breathing?: Slug | null }>> };

/** 🧾️ The scalar thermal inputs and the figure's centre-of-mass velocity at one absolute time of the input history. */
export type ThermalSample = { readonly inputs: ThermalInputs; readonly comVelocity?: Vec3 };

/** ⚙️ Solver options: `history` returns the inputs at any absolute time (the evaluator's thermal binding along the clip or pose timeline); without it the inputs of the
 * evaluated state are assumed to have held forever and the response is the steady state. `windowS` overrides the history window, `maxSteps` the step budget. */
export type ThermalOptions = Partial<JobControl> & { readonly history?: (timeS: number) => ThermalSample; readonly windowS?: number; readonly maxSteps?: number; readonly session?: ThermalSession };

export type Trajectory = { readonly temps: readonly number[]; readonly bodies: readonly BodyState[]; readonly phaseKg: readonly number[] };
export type Chain = { readonly start: Trajectory; readonly nodes: Trajectory[] };

/** 🕰️ The work the solves of one frame series share, so stepping through the frames of a clip or a timeline does not integrate the history again for every frame: the instants of the input history (one per absolute time) and the
 * integrations from one start (`startS`, grid step), whose states at the grid times every frame with that start and step passes through. Results are bit-identical to solving without a session. A session belongs to ONE asset, ONE
 * clip or pose, ONE environment and ONE `history` function; mixing them returns wrong temperatures. Bounded: the oldest instants and chains are dropped. Create with {@link thermalSession}. */
export type ThermalSession = { readonly instants: Map<number, Instant>; readonly chains: Map<string, Chain> };

const SESSION_INSTANTS = 1024;
const SESSION_CHAINS = 3;

/** 🆕️ A fresh, empty {@link ThermalSession}. */
export const thermalSession = (): ThermalSession => ({ instants: new Map(), chains: new Map() });

function remembered<K, V>(map: Map<K, V>, key: K, limit: number, make: () => V): V {
  const known = map.get(key);
  if (known !== undefined) return known;
  const made = make();
  if (map.size >= limit) map.delete(map.keys().next().value!);
  map.set(key, made);
  return made;
}

type Body = { readonly setup: BodySetup; readonly subject: Slug | null; readonly regions: readonly number[] };
type System = {
  readonly regions: readonly RegionSetup[];
  readonly bodies: readonly Body[];
  readonly place: ReadonlyMap<number, { readonly body: number; readonly slot: number }>;
  readonly source: readonly number[];
  readonly dependents: readonly (readonly number[])[];
  readonly coupled: boolean;
};

const SYSTEMS = new WeakMap<Asset, System>();

function systemOf(asset: Asset): System {
  const cached = SYSTEMS.get(asset);
  if (cached) return cached;
  const regions = asset.thermal.regions.map((r) => regionSetup(r, asset.materials));
  const bodies: Body[] = [];
  const place = new Map<number, { body: number; slot: number }>();
  for (const subject of [null, ...asset.thermal.passengers.map((p) => p.id)]) {
    const setup = bodySetup(asset, regions, subject);
    if (!setup) continue;
    const indices = setup.regions.map((br) => regions.indexOf(br.setup));
    indices.forEach((index, slot) => place.set(index, { body: bodies.length, slot }));
    bodies.push({ setup, subject, regions: indices });
  }
  const source = regions.map((s) => (s.region.coupled === undefined ? -1 : regions.findIndex((x) => x.region.id === s.region.coupled!.region)));
  const dependents = regions.map((_, j) => source.flatMap((k, d) => (k === j ? [d] : [])));
  const system = { regions, bodies, place, source, dependents, coupled: source.some((k) => k >= 0) };
  SYSTEMS.set(asset, system);
  return system;
}

/** 🔗️ The frames with their couplings resolved against the surface temperatures `temps`: a coupled region sees the temperature of its source region as its inside, and a source region carries the draw of its coupled regions as a function of its
 * own temperature (their conduction towards their present temperatures, W/m² of its own area), so the heat a wall takes from the water is missing from the water and its slope already damps the source's response. Without couplings the frames are returned as they are. */
function withCoupling(sys: System, frames: readonly RegionFrame[], temps: readonly number[]): readonly RegionFrame[] {
  if (!sys.coupled) return frames;
  return frames.map((f, j) => {
    const source = sys.source[j]!;
    if (source >= 0) return { ...f, sourceC: temps[source]! };
    const deps = sys.dependents[j]!;
    if (deps.length === 0) return f;
    const draw = (tsC: number): number => deps.reduce((sum, d) => sum + (frames[d]!.setup.region.areaM2 * inflowOf({ ...frames[d]!, sourceC: tsC }, 0)(temps[d]!)) / f.setup.region.areaM2, 0);
    return { ...f, draw };
  });
}

function subjectInputs(inputs: ThermalInputs, subject: Slug | null): ThermalInputs {
  if (subject === null) return inputs;
  const p = inputs.passengers?.[subject];
  return { ...inputs, activityMet: p?.activityMet ?? 0, externalWorkW: 0, breathing: p?.breathing ?? null };
}

export type Instant = { readonly env: Environment; readonly amb: Ambient; readonly inputs: ThermalInputs; readonly frames: readonly RegionFrame[]; readonly levels: readonly number[] };

function instantOf(sys: System, sample: ThermalSample, env: Environment): Instant {
  const amb = ambientOf(env);
  const theta = (env.windDirectionDeg * Math.PI) / 180;
  const c = sample.comVelocity ?? { x: 0, y: 0, z: 0 };
  const rel = Math.hypot(env.windMS * Math.cos(theta) - c.x, -c.y, env.windMS * Math.sin(theta) - c.z);
  const activities = sys.regions.map((s) => {
    const met = subjectInputs(sample.inputs, s.region.body?.subject ?? null).activityMet;
    return 0.0052 * Math.max(0, (met > 0 ? met : 1) * MET_WM2 - 58);
  });
  const levels: number[] = [];
  const frames = sys.regions.map((s, j): RegionFrame => {
    const r = s.region;
    const b = sample.inputs.regions[r.id];
    const sunlit = b?.sunlitFraction ?? 0;
    const key = operatingKeyOf(r);
    const level = key === null ? 0 : (sample.inputs.operating[key] ?? 0);
    levels.push(level);
    return {
      setup: s,
      exposed: b?.exposedFraction ?? 1,
      contact: b?.contact?.fraction ?? 0,
      contactTempC: b?.contact?.tempC ?? env.groundTempC ?? env.airTempC,
      wettedness: b?.wettedness ?? s.wetDefault,
      sunlit,
      speed: r.model === "body" ? Math.hypot(rel, activities[j]!) : rel,
      envC: envOf(s, env),
      solar: solarOf(s, env, sunlit),
      stomatalMS: stomataOf(s, env, sunlit),
      sourceC: r.hydronic === undefined ? env.airTempC : circuitWaterC(r.hydronic, level, env.airTempC),
      draw: NO_DRAW,
    };
  });
  return { env, amb, inputs: sample.inputs, frames, levels };
}

const bodyFrames = (b: Body, i: Instant): RegionFrame[] => b.regions.map((j) => i.frames[j]!);

type Node = { readonly target: number; readonly tau: number };

function steadyOf(f: RegionFrame, a: Ambient, level: number, guess: number): Node {
  const inflow = inflowOf(f, level);
  const point = f.setup.phase?.pointC;
  const solved = point !== undefined && residualWM2(f, a, inflow, point) >= 0 ? point : solveSteady(f, a, inflow, guess);
  const target = point === undefined ? solved : Math.min(solved, point);
  const g = f.setup.region.model === "fixed" ? Infinity : restoringConductance(f, a, inflow, target);
  return { target, tau: f.setup.region.model === "fixed" ? 0 : f.setup.region.heatCapacityJM2K / g };
}

/** 🎯️ The steady nodes of every non-body region of an instant, `null` for the body regions. A source region carries the draw of its coupled regions as a function of its own temperature, each of them solved at its steady state for that source temperature,
 * so the source's balance (and its cap at a phase-change point) already contains the heat the walls take; the coupled regions are then solved at the temperature found. */
function steadyNodesOf(sys: System, now: Instant, startC: number): (Node | null)[] {
  const drawn = sys.coupled
    ? now.frames.map((f, j): RegionFrame => {
        const deps = sys.dependents[j]!;
        if (deps.length === 0) return f;
        const draw = (tsC: number): number =>
          deps.reduce((sum, d) => {
            const g = { ...now.frames[d]!, sourceC: tsC };
            const inflow = inflowOf(g, now.levels[d]!);
            return sum + (g.setup.region.areaM2 * inflow(solveSteady(g, now.amb, inflow, tsC))) / f.setup.region.areaM2;
          }, 0);
        return { ...f, draw };
      })
    : now.frames;
  const nodes = sys.regions.map((_, j) => (sys.place.has(j) || sys.source[j]! >= 0 ? null : steadyOf(drawn[j]!, now.amb, now.levels[j]!, startC)));
  return nodes.map((n, j) => (n !== null || sys.place.has(j) ? n : steadyOf({ ...now.frames[j]!, sourceC: nodes[sys.source[j]!]!.target }, now.amb, now.levels[j]!, startC)));
}

const label = (en: string, de: string): Text => ({ en, de });

/** 🧊️ Sub-steps of a history step for regions with a phase change (the conductance is re-evaluated at each, so the cap at the phase-change point is met accurately). */
export const PHASE_SUBSTEPS = 16;

/** ➡️ Advances one non-body region over `dtS`: the first-order response, and for a region with a phase change the cap at its point: the energy that would raise the temperature beyond it
 * (`C ΔT` of the uncapped step, or the whole residual `inflow − outflow` while resting at the point) is converted into `kg` of the new phase at the latent heat. */
export function stepRegion(f: RegionFrame, amb: Ambient, level: number, temp: number, dtS: number): { readonly temp: number; readonly kg: number } {
  const region = f.setup.region;
  const phase = f.setup.phase;
  const inflow = inflowOf(f, level);
  const advance = (from: number, dt: number): number => {
    const g = restoringConductance(f, amb, inflow, from);
    return relax(from, from + residualWM2(f, amb, inflow, from) / g, dt, region.heatCapacityJM2K / g);
  };
  if (phase === null) return { temp: advance(temp, dtS), kg: 0 };
  const dt = dtS / PHASE_SUBSTEPS;
  let t = temp;
  let joulesM2 = 0;
  for (let i = 0; i < PHASE_SUBSTEPS; i++) {
    const q = t >= phase.pointC - 1e-9 ? phaseChangeWM2(f, amb, inflow, phase.pointC) : 0;
    if (q > 0) {
      joulesM2 += q * dt;
      t = phase.pointC;
      continue;
    }
    const next = advance(t, dt);
    if (next > phase.pointC) joulesM2 += region.heatCapacityJM2K * (next - phase.pointC);
    t = Math.min(next, phase.pointC);
  }
  return { temp: t, kg: (joulesM2 * region.areaM2) / phase.latentHeatJKg };
}

/** 📈️ Advances a coupled region over `dtS` while the temperature of its source ramps linearly from `fromC` to `toC`: the exponential response to the ramp, so the wall lags the water by `rate × τ` instead of jumping to the end of the step. */
function stepRamped(f: RegionFrame, amb: Ambient, level: number, temp: number, dtS: number, fromC: number, toC: number): number {
  const at = (sourceC: number): { readonly target: number; readonly tau: number } => {
    const g = { ...f, sourceC };
    const inflow = inflowOf(g, level);
    const k = restoringConductance(g, amb, inflow, temp);
    return { target: temp + residualWM2(g, amb, inflow, temp) / k, tau: f.setup.region.heatCapacityJM2K / k };
  };
  const a = at(fromC);
  const b = at(toC);
  const x = dtS / b.tau;
  const decay = Math.exp(-x);
  return b.target + (temp - b.target) * decay + (b.target - a.target) * (decay - (x > 1e-9 ? (1 - decay) / x : 1));
}

const MAX_COUPLED_SUBSTEPS = 128;
const SUBSTEPS_PER_TAU = 5;

/** 🪜️ Advances every non-body region of an instant over `dtS`. Sources and plain regions take the exponential step with the draw of the coupled regions at their present temperatures; coupled regions then follow the ramp of their source. With couplings the step is
 * split so that no sub-step is longer than the fastest coupled region's time constant, which keeps the heat the walls take from the water consistent with the heat they store. */
function stepRegions(sys: System, instant: Instant, temps: readonly number[], phaseKg: readonly number[], dtS: number): { readonly temps: number[]; readonly phaseKg: number[] } {
  let n = 1;
  if (sys.coupled) {
    const frames = withCoupling(sys, instant.frames, temps);
    const taus = frames.flatMap((f, j) => (sys.source[j]! < 0 ? [] : [f.setup.region.heatCapacityJM2K / restoringConductance(f, instant.amb, inflowOf(f, instant.levels[j]!), temps[j]!)]));
    n = Math.min(MAX_COUPLED_SUBSTEPS, Math.max(1, Math.ceil((SUBSTEPS_PER_TAU * dtS) / Math.min(...taus))));
  }
  const h = dtS / n;
  let now = temps;
  let kg = phaseKg;
  for (let s = 0; s < n; s++) {
    const frames = withCoupling(sys, instant.frames, now);
    const next: number[] = [];
    const mass: number[] = [];
    now.forEach((temp, j) => {
      if (sys.source[j]! >= 0) return;
      const region = frames[j]!.setup.region;
      if (sys.place.has(j)) [next[j], mass[j]] = [temp, 0];
      else if (region.model === "fixed") [next[j], mass[j]] = [region.fixed?.surfaceTempC ?? temp, 0];
      else {
        const r = stepRegion(frames[j]!, instant.amb, instant.levels[j]!, temp, h);
        [next[j], mass[j]] = [r.temp, r.kg];
      }
    });
    now.forEach((temp, j) => {
      const source = sys.source[j]!;
      if (source < 0) return;
      next[j] = stepRamped(instant.frames[j]!, instant.amb, instant.levels[j]!, temp, h, now[source]!, next[source]!);
      mass[j] = 0;
    });
    now = next;
    kg = kg.map((m, j) => m + mass[j]!);
  }
  return { temps: [...now], phaseKg: [...kg] };
}

/** ♨️ Solves the thermal field of an evaluated state under an environment (a constant {@link Environment} or an {@link EnvironmentTimeline} sampled at `state.timeS`;
 * absent: the asset's reference environment). The surface and skin temperatures follow the input history if `options.history` is given; otherwise the state's inputs are
 * assumed to have held forever (steady state). Independent of the drawing scale: nothing but `state.thermal`, `state.timeS`, the figure velocity and the asset is read. */
export function solveThermal(state: EvaluatedState, environment?: EnvironmentInput, options: ThermalOptions = {}): ThermalField {
  const asset = state.asset;
  const sys = systemOf(asset);
  const input = environment ?? asset.thermal.environment;
  const t = state.timeS;
  const now = instantOf(sys, { inputs: state.thermal, comVelocity: state.kinematics.comVelocity }, environmentAt(input, t));
  const steadyNodes = steadyNodesOf(sys, now, now.env.airTempC);
  const steadyBodies = sys.bodies.map((b) => steadyBody(b.setup, subjectInputs(now.inputs, b.subject), bodyFrames(b, now), now.amb));
  const skinTaus = sys.bodies.map((b, k) => skinConductances(b.setup, steadyBodies[k]!.state, bodyFrames(b, now), now.amb, steadyBodies[k]!.diag).map((g, i) => b.setup.regions[i]!.capacityJK / b.setup.regions[i]!.skinAreaM2 / g));
  const placed = (j: number): { readonly body: number; readonly slot: number } | undefined => sys.place.get(j);
  const steadyTemps = steadyNodes.map((n, j) => n?.target ?? steadyBodies[placed(j)!.body]!.diag.regions[placed(j)!.slot]!.surfaceC);
  const useHistory = options.history !== undefined || isTimeline(input);
  const temps = [...steadyTemps];
  const bodyStates: BodyState[] = steadyBodies.map((b) => b.state);
  const taus = steadyNodes.map((n, j) => n?.tau ?? skinTaus[placed(j)!.body]![placed(j)!.slot]!);

  if (useHistory && t > 0) {
    const session = options.session ?? thermalSession();
    const instantAt = (time: number): Instant =>
      remembered(session.instants, time, SESSION_INSTANTS, () => instantOf(sys, options.history?.(time) ?? { inputs: state.thermal, comVelocity: state.kinematics.comVelocity }, environmentAt(input, time)));
    const reference = instantAt(0);
    const referenceNodes = steadyNodesOf(sys, reference, reference.env.airTempC);
    const referenceBodies = sys.bodies.map((b) => steadyBody(b.setup, subjectInputs(reference.inputs, b.subject), bodyFrames(b, reference), reference.amb));
    const coreTaus = sys.bodies.map((b, k) => b.setup.coreCapacityJK / Math.max(1, referenceBodies[k]!.diag.regions.reduce((s, d, i) => s + d.conductanceWM2K * b.setup.regions[i]!.skinAreaM2, 0)));
    const tauMax = Math.max(0, ...coreTaus, ...referenceNodes.map((n) => n?.tau ?? 0));
    const plan = historyPlan(t, tauMax, { ...(options.windowS !== undefined ? { windowS: options.windowS } : {}), ...(options.maxSteps !== undefined ? { maxSteps: options.maxSteps } : {}) });
    const advance = (from: Trajectory, previous: number, at: number): Trajectory => {
      options.signal?.throwIfAborted();
      const dt = at - previous;
      const instant = instantAt(previous + dt / 2);
      const stepped = stepRegions(sys, instant, from.temps, from.phaseKg, dt);
      return {
        temps: stepped.temps,
        phaseKg: stepped.phaseKg,
        bodies: sys.bodies.map((b, i) => stepBody(b.setup, from.bodies[i]!, subjectInputs(instant.inputs, b.subject), bodyFrames(b, instant), instant.amb, dt).state),
      };
    };
    const chain = remembered(session.chains, `${plan.startS}|${plan.stepS}`, SESSION_CHAINS, (): Chain => {
      const start = instantAt(plan.startS);
      const startNodes = steadyNodesOf(sys, start, start.env.airTempC);
      const startBodies = sys.bodies.map((b) => steadyBody(b.setup, subjectInputs(start.inputs, b.subject), bodyFrames(b, start), start.amb));
      return {
        start: {
          temps: sys.regions.map((_, j) => {
            const at = sys.place.get(j);
            return at === undefined ? startNodes[j]!.target : startBodies[at.body]!.diag.regions[at.slot]!.surfaceC;
          }),
          bodies: startBodies.map((b) => b.state),
          phaseKg: sys.regions.map(() => 0),
        },
        nodes: [],
      };
    });
    const grid = plan.times.length - 1;
    for (let k = 0; k < grid; k++) {
      if (chain.nodes[k] !== undefined) continue;
      chain.nodes[k] = advance(k === 0 ? chain.start : chain.nodes[k - 1]!, k === 0 ? plan.startS : plan.times[k - 1]!, plan.times[k]!);
      options.progress?.(k + 1, plan.times.length, label("Thermal history", "Thermischer Verlauf"));
    }
    const base = grid <= 0 ? chain.start : chain.nodes[grid - 1]!;
    const final = grid < 0 ? chain.start : advance(base, grid === 0 ? plan.startS : plan.times[grid - 1]!, plan.times[grid]!);
    options.progress?.(plan.times.length, plan.times.length, label("Thermal history", "Thermischer Verlauf"));
    return assemble(asset, sys, instantAt(t), final.temps, steadyTemps, taus, [...final.bodies], input, true, final.phaseKg);
  }
  return assemble(asset, sys, now, temps, steadyTemps, taus, bodyStates, input, false, temps.map(() => 0));
}

function splitFields(s: RegionSetup, env: Environment, tsC: number): Pick<RegionField, "netRadiativeSkyWM2" | "netRadiativeGroundWM2" | "netRadiativeSurroundingsWM2"> {
  const x = splitOf(s, env, tsC);
  return { netRadiativeSkyWM2: x.sky, netRadiativeGroundWM2: x.ground, netRadiativeSurroundingsWM2: x.surroundings };
}

function bodyFieldOf(core: number, d: ReturnType<typeof evaluateBody>): BodyField {
  return { coreTempC: core, meanSkinTempC: d.meanSkinC, metabolicW: d.metabolicW, respiratorySensibleW: d.respiratory.sensibleW, respiratoryLatentW: d.respiratory.latentW, skinBloodFlowLM2H: d.meanBloodFlow, skinWettedness: d.meanWettedness, storageW: d.storageW };
}

function assemble(asset: Asset, sys: System, now: Instant, temps: readonly number[], steadyTemps: readonly number[], taus: readonly number[], bodyStates: readonly BodyState[], input: EnvironmentInput, transient: boolean, phaseKg: readonly number[]): ThermalField {
  const diags = sys.bodies.map((b, k) => evaluateBody(b.setup, bodyStates[k]!, subjectInputs(now.inputs, b.subject), bodyFrames(b, now), now.amb));
  const regions: Record<Slug, RegionField> = {};
  const frames = withCoupling(sys, now.frames, temps);
  let netRadiativeW = 0;
  let sensibleW = 0;
  let latentW = 0;
  let moistureGH = 0;
  sys.regions.forEach((s, j) => {
    const r = s.region;
    const f = frames[j]!;
    const area = r.areaM2 * f.exposed;
    const p = sys.place.get(j);
    let field: RegionField;
    if (p) {
      const d = diags[p.body]!.regions[p.slot]!;
      const skinArea = sys.bodies[p.body]!.setup.regions[p.slot]!.skinAreaM2;
      field = {
        region: r.id,
        surfaceTempC: d.surfaceC,
        steadyTempC: steadyTemps[j]!,
        skinTempC: d.skinC,
        emittedRadiantWM2: emittedOf(f, d.surfaceC),
        netRadiativeWM2: d.exposure.radiative,
        sensibleWM2: d.exposure.sensible,
        latentWM2: d.evapOuterWM2,
        moistureGM2H: (d.evapOuterWM2 / latentHeatJkg(d.skinC)) * 3.6e6,
        absorbedSolarWM2: f.solar,
        conductedWM2: d.conductedOuterWM2,
        ...splitFields(sys.regions[j]!, now.env, d.surfaceC),
        contactWM2: f.contact * d.exposure.contact,
        netRadiativeW: d.exposure.radiative * area,
        sensibleW: d.exposure.sensible * area,
        latentW: d.evapWM2 * skinArea,
        phaseChangeW: 0,
        phaseChangeKgS: 0,
        phaseChangedKg: 0,
        evaporationKgS: (d.evapWM2 * skinArea) / latentHeatJkg(d.surfaceC),
        convectionCoefficientWM2K: d.exposure.hc,
        radiationCoefficientWM2K: d.exposure.hr,
        timeConstantS: taus[j]!,
        exposedFraction: f.exposed,
        sunlitFraction: f.sunlit,
      };
    } else {
      const ts = temps[j]!;
      const x = exchange(f, now.amb, ts);
      const phase = s.phase;
      const phaseWM2 = phase !== null && ts >= phase.pointC - 1e-6 ? phaseChangeWM2(f, now.amb, inflowOf(f, now.levels[j]!), phase.pointC) : 0;
      const phaseW = phaseWM2 * r.areaM2;
      const phaseKgS = phase === null ? 0 : phaseW / phase.latentHeatJKg;
      const enclosed = r.areaM2 * Math.max(f.exposed, ENCLOSED_EXPOSURE);
      field = {
        region: r.id,
        surfaceTempC: ts,
        steadyTempC: steadyTemps[j]!,
        skinTempC: null,
        emittedRadiantWM2: emittedOf(f, ts),
        netRadiativeWM2: x.radiative,
        sensibleWM2: x.sensible,
        latentWM2: x.latent + phaseW / enclosed,
        moistureGM2H: (x.latent / latentHeatJkg(ts) + phaseKgS / enclosed) * 3.6e6,
        absorbedSolarWM2: f.solar,
        conductedWM2: inflowOf(f, now.levels[j]!)(ts),
        ...splitFields(s, now.env, ts),
        contactWM2: f.contact * x.contact,
        netRadiativeW: x.radiative * area,
        sensibleW: x.sensible * area,
        latentW: x.latent * area + phaseW,
        phaseChangeW: phaseW,
        phaseChangeKgS: phaseKgS,
        phaseChangedKg: phaseKg[j]!,
        evaporationKgS: (x.latent * area) / latentHeatJkg(ts) + phaseKgS,
        convectionCoefficientWM2K: x.hc,
        radiationCoefficientWM2K: x.hr,
        timeConstantS: taus[j]!,
        exposedFraction: f.exposed,
        sunlitFraction: f.sunlit,
      };
    }
    regions[r.id] = field;
    netRadiativeW += field.netRadiativeW;
    sensibleW += field.sensibleW;
    latentW += field.latentW;
    moistureGH += ((field.latentW - field.phaseChangeW) / latentHeatJkg(field.surfaceTempC) + field.phaseChangeKgS) * 3.6e6;
  });
  const passengers: Record<Slug, BodyField> = {};
  let primary: BodyField | null = null;
  sys.bodies.forEach((b, k) => {
    const field = bodyFieldOf(bodyStates[k]!.core, diags[k]!);
    if (b.subject === null) primary = field;
    else passengers[b.subject] = field;
  });
  const respSensible = diags.reduce((s, d) => s + d.respiratory.sensibleW, 0);
  const respLatent = diags.reduce((s, d) => s + d.respiratory.latentW, 0);
  const timeline = isTimeline(input);
  return {
    environment: now.env,
    regions,
    body: primary,
    passengers,
    totals: { netRadiativeW, sensibleW, latentW, moistureGH, emitterSensibleW: respSensible, emitterLatentW: respLatent },
    assumptions: [...asset.thermal.assumptions, ...ASSUMPTIONS],
    notes: [
      transient
        ? label("Surface and skin temperatures follow the input history (first-order response per region, core temperature with the body heat capacity).", "Oberflächen- und Hauttemperaturen folgen dem Eingabeverlauf (Antwort erster Ordnung je Region, Kerntemperatur mit der Wärmekapazität des Körpers).")
        : label("Steady state of the current inputs (no input history given).", "Stationärer Zustand der aktuellen Eingaben (kein Eingabeverlauf angegeben)."),
      ...(timeline ? [label("The environment follows its timeline.", "Die Umgebung folgt ihrer Zeitleiste.")] : []),
      ...sys.bodies.map((b, k) => label(`Whole-body ventilation ${diags[k]!.ventilationLMin.toFixed(1)} L/min${b.subject ? ` (${b.subject})` : ""} (breathing pattern plus panting), shared with the breath emitters.`, `Gesamtventilation ${diags[k]!.ventilationLMin.toFixed(1)} l/min${b.subject ? ` (${b.subject})` : ""} (Atemmuster plus Hecheln), gemeinsam mit den Atem-Emittern.`)),
    ],
  };
}

const ASSUMPTIONS: readonly Text[] = [
  label("Illustrative engineering estimate: lumped regions, no CFD, no conduction between regions, no condensation, no sweat dynamics beyond the two-node model.", "Illustrative technische Schätzung: konzentrierte Regionen, keine CFD, keine Leitung zwischen Regionen, keine Kondensation, keine Schweißdynamik jenseits des Zweiknotenmodells."),
  label("Radiation against sky, ground and surroundings weighted by the view factors of the region (sky view factor, geometry); solar by sunlit share with geometry factors; wind relative to the figure's motion.", "Strahlung gegen Himmel, Boden und Umgebung nach den Sichtfaktoren der Region (Himmelssichtfaktor, Geometrie); Sonne nach besonntem Anteil mit Geometriefaktoren; Wind relativ zur Bewegung der Figur."),
];

/** 🧮️ Area-weighted value of a quantity over merged regions (the colour of a merged small-scale shape): `Σ w_i v_i / Σ w_i` with `w_i = area_i × exposedFraction_i`.
 * Skin temperature falls back to the surface temperature of regions without skin. `null` when none of the regions exists in the field. */
export function areaWeighted(regions: readonly ThermalRegion[], field: ThermalField, ids: readonly Slug[], quantity: ThermalQuantity): number | null {
  let weight = 0;
  let sum = 0;
  for (const id of ids) {
    const f = field.regions[id];
    const r = regions.find((x) => x.id === id);
    if (!f || !r) continue;
    const v = QUANTITY_SPECS[quantity].pick(f) ?? (quantity === "skin-temperature" ? f.surfaceTempC : null);
    if (v === null) continue;
    const w = r.areaM2 * Math.max(f.exposedFraction, 1e-9);
    weight += w;
    sum += w * v;
  }
  return weight > 0 ? sum / weight : null;
}

/** 🪟️ Overall heat transmittance `U = 1 / (ΣR_layers + R_interior film + 1/h_exterior)` of a conditioned element for an exterior film coefficient `exteriorWM2K` (default 25 W/(m² K), EN ISO 6946 R_se = 0.04). */
export function uValue(region: ThermalRegion, exteriorWM2K = 25): number | null {
  if (!region.conditioned) return null;
  return 1 / (region.layers.reduce((s, l) => s + l.resistanceM2KW, 0) + region.conditioned.interiorFilmM2KW + 1 / exteriorWM2K);
}

/** ☀️ The net longwave exchange of a solved region split by counterpart (sky, ground, surroundings), W/m² of exposed area, positive = the region loses heat; the parts sum to `netRadiativeWM2`.
 * Leaves exchange through both faces; the split includes both. */
export function radiationSplit(field: ThermalField, regionId: Slug): RadiationSplit {
  const f = field.regions[regionId];
  if (!f) throw new Error(`Region ${regionId} is not part of the solved field.`);
  return { sky: f.netRadiativeSkyWM2, ground: f.netRadiativeGroundWM2, surroundings: f.netRadiativeSurroundingsWM2 };
}

/** 🥵️ The operative temperature a region experiences, `(h_r T_rad + h_c T_air)/(h_r + h_c)` with the region's own radiation and convection coefficients and its radiant environment
 * (sky, ground, surroundings by view factor): the single temperature that gives the same dry heat exchange as radiation and convection together (ISO 7726 definition). */
export function operativeTemperatureC(asset: Asset, field: ThermalField, regionId: Slug): number {
  const setup = systemOf(asset).regions.find((s) => s.region.id === regionId);
  const f = field.regions[regionId];
  if (!setup || !f) throw new Error(`Region ${regionId} is not part of the solved field of ${asset.id}.`);
  const rad = radiantEnvironmentC(field.environment, setup.region.geometry, setup.region.skyViewFactor);
  const { radiationCoefficientWM2K: hr, convectionCoefficientWM2K: hc } = f;
  return (hr * rad + hc * field.environment.airTempC) / (hr + hc);
}

/** 🧪️ Runs only the body of an asset (`subject` null) or one passenger for `durationS` seconds under constant inputs and environment, from the neutral state (`start: "neutral"`, as
 * Gagge's simulation and pythermalcomfort do) or from the steady state; returns the final whole-body state. Used by the oracle tests. */
export function simulateBody(asset: Asset, inputs: ThermalInputs, env: Environment, durationS: number, start: "neutral" | "steady" = "neutral", stepS = 60, subject: Slug | null = null): { readonly state: BodyState; readonly field: BodyField; readonly diag: ReturnType<typeof evaluateBody> } {
  const sys = systemOf(asset);
  const body = sys.bodies.find((b) => b.subject === subject);
  if (!body) throw new Error(`Asset ${asset.id} has no body${subject ? ` for ${subject}` : ""}.`);
  const instant = instantOf(sys, { inputs }, env);
  const frames = bodyFrames(body, instant);
  const own = subjectInputs(inputs, subject);
  let st = start === "steady" ? steadyBody(body.setup, own, frames, instant.amb).state : neutralState(body.setup);
  for (let t = 0; t < durationS - 1e-9; t += stepS) st = stepBody(body.setup, st, own, frames, instant.amb, Math.min(stepS, durationS - t)).state;
  const diag = evaluateBody(body.setup, st, own, frames, instant.amb);
  return { state: st, field: bodyFieldOf(st.core, diag), diag };
}
