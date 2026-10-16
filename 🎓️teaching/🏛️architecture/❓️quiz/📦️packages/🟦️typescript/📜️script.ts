#!/usr/bin/env bun
import {scriptCompilerAdmissionControlV1} from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { typecheckProject } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🟦️typescript/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
/** ❓️ `@teaching/architecture-quiz` task router: `bun ./📜️script.ts <dev|dev-site|build|test [quick|long|exhaustive]|test-e2e [dev] [rehearsal] [--serial] [--keep] [--proctor <executable>] [Playwright arguments…]|typecheck|check|publish|docker-image-build [--tag <tag>] [--jobs <n>]|docker-image-check [--tag <tag>] [--port <n>] [--keep]|docker-image-publish [--tag <tag>]|docker-stack-check [--tag <tag>] [--http-port <n>] [--https-port <n>] [--keep]|docker-stack-bundle [--tag <tag>] [--out <directory>]|deploy-check [--tag <tag>] [--jobs <n>]>`. */
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { playgroundDevPortString, playgroundPortEnv } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🟦️.ts";
import { runCmd } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts";
import { runViteBunxDev, runViteBuild, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { buildQuizImage, checkQuizCatalog, checkQuizDeployment, checkQuizImage, checkQuizStack, publishQuizImage, publishQuizSite, stageQuizStackBundle } from "../../🚀️deploy/🟦️.ts";
import { runQuizEndToEnd } from "../../🎭️e2e/🟦️.ts";
import { runDevStack } from "../../🧱️stack/🟦️.ts";

const VITE_CONFIG = "../../🏗️builder/🌐️vite/🟦️.ts";

/** 🛠️ `dev` — the whole local stack in one terminal: the proctor, then the site (`dev-site`), stopped together. */
class DevScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runDevStack(this.repoRoot, this.root, fileURLToPath(import.meta.url), { portEnv: playgroundPortEnv("architecture-quiz"), defaultPort: playgroundDevPortString("architecture-quiz") }, segments);
  }
}
/** 🌐️ `dev-site` — the site's dev server alone; it proxies the gateway routes to whatever proctor answers on
 * `PROCTOR_PORT`. */
class DevSiteScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runViteBunxDev(this.invocation, scriptCompilerAdmissionControlV1(this.invocation,"vite:entry",0), this.root, segments, { config: VITE_CONFIG, portEnv: playgroundPortEnv("architecture-quiz"), defaultPort: playgroundDevPortString("architecture-quiz"), fixedPort: true });
  }
}
class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runViteBuild(this.invocation, scriptCompilerAdmissionControlV1(this.invocation,"vite:entry",0), this.root, segments, VITE_CONFIG);
  }
}
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}
/** 🎭️ `test-e2e [dev] [rehearsal] [--serial] [--keep] [--proctor <executable>] [Playwright arguments…]` — the end-to-end
 * gate: throw-away stacks (the dev stack and the release rehearsal; both unless one is named) driven through a real
 * browser. */
class TestEndToEndScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runQuizEndToEnd(this.repoRoot, this.root, fileURLToPath(import.meta.url), playgroundPortEnv("architecture-quiz"), segments);
  }
}
/** 🪁️ `typecheck` — the site's entry, Vite configuration, local stack, deploy verbs, end-to-end gate, every test and
 * spec, and this router against the compiler (`tsconfig.json`). */
class TypecheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await typecheckProject({ workspace: this.repoRoot, stateRoot: join(this.repoRoot, ".🧬semio/🦑️repo/⚡️cache"), invocation: this.invocation, cwd: this.root, project: "tsconfig.json", owner: "architecture-quiz:typescript", args: segments, timeoutMs: 120_000, execution: { signal: this.invocation.control.signal } });
  }
}
class CheckScript extends BundleScript {
  async run(): Promise<void> {
    await checkQuizCatalog(this.repoRoot);
  }
}
class PublishScript extends BundleScript {
  async run(): Promise<void> {
    await publishQuizSite(this.root, () => runViteBuild(this.invocation, scriptCompilerAdmissionControlV1(this.invocation,"vite:entry",0), this.root, [], VITE_CONFIG));
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
class DockerImagePublishScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await publishQuizImage(this.repoRoot, segments);
  }
}
class DockerStackCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await checkQuizStack(segments);
  }
}
/** 🚛️ `docker-stack-bundle [--tag <tag>] [--out <directory>]` — everything the proctor host needs without a registry,
 * staged in one directory (default `dist/proctor`): the stack files, `certificates/`, `.env`, the image and the steps. */
class DockerStackBundleScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await stageQuizStackBundle(this.repoRoot, this.root, segments);
  }
}
class DeployCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await checkQuizDeployment(this.repoRoot, this.root, () => runViteBuild(this.invocation, scriptCompilerAdmissionControlV1(this.invocation,"vite:entry",0), this.root, [], VITE_CONFIG), segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("dev-site", DevSiteScript).register("build", BuildScript).register("test", TestScript).register("test-e2e", TestEndToEndScript).register("typecheck", TypecheckScript).register("check", CheckScript).register("publish", PublishScript).register("docker-image-build", DockerImageBuildScript).register("docker-image-check", DockerImageCheckScript).register("docker-image-publish", DockerImagePublishScript).register("docker-stack-check", DockerStackCheckScript).register("docker-stack-bundle", DockerStackBundleScript).register("deploy-check", DeployCheckScript);

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
