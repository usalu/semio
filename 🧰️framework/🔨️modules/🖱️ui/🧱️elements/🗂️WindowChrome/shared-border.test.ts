import { describe, expect, test, vi } from "vitest";
import * as React from "react";
import { createRoot } from "react-dom/client";
import { JSDOM } from "jsdom";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv/dist/2020.js";
import cases from "./compositor.cases.json";
import schema from "./compositor.schema.json";
import { BorderCompositor, isSharedBorderCell, retainSharedBorderRoot, sizeSharedBorderOverlay, useSharedBorderFrameRef, useSharedWindowBorder } from "./shared-border.ts";
import { WindowChromeFrameBorder, measureWindowFrameMetrics } from "./🟦️.tsx";
import { boxDrawnSides, type BorderSide, type OutlineBox } from "../../🔨️modules/📏️border-presentation/🟦️.ts";
import { createWindowFrameGeometry } from "../🔲️WindowFrame/🟦️.tsx";

const geometry = createWindowFrameGeometry({ width: 100, height: 100, top: { depth: 0, chips: [] }, bottom: { depth: 0, chips: [] } });

describe("overlay origin", () => {
  const overlay = (left: number, top: number) => {
    const parent = document.createElement("div");
    const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    parent.append(svg);
    vi.spyOn(parent, "getBoundingClientRect").mockReturnValue({ left, top, right: left + 59, bottom: top + 40, width: 59.078, height: 40, x: left, y: top, toJSON: () => ({}) } as DOMRect);
    return svg;
  };

  test("moves the overlay onto the snapped corner of its parent, so neighbours at different fractions share one seam coordinate", () => {
    const first = overlay(472.609, 10.25), second = overlay(531.688, 10.25);
    sizeSharedBorderOverlay(first, 59, 40);
    sizeSharedBorderOverlay(second, 59, 40);
    expect(first.style.getPropertyValue("translate")).toBe("0.391px -0.25px");
    expect(second.style.getPropertyValue("translate")).toBe("0.312px -0.25px");
    expect(472.609 + 0.391).toBeCloseTo(473, 3);
    expect(531.688 + 0.312).toBeCloseTo(532, 3);
  });

  test("leaves an overlay on a whole device pixel untouched", () => {
    const svg = overlay(100, 20);
    sizeSharedBorderOverlay(svg, 59, 40);
    expect(svg.style.getPropertyValue("translate")).toBe("");
  });
});

describe("frame overlay sizing", () => {
  test("sizes its overlay when it appears or its size changes, not on every render of the window", async () => {
    const host = document.createElement("div");
    document.body.append(host);
    const stack = document.createElement("div");
    host.append(stack);
    const reads = vi.spyOn(host, "getBoundingClientRect");
    const root = createRoot(host);
    const draw = (kind: "normal" | "active") => React.createElement(WindowChromeFrameBorder, { stack, geometry, borderKind: kind });
    await React.act(async () => root.render(draw("normal")));
    expect(reads).toHaveBeenCalled();
    reads.mockClear();
    await React.act(async () => root.render(draw("active")));
    await React.act(async () => root.render(draw("normal")));
    expect(reads).not.toHaveBeenCalled();
    await React.act(async () => root.unmount());
    host.remove();
  });
});

describe("panning the frame layer", () => {
  const rect = (left: number, top: number, width: number, height: number) => ({ left, top, width, height, right: left + width, bottom: top + height, x: left, y: top, toJSON: () => ({}) }) as DOMRect;

  test("repaints window outlines and frame overlays every pan frame without a React render and with one geometry read per element", async () => {
    const scope = document.createElement("div");
    scope.className = "semio-scope";
    scope.innerHTML = '<div id="clip" style="overflow:hidden"><div id="stack"></div><div id="frame" data-shared-border-frame></div></div><div id="mount"></div>';
    document.body.append(scope);
    const clip = scope.querySelector<HTMLElement>("#clip")!, stack = scope.querySelector<HTMLElement>("#stack")!, frame = scope.querySelector<HTMLElement>("#frame")!;
    const reads = { clip: 0, stack: 0, frame: 0 };
    let width = 100;
    clip.getBoundingClientRect = () => (reads.clip++, rect(0, 0, width, 100));
    stack.getBoundingClientRect = () => (reads.stack++, rect(0, 0, 100, 100));
    frame.getBoundingClientRect = () => (reads.frame++, rect(0, 120, 100, 40));
    const queue: FrameRequestCallback[] = [];
    const request = vi.spyOn(window, "requestAnimationFrame").mockImplementation(callback => { queue.push(callback); return queue.length; });
    const cancel = vi.spyOn(window, "cancelAnimationFrame").mockImplementation(() => undefined);
    const flush = async () => { const callbacks = queue.splice(0); await React.act(async () => callbacks.forEach(callback => callback(0))); };
    const renders = vi.fn();
    const root = createRoot(scope.querySelector("#mount")!);
    try {
      await React.act(async () => root.render(React.createElement(React.Profiler, { id: "frame", onRender: renders }, React.createElement(WindowChromeFrameBorder, { stack, geometry, borderKind: "normal" }))));
      await flush();
      await flush();
      const main = () => scope.querySelector("[data-shared-border-main]")?.getAttribute("d");
      const settled = renders.mock.calls.length;
      const paths = new Set<string | null | undefined>();
      for (const next of [90, 80, 70, 60]) {
        width = next;
        reads.clip = reads.stack = reads.frame = 0;
        const owner = retainSharedBorderRoot(scope);
        owner.compositor.schedule();
        owner.release();
        await flush();
        expect(reads).toEqual({ clip: 1, stack: 1, frame: 1 });
        paths.add(main());
      }
      expect(renders.mock.calls.length).toBe(settled);
      expect(paths.size).toBeGreaterThan(1);
    } finally {
      await React.act(async () => root.unmount());
      request.mockRestore(); cancel.mockRestore(); scope.remove();
    }
  });
});

