// #region 🧲️Header
// 💻️ framework/ui/elements/⌨️Command/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { borderNormalBottomClass } from "../../🔨️modules/📏️border-presentation/🟦️.ts";
import { uiFormControlBrowserDefaultProps } from "../../🔨️modules/📝️form-control-presentation/🟦️.ts";
import { menuListItemClassName } from "../../🔨️modules/📋️menu-item-presentation/🟦️.ts";
import { type UiLabel } from "../🎗️UiLabel/🟦️.tsx";
import { useLabel } from "../🏷️Label/🟦️.tsx";
import { SearchIcon } from "../🔣️Icons/🟦️.tsx";
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, type DialogProps } from "../💬️Dialog/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🧭️Contracts
export type CommandFilter = (value: string, search: string, keywords: readonly string[]) => number;

export interface CommandProps extends Omit<React.HTMLAttributes<HTMLDivElement>, "defaultValue" | "onChange"> {
  readonly value?: string;
  readonly defaultValue?: string;
  readonly onValueChange?: (value: string) => void;
  readonly shouldFilter?: boolean;
  readonly filter?: CommandFilter;
  readonly loop?: boolean;
}

export interface CommandDialogProps extends DialogProps {
  readonly title?: UiLabel;
  readonly description?: string;
  readonly children?: React.ReactNode;
  readonly className?: string;
  readonly showCloseButton?: boolean;
  readonly shouldFilter?: boolean;
}

export interface CommandInputProps extends Omit<React.InputHTMLAttributes<HTMLInputElement>, "defaultValue" | "value"> {
  readonly value?: string;
  readonly defaultValue?: string;
  readonly onValueChange?: (value: string) => void;
}

export type CommandListProps = React.HTMLAttributes<HTMLDivElement>;
export type CommandEmptyProps = React.HTMLAttributes<HTMLDivElement>;

export interface CommandGroupProps extends React.HTMLAttributes<HTMLDivElement> {
  readonly heading?: React.ReactNode;
  readonly forceMount?: boolean;
}

export interface CommandItemProps extends Omit<React.HTMLAttributes<HTMLDivElement>, "onSelect"> {
  readonly value?: string;
  readonly keywords?: readonly string[];
  readonly disabled?: boolean;
  readonly forceMount?: boolean;
  readonly onSelect?: (value: string) => void;
}

export type CommandShortcutProps = React.HTMLAttributes<HTMLSpanElement>;

interface CommandItemRecord {
  readonly id: string;
  readonly value: string;
  readonly keywords: readonly string[];
  readonly disabled: boolean;
  readonly authoredHidden: boolean;
  readonly forceMount: boolean;
  readonly groupId?: string;
  readonly order: number;
  readonly activate: () => void;
}

interface CommandContextValue {
  readonly activeId?: string;
  readonly listId: string;
  readonly search: string;
  readonly items: ReadonlyMap<string, CommandItemRecord>;
  readonly visibleItems: readonly CommandItemRecord[];
  readonly registerItem: (item: CommandItemRecord) => () => void;
  readonly activate: (item: CommandItemRecord) => void;
  readonly setActiveId: (id: string) => void;
  readonly setSearch: (value: string) => void;
  readonly isVisible: (item: CommandItemRecord) => boolean;
}

const CommandContext = React.createContext<CommandContextValue | null>(null);
const CommandGroupContext = React.createContext<string | undefined>(undefined);

function useCommandContext(name: string): CommandContextValue {
  const context = React.useContext(CommandContext);
  if (!context) throw new Error(`${name} must be used within Command`);
  return context;
}
// #endregion 🧭️Contracts

// #region 🔎️Ranking
function commandText(value: React.ReactNode): string {
  if (typeof value === "string" || typeof value === "number") return String(value);
  if (Array.isArray(value)) return value.map(commandText).join(" ");
  if (!React.isValidElement<{ children?: React.ReactNode; "aria-label"?: string }>(value)) return "";
  return value.props["aria-label"] ?? commandText(value.props.children);
}

