#!/usr/bin/env bun
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmd, runCargo, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
if (segments[0] === "note-document-contract") {
      if (segments[1] === "native") {
        const { runCargo } = await import("../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-note-note", "--lib", "--", "--nocapture"], this.repoRoot);
        return;
      }
      const { testNoteDocumentContractOracle } = await import("../../../../🔌️plugins/🗒️note/🧪️tests/🪪️document-contract/🟦️.ts");
      await testNoteDocumentContractOracle();
      const noteRoot = join(this.repoRoot, "✏️s/🔌️plugins/🗒️note");
      const files = ["", "📸️snapshot", "🔺️diff"].map((facet) => join(noteRoot, "🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema", facet, "🟦️.ts"));
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", ...files, join(noteRoot, "🧪️tests/🪪️document-contract/🟦️.ts")], { cwd: this.repoRoot });
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}
const router = new ScriptRouter(import.meta.dir).register("verify", OwnedVerifyScript);
await runBundleScriptMain(router, import.meta.url);
