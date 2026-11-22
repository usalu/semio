/** 🧊️ The static pose of an asset as a GLB (binary glTF 2.0) holding every authored scale and mode: one glTF scene per scale and mode (`s50-body`, `s50-thermal`, `s50-structure`, `s100-body`, …) over one skeleton, each tessellated at the quality of its own scale and skinned to the rig, the pose baked into the node transforms, all seven views inside as `KHR_materials_variants` (the base materials carry the `light-solid` look a viewer shows without selecting a variant),
 * the feature edges as `lines` primitives under the line views the glyphs of the structural views as hidden-by-default solids of the structure scenes and the developed air flow of the pose (stream tubes with arrow heads, vapour and fog puffs, see ../../🧊️volume/🌊️flow) as hidden-by-default solids of the thermal scenes, shown by the `thermal` variant.
 *
 * A GLB is one static pose: a clip is an FBX. Bytes are deterministic.
 *
 * @see ../🗿️model/🟦️.ts — naming and options
 * @see ../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🧊️glb/🟦️.ts — `encodeGlb`
 */
import type { Asset, Slug } from "../../../🧬️schema/🟦️.ts";
import { encodeGlb } from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🧊️glb/🟦️.ts";
import type { JobControl } from "../../🔗️contracts/🟦️.ts";
import { type AssetModelOptions, ModelRequestError, modelOfAsset } from "../../🧊️volume/🟦️.ts";
import { PHASES, cancellable, jobOf } from "../../🧊️volume/⏱️job/🟦️.ts";
import { resolvedOf, trianglesOf } from "../🗿️model/🟦️.ts";

/** ⚙️ The options of one GLB: the pose and the {@link AssetModelOptions} that apply to a static pose (`scales` restricts the authored scales inside, default all of them). */
export type GlbFileOptions = Omit<AssetModelOptions, "at" | "poses" | "clips"> & { readonly pose: Slug };

/** 🔄️ A GLB with the number of triangles it holds (every triangle primitive of every scene, glyphs and air flow included; the seven variants share the geometry of their mode). */
export type GlbResult = { readonly bytes: Uint8Array; readonly triangles: number };

/** 🚨️ The GLB of one pose of an asset and its triangle count. Throws {@link ModelRequestError} for an unknown pose, an unavailable scale or an asset without an authored scale; rejects with the signal's reason when cancelled. */
export async function writeGlbDetailed(asset: Asset, options: GlbFileOptions, control: JobControl = {}): Promise<GlbResult> {
  if (!asset.poses.some((p) => p.id === options.pose)) throw new ModelRequestError("model-unknown-pose", options.pose);
  const { pose, ...rest } = options;
  const job = jobOf(control, [{ label: PHASES.model, weight: 6 }, { label: PHASES.tessellate, weight: 3 }, { label: PHASES.edges, weight: 2 }, { label: PHASES.encode, weight: 1 }]);
  return cancellable(control, async () => {
    const procedural = await modelOfAsset(asset, { ...rest, at: { kind: "pose", pose }, poses: [pose], clips: [] }, job.operation(0));
    const edged = await resolvedOf(asset, procedural, job, 1, { shareVertices: true });
    const operation = job.operation(3);
    const bytes = encodeGlb(edged, { pose, animations: false, variants: true }, operation);
    job.finish();
    return { bytes, triangles: trianglesOf(edged) };
  });
}

/** 🗂️ The GLB bytes of one pose of an asset. */
export async function writeGlb(asset: Asset, options: GlbFileOptions, control: JobControl = {}): Promise<Uint8Array> {
  return (await writeGlbDetailed(asset, options, control)).bytes;
}
