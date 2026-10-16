/** ✏️ Mutation editability gate
 *
 * `schema-mutation-editability`: every aggregate is editable in history or declares why not.
 */

import { type SchemaDiagnostic } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { type Dirent, existsSync, readdirSync, readFileSync } from "node:fs";
import { join, posix } from "node:path";
import { isRecord, readJsonObject } from "../📚️schema-documents/🟦️.ts";
import { rustGroupEnd, type RustImplBlock, rustImplBlocks, rustLex, type RustToken } from "../🦀️rust-tokens/🟦️.ts";
import { MUTATION_TREE_ROOTS, mutationPayloadOwner, mutationTree, rustSources, sharedSegments } from "../🌳️mutation-tree/🟦️.ts";
import { MUTATION_LABEL_APP_TRAITS } from "../🏷️mutation-labels/🟦️.ts";

//#region ✏️MutationEditability
/** ✏️ The history-edit verdict of one operation shape: `editable` (an input schema and no foreign-step capability), `inert` (no input
 * schema — a leaf whose descriptor declares `"editable": false`, withdraw-only by declaration (design §22.20), or a non-payload phase of
 * a `#[mutation_leaf(payload = …)]` leaf) or `foreign` (a composite that may emit foreign steps). */
export type MutationEditabilityVerdict = "editable" | "inert" | "foreign";

/** ✏️ One leaf of a `#[derive(Mutations)]` aggregate: its verdict (`inert` when its descriptor declares it withdraw-only) and, for a
 * payload-marked leaf, the inert phase variants beside its editable payload. */
export type MutationLeafEditability = Readonly<{ owner: string; aggregate: string; path: string; kind: string; variant: string; verdict: MutationEditabilityVerdict; inert: readonly string[] }>;

/** 🖐️ One hand-written `impl Mutation<S> for T`: the declared reason it is outside the generic history editor — it forwards every
 * payload accessor of a derived aggregate (`forwarding`), it is a config/presence/transient/window/draft lane (`lane`), a test fixture
 * (`fixture`), uninhabited (`empty`), or no app names it as its document `type Mutation`, so no history row ever holds one
 * (`unexposed`) — or the finding `aggregateHandwritten`. */
export type MutationHandwrittenAggregate = Readonly<{ owner: string; path: string; name: string; snapshot: string; reason: "forwarding" | "lane" | "fixture" | "empty" | "unexposed" | "aggregateHandwritten" }>;

/** 📊️ One plugin's share of the editability census: `withdrawOnly` counts the leaves declared `"editable": false`, `inert` those plus
 * every inert phase of a payload-marked leaf. */
export type MutationEditabilityCensusRow = { readonly owner: string; aggregates: number; leaves: number; editable: number; foreign: number; inert: number; withdrawOnly: number; handwritten: number; findings: number; readonly refused: Record<string, number> };

/** ✏️ The `schema-mutation-editability` lint, its census, every leaf verdict and every hand-written aggregate. */
export type MutationEditabilityReport = { readonly diagnostics: readonly SchemaDiagnostic[]; readonly census: readonly MutationEditabilityCensusRow[]; readonly leaves: readonly MutationLeafEditability[]; readonly handwritten: readonly MutationHandwrittenAggregate[] };

const MUTATION_LANE_SEGMENTS = new Set(["🎚️config", "👥️presence", "🫧️transient", "🪟️window", "📝️draft"]);
const MUTATION_PAYLOAD_ACCESSORS = ["INPUT_SCHEMAS", "input_schema", "payload_value", "with_payload_value", "from_payload_value"];

/** 🖐️ The declared reason of a hand-written `impl Mutation<snapshot> for name` at `path` whose body is `body`. */
export function mutationHandwrittenReason(path: string, name: string, snapshot: string, body: readonly RustToken[], source: string): MutationHandwrittenAggregate["reason"] {
  const segments = path.split("/");
  if (new RegExp(`enum\\s+${name.replace(/[$]/gu, "\\$")}\\s*\\{\\s*\\}`, "u").test(source)) return "empty";
  if (MUTATION_PAYLOAD_ACCESSORS.every((accessor) => body.some((token) => token.kind === "ident" && token.text === accessor))) return "forwarding";
  if (segments.some((segment) => MUTATION_LANE_SEGMENTS.has(segment)) || /(Config|Presence|Transient|Draft)$/u.test(snapshot) || /(Config|Presence|Transient|Draft)Mutation$/u.test(name)) return "lane";
  if (segments.some((segment) => segment === "🧪️tests" || segment === "tests" || segment.endsWith("fixtures"))) return "fixture";
  return "aggregateHandwritten";
}

