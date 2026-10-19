/** 📦️ Resolves current authored public workspace exports against the independent resolve.exports oracle.
 * @see ../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📦️workspace-exports/🧫️fixtures/🔣️.json */
import Ajv from "ajv";
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { exports as resolveExports } from "resolve.exports";
import { parse as parseJsonc, type ParseError } from "jsonc-parser";
import ts from "typescript";
import { expect, inject, it } from "vitest";
import fixture from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📦️workspace-exports/🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📦️workspace-exports/🧬️schema/🔣️.json" with { type: "json" };
import { publicRuntimePackageExportsV1, workspaceTestAliasesV1 } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📦️workspace-exports/🟦️.ts";
import { resolveTestLevel, TEST_LEVEL_BUDGET_MS } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";

const validate = new Ajv({ strict: true }).compile(schema);
declare module "vitest" {
  interface ProvidedContext { architectureWorkspaceExports: readonly { pattern: string; flags: string; replacement: string }[] }
}
for (const row of fixture.cases) it(`resolves public package export ${row.id}`, () => {
  const actual = publicRuntimePackageExportsV1(row.manifest);
  expect(actual).toEqual(row.expected);
  expect(validate(actual), JSON.stringify(validate.errors)).toBe(true);
  for (const entry of actual) expect(entry.target).toBe(resolveExports(row.manifest, entry.specifier)?.[0]);
});
for (const row of fixture.invalid) it(`refuses nonportable or nonexplicit public exports ${JSON.stringify(row)}`, () => {
  expect(() => publicRuntimePackageExportsV1(row)).toThrow();
});
it("resolves current course dependency public surfaces without admitting their private modules", () => {
  const aliases = inject("architectureWorkspaceExports").map(row => ({ find: new RegExp(row.pattern, row.flags), replacement: row.replacement }));
  expect(aliases.length).toBeGreaterThan(0);
  for (const specifier of fixture.workspace) {
    const matches = aliases.filter(alias => alias.find.test(specifier));
    expect(matches.length, specifier).toBe(1);
    expect(existsSync(matches[0]!.replacement), specifier).toBe(true);
    expect(aliases.some(alias => alias.find.test(`${specifier}/private`)), specifier).toBe(false);
  }
});
it("honors resolution cancellation before reading workspace membership", async () => {
  const controller = new AbortController(), reason = Error("resolution cancelled");
  controller.abort(reason);
  await expect(workspaceTestAliasesV1("absent-workspace", { signal: controller.signal })).rejects.toBe(reason);
});
it("interrupts actual workspace ownership advances before the expensive scan finishes", async () => {
  const workspace = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../.."), controller = new AbortController(), reason = Error("ownership scan cancelled");
  let advances = 0;
  let eventLoopAdvanced = false;
  const heartbeat = new Promise<void>(accept => setImmediate(() => { eventLoopAdvanced = true; accept(); }));
  const began = performance.now();
  const pending = Promise.resolve().then(() => workspaceTestAliasesV1(workspace, { owner: fixture.sourceLock.owner, signal: controller.signal, advance: () => { advances++; if (advances === fixture.cancellation.afterAdvances) controller.abort(reason); } }));
  await expect(pending).rejects.toBe(reason);
  expect(advances).toBe(fixture.cancellation.afterAdvances);
  expect(eventLoopAdvanced).toBe(true);
  expect(performance.now() - began).toBeLessThan(fixture.cancellation.maximumElapsedMilliseconds);
  await heartbeat;
}, 60_000);
it("keeps the authored teaching lock parseable and its current course dependency edges exact", () => {
  const workspace = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../.."), errors: ParseError[] = [];
  const lock = parseJsonc(readFileSync(resolve(workspace, fixture.sourceLock.path), "utf8"), errors, { allowTrailingComma: true }) as { workspaces: Record<string, Record<string, unknown>> };
  expect(errors).toEqual([]);
  const entry = lock.workspaces[fixture.sourceLock.workspace];
  const validateEntry = new Ajv({ strict: true }).compile(schema.$defs.SourceLockWorkspace);
  expect(validateEntry(entry), JSON.stringify(validateEntry.errors)).toBe(true);
  const manifest = JSON.parse(readFileSync(resolve(workspace, "🎓️teaching", fixture.sourceLock.workspace, "package.json"), "utf8")) as Record<string, unknown>;
  expect(entry!.name).toBe(fixture.sourceLock.name);
  expect(entry!.version).toBe(manifest.version);
  for (const section of fixture.sourceLock.dependencySections) expect(entry![section], section).toEqual(manifest[section]);
});
it("declares the quick minimum for source ownership and course runtime unit proofs", () => {
  const script = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript/📜️script.ts");
  const source = ts.createSourceFile(script, readFileSync(script, "utf8"), ts.ScriptTarget.Latest, true);
  const calls: ts.CallExpression[] = [];
  const visit = (node: ts.Node): void => { if (ts.isCallExpression(node) && node.expression.getText(source) === "resolveTestLevel") calls.push(node); ts.forEachChild(node, visit); };
  visit(source);
  expect(calls.map(call => ts.isStringLiteral(call.arguments[1]!) ? call.arguments[1]!.text : null)).toEqual([fixture.testing.minimum]);
  const before = process.env.SEMIO_TEST_LEVEL;
  try {
    expect(resolveTestLevel(["fundamental", "--run"], "quick")).toEqual({ level: fixture.testing.minimum, rest: ["--run"] });
    expect(TEST_LEVEL_BUDGET_MS.quick).toBe(fixture.testing.budgetMilliseconds);
  } finally { if (before === undefined) delete process.env.SEMIO_TEST_LEVEL; else process.env.SEMIO_TEST_LEVEL = before; }
});

