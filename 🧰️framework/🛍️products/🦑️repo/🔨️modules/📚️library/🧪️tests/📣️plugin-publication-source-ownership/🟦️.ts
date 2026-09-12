import { describe, expect, test } from "bun:test";
import { basename, dirname, relative, resolve } from "node:path";
import { existsSync, readFileSync } from "node:fs";
import Ajv from "ajv";
import ts from "typescript";
import { semanticDirectoryKindId } from "../../🔍️discovery/🟦️.ts";

type Owner = Readonly<{ path: string; exports: readonly string[] }>;
type Fixture = Readonly<{
  owners: readonly Owner[];
  routers: readonly Readonly<{ path: string; maximumLines: number }>[];
  directoryContexts: readonly Readonly<{ name: string; parentKind: string; kind: string }>[];
  consumers: readonly Readonly<{ path: string; owners: readonly string[] }>[];
  dependencyEdges: Readonly<{
    required: readonly Readonly<{ from: string; to: string }>[];
    forbidden: readonly Readonly<{ from: string; to: string }>[];
  }>;
  registration: Readonly<{ name: string; command: string; target: string }>;
}>;

const libraryRoot = resolve(import.meta.dir, "../..");
const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(libraryRoot, "🧫️fixtures/📣️plugin-publication-source-ownership/🔣️.json"), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(resolve(libraryRoot, "🧬️schema/📣️plugin-publication-source-ownership/🔣️.json"), "utf8"));

function exportedNames(path: string): ReadonlySet<string> {
  const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  expect(source.parseDiagnostics, path).toHaveLength(0);
  const names = new Set<string>();
  for (const statement of source.statements) {
    if (ts.isExportDeclaration(statement) && statement.exportClause && ts.isNamedExports(statement.exportClause)) for (const element of statement.exportClause.elements) names.add(element.name.text);
    const exported = statement.modifiers?.some(({ kind }) => kind === ts.SyntaxKind.ExportKeyword);
    if (!exported) continue;
    if ((ts.isFunctionDeclaration(statement) || ts.isClassDeclaration(statement) || ts.isTypeAliasDeclaration(statement) || ts.isInterfaceDeclaration(statement) || ts.isEnumDeclaration(statement)) && statement.name) names.add(statement.name.text);
    if (ts.isVariableStatement(statement)) for (const declaration of statement.declarationList.declarations) if (ts.isIdentifier(declaration.name)) names.add(declaration.name.text);
  }
  return names;
}

function directRelativeModules(path: string): ReadonlySet<string> {
  const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  expect(source.parseDiagnostics, path).toHaveLength(0);
  const modules = new Set<string>();
  const add = (specifier: ts.Expression | undefined) => {
    if (!specifier || !ts.isStringLiteral(specifier) || !specifier.text.startsWith(".")) return;
    modules.add(relative(repoRoot, resolve(dirname(path), specifier.text)).replaceAll("\\", "/"));
  };
  const visit = (node: ts.Node) => {
    if (ts.isImportDeclaration(node)) add(node.moduleSpecifier);
    if (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword) add(node.arguments[0]);
    ts.forEachChild(node, visit);
  };
  visit(source);
  return modules;
}

function directRuntimeModules(path: string): ReadonlySet<string> {
  const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const modules = new Set<string>();
  const add = (specifier: ts.Expression | undefined) => {
    if (!specifier || !ts.isStringLiteral(specifier) || !specifier.text.startsWith(".")) return;
    const target = resolve(dirname(path), specifier.text);
    if (existsSync(target)) modules.add(target);
  };
  const visit = (node: ts.Node) => {
    if (ts.isImportDeclaration(node)) {
      const clause = node.importClause;
      const onlyTypes = clause?.isTypeOnly || Boolean(clause && !clause.name && clause.namedBindings && ts.isNamedImports(clause.namedBindings) && clause.namedBindings.elements.every(({ isTypeOnly }) => isTypeOnly));
      if (!onlyTypes) add(node.moduleSpecifier);
    }
    if (ts.isExportDeclaration(node) && !node.isTypeOnly) add(node.moduleSpecifier);
    if (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword) add(node.arguments[0]);
    ts.forEachChild(node, visit);
  };
  visit(source);
  return modules;
}

