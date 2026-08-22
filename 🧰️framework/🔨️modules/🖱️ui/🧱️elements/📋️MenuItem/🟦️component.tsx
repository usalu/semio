// #region 🧲️Header
// 💻️ framework/ui/elements/📋️MenuItem/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { menuListItemClassName } from "../../🔨️modules/📋️menu-item-presentation/🟦️component.ts";
// #endregion 🔌️Adapters

// #region 📋️MenuItem
/** 🎨️ Shared layout and interaction presentation for button-like menu rows. */
export const menuItemClassName = `relative flex w-full cursor-default items-center gap-single rounded-sm px-single py-half text-start text-xs text-element outline-none select-none ${menuListItemClassName}`;

/** 📨️ Native button props accepted by the owned menu-row boundary. */
export type MenuItemProps = React.ComponentPropsWithoutRef<"button">;

/** 📋️ Focusable native button with menu-item semantics and no button-group layout wrapper. */
export const MenuItem = React.forwardRef<HTMLButtonElement, MenuItemProps>(function MenuItem({ className, disabled, type = "button", role = "menuitem", ...props }, ref) {
  return <button ref={ref} {...props} data-slot="menu-item" type={type} role={role} disabled={disabled} aria-disabled={disabled || undefined} className={className ? `${menuItemClassName} ${className}` : menuItemClassName} />;
});
// #endregion 📋️MenuItem
