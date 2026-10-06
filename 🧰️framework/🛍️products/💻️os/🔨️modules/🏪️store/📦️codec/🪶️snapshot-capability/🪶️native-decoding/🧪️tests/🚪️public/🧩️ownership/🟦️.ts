/** 🏛️ Verifies higher snapshot composition after neutral decode law ownership moves downward. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import contract from "./🧫️fixtures/🔣️.json";



test("higher snapshot assembly retains product scenarios without mounting the neutral control laws", () => {
  const source = readFileSync(resolve(import.meta.dir, "../🦀️.rs"), "utf8");
  expect(source).not.toContain(contract.retiredMount);
  for (const module of contract.retainedModules) expect(source).toContain(`mod ${module};`);
});

import "../../../../🧪️tests/💰️allocation/🟦️.ts";
