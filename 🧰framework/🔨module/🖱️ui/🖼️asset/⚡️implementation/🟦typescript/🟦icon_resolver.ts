// #region 🧲Header
/** @emoji 🖼 Theme-aware catalog icon SVG resolution. */
// #endregion 🧲Header

import { ICONS, isIconName, type IconName } from "../../../../../../🧰framework/🔨module/🖱️ui/🖼️asset/⚡️implementation/🟦typescript/🔣icon/🤖generated/🟦icons.ts";
import type { UiThemeIcons } from "../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🟦typescript/🕸️graph/🗣️dsl/🫀core/🟦typescript/📦index.ts";

/** @emoji 🖼 Resolves catalog icon SVG markup with optional theme aliases and variants. */
export function resolveCatalogIconSvgFromTheme(name: IconName, icons: UiThemeIcons | undefined): string {
  const variant = icons?.variants?.[name];
  if (variant) {
    return variant;
  }
  const alias = icons?.aliases?.[name];
  const resolved = alias && isIconName(alias) ? alias : name;
  return ICONS[resolved];
}

/** @emoji 🔍 Resolves a catalog icon id through theme aliases to a compile-time {@link IconName}. */
export function resolveCatalogIconNameFromTheme(name: IconName, icons: UiThemeIcons | undefined): IconName {
  const alias = icons?.aliases?.[name];
  return alias && isIconName(alias) ? alias : name;
}
