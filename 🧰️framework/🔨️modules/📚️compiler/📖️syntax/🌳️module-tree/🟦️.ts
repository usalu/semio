import { existsSync, readFileSync, statSync } from "node:fs";
import { dirname, extname, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { ecmaStringValue, ecmaTokens } from "../🟨️ecma/🟦️.ts";
import { rustAttributes, rustFindTopLevel, rustPathAttributes, rustTokenPairs, rustTokens, rustVisibility } from "../🦀️rust/🟦️.ts";

/** 🌳️ One `#[path = "..."] mod name;` declaration: the literal and the inline module names that enclose it. */
export interface RustPathMount {
  readonly path: string;
  readonly modules: readonly string[];
}

/** 🎚️ Module tree reading options; canonical `🧪️tests` mounts are excluded unless asked for, and `read` replaces the plain file read (for example with a symlink-refusing one). */
export interface ModuleTreeOptions {
  readonly tests?: boolean;
  readonly read?: (path: string) => string;
}

const TESTS_DIRECTORY = "🧪️tests";
const ECMA_ENTRY_NAMES = ["🟦️.tsx", "🟦️.ts"];

/** 🧭️ Lists every `#[path]` file module of a Rust source in declaration order, with cfg attributes ignored and inline module nesting kept.
 * @see https://doc.rust-lang.org/reference/items/modules.html#the-path-attribute */
export function inspectRustPathMounts(source: string): readonly RustPathMount[] {
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), mounts: RustPathMount[] = [];
  const scope = (start: number, end: number, modules: readonly string[]): void => {
    for (let index = start; index < end;) {
      if (tokens[index]?.text === "#" && tokens[index + 1]?.text === "!" && tokens[index + 2]?.text === "[") {
        const close = pairs.get(index + 2);
        if (close === undefined) throw Error("Invalid inner Rust attribute");
        index = close + 1;
        continue;
      }
      const attributes = rustAttributes(tokens, pairs, index, end), at = rustVisibility(tokens, pairs, attributes.next).next;
      if (tokens[at]?.text === "mod" && tokens[at + 1]?.kind === "identifier") {
        const name = tokens[at + 1]!.text.replace(/^r#/u, ""), boundary = at + 2, paths = rustPathAttributes(tokens, pairs, attributes);
        if (paths.length > 1 || paths.some(row => row.path === null)) throw Error("Ambiguous Rust path mount: " + name);
        const mount = paths[0]?.path ?? undefined;
        if (tokens[boundary]?.text === "{") {
          const close = pairs.get(boundary)!;
          scope(boundary + 1, close, [...modules, mount ?? name]);
          index = close + 1;
          continue;
        }
        if (tokens[boundary]?.text === ";") {
          if (mount !== undefined) mounts.push({ path: mount, modules });
          index = boundary + 1;
          continue;
        }
      }
      const boundary = rustFindTopLevel(tokens, pairs, at, end, new Set([";", "{"]));
      if (boundary < 0) break;
      index = (pairs.get(boundary) ?? boundary) + 1;
    }
  };
  scope(0, tokens.length, []);
  return mounts;
}

/** 🔗️ Lists the static relative `import ... from`, `export ... from` and side-effect `import` specifiers of an ECMAScript source in order. */
export function inspectEcmaRelativeImports(source: string): readonly string[] {
  const tokens = ecmaTokens(source), specifiers: string[] = [];
  for (const [index, token] of tokens.entries()) {
    const previous = tokens[index - 1]?.text, value = ecmaStringValue(token);
    if (value === null || previous !== "from" && previous !== "import") continue;
    if (value.startsWith("./") || value.startsWith("../")) specifiers.push(value);
  }
  return specifiers;
}

const segmentsOf = (from: string, to: string): readonly string[] => relative(from, to).split(/[\\/]/u);

const filePath = (root: string | URL): string => root instanceof URL ? fileURLToPath(root) : resolve(root);

const readSource = (path: string, options: ModuleTreeOptions): string => options.read ? options.read(path) : readFileSync(path, "utf8");

function rustChildren(file: string, text: string, options: ModuleTreeOptions): readonly string[] {
  const directory = dirname(file), children: string[] = [];
  for (const mount of inspectRustPathMounts(text)) {
    const target = resolve(directory, ...mount.modules, mount.path), segments = segmentsOf(directory, target);
    if (segments.includes(TESTS_DIRECTORY) ? !options.tests : segments[0] === "..") continue;
    if (!existsSync(target)) throw Error("Rust module tree mount missing: " + target + " (declared in " + file + ")");
    children.push(target);
  }
  return children;
}

function ecmaChildren(file: string, text: string, options: ModuleTreeOptions): readonly string[] {
  const directory = dirname(file), children: string[] = [];
  for (const specifier of inspectEcmaRelativeImports(text)) {
    const base = resolve(directory, specifier), segments = segmentsOf(directory, base);
    if (segments.includes(TESTS_DIRECTORY) ? !options.tests : segments[0] === "..") continue;
    const target = [".ts", ".tsx"].includes(extname(base)) ? base : existsSync(base) && statSync(base).isDirectory() ? ECMA_ENTRY_NAMES.map(name => resolve(base, name)).find(existsSync) : undefined;
    if (target !== undefined && existsSync(target) && statSync(target).isFile()) children.push(target);
  }
  return children;
}

const walkModuleTree = (root: string | URL, options: ModuleTreeOptions): readonly Readonly<{ path: string; text: string }>[] => {
  const start = filePath(root), files: { path: string; text: string }[] = [], seen = new Set<string>();
  const children = extname(start) === ".rs" ? rustChildren : [".ts", ".tsx"].includes(extname(start)) ? ecmaChildren : undefined;
  if (children === undefined) throw Error("Module tree needs a .rs, .ts or .tsx root: " + start);
  const visit = (file: string): void => {
    if (seen.has(file)) return;
    seen.add(file);
    const text = readSource(file, options);
    files.push({ path: file, text });
    for (const child of children(file, text, options)) visit(child);
  };
  visit(start);
  return files;
};

/** 📚️ Lists the root file and every file reachable through its own module mounts (`.rs` `#[path]` declarations, `.ts`/`.tsx` relative imports below the declaring directory), parent first, children in declaration order, each once. */
export function listModuleTree(root: string | URL, options: ModuleTreeOptions = {}): readonly string[] {
  return walkModuleTree(root, options).map(file => file.path);
}

/** 📖️ Reads the whole module tree of a split source as one text: the parent first, then every child in declaration order, so source-text laws keep seeing what the unsplit file held. */
export function readModuleTreeSource(root: string | URL, options: ModuleTreeOptions = {}): string {
  return walkModuleTree(root, options).map(file => file.text.endsWith("\n") ? file.text : file.text + "\n").join("");
}

/** ✂️ Slices one marker-delimited item out of a split source the way `text.slice(text.indexOf(from), text.indexOf(to))` did on the unsplit file: inside the first file that holds `from`, up to the first `to` after it, or to the end of that file when `to` is absent or lives in a later file. */
export function sliceModuleTreeSource(root: string | URL, from: string, to?: string, options: ModuleTreeOptions = {}): string {
  for (const { text } of walkModuleTree(root, options)) {
    const start = text.indexOf(from);
    if (start < 0) continue;
    const end = to === undefined ? -1 : text.indexOf(to, start + from.length);
    return end < 0 ? text.slice(start) : text.slice(start, end);
  }
  throw Error("Module tree marker missing: " + from);
}
