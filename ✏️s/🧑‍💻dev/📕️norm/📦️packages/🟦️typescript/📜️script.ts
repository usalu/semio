#!/usr/bin/env bun
import { runCmd, runCargo, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
if (segments[0] === "norm-document-contract") {
      const { testNormDocumentContractOracle } = await import("../../../../🔌️plugins/📕️norm/🧪️tests/🪪️document-contract/🟦️.ts");
      await testNormDocumentContractOracle();
      const normRoot = join(this.repoRoot, "✏️s/🔌️plugins/📕️norm");
      const files = ["⚖️en1990", "⚡️din18599"].flatMap((artifact) => ["", "📸️snapshot", "🔺️diff"].map((facet) => join(normRoot, "🗿️artifacts", artifact, "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema", facet, "🟦️.ts")));
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", ...files, join(normRoot, "🧪️tests/🪪️document/🟦️.ts")], { cwd: this.repoRoot });
      return;
    }
if (segments[0] === "norm-results-window-ownership") {
      const configRoot = join(this.repoRoot, "✏️s/🔌️plugins/📕️norm/🪟️results/🎚️config");
      const oracle = join(configRoot, "🧪️tests/🔬️window/🟦️.ts");
      const { testNormResultsWindowOwnershipOracle } = await import(oracle);
      testNormResultsWindowOwnershipOracle();
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", join(configRoot, "🧬️schema/🟦️.ts"), join(configRoot, "🧬️schema/🧬️mutations/🟦️.ts"), oracle], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-norm-en1996", "--lib", "norm_results_window_ownership_", "--", "--nocapture"], this.repoRoot);
      }
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}
const router = new ScriptRouter(import.meta.dir).register("verify", OwnedVerifyScript);
await runScriptMain(router);
