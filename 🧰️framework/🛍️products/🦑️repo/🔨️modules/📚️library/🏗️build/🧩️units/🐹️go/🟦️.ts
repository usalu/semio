import {mkdirSync, readFileSync, rmSync, writeFileSync} from "node:fs";
import {dirname, join} from "node:path";
import {sha256V1} from "../🔏️key/🟦️.ts";
import {compareCodePointsV1, sortedCodePointsV1, type UnitDependencyV1, type UnitGraphV1, type UnitsPolicyV1, type UnitV1} from "../🕸️graph/🟦️.ts";
import type {ExecuteOutcomeV1, ExecuteRequestV1} from "../🏃️engine/🟦️.ts";

/** 📜️ Parsed `go.mod`: module path, language version, requirements and replacements. */
export type GoModFileV1 = {readonly module: string; readonly go: string; readonly requires: readonly {readonly path: string; readonly version: string}[]; readonly replaces: readonly {readonly from: string; readonly to: string; readonly toVersion: string | null}[]};
const stripLineComment = (line: string): string => {const at = line.indexOf("//"); return (at < 0 ? line : line.slice(0, at)).trim();};
const unquote = (value: string): string => value.length >= 2 && (value[0] === '"' || value[0] === "`") && value[value.length - 1] === value[0] ? value.slice(1, -1) : value;
/** 🧱️ Parses `go.mod` text (single-line and block `require`/`replace`, comments). */
export function parseGoModV1(text: string): GoModFileV1 {
  let module = "", go = "";
  const requires: {path: string; version: string}[] = [], replaces: {from: string; to: string; toVersion: string | null}[] = [];
  let block: string | null = null;
  const directive = (word: string, rest: string): void => {
    const parts = rest.split(/\s+/).filter(Boolean).map(unquote);
    if (word === "module") module = parts[0] ?? "";
    else if (word === "go") go = parts[0] ?? "";
    else if (word === "require" && parts.length >= 2) requires.push({path: parts[0]!, version: parts[1]!});
    else if (word === "replace") {
      const arrow = parts.indexOf("=>");
      if (arrow > 0 && parts[arrow + 1]) replaces.push({from: parts[0]!, to: parts[arrow + 1]!, toVersion: parts[arrow + 2] ?? null});
    }
  };
  for (const raw of text.split(/\r?\n/)) {
    const line = stripLineComment(raw);
    if (!line) continue;
    if (block) {if (line === ")") block = null; else directive(block, line); continue;}
    const open = /^(require|replace|exclude|retract|tool|godebug|ignore)\s*\($/.exec(line);
    if (open) {block = open[1]!; continue;}
    const word = /^(\S+)\s+(.*)$/.exec(line);
    if (word) directive(word[1]!, word[2]!);
  }
  return {module, go, requires, replaces};
}
/** 💼️ Parses `go.work` text: language version and `use` directories. */
export function parseGoWorkV1(text: string): {readonly go: string; readonly use: readonly string[]} {
  let go = "", block = false;
  const use: string[] = [];
  for (const raw of text.split(/\r?\n/)) {
    const line = stripLineComment(raw);
    if (!line) continue;
    if (block) {if (line === ")") block = false; else use.push(unquote(line)); continue;}
    if (/^use\s*\($/.test(line)) {block = true; continue;}
    const single = /^use\s+(\S+)$/.exec(line);
    if (single) {use.push(unquote(single[1]!)); continue;}
    const version = /^go\s+(\S+)$/.exec(line);
    if (version) go = version[1]!;
  }
  return {go, use};
}

/** 🔎️ Facts the scanner reads from the head of one Go file. */
export type GoSourceFactsV1 = {readonly package: string; readonly imports: readonly string[]; readonly constraint: string | null; readonly embeds: readonly string[]};
const isIdentifierStart = (character: string): boolean => /[\p{L}_]/u.test(character);
const isIdentifierPart = (character: string): boolean => /[\p{L}\p{N}_]/u.test(character);
const readGoString = (text: string, at: number): {value: string; end: number} | undefined => {
  const quoteCharacter = text[at];
  if (quoteCharacter === "`") {const end = text.indexOf("`", at + 1); return end < 0 ? undefined : {value: text.slice(at + 1, end), end: end + 1};}
  if (quoteCharacter !== '"') return undefined;
  let value = "", index = at + 1;
  while (index < text.length && text[index] !== '"' && text[index] !== "\n") {
    if (text[index] === "\\") {const next = text[index + 1]; value += next === "n" ? "\n" : next === "t" ? "\t" : next ?? ""; index += 2;} else value += text[index++];
  }
  return text[index] === '"' ? {value, end: index + 1} : undefined;
};
/** 🪵️ Scans the head of a Go file: the leading `//go:build` constraint (the only build constraint syntax: `// +build` lines are not read), the package clause, the import declarations and `//go:embed` patterns. Stops at the first declaration that is not an import. */
export function scanGoSourceV1(text: string): GoSourceFactsV1 {
  let at = 0, constraint: string | null = null;
  const skipSpace = (): void => {
    for (;;) {
      while (at < text.length && /\s/.test(text[at]!)) at++;
      if (text.startsWith("//", at)) {const end = text.indexOf("\n", at); at = end < 0 ? text.length : end + 1; continue;}
      if (text.startsWith("/*", at)) {const end = text.indexOf("*/", at + 2); at = end < 0 ? text.length : end + 2; continue;}
      return;
    }
  };
  skipSpace();
  const header = text.slice(0, at);
  for (const raw of header.split(/\r?\n/)) {
    const built = /^\/\/go:build\s+(.+)$/.exec(raw.trim());
    if (built && constraint === null) constraint = built[1]!.trim();
  }
  let packageName = "";
  if (text.startsWith("package", at) && !isIdentifierPart(text[at + 7] ?? " ")) {
    at += 7; skipSpace();
    const start = at;
    while (at < text.length && isIdentifierPart(text[at]!)) at++;
    packageName = text.slice(start, at);
  }
  const imports: string[] = [];
  const spec = (): boolean => {
    skipSpace();
    if (text[at] === "." || text[at] === "_") at++;
    else if (isIdentifierStart(text[at] ?? "")) while (at < text.length && isIdentifierPart(text[at]!)) at++;
    skipSpace();
    const literal = readGoString(text, at);
    if (!literal) return false;
    imports.push(literal.value); at = literal.end;
    return true;
  };
  for (;;) {
    skipSpace();
    while (text[at] === ";") {at++; skipSpace();}
    if (!text.startsWith("import", at) || isIdentifierPart(text[at + 6] ?? " ")) break;
    at += 6; skipSpace();
    if (text[at] === "(") {
      at++;
      for (;;) {skipSpace(); while (text[at] === ";") {at++; skipSpace();} if (text[at] === ")") {at++; break;} if (!spec()) {while (at < text.length && text[at] !== "\n" && text[at] !== ")") at++; if (at >= text.length) break;}}
    } else if (!spec()) break;
  }
  const embeds: string[] = [];
  if (imports.includes("embed")) for (const match of text.matchAll(/^\/\/go:embed[ \t]+(.+)$/gm)) {
    const rest = match[1]!.trim();
    for (let index = 0; index < rest.length;) {
      if (/\s/.test(rest[index]!)) {index++; continue;}
      const literal = readGoString(rest, index);
      if (literal) {embeds.push(literal.value); index = literal.end;} else {let end = index; while (end < rest.length && !/\s/.test(rest[end]!)) end++; embeds.push(rest.slice(index, end)); index = end;}
    }
  }
  return {package: packageName, imports: sortedCodePointsV1(new Set(imports)), constraint, embeds};
}

/** 🧭️ The build context a package set is planned for. */
export type GoBuildContextV1 = {readonly goos: string; readonly goarch: string; readonly goMinor: number; readonly cgo: boolean; readonly tags: readonly string[]};
/** 🏷️ Whether a build tag holds (go/build `matchTag`: GOOS, GOARCH, compiler gc, cgo, release tags, `unix`, the android/illumos/ios aliases, extra tags). */
export function goTagHoldsV1(name: string, context: GoBuildContextV1, policy: UnitsPolicyV1): boolean {
  if (context.cgo && name === "cgo") return true;
  if (name === context.goos || name === context.goarch || name === "gc") return true;
  if (context.goos === "android" && name === "linux") return true;
  if (context.goos === "illumos" && name === "solaris") return true;
  if (context.goos === "ios" && name === "darwin") return true;
  if (name === "unix" && policy.go.unix.includes(context.goos)) return true;
  const release = /^go1\.(\d+)$/.exec(name);
  if (release) return Number(release[1]) <= context.goMinor;
  return context.tags.includes(name);
}
/** 🧮️ Evaluates a `//go:build` expression (`!`, `&&`, `||`, parentheses). A malformed expression holds false. */
export function goConstraintHoldsV1(expression: string, context: GoBuildContextV1, policy: UnitsPolicyV1): boolean {
  const tokens = expression.match(/\(|\)|!|&&|\|\||[\p{L}\p{N}_.]+/gu) ?? [];
  let at = 0;
  const or = (): boolean => {let value = and(); while (tokens[at] === "||") {at++; const right = and(); value = value || right;} return value;};
  const and = (): boolean => {let value = not(); while (tokens[at] === "&&") {at++; const right = not(); value = value && right;} return value;};
  const not = (): boolean => {if (tokens[at] === "!") {at++; return !not();} return atom();};
  const atom = (): boolean => {
    const token = tokens[at++];
    if (token === "(") {const value = or(); if (tokens[at++] !== ")") throw Error("unbalanced"); return value;}
    if (token === undefined || token === ")" || token === "&&" || token === "||") throw Error("unexpected");
    return goTagHoldsV1(token, context, policy);
  };
  try {const value = or(); return at === tokens.length ? value : false;} catch {return false;}
}
/** ⏸️ go/build `goodOSArchFile`: `name_GOOS.go`, `name_GOARCH.go`, `name_GOOS_GOARCH.go` (before an optional `_test`). */
export function goFileNameHoldsV1(fileName: string, context: GoBuildContextV1, policy: UnitsPolicyV1): boolean {
  let name = fileName.split(".")[0] ?? fileName;
  const first = name.indexOf("_");
  if (first < 0) return true;
  name = name.slice(first);
  let parts = name.split("_");
  if (parts.length > 0 && parts[parts.length - 1] === "test") parts = parts.slice(0, -1);
  const count = parts.length, os = new Set(policy.go.goos), arch = new Set(policy.go.goarch);
  if (count >= 2 && os.has(parts[count - 2]!) && arch.has(parts[count - 1]!)) return goTagHoldsV1(parts[count - 2]!, context, policy) && goTagHoldsV1(parts[count - 1]!, context, policy);
  if (count >= 1 && (os.has(parts[count - 1]!) || arch.has(parts[count - 1]!))) return goTagHoldsV1(parts[count - 1]!, context, policy);
  return true;
}

/** 📦️ What `go list` calls a package, derived from sources. */
export type GoPackageV1 = {
  readonly importPath: string; readonly dir: string; readonly module: string; readonly name: string;
  readonly goFiles: readonly string[]; readonly cgoFiles: readonly string[]; readonly testGoFiles: readonly string[]; readonly xtestGoFiles: readonly string[]; readonly ignoredGoFiles: readonly string[];
  readonly auxiliaryFiles: readonly string[]; readonly imports: readonly string[]; readonly testImports: readonly string[]; readonly xtestImports: readonly string[]; readonly embedPatterns: readonly string[]; readonly testEmbedPatterns: readonly string[];
};
/** 🗂️ The workspace as the planner sees it: every file path and a text reader for `go.mod`, `go.work` and Go files; `realPath` maps a virtual overlay path back to the file that holds its bytes (unit sources name real files). */
export type GoWorkspaceSnapshotV1 = {readonly files: readonly string[]; readonly read: (path: string) => string; readonly realPath?: (path: string) => string};
type GoModuleV1 = {readonly dir: string; readonly file: GoModFileV1};

const joinPath = (...parts: readonly string[]): string => parts.filter(part => part !== "" && part !== ".").join("/");
const normalizeRelative = (path: string): string => {const out: string[] = []; for (const part of path.split("/")) {if (part === "" || part === ".") continue; if (part === "..") out.pop(); else out.push(part);} return out.join("/");};
const embedFiles = (patterns: readonly string[], directory: string, files: ReadonlySet<string>): string[] => {
  const found = new Set<string>();
  for (const raw of patterns) {
    const all = raw.startsWith("all:"), pattern = all ? raw.slice(4) : raw;
    const regex = new RegExp("^" + pattern.split("/").map(segment => segment.replace(/[.+^${}()|\\]/g, "\\$&").replace(/\*/g, "[^/]*").replace(/\?/g, "[^/]")).join("/") + "(/.*)?$", "u");
    const base = directory === "" ? "" : directory + "/";
    for (const file of files) {
      if (!file.startsWith(base)) continue;
      const relative = file.slice(base.length);
      if (!regex.test(relative)) continue;
      const match = regex.exec(relative)!;
      if (match[1] !== undefined && !all && match[1].split("/").some(part => part.startsWith(".") || part.startsWith("_"))) continue;
      found.add(file);
    }
  }
  return sortedCodePointsV1(found);
};

/** 🐹️ Derives the Go packages of a workspace from sources alone (the `go list` oracle checks it; `importPath` lets a repository layout that Go cannot load directly, such as emoji directories merged by an overlay, name its packages): every `go.mod` (a module outside the `use` list of `go.work` is planned as a standalone module), files filtered by name suffix and build constraints, imports from the file heads. */
export function planGoPackagesV1(snapshot: GoWorkspaceSnapshotV1, options: {readonly policy: UnitsPolicyV1; readonly context: GoBuildContextV1; readonly importPath?: (module: string, directory: string) => string}): {readonly packages: readonly GoPackageV1[]; readonly modules: readonly {readonly dir: string; readonly path: string; readonly go: string; readonly workspace: boolean}[]} {
  const {policy, context} = options, fileSet = new Set(snapshot.files);
  const discovered = snapshot.files.filter(path => path === "go.mod" || path.endsWith("/go.mod")).map(path => dirname(path) === "." ? "" : dirname(path));
  const used = fileSet.has("go.work") ? new Set(parseGoWorkV1(snapshot.read("go.work")).use.map(entry => normalizeRelative(entry))) : undefined;
  const moduleDirs = discovered;
  const modules: GoModuleV1[] = sortedCodePointsV1(moduleDirs).map(dir => ({dir, file: parseGoModV1(snapshot.read(joinPath(dir, "go.mod")))})).filter(row => row.file.module !== "");
  const ownerOf = (path: string): GoModuleV1 | undefined => {
    let best: GoModuleV1 | undefined;
    for (const row of modules) if ((row.dir === "" || path.startsWith(row.dir + "/")) && (!best || row.dir.length > best.dir.length)) best = row;
    return best;
  };
  const allModuleDirs = new Set(discovered);
  const groups = new Map<string, {module: GoModuleV1; dir: string; files: string[]}>();
  for (const path of snapshot.files) {
    const module = ownerOf(path);
    if (!module) continue;
    const relative = module.dir === "" ? path : path.slice(module.dir.length + 1), directory = dirname(relative) === "." ? "" : dirname(relative);
    const segments = directory === "" ? [] : directory.split("/");
    if (segments.some(segment => segment.startsWith(".") || segment.startsWith("_") || policy.go.skipDirectories.includes(segment))) continue;
    const absoluteDirectory = joinPath(module.dir, directory);
    let nested = false;
    for (let index = 0, prefix = module.dir; index < segments.length; index++) {prefix = joinPath(prefix, segments[index]!); if (allModuleDirs.has(prefix) && prefix !== module.dir) {nested = true; break;}}
    if (nested) continue;
    const name = path.slice(path.lastIndexOf("/") + 1);
    if (name.startsWith(".") || name.startsWith("_")) continue;
    const extension = name.includes(".") ? name.slice(name.lastIndexOf(".")) : "";
    if (extension !== ".go" && !policy.go.auxiliaryExtensions.includes(extension)) continue;
    const key = module.dir + "\0" + absoluteDirectory;
    if (!groups.has(key)) groups.set(key, {module, dir: absoluteDirectory, files: []});
    groups.get(key)!.files.push(path);
  }
  const packages: GoPackageV1[] = [];
  for (const group of [...groups.values()].sort((a, b) => compareCodePointsV1(a.dir, b.dir))) {
    const relative = group.module.dir === "" ? group.dir : group.dir.slice(group.module.dir.length + 1);
    const importPath = options.importPath ? options.importPath(group.module.file.module, relative) : relative === "" ? group.module.file.module : group.module.file.module + "/" + relative;
    const goFiles: string[] = [], cgoFiles: string[] = [], testGoFiles: string[] = [], xtestGoFiles: string[] = [], ignored: string[] = [], auxiliary: string[] = [];
    const imports = new Set<string>(), testImports = new Set<string>(), xtestImports = new Set<string>(), embed = new Set<string>(), testEmbed = new Set<string>();
    const parsed: {path: string; test: boolean; facts: GoSourceFactsV1}[] = [];
    for (const path of sortedCodePointsV1(group.files)) {
      const base = path.slice(path.lastIndexOf("/") + 1);
      if (!base.endsWith(".go")) {if (goFileNameHoldsV1(base, context, policy)) auxiliary.push(path); continue;}
      if (!goFileNameHoldsV1(base, context, policy)) {ignored.push(path); continue;}
      const facts = scanGoSourceV1(snapshot.read(path));
      const holds = facts.constraint === null || goConstraintHoldsV1(facts.constraint, context, policy);
      if (!holds || (facts.imports.includes("C") && !context.cgo)) {ignored.push(path); continue;}
      parsed.push({path, test: base.endsWith("_test.go"), facts});
    }
    const lead = parsed.find(row => !row.test) ?? parsed[0], name = lead ? (lead.test && lead.facts.package.endsWith("_test") ? lead.facts.package.slice(0, -5) : lead.facts.package) : "";
    for (const row of parsed) {
      const external = row.test && row.facts.package.endsWith("_test");
      if (row.test) {(external ? xtestGoFiles : testGoFiles).push(row.path); for (const item of row.facts.imports) (external ? xtestImports : testImports).add(item); for (const item of row.facts.embeds) testEmbed.add(item);}
      else {(row.facts.imports.includes("C") ? cgoFiles : goFiles).push(row.path); for (const item of row.facts.imports) imports.add(item); for (const item of row.facts.embeds) embed.add(item);}
    }
    if (!goFiles.length && !cgoFiles.length && !testGoFiles.length && !xtestGoFiles.length) continue;
    packages.push({importPath, dir: group.dir, module: group.module.file.module, name, goFiles, cgoFiles, testGoFiles, xtestGoFiles, ignoredGoFiles: ignored, auxiliaryFiles: auxiliary, imports: sortedCodePointsV1(imports), testImports: sortedCodePointsV1(testImports), xtestImports: sortedCodePointsV1(xtestImports), embedPatterns: sortedCodePointsV1(embed), testEmbedPatterns: sortedCodePointsV1(testEmbed)});
  }
  return {packages, modules: modules.map(row => ({dir: row.dir, path: row.file.module, go: row.file.go, workspace: used === undefined || used.has(row.dir)}))};
}

const moduleOfImport = (importPath: string, modules: readonly {dir: string; path: string}[]): {dir: string; path: string} | undefined => {
  let best: {dir: string; path: string} | undefined;
  for (const row of modules) if ((importPath === row.path || importPath.startsWith(row.path + "/")) && (!best || row.path.length > best.path.length)) best = row;
  return best;
};

/** 🕸️ Plans the unit graph of a Go workspace: one library (or binary) unit per package with `interface` edges to the workspace packages it imports, one test unit per package with tests with `implementation` edges. Standard library imports are covered by the toolchain identity, other modules by exact `module@version` externals. */
export function planGoUnitsV1(snapshot: GoWorkspaceSnapshotV1, options: {readonly policy: UnitsPolicyV1; readonly context: GoBuildContextV1; readonly toolchain: string; readonly tests: boolean; readonly importPath?: (module: string, directory: string) => string}): {readonly graph: UnitGraphV1; readonly packages: readonly GoPackageV1[]} {
  const {policy, context} = options, planned = planGoPackagesV1(snapshot, {policy, context, ...(options.importPath ? {importPath: options.importPath} : {})}), files = new Set(snapshot.files);
  const byImport = new Map(planned.packages.map(row => [row.importPath, row] as const)), moduleFiles = new Map(planned.modules.map(row => [row.path, parseGoModV1(snapshot.read(joinPath(row.dir, "go.mod")))] as const));
  const externalIdentity = (owner: string, importPath: string): string | undefined => {
    const candidates = [moduleFiles.get(owner), ...[...moduleFiles.entries()].filter(([path]) => path !== owner).map(([, file]) => file)];
    let best: {path: string; version: string} | undefined, replaced: string | undefined;
    for (const file of candidates) {
      if (!file) continue;
      for (const requirement of file.requires) if ((importPath === requirement.path || importPath.startsWith(requirement.path + "/")) && (!best || requirement.path.length > best.path.length)) {
        best = requirement;
        const replacement = file.replaces.find(row => row.from === requirement.path && (row.toVersion !== null));
        replaced = replacement ? `=>${replacement.to}@${replacement.toVersion}` : undefined;
      }
      if (best) break;
    }
    return best ? `go-module:${best.path}@${best.version}${replaced ?? ""}` : undefined;
  };
  const standard = (importPath: string): boolean => !(importPath.split("/")[0] ?? "").includes(".");
  const units: UnitV1[] = [], real = snapshot.realPath ?? ((path: string): string => path);
  const toolchain = (module: string): UnitV1["toolchain"] => ({id: options.toolchain, parts: [`lang=go${(planned.modules.find(row => row.path === module)?.go) || "unknown"}`, `goos=${context.goos}`, `goarch=${context.goarch}`, `cgo=${context.cgo}`, ...sortedCodePointsV1(context.tags).map(tag => "tag=" + tag)]});
  const env = {CGO_ENABLED: context.cgo ? "1" : "0", GOARCH: context.goarch, GOFLAGS: "-mod=readonly", GOOS: context.goos};
  const resolve = (module: string, imports: readonly string[], use: UnitDependencyV1["use"], dependencies: Map<string, UnitDependencyV1>, externals: Set<string>, self: string): void => {
    for (const item of imports) {
      if (item === "C" || item === "unsafe" || item === self) continue;
      if (byImport.has(item) && byImport.get(item)!.name !== "main") {const id = "go:" + item; const known = dependencies.get(id); if (!known || use === "implementation") dependencies.set(id, {unit: id, use}); continue;}
      const owner = moduleOfImport(item, planned.modules);
      if (owner && !byImport.has(item)) {externals.add(`go-missing:${item}`); continue;}
      if (standard(item)) continue;
      externals.add(externalIdentity(module, item) ?? `go-unresolved:${item}`);
    }
  };
  const compileMemo = new Map<string, boolean>();
  const compiles = (importPath: string): boolean => {
    const known = compileMemo.get(importPath);
    if (known !== undefined) return known;
    compileMemo.set(importPath, false);
    const row = byImport.get(importPath);
    let ok = row !== undefined && row.name !== "main" && row.goFiles.length > 0 && row.cgoFiles.length === 0 && row.auxiliaryFiles.length === 0 && row.embedPatterns.length === 0;
    if (ok) for (const item of row!.imports) {
      if (item === "unsafe" || item === importPath) continue;
      if (byImport.has(item)) {if (!compiles(item)) {ok = false; break;}}
      else if (!standard(item)) {ok = false; break;}
    }
    compileMemo.set(importPath, ok);
    return ok;
  };
  const langOf = (module: string): string => { const parts = (planned.modules.find(row => row.path === module)?.go ?? "").split("."); return parts.length >= 2 ? `go${parts[0]}.${parts[1]}` : "go1.21"; };
  const buildEnv = {...env, SEMIO_GO_MODE: "build", SEMIO_BAZEL_INPUTS: "sources"};
  for (const row of planned.packages) {
    const main = row.name === "main";
    if (row.goFiles.length || row.cgoFiles.length) {
      const dependencies = new Map<string, UnitDependencyV1>(), externals = new Set<string>();
      resolve(row.module, row.imports, "interface", dependencies, externals, row.importPath);
      const libraryEnv = compiles(row.importPath) ? {...env, SEMIO_GO_MODE: "compile", SEMIO_GO_IMPORT_PATH: row.importPath, SEMIO_GO_LANG: langOf(row.module), SEMIO_GO_STD: sortedCodePointsV1(row.imports.filter(item => item !== "unsafe" && !byImport.has(item) && standard(item))).join(",")} : buildEnv;
      const embedded = embedFiles(row.embedPatterns, row.dir, files);
      units.push({
        id: "go:" + row.importPath, language: "go", kind: main ? "binary" : "library", label: row.importPath,
        sources: sortedCodePointsV1([...row.goFiles, ...row.cgoFiles, ...row.auxiliaryFiles].map(real)), inputs: embedded.filter(path => !row.goFiles.includes(path) && !row.cgoFiles.includes(path)).map(real),
        dependencies: [...dependencies.values()].sort((a, b) => compareCodePointsV1(a.unit, b.unit)), externals: sortedCodePointsV1(externals),
        interface: main ? null : {kind: "go-export-data"}, outputs: [main ? "bin" : "pkg.a"], toolchain: toolchain(row.module),
        action: {argv: ["go", "-C", planned.modules.find(module => module.path === row.module)!.dir || ".", "build", "-trimpath", "-o", main ? "{out}/bin" : "{out}/pkg.a", row.importPath], env: libraryEnv},
      });
    }
    if (options.tests && (row.testGoFiles.length || row.xtestGoFiles.length)) {
      const dependencies = new Map<string, UnitDependencyV1>(), externals = new Set<string>();
      if (row.goFiles.length || row.cgoFiles.length) dependencies.set("go:" + row.importPath, {unit: "go:" + row.importPath, use: "implementation"});
      resolve(row.module, [...row.testImports, ...row.xtestImports], "implementation", dependencies, externals, row.importPath);
      const testdata = snapshot.files.filter(path => path.startsWith(row.dir === "" ? "testdata/" : row.dir + "/testdata/"));
      const embedded = embedFiles(row.testEmbedPatterns, row.dir, files);
      units.push({
        id: `go:${row.importPath}#test`, language: "go", kind: "test", label: row.importPath + " (test)",
        sources: sortedCodePointsV1([...row.testGoFiles, ...row.xtestGoFiles].map(real)), inputs: sortedCodePointsV1(new Set([...testdata, ...embedded].map(real))),
        dependencies: [...dependencies.values()].sort((a, b) => compareCodePointsV1(a.unit, b.unit)), externals: sortedCodePointsV1(externals), interface: null, outputs: [], toolchain: toolchain(row.module),
        action: {argv: ["go", "-C", planned.modules.find(module => module.path === row.module)!.dir || ".", "test", "-trimpath", "-count=1", row.importPath], env: buildEnv},
      });
    }
  }
  return {graph: {version: 1, units}, packages: planned.packages};
}

/** 🧩️ Applies a Go compiler overlay (`{Replace: {virtual: real}}` as the repository's canonical layout writes it: member directories contribute their implementation file to the parent package, case leaves become its tests) to a workspace snapshot: the real files that only exist through the overlay disappear, their virtual paths appear, reads and `realPath` follow the overlay. Paths in `replace` are absolute and below `root`; entries outside it are ignored. */
export function applyGoOverlayV1(snapshot: {readonly files: readonly string[]; readonly read: (path: string) => string}, root: string, replace: Readonly<Record<string, string>>, excludedDirectories: readonly string[] = []): GoWorkspaceSnapshotV1 {
  const prefix = root.endsWith("/") ? root : root + "/", relative = (path: string): string | undefined => path.startsWith(prefix) ? path.slice(prefix.length) : undefined;
  const virtualToReal = new Map<string, string>(), realSources = new Set<string>();
  for (const [virtual, real] of Object.entries(replace)) {const from = relative(virtual), to = relative(real); if (from !== undefined && to !== undefined) {virtualToReal.set(from, to); realSources.add(to);}}
  const excluded = new Set(excludedDirectories), moduleDirs = snapshot.files.filter(path => path === "go.mod" || path.endsWith("/go.mod")).map(path => path.includes("/") ? path.slice(0, path.lastIndexOf("/")) : "");
  const below = (path: string): string[] => {let best = ""; for (const dir of moduleDirs) if ((dir === "" || path.startsWith(dir + "/")) && dir.length >= best.length) best = dir; return path.slice(best === "" ? 0 : best.length + 1).split("/").slice(0, -1);};
  const files = [...snapshot.files.filter(path => !realSources.has(path) && !(path.endsWith(".go") && below(path).some(segment => excluded.has(segment)))), ...virtualToReal.keys()];
  return {files, read: path => snapshot.read(virtualToReal.get(path) ?? path), realPath: path => virtualToReal.get(path) ?? path};
}

/** 🗜️ Splits an `ar` archive into its members (name to bytes). */
export function goArchiveMembersV1(archive: Uint8Array): ReadonlyMap<string, Uint8Array> {
  const magic = new TextDecoder().decode(archive.subarray(0, 8));
  if (magic !== "!<arch>\n") throw Error("not an ar archive");
  const members = new Map<string, Uint8Array>(), decoder = new TextDecoder();
  for (let at = 8; at + 60 <= archive.length;) {
    const name = decoder.decode(archive.subarray(at, at + 16)).trim().replace(/\/$/, ""), size = Number(decoder.decode(archive.subarray(at + 48, at + 58)).trim());
    at += 60; members.set(name, archive.subarray(at, at + size)); at += size + (size & 1);
  }
  return members;
}
/** 🔏️ The interface digest of a compiled Go package: SHA-256 of the export data member `__.PKGDEF` without its `build id` line (the build id changes with every output). */
export function goExportDigestV1(archive: Uint8Array): string {
  const member = goArchiveMembersV1(archive).get("__.PKGDEF");
  if (!member) throw Error("archive has no __.PKGDEF member");
  const text = Buffer.from(member), lines: Buffer[] = [];
  for (let start = 0; start < text.length;) {const end = text.indexOf(0x0a, start), stop = end < 0 ? text.length : end + 1; lines.push(text.subarray(start, stop)); start = stop;}
  return sha256V1(Buffer.concat(lines.filter((line, index) => !(index < 4 && line.toString("latin1").startsWith("build id \"")))));
}

/** 🏭️ The process effect of the Go executor; the repository wires its owned child-process runner here. */
export type GoRunnerV1 = (argv: readonly string[], options: {readonly cwd: string; readonly env: Readonly<Record<string, string>>; readonly signal: AbortSignal}) => Promise<{readonly code: number; readonly output: string}>;
/** ▶️ Executes a Go unit. A pure-Go library in `compile` mode (policy of the planner: no cgo, assembly, embed or outside module) is compiled directly with `go tool compile`: its interface artifact is the export-data archive of the package plus those of its workspace dependencies, which the dependents read from their interface files instead of the dependency sources (this is what gives Bazel an early cutoff for Go); the implementation is the link object. Everything else, and every unit when no dependency artifacts exist, runs the unit's own `go build`/`go test` command. A test unit passes when the command exits zero; its output digest is only the verdict so timing lines never move the output closure. */
export function goUnitExecutorV1(options: {readonly workspace: string; readonly outputRoot: string; readonly run: GoRunnerV1; readonly overlay?: string}): (request: ExecuteRequestV1) => Promise<ExecuteOutcomeV1> {
  return async request => {
    const {unit} = request, out = join(options.outputRoot, sha256V1(unit.id).slice(0, 24));
    rmSync(out, {recursive: true, force: true}); mkdirSync(out, {recursive: true});
    const started = performance.now(), elapsed = (): number => Math.round(performance.now() - started);
    const dependencies = request.dependencies.filter(row => row.use === "interface");
    if (unit.action.env.SEMIO_GO_MODE === "compile" && unit.kind === "library" && dependencies.every(row => row.artifact)) {
      const files: Record<string, string> = {}, std = new Set((unit.action.env.SEMIO_GO_STD ?? "").split(",").filter(Boolean));
      for (const row of dependencies) {const artifact = JSON.parse(row.artifact!) as {files: Record<string, string>; std: string[]}; Object.assign(files, artifact.files); for (const item of artifact.std) std.add(item);}
      const lines: string[] = [];
      for (const [importPath, encoded] of Object.entries(files)) {const path = join(out, "dep-" + sha256V1(importPath).slice(0, 16) + ".x"); writeFileSync(path, Buffer.from(encoded, "base64")); lines.push(`packagefile ${importPath}=${path}`);}
      if (std.size) {
        const listed = await options.run(["go", "list", "-export", "-deps", "-f", "{{if .Export}}packagefile {{.ImportPath}}={{.Export}}{{end}}", ...sortedCodePointsV1(std)], {cwd: out, env: {GOFLAGS: "", GO111MODULE: "off", GOWORK: "off", ...unit.action.env}, signal: request.signal});
        if (listed.code !== 0) return {ok: false, diagnostics: listed.output.trim(), outputs: sha256V1("failed:" + listed.output), interface: null, milliseconds: elapsed()};
        for (const line of listed.output.split("\n")) if (line.startsWith("packagefile ")) lines.push(line.trim());
      }
      const config = join(out, "importcfg"), iface = join(out, "pkg.x"), object = join(out, "pkg.a");
      writeFileSync(config, sortedCodePointsV1(new Set(lines)).join("\n") + "\n");
      const sources = unit.sources.filter(path => path.endsWith(".go")), importPath = unit.action.env.SEMIO_GO_IMPORT_PATH!;
      const compiled = await options.run(["go", "tool", "compile", "-p", importPath, "-importcfg", config, "-pack", `-lang=${unit.action.env.SEMIO_GO_LANG ?? "go1.21"}`, `-trimpath=${options.workspace}`, "-complete", "-o", iface, "-linkobj", object, ...sources], {cwd: options.workspace, env: unit.action.env, signal: request.signal});
      if (compiled.code !== 0) return {ok: false, diagnostics: compiled.output.trim(), outputs: sha256V1("failed:" + compiled.output), interface: null, milliseconds: elapsed()};
      const exported = readFileSync(iface);
      files[importPath] = exported.toString("base64");
      return {ok: true, diagnostics: compiled.output.trim(), outputs: sha256V1(readFileSync(object)), interface: goExportDigestV1(exported), artifact: JSON.stringify({files: Object.fromEntries(Object.entries(files).sort(([a], [b]) => compareCodePointsV1(a, b))), std: sortedCodePointsV1(std)}), milliseconds: elapsed()};
    }
    const base = unit.action.argv.map(part => part.replace("{out}", out).replace("{unit}", unit.id)), verb = base.indexOf("build") >= 0 ? base.indexOf("build") : base.indexOf("test");
    const argv = options.overlay && verb > 0 ? [...base.slice(0, verb + 1), `-overlay=${options.overlay}`, ...base.slice(verb + 1)] : base;
    const result = await options.run(argv, {cwd: options.workspace, env: unit.action.env, signal: request.signal}), milliseconds = elapsed();
    if (unit.kind === "test") return {ok: result.code === 0, diagnostics: result.code === 0 ? "" : result.output.trim(), outputs: sha256V1(result.code === 0 ? "ok" : "fail"), interface: null, milliseconds};
    if (result.code !== 0) return {ok: false, diagnostics: result.output.trim(), outputs: sha256V1("failed:" + result.output), interface: null, milliseconds};
    const artifact = readFileSync(join(out, unit.outputs[0]!));
    return {ok: true, diagnostics: result.output.trim(), outputs: sha256V1(artifact), interface: unit.interface ? goExportDigestV1(artifact) : null, milliseconds};
  };
}
/** 📝️ Writes text for tests and tools that materialise a snapshot into a directory. */
export function writeSnapshotV1(root: string, files: Readonly<Record<string, string>>): void {
  for (const [path, text] of Object.entries(files)) {const target = join(root, path); mkdirSync(dirname(target), {recursive: true}); writeFileSync(target, text);}
}
