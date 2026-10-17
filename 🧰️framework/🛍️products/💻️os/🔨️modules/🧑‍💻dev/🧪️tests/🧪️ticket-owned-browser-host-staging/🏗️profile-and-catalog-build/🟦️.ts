/** 🏗️ The WASI component profile policy, cargo component settings, the concurrency limiter and the bounded-parallel plugin catalog build of the ticket-owned browser host staging suite. */
import { phaseCompilerAdmissionControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { createTestKit, type TestDependencies, type TestSource } from "../🧰️test-kit/🟦️.ts";

export async function registerProfileAndCatalogBuildTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: TestDependencies, source: TestSource): Promise<void> {
  const { assertPluginCatalogComplete, buildPluginCatalog, cargoProfileDir, componentCargoV1, componentProfileV1, COMPONENT_PROFILE_POLICIES, SHIPPED_COMPONENT_PROFILE, pluginWasmArtifactPath, createConcurrencyLimiter, dirname, fileURLToPath, join, readFileSync, repoRoot } = dependencies;
  const { describe, expect, it } = vitest;
  const { itLong } = createTestKit(vitest, dependencies, source);

  type PluginBuildTargetV1 = import("../../../../🔌️plugin/📇️registry/🔎️discovery/🟦️.ts").PluginBuildTargetV1;

  //#region 🔖️T-P8-tests
  describe("WASI component profile policy", () => {
    itLong("ships one profile and iterates one plugin, as the strict policy fixture and the root manifest state", async () => {
      const { default: toml } = await import("@iarna/toml");
      const fixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), "../../🧫️fixtures/🦀️wasm-profile-policy/🧬️v1/🔣️.json"), "utf8"));
      expect(SHIPPED_COMPONENT_PROFILE).toBe(fixture.shippedProfile);
      expect(Object.keys(COMPONENT_PROFILE_POLICIES)).toEqual(fixture.runtimeDirectories);
      for (const vector of fixture.cases) {
        if (!vector.valid) {
          expect(() => componentProfileV1(vector.profile)).toThrow();
          continue;
        }
        const profile = componentProfileV1(vector.profile);
        expect(COMPONENT_PROFILE_POLICIES[profile].shipped).toBe(vector.shipped);
        expect(cargoProfileDir(profile)).toBe(profile);
        expect(componentCargoV1("semio-hub-vcs", profile).args).toEqual(vector.expectedArgs);
      }
      for (const row of fixture.artifactPaths) expect(pluginWasmArtifactPath(repoRoot, row.packageName, row.profile, "/target")).toBe(join("/target", row.expectedRelativePath));
      const manifest = toml.parse(readFileSync(join(repoRoot, "Cargo.toml"), "utf8")) as any;
      expect(manifest.profile).toBeUndefined();
      const {renderCargoProfileConfigurationV1}=await import("../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🟦️.ts");
      for(const mode of ["prototype","release"] as const){const rendered=renderCargoProfileConfigurationV1({version:1,mode,target:"wasm32-wasip2",producer:"wasm-component",operation:"build"},"Cargo.toml","repository"),parsed=toml.parse(rendered.selectors.join("\n")) as {profile:Record<string,Record<string,unknown>>};expect(parsed.profile[mode]!["opt-level"]).toBe(mode==="release"?"s":1);expect(rendered.plan.profile).toBe(mode);}
    });
  });

  describe("componentCargoV1", () => {
    it("links every actor component exactly as the release driver's pass does, with the guest stack in the cargo config", async () => {
      const cargo = componentCargoV1("semio-hub-procedural", "release");
      expect(cargo.command).toBe("build");
      expect(cargo.args).toEqual(["-p", "semio-hub-procedural", "--lib", "--target", "wasm32-wasip2", "--profile", "release"]);
      expect(cargo.environment).toEqual({ CARGO_UNSTABLE_FEATURE_UNIFICATION: "true", CARGO_RESOLVER_FEATURE_UNIFICATION: "package" });
      const { default: toml } = await import("@iarna/toml");
      const config = toml.parse(readFileSync(join(repoRoot, ".cargo/config.toml"), "utf8")) as any;
      const rustflags: readonly string[] = config.target["wasm32-wasip2"].rustflags;
      expect(rustflags).toContain("link-arg=-zstack-size=8388608");
      expect(rustflags).toContain("link-arg=--max-memory=536870912");
    });

    it("can retain actor symbols for a reproducible browser trap diagnosis", () => {
      process.env.SEMIO_PLUGIN_SYMBOLS = "1";
      try {
        const cargo = componentCargoV1("semio-hub-procedural", "prototype");
        expect(cargo.command).toBe("rustc");
        expect(cargo.args.slice(-2)).toEqual(["-C", "strip=none"]);
      } finally {
        delete process.env.SEMIO_PLUGIN_SYMBOLS;
      }
    });
  });

  describe("createConcurrencyLimiter (T-P8 bounded-parallel materialize primitive)", () => {
    it("never runs more than `limit` callbacks concurrently, and still runs every one to completion", async () => {
      const limiter = createConcurrencyLimiter(2);
      let active = 0;
      let maxActive = 0;
      const tasks = Array.from({ length: 6 }, (_, i) =>
        limiter.run(async () => {
          active++;
          maxActive = Math.max(maxActive, active);
          await new Promise((r) => setTimeout(r, 5));
          active--;
          return i;
        }),
      );
      const results = await Promise.all(tasks);
      expect(results.slice().sort((a, b) => a - b)).toEqual([0, 1, 2, 3, 4, 5]);
      expect(maxActive).toBeLessThanOrEqual(2);
      expect(maxActive).toBe(2); // proves it actually overlaps, not just an accidental serialization
    });
  });

  describe("buildPluginCatalog (T-P8: bounded-parallel cargo and materialize stages)", () => {
    it("rejects incomplete explicit builds instead of accepting stale plugin artifacts", () => {
      expect(() => assertPluginCatalogComplete([])).not.toThrow();
      expect(() => assertPluginCatalogComplete(["cargo-fails", "materialize-fails"])).toThrow("plugin catalog build failed: cargo-fails, materialize-fails");
    });

    const fakeTarget = (pluginId: string): PluginBuildTargetV1 => ({
      pluginId,
      packageId: `semio-s-plugin-${pluginId}`,
      cratePath: "",
      packageName: pluginId,
      wasmOut: `${pluginId}.wasm`,
      directoryName: `🧪️${pluginId}` as PluginBuildTargetV1["directoryName"],
      role: "plugin",
      consumes: [],
      dependsOn: [],
    });

    it("overlaps cargo and materialize up to the cap, and emits every plugin", async () => {
      const targets = Array.from({ length: 6 }, (_, i) => fakeTarget(`p${i}`));
      let cargoActive = 0;
      let maxCargoActive = 0;
      let materializeActive = 0;
      let maxMaterializeActive = 0;
      const materialized: string[] = [];
      const cargoFn = async (target: PluginBuildTargetV1) => {
        cargoActive++;
        maxCargoActive = Math.max(maxCargoActive, cargoActive);
        await new Promise((r) => setTimeout(r, 5));
        cargoActive--;
        return { deliverable: { profile: "prototype" as const, directory: target.pluginId, wasm: `${target.pluginId}.wasm`, receipt: `${target.pluginId}.json` } };
      };
      const materializeFn = async (target: PluginBuildTargetV1) => {
        materializeActive++;
        maxMaterializeActive = Math.max(maxMaterializeActive, materializeActive);
        await new Promise((r) => setTimeout(r, 15));
        materializeActive--;
        materialized.push(target.pluginId);
      };
      let shardWorkerPublishCount = 0;
      const { failedPluginIds } = await buildPluginCatalog(targets, phaseCompilerAdmissionControlV1(new AbortController().signal, 120000, () => undefined, () => new Promise(done => setImmediate(done))), cargoFn, materializeFn, 3, () => {
        shardWorkerPublishCount++;
      }, 3);
      expect(failedPluginIds).toEqual([]);
      expect(materialized.slice().sort()).toEqual(targets.map((t) => t.pluginId).sort());
      expect(maxCargoActive).toBeGreaterThan(1);
      expect(maxCargoActive).toBeLessThanOrEqual(3);
      expect(maxMaterializeActive).toBeGreaterThan(1);
      expect(maxMaterializeActive).toBeLessThanOrEqual(3); // never past the cap
      expect(shardWorkerPublishCount).toBe(1); // once per catalog build, not once per plugin
    });

    it("continues past both a cargo failure and a materialize failure, reporting each pluginId exactly once", async () => {
      const targets = [fakeTarget("ok"), fakeTarget("cargo-fails"), fakeTarget("materialize-fails")];
      const cargoFn = async (target: PluginBuildTargetV1) => {
        if (target.pluginId === "cargo-fails") throw new Error("boom");
        return { deliverable: { profile: "prototype" as const, directory: target.pluginId, wasm: `${target.pluginId}.wasm`, receipt: `${target.pluginId}.json` } };
      };
      const materialized: string[] = [];
      const materializeFn = async (target: PluginBuildTargetV1) => {
        if (target.pluginId === "materialize-fails") throw new Error("boom");
        materialized.push(target.pluginId);
      };
      const { failedPluginIds } = await buildPluginCatalog(targets, phaseCompilerAdmissionControlV1(new AbortController().signal, 120000, () => undefined, () => new Promise(done => setImmediate(done))), cargoFn, materializeFn, 4, () => {}, 4);
      expect(new Set(failedPluginIds)).toEqual(new Set(["cargo-fails", "materialize-fails"]));
      expect(materialized).toEqual(["ok"]);
    });
  });
  //#endregion 🔖️T-P8-tests
}
