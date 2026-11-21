/** 🗿️ The neutral model as a file: the `semio.model/v1` JSON document of one pose or one clip of an asset with every authored scale and mode as a scene, procedural (declarative solids, the exact data the GLB and FBX are made from) or, on request, resolved (tessellated meshes at the quality of each scene's scale, with feature edges).
 *
 * The file set of an asset (one name rule for the library, the batch, the command line and the React downloads, see ./🏷️names):
 * `<asset>.<pose>.glb` (every authored scale and mode as glTF scenes, the seven views as material variants), `<asset>.<pose>.pose.json` and `<asset>.<clip>.clip.json` (the multi-scene model document of a pose and of a clip; a pose and a clip of one asset may share an id, so the file says which it holds),
 * `<asset>.<clip>.s<scale>.<view>.fbx` (one clip at one scale in one view, one scene per file) and `<asset>.zip` (all of them in `glb/`, `fbx/` and `model/`).
 *
 * @see ../../🧊️volume/🟦️.ts — `modelOfAsset`
 * @see ./🧩️scenes/🟦️.ts — the scene-wise tessellation
 */
import type { Asset, Slug } from "../../../🧬️schema/🟦️.ts";
import { featureEdges, type ModelDocument } from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { VIEWS, type JobControl, type View } from "../../🔗️contracts/🟦️.ts";
import { type AssetModelOptions, type SharedModelOptions, ModelRequestError, authoredAt, authoredScales, edgeMaterialOf, modelOfAsset } from "../../🧊️volume/🟦️.ts";
import { PHASES, cancellable, jobOf, qualityOf, type Job } from "../../🧊️volume/⏱️job/🟦️.ts";
import { fbxName, glbName, modelName, zipName } from "./🏷️names/🟦️.ts";
import { resolveScenes } from "./🧩️scenes/🟦️.ts";

export { scaleOfScene } from "./🧩️scenes/🟦️.ts";
export { fbxName, glbName, modelName, zipName } from "./🏷️names/🟦️.ts";

/** ⚙️ The options of one model file: what the analytic looks read, the authored `scales` to include (default every authored scale), the pose or the clip, and whether the document is resolved. */
export type ModelFileOptions = SharedModelOptions & { readonly scales?: readonly number[]; readonly pose?: Slug; readonly clip?: Slug; readonly resolved?: boolean };

/** ✂️ The sharp-edge angle of the feature edges: 30 degrees. */
export const EDGE_ANGLE_RAD = Math.PI / 6;

/** 🧾️ The `modelOfAsset` options of a single pose (static state, no clips) or a single clip (static state at its first instant, one animation). Throws {@link ModelRequestError} unless exactly one of `pose` and `clip` is named. */
export function singleOptions(asset: Asset, options: ModelFileOptions): AssetModelOptions {
  const { pose, clip, resolved: _resolved, ...rest } = options;
  if ((pose === undefined) === (clip === undefined)) throw new ModelRequestError("model-needs-one");
  if (pose !== undefined) {
    if (!asset.poses.some((p) => p.id === pose)) throw new ModelRequestError("model-unknown-pose", pose);
    return { ...rest, at: { kind: "pose", pose }, poses: [pose], clips: [] };
  }
  if (!asset.clips.some((c) => c.id === clip)) throw new ModelRequestError("model-unknown-clip", clip);
  return { ...rest, at: { kind: "clip", clip: clip!, timeS: 0 }, poses: [], clips: [clip!] };
}

/** 🔺️ The triangles of a resolved document: every triangle primitive of every mesh (line primitives are not counted). */
export const trianglesOf = (document: ModelDocument): number => document.meshes.reduce((sum, mesh) => sum + mesh.primitives.reduce((n, p) => n + (p.mode === "triangles" ? p.indices.length / 3 : 0), 0), 0);

