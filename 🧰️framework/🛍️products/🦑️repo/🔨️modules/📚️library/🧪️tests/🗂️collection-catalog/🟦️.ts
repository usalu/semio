/**
 * 📚️ Collection-catalog generator: catalog admission, generated-output freshness, and the generic register-row payload
 * schemas checked by Ajv (the third-party JSON Schema oracle) against every committed register-row fixture.
 * The generated TypeScript catalog imports each row type from the module that declares it and never through the entry that
 * imports the catalog (an import cycle); the TypeScript compiler API is the independent oracle for declarations and cycles.
 * @see ../../🏗️authoring/🗂️collection-catalog/🟦️.ts
 */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import ts from "typescript";
import { generateCollectionCatalogV1, parseCollectionCatalogV1, renderCollectionCatalogRustV1, renderCollectionCatalogTypeScriptV1, resolveCollectionRowOwnersV1, type CollectionCatalogV1 } from "../../🏗️authoring/🗂️collection-catalog/🟦️.ts";

const REPO = join(import.meta.dir, "../../../../../../..");
const ANY = join(REPO, "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any");
const SNAPSHOT = join(ANY, "🧬️schema/📸️snapshot/🔣️.json");
const CATALOG = join(ANY, "🧬️schema/🗄️registers/🗂️catalog/🟦️.ts");
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

const read = (path: string): string | undefined => {
  try {
    return readFileSync(path, "utf8");
  } catch {
    return undefined;
  }
};

/** 🔬️ The relative modules a TypeScript file imports or re-exports (type-only ones included) and the names it declares at its top level, read with the TypeScript compiler API. */
function moduleFacts(path: string): { readonly modules: string[]; readonly declared: Set<string> } {
  const source = ts.createSourceFile(path, read(path) ?? "", ts.ScriptTarget.ESNext);
  const modules: string[] = [];
  const declared = new Set<string>();
  for (const statement of source.statements) {
    if ((ts.isImportDeclaration(statement) || ts.isExportDeclaration(statement)) && statement.moduleSpecifier !== undefined && ts.isStringLiteral(statement.moduleSpecifier) && statement.moduleSpecifier.text.startsWith(".")) modules.push(resolve(dirname(path), statement.moduleSpecifier.text));
    if ((ts.isInterfaceDeclaration(statement) || ts.isTypeAliasDeclaration(statement)) && statement.name !== undefined) declared.add(statement.name.text);
  }
  return { modules, declared };
}

const closure = (entry: string): Set<string> => {
  const seen = new Set<string>();
  for (let stack = [entry], path = stack.pop(); path !== undefined; path = stack.pop()) {
    if (seen.has(path)) continue;
    seen.add(path);
    if (read(path) !== undefined) stack.push(...moduleFacts(path).modules);
  }
  return seen;
};

const catalogOf = (rows: readonly string[], entry: string): CollectionCatalogV1 => ({
  keyType: "DemoKey", rowType: "DemoRow", labelType: "DemoLabel", tag: "demo", content: "row", headerField: "header",
  rust: { snapshot: "crate::a::S", diff: "crate::a::D", deltas: "crate::a", rows: "crate::a", header: "crate::a::H", id: "crate::a::I", canonicalJsonOwner: "owner" },
  typescript: { rows: entry }, output: { rust: "x.rs", typescript: "x.ts" },
  collections: rows.map((row) => ({ key: row.toLowerCase(), variant: row, entity: row.toLowerCase(), row, patch: `${row}Patch`, delta: `${row}Delta`, rowsField: row.toLowerCase(), diffField: row.toLowerCase(), label: { article: "a", en: row.toLowerCase(), de: row.toLowerCase() } })),
});

