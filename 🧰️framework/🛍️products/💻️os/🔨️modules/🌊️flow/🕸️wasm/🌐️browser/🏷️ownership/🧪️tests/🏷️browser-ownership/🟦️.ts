import { receiveScriptProcessInvocation, type ScriptInvocation } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🏷️ Proves Flow browser source, declaration, publication and consumer ownership from one portable contract. */
import assert from "node:assert/strict";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { tmpdir } from "node:os";
import { fileURLToPath, pathToFileURL } from "node:url";

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
  registration: Readonly<{ packagePath: string; packageScript: string; previewPackageScript: string; packageCommand: string; previewPackageCommand: string }>;
}>;

const sourcePath = fileURLToPath(import.meta.url).replaceAll("\\", "/");
const workspaceMarker = "/🧰️framework/";
const workspaceRoot = sourcePath.slice(0, sourcePath.indexOf(workspaceMarker));
const ownershipRoot = dirname(dirname(dirname(sourcePath)));
const fixturePath = join(ownershipRoot, "🧪️tests/🧫️fixtures", "🔣️.json");

function sorted(values: readonly string[]): string[] {
  return [...values].sort((left, right) => left.localeCompare(right, "en"));
}

async function moduleExportNames(path: string): Promise<string[]> {
  const ts = await import("typescript");
  const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, path.endsWith(".js") ? ts.ScriptKind.JS : ts.ScriptKind.TS);
  const names = new Set<string>();
  for (const statement of source.statements) {
    if (ts.isExportDeclaration(statement) && statement.exportClause && ts.isNamedExports(statement.exportClause)) for (const element of statement.exportClause.elements) names.add(element.name.text);
    if (!(ts.canHaveModifiers(statement) ? ts.getModifiers(statement) : undefined)?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword)) continue;
    if ((ts.isFunctionDeclaration(statement) || ts.isClassDeclaration(statement) || ts.isInterfaceDeclaration(statement) || ts.isTypeAliasDeclaration(statement)) && statement.name) names.add(statement.name.text);
    if (ts.isVariableStatement(statement)) for (const declaration of statement.declarationList.declarations) if (ts.isIdentifier(declaration.name)) names.add(declaration.name.text);
  }
  return sorted([...names]);
}

