// #region 🧲️Header
// 💻️ framework/ui/elements/💬️Dialog/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { createPortal } from "react-dom";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️component.ts";
import { Slot } from "../../🔨️modules/🏷️class-name-composition/🟦️slot.tsx";
import { veilClass, glassClass } from "../../🔨️modules/🌈️surface-presentation/🟦️component.ts";
import { useFlow } from "../../🔨️modules/🧭️flow-direction-context/🟦️component.tsx";
import { useLabel } from "../🏷️Label/🟦️component.tsx";
import { SurfaceScope } from "../🌈️Surface/🟦️component.tsx";
import { CloseIconAlt } from "../🔣️Icons/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🧸️Dialog
// #region 📐️Contract
/** 🛑️ Owned preventable event used by dialog focus and dismissal policies. */
export interface DialogPreventableEvent<TEvent extends Event = Event> {
  readonly originalEvent?: TEvent;
  readonly target: EventTarget | null;
  readonly defaultPrevented: boolean;
  preventDefault(): void;
}

export interface DialogProps {
  children?: React.ReactNode;
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?: (open: boolean) => void;
}

export interface DialogTriggerProps extends Omit<React.ButtonHTMLAttributes<HTMLButtonElement>, "children"> {
  asChild?: boolean;
  children?: React.ReactNode;
}

export interface DialogPortalProps {
  children?: React.ReactNode;
  container?: Element | DocumentFragment | null;
}

export interface DialogCloseProps extends Omit<React.ButtonHTMLAttributes<HTMLButtonElement>, "children"> {
  asChild?: boolean;
  children?: React.ReactNode;
}

export type DialogOverlayProps = React.HTMLAttributes<HTMLDivElement>;

export interface DialogContentProps extends Omit<React.HTMLAttributes<HTMLDivElement>, "id" | "onEscape"> {
  showCloseButton?: boolean;
  onOpenAutoFocus?: (event: DialogPreventableEvent) => void;
  onCloseAutoFocus?: (event: DialogPreventableEvent) => void;
  onEscapeKeyDown?: (event: DialogPreventableEvent<KeyboardEvent>) => void;
  onPointerDownOutside?: (event: DialogPreventableEvent<PointerEvent>) => void;
  onFocusOutside?: (event: DialogPreventableEvent<FocusEvent>) => void;
  onInteractOutside?: (event: DialogPreventableEvent<PointerEvent | FocusEvent>) => void;
}

export type DialogTitleProps = Omit<React.HTMLAttributes<HTMLHeadingElement>, "id">;
export type DialogDescriptionProps = Omit<React.HTMLAttributes<HTMLParagraphElement>, "id">;

interface DialogContextValue {
  boundary: string;
  token: string;
  contentId: string;
  titleId: string;
  descriptionId: string;
  open: boolean;
  setOpen: (open: boolean) => void;
  triggerRef: React.MutableRefObject<HTMLElement | null>;
  portalRef: React.MutableRefObject<HTMLDivElement | null>;
}

interface IsolationSnapshot {
  readonly ariaHidden: string | null;
  readonly inert: string | null;
}

const DialogContext = React.createContext<DialogContextValue | null>(null);
const DialogPortalContext = React.createContext(false);
const dialogStack: string[] = [];
const dialogPortals = new Map<string, HTMLElement>();
const isolationSnapshots = new Map<HTMLElement, IsolationSnapshot>();
let bodyOverflow = "";
let bodyPaddingRight = "";

const useIsomorphicLayoutEffect = typeof window === "undefined" ? React.useEffect : React.useLayoutEffect;

/** 🧭️ Resolves the nearest owned dialog state. */
function useDialogContext(): DialogContextValue {
  const context = React.useContext(DialogContext);
  if (!context) throw new Error("Dialog parts must render inside Dialog.");
  return context;
}

