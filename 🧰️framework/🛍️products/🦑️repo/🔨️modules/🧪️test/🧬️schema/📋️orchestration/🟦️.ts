import { type FeatureStep, type SchemaDiagnostic, type SchemaFixtureReport, discoverSchemaFixtures, parseFeature, readSchemaCatalog, runSchemaFixture, schemaContractDiagnostics } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { Script } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { type InputSchemaAudit, mutationInputAudit, mutationInputInstance } from "../../../../../../🔨️modules/🛂️manifest/🟦️.ts";
import { parseSchemaInvariants } from "../../../../../../🔨️modules/🛂️manifest/🧬️schema/🟦️.ts";
import { type Dirent, existsSync, readdirSync, readFileSync } from "node:fs";
import { Validator } from "jsonschema";
import { join } from "node:path";

//#region 📚️SchemaDocuments
/** 📄️ The JSON object at the repository-relative `path`, or `null` when it is absent or not a JSON object. */
function readJsonObject(repoRoot: string, path: string): Record<string, unknown> | null {
  try {
    const value: unknown = JSON.parse(readFileSync(join(repoRoot, path), "utf8"));
    return value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : null;
  } catch {
    return null;
  }
}

/** 📚️ Every schema document under a catalogued scope directory, keyed by its `$id` — the cross-document `$ref` universe of the mutation lints. */
function catalogSchemaDocuments(repoRoot: string): Map<string, Record<string, unknown>> {
  const { catalog } = readSchemaCatalog(repoRoot);
  const documents = new Map<string, Record<string, unknown>>();
  const scopePaths = new Set(Object.values(catalog?.scopes ?? {}).map((scope) => scope.path));
  const index = (directory: string): void => {
    for (const entry of readdirSync(join(repoRoot, directory), { withFileTypes: true })) {
      const path = `${directory}/${entry.name}`;
      if (entry.isDirectory() && !scopePaths.has(path)) index(path);
      else if (entry.isFile() && entry.name.endsWith(".json")) {
        const document = readJsonObject(repoRoot, path);
        if (typeof document?.$id === "string") documents.set(document.$id, document);
      }
    }
  };
  for (const path of scopePaths) if (existsSync(join(repoRoot, path))) index(path);
  return documents;
}
//#endregion 📚️SchemaDocuments

/**
 * 🧭️ Adds to `documents` every schema an already loaded document `$ref`s by an absolute `$id` the catalog does not hold — the
 * uncatalogued component schemas of fixture and test aggregates (`🧫️fixtures/…/🧬️schema/🔣️.json`) — found by `$id` among the
 * `🧬️schema` JSON files of the mutation tree, transitively. The files are read only when a reference is missing.
 */
function resolveReferencedSchemaDocuments(repoRoot: string, documents: Map<string, Record<string, unknown>>, schemaFiles: readonly string[]): void {
  let index: Map<string, Record<string, unknown>> | null = null;
  const pending = [...documents.values()];
  const references = (node: unknown, found: Set<string>): Set<string> => {
    if (Array.isArray(node)) for (const item of node) references(item, found);
    else if (isRecord(node)) for (const [key, value] of Object.entries(node)) key === "$ref" && typeof value === "string" && /^https?:\/\//u.test(value) ? found.add(value.split("#")[0]!) : references(value, found);
    return found;
  };
  while (pending.length > 0) {
    for (const id of references(pending.pop(), new Set())) {
      if (documents.has(id)) continue;
      index ??= new Map(schemaFiles.flatMap((path) => {
        const document = readJsonObject(repoRoot, path);
        return typeof document?.$id === "string" ? [[document.$id, document] as const] : [];
      }));
      const document = index.get(id);
      if (document === undefined) continue;
      documents.set(id, document);
      pending.push(document);
    }
  }
}

//#region 🎛️MutationInputUi
/** 📊️ One plugin's share of the mutation-input census: leaves read, top-level inputs, those whose whole subtree reads clean
 * (`declared`), every finding at every nested pointer, and the findings per error class. */
export type MutationInputCensusRow = { readonly owner: string; leaves: number; inputs: number; declared: number; findings: number; readonly refused: Record<string, number> };

/** 🎛️ The `schema-mutation-input-ui` lint and its census, over every catalogued mutation leaf under `scope`. */
export type MutationInputUiReport = { readonly diagnostics: readonly SchemaDiagnostic[]; readonly census: readonly MutationInputCensusRow[] };

/** 🗂️ The owner a scope id is counted under: the plugin of an `s.`/`app.` scope, else the scope's first segment. */
const mutationInputCensusOwner = (scope: string): string => {
  const [head, second] = scope.split(".");
  return (head === "s" || head === "app") && second !== undefined ? second : (head ?? scope);
};

/** 🗂️ The owner an uncatalogued leaf directory is counted under: its plugin (`✏️s/🔌️plugins/<plugin>/…`), else its product or
 * framework module — the name with its taxonomy emoji stripped. */
const mutationLeafDirectoryOwner = (path: string): string => (path.split("/")[2] ?? path).replace(/^[^\p{L}\p{N}]+/u, "");

/** 🧬️ Whether a catalogued scope is a mutation leaf: a `mutation-leaf` owner, or a product/framework module sitting at a leaf
 * position (`…/🧬️mutations/<leaf>/🧬️schema`, e.g. `os.store.mutation.*`). */
const isMutationLeafScope = (scope: { readonly path: string; readonly level?: string }): boolean => scope.level === "mutation-leaf" || /\/🧬️mutations\/[^/]+\/🧬️schema$/u.test(scope.path);

/** 🔎️ Every mutation leaf schema module on disk under `under` (`…/🧬️mutations/<leaf>/🧬️schema` holding `🔣️.json`), outside
 * fixtures, tests and build output — the population a leaf whose `$id` misses the catalogue silently drops out of. */
function mutationLeafSchemaDirectories(repoRoot: string, under: string): string[] {
  const found: string[] = [];
  const skipped = new Set(["node_modules", "target", "dist", "🧫️fixtures", "🧪️tests", "🗑️generated"]);
  const walk = (directory: string): void => {
    let entries: Dirent[];
    try {
      entries = readdirSync(join(repoRoot, directory), { withFileTypes: true });
    } catch {
      return;
    }
    for (const entry of entries) {
      if (!entry.isDirectory() || skipped.has(entry.name) || entry.name.startsWith(".")) continue;
      const path = `${directory}/${entry.name}`;
      if (entry.name === "🧬️mutations") {
        for (const leaf of readdirSync(join(repoRoot, path), { withFileTypes: true })) {
          if (leaf.isDirectory() && existsSync(join(repoRoot, path, leaf.name, "🧬️schema", "🔣️.json"))) found.push(`${path}/${leaf.name}/🧬️schema`);
        }
      }
      walk(path);
    }
  };
  for (const root of ["✏️s", "🧰️framework"]) if (under === "" || root.startsWith(under) || under.startsWith(root)) walk(root);
  return found.filter((path) => path.startsWith(under)).sort();
}

/**
 * 🎛️ Audits every catalogued mutation leaf's payload schema with the framework's collecting reader `mutationInputAudit`
 * (the TypeScript twin of `manifest::mutation_input_audit`), resolving cross-document `$ref`s through the catalog's own
 * documents: every finding at every nested pointer is one `schema-mutation-input-ui` diagnostic naming the reader's error
 * class and the input pointer; a reader fault (any throw that is not an `InputSchemaError`) is the finding `readerFault`
 * for that leaf, never a crash of the lint. A top-level input counts as declared when no finding lies in its subtree.
 */
export function mutationInputUiReport(repoRoot: string, under = ""): MutationInputUiReport {
  const { catalog } = readSchemaCatalog(repoRoot);
  const documents = catalogSchemaDocuments(repoRoot);
  const read = (path: string): Record<string, unknown> | null => readJsonObject(repoRoot, path);
  const diagnostics: SchemaDiagnostic[] = [];
  const census = new Map<string, MutationInputCensusRow>();
  for (const [scopeId, scope] of Object.entries(catalog?.scopes ?? {}).sort(([left], [right]) => left.localeCompare(right))) {
    if (!isMutationLeafScope(scope) || !scope.path.startsWith(under)) continue;
    const path = `${scope.path}/${scope.formats["🔣️jsonschema"] ?? "🔣️.json"}`;
    const leaf = read(path);
    const owner = mutationInputCensusOwner(scopeId);
    const row = census.get(owner) ?? { owner, leaves: 0, inputs: 0, declared: 0, findings: 0, refused: {} };
    census.set(owner, row);
    row.leaves += 1;
    const refuse = (code: string, pointer: string, detail: string): void => {
      row.findings += 1;
      row.refused[code] = (row.refused[code] ?? 0) + 1;
      diagnostics.push({ code: "schema-mutation-input-ui", scope: scopeId, export: null, format: "🔣️jsonschema", path, detail: `${code} at ${JSON.stringify(pointer)}: ${detail}` });
    };
    if (leaf === null) {
      refuse("malformed", "", `${path} is not JSON`);
      continue;
    }
    let audit: InputSchemaAudit;
    try {
      audit = mutationInputAudit(leaf, (id) => documents.get(id));
    } catch (error) {
      refuse("readerFault", "", error instanceof Error ? `${error.name}: ${error.message}` : String(error));
      continue;
    }
    const top = (pointer: string): string | undefined => (pointer === "" ? undefined : pointer.slice(1).split("/")[0]);
    const refusedTops = new Set(audit.findings.map((finding) => top(finding.pointer)).filter((key): key is string => key !== undefined));
    const tops = new Set([...audit.inputs.map((input) => top(input.id)).filter((key): key is string => key !== undefined), ...refusedTops]);
    row.inputs += tops.size;
    row.declared += [...tops].filter((key) => !refusedTops.has(key)).length;
    for (const finding of audit.findings) refuse(finding.code, finding.pointer, finding.message);
  }
  const catalogued = new Set(Object.values(catalog?.scopes ?? {}).map((scope) => scope.path));
  for (const directory of mutationLeafSchemaDirectories(repoRoot, under)) {
    if (catalogued.has(directory)) continue;
    const owner = mutationLeafDirectoryOwner(directory);
    const row = census.get(owner) ?? { owner, leaves: 0, inputs: 0, declared: 0, findings: 0, refused: {} };
    census.set(owner, row);
    row.leaves += 1;
    row.findings += 1;
    row.refused.leafUncatalogued = (row.refused.leafUncatalogued ?? 0) + 1;
    const path = `${directory}/🔣️.json`;
    const id = read(path)?.$id;
    diagnostics.push({ code: "schema-mutation-input-ui", scope: null, export: null, format: "🔣️jsonschema", path, detail: `leafUncatalogued at "": $id ${JSON.stringify(id ?? null)} names no catalogued mutation-leaf scope, so neither the reader nor this census reads ${directory}` });
  }
  return { diagnostics, census: [...census.values()].sort((left, right) => right.inputs - right.declared - (left.inputs - left.declared) || left.owner.localeCompare(right.owner)) };
}

