/** 🖨️ Representation resolution and print suitability: which individually authored drawing an asset offers for a requested scale under a print profile, and whether it prints.
 *
 * {@link resolveRepresentation} picks the decision (exact preset, derived inside `validRange`, or unavailable with the supported scales and alternatives), then measures
 * {@link measureSuitability} on the evaluated paper geometry at the requested scale over **all named poses**: pens and hatch pens against `minStrokeMm`, hatch spacing against
 * `minHatchSpacingMm`, the ink gap between separate outlines and negative-space slivers inside the inked silhouette against `minGapMm` (touching or overlapping ink is a join, never a gap), and the feature width of filled shapes against `minFeatureMm` (definitions in the measure module).
 * An authored representation that violates the profile resolves to `unsuitable` with every violation and alternatives; nothing is substituted silently. Results are cached per
 * (asset, scale, thresholds).
 *
 * @see ./📐️measure/🟦️.ts — the measurements
 * @see ./🧭️pick/🟦️.ts — selection without a profile
 * @see ../🔗️contracts/🟦️.ts — `RepresentationResolution`, `Suitability`, `Violation`
 */
import type { Asset, AuthoredRepresentation, PrintProfile, Shape, Slug, SymbolRepresentation } from "../../🧬️schema/🟦️.ts";
import type { DrawingScale, RepresentationResolution, Suitability, Violation } from "../🔗️contracts/🟦️.ts";
import { outlineRuns, prepare, sourceOf } from "../🎨️depiction/🧩️shapes/🟦️.ts";
import { pointInside } from "../🎨️depiction/📐️geometry/🟦️.ts";
import { evaluate } from "../🧮️evaluation/🟦️.ts";
import { type PaperShape, featureWidthMm, gapsBetween, whiteSliver } from "./📐️measure/🟦️.ts";
import { RepresentationError, pickRepresentation, scaleAlternatives, supportedScales } from "./🧭️pick/🟦️.ts";

export { DEFAULT_PRINT_PROFILE, PRINT_PROFILES, printProfile } from "./🧾️profiles/🟦️.ts";
export { RepresentationError, pickRepresentation, scaleAlternatives, supportedScales } from "./🧭️pick/🟦️.ts";
export { resolutionLabel, resolutionText } from "./🏷️label/🟦️.ts";
export { compareRepresentations } from "./🪞️copies/🟦️.ts";
export type { Similarity } from "./🪞️copies/🟦️.ts";
export { featureWidthMm, gapsBetween, squaredDistanceTransform, whiteSliver } from "./📐️measure/🟦️.ts";
export type { Gap, PaperShape, Sliver, Stroke } from "./📐️measure/🟦️.ts";

/** 🔍️ How far apart (ink gap, millimetres) two outlines may be and still be measured as a gap. */
export const GAP_SEARCH_MM = 1;

/** 🖌️ The pen in paper millimetres a shape is drawn with in `representation`, or `null` when its role is not drawn. */
export function penOf(pens: AuthoredRepresentation["pens"] | SymbolRepresentation["pens"], shape: Shape): number | null {
  return shape.penMm ?? pens[shape.role];
}

const CACHE = new WeakMap<Asset, Map<string, Suitability>>();

function thresholdKey(requested: number, profile: PrintProfile): string {
  return `${requested}|${profile.id}|${profile.minStrokeMm}|${profile.minGapMm}|${profile.minFeatureMm}|${profile.minHatchSpacingMm}`;
}

