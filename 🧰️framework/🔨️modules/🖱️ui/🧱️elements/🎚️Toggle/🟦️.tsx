// #region 🧲️Header
// 💻️ framework/ui/elements/🎚️Toggle/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { ControlHotkeyBadge } from "../../🔨️modules/⌨️control-hotkey-presentation/🟦️.tsx";
import { type UiLabel } from "../🏷️UiLabel/🟦️.tsx";
import { type ElementProps } from "../../🔨️modules/🆔️element-identity/🟦️.ts";
import { chromeControlGroupClass, chromeControlItemClass, chromeControlItemOnClass } from "../../🔨️modules/🎛️chrome-control-presentation/🟦️.ts";
import { ToggleGroup } from "../🎛️ToggleGroup/🟦️.tsx";
import { reactHostPort } from "../🔌️Ports/🟦️.tsx";
import { Action } from "../⚡️ActionGroup/🟦️.tsx";
import { Popover, PopoverTrigger, PopoverContent } from "../🗨️Popover/🟦️.tsx";
import { ChromeControlHint } from "../💡️ChromeControlHint/🟦️.tsx";
import { Label, useControlAccessibleLabel, useControlInlineText, useControlTooltipText } from "../🏷️Label/🟦️.tsx";
import { useLevel } from "../🌈️Surface/🟦️.tsx";
import { renderControlIcon, ChevronDownIcon, type ControlIcon } from "../🔣️Icons/🟦️.tsx";
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

/** 🗡️ Native button and owned pressed-state contract shared by toggle variants. */
interface ToggleButtonProps extends Omit<React.ButtonHTMLAttributes<HTMLButtonElement>, "children" | "defaultValue" | "id" | "onChange" | "type" | "value"> {
  pressed?: boolean;
  defaultPressed?: boolean;
  onPressedChange?: (pressed: boolean) => void;
  ref?: React.Ref<HTMLButtonElement>;
}

/**
 * ToggleStandardProps holds the data fields for a ToggleStandardProps record.
 **/
interface ToggleStandardProps extends ToggleButtonProps, ElementProps {
  kind?: "default" | "icon" | "single";
  i18nPressed?: string;
  showLabel?: boolean;
  icon: ControlIcon;
  text?: string;
}

/**
 * ToggleWithActionProps holds the data fields for a ToggleWithActionProps record.
 **/
interface ToggleWithActionProps extends ToggleButtonProps, ElementProps {
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
interface ToggleDropdownProps<T extends string> extends ToggleButtonProps, ElementProps {
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

// #region 🟢️PressedButton
/** 🟢️ Owns native controlled and uncontrolled pressed-button semantics. */
function ToggleStandardButton({
  id,
  showLabel,
  className,
  icon,
  text,
  kind: _kind,
  i18nPressed: _i18nPressed,
  pressed,
  defaultPressed = false,
  onPressedChange,
  disabled,
  onClick,
  onKeyDown,
  onKeyUp,
  "aria-label": suppliedAriaLabel,
  title: suppliedTitle,
  ref,
  ...buttonProps
}: ToggleStandardProps) {
  const [uncontrolledPressed, setUncontrolledPressed] = reactHostPort.useState(defaultPressed);
  const isControlled = pressed !== undefined;
  const currentPressed = isControlled ? pressed : uncontrolledPressed;
  const level = useLevel();
  const inlineText = useControlInlineText(id, text);
  const accessibleLabel = useControlAccessibleLabel(id, text);
  const tooltipText = useControlTooltipText(id, text);
  const ariaLabel = suppliedAriaLabel ?? (inlineText ? undefined : accessibleLabel);
  const title = suppliedTitle ?? tooltipText;

  const activate = () => {
    const nextPressed = !currentPressed;
    if (!isControlled) setUncontrolledPressed(nextPressed);
    onPressedChange?.(nextPressed);
  };
  const handleClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    onClick?.(event);
    if (!event.defaultPrevented && !disabled) activate();
  };
  const handleKeyDown = (event: React.KeyboardEvent<HTMLButtonElement>) => {
    onKeyDown?.(event);
    if (event.defaultPrevented || disabled) return;
    if (event.key === "Enter") {
      event.preventDefault();
      event.currentTarget.click();
    } else if (event.key === " ") {
      event.preventDefault();
    }
  };
  const handleKeyUp = (event: React.KeyboardEvent<HTMLButtonElement>) => {
    onKeyUp?.(event);
    if (event.defaultPrevented || disabled || event.key !== " ") return;
    event.preventDefault();
    event.currentTarget.click();
  };

