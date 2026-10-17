/** 📋️ The printed key tables of the visualization API: the cells, sections, owner rows and key rows of the LaTeX tables. */
import { type LocalizedText } from "../../../🧬️schema/🟦️.ts";
import { helperKeysOn, type KeysModel, pathsReachedBy, texGroup, texSkipSpace, vizKeyPath } from "../🔑️implemented-keys/🟦️.ts";

//#region 🔖️ApiReference
/** 🧷 The two balanced arguments of the last `\ApiText{en}{de}` on a line, if there is one. */
function lastApiTextArguments(line: string): readonly [string, string] | undefined {
  const start = line.lastIndexOf("\\ApiText{");
  if (start < 0) return undefined;
  const argument = (from: number): readonly [string, number] | undefined => {
    if (line[from] !== "{") return undefined;
    let depth = 0;
    for (let cursor = from; cursor < line.length; cursor += 1) {
      if (line[cursor] === "{") depth += 1;
      else if (line[cursor] === "}" && (depth -= 1) === 0) return [line.slice(from + 1, cursor), cursor + 1];
    }
    return undefined;
  };
  const first = argument(start + "\\ApiText".length);
  const second = first === undefined ? undefined : argument(first[1]);
  return second === undefined ? undefined : [first![0], second[0]];
}

//#endregion 🔖️ApiReference
//#region 🔖️Coverage
/** 📐️ Splits a table row only at unescaped, unnested TeX column separators. */
function vizPrintedCells(row: string, separator = "&"): string[] {
  const cells: string[] = [];
  let depth = 0, start = 0;
  for (let index = 0; index < row.length; index++) {
    if (row[index] === "\\") { index++; continue; }
    if (row[index] === "{") depth++;
    else if (row[index] === "}") depth--;
    else if (row[index] === separator && depth === 0) { cells.push(row.slice(start, index)); start = index + 1; }
  }
  cells.push(row.slice(start));
  return cells;
}

/** 🧹️ Literal code samples and TeX comments never become rendered documentation evidence. */
export function vizPrintedSource(source: string): string {
  let text = source.replace(/\\begin\{(verbatim\*?|Verbatim|lstlisting|minted|SemioCode|SemioCodeBlock)\}[\s\S]*?\\end\{\1\}/g, "");
  text = text.replace(/\\verb\*?([^\sA-Za-z])[\s\S]*?\1/g, "");
  let result = "", slashes = 0;
  for (let index = 0; index < text.length; index++) {
    const char = text[index]!;
    if (char === "%" && slashes % 2 === 0) {
      while (index < text.length && text[index] !== "\n") index++;
      if (index < text.length) result += "\n";
    } else result += char;
    slashes = char === "\\" ? slashes + 1 : 0;
  }
  return result;
}

/** 🔤️ One rendered row, retaining its type, default and both localized meanings. */
export type VizPrintedKeyRow = Readonly<{ names: readonly string[]; type: string; default: string; defaultPresent: boolean; meaning: LocalizedText }>;

/** 🧬️ The neutral expected contract of one documented control. */
export type VizPrintedKeySpec = Readonly<{ type: string; default?: string; unit?: string }>;

/** 🧭️ Every section boundary ends the preceding documentation owner. */
export function vizPrintedSections(source: string): ReadonlyMap<string, string> {
  const text = vizPrintedSource(source), headings = [...text.matchAll(/\\(?:subsection|section|chapter)\*?\{(?:\\(?:Key|ApiTitle)\{([a-z0-9-]+)\})?/g)], sections = new Map<string, string>();
  for (const [index, heading] of headings.entries()) if (heading[1]) sections.set(heading[1], text.slice(heading.index! + heading[0].length, headings[index + 1]?.index ?? text.length));
  const bindings = [...text.matchAll(/\\ApiScopeSource\{([a-z0-9-]+)\}\{[^}]+\}/g)];
  for (const [index, binding] of bindings.entries()) {
    if (sections.has(binding[1]!)) continue;
    const end = Math.min(bindings[index + 1]?.index ?? text.length, headings.find(heading => heading.index! > binding.index!)?.index ?? text.length);
    sections.set(binding[1]!, text.slice(binding.index! + binding[0].length, end));
  }
  return sections;
}

