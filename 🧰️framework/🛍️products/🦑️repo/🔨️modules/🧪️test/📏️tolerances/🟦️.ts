/** 📏️ Numeric tolerances
 *
 * Core tolerance profiles and their resolution.
 */

//#region 🔌️Adapters
import { type ToleranceOverride } from "@semio-tech/framework/test/adapter";
//#endregion 🔌️Adapters

//#region 📏️Tolerance
/**
 * 📏️ Scale-relative tolerance policy. One absolute number cannot serve a micro-scale fixture and a
 * fixture translated a kilometre from the origin at the same time, so the effective tolerance is
 * `max(absolute, relative × reference)` and normalised metrics divide by the reference scale.
 */
export type ToleranceProfile = Readonly<{
  id: string;
  description?: string;
  absoluteLength?: number;
  relativeLength?: number;
  absoluteArea?: number;
  relativeArea?: number;
  absoluteVolume?: number;
  relativeVolume?: number;
  normalizedHausdorffMax?: number;
  normalizedCentroidDistanceMax?: number;
  maxOverrideFactor?: number;
}>;

/** 📏️ The domain-neutral profiles the framework owns. An owner contributes further ones by manifest. */
export const CORE_TOLERANCE_PROFILES: readonly ToleranceProfile[] = [
  { id: "analytic-strict", description: "Analytic primitives whose exact answer is known in closed form; only representation noise is permitted.", absoluteLength: 1e-9, relativeLength: 1e-12, absoluteArea: 1e-9, relativeArea: 1e-11, absoluteVolume: 1e-9, relativeVolume: 1e-11, normalizedHausdorffMax: 1e-9, normalizedCentroidDistanceMax: 1e-10, maxOverrideFactor: 10 },
  { id: "mechanical-standard", description: "Ordinary mechanical solids at millimetre scale — the default for the Boolean corpus.", absoluteLength: 1e-7, relativeLength: 1e-9, absoluteArea: 1e-7, relativeArea: 1e-8, absoluteVolume: 1e-7, relativeVolume: 1e-8, normalizedHausdorffMax: 1e-6, normalizedCentroidDistanceMax: 1e-8, maxOverrideFactor: 10 },
  { id: "contact-sensitive", description: "Exact-contact, tangency and coincident-face cases, where kernels legitimately disagree about the last bits of a classification boundary.", absoluteLength: 1e-6, relativeLength: 1e-8, absoluteArea: 1e-6, relativeArea: 1e-7, absoluteVolume: 1e-6, relativeVolume: 1e-7, normalizedHausdorffMax: 1e-5, normalizedCentroidDistanceMax: 1e-7, maxOverrideFactor: 5 },
  { id: "epsilon-degenerate", description: "Slivers, tiny edges and near-coplanar faces, where the RESULT CLASS is the assertion and the metric is secondary.", absoluteLength: 1e-5, relativeLength: 1e-7, absoluteArea: 1e-5, relativeArea: 1e-6, absoluteVolume: 1e-5, relativeVolume: 1e-6, normalizedHausdorffMax: 1e-4, normalizedCentroidDistanceMax: 1e-6, maxOverrideFactor: 4 },
  { id: "large-coordinate", description: "Geometry translated far from the origin, where absolute error grows with the coordinate and only the relative term is meaningful.", absoluteLength: 1e-4, relativeLength: 1e-9, absoluteArea: 1e-4, relativeArea: 1e-8, absoluteVolume: 1e-4, relativeVolume: 1e-8, normalizedHausdorffMax: 1e-6, normalizedCentroidDistanceMax: 1e-8, maxOverrideFactor: 5 },
  { id: "micro-scale", description: "Sub-millimetre geometry, where an absolute floor sized for millimetres would swallow the whole model.", absoluteLength: 1e-11, relativeLength: 1e-8, absoluteArea: 1e-11, relativeArea: 1e-7, absoluteVolume: 1e-11, relativeVolume: 1e-7, normalizedHausdorffMax: 1e-6, normalizedCentroidDistanceMax: 1e-8, maxOverrideFactor: 5 },
  { id: "real-world-import", description: "Third-party real-world models whose own authoring tolerance is unknown and larger than anything we generate.", absoluteLength: 1e-4, relativeLength: 1e-6, absoluteArea: 1e-4, relativeArea: 1e-5, absoluteVolume: 1e-4, relativeVolume: 1e-5, normalizedHausdorffMax: 1e-4, normalizedCentroidDistanceMax: 1e-5, maxOverrideFactor: 3 },
];

/** 📏️ `max(absolute, relative × reference)` — the one rule every dimensional tolerance is resolved by. */
export function resolveTolerance(absolute: number | undefined, relative: number | undefined, reference: number): number {
  return Math.max(absolute ?? 0, (relative ?? 0) * Math.abs(reference));
}

/** 📏️ The four resolved dimensional tolerances of one profile against one fixture's reference scale. */
export type ResolvedTolerances = Readonly<{ length: number; area: number; volume: number; normalizedHausdorffMax: number; normalizedCentroidDistanceMax: number; overrideFactor: number; overridden: boolean }>;

/**
 * 📏️ Resolves a profile against a measured reference scale and an optional fixture override. The
 * override is CAPPED by the profile's own `maxOverrideFactor`, so a fixture cannot quietly widen its
 * gate until everything passes.
 */
export function resolveToleranceProfile(profile: ToleranceProfile, reference: { diagonal: number; area: number; volume: number }, override?: ToleranceOverride): ResolvedTolerances {
  const cap = profile.maxOverrideFactor ?? 1;
  const factor = override === undefined ? 1 : Math.min(override.factor, cap);
  return {
    length: resolveTolerance(profile.absoluteLength, profile.relativeLength, reference.diagonal) * factor,
    area: resolveTolerance(profile.absoluteArea, profile.relativeArea, reference.area) * factor,
    volume: resolveTolerance(profile.absoluteVolume, profile.relativeVolume, reference.volume) * factor,
    normalizedHausdorffMax: (profile.normalizedHausdorffMax ?? 0) * factor,
    normalizedCentroidDistanceMax: (profile.normalizedCentroidDistanceMax ?? 0) * factor,
    overrideFactor: factor,
    overridden: override !== undefined,
  };
}
//#endregion 📏️Tolerance
