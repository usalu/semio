/** 🎛️ Mutation input UI gate
 *
 * `schema-mutation-input-ui`: every mutation input carries a UI descriptor, and the census of declared and inferred inputs.
 */

import { type SchemaDiagnostic, readSchemaCatalog } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { type ActionArgDef, type InputSchemaAudit, SHELL_LOCALES, SHELL_TERMINOLOGIES, argControl, inputLabelGlossary, inputNumericTransport, inputShape, isShellLocale, isShellTerminology, mutationInputAudit } from "@semio-tech/framework-manifest";
import { type Dirent, existsSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { catalogSchemaDocuments, escapePointer, isRecord, readJsonObject, unescapePointer } from "../📚️schema-documents/🟦️.ts";
import { repositorySources } from "../🌳️mutation-tree/🟦️.ts";

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
 * 📥️ Audits every catalogued mutation leaf's payload schema with the framework's collecting reader `mutationInputAudit`
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
export function runMutationInputUi(repoRoot: string, segments: string[]): void {
  const under = segments.includes("--under") ? (segments[segments.indexOf("--under") + 1] ?? "") : "";
  const report = mutationInputUiReport(repoRoot, under);
  if (under !== "" && report.census.every((row) => row.leaves === 0)) {
    console.error(`[schema mutation-inputs] --under ${JSON.stringify(under)} holds no mutation leaf: pass a repository-relative path such as ✏️s/🔌️plugins/<plugin>`);
    {process.exitCode=2;return;}
  }
  const census = segments.includes("--census");
  const listed = segments.includes("--inputs");
  if (segments.includes("--json")) {
    console.log(JSON.stringify(census ? report.census : listed ? report.inputs : { diagnostics: report.diagnostics, census: report.census, multiline: report.multiline }, null, 2));
    {process.exitCode=census || report.diagnostics.length === 0 ? 0 : 1;return;}
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
  {process.exitCode=census || report.diagnostics.length === 0 ? 0 : 1;return;}
}
//#endregion 🎛️MutationInputUi
