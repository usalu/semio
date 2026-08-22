// #region 🧲️Header
// 💻️ framework/ui/elements/⚡️ActionGroup/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
// 🧱️core: reactHostPort imported directly from 🫀️core/Ports, NOT via the barrel — this component calls
// reactHostPort.createContext at module top level, which requires a non-circular import (see
// 🧱️elements/🔌️Ports/🟦️component.tsx's header comment for why the barrel import caused a real bug).
import { reactHostPort } from "../🔌️Ports/🟦️component.tsx";
// 🧱️core: cn imported directly from its presentation module, NOT via the barrel — this component calls
// cn(...) at module top level (inside a top-level styleVariants(cn(...)) call), which requires a non-circular
// import because the barrel imports this component.
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️component.ts";
import { styleVariants } from "../../🔨️modules/🏷️style-variants/🟦️component.ts";
import { type UiLabel } from "../🏷️UiLabel/🟦️component.tsx";
import { Popover, PopoverTrigger, PopoverContent } from "../🗨️Popover/🟦️component.tsx";
import { interactiveHoverClass, interactiveActiveFillClass } from "../../🔨️modules/🖱️interaction-presentation/🟦️component.ts";
import { menuListItemClassName } from "../../🔨️modules/📋️menu-item-presentation/🟦️component.ts";
import { formControlFocusBorderClass } from "../../🔨️modules/📝️form-control-presentation/🟦️component.ts";
import { borderNormalClass } from "../../🔨️modules/📏️border-presentation/🟦️component.ts";
import { chromeControlGroupShellClass, chromeControlItemBaseClass } from "../../🔨️modules/🎛️chrome-control-presentation/🟦️component.ts";
import { loadingBorderElementClass, waitingBorderElementClass } from "../../🔨️modules/🌀️status-border-presentation/🟦️component.ts";
import { useLevel, type Level } from "../🌈️Surface/🟦️component.tsx";
import { useControlAccessibleLabel, useControlInlineText, useControlTooltipText } from "../🏷️Label/🟦️component.tsx";
import { type ControlIcon, renderControlIcon, CheckIcon } from "../🔣️Icons/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🌩️ActionGroup
// Compact action button group with dropdown support.
// Consumers MUST provide action items for the group.

/**
 * actionGroupItemVariants holds the data fields for a actionGroupItemVariants record.
 **/
const actionGroupItemVariants = styleVariants(cn(chromeControlItemBaseClass, interactiveHoverClass, "shrink-0 [&_svg]:size-tiny aspect-square h-small p-single"));

/**
 * ActionGroupContext holds the data fields for a ActionGroupContext record.
 **/
const ActionGroupContext = reactHostPort.createContext<{ level: Level }>({
  level: "base",
});

/**
 * ActionGroupProps holds the data fields for a ActionGroupProps record.
 **/
interface ActionGroupProps extends Omit<React.ComponentProps<"div">, "children"> {
  children: React.ReactNode;
}

/**
 * ActionGroup holds the data fields for a ActionGroup record.
 **/
function ActionGroup({ className, children, ...props }: ActionGroupProps) {
  const level = useLevel();
  const contextValue = reactHostPort.useMemo(() => ({ level }), [level]);
  return (
    <div data-slot="action-group" data-detail-panel-control="fit" data-level={level} className={cn("group/action-group", chromeControlGroupShellClass, "h-small", className)} {...props}>
      <ActionGroupContext.Provider value={contextValue}>{children}</ActionGroupContext.Provider>
    </div>
  );
}

/**
 * ActionGroupItem holds the data fields for a ActionGroupItem record.
 **/
function ActionGroupItem({
  className,
  children,
  id,
  icon,
  text,
  as: Component = "button",
  ...props
}: React.ComponentProps<"button"> & {
  id?: string;
  icon: ControlIcon;
  text?: string;
  as?: "button" | "div";
}) {
  const context = reactHostPort.useContext(ActionGroupContext);
  const level = context.level ?? "base";
  const inlineText = useControlInlineText(id, text);
  const hasText = Boolean(inlineText);

  const accessibleLabel = useControlAccessibleLabel(id, text);
  const tooltipText = useControlTooltipText(id, text);
  const ariaLabel = inlineText ? undefined : accessibleLabel;

  const actionGroupItemElement = (
    <Component
      data-slot="action-group-item"
      id={id}
      type={Component === "button" ? "button" : undefined}
      role={Component === "div" && (props as any).onClick ? "button" : undefined}
      tabIndex={Component === "div" && (props as any).onClick ? 0 : undefined}
      aria-label={ariaLabel}
      title={tooltipText}
      data-level={context.level || level}
      className={cn(
        actionGroupItemVariants(),
        "min-w-0 shrink-0 focus:z-panel focus-visible:z-panel",
        !id && "flex-1",
        hasText && "aspect-auto gap-single",
        className,
      )}
      {...(props as any)}
    >
      {children}
      {inlineText ? (
        <span data-slot="inline-label" className="text-tiny whitespace-nowrap">
          {inlineText}
        </span>
      ) : null}
      {renderControlIcon(icon, "tiny")}
    </Component>
  );

  return actionGroupItemElement;
}

