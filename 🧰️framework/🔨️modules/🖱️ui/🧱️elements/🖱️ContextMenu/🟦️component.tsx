// #region 🧲️Header
// 💻️ framework/ui/elements/🖱️ContextMenu/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { createPortal } from "react-dom";
import { reactHostPort } from "../🔌️Ports/🟦️component.tsx";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️component.ts";
import { type UiLabel, uiDataLabel } from "../🏷️UiLabel/🟦️component.tsx";
import { Icon, type ControlIcon, type IconSource } from "../🔣️Icons/🟦️component.tsx";
import { useLabel } from "../🏷️Label/🟦️component.tsx";
import { useShellScopeOptional } from "../🐚️ShellScope/🟦️component.tsx";
import { useFlow } from "../../🔨️modules/🧭️flow-direction-context/🟦️component.tsx";
import { formatKeybindingShortcut } from "../../🔨️modules/⌨️keybinding-text-interpretation/🟦️component.ts";
import { floatingMenuItemClass, ContextMenuChrome } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// #endregion 🔌️Adapters

// #region 🖱️ContextMenu
const contextMenuShortcutClassName = "ms-auto text-xs text-muted-foreground ps-tiny";
const contextMenuOrdinalClassName = "w-small shrink-0 text-center text-xs text-muted-foreground tabular-nums";

/** @emoji 🪟️ Context-menu row — same density as {@link floatingMenuItemClass}; `checked` paints the active/preview highlight (no tick/checkmark), kept through hover like {@link CanvasPickMenu}. */
function contextMenuItemClassName(item: Pick<ContextMenuItem, "checked" | "destructive">, ...extra: Array<string | false | null | undefined>): string {
  return cn(
    floatingMenuItemClass,
    "whitespace-nowrap data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
    item.destructive && "text-destructive focus:bg-destructive/10 hover:bg-destructive/10",
    ...extra,
    item.checked && "bg-active-base text-emphasized hover:bg-active-base/90 hover:text-emphasized",
  );
}

/**
 * 🧩️ Serializable right-click entry for {@link ContextMenu} and puzzle 2d/window surfaces.
 **/
export interface ContextMenuItem {
  id: string;
  label?: UiLabel;
  icon?: ControlIcon;
  color?: string;
  shortcut?: string;
  disabled?: boolean;
  separator?: boolean;
  checked?: boolean;
  destructive?: boolean;
  onSelect?: (event: Event) => void;
  onHover?: () => void;
  onHoverEnd?: () => void;
  children?: ContextMenuItem[];
}

function renderContextMenuColor(color: string | undefined): React.ReactNode {
  if (!color) {
    return null;
  }
  return <span aria-hidden className="size-small shrink-0 rounded-sm border border-border" style={{ background: color }} />;
}

function renderContextMenuIcon(icon: ContextMenuItem["icon"], required = false): React.ReactNode {
  const resolved = icon ?? (required ? "circle-dot" : undefined);
  if (!resolved) {
    return null;
  }
  if (React.isValidElement(resolved)) return resolved;
  return <Icon icon={resolved as IconSource} size="small" className="shrink-0" />;
}

function renderContextMenuLeading(item: ContextMenuItem): React.ReactNode {
  const requiredIcon = !item.separator;
  return (
    <>
      {renderContextMenuColor(item.color)}
      {requiredIcon ? (
        <span data-slot="context-menu-icon" className="flex size-small shrink-0 items-center justify-center">
          {renderContextMenuIcon(item.icon, true)}
        </span>
      ) : (
        renderContextMenuIcon(item.icon)
      )}
    </>
  );
}

type DOMListenerTarget = Pick<EventTarget, "addEventListener" | "removeEventListener">;

export function createDOMEventBinding() {
  const cleanups: Array<() => void> = [];
  return {
    listen<E extends Event>(target: DOMListenerTarget | null | undefined, type: string, listener: (event: E) => void, options?: boolean | AddEventListenerOptions) {
      if (!target) return;
      const wrapped = listener as EventListener;
      target.addEventListener(type, wrapped, options);
      cleanups.push(() => target.removeEventListener(type, wrapped, options));
    },
    dispose() {
      while (cleanups.length > 0) cleanups.pop()?.();
    },
  };
}

function getDocumentBody(): HTMLElement | null {
  return typeof document === "undefined" ? null : document.body;
}

export function getElementById<T extends HTMLElement = HTMLElement>(id: string): T | null {
  return typeof document === "undefined" ? null : (document.getElementById(id) as T | null);
}

