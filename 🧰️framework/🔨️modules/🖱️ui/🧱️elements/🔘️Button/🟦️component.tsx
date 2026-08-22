// #region 🧲️Header
// 💻️ framework/ui/elements/🔘️Button/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { type StyleVariantProps } from "../../🔨️modules/🏷️style-variants/🟦️component.ts";
import { type ControlIcon } from "../🔣️Icons/🟦️component.tsx";
import { ButtonGroup, ButtonGroupItem, buttonGroupItemVariants } from "../🎛️ButtonGroup/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🌩️Button
// Single-item Button built on ButtonGroup.
// Consumers MUST provide an icon for each Button.

/**
 * ButtonProps holds the data fields for a ButtonProps record.
 **/
type ButtonProps = React.ComponentProps<"button"> &
  StyleVariantProps<typeof buttonGroupItemVariants> & {
    asChild?: boolean;
    id?: string;
    icon: ControlIcon;
    text?: string;
    children?: React.ReactNode;
  };

/**
 **/
function Button({ className, asChild = false, id, icon, text, children, ...props }: ButtonProps) {
  return (
    <ButtonGroup className={className}>
      <ButtonGroupItem id={id} asChild={asChild} icon={icon} text={text} {...props}>
        {children}
      </ButtonGroupItem>
    </ButtonGroup>
  );
}

export { Button };
export type { ButtonProps };
// #endregion 🌩️Button
