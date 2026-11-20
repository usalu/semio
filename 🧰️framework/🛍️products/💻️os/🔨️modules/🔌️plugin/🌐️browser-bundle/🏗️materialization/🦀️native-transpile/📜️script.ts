#!/usr/bin/env repo
import { chmodSync, copyFileSync, mkdirSync, renameSync } from "node:fs";
import { dirname, join } from "node:path";
import { decodeCargoProvenanceV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts";
import { verifyCapturedCargoArtifactsV1, withCapturedCargoArtifactsTaskV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import { buildAdmittedCargoArtifactsV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { buildBudgetMs } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";
import { requireHostPlatformBuildArgumentsV1 } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import { nativeTranspileBinaryNameV1 } from "./🟦️.ts";

/** 🏗️ Builds the native transpile in-process under the received script invocation and stages the executable beside the other browser tools.
 * @see {@link ./🟦️.ts} */
class NativeTranspileBuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const platform = requireHostPlatformBuildArgumentsV1(segments, process.platform);
    const name = nativeTranspileBinaryNameV1(platform);
    const destination = join(this.repoRoot, ".cache", "b", "p", "m", "browser", "release", "tools", "native-transpile", platform, name);
    await buildAdmittedCargoArtifactsV1(join(this.root, "Cargo.toml"), ["--bin", "semio-native-transpile"], this.repoRoot, {
      producer: "artifact",
      invocation: this.invocation,
      control: scriptCompilerAdmissionControlV1(this.invocation, "cargo:build", buildBudgetMs()),
      consume: capture => withCapturedCargoArtifactsTaskV1(capture, async view => {
        const observation = decodeCargoProvenanceV1(view.observation);
        if (observation.status !== 0 || observation.cancelled) throw new Error("Completed native-transpile Cargo observation required");
        const executable = view.files.get(name);
        if (!executable) throw new Error(`cargo built no ${name} executable`);
        await view.recheck();
        mkdirSync(dirname(destination), { recursive: true });
        const staging = `${destination}.${process.pid}.tmp`;
        copyFileSync(executable, staging);
        chmodSync(staging, 0o755);
        renameSync(staging, destination);
        console.log(destination);
      }),
    });
  }
}

if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dirname).register("build", NativeTranspileBuildScript)).run(process.argv.slice(2), original));
