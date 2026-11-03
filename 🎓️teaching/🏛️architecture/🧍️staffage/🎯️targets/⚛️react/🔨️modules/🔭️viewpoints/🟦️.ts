/** 🔭️ Pure rules of the viewpoints: the choice of side elevation, front elevation, plan (top view) or three views, whether a viewpoint can be drawn for an asset at a scale, and the aligned layout of the three-view sheet.
 *
 * The side elevation is the rig plane (`elevation`), `front` looks along the forward axis and `plan` from above. All three project one evaluated instant at one scale, so a switch only changes the camera: pose, time, seed and scale stay.
 * The three-view sheet is a third-angle projection in the left-handed frame (x ahead, y up, z left): the plan above the side elevation (they share paper x = x; plan up = +z), the front elevation to the RIGHT of the side elevation (they share paper y = height; front paper x = z). Putting the front on the right is what makes the mitre between plan and front consistent: plan up (+z) folds to front right (+z). Sheet exports must use this arrangement; every cell
 * is cut from the same millimetres-per-metre drawing, so a metre measures the same in all three. A viewpoint the asset cannot draw keeps its cell and says why; it is never blank or replaced by another view.
 *
 * @see ../🧮️pipeline/🟦️.ts — the evaluator and `SceneFrame`
 * @see ../🗺️routing/🟦️.ts — the `vp` and `vl` address parameters
 */
import { RepresentationError, VIEWPOINTS, VIEWPOINT_CAPTIONS, ViewMissingError, resolutionLabel, resolveDepiction } from "@teaching/staffage";
import type { Asset, EvaluationQuery, Language, Text, View, Viewpoint } from "@teaching/staffage";
import { translator, type Key, type Params } from "../🌐️i18n/🟦️.ts";
import { depictionOptions, evaluateState, frameOf, sceneOf, type SceneFrame } from "../🧮️pipeline/🟦️.ts";

/** 🧭️ What the viewpoint selector offers: one viewpoint, or all three on one sheet. */
export type ViewpointChoice = Viewpoint | "all";

/** 🧭️ The offered choices in order: side, front, top, all three. */
export const VIEWPOINT_CHOICES: readonly ViewpointChoice[] = [...VIEWPOINTS, "all"];

const both = (key: Key, params?: Params): Text => ({ en: translator("en")(key, params), de: translator("de")(key, params) });

/** 🏷️ The names of the viewpoints and of the sheet, in both languages (capitalised, for labels and accessible names; the three viewpoints are the library's `VIEWPOINT_CAPTIONS`, the one vocabulary, which the bundle entries must equal). */
export const VIEWPOINT_NAMES: Readonly<Record<ViewpointChoice, Text>> = { elevation: VIEWPOINT_CAPTIONS.elevation, front: VIEWPOINT_CAPTIONS.front, plan: VIEWPOINT_CAPTIONS.plan, all: both("viewpoints.name.all") };

/** 🏷️ The names of the viewpoints inside a sentence. */
export const VIEWPOINT_PHRASES: Readonly<Record<ViewpointChoice, Text>> = { elevation: both("viewpoints.phrase.elevation"), front: both("viewpoints.phrase.front"), plan: both("viewpoints.phrase.plan"), all: both("viewpoints.phrase.all") };

/** 🗣️ A sentence of the viewpoint vocabulary in a language, from the app bundles. */
export const viewpointWord = (key: Key, language: Language, params?: Params): string => translator(language)(key, params);

/** ❓️ Whether a string names a viewpoint. */
export const isViewpoint = (value: string | null | undefined): value is Viewpoint => (VIEWPOINTS as readonly string[]).includes(value ?? "");

/** ❓️ Whether a string names a selector choice. */
export const isViewpointChoice = (value: string | null | undefined): value is ViewpointChoice => (VIEWPOINT_CHOICES as readonly string[]).includes(value ?? "");

/** 🧾️ Whether a viewpoint can be drawn, and if not, in words why. */
export type ViewResolution =
  | { readonly viewpoint: Viewpoint; readonly available: true }
  | { readonly viewpoint: Viewpoint; readonly available: false; readonly reason: "scale-unavailable" | "scale-unsuitable" | "view-missing" | "draw-failed"; readonly message: Text; readonly details?: string };

const MISSING: Readonly<Record<Viewpoint, Text>> = { elevation: both("viewpoints.missing.elevation"), front: both("viewpoints.missing.front"), plan: both("viewpoints.missing.plan") };

/** 🧭️ Resolves one viewpoint of an asset at a denominator under the screen profile. A scale the asset does not offer, a scale the profile rejects and a conventional symbol that does not draw the viewpoint are named; nothing is substituted. */
export function resolveViewpoint(asset: Asset, viewpoint: Viewpoint, denominator: number, language: Language = "en"): ViewResolution {
  try {
    const resolution = resolveDepiction(asset, depictionOptions({ view: "dark-line", viewpoint, scale: { denominator }, language }));
    const geometry = resolution.kind === "symbol" ? resolution.symbol.geometry[viewpoint] : resolution.representation.geometry[viewpoint];
    if (geometry === undefined || geometry.length === 0) return { viewpoint, available: false, reason: "view-missing", message: MISSING[viewpoint] };
    return { viewpoint, available: true };
  } catch (error) {
    if (error instanceof RepresentationError) return { viewpoint, available: false, reason: error.resolution.kind === "unavailable" ? "scale-unavailable" : "scale-unsuitable", message: resolutionLabel(error.resolution) };
    throw error;
  }
}

