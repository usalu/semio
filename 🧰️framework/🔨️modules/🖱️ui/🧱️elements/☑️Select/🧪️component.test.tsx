// #region 🔌️Adapters
import * as React from "react";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectScrollDownButton, SelectSeparator, SelectTrigger, SelectValue, resolveSelectPlacement } from "./🟦️component.tsx";
// #endregion 🔌️Adapters

// #region ☑️SelectMatrix
afterEach(() => cleanup());

function BasicSelect(props: React.ComponentProps<typeof Select> = {}): React.ReactElement {
  return (
    <Select {...props}>
      <SelectTrigger aria-label="Mode">
        <SelectValue placeholder="Choose" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="alpha">Alpha</SelectItem>
        <SelectItem value="beta" disabled>
          Beta
        </SelectItem>
        <SelectItem value="gamma">Gamma</SelectItem>
      </SelectContent>
    </Select>
  );
}

describe("Select", () => {
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
      <Select onValueChange={values}>
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

  it("honors preventable Escape and outside dismissal, then preserves outside focus", async () => {
    const escaped = vi.fn((event: { preventDefault(): void }) => event.preventDefault());
    render(
      <Select defaultOpen onOpenChange={vi.fn()}>
        <SelectTrigger aria-label="Dismiss">
          <SelectValue />
        </SelectTrigger>
        <SelectContent onEscapeKeyDown={escaped}>
          <SelectItem value="one">One</SelectItem>
        </SelectContent>
      </Select>,
    );
    await screen.findByRole("listbox");
    fireEvent.keyDown(document, { key: "Escape" });
    expect(escaped).toHaveBeenCalledTimes(1);
    expect(screen.getByRole("listbox")).toBeTruthy();
    const outside = document.createElement("button");
    document.body.appendChild(outside);
    fireEvent.pointerDown(outside);
    outside.focus();
    await waitFor(() => expect(screen.queryByRole("listbox")).toBeNull());
    expect(document.activeElement).toBe(outside);
    outside.remove();
  });

  it("dismisses only the deepest or most recently active open surface", async () => {
    const first = vi.fn();
    const second = vi.fn();
    render(
      <>
        <Select open onOpenChange={first}>
          <SelectTrigger aria-label="First">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="one">One</SelectItem>
          </SelectContent>
        </Select>
        <Select open onOpenChange={second}>
          <SelectTrigger aria-label="Second">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="two">Two</SelectItem>
          </SelectContent>
        </Select>
      </>,
    );
    await waitFor(() => expect(screen.getAllByRole("listbox")).toHaveLength(2));
    first.mockClear();
    second.mockClear();
    fireEvent.keyDown(document, { key: "Escape" });
    expect(second).toHaveBeenCalledWith(false);
    expect(first).not.toHaveBeenCalled();
    expect(screen.getAllByRole("listbox")).toHaveLength(2);
  });

  it("routes Escape to a logically nested portal before its parent", async () => {
    const parentChange = vi.fn();
    const childChange = vi.fn();
    const Nested = ({ childOpen }: { childOpen: boolean }) => (
      <Select open onOpenChange={parentChange} value="parent">
        <SelectTrigger aria-label="Parent">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="parent">Parent option</SelectItem>
        </SelectContent>
        <Select open={childOpen} onOpenChange={childChange} value="child">
          <SelectTrigger aria-label="Child">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="child">Child option</SelectItem>
          </SelectContent>
        </Select>
      </Select>
    );
    const view = render(<Nested childOpen />);
    await waitFor(() => expect(screen.getAllByRole("listbox")).toHaveLength(2));
    parentChange.mockClear();
    childChange.mockClear();
    fireEvent.keyDown(document, { key: "Escape" });
    expect(childChange).toHaveBeenCalledWith(false);
    expect(parentChange).not.toHaveBeenCalled();
    view.rerender(<Nested childOpen={false} />);
    fireEvent.keyDown(document, { key: "Escape" });
    expect(parentChange).toHaveBeenCalledWith(false);
  });

  it("does not hover-activate on touch and still commits the touch click once", async () => {
    const values = vi.fn();
    render(<BasicSelect defaultOpen value="alpha" onValueChange={values} />);
    const listbox = await screen.findByRole("listbox");
    const alpha = screen.getByRole("option", { name: "Alpha" });
    const gamma = screen.getByRole("option", { name: "Gamma" });
    expect(listbox.getAttribute("aria-activedescendant")).toBe(alpha.id);
    fireEvent.pointerMove(gamma, { pointerType: "touch" });
    expect(listbox.getAttribute("aria-activedescendant")).toBe(alpha.id);
    fireEvent.pointerDown(gamma, { pointerType: "touch" });
    fireEvent.click(gamma);
    expect(values).toHaveBeenCalledTimes(1);
    expect(values).toHaveBeenCalledWith("gamma");
  });

  it("uses owned RTL placement variables and scrolls the viewport without nested buttons", async () => {
    expect(resolveSelectPlacement({ top: 90, right: 120, bottom: 110, left: 80, width: 40, height: 20 }, { width: 100, height: 80 }, { width: 200, height: 140 }, "bottom", "start", 4, 8, true)).toMatchObject({
      side: "top",
      left: 20,
      availableHeight: 78,
    });
    const portal = document.createElement("div");
    document.body.append(portal);
    const view = render(
      <Select dir="rtl" defaultOpen defaultValue="one">
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
});
// #endregion ☑️SelectMatrix
