import{readBuildGraphV1}from"../../../🏗️build/🕸️graph/🟦️.ts";
import fixture from "../🧫️fixtures/nx-contract/🔣️.json";
import {testOwnedFixtureWireBindingsV1,testNxCustodyInputsModelV1} from "./📥️contracts/🟦️.ts";
import {scriptInvocationEnvironment,createRequestedIssuedResourceControlV1,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {type ResourceLeaseControl} from "@semio-tech/process/leases";
import {captureOwnedCommandResultV1} from "@semio-tech/process/execution";
import {isTestRecordV1,testRecordV1,testStringsV1,testMethodV1,testInputsV1} from "../📋️values/🟦️.ts";
import { testNxSourceCacheCustody } from "./🔐️custody/🟦️.ts";
import assert from "node:assert/strict";
import { dirname, join } from "node:path";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync, readFileSync, lstatSync, renameSync, symlinkSync } from "node:fs";
import { createRequire, isBuiltin } from "node:module";
import { resolve } from "node:path";
import { collectionManifestCandidate, collectionManifestsFromInventory } from "../../🚀️bootstrap/📥️admission/🟦️.ts";
import {nxToolingInstallManifestV1, nxToolingInstallIdentityV1} from "@semio-tech/nx-bootstrap-tools/script";
import {verifyNxAdmissionDirectoriesV1,resolveNxInvocation} from "../../🚀️bootstrap/📜️script.ts";
import { resolveNxBuildMode } from "../../🚀️bootstrap/🎚️mode/🟦️.ts";
import {devToolingEnv} from "../../../🏃️process/🌿️environment/🟦️.ts";
import { sourceClosureDigest } from "../../../🏗️build/📥️inputs/🧮️digest/📜️script.ts";
import {repositoryBuildGraphDataDirectoryV1} from "../../../🏗️build/📥️inputs/💾️storage/🟨️.mjs";
import {buildGraphPathV1} from "../../../🏗️build/🕸️graph/🟦️.ts";
import { cargoLockClosure, cargoLockIdentity } from "../../../🏗️build/📥️inputs/🔒️lock/🟨️.mjs";
import { selectedCargoLockDigest } from "../../../🏗️build/📥️inputs/🔒️lock/📥️read/🟨️.mjs";
import { sourceClosureTaskEnvironments, sourceClosureReceiptHolds } from "../../../🏗️build/📥️inputs/🧮️digest/🟨️.mjs";
import { sourceOutputOverlaps, internProjectInputGroups, compactSourceInputs, deduplicateProjectInputs } from "../../../🏗️build/📥️inputs/🟨️.mjs";
import { cacheInternals } from "../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs";
import { targetBuildModes } from "../../../🏗️build/🎚️mode/🟨️.mjs";
import { internals as testOwnerInternals } from "../../../../🧪️test/🟨️.mjs";
import { compactTaskPlans } from "../../🕸️graph/📤️export/🟨️.mjs";
import {nxCacheBudgetV1} from "../../🚀️bootstrap/💾️budget/🟦️.ts";
import {toolPatchReceipt} from "@semio-tech/nx-bootstrap-tools/patch";

/** 🔒️ The actual lock fields compared against independent tooling manifests. */
type ObservedBunLock={workspaces:Record<string,{name?:string;dependencies?:Record<string,string>}>;patchedDependencies?:Record<string,string>};
/** 🧷️ Narrows only a declared environment input before reading its selected inventory. */
const isEnvironmentInput=(input:unknown):input is {env:string}=>input!==null&&typeof input==="object"&&"env"in input&&typeof input.env==="string";
/** 📁️ Narrows a native fileset while preserving every other input kind separately. */
const inputFileset=(input:unknown):string|undefined=>typeof input==="string"?input:input!==null&&typeof input==="object"&&"fileset"in input&&typeof input.fileset==="string"?input.fileset:undefined;

