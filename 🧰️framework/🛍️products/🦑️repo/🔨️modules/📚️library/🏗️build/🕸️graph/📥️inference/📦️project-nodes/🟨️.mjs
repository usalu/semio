import{applyBuildGeneratedTaskDependenciesV1,buildInputSourceRoleV1,buildNativeSourceObservationV1}from"../../🚪️admission/🟦️.ts";
import {createRequire} from "node:module";
const javascriptManagerV1 = createRequire(import.meta.url)("../../../../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🟨️.cjs").selectedManagerV1("javascript");
/** 📦️ Infers every project node of one workspace snapshot with bounded concurrency and settlement. */
import { advanceScriptInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { availableParallelism } from "node:os";
import { createHash } from "node:crypto";
import { dirname, join, relative } from "node:path";
import { inferenceInventoryReferencesHoldV1 } from "../../../../🏗️build/👁️observation/📥️reuse/🟨️.mjs";
import { observeInferenceSnapshotV1, observeProjectInferenceV1, observedFs, projectInferenceObservationsV1 } from "../../../../🏗️build/👁️observation/🟨️.mjs";
import { statSync as rawStatSync } from "node:fs";
const { existsSync, readFileSync } = observedFs;

/** 🏷️ Names this component as its roster entry does. */
export const component = "project-nodes";
import { createGraphPhaseClockV1, writeGraphPhaseRowV1, writeGraphSettleRowV1 } from "../../⏱️phases/🟨️.mjs";
import { createMemoryGovernorV1, graphMemoryBudgetBytesV1, measureMemoryBaselineV1 } from "../../../🧠️memory/🟨️.mjs";
import { createErrorCollectorFromEnvV1 } from "../../🧾️collect-errors/🟨️.mjs";
import { openProjectFarmV1 } from "./🏊️project-farm/🟨️.mjs";
import { awaitGrowingV1 } from "./🏊️project-farm/🏊️pool/🟨️.mjs";
let inferenceCandidates,readBuildGeneratedCompileOutputV1,buildPath, yieldInferenceV1, catalogBuildContributionsV1, discoverNativeBuildOwnersV1, createScriptInputCache, POLICY, nativeCommandInputs, readTaxonomyProjectionV1, collectPlaygroundCatalog, PROJECT_BASENAME, playgroundSessionTargets, playgroundPreparationTargets, playgroundSiteTargetsForAdmissionV1, readToml, cargoTargets, componentBuildTargets, componentCrateRoots, projectAssemblyAuthorityV1, workspaceDataDirectory, projectWithDefaults, withNativePreparation, withReleasePipelinePreparation, loaded, readBunLockGraph, projectStampPathsV1;

/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */
export function wire(registry) {
  ({inferenceCandidates,readBuildGeneratedCompileOutputV1}=registry.ownerDiscovery);
  ({ buildPath, POLICY, PROJECT_BASENAME, workspaceDataDirectory } = registry.coordinates);
  ({ yieldInferenceV1, createScriptInputCache, nativeCommandInputs } = registry.scriptInputs);
  ({ catalogBuildContributionsV1, discoverNativeBuildOwnersV1, projectAssemblyAuthorityV1 } = registry.ownerDiscovery);
  ({ readTaxonomyProjectionV1 } = registry.sourceAnalyses);
  ({ collectPlaygroundCatalog, playgroundSessionTargets, playgroundPreparationTargets, playgroundSiteTargetsForAdmissionV1 } = registry.playgroundTargets);
  ({ readToml, withNativePreparation } = registry.nativeManifests);
  ({ cargoTargets, componentBuildTargets, componentCrateRoots, withReleasePipelinePreparation } = registry.componentTargets);
  ({ projectWithDefaults } = registry.projectAssembly);
  ({ loaded } = registry.bootstrap);
  ({ readBunLockGraph } = registry.bunLock);
  ({ projectStampPathsV1 } = registry.stamps);
}

/** ⏱️ A source modified this recently may be half-written, so its parse failure is retried by settlement instead of withholding its project (the observation racy rule). */
const UNSETTLED_SOURCE_MS = 2000;

/** 🚧️ Withholds one project whose consumed source does not parse, naming the exact bytes the snapshot observed; a freshly modified source rethrows so settlement waits for it. */
function unavailableProjectV1(root, project, workspaceRoot, error) {
  let modified = Infinity;
  try { modified = rawStatSync(error.source).mtimeMs; } catch {}
  if (modified >= Date.now() - UNSETTLED_SOURCE_MS) throw error;
  return { root, ...(project ? { project } : {}), source: buildPath(relative(workspaceRoot, error.source)), sha256: error.sha256, reason: String(error.message).slice(0, 2048) };
}

/** 🏷️ Names the project a failing inference belongs to without replacing the error, so a repeated settlement round says which project stops the build. */
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

/** 🧱️ Builds, from plain data, what the inference of one project shares across the projects of a graph construction; a worker process opens the session its parent describes. */
export function createProjectInferenceEnvironmentV1(session) {
  const { workspaceRoot, registry, commandInputs, projectsByRoot, componentRoots } = session, context = { workspaceRoot, registry }, scripts = createScriptInputCache(context);
  const contractPath = join(workspaceRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
  const contracts = existsSync(contractPath) ? readTaxonomyProjectionV1(contractPath).generatorContracts : {};
  scripts.generatedOutput = path => readBuildGeneratedCompileOutputV1(path, workspaceRoot, contracts, registry);
  return { workspaceRoot, contracts, commandInputs, scripts, projectsByRoot: new Map(projectsByRoot), componentRoots, nativeCache: new Map(), nativeClosures: new Map(), context };
}

/** 🧱️ Infers the node of one project: its defaults, its native or release preparation and its build modes; a source that does not parse withholds the project instead of failing the graph. */
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
  } catch (error) { if (error?.code !== "BUILD_SOURCE_UNPARSEABLE") throw error; return { unavailable: unavailableProjectV1(root, name, workspaceRoot, error) }; }
}

