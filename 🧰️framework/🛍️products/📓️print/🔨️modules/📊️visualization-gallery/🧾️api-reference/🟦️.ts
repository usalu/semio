/** 🧾️ The machine-readable twin of the public visualization API: the documented key lines, the Keys regions of the LaTeX packages and `vizApiReference`. */
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { basename, join } from "node:path";
import { type LocalizedText, type VizFamilyOptions, type VizFamilyOption, type VizOptionSyntaxDescriptor } from "../../../🧬️schema/🟦️.ts";
import { loadVizSchema, VIZ_API_DIR, VIZ_API_DOCUMENT, VIZ_LATEX_DIR } from "../🟦️.ts";
import { helperDeclaredKeyNames, type KeysModel, keysOfPath, scanKeys, vizKeyPath, vizKeysModel } from "../🔑️implemented-keys/🟦️.ts";
import { vizPrintedKeyRows, vizPrintedOwnerRows, vizPrintedSections, vizPrintedSource } from "../📋️printed-key-tables/🟦️.ts";

//#region 🔖️ApiReference
/** 🔑 One documented option of a key family: its name, l3keys type, default and bilingual meaning. */
export type VizApiKey = Readonly<{
  readonly name: string;
  readonly type: string;
  readonly default: string;
  readonly description: LocalizedText;
}>;

/** 🗝️ One `%region 🔖️Keys…` block: the l3keys family it documents and the keys it declares. */
export type VizApiKeyFamily = Readonly<{ readonly region: string; readonly scope: string; readonly keys: readonly VizApiKey[] }>;

/** 🔤 One public `\SemioViz…` command or `Viz…` environment, with its xparse signature. */
export type VizApiCommand = Readonly<{
  readonly name: string;
  readonly kind: "command" | "environment";
  readonly signature: string;
  readonly note: string;
}>;

/** 📦 One LaTeX package of the library: what it exposes and what it can be configured with. */
export type VizApiPackage = Readonly<{
  readonly package: string;
  readonly commands: readonly VizApiCommand[];
  readonly keyFamilies: readonly VizApiKeyFamily[];
}>;

/** 📖 The machine-readable twin of `🔓️viz-api.tex`. */
export type VizApiReference = Readonly<{
  readonly schemaVersion: 1;
  readonly packages: readonly VizApiPackage[];
  readonly families: readonly Readonly<{
    readonly name: string;
    readonly owner: string;
    readonly options: readonly (VizApiKey & Omit<VizOptionSyntaxDescriptor, "type"> & Pick<VizFamilyOption, "minimum" | "exclusiveMinimum" | "maximum" | "enum">)[];
  }>[];
  readonly totals: Readonly<{ readonly packages: number; readonly commands: number; readonly keys: number; readonly families: number }>;
}>;

