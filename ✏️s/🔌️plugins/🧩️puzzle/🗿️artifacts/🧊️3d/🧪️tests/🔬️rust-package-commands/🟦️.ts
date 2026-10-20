import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { runArtifactRustPackagesTests } from "@semio-tech/framework/process/cargo-test";
import { runCmd } from "@semio-tech/framework/process/execution";
import { readdirSync } from "node:fs";
import { resolve } from "node:path";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args: string[]): Promise<void> {
  if(args[0]==="original-fill-fixture"&&(args.length===1||args.length===2&&args[1]==="native")){runCmd("bun",["test",resolve(context.repoRoot,"✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🎟️caller/🧪️tests/🟦️.ts")],{cwd:context.repoRoot});if(args[1]==="native")await runArtifactRustPackagesTests(["semio-s-artifact-puzzle-3d-editor-session"],context.repoRoot,["--offline","--test","precompute-fill","--","--nocapture"],context.invocation);return;}
  if (args.length === 1 && args[0] === "original-work-construction") {
    runCmd("bun", ["test", resolve(context.repoRoot, "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🎬️action/🧪️tests/🟦️.ts"), resolve(context.repoRoot, "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🟦️.ts")], { cwd: context.repoRoot });
    return;
  }
  if (args.length !== 1 || args[0] !== "snapshot-sqlite-source") throw new Error("verify snapshot-sqlite-source");
  const roots: string[] = [];
  const visit = (dir: string): void => {
    for (const item of readdirSync(dir, { withFileTypes: true })) {
      const path = resolve(dir, item.name);
      if (item.isDirectory() && path !== resolve(context.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests")) visit(path);
      else if (item.name.endsWith(".ts")) roots.push(path);
    }
  };
  visit(resolve(context.root, "../../🏅️standards/🔖️1/🪆️subsets/✳️any"));
  runCmd("bun", [resolve(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--skipLibCheck", ...roots], { cwd: context.repoRoot });
}
