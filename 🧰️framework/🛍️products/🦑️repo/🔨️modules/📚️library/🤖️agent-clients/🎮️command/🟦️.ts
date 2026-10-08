import { Script } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { checkAgentClients, writeAgentClients } from "../🟦️.ts";

/** 🤖️ `agents write|check`: derives the MCP configuration of every agent and editor client from the root dashboard declaration, or fails on drift.
 * @see ../🟦️.ts */
export class AgentsScript extends Script {
  run(segments: string[]): void {
    const verb = segments[0];
    if (verb === "write") {
      const changed = writeAgentClients(this.root);
      console.log(changed.length === 0 ? "[agents write] every client file is current" : changed.map((path) => `[agents write] wrote ${path}`).join("\n"));
      return;
    }
    if (verb === "check") {
      const drift = checkAgentClients(this.root);
      for (const entry of drift) console.error(`[agents check] ${entry.path} (${entry.client}) is ${entry.reason === "missing" ? "missing" : "out of date"}; run: bun ./📜️script.ts agents write`);
      if (drift.length > 0) process.exit(1);
      console.log("[agents check] every client file matches the root declaration");
      return;
    }
    console.error("[agents] usage: bun ./📜️script.ts agents <write|check>");
    process.exit(1);
  }
}
