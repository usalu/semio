#!/usr/bin/env bun
/** 🌉️ `@semio-tech/framework-os-mcp` TS task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`.
 * Nx restores the Rust build before these process consumers run. An explicit binary override
 * remains a strict prebuilt-artifact seam; both paths require an executable before Vitest starts. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { requireMcpBinary } from "../../🟦️.ts";
import { proveMcpInferenceBridgeFixture } from "../../💡️inference-bridge/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    console.log(`[test] ${requireMcpBinary(this.repoRoot)}`);
    await runVitest(this.root, rest, "vitest.config.ts");
  }
}

/** 💡️ The MCP ↔ hub GIS Map inference bridge gate. `--source` runs only the independent Bun/AJV
 * oracle over the shared neutral fixture, the four closed wire shapes and the hub's own registered
 * routes — no Rust build, no binary, no hub. `--process` additionally builds and drives the REAL
 * `semio-os-mcp` binary over stdio JSON-RPC for the scope, binding and input laws. Neither mode
 * involves an external model provider, WGPU rendering, or a two-user journey. */
class InferenceBridgeCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const mode = segments[0] ?? "--source";
    if (segments.length > 1 || !["--source", "--process"].includes(mode)) throw new Error("usage: inference-bridge-check [--source|--process]");
    const report = proveMcpInferenceBridgeFixture(this.repoRoot);
    console.log(`inference-bridge-oracle: ajv=${report.ajv} hostile=${report.hostile} errors=${report.errors} visibility=${report.visibility} lifecycle=${report.lifecycle} routes=${report.routes} limits=${report.limits}`);
    if (mode === "--process") {
      console.log(`[inference-bridge] ${requireMcpBinary(this.repoRoot)}`);
      resolveTestLevel(["long"]);
      await runVitest(this.root, ["💡️inference-bridge.test.ts"], "vitest.config.ts");
    }
    console.log(`inference-bridge-check ${mode}: no external model provider, no WGPU rendering, and no two-user authenticated journey is run or claimed here.`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("inference-bridge-check", InferenceBridgeCheckScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
