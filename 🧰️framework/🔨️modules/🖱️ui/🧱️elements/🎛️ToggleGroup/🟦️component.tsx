// #region 🧲️Header
// 💻️ framework/ui/elements/🎛️ToggleGroup/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
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

// #region 🎛️Contracts
type ToggleGroupOrientation = "horizontal" | "vertical";
type ToggleGroupDirection = "ltr" | "rtl";

interface ToggleGroupItemProps extends Omit<React.ButtonHTMLAttributes<HTMLButtonElement>, "children" | "value"> {
  id?: string;
  icon: ControlIcon;
  text?: string;
  action?: React.ReactNode;
  value: string;
  ref?: React.Ref<HTMLButtonElement>;
}

interface ToggleGroupBaseProps extends Omit<React.HTMLAttributes<HTMLDivElement>, "defaultValue" | "dir" | "onChange"> {
  id?: string;
  showLabel?: boolean;
  items: ToggleGroupItemProps[];
  disabled?: boolean;
  orientation?: ToggleGroupOrientation;
  dir?: ToggleGroupDirection;
  loop?: boolean;
  rovingFocus?: boolean;
  ref?: React.Ref<HTMLDivElement>;
}

interface ToggleGroupSingleProps extends ToggleGroupBaseProps {
  kind?: "single";
  value?: string;
  defaultValue?: string;
  onValueChange?: (value: string) => void;
}

interface ToggleGroupMultipleProps extends ToggleGroupBaseProps {
  kind: "multiple";
  value?: string[];
  defaultValue?: string[];
  onValueChange?: (value: string[]) => void;
}

type ToggleGroupProps = ToggleGroupSingleProps | ToggleGroupMultipleProps;

interface ToggleGroupContextValue {
  level: Level;
  disabled: boolean;
  orientation: ToggleGroupOrientation;
  dir: ToggleGroupDirection;
  loop: boolean;
  rovingFocus: boolean;
  focusValue: string | undefined;
  selected: ReadonlySet<string>;
  activate: (value: string) => void;
  moveFocus: (event: React.KeyboardEvent<HTMLButtonElement>, value: string) => void;
  setFocusValue: (value: string) => void;
}
// #endregion 🎛️Contracts

// #region 🎛️ToggleGroup
const toggleVariants = styleVariants(cn(chromeControlItemClass, chromeControlItemOnClass, "aspect-square"));
const ToggleGroupContext = reactHostPort.createContext<ToggleGroupContextValue | null>(null);

