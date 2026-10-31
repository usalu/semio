import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import ts from "typescript";
import * as parsers from "../../🟦️.ts";

const schema = JSON.parse(readFileSync(new URL("../../🔣️.json", import.meta.url), "utf8"));
const corpus = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8")) as { cases: { id: string; exported: string; value: unknown; valid: boolean }[] };
const ajv = new Ajv({ strict: false });
ajv.addSchema(schema);
test("every normative Pets export has its actual typed parser", () => {
  for (const name of Object.keys(schema.$defs)) expect(typeof (parsers as Record<string, unknown>)["parse" + name]).toBe("function");
});
test("actual projected Pets source type-checks strictly", () => {
  const program = ts.createProgram([fileURLToPath(new URL("../../🟦️.ts", import.meta.url))], { strict: true, noEmit: true, skipLibCheck: true, types: [], target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler });
  expect(ts.getPreEmitDiagnostics(program).map(row => ts.flattenDiagnosticMessageText(row.messageText, "\n"))).toEqual([]);
});
for (const row of corpus.cases) test("actual Pets parser agrees with Ajv: " + row.id, () => {
  const validate = ajv.compile({ $ref: schema.$id + "#/$defs/" + row.exported });
  expect(validate(row.value)).toBe(row.valid);
  const parse = (parsers as Record<string, unknown>)["parse" + row.exported] as (value: unknown) => unknown;
  let valid = true;
  try { parse(row.value); } catch { valid = false; }
  expect(valid).toBe(row.valid);
});
