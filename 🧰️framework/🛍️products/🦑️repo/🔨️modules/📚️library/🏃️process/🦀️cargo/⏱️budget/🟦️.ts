import {TEST_LEVEL_BUDGET_MS,activeTestLevel,type TestLevel} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
/** ⏱️ Declares native cohort assertion floors. */
export const PACKAGE_TEST_BUDGET_MS: Record<string, Partial<Record<TestLevel, number>>> = {
  "semio-hub-writer": { quick: 600_000 },
  "semio-hub-mathematical": { quick: 1_800_000 },
  "semio-hub-wfc": { quick: 1_800_000 },
  "semio-hub-procedural": { quick: 1_200_000 },
  "semio-hub-flow": { quick: 1_200_000 },
  "semio-hub-gis": { quick: 900_000 },
  "semio-hub-vcs": { quick: 600_000 },
  "semio-hub-animate": { quick: 900_000 },
  "semio-hub-shooting": { quick: 600_000 },
  "semio-hub-demonstrator": { quick: 1_200_000 },
  "semio-hub-sequence": { quick: 600_000 },
  "semio-hub-fem": { quick: 1_800_000 },
  "semio-hub-architect": { quick: 1_800_000 },
  "semio-hub-bim": { quick: 1_800_000 },
  "semio-hub-process": { quick: 1_200_000 },
  "semio-hub-lowpoly": { quick: 900_000 },
  "semio-hub-reasoning": { quick: 600_000 },
  "semio-hub-forms": { quick: 600_000 },
  "semio-hub-layout": { quick: 600_000 },
  "semio-hub-cad": { quick: 1_200_000 },
  "semio-hub-norm": { quick: 1_800_000 },
  "semio-hub-playbook": { quick: 600_000 },
  "semio-hub-imperative": { quick: 600_000 },
  "semio-hub-remodel": { quick: 600_000 },
  "semio-hub-energy": { quick: 600_000 },
  "semio-hub-trinity": { quick: 600_000 },
  "semio-hub-dag": { quick: 600_000 },
  "semio-hub-draw": { quick: 900_000 },
  "semio-hub-raster": { quick: 600_000 },
  "semio-s-plugin-stdio": { quick: 1_800_000 },
  "semio-hub-note": { quick: 600_000 },
  "semio-hub-puzzle": { quick: 1_200_000 },
  "semio-hub-block": { quick: 1_200_000 },
  "semio-hub-space": { quick: 1_200_000 },
  "semio-hub-sourcing": { quick: 600_000 },
  "semio-s-plugin-cad-aec-building": { quick: 600_000 },
  "semio-s-plugin-cad-aec-energy": { quick: 600_000 },
  "semio-s-plugin-cad-aec-structure": { quick: 600_000 },
  "semio-s-plugin-cad-spatial-shape": { quick: 600_000 },
  "semio-s-plugin-imperative-control": { quick: 600_000 },
  "semio-s-plugin-imperative-effect": { quick: 600_000 },
  "semio-s-plugin-imperative-logic": { quick: 600_000 },
  "semio-s-plugin-imperative-math": { quick: 600_000 },
  "semio-s-plugin-imperative-text": { quick: 600_000 },
  "semio-s-plugin-playbook-procedural": { quick: 600_000 },
  "semio-s-plugin-sourcing-beams": { quick: 600_000 },
  "semio-s-plugin-sourcing-slabs": { quick: 600_000 },
  "semio-s-plugin-sourcing-windows": { quick: 600_000 },
};
/** 🧪️ Intersects package floors with the caller-selected test level. */
export function packageTestBudgetMs(packages: readonly string[], level: TestLevel = activeTestLevel(), environment:Readonly<Record<string,string|undefined>>=process.env): number {
  if (environment.SEMIO_TEST_BUDGET_MS !== undefined) return Number(environment.SEMIO_TEST_BUDGET_MS);
  let budget = TEST_LEVEL_BUDGET_MS[level];
  for (const pkg of packages) {
    const floor = PACKAGE_TEST_BUDGET_MS[pkg]?.[level];
    if (floor !== undefined) budget = Math.max(budget, floor);
  }
  return budget;
}