export function queryElement<T extends Element = HTMLElement>(selector: string, root?: ParentNode | null): T | null {
  return (root ?? (typeof document === "undefined" ? null : document))?.querySelector(selector) as T | null;
}

function renderPortalInto(children: React.ReactNode, container: Element | DocumentFragment | null | undefined): React.ReactNode {
  return container ? createPortal(children, container) : null;
}

export interface ContextMenuProps {
  items?: readonly ContextMenuItem[];
  children: React.ReactNode;
  /** @emoji 🪟️ Title chip on the window-chrome cap row. */
  title: UiLabel;
  /** @emoji 🪟️ Catalog icon shown in the title chip before the title. */
  titleIcon?: IconSource;
}

/**
 * 🧩️ Right-click host: always suppresses the native menu; opens the shared viewport-fixed menu only when `items` is non-empty.
 **/
export const ContextMenu: React.FC<ContextMenuProps> = ({ items, children, title, titleIcon = "list" }) => {
  const [open, setOpen] = reactHostPort.useState(false);
  const [point, setPoint] = reactHostPort.useState<{ x: number; y: number } | null>(null);
  const hasItems = !!items?.length;
  const host = (
    <div
      className="contents"
      onContextMenu={(event) => {
        event.preventDefault();
        if (!hasItems) {
          return;
        }
        event.stopPropagation();
        setPoint({ x: event.clientX, y: event.clientY });
        setOpen(true);
      }}
    >
      {children}
    </div>
  );
  if (!hasItems) {
    return host;
  }
  return (
    <>
      {host}
      <ContextMenuController open={open} position={point} items={items} onOpenChange={setOpen} title={title} titleIcon={titleIcon} />
    </>
  );
};

export interface ContextMenuControllerProps {
  open: boolean;
  position: { x: number; y: number } | null;
  items: readonly ContextMenuItem[];
  onOpenChange: (open: boolean) => void;
  /** @emoji 🪟️ Title chip on the window-chrome cap row. */
  title: UiLabel;
  /** @emoji 🪟️ Catalog icon in the title chip. */
  titleIcon?: IconSource;
  /** 🖱️ When false, selecting a row does not dismiss — the row action owns closing (e.g. acceptSuggestion). Outside pointer / Escape still dismiss. Default true. */
  closeOnSelect?: boolean;
}

/** @emoji ⌨️ Keyboard navigation direction parsed from a keydown key. */
export type ContextMenuNavDirection = "up" | "down" | "left" | "right" | "activate" | "escape";

/** @emoji 🔢️ Maps each enabled row id to its 1-based ordinal within `items` (separators skipped). */
export function contextMenuOrdinals(items: readonly ContextMenuItem[]): ReadonlyMap<string, number> {
  const map = new Map<string, number>();
  let ordinal = 0;
  for (const item of items) {
    if (item.separator || item.disabled) {
      continue;
    }
    ordinal += 1;
    map.set(item.id, ordinal);
  }
  return map;
}

function contextMenuEnabledIndices(items: readonly ContextMenuItem[]): number[] {
  return items.flatMap((item, index) => (!item.separator && !item.disabled ? [index] : []));
}

/** @emoji 📂️ Resolves the item list at `pathPrefix` (empty = top level). */
export function contextMenuItemsAtLevel(root: readonly ContextMenuItem[], pathPrefix: readonly number[]): readonly ContextMenuItem[] {
  let level = root;
  for (const index of pathPrefix) {
    const row = level[index];
    if (!row?.children?.length) {
      return level;
    }
    level = row.children;
  }
  return level;
}

/** @emoji 📍️ Resolves the row at `path` (empty path → undefined). */
export function contextMenuItemAtPath(root: readonly ContextMenuItem[], path: readonly number[]): ContextMenuItem | undefined {
  if (path.length === 0) {
    return undefined;
  }
  let level = root;
  let item: ContextMenuItem | undefined;
  for (let depth = 0; depth < path.length; depth += 1) {
    item = level[path[depth]!];
    if (!item) {
      return undefined;
    }
    if (depth < path.length - 1) {
      level = item.children ?? [];
    }
  }
  return item;
}

/** @emoji ✅️ Path to the first enabled `checked` row, if any. */
export function findContextMenuCheckedPath(root: readonly ContextMenuItem[], prefix: readonly number[] = []): number[] | undefined {
  for (let index = 0; index < root.length; index += 1) {
    const item = root[index]!;
    if (item.separator || item.disabled) {
      continue;
    }
    const path = [...prefix, index];
    if (item.checked) {
      return path;
    }
    if (item.children?.length) {
      const nested = findContextMenuCheckedPath(item.children, path);
      if (nested) {
        return nested;
      }
    }
  }
  return undefined;
}

