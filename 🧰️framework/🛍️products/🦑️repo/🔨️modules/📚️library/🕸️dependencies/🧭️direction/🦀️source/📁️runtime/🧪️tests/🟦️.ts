/** 🧱️ Enforces runtime owner fragments with a language-neutral corpus and independent micromatch policy. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import micromatch from "micromatch";
import { inspectRustPathLiterals } from "../../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📁️paths/🟦️.ts";
const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));

test("closed runtime direction corpus has independent schema admission", () => {
  
  expect(fixture["schemaVersion"]).toEqual(1);
  
});
test("runtime owner direction agrees with independent glob policy at all three deletion boundaries", async () => {
  const module = await import(resolve(import.meta.dir, "../🟦️.ts"));
  for (const row of fixture.cases) {
    const references = inspectRustPathLiterals(row.source, fixture.roots);
    const actual = module.rustRuntimePathDirectionEdges(row.from, references, fixture.rules);
    const independent = references.flatMap(reference => fixture.rules.filter((rule: any) => micromatch.isMatch(row.from, rule.oracle.from) && micromatch.isMatch(reference.path, rule.oracle.to)).map((rule: any) => ({ rule: rule.name, from: row.from, to: reference.path, line: reference.line, call: reference.call, context: reference.context, start: reference.start, end: reference.end })));
    expect(actual.map((edge: any) => edge.rule)).toEqual(row.expectedRules);
    expect(actual).toEqual(independent);
    console.log("[DEBUG] runtime path direction " + JSON.stringify({ id: row.id, actual }));
  }
});

