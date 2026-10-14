import { testNxSourceCacheCustody } from "./🔐️custody/🟦️.ts";
import assert from "node:assert/strict";
import { dirname, join } from "node:path";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync, readFileSync, lstatSync, renameSync, symlinkSync } from "node:fs";
import { createRequire, isBuiltin } from "node:module";
import { resolve } from "node:path";
import { collectionManifestCandidate, collectionManifestsFromInventory } from "../../🚀️bootstrap/📥️admission/🟦️.ts";
import {nxToolingInstallManifestV1, nxToolingInstallIdentityV1} from "../../🚀️bootstrap/🛠️tools/📜️script.ts";
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
import {toolPatchReceipt} from "../../🚀️bootstrap/🛠️tools/🩹️patch/🟦️.ts";

/** 🧪️ Portable mode selections agree with the installed Nx task graph and output resolver. */
export async function testNxBuildModes(workspace: string, output?: string): Promise<void> {
  output = output === undefined ? undefined : resolve(workspace, output);
  const parse = createRequire(resolve(workspace, ".nx/installation/package.json"))("yargs-parser");
  const scheduling = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).schedulerSelections;
  const parallelOracle=createRequire(resolve(workspace,".nx/installation/package.json"))("nx/src/command-line/yargs-utils/shared-options").readParallelFromArgsAndEnv;
  for (const row of scheduling) {
    const selected=resolveNxInvocation(row.args,row.environment??{});assert.deepEqual(selected.args,row.expectedArgs);assert.deepEqual(parse(selected.args),parse(row.expectedArgs));const environment=devToolingEnv({NX_PARALLEL:String(row.hostParallelism),...row.environment}),prior=process.env.NX_PARALLEL;
    try{process.env.NX_PARALLEL=environment.NX_PARALLEL;const head=selected.args.slice(0,selected.args.indexOf("--")<0?undefined:selected.args.indexOf("--"));assert.equal(parallelOracle(parse(head)),row.expectedParallel);}finally{if(prior===undefined)delete process.env.NX_PARALLEL;else process.env.NX_PARALLEL=prior;}
  }
  const tooling=JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json",import.meta.url),"utf8")).toolingInstall;
  assert.deepEqual(nxToolingInstallManifestV1(tooling.recipe),tooling.expected);
  const toolingIdentity = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json",import.meta.url),"utf8")).toolingIdentity, toolingFiles = new Map(Object.entries(toolingIdentity.files).map(([path, bytes]) => [path, Buffer.from(String(bytes))]));
  const toolingOracle = new Bun.CryptoHasher("sha256").update(`nx-tooling-install-v3\0${toolingIdentity.environment.platform}\0${toolingIdentity.environment.architecture}\0${toolingIdentity.environment.libc}\0`);
  for (const [path, bytes] of toolingFiles) toolingOracle.update(path + "\0").update(bytes).update("\0");
  assert.equal(nxToolingInstallIdentityV1(toolingIdentity.environment, toolingFiles), toolingOracle.digest("hex"));
  assert.notEqual(nxToolingInstallIdentityV1({ ...toolingIdentity.environment, architecture: "x64" }, toolingFiles), nxToolingInstallIdentityV1(toolingIdentity.environment, toolingFiles));
  const recipe=JSON.parse(readFileSync(new URL("../../🚀️bootstrap/🛠️tools/package.json",import.meta.url),"utf8")),frozen=Bun.JSONC.parse(readFileSync(new URL("../../🚀️bootstrap/🛠️tools/bun.lock",import.meta.url),"utf8"));
  const installed=nxToolingInstallManifestV1(recipe);
  assert.deepEqual(installed.dependencies,frozen.workspaces[""].dependencies);assert.equal(installed.name,frozen.workspaces[""].name);assert.deepEqual(installed.patchedDependencies,frozen.patchedDependencies);assert.ok(!("devDependencies" in installed));
  const cases = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).buildModes;
  for (const row of cases) {
    const invoke = () => resolveNxBuildMode({ args: row.args, env: {} }, row.environment ?? {});
    if (row.error) assert.throws(invoke);
    else { const selected = invoke(); assert.equal(selected.env.SEMIO_BUILD_MODE, row.mode); assert.deepEqual(selected.args,row.expectedArgs);assert.deepEqual(parse(selected.args),parse(row.expectedArgs)); }
  }
  const Ajv = createRequire(resolve(workspace, "package.json"))("ajv/dist/2020").default, ajv = new Ajv({ strict: false });
  const cacheBudgetSchema=JSON.parse(readFileSync(new URL("../../🚀️bootstrap/💾️budget/🔣️schema.json",import.meta.url),"utf8")),cacheBudgetValid=ajv.compile(cacheBudgetSchema),cacheBudgets=JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json",import.meta.url),"utf8")).cacheBudgets;
  const cacheSource=readFileSync(createRequire(resolve(workspace,".nx/installation/package.json")).resolve("nx/src/tasks-runner/cache.js"),"utf8"),cacheParser=cacheSource.slice(cacheSource.indexOf("function parseMaxCacheSize("),cacheSource.indexOf("function formatCacheSize(")),cacheOracle=new Function(cacheParser+";return parseMaxCacheSize;")();
  for(const row of cacheBudgets){const input={availableBytes:row.availableBytes,...(row.override===undefined?{}:{override:row.override})};assert.ok(cacheBudgetValid(input));const selected=nxCacheBudgetV1(row.availableBytes,row.override);assert.equal(selected,row.expected);assert.equal(cacheOracle(selected),cacheOracle(row.expected));}
  assert.throws(()=>nxCacheBudgetV1(-1));assert.throws(()=>nxCacheBudgetV1(1,""));
  const sourceSchema = JSON.parse(readFileSync(new URL("../../../🏗️build/📥️inputs/🧮️digest/🔣️schema.json", import.meta.url), "utf8")), lockSchema = JSON.parse(readFileSync(new URL("../../../🏗️build/📥️inputs/🔒️lock/🔣️schema.json", import.meta.url), "utf8"));
  const sourceValid = ajv.compile(sourceSchema), lockValid = ajv.compile(lockSchema.$defs.CargoLockInventory);
  assert.ok(sourceValid({ version: 1, files: ["owner/source.ts"] }));
  assert.ok(lockValid({ version: 1, manifest: "owner/Cargo.toml" }));
  for (const path of ["../escape", "/absolute", "owner/../escape", "C:/absolute", "owner\\source", ""]) { assert.equal(sourceValid({ version: 1, files: [path] }), false); assert.equal(lockValid({ version: 1, manifest: path }), false); }
  const identity = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).inputIdentity;
  const compact = deduplicateProjectInputs(identity.inputs, identity.root);
  assert.deepEqual(compact, identity.expected);
  const minimatch = createRequire(resolve(workspace, "package.json"))("minimatch").minimatch;
  const sourceInputCase = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).sourceInputCompaction;
  const sourceInputs = compactSourceInputs(sourceInputCase.inputs, workspace, sourceInputCase.root);
  assert.ok(!sourceInputs.some(input => typeof input === "string" && sourceInputCase.inputs.includes(input) && !input.startsWith("!")), "Excluded exact paths must leave the native plan together with compacted paths");
  assert.equal(sourceInputs.filter(input => typeof input === "string" && input.startsWith("!")).length, sourceInputCase.expectedNegations, "An exact digest must not retain negations after their entire positive source scope was consumed");
  const sourceInputEnv = sourceInputs.find(input => input.env?.startsWith("SEMIO_SOURCE_CLOSURE_"));
  const sourceInputInventory = JSON.parse(readFileSync(join(repositoryBuildGraphDataDirectoryV1(workspace,"build-input-closures"), sourceInputEnv.env.slice("SEMIO_SOURCE_CLOSURE_".length) + ".json"), "utf8"));
  const sourceInputOracle = sourceInputCase.inputs.filter(input => !input.startsWith("!")).map(input => input.replace("{workspaceRoot}/", "")).filter(path => !sourceInputCase.inputs.filter(input => input.startsWith("!")).some(input => minimatch(path, input.slice(1).replace("{projectRoot}", sourceInputCase.root))));
  assert.deepEqual(sourceInputOracle, sourceInputCase.expected);
  assert.deepEqual(sourceInputInventory.files.filter(path => path.startsWith(sourceInputCase.root + "/")), sourceInputOracle);
  const accepts = (inputs: unknown[], path: string) => inputs.filter((input): input is string => typeof input === "string").some(input => !input.startsWith("!") && minimatch(path, input.replaceAll("{projectRoot}", identity.root).replaceAll("{workspaceRoot}/", ""))) && !inputs.filter((input): input is string => typeof input === "string").some(input => input.startsWith("!") && minimatch(path, input.slice(1).replaceAll("{projectRoot}", identity.root).replaceAll("{workspaceRoot}/", "")));
  for (const path of ["owner/package/src/main.ts", "owner/package/dist/result.js", "other/main.ts"]) assert.equal(accepts(compact, path), accepts(identity.inputs, path), "Canonical input identity must preserve independent minimatch membership");
  const locked = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).cargoLockClosure;
  const projection = cargoLockClosure({ version: 4, package: locked.packages }, locked.roots);
  assert.deepEqual(projection.packages.map(cargoLockIdentity), locked.expected);
  const graphlib = createRequire(resolve(workspace, "package.json"))("graphlib"), dependencyGraph = new graphlib.Graph({ directed: true });
  const planCase = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).taskPlanCompaction, compactPlans = compactTaskPlans(planCase.plans), planGraph = new graphlib.Graph({ directed: true });
  assert.ok(ajv.compile(JSON.parse(readFileSync(new URL("../../🕸️graph/📤️export/🔣️schema.json", import.meta.url), "utf8")))(compactPlans));
  assert.deepEqual(compactPlans.instructions, planCase.instructions);
  for (const [id, plan] of Object.entries(planCase.plans)) for (const instruction of plan as string[]) planGraph.setEdge(id, instruction);
  for (const [id, plan] of Object.entries(compactPlans.tasks)) assert.deepEqual(plan.map(index => compactPlans.instructions[index]), planGraph.successors(id).sort(), "Compact task debug plans must preserve independent graphlib instruction incidence");
  for (const entry of locked.packages) dependencyGraph.setNode(cargoLockIdentity(entry));
  for (const entry of locked.packages) for (const dependency of entry.dependencies ?? []) {
    const [name, version] = dependency.split(" "), target = locked.packages.find((candidate: any) => candidate.name === name && (version === undefined || candidate.version === version));
    dependencyGraph.setEdge(cargoLockIdentity(entry), cargoLockIdentity(target));
  }
  const independent = graphlib.alg.preorder(dependencyGraph, [cargoLockIdentity(locked.packages[0])]).sort();
  assert.deepEqual(projection.packages.map(cargoLockIdentity), independent, "Cargo lock scope must match independent graphlib reachability");
  const unrelated = structuredClone(locked.packages); unrelated.at(-1).version = "9.0.0";
  assert.deepEqual(cargoLockClosure({ version: 4, package: unrelated }, locked.roots), projection);
  const changed = structuredClone(locked.packages); changed[2].checksum = "changed";
  assert.notDeepEqual(cargoLockClosure({ version: 4, package: changed }, locked.roots), projection);
  assert.throws(() => cargoLockClosure({ version: 4, package: [...locked.packages, { name: "local", version: "2.0.0" }] }, locked.roots), /Ambiguous/);
  const scopes = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).outputInputScope;
  for (const row of scopes.cases) assert.equal(sourceOutputOverlaps(row.output, scopes.sources, scopes.root), row.overlap);
  const repeated = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).sharedInputGroups, shared = internProjectInputGroups(repeated);
  const nxHasher = createRequire(resolve(workspace, ".nx/installation/package.json"))("nx/src/hasher/task-hasher");
  assert.ok(JSON.stringify(shared).length < JSON.stringify(repeated).length + 128);
  for (const name of Object.keys(repeated)) assert.deepEqual(nxHasher.expandNamedInput(name, shared).map(JSON.stringify).sort(), nxHasher.expandNamedInput(name, repeated).map(JSON.stringify).sort(), "Shared native groups must preserve installed Nx expansion");
  const cargo = await cacheInternals.cargoTargets("🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust", workspace, []);
  assert.equal(cargo.build.defaultConfiguration, "prototype");
  assert.equal(cargo.build.configurations.release.env.SEMIO_BUILD_MODE, "release");
  const require = createRequire(resolve(workspace, ".nx/installation/package.json"));
  const { createTaskGraph } = require("nx/src/tasks-runner/create-task-graph");
  const { getOutputs } = require("nx/src/tasks-runner/utils");
  const testOwnerCase = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).testOwnerModeOutputs;
  const testOwner = testOwnerInternals.target("testing", "cases owner", [], true, testOwnerCase.scope);
  for (const mode of ["prototype", "release"]) {
    assert.equal(testOwner.configurations[mode].env.SEMIO_TEST_OUTPUT_SCOPE, `${testOwnerCase.scope}/${mode}`);
    assert.deepEqual(getOutputs({ owner: { data: { root: "owner", targets: { test: testOwner } } } }, { project: "owner", target: "test", configuration: mode }, {}), testOwnerCase.children.map(child => `${testOwnerCase.prefix}/${mode}/${child}`));
  }
  const targets = { build: targetBuildModes("build", { executor: "nx:run-commands", cache: true, dependsOn: ["leaf:build"], outputs: ["{projectRoot}/dist/build/{options.env.SEMIO_BUILD_MODE}"], options: { command: "bun ./📜️script.ts build" } }) };
  const leaf = { ...targets.build, dependsOn: [] };
  const nodes = { owner: { name: "owner", type: "lib", data: { root: "owner", targets } }, leaf: { name: "leaf", type: "lib", data: { root: "leaf", targets: { build: leaf } } } };
  const graph = { nodes, externalNodes: {}, dependencies: { owner: [], leaf: [] } };
  for (const mode of ["prototype", "release"]) {
    const tasks = createTaskGraph(graph, {}, ["owner"], ["build"], mode, {});
    assert.deepEqual(Object.keys(tasks.tasks).sort(), [`leaf:build:${mode}`, `owner:build:${mode}`]);
    assert.deepEqual(getOutputs(nodes, tasks.tasks[`leaf:build:${mode}`].target, {}), [`leaf/dist/build/${mode}`]);
  }
  const priorMode=process.env.SEMIO_BUILD_MODE;
  try { for(const mode of ["prototype","release"]){
    process.env.SEMIO_BUILD_MODE=mode;
    assert.equal(targetBuildModes("build",targets.build).defaultConfiguration,"prototype");
    const originalTasks=createTaskGraph(graph,{},["owner"],["build"],undefined,{});assert.deepEqual(Object.keys(originalTasks.tasks).sort(),[`leaf:build:${mode}`,`owner:build:${mode}`]);assert.deepEqual(getOutputs(nodes,originalTasks.tasks[`leaf:build:${mode}`].target,{}),[`leaf/dist/build/${mode}`]);
  } } finally { if(priorMode===undefined)delete process.env.SEMIO_BUILD_MODE;else process.env.SEMIO_BUILD_MODE=priorMode; }
  nodes.owner.data.targets["native-build-release"] = targetBuildModes("native-build-release", targets.build);
  const inherited = createTaskGraph(graph, {}, ["owner"], ["native-build-release"], undefined, {});
  assert.ok(inherited.tasks["leaf:build:release"], "Default fixed release must propagate through dependencies");
  assert.throws(() => createTaskGraph(graph, {}, ["owner"], ["native-build-release"], "prototype", {}), /Conflicting build mode/);
  assert.throws(() => createTaskGraph(graph, {}, ["owner"], ["build"], "dev", {}), /prototype or release/);
  const dependencyCase = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).commandDependencyOracle;
  const entry = dependencyCase.entry;
  const commandSources = (await import("../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs")).createScriptInputCache(), commandTarget = { options: { command: `bun ${JSON.stringify(entry)} nx` } };
  const closure = await cacheInternals.genericTargetCommandInputs(commandTarget, workspace, [], commandSources);
  for (let pass = 1; pass < dependencyCase.sameGraphPasses; pass++) assert.equal(await cacheInternals.genericTargetCommandInputs(commandTarget, workspace, [], commandSources), closure, "Equivalent commands in one graph must share one external import scan");
  assert.equal(commandSources.commandDependencies.size, dependencyCase.expectedMemoizedClosures);
  assert.deepEqual(await cacheInternals.genericTargetCommandInputs(commandTarget, workspace, []), closure, "A fresh graph must observe the same external ownership independently");
  const lock = Bun.JSONC.parse(readFileSync(resolve(workspace, "bun.lock"), "utf8"));
  const patches = Object.fromEntries(Object.values(lock.patchedDependencies ?? {}).map(path => [path, readFileSync(resolve(workspace, String(path)))]));
  const model = cacheInternals.bunLockGraph(lock, patches), expected = new Set<string>();
  const oracle = await createRequire(resolve(workspace, "package.json"))("esbuild").build({ entryPoints: [entry], absWorkingDir: workspace, bundle: true, write: false, platform: "node", format: "esm", packages: "external", metafile: true, logLevel: "silent" });
  for (const [file, input] of Object.entries(oracle.metafile.inputs)) for (const dependency of input.imports) {
    if (!dependency.external || dependency.path.startsWith(".") || dependency.path.startsWith("/") || isBuiltin(dependency.path) || dependency.path.startsWith("bun:")) continue;
    const name = dependency.path.startsWith("@") ? dependency.path.split("/").slice(0, 2).join("/") : dependency.path.split("/")[0];
    const key = model.resolveImport(file, name);
    if (key && model.externalNodes[`npm:${key}`]) expected.add(`npm:${key}`);
  }
  for (const dependency of dependencyCase.dynamicExternalDependencies) { assert.ok(model.externalNodes[dependency]); expected.add(dependency); }
  assert.deepEqual(closure.find(input => input.externalDependencies)?.externalDependencies, [...expected].sort(), "Command package scope must match esbuild's independent import oracle");
  assert.ok(!closure.includes("{workspaceRoot}/package.json") && !closure.includes("{workspaceRoot}/bun.lock"));
  const workerCase=JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json",import.meta.url),"utf8")).ownerCommandClosure;
  const workerOracle=await createRequire(resolve(workspace,"package.json"))("esbuild").build({entryPoints:[workerCase.entry],absWorkingDir:workspace,bundle:true,write:false,platform:"node",format:"esm",packages:"external",metafile:true,define:{"import.meta.vitest":"false"},logLevel:"silent"}),workerFiles=Object.keys(workerOracle.metafile.inputs);
  assert.ok(workerFiles.length<=workerCase.maximumFiles);for(const file of workerFiles)for(const segment of workerCase.excludedSegments)assert.ok(!file.includes(segment),"Owner command runtime must not import unrelated UI or test implementations");
  const workerInputs=await cacheInternals.genericTargetCommandInputs({options:{cwd:".",command:`bun ${JSON.stringify(workerCase.entry)} owner-command`}},workspace,[]);for(const file of workerFiles)assert.ok(workerInputs.includes("{workspaceRoot}/"+file),"Own command input closure must retain every actual bundled runtime source");
  const nativeCase=JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json",import.meta.url),"utf8")).nativeCommandClosure,nativeOracle=await createRequire(resolve(workspace,"package.json"))("esbuild").build({entryPoints:[nativeCase.entry],absWorkingDir:workspace,bundle:true,write:false,platform:"node",format:"esm",packages:"external",metafile:true,define:{"import.meta.vitest":"false"},logLevel:"silent"}),nativeFiles=Object.keys(nativeOracle.metafile.inputs);console.log(`[DEBUG] Native command runtime source count=${nativeFiles.length}`);const paths=new Map<string,string[]>([[nativeCase.entry,[nativeCase.entry]]]);for(const file of paths.keys())for(const imported of nativeOracle.metafile.inputs[file]?.imports??[])if(!imported.external&&!paths.has(imported.path))paths.set(imported.path,[...paths.get(file)!,imported.path]);for(const file of nativeFiles)for(const segment of nativeCase.excludedSegments)assert.ok(!file.includes(segment),`Native command runtime contains unrelated source: ${(paths.get(file)??[file]).join(" -> ")}`);const nativeInputs=await cacheInternals.nativeTargetCommandInputs({options:{cwd:".",command:`bun ${JSON.stringify(nativeCase.entry)} native cargo build`}},workspace);for(const file of nativeFiles)assert.ok(nativeInputs.includes("{workspaceRoot}/"+file),"Native command input closure must retain every actual bundled runtime source");console.log(`[DEBUG] Independent esbuild source owners generic=${workerFiles.length} native=${nativeFiles.length}, unrelated UI/tests absent`);
  const workerEnvelope={version:1,policy:workerCase.policy,deadlineEpochMilliseconds:Date.now()+workerCase.policy.maximumElapsedMilliseconds,capabilities:workerCase.capabilities},workerEnvironment={...process.env,SEMIO_SCRIPT_PROCESS_INVOCATION:JSON.stringify(workerEnvelope)};delete workerEnvironment.SEMIO_SCRIPT_CAPABILITIES;
  const workerChild=Bun.spawn([process.execPath,resolve(workspace,workerCase.entry),"owner-command","--cwd",workspace,"--","node","-e","console.log(JSON.stringify({arguments:process.argv.slice(1),envelope:JSON.parse(process.env.SEMIO_SCRIPT_PROCESS_INVOCATION)}))","--",...workerCase.forwardedArguments],{cwd:workspace,env:workerEnvironment,stdout:"pipe",stderr:"pipe"}),workerStdout=await new Response(workerChild.stdout).text(),workerStderr=await new Response(workerChild.stderr).text();assert.equal(await workerChild.exited,0,workerStderr);const workerResult=JSON.parse(workerStdout.slice(workerStdout.indexOf('{"arguments":')).split("\n")[0]);assert.deepEqual(workerResult.arguments,workerCase.forwardedArguments);assert.deepEqual(workerResult.envelope,workerEnvelope);
  const routing = readFileSync(resolve(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🧩️contributions/🟦️.ts"), "utf8");
  assert.ok(!routing.includes("--skip-nx-cache"), "Owned command routing must honor declared target cache policy");
  if (output) {
    mkdirSync(output, { recursive: true });
    const patchCase=JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json",import.meta.url),"utf8")).toolingPatchReceipt,patchRoot=mkdtempSync(join(output,"p-"));
    try{
      const oracle=createRequire(resolve(workspace,".nx/installation/package.json"))("diff").applyPatch(patchCase.before,patchCase.patch);assert.equal(oracle,patchCase.after);
      writeFileSync(join(patchRoot,"main.js"),patchCase.before);assert.throws(()=>toolPatchReceipt(patchRoot,patchCase.patch));
      writeFileSync(join(patchRoot,"main.js"),oracle);const receipt=toolPatchReceipt(patchRoot,patchCase.patch);
      assert.ok(ajv.compile(JSON.parse(readFileSync(new URL("../../🚀️bootstrap/🛠️tools/🩹️patch/🔣️schema.json",import.meta.url),"utf8")))(receipt));assert.equal(receipt.files["main.js"],new Bun.CryptoHasher("sha256").update(oracle).digest("hex"));
    }finally{rmSync(patchRoot,{recursive:true,force:true});}
    await testNxSourceCacheCustody(workspace, output);
    const manifestInputs = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).manifestInputOwnership;
    const portable=JSON.parse(readFileSync(buildGraphPathV1(workspace),"utf8")),projection=(await import("../../../🟨️.mjs")).projectBuildGraphForNxV1(portable),admitted={nodes:Object.fromEntries(Object.entries(projection.projects).map(([name,data])=>[name,{name,type:"lib",data}])),dependencies:Object.fromEntries(Object.keys(portable.projects).map(name=>[name,portable.dependencies.filter(edge=>edge.source===name)]))};
    const goOwnership=JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json",import.meta.url),"utf8")).goSourceOwnership,goOwner=admitted.nodes[goOwnership.project].data,goDependency=admitted.nodes[goOwnership.dependency].data;
    assert.ok(admitted.dependencies[goOwnership.project].some(edge=>edge.target===goOwnership.dependency),"Selected Go local replacement must have a genuine Nx source owner edge");
    const goOracle=Bun.spawnSync(["go","list","-m","-json",goOwnership.module],{cwd:resolve(workspace,goOwner.root)});assert.equal(goOracle.exitCode,0);assert.equal(JSON.parse(new TextDecoder().decode(goOracle.stdout)).Dir,resolve(workspace,goDependency.root));
    const goInputs=nxHasher.expandNamedInput("default",goDependency.namedInputs),goInventory=new Set<string>();for(const input of goInputs)if(input.env?.startsWith("SEMIO_SOURCE_CLOSURE_")){const inventory=JSON.parse(readFileSync(join(repositoryBuildGraphDataDirectoryV1(workspace,"build-input-closures"),input.env.slice("SEMIO_SOURCE_CLOSURE_".length)+".json"),"utf8"));for(const file of inventory.files)goInventory.add(file);}
    const goPatterns=goInputs.map(input=>typeof input==="string"?input:input.fileset).filter(input=>typeof input==="string").map(input=>input.replaceAll("{projectRoot}",goDependency.root).replaceAll("{workspaceRoot}/",""));
    for(const file of goOwnership.files){const path=goDependency.root+"/"+file,covered=goInventory.has(path)||goPatterns.some(pattern=>!pattern.startsWith("!")&&minimatch(path,pattern))&&!goPatterns.some(pattern=>pattern.startsWith("!")&&minimatch(path,pattern.slice(1)));assert.ok(covered,"Go dependency source inventory and native glob inputs must retain both platform implementations and module identity");}
    assert.ok(goOwner.targets.build.inputs.some(input=>typeof input==="string"&&input.startsWith("^")||input.projects!==undefined),"Go build must hash genuine dependency source inputs");
    console.log("[DEBUG] Actual Go module resolver agrees with neutral durability Nx dependency and both platform source inventories");
    const nativeOwnership = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).nativeSourceOwnership;
    const nativeInputs = nxHasher.expandNamedInput("nativeSources", admitted.nodes[nativeOwnership.project].data.namedInputs), nativeInventory = new Set<string>();
    for (const input of nativeInputs) if (input.env?.startsWith("SEMIO_SOURCE_CLOSURE_")) {
      const inventory = JSON.parse(readFileSync(join(repositoryBuildGraphDataDirectoryV1(workspace,"build-input-closures"), input.env.slice("SEMIO_SOURCE_CLOSURE_".length) + ".json"), "utf8"));
      for (const file of inventory.files) nativeInventory.add(file);
    }
    for (const file of nativeOwnership.files) assert.ok(nativeInventory.has(file), "Native source inventories must retain each selected compiler entry and manifest");
    const depInfo=process.env.SEMIO_TEST_NATIVE_DEP_INFO;
    if(depInfo){
      const line=readFileSync(resolve(workspace,depInfo),"utf8").split("\n")[0],sources=line.slice(line.indexOf(": ")+2).match(/(?:\\.|[^\s])+/gu)??[];
      for(const source of sources){const path=resolve(workspace,nativeOwnership.compilerWorkingDirectory,source.replace(/\\([\s\\])/gu,"$1")).slice(resolve(workspace).length+1).split("\\").join("/");assert.ok(nativeInventory.has(path),`Compiler dep-info source missing from Nx inventory: ${path}`);}
      assert.ok(sources.length>=3);console.log(`[DEBUG] Actual rustc dep-info sources=${sources.length} are covered by admitted Nx source inventory`);
    }
    assert.ok(admitted.nodes[manifestInputs.project].data.targets[manifestInputs.target].inputs.length <= manifestInputs.maximumInputs, "Manifest-only Cargo projects must share the same source/command ownership projection as authored projects");
    const digestCase = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).sourceDigest;
    const digestRoot = mkdtempSync(join(output, "d-"));
    try {
      const missingLock = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).missingCargoLock;
      const missingManifest = join(digestRoot, "Cargo.toml");
      writeFileSync(missingManifest, missingLock.manifest);
      const missingDigest = selectedCargoLockDigest(missingManifest);
      assert.equal(missingDigest, new Bun.CryptoHasher("sha256").update(missingLock.digestInput).digest("hex"));
      writeFileSync(join(digestRoot, "Cargo.lock"), 'version = 4\n[[package]]\nname = "selected"\nversion = "1.0.0"\n');
      assert.notEqual(selectedCargoLockDigest(missingManifest), missingDigest, "Preparation establishing a missing lock must change the selected hash receipt");
      for (const [path, bytes] of Object.entries(digestCase.files)) { const file = join(digestRoot, path); mkdirSync(dirname(file), { recursive: true }); writeFileSync(file, String(bytes)); }
      const expected = new Bun.CryptoHasher("sha256");
      for (const path of [...digestCase.paths].sort()) {
        expected.update(path + "\0");
        const bytes = digestCase.files[path];
        if (bytes === undefined) expected.update("missing\0");
        else expected.update("file\0" + new Bun.CryptoHasher("sha256").update(bytes).digest("hex") + "\0");
      }
      const contract = { version: 1 as const, files: digestCase.paths }, actual = sourceClosureDigest(digestRoot, contract);
      assert.ok(sourceValid(contract));
      assert.equal(actual, expected.digest("hex"), "Exact source bytes must match independent Bun SHA-256 implementation");
      writeFileSync(join(digestRoot, digestCase.paths[0]), "changed");
      assert.notEqual(sourceClosureDigest(digestRoot, contract), actual, "Current bytes change a reused closure hash");
      writeFileSync(join(digestRoot, digestCase.paths[2]), "new");
      assert.notEqual(sourceClosureDigest(digestRoot, contract), actual, "Missing input admission changes a reused closure hash");
    } finally { rmSync(digestRoot, { recursive: true, force: true }); }
    const row = JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json", import.meta.url), "utf8")).collectionAdmission;
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
      console.log("[DEBUG] fresh structural membership, new untracked owner and ignored leaf admission passed");
    } finally { rmSync(root, { recursive: true, force: true }); }
  }
  if(output){
    const contract=JSON.parse(readFileSync(new URL("../🧫️fixtures/nx-contract/🔣️.json",import.meta.url),"utf8")).collectionCustody;
    const glob=createRequire(resolve(workspace,"package.json"))("fast-glob");
    for(const row of contract.cases){
      const root=mkdtempSync(join(output,"c-"));
      try{
        const directory=join(root,contract.directory),file=join(root,contract.manifest);mkdirSync(directory,{recursive:true});if(row.admitted)writeFileSync(file,contract.manifestContent);const before=lstatSync(directory);const admitted=row.admitted?[{path:contract.manifest}]:[];
        if(row.mutation==="remove")rmSync(directory,{recursive:true});
        else if(row.mutation==="replace"){renameSync(directory,directory+"-original");mkdirSync(directory);}
        else if(row.mutation==="symlink"){renameSync(directory,directory+"-original");symlinkSync(directory+"-original",directory,process.platform==="win32"?"junction":"dir");}
        const entries=await glob("**",{cwd:root,dot:true,onlyFiles:false,followSymbolicLinks:false,stats:true});const actual=entries.find((entry:any)=>entry.path===contract.directory);const oracle=actual===undefined?!row.admitted:actual.stats.isDirectory()&&!actual.stats.isSymbolicLink()&&actual.stats.dev===before.dev&&actual.stats.ino===before.ino;
        assert.equal(oracle,row.accepted,row.id+" independent inventory");
        let accepted=true;try{await verifyNxAdmissionDirectoriesV1(root,[{path:directory,device:before.dev,inode:before.ino}],admitted,{});}catch{accepted=false;}
        assert.equal(accepted,row.accepted,row.id+" original directory custody");
      }finally{rmSync(root,{recursive:true,force:true});}
    }
    console.log("[DEBUG] Five original directory custody cases match independent fast-glob inode/symlink inventory");
  }
  console.log(`[DEBUG] ${cases.length} Nx mode vectors and installed task graph/output oracle passed`);
}
