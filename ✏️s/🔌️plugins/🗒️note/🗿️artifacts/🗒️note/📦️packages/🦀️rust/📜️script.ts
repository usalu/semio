#!/usr/bin/env bun
/** 📦️ note note Rust artifact package router. */
import { runArtifactRustPackageMain, runArtifactRustTests } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { runCmd, runCargo, runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
if (segments[0] === "snapshot-guest") { await runArtifactRustTests("semio-s-artifact-note-note",this.repoRoot,["quick","--lib","snapshot_guest_tests"]); return; }
if (segments[0] === "sqlite-snapshot-guest") {
      await runArtifactRustTests("semio-s-artifact-note-note", this.repoRoot, ["quick", "--lib", "sqlite_guest_tests"]);
      return;
    }
if (segments[0] === "note-empty-config-ownership") {
      const windowRoot = join(this.repoRoot, "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window");
      const oracle = join(windowRoot, "🧪️tests/🔬️ownership/🟦️.ts");
      const { testNoteEmptyConfigOwnership } = await import(oracle);
      testNoteEmptyConfigOwnership();
      const workerSource = policyReadFileSafe(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs");
      const initialDrive = "let _ = active.drive_worker_step(&pool, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)?;";
      if (!toolJobMountedDispatchOneTurnExact(workerSource)
        || toolJobMountedDispatchOneTurnExact(workerSource.replace(initialDrive, ""))
        || toolJobMountedDispatchOneTurnExact(workerSource.replace(initialDrive, `${initialDrive} ${initialDrive}`))) throw new Error("Note worker dispatch must perform exactly one initial drive");
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", join(windowRoot, "🧬️schema/🟦️.ts"), oracle], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        await runArtifactRustTests("semio-s-artifact-note-note", this.repoRoot, ["--lib", "note_empty_config_owner_"]);
      }
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-note-note", { commands: { verify: OwnedVerifyScript } });
