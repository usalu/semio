#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { readServiceSession, waitForServiceReady } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🧾️session/🟦️.ts";
import { repoCacheDirectory } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";
import { PLAY_E2E_OWNER, playE2eInvocationPid, playE2eSessionRoot } from "../🧩️runtime/🧪️e2e/🟦️.ts";
import { servePublishedPlay } from "./📦️release/🟦️.ts";

/** 🎭️ Runs acceptance tests only against the server generation prepared and owned by Nx. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Play E2E accepts no compiler or server arguments");
    const root = resolve(this.root, "../.."), sessionRoot = playE2eSessionRoot(this.repoRoot), controller = new AbortController();
    const session = readServiceSession(sessionRoot, PLAY_E2E_OWNER, playE2eInvocationPid(process.env));
    const interrupt = (): void => { process.exitCode = 130; controller.abort(); }, terminate = (): void => { process.exitCode = 143; controller.abort(); };
    process.once("SIGINT", interrupt); process.once("SIGTERM", terminate);
    try {
      console.log(`Waiting for play E2E service ${session.id}`);
      const baseURL = await waitForServiceReady(sessionRoot, session, controller.signal);
      const output = process.env.SEMIO_TICKET_DIR ? join(process.env.SEMIO_TICKET_DIR, "🗑️generated/play-e2e", session.id) : join(root, "dist/reports/e2e", session.id);
      console.log(`Testing play at ${baseURL}`);
      const child = Bun.spawn([process.execPath, join(this.repoRoot, "node_modules/playwright/cli.js"), "test", "--config", join(root, "🔨️modules/🧪️e2e/🎚️config/🟦️.ts"), "--output", output], { cwd: this.repoRoot, env: { ...process.env, PLAYWRIGHT_BASE_URL: baseURL, PLAYWRIGHT_BROWSERS_PATH: repoCacheDirectory(this.repoRoot, "tools", "ms-playwright") }, stdout: "inherit", stderr: "inherit", stdin: "ignore" });
      const cancel = (): void => { child.kill("SIGTERM"); };
      controller.signal.addEventListener("abort", cancel, { once: true });
      if (controller.signal.aborted) cancel();
      let status: number;
      try { status = await child.exited; }
      finally { controller.signal.removeEventListener("abort", cancel); }
      if (status !== 0 && !controller.signal.aborted) throw new Error(`Play acceptance tests failed (${status})`);
    } finally { process.removeListener("SIGINT", interrupt); process.removeListener("SIGTERM", terminate); }
  }
}

/** 🚀 Verifies the actual release pages with fresh browser contexts and no development server. */
class TestReleaseScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const root = resolve(this.root, "../.."), service = servePublishedPlay(join(root, "dist/pages"));
    const controller = new AbortController();
    const interrupt = (): void => { process.exitCode = 130; controller.abort(); }, terminate = (): void => { process.exitCode = 143; controller.abort(); };
    process.once("SIGINT", interrupt); process.once("SIGTERM", terminate);
    try {
      const output = process.env.SEMIO_TICKET_DIR ? join(process.env.SEMIO_TICKET_DIR, "🗑️generated/play-release-e2e") : join(root, "dist/reports/release-e2e");
      console.log(`[DEBUG] Verifying published Play at ${service.baseURL}: ${JSON.stringify(service.origins)}`);
      const child = Bun.spawn([process.execPath, join(this.repoRoot, "node_modules/playwright/cli.js"), "test", "--config", join(root, "🔨️modules/🧪️e2e/🎚️config/🟦️.ts"), "--output", output, ...args], { cwd: this.repoRoot, env: { ...process.env, PLAYWRIGHT_BASE_URL: service.baseURL, PLAY_E2E_RELEASE_ORIGINS: JSON.stringify(service.origins), PLAYWRIGHT_BROWSERS_PATH: repoCacheDirectory(this.repoRoot, "tools", "ms-playwright") }, stdout: "inherit", stderr: "inherit", stdin: "ignore" });
      const cancel = (): void => { child.kill("SIGTERM"); };
      controller.signal.addEventListener("abort", cancel, { once: true });
      if (controller.signal.aborted) cancel();
      let status: number;
      try { status = await child.exited; } finally { controller.signal.removeEventListener("abort", cancel); }
      if (status !== 0 && !controller.signal.aborted) throw new Error(`Published Play acceptance tests failed (${status})`);
    } finally { process.removeListener("SIGINT", interrupt); process.removeListener("SIGTERM", terminate); await service.stop(); }
  }
}

/** 🔬 Runs the language-neutral release path and origin routing contracts. */
class TestReleaseRoutingScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Release routing tests accept no arguments");
    const child = Bun.spawn([process.execPath, "test", join(resolve(this.root, "../.."), "🧪️tests/🧪️playrelease/🟦️.ts")], { cwd: this.repoRoot, stdout: "inherit", stderr: "inherit" });
    if (await child.exited !== 0) throw new Error("Play release routing contracts failed");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-release", TestReleaseScript).register("test-release-routing", TestReleaseRoutingScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (router).run(process.argv.slice(2), original));
