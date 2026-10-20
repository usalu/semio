// #region 🔌️Adapters

import { describe } from "vitest";
import { registerOwnsUncontrolledStateExactTests } from "./🔶️owns-uncontrolled-state-exact/🟦️.tsx";
import { registerDismissesNestedSiblingDialogsTests } from "./🔸️dismisses-nested-sibling-dialogs/🟦️.tsx";
import { registerInheritsOneScopedIsolationTests } from "./🟠️inherits-one-scoped-isolation/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 💬️DialogMatrix
describe("Dialog", () => {
  registerOwnsUncontrolledStateExactTests();
  registerDismissesNestedSiblingDialogsTests();
  registerInheritsOneScopedIsolationTests();
});
// #endregion 💬️DialogMatrix
