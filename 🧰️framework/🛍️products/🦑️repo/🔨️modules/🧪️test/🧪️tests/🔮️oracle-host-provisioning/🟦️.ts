import { describe, expect, test } from "bun:test";
import { oracleHostPackagesFor } from "../../📦️packages/🟦️typescript/🤝️contributions/🟦️.ts";
import { pythonRequirement, pythonVersionSatisfies } from "../../🖥️host/🏗️materialization/🟦️.ts";
import vectors from "../🧫️fixtures/🔮️oracle-host-provisioning/🔣️.json";

const registry = (contributions: readonly unknown[]) => ({ contributions }) as unknown as Parameters<typeof oracleHostPackagesFor>[0];

describe("🔮️ oracle host provisioning is derived from the registered oracles", () => {
  for (const vector of vectors.derivation)
    test(vector.name, () => {
      const filled = vector.contributions.map((entry) => ({ oracleHostPackages: [], ...entry }));
      expect(oracleHostPackagesFor(registry(filled), vector.owner, vector.implementation as "python" | "typescript")).toEqual(vector.expected as never);
    });

  test("a pip requirement keeps a range a range and pins a bare release exactly", () => {
    for (const row of vectors.requirements) expect(pythonRequirement({ package: row.package, version: (row as { version?: string }).version }), row.expected).toBe(row.expected);
  });

  test("an installed release is judged against the declared pin", () => {
    for (const row of vectors.satisfies) expect(pythonVersionSatisfies(row.actual, (row as { pin?: string }).pin), `${row.actual} ${(row as { pin?: string }).pin ?? ""}`).toBe(row.expected);
  });
});
