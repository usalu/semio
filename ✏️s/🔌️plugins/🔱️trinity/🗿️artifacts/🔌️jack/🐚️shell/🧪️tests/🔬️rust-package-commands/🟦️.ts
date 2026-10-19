import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { join } from "node:path";

export const BINARY_NAME = "semio-s-artifact-trinity-jack-shell";

/** 🏃️ Runs the `RunScript` command under the caller's roots and invocation. */
export function runRunScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): void {
  const binary = join(context.root, "dist/build", process.platform === "win32" ? `${BINARY_NAME}.exe` : BINARY_NAME);
  const result = Bun.spawnSync([binary, ...segments], { cwd: context.repoRoot, stdout: "inherit", stderr: "inherit", stdin: "inherit" });
  if (result.exitCode !== 0) process.exit(result.exitCode ?? 1);
}
