import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { join, relative } from "node:path";
import ts from "typescript";
import { getWorkspaceRoot } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { type JsonMap, readStdioJson, slashStdioPath, STDIO_RELATIVE_ROOT, stdioArtifactDefinitionPaths, stdioArtifactPackageContract } from "../../🗿️artifacts/📇️inventory/🟦️.ts";
import { assertStdioArtifactSchema, assertStdioArtifactSourceContract } from "../../🗿️artifacts/🛂️contract/🟦️.ts";
import { assertStdioArtifactCargoMetadata, assertStdioArtifactNxGraph, stdioCargoMetadata } from "../../🗿️artifacts/🕸️graph/🟦️.ts";

type CommandOwnership = Readonly<{
  owners: readonly Readonly<{ id: string; path: string; exports: readonly string[] }>[];
  routers: readonly Readonly<{ id: "root" | "composition-package"; path: string; defaultCommand: string; commands: readonly string[] }>[];
  artifactDefinitions: readonly string[];
  inputs: Readonly<Record<"build" | "contract" | "graph", readonly string[]>>;
  registration: Readonly<{ projectPath: string; packagePath: string; launchSeedPath: string; launchPath: string; projectName: string }>;
}>;

/** 🧭️ Named Nx input that carries each declared input set of the command-ownership contract. */
const STDIO_NAMED_INPUTS: Readonly<Record<keyof CommandOwnership["inputs"], string>> = { build: "stdioCompositionBuild", contract: "stdioArtifactContract", graph: "stdioArtifactGraph" };

/** 🧭️ Declared input set each composition-package command hashes. */
const STDIO_COMMAND_INPUTS: Readonly<Record<string, keyof CommandOwnership["inputs"]>> = { build: "build", check: "build", test: "build", "package-contract": "contract", "package-graph": "graph" };

function sortedNames(values: Iterable<string>): string[] {
  return [...values].sort((left, right) => left.localeCompare(right, "en"));
}

function sourceFile(path: string): ts.SourceFile {
  return ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
}

function exportedNames(path: string): string[] {
  const names = new Set<string>();
  for (const statement of sourceFile(path).statements) {
    if (!(ts.canHaveModifiers(statement) ? ts.getModifiers(statement) : undefined)?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword)) continue;
    if ((ts.isFunctionDeclaration(statement) || ts.isClassDeclaration(statement) || ts.isInterfaceDeclaration(statement) || ts.isTypeAliasDeclaration(statement)) && statement.name) names.add(statement.name.text);
    if (ts.isVariableStatement(statement)) for (const declaration of statement.declarationList.declarations) if (ts.isIdentifier(declaration.name)) names.add(declaration.name.text);
  }
  return sortedNames(names);
}

function routedCommands(path: string): Readonly<{ commands: string[]; defaultCommand: string | undefined; declarations: string[] }> {
  const source = sourceFile(path);
  const commands: string[] = [];
  let defaultCommand: string | undefined;
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && node.expression.name.text === "register" && node.arguments[0] && ts.isStringLiteral(node.arguments[0])) commands.push(node.arguments[0].text);
    if (ts.isPropertyAssignment(node) && ts.isIdentifier(node.name) && node.name.text === "defaultCommand" && ts.isStringLiteral(node.initializer)) defaultCommand = node.initializer.text;
    ts.forEachChild(node, visit);
  };
  visit(source);
  const declarations = source.statements.flatMap((statement) => (ts.isFunctionDeclaration(statement) || ts.isClassDeclaration(statement)) && statement.name ? [statement.name.text] : []);
  return { commands: sortedNames(commands), defaultCommand, declarations };
}

/** 🔍️ Validates schema fixtures, declarations, workspaces and Cargo's own metadata projection. */
export async function testStdioArtifactPackageContract(repoRoot: string): Promise<void> {
  const stdioRoot = join(repoRoot, STDIO_RELATIVE_ROOT);
  const contract = stdioArtifactPackageContract(repoRoot, stdioRoot);
  await assertStdioArtifactSchema(contract, stdioRoot);
  await assertStdioArtifactSourceContract(repoRoot, contract, stdioRoot);
  assertStdioArtifactCargoMetadata(repoRoot, contract, await stdioCargoMetadata(repoRoot));
  console.log(`[stdio-package-contract] source packages=${contract.packages.length} dag=valid`);
}

