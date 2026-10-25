import { expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import catalog from "../../🔣️catalog.json" with { type: "json" };
import dependencySchema from "../../../🧬️schema/📥️dependencies.json" with { type: "json" };
import { buildOperationRequiresV1 } from "../../🟦️.ts";
import { readBuildDependencyInvocationV1 } from "../../../🧭️invocation/🟦️.ts";
import fixture from "./🔣️.json" with { type: "json" };

const names = (requires: string): string[] => catalog.operations.filter((row) => (row as { requires?: string }).requires === requires).map((row) => row.name);

test("every operation declares what it needs, and the declaration is the one the CLI acts on", () => {
  expect(names("dependencies")).toEqual(fixture.requires.dependencies);
  expect(names("none")).toEqual(fixture.requires.none);
  for (const requires of ["dependencies", "none"] as const) for (const name of fixture.requires[requires]) expect(buildOperationRequiresV1(name), name).toBe(requires);
  expect(buildOperationRequiresV1("unknown-operation")).toBe(fixture.unknownOperation as "none" | "dependencies");
});

test("the build CLI derives its dependency provisioning from the declaration and keeps no list of its own", () => {
  const source = readFileSync(join(import.meta.dir, "../../../⌨️command/⚙️runtime/🟦️.ts"), "utf8");
  expect(source).toContain(fixture.repoScript.ensureGateText);
  expect(source).not.toContain(fixture.repoScript.retiredText);
});

test("dependencies ensure is the whole-checkout provisioning request and refuses selections", () => {
  const oracle = new Ajv({ strict: true }).compile(dependencySchema);
  for (const row of fixture.ensure.accepted) {
    const selected = readBuildDependencyInvocationV1(row.args);
    expect(selected).toEqual(row.value as typeof selected);
    expect(oracle(selected)).toBe(true);
  }
  for (const args of fixture.ensure.refused) expect(() => readBuildDependencyInvocationV1(args), args.join(" ")).toThrow("Expected repo dependencies sync|lock --all|--scope <owner>");
});
