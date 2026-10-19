import { createTreeWindowsTestKit } from "./🧰️tree-windows-test-kit/🟦️.tsx";
import type { TestSource } from "./🧰️tree-windows-test-kit/🟦️.tsx";
import { registerHistoryScrollLayoutTests } from "./📖️history-scroll-layout/🟦️.tsx";
import { registerInterpretedTreeWindowsSuite } from "./🪟️interpreted-tree-windows/🟦️.tsx";
import { registerNeutralViewportVectorTests } from "./📐️neutral-viewport-vectors/🟦️.tsx";
import { registerShortAnswerWindowTests } from "./🧠️short-answer-window/🟦️.tsx";

/** 🪟️ The interpreter's half of the virtualised panel tree (🎫️ 26/09/16
 * ARTIFACT-TREE-VIRTUALISED-STREAMING, 📓️design-virtualised-tree.md §6.2): a container's authored
 * `window`/`windowKey` reaching the `🌳️Tree` element unchanged, and a `granularity` row synthesising
 * the tree-level `interactionSelect` pick that replaces the per-row action maps. */
export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const kit = await createTreeWindowsTestKit(vitest, dependencies, source);
  await registerHistoryScrollLayoutTests(kit);
  registerInterpretedTreeWindowsSuite(kit);
  await registerNeutralViewportVectorTests(kit);
  registerShortAnswerWindowTests(kit);
}
