import {consumeOwnerArgumentsV1} from "../../../🔌️nx-plugin/📤️arguments/🟨️.mjs";
import {nativeOwnerTestManifestRequestV1} from "./🗺️owner-test-manifests/🟦️.ts";
import { buildBudgetMs } from "../../../../../../../🔨️modules/🏃️process/⏱️budget/🟦️.ts";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { BundleScript } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { CARGO_RELAY_BUDGET_ENV, cargoStreamingStatus, runCmdStatus } from "../../../🏃️process/🟦️.ts";
import { repositoryProcessOwnerContextV1, repositoryVitestPolicyV1, repositoryCargoTestPolicyV1, repositoryWasmBuildPolicyV1 } from "../../../🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import { runRepositoryCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { repositoryCargoArtifactBuildPolicyV1, buildRepositoryCargoArtifacts } from "../🏗️native-build/🟦️.ts";
import { validateNativeCargoArguments } from "../🎛️native-input/🟦️.ts";

/** 🧩️ The ONE cargo `rustc` argument list that links a plugin/extension component: `component-<profile>` and
 * the plugin's own `describe` both build exactly this unit, so the shared build-dir compiles it once and the
 * described bytes are the shipped bytes. */
export function pluginComponentRustcArgs(packageName: string, profile: string): string[] {
  return ["-p", packageName, "--lib", "--crate-type", "cdylib", "--target", "wasm32-wasip2", "--profile", profile, ...(process.env.SEMIO_PLUGIN_SYMBOLS === "1" ? ["--", "-C", "strip=none"] : [])];
}

/** 🌊️ `bun ⚡️caching/🦀️cargo/📜️script.ts relay <cargo args…>` — the relay [[runCmd]] routes every POSIX `cargo` through
 * ([[cargoStreamingStatus]]): exits with cargo's status, 1 when cargo was stopped (budget, signal), after its forwarded output drained. */
export class CargoRelayScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const budgetMs = Number(process.env[CARGO_RELAY_BUDGET_ENV] ?? 0);
    try {
      process.exitCode = await cargoStreamingStatus(args, process.cwd(), process.env, Number.isFinite(budgetMs) ? budgetMs : 0);
    } catch (error) {
      console.error(error instanceof Error ? error.message : String(error));
      process.exitCode = 1;
    }
  }
}

/** 🦀️ Routes native Cargo and component operations to their owned behaviors. */
export class NativeScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [tool, operation] = args;
    const index = args.indexOf("--manifest");
    const manifest = index >= 0 ? args[index + 1] : undefined;
    if(tool==="owner-command"){
      const request=nativeOwnerTestManifestRequestV1(args.slice(1)),path=resolve(this.repoRoot,request.manifest),cwd=resolve(this.repoRoot,request.cwd);
      const cargo = Bun.TOML.parse(readFileSync(path, "utf8")) as { package?: { name?: string }; workspace?: object };
      if (!cargo.package?.name && !cargo.workspace) throw Error(`Native owner requires a package or workspace manifest: ${manifest}`);
      const transported=consumeOwnerArgumentsV1(process.env);
      const env: Record<string,string|undefined>={...transported.environment,SEMIO_VITEST_POLICY:JSON.stringify(repositoryVitestPolicyV1(cwd)),SEMIO_PROCESS_OWNER_CONTEXT:JSON.stringify(repositoryProcessOwnerContextV1(cwd)),SEMIO_CARGO_ARTIFACT_POLICY:JSON.stringify(repositoryCargoArtifactBuildPolicyV1(cwd))};
      delete env.SEMIO_CARGO_TEST_POLICY;
      if (cargo.package?.name) Object.assign(env, { SEMIO_CARGO_TEST_POLICY: JSON.stringify(repositoryCargoTestPolicyV1(path,cwd)) });
      const policies=request.testManifests.map(manifest=>{
        const selected=resolve(this.repoRoot,manifest);
        return repositoryCargoTestPolicyV1(selected,cwd);
      });
      if(new Set(policies.map(policy=>policy.manifestPath)).size!==policies.length)throw Error("Duplicate native test manifest authority");
      env.SEMIO_CARGO_TEST_POLICIES=JSON.stringify(policies);
      if (process.env.SEMIO_WASM_BUILD_REQUIRED === "1") Object.assign(env,{SEMIO_WASM_BUILD_POLICY:JSON.stringify(repositoryWasmBuildPolicyV1(cwd))});
      await runOwnedCommand(request.command,[...request.args,...transported.arguments],cwd,"native:owner-command",0,{env,onProgress:env.SEMIO_NATIVE_OWNER_PROGRESS==="delegated"?()=>{}:line=>process.stderr.write(`${line}\n`)});
      return;
    }
    if (tool === "cargo" && operation === "metadata") {
      if (index !== 2 || args.length !== 4 || !manifest) throw new Error("native cargo metadata --manifest <Cargo.toml>");
      const path = resolve(this.repoRoot, manifest);
      await runRepositoryCommand("cargo", ["metadata", "--locked", "--format-version=1", "--manifest-path", path], dirname(path), "cargo:locked-metadata", buildBudgetMs(), { stdout: "ignore" });
      console.log("[cargo:locked-metadata] dependency lock validated");
      return;
    }
    if (tool === "component") {
      if ((operation !== "dev" && operation !== "release") || index !== 2 || args.length !== 4 || !manifest) throw new Error("native component dev|release --manifest <Cargo.toml>");
      const cargo = Bun.TOML.parse(readFileSync(resolve(this.repoRoot, manifest), "utf8")) as { package?: { name?: string; metadata?: { component?: { package?: string }; semio?: { "component-kind"?: string } } } };
      if (!cargo.package?.name || !cargo.package?.metadata?.component?.package || !["plugin", "extension"].includes(cargo.package?.metadata?.semio?.["component-kind"] ?? "")) throw new Error(`Not a plugin component manifest: ${manifest}`);
      const packageName=cargo.package.name;
      await buildRepositoryCargoArtifacts(
        manifest,
        pluginComponentRustcArgs(packageName, `wasm-${operation}`),
        this.repoRoot,
        {
          command: "rustc",
          output: `dist/component-${operation}`,
          validate: (files) => {
            assert.equal(files.size, 1, "Component output must contain only the linked WASM component");
            const artifact = [...files][0];
            assert.ok(artifact);
            const [name, path] = artifact;
            assert.equal(name, `${packageName.replaceAll("-", "_")}.wasm`);
            assert.deepEqual([...readFileSync(path).subarray(0, 8)], [0, 97, 115, 109, 13, 0, 1, 0], "Invalid WASI component header");
          },
        },
      );
      return;
    }
    if (tool !== "cargo" || (operation !== "build" && operation !== "check" && operation !== "test") || !manifest) throw new Error("native cargo build|check|test --manifest <Cargo.toml>");
    const extra = args.slice(index + 2);
    validateNativeCargoArguments(operation, extra);
    if (operation === "build") {
      await buildRepositoryCargoArtifacts(manifest, extra, this.repoRoot);
      return;
    }
    const status = runCmdStatus("cargo", [operation, "--locked", "--manifest-path", resolve(this.repoRoot, manifest), ...extra], { cwd: this.repoRoot });
    if (status) throw new Error(`cargo ${operation} failed (${status})`);
  }
}
