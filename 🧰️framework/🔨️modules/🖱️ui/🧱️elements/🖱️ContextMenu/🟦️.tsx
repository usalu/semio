// #region 🧲️Header
// 💻️ framework/ui/elements/🖱️ContextMenu/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

import * as React from "react";
import { reactHostPort } from "../🔌️Ports/⚛️react/🟦️.ts";
import type { UiLabel } from "../🎗️UiLabel/🟦️.tsx";
import type { IconSource } from "../🔣️Icons/🟦️.tsx";
import { useLabel } from "../🏷️Label/🟦️.tsx";
import { useShellFloatingSurfaceHost } from "../🐚️ShellScope/🟦️.tsx";
import { useFlow } from "../../🔨️modules/🧭️flow-direction-context/🟦️.tsx";
import { ContextMenuChrome, ContextMenuWingFrame } from "./🥣️chrome/🟦️.tsx";
import { createDOMEventBinding } from "../../🔨️modules/👂️dom-event-binding/🟦️.ts";
import { type ContextMenuItem, renderPortalInto } from "./🎨️item-render/🟦️.tsx";
import { contextMenuHoverEndItem, contextMenuHoverItem, contextMenuItemAtPath, contextMenuNavigationFromKey, contextMenuOpenSubmenuPath, contextMenuPathForOrdinal, contextMenuPathKey, findContextMenuCheckedPath, isContextMenuEditableKeyTarget, moveContextMenuActivePath } from "./🧭️navigation/🟦️.tsx";
import { CONTEXT_MENU_NO_WINGS, type ContextMenuWingPlacement, type FixedContextMenuRenderOptions, contextMenuSubmenuPlacement, contextMenuWingPlacementsEqual, contextMenuOpenWings, renderFixedContextMenuItems } from "./🪜️submenu/🟦️.tsx";
import { buildTextSelectionContextMenuItems, copyDomTextSelection, cutDomTextSelection, isDomTextEditableTarget, isPointerEventOnDomTextSelection, pasteDomTextSelection, readDomTextSelection, selectAllDomText } from "./📝️text-selection/🟦️.ts";

export { ContextMenuTriggerButton, contextMenuAnchorPointV1, dispatchContextMenuAtV1, type ContextMenuTriggerButtonProps } from "./🔘️trigger/🟦️.tsx";
export { createDOMEventBinding, type DOMListenerTarget } from "../../🔨️modules/👂️dom-event-binding/🟦️.ts";

export interface ContextMenuProps {
  items?: readonly ContextMenuItem[];
  children: React.ReactNode;
  /** 🪟️ Title chip on the window-chrome cap row. */
  title: UiLabel;
  /** 🖼️ Catalog icon shown in the title chip before the title. */
  titleIcon?: IconSource;
}

/**
 * 👆️ Right-click host: always suppresses the native menu; opens the shared viewport-fixed menu only when `items` is non-empty.
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
  /** 🏷️ Title chip on the window-chrome cap row. */
  title: UiLabel;
  /** 📚️ Catalog icon in the title chip. */
  titleIcon?: IconSource;
  /** ☑️ When false, selecting a row does not dismiss — the row action owns closing (e.g. acceptSuggestion). Outside pointer / Escape still dismiss. Default true. */
  closeOnSelect?: boolean;
}

/** 🖱️ True when a pointer event targets any open context menu surface (including sibling menus from split world panes). */
export function isContextMenuPointerTarget(target: EventTarget | null): boolean {
  return Boolean(target instanceof Element && target.closest('[role="menu"]'));
}

/** ⌨️ Maps a keydown key to a context-menu digit shortcut (`1`–`9`), if any. */
export function contextMenuDigitFromKey(key: string): string | undefined {
  return key.length === 1 && key >= "1" && key <= "9" ? key : undefined;
}

/** 🔎️ Finds the first enabled top-level row marked `checked`. */
export function findCheckedContextMenuItem(items: readonly ContextMenuItem[]): ContextMenuItem | undefined {
  const path = findContextMenuCheckedPath(items);
  return path ? contextMenuItemAtPath(items, path) : undefined;
}

/**
 * 🧩️ Controlled right-click menu whose title chip bottom-left anchors at viewport coordinates (puzzle 2d canvas bridge), keeping the first row beside the pointer. Portals to the owning shell's floating layer (or `document.body` standalone) for unclipped `fixed` placement; outside-dismiss uses a `window` capture listener (same as {@link CanvasPickMenu}) so a canvas `stopImmediatePropagation` on pointer-down cannot swallow the bubble phase and leave the menu stuck open.
 **/