/** 🕸️ Compares declared artifact dependencies with Cargo metadata and the Nx project graph. */
export async function testStdioArtifactPackageGraph(repoRoot: string): Promise<void> {
  const contract = stdioArtifactPackageContract(repoRoot);
  await assertStdioArtifactNxGraph(repoRoot, contract, await stdioCargoMetadata(repoRoot));
}

/** 🏷️ Binds semantic owners, pure routers, admitted artifact definitions, Nx inputs, package scripts and launch entries to the command-ownership contract. */
export async function verifyStdioCommandOwnership(repoRoot = getWorkspaceRoot()): Promise<void> {
  const stdioRoot = join(repoRoot, STDIO_RELATIVE_ROOT);
  const { default: Ajv2020 } = await import("ajv/dist/2020");
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(readStdioJson(join(stdioRoot, "🧬️schema/🏃️command-ownership/🔣️.json")));
  const fixture = readStdioJson(join(stdioRoot, "🧫️fixtures/🏃️command-ownership/🔣️.json")) as CommandOwnership;
  assert(validate(fixture), JSON.stringify(validate.errors));
  for (const owner of fixture.owners) {
    const path = join(repoRoot, owner.path);
    assert(existsSync(path), `missing stdio owner ${owner.id}: ${owner.path}`);
    assert.deepEqual(exportedNames(path), sortedNames(owner.exports), `${owner.id} exports`);
  }
  for (const router of fixture.routers) {
    const routed = routedCommands(join(repoRoot, router.path));
    assert.deepEqual(routed.commands, sortedNames(router.commands), `${router.id} commands`);
    assert.equal(routed.defaultCommand, router.defaultCommand, `${router.id} default command`);
    assert.deepEqual(routed.declarations, [], `${router.id} router declares its own implementation`);
  }
  assert.deepEqual(stdioArtifactDefinitionPaths(stdioRoot, repoRoot).map((path) => slashStdioPath(relative(repoRoot, path))), [...fixture.artifactDefinitions].sort(), "artifact definitions");
  const project = readStdioJson(join(repoRoot, fixture.registration.projectPath)) as JsonMap;
  const manifest = readStdioJson(join(repoRoot, fixture.registration.packagePath)) as JsonMap;
  assert.equal(project.name, fixture.registration.projectName);
  for (const [set, name] of Object.entries(STDIO_NAMED_INPUTS)) assert.deepEqual(project.namedInputs?.[name], fixture.inputs[set as keyof CommandOwnership["inputs"]], `named input ${name}`);
  const launches = [fixture.registration.launchSeedPath, fixture.registration.launchPath].map((path) => readFileSync(join(repoRoot, path), "utf8"));
  const compositionRouter = fixture.routers.find((router) => router.id === "composition-package")!;
  for (const command of compositionRouter.commands) {
    const set = STDIO_COMMAND_INPUTS[command];
    assert(set, `stdio command ${command} declares no input set`);
    assert(project.targets?.[command]?.inputs?.includes(STDIO_NAMED_INPUTS[set]), `target ${command} does not hash ${STDIO_NAMED_INPUTS[set]}`);
    assert.equal(project.targets[command].options?.command, `bun ./📜️script.ts ${command}`);
    const invocation = `bun nx run ${fixture.registration.projectName}:${command}`;
    assert.equal(manifest.scripts?.[command], invocation, `package script ${command}`);
    for (const [index, launch] of launches.entries()) assert(launch.includes(`"command": "${invocation}"`), `${index === 0 ? fixture.registration.launchSeedPath : fixture.registration.launchPath} misses ${invocation}`);
  }
  console.log(`[stdio-command-ownership] owners=${fixture.owners.length} routers=${fixture.routers.length} artifacts=${fixture.artifactDefinitions.length} commands=${compositionRouter.commands.length}`);
}
