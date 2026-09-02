// #region 🧲️Header
// 💻️ framework/ui/elements/📑️Tabs/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { interactiveTabActiveClass, interactiveHoverClass } from "../../🔨️modules/🖱️interaction-presentation/🟦️.ts";
// #endregion 🔌️Adapters

// #region 📑️Contracts
type TabsOrientation = "horizontal" | "vertical";
type TabsDirection = "ltr" | "rtl";
type TabsActivationMode = "automatic" | "manual";

interface TabsProps extends Omit<React.HTMLAttributes<HTMLDivElement>, "defaultValue" | "dir" | "onChange"> {
  value?: string;
  defaultValue?: string;
  onValueChange?: (value: string) => void;
  orientation?: TabsOrientation;
  dir?: TabsDirection;
  activationMode?: TabsActivationMode;
  ref?: React.Ref<HTMLDivElement>;
}

interface TabsListProps extends React.HTMLAttributes<HTMLDivElement> {
  ref?: React.Ref<HTMLDivElement>;
}

interface TabsTriggerProps extends Omit<React.ButtonHTMLAttributes<HTMLButtonElement>, "value"> {
  value: string;
  ref?: React.Ref<HTMLButtonElement>;
}

interface TabsContentProps extends Omit<React.HTMLAttributes<HTMLDivElement>, "hidden"> {
  value: string;
  ref?: React.Ref<HTMLDivElement>;
}

interface TabsContextValue {
  value: string | undefined;
  orientation: TabsOrientation;
  dir: TabsDirection;
  activationMode: TabsActivationMode;
  focusValue: string | undefined;
  activate: (value: string) => void;
  setFocusValue: (value: string) => void;
  associationId: (kind: "trigger" | "content", value: string) => string;
  triggerIds: ReadonlyMap<string, string>;
  contentIds: ReadonlyMap<string, string>;
  registerId: (kind: "trigger" | "content", value: string, id: string) => () => void;
}
// #endregion 📑️Contracts

// #region 📑️Tabs
const TabsContext = React.createContext<TabsContextValue | null>(null);

/** 🧭️ Resolves the nearest owned tabs state. */
function useTabsContext(): TabsContextValue {
  const context = React.useContext(TabsContext);
  if (!context) throw new Error("Tabs parts must be rendered inside Tabs");
  return context;
}

/** 🆔️ Produces a domain-separated injective HTML-id token from exact UTF-16 code units. */
function idToken(domain: string, value: string): string {
  let encoded = "";
  for (let index = 0; index < value.length; index += 1) encoded += value.charCodeAt(index).toString(16).padStart(4, "0");
  return `${domain}-${value.length.toString(16)}-${encoded}`;
}

/** 📑️ Owns controlled or uncontrolled selection and tab associations. */
function Tabs({ className, value, defaultValue, onValueChange, orientation = "horizontal", dir = "ltr", activationMode = "automatic", id, ref, children, ...props }: TabsProps) {
  const instanceId = React.useId();
  const baseId = `tabs-${idToken("root", id ?? "")}-${idToken("instance", instanceId)}`;
  const controlled = value !== undefined;
  const [uncontrolledValue, setUncontrolledValue] = React.useState(defaultValue);
  const currentValue = controlled ? value : uncontrolledValue;
  const [focusValue, setFocusValue] = React.useState(currentValue);
  const [triggerIds, setTriggerIds] = React.useState<ReadonlyMap<string, string>>(() => new Map());
  const [contentIds, setContentIds] = React.useState<ReadonlyMap<string, string>>(() => new Map());

  React.useEffect(() => {
    if (currentValue !== undefined) setFocusValue(currentValue);
  }, [currentValue]);

  const activate = React.useCallback(
    (nextValue: string) => {
      if (nextValue === currentValue) return;
      if (!controlled) setUncontrolledValue(nextValue);
      onValueChange?.(nextValue);
    },
    [controlled, currentValue, onValueChange],
  );
  const associationId = React.useCallback((kind: "trigger" | "content", tabValue: string) => `${baseId}-${idToken(kind, tabValue)}`, [baseId]);
  const registerId = React.useCallback((kind: "trigger" | "content", tabValue: string, partId: string) => {
    const setter = kind === "trigger" ? setTriggerIds : setContentIds;
    setter((current) => {
      if (current.get(tabValue) === partId) return current;
      const next = new Map(current);
      next.set(tabValue, partId);
      return next;
    });
    return () =>
      setter((current) => {
        if (current.get(tabValue) !== partId) return current;
        const next = new Map(current);
        next.delete(tabValue);
        return next;
      });
  }, []);
  const context = React.useMemo<TabsContextValue>(
    () => ({ value: currentValue, orientation, dir, activationMode, focusValue, activate, setFocusValue, associationId, triggerIds, contentIds, registerId }),
    [activate, activationMode, associationId, contentIds, currentValue, dir, focusValue, orientation, registerId, triggerIds],
  );

  return (
    <TabsContext.Provider value={context}>
      <div {...props} ref={ref} id={id} dir={dir} data-slot="tabs" data-orientation={orientation} className={cn("flex flex-col gap-single", className)}>
        {children}
      </div>
    </TabsContext.Provider>
  );
}

