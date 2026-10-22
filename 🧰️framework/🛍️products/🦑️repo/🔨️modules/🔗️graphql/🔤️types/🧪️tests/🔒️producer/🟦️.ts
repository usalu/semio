import { expect, test } from "bun:test";
import Ajv from "ajv";
import ts from "typescript";
import { isBuiltin } from "node:module";
import { readFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { readSourceInputContract, relativeSourceInputs } from "../../../../📚️library/🕸️dependencies/🟦️typescript/🟨️.mjs";
import schema from "./🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

const root = resolve(import.meta.dir, "../../../../../../../..");

test("canonical GraphQL producer inputs exclude generated output without a type-import waiver", () => {
  const validate = new Ajv().compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  const contract = readSourceInputContract(resolve(root, fixture.contract));
  expect(contract.generated).toEqual([]);
  const actual = relativeSourceInputs(contract, root).files.map((file: string) => relative(root, file).replaceAll("\\", "/")).sort();
  const pending = contract.entries.map((entry: string) => resolve(root, entry)), seen = new Set<string>();
  while (pending.length) {
    const file = pending.pop()!;
    if (seen.has(file)) continue;
    seen.add(file);
    for (const row of ts.preProcessFile(readFileSync(file, "utf8"), true, true).importedFiles) {
      if (isBuiltin(row.fileName)) continue;
      expect(row.fileName.startsWith("."), file).toBe(true);
      pending.push(resolve(dirname(file), row.fileName));
    }
  }
  const includingTypes = [...seen].map(file => relative(root, file).replaceAll("\\", "/")).sort();
  expect(actual).toEqual([...fixture.expectedModules].sort());
  expect(includingTypes).toEqual(actual);
  for (const output of fixture.excludedGeneratedOutputs) {
    expect(actual).not.toContain(output);
    expect(includingTypes).not.toContain(output);
  }
});
