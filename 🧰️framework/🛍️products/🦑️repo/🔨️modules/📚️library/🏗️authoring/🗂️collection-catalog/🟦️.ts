import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve } from "node:path";

/** 🗂️ One id-keyed collection of an artifact snapshot, as the `x-semio-collection-catalog` extension of its JSON schema declares it. */
export interface CollectionCatalogEntryV1 {
  readonly key: string;
  readonly variant: string;
  readonly entity: string;
  readonly row: string;
  readonly patch: string;
  readonly delta: string;
  readonly rowsField: string;
  readonly diffField: string;
  readonly label: { readonly article: "a" | "an"; readonly en: string; readonly de: string };
}

/** 🗂️ The `x-semio-collection-catalog` extension: names, type paths and the collections the generic collection mutations range over. `typescript.rows` is the entry of the TypeScript module tree (relative to the generated catalog) in which the row types are declared; the generated catalog imports each row type from the module of that tree that declares it, never from the entry. */
export interface CollectionCatalogV1 {
  readonly keyType: string;
  readonly rowType: string;
  readonly labelType: string;
  readonly tag: string;
  readonly content: string;
  readonly headerField: string;
  readonly rust: { readonly snapshot: string; readonly diff: string; readonly deltas: string; readonly rows: string; readonly header: string; readonly id: string; readonly canonicalJsonOwner: string };
  readonly typescript: { readonly rows: string };
  readonly output: { readonly rust: string; readonly typescript: string };
  readonly collections: readonly CollectionCatalogEntryV1[];
}

const IDENT = /^[A-Za-z_][A-Za-z0-9_]*$/u, PASCAL = /^[A-Z][A-Za-z0-9]*$/u, CAMEL = /^[a-z][A-Za-z0-9]*$/u, KEBAB = /^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u, PATH = /^(?:crate|[a-z_][a-z0-9_]*)(?:::[A-Za-z_][A-Za-z0-9_]*)*$/u, OUTPUT = /^(?:\.\.\/)*(?:[^/\\.][^/\\]*\/)*[^/\\.][^/\\]*$/u;

/** 🧬️ Admits one JSON schema document: its catalog extension must be complete, and its `RegisterKey`-style enum and `RegisterRow`-style oneOf must list exactly the catalog's collections in order. */
export function parseCollectionCatalogV1(document: unknown): CollectionCatalogV1 {
  const root = object(document, "schema document");
  const catalog = object(root["x-semio-collection-catalog"], "x-semio-collection-catalog");
  const text = (owner: Record<string, unknown>, key: string, pattern: RegExp): string => { const value = owner[key]; if (typeof value !== "string" || !pattern.test(value)) throw Error(`collection catalog ${key} is invalid: ${JSON.stringify(value)}`); return value; };
  const rust = object(catalog.rust, "rust"), typescript = object(catalog.typescript, "typescript"), output = object(catalog.output, "output");
  if (!Array.isArray(catalog.collections) || catalog.collections.length === 0) throw Error("collection catalog declares no collections");
  const collections = catalog.collections.map((value, index): CollectionCatalogEntryV1 => {
    const entry = object(value, `collection ${index}`), label = object(entry.label, `collection ${index} label`);
    if (label.article !== "a" && label.article !== "an") throw Error(`collection ${index} article must be a or an`);
    if (typeof label.en !== "string" || !label.en || typeof label.de !== "string" || !label.de) throw Error(`collection ${index} needs English and German labels`);
    return { key: text(entry, "key", CAMEL), variant: text(entry, "variant", PASCAL), entity: text(entry, "entity", KEBAB), row: text(entry, "row", PASCAL), patch: text(entry, "patch", PASCAL), delta: text(entry, "delta", PASCAL), rowsField: text(entry, "rowsField", IDENT), diffField: text(entry, "diffField", IDENT), label: { article: label.article, en: label.en, de: label.de } };
  });
  for (const field of ["key", "variant", "entity", "diffField"] as const) if (new Set(collections.map((entry) => entry[field])).size !== collections.length) throw Error(`collection catalog ${field} values must be unique`);
  const parsed: CollectionCatalogV1 = {
    keyType: text(catalog, "keyType", PASCAL), rowType: text(catalog, "rowType", PASCAL), labelType: text(catalog, "labelType", PASCAL), tag: text(catalog, "tag", CAMEL), content: text(catalog, "content", CAMEL), headerField: text(catalog, "headerField", IDENT),
    rust: { snapshot: text(rust, "snapshot", PATH), diff: text(rust, "diff", PATH), deltas: text(rust, "deltas", PATH), rows: text(rust, "rows", PATH), header: text(rust, "header", PATH), id: text(rust, "id", PATH), canonicalJsonOwner: text(rust, "canonicalJsonOwner", IDENT) },
    typescript: { rows: text(typescript, "rows", /^\.{1,2}\/.+\.ts$/u) },
    output: { rust: text(output, "rust", OUTPUT), typescript: text(output, "typescript", OUTPUT) },
    collections,
  };
  const defs = object(root.$defs, "$defs"), keys = object(defs[parsed.keyType], parsed.keyType), rows = object(defs[parsed.rowType], parsed.rowType);
  if (JSON.stringify(keys.enum) !== JSON.stringify(collections.map((entry) => entry.key))) throw Error(`${parsed.keyType} enum must list the catalog keys in order`);
  if (!Array.isArray(rows.oneOf) || rows.oneOf.length !== collections.length) throw Error(`${parsed.rowType} oneOf must hold one alternative per collection`);
  rows.oneOf.forEach((value, index) => {
    const properties = object(object(value, `${parsed.rowType} alternative ${index}`).properties, `${parsed.rowType} alternative ${index} properties`);
    const entry = collections[index]!;
    if (object(properties[parsed.tag], "tag").const !== entry.key) throw Error(`${parsed.rowType} alternative ${index} must tag ${entry.key}`);
    const reference = object(properties[parsed.content], "content").$ref;
    if (typeof reference !== "string" || !reference.endsWith(`/$defs/${entry.row}`)) throw Error(`${parsed.rowType} alternative ${index} must reference the ${entry.row} row`);
  });
  return parsed;
}

