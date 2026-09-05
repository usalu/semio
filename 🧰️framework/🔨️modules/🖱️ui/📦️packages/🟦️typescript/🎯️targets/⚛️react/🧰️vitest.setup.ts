/** @emoji 🧪️ jsdom polyfills for `@semio-tech/ui-react` vitest. */
// #region 🔌️Adapters
import { cleanup } from "@testing-library/react";
import { afterEach } from "vitest";
// #endregion 🔌️Adapters

class ResizeObserverMock {
  observe() {}
  unobserve() {}
  disconnect() {}
}

globalThis.ResizeObserver = ResizeObserverMock as typeof ResizeObserver;

// jsdom does not implement scrollIntoView; tab strips call it on mount to reveal the active tab.
if (!Element.prototype.scrollIntoView) {
  Element.prototype.scrollIntoView = () => {};
}

// jsdom does not implement pointer capture; usePointerDrag (panel/window resize handles) calls it on pointerdown.
if (!Element.prototype.setPointerCapture) {
  Element.prototype.setPointerCapture = () => {};
}
if (!Element.prototype.hasPointerCapture) {
  Element.prototype.hasPointerCapture = () => false;
}
if (!Element.prototype.releasePointerCapture) {
  Element.prototype.releasePointerCapture = () => {};
}

// jsdom has no native PointerEvent constructor, so @testing-library/dom's fireEvent.pointer* falls back to a bare
// Event with none of MouseEvent's coordinate fields (clientX/Y) — usePointerDrag reads those, so pointer-drag tests
// (panel/window resize handles) would silently see `undefined` deltas without this shim.
if (typeof globalThis.PointerEvent === "undefined") {
  class PointerEventPolyfill extends MouseEvent {
    readonly pointerId: number;
    readonly pointerType: string;
    readonly isPrimary: boolean;
    constructor(type: string, params: PointerEventInit = {}) {
      super(type, params);
      this.pointerId = params.pointerId ?? 0;
      this.pointerType = params.pointerType ?? "mouse";
      this.isPrimary = params.isPrimary ?? true;
    }
  }
  globalThis.PointerEvent = PointerEventPolyfill as unknown as typeof PointerEvent;
}

afterEach(() => {
  cleanup();
});
