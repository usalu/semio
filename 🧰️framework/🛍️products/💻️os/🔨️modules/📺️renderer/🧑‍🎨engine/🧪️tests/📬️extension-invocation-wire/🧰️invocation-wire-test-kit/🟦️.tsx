/** 🧰️ The shared extension invocation wire test kit: the destructured vitest, the plugin runtime support and the fault encoder. */
import { type PluginRuntimeVitest, type PluginRuntimeDependencies } from "../../🛠️plugin-runtime-helpers/🟦️.tsx";

/** 🏭 Creates the shared invocation wire test kit of one registration from the injected dependencies. */
export function createInvocationWireTestKit(vitest: PluginRuntimeVitest, dependencies: PluginRuntimeDependencies) {
  const { describe, expect, it } = vitest;
  const { testState, assertAddressedInvocation, commandIngressFaultDisplay, decodePackValue, decodePackWire, DEFAULT_SHARD_BUDGET, encodePackValue, loadPluginModule, retainedWindowByActor, SemioFaultError, serializeCommandIngressForActor, wireEffectToFriendly, wireExtensionInvocation, yieldPluginUiContinuation } = dependencies;
  return { DEFAULT_SHARD_BUDGET, SemioFaultError, assertAddressedInvocation, commandIngressFaultDisplay, decodePackValue, decodePackWire, dependencies, describe, encodePackValue, expect, it, loadPluginModule, retainedWindowByActor, serializeCommandIngressForActor, testState, wireEffectToFriendly, wireExtensionInvocation, yieldPluginUiContinuation };
}
export type InvocationWireTestKit = ReturnType<typeof createInvocationWireTestKit>;
