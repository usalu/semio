import { type FeatureStep } from "../../../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { type SchemaDiagnostic, type SchemaFixtureReport, discoverSchemaFixtures, readSchemaCatalog, runSchemaFixture, schemaContractDiagnostics } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { parseFeature } from "../../../../../../🔨️modules/🧪️test/🥒️gherkin/🟦️.ts";
import { Script } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { type ActionArgDef, type FaultNoticeDefinition, type InputSchemaAudit, SHELL_LOCALES, SHELL_TERMINOLOGIES, argControl, inputLabelGlossary, inputNumericTransport, inputShape, isFaultNoticeCode, isShellLocale, isShellTerminology, mutationInputAudit, mutationInputInstance, validateFaultNotices } from "../../../../../../🔨️modules/🛂️manifest/🟦️.ts";
import { parseSchemaInvariants } from "../../../../../../🔨️modules/🛂️manifest/🧬️schema/🟦️.ts";
import { type Dirent, existsSync, readdirSync, readFileSync } from "node:fs";
import { Validator } from "jsonschema";
import { join, posix } from "node:path";

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
/** 📊️ One plugin's share of the mutation-input census: leaves read, those their descriptor declares withdraw-only
 * (`withdrawOnly`: `"editable": false`, their inputs are not judged), top-level inputs of the editable leaves, those whose whole
 * subtree states its UI facts itself (`declared`), those that read into a valid
 * descriptor only by inference somewhere in their subtree (`inferred`: a glossary label, or a number without declared step or
 * snapping points), the leaf inputs at every depth whose label comes from the glossary in some locale (`labelInferred` — a count,
 * the glossary is a legitimate label source, design §6), every finding at every nested pointer, and the findings per error class.
 * The remaining `inputs - declared - inferred` are refused: unreadable, unlabelled, or outside the vocabulary. */
export type MutationInputCensusRow = { readonly owner: string; leaves: number; withdrawOnly: number; inputs: number; declared: number; inferred: number; labelInferred: number; findings: number; readonly refused: Record<string, number> };

/** 🪞️ Where one UI fact of an input comes from: its `x-semio-ui` annotation (`declared`), the reader's inference from the
 * field name or the JSON type (`inferred`), or nowhere (`absent`). */
export type MutationInputSource = "declared" | "inferred" | "absent";

/** 🔬️ One leaf input as its payload schema states it (design §22.8): the RFC 6901 `pointer` in the payload (`/-` an array item),
 * its property `key`, settled JSON `type`, the `widget` and `role` its `x-semio-ui` declares (own annotation first, then its `$ref`
 * targets'), the source of its `label` per locale (`null` where no label shows: a hidden input, an array item), and for a number
 * that is no reference the source of its `step` (`inferred` is the integer step 1) and whether it declares `snaps` or `snapSource`. */
export type MutationInputDeclaration = Readonly<{ pointer: string; key: string; type: string | null; widget: string | null; role: string | null; label: Readonly<Record<string, MutationInputSource>> | null; step: MutationInputSource | null; snaps: boolean | null }>;

/** 🚨️ The class of a declaration finding: an interactive number with neither step nor snapping points (`numericUndeclared`), a
 * shown label that neither `x-semio-ui.label` nor the glossary supplies in some locale (`labelAbsent`), a widget outside the strict
 * vocabulary fixture (`widgetUndeclared`), a `multiline` input the reader hands no multi-line control (`multilineUncontrolled`),
 * and an editable leaf that shows no input at all — none, or every one hidden (`inputless`, design §22.20: a leaf is either
 * editable with at least one input row, or its descriptor declares it withdraw-only). */
export type MutationInputDeclarationCode = "numericUndeclared" | "labelAbsent" | "widgetUndeclared" | "multilineUncontrolled" | "inputless";

/** 🧯️ One declaration finding at the `pointer` of the input it refuses. */
export type MutationInputDeclarationFinding = Readonly<{ code: MutationInputDeclarationCode; pointer: string; detail: string }>;

/** 🎓️ One leaf input of the report: its declaration under its owner, catalogued scope and schema path, and its verdict — `refused`
 * when a reader, label or vocabulary finding sits at its pointer, `inferred` when its number facets are undeclared or its label
 * comes from the glossary in some locale, else `declared`. */
export type MutationInputVerdict = MutationInputDeclaration & Readonly<{ owner: string; scope: string; path: string; verdict: "declared" | "inferred" | "refused" }>;

/** 📓️ The `schema-mutation-input-ui` lint, its census and every leaf input's verdict, over every catalogued mutation leaf under
 * `scope`; `multiline` says whether the reader maps the `multiline` presentation to a control (the rule `multilineUncontrolled` is
 * armed by that live predicate) and how many inputs declare the widget. */
export type MutationInputUiReport = { readonly diagnostics: readonly SchemaDiagnostic[]; readonly census: readonly MutationInputCensusRow[]; readonly inputs: readonly MutationInputVerdict[]; readonly multiline: Readonly<{ armed: boolean; declared: number }> };

/** 🗂️ The owner a scope id is counted under: the plugin of an `s.`/`app.` scope, else the scope's first segment. */
const mutationInputCensusOwner = (scope: string): string => {
  const [head, second] = scope.split(".");
  return (head === "s" || head === "app") && second !== undefined ? second : (head ?? scope);
};

/** 🏠️ The owner an uncatalogued leaf directory is counted under: its plugin (`✏️s/🔌️plugins/<plugin>/…`), else its product or
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
 * 🔢️ Every payload pointer of the leaf schema `root` (resolving `$ref`s through `resolve`, across documents) whose schema admits a
 * number only as the exact binary64 word (`{bits}`, `framework/value/schema.json#/$defs/Binary64`) — never the plain number the
 * payload and every history-edit draft carry, which the time-travel validator then refuses. Reaches what the input reader never
 * reads into (hidden, structured and recursive values): properties (`/key`), array items (`/-`), tuple items (`/<index>`), additional
 * properties (`/*`), `allOf` members and union branches; a union branch that is the word is fine when a sibling branch admits a number, and
 * a recursive definition is reported once, at its first pointer.
 */
export function wordOnlyFloatPointers(root: Record<string, unknown>, resolve: (id: string) => Record<string, unknown> | undefined): string[] {
  const { word } = inputNumericTransport();
  const found = new Set<string>();
  const isObject = (value: unknown): value is Record<string, unknown> => value !== null && typeof value === "object" && !Array.isArray(value);
  const target = (document: Record<string, unknown>, reference: string): [Record<string, unknown>, unknown] | undefined => {
    const hash = reference.indexOf("#");
    const id = hash < 0 ? reference : reference.slice(0, hash);
    const owner = id === "" ? document : resolve(id);
    if (owner === undefined) return undefined;
    let current: unknown = owner;
    for (const segment of (hash < 0 ? "" : reference.slice(hash + 1)).split("/").slice(1).map((part) => part.replaceAll("~1", "/").replaceAll("~0", "~"))) current = isObject(current) ? current[segment] : Array.isArray(current) && /^\d+$/u.test(segment) ? current[Number(segment)] : undefined;
    return current === undefined ? undefined : [owner, current];
  };
  const settle = (document: Record<string, unknown>, node: unknown, active: readonly unknown[]): [Record<string, unknown>, unknown] => {
    let [owner, current] = [document, node];
    for (let hop = 0; hop < 32 && isObject(current) && typeof current.$ref === "string" && !active.includes(current); hop += 1) {
      const next = target(owner, current.$ref);
      if (next === undefined) break;
      [owner, current] = next;
    }
    return [owner, current];
  };
  const numeric = (node: unknown): boolean => isObject(node) && (["number", "integer"].includes(node.type as string) || (Array.isArray(node.type) && node.type.some((name) => name === "number" || name === "integer")));
  const visit = (document: Record<string, unknown>, start: unknown, pointer: string, active: readonly unknown[]): void => {
    const [owner, node] = settle(document, start, active);
    if (!isObject(node) || active.includes(node) || active.length > 64) return;
    if (inputShape(node as never) === word) {
      found.add(pointer);
      return;
    }
    const path = [...active, node];
    const union = Array.isArray(node.anyOf) ? node.anyOf : Array.isArray(node.oneOf) ? node.oneOf : [];
    const settled = union.map((branch) => settle(owner, branch, path));
    const admitsNumber = settled.some(([, branch]) => numeric(branch));
    settled.forEach(([branchOwner, branch]) => {
      if (!(admitsNumber && isObject(branch) && inputShape(branch as never) === word)) visit(branchOwner, branch, pointer, path);
    });
    for (const member of Array.isArray(node.allOf) ? node.allOf : []) visit(owner, member, pointer, path);
    for (const [key, child] of Object.entries(isObject(node.properties) ? node.properties : {})) visit(owner, child, `${pointer}/${key.replaceAll("~", "~0").replaceAll("/", "~1")}`, path);
    if (isObject(node.items)) visit(owner, node.items, `${pointer}/-`, path);
    if (Array.isArray(node.items)) node.items.forEach((item, index) => visit(owner, item, `${pointer}/${index}`, path));
    if (isObject(node.additionalProperties)) visit(owner, node.additionalProperties, `${pointer}/*`, path);
  };
  visit(root, root, "", []);
  return [...found].sort();
}

/** 🚚️ The 1-based lines of an artifact TS twin that read a float carrier through the strict word parsers (`parseBinary64(`/`parseBinary32(`)
 * instead of the transport readers (`parseBinary64Transport`/`parseBinary32Transport`, word | number): a twin then refuses the plain number
 * every payload and history-edit draft carries. A call re-validating the twin's own in-memory word for output (`parseBinary64(x).bits`) is
 * not a read. */
