/** 📤️ The 3D downloads of the archive, behind one lazy door: one GLB per pose (all seven representations inside), one FBX per clip in the current representation, the neutral model JSON of a pose or a clip, and the ZIP bundle of every model of the asset. Each is made on demand by the library's exporters with progress and cancellation and named like the drawings: `<asset>_<pose|clip>_model_<representation|all>_1-<denominator>.<ext>`, the word `model` standing where a drawing names its viewpoint.
 *
 * @see ../🚪️gate/🟦️.tsx — how it is loaded
 * @see ../../📤️downloads/🟦️.ts — the export engine that calls it
 * @see ../../../../../🔨️modules/📤️export/🧊️glb/🟦️.ts — the library writers
 */
import { ModelRequestError, writeFbxDetailed, writeGlb, writeModel, writeModelBundle } from "@teaching/staffage";
import type { Asset, JobControl, Slug, Text, View } from "@teaching/staffage";
import { spatialFilename, type SpatialExportFormat } from "../../📤️downloads/🟦️.ts";
import { analyticOptionsOf } from "../🏭️prepare/🟦️.ts";
import type { SpatialRequest } from "../🟦️.ts";

/** ⚙️ What a 3D download is made of: the analytic settings of the request, the representation (FBX only), and the pose (GLB) or the clip (FBX) or either (model JSON). */
export type SpatialExportOptions = Pick<SpatialRequest, "scale" | "view" | "environment" | "thermalQuantity" | "forceQuantity" | "range" | "theme"> & { readonly pose?: Slug; readonly clip?: Slug };

/** 📦️ A finished 3D download. */
export type SpatialExport = { readonly bytes: Uint8Array; readonly filename: string; readonly mime: string; readonly notes: readonly Text[]; readonly representation: View | "all" };

/** 🏷️ The MIME type of each 3D format. */
export const SPATIAL_MIME: Readonly<Record<SpatialExportFormat, string>> = { glb: "model/gltf-binary", fbx: "application/octet-stream", model: "application/json", "model-zip": "application/zip" };

const encoder = new TextEncoder();

/** 🗒️ The notices of an encoder, one line per kind: the first message of each code with how many there were (an FBX of a rig of 60 meshes would otherwise repeat one remark 60 times). */
export function groupNotices(notices: readonly { readonly code: string; readonly message: Text }[]): readonly Text[] {
  const groups = new Map<string, { readonly message: Text; count: number }>();
  for (const notice of notices) {
    const found = groups.get(notice.code);
    if (found === undefined) groups.set(notice.code, { message: notice.message, count: 1 });
    else found.count += 1;
  }
  return [...groups.values()].map(({ message, count }) => (count === 1 ? message : { en: `${message.en} (${count} times)`, de: `${message.de} (${count} Mal)` }));
}

/** 🔎️ Why a 3D format cannot be made for an asset, or `null` when it can: no volume, no named pose for a GLB, no clip for an FBX. */
export function spatialRefusal(asset: Asset, format: SpatialExportFormat): ModelRequestError | null {
  if (asset.volume === undefined) return new ModelRequestError("model-no-volume", asset.id);
  if (format === "fbx" && asset.clips.length === 0) return new ModelRequestError("model-no-clips", asset.id);
  if (format === "glb" && !asset.poses.some((pose) => pose.key !== true)) return new ModelRequestError("model-unknown-pose", asset.id);
  return null;
}

/** 🚨️ Makes one 3D download. Throws {@link ModelRequestError} when the request names nothing the format can hold (a GLB without a pose, an FBX without a clip, a model without exactly one of them) and rejects with the signal's reason when cancelled. */
export async function exportSpatial(asset: Asset, format: SpatialExportFormat, options: SpatialExportOptions, control: JobControl = {}): Promise<SpatialExport> {
  const refusal = spatialRefusal(asset, format);
  if (refusal !== null) throw refusal;
  const analytic = analyticOptionsOf(options);
  const done = (bytes: Uint8Array, what: Slug, representation: View | "all", notes: readonly Text[] = []): SpatialExport => ({ bytes, filename: spatialFilename(asset, format, what, representation, options.scale), mime: SPATIAL_MIME[format], notes, representation });
  if (format === "glb") {
    if (options.pose === undefined) throw new ModelRequestError("glb-needs-pose");
    return done(await writeGlb(asset, { ...analytic, pose: options.pose }, control), options.pose, "all");
  }
  if (format === "fbx") {
    if (options.clip === undefined) throw new ModelRequestError("fbx-needs-clip");
    const result = await writeFbxDetailed(asset, { ...analytic, clip: options.clip, view: options.view }, control);
    return done(result.bytes, options.clip, options.view, groupNotices(result.notices));
  }
  if (format === "model") {
    if ((options.pose === undefined) === (options.clip === undefined)) throw new ModelRequestError("model-needs-one");
    const text = await writeModel(asset, { ...analytic, ...(options.pose === undefined ? { clip: options.clip! } : { pose: options.pose }) }, control);
    return done(encoder.encode(text), (options.pose ?? options.clip)!, "all");
  }
  return done(await writeModelBundle(asset, { ...analytic, view: options.view }, control), "bundle", "all");
}