export interface CollectionCatalogRustFileV1 {
  readonly folder: string | null;
  readonly text: string;
}

const RUST_CHILDREN = [
  { folder: "🪪️key-descriptors", module: "key_descriptors", doc: "🪪️ Entity names, labels and diff fields of the collection keys." },
  { folder: "🔎️key-base-access", module: "key_base_access", doc: "🔎️ Position, length, row access, removal and renaming of a collection key in a snapshot." },
  { folder: "🧾️row-operations", module: "row_operations", doc: "🧾️ Key, header, insertion, replacement and equality of one typed row." },
  { folder: "🔁️row-conversions", module: "row_conversions", doc: "🔁️ Conversions of every row type into its typed row." },
] as const;

/** 🦀️ Renders the Rust catalog as a root module plus cohesive child modules: the key and typed row enums, labels, and the per-collection diff constructors the generic collection mutations call. The root is the entry with `folder` null. */
export function renderCollectionCatalogRustV1(catalog: CollectionCatalogV1): readonly CollectionCatalogRustFileV1[] {
  const { keyType: K, rowType: R, labelType: L, headerField: h, collections: c } = catalog;
  const arms = (body: (entry: CollectionCatalogEntryV1) => string, self = "Self"): string => c.map((entry) => `            ${self}::${entry.variant}${body(entry)},`).join("\n");
  const owner = catalog.rust.canonicalJsonOwner, value = "semio_framework_value";
  const rowArm = (body: (entry: CollectionCatalogEntryV1) => string): string => c.map((entry) => `            Self::${entry.variant}(row) => ${body(entry)},`).join("\n");
  const sorted = (items: string[]): string => [...new Set(items)].sort().join(", ");
  const root = `//! 🤖️ Generated by the collection-catalog generator (\`📚️library/🏗️authoring/🗂️collection-catalog\`) from the \`x-semio-collection-catalog\` extension of its snapshot schema. Edit the schema and regenerate.

use ${catalog.rust.snapshot};
use ${catalog.rust.diff};
use ${catalog.rust.header};
use ${catalog.rust.id};
use ${catalog.rust.deltas}::{${sorted(c.map((entry) => entry.delta))}};
use ${catalog.rust.rows}::{${sorted(c.flatMap((entry) => [entry.row, entry.patch]))}};

/// 🗂️ Key of one id-keyed collection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ${value}::ToValue, ${value}::FromValue, ${value}::RetireOwned, ${value}::CanonicalJsonTree, ${value}::RetainedClone)]
#[canonical_json(owner = ${owner})]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub enum ${K} {
${c.map((entry) => `    ${entry.variant},`).join("\n")}
}

/// 🗂️ One typed row of one collection, adjacently tagged by its key.
#[derive(Clone, Debug, PartialEq, ${value}::ToValue, ${value}::FromValue, ${value}::RetireOwned, ${value}::CanonicalJsonTree, ${value}::RetainedClone)]
#[canonical_json(owner = ${owner})]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "${catalog.tag}", content = "${catalog.content}", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "${catalog.tag}", content = "${catalog.content}", rename_all = "camelCase"))]
pub enum ${R} {
${c.map((entry) => `    ${entry.variant}(${entry.row}),`).join("\n")}
}

/// 🏷️ English and German labels of one collection's rows, with the English indefinite article.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ${L} {
    pub article: &'static str,
    pub en: &'static str,
    pub de: &'static str,
}

impl ${L} {
    /// 🏷️ The English label with its capitalised indefinite article, as a sentence starts.
    pub fn sentence_subject(self) -> String {
        let mut article = self.article.to_string();
        article[..1].make_ascii_uppercase();
        format!("{article} {}", self.en)
    }
}

impl ${K} {
    /// 🗂️ Every collection, in declaration order.
    pub const ALL: [${K}; ${c.length}] = [${c.map((entry) => `${K}::${entry.variant}`).join(", ")}];
}

${RUST_CHILDREN.map((child) => `#[path = "${child.folder}/🦀️.rs"]\nmod ${child.module};`).join("\n\n")}
`;
  const child = (index: number, body: string): string => `//! ${RUST_CHILDREN[index]!.doc} Generated by the collection-catalog generator; edit the schema and regenerate.\n\nuse super::*;\n\n${body.replace(/^\n+/u, "")}\n`;
  const bodies = [
    `impl ${K} {
    /// 🪪️ The kebab-case entity name of this collection's rows.
    pub fn entity(self) -> &'static str {
        match self {
${arms((entry) => ` => "${entry.entity}"`)}
        }
    }

    /// 🏷️ This collection's row labels.
    pub fn label(self) -> ${L} {
        match self {
${arms((entry) => ` => ${L} { article: "${entry.label.article}", en: "${entry.label.en}", de: "${entry.label.de}" }`)}
        }
    }

    /// 🧭️ The diff field this collection's deltas live in.
    pub fn diff_field(self) -> &'static str {
        match self {
${arms((entry) => ` => "${entry.diffField}"`)}
        }
    }

    /// 🧭️ The collection whose deltas live in \`field\`.
    pub fn from_diff_field(field: &str) -> Option<Self> {
        match field {
${c.map((entry) => `            "${entry.diffField}" => Some(Self::${entry.variant}),`).join("\n")}
            _ => None,
        }
    }
}`,
    `impl ${K} {
    /// 🔎️ Position of the row with \`id\` in this collection of \`base\`.
    pub fn position(self, base: &${last(catalog.rust.snapshot)}, id: &${last(catalog.rust.id)}) -> Option<usize> {
        match self {
${arms((entry) => ` => base.${entry.rowsField}.iter().position(|row| row.${h}.id == *id)`)}
        }
    }

    /// 📏️ Number of rows in this collection of \`base\`.
    pub fn len(self, base: &${last(catalog.rust.snapshot)}) -> usize {
        match self {
${arms((entry) => ` => base.${entry.rowsField}.len()`)}
        }
    }

    /// 🧾️ Typed copy of the row at \`index\` in this collection of \`base\`.
    pub fn row_at(self, base: &${last(catalog.rust.snapshot)}, index: usize) -> Option<${R}> {
        match self {
${arms((entry) => ` => base.${entry.rowsField}.get(index).cloned().map(${R}::${entry.variant})`)}
        }
    }

    /// 🗑️ Diff removing the row at \`index\` of this collection of \`base\`.
    pub fn removal(self, base: &${last(catalog.rust.snapshot)}, index: usize) -> ${last(catalog.rust.diff)} {
        match self {
${arms((entry) => ` => ${last(catalog.rust.diff)} { ${entry.diffField}: Some(${entry.delta}::removal(&base.${entry.rowsField}, index)), ..Default::default() }`)}
        }
    }

    /// 🏷️ Diff setting the identity name of the row with \`id\` in this collection.
    pub fn renaming(self, id: &${last(catalog.rust.id)}, name: String) -> ${last(catalog.rust.diff)} {
        match self {
${arms((entry) => ` => ${last(catalog.rust.diff)} { ${entry.diffField}: Some(${entry.delta}::modification(id.0.clone(), ${entry.patch} { name: Some(name), ..Default::default() })), ..Default::default() }`)}
        }
    }
}`,
    `impl ${R} {
    /// 🗂️ The collection this row belongs to.
    pub fn key(&self) -> ${K} {
        match self {
${c.map((entry) => `            Self::${entry.variant}(_) => ${K}::${entry.variant},`).join("\n")}
        }
    }

    /// 🪪️ This row's entity header.
    pub fn header(&self) -> &${last(catalog.rust.header)} {
        match self {
${rowArm(() => `&row.${h}`)}
        }
    }

    /// 🌱️ Diff inserting this row at \`index\` of its collection.
    pub fn insertion(&self, index: usize) -> ${last(catalog.rust.diff)} {
        match self {
${rowArm((entry) => `${last(catalog.rust.diff)} { ${entry.diffField}: Some(${entry.delta}::insertion(index, row.clone())), ..Default::default() }`)}
        }
    }

    /// ♻️ Diff replacing the row at \`index\` of its collection of \`base\` with this row, in place.
    pub fn replacement(&self, base: &${last(catalog.rust.snapshot)}, index: usize) -> ${last(catalog.rust.diff)} {
        match self {
${rowArm((entry) => `{
                let mut delta = ${entry.delta}::removal(&base.${entry.rowsField}, index);
                delta.absorb(${entry.delta}::insertion(index, row.clone()));
                ${last(catalog.rust.diff)} { ${entry.diffField}: Some(delta), ..Default::default() }
            }`)}
        }
    }

    /// ⚖️ Whether this row equals the row at \`index\` of its collection of \`base\`.
    pub fn equals_row_at(&self, base: &${last(catalog.rust.snapshot)}, index: usize) -> bool {
        match self {
${rowArm((entry) => `base.${entry.rowsField}.get(index) == Some(row)`)}
        }
    }
}`,
    `${c.map((entry) => `
impl From<${entry.row}> for ${R} {
    fn from(row: ${entry.row}) -> Self {
        Self::${entry.variant}(row)
    }
}`).join("\n")}`,
  ];
  return [{ folder: null, text: root }, ...RUST_CHILDREN.map((entry, index) => ({ folder: entry.folder, text: child(index, bodies[index]!) }))];
}

