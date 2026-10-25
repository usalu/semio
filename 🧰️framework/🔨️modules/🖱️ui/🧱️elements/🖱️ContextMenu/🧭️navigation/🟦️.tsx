import * as React from "react";
import { type ContextMenuItem, contextMenuOrdinalClassName } from "../🎨️item-render/🟦️.tsx";

/** ⌨️ Keyboard navigation direction parsed from a keydown key. */
export type ContextMenuNavDirection = "up" | "down" | "left" | "right" | "activate" | "escape";

/** 🗺️ Maps each enabled row id to its 1-based ordinal within `items` (separators skipped). */
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

/** 🏹️ The rows arrow keys reach: every row but separators — a disabled row is focusable and announces why it cannot run. */
function contextMenuFocusableIndices(items: readonly ContextMenuItem[]): number[] {
  return items.flatMap((item, index) => (!item.separator ? [index] : []));
}

/** 💬️ The id of a disabled row's reason element — what its `aria-describedby` names. */
export function contextMenuReasonId(item: ContextMenuItem): string | undefined {
  return item.disabled && item.reason ? `${item.id}::reason` : undefined;
}

/** 🧠️ A disabled row's reason, visible beside its label and named by the row's `aria-describedby`. */
export function renderContextMenuReason(item: ContextMenuItem): React.ReactNode {
  const id = contextMenuReasonId(item);
  return id ? (
    <span id={id} data-slot="context-menu-reason" className="truncate ps-tiny text-xs text-muted-foreground">
      {item.reason}
    </span>
  ) : null;
}

/** 🪜️ Resolves the item list at `pathPrefix` (empty = top level). */
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

/** 📍️ Resolves the row at `path` (empty path → undefined). */
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

/** ✅️ Path to the first enabled `checked` row, if any. */
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

/** 🧭️ Maps arrows, wasd, Enter, Space, and Escape to menu navigation. */
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

/** ⬇️ Next active path when moving up or down within the current menu level. */
export function moveContextMenuActivePath(root: readonly ContextMenuItem[], path: readonly number[], direction: "up" | "down"): number[] {
  const levelPrefix = path.length > 0 ? path.slice(0, -1) : [];
  const level = contextMenuItemsAtLevel(root, levelPrefix);
  const enabled = contextMenuFocusableIndices(level);
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

/** 🔢️ Active path for digit `ordinal` (1–9) within the level of `path`. */
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

/** 📂️ Opens the submenu under `path` and selects its first enabled child. */
export function contextMenuOpenSubmenuPath(root: readonly ContextMenuItem[], path: readonly number[]): number[] | undefined {
  const item = contextMenuItemAtPath(root, path);
  if (!item?.children?.length) {
    return undefined;
  }
  const enabled = contextMenuFocusableIndices(item.children);
  if (enabled.length === 0) {
    return [...path];
  }
  return [...path, enabled[0]!];
}

export function contextMenuHoverItem(item: ContextMenuItem | undefined): void {
  item?.onHover?.();
}

export function contextMenuHoverEndItem(item: ContextMenuItem | undefined): void {
  item?.onHoverEnd?.();
}

export function isContextMenuEditableKeyTarget(target: EventTarget | null): boolean {
  if (!(target instanceof HTMLElement)) {
    return false;
  }
  const tag = target.tagName;
  return tag === "INPUT" || tag === "TEXTAREA" || target.isContentEditable;
}

export function renderContextMenuOrdinalBadge(ordinal: number | undefined): React.ReactNode {
  if (ordinal === undefined || ordinal > 9) {
    return <span aria-hidden className={contextMenuOrdinalClassName} />;
  }
  return (
    <span aria-hidden className={contextMenuOrdinalClassName}>
      {ordinal}
    </span>
  );
}

/** 🔑️ Stable key for a row path, used to address a rendered row across submenu portals. */
export function contextMenuPathKey(path: readonly number[]): string {
  return path.join(".");
}

export function contextMenuPathsEqual(a: readonly number[], b: readonly number[]): boolean {
  return a.length === b.length && a.every((value, index) => value === b[index]);
}
