// #region 🧲️Header
/** 🖼️ Theme-aware catalog and metabolism icon SVG resolution. */
// #endregion 🧲️Header

//#region 🔖️CatalogIcons
import { ICONS, isIconName, type IconName } from "../🔣️icons/🤖️generated/🖼️icons/🟦️.ts";

/** 🎭️ Icon appearance overrides keyed by compile-time icon ids; the theme model of the styling layer satisfies this shape structurally. */
export interface CatalogThemeIconsV1 {
  readonly aliases?: Readonly<Partial<Record<string, string>>>;
  readonly variants?: Readonly<Partial<Record<string, string>>>;
  readonly themedAliases?: Readonly<Partial<Record<string, string>>>;
  readonly themedVariants?: Readonly<Partial<Record<string, string>>>;
}

/** 🖼️ Resolves catalog icon SVG markup with optional theme aliases and variants. */
export function resolveCatalogIconSvgFromTheme(name: IconName, icons: CatalogThemeIconsV1 | undefined): string {
  const variant = icons?.variants?.[name];
  if (variant) {
    return variant;
  }
  const alias = icons?.aliases?.[name];
  const resolved = alias && isIconName(alias) ? alias : name;
  return ICONS[resolved];
}

/** 🔍️ Resolves a catalog icon id through theme aliases to a compile-time {@link IconName}. */
export function resolveCatalogIconNameFromTheme(name: IconName, icons: CatalogThemeIconsV1 | undefined): IconName {
  const alias = icons?.aliases?.[name];
  return alias && isIconName(alias) ? alias : name;
}
//#endregion 🔖️CatalogIcons

//#region 🔖️MetabolismIcons
import { METABOLISM_ICONS, isMetabolismIconName, type MetabolismIconName } from "../🌱️metabolism/🔣️icons/🤖️generated/🖼️icons/🟦️.ts";

/** 🖼️ Resolves metabolism icon SVG markup with optional theme aliases and variants. */
export function resolveMetabolismIconSvgFromTheme(name: MetabolismIconName, icons: CatalogThemeIconsV1 | undefined): string {
  const variant = icons?.themedVariants?.[name];
  if (variant) {
    return variant;
  }
  const alias = icons?.themedAliases?.[name];
  const resolved = alias && isMetabolismIconName(alias) ? alias : name;
  return METABOLISM_ICONS[resolved];
}

/** 🔍️ Resolves a metabolism icon id through theme aliases to a compile-time {@link MetabolismIconName}. */
export function resolveMetabolismIconNameFromTheme(name: MetabolismIconName, icons: CatalogThemeIconsV1 | undefined): MetabolismIconName {
  const alias = icons?.themedAliases?.[name];
  return alias && isMetabolismIconName(alias) ? alias : name;
}
//#endregion 🔖️MetabolismIcons
