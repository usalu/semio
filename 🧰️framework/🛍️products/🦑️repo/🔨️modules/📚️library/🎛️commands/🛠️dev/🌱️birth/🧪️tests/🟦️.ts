import { expect, setDefaultTimeout, test } from "bun:test";

setDefaultTimeout(60000);
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { rootDevMcpBuildRequestsV1, rootDevMcpFreshCapabilitiesV1, rootDevPlaygroundFreshCapabilitiesV1, rootDevPlaygroundRequestV1 } from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import cases from "../🧫️fixtures/🔣️.json" with { type: "json" };

import playgroundCases from "../🧫️fixtures/playground.json" with { type: "json" };
import coldPlaygroundCases from "../🧫️fixtures/playground-cold-start.json" with { type: "json" };
import stagingCases from "../🧫️fixtures/playground-staging.json" with { type: "json" };
import descriptorCases from "../🧫️fixtures/descriptor-emitter.json" with { type: "json" };
import transpilerCases from "../🧫️fixtures/native-transpiler.json" with { type: "json" };
import preparationCases from "../🧫️fixtures/component-preparation.json" with { type: "json" };
import gisContribution from "../🧫️fixtures/gis-contribution.json" with { type: "json" };
import { missingPlaygroundBootComponentsV1 } from "../../🎮️playground/🟦️.ts";
import restartCases from "../🧫️fixtures/restart.json" with { type:"json" };
import { runWorkspaceScriptMain } from "../../../../🚨️policy/🧭️entrypoints/🟦️.ts";
import { rootCommandRetriesFreshBuildV1 } from "../../../🌱️birth/🟦️.ts";
import { concurrentEditV1 } from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import { ScriptProcessInterruptedV1 } from "../../../../../../../../🔨️modules/🏃️process/🛑️interruption/🟦️.ts";
import { BundleScript, Script, ScriptRouter, createIssuedResourceControlV1, createScriptProcessEnvelope, requireIssuedScriptInvocationV1, scriptInvocationEnvironment, SCRIPT_PROCESS_INVOCATION_ENV, withIssuedResourceOperationV1, withScriptProcessEnvelope, type ScriptInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readOwnedBuildInvocationV1 } from "../../../../🏗️build/📥️caller/🟦️.ts";

test("canonical dev MCP birth agrees with independent SQLite selection and AJV target validation", () => {
  const oracle = new Ajv({ strict: false, allErrors: true }), database = new Database(":memory:");
  expect(oracle.validate(schema, cases), JSON.stringify(oracle.errors)).toBe(true);
  const select = database.query("SELECT json_array_length($args)>=4 AND json_extract($args,'$[0]')='dev' AND json_extract($args,'$[1]')='mcp' AND json_extract($args,'$[2]')='stdio' AND json_extract($args,'$[3]')='os' AND NOT EXISTS(SELECT 1 FROM json_each($args) WHERE value IN ('-h','--help')) AS selected, (SELECT count(*) FROM json_each($args) WHERE key>=4 AND value='--catalog-target') AS flags, (SELECT json_extract($args,'$['||(key+1)||']') FROM json_each($args) WHERE key>=4 AND value='--catalog-target' LIMIT 1) AS target");
  try {
    for (const row of cases) {
      const result = select.get({ $args: JSON.stringify(row.argv) }) as { selected: number; flags: number; target: unknown };
      const refused = Boolean(result.selected && (result.flags > 1 || result.flags === 1 && !oracle.validate(schema.$defs.Target, result.target)));
      expect(refused, row.id).toBe(row.rejected);
      if (refused) { expect(() => rootDevMcpBuildRequestsV1(row.argv), row.id).toThrow(); continue; }
      const targets = result.selected ? [...new Set([schema.$defs.Protocol.const.buildTarget, ...(result.flags ? [result.target as string] : [])])] : null;
      expect(targets, row.id).toEqual(row.targets);
      expect(rootDevMcpBuildRequestsV1(row.argv)?.map(request => request[1]) ?? null, row.id).toEqual(targets);
    }
  } finally { database.close(); }
});

