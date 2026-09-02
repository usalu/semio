// #region 🧲️Header
// 💻️ framework/ui/elements/🗨️Popover/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { createPortal } from "react-dom";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { Slot } from "../../🔨️modules/🏷️class-name-composition/🟦️.tsx";
import { glassClass } from "../../🔨️modules/🌈️surface-presentation/🟦️.ts";
import { useFlow } from "../../🔨️modules/🧭️flow-direction-context/🟦️.tsx";
import { SurfaceScope } from "../🌈️Surface/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🌐️Popover
// #region 📐️Contract
export type PopoverSide = "top" | "right" | "bottom" | "left";
export type PopoverAlign = "start" | "center" | "end";

/** 🛑️ Owned preventable event used by popover focus and dismissal policies. */
export interface PopoverPreventableEvent<TEvent extends Event = Event> {
  readonly originalEvent?: TEvent;
  readonly target: EventTarget | null;
  readonly defaultPrevented: boolean;
  preventDefault(): void;
}

export interface PopoverProps {
  children?: React.ReactNode;
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?: (open: boolean) => void;
}

export interface PopoverTriggerProps extends Omit<React.ButtonHTMLAttributes<HTMLButtonElement>, "children"> {
  asChild?: boolean;
  children?: React.ReactNode;
}

export interface PopoverAnchorProps extends React.HTMLAttributes<HTMLElement> {
  asChild?: boolean;
  children?: React.ReactNode;
}

export interface PopoverContentProps extends Omit<React.HTMLAttributes<HTMLDivElement>, "onEscape"> {
  "data-slot"?: string;
  side?: PopoverSide;
  align?: PopoverAlign;
  sideOffset?: number;
  alignOffset?: number;
  avoidCollisions?: boolean;
  collisionPadding?: number;
  onOpenAutoFocus?: (event: PopoverPreventableEvent) => void;
  onCloseAutoFocus?: (event: PopoverPreventableEvent) => void;
  onEscapeKeyDown?: (event: PopoverPreventableEvent<KeyboardEvent>) => void;
  onPointerDownOutside?: (event: PopoverPreventableEvent<PointerEvent>) => void;
  onFocusOutside?: (event: PopoverPreventableEvent<FocusEvent>) => void;
  onInteractOutside?: (event: PopoverPreventableEvent<PointerEvent | FocusEvent>) => void;
}

interface PopoverContextValue {
  boundary: string;
  token: string;
  contentId: string;
  open: boolean;
  setOpen: (open: boolean) => void;
  triggerRef: React.MutableRefObject<HTMLElement | null>;
  anchorRef: React.MutableRefObject<HTMLElement | null>;
}

const PopoverContext = React.createContext<PopoverContextValue | null>(null);
const popoverActivity = new Map<string, number>();
let popoverActivitySequence = 0;

function markPopoverActive(token: string): void {
  popoverActivity.set(token, ++popoverActivitySequence);
}

/** 🧭️ Resolves the nearest owned popover state. */
function usePopoverContext(): PopoverContextValue {
  const context = React.useContext(PopoverContext);
  if (!context) throw new Error("Popover parts must render inside Popover.");
  return context;
}

/** 🛑️ Creates a small owned preventable event without exposing a third-party event type. */
function preventableEvent<TEvent extends Event>(originalEvent?: TEvent): PopoverPreventableEvent<TEvent> {
  let prevented = false;
  return {
    originalEvent,
    target: originalEvent?.target ?? null,
    get defaultPrevented() {
      return prevented;
    },
    preventDefault() {
      prevented = true;
    },
  };
}

function setRef<T>(ref: React.Ref<T> | undefined, value: T | null): void {
  if (typeof ref === "function") ref(value);
  else if (ref) ref.current = value;
}

function composedRef<T>(forwardedRef: React.Ref<T>, ownedRef: React.MutableRefObject<T | null>): React.RefCallback<T> {
  return (value) => {
    ownedRef.current = value;
    setRef(forwardedRef, value);
  };
}
// #endregion 📐️Contract

// #region 🎛️Root
/** 🎛️ Owns controlled or uncontrolled open state and a logical nested-popover boundary. */
function Popover({ children, open: controlledOpen, defaultOpen = false, onOpenChange }: PopoverProps) {
  const parent = React.useContext(PopoverContext);
  const [uncontrolledOpen, setUncontrolledOpen] = React.useState(defaultOpen);
  const generatedId = React.useId().replace(/[^A-Za-z0-9_-]/g, "");
  const token = `semio-popover-${generatedId}`;
  const boundary = parent ? `${parent.boundary} ${token}` : token;
  const open = controlledOpen ?? uncontrolledOpen;
  const triggerRef = React.useRef<HTMLElement | null>(null);
  const anchorRef = React.useRef<HTMLElement | null>(null);
  const setOpen = React.useCallback(
    (nextOpen: boolean) => {
      if (nextOpen === open) return;
      if (controlledOpen === undefined) setUncontrolledOpen(nextOpen);
      onOpenChange?.(nextOpen);
    },
    [controlledOpen, onOpenChange, open],
  );
  const context = React.useMemo<PopoverContextValue>(() => ({ boundary, token, contentId: `${token}-content`, open, setOpen, triggerRef, anchorRef }), [boundary, open, setOpen, token]);
  return <PopoverContext.Provider value={context}>{children}</PopoverContext.Provider>;
}
// #endregion 🎛️Root

