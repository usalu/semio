import { expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import ts from "typescript";
import { parse as parseJsonc } from "jsonc-parser";
import { inventoryTaxonomy, planTaxonomy, type TaxonomyPlanOptions } from "../../🧹️normalization/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../../");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const ticket = join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION");
const schemaPath = `${library}/🔣️taxonomy.json`;
const vector = JSON.parse(readFileSync(join(root, library, "📦️packages/🟦️typescript/🧫️fixtures/🧪️taxonomy-cli-cancellation/🔣️.json"), "utf8"));

/** 🎛️ Compiles the actual CLI plan-options expression with two independent TypeScript implementations. */
function planOptionFactories(): ((baseline: string, cancel: string | undefined, progress: NonNullable<TaxonomyPlanOptions["progress"]>) => TaxonomyPlanOptions)[] {
  const source = ts.createSourceFile("📜️script.ts", readFileSync(join(root, "📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
  const owner = source.statements.find((node) => ts.isClassDeclaration(node) && node.name?.text === "CleanScript") as ts.ClassDeclaration;
  const method = owner.members.find((node) => ts.isMethodDeclaration(node) && node.name.getText(source) === "runTaxonomy")!;
  const calls: ts.CallExpression[] = [];
  const visit = (node: ts.Node): void => { if (ts.isCallExpression(node) && node.expression.getText(source) === "planTaxonomy") calls.push(node); ts.forEachChild(node, visit); };
  visit(method);
  expect(calls).toHaveLength(1);
  const code = `function capture(baseline, cancelArgumentPath, taxonomyCliProgress) { const options = { baseline }; return (${calls[0].arguments[1].getText(source)}); }`;
  return [new Bun.Transpiler({ loader: "ts" }).transformSync(code), ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText].map((compiled) => new Function(`${compiled}\nreturn capture;`)());
}

test("the CLI forwards its guarded cancellation path to every planning implementation", () => {
  const cancel = join(ticket, vector.cancelPath), progress = () => {};
  for (const factory of planOptionFactories()) {
    expect(factory("a".repeat(40), cancel, progress)).toEqual({ baselineCommit: "a".repeat(40), excludedTreeDigests: [], cancelFile: cancel, progress });
    expect(factory("a".repeat(40), undefined, progress).cancelFile).toBeUndefined();
  }
});

test("the real CLI options cancel incoming-reference planning without changing source bytes", () => {
  for (const factory of planOptionFactories()) {
    const directory = mkdtempSync(join(ticket, "🧪️cli-plan-cancellation-"));
    const put = (path: string, bytes: string | Buffer): void => { mkdirSync(dirname(join(directory, path)), { recursive: true }); writeFileSync(join(directory, path), bytes); };
    put(schemaPath, readFileSync(join(root, schemaPath)));
    put(vector.sourcePath, vector.source);
    const git = (args: string[]): string => { const result = Bun.spawnSync(["git", ...args], { cwd: directory, stdout: "pipe", stderr: "pipe" }); if (result.exitCode !== 0) throw new Error(result.stderr.toString()); return result.stdout.toString().trim(); };
    git(["init", "--quiet", "--object-format=sha1"]);
    git(["-c", "user.name=Fixture", "-c", "user.email=fixture@invalid.example", "-c", "commit.gpgsign=false", "commit", "--quiet", "--allow-empty", "-m", "fixture"]);
    const inventory = inventoryTaxonomy({ repoRoot: directory, scope: dirname(vector.sourcePath), workers: 1 });
    const cancel = join(directory, vector.cancelPath);
    let observed = false;
    const options = factory(git(["rev-parse", "HEAD"]), cancel, (event) => { if (!observed && event.phase === vector.phase) { observed = true; put(vector.cancelPath, "cancel\n"); console.log("[DEBUG] CLI incoming-plan cancellation requested"); } });
    expect(() => planTaxonomy(inventory, options)).toThrow(/cancel/iu);
    expect(observed).toBe(true);
    expect(existsSync(cancel)).toBe(true);
    expect(readFileSync(join(directory, vector.sourcePath), "utf8")).toBe(vector.source);
  }
});

test("registers the cancellation gate through Nx and both launch catalogs", () => {
  const expected = vector.execution;
  const project = JSON.parse(readFileSync(join(root, library, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
  expect(project.targets[expected.target]?.options.command).toBe(expected.command);
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const launches = parseJsonc(readFileSync(join(root, path), "utf8")).configurations.filter((entry: { name: string }) => entry.name === expected.launchName);
    expect(launches).toHaveLength(1);
    expect(launches[0].command).toBe(expected.launchCommand);
    expect(launches[0].presentation).toEqual({ group: expected.launchGroup, order: expected.launchOrder });
  }
});