/** 🔗️ Reads a TypeScript source, or undefined when the file does not exist. */
export type CollectionCatalogReaderV1 = (path: string) => string | undefined;

const RELATIVE_MODULE = /^[ \t]*(?:import|export)\s+(?:type\s+)?(?:\{[^}]*\}|\*(?:\s+as\s+\w+)?)\s*from\s*"(\.{1,2}\/[^"\n]+\.ts)"/gmu;

const declares = (source: string, name: string): boolean => new RegExp(`^[ \\t]*(?:export\\s+)?(?:declare\\s+)?(?:interface|type|class|enum)\\s+${name}\\b`, "mu").test(source);

function reachable(entry: string, stop: string, read: CollectionCatalogReaderV1): ReadonlySet<string> {
  const seen = new Set<string>();
  const queue = [entry];
  for (let path = queue.pop(); path !== undefined; path = queue.pop()) {
    if (seen.has(path)) continue;
    const source = path === stop ? undefined : read(path);
    if (source === undefined && path !== stop) continue;
    seen.add(path);
    if (source !== undefined) for (const match of source.matchAll(RELATIVE_MODULE)) queue.push(resolve(dirname(path), match[1]!));
  }
  return seen;
}

/** 🧭️ The module that declares each row type, as the import specifier the generated catalog at `outputPath` uses for it. The modules are found by following the relative imports and re-exports of the `typescript.rows` entry (the catalog itself left out); a row type must be declared by exactly one of them, and that module must not reach the catalog again, so the generated import can never close a cycle. */
export function resolveCollectionRowOwnersV1(catalog: CollectionCatalogV1, outputPath: string, read: CollectionCatalogReaderV1): ReadonlyMap<string, string> {
  const tree = [...reachable(resolve(dirname(outputPath), catalog.typescript.rows), outputPath, read)].filter((path) => path !== outputPath);
  if (tree.length === 0) throw Error(`collection catalog typescript.rows ${catalog.typescript.rows} does not name a TypeScript module`);
  const owners = new Map<string, string>();
  for (const row of new Set(catalog.collections.map((collection) => collection.row))) {
    const declaring = tree.filter((path) => declares(read(path)!, row));
    if (declaring.length !== 1) throw Error(`collection catalog row type ${row} must be declared by exactly one module below ${catalog.typescript.rows}, found ${declaring.length}`);
    if (reachable(declaring[0]!, outputPath, read).has(outputPath)) throw Error(`collection catalog row type ${row} is declared in ${declaring[0]}, which imports the catalog: the generated import would close a cycle`);
    const specifier = relative(dirname(outputPath), declaring[0]!).split("\\").join("/");
    owners.set(row, specifier.startsWith(".") ? specifier : `./${specifier}`);
  }
  return owners;
}

