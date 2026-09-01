import { expect, test } from "bun:test";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import ts from "typescript";

//#region 🧭️Inputs
const root = resolve(import.meta.dir, "../../../../../../../../");
const inventorySchemaPath = resolve(import.meta.dir, "../🛂️schema/🔣️inventory.json");
const vectorsPath = resolve(import.meta.dir, "🔣️.json");
const vectorsSchemaPath = resolve(import.meta.dir, "🧬️schema/🔣️.json");
const rootScriptPath = resolve(root, "📜️script.ts");
const inventorySchema = JSON.parse(readFileSync(inventorySchemaPath, "utf8"));
const vectorsSchema = JSON.parse(readFileSync(vectorsSchemaPath, "utf8"));
const vectors = JSON.parse(readFileSync(vectorsPath, "utf8")) as {
  readonly schemaVersion: 1;
  readonly roles: readonly string[];
  readonly validInventory: Record<string, unknown>;
  readonly unknownRole: string;
  readonly extraRosterField: string;
};

/** 🧭️ Extracts the current public source-roster role literals without executing inventory collection. */
function sourceRecordRoles(source: string): readonly string[] {
  const file = ts.createSourceFile(rootScriptPath, source, ts.ScriptTarget.Latest, true);
  const declaration = file.statements.find((statement): statement is ts.InterfaceDeclaration => ts.isInterfaceDeclaration(statement) && statement.name.text === "MutationTaxonomySourceRecord");
  if (!declaration) throw new Error("missing MutationTaxonomySourceRecord");
  const member = declaration.members.find((entry): entry is ts.PropertySignature => ts.isPropertySignature(entry) && entry.name.getText(file) === "role");
  if (!member?.type || !ts.isUnionTypeNode(member.type)) throw new Error("MutationTaxonomySourceRecord.role is not a literal union");
  return member.type.types.map((entry) => {
    if (!ts.isLiteralTypeNode(entry) || !ts.isStringLiteral(entry.literal)) throw new Error("MutationTaxonomySourceRecord.role includes a non-string literal");
    return entry.literal.text;
  }).sort((left, right) => Buffer.compare(Buffer.from(left), Buffer.from(right)));
}

/** 🧭️ Replaces one roster record without widening the actual output envelope. */
function rosterInventory(mutator: (record: Record<string, unknown>) => Record<string, unknown>): Record<string, unknown> {
  const roster = vectors.validInventory.sourceRoster;
  if (!Array.isArray(roster)) throw new Error("neutral source roster is absent");
  return { ...vectors.validInventory, sourceRoster: roster.map((record, index) => index === 0 ? mutator(record as Record<string, unknown>) : record) };
}
//#endregion 🧭️Inputs

//#region 🧪️Schema
test("mutation source roster roles vectors are closed", () => {
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(vectorsSchema);
  expect(validate(vectors), JSON.stringify(validate.errors)).toBe(true);
  expect(new Set(vectors.roles).size).toBe(4);
});

test("mutation source roster roles match the current public source record", () => {
  expect(sourceRecordRoles(readFileSync(rootScriptPath, "utf8"))).toEqual([...vectors.roles].sort((left, right) => Buffer.compare(Buffer.from(left), Buffer.from(right))));
});

test("mutation inventory v2 source roster accepts every public role and rejects unknown fields", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(inventorySchema);
  expect(validate(vectors.validInventory), JSON.stringify(validate.errors)).toBe(true);
  expect(validate(rosterInventory((record) => ({ ...record, role: vectors.unknownRole })))).toBe(false);
  expect(validate(rosterInventory((record) => ({ ...record, [vectors.extraRosterField]: true })))).toBe(false);
});
//#endregion 🧪️Schema