/** 🪢️ A printed shared table is inherited only through actual declaration forwards. */
export function vizPrintedOwnerRows(owner: string, tables: Readonly<Record<string, readonly VizPrintedKeyRow[]>>, declared: ReadonlyMap<string, string>, sections: ReadonlyMap<string, string>, model: KeysModel, seen = new Set<string>()): readonly VizPrintedKeyRow[] {
  if (seen.has(owner)) return [];
  seen.add(owner);
  const rows = [...(tables[owner] ?? [])], root = declared.get(owner), scopes = new Set<string>();
  const visit = (scope: string): void => {
    scope = vizKeyPath(scope);
    if (scopes.has(scope)) return;
    scopes.add(scope);
    if (scope.startsWith("helper:")) {
      const helper = model.helpers.get(scope.slice(7));
      for (const macro of helper?.forwards ?? []) visit("helper:" + macro);
      for (const marker of helper?.templates.flatMap(template => [...template.keys]) ?? []) if (marker.startsWith("@forward:")) visit(marker.slice(9));
    } else {
      for (const marker of model.literal.get(scope) ?? []) if (marker.startsWith("@forward:")) visit(marker.slice(9));
      for (const install of model.installs) if (helperKeysOn(model, install.macro, install.argument, scope, new Set()).size > 0) visit("helper:" + install.macro);
    }
  };
  if (root) visit(root);
  const familyName = /^semio\s*\/\s*viz\s*\/\s*family\s*\/\s*([a-z0-9-]+)$/.exec(root ?? "")?.[1] ?? owner;
  const family = model.families.get(familyName);
  if (family) {
    visit(`semio / viz / family / ${familyName}`);
    scopes.add("registry");
    for (const scope of pathsReachedBy(model, family, 0, new Set())) if (!scope.includes("#")) visit(scope);
  }
  for (const link of (sections.get(owner) ?? "").matchAll(/\\ApiKeyScope\{([a-z0-9-]+)\}/g)) {
    const target = link[1]!, scope = declared.get(target);
    if (!scope || !scopes.has(vizKeyPath(scope))) continue;
    for (const row of vizPrintedOwnerRows(target, tables, declared, sections, model, seen)) {
      const names = row.names.filter(name => !rows.some(existing => existing.names.includes(name)));
      if (names.length) rows.push({ ...row, names });
    }
  }
  return rows;
}

/** ✂️ Extracts visible scalar text from a table's type or default cell. */
function vizPrintedScalar(cell: string, defaultCell = false): string {
  if (defaultCell && cell.trim() === "---") return "";
  const localized = lastApiTextArguments(cell);
  if (localized) cell = localized[0] === "empty" ? "" : localized[0];
  return cell.replace(/\\Key\{([^{}]*)\}/g, "$1").replace(/[{}]/g, "").replace(/\\(?:texttt|detokenize)\s*/g, "").trim();
}

/** 📖️ Reads balanced rendered key-table rows in their explicitly titled owner scope. */
export function vizPrintedKeyRows(source: string): Readonly<Record<string, readonly VizPrintedKeyRow[]>> {
  const tables: Record<string, VizPrintedKeyRow[]> = {};
  const text = vizPrintedSource(source);
  for (const match of text.matchAll(/\\SemioTableLong(?:\[[^\]]*\])?\s*\{/g)) {
    let cursor = match.index! + match[0].length - 1;
    const groups: string[] = [];
    for (let index = 0; index < 4; index++) {
      const group = texGroup(text, texSkipSpace(text, cursor));
      groups.push(group.body);
      cursor = group.end;
    }
    const owner = /^\\(?:Key|ApiTitle)\{([a-z0-9-]+)\}/.exec(groups[0]!);
    if (!owner || !groups[2]!.includes("\\ApiKey")) continue;
    const rows: VizPrintedKeyRow[] = [];
    for (const row of groups[3]!.matchAll(/\\SemioTableRow\s*\{/g)) {
      const cells = vizPrintedCells(texGroup(groups[3]!, row.index! + row[0].length - 1).body);
      const names = [...new Set([...cells[0]!.matchAll(/\\Key\{([A-Za-z0-9-]+)\}/g)].map(key => key[1]!))];
      if (names.length === 0) continue;
      const meaning = lastApiTextArguments(cells[3] ?? "");
      const types = vizPrintedCells(cells[1] ?? "", ","), defaults = vizPrintedCells(cells[2] ?? "", ",");
      for (const [index, name] of names.entries()) rows.push({ names: [name], type: vizPrintedScalar(types.length === names.length ? types[index]! : cells[1] ?? ""), default: vizPrintedScalar(defaults.length === names.length ? defaults[index]! : cells[2] ?? "", true), defaultPresent: Boolean((cells[2] ?? "").trim()), meaning: { en: meaning?.[0] ?? "", de: meaning?.[1] ?? "" } });
    }
    tables[owner[1]!] = [...(tables[owner[1]!] ?? []), ...rows];
  }
  return tables;
}

/** 🔑️ Only first-column control names can satisfy a printed owner-table requirement. */
export function vizPrintedKeyTables(source: string): Readonly<Record<string, readonly string[]>> {
  return Object.fromEntries(Object.entries(vizPrintedKeyRows(source)).map(([owner, rows]) => [owner, [...new Set(rows.flatMap(row => [...row.names]))].sort()]));
}
//#endregion 🔖️Coverage