// #region 🖱️TriggerAndAnchor
/** 🖱️ Toggles its owner while preserving exactly-one-child refs and event precedence. */
const PopoverTrigger = React.forwardRef<HTMLElement, PopoverTriggerProps>(function PopoverTrigger({ asChild = false, children, disabled = false, type, onClick, onFocus, onPointerDown, className, ...props }, forwardedRef) {
  const context = usePopoverContext();
  const ref = React.useMemo(() => composedRef(forwardedRef, context.triggerRef), [context.triggerRef, forwardedRef]);
  const sharedProps: React.HTMLAttributes<HTMLElement> & { "data-slot": string; "data-state": "open" | "closed"; disabled?: boolean; type?: "button" | "reset" | "submit" } = {
    ...props,
    "aria-controls": context.contentId,
    "aria-expanded": context.open,
    "aria-haspopup": "dialog",
    "aria-disabled": disabled || undefined,
    className: cn(className),
    "data-slot": "popover-trigger",
    "data-state": context.open ? "open" : "closed",
    onFocus: (event) => {
      (onFocus as React.FocusEventHandler<HTMLElement> | undefined)?.(event);
      if (!event.defaultPrevented && !disabled) markPopoverActive(context.token);
    },
    onPointerDown: (event) => {
      (onPointerDown as React.PointerEventHandler<HTMLElement> | undefined)?.(event);
      if (!event.defaultPrevented && !disabled) markPopoverActive(context.token);
    },
    onClick: (event) => {
      (onClick as React.MouseEventHandler<HTMLElement> | undefined)?.(event);
      if (event.defaultPrevented || disabled) return;
      markPopoverActive(context.token);
      context.setOpen(!context.open);
    },
  };

  if (asChild) {
    return (
      <Slot ref={ref} {...sharedProps}>
        {React.Children.only(children) as React.ComponentProps<typeof Slot>["children"]}
      </Slot>
    );
  }

  return (
    <button ref={ref as React.Ref<HTMLButtonElement>} {...sharedProps} disabled={disabled} type={type ?? "button"}>
      {children}
    </button>
  );
});

/** ⚓️ Registers a non-trigger positioning anchor without adding interactive semantics. */
const PopoverAnchor = React.forwardRef<HTMLElement, PopoverAnchorProps>(function PopoverAnchor({ asChild = false, children, ...props }, forwardedRef) {
  const context = usePopoverContext();
  const ref = React.useMemo(() => composedRef(forwardedRef, context.anchorRef), [context.anchorRef, forwardedRef]);
  if (asChild) {
    return (
      <Slot ref={ref} {...props} data-slot="popover-anchor">
        {React.Children.only(children) as React.ComponentProps<typeof Slot>["children"]}
      </Slot>
    );
  }
  return (
    <span ref={ref} {...props} data-slot="popover-anchor">
      {children}
    </span>
  );
});
// #endregion 🖱️TriggerAndAnchor

// #region 📍️Placement
interface PopoverPlacement {
  side: PopoverSide;
  left: number;
  top: number;
  transformOrigin: string;
}