/** 🟦️ Renders the TypeScript catalog: the key and row unions and the collection table, over the TypeScript row types imported from the modules that declare them (`owners`, see `resolveCollectionRowOwnersV1`). */
export function renderCollectionCatalogTypeScriptV1(catalog: CollectionCatalogV1, owners: ReadonlyMap<string, string>): string {
  const c = catalog.collections;
  const bySpecifier = new Map<string, Set<string>>();
  for (const entry of c) {
    const specifier = owners.get(entry.row);
    if (specifier === undefined) throw Error(`collection catalog row type ${entry.row} has no owning module`);
    bySpecifier.set(specifier, (bySpecifier.get(specifier) ?? new Set()).add(entry.row));
  }
  const imports = [...bySpecifier.keys()].sort().map((specifier) => `import type { ${[...bySpecifier.get(specifier)!].sort().join(", ")} } from "${specifier}";`).join("\n");
  return `/** 🤖️ Generated by the collection-catalog generator (\`📚️library/🏗️authoring/🗂️collection-catalog\`) from the \`x-semio-collection-catalog\` extension of its snapshot schema. Edit the schema and regenerate. */
${imports}

/** 🗂️ Key of one id-keyed collection. */
export type ${catalog.keyType} = ${c.map((entry) => JSON.stringify(entry.key)).join(" | ")};

/** 🗂️ One typed row of one collection, adjacently tagged by its key. */
export type ${catalog.rowType} =
${c.map((entry) => `  | { ${catalog.tag}: ${JSON.stringify(entry.key)}; ${catalog.content}: ${entry.row} }`).join("\n")};

/** 🗂️ One collection: its key, entity, snapshot and diff fields, and row labels. */
export interface ${catalog.keyType}Entry { readonly key: ${catalog.keyType}; readonly entity: string; readonly rowsField: string; readonly diffField: string; readonly label: { readonly article: "a" | "an"; readonly en: string; readonly de: string } }

/** 🗂️ Every collection, in declaration order. */
export const ${constName(catalog.keyType)}: readonly ${catalog.keyType}Entry[] = [
${c.map((entry) => `  { key: ${JSON.stringify(entry.key)}, entity: ${JSON.stringify(entry.entity)}, rowsField: ${JSON.stringify(entry.rowsField)}, diffField: ${JSON.stringify(entry.diffField)}, label: ${JSON.stringify(entry.label).replace(/"(\w+)":/gu, "$1: ")} },`).join("\n")}
];
`;
}

