/** 📦️ Every 3D model of an asset in one ZIP: a GLB (`glb/`) and a neutral JSON model (`model/`, `.pose.json`) per named pose, a neutral JSON model (`.clip.json`) and an FBX (`fbx/`) per scale and view per clip, named like the single exports (`<asset>.<pose>.glb`, `<asset>.<clip>.s<scale>.<view>.fbx`, see ../../🗿️model/🟦️.ts), packed with the first-party stored-entry ZIP writer.
 *
 * Progress is one bar over all members, cancellation stops before the next member and inside the running one.
 *
 * @see ../🟦️.ts — `writeZip`
 * @see ../../🗿️model/🟦️.ts — naming and `plannedFiles`
 */
import type { Asset } from "../../../../🧬️schema/🟦️.ts";
import type { JobControl, View } from "../../../🔗️contracts/🟦️.ts";
import type { SharedModelOptions } from "../../../🧊️volume/🟦️.ts";
import { writeFbx } from "../../🎬️fbx/🟦️.ts";
import { writeGlb } from "../../🧊️glb/🟦️.ts";
import { MEMBER_FOLDER, plannedFiles, writeModel } from "../../🗿️model/🟦️.ts";
import { type ZipEntry, writeZip } from "../🟦️.ts";

/** ⚙️ The options of a model bundle: what the analytic looks read, the authored `scales` and the `views` of the FBX members (default every authored scale and all seven views); the GLB and JSON members always hold every authored scale. */
export type ModelBundleOptions = Omit<SharedModelOptions, "at" | "poses" | "clips"> & { readonly scales?: readonly number[]; readonly views?: readonly View[] };

const encoder = new TextEncoder();

/** 🚨️ The ZIP of every pose GLB and model JSON and every clip model JSON and FBX of an asset. Throws `ModelRequestError` for an asset without an authored scale or an unavailable scale; rejects with the signal's reason when cancelled. */
export async function writeModelBundle(asset: Asset, options: ModelBundleOptions = {}, control: JobControl = {}): Promise<Uint8Array> {
  const { scales, views, ...shared } = options;
  const plan = plannedFiles(asset, { ...(scales === undefined ? {} : { scales }), ...(views === undefined ? {} : { views }) }).filter((f) => f.kind !== "zip");
  const entries: ZipEntry[] = [];
  let index = 0;
  const member = (): JobControl => ({
    ...(control.signal ? { signal: control.signal } : {}),
    progress: (done, total, label) => control.progress?.(index * 1000 + Math.round((done / total) * 1000), plan.length * 1000, label),
  });
  for (const file of plan) {
    control.signal?.throwIfAborted();
    const data =
      file.kind === "glb"
        ? await writeGlb(asset, { ...shared, pose: file.item as string }, member())
        : file.kind === "fbx"
          ? await writeFbx(asset, { ...shared, clip: file.item as string, scale: file.scale!, view: file.view as View }, member())
          : await writeModel(asset, { ...shared, ...(file.subject === "pose" ? { pose: file.item as string } : { clip: file.item as string }) }, member());
    entries.push({ name: `${MEMBER_FOLDER[file.kind as "glb"]}/${file.name}`, bytes: typeof data === "string" ? encoder.encode(data) : data });
    index += 1;
  }
  control.progress?.(plan.length * 1000, plan.length * 1000, { en: "Packing the model bundle", de: "Modellbündel wird gepackt" });
  return writeZip(entries, `${asset.id} 3D models`);
}
