import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import { loadTaxonomy, semanticDirectoryKindId } from "../../📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../../");
const libraryRoot = resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🧱️root-surface-abstraction-law-source/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧬️schema/🧱️root-surface-abstraction-law-source/🔣️.json"), "utf8"));

function tsIsNamed(statement: ts.Statement): statement is ts.FunctionDeclaration | ts.TypeAliasDeclaration | ts.InterfaceDeclaration | ts.ClassDeclaration {
  return ts.isFunctionDeclaration(statement) || ts.isTypeAliasDeclaration(statement) || ts.isInterfaceDeclaration(statement) || ts.isClassDeclaration(statement);
}

function declarations(path: string): string[] {
  const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  return source.statements
    .flatMap((statement) => {
      if (tsIsNamed(statement) && statement.name) return [statement.name.text];
      if (ts.isVariableStatement(statement)) return statement.declarationList.declarations.flatMap((declaration) => (ts.isIdentifier(declaration.name) ? [declaration.name.text] : []));
      return [];
    })
    .sort();
}

function relativeSpecifier(consumer: string, owner: string): string {
  const path = relative(resolve(repoRoot, dirname(consumer)), resolve(libraryRoot, owner)).replaceAll("\\", "/");
  return path.startsWith(".") ? path : `./${path}`;
}

test("validates the portable surface and abstraction ownership contract", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(JSON.parse(JSON.stringify(fixture))).toEqual(fixture);
  expect(fixture.owners).toHaveLength(17);
  expect(new Set(fixture.owners.map((owner: { path: string }) => owner.path)).size).toBe(fixture.owners.length);
  expect(ts.parseJsonText("fixture.json", JSON.stringify(fixture)).parseDiagnostics).toEqual([]);
});

test("resolves every owner through its exact semantic context", { timeout: 30_000 }, () => {
  const taxonomy = loadTaxonomy();
  for (const context of fixture.contexts) expect(semanticDirectoryKindId(context.directoryName, taxonomy, { parentKindId: context.parentKindId }), JSON.stringify(context)).toBe(context.kindId);
  for (const owner of fixture.owners) {
    expect(semanticDirectoryKindId(owner.directoryName, taxonomy, { parentKindId: owner.parentKindId }), owner.path).toBe(owner.kindId);
    expect(owner.path.split("/").at(-1)).toBe("🟦️.ts");
    expect(declarations(resolve(repoRoot, owner.path))).toEqual([...owner.declarations].sort());
  }
});

