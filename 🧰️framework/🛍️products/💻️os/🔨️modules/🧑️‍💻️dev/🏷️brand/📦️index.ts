// #region 🧲️Header
/** @emoji 🏷️ Shell brand catalog — every brand this shell host can ship as, selected via `SEMIO_BRAND` / a playground registry row's `brand` column. */
// #endregion 🧲️Header

import type { ShellBrand } from "@semio-tech/framework";
import {
  ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND,
  ENTWERFEN_MIT_BESTAND_AUSSUCHEN_BRAND,
  ENTWERFEN_MIT_BESTAND_BEARBEITEN_BRAND,
  ENTWERFEN_MIT_BESTAND_GENERATOR_BRAND,
  ENTWERFEN_MIT_BESTAND_KOORDINATOR_BRAND,
  ENTWERFEN_MIT_BESTAND_VERFOLGEN_BRAND,
} from "../../../../../../♻️mit-bestand/🧺️demonstrator/🟦️brand.ts";

//#region 🏷️ShellBrandCatalog
/** 🏷️ Brand ids referenced by `brand = "…"` playground registry rows and the `SEMIO_BRAND` env var. */
export const SHELL_BRANDS: readonly ShellBrand[] = [
  ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND,
  ENTWERFEN_MIT_BESTAND_AUSSUCHEN_BRAND,
  ENTWERFEN_MIT_BESTAND_BEARBEITEN_BRAND,
  ENTWERFEN_MIT_BESTAND_GENERATOR_BRAND,
  ENTWERFEN_MIT_BESTAND_KOORDINATOR_BRAND,
  ENTWERFEN_MIT_BESTAND_VERFOLGEN_BRAND,
];

/** 🎯️ Resolves a brand id against {@link SHELL_BRANDS}; warns on unknown ids so a typo'd kiosk build degrades visibly instead of silently unbranding. */
export function resolveShellBrandById(id: string | undefined): ShellBrand | undefined {
  if (!id) return undefined;
  const brand = SHELL_BRANDS.find((entry) => entry.id === id);
  if (!brand) console.warn(`[shell-brand] unknown brand id ${JSON.stringify(id)} — known: ${SHELL_BRANDS.map((entry) => entry.id).join(", ")}`);
  return brand;
}
//#endregion 🏷️ShellBrandCatalog

export {
  ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND,
  ENTWERFEN_MIT_BESTAND_AUSSUCHEN_BRAND,
  ENTWERFEN_MIT_BESTAND_BEARBEITEN_BRAND,
  ENTWERFEN_MIT_BESTAND_GENERATOR_BRAND,
  ENTWERFEN_MIT_BESTAND_KOORDINATOR_BRAND,
  ENTWERFEN_MIT_BESTAND_VERFOLGEN_BRAND,
};
export type { ShellBrand };