test("fresh dev MCP receives original providers and both declared native children", async () => {
  const environment: Record<string, string | undefined> = {}, root = resolve(import.meta.dirname, "../../../../../../../../..");
  const args = ["dev", "mcp", "stdio", "os", "--catalog-target", "@semio-tech/s-dev-composition:generate"];
  const capabilities = await rootDevMcpFreshCapabilitiesV1(root, args, environment) as { workspaceRoot: string; buildMode: string; native: unknown; launches: string[][] };
  expect(capabilities.workspaceRoot).toBe(root);
  expect(capabilities.buildMode).toBe("prototype");
  expect(capabilities.native).toBeDefined();
  expect(environment.SEMIO_SCRIPT_PROVIDERS).toBeDefined();
  for (const request of rootDevMcpBuildRequestsV1(args)!) expect(capabilities.launches).toContainEqual(["repo", ...request]);
  expect(await rootDevMcpFreshCapabilitiesV1(root, ["dev", "s"], {})).toBeNull();
});

test("graph-free playground admits exact renderer requests and keeps their original profiles", async () => {
  const root = resolve(import.meta.dirname, "../../../../../../../../..");
  const validate = new Ajv({ strict: true }).compile(schema.$defs.PlaygroundRequest);
  for (const { argv, profile } of playgroundCases) {
    const [family, command, variant, renderer, suppliedProfile] = argv;
    const selected = argv.length === 5 && family === "dev" && command === "playground" &&
      validate({ variant, renderer, profile: suppliedProfile });
    expect(selected ? suppliedProfile : null).toBe(profile);
    const requestData = rootDevPlaygroundRequestV1(argv);
    expect(requestData?.profile ?? null).toBe(profile);
    expect(requestData?.renderer ?? null).toBe(profile ? renderer : null);
    const capabilities = await rootDevPlaygroundFreshCapabilitiesV1(root, argv, {});
    if (!profile || !capabilities) {
      expect(capabilities).toBeNull();
      continue;
    }
    const request = ["repo", ...argv];
    expect(capabilities.buildMode).toBe(profile);
    expect(capabilities.arguments).toEqual(request);
    expect(capabilities.selection).toEqual(request);
    const { generatedPrerequisiteArgumentsV1 } = await import("../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🏭️generated-owner/🟦️.ts");
    expect(capabilities.launches).toEqual([request, ...(await generatedPrerequisiteArgumentsV1(root)).map(child => ["repo", ...child])]);
    const envelope = createScriptProcessEnvelope({ version: 1, owner: "test:dev-playground", maximumElapsedMilliseconds: 10000 }, capabilities, Date.now());
    await withScriptProcessEnvelope(envelope, async original => {
      expect(readOwnedBuildInvocationV1(root, original, argv).buildMode).toBe(profile);
      expect(() => readOwnedBuildInvocationV1(root, original, [...argv.slice(0, 2), "different", ...argv.slice(3)])).toThrow("Undeclared original Repo build selection");
    });
  }
  await expect(rootDevPlaygroundFreshCapabilitiesV1(root, playgroundCases[0]!.argv, { SEMIO_BUILD_MODE: "release" })).rejects.toThrow("Playground profile disagrees");
});


test("graph-free native playground selects the authored host recipe for both profiles", async () => {
  const workspace = resolve(import.meta.dirname, "../../../../../../../../..");
  const { nativeHostFilesystemViewV1, nativeHostTaskV1 } = await import("../../../../🎮️playground/🖥️native-host/🟦️.ts");
  const manifest = readFileSync(join(workspace, gisContribution.manifest), "utf8");
  const { parse } = await import("@iarna/toml");
  const parsed = parse(manifest) as unknown as {package:{metadata:{semio:{playground:{variant:string;nativeHost?:{cratePath:string;target:string}}[]}}}};
  const host = parsed.package.metadata.semio.playground.find(row => row.variant === "gis2d")!.nativeHost!;
  const view = nativeHostFilesystemViewV1(workspace);
  const projectPath = host.cratePath + "/📋️project.json";
  const project = JSON.parse(view.readText(projectPath));
  const target = project.targets[host.target];
  for (const mode of ["prototype", "release"] as const) {
    const recipe = nativeHostTaskV1(host, mode, view);
    expect(recipe.arguments).toEqual(["build", "native", mode]);
    expect(recipe.arguments.slice(2)).toEqual(target.configurations[mode].args);
    expect(recipe.outputs).toEqual(target.outputs);
    expect(recipe.script).toBe(host.cratePath + "/📜️script.ts");
  }
  for (const command of ["repo ./📜️script.ts build; other", "repo ../📜️script.ts build", "repo ./📜️script.ts build native --extra"]) {
    const change = {...structuredClone(project)};
    change.targets[host.target].options.command = command;
    expect(() => nativeHostTaskV1(host, "prototype", {...view,readText:path => path === projectPath ? JSON.stringify(change) : view.readText(path)})).toThrow();
  }
  const change = {...structuredClone(project)};
  change.targets[host.target].dependsOn = ["generate"];
  expect(() => nativeHostTaskV1(host, "prototype", {...view,readText:path => path === projectPath ? JSON.stringify(change) : view.readText(path)})).toThrow();
});

