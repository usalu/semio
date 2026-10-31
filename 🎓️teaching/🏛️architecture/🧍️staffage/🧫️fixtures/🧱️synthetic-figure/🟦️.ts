/** 🧱️ Test builders for the depiction cases: the sample figure, and a hand-built 1.80 m figure whose representations each hold one rectangle per shape role with the pens of that scale.
 *
 * @see ../👤️sample-figure/🔣️.json
 */
import { readFileSync } from "node:fs";
import type { Asset, AssetDocument, AuthoredRepresentation, PenSet, PrintProfile, Shape, ShapeRole } from "../../🧬️schema/🟦️.ts";
import type { DepictionOptions, EvaluatedState } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";

/** 🧍️ The sample figure asset (200, 100 and 50 authored, 500 unavailable). */
export const sampleAsset = (): Asset => (JSON.parse(readFileSync(new URL("../👤️sample-figure/🔣️.json", import.meta.url), "utf8")) as AssetDocument).asset;

/** 🧍️ The sample figure evaluated at a pose and scale. */
export const sampleState = (pose = "stand", denominator = 100): EvaluatedState => evaluate(sampleAsset(), { kind: "pose", pose }, { kind: "scale", scale: { denominator } });

/** ✒️ The pens of the role fixture per preset scale (paper millimetres). */
export const ROLE_PENS: Readonly<Record<number, PenSet>> = {
  500: { body: 0.18, detail: 0.13, texture: null, glazing: 0.13, ground: 0.25 },
  200: { body: 0.25, detail: 0.13, texture: 0.13, glazing: 0.13, ground: 0.35 },
  100: { body: 0.35, detail: 0.18, texture: 0.13, glazing: 0.18, ground: 0.5 },
  50: { body: 0.5, detail: 0.25, texture: 0.13, glazing: 0.25, ground: 0.7 },
};

const ROOT = "ground";

const rect = (id: string, role: ShapeRole, x: number, h: number): Shape => ({ id, bone: ROOT, role, z: 0, filled: true, tone: 0.5, kind: "rect", x, y: 0, w: 0.25, h });

const stillPose = (id: string, source: Asset["poses"][number]): Asset["poses"][number] => ({ ...source, id, bones: {}, root: undefined, contacts: [], loads: [], thermal: { regions: {}, emitters: {} } }) as unknown as Asset["poses"][number];

/** 🧍️ A minimal v2 asset with one fixed root bone `ground` at the origin and no thermal regions, whose four preset scales are authored with the given shapes (default: `figure` body 1.80 m, `detail`, `texture` and `glass` (glazing) rectangles) and the pens of {@link ROLE_PENS}; for cases that measure or draw geometry without rig or physics. `change` adjusts a representation. */
export function rolesAssetWith(shapesOf: (denominator: number) => readonly Shape[] = () => defaultShapes(), change: (rep: AuthoredRepresentation) => AuthoredRepresentation = (rep) => rep): Asset {
  const base = sampleAsset();
  const template = base.representations["100"] as AuthoredRepresentation;
  const ranges: Record<number, [number, number]> = { 500: [350, 1000], 200: [150, 349], 100: [75, 149], 50: [20, 74] };
  const representations = Object.fromEntries([500, 200, 100, 50].map((denominator) => [`${denominator}`, change({ ...template, denominator: denominator as 50, validRange: { minDenominator: ranges[denominator]![0], maxDenominator: ranges[denominator]![1] }, pens: ROLE_PENS[denominator]!, hatches: [], geometry: { elevation: shapesOf(denominator) } })]));
  const { respiration: _respiration, body: _body, ...thermal } = base.thermal;
  return {
    ...base,
    rig: { kind: "mechanical", root: ROOT, bones: [{ id: ROOT, parent: null, title: { en: "Ground", de: "Boden" }, x: 0, y: 0, angleDeg: 0, lengthM: 1, joint: { type: "fixed" } }] },
    anchors: [],
    representations: representations as unknown as Asset["representations"],
    symbols: [],
    poses: ["stand", "walk-a", "walk-b"].map((id) => stillPose(id, base.poses[0]!)),
    clips: base.clips.map((clip) => ({ ...clip, keys: [{ t: 0, pose: "stand", ease: "linear" as const }], contacts: [], phases: [], events: [], tracks: [], rootMotion: "none" as const })),
    thermal: { ...thermal, regions: [], emitters: [], operating: [], passengers: [] },
    structure: { ...base.structure, supports: [] },
  };
}

/** 🧱️ The default role shapes: `figure` (body, 1.80 m), `detail`, `texture` and `glass` rectangles side by side. */
export const defaultShapes = (): Shape[] => [rect("figure", "body", 0, 1.8), rect("detail", "detail", 0.5, 0.5), rect("texture", "texture", 1, 0.5), rect("glass", "glazing", 1.5, 0.5)];

/** 🧍️ The roles asset with the default shapes. */
export const rolesAsset = (): Asset => rolesAssetWith();

/** 🧍️ The roles asset evaluated at a scale. */
export const rolesState = (denominator = 100): EvaluatedState => evaluate(rolesAsset(), { kind: "pose", pose: "stand" }, { kind: "scale", scale: { denominator } });

/** 🧍️ The roles asset at a scale; the `figure` shape is the 1.80 m reference. */
export const figureState = rolesState;

/** 🎞️ An asset with the respiratory chest motion removed (it runs on the absolute timeline and is not periodic in a clip) and the idle clip's plume track of the original gait fixture, for the clip sampling cases. */
export function samplingAsset(source: Asset = sampleAsset()): Asset {
  const plume = { target: "emitter:body-plume", ease: "smooth" as const, keys: [{ t: 0, v: 0 }, { t: 2, v: 0 }, { t: 2.4, v: 1 }, { t: 3.6, v: 0.4 }, { t: 4, v: 0 }] };
  const { respiration: _removed, ...thermal } = source.thermal;
  return { ...source, thermal, clips: source.clips.map((clip) => (clip.id === "idle-breathing" ? { ...clip, tracks: [plume] } : clip)) };
}

/** 🖨️ A profile without limits: geometry and selection cases use it so a figure that is only a plumbing fixture is not rejected by the measured suitability. */
export const UNCHECKED_PROFILE: PrintProfile = { id: "unchecked", title: { en: "Unchecked", de: "Ungeprüft" }, description: { en: "Fixture profile without limits.", de: "Fixture-Profil ohne Grenzen." }, minStrokeMm: 0, minGapMm: 0, minFeatureMm: 0, minHatchSpacingMm: 0, source: "illustrative" };

/** 🎛️ The options with the unchecked profile. */
export const unchecked = (options: DepictionOptions): DepictionOptions => ({ ...options, profile: UNCHECKED_PROFILE });
