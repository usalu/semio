/** 🕸️ Assembles the build inference plugin from its revisioned components and loads a complete new component graph whenever any authored Source of the module changes. */
import {createRequire} from "node:module";
const javascriptManagerV1 = createRequire(import.meta.url)("../../../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🟨️.cjs").selectedManagerV1("javascript");
import { registerSourceAnalysisV1 } from "../../../🏗️build/👁️observation/🟨️.mjs";
import { fileURLToPath } from "node:url";
import { createRevisionIdentity, createRevisionLoader, readComponentRoster, wireRevisionedComponents } from "./🪢️revision-graph/🟨️.mjs";
import manifest from "./🗂️components/🔣️.json" with { type: "json" };
import * as coordinates from "./📍️coordinates/🟨️.mjs";
import * as stamps from "./🔖️stamps/🟨️.mjs";
import * as bootstrap from "./🔁️bootstrap/🟨️.mjs";
import * as sourceAnalyses from "./🧬️source-analyses/🟨️.mjs";
import * as bunLock from "./🧶️bun-lock/🟨️.mjs";
import * as targetPolicy from "./🎯️target-policy/🟨️.mjs";
import * as nativeManifests from "./🦀️native-manifests/🟨️.mjs";
import * as scriptInputs from "./🧭️script-inputs/🟨️.mjs";
import * as generatorOutputs from "./🏭️generator-outputs/🟨️.mjs";
import * as componentTargets from "./🧩️component-targets/🟨️.mjs";
import * as playgroundTargets from "./🎮️playground-targets/🟨️.mjs";
import * as projectAssembly from "./🧱️project-assembly/🟨️.mjs";
import * as ownerDiscovery from "./🔎️owner-discovery/🟨️.mjs";
import * as projectNodes from "./📦️project-nodes/🟨️.mjs";
import * as dependencyEdges from "./🕸️dependency-edges/🟨️.mjs";
import * as revision from "./♻️revision/🟨️.mjs";
import * as internals from "./🧰️internals/🟨️.mjs";
import * as invocation from "./🚦️invocation/🟨️.mjs";

/** 🗂️ The roster of components this module is made of: the manifest orders them, names their needs and locates each one for a revisioned reload. Adding or removing a component edits the static imports below and therefore takes effect when this module is loaded again; every other Source edit reloads in process. */
const MANIFEST = "./🗂️components/🔣️.json";
const KERNEL = "./🪢️revision-graph/🟨️.mjs";
const roster = readComponentRoster(manifest);
const staticModules = Object.freeze({
  coordinates,
  stamps,
  bootstrap,
  "source-analyses": sourceAnalyses,
  "bun-lock": bunLock,
  "target-policy": targetPolicy,
  "native-manifests": nativeManifests,
  "script-inputs": scriptInputs,
  "generator-outputs": generatorOutputs,
  "component-targets": componentTargets,
  "playground-targets": playgroundTargets,
  "project-assembly": projectAssembly,
  "owner-discovery": ownerDiscovery,
  "project-nodes": projectNodes,
  "dependency-edges": dependencyEdges,
  revision,
  internals,
  invocation,
});
if (roster.length !== Object.keys(staticModules).length || roster.some((entry) => staticModules[entry.name]?.component !== entry.name)) throw Error("Inference component roster differs from the statically imported components");

/** 🔏️ Every source file of this module, in roster order, so the implementation revision covers the complete component graph. */
const sourceFiles = Object.freeze([import.meta.url, new URL(KERNEL, import.meta.url), new URL(MANIFEST, import.meta.url), ...roster.map((entry) => new URL(entry.path, import.meta.url))].map((url) => fileURLToPath(url)).concat(createRequire(import.meta.url)("../../../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🟨️.cjs").sourceFilesV1()));

/** 📇️ Registers each source analysis once per identity and revision, so graphs of the same revision share one provider instead of re-registering its id. */
const latestRegistrations = new Map();
function register(analysis) {
  const prior = latestRegistrations.get(analysis.id);
  if (prior?.revision === analysis.revision) return prior.provider;
  const provider = registerSourceAnalysisV1(analysis);
  latestRegistrations.set(analysis.id, { revision: analysis.revision, provider });
  return provider;
}

/** 🔁️ Imports one component specifier relative to this directory, keeping its query so every revision gets its own module instances in Bun and Node. */
const load = (specifier) => import(specifier);

/** 🧰️ What a reloaded implementation offers to the graph that requested it. */
const implementationOf = (scope) => ({ cacheInternals: scope.modules.internals.cacheInternals, invokeCurrentImplementation: scope.modules.invocation.invokeCurrentImplementation });

const services = Object.freeze({ sourceFiles, loadRevision: (revision) => loader.load(revision).then(implementationOf) });
const loader = createRevisionLoader({ roster, load, services, register, url: import.meta.url });
const revisionIdentity = createRevisionIdentity(import.meta.url);
const current = wireRevisionedComponents({ roster, modules: staticModules, services, revisionIdentity, register });
revisionIdentity.bind(current.modules.revision.implementationRevision());
loader.adopt(revisionIdentity.revision, current);

/** 📋️ The authored files of the implementation this module loaded with the bytes they had at load. */
export const loadedImplementationRowsV1 = Object.freeze(current.modules.revision.implementationRows());

