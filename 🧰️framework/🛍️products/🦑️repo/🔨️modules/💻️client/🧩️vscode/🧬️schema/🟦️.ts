//#region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
/**
 * 🧬️ TypeScript projection of the `repo.client.vscode` technology catalog contract.
 *
 * Every exported type and `parse<Export>` function implements one `$defs` entry of the sibling
 * {@link https://semio.tech/schema/repo/client/vscode/schema.json 🔣️.json} draft-07 document.
 * Parsers are hand-written (no code generation, no external validator) and are the only gate the
 * extension's `🗂️technologies.json` passes through.
 */
//#endregion 🧲️Header

//#region 🔖️Contracts
/** 🪪️ Canonical `$id` of the schema these parsers implement. */
export const VSCODE_SCHEMA_ID = "https://semio.tech/schema/repo/client/vscode/schema.json";

/** 🎁️ Outcome of one parse, free of any implementation-specific error type. */
export type ParseResult<T> = { readonly success: true; readonly data: T } | { readonly success: false; readonly error: { readonly message: string } };

export interface TechnologyCatalogEntry {
  readonly id: string;
  readonly emoji: string;
  readonly iconId: string;
  readonly label: string;
  readonly filterable: boolean;
}

export type TechnologyCatalog = readonly TechnologyCatalogEntry[];
//#endregion 🔖️Contracts

//#region 🧱️Primitives
type Checked<T> = { readonly ok: true; readonly value: T } | { readonly ok: false; readonly message: string };
type Check<T> = (value: unknown, path: string) => Checked<T>;

const KEBAB = /^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/;
const NO_SPACE = /^\S+$/;
const TRIMMED = /^\S(?:.*\S)?$/;

function fail(path: string, expectation: string): Checked<never> {
  return { ok: false, message: `${path}: ${expectation}` };
}

function text(expression: RegExp, minLength: number, maxLength: number, expectation: string): Check<string> {
  return (value, path) => {
    if (typeof value !== "string") return fail(path, "expected string");
    const length = [...value].length;
    if (length < minLength) return fail(path, `expected at least ${minLength} character(s)`);
    if (length > maxLength) return fail(path, `expected at most ${maxLength} character(s)`);
    if (!expression.test(value)) return fail(path, expectation);
    return { ok: true, value };
  };
}

const boolean: Check<boolean> = (value, path) => (typeof value === "boolean" ? { ok: true, value } : fail(path, "expected boolean"));

type Fields<T> = { readonly [K in keyof T]: Check<T[K]> };

function record<T>(fields: Fields<T>): Check<T> {
  return (value, path) => {
    if (typeof value !== "object" || value === null || Array.isArray(value)) return fail(path, "expected object");
    const source = value as Readonly<Record<string, unknown>>;
    for (const key of Object.keys(source)) if (!(key in fields)) return fail(`${path}.${key}`, "unexpected property");
    const data: Record<string, unknown> = {};
    for (const [key, check] of Object.entries(fields) as [string, Check<unknown>][]) {
      if (source[key] === undefined) return fail(`${path}.${key}`, "required property missing");
      const checked = check(source[key], `${path}.${key}`);
      if (!checked.ok) return checked;
      data[key] = checked.value;
    }
    return { ok: true, value: data as T };
  };
}

function parser<T>(check: Check<T>): (value: unknown) => ParseResult<T> {
  return (value) => {
    const checked = check(value, "value");
    return checked.ok ? { success: true, data: checked.value } : { success: false, error: { message: checked.message } };
  };
}
//#endregion 🧱️Primitives

//#region 🧬️Checks
const technologyCatalogEntry = record<TechnologyCatalogEntry>({
  id: text(KEBAB, 3, 32, "expected a kebab-case entity kind id"),
  emoji: text(NO_SPACE, 1, 8, "expected an emoji without whitespace"),
  iconId: text(KEBAB, 3, 32, "expected a kebab-case icon id"),
  label: text(TRIMMED, 2, 32, "expected a trimmed label"),
  filterable: boolean,
});

const technologyCatalog: Check<TechnologyCatalog> = (value, path) => {
  if (!Array.isArray(value)) return fail(path, "expected array");
  if (value.length === 0) return fail(path, "expected at least 1 entry");
  const entries: TechnologyCatalogEntry[] = [];
  const seen = new Set<string>();
  for (let index = 0; index < value.length; index++) {
    const checked = technologyCatalogEntry(value[index], `${path}.${index}`);
    if (!checked.ok) return checked;
    const identity = JSON.stringify([checked.value.id, checked.value.emoji, checked.value.iconId, checked.value.label, checked.value.filterable]);
    if (seen.has(identity)) return fail(`${path}.${index}`, "expected unique entries");
    seen.add(identity);
    entries.push(checked.value);
  }
  return { ok: true, value: entries };
};
//#endregion 🧬️Checks

//#region 🚪️Parsers
/** 🏷️ Parses one `TechnologyCatalogEntry`. */
export const parseTechnologyCatalogEntry = parser(technologyCatalogEntry);

/** 📚️ Parses a whole `TechnologyCatalog`. */
export const parseTechnologyCatalog = parser(technologyCatalog);

/** 🗺️ Export id → parser, asserted key-for-key equal to the document's `$defs`. */
export const VSCODE_PARSERS = {
  TechnologyCatalog: parseTechnologyCatalog,
  TechnologyCatalogEntry: parseTechnologyCatalogEntry,
} as const;
//#endregion 🚪️Parsers