/** 🧭️ The tree a leaf's referenced documents are published from — the TypeScript twin of the derive's `mutation_schema_search_root`:
 * the leaf's plugin (the directory under `🔌️plugins`), else its framework module (the nearest directory under `🔨️modules`), else the
 * module owning its `🧬️schema/🧬️mutations` root. */
export function mutationSchemaSearchRoot(repoRoot: string, schemaPath: string): string {
  const segments = schemaPath.split("/");
  const under = (parent: string): number => segments.findLastIndex((_, index) => index > 0 && segments[index - 1] === parent && index < segments.length - 1);
  const plugin = segments.findIndex((_, index) => index > 0 && segments[index - 1] === "🔌️plugins");
  if (plugin >= 0) return segments.slice(0, plugin + 1).join("/");
  const module = under("🔨️modules");
  if (module >= 0) return segments.slice(0, module + 1).join("/");
  for (let end = segments.length - 1; end > 0; end -= 1) if (segments[end] === "🧬️schema" && existsSync(join(repoRoot, ...segments.slice(0, end + 1), "🧬️mutations"))) return segments.slice(0, end).join("/");
  return segments.slice(0, -1).join("/");
}

/** 🏗️ The crate manifest a leaf's types are built by — the TypeScript twin of the derive's `mutation_leaf_crate_manifest`: the nearest
 * `Cargo.toml` above the payload schema, beside a directory or in its `📦️packages/🦀️rust`. */
export function mutationLeafCrateManifest(repoRoot: string, schemaPath: string): string | undefined {
  const segments = schemaPath.split("/");
  for (let end = segments.length - 1; end > 0; end -= 1) {
    const directory = segments.slice(0, end).join("/");
    const manifest = [`${directory}/Cargo.toml`, `${directory}/📦️packages/🦀️rust/Cargo.toml`].find((candidate) => existsSync(join(repoRoot, candidate)));
    if (manifest !== undefined) return manifest;
  }
  return undefined;
}

/** 🔩️ The `path` of every inline `[dependencies]` entry of the crate manifest `manifest` whose text is `text`, resolved beside it — the
 * TypeScript twin of the derive's `mutation_manifest_path_dependencies` (oracle: Bun's TOML parser). */
export function mutationManifestPathDependencies(manifest: string, text: string): string[] {
  let dependencies = false;
  const found: string[] = [];
  for (const raw of text.split("\n")) {
    const line = raw.trim();
    if (line.startsWith("[")) {
      dependencies = line === "[dependencies]";
      continue;
    }
    const table = line.slice(line.indexOf("=") + 1).trim();
    if (!dependencies || !line.includes("=") || !table.startsWith("{")) continue;
    const path = table.slice(1).split(",").map((entry) => /^path\s*=\s*"([^"]*)"/u.exec(entry.trim())?.[1]).find((value) => value !== undefined);
    if (path !== undefined) found.push(posix.normalize(posix.join(posix.dirname(manifest), path)));
  }
  return found;
}

/** 🧶️ The trees a leaf's referenced documents are published from, own first — the TypeScript twin of the derive's
 * `mutation_schema_search_roots`: its own tree ({@link mutationSchemaSearchRoot}), then the plugin of every path dependency its crate
 * declares ({@link mutationLeafCrateManifest}), so a leaf references across exactly the plugins its types are built from. */
