// #region 🔌️Adapters
import * as React from "react";
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ContextMenuController, contextMenuPathKey, contextMenuSubmenuPlacement, copyDomTextSelection, readDomTextSelection, type ContextMenuItem } from "../../🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🖱️SubmenuReachability
/** 🧾 A group row with two children — the shape `editor.rs` gives `menu.group.transfer`. */
const items: ContextMenuItem[] = [
  { id: "reorganize", label: "Reorganize", action: "reorganize" },
  {
    id: "menu.group.transfer",
    label: "Transfer",
    action: "menu.group.transfer",
    children: [
      { id: "importDocumentRequest", label: "Import Artifact…", action: "importDocumentRequest" },
      { id: "exportDocument", label: "Export Artifact", action: "exportDocument" },
    ],
  },
];

const renderMenu = () =>
  render(<ContextMenuController open position={{ x: 40, y: 40 }} items={items} onOpenChange={() => undefined} title="Actions" />);

describe("🖱️ context menu submenu reachability", () => {
  it("places a submenu beside its anchor row and flips only when the end side overflows", () => {
    const viewport = { width: 1000, height: 800 };
    const panel = { width: 180, height: 90 };
    expect(contextMenuSubmenuPlacement({ anchor: { left: 100, right: 260, top: 200 }, panel, viewport })).toEqual({ left: 264, top: 200, flipped: false });
    expect(contextMenuSubmenuPlacement({ anchor: { left: 700, right: 900, top: 200 }, panel, viewport })).toEqual({ left: 516, top: 200, flipped: true });
  });

  it("clamps a submenu whose anchor sits at the bottom edge back into the viewport", () => {
    const placement = contextMenuSubmenuPlacement({ anchor: { left: 10, right: 100, top: 760 }, panel: { width: 120, height: 200 }, viewport: { width: 1000, height: 800 } });
    expect(placement.top).toBe(600);
    expect(placement.left).toBe(104);
  });

  it("keeps a submenu out of the scrolling menu chrome that would clip it away", () => {
    const { container, getByRole } = renderMenu();
    fireEvent.click(getByRole("menuitem", { name: /Transfer/u }));
    const panel = document.querySelector('[data-slot="context-menu-submenu"]');
    expect(panel).not.toBeNull();
    const scrollport = panel?.closest(".overflow-y-auto");
    expect(scrollport).toBeNull();
    expect(container.contains(panel as Node)).toBe(false);
    expect(panel?.getAttribute("data-context-menu-submenu-of")).toBe(contextMenuPathKey([1]));
  });

  it("gives a submenu its own menu role, so a pointer inside it is not an outside dismiss", () => {
    const { getByRole } = renderMenu();
    fireEvent.click(getByRole("menuitem", { name: /Transfer/u }));
    const panel = document.querySelector('[data-slot="context-menu-submenu"]');
    expect(panel?.querySelector('[role="menu"]')).not.toBeNull();
    expect(document.getElementById("exportDocument")?.closest('[role="menu"]')).not.toBeNull();
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
// #endregion 🖱️SubmenuReachability
