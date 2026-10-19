/** 🎮️ Projects playground catalog rows into session, preparation, activation, serving and site targets. */
import { advanceScriptInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { catalogBuildContributionForManifestV1 } from "../../../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🟦️.ts";
import { createHash } from "node:crypto";
import { createRequire } from "node:module";
import { dirname, join, relative, resolve } from "node:path";
import { frameworkOsEngineSourceOwnersV1 } from "../../../../../../../💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🏗️builder/🌐️vite/📥️artifacts/🕸️engines/🟦️.ts";
import { observedSourceAnalysisV1, observedFs } from "../../../../🏗️build/👁️observation/🟨️.mjs";
const { existsSync, lstatSync, readFileSync } = observedFs;

/** 🏷️ Names this component as its roster entry does. */
export const component = "playground-targets";
let POLICY, readToml, buildPath, LIBRARY_ROOT, executionTargetV1, PROJECT_BASENAME, readPhysicalSource, loaded, nativePreparation, componentModelOutputV1, PLUGIN_MODULES_ROOT, prepareSourceAnalysesV1, revisionIdentity;

/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */
export function wire(registry) {
  ({ POLICY, buildPath, LIBRARY_ROOT, PROJECT_BASENAME, readPhysicalSource, componentModelOutputV1, PLUGIN_MODULES_ROOT } = registry.coordinates);
  ({ readToml, nativePreparation } = registry.nativeManifests);
  ({ executionTargetV1 } = registry.targetPolicy);
  ({ loaded } = registry.bootstrap);
  ({ prepareSourceAnalysesV1 } = registry.sourceAnalyses);
  ({ revisionIdentity } = registry);
}

/** 🎮️ Flattens every plugin playground row from Cargo manifests (with authored `distDir`).
 * @param {readonly string[]} configFiles
 * @param {string} workspaceRoot
 * @param {readonly import("../../../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🟦️.ts").CatalogBuildContributionV1[]} [catalogContributions]
 */
export function collectPlaygroundCatalog(configFiles, workspaceRoot,catalogContributions=[]) {
  if(!catalogContributions.length)return [];
  const components = new Map(), playgrounds = [], nativeFiles = new Map(), nativeClosures = new Map();
  for (const path of configFiles) {
    if (!path.endsWith("Cargo.toml") || path.includes("\uFFFD") || path.includes(".🧬semio") || path.startsWith("compose/") || path.startsWith("temp/compose/") || POLICY.generatedDirectories.some((directory) => path.split("/").includes(directory))) continue;
    const buildCatalog=catalogBuildContributionForManifestV1(catalogContributions,path);if(!buildCatalog)continue;
    const manifest = readToml(join(workspaceRoot, path)), metadata = manifest.package?.metadata;
    if (!metadata?.component?.package || !["plugin", "extension"].includes(metadata.semio?.["component-kind"])) continue;
    const id = metadata.component.package.slice(6), root = buildPath(dirname(path));
    if (components.has(id)) throw new Error(`Duplicate component identity: ${id}`);
    components.set(id, metadata.semio);
    for (const row of metadata.semio.playground ?? []) playgrounds.push({ ...row, pluginId: id, cratePath: root,buildCatalog });
  }
  return playgrounds;
}

/** 🌐️ Per-component `build` / `build-<variant>-site` targets that publish CDN trees: each variant runs the distribution build of its
 * playground row declared on the component crate, and `build` is the union of them.
 * @see ../../../💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📜️script.ts */
export function pluginSiteTargetsForCrate(root, allPlaygrounds, stagingProjectRoot, workspaceRoot) {
  const rows = allPlaygrounds.filter((row) => row.cratePath === root);
  if (rows.length === 0) return {};
  if (rows.some(row => !row.distDir) && !stagingProjectRoot) throw Error("Playground site output requires the release producer's staging project");
  const output = row => {
    if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(row.variant)) throw Error("Invalid distribution variant");
    if (row.distDir !== undefined && (typeof row.distDir !== "string" || !row.distDir || /[\\:\u0000-\u001f\u007f]/u.test(row.distDir) || row.distDir.split("/").some(part => !part || part === "." || part === ".."))) throw Error("Invalid distribution output directory");
    return `{workspaceRoot}/.cache/b/p/m/vite/{options.env.SEMIO_BUILD_MODE}/${createHash("sha256").update("os-dev:"+row.variant+":react").digest("hex").slice(0,16)}/out`;
  };
  const targets = {}, distribution = buildPath(relative(workspaceRoot, join(LIBRARY_ROOT, "../../../💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📜️script.ts")));
  for (const row of rows) {
    const site = `build-${row.variant}-site`;
    const build = `build-${row.variant}-react`;
    targets[site] = executionTargetV1("source", {
      cache: false,
      outputs: [output(row)],
      executor: "repo:noop",
      dependsOn: [`@semio-tech/framework-os-dev:${build}`],
      options: {},
    });
  }
  targets.build = executionTargetV1("source", { executor: "repo:noop", cache: false, dependsOn: rows.map((row) => `build-${row.variant}-site`), outputs: rows.map(output) });
  return targets;
}

