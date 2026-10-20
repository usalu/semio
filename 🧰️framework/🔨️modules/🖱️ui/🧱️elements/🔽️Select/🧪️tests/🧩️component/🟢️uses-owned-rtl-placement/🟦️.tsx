/** 🟢️ The uses owned rtl placement tests of the Select suite. */
import { fireEvent, render, screen } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { Select, SelectContent, SelectItem, SelectScrollDownButton, SelectTrigger, SelectValue, resolveSelectPlacement } from "../../../🟦️.tsx";

/** 🔵 Registers the uses owned rtl placement tests of the Select suite. */
export function registerUsesOwnedRtlPlacementTests(): void {
  it("uses owned RTL placement variables and scrolls the viewport without nested buttons", async () => {
    expect(resolveSelectPlacement({ top: 90, right: 120, bottom: 110, left: 80, width: 40, height: 20 }, { width: 100, height: 80 }, { width: 200, height: 140 }, "bottom", "start", 4, 8, true)).toMatchObject({
      side: "top",
      left: 20,
      availableHeight: 78,
    });
    const portal = document.createElement("div");
    document.body.append(portal);
    const view = render(
      <Select id="select-component-test-9" dir="rtl" defaultOpen defaultValue="one">
        <SelectTrigger aria-label="Scroll">
          <SelectValue />
        </SelectTrigger>
        <SelectContent container={portal} position="popper">
          <SelectItem value="one">One</SelectItem>
          <SelectScrollDownButton />
        </SelectContent>
      </Select>,
    );
    const listbox = await screen.findByRole("listbox");
    expect(portal.contains(listbox)).toBe(true);
    expect(listbox.getAttribute("dir")).toBe("rtl");
    expect(listbox.querySelectorAll("button")).toHaveLength(0);
    const viewport = listbox.querySelector<HTMLElement>('[data-slot="select-viewport"]')!;
    Object.defineProperty(viewport, "clientHeight", { value: 100, configurable: true });
    const scrollBy = vi.fn();
    viewport.scrollBy = scrollBy;
    const controls = listbox.querySelectorAll<HTMLElement>('[data-slot="select-scroll-down-button"]');
    fireEvent.pointerDown(controls[0]!);
    expect(scrollBy).toHaveBeenCalledWith({ top: 80, behavior: "auto" });
    view.unmount();
    portal.remove();
  });
}
