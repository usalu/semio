/** 🏷️ Proves Flow browser source, declaration, publication and consumer ownership from one portable contract. */
import assert from "node:assert/strict";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import { fileURLToPath, pathToFileURL } from "node:url";
import Ajv from "ajv";

type Owner = Readonly<{ id: string; path: string; exports: readonly string[] }>;
type PackageExport = string | Readonly<{ types: string; import: string }>;
type Fixture = Readonly<{
  schemaVersion: 1;
  owners: readonly Owner[];
  testContract: Readonly<{ path: string; exports: readonly string[] }>;
  dataOwners: readonly string[];
  projections: readonly Readonly<{ sourceOwner: string; path: string; kind: string }>[];
  predecessors: readonly string[];
  contexts: readonly Readonly<{ ownerId: string; parentKindId: string; segments: readonly Readonly<{ name: string; kindId: string }>[] }>[];
  package: Readonly<{ manifestPath: string; name: string; files: readonly string[]; exports: Readonly<Record<string, PackageExport>> }>;
  consumers: readonly Readonly<{ path: string; tokens: readonly string[]; forbiddenTokens?: readonly string[] }>[];
  target: Readonly<{ projectPath: string; name: string; previewName: string; inputs: readonly string[] }>;
  registration: Readonly<{ packagePath: string; packageScript: string; previewPackageScript: string; launchSeedPath: string; launchPath: string; launchCommand: string; previewLaunchCommand: string }>;
}>;

const sourcePath = fileURLToPath(import.meta.url).replaceAll("\\", "/");
const workspaceMarker = "/🧰️framework/";
const workspaceRoot = sourcePath.slice(0, sourcePath.indexOf(workspaceMarker));
const ownershipRoot = dirname(dirname(sourcePath));
const schemaPath = join(ownershipRoot, "🧬️schema", "🔣️.json");
const fixturePath = join(ownershipRoot, "🧫️fixtures", "🔣️.json");

function sorted(values: readonly string[]): string[] {
  return [...values].sort((left, right) => left.localeCompare(right, "en"));
}

async function moduleExportNames(path: string): Promise<string[]> {
  const ts = await import("typescript");
  const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, path.endsWith(".js") ? ts.ScriptKind.JS : ts.ScriptKind.TS);
  const names = new Set<string>();
  for (const statement of source.statements) {
    if (ts.isExportDeclaration(statement) && statement.exportClause && ts.isNamedExports(statement.exportClause)) for (const element of statement.exportClause.elements) names.add(element.name.text);
    if (!statement.modifiers?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword)) continue;
    if ((ts.isFunctionDeclaration(statement) || ts.isClassDeclaration(statement) || ts.isInterfaceDeclaration(statement) || ts.isTypeAliasDeclaration(statement)) && statement.name) names.add(statement.name.text);
    if (ts.isVariableStatement(statement)) for (const declaration of statement.declarationList.declarations) if (ts.isIdentifier(declaration.name)) names.add(declaration.name.text);
  }
  return sorted([...names]);
}

