import { type RustToken, rustFindTopLevel, rustTokenPairs, rustTokenSegments, rustTokenText, rustTokens } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";
import type { RustAssertionMessageSpan } from "../🦀️rust-fact-contracts/🟦️.ts";

/** 🧭️ Resolves assertion messages through Rust tokens and balanced macro arguments, never textual lookalikes. */
export function inspectRustAssertionMessageSpans(source: string): readonly RustAssertionMessageSpan[] {
  if (!/\b(?:debug_)?assert(?:_eq|_ne)?\s*!/u.test(source)) return [];
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), rows: RustAssertionMessageSpan[] = [];
  const argumentsByMacro: Readonly<Record<string, number>> = { assert: 1, assert_eq: 2, assert_ne: 2, debug_assert: 1, debug_assert_eq: 2, debug_assert_ne: 2 };
  if (tokens.some((token, index) => /^[()[\]{}]$/u.test(token.text) && !pairs.has(index))) return rows;
  for (let index = 0; index < tokens.length; index++) {
    const macroName = tokens[index]!.text, argumentIndex = argumentsByMacro[macroName];
    if (argumentIndex === undefined || tokens[index + 1]?.text !== "!" || tokens[index + 2]?.text !== "(") continue;
    const close = pairs.get(index + 2);
    if (close === undefined) continue;
    const argument = rustTokenSegments(tokens, pairs, index + 3, close, ",")[argumentIndex];
    if (!argument || argument[1] !== argument[0] + 1) continue;
    const token = tokens[argument[0]]!;
    if (token.kind !== "string" || !token.text.startsWith('"') || !token.text.endsWith('"') || token.text.includes("\\")) continue;
    rows.push({ macroName, start: token.start + 1, end: token.end - 1, value: token.text.slice(1, -1) });
  }
  return rows;
}

/** 🧭️ Recognizes every same-file zero-arg `fn NAME() -> PathBuf` whose ENTIRE body is exactly the
 * ancestor-walk-to-`nx.json` idiom seeded from `CARGO_MANIFEST_DIR` (`test_repo_root`/`find_repo_root`'s
 * shared strategy — 🏃️run/🦀️.rs, 📦️bin.rs): `let mut V = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
 * loop { if V.join("nx.json").is_file() { return V; } assert!(V.pop(), ..); }`. `nx.json` is this
 * repo's one canonical root marker — a single `nx.json` exists repo-wide, at the true root, never
 * nested under any crate — so any successful return from this loop is provably the repo root, the
 * same base a literal `CARGO_MANIFEST_DIR` proves, regardless of how many `.pop()` hops the walk
 * takes at runtime. Matched purely by shape (not by name), so it generalizes to every crate's own
 * copy of this idiom; a `SEMIO_REPO_ROOT`-env-var branch ahead of the loop (`find_repo_root`'s own
 * shape) is deliberately NOT matched — that path can resolve outside `CARGO_MANIFEST_DIR`'s ancestry
 * entirely, so proving it would need a genuinely different, unverified argument. */
function rustRepoRootAncestorWalkHelperNames(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>): ReadonlySet<string> {
  const names = new Set<string>();
  const prefixes = [["PathBuf", "::", "from", "("], ["std", "::", "path", "::", "PathBuf", "::", "from", "("], ["Path", "::", "new", "("], ["std", "::", "path", "::", "Path", "::", "new", "("]];
  for (let index = 0; index < tokens.length; index++) {
    if (tokens[index]?.text !== "fn") continue;
    const nameToken = tokens[index + 1];
    if (nameToken?.kind !== "identifier" || tokens[index + 2]?.text !== "(" || tokens[index + 3]?.text !== ")" || tokens[index + 4]?.text !== "->" || tokens[index + 5]?.text !== "PathBuf" || tokens[index + 6]?.text !== "{") continue;
    const bodyOpen = index + 6, bodyClose = pairs.get(bodyOpen);
    if (bodyClose === undefined) continue;
    let cursor = bodyOpen + 1;
    if (tokens[cursor]?.text !== "let") continue;
    cursor += 1;
    if (tokens[cursor]?.text === "mut") cursor += 1;
    const varToken = tokens[cursor];
    if (varToken?.kind !== "identifier") continue;
    const variable = varToken.text;
    cursor += 1;
    if (tokens[cursor]?.text !== "=") continue;
    cursor += 1;
    const prefix = prefixes.find((candidate) => candidate.every((text, offset) => tokens[cursor + offset]?.text === text));
    if (!prefix) continue;
    const openParen = cursor + prefix.length - 1, closeParen = pairs.get(openParen), environment = ["env", "!", "(", '"CARGO_MANIFEST_DIR"', ")"];
    if (closeParen === undefined || closeParen !== openParen + environment.length + 1 || !environment.every((text, offset) => tokens[openParen + offset + 1]?.text === text)) continue;
    cursor = closeParen + 1;
    if (tokens[cursor]?.text !== ";" || tokens[cursor + 1]?.text !== "loop" || tokens[cursor + 2]?.text !== "{") continue;
    const loopOpen = cursor + 2, loopClose = pairs.get(loopOpen);
    if (loopClose === undefined || loopClose + 1 !== bodyClose) continue;
    let inner = loopOpen + 1;
    if (tokens[inner]?.text !== "if" || tokens[inner + 1]?.text !== variable || tokens[inner + 2]?.text !== "." || tokens[inner + 3]?.text !== "join" || tokens[inner + 4]?.text !== "(") continue;
    const joinOpen = inner + 4, joinClose = pairs.get(joinOpen);
    if (joinClose === undefined) continue;
    const markerArgument = tokens.slice(joinOpen + 1, joinClose);
    if (markerArgument.length !== 1 || markerArgument[0]?.kind !== "string") continue;
    let after = joinClose + 1;
    if (tokens[after]?.text !== "." || tokens[after + 1]?.text !== "is_file" || tokens[after + 2]?.text !== "(" || tokens[after + 3]?.text !== ")" || tokens[after + 4]?.text !== "{") continue;
    const ifOpen = after + 4, ifClose = pairs.get(ifOpen);
    if (ifClose === undefined || tokens[ifOpen + 1]?.text !== "return" || tokens[ifOpen + 2]?.text !== variable || tokens[ifOpen + 3]?.text !== ";" || ifOpen + 4 !== ifClose) continue;
    const assertStart = ifClose + 1;
    if (tokens[assertStart]?.text !== "assert" || tokens[assertStart + 1]?.text !== "!" || tokens[assertStart + 2]?.text !== "(") continue;
    const assertOpen = assertStart + 2, assertClose = pairs.get(assertOpen);
    if (assertClose === undefined) continue;
    if (tokens[assertOpen + 1]?.text !== variable || tokens[assertOpen + 2]?.text !== "." || tokens[assertOpen + 3]?.text !== "pop" || tokens[assertOpen + 4]?.text !== "(" || tokens[assertOpen + 5]?.text !== ")" || tokens[assertOpen + 6]?.text !== ",") continue;
    if (tokens[assertClose + 1]?.text !== ";" || assertClose + 2 !== loopClose) continue;
    names.add(nameToken.text);
  }
  return names;
}

