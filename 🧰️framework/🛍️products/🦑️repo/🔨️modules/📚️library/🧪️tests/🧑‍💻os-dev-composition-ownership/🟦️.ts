import { describe, expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import Ajv from "ajv/dist/2020.js";
import ts from "typescript";
import { loadCatalogTaxonomy, loadTaxonomy, semanticDirectoryKindId } from "../../🔍️discovery/🟦️.ts";

type Fixture = Readonly<{
  schemaVersion: 1;
  contractContext: Readonly<{ name: string; kind: string; parentKinds: readonly string[] }>;
  routerPath: string;
  owners: readonly Readonly<{ id: string; path: string; exports: readonly string[] }>[];
  contextChains: readonly Readonly<{ owner: string; parentKind: string; members: readonly Readonly<{ name: string; kind: string }>[] }>[];
  ownerImporterPath: string;
  consumers: readonly Readonly<{ id: string; mode: "direct-import" | "dynamic-import" | "source-data" | "detached-import"; path: string; requiredTokens: readonly string[]; forbiddenTokens: readonly string[]; minimumOccurrences?: number }>[];
  projectInputs: readonly Readonly<{ project: string; inputs: readonly string[] }>[];
  generatorContracts: readonly Readonly<{ id: "dev-distribution-bundle" | "playground-session" | "scale-fixture"; ownerPath: string; ownerIds: readonly string[] }>[];
  generatedBoundaries: readonly Readonly<{ id: "shard-worker" | "module-bridge"; filename: string; constant: "SHARD_WORKER_FILE" | "MODULE_BRIDGE_FILE"; authorityPath: string; producerOwner: "browser-host-staging"; producerToken: string }>[];
  routerForbiddenSymbols: readonly string[];
  registration: Readonly<{ target: string; name: string; command: string; packagePath: string; projectPath: string; seedLaunchPath: string; derivedLaunchPath: string }>;
}>;

const libraryRoot = resolve(import.meta.dir, "../..");
const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(libraryRoot, "🧫️fixtures/🧑‍💻os-dev-composition-ownership/🔣️.json"), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(resolve(libraryRoot, "🧬️schema/🧑‍💻os-dev-composition-ownership/🔣️.json"), "utf8"));