/** @emoji ⌨️ Maps arrows, wasd, Enter, Space, and Escape to menu navigation. */
export function contextMenuNavigationFromKey(key: string): ContextMenuNavDirection | undefined {
  switch (key) {
    case "ArrowUp":
    case "w":
    case "W":
      return "up";
    case "ArrowDown":
    case "s":
    case "S":
      return "down";
    case "ArrowLeft":
    case "a":
    case "A":
      return "left";
    case "ArrowRight":
    case "d":
    case "D":
      return "right";
    case "Enter":
    case " ":
      return "activate";
    case "Escape":
      return "escape";
    default:
      return undefined;
  }
}

/** @emoji ⌨️ Next active path when moving up or down within the current menu level. */
export function moveContextMenuActivePath(root: readonly ContextMenuItem[], path: readonly number[], direction: "up" | "down"): number[] {
  const levelPrefix = path.length > 0 ? path.slice(0, -1) : [];
  const level = contextMenuItemsAtLevel(root, levelPrefix);
  const enabled = contextMenuEnabledIndices(level);
  if (enabled.length === 0) {
    return [...path];
  }
  const currentIndex = path.length > 0 ? path[path.length - 1]! : -1;
  const position = enabled.indexOf(currentIndex);
  const nextPosition =
    position === -1
      ? direction === "down"
        ? 0
        : enabled.length - 1
      : direction === "down"
        ? (position + 1) % enabled.length
        : (position - 1 + enabled.length) % enabled.length;
  return [...levelPrefix, enabled[nextPosition]!];
}

/** @emoji 🔢️ Active path for digit `ordinal` (1–9) within the level of `path`. */
export function contextMenuPathForOrdinal(root: readonly ContextMenuItem[], path: readonly number[], ordinal: number): number[] | undefined {
  const levelPrefix = path.length > 0 ? path.slice(0, -1) : [];
  const level = contextMenuItemsAtLevel(root, levelPrefix);
  let seen = 0;
  for (let index = 0; index < level.length; index += 1) {
    const item = level[index]!;
    if (item.separator || item.disabled) {
      continue;
    }
    seen += 1;
    if (seen === ordinal) {
      return [...levelPrefix, index];
    }
  }
  return undefined;
}

/** @emoji 📂️ Opens the submenu under `path` and selects its first enabled child. */
export function contextMenuOpenSubmenuPath(root: readonly ContextMenuItem[], path: readonly number[]): number[] | undefined {
  const item = contextMenuItemAtPath(root, path);
  if (!item?.children?.length) {
    return undefined;
  }
  const enabled = contextMenuEnabledIndices(item.children);
  if (enabled.length === 0) {
    return [...path];
  }
  return [...path, enabled[0]!];
}

function contextMenuHoverItem(item: ContextMenuItem | undefined): void {
  item?.onHover?.();
}

function contextMenuHoverEndItem(item: ContextMenuItem | undefined): void {
  item?.onHoverEnd?.();
}

function isContextMenuEditableKeyTarget(target: EventTarget | null): boolean {
  if (!(target instanceof HTMLElement)) {
    return false;
  }
  const tag = target.tagName;
  return tag === "INPUT" || tag === "TEXTAREA" || target.isContentEditable;
}

function renderContextMenuOrdinalBadge(ordinal: number | undefined): React.ReactNode {
  if (ordinal === undefined || ordinal > 9) {
    return <span aria-hidden className={contextMenuOrdinalClassName} />;
  }
  return (
    <span aria-hidden className={contextMenuOrdinalClassName}>
      {ordinal}
    </span>
  );
}

type FixedContextMenuRenderOptions = {
  readonly rootItems: readonly ContextMenuItem[];
  readonly activePath: readonly number[];
  readonly submenuCollapsedAt: readonly number[] | null;
  readonly setActivePath: (path: number[], collapseSubmenuAt?: readonly number[] | null) => void;
  readonly onClose: () => void;
};

function contextMenuPathsEqual(a: readonly number[], b: readonly number[]): boolean {
  return a.length === b.length && a.every((value, index) => value === b[index]);
}

function isContextMenuSubmenuOpen(activePath: readonly number[], parentPath: readonly number[]): boolean {
  if (activePath.length <= parentPath.length) {
    return false;
  }
  return parentPath.every((value, index) => activePath[index] === value);
}

/** @emoji ⏱️ Delay before hovering a parent row opens its submenu, so a pointer merely passing over the row doesn't flash it open. */
const CONTEXT_MENU_SUBMENU_HOVER_DELAY_MS = 150;