/**
 * 🎛️ `test schema mutation-inputs` — the `schema-mutation-input-ui` gate: every mutation input carries a UI descriptor
 * (label in every locale, valid `x-semio-ui`, a widget its value can take). `--census` prints the per-plugin table and
 * always exits 0 — the rollout tracker; without it any finding fails.
 *
 *   bun 📜️script.ts schema mutation-inputs [--census] [--under <path>] [--json]
 */
function runMutationInputUi(repoRoot: string, segments: string[]): never {
  const under = segments.includes("--under") ? (segments[segments.indexOf("--under") + 1] ?? "") : "";
  const report = mutationInputUiReport(repoRoot, under);
  const census = segments.includes("--census");
  if (segments.includes("--json")) {
    console.log(JSON.stringify(census ? report.census : report, null, 2));
    process.exit(census || report.diagnostics.length === 0 ? 0 : 1);
  }
  const total = report.census.reduce((sum, row) => ({ leaves: sum.leaves + row.leaves, inputs: sum.inputs + row.inputs, declared: sum.declared + row.declared }), { leaves: 0, inputs: 0, declared: 0 });
  if (census) {
    const codes = [...new Set(report.census.flatMap((row) => Object.keys(row.refused)))].sort();
    console.log(["owner", "leaves", "inputs", "declared", "missing", "findings", ...codes].join("\t"));
    for (const row of report.census) console.log([row.owner, row.leaves, row.inputs, row.declared, row.inputs - row.declared, row.findings, ...codes.map((code) => row.refused[code] ?? 0)].join("\t"));
  } else for (const entry of report.diagnostics.slice(0, 40)) console.log(`[schema mutation-inputs]   ${entry.scope} — ${entry.detail}`);
  console.log(`[schema mutation-inputs] ${total.declared}/${total.inputs} input(s) of ${total.leaves} leaves carry a UI descriptor; ${report.diagnostics.length} schema-mutation-input-ui finding(s)${under === "" ? "" : ` under ${under}`}`);
  process.exit(census || report.diagnostics.length === 0 ? 0 : 1);
}
//#endregion 🎛️MutationInputUi

//#region ⚖️MutationPayloadParity
/** 🧬️ The wire layout of a `#[derive(Mutations)]` aggregate enum — its `#[value(tag, content, rename_all)]` container attribute. */
export type MutationAggregateLayout = Readonly<{ tag: string | null; content: string | null; renameAll: string | null }>;

/** 🗂️ One `#[derive(Mutations)]` aggregate enum: its file, name, wire layout, and every variant ident with its `#[value(rename)]`. */
export type MutationAggregate = Readonly<{ path: string; name: string; layout: MutationAggregateLayout; variants: ReadonlyMap<string, string | null>; payloadTypes: ReadonlyMap<string, string> }>;

/** 🦀️ One enum of a Rust source with its `#[value(...)]` wire attributes: an aggregate when it derives `Mutations`, a wrapped leaf when
 * its `#[mutation_leaf(payload = <Variant>)]` names the variant whose content is the schema-described, editable payload. */
export type RustValueEnum = MutationAggregate & Readonly<{ mutations: boolean; payloadVariant: string | null }>;

/** 🩺️ The classes of a `schema-mutation-payload-parity` finding. */
export type MutationPayloadFindingClass = "invalid" | "undescribed" | "opaque" | "layout" | "aggregate" | "unmapped" | "unresolved" | "unwitnessed" | "negative";

/**
 * 🚫️ How a committed fixture witnesses its leaf schema. `positive`: the wire must satisfy the schema. `negative`: its committed
 * outcome (`<case>/🎯️outcome/🔣️.json`) is a refusal with `mutation.invariant`, a payload the domain never admits; `invariant` is the
 * id the outcome names for a payload-intrinsic rule JSON Schema cannot express (declared in the leaf's `x-semio-invariant`).
 */
export type MutationFixtureOutcome = Readonly<{ direction: "positive" | "negative"; invariant: string | null }>;

/** 🚫️ The witness direction of a committed outcome document: `negative` for `{"status": "rejected", "code": "mutation.invariant"}`. */
export function mutationFixtureOutcome(outcome: unknown): MutationFixtureOutcome {
  const negative = isRecord(outcome) && outcome.status === "rejected" && outcome.code === "mutation.invariant";
  return { direction: negative ? "negative" : "positive", invariant: negative && typeof outcome.invariant === "string" ? outcome.invariant : null };
}

/**
 * 🚫️ The verdict on a negative witness: clean when its leaf schema rejects it (`invalid`), or when it is schema-valid but its
 * outcome names an invariant the leaf declares in `x-semio-invariant` (`declared`, the ids of `mutationSchemaInvariants`); else
 * one `negative` finding. Never `invalid`.
 */
export function mutationNegativeVerdict(payload: readonly MutationPayloadFinding[], outcome: MutationFixtureOutcome, declared: ReadonlySet<string>): MutationPayloadFinding[] {
  if (payload.some((finding) => finding.class === "invalid") || (outcome.invariant !== null && declared.has(outcome.invariant))) return [];
  const detail = outcome.invariant === null ? "the committed outcome refuses this payload with mutation.invariant and names no invariant, but the leaf schema accepts it: state the rule in the schema, or declare it in x-semio-invariant and name its id in the outcome" : `the committed outcome names the invariant ${JSON.stringify(outcome.invariant)}, which the leaf schema neither enforces nor declares in x-semio-invariant (declared: ${[...declared].map((id) => JSON.stringify(id)).join(", ") || "none"})`;
  return [{ class: "negative", pointer: "", detail }];
}

/** 🧷️ The invariant ids a schema document declares in `x-semio-invariant` on any of its nodes (read by `parseSchemaInvariants`; a malformed
 * value declares nothing here — the strict vocabulary oracle reports it). */
export function mutationSchemaInvariants(document: unknown): ReadonlySet<string> {
  const ids = new Set<string>();
  const declared = (value: unknown): readonly { readonly id: string }[] => {
    try {
      return parseSchemaInvariants(value);
    } catch {
      return [];
    }
  };
  const walk = (node: unknown): void => {
    if (Array.isArray(node)) for (const item of node) walk(item);
    else if (isRecord(node))
      for (const [key, value] of Object.entries(node)) {
        if (key === "x-semio-invariant") for (const invariant of declared(value)) ids.add(invariant.id);
        else if (key !== "const" && key !== "enum" && key !== "examples" && key !== "default") walk(value);
      }
  };
  walk(document);
  return ids;
}

/** 🔎️ One parity finding: its class, an RFC 6901 pointer (into the payload, or `<document>#<pointer>` into a schema for `opaque`), and why. */
export type MutationPayloadFinding = Readonly<{ class: MutationPayloadFindingClass; pointer: string; detail: string }>;

/** 📊️ One plugin's share of the payload-parity census: leaves, committed fixtures (of them `negatives`, and of those `invariants` naming a
 * declared `x-semio-invariant`), fixtures without a finding, findings per class. */
export type MutationPayloadCensusRow = { readonly owner: string; leaves: number; witnessed: number; fixtures: number; negatives: number; invariants: number; rows: number; clean: number; readonly findings: Record<string, number> };

/** 🥒️ One wire-form row of a feature: the leaf `kind` and its wire payload `params`, at the 1-based line of its scenario. */
export type MutationFeatureRow = Readonly<{ line: number; kind: string; params: unknown }>;

/** ⚖️ The `schema-mutation-payload-parity` lint and its census. */
export type MutationPayloadParityReport = { readonly diagnostics: readonly SchemaDiagnostic[]; readonly census: readonly MutationPayloadCensusRow[] };

type RustToken = Readonly<{ kind: "ident" | "literal" | "punct"; text: string }>;
type MutationLeafRecord = Readonly<{ directory: string; root: string; variant: string; kind: string; schemaPath: string }>;
type MutationTree = { readonly leaves: MutationLeafRecord[]; readonly aggregates: MutationAggregate[]; readonly fixtures: string[]; readonly features: string[]; readonly wrappers: Map<string, RustValueEnum[]>; readonly schemas: string[] };
type SchemaNode = Readonly<{ document: string; pointer: string; node: Record<string, unknown> }>;

const MUTATION_TREE_ROOTS = ["✏️s", "🧰️framework"];
const MUTATION_TREE_SKIPPED = new Set(["node_modules", "target", "🗑️generated", ".git", "dist"]);
const isRecord = (value: unknown): value is Record<string, unknown> => value !== null && typeof value === "object" && !Array.isArray(value);
const escapePointer = (segment: string): string => segment.replaceAll("~", "~0").replaceAll("/", "~1");
const unescapePointer = (segment: string): string => segment.replaceAll("~1", "/").replaceAll("~0", "~");
const schemaKind = (name: string): string => name.replace(/^[^\p{L}\p{N}]+/u, "");
const canonicalJson = (value: unknown): string => JSON.stringify(value, (_, entry: unknown) => (isRecord(entry) ? Object.fromEntries(Object.entries(entry).sort(([left], [right]) => left.localeCompare(right))) : entry));

/** 🐫 The TypeScript twin of value_derive `variant_wire_name` (`🌱️value/✨️derive/⚙️expansion/🦀️.rs`): an enum variant's wire name. */
export function mutationVariantWireName(ident: string, rename: string | null, renameAll: string | null): string {
  if (rename !== null) return rename;
  const words = ident.split(/(?=\p{Lu})/u).map((word) => word.toLowerCase());
  if (renameAll === "camelCase") return words.map((word, index) => (index === 0 ? word : word.charAt(0).toUpperCase() + word.slice(1))).join("");
  if (renameAll === "kebab-case") return words.join("-");
  if (renameAll === "snake_case") return words.join("_");
  if (renameAll === "lowercase") return words.join("");
  return ident;
}

/**
 * 🧾️ The leaf payload `payload_value()` yields for the variant wired as `wireName`, cut out of the aggregate wire `value` that
 * `layout` encodes: the one-key wrapper of an externally tagged enum, the `content` member of an adjacently tagged one, the
 * members beside the tag of an internally tagged one — or why `value` is not that variant in that layout.
 */
export function mutationLeafPayload(layout: MutationAggregateLayout, wireName: string, value: unknown): { payload: unknown } | { refused: string } {
  if (!isRecord(value)) return { refused: "the aggregate wire value is not an object" };
  const keys = Object.keys(value);
  if (layout.tag === null) return keys.length === 1 && keys[0] === wireName ? { payload: value[wireName] } : { refused: `an externally tagged aggregate wires this variant as {${JSON.stringify(wireName)}: …}, found the keys ${JSON.stringify(keys)}` };
  if (value[layout.tag] !== wireName) return { refused: `the tag ${JSON.stringify(layout.tag)} must be ${JSON.stringify(wireName)}, found ${JSON.stringify(value[layout.tag] ?? keys)}` };
  if (layout.content === null) return { payload: Object.fromEntries(Object.entries(value).filter(([key]) => key !== layout.tag)) };
  const extra = keys.filter((key) => key !== layout.tag && key !== layout.content);
  return extra.length > 0 ? { refused: `an adjacently tagged aggregate carries only ${JSON.stringify(layout.tag)} and ${JSON.stringify(layout.content)}, found ${JSON.stringify(extra)}` } : { payload: value[layout.content] ?? null };
}

