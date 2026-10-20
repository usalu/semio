/** 📐️ The presentation react deck, interaction geometry: ephemeral layout, marquee rules, flow offsets, measured bounds, resize and enlarge math.
 *
 * @see ../🧰️presentation-deck-helpers/🟦️.tsx — the shared source locator and helpers
 * @see ../../🎯️targets/⚛️react/🟦️.tsx — the module under test
 */
import { describe } from "vitest";
import { registerDetectsDispositionEphemeralLayoutTests } from "./🔶️detects-disposition-ephemeral-layout/🟦️.tsx";
import { registerDetectsFlowPixelOffsetTests } from "./🔸️detects-flow-pixel-offset/🟦️.tsx";
import { registerKeepsRevealMorphDataTests } from "./🟠️keeps-reveal-morph-data/🟦️.tsx";

describe("presentation interaction geometry", () => {
  registerDetectsDispositionEphemeralLayoutTests();
  registerDetectsFlowPixelOffsetTests();
  registerKeepsRevealMorphDataTests();
});
