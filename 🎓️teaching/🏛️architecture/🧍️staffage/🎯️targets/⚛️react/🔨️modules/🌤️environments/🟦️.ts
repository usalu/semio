/** 🌤️ The named environment presets of the catalog and the reference case of an asset, the same cases the cast manifest and the export command name.
 *
 * The thermal view of an asset solves under, in this order: the preset the reader chose, the asset's `thermal.referenceEnvironment` (a preset id or an inline environment), else the asset's own `thermal.environment`. The reader's slider changes sit on top of that.
 *
 * @see ../../../../🗂️catalog/🌤️environments/🔣️.json — the presets
 * @see ../../../../🔨️modules/🗂️catalog/🟦️.ts — `resolveThermalEnvironment`, the precedence the exporters use
 */
import { resolveThermalEnvironment } from "@teaching/staffage";
import type { Asset, Environment, EnvironmentDocument, EnvironmentPreset, Slug } from "@teaching/staffage";
import presetDocument from "../../../../🗂️catalog/🌤️environments/🔣️.json";

/** 🏞️ The presets in catalog order. */
export const ENVIRONMENT_PRESETS: readonly EnvironmentPreset[] = (presetDocument as unknown as EnvironmentDocument).presets;

/** 🗺️ The presets by id. */
export const PRESETS_BY_ID: ReadonlyMap<Slug, EnvironmentPreset> = new Map(ENVIRONMENT_PRESETS.map((preset) => [preset.id, preset]));

/** 🪪️ Whether `value` names a preset. */
export const isPreset = (value: string | null | undefined): value is Slug => PRESETS_BY_ID.has(value ?? "");

/** 🎯️ The case an asset's thermal view starts in: its preset id when it names one, and the environment itself. */
export type ReferenceCase = { readonly id: Slug | null; readonly environment: Environment };

/** 🎯️ The reference case of an asset. */
export function referenceCase(asset: Asset): ReferenceCase {
  const choice = asset.thermal.referenceEnvironment;
  return { id: typeof choice === "string" ? choice : null, environment: resolveThermalEnvironment(asset, PRESETS_BY_ID) ?? asset.thermal.environment };
}

/** 🌡️ The environment the viewer solves under: the chosen preset or the reference case, with the reader's changes on top. */
export function environmentOf(asset: Asset, preset: Slug | null, changes: Partial<Environment>): { readonly environment: Environment; readonly presetId: Slug | null; readonly changed: boolean } {
  const base = preset === null ? referenceCase(asset) : { id: preset, environment: PRESETS_BY_ID.get(preset)!.environment };
  const changed = Object.keys(changes).length > 0;
  return { environment: changed ? { ...base.environment, ...changes } : base.environment, presetId: base.id, changed };
}