function normalizeCommandText(value: string): string {
  return value
    .normalize("NFKD")
    .replace(/[\u0300-\u036f]/g, "")
    .toLowerCase()
    .trim()
    .replace(/\s+/g, " ");
}

/** @emoji 🧮️ Produces a stable owned command match score without browser layout or third-party state. */
export function rankCommandValue(value: string, search: string, keywords: readonly string[] = []): number {
  const query = normalizeCommandText(search);
  if (!query) return 1;
  const candidates = [normalizeCommandText(value), ...keywords.map(normalizeCommandText)].filter(Boolean);
  let best = 0;
  for (const candidate of candidates) {
    if (candidate === query) best = Math.max(best, 10_000);
    else if (candidate.startsWith(query)) best = Math.max(best, 8_000 - candidate.length);
    else if (candidate.split(" ").some((token) => token.startsWith(query))) best = Math.max(best, 6_000 - candidate.length);
    else {
      const index = candidate.indexOf(query);
      if (index >= 0) best = Math.max(best, 4_000 - index * 10 - candidate.length);
      else {
        let cursor = 0;
        let spread = 0;
        let previous = -1;
        for (const character of query) {
          const next = candidate.indexOf(character, cursor);
          if (next < 0) {
            cursor = -1;
            break;
          }
          if (previous >= 0) spread += next - previous - 1;
          previous = next;
          cursor = next + 1;
        }
        if (cursor >= 0) best = Math.max(best, 2_000 - spread * 10 - candidate.length);
      }
    }
  }
  return Math.max(0, best);
}
// #endregion 🔎️Ranking