/** 🎛️ Owns exact single/multiple selection and independent roving focus. */
function ToggleGroup(props: ToggleGroupProps) {
  const { className, id, showLabel, items, kind = "single", disabled = false, orientation = "horizontal", dir = "ltr", loop = true, rovingFocus = true, ref, ...rootProps } = props;
  const level = useLevel();
  const isMultiple = kind === "multiple";
  const controlled = props.value !== undefined;
  const [uncontrolledSingle, setUncontrolledSingle] = React.useState<string | undefined>(() => (kind === "single" ? (props as ToggleGroupSingleProps).defaultValue : undefined));
  const [uncontrolledMultiple, setUncontrolledMultiple] = React.useState<string[]>(() => (kind === "multiple" ? ((props as ToggleGroupMultipleProps).defaultValue ?? []) : []));
  const singleValue = isMultiple ? undefined : controlled ? (props as ToggleGroupSingleProps).value : uncontrolledSingle;
  const multipleValue = isMultiple ? (controlled ? ((props as ToggleGroupMultipleProps).value ?? []) : uncontrolledMultiple) : [];
  const selected = React.useMemo<ReadonlySet<string>>(() => new Set<string>(isMultiple ? multipleValue : singleValue ? [singleValue] : []), [isMultiple, multipleValue, singleValue]);
  const enabledItems = React.useMemo(() => items.filter((item) => !disabled && !item.disabled), [disabled, items]);
  const [focusValue, setFocusValue] = React.useState(() => enabledItems.find((item) => selected.has(item.value))?.value ?? enabledItems[0]?.value);

  React.useEffect(() => {
    if (focusValue && enabledItems.some((item) => item.value === focusValue)) return;
    setFocusValue(enabledItems.find((item) => selected.has(item.value))?.value ?? enabledItems[0]?.value);
  }, [enabledItems, focusValue, selected]);

  const activate = React.useCallback(
    (itemValue: string) => {
      if (disabled) return;
      if (isMultiple) {
        const next = selected.has(itemValue) ? multipleValue.filter((entry) => entry !== itemValue) : [...multipleValue, itemValue];
        if (!controlled) setUncontrolledMultiple(next);
        (props as ToggleGroupMultipleProps).onValueChange?.(next);
        return;
      }
      const next = singleValue === itemValue ? "" : itemValue;
      if (!controlled) setUncontrolledSingle(next);
      (props as ToggleGroupSingleProps).onValueChange?.(next);
    },
    [controlled, disabled, isMultiple, multipleValue, props, selected, singleValue],
  );
  const moveFocus = React.useCallback(
    (event: React.KeyboardEvent<HTMLButtonElement>, itemValue: string) => {
      if (!rovingFocus || event.target !== event.currentTarget) return;
      const previousKey = orientation === "vertical" ? "ArrowUp" : dir === "rtl" ? "ArrowRight" : "ArrowLeft";
      const nextKey = orientation === "vertical" ? "ArrowDown" : dir === "rtl" ? "ArrowLeft" : "ArrowRight";
      if (![previousKey, nextKey, "Home", "End"].includes(event.key)) return;
      const currentIndex = enabledItems.findIndex((item) => item.value === itemValue);
      if (currentIndex < 0 || enabledItems.length === 0) return;
      let nextIndex = event.key === "Home" ? 0 : event.key === "End" ? enabledItems.length - 1 : event.key === previousKey ? currentIndex - 1 : currentIndex + 1;
      if (loop) nextIndex = (nextIndex + enabledItems.length) % enabledItems.length;
      else nextIndex = Math.max(0, Math.min(enabledItems.length - 1, nextIndex));
      const nextValue = enabledItems[nextIndex]?.value;
      if (nextValue === undefined || nextValue === itemValue) return;
      event.preventDefault();
      const root = event.currentTarget.closest('[data-slot="toggle-group"]');
      const next = Array.from(root?.querySelectorAll<HTMLButtonElement>('[data-slot="toggle-group-item"]') ?? []).find((item) => item.dataset.toggleValue === nextValue);
      next?.focus();
      setFocusValue(nextValue);
    },
    [dir, enabledItems, loop, orientation, rovingFocus],
  );
  const context = React.useMemo<ToggleGroupContextValue>(
    () => ({ level, disabled, orientation, dir, loop, rovingFocus, focusValue, selected, activate, moveFocus, setFocusValue }),
    [activate, dir, disabled, focusValue, level, loop, moveFocus, orientation, rovingFocus, selected],
  );
  const rootDataState = selected.size > 0 ? "on" : "off";
  const { value: _value, defaultValue: _defaultValue, onValueChange: _onValueChange, ...htmlProps } = rootProps as ToggleGroupBaseProps & { value?: unknown; defaultValue?: unknown; onValueChange?: unknown };
  const element = (
    <div
      {...htmlProps}
      ref={ref}
      id={id}
      role="group"
      dir={dir}
      aria-disabled={disabled || undefined}
      data-slot="toggle-group"
      data-detail-panel-control="fit"
      data-state={rootDataState}
      data-disabled={disabled ? "" : undefined}
      data-orientation={orientation}
      className={cn(chromeControlGroupClass, "group/toggle-group has-[_[data-slot=inline-label]]:overflow-visible", className)}
    >
      <ToggleGroupContext.Provider value={context}>
        {items.map((item) => (
          <ToggleGroupItem key={item.value} {...item} id={item.id ?? (id ? `${id}-${item.value}` : undefined)} />
        ))}
      </ToggleGroupContext.Provider>
    </div>
  );
  return showLabel && id ? (
    <Label id={id} labelElementId={`${id}-label`}>
      {element}
    </Label>
  ) : (
    element
  );
}

