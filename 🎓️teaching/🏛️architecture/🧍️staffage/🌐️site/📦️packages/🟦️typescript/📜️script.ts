#!/usr/bin/env repo
import { scriptCompilerAdmissionControlV1 } from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { typecheckRepositoryProjectV1 } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🏗️build/🟦️typescript/🟦️.ts";
import { createIssuedResourceControlV1 } from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain, scriptInvocationBudget } from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { buildViteArtifact, playgroundDevPortString, playgroundPortEnv, runViteBunxDev } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/🟦️.ts";
import { runVitest } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️testing/🧪️vitest/🟦️.ts";
import { resolve } from "node:path";
/** 🧍️ `@teaching/staffage-site` task router: `repo ./📜️script.ts <dev|dev-site|build|test [quick|long|exhaustive]|typecheck>`. */

const VITE_CONFIG = "../../🏗️builder/🌐️vite/🟦️.ts";
const HOST_KIND = "architecture-staffage";
const SITE_OWNER = "architecture-staffage:site";

/** 🌐️ `dev` and `dev-site` — the site's dev server on its fixed port; the archive needs no backend, so both are one. */
class DevSiteScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runViteBunxDev(this.invocation, scriptCompilerAdmissionControlV1(this.invocation, "vite:entry", 0), this.root, segments, { runtimeChannels: [], config: VITE_CONFIG, portEnv: playgroundPortEnv(HOST_KIND), defaultPort: playgroundDevPortString(HOST_KIND), fixedPort: true });
  }
}

/** 🔨️ `build` — bundles the site into the products store under the mode of the original caller. */
class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("Staffage site build accepts no arguments");
    await buildViteArtifact({ workspace: this.repoRoot, root: this.root, config: resolve(this.root, VITE_CONFIG), owner: SITE_OWNER, invocation: this.invocation, control: scriptCompilerAdmissionControlV1(this.invocation, "vite:staffage-site", 0), signal: this.invocation.control.signal, timeoutMs: scriptInvocationBudget(this.invocation, buildBudgetMs(process.env)) });
  }
}

/** 🧪️ `test` — the site's own cases. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments, "quick");
    await runVitest(this.root, ["--configLoader", "native", ...rest], "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

/** 🪁️ `typecheck` — the site's entry, Vite configuration and this router against the compiler. */
class TypecheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await typecheckRepositoryProjectV1({ workspace: this.repoRoot, invocation: this.invocation, control: createIssuedResourceControlV1(this.invocation, "typecheck:entry", 0), cwd: this.root, project: "tsconfig.json", owner: "staffage-site:typescript", args: segments, timeoutMs: scriptInvocationBudget(this.invocation, buildBudgetMs(process.env)) });
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevSiteScript).register("dev-site", DevSiteScript).register("build", BuildScript).register("test", TestScript).register("typecheck", TypecheckScript);

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
