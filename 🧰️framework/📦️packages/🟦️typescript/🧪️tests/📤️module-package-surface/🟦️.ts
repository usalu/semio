/** 📤️ Module package surface law: every `exports` entry of the value, schema validator and schema TypeScript packages is an explicit, resolvable and loadable facade.
 * Cases live in `🧫️fixtures/🔣️.json` so another language can reuse them; TypeScript's own program diagnostics are the independent oracle.
 * @see https://nodejs.org/api/packages.html#subpath-exports
 */
import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import ts from "typescript";

const repoRoot = resolve(import.meta.dir, "../../../../..");
const fixture = JSON.parse(readFileSync(join(import.meta.dir, "🧫️fixtures/🔣️.json"), "utf8")) as {
  packages: { name: string; dir: string }[];
  facadeCases: { id: string; source: string; accept: boolean }[];
  exportKeyCases: { id: string; key: string; accept: boolean }[];
};
const packages = fixture.packages.map(row => ({ ...row, manifest: JSON.parse(readFileSync(join(repoRoot, row.dir, "package.json"), "utf8")) as { name: string; exports: Record<string, string> } }));
const entries = packages.flatMap(row => Object.entries(row.manifest.exports).map(([subpath, target]) => ({ name: row.name, dir: join(repoRoot, row.dir), subpath, target, file: join(repoRoot, row.dir, target) })));

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
    if (ts.isImportDeclaration(statement) && !statement.importClause) continue;
    if (!ts.isExportDeclaration(statement) || !statement.moduleSpecifier) { out.push(`not a re-export: ${ts.SyntaxKind[statement.kind]}`); continue; }
    if (!statement.exportClause) out.push("export * is not explicit");
    else if (ts.isNamespaceExport(statement.exportClause)) out.push("namespace re-export is not explicit");
  }
  return out;
}

/** 🔑️ Accepts only explicit, non-wildcard, dot-free export keys. */
function exportKeyAccepted(key: string): boolean {
  return key === "." || (/^\.\/[^*]+$/u.test(key) && key.slice(2).split("/").every(segment => segment !== "" && segment !== "." && segment !== ".."));
}

/** 🪞️ Lists the value names a facade declares, read by TypeScript's own parser. */
function facadeValueNames(file: string): string[] {
  const source = ts.createSourceFile(file, readFileSync(file, "utf8"), ts.ScriptTarget.ESNext, true, ts.ScriptKind.TS);
  const names: string[] = [];
  for (const statement of source.statements) {
    if (ts.isExportAssignment(statement) && !statement.isExportEquals) { names.push("default"); continue; }
    if (!ts.isExportDeclaration(statement) || statement.isTypeOnly || !statement.exportClause || !ts.isNamedExports(statement.exportClause)) continue;
    for (const element of statement.exportClause.elements) if (!element.isTypeOnly) names.push(element.name.text);
  }
  return names;
}

test("fixture cases classify facade sources", () => {
  for (const c of fixture.facadeCases) expect({ id: c.id, accepted: facadeViolations(`${c.id}.ts`, c.source).length === 0 }).toEqual({ id: c.id, accepted: c.accept });
});

test("fixture cases classify export keys", () => {
  for (const c of fixture.exportKeyCases) expect({ id: c.id, accepted: exportKeyAccepted(c.key) }).toEqual({ id: c.id, accepted: c.accept });
});

test("every export key is explicit and every target is a file inside its package", () => {
  expect(entries.length).toBeGreaterThan(30);
  for (const e of entries) {
    expect({ name: e.name, subpath: e.subpath, explicit: exportKeyAccepted(e.subpath) }).toEqual({ name: e.name, subpath: e.subpath, explicit: true });
    expect({ name: e.name, subpath: e.subpath, inside: e.target.startsWith("./") && !e.target.split("/").includes("..") }).toEqual({ name: e.name, subpath: e.subpath, inside: true });
    expect({ name: e.name, subpath: e.subpath, exists: existsSync(e.file) }).toEqual({ name: e.name, subpath: e.subpath, exists: true });
  }
});

test("every facade re-exports explicitly and nothing else", () => {
  for (const e of entries) expect({ name: e.name, subpath: e.subpath, violations: facadeViolations(e.target, readFileSync(e.file, "utf8")) }).toEqual({ name: e.name, subpath: e.subpath, violations: [] });
});

test("the TypeScript program has no diagnostic on any facade (names exist, values are values)", () => {
  const options: ts.CompilerOptions = { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, resolveJsonModule: true, isolatedModules: true, esModuleInterop: true, skipLibCheck: true, noEmit: true, types: [] };
  const roots = entries.map(e => e.file);
  const program = ts.createProgram(roots, options);
  const mine = new Set(roots.map(p => resolve(p)));
  const diagnostics = program.getSemanticDiagnostics().concat(program.getSyntacticDiagnostics()).filter(d => d.file && mine.has(resolve(d.file.fileName)));
  expect(diagnostics.map(d => `${d.file!.fileName}: ${ts.flattenDiagnosticMessageText(d.messageText, "\n")}`)).toEqual([]);
}, 180_000);

test("every facade loads through its package specifier in a fresh process and exposes its declared values", async () => {
  const queue = [...entries];
  const results: { specifier: string; ok: boolean; detail: string }[] = [];
  const worker = async () => {
    for (let e = queue.shift(); e !== undefined; e = queue.shift()) {
      const specifier = e.name + (e.subpath === "." ? "" : e.subpath.slice(1));
      const script = `const loaded = await import(${JSON.stringify(specifier)}); const missing = ${JSON.stringify(facadeValueNames(e.file))}.filter(name => !(name in loaded)); process.stdout.write(JSON.stringify(missing));`;
      const child = Bun.spawn([process.execPath, "-e", script], { cwd: repoRoot, stdout: "pipe", stderr: "pipe" });
      const [out, err] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text()]);
      await child.exited;
      const missing = out.trim().endsWith("]") ? (JSON.parse(out.slice(out.lastIndexOf("["))) as string[]) : null;
      results.push({ specifier, ok: missing !== null && missing.length === 0, detail: missing === null ? err.split("\n").find(line => /error|Cannot/iu.test(line)) ?? "did not run" : missing.join(",") });
    }
  };
  await Promise.all(Array.from({ length: 6 }, worker));
  expect(results.filter(r => !r.ok)).toEqual([]);
  expect(results.length).toBe(entries.length);
}, 300_000);
