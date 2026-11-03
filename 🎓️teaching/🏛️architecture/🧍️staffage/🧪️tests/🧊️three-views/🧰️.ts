/** 🧊️ The three-views guard: every authored representation draws the side (elevation), the front and the top (plan) view, none is empty for any pose or clip instant, and the three views of the default pose are one object (orthographic consistency).
 *
 * The coordinate contract: elevation x is forward and y up, front x is lateral (the left of the asset is +x, centred on the symmetry plane) and y up, plan x is forward (the elevation x) and plan y is lateral (the left is +y, the front x).
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🧊️three-views/🔣️.json — the vectors
 */
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import type { Asset, AuthoredRepresentation, Viewpoint } from "../../🧬️schema/🟦️.ts";
import type { EvaluatedShape, EvaluatedState } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { STAFFAGE_ROOT } from "../🧰️support/🧰️.ts";

/** 👁️ The three views every authored representation draws. */
export const VIEWS: readonly Viewpoint[] = ["elevation", "front", "plan"];

/** ⏱️ The fractions of a clip's duration that are sampled; the last stays inside a non-looping clip. */
export const CLIP_FRACTIONS: readonly number[] = [0, 0.2, 0.4, 0.6, 0.8, 0.999];

/** 📏️ The extent of the drawn shapes of one view. */
export type Extent = { readonly minX: number; readonly maxX: number; readonly minY: number; readonly maxY: number };

/** 🧾️ One violated check of the guard. */
export type Finding = { readonly asset: string; readonly scale: number; readonly check: string; readonly message: string };

/** 🗂️ The catalog asset documents, in id order. */
export function catalog(): Asset[] {
  const root = join(STAFFAGE_ROOT, "🗂️catalog");
  const assets: Asset[] = [];
  for (const group of readdirSync(root, { withFileTypes: true }).filter((entry) => entry.isDirectory()))
    for (const folder of readdirSync(join(root, group.name), { withFileTypes: true }).filter((entry) => entry.isDirectory())) {
      const file = join(root, group.name, folder.name, "🔣️.json");
      if (existsSync(file)) assets.push((JSON.parse(readFileSync(file, "utf8")) as { asset: Asset }).asset);
    }
  return assets.sort((a, b) => a.id.localeCompare(b.id));
}

/** 🔢️ The denominators with an authored representation. */
export function authoredScales(asset: Asset): number[] {
  return Object.values(asset.representations)
    .filter((representation): representation is AuthoredRepresentation => representation.status === "authored")
    .map((representation) => representation.denominator)
    .sort((a, b) => a - b);
}

/** 📐️ The extent of the non-ground shapes of a view (ground shapes too with `withGround`), or undefined when nothing is drawn. */
export function extentOf(shapes: readonly EvaluatedShape[], withGround = false): Extent | undefined {
  const points = shapes.filter((evaluated) => withGround || evaluated.shape.role !== "ground").flatMap((evaluated) => evaluated.rings.flat());
  if (points.length === 0) return undefined;
  const xs = points.map((p) => p.x);
  const ys = points.map((p) => p.y);
  return { minX: Math.min(...xs), maxX: Math.max(...xs), minY: Math.min(...ys), maxY: Math.max(...ys) };
}

/** 🎯️ The tolerance of the consistency checks: the larger of 2 cm and 3 % of the largest dimension. */
export const toleranceM = (asset: Asset): number => Math.max(0.02, 0.03 * Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM));

/** ▶️ Every pose and the sampled instants of every clip, named for the messages. */
export function queries(asset: Asset): { readonly name: string; readonly query: Parameters<typeof evaluate>[1] }[] {
  const poses = asset.poses.map((pose) => ({ name: `pose ${pose.id}`, query: { kind: "pose", pose: pose.id } as const }));
  const clips = asset.clips.flatMap((clip) => CLIP_FRACTIONS.map((fraction) => ({ name: `clip ${clip.id} @${(clip.durationS * fraction).toFixed(2)} s`, query: { kind: "clip", clip: clip.id, timeS: clip.durationS * fraction } as const })));
  return [...poses, ...clips];
}

const select = (denominator: number) => ({ kind: "scale", scale: { denominator } }) as const;
const finiteExtent = (extent: Extent): boolean => Object.values(extent).every(Number.isFinite);

/** 🔍️ The coverage of one authored representation: the views without any authored shape. */
export function missingViews(asset: Asset, denominator: number): Viewpoint[] {
  const { geometry } = asset.representations[String(denominator) as keyof Asset["representations"]] as AuthoredRepresentation;
  return VIEWS.filter((view) => (geometry[view] ?? []).filter((shape) => shape.role !== "ground").length === 0);
}

