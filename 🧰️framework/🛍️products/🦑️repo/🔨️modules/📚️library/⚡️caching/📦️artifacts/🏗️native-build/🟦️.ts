import { requestedCompilerAdmissionControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { readFileSync } from "node:fs";
import { join, resolve, dirname } from "node:path";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { prepareCargoWorkspaceInvocation } from "../../../🗂️workspaces/🦀️cargo/🟦️.ts";
import {repositoryCargoPreparationStorageV1} from "../📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import { cargoDirectories } from "../../🦀️cargo/🟦️.ts";
import { readBuildMode } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { repoCacheDirectory } from "../../🟦️.ts";
import { buildBudgetMs } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { buildCargoArtifacts, type CargoArtifactBuildPolicyV1, type CargoArtifactBuildOptionsV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";

/** 🏷️ The one version every crate in this workspace carries (`[workspace.package] version`). */
export function workspaceCargoVersion(repoRoot = getWorkspaceRoot()): string {
  const manifest = readFileSync(join(repoRoot, "Cargo.toml"), "utf8");
  const heading = "[workspace.package]";
  const start = manifest.indexOf(heading);
  if (start < 0) throw new Error("Cargo.toml declares no [workspace.package] section");
  const rest = manifest.slice(start + heading.length);
  const end = rest.indexOf("\n[");
  const version = /^\s*version\s*=\s*"([^"]+)"/m.exec(end < 0 ? rest : rest.slice(0, end));
  if (!version) throw new Error("Cargo.toml declares no [workspace.package] version");
  return version[1]!;
}

/** 🏗️ Authors explicit repository compiler storage for one source owner. */
export function repositoryCargoArtifactBuildPolicyV1(cwd: string, root = getWorkspaceRoot(), environment: Readonly<Record<string,string|undefined>> = process.env): CargoArtifactBuildPolicyV1 {
  return { version: 1, compilerConfiguration: join(root,".cargo/config.toml"), mode: readBuildMode(environment.SEMIO_BUILD_MODE), cwd: resolve(cwd), buildDirectory: cargoDirectories(root, environment).build, leaseDirectory: join(root, ".cache", "framework", "leases"), captureDirectory: environment.SEMIO_TEST_ARTIFACT_DIR ? resolve(environment.SEMIO_TEST_ARTIFACT_DIR) : resolve(cwd, "dist"), budgetMs: buildBudgetMs() };
}
/** 🦀️ Composes selected repository preparation with the neutral compiler capture port. */
export async function buildRepositoryCargoArtifacts(manifest: string, args: string[], root: string, options: CargoArtifactBuildOptionsV1): Promise<void> {
  const policy = repositoryCargoArtifactBuildPolicyV1(root, root, options.environment ?? process.env), control = requestedCompilerAdmissionControlV1(options.control, policy.budgetMs), path = resolve(root, manifest);
  prepareCargoWorkspaceInvocation(repositoryCargoPreparationStorageV1(root), root, [options.command ?? "build", "--manifest-path", path, ...args], root, options.environment ?? process.env);
  const environment = options.environment ?? process.env;
  await buildCargoArtifacts(path, args, { ...policy, captureDirectory: environment.SEMIO_TEST_ARTIFACT_DIR ? resolve(environment.SEMIO_TEST_ARTIFACT_DIR) : resolve(dirname(path), "dist") }, { ...options, control });
}