/** 🧭️ One path component and the immutable manifest-relative components preceding it. */
export interface RustManifestPathReference {
  readonly start: number;
  readonly end: number;
  readonly value: string;
  readonly base: readonly string[];
}

/** 🔗️ Proves local Path.join literals and single-use literal-array loop arguments from Rust syntax. */
export function inspectRustManifestPathReferences(source: string): readonly RustManifestPathReference[] {
  if (!source.includes("CARGO_MANIFEST_DIR") || !/\.\s*join\s*\(/u.test(source)) return [];
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens);
  if (tokens.some((token, index) => /^[()[\]{}]$/u.test(token.text) && !pairs.has(index))) return [];
  if (tokens.some((token, index) => token.text === "std" && ["mod", "as", "let"].includes(tokens[index - 1]?.text ?? ""))) return [];
  const repoRootHelperNames = rustRepoRootAncestorWalkHelperNames(tokens, pairs);
  type Loop = { readonly values: readonly RustToken[]; readonly uses: Set<number>; valid: boolean };
  type Binding = { readonly kind: "path"; readonly base: readonly string[] } | { readonly kind: "loop"; readonly loop: Loop };
  type Candidate = RustManifestPathReference & { readonly loop?: Loop };
  const rows: Candidate[] = [];
  const literal = (token: RustToken | undefined): token is RustToken & { readonly kind: "string" } => token?.kind === "string" && token.text.startsWith('"') && token.text.endsWith('"') && !token.text.includes("\\");
  const macros = new Set(["assert", "assert_eq", "assert_ne", "debug_assert", "debug_assert_eq", "debug_assert_ne"]);
  const formatArguments: Readonly<Record<string, number>> = { format: 0, format_args: 0, print: 0, println: 0, eprint: 0, eprintln: 0, panic: 0, write: 1, writeln: 1, assert: 1, assert_eq: 2, assert_ne: 2, debug_assert: 1, debug_assert_eq: 2, debug_assert_ne: 2 };
  const opaqueMacroContext = tokens.some((token) => token.text === "use" || token.text === "macro_rules");
  const formatCaptures = (text: string, name: string): boolean => {
    for (let index = 0; index < text.length; index++) {
      if (text[index] !== "{") continue;
      if (text[index + 1] === "{") { index++; continue; }
      const end = text.indexOf("}", index + 1), field = text.slice(index + 1, end < 0 ? undefined : end), colon = field.indexOf(":");
      if ((colon < 0 ? field : field.slice(0, colon)).trim() === name || colon >= 0 && [...field.slice(colon + 1).matchAll(/([_\p{ID_Start}][_\p{ID_Continue}]*)\$/gu)].some((match) => match[1] === name)) return true;
      if (end < 0) break;
      index = end;
    }
    return false;
  };
  const macroCaptures = (start: number, end: number, name: string): boolean => {
    for (let index = start; index + 2 < end; index++) {
      if (tokens[index + 1]?.text !== "!" || !["(", "[", "{"].includes(tokens[index + 2]?.text ?? "")) continue;
      const close = pairs.get(index + 2);
      if (close === undefined || close >= end) continue;
      const qualified = tokens[index - 1]?.text === "::", standard = qualified ? tokens[index - 2]?.text === "std" && (tokens[index - 3]?.text !== "::" || tokens[index - 4]?.kind !== "identifier") : !opaqueMacroContext;
      const argumentIndex = standard ? formatArguments[tokens[index]!.text] : undefined;
      const argument = argumentIndex === undefined ? undefined : rustTokenSegments(tokens, pairs, index + 3, close, ",")[argumentIndex], value = argument && argument[1] === argument[0] + 1 ? tokens[argument[0]] : undefined;
      if (literal(value)) { if (formatCaptures(value.text.slice(1, -1), name)) return true; }
      else if (tokens.slice(index + 3, close).some((token) => token.kind === "string" && token.text.split(/[^_\p{ID_Continue}]+/u).includes(name))) return true;
    }
    return false;
  };
  const path = (start: number, end: number, bindings: ReadonlyMap<string, Binding>, emit: boolean): { readonly end: number; readonly base: readonly string[] | null } | null => {
    let cursor = start, base: readonly string[] | null = null;
    const binding = bindings.get(tokens[cursor]?.text ?? "");
    if (binding?.kind === "path") { base = binding.base; cursor++; }
    else if (tokens[cursor]?.text === "(") {
      const close = pairs.get(cursor), nested = close === undefined ? null : path(cursor + 1, close, bindings, emit);
      if (!nested || nested.end !== close || nested.base === null) return null;
      base = nested.base; cursor = close! + 1;
    } else {
      if (tokens[cursor]?.text === "::") cursor++;
      const prefix = [["std", "::", "path", "::", "Path", "::", "new", "("], ["std", "::", "path", "::", "PathBuf", "::", "from", "("]].find((candidate) => candidate.every((text, offset) => tokens[cursor + offset]?.text === text));
      if (prefix) {
        const open = cursor + prefix.length - 1, close = pairs.get(open), environment = ["env", "!", "(", '"CARGO_MANIFEST_DIR"', ")"];
        if (close !== open + environment.length + 1 || !environment.every((text, offset) => tokens[open + offset + 1]?.text === text)) return null;
        base = []; cursor = close + 1;
      } else if (tokens[cursor]?.kind === "identifier" && repoRootHelperNames.has(tokens[cursor]!.text) && tokens[cursor + 1]?.text === "(" && tokens[cursor + 2]?.text === ")") {
        base = []; cursor = cursor + 3;
      } else return null;
    }
    while (cursor + 2 < end && tokens[cursor]?.text === "." && tokens[cursor + 1]?.text === "join" && tokens[cursor + 2]?.text === "(") {
      const close = pairs.get(cursor + 2);
      if (close === undefined || close >= end || base === null) break;
      const arguments_ = rustTokenSegments(tokens, pairs, cursor + 3, close, ",");
      const argument = arguments_.length === 1 ? arguments_[0] : undefined;
      if (!argument || argument[1] !== argument[0] + 1) break;
      const token = tokens[argument[0]]!, loop = token.kind === "identifier" ? bindings.get(token.text) : undefined;
      if (literal(token)) {
        const value = token.text.slice(1, -1);
        if (emit) rows.push({ start: token.start + 1, end: token.end - 1, value, base });
        base = [...base, value];
      } else if (loop?.kind === "loop") {
        if (emit) {
          loop.loop.uses.add(argument[0]);
          for (const value of loop.loop.values) rows.push({ start: value.start + 1, end: value.end - 1, value: value.text.slice(1, -1), base, loop: loop.loop });
        }
        base = null;
      } else break;
      cursor = close + 1;
    }
    return { end: cursor, base };
  };
  const visit = (start: number, end: number, bindings: Map<string, Binding>): void => {
    for (let index = start; index < end;) {
      const token = tokens[index]!;
      if (tokens[index + 1]?.text === "!" && ["(", "[", "{"].includes(tokens[index + 2]?.text ?? "") && !macros.has(token.text)) {
        index = (pairs.get(index + 2) ?? end - 1) + 1;
        continue;
      }
      if (token.text === "fn") {
        const open = rustFindTopLevel(tokens, pairs, index + 1, end, new Set(["{", ";"])), close = pairs.get(open);
        if (open >= 0 && tokens[open]?.text === "{" && close !== undefined) visit(open + 1, close, new Map());
        index = open < 0 ? end : (close ?? open) + 1;
        continue;
      }
      if (token.text === "|" && [undefined, "=", "(", ",", "move"].includes(tokens[index - 1]?.text)) {
        const close = tokens.findIndex((candidate, offset) => offset > index && offset < end && candidate.text === "|");
        if (close < 0) return;
        const child = new Map(bindings), boundary = rustFindTopLevel(tokens, pairs, close + 1, end, new Set([","]));
        for (const parameter of tokens.slice(index + 1, close)) if (parameter.kind === "identifier") child.delete(parameter.text);
        visit(close + 1, boundary < 0 ? end : boundary, child);
        index = boundary < 0 ? end : boundary + 1;
        continue;
      }
      if (["if", "while"].includes(token.text) && tokens[index + 1]?.text === "let") {
        const open = rustFindTopLevel(tokens, pairs, index + 2, end, new Set(["{"])), close = pairs.get(open), equal = rustFindTopLevel(tokens, pairs, index + 2, open, new Set(["="]));
        if (open < 0 || close === undefined || equal < 0) return;
        const child = new Map(bindings);
        visit(equal + 1, open, bindings);
        for (const parameter of tokens.slice(index + 2, equal)) if (parameter.kind === "identifier") child.delete(parameter.text);
        visit(open + 1, close, child);
        index = close + 1;
        continue;
      }
      if (token.text === "match") {
        const open = rustFindTopLevel(tokens, pairs, index + 1, end, new Set(["{"])), close = pairs.get(open);
        if (open < 0 || close === undefined) return;
        visit(index + 1, open, bindings);
        for (const [first, last] of rustTokenSegments(tokens, pairs, open + 1, close, ",")) {
          const arrow = rustFindTopLevel(tokens, pairs, first, last, new Set(["=>"]));
          if (arrow < 0) continue;
          const child = new Map(bindings);
          for (const parameter of tokens.slice(first, arrow)) if (parameter.kind === "identifier") child.delete(parameter.text);
          visit(arrow + 1, last, child);
        }
        index = close + 1;
        continue;
      }
      if (token.text === "for") {
        const open = rustFindTopLevel(tokens, pairs, index + 1, end, new Set(["{"])), close = pairs.get(open);
        if (open < 0 || close === undefined) return;
        const name = tokens[index + 1], array = index + 3, arrayEnd = pairs.get(array), child = new Map(bindings);
        if (name?.kind === "identifier") child.delete(name.text);
        if (name?.kind === "identifier" && tokens[index + 2]?.text === "in" && tokens[array]?.text === "[" && arrayEnd === open - 1) {
          const elements = rustTokenSegments(tokens, pairs, array + 1, arrayEnd, ",");
          if (elements.length > 0 && elements.every(([first, last]) => first + 1 === last && literal(tokens[first]))) {
            const loop: Loop = { values: elements.map(([first]) => tokens[first]!), uses: new Set(), valid: false };
            child.set(name.text, { kind: "loop", loop });
            visit(open + 1, close, child);
            const uses = tokens.slice(open + 1, close).filter((candidate) => candidate.kind === "identifier" && candidate.text === name.text).length;
            loop.valid = uses === 1 && loop.uses.size === 1 && !macroCaptures(open + 1, close, name.text);
            index = close + 1;
            continue;
          }
        }
        visit(open + 1, close, child);
        index = close + 1;
        continue;
      }
      if (token.text === "let") {
        const boundary = rustFindTopLevel(tokens, pairs, index + 1, end, new Set([";"]));
        if (boundary < 0) return;
        const equal = rustFindTopLevel(tokens, pairs, index + 1, boundary, new Set(["="])), name = tokens[index + 1]?.text === "mut" ? tokens[index + 2] : tokens[index + 1];
        if (equal >= 0) visit(equal + 1, boundary, bindings);
        const value = equal >= 0 ? path(equal + 1, boundary, bindings, false) : null;
        for (const parameter of tokens.slice(index + 1, equal < 0 ? boundary : equal)) if (parameter.kind === "identifier") bindings.delete(parameter.text);
        if (name?.kind === "identifier") {
          bindings.delete(name.text);
          if (equal === index + 2 && value?.end === boundary && value.base !== null) bindings.set(name.text, { kind: "path", base: value.base });
        }
        index = boundary + 1;
        continue;
      }
      if (token.kind === "identifier" && tokens[index + 1]?.text === "=") bindings.delete(token.text);
      const value = path(index, end, bindings, true);
      if (value) { index = value.end; continue; }
      const close = pairs.get(index);
      if (close !== undefined && close > index) {
        visit(index + 1, close, token.text === "{" ? new Map(bindings) : bindings);
        index = close + 1;
      } else index++;
    }
  };
  visit(0, tokens.length, new Map());
  const grouped = new Map<number, Candidate[]>();
  for (const row of rows.filter((row) => row.loop === undefined || row.loop.valid)) grouped.set(row.start, [...(grouped.get(row.start) ?? []), row]);
  return [...grouped.values()].filter((values) => new Set(values.map((row) => JSON.stringify(row.base))).size === 1).map(([row]) => ({ start: row!.start, end: row!.end, value: row!.value, base: row!.base })).sort((left, right) => left.start - right.start);
}

