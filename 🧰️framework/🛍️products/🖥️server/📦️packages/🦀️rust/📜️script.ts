#!/usr/bin/env bun
import {verifyCapturedCargoArtifactsV1} from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import {repositoryCargoNetworkV1} from "../../../🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
/** 🖥️ `semio-framework-server` task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`.
 *
 * `test` runs the crate's own suite, the companion suite `semio-framework-server-tests` (which
 * enables the `conformance` feature on its dependency, as every instance crate does) **and** a
 * `--features conformance` check of the library itself, so a break in the feature half stays visible
 * before hub's own build hits it. Nothing else depends on this crate, which is exactly how 28 errors
 * once survived a month here (`📓️h2-server-crate-and-wave3-memo.md` §B.7 step 14). */
import { runCargo } from "../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

import { buildRepositoryCargoArtifacts } from "../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargo(["test", ...["server", "server-tests", "server-contract", "server-storage", "server-policy", "server-authority", "server-throttle"].flatMap(name => ["-p", `semio-framework-${name}`]), ...rest], this.repoRoot);
    runCargo(["check", "-p", "semio-framework-server", "--features", "conformance"], this.repoRoot);
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await buildRepositoryCargoArtifacts(`${this.root}/Cargo.toml`, segments, this.repoRoot, {producer:"artifact",invocation:this.invocation,consume:verifyCapturedCargoArtifactsV1,network:repositoryCargoNetworkV1(this.invocation,this.repoRoot), control: scriptCompilerAdmissionControlV1(this.invocation, "cargo:build", 0) });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("build", BuildScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