describe("plugin publication source ownership", () => {
  test("validates the portable owner projection and every contextual kind", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...fixture, extra: true })).toBe(false);
    const taxonomy = JSON.parse(readFileSync(resolve(libraryRoot, "🔣️taxonomy.json"), "utf8"));
    for (const row of fixture.directoryContexts) expect(semanticDirectoryKindId(row.name, taxonomy, { parentKindId: row.parentKind }), `${row.parentKind}/${row.name}`).toBe(row.kind);
  });

  test("owns implementation bodies under anonymous semantic leaves", () => {
    expect(fixture.owners).toHaveLength(15);
    for (const owner of fixture.owners) {
      const path = resolve(repoRoot, owner.path);
      expect(existsSync(path), owner.path).toBe(true);
      expect(basename(path), owner.path).toBe("🟦️.ts");
      const names = exportedNames(path);
      for (const name of owner.exports) expect(names.has(name), `${owner.path}: ${name}`).toBe(true);
    }
  });

  test("keeps package command files as bounded routers without public API", () => {
    for (const router of fixture.routers) {
      const path = resolve(repoRoot, router.path);
      const source = readFileSync(path, "utf8");
      expect(source.trimEnd().split("\n").length, router.path).toBeLessThanOrEqual(router.maximumLines);
      expect([...exportedNames(path)], router.path).toHaveLength(0);
    }
  });

  test("closes every direct consumer on semantic owners", () => {
    const owners = new Set(fixture.owners.map(({ path }) => path));
    const routers = new Set(fixture.routers.map(({ path }) => path));
    expect(fixture.consumers).toHaveLength(63);
    for (const consumer of fixture.consumers) {
      const path = resolve(repoRoot, consumer.path);
      expect(existsSync(path), consumer.path).toBe(true);
      const modules = directRelativeModules(path);
      expect([...modules].filter((module) => owners.has(module)).sort(), consumer.path).toEqual([...consumer.owners].sort());
      expect([...modules].filter((module) => routers.has(module)), consumer.path).toHaveLength(0);
    }
  });

  test("keeps discovery and publication dependencies acyclic", () => {
    for (const edge of fixture.dependencyEdges.required) expect(directRelativeModules(resolve(repoRoot, edge.from)).has(edge.to), `${edge.from} -> ${edge.to}`).toBe(true);
    for (const edge of fixture.dependencyEdges.forbidden) expect(directRelativeModules(resolve(repoRoot, edge.from)).has(edge.to), `${edge.from} -/> ${edge.to}`).toBe(false);
    const roots = [
      resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe"),
      resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry"),
    ];
    const state = new Map<string, 1 | 2>();
    const stack: string[] = [];
    const cycles: string[][] = [];
    const visit = (path: string) => {
      if (state.get(path) === 2) return;
      if (state.get(path) === 1) {
        cycles.push(stack.slice(stack.indexOf(path)).concat(path).map((entry) => relative(repoRoot, entry).replaceAll("\\", "/")));
        return;
      }
      state.set(path, 1);
      stack.push(path);
      for (const target of directRuntimeModules(path)) if (roots.some((root) => target.startsWith(`${root}/`))) visit(target);
      stack.pop();
      state.set(path, 2);
    };
    for (const row of [...fixture.owners, ...fixture.routers]) visit(resolve(repoRoot, row.path));
    expect(cycles).toEqual([]);
  });

  test("registers the gate in package, Nx, and launch authorities", () => {
    const packageRoot = resolve(libraryRoot, "📦️packages/🟦️typescript");
    const project = JSON.parse(readFileSync(resolve(packageRoot, "📋️project.json"), "utf8"));
    expect(project.targets[fixture.registration.target]?.options?.command).toBe("bun ./📜️script.ts test plugin-publication-source-ownership");
    const manifest = JSON.parse(readFileSync(resolve(packageRoot, "package.json"), "utf8"));
    expect(manifest.scripts[fixture.registration.target]).toBe(`nx run @semio-tech/repo-lib:${fixture.registration.target}`);
    const commandSource = readFileSync(resolve(packageRoot, "📜️script.ts"), "utf8");
    expect(commandSource).toContain('segments[0] === "plugin-publication-source-ownership"');
    for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
      const launch = Bun.JSONC.parse(readFileSync(resolve(repoRoot, path), "utf8")) as { readonly configurations: readonly Readonly<{ name?: string; command?: string }>[] };
      expect(launch.configurations.filter(({ name, command }) => name === fixture.registration.name && command === fixture.registration.command), path).toHaveLength(1);
    }
  });
});
