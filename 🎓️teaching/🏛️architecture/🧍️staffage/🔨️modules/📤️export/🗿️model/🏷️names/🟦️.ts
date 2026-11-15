/** 🏷️ The file names of the 3D model set of an asset, the one name rule of the library, the batch, the command line and the React downloads: `<asset>.<pose>.glb` (every authored scale and mode as glTF scenes), `<asset>.<pose>.pose.json` and `<asset>.<clip>.clip.json` (the multi-scene model of a pose and of a clip; a pose and a clip of one asset may share an id, so the file says which it holds), `<asset>.<clip>.s<scale>.<view>.fbx` (one clip at one scale in one view, one scene per file) and `<asset>.zip`.
 *
 * Kept free of every other import: a page that only names a download (the archive entry) must not load the model pipeline.
 *
 * @see ../🟦️.ts — `plannedFiles`, the whole file set
 */
import type { Slug } from "../../../../🧬️schema/🟦️.ts";
import type { View } from "../../../🔗️contracts/🟦️.ts";

/** 🏷️ The file name of the GLB of a pose: `<asset>.<pose>.glb`. */
export const glbName = (asset: Slug, pose: Slug): string => `${asset}.${pose}.glb`;

/** 🏷️ The file name of the model JSON of a pose (`<asset>.<pose>.pose.json`) or a clip (`<asset>.<clip>.clip.json`). */
export const modelName = (asset: Slug, kind: "pose" | "clip", id: Slug): string => `${asset}.${id}.${kind}.json`;

/** 🏷️ The file name of the FBX of a clip at a scale in a view: `<asset>.<clip>.s<scale>.<view>.fbx`. */
export const fbxName = (asset: Slug, clip: Slug, scale: number, view: View): string => `${asset}.${clip}.s${scale}.${view}.fbx`;

/** 🏷️ The file name of the ZIP of an asset: `<asset>.zip`. */
export const zipName = (asset: Slug): string => `${asset}.zip`;
