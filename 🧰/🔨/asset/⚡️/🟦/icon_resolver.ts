// #region 🧲Header
/** @emoji 🖼 Theme-aware metabolism icon SVG resolution. */
// #endregion 🧲Header

import { METABOLISM_ICONS, isMetabolismIconName, type MetabolismIconName } from "./metabolism/icon/generated/metabolism_icons.ts";
import type { UiThemeIcons } from "../../../math/⚡️/🟦/graph/dsl/core/js/📦.ts";

/** @emoji 🖼 Resolves metabolism icon SVG markup with optional theme aliases and variants. */
export function resolveMetabolismIconSvgFromTheme(name: MetabolismIconName, icons: UiThemeIcons | undefined): string {
  const variant = icons?.themedVariants?.[name];
  if (variant) {
    return variant;
  }
  const alias = icons?.themedAliases?.[name];
  const resolved = alias && isMetabolismIconName(alias) ? alias : name;
  return METABOLISM_ICONS[resolved];
}

/** @emoji 🔍 Resolves a metabolism icon id through theme aliases to a compile-time {@link MetabolismIconName}. */
export function resolveMetabolismIconNameFromTheme(name: MetabolismIconName, icons: UiThemeIcons | undefined): MetabolismIconName {
  const alias = icons?.themedAliases?.[name];
  return alias && isMetabolismIconName(alias) ? alias : name;
}
