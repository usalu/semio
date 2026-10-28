/** 🔣️ Tests for context policy. */
import { expect, test } from "bun:test";
import { repositoryVitestPolicyV1 } from "../🟦️.ts";

/** 🧵️ Law: the Vitest worker bound is the plan of the one worker policy: the override variable of the policy (integer from 1 to the maximum of the runner), the memory and cores of the admitted share, or the host share; a count the caller already planned passes through, anything else is refused. */
test("the Vitest worker bound follows the worker policy and refuses what is not an integer from 1 to the maximum", () => {
  const job = { SEMIO_HOST_CAPACITY_PLAN: JSON.stringify({ version: 1, id: "k".repeat(22), jobs: 1, cpu: 1, memoryMegabytes: 2048, fixedMegabytes: 1536, floorMegabytes: 2048 }) };
  expect(repositoryVitestPolicyV1(import.meta.dir, {}).maxWorkers).toBe(4);
  expect(repositoryVitestPolicyV1(import.meta.dir, job).maxWorkers).toBeGreaterThanOrEqual(1);
  expect(repositoryVitestPolicyV1(import.meta.dir, job).maxWorkers).toBeLessThanOrEqual(4);
  for (const [text, value] of [["1", 1], ["2", 2], ["3", 3]] as const) expect(repositoryVitestPolicyV1(import.meta.dir, { SEMIO_VITEST_WORKERS: text }).maxWorkers).toBe(value);
  expect(repositoryVitestPolicyV1(import.meta.dir, { ...job, SEMIO_VITEST_WORKERS: "8" }).maxWorkers).toBe(4);
  for (const text of ["0", "9", "65", "1.5", "-1", "four", "", "NaN"]) expect(() => repositoryVitestPolicyV1(import.meta.dir, { SEMIO_VITEST_WORKERS: text }), text).toThrow("SEMIO_VITEST_WORKERS");
  expect(repositoryVitestPolicyV1(import.meta.dir, {}, 2).maxWorkers).toBe(2);
});
