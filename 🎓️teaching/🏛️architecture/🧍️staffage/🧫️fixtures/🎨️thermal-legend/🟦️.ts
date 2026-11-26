/** 🎨️ Test-support selector of the thermal-legend suite: finds, by property and not by id, a catalog figure whose authored 1:500 elevation draws a merged form (one shape listing several thermal regions that all carry area).
 *
 * @see ../../🧪️tests/🎨️thermal-legend/🥒️.feature — the scenario "Merged regions at 1:500"
 */
import type { Asset, Slug } from "../../🧬️schema/🟦️.ts";
import { catalogAsset, catalogIds } from "../🌡️thermal/🟦️.ts";

/** 🧾️ The subject of the merged-region case: the figure, the id of its first pose, the id of the merged shape and the regions it lists. */
export type MergedSilhouette = { readonly asset: Asset; readonly pose: Slug; readonly shapeId: Slug; readonly regionIds: readonly Slug[] };

/** 🔎️ The first figure (catalog order, ids sorted) with a merged form in its authored 1:500 elevation; throws when the catalog has none, so the case can never pass vacuously. */
export function mergedSilhouette(): MergedSilhouette {
  for (const id of catalogIds()) {
    const asset = catalogAsset(id);
    const representation = asset.representations?.["500"];
    if (asset.poses.length === 0 || asset.category !== "humans" || representation?.status !== "authored") continue;
    const carriers = new Set(asset.thermal.regions.filter((r) => r.areaM2 > 0).map((r) => r.id));
    const shape = representation.geometry.elevation.find((s) => (s.regions?.length ?? 0) > 1 && s.regions!.every((r) => carriers.has(r)));
    if (shape) return { asset, pose: asset.poses[0]!.id, shapeId: shape.id, regionIds: shape.regions! };
  }
  throw new Error("no catalog figure draws a merged form at 1:500");
}
