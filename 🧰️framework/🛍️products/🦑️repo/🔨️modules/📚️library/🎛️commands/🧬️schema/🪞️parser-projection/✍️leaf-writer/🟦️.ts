/** ✍️ Applies the parsers one projection run settled for a leaf to the files of its module tree: appends them to the leaf or to the parser folder of their component, restates them in the file that defines them, moves fresh ones out of the leaf, supplies the imports every file needs (a parser is imported from the file that holds it, never through the leaf) and drops the imports a move left unused.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts
 */
import { existsSync, mkdirSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { codePointOrder, escapeRegExp, jsonString } from "../🔡️json-text/🟦️.ts";
import { existingHelper, importsParser, parserRange } from "../📖️declared-types/🟦️.ts";
import { PARSER_REGION, PARSER_REGION_END, helpersBlock, refreshHelpers, withClosedHelper } from "../🚪️guard-helpers/🟦️.ts";
import { ParserFiles, appendParsers, identifiersOf, importStatements, ownsParser, parserFileText, pruneImports, specifierBetween, symbolTable, withoutModuleLines, type Binding, type ParserHomes } from "../📍️parser-files/🟦️.ts";
import { parserKeyParts, type ParserKey } from "../🧾️parser-render/🟦️.ts";
import { portable, type SourceStore } from "../🗂️documents/🟦️.ts";

//#region 🔖️Source

/** 📍️ Retargets named parser imports to the real definition moved within the schema module. */
function retargetParsers(file: string, source: string, moved: ReadonlyMap<string, { readonly from: string; readonly to: string }>): string {
  const additions = new Map<string, string[]>();
  const next = source.replace(/^import\s+(type\s+)?\{([^}]*)\}\s*from\s*"([^"\n]+)";?[ \t]*\n?/gmu, (whole, type: string | undefined, list: string, spec: string) => {
    if (!spec.startsWith(".")) return whole;
    const kept: string[] = [];
    for (const item of list.split(",").map((value: string) => value.trim()).filter(Boolean)) {
      const name = /^(?:type\s+)?(parse[A-Z][A-Za-z0-9]*)(?:\s+as\s+[\w$]+)?$/u.exec(item)?.[1], target = name === undefined ? undefined : moved.get(name);
      if (target === undefined || resolve(dirname(file), spec) !== target.from) { kept.push(item); continue; }
      if (file === target.to) continue;
      const key = (type ?? "") + "\u0000" + specifierBetween(file, target.to);
      additions.set(key, [...(additions.get(key) ?? []), item]);
    }
    return kept.length === list.split(",").filter(value => value.trim() !== "").length ? whole : kept.length === 0 ? "" : "import " + (type ?? "") + "{ " + kept.join(", ") + " } from \"" + spec + "\";\n";
  });
  const lines = [...additions].map(([key, names]) => { const [type, spec] = key.split("\u0000"); return "import " + type + "{ " + names.join(", ") + " } from \"" + spec + "\";"; }).join("\n");
  return withImports(next, lines);
}


/** 🔗️ The import statements of the parsers that sibling documents declare, one per file that holds them (a framework leaf by its package specifier). */
export function importLines(target: string, wanted: ReadonlyMap<string, ReadonlyMap<string,string>>, specifiers: ReadonlyMap<string, string>, homes: ParserHomes, current=""): string {
  const byFile = new Map<string, Map<string,string>>();
  for(const[owner,names]of wanted)for(const[name,binding]of names){
    const file=homes.home(owner,name),spec=specifiers.get(file)??specifierBetween(target,file);
    const item=binding===`parse${name}`?binding:`parse${name} as ${binding}`;
    if([...current.matchAll(/^import\s*\{([^}]*)\}\s*from\s*["']([^"'\n]+)["'];?/gmu)].some(match=>match[2]===spec&&match[1]!.split(",").some(part=>part.trim()===item)))continue;
    if(!byFile.has(file))byFile.set(file,new Map());byFile.get(file)!.set(name,binding);
  }
  return [...byFile.keys()].sort(codePointOrder).map(file=>`import { ${[...byFile.get(file)!].sort(([left],[right])=>codePointOrder(left,right)).map(([name,binding])=>binding===`parse${name}`?binding:`parse${name} as ${binding}`).join(", ")} } from ${jsonString(specifiers.get(file)??specifierBetween(target,file),false)};`).join("\n");
}