/** 🧊️ Admits one owner registry object once; its canonical declaration is immutable for the lifetime of that object. */
const admittedRegistries=new WeakMap();
function admittedRegistry(registry){
 if(registry===null||typeof registry!=="object")return createRequire(import.meta.url)("../../🚪️admission/🟦️.ts").readBuildOwnerRegistryV1(registry);
 let declaration=admittedRegistries.get(registry);
 if(!declaration){declaration=createRequire(import.meta.url)("../../🚪️admission/🟦️.ts").readBuildOwnerRegistryV1(registry);admittedRegistries.set(registry,declaration);}
 return declaration;
}

/** 🏞️ Constructs admitted site targets only through their physical canonical release producer. */
export function playgroundSiteTargetsForAdmissionV1(root,playgrounds,workspaceRoot,registry,selectedTargets){
 const rows=playgrounds.filter(row=>row.cratePath===root),names=new Set(["build",...rows.map(row=>"build-"+row.variant+"-site")]);
 if(!rows.length||selectedTargets!==undefined&&!selectedTargets.some(name=>names.has(name)))return {};
 const declaration=admittedRegistry(registry),owner=declaration.owners.find(owner=>owner.name==="@semio-tech/framework-os-dev");
 if(!owner||owner.manifest!==(owner.root==="."?PROJECT_BASENAME:owner.root+"/"+PROJECT_BASENAME))throw Error("Playground site output requires the release producer's staging project");
 const project=JSON.parse(readPhysicalSource(join(workspaceRoot,owner.manifest)).toString("utf8"));
 if(project.name!==owner.name||project.root!==undefined&&project.root!==owner.root)throw Error("Canonical playground release staging producer differs");
 return pluginSiteTargetsForCrate(root,rows,owner.root,workspaceRoot);
}

/** 🎮️ Gives each Cargo-declared playground session an independent generated artifact: a pure function of the committed
 * semantic descriptors of its runtime closure, so no session waits on a component build.
 * @returns {import("../🟨️.d.mts").GeneratedProjectV1["targets"]} */
