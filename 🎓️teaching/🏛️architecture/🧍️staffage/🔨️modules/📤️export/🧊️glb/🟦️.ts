/** 🧊️ The static pose of an asset as a GLB (binary glTF 2.0): the volume solids tessellated by the neutral kernel and skinned to the rig, the pose baked into the node transforms, all seven views inside as `KHR_materials_variants` (the base materials carry the `light-solid` look a viewer shows without selecting a variant), the feature edges as `lines`
 * primitives under the line views and the glyphs of the analytic views as hidden-by-default solids.
 *
 * A GLB is one static pose: a clip is an FBX. Bytes are deterministic.
 *
 * @see ../🗿️model/🟦️.ts — naming and options
 * @see ../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🧊️glb/🟦️.ts — `encodeGlb`
 */
import type { Asset, Slug } from "../../../🧬️schema/🟦️.ts";
import { featureEdges, tessellate } from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { encodeGlb } from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🧊️glb/🟦️.ts";
import type { JobControl } from "../../🔗️contracts/🟦️.ts";
import { type ModelOptions, ModelRequestError, edgeMaterialOf, modelOf } from "../../🧊️volume/🟦️.ts";
import { PHASES, cancellable, jobOf, qualityOf } from "../../🧊️volume/⏱️job/🟦️.ts";
import { EDGE_ANGLE_RAD, trianglesOf } from "../🗿️model/🟦️.ts";

/** ⚙️ The options of one GLB: the pose and the {@link ModelOptions} that apply to a static pose. */
export type GlbFileOptions = Omit<ModelOptions, "at" | "poses" | "clips"> & { readonly pose: Slug };

/** 🔄️ A GLB with the number of triangles it holds (every triangle primitive, glyphs and all seven variants share one geometry). */
export type GlbResult = { readonly bytes: Uint8Array; readonly triangles: number };

/** 🚨️ The GLB of one pose of an asset and its triangle count. Throws {@link ModelRequestError} for an unknown pose or an asset without volume; rejects with the signal's reason when cancelled. */
export async function writeGlbDetailed(asset: Asset, options: GlbFileOptions, control: JobControl = {}): Promise<GlbResult> {
  if (!asset.poses.some((p) => p.id === options.pose)) throw new ModelRequestError("model-unknown-pose", options.pose);
  const { pose, ...rest } = options;
  const job = jobOf(control, [{ label: PHASES.model, weight: 6 }, { label: PHASES.tessellate, weight: 3 }, { label: PHASES.edges, weight: 2 }, { label: PHASES.encode, weight: 1 }]);
  return cancellable(control, async () => {
    const procedural = await modelOf(asset, { ...rest, at: { kind: "pose", pose }, poses: [pose], clips: [] }, job.operation(0));
    const resolved = await tessellate(procedural, qualityOf(asset), job.operation(1));
    const edged = await featureEdges(resolved, EDGE_ANGLE_RAD, { material: edgeMaterialOf(procedural) }, job.operation(2));
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
