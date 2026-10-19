import { relative, resolve } from "node:path";
import { writeFileSync } from "node:fs";
import { toolJobFixedOperationFixtureRun, toolJobFixedOperationRustFixtureSource, toolJobSharedFrameworkActionFixtureRun } from "../../../⚖️laws/🎯️tool-job-coverage/⚙️fixed-operation/🟦️.ts";
import { policyReadFileSafe, policyReadModuleTreeSource } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { toolJobLiveFixedReplayExact } from "../../../⚖️laws/🎯️tool-job-coverage/🔁️live-replay/🟦️.ts";
import { policyReadOsStoreEvidence } from "../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import { toolJobArtifactEnvelopeRejectionTransferExact } from "../../../⚖️laws/🎯️tool-job-coverage/🗄️artifact-store/🟦️.ts";
import { dispatchOwnedScriptRoute } from "../../../🏃️process/🧭️routing/🧩️contributions/🟦️.ts";
import { toolJobFactoryProofActivationScan } from "../../../⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
import { toolJobCoverageRun } from "../../../⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts";
import { VerifyInteractivityLanes } from "../🔬️interactivity-lanes/🟦️.ts";

/** 🎯️ Tool-job coverage verification lane. */
export abstract class VerifyToolJobLane extends VerifyInteractivityLanes {
  /** 🎯️ Permanent Phase-8 generated inventory, factory-registration, and no-bypass gate. */
  protected async runToolJobCoverage(args: string[]): Promise<void> {
    if (args[0] === "mounted-dispatch") {
      if (args.length !== 1) throw new Error("Expected verify interactivity tool-jobs mounted-dispatch");
            console.log(`[verify interactivity tool-jobs mounted-dispatch] laws=${await this.lanes.toolJobLatestWinsSelfTests()} clean.`);
      return;
    }
    if (args.includes("--shared-action-fixture-only")) {
      console.log(JSON.stringify(toolJobSharedFrameworkActionFixtureRun(this.root)));
      return;
    }
    if (args.includes("--fixed-operation-rust-fixture-source-only")) {
      process.stdout.write(toolJobFixedOperationRustFixtureSource(this.root));
      return;
    }
    if (args.includes("--fixed-operation-fixture-only")) {
      const fixture = toolJobFixedOperationFixtureRun(this.root);
      const rustFixture = policyReadFileSafe(this.root, "🧰️framework/🔨️modules/🧵️job/🧪️tests/🧪️fixed-operation-registry-cases/🦀️.rs");
      if (rustFixture !== toolJobFixedOperationRustFixtureSource(this.root)) throw new Error("[verify interactivity tool-jobs fixed-operation-fixture] generated Rust fixture is stale.");
      const source = `${policyReadModuleTreeSource(this.root, "🧰️framework/🔨️modules/🧵️job/🦀️.rs")}\n${policyReadFileSafe(this.root, "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️fixed-operation-registry/🦀️.rs")}`;
      const mutations = this.lanes.toolJobFixedOperationRegistrySelfTests(source);
      console.log(`[verify interactivity tool-jobs fixed-operation-fixture] laws=${fixture.results.length} mutations=${mutations} rustFixtureFresh=true.`);
      console.log(JSON.stringify(fixture));
      return;
    }
    if (args.includes("--p2c-only")) {
      const actor = policyReadModuleTreeSource(this.root, "🧰️framework/🔨️modules/🎭️actor/🦀️.rs");
      const shard = policyReadModuleTreeSource(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs");
      const executor = policyReadFileSafe(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧵️executor/🦀️.rs");
      const wgpu = policyReadModuleTreeSource(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs");
      const actionBus = policyReadFileSafe(this.root, "🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs");
      const pluginApp = policyReadFileSafe(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs");
      const pluginHost = policyReadModuleTreeSource(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs");
      const programBridge = policyReadModuleTreeSource(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs");
      const mutations = args.includes("--self-test") ? this.lanes.toolJobLiveFixedReplaySelfTests(actor, shard, executor, wgpu, actionBus, pluginApp, pluginHost, programBridge) : 0;
      if (!toolJobLiveFixedReplayExact(actor, shard, executor, wgpu, actionBus, pluginApp, pluginHost, programBridge)) throw new Error("[verify interactivity tool-jobs p2c] live fixed replay contract failed.");
      console.log(`[verify interactivity tool-jobs p2c] live-source clean; hostile-mutations=${mutations}.`);
      return;
    }
    if (args.includes("--p2a1-only")) {
      const store = policyReadOsStoreEvidence(this.root);
      const mutations = args.includes("--self-test") ? this.lanes.toolJobArtifactEnvelopeRejectionTransferSelfTests(store) : 0;
      if (!toolJobArtifactEnvelopeRejectionTransferExact(store)) throw new Error("[verify interactivity tool-jobs p2a1] Store rejection-transfer contract failed.");
      console.log(`[verify interactivity tool-jobs p2a1] live-source clean; hostile-mutations=${mutations}.`);
      return;
    }
    if (args.includes("--p6i-only")) {
      await dispatchOwnedScriptRoute(this.invocation,this.root, ["verify", "fem-live-visual-publication", ...args]);
      return;
    }
    if (args.includes("--p6h-only")) {
      await dispatchOwnedScriptRoute(this.invocation,this.root, ["verify", "fem-numerical-microcursor", ...args]);
      return;
    }
    if (args.includes("--self-test")) {
      const jobRuntime = policyReadModuleTreeSource(this.root, "🧰️framework/🔨️modules/🧵️job/🦀️.rs");
      const artifactRetainedCommand = policyReadModuleTreeSource(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs");
      const fixture = toolJobFixedOperationFixtureRun(this.root);
      const rustFixture = policyReadFileSafe(this.root, "🧰️framework/🔨️modules/🧵️job/🧪️tests/🧪️fixed-operation-registry-cases/🦀️.rs");
      if (rustFixture !== toolJobFixedOperationRustFixtureSource(this.root)) throw new Error("[verify interactivity tool-jobs] generated fixed-operation Rust fixture is stale.");
      const plugin = policyReadFileSafe(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs");
      const activation = toolJobFactoryProofActivationScan(this.root);
      if (activation.failures.length > 0) throw new Error(`[verify interactivity tool-jobs] app activation factory proof scan: ${JSON.stringify(activation)}`);
      console.log(`[verify interactivity tool-jobs] exact-factory-proof-owners=${activation.owners} custom-rows=${activation.customRows} generic-rows=${activation.genericRows} clean.`);
      console.log(`[verify interactivity tool-jobs] self-tests=${(await this.lanes.toolJobCoverageSelfTests()) + this.lanes.toolJobFixedOperationRegistrySelfTests(jobRuntime) + this.lanes.toolJobDrawingGestureOperationOwnerSelfTests() + this.lanes.toolJobArtifactRetainedCommandSelfTests(artifactRetainedCommand, plugin) + fixture.results.length + 1} clean.`);
      return;
    }
    const formatIndex = args.indexOf("--format");
    const format = formatIndex >= 0 ? args[formatIndex + 1] : "text";
    if (format !== "text" && format !== "json") throw new Error(`[verify interactivity tool-jobs] unsupported format ${JSON.stringify(format)}.`);
    const report = await toolJobCoverageRun(this.root, this.lanes);
    const outputIndex = args.indexOf("--output");
    const output = outputIndex >= 0 ? args[outputIndex + 1] : undefined;
    if (output) {
      const path = resolve(this.root, output);
      const relativePath = relative(this.root, path);
      if (relativePath.startsWith("..") || relativePath === "") throw new Error("[verify interactivity tool-jobs] --output must name a file inside the repository.");
      writeFileSync(path, `${JSON.stringify(report, null, 2)}\n`, "utf8");
    }
    if (format === "json" && !output) console.log(JSON.stringify(report, null, 2));
    else if (output) console.log(`[verify interactivity tool-jobs] wrote ${output}`);
    else console.log(`[verify interactivity tool-jobs] production-hosts=${report.macroHostFiles} production-invocations=${report.macroInvocations} production-rows=${report.commandRows} literal-registrations=${report.literalRegistrations} fixture-hosts=${report.fixtureMacroHostFiles} fixture-invocations=${report.fixtureMacroInvocations} fixture-rows=${report.fixtureCommandRows} admitted=${report.boundedRows} batch-only=${report.batchOnlyRows} forbidden=${report.forbiddenRows} deleted=${report.deletedRows} remaining=${report.remainingCommands.length} unique=${report.uniqueCommandRows} factories=${report.productionFactories} registrations=${report.productionRegistrations} dispatches=${report.productionDispatches} aliases=${report.aliases} self-tests=${report.selfTests}`);
    if (report.failures.length > 0) throw new Error(`[verify interactivity tool-jobs] ${report.failures.join("; ")}`);
  }
}
