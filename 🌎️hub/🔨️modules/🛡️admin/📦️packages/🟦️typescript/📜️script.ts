#!/usr/bin/env bun
import {scriptCompilerAdmissionControlV1} from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
/** 🛡️ `@semio-tech/hub-admin` (nx `os-hub-admin`) router: `bun ./📜️script.ts <dev|build|test> [args…]`. */
import { runViteBuild } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🏗️build/🟦️.ts";
import { runViteBunxDev } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🌐️service/🟦️.ts";
import { runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🧪️vitest/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🩺️ Warns that the standalone Vite surface has no administrator authority; authenticated use is
 * exclusively owned by the loopback relay started by `os-hub:dev-secure-admin`. */
async function warnWhenHubIsUnreachable(): Promise<void> {
  const hubUrl = process.env.OS_HUB_URL ?? "http://127.0.0.1:8787";
  try {
    const response = await fetch(`${hubUrl}/admin/api/overview`, { signal: AbortSignal.timeout(2_000) });
    if (response.ok) {
      console.log(`[admin] hub reachable at ${hubUrl}; use bun repo run os-hub:dev-secure-admin for authenticated administration`);
      return;
    }
    console.warn(`[admin] hub at ${hubUrl} answered ${response.status}; use bun repo run os-hub:dev-secure-admin for the authenticated relay.`);
  } catch {
    console.warn(`[admin] no hub reachable at ${hubUrl}; this Vite surface is static-only and has no administrator credential.`);
    console.warn(`[admin] start the protected surface with: bun repo run os-hub:dev-secure-admin`);
  }
}

class DevScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await warnWhenHubIsUnreachable();
    await runViteBunxDev(this.invocation, scriptCompilerAdmissionControlV1(this.invocation,"vite:entry",0), this.root, segments, { runtimeChannels:[],
      config: "../../🏗️builder/🌐️vite/🟦️.ts",
      portEnv: "OS_HUB_ADMIN_DEV_PORT",
      defaultPort: "8790",
      fixedPort: true,
    });
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some((arg) => /^(?:--outDir|--config|--root)(?:=|$)/.test(arg))) throw new Error("Build output and configuration are owned by this build target");
    await runViteBuild(this.invocation, scriptCompilerAdmissionControlV1(this.invocation,"vite:entry",0), this.root, segments, "../../🏗️builder/🌐️vite/🟦️.ts");
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    const { verifyAdminEntryGraph, verifyAdminStylesheetGraph } = await import("../../🧪️tests/🕸️build-graph/🟦️.ts");
    verifyAdminEntryGraph(this.root);
    verifyAdminStylesheetGraph(this.root);
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "build" }) }));