test("the native host child receives the exact playground Original and profile", async () => {
  const workspace = resolve(import.meta.dirname, "../../../../../../../../..");
  const argv = playgroundCases.find(row => row.argv[3] === "native" && row.profile === "prototype")!.argv;
  const capabilities = await rootDevPlaygroundFreshCapabilitiesV1(workspace, argv, {});
  const { nativeCallerPhaseV1 } = await import("../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts");
  const { captureOwnedCommandV1 } = await import("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts");
  const envelope = createScriptProcessEnvelope({ version: 1, owner: "test:native-host-child", maximumElapsedMilliseconds: 10000 }, capabilities, Date.now());
  await withScriptProcessEnvelope(envelope, async original => {
    const phase = nativeCallerPhaseV1(original, workspace, createIssuedResourceControlV1(original, "test:native-host-child", 0));
    const options = phase.options({ ...process.env, SEMIO_BUILD_MODE: "prototype" });
    const source = `console.log(JSON.stringify({ profile: process.env.SEMIO_BUILD_MODE, capabilities: JSON.parse(process.env[${JSON.stringify(SCRIPT_PROCESS_INVOCATION_ENV)}]).capabilities }));`;
    const output = await captureOwnedCommandV1(process.execPath, ["-e", source], workspace, { ...options, control: phase.leaseControl });
    expect(JSON.parse(output)).toEqual({ profile: "prototype", capabilities });
  });
});

test("graph-free playground stages exactly the missing components needed to boot", () => {
  const database = new Database(":memory:");
  try {
    const missing = database.query(`
      SELECT boot.value AS id
      FROM json_each($boot) AS boot
      JOIN json_each($staging) AS staging ON json_extract(staging.value, '$.pluginId') = boot.value
      WHERE NOT (json_extract(staging.value, '$.directoryPresent') AND
                 COALESCE(json_extract(staging.value, '$.descriptorPluginId') = boot.value, 0) AND
                 json_extract(staging.value, '$.bridgePresent') AND
                 json_extract(staging.value, '$.artifactMarkerPresent'))
      ORDER BY boot.key`);
    for (const row of stagingCases) {
      expect(missing.all({ $boot: JSON.stringify(row.boot), $staging: JSON.stringify(row.staging) }), row.id)
        .toEqual(row.build.map(id => ({ id })));
      expect([...missingPlaygroundBootComponentsV1(new Set(row.boot), id => row.staging.find(facts => facts.pluginId === id)!)], row.id)
        .toEqual(row.build);
    }
  } finally { database.close(); }
});

test("graph-free playground engine deliveries load the declared wasm command as a module", async () => {
  const workspace = resolve(import.meta.dirname, "../../../../../../../../..");
  const engineModule = await import("../../../../../../../💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🕸️engines/🟦️.ts");
  const { default: catalog } = await import("../../../../../../../💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🕸️engines/📇️catalog.json");
  const database = new Database(":memory:");
  const select = database.query("SELECT json_extract(value,'$.sourceOwner') AS owner FROM json_each(?1) WHERE ?2='play' OR json_extract(value,'$.id')<>'puzzle' ORDER BY key");
  try {
    for (const facet of ["renderer", "play"] as const) {
      const deliveries = engineModule.frameworkOsEngineDeliveriesV1(join(workspace, ".fixture-products"), { version: 1, mode: "prototype", facet });
      expect(deliveries.map(({ sourceOwner }) => sourceOwner)).toEqual((select.all(JSON.stringify(catalog.packages), facet) as { owner: string }[]).map(({ owner }) => owner));
      for (const { sourceOwner } of deliveries) {
        const project = JSON.parse(readFileSync(join(workspace, sourceOwner, "📋️project.json"), "utf8"));
        expect(project.targets.wasm.options.command).toBe("repo ./📜️script.ts wasm");
        const previousStack = process.env.RUST_MIN_STACK;
        const { WasmScript } = await import(pathToFileURL(join(workspace, sourceOwner, "📜️script.ts")).href);
        expect(WasmScript.prototype instanceof BundleScript).toBe(true);
        expect(process.env.RUST_MIN_STACK).toBe(previousStack);
      }
    }
  } finally { database.close(); }
});

