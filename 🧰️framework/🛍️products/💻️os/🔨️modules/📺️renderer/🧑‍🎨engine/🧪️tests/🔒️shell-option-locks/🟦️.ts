/** 🔒 Shell option locks. */
import { describe } from "vitest";
import "../🧰️engine-contract-helpers/🟦️.ts";
import { registerBootLocksAndDefaultsTests } from "./🔐️boot-locks-and-defaults/🟦️.ts";
import { registerExampleOffersAndSwitchingTests } from "./📚️example-offers-and-switching/🟦️.ts";
import { registerLeftoverInteractionViewTests } from "./👁️leftover-interaction-views/🟦️.ts";
import { registerLeftoverOverlayScopeTests } from "./🫧️leftover-overlay-scopes/🟦️.ts";
import { registerRefreshScopeAndExamplePickerTests } from "./🔄️refresh-scopes-and-example-picker/🟦️.ts";
import { registerIntroductionAndOsCommandTests } from "./🎓️introduction-and-os-commands/🟦️.ts";

describe("shell option locks (SEMIO_LOCKED_*)", () => {
  registerBootLocksAndDefaultsTests();
  registerExampleOffersAndSwitchingTests();
  registerLeftoverInteractionViewTests();
  registerLeftoverOverlayScopeTests();
  registerRefreshScopeAndExamplePickerTests();
  registerIntroductionAndOsCommandTests();
});