/** 🧮️ Complete manifest-relative alternatives for one literal, never writable reference authority. */
export interface RustManifestPathCandidate {
  readonly start: number;
  readonly end: number;
  readonly value: string;
  readonly targets: readonly (readonly string[])[];
}

/** 🧮️ Evaluates bounded immutable loop receivers while preserving tuple correlation and source identity. */
export function inspectRustManifestPathCandidates(source: string): readonly RustManifestPathCandidate[] {
  if (!source.includes("CARGO_MANIFEST_DIR") || !/\.\s*join\s*\(/u.test(source)) return [];
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), limit = 256;
  if (tokens.some((token, index) => /^[()[\]{}]$/u.test(token.text) && !pairs.has(index))) return [];
  if (tokens.some((token, index) => token.text === "std" && ["mod", "as", "let", "type", "struct", "use"].includes(tokens[index - 1]?.text ?? ""))) return [];
  const repoRootHelperNames = rustRepoRootAncestorWalkHelperNames(tokens, pairs);
  type State = { valid: boolean };
  type Value = { readonly state: State; readonly dependencies: readonly State[] } & (
    { readonly kind: "string"; readonly token: RustToken } |
    { readonly kind: "path"; readonly parts: readonly string[] } |
    { readonly kind: "array" | "tuple"; readonly values: readonly Value[] } |
    { readonly kind: "metadata" });
  type Row = { readonly token: RustToken; readonly alternatives: Map<string, readonly string[]>; readonly dependencies: State[] };
  const rows = new Map<number, Row>(), standardMacros = new Set(["assert", "assert_eq", "assert_ne", "debug_assert", "debug_assert_eq", "debug_assert_ne", "print", "println", "eprint", "eprintln", "format", "format_args", "panic", "write", "writeln"]);
  const shadowedMacros = new Set<string>(), moduleEnds: number[] = [];
  let wildcardMacroImport = false;
  for (let index = 0; index < tokens.length; index++) {
    while (moduleEnds.length && moduleEnds.at(-1)! < index) moduleEnds.pop();
    if (tokens[index]?.text === "mod" && tokens[index + 2]?.text === "{" && pairs.has(index + 2)) moduleEnds.push(pairs.get(index + 2)!);
    if (tokens[index]?.text === "macro_rules" && tokens[index + 1]?.text === "!") shadowedMacros.add(tokens[index + 2]?.text ?? "");
    if (tokens[index]?.text !== "use") continue;
    const end = rustFindTopLevel(tokens, pairs, index + 1, tokens.length, new Set([";"])), specifier = tokens.slice(index + 1, end < 0 ? tokens.length : end);
    const text = specifier.map((token) => token.text).join(""), localParent = /^(?:super::)+\*$/u.test(text) && text.split("::").length - 1 <= moduleEnds.length;
    if (specifier.some((token) => token.text === "*") && !localParent) wildcardMacroImport = true;
    for (const token of specifier) if (standardMacros.has(token.text) || token.text === "env") shadowedMacros.add(token.text);
  }
  if (wildcardMacroImport || shadowedMacros.has("env") || tokens.some((token) => ["no_std", "no_implicit_prelude", "macro_use"].includes(token.text))) return [];
  let expanded = 0, overflow = false;
  const wrap = (value: Value, parents: readonly State[] = []): Value => ({ ...value, state: { valid: true }, dependencies: [...value.dependencies, value.state, ...parents] });
  const metadata = (): Value => ({ kind: "metadata", state: { valid: true }, dependencies: [] });
  const invalidate = (value: Value | undefined): void => { if (value) value.state.valid = false; };
  const states = (value: Value): readonly State[] => [value.state, ...value.dependencies];
  const parse = (start: number, end: number, bindings: ReadonlyMap<string, Value>, emit: boolean): { value: Value; end: number } | null => {
    let cursor = start, value: Value | undefined, token = tokens[cursor];
    if (!token) return null;
    if (token.kind === "string" && token.text.startsWith('"') && token.text.endsWith('"') && !token.text.includes("\\")) {
      value = { kind: "string", token, state: { valid: true }, dependencies: [] }; cursor++;
    } else if (token.kind === "number" || ["true", "false"].includes(token.text)) { value = metadata(); cursor++; }
    else if (bindings.has(token.text)) { value = bindings.get(token.text)!; cursor++; }
    else if (token.text === "(" || token.text === "[") {
      const close = pairs.get(cursor);
      if (close === undefined || close >= end) return null;
      const segments = rustTokenSegments(tokens, pairs, cursor + 1, close, ","), values: Value[] = [];
      for (const [first, last] of segments) {
        const child = parse(first, last, bindings, emit);
        if (!child || child.end !== last) return null;
        values.push(child.value);
      }
      value = token.text === "(" && values.length === 1 && tokens[close - 1]?.text !== "," ? values[0]! : { kind: token.text === "[" ? "array" : "tuple", values, state: { valid: true }, dependencies: [] };
      cursor = close + 1;
    } else if (token.text === "&" && tokens[cursor + 1]?.text === "[") {
      const close = pairs.get(cursor + 1), child = close === undefined ? null : parse(cursor + 1, close + 1, bindings, emit);
      if (!child || tokens[close! + 1]?.text !== "[" || tokens[close! + 2]?.text !== ".." || tokens[close! + 3]?.text !== "]") return null;
      value = child.value; cursor = close! + 4;
    } else {
      if (token.text === "::") cursor++;
      const prefix = [["std", "::", "path", "::", "Path", "::", "new", "("], ["std", "::", "path", "::", "PathBuf", "::", "from", "("]].find((row) => row.every((text, offset) => tokens[cursor + offset]?.text === text));
      if (prefix) {
        const open = cursor + prefix.length - 1, close = pairs.get(open), environment = ["env", "!", "(", '"CARGO_MANIFEST_DIR"', ")"];
        if (close !== open + environment.length + 1 || !environment.every((text, offset) => tokens[open + offset + 1]?.text === text)) return null;
        value = { kind: "path", parts: [], state: { valid: true }, dependencies: [] }; cursor = close + 1;
      } else if (tokens[cursor]?.kind === "identifier" && repoRootHelperNames.has(tokens[cursor]!.text) && tokens[cursor + 1]?.text === "(" && tokens[cursor + 2]?.text === ")") {
        value = { kind: "path", parts: [], state: { valid: true }, dependencies: [] }; cursor = cursor + 3;
      } else return null;
    }
    while (cursor + 2 < end && tokens[cursor]?.text === "." && tokens[cursor + 2]?.text === "(") {
      const close = pairs.get(cursor + 2);
      if (close === undefined || close >= end) break;
      if (tokens[cursor + 1]?.text === "iter" && value.kind === "array" && close === cursor + 3 && tokens.slice(close + 1, close + 5).map((item) => item.text).join("") === ".enumerate()" && close + 4 < end) {
        value = { kind: "array", values: value.values.map((item) => ({ kind: "tuple", values: [metadata(), wrap(item, states(value!))], state: { valid: true }, dependencies: [] })), state: { valid: true }, dependencies: states(value) };
        cursor = close + 5; continue;
      }
      if (tokens[cursor + 1]?.text !== "join" || value.kind !== "path") break;
      const segments = rustTokenSegments(tokens, pairs, cursor + 3, close, ","), argument = segments.length === 1 ? parse(segments[0]![0], segments[0]![1], bindings, emit) : null;
      if (!argument || argument.end !== segments[0]![1] || argument.value.kind !== "string") break;
      if (/^(?:\/|[A-Za-z]:)/u.test(argument.value.token.text.slice(1, -1))) break;
      const leaf = argument.value, parts: readonly string[] = [...value.parts, leaf.token.text.slice(1, -1)], dependencies: State[] = [...states(value), ...states(leaf)];
      if (emit) {
        const row: Row = rows.get(leaf.token.start) ?? { token: leaf.token, alternatives: new Map(), dependencies: [] };
        row.alternatives.set(JSON.stringify(parts), parts); row.dependencies.push(...dependencies); rows.set(leaf.token.start, row);
        if (row.alternatives.size > limit) overflow = true;
      }
      value = { kind: "path", parts, state: { valid: true }, dependencies }; cursor = close + 1;
    }
    return { value, end: cursor };
  };
  const bind = (start: number, end: number, value: Value, bindings: Map<string, Value>): boolean => {
    if (start + 1 === end && tokens[start]?.kind === "identifier") { bindings.set(tokens[start]!.text, wrap(value)); return true; }
    if (tokens[start]?.text !== "(" || pairs.get(start) !== end - 1 || value.kind !== "tuple") return false;
    const names = rustTokenSegments(tokens, pairs, start + 1, end - 1, ",");
    if (names.length !== value.values.length || names.some(([first, last]) => first + 1 !== last || tokens[first]?.kind !== "identifier") || new Set(names.map(([first]) => tokens[first]!.text).filter((name) => name !== "_")).size !== names.filter(([first]) => tokens[first]!.text !== "_").length) return false;
    for (let index = 0; index < names.length; index++) bindings.set(tokens[names[index]![0]]!.text, wrap(value.values[index]!, states(value)));
    return true;
  };
  const callbackNamespace = !tokens.some((token, index) => {
    if (token.text === "#" || token.text === "std" && ["enum", "union", "trait", "<", ","].includes(tokens[index - 1]?.text ?? "")) return true;
    if (tokens[index + 1]?.text !== "!" || !["(", "[", "{"].includes(tokens[index + 2]?.text ?? "")) return false;
    if (token.text !== "env" && !standardMacros.has(token.text)) return true;
    return tokens[index - 1]?.text === "::" ? tokens[index - 2]?.text !== "std" || tokens[index - 3]?.text === "::" && tokens[index - 4]?.kind === "identifier" : shadowedMacros.has(token.text);
  });
  const divergentCallback = (start: number, end: number, bindings: ReadonlyMap<string, Value>): boolean => {
    if (!callbackNamespace || tokens[start - 1]?.text !== "(" || pairs.get(start - 1) !== end || tokens[start - 2]?.text !== "unwrap_or_else" || tokens[start - 3]?.text !== "." || tokens[start - 4]?.text !== ")") return false;
    const readClose = start - 4, readOpen = pairs.get(readClose);
    if (readOpen === undefined || readClose !== readOpen + 7 || !["std", "::", "fs", "::", "read_to_string"].every((text, index) => tokens[readOpen - 5 + index]?.text === text)) return false;
    if (tokens[readOpen - 6]?.text === "." || tokens[readOpen - 6]?.text === "::" && tokens[readOpen - 7]?.kind === "identifier") return false;
    const receiver = tokens[readOpen + 1], label = tokens[readOpen + 5], parameter = tokens[start + 1];
    if (receiver?.kind !== "identifier" || label?.kind !== "identifier" || parameter?.kind !== "identifier" || parameter.text === "_" || bindings.has(parameter.text) || tokens[start + 2]?.text !== "|" || ![".", "join", "("].every((text, index) => tokens[readOpen + 2 + index]?.text === text) || pairs.get(readOpen + 4) !== readOpen + 6) return false;
    const leaf = bindings.get(label.text), path = parse(readOpen + 1, readClose, bindings, false);
    if (leaf?.kind !== "string" || !states(leaf).every((state) => state.valid) || path?.end !== readClose || path.value.kind !== "path" || !states(path.value).every((state) => state.valid) || !path.value.dependencies.includes(leaf.state)) return false;
    let cursor = start + 3;
    if (tokens[cursor]?.text === "::") cursor++;
    if (tokens[cursor]?.text === "std" && tokens[cursor + 1]?.text === "::") cursor += 2;
    else if (cursor !== start + 3 || shadowedMacros.has("panic")) return false;
    if (tokens[cursor]?.text !== "panic" || tokens[cursor + 1]?.text !== "!" || tokens[cursor + 2]?.text !== "(" || pairs.get(cursor + 2) !== cursor + 4 || cursor + 5 !== end) return false;
    const message = tokens[cursor + 3];
    if (message?.kind !== "string" || !message.text.startsWith('"') || !message.text.endsWith('"') || message.text.includes("\\")) return false;
    const text = message.text.slice(1, -1), captures = new Set<string>();
    for (let index = 0; index < text.length;) {
      if (text[index] === "}") return false;
      if (text[index] !== "{") { index++; continue; }
      const close = text.indexOf("}", index + 1), name = text.slice(index + 1, close);
      if (close < 0 || !/^[A-Za-z_][A-Za-z0-9_]*$/u.test(name) || name !== label.text && name !== parameter.text) return false;
      captures.add(name); index = close + 1;
    }
    return captures.has(label.text) && captures.has(parameter.text);
  };
  const visit = (start: number, end: number, bindings: Map<string, Value>, readOnly = false): void => {
    for (let index = start; index < end && !overflow;) {
      const token = tokens[index]!;
      if (token.text === "fn") {
        const open = rustFindTopLevel(tokens, pairs, index + 1, end, new Set(["{", ";"])), close = pairs.get(open);
        if (open >= 0 && tokens[open]?.text === "{" && close !== undefined) visit(open + 1, close, new Map());
        index = open < 0 ? end : (close ?? open) + 1; continue;
      }
      if (["unsafe", "while", "match", "loop"].includes(token.text) || token.text === "if" && tokens[index + 1]?.text === "let") {
        for (const value of bindings.values()) invalidate(value);
        const open = rustFindTopLevel(tokens, pairs, index + 1, end, new Set(["{"])), close = pairs.get(open);
        index = close === undefined ? end : close + 1; continue;
      }
      const attributeOpen = tokens[index - 1]?.text === "]" ? pairs.get(index - 1) : undefined;
      if (token.text === "|" && ([undefined, "=", "(", ",", "move"].includes(tokens[index - 1]?.text) || attributeOpen !== undefined && tokens[attributeOpen - 1]?.text === "#")) {
        if (divergentCallback(index, end, bindings)) { index = end; continue; }
        for (const value of bindings.values()) invalidate(value);
        const boundary = rustFindTopLevel(tokens, pairs, index + 1, end, new Set([";"]));
        index = boundary < 0 ? end : boundary + 1; continue;
      }
      if (token.text === "for") {
        const open = rustFindTopLevel(tokens, pairs, index + 1, end, new Set(["{"])), close = pairs.get(open), inToken = rustFindTopLevel(tokens, pairs, index + 1, open, new Set(["in"]));
        if (open < 0 || close === undefined || inToken < 0) return;
        const sequence = parse(inToken + 1, open, bindings, false);
        if (sequence?.end === open && sequence.value.kind === "array") {
          for (const item of sequence.value.values) {
            if (++expanded > limit) { overflow = true; return; }
            const child = new Map(bindings);
            if (!bind(index + 1, inToken, wrap(item, states(sequence.value)), child)) { for (const value of bindings.values()) invalidate(value); break; }
            visit(open + 1, close, child);
          }
        } else for (const value of bindings.values()) invalidate(value);
        index = close + 1; continue;
      }
      if (token.text === "let") {
        const boundary = rustFindTopLevel(tokens, pairs, index + 1, end, new Set([";"]));
        if (boundary < 0) return;
        const equal = rustFindTopLevel(tokens, pairs, index + 1, boundary, new Set(["="])), resolved = equal < 0 ? null : parse(equal + 1, boundary, bindings, false);
        if (equal >= 0) visit(equal + 1, boundary, bindings, resolved?.end === boundary);
        for (const parameter of tokens.slice(index + 1, equal < 0 ? boundary : equal)) if (parameter.kind === "identifier") { invalidate(bindings.get(parameter.text)); bindings.delete(parameter.text); }
        if (equal >= 0 && resolved?.end === boundary) bind(index + 1, equal, resolved.value, bindings);
        index = boundary + 1; continue;
      }
      if (tokens[index + 1]?.text === "!" && ["(", "[", "{"].includes(tokens[index + 2]?.text ?? "")) {
        const close = pairs.get(index + 2);
        if (close === undefined) return;
        const qualified = tokens[index - 1]?.text === "::", standard = standardMacros.has(token.text) && (qualified ? tokens[index - 2]?.text === "std" && (tokens[index - 3]?.text !== "::" || tokens[index - 4]?.kind !== "identifier") : !wildcardMacroImport && !shadowedMacros.has(token.text));
        if (standard) visit(index + 3, close, bindings, true);
        else for (const item of tokens.slice(index + 3, close)) {
          if (item.kind === "identifier") invalidate(bindings.get(item.text));
          else if (item.kind === "string") for (const [name, value] of bindings) if (item.text.split(/[^_\p{ID_Continue}]+/u).includes(name)) invalidate(value);
        }
        index = close + 1; continue;
      }
      const parsed = parse(index, end, bindings, true);
      if (parsed?.value.kind === "path" && parsed.end > index + 1) { index = parsed.end; continue; }
      if (!readOnly && token.kind === "identifier" && bindings.has(token.text)) invalidate(bindings.get(token.text));
      const close = pairs.get(index);
      if (close !== undefined && close > index) { visit(index + 1, close, token.text === "{" ? new Map(bindings) : bindings, readOnly); index = close + 1; }
      else index++;
    }
  };
  visit(0, tokens.length, new Map());
  if (overflow) return [];
  return [...rows.values()].filter((row) => row.dependencies.every((state) => state.valid)).sort((left, right) => left.token.start - right.token.start).map((row) => ({ start: row.token.start + 1, end: row.token.end - 1, value: row.token.text.slice(1, -1), targets: [...row.alternatives.entries()].sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0).map(([, parts]) => parts) }));
}

