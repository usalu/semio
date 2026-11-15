/** 🧊️ The declarative volume solids of an asset as neutral kernel solids, one list of one authored scale at a time.
 *
 * A scale of `asset.volumes` holds three lists (`body`, `thermal`, `structure`); {@link listOf} resolves the `{ of }` references of a list to the body solid of the same scale, so the geometry of a thermal or structural solid that reuses a body surface is that body solid by construction.
 * A volume solid is written in the bone-local `(u, v, w)` frame and keeps the geometry the kernel tessellator gives its raw numbers; only the frame changes: the node frame of a bone is `(u, v, -w)` (see ../🧭️frame), so every solid is mirrored in `w` through its own `transform` and `repeat` (a negative scale; the kernel re-winds the triangles to CCW outward).
 * Staffage-only properties (`bone`, `material`, `role`, `tone`, `region`, `layer`, `member`, `part`) are dropped or become tags: `role`, `region-<id>`, `layer-<id>`, `member-<id>`, `part-<kind>` and the solid's own tags.
 * There is no filter by scale: every scale is individually authored, never a filtered copy of another.
 *
 * Loft sections pass through unchanged: each lies in the plane of its own `rotation` as the neutral kernel states (the frame change is the mirror of the solid).
 *
 * @see ../../../🧬️schema/🟦️.ts — `AuthoredVolume`, `VolumeSolid`, `ThermalSolid`, `StructuralSolid`
 * @see ../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧬️schema/🟦️.ts — `Solid`
 */
import type { AuthoredVolume, RepresentationKey, ScaleVolume, Slug, StructuralPart, VolumeList, VolumeSolid } from "../../../🧬️schema/🟦️.ts";
import type { Solid } from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { mirrorRepeat, mirrorTransform } from "../🧭️frame/🟦️.ts";
import { ModelRequestError } from "../🚫️refusal/🟦️.ts";

/** 🧩️ A solid of a resolved list: the volume solid with the binding of its list (`region` and `layer` of a thermal solid, `member` and `part` of a structural one). */
export type ListSolid = VolumeSolid & { readonly region?: Slug; readonly layer?: Slug; readonly member?: Slug; readonly part?: StructuralPart };

export { authoredScales } from "../../🔗️contracts/🟦️.ts";

/** 🔌️ The scales a model can be asked for, as the keys of `asset.volumes`. */
export const SCALE_KEYS: readonly RepresentationKey[] = ["50", "100", "200", "500"];

/** 🔍️ The authored volume of a scale. Throws `model-scale-unknown` for a denominator that is not a preset scale and `model-scale-unavailable` for a scale whose volume is unavailable: there is no fallback to another scale. */
export function authoredAt(volumes: Readonly<Record<string, ScaleVolume>> | undefined, scale: number, asset: Slug): AuthoredVolume {
  const key = `${scale}`;
  if (!SCALE_KEYS.includes(key as RepresentationKey)) throw new ModelRequestError("model-scale-unknown", `${asset} 1:${scale}`);
  const found = volumes?.[key];
  if (found === undefined || found.status !== "authored") throw new ModelRequestError("model-scale-unavailable", `${asset} 1:${scale}`);
  return found;
}

/** 🔗️ The solids of a list with the `{ of }` references resolved to the geometry of their body solid (the reference keeps its own id and binding); throws `model-bad-reference` for a reference that names no body solid. */
export function listOf(volume: AuthoredVolume, list: VolumeList): readonly ListSolid[] {
  const body = new Map<Slug, VolumeSolid>(volume.body.map((solid) => [solid.id, solid]));
  if (list === "body") return volume.body;
  return (volume[list] as readonly ({ readonly id: Slug } & Record<string, unknown>)[]).map((entry) => {
    if (!("of" in entry)) return entry as unknown as ListSolid;
    const target = body.get(entry.of as Slug);
    if (target === undefined) throw new ModelRequestError("model-bad-reference", `${list}/${entry.id} -> ${entry.of as string}`);
    const { of: _of, id, ...binding } = entry;
    return { ...target, id, ...binding } as ListSolid;
  });
}

/** 🏷️ The tags of a solid: role, thermal region and layer, structural member and part, then its own tags. */
const tagsOf = (solid: ListSolid): readonly Slug[] => [solid.role, ...(solid.region === undefined ? [] : [`region-${solid.region}`]), ...(solid.layer === undefined ? [] : [`layer-${solid.layer}`]), ...(solid.member === undefined ? [] : [`member-${solid.member}`]), ...(solid.part === undefined ? [] : [`part-${solid.part}`]), ...(solid.tags ?? [])];

/** 🔺️ One volume solid as a kernel solid on node `node` (default its bone) with material `material`. */
export function solidOf(solid: ListSolid, material: Slug, node: Slug = solid.bone): Solid {
  const common = {
    id: solid.id,
    node,
    material,
    tags: tagsOf(solid),
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
