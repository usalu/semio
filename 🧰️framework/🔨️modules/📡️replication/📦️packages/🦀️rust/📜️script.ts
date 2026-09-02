#!/usr/bin/env bun
/** 🖥️ `semio-framework-replication` task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargo } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargo(["test", "-p", "semio-framework-replication", ...rest], this.repoRoot);
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["build", "-p", "semio-framework-replication", ...segments], this.repoRoot);
  }
}

class SourceTestScript extends BundleScript {
  async run(): Promise<void> {
    await import("../../../🌱️value/🗂️ordered/🧪️fixtures/📜️script.ts");
  }
}

class LocalInteractionSourceTestScript extends BundleScript {
  async run(): Promise<void> {
    await import("../../📡️wire/🏠️local-interaction/🧪️fixtures/📜️script.ts");
  }
}

class LocalInteractionNativeTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargo(["test", "-p", "semio-framework-replication", "--lib", "local_interaction_", ...rest], this.repoRoot);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("build", BuildScript).register("test-source", SourceTestScript).register("test-local-interaction-source", LocalInteractionSourceTestScript).register("test-local-interaction-native", LocalInteractionNativeTestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
