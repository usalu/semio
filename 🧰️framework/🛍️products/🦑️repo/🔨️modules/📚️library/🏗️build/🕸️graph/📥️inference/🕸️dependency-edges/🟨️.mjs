/** 🕸️ Derives project dependency edges and publishes the complete module facts of one graph. */
const javascriptManagerV1 = createRequire(import.meta.url)("../../../../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🟨️.cjs").selectedManagerV1("javascript");
import { checkResourceLeaseControl } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { checkScriptInvocation, requireIssuedResourceControlInvocationV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { createRequire, isBuiltin } from "node:module";
import { dirname, join, relative, resolve } from "node:path";
import { observedSourceAnalysisAsyncV1, observedFs } from "../../../../🏗️build/👁️observation/🟨️.mjs";
import { repositoryBuildGraphDataDirectoryV1 } from "../../../📥️inputs/💾️storage/🟨️.mjs";
import { createRelativeImportResolverV1 } from "../../🚪️admission/🟦️.ts";
const { existsSync, readFileSync } = observedFs;

/** 🏷️ Names this component as its roster entry does. */
export const component = "dependency-edges";
let workspaceDataDirectory, buildPath, owned, loaded, yieldInferenceV1, prepareSourceAnalysesV1, libraryBootstrap, readPhysicalSource, readToml, cargoTargets, createScriptInputCache, readBunLockGraph, readTaxonomyProjectionV1, goManifest, goDependencies, nativeWorkspaceV1, nativeDependencies, SCRIPT_BASENAME, revisionIdentity;

/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */
export function wire(registry) {
  ({ workspaceDataDirectory, buildPath, owned, readPhysicalSource, SCRIPT_BASENAME } = registry.coordinates);
  ({ loaded, libraryBootstrap } = registry.bootstrap);
  ({ yieldInferenceV1, createScriptInputCache } = registry.scriptInputs);
  ({ prepareSourceAnalysesV1, readTaxonomyProjectionV1 } = registry.sourceAnalyses);
  ({ readToml, goManifest, goDependencies, nativeWorkspaceV1, nativeDependencies } = registry.nativeManifests);
  ({ cargoTargets } = registry.componentTargets);
  ({ readBunLockGraph } = registry.bunLock);
  ({ revisionIdentity } = registry);
}

const moduleGraphPublications=new Map();
const moduleGraphPublicationKeyV1=root=>JSON.stringify([resolve(root),repositoryBuildGraphDataDirectoryV1(root,"inference")]);

/** 🧭️ Picks an indexed dependency source file through the original portable file map. */
export function buildTrackedSourceFile(context, projectName, preferred, basenameHint) {
  const { workspaceRoot } = context;
  const exists = (rel) => {
    try { return existsSync(join(workspaceRoot, rel)); } catch { return false; }
  };
  if (exists(preferred)) return preferred;
  const files = context.fileMap?.projectFileMap?.[projectName];
  const hint = basenameHint ?? preferred.split("/").pop();
  if (files?.length) {
    const exact = files.find((entry) => entry.file === preferred);
    if (exact && exists(exact.file)) return exact.file;
    const suffix = `/${hint}`;
    const matches = files.filter((entry) => (entry.file === hint || entry.file.endsWith(suffix)) && exists(entry.file));
    if (matches.length === 1) return matches[0].file;
  }
  return preferred;
}

/** 📚️ Admits one packed semantic record directory only when every physical temporary and pack path fits. */
export function moduleSourceRecordCacheRootV1(workspaceRoot){
 const root=resolve(workspaceDataDirectory(workspaceRoot),"mr"),path=join(root,"0".repeat(64)+".json");
 return path.length<=256&&[...path].length<=256?root:undefined;
}

/** 🏛️ Snapshots current project roots once, in deterministic ownership precedence. */
function importProjectOwners(projects) {
  return Object.entries(projects).flatMap(([name, project]) => [project.root,...project.metadata?.nativeRoot?[project.metadata.nativeRoot]:[]].map(root=>({name,root:buildPath(root)}))).sort((a,b)=>b.root.length-a.root.length||a.name.localeCompare(b.name));
}

/** 🧭️ Uses bare and explicit core loader identity so bundled packages retain their package identity. */
function coreModuleImport(specifier) {
  try {
    const loader = createRequire(import.meta.url);
    if (specifier.startsWith("node:")) return loader.resolve(specifier) === specifier;
    if (!isBuiltin(specifier)) return false;
    const core = "node:" + specifier;
    return loader.resolve(specifier) === specifier && loader.resolve(core) === core;
  } catch { return false; }
}

/** 🔗️ Resolves immutable runtime import strings against current ownership, package names, and lock scope. */
function importTargetsFromFacts(imports, file, workspaceRoot, resolveOwner, byPackage, locked) {
  const targets = new Set();
  for (const specifier of imports) {
    if (specifier.startsWith(".")) {
      const path = buildPath(relative(workspaceRoot, resolve(dirname(join(workspaceRoot, file)), specifier)));
      const target = resolveOwner(path);
      if (target) targets.add(target);
      continue;
    }
    const packageName = specifier.startsWith("@") ? specifier.split("/").slice(0, 2).join("/") : specifier.split("/")[0];
    if (coreModuleImport(packageName)) continue;
    const workspaceTarget = byPackage.get(packageName);
    if (workspaceTarget) { targets.add(workspaceTarget); continue; }
    const key = locked?.resolveImport?.(file, packageName);
    if (key && locked?.externalNodes?.[`npm:${key}`]) targets.add(`npm:${key}`);
  }
  return [...targets].sort();
}

/** 🔍️ Parses one module through the canonical owned string interface and current project authority. */
export function importTargetsFromSource(text, file, workspaceRoot, projects, byPackage, locked) {
  return importTargetsFromFacts(loaded.moduleSourceImports(file, text), buildPath(file), workspaceRoot, createRelativeImportResolverV1(importProjectOwners(projects)), byPackage, locked);
}

/** 📦️ How many owners the import edges of a graph are collected for at a time: the source facts of one slice are resolved into edges and dropped before the next slice is read, whatever the size of the workspace (the graph passes the chunk of the claim registry). */
export const EDGE_CHUNK_OWNERS = 128;

/** ⚡️ Reuses bounded source facts while resolving every full or incremental file against current authority, `context.edgeChunkOwners` owners at a time; the complete module facts are published only for `context.moduleGraphProjects` (every project when it is absent), so an owner that only gets its edges again costs no retained facts. */
export async function collectImportEdges(workspaceRoot, projectFiles, projects, byPackage, locked, add, publication, scheduledFiles, context={}) {
  const resolveOwner=createRelativeImportResolverV1(importProjectOwners(projects)),retain=context.moduleGraphProjects,names=Object.keys(projectFiles),size=Math.max(1,context.edgeChunkOwners??EDGE_CHUNK_OWNERS);
  let step=0;
  for(let from=0;from<names.length;from+=size){
    const jobs=[];
    for(const name of names.slice(from,from+size))for(const file of projectFiles[name])if(/\.[cm]?[jt]sx?$/.test(file.file))jobs.push({name,file});
    let cursor=0;
    await Promise.all(Array.from({length:Math.min(32,jobs.length)},async()=>{
      for(;;){
        const index=cursor++;if(index>=jobs.length)return;await yieldInferenceV1(context,"build:modules",++step);
        const{name,file}=jobs[index],sourceFile=buildPath(file.file);
        let fact;try{fact=await observedSourceAnalysisAsyncV1(prepareSourceAnalysesV1().module,join(workspaceRoot,sourceFile));}catch(error){if(error.code==="ENOENT")continue;throw error;}
        const{imports,productionImports}=fact;
        if(publication&&(!retain||retain.has(name)))(publication[name]??={version:1,files:[]}).files.push({file:sourceFile,imports,productionImports});
        if(name!=="workspace"&&(!scheduledFiles||scheduledFiles.has(sourceFile)))for(const target of importTargetsFromFacts(context.completeSourceImports?imports:productionImports,sourceFile,workspaceRoot,resolveOwner,byPackage,locked))add(name,target,sourceFile);
      }
    }));
  }
  if(publication)for(const value of Object.values(publication))value.files.sort((a,b)=>a.file.localeCompare(b.file));
}

/** 📇️ Publishes complete captured module facts after the original dependency phase. */
export function createMetadataImplementation(graph,_options,context){
  const publication=moduleGraphPublications.get(moduleGraphPublicationKeyV1(context.workspaceRoot));
  const owners=Object.entries(graph.nodes).filter(([,node])=>node.type!=="npm").map(([name,node])=>[name,buildPath(node.data.root)]).sort(([a],[b])=>a.localeCompare(b));
  if(!publication||publication.parser!==loaded.commandInputHash||JSON.stringify(publication.owners)!==JSON.stringify(owners))throw Error("Original full module graph producer required");
  return Object.fromEntries((publication.retained?owners.filter(([name])=>publication.retained.has(name)):owners).map(([name])=>[name,{metadata:{semio:{...graph.nodes[name].data.metadata?.semio,moduleGraph:structuredClone(publication.projects[name]??{version:1,files:[]})}}}]));
}

/**
 * 🧭️ Selects changed module files from the original file map, or the complete owned file roster.
 * @param {{ fileMap?: { projectFileMap?: Record<string, { file: string, hash?: string }[]> }, filesToProcess?: { projectFileMap?: Record<string, { file: string, hash?: string }[]> } }} context
 */
export function projectFilesToProcess(context) {
  return context.filesToProcess?.projectFileMap ?? context.fileMap?.projectFileMap ?? {};
}

/** 🏭️ Validates complete authored producers independently of the selected executable projection. */
export async function validateGeneratorOutputProducersV1(contracts,projects,workspaceRoot,selection,registry,context){
 const check=()=>{if(context.workspaceRoot!==workspaceRoot)throw Error("Actual generator inference workspace context required");if(context.invocation){requireIssuedResourceControlInvocationV1(context.phaseControl,context.invocation);checkScriptInvocation(context.invocation);checkResourceLeaseControl(context.phaseControl);}};check();
 await libraryBootstrap;check();const admission=createRequire(import.meta.url)("../../🚪️admission/🟦️.ts"),selected=admission.readBuildGraphSelectionV1(selection).kind!=="all",owners=selected?admission.readBuildOwnerRegistryV1(registry):undefined,byRoot=new Map(Object.entries(projects).flatMap(([name,project])=>[[resolve(workspaceRoot,project.root),name],...(project.metadata?.nativeRoot?[[resolve(workspaceRoot,project.metadata.nativeRoot),name]]:[])])),authored=new Map();let steps=0;
 for(const contract of Object.values(contracts)){
  const participants=[contract.ownerPath,...contract.outputRoots.flatMap(output=>output.producer?[output.producer.ownerPath]:[])];if(!participants.some(root=>typeof root==="string"&&byRoot.has(resolve(workspaceRoot,root))))continue;
  for(const output of contract.outputRoots){if(!output.producer)continue;await yieldInferenceV1(context,"build:generator-producers",++steps);const producer=output.producer,separator=producer.target.lastIndexOf(":"),name=producer.target.slice(0,separator),target=producer.target.slice(separator+1);let valid;
   if(selected){
    const owner=owners.owners.find(owner=>owner.name===name&&(owner.root===producer.ownerPath||owner.nativeRoots?.includes(producer.ownerPath)));if(!owner)throw Error(`Generator output has no Build producer: ${output.path}`);
    let project=authored.get(name);if(!project){const path=join(workspaceRoot,owner.manifest),bytes=readPhysicalSource(path),cargo=owner.manifest.endsWith("Cargo.toml"),declared=cargo?readToml(path):JSON.parse(bytes.toString("utf8"));project=cargo?{name:declared.package?.name,root:owner.root,targets:await cargoTargets(owner.root,workspaceRoot,undefined,createScriptInputCache(context))}:declared;check();if(project.name!==owner.name||project.root!==undefined&&project.root!==owner.root)throw Error(`Generator output has no Build producer: ${output.path}`);authored.set(name,project);}
    valid=Boolean(project.targets?.[target]);
   }else valid=byRoot.get(resolve(workspaceRoot,producer.ownerPath))===name&&Boolean(projects[name]?.targets?.[target]);
   if(!valid)throw Error(`Generator output has no Build producer: ${output.path}`);
  }
 }
 check();
}

/** 🏛️ What the edges of every project of a graph are resolved against: the lock graph, the owner roots, the package names and the Go modules of all projects, read once. */
async function edgeAuthorityV1(options, context) {
  const { workspaceRoot, projects } = context;
  const locked = options?.analyzeLockfile && existsSync(join(workspaceRoot, javascriptManagerV1.primaryLockName)) ? readBunLockGraph(workspaceRoot) : undefined;
  const byRoot = new Map(Object.entries(projects).flatMap(([name, project]) => [[resolve(workspaceRoot, project.root), name],...project.metadata?.nativeRoot?[[resolve(workspaceRoot,project.metadata.nativeRoot),name]]:[]]));
  const byPackage = new Map(), manifests = new Map(), goModules = new Map(), goManifests = new Map();
  const authorityPath = join(workspaceRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
  const generators = existsSync(authorityPath) ? readTaxonomyProjectionV1(authorityPath).generatorContracts : {};
  await validateGeneratorOutputProducersV1(generators,projects,workspaceRoot,context.selection??{kind:"all"},context.registry,context);
  for (const [name, project] of Object.entries(projects)) {
    const go = goManifest(project.root, workspaceRoot);
    if (go) {
      const manifest = goDependencies(readFileSync(go, "utf8"));
      goManifests.set(name, { path: go, ...manifest });
      goModules.set(manifest.module, name);
    }
    const packageFile = join(workspaceRoot, project.root, "package.json");
    if (existsSync(packageFile)) {
      const manifest = JSON.parse(readFileSync(packageFile, "utf8"));
      manifests.set(name, manifest);
      if (manifest.name) byPackage.set(manifest.name, name);
    }
  }
  return { workspaceRoot, projects, locked, byRoot, byPackage, manifests, goModules, goManifests };
}

/** ➕️ Adds one edge to `edges` when its tracked source file exists. */
function edgeAdderV1(context, edges) {
  const { workspaceRoot } = context;
  return (source, target, sourceFile, basenameHint) => {
    if (!target || source === target) return;
    const tracked = buildTrackedSourceFile(context, source, sourceFile, basenameHint);
    if (!existsSync(join(workspaceRoot, tracked))) return;
    edges.set(`${source}\0${target}\0${tracked}`, { source, target, sourceFile: tracked, type: "static" });
  };
}

/** 📜️ The edges one project declares in its manifests (Cargo.toml, package.json, go.mod) among its `files`. */
function manifestEdgesV1(authority, add, name, files) {
  const { workspaceRoot, projects, locked, byRoot, byPackage, manifests, goModules, goManifests } = authority, project = projects[name];
  if (!project || name === "workspace") return;
  const root = resolve(workspaceRoot, project.root);
  for (const file of files) {
    if (file.file.endsWith("Cargo.toml")) {
      const cargo = join(workspaceRoot, file.file);
      if (!existsSync(cargo)) continue;
      const manifest=readToml(cargo),directory=dirname(cargo),workspace=nativeWorkspaceV1(directory,workspaceRoot,manifest);
      for (const dependency of nativeDependencies(manifest, workspace.declaration)) {
        if(dependency.kind === "dev-dependencies")continue;
        add(name, byRoot.get(resolve(dependency.workspace ? workspace.root : directory, dependency.path)), file.file);
      }
    }
    if (file.file.endsWith("package.json")) {
      const manifest = manifests.get(name) ?? (existsSync(join(workspaceRoot, file.file)) ? JSON.parse(readFileSync(join(workspaceRoot, file.file), "utf8")) : undefined);
      if (!manifest) continue;
      for (const dependency of Object.keys({ ...manifest.dependencies, ...manifest.peerDependencies, ...manifest.optionalDependencies })) {
        const key = locked?.resolve(locked.workspacePackages.has(manifest.name) ? manifest.name : "", dependency);
        add(name, byPackage.get(dependency) ?? (locked?.workspacePackages.has(key) ? byRoot.get(resolve(workspaceRoot, locked.workspacePackages.get(key))) : key && locked.externalNodes[`npm:${key}`] ? `npm:${key}` : undefined), file.file);
      }
    }
  }
  const go = goManifests.get(name);
  if (go) {
    const provenance = buildPath(relative(workspaceRoot, go.path));
    if (files.some((file) => file.file === provenance || file.file.endsWith("go.mod"))) {
      const sourceFile = owned(provenance, project.root) ? provenance : buildPath(relative(workspaceRoot, join(root, SCRIPT_BASENAME)));
      for (const dependency of go.requires) add(name, goModules.get(dependency), sourceFile);
      for (const replacement of go.replacements) add(name, goModules.get(replacement.module) ?? byRoot.get(resolve(dirname(go.path), replacement.path)), sourceFile);
    }
  }
}

/** 🕸️ Native manifests and package imports contribute edges without spawning a build or installer. */
export async function createDependenciesImplementation(_options, context) {
  const { workspaceRoot, projects } = context, authority = await edgeAuthorityV1(_options, context), { locked, byPackage } = authority, edges = new Map(), add = edgeAdderV1(context, edges);
  const projectFiles = projectFilesToProcess(context);
  for (const [name, files] of Object.entries(projectFiles)) manifestEdgesV1(authority, add, name, files);
  const publication={},fullFiles=context.fileMap?.projectFileMap??projectFiles;
  await collectImportEdges(workspaceRoot,fullFiles,projects,byPackage,locked,add,publication,new Set(Object.values(projectFiles).flat().map(file=>buildPath(file.file))),context);
  moduleGraphPublications.set(moduleGraphPublicationKeyV1(workspaceRoot),{parser:loaded.commandInputHash,owners:Object.entries(projects).map(([name,project])=>[name,buildPath(project.root)]).sort(([a],[b])=>a.localeCompare(b)),projects:publication,retained:context.moduleGraphProjects});
  return [...edges.values(), ...(locked?.dependencies ?? [])];
}

/** 🌊️ The edges of an owner that only gets its edges again, one owner at a time and without reading a source: the module facts its published fragment holds are resolved against the owners, packages and lock of the current graph, its manifests are read as for every owner. Everything the owner consumed is in its own facts already (a changed source makes it stale and infers it again), so the answer equals the one the full edge discovery gives. Answers `edgesOf(name, node)`: the edges whose source is `name`, the lock dependencies of the package included. */
export async function createStreamingEdgesImplementation(options, context) {
  const authority = await edgeAuthorityV1(options, context), { projects, locked } = authority, resolveOwner = createRelativeImportResolverV1(importProjectOwners(projects)), lockedBySource = new Map();
  for (const edge of locked?.dependencies ?? []) { const list = lockedBySource.get(edge.source); if (list) list.push(edge); else lockedBySource.set(edge.source, [edge]); }
  return {
    edgesOf(name, node) {
      const edges = new Map(), add = edgeAdderV1(context, edges);
      manifestEdgesV1(authority, add, name, context.fileMap?.projectFileMap?.[name] ?? []);
      if (name !== "workspace" && projects[name]) for (const fact of node?.metadata?.semio?.moduleGraph?.files ?? []) for (const target of importTargetsFromFacts(context.completeSourceImports ? fact.imports : fact.productionImports, fact.file, context.workspaceRoot, resolveOwner, authority.byPackage, locked)) add(name, target, fact.file);
      return [...edges.values(), ...(lockedBySource.get(name) ?? [])];
    },
  };
}
