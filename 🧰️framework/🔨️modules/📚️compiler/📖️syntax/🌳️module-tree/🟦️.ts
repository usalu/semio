import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { dirname, extname, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { ecmaStringValue, ecmaTokens } from "../🟨️ecma/📦️packages/🟦️typescript/🟦️.ts";
import { rustAttributes, rustFindTopLevel, rustIdentifierSymbol, rustPathAttributes, rustStringValue, rustTokenPairs, rustTokens, rustVisibility } from "../🦀️rust/📦️packages/🟦️typescript/🟦️.ts";

/** 🌳️ One `#[path = "..."] mod name;` declaration: the literal and the inline module names that enclose it. */
export interface RustPathMount {
  readonly path: string;
  readonly modules: readonly string[];
}

/** 🪢️ One file a Rust source pulls into its module tree: a `#[path]` file module, or an `include!("...")` of a literal file whose items are spliced into the including module. */
export interface RustModuleChild extends RustPathMount {
  readonly via: "path" | "include";
}

/** 🎚️ Module tree reading options; canonical `🧪️tests` mounts are excluded unless asked for, `crates: false` stops a facade, hub or root file of the renderer partition from pulling in the crates it names, and `read` replaces the plain file read (for example with a symlink-refusing one). */
export interface ModuleTreeOptions {
  readonly tests?: boolean;
  readonly crates?: boolean;
  readonly read?: (path: string) => string;
}

const TESTS_DIRECTORY = "🧪️tests";
const ECMA_ENTRY_NAMES = ["🟦️.tsx", "🟦️.ts"];

/** 🧭️ Lists every `#[path]` file module of a Rust source in declaration order, with cfg attributes ignored and inline module nesting kept.
 * @see https://doc.rust-lang.org/reference/items/modules.html#the-path-attribute */
export function inspectRustPathMounts(source: string): readonly RustPathMount[] {
  return inspectRustModuleChildren(source).filter(child => child.via === "path").map(({ path, modules }) => ({ path, modules }));
}

/** 🦀️ Lists every `#[path]` file module and every `include!("literal")` item of a Rust source in declaration order, with cfg attributes ignored and inline module nesting kept; computed `include!` arguments are not children.
 * @see https://doc.rust-lang.org/std/macro.include.html */
export function inspectRustModuleChildren(source: string): readonly RustModuleChild[] {
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), mounts: RustModuleChild[] = [];
  const scope = (start: number, end: number, modules: readonly string[]): void => {
    for (let index = start; index < end;) {
      if (tokens[index]?.text === "#" && tokens[index + 1]?.text === "!" && tokens[index + 2]?.text === "[") {
        const close = pairs.get(index + 2);
        if (close === undefined) throw Error("Invalid inner Rust attribute");
        index = close + 1;
        continue;
      }
      const attributes = rustAttributes(tokens, pairs, index, end), at = rustVisibility(tokens, pairs, attributes.next).next;
      if (rustIdentifierSymbol(tokens[at]) === "include" && tokens[at + 1]?.text === "!" && ["(", "["].includes(tokens[at + 2]?.text ?? "") && pairs.get(at + 2) === at + 4) {
        const literal = rustStringValue(tokens[at + 3]);
        if (literal !== null) mounts.push({ path: literal, modules, via: "include" });
      }
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
          if (mount !== undefined) mounts.push({ path: mount, modules, via: "path" });
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

/** 🔝️ Lists the first path segment of every top-level `use` and `extern crate` item of a Rust source in declaration order (cfg attributes ignored, inline modules not entered): the crate names a facade, hub or root file refers to.
 * @see https://doc.rust-lang.org/reference/items/use-declarations.html */
export function inspectRustCrateReferences(source: string): readonly string[] {
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), names: string[] = [];
  for (let index = 0; index < tokens.length;) {
    if (tokens[index]?.text === "#" && tokens[index + 1]?.text === "!" && tokens[index + 2]?.text === "[") {
      const close = pairs.get(index + 2);
      if (close === undefined) throw Error("Invalid inner Rust attribute");
      index = close + 1;
      continue;
    }
    const attributes = rustAttributes(tokens, pairs, index, tokens.length), at = rustVisibility(tokens, pairs, attributes.next).next;
    const first = tokens[at]?.text === "use" ? at + 1 : tokens[at]?.text === "extern" && tokens[at + 1]?.text === "crate" ? at + 2 : -1;
    if (first >= 0) {
      const name = rustIdentifierSymbol(tokens[tokens[first]?.text === "::" ? first + 1 : first]);
      if (typeof name === "string") names.push(name.replace(/^r#/u, ""));
    }
    const boundary = rustFindTopLevel(tokens, pairs, at, tokens.length, new Set([";", "{"]));
    if (boundary < 0) break;
    index = (pairs.get(boundary) ?? boundary) + 1;
  }
  return names;
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

/** 🌲️ A child file with the directory its own mounts stay below: a module owns its own directory, an `include!`d file stays inside the directory of the file that includes it, because its items belong to that module. */
interface TreeChild { readonly target: string; readonly scope: string; }

const CRATE_PREFIX = "semio_framework_os_renderer_wgpu_";

/** 🧩️ The crates of one renderer partition (`<engine>/🎯️targets/🧊️wgpu/<crate>/📦️packages/🦀️rust/Cargo.toml`): library name to crate root, and every file a crate root owns through its own mounts. */
interface CratePartition { readonly crates: ReadonlyMap<string, string>; readonly owned: ReadonlySet<string>; readonly scope: string; }

const partitions = new Map<string, CratePartition | null>();

function cratePartition(file: string): CratePartition | null {
  for (let directory = dirname(file); directory !== dirname(directory); directory = dirname(directory)) {
    const target = resolve(directory, "🎯️targets", "🧊️wgpu");
    if (!existsSync(target) || !statSync(target).isDirectory()) continue;
    const known = partitions.get(target);
    if (known === null) continue;
    if (known !== undefined) return known;
    const crates = new Map<string, string>();
    for (const entry of readdirSync(target, { withFileTypes: true })) {
      const manifest = resolve(target, entry.name, "📦️packages", "🦀️rust", "Cargo.toml");
      if (!entry.isDirectory() || !existsSync(manifest)) continue;
      const name = /^name\s*=\s*"semio-framework-os-renderer-wgpu-([a-z0-9-]+)"/mu.exec(readFileSync(manifest, "utf8"))?.[1];
      if (name !== undefined) crates.set(CRATE_PREFIX + name.replaceAll("-", "_"), resolve(target, entry.name, "🦀️.rs"));
    }
    const owned = new Set<string>();
    for (const root of crates.values()) {
      try { for (const owner of walkModuleTree(root, { tests: true, crates: false }, directory)) owned.add(owner.path); } catch { owned.add(root); }
    }
    if (crates.size === 0) {
      partitions.set(target, null);
      continue;
    }
    const found: CratePartition = { crates, owned, scope: directory };
    partitions.set(target, found);
    return found;
  }
  return null;
}

function rustChildren(file: string, text: string, options: ModuleTreeOptions, scope: string): readonly TreeChild[] {
  const directory = dirname(file), children: TreeChild[] = [], named: TreeChild[] = [];
  if (options.crates !== false && text.includes(CRATE_PREFIX)) {
    const partition = cratePartition(file);
    if (partition !== null && !partition.owned.has(file)) {
      for (const name of inspectRustCrateReferences(text)) {
        const root = partition.crates.get(name);
        if (root !== undefined && existsSync(root)) named.push({ target: root, scope: partition.scope });
      }
    }
  }
  for (const mount of inspectRustModuleChildren(text)) {
    const included = mount.via === "include", target = included ? resolve(directory, mount.path) : resolve(directory, ...mount.modules, mount.path), segments = segmentsOf(scope, target);
    if (segments.includes(TESTS_DIRECTORY) ? !options.tests : segments[0] === "..") continue;
    if (!existsSync(target)) throw Error("Rust module tree mount missing: " + target + " (declared in " + file + ")");
    children.push({ target, scope: included ? scope : dirname(target) });
  }
  return [...children, ...named];
}

function ecmaChildren(file: string, text: string, options: ModuleTreeOptions): readonly TreeChild[] {
  const directory = dirname(file), children: TreeChild[] = [];
  for (const specifier of inspectEcmaRelativeImports(text)) {
    const base = resolve(directory, specifier), segments = segmentsOf(directory, base);
    if (segments.includes(TESTS_DIRECTORY) ? !options.tests : segments[0] === "..") continue;
    const target = [".ts", ".tsx"].includes(extname(base)) ? base : existsSync(base) && statSync(base).isDirectory() ? ECMA_ENTRY_NAMES.map(name => resolve(base, name)).find(existsSync) : undefined;
    if (target !== undefined && existsSync(target) && statSync(target).isFile()) children.push({ target, scope: dirname(target) });
  }
  return children;
}

const walkModuleTree = (root: string | URL, options: ModuleTreeOptions, startScope?: string): readonly Readonly<{ path: string; text: string }>[] => {
  const start = filePath(root), files: { path: string; text: string }[] = [], seen = new Set<string>();
  const children = extname(start) === ".rs" ? rustChildren : [".ts", ".tsx"].includes(extname(start)) ? (file: string, text: string, opts: ModuleTreeOptions, _scope: string) => ecmaChildren(file, text, opts) : undefined;
  if (children === undefined) throw Error("Module tree needs a .rs, .ts or .tsx root: " + start);
  const visit = (file: string, scope: string): void => {
    if (seen.has(file)) return;
    seen.add(file);
    const text = readSource(file, options);
    files.push({ path: file, text });
    for (const child of children(file, text, options, scope)) visit(child.target, child.scope);
  };
  const partition = startScope === undefined && extname(start) === ".rs" && options.crates !== false ? cratePartition(start) : null;
  visit(start, startScope ?? (partition !== null && [...partition.crates.values()].includes(start) ? partition.scope : dirname(start)));
  return files;
};

/** 📚️ Lists the root file and every file reachable through its own module mounts (`.rs` `#[path]` declarations and `include!("literal")` items, `.ts`/`.tsx` relative imports below the declaring directory; an included file stays inside the directory of the file that includes it), parent first, children in declaration order, each once. */
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

/** 👪️ Lists every authored `.rs` file below a directory in path order, canonical `🧪️tests` and `🧪️testing` folders excluded unless `tests` is set, for laws about a carved crate family whose members mount each other through Cargo rather than `#[path]`. */
export function listSourceFamily(directory: string | URL, options: { readonly tests?: boolean } = {}): readonly string[] {
  const out: string[] = [];
  const visit = (dir: string): void => {
    for (const entry of readdirSync(dir, { withFileTypes: true }).sort((a, b) => (a.name < b.name ? -1 : a.name > b.name ? 1 : 0))) {
      const path = resolve(dir, entry.name);
      if (entry.isDirectory()) {
        if ((options.tests === true || (entry.name !== TESTS_DIRECTORY && entry.name !== "🧪️testing")) && entry.name !== "node_modules") visit(path);
      } else if (entry.isFile() && extname(entry.name) === ".rs") out.push(path);
    }
  };
  visit(typeof directory === "string" ? resolve(directory) : fileURLToPath(directory));
  return out;
}

/** 🪵️ Reads the authored sources of a carved crate family as one text, files in path order. */
export function readSourceFamilySource(directory: string | URL, options: { readonly tests?: boolean } = {}): string {
  return listSourceFamily(directory, options).map(file => { const text = readFileSync(file, "utf8"); return text.endsWith("\n") ? text : text + "\n"; }).join("");
}
