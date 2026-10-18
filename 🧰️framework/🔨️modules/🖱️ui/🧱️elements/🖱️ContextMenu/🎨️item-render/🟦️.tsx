import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { floatingMenuItemClass } from "../../../🔨️modules/🎨️chrome-class-names/🟦️.ts";
import type { UiLabel } from "../../🎗️UiLabel/🟦️.tsx";
import { type ControlIcon, Icon, type IconSource } from "../../🔣️Icons/🟦️.tsx";
import * as React from "react";
import { createPortal } from "react-dom";

export const contextMenuShortcutClassName = "ms-auto text-xs text-muted-foreground ps-tiny";
export const contextMenuOrdinalClassName = "w-small shrink-0 text-center text-xs text-muted-foreground tabular-nums";

/** 🪟️ Context-menu row — same density as {@link floatingMenuItemClass}; `checked` paints the active/preview highlight (no tick/checkmark), kept through hover like {@link CanvasPickMenu}. */
export function contextMenuItemClassName(item: Pick<ContextMenuItem, "checked" | "destructive">, ...extra: Array<string | false | null | undefined>): string {
  return cn(
    floatingMenuItemClass,
    "pointer-events-auto whitespace-nowrap data-[disabled]:cursor-not-allowed data-[disabled]:opacity-50",
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
  /** 🚫️ A disabled row stays focusable (`aria-disabled`, WAI-ARIA focusable-when-disabled) and never runs. */
  disabled?: boolean;
  /** 💬️ Why a disabled row cannot run: shown beside its label and named by `aria-describedby`. */
  reason?: UiLabel;
  separator?: boolean;
  checked?: boolean;
  destructive?: boolean;
  action?: string;
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

export function renderContextMenuLeading(item: ContextMenuItem): React.ReactNode {
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

export function getElementById<T extends HTMLElement = HTMLElement>(id: string): T | null {
  return typeof document === "undefined" ? null : (document.getElementById(id) as T | null);
}

export function queryElement<T extends Element = HTMLElement>(selector: string, root?: ParentNode | null): T | null {
  return (root ?? (typeof document === "undefined" ? null : document))?.querySelector(selector) as T | null;
}

export function renderPortalInto(children: React.ReactNode, container: Element | DocumentFragment | null | undefined): React.ReactNode {
  return container ? createPortal(children, container) : null;
}