/** 🧵️ Proves local standard string-collection delimiters without granting path receiver authority. */
function rustStringCollectionJoinArguments(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, macros: ReadonlySet<string>): ReadonlySet<number> {
  const shadows = new Set<string>(), names = new Set(["Vec", "String", "std", "format"]), moduleEnds: number[] = [];
  let wildcard = false, override = false;
  for (let index = 0; index < tokens.length; index++) {
    const token = tokens[index]!, previous = tokens[index - 1]?.text, next = tokens[index + 1]?.text;
    while (moduleEnds.length > 0 && moduleEnds.at(-1)! < index) moduleEnds.pop();
    if (token.text === "mod" && tokens[index + 1]?.kind === "identifier" && tokens[index + 2]?.text === "{" && pairs.has(index + 2)) moduleEnds.push(pairs.get(index + 2)!);
    if (token.text === "no_implicit_prelude" || token.text === "no_std") wildcard = true;
    if (token.text === "join" && previous === "fn") override = true;
    if (names.has(token.text) && ["struct", "enum", "union", "type", "trait", "mod", "as", "let"].includes(previous ?? "")) shadows.add(token.text);
    if (["fn", "impl", "struct", "enum", "trait", "type", "union"].includes(token.text)) {
      const open = index + (token.text === "impl" ? 1 : 2);
      if (tokens[open]?.text === "<") for (let cursor = open + 1, depth = 1; cursor < tokens.length && depth > 0; cursor++) {
        const value = tokens[cursor]!.text;
        if (depth === 1 && ["<", ","].includes(tokens[cursor - 1]?.text ?? "") && names.has(value)) shadows.add(value);
        if (value === "<") depth++;
        else if (value === ">" || value === ">>") depth -= value.length;
      }
    }
    if (token.text === "macro_rules" && next === "!" && names.has(tokens[index + 2]?.text ?? "")) shadows.add(tokens[index + 2]!.text);
    if (token.text === "use") {
      const end = rustFindTopLevel(tokens, pairs, index + 1, tokens.length, new Set([";"]));
      const specifier = rustTokenText(tokens, index + 1, end < 0 ? tokens.length : end).replace(/\s/gu, "");
      const localParent = /^(?:super::)+\*$/u.test(specifier) && specifier.split("::").length - 1 <= moduleEnds.length;
      for (const item of tokens.slice(index + 1, end < 0 ? tokens.length : end)) {
        if (item.text === "*" && !localParent) wildcard = true;
        if (names.has(item.text) && item.text !== "std") shadows.add(item.text);
      }
    }
  }
  if (override) return new Set();
  type Collection = { valid: boolean; strings: boolean };
  const rows: { start: number; collection: Collection }[] = [];
  const compact = (start: number, end: number): string => rustTokenText(tokens, start, end).replace(/\s/gu, "");
  const standard = (value: string, unqualified: string, qualified: string): boolean => value === unqualified ? !wildcard && !shadows.has(unqualified.split(/[:<]/u)[0]!) : (value === qualified || value === "::" + qualified) && !shadows.has("std");
  const stringType = (start: number, end: number): boolean => {
    const match = /^(.*)<(.*)>$/u.exec(compact(start, end));
    return match !== null && standard(match[1]!, "Vec", "std::vec::Vec") && standard(match[2]!, "String", "std::string::String");
  };
  const stringValue = (start: number, end: number): boolean => {
    const token = tokens[start];
    if (start + 1 === end && token?.kind === "string" && (token.text.startsWith('"') || /^r#*"/u.test(token.text))) return true;
    let open = start;
    while (open < end && tokens[open]?.text !== "!") open++;
    if (tokens[open + 1]?.text !== "(" || pairs.get(open + 1) !== end - 1) return false;
    return standard(compact(start, open), "format", "std::format");
  };
  const clear = (start: number, end: number, bindings: Map<string, Collection>): void => { for (const token of tokens.slice(start, end)) if (token.kind === "identifier") bindings.delete(token.text); };
  const visit = (start: number, end: number, bindings: Map<string, Collection>): void => {
    for (let index = start; index < end;) {
      const token = tokens[index]!;
      if (tokens[index + 1]?.text === "!" && ["(", "[", "{"].includes(tokens[index + 2]?.text ?? "")) {
        const close = pairs.get(index + 2);
        if (close === undefined) return;
        if (macros.has(token.text)) visit(index + 3, close, bindings);
        else for (const item of tokens.slice(index + 3, close)) { const value = bindings.get(item.text); if (value) value.valid = false; }
        index = close + 1; continue;
      }
      if (token.text === "fn") {
        const open = rustFindTopLevel(tokens, pairs, index + 1, end, new Set(["{", ";"])), close = pairs.get(open);
        if (open >= 0 && tokens[open]?.text === "{" && close !== undefined) visit(open + 1, close, new Map());
        index = open < 0 ? end : (close ?? open) + 1; continue;
      }
      if (token.text === "impl") {
        const open = rustFindTopLevel(tokens, pairs, index + 1, end, new Set(["{"]));
        index = open < 0 ? end : open; continue;
      }
      if (token.text === "let") {
        const boundary = rustFindTopLevel(tokens, pairs, index + 1, end, new Set([";"]));
        if (boundary < 0) return;
        const equal = rustFindTopLevel(tokens, pairs, index + 1, boundary, new Set(["="])), nameIndex = tokens[index + 1]?.text === "mut" ? index + 2 : index + 1;
        if (equal >= 0) visit(equal + 1, boundary, bindings);
        clear(index + 1, equal < 0 ? boundary : equal, bindings);
        if (equal >= 0 && tokens[nameIndex]?.kind === "identifier" && (nameIndex + 1 === equal || tokens[nameIndex + 1]?.text === ":")) {
          const typed = tokens[nameIndex + 1]?.text === ":" && stringType(nameIndex + 2, equal);
          if (typed || standard(compact(equal + 1, boundary), "Vec::new()", "std::vec::Vec::new()")) bindings.set(tokens[nameIndex]!.text, { valid: true, strings: typed });
        }
        index = boundary + 1; continue;
      }
      if (token.text === "for" || ["if", "while"].includes(token.text) && tokens[index + 1]?.text === "let") {
        const patternStart = index + (token.text === "for" ? 1 : 2), split = rustFindTopLevel(tokens, pairs, patternStart, end, new Set([token.text === "for" ? "in" : "="])), open = rustFindTopLevel(tokens, pairs, split + 1, end, new Set(["{"])), close = pairs.get(open);
        if (split < 0 || open < 0 || close === undefined) return;
        visit(split + 1, open, bindings);
        const child = new Map(bindings); clear(patternStart, split, child); visit(open + 1, close, child);
        index = close + 1; continue;
      }
      if (token.text === "match") {
        const open = rustFindTopLevel(tokens, pairs, index + 1, end, new Set(["{"])), close = pairs.get(open);
        if (open < 0 || close === undefined) return;
        visit(index + 1, open, bindings);
        for (const [first, last] of rustTokenSegments(tokens, pairs, open + 1, close, ",")) {
          const arrow = rustFindTopLevel(tokens, pairs, first, last, new Set(["=>"]));
          if (arrow < 0) continue;
          const child = new Map(bindings); clear(first, arrow, child); visit(arrow + 1, last, child);
        }
        index = close + 1; continue;
      }
      if (token.text === "|" && [undefined, "=", "(", ",", "move"].includes(tokens[index - 1]?.text)) {
        const close = tokens.findIndex((item, offset) => offset > index && offset < end && item.text === "|");
        if (close < 0) return;
        const boundary = rustFindTopLevel(tokens, pairs, close + 1, end, new Set([",", ";"])), child = new Map(bindings);
        clear(index + 1, close, child); visit(close + 1, boundary < 0 ? end : boundary, child);
        index = boundary < 0 ? end : boundary + 1; continue;
      }
      const collection = bindings.get(token.text);
      if (collection) {
        const method = tokens[index + 2]?.text, open = index + 3, close = pairs.get(open);
        if (tokens[index + 1]?.text === "." && tokens[open]?.text === "(" && close !== undefined) {
          const arguments_ = rustTokenSegments(tokens, pairs, open + 1, close, ","), argument = arguments_.length === 1 ? arguments_[0] : undefined;
          if (method === "push") { if (argument && stringValue(...argument)) collection.strings = true; else collection.valid = false; }
          else if (method === "join" && argument && argument[0] + 1 === argument[1] && tokens[argument[0]]?.kind === "string") rows.push({ start: tokens[argument[0]]!.start, collection });
          else if (!["len", "is_empty"].includes(method ?? "") || arguments_.length !== 0) collection.valid = false;
          visit(open + 1, close, bindings); index = close + 1; continue;
        }
        collection.valid = false;
        if (tokens[index + 1]?.text === "=") bindings.delete(token.text);
      }
      const close = pairs.get(index);
      if (close !== undefined && close > index) { visit(index + 1, close, token.text === "{" ? new Map(bindings) : bindings); index = close + 1; }
      else index++;
    }
  };
  visit(0, tokens.length, new Map());
  /** 🔗️ A second, binding-free pass: `<expr>.collect::<Vec<...>>().join("literal")` chained inline (no
   * intermediate `let`) can never be a `Path`/`PathBuf` receiver — `collect::<Vec<_>>()` is syntactically
   * guaranteed to produce a `Vec`, and `Path`/`PathBuf` are never spelled with a `collect` turbofish — so
   * this is provable purely structurally, without the `bindings` machinery above (which only sees named
   * local variables and therefore missed this exact idiom, the most common way Rust joins a mapped
   * iterator into one string). */
  const chained: number[] = [];
  for (let index = 0; index + 2 < tokens.length; index++) {
    if (tokens[index]!.text !== "." || tokens[index + 1]!.text !== "join" || tokens[index + 2]!.text !== "(") continue;
    const close = pairs.get(index + 2);
    if (close === undefined) continue;
    const arguments_ = rustTokenSegments(tokens, pairs, index + 3, close, ","), argument = arguments_.length === 1 ? arguments_[0] : undefined;
    if (!argument || argument[0] + 1 !== argument[1] || tokens[argument[0]]?.kind !== "string") continue;
    if (tokens[index - 1]?.text !== ")") continue;
    const receiverOpen = pairs.get(index - 1);
    if (receiverOpen === undefined || receiverOpen !== index - 2) continue;
    let depth = 0, cursor = receiverOpen - 1, turbofishOpen = -1;
    while (cursor >= 0) {
      const text = tokens[cursor]!.text, closers = text === ">>" ? 2 : text === ">" ? 1 : 0;
      if (closers > 0) { depth += closers; cursor--; continue; }
      if (text === "<") { depth -= 1; if (depth === 0) { turbofishOpen = cursor; break; } cursor--; continue; }
      if (depth === 0) break;
      cursor--;
    }
    if (turbofishOpen < 0 || tokens[turbofishOpen - 1]?.text !== "::" || tokens[turbofishOpen - 2]?.text !== "collect" || tokens[turbofishOpen - 3]?.text !== ".") continue;
    if (!standard(tokens[turbofishOpen + 1]?.text ?? "", "Vec", "std::vec::Vec")) continue;
    chained.push(tokens[argument[0]]!.start);
  }
  return new Set([...rows.filter(({ collection }) => collection.valid && collection.strings).map(({ start }) => start), ...chained]);
}

/** 🚧️ Identifies literal path candidates while excluding only proven standard collection delimiters. */
export function inspectRustJoinArgumentSpans(source: string): readonly Pick<RustManifestPathReference, "start" | "end" | "value">[] {
  if (!/\.\s*join\s*\(/u.test(source)) return [];
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), rows = new Map<number, Pick<RustManifestPathReference, "start" | "end" | "value">>();
  const macros = new Set(["assert", "assert_eq", "assert_ne", "debug_assert", "debug_assert_eq", "debug_assert_ne", "print", "println", "eprint", "eprintln", "format", "write", "writeln", "panic"]);
  const delimiters = rustStringCollectionJoinArguments(tokens, pairs, macros);
  const literal = (token: RustToken | undefined): token is RustToken & { readonly kind: "string" } => token?.kind === "string" && token.text.startsWith('"') && token.text.endsWith('"');
  const add = (token: RustToken): void => { if (!delimiters.has(token.start)) rows.set(token.start + 1, { start: token.start + 1, end: token.end - 1, value: token.text.slice(1, -1) }); };
  const visit = (start: number, end: number, loops: ReadonlyMap<string, readonly RustToken[]>): void => {
    for (let index = start; index < end; index++) {
      const token = tokens[index]!;
      if (tokens[index + 1]?.text === "!" && ["(", "[", "{"].includes(tokens[index + 2]?.text ?? "") && !macros.has(token.text)) { index = pairs.get(index + 2) ?? end; continue; }
      if (token.text === "for" && tokens[index + 1]?.kind === "identifier" && tokens[index + 2]?.text === "in" && tokens[index + 3]?.text === "[") {
        const arrayEnd = pairs.get(index + 3), open = arrayEnd === undefined ? undefined : arrayEnd + 1, close = open === undefined ? undefined : pairs.get(open);
        if (arrayEnd !== undefined && open !== undefined && tokens[open]?.text === "{" && close !== undefined) {
          const parts = rustTokenSegments(tokens, pairs, index + 4, arrayEnd, ","), child = new Map(loops);
          child.delete(tokens[index + 1]!.text);
          if (parts.every(([first, last]) => first + 1 === last && literal(tokens[first]))) child.set(tokens[index + 1]!.text, parts.map(([first]) => tokens[first]!));
          visit(open + 1, close, child); index = close; continue;
        }
      }
      if (token.text === "." && tokens[index + 1]?.text === "join" && tokens[index + 2]?.text === "(") {
        const close = pairs.get(index + 2), argument = tokens[index + 3];
        if (close === index + 4) {
          if (literal(argument)) add(argument);
          else if (argument?.kind === "identifier") for (const value of loops.get(argument.text) ?? []) add(value);
        }
      }
      const close = pairs.get(index);
      if (close !== undefined && close > index) { visit(index + 1, close, loops); index = close; }
    }
  };
  visit(0, tokens.length, new Map());
  return [...rows.values()].sort((left, right) => left.start - right.start);
}
