/** ⏲️ Law: the funding of a build test law plan is its declared spans stretched by the host contention within the policy bounds (a `fixed` plan keeps them), the total being the sum, exactly as the independent Python model derives it from the declared plans. */
import { expect, test } from "bun:test";
import { fileURLToPath } from "node:url";
import { testPythonV1 } from "../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import { buildTestLawBudgetV1 } from "../../🟦️.ts";
import law from "../../🧬️schema/📐️law.json";

const ratios = [0, 0.5, 1, 1.3, 3, 7.9, 20];

test("stretched law plan funding equals the independent Python model for every plan and contention ratio", () => {
  const python = testPythonV1();
  if (!python) throw Error("Independent Python oracle required");
  const cases = Object.keys(law.properties.plans.const).flatMap(plan => ratios.map(ratio => ({ plan, ratio })));
  const result = Bun.spawnSync([python, "-I", fileURLToPath(new URL("./🐍️.py", import.meta.url)), fileURLToPath(new URL("../../🧬️schema/📐️law.json", import.meta.url)), fileURLToPath(new URL("../../../🎚️budget/⏲️contention/🔣️.json", import.meta.url))], { stdin: Buffer.from(JSON.stringify(cases)) });
  expect(result.stderr.toString()).toBe("");
  const model = JSON.parse(result.stdout.toString()) as unknown[];
  cases.forEach((row, index) => expect<unknown>(buildTestLawBudgetV1(row.plan as "sdk-fence", row.ratio), `${row.plan} x${row.ratio}`).toEqual(model[index]));
});

test("an idle host keeps the declared plan and a fixed plan never stretches", () => {
  expect(buildTestLawBudgetV1("sdk-fence", 1)).toEqual({ totalMilliseconds: 30000, bodyMilliseconds: 25000, drainMilliseconds: 5000 });
  expect(buildTestLawBudgetV1("receiver-proof", 8)).toEqual({ totalMilliseconds: 2000, bodyMilliseconds: 1000, drainMilliseconds: 1000 });
  expect(buildTestLawBudgetV1("sdk-fence", 3).totalMilliseconds).toBe(90000);
});