/** 📍️ Resolves measured fixed placement, flipping the main axis and clamping to the viewport. */
export function resolvePopoverPlacement(
  anchor: Pick<DOMRect, "top" | "right" | "bottom" | "left" | "width" | "height">,
  content: Pick<DOMRect, "width" | "height">,
  viewport: { width: number; height: number },
  side: PopoverSide,
  align: PopoverAlign,
  sideOffset: number,
  alignOffset: number,
  collisionPadding: number,
  rtl: boolean,
  avoidCollisions: boolean,
): PopoverPlacement {
  const alignedLeft = align === "center" ? anchor.left + (anchor.width - content.width) / 2 + alignOffset : align === (rtl ? "end" : "start") ? anchor.left + alignOffset : anchor.right - content.width + alignOffset;
  const alignedTop = align === "center" ? anchor.top + (anchor.height - content.height) / 2 + alignOffset : align === "start" ? anchor.top + alignOffset : anchor.bottom - content.height + alignOffset;
  const position = (candidate: PopoverSide): { left: number; top: number } => {
    if (candidate === "top") return { left: alignedLeft, top: anchor.top - content.height - sideOffset };
    if (candidate === "bottom") return { left: alignedLeft, top: anchor.bottom + sideOffset };
    if (candidate === "left") return { left: anchor.left - content.width - sideOffset, top: alignedTop };
    return { left: anchor.right + sideOffset, top: alignedTop };
  };
  const opposite: Record<PopoverSide, PopoverSide> = { top: "bottom", right: "left", bottom: "top", left: "right" };
  const overflowsMainAxis = (candidate: PopoverSide, point: { left: number; top: number }): boolean => {
    if (candidate === "top") return point.top < collisionPadding;
    if (candidate === "bottom") return point.top + content.height > viewport.height - collisionPadding;
    if (candidate === "left") return point.left < collisionPadding;
    return point.left + content.width > viewport.width - collisionPadding;
  };
  let resolvedSide = side;
  let point = position(side);
  if (avoidCollisions && overflowsMainAxis(side, point)) {
    const flipped = opposite[side];
    const flippedPoint = position(flipped);
    if (!overflowsMainAxis(flipped, flippedPoint)) {
      resolvedSide = flipped;
      point = flippedPoint;
    }
  }
  if (avoidCollisions) {
    point.left = Math.min(Math.max(point.left, collisionPadding), Math.max(collisionPadding, viewport.width - collisionPadding - content.width));
    point.top = Math.min(Math.max(point.top, collisionPadding), Math.max(collisionPadding, viewport.height - collisionPadding - content.height));
  }
  const transformOrigin = resolvedSide === "top" ? "center bottom" : resolvedSide === "bottom" ? "center top" : resolvedSide === "left" ? "right center" : "left center";
  return { side: resolvedSide, left: point.left, top: point.top, transformOrigin };
}

function isInsidePopoverBoundary(target: EventTarget | null, context: PopoverContextValue, content: HTMLElement | null): boolean {
  if (!(target instanceof Node)) return false;
  if (content?.contains(target) || context.triggerRef.current?.contains(target) || context.anchorRef.current?.contains(target)) return true;
  const element = target instanceof Element ? target : target.parentElement;
  return element?.closest(`[data-popover-boundary~="${context.boundary.split(" ").at(-1)}"]`) != null;
}

function firstFocusable(content: HTMLElement): HTMLElement | null {
  return content.querySelector<HTMLElement>('button:not(:disabled), [href], input:not(:disabled), select:not(:disabled), textarea:not(:disabled), [tabindex]:not([tabindex="-1"])');
}

function isTopmostPopover(content: HTMLElement | null): boolean {
  if (!content) return false;
  const openContents = Array.from(document.querySelectorAll<HTMLElement>("[data-popover-boundary]"));
  const deepest = Math.max(0, ...openContents.map((element) => element.dataset.popoverBoundary?.split(" ").length ?? 0));
  const candidates = openContents.filter((element) => element.dataset.popoverBoundary?.split(" ").length === deepest);
  const active = candidates.reduce<HTMLElement | null>((selected, candidate) => {
    if (!selected) return candidate;
    const selectedToken = selected.dataset.popoverBoundary?.split(" ").at(-1) ?? "";
    const candidateToken = candidate.dataset.popoverBoundary?.split(" ").at(-1) ?? "";
    return (popoverActivity.get(candidateToken) ?? 0) > (popoverActivity.get(selectedToken) ?? 0) ? candidate : selected;
  }, null);
  return active === content;
}
// #endregion 📍️Placement

