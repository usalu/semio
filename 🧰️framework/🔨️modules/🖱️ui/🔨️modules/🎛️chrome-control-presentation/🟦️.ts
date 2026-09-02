// #region 🧲️Header
// 💻️ framework/ui/modules/🎛️chrome-control-presentation/component.ts
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { cn } from "../🏷️class-name-composition/🟦️.ts";
import { borderNormalClass } from "../📏️border-presentation/🟦️.ts";
import { formControlFocusBorderClass } from "../📝️form-control-presentation/🟦️.ts";
import { hoverExcludingHandleBgFillClass, hoverExcludingHandleTextEmphasizedClass, interactiveHoverClass, interactiveOnClass } from "../🖱️interaction-presentation/🟦️.ts";
import { glassClass } from "../🌈️surface-presentation/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🎛️ChromeControlPresentation
/** @emoji 🎛️ Shared transparent control-cell base. */
export const chromeControlItemBaseClass = cn(
  "text-element inline-flex items-center justify-center gap-single text-xs font-medium bg-transparent",
  "cursor-selectable disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed",
  "[&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-small [&_svg]:shrink-0",
  formControlFocusBorderClass,
  "whitespace-nowrap h-medium p-single overflow-hidden leading-none",
);

/** @emoji 🎛️ Interactive chrome control cell. */
export const chromeControlItemClass = cn(chromeControlItemBaseClass, interactiveHoverClass);

/** @emoji 🎛️ Drag-handle-aware chrome tab cell. */
export const chromeControlTabItemClass = cn(chromeControlItemBaseClass, hoverExcludingHandleBgFillClass, hoverExcludingHandleTextEmphasizedClass);

/** @emoji 📑️ Default mode-dock tab label. */
export const modeDockTabClassName = cn(chromeControlTabItemClass, "group max-w-[12rem] shrink-0 cursor-pointer items-center px-single select-none transition-colors");

/** @emoji 📑️ Pane chrome toggle presentation. */
export const windowPaneChromeToggleClass = cn(
  modeDockTabClassName,
  "relative z-30 box-border min-h-medium shrink-0 border-0 bg-transparent",
  "outline-none focus-visible:ring-1 focus-visible:ring-inset focus-visible:ring-active-base",
  "disabled:pointer-events-none disabled:opacity-50",
);

/** @emoji 🎛️ Shared outer chrome control shell. */
export const chromeControlGroupShellClass = cn("flex items-center border divide-x overflow-hidden w-fit shrink-0", borderNormalClass, "divide-normal", glassClass);

/** @emoji 🎛️ Standard chrome control group height. */
export const chromeControlGroupClass = cn(chromeControlGroupShellClass, "h-medium");

/** @emoji 🎛️ Data-state on presentation for chrome controls. */
export const chromeControlItemOnClass = interactiveOnClass;

/** @emoji 🎛️ Data-active presentation for chrome tabs. */
export const chromeControlTabActiveClass = cn(
  "data-[active=true]:bg-active-base",
  "data-[active=true]:border-active-base",
  "data-[active=true]:text-emphasized",
  "data-[active=true]:hover:bg-active-base/90",
  "data-[active=true]:hover:border-active-base",
  "data-[active=true]:hover:text-emphasized",
);
// #endregion 🎛️ChromeControlPresentation
