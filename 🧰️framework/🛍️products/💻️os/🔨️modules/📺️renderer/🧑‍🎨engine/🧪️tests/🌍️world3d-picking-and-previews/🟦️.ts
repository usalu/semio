/** 🌍 World 3d picking, framing, shared previews and board world-to-screen math. */
import { describe } from "vitest";
import "../🧰️engine-contract-helpers/🟦️.ts";
import { registerRaycastsZ0GroundTests } from "./🔶️raycasts-z-0-ground/🟦️.ts";
import { registerSharesLiveWorldSelectionTests } from "./🔸️shares-live-world-selection/🟦️.ts";
import { registerCanvas2dSurfaceColorsTests } from "./🟠️canvas-2d-surface-colors/🟦️.ts";

describe("framework renderer hosts", () => {
  registerRaycastsZ0GroundTests();
  registerSharesLiveWorldSelectionTests();
  registerCanvas2dSurfaceColorsTests();
});
