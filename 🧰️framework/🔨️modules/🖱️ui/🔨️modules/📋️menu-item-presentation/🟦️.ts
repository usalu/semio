// #region 🧲️Header
// 💻️ framework/ui/modules/📋️menu-item-presentation/component.ts
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { cn } from "../🏷️class-name-composition/🟦️.ts";
import { interactiveHoverClass } from "../🖱️interaction-presentation/🟦️.ts";
// #endregion 🔌️Adapters

// #region 📋️MenuItemPresentation
/** @emoji 📋️ Shared hover, focus, and selection presentation for menu rows. */
export const menuListItemClassName = cn(
  "text-element",
  interactiveHoverClass,
  "focus:bg-hover-interactive-fill focus:text-emphasized",
  "data-[active=true]:bg-hover-interactive-fill data-[active=true]:text-emphasized",
  "data-[selected=true]:bg-active-base data-[selected=true]:border-active-base data-[selected=true]:text-emphasized",
);
// #endregion 📋️MenuItemPresentation