function measureOnce(asset: Asset, rep: AuthoredRepresentation, requested: number, profile: PrintProfile): Suitability {
  const violations: Violation[] = [];
  let minStroke = Number.POSITIVE_INFINITY;
  let minGap = Number.POSITIVE_INFINITY;
  let minFeature = Number.POSITIVE_INFINITY;
  const strokeSeen = new Set<string>();
  const stroke = (shapes: readonly Slug[], measuredMm: number): void => {
    minStroke = Math.min(minStroke, measuredMm);
    const key = `${shapes.join(",")}|${measuredMm}`;
    if (measuredMm < profile.minStrokeMm - 1e-9 && !strokeSeen.has(key)) {
      strokeSeen.add(key);
      violations.push({ check: "stroke", shapes, measuredMm, requiredMm: profile.minStrokeMm });
    }
  };
  const hatches = new Map(rep.hatches.map((hatch) => [hatch.id, hatch]));
  const hatchSeen = new Set<string>();
  for (const shape of rep.geometry.elevation) {
    const pen = penOf(rep.pens, shape);
    if (pen === null) continue;
    stroke([shape.id], pen);
    const hatch = shape.hatch === undefined ? undefined : hatches.get(shape.hatch);
    if (hatch === undefined || hatchSeen.has(hatch.id)) continue;
    hatchSeen.add(hatch.id);
    stroke([shape.id], hatch.penMm);
    hatchSpacing(hatch.spacingMm, shape.id);
  }
  function hatchSpacing(spacingMm: number, shape: Slug): void {
    if (spacingMm < profile.minHatchSpacingMm - 1e-9) violations.push({ check: "hatch", shapes: [shape], measuredMm: spacingMm, requiredMm: profile.minHatchSpacingMm });
  }
  if (profile.minGapMm <= 0 && profile.minFeatureMm <= 0) {
    return { profile, suitable: violations.length === 0, violations, minStrokeMm: minStroke, minGapMm: minGap, minFeatureMm: minFeature };
  }
  const gapWorst = new Map<string, Violation>();
  const featureWorst = new Map<Slug, Violation>();
  const k = 1000 / requested;
  const poses = asset.poses.filter((pose) => !pose.key);
  for (const pose of poses) {
    const state = evaluate(asset, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: requested } });
    const prepared = prepare(state, "elevation", sourceOf(state), k);
    const filled = prepared.filter((p) => p.closed && p.shape.filled && p.shape.role !== "ground");
    const byId = new Map(prepared.map((p) => [p.shape.id, p]));
    for (const p of prepared) {
      if (p.shape.role === "ground") continue;
      if (p.closed && p.shape.filled && profile.minFeatureMm > 0) {
        const boxMin = Math.min(p.box.maxX - p.box.minX, p.box.maxY - p.box.minY);
        const raw = featureWidthMm(p.rings, profile.minFeatureMm);
        const width = Math.max(raw, Math.min(boxMin, profile.minFeatureMm)) + p.weightMm;
        minFeature = Math.min(minFeature, width);
        if (width < profile.minFeatureMm - 1e-9) {
          const previous = featureWorst.get(p.shape.id);
          if (previous === undefined || width < previous.measuredMm) featureWorst.set(p.shape.id, { check: "feature", shapes: [p.shape.id], measuredMm: width, requiredMm: profile.minFeatureMm });
        }
      }
    }
    const grouped = new Map<Slug, { rings: (readonly { readonly x: number; readonly y: number }[])[]; penMm: number }>();
    for (const { p, run } of outlineRuns(prepared, true)) {
      if (p.shape.role === "ground") continue;
      let entry = grouped.get(p.shape.id);
      if (entry === undefined) grouped.set(p.shape.id, (entry = { rings: [], penMm: p.weightMm }));
      entry.rings.push(run.closed && run.points.length > 2 ? [...run.points, run.points[0]!] : run.points);
    }
    const paper: PaperShape[] = [...grouped.entries()].map(([id, v]) => ({ id, rings: v.rings, closed: false, penMm: v.penMm }));
    if (filled.length === 1 && paper.length === 1) {
      const f = filled[0]!;
      const sliver = whiteSliver([f.rings], f.rings.map((pts) => ({ points: pts, closed: true, penMm: f.weightMm })), profile.minGapMm);
      if (sliver !== null) {
        minGap = Math.min(minGap, sliver.widthMm);
        const previous = gapWorst.get("negative-space");
        if (previous === undefined || sliver.widthMm < previous.measuredMm) gapWorst.set("negative-space", { check: "gap", shapes: ["negative-space"], measuredMm: sliver.widthMm, requiredMm: profile.minGapMm, at: { xMm: sliver.at.x, yMm: sliver.at.y } });
      }
    }
    for (const gap of gapsBetween(paper, GAP_SEARCH_MM)) {
      const pa = byId.get(gap.a);
      const pb = byId.get(gap.b);
      if (pa === undefined || pb === undefined) continue;
      if (pa.shape.bone !== pb.shape.bone || pa.shape.z !== pb.shape.z || !pa.closed || !pb.closed || !pa.shape.filled || !pb.shape.filled) continue;
      if (pa.box.minX <= pb.box.maxX && pb.box.minX <= pa.box.maxX && pa.box.minY <= pb.box.maxY && pb.box.minY <= pa.box.maxY) continue;
      if (filled.some((f) => pointInside(gap.at, f.rings))) continue;
      minGap = Math.min(minGap, gap.gapMm);
      if (gap.gapMm < profile.minGapMm - 1e-9) {
        const key = [gap.a, gap.b].sort().join("|");
        const previous = gapWorst.get(key);
        if (previous === undefined || gap.gapMm < previous.measuredMm) gapWorst.set(key, { check: "gap", shapes: [gap.a, gap.b], measuredMm: gap.gapMm, requiredMm: profile.minGapMm, at: { xMm: gap.at.x, yMm: gap.at.y } });
      }
    }
  }
  violations.push(...gapWorst.values(), ...featureWorst.values());
  return { profile, suitable: violations.length === 0, violations, minStrokeMm: minStroke, minGapMm: minGap, minFeatureMm: minFeature };
}

