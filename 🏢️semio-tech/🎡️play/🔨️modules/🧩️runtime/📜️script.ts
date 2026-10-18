#!/usr/bin/env bun
import {scriptCompilerAdmissionControlV1} from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {frameworkOsViteArtifactInputsV1} from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🟦️.ts";
import {repositoryCachePartitionRootV1} from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import {readBuildMode} from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { serveVite } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts";
import { closeServiceSession, openServiceSession, publishServiceReady, readServiceSession } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🧾️session/🟦️.ts";
import { PLAY_RUNTIME_PANES, playRuntimeModuleLayout } from "./🟦️.ts";
import { readPlayActivation } from "./♻️activation/🟦️.ts";
import { PLAY_E2E_OWNER, playE2eInvocationPid, playE2eSessionRoot } from "./🧪️e2e/🟦️.ts";
import { builtPluginModulesRootIn } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";

/** 🎡️ Verifies every runtime component the outer preparation graph staged into the one plugin-modules root. */
class PreparationScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("prepare accepts no arguments");
    const moduleRoot = builtPluginModulesRootIn(this.repoRoot,readBuildMode(process.env.SEMIO_BUILD_MODE));
    const { pluginModuleDirNames, extensionModuleDirNames } = playRuntimeModuleLayout();
    for (const name of [...pluginModuleDirNames.slice(2), ...extensionModuleDirNames]) {
      const directory = join(moduleRoot, name), marker = JSON.parse(readFileSync(join(directory, ".semio-artifact.json"), "utf8"));
      if (!Array.isArray(marker.files) || !marker.files.includes("🌉️bridge.js") || !marker.files.includes("🔣️.json") || marker.files.some((file: string) => !existsSync(join(directory, file)))) throw new Error(`Incomplete play runtime component: ${name}`);
    }
    console.log(`Prepared play: ${PLAY_RUNTIME_PANES.length} panes, ${pluginModuleDirNames.length - 2 + extensionModuleDirNames.length} components`);
  }
}

/** 📡️ Validates the host-lane activation after its prerequisites have completed. */
class ActivationScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Play activation accepts no arguments");
    const { receipt } = readPlayActivation(this.repoRoot);
    console.log(`Activated play dev: ${receipt.plugins.length} completed components for ${PLAY_RUNTIME_PANES.length} panes`);
  }
}

/** 🖥️ Serves completed runtime artifacts for the lifetime owned by the build tool. */
class ServeScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Play serving accepts no arguments");
    readPlayActivation(this.repoRoot);
    const root = resolve(this.root, "../.."), controller = new AbortController();
    const interrupt = (): void => { process.exitCode = 130; controller.abort(); }, terminate = (): void => { process.exitCode = 143; controller.abort(); };
    process.once("SIGINT", interrupt); process.once("SIGTERM", terminate);
    try {
      await serveVite({control:scriptCompilerAdmissionControlV1(this.invocation,"vite:entry",0), invocation: this.invocation, mode: readBuildMode(process.env.SEMIO_BUILD_MODE), artifactInputs: frameworkOsViteArtifactInputsV1(repositoryCachePartitionRootV1(this.repoRoot, "products"), {version:1,mode:readBuildMode(process.env.SEMIO_BUILD_MODE),renderer:"react",command:"serve",engines:"play"}), root, config: join(root, "🏗️builder/🌐️vite/🟦️.ts"), host: process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1", port: Number(process.env.SEMIO_TECH_PLAY_PORT ?? 6033), signal: this.invocation.control.signal, ready: url => console.log(`Play ready: ${url}`) });
    } finally { process.removeListener("SIGINT", interrupt); process.removeListener("SIGTERM", terminate); }
  }
}

/** 🆕️ Prepares an invocation-specific E2E generation after its runtime and browser prerequisites. */
class PrepareTestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Play E2E preparation accepts no arguments");
    readPlayActivation(this.repoRoot);
    const session = await openServiceSession(playE2eSessionRoot(this.repoRoot), PLAY_E2E_OWNER, playE2eInvocationPid(process.env));
    console.log(`Prepared play E2E service ${session.id}`);
  }
}

/** 🧪️ Owns the isolated E2E listener and announces its prepared generation over HTTP; frozen (no file watching) so concurrent edits never restart it mid-suite. */
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
      await serveVite({control:scriptCompilerAdmissionControlV1(this.invocation,"vite:entry",0), invocation: this.invocation, mode: readBuildMode(process.env.SEMIO_BUILD_MODE), artifactInputs: frameworkOsViteArtifactInputsV1(repositoryCachePartitionRootV1(this.repoRoot, "products"), {version:1,mode:readBuildMode(process.env.SEMIO_BUILD_MODE),renderer:"react",command:"serve",engines:"play"}), root, config: join(root, "🏗️builder/🌐️vite/🟦️.ts"), host: "127.0.0.1", port: 0, signal: this.invocation.control.signal, session, ready: async url => { await publishServiceReady(sessionRoot, session, url, controller.signal); console.log(`Play E2E ready: ${url}`); } });
    } finally { process.removeListener("SIGINT", interrupt); process.removeListener("SIGTERM", terminate); await closeServiceSession(sessionRoot, session); }
  }
}

const router = new ScriptRouter(import.meta.dir).register("prepare", PreparationScript).register("activate", ActivationScript).register("serve", ServeScript).register("prepare-test", PrepareTestScript).register("serve-test", ServeTestScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (router).run(process.argv.slice(2), original));
