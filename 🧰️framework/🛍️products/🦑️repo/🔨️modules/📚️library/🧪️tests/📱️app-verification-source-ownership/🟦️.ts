import { describe, expect, test } from "bun:test";
import { basename, dirname, relative, resolve } from "node:path";
import { existsSync, readFileSync } from "node:fs";
import Ajv from "ajv";
import ts from "typescript";
import { fixedFilenameContractIdsForPath, fixedSourceDispositionDecision, loadCatalogTaxonomy, semanticDirectoryKindId } from "../../🔍️discovery/🟦️.ts";
import { fileSha256, oracleManifest, validateOracleManifest } from "../../../../../../../✏️s/🔌️plugins/🔋️energy/🔮️oracles/🛠️toolchain/🟦️.ts";

type Owner = Readonly<{ path: string; language: "typescript" | "python" | "json"; exports: readonly string[] }>;
type Fixture = Readonly<{
  owners: readonly Owner[];
  routers: readonly Readonly<{ path: string; maximumLines: number }>[];
  directoryContexts: readonly Readonly<{ name: string; parentKind: string; kind: string }>[];
  consumers: readonly Readonly<{ path: string; owners: readonly string[] }>[];
  sourceDataConsumers: readonly Readonly<{ path: string; owners: readonly string[]; evidence: string }>[];
  registration: Readonly<{ name: string; command: string; target: string }>;
}>;

const libraryRoot = resolve(import.meta.dir, "../..");
const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(libraryRoot, "🧫️fixtures/📱️app-verification-source-ownership/🔣️.json"), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(resolve(libraryRoot, "🧬️schema/📱️app-verification-source-ownership/🔣️.json"), "utf8"));