type ContextMenuSubmenuRowProps = {
  readonly item: ContextMenuItem;
  readonly rowPath: readonly number[];
  readonly ordinal: number | undefined;
  readonly isActive: boolean;
  readonly submenuOpen: boolean;
  readonly setActivePath: FixedContextMenuRenderOptions["setActivePath"];
  readonly children: React.ReactNode;
};

/** @emoji 📂️ Parent-row button for a submenu: click toggles it open/closed, hover opens it after a short delay, and the panel flips to the opposite side when it would overflow the viewport. */
function ContextMenuSubmenuRow({ item, rowPath, ordinal, isActive, submenuOpen, setActivePath, children }: ContextMenuSubmenuRowProps): React.ReactElement {
  const hoverTimerRef = reactHostPort.useRef<number | undefined>(undefined);
  const panelRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const [flipToEnd, setFlipToEnd] = reactHostPort.useState(false);
  const clearHoverTimer = reactHostPort.useCallback((): void => {
    if (hoverTimerRef.current !== undefined) {
      window.clearTimeout(hoverTimerRef.current);
      hoverTimerRef.current = undefined;
    }
  }, []);
  reactHostPort.useEffect(() => clearHoverTimer, [clearHoverTimer]);
  reactHostPort.useLayoutEffect(() => {
    if (!submenuOpen) {
      setFlipToEnd(false);
      return;
    }
    const node = panelRef.current;
    if (!node) return;
    setFlipToEnd(node.getBoundingClientRect().right > window.innerWidth);
  }, [submenuOpen]);
  const toggleSubmenu = (): void => {
    if (item.disabled) return;
    setActivePath([...rowPath], submenuOpen ? [...rowPath] : null);
  };
  return (
    <div
      className="relative"
      onPointerEnter={() => {
        clearHoverTimer();
        hoverTimerRef.current = window.setTimeout(() => {
          hoverTimerRef.current = undefined;
          setActivePath([...rowPath], null);
        }, CONTEXT_MENU_SUBMENU_HOVER_DELAY_MS);
      }}
      onPointerLeave={clearHoverTimer}
    >
      <button
        aria-disabled={item.disabled}
        aria-expanded={submenuOpen}
        className={contextMenuItemClassName(item)}
        data-active={isActive ? "true" : undefined}
        data-disabled={item.disabled ? "" : undefined}
        data-selected={item.checked ? "true" : undefined}
        disabled={item.disabled}
        onClick={toggleSubmenu}
        onPointerEnter={() => item.onHover?.()}
        onPointerLeave={() => item.onHoverEnd?.()}
        role="menuitem"
        type="button"
      >
        {renderContextMenuOrdinalBadge(ordinal)}
        {renderContextMenuLeading(item)}
        <span className="truncate">{item.label ?? item.id}</span>
        {item.shortcut ? (
          <span aria-hidden className={contextMenuShortcutClassName}>
            {item.shortcut}
          </span>
        ) : null}
        <span aria-hidden className={contextMenuShortcutClassName}>
          ›
        </span>
      </button>
      {submenuOpen ? (
        <div ref={panelRef} className={cn("absolute top-0 ms-tiny", flipToEnd ? "end-full" : "start-full")}>
          {children}
        </div>
      ) : null}
    </div>
  );
}

