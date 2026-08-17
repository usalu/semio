#!/usr/bin/env bun
/** 🛡️ `@semio-tech/hub-admin` (nx `os-hub-admin`) router: `bun ./📜️script.ts <dev|build|test> [args…]`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runViteBuild, runViteBunxDev, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

class DevScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runViteBunxDev(this.root, ["--config", "⚙️vite.config.ts", ...segments], {
      portEnv: "OS_HUB_ADMIN_DEV_PORT",
      defaultPort: "8790",
      fixedPort: true,
    });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runViteBuild(this.root, segments, "⚙️vite.config.ts");
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runVitest(this.root, segments, "🧪️vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
