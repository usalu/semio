//#region 🔌️Adapters
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
//#endregion 🔌️Adapters

//#region 🧫️Fixtures
const moduleRoot = join(import.meta.dirname, "../../🧬️schema");
const sdl = readFileSync(join(moduleRoot, "🔗️.graphql"), "utf-8");
const document = JSON.parse(readFileSync(join(moduleRoot, "🔣️.json"), "utf-8")) as {
  $schema: string;
  $id: string;
  $ref: string;
  $defs: Record<string, { type?: string; enum?: string[]; additionalProperties?: boolean; required?: string[]; properties?: Record<string, Record<string, unknown>> }>;
};

type Definition = { readonly kind: string; readonly name: string; readonly fields: ReadonlyMap<string, boolean>; readonly values: readonly string[] };

/** 🔗️ Reads every `type`/`input`/`interface`/`enum` of the normative SDL with its field nullability. */
function sdlDefinitions(): Map<string, Definition> {
  const stripped = sdl
    .split("\n")
    .map((line) => line.replace(/^\s*#.*$/, ""))
    .join("\n");
  const definitions = new Map<string, Definition>();
  for (const match of stripped.matchAll(/^(type|input|interface|enum)\s+(\w+)(?:\s+implements\s+[\w\s&]+?)?\s*\{([^}]*)\}/gm)) {
    const [, kind, name, body] = match;
    const fields = new Map<string, boolean>();
    const values: string[] = [];
    for (const line of body.split("\n").map((entry) => entry.trim()).filter((entry) => entry.length > 0)) {
      if (kind === "enum") {
        values.push(line);
        continue;
      }
      const field = /^(\w+)(?:\([^)]*\))?\s*:\s*(.+)$/.exec(line);
      expect(field, `${name}: ${line}`).not.toBeNull();
      fields.set(field![1], !field![2].trim().endsWith("!"));
    }
    definitions.set(name, { kind, name, fields, values });
  }
  return definitions;
}

/** 🫙️ Reports whether a property shape admits an explicit null. */
function admitsNull(shape: Record<string, unknown>): boolean {
  if (Array.isArray(shape.type)) return (shape.type as string[]).includes("null");
  if (Array.isArray(shape.anyOf)) return (shape.anyOf as Record<string, unknown>[]).some((option) => option.type === "null");
  return false;
}

/** ⚖️ Third-party draft-07 oracle the JSON facet is compiled by. */
function oracle(exportId: string): (data: unknown) => boolean {
  const AjvConstructor = createRequire(import.meta.url)("ajv") as new (options: Record<string, unknown>) => { compile(schema: unknown): (data: unknown) => boolean };
  return new AjvConstructor({ strict: false }).compile({ ...document, $id: `${document.$id}#${exportId}`, $ref: `#/$defs/${exportId}` });
}

const definitions = sdlDefinitions();
//#endregion 🧫️Fixtures

//#region 🧪️Module
describe("repo client mcp graphql module", () => {
  it("declares the canonical draft-07 identity", () => {
    expect(document.$schema).toBe("http://json-schema.org/draft-07/schema#");
    expect(document.$id).toBe("https://semio.tech/schema/repo/client/mcp/schema.json");
    expect(document.$ref).toBe("#/$defs/Query");
  });

  it("carries no executable document in the schema slot", () => {
    expect(sdl).not.toMatch(/^\s*(query|mutation|fragment)\s+\w+/m);
  });

  it("publishes exactly the SDL definitions plus the DateTime scalar", () => {
    expect(Object.keys(document.$defs).sort()).toEqual([...definitions.keys(), "DateTime"].sort());
    expect(document.$defs.DateTime.type).toBe("string");
    expect(sdl).toMatch(/^scalar DateTime$/m);
  });

  it("compiles under an independent draft-07 validator", () => {
    expect(oracle("Query")({})).toBe(true);
    expect(oracle("Range")({ start: 1, end: 2 })).toBe(true);
    expect(oracle("Range")({ start: 1, end: null })).toBe(false);
    expect(oracle("Range")({ start: 1, end: 2, middle: 3 })).toBe(false);
    expect(oracle("BreachPriority")("HIGH")).toBe(true);
    expect(oracle("BreachPriority")("CRITICAL")).toBe(false);
  });

  it("requires exactly the non-null fields of every input", () => {
    for (const definition of definitions.values()) {
      if (definition.kind !== "input") continue;
      const required = [...definition.fields.entries()].filter(([, nullable]) => !nullable).map(([field]) => field);
      expect([...(document.$defs[definition.name].required ?? [])].sort(), definition.name).toEqual(required.sort());
    }
  });

  it("closes every declared type and opens every interface", () => {
    for (const definition of definitions.values()) {
      if (definition.kind === "enum") continue;
      expect(document.$defs[definition.name].additionalProperties, definition.name).toBe(definition.kind === "interface");
    }
  });
});
//#endregion 🧪️Module

//#region 🧪️Parity
describe("field parity between 🔗️.graphql and 🔣️.json", () => {
  it.each([...definitions.keys()].map((name) => [name] as const))("%s", (name) => {
    const definition = definitions.get(name)!;
    const shape = document.$defs[name];
    expect(shape, name).toBeDefined();
    if (definition.kind === "enum") {
      expect(shape.type).toBe("string");
      expect(shape.enum).toEqual(definition.values);
      return;
    }
    expect(Object.keys(shape.properties ?? {})).toEqual([...definition.fields.keys()]);
    for (const [field, nullable] of definition.fields) expect(admitsNull(shape.properties![field]), `${name}.${field}`).toBe(nullable);
  });
});
//#endregion 🧪️Parity
