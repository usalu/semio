// #region 🧲️Header
// 💻️ framework/ui/elements/☑️Select/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import * as SelectPrimitive from "@radix-ui/react-select";
import { cn } from "../🫀️core/🏷️ClassNames/🟦️component.tsx";
import { reactHostPort } from "../🫀️core/🔌Ports/🟦️component.tsx";
import { borderElementClass, formControlFocusBorderClass, interactiveHoverClass, glassClass, menuListItemClassName } from "../🫀️core/🏷️ClassNames/🟦️component.tsx";
import { useTransaction, type ElementProps } from "../🫀️core/🐹️ElementProps/🟦️component.tsx";
import { SurfaceScope, useLevel } from "../🫀️core/🌈️Surface/🟦️component.tsx";
import { Label } from "../🫀️core/🏷️Label/🟦️component.tsx";
import { useFlow } from "../🫀️core/🧭️Flow/🟦️component.tsx";
import { CheckIconAlt, ChevronDownIconAlt, ChevronUpIcon, type IconSource, Icon } from "../🔣Icons/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🔎️Select
// Dropdown select built on Radix primitives.
// Consumers MUST use SelectItem children for options.

/**
 * Select holds the data fields for a Select record.
 **/
function Select({ id, showLabel, children, value, defaultValue, onOpenChange, ...props }: React.ComponentProps<typeof SelectPrimitive.Root> & ElementProps & { showLabel?: boolean }) {
  const transaction = useTransaction();
  const fallbackValue = reactHostPort.useMemo(() => {
    const findValue = (nodes: React.ReactNode[]): string | undefined => {
      for (const node of nodes) {
        if (!React.isValidElement(node)) {
          continue;
        }
        const nodeProps = node.props as { "data-slot"?: string; value?: string; children?: React.ReactNode };
        if ((node.type === SelectPrimitive.Item || nodeProps["data-slot"] === "select-item") && nodeProps.value !== undefined) {
          return nodeProps.value as string;
        }
        const nested = React.Children.toArray(nodeProps.children);
        if (nested.length) {
          const nestedValue = findValue(nested);
          if (nestedValue !== undefined) {
            return nestedValue;
          }
        }
      }
      return undefined;
    };
    return findValue(React.Children.toArray(children));
  }, [children]);

  const handleOpenChange = (open: boolean) => {
    if (open) {
      transaction?.start?.();
    } else {
      transaction?.finalize?.();
    }
    onOpenChange?.(open);
  };

  const selectElement = (
    <SelectPrimitive.Root
      onOpenChange={handleOpenChange}
      data-slot="select"
      {...(value !== null && value !== undefined ? { value } : defaultValue !== null && defaultValue !== undefined ? { defaultValue } : fallbackValue !== undefined ? { defaultValue: fallbackValue } : {})}
      {...props}
    >
      {children}
    </SelectPrimitive.Root>
  );

  if (showLabel && id) {
    return (
      <Label id={id} labelElementId={`${id}-label`}>
        {selectElement}
      </Label>
    );
  }

  return selectElement;
}

/**
 * SelectGroup holds the data fields for a SelectGroup record.
 **/
function SelectGroup({ ...props }: React.ComponentProps<typeof SelectPrimitive.Group>) {
  return <SelectPrimitive.Group data-slot="select-group" {...props} />;
}

/**
 * SelectValue holds the data fields for a SelectValue record.
 **/
function SelectValue({ ...props }: React.ComponentProps<typeof SelectPrimitive.Value>) {
  return <SelectPrimitive.Value data-slot="select-value" {...props} />;
}

/**
 * SelectTrigger holds the data fields for a SelectTrigger record.
 **/
function SelectTrigger({
  className,
  size = "default",
  children,
  id,
  ...props
}: React.ComponentProps<typeof SelectPrimitive.Trigger> & {
  size?: "sm" | "default";
  id?: string;
}) {
  const level = useLevel();

  return (
    <SelectPrimitive.Trigger
      data-slot="select-trigger"
      data-detail-panel-control="fill"
      id={id}
      data-size={size}
      data-level={level}
      className={cn(
        `text-element data-[placeholder]:text-muted-foreground [&_svg:not([class*='text-'])]:text-muted-foreground flex w-fit min-w-0 items-center justify-between gap-single border bg-transparent px-tiny py-single text-sm whitespace-nowrap ${borderElementClass} ${formControlFocusBorderClass} disabled:cursor-not-allowed disabled:opacity-50 h-medium *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex *:data-[slot=select-value]:min-w-0 *:data-[slot=select-value]:flex-1 *:data-[slot=select-value]:items-center *:data-[slot=select-value]:gap-single *:data-[slot=select-value]:overflow-hidden [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-tiny cursor-foldable`,
        interactiveHoverClass,
        className,
      )}
      {...props}
    >
      {children as React.ReactNode}
      <SelectPrimitive.Icon asChild>
        <span data-slot="select-chevron" className="pointer-events-none inline-flex size-small shrink-0 items-center justify-center text-muted-foreground">
          <Icon icon="chevron-down" size="small" />
        </span>
      </SelectPrimitive.Icon>
    </SelectPrimitive.Trigger>
  );
}

