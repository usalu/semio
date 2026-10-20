/** 🪪️ Runs the language-neutral Node oracle of the retained surface host; the host package script runs this file instead of importing the oracle. */
import { expect, test } from "bun:test";
import { resolve } from "node:path";
import { surfaceHostRetentionOracle } from "../../🔮️oracles/🪪️surface-host-retention/🟦️.ts";

test("surface-host-retention-oracle", () => {
  const checks = surfaceHostRetentionOracle(resolve(import.meta.dir, "../../../../../../../.."));
  expect(checks).toBeGreaterThan(0);
  console.log(`surface-host-retention-oracle: checks=${checks} clean`);
});
