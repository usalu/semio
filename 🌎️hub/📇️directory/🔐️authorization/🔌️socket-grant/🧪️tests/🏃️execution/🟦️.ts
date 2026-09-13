import { join } from "node:path";
import { BundleScript } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
import { exactCargoStageEnvironments } from "../../../../../🏗️build/🛂staging-environment/🟦️.ts";
import { proveScopedDirectorySocketRevocationFixture } from "../🧾️fixture-verification/🟦️.ts";
import { assertSocketGrantNativeLawSources, socketGrantNativeLawPlan, type SocketGrantNativeStage } from "../📋️native-law-plan/🟦️.ts";

export type SocketGrantNativeRunner = (stage: SocketGrantNativeStage, packageRoot: string, env: NodeJS.ProcessEnv) => Promise<void>;

const runNativeStage: SocketGrantNativeRunner = (stage, packageRoot, env) => runOwnedCommand("cargo", [...stage.args], packageRoot, `socket-grant:${stage.id}`, 600_000, { env });

/** 🏃️ Verifies the portable oracle and optionally executes the exact native law sequence. */
export async function runSocketGrantCheck(repoRoot: string, packageRoot: string, segments: readonly string[], runner: SocketGrantNativeRunner = runNativeStage): Promise<void> {
  if (!(segments.length === 0 || (segments.length === 1 && segments[0] === "oracle"))) throw new Error("socket-grant-check accepts no arguments or oracle");
  await proveScopedDirectorySocketRevocationFixture(repoRoot);
  assertSocketGrantNativeLawSources(repoRoot);
  if (segments.length === 1) return;
  const projected = exactCargoStageEnvironments();
  const env = { ...projected.env, ...projected.nativeEnv };
  for (const stage of socketGrantNativeLawPlan()) await runner(stage, packageRoot, env);
}

/** 🔌️ Routes the socket-grant gate through its domain-owned verifier and native plan. */
export class SocketGrantCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runSocketGrantCheck(this.repoRoot, this.root, segments);
  }
}
