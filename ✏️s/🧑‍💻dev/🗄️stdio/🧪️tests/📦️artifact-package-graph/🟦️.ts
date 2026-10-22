import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { join, relative,dirname,resolve,posix,win32,isAbsolute } from "node:path";
import {pathToFileURL} from "node:url";
import ts from "typescript";
import { findWorkspaceRoot } from "@semio-tech/process/routing";
import { type JsonMap, readStdioJson, slashStdioPath, STDIO_RELATIVE_ROOT, stdioArtifactDefinitionPaths, stdioArtifactPackageContract } from "@semio-tech/stdio-inventory-ts";
import { assertStdioArtifactSchema, assertStdioArtifactSourceContract } from "../../🏘️composition/🛂️artifact-contract/🟦️.ts";
import { assertStdioArtifactCargoMetadata, assertStdioArtifactBuildGraph, stdioCargoMetadata } from "../../🏘️composition/🕸️artifact-graph/🟦️.ts";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import {gunzipSync} from "node:zlib";
import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import {encodeGraphRecordsV1,decodeGraphRecordsV1} from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📇️records/🟨️.mjs";
import recordPolicy from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🔣️policy.json" with {type:"json"};
import type {ScriptInvocation} from "@semio-tech/process/routing";
import { readBuildGraphV1,buildPinSchema as pinSchema } from "@semio-tech/repo-lib/build";
import {verifyStdioCargoPhaseV1} from "./🎟️phase/🟦️.ts";

type CommandOwnership = Readonly<{
  cargoMetadata:Readonly<{owner:string;function:string;invocation:string;custody:readonly string[];capture:string;program:string;control:string;maximumBytes:number;maximumLines:number}>;
  packageRouting:readonly Readonly<{platform:"posix"|"win32";root:string;package:string;script:string;target:string;accepted:boolean}>[];
  owners: readonly Readonly<{ id: string; path: string; exports: readonly string[] }>[];
  routers: readonly Readonly<{ id: "root" | "composition-package"; path: string; defaultCommand: string; commands: readonly string[] }>[];
  artifactDefinitions: readonly string[];
  inputs: Readonly<Record<"build" | "contract" | "graph", readonly string[]>>;
  registration: Readonly<{ projectPath: string; packagePath: string; projectName: string }>;
}>;

/** 🧭️ Named build input that carries each declared input set of the command-ownership contract. */
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
export async function testStdioArtifactPackageContract(repoRoot:string,invocation:ScriptInvocation): Promise<void> {
  const stdioRoot = join(repoRoot, STDIO_RELATIVE_ROOT);
  const contract = stdioArtifactPackageContract(repoRoot, stdioRoot);
  await assertStdioArtifactSchema(contract, stdioRoot);
  await assertStdioArtifactSourceContract(repoRoot, contract, stdioRoot);
  assertStdioArtifactCargoMetadata(repoRoot, contract, await stdioCargoMetadata(repoRoot,invocation));
  console.log(`[stdio-package-contract] source packages=${contract.packages.length} dag=valid`);
}

/** 🕸️ Compares declared artifact dependencies with Cargo metadata and the portable build graph. */
export async function testStdioArtifactPackageGraph(repoRoot:string,invocation:ScriptInvocation): Promise<void> {
  await verifyStdioBuildGraphPinFixtures(repoRoot);
  const contract = stdioArtifactPackageContract(repoRoot);
  await assertStdioArtifactBuildGraph(repoRoot, contract, await stdioCargoMetadata(repoRoot,invocation),invocation);
}

