/** 🧭️ Pure selection of the authored representation for a requested drawing scale, independent of any print profile.
 *
 * A preset request (500, 200, 100, 50) takes exactly the decision the asset authored for it: an authored representation, or an explicit
 * `unavailable` decision. A custom denominator takes the authored representation whose `validRange` contains it (derived use, labelled
 * `exact: false`; when ranges overlap the one whose own denominator is nearest by `|log(d) − log(preset)|`), else it is unavailable with the
 * supported presets as alternatives. Nothing is ever substituted silently.
 *
 * @see ../../../🧬️schema/🟦️.ts — `Representation`, `AuthoredRepresentation`
 * @see ../../🔗️contracts/🟦️.ts — `RepresentationResolution`
 */
import { SCALE_DENOMINATORS } from "../../../🧬️schema/🟦️.ts";
import type { Alternative, Asset, AuthoredRepresentation, ScaleDenominator } from "../../../🧬️schema/🟦️.ts";
import type { RepresentationResolution } from "../../🔗️contracts/🟦️.ts";

/** 🧭️ The unprofiled decision for a requested scale. */
export type Picked = Extract<RepresentationResolution, { readonly kind: "unavailable" }> | { readonly kind: "authored"; readonly requested: number; readonly denominator: ScaleDenominator; readonly exact: boolean; readonly representation: AuthoredRepresentation };

/** 🚫️ Thrown when a drawing is requested at a scale the asset does not offer (`unavailable`) or the print profile rejects (`unsuitable`); carries the full resolution so a caller can explain it and offer the alternatives. */
export class RepresentationError extends Error {
  readonly resolution: Extract<RepresentationResolution, { readonly kind: "unavailable" | "unsuitable" }>;

  constructor(resolution: Extract<RepresentationResolution, { readonly kind: "unavailable" | "unsuitable" }>, asset: string) {
    super(`asset "${asset}" has no ${resolution.kind === "unavailable" ? "available" : "suitable"} representation at 1:${resolution.requested}`);
    this.name = "RepresentationError";
    this.resolution = resolution;
  }
}

/** 🔢️ The preset denominators at which the asset has an authored representation. */
export function supportedScales(asset: Asset): readonly ScaleDenominator[] {
  return SCALE_DENOMINATORS.filter((denominator) => asset.representations[`${denominator}`]?.status === "authored");
}

/** ↪️ Scale alternatives for a request: the supported presets, nearest first by log distance. */
export function scaleAlternatives(asset: Asset, requested: number): readonly Alternative[] {
  return [...supportedScales(asset)].sort((a, b) => Math.abs(Math.log(a / requested)) - Math.abs(Math.log(b / requested))).map((denominator): Alternative => ({ kind: "scale", denominator }));
}

/** 🧭️ Picks the representation for a requested denominator; see the module description. */
export function pickRepresentation(asset: Asset, requested: number): Picked {
  const supported = supportedScales(asset);
  const preset = SCALE_DENOMINATORS.find((denominator) => denominator === requested);
  if (preset !== undefined) {
    const decision = asset.representations[`${preset}`];
    if (decision?.status === "authored") return { kind: "authored", requested, denominator: preset, exact: true, representation: decision };
    if (decision?.status === "unavailable") return { kind: "unavailable", requested, reason: decision.reason, limitingFeature: decision.limitingFeature, limitingSizeM: decision.limitingSizeM, supported, alternatives: decision.alternatives };
  }
  const inside = supported.filter((denominator) => {
    const range = (asset.representations[`${denominator}`] as AuthoredRepresentation).validRange;
    return requested >= range.minDenominator && requested <= range.maxDenominator;
  });
  if (inside.length > 0) {
    const denominator = inside.reduce((best, candidate) => (Math.abs(Math.log(candidate / requested)) < Math.abs(Math.log(best / requested)) ? candidate : best));
    return { kind: "authored", requested, denominator, exact: false, representation: asset.representations[`${denominator}`] as AuthoredRepresentation };
  }
  return {
    kind: "unavailable",
    requested,
    reason: { en: `1:${requested} lies outside the valid range of every representation of this asset.`, de: `1:${requested} liegt außerhalb des Gültigkeitsbereichs jeder Darstellung dieses Assets.` },
    limitingFeature: null,
    limitingSizeM: null,
    supported,
    alternatives: scaleAlternatives(asset, requested),
  };
}
