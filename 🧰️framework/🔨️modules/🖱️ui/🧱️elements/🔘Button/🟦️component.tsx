// #region 🧲️Header
// 💻️ framework/ui/elements/🔘Button/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { type VariantProps } from "class-variance-authority";
// 🚧️W3-interim: remaining symbols still live in the ui-react barrel — clear before W6.
import { type ControlIcon, type ElementProps } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
import { ButtonGroup, ButtonGroupItem, buttonGroupItemVariants } from "../🎛️ButtonGroup/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🌩️Button
// Single-item Button and cycling Button built on ButtonGroup.
// Consumers MUST provide an icon for each Button.

/**
 * ButtonProps holds the data fields for a ButtonProps record.
 **/
type ButtonProps = React.ComponentProps<"button"> &
  VariantProps<typeof buttonGroupItemVariants> & {
    asChild?: boolean;
    id?: string;
    icon: ControlIcon;
    text?: string;
    children?: React.ReactNode;
  };

/**
 * ButtonCycleItem holds the data fields for a ButtonCycleItem record.
 **/
interface ButtonCycleItem<T extends string> {
  value: T;
  label: string;
  icon: ControlIcon;
  text?: string;
  id?: string;
}

/**
 * ButtonCycleProps holds the data fields for a ButtonCycleProps record.
 **/
interface ButtonCycleProps<T extends string> extends Omit<React.ComponentProps<"button">, "children" | "id">, ElementProps {
  value?: T;
  onValueChange?: (value: T) => void;
  items: ButtonCycleItem<T>[];
  showLabel?: boolean;
}

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

/**
 * ButtonCycle holds the data fields for a ButtonCycle record.
 **/
function ButtonCycle<T extends string = string>({ className, id, showLabel, value, onValueChange, items, ...props }: ButtonCycleProps<T>) {
  const currentIndex = items.findIndex((item) => item.value === value);
  const currentItem = currentIndex >= 0 ? items[currentIndex] : items[0];
  const cycleText = typeof currentItem?.text === "string" ? currentItem.text : typeof currentItem?.label === "string" ? currentItem.label : undefined;

  const handleCycle = () => {
    const nextIndex = (currentIndex + 1) % items.length;
    if (onValueChange) onValueChange(items[nextIndex].value);
  };

  return (
    <ButtonGroup id={id} showLabel={showLabel} className={className}>
      <ButtonGroupItem id={id} onClick={handleCycle} icon={currentItem.icon} text={cycleText} {...props} />
    </ButtonGroup>
  );
}

export { Button, ButtonCycle };
export type { ButtonCycleProps, ButtonProps };
// #endregion 🌩️Button
