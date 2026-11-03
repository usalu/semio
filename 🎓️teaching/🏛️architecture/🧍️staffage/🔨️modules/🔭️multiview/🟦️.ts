/** 🔭️ The multiview resolver: one asset at one query, projected to elevation (side), front and plan in any of the seven modes from ONE evaluated state, or a typed reason why a view is unavailable.
 *
 * {@link resolveViews} evaluates the asset once and solves the analysis the mode needs once (thermal for `thermal`, structure for `forces` and `moments`, transport when layers are asked), then depicts the three viewpoints from that
 * state, so pose, clip time, root motion, scale, environment and attachments cannot differ between the views. {@link resolveDrawing} is the single-viewpoint form of the same path. A view never substitutes another view,
 * another scale or a blank scene: it resolves to a drawing, or to a {@link DrawingUnavailable} with a code, the reason, the limiting feature and the alternatives, each labelled as physically scaled or as a conventional symbol.
 *
 * @see ../🎨️depiction/🔭️views/🟦️.ts — the cameras
 * @see ../🖨️representation/🟦️.ts — scale resolution
 * @see ../🗂️catalog/🟦️.ts — the availability rules `validateAsset` enforces
 */
import { VIEWPOINTS, type Alternative, type Asset, type Slug, type Text, type UnavailableCause, type Viewpoint } from "../../🧬️schema/🟦️.ts";
import type { DepictionOptions, DrawingScene, Fidelity, EvaluatedState, EvaluationQuery, RepresentationResolution, RepresentationSelector, View } from "../🔗️contracts/🟦️.ts";
import { type Analysis, depict, resolveDepiction } from "../🎨️depiction/🟦️.ts";
import { ViewMissingError } from "../🎨️depiction/🔭️views/🟦️.ts";
import { evaluate } from "../🧮️evaluation/🟦️.ts";
import { RepresentationError } from "../🖨️representation/🟦️.ts";
import { solveStructure } from "../🏗️structure/🟦️.ts";
import { solveThermalAt } from "../🌡️thermal/🟦️.ts";
import { solveTransport } from "../💨️transport/🟦️.ts";

/** 🎯️ Which drawing to project: the authored representation of a scale, or a named symbol shown at a scale (`denominator` maps its paper size). */
export type DrawingSource = { readonly kind: "scale"; readonly denominator: number } | { readonly kind: "symbol"; readonly symbol: Slug; readonly denominator: number };



type Distributive<T, K extends PropertyKey> = T extends unknown ? Omit<T, K> : never;

/** 🎛️ The depiction options a caller still chooses: everything of {@link DepictionOptions} except what the resolver derives (`view`, `viewpoint`, `scale`, `representation`). `profile` is required. */
export type DrawOptions = Distributive<DepictionOptions, "view" | "viewpoint" | "scale" | "representation">;

/** ✅️ A resolved drawing: the scene and the one state it was projected from. */
export type ResolvedDrawing = { readonly kind: "drawing"; readonly viewpoint: Viewpoint; readonly mode: View; readonly fidelity: Fidelity; readonly resolution: Extract<RepresentationResolution, { readonly kind: "authored" | "symbol" }>; readonly scene: DrawingScene; readonly state: EvaluatedState };

/** ↪️ An alternative offered for an unavailable view, with what it would be. */
export type AlternativeOffer = { readonly alternative: Alternative; readonly fidelity: Fidelity | "other-asset" };

/** 🚫️ The codes of {@link DrawingUnavailable}: the scale is not drawable, the profile rejects the drawing, the representation lacks that viewpoint, the symbol does not exist, the symbol is not offered at that scale. */
export const UNAVAILABLE_CODES = ["scale-unavailable", "scale-unsuitable", "view-missing", "symbol-unknown", "symbol-scale"] as const;