export function playgroundSessionTargets(configFiles, workspaceRoot,catalogContributions=[]) {
  if(!catalogContributions.length)return {};
  const components = new Map(), playgrounds = new Map();
  for (const file of configFiles) {
    if (!file.endsWith("Cargo.toml") || file.includes("\uFFFD") || file.includes(".🧬semio") || file.startsWith("compose/") || file.startsWith("temp/compose/") || POLICY.generatedDirectories.some(directory => file.split("/").includes(directory))) continue;
    const buildCatalog=catalogBuildContributionForManifestV1(catalogContributions,file);if(!buildCatalog)continue;
    const manifest=readToml(join(workspaceRoot,file)), metadata=manifest.package?.metadata;
    if(!metadata?.component?.package || !["plugin","extension"].includes(metadata.semio?.["component-kind"])) continue;
    const pluginId=metadata.component.package.slice(6), root=buildPath(dirname(file)), projectFile=join(workspaceRoot,root,PROJECT_BASENAME);
    components.set(pluginId,{...metadata.semio,pluginId,ownerRoot:buildPath(join(root,"..","..")),project:existsSync(projectFile)?JSON.parse(readFileSync(projectFile,"utf8")).name:manifest.package.name,dependsOn:[...(metadata.semio.extends?[metadata.semio.extends]:[]),...(metadata.semio["depends-on"]??[])]});
    for(const row of metadata.semio.playground??[]) {
      if(typeof row.variant!=="string" || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(row.variant) || playgrounds.has(row.variant)) throw new Error("Invalid or duplicate playground variant in "+file);
      playgrounds.set(row.variant,{...row,pluginId,buildCatalog});
    }
  }
  return Object.fromEntries([...playgrounds].sort(([a],[b])=>a.localeCompare(b)).map(([variant,row])=>["session-"+variant,executionTargetV1("source",{
    cache:true,
    inputs:["default",{dependentTasksOutputFiles:"**/*",transitive:true},...[...loaded.runtimeComponentClosure([...components.values()],[{id:row.pluginId,appScoped:row.app!==undefined}])].sort().flatMap(id=>["🛂️.descriptor.semio","🔣️.json"].map(name=>`{workspaceRoot}/${components.get(id).ownerRoot}/${name}`))],
    dependsOn:[row.buildCatalog.project+":"+row.buildCatalog.generateTarget],
    outputs:["{workspaceRoot}/.cache/b/p/m/browser/{options.env.SEMIO_BUILD_MODE}/sessions/"+variant],
    options:{command:"bun ./📜️script.ts session "+variant,env:{SEMIO_PLUGIN_CATALOG:join(workspaceRoot,row.buildCatalog.publication.outputRoot)}},
  })]));
}

/** 🎮️ Declares completed runtime prerequisites, cacheable validation and profile-specific browser producers.
 * React activation owns its restorable runtime directory. WGPU activation publishes live extensions,
 * fonts and reload notifications, so it must execute after every cacheable preparation. */
/** 🎯️ Generated Nx targets for every playground preparation, activation, install and serve command.
 * @param {readonly string[]} configFiles
 * @param {string} workspaceRoot
 * @param {string} projectRoot
 * @returns {Record<string, { cache: boolean, continuous?: boolean, dependsOn: string[], outputs: string[], inputs?: unknown[], metadata?: { semio?: { orchestrator?: boolean, execution?: object, runtimeInputAdmission?: object } }, options: { command: string, forwardAllArgs?: boolean } }>}
 */