/** 📌️ Checks current record publications and immutable pin refusal against independent schema, gzip and SHA oracles. */
export async function verifyStdioBuildGraphPinFixtures(repoRoot:string):Promise<void>{
 const base=join(repoRoot,"✏️s/🧑‍💻dev/🗄️stdio/🧪️tests/🧫️fixtures/📦️artifact-package-graph"),fixture=readStdioJson(join(base,"📌️pin.json")),schema=readStdioJson(join(base,"🧬️schema/🔣️.json")),ajv=new Ajv({strict:true}),validate=ajv.compile(pinSchema);
 assert.equal(ajv.validate(schema,fixture),true,JSON.stringify(ajv.errors));assert.deepEqual(validateJsonSchemaSubset(schema,fixture),[]);
 const encoded=await encodeGraphRecordsV1(fixture.graph,{policy:recordPolicy.records as Parameters<typeof encodeGraphRecordsV1>[1]["policy"],maximumBytes:recordPolicy.maximumSnapshotBytes}),manifest=JSON.parse(encoded.manifestBytes.toString("utf8")),sha=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex");
 assert.equal(manifest.version,fixture.publication.manifestVersion);assert.equal(manifest.graph.version,fixture.publication.graphVersion);assert.equal(encoded.digest,new Bun.CryptoHasher("sha256").update(encoded.manifestBytes).digest("hex"));
 for(const[name,digest]of manifest.projects){const bytes=encoded.records.get(digest);assert(bytes);assert.equal(sha(bytes),digest);assert.deepEqual(JSON.parse(gunzipSync(bytes).toString("utf8")),fixture.graph.projects[name]);}
 for(const row of fixture.cases){
  const records=new Map(encoded.records);let bytes=Buffer.from(encoded.manifestBytes);if(row.change==="malformed-json")bytes=Buffer.from("{");if(row.change==="whole-graph")bytes=Buffer.from(JSON.stringify(fixture.graph));if(row.change==="manifest-version")bytes=Buffer.from(JSON.stringify({...manifest,version:1}));if(row.change==="invalid-graph")bytes=Buffer.from(JSON.stringify({...manifest,graph:{...manifest.graph,version:0}}));
  const digest=sha(bytes),expected=join(repoRoot,"pin-fixture","snapshots",digest+".json"),publication:{path?:string;digest?:string}={path:expected,digest};
  if(row.change==="missing-path")delete publication.path;if(row.change==="missing-digest")delete publication.digest;if(row.change==="relative-path")publication.path="graph.json";if(row.change==="wrong-path")publication.path=join(repoRoot,"another-image","snapshots",digest+".json");if(row.change==="invalid-digest")publication.digest="invalid";if(row.change==="changed-bytes")bytes=Buffer.concat([bytes,Buffer.from(" ")]);
  const first=records.keys().next().value as string;if(row.change==="missing-record")records.delete(first);if(row.change==="changed-record")records.set(first,Buffer.concat([records.get(first)!,Buffer.from(" ")]));assert.equal(validate(publication),row.schemaAccepted,row.name);assert.equal(validateJsonSchemaSubset(pinSchema,publication).length===0,row.schemaAccepted,row.name);
  const read=async()=>{assert.equal(validate(publication),true);assert(publication.path&&isAbsolute(publication.path));assert.equal(publication.path,expected);assert.equal(sha(bytes),publication.digest);return readBuildGraphV1(await decodeGraphRecordsV1(bytes,async key=>records.get(key),{policy:recordPolicy.records as Parameters<typeof decodeGraphRecordsV1>[2]["policy"]}));};
  if(row.accepted)assert.deepEqual(await read(),fixture.graph,row.name);else await assert.rejects(read,row.name);
 }
 const source=sourceFile(join(repoRoot,"✏️s/🧑‍💻dev/🗄️stdio/🏘️composition/🕸️artifact-graph/🟦️.ts")),entry=source.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text==="readStdioArtifactBuildGraphPin");assert(entry);assert.deepEqual(entry.parameters.map(node=>node.name.getText(source)),["repoRoot","invocation"]);assert(entry.parameters.every(node=>!node.questionToken&&!node.initializer));
 const calls:ts.CallExpression[]=[];const visit=(node:ts.Node):void=>{if(ts.isCallExpression(node))calls.push(node);ts.forEachChild(node,visit);};visit(entry);
 const named=(name:string)=>calls.filter(node=>ts.isIdentifier(node.expression)&&node.expression.text===name),publication=named("withRepositoryBuildGraphPublicationV1");assert.equal(publication.length,1);assert.equal(publication[0]!.arguments.length,3);assert.deepEqual(publication[0]!.arguments.slice(0,2).map(node=>node.getText(source)),["repoRoot","control"]);assert.equal(named("createIssuedResourceControlV1")[0]?.arguments[0]?.getText(source),"invocation");assert(named("checkScriptInvocation").length>=2);assert.equal(named("checkResourceLeaseControl")[0]?.arguments[0]?.getText(source),"actual");assert.equal(named("readBuildGraphV1")[0]?.arguments[0]?.getText(source),"graph");
 for(const name of ["readFileSync","loadBuildGraphV1","JSON.parse"])assert(!calls.some(node=>node.expression.getText(source)===name),"Pinned Stdio reader cannot replace or directly parse the owned publication");
 const facade=sourceFile(join(repoRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🏗️build/🟦️.ts"));assert(facade.statements.some(node=>ts.isExportDeclaration(node)&&node.exportClause&&ts.isNamedExports(node.exportClause)&&node.exportClause.elements.some(item=>item.name.text==="withRepositoryBuildGraphPublicationV1")&&node.moduleSpecifier?.getText(facade).includes("🏗️build/🕸️graph/🟦️.ts")));
 console.log(`[stdio-package-contract] immutable pin fixtures=${fixture.cases.length} recordVersion=${manifest.version} graphVersion=${manifest.graph.version}`);
}

/** 🏷️ Binds semantic owners, pure routers, admitted artifact definitions, build inputs and package scripts to the command-ownership contract. */
export async function verifyStdioCommandOwnership(repoRoot = findWorkspaceRoot()): Promise<void> {
  await verifyStdioCargoPhaseV1(repoRoot);
  const stdioRoot = join(repoRoot, STDIO_RELATIVE_ROOT);
  const compositionRoot = join(repoRoot, "✏️s/🧑‍💻dev/🗄️stdio");
  const fixture = readStdioJson(join(compositionRoot, "🧪️tests/🧫️fixtures/🏃️command-ownership/🔣️.json")) as CommandOwnership;
  const metadata=fixture.cargoMetadata,source=sourceFile(join(repoRoot,metadata.owner)),entry=source.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text===metadata.function);assert(entry,"Current Stdio metadata owner required");
  const original=entry.parameters.find(node=>ts.isIdentifier(node.name)&&node.name.text===metadata.invocation);assert(original&&!original.questionToken&&!original.initializer,"Required Original Stdio invocation required");
  const calls:ts.CallExpression[]=[];const visit=(node:ts.Node):void=>{if(ts.isCallExpression(node))calls.push(node);ts.forEachChild(node,visit);};visit(entry);
  for(const name of [...metadata.custody,metadata.capture,"checkScriptInvocation","nativeCallerPhaseV1"])assert(calls.some(node=>ts.isIdentifier(node.expression)&&node.expression.text===name),`Stdio metadata misses original owned ${name}`);
  assert(calls.filter(node=>ts.isPropertyAccessExpression(node.expression)&&node.expression.expression.getText(source)==="phase"&&node.expression.name.text==="advance").length>=2,"Original Stdio admission and per-manifest progress required");
  const capture=calls.find(node=>ts.isIdentifier(node.expression)&&node.expression.text===metadata.capture)!;assert.equal(capture.arguments[0]?.getText(source),metadata.program);
  const options=capture.arguments[3];assert(options&&ts.isObjectLiteralExpression(options));const control=options.properties.find(node=>(ts.isPropertyAssignment(node)||ts.isShorthandPropertyAssignment(node))&&node.name.getText(source)==="control");assert.equal(control&&(ts.isPropertyAssignment(control)?control.initializer.getText(source):ts.isShorthandPropertyAssignment(control)?control.name.text:undefined),metadata.control);
  const parents:string[]=[];for(let node:ts.Node|undefined=capture.parent;node;node=node.parent)if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression))parents.push(node.expression.text);for(const name of metadata.custody)assert(parents.includes(name),`Stdio capture escapes ${name}`);
  assert(!source.statements.some(node=>ts.isImportDeclaration(node)&&ts.isStringLiteral(node.moduleSpecifier)&&node.moduleSpecifier.text==="node:child_process"),"Stdio metadata cannot own an unbound process launcher");
  for (const owner of fixture.owners) {
    const path = join(repoRoot, owner.path);
    assert(existsSync(path), `missing stdio owner ${owner.id}: ${owner.path}`);
    assert.deepEqual(exportedNames(path), sortedNames(owner.exports), `${owner.id} exports`);
  }
  for (const router of fixture.routers) {
    const routed = routedCommands(join(repoRoot, router.path));
    assert.deepEqual(routed.commands, sortedNames(router.commands), `${router.id} commands`);
    assert.equal(routed.defaultCommand, router.defaultCommand, `${router.id} default command`);
    if (router.id === "root") {
      assert.deepEqual(routed.declarations, ["ContractScript", "GraphScript"]);
      const imports = sourceFile(join(repoRoot, router.path)).statements.filter(ts.isImportDeclaration).map((row) => (row.moduleSpecifier as ts.StringLiteral).text);
      assert(imports.every((path) => path.includes("🧰️framework")), "general router imports only first-party routing");
      assert(readFileSync(join(repoRoot, router.path), "utf8").includes("dispatchOwnedScriptRoute"));
    } else assert.deepEqual(routed.declarations, [], `${router.id} router declares its own implementation`);
  }
  assert.deepEqual(stdioArtifactDefinitionPaths(stdioRoot, repoRoot).map((path) => slashStdioPath(relative(repoRoot, path))), [...fixture.artifactDefinitions].sort(), "artifact definitions");
  const project = readStdioJson(join(repoRoot, fixture.registration.projectPath)) as JsonMap;
  const manifest = readStdioJson(join(repoRoot, fixture.registration.packagePath)) as JsonMap;
  assert.equal(project.name, fixture.registration.projectName);
  for (const [set, name] of Object.entries(STDIO_NAMED_INPUTS)) assert.deepEqual(project.namedInputs?.[name], fixture.inputs[set as keyof CommandOwnership["inputs"]], `named input ${name}`);
  const owner=join(repoRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript"),{parseArgsStringToArgv}=await import(pathToFileURL(await Bun.resolve("string-argv",owner)).href),packageDirectory=dirname(join(repoRoot,fixture.registration.packagePath));
  const routing=(script:string,directory:string,workspace:string,target:string,paths:Pick<typeof posix,"resolve">)=>{const argv=parseArgsStringToArgv(script);assert.deepEqual(argv.slice(0,2),["bun","--cwd"]);assert.equal(paths.resolve(directory,argv[2]!),paths.resolve(workspace));assert.deepEqual(argv.slice(3),["repo","run",target]);};
  for(const row of fixture.packageRouting){const check=()=>routing(row.script,row.package,row.root,row.target,row.platform==="win32"?win32:posix);if(row.accepted)check();else assert.throws(check);}
  const compositionRouter = fixture.routers.find((router) => router.id === "composition-package")!;
  for (const command of compositionRouter.commands) {
    const set = STDIO_COMMAND_INPUTS[command];
    assert(set, `stdio command ${command} declares no input set`);
    assert(project.targets?.[command]?.inputs?.includes(STDIO_NAMED_INPUTS[set]), `target ${command} does not hash ${STDIO_NAMED_INPUTS[set]}`);
    assert.equal(project.targets[command].options?.command, `repo ./📜️script.ts ${command}`);
    routing(manifest.scripts?.[command],packageDirectory,repoRoot,`${fixture.registration.projectName}:${command}`,{resolve});
  }
  console.log(`[stdio-command-ownership] owners=${fixture.owners.length} routers=${fixture.routers.length} artifacts=${fixture.artifactDefinitions.length} commands=${compositionRouter.commands.length}`);
}
