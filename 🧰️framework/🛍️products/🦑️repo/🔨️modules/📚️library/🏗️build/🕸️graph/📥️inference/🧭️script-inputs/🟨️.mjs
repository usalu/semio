import {readTypeScriptPackageProducerV1,typeScriptPackagePublicDirectoryV1} from "./../../../../⚡️caching/📦️artifacts/🟦️typescript/🔗️binding/🏭️producer/🟦️.ts";
import{refuseBuildV1}from"../../../📖️texts/🟦️.ts";
import {buildInputSourceRoleV1,readBuildRuntimeSourceEntriesV1,readBuildSourceResourceEntriesV1} from "../../🚪️admission/🟦️.ts";
import {createRequire} from "node:module";
const javascriptManagerV1 = createRequire(import.meta.url)("../../../../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🟨️.cjs").selectedManagerV1("javascript");
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
import { memoizedObservedAsyncComputationV1, memoizedObservedComputationV1, observedSourceAnalysisV1, observedRequireResolve, observedTypeScriptHost, observedFs, observeDetachedV1, detachedFailureV1, retainDetachedFailureV1, observationLayerHoldsV1, observationRoundV1, observationRowsHoldV1, recordClosureFactV1, registerClosureProviderV1 } from "../../../../🏗️build/👁️observation/🟨️.mjs";
import { readCargoExecutionContractV1, selectCargoExecutionOperationV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️execution/🟦️.ts";
import { REVISION_POLICY_V1 } from "../../../📥️inputs/🔏️revision/🟨️.mjs";
import { INFERENCE_LIMITS_V1 } from "../🧬️schema/🟨️.mjs";
import { validateJsonSchemaSubset } from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { GlobV1 } from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🌐️glob/🟦️.ts";
import { isRuntimeBuiltinSpecifierV1 } from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
import { routerCommandSpecifiersV1, routerCommandWordV1 } from "./🔀️router/🟨️.mjs";
import { cargoManifestDirectoriesV1, crateNamesV1, entryFilesV1, includeRootsV1, readDerivedWalkV1 } from "../../../📥️inputs/🎯️precision/🟦️.ts";
import { buildSchedulingPrefixesV1 } from "../../../📥️source/🔏️closure/🗓️scheduling/🟦️.ts";
import { WALK_VIEW_POLICY_V1, closureRoleViewV1, walkViewAnswerV1, walkViewOfRecipeV1, walkViewPrefixesV1 } from "./👓️walk-views/🟨️.mjs";
import { PROVIDER_CLOSURE_POLICY_V1, expandClosureValueV1, joinWithProviderViewV1, providerGroupNameV1, providerProjectNameV1, providerReferenceV1, sharedReferencesOfV1, toolGroupNameV1, toolReferenceV1 } from "./🔌️provider-closures/🟨️.mjs";
const { existsSync, statSync } = observedFs;

/** 🚧️ Turns an import of a command closure that resolves to nothing into the withheld-project failure of its importing source, so one broken file withholds its own project instead of aborting the whole graph. */
function unresolvedSourceImportV1(error,path){return Object.assign(Error(String(error.message),{cause:error}),{code:"BUILD_SOURCE_UNPARSEABLE",source:path,sha256:createHash("sha256").update(observedFs.readFileSync(path)).digest("hex")});}

/** 🏷️ Names this component as its roster entry does. */
export const component = "script-inputs";
let buildPath, readPhysicalSource, requirePhysicalSourceV1, prepareSourceAnalysesV1, revisionIdentity, register, PROVIDER_PORTS, LIBRARY_ROOT, POLICY, readToml, readBunLockGraph, SCRIPT_PROVIDER_DESIGNATION, SCRIPT_PROVIDER_DESIGNATION_SCHEMA;

/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */
export function wire(registry) {
  ({ buildPath, readPhysicalSource, requirePhysicalSourceV1, LIBRARY_ROOT, POLICY, SCRIPT_PROVIDER_DESIGNATION, SCRIPT_PROVIDER_DESIGNATION_SCHEMA } = registry.coordinates);
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
  const path = cache.providerDesignation ?? join(LIBRARY_ROOT, SCRIPT_PROVIDER_DESIGNATION), schema = join(dirname(path), relative(dirname(SCRIPT_PROVIDER_DESIGNATION), SCRIPT_PROVIDER_DESIGNATION_SCHEMA));
  const declared = JSON.parse(readPhysicalSource(path).toString("utf8"));
  if (validateJsonSchemaSubset(JSON.parse(readPhysicalSource(schema).toString("utf8")), declared).length) throw Error("Complete repository script provider declaration required");
  return { path, schema, providers: declared.providers };
}

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
export function workspacePackageModuleV1(containing, specifier, workspaceRoot, compiler=parserTypescriptV1(), host=observedTypeScriptHost(compiler.sys), mode=compiler.ModuleKind.ESNext, declarations=false) {
  if (specifier.startsWith(".") || isAbsolute(specifier) || isBuiltin(specifier) || /^[a-z][a-z0-9+.-]*:/u.test(specifier)) return undefined;
  const options={moduleResolution:compiler.ModuleResolutionKind.NodeNext,module:compiler.ModuleKind.NodeNext,allowJs:true,resolveJsonModule:true,noDtsResolution:!declarations};
  let resolution=compiler.resolveModuleName(specifier,containing,options,host,undefined,undefined,mode);
  const packageName=specifier.startsWith("@")?specifier.split("/").slice(0,2).join("/"):specifier.split("/")[0],future=new Map(),packages=[];
  for(const manifest of resolution.affectingLocations??[]){
   const physical=host.realpath?host.realpath(manifest):manifest,local=relative(workspaceRoot,physical);if(local===""||local.startsWith("..")||isAbsolute(local)||local.split(sep).includes("node_modules"))continue;
   const text=host.readFile(physical);if(text===undefined||JSON.parse(text).name!==packageName)continue;
   const producer=readTypeScriptPackageProducerV1(dirname(physical),workspaceRoot,path=>host.readFile(path));packages.push({root:dirname(physical),producer});if(!producer)continue;
   for(const output of producer.outputs){future.set(output,output);future.set(resolve(dirname(manifest),relative(dirname(physical),output)),output);}
  }
  if(future.size){const view={...host,fileExists:path=>future.has(path)||host.fileExists(path),directoryExists:path=>[...future.keys()].some(file=>dirname(file)===path)||host.directoryExists?.(path)===true,realpath:path=>future.get(path)??(host.realpath?host.realpath(path):path)};resolution=compiler.resolveModuleName(specifier,containing,options,view,undefined,undefined,mode);}
  const resolved=resolution.resolvedModule?.resolvedFileName;
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
function routerKeptFromSourceV1(compiler, path, words) { return routerKeptV1(compiler, path, readPhysicalSource(path).toString("utf8"), words); }

/** 🧵️ The imports a router script needs for all the commands it runs with, or nothing when any of them is not a router this reading understands. */
function routerKeptV1(compiler, path, text, words) {
  const kept = new Set();
  for (const word of words) { let own; try { own = routerCommandSpecifiersV1(compiler, path, text, word); } catch { return undefined; } if (own === undefined) return undefined; for (const specifier of own) kept.add(specifier); }
  return kept;
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
  let compiledPaths;
  const walked=await memoizedObservedAsyncComputationV1(cache.closures,key,async()=>{
  const compiler = parserTypescriptV1(), host = observedTypeScriptHost(compiler.sys), files = new Set(),compiled=new Set(),defined=new Set();
  const visit = async(path,source=true) => {
    await checkpoint();
    if (closurePathNameV1(cache, workspaceRoot, path).startsWith("{workspaceRoot}/../")) throw new Error(`Command import escapes workspace: ${path}`);
    if(presentationResourcesV1(cache).has(buildPath(relative(workspaceRoot,path))))return;
    const seen=files.has(path);
    files.add(path);
    if(requiredSources){const admitted=cache.physical??=new Set();if(!admitted.has(path)){requirePhysicalSourceV1(path);admitted.add(path);}}
    else if (!existsSync(path)) return;
    if(!source||!DECLARED_MODULE_SUFFIX_V1.test(path)){cache.resources.add(path);if(!seen)requirePhysicalSourceV1(path);return;}
    if(compiled.has(path))return;
    compiled.add(path);cache.modules.add(path);
    const imports = await memoizedObservedAsyncComputationV1(cache.files,JSON.stringify([resolve(workspaceRoot),path,requiredSources,cache.providerDesignation??null,selectors.get(path)??null]),async()=>{
      requirePhysicalSourceV1(path);
      const ports = observedSourceAnalysisV1(PROVIDER_PORTS, path), imports = ports.length ? designatedPortsV1(cache, ports).map(port => ({ port })) : [], words = selectors.get(path), kept = words?.length ? routerKeptFromSourceV1(compiler, path, words) : undefined;
      const generated=specifier=>{const produced=cache.generatedOutput?.(specifier);if(!produced)return false;const rows=cache.generatedImports.get(path)??[];if(!rows.some(row=>row.target===produced.target&&row.output===produced.output))rows.push(produced);cache.generatedImports.set(path,rows);for(const file of produced.sources??[])imports.push({path:resolve(workspaceRoot,file),source:true});return true;};
      for (const entry of observedSourceAnalysisV1(prepareSourceAnalysesV1().command,path)) {
        await checkpoint();
        if (kept && !kept.has(entry)) continue;
        const resource=buildImportResourceV1(path,entry);
        if(generated(resource.specifier))continue;
        let resolved=workspacePackageModuleV1(path,resource.specifier,workspaceRoot,compiler,host);
        try { resolved ??= observedRequireResolve(path, resource.specifier); }
        catch (error) {
          if (error.code !== "MODULE_NOT_FOUND") throw error;
          resolved = compiler.resolveModuleName(resource.specifier, path, { moduleResolution: compiler.ModuleResolutionKind.Bundler, allowJs: true, resolveJsonModule: true }, host).resolvedModule?.resolvedFileName;
          if (!resolved) {if(requiredSources||resource.suffix||!DECLARED_MODULE_SUFFIX_V1.test(resource.specifier))throw unresolvedSourceImportV1(error,path);continue;}
          if (/\.d\.[cm]?ts$/.test(resolved)) throw error;
        }
        if(!generated(resolved))imports.push({path:resolved,source:resource.source});
      }
      const references = observedSourceAnalysisV1(prepareSourceAnalysesV1().references,path);
      for (const [specifiers, mode] of [[references.runtime.imports, compiler.ModuleKind.ESNext], [references.runtime.requires, compiler.ModuleKind.CommonJS]]) for (const specifier of specifiers) {
        await checkpoint();
        const member = workspacePackageModuleV1(path, specifier, workspaceRoot, compiler, host, mode);
        if (member&&!generated(member)) imports.push({path:member,source:true});
      }
      for (const literal of references.urls) {
        await checkpoint();
        const target = workspaceUrlTargetV1(path, literal, workspaceRoot,false);if(!target||generated(target.path))continue;
        imports.push({...target,url:true});
      }
      return imports;
    },checkpoint);
    for (const imported of imports) {
      if(imported.port!==undefined){defined.add(imported.port);continue;}
      const resource=imported.url&&resources.has(resolve(imported.path));
      if(imported.url&&!resource&&(!existsSync(imported.path)||!statSync(imported.path).isFile()))continue;
      await visit(imported.path,imported.source&&!resource);
    }
  };
  for (const entry of roots) await visit(entry);
  for (const config of resolutionConfigsV1(compiler, host, workspaceRoot, compiled)) await visit(config, false);
  const result = [...files].map((file) => closurePathNameV1(cache, workspaceRoot, file)).sort();
  cache.compiledNames.set(result, new Set([...compiled].map((file) => closurePathNameV1(cache, workspaceRoot, file))));
  cache.walkPorts.set(result, [...defined].sort());
  compiledPaths=[...compiled];
  return result;
  },checkpoint);
  const stored=cache.closures.get(key);
  if(stored){
   const question=walkQuestionV1(cache,workspaceRoot,roots,requiredSources,resources,selectors);
   if(!walkFactsV1.has(stored.observations))walkFactsV1.set(stored.observations,{row:{kind:"derived",path:question,fact:walkAnswerV1(cache,workspaceRoot,walked)},views:Object.fromEntries(Object.keys(WALK_VIEW_POLICY_V1.views).map(view=>[view,{kind:"derived",path:walkQuestionV1(cache,workspaceRoot,roots,requiredSources,resources,selectors,view),fact:walkAnswerV1(cache,workspaceRoot,walked,view)}]))});
   for(const path of compiledPaths??[]){const inner=cache.files.get(JSON.stringify([resolve(workspaceRoot),path,requiredSources,cache.providerDesignation??null,selectors.get(path)??null]));if(inner){let walks=walkMembersV1.get(inner.observations);if(!walks)walkMembersV1.set(inner.observations,walks=new Set());walks.add(question);}}
  }
  return walked;
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
function walkValueV1(recipe, workspaceRoot) {
  const local = path => join(workspaceRoot, path), list = directory => { try { return observedFs.readdirSync(local(directory), { withFileTypes: true }).map(entry => ({ name: entry.name, directory: entry.isDirectory(), file: entry.isFile(), link: entry.isSymbolicLink() })); } catch { return []; } };
  const readText = path => { try { return observedFs.readFileSync(local(path), "utf8"); } catch { return undefined; } };
  if (recipe.fn === "crate-index") return [...crateNamesV1(cargoManifestDirectoriesV1(list, recipe.skip), readText)];
  const host = { exists: path => existsSync(local(path)), listDirectory: directory => list(directory).map(({ name, directory }) => ({ name, directory })) };
  return recipe.question.mode === "entries" ? entryFilesV1(recipe.question.globs, host) : includeRootsV1(recipe.question.include, recipe.question.exclude, host);
}

/** 🌳️ One repository walk as a derived fact: the observations it read are one shared layer, named by the question and the digest of the answer, so a project that used the walk holds that one row instead of the directories the walk listed. */
function derivedWalkV1(workspaceRoot, recipe) {
  const question = JSON.stringify({ derive: 1, workspaceRoot: resolve(workspaceRoot), recipe }), cache = new Map(), value = memoizedObservedComputationV1(cache, question, () => walkValueV1(recipe, workspaceRoot)), stored = cache.get(question);
  if (stored && !walkFactsV1.has(stored.observations)) walkFactsV1.set(stored.observations, { row: { kind: "derived", path: question, fact: derivedWalkAnswerV1(value) } });
  return value;
}

/** ✍️ The stored observations of every file a command walk visited, by the questions of the walks that visited it: they are part of those walks' observations, so a project that names such a walk by its derived fact does not carry them again. */
const walkMembersV1 = new WeakMap();

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
export async function deriveClosureFactV1(row, cache) {
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
    const asked = await observeDetachedV1(() => derivedWalkV1(question.workspaceRoot, recipe));
    return { fact: derivedWalkAnswerV1(asked.value), rows: support(asked) };
  }
  if (question?.walk !== 1 || !Array.isArray(question.roots) || !Array.isArray(question.resources) || !Array.isArray(question.selectors) || question.view !== undefined && !Object.hasOwn(WALK_VIEW_POLICY_V1.views, question.view)) throw refuse();
  if ((question.provider ?? null) !== (cache.providerDesignation ?? null)) throw refuseBuildV1("invalid", "closure.derived-provider-differs", { provider: String(question.provider) });
  const asked = await observeDetachedV1(() => boundedScriptInputsV1(question.roots, question.workspaceRoot, cache, question.requiredSources, question.resources, new Map(question.selectors)));
  return { fact: walkAnswerV1(cache, question.workspaceRoot, asked.value, question.view), rows: support(asked) };
}

/** 🙈️ The resources the build policy declares presentation-only (progress labels): their edits never change a command's behavior, so they are not command inputs. The policy is read once per script input cache and the facts of that read are replayed into every receiving project, so the rows of a project never depend on which project of the same process asked first. */
function presentationResourcesV1(cache) {
  return memoizedObservedComputationV1(cache.presentation ??= new Map(), "presentation", () => new Set(JSON.parse(readPhysicalSource(join(LIBRARY_ROOT, "🏗️build", "🔣️policy.json")).toString("utf8")).presentationResources));
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
   const cargo=readToml(manifest),reference=cargo.package?.metadata?.semio?.["cargo-execution"];if(reference===undefined)continue;if(typeof reference!=="string"||!reference||isAbsolute(reference))throw Error("Relative Cargo execution declaration required");
   const declarationPath=resolve(dirname(manifest),reference);local(declarationPath);readPhysicalSource(manifest);const declaration=readCargoExecutionContractV1("declaration",JSON.parse(readPhysicalSource(declarationPath).toString("utf8")));entries.add(manifest);entries.add(declarationPath);
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
const closureHandleSymbolV1 = Symbol.for("semio.closure.handle"), knownClosureRecipesV1 = new Map();

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

const closureQueriesV1 = new Map([
  ["some", (value, substrings) => (Array.isArray(value) ? value : value?.closure ?? []).some(path => typeof path === "string" && substrings.some(part => path.includes(part)))],
  ["generated", value => value?.generatedDependencies ?? []],
  ["digest", derivedWalkAnswerV1],
]);

/** 🌂️ Registers a pure query over the closure a recipe produces (the inferrer that owns the question, such as the whole-workspace key check, registers it). */
export function registerClosureQueryV1(name, query) { closureQueriesV1.set(name, query); }

const closureScriptsV1 = new Map(), closureEntriesV1 = new Map(), closureInflightV1 = new Map(), closureFailuresV1 = new Map(), knownClosureInputsV1 = new Map();

/** 🧊️ Binds the script input cache that computes the closure entries of one workspace (the inference binds its own, a verifier binds one of its own). */
export function bindClosureScriptsV1(workspaceRoot, scripts) { closureScriptsV1.set(workspaceRoot, scripts); }

/** 🧽️ Forgets every in-process entry, as a process that starts later has none. */
export function resetClosureEntriesV1() { closureEntriesV1.clear(); closureInflightV1.clear(); closureFailuresV1.clear(); knownClosureRecipesV1.clear(); knownClosureInputsV1.clear(); }

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
    case "crate-index": case "source-roots": return derivedWalkV1(workspaceRoot, readDerivedWalkV1(recipe));
    default: throw refuseBuildV1("invalid", "closure.recipe-unknown", { recipe: String(recipe.fn) });
  }
}

const closureEntryHoldsV1 = entry => {
  const round = observationRoundV1();
  if (round && entry.checked?.round === round) return entry.checked.holds;
  const holds = observationRowsHoldV1(entry.rows) && entry.layers.every(observationLayerHoldsV1);
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
  if (known && closureEntryHoldsV1(known)) return known;
  const failed = closureFailuresV1.get(id);
  if (failed) { if (closureEntryHoldsV1(failed)) throw failed.error; closureFailuresV1.delete(id); }
  let flight = closureInflightV1.get(id);
  if (!flight) {
    const scripts = closureScriptsV1.get(workspaceRoot);
    if (!scripts) throw refuseBuildV1("missing", "closure.scripts-unbound", { workspace: String(workspaceRoot) });
    flight = observeDetachedV1(() => produceClosureRecipeV1(recipe, workspaceRoot, scripts)).then(({ value, rows, layers }) => { const entry = { recipe, value, rows, layers, answers: new Map(), checked: null }; closureEntriesV1.set(id, entry); return entry; }, error => { const facts = error?.code === "BUILD_SOURCE_UNPARSEABLE" ? detachedFailureV1(error) : undefined; if (facts) closureFailuresV1.set(id, { error, rows: facts.rows, layers: facts.layers, checked: null }); throw error; }).finally(() => closureInflightV1.delete(id));
    closureInflightV1.set(id, flight);
  }
  return flight;
}

/** 🚚️ Brings the shared entries of every reference an entry's value holds up to date, detached: the asking project holds only the answers its queries record. */
async function ensureProviderEntriesV1(workspaceRoot, entry) { for (const reference of sharedReferencesOfV1(entry.value)) await ensureClosureEntryV1(workspaceRoot, reference.recipe); }

/** 🏺️ An entry's value with its shared references expanded into the files of the shared closures, or undefined while a shared entry is missing or no longer holds. */
function expandedEntryValueV1(workspaceRoot, entry) {
  const references = sharedReferencesOfV1(entry.value);
  if (!references.length) return entry.value;
  const files = [];
  for (const reference of references) {
    const provider = closureEntriesV1.get(workspaceRoot + "\0" + closureRecipeKeyV1(reference.recipe));
    if (!provider || !closureEntryHoldsV1(provider)) return undefined;
    for (const item of provider.value) if (typeof item === "string" && item.startsWith("{workspaceRoot}/")) files.push(item);
  }
  return expandClosureValueV1(entry.value, files);
}

/** ❔️ One query answered over the closure an entry stands for, provider closures expanded; undefined while a provider entry does not hold. An entry without provider references keeps its answers. */
const closureAnswerV1 = (workspaceRoot, entry, query) => {
  const key = JSON.stringify(query), stable = !sharedReferencesOfV1(entry.value).length;
  if (stable && entry.answers.has(key)) return entry.answers.get(key);
  const run = closureQueriesV1.get(query[0]);
  if (!run) throw refuseBuildV1("invalid", "closure.query-unknown", { query: String(query[0]) });
  const value = expandedEntryValueV1(workspaceRoot, entry);
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
  const used = new Map(), fact = async (recipe, query) => { const key = closureRecipeKeyV1(recipe); knownClosureRecipesV1.set(key, recipe); used.set(key, recipe); const entry = await ensureClosureEntryV1(workspaceRoot, recipe); await ensureProviderEntriesV1(workspaceRoot, entry); const answer = closureAnswerV1(workspaceRoot, entry, query); if (answer === undefined) throw refuseBuildV1("invalid", "closure.provider-entry-stale", { port: sharedReferencesOfV1(entry.value).map(row => row.recipe.port ?? row.recipe.name).join(", ") }); recordClosureFactV1(JSON.stringify([workspaceRoot, key, query]), JSON.stringify(answer)); return answer; };
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
      if (!late) return derivedWalkV1(workspaceRoot, recipe);
      const key = closureRecipeKeyV1(recipe);
      knownClosureRecipesV1.set(key, recipe); used.set(key, recipe);
      const entry = await ensureClosureEntryV1(workspaceRoot, recipe);
      recordClosureFactV1(JSON.stringify([workspaceRoot, key, ["digest"]]), JSON.stringify(closureAnswerV1(workspaceRoot, entry, ["digest"])));
      return entry.value;
    },
    async some(list, substrings) {
      const handle = closureHandleOfV1(list);
      return handle ? fact(handle.recipe, ["some", substrings]) : (await expanded(list)).some(path => typeof path === "string" && substrings.some(part => path.includes(part)));
    },
    async query(list, name, ...args) {
      const handle = closureHandleOfV1(list);
      return handle ? fact(handle.recipe, [name, ...args]) : closureQueriesV1.get(name)(await expanded(list), ...args);
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
      const rows = [], seen = new Set(), layers = new Set(), values = new Map(), derived = new Map(), walked = new Set(), take = row => { if (!seen.has(row)) { seen.add(row); rows.push(row); } };
      for (const [key, recipe] of recipes) { const entry = await ensureClosureEntryV1(workspaceRoot, recipe), view = walkViewOfRecipeV1(recipe); values.set(key, entry.value); entry.rows.forEach(take); for (const layer of entry.layers) { layers.add(layer); const fact = walkFactsV1.get(layer); if (fact) { const row = view === undefined ? fact.row : fact.views?.[view] ?? fact.row; derived.set(row.path, { row, support: layer }); walked.add(fact.row.path); } } }
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
