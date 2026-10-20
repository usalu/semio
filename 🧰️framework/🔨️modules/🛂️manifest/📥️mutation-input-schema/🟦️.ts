/** 📥️ The mutation input schema: the resolver and error types, the input vocabulary sets, the localized text, glossary, shape and numeric transport helpers, the definitions, audit and instance entry points and the reader that turns one input schema into action argument definitions. */
import { SHELL_LOCALES, isShellLocale, SHELL_TERMINOLOGIES, isShellTerminology, type ShellLocale, type ShellTerminology, type LocalizedLabel } from "../🤖️generated/🎚️ui-axes/🟦️.ts";
import inputLabelGlossaryDocument from "../🔣️input-labels.json" with { type: "json" };
import { valueSchema as numericTransportSchemaDocument } from "../../🌱️value/📦️packages/🟦️typescript/🧬️schema/🟦️.ts";
import type { ActionArgDef, ActionArgOption, ArgPresentation, ArgSchema, NumberScale, OptionSource, ReferenceIdType, SnapSource } from "../🎬️action-types-and-semantics/🟦️.ts";

// #region 🛂️Manifest
//#region 🔌️PluginAndAppContract
//#region PluginRuntime
//#region 🔖️MutationInputs
/** 🧭️ Resolves a cross-document `$ref`: the parsed schema document whose `$id` is `id` (no fragment), or `undefined`. */
export type InputSchemaResolver = (id: string) => unknown;

/** 🚫️ The class of an {@link InputSchemaError} — the Rust `InputSchemaErrorCode` twin. */
export type InputSchemaErrorCode = "malformed" | "refUnresolved" | "uiInvalid" | "widgetIncompatible" | "labelMissing" | "optionLabelMissing" | "localeMissing" | "wordOnlyFloat";

/** 🚫️ Why a mutation payload schema yields no input descriptors: `code` and the RFC 6901 `pointer` of the input in the payload. */
export class InputSchemaError extends Error {
  readonly code: InputSchemaErrorCode;
  readonly pointer: string;
  constructor(
    code: InputSchemaErrorCode,
    pointer: string,
    detail: string,
  ) {
    super(`${code} at ${JSON.stringify(pointer)}: ${detail}`); this.code = code; this.pointer = pointer;
  }
}

type Json = null | boolean | number | string | Json[] | { [key: string]: Json };
type JsonObject = { [key: string]: Json };
type ResolvedInput = { readonly document: string | null; readonly node: JsonObject; readonly ui: ReadonlyMap<string, Json>; readonly refs: readonly string[]; readonly nullable: boolean };

const INPUT_UI_KEYS = new Set(["widget", "role", "label", "description", "step", "precision", "softMin", "softMax", "snaps", "snapSource", "optionSource", "unit", "displayUnit", "displayFactor", "scale", "group", "order", "options", "ref"]);
const INPUT_UI_NUMBER_KEYS = new Set(["step", "precision", "softMin", "softMax", "snaps", "snapSource", "displayUnit", "displayFactor", "scale"]);
/** 🧭️ The number facets a vector shares across its components — the Rust `INPUT_UI_VECTOR_KEYS` twin. */
const INPUT_UI_VECTOR_KEYS = new Set(["step", "precision", "snaps", "snapSource", "displayUnit", "displayFactor"]);
const INPUT_WIDGETS = new Set(["slider", "stepper", "dial", "toggle", "select", "segmented", "text", "multiline", "vector", "color", "reference", "hidden"]);
const INPUT_TYPES = new Set(["string", "integer", "number", "boolean", "object", "array"]);
const INPUT_REF_DEPTH = 32;

const isObject = (value: unknown): value is JsonObject => value !== null && typeof value === "object" && !Array.isArray(value);
const isInteger = (value: unknown): value is number => typeof value === "number" && Number.isInteger(value);
const inputPointer = (parent: string, key: string): string => `${parent}/${key.replaceAll("~", "~0").replaceAll("/", "~1")}`;

/** 🌐️ A `{<locale>: text}` map naming every locale, or a `{<terminology>: {<locale>: text}}` matrix naming every cell — the Rust `input_localized_text` twin. */
function inputLocalizedText(value: Json | undefined): LocalizedLabel | InputSchemaErrorCode {
  if (!isObject(value)) return "uiInvalid";
  const cell = (map: Json | undefined, locale: ShellLocale): string | undefined => (isObject(map) && typeof map[locale] === "string" && map[locale] !== "" ? (map[locale] as string) : undefined);
  const keys = Object.keys(value);
  const matrix = (resolve: (terminology: ShellTerminology, locale: ShellLocale) => string): LocalizedLabel =>
    Object.fromEntries(SHELL_TERMINOLOGIES.map((terminology) => [terminology, Object.fromEntries(SHELL_LOCALES.map((locale) => [locale, resolve(terminology, locale)]))])) as LocalizedLabel;
  if (keys.every(isShellLocale)) {
    if (SHELL_LOCALES.some((locale) => cell(value, locale) === undefined)) return "localeMissing";
    return matrix((_, locale) => cell(value, locale)!);
  }
  if (keys.every((key) => isShellTerminology(key) && isObject(value[key]) && Object.keys(value[key] as JsonObject).every(isShellLocale))) {
    if (!SHELL_TERMINOLOGIES.every((terminology) => SHELL_LOCALES.every((locale) => cell(value[terminology], locale) !== undefined))) return "localeMissing";
    return matrix((terminology, locale) => cell(value[terminology], locale)!);
  }
  return "uiInvalid";
}

let glossary: ReadonlyMap<string, LocalizedLabel> | undefined;