/** 🧪️ Portable mode selections agree with the installed Nx task graph and output resolver. */
export async function testNxBuildModes(workspace: string, output: string|undefined, invocation: ScriptInvocation, phaseControl: ResourceLeaseControl): Promise<void> {
  testOwnedFixtureWireBindingsV1(workspace,invocation,phaseControl);await testNxCustodyInputsModelV1(invocation,phaseControl);
  output = output === undefined ? undefined : resolve(workspace, output);
  const parse = createRequire(resolve(workspace, ".nx/installation/package.json"))("yargs-parser");
  const scheduling = fixture.schedulerSelections;
  const parallelOracle=createRequire(resolve(workspace,".nx/installation/package.json"))("nx/src/command-line/yargs-utils/shared-options").readParallelFromArgsAndEnv;
  for (const row of scheduling) {
    const selected=resolveNxInvocation(row.args,row.environment??{});assert.deepEqual(selected.args,row.expectedArgs);assert.deepEqual(parse(selected.args),parse(row.expectedArgs));const environment=devToolingEnv({NX_PARALLEL:String(row.hostParallelism),...row.environment}),prior=process.env.NX_PARALLEL;
    try{process.env.NX_PARALLEL=environment.NX_PARALLEL;const head=selected.args.slice(0,selected.args.indexOf("--")<0?undefined:selected.args.indexOf("--"));assert.equal(parallelOracle(parse(head)),row.expectedParallel);}finally{if(prior===undefined)delete process.env.NX_PARALLEL;else process.env.NX_PARALLEL=prior;}
  }
  const tooling=fixture.toolingInstall;
  assert.deepEqual(nxToolingInstallManifestV1(tooling.recipe),tooling.expected);
  const toolingIdentity = fixture.toolingIdentity, toolingFiles = new Map(Object.entries(toolingIdentity.files).map(([path, bytes]) => [path, Buffer.from(String(bytes))]));
  const toolingOracle = new Bun.CryptoHasher("sha256").update(`nx-tooling-install-v3\0${toolingIdentity.environment.platform}\0${toolingIdentity.environment.architecture}\0${toolingIdentity.environment.libc}\0`);
  for (const [path, bytes] of toolingFiles) toolingOracle.update(path + "\0").update(bytes).update("\0");
  assert.equal(nxToolingInstallIdentityV1(toolingIdentity.environment, toolingFiles), toolingOracle.digest("hex"));
  assert.notEqual(nxToolingInstallIdentityV1({ ...toolingIdentity.environment, architecture: "x64" }, toolingFiles), nxToolingInstallIdentityV1(toolingIdentity.environment, toolingFiles));
  const recipe=JSON.parse(readFileSync(new URL("../../🚀️bootstrap/🛠️tools/package.json",import.meta.url),"utf8")),frozen=testRecordV1(Bun.JSONC.parse(readFileSync(new URL("../../🚀️bootstrap/🛠️tools/bun.lock",import.meta.url),"utf8")));
  const installed=nxToolingInstallManifestV1(recipe);
  assert.deepEqual(installed.dependencies,testRecordV1(frozen,"workspaces","").dependencies);assert.equal(installed.name,testRecordV1(frozen,"workspaces","").name);assert.deepEqual(installed.patchedDependencies,frozen.patchedDependencies);assert.ok(!("devDependencies" in installed));
  const cases = fixture.buildModes;
  for (const row of cases) {
    const invoke = () => resolveNxBuildMode({ args: row.args, env: {} }, row.environment ?? {});
    if (row.error) assert.throws(invoke);
    else { const selected = invoke(); assert.equal(selected.env.SEMIO_BUILD_MODE, row.mode); assert.deepEqual(selected.args,row.expectedArgs);assert.deepEqual(parse(selected.args),parse(row.expectedArgs)); }
  }
  const Ajv = createRequire(resolve(workspace, "package.json"))("ajv/dist/2020").default, ajv = new Ajv({ strict: false });
  const cacheBudgetSchema=JSON.parse(readFileSync(new URL("../../🚀️bootstrap/💾️budget/🔣️schema.json",import.meta.url),"utf8")),cacheBudgetValid=ajv.compile(cacheBudgetSchema),cacheBudgets=fixture.cacheBudgets;
  const cacheSource=readFileSync(createRequire(resolve(workspace,".nx/installation/package.json")).resolve("nx/src/tasks-runner/cache.js"),"utf8"),cacheParser=cacheSource.slice(cacheSource.indexOf("function parseMaxCacheSize("),cacheSource.indexOf("function formatCacheSize(")),cacheOracle=new Function(cacheParser+";return parseMaxCacheSize;")();
  for(const row of cacheBudgets){const input={availableBytes:row.availableBytes,...(row.override===undefined?{}:{override:row.override})};assert.ok(cacheBudgetValid(input));const selected=nxCacheBudgetV1(row.availableBytes,row.override);assert.equal(selected,row.expected);assert.equal(cacheOracle(selected),cacheOracle(row.expected));}
  assert.throws(()=>nxCacheBudgetV1(-1));assert.throws(()=>nxCacheBudgetV1(1,""));
  const sourceSchema = JSON.parse(readFileSync(new URL("../../../🏗️build/📥️inputs/🧮️digest/🔣️schema.json", import.meta.url), "utf8")), lockSchema = JSON.parse(readFileSync(new URL("../../../🏗️build/📥️inputs/🔒️lock/🔣️schema.json", import.meta.url), "utf8"));
  const sourceValid = ajv.compile(sourceSchema), lockValid = ajv.compile(lockSchema.$defs.CargoLockInventory);
  assert.ok(sourceValid({ version: 1, files: ["owner/source.ts"] }));
  assert.ok(lockValid({ version: 1, manifest: "owner/Cargo.toml" }));
  for (const path of ["../escape", "/absolute", "owner/../escape", "C:/absolute", "owner\\source", ""]) { assert.equal(sourceValid({ version: 1, files: [path] }), false); assert.equal(lockValid({ version: 1, manifest: path }), false); }
  const identity = fixture.inputIdentity;
  const compact = deduplicateProjectInputs(identity.inputs, identity.root);
  assert.deepEqual(compact, identity.expected);
  const minimatch = createRequire(resolve(workspace, "package.json"))("minimatch").minimatch;
  const sourceInputCase = fixture.sourceInputCompaction;
  const sourceInputs = compactSourceInputs(sourceInputCase.inputs, workspace, sourceInputCase.root);
  assert.ok(!sourceInputs.some(input => typeof input === "string" && sourceInputCase.inputs.includes(input) && !input.startsWith("!")), "Excluded exact paths must leave the native plan together with compacted paths");
  assert.equal(sourceInputs.filter(input => typeof input === "string" && input.startsWith("!")).length, sourceInputCase.expectedNegations, "An exact digest must not retain negations after their entire positive source scope was consumed");
  const sourceInputEnv = sourceInputs.filter(isEnvironmentInput).find(input=>input.env.startsWith("SEMIO_SOURCE_CLOSURE_"));
  assert(sourceInputEnv&&typeof sourceInputEnv.env==="string");
  const sourceInputInventory = JSON.parse(readFileSync(join(repositoryBuildGraphDataDirectoryV1(workspace,"build-input-closures"), sourceInputEnv.env.slice("SEMIO_SOURCE_CLOSURE_".length) + ".json"), "utf8"));
  const sourceInputOracle = sourceInputCase.inputs.filter((input:string) => !input.startsWith("!")).map((input:string) => input.replace("{workspaceRoot}/", "")).filter((path:string) => !sourceInputCase.inputs.filter((input:string) => input.startsWith("!")).some((input:string) => minimatch(path, input.slice(1).replace("{projectRoot}", sourceInputCase.root))));
  assert.deepEqual(sourceInputOracle, sourceInputCase.expected);
  assert.deepEqual(sourceInputInventory.files.filter((path:string) => path.startsWith(sourceInputCase.root + "/")), sourceInputOracle);
  const accepts = (inputs: unknown[], path: string) => inputs.filter((input): input is string => typeof input === "string").some(input => !input.startsWith("!") && minimatch(path, input.replaceAll("{projectRoot}", identity.root).replaceAll("{workspaceRoot}/", ""))) && !inputs.filter((input): input is string => typeof input === "string").some(input => input.startsWith("!") && minimatch(path, input.slice(1).replaceAll("{projectRoot}", identity.root).replaceAll("{workspaceRoot}/", "")));
  for (const path of ["owner/package/src/main.ts", "owner/package/dist/result.js", "other/main.ts"]) assert.equal(accepts(compact, path), accepts(identity.inputs, path), "Canonical input identity must preserve independent minimatch membership");
  const locked = fixture.cargoLockClosure;
  const projection = cargoLockClosure({ version: 4, package: locked.packages }, locked.roots);
  assert.deepEqual(projection.packages.map(cargoLockIdentity), locked.expected);
  const graphlib = createRequire(resolve(workspace, "package.json"))("graphlib"), dependencyGraph = new graphlib.Graph({ directed: true });
  const planCase = fixture.taskPlanCompaction, compactPlans = compactTaskPlans(planCase.plans), planGraph = new graphlib.Graph({ directed: true });
  assert.ok(ajv.compile(JSON.parse(readFileSync(new URL("../../🕸️graph/📤️export/🔣️schema.json", import.meta.url), "utf8")))(compactPlans));
  assert.deepEqual(compactPlans.instructions, planCase.instructions);
  for (const [id, plan] of Object.entries(planCase.plans)) for (const instruction of plan as string[]) planGraph.setEdge(id, instruction);
  for (const [id, plan] of Object.entries(compactPlans.tasks)) {assert(Array.isArray(plan));assert.deepEqual(plan.map(index=>{assert(typeof index==="number");return compactPlans.instructions[index];}), planGraph.successors(id).sort(), "Compact task debug plans must preserve independent graphlib instruction incidence");}
  for (const entry of locked.packages) dependencyGraph.setNode(cargoLockIdentity(entry));
  for (const entry of locked.packages) for (const dependency of entry.dependencies ?? []) {
    const [name, version] = dependency.split(" "), target = locked.packages.find((candidate: any) => candidate.name === name && (version === undefined || candidate.version === version));
    dependencyGraph.setEdge(cargoLockIdentity(entry), cargoLockIdentity(target!));
  }
  const independent = graphlib.alg.preorder(dependencyGraph, [cargoLockIdentity(locked.packages[0])]).sort();
  assert.deepEqual(projection.packages.map(cargoLockIdentity), independent, "Cargo lock scope must match independent graphlib reachability");
  const unrelated = structuredClone(locked.packages),last=unrelated.at(-1);assert.ok(last);last.version = "9.0.0";
  assert.deepEqual(cargoLockClosure({ version: 4, package: unrelated }, locked.roots), projection);
  const changed = structuredClone(locked.packages); changed[2].checksum = "changed";
  assert.notDeepEqual(cargoLockClosure({ version: 4, package: changed }, locked.roots), projection);
  assert.throws(() => cargoLockClosure({ version: 4, package: [...locked.packages, { name: "local", version: "2.0.0" }] }, locked.roots), /Ambiguous/);
  const scopes = fixture.outputInputScope;
  for (const row of scopes.cases) assert.equal(sourceOutputOverlaps(row.output, scopes.sources, scopes.root), row.overlap);
  const repeated = fixture.sharedInputGroups, shared = internProjectInputGroups(repeated);
  const nxHasher = createRequire(resolve(workspace, ".nx/installation/package.json"))("nx/src/hasher/task-hasher");
  assert.ok(JSON.stringify(shared).length < JSON.stringify(repeated).length + 128);
  for (const name of Object.keys(repeated)) assert.deepEqual(nxHasher.expandNamedInput(name, shared).map(JSON.stringify).sort(), nxHasher.expandNamedInput(name, repeated).map(JSON.stringify).sort(), "Shared native groups must preserve installed Nx expansion");
  const cargo = testRecordV1(await testMethodV1(cacheInternals,"cargoTargets","🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust",workspace,[]));
  assert.equal(testRecordV1(cargo,"build").defaultConfiguration, "prototype");
  assert.equal(testRecordV1(cargo,"build","configurations","release","env").SEMIO_BUILD_MODE, "release");
  const require = createRequire(resolve(workspace, ".nx/installation/package.json"));
  const { createTaskGraph } = require("nx/src/tasks-runner/create-task-graph");
  const { getOutputs } = require("nx/src/tasks-runner/utils");
  const testOwnerCase = fixture.testOwnerModeOutputs;
  const testOwner = testRecordV1(testMethodV1(testOwnerInternals,"target","testing","cases owner",[],true,testOwnerCase.scope));
  for (const mode of ["prototype", "release"]) {
    assert.equal(testRecordV1(testOwner,"configurations",mode,"env").SEMIO_TEST_OUTPUT_SCOPE, `${testOwnerCase.scope}/${mode}`);
    assert.deepEqual(getOutputs({ owner: { data: { root: "owner", targets: { test: testOwner } } } }, { project: "owner", target: "test", configuration: mode }, {}), testStringsV1(testOwnerCase.children).map(child => `${testOwnerCase.prefix}/${mode}/${child}`));
  }
  const buildTarget=targetBuildModes("build", { executor: "nx:run-commands", cache: true, dependsOn: ["leaf:build"], outputs: ["{projectRoot}/dist/build/{options.env.SEMIO_BUILD_MODE}"], options: { command: "bun ./📜️script.ts build" } });const targets:{build:typeof buildTarget;[key:string]:typeof buildTarget}={build:buildTarget};
  const leaf = { ...targets.build, dependsOn: [] };
  const nodes = { owner: { name: "owner", type: "lib", data: { root: "owner", targets } }, leaf: { name: "leaf", type: "lib", data: { root: "leaf", targets: { build: leaf } } } };
  const graph = { nodes, externalNodes: {}, dependencies: { owner: [], leaf: [] } };
  for (const mode of ["prototype", "release"]) {
    const tasks = createTaskGraph(graph, {}, ["owner"], ["build"], mode, {});
    assert.deepEqual(Object.keys(tasks.tasks).sort(), [`leaf:build:${mode}`, `owner:build:${mode}`]);
    assert.deepEqual(getOutputs(nodes, testRecordV1(tasks,"tasks",`leaf:build:${mode}`).target, {}), [`leaf/dist/build/${mode}`]);
  }
  const priorMode=process.env.SEMIO_BUILD_MODE;
  try { for(const mode of ["prototype","release"]){
    process.env.SEMIO_BUILD_MODE=mode;
    assert.equal(targetBuildModes("build",targets.build).defaultConfiguration,"prototype");
    const originalTasks=createTaskGraph(graph,{},["owner"],["build"],undefined,{});assert.deepEqual(Object.keys(originalTasks.tasks).sort(),[`leaf:build:${mode}`,`owner:build:${mode}`]);assert.deepEqual(getOutputs(nodes,testRecordV1(originalTasks,"tasks",`leaf:build:${mode}`).target,{}),[`leaf/dist/build/${mode}`]);
  } } finally { if(priorMode===undefined)delete process.env.SEMIO_BUILD_MODE;else process.env.SEMIO_BUILD_MODE=priorMode; }
  nodes.owner.data.targets["native-build-release"] = targetBuildModes("native-build-release", targets.build);
  const inherited = createTaskGraph(graph, {}, ["owner"], ["native-build-release"], undefined, {});
  assert.ok(inherited.tasks["leaf:build:release"], "Default fixed release must propagate through dependencies");
  assert.throws(() => createTaskGraph(graph, {}, ["owner"], ["native-build-release"], "prototype", {}), /Conflicting build mode/);
  assert.throws(() => createTaskGraph(graph, {}, ["owner"], ["build"], "dev", {}), /prototype or release/);
  const dependencyCase = fixture.commandDependencyOracle;
  const entry = dependencyCase.entry;
  const commandSources = (await import("../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs")).createScriptInputCache(), commandTarget = { options: { command: `bun ${JSON.stringify(entry)} nx` } };
  const closure = testInputsV1(await testMethodV1(cacheInternals,"genericTargetCommandInputs",commandTarget,workspace,[],commandSources));
  for (let pass = 1; pass < dependencyCase.sameGraphPasses; pass++) assert.equal(await testMethodV1(cacheInternals,"genericTargetCommandInputs",commandTarget,workspace,[],commandSources), closure, "Equivalent commands in one graph must share one external import scan");
  const commandDependencies=testRecordV1(commandSources).commandDependencies;assert(commandDependencies instanceof Map);assert.equal(commandDependencies.size, dependencyCase.expectedMemoizedClosures);
  assert.deepEqual(await testMethodV1(cacheInternals,"genericTargetCommandInputs",commandTarget,workspace,[]), closure, "A fresh graph must observe the same external ownership independently");
  const lock = testRecordV1(Bun.JSONC.parse(readFileSync(resolve(workspace, "bun.lock"), "utf8")));
  const patches = Object.fromEntries(Object.values(lock.patchedDependencies ?? {}).map(path => [path, readFileSync(resolve(workspace, String(path)))]));
  const model = testRecordV1(testMethodV1(cacheInternals,"bunLockGraph",lock,patches)), expected = new Set<string>(),externalNodes=new Set(Object.keys(testRecordV1(model,"externalNodes")));
  const oracle:{metafile:import("esbuild").Metafile} = await createRequire(resolve(workspace, "package.json"))("esbuild").build({ entryPoints: [entry], absWorkingDir: workspace, bundle: true, write: false, platform: "node", format: "esm", packages: "external", metafile: true, logLevel: "silent" });
  for (const [file, input] of Object.entries(oracle.metafile.inputs)) {const imports=testRecordV1(input).imports;assert(Array.isArray(imports));for (const dependency of imports) {
    if (!dependency.external || dependency.path.startsWith(".") || dependency.path.startsWith("/") || isBuiltin(dependency.path) || dependency.path.startsWith("bun:")) continue;
    const name = dependency.path.startsWith("@") ? dependency.path.split("/").slice(0, 2).join("/") : dependency.path.split("/")[0];
    const key = testMethodV1(model,"resolveImport",file,name);assert(key===undefined||typeof key==="string");
    if (key && externalNodes.has(`npm:${key}`)) expected.add(`npm:${key}`);
  }
  }
  for (const dependency of dependencyCase.dynamicExternalDependencies) { assert.ok(externalNodes.has(dependency)); expected.add(dependency); }
  assert.deepEqual(closure.find((input):input is Record<string,unknown>=>isTestRecordV1(input)&&Array.isArray(input.externalDependencies))?.externalDependencies, [...expected].sort(), "Command package scope must match esbuild's independent import oracle");
  assert.ok(!closure.includes("{workspaceRoot}/package.json") && !closure.includes("{workspaceRoot}/bun.lock"));
  const workerCase=fixture.ownerCommandClosure;
  const workerOracle=await createRequire(resolve(workspace,"package.json"))("esbuild").build({entryPoints:[workerCase.entry],absWorkingDir:workspace,bundle:true,write:false,platform:"node",format:"esm",packages:"external",metafile:true,define:{"import.meta.vitest":"false"},logLevel:"silent"}),workerFiles=Object.keys(workerOracle.metafile.inputs);
  assert.ok(workerFiles.length<=workerCase.maximumFiles);for(const file of workerFiles)for(const segment of workerCase.excludedSegments)assert.ok(!file.includes(segment),"Owner command runtime must not import unrelated UI or test implementations");
  const workerInputs=testInputsV1(await testMethodV1(cacheInternals,"genericTargetCommandInputs",{options:{cwd:".",command:`bun ${JSON.stringify(workerCase.entry)} owner-command`}},workspace,[]));for(const file of workerFiles)assert.ok(workerInputs.includes("{workspaceRoot}/"+file),"Own command input closure must retain every actual bundled runtime source");
  const nativeCase=fixture.nativeCommandClosure,nativeOracle=await createRequire(resolve(workspace,"package.json"))("esbuild").build({entryPoints:[nativeCase.entry],absWorkingDir:workspace,bundle:true,write:false,platform:"node",format:"esm",packages:"external",metafile:true,define:{"import.meta.vitest":"false"},logLevel:"silent"}),nativeFiles=Object.keys(nativeOracle.metafile.inputs);const paths=new Map<string,string[]>([[nativeCase.entry,[nativeCase.entry]]]);for(const file of paths.keys())for(const imported of nativeOracle.metafile.inputs[file]?.imports??[])if(!imported.external&&!paths.has(imported.path))paths.set(imported.path,[...paths.get(file)!,imported.path]);for(const file of nativeFiles)for(const segment of nativeCase.excludedSegments)assert.ok(!file.includes(segment),`Native command runtime contains unrelated source: ${(paths.get(file)??[file]).join(" -> ")}`);const nativeInputs=testInputsV1(await testMethodV1(cacheInternals,"nativeTargetCommandInputs",{options:{cwd:".",command:`bun ${JSON.stringify(nativeCase.entry)} native cargo build`}},workspace));for(const file of nativeFiles)assert.ok(nativeInputs.includes("{workspaceRoot}/"+file),"Native command input closure must retain every actual bundled runtime source");
  const workerControl=createRequestedIssuedResourceControlV1(phaseControl,workerCase.policy.maximumElapsedMilliseconds),workerEnvironment=scriptInvocationEnvironment(invocation,process.env),wire=workerEnvironment.SEMIO_SCRIPT_PROCESS_INVOCATION;assert(typeof wire==="string");const workerEnvelope=JSON.parse(wire);
  const workerResultCapture=await captureOwnedCommandResultV1(process.execPath,[resolve(workspace,workerCase.entry),"owner-command","--cwd",workspace,"--","node","-e","console.log(JSON.stringify({arguments:process.argv.slice(1),envelope:JSON.parse(process.env.SEMIO_SCRIPT_PROCESS_INVOCATION)}))","--",...workerCase.forwardedArguments],workspace,{control:workerControl,env:workerEnvironment,output:{maximumBytes:65536,maximumLines:256}});assert.equal(workerResultCapture.status,0,workerResultCapture.stderr);assert.equal(workerResultCapture.signal,null);const resultLine=workerResultCapture.stdout.slice(workerResultCapture.stdout.indexOf('{"arguments":')).split("\n")[0];assert(resultLine!==undefined);const workerResult=JSON.parse(resultLine);assert.deepEqual(workerResult.arguments,workerCase.forwardedArguments);assert.deepEqual(workerResult.envelope,workerEnvelope);
  const routing = readFileSync(resolve(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🧩️contributions/🟦️.ts"), "utf8");
  assert.ok(!routing.includes("--skip-nx-cache"), "Owned command routing must honor declared target cache policy");
  if (output) {
    mkdirSync(output, { recursive: true });
    const patchCase=fixture.toolingPatchReceipt,patchRoot=mkdtempSync(join(output,"p-"));
    try{
      const oracle=createRequire(resolve(workspace,".nx/installation/package.json"))("diff").applyPatch(patchCase.before,patchCase.patch);assert.equal(oracle,patchCase.after);
      writeFileSync(join(patchRoot,"main.js"),patchCase.before);assert.throws(()=>toolPatchReceipt(patchRoot,patchCase.patch));
      writeFileSync(join(patchRoot,"main.js"),oracle);const receipt=toolPatchReceipt(patchRoot,patchCase.patch);
      assert.ok(ajv.compile(JSON.parse(readFileSync(new URL("../../🚀️bootstrap/🛠️tools/🩹️patch/🔣️schema.json",import.meta.url),"utf8")))(receipt));assert.equal(receipt.files["main.js"],new Bun.CryptoHasher("sha256").update(oracle).digest("hex"));
    }finally{rmSync(patchRoot,{recursive:true,force:true});}
    await testNxSourceCacheCustody(workspace, output, invocation, phaseControl);
    const manifestInputs = fixture.manifestInputOwnership;
    const portable=testRecordV1(JSON.parse(readFileSync(buildGraphPathV1(workspace),"utf8"))),projection=testRecordV1((await import("../../../🟨️.mjs")).projectBuildGraphForNxV1(readBuildGraphV1(portable))),portableDependencies=portable.dependencies;assert(Array.isArray(portableDependencies));const admitted={nodes:Object.fromEntries(Object.entries(testRecordV1(projection,"projects")).map(([name,data])=>[name,{name,type:"lib",data:testRecordV1(data)}])),dependencies:Object.fromEntries(Object.keys(testRecordV1(portable,"projects")).map(name=>[name,portableDependencies.filter((edge:unknown)=>testRecordV1(edge).source===name)]))};
    const goOwnership=fixture.goSourceOwnership,goOwner=testRecordV1(admitted,"nodes",goOwnership.project,"data"),goDependency=testRecordV1(admitted,"nodes",goOwnership.dependency,"data"),goEdges=admitted.dependencies[goOwnership.project],goRoot=goOwner.root,goDependencyRoot=goDependency.root;assert(goEdges);assert(typeof goRoot==="string"&&typeof goDependencyRoot==="string");
    assert.ok(goEdges.some((edge:unknown)=>testRecordV1(edge).target===goOwnership.dependency),"Selected Go local replacement must have a genuine Nx source owner edge");
    const goOracle=Bun.spawnSync(["go","list","-m","-json",goOwnership.module],{cwd:resolve(workspace,goRoot)});assert.equal(goOracle.exitCode,0);assert.equal(JSON.parse(new TextDecoder().decode(goOracle.stdout)).Dir,resolve(workspace,goDependencyRoot));
    const goInputs=nxHasher.expandNamedInput("default",testRecordV1(goDependency,"namedInputs")),goInventory=new Set<string>();for(const input of goInputs)if(isEnvironmentInput(input)&&input.env.startsWith("SEMIO_SOURCE_CLOSURE_")){const inventory=JSON.parse(readFileSync(join(repositoryBuildGraphDataDirectoryV1(workspace,"build-input-closures"),input.env.slice("SEMIO_SOURCE_CLOSURE_".length)+".json"),"utf8"));for(const file of inventory.files)goInventory.add(file);}
    const goPatterns=goInputs.map(inputFileset).filter((input:unknown):input is string=>typeof input==="string").map((input:string)=>input.replaceAll("{projectRoot}",goDependencyRoot).replaceAll("{workspaceRoot}/",""));
    for(const file of goOwnership.files){const path=goDependencyRoot+"/"+file,covered=goInventory.has(path)||goPatterns.some((pattern:string)=>!pattern.startsWith("!")&&minimatch(path,pattern))&&!goPatterns.some((pattern:string)=>pattern.startsWith("!")&&minimatch(path,pattern.slice(1)));assert.ok(covered,"Go dependency source inventory and native glob inputs must retain both platform implementations and module identity");}
    const goBuildInputs=testRecordV1(goOwner,"targets","build").inputs;assert(Array.isArray(goBuildInputs));assert.ok(goBuildInputs.some((input:unknown)=>typeof input==="string"?input.startsWith("^"):testRecordV1(input).projects!==undefined),"Go build must hash genuine dependency source inputs");
    const nativeOwnership = fixture.nativeSourceOwnership;
    const nativeInputs = nxHasher.expandNamedInput("nativeSources", testRecordV1(admitted,"nodes",nativeOwnership.project,"data","namedInputs")), nativeInventory = new Set<string>();
    for (const input of nativeInputs) if (input.env?.startsWith("SEMIO_SOURCE_CLOSURE_")) {
      const inventory = JSON.parse(readFileSync(join(repositoryBuildGraphDataDirectoryV1(workspace,"build-input-closures"), input.env.slice("SEMIO_SOURCE_CLOSURE_".length) + ".json"), "utf8"));
      for (const file of inventory.files) nativeInventory.add(file);
    }
    for (const file of nativeOwnership.files) assert.ok(nativeInventory.has(file), "Native source inventories must retain each selected compiler entry and manifest");
    const depInfo=process.env.SEMIO_TEST_NATIVE_DEP_INFO;
    if(depInfo){
      const line=readFileSync(resolve(workspace,depInfo),"utf8").split("\n")[0];assert(line!==undefined&&line.includes(": "));const sources=line.slice(line.indexOf(": ")+2).match(/(?:\\.|[^\s])+/gu)??[];
      for(const source of sources){const path=resolve(workspace,nativeOwnership.compilerWorkingDirectory,source.replace(/\\([\s\\])/gu,"$1")).slice(resolve(workspace).length+1).split("\\").join("/");assert.ok(nativeInventory.has(path),`Compiler dep-info source missing from Nx inventory: ${path}`);}
      assert.ok(sources.length>=3);
    }
    const manifestTargetInputs=testRecordV1(admitted,"nodes",manifestInputs.project,"data","targets",manifestInputs.target).inputs;assert(Array.isArray(manifestTargetInputs));assert.ok(manifestTargetInputs.length <= manifestInputs.maximumInputs, "Manifest-only Cargo projects must share the same source/command ownership projection as authored projects");
    const digestCase = fixture.sourceDigest,digestFiles:Readonly<Record<string,string>>=digestCase.files;
    const digestRoot = mkdtempSync(join(output, "d-"));
    try {
      const missingLock = fixture.missingCargoLock;
      const missingManifest = join(digestRoot, "Cargo.toml");
      writeFileSync(missingManifest, missingLock.manifest);
      const missingDigest = await selectedCargoLockDigest(digestRoot,missingManifest,phaseControl);
      assert.equal(missingDigest, new Bun.CryptoHasher("sha256").update(missingLock.digestInput).digest("hex"));
      writeFileSync(join(digestRoot, "Cargo.lock"), 'version = 4\n[[package]]\nname = "selected"\nversion = "1.0.0"\n');
      assert.notEqual(await selectedCargoLockDigest(digestRoot,missingManifest,phaseControl), missingDigest, "Preparation establishing a missing lock must change the selected hash receipt");
      for (const [path, bytes] of Object.entries(digestCase.files)) { const file = join(digestRoot, path); mkdirSync(dirname(file), { recursive: true }); writeFileSync(file, String(bytes)); }
      const expected = new Bun.CryptoHasher("sha256");
      for (const path of [...digestCase.paths].sort()) {
        expected.update(path + "\0");
        const bytes = digestFiles[path];
        if (bytes === undefined) expected.update("missing\0");
        else expected.update("file\0" + new Bun.CryptoHasher("sha256").update(bytes).digest("hex") + "\0");
      }
      const contract = { version: 1 as const, files: digestCase.paths }, actual = await sourceClosureDigest(digestRoot,contract,phaseControl);
      assert.ok(sourceValid(contract));
      assert.equal(actual, expected.digest("hex"), "Exact source bytes must match independent Bun SHA-256 implementation");
      writeFileSync(join(digestRoot, digestCase.paths[0]), "changed");
      assert.notEqual(await sourceClosureDigest(digestRoot,contract,phaseControl), actual, "Current bytes change a reused closure hash");
      writeFileSync(join(digestRoot, digestCase.paths[2]), "new");
      assert.notEqual(await sourceClosureDigest(digestRoot,contract,phaseControl), actual, "Missing input admission changes a reused closure hash");
    } finally { rmSync(digestRoot, { recursive: true, force: true }); }
    const row = fixture.collectionAdmission;
    const policy = JSON.parse(readFileSync(new URL("../../🔣️policy.json", import.meta.url), "utf8"));
    const collections = row.collections;
    const root = mkdtempSync(join(output, "a-"));
    try {
      for (const [path, contents] of Object.entries(row.files)) { const file = join(root, path); mkdirSync(dirname(file), { recursive: true }); writeFileSync(file, String(contents)); }
      const inventory = [row.indexed.join("\0"), row.untracked.join("\0")];
      assert.deepEqual(await collectionManifestsFromInventory(root, inventory, policy, collections), row.expected);
      const oracle = await createRequire(resolve(workspace, "package.json"))("fast-glob")(["**/📋️project.json", "**/package.json", "**/Cargo.toml"], { cwd: root, dot: true, ignore: ["**/dist/**", "**/owner/🧫️examples/**"] });
      assert.deepEqual(oracle.sort(), row.expected, "Untracked owners and ignored leaves must agree with an independent file inventory");
      const additional = join(root, row.additional.path); mkdirSync(dirname(additional), { recursive: true }); writeFileSync(additional, row.additional.content);
      assert.deepEqual(await collectionManifestsFromInventory(root, inventory, policy, collections), [...row.expected, row.additional.path].sort(), "A new ignored manifest must be discovered without changed index/source membership");
      assert.ok(collectionManifestCandidate(row.additional.path, policy, collections));
    } finally { rmSync(root, { recursive: true, force: true }); }
  }
  if(output){
    const contract=fixture.collectionCustody;
    const glob=createRequire(resolve(workspace,"package.json"))("fast-glob");
    for(const row of contract.cases){
      const root:string=mkdtempSync(join(output,"c-"));
      try{
        const directory=join(root,contract.directory),file=join(root,contract.manifest);mkdirSync(directory,{recursive:true});if(row.admitted)writeFileSync(file,contract.manifestContent);const before=lstatSync(directory);const admitted=row.admitted?[{path:contract.manifest}]:[];
        if(row.mutation==="remove")rmSync(directory,{recursive:true});
        else if(row.mutation==="replace"){renameSync(directory,directory+"-original");mkdirSync(directory);}
        else if(row.mutation==="symlink"){renameSync(directory,directory+"-original");symlinkSync(directory+"-original",directory,process.platform==="win32"?"junction":"dir");}
        const entries:readonly{path:string;stats:ReturnType<typeof lstatSync>}[]=await glob("**",{cwd:root,dot:true,onlyFiles:false,followSymbolicLinks:false,stats:true});const actual:{path:string;stats:ReturnType<typeof lstatSync>}|undefined=entries.find((entry:any)=>entry.path===contract.directory);let oracle:boolean;if(actual===undefined)oracle=!row.admitted;else{assert(actual.stats);oracle=actual.stats.isDirectory()&&!actual.stats.isSymbolicLink()&&actual.stats.dev===before.dev&&actual.stats.ino===before.ino;}
        assert.equal(oracle,row.accepted,row.id+" independent inventory");
        let accepted=true;try{await verifyNxAdmissionDirectoriesV1(root,[{path:directory,device:before.dev,inode:before.ino}],admitted,{});}catch{accepted=false;}
        assert.equal(accepted,row.accepted,row.id+" original directory custody");
      }finally{rmSync(root,{recursive:true,force:true});}
    }
  }
}