/** 🚫️ A viewpoint that has no drawing for the request, with the reason; never a substituted or blank drawing. */
export type DrawingUnavailable = {
  readonly kind: "unavailable";
  readonly code: (typeof UNAVAILABLE_CODES)[number];
  readonly viewpoint: Viewpoint;
  readonly mode: View;
  readonly requested: number;
  readonly cause: UnavailableCause | "outside-valid-range" | "unsuitable-for-profile" | "missing-geometry" | "no-such-symbol" | "symbol-not-offered";
  readonly reason: Text;
  readonly limitingFeature: Text | null;
  readonly limitingSizeM: number | null;
  readonly alternatives: readonly AlternativeOffer[];
};

/** 🔭️ The three viewpoints of one evaluated state. `state` is `null` only when no representation could be selected. */
export type MultiView = { readonly state: EvaluatedState | null; readonly analysis: Analysis; readonly views: Readonly<Record<Viewpoint, ResolvedDrawing | DrawingUnavailable>> };

const text = (en: string, de: string): Text => ({ en, de });

function fidelityOf(asset: Asset, alternative: Alternative): AlternativeOffer["fidelity"] {
  if (alternative.kind === "scale") return "physical";
  if (alternative.kind === "asset") return "other-asset";
  return asset.symbols.find((symbol) => symbol.id === alternative.symbol)?.semantics === "true-extent" ? "physical" : "conventional-symbol";
}

const offers = (asset: Asset, alternatives: readonly Alternative[]): readonly AlternativeOffer[] => alternatives.map((alternative) => ({ alternative, fidelity: fidelityOf(asset, alternative) }));

function selectorOf(source: DrawingSource): RepresentationSelector {
  return source.kind === "symbol" ? { kind: "symbol", symbol: source.symbol } : { kind: "scale", scale: { denominator: source.denominator } };
}

function allUnavailable(mode: View, unavailable: Omit<DrawingUnavailable, "viewpoint" | "mode">): MultiView {
  return { state: null, analysis: {}, views: Object.fromEntries(VIEWPOINTS.map((viewpoint) => [viewpoint, { ...unavailable, viewpoint, mode }])) as Record<Viewpoint, DrawingUnavailable> };
}

function probe(asset: Asset, source: DrawingSource, mode: View, options: DrawOptions): { readonly resolution: Extract<RepresentationResolution, { readonly kind: "authored" | "symbol" }> } | { readonly refusal: MultiView } {
  if (source.kind === "symbol") {
    const symbol = asset.symbols.find((candidate) => candidate.id === source.symbol);
    if (symbol === undefined) return { refusal: allUnavailable(mode, { kind: "unavailable", code: "symbol-unknown", requested: source.denominator, cause: "no-such-symbol", reason: text(`Asset "${asset.id}" has no symbol "${source.symbol}".`, `Asset "${asset.id}" hat kein Symbol "${source.symbol}".`), limitingFeature: null, limitingSizeM: null, alternatives: [] }) };
    if (!(symbol.denominators as readonly number[]).includes(source.denominator)) return { refusal: allUnavailable(mode, { kind: "unavailable", code: "symbol-scale", requested: source.denominator, cause: "symbol-not-offered", reason: text(`Symbol "${symbol.id}" is offered at 1:${symbol.denominators.join(", 1:")}, not at 1:${source.denominator}.`, `Symbol "${symbol.id}" wird bei 1:${symbol.denominators.join(", 1:")} angeboten, nicht bei 1:${source.denominator}.`), limitingFeature: null, limitingSizeM: null, alternatives: [] }) };
  }
  try {
    return { resolution: resolveDepiction(asset, { ...options, view: mode, viewpoint: "elevation", scale: { denominator: source.denominator }, representation: selectorOf(source) } as DepictionOptions) };
  } catch (error) {
    if (!(error instanceof RepresentationError)) throw error;
    const r = error.resolution;
    if (r.kind === "unavailable") return { refusal: allUnavailable(mode, { kind: "unavailable", code: "scale-unavailable", requested: r.requested, cause: r.cause, reason: r.reason, limitingFeature: r.limitingFeature, limitingSizeM: r.limitingSizeM, alternatives: offers(asset, r.alternatives) }) };
    return { refusal: allUnavailable(mode, { kind: "unavailable", code: "scale-unsuitable", requested: r.requested, cause: "unsuitable-for-profile", reason: text(`The 1:${r.denominator} drawing violates the print profile "${r.profile}": ${r.suitability.violations.slice(0, 3).map((v) => `${v.check} ${v.measuredMm.toFixed(3)} mm < ${v.requiredMm} mm`).join("; ")}.`, `Die Zeichnung 1:${r.denominator} verletzt das Druckprofil "${r.profile}": ${r.suitability.violations.slice(0, 3).map((v) => `${v.check} ${v.measuredMm.toFixed(3)} mm < ${v.requiredMm} mm`).join("; ")}.`), limitingFeature: null, limitingSizeM: null, alternatives: offers(asset, r.alternatives) }) };
  }
}

