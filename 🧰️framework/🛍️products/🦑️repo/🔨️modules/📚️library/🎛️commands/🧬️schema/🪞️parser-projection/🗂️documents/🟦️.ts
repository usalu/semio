/** 🗂️ Finds the schema modules under a scope, their normative documents and exports, and reads and writes the leaf sources the projection edits.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts
 */
import { spawnSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync, writeFileSync, type Dirent } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
import { EXPORT_NAME, asDict, codePointOrder, isDict, type Dict } from "../🔡️json-text/🟦️.ts";
import { listModuleTree } from "../../../../../../../../🔨️modules/📚️compiler/📦️packages/🟦️typescript/📖️syntax/🌳️module-tree/🟦️.ts";
import { PARSER_DIRECTORY } from "../📍️parser-files/🟦️.ts";
import { Owner, type SourceReader } from "../🧾️parser-render/🟦️.ts";

//#region 🔖️Leaves
/** 🍃️ The leaf file name of every projected format. */
export const LEAF = { "🦀️rust": "🦀️.rs", "🟦️typescript": "🟦️.ts", "🔗️graphql": "🔗️.graphql", "🔣️jsonschema": "🔣️.json", "🛰️protobuf": "🛰️.proto" } as const;

/** 🍃️ The projected formats, in leaf order. */
export const FORMATS = Object.keys(LEAF);

/** 🍃️ The facet directories a schema module nests its documents in. */
const FACET_DIRS = new Set(["🔺️diff", "📸️snapshot", "💡️inferences", "🧬️mutations", "📝️text", "💾️binary"]);

const SCHEMA_DIR = "🧬️schema";

/** 🗺️ The scopes whose schema documents are addressable by `$id`: every plugin and app contract, every hub composition, and the framework (a reference into the framework calls a parser its referrer already imports; it never generates one there). */
export const INDEXED_SCOPES: readonly string[] = ["✏️s", "🌎️hub", "🧰️framework"];

const FRAMEWORK_SCOPE = "🧰️framework";

/** 🗂️ Folders below a `🧬️schema` folder whose `🔣️.json` is no schema document of a facet. */
const NON_FACET_DIRS = new Set(["🧪️tests", "📦️packages", "🧾️descriptor", "🧬️mutations"]);

const SKIPPED_DIRS = new Set(["node_modules", ".venv", ".git", "target", "🎯️target", "⚡️cache"]);

/** 🧭️ A repository-relative path with forward slashes. */
export const portable = (root: string, path: string): string => relative(root, path).split(sep).join("/");

/** 📚️ The leaf sources of one run: each file is read once, and a file written with CRLF line ends keeps them. `read` and `write` address the leaf file itself; `tree` lists the leaf followed by the child modules it imports and the `🚪️parsers` file next to each of them (nothing imports a parser file, so the folder name is what finds it), and `declarations` is their text, so a TypeScript leaf split into child modules (types and parsers) still declares everything the unsplit leaf declared. */
export class SourceStore implements SourceReader {
  private readonly texts = new Map<string, string>();
  private readonly crlf = new Set<string>();
  private readonly trees = new Map<string, string>();
  private readonly listings = new Map<string, readonly string[]>();
  private readonly holdings = new Map<string, readonly string[]>();

  read(path: string): string {
    const cached = this.texts.get(path);
    if (cached !== undefined) return cached;
    let text = "";
    try {
      text = readFileSync(path, "utf8");
    } catch {
      text = "";
    }
    if (text.includes("\r")) {
      this.crlf.add(path);
      text = text.replace(/\r\n?/gu, "\n");
    }
    this.texts.set(path, text);
    return text;
  }

  tree(path: string): readonly string[] {
    const cached = this.listings.get(path);
    if (cached !== undefined) return cached;
    let files: readonly string[] = [path];
    if (path.endsWith(LEAF["🟦️typescript"])) {
      try {
        files = listModuleTree(path);
      } catch {
        files = [path];
      }
      const parserFiles = files.map((file) => join(dirname(file), PARSER_DIRECTORY, LEAF["🟦️typescript"])).filter((file) => !files.includes(file) && existsSync(file));
      files = [...files, ...parserFiles];
    }
    this.listings.set(path, files);
    return files;
  }