/** 📚️ The framework input-label glossary (`🔣️input-labels.json`), field name → label — the Rust `input_label_glossary` twin. */
export function inputLabelGlossary(): ReadonlyMap<string, LocalizedLabel> {
  glossary ??= new Map(
    Object.entries(inputLabelGlossaryDocument.labels).map(([name, label]) => {
      const localized = inputLocalizedText(label as Json);
      if (typeof localized === "string") throw new Error(`🛂️ glossary label ${name} is ${localized}`);
      return [name, localized];
    }),
  );
  return glossary;
}

/** 🏷️ Schema keys that annotate without constraining (and every `x-` key) — the Rust `INPUT_SHAPE_ANNOTATIONS` twin. */
const INPUT_SHAPE_ANNOTATIONS = new Set(["title", "description", "$comment", "examples", "default", "format"]);

/** 🧬️ The constraint shape of a schema node as canonical JSON text: annotations dropped, object keys sorted, `required` sorted —
 * the Rust `input_shape` twin. */
export function inputShape(node: Json | undefined): string {
  if (Array.isArray(node)) return `[${node.map(inputShape).join(",")}]`;
  if (!isObject(node)) return JSON.stringify(node ?? null);
  const kept = Object.keys(node)
    .filter((key) => !key.startsWith("x-") && !INPUT_SHAPE_ANNOTATIONS.has(key))
    .sort()
    .map((key) => `${JSON.stringify(key)}:${key === "required" && Array.isArray(node[key]) ? inputShape([...(node[key] as Json[])].sort()) : inputShape(node[key])}`);
  return `{${kept.join(",")}}`;
}

let numericTransport: { readonly word: string; readonly number: string } | undefined;

/** 🔢️ The shapes of the framework's numeric transport (`🌱️value/🧬️schema/🔣️.json` `$defs/Binary64` — the binary64 word — and the
 * plain-number branch of `$defs/Binary64Transport`) — the Rust `input_numeric_transport` twin. */
export function inputNumericTransport(): { readonly word: string; readonly number: string } {
  const definitions = numericTransportSchemaDocument.$defs as unknown as Record<string, JsonObject>;
  numericTransport ??= {
    word: inputShape(definitions.Binary64),
    number: inputShape((definitions.Binary64Transport!.anyOf as Json[]).find((branch) => isObject(branch) && branch.$ref === undefined)),
  };
  return numericTransport;
}

/** 🔤️ The single non-null JSON type a schema node declares — the Rust `input_type` twin. */
function inputType(node: JsonObject): string | undefined {
  const type = node.type;
  const named = typeof type === "string" ? [type] : Array.isArray(type) ? type.filter((name): name is string => typeof name === "string" && name !== "null") : node.properties !== undefined ? ["object"] : node.items !== undefined ? ["array"] : Array.isArray(node.enum) && node.enum.length > 0 && node.enum.every((value) => typeof value === "string") ? ["string"] : [];
  return named.length === 1 && INPUT_TYPES.has(named[0]!) ? named[0] : undefined;
}

/** 🎯️ `<kind>Id`/`<kind>_id` (string) or `<kind>Ids`/`<kind>_ids` (array) names a reference to `kind` — the Rust `input_inferred_reference_kind` twin. */
function inputInferredReferenceKind(key: string, many: boolean): string | undefined {
  const suffixes = many ? ["Ids", "_ids"] : ["Id", "_id"];
  const suffix = suffixes.find((candidate) => key.endsWith(candidate));
  if (suffix === undefined) return undefined;
  let stem = key.slice(0, key.length - suffix.length);
  if (stem.startsWith("new")) {
    const rest = stem.slice(3);
    if (rest === "") return undefined;
    if (/^[A-Z]/u.test(rest)) stem = rest;
  }
  if (!/^[A-Za-z][A-Za-z0-9_]*$/u.test(stem)) return undefined;
  return stem[0]!.toLowerCase() + stem.slice(1);
}

/** 🧬️ The TypeScript twin of Rust `mutation_input_defs` (`🛂️manifest/🦀️.rs`): one {@link ActionArgDef} per input of a mutation
 * leaf payload JSON Schema, byte-equal in canonical JSON; throws {@link InputSchemaError}. Fixture: `🧫️fixtures/🧫️mutation-inputs`. */
export function mutationInputDefs(schema: string | unknown, resolver: InputSchemaResolver): ActionArgDef[] {
  return inputSchemaReader(schema, resolver, false).inputs();
}

/** 🧺️ Every finding of a mutation leaf payload schema at once — the collecting twin of the fail-fast {@link mutationInputDefs}
 * (Rust `InputSchemaAudit`): every refused input at every nested pointer is recorded and reading goes on past it, so
 * `findings` names every pointer an author must fix (first occurrence of each distinct finding, in reading order; its first
 * entry is the error the fail-fast reader throws) and `inputs` holds what reads despite them. */
export type InputSchemaAudit = { readonly inputs: ActionArgDef[]; readonly findings: InputSchemaError[] };

/** 🧺️ Reads `schema` in collecting mode — the Rust `mutation_input_audit` twin; see {@link InputSchemaAudit}. */
export function mutationInputAudit(schema: string | unknown, resolver: InputSchemaResolver): InputSchemaAudit {
  let reader: ReturnType<typeof inputSchemaReader>;
  try {
    reader = inputSchemaReader(schema, resolver, true);
  } catch (error) {
    if (!(error instanceof InputSchemaError)) throw error;
    return { inputs: [], findings: [error] };
  }
  return reader.audit();
}

/** 🧾️ The TypeScript twin of Rust `mutation_input_instance`: `payload` (a leaf payload without its aggregate tag) with every root
 * `const` property of the leaf schema spliced in where absent, ready for a JSON Schema validator; throws {@link InputSchemaError}. */
export function mutationInputInstance(schema: string | unknown, resolver: InputSchemaResolver, payload: unknown): Record<string, unknown> {
  return inputSchemaReader(schema, resolver, false).instance(payload);
}

