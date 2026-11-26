import {expect,test} from "bun:test";
import {mkdtemp,rm} from "node:fs/promises";
import {availableParallelism} from "node:os";
import {join} from "node:path";
import {withCompilerCapacityScopeV1,acquireCompilerCapacityLeaseV1,scriptCompilerAdmissionControlV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {withBuildTestInvocationV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {declareLibraryBuildTestEnvelopeV1} from "../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import {resolveConfig,type InlineConfig} from "vite";
import {readViteServiceCompilerCapacityV1,requireViteServiceCompilerConfigurationV1,type ViteServiceCompilerPolicyV1} from "../🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

declareLibraryBuildTestEnvelopeV1({suite:"vite",maximumElapsedMilliseconds:15000});

test("cold startup and transforms can take the last free core only with the authored captured policy", () => {
  for (const row of fixture.cases) {
    const policy = (row.policy ?? undefined) as ViteServiceCompilerPolicyV1 | undefined;
    if (!row.request) {
      expect(() => readViteServiceCompilerCapacityV1(policy,row.captured,fixture.hostCores)).toThrow();
      continue;
    }
    const request = readViteServiceCompilerCapacityV1(policy,row.captured,fixture.hostCores);
    expect<unknown>(request).toEqual(row.request);
    const minimum = request.kind === "exact" ? request.parallelism : request.kind === "available" ? request.minimumParallelism ?? 1 : fixture.hostCores;
    expect(minimum <= fixture.freeCores).toBe(row.admitted);
  }
});

test("the projected service guard agrees with the real Vite resolver after plugins run", async () => {
  for (const row of fixture.resolvedCases) {
    const options: InlineConfig = {
      configFile: false, envFile: false, dev: {preTransformRequests: false},
      optimizeDeps: {noDiscovery: true, include: []}, server: {watch: null, preTransformRequests: false},
      plugins: [{name: "test:resolved-custody", configResolved(config) {
        const client = config.environments.client!;
        switch (row.change) {
          case "config-file": Object.assign(config, {configFile: "test.config.ts"}); break;
          case "watcher": config.server.watch = {}; break;
          case "dependency-discovery": client.optimizeDeps.noDiscovery = false; break;
          case "ssr-dependency-discovery": config.environments.ssr!.optimizeDeps.noDiscovery = false; break;
          case "explicit-optimizer-include": client.optimizeDeps.include = ["vite"]; break;
          case "pretransform": client.dev.preTransformRequests = true; break;
          case "warmup": client.dev.warmup = ["index.html"]; break;
        }
      }}],
    };
    const resolved = await resolveConfig(options, "serve");
    const verify = () => requireViteServiceCompilerConfigurationV1(resolved);
    if (row.accepted) expect(verify).not.toThrow();
    else expect(verify).toThrow();
  }
});

test("both cold operations obtain and release the one remaining real compiler lease", () =>
  withBuildTestInvocationV1(async invocation => {
    const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!artifacts) throw Error("A private test artifact directory is required");
    const directory = await mkdtemp(join(artifacts, "vite-capacity-"));
    const control = scriptCompilerAdmissionControlV1(invocation,"test:vite-service-compiler-admission",10_000);
    const host = availableParallelism();
    try {
      const holder = host > 1 ? await acquireCompilerCapacityLeaseV1({directory,control,environment:{},request:{kind:"exact",parallelism:host-1}}) : undefined;
      try {
        const request = readViteServiceCompilerCapacityV1("projected-no-optimizer", true);
        for (const phase of ["startup","transform"]) {
          const result = await withCompilerCapacityScopeV1({directory,control,environment:{},request}, async facts => ({phase,parallelism:facts.parallelism}));
          expect(result).toEqual({phase,parallelism:1});
        }
      } finally { holder?.release(); }
    } finally { await rm(directory,{force:true,recursive:true}); }
  }));
