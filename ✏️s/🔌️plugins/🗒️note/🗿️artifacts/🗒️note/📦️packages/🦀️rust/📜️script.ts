#!/usr/bin/env bun
/** 📦️ note note Rust artifact package router. */
import { runArtifactRustPackageMain, runArtifactRustTests } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { runCmd, runCargo, runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { policyReadFileSafe } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts";
import { toolJobMountedDispatchOneTurnExact } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📤️publication/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
if (segments[0] === "snapshot-guest") { await runArtifactRustTests("semio-s-artifact-note-note", this.repoRoot, ["quick","--lib","snapshot_guest_tests"], this.invocation); return; }
if (segments[0] === "sqlite-snapshot-native") { await runArtifactRustTests("semio-s-artifact-note-note", this.repoRoot, ["quick","--lib","--no-fail-fast","standards::v1::subsets::any::io::sqlite::snapshot::component::tests::sqlite_snapshot_note_"], this.invocation); return; }
if (segments[0] === "sqlite-snapshot-guest") {
      await runArtifactRustTests("semio-s-artifact-note-note", this.repoRoot, ["quick", "--lib", "sqlite_guest_tests"], this.invocation);
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
        await runArtifactRustTests("semio-s-artifact-note-note", this.repoRoot, ["--lib", "note_empty_config_owner_"], this.invocation);
      }
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-note-note", { packages: ["semio-s-artifact-note-note", "semio-s-artifact-note-note-delete-blocks", "semio-s-artifact-note-note-diff", "semio-s-artifact-note-note-insert-table-row", "semio-s-artifact-note-note-kinds-catalog", "semio-s-artifact-note-note-mut-family", "semio-s-artifact-note-note-mut-family-2", "semio-s-artifact-note-note-mut-family-3", "semio-s-artifact-note-note-mut-family-4", "semio-s-artifact-note-note-mutations", "semio-s-artifact-note-note-nudge-selection", "semio-s-artifact-note-note-schema", "semio-s-artifact-note-note-tests-hides-the-grid", "semio-s-artifact-note-note-unit-tests", "semio-s-artifact-note-note-v1"], commands: { verify: OwnedVerifyScript } });