export function mutationSchemaSearchRoots(repoRoot: string, schemaPath: string): string[] {
  const manifest = mutationLeafCrateManifest(repoRoot, schemaPath);
  const dependencies = manifest === undefined ? [] : mutationManifestPathDependencies(manifest, readFileSync(join(repoRoot, manifest), "utf8"));
  const plugins = dependencies.flatMap((dependency) => {
    const segments = dependency.split("/");
    const plugin = segments.findIndex((_, index) => index > 0 && segments[index - 1] === "🔌️plugins");
    return plugin < 0 ? [] : [segments.slice(0, plugin + 1).join("/")];
  });
  return [...new Set([mutationSchemaSearchRoot(repoRoot, schemaPath), ...plugins])];
}

/** 🗂️ `$id` → path of every JSON document inside a `🧬️schema` directory of `root` (fixtures, tests and build output skipped). The derive reads the generated schema index instead of walking. */
export function mutationSchemaDocumentIndex(repoRoot: string, root: string): Map<string, string> {
  const index = new Map<string, string>();
  const walk = (directory: string, schema: boolean): void => {
    let entries: Dirent[];
    try {
      entries = readdirSync(join(repoRoot, directory), { withFileTypes: true });
    } catch {
      return;
    }
    for (const entry of entries) {
      const path = `${directory}/${entry.name}`;
      if (entry.isDirectory() && !entry.name.startsWith(".") && !["🧪️tests", "target", "node_modules", "dist", "🗑️generated", "📦️packages"].includes(entry.name)) walk(path, schema || entry.name === "🧬️schema");
      else if (schema && entry.isFile() && entry.name.endsWith(".json")) {
        const id = readJsonObject(repoRoot, path)?.$id;
        if (typeof id === "string" && !index.has(id)) index.set(id, path);
      }
    }
  };
  walk(root, root.split("/").includes("🧬️schema"));
  return index;
}

/** 🔗️ Every absolute document id the `$ref`s of `document` name, fragments stripped. */
export function mutationSchemaReferences(document: unknown): string[] {
  const found: string[] = [];
  const walk = (node: unknown): void => {
    if (Array.isArray(node)) for (const item of node) walk(item);
    else if (isRecord(node))
      for (const [key, value] of Object.entries(node)) {
        if (key === "$ref" && typeof value === "string" && !value.startsWith("#")) found.push(value.split("#")[0]!);
        else walk(value);
      }
  };
  walk(document);
  return found.filter((id) => id.length > 0);
}

/** 🔗️ The `$id`s a leaf payload schema references (transitively) that neither its search trees ({@link mutationSchemaSearchRoots}: what
 * `#[derive(MutationLeaf)]` embeds and the runtime publishes) nor a framework scope (`frameworkIndex`) holds — the inputs the history
 * editor cannot resolve. */
export function mutationLeafUnpublishedReferences(repoRoot: string, schemaPath: string, rootIndex: ReadonlyMap<string, string>, frameworkIndex: ReadonlyMap<string, string>): string[] {
  const own = readJsonObject(repoRoot, schemaPath);
  const pending = mutationSchemaReferences(own);
  const seen = new Set<string>(typeof own?.$id === "string" ? [own.$id] : []);
  const unresolved: string[] = [];
  while (pending.length > 0) {
    const id = pending.pop()!;
    if (seen.has(id)) continue;
    seen.add(id);
    const path = rootIndex.get(id) ?? frameworkIndex.get(id);
    if (path === undefined) unresolved.push(id);
    else pending.push(...mutationSchemaReferences(readJsonObject(repoRoot, path)));
  }
  return unresolved.sort();
}

/** 🪆️ The accessors that read a composed child's content off its parent handle (`ArtifactChild::local_owner` and its twins); the writers
 * (`set_local_owner`, `with_local_owner`, …) and the retirement transfer (`take_local_owner`) are not reads. */
const CHILD_CONTENT_READS = new Set(["local_owner", "require_local_owner", "local_text", "local_text_owner"]);

/** 📞️ Every call in `tokens[start..end]`: an identifier followed by `(` or by a turbofish `::<`. */
function rustCalls(tokens: readonly RustToken[], start: number, end: number): string[] {
  const calls: string[] = [];
  for (let index = start; index < end; index += 1) {
    const token = tokens[index]!;
    if (token.kind === "ident" && (tokens[index + 1]?.text === "(" || (tokens[index + 1]?.text === ":" && tokens[index + 2]?.text === ":" && tokens[index + 3]?.text === "<"))) calls.push(token.text);
  }
  return calls;
}

