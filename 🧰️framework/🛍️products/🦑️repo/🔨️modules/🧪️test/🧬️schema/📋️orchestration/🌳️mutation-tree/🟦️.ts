/** 🌲️ Mutation tree
 *
 * Discovers the mutation leaves and the `#[derive(Mutations)]` aggregates of the repository, walks its Rust sources, and names the owner and artifact scope of a mutation schema.
 */

import { type Dirent, existsSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { readJsonObject, schemaKind } from "../📚️schema-documents/🟦️.ts";
import { rustGroupEnd, rustLex, type RustToken } from "../🦀️rust-tokens/🟦️.ts";

//#region ⚖️MutationPayloadParity
/** 🧬️ The wire layout of a `#[derive(Mutations)]` aggregate enum — its `#[value(tag, content, rename_all)]` container attribute. */
export type MutationAggregateLayout = Readonly<{ tag: string | null; content: string | null; renameAll: string | null }>;

/** 🗂️ One `#[derive(Mutations)]` aggregate enum: its file, name, wire layout, and every variant ident with its `#[value(rename)]`. */
export type MutationAggregate = Readonly<{ path: string; name: string; layout: MutationAggregateLayout; variants: ReadonlyMap<string, string | null>; payloadTypes: ReadonlyMap<string, string> }>;

/** 🦀️ One enum of a Rust source with its `#[value(...)]` wire attributes: an aggregate when it derives `Mutations`, a wrapped leaf when
 * its `#[mutation_leaf(payload = <Variant>)]` names the variant whose content is the schema-described, editable payload. */
export type RustValueEnum = MutationAggregate & Readonly<{ mutations: boolean; payloadVariant: string | null }>;

export type MutationLeafRecord = Readonly<{ directory: string; root: string; variant: string; kind: string; schemaPath: string }>;
type MutationTree = { readonly leaves: MutationLeafRecord[]; readonly aggregates: MutationAggregate[]; readonly fixtures: string[]; readonly features: string[]; readonly wrappers: Map<string, RustValueEnum[]>; readonly schemas: string[] };

export const MUTATION_TREE_ROOTS = ["✏️s", "🧰️framework"];
const MUTATION_TREE_SKIPPED = new Set(["node_modules", "target", "🗑️generated", ".git", "dist"]);

/** 🏷️ The `key = "value"` (or `key = Ident`) pairs of every `#[<name>(...)]` attribute among `attributes` (a later one wins). */
function rustValueAttributes(attributes: readonly (readonly RustToken[])[], name = "value"): Map<string, string> {
  const pairs = new Map<string, string>();
  for (const attribute of attributes) {
    if (attribute[0]?.text !== name || attribute[1]?.text !== "(") continue;
    for (let index = 2; index + 2 < attribute.length; index += 1) {
      const [key, equals, literal] = [attribute[index]!, attribute[index + 1]!, attribute[index + 2]!];
      if (key.kind === "ident" && equals.text === "=" && (literal.kind === "literal" || (literal.kind === "ident" && attribute[index + 3]?.text !== ":"))) pairs.set(key.text, literal.text.replace(/^b?r?#*"|"#*$/gu, ""));
    }
  }
  return pairs;
}

/** 📡️ Every `#[derive(… Mutations …)]` enum of one Rust source with its `#[value(...)]` wire layout and variants, read from tokens. */
export function rustMutationAggregates(path: string, source: string): MutationAggregate[] {
  return rustValueEnums(path, source).filter((candidate) => candidate.mutations);
}

/** 🌱️ Every enum of one Rust source with its `#[value(...)]` wire layout, variants (with `#[value(rename)]`) and each tuple variant's payload type, read from tokens. */
export function rustValueEnums(path: string, source: string): RustValueEnum[] {
  const tokens = rustLex(source);
  const enums: RustValueEnum[] = [];
  const attributesAt = (index: number, pending: RustToken[][]): number => {
    const inner = tokens[index + 1]?.text === "!";
    const open = index + (inner ? 2 : 1);
    if (tokens[index]?.text !== "#" || tokens[open]?.text !== "[") return -1;
    const close = rustGroupEnd(tokens, open);
    if (!inner) pending.push(tokens.slice(open + 1, close));
    return close;
  };
  let pending: RustToken[][] = [];
  for (let index = 0; index < tokens.length; index += 1) {
    const token = tokens[index]!;
    const attribute = attributesAt(index, pending);
    if (attribute >= 0) {
      index = attribute;
      continue;
    }
    if (token.kind === "ident" && token.text === "pub") {
      if (tokens[index + 1]?.text === "(") index = rustGroupEnd(tokens, index + 1);
      continue;
    }
    if (token.kind === "ident" && token.text === "enum" && tokens[index + 1]?.kind === "ident") {
      let open = index + 2;
      while (open < tokens.length && tokens[open]!.text !== "{") open += 1;
      const close = rustGroupEnd(tokens, open);
      const mutations = pending.some((attribute) => attribute[0]?.text === "derive" && attribute.some((part) => part.kind === "ident" && part.text === "Mutations"));
      const layout = rustValueAttributes(pending);
      const variants = new Map<string, string | null>();
      const payloadTypes = new Map<string, string>();
      let variantAttributes: RustToken[][] = [];
      let variant: string | null = null;
      for (let cursor = open + 1; cursor < close; cursor += 1) {
        const part = tokens[cursor]!;
        const skipped = attributesAt(cursor, variantAttributes);
        if (skipped >= 0) cursor = skipped;
        else if (part.kind === "punct" && (part.text === "(" || part.text === "{")) {
          const end = rustGroupEnd(tokens, cursor);
          const named = tokens.slice(cursor + 1, end).filter((entry) => entry.kind === "ident");
          if (part.text === "(" && variant !== null && named.length > 0) payloadTypes.set(variant, named.at(-1)!.text);
          cursor = end;
        } else if (part.kind === "ident" && variant === null) variant = part.text;
        else if (part.kind === "punct" && part.text === ",") {
          if (variant !== null) variants.set(variant, rustValueAttributes(variantAttributes).get("rename") ?? null);
          [variant, variantAttributes] = [null, []];
        }
      }
      if (variant !== null) variants.set(variant, rustValueAttributes(variantAttributes).get("rename") ?? null);
      enums.push({ path, name: tokens[index + 1]!.text, layout: { tag: layout.get("tag") ?? null, content: layout.get("content") ?? null, renameAll: layout.get("rename_all") ?? null }, variants, payloadTypes, mutations, payloadVariant: rustValueAttributes(pending, "mutation_leaf").get("payload") ?? null });
      pending = [];
      index = close;
      continue;
    }
    pending = [];
  }
  return enums;
}

/** 🌳️ Every mutation leaf descriptor, `#[derive(Mutations)]` aggregate and committed `🦠️mutation/🔣️.json` fixture under `roots` (the
 * source roots by default). */
export function mutationTree(repoRoot: string, roots: readonly string[] = MUTATION_TREE_ROOTS): MutationTree {
  const tree: MutationTree = { leaves: [], aggregates: [], fixtures: [], features: [], wrappers: new Map(), schemas: [] };
  const wrappersOf = (leaf: string): RustValueEnum[] =>
    [`${leaf}/🦀️.rs`, `${leaf}/🦠️mutation/🦀️.rs`].flatMap((path) => {
      const source = existsSync(join(repoRoot, path)) ? readFileSync(join(repoRoot, path), "utf8") : "";
      return source.includes("mutation_leaf") ? rustValueEnums(path, source).filter((candidate) => candidate.payloadVariant !== null) : [];
    });
  const visit = (directory: string, root: string | null): void => {
    let entries: Dirent[];
    try {
      entries = readdirSync(join(repoRoot, directory), { withFileTypes: true });
    } catch {
      return;
    }
    const name = directory.slice(directory.lastIndexOf("/") + 1);
    const files = new Set(entries.filter((entry) => entry.isFile()).map((entry) => entry.name));
    if (root !== null && root !== directory && files.has("🔣️.json")) {
      const descriptor = readJsonObject(repoRoot, `${directory}/🔣️.json`);
      if (typeof descriptor?.aggregateVariant === "string" && typeof descriptor.payloadSchema === "string") {
        tree.leaves.push({ directory, root, variant: descriptor.aggregateVariant, kind: String(descriptor.semanticKind ?? ""), schemaPath: `${directory}/${descriptor.payloadSchema}` });
        const wrappers = wrappersOf(directory);
        if (wrappers.length > 0) tree.wrappers.set(directory, wrappers);
      }
    }
    if (name === "🧬️mutations" && files.has("🦀️.rs")) tree.aggregates.push(...rustMutationAggregates(`${directory}/🦀️.rs`, readFileSync(join(repoRoot, directory, "🦀️.rs"), "utf8")));
    if (name === "🦠️mutation" && files.has("🔣️.json") && directory.split("/").some((segment) => segment.endsWith("fixtures") || segment === "🧪️tests")) tree.fixtures.push(`${directory}/🔣️.json`);
    if (files.has("🥒️.feature") && directory.split("/").at(-2) === "🧪️tests") tree.features.push(`${directory}/🥒️.feature`);
    if (directory.split("/").includes("🧬️schema")) for (const file of files) if (file.endsWith(".json")) tree.schemas.push(`${directory}/${file}`);
    const inner = name === "🧬️mutations" && (files.has("🦀️.rs") || !directory.split("/").at(-2)!.endsWith("fixtures")) ? directory : name.endsWith("fixtures") ? null : root;
    for (const entry of entries) if (entry.isDirectory() && !MUTATION_TREE_SKIPPED.has(entry.name)) visit(`${directory}/${entry.name}`, inner);
  };
  for (const root of roots) if (existsSync(join(repoRoot, root))) visit(root, null);
  return tree;
}

/** 🎒️ The census owner of a repository path: the plugin under `🔌️plugins`, else the product under `🛍️products`, else the root. */
export function mutationPayloadOwner(path: string): string {
  const segments = path.split("/");
  const plugin = segments.indexOf("🔌️plugins");
  const product = segments.indexOf("🛍️products");
  return schemaKind(plugin >= 0 ? segments[plugin + 1]! : product >= 0 ? segments[product + 1]! : segments[0]!);
}

/** 🗿️ The artifact tree a repository path lies in (`…/🗿️artifacts/<artifact>`), else `null` (framework and product trees): a leaf, its
 * aggregate and its fixtures never pair across two artifacts, so an orphaned fixture stays `unmapped` instead of borrowing a foreign
 * aggregate that happens to declare the same variant name. */
export function mutationArtifactScope(path: string): string | null {
  const segments = path.split("/");
  const artifacts = segments.indexOf("🗿️artifacts");
  return artifacts >= 0 && artifacts + 1 < segments.length ? segments.slice(0, artifacts + 2).join("/") : null;
}

/** 📏️ The number of leading path segments `left` and `right` share. */
export function sharedSegments(left: string, right: string): number {
  const [a, b] = [left.split("/"), right.split("/")];
  let count = 0;
  while (count < a.length && count < b.length && a[count] === b[count]) count += 1;
  return count;
}
//#endregion ⚖️MutationPayloadParity

//#region 🦀️RustSources
/** 🏠️ The roots the history gates read Rust from: the mutation roots plus the hub compositions. */
const RUST_SOURCE_ROOTS = [...MUTATION_TREE_ROOTS, "🌎️hub"];

/** 🔎️ Every `.rs` source under `under` whose text `keep` admits, repository-relative and sorted, with its text; build output, generated
 * and hidden directories are skipped. */
export function rustSources(repoRoot: string, under: string, keep: (source: string) => boolean): { readonly path: string; readonly source: string }[] {
  return repositorySources(repoRoot, under, ".rs", keep);
}

/** 📂️ Every source ending in `extension` under `under` (else every Rust source root) whose text `keep` admits — [`rustSources`]' walk. */
export function repositorySources(repoRoot: string, under: string, extension: string, keep: (source: string) => boolean): { readonly path: string; readonly source: string }[] {
  const found: { path: string; source: string }[] = [];
  const walk = (directory: string): void => {
    let entries: Dirent[];
    try {
      entries = readdirSync(join(repoRoot, directory), { withFileTypes: true });
    } catch {
      return;
    }
    for (const entry of entries) {
      const path = `${directory}/${entry.name}`;
      if (entry.isDirectory()) {
        if (!MUTATION_TREE_SKIPPED.has(entry.name) && !entry.name.startsWith(".")) walk(path);
      } else if (entry.isFile() && entry.name.endsWith(extension) && path.startsWith(under)) {
        const source = readFileSync(join(repoRoot, path), "utf8");
        if (keep(source)) found.push({ path, source });
      }
    }
  };
  const directory = under !== "" && existsSync(join(repoRoot, under)) && !under.endsWith(extension);
  for (const root of directory ? [under] : RUST_SOURCE_ROOTS) if (directory || under === "" || root.startsWith(under) || under.startsWith(root)) walk(root);
  return found.sort((left, right) => left.path.localeCompare(right.path));
}
//#endregion 🦀️RustSources
