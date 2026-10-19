/** 🎰️ The statechart machines of the framework in-source suites: the fixture machines, the test host, the trace inspector, the kernel, persistence, the actor system, model testing, bit sets and the checkout DSL twin. */
import { createMachineTestKit } from "./🧰️machine-test-kit/🟦️.ts";
import { registerStatechartKernelAndModelSuites } from "./⚙️statechart-kernel-and-model-suites/🟦️.ts";
import { registerCheckoutDslTwinTests } from "./🛒️checkout-dsl-twin/🟦️.ts";

export async function registerMachineTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: typeof import("@semio-tech/framework")): Promise<void> {
  const kit = createMachineTestKit(vitest, dependencies);
  //#region 🔄️MachineFixtures
  //#endregion 🔄️MachineFixtures
  registerStatechartKernelAndModelSuites(kit);
  registerCheckoutDslTwinTests(kit);
}
