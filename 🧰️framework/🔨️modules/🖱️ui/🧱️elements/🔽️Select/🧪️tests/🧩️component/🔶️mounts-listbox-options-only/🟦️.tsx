/** 🔶️ The mounts listbox options only tests of the Select suite. */
import { createMemoryStoragePort } from "../../../../../🔨️modules/🗄️storage-port/🟦️.ts";
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { ShellScopeProvider, createShellScope } from "../../../../🐚️ShellScope/🟦️.tsx";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue, resolveSelectFrame } from "../../../🟦️.tsx";
import { Dialog, DialogContent, DialogPortal, DialogTitle } from "../../../../💬️Dialog/🟦️.tsx";
import retainedSelectOriginFixture from "../../../../../🧪️tests/🧫️fixtures/🔽️retained-select-origin/🔣️.json";
import retainedSelectAccessibilityFixture from "../../../../../🧪️tests/🧫️fixtures/♿️retained-select-accessibility/🔣️.json";

/** 🔷 Registers the mounts listbox options only tests of the Select suite. */
export function registerMountsListboxOptionsOnlyTests(): void {
  it("mounts listbox options only while expanded and commits one option through the React authority", async () => {
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
    expect(retainedSelectOriginFixture.row.menuHeight).toBe(
      retainedSelectOriginFixture.row.borderWidth * 2
      + retainedSelectOriginFixture.row.scrollBandHeight * 2
      + retainedSelectOriginFixture.row.padding * 2
      + (retainedSelectOriginFixture.row.lineHeight + retainedSelectOriginFixture.row.padding * 2) * retainedSelectOriginFixture.row.itemCount,
    );
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
  it("anchors an open listbox to a translated shell portal instead of the viewport", async () => {
    const frame = resolveSelectFrame({
      isolation: null,
      host: { left: 0, top: 0, width: 1920, height: 1080, positioned: true, transformed: true },
      viewport: { width: 1920, height: 1080 },
    });
    expect(frame).toEqual({ position: "absolute", originLeft: 0, originTop: 0, viewportWidth: 1920, viewportHeight: 1080 });
    expect(resolveSelectFrame({ isolation: null, host: { left: 0, top: 0, width: 1920, height: 1080, positioned: true, transformed: false }, viewport: { width: 800, height: 600 } }).position).toBe("fixed");

    const scope = createShellScope({ storage: createMemoryStoragePort(), initialLocale: "en" });
    const grid = document.createElement("div");
    const layer = document.createElement("div");
    layer.style.position = "absolute";
    grid.append(layer);
    document.body.append(grid);
    scope.portalLayerRef.current = layer;
    const box = (x: number, y: number, width: number, height: number): DOMRect => ({ x, y, top: y, right: x + width, bottom: y + height, left: x, width, height, toJSON: () => ({}) }) as DOMRect;
    layer.getBoundingClientRect = () => box(0, 0, 1920, 1080);
    const computed = window.getComputedStyle.bind(window);
    const styled = vi.spyOn(window, "getComputedStyle").mockImplementation((element: Element) => {
      const style = computed(element);
      if (element === grid) return new Proxy(style, { get: (target, prop, receiver) => (prop === "transform" ? "matrix(1, 0, 0, 1, -15360, -4320)" : Reflect.get(target, prop, receiver)) });
      return style;
    });
    try {
      render(
        <ShellScopeProvider scope={scope}>
          <Select id="select-translated-shell" defaultOpen defaultValue="alpha">
            <SelectTrigger aria-label="Example">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="alpha">Alpha</SelectItem>
              <SelectItem value="beta">Beta</SelectItem>
            </SelectContent>
          </Select>
        </ShellScopeProvider>,
      );
      const trigger = screen.getByRole("combobox", { name: "Example" });
      const content = await screen.findByRole("listbox");
      trigger.getBoundingClientRect = () => box(846, 3, 192, 22);
      content.getBoundingClientRect = () => box(0, 0, 192, 80);
      fireEvent(window, new Event("resize"));
      await waitFor(() => expect(content.style.visibility).not.toBe("hidden"));
      expect(content.style.position).toBe("absolute");
      expect(Number.parseFloat(content.style.left)).toBeCloseTo(846);
      expect(Number.parseFloat(content.style.top)).toBeCloseTo(29);
      expect(content.parentElement).toBe(layer);
    } finally {
      styled.mockRestore();
      grid.remove();
    }
  });
}