// #region 🪆️Command
/** @emoji ⌨️ Owns command filtering, active-descendant navigation, and exact-once activation. */
function Command({ className, value, defaultValue = "", onValueChange, shouldFilter = true, filter = rankCommandValue, loop = false, onKeyDown, children, ...props }: CommandProps) {
  const generatedId = React.useId().replaceAll(":", "");
  const listId = `${props.id ?? `command-${generatedId}`}-list`;
  const controlled = value !== undefined;
  const [uncontrolledValue, setUncontrolledValue] = React.useState(defaultValue);
  const selectedValue = controlled ? value : uncontrolledValue;
  const [search, setSearch] = React.useState("");
  const [items, setItems] = React.useState<ReadonlyMap<string, CommandItemRecord>>(() => new Map());
  const [activeIdState, setActiveIdState] = React.useState<string>();
  const score = React.useCallback((item: CommandItemRecord) => (shouldFilter && !item.forceMount ? filter(item.value, search, item.keywords) : 1), [filter, search, shouldFilter]);
  const isVisible = React.useCallback((item: CommandItemRecord) => !item.authoredHidden && (item.forceMount || !shouldFilter || score(item) > 0), [score, shouldFilter]);
  const visibleItems = React.useMemo(() => [...items.values()].filter(isVisible).sort((left, right) => score(right) - score(left) || left.order - right.order), [isVisible, items, score]);
  const enabledItems = React.useMemo(() => visibleItems.filter((item) => !item.disabled), [visibleItems]);
  const selectedItem = React.useMemo(() => enabledItems.find((item) => item.value === selectedValue), [enabledItems, selectedValue]);
  const activeId = enabledItems.some((item) => item.id === activeIdState) ? activeIdState : (selectedItem?.id ?? enabledItems[0]?.id);

  const registerItem = React.useCallback((item: CommandItemRecord) => {
    setItems((current) => {
      const next = new Map(current);
      next.set(item.id, item);
      return next;
    });
    return () => {
      setItems((current) => {
        if (!current.has(item.id)) return current;
        const next = new Map(current);
        next.delete(item.id);
        return next;
      });
    };
  }, []);
  React.useEffect(() => setActiveIdState(activeId), [activeId]);

  const move = React.useCallback(
    (kind: "first" | "last" | "next" | "previous" | "page-next" | "page-previous") => {
      if (!enabledItems.length) return;
      const index = Math.max(
        0,
        enabledItems.findIndex((item) => item.id === activeId),
      );
      let next = index;
      if (kind === "first") next = 0;
      if (kind === "last") next = enabledItems.length - 1;
      if (kind === "next") next = index + 1;
      if (kind === "previous") next = index - 1;
      if (kind === "page-next") next = index + 5;
      if (kind === "page-previous") next = index - 5;
      if (loop) next = (next + enabledItems.length) % enabledItems.length;
      else next = Math.min(enabledItems.length - 1, Math.max(0, next));
      setActiveIdState(enabledItems[next]?.id);
    },
    [activeId, enabledItems, loop],
  );

  const activate = React.useCallback(
    (item: CommandItemRecord) => {
      if (item.disabled || !isVisible(item)) return;
      setActiveIdState(item.id);
      if (!controlled) setUncontrolledValue(item.value);
      onValueChange?.(item.value);
      item.activate();
    },
    [controlled, isVisible, onValueChange],
  );
  const context = React.useMemo<CommandContextValue>(
    () => ({ activeId, listId, search, items, visibleItems, registerItem, activate, setActiveId: setActiveIdState, setSearch, isVisible }),
    [activeId, activate, isVisible, items, listId, registerItem, search, visibleItems],
  );

  return (
    <CommandContext.Provider value={context}>
      <div
        data-slot="command"
        data-value={selectedValue || undefined}
        className={cn("bg-transparent text-popover-foreground flex h-full w-full flex-col overflow-hidden", className)}
        {...props}
        onKeyDown={(event) => {
          onKeyDown?.(event);
          if (event.defaultPrevented || event.nativeEvent.isComposing || event.keyCode === 229) return;
          const movement =
            event.key === "ArrowDown" ? "next" : event.key === "ArrowUp" ? "previous" : event.key === "Home" ? "first" : event.key === "End" ? "last" : event.key === "PageDown" ? "page-next" : event.key === "PageUp" ? "page-previous" : undefined;
          if (movement) {
            event.preventDefault();
            move(movement);
            return;
          }
          const isTextInput = event.target instanceof HTMLInputElement || event.target instanceof HTMLTextAreaElement;
          if ((event.key === "Enter" || (event.key === " " && !isTextInput)) && activeId) {
            const item = items.get(activeId);
            if (item) {
              event.preventDefault();
              activate(item);
            }
          }
        }}
      >
        {children}
      </div>
    </CommandContext.Provider>
  );
}

/** @emoji 🪟️ Composes Command strictly inside the repository-owned modal boundary. */
function CommandDialog({ title, description, children, className, showCloseButton = true, shouldFilter, ...props }: CommandDialogProps) {
  const commandPaletteLabel = useLabel("ui.common.commandPalette");
  const searchForCommandLabel = useLabel("ui.common.searchForCommand");
  return (
    <Dialog {...props}>
      <DialogContent className={cn("overflow-hidden p-0", className)} showCloseButton={showCloseButton}>
        <DialogHeader className="sr-only">
          <DialogTitle>{title ?? commandPaletteLabel}</DialogTitle>
          <DialogDescription>{description ?? searchForCommandLabel}</DialogDescription>
        </DialogHeader>
        <Command
          shouldFilter={shouldFilter}
          className="[&_[data-slot=command-group-heading]]:text-muted-foreground **:data-[slot=command-input-wrapper]:h-large [&_[data-slot=command-group-heading]]:px-single [&_[data-slot=command-group-heading]]:font-medium [&_[data-slot=command-group]]:px-single [&_[data-slot=command-group]:not([hidden])_~[data-slot=command-group]]:pt-0 [&_[data-slot=command-input-wrapper]_svg]:h-small [&_[data-slot=command-input-wrapper]_svg]:w-small [&_[data-slot=command-input]]:h-large [&_[data-slot=command-item]]:px-single [&_[data-slot=command-item]]:py-tiny [&_[data-slot=command-item]_svg]:h-small [&_[data-slot=command-item]_svg]:w-small"
        >
          {children}
        </Command>
      </DialogContent>
    </Dialog>
  );
}

