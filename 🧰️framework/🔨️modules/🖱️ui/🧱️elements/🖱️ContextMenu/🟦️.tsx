// #region 🧲️Header
// 💻️ framework/ui/elements/🖱️ContextMenu/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

import * as React from "react";
import { reactHostPort } from "../🔌️Ports/🟦️.tsx";
import type { UiLabel } from "../🎗️UiLabel/🟦️.tsx";
import type { IconSource } from "../🔣️Icons/🟦️.tsx";
import { useLabel } from "../🏷️Label/🟦️.tsx";
import { useShellFloatingSurfaceHost } from "../🐚️ShellScope/🟦️.tsx";
import { useFlow } from "../../🔨️modules/🧭️flow-direction-context/🟦️.tsx";
import { ContextMenuChrome } from "./🥣️chrome/🟦️.tsx";
import { createDOMEventBinding } from "../../🔨️modules/👂️dom-event-binding/🟦️.ts";
import { type ContextMenuItem, renderPortalInto } from "./🎨️item-render/🟦️.tsx";
import { contextMenuHoverEndItem, contextMenuHoverItem, contextMenuItemAtPath, contextMenuNavigationFromKey, contextMenuOpenSubmenuPath, contextMenuPathForOrdinal, contextMenuPathKey, findContextMenuCheckedPath, isContextMenuEditableKeyTarget, moveContextMenuActivePath } from "./🧭️navigation/🟦️.tsx";
import { CONTEXT_MENU_NO_FUSION_PANELS, type ContextMenuFusionPlacement, type FixedContextMenuRenderOptions, contextMenuFusionOffsetsEqual, contextMenuOpenFusionPanels, renderFixedContextMenuItems } from "./🔗️fusion/🟦️.tsx";
import { buildTextSelectionContextMenuItems, copyDomTextSelection, cutDomTextSelection, isDomTextEditableTarget, isPointerEventOnDomTextSelection, pasteDomTextSelection, readDomTextSelection, selectAllDomText } from "./📝️text-selection/🟦️.ts";

export { createDOMEventBinding, type DOMListenerTarget } from "../../🔨️modules/👂️dom-event-binding/🟦️.ts";

