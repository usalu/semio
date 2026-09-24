import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { readFileSync, realpathSync } from "node:fs";
import { dirname, extname, join, relative } from "node:path";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";
import ts from "typescript";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";

//#region 🧬️Contract
/** ✂️ The declared strip-only surface: config specifiers that mark a vite/vitest config, the package entries those configs
 * externalize (Node then loads them without a bundler), and the syntax vectors both oracles must judge alike. */
interface NodeNativeTypeScriptFixture {
  readonly schemaVersion: 1;
  readonly configImportSpecifiers: readonly string[];
  readonly nativeEntries: readonly string[];
  readonly vectors: readonly { readonly name: string; readonly source: string; readonly stripOnly: boolean }[];
}

const root = getWorkspaceRoot();
const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/✂️node-native-typescript/🔣️.json"), "utf8")) as NodeNativeTypeScriptFixture;
const schema = JSON.parse(readFileSync(join(import.meta.dir, "../../🧬️schema/✂️node-native-typescript/🔣️.json"), "utf8"));
const MODULE_EXTENSIONS = new Set([".ts", ".tsx", ".mts", ".js", ".mjs"]);
const STRIP_ONLY_CODES = new Set([1294, 1484, 1485]);
//#endregion 🧬️Contract

//#region 🔍️Surface
/** 🔗️ The specifiers Node's strip-only loader executes: every import or re-export declaration that is not `type`-only
 * (Node keeps an import whose names are all types, unlike a bundler). */
