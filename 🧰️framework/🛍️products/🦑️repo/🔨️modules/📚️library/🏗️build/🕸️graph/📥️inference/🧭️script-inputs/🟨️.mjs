import {buildInputSourceRoleV1,readBuildRuntimeSourceEntriesV1} from "../../🚪️admission/🟦️.ts";
/** 🧭️ Tracks the literal import closure of command scripts and the packages they consume. */
import { advanceScriptInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { bindBuildCommandV1 } from "../../../../🏗️build/📤️arguments/🟨️.mjs";
import { checkResourceLeaseControl } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { checkScriptInvocation, requireIssuedResourceControlInvocationV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { createRequire, isBuiltin } from "node:module";
import { createHash } from "node:crypto";
import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { memoizedObservedAsyncComputationV1, memoizedObservedComputationV1, observedSourceAnalysisV1, observedRequireResolve, observedTypeScriptHost, observedFs } from "../../../../🏗️build/👁️observation/🟨️.mjs";
import { readCargoExecutionContractV1, selectCargoExecutionOperationV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️execution/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
const { existsSync, statSync } = observedFs;

/** 🏷️ Names this component as its roster entry does. */
export const component = "script-inputs";
let buildPath, readPhysicalSource, requirePhysicalSourceV1, prepareSourceAnalysesV1, LIBRARY_ROOT, POLICY, readToml, readBunLockGraph, SCRIPT_PROVIDER_LOADER_MODULE, SCRIPT_PROVIDER_DESIGNATION, SCRIPT_PROVIDER_DESIGNATION_SCHEMA;

/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */
export function wire(registry) {
  ({ buildPath, readPhysicalSource, requirePhysicalSourceV1, LIBRARY_ROOT, POLICY, SCRIPT_PROVIDER_LOADER_MODULE, SCRIPT_PROVIDER_DESIGNATION, SCRIPT_PROVIDER_DESIGNATION_SCHEMA } = registry.coordinates);
  ({ prepareSourceAnalysesV1 } = registry.sourceAnalyses);
  ({ readToml } = registry.nativeManifests);
  ({ readBunLockGraph } = registry.bunLock);
}

/** 🗂️ Shares source reads and command closures only within one graph construction. */
export function createScriptInputCache(context={}) {
  if(context.invocation){requireIssuedResourceControlInvocationV1(context.phaseControl,context.invocation);checkResourceLeaseControl(context.phaseControl);}
  return { files: new Map(), closures: new Map(), commandDependencies: new Map(), modules:new Set(), resources:new Set(),generatedImports:new Map(),declared:new Map(),paths:new Map(),compiledNames:new WeakMap(),moduleSets:new WeakMap(),context,providerDesignation:context.scriptProviderDesignation };
}

/** 🔌️ Names the declared provider modules a script loads at run time, which no literal import reaches: the designation, its schema and every designated module. */
export function scriptProviderImportsV1(cache) {
  const designation = cache.providerDesignation ?? join(LIBRARY_ROOT, SCRIPT_PROVIDER_DESIGNATION), schema = join(LIBRARY_ROOT, SCRIPT_PROVIDER_DESIGNATION_SCHEMA);
  const declared = JSON.parse(readPhysicalSource(designation).toString("utf8"));
  if (validateJsonSchemaSubset(JSON.parse(readPhysicalSource(schema).toString("utf8")), declared).length) throw Error("Complete repository script provider declaration required");
  return [{ path: designation, source: false }, { path: schema, source: false }, ...Object.values(declared.providers).map(module => ({ path: resolve(dirname(designation), module), source: true }))];
}

/** 🧿️ Keeps a logical import's interpretation separate from its physical file identity. */
export function buildImportResourceV1(containing,specifier){
 if(!specifier.startsWith("."))return {specifier,suffix:"",source:true};
 const url=new URL(specifier,pathToFileURL(containing)),query=url.searchParams;
 return {specifier:fileURLToPath(url),suffix:url.search+url.hash,source:!query.has("raw")&&(!query.has("url")||query.has("worker")||query.has("sharedworker"))};
}

/** 📦️ Resolves a package specifier to the workspace module it names, so imports through `@scope/package` subpaths join a command closure like relative ones; installed registry packages stay covered by their locked external dependency. */
function workspacePackageModuleV1(containing, specifier, workspaceRoot) {
  if (specifier.startsWith(".") || isAbsolute(specifier) || isBuiltin(specifier) || /^[a-z][a-z0-9+.-]*:/u.test(specifier)) return undefined;
  let resolved;
  try { resolved = observedRequireResolve(containing, specifier); }
  catch (error) { if (error?.code === "MODULE_NOT_FOUND" || error?.code === "ERR_PACKAGE_PATH_NOT_EXPORTED") return undefined; throw error; }
  if (!isAbsolute(resolved)) return undefined;
  const local = relative(workspaceRoot, resolved);
  return local === "" || local.startsWith("..") || isAbsolute(local) || local.split(sep).includes("node_modules") ? undefined : resolved;
}

/** 🧪️ Resolves a literal module path whose task role or explicit runtime contract admits its imports. */
function declaredModuleInputV1(input, workspaceRoot, projectRoot) {
  if (typeof input !== "string" || input.startsWith("!") || /[*?{}[\]]/u.test(input.replace(/^\{(?:projectRoot|workspaceRoot)\}/u, "")) || !/\.[cm]?[jt]sx?$/u.test(input) || !/^\{(?:projectRoot|workspaceRoot)\}\//u.test(input)) return undefined;
  const path = resolve(workspaceRoot, input.replace("{projectRoot}", projectRoot).replace("{workspaceRoot}", "."));
  return !relative(workspaceRoot, path).startsWith("..") && existsSync(path) ? path : undefined;
}

/** 🔗️ Resolves a literal `new URL("…", import.meta.url)` reference to the workspace file it names, which is how a script points at the test files it runs and the data it reads. */
function workspaceUrlTargetV1(containing, literal, workspaceRoot) {
  let url;
  try { url = new URL(literal, pathToFileURL(containing)); } catch { return undefined; }
  if (url.protocol !== "file:") return undefined;
  const path = fileURLToPath(url), local = relative(workspaceRoot, path);
  if (local === "" || local.startsWith("..") || isAbsolute(local) || local.split(sep).includes("node_modules") || !existsSync(path) || !statSync(path).isFile()) return undefined;
  return { path, source: /\.[cm]?[jt]sx?$/u.test(path) };
}

/** 🔑️ Names a memo entry by the digest of its ordered parts, so a long path list is hashed once and never retained as a key string. */
function memoDigestV1(...parts) {
  const hash = createHash("sha256");
  for (const part of parts) {
    if (Array.isArray(part)) { hash.update("["); for (const item of part) hash.update(typeof item === "string" ? item : "\u0001").update("\0"); hash.update("]"); }
    else hash.update(String(part)).update("\0");
  }
  return hash.digest("hex");
}

/** 📛️ The workspace-relative name of one closure file, shared by every closure of one graph construction instead of re-created for each. */
function closurePathNameV1(cache, workspaceRoot, file) {
  let names = cache.paths.get(workspaceRoot);
  if (!names) cache.paths.set(workspaceRoot, names = new Map());
  let name = names.get(file);
  if (name === undefined) names.set(file, name = `{workspaceRoot}/${buildPath(relative(workspaceRoot, file))}`);
  return name;
}

/** 🧮️ Resolves the literal module inputs of one target list through one observed memo per distinct list: the command inputs that every generated Cargo target repeats verbatim are probed once, and every receiving project still replays the same existence facts. */
/** 🧮️ True when a target input names a module the entry closure already compiled: adding it as a closure entry changes nothing, and its existence is already implied by the file fact the closure read. */
function coveredModuleInputV1(input, projectRoot, covered) {
  if (typeof input !== "string") return false;
  if (input.startsWith("{workspaceRoot}/")) return covered.has(input);
  if (input.startsWith("{projectRoot}/")) return covered.has(projectRoot === "." ? `{workspaceRoot}/${input.slice(14)}` : `{workspaceRoot}/${projectRoot}/${input.slice(14)}`);
  return false;
}

function declaredModuleInputsV1(inputs, workspaceRoot, projectRoot, cache, covered) {
  if (!Array.isArray(inputs) || !inputs.length) return [];
  const pending = covered ? inputs.filter(input => !coveredModuleInputV1(input, projectRoot, covered)) : inputs;
  if (!pending.length) return [];
  return memoizedObservedComputationV1(cache.declared, memoDigestV1(workspaceRoot, projectRoot, pending), () => pending.flatMap(input => { const declared = declaredModuleInputV1(input, workspaceRoot, projectRoot); return declared ? [declared] : []; }));
}

/** 🧩️ Matches a module-source file name, the only kind of glob a target may declare as closure entries. */
const DECLARED_MODULE_SUFFIX_V1 = /\.[cm]?[jt]sx?$/u;
/** 🧩️ Matches a literal TypeScript project file a target declares so the compiler's own program joins its closure. */
const DECLARED_TSCONFIG_V1 = /^\{(?:projectRoot|workspaceRoot)\}\/(?:[^*?{}[\]]*\/)?tsconfig(?:\.[^/*?{}[\]]+)?\.json$/u;
/** 🧩️ Directories a declared glob never descends into. */
const DECLARED_SKIPPED_DIRECTORIES_V1 = new Set(["node_modules", ".git", "target"]);

/** 🧭️ Resolves a `{projectRoot}` or `{workspaceRoot}` input to a normalized workspace-relative pattern, or `undefined` when it names nothing inside the workspace. */
export function declaredInputPatternV1(input, projectRoot) {
  if (typeof input !== "string" || !/^\{(?:projectRoot|workspaceRoot)\}\//u.test(input)) return undefined;
  const joined = input.startsWith("{projectRoot}/") ? `${projectRoot === "." ? "" : `${projectRoot}/`}${input.slice(14)}` : input.slice(16);
  const parts = [];
  for (const part of joined.split("/")) {
    if (part === "" || part === ".") continue;
    if (part === "..") { if (!parts.length) return undefined; parts.pop(); } else parts.push(part);
  }
  return parts.join("/");
}

/** 🌿️ Expands the module-file globs a target declares (a test directory, for example) into the existing files they match; `!` inputs exclude, `node_modules` is never entered and every directory listing goes through the supplied observed reader.
 * @param {readonly unknown[]} inputs @param {string} workspaceRoot @param {string} projectRoot
 * @param {{readdir:(directory:string)=>readonly {name:string,directory:boolean,file:boolean}[]|undefined,Glob:new(pattern:string)=>{match:(path:string)=>boolean},join:(...parts:string[])=>string}} environment */
export function declaredModuleGlobsV1(inputs, workspaceRoot, projectRoot, environment) {
  const { readdir, Glob, join } = environment;
  const patterns = inputs.flatMap(input => { const pattern = declaredInputPatternV1(input, projectRoot); return pattern !== undefined && /[*?{}[\]]/u.test(pattern) && DECLARED_MODULE_SUFFIX_V1.test(pattern) ? [pattern] : []; });
  const excluded = inputs.flatMap(input => { if (typeof input !== "string" || !input.startsWith("!")) return []; const pattern = declaredInputPatternV1(input.slice(1), projectRoot); return pattern === undefined ? [] : [new Glob(pattern)]; });
  const found = new Set();
  for (const pattern of patterns) {
    const glob = new Glob(pattern), literal = [];
    for (const part of pattern.split("/")) { if (/[*?{}[\]]/u.test(part)) break; literal.push(part); }
    const walk = (relative) => {
      for (const entry of readdir(join(workspaceRoot, relative)) ?? []) {
        const path = relative ? `${relative}/${entry.name}` : entry.name;
        if (entry.directory) { if (!DECLARED_SKIPPED_DIRECTORIES_V1.has(entry.name)) walk(path); }
        else if (entry.file && glob.match(path) && !excluded.some(row => row.match(path))) found.add(path);
      }
    };
    walk(literal.join("/"));
  }
  return [...found].sort();
}

/** 🪁️ Runs the compiler's own program over the TypeScript projects a target declares, so the typecheck closure is exactly what `tsc -p` reads: the include set, every resolved import (path mappings, `.d.ts` siblings, type-only imports, JSON), the configuration chain and the ambient type packages.
 * @returns {{files:string[],packages:string[],configs:string[]}} workspace-absolute files, installed package names, and the declared configuration files */
export function declaredTypeScriptProjectsV1(inputs, workspaceRoot, projectRoot, environment) {
  const { compiler, host, readdir, join, dirname, isAbsolute } = environment;
  const configs = [...new Set(inputs.flatMap(input => DECLARED_TSCONFIG_V1.test(input) ? [declaredInputPatternV1(input, projectRoot)] : []))].filter(Boolean).sort();
  if (!configs.length) return { files: [], packages: [], configs: [] };
  if (typeof compiler.matchFiles !== "function") throw Error("TypeScript project closure requires the compiler's file matcher");
  const entries = (directory) => { const rows = readdir(directory) ?? []; return { files: rows.filter(row => row.file).map(row => row.name).sort(), directories: rows.filter(row => row.directory).map(row => row.name).sort() }; };
  const parseHost = { ...host, readDirectory: (path, extensions, excludes, includes, depth) => compiler.matchFiles(path, extensions, excludes, includes, host.useCaseSensitiveFileNames, host.getCurrentDirectory(), depth, entries, host.realpath) };
  const files = new Set(), packages = new Set(), declared = [];
  const root = workspaceRoot.replaceAll("\\", "/");
  const inside = (path) => path.startsWith(`${root}/`) && !path.slice(root.length + 1).startsWith(".cache/");
  for (const relative of configs) {
    const config = join(workspaceRoot, relative), source = compiler.readJsonConfigFile(config, host.readFile);
    const parsed = compiler.parseJsonSourceFileConfigFileContent(source, parseHost, dirname(config), undefined, config);
    const options = { ...parsed.options, noEmit: true, incremental: false, composite: false, tsBuildInfoFile: undefined };
    const compilerHost = Object.assign(compiler.createCompilerHost(options, true), { fileExists: host.fileExists, readFile: host.readFile, directoryExists: host.directoryExists, getDirectories: host.getDirectories, realpath: host.realpath });
    const program = compiler.createProgram({ rootNames: parsed.fileNames, options, projectReferences: parsed.projectReferences, host: compilerHost });
    declared.push(config);
    for (const path of [config, ...(source.extendedSourceFiles ?? [])]) if (isAbsolute(path) && inside(path)) files.add(path);
    for (const file of program.getSourceFiles()) {
      const path = file.fileName, marker = path.lastIndexOf("/node_modules/");
      if (marker >= 0) { const parts = path.slice(marker + 14).split("/"); packages.add(parts[0].startsWith("@") ? `${parts[0]}/${parts[1]}` : parts[0]); }
      else if (inside(path)) files.add(path);
    }
  }
  return { files: [...files].sort(), packages: [...packages].sort(), configs: declared };
}

/** 🔭️ The observed directory reader declared globs and TypeScript projects list through. */
function declaredDirectoryV1(directory) {
  try { return observedFs.readdirSync(directory, { withFileTypes: true }).map(entry => ({ name: entry.name, directory: entry.isDirectory(), file: entry.isFile() })); }
  catch { return undefined; }
}

/** 🌿️ The module files the target's `metadata.semio.closure.modules` globs match, which join the target's command closure as entries. */
function declaredModuleGlobEntriesV1(globs, workspaceRoot, projectRoot, cache) {
  if (!Array.isArray(globs) || !globs.length) return [];
  if (!globalThis.Bun?.Glob) throw Error("Declared module globs require Bun.Glob");
  return memoizedObservedComputationV1(cache.declared, memoDigestV1("module-globs", workspaceRoot, projectRoot, globs), () => declaredModuleGlobsV1(globs, workspaceRoot, projectRoot, { readdir: declaredDirectoryV1, Glob: globalThis.Bun.Glob, join }).map(path => resolve(workspaceRoot, path)));
}

/** 🪁️ Joins the compiler's own program for each TypeScript project in `metadata.semio.closure.typescript` to the closure, records the generator targets whose outputs it reads and names the installed packages it resolved. */
function declaredProjectClosureV1(projects, workspaceRoot, projectRoot, cache, closure) {
  if (!Array.isArray(projects) || !projects.length) return undefined;
  const project = memoizedObservedComputationV1(cache.declared, memoDigestV1("typescript-project", workspaceRoot, projectRoot, projects), () => {
    const compiler = createRequire(import.meta.url)("typescript");
    return declaredTypeScriptProjectsV1(projects, workspaceRoot, projectRoot, { compiler, host: observedTypeScriptHost(compiler.sys), readdir: declaredDirectoryV1, join, dirname, isAbsolute });
  });
  if (!project.files.length) return undefined;
  const compiled=cache.compiledNames.get(closure);if(!compiled)throw Error("Exact compiled command closure required");
  const config = resolve(workspaceRoot, project.configs[0]), names = [], generatedDependencies=new Set();
  for (const file of project.files) {
    const produced = cache.generatedOutput?.(file);
    if (produced) { const rows = cache.generatedImports.get(config) ?? []; if (!rows.some(row => row.target === produced.target && row.output === produced.output)) rows.push(produced); cache.generatedImports.set(config, rows); generatedDependencies.add(produced.target); continue; }
    readPhysicalSource(file);
    names.push(closurePathNameV1(cache, workspaceRoot, file));
  }
  const result=[...new Set([...closure,...names])].sort();cache.compiledNames.set(result,new Set(compiled));
  return { closure:result, packages: project.packages, config: closurePathNameV1(cache, workspaceRoot, config), generatedDependencies:[...generatedDependencies].sort() };
}

/** 🏭️ The generated-output targets that the modules of one closure import: the absolute module set is derived once per distinct closure instead of once per target. */
function generatedTargetsV1(cache, closure, workspaceRoot) {
  let modules = cache.moduleSets.get(closure);
  if (!modules) { const compiled=cache.compiledNames.get(closure);if(!compiled)throw Error("Exact compiled command closure required");cache.moduleSets.set(closure,modules=new Set([...compiled].map(path=>resolve(workspaceRoot,path.slice("{workspaceRoot}/".length))))); }
  return [...new Set([...cache.generatedImports].filter(([path]) => modules.has(path)).flatMap(([, rows]) => rows.map(row => row.target)))].sort();
}

/** 🧭️ Tracks literal relative command imports through the existing TypeScript tooling boundary. */
export async function relativeScriptInputs(entries, workspaceRoot, cache = createScriptInputCache(), requiredSources=false) {
  const roots = [...new Set(entries.map(entry => resolve(entry)))].sort(), key = memoDigestV1(resolve(workspaceRoot), roots, requiredSources);
  let steps=0;const checkpoint=(force=false)=>yieldInferenceV1(cache.context,"build:command-source-imports",++steps,force);
  return memoizedObservedAsyncComputationV1(cache.closures,key,async()=>{
  const compiler = createRequire(import.meta.url)("typescript"), host = observedTypeScriptHost(compiler.sys), files = new Set(),compiled=new Set();
  const visit = async(path,source=true) => {
    await checkpoint();
    if (closurePathNameV1(cache, workspaceRoot, path).startsWith("{workspaceRoot}/../")) throw new Error(`Command import escapes workspace: ${path}`);
    if(presentationResourcesV1(cache).has(buildPath(relative(workspaceRoot,path))))return;
    const seen=files.has(path);
    files.add(path);
    if(requiredSources){const admitted=cache.physical??=new Set();if(!admitted.has(path)){requirePhysicalSourceV1(path);admitted.add(path);}}
    else if (!existsSync(path)) return;
    if(!source||!/\.[cm]?[jt]sx?$/u.test(path)){cache.resources.add(path);if(!seen)requirePhysicalSourceV1(path);return;}
    if(compiled.has(path))return;
    compiled.add(path);cache.modules.add(path);
    const loadsProviders = path === join(LIBRARY_ROOT, SCRIPT_PROVIDER_LOADER_MODULE);
    const imports = await memoizedObservedAsyncComputationV1(cache.files,JSON.stringify([path,requiredSources,loadsProviders?cache.providerDesignation??null:undefined]),async()=>{
      const imports = loadsProviders ? scriptProviderImportsV1(cache) : [];
      for (const entry of observedSourceAnalysisV1(prepareSourceAnalysesV1().command,path)) {
        await checkpoint();
        const resource=buildImportResourceV1(path,entry);
        const produced=cache.generatedOutput?.(resource.specifier);if(produced){const rows=cache.generatedImports.get(path)??[];if(!rows.some(row=>row.target===produced.target&&row.output===produced.output))rows.push(produced);cache.generatedImports.set(path,rows);continue;}
        let resolved;
        try { resolved = observedRequireResolve(path, resource.specifier); }
        catch (error) {
          if (error.code !== "MODULE_NOT_FOUND") throw error;
          resolved = compiler.resolveModuleName(resource.specifier, path, { moduleResolution: compiler.ModuleResolutionKind.Bundler, allowJs: true, resolveJsonModule: true }, host).resolvedModule?.resolvedFileName;
          if (!resolved) {if(requiredSources||resource.suffix||!/\.[cm]?[jt]sx?$/u.test(resource.specifier))throw error;continue;}
          if (/\.d\.[cm]?ts$/.test(resolved)) throw error;
        }
        imports.push({path:resolved,source:resource.source});
      }
      const references = observedSourceAnalysisV1(prepareSourceAnalysesV1().references,path);
      for (const specifier of references.runtime) {
        await checkpoint();
        const member = workspacePackageModuleV1(path, specifier, workspaceRoot);
        if (member) imports.push({path:member,source:true});
      }
      for (const literal of references.urls) {
        await checkpoint();
        const target = workspaceUrlTargetV1(path, literal, workspaceRoot);
        if (target) imports.push(target);
      }
      return imports;
    },checkpoint);
    for (const imported of imports) await visit(imported.path,imported.source);
  };
  for (const entry of roots) await visit(entry);
  const result = [...files].map((file) => closurePathNameV1(cache, workspaceRoot, file)).sort();
  cache.compiledNames.set(result, new Set([...compiled].map((file) => closurePathNameV1(cache, workspaceRoot, file))));
  return result;
  },checkpoint);
}

/** 🙈️ The resources the build policy declares presentation-only (progress labels): their edits never change a command's behavior, so they are not command inputs. */
function presentationResourcesV1(cache) {
  return cache.presentation ??= new Set(JSON.parse(readPhysicalSource(join(LIBRARY_ROOT, "🏗️build", "🔣️policy.json")).toString("utf8")).presentationResources);
}

/** 🔧️ Native leaves hash their compiler contract and exact command implementation, independently of UI selection. */
export async function nativeCommandInputs(workspaceRoot, scripts) {
  const script = join(LIBRARY_ROOT, "⚡️caching/🦀️cargo/📜️script.ts");
  const cargo = POLICY.toolchains.cargo, javascript = POLICY.toolchains.javascript;
  return [
    ...await relativeScriptInputs([script], workspaceRoot, scripts),
    ...[...javascript.files.filter((file) => !["package.json", "bun.lock"].includes(file)), ...cargo.files].map((file) => `{workspaceRoot}/${file}`),
    { externalDependencies: ["@iarna/toml"] },
    ...cargo.environment.map((env) => ({ env })),
    ...[...javascript.commands, ...cargo.commands].map((runtime) => ({ runtime })),
    { runtime: 'node -p "process.platform.concat(process.arch)"' },
  ];
}

/** 🧭️ Captures outer scripts and explicitly declared child scripts in their own working directories. */
export async function targetScriptSources(target, workspaceRoot, scripts, projectRoot = ".",nativeManifest) {
 const entries=new Set();let fallback=false;
 const retain=bound=>{if(bound.script){requirePhysicalSourceV1(bound.script);entries.add(bound.script);}else fallback=true;};
 const local=path=>{const value=buildPath(relative(workspaceRoot,path));if(value===".."||value.startsWith("../")||isAbsolute(value))throw Error("Declared command Source escapes workspace");return value||".";};
 for(const options of targetCommandRecipes(target)){
  const commands=typeof options.command==="string"?[options.command]:options.commands??[];if(!commands.length)fallback=true;
  for(const operation of commands){
   const command=typeof operation==="string"?operation:operation.command,arguments_=options.forwardAllArgs!==false&&(typeof operation==="string"||operation.forwardAllArgs!==false)?options.args??[]:[],bound=bindBuildCommandV1(command,workspaceRoot,options.cwd??".",projectRoot,arguments_);retain(bound);
   const owner=options.nativeOwnerCommand,selected=owner?bindBuildCommandV1([owner.program,...owner.arguments].map(value=>JSON.stringify(value)).join(" "),workspaceRoot,owner.workingDirectory,projectRoot,arguments_):bound;if(owner)retain(selected);
   const explicitManifest=owner?.manifest??nativeManifest,manifest=resolve(workspaceRoot,explicitManifest??join(projectRoot,"Cargo.toml"));local(manifest);if(explicitManifest===undefined&&!existsSync(manifest))continue;
   const cargo=readToml(manifest),reference=cargo.package?.metadata?.semio?.["cargo-execution"];if(reference===undefined)continue;if(typeof reference!=="string"||!reference||isAbsolute(reference))throw Error("Relative Cargo execution declaration required");
   const declarationPath=resolve(dirname(manifest),reference);local(declarationPath);readPhysicalSource(manifest);const declaration=readCargoExecutionContractV1("declaration",JSON.parse(readPhysicalSource(declarationPath).toString("utf8")));entries.add(manifest);entries.add(declarationPath);
   const selectedOperation=selectCargoExecutionOperationV1(declaration,{program:selected.program,cwd:owner?.workingDirectory??options.cwd??".",arguments:selected.arguments},workspaceRoot);if(!selectedOperation?.continuation)continue;
   const child=selectedOperation.continuation,childBound=bindBuildCommandV1([child.program,...child.arguments].map(value=>JSON.stringify(value)).join(" "),workspaceRoot,child.cwd,projectRoot,[]);if(!childBound.script)throw Error("Declared Cargo continuation requires an explicit script Source");retain(childBound);
  }
 }
 const baseSize=entries.size,base=baseSize?await relativeScriptInputs([...entries],workspaceRoot,scripts,true):undefined;
 if(buildInputSourceRoleV1(true,false,[target.metadata?.semio?.sourceInputObservation])==="compile")for(const declared of declaredModuleInputsV1(target.inputs,workspaceRoot,projectRoot,scripts,base&&scripts.compiledNames.get(base)))entries.add(declared);
 for(const declared of declaredModuleGlobEntriesV1(target.metadata?.semio?.closure?.modules,workspaceRoot,projectRoot,scripts))entries.add(declared);
 for(const input of readBuildRuntimeSourceEntriesV1(target.metadata?.semio?.runtimeSourceEntries)){const path=resolve(workspaceRoot,input.replace("{projectRoot}",projectRoot).replace("{workspaceRoot}","."));if(path.length>256)throw Error("Bounded physical runtime Source entry required");const entry=declaredModuleInputV1(input,workspaceRoot,projectRoot);if(!entry)throw Error("Bounded physical runtime Source entry required");entries.add(entry);}
 let closure=entries.size===baseSize?base:entries.size?await relativeScriptInputs([...entries],workspaceRoot,scripts,true):undefined,generatedDependencies=scripts.generatedImports.size&&closure?generatedTargetsV1(scripts,closure,workspaceRoot):[];const project=closure?declaredProjectClosureV1(target.metadata?.semio?.closure?.typescript,workspaceRoot,projectRoot,scripts,closure):undefined;
 if(project){closure=project.closure;generatedDependencies=[...new Set([...(scripts.generatedImports.size?generatedTargetsV1(scripts,closure,workspaceRoot):[]),...project.generatedDependencies])].sort();}
 return {closure,fallback,...(generatedDependencies.length?{generatedDependencies}:{}),...(project?.packages.length?{packages:project.packages,projectConfig:project.config}:{})};
}

/** ⚙️ Keeps configured recipe inheritance identical for command and lock source discovery. */
function targetCommandRecipes(target) {
  return [target.options ?? {}, ...Object.values(target.configurations ?? {}).map(options => ({ ...target.options, ...options }))];
}

/** 🗂️ Keeps configured executable imports in the target's invariant source closure. */
export async function targetScriptClosure(target, workspaceRoot, scripts, projectRoot = ".") {
  return (await targetScriptSources(target,workspaceRoot,scripts,projectRoot)).closure;
}

/** 🔐️ `cargo metadata --locked` validates the shared lock against every workspace manifest, so its replay must hash all of them. */
export function nativeLockInputs(command) {
  if(command && typeof command === "object")return [...new Set(targetCommandRecipes(command).flatMap(options=>(typeof options.command==="string"?[options.command]:options.commands??[]).flatMap(operation=>nativeLockInputs(typeof operation==="string"?operation:operation.command))))];
  return typeof command === "string" && command.includes(" native cargo metadata ") ? ["{workspaceRoot}/**/Cargo.toml", "{workspaceRoot}/**/Cargo.lock"] : [];
}

/** 🧭️ Adds the selected native target's local router and executable import closure. */
export async function nativeTargetCommandInputs(target, workspaceRoot, fallback, scripts, projectRoot = ".") {
  fallback??=await nativeCommandInputs(workspaceRoot,scripts);
  const closure = await targetScriptClosure(target, workspaceRoot, scripts, projectRoot);
  return closure ? [...new Set([...closure, ...fallback])] : fallback;
}

/** 🎯️ Hashes any script-backed target's own command closure, exactly, instead of the whole router. */
export async function genericTargetCommandInputs(target, workspaceRoot, fallback, scripts = createScriptInputCache(), projectRoot = ".") {
  const sources = await targetScriptSources(target, workspaceRoot, scripts, projectRoot),closure=sources.closure;
  if (!closure) return [...fallback, "{workspaceRoot}/package.json", "{workspaceRoot}/bun.lock"];
  const key=memoDigestV1(closure,sources.fallback?fallback:null,sources.packages??[]);
  let index=0;const checkpoint=(force=false)=>yieldInferenceV1(scripts.context,"build:command-source-packages",++index,force);
  return memoizedObservedAsyncComputationV1(scripts.commandDependencies,key,async()=>{
  const packages = new Set(), graph = existsSync(join(workspaceRoot, "bun.lock")) ? readBunLockGraph(workspaceRoot) : undefined;
  for (const input of closure) {
    await checkpoint();
    const file = resolve(workspaceRoot, input.slice("{workspaceRoot}/".length));
    if (!existsSync(file)) continue;
    for (const specifier of observedSourceAnalysisV1(prepareSourceAnalysesV1().module,file).imports) {
      if (specifier.startsWith(".") || specifier.startsWith("/") || isBuiltin(specifier) || specifier.startsWith("bun:")) continue;
      const name = specifier.startsWith("@") ? specifier.split("/").slice(0, 2).join("/") : specifier.split("/")[0];
      if (!graph) { packages.add(name); continue; }
      const key = graph.resolveImport(buildPath(relative(workspaceRoot, file)), name);
      if (key && graph.externalNodes[`npm:${key}`]) packages.add(`npm:${key}`);
    }
  }
  for (const name of sources.packages ?? []) {
    if (!graph) { packages.add(name); continue; }
    const key = graph.resolveImport(sources.projectConfig.slice("{workspaceRoot}/".length), name);
    if (key && graph.externalNodes[`npm:${key}`]) packages.add(`npm:${key}`);
  }
  const inputs = [...new Set([...closure,...(sources.fallback?[...fallback,"{workspaceRoot}/package.json","{workspaceRoot}/bun.lock"]:[])]), { externalDependencies: [...packages].sort() }];
  return inputs;
  },checkpoint);
}

/** 🪢️ The previous blanket router contract, kept only as a correctness fallback for a command naming no script. */
export function genericCommandFallbackInputs(workspaceRoot) {
  const runner = buildPath(relative(workspaceRoot, LIBRARY_ROOT));
  return [
    "{workspaceRoot}/📜️script.ts",
    `{workspaceRoot}/${runner}/**/*.{ts,tsx,js,mjs,cjs,json}`,
    `{workspaceRoot}/${runner}/🟨️.mjs`,
    `{workspaceRoot}/${runner}/⚡️caching/**/*`,
  ];
}

/** ⏳️ Delegates bounded inference work to the caller's original progress and continuation control; durable progress is published at most every 200 ms so its cost never scales with the number of inferred facts. */
export async function yieldInferenceV1(context,phase,index,force=false){
 if(!context.invocation)return;
 const last=yieldInferenceV1.last??=new WeakMap(),now=performance.now();
 if(!force&&now-(last.get(context)??-Infinity)<200){context.invocation.control.signal.throwIfAborted();return;}
 last.set(context,now);
 requireIssuedResourceControlInvocationV1(context.phaseControl,context.invocation);checkScriptInvocation(context.invocation);checkResourceLeaseControl(context.phaseControl);await advanceScriptInvocation(context.invocation,phase,"running");checkResourceLeaseControl(context.phaseControl);
}
