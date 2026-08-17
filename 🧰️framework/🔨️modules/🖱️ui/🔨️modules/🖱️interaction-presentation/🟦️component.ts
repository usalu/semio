// #region 🧲️Header
// 💻️ framework/ui/modules/🖱️interaction-presentation/component.ts
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { cn } from "../🏷️class-name-composition/🟦️component.ts";
// #endregion 🔌️Adapters

// #region 🫳️InteractionPresentation
/** @emoji 🎨️ Shared transition for interactive chrome. */
export const interactiveControlTransitionClass = "transition-[color,border-color,background-color]";

const hoverExcludingHandleBgFillClass = "hover:not-data-[handle-hovered=true]:bg-hover-interactive-fill";
const hoverExcludingHandleActiveBgClass = "hover:not-data-[handle-hovered=true]:bg-active-base/90";
const hoverExcludingHandleActiveBorderClass = "hover:not-data-[handle-hovered=true]:border-active-base";

/** @emoji 🫳️ Tree-row hover fill excluding the nested drag handle. */
export const groupHoverExcludingHandleBgFillClass = "group-hover/tree-row:not-group-data-[handle-hovered=true]/tree-row:bg-hover-interactive-fill";

/** @emoji 🫳️ Hover emphasis excluding the nested drag handle. */
export const hoverExcludingHandleTextEmphasizedClass = "hover:not-data-[handle-hovered=true]:text-emphasized";

/** @emoji 🫳️ Hover fill excluding the nested drag handle. */
export { hoverExcludingHandleBgFillClass };

/** @emoji 🎨️ Normal-border interactive hover fill. */
export const interactiveHoverFillClass = "hover:bg-hover-interactive-fill";

/** @emoji 🎨️ Interactive hover fill with emphasized content. */
export const interactiveHoverClass = cn(interactiveHoverFillClass, "hover:text-emphasized");

/** @emoji 📏️ Active stroke paired with interactive active fill. */
export const interactiveActiveBorderClass = "border-active-base";

/** @emoji 🎨️ Pressed and selected active presentation. */
export const interactiveActiveFillClass = cn("bg-active-base", interactiveActiveBorderClass, "text-emphasized", hoverExcludingHandleActiveBgClass, hoverExcludingHandleActiveBorderClass, hoverExcludingHandleTextEmphasizedClass);

/** @emoji 🎨️ Data-state on presentation. */
export const interactiveOnClass = cn(
  "data-[state=on]:bg-active-base",
  "data-[state=on]:border-active-base",
  "data-[state=on]:text-emphasized",
  "data-[state=on]:hover:bg-active-base/90",
  "data-[state=on]:hover:border-active-base",
  "data-[state=on]:hover:text-emphasized",
);

/** @emoji 🎨️ Active tab presentation. */
export const interactiveTabActiveClass = cn(
  "data-[state=active]:bg-active-base",
  "data-[state=active]:border-active-base",
  "data-[state=active]:text-emphasized",
  "data-[state=active]:hover:bg-active-base/90",
  "data-[state=active]:hover:border-active-base",
  "data-[state=active]:hover:text-emphasized",
);
// #endregion 🫳️InteractionPresentation
