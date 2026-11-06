import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { runArtifactRustPackagesTests } from "@semio-tech/framework/process/cargo-test";
import { runCmd } from "@semio-tech/framework/process/execution";
import { readdirSync } from "node:fs";
import { resolve } from "node:path";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args: string[]): Promise<void> {
  const dataTests: Readonly<Record<string, string>> = { "optional-fields": "optional_part_and_grip_fields_round_trip_against_independent_json", "inline-capacity": "inline_import_matches_independent_json_capacity_boundary", "catalog-fields": "catalog_extensions_round_trip_against_independent_json" };
  const filter = args.length === 1 ? dataTests[args[0]!] : undefined;
  if (filter) {
    await runArtifactRustPackagesTests(["semio-s-artifact-puzzle-5d-v1-unit-tests"], context.repoRoot, ["--offline", "--lib", filter, "--", "--nocapture"], context.invocation);
    return;
  }
  if (args.length === 1 && args[0] === "edit-acceptance") {
    await runArtifactRustPackagesTests(["semio-s-artifact-puzzle-5d-v1-unit-tests"], context.repoRoot, ["--offline", "--lib", "created_part_board_and_world_edits_reopen_with_history", "--", "--nocapture"], context.invocation);
    return;
  }
  if (args.length === 1 && args[0] === "import-acceptance") {
    await runArtifactRustPackagesTests(["semio-s-artifact-puzzle-5d-v1-unit-tests"], context.repoRoot, ["--offline", "--lib", "import_identity_matches_neutral_fixtures_and_independent_json", "--", "--nocapture"], context.invocation);
    return;
  }
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
