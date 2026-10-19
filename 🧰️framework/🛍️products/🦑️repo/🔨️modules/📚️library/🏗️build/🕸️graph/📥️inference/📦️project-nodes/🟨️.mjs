/** 📦️ Infers every project node of one workspace snapshot with bounded concurrency and settlement. */
import { advanceScriptInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { availableParallelism } from "node:os";
import { createHash } from "node:crypto";
import { dirname, join, relative } from "node:path";
import { inferenceInventoryReferencesHoldV1 } from "../../../../🏗️build/👁️observation/📥️reuse/🟨️.mjs";
import { observeProjectInferenceV1, observedFs } from "../../../../🏗️build/👁️observation/🟨️.mjs";
import { statSync as rawStatSync } from "node:fs";
const { existsSync, readFileSync } = observedFs;

/** 🏷️ Names this component as its roster entry does. */
export const component = "project-nodes";
let buildPath, yieldInferenceV1, catalogBuildContributionsV1, discoverNativeBuildOwnersV1, createScriptInputCache, POLICY, nativeCommandInputs, readTaxonomyProjectionV1, collectPlaygroundCatalog, PROJECT_BASENAME, playgroundSessionTargets, playgroundPreparationTargets, playgroundSiteTargetsForAdmissionV1, readToml, cargoTargets, componentBuildTargets, componentCrateRoots, projectAssemblyAuthorityV1, workspaceDataDirectory, declaredSourceInputs, projectWithDefaults, withNativePreparation, withReleasePipelinePreparation, loaded, readBunLockGraph, revisionIdentity;

/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */
export function wire(registry) {
  ({ buildPath, POLICY, PROJECT_BASENAME, workspaceDataDirectory } = registry.coordinates);
  ({ yieldInferenceV1, createScriptInputCache, nativeCommandInputs } = registry.scriptInputs);
  ({ catalogBuildContributionsV1, discoverNativeBuildOwnersV1, projectAssemblyAuthorityV1 } = registry.ownerDiscovery);
  ({ readTaxonomyProjectionV1 } = registry.sourceAnalyses);
  ({ collectPlaygroundCatalog, playgroundSessionTargets, playgroundPreparationTargets, playgroundSiteTargetsForAdmissionV1 } = registry.playgroundTargets);
  ({ readToml, withNativePreparation } = registry.nativeManifests);
  ({ cargoTargets, componentBuildTargets, componentCrateRoots, withReleasePipelinePreparation } = registry.componentTargets);
  ({ declaredSourceInputs, projectWithDefaults } = registry.projectAssembly);
  ({ loaded } = registry.bootstrap);
  ({ readBunLockGraph } = registry.bunLock);
  ({ revisionIdentity } = registry);
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

/** 🧵️ Bounds concurrently inferred projects; one worker is the serial reference order. */
function projectConcurrencyV1(options) {
  const value = options?.projectConcurrency ?? Math.max(1, Math.min(8, availableParallelism()));
  if (!Number.isSafeInteger(value) || value < 1 || value > 64) throw Error("Build project inference concurrency must be an integer from 1 to 64");
  return value;
}

/** 🪨️ Reports and cancels inference settlement rounds through the original invocation. */
export function settlementOptionsV1(context) {
  return { signal: context.phaseControl?.signal, onRound: async ({ round, conflict, failure }) => { if (process.env.SEMIO_BUILD_TRACE_AUTHORITY === "true") console.error("[DEBUG] build inference settlement round " + JSON.stringify({ round, conflict, failure: failure === undefined ? undefined : String(failure) })); await yieldInferenceV1(context,"build:settle",0,true); } };
}

export async function emojiProjectJsonNodes(configFiles, options, context) {
  const { workspaceRoot } = context, unavailable = new Map();
  const catalogContributions=await catalogBuildContributionsV1(workspaceRoot,options?.catalogProviders??[]);
  const owners=await discoverNativeBuildOwnersV1(configFiles,workspaceRoot,context),{nativeCache,nativeClosures}=owners;configFiles=owners.files;
  const rootsByName = new Map(), scripts = createScriptInputCache(context);
  if(context.invocation)await advanceScriptInvocation(context.invocation,"build:commands","running");
  const commandInputs = !options?.ownerAdmission&&configFiles.some((path) => path.endsWith("Cargo.toml") && !path.includes(".🧬semio") && !POLICY.generatedDirectories.some((name) => path.split("/").includes(name))) ? await nativeCommandInputs(workspaceRoot, scripts) : undefined;
  if(context.invocation)await advanceScriptInvocation(context.invocation,"build:commands","complete");
  const contractPath = join(workspaceRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
  const contracts = existsSync(contractPath) ? readTaxonomyProjectionV1(contractPath).generatorContracts : {};
  const projectAuthority=createHash("sha256").update(JSON.stringify([revisionIdentity.revision,contracts])).digest("hex");
  const commandRevision=createHash("sha256").update(JSON.stringify(commandInputs??null)).digest("hex"),registryDigest=createHash("sha256").update(JSON.stringify(context.registry??null)).digest("hex");
  const playgroundCatalog = collectPlaygroundCatalog(configFiles, workspaceRoot,catalogContributions);

  const results = (await Promise.all(configFiles
    .filter((file) => file.endsWith(PROJECT_BASENAME))
    .filter((configFile) => {
      // 🛡️ Nx's native walker sometimes hands back lossy paths with U+FFFD where a
      // multi-byte emoji used to be; those are unopenable duplicates of a real file.
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
    const targets = await cargoTargets(root, workspaceRoot, commandInputs,scripts);
    const project = { name, root, projectType: "library", tags: ["language:cargo", "discovery:manifest"], targets: { ...targets, ...componentBuildTargets(root, workspaceRoot), ...playgroundSiteTargetsForAdmissionV1(root,playgroundCatalog,workspaceRoot,context.registry,options?.ownerAdmission?options.ownerAdmission.targets[name]??[]:undefined) } };
    results.push([configFile, { projects: { [name]: project } }]);
  }
  const projects = results.flatMap(([, result]) => Object.values(result.projects));
  const projectsByRoot = new Map(projects.flatMap(project=>[[project.root,project.name],...(project.metadata?.nativeRoot?[[project.metadata.nativeRoot,project.name]]:[])]));
  const componentRoots = componentCrateRoots(configFiles, workspaceRoot);

  const jobs=results.flatMap(([,result])=>Object.entries(result.projects).map(([name,json])=>({result,name,json})));let projectIndex=0,cursor=0,stopped=false;
  const inferProject=async({result,name,json})=>{
    const started=performance.now();
    await yieldInferenceV1(context,"build:projects",++projectIndex);
    if(process.env.SEMIO_BUILD_TRACE_AUTHORITY==="true")console.error("[DEBUG] build project start "+JSON.stringify({index:projectIndex,total:projects.length,project:name,root:json.root}));
    const inactive=options?.ownerAdmission?Object.fromEntries(Object.entries(json.targets??{}).filter(([name])=>!options.ownerAdmission.targets[json.name]?.includes(name))):{};
    const selectedTargets=options?.ownerAdmission?.targets[json.name],activeJson=options?.ownerAdmission?{...json,targets:Object.fromEntries(Object.entries(json.targets??{}).filter(([name])=>selectedTargets?.includes(name)))}:json;
    const root=json.root, nativeRoot=json.metadata?.nativeRoot??root, native=existsSync(join(workspaceRoot,nativeRoot,"Cargo.toml"))||Object.values(activeJson.targets??{}).some(target=>target.options?.command?.includes("⚡️caching/🦀️cargo/📜️script.ts")), pipeline=Object.values(activeJson.targets??{}).some(target=>target.metadata?.semio?.release);
    const assemblyAuthority=native||pipeline?projectAssemblyAuthorityV1(workspaceRoot,native?nativeRoot:undefined,projectsByRoot,pipeline?componentRoots:undefined,nativeCache,nativeClosures):null;
    const authorityMilliseconds=performance.now()-started;
    const cachePath=join(workspaceDataDirectory(workspaceRoot),"np",createHash("sha256").update(root).digest("hex").slice(0,24)+".json");
    result.projects[name]=await observeProjectInferenceV1({cachePath,key:[projectAuthority,registryDigest,native?commandRevision:null,native||pipeline?assemblyAuthority:null,root,json,selectedTargets??null],owner:root,reusable:value=>inferenceInventoryReferencesHoldV1(workspaceRoot,value,context.phaseControl)},async()=>{
      try{
      const sourceGroups=await declaredSourceInputs(activeJson,workspaceRoot,context);
      await yieldInferenceV1(context,"build:project-assembly",0);
      const project=await projectWithDefaults(json,root,join(workspaceRoot,root),workspaceRoot,contracts,new Map(),commandInputs,scripts,sourceGroups,"hash",options?.ownerAdmission?selectedTargets??[]:undefined),preparationCache=nativeCache,dependencyRootsCache=nativeClosures;
      withNativePreparation(project,workspaceRoot,contracts,preparationCache,projectsByRoot,dependencyRootsCache);
      withReleasePipelinePreparation(project,workspaceRoot,contracts,componentRoots,preparationCache,dependencyRootsCache);
      for(const[name,target]of Object.entries(project.targets))project.targets[name]=loaded.targetBuildModes(name,target);
      return project;
      }catch(error){if(error?.code!=="BUILD_SOURCE_UNPARSEABLE")throw error;return {unavailable:unavailableProjectV1(root,name,workspaceRoot,error)};}
    });
    if(result.projects[name].unavailable){unavailable.set(root,result.projects[name].unavailable);delete result.projects[name];return;}
    result.projects[name].targets={...inactive,...result.projects[name].targets};
    if(process.env.SEMIO_BUILD_TRACE_AUTHORITY==="true")console.error("[DEBUG] build project complete "+JSON.stringify({index:projectIndex,total:projects.length,project:name,authorityMilliseconds:Math.round(authorityMilliseconds),elapsedMilliseconds:Math.round(performance.now()-started)}));
  };
  const workers=Array.from({length:Math.min(projectConcurrencyV1(options),jobs.length)},async()=>{while(!stopped&&cursor<jobs.length)await inferProject(jobs[cursor++]);});
  await Promise.all(workers).catch(async error=>{stopped=true;await Promise.allSettled(workers);throw error;});
  for(const row of [...unavailable.values()].sort((a,b)=>a.root<b.root?-1:a.root>b.root?1:0))results.push([row.source,{projects:{},unavailable:[row]}]);
  if (options?.analyzeLockfile && existsSync(join(workspaceRoot, "bun.lock"))) results.push(["bun.lock", { externalNodes: readBunLockGraph(workspaceRoot).externalNodes }]);
  return results;
}