  /** 🍃️ The files of a leaf's module tree that belong to its own document: the tree without the leaves of other schema documents (a folder with a normative document) and everything only they import. */
  owned(path: string): readonly string[] {
    const cached = this.holdings.get(path);
    if (cached !== undefined) return cached;
    const files = this.tree(path);
    const foreign = files.slice(1).filter((file) => existsSync(join(dirname(file), LEAF["🔣️jsonschema"])));
    const dropped = new Set(foreign.flatMap((file) => this.tree(file)));
    const own = foreign.length === 0 ? files : files.filter((file, at) => at === 0 || !dropped.has(file));
    this.holdings.set(path, own);
    return own;
  }

  /** 🍃️ The text of the files of a leaf's own document (see `owned`): where its parsers are defined, imported and called. */
  parsers(path: string): string {
    return this.owned(path).map((file) => this.read(file)).join("\n");
  }

  declarations(path: string): string {
    const cached = this.trees.get(path);
    if (cached !== undefined) return cached;
    const tree = this.tree(path).map((file) => this.read(file)).join("\n");
    this.trees.set(path, tree);
    return tree;
  }

  write(path: string, text: string): void {
    this.read(path);
    writeFileSync(path, this.crlf.has(path) ? text.replace(/\n/gu, "\r\n") : text);
  }
}
//#endregion 🔖️Leaves

//#region 🔖️Discovery
const entriesOf = (directory: string): Dirent[] => {
  try {
    return readdirSync(directory, { withFileTypes: true });
  } catch {
    return [];
  }
};

/** 🔎️ The tracked and untracked-but-not-ignored files under `scopes`, or `undefined` when `root` is no git work tree. A tree of its own that git ignores is walked instead. */
function versionedFiles(root: string, scopes: readonly string[]): string[] | undefined {
  const listed = spawnSync("git", ["-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", ...scopes], { cwd: root, encoding: "utf8", maxBuffer: 1 << 29 });
  return listed.status === 0 ? listed.stdout.split("\0").filter((path) => path !== "") : undefined;
}

/** 🔎️ Every normative document and package manifest below `scope` found by walking the directories, for a tree that is no git work tree. */
function walkedDocuments(root: string, scope: string): string[] {
  const found: string[] = [];
  const walk = (directory: string): void => {
    for (const entry of entriesOf(directory)) {
      if (entry.isDirectory() && !SKIPPED_DIRS.has(entry.name)) walk(join(directory, entry.name));
      else if (!entry.isDirectory() && (entry.name === LEAF["🔣️jsonschema"] || entry.name === "package.json")) found.push(portable(root, join(directory, entry.name)));
    }
  };
  walk(resolve(root, scope));
  return found;
}

/** 🗂️ The schema modules of a tree and their normative documents. A module is a `🧬️schema` folder with a normative document, outside mutation folders; its documents are its own and one per nested facet folder, plus the documents of any other folder that states an `$id` next to a TypeScript leaf (`📍️feature`, `📝️definition`, …). */
export class SchemaCatalog {
  private readonly documentsByModule = new Map<string, Set<string>>();
  private readonly extras: string[] = [];
  private readonly promoted = new Set<string>();
  readonly manifests: string[] = [];

  constructor(root: string, scopes: readonly string[] = INDEXED_SCOPES) {
    const leaf = LEAF["🔣️jsonschema"];
    const listed = versionedFiles(root, scopes);
    const files = listed !== undefined && listed.some((file) => file.endsWith(`/${leaf}`)) ? listed : scopes.flatMap((scope) => walkedDocuments(root, scope));
    for (const file of files) {
      if (file.endsWith("/package.json") && file.startsWith(FRAMEWORK_SCOPE)) this.manifests.push(file);
      if (!file.endsWith(`/${leaf}`)) continue;
      const directories = file.split("/").slice(0, -1);
      const at = directories.findLastIndex((directory, position) => directory === SCHEMA_DIR && directories.slice(position + 1).every((facet) => FACET_DIRS.has(facet)));
      if (at < 0) {
        const schemaAt = directories.lastIndexOf(SCHEMA_DIR);
        const rest = directories.slice(schemaAt + 1);
        if (schemaAt >= 0 ? !rest.some((directory) => NON_FACET_DIRS.has(directory)) : directories[0] === FRAMEWORK_SCOPE && !directories.some((directory) => NON_FACET_DIRS.has(directory))) this.extras.push(file);
        continue;
      }
      const module = directories.slice(0, at + 1).join("/");
      if (`${module}/`.includes("/🧬️mutations/")) continue;
      if (!this.documentsByModule.has(module)) this.documentsByModule.set(module, new Set());
      this.documentsByModule.get(module)!.add(file);
    }
    for (const [module, documents] of this.documentsByModule) if (!documents.has(`${module}/${leaf}`)) this.documentsByModule.delete(module);
    for (const file of this.extras) {
      const directories = file.split("/").slice(0, -1);
      const module = directories.slice(0, directories.lastIndexOf(SCHEMA_DIR) + 1).join("/");
      if (!this.documentsByModule.has(module) || !existsSync(join(root, ...directories, LEAF["🟦️typescript"])) || typeof readDocument(root, file)?.$id !== "string") continue;
      this.documentsByModule.get(module)!.add(file);
      this.promoted.add(file);
    }
  }

