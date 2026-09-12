import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { relative, resolve } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import { loadTaxonomy, semanticDirectoryKindId } from "../../📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../../");
const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🧱️root-taxonomy-workflow-source/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧬️schema/🧱️root-taxonomy-workflow-source/🔣️.json"), "utf8"));
const relativeSpecifier = (consumer: string, owner: string): string => {
  const path = relative(resolve(repoRoot, consumer, ".."), resolve(repoRoot, owner)).replaceAll("\\", "/");
  return path.startsWith(".") ? path : `./${path}`;
};
const exportedNames = (path: string): Set<string> => {
  const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS), names = new Set<string>();
  for (const statement of source.statements) {
    if (!statement.modifiers?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword)) continue;
    if ((ts.isFunctionDeclaration(statement) || ts.isClassDeclaration(statement) || ts.isInterfaceDeclaration(statement) || ts.isTypeAliasDeclaration(statement) || ts.isEnumDeclaration(statement)) && statement.name) names.add(statement.name.text);
    if (ts.isVariableStatement(statement)) for (const declaration of statement.declarationList.declarations) if (ts.isIdentifier(declaration.name)) names.add(declaration.name.text);
  }
  return names;
};

test("validates the portable owner and consumer contract", () => {
  expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
  for (const owner of fixture.owners) {
    const path = resolve(repoRoot, owner.path);
    expect(existsSync(path), owner.path).toBe(true);
    const exports = exportedNames(path);
    for (const name of owner.exports) expect(exports.has(name), `${owner.path}:${name}`).toBe(true);
  }
});

test("resolves every semantic owner through its exact parent context", () => {
  const taxonomy = loadTaxonomy();
  for (const context of fixture.contexts) expect(semanticDirectoryKindId(context.directoryName, taxonomy, { parentKindId: context.parentKindId }), `${context.parentKindId}/${context.directoryName}`).toBe(context.kindId);
});

test("removes root implementation declarations and binds direct consumers", () => {
  const rootSource = readFileSync(resolve(repoRoot, "📜️script.ts"), "utf8"), rootTree = ts.createSourceFile("📜️script.ts", rootSource, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const forbidden = new Set(fixture.owners.flatMap((owner: { exports: string[] }) => owner.exports));
  for (const statement of rootTree.statements) {
    const names: string[] = [];
    if ((ts.isFunctionDeclaration(statement) || ts.isClassDeclaration(statement) || ts.isInterfaceDeclaration(statement) || ts.isTypeAliasDeclaration(statement)) && statement.name) names.push(statement.name.text);
    if (ts.isVariableStatement(statement)) for (const declaration of statement.declarationList.declarations) if (ts.isIdentifier(declaration.name)) names.push(declaration.name.text);
    for (const name of names) expect(forbidden.has(name), `root declaration ${name}`).toBe(false);
  }
  for (const consumer of fixture.consumers) {
    const source = readFileSync(resolve(repoRoot, consumer.path), "utf8");
    for (const owner of consumer.owners) expect(source.includes(relativeSpecifier(consumer.path, owner)) || source.includes(owner), `${consumer.path} -> ${owner}`).toBe(true);
  }
});

test("registers one Bun Nx and launch route", () => {
  const packageRoot = resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript"), route = fixture.route;
  const project = JSON.parse(readFileSync(resolve(packageRoot, "📋️project.json"), "utf8"));
  expect(project.targets[route.target]?.options?.command).toBe(route.command);
  expect(JSON.parse(readFileSync(resolve(packageRoot, "package.json"), "utf8")).scripts[route.target]).toBe(`nx run @semio-tech/repo-lib:${route.target}`);
  const seed = readFileSync(resolve(repoRoot, ".vscode/🧩️launch.seed.jsonc"), "utf8"), launch = readFileSync(resolve(repoRoot, ".vscode/launch.json"), "utf8");
  expect(seed.split(route.launchName).length - 1).toBe(1);
  expect(launch.split(route.launchName).length - 1).toBe(1);
  expect(seed).toContain(route.launchCommand); expect(launch).toContain(route.launchCommand);
});
