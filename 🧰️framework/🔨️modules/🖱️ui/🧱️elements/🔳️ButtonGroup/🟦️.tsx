// #region 🧲️Header
// 💻️ framework/ui/elements/🎛️ButtonGroup/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { reactHostPort } from "../🔌️Ports/🟦️.tsx";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { Slot } from "../../🔨️modules/🏷️class-name-composition/🪆️slot.tsx";
import { styleVariants, type StyleVariantProps } from "../../🔨️modules/🧬️style-variants/🟦️.ts";
import { borderElementClass } from "../../🔨️modules/📏️border-presentation/🟦️.ts";
import { ControlHotkeyBadge } from "../../🔨️modules/⌨️control-hotkey-presentation/🟦️.tsx";
import { chromeControlGroupClass, chromeControlItemClass } from "../../🔨️modules/🎛️chrome-control-presentation/🟦️.ts";
import { useLevel, type Level } from "../🌈️Surface/🟦️.tsx";
import { Label, useControlInlineText, useControlAccessibleLabel, useControlTooltipText } from "../🏷️Label/🟦️.tsx";
import { type ControlIcon, renderControlIcon } from "../🔣️Icons/🟦️.tsx";
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

function ButtonGroupItem({ className, children, id, icon, text, asChild = false, variant, ...props }: ButtonGroupItemProps) {
  const context = reactHostPort.useContext(ButtonGroupContext);
  const level = context.level ?? "base";
  const inlineText = useControlInlineText(id, text);
  const accessibleLabel = useControlAccessibleLabel(id, text);
  const tooltipText = useControlTooltipText(id, text);
  const ariaLabel = inlineText ? undefined : accessibleLabel;

  const decorations = (
    <>
      {inlineText ? (
        <span data-slot="inline-label" className={cn("min-w-0 text-xs whitespace-nowrap", /\bjustify-between\b/.test(String(className ?? "")) && "flex-1 truncate")}>
          {inlineText}
        </span>
      ) : null}
      <ControlHotkeyBadge id={id} allowInline={Boolean(inlineText)} />
      {renderControlIcon(icon)}
    </>
  );
  const itemProps = {
    "data-slot": "button-group-item",
    id,
    "aria-label": ariaLabel,
    title: tooltipText,
    "data-level": context.level || level,
    className: cn(
      buttonGroupItemVariants({ variant }),
      inlineText ? "w-auto shrink-0 focus:z-panel focus-visible:z-panel" : "min-w-0 flex-1 shrink-0 focus:z-panel focus-visible:z-panel",
      inlineText && "flex items-center gap-single py-single px-double w-auto aspect-auto",
      className,
    ),
    ...(props as any),
  };

  if (asChild) {
    if (React.Children.count(children) !== 1 || !React.isValidElement<{ children?: React.ReactNode }>(children)) {
      throw new Error("ButtonGroupItem with asChild requires exactly one valid React element child.");
    }
    const child = React.cloneElement(children, undefined, children.props.children, decorations);
    return <Slot {...itemProps}>{child}</Slot>;
  }

  return (
    <button {...itemProps}>
      {children}
      {decorations}
    </button>
  );
}

export { ButtonGroup, ButtonGroupItem, buttonGroupItemVariants };
export type { ButtonGroupProps };
// #endregion 🌩️ButtonGroup