  /** 🔎️ The schema documents in folders of a module that are no facet the projection knows (`📍️feature`, `📝️definition`, …) and that no module claims: addressable by reference, never projected. */
  extraDocuments(): readonly string[] {
    return this.extras.filter((file) => !this.promoted.has(file));
  }

  /** 🔎️ Every module at or below `under`, in code-point order. */
  modules(under: string): string[] {
    const prefix = under.endsWith("/") ? under : `${under}/`;
    return [...this.documentsByModule.keys()].filter((module) => module === under || module.startsWith(prefix)).sort(codePointOrder);
  }

  /** 🔎️ The normative documents of one module, its own first. */
  documents(module: string): string[] {
    const own = `${module}/${LEAF["🔣️jsonschema"]}`;
    return [own, ...[...(this.documentsByModule.get(module) ?? [])].filter((path) => path !== own).sort(codePointOrder)];
  }
}

/** 🔎️ The facet folder (relative to its module) that a document path belongs to; empty for the module's own document. */
export const facetOf = (module: string, path: string): string => {
  const rest = path.slice(module.length + 1);
  const cut = rest.lastIndexOf("/");
  return cut < 0 ? "" : rest.slice(0, cut);
};

/** 🍃️ The absolute path of the leaf a document projects into. */
export const leafPath = (root: string, module: string, facet: string, format: keyof typeof LEAF): string => join(root, module, facet, LEAF[format]);

const exactIntegers = (_key: string, value: unknown, context?: { readonly source?: string }): unknown =>
  typeof value === "number" && Number.isInteger(value) && !Number.isSafeInteger(value) && context?.source !== undefined && /^-?[0-9]+$/u.test(context.source) ? BigInt(context.source) : value;

/** 📥️ A parsed schema document (integers beyond 2^53 stay exact as `bigint`), or `undefined` when the file is missing, malformed or not an object. */
export function readDocument(root: string, path: string): Dict | undefined {
  let text: string;
  try {
    text = readFileSync(join(root, path), "utf8");
  } catch {
    return undefined;
  }
  try {
    const parsed: unknown = JSON.parse(text, exactIntegers as Parameters<typeof JSON.parse>[1]);
    return isDict(parsed) ? parsed : undefined;
  } catch (error) {
    if (error instanceof SyntaxError) return undefined;
    throw error;
  }
}

/** 📤️ The exports a document states: its root object (named by `title`) and every `$defs` or `definitions` entry named like an export. */
export function exportsOf(document: Dict): Map<string, Dict> {
  const found = new Map<string, Dict>();
  const root = document.title;
  const rootObject = document.type === "object" || (!Object.hasOwn(document, "type") && isDict(document.properties));
  if (typeof root === "string" && rootObject && EXPORT_NAME.test(root)) found.set(root, document);
  for (const container of [document.$defs, document.definitions]) {
    for (const [name, definition] of Object.entries(asDict(container))) {
      if (EXPORT_NAME.test(name) && !found.has(name)) found.set(name, asDict(definition));
    }
  }
  return found;
}

const urlPath = (identifier: string): string => {
  const withoutFragment = identifier.split("#")[0]!;
  const withoutScheme = withoutFragment.replace(/^[A-Za-z][A-Za-z0-9+.-]*:/u, "");
  const withoutAuthority = withoutScheme.startsWith("//") ? withoutScheme.slice(2).replace(/^[^/?]*/u, "") : withoutScheme;
  return withoutAuthority.split("?")[0]!;
};