/**
 * SelectContent holds the data fields for a SelectContent record.
 **/
function SelectContent({ className, children, position = "popper", ...props }: React.ComponentProps<typeof SelectPrimitive.Content>) {
  const flow = useFlow();
  return (
    <SelectPrimitive.Portal>
      <SelectPrimitive.Content
        data-slot="select-content"
        data-level="menu"
        dir={flow.inline === "rtl" ? "rtl" : undefined}
        className={cn(
          "text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 relative z-menu max-h-(--radix-select-content-available-height) min-w-32 origin-(--radix-select-content-transform-origin) overflow-x-hidden overflow-y-auto border",
          glassClass,
          position === "popper" && "data-[side=bottom]:translate-y-1 data-[side=left]:-translate-x-1 data-[side=right]:translate-x-1 data-[side=top]:-translate-y-1",
          className,
        )}
        position={position}
        {...props}
      >
        <SurfaceScope level="menu" fill="glass">
          <SelectScrollUpButton />
          <SelectPrimitive.Viewport className={cn("p-single", position === "popper" && "h-[var(--radix-select-trigger-height)] w-full min-w-[var(--radix-select-trigger-width)] scroll-my-single")}>{children}</SelectPrimitive.Viewport>
          <SelectScrollDownButton />
        </SurfaceScope>
      </SelectPrimitive.Content>
    </SelectPrimitive.Portal>
  );
}

/**
 * SelectLabel holds the data fields for a SelectLabel record.
 **/
function SelectLabel({ className, ...props }: React.ComponentProps<typeof SelectPrimitive.Label>) {
  return <SelectPrimitive.Label data-slot="select-label" className={cn("text-muted-foreground p-single text-xs", className)} {...props} />;
}

/**
 * SelectItem holds the data fields for a SelectItem record.
 **/
function SelectItem({
  className,
  children,
  id,
  icon,
  ...props
}: React.ComponentProps<typeof SelectPrimitive.Item> & { id?: string; icon?: IconSource }) {
  return (
    <SelectPrimitive.Item
      data-slot="select-item"
      id={id}
      className={cn(
        "focus:text-emphasized [&_svg:not([class*='text-'])]:text-muted-foreground relative flex w-full items-center gap-single rounded-sm py-single pe-medium ps-single text-sm outline-hidden select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-tiny *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-single",
        "cursor-selectable",
        menuListItemClassName,
        className,
      )}
      {...props}
    >
      <span className="absolute end-2 flex size-tiny.5 items-center justify-center">
        <SelectPrimitive.ItemIndicator>
          <CheckIconAlt className="size-tiny" />
        </SelectPrimitive.ItemIndicator>
      </span>
      {icon ? <span data-slot="select-item-icon" className="inline-flex shrink-0"><Icon icon={icon} size="small" /></span> : null}
      <SelectPrimitive.ItemText>{children}</SelectPrimitive.ItemText>
    </SelectPrimitive.Item>
  );
}

/**
 * SelectSeparator holds the data fields for a SelectSeparator record.
 **/
function SelectSeparator({ className, ...props }: React.ComponentProps<typeof SelectPrimitive.Separator>) {
  return <SelectPrimitive.Separator data-slot="select-separator" className={cn("bg-border pointer-events-none -mx-single my-single h-px", className)} {...props} />;
}

/**
 * SelectScrollUpButton holds the data fields for a SelectScrollUpButton record.
 **/
function SelectScrollUpButton({ className, ...props }: React.ComponentProps<typeof SelectPrimitive.ScrollUpButton>) {
  return (
    <SelectPrimitive.ScrollUpButton data-slot="select-scroll-up-button" className={cn("flex cursor-default items-center justify-center py-single hover:bg-hover-interactive-fill", className)} {...props}>
      <ChevronUpIcon className="size-tiny" />
    </SelectPrimitive.ScrollUpButton>
  );
}

/**
 * SelectScrollDownButton holds the data fields for a SelectScrollDownButton record.
 **/
function SelectScrollDownButton({ className, ...props }: React.ComponentProps<typeof SelectPrimitive.ScrollDownButton>) {
  return (
    <SelectPrimitive.ScrollDownButton data-slot="select-scroll-down-button" className={cn("flex cursor-default items-center justify-center py-single hover:bg-hover-interactive-fill", className)} {...props}>
      <ChevronDownIconAlt className="size-tiny" />
    </SelectPrimitive.ScrollDownButton>
  );
}

export { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectScrollDownButton, SelectScrollUpButton, SelectSeparator, SelectTrigger, SelectValue };

// #endregion 🔎️Select
