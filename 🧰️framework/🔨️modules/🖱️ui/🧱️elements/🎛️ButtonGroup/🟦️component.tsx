// #region 🧲️Header
// 💻️ framework/ui/elements/🎛️ButtonGroup/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { Slot } from "@radix-ui/react-slot";
import { reactHostPort } from "../🔌️Ports/🟦️component.tsx";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️component.ts";
import { styleVariants, type StyleVariantProps } from "../../🔨️modules/🏷️style-variants/🟦️component.ts";
import { borderElementClass } from "../../🔨️modules/📏️border-presentation/🟦️component.ts";
import { ControlHotkeyBadge } from "../../🔨️modules/⌨️control-hotkey-presentation/🟦️component.tsx";
import { chromeControlGroupClass, chromeControlItemClass } from "../../🔨️modules/🎛️chrome-control-presentation/🟦️component.ts";
import { useLevel, type Level } from "../🌈️Surface/🟦️component.tsx";
import { Label, useControlInlineText, useControlAccessibleLabel, useControlTooltipText } from "../🏷️Label/🟦️component.tsx";
import { type ControlIcon, renderControlIcon } from "../🔣️Icons/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🌩️ButtonGroup
// Grouped control buttons with shared level context.
// Consumers MUST provide ButtonGroupItem children.

/**
 * buttonGroupItemVariants holds the data fields for a buttonGroupItemVariants record.
 **/
const buttonGroupItemVariants = styleVariants(cn(chromeControlItemClass, "aspect-square"), {
  variants: {
    variant: {
      default: "",
      ghost: "border-transparent bg-transparent",
      outline: `border ${borderElementClass}`,
    },
  },
  defaultVariants: {
    variant: "default",
  },
});

/**
 * ButtonGroupContext holds the data fields for a ButtonGroupContext record.
 **/
const ButtonGroupContext = reactHostPort.createContext<{ level: Level }>({
  level: "base",
});

/**
 * ButtonGroupProps holds the data fields for a ButtonGroupProps record.
 **/
interface ButtonGroupProps extends Omit<React.ComponentProps<"div">, "id"> {
  detailPanelWidthMode?: "fit" | "fill";
  id?: string;
  showLabel?: boolean;
  children: React.ReactNode;
}

/**
 * ButtonGroup holds the data fields for a ButtonGroup record.
 **/
function ButtonGroup({ className, detailPanelWidthMode = "fit", id, showLabel, children, ...props }: ButtonGroupProps) {
  const level = useLevel();
  const buttonGroupContextValue = reactHostPort.useMemo(() => ({ level }), [level]);
  const buttonGroupElement = (
    <ButtonGroupContext.Provider value={buttonGroupContextValue}>
      <div
        data-slot="button-group"
        data-detail-panel-control={detailPanelWidthMode}
        id={id}
        data-level={level}
        className={cn(chromeControlGroupClass, detailPanelWidthMode === "fill" ? "w-full min-w-0" : "", "group/button-group", className)}
        {...props}
      >
        {children}
      </div>
    </ButtonGroupContext.Provider>
  );

  if (showLabel && id) {
    return (
      <Label id={id} labelElementId={`${id}-label`}>
        {buttonGroupElement}
      </Label>
    );
  }

  return buttonGroupElement;
}

/**
 * ButtonGroupItem holds the data fields for a ButtonGroupItem record.
 **/
type ButtonGroupItemProps = React.ComponentProps<"button"> &
  StyleVariantProps<typeof buttonGroupItemVariants> & {
    id?: string;
    icon: ControlIcon;
    text?: string;
    asChild?: boolean;
  };

function ButtonGroupItem({
  className,
  children,
  id,
  icon,
  text,
  asChild = false,
  variant,
  ...props
}: ButtonGroupItemProps) {
  const context = reactHostPort.useContext(ButtonGroupContext);
  const level = context.level ?? "base";
  const Comp = asChild ? Slot : "button";
  const inlineText = useControlInlineText(id, text);
  const accessibleLabel = useControlAccessibleLabel(id, text);
  const tooltipText = useControlTooltipText(id, text);
  const ariaLabel = inlineText ? undefined : accessibleLabel;

  const buttonGroupItemElement = (
    <Comp
      data-slot="button-group-item"
      id={id}
      aria-label={ariaLabel}
      title={tooltipText}
      data-level={context.level || level}
      className={cn(
        buttonGroupItemVariants({ variant }),
        inlineText ? "w-auto shrink-0 focus:z-panel focus-visible:z-panel" : "min-w-0 flex-1 shrink-0 focus:z-panel focus-visible:z-panel",
        inlineText && "flex items-center gap-single py-single px-double w-auto aspect-auto",
        className,
      )}
      {...(props as any)}
    >
      {children}
      {inlineText ? (
        <span data-slot="inline-label" className={cn("min-w-0 text-xs whitespace-nowrap", /\bjustify-between\b/.test(String(className ?? "")) && "flex-1 truncate")}>
          {inlineText}
        </span>
      ) : null}
      <ControlHotkeyBadge id={id} allowInline={Boolean(inlineText)} />
      {renderControlIcon(icon)}
    </Comp>
  );

  return buttonGroupItemElement;
}

export { ButtonGroup, ButtonGroupItem, buttonGroupItemVariants };
export type { ButtonGroupProps };
// #endregion 🌩️ButtonGroup