/**
 * 🧺️ The aggregate-schema branch of one variant, as the repo-wide rule derives it from the wire layout: the leaf `$ref` itself
 * (internally tagged; the leaf declares the tag `const`), `{W: $ref}` (externally tagged) or `{tag: const W, content: $ref}`
 * (adjacently tagged), each closed with `additionalProperties: false`. A wrapped leaf (`#[mutation_leaf(payload = <Variant>)]`)
 * describes its editable payload at the document root and its whole wire enum under `$defs/<wrapper>`, so the branch refers to
 * that enum: the root alone would refuse the wrapper's other variants and the variant envelope itself.
 */
export function mutationAggregateBranch(layout: MutationAggregateLayout, wireName: string, leafId: string, wrapper: string | null = null): Record<string, unknown> {
  const leaf = { $ref: wrapper === null ? leafId : `${leafId}#/$defs/${escapePointer(wrapper)}` };
  if (layout.tag !== null && layout.content === null) return leaf;
  if (layout.tag === null) return { type: "object", additionalProperties: false, required: [wireName], properties: { [wireName]: leaf } };
  return { type: "object", additionalProperties: false, required: [layout.tag, layout.content], properties: { [layout.tag]: { const: wireName }, [layout.content!]: leaf } };
}

/** 🎁️ A wrapped leaf: the leaf enum's own wire layout and the wire name of its `#[mutation_leaf(payload = <Variant>)]` variant. */
export type MutationLeafWrapper = Readonly<{ layout: MutationAggregateLayout; wireName: string }>;

/**
 * 🎁️ The editable leaf payload of one committed fixture, as `payload_value()` yields it: the aggregate wrapper is cut first
 * (`mutationLeafPayload`); a wrapped leaf then yields the content of its `payload` variant, and any other variant of it is
 * `inert` (not editable, so no witness of the leaf schema). `refused` when the fixture is not the variant in the layout.
 */
export function mutationInputPayload(layout: MutationAggregateLayout, wireName: string, value: unknown, wrapper: MutationLeafWrapper | null): { payload: unknown } | { refused: string } | { inert: string } {
  const cut = mutationLeafPayload(layout, wireName, mutationFixtureWire(value));
  if ("refused" in cut || wrapper === null) return cut;
  const inner = mutationLeafPayload(wrapper.layout, wrapper.wireName, cut.payload);
  return "payload" in inner ? inner : { inert: inner.refused };
}

/**
 * 🎬️ The step vocabulary of a mutation row: a `When` step that applies, attempts or replays the kind it names (`When the <kind>
 * mutation is applied with its parameters`, `… is attempted …`, `When <kind> is replayed against its vector …`). A `Then` step's
 * expectation docstring and a codec step (`When it is parsed … and printed back`) never apply a mutation.
 */
export function mutationFeatureStep(step: FeatureStep, kind: string): boolean {
  return step.keyword === "When" && step.text.includes(kind) && /\b(?:applied|attempted|replayed)\b/u.test(step.text);
}

/**
 * 🥒️ The wire-form rows of a `🥒️.feature`, read through the harness's own `parseFeature` (the plan every native host receives):
 * every mutation step (`mutationFeatureStep`) of an expanded scenario whose docstring is a JSON object `{"kind": <leaf kind>,
 * "params": <wire payload>, …}`, at the scenario's line. A Scenario Outline row whose template carries `"kind": "<id>"` and
 * `"params": <params>` is one such step; a round-trip, codec or expectation step carrying the same shape is not a row.
 */
export function mutationFeatureRows(source: string): MutationFeatureRow[] {
  return parseFeature(source).scenarios.flatMap((scenario) =>
    scenario.steps.flatMap((step) => {
      const value: unknown = (() => {
        try {
          return step.docString === undefined ? undefined : JSON.parse(step.docString);
        } catch {
          return undefined;
        }
      })();
      return isRecord(value) && typeof value.kind === "string" && "params" in value && mutationFeatureStep(step, value.kind) ? [{ line: scenario.line, kind: value.kind, params: value.params }] : [];
    }),
  );
}

/** ✂️ Skips whitespace, line comments and (nested) block comments of Rust source from `start`. */
function rustTrivia(source: string, start: number): number {
  let index = start;
  for (;;) {
    while (index < source.length && /\s/u.test(source[index]!)) index += 1;
    if (source.startsWith("//", index)) {
      const end = source.indexOf("\n", index);
      index = end < 0 ? source.length : end + 1;
    } else if (source.startsWith("/*", index)) {
      let depth = 0;
      do {
        if (source.startsWith("/*", index)) (depth += 1), (index += 2);
        else if (source.startsWith("*/", index)) (depth -= 1), (index += 2);
        else index += 1;
      } while (depth > 0 && index < source.length);
    } else return index;
  }
}

