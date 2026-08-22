// #region 🧲️Header
// 💻️ framework/ui/elements/☑️Select/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { createPortal } from "react-dom";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️component.ts";
import { borderElementClass } from "../../🔨️modules/📏️border-presentation/🟦️component.ts";
import { formControlFocusBorderClass } from "../../🔨️modules/📝️form-control-presentation/🟦️component.ts";
import { interactiveHoverClass } from "../../🔨️modules/🖱️interaction-presentation/🟦️component.ts";
import { menuListItemClassName } from "../../🔨️modules/📋️menu-item-presentation/🟦️component.ts";
import { type ElementProps } from "../../🔨️modules/🆔️element-identity/🟦️component.ts";
import { useFlow } from "../../🔨️modules/🧭️flow-direction-context/🟦️component.tsx";
import { glassClass } from "../../🔨️modules/🌈️surface-presentation/🟦️component.ts";
import { Label } from "../🏷️Label/🟦️component.tsx";
import { SurfaceScope, useLevel } from "../🌈️Surface/🟦️component.tsx";
import { CheckIconAlt, ChevronDownIconAlt, ChevronUpIcon, type IconSource, Icon } from "../🔣️Icons/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🔎️Select
// #region 📐️Contract
export type SelectPosition = "item-aligned" | "popper";
export type SelectSide = "top" | "bottom";
export type SelectAlign = "start" | "center" | "end";

/** 🛑️ Owned preventable event used by select focus and dismissal policies. */
export interface SelectPreventableEvent<TEvent extends Event = Event> {
  readonly originalEvent?: TEvent;
  readonly target: EventTarget | null;
  readonly defaultPrevented: boolean;
  preventDefault(): void;
}

export interface SelectProps extends ElementProps {
  children?: React.ReactNode;
  value?: string;
  defaultValue?: string;
  open?: boolean;
  defaultOpen?: boolean;
  onValueChange?: (value: string) => void;
  onOpenChange?: (open: boolean) => void;
  disabled?: boolean;
  showLabel?: boolean;
  dir?: "ltr" | "rtl";
}

export interface SelectTriggerProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  size?: "sm" | "default";
}

export interface SelectValueProps extends React.HTMLAttributes<HTMLSpanElement> {
  placeholder?: React.ReactNode;
}

export interface SelectContentProps extends Omit<React.HTMLAttributes<HTMLDivElement>, "id" | "onEscape"> {
  position?: SelectPosition;
  side?: SelectSide;
  align?: SelectAlign;
  sideOffset?: number;
  collisionPadding?: number;
  container?: Element | DocumentFragment | null;
  onOpenAutoFocus?: (event: SelectPreventableEvent) => void;
  onCloseAutoFocus?: (event: SelectPreventableEvent) => void;
  onEscapeKeyDown?: (event: SelectPreventableEvent<KeyboardEvent>) => void;
  onPointerDownOutside?: (event: SelectPreventableEvent<PointerEvent>) => void;
  onFocusOutside?: (event: SelectPreventableEvent<FocusEvent>) => void;
  onInteractOutside?: (event: SelectPreventableEvent<PointerEvent | FocusEvent>) => void;
}

export type SelectGroupProps = React.HTMLAttributes<HTMLDivElement>;
export type SelectLabelProps = React.HTMLAttributes<HTMLDivElement>;
export type SelectSeparatorProps = React.HTMLAttributes<HTMLDivElement>;
export type SelectScrollButtonProps = React.HTMLAttributes<HTMLDivElement>;

export interface SelectItemProps extends Omit<React.HTMLAttributes<HTMLDivElement>, "id"> {
  id?: string;
  value: string;
  disabled?: boolean;
  textValue?: string;
  icon?: IconSource;
}

interface SelectOption {
  value: string;
  content: React.ReactNode;
}

type SelectOpenIntent = "selected" | "first" | "last" | { typeahead: string };

interface SelectContextValue {
  boundary: string;
  token: string;
  triggerId: string;
  contentId: string;
  labelId?: string;
  disabled: boolean;
  open: boolean;
  value?: string;
  selectedContent?: React.ReactNode;
  activeId?: string;
  direction: "ltr" | "rtl";
  triggerRef: React.MutableRefObject<HTMLButtonElement | null>;
  contentRef: React.MutableRefObject<HTMLDivElement | null>;
  viewportRef: React.MutableRefObject<HTMLDivElement | null>;
  openIntentRef: React.MutableRefObject<SelectOpenIntent>;
  restoreFocusRef: React.MutableRefObject<boolean>;
  setActiveId: (id: string | undefined) => void;
  setOpen: (open: boolean, restoreFocus?: boolean) => void;
  selectValue: (value: string) => void;
}

