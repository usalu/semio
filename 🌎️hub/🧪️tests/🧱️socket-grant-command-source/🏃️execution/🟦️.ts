import { join } from "node:path";
import { BundleScript } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";

/** 🧱️ Runs the bounded socket-grant command ownership contract. */
export class HubSocketGrantCommandSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("socket-grant-command-source-check accepts no arguments");
    await runOwnedCommand("bun", ["test", join(this.repoRoot, "🌎️hub/🧪️tests/🧱️socket-grant-command-source/🟦️.ts")], this.repoRoot, "hub-socket-grant-command-source", 120_000);
  }
}