function tree(files: Readonly<Record<string, string>>): string {
  const directory = mkdtempSync(join(tmpdir(), "semio-collection-catalog-"));
  for (const [name, text] of Object.entries(files)) {
    mkdirSync(dirname(join(directory, name)), { recursive: true });
    writeFileSync(join(directory, name), text);
  }
  return directory;
}

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

  test("imports every row type of the architect catalog from the module that declares it, and no imported module reaches the catalog again", () => {
    const catalog = parseCollectionCatalogV1(json(SNAPSHOT));
    const owners = resolveCollectionRowOwnersV1(catalog, CATALOG, read);
    expect(owners.size).toBe(64);
    expect([...new Set(owners.values())].sort()).toEqual(["../../🏗️program-structure/🟦️.ts", "../../📋️requirements/🟦️.ts", "../../🗃️decision-records/🟦️.ts"].sort());
    const facts = moduleFacts(CATALOG);
    expect(facts.modules.map((path) => path.slice(path.indexOf("🧬️schema"))).sort()).toEqual(["🧬️schema/🏗️program-structure/🟦️.ts", "🧬️schema/📋️requirements/🟦️.ts", "🧬️schema/🗃️decision-records/🟦️.ts"].sort());
    const source = ts.createSourceFile(CATALOG, read(CATALOG)!, ts.ScriptTarget.ESNext);
    for (const statement of source.statements) {
      if (!ts.isImportDeclaration(statement)) continue;
      const module = resolve(dirname(CATALOG), (statement.moduleSpecifier as ts.StringLiteral).text);
      const names = (statement.importClause!.namedBindings as ts.NamedImports).elements.map((element) => element.name.text);
      const declared = moduleFacts(module).declared;
      for (const name of names) expect(declared.has(name), `${name} is declared in ${module}`).toBe(true);
      expect(closure(module).has(CATALOG), `${module} reaches the catalog`).toBe(false);
    }
    expect(renderCollectionCatalogTypeScriptV1(catalog, owners)).toBe(read(CATALOG)!);
  });

  test("resolves row types through a root that re-exports the catalog, and refuses an ambiguous, missing or cyclic row type", () => {
    const directory = tree({
      "root.ts": 'import type { Alpha } from "./a.ts";\nimport type { Beta } from "./b.ts";\nexport type { Alpha, Beta };\nexport type { DemoKey } from "./catalog/x.ts";\n',
      "a.ts": "export interface Alpha {\n  id: string;\n}\n",
      "b.ts": "export type Beta = { readonly id: string };\n",
      "clash.ts": "export interface Beta {\n  id: number;\n}\n",
      "catalog/.keep": "",
    });
    try {
      const output = join(directory, "catalog/x.ts");
      const owners = resolveCollectionRowOwnersV1(catalogOf(["Alpha", "Beta"], "../root.ts"), output, read);
      expect(Object.fromEntries(owners)).toEqual({ Alpha: "../a.ts", Beta: "../b.ts" });
      const rendered = renderCollectionCatalogTypeScriptV1(catalogOf(["Alpha", "Beta"], "../root.ts"), owners);
      expect(rendered).toContain('import type { Alpha } from "../a.ts";');
      expect(rendered).toContain('import type { Beta } from "../b.ts";');
      expect(rendered).not.toContain("root.ts");
      expect(() => resolveCollectionRowOwnersV1(catalogOf(["Gamma"], "../root.ts"), output, read)).toThrow("Gamma must be declared by exactly one module below ../root.ts, found 0");
      writeFileSync(join(directory, "root.ts"), `${read(join(directory, "root.ts"))}import type { Beta as Other } from "./clash.ts";\nexport type { Other };\n`);
      expect(() => resolveCollectionRowOwnersV1(catalogOf(["Beta"], "../root.ts"), output, read)).toThrow("Beta must be declared by exactly one module below ../root.ts, found 2");
      writeFileSync(join(directory, "root.ts"), 'export interface Alpha {\n  id: string;\n}\nexport type { DemoKey } from "./catalog/x.ts";\n');
      expect(() => resolveCollectionRowOwnersV1(catalogOf(["Alpha"], "../root.ts"), output, read)).toThrow("which imports the catalog: the generated import would close a cycle");
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
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