/** 🏷️ The label a collecting read keeps for an input whose own label is refused. */
const INPUT_LABEL_PLACEHOLDER = { native: { en: "", de: "" }, reuse: { en: "", de: "" } } as LocalizedLabel;

function inputSchemaReader(schema: string | unknown, resolver: InputSchemaResolver, collect: boolean): { inputs(): ActionArgDef[]; audit(): InputSchemaAudit; instance(payload: unknown): Record<string, unknown> } {
  let root: Json;
  try {
    root = (typeof schema === "string" ? JSON.parse(schema) : schema) as Json;
  } catch (error) {
    throw new InputSchemaError("malformed", "", String(error));
  }
  const documents = new Map<string, Json>();
  const active: string[] = ["#"];
  const fail = (code: InputSchemaErrorCode, pointer: string, detail: string): never => {
    throw new InputSchemaError(code, pointer, detail);
  };
  const findings: InputSchemaError[] | undefined = collect ? [] : undefined;
  /** 🧺️ The Rust `recover` twin: in collecting mode records a refusal and goes on with `fallback`; fail-fast rethrows. */
  const recover = <T>(attempt: () => T, fallback: () => T): T => {
    if (findings === undefined) return attempt();
    try {
      return attempt();
    } catch (error) {
      if (!(error instanceof InputSchemaError)) throw error;
      findings.push(error);
      return fallback();
    }
  };

  const target = (document: string | null, reference: string, pointer: string): [string | null, Json] => {
    const hash = reference.indexOf("#");
    const id = hash < 0 ? reference : reference.slice(0, hash);
    const fragment = hash < 0 ? "" : reference.slice(hash + 1);
    const owner = id === "" ? document : id;
    if (owner !== null && !documents.has(owner)) {
      const fetched = resolver(owner);
      if (fetched === undefined || fetched === null) fail("refUnresolved", pointer, `no schema document has $id ${owner}`);
      documents.set(owner, fetched as Json);
    }
    let current: Json | undefined = owner === null ? root : documents.get(owner);
    for (const segment of fragment.split("/").slice(1).map((part) => part.replaceAll("~1", "/").replaceAll("~0", "~"))) {
      current = isObject(current) ? current[segment] : Array.isArray(current) && /^\d+$/u.test(segment) ? current[Number(segment)] : undefined;
    }
    return current === undefined ? fail("refUnresolved", pointer, `${reference} lands on nothing`) : [owner, current];
  };

  const resolve = (document: string | null, start: Json, pointer: string): ResolvedInput => {
    let node = start;
    const ui = new Map<string, Json>();
    const refs: string[] = [];
    let nullable = false;
    for (let hop = 0; hop < INPUT_REF_DEPTH; hop += 1) {
      if (!isObject(node)) return { document, node: {}, ui, refs, nullable };
      const annotation = node["x-semio-ui"];
      if (annotation !== undefined) {
        const entries = recover((): [string, Json][] => (isObject(annotation) ? Object.entries(annotation as JsonObject) : fail("uiInvalid", pointer, "x-semio-ui is an object")), () => []);
        for (const [key, value] of entries) {
          if (!INPUT_UI_KEYS.has(key)) {
            recover(() => fail("uiInvalid", pointer, `x-semio-ui carries the undeclared key ${key}`), () => undefined);
            continue;
          }
          if (!ui.has(key)) ui.set(key, value);
        }
      }
      if (typeof node.$ref === "string") {
        const reference = node.$ref;
        [document, node] = target(document, reference, pointer);
        refs.push(`${document ?? ""}#${reference.includes("#") ? reference.slice(reference.indexOf("#") + 1) : ""}`);
        continue;
      }
      const union = Array.isArray(node.oneOf) ? node.oneOf : Array.isArray(node.anyOf) ? node.anyOf : undefined;
      if (union !== undefined) {
        const concrete = union.filter((branch) => !(isObject(branch) && branch.type === "null" && Object.keys(branch).length === 1));
        if (concrete.length === 1 && concrete.length < union.length) {
          node = concrete[0]!;
          nullable = true;
          continue;
        }
      }
      const resolvedNullable = nullable || (Array.isArray(node.type) && node.type.includes("null"));
      const transport = numericTransportAt(document, node, pointer);
      if (transport && inputShape(node) === inputNumericTransport().word) recover(() => fail("wordOnlyFloat", pointer, "a number input accepts only the exact binary64 word, never the plain number its payload and every draft carry; reference framework/value/schema.json#/$defs/Binary64Transport"), () => undefined);
      return { document, node: transport ? { type: "number" } : node, ui, refs, nullable: resolvedNullable };
    }
    return fail("malformed", pointer, "a $ref chain exceeds 32 hops");
  };

  /** 🔢️ Whether `node` (in `document`) is the framework's numeric transport: the binary64 word itself (read as a number but refused
   * as `wordOnlyFloat`), or a two-branch union of that word and a plain number (each branch restated or referenced) with nothing
   * but annotations beside it — the Rust `InputSchemaReader::numeric_transport` twin. */
  const numericTransportAt = (document: string | null, node: JsonObject, pointer: string): boolean => {
    const { word, number } = inputNumericTransport();
    if (inputShape(node) === word) return true;
    const branches = Array.isArray(node.oneOf) ? node.oneOf : Array.isArray(node.anyOf) ? node.anyOf : undefined;
    if (branches === undefined || branches.length !== 2) return false;
    if (Object.keys(node).some((key) => key !== "anyOf" && key !== "oneOf" && !key.startsWith("x-") && !INPUT_SHAPE_ANNOTATIONS.has(key))) return false;
    const shapes = branches.map((branch) => {
      let [owner, current]: [string | null, Json] = [document, branch];
      for (let hop = 0; hop < INPUT_REF_DEPTH && isObject(current) && typeof current.$ref === "string"; hop += 1) {
        try {
          [owner, current] = target(owner, current.$ref, pointer);
        } catch (error) {
          if (!(error instanceof InputSchemaError)) throw error;
          return "null";
        }
      }
      return inputShape(current);
    });
    return shapes.includes(word) && shapes.includes(number);
  };

  const uiString = (input: ResolvedInput, key: string, pointer: string): string | undefined => {
    const value = input.ui.get(key);
    if (value === undefined) return undefined;
    return typeof value === "string" && value !== "" ? value : fail("uiInvalid", pointer, `${key} is a non-empty string`);
  };
  const uiFinite = (input: ResolvedInput, key: string, pointer: string): number | undefined => {
    const value = input.ui.get(key);
    if (value === undefined) return undefined;
    return typeof value === "number" && Number.isFinite(value) ? value : fail("uiInvalid", pointer, `${key} is a finite number`);
  };
  const uiPositive = (input: ResolvedInput, key: string, pointer: string): number | undefined => {
    const value = uiFinite(input, key, pointer);
    return value !== undefined && value <= 0 ? fail("uiInvalid", pointer, `${key} is positive`) : value;
  };
  const count = (node: JsonObject, key: string): number | undefined => (isInteger(node[key]) && (node[key] as number) >= 0 ? (node[key] as number) : undefined);

  const options = (input: ResolvedInput, pointer: string): ActionArgOption[] => {
    const values = input.node.enum;
    if (!Array.isArray(values)) return [];
    const labels = input.ui.get("options");
    if (labels !== undefined && (!isObject(labels) || Object.keys(labels).some((value) => !values.includes(value)))) fail("uiInvalid", pointer, "options labels only declared enum values");
    return values.map((value) => {
      if (typeof value !== "string") return fail("malformed", pointer, "a labelled enum lists strings");
      const declared = isObject(labels) ? labels[value] : undefined;
      if (declared !== undefined) {
        const label = inputLocalizedText(declared);
        return typeof label === "string" ? fail(label, pointer, `option ${value} names every locale`) : { value, label };
      }
      const label = inputLabelGlossary().get(value);
      return label === undefined ? fail("optionLabelMissing", pointer, `option ${value} has no label`) : { value, label };
    });
  };

  /** 🗝️ A string input's `x-semio-ui.optionSource` — the Rust `input_option_source` twin. */
  const sourcedOptions = (input: ResolvedInput, pointer: string): OptionSource | undefined => {
    const source = input.ui.get("optionSource");
    if (source === undefined) return undefined;
    const entries = isObject(source) ? Object.entries(source) : [];
    const [name, template] = entries.length === 1 ? entries[0]! : ["", null];
    return name === "snapshot" && typeof template === "string" && template.startsWith("/") ? { kind: "snapshot", pointer: template } : fail("uiInvalid", pointer, "optionSource is {snapshot: pointer template}");
  };

  const number = (input: ResolvedInput, integer: boolean, pointer: string): ArgSchema => {
    const keyword = (name: string): number | undefined => (typeof input.node[name] === "number" ? (input.node[name] as number) : undefined);
    const bound = (inclusive: number | undefined, exclusive: number | undefined, lower: boolean): [number | undefined, boolean] => {
      if (exclusive === undefined) return [inclusive, false];
      if (inclusive !== undefined && ((lower && inclusive > exclusive) || (!lower && inclusive < exclusive))) return [inclusive, false];
      if (integer) return [lower ? Math.floor(exclusive) + 1 : Math.ceil(exclusive) - 1, false];
      return [exclusive, true];
    };
    const [min, minExclusive] = bound(keyword("minimum"), keyword("exclusiveMinimum"), true);
    const [max, maxExclusive] = bound(keyword("maximum"), keyword("exclusiveMaximum"), false);
    const inside = (value: number): boolean => (min === undefined || (minExclusive ? value > min : value >= min)) && (max === undefined || (maxExclusive ? value < max : value <= max));
    const rawSnaps = input.ui.get("snaps");
    if (rawSnaps !== undefined && !Array.isArray(rawSnaps)) fail("uiInvalid", pointer, "snaps is an array of numbers");
    const snaps = (rawSnaps ?? []) as Json[];
    if (!snaps.every((snap) => typeof snap === "number" && Number.isFinite(snap))) fail("uiInvalid", pointer, "snaps are finite numbers");
    const outside = (snaps as number[]).find((snap) => !inside(snap));
    if (outside !== undefined) fail("uiInvalid", pointer, `snap ${outside} lies outside the hard bounds`);
    const source = input.ui.get("snapSource");
    let snapSource: SnapSource | undefined;
    if (source !== undefined) {
      const entries = isObject(source) ? Object.entries(source) : [];
      const [name, value] = entries.length === 1 ? entries[0]! : ["", null];
      if (name === "step" && value === true) snapSource = { kind: "step" };
      else if (name === "config" && typeof value === "string" && value !== "") snapSource = { kind: "config", key: value };
      else if (name === "snapshot" && typeof value === "string" && (value === "" || value.startsWith("/"))) snapSource = { kind: "snapshot", pointer: value };
      else fail("uiInvalid", pointer, "snapSource is {step: true}, {config: key} or {snapshot: pointer}");
    }
    const softMin = uiFinite(input, "softMin", pointer);
    const softMax = uiFinite(input, "softMax", pointer);
    if ((softMin !== undefined && !inside(softMin)) || (softMax !== undefined && !inside(softMax)) || (softMin !== undefined && softMax !== undefined && softMin >= softMax)) fail("uiInvalid", pointer, "softMin < softMax lie inside the hard bounds");
    const rawScale = input.ui.get("scale");
    if (rawScale !== undefined && rawScale !== "linear" && rawScale !== "log") fail("uiInvalid", pointer, "scale is linear or log");
    const scale = rawScale as NumberScale | undefined;
    const low = softMin ?? min;
    if (scale === "log" && !(low !== undefined && low > 0)) fail("uiInvalid", pointer, "a log scale starts at a positive softMin or minimum");
    const rawPrecision = input.ui.get("precision");
    if (rawPrecision !== undefined && !(isInteger(rawPrecision) && rawPrecision >= 0 && rawPrecision <= 15)) fail("uiInvalid", pointer, "precision is an integer 0..=15");
    const displayFactor = uiFinite(input, "displayFactor", pointer);
    if (displayFactor === 0) fail("uiInvalid", pointer, "displayFactor is non-zero");
    const step = uiPositive(input, "step", pointer) ?? (integer ? 1 : undefined);
    const unit = uiString(input, "unit", pointer);
    const displayUnit = uiString(input, "displayUnit", pointer);
    return {
      kind: "number",
      ...(min === undefined ? {} : { min }),
      ...(minExclusive ? { minExclusive } : {}),
      ...(max === undefined ? {} : { max }),
      ...(maxExclusive ? { maxExclusive } : {}),
      ...(step === undefined ? {} : { step }),
      integer,
      ...(unit === undefined ? {} : { unit }),
      ...(snaps.length === 0 ? {} : { snaps: snaps as number[] }),
      ...(snapSource === undefined ? {} : { snapSource }),
      ...(softMin === undefined ? {} : { softMin }),
      ...(softMax === undefined ? {} : { softMax }),
      ...(rawPrecision === undefined ? {} : { precision: rawPrecision as number }),
      ...(displayUnit === undefined ? {} : { displayUnit }),
      ...(displayFactor === undefined ? {} : { displayFactor }),
      ...(scale === undefined ? {} : { scale }),
    };
  };

  const reference = (key: string, input: ResolvedInput, many: boolean, inferredKind: string | undefined, idType: ReferenceIdType, pointer: string): ArgSchema => {
    const declared = input.ui.get("ref");
    if (declared !== undefined && (!isObject(declared) || Object.keys(declared).some((name) => !["kind", "domain", "granularity"].includes(name)))) fail("uiInvalid", pointer, "ref is {kind, domain?, granularity?}");
    const kind = isObject(declared) ? declared.kind : undefined;
    let kinds: string[];
    if (typeof kind === "string" && kind !== "") kinds = [kind];
    else if (Array.isArray(kind) && kind.length > 0 && kind.every((entry) => typeof entry === "string" && entry !== "")) kinds = kind as string[];
    else if (kind !== undefined) return fail("uiInvalid", pointer, "ref.kind is a non-empty string or a non-empty array of them");
    else {
      const inferred = inferredKind ?? inputInferredReferenceKind(key, many);
      kinds = inferred === undefined ? fail("uiInvalid", pointer, "a target names its ref.kind") : [inferred];
    }
    const text = (name: string): string | undefined => {
      const value = isObject(declared) ? declared[name] : undefined;
      if (value === undefined) return undefined;
      return typeof value === "string" && value !== "" ? value : fail("uiInvalid", pointer, "ref.domain and ref.granularity are non-empty strings");
    };
    const domain = text("domain");
    const granularity = text("granularity");
    const minItems = many ? count(input.node, "minItems") : undefined;
    const maxItems = many ? count(input.node, "maxItems") : undefined;
    return { kind: "reference", kinds, ...(domain === undefined ? {} : { domain }), ...(granularity === undefined ? {} : { granularity }), ...(many ? { many } : {}), ...(minItems === undefined ? {} : { minItems }), ...(maxItems === undefined ? {} : { maxItems }), ...(idType === "string" ? {} : { idType }) };
  };

  /** ♾️ The Rust `guarded` twin: a `$ref` already being expanded on this path yields a structured value, never an endless tree. */
  const guarded = (node: ResolvedInput, expand: () => ArgSchema): ArgSchema => {
    if (node.refs.some((key) => active.includes(key))) return { kind: "any" };
    const mark = active.length;
    active.push(...node.refs);
    try {
      return expand();
    } finally {
      active.length = mark;
    }
  };

  const itemSchema = (items: ResolvedInput, pointer: string): ArgSchema => guarded(items, () => itemSchemaExpanded(items, pointer));

  const itemSchemaExpanded = (items: ResolvedInput, pointer: string): ArgSchema => {
    if (items.node.const !== undefined) return { kind: "any" };
    switch (inputType(items.node)) {
      case "string": {
        const choices = options(items, pointer);
        const optionSource = sourcedOptions(items, pointer);
        return { kind: "string", ...(choices.length === 0 ? {} : { options: choices }), ...(optionSource === undefined ? {} : { optionSource }) } as ArgSchema;
      }
      case "integer":
        return number(items, true, pointer);
      case "number":
        return number(items, false, pointer);
      case "boolean":
        return { kind: "boolean" };
      case "object":
        return items.node.properties !== undefined || items.node.allOf !== undefined ? { kind: "object", fields: fields(items, pointer) } : { kind: "any" };
      case "array": {
        const inner = items.node.items !== undefined ? itemSchema(resolve(items.document, items.node.items, `${pointer}/-`), `${pointer}/-`) : ({ kind: "any" } as ArgSchema);
        const minItems = count(items.node, "minItems");
        const maxItems = count(items.node, "maxItems");
        return { kind: "array", items: inner, ...(minItems === undefined ? {} : { minItems }), ...(maxItems === undefined ? {} : { maxItems }) };
      }
      default:
        return { kind: "any" };
    }
  };

  const valueSchema = (key: string, input: ResolvedInput, role: string | undefined, widget: string | undefined, pointer: string): ArgSchema => {
    const kind = inputType(input.node);
    if (kind !== "integer" && kind !== "number") {
      const misplaced = [...input.ui.keys()].find((name) => INPUT_UI_NUMBER_KEYS.has(name) && !(kind === "array" && INPUT_UI_VECTOR_KEYS.has(name)));
      if (misplaced !== undefined) recover(() => fail("uiInvalid", pointer, `${misplaced} only applies to a number`), () => undefined);
    }
    if (input.ui.has("unit") && kind !== "integer" && kind !== "number" && kind !== "array") recover(() => fail("uiInvalid", pointer, "unit only applies to a number or a vector"), () => undefined);
    if (input.ui.has("optionSource") && (kind !== "string" || input.node.enum !== undefined)) recover(() => fail("uiInvalid", pointer, "optionSource only sources the options of a string without an enum"), () => undefined);
    if (widget === "hidden" && (kind === "object" || kind === "array")) return { kind: "any" };
    const many = kind === "array";
    const items = many && input.node.items !== undefined ? recover((): ResolvedInput | undefined => resolve(input.document, input.node.items!, `${pointer}/-`), () => undefined) : undefined;
    const itemKind = items === undefined ? undefined : inputType(items.node);
    const idKind = many ? itemKind : kind;
    const referenceShaped = idKind === "string";
    const explicitReference = input.ui.has("ref") || widget === "reference" || role === "target";
    if (explicitReference && idKind !== "string" && idKind !== "integer") fail("widgetIncompatible", pointer, "a reference is a string or integer id or an array of them");
    const inferredKind = role === undefined && widget === undefined && referenceShaped && input.node.enum === undefined ? inputInferredReferenceKind(key, many) : undefined;
    if (explicitReference || inferredKind !== undefined) {
      const numeric = [...input.ui.keys()].find((name) => name === "unit" || INPUT_UI_NUMBER_KEYS.has(name));
      if (numeric !== undefined) recover(() => fail("uiInvalid", pointer, `${numeric} does not apply to a reference`), () => undefined);
      return reference(key, input, many, inferredKind, idKind === "integer" ? "integer" : "string", pointer);
    }
    if (input.ui.has("options") && input.node.enum === undefined) recover(() => fail("uiInvalid", pointer, "options only label enum values"), () => undefined);
    switch (kind) {
      case "string": {
        const minLen = count(input.node, "minLength");
        const maxLen = count(input.node, "maxLength");
        const pattern = typeof input.node.pattern === "string" ? input.node.pattern : undefined;
        const choices = options(input, pointer);
        const optionSource = sourcedOptions(input, pointer);
        return { kind: "string", ...(choices.length === 0 ? {} : { options: choices }), ...(optionSource === undefined ? {} : { optionSource }), ...(minLen === undefined ? {} : { minLen }), ...(maxLen === undefined ? {} : { maxLen }), ...(pattern === undefined ? {} : { pattern }), ...(input.node.format === "uri" ? { format: { kind: "uri" } } : {}) } as ArgSchema;
      }
      case "integer":
        return number(input, true, pointer);
      case "number":
        return number(input, false, pointer);
      case "boolean":
        return { kind: "boolean" };
      case "object":
        return input.node.properties !== undefined || input.node.allOf !== undefined ? { kind: "object", fields: fields(input, pointer) } : { kind: "any" };
      case "array": {
        const minItems = count(input.node, "minItems");
        const maxItems = count(input.node, "maxItems");
        const numeric = itemKind === "integer" || itemKind === "number";
        const fixed = minItems !== undefined && minItems === maxItems && minItems >= 2 && minItems <= 4 ? minItems : undefined;
        if (widget === "vector" || widget === "color" || (numeric && fixed !== undefined)) {
          if (!numeric || fixed === undefined || items === undefined) return fail("widgetIncompatible", pointer, "a vector is an array of 2 to 4 numbers of fixed length");
          const component = number({ document: items.document, node: items.node, ui: input.ui, refs: [], nullable: items.nullable }, itemKind === "integer", pointer) as Extract<ArgSchema, { kind: "number" }>;
          const unit = uiString(input, "unit", pointer);
          const step = uiPositive(input, "step", pointer);
          const min = component.minExclusive ? undefined : component.min;
          const max = component.maxExclusive ? undefined : component.max;
          return {
            kind: "vector",
            dims: fixed,
            ...(min === undefined ? {} : { min }),
            ...(max === undefined ? {} : { max }),
            ...(unit === undefined ? {} : { unit }),
            ...(step === undefined ? {} : { step }),
            ...(component.snaps === undefined ? {} : { snaps: component.snaps }),
            ...(component.snapSource === undefined ? {} : { snapSource: component.snapSource }),
            ...(component.precision === undefined ? {} : { precision: component.precision }),
            ...(component.displayUnit === undefined ? {} : { displayUnit: component.displayUnit }),
            ...(component.displayFactor === undefined ? {} : { displayFactor: component.displayFactor }),
          };
        }
        const vectorOnly = numeric ? undefined : [...input.ui.keys()].find((name) => INPUT_UI_VECTOR_KEYS.has(name));
        if (vectorOnly !== undefined) recover(() => fail("uiInvalid", pointer, `${vectorOnly} only applies to a number or a vector`), () => undefined);
        const inherited = numeric && items !== undefined ? [...input.ui.entries()].filter(([name]) => (name === "unit" || INPUT_UI_VECTOR_KEYS.has(name)) && !items.ui.has(name)) : [];
        const itemSchemaValue = items === undefined ? ({ kind: "any" } as ArgSchema) : itemSchema(inherited.length === 0 ? items : { ...items, ui: new Map([...items.ui, ...inherited]) }, `${pointer}/-`);
        return { kind: "array", items: itemSchemaValue, ...(minItems === undefined ? {} : { minItems }), ...(maxItems === undefined ? {} : { maxItems }) };
      }
      default:
        return { kind: "any" };
    }
  };

  const input = (key: string, resolved: ResolvedInput, required: boolean, pointer: string): ActionArgDef | undefined => {
    const rawRole = resolved.ui.get("role");
    const role = rawRole !== undefined && !(rawRole === "value" || rawRole === "target" || rawRole === "discriminator") ? recover(() => fail("uiInvalid", pointer, "role is value, target or discriminator"), () => undefined) : (rawRole as string | undefined);
    if (role === "discriminator" || resolved.node.const !== undefined) return undefined;
    const rawWidget = resolved.ui.get("widget");
    const widget = rawWidget !== undefined && !(typeof rawWidget === "string" && INPUT_WIDGETS.has(rawWidget)) ? recover(() => fail("uiInvalid", pointer, "widget is not a declared widget"), () => undefined) : (rawWidget as string | undefined);
    const label = recover((): LocalizedLabel => {
      if (resolved.ui.has("label")) {
        const localized = inputLocalizedText(resolved.ui.get("label"));
        return typeof localized === "string" ? fail(localized, pointer, "label names every locale") : localized;
      }
      const glossaryLabel = inputLabelGlossary().get(key);
      return glossaryLabel === undefined ? fail("labelMissing", pointer, `${key} has no x-semio-ui.label and no glossary label`) : glossaryLabel;
    }, () => INPUT_LABEL_PLACEHOLDER);
    const description = recover((): LocalizedLabel | undefined => {
      if (!resolved.ui.has("description")) return undefined;
      const localized = inputLocalizedText(resolved.ui.get("description"));
      return typeof localized === "string" ? fail(localized, pointer, "description names every locale") : localized;
    }, () => undefined);
    const rawGroup = resolved.ui.get("group");
    const group = rawGroup !== undefined && !(typeof rawGroup === "string" && rawGroup !== "") ? recover(() => fail("uiInvalid", pointer, "group is a non-empty string"), () => undefined) : rawGroup;
    const rawOrder = resolved.ui.get("order");
    const order = rawOrder !== undefined && !isInteger(rawOrder) ? recover(() => fail("uiInvalid", pointer, "order is an integer"), () => undefined) : rawOrder;
    const schema = recover((): ArgSchema | undefined => guarded(resolved, () => valueSchema(key, resolved, role, widget, pointer)), () => undefined);
    if (schema === undefined) {
      return {
        id: inputPointer("", key),
        label,
        schema: { kind: "any" },
        required,
        ...(resolved.nullable ? { nullable: true } : {}),
        ...(description === undefined ? {} : { description }),
        ...(group === undefined ? {} : { group: group as string }),
        ...(order === undefined ? {} : { order: order as number }),
      };
    }
    const compatible =
      widget === undefined ||
      widget === "hidden" ||
      (["slider", "stepper", "dial"].includes(widget) && schema.kind === "number") ||
      (widget === "toggle" && schema.kind === "boolean") ||
      (widget === "vector" && schema.kind === "vector") ||
      (widget === "color" && schema.kind === "vector" && (schema.dims === 3 || schema.dims === 4) && schema.min === 0 && schema.max === 1) ||
      (widget === "reference" && schema.kind === "reference") ||
      ((widget === "select" || widget === "segmented") && schema.kind === "string" && ((schema.options ?? []).length > 0 || schema.optionSource !== undefined)) ||
      ((widget === "text" || widget === "multiline") && schema.kind === "string" && (schema.options ?? []).length === 0 && schema.optionSource === undefined);
    if (!compatible) recover(() => fail("widgetIncompatible", pointer, `widget ${widget} cannot edit this value`), () => undefined);
    const presentation = widget === "slider" || widget === "stepper" || widget === "dial" || widget === "segmented" || widget === "multiline" || widget === "hidden" || widget === "color" ? ({ kind: widget } as ArgPresentation) : undefined;
    const fallback = resolved.node.default;
    return {
      id: inputPointer("", key),
      label,
      schema,
      ...(presentation === undefined ? {} : { presentation }),
      required,
      ...(resolved.nullable ? { nullable: true } : {}),
      ...(fallback === undefined ? {} : { default: fallback }),
      ...(description === undefined ? {} : { description }),
      ...(group === undefined ? {} : { group: group as string }),
      ...(order === undefined ? {} : { order: order as number }),
    };
  };

  const members = (object: ResolvedInput, pointer: string, depth: number, found: ResolvedInput[]): ResolvedInput[] => {
    if (depth === INPUT_REF_DEPTH) fail("malformed", pointer, "an allOf composition exceeds 32 levels");
    found.push(object);
    for (const member of Array.isArray(object.node.allOf) ? object.node.allOf : []) members(resolve(object.document, member, pointer), pointer, depth + 1, found);
    return found;
  };

  const fields = (object: ResolvedInput, pointer: string): ActionArgDef[] => {
    const composed = members(object, pointer, 0, []);
    const required = composed.flatMap((member) => (Array.isArray(member.node.required) ? member.node.required.filter((name): name is string => typeof name === "string") : []));
    const seen = new Set<string>();
    return composed.flatMap((member) =>
      Object.entries(isObject(member.node.properties) ? member.node.properties : {}).flatMap(([key, node]) => {
        if (seen.has(key)) return [];
        seen.add(key);
        const childPointer = inputPointer(pointer, key);
        const resolved = recover((): ResolvedInput | undefined => resolve(member.document, node, childPointer), () => undefined);
        if (resolved === undefined) return [];
        const field = recover(() => input(key, resolved, required.includes(key), childPointer), () => undefined);
        return field === undefined ? [] : [field];
      }),
    );
  };

  const union = (node: JsonObject): Json[] | undefined => (Array.isArray(node.oneOf) ? node.oneOf : Array.isArray(node.anyOf) ? node.anyOf : undefined);

  type Variant = { readonly value: string; readonly member: ResolvedInput; readonly discriminator: ResolvedInput };
  const variants = (object: ResolvedInput, pointer: string): [string, Variant[]] => {
    const candidates = (union(object.node) ?? []).map((branch) => {
      const member = resolve(object.document, branch, pointer);
      const pins: [string, string | undefined, ResolvedInput][] = [];
      for (const composed of members(member, pointer, 0, [])) {
        for (const [key, property] of Object.entries(isObject(composed.node.properties) ? composed.node.properties : {})) {
          if (pins.some(([name]) => name === key)) continue;
          const resolved = resolve(composed.document, property, inputPointer(pointer, key));
          pins.push([key, typeof resolved.node.const === "string" ? resolved.node.const : undefined, resolved]);
        }
      }
      return { member, pins };
    });
    const pinned = (key: string): string[] | undefined => {
      const values = candidates.map(({ pins }) => pins.find(([name]) => name === key)?.[1]);
      if (values.some((value) => value === undefined)) return undefined;
      return values.every((value, index) => !values.slice(0, index).includes(value)) ? (values as string[]) : undefined;
    };
    const key = candidates[0]?.pins.map(([name]) => name).find((name) => pinned(name) !== undefined);
    if (key === undefined) return fail("malformed", pointer, "a payload union names no property every variant pins to a distinct string const");
    return [key, candidates.map(({ member, pins }) => { const [, value, discriminator] = pins.find(([name]) => name === key)!; return { value: value!, member, discriminator }; })];
  };

  const variantInputs = (object: ResolvedInput, pointer: string): ActionArgDef[] => {
    const [key, found] = variants(object, pointer);
    const selector = inputPointer(pointer, key);
    const discriminator = found[0]!.discriminator;
    const label = recover((): LocalizedLabel => {
      if (discriminator.ui.has("label")) {
        const localized = inputLocalizedText(discriminator.ui.get("label"));
        return typeof localized === "string" ? fail(localized, selector, "label names every locale") : localized;
      }
      const glossaryLabel = inputLabelGlossary().get(key);
      return glossaryLabel === undefined ? fail("labelMissing", selector, `${key} has no x-semio-ui.label and no glossary label`) : glossaryLabel;
    }, () => INPUT_LABEL_PLACEHOLDER);
    const description = recover((): LocalizedLabel | undefined => {
      if (!discriminator.ui.has("description")) return undefined;
      const localized = inputLocalizedText(discriminator.ui.get("description"));
      return typeof localized === "string" ? fail(localized, selector, "description names every locale") : localized;
    }, () => undefined);
    const options: ActionArgOption[] = found.map(({ value, member }) => ({
      value,
      label: recover((): LocalizedLabel => {
        if (member.ui.has("label")) {
          const localized = inputLocalizedText(member.ui.get("label"));
          return typeof localized === "string" ? fail(localized, selector, `variant ${value} names every locale`) : localized;
        }
        const glossaryLabel = inputLabelGlossary().get(value);
        return glossaryLabel === undefined ? fail("optionLabelMissing", selector, `variant ${value} has no x-semio-ui.label and no glossary label`) : glossaryLabel;
      }, () => INPUT_LABEL_PLACEHOLDER),
    }));
    const selectorInput: ActionArgDef = {
      id: inputPointer("", key),
      label,
      schema: { kind: "string", options },
      ...(options.length <= 4 ? { presentation: { kind: "segmented" } as ArgPresentation } : {}),
      required: true,
      ...(description === undefined ? {} : { description }),
    };
    return [selectorInput, ...found.flatMap(({ value, member }) => fields(member, pointer).map((field) => ({ ...field, group: value })))];
  };

  const inputs = (): ActionArgDef[] => {
    const payload = resolve(null, root, "");
    if (payload.node.properties === undefined && payload.node.allOf === undefined) {
      if (union(payload.node) !== undefined) return variantInputs(payload, "");
      const type = inputType(payload.node);
      if (type === "object" || type === undefined) return [];
      return fail("malformed", "", "a mutation payload schema describes an object");
    }
    return fields(payload, "");
  };

  return {
    inputs,
    audit(): InputSchemaAudit {
      let read: ActionArgDef[] = [];
      try {
        read = inputs();
      } catch (error) {
        if (!(error instanceof InputSchemaError)) throw error;
        findings!.push(error);
      }
      const distinct: InputSchemaError[] = [];
      for (const finding of findings!) if (!distinct.some((kept) => kept.code === finding.code && kept.pointer === finding.pointer && kept.message === finding.message)) distinct.push(finding);
      return { inputs: read, findings: distinct };
    },
    instance(payload: unknown): Record<string, unknown> {
      if (!isObject(payload as Json)) return fail("malformed", "", "a mutation payload is an object");
      const entries: Record<string, unknown> = { ...(payload as JsonObject) };
      let target = resolve(null, root, "");
      if (target.node.properties === undefined && target.node.allOf === undefined && union(target.node) !== undefined) {
        const [key, found] = variants(target, "");
        const chosen = entries[key];
        if (typeof chosen !== "string") return fail("malformed", "", `a union payload names its variant in ${key}`);
        target = found.find((variant) => variant.value === chosen)?.member ?? fail("malformed", "", `${chosen} is no variant of this payload union`);
      }
      for (const member of members(target, "", 0, [])) {
        for (const [key, node] of Object.entries(isObject(member.node.properties) ? member.node.properties : {})) {
          if (key in entries) continue;
          const constant = resolve(member.document, node, inputPointer("", key)).node.const;
          if (constant !== undefined) entries[key] = constant;
        }
      }
      return entries;
    },
  };
}
//#endregion 🔖️MutationInputs
//#endregion PluginRuntime
//#endregion 🔌️PluginAndAppContract
// #endregion 🛂️Manifest