export async function playgroundPreparationTargets(configFiles, workspaceRoot, projectRoot, contracts = {},catalogContributions=[],context) {
  if(!catalogContributions.length)return {};
  if(!context?.invocation&&!context?.preparationControl)throw Error("Original playground preparation invocation required");
  const advance=phase=>context.preparationControl?context.preparationControl.advance(phase):advanceScriptInvocation(context.invocation,phase,"running");
  await advance("build:playground-components");
  const components = new Map(), playgrounds = [], nativeFiles = new Map(), nativeClosures = new Map();
  const projectOwners = new Map(), nativeHostOwners = new Map(), nativePlans = new Map();
  const componentPrerequisites=async id=>{if(!nativePlans.has(id)){await advance("build:playground-native");nativePlans.set(id,nativePreparation(components.get(id).cratePath,workspaceRoot,contracts,false,nativeFiles,nativeClosures).map(contract=>contract.target));}return nativePlans.get(id);};
  const closurePrerequisites=async(ids,facet)=>{const prerequisites=["@semio-tech/os-plugin-describe-rs:build",...(facet==="browser"?["workspace:deps-wasm-opt"]:[])];for(const id of ids)prerequisites.push(...await componentPrerequisites(id));return [...new Set(prerequisites)].sort();};
  const closureOutputs=(ids,facet="browser",mode="{options.env.SEMIO_BUILD_MODE}")=>[...ids].sort().flatMap(id=>{const row=components.get(id),directory=loaded.componentDeploymentDirectoryV1(row);return[componentModelOutputV1(row.cratePath,mode),...(facet==="browser"&&directory!==undefined?[`{workspaceRoot}/${PLUGIN_MODULES_ROOT}/${directory}`]:[])];});
  const orchestrator = { semio: { orchestrator: true } };
  const projectAt = (root) => {
    const path = join(workspaceRoot, root, PROJECT_BASENAME);
    if (!projectOwners.has(root)) projectOwners.set(root, existsSync(path) ? JSON.parse(readFileSync(path, "utf8")) : undefined);
    return projectOwners.get(root);
  };
  let manifestIndex=0;for (const path of configFiles) {
    if(++manifestIndex%16===0)await advance("build:playground-components");
    if (!path.endsWith("Cargo.toml") || path.includes("\uFFFD") || path.includes(".🧬semio") || path.startsWith("compose/") || path.startsWith("temp/compose/") || POLICY.generatedDirectories.some((directory) => path.split("/").includes(directory))) continue;
    const buildCatalog=catalogBuildContributionForManifestV1(catalogContributions,path);if(!buildCatalog)continue;
    const manifest = readToml(join(workspaceRoot, path)), metadata = manifest.package?.metadata;
    if (!metadata?.component?.package || !["plugin", "extension"].includes(metadata.semio?.["component-kind"])) continue;
    const id = metadata.component.package.slice(6), root = buildPath(dirname(path));
    if (components.has(id)) throw new Error(`Duplicate component identity: ${id}`);
    components.set(id, { ...metadata.semio, cratePath: root, project: projectAt(root)?.name ?? manifest.package.name });
    for (const row of metadata.semio.playground ?? []) playgrounds.push({ ...row, pluginId: id, cratePath: root,buildCatalog });
  }

  const wgpuRoot = buildPath("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript"), wgpuProject = projectAt(wgpuRoot);
  if (!wgpuProject?.name || !["prototype","release"].every(mode=>wgpuProject.targets?.wasm?.configurations?.[mode]?.env?.SEMIO_BUILD_MODE===mode)) throw new Error(`WGPU renderer must name both authored wasm profile producers: ${wgpuRoot}`);
  const flowRoot="🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/📦️packages/🦀️rust", flowProject=projectAt(flowRoot);
  if(!flowProject?.name || !flowProject.targets?.wasm)throw new Error(`Flow must name its authored browser producer: ${flowRoot}`);
  const enginePrerequisites = owners => owners.map(root=>{const project=projectAt(root);if(!project?.name||!project.targets?.wasm)throw Error(`Engine must name its authored producer: ${root}`);return `${project.name}:wasm`;});
  const rendererEngines=enginePrerequisites(frameworkOsEngineSourceOwnersV1("renderer")),distributionEngines=enginePrerequisites(frameworkOsEngineSourceOwnersV1("distribution"));
  const nativeHostView={kind:(path)=>{try{const stat=lstatSync(join(workspaceRoot,path));return stat.isSymbolicLink()?"symlink":stat.isDirectory()?"directory":stat.isFile()?"file":null;}catch(error){if(error.code==="ENOENT")return null;throw error;}},readText:(path)=>readFileSync(join(workspaceRoot,path),"utf8"),admitScript:(path)=>observedSourceAnalysisV1(prepareSourceAnalysesV1().nativeScript,join(workspaceRoot,path))};
  const nativeHostOwner = root => { if (!nativeHostOwners.has(root)) nativeHostOwners.set(root, loaded.nativeHostSourceFactsV1(root, nativeHostView)); return nativeHostOwners.get(root); };
  const result = {};
  for (const playground of playgrounds) {
    await advance("build:playground-variants");
    if (typeof playground.variant !== "string" || playground.variant.length > 256 || !/^[a-z0-9]+(?:-[a-z0-9]+)*$(?![\s\S])/u.test(playground.variant)) throw Error("Invalid playground variant");
    const composition = playground.compositionConfigPath === undefined ? undefined : loaded.playgroundCompositionPathV1(playground.compositionConfigPath), wgpuPort = playground.ports?.wgpu;
    if (!Number.isSafeInteger(wgpuPort) || wgpuPort < 1 || wgpuPort > 65535) throw Error("Invalid declared WGPU listener port");
    const componentRows = [...components].map(([pluginId,row])=>({...row,pluginId,dependsOn:[...(row.extends?[row.extends]:[]),...(row["depends-on"]??[])]}));
    const sources=[...(playground.engines??[]).map(path=>buildPath(relative(workspaceRoot,resolve(workspaceRoot,path,PROJECT_BASENAME)))),...[playground.nativeHost,playground.mcpHost].filter(Boolean).flatMap(host=>[`${host.cratePath}/Cargo.toml`,`${host.cratePath}/${PROJECT_BASENAME}`]),...(playground.devContribution?[playground.devContribution]:[]),...(composition===undefined?[]:[composition])];
    const request={component:playground.pluginId,appScoped:playground.app!==undefined,sources};
    let admission=loaded.runtimeInputAdmissionV1(componentRows,[{id:request.component,appScoped:request.appScoped}],sources,path=>existsSync(join(workspaceRoot,path)));
    if(admission.status==="admitted")for(const id of admission.selected){try{await componentPrerequisites(id);}catch(error){if(error.code!=="NATIVE_INPUT_ADMISSION")throw error;const missing=buildPath(relative(workspaceRoot,error.manifest));sources.push(missing);admission={schemaVersion:1,status:"refused",missing:{kind:"source",value:missing}};break;}}
    if(admission.status==="refused") {
      const command=`bun ${JSON.stringify(join(workspaceRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📜️script.ts"))} runtime-input-check ${Buffer.from(JSON.stringify(request)).toString("base64")}`;
      const target=executionTargetV1("source",{cache:false,outputs:[],dependsOn:[],inputs:["{workspaceRoot}/**/Cargo.toml","{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs","{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/**/*"],metadata:{semio:{runtimeInputAdmission:admission}},options:{command,forwardAllArgs:false}});
      const names=[`runtime-input-admission-${playground.variant}`,`build-${playground.variant}-react-release`];
      for(const profile of ["prototype","release"]){for(const engine of ["react","wgpu"])for(const operation of ["prepare","activate","serve","dev"])names.push(`${operation}-${playground.variant}-${engine}-${profile}`);names.push(`prepare-${playground.variant}-native-${profile}`,`run-${playground.variant}-native-${profile}`,`smoke-${playground.variant}-native-${profile}`);if(playground.mcpHost)for(const transport of ["stdio","http"])names.push(`mcp-${playground.variant}-${transport}-${profile}`);}
      for(const name of names)result[name]=target;
      continue;
    }
    const nativeHost=loaded.admitPlaygroundNativeHostV1(playground.nativeHost,nativeHostOwner);
    const mcpHost=loaded.admitPlaygroundNativeHostV1(playground.mcpHost,nativeHostOwner);
    if(mcpHost)for(const profile of ["prototype","release"])for(const transport of ["stdio","http"])result[`mcp-${playground.variant}-${transport}-${profile}`]={cache:false,continuous:true,outputs:[],dependsOn:[`${mcpHost.project}:${mcpHost.target}`],options:{command:`bun ../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts mcp ${playground.variant} ${profile} ${transport}`,forwardAllArgs:true}};
    // 🏠️ Boot closure: host sessions must serve before the full catalog materializes. `appScoped: true`
    // keeps the host's own depends-on/consumes without the host-fanout that otherwise selects every crate.
    const bootSelected = loaded.runtimeComponentClosure(componentRows, [{ id: playground.pluginId, appScoped: true }]);
    const selected = loaded.runtimeComponentClosure(componentRows, [{ id: playground.pluginId, appScoped: playground.app !== undefined }]);
    const bootPrerequisites=await closurePrerequisites(bootSelected,"browser"),selectedPrerequisites=await closurePrerequisites(selected,"browser"),nativePrerequisites=await closurePrerequisites(selected,"model");
    const engines = new Set([...new Set([...(playground.engines ?? []), ...loaded.declaredBrowserSessionEnginesV1(workspaceRoot, playground.devContribution)])].map((path) => {
      const root = buildPath(relative(workspaceRoot, resolve(workspaceRoot, path))), project = projectAt(root);
      if (root.startsWith("../") || !project?.name || !project.targets?.wasm) throw new Error(`Playground engine must name an authored wasm producer: ${path}`);
      return `${project.name}:wasm`;
    }));
    for (const profile of ["prototype", "release"]) {
      const nativeScript = "bun ../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts";
      result[`prepare-${playground.variant}-native-${profile}`] = {
        cache: false,
        metadata: orchestrator,
        outputs: [`{workspaceRoot}/.cache/b/p/m/browser/${profile}/native/${playground.variant}`, ...closureOutputs(selected,"model",profile)],
        dependsOn: [`@semio-tech/plugin-registry:session-${playground.variant}`, ...nativePrerequisites],
        options: { command: `bun ../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📦️modules/📜️script.ts publish ${playground.variant} ${profile}`, forwardAllArgs: false },
      };
      for (const operation of ["run", "smoke"]) result[`${operation}-${playground.variant}-native-${profile}`] = {
        cache: false, continuous: false, outputs: [],
        dependsOn: [`prepare-${playground.variant}-native-${profile}`, `${nativeHost?.project??wgpuProject.name}:${nativeHost?.target??"native-build"}`],
        options: { command: `${nativeScript} run ${playground.variant} ${profile}${operation === "smoke" ? " --smoke" : ""}`, forwardAllArgs: true },
      };
      // ⚡ `serve` must not wait on activate: warm `served` boots from already-staged modules (content-hash
      // freshness inside ServeScript) in seconds. `dev` still depends on boot-only activate so the host is
      // materialized before Vite, then the activation Vite plugin prefetches the rest on demand.
      result[`serve-${playground.variant}-react-${profile}`] = { cache: false, continuous: true, outputs: [], dependsOn: [`@semio-tech/plugin-registry:session-${playground.variant}`], options: { command: `bun ./📜️script.ts serve ${playground.variant} react ${profile}` } };
      result[`dev-${playground.variant}-react-${profile}`] = { cache: false, continuous: true, outputs: [], dependsOn: [`activate-${playground.variant}-react-${profile}`], options: { command: `bun ./📜️script.ts serve ${playground.variant} react ${profile}` } };
      result[`activate-${playground.variant}-react-${profile}`] = {
      cache: true,
      parallelism: false,
      outputs: [`{workspaceRoot}/.cache/b/p/m/browser/${profile}/runtime/react/${playground.variant}`],
      inputs: [{ dependentTasksOutputFiles: "**/*", transitive: true }],
      dependsOn: [`prepare-${playground.variant}-react-${profile}`],
      options: { command: `bun ./📜️script.ts activate ${playground.variant} react ${profile}` },
      };
      result[`prepare-${playground.variant}-react-${profile}`] = {
      cache: false,
      metadata: orchestrator,
      outputs: closureOutputs(bootSelected),
      dependsOn: [`@semio-tech/plugin-registry:session-${playground.variant}`, "@semio-tech/framework-plugin-web:support", "semio-framework-os-infinite:fonts", ...rendererEngines, ...engines, ...bootPrerequisites],
      options: { command: `bun ./📜️script.ts prepare ${playground.variant} react ${profile}` },
      };
      for (const command of ["serve", "dev"]) result[`${command}-${playground.variant}-wgpu-${profile}`] = { cache: false, continuous: true, outputs: [], dependsOn: [`activate-${playground.variant}-wgpu-${profile}`], options: { command: `bun ../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts serve ${playground.variant} ${profile} --port ${wgpuPort}`, ...(composition===undefined?{}:{env:{SEMIO_WGPU_COMPOSITION_PATH:composition}}) } };
      result[`activate-${playground.variant}-wgpu-${profile}`] = {
      cache: false,
      outputs: [`{workspaceRoot}/.cache/b/p/m/browser/${profile}/runtime/wgpu/${playground.variant}`],
      inputs: [{ dependentTasksOutputFiles: "**/*", transitive: true }],
      dependsOn: [`prepare-${playground.variant}-wgpu-${profile}`],
      options: { command: `bun ./📜️script.ts activate ${playground.variant} wgpu ${profile}` },
      };
      result[`prepare-${playground.variant}-wgpu-${profile}`] = {
      cache: false,
      metadata: orchestrator,
      outputs: closureOutputs(bootSelected),
      dependsOn: [`@semio-tech/plugin-registry:session-${playground.variant}`, "@semio-tech/framework-plugin-web:support", "semio-framework-os-infinite:fonts", `${wgpuProject.name}:wasm`, `${wgpuProject.name}:generate-browser-boot`, `${wgpuProject.name}:generate-frame-worker`, `${wgpuProject.name}:generate-renderer-boot`, ...bootPrerequisites],
      options: { command: `bun ./📜️script.ts prepare ${playground.variant} wgpu ${profile}` },
      };
    }
    const name = `build-${playground.variant}-react-release`;
    const buildIdentity=createHash("sha256").update("os-dev:"+playground.variant+":react").digest("hex").slice(0,16);
    const buildRoot = `{workspaceRoot}/.cache/b/p/m/vite`;
    const buildOutput = `${buildRoot}/release/${buildIdentity}/out`;
    result[name] = {
      cache: false,
      metadata: orchestrator,
      outputs: [buildOutput],
      // 🌐️ `S_HUB_URL` and `S_DATA_DIR` are baked into the bundle at Vite `define` time
      // (`🏗️builder/🌐️vite/🟦️.ts:237`, `import.meta.env.VITE_S_HUB_URL`), so two bundles that differ
      // only by which hub they sign in against are DIFFERENT artifacts. Without these env inputs the
      // cache key ignores them and a cached bundle silently answers for the wrong hub.
      inputs: ["production", "^production", { dependentTasksOutputFiles: "**/*", transitive: true }, { env: "S_HUB_URL" }, { env: "S_DATA_DIR" }, { runtime: `bun ${JSON.stringify(buildPath(relative(workspaceRoot, resolve(workspaceRoot, projectRoot, "../../🚚️distribution/📜️script.ts"))))} inputs` }],
      dependsOn: [`@semio-tech/plugin-registry:session-${playground.variant}`, "@semio-tech/framework-plugin-web:support", "semio-framework-os-infinite:fonts", "semio-framework-canvas:stage-runtime-assets", `${flowProject.name}:wasm`, ...distributionEngines, ...engines, ...selectedPrerequisites, "@semio-tech/assets:build"],
      options: { command: `bun ../../🚚️distribution/📜️script.ts build ${playground.variant} react release`, forwardAllArgs: true },
    };
    result[`build-${playground.variant}-react`] = {
      ...result[name], outputs: [`${buildRoot}/{options.env.SEMIO_BUILD_MODE}/${buildIdentity}/out`],
      configurations: {
        prototype: { command: `bun ../../🚚️distribution/📜️script.ts build ${playground.variant} react prototype` },
        release: { command: `bun ../../🚚️distribution/📜️script.ts build ${playground.variant} react release` },
      },
    };
  }
  for (const playground of playgrounds) for (const renderer of ["react", "wgpu"]) {
    const prototype = result[`prepare-${playground.variant}-${renderer}-prototype`];
    const release = result[`prepare-${playground.variant}-${renderer}-release`];
    if (!prototype || !release) continue;
    result[`prepare-${playground.variant}-${renderer}`] = {
      ...prototype,
      dependsOn: prototype.dependsOn.map(target => target.replace(/:wasm-release$/, ":wasm")),
      configurations: {
        prototype: { command: prototype.options.command },
        release: { command: release.options.command },
      },
    };
  }
  const pluginSiteReleases = playgrounds.filter((row) => row.distDir !== undefined).map((row) => `build-${row.variant}-react`);
  if (pluginSiteReleases.length > 0) {
    result["build-all-playground-cdn-sites"] = {
      cache: false,
      metadata: orchestrator,
      dependsOn: pluginSiteReleases,
      outputs: [],
      options: { command: "bun ../../🚚️distribution/📜️script.ts complete", forwardAllArgs: false },
    };
  }
  return Object.fromEntries(Object.entries(result).map(([name,target])=>{
    const owner=playgrounds.filter(row=>name.includes("-"+row.variant+"-")||name.endsWith("-"+row.variant)).sort((a,b)=>b.variant.length-a.variant.length)[0]?.buildCatalog;
    if(owner)target={...target,options:{...target.options,env:{...target.options?.env,SEMIO_PLUGIN_CATALOG:join(workspaceRoot,owner.publication.outputRoot)}}};
    return [name,target.metadata?.semio?.execution ? target : executionTargetV1("orchestration",target)];
  }));
}
