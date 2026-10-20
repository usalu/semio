/** 🖱️ Runs the language-neutral Node oracles of the World3d gestures and selection; the host package script runs this file instead of importing the oracles. */
import { expect, test } from "bun:test";
import { resolve } from "node:path";
import { interactionSelectionSetOracle, selectionMergeVocabularyOracle, world3dPointerGestureOracle } from "../../🔮️oracles/🖱️world3d-gestures-and-selection/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../../..");

for (const [name, oracle] of [["world3d-pointer-gesture-oracle", world3dPointerGestureOracle], ["interaction-selection-set-oracle", interactionSelectionSetOracle], ["selection-merge-vocabulary-oracle", selectionMergeVocabularyOracle]] as const) {
  test(name, () => {
    const checks = oracle(repoRoot);
    expect(checks).toBeGreaterThan(0);
    console.log(`${name}: checks=${checks} clean`);
  });
}