test("graph-free playground stages a missing selected-mode descriptor emitter", async () => {
  const { playgroundDescriptorEmitterMissingV1 } = await import("../../🎮️playground/🟦️.ts");
  const products = resolve(".fixture-products"), database = new Database(":memory:");
  const request = database.query(`SELECT
    'browser/' || ?1 || '/tools/descriptor-emitter/' || ?2 AS root,
    'semio-plugin-describe' || CASE WHEN ?2 = 'win32' THEN '.exe' ELSE '' END AS binary`);
  try {
    for (const row of descriptorCases) {
      const expected = request.get(row.mode, row.platform) as { root: string; binary: string };
      expect(playgroundDescriptorEmitterMissingV1(products, row.mode, row.platform, (root, files) => {
        expect(root).toBe(join(products, expected.root));
        expect(files).toEqual([".semio-artifact.json", expected.binary]);
        return files.some(file => !row.files.includes(file));
      })).toBe(row.build);
    }
  } finally { database.close(); }
});

test("graph-free component preparation keeps exact selected Cargo passes", async () => {
  const { componentOwnerPreparationArgumentsV1 } = await import("../../../../../../../💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts");
  const database = new Database(":memory:");
  const query = database.query(`SELECT json_array('build', '--manifest-path', value ->> '$.manifest',
    '--profile', value ->> '$.profile', '--target', value ->> '$.target') AS prefix,
    (SELECT json_group_array(value ->> '$.name' ORDER BY key)
     FROM json_each(pass.value, '$.packages')) AS packages
    FROM json_each(?1) AS pass ORDER BY key`);
  try {
    for (const { passes, requests } of preparationCases) {
      const expected = (query.all(JSON.stringify(passes)) as { prefix: string; packages: string }[])
        .map(({ prefix, packages }) => [...JSON.parse(prefix), ...(JSON.parse(packages) as string[]).flatMap(name => ["-p", name])]);
      expect(passes.map(componentOwnerPreparationArgumentsV1)).toEqual(expected);
      expect(expected).toEqual(requests);
    }
  } finally { database.close(); }
});

test("native transpile publishes only through the products partition store, never into a plain directory of a fresh workspace", () => {
  const source = readFileSync(resolve(import.meta.dirname, "../../../../../../../💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🦀️native-transpile/📜️script.ts"), "utf8");
  expect(source).toContain('withRepositoryCacheStoreV1(workspace, "products"');
  expect(source).toContain("stageRepositoryArtifacts(");
  expect(source).not.toMatch(/\b(mkdirSync|copyFileSync|renameSync|writeFileSync)\b/u);
});

