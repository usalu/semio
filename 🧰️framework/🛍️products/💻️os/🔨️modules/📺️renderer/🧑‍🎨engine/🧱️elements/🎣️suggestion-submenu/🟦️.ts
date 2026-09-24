// #region 🧲️Header
/** @emoji 🎣️ The context menu's live "suggest" submenu, shared by every surface host that lists placement
 * suggestions for a connector under the pointer (`World3dHost` for vortices, `Board2dHost` for grips): the
 * right-click finds the row that names a connector, starts that connector's suggestion search the moment the
 * menu opens, turns the row into a submenu over the live candidate rows, and binds each candidate row to the
 * trace record it stands for so hovering it previews exactly that one candidate.
 * @see ../🌐️World3dHost/🟦️.tsx
 * @see ../🖥️Board2dHost/🟦️.tsx */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useEffect, useRef } from "react";
import type { ContextMenuItem } from "@semio-tech/ui-react";
import type { ContextMenuItemSpec } from "@semio-tech/framework";
// #endregion 🔌️Adapters

//#region 🎣️SuggestionSubmenu
/** 🎣️ The action a context-menu row carries to name the connector whose placement suggestions it lists. */
export const SUGGESTION_SUBMENU_OPEN_ACTION = "openVortexSuggestions";
/** 🔒️ The verb that releases the search a {@link SUGGESTION_SUBMENU_OPEN_ACTION} started. */
export const SUGGESTION_SUBMENU_CLOSE_ACTION = "closeVortexSuggestions";

/** 🗂️ What a guest publishes about its open suggestion menu, in the fields this module reads. */
export type SuggestionSubmenuRecord = {
  readonly open?: boolean;
  readonly windowId?: string;
  readonly submenu?: boolean;
  readonly pending: boolean;
  /** 🔑️ A trace key is a u64: a key beyond 2^53 travels as its decimal text so it survives the JSON hop exactly. */
  readonly candidates: readonly { readonly key?: number | string }[];
};

/** 🪟️ True when this pane owns the open suggestion menu — a menu naming no window is owned by whichever pane renders it. */
export function suggestionMenuOwnsWindow(menu: Pick<SuggestionSubmenuRecord, "open" | "windowId" | "submenu"> | null | undefined, windowInstanceId: string | undefined): boolean {
  if (!menu?.open) return false;
  return !menu.windowId || menu.windowId === windowInstanceId;
}

/** 🪟️ True when this pane owns the open suggestion menu AND it floats as its own popup; a submenu-presented list
 * renders inside the regular context menu instead. */
export function suggestionPopupOwnsWindow(menu: Pick<SuggestionSubmenuRecord, "open" | "windowId" | "submenu"> | null | undefined, windowInstanceId: string | undefined): boolean {
  return suggestionMenuOwnsWindow(menu, windowInstanceId) && !menu?.submenu;
}

/** 🔎️ The connector a resolved context menu's "suggest" row points at — the first row, at any depth, that
 * dispatches {@link SUGGESTION_SUBMENU_OPEN_ACTION} with a `fullId` — so the right-click can start the search the
 * moment the menu opens, before that row is ever hovered. */
export function suggestionSubmenuTarget(specs: readonly ContextMenuItemSpec[]): string | null {
  for (const spec of specs) {
    const fullId = spec.action === SUGGESTION_SUBMENU_OPEN_ACTION ? spec.args?.["fullId"] : undefined;
    if (typeof fullId === "string" && fullId.length > 0) return fullId;
    const nested = spec.children?.length ? suggestionSubmenuTarget(spec.children) : null;
    if (nested) return nested;
  }
  return null;
}

/** 📂️ Turns every "suggest" row of a mapped context menu into a live submenu over `children` — the row stops
 * dispatching (hovering or clicking it opens the list) — and keeps every other row, recursively. */
export function withSuggestionSubmenu(items: readonly ContextMenuItem[], children: readonly ContextMenuItem[]): ContextMenuItem[] {
  return items.map((item) => {
    if (item.action === SUGGESTION_SUBMENU_OPEN_ACTION) return { ...item, onSelect: undefined, children: [...children] };
    return item.children?.length ? { ...item, children: withSuggestionSubmenu(item.children, children) } : item;
  });
}

/** 🔦️ Binds each listed candidate row to the trace record it stands for: hovering focuses that one record,
 * leaving releases it. `items` are the menu's candidate rows mapped in order, so a row lines up with
 * `menu.candidates` exactly when the menu lists candidates. */
export function suggestionRowsWithFocus(items: readonly ContextMenuItem[], menu: Pick<SuggestionSubmenuRecord, "pending" | "candidates"> | null | undefined, focus: (key: bigint | null) => void): ContextMenuItem[] {
  if (!menu || menu.pending || menu.candidates.length !== items.length) return [...items];
  return items.map((item, at) => {
    const key = menu.candidates[at]?.key;
    if (key === undefined) return item;
    return {
      ...item,
      onHover: () => {
        item.onHover?.();
        focus(BigInt(key));
      },
      onHoverEnd: () => {
        item.onHoverEnd?.();
        focus(null);
      },
    };
  });
}

/** 🎣️ A context menu whose "suggest" row names `target` starts that connector's suggestion search the moment it
 * opens and releases it when it closes (or a later right-click retargets it) — so the list is already filling
 * by the time the pointer reaches the row. The close is keyed to the SAME menu the open was, so a retarget
 * always closes the old search before opening the new one. */
export function useSuggestionSubmenuSearch(
  target: string | null,
  point: { readonly x: number; readonly y: number },
  windowInstanceId: string | undefined,
  dispatch: (action: string, args?: Record<string, unknown>) => void,
  onRelease: () => void,
): void {
  const dispatchRef = useRef(dispatch);
  dispatchRef.current = dispatch;
  const releaseRef = useRef(onRelease);
  releaseRef.current = onRelease;
  useEffect(() => {
    if (!target) return undefined;
    dispatchRef.current(SUGGESTION_SUBMENU_OPEN_ACTION, { fullId: target, x: point.x, y: point.y, windowId: windowInstanceId, submenu: true });
    return () => {
      releaseRef.current();
      dispatchRef.current(SUGGESTION_SUBMENU_CLOSE_ACTION, { fullId: target });
    };
  }, [target, point.x, point.y, windowInstanceId]);
}
//#endregion 🎣️SuggestionSubmenu