function renderFixedContextMenuItems(items: readonly ContextMenuItem[], pathPrefix: readonly number[], options: FixedContextMenuRenderOptions): React.ReactNode {
  const ordinals = contextMenuOrdinals(items);
  const { activePath, submenuCollapsedAt, setActivePath, onClose } = options;
  return items.map((item, index) => {
    const rowPath = [...pathPrefix, index];
    if (item.separator) {
      if (item.label) {
        return (
          <div key={`${item.id}-sep`} className="select-none px-single pb-half pt-single text-xs text-muted-foreground" role="separator" aria-label={item.label}>
            {item.label}
          </div>
        );
      }
      return <div key={`${item.id}-sep`} className="h-px bg-border my-single" role="separator" />;
    }
    const ordinal = ordinals.get(item.id);
    const isActive = contextMenuPathsEqual(activePath, rowPath);
    const activateRow = (): void => {
      setActivePath(rowPath, null);
    };
    if (item.children?.length) {
      const submenuOpen =
        isContextMenuSubmenuOpen(activePath, rowPath) ||
        (isActive && !(submenuCollapsedAt && contextMenuPathsEqual(submenuCollapsedAt, rowPath)));
      return (
        <ContextMenuSubmenuRow key={item.id} item={item} rowPath={rowPath} ordinal={ordinal} isActive={isActive} submenuOpen={submenuOpen} setActivePath={setActivePath}>
          <ContextMenuChrome title={item.label ?? item.id} icon={(item.icon ?? "folder") as IconSource}>
            {renderFixedContextMenuItems(item.children, rowPath, options)}
          </ContextMenuChrome>
        </ContextMenuSubmenuRow>
      );
    }
    const role = item.checked === undefined ? "menuitem" : "menuitemcheckbox";
    return (
      <button
        key={item.id}
        aria-checked={item.checked}
        aria-disabled={item.disabled}
        className={contextMenuItemClassName(item)}
        data-active={isActive ? "true" : undefined}
        data-disabled={item.disabled ? "" : undefined}
        data-selected={item.checked ? "true" : undefined}
        disabled={item.disabled}
        onClick={(event) => {
          item.onSelect?.(event.nativeEvent);
          onClose();
        }}
        onPointerEnter={() => {
          activateRow();
          item.onHover?.();
        }}
        onPointerLeave={() => item.onHoverEnd?.()}
        role={role}
        type="button"
      >
        {renderContextMenuOrdinalBadge(ordinal)}
        {renderContextMenuLeading(item)}
        <span className="truncate">{item.label ?? item.id}</span>
        {item.shortcut ? (
          <span aria-hidden className={contextMenuShortcutClassName}>
            {item.shortcut}
          </span>
        ) : null}
      </button>
    );
  });
}

/** @emoji 🖱️ True when a pointer event targets any open context menu surface (including sibling menus from split world panes). */
export function isContextMenuPointerTarget(target: EventTarget | null): boolean {
  return Boolean(target instanceof Element && target.closest('[role="menu"]'));
}

/** @emoji ⌨️ Maps a keydown key to a context-menu digit shortcut (`1`–`9`), if any. */
export function contextMenuDigitFromKey(key: string): string | undefined {
  return key.length === 1 && key >= "1" && key <= "9" ? key : undefined;
}

/** @emoji ⌨️ Finds the first enabled top-level row marked `checked`. */
export function findCheckedContextMenuItem(items: readonly ContextMenuItem[]): ContextMenuItem | undefined {
  const path = findContextMenuCheckedPath(items);
  return path ? contextMenuItemAtPath(items, path) : undefined;
}

/**
 * 🧩️ Controlled right-click menu whose title chip bottom-left anchors at viewport coordinates (puzzle 2d canvas bridge), keeping the first row beside the pointer. Portals to `document.body` for correct `fixed` placement under transformed UI; outside-dismiss uses `window` bubble listeners so they run after the puzzle 2d `eventSurface` bubble path and after `window` capture (441–442 used `document` capture and swallowed input).
 **/
