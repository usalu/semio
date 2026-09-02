// #region 🧲️Header
// 💻️ framework/ui/elements/🧾️Form/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
// #endregion 🔌️Adapters

// #region 🧾️Form
/** 🧾️ Native form boundary that preserves browser submission, validation, and Enter-key behavior. */
export type FormProps = React.ComponentPropsWithoutRef<"form">;

/** 📨️ Renders an owned semantic form without intercepting the browser submission lifecycle. */
export const Form = React.forwardRef<HTMLFormElement, FormProps>(function Form({ children, ...props }, ref) {
  return (
    <form ref={ref} {...props} data-slot="form">
      {children}
    </form>
  );
});
// #endregion 🧾️Form
