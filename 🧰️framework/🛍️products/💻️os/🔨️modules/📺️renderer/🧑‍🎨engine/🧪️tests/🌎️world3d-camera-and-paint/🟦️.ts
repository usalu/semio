/** 🌎 World 3d empty state, camera framing, mesh paint and context target resolution. */
import { describe } from "vitest";
import { registerRendersWorld3dEmptyTests } from "./🔶️renders-world-3d-empty/🟦️.ts";
import { registerFramedCameraPutsCornerTests } from "./🔸️framed-camera-puts-corner/🟦️.ts";
import { registerScopesSuggestionMenuOwnershipTests } from "./🟠️scopes-suggestion-menu-ownership/🟦️.ts";

describe("framework renderer hosts", () => {
  registerRendersWorld3dEmptyTests();
  registerFramedCameraPutsCornerTests();
  registerScopesSuggestionMenuOwnershipTests();
});
