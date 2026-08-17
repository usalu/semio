// #region 🧲️Header
// 💻️ framework/ui/elements/🎚️Toggle/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import * as TogglePrimitive from "@radix-ui/react-toggle";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️component.ts";
import { type UiLabel } from "../🏷️UiLabel/🟦️component.tsx";
import { type ElementProps } from "../../🔨️modules/🆔️element-identity/🟦️component.ts";
import { ToggleGroup } from "../🎛️ToggleGroup/🟦️component.tsx";
import { reactHostPort } from "../🔌️Ports/🟦️component.tsx";
import { Action } from "../⚡️ActionGroup/🟦️component.tsx";
import { Popover, PopoverTrigger, PopoverContent } from "../🗨️Popover/🟦️component.tsx";
import { ChromeControlHint } from "../💡️ChromeControlHint/🟦️component.tsx";
import { renderControlIcon, ChevronDownIcon, type ControlIcon } from "../🔣️Icons/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🗡️Toggle
// Toggle button with pressed/unpressed states.
// Consumers MUST handle onPressedChange events.

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

/**
 **/
const addIconSize = (element: ControlIcon): ControlIcon => {
  if (typeof element === "string") return element;
  if (typeof element === "object" && element !== null && !React.isValidElement(element)) return element;
  if (React.isValidElement(element)) {
    const existingClassName = (element.props as any).className || "";
    if (!existingClassName.includes("size-")) {
      return React.cloneElement(element, {
        className: cn(existingClassName, "size-small"),
      } as any);
    }
  }
  return element;
};

/**
 * Toggle holds the data fields for a Toggle record.
 **/
function Toggle<T extends string = string>(props: ToggleProps<T>) {
  if ("kind" in props && props.kind === "withAction") {
    const { actionIcon, onActionClick, icon, text, pressed, defaultPressed, onPressedChange, id, showLabel, className, actionId } = props as ToggleWithActionProps;
    const value = pressed !== undefined ? (pressed ? "on" : undefined) : undefined;
    return (
      <ToggleGroup
        showLabel={showLabel}
        kind="multiple"
        value={value ? [value] : []}
        defaultValue={pressed === undefined && defaultPressed ? ["on"] : []}
        onValueChange={(val: string[]) => onPressedChange?.(val.includes("on"))}
        className={className}
        items={[
          {
            value: "on",
            icon: addIconSize(icon),
            text: text,
            action: <Action as="div" id={actionId} icon={addIconSize(actionIcon)} onClick={onActionClick} />,
            id: id,
          },
        ]}
      />
    );
  }

  if ("kind" in props && props.kind === "dropdown" && "items" in props) {
    const dropdownProps = props as ToggleDropdownProps<T>;
    const {
      items,
      value: controlledValue,
      defaultValue,
      pressed,
      defaultPressed,
      onPressedChange,
      id,
      showLabel,
      className,
      dropdownId,
      dropdownSide = "bottom",
      dropdownAlign = "start",
      dropdownSideOffset = 4,
      dropdownAvoidCollisions = true,
      dropdownInstant = false,
      dropdownContentClassName,
      open: controlledOpen,
      onOpenChange,
      onValueChange,
    } = dropdownProps;
    const [internalValue, setInternalValue] = reactHostPort.useState<T | undefined>(defaultValue);
    const [internalOpen, setInternalOpen] = reactHostPort.useState(false);

    const isControlled = controlledValue !== undefined;
    const value = isControlled ? controlledValue : internalValue;
    const selectedItem = items.find((item) => item.value === value) || items[0];
    const isOpenControlled = controlledOpen !== undefined;
    const open = isOpenControlled ? controlledOpen : internalOpen;
    const setOpen = (nextOpen: boolean) => {
      if (!isOpenControlled) {
        setInternalOpen(nextOpen);
      }
      onOpenChange?.(nextOpen);
    };

    const handleSelect = (itemValue: string) => {
      if (!isControlled) {
        setInternalValue(itemValue as T);
      }
      if (onValueChange) onValueChange(itemValue as T);
      setOpen(false);
    };

    const handleToggleGroupValueChange = (toggleValue: string) => {
      const isPressed = toggleValue === selectedItem.value;
      if (onPressedChange) {
        onPressedChange(isPressed);
      }
    };

    const availableItems = items;

    const dropdownAction = (
      <Popover open={open} onOpenChange={setOpen}>
        <PopoverTrigger>
          <Action as="button" type="button" id={dropdownId} icon={<ChevronDownIcon className="size-small" />} />
        </PopoverTrigger>
        <PopoverContent
          side={dropdownSide}
          align={dropdownAlign}
          sideOffset={dropdownSideOffset}
          avoidCollisions={dropdownAvoidCollisions}
          className={cn(
            "w-auto p-single min-w-layout-popover",
            dropdownInstant ? "data-[state=open]:animate-none data-[state=closed]:animate-none data-[state=open]:fade-in-0 data-[state=closed]:fade-out-0 data-[state=open]:zoom-in-100 data-[state=closed]:zoom-out-100" : "",
            dropdownContentClassName,
          )}
        >
          <div className="flex flex-col">
            {availableItems.map((item) => {
              const dropdownText = item.dropdownText || item.text;
              const buttonElement = (
                <button
                  key={item.value}
                  onClick={() => handleSelect(item.value)}
                  className={cn("flex items-center p-single text-xs cursor-selectable transition-colors", "hover:bg-hover-interactive-fill outline-none focus-visible:bg-hover-interactive-fill")}
                >
                  <span className="flex flex-1 items-center gap-single text-start">
                    <span className="flex items-center">{renderControlIcon(addIconSize(item.icon))}</span>
                    {dropdownText ? <span className="text-xs">{dropdownText}</span> : null}
                  </span>
                </button>
              );

              if (item.id) {
                return (
                  <ChromeControlHint key={item.value} id={item.id}>
                    {buttonElement}
                  </ChromeControlHint>
                );
              }

              return buttonElement;
            })}
          </div>
        </PopoverContent>
      </Popover>
    );

    const isPressedControlled = pressed !== undefined;
    const toggleGroupProps: any = {
      id,
      showLabel,
      kind: "single" as const,
      onValueChange: handleToggleGroupValueChange,
      className,
      items: [
        {
          value: selectedItem.value,
          icon: addIconSize(selectedItem.icon),
          text: selectedItem.text,
          action: dropdownAction,

          id: selectedItem.id,
        },
      ],
    };

    if (isPressedControlled) {
      toggleGroupProps.value = pressed ? selectedItem.value : "";
    } else if (defaultPressed !== undefined) {
      toggleGroupProps.defaultValue = defaultPressed ? selectedItem.value : undefined;
    }

    return <ToggleGroup {...toggleGroupProps} />;
  }

  const { id, showLabel, className, icon, text, pressed, defaultPressed, onPressedChange } = props as ToggleStandardProps;
  const value = pressed !== undefined ? (pressed ? "on" : "") : undefined;
  return (
    <ToggleGroup
      id={id}
      showLabel={showLabel}
      className={className}
      kind="single"
      value={value}
      defaultValue={pressed === undefined && defaultPressed ? "on" : undefined}
      onValueChange={(val: string) => onPressedChange?.(val === "on")}
      items={[
        {
          value: "on",
          id,
          icon: addIconSize(icon),
          text: text,
        },
      ]}
    />
  );
}
export { Toggle };

// #endregion 🗡️Toggle
