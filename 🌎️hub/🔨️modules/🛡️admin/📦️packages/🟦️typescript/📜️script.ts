#!/usr/bin/env bun
/** 🛡️ `@semio-tech/hub-admin` (nx `os-hub-admin`) router: `bun ./📜️script.ts <dev|build|test> [args…]`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runViteBuild, runViteBunxDev, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

/** @emoji 🩺️ Preflight: this dev server is a pure vite proxy shell — every `/admin/api`, `/directory`
 * and `/auth` call it serves is forwarded to a separately-running hub. With no hub behind it the page
 * loads but every request dies with `ECONNREFUSED`, which reads as "admin is broken" rather than "the
 * hub is not up". Probing once and saying so plainly turns a confusing dead page into an instruction.
 * Deliberately a warning, not a hard failure: iterating on admin UI against a hub you are about to
 * start is legitimate. */
async function warnWhenHubIsUnreachable(): Promise<void> {
  const hubUrl = process.env.OS_HUB_URL ?? "http://127.0.0.1:8787";
  try {
    const response = await fetch(`${hubUrl}/admin/api/overview`, { signal: AbortSignal.timeout(2_000) });
    if (response.ok) {
      console.log(`[admin] hub reachable at ${hubUrl}`);
      return;
    }
    console.warn(`[admin] hub at ${hubUrl} answered ${response.status} for /admin/api/overview — if this is 401, set OS_HUB_ADMIN_TOKEN or connect from loopback.`);
  } catch {
    console.warn(`[admin] no hub reachable at ${hubUrl} — every API call from this page will fail with ECONNREFUSED.`);
    console.warn(`[admin] start one with:  OS_HUB_PORT=8787 bun nx run os-hub:dev`);
    console.warn(`[admin] (that hub also serves the built admin page itself at ${hubUrl}/admin — this 8790 dev server is only for iterating on the admin UI.)`);
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
