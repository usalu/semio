/** 🔌️ SDK surface law: every `exports` entry of `@semio-tech/framework` is an explicit, resolvable and loadable facade.
 * Cases live in `🧫️fixtures/🔣️.json` so another language can reuse them; TypeScript's own program diagnostics are the independent oracle.
 * @see https://www.typescriptlang.org/tsconfig#isolatedModules
 */
import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import ts from "typescript";
import { testWorkersV1 } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/👷️workers/🟦️.ts";
/** 👷️ The fresh processes the two loading laws run at once: three at most, fewer when the admitted memory of the test run carries fewer (each loads a facade, 400 MB); six at once held the whole run at 2.8 GB. */
const processes = testWorkersV1("bun-test", process.env, { request: 3 }).workers;

const packageDir = resolve(import.meta.dir, "../..");
const manifest = JSON.parse(readFileSync(join(packageDir, "package.json"), "utf8")) as { exports: Record<string, string> };
const fixture = JSON.parse(readFileSync(join(import.meta.dir, "🧫️fixtures/🔣️.json"), "utf8")) as { facadeCases: { id: string; source: string; accept: boolean }[]; rootBarrel: string };

/** 📄️ Names the local binding of a `import x from "<document>.json" with { type: "json" }` statement, or null. */
function jsonDocumentImport(statement: ts.Statement): string | null {
  if (!ts.isImportDeclaration(statement) || !statement.importClause?.name || statement.importClause.namedBindings || statement.importClause.isTypeOnly) return null;
  if (!ts.isStringLiteral(statement.moduleSpecifier) || !statement.moduleSpecifier.text.endsWith(".json")) return null;
  const typed = statement.attributes?.elements.some(attribute => attribute.name.text === "type" && ts.isStringLiteral(attribute.value) && attribute.value.text === "json");
  return typed ? statement.importClause.name.text : null;
}

/** 🔎️ Lists every reason a facade source is not a pure explicit re-export file. */
function facadeViolations(name: string, source: string): string[] {
  const file = ts.createSourceFile(name, source, ts.ScriptTarget.ESNext, true, ts.ScriptKind.TS);
  const out: string[] = [];
  const documents = new Set<string>();
  for (const statement of file.statements) {
    const document = jsonDocumentImport(statement);
    if (document !== null) { documents.add(document); continue; }
    if (ts.isExportAssignment(statement) && !statement.isExportEquals && ts.isIdentifier(statement.expression) && documents.has(statement.expression.text)) continue;
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
  const diagnostics = roots.flatMap(root => { const file = program.getSourceFile(root); return file ? [...program.getSemanticDiagnostics(file), ...program.getSyntacticDiagnostics(file)] : []; }).filter(d => d.file && mine.has(resolve(d.file.fileName)) && !(d.code === 2307 && /\.sql['"]/.test(ts.flattenDiagnosticMessageText(d.messageText, "\n"))));
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
  await Promise.all(Array.from({ length: processes }, worker));
  expect(results.filter(r => !r.loaded)).toEqual([]);
  expect(results.length).toBe(facades.length);
}, 300_000);

/** 🪞️ Lists the value re-exports a facade declares as `[source module file, [local name, exported name][]]`, read by TypeScript's own parser. */
function facadeValueReexports(file: string): [string, [string, string][]][] {
  const source = ts.createSourceFile(file, readFileSync(file, "utf8"), ts.ScriptTarget.ESNext, true, ts.ScriptKind.TS);
  const by = new Map<string, [string, string][]>();
  for (const statement of source.statements) {
    if (!ts.isExportDeclaration(statement) || statement.isTypeOnly || !statement.moduleSpecifier || !statement.exportClause || !ts.isNamedExports(statement.exportClause)) continue;
    const specifier = (statement.moduleSpecifier as ts.StringLiteral).text;
    const module = specifier.startsWith(".") ? resolve(join(file, ".."), specifier) : Bun.resolveSync(specifier, join(file, ".."));
    const rows = by.get(module) ?? [];
    for (const element of statement.exportClause.elements) if (!element.isTypeOnly) rows.push([(element.propertyName ?? element.name).text, element.name.text]);
    by.set(module, rows);
  }
  return [...by];
}

test("every facade value export is the very binding of the module it names (runtime identity in a fresh process)", async () => {
  const queue = facades.map(([subpath, target]) => ({ subpath, file: join(packageDir, target) }));
  const script = `const spec = JSON.parse(process.env.SEMIO_FACADE_IDENTITY); const facade = await import(spec.facade); const bad = []; for (const [module, pairs] of spec.modules) { const loaded = await import(module); for (const [local, exported] of pairs) if (!(exported in facade) || facade[exported] !== loaded[local]) bad.push(exported); } process.stdout.write(JSON.stringify(bad));`;
  const results: { subpath: string; mismatched: string[]; stderr: string }[] = [];
  const worker = async () => {
    for (let job = queue.shift(); job !== undefined; job = queue.shift()) {
      const modules = facadeValueReexports(job.file);
      if (modules.length === 0) continue;
      const child = Bun.spawn([process.execPath, "-e", script], { cwd: packageDir, stdout: "pipe", stderr: "pipe", env: { ...process.env, SEMIO_FACADE_IDENTITY: JSON.stringify({ facade: job.file, modules }) } });
      const [out, err] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text()]);
      await child.exited;
      results.push({ subpath: job.subpath, mismatched: out ? JSON.parse(out) : ["<did not run>"], stderr: out ? "" : err.split("\n").find(line => /error|Cannot/i.test(line)) ?? "" });
    }
  };
  await Promise.all(Array.from({ length: processes }, worker));
  expect(results.filter(r => r.mismatched.length > 0)).toEqual([]);
  expect(results.length).toBeGreaterThan(100);
}, 300_000);
