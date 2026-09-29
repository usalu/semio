#!/usr/bin/env bun
/** ❓️ `@teaching/architecture-quiz` task router: `bun ./📜️script.ts <dev|build|test [quick|long|exhaustive]|check|publish|docker-image-build [--tag <tag>] [--jobs <n>]|docker-image-check [--tag <tag>] [--port <n>] [--keep]|docker-image-publish [--tag <tag>]|docker-stack-check [--tag <tag>] [--http-port <n>] [--https-port <n>] [--keep]>`. */
import { BundleScript, ScriptRouter, playgroundDevPortString, playgroundPortEnv, resolveTestLevel, runBundleScriptMain, runViteBunxDev, runViteBuild, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { buildQuizImage, checkQuizCatalog, checkQuizImage, checkQuizStack, publishQuizImage, publishQuizSite } from "../../🚀️deploy/🟦️.ts";

const VITE_CONFIG = "../../🏗️builder/🌐️vite/🟦️.ts";

class DevScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runViteBunxDev(this.root, segments, { config: VITE_CONFIG, portEnv: playgroundPortEnv("architecture-quiz"), defaultPort: playgroundDevPortString("architecture-quiz"), fixedPort: true });
  }
}
class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runViteBuild(this.root, segments, VITE_CONFIG);
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
class PublishScript extends BundleScript {
  run(): void {
    publishQuizSite(this.root, () => runViteBuild(this.root, [], VITE_CONFIG));
  }
}
class DockerImageBuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await buildQuizImage(this.repoRoot, segments);
  }
}
class DockerImageCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await checkQuizImage(segments);
  }
}
class DockerImagePublishScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await publishQuizImage(this.repoRoot, segments);
  }
}
class DockerStackCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await checkQuizStack(this.repoRoot, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript).register("check", CheckScript).register("publish", PublishScript).register("docker-image-build", DockerImageBuildScript).register("docker-image-check", DockerImageCheckScript).register("docker-image-publish", DockerImagePublishScript).register("docker-stack-check", DockerStackCheckScript);

if (import.meta.main) await runBundleScriptMain(router, import.meta.url);
