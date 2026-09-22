// #region 🔌️Adapters
import * as React from "react";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import Ajv2020 from "ajv/dist/2020.js";
import { afterEach, describe, expect, it, vi } from "vitest";
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectScrollDownButton, SelectSeparator, SelectTrigger, SelectValue, resolveSelectPlacement } from "../../🟦️.tsx";
import { Dialog, DialogContent, DialogPortal, DialogTitle } from "../../../💬️Dialog/🟦️.tsx";
import selectPopupGeometryFixture from "../../../../🧫️fixtures/🔽️select-popup-geometry/🔣️.json";
import selectPopupGeometrySchema from "../../../../🧬️schema/🔽️select-popup-geometry/🔣️.json";
import retainedSelectOriginFixture from "../../../../🧫️fixtures/🔽️retained-select-origin/🔣️.json";
import retainedSelectOriginSchema from "../../../../🧬️schema/🔽️retained-select-origin/🔣️.json";
import retainedSelectAccessibilityFixture from "../../../../🧫️fixtures/♿️retained-select-accessibility/🔣️.json";
import retainedSelectAccessibilitySchema from "../../../../🧬️schema/♿️retained-select-accessibility/🔣️.json";
// #endregion 🔌️Adapters

// #region ☑️SelectMatrix
afterEach(() => cleanup());

