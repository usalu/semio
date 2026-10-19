/** 🧱️ Normalizes one project's targets and named inputs into its generated Build configuration. */
import { advanceScriptInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { checkResourceLeaseControl } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { checkScriptInvocation, requireIssuedResourceControlInvocationV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { declaredNativeOwnerCommandV1, bindBuildCommandV1 } from "../../../../🏗️build/📤️arguments/🟨️.mjs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { observedSourceAnalysisV1, observedFs } from "../../../../🏗️build/👁️observation/🟨️.mjs";
import { repositoryCachePartitionRootDataV1 } from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts";
import { repositoryWasmPackageNxOutputV1 } from "../../../../⚡️caching/📦️artifacts/🕸️wasm/📋️contract/🟦️.ts";
import { withHostPlatformOutputsV1 } from "../../../🎯️host-platform/🟦️.ts";
const { existsSync, readFileSync } = observedFs;

/** 🏷️ Names this component as its roster entry does. */
export const component = "project-assembly";
let libraryBootstrap, nativeDependencyRoots, buildPath, LIBRARY_ROOT, readTaxonomyProjectionV1, readToml, POLICY, PROJECT_BASENAME, cargoSourceInputs, createScriptInputCache, loaded, yieldInferenceV1, SOURCE_INPUT_MODULE, TAXONOMY, SCRIPT_BASENAME, goManifest, relativeScriptInputs, resolveOutputPath, exampleCollectionInputs, owned, withWasmTooling, rootCommandTargets, cargoTargets, componentBuildTargets, printDocumentTargets, prepareSourceAnalysesV1, generatorContractInputs, generatorOutputOwners, genericCommandFallbackInputs, targetWithDefaults, targetPolicy, targetExecutionKindV1, nativeTargetCommandInputs, targetScriptClosure, nativeLockInputs, genericTargetCommandInputs, outputRootInputs, generatorOutputCouplingInputs;

/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */
export function wire(registry) {
  ({ libraryBootstrap, loaded } = registry.bootstrap);
  ({ nativeDependencyRoots, readToml, cargoSourceInputs, goManifest } = registry.nativeManifests);
  ({ buildPath, LIBRARY_ROOT, POLICY, PROJECT_BASENAME, SOURCE_INPUT_MODULE, SCRIPT_BASENAME, exampleCollectionInputs, owned } = registry.coordinates);
  ({ readTaxonomyProjectionV1, TAXONOMY, prepareSourceAnalysesV1 } = registry.sourceAnalyses);
  ({ createScriptInputCache, yieldInferenceV1, relativeScriptInputs, genericCommandFallbackInputs, nativeTargetCommandInputs, targetScriptClosure, nativeLockInputs, genericTargetCommandInputs } = registry.scriptInputs);
  ({ resolveOutputPath, generatorContractInputs, generatorOutputOwners, outputRootInputs, generatorOutputCouplingInputs } = registry.generatorOutputs);
  ({ withWasmTooling, targetWithDefaults, targetPolicy, targetExecutionKindV1 } = registry.targetPolicy);
  ({ rootCommandTargets, cargoTargets, componentBuildTargets, printDocumentTargets } = registry.componentTargets);
}

/** 👁️ Projects actual Cargo/Nx compilation and owner generator inputs for live source watching. */
export async function nativeSourceWatchPlanV1(root, workspaceRoot) {
  await libraryBootstrap;
  const roots = nativeDependencyRoots(root, workspaceRoot), includes = new Set(), excludes = new Set(), files = new Set();
  const taxonomyPath = join(workspaceRoot, buildPath(relative(workspaceRoot, join(LIBRARY_ROOT, "🔣️taxonomy.json"))));
  const contracts = existsSync(taxonomyPath) ? readTaxonomyProjectionV1(taxonomyPath).generatorContracts : {};
  for (const nativeRoot of roots) {
    const manifest = readToml(join(workspaceRoot, nativeRoot, "Cargo.toml"));
    let owner = resolve(workspaceRoot, nativeRoot);
    if (manifest.package?.workspace) owner = resolve(owner, manifest.package.workspace);
    else while (owner !== workspaceRoot && (!existsSync(join(owner, "Cargo.toml")) || !readToml(join(owner, "Cargo.toml")).workspace)) owner = dirname(owner);
    for (const name of ["Cargo.toml", "Cargo.lock"]) {
      const file = join(owner, name), local = buildPath(relative(workspaceRoot, file));
      if (local.startsWith("../")) throw Error("Native watch workspace authority escapes repository");
      includes.add(local);
    }
    for (const path of POLICY.toolchains.cargo.files) includes.add(path);
    const projectPath = join(workspaceRoot, nativeRoot, PROJECT_BASENAME);
    const project = existsSync(projectPath) ? JSON.parse(readFileSync(projectPath, "utf8")) : {};
    for (const path of cargoSourceInputs(nativeRoot, workspaceRoot, new Map(), false) ?? []) files.add(path.replace(/^\{workspaceRoot\}\//u, ""));
    const declarations = await projectInputs(project, nativeRoot, workspaceRoot, new Map(), createScriptInputCache(), "source");
    const visit = (values, seen = new Set()) => {
      for (const value of values) {
        if (typeof value !== "string") continue;
        if (declarations[value]) {
          if (seen.has(value)) throw Error("Native watch named input cycle: " + value);
          visit(declarations[value], new Set([...seen, value])); continue;
        }
        const negative = value.startsWith("!"), path = (negative ? value.slice(1) : value).replaceAll("{projectRoot}", nativeRoot).replace(/^\{workspaceRoot\}\//u, "");
        if (path.includes("{") && /\{(?:workspaceRoot|projectRoot)\}/u.test(path)) throw Error("Unresolved native watch source coordinate");
        if (isAbsolute(path) || path.split("/").includes("..") || path.includes("\\")) throw Error("Native watch source input escapes workspace");
        (negative ? excludes : includes).add(path);
      }
    };
    visit(declarations.nativeSources);
  }
  for (const contract of Object.values(contracts)) if (contract.nativeConsumers?.some(path => roots.includes(path))) {
    for (const pattern of contract.inputPatterns ?? []) (pattern.startsWith("!") ? excludes : includes).add(pattern.replace(/^!/u, ""));
  }
  if (!includes.size) throw Error("Native source watch requires admitted compilation inputs");
  const ordered = values => [...values].sort((a, b) => Buffer.from(a).compare(Buffer.from(b)));
  return { schema: "semio.framework.os.plugin.source-watch-plan/v1", includes: ordered(includes), excludes: ordered(excludes), files: ordered(files) };
}

/** 🧬️ Derives named task inputs from explicit relative source-graph contracts. */
export async function declaredSourceInputs(json,workspaceRoot,context={}) {
  const groups={};
  for(const[name,path]of Object.entries(json.metadata?.sourceInputs??{})){
    if(["default","production","nativeSources","nativeTestSources","artifactSources","artifactCommandSources"].includes(name)||Object.hasOwn(json.namedInputs??{},name))throw Error(`Duplicate source input group ${name}`);
    if(typeof path!=="string"||isAbsolute(path)||path.split(/[\\/]/).includes(".."))throw Error(`Invalid source input contract path ${path}`);
    const contractPath=resolve(workspaceRoot,path),contract=loaded.readSourceInputContract(contractPath),sources=await loaded.relativeSourceInputsAsyncV1(contract,workspaceRoot,()=>yieldInferenceV1(context,"build:source-contracts",0));
    groups[name]=[...new Set([contractPath,join(LIBRARY_ROOT,SOURCE_INPUT_MODULE),join(LIBRARY_ROOT,"🕸️dependencies/🟦️typescript/🔣️schema.json"),...sources.files])].map(file=>`{workspaceRoot}/${buildPath(relative(workspaceRoot,file))}`).concat([{externalDependencies:sources.externalDependencies}]);
  }
  return groups;
}

/**
 * 🚧️ Nx plans a task's `!{workspaceRoot}/…` negations against the workspace positives of its expanded inputs; with none, a
 * negation plans every other workspace file. A native source list names its files literally, so without a workspace positive
 * (a workspace-root crate) it keeps only its project-rooted negations.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⚡️workspace-negation-closure/🟦️.ts
 */
function workspaceNegationClosed(inputs) {
  return inputs.some((input) => typeof input === "string" && input.startsWith("{workspaceRoot}/")) ? inputs : inputs.filter((input) => typeof input !== "string" || !input.startsWith("!{workspaceRoot}/"));
}

/** 📥️ Shared command implementation and host identity are inputs of every script-backed task. */
export async function projectInputs(json, root, workspaceRoot, facts, scripts=createScriptInputCache(), sourceRepresentation = "hash", sourceGroups) {
  return loaded.inferProjectInputs(json, root, workspaceRoot, facts, scripts, { POLICY, TAXONOMY, LIBRARY_ROOT, SCRIPT_BASENAME, cargoSourceInputs, goManifest, buildPath, relativeScriptInputs, resolveOutputPath, exampleCollectionInputs, workspaceNegationClosed, declaredSourceInputs:sourceGroups===undefined?(json,root)=>declaredSourceInputs(json,root,scripts.context):()=>sourceGroups, owned, sourceRepresentation });
}

/**
 * @param {Record<string, any>} json
 * @param {string} root
 * @param {string} projectDir
 */
export async function projectWithDefaults(json, root, projectDir, workspaceRoot, contracts = {}, facts, commandInputs, scripts=createScriptInputCache(), sourceGroups, sourceRepresentation = "hash", selectedTargets) {
  const ownsScript = existsSync(join(projectDir, SCRIPT_BASENAME));
  const nativeRoot = json.metadata?.nativeRoot ?? root;
  if (json.metadata?.nativeRoot && (typeof nativeRoot !== "string" || isAbsolute(nativeRoot) || nativeRoot.split(/[\\/]/).some(part => ["", ".", ".."].includes(part)) || !existsSync(join(workspaceRoot, nativeRoot, "Cargo.toml")))) throw new Error(`Invalid native package root for ${json.name}: ${nativeRoot}`);
  const nativeProject = existsSync(join(workspaceRoot, nativeRoot, "Cargo.toml"));
  const artifactTypeScript = json.tags?.includes("role:artifact") && json.tags.includes("language:typescript");
  const declared = withWasmTooling({ ...(root === "." && ownsScript ? rootCommandTargets(join(projectDir, SCRIPT_BASENAME)) : {}), ...await cargoTargets(root, workspaceRoot, commandInputs,scripts), ...json.targets, ...componentBuildTargets(root, workspaceRoot), ...await printDocumentTargets(json, root, workspaceRoot, scripts) }, ownsScript && json.targets?.wasm ? observedSourceAnalysisV1(prepareSourceAnalysesV1().script,join(projectDir,SCRIPT_BASENAME)).wasm : false);
  for (const contract of Object.values(contracts)) {
    if (contract.ownership !== "owned" || contract.ownerPath !== root) continue;
    const name = contract.target.slice(contract.target.lastIndexOf(":") + 1), target = declared[name];
    if (!target || contract.target !== `${json.name}:${name}`) throw new Error(`Generator target has no project owner: ${contract.target}`);
    const discovery = generatorContractInputs(contract);
    const producers = [...new Set(contract.outputRoots.flatMap((output) => output.producer && output.producer.target !== contract.target ? [output.producer.target] : []))].sort();
    const inputs = [...(target.inputs ?? ["default", "^default"]), ...discovery.inputs, ...(producers.length ? [{ dependentTasksOutputFiles: "**/*" }] : [])];
    const dependencies = [...discovery.dependsOn, ...producers];
    declared[name] = { ...target, inputs, ...(dependencies.length ? { dependsOn: [...new Set([...(target.dependsOn ?? []), ...dependencies])] } : {}) };
  }
  for (const [target, outputs] of generatorOutputOwners(contracts, root, json.name, declared)) declared[target.slice(json.name.length + 1)] = { ...declared[target.slice(json.name.length + 1)], outputs };
  const admitted=selectedTargets===undefined?declared:Object.fromEntries(Object.entries(declared).filter(([name])=>selectedTargets.includes(name)));
  const normalized = {};
  const genericFallback = genericCommandFallbackInputs(workspaceRoot);
  const commandSourceGroups = new Map();
  const internCommandSources = (inputs) => {
    if (!Array.isArray(inputs) || !inputs.length) return [];
    const key = JSON.stringify(inputs);
    let group = commandSourceGroups.get(key);
    if (!group) {
      group = { name: `commandSources${commandSourceGroups.size}`, inputs };
      commandSourceGroups.set(key, group);
    }
    return [group.name];
  };
  for (const [name, target] of Object.entries(admitted)) {
    const inheritedBase=targetWithDefaults({ ...(POLICY.targetDefaults?.[name] ?? {}), ...target,...(target.cache===false&&target.inputs===undefined?{inputs:["default"]}:{}) }, root, ownsScript);
    const inherited=inheritedBase.metadata?.semio?.hostPlatformOutputsV1?withHostPlatformOutputsV1(inheritedBase,process.platform):inheritedBase;
    if(typeof inherited.options?.cwd==="string")inherited.options={...inherited.options,cwd:inherited.options.cwd.replaceAll("{projectRoot}",root).replaceAll("{workspaceRoot}",workspaceRoot)};
    const policy = targetPolicy(name,inherited);
    const execution = targetExecutionKindV1(target,`${json.name}:${name}`), orchestrator = execution === "orchestration" || target.metadata?.semio?.orchestrator === true;
    if (execution === "native" && !nativeProject) throw Error(`Native execution contract requires a native project owner: ${json.name}:${name}`);
    const nativePolicyTarget = execution !== "source" && !orchestrator && nativeProject && (execution === "native" || (await nativeTargetCommandInputs({...policy,configurations:undefined},workspaceRoot,commandInputs,scripts,root)).some(path=>path.includes("/🏃️process/🧪️testing/🦀️cargo/")||path.includes("/🏃️process/📦️artifacts/🏗️native-build/")||path.includes("/🏃️process/📦️artifacts/🕸️wasm-build/")));
    const nativeTarget = execution !== "source" && !orchestrator && (nativePolicyTarget || nativeProject && /^(?:build|wasm|native|test|lint|check|verify)(?:-|$)/.test(name) || policy.options?.command?.includes("⚡️caching/🦀️cargo/📜️script.ts"));
    const nativeTests = execution === "native" ? target.metadata.semio.execution.scope === "test" : name.startsWith("test");
    const artifactTarget = artifactTypeScript && /^(?:build|check|test)$/.test(name);
    if(nativePolicyTarget){
      const command=policy.options?.command;
      if(typeof command!=="string"||!command.startsWith("bun "))throw Error(`Cargo owner test requires one Bun script command: ${json.name}:${name}`);
      const driver=buildPath(relative(workspaceRoot,join(LIBRARY_ROOT,"⚡️caching/🦀️cargo/📜️script.ts")));
      const ownerCwd=policy.options?.cwd??root;
      const boundCommand=bindBuildCommandV1(command,workspaceRoot,ownerCwd,root,[]).command;
      policy.executor="repo:script";
      let nativeOwnerCommand;try{nativeOwnerCommand=declaredNativeOwnerCommandV1(`${nativeRoot}/Cargo.toml`,ownerCwd,boundCommand);}catch(error){throw new Error(`Original discovered native command owner ${json.name}:${name}: ${error.message}`,{cause:error});}
      policy.options={...policy.options,ownerArguments:true,cwd:".",nativeOwnerCommand,command:`bun ${JSON.stringify(driver)} native owner-command --manifest ${JSON.stringify(`${nativeRoot}/Cargo.toml`)} --cwd ${JSON.stringify(ownerCwd)} -- ${boundCommand}`};
      const manifest=readToml(join(workspaceRoot,nativeRoot,"Cargo.toml"));let scope=resolve(workspaceRoot,nativeRoot);
      if(manifest.package?.workspace)scope=resolve(scope,manifest.package.workspace);
      else while(scope!==workspaceRoot && (!existsSync(join(scope,"Cargo.toml"))||!readToml(join(scope,"Cargo.toml")).workspace))scope=dirname(scope);
      const config=buildPath(relative(workspaceRoot,join(scope,".config","nextest.toml")));
      if(config.startsWith("../"))throw Error(`Cargo test policy escapes workspace: ${nativeRoot}`);
      policy.cargoTestPolicyInputs=[`{workspaceRoot}/${config}`];
    }
    if (execution !== "source" && !orchestrator && !/ native owner-command /u.test(policy.options?.command??"") && (await targetScriptClosure({...policy,configurations:undefined}, workspaceRoot, scripts, root))?.some(path=>path.includes("/🏃️process/🧪️testing/🧪️vitest/")||path.includes("/🏃️process/📋️context/"))) {
      const command=policy.options?.command,ownerCwd=policy.options?.cwd??root;
      if(typeof command!=="string"||!command.startsWith("bun "))throw Error(`Process owner requires one Bun script command: ${json.name}:${name}`);
      const driver=buildPath(relative(workspaceRoot,join(LIBRARY_ROOT,"🏃️process/🧭️routing/📥️owner-command/📜️script.ts")));
      const bound=command.replace(/^bun\s+("[^"\n]+"|'[^'\n]+'|[^\s]+)/u,(_,source)=>`bun ${JSON.stringify(buildPath(resolve(workspaceRoot,ownerCwd,source.replace(/^["']|["']$/g,""))))}`);
      policy.executor="repo:script";
      policy.options={...policy.options,ownerArguments:true,cwd:".",command:`bun ${JSON.stringify(driver)} owner-command --cwd ${JSON.stringify(ownerCwd)} -- ${bound}`};
    }
    if (nativeTarget) {
      policy.inputs = [nativeTests ? "nativeTestSources" : "nativeSources", nativeTests ? "^nativeTestSources" : "^nativeSources", ...internCommandSources(await nativeTargetCommandInputs(policy, workspaceRoot, commandInputs, scripts, root)), ...nativeLockInputs(policy),...(policy.cargoTestPolicyInputs??[])];
      delete policy.cargoTestPolicyInputs;
    }
    if (nativeTarget && nativeProject && readToml(join(workspaceRoot, nativeRoot, "Cargo.toml")).package) {
      const lockOwner = join(LIBRARY_ROOT, "🏗️build/📥️inputs/🔒️lock/📜️script.ts");
      policy.inputs.push(...await relativeScriptInputs([lockOwner], workspaceRoot, scripts), `{workspaceRoot}/${buildPath(relative(workspaceRoot, join(dirname(lockOwner), "🔣️schema.json")))}`, loaded.cargoLockInput(`${nativeRoot}/Cargo.toml`, workspaceRoot));
    }
    if (artifactTarget) policy.inputs = [...new Set([...(name === "build" ? [] : ["default", "^default"]), "artifactSources", "artifactCommandSources", ...(target.inputs ?? [])])];
    if (!nativeTarget && !artifactTarget && policy.cache) policy.inputs = [...(policy.inputs ?? ["default", "^default"]), ...internCommandSources(await genericTargetCommandInputs(policy, workspaceRoot, genericFallback, scripts, root))];
    if (artifactTarget && ["build", "test"].includes(name)) { policy.cache = false; policy.outputs = ["{projectRoot}/📤️dist/{options.env.SEMIO_BUILD_MODE}", "{projectRoot}/dist"]; }
    if(policy.metadata?.semio?.wasmPackageSourceV1){policy.cache=false;policy.outputs=[...new Set([repositoryWasmPackageNxOutputV1(workspaceRoot,repositoryCachePartitionRootDataV1(workspaceRoot,"products"),policy.metadata.semio.wasmPackageSourceV1),...(target.outputs??[])])];}
    else if (nativeProject && json.name !== "@semio-tech/framework-renderer-wgpu" && /^wasm(?:-release)?$/.test(name)) { policy.cache = false; policy.outputs = ["{projectRoot}/dist/wasm/{options.env.SEMIO_BUILD_MODE}"]; }
    if (POLICY.serialTargets?.includes(name)) policy.parallelism = false;
    normalized[name] = loaded.targetBuildModes(name, policy);
  }
  for (const contract of Object.values(contracts)) {
    if (contract.ownership !== "owned" || contract.ownerPath !== root || !contract.checkTarget) continue;
    const check = contract.checkTarget.slice(contract.checkTarget.lastIndexOf(":") + 1), target = normalized[check];
    if (!target) continue;
    const discovery = generatorContractInputs(contract);
    normalized[check] = { ...target, cache: target.cache !== false, inputs: [...(target.inputs ?? ["default", "^default"]), ...discovery.inputs, ...contract.outputRoots.map((output) => outputRootInputs(output, workspaceRoot))], ...(discovery.dependsOn.length ? { dependsOn: [...new Set([...(target.dependsOn ?? []), ...discovery.dependsOn])] } : {}) };
  }
  for (const [name, target] of Object.entries(normalized)) {
    if (target.cache !== true) continue;
    const extra = generatorOutputCouplingInputs(name, target, declared, root, workspaceRoot);
    if (extra.length) normalized[name] = { ...target, inputs: [...target.inputs, ...extra] };
  }
  const namedInputs = await projectInputs({ ...json, targets: admitted }, root, workspaceRoot, facts, scripts,sourceRepresentation,sourceGroups);
  for (const { name, inputs } of commandSourceGroups.values()) {
    if (Object.hasOwn(namedInputs, name)) throw new Error(`Duplicate command source group ${name}`);
    namedInputs[name] = sourceRepresentation === "source" ? loaded.deduplicateProjectInputs(inputs, root) : loaded.compactSourceInputs(inputs, workspaceRoot, root);
  }
  return { ...json, name: json.name, root, namedInputs, targets: normalized };
}

/** 🔎️ Queries source-project normalization through the same production stages without publishing graph facts. */
export async function inferReadOnlyBuildProjectSourcesV1(json, root, workspaceRoot, invocation,phaseControl) {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
  await libraryBootstrap;checkResourceLeaseControl(phaseControl);
  if(existsSync(join(workspaceRoot,json.metadata?.nativeRoot??root,"Cargo.toml")))throw Error("Read-only source project normalization requires a non-Cargo owner");
  await advanceScriptInvocation(invocation,"build:project-sources","running");
  const taxonomy=join(workspaceRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"),contracts=existsSync(taxonomy)?readTaxonomyProjectionV1(taxonomy).generatorContracts:{};
  const result=await projectWithDefaults(json,root,join(workspaceRoot,root),workspaceRoot,contracts,new Map(),undefined,createScriptInputCache({workspaceRoot,invocation,phaseControl}),undefined,"source");
  checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
  return result;
}
