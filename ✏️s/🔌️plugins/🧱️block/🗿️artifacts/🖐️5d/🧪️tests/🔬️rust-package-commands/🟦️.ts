import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { runCmd } from "@semio-tech/framework/process/execution";
import { readdirSync } from "node:fs";
import { resolve } from "node:path";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args: string[]): void {
  if (args.length !== 1 || args[0] !== "snapshot-sqlite-source") throw new Error("verify snapshot-sqlite-source");
  const roots: string[] = [];
  const visit = (dir: string): void => {
    for (const item of readdirSync(dir, { withFileTypes: true })) {
      const path = resolve(dir, item.name);
      if (item.isDirectory() && (item.name !== "🧪️tests" || path.includes("📸️snapshot"))) visit(path);
      else if (item.name.endsWith(".ts")) roots.push(path);
    }
  };
  visit(resolve(context.root, "../../🏅️standards/🔖️1/🪆️subsets/✳️any"));
  visit(resolve(context.root, "../../../◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any"));
  visit(resolve(context.root, "../../../🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any"));
  roots.push(resolve(context.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧩️suite/🟦️.ts"),resolve(context.root,"../../../◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧩️suite/🟦️.ts"));
  runCmd("bun", [resolve(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--skipLibCheck", ...roots], { cwd: context.repoRoot });
}