export const ContextMenuController: React.FC<ContextMenuControllerProps> = ({ open, position, items, onOpenChange, title, titleIcon = "list", closeOnSelect = true }) => {
  const close = reactHostPort.useCallback(() => onOpenChange(false), [onOpenChange]);
  // 🐚️ Falls back to `document.body` outside any shell — inside one, portals into that shell's own
  // overlay layer so a context menu never visually escapes into another mounted shell's stacking context.
  const shellScope = useShellScopeOptional();
  const flow = useFlow();
  const menuRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const chromeRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const itemsRef = reactHostPort.useRef(items);
  itemsRef.current = items;
  const [activePath, setActivePath] = reactHostPort.useState<number[]>([]);
  const [submenuCollapsedAt, setSubmenuCollapsedAt] = reactHostPort.useState<readonly number[] | null>(null);
  const activePathRef = reactHostPort.useRef(activePath);
  activePathRef.current = activePath;
  const previousHoverItemRef = reactHostPort.useRef<ContextMenuItem | undefined>(undefined);
  reactHostPort.useEffect(() => {
    if (!open) {
      setActivePath([]);
      setSubmenuCollapsedAt(null);
      previousHoverItemRef.current = undefined;
      return;
    }
    const initial = findContextMenuCheckedPath(items) ?? [];
    setActivePath(initial);
    setSubmenuCollapsedAt(null);
    previousHoverItemRef.current = initial.length ? contextMenuItemAtPath(items, initial) : undefined;
  }, [open, items]);
  const applyActivePath = reactHostPort.useCallback((nextPath: number[], collapseSubmenuAt: readonly number[] | null = null) => {
    const root = itemsRef.current;
    const previous = previousHoverItemRef.current;
    const nextItem = nextPath.length ? contextMenuItemAtPath(root, nextPath) : undefined;
    if (previous !== nextItem) {
      contextMenuHoverEndItem(previous);
      contextMenuHoverItem(nextItem);
      previousHoverItemRef.current = nextItem;
    }
    setSubmenuCollapsedAt(collapseSubmenuAt);
    setActivePath(nextPath);
  }, []);
  reactHostPort.useEffect(() => {
    if (!open || !items.length || !position) {
      return undefined;
    }
    let armed = false;
    const armTimer = window.setTimeout(() => {
      armed = true;
    }, 0);
    const handlePointerDown = (event: PointerEvent): void => {
      if (!armed) return;
      if (isContextMenuPointerTarget(event.target)) return;
      onOpenChange(false);
    };
    const handleKeyDown = (event: KeyboardEvent): void => {
      if (isContextMenuEditableKeyTarget(event.target)) {
        return;
      }
      const root = itemsRef.current;
      const path = activePathRef.current;
      const direction = contextMenuNavigationFromKey(event.key);
      if (direction === "escape") {
        event.preventDefault();
        event.stopPropagation();
        if (path.length > 1) {
          applyActivePath(path.slice(0, -1), path.slice(0, -1));
          return;
        }
        onOpenChange(false);
        return;
      }
      const digit = contextMenuDigitFromKey(event.key);
      if (digit) {
        const ordinal = Number(digit);
        const nextPath = contextMenuPathForOrdinal(root, path, ordinal);
        if (!nextPath) {
          return;
        }
        event.preventDefault();
        event.stopPropagation();
        applyActivePath(nextPath);
        return;
      }
      if (!direction) {
        return;
      }
      event.preventDefault();
      event.stopPropagation();
      if (direction === "up" || direction === "down") {
        applyActivePath(moveContextMenuActivePath(root, path, direction));
        return;
      }
      if (direction === "left") {
        if (path.length > 1) {
          applyActivePath(path.slice(0, -1), path.slice(0, -1));
        }
        return;
      }
      if (direction === "right") {
        const opened = contextMenuOpenSubmenuPath(root, path);
        if (opened) {
          applyActivePath(opened);
        }
        return;
      }
      if (direction === "activate") {
        const activeItem = path.length ? contextMenuItemAtPath(root, path) : undefined;
        if (!activeItem || activeItem.disabled) {
          return;
        }
        if (activeItem.children?.length) {
          const opened = contextMenuOpenSubmenuPath(root, path);
          if (opened) {
            applyActivePath(opened);
          }
          return;
        }
        activeItem.onSelect?.(new Event("select"));
        if (closeOnSelect) {
          close();
        }
      }
    };
    const bindings = createDOMEventBinding();
    bindings.listen(window, "pointerdown", handlePointerDown, false);
    bindings.listen(window, "keydown", handleKeyDown, false);
    return () => {
      window.clearTimeout(armTimer);
      bindings.dispose();
    };
  }, [applyActivePath, close, closeOnSelect, items.length, onOpenChange, open, position?.x, position?.y]);
  // 🖥️ Clamp the rendered menu surface fully on-screen — a right-click near a viewport edge would otherwise open a
  // menu that spills past it.
  reactHostPort.useLayoutEffect(() => {
    if (!open || !position) {
      return;
    }
    const node = chromeRef.current;
    if (!node) {
      return;
    }
    const rect = node.getBoundingClientRect();
    const maxLeft = Math.max(0, window.innerWidth - rect.width);
    const maxTop = Math.max(0, window.innerHeight - rect.height);
    const clampedLeft = Math.min(Math.max(rect.left, 0), maxLeft);
    const clampedTop = Math.min(Math.max(rect.top, 0), maxTop);
    if (clampedLeft !== rect.left) {
      node.style.left = `${clampedLeft}px`;
    }
    if (clampedTop !== rect.top) {
      node.style.top = `${clampedTop}px`;
    }
  }, [open, position?.x, position?.y, items]);
  if (!items.length) {
    return null;
  }
  if (!open || !position) {
    return null;
  }
  const renderOptions: FixedContextMenuRenderOptions = {
    rootItems: items,
    activePath,
    submenuCollapsedAt,
    setActivePath: applyActivePath,
    onClose: closeOnSelect ? close : () => undefined,
  };
  return renderPortalInto(
    <ContextMenuChrome ref={chromeRef} style={{ left: position.x, position: "fixed", top: `calc(${position.y}px - var(--size-medium))` }} title={title} icon={titleIcon}>
      <div dir={flow.inline === "rtl" ? "rtl" : undefined} onContextMenu={(event) => event.preventDefault()} ref={menuRef} role="menu">
        {renderFixedContextMenuItems(items, [], renderOptions)}
      </div>
    </ContextMenuChrome>,
    shellScope?.portalLayerRef.current ?? getDocumentBody(),
  );
};

