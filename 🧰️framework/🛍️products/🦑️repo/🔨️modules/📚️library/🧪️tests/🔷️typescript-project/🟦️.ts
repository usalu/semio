/** 🔷️ `verify typescript`: the generated project lists exactly the live sources, agrees with TypeScript's own file resolution and counts what the real compiler reports.
 * @see 🎛️commands/✅️verify/🔷️typescript-project/🟦️.ts
 */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, dirname, join, relative, resolve } from "node:path";
import ts from "typescript";
import { listTypescriptAppProjects, listTypescriptWorkspaceSources, parseTypescriptProjectArguments, readTypescriptRootList, readTypescriptWorkspaceExcludes, runTypescriptCompiler, selectsTypescriptAppProject, selectTypescriptRoots, summarizeTypescriptDiagnostics, summarizeTypescriptSyntaxFiles, typescriptProjectDocument } from "../../🎛️commands/✅️verify/🔷️typescript-project/🟦️.ts";

type Fixture = {
  readonly schema: string;
  readonly excludes: readonly string[];
  readonly sources: readonly string[];
  readonly changed: readonly string[];
  readonly cases: readonly { readonly name: string; readonly scopes: readonly string[]; readonly changedOnly: boolean; readonly listed?: readonly string[]; readonly roots: readonly string[] }[];
  readonly toRoot: string;
  readonly ambientPaths: Readonly<Record<string, string>>;
  readonly tscLog: string;
  readonly summary: { readonly total: number; readonly files: number; readonly byCode: Readonly<Record<string, number>> };
};

const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔷️typescript-project/🔣️.json", import.meta.url), "utf8")) as Fixture;
const workspaceRoot = resolve(import.meta.dir, "../../../../../../..");