// #region 🪟️Content
/** 🪟️ Portals a measured nonmodal dialog with owned dismissal and focus lifecycle. */
const PopoverContent = React.forwardRef<HTMLDivElement, PopoverContentProps>(function PopoverContent(
  {
    className,
    align = "center",
    side = "bottom",
    sideOffset = 4,
    alignOffset = 0,
    avoidCollisions = true,
    collisionPadding = 8,
    children,
    style,
    onOpenAutoFocus,
    onCloseAutoFocus,
    onEscapeKeyDown,
    onPointerDownOutside,
    onFocusOutside,
    onInteractOutside,
    ...props
  },
  forwardedRef,
) {
  const context = usePopoverContext();
  const flow = useFlow();
  const contentRef = React.useRef<HTMLDivElement | null>(null);
  const [placement, setPlacement] = React.useState<PopoverPlacement | null>(null);
  const dismissedRef = React.useRef(false);
  const ref = React.useMemo(() => composedRef(forwardedRef, contentRef), [forwardedRef]);

  React.useLayoutEffect(() => {
    const content = contentRef.current;
    const anchor = context.anchorRef.current ?? context.triggerRef.current;
    if (!context.open || !content || !anchor) return;
    const measure = () => {
      const anchorRect = anchor.getBoundingClientRect();
      const contentRect = content.getBoundingClientRect();
      setPlacement(resolvePopoverPlacement(anchorRect, contentRect, { width: window.innerWidth, height: window.innerHeight }, side, align, sideOffset, alignOffset, Math.max(0, collisionPadding), flow.inline === "rtl", avoidCollisions));
    };
    measure();
    const observer = typeof ResizeObserver === "undefined" ? null : new ResizeObserver(measure);
    observer?.observe(anchor);
    observer?.observe(content);
    window.addEventListener("resize", measure);
    window.addEventListener("scroll", measure, true);
    return () => {
      observer?.disconnect();
      window.removeEventListener("resize", measure);
      window.removeEventListener("scroll", measure, true);
    };
  }, [align, alignOffset, avoidCollisions, collisionPadding, context.anchorRef, context.open, context.triggerRef, flow.inline, side, sideOffset]);

  React.useLayoutEffect(() => {
    if (!context.open || !contentRef.current) return;
    dismissedRef.current = false;
    markPopoverActive(context.token);
    const event = preventableEvent();
    onOpenAutoFocus?.(event);
    if (!event.defaultPrevented) (firstFocusable(contentRef.current) ?? contentRef.current).focus({ preventScroll: true });
    return () => {
      const closeEvent = preventableEvent();
      onCloseAutoFocus?.(closeEvent);
      if (!closeEvent.defaultPrevented) context.triggerRef.current?.focus({ preventScroll: true });
    };
  }, [context.open, context.token, context.triggerRef, onCloseAutoFocus, onOpenAutoFocus]);

  React.useEffect(() => {
    if (!context.open) return;
    return () => {
      popoverActivity.delete(context.token);
    };
  }, [context.open, context.token]);

  React.useEffect(() => {
    if (!context.open) return;
    const dismiss = (event: PointerEvent | FocusEvent, kind: "pointer" | "focus") => {
      if (isInsidePopoverBoundary(event.target, context, contentRef.current)) {
        markPopoverActive(context.token);
        return;
      }
      if (dismissedRef.current) return;
      const owned = preventableEvent(event);
      if (kind === "pointer") onPointerDownOutside?.(owned as PopoverPreventableEvent<PointerEvent>);
      else onFocusOutside?.(owned as PopoverPreventableEvent<FocusEvent>);
      onInteractOutside?.(owned);
      if (owned.defaultPrevented) return;
      dismissedRef.current = true;
      context.setOpen(false);
    };
    const handlePointerDown = (event: PointerEvent) => dismiss(event, "pointer");
    const handleFocusIn = (event: FocusEvent) => dismiss(event, "focus");
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key !== "Escape" || !isTopmostPopover(contentRef.current)) return;
      const owned = preventableEvent(event);
      onEscapeKeyDown?.(owned);
      if (owned.defaultPrevented) return;
      event.preventDefault();
      event.stopPropagation();
      dismissedRef.current = true;
      context.setOpen(false);
    };
    document.addEventListener("pointerdown", handlePointerDown, true);
    document.addEventListener("focusin", handleFocusIn, true);
    document.addEventListener("keydown", handleKeyDown, true);
    return () => {
      document.removeEventListener("pointerdown", handlePointerDown, true);
      document.removeEventListener("focusin", handleFocusIn, true);
      document.removeEventListener("keydown", handleKeyDown, true);
    };
  }, [context, onEscapeKeyDown, onFocusOutside, onInteractOutside, onPointerDownOutside]);

  if (!context.open || typeof document === "undefined") return null;
  const resolvedSide = placement?.side ?? side;
  return createPortal(
    <div
      {...props}
      ref={ref}
      id={context.contentId}
      role={props.role ?? "dialog"}
      tabIndex={props.tabIndex ?? -1}
      dir={flow.inline === "rtl" ? "rtl" : undefined}
      data-slot={props["data-slot"] ?? "popover-content"}
      data-level="menu"
      data-state="open"
      data-side={resolvedSide}
      data-align={align}
      data-popover-boundary={context.boundary}
      className={cn(
        "text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-menu w-72 origin-(--radix-popover-content-transform-origin) border p-1 outline-hidden",
        glassClass,
        className,
      )}
      style={
        {
          position: "fixed",
          left: placement?.left ?? 0,
          top: placement?.top ?? 0,
          visibility: placement ? undefined : "hidden",
          "--radix-popover-content-transform-origin": placement?.transformOrigin ?? "center center",
          ...style,
        } as React.CSSProperties
      }
    >
      <SurfaceScope level="menu" fill="glass">
        {children}
      </SurfaceScope>
    </div>,
    document.body,
  );
});
// #endregion 🪟️Content

export { Popover, PopoverAnchor, PopoverContent, PopoverTrigger };
// #endregion 🌐️Popover
