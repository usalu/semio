/** ✍️ Applies the parsers one projection run settled for a leaf to the files of its module tree: appends them to the leaf or to the parser folder of their component, restates them in the file that defines them, moves fresh ones out of the leaf, supplies the imports every file needs, re-exports the parser folders from the leaf and drops the imports a move left unused.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts
 */
import { existsSync, mkdirSync } from "node:fs";
import { dirname, relative } from "node:path";
import { codePointOrder, escapeRegExp, jsonString } from "../🔡️json-text/🟦️.ts";
import { definesParser, existingHelper, importsParser, parserRange } from "../📖️declared-types/🟦️.ts";
import { PARSER_REGION, helpersBlock, withClosedHelper } from "../🚪️guard-helpers/🟦️.ts";
import { ParserFiles, appendParsers, identifiersOf, importStatements, ownedParserNames, ownsParser, parserFileText, pruneImports, specifierBetween, symbolTable, withReexport, withoutModuleLines, type Binding } from "../📍️parser-files/🟦️.ts";
import { parserKeyParts, type ParserKey } from "../🧾️parser-render/🟦️.ts";
import { portable, type SourceStore } from "../🗂️documents/🟦️.ts";

//#region 🔖️Source
/** 🔗️ The import statements of the parsers that sibling documents declare, one per owning leaf (a framework leaf by its package specifier). */
export const importLines = (target: string, wanted: ReadonlyMap<string, ReadonlySet<string>>, specifiers: ReadonlyMap<string, string>): string =>
  [...wanted.keys()]
    .sort(codePointOrder)
    .map((owner) => {
      const specifier = specifiers.get(owner) ?? specifierBetween(target, owner);
      return `import { ${[...wanted.get(owner)!].sort(codePointOrder).map((name) => `parse${name}`).join(", ")} } from ${jsonString(specifier, false)};`;
    })
    .join("\n");

/** 🔗️ `source` with `lines` after its leading doc comment (or in front of it when it has none). */
export function withImports(source: string, lines: string): string {
  if (lines === "") return source;
  if (source.startsWith("/**")) {
    const end = source.indexOf("*/");
    if (end !== -1) {
      const cut = source.indexOf("\n", end) + 1;
      return `${source.slice(0, cut)}${lines}\n${source.slice(cut)}`;
    }
  }
  return `${lines}\n${source}`;
}

function dropDuplicateType(source: string, name: string): string {
  const declarations = source.match(new RegExp(`^\\s*(?:export\\s+)?(?:interface|type)\\s+${escapeRegExp(name)}\\b`, "gm"));
  if (declarations === null || declarations.length <= 1) return source;
  const duplicate = new RegExp(`^export type ${escapeRegExp(name)}\\b[^\\n]*;\\n\\n?`, "m").exec(source);
  return duplicate === null ? source : source.slice(0, duplicate.index) + source.slice(duplicate.index + duplicate[0].length);
}

/** ✍️ `source` with the projected parser `name` replaced by `text`, in the place it already has. */
export function restateParser(source: string, name: string, text: string, path: string): string {
  const range = parserRange(source, name);
  if (range === undefined) throw new Error(`parse${name} is not a projected function in ${path}`);
  return dropDuplicateType(source.slice(0, range[0]) + text + source.slice(range[1]), name);
}
//#endregion 🔖️Source

//#region 🔖️Writer
const HELPER_NAMES = ["Refusal", "Object", "Closed", "Array", "String", "Boolean", "Number", "Integer", "Member", "Constant"];

/** ✍️ The edits of one leaf: collected while the run settles, written by `finish`. A report run (`apply` false) only learns which files it would touch. */
export class LeafWriter {
  private readonly files: ParserFiles;
  private readonly working = new Map<string, string>();
  private readonly parsers = new Map<string, string[]>();
  private readonly leafBlocks: string[] = [];
  private readonly foreign = new Map<string, Map<string, Set<string>>>();
  private readonly placed = new Map<string, string>();
  private readonly taken: string[] = [];
  private readonly touched = new Set<string>();
  private closed = false;

  constructor(
    private readonly sources: SourceStore,
    private readonly root: string,
    readonly leaf: string,
    private readonly helper: string,
    private readonly apply: boolean,
    private readonly specifiers: ReadonlyMap<string, string>,
  ) {
    this.files = new ParserFiles(sources, leaf);
  }

  private text(file: string): string {
    return this.working.get(file) ?? this.sources.read(file);
  }

  private touch(file: string): void {
    this.touched.add(file);
  }

  private note(home: string, keys: Iterable<ParserKey>): void {
    for (const key of keys) {
      const [owner, callee] = parserKeyParts(key);
      const text = this.text(home);
      if (owner === this.leaf || ownsParser(text, callee) || importsParser(text, callee)) continue;
      if (!this.foreign.has(home)) this.foreign.set(home, new Map());
      const byOwner = this.foreign.get(home)!;
      if (!byOwner.has(owner)) byOwner.set(owner, new Set());
      byOwner.get(owner)!.add(callee);
    }
  }

