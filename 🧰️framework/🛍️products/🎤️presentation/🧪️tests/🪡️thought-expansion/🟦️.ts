/** 🪡️ The presentation core, `expandThoughtSlides`: how thoughts expand into slides, authors, affiliations, descriptions and tiles. */
import { describe } from "vitest";
import { registerAssignsOneAutoAnimateTests } from "./🔶️assigns-one-auto-animate/🟦️.ts";
import { registerKeepsConsecutiveMorphSlidesTests } from "./🔸️keeps-consecutive-morph-slides/🟦️.ts";
import { registerStartsNewMorphRunTests } from "./🟠️starts-new-morph-run/🟦️.ts";

describe("expandThoughtSlides", () => {
  registerAssignsOneAutoAnimateTests();
  registerKeepsConsecutiveMorphSlidesTests();
  registerStartsNewMorphRunTests();
});
