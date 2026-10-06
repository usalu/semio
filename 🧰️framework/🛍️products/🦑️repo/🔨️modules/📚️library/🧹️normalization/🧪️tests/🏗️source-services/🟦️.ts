import Ajv from "ajv";
import { expect, test } from "bun:test";
import { build } from "esbuild";
import { readFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import ts from "typescript";

type Owner = Readonly<{ path: string; dependencies: readonly string[] }>;
const root = resolve(import.meta.dir, "../.."), workspace = resolve(root, "../../../../../..");
const fixture = JSON.parse(readFileSync(join(root, "🧫️fixtures/🏗️source-services/🔣️.json"), "utf8")) as Readonly<{ schemaVersion: 1; owners: readonly Owner[]; denied: readonly string[]; directTypeOwners: readonly Readonly<{ name: string; path: string }>[]; taxonomyLoad: Readonly<{ input: "required"; parser: "private"; facts: readonly string[] }> }>;

const paths = fixture.owners.map((owner) => owner.path);
const slash = (path: string): string => path.replaceAll("\\", "/");

test("canonical source owner records satisfy an independent closed schema", () => {
  
  expect(fixture["schemaVersion"]).toEqual(1);expect(fixture["owners"]).toEqual([{"path":"🛣️path/🟦️.ts","dependencies":[]},{"path":"📁️input/🟦️.ts","dependencies":[]},{"path":"🏃️operation/🟦️.ts","dependencies":[]},{"path":"🔣️taxonomy/🟦️.ts","dependencies":["🛣️path/🟦️.ts","📁️input/🟦️.ts"]},{"path":"🚪️source-admission/🟦️.ts","dependencies":["🛣️path/🟦️.ts"]},{"path":"🚪️source-admission/📁️io/🟦️.ts","dependencies":["🛣️path/🟦️.ts","📁️input/🟦️.ts","🏃️operation/🟦️.ts","🔣️taxonomy/🟦️.ts","🚪️source-admission/🟦️.ts"]}]);expect(fixture["denied"]).toEqual(["🟦️.ts","🧬️mutation/"]);expect(fixture["directTypeOwners"]).toEqual([{"name":"TaxonomyProgress","path":"🏃️operation/🟦️.ts"},{"name":"TaxonomyNodeKind","path":"🚪️source-admission/🟦️.ts"}]);expect(fixture["taxonomyLoad"]).toEqual({"input":"required","parser":"private","facts":["schema","discoverySchema","exclusions","fileKinds","directoryKinds"]});
  expect(new Set(paths).size).toBe(6);
  for (const owner of fixture.owners) for (const dependency of owner.dependencies) expect(paths).toContain(dependency);
  
  
  
  
  
  
  
  
});

test("current canonical source owners have exact acyclic imports and no forwarding exports", async () => {
  const edges = new Map<string, readonly string[]>();
  for (const owner of fixture.owners) {
    const path = join(root, owner.path), source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
    const imports = source.statements.filter(ts.isImportDeclaration).map((node) => (node.moduleSpecifier as ts.StringLiteral).text);
    const owned = imports.filter((specifier) => specifier.startsWith(".")).map((specifier) => slash(relative(root, resolve(dirname(path), specifier)))).filter((dependency) => paths.includes(dependency));
    expect([...new Set(owned)].sort(), owner.path).toEqual([...owner.dependencies].sort());
    expect(source.statements.filter(ts.isExportDeclaration), owner.path).toEqual([]);
    edges.set(owner.path, owned);
  }
  const visit = (path: string, ancestors: readonly string[]): void => { expect(ancestors, path).not.toContain(path); for (const dependency of edges.get(path) ?? []) visit(dependency, [...ancestors, path]); };
  for (const path of paths) visit(path, []);
  const output = await build({ absWorkingDir: workspace, entryPoints: paths.map((path) => join(root, path)), bundle: true, write: false, outdir: join(root, "unused"), metafile: true, platform: "node", format: "esm", packages: "external", logLevel: "silent" });
  const captured = Object.keys(output.metafile!.inputs).map((path) => slash(relative(root, isAbsolute(path) ? path : resolve(workspace, path))));
  for (const denied of fixture.denied) expect(captured.filter((path) => path === denied || path.startsWith(denied)), denied).toEqual([]);
  const external = Object.values(output.metafile!.outputs).flatMap((file) => file.imports).filter((entry) => entry.external);
  for (const entry of external) expect(entry.path.startsWith("node:"), entry.path).toBe(true);
});


test("source service types expose only their canonical declaration owners", () => {
  const path = join(root, "🟦️.ts"), source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
  expect(source.statements.filter(ts.isExportDeclaration).map((node) => node.getText(source))).toEqual([]);
  const program = ts.createProgram([path, ...fixture.directTypeOwners.map((owner) => join(root, owner.path))], { module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, target: ts.ScriptTarget.ESNext, allowImportingTsExtensions: true, noEmit: true, noResolve: true, noLib: true, types: [] });
  const checker = program.getTypeChecker();
  const exports = checker.getExportsOfModule(checker.getSymbolAtLocation(program.getSourceFile(path)!)!);
  for (const owner of fixture.directTypeOwners) {
    const ownerPath = join(root, owner.path), ownerSource = program.getSourceFile(ownerPath)!;
    const symbol = checker.getExportsOfModule(checker.getSymbolAtLocation(ownerSource)!).find((item) => item.name === owner.name);
    expect(symbol, owner.name).toBeDefined();
    expect(symbol!.declarations?.map((declaration) => slash(relative(root, declaration.getSourceFile().fileName)))).toEqual([owner.path]);
    expect(exports.filter((item) => item.name === owner.name || (item.flags & ts.SymbolFlags.Alias) !== 0 && checker.getAliasedSymbol(item) === symbol).map((item) => item.name)).toEqual([]);
  }
});

test("loaded taxonomy requires a physical input while its parser stays private", () => {
  const path = join(root, "🔣️taxonomy/🟦️.ts"), source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
  const loaded = source.statements.filter((node): node is ts.InterfaceDeclaration => ts.isInterfaceDeclaration(node) && node.name.text === "LoadedTaxonomy");
  expect(loaded).toHaveLength(1);
  const inputs = loaded[0]!.members.filter((node): node is ts.PropertySignature => ts.isPropertySignature(node) && node.name.getText(source) === "input");
  expect(inputs).toHaveLength(1);
  expect(inputs[0]!.questionToken === undefined ? "required" : "optional").toBe(fixture.taxonomyLoad.input);
  const parser = source.statements.filter((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === "parseTaxonomy");
  expect(parser).toHaveLength(1);
  expect(parser[0]!.modifiers?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword) ? "public" : "private").toBe(fixture.taxonomyLoad.parser);
  expect(parser[0]!.type?.getText(source)).toBe("TaxonomyContentFacts");
  const facts = source.statements.filter((node): node is ts.InterfaceDeclaration => ts.isInterfaceDeclaration(node) && node.name.text === "TaxonomyContentFacts");
  expect(facts).toHaveLength(1);
  expect(facts[0]!.members.map((member) => member.name?.getText(source))).toEqual([...fixture.taxonomyLoad.facts]);
  const returned = parser[0]!.body!.statements.filter(ts.isReturnStatement);
  expect(returned).toHaveLength(1);
  expect(returned[0]!.expression && ts.isObjectLiteralExpression(returned[0]!.expression)).toBe(true);
  const value = returned[0]!.expression as ts.ObjectLiteralExpression;
  expect(value.properties.map((property) => property.name?.getText(source))).toEqual([...fixture.taxonomyLoad.facts]);
});
