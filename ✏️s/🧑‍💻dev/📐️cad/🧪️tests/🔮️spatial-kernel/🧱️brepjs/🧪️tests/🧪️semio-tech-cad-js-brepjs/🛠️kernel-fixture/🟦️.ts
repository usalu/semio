/** 🛠️ The brepjs kernel fixture shared by the brepjs parts. */
import { BrepjsKernel } from "../../../📨️worker-and-kernel/🟦️.ts";
import type { BrepjsTestKit } from "../🧰️brepjs-test-kit/🟦️.ts";

/** 🔧 Creates the brepjs kernel fixture shared by the brepjs parts. */
export function createBrepjsKernelHelpers(kit: BrepjsTestKit) {
  const { BrepjsKernel } = kit;
  const kernel = new BrepjsKernel();
  return { kernel };
}
export type BrepjsKernelHelpers = ReturnType<typeof createBrepjsKernelHelpers>;
