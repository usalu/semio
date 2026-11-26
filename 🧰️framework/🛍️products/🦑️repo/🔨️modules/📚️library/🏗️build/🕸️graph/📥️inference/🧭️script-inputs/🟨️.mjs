import {readTypeScriptPackageProducerV1,typeScriptPackagePublicDirectoryV1} from "./../../../../⚡️caching/📦️artifacts/🟦️typescript/🔗️binding/🏭️producer/🟦️.ts";
import {processPidExistsV1} from "../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟨️.mjs";
import { AsyncLocalStorage } from "node:async_hooks";
import { assertNotReadParentV1, readParentV1 } from "../../../👁️observation/🚧️read-parent/🟨️.mjs";
import { closeSync as vCloseV1, fstatSync as vFstatV1, lstatSync as vLstatV1, mkdirSync as vMkdirV1, openSync as vOpenV1, readFileSync as vReadV1, writeFileSync as vWriteV1, renameSync as vRenameV1, rmSync as vRmV1 } from "node:fs";
import { tmpdir as vTmpV1 } from "node:os";
import { setTimeout as claimDelayV1 } from "node:timers/promises";
import { createBoundedCacheV1 } from "../../../🧊️bounded/🟨️.mjs";
import { createWeakCollectionTrackerV1, sharedBytesV1 } from "../../../🧠️memory/📋️slice-report/🟨️.mjs";
import { creditOwnerSharedV1, throwIfOwnerExceededV1 } from "../../../🧠️memory/🎯️owner-scope/🟨️.mjs";
import { FOOTPRINT_POLICY_V1, footprintPulseV1, releaseIfGrownV1, footprintGateV1, decideFootprintV1, noteSlowV1 } from "../../../🧠️memory/🦶️footprint/🟨️.mjs";
import { createClosureStoreV1 } from "./💾️entries/🟨️.mjs";
import { rowKeyV1 } from "../../🕰️generations/🧮️fact-rows/🟨️.mjs";
import{refuseBuildV1}from"../../../📖️texts/🟦️.ts";
import{isRepoErrorV1,repoErrorDataV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts";
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
import { activeTypeScriptProgramReuseV1, sharedSourceFilesV1 } from "./♨️reuse/🟨️.mjs";
import { memoizedObservedAsyncComputationV1, memoizedObservedComputationV1, observedSourceAnalysisV1, observedRequireResolve, observedTypeScriptHost, observedFs, observeDetachedV1, shareObservationRowsV1, detachedFailureV1, retainDetachedFailureV1, observationLayerHoldsV1, observationRoundV1, observationRowsHoldV1, recordClosureFactV1, registerClosureProviderV1, observedFieldV1, registerFieldExtractorV1, observeDetachedSyncV1, retainObservedRowsV1, withObservationPassRoundV1 , registerProjectClosureTravelV1, observeGlobalFactV1, registerGlobalFactV1, prepareGlobalFactsV1, observationWitnessHoldsSyncV1, observationLayerHoldsAsyncV1, observationRowsHoldAsyncV1, observationRowFreshV1, observationWitnessV1, observationWitnessHoldsV1, OBSERVATION_LIMITS_V1 } from "../../../../🏗️build/👁️observation/🟨️.mjs";
import { sourceTreeFilterV1 } from "../../../../🏗️build/👁️observation/📂️source-names/🟨️.mjs";
import { readCargoExecutionContractV1, selectCargoExecutionOperationV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️execution/🟦️.ts";
import { REVISION_POLICY_V1 } from "../../../📥️inputs/🔏️revision/🟨️.mjs";
import { INFERENCE_LIMITS_V1 } from "../🧬️schema/🟨️.mjs";
import { ROW_LOG_POLICY_V1 } from "../../🕰️generations/🗃️row-log/🟨️.mjs";
import { rowPathsV1, rowTouchedV1 } from "../../🧮️derived-nodes/🧩️closure-kinds/🧭️row-paths/🟨️.mjs";
import { offloadClosureRecipeV1 } from "../../🧮️derived-nodes/🧩️closure-kinds/🏭️walk-farm/🟨️.mjs";
import { validateJsonSchemaSubset } from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { GlobV1 } from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🌐️glob/🟦️.ts";
import { isRuntimeBuiltinSpecifierV1 } from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
import { routerCommandSpecifiersV1, routerCommandWordV1 } from "./🔀️router/🟨️.mjs";
import { cargoManifestDirectoriesAsyncV1, crateNamesOfV1, packageNameV1, entryFilesV1, includeRootsV1, readDerivedWalkV1 } from "../../../📥️inputs/🎯️precision/🟦️.ts";
import { buildSchedulingPrefixesV1 } from "../../../📥️source/🔏️closure/🗓️scheduling/🟦️.ts";
import { WALK_VIEW_POLICY_V1, closureRoleViewV1, walkViewAnswerV1, walkViewOfRecipeV1, walkViewPrefixesV1 } from "./👓️walk-views/🟨️.mjs";
import { PROVIDER_CLOSURE_POLICY_V1, expandClosureValueV1, joinWithProviderViewV1, providerGroupNameV1, providerProjectNameV1, providerReferenceV1, sharedReferencesOfV1, toolGroupNameV1, toolReferenceV1 } from "./🔌️provider-closures/🟨️.mjs";
import { createStringTableV1 } from "../../../👁️observation/🔢️row-table/🟨️.mjs";
import { CLOSURE_GRAPH_FACT_V1, CLOSURE_GRAPH_POLICY_V1, createClosureGraphV1, loadClosureGraphSyncV1, moduleNodeDigestV1, saveClosureGraphV1 } from "./🌳️closure-graph/🟨️.mjs";
import { pathStampV1 } from "./🌳️closure-graph/🍃️leaves/🟨️.mjs";
import { machinePathV1 } from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
const { existsSync, statSync } = observedFs;

/** 🚧️ Turns an import of a command closure that resolves to nothing into the withheld-project failure of its importing source, so one broken file withholds its own project instead of aborting the whole graph. */
function unresolvedSourceImportV1(error,path){return Object.assign(Error(String(error.message),{cause:error}),{code:"BUILD_SOURCE_UNPARSEABLE",unresolvedImport:true,source:path,sha256:createHash("sha256").update(observedFs.readFileSync(path)).digest("hex")});}

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

/** 🔌️ The ids of the source analyses this component registered itself (the provider ports of tool scripts): the rows of a record that name them can only be taken by a machine that registered the same ones, so a taker puts them in beside the ids of the source analyses component. */
export const scriptProviderIdsV1 = () => PROVIDER_PORTS === undefined ? [] : [PROVIDER_PORTS.id];

/** 🗂️ Shares source reads and command closures only within one graph construction. */
export function createScriptInputCache(context={}) {
  if(context.invocation){requireIssuedResourceControlInvocationV1(context.phaseControl,context.invocation);checkResourceLeaseControl(context.phaseControl);}
  return { files: new Map(), closures: new Map(), commandDependencies: new Map(), modules:new Set(), resources:new Set(),generatedImports:new Map(),declared:new Map(),paths:new Map(),compiledNames:new WeakMap(),walkPorts:new WeakMap(),walkTools:new WeakMap(),moduleSets:new WeakMap(),context,providerDesignation:context.scriptProviderDesignation,lateClosures:context.lateClosures===true };
}

export { askHeavyQuestionV1, isHeavyQuestionV1, preflightToolClosuresInChildV1, primeSharedClosuresInChildV1 } from "./🪆️heavy-questions/🟨️.mjs";

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
    return providerProjectOfConfigV1(config, basename(designation));
  });
}