/** 🛑️ Creates a small owned preventable event without exposing a third-party event type. */
function preventableEvent<TEvent extends Event>(originalEvent?: TEvent): DialogPreventableEvent<TEvent> {
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

function focusableElements(content: HTMLElement): HTMLElement[] {
  return Array.from(content.querySelectorAll<HTMLElement>('button:not(:disabled), [href], input:not(:disabled), select:not(:disabled), textarea:not(:disabled), [tabindex]:not([tabindex="-1"])')).filter(
    (element) => !element.hidden && element.getAttribute("aria-hidden") !== "true",
  );
}

function restoreIsolation(): void {
  for (const [element, snapshot] of isolationSnapshots) {
    if (snapshot.ariaHidden === null) element.removeAttribute("aria-hidden");
    else element.setAttribute("aria-hidden", snapshot.ariaHidden);
    if (snapshot.inert === null) element.removeAttribute("inert");
    else element.setAttribute("inert", snapshot.inert);
  }
  isolationSnapshots.clear();
}

function topDialogToken(): string | undefined {
  let selected: string | undefined;
  let selectedDepth = -1;
  for (const token of dialogStack) {
    const depth = dialogPortals.get(token)?.dataset.dialogBoundary?.split(" ").length ?? 0;
    if (depth >= selectedDepth) {
      selected = token;
      selectedDepth = depth;
    }
  }
  return selected;
}

/** 🌑️ Isolates the active modal and owns nested scroll locking without erasing prior attributes. */
function syncModalEnvironment(): void {
  if (typeof document === "undefined") return;
  restoreIsolation();
  const topToken = topDialogToken();
  if (!topToken) {
    document.body.style.overflow = bodyOverflow;
    document.body.style.paddingRight = bodyPaddingRight;
    return;
  }
  const topPortal = dialogPortals.get(topToken);
  if (!topPortal) return;
  let branch: HTMLElement = topPortal;
  while (branch.parentElement) {
    const parent = branch.parentElement;
    for (const sibling of Array.from(parent.children)) {
      if (!(sibling instanceof HTMLElement) || sibling === branch) continue;
      isolationSnapshots.set(sibling, { ariaHidden: sibling.getAttribute("aria-hidden"), inert: sibling.getAttribute("inert") });
      sibling.setAttribute("aria-hidden", "true");
      sibling.setAttribute("inert", "");
    }
    if (parent === document.body) break;
    branch = parent;
  }
  document.body.style.overflow = "hidden";
  const scrollbarWidth = Math.max(0, window.innerWidth - document.documentElement.clientWidth);
  if (scrollbarWidth > 0) {
    const paddingRight = Number.parseFloat(window.getComputedStyle(document.body).paddingRight) || 0;
    document.body.style.paddingRight = `${paddingRight + scrollbarWidth}px`;
  }
}

function registerDialog(token: string, portal: HTMLElement): void {
  const index = dialogStack.indexOf(token);
  if (index >= 0) dialogStack.splice(index, 1);
  if (dialogStack.length === 0) {
    bodyOverflow = document.body.style.overflow;
    bodyPaddingRight = document.body.style.paddingRight;
  }
  dialogStack.push(token);
  dialogPortals.set(token, portal);
  syncModalEnvironment();
}

function activateDialog(token: string): void {
  const index = dialogStack.indexOf(token);
  if (index < 0 || index === dialogStack.length - 1) return;
  dialogStack.splice(index, 1);
  dialogStack.push(token);
  syncModalEnvironment();
}

function unregisterDialog(token: string): void {
  const index = dialogStack.indexOf(token);
  if (index >= 0) dialogStack.splice(index, 1);
  dialogPortals.delete(token);
  syncModalEnvironment();
}

function isTopmostDialog(token: string): boolean {
  return topDialogToken() === token;
}

function isInsideDialogBoundary(target: EventTarget | null, context: DialogContextValue, content: HTMLElement | null): boolean {
  if (!(target instanceof Node)) return false;
  if (content?.contains(target) || context.triggerRef.current?.contains(target)) return true;
  const element = target instanceof Element ? target : target.parentElement;
  const portal = element?.closest<HTMLElement>("[data-dialog-boundary]");
  return portal?.dataset.dialogToken !== context.token && portal?.dataset.dialogBoundary?.split(" ").includes(context.token) === true;
}
// #endregion 📐️Contract

// #region 🎛️Root
/** 🎛️ Owns controlled or uncontrolled open state and a logical nested-dialog boundary. */
function Dialog({ children, open: controlledOpen, defaultOpen = false, onOpenChange }: DialogProps) {
  const parent = React.useContext(DialogContext);
  const [uncontrolledOpen, setUncontrolledOpen] = React.useState(defaultOpen);
  const generatedId = React.useId().replace(/[^A-Za-z0-9_-]/g, "");
  const token = `semio-dialog-${generatedId}`;
  const boundary = parent ? `${parent.boundary} ${token}` : token;
  const open = controlledOpen ?? uncontrolledOpen;
  const triggerRef = React.useRef<HTMLElement | null>(null);
  const portalRef = React.useRef<HTMLDivElement | null>(null);
  const setOpen = React.useCallback(
    (nextOpen: boolean) => {
      if (nextOpen === open) return;
      if (controlledOpen === undefined) setUncontrolledOpen(nextOpen);
      onOpenChange?.(nextOpen);
    },
    [controlledOpen, onOpenChange, open],
  );
  const context = React.useMemo<DialogContextValue>(() => ({ boundary, token, contentId: `${token}-content`, titleId: `${token}-title`, descriptionId: `${token}-description`, open, setOpen, triggerRef, portalRef }), [boundary, open, setOpen, token]);
  return (
    <DialogContext.Provider value={context}>
      <DialogPortalContext.Provider value={false}>{children}</DialogPortalContext.Provider>
    </DialogContext.Provider>
  );
}
// #endregion 🎛️Root

// #region 🖱️Controls
/** 🖱️ Opens its owner while preserving exactly-one-child refs and event precedence. */
const DialogTrigger = React.forwardRef<HTMLElement, DialogTriggerProps>(function DialogTrigger({ asChild = false, children, disabled = false, type, onClick, className, ...props }, forwardedRef) {
  const context = useDialogContext();
  const ref = React.useMemo(() => composedRef(forwardedRef, context.triggerRef), [context.triggerRef, forwardedRef]);
  const sharedProps: React.HTMLAttributes<HTMLElement> & { "data-slot": string; "data-state": "open" | "closed"; disabled?: boolean; type?: "button" | "reset" | "submit" } = {
    ...props,
    "aria-controls": context.contentId,
    "aria-expanded": context.open,
    "aria-haspopup": "dialog",
    "aria-disabled": disabled || undefined,
    className: cn(className),
    "data-slot": "dialog-trigger",
    "data-state": context.open ? "open" : "closed",
    onClick: (event) => {
      (onClick as React.MouseEventHandler<HTMLElement> | undefined)?.(event);
      if (event.defaultPrevented || disabled) return;
      context.setOpen(true);
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

/** ❎️ Closes its owner after the consumer's click handler has accepted the action. */
const DialogClose = React.forwardRef<HTMLElement, DialogCloseProps>(function DialogClose({ asChild = false, children, disabled = false, type, onClick, className, ...props }, forwardedRef) {
  const context = useDialogContext();
  const sharedProps: React.HTMLAttributes<HTMLElement> & { "data-slot": string; "data-state": "open" | "closed"; disabled?: boolean; type?: "button" | "reset" | "submit" } = {
    ...props,
    "aria-disabled": disabled || undefined,
    className: cn(className),
    "data-slot": "dialog-close",
    "data-state": context.open ? "open" : "closed",
    onClick: (event) => {
      (onClick as React.MouseEventHandler<HTMLElement> | undefined)?.(event);
      if (event.defaultPrevented || disabled) return;
      context.setOpen(false);
    },
  };
  if (asChild) {
    return (
      <Slot ref={forwardedRef} {...sharedProps}>
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
// #endregion 🖱️Controls

// #region 🪟️PortalAndContent
/** 🪟️ Portals owned modal parts into an isolated root with automatic cleanup. */
function DialogPortal({ children, container }: DialogPortalProps) {
  const context = useDialogContext();
  if (!context.open || typeof document === "undefined") return null;
  return createPortal(
    <div ref={context.portalRef} data-slot="dialog-portal" data-dialog-boundary={context.boundary} data-dialog-token={context.token}>
      <DialogPortalContext.Provider value>{children}</DialogPortalContext.Provider>
    </div>,
    container ?? document.body,
  );
}

/** 🌑️ Paints the modal veil while dismissal remains owned by DialogContent. */
const DialogOverlay = React.forwardRef<HTMLDivElement, DialogOverlayProps>(function DialogOverlay({ className, ...props }, forwardedRef) {
  const context = useDialogContext();
  if (!context.open) return null;
  return (
    <div
      ref={forwardedRef}
      data-slot="dialog-overlay"
      data-level="dialog"
      data-state="open"
      className={cn(veilClass, "data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-dialog", className)}
      {...props}
    />
  );
});

/** 🪟️ Owns modal isolation, focus trapping, topmost dismissal, and accessible associations. */
const DialogContent = React.forwardRef<HTMLDivElement, DialogContentProps>(function DialogContent(
  { className, showCloseButton = true, children, onOpenAutoFocus, onCloseAutoFocus, onEscapeKeyDown, onPointerDownOutside, onFocusOutside, onInteractOutside, ...props },
  forwardedRef,
) {
  const context = useDialogContext();
  const insidePortal = React.useContext(DialogPortalContext);
  const flow = useFlow();
  const closeLabel = useLabel("ui.common.close");
  const contentRef = React.useRef<HTMLDivElement | null>(null);
  const restoreFocusRef = React.useRef<HTMLElement | null>(null);
  const dismissedRef = React.useRef(false);
  const callbacksRef = React.useRef({ onOpenAutoFocus, onCloseAutoFocus, onEscapeKeyDown, onPointerDownOutside, onFocusOutside, onInteractOutside });
  callbacksRef.current = { onOpenAutoFocus, onCloseAutoFocus, onEscapeKeyDown, onPointerDownOutside, onFocusOutside, onInteractOutside };
  const ref = React.useMemo(() => composedRef(forwardedRef, contentRef), [forwardedRef]);

  useIsomorphicLayoutEffect(() => {
    const content = contentRef.current;
    const portal = content?.closest<HTMLElement>(`[data-dialog-token="${context.token}"]`);
    if (!context.open || !content || !portal) return;
    dismissedRef.current = false;
    restoreFocusRef.current = document.activeElement instanceof HTMLElement ? document.activeElement : null;
    registerDialog(context.token, portal);
    const event = preventableEvent();
    callbacksRef.current.onOpenAutoFocus?.(event);
    if (!event.defaultPrevented) (focusableElements(content)[0] ?? content).focus({ preventScroll: true });
    return () => {
      unregisterDialog(context.token);
      const closeEvent = preventableEvent();
      callbacksRef.current.onCloseAutoFocus?.(closeEvent);
      if (!closeEvent.defaultPrevented) {
        const target = context.triggerRef.current?.isConnected ? context.triggerRef.current : restoreFocusRef.current?.isConnected ? restoreFocusRef.current : null;
        target?.focus({ preventScroll: true });
      }
    };
  }, [context.open, context.portalRef, context.token, context.triggerRef]);

  React.useEffect(() => {
    if (!context.open) return;
    const dismiss = (event: PointerEvent | FocusEvent, kind: "pointer" | "focus") => {
      if (!isTopmostDialog(context.token)) return;
      if (isInsideDialogBoundary(event.target, context, contentRef.current)) {
        activateDialog(context.token);
        return;
      }
      if (dismissedRef.current) return;
      const owned = preventableEvent(event);
      if (kind === "pointer") callbacksRef.current.onPointerDownOutside?.(owned as DialogPreventableEvent<PointerEvent>);
      else callbacksRef.current.onFocusOutside?.(owned as DialogPreventableEvent<FocusEvent>);
      callbacksRef.current.onInteractOutside?.(owned);
      if (owned.defaultPrevented) return;
      if (kind === "focus") {
        (focusableElements(contentRef.current!)[0] ?? contentRef.current)?.focus({ preventScroll: true });
        return;
      }
      dismissedRef.current = true;
      context.setOpen(false);
      queueMicrotask(() => {
        dismissedRef.current = false;
      });
    };
    const handlePointerDown = (event: PointerEvent) => dismiss(event, "pointer");
    const handleFocusIn = (event: FocusEvent) => dismiss(event, "focus");
    const handleKeyDown = (event: KeyboardEvent) => {
      if (!isTopmostDialog(context.token)) return;
      if (event.key === "Escape") {
        const owned = preventableEvent(event);
        callbacksRef.current.onEscapeKeyDown?.(owned);
        if (owned.defaultPrevented) return;
        event.preventDefault();
        event.stopPropagation();
        dismissedRef.current = true;
        context.setOpen(false);
        queueMicrotask(() => {
          dismissedRef.current = false;
        });
        return;
      }
      if (event.key !== "Tab" || !contentRef.current) return;
      const focusables = focusableElements(contentRef.current);
      if (focusables.length === 0) {
        event.preventDefault();
        contentRef.current.focus({ preventScroll: true });
        return;
      }
      const first = focusables[0]!;
      const last = focusables.at(-1)!;
      const active = document.activeElement;
      if (event.shiftKey && (active === first || !contentRef.current.contains(active))) {
        event.preventDefault();
        last.focus({ preventScroll: true });
      } else if (!event.shiftKey && (active === last || !contentRef.current.contains(active))) {
        event.preventDefault();
        first.focus({ preventScroll: true });
      }
    };
    document.addEventListener("pointerdown", handlePointerDown, true);
    document.addEventListener("focusin", handleFocusIn, true);
    document.addEventListener("keydown", handleKeyDown, true);
    return () => {
      document.removeEventListener("pointerdown", handlePointerDown, true);
      document.removeEventListener("focusin", handleFocusIn, true);
      document.removeEventListener("keydown", handleKeyDown, true);
    };
  }, [context]);

  if (!context.open) return null;
  const labelledBy = props["aria-label"] ? props["aria-labelledby"] : (props["aria-labelledby"] ?? context.titleId);
  const describedBy = props["aria-describedby"] ?? context.descriptionId;
  const renderedContent = (
    <div
      {...props}
      ref={ref}
      id={context.contentId}
      role={props.role ?? "dialog"}
      aria-modal={props["aria-modal"] ?? true}
      aria-labelledby={labelledBy}
      aria-describedby={describedBy}
      tabIndex={props.tabIndex ?? -1}
      data-slot="dialog-content"
      data-level="dialog"
      data-state="open"
      dir={flow.inline === "rtl" ? "rtl" : undefined}
      className={cn(
        "data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 fixed top-[50%] left-[50%] z-dialog grid w-full max-w-[calc(100%-2*var(--ui-spacing)*var(--medium))] translate-x-[-50%] translate-y-[-50%] gap-medium border p-medium duration-200 sm:max-w-lg",
        glassClass,
        className,
      )}
      onFocusCapture={(event) => {
        props.onFocusCapture?.(event);
        if (!event.defaultPrevented) activateDialog(context.token);
      }}
      onPointerDownCapture={(event) => {
        props.onPointerDownCapture?.(event);
        if (!event.defaultPrevented) activateDialog(context.token);
      }}
    >
      <SurfaceScope level="dialog" fill="glass">
        {children}
        {showCloseButton && (
          <DialogClose className="ring-offset-background focus:ring-ring data-[state=open]:bg-accent data-[state=open]:text-muted-foreground absolute top-medium right-4 rounded-xs opacity-70 transition-opacity hover:opacity-100 focus:ring-2 focus:ring-offset-2 focus:outline-hidden disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-small">
            <CloseIconAlt />
            <span className="sr-only">{closeLabel}</span>
          </DialogClose>
        )}
      </SurfaceScope>
    </div>
  );
  if (insidePortal) return renderedContent;
  return (
    <DialogPortal>
      <DialogOverlay />
      {renderedContent}
    </DialogPortal>
  );
});
// #endregion 🪟️PortalAndContent

// #region 🏷️LabelsAndLayout
/** 🧱️ Groups dialog heading content without changing semantics. */
function DialogHeader({ className, ...props }: React.ComponentProps<"div">) {
  return <div data-slot="dialog-header" className={cn("flex flex-col gap-single text-center sm:text-start", className)} {...props} />;
}

/** 🧱️ Groups dialog actions without changing semantics. */
function DialogFooter({ className, ...props }: React.ComponentProps<"div">) {
  return <div data-slot="dialog-footer" className={cn("flex flex-col-reverse gap-single sm:flex-row sm:justify-end", className)} {...props} />;
}

/** 🏷️ Supplies the stable accessible name target owned by its dialog. */
const DialogTitle = React.forwardRef<HTMLHeadingElement, DialogTitleProps>(function DialogTitle({ className, ...props }, forwardedRef) {
  const context = useDialogContext();
  return <h2 ref={forwardedRef} id={context.titleId} data-slot="dialog-title" className={cn("text-lg font-semibold leading-none tracking-tight", className)} {...props} />;
});

/** 💬️ Supplies the stable accessible description target owned by its dialog. */
const DialogDescription = React.forwardRef<HTMLParagraphElement, DialogDescriptionProps>(function DialogDescription({ className, ...props }, forwardedRef) {
  const context = useDialogContext();
  return <p ref={forwardedRef} id={context.descriptionId} data-slot="dialog-description" className={cn("text-muted-foreground text-sm", className)} {...props} />;
});
// #endregion 🏷️LabelsAndLayout

export { Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogOverlay, DialogPortal, DialogTitle, DialogTrigger };
// #endregion 🧸️Dialog