/** 🪆️ Whether `sources` define `struct <snapshot> { … }` with a `#[child(…)]` field — a composed parent. */
export function composedSnapshot(sources: readonly { readonly path: string; readonly source: string }[], snapshot: string): boolean {
  return sources.some(({ source }) => {
    if (!source.includes(snapshot) || !source.includes("child")) return false;
    const tokens = rustLex(source);
    return tokens.some((token, index) => {
      if (token.text !== "struct" || tokens[index + 1]?.text !== snapshot) return false;
      let open = index + 2;
      while (open < tokens.length && tokens[open]!.text !== "{" && tokens[open]!.text !== ";") open += 1;
      if (tokens[open]?.text !== "{") return false;
      const close = rustGroupEnd(tokens, open);
      return tokens.slice(open, close).some((part, at) => part.text === "child" && tokens[open + at - 1]?.text === "[" && tokens[open + at - 2]?.text === "#" && tokens[open + at + 1]?.text === "(");
    });
  });
}

/** 🪆️ `<tree>\u0000<Snapshot>` for every composed parent snapshot struct (a `#[child(…)]` field) under `roots`, keyed by its owning tree. */
function composedParents(repoRoot: string, roots: readonly string[]): Set<string> {
  const found = new Set<string>();
  for (const root of roots) {
    for (const { path, source } of rustSources(repoRoot, root, (text) => text.includes("#[child("))) {
      if (path.split("/").includes("🧪️tests")) continue;
      for (const match of source.matchAll(/\bstruct\s+(\w+)/gu)) if (composedSnapshot([{ path, source }], match[1]!)) found.add(`${mutationSchemaSearchRoot(repoRoot, path)}\u0000${match[1]}`);
    }
  }
  return found;
}

/**
 * 🪆️ The parent-lane leaves of a composed parent that read its owned child's content (design §20.15: composed content is edited only on
 * the child lane, parent-lane leaves never read `local_owner`, readers compose on read). `sources` are the owning tree's Rust sources;
 * a free function or inherent method reads the child when its body calls a {@link CHILD_CONTENT_READS} accessor or, transitively, another
 * such function (by name; a name any trait impl of the tree also defines is ambiguous across types, so it never propagates); every leaf directory of
 * `leafDirectories` whose non-test sources make such a call is answered with the first call it makes.
 */
export function composedLeafChildReads(sources: readonly { readonly path: string; readonly source: string }[], leafDirectories: readonly string[]): { readonly directory: string; readonly call: string }[] {
  const production = sources.filter(({ path }) => !path.split("/").includes("🧪️tests"));
  const lexed = production.map(({ path, source }) => ({ path, tokens: rustLex(source) }));
  const functions: { name: string; calls: string[] }[] = [];
  const traitMethods = new Set<string>();
  for (const { tokens } of lexed) {
    const traitBodies = rustImplBlocks(tokens).map((block) => [block.open, block.close] as const);
    for (let index = 0; index + 1 < tokens.length; index += 1) {
      if (tokens[index]!.text !== "fn" || tokens[index + 1]!.kind !== "ident") continue;
      if (traitBodies.some(([open, close]) => index > open && index < close)) {
        traitMethods.add(tokens[index + 1]!.text);
        continue;
      }
      let open = index + 2;
      let depth = 0;
      while (open < tokens.length && !(depth === 0 && (tokens[open]!.text === "{" || tokens[open]!.text === ";"))) {
        if (tokens[open]!.text === "(" || tokens[open]!.text === "<") depth += 1;
        else if (tokens[open]!.text === ")" || (tokens[open]!.text === ">" && tokens[open - 1]?.text !== "-")) depth -= 1;
        open += 1;
      }
      if (tokens[open]?.text !== "{") continue;
      functions.push({ name: tokens[index + 1]!.text, calls: rustCalls(tokens, open, rustGroupEnd(tokens, open)) });
    }
  }
  const readers = new Set(CHILD_CONTENT_READS);
  for (let grown = true; grown; ) {
    grown = false;
    for (const { name, calls } of functions) {
      if (!readers.has(name) && !traitMethods.has(name) && calls.some((call) => readers.has(call))) {
        readers.add(name);
        grown = true;
      }
    }
  }
  const found: { directory: string; call: string }[] = [];
  for (const directory of leafDirectories) {
    const call = lexed.filter(({ path }) => path.startsWith(`${directory}/`)).flatMap(({ tokens }) => rustCalls(tokens, 0, tokens.length)).find((name) => readers.has(name));
    if (call !== undefined) found.push({ directory, call });
  }
  return found;
}

