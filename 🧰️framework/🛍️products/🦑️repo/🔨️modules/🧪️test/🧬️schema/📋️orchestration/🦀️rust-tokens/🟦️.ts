/** 🦀️ Rust tokens
 *
 * The Rust lexer of the mutation gates and its structural readers: group ends, impl blocks, functions, call arguments, paths and first literals.
 */

//#region ⚖️MutationPayloadParity
export type RustToken = Readonly<{ kind: "ident" | "literal" | "punct"; text: string }>;

/** ✂️ Skips whitespace, line comments and (nested) block comments of Rust source from `start`. */
function rustTrivia(source: string, start: number): number {
  let index = start;
  for (;;) {
    while (index < source.length && /\s/u.test(source[index]!)) index += 1;
    if (source.startsWith("//", index)) {
      const end = source.indexOf("\n", index);
      index = end < 0 ? source.length : end + 1;
    } else if (source.startsWith("/*", index)) {
      let depth = 0;
      do {
        if (source.startsWith("/*", index)) (depth += 1), (index += 2);
        else if (source.startsWith("*/", index)) (depth -= 1), (index += 2);
        else index += 1;
      } while (depth > 0 && index < source.length);
    } else return index;
  }
}

/** 🔤️ Rust source as identifier, literal and punctuation tokens; comments are dropped and strings, raw strings and char literals stay whole. */
export function rustLex(source: string): RustToken[] {
  const tokens: RustToken[] = [];
  let index = 0;
  while ((index = rustTrivia(source, index)) < source.length) {
    const char = source[index]!;
    const raw = /^b?r(#*)"/u.exec(source.slice(index, index + 16));
    let end = index + 1;
    if (raw !== null) {
      const close = source.indexOf(`"${raw[1]}`, index + raw[0].length);
      end = close < 0 ? source.length : close + 1 + raw[1]!.length;
    } else if (char === '"' || (char === "b" && source[index + 1] === '"')) {
      end = index + (char === "b" ? 2 : 1);
      while (end < source.length && source[end] !== '"') end += source[end] === "\\" ? 2 : 1;
      end += 1;
    } else if (char === "'") {
      const width = (source.codePointAt(index + 1) ?? 0) > 0xffff ? 2 : 1;
      end = source[index + 1] === "\\" ? source.indexOf("'", index + 2) + 1 : source[index + 1 + width] === "'" ? index + 2 + width : index + 1;
    } else {
      const ident = /^[\p{L}_][\p{L}\p{N}_]*/u.exec(source.slice(index, index + 256));
      if (ident !== null) {
        tokens.push({ kind: "ident", text: ident[0] });
        index += ident[0].length;
        continue;
      }
      tokens.push({ kind: "punct", text: char });
      index += 1;
      continue;
    }
    tokens.push({ kind: "literal", text: source.slice(index, end) });
    index = end;
  }
  return tokens;
}

/** 🔚️ The index of the punctuation token closing the group opened at `open`. */
export function rustGroupEnd(tokens: readonly RustToken[], open: number): number {
  let depth = 0;
  for (let index = open; index < tokens.length; index += 1) {
    const token = tokens[index]!;
    if (token.kind !== "punct") continue;
    if (token.text === "(" || token.text === "[" || token.text === "{") depth += 1;
    else if ((token.text === ")" || token.text === "]" || token.text === "}") && --depth === 0) return index;
  }
  return tokens.length - 1;
}
//#endregion ⚖️MutationPayloadParity

//#region 🦀️RustSources
/** 🧱️ One `impl … Trait<…> for Type { … }` block of a Rust source: the trait's last path segment, its first generic argument as text,
 * the implementing type's last path segment, and the token range of its body (the braces included). */
export type RustImplBlock = Readonly<{ trait: string; argument: string; implementor: string; open: number; close: number }>;

/** 🧱️ Every trait impl block of `tokens`, nested ones included. */
export function rustImplBlocks(tokens: readonly RustToken[]): RustImplBlock[] {
  const blocks: RustImplBlock[] = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index]!.kind !== "ident" || tokens[index]!.text !== "impl") continue;
    let open = index + 1;
    let angle = 0;
    let forAt = -1;
    while (open < tokens.length && !(tokens[open]!.text === "{" && angle <= 0) && tokens[open]!.text !== ";") {
      const text = tokens[open]!.text;
      if (text === "<") angle += 1;
      else if (text === ">" && tokens[open - 1]?.text !== "-") angle -= 1;
      else if (text === "for" && angle === 0 && forAt < 0) forAt = open;
      open += 1;
    }
    if (forAt < 0 || tokens[open]?.text !== "{") continue;
    let traitEnd = forAt - 1;
    let argument = "";
    if (tokens[traitEnd]?.text === ">") {
      let depth = 0;
      let start = traitEnd;
      for (; start > index; start -= 1) {
        if (tokens[start]!.text === ">" && tokens[start - 1]?.text !== "-") depth += 1;
        else if (tokens[start]!.text === "<" && --depth === 0) break;
      }
      const inner = tokens.slice(start + 1, traitEnd);
      const comma = inner.findIndex((token, at) => token.text === "," && inner.slice(0, at).filter((part) => part.text === "<").length === inner.slice(0, at).filter((part) => part.text === ">").length);
      argument = (comma < 0 ? inner : inner.slice(0, comma)).map((token) => token.text).join("").replace(/^::/u, "");
      traitEnd = start - 1;
    }
    const trait = tokens[traitEnd]?.kind === "ident" ? tokens[traitEnd]!.text : "";
    const path = tokens.slice(forAt + 1, open);
    const head = path.findIndex((token) => token.text === "<" || token.text === "where");
    const named = (head < 0 ? path : path.slice(0, head)).filter((token) => token.kind === "ident");
    blocks.push({ trait, argument, implementor: named.at(-1)?.text ?? "", open, close: rustGroupEnd(tokens, open) });
  }
  return blocks;
}

