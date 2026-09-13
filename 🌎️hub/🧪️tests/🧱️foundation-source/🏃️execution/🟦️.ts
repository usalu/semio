import { join } from "node:path";
import { BundleScript } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";

/** 🧱️ Runs the bounded Hub foundation source contract through the shared owned-process boundary. */
export class HubFoundationSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("foundation-source-check accepts no arguments");
    await runOwnedCommand("bun", ["test", join(this.repoRoot, "🌎️hub/🧪️tests/🧱️foundation-source/🟦️.ts")], this.repoRoot, "hub-foundation-source", 120_000);
  }
}
