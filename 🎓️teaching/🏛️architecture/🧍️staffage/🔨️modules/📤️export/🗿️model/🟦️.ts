/** 🗿️ The neutral model as a file: the `semio.model/v1` JSON document of one pose or one clip of an asset, procedural (declarative solids, the exact data the GLB and FBX are made from) or, on request, resolved (tessellated meshes with feature edges).
 *
 * A model file holds one pose (static, all seven views as variants) or one clip (animated); the name of every export carries asset, pose or clip, view, the word `model` where a drawing names its viewpoint, and the scale: `<asset>-<pose|clip>-<view|all>-model-1to<scale>`; the JSON of a pose ends `.pose.json`, that of a clip `.clip.json`, because a pose and a clip of one asset may share an id.
 *
 * @see ../../🧊️volume/🟦️.ts — `modelOf`
 */
import type { Asset, Slug } from "../../../🧬️schema/🟦️.ts";
import { featureEdges, tessellate, type ModelDocument } from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import type { JobControl, View } from "../../🔗️contracts/🟦️.ts";
import { type ModelOptions, ModelRequestError, edgeMaterialOf, modelOf } from "../../🧊️volume/🟦️.ts";
import { PHASES, cancellable, jobOf, qualityOf } from "../../🧊️volume/⏱️job/🟦️.ts";

/** ⚙️ The options of one model file: the {@link ModelOptions} that apply to a single pose or clip, the pose or the clip, and whether the document is resolved. */
export type ModelFileOptions = Omit<ModelOptions, "at" | "poses" | "clips"> & { readonly pose?: Slug; readonly clip?: Slug; readonly resolved?: boolean };

/** ✂️ The sharp-edge angle of the feature edges: 30 degrees. */
export const EDGE_ANGLE_RAD = Math.PI / 6;

/** 🧾️ The `modelOf` options of a single pose (static state, no clips) or a single clip (static state at its first instant, one animation). Throws {@link ModelRequestError} unless exactly one of `pose` and `clip` is named. */
export function singleOptions(asset: Asset, options: ModelFileOptions): ModelOptions {
  const { pose, clip, resolved: _resolved, ...rest } = options;
  if ((pose === undefined) === (clip === undefined)) throw new ModelRequestError("model-needs-one");
  if (pose !== undefined) {
    if (!asset.poses.some((p) => p.id === pose)) throw new ModelRequestError("model-unknown-pose", pose);
    return { ...rest, at: { kind: "pose", pose }, poses: [pose], clips: [] };
  }
  if (!asset.clips.some((c) => c.id === clip)) throw new ModelRequestError("model-unknown-clip", clip);
  return { ...rest, at: { kind: "clip", clip: clip!, timeS: 0 }, poses: [], clips: [clip!] };
}

/** 🏷️ The stem of a model file name: `<asset>-<pose|clip>-<view|all>-model-1to<scale>`. */
export const modelStem = (asset: Slug, what: Slug, view: View | "all", scale: number): string => `${asset}-${what}-${view}-model-1to${scale}`;

/** 🗂️ The extension of a neutral model file: a pose and a clip may share an id (`landing`), so the file says which it holds: `<stem>.pose.json` or `<stem>.clip.json`. */
export const modelExtension = (kind: "pose" | "clip"): string => `${kind}.json`;

/** 🔺️ The triangles of a resolved document: every triangle primitive of every mesh (line primitives are not counted). */
export const trianglesOf = (document: ModelDocument): number => document.meshes.reduce((sum, mesh) => sum + mesh.primitives.reduce((n, p) => n + (p.mode === "triangles" ? p.indices.length / 3 : 0), 0), 0);

/** ⏳️ The model document of one pose or clip as JSON text (a trailing newline). Rejects with the signal's reason when cancelled. */
export async function writeModel(asset: Asset, options: ModelFileOptions, control: JobControl = {}): Promise<string> {
  const job = jobOf(control, [{ label: PHASES.model, weight: 6 }, ...(options.resolved === true ? [{ label: PHASES.tessellate, weight: 3 }, { label: PHASES.edges, weight: 2 }] : []), { label: PHASES.encode, weight: 1 }]);
  return cancellable(control, async () => {
    let document: ModelDocument = await modelOf(asset, singleOptions(asset, options), job.operation(0));
    if (options.resolved === true) {
      const edgeMaterial = edgeMaterialOf(document);
      document = await tessellate(document, qualityOf(asset), job.operation(1));
      document = await featureEdges(document, EDGE_ANGLE_RAD, { material: edgeMaterial }, job.operation(2));
    }
    const text = `${JSON.stringify(document)}\n`;
    job.finish();
    return text;
  });
}
