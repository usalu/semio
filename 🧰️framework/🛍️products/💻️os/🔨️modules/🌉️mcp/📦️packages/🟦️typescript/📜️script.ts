#!/usr/bin/env bun
/** 🌉️ `@semio-tech/framework-os-mcp` TS task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`.
 * Nx restores the Rust build before these process consumers run. An explicit binary override
 * remains a strict prebuilt-artifact seam; both paths require an executable before Vitest starts. */
import { mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { requireMcpBinary, runMcpClientEndToEnd } from "../../🟦️.ts";
import { proveMcpInferenceBridgeFixture } from "../../💡️inference-bridge/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments, "quick");
    console.log(`[test] ${requireMcpBinary(this.repoRoot)}`);
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
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
      await runVitest(this.root, ["../../🧪️tests/💡️inference-bridge/🟦️.ts"], "../../🧪️tests/🎚️config/🟦️.ts");
    }
    console.log(`inference-bridge-check ${mode}: no external model provider, no WGPU rendering, and no two-user authenticated journey is run or claimed here.`);
  }
}

/** 🤝️ The real-client end-to-end gate: spawns BOTH `.mcp.json` servers with the literal
 * `command`/`args` a Claude Code or Cursor host uses, then drives the whole agent journey over
 * stdio JSON-RPC — handshake, tool surface, catalog health, artifact create/open, a discovered
 * mutation through `action_prepare`/`action_invoke`, the snapshot that must show it, the
 * `history_undo` that must remove it again, and `inference_run` with its job progress and cancel —
 * plus the repo server's `resources/list` and a read of `repo://goals`. Nothing is stubbed and no
 * step is skipped: a red row exits non-zero. `--folder <path>` binds an existing space instead of a
 * throwaway one; the capability catalog always comes from the repo's installed plugin registry. */
class ClientEndToEndScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const flag = segments.indexOf("--folder");
    if (flag >= 0 && !segments[flag + 1]) throw new Error("usage: client-e2e [--folder <path>]");
    const folder = flag >= 0 ? join(this.repoRoot, segments[flag + 1] as string) : mkdtempSync(join(tmpdir(), "semio-mcp-client-e2e-"));
    console.log(`[client-e2e] binary ${requireMcpBinary(this.repoRoot)}`);
    console.log(`[client-e2e] workspace folder ${folder}`);
    const steps = await runMcpClientEndToEnd(this.repoRoot, folder);
    for (const step of steps) console.log(`${step.ok ? "PASS" : "FAIL"}  ${step.step} — ${step.detail}`);
    const failed = steps.filter((step) => !step.ok);
    console.log(`client-e2e: ${steps.length - failed.length}/${steps.length} steps green over the two real .mcp.json servers.`);
    if (failed.length > 0) throw new Error(`client-e2e: ${failed.length} step(s) red: ${failed.map((step) => step.step).join(", ")}`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("inference-bridge-check", InferenceBridgeCheckScript).register("client-e2e", ClientEndToEndScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