/** 🪪️ Retains an existing foreign import under the private binding its emitted calls use. */
function bindForeignImports(target:string,source:string,wanted:ReadonlyMap<string,ReadonlyMap<string,string>>,specifiers:ReadonlyMap<string,string>,homes:ParserHomes):string{
 const bindings=new Map<string,Map<string,string>>();for(const[owner,names]of wanted)for(const[name,binding]of names){const home=homes.home(owner,name),spec=specifiers.get(home)??specifierBetween(target,home);if(!bindings.has(spec))bindings.set(spec,new Map());bindings.get(spec)!.set(name,binding);}
 return source.replace(/^import\s*\{([^}]*)\}\s*from\s*(["'])([^"'\n]+)\2;?/gmu,(whole,list:string,_quote:string,spec:string)=>{
  const names=bindings.get(spec);if(names===undefined)return whole;
  const items=list.split(",").map(item=>{const name=/^\s*parse([A-Z][A-Za-z0-9]*)\s*$/u.exec(item)?.[1],binding=name===undefined?undefined:names.get(name);return binding===undefined?item.trim():binding===`parse${name}`?binding:`parse${name} as ${binding}`;});
  return `import { ${items.join(", ")} } from ${jsonString(spec,false)};`;
 });
}

/** 📥️ `source` with `lines` after its leading doc comment (or in front of it when it has none). */
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

/** 🔤️ `source` with the projected parser `name` replaced by `text`, in the place it already has. */
export function restateParser(source: string, name: string, text: string, path: string): string {
  const range = parserRange(source, name);
  if (range === undefined) throw new Error(`parse${name} is not a projected function in ${path}`);
  return dropDuplicateType(source.slice(0, range[0]) + text + source.slice(range[1]), name);
}
//#endregion 🔖️Source

//#region 🔖️Writer
const HELPER_NAMES = ["Refusal", "Object", "Closed", "ExactObject", "Array", "String", "Boolean", "Number", "Integer", "Member", "Constant"];

