/** 🧩️ Projects Cargo, component, release-pipeline, print and root-command targets. */
import { createRequire } from "node:module";
import { dirname, join, relative } from "node:path";
import { observedFs } from "../../../../🏗️build/👁️observation/🟨️.mjs";
import { printArtifactNxOutputV1 } from "../../../../../../../📓️print/📦️packages/🟦️typescript/📦️artifacts/🟦️.ts";
const { existsSync, readFileSync, statSync } = observedFs;

/** 🏷️ Names this component as its roster entry does. */
export const component = "component-targets";
let SCRIPT_BASENAME, relativeScriptInputs, executionTargetV1, DEFAULT_EXECUTOR, buildPath, readToml, LIBRARY_ROOT, loaded, componentModelOutputV1, PLUGIN_MODULES_ROOT, POLICY, nativePreparation;

/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */
export function wire(registry) {
  ({ SCRIPT_BASENAME, DEFAULT_EXECUTOR, buildPath, LIBRARY_ROOT, componentModelOutputV1, PLUGIN_MODULES_ROOT, POLICY } = registry.coordinates);
  ({ relativeScriptInputs } = registry.scriptInputs);
  ({ executionTargetV1 } = registry.targetPolicy);
  ({ readToml, nativePreparation } = registry.nativeManifests);
  ({ loaded } = registry.bootstrap);
}

/** 📄️ Projects a document catalog into separately owned PDF tasks without executing a compiler. */
export async function printDocumentTargets(project, root, workspaceRoot, scripts) {
  const path = project.metadata?.printCatalog;
  if (!path) return {};
  const catalog = JSON.parse(readFileSync(join(workspaceRoot, path), "utf8")), compiler = dirname(dirname(path));
  if (catalog.version !== 1 || !Array.isArray(catalog.librarySources) || !catalog.librarySources.length || !Array.isArray(catalog.documents) || !catalog.documents.length) throw new Error(`Invalid Print catalog: ${path}`);
  const product = dirname(dirname(compiler)), script = `${compiler}/${SCRIPT_BASENAME}`, targets = {}, ids = new Set(), sources = new Set();
  const commandInputs = await relativeScriptInputs([join(workspaceRoot, script)], workspaceRoot, scripts);
  const libraryInputs = catalog.librarySources.map(source => {
    if (!source || source.includes("\\") || source.startsWith("/") || source.split("/").some(part => [".", "..", ""].includes(part))) throw new Error(`Invalid Print library source: ${source}`);
    const absolute = `${product}/${source}`;
    return `{workspaceRoot}/${absolute}${statSync(join(workspaceRoot, absolute)).isDirectory() ? "/**/*" : ""}`;
  });
  const inputs = [...commandInputs, ...project.targets.fonts.inputs, `{workspaceRoot}/${path}`, `{workspaceRoot}/${compiler}/🔧️toolchain/**/*`, `{workspaceRoot}/${compiler}/📚️bundle/🔒️dependencies.json`, ...libraryInputs, `{workspaceRoot}/🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json`, "{workspaceRoot}/bunfig.toml", { externalDependencies: ["pdfjs-dist", "sharp"] }, { runtime: "bun --version" }, { runtime: 'node -p "process.platform.concat(process.arch)"' }];
  for (const document of catalog.documents) {
    if (!/^[a-z]+(?:-[a-z0-9]+)*$/.test(document.id) || ids.has(document.id) || sources.has(document.texPath) || !["templates", "visualizations"].includes(document.collection)) throw new Error(`Invalid Print document owner: ${document.id}`);
    ids.add(document.id); sources.add(document.texPath);
    const paths = [...new Set([document.texPath, ...document.sources])].map(path => {
      if (!path || path.includes("\\") || path.startsWith("/") || path.split("/").some(part => [".", "..", ""].includes(part))) throw new Error(`Invalid Print source: ${path}`);
      const source = `${product}/${path}`;
      return `{workspaceRoot}/${source}${statSync(join(workspaceRoot, source)).isDirectory() ? "/**/*" : ""}`;
    });
    targets[`build-${document.id}`] = executionTargetV1("source", { executor: DEFAULT_EXECUTOR, cache: true, outputs: [printArtifactNxOutputV1(workspaceRoot,{version:1,kind:"document",mode:"prototype",namespace:"print",id:document.id})], inputs: [...inputs, ...paths], dependsOn: ["fonts", "deps-tectonic", "deps-tex", "generate"], options: { cwd: root, command: `bun ${buildPath(relative(root, script))} build ${document.id}`, forwardAllArgs: false } });
    targets[`watch-${document.id}`] = executionTargetV1("source", { executor: DEFAULT_EXECUTOR, cache: false, continuous: true, outputs: [], dependsOn: [`build-${document.id}`], options: { cwd: root, command: `bun ${buildPath(relative(root, script))} watch`, forwardAllArgs: false } });
  }
  for (const [name, collection] of [["build", "templates"], ["build-viz", "visualizations"]]) targets[name] = executionTargetV1("source", { ...project.targets[name], cache: false, outputs: [], dependsOn: catalog.documents.filter(document => document.collection === collection).map(document => `build-${document.id}`) });
  return targets;
}