/** 🏭️ The resolved multi-scene document of one pose or clip: every scene tessellated at the quality of its scale, then the feature edges (the weighted phases tessellate and edges of `job`, which starts them at `first`). */
export async function resolvedOf(asset: Asset, document: ModelDocument, job: Job, first: number, options: { readonly shareVertices?: boolean } = {}): Promise<ModelDocument> {
  const resolved = await resolveScenes(document, (scale) => qualityOf(asset, scale), job.operation(first));
  return featureEdges(resolved, EDGE_ANGLE_RAD, { ...options, material: edgeMaterialOf(document) }, job.operation(first + 1));
}

/** ⏳️ The model document of one pose or clip as JSON text (a trailing newline). Rejects with the signal's reason when cancelled. */
export async function writeModel(asset: Asset, options: ModelFileOptions, control: JobControl = {}): Promise<string> {
  const job = jobOf(control, [{ label: PHASES.model, weight: 6 }, ...(options.resolved === true ? [{ label: PHASES.tessellate, weight: 3 }, { label: PHASES.edges, weight: 2 }] : []), { label: PHASES.encode, weight: 1 }]);
  return cancellable(control, async () => {
    let document: ModelDocument = await modelOfAsset(asset, singleOptions(asset, options), job.operation(0));
    if (options.resolved === true) document = await resolvedOf(asset, document, job, 1);
    const text = `${JSON.stringify(document)}\n`;
    job.finish();
    return text;
  });
}

/** 🗂️ What a file is. */
export type FileKind = "glb" | "fbx" | "model" | "zip";

/** 🗺️ One file of the set of an asset: what it holds (`subject`, `item`), at which `scale` (`null` for the files that hold every authored scale) in which `view` (`"all"` for the files that hold all seven), its kind and name. */
export type PlannedFile = { readonly subject: "pose" | "clip" | "asset"; readonly item: Slug | "all"; readonly scale: number | null; readonly view: View | "all"; readonly kind: FileKind; readonly name: string };

/** 🗺️ The file set of an asset in bundle order: per pose a GLB then its JSON, per clip its JSON then an FBX per scale and view, last the ZIP. `scales` restricts the FBX files to those authored scales (default all; an unavailable scale is refused by name) and `views` the FBX views (default all seven); the GLB, JSON and ZIP files always hold every authored scale. */
export function plannedFiles(asset: Asset, options: { readonly scales?: readonly number[]; readonly views?: readonly View[] } = {}): readonly PlannedFile[] {
  const authored = authoredScales(asset.volumes);
  if (authored.length === 0) throw new ModelRequestError("model-no-authored-scale", asset.id);
  const scales = options.scales === undefined ? authored : options.scales.map((scale) => authoredAt(asset.volumes, scale, asset.id).denominator);
  const views = options.views ?? VIEWS;
  const out: PlannedFile[] = [];
  for (const pose of asset.poses.filter((p) => p.key !== true)) {
    out.push({ subject: "pose", item: pose.id, scale: null, view: "all", kind: "glb", name: glbName(asset.id, pose.id) });
    out.push({ subject: "pose", item: pose.id, scale: null, view: "all", kind: "model", name: modelName(asset.id, "pose", pose.id) });
  }
  for (const clip of asset.clips) {
    out.push({ subject: "clip", item: clip.id, scale: null, view: "all", kind: "model", name: modelName(asset.id, "clip", clip.id) });
    for (const scale of scales) for (const view of VIEWS.filter((v) => views.includes(v))) out.push({ subject: "clip", item: clip.id, scale, view, kind: "fbx", name: fbxName(asset.id, clip.id, scale, view) });
  }
  out.push({ subject: "asset", item: "all", scale: null, view: "all", kind: "zip", name: zipName(asset.id) });
  return out;
}

/** 📁️ The bundle folder of a file kind: `glb/`, `fbx/` and `model/`. */
export const MEMBER_FOLDER: Readonly<Record<Exclude<FileKind, "zip">, string>> = { glb: "glb", fbx: "fbx", model: "model" };