/** 🔘 Renders one owned native toggle with any secondary action as a sibling control. */
function ToggleGroupItem({ className, id, icon, text, action, value, disabled = false, ref, onClick, onFocus, onKeyDown, "aria-label": suppliedAriaLabel, title: suppliedTitle, ...props }: ToggleGroupItemProps) {
  const context = reactHostPort.useContext(ToggleGroupContext);
  if (!context) throw new Error("ToggleGroupItem must be rendered inside ToggleGroup");
  const itemDisabled = context.disabled || disabled;
  const pressed = context.selected.has(value);
  const level = context.level ?? "base";
  const inlineText = useControlInlineText(id, text);
  const accessibleLabel = useControlAccessibleLabel(id, text);
  const tooltipText = useControlTooltipText(id, text);
  const ariaLabel = suppliedAriaLabel ?? (inlineText ? undefined : accessibleLabel);
  const title = suppliedTitle ?? tooltipText;
  return (
    <div data-slot="toggle-group-item-shell" className={action ? "flex min-w-0 flex-1 items-stretch" : "contents"}>
      <button
        {...props}
        ref={ref}
        type="button"
        id={id}
        disabled={itemDisabled}
        aria-label={ariaLabel}
        aria-pressed={pressed}
        title={title}
        tabIndex={itemDisabled ? -1 : context.rovingFocus ? (context.focusValue === value ? 0 : -1) : props.tabIndex}
        data-slot="toggle-group-item"
        data-toggle-value={value}
        data-level={level}
        data-state={pressed ? "on" : "off"}
        data-disabled={itemDisabled ? "" : undefined}
        data-orientation={context.orientation}
        className={cn(
          toggleVariants(),
          inlineText ? "w-auto shrink-0 focus:z-panel focus-visible:z-panel" : "min-w-0 flex-1 shrink-0 focus:z-panel focus-visible:z-panel",
          (inlineText || action) && "flex items-center gap-single py-single px-double aspect-auto",
          inlineText && "w-auto",
          className,
        )}
        onClick={(event) => {
          onClick?.(event);
          if (!event.defaultPrevented && !itemDisabled) context.activate(value);
        }}
        onFocus={(event) => {
          onFocus?.(event);
          if (!event.defaultPrevented && !itemDisabled) context.setFocusValue(value);
        }}
        onKeyDown={(event) => {
          onKeyDown?.(event);
          if (!event.defaultPrevented && !itemDisabled) context.moveFocus(event, value);
        }}
      >
        {inlineText ? (
          <span data-slot="inline-label" className="text-xs whitespace-nowrap">
            {inlineText}
          </span>
        ) : null}
        <ControlHotkeyBadge id={id} allowInline={Boolean(inlineText)} />
        <span className={action ? "flex-1 flex items-center justify-center" : undefined}>{renderControlIcon(icon)}</span>
      </button>
      {action ? (
        <div data-slot="toggle-group-item-action" aria-disabled={itemDisabled || undefined} className={cn("flex items-center justify-center aspect-square h-full flex-shrink-0", surfaceClass, text && "ms-single")}>
          {action}
        </div>
      ) : null}
    </div>
  );
}

export { ToggleGroup, ToggleGroupItem };
export type { ToggleGroupProps, ToggleGroupSingleProps, ToggleGroupMultipleProps, ToggleGroupItemProps, ToggleGroupOrientation, ToggleGroupDirection };
// #endregion 🎛️ToggleGroup
