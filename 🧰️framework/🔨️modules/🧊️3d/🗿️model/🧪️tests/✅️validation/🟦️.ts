/** ✅️ Validation: every fixture mutation yields exactly its issue codes; documents the JSON schema (Ajv) rejects are the ones flagged; issues are bilingual with JSON-pointer paths. */
import { describe, expect, it } from "vitest";
import { validateModel } from "../../🟦️.ts";
import { fixture, validateSchema } from "../🧰️support/🟦️.ts";

type Op = { op: "set" | "remove"; path: string; value?: unknown };
const apply = (base: unknown, ops: Op[]): any => {
  const doc = structuredClone(base) as any;
  for (const { op, path, value } of ops) {
    const keys = path.split("/").slice(1);
    const last = keys.pop()!;
    const parent = keys.reduce((o: any, k) => o[k], doc);
    if (op === "set") parent[last] = value;
    else if (Array.isArray(parent)) parent.splice(Number(last), 1);
    else delete parent[last];
  }
  return doc;
};

const data = fixture("✅️validation");

describe("validateModel", () => {
  for (const c of data.cases) {
    it(c.name, () => {
      const doc = apply(data.base, c.ops);
      const issues = validateModel(doc);
      expect([...new Set(issues.map((i) => i.code))].sort()).toEqual([...c.expectCodes].sort());
      for (const i of issues) {
        expect(i.path.startsWith("/")).toBe(true);
        expect(i.message.en.length).toBeGreaterThan(0);
        expect(i.message.de.length).toBeGreaterThan(0);
      }
      expect(validateSchema(doc)).toBe(c.schemaValid);
    });
  }
});
