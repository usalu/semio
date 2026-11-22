/** 🌊️ The pure rules of the thermodynamic flow view on screen: which layers a view shows, how far the flow has developed before it is shown, which time on the transport timeline a clock time is, the box of paper the flow needs around a figure and how a world point lands on it. No solver and no DOM is loaded here; the solver sits behind {@link ./📡️session/🟦️.ts}, the drawing in {@link ./🖌️overlay/🟦️.ts}.
 *
 * Conditions. Besides the environment a reader sets the air the flow is solved in ({@link FlowSettings}: turbulence, the wind profile, the life of a parcel, the seed of the random walk) and how its marks are generalised (`FlowMarkDetail`); every one of them has a default ({@link FLOW_DEFAULTS}, {@link MARK_DEFAULTS}) and only what differs from it is kept, written to the address or sent to the solver.
 *
 * Time. A pose has no timeline of its own: its flow is shown developed for the life of a parcel ({@link FLOW_DEVELOP_S} seconds unless the reader chose another) and then simply runs on while its clock plays (the solver time only moves forward, so a wrap or a seek of the clock never restarts the field). A looping clip is periodic with its duration: its flow is developed for whole loops and a clock time maps to the earliest instant of the same loop phase that is not before the last one shown. Any other clip starts with still air at its first frame, exactly as its devices do.
 *
 * @see ../../../../🔨️modules/💨️transport/🌊️flow/🟦️.ts — the field, the streams and the tracers
 * @see ../../../../🔨️modules/🎨️depiction/🌊️flow-view/🟦️.ts — the paint shared with paper
 */
import { DEFAULT_FLOW_LAYERS, DEFAULT_TURBULENCE, FLOW_LAYERS, FLOW_STYLE, TRACER_EXPOSURE_S, TRACER_PERIOD_S, developedTimeS, flowDomainOf, flowLoopS, project } from "@teaching/staffage";
import type { Asset, AxonometricCamera, EvaluatedState, EvaluationQuery, FlowConditions, FlowLayer, FlowMarkDetail, Vec3, View, Viewpoint } from "@teaching/staffage";
import { PER_CENT, type Quantity } from "../🌤️environments/🟦️.ts";
import { DRAWN_FLOW_DEVELOP_S, type SceneFrame } from "../🧮️pipeline/🟦️.ts";

/** ⏳️ Seconds of history the flow of a pose or of a looping clip has when it is first shown, and the life of a parcel on screen; drawn files use the same. */
export const FLOW_DEVELOP_S = DRAWN_FLOW_DEVELOP_S;

/** 🔁️ Seconds the clock of a pose runs before it wraps; the flow itself does not wrap. */
export const FLOW_POSE_LOOP_S = 60;

/** 🔢️ Requests for a refreshed field are quantised to this many per second of clock time. */
export const FLOW_REFRESH_HZ = 8;

/** 🪧️ The layers of the flow view in a stable order. */
export const ALL_FLOW_LAYERS: readonly FlowLayer[] = FLOW_LAYERS;

/** 🌱️ The layers a thermal figure shows when nothing says otherwise: all but `pathlines`. */
export const SHOWN_FLOW_LAYERS: readonly FlowLayer[] = DEFAULT_FLOW_LAYERS;

/** ❓️ Whether a view shows air flow at all: only the thermal view of an asset that emits or moves air. */
export const flowsIn = (asset: Asset, view: View): boolean => view === "thermal" && asset.thermal.emitters.length > 0;

/** 🔤️ The layers of an address code (`fl=s,t,m,f,p`, the first letter of each layer; `fl=0` for none) and back; absent means the default layers ({@link SHOWN_FLOW_LAYERS}). */
const CODES: Readonly<Record<string, FlowLayer>> = { s: "streams", t: "tracers", m: "moisture", f: "fog", p: "pathlines" };
export const flowLayersFrom = (code: string | null): readonly FlowLayer[] => (code === null ? SHOWN_FLOW_LAYERS : ALL_FLOW_LAYERS.filter((layer) => code.split(",").some((part) => CODES[part] === layer)));
export const flowLayersCode = (layers: readonly FlowLayer[]): string | null => (layers.length === SHOWN_FLOW_LAYERS.length && SHOWN_FLOW_LAYERS.every((layer) => layers.includes(layer)) ? null : layers.length === 0 ? "0" : ALL_FLOW_LAYERS.filter((layer) => layers.includes(layer)).map((layer) => layer[0]).join(","));