test("graph-free playground prepares the selected-mode transpiler and validates configured overrides", async () => {
  const { nativeTranspileRequiresBuildV1 } = await import("../../../../../../../💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🦀️native-transpile/🟦️.ts");
  const { ensureNativeTranspileV1 } = await import("../../../../../../../💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🦀️native-transpile/📜️script.ts");
  expect(typeof ensureNativeTranspileV1).toBe("function");
  expect(() => nativeTranspileRequiresBuildV1(process.cwd(), "prototype", { SEMIO_NATIVE_TRANSPILE_BIN: "." })).toThrow("Native transpile executable is invalid");
  const workspace = resolve(".fixture-workspace"), database = new Database(":memory:");
  const select = database.query(`SELECT COALESCE($override, '.cache/b/p/m/browser/' || $mode || '/tools/native-transpile/' || $platform ||
    '/semio-native-transpile' || CASE WHEN $platform = 'win32' THEN '.exe' ELSE '' END) AS path,
    CASE WHEN $present THEN 'ready' WHEN $override IS NULL THEN 'build' ELSE 'refused' END AS outcome`);
  try {
    for (const { platform, mode, override, present, outcome } of transpilerCases) {
      const expected = select.get({ $platform: platform, $mode: mode, $override: override, $present: Number(present) }) as { path: string; outcome: string };
      expect(expected.outcome).toBe(outcome);
      const path = resolve(workspace, expected.path), inspected: string[] = [];
      const invoke = () => nativeTranspileRequiresBuildV1(workspace, mode as "prototype" | "release", { SEMIO_NATIVE_TRANSPILE_BIN: override ?? undefined }, platform, actual => {
        inspected.push(actual);
        return actual === path && present;
      });
      if (outcome === "refused") expect(invoke).toThrow(path);
      else expect(invoke()).toBe(outcome === "build");
      expect(inspected).toEqual([path]);
    }
  } finally { database.close(); }
});

test("graph-free component transpile cache identifies the actual native executable bytes", async () => {
  const { transpileStageRequestV1 } = await import("../../../../../../../💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts");
  const database = new Database(":memory:");
  try {
    const native = ["a".repeat(64), "b".repeat(64)];
    const stages = native.map(hash => transpileStageRequestV1("component", "jco", "materialization", "base", hash));
    const actual = database.query(`SELECT json_extract(value, '$.tools."native-transpile"') AS native FROM json_each(?1) ORDER BY key`).all(JSON.stringify(stages));
    expect(actual).toEqual(native.map(hash => ({ native: hash })));
  } finally { database.close(); }
});


test("standalone GIS and Space select their own Vite entry and engine facet", async () => {
  const workspace = resolve(import.meta.dirname, "../../../../../../../../..");
  const catalog = join(workspace, gisContribution.catalog);
  const [{ resolvePlaygroundFilter }, { loadDevContribution }, engines, { specificOsDevBuildInventoryV1 }, { createAssetBuildPluginsV1 }] = await Promise.all([
    import("@semio-tech/framework-plugin/build/plan"),
    import("../../../../../../../💻️os/🔨️modules/🧑‍💻dev/🧩️contribution/📥️loading/🟦️.ts"),
    import("../../../../../../../💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🕸️engines/🟦️.ts"),
    import("../../../../../../../../../✏️s/🧑‍💻dev/🧩️catalog/🏗️builder/🟦️.ts"),
    import("@semio-tech/assets/resolver/dispatch"),
  ]);
  const previousCatalog = process.env.SEMIO_PLUGIN_CATALOG, previousPlugin = process.env.SEMIO_PLUGIN, database = new Database(":memory:");
  process.env.SEMIO_PLUGIN_CATALOG = catalog;
  const select = database.query(`SELECT value ->> '$.devContribution' AS contribution, value ->> '$.app' AS app
    FROM json_each(?1) WHERE value ->> '$.variant' = ?2`);
  try {
    const manifest = Bun.TOML.parse(readFileSync(join(workspace, gisContribution.manifest), "utf8")) as { package: { metadata: { semio: { playground: unknown } } } };
    const inventory = specificOsDevBuildInventoryV1();
    for (const row of gisContribution.variants) {
      const { variant, app, contribution, browserEntry, facet: expectedFacet, engines: expectedEngines, factoryModules } = row;
      const expected = { contribution, app };
      for (const projection of [inventory.playgrounds,
        JSON.parse(readFileSync(join(catalog, "🚀️playgrounds.json"), "utf8")),
        JSON.parse(readFileSync(join(catalog, "🎠️playgrounds.json"), "utf8"))]) {
        expect(select.get(JSON.stringify(projection), variant), variant).toEqual(expected);
      }
      if (app !== null) expect(select.get(JSON.stringify(manifest.package.metadata.semio.playground), variant), variant).toEqual(expected);
      const selected = loadDevContribution(workspace, resolvePlaygroundFilter(variant));
      expect(selected?.viteConfig).toBe(row.viteConfig);
      expect(selected?.browserEntry).toBe(browserEntry);
      const { default: viteConfig } = await import(pathToFileURL(join(workspace, selected!.viteConfig)).href);
      expect(typeof viteConfig).toBe("function");
      const facet = engines.frameworkOsEngineFacetV1(selected?.browserSessionFactories ?? []);
      expect(facet).toBe(expectedFacet);
      expect(selected?.browserSessionFactories?.map(({ module }) => module) ?? []).toEqual(factoryModules);
      const entry = readFileSync(join(workspace, browserEntry), "utf8");
      expect(entry.includes("@semio-tech/puzzle-2d")).toBe(factoryModules.length > 0);
      expect(engines.frameworkOsEngineDeliveriesV1(join(workspace, ".fixture-products"), { version: 1, mode: "prototype", facet }).map(({ id }) => id)).toEqual(expectedEngines);
      if (row.routes) {
        const target = inventory.playgrounds.find(target => target.variant === variant)!;
        const routes = row.routes.map(route => "tile-proxy-serve" + route);
        expect(createAssetBuildPluginsV1(workspace, target.assets, inventory.assetProviders).map(plugin => plugin.name)).toEqual(routes);
        process.env.SEMIO_PLUGIN = variant;
        const configured = await viteConfig({ command: "serve", mode: "prototype" });
        expect(configured.plugins.flat(Infinity).map((plugin: { name?: string } | undefined) => plugin?.name)
          .filter((name: string | undefined) => name?.startsWith("tile-proxy-serve"))).toEqual(routes);
      }
    }
  } finally {
    database.close();
    if (previousPlugin === undefined) delete process.env.SEMIO_PLUGIN;
    else process.env.SEMIO_PLUGIN = previousPlugin;
    if (previousCatalog === undefined) delete process.env.SEMIO_PLUGIN_CATALOG;
    else process.env.SEMIO_PLUGIN_CATALOG = previousCatalog;
  }
});

