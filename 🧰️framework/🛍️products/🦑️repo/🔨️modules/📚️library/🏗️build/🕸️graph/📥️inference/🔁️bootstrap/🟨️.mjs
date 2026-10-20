/** 🔁️ Loads the revision-pinned helper modules once and publishes them through the shared `loaded` record. */
import { requireCurrentSourceInventoryV1 } from "../../../../🏗️build/📥️inputs/📋️data/📥️load/🟨️.mjs";
import { requireCurrentObservationV1 } from "../../../../🏗️build/👁️observation/🟨️.mjs";
import { createHash } from "node:crypto";
import { createRequire } from "node:module";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import * as loadedCommandInputs from "../../../../⚡️caching/📥️inference/🟨️.mjs";
import * as loadedProjectInputs from "../../../../🏗️build/📥️inputs/🟨️.mjs";
import * as loadedBuildMode from "../../../../🏗️build/🎚️mode/🟨️.mjs";

export const component = "bootstrap";
let LIBRARY_ROOT, LIBRARY_URL, RUNTIME_COMPONENT_MODULE, NATIVE_HOST_MODULE, COMPONENT_DEPLOYMENT_MODULE, INSTALLATION_IDENTITY_MODULE, INSTALLATION_IDENTITY_SCHEMA, BROWSER_SESSION_MODULE, PLAYGROUND_COMPOSITION_MODULE, SOURCE_INPUT_MODULE, COMMAND_INPUT_MODULE, PROJECT_INPUT_MODULE, BUILD_MODE_MODULE, readPhysicalSource;
export let libraryBootstrap;

/** 🔁️ Evicts Bun's canonical module entry and retains Node's revision-specific ESM identity. */
export async function importRevision(source, revision) {
  const require = createRequire(import.meta.url), url = new URL(source);
  delete require.cache[require.resolve(fileURLToPath(url))];
  url.searchParams.set("revision", revision);
  return import(url.href);
}

/** 🧷️ Admits only the command input parsers whose bytes are the current authority. */
function currentCommandInputs(commandInputs) {
  if (commandInputs.sourceHash !== loaded.commandInputHash) throw Error("Stale command input parser implementation");
  return commandInputs;
}

/** 📦️ The helper modules and parsers this revision loaded, readable by every later component once `libraryBootstrap` settles. */
export const loaded = {
  commandInputHash: undefined,
  commandSourceImports: (path, source) => currentCommandInputs(loadedCommandInputs).commandSourceImports(path, source),
  runtimeModuleSourceImports: (path, source) => currentCommandInputs(loadedCommandInputs).runtimeModuleSourceImports(path, source),
  commandUrlReferences: (path, source) => currentCommandInputs(loadedCommandInputs).commandUrlReferences(path, source),
  moduleSourceImports: (path, source) => currentCommandInputs(loadedCommandInputs).moduleSourceImports(path, source),
  productionModuleSourceImports: (path, source) => currentCommandInputs(loadedCommandInputs).productionModuleSourceImports(path, source),
  declaredBrowserSessionEnginesV1: undefined,
  playgroundCompositionPathV1: undefined,
  admitPlaygroundNativeHostV1: undefined,
  nativeHostSourceFactsV1: undefined,
  nativeHostScriptAdmissionV1: undefined,
  runtimeComponentClosure: undefined,
  runtimeInputAdmissionV1: undefined,
  readSourceInputContract: undefined,
  relativeSourceInputs: undefined,
  relativeSourceInputsAsyncV1: undefined,
  componentDeploymentDirectoryV1: undefined,
  inferProjectInputs: loadedProjectInputs.inferProjectInputs,
  compactSourceInputs: loadedProjectInputs.compactSourceInputs,
  cargoLockInput: loadedProjectInputs.cargoLockInput,
  deduplicateProjectInputs: loadedProjectInputs.deduplicateProjectInputs,
  targetBuildModes: loadedBuildMode.targetBuildModes,
};