describe("OS development composition ownership", () => {
  test("validates the exact portable semantic owner map", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...fixture, extra: true })).toBe(false);
    expect(fixture.owners).toHaveLength(49);
    expect(new Set(fixture.owners.map(({ id }) => id)).size).toBe(49);
    expect(new Set(fixture.owners.map(({ path }) => path)).size).toBe(49);
    expect(fixture.contextChains.map(({ owner }) => owner).sort()).toEqual(fixture.owners.map(({ id }) => id).sort());
    expect(new Set(fixture.consumers.map(({ id }) => id)).size).toBe(fixture.consumers.length);
    expect(new Set(fixture.projectInputs.map(({ project }) => project)).size).toBe(fixture.projectInputs.length);
    const taxonomy = loadCatalogTaxonomy();
    for (const parentKind of fixture.contractContext.parentKinds) expect(semanticDirectoryKindId(fixture.contractContext.name, taxonomy, { parentKindId: parentKind })).toBe(fixture.contractContext.kind);
  });

  test("materializes every anonymous semantic implementation owner", () => {
    for (const owner of fixture.owners) {
      expect(owner.path.includes("/📦️packages/"), owner.path).toBe(false);
      expect(owner.path.endsWith("/🟦️.ts"), owner.path).toBe(true);
      expect(existsSync(resolve(repoRoot, owner.path)), owner.path).toBe(true);
    }
  });

  test("leaves the package command as routing without domain behavior or test injection", () => {
    const router = readFileSync(resolve(repoRoot, fixture.routerPath), "utf8");
    for (const symbol of fixture.routerForbiddenSymbols) expect(router, symbol).not.toMatch(new RegExp(`(?:class|function|const|type|interface)\\s+${symbol}\\b`));
    expect(router).not.toContain("import.meta.vitest");
    expect(router).not.toContain("export { blake3Hex, Blake3Hasher }");
    expect(router).toContain("new ScriptRouter(import.meta.dir)");
    expect(router).toContain("runBundleScriptMain(router, import.meta.url");
  });

  test("resolves every owner through its complete registered semantic ancestry", () => {
    const taxonomy = loadCatalogTaxonomy();
    const owners = new Map(fixture.owners.map((owner) => [owner.id, owner]));
    for (const chain of fixture.contextChains) {
      const owner = owners.get(chain.owner)!;
      expect(dirname(owner.path).endsWith(chain.members.map(({ name }) => name).join("/")), `${chain.owner} ancestry must be the suffix of ${owner.path}`).toBe(true);
      let parentKind = chain.parentKind;
      for (const member of chain.members) {
        expect(semanticDirectoryKindId(member.name, taxonomy, { parentKindId: parentKind }), `${chain.owner}: ${parentKind}/${member.name}`).toBe(member.kind);
        parentKind = member.kind;
      }
    }
  });

  test("imports all 49 behavior owners directly into their existing verification concern", () => {
    const importerPath = resolve(repoRoot, fixture.ownerImporterPath);
    const source = ts.createSourceFile(importerPath, readFileSync(importerPath, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const ownerPaths = new Set(fixture.owners.map(({ path }) => resolve(repoRoot, path)));
    const importedOwners = source.statements.flatMap((statement) => {
      if (!ts.isImportDeclaration(statement) || !ts.isStringLiteral(statement.moduleSpecifier) || !statement.moduleSpecifier.text.startsWith(".")) return [];
      const imported = resolve(dirname(importerPath), statement.moduleSpecifier.text);
      return ownerPaths.has(imported) ? [imported] : [];
    });
    expect(new Set(importedOwners)).toEqual(ownerPaths);
    expect(importedOwners).toHaveLength(49);
  });

  test("publishes every declared owner API and binds router imports to that public map", () => {
    const ownerByPath = new Map(fixture.owners.map((owner) => [resolve(repoRoot, owner.path), owner]));
    for (const [path, owner] of ownerByPath) {
      const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
      const exported = new Set<string>();
      for (const statement of source.statements) {
        if (ts.isExportDeclaration(statement) && statement.exportClause && ts.isNamedExports(statement.exportClause)) for (const element of statement.exportClause.elements) exported.add(element.name.text);
        if (!statement.modifiers?.some(({ kind }) => kind === ts.SyntaxKind.ExportKeyword)) continue;
        if ((ts.isFunctionDeclaration(statement) || ts.isClassDeclaration(statement) || ts.isInterfaceDeclaration(statement) || ts.isTypeAliasDeclaration(statement) || ts.isEnumDeclaration(statement)) && statement.name) exported.add(statement.name.text);
        if (ts.isVariableStatement(statement)) for (const declaration of statement.declarationList.declarations) if (ts.isIdentifier(declaration.name)) exported.add(declaration.name.text);
      }
      for (const name of owner.exports) expect(exported, `${owner.id}: ${name}`).toContain(name);
    }
    const routerPath = resolve(repoRoot, fixture.routerPath);
    const router = ts.createSourceFile(routerPath, readFileSync(routerPath, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    for (const statement of router.statements) {
      if (!ts.isImportDeclaration(statement) || !ts.isStringLiteral(statement.moduleSpecifier) || !statement.moduleSpecifier.text.startsWith(".")) continue;
      const owner = ownerByPath.get(resolve(dirname(routerPath), statement.moduleSpecifier.text));
      if (!owner || !statement.importClause?.namedBindings || !ts.isNamedImports(statement.importClause.namedBindings)) continue;
      for (const element of statement.importClause.namedBindings.elements) expect(owner.exports, `${owner.id} router import ${element.name.text}`).toContain(element.propertyName?.text ?? element.name.text);
    }
  });

  test("binds dynamic, detached and source-data consumers to the real owners", () => {
    for (const consumer of fixture.consumers) {
      const source = readFileSync(resolve(repoRoot, consumer.path), "utf8");
      for (const token of consumer.requiredTokens) expect(source, `${consumer.id} requires ${token}`).toContain(token);
      for (const token of consumer.forbiddenTokens) expect(source, `${consumer.id} forbids ${token}`).not.toContain(token);
      if (consumer.minimumOccurrences) {
        const token = consumer.requiredTokens.at(-1)!;
        expect(source.split(token).length - 1, `${consumer.id} occurrences of ${token}`).toBeGreaterThanOrEqual(consumer.minimumOccurrences);
      }
    }
  });

  test("declares every external owner as an exact Nx cache input", () => {
    for (const contract of fixture.projectInputs) {
      const project = JSON.parse(readFileSync(resolve(repoRoot, contract.project), "utf8")) as { namedInputs?: { default?: string[] } };
      for (const input of contract.inputs) expect(project.namedInputs?.default, `${contract.project}: ${input}`).toContain(input);
    }
  });

  test("binds generated output contracts to their semantic source owners", () => {
    const taxonomy = loadTaxonomy();
    const owners = new Map(fixture.owners.map((owner) => [owner.id, owner]));
    for (const expected of fixture.generatorContracts) {
      const actual = taxonomy.generatorContracts[expected.id];
      expect(actual.ownerPath, expected.id).toBe(expected.ownerPath);
      for (const ownerId of expected.ownerIds) expect(actual.inputPatterns, `${expected.id}: ${ownerId}`).toContain(owners.get(ownerId)!.path);
      expect(new Set(actual.inputPatterns).size, expected.id).toBe(actual.inputPatterns.length);
      expect([...actual.inputPatterns].sort((left, right) => Buffer.from(left).compare(Buffer.from(right))), expected.id).toEqual(actual.inputPatterns);
    }
  });

  test("registers one Bun/Nx route in package, project, seed and derived launch authorities", () => {
    const registration = fixture.registration;
    const packageManifest = JSON.parse(readFileSync(resolve(repoRoot, registration.packagePath), "utf8")) as { scripts?: Record<string, string> };
    expect(packageManifest.scripts?.[registration.target]).toBe(`nx run @semio-tech/repo-lib:${registration.target}`);
    const project = JSON.parse(readFileSync(resolve(repoRoot, registration.projectPath), "utf8")) as { targets?: Record<string, { inputs?: string[]; options?: { command?: string } }> };
    const target = project.targets?.[registration.target];
    expect(target?.options?.command).toBe("bun ./📜️script.ts test os-dev-composition-ownership");
    const contractPaths = new Set([
      "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧑‍💻os-dev-composition-ownership/🔣️.json",
      "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧑‍💻os-dev-composition-ownership/🔣️.json",
      "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧑‍💻os-dev-composition-ownership/🟦️.ts",
      "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
      fixture.routerPath,
      fixture.ownerImporterPath,
      registration.packagePath,
      registration.projectPath,
      registration.seedLaunchPath,
      registration.derivedLaunchPath,
      ...fixture.owners.map(({ path }) => path),
      ...fixture.consumers.map(({ path }) => path),
      ...fixture.projectInputs.map(({ project }) => project),
      ...fixture.generatedBoundaries.map(({ authorityPath }) => authorityPath),
    ]);
    const expectedInputs = ["sharedGlobals", ...[...contractPaths].map((path) => `{workspaceRoot}/${path}`)].sort();
    expect([...(target?.inputs ?? [])].sort()).toEqual(expectedInputs);
    const packageRouter = readFileSync(resolve(repoRoot, dirname(registration.projectPath), "📜️script.ts"), "utf8");
    expect(packageRouter).toContain('segments[0] === "os-dev-composition-ownership"');
    expect(packageRouter).toContain("🧪️tests/🧑‍💻os-dev-composition-ownership/🟦️.ts");
    for (const path of [registration.seedLaunchPath, registration.derivedLaunchPath]) {
      const launch = Bun.JSONC.parse(readFileSync(resolve(repoRoot, path), "utf8")) as { configurations: readonly { name?: string; command?: string }[] };
      expect(launch.configurations.filter(({ name, command }) => name === registration.name && command === registration.command), path).toHaveLength(1);
    }
  });

  test("keeps exact generated boundary names visible to their staging producer", () => {
    const owners = new Map(fixture.owners.map((owner) => [owner.id, owner]));
    for (const boundary of fixture.generatedBoundaries) {
      const authority = readFileSync(resolve(repoRoot, boundary.authorityPath), "utf8");
      const producer = readFileSync(resolve(repoRoot, owners.get(boundary.producerOwner)!.path), "utf8");
      expect(authority, boundary.id).toContain(`export const ${boundary.constant} = ${JSON.stringify(boundary.filename)}`);
      expect(producer, boundary.id).toContain(boundary.constant);
      expect(producer, boundary.id).toContain(boundary.producerToken);
    }
  });
});
