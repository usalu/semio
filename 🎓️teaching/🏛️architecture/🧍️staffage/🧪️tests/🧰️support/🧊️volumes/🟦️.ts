/** 🧊️ Shared helpers of the per-scale 3D model tests: the sample figure with its handcrafted four-scale volumes, and every catalog asset with the scales it has authored so far.
 *
 * The catalog assets are re-authored by others and become valid gradually: {@link catalogVolumeStatus} reports per asset whether `volumes` is present and which scales are authored, so a guard can run over the authored ones, name the rest in its report and fail on any asset that claims an authored scale and breaks.
 *
 * Third-party libraries are test oracles only; the production code depends on none of them.
 */
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import type { Asset, ScaleVolume, VolumeSolid } from "../../../🧬️schema/🟦️.ts";
import { STAFFAGE_ROOT, fixture } from "../🧰️.ts";
import { sampleFigure } from "../🧊️.ts";

/** 🧍️ The sample figure with the pipeline fixture `volumes` of `🧫️fixtures/🧊️volume-scales` (body, thermal and structure at 1:50, 1:100, 1:200 and 1:500, whose ids and decisions the pipeline tests know), whatever volumes the sample figure fixture itself carries. */
export function scaledFigure(): Asset {
  const { volume: _volume, ...rest } = sampleFigure() as Asset & { volume?: unknown };
  return { ...rest, volumes: fixture<{ volumes: Asset["volumes"] }>("🧊️volume-scales").volumes } as Asset;
}

/** 🗂️ The state of one catalog asset: its parsed document and the authored scales of its `volumes` (empty while it is not re-authored). */
export type CatalogVolumeStatus = { readonly slug: string; readonly category: string; readonly asset: Asset; readonly authored: readonly number[]; readonly unavailable: readonly number[]; readonly reauthored: boolean };

/** 📋️ Every catalog asset with the scales it has authored, read straight from the files (no validation: that is the validator's work). */
export function catalogVolumeStatus(): CatalogVolumeStatus[] {
  const root = join(STAFFAGE_ROOT, "🗂️catalog");
  const out: CatalogVolumeStatus[] = [];
  for (const category of readdirSync(root, { withFileTypes: true })) {
    if (!category.isDirectory()) continue;
    for (const slug of readdirSync(join(root, category.name), { withFileTypes: true })) {
      if (!slug.isDirectory()) continue;
      let text: string;
      try {
        text = readFileSync(join(root, category.name, slug.name, "🔣️.json"), "utf8");
      } catch {
        continue;
      }
      const asset = (JSON.parse(text) as { asset: Asset }).asset;
      const volumes = (asset.volumes ?? {}) as Readonly<Record<string, ScaleVolume>>;
      const statusOf = (status: ScaleVolume["status"]): number[] => Object.values(volumes).filter((v) => v.status === status).map((v) => v.denominator).sort((a, b) => a - b);
      out.push({ slug: asset.id, category: category.name, asset, authored: statusOf("authored"), unavailable: statusOf("unavailable"), reauthored: asset.volumes !== undefined });
    }
  }
  return out.sort((a, b) => a.slug.localeCompare(b.slug));
}

/** 🧊️ The body solids of the authored volume of a scale; throws for a scale that is not authored. */
export function bodyOf(asset: Asset, scale: number): readonly VolumeSolid[] {
  const volume = asset.volumes[`${scale}` as "50"];
  if (volume?.status !== "authored") throw new Error(`${asset.id} has no authored volume at 1:${scale}`);
  return volume.body;
}
