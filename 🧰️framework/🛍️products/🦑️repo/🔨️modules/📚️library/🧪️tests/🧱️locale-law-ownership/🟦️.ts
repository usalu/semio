/** 🌐️ Keeps the retained history-label law under its Kernel consumer with neutral localized inputs. */
import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { cpSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync, symlinkSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import Ajv from "ajv";
import ts from "typescript";
import { findWorkspaceRoot } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { readVitestPolicyV1 } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🧪️vitest/🟦️.ts";
import corpus from "../../🧫️fixtures/🧱️locale-law-ownership/🔣️.json";
import schema from "../../🧬️schema/🧱️locale-law-ownership/🔣️.json";

const root = findWorkspaceRoot(import.meta.dir);
const read = (path: string): string => readFileSync(join(root, path), "utf8");
const digest = (bytes: string | Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
const syntax = (path: string): ts.SourceFile => ts.createSourceFile(path, read(path), ts.ScriptTarget.Latest, true);

function includes(source: ts.SourceFile, property = "include"): string[] {
  let result: string[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isPropertyAssignment(node) && node.name.getText(source) === property && ts.isArrayLiteralExpression(node.initializer) && ts.isPropertyAssignment(node.parent.parent) && node.parent.parent.name.getText(source) === "test") result = node.initializer.elements.filter(ts.isStringLiteral).map(node => node.text);
    ts.forEachChild(node, visit);
  };
  visit(source);
  return result;
}

test("the unchanged general Kernel suite executes with all product directories absent", () => {
  const isolated = join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-strict-boundary/locale-general-absent");
  rmSync(isolated, { recursive: true, force: true });
  mkdirSync(isolated, { recursive: true });
  const visited = new Set<string>();
  const copy = (path: string): void => {
    const absolute = resolve(root, path);
    if (visited.has(absolute)) return;
    visited.add(absolute);
    expect(corpus.productRoots.every(area => !absolute.startsWith(join(root, area)))).toBe(true);
    if (statSync(absolute).isDirectory()) {
      mkdirSync(join(isolated, path), { recursive: true });
      return;
    }
    mkdirSync(dirname(join(isolated, path)), { recursive: true });
    cpSync(absolute, join(isolated, path));
    expect(digest(readFileSync(join(isolated, path)))).toBe(digest(readFileSync(absolute)));
    if (!/\.[cm]?[jt]sx?$/u.test(path)) return;
    const source = syntax(path);
    const dependency = (text: string): void => {
      if (!text.startsWith(".")) return;
      const destination = join(dirname(path), text);
      const existing = [destination, destination.replace(/\.js$/u, ".ts"), ...[".ts", ".js", "/index.ts", "/🟦️.ts"].map(extension => destination + extension)].find(candidate => existsSync(join(root, candidate)));
      if (!existing) throw new Error(`Missing actual dependency ${path}: ${text}`);
      copy(existing);
    };
    const visit = (node: ts.Node): void => {
      if ((ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) && node.moduleSpecifier && ts.isStringLiteral(node.moduleSpecifier)) dependency(node.moduleSpecifier.text);
      if (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword && node.arguments[0] && ts.isStringLiteral(node.arguments[0])) dependency(node.arguments[0].text);
      if (ts.isNewExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === "URL" && node.arguments?.[0] && ts.isStringLiteral(node.arguments[0]) && node.arguments[1]?.getText(source) === "import.meta.url") dependency(node.arguments[0].text);
      ts.forEachChild(node, visit);
    };
    visit(source);
  };
  copy(corpus.generalConfig);
  for (const path of includes(syntax(corpus.generalConfig))) copy(join(corpus.generalRoot, path));
  for (const path of ["🟦️.ts", "📤️return/📦️content/🟦️.ts"]) copy(join(corpus.generalRoot, path));
  const copyInputs = (path: string): void => {
    for (const entry of readdirSync(join(root, path), { withFileTypes: true })) {
      const child = join(path, entry.name);
      if (entry.isDirectory()) copyInputs(child);
      else copy(child);
    }
  };
  for (const path of corpus.generalInputDirectories) copyInputs(path);
  symlinkSync(join(root, "node_modules"), join(isolated, "node_modules"), process.platform === "win32" ? "junction" : "dir");
  for (const area of corpus.productRoots) expect(existsSync(join(isolated, area))).toBe(false);
  const policy = readVitestPolicyV1(process.env);
  const observed = spawnSync(policy.runtime, [policy.toolPath, "run", "--config", join(isolated, corpus.generalConfig), "--reporter=default", "--reporter=json", "--outputFile=" + join(isolated, "kernel-law-roster.json")], { cwd: isolated, encoding: "utf8", timeout: 15_000, env: { ...process.env, SEMIO_VITEST_POLICY: JSON.stringify({ ...policy, cwd: isolated }) } });
  expect(observed.error).toBeUndefined();
  expect(observed.status, observed.stderr + observed.stdout).toBe(0);
  const roster = JSON.parse(readFileSync(join(isolated, "kernel-law-roster.json"), "utf8")) as { testResults: { name: string; assertionResults: { fullName: string; status: string }[] }[] };
  expect(roster.testResults.flatMap(row => row.assertionResults).length).toBe(corpus.generalTestCount);
  expect(roster.testResults.flatMap(row => row.assertionResults).every(row => row.status === "passed")).toBe(true);
  expect(observed.stdout).toContain("passed");
  expect(observed.stdout.replace(/\u001b\[[0-9;]*m/gu, "")).toMatch(new RegExp(`Tests\\s+${corpus.generalTestCount} passed \\(${corpus.generalTestCount}\\)`, "u"));
}, 15_000);

test("the schema pins intact locale laws, original witnesses and canonical neutral registrations", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
  expect(existsSync(join(root, corpus.formerOwner))).toBe(false);
  const source = read(corpus.owner);
  expect(digest(source.slice(source.indexOf("const validate =")))).toBe(corpus.retainedTextSha256);
  expect(includes(syntax(corpus.generalConfig))).toEqual(corpus.generalIncludes);
  expect(includes(syntax(corpus.generalConfig), "includeSource")).toEqual(corpus.generalIncludeSource);
  expect(includes(syntax(corpus.specificConfig))).toEqual(["🧪️tests/🏷️localized-label-fixture/🟦️.ts"]);
  for (const input of corpus.inputs) expect(digest(readFileSync(join(root, input.path)))).toBe(input.sha256);
  const project = JSON.parse(read(corpus.project));
  expect(project.name).toBe(corpus.target.slice(0, corpus.target.lastIndexOf(":")));
  const target = project.targets[corpus.target.slice(corpus.target.lastIndexOf(":") + 1)];
  expect(target.options.command).toBe("bun ./📜️script.ts test localized-label");
  expect(target.cache).toBe(false);
  expect(read(corpus.script)).toContain('segments[0] === "localized-label"');
});
