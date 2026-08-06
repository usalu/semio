// #region 🧲️Header
/** @emoji 🖼️ Theme-aware catalog and metabolism icon SVG resolution. */
// #endregion 🧲️Header

//#region 🔖️CatalogIcons
import { ICONS, isIconName, type IconName } from "../🔣️icons/🤖️generated/🟦️icons.ts";
import type { UiThemeIcons } from "../../🖱️ui/🎨️styling/📦️packages/🟦️typescript/📦️index.ts";

/** @emoji 🖼️ Resolves catalog icon SVG markup with optional theme aliases and variants. */
export function resolveCatalogIconSvgFromTheme(name: IconName, icons: UiThemeIcons | undefined): string {
  const variant = icons?.variants?.[name];
  if (variant) {
    return variant;
  }
  const alias = icons?.aliases?.[name];
  const resolved = alias && isIconName(alias) ? alias : name;
  return ICONS[resolved];
}

/** @emoji 🔍️ Resolves a catalog icon id through theme aliases to a compile-time {@link IconName}. */
export function resolveCatalogIconNameFromTheme(name: IconName, icons: UiThemeIcons | undefined): IconName {
  const alias = icons?.aliases?.[name];
  return alias && isIconName(alias) ? alias : name;
}
//#endregion 🔖️CatalogIcons

//#region 🔖️MetabolismIcons
import { METABOLISM_ICONS, isMetabolismIconName, type MetabolismIconName } from "../🌱️metabolism/🔣️icons/🤖️generated/🟦️metabolism_icons.ts";

/** @emoji 🖼️ Resolves metabolism icon SVG markup with optional theme aliases and variants. */
export function resolveMetabolismIconSvgFromTheme(name: MetabolismIconName, icons: UiThemeIcons | undefined): string {
  const variant = icons?.themedVariants?.[name];
  if (variant) {
    return variant;
  }
  const alias = icons?.themedAliases?.[name];
  const resolved = alias && isMetabolismIconName(alias) ? alias : name;
  return METABOLISM_ICONS[resolved];
}

/** @emoji 🔍️ Resolves a metabolism icon id through theme aliases to a compile-time {@link MetabolismIconName}. */
export function resolveMetabolismIconNameFromTheme(name: MetabolismIconName, icons: UiThemeIcons | undefined): MetabolismIconName {
  const alias = icons?.themedAliases?.[name];
  return alias && isMetabolismIconName(alias) ? alias : name;
}
//#endregion 🔖️MetabolismIcons