/** 🌪️ The conditions of the flow as a reader sets them, each on its own: turbulence intensity (σ_u over the wind speed), the smallest σ_u (m/s) and the Lagrangian time scale (s); the wind profile, uniform or a logarithmic boundary layer of roughness length `roughnessM` that reaches the wind speed of the environment at `referenceHeightM`; the life of a parcel, which is also the time the flow has developed when it is first shown (s); the seed of the turbulent random walk. */
export type FlowSettings = { readonly turbulenceIntensity?: number; readonly turbulenceFloorMS?: number; readonly lagrangianTimeS?: number; readonly profile?: "logarithmic"; readonly roughnessM?: number; readonly referenceHeightM?: number; readonly lifeS?: number; readonly seed?: number };

/** 🌱️ What the solver uses where a reader set nothing: the library's turbulence, a uniform wind (the logarithmic profile, once chosen, starts over open grass, roughness 0.03 m, with the wind speed measured at 10 m), a parcel life of {@link FLOW_DEVELOP_S} seconds, seed 1. */
export const FLOW_DEFAULTS = { turbulenceIntensity: DEFAULT_TURBULENCE.intensity, turbulenceFloorMS: DEFAULT_TURBULENCE.floorMS, lagrangianTimeS: DEFAULT_TURBULENCE.lagrangianTimeS, roughnessM: 0.03, referenceHeightM: 10, lifeS: FLOW_DEVELOP_S, seed: 1 } as const satisfies Required<Omit<FlowSettings, "profile">>;

/** 🎚️ The numbers of {@link FlowSettings} as quantities, in the order of the address. */
export const FLOW_QUANTITIES: readonly Quantity<keyof typeof FLOW_DEFAULTS>[] = [
  { key: "roughnessM", param: "z0", attribute: "roughness", min: 0.001, max: 1, step: 0.001, digits: 3, unit: "m", shown: (v) => v, stored: (v) => v },
  { key: "referenceHeightM", param: "zr", attribute: "reference-height", min: 2, max: 100, step: 0.5, digits: 1, unit: "m", shown: (v) => v, stored: (v) => v },
  { key: "turbulenceIntensity", param: "ti", attribute: "turbulence", min: 0, max: 100, step: 1, digits: 0, unit: "%", ...PER_CENT },
  { key: "turbulenceFloorMS", param: "tf", attribute: "turbulence-floor", min: 0, max: 1, step: 0.01, digits: 2, unit: "m/s", shown: (v) => v, stored: (v) => v },
  { key: "lagrangianTimeS", param: "tt", attribute: "turbulence-time", min: 0.1, max: 30, step: 0.1, digits: 1, unit: "s", shown: (v) => v, stored: (v) => v },
  { key: "lifeS", param: "life", attribute: "flow-life", min: 2, max: 60, step: 1, digits: 0, unit: "s", shown: (v) => v, stored: (v) => v },
  { key: "seed", param: "seed", attribute: "flow-seed", min: 1, max: 9999, step: 1, digits: 0, unit: "", shown: (v) => v, stored: (v) => v },
];

/** 🔑️ The members of the mark detail a reader sets; `loopS` is not one of them, it belongs to a looping export. */
export type MarkKey = "spacingMm" | "minStreakMm" | "periodS" | "exposureS" | "trailS" | "strokeMm";

/** 🌱️ The generalisation, timing and weight of the marks where a reader set nothing: the rule of the drawing scale of the library (`flowDetailOf`), its tracer model, the whole parcel path (a trail of 0 s is no trail) and the hairline of a print. */
export const MARK_DEFAULTS = { spacingMm: 2.2, minStreakMm: 0.9, periodS: TRACER_PERIOD_S, exposureS: TRACER_EXPOSURE_S, trailS: 0, strokeMm: FLOW_STYLE.streamMaxMm } as const satisfies Required<Pick<FlowMarkDetail, MarkKey>>;

/** 🎚️ The numbers of the mark detail as quantities, in the order of the address. */
export const MARK_QUANTITIES: readonly Quantity<MarkKey>[] = [
  { key: "spacingMm", param: "fsp", attribute: "flow-spacing", min: 0.5, max: 20, step: 0.1, digits: 1, unit: "mm", shown: (v) => v, stored: (v) => v },
  { key: "minStreakMm", param: "fst", attribute: "flow-streak", min: 0, max: 10, step: 0.1, digits: 1, unit: "mm", shown: (v) => v, stored: (v) => v },
  { key: "periodS", param: "fpe", attribute: "flow-period", min: 0.2, max: 10, step: 0.1, digits: 1, unit: "s", shown: (v) => v, stored: (v) => v },
  { key: "exposureS", param: "fex", attribute: "flow-exposure", min: 0.05, max: 5, step: 0.05, digits: 2, unit: "s", shown: (v) => v, stored: (v) => v },
  { key: "trailS", param: "ftr", attribute: "flow-trail", min: 0, max: 60, step: 0.5, digits: 1, unit: "s", shown: (v) => v, stored: (v) => v },
  { key: "strokeMm", param: "fsw", attribute: "flow-stroke", min: 0.05, max: 2, step: 0.01, digits: 2, unit: "mm", shown: (v) => v, stored: (v) => v },
];