test("fresh native root retries only after failed Original custody is retired", async () => {
  const workspace = resolve(import.meta.dirname, "../../../../../../../../.."), argv = [...playgroundCases[0]!.argv];
  const db = new Database(":memory:");
  const previousWire = process.env[SCRIPT_PROCESS_INVOCATION_ENV];
  const attempts = db.query("SELECT CASE WHEN ?1='fresh' AND ?2='source' THEN 2 ELSE 1 END AS count");
  try {
    delete process.env[SCRIPT_PROCESS_INVOCATION_ENV];
    expect(rootCommandRetriesFreshBuildV1(argv)).toBe(true);
    expect(rootCommandRetriesFreshBuildV1(["components", "describe", "raster"])).toBe(true);
    expect(rootCommandRetriesFreshBuildV1(["components", "iterate", "raster"])).toBe(false);
    expect(rootCommandRetriesFreshBuildV1([...argv, "--port", "9000"])).toBe(false);
    for (const row of restartCases) {
      const failure = row.failure === "source" ? concurrentEditV1("bytes", "/fixture.rs", "fixture source changed", "build")
        : row.failure === "interrupted" ? new ScriptProcessInterruptedV1() : new Error("fixture compiler refusal");
      const capabilities = await rootDevPlaygroundFreshCapabilitiesV1(workspace, argv, {});
      const seen: ScriptInvocation[] = [];
      let issued = 0;
      class Command extends Script {
        async run(segments: string[]): Promise<void> {
          expect(segments).toEqual(argv.slice(1));
          expect(readOwnedBuildInvocationV1(workspace, this.invocation, argv).buildMode).toBe("prototype");
          if (seen.length) expect(() => requireIssuedScriptInvocationV1(seen.at(-1)!)).toThrow("Live Original process issuance required");
          seen.push(this.invocation);
          const phase = createIssuedResourceControlV1(this.invocation, "test:root-retry", 0);
          await withIssuedResourceOperationV1(phase, async () => { if (seen.length === 1) throw failure; });
        }
      }
      const router = new ScriptRouter(workspace, workspace).register("dev", Command);
      const options = { argv, retryFreshBuild: row.mode !== "ordinary", freshCapabilities: async () => { issued++;return capabilities!; } };
      const run = row.mode === "forwarded" ? withScriptProcessEnvelope(
        createScriptProcessEnvelope({version:1,owner:"test:forwarded-root",maximumElapsedMilliseconds:10000},capabilities!,Date.now()),
        async original => {
          process.env[SCRIPT_PROCESS_INVOCATION_ENV] = scriptInvocationEnvironment(original, {})[SCRIPT_PROCESS_INVOCATION_ENV]!;
          try { await runWorkspaceScriptMain(router, options); }
          finally { delete process.env[SCRIPT_PROCESS_INVOCATION_ENV]; expect(seen[0]).toBe(original); }
        },
      ) : runWorkspaceScriptMain(router, options);
      if (row.succeeds) await run;
      else await expect(run).rejects.toBe(failure);
      expect(seen.length,row.id).toBe((attempts.get(row.mode,row.failure) as {count:number}).count);
      expect(seen.length,row.id).toBe(row.attempts);
      expect(new Set(seen).size).toBe(seen.length);
      expect(issued).toBe(row.mode === "forwarded" ? 0 : 1);
    }
  } finally {
    db.close();
    if (previousWire === undefined) delete process.env[SCRIPT_PROCESS_INVOCATION_ENV];
    else process.env[SCRIPT_PROCESS_INVOCATION_ENV] = previousWire;
  }
});

