/** 📬️ The extension invocation completion publication describe block: its wrapper, requester helper and parts. */
import type { InvocationWireTestKit } from "../🧰️invocation-wire-test-kit/🟦️.tsx";
import { createCompletionPublicationHelpers } from "../🛠️requester-helper/🟦️.tsx";
import { registerCompletionAndJobSliceTests } from "../✅️completion-and-job-slices/🟦️.tsx";
import { registerActorBindingAndDisposalTests } from "../🔗️actor-binding-and-disposal/🟦️.tsx";
import { registerLateCompletionAndActivationTests } from "../⏳️late-completion-and-activation/🟦️.tsx";

/** 📮 Registers the extension invocation completion publication describe block from its helpers and parts. */
export function registerCompletionPublicationSuite(kit: InvocationWireTestKit): void {
  const { describe } = kit;
  describe("extension invocation completion publication", () => {
    const helpers = createCompletionPublicationHelpers(kit);
    registerCompletionAndJobSliceTests(kit, helpers);
    registerActorBindingAndDisposalTests(kit, helpers);
    registerLateCompletionAndActivationTests(kit, helpers);
  });
}
