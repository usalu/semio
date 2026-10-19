/** 🪟️ The interpreted tree windows describe block: its wrapper and parts. */
import type { TreeWindowsTestKit } from "../🧰️tree-windows-test-kit/🟦️.tsx";
import { registerWindowDataAndGranularityPickTests } from "../🧬️window-data-and-granularity-picks/🟦️.tsx";
import { registerScrollContainerResolutionTests } from "../📜️scroll-container-resolution/🟦️.tsx";
import { registerWindowRequestsAndNestingTests } from "../🪜️window-requests-and-nesting/🟦️.tsx";
import { registerFoldSeedingIdsAndDuplicateTests } from "../🧱️fold-seeding-ids-and-duplicates/🟦️.tsx";

/** 🌲 Registers the interpreted tree windows describe block from its parts. */
export function registerInterpretedTreeWindowsSuite(kit: TreeWindowsTestKit): void {
  const { describe } = kit;
  describe("🪟️ interpreted tree windows", () => {
    registerWindowDataAndGranularityPickTests(kit);
    registerScrollContainerResolutionTests(kit);
    registerWindowRequestsAndNestingTests(kit);
    registerFoldSeedingIdsAndDuplicateTests(kit);
  });
}