/** 📝️ The edits of one leaf: collected while the run settles, written by `finish`. A report run (`apply` false) only learns which files it would touch. */
export class LeafWriter {
  private readonly files: ParserFiles;
  private readonly working = new Map<string, string>();
  private readonly parsers = new Map<string, string[]>();
  private readonly leafBlocks: string[] = [];
  private readonly foreign = new Map<string, Map<string, Map<string,string>>>();
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
    private readonly homes: ParserHomes,
  ) {
    this.files = homes.files(leaf);
  }

  private text(file: string): string {
    return this.working.get(file) ?? this.sources.read(file);
  }

  private touch(file: string): void {
    this.touched.add(file);
  }

  private note(home: string, keys: ReadonlyMap<ParserKey,string>): void {
    for (const [key,binding] of keys) {
      const [owner, callee] = parserKeyParts(key);
      const text = this.text(home);
      if (owner === this.leaf || binding===`parse${callee}`&&(ownsParser(text, callee) || importsParser(text, callee))) continue;
      if (!this.foreign.has(home)) this.foreign.set(home, new Map());
      const byOwner = this.foreign.get(home)!;
      if (!byOwner.has(owner)) byOwner.set(owner, new Map());
      byOwner.get(owner)!.set(callee,binding);
    }
  }

  /** ➕️ Adds a type declaration block to the leaf. */
  addType(block: string): void {
    this.touch(this.leaf);
    this.leafBlocks.push(block);
  }

  /** 📍️ Adds a new parser to the place its component gives it. */
  add(name: string, text: string, foreign: ReadonlyMap<ParserKey,string>, closed: boolean): void {
    const home = this.files.placementOf(name);
    this.placed.set(name, home);
    this.touch(home);
    this.closed ||= closed;
    if (home === this.leaf) this.leafBlocks.push(text);
    else this.parsers.set(home, [...(this.parsers.get(home) ?? []), text]);
    this.note(home, foreign);
  }

  /** 📄️ Replaces a parser in the file that defines it (the leaf when only a re-export names it). */
  restate(name: string, text: string, foreign: ReadonlyMap<ParserKey,string>, closed: boolean): void {
    const file = this.files.definingFile(name) ?? this.leaf;
    this.placed.set(name, file);
    this.touch(file);
    this.closed ||= closed;
    if (this.apply) this.working.set(file, restateParser(this.text(file), name, text, portable(this.root, file)));
    this.note(file, foreign);
  }

  /** 🆕️ Moves the fresh parsers of the leaf that belong to a component folder into it, verbatim and in the order of the leaf. */
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


  /** ⬆️ Moves existing callees into the leaf when a new leaf caller would otherwise import its own helper consumer. */
  promote(names: readonly string[]): void {
    const moved = new Map<string, { from: string; to: string }>();
    for (const name of names) {
      const from = this.files.definingFile(name);
      if (from === undefined || from === this.leaf) continue;
      const current = this.text(from), range = parserRange(current, name);
      if (range === undefined) throw new Error("The current parser cannot be moved: " + name);
      const text = current.slice(range[0], range[1]);
      this.working.set(from, current.slice(0, range[0]) + current.slice(range[1]));
      this.leafBlocks.push(text);
      this.placed.set(name, this.leaf);
      this.touch(from);
      this.touch(this.leaf);
      moved.set("parse" + name, { from, to: this.leaf });
    }
    if (moved.size === 0) return;
    const table = new Map([...moved].map(([name, target]) => [name, { from: target.to, type: false } satisfies Binding]));
    for (const file of this.sources.tree(this.leaf)) {
      const current = this.text(file);
      let next = retargetParsers(file, current, moved);
      const referenced = identifiersOf(withoutModuleLines(next), false);
      next = withImports(next, importStatements(file, next, [...moved.keys()].filter(name => referenced.has(name)), table, new Set()).join("\n"));
      if (next !== current) { this.working.set(file, next); this.touch(file); }
    }
  }

  /** 🏁️ Writes every edited file (when applying) and returns the repository-relative paths that were or would be touched. */
  finish(): string[] {
    if (this.parsers.size > 0 && this.helpersMissing()) this.touch(this.leaf);
    if (this.apply && this.touched.size > 0) this.write();
    return [...this.touched].map((file) => portable(this.root, file));
  }

  private helpersMissing(): boolean {
    const text = this.text(this.leaf);
    return !text.includes(PARSER_REGION) || (this.closed && withClosedHelper(text, this.helper) !== text);
  }

  private write(): void {
    let leafText = this.text(this.leaf);
    if (leafText.includes(PARSER_REGION)) {
      if (existingHelper(leafText) !== this.helper) throw new Error(`the guard prefix of ${portable(this.root, this.leaf)} is not ${this.helper}`);
      const start = leafText.indexOf(PARSER_REGION), end = leafText.indexOf(PARSER_REGION_END, start);
      if (end < 0) throw new Error(`the helper block of ${portable(this.root, this.leaf)} is unclosed`);
      leafText = refreshHelpers(leafText, this.helper);
    } else if (this.leafBlocks.length > 0 || this.parsers.size > 0) this.leafBlocks.unshift(helpersBlock(this.helper));
    const table = symbolTable(this.sources, this.files.files);
    for (const [name, home] of this.placed) table.set(`parse${name}`, { from: home, type: false } satisfies Binding);
    for (const suffix of HELPER_NAMES) table.set(`${this.helper}${suffix}`, { from: this.leaf, type: false } satisfies Binding);
    const lines = (file: string, current: string, texts: readonly string[]): string => {
      const wanted = this.foreign.get(file) ?? new Map<string, Map<string,string>>();
      const provided = new Set([...wanted.values()].flatMap((names) => [...names.values()]));
      const own = importStatements(file, current, identifiersOf(texts.join("\n"), false), table, provided);
      const foreign = importLines(file, wanted, this.specifiers, this.homes,current);
      return [...own, ...(foreign === "" ? [] : [foreign])].join("\n");
    };
    const block = this.leafBlocks.join("\n").trimEnd();
    if (block !== "") leafText = `${leafText.trimEnd()}\n\n${block}\n`;
    leafText=bindForeignImports(this.leaf,leafText,this.foreign.get(this.leaf)??new Map(),this.specifiers,this.homes);
    leafText = withImports(leafText, lines(this.leaf, leafText, this.leafBlocks));
    leafText = pruneImports(leafText, identifiersOf(this.taken.join("\n"), true));
    for (const [file, texts] of this.parsers) {
      const current = this.sources.read(file);
      const joined = texts.join("\n").trimEnd();
      const next = bindForeignImports(file,existsSync(file) && current !== "" ? appendParsers(current, joined) : parserFileText(joined),this.foreign.get(file)??new Map(),this.specifiers,this.homes);
      this.working.set(file, withImports(next, lines(file, next, texts)));
    }
    for(const[file,wanted]of this.foreign){
      if(file===this.leaf||this.parsers.has(file))continue;
      const current=bindForeignImports(file,this.text(file),wanted,this.specifiers,this.homes);
      this.working.set(file,withImports(current,lines(file,current,[current])));
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
