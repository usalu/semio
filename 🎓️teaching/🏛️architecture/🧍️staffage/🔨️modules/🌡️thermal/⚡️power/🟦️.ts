/** ⚡️ Total power of an asset: the sum over all its elements of their nameplate power plus the metabolic heat of its living bodies.
 *
 * - Every element with a rating states it once, on one of its regions: `ratedPowerW` with its `ratedPowerBasis` (electrical input, fuel input, rated output of an engine or motor, or 0 W passive). A heat pump counts the
 *   electrical input of its compressor and fan, not its heating capacity; a radiator, coil or collector fed from elsewhere is passive. Ratings are never derived from `operating.heatW`, which is released heat.
 * - Living bodies (the primary body and every passenger) contribute their metabolic heat in the default pose: `max(basal, met · metWM2 · skinAreaM2)`, with the activity the default pose binds
 *   (`activityMet`, else `thermal.baseMet`, else 1 met; passengers 1 met).
 * - Poses do not matter: an engine that is off in the default pose keeps its rating; elements that never run together are summed, so the value is an installed power, not a simultaneous load.
 * - An asset without a living body or a rating has 0 W.
 *
 * @see ../../../🧬️schema/🟦️.ts — ThermalRegion.ratedPowerW, PowerBasis
 * @see ../../../README.md — Power
 */
import type { Asset, Text } from "../../../🧬️schema/🟦️.ts";

/** 🔌️ One element that contributes power: a living body or a rated region. */
export type PowerElement = { readonly id: string; readonly kind: "body" | "passenger" | "region"; readonly title: Text; readonly powerW: number };

/** 🧾️ The total power of an asset with its elements, largest first. */
export type AssetPower = { readonly totalW: number; readonly elements: readonly PowerElement[] };

/** 🏷️ The tolerance between the manifest and the asset in watts. */
export const POWER_TOLERANCE_W = 0.5;

const BODY_TITLE: Text = { en: "Living body", de: "Lebender Körper" };

/** ⚡️ The power of every element of an asset and their sum. */
export function powerOf(asset: Asset): AssetPower {
  const { thermal } = asset;
  const pose = asset.poses.find((candidate) => candidate.id === asset.presentation.defaultPose);
  const metabolic = (physiology: NonNullable<typeof thermal.body>, met: number | undefined, fallback: number): number =>
    Math.max(physiology.basalMetabolismW, (met !== undefined && met > 0 ? met : fallback) * physiology.metWM2 * physiology.skinAreaM2);
  const elements: PowerElement[] = [];
  if (thermal.body !== undefined) elements.push({ id: "body", kind: "body", title: BODY_TITLE, powerW: metabolic(thermal.body, pose?.thermal.activityMet, thermal.baseMet ?? 1) });
  for (const passenger of thermal.passengers) elements.push({ id: passenger.id, kind: "passenger", title: passenger.title, powerW: metabolic(passenger.physiology, pose?.thermal.passengers?.[passenger.id]?.activityMet, 1) });
  for (const region of thermal.regions) if (region.ratedPowerW !== undefined && region.ratedPowerW > 0) elements.push({ id: region.id, kind: "region", title: region.title, powerW: region.ratedPowerW });
  elements.sort((a, b) => b.powerW - a.powerW || a.id.localeCompare(b.id));
  return { totalW: elements.reduce((sum, element) => sum + element.powerW, 0), elements };
}

/** 🔋️ The total power of an asset in watts. */
export const totalPowerW = (asset: Asset): number => powerOf(asset).totalW;
