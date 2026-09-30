/** 🎯️ Cross-plugin app addresses belong to the manifest contract. */
import { dialectCoordinate, parseDialectCoordinate, type ArtifactDialect } from "../../🚪️io/🧬️schema/🟦️.ts";
export type AppRole = "viewer" | "editor";
export interface AppRef { pluginId: string; appId: string }

/** 🪟️ Serializes one dialect's app role. */
export function surfaceAppId(dialect: ArtifactDialect, role: AppRole): string {
  return `${dialectCoordinate(dialect)}#${role}`;
}

/** 🪟️ Resolves the final role separator against the canonical IO coordinate. */
export function parseSurfaceAppId(id: string): { dialect: ArtifactDialect; role: AppRole } {
  const separator = id.lastIndexOf("#");
  if (separator < 0) throw new Error("surface id is missing '#'");
  const role = id.slice(separator + 1);
  if (role !== "viewer" && role !== "editor") throw new Error("surface id requires viewer or editor role");
  return { dialect: parseDialectCoordinate(id.slice(0, separator)), role };
}

/** 🌐️ One non-empty text per shell locale — the `InputLabelLocales` export. */
export interface InputLabelLocales { readonly en: string; readonly de: string }
/** 🌐️ A localized input label: one text per locale, or the full terminology × locale matrix — the `InputLabel` export. */
export type InputLabel = InputLabelLocales | { readonly native: InputLabelLocales; readonly reuse: InputLabelLocales };
/** 🎛️ The value of one `x-semio-ui` annotation — the `InputUi` export (UI facts only; hard bounds stay in the standard keywords). */
export interface InputUi {
  readonly widget?: "slider" | "stepper" | "dial" | "toggle" | "select" | "segmented" | "text" | "multiline" | "vector" | "color" | "reference" | "hidden";
  readonly role?: "value" | "target" | "discriminator";
  readonly label?: InputLabel;
  readonly description?: InputLabel;
  readonly step?: number;
  readonly precision?: number;
  readonly softMin?: number;
  readonly softMax?: number;
  readonly snaps?: readonly number[];
  readonly snapSource?: { readonly step: true } | { readonly config: string } | { readonly snapshot: string };
  readonly unit?: string;
  readonly displayUnit?: string;
  readonly displayFactor?: number;
  readonly scale?: "linear" | "log";
  readonly group?: string;
  readonly order?: number;
  readonly options?: Readonly<Record<string, InputLabel>>;
  readonly ref?: { readonly kind?: string | readonly string[]; readonly domain?: string; readonly granularity?: string };
}
/** 🧷️ One payload-intrinsic invariant a schema states that JSON Schema draft-07 cannot express — the `SchemaInvariant` export. */
export interface SchemaInvariant { readonly id: string; readonly description: InputLabelLocales }
/** 🧷️ The value of one `x-semio-invariant` annotation — the `SchemaInvariants` export. */
export type SchemaInvariants = readonly SchemaInvariant[];
/** 📚️ The framework input-label glossary document — the `InputLabelGlossary` export. */
export interface InputLabelGlossary { readonly schema: "semio.manifest.input-label-glossary/v1"; readonly labels: Readonly<Record<string, InputLabelLocales>> }

