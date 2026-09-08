//#region 🔌️Adapters
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
//#endregion 🔌️Adapters

//#region 🧫️Fixtures
const serverRoot = join(import.meta.dirname, "../../..");
const sql = readFileSync(join(serverRoot, "🧬️schema/🐘️postgres/🗄️.sql"), "utf-8");
const document = JSON.parse(readFileSync(join(serverRoot, "🧬️schema/🔣️.json"), "utf-8")) as {
  $schema: string;
  $id: string;
  $defs: Record<string, { type?: string; required?: string[]; properties?: Record<string, Record<string, unknown>> }>;
};

/** 🚧️ Leading keywords of a table constraint clause, anchored on a word boundary so a column named `checkpoint`, `uniqueness` or `constraints` is not read as a constraint. */
const TABLE_CONSTRAINT = /^(?:PRIMARY KEY|UNIQUE|FOREIGN KEY|CHECK|CONSTRAINT|EXCLUDE)\b/i;

/** ✂️ Splits a `CREATE TABLE` body on its top-level commas. */
function splitClauses(body: string): string[] {
  const clauses: string[] = [];
  let depth = 0;
  let current = "";
  for (const character of body) {
    if (character === "(") depth++;
    if (character === ")") depth--;
    if (character === "," && depth === 0) {
      clauses.push(current);
      current = "";
      continue;
    }
    current += character;
  }
  clauses.push(current);
  return clauses.map((clause) => clause.trim()).filter((clause) => clause.length > 0);
}

/** 🐘️ Reads every table of the native PostgreSQL implementation with its column nullability. */
function sqlTables(): Map<string, Map<string, boolean>> {
  const stripped = sql
    .split("\n")
    .map((line) => line.replace(/--.*$/, ""))
    .join("\n");
  const tables = new Map<string, Map<string, boolean>>();
  for (const match of stripped.matchAll(/CREATE TABLE IF NOT EXISTS\s+(\w+)\s*\(([\s\S]*?)\n\);/g)) {
    const columns = new Map<string, boolean>();
    for (const clause of splitClauses(match[2])) {
      if (TABLE_CONSTRAINT.test(clause)) continue;
      const column = /^(\w+)\s+(.*)$/s.exec(clause);
      expect(column, clause).not.toBeNull();
      const rest = column![2].replace(/\s+/g, " ");
      columns.set(column![1], !/\bNOT NULL\b/i.test(rest) && !/\bPRIMARY KEY\b/i.test(rest));
    }
    tables.set(match[1], columns);
  }
  return tables;
}

function exportId(table: string): string {
  return `${table.split("_").map((part) => part.charAt(0).toUpperCase() + part.slice(1)).join("")}Row`;
}

/** 🫙️ Reports whether a property shape admits an explicit null. */
function admitsNull(shape: Record<string, unknown>): boolean {
  if (Array.isArray(shape.type)) return (shape.type as string[]).includes("null");
  if (Array.isArray(shape.anyOf)) return (shape.anyOf as Record<string, unknown>[]).some((option) => option.type === "null");
  return false;
}

const tables = sqlTables();
//#endregion 🧫️Fixtures

//#region 🧪️Module
describe("repo server persistence module", () => {
  it("declares the canonical draft-07 identity", () => {
    expect(document.$schema).toBe("http://json-schema.org/draft-07/schema#");
    expect(document.$id).toBe("https://semio.tech/schema/repo/server/schema.json");
  });

  it("compiles under an independent draft-07 validator", () => {
    const AjvConstructor = createRequire(import.meta.url)("ajv") as new (options: Record<string, unknown>) => { compile(schema: unknown): (data: unknown) => boolean };
    const validate = new AjvConstructor({ strict: false }).compile({ $ref: `${document.$id}#/$defs/TicketsRow`, definitions: {}, ...document });
    expect(validate({ id: "2026/09/08/t", status: "open", title: "T", prompt: "", summary: "", llm: "", client: "", author: "", github_issue: "", goal: "", parent: null, created_at: "2026-09-08T10:00:00Z", closed_at: null })).toBe(true);
    expect(validate({ id: "2026/09/08/t", status: "archived", title: "T", prompt: "", summary: "", llm: "", client: "", author: "", github_issue: "", goal: "", parent: null, created_at: "2026-09-08T10:00:00Z", closed_at: null })).toBe(false);
  });

  it("exports exactly one row contract per native table", () => {
    expect(tables.size).toBeGreaterThan(0);
    const expected = [...tables.keys()].map(exportId).sort();
    const actual = Object.keys(document.$defs).filter((name) => name.endsWith("Row")).sort();
    expect(actual).toEqual(expected);
  });
});
//#endregion 🧪️Module

//#region 🧪️Parity
describe("column parity between 🐘️postgres/🗄️.sql and 🔣️.json", () => {
  it.each([...tables.keys()].map((table) => [table] as const))("%s", (table) => {
    const columns = tables.get(table)!;
    const shape = document.$defs[exportId(table)];
    expect(shape, exportId(table)).toBeDefined();
    expect(Object.keys(shape.properties ?? {}).sort()).toEqual([...columns.keys()].sort());
    expect([...(shape.required ?? [])].sort()).toEqual([...columns.keys()].sort());
    for (const [column, nullable] of columns) expect(admitsNull(shape.properties![column]), `${table}.${column}`).toBe(nullable);
  });
});
//#endregion 🧪️Parity