interface SelectPlacement {
  side: SelectSide;
  left: number;
  top: number;
  availableHeight: number;
  triggerWidth: number;
  triggerHeight: number;
  transformOrigin: string;
}

const SelectContext = React.createContext<SelectContextValue | null>(null);
const SelectGroupContext = React.createContext<string | null>(null);
const selectActivity = new Map<string, number>();
const handledSelectEvents = new WeakSet<Event>();
let selectActivitySequence = 0;
const useIsomorphicLayoutEffect = typeof window === "undefined" ? React.useEffect : React.useLayoutEffect;

/** 🧭️ Resolves the nearest owned select state. */
function useSelectContext(): SelectContextValue {
  const context = React.useContext(SelectContext);
  if (!context) throw new Error("Select parts must render inside Select.");
  return context;
}

/** 🛑️ Creates an owned preventable event without exporting implementation-specific types. */
function preventableEvent<TEvent extends Event>(originalEvent?: TEvent): SelectPreventableEvent<TEvent> {
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

function nodeText(node: React.ReactNode): string {
  if (typeof node === "string" || typeof node === "number") return String(node);
  if (!React.isValidElement(node)) return "";
  return React.Children.toArray((node.props as { children?: React.ReactNode }).children)
    .map(nodeText)
    .join(" ")
    .replace(/\s+/g, " ")
    .trim();
}

function optionsFromChildren(children: React.ReactNode): SelectOption[] {
  const options: SelectOption[] = [];
  const visit = (nodes: React.ReactNode): void => {
    for (const node of React.Children.toArray(nodes)) {
      if (!React.isValidElement(node)) continue;
      const props = node.props as { children?: React.ReactNode; value?: string };
      if (node.type === SelectItem && props.value !== undefined) options.push({ value: props.value, content: props.children });
      else if (node.type !== Select) visit(props.children);
    }
  };
  visit(children);
  return options;
}

function normalizeSelectText(value: string): string {
  return value
    .normalize("NFKD")
    .replace(/[\u0300-\u036f]/g, "")
    .toLowerCase()
    .replace(/\s+/g, " ")
    .trim();
}

function markSelectActive(token: string): void {
  selectActivity.set(token, ++selectActivitySequence);
}

function selectOptionElements(content: HTMLElement): HTMLElement[] {
  return Array.from(content.querySelectorAll<HTMLElement>('[data-slot="select-item"]')).filter((item) => item.dataset.disabled !== "true" && !item.hidden);
}

function activeOption(content: HTMLElement, activeId: string | undefined): HTMLElement | undefined {
  return selectOptionElements(content).find((item) => item.id === activeId);
}

function isInsideSelectBoundary(target: EventTarget | null, context: SelectContextValue): boolean {
  if (!(target instanceof Node)) return false;
  if (context.contentRef.current?.contains(target) || context.triggerRef.current?.contains(target)) return true;
  const element = target instanceof Element ? target : target.parentElement;
  return element?.closest<HTMLElement>("[data-select-boundary]")?.dataset.selectBoundary?.split(" ").includes(context.token) === true;
}

function topSelectToken(): string | undefined {
  if (typeof document === "undefined") return undefined;
  const contents = Array.from(document.querySelectorAll<HTMLElement>("[data-select-boundary]"));
  let selected: HTMLElement | undefined;
  for (const content of contents) {
    if (!selected) {
      selected = content;
      continue;
    }
    const depth = content.dataset.selectBoundary?.split(" ").length ?? 0;
    const selectedDepth = selected.dataset.selectBoundary?.split(" ").length ?? 0;
    const token = content.dataset.selectToken ?? "";
    const selectedToken = selected.dataset.selectToken ?? "";
    if (depth > selectedDepth || (depth === selectedDepth && (selectActivity.get(token) ?? 0) > (selectActivity.get(selectedToken) ?? 0))) selected = content;
  }
  return selected?.dataset.selectToken;
}

/** 📍️ Resolves viewport-safe trigger-relative placement with vertical flipping and inline clamping. */
export function resolveSelectPlacement(
  trigger: Pick<DOMRect, "top" | "right" | "bottom" | "left" | "width" | "height">,
  content: Pick<DOMRect, "width" | "height">,
  viewport: { width: number; height: number },
  side: SelectSide,
  align: SelectAlign,
  sideOffset: number,
  collisionPadding: number,
  rtl: boolean,
): SelectPlacement {
  const below = Math.max(0, viewport.height - collisionPadding - trigger.bottom - sideOffset);
  const above = Math.max(0, trigger.top - collisionPadding - sideOffset);
  const resolvedSide = side === "bottom" && content.height > below && above > below ? "top" : side === "top" && content.height > above && below > above ? "bottom" : side;
  const availableHeight = resolvedSide === "bottom" ? below : above;
  const inlineStart = rtl ? trigger.right - content.width : trigger.left;
  const inlineEnd = rtl ? trigger.left : trigger.right - content.width;
  const alignedLeft = align === "center" ? trigger.left + (trigger.width - content.width) / 2 : align === "start" ? inlineStart : inlineEnd;
  const left = Math.min(Math.max(alignedLeft, collisionPadding), Math.max(collisionPadding, viewport.width - collisionPadding - content.width));
  const top = resolvedSide === "bottom" ? trigger.bottom + sideOffset : trigger.top - Math.min(content.height, availableHeight) - sideOffset;
  return { side: resolvedSide, left, top: Math.max(collisionPadding, top), availableHeight, triggerWidth: trigger.width, triggerHeight: trigger.height, transformOrigin: resolvedSide === "bottom" ? "center top" : "center bottom" };
}
// #endregion 📐️Contract

// #region 🎛️Root
/** 🎛️ Owns controlled or uncontrolled value/open state and the logical nested-select boundary. */
function Select({ id, showLabel = false, children, value: controlledValue, defaultValue, open: controlledOpen, defaultOpen = false, onValueChange, onOpenChange, disabled = false, dir }: SelectProps) {
  const parent = React.useContext(SelectContext);
  const flow = useFlow();
  const generatedId = React.useId().replace(/[^A-Za-z0-9_-]/g, "");
  const token = `semio-select-${generatedId}`;
  const boundary = parent ? `${parent.boundary} ${token}` : token;
  const options = React.useMemo(() => optionsFromChildren(children), [children]);
  const [uncontrolledValue, setUncontrolledValue] = React.useState(defaultValue ?? options[0]?.value);
  const [uncontrolledOpen, setUncontrolledOpen] = React.useState(defaultOpen);
  const [activeId, setActiveId] = React.useState<string>();
  const value = controlledValue ?? uncontrolledValue;
  const open = controlledOpen ?? uncontrolledOpen;
  const selectedContent = options.find((option) => option.value === value)?.content;
  const triggerRef = React.useRef<HTMLButtonElement | null>(null);
  const contentRef = React.useRef<HTMLDivElement | null>(null);
  const viewportRef = React.useRef<HTMLDivElement | null>(null);
  const openIntentRef = React.useRef<SelectOpenIntent>("selected");
  const restoreFocusRef = React.useRef(true);
  const setOpen = React.useCallback(
    (nextOpen: boolean, restoreFocus = true) => {
      restoreFocusRef.current = restoreFocus;
      if (nextOpen === open) return;
      if (controlledOpen === undefined) setUncontrolledOpen(nextOpen);
      onOpenChange?.(nextOpen);
    },
    [controlledOpen, onOpenChange, open],
  );
  const selectValue = React.useCallback(
    (nextValue: string) => {
      if (nextValue !== value) {
        if (controlledValue === undefined) setUncontrolledValue(nextValue);
        onValueChange?.(nextValue);
      }
      setOpen(false, true);
    },
    [controlledValue, onValueChange, setOpen, value],
  );
  React.useEffect(() => {
    if (!open) setActiveId(undefined);
  }, [open]);
  const context = React.useMemo<SelectContextValue>(
    () => ({
      boundary,
      token,
      triggerId: id ? `${id}-trigger` : `${token}-trigger`,
      contentId: `${token}-content`,
      labelId: showLabel && id ? `${id}-label` : undefined,
      disabled,
      open,
      value,
      selectedContent,
      activeId,
      direction: dir ?? (flow.inline === "rtl" ? "rtl" : "ltr"),
      triggerRef,
      contentRef,
      viewportRef,
      openIntentRef,
      restoreFocusRef,
      setActiveId,
      setOpen,
      selectValue,
    }),
    [activeId, boundary, dir, disabled, flow.inline, id, open, selectedContent, selectValue, setOpen, showLabel, token, value],
  );
  const selectElement = <SelectContext.Provider value={context}>{children}</SelectContext.Provider>;
  return showLabel && id ? (
    <Label id={id} labelElementId={`${id}-label`}>
      {selectElement}
    </Label>
  ) : (
    selectElement
  );
}
// #endregion 🎛️Root

// #region 🖱️TriggerAndValue
/** 🖱️ Opens the owned listbox and exposes its value, active row, and label associations. */
const SelectTrigger = React.forwardRef<HTMLButtonElement, SelectTriggerProps>(function SelectTrigger({ className, size = "default", children, id, disabled, type, onClick, onPointerDown, onFocus, onKeyDown, ...props }, forwardedRef) {
  const context = useSelectContext();
  const level = useLevel();
  const resolvedDisabled = context.disabled || disabled === true;
  const ref = React.useMemo(() => composedRef(forwardedRef, context.triggerRef), [context.triggerRef, forwardedRef]);
  const openWith = (intent: SelectOpenIntent): void => {
    context.openIntentRef.current = intent;
    markSelectActive(context.token);
    context.setOpen(true);
  };
  return (
    <button
      {...props}
      ref={ref}
      id={id ?? context.triggerId}
      type={type ?? "button"}
      role="combobox"
      aria-haspopup="listbox"
      aria-expanded={context.open}
      aria-controls={context.contentId}
      aria-activedescendant={context.open ? context.activeId : undefined}
      aria-labelledby={props["aria-label"] ? props["aria-labelledby"] : (props["aria-labelledby"] ?? context.labelId)}
      disabled={resolvedDisabled}
      dir={context.direction}
      data-slot="select-trigger"
      data-detail-panel-control="fill"
      data-size={size}
      data-level={level}
      data-state={context.open ? "open" : "closed"}
      data-placeholder={context.selectedContent === undefined ? "" : undefined}
      className={cn(
        `text-element data-[placeholder]:text-muted-foreground [&_svg:not([class*='text-'])]:text-muted-foreground flex w-fit min-w-0 items-center justify-between gap-single border bg-transparent px-tiny py-single text-sm whitespace-nowrap ${borderElementClass} ${formControlFocusBorderClass} disabled:cursor-not-allowed disabled:opacity-50 h-medium *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex *:data-[slot=select-value]:min-w-0 *:data-[slot=select-value]:flex-1 *:data-[slot=select-value]:items-center *:data-[slot=select-value]:gap-single *:data-[slot=select-value]:overflow-hidden [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-tiny cursor-foldable`,
        interactiveHoverClass,
        className,
      )}
      onFocus={(event) => {
        onFocus?.(event);
        if (!event.defaultPrevented && !resolvedDisabled) markSelectActive(context.token);
      }}
      onPointerDown={(event) => {
        onPointerDown?.(event);
        if (!event.defaultPrevented && !resolvedDisabled) markSelectActive(context.token);
      }}
      onClick={(event) => {
        onClick?.(event);
        if (event.defaultPrevented || resolvedDisabled) return;
        context.openIntentRef.current = "selected";
        context.setOpen(!context.open);
      }}
      onKeyDown={(event) => {
        onKeyDown?.(event);
        if (event.defaultPrevented || resolvedDisabled || event.nativeEvent.isComposing) return;
        if (event.key === "ArrowDown" || event.key === "Enter" || event.key === " ") {
          event.preventDefault();
          openWith("selected");
        } else if (event.key === "ArrowUp") {
          event.preventDefault();
          openWith("last");
        } else if (event.key.length === 1 && !event.altKey && !event.ctrlKey && !event.metaKey) {
          event.preventDefault();
          openWith({ typeahead: event.key });
        }
      }}
    >
      {children}
      <span data-slot="select-chevron" aria-hidden className="pointer-events-none inline-flex size-small shrink-0 items-center justify-center text-muted-foreground">
        <Icon icon="chevron-down" size="small" />
      </span>
    </button>
  );
});

/** 🔤️ Projects the selected option's authored label or the caller's placeholder. */
const SelectValue = React.forwardRef<HTMLSpanElement, SelectValueProps>(function SelectValue({ placeholder, children, ...props }, forwardedRef) {
  const context = useSelectContext();
  return (
    <span {...props} ref={forwardedRef} data-slot="select-value" data-placeholder={context.selectedContent === undefined ? "" : undefined}>
      {children ?? context.selectedContent ?? placeholder}
    </span>
  );
});
// #endregion 🖱️TriggerAndValue

// #region 📍️Content
/** 🧭️ Moves active-descendant state through the current enabled rendered option set. */
function moveActive(context: SelectContextValue, direction: "first" | "last" | "next" | "previous" | "page-next" | "page-previous"): void {
  const content = context.contentRef.current;
  if (!content) return;
  const options = selectOptionElements(content);
  if (options.length === 0) return;
  const activeIndex = options.findIndex((option) => option.id === context.activeId);
  const nextIndex =
    direction === "first"
      ? 0
      : direction === "last"
        ? options.length - 1
        : direction === "next"
          ? (activeIndex + 1 + options.length) % options.length
          : direction === "previous"
            ? (activeIndex - 1 + options.length) % options.length
            : direction === "page-next"
              ? Math.min(options.length - 1, Math.max(0, activeIndex) + 10)
              : Math.max(0, (activeIndex < 0 ? options.length : activeIndex) - 10);
  const next = options[nextIndex];
  context.setActiveId(next?.id);
  next?.scrollIntoView?.({ block: "nearest" });
}

function findTypeaheadOption(context: SelectContextValue, query: string): HTMLElement | undefined {
  const content = context.contentRef.current;
  if (!content) return undefined;
  const options = selectOptionElements(content);
  const activeIndex = options.findIndex((option) => option.id === context.activeId);
  const normalized = normalizeSelectText(query);
  for (let offset = 1; offset <= options.length; offset += 1) {
    const option = options[(Math.max(activeIndex, -1) + offset) % options.length];
    if (option && normalizeSelectText(option.dataset.textValue ?? option.textContent ?? "").startsWith(normalized)) return option;
  }
  return undefined;
}

/** 🪟️ Portals a viewport-bounded owned listbox with focus, dismissal, and navigation policy. */
const SelectContent = React.forwardRef<HTMLDivElement, SelectContentProps>(function SelectContent(
  {
    className,
    children,
    position = "popper",
    side = "bottom",
    align = "start",
    sideOffset = 4,
    collisionPadding = 8,
    container,
    style,
    onOpenAutoFocus,
    onCloseAutoFocus,
    onEscapeKeyDown,
    onPointerDownOutside,
    onFocusOutside,
    onInteractOutside,
    onKeyDown,
    onPointerDown,
    ...props
  },
  forwardedRef,
) {
  const context = useSelectContext();
  const [placement, setPlacement] = React.useState<SelectPlacement>();
  const typeaheadRef = React.useRef("");
  const typeaheadTimerRef = React.useRef<number | undefined>(undefined);
  const openAutoFocusRef = React.useRef(onOpenAutoFocus);
  const closeAutoFocusRef = React.useRef(onCloseAutoFocus);
  openAutoFocusRef.current = onOpenAutoFocus;
  closeAutoFocusRef.current = onCloseAutoFocus;
  const ref = React.useMemo(() => composedRef(forwardedRef, context.contentRef), [context.contentRef, forwardedRef]);

  useIsomorphicLayoutEffect(() => {
    const content = context.contentRef.current;
    const trigger = context.triggerRef.current;
    if (!context.open || !content || !trigger) return;
    const measure = () =>
      setPlacement(
        resolveSelectPlacement(trigger.getBoundingClientRect(), content.getBoundingClientRect(), { width: window.innerWidth, height: window.innerHeight }, side, align, sideOffset, Math.max(0, collisionPadding), context.direction === "rtl"),
      );
    measure();
    const observer = typeof ResizeObserver === "undefined" ? null : new ResizeObserver(measure);
    observer?.observe(trigger);
    observer?.observe(content);
    window.addEventListener("resize", measure);
    window.addEventListener("scroll", measure, true);
    return () => {
      observer?.disconnect();
      window.removeEventListener("resize", measure);
      window.removeEventListener("scroll", measure, true);
    };
  }, [align, collisionPadding, context.contentRef, context.direction, context.open, context.triggerRef, side, sideOffset]);

  useIsomorphicLayoutEffect(() => {
    const content = context.contentRef.current;
    if (!context.open || !content) return;
    markSelectActive(context.token);
    const options = selectOptionElements(content);
    const intent = context.openIntentRef.current;
    const selected = options.find((option) => option.getAttribute("aria-selected") === "true");
    const initial =
      intent === "last"
        ? options.at(-1)
        : intent === "first"
          ? options[0]
          : typeof intent === "object"
            ? options.find((option) => normalizeSelectText(option.dataset.textValue ?? option.textContent ?? "").startsWith(normalizeSelectText(intent.typeahead)))
            : (selected ?? options[0]);
    context.setActiveId(initial?.id);
    initial?.scrollIntoView?.({ block: "nearest" });
    const openEvent = preventableEvent();
    openAutoFocusRef.current?.(openEvent);
    if (!openEvent.defaultPrevented) content.focus({ preventScroll: true });
    return () => {
      selectActivity.delete(context.token);
      if (!context.restoreFocusRef.current) return;
      const closeEvent = preventableEvent();
      closeAutoFocusRef.current?.(closeEvent);
      if (!closeEvent.defaultPrevented) context.triggerRef.current?.focus({ preventScroll: true });
    };
  }, [context.contentRef, context.open, context.openIntentRef, context.restoreFocusRef, context.setActiveId, context.token, context.triggerRef]);

  React.useEffect(() => {
    if (!context.open) return;
    const dismiss = (event: PointerEvent | FocusEvent, kind: "pointer" | "focus") => {
      if (handledSelectEvents.has(event) || isInsideSelectBoundary(event.target, context) || topSelectToken() !== context.token) return;
      const owned = preventableEvent(event);
      if (kind === "pointer") onPointerDownOutside?.(owned as SelectPreventableEvent<PointerEvent>);
      else onFocusOutside?.(owned as SelectPreventableEvent<FocusEvent>);
      onInteractOutside?.(owned);
      if (!owned.defaultPrevented) {
        handledSelectEvents.add(event);
        context.setOpen(false, false);
      }
    };
    const handlePointerDown = (event: PointerEvent) => dismiss(event, "pointer");
    const handleFocusIn = (event: FocusEvent) => dismiss(event, "focus");
    const handleDocumentKeyDown = (event: KeyboardEvent) => {
      if (event.key !== "Escape" || handledSelectEvents.has(event) || topSelectToken() !== context.token) return;
      const owned = preventableEvent(event);
      onEscapeKeyDown?.(owned);
      if (owned.defaultPrevented) return;
      handledSelectEvents.add(event);
      event.preventDefault();
      event.stopPropagation();
      context.setOpen(false, true);
    };
    document.addEventListener("pointerdown", handlePointerDown, true);
    document.addEventListener("focusin", handleFocusIn, true);
    document.addEventListener("keydown", handleDocumentKeyDown, true);
    return () => {
      document.removeEventListener("pointerdown", handlePointerDown, true);
      document.removeEventListener("focusin", handleFocusIn, true);
      document.removeEventListener("keydown", handleDocumentKeyDown, true);
      if (typeaheadTimerRef.current !== undefined) window.clearTimeout(typeaheadTimerRef.current);
    };
  }, [context, onEscapeKeyDown, onFocusOutside, onInteractOutside, onPointerDownOutside]);

  if (!context.open || typeof document === "undefined") return null;
  return createPortal(
    <div
      {...props}
      ref={ref}
      id={context.contentId}
      role="listbox"
      tabIndex={props.tabIndex ?? -1}
      aria-activedescendant={context.activeId}
      aria-labelledby={props["aria-label"] ? props["aria-labelledby"] : (props["aria-labelledby"] ?? context.labelId ?? context.triggerRef.current?.id)}
      dir={context.direction}
      data-slot="select-content"
      data-level="menu"
      data-state="open"
      data-side={placement?.side ?? side}
      data-align={align}
      data-position={position}
      data-select-token={context.token}
      data-select-boundary={context.boundary}
      className={cn(
        "text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=top]:slide-in-from-bottom-2 relative z-menu max-h-(--semio-select-content-available-height) min-w-32 origin-(--semio-select-content-transform-origin) overflow-hidden border outline-hidden",
        glassClass,
        position === "popper" && "data-[side=bottom]:translate-y-1 data-[side=top]:-translate-y-1",
        className,
      )}
      style={
        {
          position: "fixed",
          left: placement?.left ?? 0,
          top: placement?.top ?? 0,
          visibility: placement ? undefined : "hidden",
          "--semio-select-trigger-width": `${placement?.triggerWidth ?? 0}px`,
          "--semio-select-trigger-height": `${placement?.triggerHeight ?? 0}px`,
          "--semio-select-content-available-height": `${placement?.availableHeight ?? 0}px`,
          "--semio-select-content-transform-origin": placement?.transformOrigin ?? "center top",
          ...style,
        } as React.CSSProperties
      }
      onPointerDown={(event) => {
        onPointerDown?.(event);
        if (!event.defaultPrevented) markSelectActive(context.token);
      }}
      onKeyDown={(event) => {
        onKeyDown?.(event);
        if (event.defaultPrevented || event.nativeEvent.isComposing) return;
        const movement =
          event.key === "ArrowDown" ? "next" : event.key === "ArrowUp" ? "previous" : event.key === "Home" ? "first" : event.key === "End" ? "last" : event.key === "PageDown" ? "page-next" : event.key === "PageUp" ? "page-previous" : undefined;
        if (movement) {
          event.preventDefault();
          moveActive(context, movement);
          return;
        }
        if (event.key === "Enter" || event.key === " ") {
          event.preventDefault();
          const option = activeOption(context.contentRef.current!, context.activeId);
          if (option?.dataset.value !== undefined) context.selectValue(option.dataset.value);
          return;
        }
        if (event.key === "Tab") {
          context.setOpen(false, false);
          return;
        }
        if (event.key.length !== 1 || event.altKey || event.ctrlKey || event.metaKey) return;
        typeaheadRef.current += event.key;
        if (typeaheadTimerRef.current !== undefined) window.clearTimeout(typeaheadTimerRef.current);
        typeaheadTimerRef.current = window.setTimeout(() => {
          typeaheadRef.current = "";
          typeaheadTimerRef.current = undefined;
        }, 700);
        const option = findTypeaheadOption(context, typeaheadRef.current);
        if (option) {
          context.setActiveId(option.id);
          option.scrollIntoView?.({ block: "nearest" });
        }
      }}
    >
      <SurfaceScope level="menu" fill="glass">
        <SelectScrollUpButton />
        <div ref={context.viewportRef} data-slot="select-viewport" className={cn("max-h-(--semio-select-content-available-height) overflow-y-auto p-single", position === "popper" && "w-full min-w-(--semio-select-trigger-width) scroll-my-single")}>
          {children}
        </div>
        <SelectScrollDownButton />
      </SurfaceScope>
    </div>,
    container ?? document.body,
  );
});
// #endregion 📍️Content

// #region 📋️Collection
/** 🗂️ Groups related options under an owned stable label association. */
const SelectGroup = React.forwardRef<HTMLDivElement, SelectGroupProps>(function SelectGroup({ children, ...props }, forwardedRef) {
  const generatedId = React.useId().replace(/[^A-Za-z0-9_-]/g, "");
  const labelId = `semio-select-group-${generatedId}-label`;
  return (
    <SelectGroupContext.Provider value={labelId}>
      <div {...props} ref={forwardedRef} role="group" aria-labelledby={props["aria-label"] ? props["aria-labelledby"] : (props["aria-labelledby"] ?? labelId)} data-slot="select-group">
        {children}
      </div>
    </SelectGroupContext.Provider>
  );
});

/** 🏷️ Labels the nearest owned option group. */
const SelectLabel = React.forwardRef<HTMLDivElement, SelectLabelProps>(function SelectLabel({ className, id, ...props }, forwardedRef) {
  const groupLabelId = React.useContext(SelectGroupContext);
  return <div {...props} ref={forwardedRef} id={id ?? groupLabelId ?? undefined} data-slot="select-label" className={cn("text-muted-foreground p-single text-xs", className)} />;
});

/** ✅️ Renders one stable owned option and delegates value authority to its root. */
const SelectItem = React.forwardRef<HTMLDivElement, SelectItemProps>(function SelectItem({ className, children, id, value, disabled = false, textValue, icon, onClick, onPointerDown, onPointerMove, onFocus, ...props }, forwardedRef) {
  const context = useSelectContext();
  const generatedId = React.useId().replace(/[^A-Za-z0-9_-]/g, "");
  const itemId = id ?? `${context.token}-item-${generatedId}`;
  const selected = context.value === value;
  const active = context.activeId === itemId;
  return (
    <div
      {...props}
      ref={forwardedRef}
      id={itemId}
      role="option"
      aria-selected={selected}
      aria-disabled={disabled || undefined}
      data-slot="select-item"
      data-value={value}
      data-text-value={textValue ?? nodeText(children)}
      data-state={selected ? "checked" : "unchecked"}
      data-highlighted={active ? "" : undefined}
      data-disabled={disabled ? "true" : undefined}
      className={cn(
        "focus:text-emphasized [&_svg:not([class*='text-'])]:text-muted-foreground relative flex w-full items-center gap-single rounded-sm py-single pe-medium ps-single text-sm outline-hidden select-none data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-tiny *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-single",
        "cursor-selectable",
        menuListItemClassName,
        className,
      )}
      onFocus={(event) => {
        onFocus?.(event);
        if (!event.defaultPrevented && !disabled) context.setActiveId(itemId);
      }}
      onPointerMove={(event) => {
        onPointerMove?.(event);
        if (!event.defaultPrevented && !disabled && event.pointerType !== "touch") context.setActiveId(itemId);
      }}
      onPointerDown={(event) => {
        onPointerDown?.(event);
        if (!event.defaultPrevented && !disabled && event.pointerType !== "touch") event.preventDefault();
      }}
      onClick={(event) => {
        onClick?.(event);
        if (!event.defaultPrevented && !disabled) context.selectValue(value);
      }}
    >
      <span aria-hidden className="absolute end-2 flex size-tiny.5 items-center justify-center">
        {selected ? <CheckIconAlt className="size-tiny" /> : null}
      </span>
      {icon ? (
        <span data-slot="select-item-icon" className="inline-flex shrink-0">
          <Icon icon={icon} size="small" />
        </span>
      ) : null}
      <span data-slot="select-item-text">{children}</span>
    </div>
  );
});

/** ➖️ Separates option groups without entering the active-descendant set. */
const SelectSeparator = React.forwardRef<HTMLDivElement, SelectSeparatorProps>(function SelectSeparator({ className, ...props }, forwardedRef) {
  return <div {...props} ref={forwardedRef} role="separator" aria-orientation="horizontal" data-slot="select-separator" className={cn("bg-border pointer-events-none -mx-single my-single h-px", className)} />;
});

function scrollSelectViewport(context: SelectContextValue, direction: -1 | 1): void {
  const viewport = context.viewportRef.current;
  if (!viewport) return;
  const distance = Math.max(24, Math.floor(viewport.clientHeight * 0.8)) * direction;
  if (typeof viewport.scrollBy === "function") viewport.scrollBy({ top: distance, behavior: "auto" });
  else viewport.scrollTop += distance;
}

/** ⬆️ Scrolls the owned viewport upward without adding a nested interactive control. */
const SelectScrollUpButton = React.forwardRef<HTMLDivElement, SelectScrollButtonProps>(function SelectScrollUpButton({ className, onPointerDown, ...props }, forwardedRef) {
  const context = useSelectContext();
  return (
    <div
      {...props}
      ref={forwardedRef}
      aria-hidden
      data-slot="select-scroll-up-button"
      className={cn("flex cursor-default items-center justify-center py-single hover:bg-hover-interactive-fill", className)}
      onPointerDown={(event) => {
        onPointerDown?.(event);
        if (!event.defaultPrevented) {
          event.preventDefault();
          scrollSelectViewport(context, -1);
        }
      }}
    >
      <ChevronUpIcon className="size-tiny" />
    </div>
  );
});

/** ⬇️ Scrolls the owned viewport downward without adding a nested interactive control. */
const SelectScrollDownButton = React.forwardRef<HTMLDivElement, SelectScrollButtonProps>(function SelectScrollDownButton({ className, onPointerDown, ...props }, forwardedRef) {
  const context = useSelectContext();
  return (
    <div
      {...props}
      ref={forwardedRef}
      aria-hidden
      data-slot="select-scroll-down-button"
      className={cn("flex cursor-default items-center justify-center py-single hover:bg-hover-interactive-fill", className)}
      onPointerDown={(event) => {
        onPointerDown?.(event);
        if (!event.defaultPrevented) {
          event.preventDefault();
          scrollSelectViewport(context, 1);
        }
      }}
    >
      <ChevronDownIconAlt className="size-tiny" />
    </div>
  );
});
// #endregion 📋️Collection

export { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectScrollDownButton, SelectScrollUpButton, SelectSeparator, SelectTrigger, SelectValue };
// #endregion 🔎️Select