/** @emoji 🔍️ Owns the command query while preserving authoritative controlled input state. */
const CommandInput = React.forwardRef<HTMLInputElement, CommandInputProps>(function CommandInput({ className, value, defaultValue = "", onValueChange, onChange, onCompositionStart, onCompositionEnd, ...props }, ref) {
  const context = useCommandContext("CommandInput");
  const controlled = value !== undefined;
  const [uncontrolledValue, setUncontrolledValue] = React.useState(defaultValue);
  const resolvedValue = controlled ? value : uncontrolledValue;
  React.useEffect(() => context.setSearch(resolvedValue), [context.setSearch, resolvedValue]);
  return (
    <div data-slot="command-input-wrapper" className={cn("flex h-medium items-center gap-single px-tiny", borderNormalBottomClass)}>
      <SearchIcon className="size-small shrink-0 opacity-50" />
      <input
        {...uiFormControlBrowserDefaultProps}
        {...props}
        ref={ref}
        data-slot="command-input"
        role="combobox"
        aria-autocomplete="list"
        aria-controls={context.listId}
        aria-expanded="true"
        aria-activedescendant={context.activeId}
        className={cn("placeholder:text-muted-foreground flex h-medium w-full bg-transparent text-sm outline-hidden disabled:cursor-not-allowed disabled:opacity-50", className)}
        value={resolvedValue}
        onChange={(event) => {
          onChange?.(event);
          if (event.defaultPrevented) return;
          const next = event.currentTarget.value;
          if (!controlled) {
            setUncontrolledValue(next);
            context.setSearch(next);
          }
          onValueChange?.(next);
        }}
        onCompositionStart={onCompositionStart}
        onCompositionEnd={onCompositionEnd}
      />
    </div>
  );
});

/** @emoji 📜️ Supplies the owned listbox boundary associated with CommandInput. */
const CommandList = React.forwardRef<HTMLDivElement, CommandListProps>(function CommandList({ className, ...props }, ref) {
  const context = useCommandContext("CommandList");
  return <div {...props} ref={ref} id={context.listId} role="listbox" data-slot="command-list" className={cn("max-h-layout-command scroll-py-single overflow-x-hidden overflow-y-auto", className)} />;
});

/** @emoji 🕳️ Stays mounted as a live status and is hidden whenever at least one result remains. */
const CommandEmpty = React.forwardRef<HTMLDivElement, CommandEmptyProps>(function CommandEmpty({ className, hidden, ...props }, ref) {
  const context = useCommandContext("CommandEmpty");
  return <div {...props} ref={ref} role="status" aria-live="polite" data-slot="command-empty" className={cn("py-medium text-center text-sm", className)} hidden={hidden || context.visibleItems.length > 0} />;
});

/** @emoji 🗂️ Groups command options and hides, rather than unmounts, empty result groups. */
const CommandGroup = React.forwardRef<HTMLDivElement, CommandGroupProps>(function CommandGroup({ className, heading, children, hidden, forceMount = false, ...props }, ref) {
  const context = useCommandContext("CommandGroup");
  const generatedId = React.useId().replaceAll(":", "");
  const groupId = props.id ?? `command-group-${generatedId}`;
  const headingId = `${groupId}-heading`;
  const registered = [...context.items.values()].filter((item) => item.groupId === groupId);
  const groupHidden = Boolean(hidden) || (!forceMount && registered.length > 0 && !registered.some(context.isVisible));
  return (
    <CommandGroupContext.Provider value={groupId}>
      <div
        {...props}
        ref={ref}
        id={groupId}
        role="group"
        aria-labelledby={heading === undefined ? undefined : headingId}
        data-slot="command-group"
        hidden={groupHidden}
        className={cn(
          "text-element [&_[data-slot=command-group-heading]]:text-muted-foreground overflow-hidden p-single [&_[data-slot=command-group-heading]]:px-single [&_[data-slot=command-group-heading]]:py-single [&_[data-slot=command-group-heading]]:text-xs [&_[data-slot=command-group-heading]]:font-medium",
          className,
        )}
      >
        {heading === undefined ? null : (
          <div id={headingId} data-slot="command-group-heading">
            {heading}
          </div>
        )}
        {children}
      </div>
    </CommandGroupContext.Provider>
  );
});

