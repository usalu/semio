import { join } from "node:path";
import { BundleScript } from "../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";

/** 🤖️ Runs the hub-agent-participant gate: a real `semio-os-mcp` stdio gateway bound to a REMOTE
 * hub space with a delegated agent credential, driven through delegation → agent session →
 * `artifact_open` → `artifact_snapshot` → `action_prepare`/`action_invoke` → revocation. The hub is
 * a precondition rather than something this gate boots (a trusted-catalog publication costs
 * minutes); the gate names the origin it looked for when none answers. `OS_MCP_HUB_ORIGIN`,
 * `OS_MCP_HUB_EMAIL`, `OS_MCP_HUB_PASSWORD` and `OS_MCP_HUB_SPACE` select the target. */
export class OsMcpHubAgentParticipantScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("hub-agent-participant-check accepts no arguments");
    await runOwnedCommand("bun", [join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🤖️hub-agent-participant/🟦️.ts")], this.repoRoot, "os-mcp-hub-agent-participant", 900_000);
  }
}
