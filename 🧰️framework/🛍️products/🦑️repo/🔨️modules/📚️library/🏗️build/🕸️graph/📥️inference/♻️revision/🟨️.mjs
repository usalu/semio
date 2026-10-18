/** ♻️ Identifies the complete authored graph implementation, its policy authority and its dependency resolution authority. */
import { buildModesV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { createHash } from "node:crypto";
import { isAbsolute, join, relative, resolve } from "node:path";
import { observedFs } from "../../../../🏗️build/👁️observation/🟨️.mjs";
const { existsSync } = observedFs;

/** 🏷️ Names this component as its roster entry does. */
export const component = "revision";
let readPhysicalSource, TARGET_EXECUTION_SCHEMA, COMMAND_INPUT_MODULE, OWNER_ARGUMENTS_MODULE, OBSERVATION_MODULE, CARGO_EXECUTION_MODULE, CARGO_EXECUTION_SCHEMA, BUILD_MODE_MODULE, BUILD_MODE_API, PROJECT_INPUT_MODULE, RUNTIME_COMPONENT_MODULE, SOURCE_INPUT_MODULE, BROWSER_SESSION_MODULE, PLAYGROUND_COMPOSITION_MODULE, NATIVE_HOST_MODULE, COMPONENT_DEPLOYMENT_MODULE, readTaxonomyProjectionV1, LIBRARY_ROOT, buildPath, readBunLockGraph, goManifest, bunLockCacheHash, services;

/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */
export function wire(registry) {
  ({ readPhysicalSource, TARGET_EXECUTION_SCHEMA, COMMAND_INPUT_MODULE, OWNER_ARGUMENTS_MODULE, OBSERVATION_MODULE, CARGO_EXECUTION_MODULE, CARGO_EXECUTION_SCHEMA, BUILD_MODE_MODULE, BUILD_MODE_API, PROJECT_INPUT_MODULE, RUNTIME_COMPONENT_MODULE, SOURCE_INPUT_MODULE, BROWSER_SESSION_MODULE, PLAYGROUND_COMPOSITION_MODULE, NATIVE_HOST_MODULE, COMPONENT_DEPLOYMENT_MODULE, LIBRARY_ROOT, buildPath } = registry.coordinates);
  ({ readTaxonomyProjectionV1 } = registry.sourceAnalyses);
  ({ readBunLockGraph, bunLockCacheHash } = registry.bunLock);
  ({ goManifest } = registry.nativeManifests);
  ({ services } = registry);
}

/** ♻️ Identifies the complete authored graph implementation and policy authority. */
export function implementationRevision() {
  const hash = createHash("sha256");
  for (const path of services.sourceFiles) hash.update(readPhysicalSource(path));
  for (const path of ["🏗️build/📥️inputs/🔏️revision/🟨️.mjs", "🏗️build/📥️inputs/🔏️revision/🧬️schema/🔣️.json", "⚡️caching/🔣️policy.json", "🔣️taxonomy.json", TARGET_EXECUTION_SCHEMA, COMMAND_INPUT_MODULE, OWNER_ARGUMENTS_MODULE, OBSERVATION_MODULE, "🏗️build/👁️observation/📥️reuse/🟨️.mjs", "🏗️build/👁️observation/📥️reuse/🧬️schema/🔣️.json", CARGO_EXECUTION_MODULE, CARGO_EXECUTION_SCHEMA, BUILD_MODE_MODULE, BUILD_MODE_API, PROJECT_INPUT_MODULE, "🏗️build/📥️inputs/📋️data/🟨️.mjs", "🏗️build/📥️inputs/📋️data/📥️load/🟨️.mjs", "🏗️build/📥️inputs/📋️data/🧬️schema/🔣️.json", "🏗️build/📥️inputs/🧮️digest/🟨️.mjs", "🏗️build/📥️inputs/🧮️digest/🔣️schema.json", "🏗️build/📥️inputs/🧮️digest/📥️read/🟨️.mjs", "🏗️build/📥️inputs/🧮️digest/📥️read/🧬️schema/🔣️.json", "🏗️build/📥️inputs/🔒️lock/🟨️.mjs", "🏗️build/📥️inputs/🔒️lock/🔣️schema.json", "🏗️build/📥️inputs/🔒️lock/📥️read/🟨️.mjs", "🏗️build/📥️inputs/💾️storage/🟨️.mjs", "🏗️build/📥️inputs/💾️storage/📒️ledger/🟨️.mjs", "🏗️build/📥️inputs/💾️storage/🧬️schema/🔣️.json", "⚡️caching/🗃️records/🟦️.ts", "⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json", RUNTIME_COMPONENT_MODULE, SOURCE_INPUT_MODULE, BROWSER_SESSION_MODULE, PLAYGROUND_COMPOSITION_MODULE, NATIVE_HOST_MODULE, COMPONENT_DEPLOYMENT_MODULE, "../../../../🔨️modules/🪪️identity/📁️installation/🟨️.mjs", "../../../../🔨️modules/🪪️identity/📁️installation/🧬️schema/🔣️.json"]) hash.update(path === "🔣️taxonomy.json" ? JSON.stringify(readTaxonomyProjectionV1(join(LIBRARY_ROOT, path))) : readPhysicalSource(join(LIBRARY_ROOT, path)));
  hash.update(JSON.stringify(buildModesV1()));
  return hash.digest("hex");
}

/** 🧬️ Identifies current dependency resolution authority using only owned immutable source facts.
 * @param {string} workspaceRoot
 * @param {Record<string, { root: string }>} projects
 * @returns {string}
 */
export function dependencyResolutionAuthorityImplementation(workspaceRoot, projects) {
  const hash = createHash("sha256").update(implementationRevision());
  const source = (path) => { hash.update(JSON.stringify(buildPath(relative(workspaceRoot, path)))); hash.update(existsSync(path) ? readPhysicalSource(path) : "\0absent\0"); };
  for (const path of ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/🔣️.json", "Cargo.toml", "go.work", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/🟨️.mjs"]) source(join(workspaceRoot, path));
  const recipe = join(workspaceRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools"), manifest = join(recipe, "package.json");
  for (const path of [join(workspaceRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts"), join(recipe, "📜️script.ts"), manifest, join(recipe, "bun.lock")]) source(path);
  if (existsSync(manifest)) for (const path of [...new Set(Object.values(JSON.parse(readPhysicalSource(manifest)).semio?.toolPatches ?? {}))].sort()) {
    if (typeof path !== "string") throw new Error("Invalid tooling authority patch path");
    const file = resolve(workspaceRoot, path), local = buildPath(relative(workspaceRoot, file));
    if (isAbsolute(local) || local === ".." || local.startsWith("../")) throw new Error("Tooling authority patch must belong to this workspace");
    source(file);
  }
  if (existsSync(join(workspaceRoot, "bun.lock"))) { readBunLockGraph(workspaceRoot); hash.update(bunLockCacheHash()); }
  else hash.update("\0no-bun-lock\0");
  for (const [name, project] of Object.entries(projects).sort(([a], [b]) => a.localeCompare(b))) {
    hash.update(JSON.stringify([name, buildPath(project.root)]));
    for (const filename of ["package.json", "📋️project.json", "Cargo.toml"]) source(join(workspaceRoot, project.root, filename));
    const go = goManifest(project.root, workspaceRoot);
    if (go) source(go);
  }
  return hash.digest("hex");
}
