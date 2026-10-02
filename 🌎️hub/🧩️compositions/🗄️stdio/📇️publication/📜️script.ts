#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 📣️ Runs the concrete composition's catalog publication and portable laws. */
import { runVitest, getWorkspaceRoot } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { publishTrustedStdioCatalogV1 } from "./✅️trusted-stdio-catalog/🟦️.ts";

import { join } from "node:path";

/** 🗄️ Publishes the first-party stdio native-codec catalog without claiming hub bundle admission. */
class TrustedCatalogPublishScript extends BundleScript {
  run(segments: string[]): void {
    const option = (name: string): string | undefined => {
      const index = segments.indexOf(name);
      return index < 0 ? undefined : segments[index + 1];
    };
    const outDir = option("--out") ?? join(this.root, "🤖️generated");
    const receipt = publishTrustedStdioCatalogV1({ outDir, repoRoot: getWorkspaceRoot() });
    console.log(`trusted stdio catalog ${receipt.publication}: ${receipt.catalog.nativeCodecs.length} codecs, open=${receipt.catalog.openTargets[0]?.artifactKind}, hubBundle=${receipt.catalog.hubBundle}, component=${receipt.catalog.componentAdmission.status}, sha256=${receipt.catalogSha256} -> ${receipt.outPath}`);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "./🧪️tests/🎚️config/🟦️.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("publish", TrustedCatalogPublishScript).register("test", TestScript);
if (import.meta.main) await runScriptMain(router, { defaultCommand: "test" });