export interface ContextMenuProps {
  items?: readonly ContextMenuItem[];
  children: React.ReactNode;
  /** 🪟️ Title chip on the window-chrome cap row. */
  title: UiLabel;
  /** 🪟️ Catalog icon shown in the title chip before the title. */
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
  /** 🪟️ Title chip on the window-chrome cap row. */
  title: UiLabel;
  /** 🪟️ Catalog icon in the title chip. */
  titleIcon?: IconSource;
  /** 🖱️ When false, selecting a row does not dismiss — the row action owns closing (e.g. acceptSuggestion). Outside pointer / Escape still dismiss. Default true. */
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

/** ⌨️ Finds the first enabled top-level row marked `checked`. */
export function findCheckedContextMenuItem(items: readonly ContextMenuItem[]): ContextMenuItem | undefined {
  const path = findContextMenuCheckedPath(items);
  return path ? contextMenuItemAtPath(items, path) : undefined;
}

/**
 * 🧩️ Controlled right-click menu whose title chip bottom-left anchors at viewport coordinates (puzzle 2d canvas bridge), keeping the first row beside the pointer. Portals to `document.body` for correct `fixed` placement under transformed UI; outside-dismiss uses a `window` capture listener (same as {@link CanvasPickMenu}) so a canvas `stopImmediatePropagation` on pointer-down cannot swallow the bubble phase and leave the menu stuck open.
 **/
export const ContextMenuController: React.FC<ContextMenuControllerProps> = ({ open, position, items, onOpenChange, title, titleIcon = "list", closeOnSelect = true }) => {
  const close = reactHostPort.useCallback(() => onOpenChange(false), [onOpenChange]);
  // 🐚️ Falls back to `document.body` outside any shell — inside one, portals into that shell's own
  // overlay layer so a context menu never escapes that shell's stacking context OR its appearance scope.
  const floatingHost = useShellFloatingSurfaceHost();
  const flow = useFlow();
  const menuRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const fusionBodyRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const chromeRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const [fusionOffsets, setFusionOffsets] = reactHostPort.useState<ReadonlyMap<string, ContextMenuFusionPlacement>>(() => new Map());
  const itemsRef = reactHostPort.useRef(items);
  itemsRef.current = items;
  const [activePath, setActivePath] = reactHostPort.useState<number[]>([]);
  const [submenuCollapsedAt, setSubmenuCollapsedAt] = reactHostPort.useState<readonly number[] | null>(null);
  const activePathRef = reactHostPort.useRef(activePath);
  activePathRef.current = activePath;
  const previousHoverItemRef = reactHostPort.useRef<ContextMenuItem | undefined>(undefined);
  // ⌨️ Rows hand their element in here so the active row can take REAL DOM focus wherever it was
  // portaled to. Before this the active row lived in React state alone: `document.activeElement`
  // stayed `<body>` through a whole arrow walk, so a screen reader was told nothing at all.
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
  }, [open, activePath]);
  // ⌨️ The active path is SEEDED when the menu opens and only re-seeded when a new `items` no longer holds the
  // row the user is on. A live menu (a suggestion submenu whose candidates stream in) republishes its rows
  // while open; re-seeding on every one threw the pointer/keyboard position back to the checked row, so an
  // Escape that collapsed a submenu was undone by the next republish and the menu could never be closed.
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
  const fusionPanels = reactHostPort.useMemo(() => (open ? contextMenuOpenFusionPanels(items, activePath, submenuCollapsedAt) : CONTEXT_MENU_NO_FUSION_PANELS), [open, activePath, items, submenuCollapsedAt]);
  reactHostPort.useLayoutEffect(() => {
    const body = fusionBodyRef.current;
    if (!open || !body || fusionPanels.length === 0) {
      setFusionOffsets((previous) => (previous.size === 0 ? previous : new Map()));
      if (body) {
        body.style.removeProperty("padding-right");
        body.style.removeProperty("padding-bottom");
      }
      return;
    }
    const primary = body.querySelector<HTMLElement>('[data-slot="context-menu-fusion-primary"]');
    if (!primary) {
      return;
    }
    const bodyRect = body.getBoundingClientRect();
    const primaryRect = primary.getBoundingClientRect();
    const next = new Map<string, ContextMenuFusionPlacement>();
    for (let index = 0; index < fusionPanels.length; index += 1) {
      const panel = fusionPanels[index]!;
      const panelKey = contextMenuPathKey(panel.parentPath);
      const anchor = rowNodesRef.current.get(panelKey);
      if (!anchor) {
        continue;
      }
      const anchorRect = anchor.getBoundingClientRect();
      let attachRight = primaryRect.right;
      if (index > 0) {
        const previousKey = contextMenuPathKey(fusionPanels[index - 1]!.parentPath);
        const previousWing = body.querySelector<HTMLElement>(`[data-context-menu-submenu-of="${previousKey}"]`);
        if (previousWing) {
          attachRight = previousWing.getBoundingClientRect().right;
        }
      }
      next.set(panelKey, { left: attachRight - bodyRect.left, top: anchorRect.top - bodyRect.top });
    }
    setFusionOffsets((previous) => (contextMenuFusionOffsetsEqual(previous, next) ? previous : next));
  }, [fusionPanels, open]);
  reactHostPort.useLayoutEffect(() => {
    const body = fusionBodyRef.current;
    if (!open || !body || fusionPanels.length === 0) {
      return;
    }
    const primary = body.querySelector<HTMLElement>('[data-slot="context-menu-fusion-primary"]');
    if (!primary) {
      return;
    }
    const primaryRect = primary.getBoundingClientRect();
    let maxRight = primaryRect.right;
    let maxBottom = primaryRect.bottom;
    for (const panel of fusionPanels) {
      const wing = body.querySelector<HTMLElement>(`[data-context-menu-submenu-of="${contextMenuPathKey(panel.parentPath)}"]`);
      if (!wing) {
        continue;
      }
      const wingRect = wing.getBoundingClientRect();
      maxRight = Math.max(maxRight, wingRect.right);
      maxBottom = Math.max(maxBottom, wingRect.bottom);
    }
    body.style.paddingRight = `${Math.max(0, maxRight - primaryRect.right)}px`;
    body.style.paddingBottom = `${Math.max(0, maxBottom - primaryRect.bottom)}px`;
    const chrome = chromeRef.current;
    if (chrome) {
      chrome.setAttribute("data-silhouette-remeasure", String(performance.now()));
    }
  }, [fusionOffsets, fusionPanels, open]);
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
    <ContextMenuChrome ref={chromeRef} style={{ left: position.x, position: "fixed", top: `calc(${position.y}px - var(--size-medium))` }} title={title} icon={titleIcon}>
      <div
        data-slot="context-menu-fusion-body"
        dir={flow.inline === "rtl" ? "rtl" : undefined}
        onContextMenu={(event) => event.preventDefault()}
        ref={fusionBodyRef}
        className="relative w-fit max-w-none"
      >
        <div aria-label={title} data-slot="context-menu-fusion-primary" ref={menuRef} role="menu" tabIndex={-1} className="w-max shrink-0">
          {renderFixedContextMenuItems(items, [], renderOptions)}
        </div>
        {fusionPanels.map((panel) => {
          const panelKey = contextMenuPathKey(panel.parentPath);
          return (
            <div
              key={panelKey}
              aria-label={contextMenuItemAtPath(items, panel.parentPath)?.label ?? panelKey}
              data-context-menu-submenu-of={panelKey}
              data-slot="context-menu-submenu"
              role="menu"
              className="absolute top-0 flex w-max min-w-0 flex-col"
              style={{ left: fusionOffsets.get(panelKey)?.left ?? 0, top: fusionOffsets.get(panelKey)?.top ?? 0 }}
            >
              {renderFixedContextMenuItems(panel.items, panel.parentPath, renderOptions)}
            </div>
          );
        })}
      </div>
    </ContextMenuChrome>,
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
      // 📋️ Read the selection HERE, while it still exists: opening the menu moves focus onto it.
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