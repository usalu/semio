/** 🧫️ Test-support builder of the transport fixtures: scenario assets (human, machine, tree), environments, stub thermal fields and a synthetic evaluator that places anchors from simple kinematics.
 *
 * The synthetic evaluator replaces `evaluate()` so that transport can be tested independent of the rig, with kinematics stated in the fixture: root x(t) = x0 + vx·t on the absolute timeline,
 * head angle piecewise linear in time, operating levels and emitter strengths piecewise linear. Anchors rotate about the bone pivot `(root x + pivotX, pivotY)`.
 *
 * @see ../../🔨️modules/💨️transport/🟦️.ts — the solver under test
 */
import type { Asset, Environment, Slug } from "../../🧬️schema/🟦️.ts";
import type { EnvironmentTimeline, EvaluatedState, EvaluationQuery, Point, ThermalField } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import scenarios from "./🔣️.json" with { type: "json" };

/** 🧾️ Piecewise-linear keys `[t, value]`. */
export type Keys = readonly (readonly [number, number])[];

/** 🧾️ Kinematics of a scenario asset. */
export type Kinematics = { readonly pivotX: number; readonly pivotY: number; readonly rootX: { readonly x0: number; readonly vx: number }; readonly angleKeysDeg: Keys; readonly operating: Readonly<Record<Slug, Keys>>; readonly emitters: Readonly<Record<Slug, Keys>> };

/** 🧾️ The scenario document. */
export const SCENARIOS = scenarios as unknown as {
  readonly assets: Readonly<Record<"human" | "machine" | "tree" | "carrier", Asset>>;
  readonly environments: { readonly "calm-cold": Environment; readonly breeze: Environment; readonly gust: EnvironmentTimeline };
  readonly thermal: Readonly<Record<"human" | "machine" | "tree" | "carrier", { readonly regions: Record<Slug, { sensibleW: number; latentW: number; surfaceTempC: number }>; readonly totals: { sensibleW: number; latentW: number } }>>;
  readonly kinematics: Readonly<Record<"human" | "machine" | "tree" | "carrier", Kinematics>>;
};

/** 📈️ Linear interpolation of keys, held outside. */
export function lerpKeys(keys: Keys, t: number, fallback: number): number {
  if (keys.length === 0) return fallback;
  if (t <= keys[0]![0]) return keys[0]![1];
  for (let i = 1; i < keys.length; i++) {
    const [t1, v1] = keys[i]!;
    const [t0, v0] = keys[i - 1]!;
    if (t <= t1) return v0 + ((v1 - v0) * (t - t0)) / (t1 - t0);
  }
  return keys[keys.length - 1]![1];
}

/** 🧱️ A synthetic evaluator for a scenario asset with optional kinematic overrides. */
export function syntheticEvaluator(kinematics: Kinematics): (asset: Asset, query: EvaluationQuery) => EvaluatedState {
  return (asset, query) => {
    const t = query.timeS ?? 0;
    const angleDeg = lerpKeys(kinematics.angleKeysDeg, t, 0);
    const a = (angleDeg * Math.PI) / 180;
    const px = kinematics.rootX.x0 + kinematics.rootX.vx * t + kinematics.pivotX;
    const anchors: Record<Slug, { anchor: Asset["anchors"][number]; at: Point }> = {};
    for (const anchor of asset.anchors) anchors[anchor.id] = { anchor, at: { x: px + Math.cos(a) * anchor.x - Math.sin(a) * anchor.y, y: kinematics.pivotY + Math.sin(a) * anchor.x + Math.cos(a) * anchor.y } };
    const bones: Record<string, unknown> = {};
    for (const anchor of asset.anchors) bones[anchor.bone] = { id: anchor.bone, angleRad: a };
    const operating: Record<Slug, number> = {};
    for (const [key, keys] of Object.entries(kinematics.operating)) operating[key] = lerpKeys(keys, t, 0);
    const emitters: Record<Slug, number> = {};
    for (const [key, keys] of Object.entries(kinematics.emitters)) emitters[key] = lerpKeys(keys, t, 1);
    return { asset, query, timeS: t, anchors, bones, thermal: { activityMet: 1, externalWorkW: 0, operating, regions: {}, emitters, breathing: null } } as unknown as EvaluatedState;
  };
}

/** 🌡️ A stub thermal solver returning the fixture's sensible and latent heat per region. */
export function stubThermal(key: "human" | "machine" | "tree" | "carrier"): (state: EvaluatedState, environment: Environment) => ThermalField {
  const t = SCENARIOS.thermal[key];
  return (_state, environment) => ({ environment, regions: Object.fromEntries(Object.entries(t.regions).map(([id, r]) => [id, { region: id, ...r }])), body: null, totals: { netRadiativeW: 0, sensibleW: t.totals.sensibleW, latentW: t.totals.latentW, moistureGH: 0, emitterSensibleW: 0, emitterLatentW: 0 }, assumptions: [], notes: [] }) as unknown as ThermalField;
}

/** 🧱️ Scenario asset with the kinematics overridden by `patch`. */
export function scenario(key: "human" | "machine" | "tree" | "carrier", patch: Partial<Kinematics> = {}): { readonly asset: Asset; readonly options: { readonly evaluate: ReturnType<typeof syntheticEvaluator>; readonly thermal: ReturnType<typeof stubThermal> }; readonly kinematics: Kinematics } {
  const kinematics = { ...SCENARIOS.kinematics[key], ...patch };
  return { asset: SCENARIOS.assets[key], options: { evaluate: syntheticEvaluator(kinematics), thermal: stubThermal(key) }, kinematics };
}
