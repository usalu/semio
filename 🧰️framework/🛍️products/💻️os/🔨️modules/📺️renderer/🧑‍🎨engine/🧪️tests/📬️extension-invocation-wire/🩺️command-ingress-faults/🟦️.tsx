/** 🩺️ The command ingress fault diagnostics tests. */
import { createPluginRuntimeSupport } from "../../🛠️plugin-runtime-helpers/🟦️.tsx";
import type { InvocationWireTestKit } from "../🧰️invocation-wire-test-kit/🟦️.tsx";

/** 🚨 Registers the command ingress fault diagnostics tests. */
export function registerCommandIngressFaultTests(kit: InvocationWireTestKit): void {
  const { commandIngressFaultDisplay, dependencies, describe, expect, it } = kit;
  const { encodeFaultBytes } = createPluginRuntimeSupport(dependencies);
    describe("command-ingress fault diagnostics", () => {
      it("decodes the normalized scalar-wire fault payload instead of hiding the terminal cause", () => {
        const bytes = encodeFaultBytes("plugin.command-rejected");
        expect(commandIngressFaultDisplay({ tag: "fault", val: { cursor: {}, fault: { tag: "fault", val: Array.from(bytes) } } })).toBe("plugin.command-rejected: plugin.command-rejected");
      });
    });
}
