#!/usr/bin/env bun
import {scriptCompilerAdmissionControlV1} from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { typecheckRepositoryProjectV1 } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🟦️typescript/🟦️.ts";
import { createIssuedResourceControlV1 } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
/** ❓️ `@teaching/architecture-course` task router: `bun ./📜️script.ts <dev|dev-site|build|test [quick|long|exhaustive]|test-e2e [--topology <both|dev|rehearsal>] [--serial] [--keep] [--proctor <executable>] [-- <Playwright arguments…>]|typecheck|check|publish|docker-image-build [--tag <tag>]|docker-image-check [--tag <tag>] [--port <n>] [--keep]|docker-image-publish [--tag <tag>]|docker-stack-check [--tag <tag>] [--http-port <n>] [--https-port <n>] [--keep]|docker-stack-bundle [--tag <tag>] [--out <directory>]|deploy-check [--tag <tag>]>`. */
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { playgroundDevPortString, playgroundPortEnv, runViteBunxDev, runViteBuild } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/🟦️.ts";
import { runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🧪️vitest/🟦️.ts";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { checkCourse } from "../../🚀️deploy/🟦️.ts";
import { buildCourseReleaseSite, publishCourseSite } from "../../🚀️deploy/🌐️site/🟦️.ts";
import { buildCourseImage, publishCourseImage } from "../../🚀️deploy/🐳️docker-image/🟦️.ts";
import { checkCourseImage } from "../../🚀️deploy/🩺️docker-image-check/🟦️.ts";
import { checkCourseStack } from "../../🚀️deploy/🧱️docker-stack/🟦️.ts";
import { stageCourseStackBundle } from "../../🚀️deploy/🎁️stack-bundle/🟦️.ts";
import { checkCourseDeployment } from "../../🚀️deploy/⚖️readiness/🟦️.ts";
import { runCourseEndToEnd } from "../../🎭️e2e/🟦️.ts";
import { runDevStack } from "../../🧱️stack/🟦️.ts";
import { runBuildPreflightV1 } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️preflight/🟦️.ts";
import { runOwnedCommand } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { scriptInvocationBudget } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const VITE_CONFIG = "../../🏗️builder/🌐️vite/🟦️.ts";

/** 🩺️ Runs generic host preflight through the course dashboard owner; image production validates its native runtime contract. */
class PreflightScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if(args.length)throw new Error("Course preflight accepts no arguments");
    await runBuildPreflightV1(this.repoRoot,["preflight"],this.invocation);
  }
}
/** 🧰️ Installs Chromium's Debian system packages under the original CI process control. */
class CiBrowserPrepareScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if(args.length)throw new Error("Course CI browser preparation accepts no arguments");
    await runOwnedCommand("node",[join(this.repoRoot,"node_modules/playwright/cli.js"),"install-deps","chromium"],this.repoRoot,"architecture-course:ci-browser-prepare",scriptInvocationBudget(this.invocation,600_000),{signal:this.invocation.control.signal});
  }
}

/** 🛠️ `dev` — the whole local stack in one terminal: the proctor, then the site (`dev-site`), stopped together. */
class DevScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runDevStack(this.repoRoot, this.root, fileURLToPath(import.meta.url), { portEnv: playgroundPortEnv("architecture-course"), defaultPort: playgroundDevPortString("architecture-course") }, this.invocation, segments);
  }
}
/** 🌐️ `dev-site` — the site's dev server alone; it proxies the gateway routes to whatever proctor answers on
 * `PROCTOR_PORT`. */
class DevSiteScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runViteBunxDev(this.invocation, scriptCompilerAdmissionControlV1(this.invocation,"vite:entry",0), this.root, segments, { runtimeChannels: [], config: VITE_CONFIG, portEnv: playgroundPortEnv("architecture-course"), defaultPort: playgroundDevPortString("architecture-course"), fixedPort: true });
  }
}
class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runViteBuild(this.invocation, scriptCompilerAdmissionControlV1(this.invocation,"vite:entry",0), this.root, segments, VITE_CONFIG);
  }
}
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments, "quick");
    await runVitest(this.root, ["--configLoader", "native", ...rest], "../../🧪️tests/🎚️config/🟦️.ts");
  }
}
/** 🎭️ `test-e2e [--topology <both|dev|rehearsal>] [--serial] [--keep] [--proctor <executable>] [-- <Playwright arguments…>]` — the end-to-end
 * gate: throw-away stacks (the dev stack and the release rehearsal; both unless one is named) driven through a real
 * browser. */
class TestEndToEndScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCourseEndToEnd(this.invocation, this.repoRoot, this.root, fileURLToPath(import.meta.url), playgroundPortEnv("architecture-course"), segments);
  }
}
/** 🪁️ `typecheck` — the site's entry, Vite configuration, local stack, deploy verbs, end-to-end gate, every test and
 * spec, and this router against the compiler (`tsconfig.json`). */
class TypecheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await typecheckRepositoryProjectV1({ workspace: this.repoRoot, invocation: this.invocation, control: createIssuedResourceControlV1(this.invocation, "typecheck:entry", 0), cwd: this.root, project: "tsconfig.json", owner: "architecture-course:typescript", args: segments, timeoutMs: 120_000 });
  }
}
class CheckScript extends BundleScript {
  async run(): Promise<void> {
    await checkCourse(this.repoRoot, this.invocation);
  }
}
class PublishScript extends BundleScript {
  async run(): Promise<void> {
    await publishCourseSite(this.repoRoot, this.root, this.invocation, () => buildCourseReleaseSite(this.repoRoot, this.root, this.invocation));
  }
}
class DockerImageBuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await buildCourseImage(this.repoRoot, segments, this.invocation);
  }
}
class DockerImageCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await checkCourseImage(this.repoRoot, segments, this.invocation);
  }
}
class DockerImagePublishScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await publishCourseImage(this.repoRoot, segments, this.invocation);
  }
}
class DockerStackCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await checkCourseStack(segments, this.invocation);
  }
}
/** 🚛️ `docker-stack-bundle [--tag <tag>] [--out <directory>]` — everything the proctor host needs without a registry,
 * staged in one directory (default `dist/proctor`): the stack files, `certificates/`, `.env`, the image and the steps. */
class DockerStackBundleScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await stageCourseStackBundle(this.repoRoot, this.root, segments, this.invocation);
  }
}
/** ⚖️ `deploy-check` — the readiness gate. The two end-to-end topologies run one after the other, so they never share the host's sockets. */
class DeployCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await checkCourseDeployment(this.repoRoot, this.root, {
      build: () => buildCourseReleaseSite(this.repoRoot, this.root, this.invocation),
      typecheck: () => typecheckRepositoryProjectV1({ workspace: this.repoRoot, invocation: this.invocation, control: createIssuedResourceControlV1(this.invocation, "typecheck:course-deployment", 0), cwd: this.root, project: "tsconfig.json", owner: "architecture-course:typescript", args: [], timeoutMs: 120_000 }),
      endToEnd: () => runCourseEndToEnd(this.invocation, this.repoRoot, this.root, fileURLToPath(import.meta.url), playgroundPortEnv("architecture-course"), ["--serial", "--"]),
    }, segments, this.invocation);
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("dev-site", DevSiteScript).register("build", BuildScript).register("test", TestScript).register("test-e2e", TestEndToEndScript).register("typecheck", TypecheckScript).register("check", CheckScript).register("publish", PublishScript).register("docker-image-build", DockerImageBuildScript).register("docker-image-check", DockerImageCheckScript).register("docker-image-publish", DockerImagePublishScript).register("docker-stack-check", DockerStackCheckScript).register("docker-stack-bundle", DockerStackBundleScript).register("deploy-check", DeployCheckScript).register("preflight", PreflightScript).register("ci-browser-prepare", CiBrowserPrepareScript);

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
