/** 📬 Extension invocation WIT identity, icon export and host effect wire decoding, completion publication and command-ingress fault diagnostics. */
import { type PluginRuntimeVitest, type PluginRuntimeDependencies } from "../🛠️plugin-runtime-helpers/🟦️.tsx";
import { createInvocationWireTestKit } from "./🧰️invocation-wire-test-kit/🟦️.tsx";
import { registerIdentityAndWireDecodingTests } from "./🔖️identity-and-wire-decoding/🟦️.tsx";
import { registerCompletionPublicationSuite } from "./📬️completion-publication/🟦️.tsx";
import { registerCommandIngressFaultTests } from "./🩺️command-ingress-faults/🟦️.tsx";

export async function registerExtensionInvocationWireTests(vitest: PluginRuntimeVitest, dependencies: PluginRuntimeDependencies): Promise<void> {
  const kit = createInvocationWireTestKit(vitest, dependencies);
  registerIdentityAndWireDecodingTests(kit);
  registerCompletionPublicationSuite(kit);
  registerCommandIngressFaultTests(kit);
}
