import * as React from "react";
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import { type ContextMenuItem, contextMenuItemClassName, contextMenuShortcutClassName, renderContextMenuLeading } from "../🎨️item-render/🟦️.tsx";
import { contextMenuItemAtPath, contextMenuOrdinals, contextMenuPathKey, contextMenuPathsEqual, contextMenuReasonId, renderContextMenuOrdinalBadge, renderContextMenuReason } from "../🧭️navigation/🟦️.tsx";

export type FixedContextMenuRenderOptions = {
  readonly rootItems: readonly ContextMenuItem[];
  readonly activePath: readonly number[];
  readonly submenuCollapsedAt: readonly number[] | null;
  readonly setActivePath: (path: number[], collapseSubmenuAt?: readonly number[] | null) => void;
  readonly onClose: () => void;
  /** 🎯️ Every rendered row hands its element in under {@link contextMenuPathKey}, so the controller can move real DOM focus onto the active row wherever that row was portaled to. */
  readonly registerRow: (pathKey: string, node: HTMLButtonElement | null) => void;
};

export function contextMenuFusionOffsetsEqual(a: ReadonlyMap<string, ContextMenuFusionPlacement>, b: ReadonlyMap<string, ContextMenuFusionPlacement>): boolean {
  if (a.size !== b.size) {
    return false;
  }
  for (const [key, value] of a) {
    const other = b.get(key);
    if (!other || other.left !== value.left || other.top !== value.top) {
      return false;
    }
  }
  return true;
}

/** 📐️ Absolute placement of a fused submenu column inside {@link ContextMenuController}'s fusion body. */
export interface ContextMenuFusionPlacement {
  readonly left: number;
  readonly top: number;
}

function isContextMenuSubmenuOpen(activePath: readonly number[], parentPath: readonly number[]): boolean {
  if (activePath.length <= parentPath.length) {
    return false;
  }
  return parentPath.every((value, index) => activePath[index] === value);
}

/** ⏱️ Delay before hovering a parent row opens its submenu, so a pointer merely passing over the row doesn't flash it open. */
const CONTEXT_MENU_SUBMENU_HOVER_DELAY_MS = 150;

/** ↔️ Gap between a parent row and its submenu panel, in px. */
const CONTEXT_MENU_SUBMENU_GAP_PX = 0;

/** 📐️ Viewport-fixed placement of a submenu panel beside its parent row. */
export interface ContextMenuSubmenuPlacement {
  readonly left: number;
  readonly top: number;
  readonly flipped: boolean;
}

/** 📂 One fused submenu column beside its parent row inside the shared {@link ContextMenuChrome}. */
export interface ContextMenuFusionPanel {
  readonly parentPath: readonly number[];
  readonly items: readonly ContextMenuItem[];
}

/** 🫙 The fused columns of a closed menu: one object, so a closed menu handed fresh `items` measures nothing. */
export const CONTEXT_MENU_NO_FUSION_PANELS: readonly ContextMenuFusionPanel[] = [];

/** 📂 Open submenu columns to render fused into the root menu chrome (one outline, no nested title chips). */
export function contextMenuOpenFusionPanels(
  root: readonly ContextMenuItem[],
  activePath: readonly number[],
  submenuCollapsedAt: readonly number[] | null,
): readonly ContextMenuFusionPanel[] {
  const panels: ContextMenuFusionPanel[] = [];
  for (let depth = 0; depth < activePath.length; depth += 1) {
    const parentPath = activePath.slice(0, depth + 1);
    const item = contextMenuItemAtPath(root, parentPath);
    if (!item?.children?.length) {
      break;
    }
    const submenuOpen =
      isContextMenuSubmenuOpen(activePath, parentPath) ||
      (contextMenuPathsEqual(activePath, parentPath) && !(submenuCollapsedAt && contextMenuPathsEqual(submenuCollapsedAt, parentPath)));
    if (!submenuOpen) {
      break;
    }
    panels.push({ parentPath, items: item.children });
  }
  return panels;
}

/**
 * 📐️ Places a submenu panel beside its anchor row in viewport coordinates, flipping to the
 * anchor's start side when the end side would overflow and clamping the top edge into view.
 **/
export function contextMenuSubmenuPlacement(input: {
  readonly anchor: { readonly left: number; readonly right: number; readonly top: number };
  readonly panel: { readonly width: number; readonly height: number };
  readonly viewport: { readonly width: number; readonly height: number };
  readonly gap?: number;
}): ContextMenuSubmenuPlacement {
  const gap = input.gap ?? CONTEXT_MENU_SUBMENU_GAP_PX;
  const endLeft = input.anchor.right + gap;
  const startLeft = input.anchor.left - input.panel.width - gap;
  const overflowsEnd = endLeft + input.panel.width > input.viewport.width;
  const flipped = overflowsEnd && startLeft >= 0;
  const left = Math.max(0, Math.min(flipped ? startLeft : endLeft, Math.max(0, input.viewport.width - input.panel.width)));
  const top = Math.max(0, Math.min(input.anchor.top, Math.max(0, input.viewport.height - input.panel.height)));
  return { left, top, flipped };
}

