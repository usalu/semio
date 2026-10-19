/** 🧾️ The TypedWire describe block: its wrapper, page fixture helper and parts. */
import type { ResidentTestKit } from "../🧰️resident-test-kit/🟦️.tsx";
import { createResidentPageHelpers } from "../🛠️page-fixture/🟦️.tsx";
import { registerPoolSlotAndScopeTests } from "../🏊️pool-slot-and-scope/🟦️.tsx";
import { registerCompositionAndPageTests } from "../📦️composition-and-page/🟦️.tsx";
import { registerPageLifecycleTests } from "../📄️page-lifecycle/🟦️.tsx";
import { registerReaderBindingTests } from "../👓️reader-binding/🟦️.tsx";
import { registerBuilderCopiedAndInputTests } from "../🧱️builder-copied-and-input/🟦️.tsx";
import { registerEvidenceBindingPayloadTests } from "../🔏️evidence-binding-payload/🟦️.tsx";

/** 📡 Registers the TypedWire describe block from its helpers and parts. */
export function registerTypedWireSuite(kit: ResidentTestKit): void {
  const { describe } = kit;
  describe("TypedWire", () => {
    const helpers = createResidentPageHelpers(kit);
    registerPoolSlotAndScopeTests(kit);
    registerCompositionAndPageTests(kit, helpers);
    registerPageLifecycleTests(kit, helpers);
    registerReaderBindingTests(kit);
    registerBuilderCopiedAndInputTests(kit);
    registerEvidenceBindingPayloadTests(kit);
  });
}