function run(asset: Asset, query: EvaluationQuery, viewpoints: readonly Viewpoint[], source: DrawingSource, mode: View, options: DrawOptions): MultiView {
  const picked = probe(asset, source, mode, options);
  if ("refusal" in picked) return picked.refusal;
  const representation = selectorOf(source);
  const state = evaluate(asset, query, representation);
  const environment = options.environmentTimeline ?? options.environment;
  const thermal = mode === "thermal" ? solveThermalAt(asset, query, environment) : undefined;
  const structure = mode === "forces" || mode === "moments" ? solveStructure(state) : undefined;
  const transport = options.transport.length > 0 ? solveTransport(asset, query, environment ?? asset.thermal.environment) : undefined;
  const analysis: Analysis = { ...(thermal ? { thermal } : {}), ...(structure ? { structure } : {}), ...(transport ? { transport } : {}) };
  const views = Object.fromEntries(
    viewpoints.map((viewpoint): [Viewpoint, ResolvedDrawing | DrawingUnavailable] => {
      try {
        const scene = depict(state, { ...options, view: mode, viewpoint, scale: { denominator: source.denominator }, representation } as DepictionOptions, analysis);
        return [viewpoint, { kind: "drawing", viewpoint, mode, fidelity: scene.metadata.fidelity, resolution: picked.resolution, scene, state }];
      } catch (error) {
        if (!(error instanceof ViewMissingError)) throw error;
        return [viewpoint, { kind: "unavailable", code: "view-missing", viewpoint, mode, requested: source.denominator, cause: "missing-geometry", reason: text(`The selected drawing of "${asset.id}" has no ${viewpoint} view; no other view is substituted.`, `Die gewählte Zeichnung von "${asset.id}" hat keine Ansicht "${viewpoint}"; es wird keine andere Ansicht eingesetzt.`), limitingFeature: null, limitingSizeM: null, alternatives: [] }];
      }
    }),
  ) as Record<Viewpoint, ResolvedDrawing | DrawingUnavailable>;
  return { state, analysis, views };
}

/** 🔭️ The three viewpoints of the asset at the query from one evaluated state and one analysis solve; each is a drawing or a typed unavailability. Throws only for programmer errors (unknown pose or clip, malformed asset). */
export function resolveViews(asset: Asset, query: EvaluationQuery, source: DrawingSource, mode: View, options: DrawOptions): MultiView {
  return run(asset, query, VIEWPOINTS, source, mode, options);
}

/** 🖼️ One viewpoint of the asset at the query, through the same single-state path as {@link resolveViews}: `resolveDrawing(asset, query, viewpoint, source, mode, options)`. */
export function resolveDrawing(asset: Asset, query: EvaluationQuery, viewpoint: Viewpoint, source: DrawingSource, mode: View, options: DrawOptions): ResolvedDrawing | DrawingUnavailable {
  return run(asset, query, [viewpoint], source, mode, options).views[viewpoint];
}

export { ViewMissingError } from "../🎨️depiction/🔭️views/🟦️.ts";
export { CAMERAS, nearness, project } from "../🎨️depiction/🔭️views/🟦️.ts";
export type { Camera } from "../🎨️depiction/🔭️views/🟦️.ts";
export type { Fidelity } from "../🔗️contracts/🟦️.ts";
