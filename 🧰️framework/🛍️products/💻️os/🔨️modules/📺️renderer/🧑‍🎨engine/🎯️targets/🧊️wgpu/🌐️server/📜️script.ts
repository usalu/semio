#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { serveVite } from "../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts";
import { playgroundCompositionPathV1 } from "../../../../../🔌️plugin/📇️registry/🎮️playground/🧩️composition/🟦️.ts";

/** 🌐️ Owns one browser listener consuming Nx-completed WGPU artifacts. */
class ServeScript extends BundleScript {
  async run([variant, profile, ...args]: string[]): Promise<void> {
    if (variant === undefined || !["dev", "release"].includes(profile)) throw new Error("serve <variant> <dev|release> [--port <port>] [--host <host>]");
    let composition = process.env.SEMIO_WGPU_COMPOSITION_PATH === undefined ? join(import.meta.dir, "🎚️config/🟦️.ts") : resolve(this.repoRoot, playgroundCompositionPathV1(process.env.SEMIO_WGPU_COMPOSITION_PATH));
    let port = Number(process.env.S_OS_PORT), host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
    for (let index = 0; index < args.length; index += 2) {
      const value = args[index + 1];
      if (!value) throw new Error("Missing browser server option value");
      if (args[index] === "--port") port = Number(value);
      else if (args[index] === "--host") host = value;
      else if (args[index] === "--composition") composition = resolve(this.repoRoot, playgroundCompositionPathV1(value));
      else throw new Error("Unknown browser server option: " + args[index]);
    }
    if (!Number.isSafeInteger(port) || port < 1 || port > 65535) throw new Error("Select an explicit browser listener port");
    process.env.S_OS_PORT = String(port);
    process.env.SEMIO_PLUGIN = variant;
    process.env.SEMIO_RENDERER = "wgpu";
    process.env.SEMIO_BUILD_MODE = profile === "release" ? "ship" : "dev";
    const controller = new AbortController(), cancel = () => controller.abort();
    process.once("SIGINT", cancel);
    process.once("SIGTERM", cancel);
    try {
      await serveVite({ root: import.meta.dir, config: composition, configLoader: "native", host, port, signal: controller.signal, ready: url => console.log("WGPU browser ready: " + url + "?plugin=" + variant) });
    } finally { process.off("SIGINT", cancel); process.off("SIGTERM", cancel); }
  }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dir).register("serve", ServeScript)).run(process.argv.slice(2), original));
