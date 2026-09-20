import { join } from "node:path";
import { BundleScript } from "../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";

/** 🤖️ Runs the live agent-loop gate: a real `semio-os-mcp` stdio gateway launched from `.mcp.json`
 * against an already-running React `dev` session, driven through the whole (a)–(e) transcript in a
 * real browser. The session is a precondition rather than something this gate boots, because an
 * activation costs minutes and every developer already has one open — the gate says exactly which
 * launch row to start when none answers. */
export class OsMcpLiveAgentLoopScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("live-agent-loop-check accepts no arguments");
    await runOwnedCommand("bun", [join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🤖️live-agent-loop/🟦️.ts")], this.repoRoot, "os-mcp-live-agent-loop", 900_000);
  }
}
