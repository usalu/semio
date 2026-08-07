// #region 🧲️Header
// 💻️ framework/ui/elements/🎚️Toggle/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import * as TogglePrimitive from "@radix-ui/react-toggle";
import { cva } from "class-variance-authority";
// 🧱️core: cn imported directly from 🫀️core/ClassNames, NOT via the barrel — this component calls
// cn(...) at module top level (inside a top-level cva(cn(...)) call), which requires a non-circular
// import (see 🧱️elements/🏷️ClassNames/🟦️component.tsx's header comment for why the barrel import
// caused a real bug).
import { cn } from "../🏷️ClassNames/🟦️component.tsx";
import { type UiLabel } from "../🏷️UiLabel/🟦️component.tsx";
import { type ElementProps } from "../🐹️ElementProps/🟦️component.tsx";
import { chromeControlItemClass, chromeControlItemOnClass } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
import { type ControlIcon } from "../🔣Icons/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🗡️Toggle
// Toggle button with pressed/unpressed states.
// Consumers MUST handle onPressedChange events.

/**
 * toggleVariants holds the data fields for a toggleVariants record.
 **/
const toggleVariants = cva(cn(chromeControlItemClass, chromeControlItemOnClass, "aspect-square"));

/**
 * Configuration interface for a single toggle option with value and label.
 **/
export interface ToggleItem<T extends string> {
  value: T;
  icon: ControlIcon;
  text?: string;
  dropdownText?: string;
  id?: string;
}

/**
 * ToggleStandardProps holds the data fields for a ToggleStandardProps record.
 **/
interface ToggleStandardProps extends Omit<React.ComponentProps<typeof TogglePrimitive.Root>, "type" | "id">, ElementProps {
  kind?: "default" | "icon" | "single";
  i18nPressed?: string;
  showLabel?: boolean;
  icon: ControlIcon;
  text?: string;
}

/**
 * ToggleWithActionProps holds the data fields for a ToggleWithActionProps record.
 **/
interface ToggleWithActionProps extends Omit<React.ComponentProps<typeof TogglePrimitive.Root>, "type" | "id">, ElementProps {
  kind: "withAction";
  actionIcon: ControlIcon;
  onActionClick: () => void;
  showLabel?: boolean;
  actionId?: string;
  icon: ControlIcon;
  text?: string;
}

/**
 * ToggleDropdownProps holds the data fields for a ToggleDropdownProps record.
 **/
interface ToggleDropdownProps<T extends string> extends Omit<React.ComponentProps<typeof TogglePrimitive.Root>, "type" | "id">, ElementProps {
  kind: "dropdown";
  value?: T;
  defaultValue?: T;
  onValueChange?: (value: T) => void;
  items: ToggleItem<T>[];
  showLabel?: boolean;
  placeholder?: UiLabel;
  dropdownId?: string;
  dropdownSide?: "top" | "right" | "bottom" | "left";
  dropdownAlign?: "start" | "center" | "end";
  dropdownSideOffset?: number;
  dropdownAvoidCollisions?: boolean;
  dropdownInstant?: boolean;
  dropdownContentClassName?: string;
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
}
type ToggleProps<T extends string = string> = ToggleStandardProps | ToggleWithActionProps | ToggleDropdownProps<T>;

export type { ToggleProps, ToggleStandardProps, ToggleWithActionProps, ToggleDropdownProps };
export { toggleVariants };

// #endregion 🗡️Toggle
