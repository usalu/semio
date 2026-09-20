#!/usr/bin/env bun
import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { serveVite } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts";
import { closeServiceSession, openServiceSession, publishServiceReady, readServiceSession } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🧾️session/🟦️.ts";
import { PLAY_RUNTIME_PANES, playRuntimeModuleLayout } from "./🟦️.ts";
import { readPlayActivation } from "./♻️activation/🟦️.ts";
import { PLAY_E2E_OWNER, playE2eInvocationPid, playE2eSessionRoot } from "./🧪️e2e/🟦️.ts";

/** @emoji 🎡️ Verifies every runtime component the outer Nx preparation graph staged for the profile. */
class PreparationScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const profile = args[0];
    if (args.length !== 1 || !["dev", "release"].includes(profile)) throw new Error("prepare <dev|release>");
    const moduleRoot = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist", profile, "🔌️plugin-modules");
    const { pluginModuleDirNames, extensionModuleDirNames } = playRuntimeModuleLayout();
    for (const name of [...pluginModuleDirNames.slice(2), ...extensionModuleDirNames]) {
      const directory = join(moduleRoot, name), marker = JSON.parse(readFileSync(join(directory, ".nx-artifact.json"), "utf8"));
      if (!Array.isArray(marker.files) || !marker.files.includes("🌉️bridge.js") || !marker.files.includes("🔣️.json") || marker.files.some((file: string) => !existsSync(join(directory, file)))) throw new Error(`Incomplete play runtime component: ${name}`);
    }
    console.log(`Prepared play ${profile}: ${PLAY_RUNTIME_PANES.length} panes, ${pluginModuleDirNames.length - 2 + extensionModuleDirNames.length} components`);
  }
}

/** @emoji 📡️ Validates the host-lane activation after its Nx prerequisites have completed. */
class ActivationScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Play activation accepts no arguments");
    const { receipt } = readPlayActivation(this.repoRoot);
    console.log(`Activated play dev: ${receipt.plugins.length} completed components for ${PLAY_RUNTIME_PANES.length} panes`);
  }
}

/** @emoji 🖥️ Serves completed runtime artifacts for the lifetime owned by Nx. */
class ServeScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Play serving accepts no arguments");
    readPlayActivation(this.repoRoot);
    const root = resolve(this.root, "../.."), controller = new AbortController();
    const interrupt = (): void => { process.exitCode = 130; controller.abort(); }, terminate = (): void => { process.exitCode = 143; controller.abort(); };
    process.once("SIGINT", interrupt); process.once("SIGTERM", terminate);
    try {
      await serveVite({ root, config: join(root, "🏗️builder/🌐️vite/🟦️.ts"), host: process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1", port: Number(process.env.SEMIO_TECH_PLAY_PORT ?? 6033), signal: controller.signal, ready: url => console.log(`Play ready: ${url}`) });
    } finally { process.removeListener("SIGINT", interrupt); process.removeListener("SIGTERM", terminate); }
  }
}

/** @emoji 🆕️ Prepares an invocation-specific E2E generation after its runtime and browser prerequisites. */
class PrepareTestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Play E2E preparation accepts no arguments");
    readPlayActivation(this.repoRoot);
    const session = await openServiceSession(playE2eSessionRoot(this.repoRoot), PLAY_E2E_OWNER, playE2eInvocationPid(process.env));
    console.log(`Prepared play E2E service ${session.id}`);
  }
}

/** @emoji 🧪️ Owns the isolated E2E listener and announces its prepared generation over HTTP; frozen (no file watching) so concurrent edits never restart it mid-suite. */
class ServeTestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Play E2E serving accepts no arguments");
    const sessionRoot = playE2eSessionRoot(this.repoRoot), session = readServiceSession(sessionRoot, PLAY_E2E_OWNER, playE2eInvocationPid(process.env));
    const root = resolve(this.root, "../.."), controller = new AbortController();
    const interrupt = (): void => { process.exitCode = 130; controller.abort(); }, terminate = (): void => { process.exitCode = 143; controller.abort(); };
    process.once("SIGINT", interrupt); process.once("SIGTERM", terminate);
    try {
      readPlayActivation(this.repoRoot);
      process.env.SEMIO_TECH_PLAY_FROZEN = "true";
      await serveVite({ root, config: join(root, "🏗️builder/🌐️vite/🟦️.ts"), host: "127.0.0.1", port: 0, signal: controller.signal, session, ready: async url => { await publishServiceReady(sessionRoot, session, url, controller.signal); console.log(`Play E2E ready: ${url}`); } });
    } finally { process.removeListener("SIGINT", interrupt); process.removeListener("SIGTERM", terminate); await closeServiceSession(sessionRoot, session); }
  }
}

const router = new ScriptRouter(import.meta.dir).register("prepare", PreparationScript).register("activate", ActivationScript).register("serve", ServeScript).register("prepare-test", PrepareTestScript).register("serve-test", ServeTestScript);
if (import.meta.main) await router.run(process.argv.slice(2));
