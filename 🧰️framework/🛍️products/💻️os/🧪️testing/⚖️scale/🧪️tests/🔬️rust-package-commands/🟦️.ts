import type { ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { verifyCapturedCargoArtifactsV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { buildAdmittedCargoArtifactsV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";

/** 🏃️ Runs the `BuildWasmScript` command under the caller's roots and invocation. */
export async function runBuildWasmScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments.length) throw new Error("Scale component build has a fixed output contract");
  await buildAdmittedCargoArtifactsV1(join(context.root, "Cargo.toml"), ["-p", "semio-framework-os-scale-fixture", "--lib", "--target", "wasm32-wasip2"], context.repoRoot, {producer:"artifact",invocation:context.invocation,consume:verifyCapturedCargoArtifactsV1,control: scriptCompilerAdmissionControlV1(context.invocation, "cargo:build", 0), 
    command: "rustc",
    output: "dist/component",
    validate(files) {
      const artifact = files.get("semio_framework_os_scale_fixture.wasm");
      if (!artifact || !readFileSync(artifact).subarray(0, 8).equals(Buffer.from([0, 97, 115, 109, 13, 0, 1, 0]))) throw new Error("Scale output must be a WASI component");
    },
  });
}
