import {isOpaqueSourcePathV1} from "../../../../⚡️caching/🔍️discovery/📂️source/🟦️.ts";
import{applyBuildGeneratedTaskDependenciesV1,buildInputSourceRoleV1,buildNativeSourceObservationV1}from"../../🚪️admission/🟦️.ts";
import {createRequire} from "node:module";
const javascriptManagerV1 = createRequire(import.meta.url)("../../../../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🟨️.cjs").selectedManagerV1("javascript");
/** 📦️ Infers every project node of one workspace snapshot with bounded concurrency and settlement. */
import { advanceScriptInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { availableParallelism } from "node:os";
import { createHash } from "node:crypto";
import { dirname, join, relative } from "node:path";
import { inferenceInventoryReferencesHoldV1 } from "../../../../🏗️build/👁️observation/📥️reuse/🟨️.mjs";
import { OBSERVATION_LIMITS_V1, detachedFailureV1, memoizedObservedAsyncComputationV1, observeInferenceSnapshotV1, observeProjectInferenceV1, observedFs, projectInferenceObservationsV1 } from "../../../../🏗️build/👁️observation/🟨️.mjs";
import { statSync as rawStatSync } from "node:fs";
import { INFERENCE_LIMITS_V1 } from "../🧬️schema/🟨️.mjs";
const { existsSync, readFileSync } = observedFs;

/** 🏷️ Names this component as its roster entry does. */
export const component = "project-nodes";
import { createGraphPhaseClockV1, writeGraphPhaseRowV1, writeGraphSettleRowV1 } from "../../⏱️phases/🟨️.mjs";
import { createGraphParentMemoryGovernorV1 } from "../../../🧠️memory/🟨️.mjs";
import { createErrorCollectorFromEnvV1 } from "../../🧾️collect-errors/🟨️.mjs";
import { openProjectFarmV1 } from "./🏊️project-farm/🟨️.mjs";
import { awaitGrowingV1 } from "./🏊️project-farm/🏊️pool/🟨️.mjs";
import {refuseBuildV1} from "../../../📖️texts/🟦️.ts";
let closureApiV1, closureHandleOfV1, closureHandleFromRecipeV1, deriveClosureFactV1, resolveClosureRefsV1, inferenceCandidates,readBuildProducedCompileOutputV1,buildPath, yieldInferenceV1, catalogBuildContributionsV1, discoverNativeBuildOwnersV1, createScriptInputCache, POLICY, nativeCommandInputs, readTaxonomyProjectionV1, collectPlaygroundCatalog, PROJECT_BASENAME, playgroundSessionTargets, playgroundPreparationTargets, playgroundSiteTargetsForAdmissionV1, readToml, cargoTargets, componentBuildTargets, componentCrateRoots, projectAssemblyAuthorityV1, workspaceDataDirectory, projectWithDefaults, withNativePreparation, withReleasePipelinePreparation, loaded, readBunLockGraph, projectStampPathsV1;

/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */
export function wire(registry) {
  ({inferenceCandidates,readBuildProducedCompileOutputV1}=registry.ownerDiscovery);
  ({ buildPath, POLICY, PROJECT_BASENAME, workspaceDataDirectory } = registry.coordinates);
  ({ yieldInferenceV1, createScriptInputCache, nativeCommandInputs, closureApiV1, closureHandleOfV1, closureHandleFromRecipeV1, deriveClosureFactV1 } = registry.scriptInputs);
  ({ catalogBuildContributionsV1, discoverNativeBuildOwnersV1, projectAssemblyAuthorityV1 } = registry.ownerDiscovery);
  ({ readTaxonomyProjectionV1 } = registry.sourceAnalyses);
  ({ collectPlaygroundCatalog, playgroundSessionTargets, playgroundPreparationTargets, playgroundSiteTargetsForAdmissionV1 } = registry.playgroundTargets);
  ({ readToml, withNativePreparation } = registry.nativeManifests);
  ({ cargoTargets, componentBuildTargets, componentCrateRoots, withReleasePipelinePreparation } = registry.componentTargets);
  ({ projectWithDefaults, resolveClosureRefsV1 } = registry.projectAssembly);
  ({ loaded } = registry.bootstrap);
  ({ readBunLockGraph } = registry.bunLock);
  ({ projectStampPathsV1 } = registry.stamps);
}

/** ⏱️ A source modified this recently may be half-written, so its parse failure is retried by settlement instead of withholding its project (the observation racy rule). */
const UNSETTLED_SOURCE_MS = OBSERVATION_LIMITS_V1.racyMilliseconds;

/** 🕸️ How often the projects being inferred are named again while none finishes, so a heavy project stays visible with its running seconds. */
const RUNNING_TICK_MS = INFERENCE_LIMITS_V1.runningTickMilliseconds;

/** 🚧️ Withholds one project whose consumed source does not parse, naming the exact bytes the snapshot observed; a freshly modified source rethrows so settlement waits for it. */
function unavailableProjectV1(root, project, workspaceRoot, error) {
  let modified = Infinity;
  try { modified = rawStatSync(error.source).mtimeMs; } catch {}
  if (modified >= Date.now() - UNSETTLED_SOURCE_MS) throw error;
  return { root, ...(project ? { project } : {}), source: buildPath(relative(workspaceRoot, error.source)), sha256: error.sha256, reason: String(error.message).slice(0, 2048) };
}

/** 🧷️ Reads the withholding source inside the observation of the project it withholds and refuses a withholding that rests on other bytes than the ones the project now holds: the project then stays stale until exactly that file changes, and a read that disagrees with the failure is a half-written source that settlement retries. */
function observeWithheldSourceV1(error) {
  let digest;
  try { digest = createHash("sha256").update(readFileSync(error.source)).digest("hex"); } catch { throw error; }
  if (digest !== error.sha256) throw error;
}

/** 🧾️ The facts that withhold one project outside its own observation: what the failed computation read, then the bytes of the source it names. */
function withheldRowsV1(error) {
  const failure = detachedFailureV1(error), rows = failure ? [...failure.rows, ...failure.layers.flat()] : [];
  const stat = rawStatSync(error.source), bytes = readFileSync(error.source);
  if (createHash("sha256").update(bytes).digest("hex") !== error.sha256) throw error;
  rows.push({ kind: "file", path: error.source, fact: { ctimeMs: stat.ctimeMs, ino: stat.ino, mtimeMs: stat.mtimeMs, sha256: error.sha256, size: stat.size } });
  return rows;
}

/** 📛️ Names the project a failing inference belongs to without replacing the error, so a repeated settlement round says which project stops the build. */
async function tagProjectFailureV1(root, run) {
  try { return await run(); }
  catch (error) { if (error !== null && typeof error === "object" && !Object.hasOwn(error, "semioProject") && Object.isExtensible(error)) Object.defineProperty(error, "semioProject", { value: root, configurable: true }); throw error; }
}

/** 🧵️ Bounds concurrently inferred projects; one worker is the serial reference order. */
function projectConcurrencyV1(options) {
  const value = options?.projectConcurrency ?? Math.max(1, Math.min(8, availableParallelism()));
  if (!Number.isSafeInteger(value) || value < 1 || value > 64) throw Error("Build project inference concurrency must be an integer from 1 to 64");
  return value;
}

/** 🪨️ Reports and cancels inference settlement rounds through the original invocation. */
export function settlementOptionsV1(context) {
  return { signal: context.phaseControl?.signal, onRound: async round => { writeGraphSettleRowV1(round); await yieldInferenceV1(context,"build:settle",0,true); } };
}

/** 📜️ The generator contracts of the workspace taxonomy, read through the observed source analysis in the scope of the caller: a graph construction reads them once and keys every project record with their digest. */
export function projectGeneratorContractsV1(workspaceRoot) {
  const contractPath = join(workspaceRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
  return existsSync(contractPath) ? readTaxonomyProjectionV1(contractPath).generatorContracts : {};
}

/** 🧱️ Builds, from plain data, what the inference of one project shares across the projects of a graph construction; a worker process opens the session its parent describes, the generator contracts included: they are no fact of a project record but part of its key, so a worker that read them again after a peer edit would write content of other contracts under the key of its parent. */
export function createProjectInferenceEnvironmentV1(session) {
  const { workspaceRoot, registry, projectsByRoot, componentRoots, contracts } = session, context = { workspaceRoot, registry, lateClosures: session.lateClosures === true }, scripts = createScriptInputCache(context), commandInputs = session.commandInputs?.closureRecipe ? closureHandleFromRecipeV1(session.commandInputs.closureRecipe) : session.commandInputs;
  if (contracts === null || typeof contracts !== "object" || Array.isArray(contracts)) throw refuseBuildV1("invalid","graph.generator-contracts-required");
  scripts.generatedOutput = path => readBuildProducedCompileOutputV1(path, workspaceRoot, contracts, registry);
  return { workspaceRoot, contracts, commandInputs, scripts, projectsByRoot: new Map(projectsByRoot), componentRoots, nativeCache: new Map(), nativeClosures: new Map(), context };
}

/** 🌐️ Infers the node of one project: its defaults, its native or release preparation and its build modes; a source that does not parse withholds the project instead of failing the graph. */
export async function inferProjectBodyV1(environment, job, admission) {
  const { workspaceRoot, contracts, commandInputs, scripts, projectsByRoot, componentRoots, nativeCache, nativeClosures, context } = environment, { name, root, json, native, pipeline, selectedTargets, selectedInputs } = job;
  try {
    await yieldInferenceV1(context, "build:project-assembly", 0);
    const project = await projectWithDefaults(json, root, join(workspaceRoot, root), workspaceRoot, contracts, new Map(), commandInputs, scripts, undefined, "hash", admission ? selectedTargets ?? [] : undefined, selectedInputs);
    const prepare = (apply, predicate) => { const preparing = admission ? { ...project, targets: Object.fromEntries(Object.entries(project.targets).filter(([, target]) => predicate(target))) } : project; apply(preparing); project.targets = { ...project.targets, ...preparing.targets }; if (preparing.metadata !== undefined) project.metadata = preparing.metadata; };
    if (native) prepare(project => withNativePreparation(project, workspaceRoot, contracts, nativeCache, projectsByRoot, nativeClosures), target => target.metadata?.semio?.execution?.kind !== "source");
    if (pipeline) prepare(project => withReleasePipelinePreparation(project, workspaceRoot, contracts, componentRoots, nativeCache, nativeClosures), target => Boolean(target.metadata?.semio?.release));
    for (const [targetName, target] of Object.entries(project.targets)) project.targets[targetName] = loaded.targetBuildModes(targetName, target);
    return project;
  } catch (error) { if (error?.code !== "BUILD_SOURCE_UNPARSEABLE") throw error; const unavailable = unavailableProjectV1(root, name, workspaceRoot, error); observeWithheldSourceV1(error); return { unavailable }; }
}

/** ♻️ Whether a stored project record may be reused: a failure is never reused, because a record that withholds its project rests on a source the failure may not have named completely, and the new inference costs one cached walk. */
export const projectRecordReusableV1 = value => value?.unavailable === undefined;

/** 🔑️ The key a project record is stored under: what the graph shares across projects, then everything the project itself is made of. */
export const projectRecordKeyV1 = (head, root, json, selectedTargets, selectedInputs) => [...head, root, json, selectedTargets ?? null, selectedInputs];

/** ✍️ Writes the persisted record of one project into the graph directories the running body is staged to, under the key and the observation scope the parent uses, so the parent finds it valid once it adopted the leaves. */
export async function produceProjectRecordV1(environment, job) {
  const cachePath = join(workspaceDataDirectory(environment.workspaceRoot), "np", job.cacheKey + ".json");
  await observeInferenceSnapshotV1(() => observeProjectInferenceV1({ cachePath, key: projectRecordKeyV1(job.head, job.root, job.json, undefined, job.selectedInputs), owner: job.root, stamps: projectStampPathsV1({ native: job.native }), reusable: projectRecordReusableV1 }, () => inferProjectBodyV1(environment, job, undefined)));
}

/** 🔁️ Opens the answer of derived facts for a generation: the same inference environment as a build, so asking a command walk again reads the live files exactly as the inference that recorded it did; every question runs in its own observation snapshot. */
export async function openGenerationDeriverV1({ workspaceRoot, registry }) {
  const environment = await observeInferenceSnapshotV1(() => createProjectInferenceEnvironmentV1({ workspaceRoot, registry, contracts: projectGeneratorContractsV1(workspaceRoot), projectsByRoot: [], componentRoots: [], commandInputs: null, lateClosures: false }));
  return (row, signal) => { signal?.throwIfAborted(); return observeInferenceSnapshotV1(() => deriveClosureFactV1(row, environment.scripts)); };
}

/** 🧺️ Runs the whole-workspace setup of an inference once per share: a later call of the same command (the build after the prefetch) reuses it while every observation it was made from still holds, replays those observations into its own capture, and counts `computed` and `reused` on the share. */
export async function sharedSetupV1(share, key, compute, checkpoint) {
  const before = share.computed;
  const value = await memoizedObservedAsyncComputationV1(share.cache, key, async () => { share.computed++; return compute(); }, checkpoint);
  if (share.computed === before) share.reused++;
  return value;
}

/** 🛡️ Infers the project nodes of every authored emoji-named project config. Nx's native walker sometimes hands back lossy paths with U+FFFD where a multi-byte emoji used to be: those are unopenable duplicates of a real file and are skipped. */
export async function emojiProjectJsonNodes(configFiles, options, context) {
  const { workspaceRoot } = context, unavailable = new Map(), wantedRoots = options?.ownerRoots, wanted = root => !wantedRoots || wantedRoots.has(root);
  const collector = createErrorCollectorFromEnvV1({ workspaceRoot, signal: context.invocation?.control.signal });
  const isAuthoredProjectConfigV1 = configFile => !isOpaqueSourcePathV1(configFile) && configFile.endsWith(PROJECT_BASENAME) && !configFile.includes("\uFFFD") && !configFile.includes(".🧬semio") && !POLICY.generatedDirectories.some((name) => configFile.split("/").includes(name));
  const sharedSetup=!options?.ownerAdmission&&context.setupShare?.cache instanceof Map?context.setupShare:undefined;
  const setupPass=async()=>{
    const catalog=await catalogBuildContributionsV1(workspaceRoot,options?.catalogProviders??[]);
    const found=options?.ownerAdmission?{files:inferenceCandidates(configFiles,workspaceRoot),nativeCache:new Map(),nativeClosures:new Map()}:await discoverNativeBuildOwnersV1(configFiles,workspaceRoot,context),texts=new Map();
    for(const file of found.files.filter(isAuthoredProjectConfigV1)){const abs=join(workspaceRoot,file);texts.set(file,existsSync(abs)?readFileSync(abs):null);}
    return{catalog,files:found.files,nativeCache:found.nativeCache,nativeClosures:found.nativeClosures,playground:collectPlaygroundCatalog(found.files,workspaceRoot,catalog),texts};
  };
  let setupSteps=0;
  const setup=sharedSetup?await sharedSetupV1(sharedSetup,JSON.stringify([workspaceRoot,options?.catalogProviders??[],createHash("sha256").update(configFiles.join("\0")).digest("hex")]),setupPass,(force=false)=>yieldInferenceV1(context,"build:cargo-owners",++setupSteps,force)):await setupPass();
  const catalogContributions=setup.catalog,nativeCache=new Map(setup.nativeCache),nativeClosures=new Map(setup.nativeClosures);configFiles=setup.files;
  const late = !options?.ownerAdmission && options?.snapshot === true && typeof options?.onOwnerObservations === "function", rootsByName = new Map(), scripts = createScriptInputCache({ ...context, lateClosures: late });
  const contracts = projectGeneratorContractsV1(workspaceRoot);
  scripts.generatedOutput=path=>readBuildProducedCompileOutputV1(path,workspaceRoot,contracts,context.registry);
  if(context.invocation)await advanceScriptInvocation(context.invocation,"build:commands","running");
  const commandInputs = !options?.ownerAdmission&&configFiles.some((path) => !isOpaqueSourcePathV1(path) && path.endsWith("Cargo.toml") && !path.includes(".🧬semio") && !POLICY.generatedDirectories.some((name) => path.split("/").includes(name))) ? await closureApiV1(scripts, workspaceRoot).nativeCommandInputs() : undefined;
  if(context.invocation)await advanceScriptInvocation(context.invocation,"build:commands","complete");
  const projectAuthority=createHash("sha256").update(JSON.stringify([contracts])).digest("hex");
  const commandRevision=createHash("sha256").update(JSON.stringify(closureHandleOfV1(commandInputs)?.key??commandInputs??null)).digest("hex"),registryDigest=createHash("sha256").update(JSON.stringify(options?.ownerAdmission?context.registry??null:context.registry?.catalogProviders?.map(({ownerRoot,sourcePath})=>[ownerRoot,sourcePath])??null)).digest("hex");
  const playgroundCatalog = setup.playground;

  const results = (await Promise.all(configFiles
    .filter(isAuthoredProjectConfigV1)
    .map(async (configFile) => {
      const abs = join(workspaceRoot, configFile), bytes = setup.texts.get(configFile);
      if (!bytes) return null;
      let json;
      try {
        json = JSON.parse(bytes.toString("utf8"));
      } catch (error) {
        const root = buildPath(relative(workspaceRoot, dirname(abs))) || ".";
        unavailable.set(root, unavailableProjectV1(root, undefined, workspaceRoot, Object.assign(Error(`Invalid Build project metadata ${configFile}: ${error.message}`), { code: "BUILD_SOURCE_UNPARSEABLE", source: abs, sha256: createHash("sha256").update(bytes).digest("hex") })));
        return null;
      }
      const name = json.name;
      if (!name) throw new Error(`Build project metadata ${configFile} requires a name`);
      const projectDir = dirname(abs);
      const root = buildPath(relative(workspaceRoot, projectDir)) || ".";
      const prior = rootsByName.get(name);
      if (prior !== undefined && prior !== root) throw new Error(`Duplicate Build project ${name}: ${prior} and ${root}`);
      if (prior === undefined) rootsByName.set(name, root);
      if (name === "@semio-tech/plugin-registry") json.targets = { ...json.targets, ...playgroundSessionTargets(configFiles, workspaceRoot,catalogContributions) };
      if (name === "@semio-tech/framework-os-dev") json.targets = { ...json.targets, ...await playgroundPreparationTargets(configFiles, workspaceRoot, root, contracts,catalogContributions,context) };
      if (playgroundCatalog.some(row => row.cratePath === root)) {
        json.targets = { ...playgroundSiteTargetsForAdmissionV1(root,playgroundCatalog,workspaceRoot,context.registry,options?.ownerAdmission?options.ownerAdmission.targets[name]??[]:undefined), ...json.targets };
      }
      const canonicalConfig = buildPath(relative(workspaceRoot, join(projectDir, PROJECT_BASENAME)));
      return [canonicalConfig, { projects: { [name]: {...json,root} } }];
    })))
    .filter(Boolean);
  const declaredRoots = new Set([...rootsByName.values()]);let cargoIndex=0;
  for (const configFile of configFiles.filter((file) => file.endsWith("Cargo.toml"))) {
    await yieldInferenceV1(context,"build:cargo-targets",++cargoIndex);
    if (isOpaqueSourcePathV1(configFile) || configFile.includes("\uFFFD") || configFile.startsWith("compose/") || configFile.startsWith("temp/compose/") || configFile.includes(".🧬semio") || POLICY.generatedDirectories.some((name) => configFile.split("/").includes(name))) continue;
    if (!existsSync(join(workspaceRoot, configFile))) continue;
    const projectDir = dirname(join(workspaceRoot, configFile));
    const root = buildPath(relative(workspaceRoot, projectDir)) || ".";
    if (declaredRoots.has(root)) continue;
    let manifest;
    try { manifest = readToml(join(projectDir, "Cargo.toml")); }
    catch (error) { if (error?.code !== "BUILD_SOURCE_UNPARSEABLE") throw error; unavailable.set(root, unavailableProjectV1(root, undefined, workspaceRoot, error)); continue; }
    if (!manifest.package?.name) continue;
    const name = manifest.package.name;
    if (rootsByName.has(name)) throw new Error(`Duplicate Build project ${name}: ${rootsByName.get(name)} and ${root}`);
    rootsByName.set(name, root);
    if (!wanted(root)) { results.push([configFile, { projects: { [name]: { name, root, projectType: "library", targets: {} } } }]); continue; }
    const build = async () => {
    const targets = await cargoTargets(root, workspaceRoot, commandInputs,scripts);
    const project = { name, root, projectType: "library", tags: ["language:cargo", "discovery:manifest"], targets: { ...targets, ...componentBuildTargets(root, workspaceRoot), ...playgroundSiteTargetsForAdmissionV1(root,playgroundCatalog,workspaceRoot,context.registry,options?.ownerAdmission?options.ownerAdmission.targets[name]??[]:undefined) } };
    results.push([configFile, { projects: { [name]: project } }]);
    };
    if (collector) await collector.guard({ project: name, root, phase: "cargo-targets" }, build); else await build();
  }
  const projects = results.flatMap(([, result]) => Object.values(result.projects));
  const projectsByRoot = new Map(projects.flatMap(project=>[[project.root,project.name],...(project.metadata?.nativeRoot?[[project.metadata.nativeRoot,project.name]]:[])]));
  const componentRoots = componentCrateRoots(configFiles, workspaceRoot);
  const projectEnvironment = { workspaceRoot, contracts, commandInputs, scripts, projectsByRoot, componentRoots, nativeCache, nativeClosures, context };

  const jobs=results.flatMap(([,result])=>Object.entries(result.projects).map(([name,json])=>({result,name,json}))).filter(job=>wanted(job.json.root));let projectIndex=0,cursor=0,stopped=false;
  const inferProject=async({result,name,json})=>{
    if(options?.ownerAdmission?.generatedDependencies?.[name])json=applyBuildGeneratedTaskDependenciesV1(json,options.ownerAdmission.generatedDependencies[name]);
    const started=performance.now();
    await yieldInferenceV1(context,"build:projects",++projectIndex);
    const inactive=options?.ownerAdmission?Object.fromEntries(Object.entries(json.targets??{}).filter(([name])=>!options.ownerAdmission.targets[json.name]?.includes(name))):{};
    const selectedTargets=options?.ownerAdmission?.targets[json.name],selectedInputs=[...new Set(options?.ownerAdmission?.inputs?.[json.name]??[])].sort(),activeJson=options?.ownerAdmission?{...json,targets:Object.fromEntries(Object.entries(json.targets??{}).filter(([name])=>selectedTargets?.includes(name)))}:json;
    const root=json.root,nativeRoot=json.metadata?.nativeRoot??root,nativeTargets=Object.entries(activeJson.targets??{}).filter(([,target])=>target.metadata?.semio?.execution?.kind!=="source"),nativeCandidate=existsSync(join(workspaceRoot,nativeRoot,"Cargo.toml"))||nativeTargets.some(([,target])=>target.options?.command?.includes("⚡️caching/🦀️cargo/📜️script.ts")),native=options?.ownerAdmission?buildNativeSourceObservationV1({selectedSource:Object.values(activeJson.targets??{}).some(target=>target.metadata?.semio?.execution?.kind==="source"),selectedNative:nativeCandidate&&nativeTargets.length>0,requestedInputs:selectedInputs.length>0,dependencyInputs:false,inputRole:buildInputSourceRoleV1(Object.keys(activeJson.targets??{}).length>0,false,Object.values(activeJson.targets??{}).map(target=>target.metadata?.semio?.sourceInputObservation))})==="compile":nativeCandidate,pipeline=Object.values(activeJson.targets??{}).some(target=>target.metadata?.semio?.release);
    const assemblyAuthority=native||pipeline?projectAssemblyAuthorityV1(workspaceRoot,native?nativeRoot:undefined,projectsByRoot,pipeline?componentRoots:undefined,nativeCache,nativeClosures):null;
    const authorityMilliseconds=performance.now()-started;
    const cacheKey=createHash("sha256").update(root).digest("hex").slice(0,24),cachePath=join(workspaceDataDirectory(workspaceRoot),"np",cacheKey+".json"),head=[projectAuthority,registryDigest,native?commandRevision:null,native||pipeline?assemblyAuthority:null,...(late?["closure-references/4"]:[])];
    result.projects[name]=await tagProjectFailureV1(root,()=>observeProjectInferenceV1({cachePath,key:projectRecordKeyV1(head,root,json,selectedTargets,selectedInputs),owner:root,stamps:projectStampPathsV1({native}),reusable:value=>projectRecordReusableV1(value)&&inferenceInventoryReferencesHoldV1(workspaceRoot,value,context.phaseControl),prepare:farm?()=>farm.prepare({name,root,json,native,pipeline,nativeRoot,selectedInputs,head,cacheKey}):undefined},()=>inferProjectBodyV1(projectEnvironment,{name,root,json,native,pipeline,nativeRoot,selectedTargets,selectedInputs},options?.ownerAdmission)));
    let rows=projectInferenceObservationsV1(cachePath)??[],derived=[];
    if(late&&!result.projects[name].unavailable){
      try{const resolved=await tagProjectFailureV1(root,()=>resolveClosureRefsV1(result.projects[name],workspaceRoot,scripts));result.projects[name]=resolved.project;rows=[...rows,...resolved.rows,...resolved.derived.map(fact=>fact.row)];derived=resolved.derived;}
      catch(error){if(error?.code!=="BUILD_SOURCE_UNPARSEABLE")throw error;const unavailable=unavailableProjectV1(root,name,workspaceRoot,error);rows=[...rows,...withheldRowsV1(error)];result.projects[name]={unavailable};}
    }
    options?.onOwnerObservations?.(name,rows,derived);
    if(result.projects[name].unavailable){unavailable.set(root,result.projects[name].unavailable);delete result.projects[name];return;}
    result.projects[name].targets={...inactive,...result.projects[name].targets};
  };
  const phases=context.invocation?createGraphPhaseClockV1({publish:async resource=>{context.invocation.control.signal.throwIfAborted();await context.invocation.control.publish({owner:context.invocation.policy.owner,command:"build:projects",stage:"running",activity:{resource,mode:"shared",elapsedMs:0}});},write:writeGraphPhaseRowV1}):undefined;let finished=0;
  await phases?.begin("inference",jobs.length);
  const farm=options?.ownerAdmission?undefined:openProjectFarmV1({workspaceRoot,context,contracts,projects:jobs.length,lateClosures:late,commandInputs:closureHandleOfV1(commandInputs)?{closureRecipe:closureHandleOfV1(commandInputs).recipe}:commandInputs,projectsByRoot,componentRoots,signal:context.invocation?.control.signal,processed:()=>projectIndex});
  const limit=Math.min(projectConcurrencyV1(options),jobs.length),governor=limit>0?createGraphParentMemoryGovernorV1({workers:limit,signal:context.phaseControl?.signal,report:line=>process.stderr.write(line+"\n"),environment:process.env}):undefined;
  const active=new Map(),publishRunning=()=>{const at=performance.now();return phases?.running("inference",[...active].sort((a,b)=>a[1]-b[1]).map(([name,since])=>({name,seconds:(at-since)/1000})));};
  const loop=async()=>{while(!stopped&&cursor<jobs.length){{const job=jobs[cursor++];await governor.run(async()=>{active.set(job.name,performance.now());await publishRunning();try{if(collector)await collector.guard({project:job.name,root:job.json.root,phase:"project"},()=>inferProject(job));else await inferProject(job);}finally{active.delete(job.name);}});}await publishRunning();await phases?.advance("inference",++finished);}};
  const ticker=phases?setInterval(()=>{Promise.resolve(publishRunning()).catch(()=>{});},RUNNING_TICK_MS):undefined;ticker?.unref?.();
  const workers=Array.from({length:limit},loop);
  farm?.onReady(loops=>{for(let extra=loops-workers.length;extra>0&&!stopped&&cursor<jobs.length;extra--)workers.push(loop());});
  try{await awaitGrowingV1(workers);}catch(error){stopped=true;await Promise.allSettled(workers);throw error;}finally{clearInterval(ticker);governor?.dispose();await farm?.close();}
  await phases?.end("inference",{projects:jobs.length,...(governor?{memory:governor.stats()}:{})});
  if (collector) await collector.finish({ projects: jobs.length });
  if(wantedRoots)for(const entry of results)entry[1].projects=Object.fromEntries(Object.entries(entry[1].projects??{}).filter(([,project])=>wanted(project.root)));
  for(const row of [...unavailable.values()].sort((a,b)=>a.root<b.root?-1:a.root>b.root?1:0))results.push([row.source,{projects:{},unavailable:[row]}]);
  if (options?.analyzeLockfile && existsSync(join(workspaceRoot, javascriptManagerV1.primaryLockName))) results.push([javascriptManagerV1.primaryLockName, { externalNodes: readBunLockGraph(workspaceRoot).externalNodes }]);
  return results;
}
