/** 🧱️ The cad-js brepjs describe block: its wrapper, kernel fixture and parts. */
import type { BrepjsTestKit } from "../🧰️brepjs-test-kit/🟦️.ts";
import { createBrepjsKernelHelpers } from "../🛠️kernel-fixture/🟦️.ts";
import { registerBoxAndExtrudeTests } from "../📦️box-and-extrude/🟦️.ts";
import { registerTessellationCacheTests } from "../🕸️tessellation-cache/🟦️.ts";
import { registerTopologyAndMeasureTests } from "../📐️topology-and-measures/🟦️.ts";
import { registerCurveCommandTests } from "../〰️curve-commands/🟦️.ts";

/** 🏛 Registers the cad-js brepjs describe block from its kernel fixture and parts. */
export function registerBrepjsSuite(kit: BrepjsTestKit): void {
  const { describe } = kit;
  describe("@semio-tech/cad-js/brepjs", () => {
    const helpers = createBrepjsKernelHelpers(kit);
    registerBoxAndExtrudeTests(kit, helpers);
    registerTessellationCacheTests(kit, helpers);
    registerTopologyAndMeasureTests(kit, helpers);
    registerCurveCommandTests(kit, helpers);
  });
}