type ContextMenuSubmenuRowProps = {
  readonly item: ContextMenuItem;
  readonly rowPath: readonly number[];
  readonly ordinal: number | undefined;
  readonly isActive: boolean;
  readonly submenuOpen: boolean;
  readonly setActivePath: FixedContextMenuRenderOptions["setActivePath"];
  readonly registerRow: FixedContextMenuRenderOptions["registerRow"];
};

/** 📂️ Parent-row button for a submenu: click toggles it open/closed; hover opens it after a short delay. Child rows render in fused columns beside this row. */
function ContextMenuSubmenuRow({ item, rowPath, ordinal, isActive, submenuOpen, setActivePath, registerRow }: ContextMenuSubmenuRowProps): React.ReactElement {
  const hoverTimerRef = reactHostPort.useRef<number | undefined>(undefined);
  const pathKey = contextMenuPathKey(rowPath);
  const clearHoverTimer = reactHostPort.useCallback((): void => {
    if (hoverTimerRef.current !== undefined) {
      window.clearTimeout(hoverTimerRef.current);
      hoverTimerRef.current = undefined;
    }
  }, []);
  reactHostPort.useEffect(() => clearHoverTimer, [clearHoverTimer]);
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
        id={item.id}
        ref={(node) => registerRow(pathKey, node)}
        aria-disabled={item.disabled || undefined}
        aria-describedby={contextMenuReasonId(item)}
        aria-expanded={submenuOpen}
        aria-haspopup="menu"
        className={contextMenuItemClassName(item)}
        data-active={isActive ? "true" : undefined}
        data-context-menu-path={pathKey}
        data-disabled={item.disabled ? "" : undefined}
        data-selected={item.checked ? "true" : undefined}
        data-menu-action={item.action}
        onClick={toggleSubmenu}
        onPointerEnter={() => item.onHover?.()}
        onPointerLeave={() => item.onHoverEnd?.()}
        role="menuitem"
        tabIndex={-1}
        type="button"
      >
        {renderContextMenuOrdinalBadge(ordinal)}
        {renderContextMenuLeading(item)}
        <span className="truncate">{item.label ?? item.id}</span>
        {renderContextMenuReason(item)}
        {item.shortcut ? (
          <span aria-hidden className={contextMenuShortcutClassName}>
            {item.shortcut}
          </span>
        ) : null}
        <span aria-hidden className={contextMenuShortcutClassName}>
          ›
        </span>
      </button>
    </div>
  );
}

export function renderFixedContextMenuItems(items: readonly ContextMenuItem[], pathPrefix: readonly number[], options: FixedContextMenuRenderOptions): React.ReactNode {
  const ordinals = contextMenuOrdinals(items);
  const { activePath, submenuCollapsedAt, setActivePath, onClose, registerRow } = options;
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
        <ContextMenuSubmenuRow key={item.id} item={item} rowPath={rowPath} ordinal={ordinal} isActive={isActive} submenuOpen={submenuOpen} setActivePath={setActivePath} registerRow={registerRow} />
      );
    }
    const role = item.checked === undefined ? "menuitem" : "menuitemcheckbox";
    return (
      <button
        key={item.id}
        id={item.id}
        ref={(node) => registerRow(contextMenuPathKey(rowPath), node)}
        aria-checked={item.checked}
        aria-disabled={item.disabled || undefined}
        aria-describedby={contextMenuReasonId(item)}
        className={contextMenuItemClassName(item)}
        data-active={isActive ? "true" : undefined}
        data-context-menu-path={contextMenuPathKey(rowPath)}
        data-disabled={item.disabled ? "" : undefined}
        data-selected={item.checked ? "true" : undefined}
        data-menu-action={item.action}
        onClick={(event) => {
          if (item.disabled) return;
          item.onSelect?.(event.nativeEvent);
          onClose();
        }}
        onPointerEnter={() => {
          activateRow();
          item.onHover?.();
        }}
        onPointerLeave={() => item.onHoverEnd?.()}
        role={role}
        tabIndex={-1}
        type="button"
      >
        {renderContextMenuOrdinalBadge(ordinal)}
        {renderContextMenuLeading(item)}
        <span className="truncate">{item.label ?? item.id}</span>
        {renderContextMenuReason(item)}
        {item.shortcut ? (
          <span aria-hidden className={contextMenuShortcutClassName}>
            {item.shortcut}
          </span>
        ) : null}
      </button>
    );
  });
}