/** 🪪️ The name of the providers project read as two fields of its configuration (its name and the designation it declares), so an edit of any other line of the configuration moves no walk of any project. */
export function providerProjectOfConfigV1(config, designation) {
  const key = PROVIDER_CLOSURE_POLICY_V1.project.metadata, field = (pointer) => { try { return observedFieldV1(config, "json", pointer); } catch (error) { if (String(error?.message).startsWith("Field absent")) return undefined; throw error; } };
  return providerProjectNameV1({ name: field("/name"), metadata: { semio: { [key]: field("/metadata/semio/" + key) } } }, designation);
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

/** 🩺️ Why a package manifest does not resolve a subpath: `export-not-declared` when its `exports` name no such entry, `export-target-missing` (with the absolute target) when the entry is declared and the file it maps to is absent, the state of a manifest whose entry was written before its target; read from the manifest bytes the resolver just observed. */
function exportDiagnosisV1(manifest, packageName, specifier, directory) {
  const entry = packageExportEntryV1(manifest, packageName, specifier), leaf = (value) => typeof value === "string" ? value : value && typeof value === "object" ? Object.values(value).map(leaf).find((item) => item !== undefined) : undefined, target = leaf(entry);
  return target === undefined || target.includes("*") ? { reason: "export-not-declared" } : { reason: "export-target-missing", target: resolve(directory, target) };
}

/** 🪪️ The entry for an exact public export, including the root shorthand. */
function packageExportEntryV1(manifest, packageName, specifier) {
  const subpath = "." + specifier.slice(packageName.length);
  return typeof manifest.exports === "string" ? (subpath === "." ? manifest.exports : undefined) : manifest.exports?.[subpath];
}

/** 🎨️ Finds the physical stylesheet of an exact package export when the TypeScript resolver cannot load it as a module. */
function directStylesheetExportV1(manifest, packageName, specifier, root, host) {
  const target = packageExportEntryV1(manifest, packageName, specifier);
  if (typeof target !== "string" || !target.startsWith("./") || !target.endsWith(".css") || /%(?:2f|5c)|[\\*#?]/iu.test(target)) return undefined;
  let path;
  try {
    const segments = target.slice(2).split("/").map(decodeURIComponent);
    if (segments.some(segment => /^(?:\.{0,2}|node_modules)$/iu.test(segment) || /[\u0000:]/u.test(segment))) return undefined;
    path = join(root, ...segments);
  } catch {
    return undefined;
  }
  if (!host.fileExists(path)) return undefined;
  return host.realpath ? host.realpath(path) : path;
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
  const resolvedName=resolution.resolvedModule?.resolvedFileName??(declarations&&promised?.declarations?resolve(workspaceRoot,promised.declarations):undefined);
  let resolved = resolvedName !== undefined && isAbsolute(resolvedName) ? resolve(resolvedName) : resolvedName;
  if (!resolved) {
    for (const manifest of resolution.affectingLocations ?? []) {
      const physical = host.realpath ? host.realpath(manifest) : manifest;
      const local = relative(workspaceRoot, physical);
      if (local === "" || local.startsWith("..") || isAbsolute(local) || local.split(sep).includes("node_modules")) continue;
      const text = host.readFile(physical);
      if (text === undefined) continue;
      const metadata = JSON.parse(text);
      if (metadata.name !== packageName) continue;
      const root = dirname(physical);
      resolved = declarations ? undefined : directStylesheetExportV1(metadata, packageName, specifier, root, host);
      if (resolved) break;
      const { reason, target } = exportDiagnosisV1(metadata, packageName, specifier, root);
      throw Object.assign(
        new Error(`Workspace import is not resolved by current package metadata: ${specifier} from ${containing} (${reason}${target ? `: ${target}` : ""})`),
        { code: "MODULE_NOT_FOUND", unresolved: true, importer: containing, specifier, watched: [physical], ...(target === undefined ? {} : { target }) },
      );
    }
  }
  if (!resolved || !isAbsolute(resolved)) return undefined;
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
      for (const entry of sourceTreeFilterV1(readdir(join(workspaceRoot, relative)) ?? [], row => row.name, row => row.directory || row.link === true)) {
        const path = relative ? `${relative}/${entry.name}` : entry.name;
        if (entry.directory) walk(path);
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
  const { compiler, host, readdir, join, dirname, isAbsolute, reuse, release = releaseIfGrownV1 } = environment;
  const configs = [...new Set(inputs.flatMap(input => DECLARED_TSCONFIG_V1.test(input) ? [declaredInputPatternV1(input, projectRoot)] : []))].filter(Boolean).sort();
  if (!configs.length) return { files: [], packages: [], configs: [] };
  if (typeof compiler.matchFiles !== "function") throw Error("TypeScript project closure requires the compiler's file matcher");
  const entries = (directory) => { const rows = readdir(directory) ?? []; return { files: rows.filter(row => row.file).map(row => row.name).sort(), directories: rows.filter(row => row.directory).map(row => row.name).sort() }; };
  const parseHost = { ...host, readDirectory: (path, extensions, excludes, includes, depth) => compiler.matchFiles(path, extensions, excludes, includes, host.useCaseSensitiveFileNames, host.getCurrentDirectory(), depth, entries, host.realpath) };
  const files = new Set(), packages = new Set(), declared = [];
  const root = workspaceRoot.replaceAll("\\", "/");
  const inside = (path) => path.startsWith(`${root}/`) && !path.slice(root.length + 1).startsWith(".cache/");
  for (const relative of configs) {
    if (!reuse || reuse.shared) release();
    const config = join(workspaceRoot, relative), source = compiler.readJsonConfigFile(config, host.readFile);
    const parsed = compiler.parseJsonSourceFileConfigFileContent(source, parseHost, dirname(config), undefined, config);
    const options = { ...parsed.options, noEmit: true, incremental: false, composite: false, tsBuildInfoFile: undefined };
    const compilerHost = Object.assign(compiler.createCompilerHost(options, true), { fileExists: host.fileExists, readFile: host.readFile, directoryExists: host.directoryExists, getDirectories: host.getDirectories, realpath: host.realpath });
    if (reuse) compilerHost.getSourceFile = reuse.sourceFile(config, compilerHost.getSourceFile, host.readFile, reuse.shared ? JSON.stringify(options, (key, value) => key === "configFilePath" || key === "pathsBasePath" || key === "configFile" ? undefined : value) : JSON.stringify(options));
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
  if (!reuse || reuse.shared) release();
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
  return memoizedObservedComputationV1(cache.declared, memoDigestV1("module-globs", workspaceRoot, projectRoot, globs), () => { const startedAt = Date.now(); try { return declaredModuleGlobsV1(globs, workspaceRoot, projectRoot, { readdir: declaredDirectoryV1, Glob: GlobV1, join }).map(path => resolve(workspaceRoot, path)); } finally { noteSlowV1("module-globs " + projectRoot, startedAt); } });
}

/** 🏃️ Runs the compiler's own program over the TypeScript projects a target declares, reading the live files through the observed host. */
function typescriptProgramV1(projects, workspaceRoot, projectRoot) {
  const compiler = parserTypescriptV1(), startedAt = Date.now();
  try { return declaredTypeScriptProjectsV1(projects, workspaceRoot, projectRoot, { compiler, host: observedTypeScriptHost(compiler.sys), readdir: declaredDirectoryV1, join, dirname, isAbsolute, reuse: activeTypeScriptProgramReuseV1() ?? sharedSourceFilesV1 }); }
  finally { noteSlowV1("typescript-program " + projectRoot + " " + JSON.stringify(projects).slice(0, 160), startedAt); }
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
  const standard = prefixes === undefined, key = standard ? undefined : prefixes.join("\0"), views = keptClosuresV1.get(closure) ?? keptClosuresV1.set(closure, new Map()).get(closure), known = views.get(key);
  if (known !== undefined) return known;
  const selected = prefixes ?? buildSchedulingPrefixesV1(), kept = closure.filter(entry => typeof entry !== "string" || !selected.some(prefix => (entry.startsWith("{workspaceRoot}/") ? entry.slice("{workspaceRoot}/".length) : entry).startsWith(prefix)));
  views.set(key, kept);
  return kept;
}

/** 🪆️ The filtered copy of one command closure per prefix set, so a closure read through the same view again (every port of a provider designation reads the same walks) returns the identical array and keeps its compiled evidence. */
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
  assertNotReadParentV1("resolve", workspaceRoot);
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

/** 🧠️ Tracks literal relative command imports through the existing TypeScript tooling boundary, up to the script provider ports: a module that defines ports adds the designated ports to the walk's ports (`cache.walkPorts`) instead of the provider modules, so a walk and its derived fact never contain a provider's files; a router script named in `commands` (absolute path to the registered command words it is run with) contributes only the imports those commands need, and every router this reading does not fully understand keeps all of its imports. What the walk read is ONE `closure-graph` global row: its question (with the view of the closure recipe being derived, `./👓️walk-views`) and what the module graph of the tree answers for it ({@link closureGraphAnswerV1}: the closure digest and the answer through the view), never the rows of the modules it visited (the walk and the graph pass run detached, only the reasons of a failure join the caller), so a closure entry or a project that used the walk is valid while that digest holds, or while the answer through its view is the same after the digest moved; the files are walked again only when the digest moved. */
async function boundedScriptInputsV1(entries, workspaceRoot, cache = createScriptInputCache(), requiredSources=false, resourceEntries=[], commands=new Map(), stops=[]) {
  const question = { roots: [...new Set(entries.map(entry => resolve(entry)))].sort(), requiredSources: Boolean(requiredSources), resources: [...new Set(resourceEntries.map(path => resolve(path)))].sort(), selectors: [...commands].map(([path, words]) => [resolve(path), [...new Set(words)].sort()]).sort(byNameV1), stops, view: closureViewScopeV1.getStore() };
  const pass = closureWalkPassV1(cache), key = memoDigestV1(resolve(workspaceRoot), question.roots, question.requiredSources, question.resources, question.selectors, ...(stops.length ? [stopDigestV1(stops)] : []));
  let answered;
  try {
    answered = (await observeDetachedV1(async () => {
      const fact = await closureGraphAnswerV1(cache, workspaceRoot, question, pass);
      let held = cache.closures.get(key);
      if (held?.digest !== fact.digest) {
        let steps = 0;
        const compiler = parserTypescriptV1(), checkpoint = (force = false) => yieldInferenceV1(cache.context, "build:command-source-imports", ++steps, force);
        const walk = await reachWalkV1({ roots: question.roots, workspaceRoot, cache, requiredSources: question.requiredSources, resources: new Set(question.resources), selectors: new Map(question.selectors), checkpoint, compiler, host: observedTypeScriptHost(compiler.sys), stops: new Map(stops), pass });
        cache.compiledNames.set(walk.result, new Set([...walk.compiled].map((file) => closurePathNameV1(cache, workspaceRoot, file))));
        cache.walkPorts.set(walk.result, [...walk.defined].sort());
        cache.walkTools.set(walk.result, [...walk.tools].sort());
        cache.closures.set(key, held = { digest: fact.digest, value: walk.result });
      }
      return { fact, value: held.value, segments: closureGraphSegmentsV1(cache, workspaceRoot, question) };
    })).value;
  } catch (error) { retainDetachedFailureV1(error); throw error; }
  observeGlobalFactV1(CLOSURE_GRAPH_FACT_V1, resolve(workspaceRoot), answered.segments, answered.fact);
  return answered.value;
}

/** 🧭️ The edges of one module: the designated ports it defines, its command imports (through the router selectors of the walk, the workspace packages, the runtime and the TypeScript resolution), its runtime references and its literal URLs, resolved to the files they name. */
async function walkImportsV1({ cache, workspaceRoot, requiredSources, selectors, compiler, host, checkpoint }, path) {
  let importer;
  const sink = new Set(), output = cache.generatedOutput ? (specifier, authored = specifier) => { importer ??= buildPath(relative(workspaceRoot, path)); const got = observeDetachedSyncV1(() => cache.generatedOutput(specifier, { importer, specifier: authored })); for (const part of [got.rows, ...got.layers]) if (part.length) sink.add(part); return got.value; } : undefined;
      requirePhysicalSourceV1(path);
      const ports = observedSourceAnalysisV1(PROVIDER_PORTS, path), imports = ports.length ? designatedPortsV1(cache, ports).map(port => ({ port })) : [], words = selectors.get(path), kept = words?.length ? routerKeptFromSourceV1(compiler, path, words) : undefined;
      const generated=(specifier,authored)=>{const produced=output?.(specifier,authored);if(!produced)return false;const rows=cache.generatedImports.get(path)??[];if(!rows.some(row=>row.target===produced.target&&row.output===produced.output))rows.push(produced);cache.generatedImports.set(path,rows);for(const file of produced.sources??[])imports.push({path:resolve(workspaceRoot,file),source:true});return true;};
      for (const entry of observedSourceAnalysisV1(prepareSourceAnalysesV1().command,path)) {
        await checkpoint();
        if (kept && !kept.has(entry)) continue;
        const resource=buildImportResourceV1(path,entry);
        if(generated(resource.specifier,entry))continue;
        let resolved;try{resolved=workspacePackageModuleV1(path,resource.specifier,workspaceRoot,compiler,host,compiler.ModuleKind.ESNext,false,output);}catch(error){if(error?.unresolved!==true)throw error;}
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
        const resource=buildImportResourceV1(path,specifier);if(generated(resource.specifier,specifier))continue;let member;try{member=workspacePackageModuleV1(path,resource.specifier,workspaceRoot,compiler,host,mode,false,output);}catch(error){if(error?.unresolved!==true)throw error;}
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

/** 🕸️ One module of the command walk as a graph node: what visiting `path` (as a source module or as a resource) observed and the edges it resolved; built once per cache and pass, so a closure is a walk over nodes that are already in memory, and the node of the module closure graph ({@link closureGraphPassV1}) is made from it with the witness of its rows.
 * @see ./🌳️closure-graph/🟨️.mjs */
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
    meta = { paths: new Set(), dirs: new Set(), volatile: false, outside: [] };
    for (const row of part) { const one = rowPathsV1(row, relativeName, OUTSIDE_V1); if (one.outside) meta.outside.push(row); for (const path of one.paths) meta.paths.add(path); for (const dir of one.dirs) meta.dirs.add(dir); meta.volatile ||= one.volatile; }
    meta.paths = [...meta.paths]; meta.dirs = [...meta.dirs];
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

/** 🌳️ The command walk over graph nodes: depth first over `(path, source)` pairs, the same visiting rules as the inline walk; no row of a node joins the caller's scope (the walk is named by its `closure-graph` row, see {@link boundedScriptInputsV1}); a module that is the entry of a declared tool in `stops` (path to tool name) and no root of the walk is not visited, the walk names the tool in `tools` instead and none of the tool's rows is a row of the walk. */
async function reachWalkV1({ roots, workspaceRoot, cache, requiredSources, resources, selectors, checkpoint, compiler, host, stops, pass = closureWalkPassV1(cache) }) {
  const env = { cache, workspaceRoot, requiredSources, selectors, compiler, host, checkpoint, pass };
  env.pass.valid ??= new WeakSet();
  const files = new Set(), compiled = new Set(), defined = new Set(), seen = new Set(), stack = roots.map((path) => [path, true]), directories = new Set(), crossed = new Set();
  let steps = 0;
  const drain = async () => {
    while (stack.length) {
      const [path, source] = stack.pop(), mark = (source ? "1" : "0") + path;
      if (seen.has(mark)) continue;
      seen.add(mark);
      if (++steps % 64 === 0) await checkpoint();
      const tool = source && stops?.get(path);
      if (tool && !roots.includes(path) && !selectors.has(path)) { crossed.add(tool); continue; }
      const node = await walkNodeV1(env, path, source);
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
  for (const directory of directories) for (const config of (await walkConfigNodeV1(env, directory)).configs) configs.add(config);
  for (const config of [...configs].sort()) stack.push([config, false]);
  await drain();
  return { result: [...files].map((file) => closurePathNameV1(cache, workspaceRoot, file)).sort(), compiled, defined, tools: crossed };
}

const moduleKeyV1 = (cache, text) => (cache.moduleKeys ??= createStringTableV1()).id(text);
const walkKeyV1 = (env, path, source) => moduleKeyV1(env.cache, JSON.stringify([resolve(env.workspaceRoot), path, env.requiredSources, env.cache.providerDesignation ?? null, env.selectors.get(path) ?? null, source]));

const managedBytesV1 = () => { const usage = process.memoryUsage(); return usage.heapUsed + usage.external; };

/** 📏️ How many rows a row array shared by nodes needs to be a layer of its own: the compile index of generated outputs has thousands, the lookups of one import a handful. */
const WITNESS_LAYER_ROWS_V1 = 64;
const partIdsV1 = new WeakMap();
let partCounterV1 = 0;
/** 🪪️ The witness of one graph node as two layers: what the node read itself (`parts`, its configuration) and, as its `base`, what every node of one pass shares by identity (the row arrays of `shared`, such as the compile index of generated outputs, thousands of rows that each node used to repeat), computed once per distinct set of shared arrays in `layers` (one per pass, so its stamps are of the moment of the pass) and stored once by the graph.
 * @see ./🌳️closure-graph/🟨️.mjs */
function layeredWitnessV1(layers, node, config) {
  const wide = node.shared.filter(part => part.length >= WITNESS_LAYER_ROWS_V1), narrow = node.shared.filter(part => part.length < WITNESS_LAYER_ROWS_V1);
  const own = observationWitnessV1([...new Set([...node.parts, ...narrow, ...config?.parts ?? []].flat())]);
  if (own === undefined) return null;
  if (!wide.length) return own;
  const key = wide.map(part => { let id = partIdsV1.get(part); if (id === undefined) partIdsV1.set(part, id = ++partCounterV1); return id; }).join(",");
  let layer = layers.get(key);
  if (layer === undefined) layers.set(key, layer = observationWitnessV1([...new Set(wide.flat())]) ?? null);
  return layer === null ? null : { ...own, base: layer };
}

/** 🌳️ One pass of the module closure graph (`./🌳️closure-graph`) over walk `questions` (`{roots, requiredSources, resources, selectors, stops}` as {@link boundedScriptInputsV1} takes them): every node a question reaches is visited through the same graph nodes as the walk (`walkNodeV1`, valid while its rows hold in `cache.pass`, so a pass with a journal dirty set rebuilds only the touched modules), put once per pass into `graph` keyed by its workspace name, source flag, required flag, selected router words and, only for a module whose URL imports a question's resources change, that resource set (its importers reach it through a redirect, so a resource set never copies the graph); a node object and config unchanged since an earlier pass reuse their record (no digest or witness taken again); its digest over kind, name, resolved edges, ports, configs, generated imports and the provider designation; the answer of each question is its closure digest (tools cut, selected routers redirected). A node unchanged since the last pass moves no digest, so a warm pass after an edit recomputes only the components above the edited module. `witness` keeps `{witness, urls}` as the node's stored data: the witness of its rows (`observationWitnessV1`) and the names its URL imports point at; `prune` drops every node no question of a whole pass reached; `trusted` (see {@link closureGraphWitnessedV1}) names the stored nodes whose witness held this round, which are followed through their stored edges without building their walk node (a module whose stored URL imports meet the question's resources only through its record of that resource set: a module whose URL imports change stops being trusted, so a trusted plain record is never the wrong variant). Answers `{digests, visited, built, pruned, graph}`; `progress({done,total})` after each question, `signal` cancels between nodes. A question whose walk fails (a module that does not parse or resolve) leaves the rows of every node it visited on the way in the caller's scope before the error goes on, so whoever withholds work because of it holds what can lift it.
 * @see ./🌳️closure-graph/🧪️tests */
export async function closureGraphPassV1(cache, workspaceRoot, questions, { graph = cache.closureGraph ??= createClosureGraphV1(), signal, progress, witness = false, prune = false, trusted, pass = closureWalkPassV1(cache) } = {}) {
  const compiler = parserTypescriptV1(), host = observedTypeScriptHost(compiler.sys), done = new Set(), answers = [], put = new Map(), memo = cache.closureGraphRecords ??= new WeakMap();
  pass.valid ??= new WeakSet();
  const layers = new Map();
  let steps = 0, visited = 0, built = 0;
  const checkpoint = (force = false) => { signal?.throwIfAborted(); return yieldInferenceV1(cache.context, "build:closure-graph", ++steps, force); };
  const nameOf = path => closurePathNameV1(cache, workspaceRoot, path);
  for (const [at, question] of questions.entries()) {
    const required = Boolean(question.requiredSources), resources = new Set((question.resources ?? []).map(path => resolve(path))), resourceKey = [...resources].sort().map(nameOf).join("\0"), selectors = new Map([...(question.selectors ?? [])].map(([path, words]) => [resolve(path), [...new Set(words)].sort()])), roots = [...new Set(question.roots.map(path => resolve(path)))].sort();
    const env = { cache, workspaceRoot, requiredSources: required, selectors, compiler, host, checkpoint, pass }, keyOf = (path, source, words, resource = "") => JSON.stringify([nameOf(path), source ? 1 : 0, required ? 1 : 0, resource, words ?? null]), whole = resources.size > 0;
    const stops = new Map(), redirect = new Map(), seen = new Set(), stack = roots.map(path => [path, true]);
    for (const [path, tool] of question.stops ?? []) if (!roots.includes(resolve(path)) && !selectors.has(resolve(path))) stops.set(keyOf(resolve(path), true), tool);
    for (const [path, words] of selectors) for (const source of [true, false]) redirect.set(keyOf(path, source), keyOf(path, source, words));
    const walk = async full => {
      while (stack.length) {
        const [path, source] = stack.pop(), plain = keyOf(path, source), selected = keyOf(path, source, selectors.get(path));
        if (seen.has(selected) || source && stops.has(plain)) continue;
        seen.add(selected); visited++;
        if (visited % 64 === 0) await checkpoint();
        if (!full && !whole && done.has(selected)) continue;
        const held = closureHeldRecordV1(graph, trusted, selected, whole ? keyOf(path, source, selectors.get(path), resourceKey) : undefined, name => resources.has(resolve(workspaceRoot, name.slice("{workspaceRoot}/".length))));
        if (held !== undefined) { done.add(selected); if (held.key !== plain) redirect.set(plain, held.key); for (const edge of held.edges) { const [name, flag] = JSON.parse(edge); stack.push([resolve(workspaceRoot, name.slice("{workspaceRoot}/".length)), flag === 1]); } continue; }
        const known = cache.walkNodes?.get(walkKeyV1(env, path, source)), node = await walkNodeV1(env, path, source);
        reached.push(node);
        if (known?.node !== node) built++;
        const affected = node.kind === "module" && node.imports.some(imported => imported.url && resources.has(resolve(imported.path))), key = affected ? keyOf(path, source, selectors.get(path), resourceKey) : selected;
        if (key !== plain) redirect.set(plain, key);
        const config = node.kind === "module" ? await walkConfigNodeV1(env, dirname(path)) : undefined, targets = [];
        if (node.kind === "module") for (const imported of node.imports) { if (imported.port !== undefined) continue; const resource = imported.url && resources.has(resolve(imported.path)); if (imported.url && !resource && !imported.file) continue; targets.push([imported.path, Boolean(imported.source && !resource)]); }
        for (const file of config?.configs ?? []) targets.push([file, false]);
        let mine = memo.get(node); if (mine === undefined) memo.set(node, mine = new Map());
        const kept = mine.get(key);
        let record = kept !== undefined && kept.config === config && (!witness || kept.record.extra?.witness) ? kept.record : undefined;
        if (record === undefined) {
          const content = { kind: node.kind, name: node.name, source: source ? 1 : 0, designation: cache.providerDesignation ?? null, ...node.kind === "module" ? { imports: targets.map(([target, flag]) => nameOf(target) + "\0" + (flag ? 1 : 0)).sort(), ports: [...new Set(node.imports.filter(imported => imported.port !== undefined).map(imported => imported.port))].sort(), generated: cache.generatedImports.get(path) ?? [], words: selectors.get(path) ?? null } : {} };
          record = { key, name: node.name, kind: node.kind, digest: moduleNodeDigestV1(content), edges: targets.map(([target, flag]) => keyOf(target, flag)), extra: witness ? { witness: layeredWitnessV1(layers, node, config), urls: node.kind === "module" ? [...new Set(node.imports.filter(imported => imported.url).map(imported => nameOf(imported.path)))].sort() : [], sha256: node.kind === "module" || node.kind === "resource" ? fileSha256V1(path) : null, provider: closureProviderNameV1(cache, workspaceRoot), ...content.ports?.length ? { ports: content.ports } : {}, ...content.generated?.length ? { generated: content.generated } : {} } : null };
          mine.set(key, { config, record });
        }
        put.set(key, record);
        done.add(selected);
        for (const target of targets) stack.push(target);
      }
    };
    const reached = [];
    try { await walk(false); }
    catch (error) {
      for (const node of reached) for (const part of [...node.parts, ...node.shared, ...node.kind === "module" ? (await walkConfigNodeV1(env, dirname(node.path)).catch(() => undefined))?.parts ?? [] : []]) retainObservedRowsV1(part);
      throw error;
    }
    if (put.size) { graph.update(put.values()); put.clear(); }
    const options = { stops, redirect }, rootKeys = roots.map(path => { const plain = keyOf(path, true); return redirect.get(plain) ?? plain; });
    let digest = graph.closureDigest(rootKeys, options);
    if (digest === undefined) { seen.clear(); stack.push(...roots.map(path => [path, true])); await walk(true); graph.update(put.values()); put.clear(); digest = graph.closureDigest(rootKeys, options); }
    answers.push(digest);
    await progress?.({ done: at + 1, total: questions.length });
  }
  const pruned = prune ? [...graph.records()].filter(record => !done.has(record.key)).map(record => record.key) : [];
  if (pruned.length) graph.update([], pruned);
  return { digests: answers, visited, built, pruned: pruned.length, graph };
}

/** 🪪️ The keys of the stored nodes of `graph` whose witness (`extra.witness`, see {@link closureGraphPassV1} `witness`) holds now: one stat per distinct path and one probe per distinct live row for the whole graph (shared memos), so a process that loads the machine's graph trusts every unchanged module without reading or parsing it; the parsers are registered first (a fresh process would otherwise find the parser of every module's `analysis` row unregistered and trust no module); a node without a witness is never trusted. `step` is awaited between nodes. */
export async function closureGraphWitnessedV1(graph, { step } = {}) {
  const trusted = new Set(), memos = { tuples: new Map(), verdicts: new Map(), step };
  preparedAnalysesV1();
  for (const record of graph.records()) { const witness = record.extra?.witness; if (witness !== null && typeof witness === "object" && await observationWitnessHoldsV1(witness, memos)) trusted.add(record.key); await step?.(); }
  return trusted;
}

const WORKSPACE_NAME_V1 = "{workspaceRoot}/";
/** 🔏️ The SHA-256 of the bytes of a file (null when it cannot be read): what another machine compares with its own bytes before it trusts a node of this graph (`./🌳️closure-graph/🛰️fleet`). */
const fileSha256V1 = path => { try { return createHash("sha256").update(vReadV1(path)).digest("hex"); } catch { return null; } };
const closureRoundV1 = () => { const round = closurePassV1 ?? observationRoundV1(); return round !== null && typeof round === "object" ? round : undefined; };
/** 🗃️ The table of the running validation round in `tables` (made by `make` on first use), undefined outside a round. */
const roundTableV1 = (tables, make) => { const round = closureRoundV1(); if (round === undefined) return undefined; let table = tables.get(round); if (table === undefined) tables.set(round, table = make()); return table; };
let turnAnswersV1;
/** 🗃️ The answers a judge of `closure-graph` rows may take without a pass: those of the running round, and those of the running turn of the event loop (an asynchronous judge prepares and then judges in one turn); `ready` makes the tables, otherwise an absent table is undefined. */
const judgeAnswerTablesV1 = ready => [roundTableV1(graphAnswersV1, () => new Map()), ready ? turnAnswersV1 ??= (setImmediate(() => { turnAnswersV1 = undefined; }), new Map()) : turnAnswersV1].filter(table => table !== undefined);
const graphAnswersV1 = new WeakMap(), passAnswersV1 = new WeakMap(), graphTrustV1 = new WeakMap(), storedGraphsV1 = createBoundedCacheV1({ name: "inference:closure-graphs", maxEntries: 4, maxBytes: 4 }), latestGraphsV1 = createBoundedCacheV1({ name: "inference:closure-graphs-latest", maxEntries: 8, maxBytes: 8 }), closureGraphSavesV1 = new Set();

/** 🧭️ The pass the walks and the module graph of a cache share in the running validation round: `cache.pass` when a caller set one (a generation deriver), else one per round, so a node is checked once per round and an answer of the round stays its answer; outside a round every walk has its own. */
function closureWalkPassV1(cache) {
  if (cache.pass !== undefined) return cache.pass;
  const passes = cache.roundPasses ??= new WeakMap(), round = closureRoundV1();
  if (round === undefined) return {};
  let pass = passes.get(round);
  if (pass === undefined) passes.set(round, pass = {});
  return pass;
}

/** 🏷️ The script provider designation a cache walks with, as a workspace name (null without one): the ports a module defines resolve through it, so a node is only ever trusted by a walk under the same designation. */
const closureProviderNameV1 = (cache, workspaceRoot) => cache.providerDesignation ? closurePathNameV1(cache, workspaceRoot, resolve(cache.providerDesignation)) : null;
/** 🏷️ A walk question as the segments of its `closure-graph` row (with the provider designation it is walked under): workspace names only (`{workspaceRoot}/…`), so the row travels between machines and checkouts unchanged. */
function closureGraphSegmentsV1(cache, workspaceRoot, question) {
  const nameOf = path => closurePathNameV1(cache, workspaceRoot, path);
  return { roots: question.roots.map(nameOf), requiredSources: Boolean(question.requiredSources), resources: (question.resources ?? []).map(nameOf), selectors: [...(question.selectors ?? [])].map(([path, words]) => [nameOf(path), [...words]]), stops: (question.stops ?? []).map(([path, tool]) => [nameOf(path), tool]), view: question.view ?? null, provider: closureProviderNameV1(cache, workspaceRoot) };
}
const workspaceNameAdmittedV1 = name => typeof name === "string" && name.startsWith(WORKSPACE_NAME_V1) && !name.split("/").includes("..");
/** 🚏️ Whether the segments of a `closure-graph` row are a walk question: the roots, resources and routers as workspace names inside the checkout, selector words and tool names as strings; a tool entry the walk is cut at is a workspace name that may lie outside (it is never read). */
const closureGraphSegmentsAdmittedV1 = segments => segments !== null && typeof segments === "object" && Array.isArray(segments.roots) && segments.roots.every(workspaceNameAdmittedV1) && typeof segments.requiredSources === "boolean" && Array.isArray(segments.resources) && segments.resources.every(workspaceNameAdmittedV1) && Array.isArray(segments.selectors) && segments.selectors.every(row => Array.isArray(row) && row.length === 2 && workspaceNameAdmittedV1(row[0]) && Array.isArray(row[1]) && row[1].every(word => typeof word === "string")) && Array.isArray(segments.stops) && segments.stops.every(row => Array.isArray(row) && row.length === 2 && typeof row[0] === "string" && row[0].startsWith(WORKSPACE_NAME_V1) && typeof row[1] === "string") && (segments.view === null || Object.hasOwn(WALK_VIEW_POLICY_V1.views, segments.view)) && (segments.provider === null || typeof segments.provider === "string" && segments.provider.startsWith(WORKSPACE_NAME_V1));
/** 🧭️ The walk question the segments of a `closure-graph` row name, in the paths of `workspaceRoot`. */
function closureGraphQuestionOfV1(workspaceRoot, segments) {
  const local = name => resolve(workspaceRoot, name.slice(WORKSPACE_NAME_V1.length));
  return { roots: segments.roots.map(local), requiredSources: segments.requiredSources, resources: segments.resources.map(local), selectors: segments.selectors.map(([name, words]) => [local(name), words]), stops: segments.stops.map(([name, tool]) => [local(name), tool]), view: segments.view ?? undefined };
}

/** 🗄️ The directory the module graph of a workspace is kept in on this machine (`.cache/f/<directory>` of the machine state, whatever closure store a process binds), shared by every process of the machine: the read, its farm and its pool children, and the graph another machine offered once it is adopted here. */
export function closureGraphDirectoryV1(workspaceRoot) { return machinePathV1(resolve(workspaceRoot), ".cache", "f", CLOSURE_GRAPH_POLICY_V1.directory); }
/** 🗄️ The module graph of one script input cache: the graph another cache of the process uses for the workspace under the same designation, else, for the repository's own designation, the machine's stored graph when the cache first needs one (a damaged store counts as none), kept current by every pass of the cache (a graph is the tree's, whatever cache asks); a walk under another designation keeps a graph of its own process, so the machine's graph is loaded once per process; the graph a cache of the workspace last used is what a judge of a `closure-graph` row consults when no cache is bound for the workspace. */
function machineClosureGraphV1(cache, workspaceRoot) {
  if (cache.closureGraph === undefined) {
    const directory = cache.providerDesignation === undefined ? closureGraphDirectoryV1(workspaceRoot) : undefined;
    let graph = latestGraphsV1.get(workspaceRoot + "\0" + closureProviderNameV1(cache, workspaceRoot));
    if (graph === undefined) try { graph = directory === undefined ? undefined : loadClosureGraphSyncV1(directory); } catch {}
    cache.closureGraph = graph ?? createClosureGraphV1();
  }
  latestGraphsV1.set(workspaceRoot + "\0" + closureProviderNameV1(cache, workspaceRoot), cache.closureGraph);
  return cache.closureGraph;
}
/** 🗄️ The machine's stored module graph of a workspace for a process that binds no script input cache (a reader that only judges records), loaded again when its head was replaced. */
function storedClosureGraphV1(workspaceRoot) {
  const directory = closureGraphDirectoryV1(workspaceRoot);
  if (directory === undefined) return undefined;
  let mtime; try { mtime = vLstatV1(join(directory, "head.json")).mtimeMs; } catch { return undefined; }
  const held = storedGraphsV1.get(directory);
  if (held?.mtime === mtime) return held.graph;
  let graph; try { graph = loadClosureGraphSyncV1(directory); } catch { return undefined; }
  if (graph !== undefined) storedGraphsV1.set(directory, { mtime, graph });
  return graph;
}
/** 🪪️ The stored nodes of `graph` a pass may use without visiting them: those whose witness holds, each judged once per round (one stat per distinct path and one probe per distinct live row, shared by every node and graph of the round). */
function storedTrustV1(graph, workspaceRoot, provider) {
  const memo = roundTableV1(graphTrustV1, () => ({ tuples: new Map(), verdicts: new Map(), nodes: new WeakMap() })) ?? { tuples: new Map(), verdicts: new Map(), nodes: new WeakMap() };
  preparedAnalysesV1();
  return { has(key) { const record = graph.get(key); if (record === undefined) return false; let holds = memo.nodes.get(record); if (holds === undefined) { const witness = record.extra?.witness, adopted = record.extra?.adopted; holds = (record.extra?.provider ?? null) === provider && (witness !== null && typeof witness === "object" ? observationWitnessHoldsSyncV1(witness, memo) : typeof adopted === "string" && adoptedNodeHoldsV1(workspaceRoot, record, adopted, memo)); memo.nodes.set(record, holds); } return holds; } };
}

/** 🛰️ Whether a node this machine adopted from another machine's graph (`extra.adopted`, the local stamp of its file when its bytes were found equal, see `./🌳️closure-graph/🛰️fleet`) still stands for the disk: the file has that stamp now. */
function adoptedNodeHoldsV1(workspaceRoot, record, adopted, memo) {
  if (!workspaceNameAdmittedV1(record.name)) return false;
  const path = resolve(workspaceRoot, record.name.slice(WORKSPACE_NAME_V1.length));
  let now = memo.tuples.get("\0adopted\0" + path);
  if (now === undefined) { now = pathStampV1(path, Number.POSITIVE_INFINITY, 0) ?? "?"; memo.tuples.set("\0adopted\0" + path, now); }
  return now === adopted;
}

/** 👓️ The view a closure recipe reads its walks through (`./👓️walk-views`), undefined for a whole walk or a recipe that names none. */
const recipeViewV1 = recipe => { try { return walkViewOfRecipeV1(recipe); } catch { return undefined; } };
/** 👓️ The view of the closure recipe whose derivation is running: every walk the derivation asks records its `closure-graph` row through it. */
const closureViewScopeV1 = new AsyncLocalStorage();

/** 🆔️ What a project reads of a walk, from the nodes it reached: the answer digest of `./👓️walk-views` (the roster, the compiled modules, the generated outputs they import and the provider ports, through `view`), the same function of the same inputs a walk over the files gives. */
function closureGraphViewAnswerV1(graph, keys, view) {
  const files = new Set(), compiled = new Map(), ports = new Set();
  for (const key of keys) {
    const record = graph.get(key);
    if (record.kind === "skip") continue;
    files.add(record.name);
    if (record.kind !== "module") continue;
    if (!compiled.has(record.name)) compiled.set(record.name, record.extra?.generated ?? []);
    for (const port of record.extra?.ports ?? []) ports.add(port);
  }
  const names = [...compiled.keys()].sort();
  return walkViewAnswerV1([...files].sort(), names, names.map(name => [name, compiled.get(name)]), view, [...ports].sort());
}

/** 🌳️ What the module graph of the tree answers for one walk question, once per walk pass (`pass`: one per validation round, the one a caller set, or a fresh one): `digest` the closure digest of the question, `answer` the answer through its view ({@link closureGraphViewAnswerV1}); the machine's stored graph is the start, a stored node whose witness holds is used as it is, every other node is visited through the walk's own nodes ({@link closureGraphPassV1}), and a graph that changed is saved for the next process of the machine; a question that fails is passed again without stored nodes, so the failure names every node on its way. The memory a pass that built nodes kept is credited to the cache (`sharedBytes`) and the running owner scope: the graph is what every owner after it reads, never the growth of the owner that happened to pay it. */
export async function closureGraphAnswerV1(cache, workspaceRoot, question, pass = closureWalkPassV1(cache)) {
  const root = resolve(workspaceRoot), id = root + "\0" + JSON.stringify(closureGraphSegmentsV1(cache, root, question)), answers = passAnswersV1.get(pass) ?? passAnswersV1.set(pass, new Map()).get(pass), held = answers.get(id);
  if (held !== undefined) return held;
  const graph = machineClosureGraphV1(cache, root), bytes = managedBytesV1(), { digests: [digest], built } = await closureGraphPassV1(cache, root, [question], { graph, witness: true, trusted: storedTrustV1(graph, root, closureProviderNameV1(cache, root)), pass }).catch(() => closureGraphPassV1(cache, root, [question], { graph, witness: true, pass }));
  if (built > 0) { const credit = Math.max(0, managedBytesV1() - bytes); cache.sharedBytes = (cache.sharedBytes ?? 0) + credit; creditOwnerSharedV1(credit); }
  if (digest === undefined) throw refuseBuildV1("missing", "closure.graph-incomplete", { key: String(question.roots[0] ?? "").slice(0, 160) });
  const reached = closureGraphDigestSyncV1(graph, closureGraphSegmentsV1(cache, root, question), { has: key => graph.has(key) });
  if (reached === undefined) throw refuseBuildV1("missing", "closure.graph-incomplete", { key: String(question.roots[0] ?? "").slice(0, 160) });
  const fact = { digest, answer: closureGraphViewAnswerV1(graph, reached.keys, question.view) };
  answers.set(id, fact);
  for (const table of judgeAnswerTablesV1(true)) table.set(id, fact);
  if (built > 0) scheduleClosureGraphSaveV1(cache, root);
  return fact;
}

/** 🔑️ The stored record a trusted node of a walk question is followed through, the one derivation of {@link closureGraphPassV1} and {@link closureGraphDigestSyncV1}: the record of the resource-varied key (`varied`, set when the question has resources) when it is trusted, since a module whose URL imports meet the question's resources is stored under that key alone; else the plain record of `selected` when it is trusted and none of its stored URL imports `hits`; else undefined (the node must be visited). `trusted` is undefined when nothing is trusted.
 * @see ./🌳️closure-graph/🧪️tests/🔑️resource-key */
function closureHeldRecordV1(graph, trusted, selected, varied, hits) {
  if (trusted === undefined) return undefined;
  if (varied !== undefined && trusted.has(varied)) return graph.get(varied);
  if (!trusted.has(selected)) return undefined;
  const base = graph.get(selected);
  return varied !== undefined && base.extra?.urls?.some(hits) ? undefined : base;
}

/** 🌳️ The closure digest and the reached keys of a walk question from the stored nodes of `graph` alone, without visiting a module: the traversal of {@link closureGraphPassV1} where every reached node is trusted (`trusted.has`); undefined as soon as a reached node is missing or not trusted (the question must then be passed). */
function closureGraphDigestSyncV1(graph, segments, trusted) {
  const required = segments.requiredSources ? 1 : 0, words = new Map(segments.selectors), resources = new Set(segments.resources), resourceKey = [...resources].sort().join("\0"), whole = resources.size > 0, roots = segments.roots;
  const keyOf = (name, source, chosen, resource = "") => JSON.stringify([name, source ? 1 : 0, required, resource, chosen ?? null]), stops = new Map(), redirect = new Map(), seen = new Set(), stack = roots.map(name => [name, true]);
  for (const [name, tool] of segments.stops) if (!roots.includes(name) && !words.has(name)) stops.set(keyOf(name, true), tool);
  for (const [name, chosen] of words) for (const source of [true, false]) redirect.set(keyOf(name, source), keyOf(name, source, chosen));
  while (stack.length) {
    const [name, source] = stack.pop(), plain = keyOf(name, source), selected = keyOf(name, source, words.get(name));
    if (seen.has(selected) || source && stops.has(plain)) continue;
    seen.add(selected);
    const held = closureHeldRecordV1(graph, trusted, selected, whole ? keyOf(name, source, words.get(name), resourceKey) : undefined, url => resources.has(url));
    if (held === undefined) return undefined;
    if (held.key !== plain) redirect.set(plain, held.key);
    for (const edge of held.edges) { const [target, flag] = JSON.parse(edge); stack.push([target, flag === 1]); }
  }
  const options = { stops, redirect }, rootKeys = roots.map(name => redirect.get(keyOf(name, true)) ?? keyOf(name, true));
  return { digest: graph.closureDigest(rootKeys, options), keys: graph.reach(rootKeys, options).keys };
}

/** 🪪️ Whether the `closure-graph` row of a walk still holds: what the running round answered for its question, else what the stored nodes of the machine's graph give while every node the question reaches is witnessed ({@link closureGraphDigestSyncV1}); it holds while the closure digest is the recorded one, or, after the digest moved, while the answer through the row's view is the recorded one (an edit the view leaves out). A question that cannot be decided without visiting modules does not hold here (an asynchronous judge prepares it first, {@link prepareClosureGraphRowsV1}), so the verdict is never wrong, at worst an entry is derived again. */
function closureGraphFactHoldsV1(workspaceRoot, segments, fact) {
  if (!closureGraphSegmentsAdmittedV1(segments) || typeof fact?.digest !== "string" || typeof fact.answer !== "string") return false;
  const id = workspaceRoot + "\0" + JSON.stringify(segments), tables = judgeAnswerTablesV1(false);
  let now = tables.map(table => table.get(id)).find(fact => fact !== undefined);
  if (now === undefined) {
    const scripts = closureScriptsV1.get(workspaceRoot), provider = segments.provider ?? null, graph = scripts !== undefined && closureProviderNameV1(scripts, workspaceRoot) === provider ? machineClosureGraphV1(scripts, workspaceRoot) : latestGraphsV1.get(workspaceRoot + "\0" + provider) ?? storedClosureGraphV1(workspaceRoot), reached = graph === undefined ? undefined : closureGraphDigestSyncV1(graph, segments, storedTrustV1(graph, workspaceRoot, provider));
    if (reached === undefined) return false;
    now = { digest: reached.digest, answer: closureGraphViewAnswerV1(graph, reached.keys, segments.view ?? undefined) };
    tables[0]?.set(id, now);
  }
  return now.digest === fact.digest || now.answer === fact.answer;
}
/** 🛎️ Gets the `closure-graph` rows among `rows` answered for the running round, each question once, through the script input cache bound for its workspace; a question whose workspace binds none, or that cannot be walked, is left to {@link closureGraphFactHoldsV1}. */
async function prepareClosureGraphRowsV1(rows) {
  for (const row of rows) {
    let workspaceRoot, segments;
    try { [, workspaceRoot, segments] = JSON.parse(row.path); } catch { continue; }
    const scripts = closureScriptsV1.get(workspaceRoot);
    if (scripts === undefined || !closureGraphSegmentsAdmittedV1(segments) || closureProviderNameV1(scripts, workspaceRoot) !== (segments.provider ?? null)) continue;
    try { await closureGraphAnswerV1(scripts, workspaceRoot, closureGraphQuestionOfV1(workspaceRoot, segments)); } catch {}
  }
}
registerGlobalFactV1(CLOSURE_GRAPH_FACT_V1, closureGraphFactHoldsV1, prepareClosureGraphRowsV1);
const CLOSURE_GRAPH_PATH_V1 = JSON.stringify([CLOSURE_GRAPH_FACT_V1]).slice(0, -1) + ",";
/** 🪪️ The `closure-graph` rows among the own rows of an entry or a bundle, prepared and judged in one turn before anything else is (a long row check gives the event loop turns), and the other rows for the caller to judge. */
async function closureGraphRowsHoldV1(rows) {
  const graph = rows.filter(row => row.kind === "global" && row.path.startsWith(CLOSURE_GRAPH_PATH_V1));
  if (!graph.length) return { holds: true, rest: rows };
  await prepareGlobalFactsV1(graph);
  return { holds: observationRowsHoldV1(graph), rest: rows.filter(row => !graph.includes(row)) };
}

/** 💾️ Saves the module graph of a cache walking under the repository's own designation into the machine's directory `saveDelayMilliseconds` after it changed, one save at a time and again when it changed meanwhile, and at the latest before the process exits ({@link flushClosureGraphsV1}); a process that only reads the store (a worker child) never writes it, nor does a walk under another designation. */
function scheduleClosureGraphSaveV1(cache, workspaceRoot) {
  const bound = closureStoreBindingsV1.get(workspaceRoot), directory = closureGraphDirectoryV1(workspaceRoot);
  if (directory === undefined || cache.providerDesignation !== undefined || bound !== undefined && !bound.writable) return;
  const state = cache.closureGraphSave ??= { directory, cache, dirty: false, timer: undefined, running: undefined };
  state.dirty = true;
  closureGraphSavesV1.add(state);
  if (!closureGraphSavesV1.registered) { closureGraphSavesV1.registered = true; process.once("beforeExit", () => { void flushClosureGraphsV1(); }); }
  if (state.timer !== undefined || state.running !== undefined) return;
  state.timer = setTimeout(() => { state.timer = undefined; void saveClosureGraphStateV1(state); }, CLOSURE_GRAPH_POLICY_V1.saveDelayMilliseconds);
  state.timer.unref?.();
}
async function saveClosureGraphStateV1(state) {
  while (state.dirty) { state.dirty = false; state.running = saveClosureGraphV1(state.cache.closureGraph, state.directory).catch(() => undefined); await state.running; state.running = undefined; }
  closureGraphSavesV1.delete(state);
}
/** 💾️ Saves now every module graph of this process that changed since its last save. */
export async function flushClosureGraphsV1() {
  for (const state of [...closureGraphSavesV1]) { if (state.timer !== undefined) { clearTimeout(state.timer); state.timer = undefined; } await state.running; await saveClosureGraphStateV1(state); }
}

/** 🧭️ The walk question of one target's command entries as {@link targetScriptSources} asks it for an owner record (`cover` "queries"): the script entries no declared tool covers, the router selectors, the resources and the declared tool stops; the declared module inputs a target adds after its first walk are not part of it. */
export async function targetClosureQuestionV1(target, workspaceRoot, scripts, projectRoot = ".", nativeManifest) {
  const { resources, entries, words } = await targetScriptEntriesV1(target, workspaceRoot, projectRoot, nativeManifest), tools = toolCoverV1(scripts, target, entries, words, resources, false);
  for (const entry of tools.covered) entries.delete(entry);
  if (!entries.size) return undefined;
  return { roots: [...entries], requiredSources: true, resources, selectors: [...words].filter(([, known]) => known).map(([script, known]) => [script, [...known]]), stops: toolStopsV1(scripts, false) };
}

/** 🌐️ The full command closure of `entries` as a process loads it: the bounded walk and, for every script provider port it names,the designation, its schema and the provider's own bounded walk, transitively; composed from the bounded walks, so each provider walk is computed once and every caller holds the same walk facts. Callers that list files read this (owner discovery, component targets, input precision, a whole-walk closure such as the test discovery); a command closure read through a view names its providers by reference instead (`./🔌️provider-closures`). */
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

/** 🪢️ The derived fact of every repository walk and TypeScript program whose stored observations a project can reuse, by those observations: the question it answered and the digest of its answer, so a project names it by one row instead of by the rows it read (a command walk is named by its `closure-graph` row instead, see {@link boundedScriptInputsV1}). */
const walkFactsV1 = new WeakMap();

/** 🗝️ The digest of the answer of a repository walk: what a project that used the walk depends on. */
export function derivedWalkAnswerV1(value) { return createHash("sha256").update(JSON.stringify(value)).digest("hex"); }

/** 🚶️ The answer of one repository walk (the crate index of the Cargo manifests, or the TypeScript files a question selects), read through the observed file system. */
async function walkValueV1(recipe, workspaceRoot, step) {
  const local = path => join(workspaceRoot, path), list = directory => { try { return sourceTreeFilterV1(observedFs.readdirSync(local(directory), { withFileTypes: true }), entry => entry.name, entry => entry.isDirectory() || entry.isSymbolicLink()).map(entry => ({ name: entry.name, directory: entry.isDirectory(), file: entry.isFile(), link: entry.isSymbolicLink() })); } catch { return []; } };
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

/** 🔁️ Asks a derived fact of the command closure again against the live files (a repository walk or a TypeScript program): its answer digest and every observation the computation read, for a generation that decides whether the projects holding the fact stay valid. */
export async function deriveClosureFactV1(row, cache, pass = {}) {
  assertNotReadParentV1("walk", row?.path ?? row?.kind ?? "");
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
  throw refuse();
}

/** 🙈️ The resources the build policy declares presentation-only (progress labels): their edits never change a command's behavior, so they are not command inputs. The policy is read once per script input cache and the facts of that read are replayed into every receiving project, so the rows of a project never depend on which project of the same process asked first. */
function presentationResourcesV1(cache) {
  return memoizedObservedComputationV1(cache.presentation ??= new Map(), "presentation", () => { const path = join(LIBRARY_ROOT, "🏗️build", "🔣️policy.json"); requirePhysicalSourceV1(path); return new Set(observedFieldV1(path, "json", "/presentationResources")); });
}

/** 🔧️ Native leaves hash their compiler contract and exact command implementation, independently of UI selection: the cargo driver and the toolchain files by reference to their tool closures (read through the native view: the scheduling code of leases, capacity, storage, supervision, retries and progress texts stays out, the native-build modules that assemble the Cargo arguments stay in), environment and commands by value. */
export async function nativeCommandInputs(workspaceRoot, scripts) {
  const cargo = POLICY.toolchains.cargo, javascript = POLICY.toolchains.javascript;
  return [
    ...toolReferencesV1(scripts, "cargo-command"),
    ...toolReferencesV1(scripts, "native-toolchain"),
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
export async function targetScriptEntriesV1(target, workspaceRoot, projectRoot = ".", nativeManifest) {
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
 return {resources,entries,words,fallback};
}

const toolTablesV1 = new WeakMap();

/** 📌️ The digest of the stops a walk is cut at, once per stops list. */
const stopDigests = new WeakMap();
const stopDigestV1 = stops => stopDigests.get(stops) ?? stopDigests.set(stops, memoDigestV1(stops)).get(stops);

/** 🛑️ The declared tool entry modules (absolute path and tool name, sorted) at which the walk of a target stops: a module the walk imports that is the entry of a declared tool is a tool boundary (for a generic command only a tool whose view carries the external packages, like the cover of an entry script), the tool's closure belongs to the tool's group of the providers project, and the rows of the tool's modules are no row of any command closure that imports it, so an edit inside a tool moves one group and no owner.
 * @see ./🔌️provider-closures/🔣️.json */
function toolStopsV1(scripts, packages) {
  const table = toolTableV1(scripts), kept = table.stops ??= new Map();
  if (!kept.has(Boolean(packages))) kept.set(Boolean(packages), Object.freeze([...table.entries].filter(([, name]) => !packages || PROVIDER_CLOSURE_POLICY_V1.views[PROVIDER_CLOSURE_POLICY_V1.tools.closures[name].view].packages).sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0)));
  return kept.get(Boolean(packages));
}

/** 🛞️ The declared tool commands of the providers project by the absolute path of each entry module and by name: where a command's script entry is one of them, the command closure names the tool's group instead of walking the script again.
 * @see ./🔌️provider-closures/🔣️.json */
function toolTableV1(scripts) {
  let table = toolTablesV1.get(scripts);
  if (table !== undefined) return table;
  const base = dirname(providerDesignationV1(scripts).path), entries = new Map(), resources = new Map();
  for (const [name, tool] of Object.entries(PROVIDER_CLOSURE_POLICY_V1.tools.closures).sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0)) {
    resources.set(name, new Set(tool.resources.map(path => resolve(base, path))));
    for (const entry of tool.entries) { const path = resolve(base, entry); if (!entries.has(path)) entries.set(path, name); }
  }
  toolTablesV1.set(scripts, table = { entries, resources });
  return table;
}

/** 🪝️ The script entries of one target that a declared tool command already covers, and the references to name instead of walking them. A tool covers an entry when its entry module is the entry and the command selects no router words (or the tool states that its closure is the whole router); a resource no covering tool carries keeps the whole walk of the target. `packages` asks for tools whose view carries the external packages (a generic command keys on them).
 * @see ./🔌️provider-closures/🟨️.mjs */
function toolCoverV1(scripts, target, entries, words, resources, packages) {
  const none = { covered: [], references: [] };
  const table = toolTableV1(scripts), covered = [], names = new Set();
  for (const entry of entries) {
    const name = table.entries.get(entry);
    if (name === undefined) continue;
    const tool = PROVIDER_CLOSURE_POLICY_V1.tools.closures[name];
    if (packages && !PROVIDER_CLOSURE_POLICY_V1.views[tool.view].packages) continue;
    if (words.get(entry) && tool.selection !== "included") continue;
    covered.push(entry); names.add(name);
  }
  if (!covered.length) return none;
  if (resources.some(path => ![...names].some(name => table.resources.get(name).has(path)))) return none;
  const project = providerProjectV1(scripts);
  return { covered, references: [...names].sort().map(name => toolReferenceV1(project, name)) };
}

/** 🪝️ The declared modules of a target (module literals, module globs, runtime entries) that are themselves the entry of a declared tool: the tool names them by one reference and the target does not walk them, so an edit inside the tool moves the group of the tool and no entry of any target that declares it (the toolchain script every component target lists as an input used to carry the whole walk of two thousand modules into the entry of each of them). A tool whose view carries no external packages does not cover a generic command, as in {@link toolCoverV1}.
 * @see ./🔌️provider-closures/🔣️.json */
function toolCoverDeclaredV1(scripts, entries, commandEntries, tools, packages) {
  const table = toolTableV1(scripts), project = providerProjectV1(scripts), names = new Set();
  for (const entry of [...entries]) {
    const name = commandEntries.has(entry) ? undefined : table.entries.get(entry);
    if (name === undefined || packages && !PROVIDER_CLOSURE_POLICY_V1.views[PROVIDER_CLOSURE_POLICY_V1.tools.closures[name].view].packages) continue;
    entries.delete(entry); names.add(name);
  }
  for (const name of [...names].sort()) tools.references.push(toolReferenceV1(project, name));
}

const byNameV1 = ([left], [right]) => left < right ? -1 : left > right ? 1 : 0;
/** 🫙️ The files of the group of one declared tool command, once per script input cache, however many targets ask while the first derivation runs.
 * @see ./🔌️provider-closures/🟨️.mjs */
async function toolGroupFilesV1(workspaceRoot, scripts, name) {
  const groups = scripts.toolGroups ??= new Map();
  let files = groups.get(name);
  if (files === undefined) { const pending = toolClosureGroupV1(workspaceRoot, scripts, name).then(list => new Set(list.filter(item => typeof item === "string"))); groups.set(name, pending); pending.catch(() => { if (groups.get(name) === pending) groups.delete(name); }); files = pending; }
  return files;
}

/** 🛃️ Walks every declared tool closure and the closure of every designated provider port once, through one shared cache, and returns the refusals instead of stopping at the first: a closure that can never be derived (it imports a generated output, names an unknown port) is known before a read derives anything. `progress({done,total})` is awaited after each closure, so a caller can let its event loop turn between the closures (the capped child of the heavy questions reports its phase and its heartbeat there). `refused` holds `{ name, version, code, key, values }` rows, `name` the tool or `port:<name>`. */
export async function preflightToolClosuresV1(workspaceRoot, scripts, { progress } = {}) {
  const refused = [], names = Object.keys(PROVIDER_CLOSURE_POLICY_V1.tools.closures).sort(), view = Object.keys(PROVIDER_CLOSURE_POLICY_V1.views)[0], ports = Object.keys(providerDesignationV1(scripts).providers).sort(), total = names.length + ports.length;
  let done = 0;
  const attempt = async (name, run) => { try { await run(); } catch (error) { if (!isRepoErrorV1(error)) throw error; refused.push({ name, ...repoErrorDataV1(error) }); } await progress?.({ done: ++done, total }); };
  for (const name of names) await attempt(name, () => toolGroupFilesV1(workspaceRoot, scripts, name));
  for (const port of ports) await attempt("port:" + port, () => scriptProviderGroupV1(workspaceRoot, scripts, port, view));
  return { checked: names.length + ports.length, refused };
}

/** 🪚️ The command closure of a target with the declared tools named: the references of the entries a tool covers, and for every declared tool whose entry module the walk reached the references to that tool and the closure without the files of the tool's group (the group is keyed through the reference, so the closure never keys on fewer files than the walk). `mode` is the consumer ("native", "generic" or "queries"); undefined keeps the closure as it is.
 * @see ./🔌️provider-closures/🟨️.mjs */
async function toolFinishV1(scripts, workspaceRoot, own, entryCover, mode, crossed = []) {
  if (mode === undefined) return { list: own, covered: [] };
  const references = new Map(entryCover.references.map(row => [JSON.stringify(row), row]));
  let list = own, left = [];
  if (own !== undefined) {
    const present = new Set(own.filter(item => typeof item === "string")), base = dirname(providerDesignationV1(scripts).path), covered = new Set();
    for (const [name, tool] of Object.entries(PROVIDER_CLOSURE_POLICY_V1.tools.closures).sort(byNameV1)) {
      const reached = crossed.includes(name);
      if (!reached && !tool.entries.some(entry => present.has(closurePathNameV1(scripts, workspaceRoot, resolve(base, entry))))) continue;
      if (!reached) for (const file of await toolGroupFilesV1(workspaceRoot, scripts, name)) covered.add(file);
      const row = toolReferenceV1(providerProjectV1(scripts), name);
      references.set(JSON.stringify(row), row);
    }
    if (covered.size) list = own.filter(item => typeof item !== "string" || !covered.has(item));
    if (covered.size) left = own.filter(item => typeof item === "string" && covered.has(item));
  }
  return { list: references.size ? [...(list ?? []), ...references.values()] : list, covered: left };
}

/** 📽️ Captures outer scripts and explicitly declared child scripts in their own working directories; with `cover` ("native", "generic" or "queries") the entries a declared tool command covers are named by reference to the tool group and not walked. */
export async function targetScriptSources(target, workspaceRoot, scripts, projectRoot = ".",nativeManifest, cover) {
 const {resources,entries,words,fallback}=await targetScriptEntriesV1(target,workspaceRoot,projectRoot,nativeManifest),tools=cover===undefined?{covered:[],references:[]}:toolCoverV1(scripts,target,entries,words,resources,cover==="generic"),stops=cover===undefined?[]:toolStopsV1(scripts,cover==="generic");
 for(const entry of tools.covered)entries.delete(entry);
 const commandEntries=new Set(entries);
 const commands=new Map([...words].filter(([,known])=>known).map(([script,known])=>[script,[...known]])),baseSize=entries.size,base=baseSize?await boundedScriptInputsV1([...entries],workspaceRoot,scripts,true,resources,commands,stops):undefined;
 if(buildInputSourceRoleV1(true,false,[target.metadata?.semio?.sourceInputObservation])==="compile")for(const declared of declaredModuleInputsV1(target.inputs,workspaceRoot,projectRoot,scripts,base&&scripts.compiledNames.get(base)))entries.add(declared);
 for(const declared of declaredModuleGlobEntriesV1(target.metadata?.semio?.closure?.modules,workspaceRoot,projectRoot,scripts))entries.add(declared);
 for(const input of readBuildRuntimeSourceEntriesV1(target.metadata?.semio?.runtimeSourceEntries)){const path=resolve(workspaceRoot,input.replace("{projectRoot}",projectRoot).replace("{workspaceRoot}","."));if(path.length>REVISION_POLICY_V1.maximumPathUnits)throw Error("Bounded physical runtime Source entry required");const entry=declaredModuleInputV1(input,workspaceRoot,projectRoot);if(!entry)throw Error("Bounded physical runtime Source entry required");entries.add(entry);}
 if(cover!==undefined)toolCoverDeclaredV1(scripts,entries,commandEntries,tools,cover==="generic");
 let closure=entries.size===baseSize?base:entries.size?await boundedScriptInputsV1([...entries],workspaceRoot,scripts,true,resources,commands,stops):undefined,generatedDependencies=scripts.generatedImports.size&&closure?generatedTargetsV1(scripts,closure,workspaceRoot):[];const walked=closure,project=closure?declaredProjectClosureV1(target.metadata?.semio?.closure?.typescript,workspaceRoot,projectRoot,scripts,closure):undefined;
 if(project){closure=project.closure;generatedDependencies=[...new Set([...(scripts.generatedImports.size?generatedTargetsV1(scripts,closure,workspaceRoot):[]),...project.generatedDependencies])].sort();}
 const references=walked?providerReferencesV1(scripts,scripts.walkPorts.get(walked)??[],"scheduling"):[],pruned=closure&&withoutSchedulingV1(closure),own=pruned&&references.length?[...pruned,...references]:pruned,finished=await toolFinishV1(scripts,workspaceRoot,own,tools,cover,walked?scripts.walkTools.get(walked)??[]:[]),kept=finished.list;
 if(kept&&kept!==closure){const compiled=scripts.compiledNames.get(closure);if(compiled){const present=new Set(kept);scripts.compiledNames.set(kept,new Set([...compiled].filter(name=>present.has(name))));}}
 return {closure:kept,fallback,...(finished.covered.length?{covered:finished.covered}:{}),...(generatedDependencies.length?{generatedDependencies}:{}),...(project?.packages.length?{packages:project.packages,projectConfig:project.config}:{})};
}

/** ⚙️ Keeps configured recipe inheritance identical for command and lock source discovery. */
function targetCommandRecipes(target) {
  return [target.options ?? {}, ...Object.values(target.configurations ?? {}).map(options => ({ ...target.options, ...options }))];
}

/** 🧳️ Keeps configured executable imports in the target's invariant source closure. */
export async function targetScriptClosure(target, workspaceRoot, scripts, projectRoot = ".", cover) {
  return (await targetScriptSources(target,workspaceRoot,scripts,projectRoot,undefined,cover)).closure;
}

/** 🔐️ `cargo metadata --locked` validates the shared lock against every workspace manifest, so its replay must hash all of them. */
export function nativeLockInputs(command) {
  if(command && typeof command === "object")return [...new Set(targetCommandRecipes(command).flatMap(options=>(typeof options.command==="string"?[options.command]:options.commands??[]).flatMap(operation=>nativeLockInputs(typeof operation==="string"?operation:operation.command))))];
  return typeof command === "string" && command.includes(" native cargo metadata ") ? ["{workspaceRoot}/**/Cargo.toml", "{workspaceRoot}/**/Cargo.lock"] : [];
}

/** ⚖️ Adds the selected native target's local router and executable import closure. */
export async function nativeTargetCommandInputs(target, workspaceRoot, fallback, scripts = createScriptInputCache(), projectRoot = ".") {
  fallback??=await nativeCommandInputs(workspaceRoot,scripts);
  const closure = await targetScriptClosure(target, workspaceRoot, scripts, projectRoot, "native");
  return closure ? joinWithProviderViewV1([closure, fallback], "native") : fallback;
}

/** 🎯️ Hashes any script-backed target's own command closure, exactly, instead of the whole router. */
export async function genericTargetCommandInputs(target, workspaceRoot, fallback, scripts = createScriptInputCache(), projectRoot = ".") {
  const sources = await targetScriptSources(target, workspaceRoot, scripts, projectRoot, undefined, "generic"),closure=sources.closure;
  if (!closure) return [...fallback, "{workspaceRoot}/package.json", `{workspaceRoot}/${javascriptManagerV1.primaryLockName}`];
  const key=memoDigestV1(closure,sources.fallback?fallback:null,sources.packages??[],sources.covered??[]);
  let index=0;const checkpoint=(force=false)=>yieldInferenceV1(scripts.context,"build:command-source-packages",++index,force);
  return memoizedObservedAsyncComputationV1(scripts.commandDependencies,key,async()=>{
  const graph = existsSync(join(workspaceRoot, javascriptManagerV1.primaryLockName)) ? readBunLockGraph(workspaceRoot) : undefined, packages = await closurePackagesV1([...closure, ...(sources.covered ?? [])], workspaceRoot, graph, checkpoint);
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
  const designation = providerDesignationV1(scripts), base = dirname(designation.path), inside = path => !closurePathNameV1(scripts, workspaceRoot, path).startsWith("{workspaceRoot}/../"), walk = await boundedScriptInputsV1(tool.entries.map(entry => resolve(base, entry)).filter(inside), workspaceRoot, scripts), ports = scripts.walkPorts.get(walk) ?? [], files = new Set();
  for (const each of [walk, ...await providerWalksV1(scripts, workspaceRoot, ports, designation)]) {
    const generated = scripts.generatedImports.size ? generatedTargetsV1(scripts, each, workspaceRoot) : [];
    if (generated.length) throw refuseBuildV1("invalid", "closure.provider-generated-import", { port: String(name), targets: generated.join(", ") });
    for (const file of withoutSchedulingV1(each, walkViewPrefixesV1(tool.view))) files.add(file);
  }
  for (const resource of tool.resources) { const path = resolve(base, resource); if (!inside(path)) continue; requirePhysicalSourceV1(path); files.add(closurePathNameV1(scripts, workspaceRoot, path)); }
  if (ports.length) for (const path of [designation.path, designation.schema]) files.add(closurePathNameV1(scripts, workspaceRoot, path));
  const roster = [...files].sort();
  if (!PROVIDER_CLOSURE_POLICY_V1.views[tool.view].packages) return roster;
  const graph = existsSync(join(workspaceRoot, javascriptManagerV1.primaryLockName)) ? readBunLockGraph(workspaceRoot) : undefined;
  let index = 0;
  return [...roster, { externalDependencies: [...await closurePackagesV1(roster, workspaceRoot, graph, (force = false) => yieldInferenceV1(scripts.context, "build:command-source-packages", ++index, force))].sort() }];
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

/** 🔢️ The bytes the parsed layers of a derived entry are charged: `closureLayerRowBytes` per row, nothing once the entry keeps only the digests of its stored layers. */
const closureLayerChargeV1 = layers => layers ? INFERENCE_LIMITS_V1.closureLayerRowBytes * layers.reduce((sum, layer) => sum + layer.length, 0) : 0;
const closureEntryCacheV1 = maxBytes => createBoundedCacheV1({ name: "inference:closure-entries", maxEntries: INFERENCE_LIMITS_V1.closureEntryCacheEntries, maxBytes, sizeOf: entry => INFERENCE_LIMITS_V1.closureEntryBaseBytes + INFERENCE_LIMITS_V1.closureEntryDigestBytes * (entry.digests?.length ?? 0) + INFERENCE_LIMITS_V1.closureEntryRowBytes * entry.rows.length + INFERENCE_LIMITS_V1.closureEntryValueItemBytes * closureValueItemsV1(entry.value) + closureLayerChargeV1(entry.layers) });
/** 🔢️ How many items one closure value holds: the files of a list, or the files and the other lists of a closure record, without serialising it. */
const closureValueItemsV1 = value => Array.isArray(value) ? value.length : Object.values(value ?? {}).reduce((sum, part) => sum + (Array.isArray(part) ? part.length : 1), 1);
let closureEntriesV1 = closureEntryCacheV1(INFERENCE_LIMITS_V1.closureEntryCacheBytes);
const closureScriptsV1 = createBoundedCacheV1({ name: "inference:closure-scripts", maxEntries: 16, maxBytes: 16 }),
  closureStoresV1 = createBoundedCacheV1({ name: "inference:closure-stores", maxEntries: 16, maxBytes: 16 }),
  closureInflightV1 = createBoundedCacheV1({ name: "inference:closure-inflight", maxEntries: 4096, maxBytes: 4096 }),
  closureFailuresV1 = createBoundedCacheV1({ name: "inference:closure-failures", maxEntries: 1024, maxBytes: 1024 }),
  knownClosureInputsV1 = createBoundedCacheV1({ name: "inference:closure-inputs", maxEntries: 65536, maxBytes: 67108864, sizeOf: inputs => 128 + JSON.stringify(inputs).length });

const closureCountsV1 = { derived: 0, loaded: 0, stale: 0, checked: 0, stamped: 0, stampsKept: 0, fetched: 0, reasked: 0, reaskFailed: 0, reaskEqual: 0, reaskDifferent: 0 };
/** 🔢️ How many closure entries this process derived, loaded from the durable store, found stale there and verified row by row (once per discovery pass, however many owners ask) since the last reset. */
export function closureEntryCountsV1() { return { ...closureCountsV1 }; }

/** 🔢️ How many in-process closure entries still hold their parsed layers: only derived entries that no durable store saved. */
export function closureEntriesWithLayersV1() { return closureEntriesV1.keys().filter(key => closureEntriesV1.peek(key)?.layers !== undefined).length; }
const closureEntryBytesV1 = entry => JSON.stringify([entry.value, entry.rows]).length + closureLayerChargeV1(entry.layers);
const closureSidecarV1 = layers => ({ facts: layers.map(layer => walkFactsV1.get(layer) ?? null) });
const walkFactsByDigestV1 = createBoundedCacheV1({ name: "inference:closure-walk-facts", maxEntries: 65536, maxBytes: 65536 }), layerListsV1 = new WeakMap();
/** 🌱️ Hands the derived facts of a derived entry's layers to the digest table before the entry drops its parsed arrays, so the stored shape answers the same questions. */
const seedLayerDigestsV1 = entry => entry.layers.forEach((layer, at) => {
  const digest = entry.digests[at], fact = walkFactsV1.get(layer);
  if (fact && !walkFactsByDigestV1.has(digest)) walkFactsByDigestV1.set(digest, fact);
});
/** 🪩️ The refusal of a layer an entry names that the store no longer has (the collector removes a layer no entry names after the grace, a peer or a pool child supersedes an entry the running read still holds): tagged so that {@link answeredAgainstLiveLayersV1} reads the entry anew, not so that the command fails. Only a lost layer counts: layers and entries that are added do not touch a query that did not read them. */
const layerLostRefusalV1 = digest => Object.assign(refuseBuildV1("invalid", "closure.provider-entry-stale", { port: "layers" }), { closureLayerLost: true, storeChanged: true, layerDigest: digest });
/** 🧱️ The shared layers of an entry as views: a derived entry names its parsed arrays, a stored entry only the digests of its layers (read from the store when a view is consumed, never cached parsed) with the derived facts its sidecar and every other stored entry of the process recorded for them. */
const closureLayerViewsV1 = entry => {
  if (entry.layers) return entry.layers.map(layer => ({ key: layer, rows: layer, fact: walkFactsV1.get(layer) }));
  if (entry.digests === undefined) throw layerLostRefusalV1(undefined);
  return entry.digests.map((digest, at) => {
    const fact = entry.sidecar?.facts[at];
    if (fact && !walkFactsByDigestV1.has(digest)) walkFactsByDigestV1.set(digest, fact);
    return { key: digest, digest, store: entry.store, fact: walkFactsByDigestV1.get(digest) };
  });
};
/** 🚰️ The batches of one layer view: its array, or the batches the store streams. */
function* closureLayerBatchesV1(view) {
  if (view.rows) { yield view.rows; return; }
  try { yield* view.store.layerBatchesSync(view.digest, ROW_LOG_POLICY_V1.batchRows); }
  catch (error) { if (error?.code === "CLOSURE_LAYER_LOST") throw layerLostRefusalV1(view.digest); throw error; }
}
/** 🪪️ The list of a stored layer in `log` from the row ids the store listed for it: the log id of each distinct dictionary row is learned once per log (the row is read from the dictionary and written to the log), a layer whose rows are all known costs a lookup per row and no parse; undefined while the layer has no list or a row of it cannot be read, so the caller parses the layer. */
function indexedLayerListV1(log, lists, view) {
  const store = view.store; if (view.digest === undefined || typeof store?.layerListingSync !== "function" || typeof log.id !== "function") return undefined;
  const listing = store.layerListingSync(view.digest); if (listing === undefined) return undefined;
  const {ids,dictionary} = listing;
  let known = lists.dictionary.get(dictionary);
  if (!known) lists.dictionary.set(dictionary, known = new Map());
  const mapped = new Uint32Array(ids.length);
  for (let at = 0; at < ids.length; at++) {
    const dictionaryId = ids[at]; let id = known.get(dictionaryId);
    if (id === undefined) { const row = listing.rowSync(dictionaryId); if (row === undefined) return undefined; id = log.id(row); known.set(dictionaryId, id); }
    mapped[at] = id;
  }
  const builder = log.builder(); builder.addIds(mapped); return builder.finish();
}
/** 🗂️ The rows of one layer view as a list in `log`, written once per layer and log; the rows of a layer exist in memory one batch at a time. */
export function closureLayerListV1(log, view) {
  let lists = layerListsV1.get(log); if (!lists) layerListsV1.set(log, lists = { digests: new Map(), arrays: new WeakMap(), dictionary: new WeakMap() });
  const table = typeof view.key === "string" ? lists.digests : lists.arrays;
  let list = table.get(view.key);
  if (list === undefined) { list = indexedLayerListV1(log, lists, view); if (list === undefined) { const builder = log.builder(); for (const batch of closureLayerBatchesV1(view)) builder.add(batch); list = builder.finish(); } table.set(view.key, list); }
  return list;
}
const closureStoreBindingsV1 = createBoundedCacheV1({ name: "inference:closure-store-bindings", maxEntries: 256, maxBytes: 256 });
/** 📁️ The directory of the durable closure entries of a workspace: the graph's inference data directory of the running writer, or undefined where no graph writer is running (the entries then live in this process only). */
export function closureStoreDirectoryV1(workspaceRoot) { try { return join(workspaceDataDirectory(workspaceRoot), "ce"); } catch { return undefined; } }
/** 🔗️ Binds the directory a worker process reads the closure entries of its parent from; a worker never writes there, its parent persists the entries it derives itself. `writable` lets a long-lived pool child persist what it derives for the next child of its machine; `limits` (a part of the declared inference limits) bounds that store on disk. */
export function bindClosureStoreV1(workspaceRoot, directory, writable = false, limits = undefined) { closureStoreBindingsV1.set(workspaceRoot, { directory, writable, ...limits === undefined ? {} : { limits: { ...INFERENCE_LIMITS_V1, ...limits } } }); closureStoresV1.delete(workspaceRoot); }

const closureStoreOfV1 = workspaceRoot => {
  let store = closureStoresV1.get(workspaceRoot);
  if (store !== undefined) return store;
  const bound = closureStoreBindingsV1.get(workspaceRoot), directory = bound?.directory ?? closureStoreDirectoryV1(workspaceRoot);
  store = directory === undefined ? null : createClosureStoreV1({ directory, readOnly: bound !== undefined && !bound.writable, ...bound?.limits === undefined ? {} : { limits: bound.limits } });
  closureStoresV1.set(workspaceRoot, store);
  return store;
};

/** 🚚️ The closure entries of a project record as portable bundles and back: `export` walks the recipes a record names and the shared entries their values refer to and answers the bundles the durable store holds (an entry the store lacks is simply not exported), `import` stores the bundles of another machine in the durable store of this workspace, unless an entry of that recipe is already there; a bundle whose recipe does not hash to its key is refused. Every consumer verifies an entry's rows against its own tree, so a wrong bundle costs a derivation and never a wrong graph. */
const closureBundlesV1 = createBoundedCacheV1({ name: "inference:closure-bundles", maxEntries: 512, maxBytes: INFERENCE_LIMITS_V1.closureBundleCacheBytes, sizeOf: row => INFERENCE_LIMITS_V1.closureEntryBaseBytes + INFERENCE_LIMITS_V1.closureEntryRowBytes * (row.bundle.rows.length + row.layerCount) });
/** 🌊️ The entry of a store as a bundle whose layers are read from the store one at a time while it is iterated: nothing of the layers is held, however many there are. */
const lazyBundleV1 = (store, entry) => ({ version: 1, recipe: entry.recipe, value: entry.value, rows: entry.rows, sidecar: entry.sidecar, layers: { [Symbol.iterator]: () => store.layersSync(entry.digests) } });
const TRAVEL_DIGEST_V1 = /^[0-9a-f]{64}$/u;
/** 🚫️ The first own row of a shipped closure entry that did not hold in this tree when the entry was refused at the intake (the entry is then not stored here), by entry id, bounded: a take that later finds no entry names the refusing row instead of "no moved row found".
 * @see ./🧪️tests/🚫️import-refused */
export const importRefusalsV1 = createBoundedCacheV1({ name: "inference:import-refusals", maxEntries: 4096, maxBytes: 4096 });
export function rememberImportRefusalV1(id, bundle) {
  let row;
  try { row = (bundle?.rows ?? []).find(item => !observationRowFreshV1(item)); } catch {}
  importRefusalsV1.set(id, row ?? null);
}
/** 🧪️ Whether the own rows of a bundle hold in this tree (true while no scripts are bound, where nothing can be decided): a bundle whose own rows do not hold is stale and is not stored, since every consumer checks the rows of a stored entry against its own tree and derives it again when they do not hold, so storing it only costs the write; its layers are left to that check, which a bundle that holds pays anyway. */
async function bundleRowsHoldV1(workspaceRoot, bundle) {
  const scripts = closureScriptsV1.get(workspaceRoot);
  if (!scripts) return true;
  const own = await closureGraphRowsHoldV1(bundle.rows);
  return own.holds && observationRowsHoldAsyncV1(own.rest, cooperativeStepV1(scripts, "build:closure-entry-rows"));
}
/** 🧪️ Whether the layers of a bundle hold in this tree (false while no scripts are bound, where nothing can be decided and nothing is stamped). */
async function bundleLayersHoldV1(workspaceRoot, bundle) {
  const scripts = closureScriptsV1.get(workspaceRoot);
  if (!scripts) return false;
  const step = cooperativeStepV1(scripts, "build:closure-entry-rows");
  for (const layer of bundle.layers) if (!await observationLayerHoldsAsyncV1(layer, step)) return false;
  return true;
}
/** 🧪️ Whether the rows and the layers of a bundle hold in this tree (true while no scripts are bound, where nothing can be decided). */
async function bundleHoldsV1(workspaceRoot, bundle) {
  const scripts = closureScriptsV1.get(workspaceRoot);
  if (!scripts) return true;
  const step = cooperativeStepV1(scripts, "build:closure-entry-rows"), own = await closureGraphRowsHoldV1(bundle.rows);
  if (!own.holds || !await observationRowsHoldAsyncV1(own.rest, step)) return false;
  for (const layer of bundle.layers) if (!await observationLayerHoldsAsyncV1(layer, step)) return false;
  return true;
}
/** 🔁️ Whether a bundle taken from the fleet replaces the entry this store already holds under its recipe: it does when the stored entry no longer holds in this tree and the bundle does, so a stale entry (derived before a change, or taken with a row that could never hold) never blocks an entry another machine derived for the same inputs; a bundle that does not hold here leaves the stored entry alone. */
async function bundleSupersedesStoredV1(workspaceRoot, store, stored, bundle) {
  const scripts = closureScriptsV1.get(workspaceRoot);
  if (!await bundleHoldsV1(workspaceRoot, bundle)) return false;
  return !await closureEntryHoldsAsyncV1({ ...stored, layers: undefined, store, answers: new Map(), checked: null }, scripts);
}
/** 🪢️ Whether the entry the store holds under `id` still holds in this tree (loaded once into the entries of the process, so the next question and the next take of the entry find it there): what lets an intake leave a shared document it already has alone, whatever bytes another entry brings it in. */
async function storedEntryHoldsV1(workspaceRoot, store, id) {
  const scripts = closureScriptsV1.get(workspaceRoot);
  if (!scripts) return false;
  const known = closureEntriesV1.peek(id);
  if (known) return closureEntryHoldsAsyncV1(known, scripts);
  const stored = await store.load(id);
  if (!stored) return false;
  const entry = { ...stored, layers: undefined, store, answers: new Map(), checked: null };
  if (!await closureEntryHoldsAsyncV1(entry, scripts)) return false;
  closureEntriesV1.set(id, entry);
  return true;
}
export const closureTravelV1 = Object.freeze({
  async export(workspaceRoot, image) {
    const store = closureStoreOfV1(workspaceRoot);
    if (!store) return [];
    let table;
    try { table = JSON.parse(image)?.closureRecipes; } catch { return []; }
    const seen = new Map();
    const visit = async recipe => {
      const key = closureRecipeKeyV1(recipe);
      if (seen.has(key)) return;
      seen.set(key, undefined);
      const id = workspaceRoot + "\0" + key, stamp = await store.stamp(id);
      if (stamp === undefined) return;
      let described = closureBundlesV1.get(id)?.stamp === stamp ? closureBundlesV1.get(id) : undefined;
      if (!described) {
        const entry = await store.load(id);
        if (entry?.digests) { described = { stamp, bundle: lazyBundleV1(store, entry), layerCount: entry.digests.length }; closureBundlesV1.set(id, described); }
      }
      if (!described) return;
      seen.set(key, described);
      for (const reference of sharedReferencesOfV1(described.bundle.value)) await visit(reference.recipe);
    };
    for (const recipe of Object.values(table ?? {})) await visit(recipe);
    return [...seen].filter(([, described]) => described !== undefined).map(([recipeKey, described]) => ({ recipeKey, stamp: described.stamp, bundle: described.bundle }));
  },
  async import(workspaceRoot, bundles) {
    const bound = closureStoreBindingsV1.get(workspaceRoot), directory = bound?.directory ?? closureStoreDirectoryV1(workspaceRoot);
    if (directory === undefined) return 0;
    const store = createClosureStoreV1({ directory, ...bound?.limits === undefined ? {} : { limits: bound.limits } });
    let taken = 0;
    for (const { recipeKey, bundle, source, neutral } of bundles) {
      try {
        if (closureRecipeKeyV1(bundle?.recipe) !== recipeKey) continue;
        const id = workspaceRoot + "\0" + recipeKey, stored = await store.load(id), started = Date.now();
        if (stored === undefined ? await bundleRowsHoldV1(workspaceRoot, bundle) : await bundleSupersedesStoredV1(workspaceRoot, store, stored, bundle)) { const saved = await store.adopt(id, bundle); importRefusalsV1.delete(id); taken++; if (saved?.digest !== undefined && (stored !== undefined || await bundleLayersHoldV1(workspaceRoot, bundle))) await stampKnownEntryV1(store, saved.digest, bundle.rows, bundle.layers, started); }
        else if (stored === undefined) rememberImportRefusalV1(id, bundle);
        if (typeof source === "string" && TRAVEL_DIGEST_V1.test(source)) await store.remember(source, recipeKey);
        if (typeof neutral === "string" && TRAVEL_DIGEST_V1.test(neutral)) await store.rememberNeutral(neutral, recipeKey);
      } catch { continue; }
    }
    return taken;
  },
  async held(workspaceRoot, source, neutral, refusedMilliseconds = 0) {
    const store = closureStoreOfV1(workspaceRoot);
    if (!store) return undefined;
    try {
      if (TRAVEL_DIGEST_V1.test(source)) {
        const key = await store.travelled(source);
        if (key !== undefined && TRAVEL_DIGEST_V1.test(key) && await store.stamp(workspaceRoot + "\0" + key) !== undefined) return key;
      }
      if (typeof neutral === "string" && TRAVEL_DIGEST_V1.test(neutral)) {
        const known = await store.travelledNeutral(neutral);
        if (known !== undefined && TRAVEL_DIGEST_V1.test(known.key) && (known.ageMilliseconds < refusedMilliseconds || await storedEntryHoldsV1(workspaceRoot, store, workspaceRoot + "\0" + known.key))) return known.key;
      }
    } catch { return undefined; }
    return undefined;
  },
  async claim(workspaceRoot, neutral, staleMilliseconds) {
    const store = closureStoreOfV1(workspaceRoot);
    if (!store || !TRAVEL_DIGEST_V1.test(neutral)) return async () => undefined;
    try { return await store.claim(neutral, staleMilliseconds); } catch { return async () => undefined; }
  },
  async claimed(workspaceRoot, neutral) {
    const store = closureStoreOfV1(workspaceRoot);
    if (!store || !TRAVEL_DIGEST_V1.test(neutral)) return false;
    try { return await store.claimed(neutral); } catch { return false; }
  },
});
registerProjectClosureTravelV1(closureTravelV1);

/** 🧊️ Binds the script input cache that computes the closure entries of one workspace (the inference binds its own, a verifier binds one of its own). */
export function bindClosureScriptsV1(workspaceRoot, scripts) { closureScriptsV1.set(workspaceRoot, scripts); }

/** 🧽️ Forgets every in-process entry, as a process that starts later has none; `entryBytes` sets the byte budget of the in-process entries for the new process (the declared limit by default). */
export function resetClosureEntriesV1(entryBytes = INFERENCE_LIMITS_V1.closureEntryCacheBytes) { closureScriptsV1.clear(); closureEntriesV1.clear(); closureEntriesV1 = closureEntryCacheV1(entryBytes); closureInflightV1.clear(); closureFailuresV1.clear(); knownClosureRecipesV1.clear(); knownClosureInputsV1.clear(); closureStoresV1.clear(); walkFactsByDigestV1.clear(); closureCountsV1.derived = closureCountsV1.loaded = closureCountsV1.stale = closureCountsV1.checked = closureCountsV1.fetched = 0; }

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
    case "target-sources": return targetScriptSources(target, workspaceRoot, scripts, recipe.projectRoot, recipe.nativeManifest, "queries");
    case "crate-index": case "source-roots": return derivedWalkV1(workspaceRoot, readDerivedWalkV1(recipe), scripts);
    default: throw refuseBuildV1("invalid", "closure.recipe-unknown", { recipe: String(recipe.fn) });
  }
}

let closurePassV1;
/** 🧭️ Runs one inference pass during which a shared closure entry is verified once, however many projects ask for it: the files do not change inside a pass, and a pass that nests another shares its token. */
export async function withClosurePassV1(compute, { verdicts } = {}) {
  const outer = closurePassV1;
  closurePassV1 = outer ?? observationRoundV1() ?? new Map();
  try { return await verdictOverrideV1.run(verdicts ?? verdictOverrideV1.getStore(), () => withObservationPassRoundV1(closurePassV1, compute)); } finally { closurePassV1 = outer; }
}

/** 🫧️ One cooperative step of a long loop: every `yieldMilliseconds` it gives the event loop one turn, reports through the inference checkpoint (`phase`) and stops at cancellation. */
function cooperativeStepV1(scripts, phase) {
  let since = -Infinity, index = 0;
  const context = scripts?.context ?? {}, signal = context.ownerSignal ?? context.invocation?.control.signal ?? context.phaseControl?.signal;
  return async () => {
    if (performance.now() - since < OBSERVATION_LIMITS_V1.yieldMilliseconds) return;
    since = performance.now();
    await new Promise(resolve => setImmediate(resolve));
    await yieldInferenceV1(context, phase, ++index);
    signal?.throwIfAborted();
  };
}

/** ⚖️ The verdict on one stored layer of an entry: read from the store batch by batch (no layer is cached parsed), checked row by row, remembered per validation round and digest so every entry that names the layer reads it once; a lost layer does not hold. */
const verdictMapsV1 = createWeakCollectionTrackerV1(), trackVerdictsV1 = round => { const map = new Map(); layerVerdictsV1.set(round, map); verdictMapsV1.track(map); return map; };
/** 🧮️ The layer verdict maps of the live validation rounds, for the slice memory report. */
export function closureCollectionsV1() { let entries = 0, alive = 0, largest; for (const map of verdictMapsV1) { alive++; entries += map.size; if (largest === undefined || map.size > largest.size) largest = map; } const spread = (cache, map) => { const keys = cache.keys(), step = Math.max(1, Math.floor(keys.length / 16)); return { entries: keys.length, sample: () => keys.filter((_key, at) => at % step === 0).slice(0, 16).map(key => map(cache.peek(key))) }; };
  const part = key => ({ name: "closure:entries." + key, ...spread(closureEntriesV1, entry => key === "answers" ? [...entry.answers ?? []] : entry[key]) }), once = (name, cache, pick) => { const seen = new Set(); let bytes = 0, entries = 0; for (const key of cache.keys()) { const value = cache.peek(key); if (value === undefined) continue; entries++; bytes += 8 + sharedBytesV1(pick(value), seen); } return { name, entries, bytes }; }, real = [part("value"), part("rows"), once("closure:entries.digests", closureEntriesV1, entry => entry.digests), once("closure:entries.sidecar", closureEntriesV1, entry => entry.sidecar), part("answers"), part("recipe"), part("layers"), { name: "closure:walkFacts (real)", ...spread(walkFactsByDigestV1, fact => fact) }, { name: "closure:recipes (real)", ...spread(knownClosureRecipesV1, recipe => recipe) }];
  return [...real, ...alive === 0 ? [] : [{ name: "closure:layerVerdicts (" + alive + " rounds)", entries, bytes: entries * 96 }]]; }
/** 🗂️ The verdicts of one read, shared by every process of it: the first process that verifies a stored closure layer or entry writes a small verdict file named by its digest into a directory of the validation round it ran in, every other look of that round (the parent, farm children, pool children) reads the file instead of probing the rows again; a round is one snapshot of the tree, so a new round has a new directory and sees an edit at once. Without a round there is no directory and nothing is shared. A child joins the round of its parent with \`withClosurePassV1(compute, { verdicts })\`.
 * @see ./🧪️tests/🧬️read-verdicts */
const verdictOverrideV1 = new AsyncLocalStorage(), verdictDirsV1 = new WeakMap(), verdictSeenV1 = new Map(), verdictOwnedV1 = new Set(), verdictCountsV1 = { read: 0, written: 0, waited: 0 };
let verdictSerialV1 = 0;
export function closureVerdictScopeV1() {
  const given = verdictOverrideV1.getStore(); if (given !== undefined) return given;
  const round = closurePassV1 ?? observationRoundV1(); if (round === null || typeof round !== "object") return undefined;
  let dir = verdictDirsV1.get(round);
  if (dir === undefined) {
    const parent = join(vTmpV1(), "semio-read-verdicts-" + (process.getuid?.() ?? 0));
    dir = join(parent, process.pid + "-" + Date.now().toString(36) + "-" + ++verdictSerialV1); verdictDirsV1.set(round, dir); verdictOwnedV1.add(dir);
  }
  return dir;
}
process.once("exit", () => { for (const dir of verdictOwnedV1) { try { vRmV1(dir, { recursive: true, force: true }); } catch {} } });
const verdictFileV1 = (kind, digest) => { if (typeof digest !== "string") return undefined; const dir = closureVerdictScopeV1(); return dir === undefined ? undefined : join(dir, kind + "-" + digest); };
function observeVerdictV1(file) {
  if (file === undefined) return "unavailable";
  const seen = verdictSeenV1.get(file); if (seen !== undefined) return seen;
  try {
    const facts = vLstatV1(file);
    if (!facts.isFile() || facts.size > 1) return "unavailable";
    const text = vReadV1(file, "utf8"), holds = text === "1" ? true : text === "0" ? false : undefined;
    if (holds === undefined) return text === "" ? "pending" : "unavailable";
    if (verdictSeenV1.size > 100000) verdictSeenV1.clear();
    verdictSeenV1.set(file, holds);
    return holds;
  } catch (error) { return error?.code === "ENOENT" ? "missing" : "unavailable"; }
}
const readVerdictV1 = file => { const observed = observeVerdictV1(file); return typeof observed === "boolean" ? observed : undefined; };
const verdictDirsMadeV1 = new Set();
const verdictDirMadeV1 = dir => { if (verdictDirsMadeV1.has(dir)) return; vMkdirV1(dir, { recursive: true }); verdictDirsMadeV1.add(dir); };
const writeVerdictV1 = (file, holds) => {
  if (file === undefined) return;
  try { verdictDirMadeV1(dirname(file)); vWriteV1(file, holds ? "1" : "0"); verdictCountsV1.written++; verdictSeenV1.set(file, holds); } catch {}
};
async function sharedVerdictV1(file, compute, signal) {
  const checked = async () => { signal?.throwIfAborted(); const holds = await compute(); signal?.throwIfAborted(); return holds; };
  if (file === undefined) return checked();
  for (let at = 0; at <= 1200; at++) {
    signal?.throwIfAborted();
    const known = observeVerdictV1(file);
    if (typeof known === "boolean") { verdictCountsV1.read++; if (at) verdictCountsV1.waited++; return known; }
    if (known === "unavailable" || at === 1200) break;
    if (at === 0 || known === "missing") {
      try { verdictDirMadeV1(dirname(file)); } catch { break; }
      let descriptor;
      try { descriptor = vOpenV1(file, "wx", 0o600); } catch (error) { if (error?.code !== "EEXIST") break; }
      if (descriptor !== undefined) {
        try {
          const original = vFstatV1(descriptor, { bigint: true });
          const ownsPending = () => { try { const current = vLstatV1(file, { bigint: true }); return current.isFile() && current.size === 0n && current.dev === original.dev && current.ino === original.ino; } catch { return false; } };
          try {
            const holds = await checked();
            if (ownsPending()) try { vWriteV1(descriptor, holds ? "1" : "0"); verdictCountsV1.written++; verdictSeenV1.set(file, holds); } catch {}
            return holds;
          } catch (error) { if (ownsPending()) try { vRmV1(file); } catch {} throw error; }
        } finally { vCloseV1(descriptor); }
      }
    }
    try { await claimDelayV1(25, undefined, { signal }); }
    catch (error) { signal?.throwIfAborted(); throw error; }
  }
  return checked();
}
/** 📊️ How the verdict files went in this process: read from another look, written, and waited for while another process verified. */
export function closureVerdictCountsV1() { return { ...verdictCountsV1 }; }
const layerVerdictsV1 = new WeakMap(), layerFlightsV1 = new WeakMap(), layerCountsV1 = { hits: 0, joined: 0, checked: 0, indexed: 0, milliseconds: 0 };
const rowVerdictsV1 = new WeakMap();
function dictionaryVerdictsV1(round, dictionary) {
  if (!round) return new Map();
  let dictionaries = rowVerdictsV1.get(round);
  if (!dictionaries) rowVerdictsV1.set(round, dictionaries = new WeakMap());
  let verdicts = dictionaries.get(dictionary);
  if (!verdicts) { dictionaries.set(dictionary, verdicts = new Map()); verdictMapsV1.track(verdicts); }
  return verdicts;
}
/** 🪪️ The verdict on one stored layer from the row ids a process listed for it earlier (`layerIdsSync` of the store): the layer is not read, decompressed or parsed, each distinct row of the dictionary is probed against the tree once per round and the layer holds while all its rows do; undefined while the layer has no list, so the caller reads it. A fact that changed since the list was made stales the layers that name its row, exactly. */
async function indexedLayerHoldsAsyncV1(entry, digest, round, step) {
  const store = entry.store; if (typeof store?.layerListingSync !== "function") return undefined;
  const listing = store.layerListingSync(digest); if (listing === undefined) return undefined;
  const {ids,dictionary} = listing, known = dictionaryVerdictsV1(round, dictionary);
  let probed = 0;
  for (let at = 0; at < ids.length; at++) {
    const id = ids[at]; let holds = known.get(id);
    if (holds === undefined) { const row = listing.rowSync(id); if (row === undefined) return undefined; holds = observationRowFreshV1(row); known.set(id, holds); if ((++probed & 63) === 0) await step(); }
    if (!holds) return false;
  }
  return true;
}
/** 🪪️ The synchronous form of {@link indexedLayerHoldsAsyncV1}. */
function indexedLayerHoldsV1(entry, digest, round) {
  const store = entry.store; if (typeof store?.layerListingSync !== "function") return undefined;
  const listing = store.layerListingSync(digest); if (listing === undefined) return undefined;
  const {ids,dictionary} = listing, known = dictionaryVerdictsV1(round, dictionary);
  for (let at = 0; at < ids.length; at++) {
    const id = ids[at]; let holds = known.get(id);
    if (holds === undefined) { const row = listing.rowSync(id); if (row === undefined) return undefined; holds = observationRowFreshV1(row); known.set(id, holds); }
    if (!holds) return false;
  }
  return true;
}

/** 📊️ How the verdicts on the stored layers went in this process: asked again and answered from the round (`hits`), joined to the check another look had started (`joined`), checked against the tree (`checked`, with their `milliseconds`). */
export function layerVerdictCountsV1() { return { ...layerCountsV1, milliseconds: Math.round(layerCountsV1.milliseconds) }; }
async function storedLayerHoldsAsyncV1(entry, digest, step, signal) {
  signal?.throwIfAborted();
  const round = closurePassV1 ?? observationRoundV1(), verdicts = round ? layerVerdictsV1.get(round) ?? trackVerdictsV1(round) : undefined, known = verdicts?.get(digest);
  if (known !== undefined) { layerCountsV1.hits++; return known; }
  const flights = round ? layerFlightsV1.get(round) ?? layerFlightsV1.set(round, new Map()).get(round) : undefined, running = flights?.get(digest);
  if (running !== undefined) { layerCountsV1.joined++; const result = await running; signal?.throwIfAborted(); return result; }
  const check = sharedVerdictV1(verdictFileV1("l", digest), async () => {
    const started = performance.now();
    const listed = await indexedLayerHoldsAsyncV1(entry, digest, round, step);
    if (listed !== undefined) { layerCountsV1.indexed++; layerCountsV1.milliseconds += performance.now() - started; return listed; }
    let holds = true;
    try { for (const batch of entry.store.layerBatchesSync(digest, ROW_LOG_POLICY_V1.batchRows)) if (!await observationLayerHoldsAsyncV1(batch, step)) { holds = false; break; } } catch (error) { if (error?.code !== "CLOSURE_LAYER_LOST") throw error; holds = false; }
    layerCountsV1.checked++; layerCountsV1.milliseconds += performance.now() - started;
    return holds;
  }, signal);
  flights?.set(digest, check);
  try { const result = await check; signal?.throwIfAborted(); verdicts?.set(digest, result); return result; } finally { flights?.delete(digest); }
}
function storedLayerHoldsV1(entry, digest) {
  const file = verdictFileV1("l", digest), filed = readVerdictV1(file);
  if (filed !== undefined) { verdictCountsV1.read++; return filed; }
  const holds = storedLayerComputeV1(entry, digest);
  writeVerdictV1(file, holds);
  return holds;
}
function storedLayerComputeV1(entry, digest) {
  const round = closurePassV1 ?? observationRoundV1(), verdicts = round ? layerVerdictsV1.get(round) ?? trackVerdictsV1(round) : undefined, known = verdicts?.get(digest);
  if (known !== undefined) return known;
  const listed = indexedLayerHoldsV1(entry, digest, round);
  if (listed !== undefined) { layerCountsV1.indexed++; verdicts?.set(digest, listed); return listed; }
  let holds = true;
  try { for (const batch of entry.store.layerBatchesSync(digest, ROW_LOG_POLICY_V1.batchRows)) if (!observationLayerHoldsV1(batch)) { holds = false; break; } } catch (error) { if (error?.code !== "CLOSURE_LAYER_LOST") throw error; holds = false; }
  verdicts?.set(digest, holds);
  return holds;
}

/** 🔌️ Registers the structural parsers of this process before any stored row is judged: an `analysis` row names the parser that made it, and a process that has not prepared its parsers yet (a fresh pool child, whose first look at a stored entry comes before its first derivation) finds the parser of every row unregistered, so every entry that rests on a parsed file reads as stale and is walked again although nothing moved. */
function preparedAnalysesV1() { try { if (typeof prepareSourceAnalysesV1 === "function") prepareSourceAnalysesV1(); } catch {} }
/** 🪪️ The stamps and live verdicts of one validation round, shared by every entry the round verifies: a path is stamped once and a live row probed once per round. */
const stampMemosV1 = new WeakMap();
const stampsOnV1 = () => process.env.SEMIO_VERIFICATION_STAMPS !== "off";
/** 🪪️ Whether the verification stamp the store holds for an entry still matches the disk (see `observationWitnessV1`): a process that finds every row of an entry holding leaves the stamp for every later process of the machine, so the layers of an unchanged entry are verified once per machine and not once per child; false (verify in full) while there is no stamp or any stamped path moved. */
async function entryStampHoldsV1(entry, round, step) {
  const store = entry.store; if (!stampsOnV1() || typeof store?.loadStamp !== "function" || typeof entry.digest !== "string") return false;
  const witness = store.loadStamp(entry.digest); if (witness === undefined) return false;
  let memos = round !== null && typeof round === "object" ? stampMemosV1.get(round) : undefined;
  if (memos === undefined) { memos = { tuples: new Map(), verdicts: new Map() }; if (round !== null && typeof round === "object") stampMemosV1.set(round, memos); }
  const holds = await observationWitnessHoldsV1(witness, { ...memos, step });
  if (holds) closureCountsV1.stamped++;
  return holds;
}
/** 🪪️ Leaves the verification stamp of an entry whose rows all held (`started` is when that verification began): the rows of its layers are read through the row dictionary, each distinct row once. */
async function keepEntryStampV1(entry, started, step) {
  const store = entry.store; if (!stampsOnV1() || typeof store?.saveStamp !== "function" || typeof entry.digest !== "string") return;
  try {
    const rows = [], seen = new Map();
    for (const digest of entry.digests) {
      const listing = store.layerListingSync(digest); let listed = listing !== undefined;
      if (listed) {
        let ids = seen.get(listing.dictionary); if (ids === undefined) seen.set(listing.dictionary, ids = new Set());
        for (const id of listing.ids) { if (ids.has(id)) continue; const row = listing.rowSync(id); if (row === undefined) { listed = false; break; } ids.add(id); rows.push(row); }
      }
      if (!listed) for (const batch of store.layerBatchesSync(digest, ROW_LOG_POLICY_V1.batchRows)) rows.push(...batch);
      await step();
    }
    const witness = observationWitnessV1(rows, { since: started });
    if (witness === undefined) return;
    await store.saveStamp(entry.digest, witness); closureCountsV1.stampsKept++;
  } catch {}
}

/** 🪪️ Leaves the verification stamp of an entry whose rows are in memory and held when they were observed or verified at `started`: no stored layer is read back (a derived or taken entry was stamped only by the next process that verified its layers from disk, once per child, reading and indexing every layer file). A path written after `started` refuses the stamp, so it is never kept over a write inside the walk or the verification. */
async function stampKnownEntryV1(store, digest, rows, layers, started) {
  if (!stampsOnV1() || typeof store?.saveStamp !== "function" || typeof digest !== "string" || started === undefined) return;
  try {
    const all = [...rows], seen = new Set(all);
    for (const layer of layers) for (const row of layer) if (!seen.has(row)) { seen.add(row); all.push(row); }
    const witness = observationWitnessV1(all, { since: started });
    if (witness === undefined) return;
    await store.saveStamp(digest, witness); closureCountsV1.stampsKept++;
  } catch {}
}

/** 🫧️ The verdict of {@link closureEntryHoldsV1}, checked row by row with cooperative steps and remembered for the round, so a stored entry of tens of thousands of rows never blocks the event loop. */
async function closureEntryHoldsAsyncV1(entry, scripts) {
  const signal = scripts?.context?.ownerSignal ?? scripts?.context?.invocation?.control.signal ?? scripts?.context?.phaseControl?.signal;
  signal?.throwIfAborted();
  preparedAnalysesV1();
  const round = closurePassV1 ?? observationRoundV1();
  if (round && entry.checked?.round === round) return entry.checked.holds;
  const entryFile = verdictFileV1("e", entry.digest), filed = readVerdictV1(entryFile);
  if (filed !== undefined) { verdictCountsV1.read++; if (round) entry.checked = { round, holds: filed }; return filed; }
  closureCountsV1.checked++;
  const step = cooperativeStepV1(scripts, "build:closure-entry-rows"), own = await closureGraphRowsHoldV1(entry.rows);
  let holds = (entry.layers !== undefined || entry.digests !== undefined) && own.holds && await observationRowsHoldAsyncV1(own.rest, step);
  if (holds && entry.layers) for (const layer of entry.layers) if (!await observationLayerHoldsAsyncV1(layer, step)) { holds = false; break; }
  if (holds && !entry.layers && !await entryStampHoldsV1(entry, round, step)) {
    const started = Date.now();
    for (const digest of entry.digests) if (!await storedLayerHoldsAsyncV1(entry, digest, step, signal)) { holds = false; break; }
    if (holds) await keepEntryStampV1(entry, started, step);
  }
  signal?.throwIfAborted();
  writeVerdictV1(entryFile, holds);
  if (round) entry.checked = { round, holds };
  return holds;
}

const closureEntryHoldsV1 = entry => {
  preparedAnalysesV1();
  const round = closurePassV1 ?? observationRoundV1();
  if (round && entry.checked?.round === round) return entry.checked.holds;
  const entryFile = verdictFileV1("e", entry.digest), filed = readVerdictV1(entryFile);
  if (filed !== undefined) { verdictCountsV1.read++; if (round) entry.checked = { round, holds: filed }; return filed; }
  const holds = (entry.layers !== undefined || entry.digests !== undefined) && observationRowsHoldV1(entry.rows) && (entry.layers ? entry.layers.every(observationLayerHoldsV1) : entry.digests.every(digest => storedLayerHoldsV1(entry, digest)));
  writeVerdictV1(entryFile, holds);
  if (round) entry.checked = { round, holds };
  return holds;
};

/** 🗃️ The entry of one recipe: computed once with its observations detached from the project that asked, kept while those observations hold, shared by every project and every verification. A recipe whose sources do not parse fails the same way for every project while the facts it read hold; each asking project records those facts, because they are the reason it is withheld and the only thing that can lift it. */
/** 🏭️ The detached walk of one recipe, `{ value, rows, layers }`, as a walk child runs it (see `offloadClosureRecipeV1`). */
export function deriveClosureRecipeV1(workspaceRoot, recipe, scripts) { return observeDetachedV1(() => closureViewScopeV1.run(recipeViewV1(recipe), () => produceClosureRecipeV1(recipe, workspaceRoot, scripts))); }

/** 🏭️ The walk of a child with the derived facts its walks recorded for the layers (identity data of the process that walked them, so they travel beside the layers). */
export async function deriveClosureRecipeForChildV1(workspaceRoot, recipe, scripts) { const walked = await deriveClosureRecipeV1(workspaceRoot, recipe, scripts); return { ...walked, sidecar: closureSidecarV1(walked.layers) }; }

/** 🏭️ The walk of one recipe in a walk child while one is granted, else on this thread: the owners that wait for a shared entry wait on the child, this thread parses the answer and hands the derived facts of the layers back to the process. */
async function walkClosureRecipeV1(workspaceRoot, recipe, scripts, { farmOnly = false } = {}) {
  const farmed = await offloadClosureRecipeV1(workspaceRoot, recipe, farmOnly ? { after: 0 } : {});
  if (farmed === undefined) return farmOnly ? undefined : deriveClosureRecipeV1(workspaceRoot, recipe, scripts);
  farmed.layers.forEach((layer, at) => { const fact = farmed.sidecar?.facts[at]; if (fact) walkFactsV1.set(layer, fact); });
  return farmed;
}

async function ensureClosureEntryV1(workspaceRoot, recipe) {
  try { return await produceClosureEntryV1(workspaceRoot, recipe); }
  catch (error) { retainDetachedFailureV1(error); throw error; }
}

const closureTakeV1 = new AsyncLocalStorage();

/** 🪤️ Runs a look at a stored owner record (the take pass) so that no closure entry is derived inside it: an entry that is not stored or no longer holds ends the look with `BUILD_TAKE_MISS` (the owner is then inferred by the governed owner loop, with its own memory limit, one hand-off to the fleet and the plan guard), instead of the ungoverned walk of hundreds of recipes in the parent that grew the slice worker of a 16 GB machine by gigabytes in seconds.
 * `reask` lets the take hand a stale or absent entry to a walk child instead of ending the look: `true` asks the walk farm of this process (never this thread), a function `(workspaceRoot, recipe) => walked | undefined` asks that port; the owner record then stays valid when the answer equals the recorded one.
 * @see ./🧪️tests/🪤️take-no-derive
 * @see ./🧪️tests/🔁️answer-validity */
export function runClosureTakeV1(body, signal, { reask } = {}) { return closureTakeV1.run({ signal, reask }, body); }

/** 🧭️ The first row of a stored closure entry that no longer holds in this tree (its own rows, then its layers in order), so a verdict on a stale entry names the moved file instead of the whole entry; undefined when none is found within `budgetMilliseconds` or the layers are gone. Rows that stand for machine-local paths hold by definition, so only an input of the walk is ever named.
 * @see ./🧪️tests/🧭️moved-row */
export function firstMovedRowOfEntryV1(entry, budgetMilliseconds = INFERENCE_LIMITS_V1.closureMovedRowMilliseconds) {
  const until = performance.now() + budgetMilliseconds, moved = row => performance.now() <= until && row !== undefined && !observationRowFreshV1(row);
  try {
    for (const row of entry.rows ?? []) { if (performance.now() > until) return undefined; if (moved(row)) return row; }
    if (entry.layers) { for (const layer of entry.layers) for (const row of layer) { if (performance.now() > until) return undefined; if (moved(row)) return row; } return undefined; }
    for (const digest of entry.digests ?? []) for (const batch of entry.store.layerBatchesSync(digest, ROW_LOG_POLICY_V1.batchRows)) for (const row of batch) { if (performance.now() > until) return undefined; if (moved(row)) return row; }
  } catch {}
  return undefined;
}

/** 🏷️ Why one group of closure rows could not be verified, in the words of the verdict: a take that met a stale stored entry (never derived inside a take) says so, any other failure carries its code and first line, and always the recipe key, so `closure-probe` is never the whole story.
 * @see ./🧪️tests/🧭️moved-row */
export function closureFailureWhyV1(error, key) {
  const recipe = String(key).slice(0, 12), moved = error?.movedRow;
  if (error?.code === "BUILD_TAKE_MISS" && error.absent === true) return "closure-entry-absent (recipe " + recipe + (moved === undefined ? ", never shipped or taken here" : ", refused at the intake, moved " + moved.kind + " " + String(moved.path).slice(-120)) + ")";
  if (error?.code === "BUILD_TAKE_MISS") return "closure-entry-stale (recipe " + recipe + (moved === undefined ? ", no moved row found" : ", moved " + moved.kind + " " + String(moved.path).slice(-120)) + ")";
  return "closure-probe " + String(error?.code ?? error?.name ?? "Error") + ": " + String(error?.message ?? error).split("\n")[0].slice(0, 160) + " (recipe " + recipe + ")";
}

/** 🗂️ The first moved row of the stale entries a take judged, per round and entry id, beside {@link staleTakesV1}: bounded by the entries of one round. */
const movedTakesV1 = new WeakMap();

/** 📖️ The stored entry of one recipe for a take: loaded and checked like a derivation's first step, never derived; the miss of a stale entry carries the first moved row of the entry (`movedRow`). */
async function takenClosureEntryV1(workspaceRoot, id, recipe) {
  const scripts = closureScriptsV1.get(workspaceRoot), store = closureStoreOfV1(workspaceRoot);
  if (!scripts) throw refuseBuildV1("missing", "closure.scripts-unbound", { workspace: String(workspaceRoot) });
  const miss = (movedRow, absent = false) => Object.assign(new Error("A closure entry is not stored here or no longer holds: the owner is inferred by the owner loop, not taken"), { code: "BUILD_TAKE_MISS", movedRow, absent });
  const round = closurePassV1 ?? observationRoundV1(), stamp = round !== null && typeof round === "object" && store ? await store.stamp(id) : undefined, judged = stamp === undefined ? undefined : staleTakesV1.get(round) ?? staleTakesV1.set(round, new Map()).get(round), moves = stamp === undefined ? undefined : movedTakesV1.get(round) ?? movedTakesV1.set(round, new Map()).get(round);
  if (stamp !== undefined && judged?.get(id) === stamp) throw miss(moves?.get(id));
  const stored = store ? await store.load(id) : undefined, entry = stored && { ...stored, layers: undefined, store, answers: new Map(), checked: null };
  if (entry && await closureEntryHoldsAsyncV1(entry, scripts)) { closureCountsV1.loaded++; closureEntriesV1.set(id, entry); return entry; }
  if (entry) closureCountsV1.stale++;
  const reasked = await reaskClosureEntryV1(workspaceRoot, id, recipe, scripts, store, closureTakeV1.getStore());
  if (reasked) return reasked;
  let movedRow;
  if (entry) { judged?.set(id, stamp); movedRow = firstMovedRowOfEntryV1(entry); if (movedRow !== undefined) moves?.set(id, movedRow); }
  if (!entry) throw miss(importRefusalsV1.get(id) ?? undefined, true);
  throw miss(movedRow);
}

/** 🔁️ The re-derivations of closure entries in flight, by entry id: the owners of one closure group that meet the same stale or absent entry wait for one walk. */
const closureReasksV1 = createBoundedCacheV1({ name: "inference:closure-reasks", maxEntries: 4096, maxBytes: 4096 });

/** 🪦️ The entries whose re-derivation failed, per round: the next look of the round meets the miss at once. */
const reaskFailedV1 = new WeakMap();

/** 🔁️ Re-asks the closure question of a stale or absent entry in a walk child (never on this thread, never in a read parent that has a farm), keeps the answer as the stored entry and answers it, so that an owner record is judged on the answer its closure rows recorded and not on the bytes of every file of the closure; undefined (the miss stands, as before) when no walk child answers, the walk fails, takes longer than `closureReaskMilliseconds` or `SEMIO_CLOSURE_REASK=off`. One walk serves every owner that asks for the entry while it runs, and the stored entry serves every later one.
 * @see ./🧪️tests/🔁️answer-validity */
async function reaskClosureEntryV1(workspaceRoot, id, recipe, scripts, store, { signal, reask } = {}) {
  if (!reask || !store || recipe === undefined || process.env.SEMIO_CLOSURE_REASK === "off") return undefined;
  signal?.throwIfAborted();
  const round = closurePassV1 ?? observationRoundV1(), failed = round !== null && typeof round === "object" ? reaskFailedV1.get(round) ?? reaskFailedV1.set(round, new Set()).get(round) : undefined;
  if (failed?.has(id)) return undefined;
  let flight = closureReasksV1.get(id);
  if (!flight) {
    const started = performance.now();
    flight = (async () => {
      assertNotReadParentV1("derive", recipe?.fn ?? "");
      const walked = reask === true ? await walkClosureRecipeV1(workspaceRoot, recipe, scripts, { farmOnly: true }) : await reask(workspaceRoot, recipe);
      if (walked === undefined) return undefined;
      const entry = await keepDerivedClosureV1(id, recipe, store, walked);
      entry.reasked = true; closureCountsV1.reasked++;
      for (const language of ["en", "de"]) { try { process.stderr.write("[owners] " + (language === "de" ? "Closure-Frage neu gestellt: Rezept " + id.split("\0")[1]?.slice(0, 12) + " in " + Math.round(performance.now() - started) + " ms" : "closure question asked again: recipe " + id.split("\0")[1]?.slice(0, 12) + " in " + Math.round(performance.now() - started) + " ms") + "\n"); } catch {} }
      return entry;
    })().catch(() => undefined).finally(() => closureReasksV1.delete(id));
    closureReasksV1.set(id, flight);
  }
  const entry = await new Promise((resolve, reject) => {
    const timer = setTimeout(() => resolve(undefined), INFERENCE_LIMITS_V1.closureReaskMilliseconds), abort = () => { clearTimeout(timer); reject(signal.reason); };
    signal?.addEventListener("abort", abort, { once: true });
    flight.then(found => { clearTimeout(timer); signal?.removeEventListener("abort", abort); resolve(found); });
  });
  if (entry === undefined) { closureCountsV1.reaskFailed++; failed?.add(id); }
  return entry;
}

/** 🪦️ The entries a take found stale, per round, with the stamp of the stored entry that was judged: the next look of the same round that meets the same stored entry is a miss at once, without loading and verifying it again (a take pass met the same stale shared entries in every owner: 13 084 stale verdicts over 582 looks, each a load of the entry file and a check of its layers). A round is one snapshot of the tree, so the verdict cannot change within it; a stored entry of another stamp is judged anew.
 * @see ./🧪️tests/🪦️stale-take */
const staleTakesV1 = new WeakMap();

/** ♻️ The entry of one recipe from the cache, a flight in progress, or a new detached computation; an unparseable source is remembered as a failure for as long as the facts it rests on hold. */
const closureFleetLanesV1 = new Map(), closureOffersFlushV1 = { registered: false };
/** 📤️ Waits until the offers this process queued for the fleet are stored (or refused) and its changed module graphs are saved into the machine's directory, for a caller that is about to end the process; the owners never wait for them. */
export async function flushClosureOffersV1() { for (const lane of [...closureFleetLanesV1.values()]) await (await lane)?.flush().catch(() => undefined); await flushClosureGraphsV1(); }
/** 🧶️ The fleet lane of closure entries of one workspace (see `🪪️graph-state/📦️project-records/🚚️entry-lane`), or undefined when no fleet cache is configured here or `SEMIO_CLOSURE_FLEET=off`; opened once. */
export function closureFleetLaneV1(workspaceRoot) {
  if (process.env.SEMIO_CLOSURE_FLEET === "off") return Promise.resolve(undefined);
  let lane = closureFleetLanesV1.get(workspaceRoot);
  if (lane === undefined) { lane = import("../../../🪪️graph-state/📦️project-records/🚚️entry-lane/🟦️.ts").then(module => { const directory = closureStoreBindingsV1.get(workspaceRoot)?.directory ?? closureStoreDirectoryV1(workspaceRoot), opened = module.createClosureEntryLaneV1({ root: workspaceRoot, environment: process.env, ...directory === undefined ? {} : { outbox: join(directory, "o"), documents: join(directory, "d") } }); if (opened !== undefined) { try { opened.adopt(closureTravelV1); } catch {} if (!closureOffersFlushV1.registered) { closureOffersFlushV1.registered = true; process.once("beforeExit", () => { void opened.flush().catch(() => undefined); }); } } return opened; }, () => undefined).catch(() => undefined); closureFleetLanesV1.set(workspaceRoot, lane); }
  return lane;
}
/** 🧶️ Takes the entries of `recipes` that this workspace's store lacks from the fleet in one batch, before any of them is derived; answers how many were taken. */
export async function prefetchClosureEntriesV1(workspaceRoot, recipes, signal = closureTakeV1.getStore()?.signal) {
  signal?.throwIfAborted();
  const store = closureStoreOfV1(workspaceRoot), lane = store ? await closureFleetLaneV1(workspaceRoot) : undefined;
  if (!lane) return 0;
  const missing = [];
  for (const recipe of recipes) { signal?.throwIfAborted(); const id = workspaceRoot + "\0" + closureRecipeKeyV1(recipe); if (closureEntriesV1.peek(id) === undefined && await store.stamp(id) === undefined) missing.push(recipe); }
  if (missing.length === 0) return 0;
  const taken = await lane.prefetch(missing, closureTravelV1, signal).catch(() => 0);
  signal?.throwIfAborted();
  closureCountsV1.fetched += taken;
  return taken;
}
/** 🪧️ The claim on the first derivation of one shared closure entry: the process that takes it derives the entry, the others wait for the stored entry and reuse it; undefined when the read has no verdict directory or no store. A claim is released when the derivation ends, so a later derivation of a stale entry is claimed anew. */
export async function claimDerivationV1(id, store, scripts) {
  const directory = closureVerdictScopeV1();
  if (directory === undefined || !store) return undefined;
  const signal = scripts?.context?.ownerSignal ?? scripts?.context?.phaseControl?.signal ?? scripts?.context?.invocation?.control?.signal;
  const file = join(directory, "claim-" + createHash("sha256").update(id).digest("hex").slice(0, 32));
  for (let tick = 0; tick < 1200; tick++) {
    signal?.throwIfAborted();
    let mine = false;
    try { verdictDirMadeV1(directory); vWriteV1(file, String(process.pid), { flag: "wx" }); mine = true; } catch {}
    if (mine) return { release: () => { try { vRmV1(file, { force: true }); } catch {} } };
    let holder; try { holder = Number(vReadV1(file, "utf8")); } catch {}
    if (Number.isInteger(holder) && holder > 0 && holder !== process.pid) { if (!processPidExistsV1(holder)) { try { vRmV1(file, { force: true }); } catch {} continue; } }
    const stored = await store.load(id), entry = stored && { ...stored, layers: undefined, store, answers: new Map(), checked: null };
    signal?.throwIfAborted();
    if (entry && await closureEntryHoldsAsyncV1(entry, scripts)) return { entry };
    try { await claimDelayV1(100, undefined, { signal }); }
    catch (error) { signal?.throwIfAborted(); throw error; }
  }
  return undefined;
}
/** 🗄️ Keeps a derived closure entry: its rows shared, the entry saved to the durable store (when there is one) and held in memory, counted as derived. */
async function keepDerivedClosureV1(id, recipe, store, { value, rows, layers }, started = undefined) { shareObservationRowsV1(rows); for (const layer of layers) shareObservationRowsV1(layer); closureCountsV1.derived++; const pass = closurePassV1, entry = { recipe, value, rows, layers, answers: new Map(), checked: pass ? { round: pass, holds: true } : null, bytes: 0, store, digests: undefined }; try { const saved = store ? await store.save(id, { ...entry, sidecar: closureSidecarV1(layers) }) : undefined; entry.bytes = saved?.bytes ?? closureEntryBytesV1(entry); entry.digests = saved?.digests; if (saved?.digests) { await stampKnownEntryV1(store, saved.digest, rows, layers, started); entry.sidecar = closureSidecarV1(layers); seedLayerDigestsV1(entry); entry.layers = undefined; } } catch { entry.bytes = closureEntryBytesV1(entry); } closureEntriesV1.set(id, entry); return entry; }

async function produceClosureEntryV1(workspaceRoot, recipe) {
  const id = workspaceRoot + "\0" + closureRecipeKeyV1(recipe), known = closureEntriesV1.get(id);
  if (known && await closureEntryHoldsAsyncV1(known, closureScriptsV1.get(workspaceRoot))) return known;
  const failed = closureFailuresV1.get(id);
  if (failed) { if (await closureEntryHoldsAsyncV1(failed, closureScriptsV1.get(workspaceRoot))) throw failed.error; closureFailuresV1.delete(id); }
  if (closureTakeV1.getStore() !== undefined && !closureInflightV1.has(id)) return takenClosureEntryV1(workspaceRoot, id, recipe);
  let flight = closureInflightV1.get(id);
  if (!flight) {
    const scripts = closureScriptsV1.get(workspaceRoot);
    if (!scripts) throw refuseBuildV1("missing", "closure.scripts-unbound", { workspace: String(workspaceRoot) });
    const signal = scripts.context?.ownerSignal ?? scripts.context?.phaseControl?.signal ?? scripts.context?.invocation?.control?.signal;
    signal?.throwIfAborted();
    const store = closureStoreOfV1(workspaceRoot), derive = () => { assertNotReadParentV1("derive", recipe?.fn ?? ""); return walkClosureRecipeV1(workspaceRoot, recipe, scripts); }, deriveAndKeep = () => { const started = Date.now(); return derive().then(derived => keepDerivedClosureV1(id, recipe, store, derived, started)); };
    flight = (store ? store.load(id) : Promise.resolve(undefined)).then(async stored => { signal?.throwIfAborted(); const entry = stored && { ...stored, layers: undefined, store, answers: new Map(), checked: null }; if (entry && await closureEntryHoldsAsyncV1(entry, scripts)) { signal?.throwIfAborted(); closureCountsV1.loaded++; closureEntriesV1.set(id, entry); return entry; } if (entry) closureCountsV1.stale++;
      const claim = await claimDerivationV1(id, store, scripts);
      try {
      signal?.throwIfAborted();
      if (claim?.entry) { closureCountsV1.loaded++; closureEntriesV1.set(id, claim.entry); return claim.entry; }
      const lane = store ? await closureFleetLaneV1(workspaceRoot) : undefined;
      signal?.throwIfAborted();
      if (!lane) return await deriveAndKeep();
      const taken = await lane.take(recipe, closureTravelV1, signal).catch(() => { signal?.throwIfAborted(); return undefined; });
      if (taken?.outcome === "taken") { const again = await store.load(id), next = again && { ...again, layers: undefined, store, answers: new Map(), checked: null }; if (next && await closureEntryHoldsAsyncV1(next, scripts)) { signal?.throwIfAborted(); closureCountsV1.fetched++; closureEntriesV1.set(id, next); return next; } }
      let derived; try { signal?.throwIfAborted(); derived = await deriveAndKeep(); } catch (error) { await taken?.hold?.release().catch(() => undefined); throw error; }
      if (derived.digests) lane.enqueue(recipe, closureTravelV1, taken?.hold); else await taken?.hold?.release().catch(() => undefined);
      return derived;
      } finally { claim?.release?.(); } }).then(entry => entry, error => { const facts = error?.code === "BUILD_SOURCE_UNPARSEABLE" ? detachedFailureV1(error) : undefined; if (facts) closureFailuresV1.set(id, { error, rows: facts.rows, layers: facts.layers, checked: null }); throw error; }).finally(() => closureInflightV1.delete(id));
    closureInflightV1.set(id, flight);
  }
  return flight;
}

/** 🔢️ How many finished owners share the walks of the command closures before the cache they fill is released: the walk of a closure of hundreds of modules costs more than the rest of an owner, so releasing it after every owner made each crate cost five times its own work, while the cache grows by a few megabytes per owner. */
const OWNERS_PER_RELEASE_V1 = INFERENCE_LIMITS_V1.ownersPerRelease;

/** 🔢️ How many finished owners keep the walk state they share while the host is short of memory: the walk nodes cost more than the owners that read them, so dropping them after every owner under a lasting shortage made each owner rebuild them (every project of a plan grew by the same several gigabytes); the per-owner state is dropped after every owner meanwhile. */
const OWNERS_PER_SHRINK_RELEASE_V1 = INFERENCE_LIMITS_V1.ownersPerShrinkRelease;
const FINISHED_OWNERS_V1 = new WeakMap();

/** 🫧️ Releases what finished owners left behind in the cache shared by the owners, so the memory of the owner phase stays bounded: every `OWNERS_PER_RELEASE_V1` owners, or after every owner while `shrink` (the host is short of memory), the source reads, the walk nodes (the module closure graph of the cache stays: it is what every later walk looks its answer up in, and its nodes are judged by their witnesses), closure walks, command dependency memos, declared module and TypeScript program memos and workspace path names are dropped and, when `shrink`, half of the in-process closure entries, which a later owner reads again from the durable store; the shared provider designation and the generated imports stay. The interval does not wait for the footprint: a worker that ran every owner of a plan below the soft share of its grant held 4.4 GB of managed memory after 262 owners (about 15 MB left by each) while the footprint of its host read 623 MB, so only `shrink` and the spill of entries follow the footprint. Shrinking after every owner under a lasting shortage unpacked and parsed the stored entries again for each owner (two thirds of the time of a refresh in a profile). */
export function releaseOwnerStateV1(scripts, { shrink = false, gate = footprintGateV1(), now = false } = {}) {
  const grant = gate?.grantBytes(), share = grant > 0 ? decideFootprintV1({ footprintBytes: gate.footprintBytes(), grantBytes: grant }) : undefined;
  if (share !== undefined) shrink = share === "wait";
  const finished = (FINISHED_OWNERS_V1.get(scripts) ?? 0) + 1;
  if (shrink) { scripts.files.clear(); scripts.closures.clear(); scripts.commandDependencies.clear(); scripts.groupLists?.clear(); }
  if (!now && finished < (shrink ? OWNERS_PER_SHRINK_RELEASE_V1 : OWNERS_PER_RELEASE_V1)) { FINISHED_OWNERS_V1.set(scripts, finished); return; }
  FINISHED_OWNERS_V1.set(scripts, 0);
  scripts.sharedBytes = 0;
  scripts.files.clear(); scripts.closures.clear(); scripts.commandDependencies.clear(); scripts.groupLists?.clear(); scripts.declared.clear(); scripts.paths.clear(); scripts.walkNodes?.clear(); scripts.toolGroups?.clear(); sharedSourceFilesV1.clear();
  if (shrink) closureEntriesV1.shrink(0.5);
}

const LOOK_LOWEST_V1 = new WeakMap();

/** 📈️ The release after a look follows the growth of the managed memory beyond its lowest reading since the last release (`lookReleaseGrowthBytes`), not a count of looks: a look at a stored record mostly reads the cache and leaves nothing, so a release every 32 looks only made the next looks rebuild the module graph (a burst of Rust owner looks moved the heap from 0.7 to 1.2 GB and the resident set from 1.9 to 5.7 GB each time); while `shrink` every look releases as before.
 * @see ./🧪️tests/🟦️.ts */
export function lookReleaseDueV1(lowestBytes, managedBytes, { shrink = false, limits = INFERENCE_LIMITS_V1 } = {}) {
  return shrink || managedBytes - lowestBytes >= limits.lookReleaseGrowthBytes;
}
/** 🫧️ What a look at an owner's stored record (the take pass: the owner is present, fetched from the fleet or missing, and nothing is inferred) leaves behind in the cache shared by the owners is released like what an inference leaves: the look resolves the closure references of the owner, which fills the module graph, the source reads and the closure walks of this cache, and a take pass over every owner of a cold machine (4 774 owners, never released before: about 16 MB of strings for every owner taken, 10 GB after 650) must not keep them. Safe beside other looks and inferences: they rebuild what is dropped. */
export function releaseAfterTakeLookV1(scripts, options) {
  if (scripts === undefined || scripts === null) return;
  const managed = managedBytesV1(), lowest = Math.min(LOOK_LOWEST_V1.get(scripts) ?? managed, managed);
  if (!lookReleaseDueV1(lowest, managed, options)) { LOOK_LOWEST_V1.set(scripts, lowest); return; }
  LOOK_LOWEST_V1.set(scripts, undefined);
  releaseOwnerStateV1(scripts, { ...options, now: true });
}

/** 📋️ The recipes every owner reads and none depends on its own target: the native command inputs, the group of every provider port in every view and every tool closure. */
export function sharedClosureRecipesV1(scripts) {
  const designation = providerDesignationV1(scripts), recipes = [{ fn: "native-command" }];
  for (const port of Object.keys(designation.providers).sort()) for (const view of Object.keys(PROVIDER_CLOSURE_POLICY_V1.views)) recipes.push({ fn: "script-provider", port, view });
  for (const name of Object.keys(PROVIDER_CLOSURE_POLICY_V1.tools.closures).sort()) recipes.push({ fn: "tool-closure", name, view: PROVIDER_CLOSURE_POLICY_V1.tools.closures[name].view });
  return recipes;
}

/** 🔢️ How many recipes the prime derives at once: the walk farm answers several recipes at a time, the walks of one recipe wait on the disk and on the farm, not on this thread. */
const PRIME_CONCURRENCY_V1 = 4;

/** 🧰️ Derives the shared closure entries (up to `PRIME_CONCURRENCY_V1` at once; before: one after the other, so the farm was used by one question at a time and a prime of 60 recipes took the sum of all of them) before the owners start: each is a durable memo node keyed by its recipe, so an owner only reads it and a process that finds it again derives nothing. Progress and cancellation follow the caller. */
export async function primeSharedClosuresV1(workspaceRoot, scripts, { signal, progress, recipes = sharedClosureRecipesV1(scripts), concurrency = PRIME_CONCURRENCY_V1 } = {}) {
  bindClosureScriptsV1(workspaceRoot, scripts);
  const before = closureEntryCountsV1();
  let done = 0, next = 0;
  await progress?.({ done, total: recipes.length });
  await Promise.all(Array.from({ length: Math.max(1, Math.min(concurrency, recipes.length)) }, async () => {
    while (next < recipes.length) {
      signal?.throwIfAborted();
      const recipe = recipes[next++];
      knownClosureRecipesV1.set(closureRecipeKeyV1(recipe), recipe);
      await ensureProviderEntriesV1(workspaceRoot, await ensureClosureEntryV1(workspaceRoot, recipe));
      await progress?.({ done: ++done, total: recipes.length });
    }
  }));
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

/** 🔁️ Answers a query over a closure again, after `refresh` brought its shared provider entries up to date, while the answer is undefined (a provider file changed between the check that admitted the entry and the check that expanded it: the files of the hot tool directories change while peers work), at most `attempts` times; undefined when the last answer is still undefined. A pure function of its arguments.
 * @see ../🧬️schema/🔣️.json */
export async function answeredAgainstFreshProvidersV1(answer, refresh, attempts = INFERENCE_LIMITS_V1.closureAnswerAttempts) {
  for (let attempt = 0; ; attempt++) {
    const value = answer();
    if (value !== undefined || attempt >= attempts) return value;
    await refresh();
  }
}

const forgetVerdictV1 = file => { if (file === undefined) return; verdictSeenV1.delete(file); try { vRmV1(file, { force: true }); } catch {} };
/** 🫚️ Drops what a read holds of a lost layer: its verdicts (this round and the verdict files of the read, so the next look judges the layer by what the store holds then), and every in-memory entry of the workspace that names it (or whose layers are not known), so that the next ask loads the stored entry, finds it stale and derives it or takes it from the fleet. Answers how many entries were dropped. */
function forgetLostLayerV1(workspaceRoot, error) {
  const digest = error.layerDigest, store = closureStoreOfV1(workspaceRoot), round = closurePassV1 ?? observationRoundV1(), held = [], lost = new Set(digest === undefined ? [] : [digest]);
  for (const key of closureEntriesV1.keys()) {
    const entry = closureEntriesV1.peek(key);
    if (!entry || !key.startsWith(workspaceRoot + "\0")) continue;
    if (digest === undefined ? entry.layers === undefined && entry.digests === undefined : entry.digests?.includes(digest)) { held.push([key, entry]); for (const missing of store?.lostLayersSync?.(entry.digests ?? []) ?? []) lost.add(missing); }
  }
  for (const missing of lost) { forgetVerdictV1(verdictFileV1("l", missing)); if (round) layerVerdictsV1.get(round)?.delete(missing); }
  for (const [key, entry] of held) { entry.checked = null; forgetVerdictV1(verdictFileV1("e", entry.digest)); closureEntriesV1.delete(key); }
  return held.length;
}
/** 🪈️ Runs `body` (a query over command closures that reads stored layers) again, against entries read anew, while a layer it read was lost, at most `attempts` times, each time saying why in English and German; the refusal of the last attempt is thrown with `closureLayerLost`, which the reader of the owner answers by requeueing the owner once. Everything else is thrown at once.
 * @see ../🧬️schema/🔣️.json */
export async function answeredAgainstLiveLayersV1(workspaceRoot, body, { attempts = INFERENCE_LIMITS_V1.closureLayerAttempts, say = line => { try { process.stderr.write("[closure] " + line + "\n"); } catch {} } } = {}) {
  for (let attempt = 0; ; attempt++) {
    try { return await body(); }
    catch (error) {
      if (error?.closureLayerLost !== true) throw error;
      const dropped = forgetLostLayerV1(workspaceRoot, error), layer = String(error.layerDigest ?? "?").slice(0, 12), farmed = readParentV1();
      if (farmed || attempt >= attempts) {
        if (farmed) say("closure layer " + layer + " is gone from the store, " + dropped + " entries forgotten, the farm derives them again when the owner is asked again / Closure-Ebene " + layer + " fehlt im Speicher, " + dropped + " Einträge vergessen, die Farm leitet sie neu ab, wenn der Besitzer erneut gefragt wird");
        throw error;
      }
      say("closure layer " + layer + " is gone from the store, " + dropped + " entries read anew, asking again " + (attempt + 1) + " of " + attempts + " / Closure-Ebene " + layer + " fehlt im Speicher, " + dropped + " Einträge neu gelesen, erneute Abfrage " + (attempt + 1) + " von " + attempts);
    }
  }
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
  async verify(rows, value, failed) {
    const signal = closureTakeV1.getStore()?.signal;
    signal?.throwIfAborted();
    prepareSourceAnalysesV1();
    for (const [key, recipe] of Object.entries(value?.closureRecipes ?? {})) knownClosureRecipesV1.set(key, recipe);
    for (const [digest, inputs] of Object.entries(value?.closureInputs ?? {})) knownClosureInputsV1.set(digest, inputs);
    const groups = new Map();
    for (const row of rows) {
      let workspaceRoot, key, query;
      try { [workspaceRoot, key, query] = JSON.parse(row.path); } catch { failed?.(row, "closure-query"); return false; }
      const id = workspaceRoot + "\0" + key, group = groups.get(id) ?? { workspaceRoot, key, queries: [] };
      group.queries.push([query, row]);
      groups.set(id, group);
    }
    for (const { workspaceRoot, key, queries } of groups.values()) {
      signal?.throwIfAborted();
      const recipe = knownClosureRecipesV1.get(key);
      if (!recipe) { failed?.(undefined, "closure-recipe-unavailable"); return false; }
      try {
        let entry = await ensureClosureEntryV1(workspaceRoot, recipe), providers = await ensureProviderEntriesV1(workspaceRoot, entry);
        const reasked = entry.reasked === true;
        for (const [query, row] of queries) {
          signal?.throwIfAborted();
          const answer = await answeredAgainstFreshProvidersV1(() => closureAnswerV1(workspaceRoot, entry, query, providers), async () => { entry = await ensureClosureEntryV1(workspaceRoot, recipe); providers = await ensureProviderEntriesV1(workspaceRoot, entry); });
          if (answer === undefined || JSON.stringify(answer) !== row.fact) { if (reasked) closureCountsV1.reaskDifferent++; failed?.(row, answer === undefined ? "closure-unavailable" : "closure-value"); return false; }
          if (reasked) closureCountsV1.reaskEqual++;
        }
      } catch (error) { signal?.throwIfAborted(); failed?.(error?.movedRow ?? queries[0]?.[1], closureFailureWhyV1(error, key)); return false; }
    }
    return true;
  },
  async settle(paths, value) {
    const signal = closureTakeV1.getStore()?.signal;
    signal?.throwIfAborted();
    prepareSourceAnalysesV1();
    for (const [key, recipe] of Object.entries(value?.closureRecipes ?? {})) knownClosureRecipesV1.set(key, recipe);
    for (const [digest, inputs] of Object.entries(value?.closureInputs ?? {})) knownClosureInputsV1.set(digest, inputs);
    const wanted = new Map();
    for (const path of paths) { try { const [workspaceRoot, key] = JSON.parse(path), recipe = knownClosureRecipesV1.get(key); if (recipe) wanted.set(workspaceRoot + "\0" + key, [workspaceRoot, recipe, key]); } catch {} }
    const byRoot = new Map();
    for (const [workspaceRoot, recipe] of wanted.values()) { byRoot.set(workspaceRoot, [...byRoot.get(workspaceRoot) ?? [], recipe]); }
    for (const [workspaceRoot, recipes] of byRoot) { signal?.throwIfAborted(); try { await prefetchClosureEntriesV1(workspaceRoot, recipes, signal); } catch { signal?.throwIfAborted(); } }
    for (const [workspaceRoot, recipe, key] of wanted.values()) { signal?.throwIfAborted(); try { await ensureProviderEntriesV1(workspaceRoot, await ensureClosureEntryV1(workspaceRoot, recipe)); } catch { signal?.throwIfAborted(); closureEntriesV1.delete(workspaceRoot + "\0" + key); } }
  },
});

/** 📑️ The command inputs of one inference: in the late mode symbolic lists whose queries are recorded as facts, otherwise the lists themselves; project assembly reads both through this one interface. */
export function closureApiV1(scripts, workspaceRoot) {
  const late = scripts.lateClosures === true;
  if (late) bindClosureScriptsV1(workspaceRoot, scripts);
  const used = new Map(), fact = async (recipe, query) => { const key = closureRecipeKeyV1(recipe); knownClosureRecipesV1.set(key, recipe); used.set(key, recipe); let entry = await ensureClosureEntryV1(workspaceRoot, recipe), providers = await ensureProviderEntriesV1(workspaceRoot, entry); const answer = await answeredAgainstFreshProvidersV1(() => closureAnswerV1(workspaceRoot, entry, query, providers), async () => { entry = await ensureClosureEntryV1(workspaceRoot, recipe); providers = await ensureProviderEntriesV1(workspaceRoot, entry); }); if (answer === undefined) throw Object.assign(refuseBuildV1("invalid", "closure.provider-entry-stale", { port: sharedReferencesOfV1(entry.value).map(row => row.recipe.port ?? row.recipe.name).join(", ") }), { storeChanged: true }); recordClosureFactV1(JSON.stringify([workspaceRoot, key, query]), JSON.stringify(answer)); return answer; };
  const expanded = value => expandedCommandInputsV1(value, workspaceRoot, scripts);
  const relativeEntries = entries => entries.map(entry => relative(workspaceRoot, entry).split(sep).join("/"));
  const sharedClosureGroupReaders = (projectRoot, declared) => {
    const designation = providerDesignationV1(scripts);
    if (resolve(workspaceRoot, projectRoot, declared) !== resolve(designation.path)) throw refuseBuildV1("invalid", "closure.provider-designation-differs", { designation: String(declared) });
    const readers = [];
    for (const port of Object.keys(designation.providers).sort()) for (const view of Object.keys(PROVIDER_CLOSURE_POLICY_V1.views)) readers.push([providerGroupNameV1(port, view), () => late ? closureHandleV1({ fn: "script-provider", port, view }) : scriptProviderGroupV1(workspaceRoot, scripts, port, view)]);
    for (const name of Object.keys(PROVIDER_CLOSURE_POLICY_V1.tools.closures).sort()) readers.push([toolGroupNameV1(name), () => late ? closureHandleV1({ fn: "tool-closure", name, view: PROVIDER_CLOSURE_POLICY_V1.tools.closures[name].view }) : toolClosureGroupV1(workspaceRoot, scripts, name)]);
    return readers;
  };
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
    sharedClosureGroupReaders,
    async sharedClosureGroups(projectRoot, declared) {
      const groups = [];
      for (const [name, read] of sharedClosureGroupReaders(projectRoot, declared)) groups.push([name, await read()]);
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
    resolve(recipes, log) {
      return answeredAgainstLiveLayersV1(workspaceRoot, async () => {
      const values = new Map(), derived = new Map(), layers = new Map(), seen = new Set(), rows = [], take = row => { const key = rowKeyV1(row); if (!seen.has(key)) { seen.add(key); rows.push(row); } };
      const builder = log?.builder(), addRows = items => { if (builder) builder.add(items); else for (const row of items) take(row); }, addLayer = view => { if (builder) builder.addList(closureLayerListV1(log, view)); else for (const batch of closureLayerBatchesV1(view)) addRows(batch); }, supportOf = view => log ? closureLayerListV1(log, view) : view.rows ?? [...closureLayerBatchesV1(view)].flat();
      const entries = [];
      for (const [key, recipe] of recipes) { const entry = await ensureClosureEntryV1(workspaceRoot, recipe), view = walkViewOfRecipeV1(recipe); values.set(key, entry.value); entries.push(entry); for (const layer of closureLayerViewsV1(entry)) { layers.set(layer.key, layer); if (layer.fact) { const row = view === undefined ? layer.fact.row : layer.fact.views?.[view] ?? layer.fact.row; derived.set(row.path, { row, support: supportOf(layer) }); } } }
      for (const { row, support } of derived.values()) { if (row.path.startsWith('{"program":')) continue; if (builder) builder.exclude(support); else for (const supported of support) seen.add(rowKeyV1(supported)); }
      for (const entry of entries) addRows(entry.rows);
      for (const layer of layers.values()) if (!layer.fact) addLayer(layer);
      return { values, rows: builder ? builder.finish() : rows, derived: [...derived.values()] };
      });
    },
    /** 🫗️ The shape of {@link resolve} for a process that writes no row log: the values of the recipes and, instead of rows, the plan a log is made from (the rows of the entries, the digests of the stored layers to add, the digests whose rows a derived fact stands for, the derived facts with the digest of their layer), so that the process that owns the log lists the rows from the closure store without opening one entry. Undefined when a layer is not stored (it has no digest). */
    plan(recipes) {
      return answeredAgainstLiveLayersV1(workspaceRoot, async () => {
      const values = new Map(), derived = new Map(), layers = new Map(), entries = [];
      for (const [key, recipe] of recipes) { const entry = await ensureClosureEntryV1(workspaceRoot, recipe), view = walkViewOfRecipeV1(recipe); values.set(key, entry.value); entries.push(entry); for (const layer of closureLayerViewsV1(entry)) { if (typeof layer.digest !== "string") return undefined; layers.set(layer.key, layer); if (layer.fact) { const row = view === undefined ? layer.fact.row : layer.fact.views?.[view] ?? layer.fact.row; derived.set(row.path, { row, digest: layer.digest }); } } }
      const entryRows = []; for (const entry of entries) for (const row of entry.rows) entryRows.push(row);
      return { values, plan: { entryRows, excluded: [...derived.values()].filter(({ row }) => !row.path.startsWith('{"program":')).map(({ digest }) => digest), layers: [...layers.values()].filter(layer => !layer.fact).map(layer => layer.digest), derived: [...derived.values()] } };
      });
    },
    /** 🫗️ The rows and derived facts of a {@link plan} as lists of `log`, the layers listed once per log from the stored ids (no entry is loaded, nothing is derived); throws when a stored layer is gone, so the caller resolves itself. */
    adopt(plan, log) {
      const store = closureStoreOfV1(workspaceRoot);
      if (!store) throw layerLostRefusalV1(undefined);
      const listOf = digest => closureLayerListV1(log, { key: digest, digest, store }), builder = log.builder();
      for (const digest of plan.excluded) builder.exclude(listOf(digest));
      builder.add(plan.entryRows);
      for (const digest of plan.layers) builder.addList(listOf(digest));
      return { rows: builder.finish(), derived: plan.derived.map(({ row, digest }) => ({ row, support: listOf(digest) })) };
    },
  });
}

/** 🗄️ The group a shared reference stands for (a tool closure or a script provider of the providers project), derived once per script input cache and replayed with its observed rows into every project that expands it again; the 17 targets of a crate expanded the same 18 groups 690 times, each time walking the 1800 modules of the cargo command again. A group that is being derived is awaited, then replayed, so concurrent expansions neither derive twice nor lose the rows. Any other recipe is produced as before.
 * @see ./🧪️tests/🗂️shared-groups/🟦️.ts */
async function sharedGroupV1(recipe, workspaceRoot, scripts) {
  if (recipe.fn !== "tool-closure" && recipe.fn !== "script-provider") return produceClosureRecipeV1(recipe, workspaceRoot, scripts);
  let index = 0;
  const key = closureRecipeKeyV1(recipe), cache = scripts.groupLists ??= new Map(), flights = scripts.groupFlights ??= new Map();
  const run = () => memoizedObservedAsyncComputationV1(cache, key, () => { scripts.groupComputations = (scripts.groupComputations ?? 0) + 1; return produceClosureRecipeV1(recipe, workspaceRoot, scripts); }, (force = false) => yieldInferenceV1(scripts.context, "build:command-source-packages", ++index, force));
  const flight = flights.get(key);
  if (flight !== undefined) { await flight.catch(() => {}); return run(); }
  const mine = run();
  flights.set(key, mine);
  try { return await mine; } finally { if (flights.get(key) === mine) flights.delete(key); }
}

/** 📇️ A command input list (or a target-sources value) with every shared reference (a script provider or tool closure of the providers project, `./🔌️provider-closures`) replaced by the files of its group: what a consumer that needs the literal files of a command closure reads (a read oracle, a law, a query) instead of the references a node holds. */
export async function expandedCommandInputsV1(value, workspaceRoot, scripts = createScriptInputCache()) {
  const references = sharedReferencesOfV1(value);
  if (!references.length) return value;
  const files = [];
  for (const reference of references) for (const item of await sharedGroupV1(reference.recipe, workspaceRoot, scripts)) if (typeof item === "string" && item.startsWith("{workspaceRoot}/")) files.push(item);
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

/** ⏳️ Delegates bounded inference work to the caller's original progress and continuation control, and is the point where the inference of an owner that grew past what the machine can give one project stops (`throwIfOwnerExceededV1`, set by the memory governor); durable progress is published at most every 200 ms so its cost never scales with the number of inferred facts. */
export async function yieldInferenceV1(context,phase,index,force=false){
 context.ownerSignal?.throwIfAborted();
 footprintPulseV1();releaseIfGrownV1(closureTakeV1.getStore() !== undefined ? { every: 16, threshold: FOOTPRINT_POLICY_V1.takeReleaseGrowthBytes } : { every: 16 });
 throwIfOwnerExceededV1();
 if(!context.invocation)return;
 const last=yieldInferenceV1.last??=new WeakMap(),now=performance.now();
 if(!force&&now-(last.get(context.invocation)??-Infinity)<INFERENCE_LIMITS_V1.advanceThrottleMilliseconds){context.invocation.control.signal.throwIfAborted();return;}
 last.set(context.invocation,now);
 requireIssuedResourceControlInvocationV1(context.phaseControl,context.invocation);checkScriptInvocation(context.invocation);checkResourceLeaseControl(context.phaseControl);await advanceScriptInvocation(context.invocation,phase,"running");checkResourceLeaseControl(context.phaseControl);
}