for (const row of fixture.sourceExports) it(`resolves authored public source wrapper ${row.specifier}`, () => {
  const workspace = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../.."), manifestPath = resolve(workspace, row.manifest);
  const manifest = JSON.parse(readFileSync(manifestPath, "utf8")) as { name: string; exports: Record<string, string> };
  const actual = publicRuntimePackageExportsV1(manifest).find(entry => entry.specifier === row.specifier);
  expect(actual?.target).toBe(row.target);
  expect(resolveExports(manifest, row.specifier)).toEqual([row.target]);
  const wrapper = resolve(dirname(manifestPath), row.target), source = ts.createSourceFile(wrapper, readFileSync(wrapper, "utf8"), ts.ScriptTarget.Latest, true);
  const values: string[] = [], types: string[] = [];
  for (const statement of source.statements) {
    expect(ts.isExportDeclaration(statement)).toBe(true);
    if (!ts.isExportDeclaration(statement)) continue;
    expect(statement.exportClause && ts.isNamedExports(statement.exportClause)).toBe(true);
    expect(statement.moduleSpecifier && ts.isStringLiteral(statement.moduleSpecifier)).toBe(true);
    if (!statement.exportClause || !ts.isNamedExports(statement.exportClause) || !statement.moduleSpecifier || !ts.isStringLiteral(statement.moduleSpecifier)) continue;
    expect(resolve(dirname(wrapper), statement.moduleSpecifier.text)).toBe(resolve(workspace, row.source));
    for (const entry of statement.exportClause.elements) (statement.isTypeOnly || entry.isTypeOnly ? types : values).push(entry.name.text);
  }
  expect(values.sort()).toEqual([...row.values].sort());
  expect(types.sort()).toEqual([...row.types].sort());
  const resolved = ts.resolveModuleName(row.specifier, resolve(dirname(manifestPath), "__source_contract__.ts"), { moduleResolution: ts.ModuleResolutionKind.Bundler, module: ts.ModuleKind.ESNext, allowImportingTsExtensions: true }, ts.sys);
  expect(resolved.resolvedModule?.resolvedFileName).toBe(wrapper.replaceAll("\\", "/"));
});
