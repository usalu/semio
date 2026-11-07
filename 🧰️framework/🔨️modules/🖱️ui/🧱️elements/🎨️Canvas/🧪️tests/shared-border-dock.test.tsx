import * as React from "react";
import { readFileSync, existsSync } from "node:fs";
import Decimal from "decimal.js";
import Ajv from "ajv";
import schema from "./dock-cases.schema.json";
import { Canvas, HorizontalWindows, VerticalWindows, Layout, Mode } from "@semio-tech/ui-react";
import { render, fireEvent, cleanup, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { renderToStaticMarkup } from "react-dom/server";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup, RESIZABLE_HIT_TARGET_MIN_FINE_PX, RESIZABLE_HIT_TARGET_MIN_COARSE_PX } from "../../↔️Resizable/🟦️.tsx";
import { computeModeSplitPreviewInBody } from "../🎯️drop/🟦️.ts";
import { applyAxisGroupLayoutDelta, resolveJoinCornerResizeDeltas } from "../🔗️join-corners/🟦️.ts";
import fixture from "./dock-cases.json";
import controlBorders from "./control-border.cases.json";
import controlBorderSchema from "./control-border.schema.json";
import { Input } from "../../✏️Input/🟦️.tsx";
import { Textarea } from "../../🔤️Textarea/🟦️.tsx";
import { Stepper } from "../../🪜️Stepper/🟦️.tsx";
import { Search } from "../../🔍️WindowSearch/🟦️.tsx";

afterEach(() => { cleanup(); vi.restoreAllMocks(); });

function source(relative: string) {
  return readFileSync(new URL(relative, import.meta.url), "utf8");
}

function split(orientation: "horizontal" | "vertical", sizes = [50, 50], extent = 200) {
  return <ResizablePanelGroup orientation={orientation} defaultLayout={Object.fromEntries(sizes.map((size, i) => [String(i), size]))} style={orientation === "horizontal" ? { width: extent, height: 100 } : { width: 100, height: extent }}>
    {sizes.flatMap((size, i) => [i > 0 ? <ResizableHandle key={`s${i}`} orientation={orientation} /> : null, <ResizablePanel key={i} id={String(i)} defaultSize={size}><button>Control {i}</button></ResizablePanel>])}
  </ResizablePanelGroup>;
}