let commandItemOrder = 0;
const EMPTY_COMMAND_KEYWORDS: readonly string[] = [];

/** @emoji 🎯️ Owns one stable option identity and activates it once per accepted gesture. */
const CommandItem = React.forwardRef<HTMLDivElement, CommandItemProps>(function CommandItem(
  { className, value, keywords = EMPTY_COMMAND_KEYWORDS, disabled = false, forceMount = false, hidden = false, onSelect, onClick, onPointerDown, onPointerMove, children, id, ...props },
  forwardedRef,
) {
  const context = useCommandContext("CommandItem");
  const groupId = React.useContext(CommandGroupContext);
  const generatedId = React.useId().replaceAll(":", "");
  const itemId = id ?? `command-item-${generatedId}`;
  const resolvedValue = value ?? commandText(children).trim();
  const keywordKey = keywords.join("\u0000");
  const stableKeywords = React.useMemo(() => (keywordKey ? keywordKey.split("\u0000") : EMPTY_COMMAND_KEYWORDS), [keywordKey]);
  const orderRef = React.useRef<number>(undefined);
  if (orderRef.current === undefined) orderRef.current = commandItemOrder++;
  const elementRef = React.useRef<HTMLDivElement>(null);
  const suppressClick = React.useRef(false);
  React.useImperativeHandle(forwardedRef, () => elementRef.current as HTMLDivElement);
  const activate = React.useCallback(() => onSelect?.(resolvedValue), [onSelect, resolvedValue]);
  const record = React.useMemo<CommandItemRecord>(
    () => ({ id: itemId, value: resolvedValue, keywords: stableKeywords, disabled, authoredHidden: Boolean(hidden), forceMount, groupId, order: orderRef.current!, activate }),
    [activate, disabled, forceMount, groupId, hidden, itemId, resolvedValue, stableKeywords],
  );
  React.useEffect(() => context.registerItem(record), [context.registerItem, record]);
  const visible = context.isVisible(record);
  const active = context.activeId === itemId;
  return (
    <div
      {...props}
      ref={elementRef}
      id={itemId}
      role="option"
      aria-selected={active}
      aria-disabled={disabled || undefined}
      data-slot="command-item"
      data-active={active ? "true" : undefined}
      data-disabled={disabled ? "true" : undefined}
      data-value={resolvedValue}
      hidden={!visible}
      className={cn(
        "[&_svg:not([class*='text-'])]:text-muted-foreground relative flex items-center gap-single p-single text-sm outline-hidden select-none data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-tiny cursor-selectable",
        menuListItemClassName,
        className,
      )}
      onPointerMove={(event) => {
        onPointerMove?.(event);
        if (!event.defaultPrevented && !disabled) context.setActiveId(itemId);
      }}
      onPointerDown={(event) => {
        suppressClick.current = false;
        onPointerDown?.(event);
        suppressClick.current = event.defaultPrevented;
      }}
      onClick={(event) => {
        onClick?.(event);
        if (event.defaultPrevented || disabled || suppressClick.current) {
          suppressClick.current = false;
          return;
        }
        context.activate(record);
      }}
    >
      {children}
    </div>
  );
});

/** @emoji ⌘️ Renders decorative shortcut text without entering the option focus model. */
const CommandShortcut = React.forwardRef<HTMLSpanElement, CommandShortcutProps>(function CommandShortcut({ className, ...props }, ref) {
  return <span {...props} ref={ref} aria-hidden="true" data-slot="command-shortcut" className={cn("text-muted-foreground ms-auto text-xs tracking-widest", className)} />;
});

export { Command, CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandShortcut };
// #endregion 🪆️Command