/** 🧭️ The resolution of every viewpoint, in the order of {@link VIEWPOINTS}. */
export const resolveViewpoints = (asset: Asset, denominator: number, language: Language = "en"): Readonly<Record<Viewpoint, ViewResolution>> => ({
  elevation: resolveViewpoint(asset, "elevation", denominator, language),
  front: resolveViewpoint(asset, "front", denominator, language),
  plan: resolveViewpoint(asset, "plan", denominator, language),
});

/** 🧱️ The cells in visual and reading order: the plan above, then the side elevation, then the front elevation to its right, so the tab order follows the grid. */
export const SHEET_ORDER: readonly Viewpoint[] = ["plan", "elevation", "front"];

/** 🧱️ One cell of the three-view sheet: its column and row, the window in paper millimetres around the shared model origin, and the size of that window. */
export type ThreeViewCell = { readonly viewpoint: Viewpoint; readonly column: 1 | 2; readonly row: 1 | 2; readonly frame: SceneFrame; readonly widthMm: number; readonly heightMm: number };

/** 🧱️ The aligned sheet: three cells and the total paper size including the gap between cells. */
export type ThreeViewLayout = { readonly cells: Readonly<Record<Viewpoint, ThreeViewCell>>; readonly widthMm: number; readonly heightMm: number; readonly gapMm: number };

/** 📏️ The gap between two cells in paper millimetres. */
export const THREE_VIEW_GAP_MM = 4;

const EMPTY_SPAN = { min: -40, max: 40 } as const;
const span = (frames: readonly (SceneFrame | null)[], axis: "x" | "y"): { readonly min: number; readonly max: number } => {
  const known = frames.filter((frame): frame is SceneFrame => frame !== null);
  if (known.length === 0) return EMPTY_SPAN;
  return axis === "x" ? { min: Math.min(...known.map((f) => f.minX)), max: Math.max(...known.map((f) => f.maxX)) } : { min: Math.min(...known.map((f) => f.minY)), max: Math.max(...known.map((f) => f.maxY)) };
};

/** 📐️ Places the three windows so the axes line up: the side and top windows share their horizontal span (the forward axis), the side and front windows share their vertical span (the height axis). `null` is a viewpoint that cannot be drawn; its cell keeps the shared spans so the others do not move. */
export function threeViewLayout(frames: Readonly<Record<Viewpoint, SceneFrame | null>>, gapMm: number = THREE_VIEW_GAP_MM): ThreeViewLayout {
  const forward = span([frames.elevation, frames.plan], "x");
  const height = span([frames.elevation, frames.front], "y");
  const lateralX = span([frames.front], "x");
  const lateralY = span([frames.plan], "y");
  const cell = (viewpoint: Viewpoint, column: 1 | 2, row: 1 | 2, x: { min: number; max: number }, y: { min: number; max: number }): ThreeViewCell => ({ viewpoint, column, row, frame: { minX: x.min, maxX: x.max, minY: y.min, maxY: y.max }, widthMm: x.max - x.min, heightMm: y.max - y.min });
  const plan = cell("plan", 1, 1, forward, lateralY);
  const elevation = cell("elevation", 1, 2, forward, height);
  const front = cell("front", 2, 2, lateralX, height);
  return { cells: { elevation, front, plan }, widthMm: elevation.widthMm + gapMm + front.widthMm, heightMm: plan.heightMm + gapMm + elevation.heightMm, gapMm };
}

/** 🧾️ The sheet of one instant: whether each viewpoint can be drawn and where the cells sit. */
export type Sheet = { readonly resolutions: Readonly<Record<Viewpoint, ViewResolution>>; readonly layout: ThreeViewLayout };

const FAILED: Text = both("viewpoints.failed");

/** 📐️ The sheet of an asset at the given instants (one or the samples of a clip, so the windows hold the whole motion): each drawable viewpoint is depicted in the dark line view and in `view`, the window is the union around the model origin, and the three windows are aligned. A viewpoint that fails to draw is named as failed and keeps its cell. */
export function sheetOf(asset: Asset, queries: readonly EvaluationQuery[], options: { readonly view: View; readonly denominator: number; readonly language: Language; readonly theme?: "light" | "dark" }): Sheet {
  const resolved = resolveViewpoints(asset, options.denominator, options.language);
  const frames: Record<Viewpoint, SceneFrame | null> = { elevation: null, front: null, plan: null };
  const failed: Partial<Record<Viewpoint, ViewResolution>> = {};
  for (const viewpoint of VIEWPOINTS) {
    if (!resolved[viewpoint].available) continue;
    try {
      const scenes = queries.flatMap((query) => {
        const state = evaluateState(asset, query);
        const make = (view: View) => sceneOf(state, depictionOptions({ view, viewpoint, scale: { denominator: options.denominator }, language: options.language, ...(options.theme === undefined ? {} : { theme: options.theme }), transparent: true }));
        return options.view === "dark-line" ? [make("dark-line")] : [make("dark-line"), make(options.view)];
      });
      frames[viewpoint] = frameOf(scenes);
    } catch (error) {
      if (error instanceof ViewMissingError) {
        failed[viewpoint] = { viewpoint, available: false, reason: "view-missing", message: MISSING[viewpoint] };
        continue;
      }
      failed[viewpoint] = { viewpoint, available: false, reason: "draw-failed", message: FAILED, details: error instanceof Error ? error.message : String(error) };
    }
  }
  return { resolutions: { ...resolved, ...failed } as Readonly<Record<Viewpoint, ViewResolution>>, layout: threeViewLayout(frames) };
}