  /** 📍️ Adds a type declaration block to the leaf. */
  addType(block: string): void {
    this.touch(this.leaf);
    this.leafBlocks.push(block);
  }

  /** 📍️ Adds a new parser to the place its component gives it. */
  add(name: string, text: string, foreign: Iterable<ParserKey>, closed: boolean): void {
    const home = this.files.placementOf(name);
    this.placed.set(name, home);
    this.touch(home);
    if (home !== this.leaf) this.touch(this.leaf);
    this.closed ||= closed;
    if (home === this.leaf) this.leafBlocks.push(text);
    else this.parsers.set(home, [...(this.parsers.get(home) ?? []), text]);
    this.note(home, foreign);
  }

  /** 📍️ Replaces a parser in the file that defines it (the leaf when only a re-export names it). */
  restate(name: string, text: string, foreign: Iterable<ParserKey>, closed: boolean): void {
    const file = this.files.definingFile(name) ?? this.leaf;
    this.placed.set(name, file);
    this.touch(file);
    this.closed ||= closed;
    if (this.apply) this.working.set(file, restateParser(this.text(file), name, text, portable(this.root, file)));
    this.note(file, foreign);
  }

  /** 📍️ Moves the fresh parsers of the leaf that belong to a component folder into it, verbatim and in the order of the leaf. */
  distribute(names: Iterable<string>): void {
    for (const name of names) {
      const home = this.files.placementOf(name);
      const source = this.text(this.leaf);
      const range = parserRange(source, name);
      if (home === this.leaf || range === undefined) continue;
      this.placed.set(name, home);
      this.touch(this.leaf);
      this.touch(home);
      if (!this.apply) continue;
      const text = source.slice(range[0], range[1]);
      this.working.set(this.leaf, source.slice(0, range[0]) + source.slice(source[range[1]] === "\n" ? range[1] + 1 : range[1]));
      this.parsers.set(home, [...(this.parsers.get(home) ?? []), text]);
      this.taken.push(text);
    }
  }

  /** 📍️ Writes every edited file (when applying) and returns the repository-relative paths that were or would be touched. */
  finish(): string[] {
    if (this.apply && this.touched.size > 0) this.write();
    return [...this.touched].map((file) => portable(this.root, file));
  }

  private write(): void {
    let leafText = this.text(this.leaf);
    if (leafText.includes(PARSER_REGION)) {
      if (this.closed && existingHelper(leafText) !== this.helper) throw new Error(`the guard prefix of ${portable(this.root, this.leaf)} is not ${this.helper}`);
      if (this.closed) leafText = withClosedHelper(leafText, this.helper);
    } else if (this.leafBlocks.length > 0 || this.parsers.size > 0) this.leafBlocks.unshift(helpersBlock(this.helper));
    const table = symbolTable(this.sources, this.files.files);
    for (const [name, home] of this.placed) table.set(`parse${name}`, { from: home, type: false } satisfies Binding);
    for (const suffix of HELPER_NAMES) table.set(`${this.helper}${suffix}`, { from: this.leaf, type: false } satisfies Binding);
    const lines = (file: string, current: string, texts: readonly string[], extra: Iterable<string> = []): string => {
      const wanted = this.foreign.get(file) ?? new Map<string, Set<string>>();
      const provided = new Set([...wanted.values()].flatMap((names) => [...names].map((name) => `parse${name}`)));
      const own = importStatements(file, current, new Set([...identifiersOf(texts.join("\n"), false), ...extra]), table, provided);
      const foreign = importLines(file, wanted, this.specifiers);
      return [...own, ...(foreign === "" ? [] : [foreign])].join("\n");
    };
    const block = this.leafBlocks.join("\n").trimEnd();
    if (block !== "") leafText = `${leafText.trimEnd()}\n\n${block}\n`;
    const remaining = identifiersOf(withoutModuleLines(leafText), true);
    const outward = [...this.placed].filter(([name, home]) => home !== this.leaf && remaining.has(`parse${name}`)).map(([name]) => `parse${name}`);
    leafText = withImports(leafText, lines(this.leaf, this.text(this.leaf), this.leafBlocks, outward));
    leafText = pruneImports(leafText, identifiersOf(this.taken.join("\n"), true));
    const created: string[] = [];
    for (const [file, texts] of this.parsers) {
      const current = this.sources.read(file);
      const joined = texts.join("\n").trimEnd();
      const next = existsSync(file) && current !== "" ? appendParsers(current, joined) : parserFileText(joined);
      this.working.set(file, withImports(next, lines(file, current, texts)));
      created.push(file);
      leafText = withReexport(leafText, specifierBetween(this.leaf, file), ownedParserNames(this.text(file)).map((name) => `parse${name}`));
    }
    this.working.set(this.leaf, leafText);
    for (const file of this.touched) {
      const text = this.working.get(file);
      if (text === undefined) continue;
      if (!existsSync(file)) mkdirSync(dirname(file), { recursive: true });
      this.sources.write(file, text);
    }
  }
}
//#endregion 🔖️Writer