/** 🔧️ Every `fn <name>` item directly inside the block `open..close`, with the token range of its body (absent for a declaration). */
export function rustFunctions(tokens: readonly RustToken[], open: number, close: number): { readonly name: string; readonly start: number; readonly body: readonly [number, number] | null }[] {
  const functions: { name: string; start: number; body: readonly [number, number] | null }[] = [];
  for (let index = open + 1; index < close; index += 1) {
    const token = tokens[index]!;
    if (token.kind === "punct" && token.text === "{") {
      index = rustGroupEnd(tokens, index);
      continue;
    }
    if (token.kind !== "ident" || token.text !== "fn" || tokens[index + 1]?.kind !== "ident") continue;
    let cursor = index + 2;
    while (cursor < close && tokens[cursor]!.text !== "(") cursor += 1;
    cursor = rustGroupEnd(tokens, cursor) + 1;
    while (cursor < close && tokens[cursor]!.text !== "{" && tokens[cursor]!.text !== ";") cursor += 1;
    const body = tokens[cursor]?.text === "{" ? ([cursor, rustGroupEnd(tokens, cursor)] as const) : null;
    functions.push({ name: tokens[index + 1]!.text, start: index, body });
    index = body === null ? cursor : body[1];
  }
  return functions;
}

/** ✂️ The arguments of the call whose `(` sits at `open`: one token run per top-level comma (a trailing comma adds none). */
export function rustCallArguments(tokens: readonly RustToken[], open: number): RustToken[][] {
  const close = rustGroupEnd(tokens, open);
  const args: RustToken[][] = [[]];
  let depth = 0;
  for (let index = open + 1; index < close; index += 1) {
    const token = tokens[index]!;
    if (token.kind === "punct" && "([{".includes(token.text)) depth += 1;
    else if (token.kind === "punct" && ")]}".includes(token.text)) depth -= 1;
    if (depth === 0 && token.kind === "punct" && token.text === ",") args.push([]);
    else args.at(-1)!.push(token);
  }
  return args.at(-1)!.length === 0 ? args.slice(0, -1) : args;
}

/** 🔗️ Whether `tokens[index…]` spells the path `segments` (`A::b`). */
export function rustPathAt(tokens: readonly RustToken[], index: number, segments: readonly string[]): boolean {
  return segments.every((segment, offset) => tokens[index + offset * 3]?.text === segment && (offset === segments.length - 1 || (tokens[index + offset * 3 + 1]?.text === ":" && tokens[index + offset * 3 + 2]?.text === ":")));
}

/** 🔤️ The text of the first string literal of `tokens`, quotes stripped; `null` when it holds none. */
export function rustFirstLiteral(tokens: readonly RustToken[]): string | null {
  const literal = tokens.find((token) => token.kind === "literal" && /^b?r?#*"/u.test(token.text));
  return literal === undefined ? null : literal.text.replace(/^b?r?#*"|"#*$/gu, "");
}
//#endregion 🦀️RustSources
