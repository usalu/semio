/** 🧊️ The declarative volume solids of an asset as neutral kernel solids.
 *
 * A volume solid is written in the bone-local `(u, v, w)` frame and keeps the geometry the kernel tessellator gives its raw numbers; only the frame changes: the node frame of a bone is `(u, v, -w)` (see ../🧭️frame), so every solid is mirrored in `w` through its own `transform` and `repeat` (a negative scale; the kernel re-winds the triangles to CCW outward).
 * Staffage-only properties (`bone`, `material`, `role`, `tone`, `region(s)`, `maxDenominator`) are dropped or become tags: `role`, `region-<id>` for every thermal region and the solid's own tags.
 *
 * Loft sections pass through unchanged: each lies in the plane of its own `rotation` as the neutral kernel states (the frame change is the mirror of the solid).
 *
 * `maxDenominator` filters by scale: a solid whose `maxDenominator` is below the requested denominator is not part of the model at that scale (the coarsest preset scale that still shows it).
 *
 * @see ../../../🧬️schema/🟦️.ts — `VolumeSolid`
 * @see ../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧬️schema/🟦️.ts — `Solid`
 */
import type { Slug, Volume, VolumeSolid } from "../../../🧬️schema/🟦️.ts";
import type { Solid } from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { mirrorRepeat, mirrorTransform } from "../🧭️frame/🟦️.ts";

/** 🌿️ The thermal regions of a solid: `region` then `regions`, without duplicates. */
export const regionsOfSolid = (solid: Pick<VolumeSolid, "region" | "regions">): readonly Slug[] => [...new Set([...(solid.region === undefined ? [] : [solid.region]), ...(solid.regions ?? [])])];

/** 👁️ Whether a solid is part of the model at the preset denominator: `maxDenominator` is the largest denominator that still shows it. */
export const shownAt = (solid: Pick<VolumeSolid, "maxDenominator">, denominator: number): boolean => solid.maxDenominator === undefined || denominator <= solid.maxDenominator;

/** 🔺️ One volume solid as a kernel solid on node `bone` with material `material`. */
export function solidOf(solid: VolumeSolid, material: Slug): Solid {
  const regions = regionsOfSolid(solid);
  const common = {
    id: solid.id,
    node: solid.bone,
    material,
    tags: [solid.role, ...regions.map((r) => `region-${r}`), ...(solid.tags ?? [])],
    transform: mirrorTransform(solid.transform),
    ...(solid.repeat === undefined ? {} : { repeat: mirrorRepeat(solid.repeat)! }),
    ...(solid.weights === undefined ? {} : { weights: solid.weights.map((w) => ({ node: w.bone, weight: w.weight })) }),
  };
  switch (solid.kind) {
    case "box":
      return { ...common, kind: "box", size: solid.size, ...(solid.cornerRadius === undefined ? {} : { cornerRadius: solid.cornerRadius }) };
    case "ellipsoid":
      return { ...common, kind: "ellipsoid", radii: solid.radii };
    case "capsule":
      return { ...common, kind: "capsule", p0: solid.p0, p1: solid.p1, r0: solid.r0, r1: solid.r1, ...(solid.lateralScale === undefined ? {} : { lateralScale: solid.lateralScale }) };
    case "revolve":
      return { ...common, kind: "revolve", profile: solid.profile, axis: solid.axis, origin: solid.origin, ...(solid.sweepRad === undefined ? {} : { sweepRad: solid.sweepRad }), ...(solid.lateralScale === undefined ? {} : { lateralScale: solid.lateralScale }) };
    case "extrude":
      return { ...common, kind: "extrude", profile: solid.profile, ...(solid.holes === undefined ? {} : { holes: solid.holes }), plane: solid.plane, from: solid.from, to: solid.to, ...(solid.taperTo === undefined ? {} : { taperTo: solid.taperTo }) };
    case "loft":
      return { ...common, kind: "loft", sections: solid.sections, capStart: solid.capStart, capEnd: solid.capEnd };
  }
}

/** 📏️ The solids of a volume shown at a denominator, in authored order. */
export const shownSolids = (volume: Volume, denominator: number): readonly VolumeSolid[] => volume.solids.filter((solid) => shownAt(solid, denominator));
