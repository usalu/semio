#!/usr/bin/env bun
import { runBudgetedTestCommand } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🌉️ `@semio-tech/framework-os-mcp` TS task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`.
 * Nx restores the Rust build before these process consumers run. An explicit binary override
 * remains a strict prebuilt-artifact seam; both paths require an executable before Vitest starts. */
import { mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { runVitest } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { requireMcpBinary, runMcpClientEndToEnd } from "../../🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments, "quick");
    console.log(`[test] ${requireMcpBinary(this.repoRoot)}`);
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
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

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("client-e2e", ClientEndToEndScript);

/** 🧱️ Runs the retained owned fixture law against its independent oracle. */
class ApprovalRequestFixtureScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-approval-request-fixture accepts no arguments");
    await runBudgetedTestCommand(process.execPath, ["test", join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/✅️approval-request/🟦️.ts")], { cwd: this.repoRoot, env: process.env, budgetMs: 15_000, throwOnFailure: true });
  }
}

router.register("test-approval-request-fixture", ApprovalRequestFixtureScript);

await runScriptMain(router, { defaultCommand: "test" });
