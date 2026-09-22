import { join } from "node:path";
import { BundleScript } from "../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";

/** 💬️ Runs the agent-reply gate: a real `semio-os-mcp` stdio gateway launched from `.mcp.json`
 * against an already-running React `dev` session, driving the whole `conversation_reply` channel in
 * a real browser — the tool on the live surface, its scope gate, a streamed turn rendered as one
 * row, no duplicated tool-call row, and the human's own typed turn reaching the agent's inbox. The
 * session is a precondition rather than something this gate boots, because an activation costs
 * minutes and every developer already has one open. */
export class OsMcpAgentReplyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("agent-reply-check accepts no arguments");
    await runOwnedCommand("bun", [join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/💬️agent-reply/🟦️.ts")], this.repoRoot, "os-mcp-agent-reply", 900_000);
  }
}