const inputRecord = (value: unknown, what: string): Record<string, unknown> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${what} must be an object`);
  return value as Record<string, unknown>;
};
const inputText = (value: unknown, what: string): string => {
  if (typeof value !== "string" || value === "") throw new Error(`${what} must be a non-empty string`);
  return value;
};
const inputNumber = (value: unknown, what: string): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) throw new Error(`${what} must be a finite number`);
  return value;
};
const inputOnly = (record: Record<string, unknown>, keys: readonly string[], what: string): void => {
  const extra = Object.keys(record).find((key) => !keys.includes(key));
  if (extra !== undefined) throw new Error(`${what} carries the undeclared key ${extra}`);
};

/** 🌐️ Parses an `InputLabelLocales` value. */
export function parseInputLabelLocales(value: unknown): InputLabelLocales {
  const record = inputRecord(value, "InputLabelLocales");
  inputOnly(record, ["en", "de"], "InputLabelLocales");
  return { en: inputText(record.en, "InputLabelLocales.en"), de: inputText(record.de, "InputLabelLocales.de") };
}

/** 🧷️ Parses an `x-semio-invariant` value: a non-empty list of invariants, each a unique kebab-case id with a description per locale. */
export function parseSchemaInvariants(value: unknown): SchemaInvariants {
  if (!Array.isArray(value) || value.length === 0) throw new Error("SchemaInvariants must be a non-empty array");
  const ids = new Set<string>();
  return value.map((entry, index) => {
    const record = inputRecord(entry, `SchemaInvariants[${index}]`);
    inputOnly(record, ["id", "description"], "SchemaInvariant");
    const id = inputText(record.id, "SchemaInvariant.id");
    if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(id)) throw new Error(`SchemaInvariant.id ${id} must be kebab-case`);
    if (ids.has(id)) throw new Error(`SchemaInvariant.id ${id} is declared twice`);
    ids.add(id);
    return { id, description: parseInputLabelLocales(record.description) };
  });
}

/** 🌐️ Parses an `InputLabel` value. */
export function parseInputLabel(value: unknown): InputLabel {
  const record = inputRecord(value, "InputLabel");
  if ("native" in record || "reuse" in record) {
    inputOnly(record, ["native", "reuse"], "InputLabel");
    return { native: parseInputLabelLocales(record.native), reuse: parseInputLabelLocales(record.reuse) };
  }
  return parseInputLabelLocales(record);
}

/** 🎛️ Parses an `InputUi` value. */
export function parseInputUi(value: unknown): InputUi {
  const record = inputRecord(value, "InputUi");
  inputOnly(record, ["widget", "role", "label", "description", "step", "precision", "softMin", "softMax", "snaps", "snapSource", "unit", "displayUnit", "displayFactor", "scale", "group", "order", "options", "ref"], "InputUi");
  const oneOf = <T extends string>(entry: unknown, allowed: readonly T[], what: string): T => {
    if (typeof entry !== "string" || !(allowed as readonly string[]).includes(entry)) throw new Error(`${what} must be one of ${allowed.join(", ")}`);
    return entry as T;
  };
  const ui: Record<string, unknown> = {};
  if (record.widget !== undefined) ui.widget = oneOf(record.widget, ["slider", "stepper", "dial", "toggle", "select", "segmented", "text", "multiline", "vector", "color", "reference", "hidden"] as const, "InputUi.widget");
  if (record.role !== undefined) ui.role = oneOf(record.role, ["value", "target", "discriminator"] as const, "InputUi.role");
  if (record.label !== undefined) ui.label = parseInputLabel(record.label);
  if (record.description !== undefined) ui.description = parseInputLabel(record.description);
  if (record.step !== undefined && inputNumber(record.step, "InputUi.step") <= 0) throw new Error("InputUi.step must be positive");
  if (record.step !== undefined) ui.step = record.step;
  if (record.precision !== undefined && !(Number.isInteger(record.precision) && (record.precision as number) >= 0 && (record.precision as number) <= 15)) throw new Error("InputUi.precision must be an integer 0..=15");
  if (record.precision !== undefined) ui.precision = record.precision;
  for (const key of ["softMin", "softMax", "displayFactor"] as const) if (record[key] !== undefined) ui[key] = inputNumber(record[key], `InputUi.${key}`);
  if (ui.displayFactor === 0) throw new Error("InputUi.displayFactor must be non-zero");
  if (record.snaps !== undefined) {
    if (!Array.isArray(record.snaps)) throw new Error("InputUi.snaps must be an array");
    ui.snaps = record.snaps.map((snap, index) => inputNumber(snap, `InputUi.snaps[${index}]`));
  }
  if (record.snapSource !== undefined) {
    const source = inputRecord(record.snapSource, "InputUi.snapSource");
    const entries = Object.entries(source);
    const [name, entry] = entries.length === 1 ? entries[0]! : ["", undefined];
    if (name === "step" && entry === true) ui.snapSource = { step: true };
    else if (name === "config") ui.snapSource = { config: inputText(entry, "InputUi.snapSource.config") };
    else if (name === "snapshot" && typeof entry === "string" && (entry === "" || entry.startsWith("/"))) ui.snapSource = { snapshot: entry };
    else throw new Error("InputUi.snapSource must be {step: true}, {config: key} or {snapshot: pointer}");
  }
  for (const key of ["unit", "displayUnit", "group"] as const) if (record[key] !== undefined) ui[key] = inputText(record[key], `InputUi.${key}`);
  if (record.scale !== undefined) ui.scale = oneOf(record.scale, ["linear", "log"] as const, "InputUi.scale");
  if (record.order !== undefined && !Number.isInteger(record.order)) throw new Error("InputUi.order must be an integer");
  if (record.order !== undefined) ui.order = record.order;
  if (record.options !== undefined) ui.options = Object.fromEntries(Object.entries(inputRecord(record.options, "InputUi.options")).map(([option, label]) => [option, parseInputLabel(label)]));
  if (record.ref !== undefined) {
    const reference = inputRecord(record.ref, "InputUi.ref");
    inputOnly(reference, ["kind", "domain", "granularity"], "InputUi.ref");
    const kind = reference.kind;
    if (kind !== undefined && !(typeof kind === "string" ? kind !== "" : Array.isArray(kind) && kind.length > 0 && kind.every((entry) => typeof entry === "string" && entry !== ""))) throw new Error("InputUi.ref.kind must be a non-empty string or array of them");
    ui.ref = { ...(kind === undefined ? {} : { kind }), ...(reference.domain === undefined ? {} : { domain: inputText(reference.domain, "InputUi.ref.domain") }), ...(reference.granularity === undefined ? {} : { granularity: inputText(reference.granularity, "InputUi.ref.granularity") }) };
  }
  return ui as InputUi;
}

/** 📚️ Parses an `InputLabelGlossary` document. */
export function parseInputLabelGlossary(value: unknown): InputLabelGlossary {
  const record = inputRecord(value, "InputLabelGlossary");
  inputOnly(record, ["schema", "labels"], "InputLabelGlossary");
  if (record.schema !== "semio.manifest.input-label-glossary/v1") throw new Error("InputLabelGlossary.schema must be semio.manifest.input-label-glossary/v1");
  const labels = Object.fromEntries(Object.entries(inputRecord(record.labels, "InputLabelGlossary.labels")).map(([name, label]) => [name, parseInputLabelLocales(label)]));
  if (Object.keys(labels).length < 200) throw new Error("InputLabelGlossary.labels must name at least 200 inputs");
  return { schema: "semio.manifest.input-label-glossary/v1", labels };
}
