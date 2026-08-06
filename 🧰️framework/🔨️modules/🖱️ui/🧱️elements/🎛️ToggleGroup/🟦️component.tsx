// #region 🧲️Header
// 💻️ framework/ui/elements/🎛️ToggleGroup/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import * as ToggleGroupPrimitive from "@radix-ui/react-toggle-group";
import { toggleVariants, type ToggleProps, type ToggleStandardProps, type ToggleWithActionProps, type ToggleDropdownProps } from "../../🧱️elements/🎚️Toggle/🟦️component.tsx";
// 🧱️core: reactHostPort imported directly from 🫀️core/Ports, NOT via the barrel — this component calls
// reactHostPort.createContext at module top level, which requires a non-circular import (see
// 🧱️elements/🫀️core/🔌Ports/🟦️component.tsx's header comment for why the barrel import caused a real bug).
import { reactHostPort } from "../🫀️core/🔌Ports/🟦️component.tsx";
import { cn } from "../🫀️core/🏷️ClassNames/🟦️component.tsx";
import { Action } from "../⚡️ActionGroup/🟦️component.tsx";
import { Popover, PopoverTrigger, PopoverContent } from "../🗨️Popover/🟦️component.tsx";
import { surfaceClass } from "../🫀️core/🏷️ClassNames/🟦️component.tsx";
import { type Level, useLevel } from "../🫀️core/🌈️Surface/🟦️component.tsx";
import { Label, useControlInlineText, useControlAccessibleLabel } from "../🫀️core/🏷️Label/🟦️component.tsx";
import { chromeControlGroupClass, ControlHotkeyBadge, ChromeControlHint } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
import { renderControlIcon, ChevronDownIcon, type ControlIcon } from "../🔣Icons/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🧩️ToggleGroup
// Group of mutually exclusive or multi-select toggles.
// Consumers MUST provide items with distinct values.

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
  const ariaLabel = inlineText ? undefined : accessibleLabel;

  const toggleGroupItemElement = (
    <ToggleGroupPrimitive.Item
      data-slot="toggle-group-item"
      id={id}
      aria-label={ariaLabel}
      title={ariaLabel}
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
export { Toggle, ToggleGroup, ToggleGroupItem, toggleVariants };

// #endregion 🧩️ToggleGroup
