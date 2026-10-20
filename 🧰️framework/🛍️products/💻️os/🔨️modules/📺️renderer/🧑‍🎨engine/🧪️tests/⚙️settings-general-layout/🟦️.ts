/** ⚙️ Neutral and React oracle for General Tree projection and bottom-panel flow. */
import { describe } from "vitest";
import { registerActualReactStepperMatchesTests } from "./🔶️actual-react-stepper-matches/🟦️.ts";
import { registerReactNeutralGeometryKeepTests } from "./🔸️react-neutral-geometry-keep/🟦️.ts";
import { registerWgpuPublishesGeneralDirectlyTests } from "./🟠️wgpu-publishes-general-directly/🟦️.ts";

describe("⚙️ General Settings Tree and bottom-panel flow", () => {
  registerActualReactStepperMatchesTests();
  registerReactNeutralGeometryKeepTests();
  registerWgpuPublishesGeneralDirectlyTests();
});
