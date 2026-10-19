import { join, resolve } from "node:path";
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import assert from "node:assert/strict";
import { pathToFileURL } from "node:url";
import type { ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type { ResourceLeaseControl } from "@semio-tech/process/leases";
import { testCacheContractBindingsV1 } from "./📋️bindings/🟦️.ts";
import { testCommandInputs } from "./🧾️command-inputs/🟦️.ts";
import { ticketOutput } from "../../🎫️output/🟦️.ts";
import { runProcessTestOwner } from "../🧭️ownership/🟦️.ts";
import { testDependencyBootstrap, testDependencyCancellation, testNxTooling } from "../📦️dependencies/🟦️.ts";
import { inventory } from "../../📇️inventory/🧮️composition/🟦️.ts";
import { testStringsV1 } from "../📋️values/🟦️.ts";
import { resolveBuildSelectionV1 } from "../../../🏗️build/🧭️invocation/🟦️.ts";
import { resolveNxBuildMode } from "../../🚀️bootstrap/🎚️mode/🟦️.ts";
import { slash } from "../../🔍️discovery/📂️source/🟦️.ts";

/** 🛡️ Proves explicit owner cache refusal against the portable policy corpus and installed Nx task semantics. */
export function proveCachePolicy(workspace: string, internals: { targetPolicy(name: string, target: object, policy: object): { cache: boolean; continuous?: boolean } }): void {
  const root = resolve(import.meta.dir, "../.."), require = createRequire(join(workspace, "package.json"));
  const vectors = JSON.parse(readFileSync(join(root, "🧫️fixtures/nx-contract/🔣️.json"), "utf8"));
  
  
  const policy = JSON.parse(readFileSync(join(root, "🔣️policy.json"), "utf8"));
  const { isCacheableTask } = require("nx/src/tasks-runner/utils");
  for (const row of vectors.policies) for (const authored of [true, false]) {
    const target = internals.targetPolicy(row.target, { cache: authored, options: { command: row.command ?? `bun ./📜️script.ts ${row.target}` } }, policy);
    const expected = authored ? row.cache : row.disabledCache;
    assert.deepEqual({ cache: target.cache, continuous: target.continuous ?? false }, { cache: expected, continuous: row.continuous }, `${row.target}: authored cache ${authored}`);
    assert.equal(isCacheableTask({ cache: target.cache, continuous: target.continuous === true, target: { project: "probe", target: row.target }, overrides: {} }), expected, `${row.target}: installed Nx cache semantics`);
  }
  console.log(`[cache-policy] ${vectors.policies.length} portable owner-cache laws agree with installed Nx`);
}

export function createCachePolicyTests(dependencies: Record<string, any>, testSource: { directory: string; url: string }) {
  const { assert, cacheInternals, chmodSync, copyFileSync, createRequire, devToolingEnv, dirname, EventEmitter, existsSync, getWorkspaceRoot, join, lstatSync, mkdirSync, mkdtempSync, plugin, readFileSync, relative, resolve, rmSync, SCRIPT_ROOT, spawn, stageRepositoryArtifacts, utimesSync, wasmBindgenVersion, wasmBuildArguments, wasmBuildEnvironment, writeFileSync } = dependencies;

  /** 🧪️ Executes language-neutral policy examples against the Nx project plugin. */
  async function testCacheContracts(invocation: ScriptInvocation, phaseControl: ResourceLeaseControl): Promise<void> {
    testCacheContractBindingsV1();
    const { validate } = createRequire(testSource.url)("jsonschema");
    const policy = JSON.parse(readFileSync(join(SCRIPT_ROOT, "🔣️policy.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(SCRIPT_ROOT, "🧬️schema/🔣️.json"), "utf8"));
    assert.equal(validate(policy, schema).valid, true);
    assert.ok(cacheInternals, "cache policy must be exposed for contract verification");
    const authoredWorkspace = JSON.parse(readFileSync(join(getWorkspaceRoot(), "📋️project.json"), "utf8"));
    const { availableParallelism } = await import("node:os");
    const { semioBuildParallel } = await import("../../../🏃️process/🌿️environment/🟦️.ts");
    const previousParallel = process.env.SEMIO_BUILD_PARALLEL;
    delete process.env.SEMIO_BUILD_PARALLEL;
    try {
      assert.equal(semioBuildParallel(), Math.max(1, availableParallelism()));
      process.env.SEMIO_BUILD_PARALLEL = "5";
      assert.equal(semioBuildParallel(), 5);
    } finally {
      if (previousParallel === undefined) delete process.env.SEMIO_BUILD_PARALLEL;
      else process.env.SEMIO_BUILD_PARALLEL = previousParallel;
    }
    assert.equal(devToolingEnv().NX_PARALLEL, String(semioBuildParallel()));
    const { semioBuildParallelFlag } = await import("../../../🏃️process/🌿️environment/🟦️.ts");
    assert.deepEqual(semioBuildParallelFlag(), ["--parallel", String(semioBuildParallel())]);
    assert.equal(JSON.parse(readFileSync(join(getWorkspaceRoot(), "nx.json"), "utf8")).parallel, undefined);
    for (const name of policy.serialTargets) {
      const target = authoredWorkspace.targets[name];
      if (target?.parallelism !== undefined) assert.equal(target.parallelism, false, name);
    }
    for (const name of ["test", "lint", "build", "verify"]) assert.equal(authoredWorkspace.targets[name].cache, true, name);
    for (const name of ["format", "setup", "publish"]) assert.equal(authoredWorkspace.targets[name].cache, false, name);
    const root = getWorkspaceRoot();
    assert.equal(Object.keys(JSON.parse(readFileSync(join(root, "nx.json"), "utf8")).targetDefaults ?? {}).length, 0, "Native Nx defaults override custom project metadata; apply defaults inside the repository plugin");
    assert.equal(wasmBuildEnvironment(root, {}).CARGO_TARGET_DIR, undefined);
    assert.equal(wasmBuildEnvironment(root, { CARGO_TARGET_DIR: "chosen-cache" }).CARGO_TARGET_DIR, "chosen-cache");
    const vectors = JSON.parse(readFileSync(join(SCRIPT_ROOT, "🧫️fixtures/nx-contract/🔣️.json"), "utf8"));
    
    const loggingKeys = Object.keys(vectors.daemonEnvironment), savedLogging = Object.fromEntries(loggingKeys.map((key) => [key, process.env[key]]));
    const quiet = devToolingEnv(Object.fromEntries(loggingKeys.map((key) => [key, undefined])));
    try {
      for (const [key, value] of Object.entries(vectors.daemonEnvironment)) { assert.equal(quiet[key], value); process.env[key] = quiet[key]; }
      const oracle = createRequire(testSource.url)("nx/src/daemon/client/daemon-environment.js").getDaemonSpawnEnv();
      for (const [key, value] of Object.entries(vectors.daemonEnvironment)) assert.equal(oracle[key], value, key);
      assert.equal(devToolingEnv({ NX_NATIVE_LOGGING: "nx=debug" }).NX_NATIVE_LOGGING, "nx=debug");
    } finally { for (const [key, value] of Object.entries(savedLogging)) if (value === undefined) delete process.env[key]; else process.env[key] = value; }
    for (const row of vectors.wasmProfiles) {
      const args = wasmBuildArguments(row.profile);
      assert.deepEqual(args, { pack: row.pack, cargo: row.cargo });
      for (const [tool, flags] of [["cargo", args.cargo], ["wasm-pack", args.pack]] as const) {
        const help = Bun.spawnSync([tool, "build", ...flags, "--help"], { stdout: "pipe", stderr: "pipe" });
        assert.equal(help.exitCode, 0, `${tool} rejected ${flags.join(" ")}: ${help.stderr.toString()}`);
      }
    }
    const socketFrames = createRequire(testSource.url)("nx/src/utils/consume-messages-from-socket.js");
    assert.equal(typeof socketFrames.writeMessage, "function", "Nx must preserve Unicode across socket frame boundaries");
    for (const size of vectors.socketFrames.chunkSizes) {
      const frames: Buffer[] = [], received: unknown[] = [];
      for (const message of vectors.socketFrames.messages) socketFrames.writeMessage({ write: (chunk: Buffer) => frames.push(chunk) }, Buffer.from(JSON.stringify(message)));
      const consume = socketFrames.consumeMessagesFromSocket((message: Buffer) => received.push(socketFrames.parseMessage(message)));
      const bytes = Buffer.concat(frames);
      for (let offset = 0; offset < bytes.length; offset += size) consume(bytes.subarray(offset, offset + size));
      assert.deepEqual(received, vectors.socketFrames.messages, `Unicode socket frame size ${size}`);
    }
    const { isCacheableTask } = createRequire(testSource.url)("nx/src/tasks-runner/utils");
    assert.equal(typeof cacheInternals.matchesUncached, "function");
    assert.equal(typeof cacheInternals.cacheableFamily, "function");
    assert.equal(cacheInternals.matchesUncached("format-check", policy), false);
    assert.equal(cacheInternals.matchesUncached("format", policy), true);
    assert.equal(cacheInternals.matchesUncached("commit", policy), true);
    assert.equal(cacheInternals.matchesUncached("os", policy), true);
    assert.equal(cacheInternals.matchesUncached("os-dev", policy), false);
    assert.equal(cacheInternals.matchesUncached("setup-git", policy), true);
    assert.equal(cacheInternals.matchesUncached("cache-report", policy), true);
    assert.equal(cacheInternals.cacheableFamily("format-check"), true);
    assert.equal(cacheInternals.cacheableFamily("generator-inputs"), false);
    assert.equal(cacheInternals.mutatingName("build-report"), false, "a document id that happens to be named 'report' must not be treated as a mutating report writer");
    assert.equal(cacheInternals.mutatingName("verify-dependencies-freeze-write-baseline"), false, "a deterministic baseline writer must not be blanket-matched by 'write-baseline'");
    assert.equal(cacheInternals.mutatingName("reset-document-ownership"), false, "a cargo test proving reset behavior is not itself a mutating reset");
    assert.equal(cacheInternals.mutatingName("cache-prune"), true);
    assert.equal(cacheInternals.mutatingName("stdio-fuzz"), true);
    assert.equal(cacheInternals.liveName("hub-live-catalog-check"), true, "the name heuristic stays domain-neutral");
    assert.equal(cacheInternals.targetPolicy("hub-live-catalog-check", { cache: false }, policy).cache, false, "owner cache refusal precedes cachedExact");
    assert.equal(cacheInternals.liveName("admin-live-journey-check"), true, "a genuine live-browser/live-server journey stays uncached");
    assert.equal(cacheInternals.liveName("collab-e2e"), true);
    for (const row of vectors.policies) {
      const enabled = cacheInternals.targetPolicy(row.target, { cache: true, options: { command: row.command ?? `bun ./📜️script.ts ${row.target}` } }, policy);
      const disabled = cacheInternals.targetPolicy(row.target, { cache: false, options: { command: row.command ?? `bun ./📜️script.ts ${row.target}` } }, policy);
      assert.deepEqual({ cache: enabled.cache, continuous: enabled.continuous ?? false }, { cache: row.cache, continuous: row.continuous }, row.target);
      assert.equal(disabled.cache, row.disabledCache, `${row.target}: owner cache refusal precedes automatic cache families`);
      assert.equal(isCacheableTask({ cache: enabled.cache, continuous: enabled.continuous === true, target: { project: "probe", target: row.target }, overrides: {} }), row.cache, `${row.target}: nx isCacheableTask`);
    }
    await testCommandInputs(root, ticketOutput(root, []), invocation, phaseControl);
    await (await import("../🚀️bootstrap/🟦️.ts")).testNxBootstrap(root, ticketOutput(root, []));
    await runProcessTestOwner(invocation, root, ticketOutput(root, []), "artifact-publication");
    await runProcessTestOwner(invocation, root, ticketOutput(root, []), "native-artifact-publication");
    await (await import("../📇️artifacts/🟦️.ts")).testArtifactRegistry(root, ticketOutput(root, []));
    await (await import("../🌎️hub/🟦️.ts")).testHubBuild(root);
    await testDependencyBootstrap(root, ticketOutput(root, []));
    await testNxTooling(root, ticketOutput(root, []));
    await testDependencyCancellation(root, ticketOutput(root, []));
    const result = await (await import("../../📇️inventory/🧪️tests/🕸️coverage/🟦️.ts")).testNativeInventory(root, inventory), contracts = result.projects;
    const toml = createRequire(testSource.url)("@iarna/toml");
    let componentPackages = 0;
    const componentLaunchers: { project: string; pluginId: string }[] = [];
    for (const project of contracts) {
      const path = join(root, project.root, "Cargo.toml");
      if (!existsSync(path)) continue;
      const manifest = toml.parse(readFileSync(path, "utf8"));
      if (!manifest.package?.metadata?.component?.package || !["plugin", "extension"].includes(manifest.package?.metadata?.semio?.["component-kind"])) continue;
      componentPackages++;
      componentLaunchers.push({ project: project.name, pluginId: manifest.package.metadata.component.package.slice("semio:".length) });
      for (const removed of vectors.materialization.removedTargets) assert.equal(project.targets[removed], undefined, `${project.name}:${removed} is a driver stage, never a per-component Nx target`);
      const component = project.targets[vectors.componentTarget.target];
      assert.equal(component?.cache, false, `${project.name}:component answers to Cargo freshness and the stage cache`);
      assert.equal(component.metadata?.semio?.orchestrator, true, `${project.name}:component owns its driver without an owner-command deadline`);
      for (const dependency of vectors.componentTarget.dependsOn) assert.ok(component.dependsOn.includes(dependency), `${project.name}:component follows ${dependency}`);
      assert.equal(component.outputs[0], vectors.componentTarget.output);
      assert.ok(component.options.command.includes(vectors.componentTarget.command));
    }
    {
      const row = vectors.materialization, shared = contracts.find((project) => project.name === row.project)?.targets[row.support];
      assert.equal(shared?.cache, row.supportCache);
      const supportRoot = `{projectRoot}/${row.modules}`;
      assert.deepEqual(shared.outputs, [supportRoot, `{projectRoot}/${row.canonicalModules}`].flatMap(base => [`${base}/🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim`, `${base}/🪞️vendor/🔤️guestslim-typst-fonts.bin`, `${base}/🧵️shard`]), "browser support owns the preview2 shims, the typst font asset every served module root needs, and the shard worker");
      assert.deepEqual(shared.dependsOn, ["semio-framework-os-infinite:fonts"], "browser support stages the font the fonts target publishes");
    }
    for (const row of vectors.releaseOrchestrators) {
      const target = contracts.find((project) => project.name === row.project)?.targets[row.target];
      assert.ok(target, `${row.project}:${row.target} must be a release orchestrator`);
      assert.equal(target.cache, false);
      assert.equal(target.metadata?.semio?.orchestrator, true);
      assert.equal(target.executor, "nx:run-commands", `${row.project}:${row.target} must not be wrapped by an owner command deadline`);
      assert.deepEqual(target.metadata.semio.release.jobs.map((job: any) => job.id), row.jobs);
      for (const prerequisite of row.prerequisites) assert.ok(target.dependsOn.includes(prerequisite), `${row.project}:${row.target} needs ${prerequisite}`);
      const generators = new Set(Object.values(JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")).generatorContracts ?? {}).map((contract: any) => contract.target));
      assert.ok(target.dependsOn.every((dependency: string) => row.prerequisites.includes(dependency) || generators.has(dependency)), `${row.project}:${row.target} depends on generators and prerequisites only: ${target.dependsOn.filter((dependency: string) => !row.prerequisites.includes(dependency) && !generators.has(dependency)).join(", ")}`);
    }
    assert.ok(componentPackages > 0, "Component metadata must discover plugin producers");
    const fontProject = contracts.find((project) => project.name === vectors.fontAssets.project)!;
    const fontToolProject = contracts.find((project) => project.name === vectors.fontAssets.toolProject)!;
    for (const [project, name, output] of [[fontToolProject, vectors.fontAssets.tool, vectors.fontAssets.toolOutput], [fontProject, vectors.fontAssets.producer, vectors.fontAssets.output]] as const) {
      assert.equal(project.targets[name]?.cache, true, `${name} must be separately cacheable`);
      assert.deepEqual(project.targets[name].outputs, [output]);
    }
    assert.deepEqual(fontProject.targets[vectors.fontAssets.producer].dependsOn, [`${vectors.fontAssets.toolProject}:${vectors.fontAssets.tool}`]);
    const { testSelectedFontDependencies, testSelectedPackageIdentities } = await import("../🎯️selected-runtime/🟦️.ts");
    testSelectedPackageIdentities(root);
    await testSelectedFontDependencies(root);
    assert.ok(fontProject.targets[vectors.fontAssets.producer].inputs.some((input: any) => input.dependentTasksOutputFiles === "**/*"));
    const activationRoot = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation");
    const activation = await import(join(activationRoot, "🟦️.ts"));
    const activationCases = JSON.parse(readFileSync(join(activationRoot, "🧫️cases.json"), "utf8"));
    const ActivationAjv = createRequire(testSource.url)("ajv");
    const activationSchema = JSON.parse(readFileSync(join(activationRoot, "🧬️schema/🔣️.json"), "utf8"));
    const validateActivation = new ActivationAjv().addSchema(activationSchema).getSchema(`${activationSchema.$id}#/$defs/DevActivationV1`)!;
    const { keyBy, sortBy } = createRequire(testSource.url)("lodash");
    let activationReceipt: any;
    for (const cycle of activationCases.cycles) {
      const completed = cycle.plugins.map((row:unknown) => {const[pluginId,digest]=testStringsV1(row);if(typeof pluginId!=="string"||typeof digest!=="string")throw Error("Actual activation plugin/digest pair required");return{pluginId,artifactSha256:digest.repeat(64)};});
      const prior = keyBy(activationReceipt?.plugins ?? [], "pluginId");
      const nextTimestamp = Math.max(cycle.now, 1, ...Object.values(prior).map((row: any) => row.rebuiltAt + 1));
      const oracle = sortBy(completed, "pluginId").map((row: any) => ({ ...row, rebuiltAt: prior[row.pluginId]?.artifactSha256 === row.artifactSha256 ? prior[row.pluginId].rebuiltAt : nextTimestamp }));
      const next = activation.nextActivationReceipt(activationCases.variant, activationCases.profile, completed, activationReceipt, cycle.now);
      assert.equal(validateActivation(next), true, JSON.stringify(validateActivation.errors));
      assert.deepEqual(next.plugins, oracle);
      assert.deepEqual(next.plugins.map((row: any) => [row.pluginId, row.artifactSha256[0], row.rebuiltAt]), cycle.expected);
      assert.deepEqual(activation.parseActivationReceipt(JSON.parse(JSON.stringify(next))), next);
      activationReceipt = next;
    }
    assert.throws(() => activation.parseActivationReceipt({ ...activationReceipt, plugins: [...activationReceipt.plugins, ...activationReceipt.plugins] }), /Duplicate/);
    assert.throws(() => activation.parseActivationReceipt({ ...activationReceipt, unknown: true }), /Invalid/);
    assert.throws(() => activation.nextActivationReceipt("other", "prototype", [], activationReceipt, 500), /identity/);
    assert.throws(() => activation.nextActivationReceipt("../note", "prototype", [], undefined, 500), /Invalid/);
    const sessionProject = contracts.find((project) => project.name === vectors.playgroundSessions.project)!;
    for (const variant of vectors.playgroundSessions.variants) {
      const session = sessionProject.targets[`session-${variant}`];
      assert.equal(session?.cache, true, `${variant} needs its own cacheable playground session`);
      assert.ok(session.dependsOn.includes((await cacheInternals.workspaceCatalogBuildContributionsV1(root))[0].project+":"+(await cacheInternals.workspaceCatalogBuildContributionsV1(root))[0].generateTarget));
      assert.deepEqual(session.dependsOn, [(await cacheInternals.workspaceCatalogBuildContributionsV1(root))[0].project+":"+(await cacheInternals.workspaceCatalogBuildContributionsV1(root))[0].generateTarget], `${variant} session needs no component build`);
      const descriptors = session.inputs.filter((input: unknown) => typeof input === "string" && input.startsWith("{workspaceRoot}/"));
      assert.ok(descriptors.length > 0 && descriptors.every((input: string) => vectors.playgroundSessions.descriptorInputs.some((name: string) => input.endsWith(`/${name}`))), `${variant} session hashes the committed semantic descriptors of its closure`);
      assert.deepEqual(session.outputs, [`{projectRoot}/dist/sessions/${variant}`]);
      assert.ok(session.inputs.some((input: any) => input.dependentTasksOutputFiles === "**/*"));
    }
    const registryRoot = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry");
    const { generatePlaygroundRegistry } = await import(join(registryRoot, "🎮️playground/🔎️discovery/🟦️.ts"));
    const { buildPlaygroundSession } = await import(join(registryRoot, "🎮️playground/🧭️session/🟦️.ts"));
    const playgrounds = generatePlaygroundRegistry(root);
    for (const project of contracts) for (const [name, target] of Object.entries(project.targets) as [string, any][]) if (target.options?.command?.includes("⚡️caching/🦀️cargo/📜️script.ts") && ["build", "check", "test"].includes(name)) assert.ok(project.targets[name], `Missing native editor target ${project.name}:${name}`);
    const preparationProject = contracts.find((project) => project.name === vectors.playgroundPreparation.project)!;
    for (const playground of playgrounds) {
      for (const engine of playground.engines) assert.ok(contracts.some((project) => resolve(root, project.root) === resolve(root, engine) && project.targets.wasm), `Engine ${engine} must name an authored producer`);
      for (const profile of vectors.playgroundPreparation.profiles) {
        for (const command of ["prepare", "run", "smoke"]) assert.ok(preparationProject.targets[`${command}-${playground.variant}-native-${profile}`], "Missing native variant editor target");

        const targetName = `prepare-${playground.variant}-react-${profile}`, preparation = preparationProject.targets[targetName];
        assert.ok(preparation, `${targetName} needs declared prerequisites`);
        assert.equal(preparation.cache, false, `${targetName} builds its closure through the driver`);
        assert.equal(preparation.metadata?.semio?.orchestrator, true);
        const wgpuTargetName = `prepare-${playground.variant}-wgpu-${profile}`, wgpuPreparation = preparationProject.targets[wgpuTargetName];
        assert.ok(wgpuPreparation, `${wgpuTargetName} needs declared prerequisites`);
        assert.ok(wgpuPreparation.dependsOn.includes(`@semio-tech/framework-renderer-wgpu:wasm`));
        assert.equal(wgpuPreparation.cache, false, `${wgpuTargetName} builds its closure through the driver`);
        assert.deepEqual(wgpuPreparation.outputs, preparation.outputs, `${wgpuTargetName} publishes the same boot closure modules`);
        const activationName = `activate-${playground.variant}-react-${profile}`, activationTarget = preparationProject.targets[activationName];
        assert.ok(activationTarget, `${activationName} must follow completed preparation`);
        assert.equal(activationTarget.cache, true, `${activationName} publishes into a variant/profile-scoped runtime root and is safe to replay`);
        assert.deepEqual(activationTarget.outputs, [`{projectRoot}/dist/runtime/react/${profile}/${playground.variant}`]);
        assert.deepEqual(activationTarget.dependsOn, [targetName]);
        const wgpuActivationName = `activate-${playground.variant}-wgpu-${profile}`, wgpuActivationTarget = preparationProject.targets[wgpuActivationName];
        assert.ok(wgpuActivationTarget, `${wgpuActivationName} must follow completed preparation`);
        assert.equal(wgpuActivationTarget.cache, false, `${wgpuActivationName} publishes live extensions and reload notifications on every activation`);
        assert.deepEqual(wgpuActivationTarget.outputs, [`{projectRoot}/dist/runtime/wgpu/${profile}/${playground.variant}`]);
        assert.deepEqual(wgpuActivationTarget.dependsOn, [wgpuTargetName]);
        {
          const serverName = `serve-${playground.variant}-react-${profile}`, server = preparationProject.targets[serverName];
          assert.ok(server, `${serverName} needs an Nx server owner`);
          assert.equal(server.continuous, true);
          assert.equal(server.cache, false);
          assert.deepEqual(server.outputs, []);
          assert.deepEqual(server.dependsOn, [`@semio-tech/plugin-registry:session-${playground.variant}`]);
          assert.equal(server.options.command, `bun ./📜️script.ts serve ${playground.variant} react ${profile}`);
        }
        {
          const serverName = `dev-${playground.variant}-react-${profile}`, server = preparationProject.targets[serverName];
          assert.ok(server, `${serverName} needs an Nx server owner`);
          assert.equal(server.continuous, true);
          assert.equal(server.cache, false);
          assert.deepEqual(server.outputs, []);
          assert.deepEqual(server.dependsOn, [activationName]);
          assert.equal(server.options.command, `bun ./📜️script.ts serve ${playground.variant} react ${profile}`);
        }
        const hostLauncher = componentLaunchers.find((entry) => entry.pluginId === playground.pluginId);
        if (!hostLauncher) throw new Error(`${playground.variant}: host plugin must launch`);
        const moduleOf = (pluginId: string) => contracts.find((project) => project.name === componentLaunchers.find((entry) => entry.pluginId === pluginId)?.project);
        const hostDirectory = moduleOf(playground.pluginId) && toml.parse(readFileSync(join(root, moduleOf(playground.pluginId)!.root, "Cargo.toml"), "utf8")).package.metadata.semio["deployment-directory"];
        if (hostDirectory !== undefined) assert.ok(preparation.outputs.some((output: string) => output.endsWith(`/${hostDirectory}`)), `${targetName}: boot prepare must publish the host module`);
        const fullSession = buildPlaygroundSession(playground.variant, (await import("@semio-tech/plugin-registry/catalog-view")).readGeneratedCatalogProjection(join(root,(await cacheInternals.workspaceCatalogBuildContributionsV1(root))[0].publication.outputRoot))).plugins;
        assert.ok(preparation.outputs.length <= fullSession.length, `${targetName}: boot prepare must not exceed the full session closure`);
        if (fullSession.length > preparation.outputs.length + 5) assert.ok(preparation.outputs.length < fullSession.length / 2, `${targetName}: host boot prepare must skip the catalog fanout`);
        for (const prerequisite of vectors.closurePreparation.prerequisites) assert.ok(preparation.dependsOn.includes(prerequisite));
        const engines = playground.engines.map((engine: string) => contracts.find((project) => resolve(root, project.root) === resolve(root, engine))!.name + ":wasm");
        for (const engine of engines) assert.ok(preparation.dependsOn.includes(engine));
        assert.ok(preparation.dependsOn.includes(`@semio-tech/plugin-registry:session-${playground.variant}`));
        assert.ok(preparation.dependsOn.includes(`${vectors.materialization.project}:${vectors.materialization.support}`));
        assert.ok(preparation.dependsOn.includes(vectors.playgroundPreparation.fonts));
      }
      const target = `session-${playground.variant}`;
      assert.ok(sessionProject.targets[target]);
    }
    assert.deepEqual(result.violations.filter((finding) => finding.rule === "ORCH-01"), [], "All public commands must use one script entrypoint");
    for (const entry of vectors.entrypoints) assert.ok(contracts.find((project) => project.name === entry.project)?.targets[entry.target].dependsOn?.includes(entry.prerequisite), `${entry.project}:${entry.target} needs ${entry.prerequisite} in Nx`);
    for (const entry of vectors.nativeBinaries) {
      const project = contracts.find((project) => project.name === entry.project)!;
      assert.equal(project.targets[entry.target].cache, true, `${entry.project} binary must be restorable`);
      const oracle=createRequire(testSource.url)("nx/src/tasks-runner/utils").getOutputs({[entry.project]:{data:project}}, {project:entry.project,target:entry.target,configuration:entry.configuration},{});
      assert.deepEqual(oracle,[entry.output.replace("{projectRoot}",project.root)]);
    }
    for (const entry of vectors.components) {
      const project = contracts.find((project) => project.name === entry.project)!;
      const target = project.targets[entry.target];
      assert.ok(project.namedInputs?.default?.includes(entry.schemaInput), `${entry.project} must hash its shared component schema`);
      assert.equal(target.cache, true, `${entry.project}:${entry.target} component must be restorable`);
      assert.deepEqual(target.outputs, [entry.output]);
      assert.ok(contracts.find((project) => project.name === entry.consumerProject)!.targets[entry.consumerTarget].dependsOn.includes(`${entry.project}:${entry.target}`));
    }
    const mcpRoot = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp";
    const mcp = await import(join(root, mcpRoot, "🟦️.ts"));
    const binaryVectors = JSON.parse(readFileSync(join(root, mcpRoot, "🎚️config/🧱️binary-gate.json"), "utf8"));
    const pathOracle = createRequire(testSource.url)("node:path");
    for (const row of binaryVectors.pathCases) {
      assert.equal(mcp.resolveMcpBinaryPath(row.repoRoot, row.environment, row.platform), row.expected);
      const paths = row.platform === "win32" ? pathOracle.win32 : pathOracle.posix;
      assert.equal(paths.resolve(row.repoRoot, row.environment.SEMIO_OS_MCP_BIN ?? binaryVectors.artifactRoot, ...(row.environment.SEMIO_OS_MCP_BIN ? [] : [binaryVectors.cargoBinary + (row.platform === "win32" ? ".exe" : "")])), row.expected);
    }
    const generators = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")).generatorContracts;
    for (const id of vectors.generators) {
      const authority = generators[id], split = authority.target.lastIndexOf(":"), project = contracts.find((project) => project.name === authority.target.slice(0, split))!;
      const target = project.targets[authority.target.slice(split + 1)];
      assert.equal(target.cache, true, `${id} must cache its verified deliverables`);
      assert.deepEqual(target.outputs, authority.outputRoots.filter((output: any) => !output.producer || output.producer.target === authority.target).map((output: any) => `{workspaceRoot}/${output.path}`), id);
      for (const output of authority.outputRoots.filter((output: any) => output.producer && output.producer.target !== authority.target)) {
        const split = output.producer.target.lastIndexOf(":"), producer = contracts.find((project) => project.name === output.producer.target.slice(0, split));
        assert.ok(producer!.targets[output.producer.target.slice(split + 1)].outputs.includes(`{workspaceRoot}/${output.path}`));
        assert.ok(target.dependsOn.includes(output.producer.target));
      }
      for (const path of authority.inputPatterns) assert.ok(target.inputs.includes(`{workspaceRoot}/${path}`), `${id} missing ${path}`);
      if (authority.inputDiscovery) {
        const fingerprint = policy.generatorInputs[authority.inputDiscovery.kind], separator = fingerprint.target.lastIndexOf(":"), owner = contracts.find((project) => project.name === fingerprint.target.slice(0, separator));
        const guard = owner!.targets[fingerprint.target.slice(separator + 1)];
        assert.ok(target.inputs.some((input: any) => input.dependentTasksOutputFiles === fingerprint.output), `${id} must hash the discovered input receipt`);
        assert.ok(!target.inputs.some((input: any) => input.runtime?.includes("generator-inputs")), `${id} must not repeat discovery inside the hasher`);
        assert.ok(target.dependsOn.includes(fingerprint.target)); assert.equal(guard.cache, false, `${id} discovery receipt producer re-digests on every run: its receipt digests bytes its own cache key cannot name`);
        assert.deepEqual(guard.outputs, [`{workspaceRoot}/${fingerprint.output}`]);
      }
      if (authority.checkTarget) {
        const guardName = authority.checkTarget.slice(authority.checkTarget.lastIndexOf(":") + 1), guardTarget = project.targets[guardName];
        assert.equal(guardTarget.cache, true, `${id} freshness checks must be soundly cacheable`);
        for (const path of authority.inputPatterns) assert.ok(guardTarget.inputs.includes(`{workspaceRoot}/${path}`), `${id} check target missing generator input ${path}`);
        for (const output of authority.outputRoots) assert.ok(guardTarget.inputs.some((input: any) => typeof input === "object" && typeof input.runtime === "string" && input.runtime.includes(JSON.stringify(join(root, output.path)))), `${id} check target must hash its own generated output ${output.path} live, since Nx's own fileset walk silently drops anything .gitignore covers`);
        if (authority.inputDiscovery) {
          const fingerprint = policy.generatorInputs[authority.inputDiscovery.kind];
          assert.ok(guardTarget.inputs.some((input: any) => input.dependentTasksOutputFiles === fingerprint.output), `${id} check target must hash the discovered input receipt too`);
          assert.ok(guardTarget.dependsOn?.includes(fingerprint.target), `${id} check target must depend on the discovery fingerprint owner`);
        }
      }
    }
    const [validatedProject, validatedTarget] = vectors.validationPrerequisite.target.split(":");
    assert.ok(contracts.find((project) => project.name === validatedProject)?.targets[validatedTarget].dependsOn.includes(vectors.validationPrerequisite.prerequisite));
    assert.ok(!contracts.find((project) => project.name === validatedProject)?.targets[validatedTarget].inputs.includes("^default"), "Exact catalog discovery must not reintroduce generated outputs through broad dependency defaults");
    const workspace = contracts.find((project) => project.name === "workspace")!;
    assert.deepEqual(workspace.targets.setup.dependsOn, vectors.lifecycle.setupDependencies);
    assert.deepEqual(workspace.targets.prepare.dependsOn, vectors.lifecycle.prepareDependencies);
    for (const target of vectors.lifecycle.setupDependencies) assert.equal(workspace.targets[target]?.cache, false);
    for (const name of ["test", "lint", "build", "verify"]) assert.equal(workspace.targets[name]?.cache, true, name);
    assert.equal(workspace.targets.dev, undefined, "the playground variant switch is dissolved: playgrounds are dashboard commands");
    assert.equal((await cacheInternals.nativeTargetCommandInputs({ options: { command: "bun ./x.ts", cwd: "." } }, root)).some((input: { readonly env?: string }) => input.env && input.env.startsWith("SEMIO_")), true, "native leaves must fingerprint SEMIO toolchain env");
    const hostProject = contracts.find((project) => project.name === vectors.platformEnvironment.project)!;
    for (const target of Object.values(hostProject.targets) as any[]) for (const key of vectors.platformEnvironment.keys) assert.equal(target.options?.env?.[key], undefined, `Shared target metadata cannot force ${key}`);
    const bootstrap = vectors.bootstrap, dotnet = contracts.find((project) => project.name === bootstrap.dotnetProject);
    assert.ok(dotnet, "The current .NET support library needs an Nx owner");
    assert.deepEqual(dotnet!.targets.build.outputs, ["{projectRoot}/dist/build"]);
    assert.ok(dotnet!.targets.build.dependsOn.includes("deps"));
    assert.equal(dotnet!.targets.deps.cache, false);
    assert.ok(workspace.targets["deps-dotnet"].dependsOn.includes(`${bootstrap.dotnetProject}:deps`));
    assert.ok(readFileSync(join(root, "Monorepo.sln"), "utf8").includes(bootstrap.dotnetPath.replaceAll("/", "\\")));
    assert.ok(readFileSync(join(root, bootstrap.dotnetPath, "🧪️Semio.Repo.Test.csproj"), "utf8").includes(`Include="${bootstrap.compile}"`));
    assert.ok(existsSync(join(root, bootstrap.dotnetPath, bootstrap.compile)), "The .NET compile item must resolve to its current source file");
    const python = createRequire(testSource.url)("@iarna/toml").parse(readFileSync(join(root, "pyproject.toml"), "utf8"));
    assert.deepEqual(python.tool.uv.workspace.members, bootstrap.pythonMembers);
    assert.deepEqual(workspace.targets["deps-python"].dependsOn, bootstrap.pythonDependencies);
    const styling = contracts.find((project) => project.name === bootstrap.stylingProject)!;
    assert.ok(styling.targets.generate.dependsOn.includes(bootstrap.stylingGenerator));
    assert.deepEqual(styling.targets.generate.outputs, []);
    const nativeGraph = createRequire(testSource.url)("@nx/devkit").readCachedProjectGraph();
    const { createTaskGraph } = createRequire(testSource.url)("nx/src/tasks-runner/create-task-graph");
    for (const [name, target] of Object.entries(styling.targets) as [string, any][]) if (target.options.command.includes(" test")) {
      const tasks = createTaskGraph(nativeGraph, {}, [styling.name], [name], undefined, {}, false);
      assert.ok(tasks.tasks[bootstrap.stylingGenerator], `${styling.name}:${name} must schedule the styling generator through its prerequisites`);
    }
    assert.ok(!/compose|topologic|vcpkg/i.test(readFileSync(join(root, "CMakeLists.txt"), "utf8") + readFileSync(join(root, "CMakePresets.json"), "utf8")));
    const ts = createRequire(testSource.url)("typescript");
    for (const row of vectors.engineOutputs) {
      const project = contracts.find((project) => project.name === row.project)!;
      const physicalOutputs = (outputs: string[]): string[] => outputs.map(output => cacheInternals.resolveOutputPath(output, project.root, root)).sort();
      assert.deepEqual(physicalOutputs(project.targets.wasm.outputs), physicalOutputs(row.outputs));
      const engineSource = ts.createSourceFile("engine.ts", readFileSync(join(root, project.root, "📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
      const outputs: string[] = [];
      const inspect = (node: any): void => {
        if (ts.isCallExpression(node) && ["buildWasmWebV1","buildRepositoryWasmWebV1"].includes(node.expression.getText(engineSource))) {
          const output = node.arguments[0].properties.find((property: any) => property.name?.getText(engineSource) === "outputDirectory");
          outputs.push(output?.initializer.text ?? "pkg");
        }
        ts.forEachChild(node, inspect);
      };
      inspect(engineSource);
      assert.deepEqual(outputs, [row.directory]);
    }
    const source = ts.createSourceFile("setup.ts", readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🔧️setup/🟦️.ts"), "utf8"), ts.ScriptTarget.Latest, true);
    const nxSource = ts.createSourceFile("nx.ts", readFileSync(join(SCRIPT_ROOT, "🚀️bootstrap/📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
    const { testNxCoordinator } = await import("../🛑️coordinator/🟦️.ts");
    await testNxCoordinator(root);
    const invocationNode = nxSource.statements.find((node: any) => ts.isFunctionDeclaration(node) && node.name?.text === "resolveNxInvocation");
    const route = ts.transpileModule(invocationNode.getText(nxSource).replace(/^export /, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
    const resolveInvocation = new Function("process", "WORKSPACE_ROOT", "resolveBuildSelectionV1", "resolveNxBuildMode", `${route}; return resolveNxInvocation;`)({ env: {} }, root, resolveBuildSelectionV1, resolveNxBuildMode);
    const resolvePrint = new Function("process", "WORKSPACE_ROOT", "resolveBuildSelectionV1", "resolveNxBuildMode", `${route}; return resolveNxInvocation;`)({ env: {} }, root, resolveBuildSelectionV1, resolveNxBuildMode);
    for (const path of ["🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🧫️invocations.json", "♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/🧫️invocations.json", "♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️invocations.json"]) {
      const invocations = JSON.parse(readFileSync(join(root, path), "utf8"));
      for (const vector of invocations.valid) {
        const result = resolvePrint(vector.input);
        assert.deepEqual(result.args, vector.args);
        assert.equal(result.watch, vector.watch);
        if (vector.env) assert.deepEqual(result.env, vector.env);
      }
      for (const vector of invocations.invalid) assert.throws(() => resolvePrint(vector));
    }
    for (const command of ["prepare", "activate", "serve", "dev"]) {
      const selected = `@semio-tech/framework-os-dev:${command}-note-react-prototype`;
      const invocation = resolveInvocation(["run", selected]);
      assert.equal(invocation.watch, command === "dev" ? "@semio-tech/framework-os-dev:activate-note-react-prototype" : undefined);
      assert.equal(invocation.env.SEMIO_BUILD_MODE, "prototype");
      assert.equal(resolveInvocation(["run", selected, "--graph=stdout"]).watch, undefined);
    }
    for (const renderer of vectors.benchmarkRenderers) {
      const target = `bench-plugins-${renderer}`;
      const args = resolveInvocation(["run", "workspace:bench", "--", "plugins", `--renderer=${renderer}`, "--count", "3"]).args;
      assert.deepEqual(args, ["run", `@semio-tech/framework-os-dev:${target}`, "--", "--count", "3"]);
      assert.equal(hostProject.targets[target].dependsOn.includes("@semio-tech/framework-os-scale-fixture:build-wasm"), renderer === "native");
      assert.equal(hostProject.targets[target].dependsOn.includes("@semio-tech/framework-renderer-wgpu:native-build"), renderer === "native");
    }
    const hostSource = ts.createSourceFile("host.ts", readFileSync(join(root, vectors.platformEnvironment.source), "utf8"), ts.ScriptTarget.Latest, true);
    const apple = hostSource.statements.find((node: any) => ts.isFunctionDeclaration(node) && node.name?.text === "ensureAppleDeveloperDir");
    assert.ok(apple, `Missing Apple tooling selector in ${vectors.platformEnvironment.source}`);
    const selectApple = ts.transpileModule(apple.getText(hostSource), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
    for (const platform of vectors.platformEnvironment.platforms) {
      const host = { platform, env: {} as Record<string, string> };
      new Function("process", "existsSync", `${selectApple}; ensureAppleDeveloperDir();`)(host, () => true);
      assert.deepEqual(Object.keys(host.env).sort(), platform === "darwin" ? [...vectors.platformEnvironment.keys].sort() : []);
    }
    const setup = source.statements.find((node: any) => ts.isClassDeclaration(node) && node.name?.text === "SetupScript").getText(source);
    assert.ok(!/runWorkspaceCodegen|buildRepoMcpClient|runNx|\["nx"|\["build"/.test(setup), "Setup must only provision dependency environments");
    const lock = readFileSync(join(root, "Cargo.lock"), "utf8");
    const lockOracle = createRequire(testSource.url)("@iarna/toml").parse(lock);
    assert.equal(wasmBindgenVersion(lock), lockOracle.package.find((pkg: any) => pkg.name === "wasm-bindgen").version);
    for (const row of vectors.wasm) {
      const project = contracts.find((project) => project.name === row.project);
      assert.ok(project, row.project);
      const expectedOutput = cacheInternals.resolveOutputPath(row.output, project!.root, root);
      assert.ok(expectedOutput, row.output);
      assert.ok(project!.targets.wasm.outputs?.some((output: string) => cacheInternals.resolveOutputPath(output, project!.root, root) === expectedOutput), `${row.project}:wasm must own ${row.output}`);
      assert.notEqual(project?.targets.wasm.cache, false, `${row.project}:wasm must be cacheable`);
      assert.ok(project?.namedInputs?.default.some((input: any) => input.env === "RUSTFLAGS"), `${row.project} must preserve toolchain inputs`);
      if (row.output.startsWith("{projectRoot}/") && !row.output.includes("..")) assert.ok(project?.namedInputs?.default.includes(`!${row.output}/**/*`), `${row.project} must exclude its output in the project fileset`);
    }
    const ticket = process.env.SEMIO_TICKET_DIR;
    assert.ok(ticket, "SEMIO_TICKET_DIR must point to the active ticket for generated test files");
    const fixture = join(resolve(root, ticket), "🗑️generated", "policy-contract");
    const rustFixture = join(fixture, "rust-inputs");
    for (const [path, contents] of Object.entries(vectors.rust.files)) { mkdirSync(dirname(join(rustFixture, path)), { recursive: true }); writeFileSync(join(rustFixture, path), contents as string); }
    const rustInputs = cacheInternals.rustSourceFiles([join(rustFixture, vectors.rust.entry)], rustFixture).map((path: string) => slash(relative(rustFixture, path))).sort();
    assert.deepEqual(rustInputs, vectors.rust.inputs);
    const rustOracle = Bun.spawnSync(["rustc", "--crate-type=lib", "--emit=dep-info", "-o", join(rustFixture, "oracle.d"), vectors.rust.entry], { cwd: rustFixture, stdout: "pipe", stderr: "pipe" });
    assert.equal(rustOracle.exitCode, 0, rustOracle.stderr.toString());
    const rustDependencies = readFileSync(join(rustFixture, "oracle.d"), "utf8").split("\n")[0]!.split(": ")[1]!.trim().split(/\s+/).map((path: string) => slash(relative(rustFixture, resolve(rustFixture, path)))).sort();
    assert.deepEqual(rustInputs, [...new Set(rustDependencies)]);
    const discovery = vectors.rustDiscoveryCache, sourceCache = cacheInternals.createRustSourceCache(discovery.limitBytes);
    const revisionRoot = join(fixture, "rust-revisions");
    for (const [path, contents] of Object.entries(discovery.files)) { mkdirSync(dirname(join(revisionRoot, path)), { recursive: true }); writeFileSync(join(revisionRoot, path), contents as string); }
    mkdirSync(dirname(join(revisionRoot, discovery.entry)), { recursive: true });
    for (const revision of discovery.revisions) {
      const entry = join(revisionRoot, discovery.entry);
      writeFileSync(entry, revision.source);
      utimesSync(entry, 1_700_000_000, 1_700_000_000);
      const inputs = cacheInternals.rustSourceFiles([entry], revisionRoot, sourceCache).map((path: string) => slash(relative(revisionRoot, path))).sort();
      assert.deepEqual(inputs, revision.inputs);
      const oracle = Bun.spawnSync(["rustc", "--crate-type=lib", "--emit=dep-info", "-o", "oracle.d", discovery.entry], { cwd: revisionRoot, stdout: "pipe", stderr: "pipe" });
      assert.equal(oracle.exitCode, 0, oracle.stderr.toString());
      const dependencies = readFileSync(join(revisionRoot, "oracle.d"), "utf8").split("\n")[0]!.split(": ")[1]!.trim().split(/\s+/).map((path: string) => slash(relative(revisionRoot, resolve(revisionRoot, path)))).sort();
      assert.deepEqual(inputs, [...new Set(dependencies)]);
      const hits = sourceCache.hits;
      assert.deepEqual(cacheInternals.rustSourceFiles([entry], revisionRoot, sourceCache).map((path: string) => slash(relative(revisionRoot, path))).sort(), inputs);
      assert.ok(sourceCache.hits > hits);
      assert.ok(sourceCache.bytes <= discovery.limitBytes);
    }
    for (let index = 0; index < 100; index++) {
      const entry = join(revisionRoot, discovery.entry);
      writeFileSync(entry, `pub const VALUE_${index}: usize = ${index};`);
      cacheInternals.rustSourceFiles([entry], revisionRoot, sourceCache);
      assert.ok(sourceCache.bytes <= discovery.limitBytes);
    }
    assert.ok(sourceCache.entries.size < 100);
    const { captureOwnedCommandV1 } = await import(pathToFileURL(join(root, vectors.materialization.source)).href);
    const materializerSpawn = (command: string, args: readonly string[], cwd: string, signal?: AbortSignal) => captureOwnedCommandV1(command, args, cwd, signal ? { signal } : {});
    const materializerFixture = join(fixture, "materializer-cancel");
    mkdirSync(materializerFixture, { recursive: true });
    const childScript = join(materializerFixture, "📜️script.ts"), pidPath = join(materializerFixture, "pid");
    writeFileSync(childScript, 'import { renameSync, writeFileSync } from "node:fs"; if (process.argv[2] === "fail") { process.stderr.write("x".repeat(200000)); process.exit(1); } process.on("SIGTERM", () => {}); setInterval(() => {}, 1000); const stage = process.argv[2] + ".stage"; writeFileSync(stage, String(process.pid)); renameSync(stage, process.argv[2]);');
    rmSync(pidPath, { force: true });
    const controller = new AbortController();
    const execution = materializerSpawn("bun", [childScript, pidPath], materializerFixture, controller.signal).then(() => undefined, (error: Error) => error);
    let childPid = 0;
    try {
      const startupDeadline = Date.now() + 10000;
      while (!existsSync(pidPath)) { assert.ok(Date.now() < startupDeadline, "Materializer fixture must become ready"); await new Promise((accept) => setTimeout(accept, 25)); }
      childPid = Number(readFileSync(pidPath, "utf8"));
      assert.ok(Number.isSafeInteger(childPid) && childPid > 0, "Materializer readiness must identify its child process");
      const started = Date.now();
      controller.abort(new Error("materializer cancelled"));
      assert.match(String(await execution), /materializer cancelled/);
      assert.ok(Date.now() - started < vectors.materialization.cancellation.shutdownMilliseconds);
      assert.throws(() => process.kill(childPid, 0), "Cancelled code generation must terminate the subprocess");
    } finally {
      controller.abort();
      if (childPid && process.platform !== "win32") try { process.kill(-childPid, "SIGKILL"); } catch {}
    }
    const diagnostic = await materializerSpawn("bun", [childScript, "fail"], materializerFixture).then(() => "", (error: Error) => error.message);
    assert.match(diagnostic, /exited with status 1/);
    assert.ok(diagnostic.length < vectors.materialization.cancellation.maximumDiagnosticCharacters);
    const kernelInputs = contracts.find((project) => project.name === "@semio-tech/framework-os-kernel")?.namedInputs?.default;
    assert.ok(kernelInputs?.includes("{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"));
    assert.ok(!kernelInputs?.includes("{workspaceRoot}/🧰️framework/🛍️products/💻️os/**/*.{rs,toml,json,semio,wit,wgsl,glsl,h,c,cpp}"));
    const graph = vectors.pluginGraph;
    mkdirSync(join(fixture, graph.leaf.directory), { recursive: true });
    mkdirSync(join(fixture, "other"), { recursive: true });
    writeFileSync(join(fixture, graph.leaf.directory, "📋️project.json"), JSON.stringify({ name: graph.leaf.name, targets: { test: { options: { command: graph.leaf.testCommand } } } }));
    writeFileSync(join(fixture, graph.leaf.directory, "📜️script.ts"), graph.leaf.script);
    writeFileSync(join(fixture, "other", "📋️project.json"), JSON.stringify({ name: graph.leaf.name }));
    writeFileSync(join(fixture, "📜️script.ts"), graph.workspaceRoot.script);
    writeFileSync(join(fixture, "📋️project.json"), JSON.stringify({ name: graph.workspaceRoot.name, targets: Object.fromEntries(graph.workspaceRoot.targets.map((name: string) => [name, { options: { command: `bun ./📜️script.ts ${name}` } }])) }));
    const goManifest = vectors.go.manifest;
    writeFileSync(join(fixture, "go.mod"), goManifest);
    const goOracle = Bun.spawnSync(["go", "mod", "edit", "-json", `-modfile=${join(fixture, "go.mod")}`], { cwd: fixture, env: { ...process.env, GOWORK: "off" }, stdout: "pipe", stderr: "pipe" });
    assert.equal(goOracle.exitCode, 0, goOracle.stderr.toString());
    const go = JSON.parse(goOracle.stdout.toString());
    const { manifest: _manifest, ...expectedGo } = vectors.go;
    assert.deepEqual(cacheInternals.goDependencies(goManifest), expectedGo);
    assert.deepEqual(cacheInternals.goDependencies(goManifest), { module: go.Module.Path, requires: go.Require.map((row: any) => row.Path), replacements: go.Replace.map((row: any) => ({ module: row.Old.Path, path: row.New.Path })) });
    await assert.rejects(async () => await plugin.createNodesV2[1]([`${graph.leaf.directory}/📋️project.json`, "other/📋️project.json"], {}, { workspaceRoot: fixture }), new RegExp(`duplicate.*${graph.leaf.name}`, "i"));
    const projects = await plugin.createNodesV2[1]([`${graph.leaf.directory}/📋️project.json`], {}, { workspaceRoot: fixture });
    assert.equal(projects[0][1].projects[graph.leaf.name].targets[graph.leaf.cachedTarget].cache, true);
    const rooted = await plugin.createNodesV2[1](["📋️project.json"], {}, { workspaceRoot: fixture });
    const resolvedRoot = rooted[0][1].projects[graph.workspaceRoot.name].targets;
    for (const name of graph.workspaceRoot.targets) {
      const expected = vectors.policies.find((row: { readonly target: string }) => row.target === name);
      assert.ok(expected, `workspace-root fixture target ${name} needs a policy row`);
      assert.equal(resolvedRoot[name].cache, expected.cache, `workspace-root ${name}`);
      assert.equal(resolvedRoot[name].continuous ?? false, expected.continuous, `workspace-root ${name} continuous`);
    }
    assert.equal(projects[0][1].projects[graph.leaf.name].targets.test.executor, "nx:run-commands");
    assert.deepEqual(cacheInternals.nativeDependencies({ dependencies: { core: { path: "../core" }, serde: "1" } }, {}), [{ name: "core", path: "../core", workspace: false, kind: "dependencies" }]);
    const artifact = join(fixture, "consumer");
    writeFileSync(artifact, "native fixture\n");
    chmodSync(artifact, 0o755);
    const staged = join(fixture, "dist/build");
    await stageRepositoryArtifacts(staged, "leaf/Cargo.toml", new Map([["consumer", artifact], ["obsolete", artifact]]));
    await stageRepositoryArtifacts(staged, "leaf/Cargo.toml", new Map([["consumer", artifact]]));
    assert.deepEqual(readFileSync(join(staged, "consumer")), readFileSync(artifact));
    assert.equal(existsSync(join(staged, "obsolete")), false);
    if (process.platform !== "win32") assert.equal(lstatSync(join(staged, "consumer")).mode & 0o111, 0o111);
    await assert.rejects(() => stageRepositoryArtifacts(staged, "another/Cargo.toml", new Map()), /Unowned/);
    await assert.rejects(() => stageRepositoryArtifacts(staged, "leaf/Cargo.toml", new Map([["../escape", artifact]])), /Invalid artifact/);
    assert.ok(existsSync(join(staged, "consumer")));
    const testApi = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts"));
    const taxonomy = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
    mkdirSync(dirname(join(fixture, taxonomy)), { recursive: true });
    copyFileSync(join(root, taxonomy), join(fixture, taxonomy));
    const results = join(testApi.testCacheRoot(fixture), "tasks", "probe", "test", "results", "run");
    mkdirSync(results, { recursive: true });
    const retainedHash = "sha256:" + "a".repeat(64);
    writeFileSync(join(results, "🗝️run.json"), JSON.stringify({ artifacts: [{ sha256: retainedHash }] }));
    assert.ok(testApi.markReferencedBlobs(fixture, { contributions: [] } as any).has(retainedHash), "Task-scoped restored reports must retain their referenced blobs");
    const past = new Date(Date.now() - 30 * 86400000).toISOString();
    testApi.writeLease(results, { runId: "complete", agentId: "cache-test", pid: process.pid, state: "complete", createdAt: past, heartbeatAt: past, retention: "ephemeral-success" });
    const active = join(dirname(results), "active");
    testApi.writeLease(active, { runId: "active", agentId: "cache-test", pid: process.pid, state: "active", createdAt: past, heartbeatAt: past, retention: "ephemeral-success" });
    const pendingBlob = testApi.installFixtureBlob(fixture, new TextEncoder().encode("active publication"));
    const gc = testApi.collectGarbage(fixture, { contributions: [] } as any, { dry: false, olderThanMs: 7 * 86400000 });
    assert.ok(gc.removed.some((path: string) => path.endsWith("/results/run")), "Completed scoped runs must be collectible");
    assert.ok(existsSync(active), "Active scoped runs must survive collection");
    assert.ok(existsSync(testApi.fixtureBlobPath(fixture, pendingBlob)), "An active run may not yet have published its blob references");
    writeFileSync(join(active, "🗝️run.json"), "incomplete");
    assert.throws(() => testApi.markReferencedBlobs(fixture, { contributions: [] } as any), /Cannot determine/);
    const { resolveNxInvocation } = await import("../../🚀️bootstrap/📜️script.ts");
    assert.deepEqual(resolveNxInvocation(["run", "workspace:build", "--graph=stdout", "--", "assets"]).args, ["run", "@semio-tech/assets:build", "--graph=stdout"]);
    assert.deepEqual(resolveNxInvocation(["show", "projects"]).args, ["show", "projects"]);
    assert.deepEqual(resolveNxInvocation(["run", "workspace:cpp", "--", "build"]).args, ["run", "workspace:cpp-build"]);
    const cppSource = ts.createSourceFile("cmake.ts", readFileSync(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/📜️script.ts"),"utf8"), ts.ScriptTarget.Latest, true);
    const cpp = cppSource.statements.find((node: any) => ts.isClassDeclaration(node) && node.name?.text === "CppScript");
    const configure = cpp.members.find((node: any) => node.name?.getText(cppSource) === "runConfigure").getText(cppSource);
    assert.ok(!configure.includes("runSetup"), "CMake configuration must not reinstall its prerequisites");
    assert.ok(!cpp.getText(cppSource).includes("ensureVcpkg"), "No current project consumes a vcpkg installation");
    assert.deepEqual(resolveNxInvocation(["run", "workspace:test", "--", "quick"]).args, ["run", "workspace:test", "--", "quick"]);
    assert.deepEqual(resolveNxInvocation(["run", "workspace:test"]).args, ["run", "workspace:test", "--", "fundamental"]);
    assert.deepEqual(resolveNxInvocation(["run", "workspace:test", "--", "long", "repo-mcp", "-run", "TestX"]).args, ["run", "repo-mcp:test", "--", "long", "-run", "TestX"]);
    assert.equal(resolveNxInvocation(["run", "workspace:setup", "--", "deps", "wasm"]).args[1], "workspace:deps-wasm");
    const rootTest = source.statements.find((node: any) => ts.isClassDeclaration(node) && node.name?.text === "TestScript").getText(source);
    assert.ok(!rootTest.includes("run-many"), "The root test target cannot schedule a second Nx graph");
    for (const level of ["fundamental", "quick", "long", "exhaustive"]) assert.equal(workspace.targets[`test-${level}`], undefined, level);
    assert.deepEqual(workspace.targets.test.dependsOn.find((row: any) => typeof row === "object" && row.target === "test"), { target: "test", projects: ["*", "!workspace", "!@semio-tech/repo-test-domain"], params: "forward" });
    for (const retired of [["run", "workspace:dev", "--", "s"], ["run", "workspace:dev", "--", "mcp", "http", "os"], ["run", "@semio-tech/framework-os-dev:dev"]]) assert.deepEqual(resolveNxInvocation(retired).args, retired, `${retired.join(" ")}: the bootstrap is no playground or MCP resolver; the dashboard registry resolves playgrounds`);
    const osDevProject = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json"), "utf8")) as { targets: Record<string, { dependsOn?: readonly string[]; options?: { command?: string } }> };
    assert.equal(osDevProject.targets.dev, undefined, "the framework-os-dev dev alias is dissolved: playgrounds are the generated dev-<variant>-<renderer>-<profile> targets");
    assert.ok(!invocationNode.getText(nxSource).includes("PlaygroundSelections") && !invocationNode.getText(nxSource).includes("PlaygroundCatalog"), "the bootstrap resolves no playground variant");
    assert.equal(root.length > 0, true);
    rmSync(fixture, { recursive: true });
    console.log("[cache-contract] schema, graph ownership, source-byte discovery, native dependency oracles and materializer cancellation passed");
  }
  return { testCacheContracts };
}
