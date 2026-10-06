import { rustIdentifierSymbol, rustStringValue, rustTokenPairs, rustTokenSegments, rustTokens } from "../🟦️.ts";

/** 📁️ An owner-qualified literal argument with UTF-16 offsets; method receiver types remain unresolved. */
export interface RustPathLiteral {
  readonly value: string;
  readonly path: string;
  readonly root: string;
  readonly call: string;
  readonly context: "method" | "qualified";
  readonly start: number;
  readonly end: number;
  readonly line: number;
}
const qualifiedCalls = new Set([
  "std::path::Path::new", "std::path::PathBuf::from",
  ...["read", "read_to_string", "read_dir", "write", "metadata", "symlink_metadata", "canonicalize", "create_dir", "create_dir_all", "remove_file", "remove_dir", "remove_dir_all"].map(name => "std::fs::" + name),
  "std::fs::File::open", "std::fs::File::create",
]);

/** ⛔️ Refuses ambiguous or noncanonical caller-owned path roots. */
export class RustPathRootError extends Error {
  readonly code = "invalid-root";
  constructor() {
    super("Rust path root must be unique and canonical relative");
    this.name = "RustPathRootError";
  }
}

/** 🧭️ Finds whole literal first arguments of path methods and explicitly qualified standard path calls. */
export function inspectRustPathLiterals(source: string, roots: readonly string[]): readonly RustPathLiteral[] {
  if (new Set(roots).size !== roots.length || roots.some(root => !root || /[\\:\u0000\r\n]/u.test(root) || root.split("/").some(part => !part || part === "." || part === ".."))) throw new RustPathRootError();
  const ordered = [...roots].sort((a, b) => b.length - a.length), tokens = rustTokens(source), pairs = rustTokenPairs(tokens), references: RustPathLiteral[] = [];
  let cursor = 0, line = 1;
  for (let open = 0; open < tokens.length; open++) {
    if (tokens[open]!.text !== "(") continue;
    const close = pairs.get(open), name = rustIdentifierSymbol(tokens[open - 1]);
    if (close === undefined || !name) continue;
    const context = tokens[open - 2]?.text === "." ? "method" : "qualified";
    let first = open - 1;
    if (context === "qualified") while (tokens[first - 1]?.text === "::" && rustIdentifierSymbol(tokens[first - 2])) first -= 2;
    const call = context === "method" ? name : tokens.slice(first, open).map(token => token.text).join("");
    if (!(context === "method" ? name === "join" || name === "push" : qualifiedCalls.has(call))) continue;
    const argument = rustTokenSegments(tokens, pairs, open + 1, close, ",")[0];
    if (!argument || argument[1] !== argument[0] + 1) continue;
    const token = tokens[argument[0]]!, value = rustStringValue(token);
    if (value === null) continue;
    const path = value.replaceAll("\\", "/").replace(/^(?:(?:\.|\.\.)\/)+/u, "");
    const root = ordered.find(root => path === root || path.startsWith(root + "/"));
    if (!root || /[\u0000\r\n]/u.test(path)) continue;
    while (cursor < token.start) if (source.charCodeAt(cursor++) === 10) line++;
    references.push({ value, path, root, call, context, start: token.start, end: token.end, line });
  }
  return references.sort((a, b) => a.start - b.start);
}
