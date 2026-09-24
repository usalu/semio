/** @emoji 🎣️ Laws of the world context menu's live "suggest" submenu: the right-click finds the row that names a
 * vortex (so the search starts the moment the menu opens), that row becomes a submenu over the live candidate
 * rows instead of dispatching, each candidate row focuses exactly its own trace record while hovered, and the
 * floating popup only renders for a popup-presented menu. */
import { describe, expect, it } from "vitest";
import type { ContextMenuItem } from "@semio-tech/ui-react";
import type { ContextMenuItemSpec } from "@semio-tech/framework";
import { suggestionMenuItems } from "../../../🌐️World3dHost/🟦️.tsx";
import { SUGGESTION_SUBMENU_OPEN_ACTION, suggestionMenuOwnsWindow, suggestionPopupOwnsWindow, suggestionRowsWithFocus, suggestionSubmenuTarget, withSuggestionSubmenu } from "../../🟦️.ts";

const labels = { checkingPlacement: "Checking placement…", noPlacement: "No placement" } as never;

describe("world context menu suggestion submenu", () => {
  it("finds the suggest row's vortex at any depth, and nothing without a named vortex", () => {
    const specs: ContextMenuItemSpec[] = [
      { id: "zoom", action: "focusSelection" },
      { id: "menu.group.tools", children: [{ id: "suggest", action: SUGGESTION_SUBMENU_OPEN_ACTION, args: { fullId: "part-ä:grip-1" } }] },
    ];
    expect(suggestionSubmenuTarget(specs)).toBe("part-ä:grip-1");
    expect(suggestionSubmenuTarget([{ id: "suggest", action: SUGGESTION_SUBMENU_OPEN_ACTION, args: {} }])).toBeNull();
    expect(suggestionSubmenuTarget([{ id: "delete", action: "deleteSelection" }])).toBeNull();
  });

  it("turns only the suggest row into a submenu over the candidate rows, keeping every other row", () => {
    const selected: string[] = [];
    const items: ContextMenuItem[] = [
      { id: "suggest", action: SUGGESTION_SUBMENU_OPEN_ACTION, onSelect: () => selected.push("suggest") },
      { id: "delete", action: "deleteSelection", onSelect: () => selected.push("delete") },
      { id: "menu.group.more", children: [{ id: "nested-suggest", action: SUGGESTION_SUBMENU_OPEN_ACTION, onSelect: () => selected.push("nested") }] },
    ];
    const children: ContextMenuItem[] = [{ id: "pending", label: "Checking placement…" as never, disabled: true }];
    const menu = withSuggestionSubmenu(items, children);
    expect(menu[0]!.onSelect).toBeUndefined();
    expect(menu[0]!.children).toEqual(children);
    expect(menu[1]).toBe(items[1]);
    expect(menu[2]!.children![0]!.children).toEqual(children);
    expect(items[0]!.children).toBeUndefined();
  });

  it("focuses exactly the hovered candidate's trace record and releases it on leave", () => {
    const menu = { pending: false, vortexFullId: "part-ä:grip-1", candidates: [
      { index: 0, key: 3, objectLabel: "Kapsel", vortexLabel: "Griff 1" },
      { index: 1, key: 7, objectLabel: "Kern", vortexLabel: "Griff 2" },
    ] };
    const specs = suggestionMenuItems(menu, 0, labels);
    const rows: ContextMenuItem[] = specs.map((spec) => ({ id: spec.id, label: spec.label as never, action: spec.action }));
    const focused: (bigint | null)[] = [];
    const bound = suggestionRowsWithFocus(rows, menu, (key) => focused.push(key));
    bound[1]!.onHover?.();
    bound[1]!.onHoverEnd?.();
    bound[0]!.onHover?.();
    expect(focused).toEqual([7n, null, 3n]);
  });

  it("leaves pending and empty menus without focus bindings", () => {
    const pending = { pending: true, vortexFullId: "part-ä:grip-1", candidates: [] };
    const rows: ContextMenuItem[] = suggestionMenuItems(pending, 0, labels).map((spec) => ({ id: spec.id, label: spec.label as never }));
    const bound = suggestionRowsWithFocus(rows, pending, () => {
      throw new Error("a pending row focuses nothing");
    });
    expect(bound).toEqual(rows);
    expect(bound[0]!.onHover).toBeUndefined();
  });

  it("renders the floating popup only for a popup-presented menu owned by this window", () => {
    expect(suggestionPopupOwnsWindow({ open: true, windowId: "top" }, "top")).toBe(true);
    expect(suggestionPopupOwnsWindow({ open: true, windowId: "top", submenu: true }, "top")).toBe(false);
    expect(suggestionMenuOwnsWindow({ open: true, windowId: "top", submenu: true }, "top")).toBe(true);
    expect(suggestionPopupOwnsWindow({ open: true, windowId: "top" }, "perspective")).toBe(false);
    expect(suggestionPopupOwnsWindow(null, "top")).toBe(false);
  });
});