/** 🔖️ Turns implementation rows into the facts a generation names: the declared stamp of each inferrer and the content of the data no inferrer covers. */
export const classifyImplementationRowsV1 = current.modules.stamps.classifyImplementationRowsV1;

/** 🚧️ Names the loaded executable files that no inferrer and no infrastructure fragment of the protocol claims. */
export const unclassifiedImplementationCodeV1 = current.modules.stamps.unclassifiedCodeV1;

/** 📜️ The declared inference protocol of the loaded revision. */
export const inferenceProtocolV1 = current.modules.stamps.inferenceProtocolV1;

const { invokeCurrentImplementation } = current.modules.invocation;

/** 🧬️ Binds structural parsers to their evaluated implementation and compiler bytes. */
export const prepareSourceAnalysesV1 = current.modules["source-analyses"].prepareSourceAnalysesV1;

/** 🗂️ Shares source reads and command closures only within one graph construction. */
export const createScriptInputCache = current.modules["script-inputs"].createScriptInputCache;

/** 👁️ Projects actual Cargo/Nx compilation and owner generator inputs for live source watching. */
export const nativeSourceWatchPlanV1 = current.modules["project-assembly"].nativeSourceWatchPlanV1;

/** 🔎️ Queries source-project normalization through the same production stages without publishing graph facts. */
export const inferReadOnlyBuildProjectSourcesV1 = current.modules["project-assembly"].inferReadOnlyBuildProjectSourcesV1;

/** 🪪️ Expands actual native owners through the same read-only phase and original caller controls. */
export const discoverNativeBuildOwnersV1 = current.modules["owner-discovery"].discoverNativeBuildOwnersV1;

/** 🚪️ Observes one canonical owner and only its selected executable source closures. */
export const discoverBuildOwnerAdmissionFactsV1 = current.modules["owner-discovery"].discoverBuildOwnerAdmissionFactsV1;

/** 📜️ Reads the workspace's canonical owner declaration through original physical Source ancestry. */
export const readBuildOwnerRegistrySourceV1 = current.modules["owner-discovery"].readBuildOwnerRegistrySourceV1;

/** 🔏️ Names the revision of the Source this module has on disk right now. */
export const currentImplementationRevisionV1 = current.modules.revision.implementationRevision;

/** 🔁️ Resolves the implementation of the Source this module has on disk right now: this graph while nothing changed, otherwise the freshly loaded graph of the new revision, so consumers never hold stale components. */
export async function loadCurrentImplementationV1() {
  const revision = current.modules.revision.implementationRevision();
  const scope = await loader.load(revision);
  return { revision, cacheInternals: scope.modules.internals.cacheInternals, libraryBootstrap: scope.modules.bootstrap.libraryBootstrap, createScriptInputCache: scope.modules["script-inputs"].createScriptInputCache };
}

/** 🏛️ Supplies current roots, manifests, lock scope, and actual implementation identity behind an owned digest interface. */
export function dependencyResolutionAuthority(...args) { return invokeCurrentImplementation("authority", args); }

export function createDependencies(...args) { return invokeCurrentImplementation("dependencies", args); }

/** 📇️ Exposes original complete module ownership independently of scheduling dependencies. */
export function createMetadata(...args){return invokeCurrentImplementation("metadata",args);}

/**
 * 🧩️ One Build project configuration this plugin generates, as its consumers read it.
 * @typedef {{ targets: Record<string, { cache?: boolean, continuous?: boolean, dependsOn?: string[], outputs?: string[], inputs?: unknown[], namedInputs?: Record<string, unknown[]>, options?: Record<string, unknown>, metadata?: Record<string, unknown> }>, root?: string, name?: string, projectType?: string, tags?: string[], sourceRoot?: string }} GeneratedProject
 */

/**
 * 🧩️ The `createNodesV2` result: one entry per matched file, each carrying the projects it contributes.
 * @typedef {[string, { projects: Record<string, GeneratedProject> }][]} GeneratedNodes
 */

/**
 * 🎛️ Receives the OS invocation or preserves an explicitly supplied Original and its mandatory issued phase.
 * @typedef {{workspaceRoot:string} & ({invocation?:undefined,phaseControl?:undefined}|{invocation:import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation,phaseControl:import("../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts").ResourceLeaseControl})} BuildInferenceContextV1
 */

export default {
  name: "@repo/emoji-project-json",
  createNodesV2: /** @type {[string, (files: readonly string[], options: unknown, context: BuildInferenceContextV1) => Promise<GeneratedNodes>]} */ ([`**/{${current.modules.coordinates.PROJECT_BASENAME},Cargo.toml,${javascriptManagerV1.lockFileNames.join(",")},*.patch}`, (...args) => invokeCurrentImplementation("nodes", args)]),
  createDependencies,
  createMetadata,
};

/** 🔁️ Settles once this revision's helper modules and parsers are loaded. */
export const libraryBootstrap = current.modules.bootstrap.libraryBootstrap;

export const cacheInternals = current.modules.internals.cacheInternals;

/** 🏗️ Discovers owned project contributions without loading a task-runner runtime. */
export async function discoverBuildProjectsV1(files, options, context) {
  return invokeCurrentImplementation("nodes",[files,options,context]);
}

/** 🪢️ Resolves owned dependency contributions without loading a task-runner runtime. */
export async function discoverBuildDependenciesV1(options, context) {
  return invokeCurrentImplementation("dependencies",[options,context]);
}
