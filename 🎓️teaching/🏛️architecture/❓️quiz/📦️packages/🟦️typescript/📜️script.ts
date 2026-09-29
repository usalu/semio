#!/usr/bin/env bun
/** ❓️ `@teaching/architecture-quiz` task router: `bun ./📜️script.ts <dev|build|test [quick|long|exhaustive]|check|docker-image-build [--tag <tag>] [--jobs <n>]|docker-image-check [--tag <tag>] [--port <n>] [--keep]>`. */
import { BundleScript, ScriptRouter, playgroundDevPortString, playgroundPortEnv, resolveTestLevel, runBundleScriptMain, runViteBunxDev, runViteBuild, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { buildQuizImage, checkQuizCatalog, checkQuizImage } from "../../🚀️deploy/🟦️.ts";

class DevScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runViteBunxDev(this.root, segments, { config: "../../🏗️builder/🌐️vite/🟦️.ts", portEnv: playgroundPortEnv("architecture-quiz"), defaultPort: playgroundDevPortString("architecture-quiz"), fixedPort: true });
  }
}
class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runViteBuild(this.root, segments, "../../🏗️builder/🌐️vite/🟦️.ts");
  }
}
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}
class CheckScript extends BundleScript {
  async run(): Promise<void> {
    await checkQuizCatalog(this.repoRoot);
  }
}
class DockerImageBuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await buildQuizImage(this.repoRoot, segments);
  }
}
class DockerImageCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await checkQuizImage(this.repoRoot, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript).register("check", CheckScript).register("docker-image-build", DockerImageBuildScript).register("docker-image-check", DockerImageCheckScript);

if (import.meta.main) await runBundleScriptMain(router, import.meta.url);