function runtimeSpecifiers(file: string): string[] {
  const source = ts.createSourceFile(file, readFileSync(file, "utf8"), ts.ScriptTarget.Latest, false, extname(file) === ".tsx" ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  return source.statements.flatMap((statement) => {
    if (ts.isImportDeclaration(statement) && !statement.importClause?.isTypeOnly) return [(statement.moduleSpecifier as ts.StringLiteral).text];
    if (ts.isExportDeclaration(statement) && !statement.isTypeOnly && statement.moduleSpecifier) return [(statement.moduleSpecifier as ts.StringLiteral).text];
    return [];
  }).filter((specifier) => !specifier.startsWith("node:") && !specifier.startsWith("bun:"));
}

/** 📍️ Resolves one specifier the way the loader will, answering `null` for installed third-party packages. */
function repositoryTarget(specifier: string, from: string): string | null {
  const resolved = realpathSync(Bun.resolveSync(specifier, dirname(from)));
  return resolved.startsWith(root + "/") && !resolved.includes("/node_modules/") ? resolved : null;
}

/** 📦️ Workspace package entries a vite/vitest config externalizes: the config bundler inlines its relative graph and
 * leaves every bare specifier to Node, so each one that resolves inside the repository is loaded strip-only. */
function configExternalizedEntries(): string[] {
  const listed = spawnSync("git", ["ls-files", "-z", "--", "*.ts", "*.mts"], { cwd: root, encoding: "utf8", maxBuffer: 256 * 1024 * 1024 });
  expect(listed.status, listed.stderr).toBe(0);
  const markers = fixture.configImportSpecifiers.map((specifier) => `from "${specifier}"`);
  const configs = listed.stdout.split("\0").filter(Boolean).map((path) => join(root, path)).filter((path) => {
    try { const text = readFileSync(path, "utf8"); return markers.some((marker) => text.includes(marker)); } catch { return false; }
  });
  const entries = new Set<string>();
  for (const config of configs) {
    const bundled = new Set<string>(), queue = [realpathSync(config)];
    while (queue.length) {
      const file = queue.pop()!;
      if (bundled.has(file) || !MODULE_EXTENSIONS.has(extname(file))) continue;
      bundled.add(file);
      for (const specifier of runtimeSpecifiers(file)) {
        const target = repositoryTarget(specifier, file);
        if (target === null) continue;
        if (specifier.startsWith(".") || specifier.startsWith("/")) queue.push(target);
        else entries.add(relative(root, target).replaceAll("\\", "/"));
      }
    }
  }
  return [...entries].sort();
}

/** 🌳️ The static runtime closure Node parses when it imports one entry, stopping at installed packages. */
function nativeClosure(entry: string): string[] {
  const closure = new Set<string>(), queue = [realpathSync(join(root, entry))];
  while (queue.length) {
    const file = queue.pop()!;
    if (closure.has(file) || !MODULE_EXTENSIONS.has(extname(file))) continue;
    closure.add(file);
    for (const specifier of runtimeSpecifiers(file)) {
      const target = repositoryTarget(specifier, file);
      if (target !== null) queue.push(target);
    }
  }
  return [...closure].sort();
}

/** 🧾️ TypeScript's own verdict under `erasableSyntaxOnly` + `verbatimModuleSyntax`: every construct Node cannot strip, and
 * every value import of a type-only binding (which Node would keep and fail to link). */
function stripOnlyFindings(files: readonly string[], host?: ts.CompilerHost): string[] {
  const program = ts.createProgram(files, { noEmit: true, erasableSyntaxOnly: true, verbatimModuleSyntax: true, noResolve: true, noLib: true, types: [], target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, resolveJsonModule: true, jsx: ts.JsxEmit.ReactJSX, skipLibCheck: true }, host);
  return files.flatMap((file) => {
    const source = program.getSourceFile(file)!;
    return [...program.getSyntacticDiagnostics(source), ...program.getSemanticDiagnostics(source)].filter((diagnostic) => STRIP_ONLY_CODES.has(diagnostic.code)).map((diagnostic) => {
      const { line } = source.getLineAndCharacterOfPosition(diagnostic.start ?? 0);
      return `${relative(root, file).replaceAll("\\", "/")}:${line + 1} TS${diagnostic.code}`;
    });
  });
}

/** 🟢️ Node's own answer for one source: its type stripper accepts it or refuses it. */
function nodeStripVerdicts(sources: readonly string[]): boolean[] {
  const script = 'import { stripTypeScriptTypes } from "node:module"; import { readFileSync } from "node:fs"; const sources = JSON.parse(readFileSync(0, "utf8")); process.stdout.write(JSON.stringify(sources.map((source) => { try { stripTypeScriptTypes(source); return true; } catch { return false; } })));';
  const run = spawnSync("node", ["--no-warnings", "--input-type=module", "-e", script], { input: JSON.stringify(sources), encoding: "utf8" });
  expect(run.status, run.stderr).toBe(0);
  return JSON.parse(run.stdout) as boolean[];
}
//#endregion 🔍️Surface

//#region 🧪️Laws
test("strip-only vectors: TypeScript's erasableSyntaxOnly and Node's type stripper judge every vector alike", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  for (const invalid of [{ ...fixture, extra: true }, { ...fixture, schemaVersion: 2 }, { ...fixture, nativeEntries: ["/absolute.ts"] }, { ...fixture, vectors: [{ name: "Bad Name", source: "x", stripOnly: true }, fixture.vectors[0]] }]) expect(validate(invalid)).toBe(false);
  const node = nodeStripVerdicts(fixture.vectors.map((vector) => vector.source));
  const files = new Map(fixture.vectors.map((vector) => [join(root, `✂️${vector.name}.ts`), vector.source]));
  const host = ts.createCompilerHost({});
  const readFile = host.readFile.bind(host), fileExists = host.fileExists.bind(host), getSourceFile = host.getSourceFile.bind(host);
  host.readFile = (path) => files.get(path) ?? readFile(path);
  host.fileExists = (path) => files.has(path) || fileExists(path);
  host.getSourceFile = (path, language) => files.has(path) ? ts.createSourceFile(path, files.get(path)!, language, true, ts.ScriptKind.TS) : getSourceFile(path, language);
  const findings = stripOnlyFindings([...files.keys()], host);
  fixture.vectors.forEach((vector, index) => {
    expect(node[index], `node: ${vector.name}`).toBe(vector.stripOnly);
    expect(findings.some((finding) => finding.startsWith(`✂️${vector.name}.ts:`)), `typescript: ${vector.name}`).toBe(!vector.stripOnly);
  });
});

test("the packages vite/vitest configs externalize are exactly the declared native surface", () => {
  expect(configExternalizedEntries()).toEqual([...fixture.nativeEntries].sort());
}, 60_000);

test("every native entry's runtime closure is erasable, imports types as types, and loads in Node without a bundler", () => {
  for (const entry of fixture.nativeEntries) {
    expect(stripOnlyFindings(nativeClosure(entry)), entry).toEqual([]);
    const load = spawnSync("node", ["--no-warnings", "--experimental-strip-types", "--input-type=module", "-e", `await import(${JSON.stringify(pathToFileURL(join(root, entry)).href)});`], { cwd: root, encoding: "utf8" });
    expect(load.status, `${entry}\n${load.stderr.slice(0, 2048)}`).toBe(0);
  }
}, 120_000);
//#endregion 🧪️Laws