/** 🧹️ Sweeps every pose and clip instant of one authored representation (instants with a laterally spread or twisted bone must also keep the three views one object: `spread-*` checks): each view that is authored must evaluate to non-empty geometry with finite bounds; a view with no authored shape is reported once. */
export function sweep(asset: Asset, denominator: number): Finding[] {
  const findings: Finding[] = [];
  const add = (check: string, message: string): void => void findings.push({ asset: asset.id, scale: denominator, check, message });
  const missing = missingViews(asset, denominator);
  for (const view of missing) add(`${view}-missing`, `1:${denominator} ${asset.id} authors no ${view} geometry (the ${view} view is mandatory for every authored representation)`);
  for (const { name, query } of queries(asset)) {
    let state: EvaluatedState;
    try {
      state = evaluate(asset, query, select(denominator));
    } catch (error) {
      add("evaluate", `1:${denominator} ${asset.id} ${name} fails to evaluate: ${(error as Error).message}`);
      continue;
    }
    if (missing.length === 0 && Object.values(state.bones).some((bone) => Math.abs(bone.offset.abductionDeg) > 1e-9 || Math.abs(bone.offset.twistDeg) > 1e-9)) {
      const e = extentOf(state.shapes.elevation);
      const f = extentOf(state.shapes.front);
      const p = extentOf(state.shapes.plan);
      const tolerance = toleranceM(asset);
      const agree = (check: string, label: string, a: readonly [number, number], b: readonly [number, number]): void => {
        if (!(Math.max(Math.abs(a[0] - b[0]), Math.abs(a[1] - b[1])) <= tolerance)) add(check, `1:${denominator} ${asset.id} ${name} (laterally spread or twisted, tolerance ${tolerance.toFixed(3)} m): ${label} differ: [${a[0].toFixed(3)}, ${a[1].toFixed(3)}] vs [${b[0].toFixed(3)}, ${b[1].toFixed(3)}] m`);
      };
      if (e !== undefined && f !== undefined && p !== undefined) {
        agree("spread-elevation-x-plan-x", "elevation x and plan x", [e.minX, e.maxX], [p.minX, p.maxX]);
        agree("spread-elevation-y-front-y", "elevation y and front y", [e.minY, e.maxY], [f.minY, f.maxY]);
        agree("spread-front-x-plan-y", "front x and plan y", [f.minX, f.maxX], [p.minY, p.maxY]);
      }
    }
    for (const view of VIEWS.filter((candidate) => !missing.includes(candidate))) {
      const extent = extentOf(state.shapes[view]);
      if (extent === undefined) add(`${view}-empty`, `1:${denominator} ${asset.id} ${name}: ${view} view evaluates to no geometry`);
      else if (!finiteExtent(extent)) add(`${view}-bounds`, `1:${denominator} ${asset.id} ${name}: ${view} view has non-finite bounds ${JSON.stringify(extent)}`);
    }
  }
  return findings;
}

/** 🧭️ The orthographic consistency of the default pose of one authored representation: the three views are one object and match the dimensions. A missing view is reported by {@link sweep}, not here. */
export function consistency(asset: Asset, denominator: number): Finding[] {
  const findings: Finding[] = [];
  const tolerance = toleranceM(asset);
  const add = (check: string, message: string): void => void findings.push({ asset: asset.id, scale: denominator, check, message });
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, select(denominator));
  const e = extentOf(state.shapes.elevation);
  const f = extentOf(state.shapes.front);
  const p = extentOf(state.shapes.plan);
  const where = `1:${denominator} ${asset.id} (pose ${asset.presentation.defaultPose}, tolerance ${tolerance.toFixed(3)} m)`;
  const range = (check: string, label: string, a: readonly [number, number], b: readonly [number, number], names: readonly [string, string]): void => {
    const delta = Math.max(Math.abs(a[0] - b[0]), Math.abs(a[1] - b[1]));
    if (!(delta <= tolerance)) add(check, `${where}: ${label} differ: ${names[0]} [${a[0].toFixed(3)}, ${a[1].toFixed(3)}] vs ${names[1]} [${b[0].toFixed(3)}, ${b[1].toFixed(3)}] m`);
  };
  const size = (check: string, label: string, measured: number, nominal: number): void => {
    if (!(Math.abs(measured - nominal) <= tolerance)) add(check, `${where}: ${label} is ${measured.toFixed(3)} m but the dimensions say ${nominal.toFixed(3)} m`);
  };
  if (e !== undefined && p !== undefined) range("elevation-x-plan-x", "the forward ranges", [e.minX, e.maxX], [p.minX, p.maxX], ["elevation x", "plan x"]);
  if (e !== undefined && f !== undefined) range("elevation-y-front-y", "the height ranges", [e.minY, e.maxY], [f.minY, f.maxY], ["elevation y", "front y"]);
  if (f !== undefined && p !== undefined) range("front-x-plan-y", "the lateral ranges", [f.minX, f.maxX], [p.minY, p.maxY], ["front x", "plan y"]);
  if (e !== undefined) {
    size("elevation-width", "the elevation x extent (widthM)", e.maxX - e.minX, asset.dimensions.widthM);
    size("elevation-height", "the elevation height (heightM)", e.maxY - e.minY, asset.dimensions.heightM);
  }
  if (f !== undefined) {
    size("front-width", "the front lateral extent (depthM)", f.maxX - f.minX, asset.dimensions.depthM);
    size("front-height", "the front height (heightM)", f.maxY - f.minY, asset.dimensions.heightM);
  }
  if (p !== undefined) {
    size("plan-length", "the plan x extent (widthM)", p.maxX - p.minX, asset.dimensions.widthM);
    size("plan-width", "the plan lateral extent (depthM)", p.maxY - p.minY, asset.dimensions.depthM);
  }
  return findings;
}