/** @emoji 📋️ Non-collapsed DOM text selection string, or empty. */
export function readDomTextSelection(): string {
  if (typeof window === "undefined") return "";
  const selection = window.getSelection();
  if (!selection || selection.isCollapsed || selection.rangeCount < 1) return "";
  return selection.toString();
}

/** @emoji 📋️ True when `target` intersects the current non-empty DOM text selection. */
export function isPointerEventOnDomTextSelection(target: EventTarget | null): boolean {
  if (typeof window === "undefined") return false;
  if (!(target instanceof Node)) return false;
  const selection = window.getSelection();
  if (!selection || selection.isCollapsed || selection.rangeCount < 1) return false;
  if (!selection.toString()) return false;
  try {
    return selection.getRangeAt(0).intersectsNode(target);
  } catch {
    return false;
  }
}

/** @emoji ✏️ True when the event target (or its editable ancestor) accepts cut/paste. */
export function isDomTextEditableTarget(target: EventTarget | null): boolean {
  if (target instanceof Text) return isDomTextEditableTarget(target.parentElement);
  if (!(target instanceof Element)) return false;
  if (target instanceof HTMLTextAreaElement) return !target.readOnly && !target.disabled;
  if (target instanceof HTMLInputElement) {
    const kind = (target.type || "text").toLowerCase();
    if (kind === "button" || kind === "checkbox" || kind === "radio" || kind === "file" || kind === "range" || kind === "color" || kind === "hidden") return false;
    return !target.readOnly && !target.disabled;
  }
  if ((target as HTMLElement).isContentEditable) return true;
  return Boolean(target.closest('[contenteditable="true"], [role="textbox"]'));
}

export interface TextSelectionContextMenuLabels {
  readonly cut: UiLabel;
  readonly copy: UiLabel;
  readonly paste: UiLabel;
  readonly selectAll: UiLabel;
}

export interface TextSelectionContextMenuActions {
  readonly cut: () => void;
  readonly copy: () => void;
  readonly paste: () => void;
  readonly selectAll: () => void;
}

/** @emoji 📋️ Builds Cut/Copy/Paste/Select All rows for a DOM text selection. */
export function buildTextSelectionContextMenuItems(input: { readonly editable: boolean; readonly hasSelection: boolean }, labels: TextSelectionContextMenuLabels, actions: TextSelectionContextMenuActions): ContextMenuItem[] {
  const items: ContextMenuItem[] = [];
  if (input.editable) {
    items.push({ id: "text-cut", label: labels.cut, icon: "scissors", shortcut: formatKeybindingShortcut("mod+x"), disabled: !input.hasSelection, onSelect: () => actions.cut() });
  }
  items.push({ id: "text-copy", label: labels.copy, icon: "copy", shortcut: formatKeybindingShortcut("mod+c"), disabled: !input.hasSelection, onSelect: () => actions.copy() });
  if (input.editable) {
    items.push({ id: "text-paste", label: labels.paste, icon: "clipboard", shortcut: formatKeybindingShortcut("mod+v"), onSelect: () => actions.paste() });
  }
  items.push({ id: "text-sep", label: uiDataLabel(""), separator: true });
  items.push({ id: "text-select-all", label: labels.selectAll, icon: "select-all", shortcut: formatKeybindingShortcut("mod+a"), onSelect: () => actions.selectAll() });
  return items;
}

/** @emoji 📋️ Copies the current DOM text selection to the clipboard. */
export async function copyDomTextSelection(): Promise<void> {
  const text = readDomTextSelection();
  if (!text || typeof navigator === "undefined" || !navigator.clipboard?.writeText) return;
  await navigator.clipboard.writeText(text);
}

/** @emoji ✂️ Cuts the current DOM text selection when the focus target is editable. */
export async function cutDomTextSelection(target: EventTarget | null): Promise<void> {
  await copyDomTextSelection();
  if (!isDomTextEditableTarget(target)) return;
  if (target instanceof HTMLInputElement || target instanceof HTMLTextAreaElement) {
    const start = target.selectionStart ?? 0;
    const end = target.selectionEnd ?? 0;
    if (start === end) return;
    const value = target.value;
    target.value = `${value.slice(0, start)}${value.slice(end)}`;
    target.setSelectionRange(start, start);
    target.dispatchEvent(new Event("input", { bubbles: true }));
    return;
  }
  if (typeof document !== "undefined" && typeof document.execCommand === "function") {
    document.execCommand("delete");
    return;
  }
  window.getSelection()?.deleteFromDocument();
}