export const ContextMenuController: React.FC<ContextMenuControllerProps> = ({ open, position, items, onOpenChange, title, titleIcon = "list", closeOnSelect = true }) => {
  const close = reactHostPort.useCallback(() => onOpenChange(false), [onOpenChange]);
  const floatingHost = useShellFloatingSurfaceHost();
  const flow = useFlow();
  const menuRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const chromeRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const [wingPlacements, setWingPlacements] = reactHostPort.useState<ReadonlyMap<string, ContextMenuWingPlacement>>(() => new Map());
  const itemsRef = reactHostPort.useRef(items);
  itemsRef.current = items;
  const [activePath, setActivePath] = reactHostPort.useState<number[]>([]);
  const [submenuCollapsedAt, setSubmenuCollapsedAt] = reactHostPort.useState<readonly number[] | null>(null);
  const activePathRef = reactHostPort.useRef(activePath);
  activePathRef.current = activePath;
  const previousHoverItemRef = reactHostPort.useRef<ContextMenuItem | undefined>(undefined);
  const rowNodesRef = reactHostPort.useRef<Map<string, HTMLButtonElement>>(new Map());
  const registerRow = reactHostPort.useCallback((pathKey: string, node: HTMLButtonElement | null): void => {
    if (node) {
      rowNodesRef.current.set(pathKey, node);
      return;
    }
    rowNodesRef.current.delete(pathKey);
  }, []);
  const focusReturnRef = reactHostPort.useRef<HTMLElement | null>(null);
  reactHostPort.useEffect(() => {
    if (!open) {
      return undefined;
    }
    focusReturnRef.current = typeof document === "undefined" ? null : (document.activeElement as HTMLElement | null);
    return () => {
      const previous = focusReturnRef.current;
      focusReturnRef.current = null;
      if (previous?.isConnected) previous.focus({ preventScroll: true });
    };
  }, [open]);
  reactHostPort.useLayoutEffect(() => {
    if (!open) {
      return;
    }
    const node = activePath.length ? rowNodesRef.current.get(contextMenuPathKey(activePath)) : undefined;
    (node ?? menuRef.current)?.focus({ preventScroll: true });
    const scroller = node?.closest<HTMLElement>("[data-context-menu-scroll]");
    if (!node || !scroller) return;
    const row = node.getBoundingClientRect();
    const viewport = scroller.getBoundingClientRect();
    if (row.top < viewport.top) scroller.scrollTop += row.top - viewport.top;
    else if (row.bottom > viewport.bottom) scroller.scrollTop += row.bottom - viewport.bottom;
    if (row.left < viewport.left) scroller.scrollLeft += row.left - viewport.left;
    else if (row.right > viewport.right) scroller.scrollLeft += row.right - viewport.right;
  }, [open, activePath]);
  const seededRef = reactHostPort.useRef(false);
  reactHostPort.useEffect(() => {
    if (!open) {
      if (seededRef.current) {
        setActivePath([]);
        setSubmenuCollapsedAt(null);
      }
      seededRef.current = false;
      previousHoverItemRef.current = undefined;
      return;
    }
    const current = activePathRef.current;
    const currentItem = current.length ? contextMenuItemAtPath(items, current) : undefined;
    if (seededRef.current && (current.length === 0 || currentItem?.id === previousHoverItemRef.current?.id)) {
      previousHoverItemRef.current = currentItem;
      return;
    }
    seededRef.current = true;
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
    bindings.listen(window, "pointerdown", handlePointerDown, true);
    bindings.listen(window, "keydown", handleKeyDown, false);
    return () => {
      window.clearTimeout(armTimer);
      bindings.dispose();
    };
  }, [applyActivePath, close, closeOnSelect, items.length, onOpenChange, open, position?.x, position?.y]);
  const wings = reactHostPort.useMemo(() => (open ? contextMenuOpenWings(items, activePath, submenuCollapsedAt) : CONTEXT_MENU_NO_WINGS), [open, activePath, items, submenuCollapsedAt]);
  reactHostPort.useLayoutEffect(() => {
    const chrome = chromeRef.current;
    if (!open || !position || !chrome) return;
    const frames = wings.map((wing) => {
      const key = contextMenuPathKey(wing.parentPath);
      return { key, node: document.querySelector<HTMLElement>(`[data-context-menu-submenu-of="${key}"]`) };
    });
    const measure = (): void => {
      chrome.style.left = `${position.x}px`;
      chrome.style.top = `calc(${position.y}px - var(--chip-row-height))`;
      const rect = chrome.getBoundingClientRect();
      if (rect.width > 0 && rect.height > 0) {
        chrome.style.left = `${Math.min(Math.max(rect.left, 0), Math.max(0, window.innerWidth - rect.width))}px`;
        chrome.style.top = `${Math.min(Math.max(rect.top, 0), Math.max(0, window.innerHeight - rect.height))}px`;
      }
      const viewport = { width: window.innerWidth, height: window.innerHeight };
      const next = new Map<string, ContextMenuWingPlacement>();
      let parentRect = chrome.getBoundingClientRect();
      for (const { key, node } of frames) {
        const anchor = rowNodesRef.current.get(key);
        if (!node || !anchor) continue;
        const own = node.getBoundingClientRect();
        const first = node.querySelector<HTMLElement>('[role="menuitem"], [role="menuitemcheckbox"]');
        const inset = first ? first.getBoundingClientRect().top - own.top : 0;
        const placed = contextMenuSubmenuPlacement({ anchor: { left: parentRect.left, right: parentRect.right, top: anchor.getBoundingClientRect().top - inset }, panel: { width: own.width, height: own.height }, viewport });
        node.style.left = `${placed.left}px`;
        node.style.top = `${placed.top}px`;
        next.set(key, { left: placed.left, top: placed.top });
        parentRect = node.getBoundingClientRect();
      }
      setWingPlacements((previous) => (contextMenuWingPlacementsEqual(previous, next) ? previous : next));
    };
    measure();
    const observer = typeof ResizeObserver === "undefined" ? null : new ResizeObserver(measure);
    observer?.observe(chrome);
    for (const { key, node } of frames) {
      if (node) observer?.observe(node);
      const anchor = rowNodesRef.current.get(key);
      if (anchor) observer?.observe(anchor);
    }
    window.addEventListener("resize", measure);
    window.addEventListener("scroll", measure, true);
    return () => {
      observer?.disconnect();
      window.removeEventListener("resize", measure);
      window.removeEventListener("scroll", measure, true);
    };
  }, [wings, open, position?.x, position?.y, floatingHost]);
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
    registerRow,
  };
  return renderPortalInto(
    <>
      <ContextMenuChrome ref={chromeRef} style={{ left: position.x, position: "fixed", top: `calc(${position.y}px - var(--chip-row-height))` }} title={title} icon={titleIcon}>
        <div
          aria-label={title}
          data-context-menu-scroll=""
          data-slot="context-menu-primary"
          dir={flow.inline === "rtl" ? "rtl" : undefined}
          onContextMenu={(event) => event.preventDefault()}
          ref={menuRef}
          role="menu"
          tabIndex={-1}
          className="w-max max-w-none overflow-auto"
          style={{ maxWidth: "calc(100vw - 2px)", maxHeight: "calc(100dvh - var(--chip-row-height) - 2px)" }}
        >
          {renderFixedContextMenuItems(items, [], renderOptions)}
        </div>
      </ContextMenuChrome>
      {wings.map((wing) => {
        const wingKey = contextMenuPathKey(wing.parentPath);
        return (
          <ContextMenuWingFrame key={wingKey} parentKey={wingKey} style={{ position: "fixed", left: wingPlacements.get(wingKey)?.left ?? 0, top: wingPlacements.get(wingKey)?.top ?? 0 }}>
            <div
              aria-label={contextMenuItemAtPath(items, wing.parentPath)?.label ?? wingKey}
              data-context-menu-scroll=""
              dir={flow.inline === "rtl" ? "rtl" : undefined}
              onContextMenu={(event) => event.preventDefault()}
              role="menu"
              tabIndex={-1}
              className="flex w-max min-w-0 max-w-none flex-col overflow-auto"
              style={{ maxWidth: "calc(100vw - 2px)", maxHeight: "calc(100dvh - 2px)" }}
            >
              {renderFixedContextMenuItems(wing.items, wing.parentPath, renderOptions)}
            </div>
          </ContextMenuWingFrame>
        );
      })}
    </>,
    floatingHost,
  );
};

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
  const selectionRef = reactHostPort.useRef<string>("");
  reactHostPort.useEffect(() => {
    if (typeof document === "undefined") return undefined;
    const onContextMenu = (event: MouseEvent): void => {
      if (!isPointerEventOnDomTextSelection(event.target)) return;
      if (isContextMenuPointerTarget(event.target)) return;
      event.preventDefault();
      event.stopPropagation();
      targetRef.current = event.target;
      const editable = isDomTextEditableTarget(event.target);
      const captured = readDomTextSelection();
      selectionRef.current = captured;
      const hasSelection = Boolean(captured);
      setItems(
        buildTextSelectionContextMenuItems(
          { editable, hasSelection },
          { cut: cutLabel, copy: copyLabel, paste: pasteLabel, selectAll: selectAllLabel },
          {
            cut: () => {
              void cutDomTextSelection(targetRef.current, selectionRef.current);
            },
            copy: () => {
              void copyDomTextSelection(selectionRef.current);
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