/** 🔨️ Every concrete Cargo package exposes native leaves even when its authored metadata only defines generators. `build` queues on the Cargo build lease and may overlap other work; `check` and `test` run Cargo directly and stay `parallelism: false`. */
export async function cargoTargets(root, workspaceRoot, commandInputs,scripts) {
  const manifest = join(workspaceRoot, root, "Cargo.toml");
  if (!existsSync(manifest) || !readToml(manifest).package) return {};
  const script = buildPath(relative(workspaceRoot, join(LIBRARY_ROOT, "⚡️caching/🦀️cargo/📜️script.ts")));
  commandInputs ??= [];
  return Object.fromEntries(["build", "check", "test"].map((target) => [target, loaded.targetBuildModes(target, executionTargetV1("native", {
    executor: DEFAULT_EXECUTOR,
    cache: true,
    ...(target === "build" ? {} : { parallelism: false }),
    outputs: target === "build" ? ["{projectRoot}/dist/build/{options.env.SEMIO_BUILD_MODE}"] : [],
    inputs: [target === "test" ? "nativeTestSources" : "nativeSources", target === "test" ? "^nativeTestSources" : "^nativeSources", ...commandInputs],
    options: { cwd: ".", command: `bun ${JSON.stringify(script)} native cargo ${target} --manifest ${JSON.stringify(buildPath(relative(workspaceRoot, manifest)))}` },
  }, target === "test" ? "test" : "production"))]));
}

/** 🧩️ The one on-demand target of every plugin and extension crate: `component` builds exactly that component through the build driver (its Cargo pass
 * restricted to the one package, then describe, receipt, transpile, `wasm-opt`, probe and module) for the tests and tools that load it. It is uncached
 * (Cargo freshness and the stage cache are the authorities) and never part of a release graph, which compiles every component inside its one driver task.
 * @see ../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts */
export function componentBuildTargets(root, workspaceRoot) {
  const path = join(workspaceRoot, root, "Cargo.toml");
  if (!existsSync(path)) return {};
  const manifest = readToml(path), metadata = manifest.package?.metadata;
  if (!metadata?.component?.package || !["plugin", "extension"].includes(metadata.semio?.["component-kind"])) return {};
  if (manifest.lib?.["crate-type"]?.length !== 1 || manifest.lib["crate-type"][0] !== "cdylib") throw new Error(`Component ${metadata.component.package} must be a cdylib-only component crate: ${path}`);
  const script = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts", moduleDirectory = loaded.componentDeploymentDirectoryV1(metadata.semio);
  return { component: executionTargetV1("orchestration", {
    executor: DEFAULT_EXECUTOR,
    cache: false,
    metadata: { semio: { orchestrator: true } },
    dependsOn: ["@semio-tech/os-plugin-describe-rs:build", "@semio-tech/framework-plugin-web:support", "workspace:deps-wasm-opt"],
    outputs: [componentModelOutputV1(buildPath(root)), ...(moduleDirectory === undefined ? [] : [`{workspaceRoot}/${PLUGIN_MODULES_ROOT}/${moduleDirectory}`])],
    options: { cwd: ".", command: `bun ${JSON.stringify(script)} component --manifest ${JSON.stringify(buildPath(relative(workspaceRoot, path)))}`, forwardAllArgs: false },
  }) };
}