  const button = (
    <button
      {...buttonProps}
      ref={ref}
      type="button"
      id={id}
      disabled={disabled}
      aria-label={ariaLabel}
      aria-pressed={currentPressed}
      title={title}
      data-slot="toggle-group-item"
      data-level={level}
      data-state={currentPressed ? "on" : "off"}
      className={cn(
        chromeControlItemClass,
        chromeControlItemOnClass,
        "aspect-square",
        inlineText ? "w-auto shrink-0 focus:z-panel focus-visible:z-panel" : "min-w-0 flex-1 shrink-0 focus:z-panel focus-visible:z-panel",
        inlineText && "flex items-center gap-single py-single px-double aspect-auto",
      )}
      onClick={handleClick}
      onKeyDown={handleKeyDown}
      onKeyUp={handleKeyUp}
    >
      {inlineText ? (
        <span data-slot="inline-label" className="text-xs whitespace-nowrap">
          {inlineText}
        </span>
      ) : null}
      <ControlHotkeyBadge id={id} allowInline={Boolean(inlineText)} />
      <span>{renderControlIcon(addIconSize(icon))}</span>
    </button>
  );
  const toggle = (
    <div data-slot="toggle-group" data-detail-panel-control="fit" data-state={currentPressed ? "on" : "off"} role="group" className={cn(chromeControlGroupClass, "group/toggle-group has-[_[data-slot=inline-label]]:overflow-visible", className)}>
      {button}
    </div>
  );

  return showLabel ? (
    <Label id={id} labelElementId={`${id}-label`}>
      {toggle}
    </Label>
  ) : (
    toggle
  );
}
// #endregion 🟢️PressedButton

/**
 * Toggle holds the data fields for a Toggle record.
 **/
function Toggle<T extends string = string>(props: ToggleProps<T>) {
  if ("kind" in props && props.kind === "withAction") {
    const { actionIcon, onActionClick, icon, text, pressed, defaultPressed, onPressedChange, id, showLabel, className, actionId, disabled, ref } = props as ToggleWithActionProps;
    const controlledValueProps = pressed !== undefined ? { value: pressed ? ["on"] : [] } : { defaultValue: defaultPressed ? ["on"] : [] };
    return (
      <ToggleGroup
        showLabel={showLabel}
        kind="multiple"
        disabled={disabled}
        {...controlledValueProps}
        onValueChange={(val: string[]) => onPressedChange?.(val.includes("on"))}
        className={className}
        items={[
          {
            value: "on",
            icon: addIconSize(icon),
            text: text,
            action: <Action as="button" disabled={disabled} id={actionId} icon={addIconSize(actionIcon)} onClick={onActionClick} />,
            id: id,
            ref,
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
      disabled,
      ref,
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
        <PopoverTrigger asChild>
          <Action as="button" type="button" disabled={disabled} id={dropdownId} icon={<ChevronDownIcon className="size-small" />} />
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
      disabled,
      onValueChange: handleToggleGroupValueChange,
      className,
      items: [
        {
          value: selectedItem.value,
          icon: addIconSize(selectedItem.icon),
          text: selectedItem.text,
          action: dropdownAction,
          id: selectedItem.id,
          ref,
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

  return <ToggleStandardButton {...(props as ToggleStandardProps)} />;
}
export { Toggle };

// #endregion 🗡️Toggle