/** 🧪️ Runs the portable Flow browser ownership, projection, package and bundle oracle. */
export async function testFlowBrowserOwnership(): Promise<void> {
  const schema = JSON.parse(readFileSync(schemaPath, "utf8"));
  const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Fixture;
  const ajv = new Ajv({ strict: true, allErrors: true });
  const validate = ajv.compile(schema);
  assert.equal(validate(fixture), true, JSON.stringify(validate.errors));
  assert.equal(fixture.owners.length, 4);

  for (const owner of fixture.owners) {
    const path = join(workspaceRoot, owner.path);
    assert.equal(existsSync(path), true, `missing Flow owner ${owner.id}: ${owner.path}`);
    assert.deepEqual(await moduleExportNames(path), sorted(owner.exports), `${owner.id} exports`);
  }
  assert.deepEqual(await moduleExportNames(join(workspaceRoot, fixture.testContract.path)), sorted(fixture.testContract.exports));
  for (const path of [...fixture.dataOwners, ...fixture.projections.map((projection) => projection.path)]) assert.equal(existsSync(join(workspaceRoot, path)), true, path);
  for (const path of fixture.predecessors) assert.equal(existsSync(join(workspaceRoot, path)), false, `retired Flow path remains: ${path}`);

  const discovery = await import(pathToFileURL(join(workspaceRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts")).href);
  const taxonomy = discovery.loadCatalogTaxonomy();
  for (const context of fixture.contexts) {
    const owner = fixture.owners.find((candidate) => candidate.id === context.ownerId);
    assert.ok(owner, context.ownerId);
    const directoryNames = dirname(owner.path).split("/");
    assert.deepEqual(directoryNames.slice(-context.segments.length), context.segments.map((segment) => segment.name), `${context.ownerId} context path`);
    let parentKindId: string | undefined = context.parentKindId;
    for (const segment of context.segments) {
      assert.equal(discovery.semanticDirectoryKindId(segment.name, taxonomy, parentKindId ? { parentKindId } : {}), segment.kindId, `${context.ownerId}:${segment.name}`);
      parentKindId = segment.kindId;
    }
  }

  for (const consumer of fixture.consumers) {
    const text = readFileSync(join(workspaceRoot, consumer.path), "utf8");
    for (const token of consumer.tokens) assert.ok(text.includes(token), `${consumer.path} missing ${token}`);
    for (const token of consumer.forbiddenTokens ?? []) assert.equal(text.includes(token), false, `${consumer.path} retains ${token}`);
  }

  const project = JSON.parse(readFileSync(join(workspaceRoot, fixture.target.projectPath), "utf8"));
  assert.deepEqual(project.targets[fixture.target.name].inputs, fixture.target.inputs);
  assert.equal(project.targets[fixture.target.name].cache, true);
  assert.equal(project.targets[fixture.target.name].options.command, "bun ./📜️script.ts test-browser-ownership");
  assert.deepEqual(project.targets[fixture.target.previewName].inputs, ["flowBrowserProduction"]);
  assert.deepEqual(project.targets[fixture.target.previewName].outputs, []);
  assert.equal(project.targets[fixture.target.previewName].options.command, "bun ./📜️script.ts preview-generated");
  assert.equal(project.targets[fixture.target.previewName].options.cwd, dirname(fixture.target.projectPath));
  const generator = taxonomy.generatorContracts["flow-browser-package"];
  assert.equal(generator.target, "semio-framework-os-flow-core:wasm");
  assert.equal(generator.previewTarget, `semio-framework-os-flow-core:${fixture.target.previewName}`);
  const buildPackage = JSON.parse(readFileSync(join(workspaceRoot, fixture.registration.packagePath), "utf8"));
  assert.equal(buildPackage.scripts[fixture.registration.packageScript], fixture.registration.launchCommand);
  assert.equal(buildPackage.scripts[fixture.registration.previewPackageScript], fixture.registration.previewLaunchCommand);
  for (const path of [fixture.registration.launchSeedPath, fixture.registration.launchPath]) {
    const source = readFileSync(join(workspaceRoot, path), "utf8");
    assert.ok(source.includes(fixture.registration.launchCommand), path);
    assert.ok(source.includes(fixture.registration.previewLaunchCommand), path);
  }

  const packageRoot = dirname(join(workspaceRoot, fixture.package.manifestPath));
  const manifest = JSON.parse(readFileSync(join(workspaceRoot, fixture.package.manifestPath), "utf8"));
  assert.equal(manifest.name, fixture.package.name);
  assert.deepEqual(sorted(manifest.files), sorted(fixture.package.files));
  assert.deepEqual(manifest.exports, fixture.package.exports);
  for (const name of fixture.package.files) assert.equal(existsSync(join(packageRoot, name)), true, name);

  const declarationOwner = fixture.owners.find((owner) => owner.id === "declaration-projection")!;
  const declaration = await import(`${pathToFileURL(join(workspaceRoot, declarationOwner.path)).href}?ownership=${Date.now()}`);
  const declarationText = declaration.flowBrowserDeclaration();
  const authoredDeclaration = fixture.projections.find((projection) => projection.kind === "authored-declaration")!;
  const browserExport = fixture.package.exports["./🌐️flow-browser.js"];
  const hostExport = fixture.package.exports["./🖥️flow-host.js"];
  if (typeof browserExport !== "object" || typeof hostExport !== "string") throw new Error("Flow browser package export kinds changed");
  assert.equal(readFileSync(join(workspaceRoot, authoredDeclaration.path), "utf8"), declarationText);
  assert.equal(readFileSync(join(packageRoot, browserExport.types), "utf8"), declarationText);

  const browserOwner = fixture.owners.find((owner) => owner.id === "browser-runtime")!;
  const hostOwner = fixture.owners.find((owner) => owner.id === "host-runtime")!;
  assert.equal(readFileSync(join(workspaceRoot, hostOwner.path), "utf8"), readFileSync(join(packageRoot, hostExport), "utf8"));
  const publisher = await import(`${pathToFileURL(join(workspaceRoot, fixture.owners.find((owner) => owner.id === "browser-publication")!.path)).href}?ownership=${Date.now()}`);
  const outputs = await publisher.bundleFlowBrowserModule(false);
  assert.equal(outputs.length, 1);
  const bundled = await outputs[0].text();
  assert.ok(bundled.includes('import("../flow_core.js")'));
  assert.ok(bundled.includes('from "../🖥️host/🟨️.js"'));
  assert.equal(bundled.includes(browserOwner.path), false);

  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir();
  mkdirSync(artifactRoot, { recursive: true });
  const sandbox = mkdtempSync(join(artifactRoot, "flow-browser-publication-"));
  const nativeFiles = ["flow_core_bg.wasm", "flow_core.js", "flow_core.d.ts", "flow_core_bg.wasm.d.ts"] as const;
  try {
    const family = join(sandbox, "family");
    const published = join(sandbox, "published");
    mkdirSync(family);
    for (const name of nativeFiles) copyFileSync(join(packageRoot, name), join(family, name));
    writeFileSync(join(family, "package.json"), `${JSON.stringify({ type: "module", version: "0.1.0", name: "@semio-tech/flow-core" })}\n`);
    await publisher.publishFlowBrowserPackage(family, published);
    assert.equal(existsSync(join(family, "package.json")), false);
    assert.equal(JSON.parse(readFileSync(join(published, "package.json"), "utf8")).name, fixture.package.name);

    const brokenFamily = join(sandbox, "broken-family");
    const preserved = join(sandbox, "preserved");
    mkdirSync(brokenFamily);
    mkdirSync(preserved);
    for (const name of nativeFiles.slice(0, -1)) copyFileSync(join(packageRoot, name), join(brokenFamily, name));
    writeFileSync(join(brokenFamily, "package.json"), '{"name":"@semio-tech/flow-core"}\n');
    writeFileSync(join(preserved, "package.json"), '{"name":"preserved"}\n');
    writeFileSync(join(preserved, "sentinel"), "unchanged\n");
    await assert.rejects(publisher.publishFlowBrowserPackage(brokenFamily, preserved), /did not emit flow_core_bg\.wasm\.d\.ts/u);
    assert.equal(readFileSync(join(preserved, "package.json"), "utf8"), '{"name":"preserved"}\n');
    assert.equal(readFileSync(join(preserved, "sentinel"), "utf8"), "unchanged\n");
    assert.equal(existsSync(join(brokenFamily, "package.json")), true);
  } finally {
    rmSync(sandbox, { recursive: true, force: true });
  }

  const ts = await import("typescript");
  const consumerPath = join(workspaceRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/consumer.ts");
  for (const [subpath, entry] of Object.entries(fixture.package.exports)) {
    if (typeof entry === "string") continue;
    const packageName = fixture.package.name + (subpath === "." ? "" : subpath.slice(1));
    const resolution = ts.resolveModuleName(packageName, consumerPath, { moduleResolution: ts.ModuleResolutionKind.Bundler, module: ts.ModuleKind.ESNext }, ts.sys).resolvedModule;
    assert.ok(resolution, packageName);
    assert.equal(realpathSync(resolution.resolvedFileName), realpathSync(join(packageRoot, entry.types)));
  }
  console.log(`[DEBUG] Flow browser ownership: ${fixture.owners.length} owners, ${fixture.consumers.length} consumers, ${fixture.target.inputs.length} exact Nx inputs, staged publication, pre-promotion preservation and native TypeScript/Bun projection parity PASS`);
}

if (import.meta.main) await testFlowBrowserOwnership();
