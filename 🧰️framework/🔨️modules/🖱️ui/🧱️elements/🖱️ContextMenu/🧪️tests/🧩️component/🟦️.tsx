// #region 🔌️Adapters
import * as React from "react";
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ContextMenuController, contextMenuPathKey, contextMenuSubmenuPlacement, copyDomTextSelection, readDomTextSelection, type ContextMenuItem } from "../../🟦️.tsx";
import { uiDataLabel } from "../../../🎗️UiLabel/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🖱️SubmenuReachability
/** 🧾 A group row with two children — the shape `editor.rs` gives `menu.group.transfer`. */
const items: ContextMenuItem[] = [
  { id: "reorganize", label: uiDataLabel("Reorganize"), action: "reorganize" },
  {
    id: "menu.group.transfer",
    label: uiDataLabel("Transfer"),
    action: "menu.group.transfer",
    children: [
      { id: "importDocumentRequest", label: uiDataLabel("Import Artifact…"), action: "importDocumentRequest" },
      { id: "exportDocument", label: uiDataLabel("Export Artifact"), action: "exportDocument" },
    ],
  },
];

const renderMenu = () =>
  render(<ContextMenuController open position={{ x: 40, y: 40 }} items={items} onOpenChange={() => undefined} title={uiDataLabel("Actions")} />);

describe("🖱️ context menu submenu reachability", () => {
  it("places a submenu beside its anchor row and flips only when the end side overflows", () => {
    const viewport = { width: 1000, height: 800 };
    const panel = { width: 180, height: 90 };
    expect(contextMenuSubmenuPlacement({ anchor: { left: 100, right: 260, top: 200 }, panel, viewport })).toEqual({ left: 260, top: 200, flipped: false });
    expect(contextMenuSubmenuPlacement({ anchor: { left: 700, right: 900, top: 200 }, panel, viewport })).toEqual({ left: 520, top: 200, flipped: true });
  });

  it("clamps a submenu whose anchor sits at the bottom edge back into the viewport", () => {
    const placement = contextMenuSubmenuPlacement({ anchor: { left: 10, right: 100, top: 760 }, panel: { width: 120, height: 200 }, viewport: { width: 1000, height: 800 } });
    expect(placement.top).toBe(600);
    expect(placement.left).toBe(100);
  });

  it("fuses an open submenu into the same context menu chrome without a nested title chip", () => {
    const { getByRole } = renderMenu();
    fireEvent.click(getByRole("menuitem", { name: /Transfer/u }));
    const panel = document.querySelector('[data-slot="context-menu-submenu"]');
    expect(panel).not.toBeNull();
    const chrome = document.querySelector('[data-slot="context-menu-content"]');
    expect(chrome?.contains(panel as Node)).toBe(true);
    expect(document.querySelectorAll('[data-slot="context-menu-title-chip"]')).toHaveLength(1);
    expect(panel?.getAttribute("data-context-menu-submenu-of")).toBe(contextMenuPathKey([1]));
    const transfer = getByRole("menuitem", { name: /Transfer/u });
    expect(Math.abs((panel as Element).getBoundingClientRect().top - transfer.getBoundingClientRect().top)).toBeLessThan(2);
  });

  it("gives a submenu its own menu role, so a pointer inside it is not an outside dismiss", () => {
    const { getByRole } = renderMenu();
    fireEvent.click(getByRole("menuitem", { name: /Transfer/u }));
    const panel = document.querySelector('[data-slot="context-menu-submenu"]');
    expect(panel?.getAttribute("role")).toBe("menu");
    expect(document.getElementById("exportDocument")?.closest('[role="menu"]')).toBe(panel);
  });

  it("moves real DOM focus onto the active row, not only the painted mark", () => {
    renderMenu();
    fireEvent.keyDown(window, { key: "ArrowDown" });
    expect(document.activeElement?.id).toBe("reorganize");
    fireEvent.keyDown(window, { key: "ArrowDown" });
    expect(document.activeElement?.id).toBe("menu.group.transfer");
    fireEvent.keyDown(window, { key: "ArrowRight" });
    expect(document.activeElement?.id).toBe("importDocumentRequest");
    fireEvent.keyDown(window, { key: "ArrowDown" });
    expect(document.activeElement?.id).toBe("exportDocument");
    expect(document.getElementById("exportDocument")?.getAttribute("data-active")).toBe("true");
  });

  it("copies the selection the menu was OPENED on, because opening it took focus and collapsed that selection", async () => {
    const writeText = vi.fn().mockResolvedValue(undefined);
    Object.defineProperty(navigator, "clipboard", { configurable: true, value: { writeText, readText: vi.fn() } });
    expect(readDomTextSelection()).toBe("");
    await copyDomTextSelection("Copy me please");
    expect(writeText).toHaveBeenCalledWith("Copy me please");
    writeText.mockClear();
    await copyDomTextSelection();
    expect(writeText).not.toHaveBeenCalled();
  });

  it("announces a group row as a popup owner", () => {
    const { getByRole } = renderMenu();
    const group = getByRole("menuitem", { name: /Transfer/u });
    expect(group.getAttribute("aria-haspopup")).toBe("menu");
    expect(group.getAttribute("aria-expanded")).toBe("false");
    fireEvent.click(group);
    expect(group.getAttribute("aria-expanded")).toBe("true");
  });
});

