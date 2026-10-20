/** ⏱️ The clock of a timed task: closed until opened, running, warning, up and locked, in English and German, with the submission and the timers it leaves behind. */
import { cleanup } from "@testing-library/react";
import { afterEach, describe, vi } from "vitest";
import { registerClosedUntilOpenedRunsTests } from "./🔶️closed-until-opened-runs/🟦️.tsx";
import { registerTellsOnceTimeTaskTests } from "./🔸️tells-once-time-task/🟦️.tsx";

afterEach(() => {
  cleanup();
  vi.useRealTimers();
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
});

describe("⏱️ the clock of a timed task", () => {
  registerClosedUntilOpenedRunsTests();
  registerTellsOnceTimeTaskTests();
});