/** @emoji 📋️ Pastes clipboard text into the editable target (or active element). */
export async function pasteDomTextSelection(target: EventTarget | null): Promise<void> {
  if (typeof navigator === "undefined" || !navigator.clipboard?.readText) return;
  const text = await navigator.clipboard.readText();
  const focus = (target instanceof Element ? target : null) ?? (typeof document !== "undefined" ? document.activeElement : null);
  if (focus instanceof HTMLInputElement || focus instanceof HTMLTextAreaElement) {
    if (focus.readOnly || focus.disabled) return;
    const start = focus.selectionStart ?? focus.value.length;
    const end = focus.selectionEnd ?? focus.value.length;
    focus.value = `${focus.value.slice(0, start)}${text}${focus.value.slice(end)}`;
    const caret = start + text.length;
    focus.setSelectionRange(caret, caret);
    focus.dispatchEvent(new Event("input", { bubbles: true }));
    return;
  }
  if (typeof document !== "undefined" && typeof document.execCommand === "function") {
    document.execCommand("insertText", false, text);
  }
}

/** @emoji 🅰️ Selects all text in the editable focus target, or the current selection's root element. */
export function selectAllDomText(target: EventTarget | null): void {
  const focus = (target instanceof Element ? target : target instanceof Text ? target.parentElement : null) ?? (typeof document !== "undefined" ? document.activeElement : null);
  if (focus instanceof HTMLInputElement || focus instanceof HTMLTextAreaElement) {
    focus.select();
    return;
  }
  if (typeof window === "undefined") return;
  const selection = window.getSelection();
  if (!selection) return;
  if (focus instanceof HTMLElement && focus.isContentEditable) {
    const range = document.createRange();
    range.selectNodeContents(focus);
    selection.removeAllRanges();
    selection.addRange(range);
    return;
  }
  if (selection.rangeCount < 1) return;
  const ancestor = selection.getRangeAt(0).commonAncestorContainer;
  const element = ancestor instanceof Element ? ancestor : ancestor.parentElement;
  if (!element) return;
  const range = document.createRange();
  range.selectNodeContents(element);
  selection.removeAllRanges();
  selection.addRange(range);
}

/**
 * 📋️ Global host: right-click on a DOM text selection opens Cut/Copy/Paste/Select All (native menus are suppressed by {@link installElementsSurfaceBrowserDefaultSuppression}).
 **/
export const TextSelectionContextMenuHost: React.FC = () => {
  const contextMenuTitle = useLabel("ui.common.actions");
  const cutLabel = useLabel("ui.contextMenu.cut");
  const copyLabel = useLabel("ui.contextMenu.copy");
  const pasteLabel = useLabel("ui.contextMenu.paste");
  const selectAllLabel = useLabel("ui.contextMenu.selectAll");
  const [open, setOpen] = reactHostPort.useState(false);
  const [position, setPosition] = reactHostPort.useState<{ x: number; y: number } | null>(null);
  const [items, setItems] = reactHostPort.useState<readonly ContextMenuItem[]>([]);
  const targetRef = reactHostPort.useRef<EventTarget | null>(null);
  reactHostPort.useEffect(() => {
    if (typeof document === "undefined") return undefined;
    const onContextMenu = (event: MouseEvent): void => {
      if (!isPointerEventOnDomTextSelection(event.target)) return;
      if (isContextMenuPointerTarget(event.target)) return;
      event.preventDefault();
      event.stopPropagation();
      targetRef.current = event.target;
      const editable = isDomTextEditableTarget(event.target);
      const hasSelection = Boolean(readDomTextSelection());
      setItems(
        buildTextSelectionContextMenuItems(
          { editable, hasSelection },
          { cut: cutLabel, copy: copyLabel, paste: pasteLabel, selectAll: selectAllLabel },
          {
            cut: () => {
              void cutDomTextSelection(targetRef.current);
            },
            copy: () => {
              void copyDomTextSelection();
            },
            paste: () => {
              void pasteDomTextSelection(targetRef.current);
            },
            selectAll: () => {
              selectAllDomText(targetRef.current);
            },
          },
        ),
      );
      setPosition({ x: event.clientX, y: event.clientY });
      setOpen(true);
    };
    document.addEventListener("contextmenu", onContextMenu, true);
    return () => document.removeEventListener("contextmenu", onContextMenu, true);
  }, [cutLabel, copyLabel, pasteLabel, selectAllLabel]);
  return <ContextMenuController open={open} position={position} items={items} onOpenChange={setOpen} title={contextMenuTitle} />;
};

// #endregion 🖱️ContextMenu
