// @vitest-environment jsdom

import { act } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { computeAccessibleName, fixture, getRole, mounted } from "./🟣️shared/🟦️.tsx";
import { registerValidatesAcceptedChromeEpochTests } from "./🔶️validates-accepted-chrome-epoch/🟦️.tsx";
import { registerCarriesOneMirrorActivationTests } from "./🔸️carries-one-mirror-activation/🟦️.tsx";
import { registerActivatesActionableTreeRowTests } from "./🟠️activates-actionable-tree-row/🟦️.tsx";
import { registerKeepsCanvasKeyboardDeliveryTests } from "./🟢️keeps-canvas-keyboard-delivery/🟦️.tsx";

(globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

afterEach(() => {
  vi.useRealTimers();
  while (mounted.length) act(() => mounted.pop()?.unmount());
  document.body.replaceChildren();
});

describe("wgpu accessibility interaction contract", () => {
  registerValidatesAcceptedChromeEpochTests();
  registerCarriesOneMirrorActivationTests();
  registerActivatesActionableTreeRowTests();
  registerKeepsCanvasKeyboardDeliveryTests();
});

it("keeps passive layout regions out of the native button vocabulary", () => {
  for (const row of fixture.passiveChrome) {
    const element = document.createElement(row.element);
    element.dataset.controlId = row.id;
    if (row.label) element.setAttribute("aria-label", row.label);
    document.body.append(element);
    expect(getRole(element) === "button").toBe(row.announced);
    if (row.label) expect(computeAccessibleName(element)).toBe(row.label);
    element.remove();
  }
});