/** ✅️ The suitability of an authored representation of the asset drawn at `requested` (paper size follows the requested scale) under a print profile, measured over all named poses; `minGapMm` and `minFeatureMm` are `Infinity` when there is nothing to measure. Cached. */
export function measureSuitability(asset: Asset, representation: AuthoredRepresentation, requested: number, profile: PrintProfile): Suitability {
  let byKey = CACHE.get(asset);
  if (byKey === undefined) CACHE.set(asset, (byKey = new Map()));
  const key = `${representation.denominator}|${thresholdKey(requested, profile)}`;
  let result = byKey.get(key);
  if (result === undefined) byKey.set(key, (result = measureOnce(asset, representation, requested, profile)));
  return result;
}

/** 🖨️ How a scale request resolves for the asset under the profile: `authored` (exact or derived, with its suitability), `unavailable` (the asset's decision or outside every valid range) or `unsuitable` (measured violations of the profile); see the module description. */
export function resolveRepresentation(asset: Asset, scale: DrawingScale, profile: PrintProfile): RepresentationResolution {
  const picked = pickRepresentation(asset, scale.denominator);
  if (picked.kind === "unavailable") return picked;
  const suitability = measureSuitability(asset, picked.representation, picked.requested, profile);
  if (suitability.suitable) return { ...picked, suitability };
  return {
    kind: "unsuitable",
    requested: picked.requested,
    denominator: picked.denominator,
    profile: profile.id,
    suitability,
    supported: supportedScales(asset),
    alternatives: scaleAlternatives(asset, picked.requested).filter((alternative) => alternative.kind !== "scale" || alternative.denominator !== picked.denominator),
  };
}

/** 🔣️ The resolution of an explicitly selected symbol; throws when the asset has no such symbol. */
export function resolveSymbol(asset: Asset, symbol: Slug, requested: number): RepresentationResolution {
  const found = asset.symbols.find((candidate) => candidate.id === symbol);
  if (found === undefined) throw new Error(`asset "${asset.id}" has no symbol "${symbol}"; symbols: ${asset.symbols.map((s) => s.id).join(", ")}`);
  return { kind: "symbol", requested, symbol: found };
}

/** 🚫️ Throws {@link RepresentationError} unless the resolution is drawable (`authored` or `symbol`); returns it otherwise. */
export function requireDrawable(resolution: RepresentationResolution, asset: string): Extract<RepresentationResolution, { readonly kind: "authored" | "symbol" }> {
  if (resolution.kind === "unavailable" || resolution.kind === "unsuitable") throw new RepresentationError(resolution, asset);
  return resolution;
}