/** 🔤️ Rust source as identifier, literal and punctuation tokens; comments are dropped and strings, raw strings and char literals stay whole. */
function rustLex(source: string): RustToken[] {
  const tokens: RustToken[] = [];
  let index = 0;
  while ((index = rustTrivia(source, index)) < source.length) {
    const char = source[index]!;
    const raw = /^b?r(#*)"/u.exec(source.slice(index, index + 16));
    let end = index + 1;
    if (raw !== null) {
      const close = source.indexOf(`"${raw[1]}`, index + raw[0].length);
      end = close < 0 ? source.length : close + 1 + raw[1]!.length;
    } else if (char === '"' || (char === "b" && source[index + 1] === '"')) {
      end = index + (char === "b" ? 2 : 1);
      while (end < source.length && source[end] !== '"') end += source[end] === "\\" ? 2 : 1;
      end += 1;
    } else if (char === "'") {
      const width = (source.codePointAt(index + 1) ?? 0) > 0xffff ? 2 : 1;
      end = source[index + 1] === "\\" ? source.indexOf("'", index + 2) + 1 : source[index + 1 + width] === "'" ? index + 2 + width : index + 1;
    } else {
      const ident = /^[\p{L}_][\p{L}\p{N}_]*/u.exec(source.slice(index, index + 256));
      if (ident !== null) {
        tokens.push({ kind: "ident", text: ident[0] });
        index += ident[0].length;
        continue;
      }
      tokens.push({ kind: "punct", text: char });
      index += 1;
      continue;
    }
    tokens.push({ kind: "literal", text: source.slice(index, end) });
    index = end;
  }
  return tokens;
}

/** 🔚️ The index of the punctuation token closing the group opened at `open`. */
function rustGroupEnd(tokens: readonly RustToken[], open: number): number {
  let depth = 0;
  for (let index = open; index < tokens.length; index += 1) {
    const token = tokens[index]!;
    if (token.kind !== "punct") continue;
    if (token.text === "(" || token.text === "[" || token.text === "{") depth += 1;
    else if ((token.text === ")" || token.text === "]" || token.text === "}") && --depth === 0) return index;
  }
  return tokens.length - 1;
}

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

/** 🦀️ Every `#[derive(… Mutations …)]` enum of one Rust source with its `#[value(...)]` wire layout and variants, read from tokens. */
export function rustMutationAggregates(path: string, source: string): MutationAggregate[] {
  return rustValueEnums(path, source).filter((candidate) => candidate.mutations);
}

/** 🦀️ Every enum of one Rust source with its `#[value(...)]` wire layout, variants (with `#[value(rename)]`) and each tuple variant's payload type, read from tokens. */
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
function mutationTree(repoRoot: string, roots: readonly string[] = MUTATION_TREE_ROOTS): MutationTree {
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

/** 📦️ The aggregate wire value a committed `🦠️mutation/🔣️.json` holds: the whole document, or the `mutation` member of an exhaustive
 * case record `{kind, mutation, before, after}` (the stdio `mutate-*` cases). */
export function mutationFixtureWire(value: unknown): unknown {
  return isRecord(value) && isRecord(value.mutation) && "before" in value && "after" in value ? value.mutation : value;
}

/** 🗂️ The census owner of a repository path: the plugin under `🔌️plugins`, else the product under `🛍️products`, else the root. */
function mutationPayloadOwner(path: string): string {
  const segments = path.split("/");
  const plugin = segments.indexOf("🔌️plugins");
  const product = segments.indexOf("🛍️products");
  return schemaKind(plugin >= 0 ? segments[plugin + 1]! : product >= 0 ? segments[product + 1]! : segments[0]!);
}

/** 📏️ The number of leading path segments `left` and `right` share. */
function sharedSegments(left: string, right: string): number {
  const [a, b] = [left.split("/"), right.split("/")];
  let count = 0;
  while (count < a.length && count < b.length && a[count] === b[count]) count += 1;
  return count;
}

/**
 * ⚖️ The parity checks of one leaf payload schema over the `schema://` document universe, validated by the third-party npm
 * `jsonschema`: `invalid` (the payload fails its leaf schema), `undescribed` (a payload member no `properties`,
 * `patternProperties` or `additionalProperties` describes, walked through `$ref`, `allOf` and the matching `oneOf`/`anyOf`
 * branch), and `opaque` (an object node of the schema that declares no members at all).
 */
export function mutationPayloadChecker(documents: ReadonlyMap<string, Record<string, unknown>>) {
  const validator = new Validator();
  for (const [id, document] of documents) {
    try {
      validator.addSchema(document, id);
    } catch {
      validator.unresolvedRefs = [];
    }
    validator.schemas[id] = document;
  }
  const at = (document: string, pointer: string): unknown => {
    let current: unknown = documents.get(document);
    for (const segment of pointer.split("/").slice(1).map(unescapePointer)) current = isRecord(current) ? current[segment] : Array.isArray(current) && /^\d+$/u.test(segment) ? current[Number(segment)] : undefined;
    return current;
  };
  const resolve = (document: string, pointer: string): SchemaNode | null => {
    for (let hop = 0; hop < 32; hop += 1) {
      const node = at(document, pointer);
      if (!isRecord(node)) return null;
      if (typeof node.$ref !== "string") return { document, pointer, node };
      const hash = node.$ref.indexOf("#");
      const id = hash < 0 ? node.$ref : node.$ref.slice(0, hash);
      try {
        document = id === "" ? document : new URL(id, document).href;
      } catch {
        return null;
      }
      pointer = hash < 0 ? "" : decodeURIComponent(node.$ref.slice(hash + 1));
      if (!documents.has(document)) return null;
    }
    return null;
  };
  const reference = (document: string, pointer: string): string => `${document}#${pointer.split("/").slice(1).map((segment) => `/${encodeURIComponent(escapePointer(unescapePointer(segment)))}`).join("")}`;
  const valid = (instance: unknown, document: string, pointer: string): boolean => {
    try {
      return validator.validate(instance, { $ref: reference(document, pointer) }).valid;
    } catch {
      return false;
    }
  };
  const members = (instance: unknown, document: string, pointer: string, seen: Set<string>): SchemaNode[] => {
    const resolved = resolve(document, pointer);
    if (resolved === null || seen.has(`${resolved.document}#${resolved.pointer}`)) return [];
    seen.add(`${resolved.document}#${resolved.pointer}`);
    const node = resolved.node;
    const found = [resolved];
    const branch = (keyword: string, index: number | null): void => void found.push(...members(instance, resolved.document, `${resolved.pointer}/${keyword}${index === null ? "" : `/${index}`}`, seen));
    if (Array.isArray(node.allOf)) node.allOf.forEach((_, index) => branch("allOf", index));
    for (const keyword of ["oneOf", "anyOf"]) {
      if (!Array.isArray(node[keyword])) continue;
      const matching = (node[keyword] as unknown[]).map((_, index) => index).filter((index) => valid(instance, resolved.document, `${resolved.pointer}/${keyword}/${index}`));
      for (const index of keyword === "oneOf" ? matching.slice(0, 1) : matching) branch(keyword, index);
    }
    if (node.if !== undefined) {
      const chosen = valid(instance, resolved.document, `${resolved.pointer}/if`) ? "then" : "else";
      if (node[chosen] !== undefined) branch(chosen, null);
    }
    return found;
  };
  const where = (node: SchemaNode): string => `${node.document}#${node.pointer}`;
  const describe = (instance: unknown, document: string, pointer: string, path: string, findings: MutationPayloadFinding[], depth: number): void => {
    const nodes = depth > 64 ? [] : members(instance, document, pointer, new Set());
    if (nodes.length === 0) return;
    if (isRecord(instance)) {
      const declared = nodes.some(({ node }) => node.properties !== undefined || node.patternProperties !== undefined || node.additionalProperties !== undefined);
      const typed = nodes.some(({ node }) => node.type === "object" || (Array.isArray(node.type) && node.type.includes("object")));
      if (!declared) {
        if (typed) for (const key of Object.keys(instance)) findings.push({ class: "undescribed", pointer: `${path}/${escapePointer(key)}`, detail: `${where(nodes[0]!)} declares an object without members, yet the payload carries ${JSON.stringify(key)}` });
        return;
      }
      for (const [key, value] of Object.entries(instance)) {
        const child = `${path}/${escapePointer(key)}`;
        const targets: [string, string][] = [];
        for (const { document: owner, pointer: base, node } of nodes) {
          if (isRecord(node.properties) && key in node.properties) targets.push([owner, `${base}/properties/${escapePointer(key)}`]);
          if (isRecord(node.patternProperties)) for (const pattern of Object.keys(node.patternProperties)) if (new RegExp(pattern, "u").test(key)) targets.push([owner, `${base}/patternProperties/${escapePointer(pattern)}`]);
        }
        const additional = nodes.find(({ node }) => node.additionalProperties !== undefined);
        if (targets.length === 0 && additional === undefined) findings.push({ class: "undescribed", pointer: child, detail: `${JSON.stringify(key)} is no declared member of ${where(nodes[0]!)}` });
        else if (targets.length === 0 && isRecord(additional?.node.additionalProperties)) targets.push([additional.document, `${additional.pointer}/additionalProperties`]);
        for (const [owner, base] of targets) describe(value, owner, base, child, findings, depth + 1);
      }
    } else if (Array.isArray(instance))
      for (const { document: owner, pointer: base, node } of nodes) {
        if (isRecord(node.items)) instance.forEach((item, index) => describe(item, owner, `${base}/items`, `${path}/${index}`, findings, depth + 1));
        else if (Array.isArray(node.items)) instance.forEach((item, index) => describe(item, owner, index < (node.items as unknown[]).length ? `${base}/items/${index}` : `${base}/additionalItems`, `${path}/${index}`, findings, depth + 1));
      }
  };
  const opaque = (document: string, pointer: string, root: string, findings: MutationPayloadFinding[], seen: Set<string>): void => {
    const resolved = resolve(document, pointer);
    if (resolved === null || seen.has(where(resolved))) return;
    seen.add(where(resolved));
    const node = resolved.node;
    const typed = node.type === "object" || (Array.isArray(node.type) && node.type.includes("object"));
    const composed = ["allOf", "oneOf", "anyOf", "if"].some((keyword) => node[keyword] !== undefined);
    const described = node.patternProperties !== undefined || node.additionalProperties !== undefined || (isRecord(node.properties) && Object.keys(node.properties).length > 0);
    if (typed && !composed && !described) findings.push({ class: "opaque", pointer: resolved.document === root ? `#${resolved.pointer}` : where(resolved), detail: "an object schema declares no members (properties, patternProperties or additionalProperties), so its payload fields are undescribed" });
    const child = (suffix: string): void => opaque(resolved.document, `${resolved.pointer}/${suffix}`, root, findings, seen);
    for (const keyword of ["properties", "patternProperties"]) if (isRecord(node[keyword])) for (const key of Object.keys(node[keyword] as object)) child(`${keyword}/${escapePointer(key)}`);
    for (const keyword of ["additionalProperties", "additionalItems", "then", "else"]) if (isRecord(node[keyword])) child(keyword);
    if (isRecord(node.items)) child("items");
    for (const keyword of ["items", "allOf", "oneOf", "anyOf"]) if (Array.isArray(node[keyword])) (node[keyword] as unknown[]).forEach((_, index) => child(`${keyword}/${index}`));
  };
  const checks = {
    /** 🔎️ The `invalid` and `undescribed` findings of one normalized leaf payload against the leaf document `id`. */
    payload(id: string, instance: unknown): MutationPayloadFinding[] {
      const findings: MutationPayloadFinding[] = [];
      try {
        const errors = validator.validate(instance, { $ref: id }).errors;
        if (errors.length > 0) findings.push({ class: "invalid", pointer: "", detail: errors.slice(0, 4).map((error) => `${error.property.replace(/^instance/u, "") || "/"} ${error.message}`).join("; ") });
      } catch (error) {
        return [{ class: "unresolved", pointer: "", detail: error instanceof Error ? error.message : String(error) }];
      }
      describe(instance, id, "", "", findings, 0);
      return findings;
    },
    /** 🧺️ The `aggregate` finding when the aggregate document `id` rejects the whole aggregate wire value of a fixture. */
    aggregate(id: string, wire: unknown): MutationPayloadFinding[] {
      try {
        const errors = validator.validate(wire, { $ref: id }).errors;
        return errors.length === 0 ? [] : [{ class: "aggregate", pointer: "", detail: `${id} rejects the aggregate wire value: ${errors.slice(0, 2).map((error) => `${error.property.replace(/^instance/u, "") || "/"} ${error.message}`).join("; ")}` }];
      } catch (error) {
        return [{ class: "aggregate", pointer: "", detail: `${id} does not resolve: ${error instanceof Error ? error.message : String(error)}` }];
      }
    },
    /** 🧷️ The invariant ids the leaf document `id` declares in `x-semio-invariant`. */
    invariants(id: string): ReadonlySet<string> {
      return mutationSchemaInvariants(documents.get(id));
    },
    /** 🕳️ The `opaque` findings of the leaf document `id`, walked through every reachable subschema and `$ref`. */
    opaque(id: string): MutationPayloadFinding[] {
      const findings: MutationPayloadFinding[] = [];
      opaque(id, "", id, findings, new Set());
      return findings;
    },
    /**
     * 🧫️ Every finding of one committed fixture `value` of the variant wired as `wireName`: `layout` when it is not that
     * variant in `layout`, else the payload checks of the editable leaf payload (`mutationInputPayload`; root `const`s spliced
     * back by the manifest reader `mutationInputInstance`) against the leaf document `leafId` — skipped for a non-editable
     * variant of a wrapped leaf — and the aggregate check against `aggregateId`.
     */
    fixture(layout: MutationAggregateLayout, wireName: string, leafId: string, aggregateId: string | null, value: unknown, wrapper: MutationLeafWrapper | null = null, outcome: MutationFixtureOutcome = { direction: "positive", invariant: null }): MutationPayloadFinding[] {
      const wire = mutationFixtureWire(value);
      const cut = mutationInputPayload(layout, wireName, value, wrapper);
      if ("refused" in cut) return [{ class: "layout", pointer: "", detail: cut.refused }];
      if ("inert" in cut) return aggregateId === null ? [] : checks.aggregate(aggregateId, wire);
      const instance = (() => {
        try {
          return isRecord(cut.payload) ? mutationInputInstance(documents.get(leafId), (reference) => documents.get(reference), cut.payload) : cut.payload;
        } catch {
          return cut.payload;
        }
      })();
      const payload = checks.payload(leafId, instance);
      if (outcome.direction === "negative") return mutationNegativeVerdict(payload, outcome, checks.invariants(leafId));
      return [...payload, ...(aggregateId === null ? [] : checks.aggregate(aggregateId, wire))];
    },
  };
  return checks;
}

/**
 * ⚖️ Reads every committed mutation fixture (`<owner>/🧫️fixtures/🧬️mutations/<leaf>/<case>/🦠️mutation/🔣️.json` and every
 * other `…fixtures/…/🦠️mutation` or `🧪️tests/…/🦠️mutation` layout), maps it to its leaf (by the leaf directory the fixture path
 * names, else by the variant its payload names), cuts the leaf payload out of the aggregate wire value exactly as
 * `payload_value()` does (the layout read off the aggregate's `#[value(...)]` attribute), splices the leaf schema's root
 * `const`s back in with the manifest reader `mutationInputInstance`, and validates the result against the leaf payload schema.
 * The whole aggregate wire value is validated against the aggregate's own `🧬️mutations/🔣️.json` document where it exists (an
 * internally tagged aggregate's leaf schemas declare the tag `const`, so its `oneOf` of leaf `$ref`s admits the wire; an externally
 * or adjacently tagged aggregate wraps each leaf `$ref` in its variant key or content member). Every leaf schema under `under` is
 * also walked for `opaque` object nodes. The fixture is the witness of the Rust wire shape. A fixture whose committed outcome
 * (`<case>/🎯️outcome/🔣️.json`) refuses it with `mutation.invariant` is a negative witness (`mutationFixtureOutcome`): its leaf
 * schema must reject it, or — for a payload-intrinsic rule JSON Schema cannot express — its outcome must name an `invariant` the leaf
 * declares in `x-semio-invariant` (counted in `invariants`); else `negative`. It is counted in `negatives`, never reported
 * `invalid`, and never witnesses the leaf.
 */
export function mutationPayloadParityReport(repoRoot: string, under = ""): MutationPayloadParityReport {
  const tree = mutationTree(repoRoot);
  const documents = catalogSchemaDocuments(repoRoot);
  const { catalog } = readSchemaCatalog(repoRoot);
  const scopes = new Map(Object.entries(catalog?.scopes ?? {}).map(([id, scope]) => [scope.path, id]));
  const leafDocument = new Map<string, string>();
  for (const leaf of tree.leaves) {
    const document = readJsonObject(repoRoot, leaf.schemaPath);
    if (document === null) continue;
    const declared = typeof document.$id === "string" ? document.$id : null;
    const id = declared !== null && (!documents.has(declared) || JSON.stringify(documents.get(declared)) === JSON.stringify(document)) ? declared : `urn:semio:mutation-leaf:${leafDocument.size}`;
    documents.set(id, document);
    leafDocument.set(leaf.directory, id);
  }
  const aggregateDocument = new Map<string, string>();
  const aggregateFile = new Map<string, string | null>();
  for (const { path, name } of tree.aggregates) {
    const schemaPath = `${path.slice(0, path.lastIndexOf("/"))}/🔣️.json`;
    if (!aggregateFile.has(path)) {
      const document = readJsonObject(repoRoot, schemaPath);
      const id = document === null ? null : typeof document.$id === "string" && (!documents.has(document.$id) || JSON.stringify(documents.get(document.$id)) === JSON.stringify(document)) ? document.$id : `urn:semio:mutation-aggregate:${aggregateFile.size}`;
      if (document !== null && id !== null) documents.set(id, document);
      aggregateFile.set(path, id);
    }
    const id = aggregateFile.get(path);
    if (id === null || id === undefined) continue;
    const definitions = documents.get(id)?.$defs;
    aggregateDocument.set(`${path}|${name}`, isRecord(definitions) && isRecord(definitions[name]) ? `${id}#/$defs/${escapePointer(name)}` : id);
  }
  resolveReferencedSchemaDocuments(repoRoot, documents, tree.schemas);
  const checker = mutationPayloadChecker(documents);
  const kindPath = (segments: readonly string[]): string => segments.map(schemaKind).join("/");
  const leafByKind = new Map(tree.leaves.map((leaf) => [`${leaf.root}|${kindPath(leaf.directory.slice(leaf.root.length + 1).split("/"))}`, leaf]));
  const leafByDirectory = new Map(tree.leaves.map((leaf) => [leaf.directory, leaf]));
  const nearest = <T,>(items: readonly T[], path: (item: T) => string, target: string): T[] => {
    const scored = items.map((item) => [sharedSegments(path(item), target), item] as const);
    const best = Math.max(0, ...scored.map(([score]) => score));
    return scored.filter(([score]) => score === best && best > 0).map(([, item]) => item);
  };
  const witnesses = new Map<string, number>();
  const wrapperOf = (leaf: MutationLeafRecord, aggregate: MutationAggregate): MutationLeafWrapper | null => {
    const wrapper = tree.wrappers.get(leaf.directory)?.find((candidate) => candidate.name === aggregate.payloadTypes.get(leaf.variant));
    return wrapper === undefined || wrapper.payloadVariant === null ? null : { layout: wrapper.layout, wireName: mutationVariantWireName(wrapper.payloadVariant, wrapper.variants.get(wrapper.payloadVariant) ?? null, wrapper.layout.renameAll) };
  };
  const aggregateOf = (leaf: MutationLeafRecord): MutationAggregate | null => {
    const candidates = nearest(tree.aggregates.filter((aggregate) => aggregate.variants.has(leaf.variant)), (aggregate) => aggregate.path, leaf.directory);
    return candidates.length > 0 && candidates.every((candidate) => JSON.stringify(candidate.layout) === JSON.stringify(candidates[0]!.layout)) ? candidates[0]! : null;
  };
  const diagnostics: SchemaDiagnostic[] = [];
  const census = new Map<string, MutationPayloadCensusRow>();
  const row = (path: string): MutationPayloadCensusRow => {
    const owner = mutationPayloadOwner(path);
    const existing = census.get(owner) ?? { owner, leaves: 0, witnessed: 0, fixtures: 0, negatives: 0, invariants: 0, rows: 0, clean: 0, findings: {} };
    census.set(owner, existing);
    return existing;
  };
  const report = (path: string, schemaPath: string | null, findings: readonly MutationPayloadFinding[]): void => {
    const target = row(path);
    for (const finding of findings) {
      target.findings[finding.class] = (target.findings[finding.class] ?? 0) + 1;
      const scope = schemaPath === null ? null : (scopes.get(schemaPath.slice(0, schemaPath.lastIndexOf("/"))) ?? null);
      diagnostics.push({ code: "schema-mutation-payload-parity", scope, export: null, format: "🔣️jsonschema", path, detail: `${finding.class} at ${JSON.stringify(finding.pointer)}: ${finding.detail}${schemaPath === null || schemaPath === path ? "" : ` (leaf schema ${schemaPath})`}` });
    }
  };
  for (const leaf of tree.leaves) {
    if (!leaf.directory.startsWith(under)) continue;
    row(leaf.directory).leaves += 1;
    const id = leafDocument.get(leaf.directory);
    report(leaf.schemaPath, leaf.schemaPath, id === undefined ? [{ class: "unresolved", pointer: "", detail: `${leaf.schemaPath} is absent or not a JSON object` }] : checker.opaque(id));
  }
  const aggregateNode = (reference: string): Record<string, unknown> | null => {
    const [id, pointer = ""] = reference.split("#");
    let node: unknown = documents.get(id!);
    for (const segment of pointer.split("/").slice(1)) node = isRecord(node) ? node[unescapePointer(segment)] : undefined;
    return isRecord(node) ? node : null;
  };
  for (const aggregate of tree.aggregates) {
    if (!aggregate.path.startsWith(under)) continue;
    const schemaPath = `${aggregate.path.slice(0, aggregate.path.lastIndexOf("/"))}/🔣️.json`;
    const reference = aggregateDocument.get(`${aggregate.path}|${aggregate.name}`);
    const findings: MutationPayloadFinding[] = [];
    const refuse = (detail: string): void => void findings.push({ class: "aggregate", pointer: "", detail });
    const expected: Record<string, unknown>[] = [];
    for (const [variant, rename] of aggregate.variants) {
      const wireName = mutationVariantWireName(variant, rename, aggregate.layout.renameAll);
      const leaves = nearest(tree.leaves.filter((leaf) => leaf.variant === variant), (leaf) => leaf.directory, aggregate.path);
      const id = leaves.length === 1 ? leafDocument.get(leaves[0]!.directory) : undefined;
      if (leaves.length !== 1) refuse(`${aggregate.name}::${variant} has ${leaves.length} mutation leaves near ${aggregate.path}, not one`);
      else if (id === undefined || id.startsWith("urn:semio:")) refuse(`the leaf of ${aggregate.name}::${variant} (${leaves[0]!.schemaPath}) declares no unique $id`);
      else {
        const tag = aggregate.layout.tag;
        const properties = documents.get(id)?.properties;
        if (tag !== null && aggregate.layout.content === null && !(isRecord(properties) && isRecord(properties[tag]) && properties[tag].const === wireName)) refuse(`the leaf of ${aggregate.name}::${variant} does not declare the tag ${JSON.stringify(tag)} as const ${JSON.stringify(wireName)}`);
        const wrapper = tree.wrappers.get(leaves[0]!.directory)?.find((candidate) => candidate.name === aggregate.payloadTypes.get(variant) && candidate.payloadVariant !== null);
        expected.push(mutationAggregateBranch(aggregate.layout, wireName, id, wrapper?.name ?? null));
      }
    }
    const node = reference === undefined ? null : aggregateNode(reference);
    if (reference === undefined) refuse(`#[derive(Mutations)] enum ${aggregate.name} has no aggregate schema ${schemaPath}`);
    else if (reference.startsWith("urn:semio:")) refuse(`${schemaPath} declares no $id`);
    else if (findings.length === 0 && canonicalJson(node?.oneOf) !== canonicalJson(expected)) refuse(`the oneOf of ${reference} is not the ${expected.length}-branch union the #[value(...)] layout of ${aggregate.name} and its leaf $ids derive`);
    report(aggregate.path, schemaPath, findings);
  }
  for (const fixture of tree.fixtures.sort()) {
    if (!fixture.startsWith(under)) continue;
    const target = row(fixture);
    target.fixtures += 1;
    const outcome = mutationFixtureOutcome(readJsonObject(repoRoot, `${fixture.slice(0, fixture.lastIndexOf("/🦠️mutation/"))}/🎯️outcome/🔣️.json`));
    if (outcome.direction === "negative") target.negatives += 1;
    const segments = fixture.split("/").slice(0, -2);
    const anchor = segments.findLastIndex((segment) => segment.endsWith("fixtures") || segment === "🧪️tests");
    const owner = segments.slice(0, anchor).join("/");
    const value: unknown = (() => {
      try {
        return JSON.parse(readFileSync(join(repoRoot, fixture), "utf8"));
      } catch {
        return undefined;
      }
    })();
    const named = segments.slice(anchor + 1).filter((segment) => segment !== "🧬️mutations");
    const windows = named.flatMap((_, start) => named.map((__, end) => named.slice(start, named.length - end)).filter((window) => window.length > 0));
    let leaf = leafByDirectory.get(owner) ?? windows.map((window) => leafByKind.get(`${owner}/🧬️schema/🧬️mutations|${kindPath(window)}`)).find((found) => found !== undefined);
    if (leaf === undefined)
      for (const aggregate of nearest(tree.aggregates, (candidate) => candidate.path, fixture))
        for (const [variant, rename] of aggregate.variants)
          if ("payload" in mutationLeafPayload(aggregate.layout, mutationVariantWireName(variant, rename, aggregate.layout.renameAll), mutationFixtureWire(value))) leaf ??= nearest(tree.leaves.filter((candidate) => candidate.variant === variant), (candidate) => candidate.directory, aggregate.path)[0];
    if (leaf === undefined) {
      report(fixture, null, [{ class: "unmapped", pointer: "", detail: value === undefined ? "the fixture is not JSON" : "no mutation leaf matches the fixture's path, and no nearby aggregate declares the variant its payload names" }]);
      continue;
    }
    const aggregate = aggregateOf(leaf);
    const id = leafDocument.get(leaf.directory);
    if (aggregate === null && id !== undefined && readJsonObject(repoRoot, `${leaf.directory}/🔣️.json`)?.composition === "composite") {
      const payload = checker.payload(id, value);
      const findings = outcome.direction === "negative" ? mutationNegativeVerdict(payload, outcome, checker.invariants(id)) : payload;
      if (outcome.direction === "negative" && outcome.invariant !== null && checker.invariants(id).has(outcome.invariant)) target.invariants += 1;
      if (outcome.direction === "positive") witnesses.set(leaf.directory, (witnesses.get(leaf.directory) ?? 0) + 1);
      if (findings.length === 0) target.clean += 1;
      report(fixture, leaf.schemaPath, findings);
      continue;
    }
    if (aggregate === null || id === undefined) {
      report(fixture, leaf.schemaPath, [{ class: aggregate === null ? "unmapped" : "unresolved", pointer: "", detail: aggregate === null ? `no single #[derive(Mutations)] aggregate near ${leaf.directory} declares the variant ${leaf.variant}` : `${leaf.schemaPath} is absent or not a JSON object` }]);
      continue;
    }
    const wireName = mutationVariantWireName(leaf.variant, aggregate.variants.get(leaf.variant) ?? null, aggregate.layout.renameAll);
    const wrapper = wrapperOf(leaf, aggregate);
    const findings = checker.fixture(aggregate.layout, wireName, id, aggregateDocument.get(`${aggregate.path}|${aggregate.name}`) ?? null, value, wrapper, outcome).map((finding) => (finding.class === "layout" ? { ...finding, detail: `${finding.detail} (${aggregate.name} in ${aggregate.path})` } : finding));
    if (outcome.direction === "negative" && outcome.invariant !== null && checker.invariants(id).has(outcome.invariant)) target.invariants += 1;
    if (outcome.direction === "positive" && "payload" in mutationInputPayload(aggregate.layout, wireName, value, wrapper)) witnesses.set(leaf.directory, (witnesses.get(leaf.directory) ?? 0) + 1);
    if (findings.length === 0) target.clean += 1;
    report(fixture, leaf.schemaPath, findings);
  }
  const standardScope = (path: string): string => {
    const segments = path.split("/");
    const standard = segments.indexOf("🏅️standards");
    return standard >= 0 && standard + 1 < segments.length ? segments.slice(0, standard + 2).join("/") : path;
  };
  const aggregatePathById = new Map([...aggregateFile].flatMap(([path, id]) => (id === null ? [] : [[id, path] as const])));
  const reexportedVocabularies = (owner: string): string[] => {
    const document = readJsonObject(repoRoot, `${owner}/🧬️schema/🧬️mutations/🔣️.json`);
    if (document === null || document.oneOf !== undefined || !Array.isArray(document.allOf)) return [];
    return document.allOf.flatMap((branch) => (isRecord(branch) && typeof branch.$ref === "string" ? [aggregatePathById.get(branch.$ref)] : [])).filter((path): path is string => path !== undefined);
  };
  for (const feature of tree.features.sort()) {
    if (!feature.startsWith(under)) continue;
    const owner = feature.slice(0, feature.indexOf("/🧪️tests/"));
    const scopes = [owner, ...reexportedVocabularies(owner)].map(standardScope);
    for (const entry of mutationFeatureRows(readFileSync(join(repoRoot, feature), "utf8"))) {
      const target = row(feature);
      target.rows += 1;
      const where = `row ${JSON.stringify(entry.kind)} at line ${entry.line}`;
      const leaf = nearest(tree.leaves.filter((candidate) => candidate.kind === entry.kind && scopes.some((scope) => candidate.directory.startsWith(scope))), (candidate) => candidate.directory, owner)[0];
      const id = leaf === undefined ? undefined : leafDocument.get(leaf.directory);
      if (leaf === undefined || id === undefined) {
        report(feature, null, [{ class: "unmapped", pointer: "", detail: `${where}: no mutation leaf of kind ${JSON.stringify(entry.kind)} under ${scopes.join(" or ")}` }]);
        continue;
      }
      const payload = entry.params;
      const instance = (() => {
        try {
          return isRecord(payload) ? mutationInputInstance(documents.get(id), (reference) => documents.get(reference), payload) : payload;
        } catch {
          return payload;
        }
      })();
      const findings = checker.payload(id, instance).map((finding) => ({ ...finding, detail: `${where}: ${finding.detail}` }));
      witnesses.set(leaf.directory, (witnesses.get(leaf.directory) ?? 0) + 1);
      if (findings.length === 0) target.clean += 1;
      report(feature, leaf.schemaPath, findings);
    }
  }
  for (const leaf of tree.leaves) {
    if (!leaf.directory.startsWith(under)) continue;
    if (witnesses.has(leaf.directory)) row(leaf.directory).witnessed += 1;
    else report(leaf.schemaPath, leaf.schemaPath, [{ class: "unwitnessed", pointer: "", detail: `no committed wire fixture (…fixtures/…/🦠️mutation/🔣️.json, the Rust ToValue of an aggregate op of this variant, editable payload variant for a wrapped leaf) witnesses ${leaf.directory}, so its schema is unchecked against the wire` }]);
  }
  for (const aggregate of tree.aggregates) {
    if (!aggregate.path.startsWith(under)) continue;
    const covered = [...aggregate.variants.keys()].some((variant) => nearest(tree.leaves.filter((leaf) => leaf.variant === variant), (leaf) => leaf.directory, aggregate.path).some((leaf) => witnesses.has(leaf.directory)));
    if (!covered) report(aggregate.path, null, [{ class: "unwitnessed", pointer: "", detail: `no committed wire fixture witnesses any of the ${aggregate.variants.size} variants of #[derive(Mutations)] enum ${aggregate.name}` }]);
  }
  return { diagnostics, census: [...census.values()].sort((left, right) => Object.values(right.findings).reduce((sum, count) => sum + count, 0) - Object.values(left.findings).reduce((sum, count) => sum + count, 0) || left.owner.localeCompare(right.owner)) };
}

/**
 * ⚖️ `test schema mutation-payloads` — the `schema-mutation-payload-parity` gate: every committed mutation fixture's leaf payload
 * validates against, and is fully described by, its leaf payload schema, and no leaf schema holds an opaque object node.
 * `--census` prints the per-plugin table and always exits 0; without it any finding fails.
 *
 *   bun 📜️script.ts schema mutation-payloads [--census] [--under <path>] [--json]
 */
function runMutationPayloadParity(repoRoot: string, segments: string[]): never {
  const under = segments.includes("--under") ? (segments[segments.indexOf("--under") + 1] ?? "") : "";
  const report = mutationPayloadParityReport(repoRoot, under);
  const census = segments.includes("--census");
  if (segments.includes("--json")) {
    console.log(JSON.stringify(census ? report.census : report, null, 2));
    process.exit(census || report.diagnostics.length === 0 ? 0 : 1);
  }
  const total = report.census.reduce((sum, row) => ({ leaves: sum.leaves + row.leaves, witnessed: sum.witnessed + row.witnessed, fixtures: sum.fixtures + row.fixtures + row.rows, negatives: sum.negatives + row.negatives, invariants: sum.invariants + row.invariants, clean: sum.clean + row.clean }), { leaves: 0, witnessed: 0, fixtures: 0, negatives: 0, invariants: 0, clean: 0 });
  if (census) {
    const classes = [...new Set(report.census.flatMap((row) => Object.keys(row.findings)))].sort();
    console.log(["owner", "leaves", "witnessed", "fixtures", "negatives", "invariants", "rows", "clean", ...classes].join("\t"));
    for (const row of report.census) console.log([row.owner, row.leaves, row.witnessed, row.fixtures, row.negatives, row.invariants, row.rows, row.clean, ...classes.map((name) => row.findings[name] ?? 0)].join("\t"));
  } else for (const entry of report.diagnostics.slice(0, 40)) console.log(`[schema mutation-payloads]   ${entry.path} — ${entry.detail}`);
  console.log(`[schema mutation-payloads] ${total.clean}/${total.fixtures} fixture payload(s) and wire-form feature row(s) meet their leaf schema (${total.negatives} negative witness(es) it must reject, ${total.invariants} of them by a declared x-semio-invariant); ${total.witnessed}/${total.leaves} leaves witnessed; ${report.diagnostics.length} schema-mutation-payload-parity finding(s)${under === "" ? "" : ` under ${under}`}`);
  process.exit(census || report.diagnostics.length === 0 ? 0 : 1);
}
//#endregion ⚖️MutationPayloadParity

//#region 🦀️RustSources
/** 🗂️ The roots the history gates read Rust from: the mutation roots plus the hub compositions. */
const RUST_SOURCE_ROOTS = [...MUTATION_TREE_ROOTS, "🌎️hub"];

/** 🔎️ Every `.rs` source under `under` whose text `keep` admits, repository-relative and sorted, with its text; build output, generated
 * and hidden directories are skipped. */
function rustSources(repoRoot: string, under: string, keep: (source: string) => boolean): { readonly path: string; readonly source: string }[] {
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
      } else if (entry.isFile() && entry.name.endsWith(".rs") && path.startsWith(under)) {
        const source = readFileSync(join(repoRoot, path), "utf8");
        if (keep(source)) found.push({ path, source });
      }
    }
  };
  const directory = under !== "" && existsSync(join(repoRoot, under)) && !under.endsWith(".rs");
  for (const root of directory ? [under] : RUST_SOURCE_ROOTS) if (directory || under === "" || root.startsWith(under) || under.startsWith(root)) walk(root);
  return found.sort((left, right) => left.path.localeCompare(right.path));
}