test("cold playground retains the original runtime through import, closed stores and serve", async () => {
  const ts = await import("typescript");
  const source = readFileSync(new URL("../../🟦️.ts", import.meta.url), "utf8");
  const tree = ts.createSourceFile("dev.ts", source, ts.ScriptTarget.Latest, true);
  const actual: string[] = [];
  const visit = (node: import("typescript").Node): void => {
    if (ts.isCallExpression(node)) {
      const name = node.expression.getText(tree);
      if (name === "withRepositoryCompilerRuntimeEntriesV1") {
        expect(node.arguments.slice(0, 3).map(argument => argument.getText(tree))).toEqual(["original.codeRoot", "this.invocation", "control"]);
        actual.push("open:runtime");
        ts.forEachChild(node, visit);
        actual.push("close:runtime");
        return;
      }
      if (name === "withRepositoryCacheStoreV1") {
        const partition = node.arguments[1];
        if (partition && ts.isStringLiteral(partition)) {
          if (partition.text === "dependencies") expect(ts.isAwaitExpression(node.parent)).toBe(true);
          actual.push(`open:${partition.text}`);
          ts.forEachChild(node, visit);
          actual.push(`close:${partition.text}`);
          return;
        }
      }
      if (name === "import" && node.arguments[0]?.getText(tree).includes("🎮️playground")) actual.push("import");
      if (name === "serveRootPlaygroundV1") actual.push("serve");
    }
    ts.forEachChild(node, visit);
  };
  visit(tree);
  const database = new Database(":memory:");
  try {
    const oracle = database.query(`
      WITH transitions AS (SELECT key AS position, value AS step FROM json_each($steps))
      SELECT
        (SELECT position FROM transitions WHERE step = 'open:runtime') <
        (SELECT position FROM transitions WHERE step = 'open:dependencies') AND
        (SELECT position FROM transitions WHERE step = 'open:dependencies') <
        (SELECT position FROM transitions WHERE step = 'open:products') AND
        (SELECT position FROM transitions WHERE step = 'open:products') <
        (SELECT position FROM transitions WHERE step = 'import') AND
        (SELECT position FROM transitions WHERE step = 'import') <
        (SELECT position FROM transitions WHERE step = 'close:products') AND
        (SELECT position FROM transitions WHERE step = 'close:products') <
        (SELECT position FROM transitions WHERE step = 'close:dependencies') AND
        (SELECT position FROM transitions WHERE step = 'close:dependencies') <
        (SELECT position FROM transitions WHERE step = 'serve') AND
        (SELECT position FROM transitions WHERE step = 'serve') <
        (SELECT position FROM transitions WHERE step = 'close:runtime') AS valid
    `);
    for (const row of coldPlaygroundCases) {
      expect(oracle.get({ $steps: JSON.stringify(row.steps) }), row.id).toEqual({ valid: Number(row.valid) });
    }
    expect(actual).toEqual(coldPlaygroundCases.find(row => row.valid)?.steps);
  } finally {
    database.close();
  }
});
