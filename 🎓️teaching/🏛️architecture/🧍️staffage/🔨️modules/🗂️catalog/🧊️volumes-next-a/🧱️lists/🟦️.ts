/** 🧱️ The lists of the volumes of an asset: which scales are authored and the solids of each list with their `{ of }` references resolved to full solids.
 *
 * @see ../../../../🧬️schema/🔣️.json — `Volumes`, `AuthoredVolume`, `ThermalSolid`, `StructuralSolid` */
import type { Asset, AuthoredVolume, ScaleDenominator, StructuralSolid, ThermalSolid, VolumeList, VolumeSolid } from "../../../../🧬️schema/🟦️.ts";
import { SCALE_DENOMINATORS } from "../../../../🧬️schema/🟦️.ts";

/** 🔗️ Whether a solid of a list is a `{ of }` reference to a body solid. */
export const isReference = (solid: object): boolean => typeof (solid as { of?: unknown }).of === "string";

/** 🔢️ The preset scales from the finest (1:50) to the coarsest (1:500). */
export const FINE_TO_COARSE: readonly ScaleDenominator[] = [...SCALE_DENOMINATORS].reverse();

/** 🧭️ The authored volumes of an asset, finest scale first; scales that are unavailable or absent are left out. */
export function authoredVolumes(asset: Asset): readonly AuthoredVolume[] {
  const volumes = asset.volumes as Partial<Record<string, AuthoredVolume | { status: "unavailable" }>> | undefined;
  return FINE_TO_COARSE.flatMap((denominator) => {
    const volume = volumes?.[String(denominator)];
    return volume !== undefined && volume.status === "authored" ? [volume as AuthoredVolume] : [];
  });
}

/** 📚️ The solids of one list as full {@link VolumeSolid}s: a reference takes the geometry of the body solid it names (an unresolved reference is dropped, `volumes-bones` reports it), keeps its own id and takes its own `sectionFaces` (a reference is never `slender`). */
export function solidsOf(volume: AuthoredVolume, list: VolumeList): readonly VolumeSolid[] {
  if (list === "body") return volume.body;
  const body = new Map(volume.body.map((solid) => [solid.id, solid] as const));
  const out: VolumeSolid[] = [];
  for (const solid of volume[list] as readonly (ThermalSolid | StructuralSolid)[]) {
    if (!isReference(solid)) {
      out.push(solid as VolumeSolid);
      continue;
    }
    const target = body.get((solid as { of: string }).of);
    if (target === undefined) continue;
    const { sectionFaces: _bodyFaces, slender: _bodySlender, ...shape } = target;
    const faces = (solid as { sectionFaces?: VolumeSolid["sectionFaces"] }).sectionFaces;
    out.push({ ...shape, id: solid.id, ...(faces === undefined ? {} : { sectionFaces: faces }) } as VolumeSolid);
  }
  return out;
}

/** 🧮️ The number of solids once every `repeat` is expanded. */
export const expandedCount = (solids: readonly VolumeSolid[]): number => solids.reduce((sum, solid) => sum + (solid.repeat?.count ?? 1), 0);
