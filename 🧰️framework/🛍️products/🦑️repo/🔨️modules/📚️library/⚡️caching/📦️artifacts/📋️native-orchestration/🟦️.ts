import {consumeOwnerArgumentsV1} from "../../../🔌️nx-plugin/📤️arguments/🟨️.mjs";
import {nativeOwnerTestManifestRequestV1} from "./🗺️owner-test-manifests/🟦️.ts";
import { buildBudgetMs } from "../../../../../../../🔨️modules/🏃️process/⏱️budget/🟦️.ts";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { BundleScript, scriptInvocationBudget } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { advanceScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🟦️.ts";
import { readNativeOwnerCapabilities } from "./📥️invocation/🟦️.ts";
import { CARGO_RELAY_BUDGET_ENV, cargoStreamingStatus, runCmdStatus } from "../../../🏃️process/🟦️.ts";
import { repositoryProcessOwnerContextV1, repositoryVitestPolicyV1, repositoryCargoTestPolicyV1, repositoryWasmBuildPolicyV1 } from "../../../🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import { runRepositoryCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { repositoryCargoArtifactBuildPolicyV1, buildRepositoryCargoArtifacts } from "../🏗️native-build/🟦️.ts";
import { artifactRustCargoArguments } from "../🎛️native-input/🟦️.ts";

/** 🔐️ The owner policies a native or script owner's environment carries, exactly as the Nx owner commands author them: `process` (Vitest policy and process
 * owner context), `cargo` (the repository Cargo artifact policy) and `wasm` (the repository wasm-bindgen compiler policy). `buildDirectory` redirects both
 * compiler policies into a release generation's build directory. */
export function ownerEnvironmentV1(cwd: string, policies: readonly ("process" | "cargo" | "wasm")[], buildDirectory?: string): Record<string, string> {
  const environment: Record<string, string> = {};
  if (policies.includes("process")) Object.assign(environment, { SEMIO_VITEST_POLICY: JSON.stringify(repositoryVitestPolicyV1(cwd)), SEMIO_PROCESS_OWNER_CONTEXT: JSON.stringify(repositoryProcessOwnerContextV1(cwd)) });
  if (policies.includes("cargo")) environment.SEMIO_CARGO_ARTIFACT_POLICY = JSON.stringify({ ...repositoryCargoArtifactBuildPolicyV1(cwd), ...(buildDirectory ? { buildDirectory } : {}) });
  if (policies.includes("wasm")) environment.SEMIO_WASM_BUILD_POLICY = JSON.stringify({ ...repositoryWasmBuildPolicyV1(cwd), ...(buildDirectory ? { buildDirectory } : {}) });
  return environment;
}

/** 🌊️ `bun ⚡️caching/🦀️cargo/📜️script.ts relay <cargo args…>` — the relay [[runCmd]] routes every POSIX `cargo` through
 * ([[cargoStreamingStatus]]): exits with cargo's status, 1 when cargo was stopped (budget, signal), after its forwarded output drained. */
export class CargoRelayScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const budgetMs = Number(process.env[CARGO_RELAY_BUDGET_ENV] ?? 0);
    try {
      process.exitCode = await cargoStreamingStatus(args, process.cwd(), process.env, scriptInvocationBudget(this.invocation, budgetMs));
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
      const original=readNativeOwnerCapabilities(this.invocation.capabilities),request=nativeOwnerTestManifestRequestV1(args.slice(1)),path=resolve(this.repoRoot,request.manifest),cwd=resolve(this.repoRoot,request.cwd),transported=consumeOwnerArgumentsV1(process.env),arguments_=[...request.args,...transported.arguments];
      if(original.repositoryRoot!==this.repoRoot||resolve(this.repoRoot,original.command.manifest)!==path||resolve(this.repoRoot,original.command.workingDirectory)!==cwd||original.command.program!==request.command||original.command.arguments.length!==arguments_.length||original.command.arguments.some((argument,index)=>argument!==arguments_[index])||typeof transported.environment.CARGO_TARGET_DIR!=="string"||resolve(this.repoRoot,original.artifactDirectory)!==resolve(this.repoRoot,transported.environment.CARGO_TARGET_DIR))throw Error("Original native caller command refused");
      await advanceScriptInvocation(this.invocation,"native:owner-command","running");
      const cargo = Bun.TOML.parse(readFileSync(path, "utf8")) as { package?: { name?: string }; workspace?: object };
      if (!cargo.package?.name && !cargo.workspace) throw Error(`Native owner requires a package or workspace manifest: ${manifest}`);
      const env: Record<string,string|undefined>={...transported.environment,...ownerEnvironmentV1(cwd,["process","cargo"])};
      delete env.SEMIO_CARGO_TEST_POLICY;
      if (cargo.package?.name) Object.assign(env, { SEMIO_CARGO_TEST_POLICY: JSON.stringify(repositoryCargoTestPolicyV1(path,cwd)) });
      const policies=request.testManifests.map(manifest=>{
        const selected=resolve(this.repoRoot,manifest);
        return repositoryCargoTestPolicyV1(selected,cwd);
      });
      if(new Set(policies.map(policy=>policy.manifestPath)).size!==policies.length)throw Error("Duplicate native test manifest authority");
      env.SEMIO_CARGO_TEST_POLICIES=JSON.stringify(policies);
      if (process.env.SEMIO_WASM_BUILD_REQUIRED === "1") Object.assign(env,ownerEnvironmentV1(cwd,["wasm"]));
      const remaining=this.invocation.control.remainingMilliseconds(),budget=remaining===null?original.child.maximumElapsedMilliseconds:Math.min(original.child.maximumElapsedMilliseconds,remaining);
      if(!Number.isFinite(budget)||budget<=0)throw Error("Original native child deadline exhausted");
      await runOwnedCommand(request.command,arguments_,cwd,"native:owner-command",budget,{env,output:original.transport,signal:this.invocation.control.signal,onProgress:async line=>{await advanceScriptInvocation(this.invocation,"native:owner-command","running");if(env.SEMIO_NATIVE_OWNER_PROGRESS!=="delegated")process.stderr.write(`${line}\n`);}});
      return;
    }
    if (tool === "cargo" && operation === "metadata") {
      if (index !== 2 || args.length !== 4 || !manifest) throw new Error("native cargo metadata --manifest <Cargo.toml>");
      const path = resolve(this.repoRoot, manifest);
      await runRepositoryCommand(this.invocation,"cargo",["metadata", "--locked", "--format-version=1", "--manifest-path", path],dirname(path),"cargo:locked-metadata",{version:1,maximumElapsedMilliseconds:buildBudgetMs(),output:null},{stdout: "ignore"});
      console.log("[cargo:locked-metadata] dependency lock validated");
      return;
    }
    if (tool === "component") {
      if (index !== 1 || args.length !== 3 || !manifest) throw new Error("native component --manifest <Cargo.toml>");
      const metadata = (Bun.TOML.parse(readFileSync(resolve(this.repoRoot, manifest), "utf8")) as { package?: { metadata?: { component?: { package?: string } } } }).package?.metadata?.component?.package;
      if (!metadata?.startsWith("semio:")) throw new Error(`Not a plugin component manifest: ${manifest}`);
      const { buildComponentClosureV1 } = await import("../../../../../../💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts");
      const outcome = await buildComponentClosureV1(this.repoRoot, new Set([metadata.slice("semio:".length)]), { signal: this.invocation.control.signal });
      console.log(`[component] ${metadata}: built, described and materialized; report ${outcome.report}`);
      return;
    }
    if (tool !== "cargo" || (operation !== "build" && operation !== "check" && operation !== "test") || !manifest) throw new Error("native cargo build|check|test --manifest <Cargo.toml> [quick|long|exhaustive] [cargo argument…]");
    const { cargoArgs: extra, testLevel } = artifactRustCargoArguments(operation, args.slice(index + 2));
    if (operation === "build") {
      await buildRepositoryCargoArtifacts(manifest, extra, this.repoRoot);
      return;
    }
    const status = runCmdStatus("cargo", [operation, "--locked", "--manifest-path", resolve(this.repoRoot, manifest), ...extra], { cwd: this.repoRoot, ...(testLevel ? { env: { ...process.env, SEMIO_TEST_LEVEL: testLevel } } : {}) });
    if (status) throw new Error(`cargo ${operation} failed (${status})`);
  }
}