function BasicSelect(props: Omit<React.ComponentProps<typeof Select>, "id"> = {}): React.ReactElement {
  return (
    <Select id="select-component-test-1" {...props}>
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
  it("mounts listbox options only while expanded and commits one option through the React authority", async () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(retainedSelectAccessibilitySchema);
    expect(validate(retainedSelectAccessibilityFixture), JSON.stringify(validate.errors)).toBe(true);
    const committed = vi.fn();
    render(
      <Select id="retained-select-accessibility" defaultValue={retainedSelectAccessibilityFixture.select.value} onValueChange={committed}>
        <SelectTrigger aria-label={retainedSelectAccessibilityFixture.select.label}><SelectValue /></SelectTrigger>
        <SelectContent>
          {retainedSelectAccessibilityFixture.select.options.map((option) => <SelectItem key={option.value} value={option.value}>{option.label}</SelectItem>)}
        </SelectContent>
      </Select>,
    );
    const trigger = screen.getByRole("combobox", { name: retainedSelectAccessibilityFixture.select.label });
    expect([trigger.getAttribute("role")]).toEqual(retainedSelectAccessibilityFixture.closedRoles);
    expect(screen.queryByRole("listbox")).toBeNull();
    fireEvent.click(trigger);
    const listbox = await screen.findByRole("listbox");
    const options = screen.getAllByRole("option");
    expect([trigger, listbox, ...options].map((node) => node.getAttribute("role"))).toEqual(retainedSelectAccessibilityFixture.openRoles);
    expect(options.map((node) => node.getAttribute("aria-selected") === "true")).toEqual(retainedSelectAccessibilityFixture.select.options.map((option) => option.selected));
    fireEvent.click(options.find((node) => node.textContent === "Dark")!);
    expect(committed).toHaveBeenCalledTimes(retainedSelectAccessibilityFixture.activation.expectedActions);
    expect(committed).toHaveBeenCalledWith(retainedSelectAccessibilityFixture.activation.expectedValue);
    expect(screen.queryByRole("listbox")).toBeNull();
    await waitFor(() => expect(document.activeElement).toBe(trigger));
  });

  it("mounts scoped bottom and flipped-top content at the fixture's viewport-local origin", async () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(retainedSelectOriginSchema);
    expect(validate(retainedSelectOriginFixture), JSON.stringify(validate.errors)).toBe(true);
    const [originX, originY] = retainedSelectOriginFixture.viewport.origin;
    const [viewportWidth, viewportHeight] = retainedSelectOriginFixture.viewport.size;
    for (const testCase of retainedSelectOriginFixture.cases) {
      const root = document.createElement("section"), app = document.createElement("div"), layer = document.createElement("div");
      root.style.position = "relative";
      layer.style.position = "absolute";
      root.append(app, layer);
      document.body.appendChild(root);
      root.getBoundingClientRect = () => ({ x: originX, y: originY, top: originY, right: originX + viewportWidth, bottom: originY + viewportHeight, left: originX, width: viewportWidth, height: viewportHeight, toJSON: () => ({}) });
      const view = render(
        <Dialog defaultOpen isolationRoot={root}>
          <DialogPortal container={layer}>
            <DialogContent showCloseButton={false}>
              <DialogTitle>Select origin</DialogTitle>
            </DialogContent>
          </DialogPortal>
          <Select id={`select-origin-${testCase.id}`} defaultOpen defaultValue="alpha">
            <SelectTrigger aria-label={testCase.id}>
              <SelectValue />
            </SelectTrigger>
            <SelectContent sideOffset={retainedSelectOriginFixture.sideOffset} collisionPadding={retainedSelectOriginFixture.collisionPadding}>
              <SelectItem value="alpha">Alpha</SelectItem>
              <SelectItem value="beta">Beta</SelectItem>
              <SelectItem value="gamma">Gamma</SelectItem>
            </SelectContent>
          </Select>
        </Dialog>,
        { container: app, baseElement: root },
      );
      try {
        const trigger = view.getByRole("combobox", { name: testCase.id, hidden: true });
        const content = await view.findByRole("listbox");
        const [triggerX, triggerY, triggerWidth, triggerHeight] = testCase.triggerLocal;
        trigger.getBoundingClientRect = () => ({
          x: originX + triggerX,
          y: originY + triggerY,
          top: originY + triggerY,
          right: originX + triggerX + triggerWidth,
          bottom: originY + triggerY + triggerHeight,
          left: originX + triggerX,
          width: triggerWidth,
          height: triggerHeight,
          toJSON: () => ({}),
        });
        content.getBoundingClientRect = () => ({ x: 0, y: 0, top: 0, right: testCase.menuLocal[2], bottom: testCase.menuLocal[3], left: 0, width: testCase.menuLocal[2], height: testCase.menuLocal[3], toJSON: () => ({}) });
        fireEvent(window, new Event("resize"));
        await waitFor(() => expect(content.style.visibility).not.toBe("hidden"));
        expect(content.dataset.side).toBe(testCase.side);
        const mounted = [Number.parseFloat(content.style.left), Number.parseFloat(content.style.top)];
        expect(mounted[0]).toBeCloseTo(testCase.menuLocal[0]);
        expect(mounted[1]).toBeCloseTo(testCase.menuLocal[1]);
        const global = [originX + mounted[0], originY + mounted[1], Number.parseFloat(content.style.getPropertyValue("--semio-select-trigger-width")), testCase.menuLocal[3]];
        global.forEach((value, index) => expect(value).toBeCloseTo(testCase.menuGlobal[index]!));
        expect(view.getAllByRole("option")).toHaveLength(retainedSelectOriginFixture.row.itemCount);
      } finally {
        view.unmount();
        root.remove();
      }
    }
  });

  it("keeps the measured popup minimum, border, scroll bands, viewport, and selected-row reveal contract", async () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(selectPopupGeometrySchema);
    expect(validate(selectPopupGeometryFixture), JSON.stringify(validate.errors)).toBe(true);
    const [normal, constrained] = selectPopupGeometryFixture.cases;
    expect(normal.popupWidth).toBe(Math.max(selectPopupGeometryFixture.triggerWidth, selectPopupGeometryFixture.minimumContentWidth));
    expect(normal.popupHeight).toBe(selectPopupGeometryFixture.borderWidth * 2 + selectPopupGeometryFixture.scrollBandHeight * 2 + normal.scrollViewportHeight);
    expect(constrained.popupHeight).toBe(selectPopupGeometryFixture.borderWidth * 2 + selectPopupGeometryFixture.scrollBandHeight * 2 + constrained.scrollViewportHeight);
    expect(constrained.initialScroll + constrained.wheelDelta).toBeGreaterThan(constrained.scrollAfterWheel);

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

  it("honors preventable Escape and outside dismissal, then preserves outside focus", async () => {
    const escaped = vi.fn((event: { preventDefault(): void }) => event.preventDefault());
    render(
      <Select id="select-component-test-4" defaultOpen onOpenChange={vi.fn()}>
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
        <Select id="select-component-test-5" open onOpenChange={first}>
          <SelectTrigger aria-label="First">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="one">One</SelectItem>
          </SelectContent>
        </Select>
        <Select id="select-component-test-6" open onOpenChange={second}>
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

  it("does not let a page Select consume events owned by a scoped application dialog", async () => {
    const pageChange = vi.fn(), scopedChange = vi.fn();
    const page = render(<BasicSelect open onOpenChange={pageChange} />);
    const root = document.createElement("section"), app = document.createElement("div"), layer = document.createElement("div");
    root.style.position = "relative";
    layer.style.position = "absolute";
    root.append(app, layer);
    document.body.appendChild(root);
    const scoped = render(
      <Dialog defaultOpen isolationRoot={root} onOpenChange={scopedChange}>
        <DialogPortal container={layer}>
          <DialogContent showCloseButton={false}>
            <DialogTitle>Scoped application</DialogTitle>
            <button type="button">Scoped action</button>
          </DialogContent>
        </DialogPortal>
      </Dialog>,
      { container: app, baseElement: root },
    );
    try {
      await page.findByRole("listbox");
      const action = scoped.getByRole("button", { name: "Scoped action" });
      fireEvent.pointerDown(action);
      expect(pageChange).not.toHaveBeenCalled();
      fireEvent.keyDown(action, { key: "Escape" });
      expect(pageChange).not.toHaveBeenCalled();
      expect(scopedChange).toHaveBeenCalledWith(false);
    } finally {
      scoped.unmount();
      page.unmount();
      root.remove();
    }
  });

  it("routes Escape to a logically nested portal before its parent", async () => {
    const parentChange = vi.fn();
    const childChange = vi.fn();
    const Nested = ({ childOpen }: { childOpen: boolean }) => (
      <Select id="select-component-test-7" open onOpenChange={parentChange} value="parent">
        <SelectTrigger aria-label="Parent">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="parent">Parent option</SelectItem>
        </SelectContent>
        <Select id="select-component-test-8" open={childOpen} onOpenChange={childChange} value="child">
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
});
// #endregion ☑️SelectMatrix