/** 🧪️ Runs the portable Flow browser ownership, projection, package and bundle oracle. */
export async function testFlowBrowserOwnership(invocation: ScriptInvocation): Promise<void> {
  await testColdFlowDeclaration();
  const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Fixture;
  assert.equal(fixture.owners.length, 4);

  for (const owner of fixture.owners) {
    const path = join(workspaceRoot, owner.path);
    assert.equal(existsSync(path), true, `missing Flow owner ${owner.id}: ${owner.path}`);
    assert.deepEqual(await moduleExportNames(path), sorted(owner.exports), `${owner.id} exports`);
  }
  assert.deepEqual(await moduleExportNames(join(workspaceRoot, fixture.testContract.path)), sorted(fixture.testContract.exports));
  for (const path of [...fixture.dataOwners, ...fixture.projections.map((projection) => projection.path)]) assert.equal(existsSync(join(workspaceRoot, path)), true, path);
  for (const path of fixture.predecessors) assert.equal(existsSync(join(workspaceRoot, path)), false, `retired Flow path remains: ${path}`);

  const discovery = await import(pathToFileURL(join(workspaceRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts")).href);
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
  assert.equal(generator.target, "semio-framework-os-flow-browser-package:wasm");
  assert.equal(generator.previewTarget, `semio-framework-os-flow-browser-package:${fixture.target.previewName}`);
  const buildPackage = JSON.parse(readFileSync(join(workspaceRoot, fixture.registration.packagePath), "utf8"));
  assert.equal(buildPackage.scripts[fixture.registration.packageScript], fixture.registration.packageCommand);
  assert.equal(buildPackage.scripts[fixture.registration.previewPackageScript], fixture.registration.previewPackageCommand);

  const declarationOwner = fixture.owners.find((owner) => owner.id === "declaration-projection")!;
  const declaration = await import(`${pathToFileURL(join(workspaceRoot, declarationOwner.path)).href}?ownership=${Date.now()}`);
  const declarationText = declaration.flowBrowserDeclaration();
  const authoredDeclaration = fixture.projections.find((projection) => projection.kind === "authored-declaration")!;
  const browserExport = fixture.package.exports["./🌐️flow-browser.js"];
  const hostExport = fixture.package.exports["./🖥️flow-host.js"];
  if (typeof browserExport !== "object" || typeof hostExport !== "string") throw new Error("Flow browser package export kinds changed");
  assert.equal(readFileSync(join(workspaceRoot, authoredDeclaration.path), "utf8"), declarationText);
  const publisher = await import(pathToFileURL(join(workspaceRoot, fixture.owners.find(owner=>owner.id==="browser-publication")!.path)).href);
  const manifest=publisher.flowBrowserPackageManifestV1("0.1.0");
  assert.equal(manifest.name,fixture.package.name);assert.deepEqual(sorted(manifest.files),sorted(fixture.package.files));assert.deepEqual(manifest.exports,fixture.package.exports);
  const database=new(await import("bun:sqlite")).Database(":memory:");try{database.exec("create table files(name text primary key)");for(const name of fixture.package.files)database.query("insert into files values(?)").run(name);assert.deepEqual(database.query("select count(*) count from files").get(),{count:manifest.files.length});}finally{database.close();}
  for(const snippets of [["../escape"],["snippets/a","snippets/a"],["snippets/../escape"]])assert.throws(()=>publisher.flowBrowserPackageManifestV1("0.1.0",snippets));
  assert.ok(Object.isFrozen(manifest));assert.ok(Object.isFrozen(manifest.exports));assert.ok(Object.isFrozen(manifest.exports["."]));
  assert.throws(()=>publisher.publishFlowBrowserPackage(Object.freeze({}),invocation));
  const ts=await import("typescript"),syntax=ts.createSourceFile("flow.d.ts",declarationText,ts.ScriptTarget.Latest,true);
  assert.equal((syntax as unknown as{parseDiagnostics:readonly unknown[]}).parseDiagnostics.length,0);
  const publicationSource=ts.createSourceFile("publication.ts",readFileSync(join(workspaceRoot,fixture.owners.find(owner=>owner.id==="browser-publication")!.path),"utf8"),ts.ScriptTarget.Latest,true),calls=new Set<string>();
  const visit=(node:import("typescript").Node):void=>{if(ts.isCallExpression(node))calls.add(node.expression.getText(publicationSource));ts.forEachChild(node,visit);};visit(publicationSource);
  for(const name of ["withProducedWasmPackageTaskV1","withRepositoryArtifactV1","withRepositoryArtifactTaskV1","withRepositoryArtifactDestinationV1","stageRepositoryArtifacts"])assert.ok(calls.has(name),name);
  console.log(`Flow browser ownership: ${fixture.owners.length} owners, ${fixture.consumers.length} consumers, ${fixture.target.inputs.length} exact Nx inputs, finite manifest DATA, private producer refusal and independent TypeScript/SQLite source parity PASS`);
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, testFlowBrowserOwnership);

/** 🧊️ Publishes the ABI declaration into absent parent directories and admits it through TypeScript. */
async function testColdFlowDeclaration():Promise<void>{
  const compiler=await import("typescript"),projectionPath=join(workspaceRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/📝️declaration/📤️projection/🟦️.ts");
  const source=compiler.createSourceFile(projectionPath,readFileSync(projectionPath,"utf8"),compiler.ScriptTarget.Latest,true),operation=source.statements.find(node=>compiler.isFunctionDeclaration(node)&&node.name?.text==="writeFlowBrowserDeclaration")!;
  const {flowBrowserDeclaration}=await import(pathToFileURL(projectionPath).href);
  const output=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();mkdirSync(output,{recursive:true});
  const sandbox=mkdtempSync(join(output,"flow-cold-declaration-")),declaration=join(sandbox,"absent/🤖️generated/🟦️.d.ts"),expected=flowBrowserDeclaration();
  const run=new Function("fileURLToPath","declarationUrl","writeFileSync","mkdirSync","dirname","flowBrowserDeclaration",compiler.transpileModule(operation.getText(source).replace(/^export /,""),{compilerOptions:{target:compiler.ScriptTarget.ES2022}}).outputText+";return writeFlowBrowserDeclaration;")(fileURLToPath,pathToFileURL(declaration),writeFileSync,mkdirSync,dirname,flowBrowserDeclaration);
  assert.equal(run(),declaration);assert.equal(readFileSync(declaration,"utf8"),expected);
  const syntax=compiler.createSourceFile(declaration,expected,compiler.ScriptTarget.Latest,true);
  assert.equal((syntax as unknown as {parseDiagnostics:readonly unknown[]}).parseDiagnostics.length,0);
  assert.ok(syntax.statements.some(node=>compiler.isInterfaceDeclaration(node)&&node.name.text==="FlowBrowserRuntime"));
}