/** 🧪️ Materializes the fixture sources below a scratch root and hands it to `use`, removing it afterwards. */
function withScratchRoot<T>(use: (scratch: string) => T): T {
  const scratch = mkdtempSync(join(tmpdir(), "semio-typescript-project-"));
  try {
    for (const source of fixture.sources) {
      mkdirSync(dirname(join(scratch, source)), { recursive: true });
      writeFileSync(join(scratch, source), "export {};\n");
    }
    return use(scratch);
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
}

/** 🔀️ Code-point order, the order the generator sorts roots in. */
function byCodePoint(paths: readonly string[]): string[] {
  return [...paths].sort((left, right) => (left < right ? -1 : left > right ? 1 : 0));
}

test("the fixture declares its schema", () => {
  expect(fixture.schema).toBe("semio.repo.library.typescript-project/v1");
});

for (const row of fixture.cases) {
  test(`root selection: ${row.name}`, () => {
    expect(selectTypescriptRoots(fixture.sources, fixture.excludes, { scopes: row.scopes, changedOnly: row.changedOnly, listed: row.listed }, fixture.changed)).toEqual([...row.roots]);
  });
}

test("the exclusion rules agree with TypeScript's own include/exclude resolution", () => {
  withScratchRoot((scratch) => {
    const parsed = ts.parseJsonConfigFileContent({ include: ["**/*.ts", "**/*.tsx"], exclude: [...fixture.excludes] }, ts.sys, scratch);
    const theirs = byCodePoint(parsed.fileNames.map((name) => relative(scratch, name).replaceAll("\\", "/")));
    expect(selectTypescriptRoots(fixture.sources, fixture.excludes, { scopes: [], changedOnly: false }, [])).toEqual(theirs);
  });
});

test("the generated document resolves to the same files through TypeScript with relative paths only", () => {
  withScratchRoot((scratch) => {
    const roots = selectTypescriptRoots(fixture.sources, fixture.excludes, { scopes: [], changedOnly: false }, []);
    const document = typescriptProjectDocument(roots, fixture.toRoot, fixture.ambientPaths);
    const serialized = JSON.stringify(document);
    expect(serialized.includes(scratch)).toBe(false);
    expect(document.extends).toBe(`${fixture.toRoot}/tsconfig.json`);
    expect((document.compilerOptions as { paths: Record<string, string[]> }).paths["lines-and-columns"]).toEqual([`${fixture.toRoot}/node_modules/lines-and-columns/build/index.d.ts`]);
    writeFileSync(join(scratch, "tsconfig.json"), "{}\n");
    const home = join(scratch, ".cache", "semio", "typescript");
    mkdirSync(home, { recursive: true });
    const parsed = ts.parseJsonConfigFileContent(document, ts.sys, home);
    expect(parsed.errors.filter((error) => error.code !== 18003)).toEqual([]);
    expect(byCodePoint(parsed.fileNames.map((name) => relative(scratch, name).replaceAll("\\", "/")))).toEqual(roots);
  });
});

test("the diagnostic summary counts a recorded tsc log, indented continuation lines excluded", () => {
  expect(summarizeTypescriptDiagnostics(fixture.tscLog)).toEqual(fixture.summary);
});

test("the diagnostic summary agrees with the real compiler on a project with two known errors", () => {
  const scratch = mkdtempSync(join(tmpdir(), "semio-typescript-compiler-"));
  try {
    writeFileSync(join(scratch, "tsconfig.json"), JSON.stringify({ compilerOptions: { strict: true, noEmit: true } }));
    writeFileSync(join(scratch, "wrong.ts"), "export const count: number = 'text';\nexport const missing = undeclared;\n");
    writeFileSync(join(scratch, "right.ts"), "export const fine: number = 1;\n");
    const home = join(scratch, ".cache", "semio", "typescript");
    mkdirSync(home, { recursive: true });
    const document = typescriptProjectDocument(["wrong.ts", "right.ts"], "../../..", {});
    writeFileSync(join(home, "tsconfig.json"), JSON.stringify({ ...document, compilerOptions: { ...(document.compilerOptions as object), types: [] } }));
    const run = spawnSync("node", [join(workspaceRoot, "node_modules", "typescript", "bin", "tsc"), "-p", join(home, "tsconfig.json"), "--noEmit", "--pretty", "false"], { cwd: scratch, encoding: "utf8" });
    expect(run.status).toBe(2);
    expect(summarizeTypescriptDiagnostics(run.stdout)).toEqual({ total: 2, files: 1, byCode: { TS2322: 1, TS2304: 1 } });
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
}, 120_000);

test("a root list file is read as repo-relative paths, absolute ones made relative, comments and blanks skipped", () => {
  const scratch = mkdtempSync(join(tmpdir(), "semio-typescript-list-"));
  try {
    writeFileSync(join(scratch, "roots.txt"), `# roots\n\na/one.ts\n${join(scratch, "b", "two.ts")}\r\na/one.ts\n  c\\three.tsx  \n`);
    expect(readTypescriptRootList(scratch, "roots.txt")).toEqual(["a/one.ts", "b/two.ts", "c/three.tsx"]);
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
});

test("flags parse strictly", () => {
  expect(parseTypescriptProjectArguments(["--scope", "./a\\b/", "--scope", "c", "--changed", "--heap-mb", "8192", "--max-diagnostics", "5", "--list", "--files-from", "roots.txt"])).toEqual({ selection: { scopes: ["a/b", "c"], changedOnly: true }, out: undefined, filesFrom: "roots.txt", heapMegabytes: 8192, timeoutMinutes: 180, maximumDiagnostics: 5, list: true });
  expect(() => parseTypescriptProjectArguments(["--scope", "../outside"])).toThrow();
  expect(() => parseTypescriptProjectArguments(["--scope", "/absolute"])).toThrow();
  expect(() => parseTypescriptProjectArguments(["--heap-mb", "12"])).toThrow();
  expect(() => parseTypescriptProjectArguments(["--unknown"])).toThrow();
  expect(() => parseTypescriptProjectArguments(["--scope"])).toThrow();
});

test("the live workspace yields only roots that exist and the exclude list is the one TypeScript reads", () => {
  const excludes = readTypescriptWorkspaceExcludes(workspaceRoot);
  expect(excludes).toEqual(ts.readConfigFile(join(workspaceRoot, "tsconfig.json"), ts.sys.readFile).config.exclude);
  const roots = selectTypescriptRoots(listTypescriptWorkspaceSources(workspaceRoot), excludes, { scopes: [], changedOnly: false }, []);
  expect(roots.length).toBeGreaterThan(1000);
  expect(roots.filter((path) => !existsSync(join(workspaceRoot, path)))).toEqual([]);
  expect(roots.filter((path) => path.includes("/node_modules/") || path.includes("/🗑️generated/") || path.startsWith(".🧬semio/🦑️repo/🎫️tickets/"))).toEqual([]);
}, 120_000);

test("an application whose ambient file is excluded is listed with its own project, and one without a project is refused loudly", () => {
  const scratch = mkdtempSync(join(tmpdir(), "semio-typescript-apps-"));
  try {
    mkdirSync(join(scratch, "web", "app"), { recursive: true });
    writeFileSync(join(scratch, "web", "tsconfig.json"), "{}\n");
    const sources = ["web/next-env.d.ts", "web/app/page.tsx", "lib/next-env.ts", "other/types.d.ts"];
    expect(listTypescriptAppProjects(scratch, sources, ["**/next-env.d.ts"])).toEqual([{ folder: "web", project: "web/tsconfig.json" }]);
    expect(listTypescriptAppProjects(scratch, sources, ["**/dist/**"])).toEqual([]);
    expect(() => listTypescriptAppProjects(scratch, [...sources, "orphan/next-env.d.ts"], ["**/next-env.d.ts"])).toThrow("verify.typescript.is-excluded-from-the-workspace");
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
});

test("an application is checked by an unrestricted run and by a selection that touches its folder only", () => {
  const app = { folder: "a/web", project: "a/web/tsconfig.json" };
  expect(selectsTypescriptAppProject(app, { scopes: [], changedOnly: false }, [])).toBe(true);
  expect(selectsTypescriptAppProject(app, { scopes: ["a"], changedOnly: false }, [])).toBe(true);
  expect(selectsTypescriptAppProject(app, { scopes: ["a/web/app"], changedOnly: false }, [])).toBe(true);
  expect(selectsTypescriptAppProject(app, { scopes: ["a/webby"], changedOnly: false }, [])).toBe(false);
  expect(selectsTypescriptAppProject(app, { scopes: [], changedOnly: true }, ["a/web/app/page.tsx"])).toBe(true);
  expect(selectsTypescriptAppProject(app, { scopes: [], changedOnly: true }, ["b/x.ts"])).toBe(false);
  expect(selectsTypescriptAppProject(app, { scopes: [], changedOnly: false, listed: ["a/web/app/page.tsx"] }, [])).toBe(true);
  expect(selectsTypescriptAppProject(app, { scopes: [], changedOnly: false, listed: ["b/x.ts"] }, [])).toBe(false);
});

test("the live workspace excludes the Next application from the program and still gives it a project that type-checks clean", async () => {
  const sources = listTypescriptWorkspaceSources(workspaceRoot);
  const apps = listTypescriptAppProjects(workspaceRoot, sources, readTypescriptWorkspaceExcludes(workspaceRoot));
  expect(apps.length).toBeGreaterThan(0);
  const scratch = mkdtempSync(join(tmpdir(), "semio-typescript-app-run-"));
  try {
    const log = join(scratch, "app.tsc.txt");
    expect(await runTypescriptCompiler(workspaceRoot, join(workspaceRoot, apps[0]!.project), log, join(scratch, "app.syntax.txt"), 8192, 10)).toBe(0);
    expect(summarizeTypescriptDiagnostics(readFileSync(log, "utf8")).total).toBe(0);
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
}, 600_000);

test("an application project with a real type error is counted by the same compiler run", async () => {
  const scratch = mkdtempSync(join(tmpdir(), "semio-typescript-app-error-"));
  try {
    writeFileSync(join(scratch, "tsconfig.json"), JSON.stringify({ compilerOptions: { strict: true, noEmit: true, types: [] }, include: ["page.ts"] }));
    writeFileSync(join(scratch, "page.ts"), "export const count: number = 'text';\n");
    const log = join(scratch, "app.tsc.txt");
    expect(await runTypescriptCompiler(workspaceRoot, join(scratch, "tsconfig.json"), log, join(scratch, "app.syntax.txt"), 2048, 5)).toBe(2);
    expect(summarizeTypescriptDiagnostics(readFileSync(log, "utf8"))).toEqual({ total: 1, files: 1, byCode: { TS2322: 1 } });
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
}, 120_000);

test("a file that does not parse is reported on its own and every other file is still type-checked, where tsc itself reports only the syntax error", async () => {
  const scratch = mkdtempSync(join(tmpdir(), "semio-typescript-syntax-"));
  try {
    writeFileSync(join(scratch, "tsconfig.json"), JSON.stringify({ compilerOptions: { strict: true, noEmit: true, types: [], allowImportingTsExtensions: true }, include: ["*.ts"] }));
    writeFileSync(join(scratch, "broken.ts"), "export const half = (;\nexport const also: number = 'text';\n");
    writeFileSync(join(scratch, "importer.ts"), "import { half } from './broken.ts';\nexport const text: string = 1;\nexport { half };\n");
    writeFileSync(join(scratch, "wrong.ts"), "export const count: number = 'text';\n");
    writeFileSync(join(scratch, "right.ts"), "export const fine: number = 1;\n");
    const reference = spawnSync("node", [join(workspaceRoot, "node_modules", "typescript", "bin", "tsc"), "-p", join(scratch, "tsconfig.json"), "--noEmit", "--pretty", "false"], { cwd: scratch, encoding: "utf8" });
    expect(summarizeTypescriptDiagnostics(reference.stdout).byCode).toEqual({ TS1109: 1 });
    const log = join(scratch, "check.tsc.txt"), syntaxLog = join(scratch, "check.syntax.txt");
    expect(await runTypescriptCompiler(scratch, join(scratch, "tsconfig.json"), log, syntaxLog, 2048, 5)).toBe(2);
    const summary = summarizeTypescriptDiagnostics(readFileSync(log, "utf8"));
    expect(summary).toEqual({ total: 3, files: 3, byCode: { TS1109: 1, TS2322: 2 } });
    const syntax = summarizeTypescriptSyntaxFiles(readFileSync(syntaxLog, "utf8"));
    expect(syntax.map(({ file, count }) => [basename(file), count])).toEqual([["broken.ts", 1]]);
    expect(readFileSync(log, "utf8")).toContain("importer.ts(2,");
    expect(readFileSync(log, "utf8")).toContain("wrong.ts(1,");
    expect(readFileSync(log, "utf8")).not.toContain("broken.ts(2,");
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
}, 120_000);

test("on a project that parses, the child reports exactly what tsc reports", async () => {
  const scratch = mkdtempSync(join(tmpdir(), "semio-typescript-parity-"));
  try {
    writeFileSync(join(scratch, "tsconfig.json"), JSON.stringify({ compilerOptions: { strict: true, noEmit: true, types: [] }, include: ["*.ts"] }));
    writeFileSync(join(scratch, "wrong.ts"), "export const count: number = 'text';\nexport const missing = undeclared;\n");
    writeFileSync(join(scratch, "also.ts"), "export const flag: boolean = 1;\n");
    const reference = spawnSync("node", [join(workspaceRoot, "node_modules", "typescript", "bin", "tsc"), "-p", join(scratch, "tsconfig.json"), "--noEmit", "--pretty", "false"], { cwd: scratch, encoding: "utf8" });
    const log = join(scratch, "check.tsc.txt");
    expect(await runTypescriptCompiler(scratch, join(scratch, "tsconfig.json"), log, join(scratch, "check.syntax.txt"), 2048, 5)).toBe(reference.status ?? -1);
    expect(readFileSync(log, "utf8")).toBe(reference.stdout);
    expect(readFileSync(join(scratch, "check.syntax.txt"), "utf8")).toBe("");
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
}, 120_000);

test("syntax files are listed once each with their count and first diagnostic", () => {
  const log = "a/one.ts(1,19): error TS1109: Expression expected.\n  continuation\na/one.ts(3,1): error TS1005: ';' expected.\nb/two.tsx(2,2): error TS1128: Declaration or statement expected.\n";
  expect(summarizeTypescriptSyntaxFiles(log)).toEqual([
    { file: "a/one.ts", count: 2, first: "a/one.ts(1,19): error TS1109: Expression expected." },
    { file: "b/two.tsx", count: 1, first: "b/two.tsx(2,2): error TS1128: Declaration or statement expected." },
  ]);
  expect(summarizeTypescriptSyntaxFiles("")).toEqual([]);
});
