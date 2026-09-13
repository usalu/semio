import { expect, test } from "bun:test";
import Ajv from "ajv";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import ts from "typescript";
import { loadTaxonomy, semanticDirectoryKindId } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import { localRelayExecutionTargetAsset, localRelayInferencePath, localRelaySpaceArtifactCreationPath, localRelayUpstreamPath } from "../../🚀️local-relay/🧭️routing/🟦️.ts";
import { directorySocketGrantDecision, type DirectorySocketGrantVector } from "../../📇️directory/🔐️authorization/🔌️socket-grant/🧭️decision/🟦️.ts";
import { assertSocketGrantNativeLawSources, socketGrantNativeLawPlan } from "../../📇️directory/🔐️authorization/🔌️socket-grant/🧪️tests/📋️native-law-plan/🟦️.ts";
import { runSocketGrantCheck } from "../../📇️directory/🔐️authorization/🔌️socket-grant/🧪️tests/🏃️execution/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../..");
const hubRoot = join(repoRoot, "🌎️hub");
const schemaPath = join(hubRoot, "🧬️schema/🧱️socket-grant-command-source/🔣️.json");
const fixturePath = join(hubRoot, "🧫️fixtures/🧱️socket-grant-command-source/🔣️.json");
const fixtureSource = readFileSync(fixturePath, "utf8");
const fixture = JSON.parse(fixtureSource) as {
  readonly schemaVersion: 1;
  readonly owners: readonly { readonly path: string; readonly language: "typescript" | "rust"; readonly declarations: readonly string[]; readonly imports: readonly string[]; readonly rootImports: readonly string[]; readonly contextChain: readonly string[] }[];
  readonly contexts: readonly { readonly directoryName: string; readonly parentKindId: string; readonly kindId: string }[];
  readonly routes: Readonly<Record<"source" | "native", { readonly command: string; readonly target: string; readonly launchName: string; readonly launchCommand: string; readonly launchGroup: string; readonly launchOrder: number; readonly inputs: readonly string[] }>>;
  readonly nativeStages: readonly { readonly id: string; readonly args: readonly string[]; readonly source: string | null; readonly declaration: string | null }[];
};

function exportedDeclarations(path: string): Set<string> {
  const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const names = new Set<string>();
  source.forEachChild((node) => {
    if (
      (ts.isFunctionDeclaration(node) || ts.isClassDeclaration(node) || ts.isInterfaceDeclaration(node) || ts.isTypeAliasDeclaration(node) || ts.isVariableStatement(node)) &&
      node.modifiers?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword)
    ) {
      if ("name" in node && node.name) names.add(node.name.text);
      if (ts.isVariableStatement(node)) for (const declaration of node.declarationList.declarations) if (ts.isIdentifier(declaration.name)) names.add(declaration.name.text);
    }
  });
  return names;
}

function internalOwnerImports(path: string, owners: ReadonlySet<string>): string[] {
  const absolute = join(repoRoot, path);
  const source = ts.createSourceFile(path, readFileSync(absolute, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  return source.statements
    .flatMap((node) => {
      if (!ts.isImportDeclaration(node) || !ts.isStringLiteral(node.moduleSpecifier) || !node.moduleSpecifier.text.startsWith(".")) return [];
      const imported = `${relative(repoRoot, resolve(dirname(absolute), node.moduleSpecifier.text)).replaceAll("\\", "/")}.ts`.replace(".ts.ts", ".ts");
      return owners.has(imported) ? [imported] : [];
    })
    .sort();
}

function rootImportsForOwner(routerPath: string, ownerPath: string): string[] {
  const source = ts.createSourceFile(routerPath, readFileSync(routerPath, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const owner = join(repoRoot, ownerPath);
  return source.statements.flatMap((node) => {
    if (!ts.isImportDeclaration(node) || !ts.isStringLiteral(node.moduleSpecifier) || !node.moduleSpecifier.text.startsWith(".")) return [];
    if (resolve(dirname(routerPath), node.moduleSpecifier.text) !== owner) return [];
    const bindings = node.importClause?.namedBindings;
    return bindings && ts.isNamedImports(bindings) ? bindings.elements.map((element) => element.name.text) : [];
  });
}

test("socket-grant command contract is schema-first and independently parsed", async () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(schemaPath, "utf8")));
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...fixture, nativeStages: fixture.nativeStages.slice(1) })).toBe(false);
  expect(validate({ ...fixture, owners: fixture.owners.slice(1) })).toBe(false);
  const jsonc = await import("jsonc-parser");
  const errors: import("jsonc-parser").ParseError[] = [];
  expect(jsonc.parse(fixtureSource, errors, { allowTrailingComma: false, disallowComments: true })).toEqual(fixture);
  expect(errors).toEqual([]);
});

