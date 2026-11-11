import {readTypeScriptPackageProducerV1,typeScriptPackagePublicDirectoryV1} from "./../../../../⚡️caching/📦️artifacts/🟦️typescript/🔗️binding/🏭️producer/🟦️.ts";
import { createBoundedCacheV1 } from "../../../🧊️bounded/🟨️.mjs";
import { createClosureStoreV1 } from "./💾️entries/🟨️.mjs";
import { rowKeyV1 } from "../../🕰️generations/🧮️fact-rows/🟨️.mjs";
import{refuseBuildV1}from"../../../📖️texts/🟦️.ts";
import {buildInputSourceRoleV1,readBuildRuntimeSourceEntriesV1,readBuildSourceResourceEntriesV1} from "../../🚪️admission/🟦️.ts";
import managersV1 from "../../../../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🟨️.cjs";
const javascriptManagerV1 = managersV1.selectedManagerV1("javascript");
/** 🧭️ Tracks the literal import closure of command scripts and the packages they consume. */
import { advanceScriptInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { bindBuildCommandV1 } from "../../../../🏗️build/📤️arguments/🟨️.mjs";
import { checkResourceLeaseControl } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { checkScriptInvocation, requireIssuedResourceControlInvocationV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { isBuiltin } from "node:module";
import { parserTypescriptV1 } from "../../../🚀️bootstrap/📦️parsers/🟨️.mjs";
import { createHash } from "node:crypto";
import { basename, dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { activeTypeScriptProgramReuseV1 } from "./♨️reuse/🟨️.mjs";
import { memoizedObservedAsyncComputationV1, memoizedObservedComputationV1, observedSourceAnalysisV1, observedRequireResolve, observedTypeScriptHost, observedFs, observeDetachedV1, shareObservationRowsV1, detachedFailureV1, retainDetachedFailureV1, observationLayerHoldsV1, observationRoundV1, observationRowsHoldV1, recordClosureFactV1, registerClosureProviderV1, observedFieldV1, registerFieldExtractorV1, retainObservedRowsV1, observeDetachedSyncV1, withObservationPassRoundV1 , observationLayerHoldsAsyncV1, observationRowsHoldAsyncV1, OBSERVATION_LIMITS_V1 } from "../../../../🏗️build/👁️observation/🟨️.mjs";
import { readCargoExecutionContractV1, selectCargoExecutionOperationV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️execution/🟦️.ts";
import { REVISION_POLICY_V1 } from "../../../📥️inputs/🔏️revision/🟨️.mjs";
import { INFERENCE_LIMITS_V1 } from "../🧬️schema/🟨️.mjs";
import { rowPathsV1, rowTouchedV1 } from "../../🧮️derived-nodes/🧩️closure-kinds/🧭️row-paths/🟨️.mjs";
import { validateJsonSchemaSubset } from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { GlobV1 } from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🌐️glob/🟦️.ts";
import { isRuntimeBuiltinSpecifierV1 } from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
import { routerCommandSpecifiersV1, routerCommandWordV1 } from "./🔀️router/🟨️.mjs";
import { cargoManifestDirectoriesAsyncV1, crateNamesOfV1, packageNameV1, entryFilesV1, includeRootsV1, readDerivedWalkV1 } from "../../../📥️inputs/🎯️precision/🟦️.ts";
import { buildSchedulingPrefixesV1 } from "../../../📥️source/🔏️closure/🗓️scheduling/🟦️.ts";
import { WALK_VIEW_POLICY_V1, closureRoleViewV1, walkViewAnswerV1, walkViewOfRecipeV1, walkViewPrefixesV1 } from "./👓️walk-views/🟨️.mjs";
import { PROVIDER_CLOSURE_POLICY_V1, expandClosureValueV1, joinWithProviderViewV1, providerGroupNameV1, providerProjectNameV1, providerReferenceV1, sharedReferencesOfV1, toolGroupNameV1, toolReferenceV1 } from "./🔌️provider-closures/🟨️.mjs";
import { createStringTableV1 } from "../../../👁️observation/🔢️row-table/🟨️.mjs";
import { createReachSetsV1 } from "./🧠️reach-memo/🟨️.mjs";
const { existsSync, statSync } = observedFs;

/** 🚧️ Turns an import of a command closure that resolves to nothing into the withheld-project failure of its importing source, so one broken file withholds its own project instead of aborting the whole graph. */
function unresolvedSourceImportV1(error,path){return Object.assign(Error(String(error.message),{cause:error}),{code:"BUILD_SOURCE_UNPARSEABLE",source:path,sha256:createHash("sha256").update(observedFs.readFileSync(path)).digest("hex")});}

/** 🏷️ Names this component as its roster entry does. */
export const component = "script-inputs";
let workspaceDataDirectory, buildPath, readPhysicalSource, requirePhysicalSourceV1, prepareSourceAnalysesV1, revisionIdentity, register, PROVIDER_PORTS, LIBRARY_ROOT, POLICY, readToml, readBunLockGraph, SCRIPT_PROVIDER_DESIGNATION, SCRIPT_PROVIDER_DESIGNATION_SCHEMA;

/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */
export function wire(registry) {
  ({ workspaceDataDirectory, buildPath, readPhysicalSource, requirePhysicalSourceV1, LIBRARY_ROOT, POLICY, SCRIPT_PROVIDER_DESIGNATION, SCRIPT_PROVIDER_DESIGNATION_SCHEMA } = registry.coordinates);
  ({ prepareSourceAnalysesV1 } = registry.sourceAnalyses);
  ({ revisionIdentity, register } = registry);
  PROVIDER_PORTS = register({ id: `build:script-provider-ports:v1:${createHash("sha256").update(revisionIdentity.url).digest("hex")}`, revision: createHash("sha256").update(scriptProviderPortsV1.toString()).digest("hex"), analyze: (_path, bytes) => scriptProviderPortsV1(bytes.toString("utf8")) });
  ({ readToml } = registry.nativeManifests);
  ({ readBunLockGraph } = registry.bunLock);
}

/** 🗂️ Shares source reads and command closures only within one graph construction. */
export function createScriptInputCache(context={}) {
  if(context.invocation){requireIssuedResourceControlInvocationV1(context.phaseControl,context.invocation);checkResourceLeaseControl(context.phaseControl);}
  return { files: new Map(), closures: new Map(), commandDependencies: new Map(), modules:new Set(), resources:new Set(),generatedImports:new Map(),declared:new Map(),paths:new Map(),compiledNames:new WeakMap(),walkPorts:new WeakMap(),moduleSets:new WeakMap(),context,providerDesignation:context.scriptProviderDesignation,lateClosures:context.lateClosures===true };
}

export { askHeavyQuestionV1, isHeavyQuestionV1, primeSharedClosuresInChildV1 } from "./🪆️heavy-questions/🟨️.mjs";

/** 🌊️ The script provider ports one module defines: the literal names of its `defineScriptProviderV1("…")` calls, `*` when a use of the function is not such a call (every designated provider is then requested); import and export specifiers and the declaration itself are no uses. */
export function scriptProviderPortsV1(text) {
  const names = new Set();
  for (const match of text.matchAll(/(?<!function\s)\bdefineScriptProviderV1\b(?!\s*[,}])/gu)) {
    const literal = /^(?:\s*<[^()]*>)?\s*\(\s*(["'])([a-z0-9-]+)\1/u.exec(text.slice(match.index + match[0].length));
    names.add(literal ? literal[2] : "*");
  }
  return [...names].sort();
}

/** 📜️ The script provider designation one walk resolves ports through, validated against the schema beside it (where the repository's designation keeps its own): its path, the schema's path and the declared providers. */
function providerDesignationV1(cache) {
  const path = cache.providerDesignation ?? join(LIBRARY_ROOT, SCRIPT_PROVIDER_DESIGNATION), schema = join(dirname(path), relative(dirname(SCRIPT_PROVIDER_DESIGNATION), SCRIPT_PROVIDER_DESIGNATION_SCHEMA)), domain = join(LIBRARY_ROOT, SCRIPT_PROVIDER_DESIGNATION_SCHEMA);
  for (const file of [path, schema, domain]) requirePhysicalSourceV1(file);
  return { path, schema, providers: observedFieldV1(path, "script-provider-designation", schema, { paths: [schema, domain] }) };
}

/** 🔌️ The crate name of a Cargo manifest: the one fact the crate index reads of it, so an edit of any other manifest line moves no walk. */
registerFieldExtractorV1("cargo-package-name", (text) => packageNameV1(text));

/** 🔌️ The providers a designation declares once it validates against the schema beside it and the repository's own: the projection a walk depends on, so an edit that changes neither the providers nor the verdict moves no closure. */
registerFieldExtractorV1("script-provider-designation", (text, schemaPath, _path, readOther) => {
  const declared = JSON.parse(text), document = JSON.parse(readOther(schemaPath)), domain = JSON.parse(readOther(join(LIBRARY_ROOT, SCRIPT_PROVIDER_DESIGNATION_SCHEMA)));
  if (validateJsonSchemaSubset(document, declared, document, domain?.$id === document?.$id ? [] : [domain]).length) throw Error("Complete repository script provider declaration required");
  return declared.providers;
});

/** 🚰️ The designated ports among the ports one module defines, which a walk names instead of descending into their modules: every designated port when a use is not a literal call (`*`); a port the designation does not name requests nothing. */
export function designatedPortsV1(cache, ports) {
  const { providers } = providerDesignationV1(cache);
  return ports.includes("*") ? Object.keys(providers).sort() : ports.filter((name) => Object.hasOwn(providers, name));
}

/** 🪪️ The providers project: the project whose root holds the designation the walk resolved ports through and declares it (`./🔌️provider-closures`); read through the observed memo, so every project that names a provider records the read. */
function providerProjectV1(cache) {
  const designation = cache.providerDesignation ?? join(LIBRARY_ROOT, SCRIPT_PROVIDER_DESIGNATION), config = join(dirname(designation), PROVIDER_CLOSURE_POLICY_V1.project.basename);
  return memoizedObservedComputationV1(cache.declared, memoDigestV1("provider-project", config), () => {
    if (!existsSync(config)) throw refuseBuildV1("invalid", "closure.provider-project-required", { designation: String(designation) });
    return providerProjectNameV1(JSON.parse(readPhysicalSource(config).toString("utf8")), basename(designation));
  });
}

/** 🔗️ The references a command closure read through `view` holds for the providers of `ports`: one named input of the providers project per port, never the provider's files. */
function providerReferencesV1(cache, ports, view) {
  if (!ports.length) return [];
  const project = providerProjectV1(cache);
  return ports.map(port => providerReferenceV1(project, port, view));
}

/** 🧰️ The reference a command closure holds for the declared tool command `name` (the cargo driver, the lock owner): its closure is the same for every target that runs the tool, so the providers project holds it once (`./🔌️provider-closures`). */
function toolReferencesV1(cache, name) {
  return [toolReferenceV1(providerProjectV1(cache), name)];
}

/** 🧿️ Keeps a logical import's interpretation separate from its physical file identity. */
export function buildImportResourceV1(containing,specifier){
 if(/^[a-z][a-z0-9+.-]*:/iu.test(specifier))return {specifier,suffix:"",source:true};
 const relative=specifier.startsWith(".");if(relative){const physical=resolve(dirname(containing),specifier);if(existsSync(physical))return {specifier:physical,suffix:"",source:true};}
 const start=specifier.startsWith("#")?1:0,index=specifier.slice(start).search(/[?#]/u),boundary=index<0?specifier.length:start+index,base=specifier.slice(0,boundary),url=new URL(relative?specifier:"./resource"+specifier.slice(boundary),pathToFileURL(containing)),query=url.searchParams;
 return {specifier:relative?fileURLToPath(url):base,suffix:url.search+url.hash,source:!query.has("raw")&&(!query.has("url")||query.has("worker")||query.has("sharedworker"))};
}

/** 📦️ Resolves a package specifier to the workspace module it names, so imports through `@scope/package` subpaths join a command closure like relative ones; installed registry packages stay covered by their locked external dependency. */
export function workspacePackageModuleV1(containing, specifier, workspaceRoot, compiler=parserTypescriptV1(), host=observedTypeScriptHost(compiler.sys), mode=compiler.ModuleKind.ESNext, declarations=false,producedOutput) {
  if (specifier.startsWith(".") || isAbsolute(specifier) || isBuiltin(specifier) || /^[a-z][a-z0-9+.-]*:/u.test(specifier)) return undefined;
  specifier=buildImportResourceV1(containing,specifier).specifier;
  const promised=producedOutput?.(specifier);if(promised&&!declarations)return resolve(workspaceRoot,promised.output);
  const options={...tsconfigCompilerOptionsV1(compiler,host,workspaceRoot,containing),moduleResolution:compiler.ModuleResolutionKind.NodeNext,module:compiler.ModuleKind.NodeNext,allowJs:true,resolveJsonModule:true,noDtsResolution:!declarations};
  let resolution=compiler.resolveModuleName(specifier,containing,options,host,undefined,undefined,mode);
  if(!resolution.resolvedModule&&options.paths)resolution=compiler.resolveModuleName(specifier,containing,{...options,moduleResolution:compiler.ModuleResolutionKind.Bundler,module:compiler.ModuleKind.ESNext},host,undefined,undefined,mode);
  const packageName=specifier.startsWith("@")?specifier.split("/").slice(0,2).join("/"):specifier.split("/")[0],future=new Map(),packages=[];
  for(const manifest of resolution.affectingLocations??[]){
   const physical=host.realpath?host.realpath(manifest):manifest,local=relative(workspaceRoot,physical);if(local===""||local.startsWith("..")||isAbsolute(local)||local.split(sep).includes("node_modules"))continue;
   const text=host.readFile(physical);if(text===undefined||JSON.parse(text).name!==packageName)continue;
   const producer=readTypeScriptPackageProducerV1(dirname(physical),workspaceRoot,path=>host.readFile(path));packages.push({root:dirname(physical),producer});if(!producer)continue;
   for(const output of producer.outputs){future.set(resolve(output),output);future.set(resolve(dirname(manifest),relative(dirname(physical),output)),output);}
  }
  if(future.size){const view={...host,fileExists:path=>future.has(resolve(path))||host.fileExists(path),directoryExists:path=>[...future.keys()].some(file=>dirname(file)===resolve(path))||host.directoryExists?.(path)===true,realpath:path=>future.get(resolve(path))??(host.realpath?host.realpath(path):path)};resolution=compiler.resolveModuleName(specifier,containing,options,view,undefined,undefined,mode);}
  const resolvedName=resolution.resolvedModule?.resolvedFileName??(declarations&&promised?.declarations?resolve(workspaceRoot,promised.declarations):undefined),resolved=resolvedName===undefined?undefined:isAbsolute(resolvedName)?resolve(resolvedName):resolvedName;
  if (!resolved) {
    const packageName=specifier.startsWith("@")?specifier.split("/").slice(0,2).join("/"):specifier.split("/")[0];
    for(const manifest of resolution.affectingLocations??[]){
      const physical=host.realpath?host.realpath(manifest):manifest,local=relative(workspaceRoot,physical);
      if(local===""||local.startsWith("..")||isAbsolute(local)||local.split(sep).includes("node_modules"))continue;
      const text=host.readFile(physical);
      if(text!==undefined&&JSON.parse(text).name===packageName){const error=new Error("Workspace import is not resolved by current package metadata: "+specifier+" from "+containing);error.code="MODULE_NOT_FOUND";throw error;}
    }
    return undefined;
  }
  if(!isAbsolute(resolved))return undefined;
  for(const {root,producer} of packages)if(resolved.startsWith(join(root,typeScriptPackagePublicDirectoryV1)+sep)&&!producer?.outputs.includes(resolved))throw Object.assign(Error("Workspace compiled output requires its authored artifact producer: "+specifier),{code:"MODULE_NOT_FOUND"});
  const local = relative(workspaceRoot, resolved);
  return local === "" || local.startsWith("..") || isAbsolute(local) || local.split(sep).includes("node_modules") ? undefined : resolved;
}

/** 🧾️ Whether a declared input is a literal module path of the project or the workspace, the only kind of input that can join a command closure as an entry. */
function declaredModuleLiteralV1(input) {
  return typeof input === "string" && !input.startsWith("!") && !/[*?{}[\]]/u.test(input.replace(/^\{(?:projectRoot|workspaceRoot)\}/u, "")) && DECLARED_MODULE_SUFFIX_V1.test(input) && /^\{(?:projectRoot|workspaceRoot)\}\//u.test(input);
}

/** 🧪️ Resolves a literal module path whose task role or explicit runtime contract admits its imports. */
function declaredModuleInputV1(input, workspaceRoot, projectRoot) {
  if (!declaredModuleLiteralV1(input)) return undefined;
  const path = resolve(workspaceRoot, input.replace("{projectRoot}", projectRoot).replace("{workspaceRoot}", "."));
  return !relative(workspaceRoot, path).startsWith("..") && existsSync(path) ? path : undefined;
}

/** 🔎️ Resolves a literal `new URL("…", import.meta.url)` reference to the workspace file it names, which is how a script points at the test files it runs and the data it reads. */
function workspaceUrlTargetV1(containing, literal, workspaceRoot, physical=true) {
  let url;
  try { url = new URL(literal, pathToFileURL(containing)); } catch { return undefined; }
  if (url.protocol !== "file:") return undefined;
  const path = fileURLToPath(url), local = relative(workspaceRoot, path);
  if (local === "" || local.startsWith("..") || isAbsolute(local) || local.split(sep).includes("node_modules") || physical&&(!existsSync(path) || !statSync(path).isFile())) return undefined;
  return { path, source: DECLARED_MODULE_SUFFIX_V1.test(path) };
}

/** 🔑️ Names a memo entry by the digest of its ordered parts, so a long path list is hashed once and never retained as a key string. */
function memoDigestV1(...parts) {
  const hash = createHash("sha256");
  for (const part of parts) {
    if (Array.isArray(part)) { hash.update("["); for (const item of part) hash.update(typeof item === "string" ? item : `\u0001${JSON.stringify(item)}`).update("\0"); hash.update("]"); }
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
/** ✔️ True when a target input names a module the entry closure already compiled: adding it as a closure entry changes nothing, and its existence is already implied by the file fact the closure read. */
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
/** 🕸️ Matches a literal TypeScript project file a target declares so the compiler's own program joins its closure. */
const DECLARED_TSCONFIG_V1 = /^\{(?:projectRoot|workspaceRoot)\}\/(?:[^*?{}[\]]*\/)?tsconfig(?:\.[^/*?{}[\]]+)?\.json$/u;
/** 📁️ Directories a declared glob never descends into. */
const DECLARED_SKIPPED_DIRECTORIES_V1 = new Set(["node_modules", ".git", "target"]);

/** 🗺️ Resolves a `{projectRoot}` or `{workspaceRoot}` input to a normalized workspace-relative pattern, or `undefined` when it names nothing inside the workspace. */
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

/** 🪁️ Runs the compiler's own program over the TypeScript projects a target declares (with `environment.reuse`, a long-lived deriver's store of parsed files and previous programs, see `./♨️reuse/🟨️.mjs`), so the typecheck closure is exactly what `tsc -p` reads: the include set, every resolved import (path mappings, `.d.ts` siblings, type-only imports, JSON), the configuration chain and the ambient type packages.
 * @returns {{files:string[],packages:string[],configs:string[]}} workspace-absolute files, installed package names, and the declared configuration files */
export function declaredTypeScriptProjectsV1(inputs, workspaceRoot, projectRoot, environment) {
  const { compiler, host, readdir, join, dirname, isAbsolute, reuse } = environment;
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
    if (reuse) compilerHost.getSourceFile = reuse.sourceFile(config, compilerHost.getSourceFile, host.readFile, JSON.stringify(options));
    const oldProgram = reuse?.oldProgram(config);
    const program = compiler.createProgram({ rootNames: parsed.fileNames, options, projectReferences: parsed.projectReferences, host: compilerHost, ...(oldProgram ? { oldProgram } : {}) });
    reuse?.keep(config, program);
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

/** 📂️ The module files the target's `metadata.semio.closure.modules` globs match, which join the target's command closure as entries. */
function declaredModuleGlobEntriesV1(globs, workspaceRoot, projectRoot, cache) {
  if (!Array.isArray(globs) || !globs.length) return [];
  return memoizedObservedComputationV1(cache.declared, memoDigestV1("module-globs", workspaceRoot, projectRoot, globs), () => declaredModuleGlobsV1(globs, workspaceRoot, projectRoot, { readdir: declaredDirectoryV1, Glob: GlobV1, join }).map(path => resolve(workspaceRoot, path)));
}

/** 🏃️ Runs the compiler's own program over the TypeScript projects a target declares, reading the live files through the observed host. */
function typescriptProgramV1(projects, workspaceRoot, projectRoot) {
  const compiler = parserTypescriptV1();
  return declaredTypeScriptProjectsV1(projects, workspaceRoot, projectRoot, { compiler, host: observedTypeScriptHost(compiler.sys), readdir: declaredDirectoryV1, join, dirname, isAbsolute, reuse: activeTypeScriptProgramReuseV1() });
}

/** ❓️ The canonical question one TypeScript program answers: the workspace, the project root and the declared project files; the files themselves are what the program reads. */
function programQuestionV1(workspaceRoot, projectRoot, projects) {
  return JSON.stringify({ program: 1, workspaceRoot: resolve(workspaceRoot), projectRoot, projects });
}

/** 🔏️ The digest of everything a project reads from one TypeScript program: its files, the installed packages it resolved and its configuration files. */
function programAnswerV1(project) {
  return createHash("sha256").update(JSON.stringify([project.files, project.packages, project.configs])).digest("hex");
}

/** 🏗️ Joins the compiler's own program for each TypeScript project in `metadata.semio.closure.typescript` to the closure, records the generator targets whose outputs it reads and names the installed packages it resolved. A program file joins by its physical identity alone (`requirePhysicalSourceV1`): its bytes are the derived program fact's support, so an edit that leaves the program as it was keeps the project's own facts (law `🧪️tests/🪁️program-reads`). */
function declaredProjectClosureV1(projects, workspaceRoot, projectRoot, cache, closure) {
  if (!Array.isArray(projects) || !projects.length) return undefined;
  const programKey = memoDigestV1("typescript-project", workspaceRoot, projectRoot, projects);
  const project = memoizedObservedComputationV1(cache.declared, programKey, () => typescriptProgramV1(projects, workspaceRoot, projectRoot));
  const programStored = cache.declared.get(programKey);
  if (programStored && !walkFactsV1.has(programStored.observations)) walkFactsV1.set(programStored.observations, { row: { kind: "derived", path: programQuestionV1(workspaceRoot, projectRoot, projects), fact: programAnswerV1(project) } });
  if (!project.files.length) return undefined;
  const compiled=cache.compiledNames.get(closure);if(!compiled)throw Error("Exact compiled command closure required");
  const config = resolve(workspaceRoot, project.configs[0]), names = [], generatedDependencies=new Set();
  for (const file of project.files) {
    const produced = cache.generatedOutput?.(file);
    if (produced) { const rows = cache.generatedImports.get(config) ?? []; if (!rows.some(row => row.target === produced.target && row.output === produced.output)) rows.push(produced); cache.generatedImports.set(config, rows); generatedDependencies.add(produced.target); continue; }
    requirePhysicalSourceV1(file);
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

/** 🗓️ Drops the scheduling code from a command closure: the files of the declared scheduling domains decide when and where the command runs, never what it writes, so an edit of one must not change the key of the target (the policy and its law live in the closure module's scheduling domains). */
export function withoutSchedulingV1(closure, prefixes) {
  const standard = prefixes === undefined, known = standard ? keptClosuresV1.get(closure) : undefined;
  if (known !== undefined) return known;
  const selected = prefixes ?? buildSchedulingPrefixesV1(), kept = closure.filter(entry => typeof entry !== "string" || !selected.some(prefix => (entry.startsWith("{workspaceRoot}/") ? entry.slice("{workspaceRoot}/".length) : entry).startsWith(prefix)));
  if (standard) keptClosuresV1.set(closure, kept);
  return kept;
}

/** 🪆️ The filtered copy of one command closure under the standard scheduling prefixes, so a warm replay returns the identical array and keeps its compiled evidence. */
const keptClosuresV1 = new WeakMap();

/** 🔀️ The router script is the one closure member that is a content fact: its command table is read from its bytes, so the imports of the commands it runs with are exactly the ones its text names. */
function routerKeptFromSourceV1(_compiler, path, words) { requirePhysicalSourceV1(path); const kept = observedFieldV1(path, "script-router-kept", JSON.stringify(words)); return kept === null ? undefined : new Set(kept); }

/** 🔌️ The imports a router script keeps for a list of command words (`null` when it is no router this reading understands): the one fact a walk reads of the router's text, so an edit that keeps them moves no walk. */
registerFieldExtractorV1("script-router-kept", (text, pointer, path) => { const kept = routerKeptV1(parserTypescriptV1(), path, text, JSON.parse(pointer)); return kept === undefined ? null : [...kept].sort(); });

/** 🧵️ The imports a router script needs for all the commands it runs with, or nothing when any of them is not a router this reading understands. */
function routerKeptV1(compiler, path, text, words) {
  const kept = new Set();
  for (const word of words) { let own; try { own = routerCommandSpecifiersV1(compiler, path, text, word); } catch { return undefined; } if (own === undefined) return undefined; for (const specifier of own) kept.add(specifier); }
  return kept;
}

const tsconfigOptionsByHostV1 = new WeakMap();
/** 🧭️ The `baseUrl`, `paths` and `extends` semantics bun resolves modules with: the options of the nearest `tsconfig.json` at or above the importing file below the workspace root, parsed by TypeScript itself through the observed host, so an edit of it moves the walk.
 * @see https://www.typescriptlang.org/tsconfig#paths */
export function tsconfigCompilerOptionsV1(compiler, host, workspaceRoot, file) {
  const root = resolve(workspaceRoot), memo = tsconfigOptionsByHostV1.get(host) ?? new Map();
  tsconfigOptionsByHostV1.set(host, memo);
  let at = dirname(file), config;
  while (config === undefined && (at === root || at.startsWith(root + sep))) { const candidate = join(at, "tsconfig.json"); if (host.fileExists(candidate)) config = candidate; if (at === root) break; at = dirname(at); }
  if (config === undefined) return {};
  if (!memo.has(config)) {
    const read = compiler.readConfigFile(config, host.readFile);
    memo.set(config, read.error || !read.config ? {} : compiler.parseJsonConfigFileContent(read.config, { useCaseSensitiveFileNames: true, readDirectory: () => [], fileExists: host.fileExists, readFile: host.readFile }, dirname(config), undefined, config).options);
  }
  return memo.get(config);
}
/** 🧭️ The Bundler resolution options of a module of a command closure, with the `baseUrl` and `paths` of its tsconfig. */
export function tsconfigResolutionOptionsV1(compiler, host, workspaceRoot, file) {
  return { ...tsconfigCompilerOptionsV1(compiler, host, workspaceRoot, file), moduleResolution: compiler.ModuleResolutionKind.Bundler, module: compiler.ModuleKind.ESNext, allowJs: true, resolveJsonModule: true };
}

/** 🌍️ The module resolution configuration bun reads for every module of a command closure (checked with the kernel read oracle: `undeclared-g`): the nearest `package.json` and the nearest `tsconfig.json` above each module directory, and the `tsconfig.json` files an `extends` names; the walk stops below the workspace root, whose own files are shared inputs of every target. Answers through the observed host, so a config that appears or changes re-keys the walk. */
export function resolutionConfigsV1(compiler, host, workspaceRoot, modules) {
  const root = resolve(workspaceRoot), found = new Set(), nearest = new Map();
  const above = (directory, name) => {
    const key = `${name}\0${directory}`;
    if (!nearest.has(key)) { let at = directory, hit; while (hit === undefined && at.startsWith(root + sep)) { const candidate = join(at, name); if (host.fileExists(candidate)) hit = candidate; at = dirname(at); } nearest.set(key, hit); }
    return nearest.get(key);
  };
  const extend = (config) => {
    if (found.has(config)) return;
    found.add(config);
    const named = compiler.readConfigFile(config, host.readFile).config?.extends;
    for (const specifier of [named].flat().filter((value) => typeof value === "string" && value.startsWith("."))) {
      const target = resolve(dirname(config), specifier), candidate = host.fileExists(target) ? target : `${target}.json`;
      if (candidate.startsWith(root + sep) && host.fileExists(candidate)) extend(candidate);
    }
  };
  for (const directory of new Set([...modules].map((file) => dirname(file)))) {
    const manifest = above(directory, "package.json"), config = above(directory, "tsconfig.json");
    if (manifest) found.add(manifest);
    if (config) extend(config);
  }
  return [...found].sort();
}

/** 🧠️ Tracks literal relative command imports through the existing TypeScript tooling boundary, up to the script provider ports: a module that defines ports adds the designated ports to the walk's ports (`cache.walkPorts`) instead of the provider modules, so a walk and its derived fact never contain a provider's files; a router script named in `commands` (absolute path to the registered command words it is run with) contributes only the imports those commands need, and every router this reading does not fully understand keeps all of its imports. */
async function boundedScriptInputsV1(entries, workspaceRoot, cache = createScriptInputCache(), requiredSources=false, resourceEntries=[], commands=new Map()) {
  const resources=new Set(resourceEntries.map(path=>resolve(path))),resourceKey=memoDigestV1([...resources].sort()),selectors=new Map([...commands].map(([path,words])=>[resolve(path),[...new Set(words)].sort()])),selectorKey=memoDigestV1([...selectors].sort(([a],[b])=>a<b?-1:a>b?1:0)),roots = [...new Set(entries.map(entry => resolve(entry)))].sort(), key = memoDigestV1(resolve(workspaceRoot), roots, requiredSources,resourceKey,selectorKey);
  let steps=0;const checkpoint=(force=false)=>yieldInferenceV1(cache.context,"build:command-source-imports",++steps,force);
  let memberParts=[];
  const walked=await memoizedObservedAsyncComputationV1(cache.closures,key,async()=>{
  const compiler = parserTypescriptV1(), host = observedTypeScriptHost(compiler.sys);
  const walk = await reachWalkV1({ roots, workspaceRoot, cache, requiredSources, resources, selectors, checkpoint, compiler, host });
  cache.compiledNames.set(walk.result, new Set([...walk.compiled].map((file) => closurePathNameV1(cache, workspaceRoot, file))));
  cache.walkPorts.set(walk.result, [...walk.defined].sort());
  memberParts = walk.members;
  return walk.result;
  },checkpoint);
  const stored=cache.closures.get(key);
  if(stored){
   const question=walkQuestionV1(cache,workspaceRoot,roots,requiredSources,resources,selectors);
   if(!walkFactsV1.has(stored.observations))walkFactsV1.set(stored.observations,{row:{kind:"derived",path:question,fact:walkAnswerV1(cache,workspaceRoot,walked)},views:Object.fromEntries(Object.keys(WALK_VIEW_POLICY_V1.views).map(view=>[view,{kind:"derived",path:walkQuestionV1(cache,workspaceRoot,roots,requiredSources,resources,selectors,view),fact:walkAnswerV1(cache,workspaceRoot,walked,view)}]))});
   for(const part of memberParts){let walks=walkMembersV1.get(part);if(!walks)walkMembersV1.set(part,walks=new Set());walks.add(question);}
   if(memberParts.length)walkPartsV1.set(walked,memberParts);
   if(cache.pass?.transient)cache.closures.delete(key);
  }
  return walked;
}

/** 🧭️ The edges of one module: the designated ports it defines, its command imports (through the router selectors of the walk, the workspace packages, the runtime and the TypeScript resolution), its runtime references and its literal URLs, resolved to the files they name. */
async function walkImportsV1({ cache, workspaceRoot, requiredSources, selectors, compiler, host, checkpoint }, path) {
  const sink = new Set(), output = cache.generatedOutput ? (specifier) => { const got = observeDetachedSyncV1(() => cache.generatedOutput(specifier)); for (const part of [got.rows, ...got.layers]) if (part.length) sink.add(part); return got.value; } : undefined;
      requirePhysicalSourceV1(path);
      const ports = observedSourceAnalysisV1(PROVIDER_PORTS, path), imports = ports.length ? designatedPortsV1(cache, ports).map(port => ({ port })) : [], words = selectors.get(path), kept = words?.length ? routerKeptFromSourceV1(compiler, path, words) : undefined;
      const generated=specifier=>{const produced=output?.(specifier);if(!produced)return false;const rows=cache.generatedImports.get(path)??[];if(!rows.some(row=>row.target===produced.target&&row.output===produced.output))rows.push(produced);cache.generatedImports.set(path,rows);for(const file of produced.sources??[])imports.push({path:resolve(workspaceRoot,file),source:true});return true;};
      for (const entry of observedSourceAnalysisV1(prepareSourceAnalysesV1().command,path)) {
        await checkpoint();
        if (kept && !kept.has(entry)) continue;
        const resource=buildImportResourceV1(path,entry);
        if(generated(resource.specifier))continue;
        let resolved=workspacePackageModuleV1(path,resource.specifier,workspaceRoot,compiler,host,compiler.ModuleKind.ESNext,false,output);
        try { resolved ??= observedRequireResolve(path, resource.specifier); }
        catch (error) {
          if (error.code !== "MODULE_NOT_FOUND") throw error;
          resolved = compiler.resolveModuleName(resource.specifier, path, tsconfigResolutionOptionsV1(compiler, host, workspaceRoot, path), host).resolvedModule?.resolvedFileName;
          if (!resolved) {if(requiredSources||resource.suffix||!DECLARED_MODULE_SUFFIX_V1.test(resource.specifier))throw unresolvedSourceImportV1(error,path);continue;}
          if (/\.d\.[cm]?ts$/.test(resolved)) throw error;
        }
        if(!generated(resolved))imports.push({path:resolved,source:resource.source});
      }
      const references = observedSourceAnalysisV1(prepareSourceAnalysesV1().references,path);
      for (const [specifiers, mode] of [[references.runtime.imports, compiler.ModuleKind.ESNext], [references.runtime.requires, compiler.ModuleKind.CommonJS]]) for (const specifier of specifiers) {
        await checkpoint();
        const resource=buildImportResourceV1(path,specifier);if(generated(resource.specifier))continue;const member=workspacePackageModuleV1(path,resource.specifier,workspaceRoot,compiler,host,mode,false,output);
        if (member&&!generated(member)) imports.push({path:member,source:resource.source});
      }
      for (const literal of references.urls) {
        await checkpoint();
        const target = workspaceUrlTargetV1(path, literal, workspaceRoot,false);if(!target||generated(target.path))continue;
        imports.push({...target,url:true});
      }
      Object.defineProperty(imports, "shared", { value: [...sink] });
      return imports;
}

/** 🕸️ One module of the command walk as a graph node: what visiting `path` (as a source module or as a resource) observed and the edges it resolved; built once per cache and pass, so a closure is a walk over nodes that are already in memory and its support is the nodes' rows by reference.
 * @see ../../../../👁️observation/🟨️.mjs `retainObservedRowsV1` */
async function walkNodeV1(env, path, source) {
  const { cache, workspaceRoot, requiredSources, selectors, compiler, host, checkpoint } = env, nodes = cache.walkNodes ??= new Map();
  const key = walkKeyV1(env, path, source);
  let entry = nodes.get(key);
  if (entry?.pass !== env.pass && entry?.node !== undefined) { if (walkPartsHoldV1(env.pass, [...entry.node.parts, ...entry.node.shared], workspaceRoot)) entry.pass = env.pass; else { nodes.delete(key); entry = undefined; } }
  if (entry === undefined) {
    entry = { pass: env.pass, node: undefined, building: undefined }; nodes.set(key, entry);
    entry.building = (async () => {
      const captured = await observeDetachedV1(async () => {
        if (closurePathNameV1(cache, workspaceRoot, path).startsWith("{workspaceRoot}/../")) throw new Error(`Command import escapes workspace: ${path}`);
        if (presentationResourcesV1(cache).has(buildPath(relative(workspaceRoot, path)))) return { kind: "skip", imports: [] };
        if (requiredSources) requirePhysicalSourceV1(path); else if (!existsSync(path)) return { kind: "missing", imports: [] };
        if (!source || !DECLARED_MODULE_SUFFIX_V1.test(path)) { requirePhysicalSourceV1(path); return { kind: "resource", imports: [] }; }
        const imports = await memoizedObservedAsyncComputationV1(cache.files, moduleKeyV1(cache, JSON.stringify([resolve(workspaceRoot), path, requiredSources, cache.providerDesignation ?? null, selectors.get(path) ?? null])), async () => walkImportsV1(env, path), checkpoint);
        return { kind: "module", shared: imports.shared ?? [], imports: imports.map((imported) => imported.url ? { ...imported, file: existsSync(imported.path) && statSync(imported.path).isFile() } : imported) };
      });
      const node = { shared: [], ...captured.value, parts: [captured.rows, ...captured.layers], path, source, name: closurePathNameV1(cache, workspaceRoot, path), no: cache.walkCount = (cache.walkCount ?? 0) + 1 };
      if (node.kind === "module") cache.modules.add(path); else if (node.kind === "resource") cache.resources.add(path);
      return node;
    })();
  }
  try { entry.node ??= await entry.building; } catch (error) { nodes.delete(key); retainDetachedFailureV1(error); throw error; }
  return entry.node;
}

/** ✅️ Whether every row array of a node still holds in this pass; an array shared by many nodes (the Wasm compile index) is checked once per pass. */
function walkPartsHoldV1(pass, parts, workspaceRoot) {
  pass.valid ??= new WeakSet();
  for (const part of parts) {
    if (pass.valid.has(part)) continue;
    if (pass.dirty !== undefined) {
      const meta = partReachV1(workspaceRoot, part);
      if (!rowTouchedV1(meta, pass.dirty)) {
        if (meta.outside.length && !pass.valid.has(meta.outside) && !observationRowsHoldV1(meta.outside)) return false;
        pass.valid.add(meta.outside); pass.valid.add(part); continue;
      }
    }
    if (!observationRowsHoldV1(part)) return false;
    pass.valid.add(part);
  }
  return true;
}

const OUTSIDE_V1 = "\0outside";
const partReaches = new WeakMap();
/** 🧭️ What can move the rows of one part (see `rowPathsV1`), with the rows about paths above the checkout kept apart: a journal that watches the checkout vouches for the rest, those are checked once per pass. */
function partReachV1(workspaceRoot, part) {
  let meta = partReaches.get(part);
  if (meta === undefined) {
    const relativeName = (path) => { const name = relative(workspaceRoot, path); return name === "" ? "" : name.startsWith("..") || isAbsolute(name) ? OUTSIDE_V1 : buildPath(name); };
    meta = { paths: new Set(), dirs: new Set(), bare: new Set(), config: false, volatile: false, outside: [] };
    for (const row of part) { const one = rowPathsV1(row, relativeName, OUTSIDE_V1); if (one.outside) meta.outside.push(row); for (const path of one.paths) meta.paths.add(path); for (const dir of one.dirs) meta.dirs.add(dir); for (const dir of one.bare) meta.bare.add(dir); meta.config ||= one.config; meta.volatile ||= one.volatile; }
    meta.paths = [...meta.paths]; meta.dirs = [...meta.dirs]; meta.bare = [...meta.bare];
    partReaches.set(part, meta);
  }
  return meta;
}

/** 🧭️ The directory configuration node: the configs the walk reads for the modules of one directory, with the rows that read them; the union over directories is `resolutionConfigsV1` of the whole set. */
async function walkConfigNodeV1(env, directory) {
  const { cache, workspaceRoot, compiler, host } = env, nodes = cache.walkNodes ??= new Map(), key = moduleKeyV1(cache, JSON.stringify(["configs", resolve(workspaceRoot), directory]));
  let entry = nodes.get(key);
  if (entry?.pass !== env.pass && entry?.node !== undefined) { if (walkPartsHoldV1(env.pass, entry.node.parts, workspaceRoot)) entry.pass = env.pass; else { nodes.delete(key); entry = undefined; } }
  if (entry === undefined) {
    entry = { pass: env.pass, node: undefined, building: undefined }; nodes.set(key, entry);
    entry.building = (async () => {
      const captured = await observeDetachedV1(async () => resolutionConfigsV1(compiler, host, workspaceRoot, [join(directory, "_")]));
      return { configs: captured.value, parts: [captured.rows, ...captured.layers] };
    })();
  }
  try { entry.node ??= await entry.building; } catch (error) { nodes.delete(key); throw error; }
  return entry.node;
}

/** 🌳️ The command walk over graph nodes: depth first over `(path, source)` pairs, the same visiting rules as the inline walk, every node's rows merged by reference into the closure's scope. */
async function reachWalkV1({ roots, workspaceRoot, cache, requiredSources, resources, selectors, checkpoint, compiler, host }) {
  const env = { cache, workspaceRoot, requiredSources, selectors, compiler, host, checkpoint, pass: cache.pass ?? {} };
  env.pass.valid ??= new WeakSet();
  const files = new Set(), compiled = new Set(), defined = new Set(), seen = new Set(), stack = roots.map((path) => [path, true]), directories = new Set(), members = new Set();
  let steps = 0;
  const drain = async () => {
    while (stack.length) {
      const [path, source] = stack.pop(), mark = (source ? "1" : "0") + path;
      if (seen.has(mark)) continue;
      seen.add(mark);
      if (++steps % 64 === 0) await checkpoint();
      const node = await walkNodeV1(env, path, source);
      for (const part of node.parts) { retainObservedRowsV1(part); members.add(part); }
      for (const part of node.shared) { retainObservedRowsV1(part); members.add(part); }
      if (node.kind === "skip") continue;
      files.add(path);
      if (node.kind === "missing") continue;
      if (node.kind === "resource") { cache.resources.add(path); continue; }
      compiled.add(path); cache.modules.add(path); directories.add(dirname(path));
      for (const imported of node.imports) {
        if (imported.port !== undefined) { defined.add(imported.port); continue; }
        const resource = imported.url && resources.has(resolve(imported.path));
        if (imported.url && !resource && !imported.file) continue;
        stack.push([imported.path, imported.source && !resource]);
      }
    }
  };
  await drain();
  const configs = new Set();
  for (const directory of directories) { const node = await walkConfigNodeV1(env, directory); for (const part of node.parts) { retainObservedRowsV1(part); members.add(part); } for (const config of node.configs) configs.add(config); }
  for (const config of [...configs].sort()) stack.push([config, false]);
  await drain();
  return { result: [...files].map((file) => closurePathNameV1(cache, workspaceRoot, file)).sort(), compiled, defined, members: [...members] };
}

const MEMO_SLOT_V1 = 2 ** 22;
const moduleKeyV1 = (cache, text) => (cache.moduleKeys ??= createStringTableV1()).id(text);
const walkKeyV1 = (env, path, source) => moduleKeyV1(env.cache, JSON.stringify([resolve(env.workspaceRoot), path, env.requiredSources, env.cache.providerDesignation ?? null, env.selectors.get(path) ?? null, source]));

/** 🧮️ The module graph of one cache with the reach set of every node: the dense bitset of the node numbers reachable from it (strongly connected components share one set), persistent across passes. A pass first checks which nodes its journal dirty set (or, without one, the files) can have changed, drops those and every node that reaches them, and rebuilds only those: an edit of one module recomputes its reverse-import path once and every closure over untouched nodes stays a lookup.
 * @returns {{graph:Map<number,{node:object;targets:number[]}>;reverse:Map<number,Set<number>>;reaches:Map<number,Uint32Array>;nodes:Map<number,object>;pass:object|undefined}} */
const reachStateV1 = (cache) => cache.reachState ??= { graph: new Map(), reverse: new Map(), reaches: new Map(), resources: new Map(), sets: createReachSetsV1(), nodes: new Map(), pass: undefined };

async function reachPassV1(env, state) {
  const { pass, workspaceRoot } = env;
  if (state.pass === pass) return;
  state.pass = pass;
  pass.valid ??= new WeakSet();
  const touched = [];
  for (const [key, entry] of state.graph) {
    const { node } = entry, config = node.config?.node;
    if (!walkPartsHoldV1(pass, [...node.parts, ...node.shared, ...(config?.parts ?? [])], workspaceRoot)) touched.push(key);
  }
  const stale = new Set(touched), queue = [...touched];
  while (queue.length) for (const above of state.reverse.get(queue.pop()) ?? []) if (!stale.has(above)) { stale.add(above); queue.push(above); }
  for (const key of stale) state.graph.delete(key);
  for (const key of touched) env.cache.walkNodes?.delete(key);
  for (const memoKey of [...state.reaches.keys()]) if (stale.has(memoKey % MEMO_SLOT_V1)) state.reaches.delete(memoKey);
  if (stale.size) state.sets.retain(new Set(state.reaches.values()));
}

/** 🧮️ The files a command walk reaches from `entries` as a union of per-node reach sets (see `reachStateV1`). The sets are built without router selectors, so every walk shares them; a router named by a selector keeps only the imports its commands need, so the walk expands exactly the nodes that can reach such a router (their own edges, the router through its selected node) and takes every other subtree as one set. The answer, the compiled modules, the ports and the support parts are those of the walk over the same nodes (`reachWalkV1`); a differential law holds them equal.
 * @returns {Promise<{result:string[];compiled:Set<string>;defined:Set<string>;parts:Set<readonly unknown[]>}>} */
async function reachClosureV1({ roots, workspaceRoot, cache, requiredSources, resources, selectors, checkpoint, compiler, host }) {
  const pass = cache.pass ?? {}, base = { cache, workspaceRoot, requiredSources, selectors: new Map(), compiler, host, checkpoint, pass }, chosen = { ...base, selectors }, state = reachStateV1(cache);
  pass.valid ??= new WeakSet();
  await reachPassV1(base, state);
  const resourceKey = [...resources].sort().join("\0"), { graph, reverse, reaches, nodes, sets } = state, slot = state.resources.get(resourceKey) ?? state.resources.set(resourceKey, state.resources.size).get(resourceKey), memoKey = key => { if (key >= MEMO_SLOT_V1) throw Error("Module key space exhausted"); return slot * MEMO_SLOT_V1 + key; };
  const configsOf = async (env, node) => { if (node.kind !== "module") return []; const have = node.config?.pass === pass ? node.config.node : (node.config = { pass, node: await walkConfigNodeV1(env, dirname(node.path)) }).node; return have.configs; };
  const targetsOf = (node, configs) => {
    const out = [];
    for (const imported of node.imports) {
      if (imported.port !== undefined) continue;
      const resource = imported.url && resources.has(resolve(imported.path));
      if (imported.url && !resource && !imported.file) continue;
      out.push([imported.path, imported.source && !resource]);
    }
    for (const config of configs) out.push([config, false]);
    return out;
  };
  let steps = 0;
  const recordOf = async (env, path, source) => {
    const key = walkKeyV1(env, path, source);
    if (env === base) { let record = graph.get(key); if (record !== undefined) return record; }
    const cached = cache.walkNodes?.get(key), node = cached?.node !== undefined && cached.pass === pass ? cached.node : await walkNodeV1(env, path, source);
    nodes.set(node.no, node);
    const targets = targetsOf(node, await configsOf(env, node)), record = { node, targets: targets.map(([target, targetSource]) => walkKeyV1(base, target, targetSource)), edges: targets };
    if (env === base) { graph.set(key, record); for (const target of record.targets) { let above = reverse.get(target); if (above === undefined) reverse.set(target, above = new Set()); above.add(key); } }
    return record;
  };
  const build = async entry => {
    const queue = [entry], seen = new Set();
    while (queue.length) {
      const [path, source] = queue.pop(), key = walkKeyV1(base, path, source);
      if (reaches.has(memoKey(key)) || seen.has(key)) continue;
      seen.add(key);
      if (++steps % 64 === 0) await checkpoint();
      const record = await recordOf(base, path, source);
      for (const target of record.edges) queue.push(target);
    }
  };
  const reachOf = async (path, source) => {
    const key = walkKeyV1(base, path, source);
    if (!reaches.has(memoKey(key))) {
      await build([path, source]);
      const order = new Map(), low = new Map(), onStack = new Set(), stack = []; let counter = 0;
      const visit = at => {
        order.set(at, counter); low.set(at, counter); counter++; stack.push(at); onStack.add(at);
        for (const target of graph.get(at).targets) {
          if (reaches.has(memoKey(target))) continue;
          if (!order.has(target)) { visit(target); low.set(at, Math.min(low.get(at), low.get(target))); }
          else if (onStack.has(target)) low.set(at, Math.min(low.get(at), order.get(target)));
        }
        if (low.get(at) === order.get(at)) {
          const members = []; let top;
          do { top = stack.pop(); onStack.delete(top); members.push(top); } while (top !== at);
          const lists = [sets.of(members.map(member => graph.get(member).node.no))];
          for (const member of members) for (const target of graph.get(member).targets) { const done = reaches.get(memoKey(target)); if (done !== undefined) lists.push(done); }
          const reach = sets.union(lists);
          for (const member of members) reaches.set(memoKey(member), reach);
        }
      };
      visit(key);
    }
    return reaches.get(memoKey(key));
  };
  const lists = [], individual = [];
  if (selectors.size === 0) for (const path of roots) lists.push(await reachOf(path, true));
  else {
    const listed = [];
    for (const path of selectors.keys()) { await reachOf(path, true); const record = graph.get(walkKeyV1(base, path, true)); if (record !== undefined) listed.push(record.node.no); }
    const visited = new Set(), queue = roots.map(path => [path, true]);
    while (queue.length) {
      const [path, source] = queue.pop(), key = walkKeyV1(chosen, path, source);
      if (visited.has(key)) continue;
      visited.add(key);
      if (selectors.has(path) && source) { const record = await recordOf(chosen, path, true); individual.push(record.node); for (const target of record.edges) queue.push(target); continue; }
      const reach = await reachOf(path, source);
      if (!listed.some(no => sets.has(reach, no))) { lists.push(reach); continue; }
      const record = await recordOf(base, path, source); individual.push(record.node); for (const target of record.edges) queue.push(target);
    }
  }
  const merged = sets.members(sets.union([...lists, sets.of(individual.map(node => node.no))])), files = [], compiled = new Set(), defined = new Set(), parts = new Set();
  for (const no of merged) {
    const node = nodes.get(no);
    for (const part of node.parts) parts.add(part);
    for (const part of node.shared) parts.add(part);
    if (node.kind === "skip") continue;
    files.push(node.name);
    if (node.kind === "module") { compiled.add(node.name); for (const imported of node.imports) if (imported.port !== undefined) defined.add(imported.port); for (const part of node.config?.node?.parts ?? []) parts.add(part); }
  }
  return { result: [...new Set(files)].sort(), compiled, defined, parts };
}

/** 🌐️ The full command closure of `entries` as a process loads it: the bounded walk and, for every script provider port it names, the designation, its schema and the provider's own bounded walk, transitively; composed from the bounded walks, so each provider walk is computed once and every caller holds the same walk facts. Callers that list files read this (owner discovery, component targets, input precision, a whole-walk closure such as the test discovery); a command closure read through a view names its providers by reference instead (`./🔌️provider-closures`). */
export async function relativeScriptInputs(entries, workspaceRoot, cache = createScriptInputCache(), requiredSources=false, resourceEntries=[], commands=new Map()) {
  const own = await boundedScriptInputsV1(entries, workspaceRoot, cache, requiredSources, resourceEntries, commands), pending = [...(cache.walkPorts.get(own) ?? [])];
  if (!pending.length) return own;
  const designation = providerDesignationV1(cache), files = new Set([...own, closurePathNameV1(cache, workspaceRoot, designation.path), closurePathNameV1(cache, workspaceRoot, designation.schema)]), compiled = new Set(cache.compiledNames.get(own)), seen = new Set(pending);
  for (const walk of await providerWalksV1(cache, workspaceRoot, pending, designation)) { for (const name of walk) files.add(name); for (const name of cache.compiledNames.get(walk) ?? []) compiled.add(name); }
  const result = [...files].sort();
  cache.compiledNames.set(result, compiled);
  return result;
}

/** 📏️ The bounded walks of the designated modules of `ports` and of every port those walks name in turn; a designated module's sources are required (a designated module that is missing fails closed). */
async function providerWalksV1(cache, workspaceRoot, ports, designation = providerDesignationV1(cache)) {
  const pending = [...ports], seen = new Set(pending), walks = [];
  while (pending.length) {
    const walk = await boundedScriptInputsV1([resolve(dirname(designation.path), designation.providers[pending.shift()])], workspaceRoot, cache, true);
    walks.push(walk);
    for (const port of cache.walkPorts.get(walk) ?? []) if (!seen.has(port)) { seen.add(port); pending.push(port); }
  }
  return walks;
}

/** 🪢️ The derived fact of every command walk whose stored observations a project can reuse, by those observations: the question the walk answered and the digest of its answer, so a project names the walk by one row instead of by the thousands of rows it read; `views` holds the same walk asked through each declared view (`./👓️walk-views`), the fact a project holds when every consumer of its recipe reads the walk through that view. */
const walkFactsV1 = new WeakMap();

/** 🗝️ The digest of the answer of a repository walk: what a project that used the walk depends on. */
export function derivedWalkAnswerV1(value) { return createHash("sha256").update(JSON.stringify(value)).digest("hex"); }

/** 🚶️ The answer of one repository walk (the crate index of the Cargo manifests, or the TypeScript files a question selects), read through the observed file system. */
async function walkValueV1(recipe, workspaceRoot, step) {
  const local = path => join(workspaceRoot, path), list = directory => { try { return observedFs.readdirSync(local(directory), { withFileTypes: true }).map(entry => ({ name: entry.name, directory: entry.isDirectory(), file: entry.isFile(), link: entry.isSymbolicLink() })); } catch { return []; } };
  const readText = path => { try { return observedFs.readFileSync(local(path), "utf8"); } catch { return undefined; } };
  if (recipe.fn === "crate-index") return [...crateNamesOfV1(await cargoManifestDirectoriesAsyncV1(list, recipe.skip, step), path => { try { return observedFieldV1(local(path), "cargo-package-name", ""); } catch { return undefined; } })];
  const host = { exists: path => existsSync(local(path)), listDirectory: directory => list(directory).map(({ name, directory }) => ({ name, directory })) };
  return recipe.question.mode === "entries" ? entryFilesV1(recipe.question.globs, host) : includeRootsV1(recipe.question.include, recipe.question.exclude, host);
}

/** 🌳️ One repository walk as a derived fact: the observations it read are one shared layer, named by the question and the digest of the answer, so a project that used the walk holds that one row instead of the directories the walk listed. */
async function derivedWalkV1(workspaceRoot, recipe, scripts) {
  const step = cooperativeStepV1(scripts, "build:derived-walk"), question = JSON.stringify({ derive: 1, workspaceRoot: resolve(workspaceRoot), recipe }), cache = new Map(), value = await memoizedObservedAsyncComputationV1(cache, question, () => walkValueV1(recipe, workspaceRoot, step), () => step()), stored = cache.get(question);
  if (stored && !walkFactsV1.has(stored.observations)) walkFactsV1.set(stored.observations, { row: { kind: "derived", path: question, fact: derivedWalkAnswerV1(value) } });
  return value;
}

/** ✍️ The stored observations of every file a command walk visited, by the questions of the walks that visited it: they are part of those walks' observations, so a project that names such a walk by its derived fact does not carry them again. */
const walkMembersV1 = new WeakMap();

/** 🧱️ The row arrays (node parts and shared computations) the walk that produced a result visited: the support of its derived fact, by reference. */
const walkPartsV1 = new WeakMap();

/** 🛤️ The canonical question one command walk answers: the workspace, the entry files, whether sources are required, the resource files, the router selectors and the script provider designation; nothing else decides the walk. A view names the declared view its answer is read through (`./👓️walk-views`); the whole walk names none. */
function walkQuestionV1(cache, workspaceRoot, roots, requiredSources, resources, selectors, view) {
  return JSON.stringify({ walk: 1, workspaceRoot: resolve(workspaceRoot), roots, requiredSources: Boolean(requiredSources), resources: [...resources].sort(), selectors: [...selectors].sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0), provider: cache.providerDesignation ?? null, ...(view === undefined ? {} : { view }) });
}

/** 🆔️ The digest of everything a project reads from one walk, through a view when one is named: the roster, the modules it compiled and the generated outputs those modules import. */
function walkAnswerV1(cache, workspaceRoot, value, view) {
  const compiled = [...(cache.compiledNames.get(value) ?? [])].sort();
  const generated = compiled.map(name => [name, cache.generatedImports.get(resolve(workspaceRoot, name.slice("{workspaceRoot}/".length))) ?? []]);
  return walkViewAnswerV1(value, compiled, generated, view, cache.walkPorts.get(value) ?? []);
}

/** 🔁️ Asks a derived fact of the command closure again against the live files (a command walk or a TypeScript program): its answer digest and every observation the computation read, for a generation that decides whether the projects holding the fact stay valid. */
export async function deriveClosureFactV1(row, cache, pass = {}) {
  cache.pass = pass;
  pass.round ??= new Map();
  return withObservationPassRoundV1(pass.round, () => deriveClosureFactInRoundV1(row, cache));
}

async function deriveClosureFactInRoundV1(row, cache) {
  const question = JSON.parse(row.path), refuse = () => refuseBuildV1("invalid", "closure.derived-question-unknown", { question: String(row.path).slice(0, 96) });
  const support = ({ rows, layers }) => [...new Set([...rows, ...layers.flat()])];
  if (question?.program === 1) {
    if (!Array.isArray(question.projects) || typeof question.projectRoot !== "string") throw refuse();
    const asked = await observeDetachedV1(() => typescriptProgramV1(question.projects, question.workspaceRoot, question.projectRoot));
    return { fact: programAnswerV1(asked.value), rows: support(asked) };
  }
  if (question?.derive === 1) {
    let recipe;
    try { recipe = readDerivedWalkV1(question.recipe); } catch { throw refuse(); }
    const asked = await observeDetachedV1(() => derivedWalkV1(question.workspaceRoot, recipe, cache));
    return { fact: derivedWalkAnswerV1(asked.value), rows: support(asked) };
  }
  if (question?.walk !== 1 || !Array.isArray(question.roots) || !Array.isArray(question.resources) || !Array.isArray(question.selectors) || question.view !== undefined && !Object.hasOwn(WALK_VIEW_POLICY_V1.views, question.view)) throw refuse();
  if ((question.provider ?? null) !== (cache.providerDesignation ?? null)) throw refuseBuildV1("invalid", "closure.derived-provider-differs", { provider: String(question.provider) });
  if (cache.pass?.transient === true) {
    const compiler = parserTypescriptV1(), host = observedTypeScriptHost(compiler.sys), resources = new Set(question.resources.map((path) => resolve(path))), selectors = new Map(question.selectors.map(([path, words]) => [resolve(path), [...new Set(words)].sort()])), roots = [...new Set(question.roots.map((entry) => resolve(entry)))].sort();
    let steps = 0;
    const walk = await reachClosureV1({ roots, workspaceRoot: question.workspaceRoot, cache, requiredSources: Boolean(question.requiredSources), resources, selectors, checkpoint: (force = false) => yieldInferenceV1(cache.context, "build:command-source-imports", ++steps, force), compiler, host });
    cache.compiledNames.set(walk.result, walk.compiled);
    cache.walkPorts.set(walk.result, [...walk.defined].sort());
    const reached = { fact: walkAnswerV1(cache, question.workspaceRoot, walk.result, question.view), parts: [...walk.parts] };
    return Object.defineProperties(reached, { rows: { get: () => [...new Set(reached.parts.flat())], enumerable: true }, files: { value: walk.result } });
  }
  const asked = await observeDetachedV1(() => boundedScriptInputsV1(question.roots, question.workspaceRoot, cache, question.requiredSources, question.resources, new Map(question.selectors)));
  const answer = { fact: walkAnswerV1(cache, question.workspaceRoot, asked.value, question.view), parts: [asked.rows, ...(walkPartsV1.get(asked.value) ?? asked.layers)] };
  return Object.defineProperties(answer, { rows: { get: () => support(asked), enumerable: true }, files: { value: asked.value } });
}

/** 🙈️ The resources the build policy declares presentation-only (progress labels): their edits never change a command's behavior, so they are not command inputs. The policy is read once per script input cache and the facts of that read are replayed into every receiving project, so the rows of a project never depend on which project of the same process asked first. */
function presentationResourcesV1(cache) {
  return memoizedObservedComputationV1(cache.presentation ??= new Map(), "presentation", () => { const path = join(LIBRARY_ROOT, "🏗️build", "🔣️policy.json"); requirePhysicalSourceV1(path); return new Set(observedFieldV1(path, "json", "/presentationResources")); });
}

/** 🔧️ Native leaves hash their compiler contract and exact command implementation, independently of UI selection: the cargo driver by reference to its tool closure (read through the native view: the scheduling code of leases, capacity, storage, supervision, retries and progress texts stays out, the native-build modules that assemble the Cargo arguments stay in), the toolchain files, environment and commands by value. */
export async function nativeCommandInputs(workspaceRoot, scripts) {
  const cargo = POLICY.toolchains.cargo, javascript = POLICY.toolchains.javascript;
  return [
    ...toolReferencesV1(scripts, "cargo-command"),
    ...[...javascript.files.filter((file) => !["package.json", javascriptManagerV1.primaryLockName].includes(file)), ...cargo.files].map((file) => `{workspaceRoot}/${file}`),
    { externalDependencies: ["@iarna/toml"] },
    ...cargo.environment.map((env) => ({ env })),
    ...[...javascript.commands, ...cargo.commands].map((runtime) => ({ runtime })),
    { runtime: 'node -p "process.platform.concat(process.arch)"' },
  ];
}

/** 👷️ The literal child a process-owner wrap runs behind its driver (`repo "<driver>" owner-command --cwd <dir> -- <child>`), or undefined for any other bound command: the driver only forwards, so the files the child loads decide what the target writes and belong to its closure like those of the unwrapped command; a native wrap names its child by `nativeOwnerCommand` instead. */
export function ownerCommandChildV1(options, bound) {
  if (options.ownerArguments !== true || options.nativeOwnerCommand !== undefined || !bound.script) return undefined;
  const at = bound.arguments.indexOf(bound.script.replaceAll("\\", "/")), rest = at < 0 ? [] : bound.arguments.slice(at + 1), split = rest.indexOf("--"), flag = rest.indexOf("--cwd");
  if (rest[0] !== "owner-command" || split < 0 || flag < 0 || flag > split - 2 || split === rest.length - 1) return undefined;
  return { cwd: rest[flag + 1], command: rest.slice(split + 1).map(value => JSON.stringify(value)).join(" ") };
}

/** 📸️ Captures outer scripts and explicitly declared child scripts in their own working directories. */
export async function targetScriptSources(target, workspaceRoot, scripts, projectRoot = ".",nativeManifest) {
  const resources=readBuildSourceResourceEntriesV1(target.metadata?.semio?.closure?.resources).map(input=>{const path=resolve(workspaceRoot,input.replace("{projectRoot}",projectRoot).replace("{workspaceRoot}",".")),local=relative(workspaceRoot,path);if(path.length>REVISION_POLICY_V1.maximumPathUnits||local===".."||local.startsWith(".."+sep)||isAbsolute(local))throw Error("Bounded physical Source resource required");return path;});if(new Set(resources).size!==resources.length)throw Error("Unique physical Source resource entries required");
  const entries=new Set();let fallback=false;
 const words=new Map(),retain=bound=>{if(bound.script){requirePhysicalSourceV1(bound.script);entries.add(bound.script);const word=routerCommandWordV1(bound),known=words.get(bound.script);if(word===undefined)words.set(bound.script,null);else if(known===undefined)words.set(bound.script,new Set([word]));else if(known)known.add(word);}else fallback=true;};
 const local=path=>{const value=buildPath(relative(workspaceRoot,path));if(value===".."||value.startsWith("../")||isAbsolute(value))throw Error("Declared command Source escapes workspace");return value||".";};
 for(const options of targetCommandRecipes(target)){
  const commands=typeof options.command==="string"?[options.command]:options.commands??[];if(!commands.length)fallback=true;
  for(const operation of commands){
   const command=typeof operation==="string"?operation:operation.command,arguments_=options.forwardAllArgs!==false&&(typeof operation==="string"||operation.forwardAllArgs!==false)?options.args??[]:[],bound=bindBuildCommandV1(command,workspaceRoot,options.cwd??".",projectRoot,arguments_);retain(bound);
   const ownerWrap=ownerCommandChildV1(options,bound),ownerChild=ownerWrap&&bindBuildCommandV1(ownerWrap.command,workspaceRoot,ownerWrap.cwd,projectRoot,[]);if(ownerChild?.script)retain(ownerChild);
   const owner=options.nativeOwnerCommand,selected=owner?bindBuildCommandV1([owner.program,...owner.arguments].map(value=>JSON.stringify(value)).join(" "),workspaceRoot,owner.workingDirectory,projectRoot,arguments_):bound;if(owner)retain(selected);
   const explicitManifest=owner?.manifest??nativeManifest,manifest=resolve(workspaceRoot,explicitManifest??join(projectRoot,"Cargo.toml"));local(manifest);if(explicitManifest===undefined&&!existsSync(manifest))continue;
   let reference;try{reference=observedFieldV1(manifest,"toml","/package/metadata/semio/cargo-execution");}catch(error){if(!String(error?.message).startsWith("Field absent"))throw error;}if(reference===undefined)continue;if(typeof reference!=="string"||!reference||isAbsolute(reference))throw Error("Relative Cargo execution declaration required");
   const declarationPath=resolve(dirname(manifest),reference);local(declarationPath);requirePhysicalSourceV1(manifest);const declaration=readCargoExecutionContractV1("declaration",JSON.parse(readPhysicalSource(declarationPath).toString("utf8")));entries.add(manifest);entries.add(declarationPath);
   const selectedOperation=selectCargoExecutionOperationV1(declaration,{program:selected.program,cwd:owner?.workingDirectory??options.cwd??".",arguments:selected.arguments},workspaceRoot);if(!selectedOperation?.continuation)continue;
   const child=selectedOperation.continuation,childBound=bindBuildCommandV1([child.program,...child.arguments].map(value=>JSON.stringify(value)).join(" "),workspaceRoot,child.cwd,projectRoot,[]);if(!childBound.script)throw Error("Declared Cargo continuation requires an explicit script Source");retain(childBound);
  }
 }
 const commands=new Map([...words].filter(([,known])=>known).map(([script,known])=>[script,[...known]])),baseSize=entries.size,base=baseSize?await boundedScriptInputsV1([...entries],workspaceRoot,scripts,true,resources,commands):undefined;
 if(buildInputSourceRoleV1(true,false,[target.metadata?.semio?.sourceInputObservation])==="compile")for(const declared of declaredModuleInputsV1(target.inputs,workspaceRoot,projectRoot,scripts,base&&scripts.compiledNames.get(base)))entries.add(declared);
 for(const declared of declaredModuleGlobEntriesV1(target.metadata?.semio?.closure?.modules,workspaceRoot,projectRoot,scripts))entries.add(declared);
 for(const input of readBuildRuntimeSourceEntriesV1(target.metadata?.semio?.runtimeSourceEntries)){const path=resolve(workspaceRoot,input.replace("{projectRoot}",projectRoot).replace("{workspaceRoot}","."));if(path.length>REVISION_POLICY_V1.maximumPathUnits)throw Error("Bounded physical runtime Source entry required");const entry=declaredModuleInputV1(input,workspaceRoot,projectRoot);if(!entry)throw Error("Bounded physical runtime Source entry required");entries.add(entry);}
 let closure=entries.size===baseSize?base:entries.size?await boundedScriptInputsV1([...entries],workspaceRoot,scripts,true,resources,commands):undefined,generatedDependencies=scripts.generatedImports.size&&closure?generatedTargetsV1(scripts,closure,workspaceRoot):[];const walked=closure,project=closure?declaredProjectClosureV1(target.metadata?.semio?.closure?.typescript,workspaceRoot,projectRoot,scripts,closure):undefined;
 if(project){closure=project.closure;generatedDependencies=[...new Set([...(scripts.generatedImports.size?generatedTargetsV1(scripts,closure,workspaceRoot):[]),...project.generatedDependencies])].sort();}
 const references=walked?providerReferencesV1(scripts,scripts.walkPorts.get(walked)??[],"scheduling"):[],pruned=closure&&withoutSchedulingV1(closure),kept=pruned&&references.length?[...pruned,...references]:pruned;
 if(kept&&kept!==closure){const compiled=scripts.compiledNames.get(closure);if(compiled){const present=new Set(kept);scripts.compiledNames.set(kept,new Set([...compiled].filter(name=>present.has(name))));}}
 return {closure:kept,fallback,...(generatedDependencies.length?{generatedDependencies}:{}),...(project?.packages.length?{packages:project.packages,projectConfig:project.config}:{})};
}

/** ⚙️ Keeps configured recipe inheritance identical for command and lock source discovery. */
function targetCommandRecipes(target) {
  return [target.options ?? {}, ...Object.values(target.configurations ?? {}).map(options => ({ ...target.options, ...options }))];
}

/** 🧳️ Keeps configured executable imports in the target's invariant source closure. */
export async function targetScriptClosure(target, workspaceRoot, scripts, projectRoot = ".") {
  return (await targetScriptSources(target,workspaceRoot,scripts,projectRoot)).closure;
}

/** 🔐️ `cargo metadata --locked` validates the shared lock against every workspace manifest, so its replay must hash all of them. */
export function nativeLockInputs(command) {
  if(command && typeof command === "object")return [...new Set(targetCommandRecipes(command).flatMap(options=>(typeof options.command==="string"?[options.command]:options.commands??[]).flatMap(operation=>nativeLockInputs(typeof operation==="string"?operation:operation.command))))];
  return typeof command === "string" && command.includes(" native cargo metadata ") ? ["{workspaceRoot}/**/Cargo.toml", "{workspaceRoot}/**/Cargo.lock"] : [];
}

/** ⚖️ Adds the selected native target's local router and executable import closure. */
export async function nativeTargetCommandInputs(target, workspaceRoot, fallback, scripts = createScriptInputCache(), projectRoot = ".") {
  fallback??=await nativeCommandInputs(workspaceRoot,scripts);
  const closure = await targetScriptClosure(target, workspaceRoot, scripts, projectRoot);
  return closure ? joinWithProviderViewV1([closure, fallback], "native") : fallback;
}

/** 🎯️ Hashes any script-backed target's own command closure, exactly, instead of the whole router. */
export async function genericTargetCommandInputs(target, workspaceRoot, fallback, scripts = createScriptInputCache(), projectRoot = ".") {
  const sources = await targetScriptSources(target, workspaceRoot, scripts, projectRoot),closure=sources.closure;
  if (!closure) return [...fallback, "{workspaceRoot}/package.json", `{workspaceRoot}/${javascriptManagerV1.primaryLockName}`];
  const key=memoDigestV1(closure,sources.fallback?fallback:null,sources.packages??[]);
  let index=0;const checkpoint=(force=false)=>yieldInferenceV1(scripts.context,"build:command-source-packages",++index,force);
  return memoizedObservedAsyncComputationV1(scripts.commandDependencies,key,async()=>{
  const graph = existsSync(join(workspaceRoot, javascriptManagerV1.primaryLockName)) ? readBunLockGraph(workspaceRoot) : undefined, packages = await closurePackagesV1(closure, workspaceRoot, graph, checkpoint);
  for (const name of sources.packages ?? []) {
    if (!graph) { packages.add(name); continue; }
    const key = graph.resolveImport(sources.projectConfig.slice("{workspaceRoot}/".length), name);
    if (key && graph.externalNodes[`npm:${key}`]) packages.add(`npm:${key}`);
  }
  const inputs = [...new Set([...closure,...(sources.fallback?[...fallback,"{workspaceRoot}/package.json",`{workspaceRoot}/${javascriptManagerV1.primaryLockName}`]:[])]), { externalDependencies: [...packages].sort() }];
  return inputs;
  },checkpoint);
}

/** 🧱️ The external packages the modules of a closure's files import, as lock nodes when the workspace has a lock graph; references and other non-file items name no module. */
async function closurePackagesV1(closure, workspaceRoot, graph, checkpoint) {
  const packages = new Set();
  for (const input of closure) {
    if (typeof input !== "string") continue;
    await checkpoint();
    const file = resolve(workspaceRoot, input.slice("{workspaceRoot}/".length));
    if (!existsSync(file)) continue;
    for (const specifier of observedSourceAnalysisV1(prepareSourceAnalysesV1().module,file).imports) {
      if (specifier.startsWith(".") || specifier.startsWith("/") || isBuiltin(specifier) || isRuntimeBuiltinSpecifierV1(specifier)) continue;
      const name = specifier.startsWith("@") ? specifier.split("/").slice(0, 2).join("/") : specifier.split("/")[0];
      if (!graph) { packages.add(name); continue; }
      const key = graph.resolveImport(buildPath(relative(workspaceRoot, file)), name);
      if (key && graph.externalNodes[`npm:${key}`]) packages.add(`npm:${key}`);
    }
  }
  return packages;
}

/** 🎭️ A `closure-of` list: read through the view its declared role names (`./👓️walk-views`), the bounded walk without that view's prefixes and its providers by reference; a role read whole gets the full closure (`relativeScriptInputs`). */
async function closureOfV1(entries, workspaceRoot, scripts, extras, role) {
  const view = closureRoleViewV1(role);
  if (view === undefined) return [...await relativeScriptInputs(entries, workspaceRoot, scripts), ...extras];
  const walk = await boundedScriptInputsV1(entries, workspaceRoot, scripts);
  return [...withoutSchedulingV1(walk, walkViewPrefixesV1(view)), ...providerReferencesV1(scripts, scripts.walkPorts.get(walk) ?? [], view), ...extras];
}

/** 🛰️ The named input group of one provider closure read through `view` (`./🔌️provider-closures`): the designation, its schema and the bounded walks of the port's module and of every port they name in turn, without the view's prefixes, and for a view that carries packages the external packages their modules import. A provider closure that imports a generated output is refused: the projects that name it could not depend on its producer. */
async function scriptProviderGroupV1(workspaceRoot, scripts, port, view) {
  const designation = providerDesignationV1(scripts);
  if (!Object.hasOwn(designation.providers, port)) throw refuseBuildV1("invalid", "closure.provider-port-unknown", { port: String(port) });
  const files = new Set();
  for (const walk of await providerWalksV1(scripts, workspaceRoot, [port], designation)) {
    const generated = scripts.generatedImports.size ? generatedTargetsV1(scripts, walk, workspaceRoot) : [];
    if (generated.length) throw refuseBuildV1("invalid", "closure.provider-generated-import", { port: String(port), targets: generated.join(", ") });
    for (const name of withoutSchedulingV1(walk, walkViewPrefixesV1(view))) files.add(name);
  }
  let index = 0;
  const roster = [closurePathNameV1(scripts, workspaceRoot, designation.path), closurePathNameV1(scripts, workspaceRoot, designation.schema), ...[...files].sort()];
  if (!PROVIDER_CLOSURE_POLICY_V1.views[view].packages) return roster;
  const graph = existsSync(join(workspaceRoot, javascriptManagerV1.primaryLockName)) ? readBunLockGraph(workspaceRoot) : undefined;
  return [...roster, { externalDependencies: [...await closurePackagesV1(roster, workspaceRoot, graph, (force = false) => yieldInferenceV1(scripts.context, "build:command-source-packages", ++index, force))].sort() }];
}

/** 🪡️ The named input group of one declared tool command (`./🔌️provider-closures`): the bounded walk of its entry modules (relative to the providers project) and the bounded walks of every provider port they name, transitively, without the tool's view's prefixes, the designation and its schema when it names a port, and its resources. A tool closure that imports a generated output is refused, like a provider closure. */
async function toolClosureGroupV1(workspaceRoot, scripts, name) {
  const tool = PROVIDER_CLOSURE_POLICY_V1.tools.closures[name];
  if (!tool) throw refuseBuildV1("invalid", "closure.tool-unknown", { name: String(name) });
  const designation = providerDesignationV1(scripts), base = dirname(designation.path), walk = await boundedScriptInputsV1(tool.entries.map(entry => resolve(base, entry)), workspaceRoot, scripts), ports = scripts.walkPorts.get(walk) ?? [], files = new Set();
  for (const each of [walk, ...await providerWalksV1(scripts, workspaceRoot, ports, designation)]) {
    const generated = scripts.generatedImports.size ? generatedTargetsV1(scripts, each, workspaceRoot) : [];
    if (generated.length) throw refuseBuildV1("invalid", "closure.provider-generated-import", { port: String(name), targets: generated.join(", ") });
    for (const file of withoutSchedulingV1(each, walkViewPrefixesV1(tool.view))) files.add(file);
  }
  for (const resource of tool.resources) { const path = resolve(base, resource); requirePhysicalSourceV1(path); files.add(closurePathNameV1(scripts, workspaceRoot, path)); }
  if (ports.length) for (const path of [designation.path, designation.schema]) files.add(closurePathNameV1(scripts, workspaceRoot, path));
  return [...files].sort();
}

/** 🌀️ The previous blanket router contract, kept only as a correctness fallback for a command naming no script. */
export function genericCommandFallbackInputs(workspaceRoot) {
  const runner = buildPath(relative(workspaceRoot, LIBRARY_ROOT));
  return [
    "{workspaceRoot}/📜️script.ts",
    `{workspaceRoot}/${runner}/**/*.{ts,tsx,js,mjs,cjs,json}`,
    `{workspaceRoot}/${runner}/🟨️.mjs`,
    `{workspaceRoot}/${runner}/⚡️caching/**/*`,
  ];
}

const sortedJsonV1 = value => Array.isArray(value) ? value.map(sortedJsonV1) : value !== null && typeof value === "object" ? Object.fromEntries(Object.keys(value).filter(key => value[key] !== undefined).sort().map(key => [key, sortedJsonV1(value[key])])) : value;
const closureHandleSymbolV1 = Symbol.for("semio.closure.handle"), knownClosureRecipesV1 = createBoundedCacheV1({ name: "inference:closure-recipes", maxEntries: 262144, maxBytes: 67108864, sizeOf: recipe => 128 + JSON.stringify(recipe).length });

/** 📋️ The recipe a symbolic list was made from, by its key. */
export function closureRecipeByKeyV1(key) { const recipe = knownClosureRecipesV1.get(key); if (!recipe) throw refuseBuildV1("invalid", "closure.key-unknown", { key }); return recipe; }

/** 🌁️ The identity of one command closure recipe: the kind of command input and the arguments that determine it, never the files it names. */
export function closureRecipeKeyV1(recipe) { return createHash("sha256").update(JSON.stringify(sortedJsonV1(recipe))).digest("hex"); }

/** 🧬️ A symbolic command input list: it stands, as one `{closureRef}` item, for the whole list its recipe produces, so a project record names the closure instead of its files; every other way of reading it throws, because a project that read the files would depend on them and must ask a query that records its answer. */
function closureHandleV1(recipe) {
  const key = closureRecipeKeyV1(recipe), marker = Object.freeze({ closureRef: key }), identity = Object.freeze({ recipe, key, marker });
  knownClosureRecipesV1.set(key, recipe);
  return new Proxy([], {
    get(_target, name) {
      if (name === closureHandleSymbolV1) return identity;
      if (name === Symbol.iterator) return function* () { yield marker; };
      if (name === "then" || name === "toJSON" || typeof name === "symbol") return undefined;
      throw refuseBuildV1("invalid", "closure.read-through-queries-only", { name });
    },
  });
}

/** 🧷️ The symbolic list of a recipe (a worker process rebuilds the lists its parent describes by recipe). */
export function closureHandleFromRecipeV1(recipe) { return closureHandleV1(recipe); }

/** 🧺️ The identity (recipe, key, marker) of a symbolic command input list, or undefined for an ordinary list. */
export function closureHandleOfV1(value) { return value !== null && typeof value === "object" ? value[closureHandleSymbolV1] : undefined; }

const CLOSURE_QUERIES_V1 = new Map([
  ["some", (value, substrings) => (Array.isArray(value) ? value : value?.closure ?? []).some(path => typeof path === "string" && substrings.some(part => path.includes(part)))],
  ["generated", value => value?.generatedDependencies ?? []],
  ["digest", derivedWalkAnswerV1],
]);

/** 🌂️ Registers a pure query over the closure a recipe produces (the inferrer that owns the question, such as the whole-workspace key check, registers it). */
export function registerClosureQueryV1(name, query) { CLOSURE_QUERIES_V1.set(name, query); }

const closureEntryCacheV1 = maxBytes => createBoundedCacheV1({ name: "inference:closure-entries", maxEntries: INFERENCE_LIMITS_V1.closureEntryCacheEntries, maxBytes, sizeOf: entry => INFERENCE_LIMITS_V1.closureEntryBaseBytes + INFERENCE_LIMITS_V1.closureEntryRowBytes * entry.rows.length + 2 * (entry.valueBytes ??= JSON.stringify(entry.value).length) });
let closureEntriesV1 = closureEntryCacheV1(INFERENCE_LIMITS_V1.closureEntryCacheBytes);
const closureScriptsV1 = createBoundedCacheV1({ name: "inference:closure-scripts", maxEntries: 16, maxBytes: 16 }),
  closureStoresV1 = createBoundedCacheV1({ name: "inference:closure-stores", maxEntries: 16, maxBytes: 16 }),
  closureInflightV1 = createBoundedCacheV1({ name: "inference:closure-inflight", maxEntries: 4096, maxBytes: 4096 }),
  closureFailuresV1 = createBoundedCacheV1({ name: "inference:closure-failures", maxEntries: 1024, maxBytes: 1024 }),
  knownClosureInputsV1 = createBoundedCacheV1({ name: "inference:closure-inputs", maxEntries: 65536, maxBytes: 67108864, sizeOf: inputs => 128 + JSON.stringify(inputs).length });

const closureCountsV1 = { derived: 0, loaded: 0, stale: 0, checked: 0 };
/** 🔢️ How many closure entries this process derived, loaded from the durable store, found stale there and verified row by row (once per discovery pass, however many owners ask) since the last reset. */
export function closureEntryCountsV1() { return { ...closureCountsV1 }; }

const closureEntryBytesV1 = entry => JSON.stringify([entry.value, entry.rows]).length + 8 * entry.layers.length;
const closureSidecarV1 = layers => ({ facts: layers.map(layer => walkFactsV1.get(layer) ?? null), members: layers.map(layer => { const walks = walkMembersV1.get(layer); return walks ? [...walks] : null; }) });
const closureLayersV1 = entry => {
  if (entry.layers) return entry.layers;
  const layers = entry.store.layers(entry.digests);
  if (layers === undefined) throw refuseBuildV1("invalid", "closure.provider-entry-stale", { port: "layers" });
  layers.forEach((layer, at) => {
    const fact = entry.sidecar?.facts[at], members = entry.sidecar?.members[at];
    if (fact && !walkFactsV1.has(layer)) walkFactsV1.set(layer, fact);
    if (members) { let walks = walkMembersV1.get(layer); if (!walks) walkMembersV1.set(layer, walks = new Set()); for (const walk of members) walks.add(walk); }
  });
  return layers;
};
const closureStoreBindingsV1 = new Map();
/** 📁️ The directory of the durable closure entries of a workspace: the graph's inference data directory of the running writer, or undefined where no graph writer is running (the entries then live in this process only). */
export function closureStoreDirectoryV1(workspaceRoot) { try { return join(workspaceDataDirectory(workspaceRoot), "ce"); } catch { return undefined; } }
/** 🔗️ Binds the directory a worker process reads the closure entries of its parent from; a worker never writes there, its parent persists the entries it derives itself. */
export function bindClosureStoreV1(workspaceRoot, directory, writable = false) { closureStoreBindingsV1.set(workspaceRoot, { directory, writable }); closureStoresV1.delete(workspaceRoot); }

const closureStoreOfV1 = workspaceRoot => {
  let store = closureStoresV1.get(workspaceRoot);
  if (store !== undefined) return store;
  const bound = closureStoreBindingsV1.get(workspaceRoot), directory = bound?.directory ?? closureStoreDirectoryV1(workspaceRoot);
  store = directory === undefined ? null : createClosureStoreV1({ directory, readOnly: bound !== undefined && !bound.writable });
  closureStoresV1.set(workspaceRoot, store);
  return store;
};

/** 🧊️ Binds the script input cache that computes the closure entries of one workspace (the inference binds its own, a verifier binds one of its own). */
export function bindClosureScriptsV1(workspaceRoot, scripts) { closureScriptsV1.set(workspaceRoot, scripts); }

/** 🧽️ Forgets every in-process entry, as a process that starts later has none; `entryBytes` sets the byte budget of the in-process entries for the new process (the declared limit by default). */
export function resetClosureEntriesV1(entryBytes = INFERENCE_LIMITS_V1.closureEntryCacheBytes) { closureScriptsV1.clear(); closureEntriesV1.clear(); closureEntriesV1 = closureEntryCacheV1(entryBytes); closureInflightV1.clear(); closureFailuresV1.clear(); knownClosureRecipesV1.clear(); knownClosureInputsV1.clear(); closureStoresV1.clear(); closureCountsV1.derived = closureCountsV1.loaded = closureCountsV1.stale = closureCountsV1.checked = 0; }

/** 🌃️ Produces what one recipe stands for, with the same functions the direct inference calls. */
async function produceClosureRecipeV1(recipe, workspaceRoot, scripts) {
  const target = recipe.target === undefined ? undefined : closureTargetOfV1(recipe.target);
  switch (recipe.fn) {
    case "native-command": return nativeCommandInputs(workspaceRoot, scripts);
    case "native-target": return nativeTargetCommandInputs(target, workspaceRoot, recipe.fallback ? await produceClosureRecipeV1(recipe.fallback, workspaceRoot, scripts) : undefined, scripts, recipe.projectRoot);
    case "generic-target": return genericTargetCommandInputs(target, workspaceRoot, genericCommandFallbackInputs(workspaceRoot), scripts, recipe.projectRoot);
    case "closure-of": return closureOfV1(recipe.entries.map(entry => resolve(workspaceRoot, entry)), workspaceRoot, scripts, recipe.extras, recipe.role);
    case "script-provider": return scriptProviderGroupV1(workspaceRoot, scripts, recipe.port, recipe.view);
    case "tool-closure": return toolClosureGroupV1(workspaceRoot, scripts, recipe.name);
    case "target-sources": return targetScriptSources(target, workspaceRoot, scripts, recipe.projectRoot, recipe.nativeManifest);
    case "crate-index": case "source-roots": return derivedWalkV1(workspaceRoot, readDerivedWalkV1(recipe), scripts);
    default: throw refuseBuildV1("invalid", "closure.recipe-unknown", { recipe: String(recipe.fn) });
  }
}

let closurePassV1;
/** 🧭️ Runs one inference pass during which a shared closure entry is verified once, however many projects ask for it: the files do not change inside a pass, and a pass that nests another shares its token. */
export async function withClosurePassV1(compute) {
  const outer = closurePassV1;
  closurePassV1 = outer ?? {};
  try { return await compute(); } finally { closurePassV1 = outer; }
}

/** 🫧️ One cooperative step of a long loop: every `yieldMilliseconds` it gives the event loop one turn, reports through the inference checkpoint (`phase`) and stops at cancellation. */
function cooperativeStepV1(scripts, phase) {
  let since = -Infinity, index = 0;
  const context = scripts?.context ?? {}, signal = context.invocation?.control.signal ?? context.phaseControl?.signal;
  return async () => {
    if (performance.now() - since < OBSERVATION_LIMITS_V1.yieldMilliseconds) return;
    since = performance.now();
    await new Promise(resolve => setImmediate(resolve));
    await yieldInferenceV1(context, phase, ++index);
    signal?.throwIfAborted();
  };
}

/** 🫧️ The verdict of {@link closureEntryHoldsV1}, checked row by row with cooperative steps and remembered for the round, so a stored entry of tens of thousands of rows never blocks the event loop. */
async function closureEntryHoldsAsyncV1(entry, scripts) {
  const round = closurePassV1 ?? observationRoundV1();
  if (round && entry.checked?.round === round) return entry.checked.holds;
  closureCountsV1.checked++;
  const step = cooperativeStepV1(scripts, "build:closure-entry-rows"), layers = entry.layers ?? entry.store.layers(entry.digests);
  let holds = layers !== undefined && await observationRowsHoldAsyncV1(entry.rows, step);
  if (holds) for (const layer of layers) if (!await observationLayerHoldsAsyncV1(layer, step)) { holds = false; break; }
  if (round) entry.checked = { round, holds };
  return holds;
}

const closureEntryHoldsV1 = entry => {
  const round = closurePassV1 ?? observationRoundV1();
  if (round && entry.checked?.round === round) return entry.checked.holds;
  const layers = entry.layers ?? entry.store.layers(entry.digests), holds = layers !== undefined && observationRowsHoldV1(entry.rows) && layers.every(observationLayerHoldsV1);
  if (round) entry.checked = { round, holds };
  return holds;
};

/** 🗃️ The entry of one recipe: computed once with its observations detached from the project that asked, kept while those observations hold, shared by every project and every verification. A recipe whose sources do not parse fails the same way for every project while the facts it read hold; each asking project records those facts, because they are the reason it is withheld and the only thing that can lift it. */
async function ensureClosureEntryV1(workspaceRoot, recipe) {
  try { return await produceClosureEntryV1(workspaceRoot, recipe); }
  catch (error) { retainDetachedFailureV1(error); throw error; }
}

/** ♻️ The entry of one recipe from the cache, a flight in progress, or a new detached computation; an unparseable source is remembered as a failure for as long as the facts it rests on hold. */
async function produceClosureEntryV1(workspaceRoot, recipe) {
  const id = workspaceRoot + "\0" + closureRecipeKeyV1(recipe), known = closureEntriesV1.get(id);
  if (known && await closureEntryHoldsAsyncV1(known, closureScriptsV1.get(workspaceRoot))) return known;
  const failed = closureFailuresV1.get(id);
  if (failed) { if (await closureEntryHoldsAsyncV1(failed, closureScriptsV1.get(workspaceRoot))) throw failed.error; closureFailuresV1.delete(id); }
  let flight = closureInflightV1.get(id);
  if (!flight) {
    const scripts = closureScriptsV1.get(workspaceRoot);
    if (!scripts) throw refuseBuildV1("missing", "closure.scripts-unbound", { workspace: String(workspaceRoot) });
    const store = closureStoreOfV1(workspaceRoot), derive = () => observeDetachedV1(() => produceClosureRecipeV1(recipe, workspaceRoot, scripts)).then(async ({ value, rows, layers }) => { shareObservationRowsV1(rows); for (const layer of layers) shareObservationRowsV1(layer); closureCountsV1.derived++; const pass = closurePassV1, entry = { recipe, value, rows, layers, answers: new Map(), checked: pass ? { round: pass, holds: true } : null, bytes: 0, store, digests: undefined }; try { const saved = store ? await store.save(id, { ...entry, sidecar: closureSidecarV1(layers) }) : undefined; entry.bytes = saved?.bytes ?? closureEntryBytesV1(entry); entry.digests = saved?.digests; } catch { entry.bytes = closureEntryBytesV1(entry); } closureEntriesV1.set(id, entry); return entry; });
    flight = (store ? store.load(id) : Promise.resolve(undefined)).then(async stored => { const entry = stored && { ...stored, layers: undefined, store, answers: new Map(), checked: null }; if (entry && await closureEntryHoldsAsyncV1(entry, scripts)) { closureCountsV1.loaded++; closureEntriesV1.set(id, entry); return entry; } if (entry) closureCountsV1.stale++; return derive(); }).then(entry => entry, error => { const facts = error?.code === "BUILD_SOURCE_UNPARSEABLE" ? detachedFailureV1(error) : undefined; if (facts) closureFailuresV1.set(id, { error, rows: facts.rows, layers: facts.layers, checked: null }); throw error; }).finally(() => closureInflightV1.delete(id));
    closureInflightV1.set(id, flight);
  }
  return flight;
}

/** 🫧️ Releases what one finished owner left behind in the cache shared by the owners: the source reads, closure walks and command dependency memos of the call and half of the in-process closure entries, which a later owner reads again from the durable store; the shared provider designation and the generated imports stay. */
export function releaseOwnerStateV1(scripts) {
  scripts.files.clear(); scripts.closures.clear(); scripts.commandDependencies.clear(); scripts.walkNodes?.clear();
  closureEntriesV1.shrink(0.5);
}

/** 📋️ The recipes every owner reads and none depends on its own target: the native command inputs, the group of every provider port in every view and every tool closure. */
export function sharedClosureRecipesV1(scripts) {
  const designation = providerDesignationV1(scripts), recipes = [{ fn: "native-command" }];
  for (const port of Object.keys(designation.providers).sort()) for (const view of Object.keys(PROVIDER_CLOSURE_POLICY_V1.views)) recipes.push({ fn: "script-provider", port, view });
  for (const name of Object.keys(PROVIDER_CLOSURE_POLICY_V1.tools.closures).sort()) recipes.push({ fn: "tool-closure", name, view: PROVIDER_CLOSURE_POLICY_V1.tools.closures[name].view });
  return recipes;
}

/** 🧰️ Derives the shared closure entries once, one after the other, before the owners start: each is a durable memo node keyed by its recipe, so an owner only reads it and a process that finds it again derives nothing. Progress and cancellation follow the caller. */
export async function primeSharedClosuresV1(workspaceRoot, scripts, { signal, progress, recipes = sharedClosureRecipesV1(scripts) } = {}) {
  bindClosureScriptsV1(workspaceRoot, scripts);
  const before = closureEntryCountsV1();
  let done = 0;
  await progress?.({ done, total: recipes.length });
  for (const recipe of recipes) {
    signal?.throwIfAborted();
    knownClosureRecipesV1.set(closureRecipeKeyV1(recipe), recipe);
    await ensureProviderEntriesV1(workspaceRoot, await ensureClosureEntryV1(workspaceRoot, recipe));
    await progress?.({ done: ++done, total: recipes.length });
  }
  const after = closureEntryCountsV1();
  return { recipes: recipes.length, derived: after.derived - before.derived, loaded: after.loaded - before.loaded, stale: after.stale - before.stale };
}

/** 🚚️ Brings the shared entries of every reference an entry's value holds up to date, detached: the asking project holds only the answers its queries record. */
async function ensureProviderEntriesV1(workspaceRoot, entry) { const providers = new Map(); for (const reference of sharedReferencesOfV1(entry.value)) { const key = closureRecipeKeyV1(reference.recipe); providers.set(key, await ensureClosureEntryV1(workspaceRoot, reference.recipe)); } return providers; }

/** 🏺️ An entry's value with its shared references expanded into the files of the shared closures, or undefined while a shared entry is missing or no longer holds. */
function expandedEntryValueV1(workspaceRoot, entry, providers) {
  const references = sharedReferencesOfV1(entry.value);
  if (!references.length) return entry.value;
  const files = [];
  for (const reference of references) {
    const key = closureRecipeKeyV1(reference.recipe), provider = providers?.get(key) ?? closureEntriesV1.get(workspaceRoot + "\0" + key);
    if (!provider || !closureEntryHoldsV1(provider)) return undefined;
    for (const item of provider.value) if (typeof item === "string" && item.startsWith("{workspaceRoot}/")) files.push(item);
  }
  return expandClosureValueV1(entry.value, files);
}

/** ❔️ One query answered over the closure an entry stands for, provider closures expanded; undefined while a provider entry does not hold. An entry without provider references keeps its answers. */
const closureAnswerV1 = (workspaceRoot, entry, query, providers) => {
  const key = JSON.stringify(query), stable = !sharedReferencesOfV1(entry.value).length;
  if (stable && entry.answers.has(key)) return entry.answers.get(key);
  const run = CLOSURE_QUERIES_V1.get(query[0]);
  if (!run) throw refuseBuildV1("invalid", "closure.query-unknown", { query: String(query[0]) });
  const value = expandedEntryValueV1(workspaceRoot, entry, providers);
  if (value === undefined) return undefined;
  const answer = run(value, ...query.slice(1));
  if (stable) entry.answers.set(key, answer);
  return answer;
};

registerClosureProviderV1({
  answer(path) {
    let workspaceRoot, key, query;
    try { [workspaceRoot, key, query] = JSON.parse(path); } catch { return "!unreadable"; }
    const entry = closureEntriesV1.get(workspaceRoot + "\0" + key);
    if (!entry || !closureEntryHoldsV1(entry)) return "!stale";
    const answer = closureAnswerV1(workspaceRoot, entry, query);
    return answer === undefined ? "!stale" : JSON.stringify(answer);
  },
  async settle(paths, value) {
    for (const [key, recipe] of Object.entries(value?.closureRecipes ?? {})) knownClosureRecipesV1.set(key, recipe);
    for (const [digest, inputs] of Object.entries(value?.closureInputs ?? {})) knownClosureInputsV1.set(digest, inputs);
    const wanted = new Map();
    for (const path of paths) { try { const [workspaceRoot, key] = JSON.parse(path), recipe = knownClosureRecipesV1.get(key); if (recipe) wanted.set(workspaceRoot + "\0" + key, [workspaceRoot, recipe, key]); } catch {} }
    for (const [workspaceRoot, recipe, key] of wanted.values()) { try { await ensureProviderEntriesV1(workspaceRoot, await ensureClosureEntryV1(workspaceRoot, recipe)); } catch { closureEntriesV1.delete(workspaceRoot + "\0" + key); } }
  },
});

/** 📑️ The command inputs of one inference: in the late mode symbolic lists whose queries are recorded as facts, otherwise the lists themselves; project assembly reads both through this one interface. */
export function closureApiV1(scripts, workspaceRoot) {
  const late = scripts.lateClosures === true;
  if (late) bindClosureScriptsV1(workspaceRoot, scripts);
  const used = new Map(), fact = async (recipe, query) => { const key = closureRecipeKeyV1(recipe); knownClosureRecipesV1.set(key, recipe); used.set(key, recipe); const entry = await ensureClosureEntryV1(workspaceRoot, recipe), providers = await ensureProviderEntriesV1(workspaceRoot, entry), answer = closureAnswerV1(workspaceRoot, entry, query, providers); if (answer === undefined) throw refuseBuildV1("invalid", "closure.provider-entry-stale", { port: sharedReferencesOfV1(entry.value).map(row => row.recipe.port ?? row.recipe.name).join(", ") }); recordClosureFactV1(JSON.stringify([workspaceRoot, key, query]), JSON.stringify(answer)); return answer; };
  const expanded = value => expandedCommandInputsV1(value, workspaceRoot, scripts);
  const relativeEntries = entries => entries.map(entry => relative(workspaceRoot, entry).split(sep).join("/"));
  return Object.freeze({
    late,
    async nativeCommandInputs() { return late ? closureHandleV1({ fn: "native-command" }) : nativeCommandInputs(workspaceRoot, scripts); },
    async nativeTargetCommandInputs(target, fallback, projectRoot) {
      if (!late) return nativeTargetCommandInputs(target, workspaceRoot, fallback, scripts, projectRoot);
      const base = fallback === undefined ? undefined : closureHandleOfV1(fallback);
      if (fallback !== undefined && !base) throw refuseBuildV1("invalid", "closure.symbolic-fallback-required");
      return closureHandleV1({ fn: "native-target", target: closureTargetV1(target), projectRoot, fallback: base?.recipe ?? null });
    },
    async genericTargetCommandInputs(target, projectRoot) { return late ? closureHandleV1({ fn: "generic-target", target: closureTargetV1(target), projectRoot }) : genericTargetCommandInputs(target, workspaceRoot, genericCommandFallbackInputs(workspaceRoot), scripts, projectRoot); },
    async closureOf(entries, extras, role) { closureRoleViewV1(role); return late ? closureHandleV1({ fn: "closure-of", entries: relativeEntries(entries), extras, role }) : closureOfV1(entries, workspaceRoot, scripts, extras, role); },
    async toolClosure(name) { return toolReferencesV1(scripts, name); },
    async sharedClosureGroups(projectRoot, declared) {
      const designation = providerDesignationV1(scripts);
      if (resolve(workspaceRoot, projectRoot, declared) !== resolve(designation.path)) throw refuseBuildV1("invalid", "closure.provider-designation-differs", { designation: String(declared) });
      const groups = [];
      for (const port of Object.keys(designation.providers).sort()) for (const view of Object.keys(PROVIDER_CLOSURE_POLICY_V1.views)) groups.push([providerGroupNameV1(port, view), late ? closureHandleV1({ fn: "script-provider", port, view }) : await scriptProviderGroupV1(workspaceRoot, scripts, port, view)]);
      for (const name of Object.keys(PROVIDER_CLOSURE_POLICY_V1.tools.closures).sort()) groups.push([toolGroupNameV1(name), late ? closureHandleV1({ fn: "tool-closure", name, view: PROVIDER_CLOSURE_POLICY_V1.tools.closures[name].view }) : await toolClosureGroupV1(workspaceRoot, scripts, name)]);
      return groups;
    },
    async derive(recipe) {
      if (!late) return derivedWalkV1(workspaceRoot, recipe, scripts);
      const key = closureRecipeKeyV1(recipe);
      knownClosureRecipesV1.set(key, recipe); used.set(key, recipe);
      const entry = await ensureClosureEntryV1(workspaceRoot, recipe);
      recordClosureFactV1(JSON.stringify([workspaceRoot, key, ["digest"]]), JSON.stringify(closureAnswerV1(workspaceRoot, entry, ["digest"], await ensureProviderEntriesV1(workspaceRoot, entry))));
      return entry.value;
    },
    async some(list, substrings) {
      const handle = closureHandleOfV1(list);
      return handle ? fact(handle.recipe, ["some", substrings]) : (await expanded(list)).some(path => typeof path === "string" && substrings.some(part => path.includes(part)));
    },
    async query(list, name, ...args) {
      const handle = closureHandleOfV1(list);
      return handle ? fact(handle.recipe, [name, ...args]) : CLOSURE_QUERIES_V1.get(name)(await expanded(list), ...args);
    },
    async targetClosureSome(target, projectRoot, substrings) {
      return late ? fact({ fn: "target-sources", target: closureTargetV1(target), projectRoot }, ["some", substrings]) : (await expanded((await targetScriptClosure(target, workspaceRoot, scripts, projectRoot)) ?? [])).some(path => typeof path === "string" && substrings.some(part => path.includes(part)));
    },
    async generatedDependencies(target, projectRoot, nativeManifest) {
      return late ? fact({ fn: "target-sources", target: closureTargetV1(target), projectRoot, ...(nativeManifest === undefined ? {} : { nativeManifest }) }, ["generated"]) : (await targetScriptSources(target, workspaceRoot, scripts, projectRoot, nativeManifest)).generatedDependencies ?? [];
    },
    table(keys) {
      const closureRecipes = {}, closureInputs = {};
      for (const key of [...new Set([...keys, ...used.keys()])].sort()) { const recipe = closureRecipeByKeyV1(key); closureRecipes[key] = recipe; for (const digest of [recipe.target?.inputsDigest, recipe.fallback?.target?.inputsDigest]) if (digest) closureInputs[digest] = knownClosureInputsV1.get(digest); }
      return { closureRecipes, closureInputs };
    },
    async resolve(recipes) {
      const rows = [], seen = new Set(), layers = new Set(), values = new Map(), derived = new Map(), walked = new Set(), take = row => { const key = rowKeyV1(row); if (!seen.has(key)) { seen.add(key); rows.push(row); } };
      for (const [key, recipe] of recipes) { const entry = await ensureClosureEntryV1(workspaceRoot, recipe), view = walkViewOfRecipeV1(recipe); values.set(key, entry.value); entry.rows.forEach(take); for (const layer of closureLayersV1(entry)) { layers.add(layer); const fact = walkFactsV1.get(layer); if (fact) { const row = view === undefined ? fact.row : fact.views?.[view] ?? fact.row; derived.set(row.path, { row, support: layer }); walked.add(fact.row.path); } } }
      for (const layer of layers) { if (walkFactsV1.has(layer)) continue; const walks = walkMembersV1.get(layer); if (!walks || ![...walks].some(question => walked.has(question))) layer.forEach(take); }
      return { values, rows, derived: [...derived.values()] };
    },
  });
}

/** 📇️ A command input list (or a target-sources value) with every shared reference (a script provider or tool closure of the providers project, `./🔌️provider-closures`) replaced by the files of its group: what a consumer that needs the literal files of a command closure reads (a read oracle, a law, a query) instead of the references a node holds. */
export async function expandedCommandInputsV1(value, workspaceRoot, scripts = createScriptInputCache()) {
  const references = sharedReferencesOfV1(value);
  if (!references.length) return value;
  const files = [];
  for (const reference of references) for (const item of await produceClosureRecipeV1(reference.recipe, workspaceRoot, scripts)) if (typeof item === "string" && item.startsWith("{workspaceRoot}/")) files.push(item);
  return expandClosureValueV1(value, files);
}

/** 🔬️ The command walk of `entries` for a caller that is no project inference (the test discovery): the roster, the rows that support the walk apart from the walks it consists of, and the derived facts that stand for those walks, exactly as a project record holds a late closure. The walk is computed once per process, by the script input cache the project inference of this process bound for the workspace (`cache`, made with `lateClosures`, binds when none is), and shared with every project that asks the same question. */
export async function resolveCommandWalkV1(workspaceRoot, entries, cache) {
  if (cache.lateClosures !== true) throw refuseBuildV1("invalid", "closure.late-cache-required", { workspace: String(workspaceRoot) });
  const api = closureApiV1(closureScriptsV1.get(workspaceRoot) ?? cache, workspaceRoot), handle = closureHandleOfV1(await api.closureOf(entries, [], "test-discovery")), resolved = await api.resolve(new Map([[handle.key, handle.recipe]]));
  return { value: resolved.values.get(handle.key), rows: resolved.rows, derived: resolved.derived };
}

/** 🗳️ The fields of a target that decide its command closure; the recipe names nothing else of the target. */
function closureTargetV1(target) {
  const semio = target.metadata?.semio, modules = Array.isArray(target.inputs) ? target.inputs.filter(declaredModuleLiteralV1) : [];
  let inputsDigest;
  if (modules.length) { inputsDigest = createHash("sha256").update(JSON.stringify(modules)).digest("hex"); knownClosureInputsV1.set(inputsDigest, modules); }
  return sortedJsonV1({ options: target.options, configurations: target.configurations, inputsDigest, metadata: semio ? { semio: { closure: semio.closure, sourceInputObservation: semio.sourceInputObservation, runtimeSourceEntries: semio.runtimeSourceEntries } } : undefined });
}

/** 🌄️ The target a recipe stands for: the fields it kept and the literal module inputs its digest names. */
function closureTargetOfV1({ inputsDigest, ...rest }) {
  if (inputsDigest === undefined) return rest;
  const inputs = knownClosureInputsV1.get(inputsDigest);
  if (!inputs) throw refuseBuildV1("invalid", "closure.literal-inputs-unknown", { digest: String(inputsDigest) });
  return { ...rest, inputs };
}

/** ⏳️ Delegates bounded inference work to the caller's original progress and continuation control; durable progress is published at most every 200 ms so its cost never scales with the number of inferred facts. */
export async function yieldInferenceV1(context,phase,index,force=false){
 if(!context.invocation)return;
 const last=yieldInferenceV1.last??=new WeakMap(),now=performance.now();
 if(!force&&now-(last.get(context)??-Infinity)<INFERENCE_LIMITS_V1.advanceThrottleMilliseconds){context.invocation.control.signal.throwIfAborted();return;}
 last.set(context,now);
 requireIssuedResourceControlInvocationV1(context.phaseControl,context.invocation);checkScriptInvocation(context.invocation);checkResourceLeaseControl(context.phaseControl);await advanceScriptInvocation(context.invocation,phase,"running");checkResourceLeaseControl(context.phaseControl);
}
