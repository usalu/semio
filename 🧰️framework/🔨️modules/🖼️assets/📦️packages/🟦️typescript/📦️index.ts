// #region 🧲️Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Barrel export for all asset modules including icons, fonts, representations and images.

// #endregion 🧲️Header

//#region 🗃️Exports
// Builtin UI icons only — the Metabolism kit fixture (and its derived MetabolismKit* exports) moved
// to `@semio-tech/compose-fixture` (REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT): it had zero runtime
// consumers outside `.storybook/stories/compose/**`, yet this barrel is imported by every document
// through `ui-react`, so the 7.3MB JSON was being parsed+flattened for nothing on every boot.

//#region 🔖️Icons
export type { IconName } from "../../🔣️icons/🤖️generated/🟦️icons.ts";
export { ICONS, ICON_NAMES, isIconName } from "../../🔣️icons/🤖️generated/🟦️icons.ts";
export { assertUniqueIconConceptAssignments, ICON_CONCEPT_ASSIGNMENTS, type IconConceptId } from "../../🎯️concepts/🟦️component.ts";
export { resolveCatalogIconNameFromTheme, resolveCatalogIconSvgFromTheme } from "../../🔍️resolver/🟦️component.ts";
export {
  SHORTCODE_CATALOG,
  SHORTCODE_EMOJI,
  shortcodeCatalogKey,
  shortcodeEmoji,
  type ShortcodeCatalogName,
  type ShortcodeEmojiName,
} from "../../🔣️icons/🤖️generated/🟦️shortcodes.ts";
export { isMetabolismIconName, METABOLISM_ICONS, METABOLISM_ICON_NAMES, type MetabolismIconName } from "../../🌱️metabolism/🔣️icons/🤖️generated/🟦️metabolism_icons.ts";
export { resolveMetabolismIconNameFromTheme, resolveMetabolismIconSvgFromTheme } from "../../🔍️resolver/🟦️component.ts";
//#endregion 🔖️Icons

//#endregion 🗃️Exports
