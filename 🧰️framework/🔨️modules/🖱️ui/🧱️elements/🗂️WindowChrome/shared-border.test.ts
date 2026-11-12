import { describe, expect, test, vi } from "vitest";
import * as React from "react";
import { createRoot } from "react-dom/client";
import { JSDOM } from "jsdom";
import Ajv from "ajv/dist/2020.js";
import cases from "./compositor.cases.json";
import schema from "./compositor.schema.json";
import { BorderCompositor, retainSharedBorderRoot, useSharedBorderFrameRef, useSharedWindowBorder } from "./shared-border.ts";
import { measureWindowSilhouetteMetrics } from "./🟦️.tsx";
import { createWindowSilhouetteGeometry } from "../🔲️WindowSilhouette/🟦️.tsx";

const geometry = createWindowSilhouetteGeometry({ width: 100, height: 100, top: { depth: 0, chips: [] }, bottom: { depth: 0, chips: [] } });

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
  test("marker-only sibling frames share ownership and independent participant lifetimes", () => {
    const dom = new JSDOM('<div class="semio-scope"><div id="left" data-shared-border-frame></div><div id="right" data-shared-border-frame></div></div><div id="other" class="semio-scope"></div>');
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
    const dom = new JSDOM('<div class="semio-scope"><div id="visible" data-shared-border-frame></div><div id="clip" style="overflow:hidden"><div id="hidden" data-shared-border-frame data-active="true"></div></div></div>');
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
          expect(overlay.querySelector("mask .window-silhouette-border-celebrated-mask")).not.toBeNull();
          expect(overlay.querySelector("foreignObject .window-silhouette-border-celebrated-fill")).not.toBeNull();
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
  test("unequally scaled partial overlaps connect orthogonally without moving on hover", () => {
    const dom = new JSDOM('<div class="semio-scope"><div id="parent"><div id="child"></div></div></div>');
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!, parent = root.querySelector<HTMLElement>("#parent")!, child = root.querySelector<HTMLElement>("#child")!;
    parent.getBoundingClientRect = () => ({ left: 0, top: 0, width: 100, height: 100 }) as DOMRect;
    child.getBoundingClientRect = () => ({ left: 0, top: 30, width: 100, height: 40 }) as DOMRect;
    Object.defineProperties(child, { offsetWidth: { value: 50 }, offsetHeight: { value: 20 } });
    const childGeometry = createWindowSilhouetteGeometry({ width: 50, height: 20, top: { depth: 0, chips: [] }, bottom: { depth: 0, chips: [] } });
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
    const dom = new JSDOM('<div class="semio-scope"><div id="parent"><div id="child"></div></div></div>');
    const previous = { Element: globalThis.Element, MutationObserver: globalThis.MutationObserver };
    Object.assign(globalThis, { Element: dom.window.Element, MutationObserver: dom.window.MutationObserver });
    const root = dom.window.document.querySelector<HTMLElement>(".semio-scope")!, parent = root.querySelector<HTMLElement>("#parent")!, child = root.querySelector<HTMLElement>("#child")!;
    const f = cases.fractional;
    const rect = (top: number, width: number, height: number) => ({ left: 0, top, width, height, right: width, bottom: top + height }) as DOMRect;
    parent.getBoundingClientRect = () => rect(0, f.parentWidth, f.parentHeight);
    child.getBoundingClientRect = () => rect(f.parentHeight - f.childHeight, f.childWidth, f.childHeight);
    Object.defineProperties(parent, { offsetWidth: { value: Math.round(f.parentWidth) }, offsetHeight: { value: Math.round(f.parentHeight) } });
    Object.defineProperties(child, { offsetWidth: { value: Math.round(f.childWidth) }, offsetHeight: { value: Math.round(f.childHeight) } });
    const shape = (width: number, height: number) => createWindowSilhouetteGeometry({ width, height, top: { depth: 0, chips: [] }, bottom: { depth: 0, chips: [] } });
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
  test("clips measured fusion wings to the visible scrolling viewport", () => {
    const dom = new JSDOM('<div data-window-silhouette><div data-slot="context-menu-fusion-body"><div data-slot="context-menu-fusion-primary"></div><div data-slot="context-menu-submenu"></div></div></div>');
    const stack = dom.window.document.querySelector<HTMLElement>("[data-window-silhouette]")!;
    const rect = (left: number, top: number, right: number, bottom: number) => ({ left, top, right, bottom, width: right - left, height: bottom - top }) as DOMRect;
    stack.getBoundingClientRect = () => rect(0, 0, 100, 100);
    stack.querySelector<HTMLElement>('[data-slot="context-menu-fusion-body"]')!.getBoundingClientRect = () => rect(0, 20, 100, 100);
    stack.querySelector<HTMLElement>('[data-slot="context-menu-fusion-primary"]')!.getBoundingClientRect = () => rect(0, 20, 60, 100);
    stack.querySelector<HTMLElement>('[data-slot="context-menu-submenu"]')!.getBoundingClientRect = () => rect(60, 40, 160, 150);
    expect(measureWindowSilhouetteMetrics(stack)?.contextMenuFusion?.wings).toEqual([{ left: 60, top: 40, right: 100, bottom: 100 }]);
    dom.window.close();
  });
  test("deduplicates reversed-order sibling seams and restores detached frames", () => {
    const dom = new JSDOM('<div class="semio-scope"><div id="left"></div><div id="right"></div><nav data-shared-border-frame data-shared-border-edges="bottom"></nav></div>');
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
      const childGeometry = createWindowSilhouetteGeometry({ width: 50, height: 40, top: { depth: 0, chips: [] }, bottom: { depth: 0, chips: [] } });
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
});
