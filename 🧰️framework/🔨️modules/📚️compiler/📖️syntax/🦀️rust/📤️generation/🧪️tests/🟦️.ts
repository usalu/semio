import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import { inspectRustCompileReferences, type RustGeneratedTokenOutput } from "../../🟦️.ts";

const owner = resolve(import.meta.dir, ".."), fixture = JSON.parse(readFileSync(resolve(owner, "🧫️fixtures/🔣️.json"), "utf8"));
const outputContract = JSON.parse(readFileSync(resolve(owner, "🔣️.json"), "utf8"));

test("quote token output retains every generated input and every outside authored include", () => {
  for (const row of fixture.cases) {
    const outputs: RustGeneratedTokenOutput[] = [];
    const references = inspectRustCompileReferences(row.source, output => outputs.push(output));
    expect(outputs).toHaveLength(1);
    expect(new Ajv({ strict: true }).compile(outputContract)(outputs[0])).toBe(true);
    expect(outputs[0]!.macro).toBe(row.macro);
    expect(outputs.flatMap(output => output.inputs.map(input => input.expression))).toEqual(row.expressions);
    expect(references.map(reference => reference.path)).toEqual(row.authored ? [row.authored] : []);
    expect(row.source.slice(outputs[0]!.start, outputs[0]!.end)).toContain(row.macro + "!");
    expect(() => inspectRustCompileReferences(row.source)).toThrow("explicit generated-input observer");
    console.error("[DEBUG] owned generated token input " + row.id);
  }
});

test("shadowed and unresolved generator origins fail closed", () => {
  for (const row of fixture.unsupported) expect(() => inspectRustCompileReferences(row.source, () => {}), row.id).toThrow("Unsupported Rust compile");
});

test("a finite local macro named quote keeps its actual authored inputs", () => {
  for (const row of fixture.authoredMacros) {
    const outputs: RustGeneratedTokenOutput[] = [];
    expect(inspectRustCompileReferences(row.source, output => outputs.push(output)).map(reference => reference.path)).toEqual(row.paths);
    expect(outputs).toEqual([]);
  }
});