test("seven anonymous owners are exported, acyclic, and type-correct", () => {
  const ownerPaths = new Set(fixture.owners.map((owner) => owner.path));
  expect(fixture.owners).toHaveLength(7);
  for (const owner of fixture.owners) {
    const absolute = join(repoRoot, owner.path);
    expect(existsSync(absolute), owner.path).toBe(true);
    if (owner.language === "rust") {
      const source = readFileSync(absolute, "utf8");
      for (const declaration of owner.declarations) expect(source).toContain(`fn ${declaration}(`);
      continue;
    }
    expect(absolute.endsWith("/🟦️.ts")).toBe(true);
    const declarations = exportedDeclarations(absolute);
    for (const declaration of owner.declarations) expect(declarations.has(declaration), `${owner.path}:${declaration}`).toBe(true);
    expect(internalOwnerImports(owner.path, ownerPaths), owner.path).toEqual([...owner.imports].sort());
  }
  const visiting = new Set<string>();
  const visited = new Set<string>();
  const edges = new Map(fixture.owners.map((owner) => [owner.path, owner.imports]));
  const visit = (path: string): void => {
    if (visiting.has(path)) throw new Error(`cycle at ${path}`);
    if (visited.has(path)) return;
    visiting.add(path);
    for (const dependency of edges.get(path) ?? []) visit(dependency);
    visiting.delete(path);
    visited.add(path);
  };
  for (const path of ownerPaths) visit(path);
  expect(visited.size).toBe(7);

  const paths = fixture.owners.filter((owner) => owner.language === "typescript").map((owner) => join(repoRoot, owner.path));
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
  expect(paths.flatMap((path) => [...program.getSyntacticDiagnostics(program.getSourceFile(path)), ...program.getSemanticDiagnostics(program.getSourceFile(path))]).map((diagnostic) => ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n"))).toEqual([]);
  const routerPath = join(hubRoot, "📦️packages/🦀️rust/📜️script.ts");
  for (const path of paths) {
    const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    for (const statement of source.statements) if (ts.isImportDeclaration(statement) && ts.isStringLiteral(statement.moduleSpecifier) && statement.moduleSpecifier.text.startsWith(".")) expect(resolve(dirname(path), statement.moduleSpecifier.text), path).not.toBe(routerPath);
  }
});

test("owner paths bind all eighteen taxonomy contexts", () => {
  const taxonomy = loadTaxonomy();
  for (const context of fixture.contexts) expect(semanticDirectoryKindId(context.directoryName, taxonomy, { parentKindId: context.parentKindId }), context.kindId).toBe(context.kindId);
  for (const owner of fixture.owners) {
    const segments = relative(hubRoot, join(repoRoot, owner.path)).replaceAll("\\", "/").split("/").slice(0, -1);
    const directories = owner.contextChain.map((kindId) => fixture.contexts.find((context) => context.kindId === kindId)!.directoryName);
    expect(segments.slice(-directories.length), owner.path).toEqual(directories);
  }
});

test("socket decisions and relay routes reject hostile scope substitutions", () => {
  const decisionFixture = JSON.parse(readFileSync(join(hubRoot, "📇️directory/🧫️fixtures/🔌️scoped-socket-revocation-v1/🔣️.json"), "utf8"));
  expect(decisionFixture.vectors).toHaveLength(19);
  for (const vector of decisionFixture.vectors) expect(directorySocketGrantDecision(vector as DirectorySocketGrantVector), vector.name).toEqual({ outcome: vector.expected, closeCode: vector.closeCode, cursorAdvance: vector.cursorAdvance, textFrames: vector.textFrames });
  const exact = "/directory/spaces/space%2Fa/documents/document%20b/socket-grants";
  expect(localRelayUpstreamPath("POST", new URL(`http://relay.invalid/_semio/hub${exact}`))).toBe(exact);
  expect(localRelayUpstreamPath("GET", new URL(`http://relay.invalid/_semio/hub${exact}`))).toBeUndefined();
  expect(localRelayUpstreamPath("POST", new URL(`http://relay.invalid/_semio/hub${exact}?extra=1`))).toBeUndefined();
  expect(localRelayExecutionTargetAsset("/spaces/space-a/documents/document-a/execution-target/browser-actor")).toBe("browser-actor");
  expect(localRelayExecutionTargetAsset("/spaces/../documents/document-a/execution-target/manifest")).toBeUndefined();
  expect(localRelaySpaceArtifactCreationPath("POST", `/spaces/space-a/artifact-creations/${"a".repeat(32)}/cancel`)).toBe(true);
  expect(localRelaySpaceArtifactCreationPath("POST", `/spaces/space-a/artifact-creations/${"0".repeat(32)}/cancel`)).toBe(false);
  expect(localRelayInferencePath("GET", `/spaces/space-a/documents/document-a/inference/gis-map/jobs/${"a".repeat(32)}/events?after=0`)).toBe(true);
  expect(localRelayInferencePath("GET", `/spaces/space-a/documents/document-a/inference/gis-map/jobs/${"a".repeat(32)}/events?after=999999999999`)).toBe(false);
});

test("native law planning is source-bound, ordered, cancellable, and phase-exact", async () => {
  const plan = socketGrantNativeLawPlan();
  expect(plan).toEqual(fixture.nativeStages);
  expect(plan).toHaveLength(19);
  expect(plan.filter((stage) => stage.source === "🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs")).toHaveLength(15);
  expect(plan.some((stage) => stage.declaration?.includes("forced_lag_is_scope"))).toBe(false);
  expect(() => assertSocketGrantNativeLawSources(repoRoot)).not.toThrow();
  const packageRoot = join(hubRoot, "📦️packages/🦀️rust");
  const oracleCalls: string[] = [];
  await runSocketGrantCheck(repoRoot, packageRoot, ["oracle"], async (stage) => void oracleCalls.push(stage.id));
  expect(oracleCalls).toEqual([]);
  for (const segments of [["unknown"], ["oracle", "extra"]]) await expect(runSocketGrantCheck(repoRoot, packageRoot, segments, async () => undefined)).rejects.toThrow(/no arguments or oracle/u);
  const calls: { id: string; stack: string | undefined }[] = [];
  await runSocketGrantCheck(repoRoot, packageRoot, [], async (stage, _root, env) => void calls.push({ id: stage.id, stack: env.RUST_MIN_STACK }));
  expect(calls.map((call) => call.id)).toEqual(plan.map((stage) => stage.id));
  expect(new Set(calls.map((call) => call.stack))).toEqual(new Set(["268435456"]));
  const interrupted: string[] = [];
  await expect(runSocketGrantCheck(repoRoot, packageRoot, [], async (stage) => {
    interrupted.push(stage.id);
    if (interrupted.length === 3) throw new Error("injected cancellation");
  })).rejects.toThrow(/injected cancellation/u);
  expect(interrupted).toEqual(plan.slice(0, 3).map((stage) => stage.id));
});

test("Rust oracle independently parses exact test declarations and hostile lookalikes", () => {
  const oracle = readFileSync(join(hubRoot, "📇️directory/🔐️authorization/🔌️socket-grant/🧪️tests/🔮️oracles/🦀️.rs"), "utf8");
  const library = readFileSync(join(hubRoot, "📦️packages/🦀️rust/🦀️.rs"), "utf8");
  expect(oracle).toContain("syn::parse_file");
  expect(oracle).toContain("test_declarations(hostile)");
  expect(oracle).toContain("!exact_declaration(&duplicates");
  expect(oracle).toContain("hub_socket_grant_fixture_serde_parity");
  expect(library).toContain('socket-grant/🧪️tests/🔮️oracles/🦀️.rs"]');
});

test("package, cache input, targets, and launch records bind only the moved owners", async () => {
  const routerPath = join(hubRoot, "📦️packages/🦀️rust/📜️script.ts");
  const router = readFileSync(routerPath, "utf8");
  const routerAst = ts.createSourceFile(routerPath, router, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const declared = new Set<string>();
  routerAst.forEachChild((node) => {
    if ((ts.isFunctionDeclaration(node) || ts.isClassDeclaration(node) || ts.isTypeAliasDeclaration(node)) && node.name) declared.add(node.name.text);
    if (ts.isVariableStatement(node)) for (const declaration of node.declarationList.declarations) if (ts.isIdentifier(declaration.name)) declared.add(declaration.name.text);
  });
  for (const owner of fixture.owners) {
    for (const declaration of owner.declarations) expect(declared.has(declaration), declaration).toBe(false);
    if (owner.language === "typescript") expect(rootImportsForOwner(routerPath, owner.path), owner.path).toEqual(owner.rootImports);
  }
  expect(router).toContain(`.register("${fixture.routes.source.command}", HubSocketGrantCommandSourceScript)`);
  expect(router).toContain(`.register("${fixture.routes.native.command}", SocketGrantCheckScript)`);
  const project = JSON.parse(readFileSync(join(hubRoot, "📦️packages/🦀️rust/📋️project.json"), "utf8"));
  expect(project.namedInputs.hubSocketGrantCommandSources).toEqual(fixture.routes.source.inputs);
  expect(project.targets[fixture.routes.source.target]?.inputs).toEqual(["hubSocketGrantCommandSources"]);
  expect(project.targets[fixture.routes.source.target]?.options?.command).toBe("bun ./📜️script.ts socket-grant-command-source-check");
  expect(project.targets[fixture.routes.native.target]?.inputs).toEqual(["hubSocketGrantCommandSources"]);
  expect(project.targets[fixture.routes.native.target]?.options?.command).toBe("bun ./📜️script.ts socket-grant-check");
  const jsonc = await import("jsonc-parser");
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const launch = jsonc.parse(readFileSync(join(repoRoot, path), "utf8"));
    for (const route of Object.values(fixture.routes)) {
      const matches = launch.configurations.filter((entry: { name?: string }) => entry.name === route.launchName);
      expect(matches, `${path}:${route.launchName}`).toHaveLength(1);
      expect(matches[0].command).toBe(route.launchCommand);
      expect(matches[0].presentation).toEqual({ group: route.launchGroup, order: route.launchOrder });
    }
  }
});