// #region 🔴️LiveItems
/** 🔴️ A live menu (a suggestion submenu whose candidates stream in) republishes NEW row objects with the SAME ids
 * while open, the checked candidate sitting inside the submenu. */
const liveItems = (): ContextMenuItem[] => [
  {
    id: "suggest",
    label: uiDataLabel("Suggest parts"),
    children: [
      { id: "suggestion-0", label: uiDataLabel("Capsule · Grip 1"), checked: true, action: "acceptSuggestion" },
      { id: "suggestion-1", label: uiDataLabel("Core · Grip 2"), action: "acceptSuggestion" },
    ],
  },
  { id: "delete", label: uiDataLabel("Delete"), action: "deleteSelection" },
];

describe("🖱️ context menu with live items", () => {
  it("keeps the keyboard position across republished rows, so Escape collapses the submenu and then closes the menu", () => {
    const onOpenChange = vi.fn();
    const menu = (items: ContextMenuItem[]) => <ContextMenuController open position={{ x: 40, y: 40 }} items={items} onOpenChange={onOpenChange} title={uiDataLabel("Actions")} />;
    const { rerender } = render(menu(liveItems()));
    expect(document.activeElement?.id).toBe("suggestion-0");
    fireEvent.keyDown(window, { key: "Escape" });
    expect(document.activeElement?.id).toBe("suggest");
    rerender(menu(liveItems()));
    expect(document.activeElement?.id).toBe("suggest");
    expect(onOpenChange).not.toHaveBeenCalled();
    fireEvent.keyDown(window, { key: "Escape" });
    expect(onOpenChange).toHaveBeenCalledWith(false);
  });
});
// #endregion 🔴️LiveItems
// #endregion 🖱️SubmenuReachability

// #region 🖱️OutsideDismiss
describe("🖱️ context menu outside dismiss", () => {
  it("still dismisses when canvas capture calls stopImmediatePropagation and bubble never reaches window", async () => {
    const onOpenChange = vi.fn();
    const canvas = document.createElement("canvas");
    document.body.append(canvas);
    canvas.addEventListener(
      "pointerdown",
      (event) => {
        event.stopImmediatePropagation();
      },
      true,
    );
    render(<ContextMenuController open position={{ x: 8, y: 8 }} items={[{ id: "a", label: uiDataLabel("Alpha") }]} onOpenChange={onOpenChange} title={uiDataLabel("Actions")} />);
    await new Promise((resolve) => setTimeout(resolve, 0));
    canvas.dispatchEvent(new PointerEvent("pointerdown", { bubbles: true, cancelable: true, button: 0, clientX: 4, clientY: 4 }));
    expect(onOpenChange).toHaveBeenCalledWith(false);
    canvas.remove();
  });
});
// #endregion 🖱️OutsideDismiss
