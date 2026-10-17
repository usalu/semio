/**
 * 🗂️ Collection-catalog generator: catalog admission, generated-output freshness, and the generic register-row payload
 * schemas checked by Ajv (the third-party JSON Schema oracle) against every committed register-row fixture.
 * @see ../../🏗️authoring/🗂️collection-catalog/🟦️.ts
 */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { generateCollectionCatalogV1, parseCollectionCatalogV1, renderCollectionCatalogRustV1 } from "../../🏗️authoring/🗂️collection-catalog/🟦️.ts";

const REPO = join(import.meta.dir, "../../../../../../..");
const ANY = join(REPO, "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any");
const SNAPSHOT = join(ANY, "🧬️schema/📸️snapshot/🔣️.json");
const OPS = ["🌱️create", "🗑️delete", "🏷️rename", "♻️replace"] as const;

const json = (path: string): Record<string, unknown> => JSON.parse(readFileSync(path, "utf8"));

/** 🗂️ Every schema document the generated mutation schema index knows, by \`$id\`, so Ajv resolves references on demand. */
const INDEX: Record<string, string> = Object.assign({}, ...Object.values((json(join(REPO, "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json")).roots as Record<string, Record<string, string>>)));

/** 🧪️ Ajv validators for the four leaf payload schemas, their references loaded from the schema index. */
async function oracle(): Promise<Record<(typeof OPS)[number], (value: unknown) => boolean>> {
  const ajv = new Ajv({ strict: false, allErrors: true, loadSchema: async (uri) => json(join(REPO, INDEX[uri.split("#")[0]!] ?? uri)) });
  const validators = {} as Record<(typeof OPS)[number], (value: unknown) => boolean>;
  for (const op of OPS) validators[op] = await ajv.compileAsync(json(join(ANY, "🧬️schema/🧬️mutations/📋️register-row", op, "🧬️schema/🔣️.json")));
  return validators;
}

const LEAVES = oracle();

describe("collection catalog", () => {
  test("admits the architect snapshot catalog and refuses a key enum out of order", () => {
    const document = json(SNAPSHOT);
    expect(parseCollectionCatalogV1(document).collections).toHaveLength(64);
    const defs = document.$defs as Record<string, { enum: string[] }>;
    const reordered = { ...document, $defs: { ...defs, RegisterKey: { ...defs.RegisterKey, enum: [...defs.RegisterKey.enum].reverse() } } };
    expect(() => parseCollectionCatalogV1(reordered)).toThrow("RegisterKey enum must list the catalog keys in order");
  });

  test("keeps the generated Rust and TypeScript catalog fresh", () => {
    expect(generateCollectionCatalogV1(SNAPSHOT, "check")).toEqual([]);
  });

  test("cuts the generated Rust catalog into a root that mounts cohesive children, one first emoji each, with one conversion per collection", () => {
    const files = renderCollectionCatalogRustV1(parseCollectionCatalogV1(json(SNAPSHOT)));
    expect(files.map((file) => file.folder)).toEqual([null, "🪪️key-descriptors", "🔎️key-base-access", "🧾️row-operations", "🔁️row-conversions"]);
    for (const file of files) expect(file.text.split("\n").length).toBeLessThan(1000);
    for (const file of files.slice(1)) expect(files[0]!.text).toContain(`#[path = "${file.folder}/🦀️.rs"]`);
    expect(new Set(files.map((file) => file.text.split(" ")[1])).size).toBe(files.length);
    expect(files.map((file) => file.text).join("\n").match(/^impl From<\w+> for RegisterRow \{$/gmu)).toHaveLength(64);
  });

  test("Ajv accepts every committed register-row payload against its leaf schema", async () => {
    const leaf = await LEAVES;
    const failures: string[] = [];
    let cases = 0;
    for (const op of OPS) {
      const directory = join(ANY, "🧪️tests/🧫️fixtures/🧬️mutations/📋️register-row", op);
      for (const name of readdirSync(directory)) {
        cases += 1;
        const validate = leaf[op] as ((value: unknown) => boolean) & { errors?: unknown };
        if (!validate(json(join(directory, name, "🦠️mutation/🔣️.json")))) failures.push(`${op}/${name}: ${JSON.stringify(validate.errors).slice(0, 300)}`);
      }
    }
    expect(cases).toBe(326);
    expect(failures).toEqual([]);
  }, 120_000);

  test("Ajv refuses a row tagged with another register and a register outside the catalog", async () => {
    const leaf = await LEAVES;
    const directory = join(ANY, "🧪️tests/🧫️fixtures/🧬️mutations/📋️register-row");
    const create = json(join(directory, "🌱️create/👥️stakeholder-creates-a/🦠️mutation/🔣️.json")) as { row: { register: string } };
    expect(leaf["🌱️create"]({ ...create, row: { ...create.row, register: "risk" } })).toBe(false);
    const remove = json(join(directory, "🗑️delete/👥️stakeholder-deletes-a/🦠️mutation/🔣️.json"));
    expect(leaf["🗑️delete"]({ ...remove, register: "stakeholders" })).toBe(false);
  }, 120_000);
});
