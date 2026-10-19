import type { ScriptInvocation } from "@semio-tech/process/routing";
import { runCmd, runCargo } from "@semio-tech/process/execution";
import { join } from "node:path";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
if (segments[0] === "forms-try-window-ownership") {
    const configRoot = join(context.repoRoot, "✏️s/🔌️plugins/📋️forms/\u{1F5FF}\uFE0Fartifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config");
    const { testFormsTryWindowOwnership } = await import(`${configRoot}/🧪️tests/🔬️window/🟦️.ts`);
    testFormsTryWindowOwnership();
    runCmd("bun", [join(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", join(configRoot, "🧬️schema/🟦️.ts"), join(configRoot, "../🫧️transient/🧬️schema/🟦️.ts"), join(configRoot, "🧪️tests/🔬️window/🟦️.ts")], { cwd: context.repoRoot });
    if (segments[1] === "native") {
      const { runCargo } = await import("@semio-tech/repo-lib/cargo");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-forms-forms", "--lib", "forms_try_window_ownership_", "--", "--nocapture"], context.repoRoot);
    }
    return;
  }
  throw new Error('Unknown owned verification '+segments.join(' '));
}