/** 🧱️ One `impl … Trait<…> for Type { … }` block of a Rust source: the trait's last path segment, its first generic argument as text,
 * the implementing type's last path segment, and the token range of its body (the braces included). */
type RustImplBlock = Readonly<{ trait: string; argument: string; implementor: string; open: number; close: number }>;

/** 🧱️ Every trait impl block of `tokens`, nested ones included. */
function rustImplBlocks(tokens: readonly RustToken[]): RustImplBlock[] {
  const blocks: RustImplBlock[] = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index]!.kind !== "ident" || tokens[index]!.text !== "impl") continue;
    let open = index + 1;
    let angle = 0;
    let forAt = -1;
    while (open < tokens.length && !(tokens[open]!.text === "{" && angle <= 0) && tokens[open]!.text !== ";") {
      const text = tokens[open]!.text;
      if (text === "<") angle += 1;
      else if (text === ">" && tokens[open - 1]?.text !== "-") angle -= 1;
      else if (text === "for" && angle === 0 && forAt < 0) forAt = open;
      open += 1;
    }
    if (forAt < 0 || tokens[open]?.text !== "{") continue;
    let traitEnd = forAt - 1;
    let argument = "";
    if (tokens[traitEnd]?.text === ">") {
      let depth = 0;
      let start = traitEnd;
      for (; start > index; start -= 1) {
        if (tokens[start]!.text === ">" && tokens[start - 1]?.text !== "-") depth += 1;
        else if (tokens[start]!.text === "<" && --depth === 0) break;
      }
      const inner = tokens.slice(start + 1, traitEnd);
      const comma = inner.findIndex((token, at) => token.text === "," && inner.slice(0, at).filter((part) => part.text === "<").length === inner.slice(0, at).filter((part) => part.text === ">").length);
      argument = (comma < 0 ? inner : inner.slice(0, comma)).map((token) => token.text).join("").replace(/^::/u, "");
      traitEnd = start - 1;
    }
    const trait = tokens[traitEnd]?.kind === "ident" ? tokens[traitEnd]!.text : "";
    const path = tokens.slice(forAt + 1, open);
    const head = path.findIndex((token) => token.text === "<" || token.text === "where");
    const named = (head < 0 ? path : path.slice(0, head)).filter((token) => token.kind === "ident");
    blocks.push({ trait, argument, implementor: named.at(-1)?.text ?? "", open, close: rustGroupEnd(tokens, open) });
  }
  return blocks;
}

