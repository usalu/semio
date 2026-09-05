// #region 🧲️Header
// 💻️ framework/ui/elements/↕️Collapsible/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { Slot } from "../../🔨️modules/🏷️class-name-composition/🪆️slot.tsx";
import { interactiveControlTransitionClass, interactiveHoverClass } from "../../🔨️modules/🖱️interaction-presentation/🟦️.ts";
// #endregion 🔌️Adapters

// #region ↕️Collapsible
// #region 📐️Contract
/** ↕️ Owned state and host-element contract for a collapsible region. */
export type CollapsibleProps = Omit<React.HTMLAttributes<HTMLDivElement>, "onChange"> & {
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?: (open: boolean) => void;
  disabled?: boolean;
  contentId?: string;
};

/** 🖱️ Owned trigger contract supporting the existing exactly-one-child composition boundary. */
export type CollapsibleTriggerProps = Omit<React.ButtonHTMLAttributes<HTMLButtonElement>, "children"> & {
  asChild?: boolean;
  children?: React.ReactNode;
};

/** 📂 Owned host-element contract for collapsible content whose visibility follows disclosure state. */
export type CollapsibleContentProps = Omit<React.HTMLAttributes<HTMLDivElement>, "hidden">;

interface CollapsibleContextValue {
  contentId: string;
  disabled: boolean;
  open: boolean;
  setOpen: (open: boolean) => void;
}

const CollapsibleContext = React.createContext<CollapsibleContextValue | null>(null);

const useCollapsibleContext = (): CollapsibleContextValue => {
  const context = React.useContext(CollapsibleContext);
  if (!context) throw new Error("CollapsibleTrigger and CollapsibleContent must render inside Collapsible.");
  return context;
};
// #endregion 📐️Contract

// #region 🎛️Root
/** 🎛️ Provides controlled or uncontrolled disclosure state to one collapsible region. */
const Collapsible = React.forwardRef<HTMLDivElement, CollapsibleProps>(function Collapsible({ children, contentId: providedContentId, defaultOpen = false, disabled = false, onOpenChange, open: controlledOpen, ...props }, forwardedRef) {
  const [uncontrolledOpen, setUncontrolledOpen] = React.useState(defaultOpen);
  const generatedId = React.useId();
  const contentId = providedContentId ?? `semio-collapsible-${generatedId.replace(/[^A-Za-z0-9_-]/g, "")}-content`;
  const open = controlledOpen ?? uncontrolledOpen;
  const setOpen = React.useCallback(
    (nextOpen: boolean) => {
      if (disabled || nextOpen === open) return;
      if (controlledOpen === undefined) setUncontrolledOpen(nextOpen);
      onOpenChange?.(nextOpen);
    },
    [controlledOpen, disabled, onOpenChange, open],
  );
  const context = React.useMemo(() => ({ contentId, disabled, open, setOpen }), [contentId, disabled, open, setOpen]);

  return (
    <CollapsibleContext.Provider value={context}>
      <div ref={forwardedRef} {...props} data-slot="collapsible" data-state={open ? "open" : "closed"} data-disabled={disabled ? "" : undefined}>
        {children}
      </div>
    </CollapsibleContext.Provider>
  );
});
// #endregion 🎛️Root

// #region 🖱️Trigger
/** 🖱️ Toggles its owning collapsible through pointer or keyboard activation. */
const CollapsibleTrigger = React.forwardRef<HTMLElement, CollapsibleTriggerProps>(function CollapsibleTrigger({ asChild = false, children, className, disabled: triggerDisabled = false, onClick, onKeyDown, onKeyUp, type, ...props }, forwardedRef) {
  const context = useCollapsibleContext();
  const disabled = context.disabled || triggerDisabled;
  const activate = React.useCallback(() => {
    if (!disabled) context.setOpen(!context.open);
  }, [context, disabled]);
  const handleClick = (event: React.MouseEvent<HTMLElement>) => {
    if (disabled) {
      event.preventDefault();
      event.stopPropagation();
      return;
    }
    (onClick as React.MouseEventHandler<HTMLElement> | undefined)?.(event);
    if (!event.defaultPrevented) activate();
  };
  const handleKeyDown = (event: React.KeyboardEvent<HTMLElement>) => {
    if (disabled && (event.key === "Enter" || event.key === " ")) {
      event.preventDefault();
      event.stopPropagation();
      return;
    }
    (onKeyDown as React.KeyboardEventHandler<HTMLElement> | undefined)?.(event);
    if (event.defaultPrevented || event.repeat || event.currentTarget.tagName === "BUTTON") return;
    if (event.key === "Enter") {
      event.preventDefault();
      activate();
    } else if (event.key === " ") {
      event.preventDefault();
    }
  };
  const handleKeyUp = (event: React.KeyboardEvent<HTMLElement>) => {
    if (disabled && event.key === " ") {
      event.preventDefault();
      event.stopPropagation();
      return;
    }
    (onKeyUp as React.KeyboardEventHandler<HTMLElement> | undefined)?.(event);
    if (event.defaultPrevented || event.key !== " " || event.currentTarget.tagName === "BUTTON") return;
    event.preventDefault();
    activate();
  };
  const sharedProps: React.HTMLAttributes<HTMLElement> & {
    "data-disabled"?: "";
    "data-slot": "collapsible-trigger";
    "data-state": "closed" | "open";
  } = {
    ...props,
    "aria-controls": context.contentId,
    "aria-disabled": disabled || undefined,
    "aria-expanded": context.open,
    className: cn("cursor-selectable", interactiveControlTransitionClass, interactiveHoverClass, className),
    "data-disabled": disabled ? "" : undefined,
    "data-slot": "collapsible-trigger",
    "data-state": context.open ? "open" : "closed",
    onClick: handleClick,
    onKeyDown: handleKeyDown,
    onKeyUp: handleKeyUp,
    role: "button",
  };

  if (asChild) {
    return (
      <Slot ref={forwardedRef} {...sharedProps} tabIndex={disabled ? -1 : (props.tabIndex ?? 0)}>
        {React.Children.only(children) as React.ComponentProps<typeof Slot>["children"]}
      </Slot>
    );
  }

  return (
    <button ref={forwardedRef as React.Ref<HTMLButtonElement>} {...sharedProps} disabled={disabled} type={type ?? "button"}>
      {children}
    </button>
  );
});
// #endregion 🖱️Trigger

// #region 📂Content
/** 📂 Keeps disclosure content mounted while hiding it from layout and accessibility when closed. */
const CollapsibleContent = React.forwardRef<HTMLDivElement, CollapsibleContentProps>(function CollapsibleContent(props, forwardedRef) {
  const context = useCollapsibleContext();
  return <div ref={forwardedRef} {...props} data-slot="collapsible-content" data-state={context.open ? "open" : "closed"} hidden={!context.open} id={context.contentId} />;
});
// #endregion 📂Content

export { Collapsible, CollapsibleContent, CollapsibleTrigger };
// #endregion ↕️Collapsible
