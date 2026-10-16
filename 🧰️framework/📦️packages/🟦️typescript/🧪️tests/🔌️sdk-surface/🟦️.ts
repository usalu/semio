/** 🔌️ SDK surface law: every `exports` entry of `@semio-tech/framework` is an explicit, resolvable and loadable facade.
 * Cases live in `🧫️fixtures/🔣️.json` so another language can reuse them; TypeScript's own program diagnostics are the independent oracle.
 * @see https://www.typescriptlang.org/tsconfig#isolatedModules
 */
import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import ts from "typescript";

const packageDir = resolve(import.meta.dir, "../..");
const manifest = JSON.parse(readFileSync(join(packageDir, "package.json"), "utf8")) as { exports: Record<string, string> };
const fixture = JSON.parse(readFileSync(join(import.meta.dir, "🧫️fixtures/🔣️.json"), "utf8")) as { facadeCases: { id: string; source: string; accept: boolean }[]; rootBarrel: string };

/** 🔎️ Lists every reason a facade source is not a pure explicit re-export file. */
function facadeViolations(name: string, source: string): string[] {
  const file = ts.createSourceFile(name, source, ts.ScriptTarget.ESNext, true, ts.ScriptKind.TS);
  const out: string[] = [];
  for (const statement of file.statements) {
    if (!ts.isExportDeclaration(statement) || !statement.moduleSpecifier) { out.push(`not a re-export: ${ts.SyntaxKind[statement.kind]}`); continue; }
    if (!statement.exportClause) out.push("export * is not explicit");
    else if (ts.isNamespaceExport(statement.exportClause)) out.push("namespace re-export is not explicit");
  }
  return out;
}

const facades = Object.entries(manifest.exports).filter(([subpath]) => subpath !== ".");

test("fixture cases classify facade sources", () => {
  for (const c of fixture.facadeCases) expect({ id: c.id, accepted: facadeViolations(`${c.id}.ts`, c.source).length === 0 }).toEqual({ id: c.id, accepted: c.accept });
});

test("every export target is a file inside the package", () => {
  for (const [subpath, target] of Object.entries(manifest.exports)) {
    expect({ subpath, inside: target.startsWith("./") && !target.split("/").includes("..") }).toEqual({ subpath, inside: true });
    expect({ subpath, exists: existsSync(join(packageDir, target)) }).toEqual({ subpath, exists: true });
  }
  expect(manifest.exports["."]).toBe(fixture.rootBarrel);
});

test("every non-root facade re-exports explicitly and nothing else", () => {
  for (const [subpath, target] of facades) expect({ subpath, violations: facadeViolations(target, readFileSync(join(packageDir, target), "utf8")) }).toEqual({ subpath, violations: [] });
});

test("the TypeScript program has no diagnostic on any facade (names exist, values are values)", () => {
  const options: ts.CompilerOptions = { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, resolveJsonModule: true, isolatedModules: true, esModuleInterop: true, jsx: ts.JsxEmit.ReactJSX, skipLibCheck: true, noEmit: true, types: [], noResolve: false };
  const roots = facades.map(([, target]) => join(packageDir, target));
  const program = ts.createProgram(roots, options);
  const mine = new Set(roots.map(p => resolve(p)));
  const diagnostics = program.getSemanticDiagnostics().concat(program.getSyntacticDiagnostics()).filter(d => d.file && mine.has(resolve(d.file.fileName)) && !(d.code === 2307 && /\.sql['"]/.test(ts.flattenDiagnosticMessageText(d.messageText, "\n"))));
  expect(diagnostics.map(d => `${d.file!.fileName}: ${ts.flattenDiagnosticMessageText(d.messageText, "\n")}`)).toEqual([]);
}, 180_000);

test("every facade loads through its package specifier in a fresh process", async () => {
  const queue = facades.map(([subpath]) => subpath);
  const results: { subpath: string; loaded: boolean; stderr: string }[] = [];
  const worker = async () => {
    for (let subpath = queue.shift(); subpath !== undefined; subpath = queue.shift()) {
      const child = Bun.spawn([process.execPath, "-e", `await import(${JSON.stringify(`@semio-tech/framework${subpath.slice(1)}`)}); process.stdout.write("LOADED");`], { cwd: packageDir, stdout: "pipe", stderr: "pipe" });
      const [out, err] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text()]);
      await child.exited;
      results.push({ subpath, loaded: out.includes("LOADED"), stderr: err.split("\n").find(line => /error|Cannot/i.test(line)) ?? "" });
    }
  };
  await Promise.all(Array.from({ length: 6 }, worker));
  expect(results.filter(r => !r.loaded)).toEqual([]);
  expect(results.length).toBe(facades.length);
}, 300_000);
