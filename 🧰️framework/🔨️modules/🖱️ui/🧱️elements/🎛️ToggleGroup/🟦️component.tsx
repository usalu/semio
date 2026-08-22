// #region 🧲️Header
// 💻️ framework/ui/elements/🎛️ToggleGroup/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import * as ToggleGroupPrimitive from "@radix-ui/react-toggle-group";
// 🧱️core: reactHostPort imported directly from 🫀️core/Ports, NOT via the barrel — this component calls
// reactHostPort.createContext at module top level, which requires a non-circular import (see
// 🧱️elements/🔌️Ports/🟦️component.tsx's header comment for why the barrel import caused a real bug).
import { reactHostPort } from "../🔌️Ports/🟦️component.tsx";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️component.ts";
import { styleVariants } from "../../🔨️modules/🏷️style-variants/🟦️component.ts";
import { surfaceClass } from "../../🔨️modules/🌈️surface-presentation/🟦️component.ts";
import { ControlHotkeyBadge } from "../../🔨️modules/⌨️control-hotkey-presentation/🟦️component.tsx";
import { chromeControlGroupClass, chromeControlItemClass, chromeControlItemOnClass } from "../../🔨️modules/🎛️chrome-control-presentation/🟦️component.ts";
import { type Level, useLevel } from "../🌈️Surface/🟦️component.tsx";
import { Label, useControlInlineText, useControlAccessibleLabel, useControlTooltipText } from "../🏷️Label/🟦️component.tsx";
import { renderControlIcon, type ControlIcon } from "../🔣️Icons/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🧩️ToggleGroup
// Group of mutually exclusive or multi-select toggles.
// Consumers MUST provide items with distinct values.

const toggleVariants = styleVariants(cn(chromeControlItemClass, chromeControlItemOnClass, "aspect-square"));

/**
 * ToggleGroupContext holds the data fields for a ToggleGroupContext record.
 **/
const ToggleGroupContext = reactHostPort.createContext<{ level: Level }>({
  level: "base",
});

/**
 * ToggleGroupItemProps holds the data fields for a ToggleGroupItemProps record.
 **/
type ToggleGroupItemProps = Omit<React.ComponentProps<typeof ToggleGroupPrimitive.Item>, "children"> & {
  id?: string;
  icon: ControlIcon;
  text?: string;
  action?: React.ReactNode;
  value: string;
};

/**
 * ToggleGroupProps holds the data fields for a ToggleGroupProps record.
 **/
interface ToggleGroupProps extends Omit<React.ComponentProps<typeof ToggleGroupPrimitive.Root>, "children" | "type" | "id"> {
  id?: string;
  showLabel?: boolean;
  kind?: "single" | "multiple";
  items: ToggleGroupItemProps[];
}

/**
 * ToggleGroup holds the data fields for a ToggleGroup record.
 **/
function ToggleGroup({ className, id, showLabel, items, kind = "single", ...restProps }: ToggleGroupProps) {
  const level = useLevel();

  const controlledValue = (restProps as any).value;
  const rootDataState = kind === "single" && controlledValue !== undefined ? (controlledValue ? "on" : "off") : undefined;

  const toggleGroupElement = (
    <ToggleGroupPrimitive.Root
      data-slot="toggle-group"
      data-detail-panel-control="fit"
      data-state={rootDataState}
      id={id}
      type={kind}
      className={cn(chromeControlGroupClass, "group/toggle-group has-[_[data-slot=inline-label]]:overflow-visible", className)}
      {...(restProps as any)}
    >
      <ToggleGroupContext.Provider value={{ level }}>
        {items.map((item) => (
          <ToggleGroupItem key={item.value} {...item} id={item.id ?? id} />
        ))}
      </ToggleGroupContext.Provider>
    </ToggleGroupPrimitive.Root>
  );

  if (showLabel && id) {
    return (
      <Label id={id} labelElementId={`${id}-label`}>
        {toggleGroupElement}
      </Label>
    );
  }

  return toggleGroupElement;
}

/**
 * ToggleGroupItem holds the data fields for a ToggleGroupItem record.
 **/
function ToggleGroupItem({ className, id, icon, text, action, ...props }: ToggleGroupItemProps) {
  const context = reactHostPort.useContext(ToggleGroupContext);
  const level = context.level ?? "base";
  const inlineText = useControlInlineText(id, text);
  const accessibleLabel = useControlAccessibleLabel(id, text);
  const tooltipText = useControlTooltipText(id, text);
  const ariaLabel = inlineText ? undefined : accessibleLabel;

  const toggleGroupItemElement = (
    <ToggleGroupPrimitive.Item
      data-slot="toggle-group-item"
      id={id}
      aria-label={ariaLabel}
      title={tooltipText}
      data-level={level}
      className={cn(
        toggleVariants(),
        inlineText ? "w-auto shrink-0 focus:z-panel focus-visible:z-panel" : "min-w-0 flex-1 shrink-0 focus:z-panel focus-visible:z-panel",
        (inlineText || action) && "flex items-center gap-single py-single px-double aspect-auto",
        inlineText && "w-auto",
        className,
      )}
      {...props}
    >
      {inlineText ? (
        <span data-slot="inline-label" className="text-xs whitespace-nowrap">
          {inlineText}
        </span>
      ) : null}
      <ControlHotkeyBadge id={id} allowInline={Boolean(inlineText)} />
      <span className={action ? "flex-1 flex items-center justify-center" : undefined}>{renderControlIcon(icon)}</span>
      {action && (
        <div
          className={cn("flex items-center justify-center aspect-square h-full flex-shrink-0", surfaceClass, text && "ms-single")}
          onClick={(e) => e.stopPropagation()}
          onPointerDown={(e) => e.stopPropagation()}
          onMouseDown={(e) => e.stopPropagation()}
          onMouseUp={(e) => e.stopPropagation()}
          onDoubleClick={(e) => e.stopPropagation()}
        >
          {action}
        </div>
      )}
    </ToggleGroupPrimitive.Item>
  );

  return toggleGroupItemElement;
}

export { ToggleGroup, ToggleGroupItem };

// #endregion 🧩️ToggleGroup