/** 🏭️ Writes (or, in check mode, compares) the generated Rust and TypeScript at the catalog's declared `output` paths, relative to its schema, and returns the paths that differ. */
export function generateCollectionCatalogV1(schemaPath: string, mode: "write" | "check"): readonly string[] {
  const catalog = parseCollectionCatalogV1(JSON.parse(readFileSync(schemaPath, "utf8")));
  const rust = join(dirname(schemaPath), catalog.output.rust);
  const typescript = join(dirname(schemaPath), catalog.output.typescript);
  const owners = resolveCollectionRowOwnersV1(catalog, typescript, (path) => (existsSync(path) ? readFileSync(path, "utf8") : undefined));
  const outputs: [string, string][] = [...renderCollectionCatalogRustV1(catalog).map((file): [string, string] => [file.folder === null ? rust : join(dirname(rust), file.folder, basename(rust)), file.text]), [typescript, renderCollectionCatalogTypeScriptV1(catalog, owners)]];
  const stale = outputs.filter(([path, text]) => !existsSync(path) || readFileSync(path, "utf8") !== text).map(([path]) => path);
  if (mode === "write") for (const [path, text] of outputs) if (stale.includes(path)) { mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, text); }
  return stale;
}

function object(value: unknown, name: string): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw Error(`${name} must be an object`);
  return value as Record<string, unknown>;
}

function last(path: string): string {
  return path.split("::").at(-1)!;
}

function constName(type: string): string {
  return type.replace(/Key$/u, "").replace(/([a-z0-9])([A-Z])/gu, "$1_$2").toUpperCase() + "S";
}