/** 🔌️ Binds the shared coordinates and starts the asynchronous helper load of this revision. */
export function wire(registry) {
  ({ LIBRARY_ROOT, LIBRARY_URL, RUNTIME_COMPONENT_MODULE, NATIVE_HOST_MODULE, COMPONENT_DEPLOYMENT_MODULE, INSTALLATION_IDENTITY_MODULE, INSTALLATION_IDENTITY_SCHEMA, BROWSER_SESSION_MODULE, PLAYGROUND_COMPOSITION_MODULE, SOURCE_INPUT_MODULE, COMMAND_INPUT_MODULE, PROJECT_INPUT_MODULE, BUILD_MODE_MODULE, readPhysicalSource } = registry.coordinates);
  loaded.commandInputHash = createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT, COMMAND_INPUT_MODULE))).digest("hex");
  libraryBootstrap = Promise.resolve().then(()=>{requireCurrentSourceInventoryV1();requireCurrentObservationV1();return (loadedCommandInputs.sourceHash === loaded.commandInputHash ? Promise.resolve(loadedCommandInputs) : importRevision(new URL(`./${COMMAND_INPUT_MODULE}`, LIBRARY_URL), loaded.commandInputHash));}).then((commandInputs) => {
    const inputRevision = createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT, PROJECT_INPUT_MODULE))).digest("hex");
    const modeRevision = createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT, BUILD_MODE_MODULE))).digest("hex");
    return Promise.all([importRevision(new URL(`./${PROJECT_INPUT_MODULE}`, LIBRARY_URL), inputRevision), importRevision(new URL(`./${BUILD_MODE_MODULE}`, LIBRARY_URL), modeRevision)]).then(([inputs, mode]) => {
      ({ inferProjectInputs: loaded.inferProjectInputs, compactSourceInputs: loaded.compactSourceInputs, cargoLockInput: loaded.cargoLockInput, deduplicateProjectInputs: loaded.deduplicateProjectInputs } = inputs); ({ targetBuildModes: loaded.targetBuildModes } = mode);
      return commandInputs;
    });
  }).then((commandInputs) => {
    const admitted = currentCommandInputs(commandInputs);
    loaded.commandSourceImports = admitted.commandSourceImports;
    loaded.runtimeModuleSourceImports = admitted.runtimeModuleSourceImports;
    loaded.commandUrlReferences = admitted.commandUrlReferences;
    loaded.moduleSourceImports = admitted.moduleSourceImports;
    loaded.productionModuleSourceImports = admitted.productionModuleSourceImports;
    return importRevision(new URL(`./${RUNTIME_COMPONENT_MODULE}`, LIBRARY_URL), createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT, RUNTIME_COMPONENT_MODULE))).digest("hex"));
  }).then((runtime) => {
    loaded.runtimeComponentClosure = runtime.runtimeComponentClosure;
    loaded.runtimeInputAdmissionV1 = runtime.runtimeInputAdmissionV1;
    return importRevision(new URL(`./${SOURCE_INPUT_MODULE}`, LIBRARY_URL), createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT, SOURCE_INPUT_MODULE))).digest("hex"));
  }).then((source) => {
    loaded.readSourceInputContract = source.readSourceInputContract;
    loaded.relativeSourceInputs = source.relativeSourceInputs;
    loaded.relativeSourceInputsAsyncV1 = source.relativeSourceInputsAsyncV1;
    return importRevision(new URL(`./${NATIVE_HOST_MODULE}`, LIBRARY_URL),createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT,NATIVE_HOST_MODULE))).digest("hex"));
  }).then(async nativeHost => {
    loaded.admitPlaygroundNativeHostV1 = nativeHost.admitPlaygroundNativeHostV1;
    loaded.nativeHostSourceFactsV1 = nativeHost.nativeHostSourceFactsV1;
    loaded.nativeHostScriptAdmissionV1 = nativeHost.nativeHostScriptAdmissionV1;
    const revision = createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT, COMPONENT_DEPLOYMENT_MODULE))).update(readPhysicalSource(join(LIBRARY_ROOT, INSTALLATION_IDENTITY_MODULE))).update(readPhysicalSource(join(LIBRARY_ROOT, INSTALLATION_IDENTITY_SCHEMA))).digest("hex");
    const identity = await importRevision(new URL(INSTALLATION_IDENTITY_MODULE, LIBRARY_URL), revision);
    const admit = identity.installationDirectoryParserV1(JSON.parse(readPhysicalSource(join(LIBRARY_ROOT, INSTALLATION_IDENTITY_SCHEMA)).toString("utf8")));
    const deployment = await importRevision(new URL(`./${COMPONENT_DEPLOYMENT_MODULE}`, LIBRARY_URL), revision);
    loaded.componentDeploymentDirectoryV1 = metadata => deployment.componentDeploymentDirectoryV1(metadata, admit);
    const browser = await importRevision(new URL(BROWSER_SESSION_MODULE, LIBRARY_URL), createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT, BROWSER_SESSION_MODULE))).digest("hex"));
    loaded.declaredBrowserSessionEnginesV1 = browser.declaredBrowserSessionEnginesV1;
    const composition = await importRevision(new URL(PLAYGROUND_COMPOSITION_MODULE, LIBRARY_URL), createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT, PLAYGROUND_COMPOSITION_MODULE))).digest("hex"));
    loaded.playgroundCompositionPathV1 = composition.playgroundCompositionPathV1;
  });
}