/** 🔑️ The key a project record is stored under: what the graph shares across projects, then everything the project itself is made of. */
export const projectRecordKeyV1 = (head, root, json, selectedTargets, selectedInputs) => [...head, root, json, selectedTargets ?? null, selectedInputs];

/** 🧾️ Writes the persisted record of one project into the graph directories the running body is staged to, under the key and the observation scope the parent uses, so the parent finds it valid once it adopted the leaves. */
export async function produceProjectRecordV1(environment, job) {
  const cachePath = join(workspaceDataDirectory(environment.workspaceRoot), "np", job.cacheKey + ".json");
  await observeInferenceSnapshotV1(() => observeProjectInferenceV1({ cachePath, key: projectRecordKeyV1(job.head, job.root, job.json, undefined, job.selectedInputs), owner: job.root, stamps: projectStampPathsV1({ native: job.native }), reusable: () => true }, () => inferProjectBodyV1(environment, job, undefined)));
}

/** 🛡️ Infers the project nodes of every authored emoji-named project config. Nx's native walker sometimes hands back lossy paths with U+FFFD where a multi-byte emoji used to be: those are unopenable duplicates of a real file and are skipped. */
export async function emojiProjectJsonNodes(configFiles, options, context) {
  const { workspaceRoot } = context, unavailable = new Map(), wantedRoots = options?.ownerRoots, wanted = root => !wantedRoots || wantedRoots.has(root);
  const collector = createErrorCollectorFromEnvV1({ workspaceRoot, signal: context.invocation?.control.signal });
  const catalogContributions=await catalogBuildContributionsV1(workspaceRoot,options?.catalogProviders??[]);
  const owners=options?.ownerAdmission?{files:inferenceCandidates(configFiles,workspaceRoot),nativeCache:new Map(),nativeClosures:new Map()}:await discoverNativeBuildOwnersV1(configFiles,workspaceRoot,context),{nativeCache,nativeClosures}=owners;configFiles=owners.files;
  const rootsByName = new Map(), scripts = createScriptInputCache(context);
  const contractPath = join(workspaceRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
  const contracts = existsSync(contractPath) ? readTaxonomyProjectionV1(contractPath).generatorContracts : {};
  scripts.generatedOutput=path=>readBuildGeneratedCompileOutputV1(path,workspaceRoot,contracts,context.registry);
  if(context.invocation)await advanceScriptInvocation(context.invocation,"build:commands","running");
  const commandInputs = !options?.ownerAdmission&&configFiles.some((path) => path.endsWith("Cargo.toml") && !path.includes(".🧬semio") && !POLICY.generatedDirectories.some((name) => path.split("/").includes(name))) ? await nativeCommandInputs(workspaceRoot, scripts) : undefined;
  if(context.invocation)await advanceScriptInvocation(context.invocation,"build:commands","complete");
  const projectAuthority=createHash("sha256").update(JSON.stringify([contracts])).digest("hex");
  const commandRevision=createHash("sha256").update(JSON.stringify(commandInputs??null)).digest("hex"),registryDigest=createHash("sha256").update(JSON.stringify(context.registry??null)).digest("hex");
  const playgroundCatalog = collectPlaygroundCatalog(configFiles, workspaceRoot,catalogContributions);

  const results = (await Promise.all(configFiles
    .filter((file) => file.endsWith(PROJECT_BASENAME))
    .filter((configFile) => {
      if (configFile.includes("\uFFFD")) return false;
      if (configFile.includes(".🧬semio") || POLICY.generatedDirectories.some((name) => configFile.split("/").includes(name))) {
        return false;
      }
      return true;
    })
    .map(async (configFile) => {
      const abs = join(workspaceRoot, configFile);
      if (!existsSync(abs)) return null;
      const bytes = readFileSync(abs);
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
    if (configFile.includes("\uFFFD") || configFile.startsWith("compose/") || configFile.startsWith("temp/compose/") || configFile.includes(".🧬semio") || POLICY.generatedDirectories.some((name) => configFile.split("/").includes(name))) continue;
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
    const cacheKey=createHash("sha256").update(root).digest("hex").slice(0,24),cachePath=join(workspaceDataDirectory(workspaceRoot),"np",cacheKey+".json"),head=[projectAuthority,registryDigest,native?commandRevision:null,native||pipeline?assemblyAuthority:null];
    result.projects[name]=await tagProjectFailureV1(root,()=>observeProjectInferenceV1({cachePath,key:projectRecordKeyV1(head,root,json,selectedTargets,selectedInputs),owner:root,stamps:projectStampPathsV1({native}),reusable:value=>inferenceInventoryReferencesHoldV1(workspaceRoot,value,context.phaseControl),prepare:farm?()=>farm.prepare({name,root,json,native,pipeline,nativeRoot,selectedInputs,head,cacheKey}):undefined},()=>inferProjectBodyV1(projectEnvironment,{name,root,json,native,pipeline,nativeRoot,selectedTargets,selectedInputs},options?.ownerAdmission)));
    options?.onOwnerObservations?.(name,projectInferenceObservationsV1(cachePath)??[]);
    if(result.projects[name].unavailable){unavailable.set(root,result.projects[name].unavailable);delete result.projects[name];return;}
    result.projects[name].targets={...inactive,...result.projects[name].targets};
  };
  const phases=context.invocation?createGraphPhaseClockV1({publish:async resource=>{context.invocation.control.signal.throwIfAborted();await context.invocation.control.publish({owner:context.invocation.policy.owner,command:"build:projects",stage:"running",activity:{resource,mode:"shared",elapsedMs:0}});},write:writeGraphPhaseRowV1}):undefined;let finished=0;
  await phases?.begin("inference",jobs.length);
  const farm=options?.ownerAdmission?undefined:openProjectFarmV1({workspaceRoot,context,projects:jobs.length,commandInputs,projectsByRoot,componentRoots,signal:context.invocation?.control.signal,processed:()=>projectIndex});
  const limit=Math.min(projectConcurrencyV1(options),jobs.length),governor=limit>0?createMemoryGovernorV1({budgetBytes:graphMemoryBudgetBytesV1(process.env),baselineBytes:measureMemoryBaselineV1(),workers:limit,signal:context.phaseControl?.signal,report:line=>process.stderr.write(line+"\n"),environment:process.env}):undefined;
  const loop=async()=>{while(!stopped&&cursor<jobs.length){{const job=jobs[cursor++],run=async()=>{if(collector)await collector.guard({project:job.name,root:job.json.root,phase:"project"},()=>inferProject(job));else await inferProject(job);};await (governor?governor.run(run):run());}await phases?.advance("inference",++finished);}};
  const workers=Array.from({length:limit},loop);
  farm?.onReady(loops=>{for(let extra=loops-workers.length;extra>0&&!stopped&&cursor<jobs.length;extra--)workers.push(loop());});
  try{await awaitGrowingV1(workers);}catch(error){stopped=true;await Promise.allSettled(workers);throw error;}finally{await farm?.close();}
  await phases?.end("inference",{projects:jobs.length,...(governor?{memory:governor.stats()}:{})});
  if (collector) await collector.finish({ projects: jobs.length });
  if(wantedRoots)for(const entry of results)entry[1].projects=Object.fromEntries(Object.entries(entry[1].projects??{}).filter(([,project])=>wanted(project.root)));
  for(const row of [...unavailable.values()].sort((a,b)=>a.root<b.root?-1:a.root>b.root?1:0))results.push([row.source,{projects:{},unavailable:[row]}]);
  if (options?.analyzeLockfile && existsSync(join(workspaceRoot, javascriptManagerV1.primaryLockName))) results.push([javascriptManagerV1.primaryLockName, { externalNodes: readBunLockGraph(workspaceRoot).externalNodes }]);
  return results;
}
