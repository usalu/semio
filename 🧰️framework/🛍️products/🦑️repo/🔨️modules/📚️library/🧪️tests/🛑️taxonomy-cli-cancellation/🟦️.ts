import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { deflateSync } from "node:zlib";
import ts from "typescript";
import { parse as parseJsonc } from "jsonc-parser";
import { inventoryTaxonomy, planTaxonomy, type TaxonomyPlanOptions } from "../../🧹️normalization/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../../");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required for taxonomy cancellation output.");
const ticket = join(artifactRoot, "taxonomy-cli-cancellation");
const workflowPath = resolve(import.meta.dir, "../../🧹️normalization/🎮️command-contract/🔁️workflow/🟦️.ts");
const commandPath = resolve(import.meta.dir, "../../🧼️workspace-cleanup/🎮️command/🟦️.ts");
const schemaPath = `${library}/🔣️taxonomy.json`;
const vector = JSON.parse(readFileSync(join(root, library, "🧫️fixtures/🛑️taxonomy-cli-cancellation/🔣️.json"), "utf8"));

/** 🧱️ Materializes one deterministic empty committed repository without invoking a mutating Git command. */
function materializeCommittedFixture(directory: string): string {
  const objects = join(directory, ".git", "objects");
  const object = (kind: "tree" | "commit", bytes: Buffer): string => {
    const payload = Buffer.concat([Buffer.from(`${kind} ${bytes.length}\0`), bytes]);
    const identity = createHash("sha1").update(payload).digest("hex");
    mkdirSync(join(objects, identity.slice(0, 2)), { recursive: true });
    writeFileSync(join(objects, identity.slice(0, 2), identity.slice(2)), deflateSync(payload));
    return identity;
  };
  const tree = object("tree", Buffer.alloc(0));
  const commit = object("commit", Buffer.from(`tree ${tree}\nauthor Fixture <fixture@invalid.example> 0 +0000\ncommitter Fixture <fixture@invalid.example> 0 +0000\n\nfixture\n`));
  mkdirSync(join(directory, ".git", "refs", "heads"), { recursive: true });
  writeFileSync(join(directory, ".git", "HEAD"), "ref: refs/heads/main\n");
  writeFileSync(join(directory, ".git", "refs", "heads", "main"), `${commit}\n`);
  return commit;
}

/** 🎛️ Compiles the actual CLI plan-options expression with two independent TypeScript implementations. */
function planOptionFactories(): ((baseline: string, cancel: string | undefined, progress: NonNullable<TaxonomyPlanOptions["progress"]>) => TaxonomyPlanOptions)[] {
  const source = ts.createSourceFile("🟦️.ts", readFileSync(workflowPath, "utf8"), ts.ScriptTarget.Latest, true);
  const owner = source.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === "runTaxonomyCliWorkflow") as ts.FunctionDeclaration;
  const calls: ts.CallExpression[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && node.expression.getText(source) === "planTaxonomy") calls.push(node);
    ts.forEachChild(node, visit);
  };
  visit(owner);
  expect(calls).toHaveLength(1);
  const code = `function capture(baseline, cancelArgumentPath, taxonomyCliProgress) { const options = { baseline }; return (${calls[0].arguments[1].getText(source)}); }`;
  return [new Bun.Transpiler({ loader: "ts" }).transformSync(code), ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText].map((compiled) => new Function(`${compiled}\nreturn capture;`)());
}

test("the clean command delegates taxonomy arguments to the extracted workflow owner", () => {
  const source = readFileSync(commandPath, "utf8");
  const syntax = ts.createSourceFile("🟦️.ts", source, ts.ScriptTarget.Latest, true);
  const owner = syntax.statements.find((node) => ts.isClassDeclaration(node) && node.name?.text === "CleanScript") as ts.ClassDeclaration;
  const run = owner.members.find((node) => ts.isMethodDeclaration(node) && node.name.getText(syntax) === "run")!;
  const calls: ts.CallExpression[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && node.expression.getText(syntax) === "runTaxonomyCliWorkflow") calls.push(node);
    ts.forEachChild(node, visit);
  };
  visit(run);
  expect(calls).toHaveLength(1);
  expect(calls[0]!.arguments.map((argument) => argument.getText(syntax))).toEqual(["this.root", "segments.slice(1)"]);
});

test("the CLI forwards its guarded cancellation path to every planning implementation", () => {
  const cancel = join(ticket, vector.cancelPath),
    progress = () => {};
  for (const factory of planOptionFactories()) {
    expect(factory("a".repeat(40), cancel, progress)).toEqual({ baselineCommit: "a".repeat(40), excludedTreeDigests: [], cancelFile: cancel, progress });
    expect(factory("a".repeat(40), undefined, progress).cancelFile).toBeUndefined();
  }
});

test("the real CLI options cancel incoming-reference planning without changing source bytes", { timeout: 30_000 }, () => {
  mkdirSync(ticket, { recursive: true });
  for (const factory of planOptionFactories()) {
    const directory = mkdtempSync(join(ticket, "🧪️cli-plan-cancellation-"));
    const put = (path: string, bytes: string | Buffer): void => {
      mkdirSync(dirname(join(directory, path)), { recursive: true });
      writeFileSync(join(directory, path), bytes);
    };
    put(schemaPath, readFileSync(join(root, schemaPath)));
    put(vector.sourcePath, vector.source);
    const git = (args: string[]): string => {
      const result = Bun.spawnSync(["git", ...args], { cwd: directory, stdout: "pipe", stderr: "pipe" });
      if (result.exitCode !== 0) throw new Error(result.stderr.toString());
      return result.stdout.toString().trim();
    };
    const baseline = materializeCommittedFixture(directory);
    expect(git(["rev-parse", "HEAD"])).toBe(baseline);
    expect(git(["cat-file", "-t", baseline])).toBe("commit");
    const inventory = inventoryTaxonomy({ repoRoot: directory, scope: dirname(vector.sourcePath), workers: 1 });
    const cancel = join(directory, vector.cancelPath);
    let observed = false;
    const options = factory(baseline, cancel, (event) => {
      if (!observed && event.phase === vector.phase) {
        observed = true;
        put(vector.cancelPath, "cancel\n");
        console.log("[DEBUG] CLI incoming-plan cancellation requested");
      }
    });
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