/** 🗂️ Hosts one accessible tablist and initializes its roving focus stop. */
function TabsList({ className, ref, children, ...props }: TabsListProps) {
  const context = useTabsContext();
  const localRef = React.useRef<HTMLDivElement | null>(null);
  React.useLayoutEffect(() => {
    if (context.focusValue !== undefined) return;
    const first = localRef.current?.querySelector<HTMLButtonElement>('[role="tab"]:not(:disabled)');
    const firstValue = first?.dataset.tabsValue;
    if (firstValue !== undefined) context.setFocusValue(firstValue);
  }, [context]);
  const setRef = React.useCallback(
    (node: HTMLDivElement | null) => {
      localRef.current = node;
      if (typeof ref === "function") ref(node);
      else if (ref) (ref as React.MutableRefObject<HTMLDivElement | null>).current = node;
    },
    [ref],
  );
  return (
    <div
      {...props}
      ref={setRef}
      role="tablist"
      aria-orientation={context.orientation}
      data-slot="tabs-list"
      data-orientation={context.orientation}
      className={cn("text-muted-foreground inline-flex h-large w-fit items-center justify-center p-single bg-transparent", className)}
    >
      {children}
    </div>
  );
}

/** ⌨️ Moves focus through the enabled triggers in one tablist. */
function moveTabFocus(event: React.KeyboardEvent<HTMLButtonElement>, context: TabsContextValue, value: string): void {
  const horizontalPrevious = context.dir === "rtl" ? "ArrowRight" : "ArrowLeft";
  const horizontalNext = context.dir === "rtl" ? "ArrowLeft" : "ArrowRight";
  const previousKey = context.orientation === "horizontal" ? horizontalPrevious : "ArrowUp";
  const nextKey = context.orientation === "horizontal" ? horizontalNext : "ArrowDown";
  if (![previousKey, nextKey, "Home", "End"].includes(event.key)) return;
  const list = event.currentTarget.closest('[role="tablist"]');
  const triggers = Array.from(list?.querySelectorAll<HTMLButtonElement>('[role="tab"]:not(:disabled)') ?? []);
  const currentIndex = triggers.findIndex((trigger) => trigger.dataset.tabsValue === value);
  if (currentIndex < 0 || triggers.length === 0) return;
  const nextIndex = event.key === "Home" ? 0 : event.key === "End" ? triggers.length - 1 : event.key === previousKey ? (currentIndex - 1 + triggers.length) % triggers.length : (currentIndex + 1) % triggers.length;
  const next = triggers[nextIndex];
  const nextValue = next?.dataset.tabsValue;
  if (!next || nextValue === undefined) return;
  event.preventDefault();
  next.focus();
  context.setFocusValue(nextValue);
  if (context.activationMode === "automatic") context.activate(nextValue);
}

/** 🏷️ Renders one accessible tab trigger. */
function TabsTrigger({ className, value, disabled = false, id, ref, onClick, onFocus, onKeyDown, ...props }: TabsTriggerProps) {
  const context = useTabsContext();
  const selected = context.value === value;
  const triggerId = id ?? context.associationId("trigger", value);
  const contentId = context.contentIds.get(value) ?? context.associationId("content", value);
  React.useEffect(() => context.registerId("trigger", value, triggerId), [context.registerId, triggerId, value]);
  return (
    <button
      {...props}
      ref={ref}
      type="button"
      id={triggerId}
      role="tab"
      disabled={disabled}
      aria-selected={selected}
      aria-controls={contentId}
      tabIndex={disabled ? -1 : context.focusValue === value ? 0 : -1}
      data-slot="tabs-trigger"
      data-tabs-value={value}
      data-state={selected ? "active" : "inactive"}
      data-disabled={disabled ? "" : undefined}
      className={cn(
        "focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:outline-ring text-element inline-flex h-[calc(100%-var(--stroke-hairline))] flex-1 items-center justify-center gap-single border border-transparent p-single text-sm font-medium whitespace-nowrap transition-[color,box-shadow] focus-visible:ring-[length:var(--stroke-focus)] focus-visible:outline-1 disabled:pointer-events-none disabled:opacity-50 data-[state=active]:shadow-sm [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
        interactiveTabActiveClass,
        interactiveHoverClass,
        className,
      )}
      onClick={(event) => {
        onClick?.(event);
        if (!event.defaultPrevented && !disabled) context.activate(value);
      }}
      onFocus={(event) => {
        onFocus?.(event);
        if (!event.defaultPrevented && !disabled) context.setFocusValue(value);
      }}
      onKeyDown={(event) => {
        onKeyDown?.(event);
        if (event.defaultPrevented || disabled) return;
        if (context.activationMode === "manual" && (event.key === "Enter" || event.key === " ")) {
          event.preventDefault();
          context.activate(value);
          return;
        }
        moveTabFocus(event, context, value);
      }}
    />
  );
}

/** 📄️ Renders only the active panel so inactive descendants cannot run production effects. */
function TabsContent({ className, value, id, ref, ...props }: TabsContentProps) {
  const context = useTabsContext();
  const selected = context.value === value;
  const contentId = id ?? context.associationId("content", value);
  const triggerId = context.triggerIds.get(value) ?? context.associationId("trigger", value);
  React.useEffect(() => context.registerId("content", value, contentId), [contentId, context.registerId, value]);
  if (!selected) return null;
  return <div {...props} ref={ref} id={contentId} role="tabpanel" aria-labelledby={triggerId} tabIndex={0} data-slot="tabs-content" data-state="active" className={cn("flex-1 outline-none", className)} />;
}

export { Tabs, TabsContent, TabsList, TabsTrigger };
export type { TabsProps, TabsListProps, TabsTriggerProps, TabsContentProps, TabsOrientation, TabsDirection, TabsActivationMode };
// #endregion 📑️Tabs
