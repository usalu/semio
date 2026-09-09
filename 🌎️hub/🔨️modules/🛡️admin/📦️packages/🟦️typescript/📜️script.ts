#!/usr/bin/env bun
/** 🛡️ `@semio-tech/hub-admin` (nx `os-hub-admin`) router: `bun ./📜️script.ts <dev|build|test> [args…]`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runViteBuild, runViteBunxDev, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

/** 🩺️ Warns that the standalone Vite surface has no administrator authority; authenticated use is
 * exclusively owned by the loopback relay started by `os-hub:dev-secure-admin`. */
async function warnWhenHubIsUnreachable(): Promise<void> {
  const hubUrl = process.env.OS_HUB_URL ?? "http://127.0.0.1:8787";
  try {
    const response = await fetch(`${hubUrl}/admin/api/overview`, { signal: AbortSignal.timeout(2_000) });
    if (response.ok) {
      console.log(`[admin] hub reachable at ${hubUrl}; use bun nx run os-hub:dev-secure-admin for authenticated administration`);
      return;
    }
    console.warn(`[admin] hub at ${hubUrl} answered ${response.status}; use bun nx run os-hub:dev-secure-admin for the authenticated relay.`);
  } catch {
    console.warn(`[admin] no hub reachable at ${hubUrl}; this Vite surface is static-only and has no administrator credential.`);
    console.warn(`[admin] start the protected surface with: bun nx run os-hub:dev-secure-admin`);
  }
}

class DevScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await warnWhenHubIsUnreachable();
    await runViteBunxDev(this.root, ["--config", "⚙️vite.config.ts", ...segments], {
      portEnv: "OS_HUB_ADMIN_DEV_PORT",
      defaultPort: "8790",
      fixedPort: true,
    });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.some((arg) => /^(?:--outDir|--config|--root)(?:=|$)/.test(arg))) throw new Error("Build output and configuration are owned by this Nx target");
    runViteBuild(this.root, segments, "⚙️vite.config.ts");
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    const { verifyAdminEntryGraph, verifyAdminStylesheetGraph } = await import("../../🧪️tests/🕸️build-graph/🟦️.ts");
    verifyAdminEntryGraph(this.root);
    verifyAdminStylesheetGraph(this.root);
    await runVitest(this.root, rest, "vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
