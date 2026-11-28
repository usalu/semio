import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";

const driver = readFileSync(new URL("../🟦️.ts", import.meta.url), "utf8");
const body = (name: string): string => {
  const start = driver.indexOf(`export async function ${name}`);
  expect(start).toBeGreaterThan(0);
  const next = driver.indexOf("\n/** ", start);
  return driver.slice(start, next < 0 ? undefined : next);
};

test("every component build entry prepares declared crate preparations before the driver admits sources", () => {
  for (const name of ["buildComponentClosureV1", "describeComponentsV1"]) {
    const entry = body(name), prepare = entry.indexOf("await prepareComponentOwnersV1("), run = entry.indexOf("runRepositoryComponentDriverV1(");
    expect(prepare).toBeGreaterThan(0);
    expect(prepare).toBeLessThan(run);
  }
});

test("component owner preparation reuses the Cargo owner preparation and names no generator", () => {
  const helper = driver.split("async function prepareComponentOwnersV1")[1]!.split("\n}\n")[0]!;
  expect(helper).toContain("prepareCargoWorkspaceInvocation(");
  expect(helper).toContain("componentOwnerPreparationArgumentsV1(pass)");
  expect(driver).not.toContain("prepareCargoOwners?");
  expect(driver).not.toMatch(/graph-generate|composition-prepare|📜️script\.ts"?,\s*\["composition/u);
});
