import { join } from "node:path";
import { resolveCargoPackageNames } from "../🔍️package-names/🟦️.ts";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { cargoPackageSelectionsNearV1 } from "../../../🗂️workspaces/🦀️cargo/🟦️.ts";
import { runCmd } from "../../🟦️.ts";
import { buildBudgetMs } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";

/**
 * 🧹️Zero-warning gate for a crate: `cargo clippy -p <pkg> --all-targets -- -D warnings`.
 * Deny-on-warnings lives ONLY in this trailing clippy arg — never in RUSTFLAGS, which
 * would replace (not merge with) `.cargo/config.toml`'s rustflags (`-Z threads=8`, the
 * wasm32 `getrandom_backend` cfg, mold) and break every wasm build. See
 * `[workspace.lints]` in the root `Cargo.toml` for the shared lint baseline this checks.
 */
export function runCargoLint(packages: string[], cwd: string, extraArgs: string[] = [], env: NodeJS.ProcessEnv = process.env): void {
  const resolvedPackages = resolveCargoPackageNames(packages, cwd);
  const repository = getWorkspaceRoot(), rows = cargoPackageSelectionsNearV1(repository, join(cwd, "Cargo.toml"), resolvedPackages);
  const scopes = [...new Set(rows.map(row => row.workspace))];
  if (scopes.length > 1) { for (const scope of scopes) runCargoLint(rows.filter(row => row.workspace === scope).map(row => row.name), cwd, extraArgs, env); return; }
  const packageArgs = [...(rows.length ? ["--manifest-path", join(repository, rows[0]!.manifest)] : []), ...resolvedPackages.flatMap((pkg) => ["-p", pkg])];
  runCmd("cargo", ["clippy", ...packageArgs, "--all-targets", ...extraArgs, "--", "-D", "warnings"], { cwd, env, budgetMs: buildBudgetMs() });
}
