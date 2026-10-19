
import{createIssuedResourceControlV1}from"../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { readCargoTestPolicyV1, runAdmittedCargoLintV1 as runCargoLintV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";

import { BundleScript, ScriptRouter } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runTestScript, runCanonicalArchitectureScript, runBenchScript, SEMIO_FRAMEWORK_3D_PACKAGES } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

class CanonicalArchitectureScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCanonicalArchitectureScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 📈️ Runs Cargo's benchmark mode for the `semio-framework-3d` packages; they declare no standalone benchmark target.
 * The dedicated Brep Criterion benchmark belongs to `semio-s-plugin-stdio` and its own `bench` command. */
class BenchScript extends BundleScript {
  async run(): Promise<void> {
    await runBenchScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}


/** 🧹️Zero-warning clippy gate: `cargo clippy -p <each 3d package> --all-targets -- -D warnings`. */
class LintScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
    await runCargoLintV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:SEMIO_FRAMEWORK_3D_PACKAGES,cwd:this.root,extraArgs:segments},readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
  }
}

const router = new ScriptRouter(import.meta.dir).register("canonical-architecture", CanonicalArchitectureScript).register("test", TestScript).register("bench", BenchScript).register("lint", LintScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