test("typechecks all surface and abstraction owners", { timeout: 30_000 }, () => {
  const paths = fixture.owners.map((owner: { path: string }) => resolve(repoRoot, owner.path));
  const program = ts.createProgram(paths, {
    target: ts.ScriptTarget.ESNext,
    module: ts.ModuleKind.ESNext,
    moduleResolution: ts.ModuleResolutionKind.Bundler,
    strict: true,
    noUncheckedIndexedAccess: true,
    allowImportingTsExtensions: true,
    skipLibCheck: true,
    noEmit: true,
    types: ["node"],
  });
  const diagnostics = paths.flatMap((path: string) => {
    const source = program.getSourceFile(path);
    expect(source).toBeDefined();
    return source ? [...program.getSyntacticDiagnostics(source), ...program.getSemanticDiagnostics(source)] : [];
  });
  expect(diagnostics.map((diagnostic) => ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n"))).toEqual([]);
});

test("removes root implementations and binds direct command consumers", () => {
  const moved = new Set(fixture.owners.flatMap((owner: { declarations: string[] }) => owner.declarations));
  expect(declarations(resolve(repoRoot, "📜️script.ts")).filter((name) => moved.has(name))).toEqual([]);
  for (const consumer of fixture.consumers) {
    const source = readFileSync(resolve(repoRoot, consumer.path), "utf8");
    for (const owner of consumer.owners) expect(source.includes(relativeSpecifier(consumer.path, owner)) || source.includes(owner), `${consumer.path} -> ${owner}`).toBe(true);
  }
});

test("keeps the owner graph acyclic and free of root back imports", () => {
  const owners = new Set(fixture.owners.map((owner: { path: string }) => resolve(repoRoot, owner.path)));
  const edges = new Map<string, string[]>([...owners].map((owner) => [owner, []]));
  for (const owner of owners) {
    const syntax = ts.createSourceFile(owner, readFileSync(owner, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    for (const statement of syntax.statements) {
      const specifier = ts.isImportDeclaration(statement) && ts.isStringLiteral(statement.moduleSpecifier) ? statement.moduleSpecifier.text : null;
      if (!specifier?.startsWith(".")) continue;
      const target = resolve(dirname(owner), specifier);
      expect(target).not.toBe(resolve(repoRoot, "📜️script.ts"));
      if (owners.has(target)) edges.get(owner)!.push(target);
    }
  }
  const visiting = new Set<string>(),
    visited = new Set<string>();
  const visit = (owner: string): void => {
    if (visiting.has(owner)) throw new Error(`surface/abstraction owner cycle at ${owner}`);
    if (visited.has(owner)) return;
    visiting.add(owner);
    for (const target of edges.get(owner)!) visit(target);
    visiting.delete(owner);
    visited.add(owner);
  };
  for (const owner of owners) visit(owner);
  expect(visited.size).toBe(owners.size);
});

test("preserves distinct artifact-root and dialect inventories and surfaces unreadable roots", async () => {
  const [{ policyListPluginArtifactDirs }, { policyListArtifactDialectDirs }, { policyDiscoverAppSchemaOwners }, { policyAppSchemaConfigRelocationBreaches }] = await Promise.all([
    import("../../🔍️discovery/🗿️artifact/🏠️roots/🟦️.ts"),
    import("../../🔍️discovery/🗿️artifact/🗣️dialects/🟦️.ts"),
    import("../../🧬️schema/🗺️surface/🔍️owner-discovery/🟦️.ts"),
    import("../../🧬️schema/🗺️surface/⚖️laws/🚚️config-relocation/🟦️.ts"),
  ]);
  const roots = policyListPluginArtifactDirs(repoRoot),
    dialects = policyListArtifactDialectDirs(repoRoot);
  expect(roots.length).toBeGreaterThan(0);
  expect(dialects.length).toBeGreaterThan(0);
  expect(dialects.every((dialect) => roots.includes(dialect.artRel))).toBe(true);
  expect(dialects.some((dialect) => !roots.includes(dialect.subsetRel))).toBe(true);
  const operations = {
    lstat: (path: string) => {
      const rel = relative("/repo", path).replaceAll("\\", "/");
      if (rel === "✏️s/🔌️plugins") throw Object.assign(new Error(rel), { code: "EACCES" });
      if (["", "✏️s"].includes(rel)) return { isFile: false, isDirectory: true, isSymbolicLink: false };
      throw Object.assign(new Error(rel), { code: "ENOENT" });
    },
    readFile: () => "",
    readdir: () => [],
  };
  expect(() => policyListPluginArtifactDirs("/repo", operations)).toThrow("unreadable");
  expect(() => policyDiscoverAppSchemaOwners("/repo", operations)).toThrow("unreadable");
  expect(policyAppSchemaConfigRelocationBreaches("/repo", operations)).toEqual([expect.objectContaining({ kind: "app-schema/source-unreadable", scope: "✏️s/🔌️plugins" })]);
});

test("retains abstraction source-data and Ajv proof behavior", { timeout: 30_000 }, async () => {
  const { abstractionOwnershipChecks } = await import("../../📏️ownership/🏛️abstraction/✅️verification/🟦️.ts");
  expect(abstractionOwnershipChecks(repoRoot)).toBe(11);
  const sources = [
    readFileSync(resolve(libraryRoot, "📏️ownership/🏛️abstraction/🧱️contract/🟦️.ts"), "utf8"),
    readFileSync(resolve(libraryRoot, "📏️ownership/🏛️abstraction/✅️verification/🟦️.ts"), "utf8"),
    readFileSync(resolve(libraryRoot, "🧬️schema/🗿️artifact/⚖️laws/🪪️ownership-field-parity/🟦️.ts"), "utf8"),
  ].join("\n");
  for (const path of fixture.sourceData) expect(sources).toContain(path);
});

test("retains live surface law behavior without freezing diagnostic totals", { timeout: 30_000 }, async () => {
  const { policyAppSchemaBreaches } = await import("../../🧬️schema/🗺️surface/⚖️laws/📋️aggregate/🟦️.ts"),
    breaches = policyAppSchemaBreaches(repoRoot);
  expect(breaches.length).toBeGreaterThan(0);
  expect(breaches.every((breach) => fixture.lawKinds.includes(breach.kind))).toBe(true);
});

test("registers one Bun Nx and seed-derived launch route", () => {
  const project = JSON.parse(readFileSync(resolve(libraryRoot, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
  const packageJson = JSON.parse(readFileSync(resolve(libraryRoot, "📦️packages/🟦️typescript/package.json"), "utf8"));
  expect(project.targets[fixture.route.target]?.options.command).toBe(fixture.route.command);
  expect(packageJson.scripts[fixture.route.target]).toBe(`nx run @semio-tech/repo-lib:${fixture.route.target}`);
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const source = readFileSync(resolve(repoRoot, path), "utf8");
    expect(source.split(fixture.route.launchName).length - 1).toBe(1);
    expect(source).toContain(fixture.route.launchCommand);
  }
});