export function wordOnlyFloatTwinLines(source: string): number[] {
  const lines: number[] = [];
  for (const match of source.matchAll(/\bparseBinary(?:64|32)\(/g)) {
    let depth = 0;
    let end = match.index! + match[0].length - 1;
    for (; end < source.length; end += 1) {
      depth += source[end] === "(" ? 1 : source[end] === ")" ? -1 : 0;
      if (depth === 0) break;
    }
    if (source.slice(end + 1, end + 6) !== ".bits") lines.push(source.slice(0, match.index).split("\n").length);
  }
  return lines;
}

const MUTATION_INPUT_TYPES = new Set(["string", "integer", "number", "boolean", "object", "array"]);
const MUTATION_INPUT_ANNOTATIONS = new Set(["title", "description", "$comment", "examples", "default", "format"]);
const MUTATION_INPUT_NUMBER_WIDGETS = new Set(["slider", "stepper", "dial"]);
const MUTATION_INPUT_ITEM_FACETS = new Set(["unit", "step", "precision", "snaps", "snapSource", "displayUnit", "displayFactor"]);
const MUTATION_INPUT_INFERENCE_CODES: ReadonlySet<string> = new Set(["numericUndeclared"]);
const MUTATION_INPUT_LABEL_REFUSALS: ReadonlySet<string> = new Set(["labelMissing", "localeMissing"]);
const MUTATION_INPUT_VOCABULARY = "🧰️framework/🔨️modules/🧬️schema/🧬️vendor-annotation-vocabulary/🔣️.json";

/**
 * 🎚️ Every leaf input of the mutation payload schema `root` with the source of each UI fact — the gate's own walk of the schema,
 * independent of the reader's inference, so a fact the reader infers is never counted as stated. It follows the reader's shape rules
 * (`manifest::mutation_input_defs`): an outer `x-semio-ui` overrides its `$ref` target's key by key, a `null` branch beside one value
 * branch is that value, `allOf` members compose, a root union reads its discriminator as the selector input and every variant's
 * fields, a discriminator or `const` is no input, a hidden input and a reference are not read into, a fixed array of 2 to 4 numbers
 * is one vector, and the numbers of any other array inherit the array's number facets. Rows come in reading order, each distinct row once.
 */
export function mutationInputDeclarations(root: Record<string, unknown>, resolve: (id: string) => Record<string, unknown> | undefined): MutationInputDeclaration[] {
  type Settled = Readonly<{ owner: Record<string, unknown>; node: Record<string, unknown>; ui: ReadonlyMap<string, unknown> }>;
  const glossary = inputLabelGlossary();
  const { word, number } = inputNumericTransport();
  const rows = new Map<string, MutationInputDeclaration>();
  const text = (value: unknown): string | null => (typeof value === "string" ? value : null);
  const union = (node: Record<string, unknown>): unknown[] | undefined => (Array.isArray(node.oneOf) ? node.oneOf : Array.isArray(node.anyOf) ? node.anyOf : undefined);
  const target = (owner: Record<string, unknown>, reference: string): [Record<string, unknown>, unknown] | undefined => {
    const hash = reference.indexOf("#");
    const document = hash === 0 ? owner : resolve(hash < 0 ? reference : reference.slice(0, hash));
    let current: unknown = document;
    for (const segment of (hash < 0 ? "" : reference.slice(hash + 1)).split("/").slice(1).map(unescapePointer)) current = isRecord(current) ? current[segment] : Array.isArray(current) && /^\d+$/u.test(segment) ? current[Number(segment)] : undefined;
    return document === undefined || current === undefined ? undefined : [document, current];
  };
  const settle = (start: Record<string, unknown>, from: unknown): Settled | null => {
    let [owner, node] = [start, from];
    const ui = new Map<string, unknown>();
    for (let hop = 0; hop < 32; hop += 1) {
      if (!isRecord(node)) return null;
      const annotation = node["x-semio-ui"];
      for (const [key, value] of isRecord(annotation) ? Object.entries(annotation) : []) if (!ui.has(key)) ui.set(key, value);
      if (typeof node.$ref === "string") {
        const next = target(owner, node.$ref);
        if (next === undefined) return null;
        [owner, node] = next;
        continue;
      }
      const branches = union(node) ?? [];
      const concrete = branches.filter((branch) => !(isRecord(branch) && branch.type === "null" && Object.keys(branch).length === 1));
      if (concrete.length !== 1 || concrete.length === branches.length) return { owner, node, ui };
      node = concrete[0];
    }
    return null;
  };
  const shape = (owner: Record<string, unknown>, branch: unknown): string => {
    let [document, current] = [owner, branch];
    for (let hop = 0; hop < 32 && isRecord(current) && typeof current.$ref === "string"; hop += 1) {
      const next = target(document, current.$ref);
      if (next === undefined) return "null";
      [document, current] = next;
    }
    return inputShape(current as never);
  };
  const type = ({ owner, node }: Settled): string | null => {
    const branches = union(node);
    const shapes = branches?.length === 2 && Object.keys(node).every((key) => key === "anyOf" || key === "oneOf" || key.startsWith("x-") || MUTATION_INPUT_ANNOTATIONS.has(key)) ? branches.map((branch) => shape(owner, branch)) : [];
    if (inputShape(node as never) === word || (shapes.includes(word) && shapes.includes(number))) return "number";
    const named: unknown[] = typeof node.type === "string" ? [node.type] : Array.isArray(node.type) ? node.type.filter((name) => name !== "null") : node.properties !== undefined ? ["object"] : node.items !== undefined ? ["array"] : Array.isArray(node.enum) && node.enum.length > 0 && node.enum.every((value) => typeof value === "string") ? ["string"] : [];
    return named.length === 1 && typeof named[0] === "string" && MUTATION_INPUT_TYPES.has(named[0]) ? named[0] : null;
  };
  const label = (key: string, stated: unknown): Record<string, MutationInputSource> => {
    const cell = (map: unknown, locale: string): boolean => isRecord(map) && typeof map[locale] === "string" && map[locale] !== "";
    const names = isRecord(stated) ? Object.keys(stated) : [];
    const declares = (locale: string): boolean => names.length > 0 && (names.every(isShellLocale) ? cell(stated, locale) : names.every(isShellTerminology) && SHELL_TERMINOLOGIES.every((terminology) => cell((stated as Record<string, unknown>)[terminology], locale)));
    return Object.fromEntries(SHELL_LOCALES.map((locale) => [locale, declares(locale) ? "declared" : glossary.has(key) ? "inferred" : "absent"]));
  };
  const referenced = ({ ui }: Settled): boolean => ui.has("ref") || ui.get("widget") === "reference" || ui.get("role") === "target";
  const emit = (pointer: string, key: string, settled: Settled, kind: string | null, labelled: boolean): void => {
    const widget = text(settled.ui.get("widget"));
    const numeric = (kind === "integer" || kind === "number") && !referenced(settled);
    const snaps = settled.ui.get("snaps");
    const row: MutationInputDeclaration = { pointer, key, type: kind, widget, role: text(settled.ui.get("role")), label: labelled && widget !== "hidden" ? label(key, settled.ui.get("label")) : null, step: numeric ? (settled.ui.has("step") ? "declared" : kind === "integer" ? "inferred" : "absent") : null, snaps: numeric ? (Array.isArray(snaps) && snaps.length > 0) || settled.ui.has("snapSource") : null };
    rows.set(JSON.stringify(row), row);
  };
  const properties = (object: Settled): Map<string, Settled> => {
    const found = new Map<string, Settled>();
    const compose = (member: Settled, depth: number): void => {
      for (const [key, child] of Object.entries(isRecord(member.node.properties) ? member.node.properties : {})) {
        const settled = found.has(key) ? null : settle(member.owner, child);
        if (settled !== null) found.set(key, settled);
      }
      for (const part of depth < 32 && Array.isArray(member.node.allOf) ? member.node.allOf : []) {
        const next = settle(member.owner, part);
        if (next !== null) compose(next, depth + 1);
      }
    };
    compose(object, 0);
    return found;
  };
  const fields = (object: Settled, pointer: string, active: readonly unknown[]): void => {
    for (const [key, settled] of properties(object)) {
      if (settled.ui.get("role") === "discriminator" || settled.node.const !== undefined) continue;
      const kind = type(settled);
      emit(`${pointer}/${escapePointer(key)}`, key, settled, kind, true);
      if (settled.ui.get("widget") !== "hidden" && !referenced(settled)) value(settled, kind, `${pointer}/${escapePointer(key)}`, active);
    }
  };
  const value = (settled: Settled, kind: string | null, pointer: string, active: readonly unknown[]): void => {
    const { owner, node, ui } = settled;
    if (active.includes(node) || active.length >= 32) return;
    if (kind === "object") return fields(settled, pointer, [...active, node]);
    const items = kind === "array" ? settle(owner, node.items) : null;
    if (items === null || items.node.const !== undefined) return;
    const itemKind = type(items);
    const numeric = itemKind === "integer" || itemKind === "number";
    if (ui.get("widget") === "vector" || ui.get("widget") === "color" || (numeric && typeof node.minItems === "number" && node.minItems === node.maxItems && node.minItems >= 2 && node.minItems <= 4)) return;
    const merged: Settled = { ...items, ui: new Map([...items.ui, ...(numeric ? [...ui].filter(([name]) => MUTATION_INPUT_ITEM_FACETS.has(name) && !items.ui.has(name)) : [])]) };
    if (numeric || (itemKind !== "object" && itemKind !== "array" && merged.ui.has("widget"))) emit(`${pointer}/-`, "-", merged, itemKind, false);
    value(merged, itemKind, `${pointer}/-`, [...active, node]);
  };
  const payload = settle(root, root);
  if (payload === null) return [];
  if (payload.node.properties !== undefined || payload.node.allOf !== undefined) fields(payload, "", [payload.node]);
  else {
    const variants = (union(payload.node) ?? []).flatMap((branch) => settle(payload.owner, branch) ?? []);
    const pins = variants.map(properties);
    const selector = [...(pins[0]?.keys() ?? [])].find((name) => {
      const values = pins.map((found) => found.get(name)?.node.const);
      return values.every((pinned, index) => typeof pinned === "string" && !values.slice(0, index).includes(pinned));
    });
    if (selector !== undefined) emit(`/${escapePointer(selector)}`, selector, pins[0]!.get(selector)!, "string", true);
    for (const variant of variants) fields(variant, "", [payload.node, variant.node]);
  }
  return [...rows.values()];
}

/**
 * 🧨️ The declaration findings of one leaf's `rows` (design §22.8, §22.20), each distinct one once. A withdraw-only leaf (`editable`
 * false: its descriptor says `"editable": false`) is not judged at all. An editable leaf is `inputless` (at the payload root `""`)
 * when no top-level input of it is shown — it has none, or every one is hidden. Per input: `widgetUndeclared` for a widget outside `widgets`
 * (the strict vocabulary); `numericUndeclared` for an interactive number — role `value` or none, widget slider, stepper, dial or
 * none — that declares neither `step` nor `snaps`/`snapSource`; `labelAbsent` for a shown label that neither its `x-semio-ui.label`
 * nor the glossary supplies in some locale (a label the glossary supplies is counted, never refused: design §6); and, when
 * `controls` (pointer → control kind of the reader's descriptors, {@link mutationInputControls}) is given, `multilineUncontrolled`
 * for a `multiline` input whose control is not the multi-line one.
 */
export function mutationInputDeclarationFindings(rows: readonly MutationInputDeclaration[], widgets: ReadonlySet<string>, controls: ReadonlyMap<string, string> | null, editable = true): MutationInputDeclarationFinding[] {
  const found = new Map<string, MutationInputDeclarationFinding>();
  const refuse = (code: MutationInputDeclarationCode, pointer: string, detail: string): void => void found.set(`${code}\n${pointer}\n${detail}`, { code, pointer, detail });
  if (!editable) return [];
  if (!rows.some((row) => row.pointer.lastIndexOf("/") === 0 && row.widget !== "hidden")) refuse("inputless", "", `an editable leaf shows no input (${rows.some((row) => row.pointer.lastIndexOf("/") === 0) ? "every one is hidden" : "it has none"}): give it an input its editor can show, or declare the leaf withdraw-only with "editable": false in its descriptor`);
  for (const row of rows) {
    const absent = Object.entries(row.label ?? {}).filter(([, source]) => source === "absent").map(([locale]) => locale);
    if (row.widget !== null && !widgets.has(row.widget)) refuse("widgetUndeclared", row.pointer, `widget ${JSON.stringify(row.widget)} is outside the strict x-semio-ui vocabulary (${[...widgets].join(", ")})`);
    if (row.step !== null && row.step !== "declared" && row.snaps !== true && (row.widget === null || MUTATION_INPUT_NUMBER_WIDGETS.has(row.widget)) && (row.role === null || row.role === "value")) refuse("numericUndeclared", row.pointer, `an interactive ${row.type} declares neither x-semio-ui.step nor snaps/snapSource (${row.step === "inferred" ? "its step is the inferred integer step 1" : "it has no step"}): declare the step or the snapping points its control offers`);
    if (absent.length > 0) refuse("labelAbsent", row.pointer, `the label of ${row.key} resolves to nothing in ${absent.join(", ")}: neither x-semio-ui.label nor the input-label glossary names it there`);
    if (controls !== null && row.widget === "multiline" && controls.get(row.pointer) !== "multiline") refuse("multilineUncontrolled", row.pointer, `widget multiline reaches ${controls.has(row.pointer) ? `the ${controls.get(row.pointer)} control` : "no control"}, never the multi-line text control`);
  }
  return [...found.values()];
}

/** 🎹️ The control kind ({@link argControl}) of every input the reader read, keyed by its payload pointer: nested object fields
 * under their parent's pointer, the fields of an array's object items under `<pointer>/-`. A scalar array item has no descriptor of
 * its own, so it has no entry. */
export function mutationInputControls(inputs: readonly ActionArgDef[], parent = ""): Map<string, string> {
  const found = new Map<string, string>();
  const nested = (schema: ActionArgDef["schema"], pointer: string): void => {
    if (schema.kind === "object") for (const [inner, kind] of mutationInputControls(schema.fields, pointer)) found.set(inner, kind);
    if (schema.kind === "array") nested(schema.items, `${pointer}/-`);
  };
  for (const input of inputs) {
    found.set(`${parent}${input.id}`, argControl(input).kind);
    nested(input.schema, `${parent}${input.id}`);
  }
  return found;
}

/** 💡️ Whether the reader maps the `multiline` presentation of a string input to a multi-line control — the live predicate that
 * arms `multilineUncontrolled` (design §22.7): until it holds, every `multiline` input is edited in a one-line field. */
export function mutationInputMultilineArmed(): boolean {
  const probe: ActionArgDef = { id: "/probe", label: null, schema: { kind: "string", options: [] }, presentation: { kind: "multiline" }, required: false };
  return (argControl(probe).kind as string) === "multiline";
}

/** 🗝️ The widgets of the strict vocabulary: the `widget` enum of the `x-semio-ui` meta-schema the vendor-annotation vocabulary
 * domain vocabulary registers, its `$ref` followed through `resolve`. Throws when the vocabulary states none — the gate never falls back to a list of its own. */
export function mutationInputWidgetVocabulary(repoRoot: string, resolve: (id: string) => Record<string, unknown> | undefined): ReadonlySet<string> {
  const keywords = readJsonObject(repoRoot, MUTATION_INPUT_VOCABULARY)?.keywords;
  let node: unknown = isRecord(keywords) ? keywords["x-semio-ui"] : undefined;
  for (let hop = 0; hop < 32 && isRecord(node) && typeof node.$ref === "string"; hop += 1) {
    const [id = "", pointer = ""] = node.$ref.split("#");
    node = pointer.split("/").slice(1).map(unescapePointer).reduce<unknown>((current, segment) => (isRecord(current) ? current[segment] : undefined), resolve(id));
  }
  const widget = isRecord(node) && isRecord(node.properties) ? node.properties.widget : undefined;
  const names = isRecord(widget) && Array.isArray(widget.enum) ? widget.enum.filter((name): name is string => typeof name === "string") : [];
  if (names.length === 0) throw new Error(`[schema mutation-inputs] ${MUTATION_INPUT_VOCABULARY} states no x-semio-ui widget vocabulary`);
  return new Set(names);
}

/**
 * 🎛️ Audits every catalogued mutation leaf's payload schema with the framework's collecting reader `mutationInputAudit`
 * (the TypeScript twin of `manifest::mutation_input_audit`), resolving cross-document `$ref`s through the catalog's own
 * documents: every finding at every nested pointer is one `schema-mutation-input-ui` diagnostic naming the reader's error
 * class and the input pointer; a reader fault (any throw that is not an `InputSchemaError`) is the finding `readerFault`
 * for that leaf, never a crash of the lint. On top of the reader it measures declarations (design §22.8): every leaf input is
 * walked by {@link mutationInputDeclarations} and judged by {@link mutationInputDeclarationFindings} against the strict widget
 * vocabulary, so a number whose step the reader only infers fails as `numericUndeclared`; a label the glossary supplies is counted
 * (`labelInferred`), and `labelAbsent` is reported only where the reader does not already refuse that label (`labelMissing`,
 * `localeMissing`). A top-level input counts as `declared` when nothing in its subtree is refused or inferred, as `inferred` when
 * its subtree is read but rests on inference. A leaf whose descriptor (`<leaf>/🔣️.json` beside its `🧬️schema`) says
 * `"editable": false` is withdraw-only (design §22.20): it is counted and none of its inputs is read or judged.
 */
export function mutationInputUiReport(repoRoot: string, under = ""): MutationInputUiReport {
  const { catalog } = readSchemaCatalog(repoRoot);
  const documents = catalogSchemaDocuments(repoRoot);
  const read = (path: string): Record<string, unknown> | null => readJsonObject(repoRoot, path);
  const widgets = mutationInputWidgetVocabulary(repoRoot, (id) => documents.get(id));
  const armed = mutationInputMultilineArmed();
  const diagnostics: SchemaDiagnostic[] = [];
  const census = new Map<string, MutationInputCensusRow>();
  const inputs: MutationInputVerdict[] = [];
  for (const [scopeId, scope] of Object.entries(catalog?.scopes ?? {}).sort(([left], [right]) => left.localeCompare(right))) {
    if (!isMutationLeafScope(scope) || !scope.path.startsWith(under)) continue;
    const path = `${scope.path}/${scope.formats["🔣️jsonschema"] ?? "🔣️.json"}`;
    const leaf = read(path);
    const owner = mutationInputCensusOwner(scopeId);
    const row = census.get(owner) ?? { owner, leaves: 0, withdrawOnly: 0, inputs: 0, declared: 0, inferred: 0, labelInferred: 0, findings: 0, refused: {} };
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
    if (read(`${scope.path.slice(0, scope.path.lastIndexOf("/"))}/🔣️.json`)?.editable === false) {
      row.withdrawOnly += 1;
      continue;
    }
    let audit: InputSchemaAudit;
    try {
      audit = mutationInputAudit(leaf, (id) => documents.get(id));
    } catch (error) {
      refuse("readerFault", "", error instanceof Error ? `${error.name}: ${error.message}` : String(error));
      continue;
    }
    const declarations = mutationInputDeclarations(leaf, (id) => documents.get(id));
    const stated = mutationInputDeclarationFindings(declarations, widgets, armed ? mutationInputControls(audit.inputs) : null);
    const outside = new Set(stated.filter((finding) => finding.code === "widgetUndeclared").map((finding) => finding.pointer));
    const unlabelled = new Set(audit.findings.filter((finding) => MUTATION_INPUT_LABEL_REFUSALS.has(finding.code)).map((finding) => finding.pointer));
    const glossed = new Set(declarations.filter((declaration) => Object.values(declaration.label ?? {}).includes("inferred")).map((declaration) => declaration.pointer));
    const findings = [...audit.findings.filter((finding) => !(finding.code === "uiInvalid" && outside.has(finding.pointer) && finding.message.includes("widget is not a declared widget"))).map(({ code, pointer, message }) => ({ code: code as string, pointer, detail: message })), ...stated.filter((finding) => !(finding.code === "labelAbsent" && unlabelled.has(finding.pointer)))];
    const top = (pointer: string): string | undefined => (pointer === "" ? undefined : pointer.slice(1).split("/")[0]);
    const tops = (pointers: readonly string[]): Set<string> => new Set(pointers.map(top).filter((key): key is string => key !== undefined));
    const refusedTops = tops(findings.filter((finding) => !MUTATION_INPUT_INFERENCE_CODES.has(finding.code)).map((finding) => finding.pointer));
    const inferredTops = new Set([...tops([...findings.filter((finding) => MUTATION_INPUT_INFERENCE_CODES.has(finding.code)).map((finding) => finding.pointer), ...glossed])].filter((key) => !refusedTops.has(key)));
    const allTops = new Set([...tops(audit.inputs.map((input) => input.id)), ...tops(declarations.map((declaration) => declaration.pointer)), ...refusedTops, ...inferredTops]);
    row.inputs += allTops.size;
    row.inferred += inferredTops.size;
    row.labelInferred += glossed.size;
    row.declared += allTops.size - refusedTops.size - inferredTops.size;
    for (const finding of findings) refuse(finding.code, finding.pointer, finding.detail);
    for (const declaration of declarations) {
      const here = findings.filter((finding) => finding.pointer === declaration.pointer);
      inputs.push({ ...declaration, owner, scope: scopeId, path, verdict: here.some((finding) => !MUTATION_INPUT_INFERENCE_CODES.has(finding.code)) ? "refused" : here.length > 0 || glossed.has(declaration.pointer) ? "inferred" : "declared" });
    }
    const readWords = new Set(audit.findings.filter((finding) => finding.code === "wordOnlyFloat").map((finding) => finding.pointer));
    for (const pointer of wordOnlyFloatPointers(leaf, (id) => documents.get(id))) {
      if (!readWords.has(pointer)) refuse("wordOnlyFloat", pointer, "a payload value accepts only the exact binary64 word, never the plain number the payload and every history-edit draft carry; reference framework/value/schema.json#/$defs/Binary64Transport");
    }
  }
  const twins = repositorySources(repoRoot, under, "🟦️.ts", (text) => /\bparseBinary(?:64|32)\(/.test(text)).filter(({ path }) => path.includes("🗿️artifacts/") && !path.includes("🪶️sqlite") && !path.includes("🧪️tests"));
  for (const { path, source } of twins) {
    const owner = mutationLeafDirectoryOwner(path);
    const row = census.get(owner) ?? { owner, leaves: 0, withdrawOnly: 0, inputs: 0, declared: 0, inferred: 0, labelInferred: 0, findings: 0, refused: {} };
    census.set(owner, row);
    for (const line of wordOnlyFloatTwinLines(source)) {
      row.findings += 1;
      row.refused.wordOnlyFloatTwin = (row.refused.wordOnlyFloatTwin ?? 0) + 1;
      diagnostics.push({ code: "schema-mutation-input-ui", scope: null, export: null, format: "🟦️typescript", path, detail: `wordOnlyFloatTwin at "line ${line}": the artifact twin reads a float carrier through the strict word parser, refusing the plain number every payload and history-edit draft carries; read it with parseBinary64Transport/parseBinary32Transport (framework/value/schema.json#/$defs/Binary64Transport)` });
    }
  }
  const catalogued = new Set(Object.values(catalog?.scopes ?? {}).map((scope) => scope.path));
  for (const directory of mutationLeafSchemaDirectories(repoRoot, under)) {
    if (catalogued.has(directory)) continue;
    const owner = mutationLeafDirectoryOwner(directory);
    const row = census.get(owner) ?? { owner, leaves: 0, withdrawOnly: 0, inputs: 0, declared: 0, inferred: 0, labelInferred: 0, findings: 0, refused: {} };
    census.set(owner, row);
    row.leaves += 1;
    row.findings += 1;
    row.refused.leafUncatalogued = (row.refused.leafUncatalogued ?? 0) + 1;
    const path = `${directory}/🔣️.json`;
    const id = read(path)?.$id;
    diagnostics.push({ code: "schema-mutation-input-ui", scope: null, export: null, format: "🔣️jsonschema", path, detail: `leafUncatalogued at "": $id ${JSON.stringify(id ?? null)} names no catalogued mutation-leaf scope, so neither the reader nor this census reads ${directory}` });
  }
  return { diagnostics, census: [...census.values()].sort((left, right) => right.inputs - right.declared - (left.inputs - left.declared) || left.owner.localeCompare(right.owner)), inputs, multiline: { armed, declared: inputs.filter((input) => input.widget === "multiline").length } };
}

/**
 * 🚦️ `test schema mutation-inputs` — the `schema-mutation-input-ui` gate: every mutation input carries its UI descriptor (a label
 * in every locale from `x-semio-ui.label` or the glossary, valid `x-semio-ui` inside the strict vocabulary, a widget its value can
 * take), an interactive number DECLARES its step or snapping points, and an editable leaf shows at least one input (a leaf whose
 * descriptor says `"editable": false` is withdraw-only and not judged); what the reader only infers is counted apart. `--census`
 * prints the per-plugin table (withdraw-only leaves, declared, inferred, refused, glossary labels) and always exits 0 — the
 * rollout tracker; `--inputs` prints one row per leaf input with its verdict and the source of each fact; without `--census` any
 * finding fails.
 *
 *   bun 📜️script.ts schema mutation-inputs [--census] [--inputs] [--under <path>] [--json]
 */
function runMutationInputUi(repoRoot: string, segments: string[]): never {
  const under = segments.includes("--under") ? (segments[segments.indexOf("--under") + 1] ?? "") : "";
  const report = mutationInputUiReport(repoRoot, under);
  if (under !== "" && report.census.every((row) => row.leaves === 0)) {
    console.error(`[schema mutation-inputs] --under ${JSON.stringify(under)} holds no mutation leaf: pass a repository-relative path such as ✏️s/🔌️plugins/<plugin>`);
    process.exit(2);
  }
  const census = segments.includes("--census");
  const listed = segments.includes("--inputs");
  if (segments.includes("--json")) {
    console.log(JSON.stringify(census ? report.census : listed ? report.inputs : { diagnostics: report.diagnostics, census: report.census, multiline: report.multiline }, null, 2));
    process.exit(census || report.diagnostics.length === 0 ? 0 : 1);
  }
  const total = report.census.reduce((sum, row) => ({ leaves: sum.leaves + row.leaves, withdrawOnly: sum.withdrawOnly + row.withdrawOnly, inputs: sum.inputs + row.inputs, declared: sum.declared + row.declared, inferred: sum.inferred + row.inferred, labelInferred: sum.labelInferred + row.labelInferred }), { leaves: 0, withdrawOnly: 0, inputs: 0, declared: 0, inferred: 0, labelInferred: 0 });
  if (census) {
    const codes = [...new Set(report.census.flatMap((row) => Object.keys(row.refused)))].sort();
    console.log(["owner", "leaves", "withdrawOnly", "inputs", "declared", "inferred", "refused", "labelInferred", "findings", ...codes].join("\t"));
    for (const row of report.census) console.log([row.owner, row.leaves, row.withdrawOnly, row.inputs, row.declared, row.inferred, row.inputs - row.declared - row.inferred, row.labelInferred, row.findings, ...codes.map((code) => row.refused[code] ?? 0)].join("\t"));
  } else if (listed) {
    console.log(["owner", "verdict", "pointer", "type", "widget", "role", "label", "step", "snaps", "path"].join("\t"));
    for (const input of report.inputs) console.log([input.owner, input.verdict, input.pointer, input.type ?? "", input.widget ?? "", input.role ?? "", input.label === null ? "" : Object.entries(input.label).map(([locale, source]) => `${locale}:${source}`).join(","), input.step ?? "", input.snaps ?? "", input.path].join("\t"));
  } else for (const entry of report.diagnostics.slice(0, 40)) console.log(`[schema mutation-inputs]   ${entry.scope} — ${entry.detail}`);
  console.log(`[schema mutation-inputs] ${total.declared} declared + ${total.inferred} inferred of ${total.inputs} input(s) of ${total.leaves} leaves (${total.inputs - total.declared - total.inferred} refused, ${total.withdrawOnly} leaves withdraw-only); ${total.labelInferred} glossary label(s) at every depth; ${report.multiline.declared} multiline input(s), multilineUncontrolled ${report.multiline.armed ? "armed" : "pending (the reader maps multiline to no control yet)"}; ${report.diagnostics.length} schema-mutation-input-ui finding(s)${under === "" ? "" : ` under ${under}`}`);
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

/** 🗿️ The artifact tree a repository path lies in (`…/🗿️artifacts/<artifact>`), else `null` (framework and product trees): a leaf, its
 * aggregate and its fixtures never pair across two artifacts, so an orphaned fixture stays `unmapped` instead of borrowing a foreign
 * aggregate that happens to declare the same variant name. */
export function mutationArtifactScope(path: string): string | null {
  const segments = path.split("/");
  const artifacts = segments.indexOf("🗿️artifacts");
  return artifacts >= 0 && artifacts + 1 < segments.length ? segments.slice(0, artifacts + 2).join("/") : null;
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
  const leafBySegments = new Map(tree.leaves.map((leaf) => [`${leaf.root}|${leaf.directory.slice(leaf.root.length + 1)}`, leaf]));
  const leafByDirectory = new Map(tree.leaves.map((leaf) => [leaf.directory, leaf]));
  const nearest = <T,>(items: readonly T[], path: (item: T) => string, target: string): T[] => {
    const scope = mutationArtifactScope(target);
    const scored = items.filter((item) => mutationArtifactScope(path(item)) === scope).map((item) => [sharedSegments(path(item), target), item] as const);
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
        const leafSchema = documents.get(id);
        const pinsTag = (node: unknown): boolean => { const properties=isRecord(node)?node.properties:undefined;const property=isRecord(properties)&&tag!==null?properties[tag]:undefined;return isRecord(property)&&property.const===wireName; };
        const variants = isRecord(leafSchema) ? (Array.isArray(leafSchema.oneOf) ? leafSchema.oneOf : Array.isArray(leafSchema.anyOf) ? leafSchema.anyOf : []) : [];
        if (tag !== null && aggregate.layout.content === null && !(pinsTag(leafSchema) || (variants.length > 0 && variants.every(pinsTag)))) refuse(`the leaf of ${aggregate.name}::${variant} does not declare the tag ${JSON.stringify(tag)} as const ${JSON.stringify(wireName)}`);
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
    const shortened = (window: readonly string[]): MutationLeafRecord | undefined => {
      const root = `${owner}/🧬️schema/🧬️mutations`;
      const matches = tree.leaves.filter((candidate) => candidate.root === root && candidate.directory.slice(root.length + 1).startsWith(`${window.join("/")}-`));
      return matches.length === 1 ? matches[0] : undefined;
    };
    let leaf =
      leafByDirectory.get(owner) ??
      windows.map((window) => leafBySegments.get(`${owner}/🧬️schema/🧬️mutations|${window.join("/")}`)).find((found) => found !== undefined) ??
      windows.map(shortened).find((found) => found !== undefined) ??
      windows.map((window) => leafByKind.get(`${owner}/🧬️schema/🧬️mutations|${kindPath(window)}`)).find((found) => found !== undefined);
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

//#region 🐘️MutationCaps
/** 🐘️ One mutation leaf whose payload schema caps its inverse rows (`x-semio-inverse-rows.bounded`): its directory, the aggregate
 * variant its descriptor names and the cap. */
export type MutationCapLeaf = Readonly<{ directory: string; variant: string; bounded: number }>;

/** 🎩️ One `#[derive(Mutations)]` aggregate as the cap rule reads it: its source path, name, variants and whether it is generic. The
 * derive emits the `#[cfg(test)]` payload law (`semio_payload_law_<aggregate>`: every operation answers its schema-declared inverse
 * rows, every fixture case's inverse fits them, and a leaf past the store ceiling is refused `mutation.too-large`) only for a
 * non-generic aggregate. */
export type MutationCapAggregate = Readonly<{ path: string; name: string; variants: readonly string[]; generic: boolean }>;

/** 🪤️ A bounded leaf the derived cap law never runs over. */
export type MutationCapFinding = Readonly<{ code: "capLawMissing"; directory: string; detail: string }>;

/** 🧢️ The cap `x-semio-inverse-rows.bounded` of a leaf payload schema, or `null` when it declares none. */
export function mutationLeafBound(schema: Record<string, unknown>): number | null {
  const rows = schema["x-semio-inverse-rows"];
  return isRecord(rows) && typeof rows.bounded === "number" && Number.isInteger(rows.bounded) && rows.bounded > 0 ? rows.bounded : null;
}

/** 🧗️ Whether the enum `name` of a Rust source declares generic parameters. */
export function mutationAggregateGeneric(source: string, name: string): boolean {
  return new RegExp(`\\benum\\s+${name}\\s*<`, "u").test(source);
}

/** 🏔️ The structural cap rule (audit F3, coordinator decision 2026-10-05): every `bounded` leaf is wrapped by a `#[derive(Mutations)]`
 * aggregate that is not generic — the nearest one of its own artifact that names its variant, never a foreign artifact's — so the
 * derived payload law holds its declared rows. The per-leaf oracle is that Rust law; this rule only proves the law exists. */
export function mutationCapFindings(leaves: readonly MutationCapLeaf[], aggregates: readonly MutationCapAggregate[]): MutationCapFinding[] {
  return leaves.flatMap((leaf): MutationCapFinding[] => {
    const scope = mutationArtifactScope(leaf.directory);
    const scored = aggregates.filter((aggregate) => aggregate.variants.includes(leaf.variant) && mutationArtifactScope(aggregate.path) === scope).map((aggregate) => [sharedSegments(aggregate.path, leaf.directory), aggregate] as const);
    const best = Math.max(0, ...scored.map(([score]) => score));
    const nearest = scored.filter(([score]) => score === best && best > 0).map(([, aggregate]) => aggregate);
    if (nearest.length === 0) return [{ code: "capLawMissing", directory: leaf.directory, detail: `the leaf caps its inverse at ${leaf.bounded} row(s) but no #[derive(Mutations)] aggregate of its artifact wraps ${leaf.variant}: a hand-written aggregate runs no derived cap law` }];
    return nearest.every((aggregate) => aggregate.generic) ? [{ code: "capLawMissing", directory: leaf.directory, detail: `the leaf caps its inverse at ${leaf.bounded} row(s) but its aggregate ${nearest[0]!.name} is generic: the derive emits no payload law for a generic aggregate` }] : [];
  });
}

/** 📐️ The cap rule over every mutation leaf under `under`: the bounded leaves and those without the derived cap law. */
export function mutationCapReport(repoRoot: string, under = ""): { readonly leaves: readonly MutationCapLeaf[]; readonly findings: readonly MutationCapFinding[] } {
  const tree = mutationTree(repoRoot);
  const leaves = tree.leaves.filter((leaf) => leaf.directory.startsWith(under)).flatMap((leaf) => {
    const schema = readJsonObject(repoRoot, leaf.schemaPath);
    const bounded = schema === null ? null : mutationLeafBound(schema);
    return bounded === null ? [] : [{ directory: leaf.directory, variant: leaf.variant, bounded }];
  });
  const sources = new Map<string, string>();
  const aggregates = tree.aggregates.map((aggregate) => {
    if (!sources.has(aggregate.path)) sources.set(aggregate.path, readFileSync(join(repoRoot, aggregate.path), "utf8"));
    return { path: aggregate.path, name: aggregate.name, variants: [...aggregate.variants.keys()], generic: mutationAggregateGeneric(sources.get(aggregate.path)!, aggregate.name) };
  });
  return { leaves, findings: mutationCapFindings(leaves, aggregates) };
}
//#endregion 🐘️MutationCaps

//#region 🦀️RustSources
/** 🗂️ The roots the history gates read Rust from: the mutation roots plus the hub compositions. */
const RUST_SOURCE_ROOTS = [...MUTATION_TREE_ROOTS, "🌎️hub"];

/** 🔎️ Every `.rs` source under `under` whose text `keep` admits, repository-relative and sorted, with its text; build output, generated
 * and hidden directories are skipped. */
function rustSources(repoRoot: string, under: string, keep: (source: string) => boolean): { readonly path: string; readonly source: string }[] {
  return repositorySources(repoRoot, under, ".rs", keep);
}

/** 📂️ Every source ending in `extension` under `under` (else every Rust source root) whose text `keep` admits — [`rustSources`]' walk. */
function repositorySources(repoRoot: string, under: string, extension: string, keep: (source: string) => boolean): { readonly path: string; readonly source: string }[] {
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
 * text line used as a label (`labelOpText`), a label body the gate cannot read as one of the admitted shapes (`labelUnresolved`), and a
 * history-row label written by hand on an emission — `Emit::commit(mutations, label)` or an `Emit { description: … }` literal
 * (`labelHandwritten`, design §20.4). */
export type MutationLabelFindingClass = "labelOverride" | "labelLocaleInvariant" | "labelLocaleEmpty" | "labelOpText" | "labelUnresolved" | "labelHandwritten";

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
  return [...sites, ...rustEmitLabelSites(path, tokens)];
}

/** ✍️ Every hand-written history-row label of an emission among `tokens`, in source order (design §20.4): an `Emit::commit(…, label)`
 * call (implementor `commit`) and an `Emit { … description: <anything but None> … }` struct literal (implementor `description`, the
 * shorthand `description` included), the `Emit` path optionally qualified and turbofished; a destructuring pattern (`… } =`, `… } =>`)
 * is no literal. Each is `labelHandwritten`, with the label's first string literal as its text. */
export function rustEmitLabelSites(path: string, tokens: readonly RustToken[]): MutationLabelSite[] {
  const sites: MutationLabelSite[] = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index]!.kind !== "ident" || tokens[index]!.text !== "Emit") continue;
    let start = index;
    while (tokens[start - 1]?.text === ":" && tokens[start - 2]?.text === ":" && tokens[start - 3]?.kind === "ident") start -= 3;
    const before = tokens[start - 1];
    if (before !== undefined && (["struct", "enum", "impl", "for", "trait", "type", "use", "as"].includes(before.text) || (before.text === ">" && tokens[start - 2]?.text === "-"))) continue;
    let after = index + 1;
    if (tokens[after]?.text === ":" && tokens[after + 1]?.text === ":" && tokens[after + 2]?.text === "<") {
      let depth = 0;
      for (after += 2; after < tokens.length; after += 1) {
        if (tokens[after]!.text === "<") depth += 1;
        else if (tokens[after]!.text === ">" && tokens[after - 1]?.text !== "-" && --depth === 0) break;
      }
      after += 1;
    }
    if (tokens[after]?.text === ":" && tokens[after + 1]?.text === ":" && tokens[after + 2]?.text === "commit" && tokens[after + 3]?.text === "(") {
      sites.push({ path, trait: "Emit", implementor: "commit", verdict: "labelHandwritten", texts: [[rustFirstLiteral(rustCallArguments(tokens, after + 3)[1] ?? []), null]] });
    } else if (tokens[after]?.text === "{" && tokens[rustGroupEnd(tokens, after) + 1]?.text !== "=") {
      const close = rustGroupEnd(tokens, after);
      let depth = 0;
      for (let cursor = after + 1; cursor < close; cursor += 1) {
        const token = tokens[cursor]!;
        if (token.kind === "punct" && "([{".includes(token.text)) depth += 1;
        else if (token.kind === "punct" && ")]}".includes(token.text)) depth -= 1;
        if (depth !== 0 || token.text !== "description" || !["{", ","].includes(tokens[cursor - 1]?.text ?? "")) continue;
        if (cursor + 1 === close || tokens[cursor + 1]?.text === ",") {
          sites.push({ path, trait: "Emit", implementor: "description", verdict: "labelHandwritten", texts: [[null, null]] });
          continue;
        }
        if (tokens[cursor + 1]?.text !== ":" || tokens[cursor + 2]?.text === ":") continue;
        const value: RustToken[] = [];
        let nested = 0;
        for (let part = cursor + 2; part < close; part += 1) {
          const piece = tokens[part]!;
          if (piece.kind === "punct" && "([{".includes(piece.text)) nested += 1;
          else if (piece.kind === "punct" && ")]}".includes(piece.text)) nested -= 1;
          if (nested === 0 && piece.text === ",") break;
          value.push(piece);
        }
        if (!(value.length === 1 && value[0]!.text === "None")) sites.push({ path, trait: "Emit", implementor: "description", verdict: "labelHandwritten", texts: [[rustFirstLiteral(value), null]] });
      }
    }
  }
  return sites;
}

/**
 * 🏷️ Reads every label site of every Rust source under `under` (design §16.2): a history row is labelled by its leaf in every shell
 * locale, so each leaf label is `LocalizedLabel::native(en, de)` or a forward to one, no app overrides the leaf label, no operation's
 * text line is ever a label, and no emission labels its row by hand (§20.4). Every other verdict is one `schema-mutation-label` diagnostic.
 */
export function mutationLabelReport(repoRoot: string, under = ""): MutationLabelReport {
  const diagnostics: SchemaDiagnostic[] = [];
  const census = new Map<string, MutationLabelCensusRow>();
  const sites: MutationLabelSite[] = [];
  const relevant = (source: string): boolean => !source.includes("quote!") && (source.includes("fn label") || source.includes("fn mutation_label") || (source.includes("LocalizedLabel::data") && source.includes("print_op")) || (source.includes("Emit") && (source.includes("commit") || source.includes("description"))));
  for (const { path, source } of rustSources(repoRoot, under, relevant)) {
    if (path.split("/").includes("🧪️tests")) continue;
    for (const site of rustMutationLabelSites(path, source)) {
      sites.push(site);
      const owner = mutationPayloadOwner(path);
      const row = census.get(owner) ?? { owner, labels: 0, native: 0, forward: 0, findings: 0, refused: {} };
      census.set(owner, row);
      if (MUTATION_LABEL_TRAITS.has(site.trait)) row.labels += 1;
      if (site.verdict === "native") row.native += 1;
      else if (site.verdict === "forward") row.forward += 1;
      else {
        row.findings += 1;
        row.refused[site.verdict] = (row.refused[site.verdict] ?? 0) + 1;
        const subject = site.trait === "Emit" ? (site.implementor === "commit" ? "Emit::commit(mutations, label)" : "Emit { description: … }") : `${site.trait} for ${site.implementor || "a LocalizedLabel::data call"}`;
        diagnostics.push({ code: "schema-mutation-label", scope: null, export: null, format: null, path, detail: `${site.verdict}: ${subject}${site.texts.length > 0 ? ` ${JSON.stringify(site.texts)}` : ""}` });
      }
    }
  }
  return { diagnostics, census: [...census.values()].sort((left, right) => right.findings - left.findings || left.owner.localeCompare(right.owner)), sites };
}

/**
 * 🏷️ `test schema mutation-labels` — the `schema-mutation-label` gate: every applied leaf of every plugin labels itself in every shell
 * locale, no app overrides that label, no emission labels its row by hand, no history row falls back to an operation's text line.
 * `--census` prints the per-plugin table
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

//#region 📢️FaultNotices
/** 🚥️ The classes of a `schema-fault-notice` finding (design §20.12): a guest code no table labels (`faultNoticeMissing`), a guest code
 * that cannot be an app notice code (`faultNoticeSyntax`: fewer than three kebab segments), a guest code in a framework namespace the
 * framework table does not label (`faultNoticeFramework`), an English-only fault without a code (`faultAnonymous`: `Fault::from` of a
 * literal, `format!` or `to_string()`), a declared table breaking a rule (`faultNoticeInvalid`), a `fn fault_notices` body the gate
 * cannot read (`faultNoticeUnresolved`), a declared code no guest source emits (`faultNoticeStale`), and a committed descriptor whose
 * published codes differ from the sources (`faultNoticeDescriptor`, a `describe` is owed). */
export type FaultNoticeFindingClass = "faultNoticeMissing" | "faultNoticeSyntax" | "faultNoticeFramework" | "faultAnonymous" | "faultNoticeInvalid" | "faultNoticeUnresolved" | "faultNoticeStale" | "faultNoticeDescriptor";

/** 📍️ One fault code a guest source emits: `FaultCode::new|from("…")`, `Fault::new(origin, "…", …)`, `fault_from_error!(…, "…")`, a
 * code literal of a `fn code`/`fn fault_code` body (`codeFn`), a call of a refusal helper (`faultHelper`, see {@link rustFaultHelpers}),
 * or an anonymous `Fault::from(…)` (`code` null, `text` its string literal if it has one); `within` names the innermost `fn` around it
 * (`null` outside any), `defaulted` tells whether that `fn` is a trait's default method — the capability-absent contract a host never
 * reaches through an implementor that overrides it. Sites in `#[cfg(test)]`-gated items are not guest sites and are never read. */
export type FaultCodeSite = Readonly<{ path: string; code: string | null; via: "faultCode" | "faultNew" | "faultMacro" | "codeFn" | "faultHelper" | "faultFrom"; text: string | null; within: string | null; defaulted: boolean }>;

/** 🧰️ A refusal helper: the position of the parameter its fault code comes from, or the one fixed code it always refuses with. */
export type FaultHelper = number | string;

/** 📋️ One `fn fault_notices` of an `ArtifactApp`/`ArtifactEditor`/`ArtifactViewer` impl: its implementor and the `(code, en, de)` rows
 * read from `LocalizedLabel::native` tuples (a code is a string literal or a `const NAME: &str` of the plugin) — or, for a body that
 * composes other tables (`forwards`: one `name()` call, or several chained `…fault_notices()` calls), the union of their rows, resolved
 * among the plugin's sources — or `null` when the table is not readable as that shape. */
export type FaultNoticeDeclaration = Readonly<{ path: string; implementor: string; forwards: readonly string[]; notices: readonly Readonly<{ code: string; en: string; de: string }>[] | null }>;

/** 📈️ One plugin's share of the fault-notice census: emitted codes, labelled ones, declared notices and findings per class. */
export type FaultNoticeCensusRow = { readonly owner: string; codes: number; labelled: number; declared: number; anonymous: number; findings: number; readonly refused: Record<string, number> };

/** 📰️ The `schema-fault-notice` lint, its census, every emitted code and every declared table read. */
export type FaultNoticeReport = { readonly diagnostics: readonly SchemaDiagnostic[]; readonly census: readonly FaultNoticeCensusRow[]; readonly sites: readonly FaultCodeSite[]; readonly declarations: readonly FaultNoticeDeclaration[] };

const FAULT_NOTICE_APP_TRAITS = new Set(["ArtifactApp", "ArtifactEditor", "ArtifactViewer"]);
/** 🏛️ Code namespaces the framework owns: a guest refusal in one of them is labelled by the framework's table or by no one. */
const FAULT_NOTICE_FRAMEWORK_NAMESPACES = new Set(["app", "plugin", "os", "framework", "module", "mutation", "viewer", "surface", "history", "timeTravel", "toolTransaction", "document", "pure"]);
const FAULT_NOTICE_PLUGINS_ROOT = "✏️s/🔌️plugins";
/** 🔌️ The plugin SDK every guest links: its refusals reach the person like a plugin's own, under the owner `🔌️plugin`; every code it
 * raises is a framework code (labelled by the framework's tables or by no one), and it declares no notice table of its own. */
const FAULT_NOTICE_SDK_ROOT = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const FAULT_NOTICE_SDK_OWNER = "🔌️plugin";
/** 🫙️ Catch-all codes that name no refusal: a site raising one is an anonymous fault. */
const FAULT_NOTICE_CATCH_ALL = new Set(["plugin.internal"]);
/** 🗄️ The framework's notice tables (history-lane refusals, framework-namespace refusals any app raises): a code they declare is labelled. */
const FAULT_NOTICE_FRAMEWORK_TABLES = ["🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/🧫️history-notices/🔣️.json", "🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/🧫️framework-notices/🔣️.json"] as const;

/** 🔭️ The findings `schema fault-notices` reports: `all`, or `history-editing` — the refusals a person meets while editing history,
 * loading a document or running a tool (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING scope): every framework-namespace code a plugin
 * raises, every SDK code and every other code or anonymous fault whose code or text has a tool-flow segment or one of whose live sites
 * is a tool-flow site ({@link faultSiteInToolFlow}), and every verdict on a declared table or descriptor ({@link faultNoticeInScope}). */
export type FaultNoticeScope = "all" | "history-editing";
const FAULT_NOTICE_TOOL_FLOW_CODE = /(?:^|[.-])(?:tool|tools|gesture|gumball|drag|scrub|press|stroke|transaction|history|replay|ink|timeTravel|toolRun|toolTransaction)(?:[.-]|$)/u;
const FAULT_NOTICE_TOOL_FLOW_PATH = /🛠️|tool|gesture|gumball|scrub|press|transaction|time-travel/iu;
const FAULT_NOTICE_TOOL_FLOW_FN = /(?:^|_)(?:tool|tools|gesture|gumball|drag|scrub|press|stroke|transaction|history|replay|ink|time_travel)(?:_|$)|(?:^|_)(?:document|archive|envelope|retained)_load(?:_|$)|(?:^|_)load_(?:document|archive|envelope)(?:_|$)/u;
const FAULT_NOTICE_TOOL_FLOW_SOURCE = /\bChildEmit\b|\bEmit::[a-z_]*drag[a-z_]*\b/u;

/** 🪪️ What {@link faultNoticeInScope} weighs for one finding: the code or anonymous text it names, whether one of its live sites is a
 * tool-flow site, whether the SDK raises it, and whether any of its sites is live (outside a trait's default method). */
export type FaultNoticeSubject = Readonly<{ named: string | null; toolFlow: boolean; sdk: boolean; live: boolean }>;
const FAULT_NOTICE_DECLARED_SUBJECT: FaultNoticeSubject = { named: null, toolFlow: false, sdk: false, live: true };

/** 🧲️ Whether one finding belongs to `scope` — see {@link FaultNoticeScope}: under `history-editing` a code or anonymous fault is kept
 * when it is live and in the flow (a tool-flow site or a tool-flow code or text), a plugin's framework-namespace code whenever it is
 * live, and every verdict on a declared table or descriptor always. */
export function faultNoticeInScope(scope: FaultNoticeScope, verdict: FaultNoticeFindingClass, subject: FaultNoticeSubject = FAULT_NOTICE_DECLARED_SUBJECT): boolean {
  if (scope === "all") return true;
  const flow = subject.live && (subject.toolFlow || (subject.named !== null && FAULT_NOTICE_TOOL_FLOW_CODE.test(subject.named)));
  switch (verdict) {
    case "faultNoticeFramework":
      return subject.sdk ? flow : subject.live;
    case "faultNoticeMissing":
    case "faultNoticeSyntax":
    case "faultAnonymous":
      return flow;
    default:
      return true;
  }
}

/** 🛤️ Whether a site is a tool-flow site: its path has a tool-flow segment, it sits inside a tool-flow `fn` (`build_tool_job`,
 * `begin_gesture`, `apply_time_travel_action`, …) or a document-load `fn` (`begin_document_archive_load`, `advance_artifact_envelope_load`,
 * …), or its plugin source publishes a child emit or a drag emit (`ChildEmit`, `Emit::*drag*`). */
function faultSiteInToolFlow(site: FaultCodeSite, emitting: ReadonlySet<string>): boolean {
  return emitting.has(site.path) || (site.within !== null && FAULT_NOTICE_TOOL_FLOW_FN.test(site.within)) || site.path.split("/").some((segment) => FAULT_NOTICE_TOOL_FLOW_PATH.test(segment));
}
const DOTTED_CODE = /^[A-Za-z][\w-]*(?:\.[A-Za-z0-9][\w-]*)+$/u;

/** 🔡️ The value of one Rust string literal token: quotes and raw hashes stripped, the common escapes resolved. */
export function rustStringValue(literal: string): string {
  const raw = /^b?r(#*)"/u.exec(literal);
  if (raw !== null) return literal.slice(raw[0].length, literal.length - 1 - raw[1]!.length);
  return literal
    .replace(/^b?"|"$/gu, "")
    .replace(/\\u\{([0-9a-fA-F]+)\}|\\(["'\\nrt0])/gu, (_, hex: string | undefined, escaped: string | undefined) => (hex !== undefined ? String.fromCodePoint(Number.parseInt(hex, 16)) : ({ n: "\n", r: "\r", t: "\t", "0": "\0" } as Record<string, string>)[escaped!] ?? escaped!));
}

/** 🔖️ Every `const NAME: &str = "…";` (also `&'static str`, any visibility) of one Rust source, by name. */
export function rustStringConsts(source: string): Map<string, string> {
  const tokens = rustLex(source);
  const consts = new Map<string, string>();
  for (let index = 0; index + 1 < tokens.length; index += 1) {
    if (tokens[index]!.text !== "const" || tokens[index + 1]?.kind !== "ident" || tokens[index + 2]?.text !== ":") continue;
    let at = index + 3;
    while (at < tokens.length && tokens[at]!.text !== "=" && tokens[at]!.text !== ";") at += 1;
    const type = tokens.slice(index + 3, at).map((token) => token.text).join("");
    if (!/^&(?:'static)?str$/u.test(type) || tokens[at]?.text !== "=" || tokens[at + 1]?.kind !== "literal" || tokens[at + 2]?.text !== ";" || !/^b?r?#*"/u.test(tokens[at + 1]!.text)) continue;
    consts.set(tokens[index + 1]!.text, rustStringValue(tokens[at + 1]!.text));
  }
  return consts;
}

/** 🪝️ The refusal helpers of one Rust source, by name — free `fn`s (no `self`), nested ones included: one whose body builds its fault
 * from a parameter (`FaultCode::new|from(code)` or `Fault::new(origin, code, …)`) is that parameter's position; one returning `Fault`
 * whose body builds exactly one fault, from a string literal or a named `const` of `consts`, is that fixed code. */
export function rustFaultHelpers(source: string, consts: ReadonlyMap<string, string> = new Map()): Map<string, FaultHelper> {
  const tokens = rustLex(source);
  const helpers = new Map<string, FaultHelper>();
  for (const fn of rustFnBodies(tokens)) {
    const params = rustCallArguments(tokens, fn.params).map((param) => (param[0]?.kind === "ident" && param[1]?.text === ":" && param[2]?.text !== ":" ? param[0].text : null));
    if (rustCallArguments(tokens, fn.params)[0]?.some((token) => token.text === "self")) continue;
    const codeArgs: (readonly RustToken[])[] = [];
    for (let index = fn.open; index < fn.close; index += 1) {
      const codeArg = (rustPathAt(tokens, index, ["FaultCode", "new"]) || rustPathAt(tokens, index, ["FaultCode", "from"])) && tokens[index + 4]?.text === "(" ? rustCallArguments(tokens, index + 4)[0] : rustPathAt(tokens, index, ["Fault", "new"]) && tokens[index + 4]?.text === "(" ? rustCallArguments(tokens, index + 4)[1] : undefined;
      if (codeArg !== undefined && codeArg.every((token) => token.text !== "FaultCode")) codeArgs.push(codeArg);
    }
    const position = codeArgs.map((arg) => (arg.length === 1 && arg[0]!.kind === "ident" ? params.indexOf(arg[0]!.text) : -1)).find((at) => at >= 0);
    const returnsFault = /^->(?:\w+::)*Fault$/u.test(tokens.slice(rustGroupEnd(tokens, fn.params) + 1, fn.open).map((token) => token.text).join(""));
    const fixed = codeArgs.length === 1 && codeArgs[0]!.length === 1 ? (codeArgs[0]![0]!.kind === "literal" && /^b?r?#*"/u.test(codeArgs[0]![0]!.text) ? rustStringValue(codeArgs[0]![0]!.text) : (consts.get(codeArgs[0]![0]!.text) ?? null)) : null;
    if (position !== undefined) helpers.set(fn.name, position);
    else if (returnsFault && fixed !== null && DOTTED_CODE.test(fixed)) helpers.set(fn.name, fixed);
  }
  return helpers;
}

/** 🗺️ Every `fn` with a body in a token stream — methods and nested ones included — by name with the `(` of its parameters, its `{…}`
 * span and whether it is a trait's default method (its innermost enclosing block is a `trait` body), in source order. */
function rustFnBodies(tokens: readonly RustToken[]): { readonly name: string; readonly params: number; readonly open: number; readonly close: number; readonly defaulted: boolean }[] {
  const traits: (readonly [number, number])[] = [];
  const bodies: { readonly name: string; readonly params: number; readonly open: number; readonly close: number; defaulted: boolean }[] = [];
  for (let at = 0; at + 1 < tokens.length; at += 1) {
    if (tokens[at]!.text === "trait" && tokens[at + 1]!.kind === "ident") {
      let open = at + 2;
      while (open < tokens.length && tokens[open]!.text !== "{" && tokens[open]!.text !== ";") open += 1;
      if (tokens[open]?.text === "{") traits.push([open, rustGroupEnd(tokens, open)]);
    }
    if (tokens[at]!.text !== "fn" || tokens[at + 1]!.kind !== "ident") continue;
    let params = at + 2;
    while (params < tokens.length && tokens[params]!.text !== "(") params += 1;
    let open = rustGroupEnd(tokens, params) + 1;
    while (open < tokens.length && tokens[open]!.text !== "{" && tokens[open]!.text !== ";") open += 1;
    if (tokens[open]?.text === "{") bodies.push({ name: tokens[at + 1]!.text, params, open, close: rustGroupEnd(tokens, open), defaulted: false });
  }
  for (const body of bodies) {
    const trait = traits.findLast(([open, close]) => open < body.open && body.close <= close);
    body.defaulted = trait !== undefined && !bodies.some((outer) => outer !== body && trait[0] < outer.open && outer.open < body.open && body.close <= outer.close);
  }
  return bodies;
}

/** 🧪️ The token spans of every `#[cfg(…test…)]`-gated item of a token stream (`cfg(test)`, `cfg(any(test, …))`, not `cfg(not(test))`) —
 * test code, never a guest source; an inner `#![cfg(test)]` gates the whole source. */
function rustTestOnlySpans(tokens: readonly RustToken[]): (readonly [number, number])[] {
  const spans: (readonly [number, number])[] = [];
  const testOnly = (open: number): boolean => {
    const attribute = tokens.slice(open + 1, rustGroupEnd(tokens, open)).map((token) => token.text).join("");
    return /^cfg\(/u.test(attribute) && /\btest\b/u.test(attribute) && !/not\(test\)/u.test(attribute);
  };
  for (let at = 0; at + 2 < tokens.length; at += 1) {
    if (tokens[at]!.text !== "#") continue;
    if (tokens[at + 1]!.text === "!" && tokens[at + 2]!.text === "[" && testOnly(at + 2)) return [[0, tokens.length]];
    if (tokens[at + 1]!.text !== "[" || !testOnly(at + 1)) continue;
    let item = rustGroupEnd(tokens, at + 1) + 1;
    while (tokens[item]?.text === "#" && tokens[item + 1]?.text === "[") item = rustGroupEnd(tokens, item + 1) + 1;
    while (item < tokens.length && tokens[item]!.text !== "{" && tokens[item]!.text !== ";") item = (tokens[item]!.text === "(" || tokens[item]!.text === "[" ? rustGroupEnd(tokens, item) : item) + 1;
    spans.push([at, tokens[item]?.text === "{" ? rustGroupEnd(tokens, item) : item]);
  }
  return spans;
}

/** 🎯️ Every fault code site of one Rust source, in source order — see {@link FaultCodeSite}; a code argument is a string literal or a
 * (path to a) `const` named in `consts`; a call of a refusal helper in `helpers` carries its code at the helper's position; a
 * `fn code`/`fn fault_code` is read in any impl, inherent ones included (a named refusal enum's `fn code(&self) -> &'static str`),
 * literals and named consts alike; a site inside a `#[cfg(test)]`-gated item is not read. */
export function rustFaultCodeSites(path: string, source: string, consts: ReadonlyMap<string, string> = new Map(), helpers: ReadonlyMap<string, FaultHelper> = new Map()): FaultCodeSite[] {
  const tokens = rustLex(source);
  const bodies = rustFnBodies(tokens);
  const tests = rustTestOnlySpans(tokens);
  const sites: (Omit<FaultCodeSite, "within" | "defaulted"> & { readonly at: number })[] = [];
  const literal = (arg: readonly RustToken[] | undefined): string | null => {
    if (arg === undefined) return null;
    if (arg.length === 1 && arg[0]!.kind === "literal" && /^b?r?#*"/u.test(arg[0]!.text)) return rustStringValue(arg[0]!.text);
    const last = arg.at(-1);
    return last?.kind === "ident" && arg.every((token, at) => (at % 3 === 0 ? token.kind === "ident" : token.text === ":")) ? (consts.get(last.text) ?? null) : null;
  };
  for (let index = 0; index < tokens.length; index += 1) {
    if ((rustPathAt(tokens, index, ["FaultCode", "new"]) || rustPathAt(tokens, index, ["FaultCode", "from"])) && tokens[index + 4]?.text === "(") {
      const code = literal(rustCallArguments(tokens, index + 4)[0]);
      if (code !== null) sites.push({ path, code, via: "faultCode", text: null, at: index });
    } else if (rustPathAt(tokens, index, ["Fault", "new"]) && tokens[index + 4]?.text === "(") {
      const code = literal(rustCallArguments(tokens, index + 4)[1]);
      if (code !== null) sites.push({ path, code, via: "faultNew", text: null, at: index });
    } else if (rustPathAt(tokens, index, ["Fault", "from"]) && tokens[index + 4]?.text === "(") {
      const arg = rustCallArguments(tokens, index + 4)[0] ?? [];
      const text = arg.map((token) => token.text).join("");
      if (literal(arg) !== null || /^format!/u.test(text) || /\.(to_string|to_owned)\(\)$/u.test(text)) sites.push({ path, code: null, via: "faultFrom", text: literal(arg), at: index });
    } else if (tokens[index]!.text === "fault_from_error" && tokens[index + 1]?.text === "!" && tokens[index + 2]?.text === "(") {
      const code = literal(rustCallArguments(tokens, index + 2)[2]);
      if (code !== null) sites.push({ path, code, via: "faultMacro", text: null, at: index });
    } else if (tokens[index]!.kind === "ident" && helpers.has(tokens[index]!.text) && tokens[index + 1]?.text === "(" && tokens[index - 1]?.text !== "fn" && tokens[index - 1]?.text !== ".") {
      const helper = helpers.get(tokens[index]!.text)!;
      const code = typeof helper === "string" ? helper : literal(rustCallArguments(tokens, index + 1)[helper]);
      if (code !== null) sites.push({ path, code, via: "faultHelper", text: null, at: index });
    }
  }
  for (const body of bodies.filter((candidate) => candidate.name === "code" || candidate.name === "fault_code")) {
    for (let index = body.open; index < body.close; index += 1) {
      const token = tokens[index]!;
      if (rustPathAt(tokens, index - 4, ["FaultCode", "new"]) || rustPathAt(tokens, index - 4, ["FaultCode", "from"])) continue;
      const code = token.kind === "literal" && /^b?r?#*"/u.test(token.text) ? rustStringValue(token.text) : token.kind === "ident" ? (consts.get(token.text) ?? null) : null;
      if (code !== null && DOTTED_CODE.test(code)) sites.push({ path, code, via: "codeFn", text: null, at: index });
    }
  }
  const within = (at: number) => bodies.findLast((body) => body.open < at && at < body.close);
  return sites
    .filter((site) => !tests.some(([open, close]) => open <= site.at && site.at <= close))
    .sort((left, right) => left.at - right.at)
    .map(({ at, ...site }) => ({ ...site, within: within(at)?.name ?? null, defaulted: within(at)?.defaulted ?? false }));
}

/** 🧮️ The `(code, en, de)` rows of one table body `tokens[open..close]`: each `LocalizedLabel::native(en, de)` call preceded by `"code",`
 * or by `NAME,` of a const in `consts` is one row; `null` when a call is not that shape or the body names none. */
function rustFaultNoticeRows(tokens: readonly RustToken[], open: number, close: number, consts: ReadonlyMap<string, string> = new Map()): { code: string; en: string; de: string }[] | null {
  const notices: { code: string; en: string; de: string }[] = [];
  for (let index = open; index < close; index += 1) {
    if (!rustPathAt(tokens, index, ["LocalizedLabel", "native"]) || tokens[index + 4]?.text !== "(") continue;
    let start = index;
    while (tokens[start - 1]?.text === ":" && tokens[start - 2]?.text === ":" && tokens[start - 3]?.kind === "ident") start -= 3;
    const texts = rustCallArguments(tokens, index + 4).map((arg) => (arg.length === 1 && arg[0]!.kind === "literal" ? rustStringValue(arg[0]!.text) : null));
    const key = tokens[start - 1]?.text === "," ? tokens[start - 2] : undefined;
    const code = key?.kind === "literal" ? rustStringValue(key.text) : key?.kind === "ident" ? (consts.get(key.text) ?? null) : null;
    if (code === null || texts.length !== 2 || texts.some((text) => text === null)) return null;
    notices.push({ code, en: texts[0]!, de: texts[1]! });
  }
  return notices.length > 0 ? notices : null;
}

/** 📜️ Every `fn fault_notices` declaration of one Rust source — see {@link FaultNoticeDeclaration}: an empty `&[]` body declares none, a
 * body that is one call `path::name()` forwards to `name`, a body without rows of its own that chains `…fault_notices()` calls forwards to
 * each of them (rows resolved by {@link rustFaultNoticeTable}), any other body is a table; an impl in a `#[cfg(test)]`-gated item is
 * not read. */
export function rustFaultNoticeDeclarations(path: string, source: string, consts: ReadonlyMap<string, string> = new Map()): FaultNoticeDeclaration[] {
  const tokens = rustLex(source);
  const tests = rustTestOnlySpans(tokens);
  const declarations: FaultNoticeDeclaration[] = [];
  for (const block of rustImplBlocks(tokens)) {
    if (!FAULT_NOTICE_APP_TRAITS.has(block.trait) || tests.some(([open, close]) => open <= block.open && block.open <= close)) continue;
    for (const fn of rustFunctions(tokens, block.open, block.close)) {
      if (fn.body === null || fn.name !== "fault_notices") continue;
      const [open, close] = fn.body;
      const body = tokens.slice(open + 1, close).map((token) => token.text).join("");
      const single = /^(?:[\p{L}\p{N}_]+::)*([\p{L}_][\p{L}\p{N}_]*)\(\)$/u.exec(body)?.[1];
      const rows = single === undefined && body !== "&[]" ? rustFaultNoticeRows(tokens, open, close, consts) : null;
      const chained = tokens.slice(open + 1, close).flatMap((token, at, all) => (token.kind === "ident" && token.text.endsWith("fault_notices") && all[at + 1]?.text === "(" && all[at + 2]?.text === ")" ? [token.text] : []));
      const forwards = single !== undefined ? [single] : rows === null && !body.includes("LocalizedLabel::native") ? [...new Set(chained)] : [];
      declarations.push({ path, implementor: block.implementor, forwards, notices: body === "&[]" ? [] : forwards.length > 0 ? null : rows });
    }
  }
  return declarations;
}

/** 📑️ The rows of the free function `name` of one Rust source (a forwarded table), or `undefined` when the source defines none. */
export function rustFaultNoticeTable(source: string, name: string, consts: ReadonlyMap<string, string> = new Map()): { code: string; en: string; de: string }[] | null | undefined {
  const tokens = rustLex(source);
  const fn = rustFunctions(tokens, -1, tokens.length).find((candidate) => candidate.name === name && candidate.body !== null);
  return fn === undefined ? undefined : rustFaultNoticeRows(tokens, fn.body![0], fn.body![1], consts);
}

/** 🧩️ The plugin directory segment of a repository-relative path under `✏️s/🔌️plugins`, or `null`. */
function faultNoticePlugin(path: string): string | null {
  if (path.startsWith(`${FAULT_NOTICE_SDK_ROOT}/`)) return FAULT_NOTICE_SDK_OWNER;
  const segments = path.split("/");
  const plugins = segments.indexOf("🔌️plugins");
  return plugins >= 0 ? (segments[plugins + 1] ?? null) : null;
}

/** 🏘️ The crate-sized unit consts, refusal helpers and forwarded tables resolve within: the `🗿️artifacts/<artifact>` root of a path, else
 * its plugin directory. */
function faultNoticeUnit(path: string): string {
  if (path.startsWith(`${FAULT_NOTICE_SDK_ROOT}/`)) return FAULT_NOTICE_SDK_ROOT;
  const segments = path.split("/");
  const artifacts = segments.indexOf("🗿️artifacts");
  return artifacts >= 0 && segments[artifacts + 1] !== undefined ? segments.slice(0, artifacts + 2).join("/") : segments.slice(0, segments.indexOf("🔌️plugins") + 2).join("/");
}

/** 🗣️ A declared `(code, en, de)` row as the definition its descriptor publishes (`LocalizedLabel::native` is terminology-invariant). */
function faultNoticeDefinition(row: Readonly<{ code: string; en: string; de: string }>): FaultNoticeDefinition {
  return { code: row.code, label: { native: { en: row.en, de: row.de }, reuse: { en: row.en, de: row.de } } };
}

/**
 * 📢️ Reads every guest fault code and every declared notice table of every plugin under `under` (design §20.12): each code a guest
 * emits is labelled by the framework's tables ({@link FAULT_NOTICE_FRAMEWORK_TABLES}) or by one of its plugin's `fault_notices`
 * tables in every locale; every declared table is valid (`validateFaultNotices`) and names only emitted codes; every committed
 * descriptor (`🌎️hub/🧩️compositions/<plugin>/🔣️.json`) publishes exactly the declared codes, each table valid. Every other verdict
 * in `scope` ({@link faultNoticeInScope}) is one `schema-fault-notice` diagnostic.
 */
export function faultNoticeReport(repoRoot: string, under = "", scope: FaultNoticeScope = "all"): FaultNoticeReport {
  const diagnostics: SchemaDiagnostic[] = [];
  const census = new Map<string, FaultNoticeCensusRow>();
  const sites: FaultCodeSite[] = [];
  const declarations: FaultNoticeDeclaration[] = [];
  const framework = new Set(FAULT_NOTICE_FRAMEWORK_TABLES.flatMap((table) => ((readJsonObject(repoRoot, table)?.notices ?? []) as { readonly code: string }[]).map((notice) => notice.code)));
  const relevant = (source: string): boolean => source.includes("Fault") || source.includes("fn code") || source.includes("fault_notices");
  const sources: { readonly path: string; readonly source: string }[] = [];
  for (const root of under === "" ? [FAULT_NOTICE_PLUGINS_ROOT, FAULT_NOTICE_SDK_ROOT] : [under]) {
    for (const { path, source } of rustSources(repoRoot, root, relevant)) if (!path.split("/").includes("🧪️tests") && faultNoticePlugin(path) !== null) sources.push({ path, source });
  }
  const unitConsts = new Map<string, Map<string, string | null>>();
  const unitHelpers = new Map<string, Map<string, FaultHelper | null>>();
  for (const { path, source } of sources) {
    const consts = unitConsts.get(faultNoticeUnit(path)) ?? new Map<string, string | null>();
    unitConsts.set(faultNoticeUnit(path), consts);
    for (const [name, value] of rustStringConsts(source)) consts.set(name, consts.has(name) && consts.get(name) !== value ? null : value);
  }
  const resolved = <T,>(table: Map<string, T | null> | undefined): Map<string, T> => new Map([...(table ?? [])].filter((entry): entry is [string, T] => entry[1] !== null));
  const constsOf = (path: string): Map<string, string> => resolved(unitConsts.get(faultNoticeUnit(path)));
  for (const { path, source } of sources) {
    const helpers = unitHelpers.get(faultNoticeUnit(path)) ?? new Map<string, FaultHelper | null>();
    unitHelpers.set(faultNoticeUnit(path), helpers);
    for (const [name, helper] of rustFaultHelpers(source, constsOf(path))) helpers.set(name, helpers.has(name) && helpers.get(name) !== helper ? null : helper);
  }
  const helpersOf = (path: string): Map<string, FaultHelper> => resolved(unitHelpers.get(faultNoticeUnit(path)));
  const emitting = new Set(sources.filter(({ path, source }) => faultNoticePlugin(path) !== FAULT_NOTICE_SDK_OWNER && FAULT_NOTICE_TOOL_FLOW_SOURCE.test(source)).map(({ path }) => path));
  for (const { path, source } of sources) sites.push(...rustFaultCodeSites(path, source, constsOf(path), helpersOf(path)));
  for (const { path, source } of sources) {
    const consts = constsOf(path);
    for (const declaration of faultNoticePlugin(path) === FAULT_NOTICE_SDK_OWNER ? [] : rustFaultNoticeDeclarations(path, source, consts)) {
      const holders = (sameUnit: boolean) => sources.filter((candidate) => (sameUnit ? faultNoticeUnit(candidate.path) === faultNoticeUnit(path) : faultNoticePlugin(candidate.path) === faultNoticePlugin(path)) && candidate.source.includes("fn "));
      const table = (forward: string): { code: string; en: string; de: string }[] | null => [...holders(true), ...holders(false)].filter((candidate) => candidate.source.includes(`fn ${forward}`)).map((candidate) => rustFaultNoticeTable(candidate.source, forward, constsOf(candidate.path))).find((rows) => rows !== undefined) ?? null;
      const tables = declaration.forwards.map(table);
      const notices = declaration.forwards.length === 0 ? declaration.notices : tables.some((rows) => rows === null) ? null : tables.flatMap((rows) => rows!);
      declarations.push({ ...declaration, notices });
    }
  }
  const row = (owner: string): FaultNoticeCensusRow => {
    const found = census.get(owner) ?? { owner, codes: 0, labelled: 0, declared: 0, anonymous: 0, findings: 0, refused: {} };
    census.set(owner, found);
    return found;
  };
  const finding = (owner: string, verdict: FaultNoticeFindingClass, path: string, detail: string, subject: FaultNoticeSubject = FAULT_NOTICE_DECLARED_SUBJECT): void => {
    if (!faultNoticeInScope(scope, verdict, subject)) return;
    const target = row(owner);
    target.findings += 1;
    target.refused[verdict] = (target.refused[verdict] ?? 0) + 1;
    diagnostics.push({ code: "schema-fault-notice", scope: null, export: null, format: null, path, detail: `${verdict}: ${detail}` });
  };
  const declared = new Map<string, Set<string>>();
  for (const declaration of declarations) {
    const plugin = faultNoticePlugin(declaration.path)!;
    const codes = declared.get(plugin) ?? new Set<string>();
    declared.set(plugin, codes);
    if (declaration.notices === null) {
      finding(plugin, "faultNoticeUnresolved", declaration.path, `fn fault_notices of ${declaration.implementor}${declaration.forwards.length === 0 ? "" : ` (forward to ${declaration.forwards.join(", ")})`} is not a table of (code, LocalizedLabel::native(en, de)) rows`);
      continue;
    }
    row(plugin).declared += declaration.notices.length;
    for (const notice of declaration.notices) codes.add(notice.code);
    for (const error of validateFaultNotices(declaration.notices.map(faultNoticeDefinition))) finding(plugin, "faultNoticeInvalid", declaration.path, `${error.rule} ${error.code}${"locale" in error ? ` (${error.terminology}/${error.locale})` : ""} in ${declaration.implementor}`);
  }
  const emitted = new Map<string, Set<string>>();
  const told = new Set<string>();
  const flows = new Map<string, { toolFlow: boolean; live: boolean }>();
  for (const site of sites) {
    if (site.code === null) continue;
    const key = `${faultNoticePlugin(site.path)!}\u0000${site.code}`;
    const flow = flows.get(key) ?? { toolFlow: false, live: false };
    flows.set(key, { toolFlow: flow.toolFlow || (!site.defaulted && faultSiteInToolFlow(site, emitting)), live: flow.live || !site.defaulted });
  }
  for (const site of sites) {
    const plugin = faultNoticePlugin(site.path)!;
    const sdk = plugin === FAULT_NOTICE_SDK_OWNER;
    if (site.code === null || FAULT_NOTICE_CATCH_ALL.has(site.code)) {
      if (site.code !== null && typeof helpersOf(site.path).get(site.within ?? "") === "string") continue;
      row(plugin).anonymous += 1;
      finding(plugin, "faultAnonymous", site.path, site.code !== null ? `${site.code} (${site.via}) names no refusal — name it and declare its notice` : `Fault::from(${site.text === null ? "text" : JSON.stringify(site.text)}) carries no code — name the refusal and declare its notice`, { named: site.text, toolFlow: faultSiteInToolFlow(site, emitting), sdk, live: !site.defaulted });
      continue;
    }
    const codes = emitted.get(plugin) ?? new Set<string>();
    emitted.set(plugin, codes);
    if (codes.has(site.code)) continue;
    codes.add(site.code);
    row(plugin).codes += 1;
    if (framework.has(site.code) || declared.get(plugin)?.has(site.code)) {
      row(plugin).labelled += 1;
      continue;
    }
    const key = `${plugin}\u0000${site.code}`;
    if (told.has(key)) continue;
    told.add(key);
    const namespace = site.code.split(".")[0]!;
    finding(plugin, sdk || FAULT_NOTICE_FRAMEWORK_NAMESPACES.has(namespace) ? "faultNoticeFramework" : isFaultNoticeCode(site.code) ? "faultNoticeMissing" : "faultNoticeSyntax", site.path, `${site.code} (${site.via})`, { named: site.code, sdk, ...flows.get(key)! });
  }
  for (const [plugin, codes] of declared) for (const code of codes) if (!emitted.get(plugin)?.has(code)) finding(plugin, "faultNoticeStale", declarations.find((declaration) => faultNoticePlugin(declaration.path) === plugin)!.path, `${code} is declared but no guest source of ${plugin} emits it`);
  for (const plugin of new Set([...census.keys()])) {
    const path = `🌎️hub/🧩️compositions/${plugin}/🔣️.json`;
    const descriptor = readJsonObject(repoRoot, path);
    if (descriptor === null || (under !== "" && !under.startsWith(`${FAULT_NOTICE_PLUGINS_ROOT}/${plugin}`) && under !== FAULT_NOTICE_PLUGINS_ROOT)) continue;
    const apps = ((descriptor.manifest as { readonly apps?: readonly { readonly id?: string; readonly faultNotices?: readonly FaultNoticeDefinition[] }[] } | undefined)?.apps ?? []);
    const published = new Set(apps.flatMap((app) => (app.faultNotices ?? []).map((notice) => notice.code)));
    for (const app of apps) for (const error of validateFaultNotices(app.faultNotices ?? [])) finding(plugin, "faultNoticeInvalid", path, `${error.rule} ${error.code}${"locale" in error ? ` (${error.terminology}/${error.locale})` : ""} in published app ${app.id ?? "?"}`);
    const sources = declared.get(plugin) ?? new Set<string>();
    const missing = [...sources].filter((code) => !published.has(code)).sort();
    const extra = [...published].filter((code) => !sources.has(code)).sort();
    if (missing.length + extra.length > 0) finding(plugin, "faultNoticeDescriptor", path, `describe owed — unpublished ${JSON.stringify(missing)}, no longer declared ${JSON.stringify(extra)}`);
  }
  return { diagnostics, census: [...census.values()].sort((left, right) => right.findings - left.findings || left.owner.localeCompare(right.owner)), sites, declarations };
}

/**
 * 🖥️ `test schema fault-notices` — the `schema-fault-notice` gate (design §20.12): every fault code a guest can refuse with reaches the
 * person as a notice in every shell locale, never as a raw code. `--census` prints the per-plugin table (owner, distinct codes, labelled
 * ones, declared notices, anonymous faults, findings per class) and always exits 0; without it any finding fails. `--scope
 * history-editing` keeps the refusals of the history-editing and tool flows ({@link faultNoticeInScope}).
 *
 *   bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts schema fault-notices [--census] [--under <path>] [--scope all|history-editing] [--json]
 */
function runFaultNotices(repoRoot: string, segments: string[]): never {
  const under = segments.includes("--under") ? (segments[segments.indexOf("--under") + 1] ?? "") : "";
  const scope = segments.includes("--scope") ? segments[segments.indexOf("--scope") + 1] : "all";
  if (scope !== "all" && scope !== "history-editing") throw new Error(`[schema fault-notices] --scope expects all or history-editing, got ${JSON.stringify(scope)}`);
  const report = faultNoticeReport(repoRoot, under, scope);
  const census = segments.includes("--census");
  if (segments.includes("--json")) {
    console.log(JSON.stringify(census ? report.census : { diagnostics: report.diagnostics, census: report.census }, null, 2));
    process.exit(census || report.diagnostics.length === 0 ? 0 : 1);
  }
  const total = report.census.reduce((sum, row) => ({ codes: sum.codes + row.codes, labelled: sum.labelled + row.labelled, declared: sum.declared + row.declared, anonymous: sum.anonymous + row.anonymous }), { codes: 0, labelled: 0, declared: 0, anonymous: 0 });
  if (census) {
    const classes = [...new Set(report.census.flatMap((row) => Object.keys(row.refused)))].sort();
    console.log(["owner", "codes", "labelled", "declared", "anonymous", "findings", ...classes].join("\t"));
    for (const row of report.census) console.log([row.owner, row.codes, row.labelled, row.declared, row.anonymous, row.findings, ...classes.map((name) => row.refused[name] ?? 0)].join("\t"));
  } else for (const entry of report.diagnostics.slice(0, 40)) console.log(`[schema fault-notices]   ${entry.path} — ${entry.detail}`);
  console.log(`[schema fault-notices] ${total.labelled} labelled of ${total.codes} guest fault code(s), ${total.declared} declared notice(s), ${total.anonymous} anonymous fault(s); ${report.diagnostics.length} schema-fault-notice finding(s)${under === "" ? "" : ` under ${under}`}${scope === "all" ? "" : ` in scope ${scope}`}`);
  process.exit(census || report.diagnostics.length === 0 ? 0 : 1);
}
//#endregion 📢️FaultNotices

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

/** 🗂️ `$id` → path of every JSON document inside a `🧬️schema` directory of `root` (fixtures, tests and build output skipped) — the twin
 * of the derive's `mutation_schema_document_index`. */
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
function runMutationEditability(repoRoot: string, segments: string[]): never {
  const under = segments.includes("--under") ? (segments[segments.indexOf("--under") + 1] ?? "") : "";
  const report = mutationEditabilityReport(repoRoot, under);
  const census = segments.includes("--census");
  if (segments.includes("--json")) {
    console.log(JSON.stringify(census ? report.census : report, null, 2));
    process.exit(census || report.diagnostics.length === 0 ? 0 : 1);
  }
  const total = report.census.reduce((sum, row) => ({ aggregates: sum.aggregates + row.aggregates, leaves: sum.leaves + row.leaves, editable: sum.editable + row.editable, foreign: sum.foreign + row.foreign, inert: sum.inert + row.inert, withdrawOnly: sum.withdrawOnly + row.withdrawOnly, handwritten: sum.handwritten + row.handwritten }), { aggregates: 0, leaves: 0, editable: 0, foreign: 0, inert: 0, withdrawOnly: 0, handwritten: 0 });
  if (census) {
    const classes = [...new Set(report.census.flatMap((row) => Object.keys(row.refused)))].sort();
    console.log(["owner", "aggregates", "leaves", "editable", "foreign", "inert", "withdrawOnly", "handwritten", "findings", ...classes].join("\t"));
    for (const row of report.census) console.log([row.owner, row.aggregates, row.leaves, row.editable, row.foreign, row.inert, row.withdrawOnly, row.handwritten, row.findings, ...classes.map((name) => row.refused[name] ?? 0)].join("\t"));
  } else for (const entry of report.diagnostics.slice(0, 40)) console.log(`[schema mutation-editability]   ${entry.path} — ${entry.detail}`);
  console.log(`[schema mutation-editability] ${total.editable}/${total.leaves} leaves of ${total.aggregates} aggregates editable (${total.foreign} composite with foreign-step capability, ${total.withdrawOnly} withdraw-only by declaration, ${total.inert - total.withdrawOnly} inert phase(s)); ${total.handwritten} hand-written aggregate(s); ${report.diagnostics.length} schema-mutation-editability finding(s)${under === "" ? "" : ` under ${under}`}`);
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
 * history or declares why not (`schema-mutation-editability`), and does every guest fault code reach the person as a notice in
 * every locale (`schema-fault-notice`). Nothing
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
    if (segments[0] === "fault-notices") runFaultNotices(this.repoRoot, segments.slice(1));
    const under = segments[segments.indexOf("--under") + 1];
    const scope = segments.includes("--under") && under !== undefined ? under : "";
    const diagnostics: SchemaDiagnostic[] = [
      ...schemaContractDiagnostics(this.repoRoot, scope),
      ...mutationInputUiReport(this.repoRoot, scope).diagnostics,
      ...mutationPayloadParityReport(this.repoRoot, scope).diagnostics,
      ...mutationLabelReport(this.repoRoot, scope).diagnostics,
      ...mutationEditabilityReport(this.repoRoot, scope).diagnostics,
      ...faultNoticeReport(this.repoRoot, scope).diagnostics,
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
