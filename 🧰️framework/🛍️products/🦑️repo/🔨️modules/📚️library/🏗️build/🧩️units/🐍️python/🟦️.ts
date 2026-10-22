import {mkdirSync, readdirSync, readFileSync, rmSync, statSync} from "node:fs";
import {join} from "node:path";
import {sha256V1} from "../🔏️key/🟦️.ts";
import {compareCodePointsV1, condensationV1, sortedCodePointsV1, type UnitDependencyV1, type UnitGraphV1, type UnitsPolicyV1, type UnitV1} from "../🕸️graph/🟦️.ts";
import type {ExecuteOutcomeV1, ExecuteRequestV1} from "../🏃️engine/🟦️.ts";

/** 🔤️ One lexical token of Python source. */
export type PythonTokenV1 = {readonly kind: "name" | "number" | "string" | "op" | "error"; readonly text: string; readonly line: number};
/** 📏️ One logical line: nesting depth from the indentation stack and its tokens (no NEWLINE, comments dropped). */
export type PythonLogicalLineV1 = {readonly depth: number; readonly tokens: readonly PythonTokenV1[]; readonly line: number};

const OPERATORS = ["**=", "//=", ">>=", "<<=", "...", "->", ":=", "==", "!=", "<=", ">=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "@=", "**", "//", "<<", ">>", "<>"];
const SINGLE = new Set("+-*/%&|^~<>()[]{},:.;@=!".split(""));
const NUMBER = /(?:0[xX][0-9a-fA-F_]+|0[bB][01_]+|0[oO][0-7_]+|(?:\d[\d_]*(?:\.[\d_]*)?|\.\d[\d_]*)(?:[eE][+-]?\d[\d_]*)?[jJ]?)/y;
const NAME_START = /[\p{L}\p{Nl}_]/u, NAME_PART = /[\p{L}\p{Nl}\p{Mn}\p{Mc}\p{Nd}\p{Pc}_]/u;

/** 🔤️ Lexes Python source into logical lines. This is a tokenizer, not a parser: it reports only lexical errors (unterminated string, unbalanced brackets, inconsistent dedent); syntax belongs to the interpreter (`py_compile`). */
export function lexPythonV1(text: string): {readonly lines: readonly PythonLogicalLineV1[]; readonly error: string | null} {
  const lines: PythonLogicalLineV1[] = [], stack = [0], n = text.length;
  let error: string | null = null, i = 0, line = 1, depth = 0, current: PythonTokenV1[] = [], currentLine = 1, atStart = true;
  const fail = (message: string): void => {if (error === null) error = `line ${line}: ${message}`;};
  const finish = (): void => {if (current.length) lines.push({depth: stack.length - 1, tokens: current, line: currentLine}); current = []; atStart = true;};
  while (i < n) {
    if (atStart && depth === 0) {
      let column = 0, j = i;
      for (; j < n; j++) {const c = text[j]!; if (c === " ") column++; else if (c === "\t") column = (Math.floor(column / 8) + 1) * 8; else if (c === "\f") column = 0; else break;}
      const c = text[j];
      if (c === undefined) {i = j; break;}
      if (c === "\n" || c === "\r") {i = j + (c === "\r" && text[j + 1] === "\n" ? 2 : 1); line++; continue;}
      if (c === "#") {while (j < n && text[j] !== "\n" && text[j] !== "\r") j++; i = j; continue;}
      i = j; atStart = false; currentLine = line;
      if (column > stack[stack.length - 1]!) stack.push(column);
      else if (column < stack[stack.length - 1]!) {while (stack.length > 1 && column < stack[stack.length - 1]!) stack.pop(); if (stack[stack.length - 1] !== column) fail("unindent does not match any outer indentation level");}
    }
    const c = text[i]!;
    if (c === " " || c === "\t" || c === "\f") {i++; continue;}
    if (c === "#") {while (i < n && text[i] !== "\n" && text[i] !== "\r") i++; continue;}
    if (c === "\n" || c === "\r") {
      i += c === "\r" && text[i + 1] === "\n" ? 2 : 1; line++;
      if (depth === 0) finish();
      continue;
    }
    if (c === "\\") {
      if (text[i + 1] === "\n") {i += 2; line++; continue;}
      if (text[i + 1] === "\r") {i += text[i + 2] === "\n" ? 3 : 2; line++; continue;}
      current.push({kind: "error", text: c, line}); i++; fail("unexpected character after line continuation"); continue;
    }
    let prefix = 0;
    if (NAME_START.test(c)) {
      let j = i + 1;
      while (j < n && NAME_PART.test(text[j]!)) j++;
      const word = text.slice(i, j);
      if (/^(?:[rRbBuUfF]|[rR][bBfF]|[bBfF][rR])$/.test(word) && (text[j] === '"' || text[j] === "'")) prefix = j - i;
      else {current.push({kind: "name", text: word, line}); i = j; continue;}
    }
    if (c === '"' || c === "'" || prefix) {
      const start = i, quoteAt = i + prefix, quote = text[quoteAt]!, triple = text.startsWith(quote.repeat(3), quoteAt), startLine = line;
      let j = quoteAt + (triple ? 3 : 1), closed = false;
      while (j < n) {
        const d = text[j]!;
        if (d === "\\") {if (text[j + 1] === "\n") line++; j += 2; continue;}
        if (triple) {if (text.startsWith(quote.repeat(3), j)) {j += 3; closed = true; break;} if (d === "\n") line++; j++; continue;}
        if (d === quote) {j++; closed = true; break;}
        if (d === "\n" || d === "\r") break;
        j++;
      }
      if (!closed) {fail(triple ? "unterminated triple-quoted string literal" : "unterminated string literal");}
      current.push({kind: closed ? "string" : "error", text: text.slice(start, Math.min(j, n)), line: startLine}); i = Math.min(j, n); continue;
    }
    if ((c >= "0" && c <= "9") || (c === "." && text[i + 1] !== undefined && text[i + 1]! >= "0" && text[i + 1]! <= "9")) {
      NUMBER.lastIndex = i;
      const match = NUMBER.exec(text);
      if (match) {current.push({kind: "number", text: match[0], line}); i += match[0].length; continue;}
    }
    const operator = OPERATORS.find(candidate => text.startsWith(candidate, i));
    if (operator) {current.push({kind: "op", text: operator, line}); i += operator.length; continue;}
    if (SINGLE.has(c)) {
      if (c === "(" || c === "[" || c === "{") depth++;
      else if (c === ")" || c === "]" || c === "}") {depth--; if (depth < 0) {depth = 0; fail("unmatched bracket");}}
      current.push({kind: "op", text: c, line}); i++; continue;
    }
    current.push({kind: "error", text: c, line}); fail(`invalid character ${JSON.stringify(c)}`); i++;
  }
  if (depth > 0) fail("EOF in multi-line statement");
  finish();
  return {lines, error};
}

/** 📜️ One simple or header statement with its nesting depth (`;` siblings share it, an inline body is one deeper). */
type Statement = {readonly depth: number; readonly tokens: readonly PythonTokenV1[]; readonly line: number};
const COMPOUND = new Set(["if", "elif", "else", "try", "except", "finally", "with", "for", "while", "def", "class"]);
const bracketDelta = (token: PythonTokenV1): number => token.kind !== "op" ? 0 : "([{".includes(token.text) ? 1 : ")]}".includes(token.text) ? -1 : 0;
const headerColon = (tokens: readonly PythonTokenV1[]): number => {
  let depth = 0, lambdas = 0;
  for (let index = 0; index < tokens.length; index++) {
    const token = tokens[index]!;
    depth += bracketDelta(token);
    if (depth !== 0) continue;
    if (token.kind === "name" && token.text === "lambda") lambdas++;
    else if (token.kind === "op" && token.text === ":") {if (lambdas > 0) lambdas--; else return index;}
  }
  return -1;
};
const splitSemicolons = (tokens: readonly PythonTokenV1[]): PythonTokenV1[][] => {
  const parts: PythonTokenV1[][] = [[]];
  let depth = 0;
  for (const token of tokens) {depth += bracketDelta(token); if (depth === 0 && token.kind === "op" && token.text === ";") parts.push([]); else parts[parts.length - 1]!.push(token);}
  return parts.filter(part => part.length);
};
const isHeader = (tokens: readonly PythonTokenV1[]): boolean => {
  const first = tokens[0]!;
  if (first.kind !== "name") return false;
  if (first.text === "async") return tokens[1]?.kind === "name" && (tokens[1].text === "def" || tokens[1].text === "for" || tokens[1].text === "with");
  return COMPOUND.has(first.text);
};
const statementsOf = (lines: readonly PythonLogicalLineV1[]): Statement[] => {
  const out: Statement[] = [];
  for (const logical of lines) {
    if (isHeader(logical.tokens)) {
      const colon = headerColon(logical.tokens);
      if (colon < 0) {out.push({depth: logical.depth, tokens: logical.tokens, line: logical.line}); continue;}
      out.push({depth: logical.depth, tokens: logical.tokens.slice(0, colon), line: logical.line});
      for (const part of splitSemicolons(logical.tokens.slice(colon + 1))) out.push({depth: logical.depth + 1, tokens: part, line: logical.line});
    } else for (const part of splitSemicolons(logical.tokens)) out.push({depth: logical.depth, tokens: part, line: logical.line});
  }
  return out;
};

/** 📥️ One import statement: dotted module, relative level and imported names (`*` for a star import). */
export type PythonImportV1 = {readonly module: string; readonly level: number; readonly names: readonly string[]; readonly bound: readonly {readonly name: string; readonly redundant: boolean}[]; readonly line: number; readonly lazy: boolean; readonly guarded: boolean};
const parseImport = (tokens: readonly PythonTokenV1[], line: number, lazy: boolean, guarded: boolean): PythonImportV1 | undefined => {
  const first = tokens[0]!.text;
  const dotted = (from: number): {name: string; next: number} => {
    let name = "", index = from;
    while (index < tokens.length && tokens[index]!.kind === "name" && tokens[index]!.text !== "import" && tokens[index]!.text !== "as") {name += tokens[index]!.text; index++; if (tokens[index]?.text === "." && tokens[index + 1]?.kind === "name") {name += "."; index++;} else break;}
    return {name, next: index};
  };
  if (first === "import") {
    const names: string[] = [], bound: {name: string; redundant: boolean}[] = [];
    let index = 1;
    while (index < tokens.length) {
      const target = dotted(index);
      if (!target.name) return undefined;
      index = target.next;
      let alias: string | undefined;
      if (tokens[index]?.text === "as" && tokens[index + 1]?.kind === "name") {alias = tokens[index + 1]!.text; index += 2;}
      names.push(target.name);
      bound.push({name: alias ?? target.name.split(".")[0]!, redundant: alias !== undefined && alias === target.name});
      if (tokens[index]?.text === ",") index++; else break;
    }
    return {module: "", level: 0, names, bound, line, lazy, guarded};
  }
  if (first !== "from") return undefined;
  let index = 1, level = 0;
  while (tokens[index]?.kind === "op" && (tokens[index]!.text === "." || tokens[index]!.text === "...")) {level += tokens[index]!.text.length; index++;}
  const target = dotted(index);
  index = target.next;
  if (tokens[index]?.text !== "import") return undefined;
  index++;
  const names: string[] = [], bound: {name: string; redundant: boolean}[] = [];
  if (tokens[index]?.text === "*") return {module: target.name, level, names: ["*"], bound: [{name: "*", redundant: true}], line, lazy, guarded};
  if (tokens[index]?.text === "(") index++;
  while (index < tokens.length && tokens[index]!.kind === "name") {
    const name = tokens[index]!.text;
    index++;
    let alias: string | undefined;
    if (tokens[index]?.text === "as" && tokens[index + 1]?.kind === "name") {alias = tokens[index + 1]!.text; index += 2;}
    names.push(name); bound.push({name: alias ?? name, redundant: alias !== undefined && alias === name});
    if (tokens[index]?.text === ",") index++; else break;
  }
  return {module: target.name, level, names, bound, line, lazy, guarded};
};

const stringContent = (token: string): string => token.replace(/^[A-Za-z]*('''|"""|'|")/, "").replace(/('''|"""|'|")$/, "");
const isPublic = (name: string, exported: ReadonlySet<string>): boolean => exported.has(name) || !name.startsWith("_") || (name.startsWith("__") && name.endsWith("__") && name.length > 4);
const render = (tokens: readonly PythonTokenV1[]): string => tokens.map(token => token.text).join(" ");
/** ✂️ A def or class header without the trailing comma of its outermost parenthesis (`def f(a,\n):` equals `def f(a):`; commas inside nested tuples stay). */
const renderHeader = (tokens: readonly PythonTokenV1[]): string => {
  let depth = 0;
  const kept: PythonTokenV1[] = [];
  for (let index = 0; index < tokens.length; index++) {
    const token = tokens[index]!;
    if (token.kind === "op" && token.text === "," && depth === 1 && tokens[index + 1]?.text === ")" ) continue;
    depth += bracketDelta(token);
    kept.push(token);
  }
  return render(kept);
};

const KEYWORDS = new Set(["None", "True", "False", "in", "for", "async", "await", "not", "and", "or", "is", "lambda", "if", "else", "as", "with", "yield"]);
const topLevelEquals = (tokens: readonly PythonTokenV1[]): number[] => {
  const found: number[] = [];
  let depth = 0;
  for (let index = 0; index < tokens.length; index++) {depth += bracketDelta(tokens[index]!); if (depth === 0 && tokens[index]!.kind === "op" && tokens[index]!.text === "=") found.push(index);}
  return found;
};
/** 🎯️ The target segments of an assignment statement (every top-level `=` ends one; a `lambda` starts the value), or undefined when it is not one. */
const splitAssignment = (tokens: readonly PythonTokenV1[]): PythonTokenV1[][] | undefined => {
  const cuts = topLevelEquals(tokens), segments: PythonTokenV1[][] = [];
  let from = 0;
  for (const cut of cuts) {
    const segment = tokens.slice(from, cut);
    if (!segment.length || segment.some(token => token.kind === "name" && token.text === "lambda")) break;
    segments.push(segment); from = cut + 1;
  }
  return segments.length ? segments : undefined;
};
/** 🎯️ The names an assignment, `for` or `with ... as` target binds: plain names and names inside groups and star targets; not the base of an attribute, call or subscript and nothing inside those. */
const targetNames = (tokens: readonly PythonTokenV1[]): string[] => {
  const names: string[] = [], access: boolean[] = [];
  for (let index = 0; index < tokens.length; index++) {
    const token = tokens[index]!, previous = tokens[index - 1], next = tokens[index + 1];
    if (token.kind === "op" && (token.text === "(" || token.text === "[" || token.text === "{")) {access.push(previous !== undefined && (previous.kind === "name" || previous.text === ")" || previous.text === "]")); continue;}
    if (token.kind === "op" && (token.text === ")" || token.text === "]" || token.text === "}")) {access.pop(); continue;}
    if (token.kind !== "name" || access.some(Boolean) || previous?.text === "." || (next && (next.text === "." || next.text === "(" || next.text === "["))) continue;
    if (!KEYWORDS.has(token.text)) names.push(token.text);
  }
  return names;
};

/** 🧾️ Everything the scanner knows about one Python file. `api` is the interface text (see policy `python.api`), `names` the names bound at module level (private included), `dynamic` true when the names cannot be enumerated. */
export type PythonSourceFactsV1 = {readonly lexicalError: string | null; readonly imports: readonly PythonImportV1[]; readonly apiLines: readonly string[]; readonly api: string; readonly names: readonly string[]; readonly dynamic: boolean};

/** 🔎️ Analyses one Python file: imports anywhere, the structural API lines (function bodies skipped), module-level bound names. `init` marks a package `__init__`, whose imports are re-exports. */
export function analyzePythonSourceV1(text: string, options: {readonly init: boolean; readonly unannotatedReturns: "ignore" | "body"}): PythonSourceFactsV1 {
  const lexed = lexPythonV1(text), statements = statementsOf(lexed.lines), imports: PythonImportV1[] = [], apiLines: string[] = [], bound = new Set<string>();
  const exported = new Set<string>();
  let dynamic = false;
  for (const statement of statements) {
    const [first, second] = statement.tokens;
    if (statement.depth === 0 && first?.text === "__all__" && (second?.text === "=" || second?.text === "+=" || second?.text === ":")) for (const token of statement.tokens) if (token.kind === "string") exported.add(stringContent(token.text));
    if (statement.depth === 0 && first?.text === "__all__" && second?.text === ".") for (const token of statement.tokens) if (token.kind === "string") exported.add(stringContent(token.text));
  }
  type Context = {readonly depth: number; readonly kind: "def" | "class" | "private" | "compound"};
  const stack: Context[] = [];
  let decorators: string[] = [];
  const dynamicNames = new Set(["globals", "vars", "setattr", "exec", "__dict__", "__getattr__", "locals"]);
  for (let index = 0; index < statements.length; index++) {
    const statement = statements[index]!, tokens = statement.tokens;
    while (stack.length && statement.depth <= stack[stack.length - 1]!.depth) stack.pop();
    const skipped = stack.some(context => context.kind === "def" || context.kind === "private");
    const moduleLevel = !stack.some(context => context.kind === "def" || context.kind === "private" || context.kind === "class");
    const first = tokens[0]!;
    for (const token of tokens) if (token.kind === "name" && dynamicNames.has(token.text)) dynamic = true;
    if (first.kind === "name" && (first.text === "import" || first.text === "from")) {
      const parsed = parseImport(tokens, statement.line, skipped || !moduleLevel, stack.some(context => context.kind === "compound"));
      if (parsed) {
        imports.push(parsed);
        if (!skipped) {
          if (moduleLevel) for (const row of parsed.bound) if (row.name === "*") dynamic = true; else bound.add(row.name);
          if (parsed.bound.some(row => row.redundant || row.name === "*" || exported.has(row.name) || options.init)) apiLines.push(`${stack.length}|${render(tokens)}`);
        }
      }
      continue;
    }
    if (first.kind === "name" && first.text === "global" && skipped) {for (const token of tokens.slice(1)) if (token.kind === "name") bound.add(token.text); continue;}
    if (skipped) continue;
    if (first.kind === "op" && first.text === "@") {decorators.push(render(tokens)); continue;}
    const async = first.text === "async", head = async ? tokens[1]! : first;
    if (head.kind === "name" && head.text === "def") {
      const name = tokens[async ? 2 : 1]?.text ?? "", open = decorators;
      decorators = [];
      if (moduleLevel) bound.add(name);
      if (isPublic(name, exported)) {
        for (const decorator of open) apiLines.push(`${stack.length}|${decorator}`);
        let line = `${stack.length}|${renderHeader(tokens)}`;
        if (options.unannotatedReturns === "body" && !tokens.some(token => token.text === "->")) {
          const body: string[] = [];
          for (let next = index + 1; next < statements.length && statements[next]!.depth > statement.depth; next++) body.push(`${statements[next]!.depth}:${render(statements[next]!.tokens)}`);
          line += " body:" + sha256V1(body.join("\n"));
        }
        apiLines.push(line);
      }
      stack.push({depth: statement.depth, kind: "def"});
      continue;
    }
    if (head.kind === "name" && head.text === "class") {
      const name = tokens[1]?.text ?? "", open = decorators;
      decorators = [];
      if (moduleLevel) bound.add(name);
      if (isPublic(name, exported)) {for (const decorator of open) apiLines.push(`${stack.length}|${decorator}`); apiLines.push(`${stack.length}|${renderHeader(tokens)}`); stack.push({depth: statement.depth, kind: "class"});}
      else stack.push({depth: statement.depth, kind: "private"});
      continue;
    }
    decorators = [];
    if (moduleLevel) for (let at = 0; at + 1 < tokens.length; at++) if (tokens[at]!.kind === "name" && tokens[at + 1]!.text === ":=") bound.add(tokens[at]!.text);
    if (head.kind === "name" && COMPOUND.has(head.text)) {
      apiLines.push(`${stack.length}|${render(tokens)}`);
      if (moduleLevel && head.text === "for") {const inAt = tokens.findIndex(token => token.text === "in"); for (const name of targetNames(tokens.slice(1, inAt < 0 ? 1 : inAt))) bound.add(name);}
      if (moduleLevel && head.text === "with") for (let at = 0; at < tokens.length; at++) if (tokens[at]!.text === "as") {
        let end = at + 1, depth = 0;
        for (; end < tokens.length; end++) {depth += bracketDelta(tokens[end]!); if (depth < 0 || (depth === 0 && tokens[end]!.text === ",")) break;}
        for (const name of targetNames(tokens.slice(at + 1, end))) bound.add(name);
      }
      stack.push({depth: statement.depth, kind: "compound"});
      continue;
    }
    const inClass = stack.length > 0 && stack[stack.length - 1]!.kind === "class";
    if (first.kind === "name" && tokens[1]?.text === ":") {
      const name = first.text, rest = tokens.slice(2), equals = topLevelEquals(rest).at(0) ?? -1;
      const annotation = equals < 0 ? rest : rest.slice(0, equals), value = equals < 0 ? [] : rest.slice(equals + 1);
      if (moduleLevel) bound.add(name);
      if (isPublic(name, exported)) apiLines.push(`${stack.length}|${name} : ${render(annotation)}${value.length && (inClass || annotation.some(token => token.text === "Final")) ? " = " + render(value) : ""}`);
      continue;
    }
    const assignment = splitAssignment(tokens);
    if (assignment) {
      const targets = assignment.flatMap(targetNames);
      if (moduleLevel) for (const name of targets) bound.add(name);
      if (targets.some(name => isPublic(name, exported))) apiLines.push(`${stack.length}|${render(tokens)}`);
      continue;
    }
    if (first.kind === "name" && first.text === "__all__") apiLines.push(`${stack.length}|${render(tokens)}`);
  }
  const names = sortedCodePointsV1(bound);
  const lines = ["polyglot-python-api-v1", ...apiLines, `names|${dynamic ? "*" : names.join(",")}`];
  return {lexicalError: lexed.error, imports, apiLines, api: lines.join("\n"), names, dynamic};
}
/** 🔏️ The interface digest of a module text (or of its `.pyi` stub). */
export const pythonInterfaceDigestV1 = (text: string, options: {readonly init: boolean; readonly unannotatedReturns: "ignore" | "body"}): string => sha256V1(analyzePythonSourceV1(text, options).api);

/** 🐍️ One importable module of the workspace. */
export type PythonModuleV1 = {readonly name: string; readonly file: string; readonly stub: string | null; readonly unit: string; readonly package: boolean};
export type PythonWorkspaceSnapshotV1 = {readonly files: readonly string[]; readonly read: (path: string) => string};
const isDotted = (name: string): boolean => name.split(".").every(part => /^[\p{L}\p{Nl}_][\p{L}\p{Nl}\p{Mn}\p{Mc}\p{Nd}\p{Pc}_]*$/u.test(part));

/** 🐍️ Plans the unit graph of Python sources: one unit per module (a sibling `.pyi` is its interface), `interface` edges to the workspace modules it imports (parent packages included), import cycles condensed into one unit, third-party imports pinned through `pins`. */
export function planPythonUnitsV1(snapshot: PythonWorkspaceSnapshotV1, options: {readonly policy: UnitsPolicyV1; readonly roots: readonly string[]; readonly toolchain: string; readonly pins: Readonly<Record<string, string>>}): {readonly graph: UnitGraphV1; readonly modules: readonly PythonModuleV1[]; readonly lexicalErrors: Readonly<Record<string, string>>} {
  const {policy} = options, roots = [...options.roots].sort((a, b) => b.length - a.length), fileSet = new Set(snapshot.files);
  const stdlib = new Set(policy.python.stdlib);
  const candidates: {file: string; name: string; id: string; package: boolean}[] = [];
  const claimed = new Set<string>();
  for (const file of sortedCodePointsV1(snapshot.files.filter(path => path.endsWith(".py")))) {
    if (file.split("/").some(part => policy.python.skipDirectories.includes(part))) continue;
    const root = roots.find(candidate => candidate === "" || file.startsWith(candidate + "/"));
    if (root === undefined) continue;
    const relative = root === "" ? file : file.slice(root.length + 1), stem = relative.slice(0, -3), package_ = stem === "__init__" || stem.endsWith("/__init__");
    const name = (package_ ? stem.replace(/\/?__init__$/, "") : stem).split("/").join("."), importable = name !== "" && isDotted(name) && !claimed.has(name);
    if (importable) claimed.add(name);
    candidates.push({file, name: importable ? name : "", id: importable ? "python:" + name : "python:file:" + file, package: package_});
  }
  const modules: PythonModuleV1[] = candidates.filter(row => row.name).map(row => ({name: row.name, file: row.file, stub: fileSet.has(row.file + "i") ? row.file + "i" : null, unit: row.id, package: row.package}));
  const byName = new Map(modules.map(row => [row.name, row] as const));
  const facts = new Map<string, {imports: readonly PythonImportV1[]; error: string | null}>(), lexicalErrors: Record<string, string> = {};
  for (const row of candidates) {
    const scanned = analyzePythonSourceV1(snapshot.read(row.file), {init: row.package, unannotatedReturns: policy.python.unannotatedReturns});
    facts.set(row.file, {imports: scanned.imports, error: scanned.lexicalError});
    if (scanned.lexicalError) lexicalErrors[row.file] = scanned.lexicalError;
  }
  const resolve = (module: string, level: number, base: PythonModuleV1 | undefined, row: {file: string; package: boolean}): string | undefined => {
    if (level === 0) return module;
    const here = base ? base.name : "", parts = here === "" ? [] : here.split(".");
    const packageParts = row.package ? parts : parts.slice(0, -1);
    if (level - 1 > packageParts.length) return undefined;
    const anchor = packageParts.slice(0, packageParts.length - (level - 1));
    return [...anchor, ...(module ? [module] : [])].join(".");
  };
  const tops = new Set(modules.map(row => row.name.split(".")[0]!)), unitDeps = new Map<string, Set<string>>(), unitExternals = new Map<string, Set<string>>();
  for (const row of candidates) {
    const deps = new Set<string>(), externals = new Set<string>(), self = byName.get(row.name);
    for (const item of facts.get(row.file)!.imports) {
      const targets: string[] = [];
      if (item.module === "" && item.level === 0) targets.push(...item.names);
      else {
        const base = resolve(item.module, item.level, self, row);
        if (base === undefined) continue;
        if (base !== "") targets.push(base);
        for (const name of item.names) if (name !== "*") targets.push(base === "" ? name : base + "." + name);
      }
      for (const target of targets) {
        if (target === "") continue;
        const module = byName.get(target);
        if (module) {if (module.unit !== row.id) deps.add(module.unit); continue;}
        const top = target.split(".")[0]!;
        if (item.level === 0 && !stdlib.has(top) && !tops.has(top)) externals.add(`python-import:${top}@${options.pins[top] ?? "unpinned"}`);
      }
    }
    unitDeps.set(row.id, deps); unitExternals.set(row.id, externals);
  }
  const ids = candidates.map(row => row.id), adjacent = new Map<string, Set<string>>(ids.map(id => [id, unitDeps.get(id)!]));
  const {components, componentOf} = condensationV1(ids, adjacent);
  const rowOf = new Map(candidates.map(row => [row.id, row] as const)), unitIdOf = new Map<string, string>();
  for (const component of components) unitIdOf.set(component[0]!, component.length === 1 ? component[0]! : "python:cycle:" + component[0]!.slice("python:".length));
  const units: UnitV1[] = [];
  const env = {PYTHONPYCACHEPREFIX: "{out}/pyc", SOURCE_DATE_EPOCH: "0"};
  for (const component of components) {
    const members = component.map(id => rowOf.get(id)!), single = members.length === 1, id = unitIdOf.get(component[0]!)!;
    const sources = sortedCodePointsV1(members.flatMap(row => [row.file, ...(fileSet.has(row.file + "i") ? [row.file + "i"] : [])]));
    const dependencies = new Map<string, UnitDependencyV1>(), externals = new Set<string>();
    for (const member of component) {
      for (const dependency of unitDeps.get(member)!) {const target = unitIdOf.get(componentOf.get(dependency)!)!; if (target !== id) dependencies.set(target, {unit: target, use: "interface"});}
      for (const external of unitExternals.get(member)!) externals.add(external);
    }
    units.push({
      id, language: "python", kind: "library", label: single ? members[0]!.file : `import cycle of ${members.length} modules`, sources, inputs: [],
      dependencies: [...dependencies.values()].sort((a, b) => compareCodePointsV1(a.unit, b.unit)), externals: sortedCodePointsV1(externals), interface: {kind: "python-api"}, outputs: ["pyc"],
      toolchain: {id: options.toolchain, parts: [`unannotatedReturns=${policy.python.unannotatedReturns}`]},
      action: {argv: ["python3", "-m", "py_compile", ...sources.filter(path => path.endsWith(".py"))], env: {...env, SEMIO_PYTHON_MODULES: JSON.stringify(Object.fromEntries(members.filter(row => row.name).sort((a, b) => compareCodePointsV1(a.file, b.file)).map(row => [row.file, {name: row.name, package: row.package}])))}},
    });
  }
  return {graph: {version: 1, units}, modules: modules.map(row => ({...row, unit: unitIdOf.get(componentOf.get(row.unit)!)!})), lexicalErrors};
}

/** 🏷️ The facts of one finished unit the import check needs. */
type UnitNames = {readonly names: ReadonlySet<string>; readonly dynamic: boolean};
/** 🔎️ Import-name check: `from m import n` where `m` is a workspace module (outside `try`/`if` guards, which exist to tolerate a missing name) must find `n` among the names `m` binds at module level, a submodule `m.n`, or a dynamic module. It reports only what cannot exist. */
export function checkPythonImportsV1(items: readonly {readonly file: string; readonly module: PythonModuleV1 | undefined; readonly package: boolean; readonly imports: readonly PythonImportV1[]}[], modules: ReadonlyMap<string, PythonModuleV1>, namesOf: (module: PythonModuleV1) => UnitNames): string[] {
  const diagnostics: string[] = [];
  for (const row of items) for (const item of row.imports) {
    if (item.names.length === 0 || item.names[0] === "*" || item.guarded) continue;
    let base: string | undefined;
    if (item.level === 0) base = item.module;
    else {
      const parts = row.module ? row.module.name.split(".") : [], packageParts = row.package ? parts : parts.slice(0, -1);
      if (item.level - 1 > packageParts.length) continue;
      base = [...packageParts.slice(0, packageParts.length - (item.level - 1)), ...(item.module ? [item.module] : [])].join(".");
    }
    if (base === undefined || base === "") continue;
    const target = modules.get(base);
    if (!target) continue;
    const known = namesOf(target);
    if (known.dynamic) continue;
    for (const name of item.names) if (!known.names.has(name) && !modules.has(base + "." + name)) diagnostics.push(`${row.file}:${item.line}: cannot import name '${name}' from '${base}'`);
  }
  return diagnostics;
}

/** 🏭️ The process effect of the Python executor. */
export type PythonRunnerV1 = (argv: readonly string[], options: {readonly cwd: string; readonly env: Readonly<Record<string, string>>; readonly signal: AbortSignal}) => Promise<{readonly code: number; readonly output: string}>;
const pycFiles = (root: string): string[] => {
  const found: string[] = [];
  const walk = (directory: string): void => {for (const entry of readdirSync(directory, {withFileTypes: true})) {const path = join(directory, entry.name); if (entry.isDirectory()) walk(path); else if (entry.name.endsWith(".pyc")) found.push(path);}};
  try {if (statSync(root).isDirectory()) walk(root);} catch {return [];}
  return found;
};
/** 🏭️ Executes a Python unit: `python -m py_compile` into a private pycache prefix (checked-hash pycs through `SOURCE_DATE_EPOCH`, so output bytes depend on content only), the import-name check against the interfaces of the workspace modules it imports, and the interface digest (the `.pyi` stub when present, else the extracted API; a cycle unit folds its members). The names of imported modules come from the dependencies' interface artifacts when the request carries them (hermetic builds see no dependency sources), else from the dependency sources through `modules` and `read`. The interface artifact of the unit is `{"modules": {name: {names, dynamic}}}`. */
export function pythonUnitExecutorV1(options: {readonly workspace: string; readonly outputRoot: string; readonly policy: UnitsPolicyV1; readonly modules: readonly PythonModuleV1[]; readonly read: (path: string) => string; readonly run: PythonRunnerV1}): (request: ExecuteRequestV1) => Promise<ExecuteOutcomeV1> {
  const known = new Map(options.modules.map(row => [row.name, row] as const)), knownByFile = new Map(options.modules.map(row => [row.file, row] as const));
  const analyzed = new Map<string, PythonSourceFactsV1>();
  const analyze = (module: {file: string; stub: string | null; package: boolean}): PythonSourceFactsV1 => {
    const path = module.stub ?? module.file, text = options.read(path), key = path + "\0" + sha256V1(text);
    let facts = analyzed.get(key);
    if (!facts) {facts = analyzePythonSourceV1(text, {init: module.package, unannotatedReturns: options.policy.python.unannotatedReturns}); analyzed.set(key, facts);}
    return facts;
  };
  return async request => {
    const {unit} = request, out = join(options.outputRoot, sha256V1(unit.id).slice(0, 24));
    rmSync(out, {recursive: true, force: true}); mkdirSync(out, {recursive: true});
    const started = performance.now(), env = Object.fromEntries(Object.entries(unit.action.env).map(([key, value]) => [key, value.replace("{out}", out)]));
    const compiled = await options.run(unit.action.argv.map(part => part.replace("{out}", out)), {cwd: options.workspace, env, signal: request.signal});
    if (compiled.code !== 0) return {ok: false, diagnostics: compiled.output.trim(), outputs: sha256V1("failed:" + compiled.output), interface: null, milliseconds: Math.round(performance.now() - started)};
    const own = unit.sources.filter(path => path.endsWith(".py"));
    const declared = JSON.parse(unit.action.env.SEMIO_PYTHON_MODULES ?? "{}") as Record<string, {name: string; package: boolean}>;
    const stubs = new Set(unit.sources.filter(path => path.endsWith(".pyi")));
    const members: PythonModuleV1[] = own.flatMap(file => {const row = knownByFile.get(file); if (row) return [row]; const found = declared[file]; return found ? [{name: found.name, file, stub: stubs.has(file + "i") ? file + "i" : null, unit: unit.id, package: found.package}] : [];});
    const artifacts = new Map<string, {names: ReadonlySet<string>; dynamic: boolean}>();
    for (const dependency of request.dependencies) if (dependency.artifact) for (const [name, row] of Object.entries((JSON.parse(dependency.artifact) as {modules: Record<string, {names: string[]; dynamic: boolean}>}).modules)) artifacts.set(name, {names: new Set(row.names), dynamic: row.dynamic});
    const byName = new Map(known);
    for (const name of artifacts.keys()) if (!byName.has(name)) byName.set(name, {name, file: "", stub: null, unit: "", package: false});
    for (const row of members) byName.set(row.name, row);
    const ownFacts = new Map(members.map(row => [row.name, analyze(row)] as const));
    const diagnostics = checkPythonImportsV1(
      own.map(file => ({file, module: members.find(row => row.file === file), package: file.endsWith("/__init__.py") || file === "__init__.py", imports: analyzePythonSourceV1(options.read(file), {init: false, unannotatedReturns: "ignore"}).imports})),
      byName,
      module => {
        const fromArtifact = artifacts.get(module.name);
        if (fromArtifact) return fromArtifact;
        const fromOwn = ownFacts.get(module.name);
        if (fromOwn) return {names: new Set(fromOwn.names), dynamic: fromOwn.dynamic};
        if (module.file === "") return {names: new Set<string>(), dynamic: true};
        const facts = analyze(module);
        return {names: new Set(facts.names), dynamic: facts.dynamic};
      },
    );
    const pycs = pycFiles(out).map(path => sha256V1(readFileSync(path))).sort(compareCodePointsV1);
    const apiDigests = (members.length ? members.map(row => ownFacts.get(row.name)!.api) : own.map(file => analyzePythonSourceV1(options.read(file), {init: false, unannotatedReturns: options.policy.python.unannotatedReturns}).api)).map(sha256V1);
    const artifact = JSON.stringify({modules: Object.fromEntries(members.sort((a, b) => compareCodePointsV1(a.name, b.name)).map(row => [row.name, {names: [...ownFacts.get(row.name)!.dynamic ? [] : ownFacts.get(row.name)!.names], dynamic: ownFacts.get(row.name)!.dynamic}]))});
    return {ok: diagnostics.length === 0, diagnostics: diagnostics.join("\n"), outputs: sha256V1(pycs.join("\n")), interface: sha256V1(apiDigests.join("\n")), artifact, milliseconds: Math.round(performance.now() - started)};
  };
}