/** 🧩️ The crate roots of every plugin and extension component, the crates a release orchestrator's driver compiles. */
export function componentCrateRoots(configFiles, workspaceRoot) {
  return configFiles.filter((path) => path.endsWith("Cargo.toml") && !path.includes("\uFFFD") && !path.includes(".🧬semio") && !path.startsWith("compose/") && !path.startsWith("temp/compose/") && !POLICY.generatedDirectories.some((directory) => path.split("/").includes(directory))).flatMap((path) => {
    const metadata = readToml(join(workspaceRoot, path)).package?.metadata;
    return metadata?.component?.package && ["plugin", "extension"].includes(metadata.semio?.["component-kind"]) ? [buildPath(dirname(path))] : [];
  }).sort();
}

/** 🚀️ Gives every release orchestrator target (`metadata.semio.release`, a `PipelineV1` declaration) the Nx prerequisites its driver consumes and nothing else:
 * the declared prerequisites (toolchains, catalog generation) and the native generators of every component crate and every job root, all cached at t = 0. The
 * driver itself compiles, so the release graph is these generators, the orchestrator, and nothing per component.
 * @see ../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧬️schema/🔣️.json */
export function withReleasePipelinePreparation(project, workspaceRoot, contracts, componentRoots, cache = new Map(), closures = new Map()) {
  for (const target of Object.values(project.targets ?? {})) {
    const pipeline = target.metadata?.semio?.release;
    if (!pipeline) continue;
    if (target.metadata.semio.orchestrator !== true || target.cache !== false || !Array.isArray(pipeline.prerequisites) || !Array.isArray(pipeline.jobs)) throw new Error(`Release pipeline must be an uncached orchestrator with prerequisites and jobs: ${project.name}`);
    const roots = [...new Set([...componentRoots, ...pipeline.jobs.map((job) => buildPath(job.root)).filter((root) => existsSync(join(workspaceRoot, root, "Cargo.toml")) && readToml(join(workspaceRoot, root, "Cargo.toml")).package)])];
    const generators = roots.flatMap((root) => nativePreparation(root, workspaceRoot, contracts, false, cache, closures).map((contract) => contract.target));
    target.dependsOn = [...new Set([...(target.dependsOn ?? []), ...pipeline.prerequisites, ...generators])].sort();
  }
  return project;
}

/** 🚪️ Exposes registered workspace command coordinators without inferring forwarding package scripts.
 * @returns {import("../🟨️.d.mts").GeneratedProjectV1["targets"]} */
export function rootCommandTargets(script) {
  const ts = createRequire(import.meta.url)("typescript");
  const source = ts.createSourceFile(script, readFileSync(script, "utf8"), ts.ScriptTarget.Latest, true);
  const targets = {};
  const inspect = (node) => {
    if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && node.expression.name.text === "register" && ts.isStringLiteral(node.arguments[0])) {
      const name = node.arguments[0].text;
      if (!["nx", "repo"].includes(name)) targets[name] = executionTargetV1("orchestration", { executor: DEFAULT_EXECUTOR, outputs: [], options: { command: `bun ./📜️script.ts ${name}`, forwardAllArgs: true } });
      inspect(node.expression.expression);
    }
  };
  for (const statement of source.statements) {
    if (!ts.isVariableStatement(statement)) continue;
    for (const declaration of statement.declarationList.declarations) if (declaration.name.getText(source) === "router" && declaration.initializer) inspect(declaration.initializer);
  }
  return targets;
}
