import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

/** ♻️ Independent JSON-tree accounting for exact retained document release receipts. */
export function testRewritingDocumentRetirementOracle(): void {
  const fixture = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv({ strict: true }).compile({ type: "object", required: ["schema", "budgets", "snapshots", "mutations"], properties: { schema: { const: "semio.rewriting.document-retirement/v1" }, budgets: { type: "array", minItems: 2 }, snapshots: { type: "array", minItems: 2 }, mutations: { type: "array", minItems: 7, maxItems: 7 } }, additionalProperties: false });
  assert(validate(fixture), JSON.stringify(validate.errors));
  const text = (value: string): number => Buffer.byteLength(value, "utf8");
  const property = (value: unknown): number => value === null ? 0 : typeof value === "string" ? text(value) : typeof value === "number" ? 8 : typeof value === "boolean" ? 1 : Array.isArray(value) ? value.reduce((sum, item) => sum + property(item), 0) : Object.entries(value as Record<string, unknown>).reduce((sum, [key, item]) => sum + text(key) + property(item), 0);
  for (const row of fixture.snapshots) {
    const state = row.value;
    const bytes = text(state.beforeFixtureJson) + text(state.lhsJson) + text(state.rhsJson) + property(state.parameterBindings) + Object.keys(state.ruleLayout).reduce((sum, key) => sum + text(key) + 16, 0);
    assert.equal(bytes, row.bytes);
  }
  for (const row of fixture.mutations) {
    const value = row.value;
    const bytes = value.newBeforeFixtureJson !== undefined ? text(value.newBeforeFixtureJson) : value.newLhsJson !== undefined ? text(value.newLhsJson) : value.newRhsJson !== undefined ? text(value.newRhsJson) : text(value.key) + (value.newValue !== undefined ? property(value.newValue) : value.newPoint !== undefined ? 16 : 0);
    assert.equal(bytes, row.bytes);
  }
  console.log("[DEBUG] Rewriting retirement oracle: two document roots and seven mutation payloads match independent JSON-tree byte accounting");
}
