/** 🧱️ Proves physical product law ownership without changing retained witnesses. */
import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { cpSync, existsSync, mkdirSync, readFileSync, rmSync, symlinkSync } from "node:fs";
import { dirname, join } from "node:path";
import { spawnSync } from "node:child_process";
import Ajv from "ajv";
import ts from "typescript";
import { findWorkspaceRoot } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import corpus from "../../🧫️fixtures/🧱️fixture-law-ownership/🔣️.json";


const root = findWorkspaceRoot(import.meta.dir);
const read = (path: string): string => readFileSync(join(root, path), "utf8");
const digest = (bytes: string | Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
const parse = (path: string): ts.SourceFile => ts.createSourceFile(path, read(path), ts.ScriptTarget.Latest, true);

function laws(source: ts.SourceFile): ts.ExpressionStatement[] {
  return source.statements.filter((node): node is ts.ExpressionStatement => ts.isExpressionStatement(node) && ts.isCallExpression(node.expression) && ts.isIdentifier(node.expression.expression) && node.expression.expression.text === "test");
}

function registrations(source: ts.SourceFile): string[] {
  const result: string[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && node.expression.name.text === "register" && node.arguments[0] && ts.isStringLiteral(node.arguments[0])) result.push(node.arguments[0].text);
    if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === "runArtifactRustPackageMain" && node.arguments[2] && ts.isObjectLiteralExpression(node.arguments[2])) {
      const commands = node.arguments[2].properties.find(property => ts.isPropertyAssignment(property) && property.name.getText(source) === "commands");
      if (commands && ts.isPropertyAssignment(commands) && ts.isObjectLiteralExpression(commands.initializer)) for (const property of commands.initializer.properties) if (ts.isPropertyAssignment(property)) result.push(property.name.getText(source));
    }
    ts.forEachChild(node, visit);
  };
  visit(source);
  return result;
}

test("the actual copied general owner executes with every product directory absent", () => {
  const isolated = join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-strict-boundary/fixture-general-absent");
  rmSync(isolated, { recursive: true, force: true });
  mkdirSync(isolated, { recursive: true });
  for (const path of [corpus.generalOwner, ...corpus.generalInputs.map(row => row.path)]) {
    mkdirSync(dirname(join(isolated, path)), { recursive: true });
    cpSync(join(root, path), join(isolated, path));
    expect(digest(readFileSync(join(isolated, path)))).toBe(digest(readFileSync(join(root, path))));
  }
  symlinkSync(join(root, "node_modules"), join(isolated, "node_modules"), process.platform === "win32" ? "junction" : "dir");
  for (const area of corpus.productRoots) expect(existsSync(join(isolated, area))).toBe(false);
  const observed = spawnSync(process.execPath, ["test", join(isolated, corpus.generalOwner)], { cwd: isolated, encoding: "utf8", timeout: 6_000, env: process.env });
  expect(observed.error).toBeUndefined();
  expect(observed.status, observed.stderr).toBe(0);
  expect(observed.stderr).toContain("1 pass");
  expect(observed.stderr).toContain("0 fail");
});

test("the portable ownership schema preserves every original law, witness byte and concrete registration", () => {
  
  expect(corpus["version"]).toEqual(1);
  const general = parse(corpus.generalOwner);
  expect(laws(general)).toHaveLength(1);
  expect(digest(laws(general)[0]!.getText(general))).toBe(corpus.generalLawSha256);
  for (const row of corpus.moved) {
    const source = parse(row.owner);
    expect(laws(source)).toHaveLength(1);
    expect(digest(laws(source)[0]!.getText(source)), row.law).toBe(row.sha256);
    expect(read(corpus.generalOwner)).not.toContain(row.law);
    expect(registrations(parse(row.script))).toContain(row.command.split(" ")[0]!);
    const project = JSON.parse(read(row.project));
    expect(project.name).toBe(row.target.slice(0, row.target.lastIndexOf(":")));
    const target = project.targets[row.target.slice(row.target.lastIndexOf(":") + 1)];
    expect(target.options.command).toBe("bun ./📜️script.ts " + row.command);
    expect(target.cache).toBe(false);
    for (const input of row.inputs) expect(digest(readFileSync(join(root, input.path)))).toBe(input.sha256);
  }
  for (const input of corpus.generalInputs) expect(digest(readFileSync(join(root, input.path)))).toBe(input.sha256);
  const taxonomy = JSON.parse(read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"));
  for (const kind of corpus.directoryKinds) {
    expect(taxonomy.semanticDirectoryKinds[kind.id].emoji).toBe(kind.emoji);
    expect(taxonomy.semanticDirectoryKinds[kind.id].memberNames).toContain(kind.member);
    expect(taxonomy.semanticDirectoryKinds[kind.id].parentKindIds).toEqual(kind.parents);
  }
});