/** 🏷️ `https://…/s/stdio/csv/rfc4180/any/diff.json` to `stdioCsvRfc4180AnyDiffGuard`: the document's own `$id` path names its guard helpers, so two modules compiled into one program never collide. */
export function helperPrefix(documentId: unknown): string {
  if (typeof documentId !== "string") return "schemaGuard";
  const path = urlPath(documentId).replace(/\.json$/u, "");
  let parts = path.split("/").filter((part) => part !== "").map((part) => part.replace(/[^A-Za-z0-9]/gu, ""));
  if (parts.length > 2 && ["s", "app"].includes(parts[0]!)) parts = parts.slice(1);
  if (parts.length > 1 && parts.at(-1) === "schema") parts = parts.slice(0, -1);
  if (parts.length === 0 || parts[0] === "") return "schemaGuard";
  const head = parts[0]!.slice(0, 1).toLowerCase() + parts[0]!.slice(1);
  return `${head}${parts.slice(1).map((part) => part.slice(0, 1).toUpperCase() + part.slice(1)).join("")}Guard`;
}

/** 📦️ The module specifier under which each leaf of the framework is imported across packages: a package entry file (`./modules/store/child/schema` of `@semio-tech/framework-os`) names itself and every file it re-exports, so the real leaf behind the entry is found. */
export function frameworkSpecifiers(root: string, manifests: readonly string[]): Map<string, string> {
  const found = new Map<string, string>();
  for (const manifestPath of manifests) {
    let manifest: unknown;
    try {
      manifest = JSON.parse(readFileSync(join(root, manifestPath), "utf8"));
    } catch {
      continue;
    }
    if (!isDict(manifest) || typeof manifest.name !== "string" || !isDict(manifest.exports)) continue;
    const directory = resolve(root, dirname(manifestPath));
    for (const [key, target] of Object.entries(manifest.exports)) {
      if (typeof target !== "string") continue;
      const specifier = key === "." ? manifest.name : `${manifest.name}/${key.replace(/^\.\//u, "")}`;
      const entry = resolve(directory, target);
      if (!found.has(entry)) found.set(entry, specifier);
      let text = "";
      try {
        text = readFileSync(entry, "utf8");
      } catch {
        continue;
      }
      for (const reexport of text.matchAll(/\bfrom\s+"(\.[^"]+)"/gu)) {
        const real = resolve(dirname(entry), reexport[1]!);
        if (!found.has(real)) found.set(real, specifier);
      }
    }
  }
  return found;
}

/** 🗺️ Every schema document of the indexed scopes, by `$id`, so a `$ref` into another document finds that document's TypeScript leaf. */
export function buildOwnerIndex(root: string, catalog: SchemaCatalog): Map<string, Owner> {
  const index = new Map<string, Owner>();
  const specifiers = frameworkSpecifiers(root, catalog.manifests);
  for (const module of INDEXED_SCOPES.flatMap((scope) => catalog.modules(scope))) {
    for (const path of catalog.documents(module)) {
      const document = readDocument(root, path);
      if (document === undefined) continue;
      const exports = exportsOf(document);
      const title = typeof document.title === "string" && exports.has(document.title) ? document.title : undefined;
      const leaf = leafPath(root, module, facetOf(module, path), "🟦️typescript");
      const shared = module.startsWith(FRAMEWORK_SCOPE);
      const owner = new Owner(leaf, new Set(exports.keys()), title, shared, () => readDocument(root, path), shared ? specifiers.get(leaf) : undefined);
      if (typeof document.$id === "string") index.set(document.$id, owner);
      index.set(`file:${resolve(root, path)}`, owner);
    }
  }
  for (const path of catalog.extraDocuments()) {
    const document = readDocument(root, path);
    if (document === undefined) continue;
    const exports = exportsOf(document);
    const title = typeof document.title === "string" && exports.has(document.title) ? document.title : undefined;
    const leaf = join(root, path.slice(0, path.lastIndexOf("/")), LEAF["🟦️typescript"]);
    const shared = path.startsWith(FRAMEWORK_SCOPE);
    const owner = new Owner(leaf, new Set(exports.keys()), title, shared, () => readDocument(root, path), shared ? specifiers.get(leaf) : undefined);
    if (typeof document.$id === "string") index.set(document.$id, owner);
    index.set(`file:${resolve(root, path)}`, owner);
  }
  return index;
}
//#endregion 🔖️Discovery
