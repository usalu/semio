// #region 🧲️Header
// 💻️ framework/ui/elements/☑️Checkbox/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
// #endregion 🔌️Adapters

// #region ☑️Checkbox
/** 🎚️ Controlled native checkbox states, including the DOM-only mixed presentation. */
export type CheckboxState = boolean | "indeterminate";

/** 📨️ Native checkbox props with a tri-state controlled value. */
export type CheckboxProps = Omit<React.ComponentPropsWithoutRef<"input">, "aria-checked" | "checked" | "type"> & {
  readonly checked?: CheckboxState;
};

/** ☑️ Accessible native checkbox that synchronizes `checked`, `aria-checked`, and `indeterminate`. */
export const Checkbox = React.forwardRef<HTMLInputElement, CheckboxProps>(function Checkbox({ checked, ...props }, forwardedRef) {
  const inputRef = React.useRef<HTMLInputElement>(null);
  React.useImperativeHandle(forwardedRef, () => inputRef.current!, []);
  React.useLayoutEffect(() => {
    if (inputRef.current) inputRef.current.indeterminate = checked === "indeterminate";
  }, [checked]);

  return <input ref={inputRef} {...props} data-slot="checkbox" type="checkbox" checked={checked === "indeterminate" ? false : checked} aria-checked={checked === "indeterminate" ? "mixed" : checked} />;
});
// #endregion ☑️Checkbox