const API_COMMAND_PATTERN = /\\New(?:Expandable)?DocumentCommand\s*(\\SemioViz[A-Za-z]*)\s*\{/;
const API_ENVIRONMENT_PATTERN = /\\NewDocumentEnvironment\s*\{\s*(Viz[A-Za-z]*)\s*\}\s*\{/;

/** 🧷 The content of the brace group that starts at `from`; an xparse signature contains braces of
 * its own (`O{}`, `m O{default}`), so it cannot be read with a non-greedy regex. */
function balancedGroup(line: string, from: number): string {
  let depth = 0;
  for (let cursor = from; cursor < line.length; cursor += 1) {
    if (line[cursor] === "{") depth += 1;
    else if (line[cursor] === "}" && (depth -= 1) === 0) return line.slice(from + 1, cursor);
  }
  return line.slice(from + 1);
}

/** 🗒️ The emoji-prefixed `%` note directly above a definition, if there is one. */
function apiNoteAbove(lines: readonly string[], index: number): string {
  for (let cursor = index - 1; cursor >= 0 && cursor >= index - 3; cursor -= 1) {
    const line = lines[cursor]!.trimStart();
    if (!line.startsWith("%") || line.startsWith("%region") || line.startsWith("%endregion")) break;
    return line.replace(/^%+\s*/, "").trim();
  }
  return "";
}

/** 🧹 Collapses runs of whitespace so a signature reads the way it is written in the manual. */
function apiCompact(value: string): string {
  return value.replace(/\s+/g, " ").trim();
}

/** 🔑 Parses one documented key line of a `%region 🔖️Keys…` block into its four fields.
 *
 * Three spellings are in use across the kernel packages and all three are accepted:
 * `name (type, default)  meaning`, `name  type  default <default>  meaning` and
 * `name  (type)  meaning, default <default>`. A leading name list (`x, y`) yields one key each.
 */
function parseVizApiKeyLine(line: string): VizApiKey[] {
  const body = line.replace(/^%\s{2,}/, "");
  if (body === line || body.length === 0) return [];
  const paren = /^([A-Za-z0-9@_,\s-]+?)\s*\(([^)]*)\)\s*(.*)$/.exec(body);
  const spaced = /^([A-Za-z0-9@_,\s-]+?)\s{2,}(\S+)\s+default\s+(\S+)\s{2,}(.*)$/.exec(body);
  let names = "";
  let type = "";
  let fallback = "";
  let description = "";
  if (spaced !== null) {
    const[match,foundNames,foundType,foundFallback,foundDescription]=spaced;if(match===undefined||foundNames===undefined||foundType===undefined||foundFallback===undefined||foundDescription===undefined)return [];names=foundNames;type=foundType;fallback=foundFallback;description=foundDescription;
  } else if (paren !== null) {
    const [,rawNames,inside,rest]=paren;if(rawNames===undefined||inside===undefined)return [];
    names=rawNames;
    const parts=inside.split(",").map((part) => part.trim());
    type = parts[0] ?? "";
    fallback = parts.slice(1).join(", ");
    description = rest ?? "";
  } else {
    const plain = /^([A-Za-z0-9@_,\s-]+?)\s+default\s+(\S+)\s{2,}(.*)$/.exec(body);
    if (plain === null) return [];
    const[,foundNames,foundFallback,foundDescription]=plain;if(foundNames===undefined||foundFallback===undefined||foundDescription===undefined)return [];names=foundNames;fallback=foundFallback;description=foundDescription;
  }
  const trailing = /,?\s*default\s+([^.,]+)[.,]?\s*$/.exec(description ?? "");
  if (trailing !== null && (fallback ?? "").length === 0) {
    fallback = trailing[1]!.trim();
    description = description!.slice(0, trailing.index).trim();
  }
  const bare = (value: string): string => apiCompact(value).replace(/`/g, "");
  return names!.split(",").map((name) => name.trim()).filter((name) => name.length > 0 && !name.includes(" "))
    .map((name) => ({ name, type: bare(type ?? ""), default: bare(fallback ?? ""), description: { en: apiCompact(description ?? ""), de: "" } }));
}

/** 🌍️ Translations are read only from explicitly selected rendered owner scopes. */
export function vizDocumentKeyGerman(source?: string, scopes: readonly string[] = []): ReadonlyMap<string, string> {
  const path = join(VIZ_API_DIR, VIZ_API_DOCUMENT);
  if (source === undefined && !existsSync(path)) return new Map();
  return vizDocumentGermanReader(source ?? readFileSync(path, "utf8"))(scopes);
}

/** 🗂️️ A reference scan is shared by every package region in one metadata collection. */
function vizDocumentGermanReader(text: string): (scopes: readonly string[]) => ReadonlyMap<string, string> {
  const tables = vizPrintedKeyRows(text);
  const declared = new Map([...vizPrintedSource(text).matchAll(/\\ApiScopeSource\{([a-z0-9-]+)\}\{([^}]+)\}/g)].map(match => [match[1]!, match[2]!.replaceAll("\\_", "_")])), sections = vizPrintedSections(text), model = vizKeysModel();
  return scopes => {
  const owners = new Set(scopes.flatMap(scope => {
    const family = /^semio\s*\/\s*viz\s*\/\s*family\s*\/\s*([a-z0-9-]+)$/.exec(scope);
    return [scope, ...(family ? [family[1]!] : []), ...[...declared].filter(([, path]) => path === scope).map(([owner]) => owner)];
  }));
  const seen = new Map<string, Set<string>>();
  for (const owner of owners) for (const row of vizPrintedOwnerRows(owner, tables, declared, sections, model)) {
    if (!row.meaning.de.trim()) continue;
    for (const name of row.names) seen.set(name, (seen.get(name) ?? new Set()).add(row.meaning.de));
  }
  return new Map([...seen].filter(([, meanings]) => meanings.size === 1).map(([name, meanings]) => [name, [...meanings][0]!]));
  };
}

/** 🧭️ Resolves a documented Keys region to its actual following l3keys declaration. */
function vizDocumentRegionScopes(source: string, start: number): readonly string[] {
  const tail = source.slice(start), next = /\n%region[^\n]*Keys/.exec(tail);
  const region = tail.slice(0, next?.index ?? tail.length), scopes = new Set<string>();
  for (const declaration of region.matchAll(/\\keys_define:nn\s*\{([^}]+)\}/g)) {
    const scope = declaration[1]!.trim();
    if (!scope.includes("#")) scopes.add(scope);
    else {
      const before = source.slice(0, start + declaration.index!);
      const helper = [...before.matchAll(/\\cs_(?:new|set)[a-z_]*:Npn\s+\\([a-z_]+):/g)].at(-1)?.[1];
      if (helper) scopes.add("helper:" + helper);
    }
  }
  return [...scopes];
}

/** 🧭️ Keeps repeated Keys region names bound to their own physical source position. */
export function vizDocumentKeyRegions(source: string, model: KeysModel = vizKeysModel([source])): readonly { region: string; keys: readonly VizApiKey[]; scopes: readonly string[] }[] {
  const entries: { region: string; keys: readonly VizApiKey[]; scopes: readonly string[] }[] = [];
  let region: string | undefined, start = 0, offset = 0, keys: VizApiKey[] = [];
  for (const line of source.split("\n")) {
    const trimmed = line.trim();
    if (trimmed.startsWith("%region") && trimmed.includes("Keys")) {
      region = trimmed.replace(/^%region\s*/, "");
      start = offset;
      keys = [];
    } else if (trimmed.startsWith("%endregion") && region !== undefined) {
      if (keys.length) entries.push({ region, keys, scopes: vizDocumentRegionScopes(source, start) });
      region = undefined;
    } else if (region !== undefined && trimmed.startsWith("%")) keys.push(...parseVizApiKeyLine(line.trimStart()));
    offset += line.length + 1;
  }
  if (region !== undefined && keys.length) entries.push({ region, keys, scopes: vizDocumentRegionScopes(source, start) });
  const literal = scanKeys(source).literals;
  return entries.map(entry => {
    const accepted = new Set(entry.scopes.flatMap(scope => scope.startsWith("helper:")
      ? [...helperDeclaredKeyNames(model, scope.slice(7), new Set())]
      : [...keysOfPath(model, scope, new Set()), ...literal.filter(unit => unit.path === scope).flatMap(unit => unit.keys)]));
    return { ...entry, keys: entry.keys.filter(key => accepted.has(key.name)) };
  });
}

/** 🧭️ Selects complete rendered rows from one actual native owner scope. */
export function vizDocumentKeyReader(reference: string, model: KeysModel): (scope: string) => ReadonlyMap<string, VizApiKey> {
  const text = vizPrintedSource(reference), tables = vizPrintedKeyRows(text), sections = vizPrintedSections(text);
  const declared = new Map([...text.matchAll(/\\ApiScopeSource\{([a-z0-9-]+)\}\{([^}]+)\}/g)].map(match => [match[1]!, match[2]!.replaceAll("\\_", "_")]));
  return scope => {
    const bound = [...declared].filter(([, path]) => path.replace(/\s*\/\s*/g, "/") === scope.replace(/\s*\/\s*/g, "/")).map(([owner]) => owner);
    const family = /^semio\s*\/\s*viz\s*\/\s*family\s*\/\s*([a-z0-9-]+)$/.exec(scope)?.[1];
    const owners = bound.length ? bound : family ? [family] : [scope];
    const result = new Map<string, VizApiKey>();
    for (const owner of owners) for (const row of vizPrintedOwnerRows(owner, tables, declared, sections, model)) for (const name of row.names) {
      if (!row.type.trim() || !row.defaultPresent || !row.meaning.en.trim() || !row.meaning.de.trim()) throw new Error("Incomplete printed key contract: " + scope + ":" + name);
      const key: VizApiKey = { name, type: row.type, default: row.default, description: row.meaning }, prior = result.get(name);
      if (prior && JSON.stringify(prior) !== JSON.stringify(key)) throw new Error("Ambiguous printed key contract: " + scope + ":" + name);
      result.set(name, key);
    }
    if (bound.length && family) for (const row of vizPrintedOwnerRows(family, tables, declared, sections, model)) for (const name of row.names) {
      if (result.has(name)) continue;
      if (!row.type.trim() || !row.defaultPresent || !row.meaning.en.trim() || !row.meaning.de.trim()) throw new Error("Incomplete printed key contract: " + scope + ":" + name);
      result.set(name, { name, type: row.type, default: row.default, description: row.meaning });
    }
    return result;
  };
}

/** 🏷️ Shared helper vocabularies are published only under an explicit printed source binding. */
export function vizDocumentHelperScopes(reference: string): ReadonlySet<string> {
  return new Set([...vizPrintedSource(reference).matchAll(/\\ApiScopeSource\{[a-z0-9-]+\}\{(helper:[^}]+)\}/g)].map(match => match[1]!));
}
/** 📚️ Projects each native declaration scope through its complete rendered row contracts. */
export function vizDocumentKeyFamilies(implementation: string, reference: string, model: KeysModel = vizKeysModel([implementation])): readonly VizApiKeyFamily[] {
  return vizDocumentKeyFamilyProjection(implementation, model, vizDocumentKeyReader(reference, model), vizDocumentHelperScopes(reference));
}

/** 🧾️ Native public scopes include actual installed controls and explicitly documented helpers. */
export function vizDocumentKeyScopes(implementation: string, model: KeysModel, helpers: ReadonlySet<string>): ReadonlyMap<string, { region: string; names: ReadonlySet<string> }> {
  const entries = new Map<string, { region: string; names: Set<string> }>();
  const add = (scope: string, region: string, names: readonly string[]): void => {
    scope = vizKeyPath(scope);
    const entry = entries.get(scope) ?? { region, names: new Set<string>() };
    for (const name of names) if (!name.startsWith("@forward:")) entry.names.add(name);
    entries.set(scope, entry);
  };
  for (const entry of vizDocumentKeyRegions(implementation, model)) for (const scope of entry.scopes) {
    const accepted = scope.startsWith("helper:") ? helperDeclaredKeyNames(model, scope.slice(7), new Set()) : keysOfPath(model, scope, new Set());
    add(scope, entry.region, entry.keys.map(key => key.name).filter(name => accepted.has(name)));
  }
  for (const unit of scanKeys(implementation).literals) add(unit.path, "Keys", [...keysOfPath(model, unit.path, new Set())]);
  for (const match of implementation.matchAll(/\\cs_(?:new|set)[a-z_]*:Npn\s+\\([a-z_]+):/g)) {
    const macro = match[1]!;
    if (helpers.has("helper:" + macro) && model.helpers.get(macro)?.templates.length) add("helper:" + macro, "Keys", [...helperDeclaredKeyNames(model, macro, new Set())]);
  }
  return entries;
}

/** 🗝️ Shares the parsed reference across all physical packages in one collection. */
function vizDocumentKeyFamilyProjection(implementation: string, model: KeysModel, read: (scope: string) => ReadonlyMap<string, VizApiKey>, helpers: ReadonlySet<string>): readonly VizApiKeyFamily[] {
  const families: VizApiKeyFamily[] = [];
  const entries = vizDocumentKeyScopes(implementation, model, helpers);
  for (const [scope, entry] of entries) {
    if (!entry.names.size) continue;
    const rows = read(scope);
    const keys = [...entry.names].map(name => {
      const key = rows.get(name);
      if (!key) throw new Error("Missing printed key contract: " + scope + ":" + name);
      return key;
    });
    families.push({ region: entry.region, scope, keys });
  }
  return families;
}
/** 📖 The whole public API as data: every package with its commands and its documented key families,
 * plus the schema's family option vocabulary. This is the twin of `🔓️viz-api.tex` that tooling
 * reads; the document is what a person reads. */
export function vizApiReference(): VizApiReference {
  const schema = loadVizSchema();
  const reference = readFileSync(join(VIZ_API_DIR, VIZ_API_DOCUMENT), "utf8");

  const packages: VizApiPackage[] = [];
  const model = vizKeysModel(), readKeys = vizDocumentKeyReader(reference, model), helpers = vizDocumentHelperScopes(reference);
  let commandCount = 0;
  let keyCount = 0;
  for (const file of readdirSync(VIZ_LATEX_DIR).filter((name) => name.startsWith("semio-viz") && name.endsWith(".sty")).sort()) {
    const source = readFileSync(join(VIZ_LATEX_DIR, file), "utf8"), lines = source.split("\n");
    const commands: VizApiCommand[] = [];
    const keyFamilies = vizDocumentKeyFamilyProjection(source, model, readKeys, helpers);
    for (const [index, line] of lines.entries()) {
      const trimmed = line.trim();
      const command = API_COMMAND_PATTERN.exec(line);
      const environment = command === null ? API_ENVIRONMENT_PATTERN.exec(line) : null;
      const match = command ?? environment;
      if (match === null) continue;
      commands.push({
        name: match[1]!,
        kind: command === null ? "environment" : "command",
        signature: apiCompact(balancedGroup(line, match.index! + match[0]!.length - 1)),
        note: apiNoteAbove(lines, index),
      });
    }
    if (commands.length === 0 && keyFamilies.length === 0) continue;
    commandCount += commands.length;
    for (const family of keyFamilies) keyCount += family.keys.length;
    packages.push({ package: basename(file, ".sty"), commands, keyFamilies });
  }
  const families = Object.entries(schema["x-semio-family-options"] as Record<string, VizFamilyOptions>)
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([name, family]) => ({
      name,
      owner: (family as { owner?: string }).owner ?? "",
      options: Object.entries(family.options ?? {}).map(([option, spec]) => {
        const typed = spec as VizFamilyOption;
        return {
          name: option,
          syntax: typed.syntax,
          ...(typed.items ? { items: typed.items } : {}),
          ...(typed.itemCount !== undefined ? { itemCount: typed.itemCount } : {}),
          ...(typed.minimum !== undefined ? { minimum: typed.minimum } : {}),
          ...(typed.exclusiveMinimum !== undefined ? { exclusiveMinimum: typed.exclusiveMinimum } : {}),
          ...(typed.maximum !== undefined ? { maximum: typed.maximum } : {}),
          ...(typed.enum ? { enum: typed.enum } : {}),
          type: typeof typed.type === "string" ? typed.type : typed.type?.join(" | ") ?? "",
          default: typed.default === undefined || typed.default === null ? "" : String(typed.default),
          description: { en: typed.description?.en ?? "", de: typed.description?.de ?? "" },
        };
      }),
    }));
  return {
    schemaVersion: 1,
    packages,
    families,
    totals: { packages: packages.length, commands: commandCount, keys: keyCount, families: families.length },
  };
}
//#endregion 🔖️ApiReference
