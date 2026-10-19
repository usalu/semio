import type { ScriptInvocation } from "@semio-tech/process/routing";
import { runCmd } from "@semio-tech/framework/process/execution";
import { readdirSync } from "node:fs";
import { resolve } from "node:path";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args: string[]): void {
  if (args.length === 1 && args[0] === "original-work-construction") {
    runCmd("bun", ["test", resolve(context.repoRoot, "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🧪️tests/🟦️.ts")], { cwd: context.repoRoot });
    return;
  }
  if (args.length !== 1 || args[0] !== "snapshot-sqlite-source") throw new Error("verify snapshot-sqlite-source");
  const roots: string[] = [];
  const visit = (dir: string): void => {
    for (const item of readdirSync(dir, { withFileTypes: true })) {
      const path = resolve(dir, item.name);
      if (item.isDirectory()) visit(path);
      else if (item.name.endsWith(".ts")) roots.push(path);
    }
  };
  visit(resolve(context.root, "../../🏅️standards/🔖️1/🪆️subsets/✳️any"));
  runCmd("bun", [resolve(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--skipLibCheck", ...roots], { cwd: context.repoRoot });
}
