/** 🏛️ Verifies higher snapshot composition after neutral decode law ownership moves downward. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import contract from "./🧫️fixtures/🔣️.json";
import schema from "./🧬️schema/🔣️.json";

test("higher assembly ownership has a closed independent contract", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(validate(contract)).toBe(true);
  for (const value of [{ ...contract, unknown: true }, { ...contract, retainedModules: [] }]) expect(validate(value)).toBe(false);
});

test("higher snapshot assembly retains product scenarios without mounting the neutral control laws", () => {
  const source = readFileSync(resolve(import.meta.dir, "../🦀️.rs"), "utf8");
  expect(source).not.toContain(contract.retiredMount);
  for (const module of contract.retainedModules) expect(source).toContain(`mod ${module};`);
});

import "../../../../🧪️tests/💰️allocation/🟦️.ts";