describe("root border compositor", () => {
  test("neutral compositor schema", () => expect(new Ajv().validate(schema, cases)).toBe(true));
  test("overview frame work remains pending until each scheduled compositor frame commits", () => {
    const dom = new JSDOM('<div class="semio-scope"><div data-shared-border-frame></div></div>', { pretendToBeVisual: true });
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!;
    const queue = new Map<number, FrameRequestCallback>();
    let next = 0;
    dom.window.requestAnimationFrame = callback => { queue.set(++next, callback); return next; };
    dom.window.cancelAnimationFrame = id => { queue.delete(id); };
    const flush = () => { const callbacks = [...queue.values()]; queue.clear(); callbacks.forEach(callback => callback(0)); };
    const lease = retainSharedBorderRoot(root);
    try {
      expect(root.hasAttribute("data-shared-border-root")).toBe(true);
      expect(root.hasAttribute("data-shared-border-pending")).toBe(true);
      flush();
      expect(root.hasAttribute("data-shared-border-pending")).toBe(false);
      lease.compositor.schedule();
      expect(root.hasAttribute("data-shared-border-pending")).toBe(true);
      flush();
      expect(root.hasAttribute("data-shared-border-pending")).toBe(false);
      for (let i = 0; i < 3; i++) {
        flush();
        expect(root.hasAttribute("data-shared-border-pending")).toBe(false);
        expect(queue.size).toBe(0);
      }
      lease.compositor.schedule();
      expect(root.hasAttribute("data-shared-border-pending")).toBe(true);
    } finally {
      lease.release();
      expect(root.hasAttribute("data-shared-border-root")).toBe(false);
      expect(root.hasAttribute("data-shared-border-pending")).toBe(false);
      expect(queue.size).toBe(0);
      Object.assign(globalThis, previous);
      dom.window.close();
    }
  });
  test("reattaches the owned overlay after a translated text-only button updates", async () => {
    const scope = document.createElement("div");
    scope.className = "semio-scope";
    document.body.append(scope);
    const callbacks: FrameRequestCallback[] = [];
    const request = vi.spyOn(window, "requestAnimationFrame").mockImplementation(callback => callbacks.push(callback));
    const flush = () => callbacks.splice(0).forEach(callback => callback(0));
    const root = createRoot(scope);
    const fixture = cases.labelChange;
    const draw = (label: string) => React.createElement("button", { "data-shared-border-frame": "", "data-shared-border-state": "active" }, label);
    const lease = retainSharedBorderRoot(scope);
    try {
      await React.act(async () => root.render(draw(fixture.initial)));
      const button = scope.querySelector("button")!;
      button.getBoundingClientRect = () => new DOMRect(0, 0, 100, 30);
      flush();
      const overlay = button.querySelector("svg[data-shared-border-overlay]")!;
      expect(overlay.querySelectorAll("path").length).toBeGreaterThan(0);
      await React.act(async () => root.render(draw(fixture.updated)));
      expect(button.textContent).toBe(fixture.updated);
      expect(overlay.parentElement).toBeNull();
      lease.compositor.scheduleGeometry();
      flush();
      expect(button.querySelectorAll("svg[data-shared-border-overlay]")).toHaveLength(fixture.expectedOverlays);
      expect(overlay.parentElement).toBe(button);
    } finally {
      lease.release();
      await React.act(async () => root.unmount());
      request.mockRestore();
      scope.remove();
    }
  });

  test("marker-only sibling frames share ownership and independent participant lifetimes", () => {
    const dom = new JSDOM('<div class="semio-scope"><div id="left" data-shared-border-frame data-shared-border-interactive></div><div id="right" data-shared-border-frame data-shared-border-interactive></div></div><div id="other" class="semio-scope"></div>');
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!;
    const left = root.querySelector<HTMLElement>("#left")!, right = root.querySelector<HTMLElement>("#right")!;
    const size = cases.markerOnly.size;
    left.getBoundingClientRect = () => ({ left: 0, top: 0, width: size, height: size }) as DOMRect;
    right.getBoundingClientRect = () => ({ left: size, top: 0, width: size, height: size }) as DOMRect;
    const a = retainSharedBorderRoot(root), b = retainSharedBorderRoot(root);
    const other = retainSharedBorderRoot(dom.window.document.querySelector<HTMLElement>("#other")!);
    try {
      expect(a.compositor).toBe(b.compositor);
      expect(other.compositor).not.toBe(a.compositor);
      const paths = (element: HTMLElement) => [...element.querySelectorAll("path")].map(path => path.getAttribute("d"));
      expect(paths(left)).toContain(cases.markerOnly.normalSeam);
      expect(paths(right)).not.toContain(cases.markerOnly.hoverSeam);
      right.dispatchEvent(new dom.window.MouseEvent("pointerover", { bubbles: true }));
      expect(paths(left)).not.toContain(cases.markerOnly.normalSeam);
      expect(paths(right)).toContain(cases.markerOnly.hoverSeam);
      expect(right.querySelector('[data-shared-border-state="hover"]')).not.toBeNull();
      const windowLease = retainSharedBorderRoot(root);
      windowLease.compositor.windows.set("window", { element: left, geometry, state: "normal", update: () => {} });
      windowLease.compositor.windows.delete("window");
      windowLease.release();
      a.release(); a.release();
      expect(root.querySelectorAll("[data-shared-border-overlay]")).toHaveLength(2);
      b.release();
      a.compositor.schedule();
      expect(root.querySelector("[data-shared-border-overlay]")).toBeNull();
      expect(left.hasAttribute("data-shared-border-attached")).toBe(false);
      expect(left.style.position).toBe("");
      const replacement = retainSharedBorderRoot(root);
      expect(replacement.compositor).not.toBe(a.compositor);
      replacement.release();
    } finally {
      a.release(); b.release(); other.release(); Object.assign(globalThis, previous); dom.window.close();
    }
  });
  test("disabled frames do not acquire hover paint", () => {
    const dom = new JSDOM('<div class="semio-scope"><button data-shared-border-frame></button></div>');
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!;
    const button = root.querySelector<HTMLButtonElement>("button")!;
    button.getBoundingClientRect = () => ({ left: 0, top: 0, width: 100, height: 100 }) as DOMRect;
    const compositor = new BorderCompositor(root);
    compositor.schedule();
    try {
      for (const fixture of cases.disabled) {
        button.removeAttribute("disabled"); button.removeAttribute("aria-disabled");
        button.setAttribute(fixture.attribute, fixture.value);
        if (fixture.attribute === "disabled") expect(button.matches(":disabled")).toBe(true);
        button.dispatchEvent(new dom.window.MouseEvent("pointerover", { bubbles: true }));
        expect(button.querySelector("path")?.getAttribute("data-shared-border-state")).toBe(fixture.expected);
        button.dispatchEvent(new dom.window.MouseEvent("pointerout", { bubbles: true }));
      }
    } finally {
      compositor.dispose(); Object.assign(globalThis, previous); dom.window.close();
    }
  });
  test("disabled changes repaint an already hovered frame", async () => {
    const dom = new JSDOM('<div class="semio-scope"><button data-shared-border-frame></button></div>');
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!;
    const button = root.querySelector<HTMLButtonElement>("button")!;
    button.getBoundingClientRect = () => ({ left: 0, top: 0, width: 100, height: 100 }) as DOMRect;
    const compositor = new BorderCompositor(root);
    compositor.schedule();
    try {
      button.dispatchEvent(new dom.window.MouseEvent("pointerover", { bubbles: true }));
      expect(button.querySelector("path")?.getAttribute("data-shared-border-state")).toBe("hover");
      button.disabled = true;
      await Promise.resolve();
      expect(button.querySelector("path")?.getAttribute("data-shared-border-state")).toBe("normal");
      button.disabled = false;
      await Promise.resolve();
      expect(button.querySelector("path")?.getAttribute("data-shared-border-state")).toBe("hover");
    } finally {
      compositor.dispose(); Object.assign(globalThis, previous); dom.window.close();
    }
  });
  test("observed mutations schedule a repaint unless they touch nothing but the compositor's own overlays, and transitions of other properties are ignored", async () => {
    const dom = new JSDOM('<div class="semio-scope"><div data-shared-border-frame></div></div>', { pretendToBeVisual: true });
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!;
    const queue = new Map<number, FrameRequestCallback>();
    let next = 0;
    dom.window.requestAnimationFrame = callback => { queue.set(++next, callback); return next; };
    dom.window.cancelAnimationFrame = id => { queue.delete(id); };
    const flush = () => { const callbacks = [...queue.values()]; queue.clear(); callbacks.forEach(callback => callback(0)); };
    const pending = () => root.hasAttribute("data-shared-border-pending");
    const errors: unknown[] = [];
    dom.window.addEventListener("error", event => errors.push(event.error));
    const compositor = new BorderCompositor(root);
    try {
      flush();
      expect(pending()).toBe(false);
      const overlay = dom.window.document.createElement("span");
      overlay.setAttribute("data-shared-border-overlay", "");
      root.append(overlay);
      await Promise.resolve();
      expect(pending()).toBe(false);
      overlay.setAttribute("class", "painted");
      await Promise.resolve();
      expect(pending()).toBe(false);
      root.append(dom.window.document.createTextNode("text"));
      await Promise.resolve();
      expect(pending()).toBe(true);
      flush();
      expect(pending()).toBe(false);
      root.setAttribute("class", "changed");
      await Promise.resolve();
      expect(pending()).toBe(true);
      flush();
      root.dispatchEvent(new dom.window.Event("transitionrun"));
      expect(pending()).toBe(false);
      root.dispatchEvent(Object.assign(new dom.window.Event("transitionend", { bubbles: true }), { propertyName: "opacity" }));
      expect(pending()).toBe(false);
      root.dispatchEvent(Object.assign(new dom.window.Event("transitionrun", { bubbles: true }), { propertyName: "width" }));
      expect(pending()).toBe(true);
      expect(errors).toEqual([]);
    } finally {
      compositor.dispose(); Object.assign(globalThis, previous); dom.window.close();
    }
  });
  test("frame hooks preserve refs and either window/frame teardown order under StrictMode", async () => {
    const queue = new Map<number, FrameRequestCallback>();
    let next = 0;
    const request = vi.spyOn(window, "requestAnimationFrame").mockImplementation(callback => { queue.set(++next, callback); return next; });
    const cancel = vi.spyOn(window, "cancelAnimationFrame").mockImplementation(id => { queue.delete(id); });
    const scope = document.createElement("div");
    scope.className = "semio-scope"; document.body.append(scope);
    const forwarded = React.createRef<HTMLDivElement>();
    const cleanup = vi.fn();
    const callback = vi.fn((element: HTMLDivElement | null) => element ? cleanup : undefined);
    function Frame({ callbackRef = false }: { callbackRef?: boolean }) {
      const ref = useSharedBorderFrameRef(callbackRef ? callback : forwarded);
      return React.createElement("div", { ref, "data-shared-border-frame": "" });
    }
    function Window() {
      const [element, setElement] = React.useState<HTMLDivElement | null>(null);
      useSharedWindowBorder(element, geometry, "normal");
      return React.createElement("div", { ref: setElement });
    }
    const rect = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(() => ({ left: 0, top: 0, width: 100, height: 100, right: 100, bottom: 100 }) as DOMRect);
    const flush = async () => {
      for (let count = 0; queue.size && count < 10; count++) await React.act(async () => {
        const callbacks = [...queue.values()]; queue.clear(); callbacks.forEach(callback => callback(0));
      });
      expect(queue.size).toBe(0);
    };
    try {
      for (const order of cases.markerOnly.releaseOrders) {
        const root = createRoot(scope);
        const render = async (frame: boolean, window: boolean) => {
          await React.act(async () => root.render(React.createElement(React.StrictMode, null,
            frame ? React.createElement(Frame, { key: "frame" }) : null,
            window ? React.createElement(Window, { key: "window" }) : null)));
          await flush();
        };
        await render(true, true);
        expect(forwarded.current?.hasAttribute("data-shared-border-attached")).toBe(true);
        const remaining = order[0];
        await render(remaining !== "frame", remaining !== "window");
        if (remaining === "window") expect(forwarded.current?.querySelector("[data-shared-border-overlay]")).not.toBeNull();
        const lease = retainSharedBorderRoot(scope);
        expect(lease.compositor.windows.size).toBe(remaining === "frame" ? 1 : 0);
        lease.release();
        await render(false, false);
        expect(forwarded.current).toBeNull();
        expect(scope.querySelector("[data-shared-border-overlay]")).toBeNull();
        await React.act(async () => {
          const probe = document.createElement("div"); scope.append(probe); probe.remove();
          scope.dispatchEvent(new Event("pointerover", { bubbles: true }));
          window.dispatchEvent(new Event("resize"));
        });
        expect(queue.size).toBe(0);
        await React.act(async () => root.unmount());
      }
      const root = createRoot(scope);
      await React.act(async () => root.render(React.createElement(Frame, { callbackRef: true })));
      expect(callback).toHaveBeenCalledWith(expect.any(HTMLElement));
      await React.act(async () => root.unmount());
      expect(cleanup).toHaveBeenCalledTimes(1);
      expect(queue.size).toBe(0);
      expect(cancel).toHaveBeenCalled();
    } finally {
      scope.remove(); request.mockRestore(); cancel.mockRestore(); rect.mockRestore();
    }
  });
  test("clipped frames cannot steal visible seams; protected overlays retain effect precedence", () => {
    const dom = new JSDOM('<div class="semio-scope"><div id="visible" data-shared-border-frame data-shared-border-interactive></div><div id="clip" style="overflow:hidden"><div id="hidden" data-shared-border-frame data-active="true"></div></div></div>');
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!;
    const visible = root.querySelector<HTMLElement>("#visible")!, hidden = root.querySelector<HTMLElement>("#hidden")!, clip = root.querySelector<HTMLElement>("#clip")!;
    const rect = (left: number, top: number, width: number, height: number) => ({ left, top, width, height, right: left + width, bottom: top + height }) as DOMRect;
    visible.getBoundingClientRect = () => rect(0, 0, 100, 100);
    const fixture = cases.clippedOwner;
    clip.getBoundingClientRect = () => rect(0, 0, fixture.viewportRight, 100);
    hidden.getBoundingClientRect = () => rect(fixture.left, fixture.top, fixture.width, fixture.height);
    const compositor = new BorderCompositor(root);
    try {
      compositor.schedule();
      const overlay = visible.querySelector<SVGSVGElement>("[data-shared-border-overlay]")!;
      for (const [property, expected] of Object.entries(cases.overlay)) {
        expect(overlay.style.getPropertyValue(property)).toBe(expected);
        expect(overlay.style.getPropertyPriority(property)).toBe("important");
      }
      expect(hidden.querySelectorAll("[data-shared-border-segment]").length).toBe(fixture.expectedSegments);
      expect([...overlay.querySelectorAll("path")].some(path => path.getAttribute("d") === "M 99 1 L 99 99")).toBe(true);
      for (const effect of cases.effects) {
        visible.setAttribute(effect.attribute, effect.value);
        compositor.schedule();
        expect(overlay.querySelector("path")!.getAttribute("data-shared-border-state")).toBe(effect.state);
        if (effect.state === "celebrated") {
          expect(overlay.querySelector("mask .window-frame-border-celebrated-mask")).not.toBeNull();
          expect(overlay.querySelector("foreignObject .window-frame-border-celebrated-fill")).not.toBeNull();
        }
        visible.dispatchEvent(new dom.window.MouseEvent("pointerover", { bubbles: true }));
        expect(overlay.querySelector("path")!.getAttribute("data-shared-border-state")).toBe("hover");
        visible.dispatchEvent(new dom.window.MouseEvent("pointerout", { bubbles: true }));
        visible.removeAttribute(effect.attribute);
      }
    } finally {
      compositor.dispose(); Object.assign(globalThis, previous); dom.window.close();
    }
  });
  test("measures each clipping ancestor once per commit however many frames lie beneath it, and reads each frame's rect once to size its overlay", () => {
    const dom = new JSDOM('<div class="semio-scope"><div id="clip" style="overflow:hidden"><div class="a" data-shared-border-frame></div><div class="b" data-shared-border-frame></div><div class="c" data-shared-border-frame></div></div></div>');
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!;
    const clip = root.querySelector<HTMLElement>("#clip")!;
    const rect = (left: number, top: number, width: number, height: number) => ({ left, top, width, height, right: left + width, bottom: top + height }) as DOMRect;
    const clipRect = vi.fn(() => rect(0, 0, 400, 100));
    clip.getBoundingClientRect = clipRect;
    root.querySelectorAll<HTMLElement>("[data-shared-border-frame]").forEach((frame, index) => { frame.getBoundingClientRect = () => rect(index * 100, 0, 90, 90); });
    const compositor = new BorderCompositor(root);
    try {
      compositor.schedule();
      expect(clipRect).toHaveBeenCalledTimes(1);
      clipRect.mockClear();
      compositor.schedule();
      expect(clipRect).toHaveBeenCalledTimes(1);
    } finally {
      compositor.dispose(); Object.assign(globalThis, previous); dom.window.close();
    }
  });
  test("a hover, focus or state change measures again before painting, so a stated frame is never drawn from stale geometry", async () => {
    const dom = new JSDOM('<div class="semio-scope"><div id="clip" style="overflow:hidden"><div class="a" data-shared-border-frame data-shared-border-interactive></div><div class="b" data-shared-border-frame></div></div></div>');
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!;
    const rect = (left: number, top: number, width: number, height: number) => ({ left, top, width, height, right: left + width, bottom: top + height }) as DOMRect;
    const reads = vi.fn();
    const a = root.querySelector<HTMLElement>(".a")!, b = root.querySelector<HTMLElement>(".b")!, clip = root.querySelector<HTMLElement>("#clip")!;
    for (const [element, box] of [[a, rect(0, 0, 90, 90)], [b, rect(100, 0, 90, 90)], [clip, rect(0, 0, 400, 100)]] as const) element.getBoundingClientRect = () => (reads(), box);
    const compositor = new BorderCompositor(root);
    const settle = () => new Promise(resolve => dom.window.setTimeout(resolve, 0));
    const state = () => a.querySelector("[data-shared-border-overlay] path")?.getAttribute("data-shared-border-state");
    try {
      compositor.schedule();
      expect(reads).toHaveBeenCalled();
      await settle();
      reads.mockClear();
      a.dispatchEvent(new dom.window.MouseEvent("pointerover", { bubbles: true }));
      expect(state()).toBe("hover");
      a.dispatchEvent(new dom.window.MouseEvent("pointerout", { bubbles: true, relatedTarget: root }));
      a.dispatchEvent(new dom.window.FocusEvent("focusin", { bubbles: true }));
      b.setAttribute("data-active", "true");
      await settle();
      expect(state()).toBeDefined();
      expect(reads).toHaveBeenCalled();
    } finally {
      compositor.dispose(); Object.assign(globalThis, previous); dom.window.close();
    }
  });
  test("hands a window its segments only when they changed, so an unchanged repaint renders no window", () => {
    const dom = new JSDOM('<div class="semio-scope"><div id="window"></div></div>');
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!;
    const element = root.querySelector<HTMLElement>("#window")!;
    element.getBoundingClientRect = () => ({ left: 0, top: 0, width: 100, height: 100, right: 100, bottom: 100 }) as DOMRect;
    const update = vi.fn();
    const compositor = new BorderCompositor(root);
    try {
      compositor.windows.set("w", { element, geometry, state: "normal", update });
      compositor.schedule();
      expect(update).toHaveBeenCalledTimes(1);
      compositor.scheduleState();
      compositor.schedule();
      expect(update).toHaveBeenCalledTimes(1);
      compositor.windows.get("w")!.state = "active";
      compositor.scheduleState();
      expect(update).toHaveBeenCalledTimes(2);
    } finally {
      compositor.dispose(); Object.assign(globalThis, previous); dom.window.close();
    }
  });
  test("unequally scaled partial overlaps connect orthogonally without moving on hover", () => {
    const dom = new JSDOM('<div class="semio-scope"><div id="parent" data-shared-border-interactive><div id="child"></div></div></div>');
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!, parent = root.querySelector<HTMLElement>("#parent")!, child = root.querySelector<HTMLElement>("#child")!;
    parent.getBoundingClientRect = () => ({ left: 0, top: 0, width: 100, height: 100 }) as DOMRect;
    child.getBoundingClientRect = () => ({ left: 0, top: 30, width: 100, height: 40 }) as DOMRect;
    Object.defineProperties(child, { offsetWidth: { value: 50 }, offsetHeight: { value: 20 } });
    const childGeometry = createWindowFrameGeometry({ width: 50, height: 20, top: { depth: 0, chips: [] }, bottom: { depth: 0, chips: [] } });
    const updates = new Map<string, readonly { d: string; state: string }[]>();
    const compositor = new BorderCompositor(root);
    try {
      compositor.windows.set("parent", { element: parent, geometry, state: "normal", update: value => updates.set("parent", value) });
      compositor.windows.set("child", { element: child, geometry: childGeometry, state: "active", update: value => updates.set("child", value) });
      compositor.schedule();
      expect(updates.get("child")!.some(segment => segment.d === "M 0.75 1 L 0.75 19")).toBe(true);
      expect(updates.get("parent")!.some(segment => segment.d === "M 1 1 L 1 32 L 1.5 32")).toBe(true);
      parent.dispatchEvent(new dom.window.MouseEvent("pointerover", { bubbles: true }));
      expect(updates.get("parent")!.some(segment => segment.d === "M 1.5 32 L 1.5 68")).toBe(true);
      for (const segments of updates.values()) for (const segment of segments) {
        const values = segment.d.match(/-?\d+(?:\.\d+)?/g)!.map(Number);
        for (let index = 2; index < values.length; index += 2) expect(values[index] === values[index - 2] || values[index + 1] === values[index - 1]).toBe(true);
      }
    } finally {
      compositor.dispose(); Object.assign(globalThis, previous); dom.window.close();
    }
  });
  test("fractional unscaled pane bounds deduplicate with their containing window", () => {
    const dom = new JSDOM('<div class="semio-scope"><div id="parent" data-shared-border-interactive><div id="child"></div></div></div>');
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!, parent = root.querySelector<HTMLElement>("#parent")!, child = root.querySelector<HTMLElement>("#child")!;
    const f = cases.fractional;
    const rect = (top: number, width: number, height: number) => ({ left: 0, top, width, height, right: width, bottom: top + height }) as DOMRect;
    parent.getBoundingClientRect = () => rect(0, f.parentWidth, f.parentHeight);
    child.getBoundingClientRect = () => rect(f.parentHeight - f.childHeight, f.childWidth, f.childHeight);
    Object.defineProperties(parent, { offsetWidth: { value: Math.round(f.parentWidth) }, offsetHeight: { value: Math.round(f.parentHeight) } });
    Object.defineProperties(child, { offsetWidth: { value: Math.round(f.childWidth) }, offsetHeight: { value: Math.round(f.childHeight) } });
    const shape = (width: number, height: number) => createWindowFrameGeometry({ width, height, top: { depth: 0, chips: [] }, bottom: { depth: 0, chips: [] } });
    const updates = new Map<string, readonly { d: string; state: string }[]>();
    const compositor = new BorderCompositor(root);
    try {
      compositor.windows.set("parent", { element: parent, geometry: shape(f.parentWidth, f.parentHeight), state: "active", update: value => updates.set("parent", value) });
      compositor.windows.set("child", { element: child, geometry: shape(f.childWidth, f.childHeight), state: "normal", update: value => updates.set("child", value) });
      compositor.schedule();
      const duplicateBottom = updates.get("child")!.some(segment => {
        const points = segment.d.match(/-?\d+(?:\.\d+)?(?:e[+-]?\d+)?/g)!.map(Number);
        for (let i = 2; i < points.length; i += 2) if (Math.abs(points[i + 1] - points[i - 1]) < 1e-8 && Math.abs(points[i + 1] - (f.childHeight - 1)) < 1e-8 && Math.abs(points[i] - points[i - 2]) > 1) return true;
        return false;
      });
      expect(duplicateBottom).toBe(false);
      expect(updates.get("parent")!.every(segment => segment.state === "active")).toBe(true);
    } finally {
      compositor.dispose(); Object.assign(globalThis, previous); dom.window.close();
    }
  });
  test("paints the inner edges of every chip region in the normal state and no edge on the frame outline twice", () => {
    const dom = new JSDOM('<div class="semio-scope"><div id="stack" data-window-frame></div></div>');
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!;
    const stack = root.querySelector<HTMLElement>("#stack")!;
    stack.getBoundingClientRect = () => ({ left: 0, top: 0, right: 100, bottom: 100, width: 100, height: 100 }) as DOMRect;
    const compositor = new BorderCompositor(root);
    let painted: readonly { d: string; state: string }[] = [];
    try {
      const chipped = createWindowFrameGeometry({ width: 100, height: 100, top: { depth: 20, chips: [{ left: 0, right: 40 }, { left: 70, right: 100 }] }, bottom: { depth: 12, chips: [{ left: 30, right: 70 }] } });
      compositor.windows.set("w", { element: stack, geometry: chipped, state: "normal", update: value => { painted = value; } });
      compositor.schedule();
      const points = painted.flatMap(segment => [...segment.d.matchAll(/(-?[\d.]+) (-?[\d.]+)/g)].map(match => `${match[1]},${match[2]}`));
      for (const corner of ["1,1", "99,1", "99,99", "1,99", "39,19", "71,19", "71,1", "31,89", "69,89", "39,1", "31,99"]) expect(points, corner).toContain(corner);
      expect(painted.every(segment => segment.state === "normal")).toBe(true);
      const plain = createWindowFrameGeometry({ width: 100, height: 100, top: { depth: 0, chips: [] }, bottom: { depth: 0, chips: [] } });
      compositor.windows.set("w", { element: stack, geometry: plain, state: "normal", update: value => { painted = value; } });
      compositor.schedule();
      expect(painted).toEqual([{ d: plain.borderPath, state: "normal" }]);
    } finally {
      compositor.dispose(); Object.assign(globalThis, previous); dom.window.close();
    }
  });
  test("deduplicates reversed-order sibling seams and restores detached frames", () => {
    const dom = new JSDOM('<div class="semio-scope"><div id="left" data-shared-border-interactive></div><div id="right"></div><nav data-shared-border-frame data-shared-border-edges="bottom"></nav></div>');
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!;
    const rect = (left: number, top = 0, width = 100, height = 100) => ({ left, top, right: left + width, bottom: top + height, width, height, x: left, y: top, toJSON() {} }) as DOMRect;
    const left = root.querySelector<HTMLElement>("#left")!, right = root.querySelector<HTMLElement>("#right")!, nav = root.querySelector<HTMLElement>("nav")!;
    left.getBoundingClientRect = () => rect(0); right.getBoundingClientRect = () => rect(100);
    nav.getBoundingClientRect = () => rect(0, -30, 200, 20);
    const updates = new Map<string, readonly { d: string; state: string }[]>();
    const compositor = new BorderCompositor(root);
    try {
      compositor.windows.set("b", { element: right, geometry, state: "active", update: value => updates.set("b", value) });
      compositor.windows.set("a", { element: left, geometry, state: "normal", update: value => updates.set("a", value) });
      compositor.schedule();
      expect(updates.get("a")!.some(segment => segment.d === "M 99 1 L 99 99")).toBe(false);
      expect(updates.get("a")!.some(segment => segment.d === "M 1 1 L 100 1")).toBe(true);
      expect(updates.get("b")!.some(segment => segment.d === "M 0 1 L 99 1")).toBe(true);
      expect(updates.get("b")!.some(segment => segment.d === "M 0 1 L 0 99")).toBe(true);
      expect(nav.getAttribute("data-shared-border-attached")).toBe("bottom");
      const overlay = nav.querySelector<SVGSVGElement>("[data-shared-border-overlay]")!;
      expect(overlay.style.getPropertyValue("width")).toBe("200px");
      expect(overlay.style.getPropertyPriority("width")).toBe("important");
      expect(overlay.style.getPropertyValue("height")).toBe("20px");
      expect(overlay.style.getPropertyValue("max-width")).toBe("none");
      expect(nav.querySelectorAll("[data-shared-border-segment]").length).toBeLessThanOrEqual(2);
      left.dispatchEvent(new dom.window.MouseEvent("pointerover", { bubbles: true }));
      expect(updates.get("a")!.some(segment => segment.d === "M 100 1 L 100 99" && segment.state === "hover")).toBe(true);
      expect(updates.get("b")!.some(segment => segment.d === "M 0 1 L 0 99")).toBe(false);
      expect(updates.get("b")!.some(segment => segment.d === "M 1 1 L 1 99")).toBe(false);
      expect(root.querySelectorAll("[data-shared-border-overlay]").length).toBe(1);
      left.dispatchEvent(new dom.window.MouseEvent("pointerout", { bubbles: true }));
      const child = dom.window.document.createElement("div");
      left.append(child);
      child.getBoundingClientRect = () => rect(0, 30, 50, 40);
      const childGeometry = createWindowFrameGeometry({ width: 50, height: 40, top: { depth: 0, chips: [] }, bottom: { depth: 0, chips: [] } });
      compositor.windows.set("child", { element: child, geometry: childGeometry, state: "hover", update: value => updates.set("child", value) });
      compositor.schedule();
      const leftPieces = updates.get("a")!.map(segment => segment.d.match(/-?\d+(?:\.\d+)?/g)!.map(Number)).filter(values => values[0] === 1 && values[2] === 1);
      expect(leftPieces.length).toBe(2);
      expect(leftPieces[0]![1]).toBeCloseTo(1);
      expect(leftPieces[0]![3]).toBeCloseTo(31);
      expect(leftPieces[1]![1]).toBeCloseTo(69);
      expect(leftPieces[1]![3]).toBeCloseTo(99);
      expect(updates.get("child")).toEqual([{ d: childGeometry.borderPath, state: "hover" }]);
    } finally {
      compositor.dispose();
      expect(nav.hasAttribute("data-shared-border-attached")).toBe(false);
      expect(root.querySelectorAll("[data-shared-border-overlay]").length).toBe(0);
      Object.assign(globalThis, previous);
      dom.window.close();
    }
  });
  test("a seam between abutting windows that clip their overflow lies fully inside its owner, so no half of it is cut", () => {
    const dom = new JSDOM('<div class="semio-scope"><div id="left" style="overflow-x:hidden;overflow-y:hidden"></div><div id="right" style="overflow-x:hidden;overflow-y:hidden"></div></div>');
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!;
    const rect = (left: number) => ({ left, top: 0, right: left + 100, bottom: 100, width: 100, height: 100, x: left, y: 0, toJSON() {} }) as DOMRect;
    const left = root.querySelector<HTMLElement>("#left")!, right = root.querySelector<HTMLElement>("#right")!;
    left.getBoundingClientRect = () => rect(0); right.getBoundingClientRect = () => rect(100);
    const updates = new Map<string, readonly { d: string; state: string }[]>();
    const compositor = new BorderCompositor(root);
    try {
      compositor.windows.set("b", { element: right, geometry, state: "active", update: value => updates.set("b", value) });
      compositor.windows.set("a", { element: left, geometry, state: "normal", update: value => updates.set("a", value) });
      compositor.schedule();
      const lines = (id: string) => (updates.get(id) ?? []).flatMap(segment => [...segment.d.matchAll(/M\s*([\d.-]+)\s+([\d.-]+)\s+L\s*([\d.-]+)\s+([\d.-]+)/g)].map(match => ({ x1: Number(match[1]), x2: Number(match[3]) })));
      expect(lines("b").some(line => line.x1 === 1 && line.x2 === 1), "the active seam is held one inset inside the right window").toBe(true);
      expect(lines("b").some(line => line.x1 <= 0 && line.x2 <= 0), "nothing of it lies on or beyond the clip edge").toBe(false);
      expect(lines("a").some(line => line.x1 >= 100 && line.x2 >= 100), "the losing window paints no seam of its own").toBe(false);
    } finally {
      compositor.dispose(); Object.assign(globalThis, previous); dom.window.close();
    }
  });
  test("framed controls that are direct children of a cells owner are cells: the owner paints one frame, they register none", () => {
    const { row: size, cells, expectedOverlays, rowPaths } = cases.cellFrames;
    const dom = new JSDOM('<div class="semio-scope"><div id="row" data-shared-border-frame data-shared-border-cells>' + '<div class="cell" data-shared-border-frame data-shared-border-interactive></div>'.repeat(cells) + '<div id="wrapper"><div id="deep" data-shared-border-frame></div></div></div></div>');
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!;
    const row = root.querySelector<HTMLElement>("#row")!, deep = root.querySelector<HTMLElement>("#deep")!;
    const rect = (left: number, top: number, width: number, height: number) => ({ left, top, right: left + width, bottom: top + height, width, height }) as DOMRect;
    row.getBoundingClientRect = () => rect(0, 0, size.width, size.height);
    const items = [...root.querySelectorAll<HTMLElement>(".cell")];
    items.forEach((item, index) => { item.getBoundingClientRect = () => rect(index * (size.width / cells), 0, size.width / cells, size.height); });
    deep.getBoundingClientRect = () => rect(0, size.height + 50, 40, size.height);
    expect([...items, row, deep].map(isSharedBorderCell)).toEqual([...items.map(() => true), false, false]);
    const compositor = new BorderCompositor(root);
    try {
      compositor.schedule();
      const owned = [...row.querySelectorAll(":scope > svg[data-shared-border-overlay] path")].map(path => path.getAttribute("d"));
      expect(owned.sort()).toEqual([...rowPaths].sort());
      expect(items.every(item => !item.hasAttribute("data-shared-border-attached") && !item.querySelector("svg"))).toBe(true);
      expect(root.querySelectorAll("[data-shared-border-overlay]")).toHaveLength(expectedOverlays + 1);
      expect(deep.querySelector("svg[data-shared-border-overlay]")).not.toBeNull();
      row.dispatchEvent(new dom.window.MouseEvent("pointerover", { bubbles: true }));
      expect(row.querySelector(':scope > svg [data-shared-border-state="hover"]')).toBeNull();
      items[1]!.dispatchEvent(new dom.window.MouseEvent("pointerover", { bubbles: true }));
      const hovered = [...row.querySelectorAll(':scope > svg [data-shared-border-state="hover"]')].map(path => path.getAttribute("d"));
      expect(hovered.sort()).toEqual([...cases.cellFrames.hoveredCellPaths].sort());
      expect(items.every(item => !item.querySelector("svg"))).toBe(true);
      items[0]!.removeAttribute("data-shared-border-frame");
      deep.parentElement!.setAttribute("data-shared-border-cells", "");
      compositor.schedule();
      expect(deep.querySelector("svg[data-shared-border-overlay]")).toBeNull();
      expect(deep.hasAttribute("data-shared-border-attached")).toBe(false);
    } finally {
      compositor.dispose();
      Object.assign(globalThis, previous);
      dom.window.close();
    }
  });
  test("a group's cells state only themselves: hover, keyboard focus and selection close the cell's outline and never recolour the group", () => {
    const dom = new JSDOM('<div class="semio-scope"><div id="group" data-shared-border-frame><button id="a" data-shared-border-cell></button><button id="b" data-shared-border-cell></button></div></div>');
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!;
    const rect = (left: number, width: number) => ({ left, top: 0, right: left + width, bottom: 30, width, height: 30 }) as DOMRect;
    const group = root.querySelector<HTMLElement>("#group")!, a = root.querySelector<HTMLElement>("#a")!, b = root.querySelector<HTMLElement>("#b")!;
    group.getBoundingClientRect = () => rect(0, 200); a.getBoundingClientRect = () => rect(0, 100); b.getBoundingClientRect = () => rect(100, 100);
    const compositor = new BorderCompositor(root);
    const painted = (state: string) => [...group.querySelectorAll(`:scope > svg [data-shared-border-state="${state}"]`)].map(path => path.getAttribute("d")).sort();
    const closed = (left: number, right: number) => [`M ${left} 1 L ${left} 29`, `M ${right} 1 L ${right} 29`, `M ${left} 1 L ${right} 1`, `M ${left} 29 L ${right} 29`].sort();
    try {
      compositor.schedule();
      expect(painted("normal")).toHaveLength(7);
      a.dispatchEvent(new dom.window.MouseEvent("pointerover", { bubbles: true }));
      expect(painted("hover")).toEqual(closed(1, 100));
      expect(painted("normal")).toEqual(["M 100 1 L 199 1", "M 100 29 L 199 29", "M 199 1 L 199 29"]);
      a.dispatchEvent(new dom.window.MouseEvent("pointerout", { bubbles: true, relatedTarget: root }));
      expect(painted("hover")).toEqual([]);
      b.matches = (selector: string) => selector === ":focus-visible" ? false : Element.prototype.matches.call(b, selector);
      b.focus(); compositor.scheduleState();
      expect(painted("focus")).toEqual([]);
      b.matches = (selector: string) => selector === ":focus-visible" || Element.prototype.matches.call(b, selector);
      compositor.scheduleState();
      expect(painted("focus")).toEqual(closed(100, 199));
      b.setAttribute("aria-pressed", "true");
      compositor.scheduleState();
      expect(painted("focus")).toEqual(closed(100, 199));
      expect(painted("active")).toEqual([]);
      expect(group.querySelector('svg [data-shared-border-state="hover"]')).toBeNull();
    } finally {
      compositor.dispose(); Object.assign(globalThis, previous); dom.window.close();
    }
  });
  test("a borderless box draws nothing at rest and adds no line on hover; keyboard focus closes its full outline and the overlay is released again", () => {
    const dom = new JSDOM('<div class="semio-scope"><button id="tab" data-shared-border-frame data-shared-border-edges=""></button></div>');
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!, tab = root.querySelector<HTMLElement>("#tab")!;
    tab.getBoundingClientRect = () => ({ left: 0, top: 0, right: 40, bottom: 20, width: 40, height: 20 }) as DOMRect;
    const compositor = new BorderCompositor(root);
    try {
      compositor.schedule();
      expect(tab.querySelector("svg")).toBeNull();
      expect(tab.style.position).toBe("");
      tab.dispatchEvent(new dom.window.MouseEvent("pointerover", { bubbles: true }));
      expect(tab.querySelectorAll("svg [data-shared-border-state]")).toHaveLength(0);
      tab.dispatchEvent(new dom.window.MouseEvent("pointerout", { bubbles: true, relatedTarget: root }));
      expect(tab.querySelector("svg")).toBeNull();
      tab.matches = (selector: string) => selector === ":focus-visible" || Element.prototype.matches.call(tab, selector);
      tab.focus(); compositor.scheduleState();
      expect([...tab.querySelectorAll('svg [data-shared-border-state="focus"]')].map(path => path.getAttribute("d")).sort()).toEqual(["M 1 1 L 1 19", "M 1 1 L 39 1", "M 1 19 L 39 19", "M 39 1 L 39 19"]);
      expect(tab.querySelector<SVGSVGElement>("svg")!.style.zIndex).toBe("41");
      tab.blur(); compositor.scheduleState();
      expect(tab.querySelector("svg")).toBeNull();
      expect(tab.style.position).toBe("");
    } finally {
      compositor.dispose(); Object.assign(globalThis, previous); dom.window.close();
    }
  });
  describe("nested and wrapped cells, tabs and chips", () => {
    const rect = (left: number, top: number, width: number, height: number) => ({ left, top, right: left + width, bottom: top + height, width, height }) as DOMRect;
    const harness = (html: string, rects: Record<string, DOMRect>) => {
      const dom = new JSDOM(html);
      const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
      Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
      const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!;
      for (const [id, box] of Object.entries(rects)) root.querySelector<HTMLElement>(`#${id}`)!.getBoundingClientRect = () => box;
      const compositor = new BorderCompositor(root);
      const hover = (id: string) => root.querySelector(`#${id}`)!.dispatchEvent(new dom.window.MouseEvent("pointerover", { bubbles: true }));
      const painted = (state: string) => [...root.querySelectorAll(`svg [data-shared-border-state="${state}"]`)].map(path => path.getAttribute("d")).sort();
      return { dom, root, compositor, hover, painted, done: () => { compositor.dispose(); Object.assign(globalThis, previous); dom.window.close(); } };
    };
    const closed = (left: number, right: number, top = 1, bottom = 19) => [`M ${left} ${top} L ${left} ${bottom}`, `M ${right} ${top} L ${right} ${bottom}`, `M ${left} ${top} L ${right} ${top}`, `M ${left} ${bottom} L ${right} ${bottom}`].sort();

    test("a passive table band keeps its own seam when a standalone control is hovered", () => {
      const h = harness('<div class="semio-scope"><table><thead id="header" data-shared-border-frame data-shared-border-edges="bottom" data-shared-border-delegate="self"><tr><th><button id="sort">Sort</button></th></tr></thead></table></div>', { header: rect(0, 0, 100, 20), sort: rect(10, 2, 30, 16) });
      try {
        h.compositor.schedule();
        h.hover("sort");
        expect(h.painted("normal")).toEqual(["M 1 19 L 99 19"]);
        expect(h.painted("hover")).toEqual([]);
      } finally { h.done(); }
    });

    test("a borderless neighbour recolours an existing seam without moving its centerline", () => {
      const h = harness('<div class="semio-scope"><div id="frame" data-shared-border-frame></div><button id="button" data-shared-border-frame data-shared-border-edges=""></button></div>', { frame: rect(0, 0, 50, 20), button: rect(50, 0, 50, 20) });
      try {
        h.compositor.schedule();
        expect(h.painted("normal")).toContain("M 49 1 L 49 19");
        h.hover("button");
        expect(h.painted("hover")).toEqual(["M -1 1 L -1 19"]);
        expect(h.painted("normal")).not.toContain("M 49 1 L 49 19");
      } finally { h.done(); }
    });

    test("a cell inside a frame that is itself a cell is painted by the nearest owner with an overlay, and its hover emphasises its drawn divider alone", () => {
      const h = harness('<div class="semio-scope"><div id="controls" data-shared-border-group data-shared-border-cells><div id="group" data-shared-border-frame><button id="a" data-shared-border-cell></button><button id="b" data-shared-border-cell></button></div></div></div>', { controls: rect(0, 0, 100, 20), group: rect(0, 0, 100, 20), a: rect(0, 0, 50, 20), b: rect(50, 0, 50, 20) });
      try {
        h.compositor.schedule();
        expect(h.root.querySelector("#group > svg")).toBeNull();
        h.hover("a");
        expect(h.painted("hover")).toEqual(["M 50 1 L 50 19"]);
        expect(h.root.querySelector("#controls > svg")).not.toBeNull();
      } finally { h.done(); }
    });

    test("a cell below a plain wrapper is owned by the nearest ancestor owner, not by its parent element, and a hovered tab emphasises its divider and the frame line beneath it", () => {
      const h = harness('<div class="semio-scope"><div id="frame" data-shared-border-frame data-shared-border-edges="bottom"><div id="tabs" data-shared-border-group><span><button id="a" data-shared-border-cell></button></span><span><button id="b" data-shared-border-cell></button></span></div></div></div>', { frame: rect(0, 0, 100, 20), tabs: rect(0, 0, 100, 20), a: rect(0, 0, 50, 20), b: rect(50, 0, 50, 20) });
      try {
        h.compositor.schedule();
        h.hover("b");
        expect(h.painted("hover")).toEqual(["M 50 1 L 50 19", "M 50 19 L 99 19"]);
        expect(h.painted("normal").every(d => !d?.startsWith("M 50 1 L 50"))).toBe(true);
      } finally { h.done(); }
    });

    test("a stepper cell flush with the bottom edge of its row closes its own bottom over its full width", () => {
      const h = harness('<div class="semio-scope"><div id="row" data-shared-border-frame data-shared-border-edges="bottom"><div><div id="stepper" data-shared-border-frame><button id="minus" data-shared-border-cell></button><button id="plus" data-shared-border-cell></button></div></div></div></div>', { row: rect(0, 0, 200, 20), stepper: rect(100, 0, 100, 20), minus: rect(100, 0, 50, 20), plus: rect(150, 0, 50, 20) });
      try {
        h.compositor.schedule();
        h.hover("plus");
        expect(h.painted("hover")).toEqual(["M 50 1 L 50 19", "M 50 19 L 99 19"]);
      } finally { h.done(); }
    });

    test("mode dock tabs are borderless boxes: a hovered tab emphasises the frame lines it lies on and adds none", () => {
      const h = harness('<div class="semio-scope"><div id="stack"><div id="chip" data-window-frame-chip><div role="tablist"><div id="tab" role="tab" data-slot="mode-dock-tab" tabindex="0"></div><div id="other" role="tab" data-slot="mode-dock-tab" tabindex="0"></div></div></div></div></div>', { stack: rect(0, 0, 100, 40), chip: rect(0, 0, 60, 20), tab: rect(0, 0, 30, 20), other: rect(30, 0, 30, 20) });
      try {
        h.compositor.windows.set("w", { element: h.root.querySelector<HTMLElement>("#stack")!, geometry, state: "normal", update: () => {} });
        h.compositor.schedule();
        expect(h.root.querySelector("#tab > svg")).toBeNull();
        h.hover("tab");
        expect(h.painted("hover")).toEqual(["M 1 1 L 1 19", "M 1 1 L 29 1"]);
      } finally { h.done(); }
    });

    test("a chip hover emphasises its own edges and the full enclosing card frame", () => {
      const h = harness('<div class="semio-scope"><section id="card" data-overview-card><div id="stack" data-shared-border-hover><div id="chip" data-window-frame-chip><button id="control"></button></div></div></section></div>', { card: rect(0, 0, 100, 100), stack: rect(0, 0, 100, 100), chip: rect(0, 0, 40, 20), control: rect(0, 0, 40, 20) });
      const updates = new Map<string, readonly { d: string; state: string }[]>();
      try {
        h.compositor.windows.set("w", { element: h.root.querySelector<HTMLElement>("#stack")!, geometry, state: "normal", update: value => updates.set("w", value) });
        h.compositor.schedule();
        const idle = updates.get("w")!.map(segment => segment.d);
        h.hover("stack");
        expect(updates.get("w")).toEqual(idle.map(d => ({ d, state: "hover" })));
        h.hover("control");
        const hovered = updates.get("w")!.filter(segment => segment.state === "hover");
        const numbers = hovered.flatMap(segment => segment.d.match(/-?\d+(?:\.\d+)?/g)!.map(Number));
        expect(Math.max(...numbers)).toBe(99);
        expect(updates.get("w")!.every(segment => segment.state === "hover")).toBe(true);
      } finally { h.done(); }
    });

    test("a stated cell stays closed and fully visible under a clipping row whose edge coincides with, or cuts, its bottom", () => {
      for (const [clipHeight, bottom] of [[20, 19], [19.6, 19], [12, 11]] as const) {
        const h = harness(`<div class="semio-scope"><div id="clip" style="overflow:hidden"><div id="group" data-shared-border-frame><button id="a" data-shared-border-cell></button><button id="b" data-shared-border-cell></button></div></div></div>`, { clip: rect(0, 0, 100, clipHeight), group: rect(0, 0, 100, 20), a: rect(0, 0, 50, 20), b: rect(50, 0, 50, 20) });
        try {
          h.compositor.schedule();
          h.hover("b");
          expect(h.painted("hover"), `clip height ${clipHeight}`).toEqual(closed(50, 99, 1, bottom));
        } finally { h.done(); }
      }
    });

    test("a utility-bar chip flush with a clipping ancestor still paints its stated outline", () => {
      const h = harness('<div class="semio-scope"><div id="clip" style="overflow:hidden"><div id="stack" data-window-frame><div id="chip" data-window-frame-chip><button id="unfold"></button></div></div></div></div>', { clip: rect(0, 0, 40, 20), stack: rect(0, 0, 40, 20), chip: rect(0, 0, 40, 20), unfold: rect(0, 0, 40, 20) });
      const updates = new Map<string, readonly { d: string; state: string }[]>();
      try {
        h.compositor.windows.set("w", { element: h.root.querySelector<HTMLElement>("#stack")!, geometry, state: "normal", update: value => updates.set("w", value) });
        h.compositor.schedule();
        h.hover("unfold");
        const hovered = updates.get("w")!.filter(segment => segment.state === "hover").map(segment => segment.d);
        expect(hovered.length).toBeGreaterThanOrEqual(4);
      } finally { h.done(); }
    });

    test("a narrow window chip keeps its side beside a nearby nonincident window edge", () => {
      const h = harness('<div class="semio-scope"><div id="stack" data-window-frame><div id="chip" data-window-frame-chip><button id="unfold"></button></div></div></div>', { stack: rect(0, 0, 100, 100), chip: rect(10, 30, 30, 24), unfold: rect(10, 30, 30, 24) });
      const geometry = createWindowFrameGeometry({ width: 100, height: 100, top: { depth: 20, chips: [{ left: 12.5, right: 40 }] }, bottom: { depth: 0, chips: [] } });
      let segments: readonly { d: string; state: string }[] = [];
      try {
        h.compositor.windows.set("w", { element: h.root.querySelector<HTMLElement>("#stack")!, geometry, state: "normal", update: value => { segments = value; } });
        h.compositor.schedule();
        h.hover("unfold");
        expect(segments.filter(segment => segment.state === "hover").map(segment => segment.d).sort()).toEqual([
          "M 11 31 L 11 53", "M 11 31 L 39 31", "M 11 53 L 39 53", "M 39 31 L 39 53",
        ]);
      } finally { h.done(); }
    });

    test("a chip as large as its whole window paints a stated outline in the window's own segments", () => {
      const h = harness('<div class="semio-scope"><div id="stack" data-window-frame><div id="chip" data-window-frame-chip><button id="unfold"></button></div></div></div>', { stack: rect(0, 0, 100, 100), chip: rect(0, 0, 100, 100), unfold: rect(0, 0, 100, 100) });
      const updates = new Map<string, readonly { d: string; state: string }[]>();
      try {
        h.compositor.windows.set("w", { element: h.root.querySelector<HTMLElement>("#stack")!, geometry, state: "normal", update: value => updates.set("w", value) });
        h.compositor.schedule();
        h.hover("unfold");
        expect(updates.get("w")!.filter(segment => segment.state === "hover").length).toBeGreaterThanOrEqual(4);
      } finally { h.done(); }
    });

    test("a chip of a window without chip regions still paints its stated outline", () => {
      const h = harness('<div class="semio-scope"><div id="stack" data-window-frame><div id="chip" data-window-frame-chip><button id="unfold"></button></div></div></div>', { stack: rect(0, 0, 100, 100), chip: rect(0, 0, 40, 20), unfold: rect(0, 0, 40, 20) });
      const updates = new Map<string, readonly { d: string; state: string }[]>();
      try {
        h.compositor.windows.set("w", { element: h.root.querySelector<HTMLElement>("#stack")!, geometry, state: "normal", update: value => updates.set("w", value) });
        h.compositor.schedule();
        h.hover("unfold");
        expect(updates.get("w")!.filter(segment => segment.state === "hover").length).toBeGreaterThan(0);
        expect(updates.get("w")!.some(segment => segment.state === "normal")).toBe(true);
      } finally { h.done(); }
    });
  });
  test("table rows and zero-width separators are boxes: the overlay is appended to the positioned row, a hover adds no line to either, and a 0 px handle paints its centre line only while focused", () => {
    const dom = new JSDOM('<div class="semio-scope"><table><tbody><tr id="row" tabindex="0" data-shared-border-frame data-shared-border-edges=""><td>a</td></tr></tbody></table><div id="handle" role="separator" tabindex="0" data-shared-border-frame data-shared-border-edges=""></div></div>');
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!, row = root.querySelector<HTMLElement>("#row")!, handle = root.querySelector<HTMLElement>("#handle")!;
    row.getBoundingClientRect = () => ({ left: 0, top: 0, right: 100, bottom: 20, width: 100, height: 20 }) as DOMRect;
    handle.getBoundingClientRect = () => ({ left: 50, top: 30, right: 50, bottom: 60, width: 0, height: 30 }) as DOMRect;
    const compositor = new BorderCompositor(root);
    const states = (host: HTMLElement) => [...host.querySelectorAll("svg [data-shared-border-state]")].map(path => `${path.getAttribute("data-shared-border-state")} ${path.getAttribute("d")}`).sort();
    try {
      compositor.schedule();
      expect(row.querySelector("svg")).toBeNull();
      expect(handle.querySelector("svg")).toBeNull();
      const focus = (element: HTMLElement) => { element.matches = (selector: string) => selector === ":focus-visible" || Element.prototype.matches.call(element, selector); element.focus(); compositor.scheduleState(); };
      row.dispatchEvent(new dom.window.MouseEvent("pointerover", { bubbles: true }));
      expect(states(row)).toEqual([]);
      row.dispatchEvent(new dom.window.MouseEvent("pointerout", { bubbles: true, relatedTarget: root }));
      focus(row);
      expect(row.querySelector(":scope > svg")).not.toBeNull();
      expect(states(row)).toEqual(["focus M 1 1 L 1 19", "focus M 1 1 L 99 1", "focus M 1 19 L 99 19", "focus M 99 1 L 99 19"]);
      handle.dispatchEvent(new dom.window.MouseEvent("pointerover", { bubbles: true }));
      expect(states(handle)).toEqual([]);
      handle.dispatchEvent(new dom.window.MouseEvent("pointerout", { bubbles: true, relatedTarget: root }));
      focus(handle);
      expect(states(handle)).toEqual(["focus M 0 1 L 0 29"]);
      expect(handle.querySelector("svg")!.hasAttribute("viewBox")).toBe(false);
      handle.blur(); compositor.scheduleState();
      expect(handle.querySelector("svg")).toBeNull();
    } finally {
      compositor.dispose(); Object.assign(globalThis, previous); dom.window.close();
    }
  });
  test("invalid and selected markers: aria-invalid or data-invalid on the box or its single control states invalid, data-selected states active, and focus outranks invalid and active while it lasts", () => {
    const dom = new JSDOM('<div class="semio-scope"><div id="field" data-shared-border-frame><input id="input" aria-invalid="true" /></div><div id="item" role="option" data-selected="true" data-shared-border-frame></div><div id="flag" data-shared-border-frame data-invalid><input /></div></div>');
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!;
    for (const [index, id] of ["field", "item", "flag"].entries()) root.querySelector<HTMLElement>(`#${id}`)!.getBoundingClientRect = () => ({ left: 0, top: index * 30, right: 50, bottom: index * 30 + 20, width: 50, height: 20 }) as DOMRect;
    const compositor = new BorderCompositor(root);
    const state = (id: string) => root.querySelector(`#${id} > svg [data-shared-border-state]`)?.getAttribute("data-shared-border-state");
    try {
      compositor.schedule();
      expect([state("field"), state("item"), state("flag")]).toEqual(["invalid", "active", "invalid"]);
      const input = root.querySelector<HTMLElement>("#input")!;
      input.matches = (selector: string) => selector === ":focus-visible" || Element.prototype.matches.call(input, selector);
      input.focus(); compositor.scheduleState();
      expect(state("field")).toBe("focus");
    } finally {
      compositor.dispose(); Object.assign(globalThis, previous); dom.window.close();
    }
  });
  test("a box that appears without any hook still gets a compositor, which is released when the box is gone", async () => {
    const scope = document.createElement("div");
    scope.className = "semio-scope";
    const button = document.createElement("button");
    button.setAttribute("data-shared-border-frame", ""); button.setAttribute("data-shared-border-edges", "top right bottom left");
    button.getBoundingClientRect = () => ({ left: 0, top: 0, right: 40, bottom: 20, width: 40, height: 20 }) as DOMRect;
    scope.append(button);
    document.body.append(scope);
    try {
      await new Promise(resolve => setTimeout(resolve, 0));
      expect(scope.hasAttribute("data-shared-border-root")).toBe(true);
      button.dispatchEvent(new MouseEvent("pointerover", { bubbles: true }));
      await new Promise(resolve => setTimeout(resolve, 50));
      expect(button.querySelector('svg [data-shared-border-state="hover"]')).not.toBeNull();
      button.remove();
      await new Promise(resolve => setTimeout(resolve, 0));
      expect(scope.hasAttribute("data-shared-border-root")).toBe(false);
    } finally { scope.remove(); }
  });
  test("a void or replaced element that is a box is painted by the nearest ancestor that can hold the overlay", () => {
    const dom = new JSDOM('<div class="semio-scope"><div id="wrap"><input id="field" data-shared-border-frame data-shared-border-edges="" /><img id="picture" data-shared-border-frame data-shared-border-edges="" data-shared-border-interactive /></div></div>');
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!, wrap = root.querySelector<HTMLElement>("#wrap")!, field = root.querySelector<HTMLElement>("#field")!, picture = root.querySelector<HTMLElement>("#picture")!;
    wrap.getBoundingClientRect = () => ({ left: 10, top: 10, right: 110, bottom: 30, width: 100, height: 20 }) as DOMRect;
    field.getBoundingClientRect = () => ({ left: 10, top: 10, right: 60, bottom: 30, width: 50, height: 20 }) as DOMRect;
    picture.getBoundingClientRect = () => ({ left: 60, top: 10, right: 110, bottom: 30, width: 50, height: 20 }) as DOMRect;
    const compositor = new BorderCompositor(root);
    const painted = () => [...wrap.querySelectorAll(':scope > svg [data-shared-border-state="active"]')].map(path => path.getAttribute("d")).sort();
    try {
      compositor.schedule();
      expect(wrap.querySelector("svg")).toBeNull();
      expect(field.querySelector("svg")).toBeNull();
      field.dispatchEvent(new dom.window.MouseEvent("pointerover", { bubbles: true }));
      expect(wrap.querySelectorAll(":scope > svg [data-shared-border-state]")).toHaveLength(0);
      field.dispatchEvent(new dom.window.MouseEvent("pointerout", { bubbles: true, relatedTarget: root }));
      field.setAttribute("data-shared-border-state", "active"); compositor.scheduleState();
      expect(painted()).toEqual(["M 1 1 L 1 19", "M 1 1 L 49 1", "M 1 19 L 49 19", "M 49 1 L 49 19"]);
      field.removeAttribute("data-shared-border-state");
      picture.setAttribute("data-shared-border-state", "active"); compositor.scheduleState();
      expect(painted()).toEqual(["M 51 1 L 51 19", "M 51 1 L 99 1", "M 51 19 L 99 19", "M 99 1 L 99 19"]);
      expect(picture.querySelector("svg")).toBeNull();
    } finally {
      compositor.dispose(); Object.assign(globalThis, previous); dom.window.close();
    }
  });
  test("a disabled box that draws normal edges marks them for the forced-colours disabled stroke, and the box API reaches products through the chrome entry", () => {
    const dom = new JSDOM('<div class="semio-scope"><button id="off" disabled data-shared-border-frame></button><button id="on" data-shared-border-frame></button></div>');
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!;
    root.querySelector<HTMLElement>("#off")!.getBoundingClientRect = () => ({ left: 0, top: 0, right: 40, bottom: 20, width: 40, height: 20 }) as DOMRect;
    root.querySelector<HTMLElement>("#on")!.getBoundingClientRect = () => ({ left: 100, top: 0, right: 140, bottom: 20, width: 40, height: 20 }) as DOMRect;
    const compositor = new BorderCompositor(root);
    try {
      compositor.schedule();
      expect(root.querySelectorAll("#off svg [data-shared-border-disabled]").length).toBeGreaterThan(0);
      expect(root.querySelectorAll("#on svg [data-shared-border-disabled]").length).toBe(0);
      const chrome = readFileSync(resolve(import.meta.dirname, "../../🎯️targets/⚛️react/🪟️chrome/🟦️.ts"), "utf8");
      expect(chrome).toContain("useSharedBorderBox");
      expect(chrome).toContain("sharedBorderlessBoxProps");
    } finally {
      compositor.dispose(); Object.assign(globalThis, previous); dom.window.close();
    }
  });
  test("interlocked cards: every coincident segment of a stack or grid is painted by exactly one owner", () => {
    for (const fixture of cases.interlock) {
      const dom = new JSDOM('<div class="semio-scope">' + fixture.windows.map(window => `<div id="${window.id}"></div>`).join("") + "</div>");
      const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
      Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
      const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!;
      const compositor = new BorderCompositor(root);
      const painted = new Map<string, { left: number; top: number; segments: readonly { d: string }[] }>();
      for (const window of fixture.windows) {
        const element = root.querySelector<HTMLElement>(`#${window.id}`)!;
        element.getBoundingClientRect = () => ({ left: window.left, top: window.top, right: window.left + window.width, bottom: window.top + window.height, width: window.width, height: window.height }) as DOMRect;
        compositor.windows.set(window.id, { element, geometry: createWindowFrameGeometry({ width: window.width, height: window.height, top: window.topEdge, bottom: window.bottomEdge }), state: "normal", update: segments => painted.set(window.id, { left: window.left, top: window.top, segments }) });
      }
      try {
        compositor.schedule();
        for (const line of fixture.lines) {
          const spans: { id: string; at: number; low: number; high: number }[] = [];
          for (const [id, { left, top, segments }] of painted) for (const segment of segments) {
            const points = [...segment.d.matchAll(/(-?\d+(?:\.\d+)?) (-?\d+(?:\.\d+)?)/g)].map(match => ({ x: left + Number(match[1]), y: top + Number(match[2]) }));
            points.slice(1).forEach((to, index) => {
              const from = points[index]!;
              const along = line.axis === "h" ? [from.x, to.x] : [from.y, to.y];
              const normal = line.axis === "h" ? [from.y, to.y] : [from.x, to.x];
              if (Math.abs(normal[0]! - normal[1]!) < 0.01 && Math.abs(normal[0]! - line.at) <= 1) spans.push({ id, at: normal[0]!, low: Math.max(line.from, Math.min(...along)), high: Math.min(line.to, Math.max(...along)) });
            });
          }
          const live = spans.filter(span => span.high - span.low > 1.5);
          const normals = [...new Set(live.map(span => span.at))];
          expect({ name: fixture.name, straight: Math.max(...normals) - Math.min(...normals) < 0.01 }).toEqual({ name: fixture.name, straight: true });
          const doubled = live.some((a, i) => live.some((b, j) => j > i && a.id !== b.id && Math.min(a.high, b.high) - Math.max(a.low, b.low) > 1.5));
          const sorted = [...live].sort((a, b) => a.low - b.low);
          let reach = line.from;
          for (const span of sorted) if (span.low <= reach + 1.5) reach = Math.max(reach, span.high);
          expect({ name: fixture.name, line, doubled, covered: reach >= line.to - 1.5 }).toEqual({ name: fixture.name, line, doubled: false, covered: true });
        }
      } finally {
        compositor.dispose();
        Object.assign(globalThis, previous);
        dom.window.close();
      }
    }
  });
  test("band controls flush with the navbar or footer share its edges and paint no outer edge the band leaves open", () => {
    const dom = new JSDOM('<div class="semio-scope"><nav data-shared-border-frame data-shared-border-edges="bottom"><div data-shared-border-frame data-shared-border-edges="top right bottom left"></div></nav><footer data-shared-border-frame data-shared-border-edges="top"><div data-shared-border-frame></div></footer></div>');
    const rect = (left: number, top: number, right: number, bottom: number) => ({ left, top, right, bottom, width: right - left, height: bottom - top }) as DOMRect;
    const [group, inset] = [dom.window.document.querySelector<HTMLElement>("nav > div")!, dom.window.document.querySelector<HTMLElement>("footer > div")!];
    const nav = dom.window.document.querySelector<HTMLElement>("nav")!, footer = dom.window.document.querySelector<HTMLElement>("footer")!;
    nav.getBoundingClientRect = () => rect(0, 0, 400, 28);
    footer.getBoundingClientRect = () => rect(0, 72, 400, 100);
    const parentBox = (id: string, edges: BorderSide[], box: DOMRect): OutlineBox => ({ id, parent: null, owner: id, rect: box, edges, state: "normal" });
    const drawn = (parent: OutlineBox, box: DOMRect) => boxDrawnSides({ id: "c", parent: parent.id, owner: "c", rect: box, edges: ["top", "right", "bottom", "left"], state: "normal" }, parent);
    const bar = parentBox("nav", ["bottom"], rect(0, 0, 400, 28)), band = parentBox("footer", ["top"], rect(0, 72, 400, 100));
    expect(drawn(bar, rect(10, 0, 90, 28))).toEqual(["right", "bottom", "left"]);
    expect(drawn(bar, rect(10, 3, 90, 25))).toEqual(["top", "right", "bottom", "left"]);
    expect(drawn(band, rect(10, 72, 90, 100))).toEqual(["top", "right", "left"]);
    expect(group.isConnected && inset.isConnected).toBe(true);
    dom.window.close();
  });
  test("navbar and footer items span the band height and stretch their framed controls", () => {
    const here = import.meta.dirname;
    const navbar = readFileSync(resolve(here, "../🔝️Navbar/🟦️.tsx"), "utf8");
    expect(navbar.split("\n").filter(line => !line.trimStart().startsWith("*") && line.includes("h-medium"))).toEqual([]);
    expect(navbar.match(/h-full flex/g)?.length).toBe(2);
    const css = readFileSync(resolve(here, "../../🎨️styling/🖌️ui/🎨️.css"), "utf8");
    expect(css).toMatch(/:is\(\[data-slot="navbar"\], \[data-slot="footer"\]\) \[data-shared-border-frame\]:not\([^)]*\) \{\s*height: auto;/);
  });

  test("elements defer their edges to the shared mechanism instead of stacking native ones", () => {
    const here = import.meta.dirname;
    const read = (path: string) => readFileSync(resolve(here, path), "utf8");
    const layout = read("../📐️Layout/🟦️.tsx");
    expect(layout).not.toContain("chrome-frame");
    expect(layout).toMatch(/ref=\{frameRef\}\s+data-shared-border-frame=""/);
    const canvas = read("../🎨️Canvas/🟦️.tsx");
    expect(canvas).toMatch(/data-slot="app-mode-nav" data-shared-border-frame="" data-shared-border-cells="" data-shared-border-edges="bottom"/);
    expect(canvas).not.toMatch(/"[^"]*\bborder-b\b[^"]*"/);
    const history = read("../🕰️HistoryTable/🟦️.tsx");
    expect(history).not.toContain("border-[var(--border)]");
    expect(history.match(/data-slot="history-row-divider"/g)?.length).toBe(1);
    expect(history).toContain("index < sorted.length - 1");
    const css = read("../../🎨️styling/🖌️ui/🎨️.css");
    expect(css).not.toMatch(/data-slot="(chrome-frame|side-panel-tabs|mobile-panel-tabs|accordion-item)"/);
    expect(css).toMatch(/table > tbody > tr:last-child \{\s*border-bottom-width: 0;/);
  });
});

describe("state outlines of product shapes under mocked layout", () => {
  const rect = (left: number, top: number, width: number, height: number) => ({ left, top, right: left + width, bottom: top + height, width, height, x: left, y: top, toJSON: () => ({}) }) as DOMRect;
  const harness = (html: string, rects: Record<string, DOMRect>) => {
    const dom = new JSDOM(html);
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!;
    for (const [id, box] of Object.entries(rects)) root.querySelector<HTMLElement>(`#${id}`)!.getBoundingClientRect = () => box;
    const compositor = new BorderCompositor(root);
    const hover = (id: string) => root.querySelector(`#${id}`)!.dispatchEvent(new dom.window.MouseEvent("pointerover", { bubbles: true }));
    const painted = (state: string) => [...root.querySelectorAll(`svg [data-shared-border-state="${state}"]`)].map(path => path.getAttribute("d")).sort();
    return { dom, root, compositor, hover, painted, done: () => { compositor.dispose(); Object.assign(globalThis, previous); dom.window.close(); } };
  };
  const closed = (left: number, right: number, top: number, bottom: number) => [`M ${left} ${top} L ${left} ${bottom}`, `M ${right} ${top} L ${right} ${bottom}`, `M ${left} ${top} L ${right} ${top}`, `M ${left} ${bottom} L ${right} ${bottom}`].sort();
  const lines = (segments: readonly { d: string }[]) => segments.flatMap(segment => [...segment.d.matchAll(/([ML]) (-?[\d.]+) (-?[\d.]+)/g)].map(match => ({ x: Number(match[2]), y: Number(match[3]) })));

  describe("mode dock tabs with several interactive controls", () => {
    const dock = (state: "active" | "normal") => {
      const html = `<div class="semio-scope"><div id="stack" data-window-frame><div id="chip" data-window-frame-chip data-dock="top"><div id="tabs" role="tablist" style="overflow-x:auto;overflow-y:hidden"><div id="tab" data-slot="mode-dock-tab" ${state === "active" ? 'data-active="true"' : ""}><button id="select" role="tab"></button><button id="focusing" data-slot="mode-dock-tab-focus"></button><button id="close" data-slot="mode-dock-tab-close"></button></div></div></div></div></div>`;
      const h = harness(html, { stack: rect(0, 0, 100, 100), chip: rect(0, 0, 60, 30), tabs: rect(0, 0, 60, 30), tab: rect(0, 0, 60, 30), select: rect(0, 0, 30, 30), focusing: rect(30, 0, 15, 30), close: rect(45, 0, 15, 30) });
      const windowGeometry = createWindowFrameGeometry({ width: 100, height: 100, top: { depth: 30, chips: [{ left: 0, right: 60 }] }, bottom: { depth: 0, chips: [] } });
      const updates: { d: string; state: string }[][] = [];
      h.compositor.windows.set("w", { element: h.root.querySelector<HTMLElement>("#stack")!, geometry: windowGeometry, state: state === "active" ? "active" : "normal", update: value => { updates.push([...value]); } });
      h.compositor.schedule();
      const all = (state: string) => [...(updates.at(-1) ?? []).filter(segment => segment.state === state), ...[...h.root.querySelectorAll(`#tab svg [data-shared-border-state="${state}"]`)].map(path => ({ d: path.getAttribute("d")!, state }))];
      return { h, updates, all };
    };

    test("hovering any control of the tab states the tab box alone, and its closed bottom lies on the chip inner edge", () => {
      const { h, updates, all } = dock("active");
      try {
        h.hover("close");
        const hovered = all("hover");
        expect(hovered.length).toBeGreaterThan(0);
        expect(h.painted("hover")).toEqual(expect.arrayContaining(["M 1 1 L 59 1", "M 1 29 L 59 29", "M 59 1 L 59 29", "M 1 1 L 1 29"]));
        const ys = new Set(lines(hovered).map(point => point.y));
        expect([...ys].sort((a, b) => a - b)).toEqual([1, 29]);
        const normalOrActive = (updates.at(-1) ?? []).filter(segment => segment.state !== "hover");
        const horizontalAtCap = lines(normalOrActive).map(point => point.y).filter(y => y > 25 && y < 35);
        expect(new Set(horizontalAtCap)).toEqual(new Set([29]));
        const verticalRight = lines(hovered).filter(point => point.x > 50);
        expect(new Set(verticalRight.map(point => point.x))).toEqual(new Set([59]));
      } finally { h.done(); }
    });

    test("the closed tab bottom and the chip inner edge are one line without staircase jogs, whatever the tab edge offset within two pixels", () => {
      for (const bottom of [30, 29, 31]) {
        const { h, updates } = dock("active");
        try {
          h.root.querySelector<HTMLElement>("#tab")!.getBoundingClientRect = () => rect(0, 0, 60, bottom);
          h.root.querySelector<HTMLElement>("#chip")!.getBoundingClientRect = () => rect(0, 0, 60, bottom);
          h.compositor.schedule();
          h.hover("select");
          const segments = updates.at(-1) ?? [];
          const horizontal = segments.flatMap(segment => { const points = lines([segment]); return points.length >= 2 && points[0]!.y === points[1]!.y ? [points[0]!.y] : []; });
          expect(horizontal.filter(y => y > bottom - 4 && y < bottom + 4), `tab bottom ${bottom}`).toSatisfy((ys: number[]) => new Set(ys).size <= 1);
        } finally { h.done(); }
      }
    });

    test("a focused tab states only itself and the chip and window keep their own state", () => {
      const { h, updates } = dock("normal");
      try {
        h.root.querySelector<HTMLElement>("#select")!.focus();
        h.hover("select");
        const hovered = (updates.at(-1) ?? []).filter(segment => segment.state === "hover");
        expect(Math.max(...lines(hovered).map(point => point.x))).toBeLessThanOrEqual(59);
        expect(Math.max(...lines(hovered).map(point => point.y))).toBeLessThanOrEqual(29);
        expect((updates.at(-1) ?? []).some(segment => segment.state === "normal")).toBe(true);
      } finally { h.done(); }
    });
  });

  test("a bare input that is a direct child of a cells owner is a cell: hover emphasises its divider and the row's bottom line in the row's overlay", () => {
    const h = harness('<div class="semio-scope"><div id="row" data-shared-border-cells data-shared-border-frame data-shared-border-edges="top bottom"><div id="label" data-shared-border-cell></div><input id="name" /></div></div>', { row: rect(0, 0, 100, 60), label: rect(0, 0, 100, 20), name: rect(0, 20, 100, 40) });
    try {
      h.compositor.schedule();
      h.hover("name");
      expect(h.painted("hover")).toEqual(["M 1 20 L 99 20", "M 1 59 L 99 59"]);
      expect(h.root.querySelector("#name > svg")).toBeNull();
      h.root.querySelector<HTMLElement>("#name")!.focus();
      expect(isSharedBorderCell(h.root.querySelector("#name")!)).toBe(true);
      expect(isSharedBorderCell(h.root.querySelector("#label")!)).toBe(true);
    } finally { h.done(); }
  });

  test("a hidden input and an input outside a cells owner are not cells", () => {
    const h = harness('<div class="semio-scope"><div id="row" data-shared-border-cells data-shared-border-group><input id="secret" type="hidden" /></div><input id="free" /></div>', {});
    try {
      expect(isSharedBorderCell(h.root.querySelector("#secret")!)).toBe(false);
      expect(isSharedBorderCell(h.root.querySelector("#free")!)).toBe(false);
    } finally { h.done(); }
  });

  describe("controls inside a horizontally scrolling table", () => {
    const table = (scrolledOut: boolean) => harness('<div class="semio-scope"><div id="frame" data-shared-border-frame><div id="region" style="overflow-x:auto"><div id="group" data-shared-border-frame style="border:1px solid"><button id="suspend" data-shared-border-cell></button></div></div></div></div>', { frame: rect(0, 0, 100, 40), region: rect(1, 1, 98, 38), group: scrolledOut ? rect(150, 10, 32, 20) : rect(60, 10, 32, 20), suspend: scrolledOut ? rect(151, 11, 30, 18) : rect(61, 11, 30, 18) });

    test("a control scrolled out of its scroller paints nothing and states no ancestor, and closes fully once it is in view", () => {
      const out = table(true);
      try {
        out.compositor.schedule();
        out.hover("suspend");
        expect(out.painted("hover")).toEqual([]);
        expect(out.root.querySelector("#frame > svg [data-shared-border-state='hover']")).toBeNull();
      } finally { out.done(); }
      const shown = table(false);
      try {
        shown.compositor.schedule();
        shown.hover("suspend");
        expect(shown.painted("hover")).toEqual(closed(1, 31, 1, 19));
      } finally { shown.done(); }
    });
  });
});

describe("compositor commit economy", () => {
  const setup = (boxes: number, figures: number, nodesPerFigure: number) => {
    const root = document.createElement("div");
    root.className = "semio-scope";
    for (let box = 0; box < boxes; box++) { const frame = document.createElement("div"); frame.setAttribute("data-shared-border-frame", ""); frame.className = "box"; root.append(frame); }
    for (let figure = 0; figure < figures; figure++) {
      const host = document.createElement("div");
      host.className = "figure";
      host.setAttribute("data-time", "0");
      host.innerHTML = `<svg><path d="M0 0"></path>${"<g><rect></rect></g>".repeat(nodesPerFigure / 2)}</svg>`;
      root.append(host);
    }
    document.body.append(root);
    const queue = new Map<number, FrameRequestCallback>();
    let next = 0;
    const request = vi.spyOn(window, "requestAnimationFrame").mockImplementation(callback => { queue.set(++next, callback); return next; });
    vi.spyOn(window, "cancelAnimationFrame").mockImplementation(id => { queue.delete(id); });
    const flush = async () => { await Promise.resolve(); const callbacks = [...queue.values()]; queue.clear(); callbacks.forEach(callback => callback(0)); };
    return { root, queue, flush, done: () => { request.mockRestore(); root.remove(); } };
  };

  test("300 boxes and 20,000 nodes: 1,000 animated mutations inside non-box descendants commit nothing", async () => {
    const { root, queue, flush, done } = setup(300, 100, 200);
    const lease = retainSharedBorderRoot(root);
    try {
      expect(root.querySelectorAll("*").length).toBeGreaterThan(20_000);
      await flush();
      expect(lease.compositor.commits).toBe(1);
      const figures = [...root.querySelectorAll<HTMLElement>(".figure")];
      const paths = figures.map(figure => figure.querySelector("path")!);
      for (let i = 0; i < 1000; i++) {
        const figure = figures[i % figures.length]!;
        if (i % 4 === 0) paths[i % paths.length]!.setAttribute("d", `M${i} 0`);
        else if (i % 4 === 1) figure.setAttribute("data-time", String(i));
        else if (i % 4 === 2) figure.setAttribute("aria-label", `t${i}`);
        else figure.querySelector("rect")!.setAttribute("style", `opacity:${i / 1000}`);
        if (i % 250 === 0) figure.innerHTML = `<svg><path d="M${i} 1"></path></svg>`;
        if (i % 100 === 0) { await flush(); figure.style.opacity = String(i); }
      }
      await flush();
      expect(lease.compositor.commits).toBe(1);
      expect(queue.size).toBe(0);
    } finally { lease.release(); done(); }
  });

  test("a mutation on a box or its ancestor, or a new box, commits once; geometry commits reuse the cached box list", async () => {
    const { root, flush, done } = setup(300, 10, 100);
    const lease = retainSharedBorderRoot(root);
    const lookups = vi.spyOn(root, "querySelectorAll");
    try {
      await flush();
      expect(lease.compositor.commits).toBe(1);
      lookups.mockClear();
      root.querySelector<HTMLElement>(".box")!.style.width = "10px";
      root.style.opacity = "0.5";
      await flush();
      expect(lease.compositor.commits).toBe(2);
      expect(lookups).not.toHaveBeenCalled();
      const added = document.createElement("div");
      added.setAttribute("data-shared-border-frame", "");
      root.append(added);
      await flush();
      expect(lease.compositor.commits).toBe(3);
      expect(lookups).toHaveBeenCalled();
      added.setAttribute("data-active", "true");
      await flush();
      expect(lease.compositor.commits).toBe(4);
    } finally { lease.release(); done(); }
  });

  test("a sibling that grows above a hovered button re-measures it on the next state commit and its parent is resize-observed", async () => {
    const dom = new JSDOM('<div class="semio-scope"><div id="column"><p>one line</p><button data-shared-border-frame>go</button></div></div>');
    const observed = new Set<Element>();
    class Resize { observe(element: Element) { observed.add(element); } unobserve(element: Element) { observed.delete(element); } disconnect() { observed.clear(); } }
    Object.assign(dom.window, { ResizeObserver: Resize });
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver, ResizeObserver: globalThis.ResizeObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver, ResizeObserver: Resize });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!;
    const button = root.querySelector<HTMLButtonElement>("button")!, paragraph = root.querySelector("p")!;
    let top = 20;
    const reads: number[] = [];
    button.getBoundingClientRect = () => (reads.push(top), { left: 0, top, right: 100, bottom: top + 30, width: 100, height: 30, x: 0, y: top, toJSON: () => ({}) }) as DOMRect;
    const compositor = new BorderCompositor(root);
    compositor.schedule();
    try {
      expect(observed.has(root.querySelector("#column")!)).toBe(true);
      button.dispatchEvent(new dom.window.MouseEvent("pointerover", { bubbles: true }));
      expect(button.querySelector("path")?.getAttribute("data-shared-border-state")).toBe("hover");
      expect(reads.at(-1)).toBe(20);
      const svg = button.querySelector("svg")!;
      const sizeChanges: MutationRecord[] = [];
      const observer = new dom.window.MutationObserver(records => sizeChanges.push(...records));
      observer.observe(svg, { attributes: true, attributeFilter: ["width", "height", "viewBox"] });
      top = 50;
      paragraph.textContent = "one line\nthat grows\nto three lines";
      await Promise.resolve();
      button.dispatchEvent(new dom.window.MouseEvent("pointerout", { bubbles: true }));
      button.dispatchEvent(new dom.window.MouseEvent("pointerover", { bubbles: true }));
      expect(reads.at(-1)).toBe(50);
      expect(button.querySelector("path")?.getAttribute("data-shared-border-state")).toBe("hover");
      expect(sizeChanges.length + observer.takeRecords().length).toBe(0);
      observer.disconnect();
    } finally {
      compositor.dispose(); Object.assign(globalThis, previous); dom.window.close();
    }
  });

  test("a resize notification schedules a commit only when an observed size really changed, and the compositor's own pending marker never schedules", async () => {
    const dom = new JSDOM('<div class="semio-scope"><div id="column"><button data-shared-border-frame>go</button></div></div>');
    let notify: ResizeObserverCallback = () => undefined;
    class Resize { constructor(callback: ResizeObserverCallback) { notify = callback; } observe() {} unobserve() {} disconnect() {} }
    Object.assign(dom.window, { ResizeObserver: Resize });
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver, ResizeObserver: globalThis.ResizeObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver, ResizeObserver: Resize });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!;
    const column = root.querySelector("#column")!;
    const compositor = new BorderCompositor(root);
    const entry = (width: number) => [{ target: column, contentRect: { width, height: 10 } }] as unknown as ResizeObserverEntry[];
    try {
      compositor.schedule();
      const base = compositor.commits;
      notify(entry(100), {} as ResizeObserver);
      notify(entry(100), {} as ResizeObserver);
      expect(compositor.commits).toBe(base);
      notify(entry(120), {} as ResizeObserver);
      expect(compositor.commits).toBe(base + 1);
      root.setAttribute("data-shared-border-pending", "");
      root.removeAttribute("data-shared-border-pending");
      await Promise.resolve();
      expect(compositor.commits).toBe(base + 1);
    } finally {
      compositor.dispose(); Object.assign(globalThis, previous); dom.window.close();
    }
  });
});
