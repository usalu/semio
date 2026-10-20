// #region 🔌️Adapters

import { cleanup } from "@testing-library/react";
import { afterEach, describe } from "vitest";
import { registerMountsListboxOptionsOnlyTests } from "./🔶️mounts-listbox-options-only/🟦️.tsx";
import { registerKeepsMeasuredPopupMinimumTests } from "./🔸️keeps-measured-popup-minimum/🟦️.tsx";
import { registerHonorsPreventableEscapeOutsideTests } from "./🟠️honors-preventable-escape-outside/🟦️.tsx";
import { registerUsesOwnedRtlPlacementTests } from "./🟢️uses-owned-rtl-placement/🟦️.tsx";
// #endregion 🔌️Adapters

// #region ☑️SelectMatrix
afterEach(() => cleanup());

describe("Select", () => {
  registerMountsListboxOptionsOnlyTests();
  registerKeepsMeasuredPopupMinimumTests();
  registerHonorsPreventableEscapeOutsideTests();
  registerUsesOwnedRtlPlacementTests();
});
// #endregion ☑️SelectMatrix
