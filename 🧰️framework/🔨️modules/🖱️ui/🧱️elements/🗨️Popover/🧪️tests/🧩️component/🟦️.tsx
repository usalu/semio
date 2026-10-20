// #region 🔌️Adapters

import { describe } from "vitest";
import { registerOwnsUncontrolledDefaultControlledTests } from "./🔶️owns-uncontrolled-default-controlled/🟦️.tsx";
import { registerRoutesSiblingEscapeFocusTests } from "./🔸️routes-sibling-escape-focus/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🗨️PopoverMatrix
describe("Popover", () => {
  registerOwnsUncontrolledDefaultControlledTests();
  registerRoutesSiblingEscapeFocusTests();
});
// #endregion 🗨️PopoverMatrix
