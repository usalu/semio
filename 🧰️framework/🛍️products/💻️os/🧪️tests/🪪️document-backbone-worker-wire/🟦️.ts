/** 🪪️ Document backbone worker wire: admitting inference openings, requests and responses of the backbone worker.
 *
 * @see ../🧮️directory-client-helpers/🟦️.ts — the shared helpers
 * @see ../../🟦️.ts — the package entry the suite reads fixtures relative to
 */
import { describe } from "vitest";
import { registerAdmitsInferenceOpeningOnlyTests } from "./🔶️admits-inference-opening-only/🟦️.ts";
import { registerSettlesOnlyExactLiveTests } from "./🔸️settles-only-exact-live/🟦️.ts";
import { registerPreservesCompleteBrowserUiTests } from "./🟠️preserves-complete-browser-ui/🟦️.ts";

describe("document backbone worker wire", () => {
  registerAdmitsInferenceOpeningOnlyTests();
  registerSettlesOnlyExactLiveTests();
  registerPreservesCompleteBrowserUiTests();
});
