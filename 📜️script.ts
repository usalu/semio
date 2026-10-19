#!/usr/bin/env bun
/**
 * 🧭️ Monorepo command router: `bun ./📜️script.ts <verb> [segments…]` (e.g. `📜️script.ts dev storybook`, `📜️script.ts dev mcp stdio os`).
 */
import type { TechnologyLinter } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚨️policy/🧩️linters/🟦️.ts";
import { type BreachRecord } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚨️policy/🔖️breach/🟦️.ts";
import { Script, ScriptRouter } from "./🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import type { SelfTestLanes } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧪️self-test-lanes/🟦️.ts";

const WORKSPACE_ROOT = dirname(fileURLToPath(import.meta.url));

/** 💤️ Loads verification lanes only when the verification command is selected. */
async function loadRootVerifyScript() {
  const { VerifyScript } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🟦️.ts");
  const { semioMemberStoreProducerSourceSelfTests } = await import("./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/📥️source/🧪️tests/🟦️.ts");
  const { toolJobArtifactEnvelopeRejectionTransferSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🧪️tests/🔬️tool-job-artifact-envelope-rejection-transfer/🟦️.ts");
  const { toolJobLiveFixedReplaySelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🧪️tests/🔬️tool-job-live-fixed-replay/🟦️.ts");
  const { toolJobFixedOperationRegistrySelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🧪️tests/🔬️tool-job-fixed-operation-registry/🟦️.ts");
  const { toolJobDrawingGestureOperationOwnerSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🧪️tests/🔬️tool-job-drawing-gesture-operation-owner/🟦️.ts");
  const { toolJobArtifactRetainedCommandSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🧪️tests/🔬️tool-job-artifact-retained-command/🟦️.ts");
  const { interactivityAllAppDiscoverySelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧪️tests/🔬️interactivity-all-app-discovery/🟦️.ts");
  const { interactivityRuntimeSourceSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧪️tests/🔬️interactivity-runtime-source/🟦️.ts");
  const { interactivityToolRunPolicySelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧪️tests/🧪️interactivity-tool-run-policy/🟦️.ts");
  const { historyClosurePolicySelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🪣️tool-run-and-history/🧪️tests/🧪️history-closure-policy/🟦️.ts");
  const { interactivityLiveReconcileSelfTests } = await import("./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-live-reconcile/🟦️.ts");
  const { interactivityMountedLayoutTextSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🖥️mounted/🧪️tests/🔬️interactivity-mounted-layout-text/🟦️.ts");
  const { interactivityMountedFrameTransactionSelfTests } = await import("./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-mounted-frame-transaction/🟦️.ts");
  const { interactivityMountedEngineSurfaceLifetimeSelfTests } = await import("./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-mounted-engine-surface-lifetime/🟦️.ts");
  const { interactivityMountedPreparedRenderSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🖥️mounted/🖼️mounted-surface/🧪️tests/🔬️interactivity-mounted-prepared-render/🟦️.ts");
  const { interactivityMountedSurfaceLaneSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🖥️mounted/🖼️mounted-surface/🧪️tests/🔬️interactivity-mounted-surface-lane/🟦️.ts");
  const { interactivityPreparedRasterProducerSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🧪️tests/🔬️interactivity-prepared-raster-producer/🟦️.ts");
  const { interactivityShardExecutorSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🧪️tests/🔬️interactivity-shard-executor/🟦️.ts");
  const { interactivityStoreSyncSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🧪️tests/🔬️interactivity-store-sync/🟦️.ts");
  const { interactivityDbIoB1B6SelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🧪️tests/🔬️interactivity-db-io-b1-b6/🟦️.ts");
  const { interactivityP1qR4SelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🧪️tests/🔬️interactivity-p1q-r4/🟦️.ts");
  const { interactivityDbIoCallerMigrationSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🧪️tests/🔬️interactivity-db-io-caller-migration/🟦️.ts");
  const { interactivityDbIoDirectWriterSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🧪️tests/🔬️interactivity-db-io-direct-writer/🟦️.ts");
  const { interactivityDbIoSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🧪️tests/🔬️interactivity-db-io/🟦️.ts");
  const { interactivityArtifactSubmitSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🧪️tests/🔬️interactivity-artifact-submit/🟦️.ts");
  const { interactivityDatabaseCapabilityOpenSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🧪️tests/🔬️interactivity-database-capability-open/🟦️.ts");
  const { interactivityDatabaseCatalogReadSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🧪️tests/🔬️interactivity-database-catalog-read/🟦️.ts");
  const { interactivityDatabaseCatalogBootstrapSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🧪️tests/🔬️interactivity-database-catalog-bootstrap/🟦️.ts");
  const { interactivityDatabaseCreateCatalogSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🧪️tests/🔬️interactivity-database-create-catalog/🟦️.ts");
  const { interactivityDatabaseCompactionSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🧪️tests/🔬️interactivity-database-compaction/🟦️.ts");
  const { interactivityDatabaseSyncHelloSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🧪️tests/🔬️interactivity-database-sync-hello/🟦️.ts");
  const { interactivityArtifactHistorySelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🧪️tests/🔬️interactivity-artifact-history/🟦️.ts");
  const { interactivityDbCliSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🧪️tests/🔬️interactivity-db-cli/🟦️.ts");
  const { interactivityMcpHttpTransportSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🧪️tests/🔬️interactivity-mcp-http-transport/🟦️.ts");
  const { dependencyJsLockParitySelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧪️tests/🔬️dependency-js-lock-parity/🟦️.ts");
  const { dependencyTruthSelfTests } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧪️tests/🔬️dependency-truth/🟦️.ts");
  const selfTestLanes: SelfTestLanes = {
    semioMemberStoreProducerSourceSelfTests,
    toolJobArtifactEnvelopeRejectionTransferSelfTests,
    toolJobLiveFixedReplaySelfTests,
    toolJobFixedOperationRegistrySelfTests,
    toolJobDrawingGestureOperationOwnerSelfTests,
    toolJobArtifactRetainedCommandSelfTests,
    interactivityAllAppDiscoverySelfTests,
    interactivityRuntimeSourceSelfTests,
    interactivityToolRunPolicySelfTests,
    historyClosurePolicySelfTests,
    interactivityLiveReconcileSelfTests,
    interactivityMountedLayoutTextSelfTests,
    interactivityMountedFrameTransactionSelfTests,
    interactivityMountedEngineSurfaceLifetimeSelfTests,
    interactivityMountedPreparedRenderSelfTests,
    interactivityMountedSurfaceLaneSelfTests,
    interactivityPreparedRasterProducerSelfTests,
    interactivityShardExecutorSelfTests,
    interactivityStoreSyncSelfTests,
    interactivityDbIoB1B6SelfTests,
    interactivityP1qR4SelfTests,
    interactivityDbIoCallerMigrationSelfTests,
    interactivityDbIoDirectWriterSelfTests,
    interactivityDbIoSelfTests,
    interactivityArtifactSubmitSelfTests,
    interactivityDatabaseCapabilityOpenSelfTests,
    interactivityDatabaseCatalogReadSelfTests,
    interactivityDatabaseCatalogBootstrapSelfTests,
    interactivityDatabaseCreateCatalogSelfTests,
    interactivityDatabaseCompactionSelfTests,
    interactivityDatabaseSyncHelloSelfTests,
    interactivityArtifactHistorySelfTests,
    interactivityDbCliSelfTests,
    interactivityMcpHttpTransportSelfTests,
    dependencyJsLockParitySelfTests,
    dependencyTruthSelfTests,
    toolJobCoverageSelfTests: async () => (await import("./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-coverage/🟦️.ts")).toolJobCoverageSelfTests(),
    toolJobLatestWinsSelfTests: async () => (await import("./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts")).toolJobLatestWinsSelfTests(),
  };

  /** ✅️ The root `verify` command bound to this checkout's self-test lanes. */
  class RootVerifyScript extends VerifyScript {
    protected readonly lanes = selfTestLanes;
  }
  return RootVerifyScript;
}

//#region 🔖️Dispatch
const router = new ScriptRouter(WORKSPACE_ROOT, WORKSPACE_ROOT)
  .registerLazy("repo", async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts")).RepoScript)
  .registerLazy("os", async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧩️os/🟦️.ts")).OsScript)
  .registerLazy("components", async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🚀️release/🟦️.ts")).ComponentsScript)
  .registerLazy("semio", async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧩️os/🟦️.ts")).SemioScript)
  .registerLazy("examples", async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📚️examples/🟦️.ts")).ExamplesScript)
  .registerLazy("setup", async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🔧️setup/🟦️.ts")).SetupScript)
  .registerLazy("agents", async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🤖️agent-clients/🎮️command/🟦️.ts")).AgentsScript)
  .registerLazy("dev", async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🟦️.ts")).DevScript)
  .registerLazy("generate", async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🏭️generate/🟦️.ts")).GenerateScript)
  .register(
    "scale-fixture",
    /** 🧫️ 50×50 scale fixture (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, packet F1) —
     * `check` regenerates the seeded registry/catalog and diffs, the same freshness idiom
     * `plugin-registry:check` uses. */
    class extends Script {
      async run(segments: string[]): Promise<void> {
        const sub = segments[0];
        if (sub === "check") {
          const { runCmd } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts");
          runCmd("bun", ["repo", "run", "@semio-tech/framework-os-dev:scale-fixture-check"], { cwd: this.root });
          return;
        }
        throw new Error(`unknown scale-fixture subcommand: ${sub} (expected check)`);
      }
    },
  )
  .registerLazy("new", async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts")).CleanMechanismNewScript)
  .registerLazy("schema", async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts")).SchemaScript)
  .registerLazy("lint", async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧹️code-style/🟦️.ts")).LintScript)
  .registerLazy("verify", loadRootVerifyScript)
  .registerLazy("format", async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧹️code-style/🟦️.ts")).FormatScript)
  .registerLazy("test", async () => class extends Script {
    async run(segments: string[]): Promise<void> {
      if (segments[0] === "entrypoint") {
        if (segments.length !== 1) throw Error("Expected test entrypoint without additional arguments");
        const { runOwnedCommand } = await import("./🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts");
        const { scriptInvocationBudget } = await import("./🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
        await runOwnedCommand(process.execPath, ["test", "./🧪️tests/🧭️entrypoint/🟦️.ts"], this.repoRoot, "workspace:command-startup", scriptInvocationBudget(this.invocation, 120_000), { signal: this.invocation.control.signal });
        return;
      }
      const { TestScript } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧪️test/🟦️.ts");
      await new TestScript(this.root, this.repoRoot, this.invocation).run(segments);
    }
  })
  .registerLazy("bench", async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧪️test/🟦️.ts")).BenchScript)
  .registerLazy("stdio", async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗄️stdio/🟦️.ts")).StdioScript)
  .registerLazy("build", async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts")).BuildScript)
  .registerLazy("release", async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🚀️release/🟦️.ts")).ReleaseScript)
  .registerLazy("cpp", async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/📜️script.ts")).CppScript)
  .registerLazy("publish", async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🚀️release/🟦️.ts")).PublishScript)
  .registerLazy("clean", async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts")).CleanScript)
  .registerLazy("micro-commit", async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/🟦️.ts")).MicroCommitScript)
  .registerLazy("commit", async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/🟦️.ts")).CommitScript);

//#endregion 🔖️Dispatch

/** ⚖️ The Wave 4 app-plugin consistency lint the policy runner loads from this script (`bun ./📜️script.ts policy`). */
export const policy = async (_l: TechnologyLinter): Promise<BreachRecord[]> => {
  const { policyBreaches } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧭️breaches/🟦️.ts");
  return policyBreaches();
};


if (import.meta.main) {
  const args = process.argv.slice(2);
  if (args.length === 1 && ["--help", "-h"].includes(args[0]!)) console.log(router.usage());
  else if (args[0] === "repo") {
    const { runBuildCliV1 } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts");
    await runBuildCliV1(WORKSPACE_ROOT, args);
  }
  else {
    const { dispatchPolicyArgv, runWorkspaceScriptMain } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚨️policy/🧭️entrypoints/🟦️.ts");
    if (!(await dispatchPolicyArgv(args, import.meta.url))) await runWorkspaceScriptMain(router);
  }
}
