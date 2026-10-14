#!/usr/bin/env bun
import {repositoryCargoNetworkV1} from "../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🧪️ `@semio-tech/framework-os-scale-fixture` test task router — F1-scale-fixture
 * (26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME). `check` is the native `--all-targets` proof
 * (unit tests included); `check-wasm` is the real wasm32-wasip2 component build this ticket's
 * whole claim rests on. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { runCargo } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

import { buildRepositoryCargoArtifacts } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";

class CheckScript extends BundleScript {
  run(): void {
    runCargo(["check", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-scale-fixture", "--all-targets"], this.root);
  }
}

class CheckWasmScript extends BundleScript {
  run(): void {
    runCargo(["check", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-scale-fixture", "--target", "wasm32-wasip2"], this.root);
  }
}

class BuildWasmScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("Scale component build has a fixed output contract");
    await buildRepositoryCargoArtifacts(join(this.root, "Cargo.toml"), ["-p", "semio-framework-os-scale-fixture", "--lib", "--target", "wasm32-wasip2", "--profile", "wasm-release"], this.repoRoot, {network:repositoryCargoNetworkV1(this.invocation,this.repoRoot), control: scriptCompilerAdmissionControlV1(this.invocation, "cargo:build", 0), 
      command: "rustc",
      output: "dist/component",
      validate(files) {
        const artifact = files.get("semio_framework_os_scale_fixture.wasm");
        if (!artifact || !readFileSync(artifact).subarray(0, 8).equals(Buffer.from([0, 97, 115, 109, 13, 0, 1, 0]))) throw new Error("Scale output must be a WASI component");
      },
    });
  }
}

class TestScript extends BundleScript {
  run(): void {
    runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-scale-fixture", "--lib"], this.root);
  }
}

const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("check-wasm", CheckWasmScript).register("build-wasm", BuildWasmScript).register("test", TestScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "check" }) }));