/**
 * ActionDropdownOption holds the data fields for a ActionDropdownOption record.
 **/
interface ActionDropdownOption {
  value: string;
  icon: ControlIcon;
  label?: UiLabel;
}

/**
 * ActionDropdownProps holds the data fields for a ActionDropdownProps record.
 **/
interface ActionDropdownProps extends Omit<React.ComponentProps<"button">, "children" | "id"> {
  id: string;
  options: ActionDropdownOption[];
  value: string;
  onValueChange?: (value: string) => void;
  startTransaction?: () => void;
  finalizeTransaction?: () => void;
}

/**
 * ActionDropdown holds the data fields for a ActionDropdown record.
 **/
function ActionDropdown({ className, id, options, value, onValueChange, startTransaction, finalizeTransaction, ...props }: ActionDropdownProps) {
  const [open, setOpen] = reactHostPort.useState(false);
  const level = useLevel();

  const selectedOption = options.find((option) => option.value === value);

  const handleOpenChange = (isOpen: boolean) => {
    if (isOpen) startTransaction?.();
    setOpen(isOpen);
    if (!isOpen) finalizeTransaction?.();
  };

  const handleSelect = (optionValue: string) => {
    if (onValueChange) onValueChange(optionValue);
    setOpen(false);
  };

  const buttonElement = (
    <Popover open={open} onOpenChange={handleOpenChange}>
      <PopoverTrigger asChild>
        <ActionGroup id={id} className={className}>
          <ActionGroupItem id={id} icon={selectedOption?.icon ?? "chevron-down"} {...props} />
        </ActionGroup>
      </PopoverTrigger>
      <PopoverContent className="w-auto p-single min-w-layout-popover" align="start">
        <div className="flex flex-col">
          {options.map((option) => (
            <button
              key={option.value}
              onClick={() => handleSelect(option.value)}
              className={cn("flex items-center gap-single p-single text-xs cursor-selectable outline-none", menuListItemClassName, value === option.value && interactiveActiveFillClass)}
            >
              <span className="flex items-center justify-center size-3">{renderControlIcon(option.icon, "tiny")}</span>
              {option.label && <span className="flex-1 text-start">{option.label}</span>}
              {value === option.value && <CheckIcon className="size-tiny ms-auto" />}
            </button>
          ))}
        </div>
      </PopoverContent>
    </Popover>
  );

  return buttonElement;
}

/**
 * ActionProps holds the data fields for a ActionProps record.
 **/
interface ActionProps extends Omit<React.ComponentProps<"button">, "children"> {
  as?: "button" | "div";
  loading?: boolean;
  waiting?: boolean;
  icon: ControlIcon;
  text?: string;
  id?: string;
}

/**
 * Action holds the data fields for a Action record.
 **/
function Action({ className, id, icon, text, as = "button", loading = false, waiting = false, ...props }: ActionProps) {
  const level = useLevel();
  const Comp = as;
  const inlineText = useControlInlineText(id, text);
  const accessibleLabel = useControlAccessibleLabel(id, text);
  const tooltipText = useControlTooltipText(id, text);
  const hasText = Boolean(inlineText);
  const ariaLabel = inlineText ? undefined : accessibleLabel;

  const actionElement = (
    <Comp
      data-slot="action"
      type={Comp === "button" ? "button" : undefined}
      role={Comp === "div" && (props as any).onClick ? "button" : undefined}
      tabIndex={Comp === "div" && (props as any).onClick ? 0 : undefined}
      id={id}
      aria-label={ariaLabel}
      aria-busy={loading || waiting || undefined}
      title={tooltipText}
      data-level={level}
      className={cn(
        `text-element inline-flex items-center justify-center shrink-0 cursor-selectable disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed [&_svg]:pointer-events-none [&_svg]:size-tiny [&_svg]:shrink-0 overflow-hidden aspect-square p-single h-medium border ${formControlFocusBorderClass}`,
        hasText && "aspect-auto gap-single",
        interactiveHoverClass,
        borderNormalClass,
        (loading && loadingBorderElementClass) || (waiting && waitingBorderElementClass),
        className,
      )}
      {...(props as any)}
    >
      {inlineText ? <span className="text-tiny whitespace-nowrap">{inlineText}</span> : null}
      {renderControlIcon(icon, "tiny")}
    </Comp>
  );

  return actionElement;
}

export { Action, ActionDropdown, ActionGroup, ActionGroupItem, actionGroupItemVariants };
export type { ActionDropdownOption, ActionDropdownProps, ActionProps };

// #endregion 🌩️ActionGroup