/**
 * ✏️ Enumerates every mutation aggregate under `under` (design §16.3) and decides, from source alone, which history mutations the
 * generic editor can edit: each leaf of a `#[derive(Mutations)]` aggregate is `editable` unless its descriptor declares
 * `"editable": false` (`inert`: withdraw-only by declaration, design §22.20 — counted, never the subject of an editability finding) or
 * composes a plan (`foreign`, the `may_emit_foreign_steps` capability), a payload-marked leaf adds its inert phases; a generic aggregate gets no emitted
 * payload law (`aggregateGeneric`), a variant without a leaf descriptor is `leafUnresolved`, and a hand-written `impl Mutation` must
 * declare its reason (`mutationHandwrittenReason`) or is `aggregateHandwritten`. `roots` bounds the tree read (the source roots by
 * default); a narrower root also bounds the app-document scan to `under`.
 */
export function mutationEditabilityReport(repoRoot: string, under = "", roots: readonly string[] = MUTATION_TREE_ROOTS): MutationEditabilityReport {
  const tree = mutationTree(repoRoot, roots);
  const diagnostics: SchemaDiagnostic[] = [];
  const census = new Map<string, MutationEditabilityCensusRow>();
  const rowOf = (owner: string): MutationEditabilityCensusRow => {
    const row = census.get(owner) ?? { owner, aggregates: 0, leaves: 0, editable: 0, foreign: 0, inert: 0, withdrawOnly: 0, handwritten: 0, findings: 0, refused: {} };
    census.set(owner, row);
    return row;
  };
  const refuse = (row: MutationEditabilityCensusRow, kind: string, path: string, detail: string): void => {
    row.findings += 1;
    row.refused[kind] = (row.refused[kind] ?? 0) + 1;
    diagnostics.push({ code: "schema-mutation-editability", scope: null, export: null, format: null, path, detail: `${kind}: ${detail}` });
  };
  const leaves: MutationLeafEditability[] = [];
  const rootIndexes = new Map<string, Map<string, string>>();
  let frameworkIndex: Map<string, string> | undefined;
  let composedSnapshots: Set<string> | undefined;
  const treeSources = new Map<string, { readonly path: string; readonly source: string }[]>();
  for (const aggregate of tree.aggregates) {
    if (!aggregate.path.startsWith(under)) continue;
    const root = aggregate.path.slice(0, aggregate.path.lastIndexOf("/"));
    const owner = mutationPayloadOwner(aggregate.path);
    const row = rowOf(owner);
    row.aggregates += 1;
    if (new RegExp(`enum\\s+${aggregate.name}\\s*<`, "u").test(readFileSync(join(repoRoot, aggregate.path), "utf8"))) refuse(row, "aggregateGeneric", aggregate.path, `${aggregate.name} is generic, so #[derive(Mutations)] emits no payload law for it`);
    for (const variant of aggregate.variants.keys()) {
      const candidates = tree.leaves.filter((candidate) => candidate.variant === variant);
      const closest = Math.max(0, ...candidates.map((candidate) => sharedSegments(candidate.directory, root)));
      const leaf = candidates.find((candidate) => candidate.root === root) ?? candidates.find((candidate) => closest >= 4 && sharedSegments(candidate.directory, root) === closest);
      if (leaf === undefined) {
        refuse(row, "leafUnresolved", aggregate.path, `${aggregate.name}::${variant} names no leaf descriptor near ${root}`);
        continue;
      }
      const descriptor = readJsonObject(repoRoot, `${leaf.directory}/🔣️.json`);
      const leafSourcePath = join(repoRoot, leaf.directory, "🦀️.rs");
      const composite = descriptor?.composition === "composite" && existsSync(leafSourcePath) && /fn\s+may_emit_foreign_steps\s*\(\s*&self\s*\)\s*->\s*bool\s*\{\s*true\b/u.test(readFileSync(leafSourcePath, "utf8"));
      const withdrawOnly = descriptor?.editable === false;
      const wrapper = tree.wrappers.get(leaf.directory)?.find((candidate) => candidate.name === aggregate.payloadTypes.get(variant));
      const inert = wrapper === undefined ? [] : [...wrapper.variants.keys()].filter((phase) => phase !== wrapper.payloadVariant);
      leaves.push({ owner, aggregate: aggregate.name, path: leaf.directory, kind: leaf.kind, variant, verdict: withdrawOnly ? "inert" : composite ? "foreign" : "editable", inert });
      row.leaves += 1;
      row.inert += inert.length;
      if (withdrawOnly) {
        row.withdrawOnly += 1;
        row.inert += 1;
        continue;
      }
      const rootIndex = new Map<string, string>();
      for (const searchRoot of mutationSchemaSearchRoots(repoRoot, leaf.schemaPath)) {
        const index = rootIndexes.get(searchRoot) ?? mutationSchemaDocumentIndex(repoRoot, searchRoot);
        rootIndexes.set(searchRoot, index);
        for (const [id, path] of index) if (!rootIndex.has(id)) rootIndex.set(id, path);
      }
      frameworkIndex ??= new Map(["🧰️framework", "🌎️hub"].flatMap((root) => [...mutationSchemaDocumentIndex(repoRoot, root)]));
      const unpublished = mutationLeafUnpublishedReferences(repoRoot, leaf.schemaPath, rootIndex, frameworkIndex);
      if (unpublished.length > 0) refuse(row, "leafReferenceUnpublished", leaf.schemaPath, `${leaf.kind}'s payload schema references ${unpublished.join(", ")}, which neither its own tree nor a plugin its crate depends on (published beside the leaf) nor a framework scope holds, so the history editor cannot resolve it`);
      if (composite) row.foreign += 1;
      else row.editable += 1;
    }
    const snapshot = new RegExp(`#\\[mutations\\([^\\]]*?snapshot\\s*=\\s*(\\w+)[^\\]]*\\][\\s\\S]{0,800}?enum\\s+${aggregate.name}\\b`, "u").exec(readFileSync(join(repoRoot, aggregate.path), "utf8"))?.[1];
    const owningTree = mutationSchemaSearchRoot(repoRoot, aggregate.path);
    if (snapshot !== undefined && (composedSnapshots ??= composedParents(repoRoot, roots)).has(`${owningTree}\u0000${snapshot}`)) {
      const sources = treeSources.get(owningTree) ?? rustSources(repoRoot, owningTree, () => true);
      treeSources.set(owningTree, sources);
      const directories = leaves.filter((candidate) => candidate.aggregate === aggregate.name && candidate.path.startsWith(owningTree)).map((candidate) => candidate.path);
      for (const { directory, call } of composedLeafChildReads(sources, directories)) refuse(row, "parentLeafReadsChild", directory, `${directory.split("/").at(-1)} is a parent-lane leaf of the composed parent ${snapshot} and reads its owned child's content through ${call} — composed content is edited only on the child lane (design §20.15)`);
    }
  }
  const handwritten: MutationHandwrittenAggregate[] = [];
  const appMutations = new Set<string>();
  const pending: { path: string; source: string; block: RustImplBlock; body: RustToken[] }[] = [];
  for (const { path, source } of rustSources(repoRoot, roots === MUTATION_TREE_ROOTS ? "" : under, (text) => !text.includes("quote!") && /\bimpl\b[^{;]*\bMutation\b/u.test(text))) {
    const tokens = rustLex(source);
    for (const block of rustImplBlocks(tokens)) {
      const body = tokens.slice(block.open, block.close + 1);
      if (MUTATION_LABEL_APP_TRAITS.has(block.trait))
        for (let index = 0; index + 3 < body.length; index += 1) {
          if (body[index]!.text !== "type" || body[index + 1]!.text !== "Mutation" || body[index + 2]!.text !== "=") continue;
          const end = body.findIndex((token, at) => at > index && token.text === ";");
          const head = body.slice(index + 3, end).findIndex((token) => token.text === "<");
          const named = body.slice(index + 3, head < 0 ? end : index + 3 + head).filter((token) => token.kind === "ident");
          if (named.length > 0) appMutations.add(named.at(-1)!.text);
        }
      if (block.trait === "Mutation" && path.startsWith(under)) pending.push({ path, source, block, body });
    }
  }
  for (const { path, source, block, body } of pending) {
    const owner = mutationPayloadOwner(path);
    const declared = mutationHandwrittenReason(path, block.implementor, block.argument, body, source);
    const reason = declared === "aggregateHandwritten" && !appMutations.has(block.implementor) ? "unexposed" : declared;
    handwritten.push({ owner, path, name: block.implementor, snapshot: block.argument, reason });
    const row = rowOf(owner);
    row.handwritten += 1;
    if (reason === "aggregateHandwritten") refuse(row, reason, path, `impl Mutation<${block.argument}> for ${block.implementor} is an app document aggregate that is neither derived nor forwards ${MUTATION_PAYLOAD_ACCESSORS.join(", ")}, so the history editor cannot edit its operations and nothing declares why`);
  }
  return { diagnostics, census: [...census.values()].sort((left, right) => right.findings - left.findings || right.foreign - left.foreign || left.owner.localeCompare(right.owner)), leaves, handwritten };
}

/**
 * ✏️ `test schema mutation-editability` — the `schema-mutation-editability` gate: every mutation aggregate is editable by the generic
 * history editor or declares why not. `--census` prints the per-plugin table and always exits 0; `--json` adds every leaf verdict and
 * hand-written aggregate (the non-editable list); without either any finding fails.
 *
 *   bun 📜️script.ts schema mutation-editability [--census] [--under <path>] [--json]
 */
export function runMutationEditability(repoRoot: string, segments: string[]): void {
  const under = segments.includes("--under") ? (segments[segments.indexOf("--under") + 1] ?? "") : "";
  const report = mutationEditabilityReport(repoRoot, under);
  const census = segments.includes("--census");
  if (segments.includes("--json")) {
    console.log(JSON.stringify(census ? report.census : report, null, 2));
    {process.exitCode=census || report.diagnostics.length === 0 ? 0 : 1;return;}
  }
  const total = report.census.reduce((sum, row) => ({ aggregates: sum.aggregates + row.aggregates, leaves: sum.leaves + row.leaves, editable: sum.editable + row.editable, foreign: sum.foreign + row.foreign, inert: sum.inert + row.inert, withdrawOnly: sum.withdrawOnly + row.withdrawOnly, handwritten: sum.handwritten + row.handwritten }), { aggregates: 0, leaves: 0, editable: 0, foreign: 0, inert: 0, withdrawOnly: 0, handwritten: 0 });
  if (census) {
    const classes = [...new Set(report.census.flatMap((row) => Object.keys(row.refused)))].sort();
    console.log(["owner", "aggregates", "leaves", "editable", "foreign", "inert", "withdrawOnly", "handwritten", "findings", ...classes].join("\t"));
    for (const row of report.census) console.log([row.owner, row.aggregates, row.leaves, row.editable, row.foreign, row.inert, row.withdrawOnly, row.handwritten, row.findings, ...classes.map((name) => row.refused[name] ?? 0)].join("\t"));
  } else for (const entry of report.diagnostics.slice(0, 40)) console.log(`[schema mutation-editability]   ${entry.path} — ${entry.detail}`);
  console.log(`[schema mutation-editability] ${total.editable}/${total.leaves} leaves of ${total.aggregates} aggregates editable (${total.foreign} composite with foreign-step capability, ${total.withdrawOnly} withdraw-only by declaration, ${total.inert - total.withdrawOnly} inert phase(s)); ${total.handwritten} hand-written aggregate(s); ${report.diagnostics.length} schema-mutation-editability finding(s)${under === "" ? "" : ` under ${under}`}`);
  {process.exitCode=census || report.diagnostics.length === 0 ? 0 : 1;return;}
}
//#endregion ✏️MutationEditability
