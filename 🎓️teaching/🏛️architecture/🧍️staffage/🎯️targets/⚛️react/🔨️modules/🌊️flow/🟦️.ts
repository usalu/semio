/** 🌊️ The pure rules of the thermodynamic flow view on screen: which layers a view shows, how far the flow has developed before it is shown, which time on the transport timeline a clock time is, the box of paper the flow needs around a figure and how a world point lands on it. No solver and no DOM is loaded here; the solver sits behind {@link ./📡️session/🟦️.ts}, the drawing in {@link ./🖌️overlay/🟦️.ts}.
 *
 * Time. A pose has no timeline of its own: its flow is shown developed for {@link FLOW_DEVELOP_S} seconds and then simply runs on while its clock plays (the solver time only moves forward, so a wrap or a seek of the clock never restarts the field). A looping clip is periodic with its duration: its flow is developed for whole loops and a clock time maps to the earliest instant of the same loop phase that is not before the last one shown. Any other clip starts with still air at its first frame, exactly as its devices do.
 *
 * @see ../../../../🔨️modules/💨️transport/🌊️flow/🟦️.ts — the field, the streams and the tracers
 * @see ../../../../🔨️modules/🎨️depiction/🌊️flow-view/🟦️.ts — the paint shared with paper
 */
import { FLOW_LAYERS, developedTimeS, flowDomainOf, project } from "@teaching/staffage";
import type { Asset, AxonometricCamera, EvaluatedState, EvaluationQuery, FlowLayer, Vec3, View, Viewpoint } from "@teaching/staffage";
import type { SceneFrame } from "../🧮️pipeline/🟦️.ts";

/** ⏳️ Seconds of history the flow of a pose or of a looping clip has when it is first shown, and the life of a parcel on screen. */
export const FLOW_DEVELOP_S = 14;

/** 🔁️ Seconds the clock of a pose runs before it wraps; the flow itself does not wrap. */
export const FLOW_POSE_LOOP_S = 60;

/** 🔢️ Requests for a refreshed field are quantised to this many per second of clock time. */
export const FLOW_REFRESH_HZ = 8;

/** 🪧️ The layers of the flow view in a stable order. */
export const ALL_FLOW_LAYERS: readonly FlowLayer[] = FLOW_LAYERS;

/** ❓️ Whether a view shows air flow at all: only the thermal view of an asset that emits or moves air. */
export const flowsIn = (asset: Asset, view: View): boolean => view === "thermal" && asset.thermal.emitters.length > 0;

/** 🔤️ The layers of an address code (`fl=s,t,m,f`, `fl=0` for none) and back; absent means all. */
const CODES: Readonly<Record<string, FlowLayer>> = { s: "streams", t: "tracers", m: "moisture", f: "fog" };
export const flowLayersFrom = (code: string | null): readonly FlowLayer[] => (code === null ? ALL_FLOW_LAYERS : ALL_FLOW_LAYERS.filter((layer) => code.split(",").some((part) => CODES[part] === layer)));
export const flowLayersCode = (layers: readonly FlowLayer[]): string | null => (layers.length === ALL_FLOW_LAYERS.length ? null : layers.length === 0 ? "0" : ALL_FLOW_LAYERS.filter((layer) => layers.includes(layer)).map((layer) => layer[0]).join(","));

/** 🔁️ The period of the flow of a query: the duration of a looping clip, else 0 (not periodic). */
export function flowPeriodS(asset: Asset, query: EvaluationQuery): number {
  const clip = query.kind === "clip" ? asset.clips.find((candidate) => candidate.id === query.clip) : undefined;
  return clip?.loop === true && clip.durationS > 0 ? clip.durationS : 0;
}

/** 🕰️ The time on the transport timeline that a clock time shows, given the last time shown (`null` at the start): see the module text. */
export function flowTimeS(query: EvaluationQuery, periodS: number, clockS: number, elapsedS: number, lastS: number | null): number {
  if (query.kind === "pose") return FLOW_DEVELOP_S + elapsedS;
  if (periodS <= 0) return Math.max(0, clockS);
  const first = developedTimeS(((clockS % periodS) + periodS) % periodS, FLOW_DEVELOP_S, periodS);
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
