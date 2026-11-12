/** 📦️ Every 3D model of an asset in one ZIP: a GLB (`glb/`) and a neutral JSON model (`model/`, `.pose.json`) per named pose, an FBX (`fbx/`) and a neutral JSON model (`.clip.json`) per clip, named like the single exports (`<asset>-<pose|clip>-<view|all>-model-1to<scale>`), packed with the first-party stored-entry ZIP writer.
 *
 * Progress is one bar over all members, cancellation stops before the next member and inside the running one.
 *
 * @see ../🟦️.ts — `writeZip`
 * @see ../../🗿️model/🟦️.ts — naming
 */
import type { Asset } from "../../../../🧬️schema/🟦️.ts";
import type { JobControl, View } from "../../../🔗️contracts/🟦️.ts";
import { type ModelOptions, ModelRequestError } from "../../../🧊️volume/🟦️.ts";
import { writeFbx } from "../../🎬️fbx/🟦️.ts";
import { writeGlb } from "../../🧊️glb/🟦️.ts";
import { modelExtension, modelStem, writeModel } from "../../🗿️model/🟦️.ts";
import { type ZipEntry, writeZip } from "../🟦️.ts";

/** ⚙️ The options of a model bundle: the {@link ModelOptions} that apply to every member and the view (default `light-solid`) of the FBX members. */
export type ModelBundleOptions = Omit<ModelOptions, "at" | "poses" | "clips" | "views"> & { readonly view?: View };

const encoder = new TextEncoder();

/** 🚨️ The ZIP of every pose GLB and model JSON and every clip FBX and model JSON of an asset. Throws {@link ModelRequestError} for an asset without volume; rejects with the signal's reason when cancelled. */
export async function writeModelBundle(asset: Asset, options: ModelBundleOptions, control: JobControl = {}): Promise<Uint8Array> {
  if (asset.volume === undefined) throw new ModelRequestError("model-no-volume", asset.id);
  const { view = "light-solid", ...shared } = options;
  const poses = asset.poses.filter((p) => p.key !== true).map((p) => p.id);
  const clips = asset.clips.map((c) => c.id);
  const members = poses.length * 2 + clips.length * 2;
  const entries: ZipEntry[] = [];
  let index = 0;
  const member = (): JobControl => ({
    ...(control.signal ? { signal: control.signal } : {}),
    progress: (done, total, label) => control.progress?.(index * 1000 + Math.round((done / total) * 1000), members * 1000, label),
  });
  const add = async (name: string, make: () => Promise<Uint8Array | string>): Promise<void> => {
    control.signal?.throwIfAborted();
    const data = await make();
    entries.push({ name, bytes: typeof data === "string" ? encoder.encode(data) : data });
    index += 1;
  };
  for (const pose of poses) {
    await add(`glb/${modelStem(asset.id, pose, "all", options.scale)}.glb`, () => writeGlb(asset, { ...shared, pose }, member()));
    await add(`model/${modelStem(asset.id, pose, "all", options.scale)}.${modelExtension("pose")}`, () => writeModel(asset, { ...shared, pose }, member()));
  }
  for (const clip of clips) {
    await add(`fbx/${modelStem(asset.id, clip, view, options.scale)}.fbx`, () => writeFbx(asset, { ...shared, clip, view }, member()));
    await add(`model/${modelStem(asset.id, clip, "all", options.scale)}.${modelExtension("clip")}`, () => writeModel(asset, { ...shared, clip }, member()));
  }
  control.progress?.(members * 1000, members * 1000, { en: "Packing the model bundle", de: "Modellbündel wird gepackt" });
  return writeZip(entries, `${asset.id} 3D models 1:${options.scale}`);
}
