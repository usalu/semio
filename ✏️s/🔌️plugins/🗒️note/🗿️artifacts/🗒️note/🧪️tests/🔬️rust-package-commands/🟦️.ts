import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { runArtifactRustTests } from "@semio-tech/framework/process/cargo-test";
import { runCmd } from "@semio-tech/framework/process/execution";
import { join } from "node:path";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
if (segments[0] === "snapshot-guest") { await runArtifactRustTests("semio-s-artifact-note-note", context.repoRoot, ["quick","--lib","snapshot_guest_tests"], context.invocation); return; }
if (segments[0] === "sqlite-snapshot-native") { await runArtifactRustTests("semio-s-artifact-note-note", context.repoRoot, ["quick","--lib","--no-fail-fast","standards::v1::subsets::any::io::sqlite::snapshot::component::tests::sqlite_snapshot_note_"], context.invocation); return; }
if (segments[0] === "sqlite-snapshot-guest") {
    await runArtifactRustTests("semio-s-artifact-note-note", context.repoRoot, ["quick", "--lib", "sqlite_guest_tests"], context.invocation);
    return;
  }
if (segments[0] === "note-empty-config-ownership") {
    const windowRoot = join(context.repoRoot, "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window");
    const oracle = join(windowRoot, "🧪️tests/🔬️ownership/🟦️.ts");
    const { testNoteEmptyConfigOwnership } = await import(oracle);
    testNoteEmptyConfigOwnership();
    runCmd("bun", [join(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", join(windowRoot, "🧬️schema/🟦️.ts"), oracle], { cwd: context.repoRoot });
    if (segments[1] === "native") {
      await runArtifactRustTests("semio-s-artifact-note-note", context.repoRoot, ["--lib", "note_empty_config_owner_"], context.invocation);
    }
    return;
  }
  throw new Error('Unknown owned verification '+segments.join(' '));
}