function exportedNames(path: string): ReadonlySet<string> {
  const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  expect(source.parseDiagnostics, path).toHaveLength(0);
  const names = new Set<string>();
  for (const statement of source.statements) {
    const exported = statement.modifiers?.some(({ kind }) => kind === ts.SyntaxKind.ExportKeyword);
    if (!exported) continue;
    if ((ts.isFunctionDeclaration(statement) || ts.isClassDeclaration(statement) || ts.isTypeAliasDeclaration(statement) || ts.isInterfaceDeclaration(statement)) && statement.name) names.add(statement.name.text);
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

describe("app verification source ownership", () => {
  test("validates the portable owner projection and every contextual kind", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...fixture, extra: true })).toBe(false);
    const taxonomy = loadCatalogTaxonomy();
    for (const row of fixture.directoryContexts) expect(semanticDirectoryKindId(row.name, taxonomy, { parentKindId: row.parentKind }), `${row.parentKind}/${row.name}`).toBe(row.kind);
  });

  test("keeps executable behavior under test or oracle owners with anonymous leaves", () => {
    expect(fixture.owners).toHaveLength(13);
    for (const owner of fixture.owners) {
      const path = resolve(repoRoot, owner.path);
      expect(existsSync(path), owner.path).toBe(true);
      expect(basename(path), owner.path).toBe(owner.language === "typescript" ? "🟦️.ts" : owner.language === "python" ? "🐍️.py" : "🔣️.json");
      if (owner.language === "python") {
        const source = readFileSync(path, "utf8");
        for (const name of owner.exports) expect(source).toMatch(new RegExp(`^def ${name}\\(`, "m"));
      } else if (owner.language === "typescript") {
        const names = exportedNames(path);
        for (const name of owner.exports) expect(names.has(name), `${owner.path}: ${name}`).toBe(true);
      } else {
        expect(() => JSON.parse(readFileSync(path, "utf8"))).not.toThrow();
      }
    }
  });

  test("keeps package command files as admitted routers without verifier or provisioning bodies", () => {
    const taxonomy = loadCatalogTaxonomy();
    const forbidden = ["function oracle(", "export async function proveGis", "export async function proveVcs", "function runOraclePython(", "ensureOracleTool("];
    for (const router of fixture.routers) {
      const path = resolve(repoRoot, router.path);
      const source = readFileSync(path, "utf8");
      expect(source.trimEnd().split("\n").length, router.path).toBeLessThanOrEqual(router.maximumLines);
      expect([...exportedNames(path)], router.path).toHaveLength(0);
      for (const marker of forbidden) expect(source.includes(marker), `${router.path}: ${marker}`).toBe(false);
      const contract = fixedFilenameContractIdsForPath(router.path, taxonomy)[0];
      expect(fixedSourceDispositionDecision(contract, source, taxonomy)?.finding, router.path).toBeNull();
    }
  });

  test("closes every direct app verifier consumer on semantic owners", () => {
    const owners = new Set(fixture.owners.filter(({ language }) => language === "typescript").map(({ path }) => path));
    const routers = new Set(fixture.routers.map(({ path }) => path));
    expect(fixture.consumers).toHaveLength(11);
    for (const consumer of fixture.consumers) {
      const modules = directRelativeModules(resolve(repoRoot, consumer.path));
      expect([...modules].filter((module) => owners.has(module)).sort(), consumer.path).toEqual([...consumer.owners].sort());
      expect([...modules].filter((module) => routers.has(module)), consumer.path).toHaveLength(0);
    }
    expect(fixture.sourceDataConsumers).toHaveLength(2);
    for (const consumer of fixture.sourceDataConsumers) {
      const source = readFileSync(resolve(repoRoot, consumer.path), "utf8");
      for (const owner of consumer.owners) expect(existsSync(resolve(repoRoot, owner)), owner).toBe(true);
      expect(source.includes(consumer.evidence), `${consumer.path}: ${consumer.evidence}`).toBe(true);
    }
  });

  test("preserves mathematical, GIS, VCS, and Energy portable authority", async () => {
    const mathematical = JSON.parse(readFileSync(resolve(repoRoot, "✏️s/🔌️plugins/➗️mathematical/🧫️fixtures/📣️publication-authority/🔣️.json"), "utf8"));
    expect(mathematical.routes).toHaveLength(6);
    expect(mathematical.routes.find(({ id }: any) => id === "nodeGraphViewport")?.lane).toBe("WindowConfig");
    const gisSchemaOwner = readFileSync(resolve(repoRoot, "✏️s/🔌️plugins/🌍️gis/🧬️schema/🟦️.ts"), "utf8");
    expect(gisSchemaOwner).toContain('keyword: "x-semio-child-kind"');
    const gisMapOwner = readFileSync(resolve(repoRoot, "✏️s/🔌️plugins/🌍️gis/🧪️tests/🧩️map-create-region-group/🟦️.ts"), "utf8");
    expect(gisMapOwner).toContain("GIS_MAP_SCHEMA_DEPENDENCIES");
    expect(gisMapOwner).toContain("GIS_MAP_CONTROL_SCHEMA_MODULE");
    const vcsCodecOwner = readFileSync(resolve(repoRoot, "✏️s/🔌️plugins/🌿️vcs/🧪️tests/📇️native-codecs/🟦️.ts"), "utf8");
    const vcsIdentityOwner = readFileSync(resolve(repoRoot, "✏️s/🔌️plugins/🌿️vcs/🧪️tests/🪪️native-openable-identity/🟦️.ts"), "utf8");
    expect(vcsCodecOwner).toContain('process.argv.includes("--oracle-only")');
    expect(vcsIdentityOwner).toContain('process.argv.includes("--oracle-only")');

    const packageRoot = resolve(repoRoot, "✏️s/🔌️plugins/🔋️energy/🔮️oracles/📦️packages/🐍️python");
    const manifestPath = resolve(repoRoot, "✏️s/🔌️plugins/🔋️energy/🔮️oracles/🛠️toolchain/🔣️.json");
    const manifest = oracleManifest(repoRoot);
    expect(manifest.tools[0]!.platforms[process.platform === "darwin" ? `darwin-${process.arch}` : `${process.platform}-${process.arch}`]).toBeDefined();
    const hostile = structuredClone(manifest) as any;
    hostile.tools[0].platforms["linux-x64"].sha256 = "0".repeat(64);
    expect(() => validateOracleManifest(hostile)).toThrow();
    const bytes = readFileSync(manifestPath);
    expect(await fileSha256(manifestPath)).toBe(Buffer.from(await crypto.subtle.digest("SHA-256", bytes)).toString("hex"));
    expect(existsSync(resolve(packageRoot, "🔣️.json"))).toBe(false);
    const contribution = JSON.parse(readFileSync(resolve(repoRoot, "✏️s/🔌️plugins/🔋️energy/🔮️oracles/🔣️.json"), "utf8"));
    expect(contribution.oracleHostPackages).toContainEqual(expect.objectContaining({ implementation: "python", path: "✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution", module: "🐍️" }));
    const execution = readFileSync(resolve(repoRoot, "✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🟦️.ts"), "utf8");
    expect(execution).toContain("entry.path === PYTHON_OWNER");
    expect(execution).toContain("import_module");
    expect(execution).toContain("env.PYTHONPATH");
    expect(readFileSync(resolve(repoRoot, "✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🐍️.py"), "utf8")).toContain("@see ../🛠️toolchain/🔣️.json");
    expect(existsSync(resolve(packageRoot, "🔮️oracles/🐍️.py"))).toBe(false);
    const registrations = readFileSync(resolve(repoRoot, "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json"), "utf8");
    expect(registrations).toContain("✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🐍️.py");
    expect(registrations).toContain("✏️s/🔌️plugins/🔋️energy/🔮️oracles/🛠️toolchain/🔣️.json");
    expect(registrations).not.toContain("📦️packages/🐍️python/🔮️oracles/🐍️.py");
    expect(registrations).not.toContain("🔮️oracles/📦️packages/🐍️python/🔣️.json");
  });

  test("registers the portable gate through package, Nx, and both launch projections", () => {
    const packageRoot = resolve(libraryRoot, "📦️packages/🟦️typescript");
    const project = JSON.parse(readFileSync(resolve(packageRoot, "📋️project.json"), "utf8"));
    expect(project.targets[fixture.registration.target]?.options?.command).toBe("bun ./📜️script.ts test app-verification-source-ownership");
    const manifest = JSON.parse(readFileSync(resolve(packageRoot, "package.json"), "utf8"));
    expect(manifest.scripts[fixture.registration.target]).toBe(`nx run @semio-tech/repo-lib:${fixture.registration.target}`);
    const commandSource = readFileSync(resolve(packageRoot, "📜️script.ts"), "utf8");
    expect(commandSource).toContain('segments[0] === "app-verification-source-ownership"');
    for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
      const launch = Bun.JSONC.parse(readFileSync(resolve(repoRoot, path), "utf8")) as { configurations: { name?: string; command?: string }[] };
      expect(launch.configurations.filter(({ name, command }) => name === fixture.registration.name && command === fixture.registration.command), path).toHaveLength(1);
    }
  });
});