/** 🔧️ Every `fn <name>` item directly inside the block `open..close`, with the token range of its body (absent for a declaration). */
function rustFunctions(tokens: readonly RustToken[], open: number, close: number): { readonly name: string; readonly start: number; readonly body: readonly [number, number] | null }[] {
  const functions: { name: string; start: number; body: readonly [number, number] | null }[] = [];
  for (let index = open + 1; index < close; index += 1) {
    const token = tokens[index]!;
    if (token.kind === "punct" && token.text === "{") {
      index = rustGroupEnd(tokens, index);
      continue;
    }
    if (token.kind !== "ident" || token.text !== "fn" || tokens[index + 1]?.kind !== "ident") continue;
    let cursor = index + 2;
    while (cursor < close && tokens[cursor]!.text !== "(") cursor += 1;
    cursor = rustGroupEnd(tokens, cursor) + 1;
    while (cursor < close && tokens[cursor]!.text !== "{" && tokens[cursor]!.text !== ";") cursor += 1;
    const body = tokens[cursor]?.text === "{" ? ([cursor, rustGroupEnd(tokens, cursor)] as const) : null;
    functions.push({ name: tokens[index + 1]!.text, start: index, body });
    index = body === null ? cursor : body[1];
  }
  return functions;
}

/** ✂️ The arguments of the call whose `(` sits at `open`: one token run per top-level comma (a trailing comma adds none). */
function rustCallArguments(tokens: readonly RustToken[], open: number): RustToken[][] {
  const close = rustGroupEnd(tokens, open);
  const args: RustToken[][] = [[]];
  let depth = 0;
  for (let index = open + 1; index < close; index += 1) {
    const token = tokens[index]!;
    if (token.kind === "punct" && "([{".includes(token.text)) depth += 1;
    else if (token.kind === "punct" && ")]}".includes(token.text)) depth -= 1;
    if (depth === 0 && token.kind === "punct" && token.text === ",") args.push([]);
    else args.at(-1)!.push(token);
  }
  return args.at(-1)!.length === 0 ? args.slice(0, -1) : args;
}

