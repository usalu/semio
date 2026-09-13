#!/usr/bin/env bun
import { join } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { serveVite } from "../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts";
import { PLAYGROUND_BUILD_TARGETS } from "../../../../../🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";

/** 🌐️ Owns one browser listener consuming Nx-completed WGPU artifacts. */
class ServeScript extends BundleScript {
  async run([variant, profile, ...args]: string[]): Promise<void> {
    const playground = PLAYGROUND_BUILD_TARGETS.find(row => row.variant === variant);
    if (!playground || !["dev", "release"].includes(profile)) throw new Error("serve <variant> <dev|release> [--port <port>] [--host <host>]");
    let port = Number(process.env.S_OS_PORT ?? playground.ports.wgpu), host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
    for (let index = 0; index < args.length; index += 2) {
      const value = args[index + 1];
      if (!value) throw new Error("Missing browser server option value");
      if (args[index] === "--port") port = Number(value);
      else if (args[index] === "--host") host = value;
      else throw new Error("Unknown browser server option: " + args[index]);
    }
    process.env.SEMIO_PLUGIN = variant;
    process.env.SEMIO_RENDERER = "wgpu";
    process.env.SEMIO_BUILD_MODE = profile === "release" ? "ship" : "dev";
    const controller = new AbortController(), cancel = () => controller.abort();
    process.once("SIGINT", cancel);
    process.once("SIGTERM", cancel);
    try {
      await serveVite({ root: import.meta.dir, config: join(import.meta.dir, "🎚️config/🟦️.ts"), configLoader: "native", host, port, signal: controller.signal, ready: url => console.log("WGPU browser ready: " + url + "?plugin=" + variant) });
    } finally { process.off("SIGINT", cancel); process.off("SIGTERM", cancel); }
  }
}

if (import.meta.main) await new ScriptRouter(import.meta.dir).register("serve", ServeScript).run(process.argv.slice(2));
