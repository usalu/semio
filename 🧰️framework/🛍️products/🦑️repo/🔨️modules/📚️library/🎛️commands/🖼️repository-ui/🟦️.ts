import { join } from "node:path";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🧪️ Supplies repository-owned tool and cache policy to a selected owner's test command. */
/** 🖱️ Dispatches the actual repository UI owner. */
export class RepositoryUiScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { runOwnedCommand } = await import("../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts");
    await runOwnedCommand(process.execPath, [join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖱️ui/📜️script.ts"), ...segments], this.repoRoot, "repo-ui", 0, { env: process.env });
  }
}
