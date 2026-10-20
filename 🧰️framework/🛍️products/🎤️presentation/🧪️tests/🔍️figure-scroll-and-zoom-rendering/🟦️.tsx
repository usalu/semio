/** 🔍️ The presentation react deck, rendering of the figure scroll, cover and zoom math, pdf page subsets, crop sizing and mosaic windows. */
import { describe } from "vitest";
import { registerResolvepresentationasseturlMapsDeckRelativeTests } from "./🔶️resolvepresentationasseturl-maps-deck-re/🟦️.tsx";
import { registerRendersClippedCropFiguresTests } from "./🔸️renders-clipped-crop-figures/🟦️.tsx";
import { registerSetsRevealmorphfrommorphtoframePreviousSlideTests } from "./🟠️sets-revealmorphfrommorphtoframe-previou/🟦️.tsx";

describe("PresentationDeck", () => {
  registerResolvepresentationasseturlMapsDeckRelativeTests();
  registerRendersClippedCropFiguresTests();
  registerSetsRevealmorphfrommorphtoframePreviousSlideTests();
});
