/** 🧫️ Test-support builder of the thermal fixtures: turns the language-agnostic fixture documents (`🧫️fixtures/<case>/🔣️.json`) into minimal evaluated states.
 *
 * Only the fields the thermal solver reads are populated (the others are inert placeholders): `asset.thermal`, `asset.materials`, `asset.dimensions`, thermal inputs, anchor world positions,
 * bone world angles, `timeS` and the figure velocity.
 *
 * @see ../../🔨️modules/🌡️thermal/🟦️.ts — the solver under test
 */
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { Anchor, Asset, Material, Slug, ThermalModel } from "../../🧬️schema/🟦️.ts";
import type { EvaluatedBone, EvaluatedState, Point, ThermalInputs } from "../../🔨️modules/🔗️contracts/🟦️.ts";

/** 🧾️ The state part of a thermal fixture. */
export type ThermalFixtureState = {
  readonly timeS: number;
  readonly thermal: Omit<ThermalInputs, "passengers"> & { readonly passengers?: ThermalInputs["passengers"] };
  readonly anchors: Readonly<Record<Slug, { readonly bone: Slug; readonly x: number; readonly y: number }>>;
  readonly bones: Readonly<Record<Slug, { readonly angleDeg: number }>>;
  readonly comVelocity?: Point;
};

/** 🧾️ The model part of a thermal fixture. */
export type ThermalFixtureAsset = {
  readonly dimensions: { readonly heightM: number; readonly massKg: number };
  readonly materials: readonly Material[];
  readonly thermal: ThermalModel;
};

/** 🧾️ A thermal fixture document: asset model, evaluated-state inputs and the oracle's expectations. */
export type ThermalFixture = {
  readonly id: string;
  readonly oracle: string;
  readonly tolerances: Readonly<Record<string, number>>;
  readonly asset: ThermalFixtureAsset;
  readonly state: ThermalFixtureState;
  readonly expected: unknown;
};

/** 🧱️ Builds the minimal {@link EvaluatedState} of a fixture; `state` overrides parts of the fixture's state (time, emitters, bone angles). */
export function thermalState(fixture: { readonly asset: ThermalFixtureAsset; readonly state: ThermalFixtureState }, override: Partial<ThermalFixtureState> = {}): EvaluatedState {
  const state = { ...fixture.state, ...override };
  const bones: Record<Slug, EvaluatedBone> = {};
  for (const [id, b] of Object.entries(state.bones)) {
    const angleRad = (b.angleDeg * Math.PI) / 180;
    bones[id] = { id, world: [Math.cos(angleRad), Math.sin(angleRad), -Math.sin(angleRad), Math.cos(angleRad), 0, 0], origin: { x: 0, y: 0 }, tip: { x: 0, y: 0 }, angleRad, offset: { angleDeg: 0, dx: 0, dy: 0 } };
  }
  const anchors: Record<Slug, { anchor: Anchor; at: Point }> = {};
  for (const [id, a] of Object.entries(state.anchors)) anchors[id] = { anchor: { id, bone: a.bone, x: 0, y: 0, kind: "emitter", title: { en: id, de: id } }, at: { x: a.x, y: a.y } };
  const asset = { id: fixture.asset.thermal.regions[0]?.id ?? "fixture", materials: fixture.asset.materials, dimensions: { ...fixture.asset.dimensions, widthM: 0.5, depthM: 0.3 }, thermal: fixture.asset.thermal } as unknown as Asset;
  return {
    asset,
    query: { kind: "pose", pose: "fixture" },
    timeS: state.timeS,
    frame: 0,
    phase: null,
    bones,
    shapes: { elevation: [], plan: [] },
    anchors,
    contacts: [],
    loads: [],
    thermal: { ...state.thermal, passengers: state.thermal.passengers ?? {} },
    bounds: { minX: 0, minY: 0, maxX: 1, maxY: 1 },
    respiration: null,
    cycle: 0,
    representation: { kind: "none" },
    kinematics: { comVelocity: state.comVelocity ?? { x: 0, y: 0 }, segmentAccelerations: {} },
  } satisfies EvaluatedState;
}

/** 🗂️ Reads one catalog asset document by id from `🗂️catalog/<category folder>/<id>/🔣️.json` (test support; the catalog loader of 🗂️catalog is async and validates). */
export function catalogAsset(id: string): Asset {
  const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../🗂️catalog");
  for (const folder of readdirSync(root, { withFileTypes: true }))
    if (folder.isDirectory()) {
      const file = join(root, folder.name, id, "🔣️.json");
      if (existsSync(file)) return (JSON.parse(readFileSync(file, "utf8")) as { asset: Asset }).asset;
    }
  throw new Error(`catalog asset ${id} not found`);
}

/** 🗂️ The ids of every catalog asset. */
export function catalogIds(): string[] {
  const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../🗂️catalog");
  return readdirSync(root, { withFileTypes: true }).flatMap((f) => (f.isDirectory() ? readdirSync(join(root, f.name), { withFileTypes: true }).filter((a) => a.isDirectory() && existsSync(join(root, f.name, a.name, "🔣️.json"))).map((a) => a.name) : [])).sort();
}
