/** 🫥️ The pet layer is decoration and behaves like it: one static, hidden element that no assistive technology and no
 * keyboard ever meets (judged by `@testing-library`'s accessibility tree and the role model of `aria-query`) and that
 * a fine pointer never hits — a press reaches the pets only through the hand's listeners on the window, never a
 * press on a control —, in which the cast of a scene appears on the top edges of the page's surfaces, keeps living
 * outside React, stands still when told to, runs nothing while the document is hidden or forced colours are active,
 * and leaves no listener, observer, frame, timer or element behind when it goes. Nothing is ever written to the
 * console. What the hand hands the stage is held in `🤏️pet-handling`.
 *
 * @see ../../🎯️targets/⚛️react/🔨️modules/🫧️layer/🟦️.tsx
 * @see ../🤏️pet-handling/🟦️.tsx
 * @see ../../🎯️targets/⚛️react/🎨️.css
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pause-stop-hide.html
 */
import { cleanup } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, vi, type MockInstance } from "vitest";
import { registerSpeciesAndAccessibilityTests } from "./♿️species-and-accessibility/🟦️.tsx";
import { registerPointerAndStylesheetPolicyTests } from "./🎨️pointer-and-stylesheet-policy/🟦️.tsx";
import { registerStillAndLivingStageTests } from "./🎞️still-and-living-stage-behaviour/🟦️.tsx";
import { registerEventsCastsAndTimeTests } from "./📡️events-casts-and-time/🟦️.tsx";
import { registerNarrowHiddenAndForcedColoursTests } from "./🌙️narrow-hidden-and-forced-colours/🟦️.tsx";
import { registerTeardownAndMountingTests } from "./🧹️teardown-and-mounting/🟦️.tsx";

const CONSOLE = ["log", "info", "warn", "error", "debug", "trace"] as const;
let spoken: MockInstance[] = [];

beforeEach(() => {
  vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "requestAnimationFrame", "cancelAnimationFrame", "performance"] });
  spoken = CONSOLE.map((method) => vi.spyOn(console, method).mockImplementation(() => {}));
});

afterEach(() => {
  cleanup();
  for (const [index, spy] of spoken.entries()) expect(spy.mock.calls, `console.${CONSOLE[index]}`).toEqual([]);
  expect(vi.getTimerCount()).toBe(0);
  document.body.replaceChildren();
  document.head.replaceChildren();
  Reflect.deleteProperty(document.documentElement, "clientWidth");
  Reflect.deleteProperty(document.documentElement, "clientHeight");
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
  vi.useRealTimers();
});

describe("🫥️ decorative layer", () => {
  registerSpeciesAndAccessibilityTests();
  registerPointerAndStylesheetPolicyTests();
  registerStillAndLivingStageTests();
  registerEventsCastsAndTimeTests();
  registerNarrowHiddenAndForcedColoursTests();
  registerTeardownAndMountingTests();
});
