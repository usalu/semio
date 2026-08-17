/** @emoji 🧪️ jsdom polyfills for `@semio-tech/hub-admin` vitest — mirrors `ui-react`'s own
 * `🟦️vitest.setup.ts` (Radix `Select`/`Dialog` call these during tests, jsdom implements neither). */
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

if (!Element.prototype.scrollIntoView) {
  Element.prototype.scrollIntoView = () => {};
}
if (!Element.prototype.hasPointerCapture) {
  Element.prototype.hasPointerCapture = () => false;
}
if (!Element.prototype.setPointerCapture) {
  Element.prototype.setPointerCapture = () => {};
}
if (!Element.prototype.releasePointerCapture) {
  Element.prototype.releasePointerCapture = () => {};
}

afterEach(() => {
  cleanup();
});