/** 🔗️ Whether `tokens[index…]` spells the path `segments` (`A::b`). */
function rustPathAt(tokens: readonly RustToken[], index: number, segments: readonly string[]): boolean {
  return segments.every((segment, offset) => tokens[index + offset * 3]?.text === segment && (offset === segments.length - 1 || (tokens[index + offset * 3 + 1]?.text === ":" && tokens[index + offset * 3 + 2]?.text === ":")));
}

/** 🔤️ The text of the first string literal of `tokens`, quotes stripped; `null` when it holds none. */
function rustFirstLiteral(tokens: readonly RustToken[]): string | null {
  const literal = tokens.find((token) => token.kind === "literal" && /^b?r?#*"/u.test(token.text));
  return literal === undefined ? null : literal.text.replace(/^b?r?#*"|"#*$/gu, "");
}
//#endregion 🦀️RustSources

//#region 🏷️MutationLabels
/** 🩺️ The classes of a `schema-mutation-label` finding: an app overriding the leaf label (`labelOverride`), a leaf label that is
 * locale-invariant data (`labelLocaleInvariant`), a locale argument that is the empty literal (`labelLocaleEmpty`), an operation's
 * text line used as a label (`labelOpText`), and a label body the gate cannot read as one of the admitted shapes (`labelUnresolved`). */
export type MutationLabelFindingClass = "labelOverride" | "labelLocaleInvariant" | "labelLocaleEmpty" | "labelOpText" | "labelUnresolved";

/** 🏷️ How one label body names its operation: `LocalizedLabel::native(en, de)` (`native`), a forward to another label (`forward`), or a finding. */
export type MutationLabelVerdict = "native" | "forward" | MutationLabelFindingClass;

/** 🏷️ One label site of a Rust source: the trait it implements (`MutationKind`, `CompositeMutationKind`, `SemanticMutation`, an app
 * trait for an override, or `data` for a `LocalizedLabel::data` call), its implementor, the verdict and every `[en, de]` template read. */
export type MutationLabelSite = Readonly<{ path: string; trait: string; implementor: string; verdict: MutationLabelVerdict; texts: readonly (readonly [string | null, string | null])[] }>;

/** 📊️ One plugin's share of the label census: label bodies read, `native` and `forward` ones, and the findings per class. */
export type MutationLabelCensusRow = { readonly owner: string; labels: number; native: number; forward: number; findings: number; readonly refused: Record<string, number> };

/** 🏷️ The `schema-mutation-label` lint, its census and every label site read. */
export type MutationLabelReport = { readonly diagnostics: readonly SchemaDiagnostic[]; readonly census: readonly MutationLabelCensusRow[]; readonly sites: readonly MutationLabelSite[] };

const MUTATION_LABEL_TRAITS = new Set(["MutationKind", "CompositeMutationKind", "SemanticMutation"]);
const MUTATION_LABEL_APP_TRAITS = new Set(["ArtifactApp", "ArtifactEditor", "ArtifactViewer"]);

/** 🏷️ The verdict of one label body `tokens[start..end]`: every `LocalizedLabel::native` call must take two locale arguments, neither
 * the empty literal; `LocalizedLabel::data` and `print_op` are findings; a body without `native` must forward to a `label` call. */
export function rustLabelVerdict(tokens: readonly RustToken[], start: number, end: number): { verdict: MutationLabelVerdict; texts: (readonly [string | null, string | null])[] } {
  const texts: (readonly [string | null, string | null])[] = [];
  const found = new Set<MutationLabelVerdict>();
  let forward = false;
  for (let index = start; index < end; index += 1) {
    const token = tokens[index]!;
    if (rustPathAt(tokens, index, ["LocalizedLabel", "native"]) && tokens[index + 4]?.text === "(") {
      const args = rustCallArguments(tokens, index + 4);
      if (args.length !== 2) found.add("labelUnresolved");
      else {
        const [en, de] = args.map(rustFirstLiteral) as [string | null, string | null];
        texts.push([en, de]);
        if (args.some((arg) => arg.filter((part) => part.text !== "&").length === 1 && rustFirstLiteral(arg) === "")) found.add("labelLocaleEmpty");
      }
    } else if (rustPathAt(tokens, index, ["LocalizedLabel", "data"])) found.add("labelLocaleInvariant");
    else if (token.kind === "ident" && token.text === "print_op") found.add("labelOpText");
    else if (token.kind === "ident" && /(^|_)label$/u.test(token.text) && tokens[index + 1]?.text === "(" && tokens[index - 1]?.text !== "fn") forward = true;
  }
  const uninhabited = tokens.slice(start + 1, end).map((token) => token.text).join(" ") === "match * self { }";
  const worst = (["labelOpText", "labelLocaleInvariant", "labelLocaleEmpty", "labelUnresolved"] as const).find((verdict) => found.has(verdict));
  return { verdict: worst ?? (texts.length > 0 ? "native" : forward || uninhabited ? "forward" : "labelUnresolved"), texts };
}

/** 🏷️ Every label site of one Rust source: each `fn label` of a `MutationKind`/`CompositeMutationKind`/`SemanticMutation` impl, each
 * `fn mutation_label` of an app impl (an override, always a finding), and each `LocalizedLabel::data(…)` call fed by `print_op`. */
export function rustMutationLabelSites(path: string, source: string): MutationLabelSite[] {
  const tokens = rustLex(source);
  const sites: MutationLabelSite[] = [];
  for (const block of rustImplBlocks(tokens)) {
    for (const fn of rustFunctions(tokens, block.open, block.close)) {
      if (fn.body === null) continue;
      if (MUTATION_LABEL_TRAITS.has(block.trait) && fn.name === "label") sites.push({ path, trait: block.trait, implementor: block.implementor, ...rustLabelVerdict(tokens, fn.body[0], fn.body[1]) });
      if (MUTATION_LABEL_APP_TRAITS.has(block.trait) && fn.name === "mutation_label") sites.push({ path, trait: block.trait, implementor: block.implementor, verdict: "labelOverride", texts: [] });
    }
  }
  for (let index = 0; index < tokens.length; index += 1) {
    if (!rustPathAt(tokens, index, ["LocalizedLabel", "data"]) || tokens[index + 4]?.text !== "(") continue;
    const close = rustGroupEnd(tokens, index + 4);
    if (tokens.slice(index + 5, close).some((token) => token.kind === "ident" && token.text === "print_op")) sites.push({ path, trait: "data", implementor: "", verdict: "labelOpText", texts: [] });
  }
  return sites;
}

/**
 * 🏷️ Reads every label site of every Rust source under `under` (design §16.2): a history row is labelled by its leaf in every shell
 * locale, so each leaf label is `LocalizedLabel::native(en, de)` or a forward to one, no app overrides the leaf label, and no
 * operation's text line is ever a label. Every other verdict is one `schema-mutation-label` diagnostic.
 */
export function mutationLabelReport(repoRoot: string, under = ""): MutationLabelReport {
  const diagnostics: SchemaDiagnostic[] = [];
  const census = new Map<string, MutationLabelCensusRow>();
  const sites: MutationLabelSite[] = [];
  const relevant = (source: string): boolean => !source.includes("quote!") && (source.includes("fn label") || source.includes("fn mutation_label") || (source.includes("LocalizedLabel::data") && source.includes("print_op")));
  for (const { path, source } of rustSources(repoRoot, under, relevant)) {
    if (path.split("/").includes("🧪️tests")) continue;
    for (const site of rustMutationLabelSites(path, source)) {
      sites.push(site);
      const owner = mutationPayloadOwner(path);
      const row = census.get(owner) ?? { owner, labels: 0, native: 0, forward: 0, findings: 0, refused: {} };
      census.set(owner, row);
      if (site.trait !== "data" && !MUTATION_LABEL_APP_TRAITS.has(site.trait)) row.labels += 1;
      if (site.verdict === "native") row.native += 1;
      else if (site.verdict === "forward") row.forward += 1;
      else {
        row.findings += 1;
        row.refused[site.verdict] = (row.refused[site.verdict] ?? 0) + 1;
        diagnostics.push({ code: "schema-mutation-label", scope: null, export: null, format: null, path, detail: `${site.verdict}: ${site.trait} for ${site.implementor || "a LocalizedLabel::data call"}${site.texts.length > 0 ? ` ${JSON.stringify(site.texts)}` : ""}` });
      }
    }
  }
  return { diagnostics, census: [...census.values()].sort((left, right) => right.findings - left.findings || left.owner.localeCompare(right.owner)), sites };
}

/**
 * 🏷️ `test schema mutation-labels` — the `schema-mutation-label` gate: every applied leaf of every plugin labels itself in every shell
 * locale, no app overrides that label, no history row falls back to an operation's text line. `--census` prints the per-plugin table
 * and always exits 0; without it any finding fails.
 *
 *   bun 📜️script.ts schema mutation-labels [--census] [--under <path>] [--json]
 */
function runMutationLabels(repoRoot: string, segments: string[]): never {
  const under = segments.includes("--under") ? (segments[segments.indexOf("--under") + 1] ?? "") : "";
  const report = mutationLabelReport(repoRoot, under);
  const census = segments.includes("--census");
  if (segments.includes("--json")) {
    console.log(JSON.stringify(census ? report.census : { diagnostics: report.diagnostics, census: report.census }, null, 2));
    process.exit(census || report.diagnostics.length === 0 ? 0 : 1);
  }
  const total = report.census.reduce((sum, row) => ({ labels: sum.labels + row.labels, native: sum.native + row.native, forward: sum.forward + row.forward }), { labels: 0, native: 0, forward: 0 });
  if (census) {
    const classes = [...new Set(report.census.flatMap((row) => Object.keys(row.refused)))].sort();
    console.log(["owner", "labels", "native", "forward", "findings", ...classes].join("\t"));
    for (const row of report.census) console.log([row.owner, row.labels, row.native, row.forward, row.findings, ...classes.map((name) => row.refused[name] ?? 0)].join("\t"));
  } else for (const entry of report.diagnostics.slice(0, 40)) console.log(`[schema mutation-labels]   ${entry.path} — ${entry.detail}`);
  console.log(`[schema mutation-labels] ${total.native} native and ${total.forward} forwarding of ${total.labels} leaf label(s); ${report.diagnostics.length} schema-mutation-label finding(s)${under === "" ? "" : ` under ${under}`}`);
  process.exit(census || report.diagnostics.length === 0 ? 0 : 1);
}
//#endregion 🏷️MutationLabels

//#region ✏️MutationEditability
/** ✏️ The history-edit verdict of one operation shape: `editable` (an input schema and no foreign-step capability), `inert` (no input
 * schema — a non-payload phase of a `#[mutation_leaf(payload = …)]` leaf) or `foreign` (a composite that may emit foreign steps). */
export type MutationEditabilityVerdict = "editable" | "inert" | "foreign";

/** ✏️ One leaf of a `#[derive(Mutations)]` aggregate: its verdict and, for a payload-marked leaf, the inert phase variants beside its
 * editable payload. */
export type MutationLeafEditability = Readonly<{ owner: string; aggregate: string; path: string; kind: string; variant: string; verdict: "editable" | "foreign"; inert: readonly string[] }>;

/** 🖐️ One hand-written `impl Mutation<S> for T`: the declared reason it is outside the generic history editor — it forwards every
 * payload accessor of a derived aggregate (`forwarding`), it is a config/presence/transient/window/draft lane (`lane`), a test fixture
 * (`fixture`), uninhabited (`empty`), or no app names it as its document `type Mutation`, so no history row ever holds one
 * (`unexposed`) — or the finding `aggregateHandwritten`. */
export type MutationHandwrittenAggregate = Readonly<{ owner: string; path: string; name: string; snapshot: string; reason: "forwarding" | "lane" | "fixture" | "empty" | "unexposed" | "aggregateHandwritten" }>;

/** 📊️ One plugin's share of the editability census. */
export type MutationEditabilityCensusRow = { readonly owner: string; aggregates: number; leaves: number; editable: number; foreign: number; inert: number; handwritten: number; findings: number; readonly refused: Record<string, number> };

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

/**
 * ✏️ Enumerates every mutation aggregate under `under` (design §16.3) and decides, from source alone, which history mutations the
 * generic editor can edit: each leaf of a `#[derive(Mutations)]` aggregate is `editable` unless its descriptor composes a plan
 * (`foreign`, the `may_emit_foreign_steps` capability), a payload-marked leaf adds its inert phases; a generic aggregate gets no emitted
 * payload law (`aggregateGeneric`), a variant without a leaf descriptor is `leafUnresolved`, and a hand-written `impl Mutation` must
 * declare its reason (`mutationHandwrittenReason`) or is `aggregateHandwritten`. `roots` bounds the tree read (the source roots by
 * default); a narrower root also bounds the app-document scan to `under`.
 */
export function mutationEditabilityReport(repoRoot: string, under = "", roots: readonly string[] = MUTATION_TREE_ROOTS): MutationEditabilityReport {
  const tree = mutationTree(repoRoot, roots);
  const diagnostics: SchemaDiagnostic[] = [];
  const census = new Map<string, MutationEditabilityCensusRow>();
  const rowOf = (owner: string): MutationEditabilityCensusRow => {
    const row = census.get(owner) ?? { owner, aggregates: 0, leaves: 0, editable: 0, foreign: 0, inert: 0, handwritten: 0, findings: 0, refused: {} };
    census.set(owner, row);
    return row;
  };
  const refuse = (row: MutationEditabilityCensusRow, kind: string, path: string, detail: string): void => {
    row.findings += 1;
    row.refused[kind] = (row.refused[kind] ?? 0) + 1;
    diagnostics.push({ code: "schema-mutation-editability", scope: null, export: null, format: null, path, detail: `${kind}: ${detail}` });
  };
  const leaves: MutationLeafEditability[] = [];
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
      const composite = readJsonObject(repoRoot, `${leaf.directory}/🔣️.json`)?.composition === "composite";
      const wrapper = tree.wrappers.get(leaf.directory)?.find((candidate) => candidate.name === aggregate.payloadTypes.get(variant));
      const inert = wrapper === undefined ? [] : [...wrapper.variants.keys()].filter((phase) => phase !== wrapper.payloadVariant);
      leaves.push({ owner, aggregate: aggregate.name, path: leaf.directory, kind: leaf.kind, variant, verdict: composite ? "foreign" : "editable", inert });
      row.leaves += 1;
      row.inert += inert.length;
      if (composite) row.foreign += 1;
      else row.editable += 1;
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
function runMutationEditability(repoRoot: string, segments: string[]): never {
  const under = segments.includes("--under") ? (segments[segments.indexOf("--under") + 1] ?? "") : "";
  const report = mutationEditabilityReport(repoRoot, under);
  const census = segments.includes("--census");
  if (segments.includes("--json")) {
    console.log(JSON.stringify(census ? report.census : report, null, 2));
    process.exit(census || report.diagnostics.length === 0 ? 0 : 1);
  }
  const total = report.census.reduce((sum, row) => ({ aggregates: sum.aggregates + row.aggregates, leaves: sum.leaves + row.leaves, editable: sum.editable + row.editable, foreign: sum.foreign + row.foreign, inert: sum.inert + row.inert, handwritten: sum.handwritten + row.handwritten }), { aggregates: 0, leaves: 0, editable: 0, foreign: 0, inert: 0, handwritten: 0 });
  if (census) {
    const classes = [...new Set(report.census.flatMap((row) => Object.keys(row.refused)))].sort();
    console.log(["owner", "aggregates", "leaves", "editable", "foreign", "inert", "handwritten", "findings", ...classes].join("\t"));
    for (const row of report.census) console.log([row.owner, row.aggregates, row.leaves, row.editable, row.foreign, row.inert, row.handwritten, row.findings, ...classes.map((name) => row.refused[name] ?? 0)].join("\t"));
  } else for (const entry of report.diagnostics.slice(0, 40)) console.log(`[schema mutation-editability]   ${entry.path} — ${entry.detail}`);
  console.log(`[schema mutation-editability] ${total.editable}/${total.leaves} leaves of ${total.aggregates} aggregates editable (${total.foreign} composite with foreign-step capability, ${total.inert} inert phase(s)); ${total.handwritten} hand-written aggregate(s); ${report.diagnostics.length} schema-mutation-editability finding(s)${under === "" ? "" : ` under ${under}`}`);
  process.exit(census || report.diagnostics.length === 0 ? 0 : 1);
}
//#endregion ✏️MutationEditability

/**
 * 🧬️ `test schema` — the scope-owned schema contract gate.
 *
 * It answers seven questions in one pass and keeps them apart in the output: does every contract sit
 * on an eligible owner in the one place it belongs (the invariants), does every `schema://` reference
 * resolve through the declared catalog (resolution), does every bound fixture fail or pass at the
 * STAGE it declared (the fixtures), does every mutation input carry a UI descriptor
 * (`schema-mutation-input-ui`), does every mutation leaf and aggregate schema describe the wire its
 * committed fixtures and `#[derive(Mutations)]` enum witness (`schema-mutation-payload-parity`), is
 * every leaf labelled in every locale (`schema-mutation-label`), and is every aggregate editable in
 * history or declares why not (`schema-mutation-editability`). Nothing
 * here searches the tree for a schema; an absent catalog is reported as absent, because a gate that
 * silently found a substitute would be measuring the substitute.
 *
 *   bun 📜️script.ts test schema                     # the whole tree
 *   bun 📜️script.ts test schema --under <path>      # one subtree, while the rest is mid-migration
 *   bun 📜️script.ts test schema --json
 */
export class SchemaScript extends Script {
  run(segments: string[]): void {
    if (segments[0] === "mutation-inputs") runMutationInputUi(this.repoRoot, segments.slice(1));
    if (segments[0] === "mutation-payloads") runMutationPayloadParity(this.repoRoot, segments.slice(1));
    if (segments[0] === "mutation-labels") runMutationLabels(this.repoRoot, segments.slice(1));
    if (segments[0] === "mutation-editability") runMutationEditability(this.repoRoot, segments.slice(1));
    const under = segments[segments.indexOf("--under") + 1];
    const scope = segments.includes("--under") && under !== undefined ? under : "";
    const diagnostics: SchemaDiagnostic[] = [
      ...schemaContractDiagnostics(this.repoRoot, scope),
      ...mutationInputUiReport(this.repoRoot, scope).diagnostics,
      ...mutationPayloadParityReport(this.repoRoot, scope).diagnostics,
      ...mutationLabelReport(this.repoRoot, scope).diagnostics,
      ...mutationEditabilityReport(this.repoRoot, scope).diagnostics,
    ];
    const reports: SchemaFixtureReport[] = [];
    for (const collection of discoverSchemaFixtures(this.repoRoot, scope)) for (const fixture of collection.fixtures) reports.push(runSchemaFixture(this.repoRoot, fixture, collection.caseDir));
    const failed = reports.filter((report) => report.outcome === "failed");
    if (segments.includes("--json")) {
      console.log(JSON.stringify({ diagnostics, fixtures: reports }, null, 2));
      process.exit(diagnostics.length === 0 && failed.length === 0 ? 0 : 1);
    }
    const byCode = new Map<string, number>();
    for (const entry of diagnostics) byCode.set(entry.code, (byCode.get(entry.code) ?? 0) + 1);
    console.log(`[test schema] ${diagnostics.length} invariant finding(s) over ${scope.length === 0 ? "the repository" : scope}`);
    for (const [code, count] of [...byCode].sort((a, b) => b[1] - a[1])) console.log(`[test schema]   ${String(count).padStart(5)} × ${code}`);
    for (const entry of diagnostics.slice(0, 40)) console.log(`[test schema]   ${entry.code} ${entry.path ?? entry.scope ?? ""} — ${entry.detail}`);
    if (diagnostics.length > 40) console.log(`[test schema]   … and ${diagnostics.length - 40} more`);
    console.log(`[test schema] ${reports.length - failed.length}/${reports.length} schema-bound fixture(s) reached their declared stage`);
    for (const report of reports) {
      const line = report.stages.map((entry) => `${entry.stage}=${entry.result}`).join(" ");
      console.log(`[test schema]   ${report.outcome === "passed" ? "✔" : "✘"} ${report.fixture} ${report.uri} — ${line}`);
      if (report.outcome === "failed") console.log(`[test schema]     ${report.detail}`);
    }
    process.exit(diagnostics.length === 0 && failed.length === 0 ? 0 : 1);
  }
}
