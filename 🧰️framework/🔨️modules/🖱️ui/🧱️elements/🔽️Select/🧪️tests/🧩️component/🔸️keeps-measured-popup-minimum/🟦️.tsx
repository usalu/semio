/** 🔸️ The keeps measured popup minimum tests of the Select suite. */
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectSeparator, SelectTrigger, SelectValue } from "../../../🟦️.tsx";
import selectPopupGeometryFixture from "../../../../../🧪️tests/🧫️fixtures/🔽️select-popup-geometry/🔣️.json";
import { BasicSelect } from "../🟣️shared/🟦️.tsx";

/** 🔹 Registers the keeps measured popup minimum tests of the Select suite. */
export function registerKeepsMeasuredPopupMinimumTests(): void {
  it("keeps the measured popup minimum, border, scroll bands, viewport, and selected-row reveal contract", async () => {
    const [normal, constrained] = selectPopupGeometryFixture.cases;
    expect(normal.popupWidth).toBe(Math.max(selectPopupGeometryFixture.triggerWidth, selectPopupGeometryFixture.minimumContentWidth));
    expect(normal.popupHeight).toBe(selectPopupGeometryFixture.borderWidth * 2 + selectPopupGeometryFixture.scrollBandHeight * 2 + normal.scrollViewportHeight);
    expect(constrained.popupHeight).toBe(selectPopupGeometryFixture.borderWidth * 2 + selectPopupGeometryFixture.scrollBandHeight * 2 + constrained.scrollViewportHeight);
    const { wheelDelta, scrollAfterWheel } = constrained;
    if (wheelDelta === undefined || scrollAfterWheel === undefined) throw new Error("the constrained select geometry case must carry its wheel oracle");
    expect(constrained.initialScroll + wheelDelta).toBeGreaterThan(scrollAfterWheel);

    const scrollIntoView = vi.fn();
    const descriptor = Object.getOwnPropertyDescriptor(HTMLElement.prototype, "scrollIntoView");
    Object.defineProperty(HTMLElement.prototype, "scrollIntoView", { configurable: true, value: scrollIntoView });
    try {
      render(
        <Select id="select-popup-geometry" defaultOpen defaultValue="row-6">
          <SelectTrigger aria-label="Projection"><SelectValue /></SelectTrigger>
          <SelectContent>
            {Array.from({ length: selectPopupGeometryFixture.optionCount }, (_, index) => <SelectItem key={index} value={`row-${index}`}>Row {index}</SelectItem>)}
          </SelectContent>
        </Select>,
      );
      const listbox = await screen.findByRole("listbox");
      expect(listbox.className).toContain("min-w-32");
      expect(listbox.className).toContain("border");
      expect(listbox.querySelectorAll('[data-slot="select-scroll-up-button"]')).toHaveLength(1);
      expect(listbox.querySelectorAll('[data-slot="select-scroll-down-button"]')).toHaveLength(1);
      const viewport = listbox.querySelector<HTMLElement>('[data-slot="select-viewport"]')!;
      expect(viewport.className).toContain("overflow-y-auto");
      expect(viewport.className).toContain("p-single");
      expect(scrollIntoView).toHaveBeenCalledWith({ block: "nearest" });
    } finally {
      if (descriptor) Object.defineProperty(HTMLElement.prototype, "scrollIntoView", descriptor);
      else delete (HTMLElement.prototype as { scrollIntoView?: unknown }).scrollIntoView;
    }
  });
  it("owns fallback value, projected text, pointer selection, and focus return", async () => {
    const changes = vi.fn();
    render(<BasicSelect onValueChange={changes} />);
    const trigger = screen.getByRole("combobox", { name: "Mode" });
    expect(trigger.textContent).toContain("Alpha");
    fireEvent.click(trigger);
    const listbox = await screen.findByRole("listbox");
    expect(document.activeElement).toBe(listbox);
    expect(trigger.getAttribute("aria-controls")).toBe(listbox.id);
    fireEvent.pointerMove(screen.getByRole("option", { name: "Gamma" }));
    fireEvent.pointerDown(screen.getByRole("option", { name: "Gamma" }));
    fireEvent.click(screen.getByRole("option", { name: "Gamma" }));
    expect(changes).toHaveBeenCalledTimes(1);
    expect(changes).toHaveBeenCalledWith("gamma");
    await waitFor(() => expect(screen.queryByRole("listbox")).toBeNull());
    expect(trigger.textContent).toContain("Gamma");
    expect(document.activeElement).toBe(trigger);
  });
  it("keeps controlled value and open state authoritative during parent lag", async () => {
    const values = vi.fn();
    const openings = vi.fn();
    const view = render(<BasicSelect value="alpha" open={false} onValueChange={values} onOpenChange={openings} />);
    const trigger = screen.getByRole("combobox", { name: "Mode" });
    fireEvent.click(trigger);
    expect(openings).toHaveBeenCalledWith(true);
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(screen.queryByRole("listbox")).toBeNull();
    view.rerender(<BasicSelect value="alpha" open onValueChange={values} onOpenChange={openings} />);
    const gamma = await screen.findByRole("option", { name: "Gamma" });
    fireEvent.click(gamma);
    expect(values).toHaveBeenCalledWith("gamma");
    expect(openings).toHaveBeenLastCalledWith(false);
    expect(screen.getByRole("listbox")).toBeTruthy();
    expect(trigger.textContent).toContain("Alpha");
    expect(screen.getByRole("option", { name: "Alpha" }).getAttribute("aria-selected")).toBe("true");
  });
  it("keeps active option separate from selection and skips disabled rows", async () => {
    const values = vi.fn();
    render(<BasicSelect value="alpha" onValueChange={values} />);
    const trigger = screen.getByRole("combobox", { name: "Mode" });
    fireEvent.keyDown(trigger, { key: "ArrowDown" });
    const listbox = await screen.findByRole("listbox");
    const alpha = screen.getByRole("option", { name: "Alpha" });
    const gamma = screen.getByRole("option", { name: "Gamma" });
    expect(alpha.getAttribute("aria-selected")).toBe("true");
    fireEvent.keyDown(listbox, { key: "ArrowDown" });
    expect(listbox.getAttribute("aria-activedescendant")).toBe(gamma.id);
    expect(alpha.getAttribute("aria-selected")).toBe("true");
    expect(gamma.getAttribute("aria-selected")).toBe("false");
    fireEvent.keyDown(listbox, { key: "Enter" });
    expect(values).toHaveBeenCalledTimes(1);
    expect(values).toHaveBeenCalledWith("gamma");
  });
  it("supports Home, End, Page, Space, and locale-invariant typeahead", async () => {
    const values = vi.fn();
    render(
      <Select id="select-component-test-2" onValueChange={values}>
        <SelectTrigger aria-label="City">
          <SelectValue placeholder="Choose" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="istanbul">İSTANBUL</SelectItem>
          {Array.from({ length: 12 }, (_, index) => (
            <SelectItem key={index} value={`row-${index}`}>
              Row {index}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>,
    );
    const trigger = screen.getByRole("combobox", { name: "City" });
    fireEvent.keyDown(trigger, { key: "i" });
    const listbox = await screen.findByRole("listbox");
    expect(listbox.getAttribute("aria-activedescendant")).toBe(screen.getByRole("option", { name: "İSTANBUL" }).id);
    fireEvent.keyDown(listbox, { key: "End" });
    expect(listbox.getAttribute("aria-activedescendant")).toBe(screen.getByRole("option", { name: "Row 11" }).id);
    fireEvent.keyDown(listbox, { key: "Home" });
    fireEvent.keyDown(listbox, { key: "PageDown" });
    expect(listbox.getAttribute("aria-activedescendant")).toBe(screen.getByRole("option", { name: "Row 9" }).id);
    fireEvent.keyDown(listbox, { key: " " });
    expect(values).toHaveBeenCalledWith("row-9");
  });
  it("associates owned labels and groups with injective option IDs", async () => {
    render(
      <Select id="mode.select" showLabel defaultOpen defaultValue="one">
        <SelectTrigger>
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            <SelectLabel>Group A</SelectLabel>
            <SelectItem value="one">Duplicate</SelectItem>
            <SelectItem value="two">Duplicate</SelectItem>
          </SelectGroup>
          <SelectSeparator aria-label="More" />
        </SelectContent>
      </Select>,
    );
    const trigger = screen.getByRole("combobox");
    const listbox = await screen.findByRole("listbox");
    const group = screen.getByRole("group", { name: "Group A" });
    const options = screen.getAllByRole("option", { name: "Duplicate" });
    expect(trigger.getAttribute("aria-labelledby")).toBe("mode.select-label");
    expect(listbox.getAttribute("aria-labelledby")).toBe("mode.select-label");
    expect(group.getAttribute("aria-labelledby")).toBe(group.querySelector('[data-slot="select-label"]')?.id);
    expect(options[0]!.id).not.toBe(options[1]!.id);
  });
}
