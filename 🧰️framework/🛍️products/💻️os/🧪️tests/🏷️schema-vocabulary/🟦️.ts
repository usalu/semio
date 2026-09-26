/** 🏷️ The `x-semio-*` vendor annotation vocabulary (`🧪️tests/🧬️schema-oracle/🔣️.json`) against the schemas that use it: every
 * annotation any os or hub schema carries is declared (with a value its meta-schema admits), the shared strict oracle compiles
 * the schemas that failed before it existed (`📇️directory/🧬️schema`, `🌎️hub/🧬️schema/🤝️two-client-document-v1` carry
 * `x-semio-note`), and strictness survives: an undeclared keyword or an ill-typed annotation still refuses to compile. */
import { describe, expect, it } from "vitest";
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { SEMIO_SCHEMA_VENDOR_VOCABULARY_V1, semioSchemaAjvV1 } from "../🧬️schema-oracle/🟦️.ts";

const os = fileURLToPath(new URL("../../", import.meta.url));
const hub = fileURLToPath(new URL("../../../../../🌎️hub/", import.meta.url));
const PRUNED = new Set(["node_modules", "target", "dist", "🗑️generated", "🤖️generated", ".git"]);
const schemaFiles = (root: string): string[] =>
  readdirSync(root).flatMap((name) => {
    if (PRUNED.has(name)) return [];
    const path = join(root, name);
    return statSync(path).isDirectory() ? schemaFiles(path) : name === "🔣️.json" ? [path] : [];
  });
const annotations = (node: unknown, found: [string, unknown][] = []): [string, unknown][] => {
  if (Array.isArray(node)) for (const child of node) annotations(child, found);
  else if (node !== null && typeof node === "object")
    for (const [key, child] of Object.entries(node)) {
      if (key.startsWith("x-semio-")) found.push([key, child]);
      annotations(child, found);
    }
  return found;
};
const json = (path: string) => JSON.parse(readFileSync(path, "utf8"));

describe("🏷️ x-semio vendor annotation vocabulary", () => {
  it("declares every annotation an os or hub schema carries, with a value its meta-schema admits", () => {
    const ajv = semioSchemaAjvV1();
    const declared = SEMIO_SCHEMA_VENDOR_VOCABULARY_V1.keywords as Record<string, object>;
    const undeclared = new Set<string>();
    const illTyped: string[] = [];
    const vocabularyFile = fileURLToPath(new URL("../🧬️schema-oracle/🔣️.json", import.meta.url));
    for (const path of [...schemaFiles(os), ...schemaFiles(hub)].filter((candidate) => candidate !== vocabularyFile)) {
      let document: unknown;
      try {
        document = json(path);
      } catch {
        continue;
      }
      for (const [keyword, value] of annotations(document)) {
        if (!(keyword in declared)) undeclared.add(keyword);
        else if (!ajv.validate(declared[keyword]!, value)) illTyped.push(`${keyword} in ${path.slice(path.indexOf("🧰️framework") >= 0 ? path.indexOf("🧰️framework") : path.indexOf("🌎️hub"))}`);
      }
    }
    expect([...undeclared]).toEqual([]);
    expect(illTyped).toEqual([]);
  }, 120_000);

  it("compiles the schemas carrying x-semio-note in strict mode", () => {
    const directory = json(fileURLToPath(new URL("../../🔨️modules/📇️directory/🧬️schema/🔣️.json", import.meta.url)));
    const twoClient = json(join(hub, "🧬️schema/🤝️two-client-document-v1/🔣️.json"));
    expect(annotations(directory).some(([keyword]) => keyword === "x-semio-note")).toBe(true);
    expect(() => semioSchemaAjvV1({ allErrors: true }).addSchema(directory).addSchema(twoClient).compile({ $ref: directory.$id })).not.toThrow();
    expect(() => semioSchemaAjvV1().compile(twoClient)).not.toThrow();
  });

  it("keeps strict mode: an undeclared x-semio keyword and an ill-typed annotation both refuse to compile", () => {
    expect(() => semioSchemaAjvV1().compile({ type: "string", "x-semio-undeclared": "anything" })).toThrow(/unknown keyword/u);
    expect(() => semioSchemaAjvV1().compile({ type: "string", "x-semio-note": 7 })).toThrow();
    expect(semioSchemaAjvV1().compile({ type: "string", "x-semio-note": "an annotation only" })("value")).toBe(true);
  });
});
