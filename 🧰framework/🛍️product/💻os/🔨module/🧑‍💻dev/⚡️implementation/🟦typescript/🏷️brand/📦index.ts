// #region 🧲Header
/** @emoji 🏷️ Shell brand catalog — every brand this shell host can ship as, selected via `SEMIO_BRAND` / a playground registry row's `brand` column. */
// #endregion 🧲Header

import type { ShellBrand } from "../../../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🟦typescript/🕸️graph/🗣️dsl/🫀core/🟦typescript/📦index.ts";
import { ENTWERFEN_MIT_BESTAND_BRAND } from "../../../../../../../../♻️mit-bestand/🧺aggregator/🟦🟦brand.ts";

//#region 🏷️ShellBrandCatalog
/** 🏷️ Brand ids referenced by `brand = "…"` playground registry rows and the `SEMIO_BRAND` env var. */
export const SHELL_BRANDS: readonly ShellBrand[] = [ENTWERFEN_MIT_BESTAND_BRAND];

/** 🎯 Resolves a brand id against {@link SHELL_BRANDS}; warns on unknown ids so a typo'd kiosk build degrades visibly instead of silently unbranding. */
export function resolveShellBrandById(id: string | undefined): ShellBrand | undefined {
  if (!id) return undefined;
  const brand = SHELL_BRANDS.find((entry) => entry.id === id);
  if (!brand) console.warn(`[shell-brand] unknown brand id ${JSON.stringify(id)} — known: ${SHELL_BRANDS.map((entry) => entry.id).join(", ")}`);
  return brand;
}
//#endregion 🏷️ShellBrandCatalog

export { ENTWERFEN_MIT_BESTAND_BRAND };
export type { ShellBrand };