/** 🧮️ The conditions the solver takes for a reader's settings: only what was set, so an untouched viewer asks for exactly the solver's defaults. Turbulence and the boundary layer are whole objects: a member the reader left alone takes its default. */
export function flowConditionsOf(settings: FlowSettings): FlowConditions {
  const turbulent = settings.turbulenceIntensity !== undefined || settings.turbulenceFloorMS !== undefined || settings.lagrangianTimeS !== undefined;
  return {
    ...(turbulent ? { turbulence: { intensity: settings.turbulenceIntensity ?? FLOW_DEFAULTS.turbulenceIntensity, floorMS: settings.turbulenceFloorMS ?? FLOW_DEFAULTS.turbulenceFloorMS, lagrangianTimeS: settings.lagrangianTimeS ?? FLOW_DEFAULTS.lagrangianTimeS } } : {}),
    ...(settings.profile === "logarithmic" ? { boundaryLayer: { z0M: settings.roughnessM ?? FLOW_DEFAULTS.roughnessM, referenceHeightM: settings.referenceHeightM ?? FLOW_DEFAULTS.referenceHeightM } } : {}),
    ...(settings.lifeS === undefined ? {} : { maxAgeS: settings.lifeS }),
    ...(settings.seed === undefined ? {} : { seed: settings.seed }),
  };
}

/** 🧭️ The eight directions a bearing is worded with, in steps of 45° from ahead (0°) over the asset's left (90°) and behind it (180°) to its right (270°). */
export const WIND_SECTORS = ["ahead", "aheadLeft", "left", "behindLeft", "behind", "behindRight", "right", "aheadRight"] as const;

/** 🗣️ The sector of {@link WIND_SECTORS} a bearing lies in. */
export const windSector = (bearingDeg: number): (typeof WIND_SECTORS)[number] => WIND_SECTORS[Math.round(((((bearingDeg % 360) + 360) % 360) / 45)) % 8]!;

/** 🔁️ The period of the flow of a query: the library's `flowLoopS` (the duration of a looping clip), else 0 (not periodic). */
export function flowPeriodS(asset: Asset, query: EvaluationQuery): number {
  return flowLoopS({ asset, query }) ?? 0;
}

/** 🕰️ The time on the transport timeline that a clock time shows, given the last time shown (`null` at the start): see the module text. */
export function flowTimeS(query: EvaluationQuery, periodS: number, clockS: number, elapsedS: number, lastS: number | null, developS = FLOW_DEVELOP_S): number {
  if (query.kind === "pose") return developS + elapsedS;
  if (periodS <= 0) return Math.max(0, clockS);
  const first = developedTimeS(((clockS % periodS) + periodS) % periodS, developS, periodS);
  return lastS === null || first >= lastS ? first : first + Math.ceil((lastS - first) / periodS - 1e-9) * periodS;
}

/** 🪟️ The paper window (millimetres about the scene origin, y down) that the flow domain of a state takes in a viewpoint at `mmPerMetre`. */
export function flowFrameOf(state: EvaluatedState, viewpoint: Viewpoint, mmPerMetre: number, camera?: AxonometricCamera): SceneFrame {
  const { min, max } = flowDomainOf(state.bounds, state.asset.dimensions.widthM / 2);
  const corners: Vec3[] = [];
  for (const x of [min.x, max.x]) for (const y of [min.y, max.y]) for (const z of [min.z, max.z]) corners.push({ x, y, z });
  const points = corners.map((corner) => project(viewpoint, corner, camera));
  return { minX: Math.min(...points.map((p) => p.x)) * mmPerMetre, minY: -Math.max(...points.map((p) => p.y)) * mmPerMetre, maxX: Math.max(...points.map((p) => p.x)) * mmPerMetre, maxY: -Math.min(...points.map((p) => p.y)) * mmPerMetre };
}

/** ➕️ The smallest window that holds both. */
export const unionFrame = (a: SceneFrame, b: SceneFrame): SceneFrame => ({ minX: Math.min(a.minX, b.minX), minY: Math.min(a.minY, b.minY), maxX: Math.max(a.maxX, b.maxX), maxY: Math.max(a.maxY, b.maxY) });
