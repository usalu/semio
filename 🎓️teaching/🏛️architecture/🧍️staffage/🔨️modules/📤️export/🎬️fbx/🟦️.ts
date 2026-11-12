/** 🎬️ One clip of an asset as an FBX (binary 7.4): the volume solids tessellated by the neutral kernel, skinned to the rig, the clip sampled at its own fps (root motion, looping, events) as node curves and, for the analytic views, material colour curves, in one view (default `light-solid`, any of the seven selectable).
 *
 * An FBX is one clip: a static pose is a GLB. Lines (feature edges) are not part of an FBX; glyphs of the chosen analytic view are. The encoder reports what FBX cannot express as notices, returned with the bytes by {@link writeFbxDetailed}.
 *
 * @see ../🗿️model/🟦️.ts — naming and options
 * @see ../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🎬️fbx/🟦️.ts — `encodeFbxDetailed`
 */
import type { Asset, Slug } from "../../../🧬️schema/🟦️.ts";
import { tessellate } from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { encodeFbxDetailed, type FbxResult } from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🎬️fbx/🟦️.ts";
import type { JobControl, View } from "../../🔗️contracts/🟦️.ts";
import { type ModelOptions, ModelRequestError, modelOf } from "../../🧊️volume/🟦️.ts";
import { PHASES, cancellable, jobOf, qualityOf } from "../../🧊️volume/⏱️job/🟦️.ts";
import { trianglesOf } from "../🗿️model/🟦️.ts";

/** ⚙️ The options of one FBX: the clip, the view (default `light-solid`) and the {@link ModelOptions} that apply to one view of a clip. */
export type FbxFileOptions = Omit<ModelOptions, "at" | "poses" | "clips" | "views"> & { readonly clip: Slug; readonly view?: View };

/** 🔄️ An FBX with its notices and the number of triangles it holds. */
export type FbxFileResult = FbxResult & { readonly triangles: number };

/** 🚨️ The FBX of one clip of an asset, the notices of the encoder and the triangle count. Throws {@link ModelRequestError} for an unknown clip or an asset without volume; rejects with the signal's reason when cancelled. */
export async function writeFbxDetailed(asset: Asset, options: FbxFileOptions, control: JobControl = {}): Promise<FbxFileResult> {
  if (!asset.clips.some((c) => c.id === options.clip)) throw new ModelRequestError("model-unknown-clip", options.clip);
  const { clip, view = "light-solid", ...rest } = options;
  const job = jobOf(control, [{ label: PHASES.model, weight: 6 }, { label: PHASES.tessellate, weight: 3 }, { label: PHASES.encode, weight: 2 }]);
  return cancellable(control, async () => {
    const procedural = await modelOf(asset, { ...rest, at: { kind: "clip", clip, timeS: 0 }, poses: [], clips: [clip], views: [view] }, job.operation(0));
    const resolved = await tessellate(procedural, qualityOf(asset), job.operation(1));
    const result = encodeFbxDetailed(resolved, { animation: clip, variant: view, format: "binary" }, job.operation(2));
    job.finish();
    return { ...result, triangles: trianglesOf(resolved) };
  });
}

/** 🗂️ The FBX bytes of one clip of an asset. */
export async function writeFbx(asset: Asset, options: FbxFileOptions, control: JobControl = {}): Promise<Uint8Array> {
  return (await writeFbxDetailed(asset, options, control)).bytes;
}