describe("flush dock boundaries", () => {
  it("registers text fields and the stepper as shared frames with single internal seams", () => {
    const validate = new Ajv().compile(controlBorderSchema);
    expect(validate(controlBorders), JSON.stringify(validate.errors)).toBe(true);
    const controls = { input: <Input id="field.input" />, textarea: <Textarea id="field.textarea" />, stepper: <Stepper id="field.stepper" aria-label="Value" unit="mm" /> };
    for (const { control, slot } of controlBorders.frames) {
      const { container, unmount } = render(controls[control as keyof typeof controls]);
      const frame = container.querySelector(`[data-slot="${slot}"]`)!;
      expect(frame.hasAttribute("data-shared-border-frame")).toBe(true);
      expect(frame.getAttribute("data-shared-border-edges")).toBe(controlBorders.edges);
      if (control === "stepper") expect(frame.querySelectorAll(":scope > [data-shared-border-cell]")).toHaveLength(controlBorders.stepperCells);
      unmount();
    }
  });

  it("keeps disabled and collapsed fields registered without advertising hoverable frames", () => {
    for (const control of [<Input id="field.input" disabled showLabel />, <Textarea id="field.textarea" disabled showLabel />, <Stepper id="field.stepper" disabled />]) {
      const { container, unmount } = render(control);
      const frame = container.querySelector('[data-shared-border-frame]')!;
      expect(frame.getAttribute("aria-disabled")).toBe("true");
      if (frame.dataset.slot !== "stepper-group") expect(frame.querySelector('[data-collapsed="true"]')).not.toBeNull();
      unmount();
    }
  });

  it("paints one shared field seam and transfers its interaction state", async () => {
    const { width, height, seamLength } = controlBorders.geometry;
    vi.spyOn(HTMLElement.prototype, "offsetWidth", "get").mockReturnValue(width);
    vi.spyOn(HTMLElement.prototype, "offsetHeight", "get").mockReturnValue(height);
    vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
      const slot = this.dataset.slot;
      return slot === "input-root" ? new DOMRect(0, 0, width, height) : slot === "textarea-root" ? new DOMRect(width, 0, width, height) : slot === "stepper-group" ? new DOMRect(2 * width, 0, width, height) : new DOMRect();
    });
    const { container, unmount, rerender } = render(<div><Input id="field.first" /><Textarea id="field.second" /><Stepper id="field.third" /></div>);
    const frames = [...container.querySelectorAll<HTMLElement>('[data-shared-border-frame]')];
    const seam = (seamX = width) => frames.flatMap((frame, index) => [...frame.querySelectorAll<SVGPathElement>('[data-shared-border-segment]')].flatMap(path => {
      const points = [...(path.getAttribute("d") ?? "").matchAll(/[ML]\s*([\d.e+-]+)\s+([\d.e+-]+)/gi)].map(match => ({ x: Number(match[1]) + index * width, y: Number(match[2]) }));
      return points.slice(1).flatMap((point, i) => point.x === seamX && points[i]!.x === seamX && point.y !== points[i]!.y ? [{ length: Math.abs(point.y - points[i]!.y), state: path.getAttribute("data-shared-border-state") }] : []);
    }));
    await waitFor(() => expect(new Decimal(seam().reduce((sum, part) => sum + part.length, 0)).toNumber()).toBe(seamLength));
    await waitFor(() => expect(new Decimal(seam(2 * width).reduce((sum, part) => sum + part.length, 0)).toNumber()).toBe(seamLength));
    const first = container.querySelector<HTMLInputElement>('[data-slot="input"]')!;
    const second = container.querySelector<HTMLTextAreaElement>('[data-slot="textarea"]')!;
    fireEvent.focusIn(first);
    first.focus();
    fireEvent.pointerOver(second);
    await waitFor(() => expect(seam().every(part => part.state === "hover")).toBe(true));
    fireEvent.pointerOut(second);
    fireEvent.pointerOver(first);
    await waitFor(() => expect(seam().every(part => part.state === "hover")).toBe(true));
    fireEvent.pointerOut(first);
    first.blur();
    rerender(<div><Input id="field.first" /><Textarea id="field.second" disabled /><Stepper id="field.third" /></div>);
    fireEvent.pointerOver(second);
    await waitFor(() => expect(seam().every(part => part.state !== "hover")).toBe(true));
    expect(frames.map(frame => frame.offsetWidth)).toEqual([width, width, width]);
    unmount();
    expect(document.querySelector('[data-shared-border-overlay]')).toBeNull();
  });
  it("shares the Search seam on hover and restores the input edge when suggestions disappear", async () => {
    const { width, height, seamLength } = controlBorders.geometry;
    vi.spyOn(HTMLElement.prototype, "offsetWidth", "get").mockImplementation(function (this: HTMLElement) {
      return this.dataset.slot === "search-possibles-toggle" ? height : width;
    });
    vi.spyOn(HTMLElement.prototype, "offsetHeight", "get").mockReturnValue(height);
    vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
      const row = this.closest<HTMLElement>('[data-slot="search-row"]');
      const gap = row?.classList.contains("gap-half") ? 4 : 0;
      if (this.dataset.slot === "input-root") return new DOMRect(0, 0, width, height);
      if (this.dataset.slot === "search-possibles-toggle") return new DOMRect(width + gap, 0, height, height);
      return new DOMRect();
    });
    const possibles = [{ id: "possible", label: "Possible" }];
    const { container, rerender, unmount } = render(<Search input={{}} possibles={possibles} active />);
    const inputFrame = container.querySelector<HTMLElement>('[data-slot="input-root"]')!;
    const toggle = container.querySelector<HTMLElement>('[data-slot="search-possibles-toggle"]')!;
    const vertical = (frame: HTMLElement, offset: number, x: number) => [...frame.querySelectorAll<SVGPathElement>('[data-shared-border-segment]')].flatMap(path => {
      const points = [...(path.getAttribute("d") ?? "").matchAll(/[ML]\s*([\d.e+-]+)\s+([\d.e+-]+)/gi)].map(match => ({ x: Number(match[1]) + offset, y: Number(match[2]) }));
      return points.slice(1).flatMap((point, i) => point.x === x && points[i]!.x === x && point.y !== points[i]!.y ? [{ length: Math.abs(point.y - points[i]!.y), state: path.getAttribute("data-shared-border-state") }] : []);
    });
    const seam = () => [...vertical(inputFrame, 0, width), ...vertical(toggle, toggle.getBoundingClientRect().left, width)];
    expect(new Decimal(height).minus(2).toNumber()).toBe(seamLength);
    await waitFor(() => expect(seam().reduce((sum, part) => sum + part.length, 0)).toBe(seamLength));
    fireEvent.pointerOver(toggle);
    await waitFor(() => {
      const owned = vertical(toggle, toggle.getBoundingClientRect().left, width);
      expect(owned.reduce((sum, part) => sum + part.length, 0)).toBe(seamLength);
      expect(owned.every(part => part.state === "hover")).toBe(true);
      expect(vertical(inputFrame, 0, width)).toHaveLength(0);
    });
    fireEvent.pointerOut(toggle);
    rerender(<Search input={{}} possibles={[]} active />);
    await waitFor(() => expect(container.querySelector('[data-slot="search-possibles-toggle"]')).toBeNull());
    await waitFor(() => expect(vertical(inputFrame, 0, width - 1).reduce((sum, part) => sum + part.length, 0)).toBe(seamLength));
    unmount();
  });

  it("validates language-neutral flush layout vectors", () => {
    const validate = new Ajv().compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  it("has no macro canvas padding, tiled gaps or mode inset", () => {
    const canvas = source("../🟦️.tsx");
    expect(canvas).not.toContain("h-full w-full p-[var(--padding-standard)]");
    expect(canvas).not.toContain("h-full w-full gap-[var(--padding-standard)]");
    expect(canvas).toContain('MODE_CANVAS_INSET_CLASS = "p-0"');
    const dock = source("../⚓️dock/🟦️.tsx");
    expect(dock).toContain('overflow-hidden p-0", windowBodyFrameClass');
    expect(dock).toContain("registerStackDropTargets(stackPath, corner, cornerTabBarRefs.current[corner], stackRef.current)");
    for (const Windows of [HorizontalWindows, VerticalWindows]) {
      const { container, unmount } = render(<Canvas><Windows><div>Pane</div><div>Pane</div></Windows></Canvas>);
      const canvas = container.querySelector('[data-slot="canvas"]')!;
      expect(canvas.className).toContain("p-0");
      expect(canvas.firstElementChild!.className).toContain("gap-0");
      unmount();
    }
  });

  it("keeps navbar, tiled stacks and footer adjacent in the real layout composition", () => {
    const windows = [{ id: "a", title: "A", children: <div>A</div> }, { id: "b", title: "B", children: <div>B</div> }];
    const { container } = render(<Layout navbar={<div data-testid="navbar" />} footer={<div data-testid="footer" />} canvas={<Canvas><Mode windows={windows} activeWindowId="a" layout={{ kind: "row", children: windows.map(({ id }) => ({ kind: "stack", children: [{ kind: "window", id }] })) }} /></Canvas>} />);
    const body = container.querySelector('[data-slot="mode-body"]')!;
    expect(body.className).toContain("p-0");
    const stacks = body.querySelectorAll('[data-slot="mode-dock-stack"]');
    expect(stacks).toHaveLength(2);
    expect(body.querySelector<HTMLElement>('[data-slot="resizable-handle"]')!.style.width).toBe("0px");
    for (const stack of stacks) expect(stack.querySelector('[data-slot="mode-dock-stack-body"]')!.className).toContain("p-0");
    const layout = container.querySelector('[data-slot="layout"]')!;
    expect(layout.children[0]!.querySelector('[data-testid="navbar"]')).not.toBeNull();
    expect(layout.children[1]!.contains(body)).toBe(true);
    expect(layout.children[2]!.querySelector('[data-testid="footer"]')).not.toBeNull();
  });

  it("propagates controlled Mode selection to the complete measured outline before interaction, then transfers activation and lets hover win", async () => {
    vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
      const stack = this.closest<HTMLElement>('[data-slot="mode-dock-stack"]');
      const x = stack?.dataset.stackPath === "1" ? 600 : 16;
      if (this.dataset.slot === "mode-dock-stack") return new DOMRect(x, 46, 584, 370);
      if (this.dataset.slot === "mode-dock-tab-cap") return new DOMRect(x, 46, 108, 30);
      if (this.dataset.slot === "mode-dock-tabbar") return new DOMRect(x, 46, 584, 30);
      if (this.dataset.slot === "mode-dock-tab-gap") return new DOMRect(x + 108, 46, 476, 30);
      return stack ? new DOMRect() : new DOMRect(16, 46, 1168, 370);
    });
    const changed = vi.fn();
    function AppFixture() {
      const [activeWindowId, setActiveWindowId] = React.useState<string | null>("left");
      return <Mode windows={[{ id: "left", iconId: "app-window", children: <div>Design Left</div> }, { id: "right", iconId: "app-window", children: <div>Design Right</div> }]} activeWindowId={activeWindowId} onActiveWindowChange={id => { changed(id); setActiveWindowId(id); }} />;
    }
    const { container } = render(<AppFixture />);
    const stacks = [...container.querySelectorAll<HTMLElement>('[data-slot="mode-dock-stack"]')];
    const border = (index: number) => stacks[index]!.querySelector('[data-slot="mode-dock-silhouette-border"]')!;
    const paths = (index: number, state: string) => [...border(index).querySelectorAll(`path[data-shared-border-state="${state}"]`)].map(path => path.getAttribute("d") ?? "").join(" ");
    const complete = (index: number, state: string) => {
      const d = paths(index, state);
      expect(d).not.toBe("");
      const commands = [...d.matchAll(/([ML])\s*([\d.e+-]+)[ ,]+([\d.e+-]+)/gi)].map(match => ({ move: match[1] === "M", x: Number(match[2]), y: Number(match[3]) }));
      const left = index === 0 ? 1 : 0, right = index === 0 ? 584 : 583;
      const edges = [[1, 1, 108, 1], [108, 1, 108, 31], [108, 31, right, 31], [right, 31, right, 369], [left, 369, right, 369], [left, 31, left, 369], [1, 1, 1, 31]];
      for (const [x1, y1, x2, y2] of edges as [number, number, number, number][]) {
        const horizontal = y1 === y2, lo = horizontal ? x1 : y1, hi = horizontal ? x2 : y2;
        let covered = new Decimal(0);
        commands.forEach((to, i) => {
          const from = commands[i - 1];
          if (!from || to.move || (horizontal ? from.y !== y1 || to.y !== y1 : from.x !== x1 || to.x !== x1)) return;
          const a = horizontal ? from.x : from.y, b = horizontal ? to.x : to.y;
          covered = covered.add(Math.max(0, Math.min(hi, Math.max(a, b)) - Math.max(lo, Math.min(a, b))));
        });
        expect(covered.toNumber(), `${state}: ${JSON.stringify([x1, y1, x2, y2])} in ${d}`).toBeCloseTo(hi - lo, 4);
      }
    };
    expect(stacks).toHaveLength(2);
    expect(stacks[0]!.querySelector('[role="tab"]')!.getAttribute("aria-selected")).toBe("true");
    await waitFor(() => {
      expect(border(0).getAttribute("data-kind")).toBe("active");
      expect(border(1).getAttribute("data-kind")).toBe("normal");
      complete(0, "active");
      expect(paths(1, "active")).toBe("");
    });
    fireEvent.click(stacks[1]!.querySelector('[role="tab"]')!);
    expect(changed).toHaveBeenCalledWith("right");
    await waitFor(() => {
      expect(border(0).getAttribute("data-kind")).toBe("normal");
      expect(border(1).getAttribute("data-kind")).toBe("active");
      complete(1, "active");
      expect(paths(0, "active")).toBe("");
    });
    fireEvent.pointerOver(stacks[1]!.querySelector('[role="tab"]')!);
    await waitFor(() => { complete(1, "hover"); expect(paths(1, "active")).toBe(""); });
    fireEvent.pointerOut(stacks[1]!, { relatedTarget: document.body });
    await waitFor(() => complete(1, "active"));
  });

  it.each(["horizontal", "vertical"] as const)("%s splitter reserves zero layout space but retains keyboard semantics", (orientation) => {
    const { container } = render(split(orientation));
    const handle = container.querySelector<HTMLElement>('[data-slot="resizable-handle"]')!;
    expect(handle.style[orientation === "horizontal" ? "width" : "height"]).toBe("0px");
    expect(handle.getAttribute("role")).toBe("separator");
    expect(handle.tabIndex).toBe(0);
    expect(handle.className).not.toMatch(/(?:w|h)-single/);
    expect(RESIZABLE_HIT_TARGET_MIN_FINE_PX).toBe(fixture.fineHitSize);
    expect(RESIZABLE_HIT_TARGET_MIN_COARSE_PX).toBe(fixture.coarseHitSize);
  });

  it("does not capture window controls inside the expanded resize hit region", () => {
    vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
      const slot = this.dataset.slot;
      if (slot === "resizable-panel-group") return new DOMRect(0, 0, 200, 100);
      if (slot === "resizable-handle") return new DOMRect(100, 0, 0, 100);
      if (slot === "resizable-panel") return new DOMRect(this.id === "0" ? 0 : 100, 0, 100, 100);
      return new DOMRect(90, 0, 10, 20);
    });
    const { container } = render(split("horizontal"));
    const control = container.querySelector("button")!;
    expect(fireEvent.pointerDown(control, { button: 0, clientX: 96, clientY: 10, pointerId: 1, isPrimary: true })).toBe(true);
    const pane = container.querySelector('[data-slot="resizable-panel"]')!;
    for (const [attribute, value] of [["role", "textbox"], ["role", "combobox"], ["contenteditable", ""], ["contenteditable", "plaintext-only"]]) {
      const target = document.createElement("div");
      target.setAttribute(attribute, value);
      pane.appendChild(target);
      expect(fireEvent.pointerDown(target, { button: 0, clientX: 96, clientY: 10, pointerId: 1, isPrimary: true }), `${attribute}=${value}`).toBe(true);
      target.remove();
    }
    expect(fireEvent.pointerDown(pane, { button: 0, clientX: 96, clientY: 50, pointerId: 1, isPrimary: true })).toBe(false);
    fireEvent.pointerUp(pane, { pointerId: 1 });
  });

  it.each(["horizontal", "vertical"] as const)("%s splitters retain a centered 20px invisible hit region", (orientation) => {
    const horizontal = orientation === "horizontal";
    vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
      if (this.dataset.slot === "resizable-panel-group") return new DOMRect(0, 0, horizontal ? 200 : 100, horizontal ? 100 : 200);
      if (this.dataset.slot === "resizable-handle") return new DOMRect(horizontal ? 100 : 0, horizontal ? 0 : 100, horizontal ? 0 : 100, horizontal ? 100 : 0);
      return new DOMRect(horizontal && this.id === "1" ? 100 : 0, !horizontal && this.id === "1" ? 100 : 0, 100, 100);
    });
    const { container } = render(split(orientation));
    const panes = container.querySelectorAll('[data-slot="resizable-panel"]');
    for (const row of fixture.hitCases) {
      const pane = panes[row.offset < 0 ? 0 : 1]!;
      const event = { button: 0, clientX: horizontal ? 100 + row.offset : 50, clientY: horizontal ? 50 : 100 + row.offset, pointerId: 3, isPrimary: true };
      expect(!fireEvent.pointerDown(pane, event), JSON.stringify(row)).toBe(row.resize);
      fireEvent.pointerUp(pane, event);
    }
  });

  it("keeps corner hit areas invisible to controls and supports dual-axis dragging", () => {
    const resize = vi.fn();
    const spec = { parentKind: "row", mainAxisPath: "", mainSeparatorIndex: 1, crossAxisPath: "0", crossSeparatorIndex: 1, edgeSide: "leading", alongFraction: 0.5 } as const;
    vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
      if (this.dataset.slot === "resizable-corner") return new DOMRect(90, 0, 20, 20);
      return new DOMRect(0, 0, 200, 100);
    });
    const { container } = render(<ResizablePanelGroup><ResizablePanel id="0"><button>Close</button></ResizablePanel><ResizableHandle joinCorners={[spec]} onJoinCornerResize={resize} /><ResizablePanel id="1" /></ResizablePanelGroup>);
    expect(container.querySelector('[data-slot="resizable-corner"]')!.className).toContain("pointer-events-none");
    const control = container.querySelector("button")!;
    fireEvent.pointerDown(control, { button: 0, clientX: 96, clientY: 10, pointerId: 2 });
    fireEvent.pointerMove(window, { clientX: 106, clientY: 20, pointerId: 2 });
    expect(resize).not.toHaveBeenCalled();
    const pane = container.querySelector('[data-slot="resizable-panel"]')!;
    fireEvent.pointerDown(pane, { button: 0, clientX: 96, clientY: 10, pointerId: 2 });
    fireEvent.pointerMove(window, { clientX: 106, clientY: 20, pointerId: 2 });
    expect(resize).toHaveBeenCalledWith(spec, 10, 10);
    fireEvent.pointerCancel(window, { pointerId: 2 });
    fireEvent.pointerMove(window, { clientX: 116, clientY: 30, pointerId: 2 });
    expect(resize).toHaveBeenCalledTimes(1);
  });

  it.each(fixture.cases)("$id uses full extent for resize and half-panel previews", (row) => {
    const mainKind = row.orientation === "horizontal" ? "row" : "column";
    const deltas = resolveJoinCornerResizeDeltas(mainKind, row.delta, row.delta, row.extent, row.extent);
    const sizes = Object.fromEntries(row.sizes.map((size, i) => [String(i), size]));
    const resized = Object.values(applyAxisGroupLayoutDelta(sizes, "", 1, deltas.mainDeltaPct));
    expect(resized).toEqual(row.resized);
    const independentDelta = new Decimal(row.delta).div(row.extent).mul(100);
    expect(resized).toEqual(row.sizes.map((size, i) => new Decimal(size).add(i === 0 ? independentDelta : i === 1 ? independentDelta.negated() : 0).toNumber()));
    let accumulated = new Decimal(0);
    expect([0, ...row.sizes.map((size) => { accumulated = accumulated.add(new Decimal(row.extent).mul(size).div(100)); return accumulated.toNumber(); })]).toEqual(row.boundaries);
    const width = row.orientation === "horizontal" ? row.extent : 100;
    const height = row.orientation === "vertical" ? row.extent : 100;
    const first = computeModeSplitPreviewInBody(width, height, row.orientation === "horizontal" ? "left" : "top");
    const last = computeModeSplitPreviewInBody(width, height, row.orientation === "horizontal" ? "right" : "bottom");
    expect(first.left + first.width).toBe(row.orientation === "horizontal" ? last.left : width);
    expect(first.top + first.height).toBe(row.orientation === "vertical" ? last.top : height);
  });

  it("matches neutral physical boundaries in the independent browser flex implementation", async (context) => {
    const { chromium } = await import("playwright");
    if (!existsSync(chromium.executablePath())) context.skip("Playwright Chromium is not provisioned; no browser was installed by this task.");
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage();
      for (const row of fixture.cases) {
        const markup = renderToStaticMarkup(split(row.orientation as "horizontal" | "vertical", row.sizes, row.extent));
        await page.setContent(`<style>body{margin:0}[data-slot="resizable-handle"]{position:relative;flex-shrink:0}</style>${markup}`);
        const boundaries = await page.locator('[data-slot="resizable-panel"]').evaluateAll((panels, orientation) => {
          const rects = panels.map((panel) => panel.getBoundingClientRect());
          return [orientation === "horizontal" ? rects[0]!.left : rects[0]!.top, ...rects.map((rect) => orientation === "horizontal" ? rect.right : rect.bottom)];
        }, row.orientation);
        boundaries.forEach((boundary, i) => expect(boundary, row.id).toBeCloseTo(row.boundaries[i]!, 4));
      }
    } finally { await browser.close(); }
  });
});
