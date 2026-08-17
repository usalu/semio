// #region 🧲️Header
// 💻️ framework/ui/modules/📝️form-control-presentation/component.ts
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { cn } from "../🏷️class-name-composition/🟦️component.ts";
import { interactiveControlTransitionClass } from "../🖱️interaction-presentation/🟦️component.ts";
// #endregion 🔌️Adapters

// #region 🧾️FormControlPresentation
/** @emoji 🎯️ Focus and open presentation for form controls. */
export const formControlFocusBorderClass = cn("outline-none", interactiveControlTransitionClass, "focus-visible:border-accent data-[state=open]:border-accent aria-invalid:border-destructive focus-visible:ring-0 shadow-none");

/** @emoji 🚫️ Native browser affordances disabled on editable UI controls. */
export const uiFormControlBrowserDefaultProps = {
  autoComplete: "off",
  autoCorrect: "off",
  autoCapitalize: "off",
  spellCheck: false,
  "data-1p-ignore": true,
  "data-lpignore": "true",
} as const;
// #endregion 🧾️FormControlPresentation
