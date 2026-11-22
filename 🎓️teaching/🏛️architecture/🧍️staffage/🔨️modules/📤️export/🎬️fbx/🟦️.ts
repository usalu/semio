/** 🎬️ One clip of an asset at one scale as an FBX (binary 7.4): the solids of the list that the view's mode draws from (tone views from `body`, `thermal` from `thermal`, `forces` and `moments` from `structure`), tessellated by the neutral kernel at the quality of the scale, skinned to the rig, the clip sampled at its own fps (root motion, looping, events) as node curves and, for the analytic views, material colour curves, in one view (default `light-solid`, any of the seven selectable).
 *
 * An FBX is one clip, one scale and one view, one scene: a static pose is a GLB, all scales and modes in one file are a GLB or the model JSON. Lines (feature edges) are not part of an FBX; glyphs of the chosen analytic view are, and with the `thermal` view the developed air flow of the clip's first instant as static stream tubes, arrow heads and puffs (it does not follow the figure while the clip plays). The encoder reports what FBX cannot express as notices, returned with the bytes by {@link writeFbxDetailed}.
 * The root motion of the clip is kept: the file plays the clip the way the viewer does (see the React stage, which follows the figure instead of cutting its travel).
 *
 * @see ../🗿️model/🟦️.ts — naming and options
 * @see ../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🎬️fbx/🟦️.ts — `encodeFbxDetailed`
 */
import type { Asset, Slug } from "../../../🧬️schema/🟦️.ts";
import { tessellate } from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { encodeFbxDetailed, type FbxResult } from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🎬️fbx/🟦️.ts";
import type { JobControl, View } from "../../🔗️contracts/🟦️.ts";
import { type SharedModelOptions, ModelRequestError, modeOf, modelOf } from "../../🧊️volume/🟦️.ts";
import { PHASES, cancellable, jobOf, qualityOf } from "../../🧊️volume/⏱️job/🟦️.ts";
import { trianglesOf } from "../🗿️model/🟦️.ts";

/** ⚙️ The options of one FBX: the clip, the preset `scale` (a key of `asset.volumes`), the view (default `light-solid`) and what the analytic looks read. */
export type FbxFileOptions = Omit<SharedModelOptions, "at" | "poses" | "clips"> & { readonly clip: Slug; readonly scale: number; readonly view?: View };

/** 🔄️ An FBX with its notices and the number of triangles it holds. */
export type FbxFileResult = FbxResult & { readonly triangles: number };

/** 🎞️ One clip at one scale in several views of one mode: the model is built and tessellated once and encoded once per view (`views` must all belong to the same mode). Throws {@link ModelRequestError} for an unknown clip, an unavailable scale or views of different modes; rejects with the signal's reason when cancelled. */
export async function writeFbxViews(asset: Asset, options: Omit<FbxFileOptions, "view"> & { readonly views: readonly View[] }, control: JobControl = {}): Promise<readonly { readonly view: View; readonly result: FbxFileResult }[]> {
  if (!asset.clips.some((c) => c.id === options.clip)) throw new ModelRequestError("model-unknown-clip", options.clip);
  const { clip, views, scale, ...rest } = options;
  if (views.length === 0 || new Set(views.map(modeOf)).size !== 1) throw new ModelRequestError("model-bad-option", "views");
  const job = jobOf(control, [{ label: PHASES.model, weight: 6 }, { label: PHASES.tessellate, weight: 3 }, { label: PHASES.encode, weight: 2 }]);
  return cancellable(control, async () => {
    const procedural = await modelOf(asset, { ...rest, scale, view: views[0]!, at: { kind: "clip", clip, timeS: 0 }, poses: [], clips: [clip] }, job.operation(0));
    const resolved = await tessellate(procedural, qualityOf(asset, scale), job.operation(1));
    const out: { view: View; result: FbxFileResult }[] = [];
    for (const [index, view] of views.entries()) {
      const share = job.operation(2);
      const result = encodeFbxDetailed(resolved, { animation: clip, variant: view, format: "binary" }, { report: (f) => share.report((index + f) / views.length), cancelled: () => share.cancelled() });
      out.push({ view, result: { ...result, triangles: trianglesOf(resolved) } });
    }
    job.finish();
    return out;
  });
}

/** 🚨️ The FBX of one clip of an asset at one scale in one view, the notices of the encoder and the triangle count. Throws {@link ModelRequestError} for an unknown clip or an unavailable scale; rejects with the signal's reason when cancelled. */
export async function writeFbxDetailed(asset: Asset, options: FbxFileOptions, control: JobControl = {}): Promise<FbxFileResult> {
  const { view = "light-solid", ...rest } = options;
  return (await writeFbxViews(asset, { ...rest, views: [view] }, control))[0]!.result;
}

/** 🗂️ The FBX bytes of one clip of an asset at one scale. */
export async function writeFbx(asset: Asset, options: FbxFileOptions, control: JobControl = {}): Promise<Uint8Array> {
  return (await writeFbxDetailed(asset, options, control)).bytes;
}
