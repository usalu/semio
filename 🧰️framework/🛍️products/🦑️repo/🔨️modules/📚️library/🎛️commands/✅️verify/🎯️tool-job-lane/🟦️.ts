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
import {sayVerifyV1,refuseVerifyV1} from "../📖️texts/🟦️.ts";

/** 🎯️ Tool-job coverage verification lane. */
export abstract class VerifyToolJobLane extends VerifyInteractivityLanes {
  /** 🏭️ Permanent Phase-8 generated inventory, factory-registration, and no-bypass gate. */
  protected async runToolJobCoverage(args: string[]): Promise<void> {
    if (args[0] === "mounted-dispatch") {
      if (args.length !== 1) throw refuseVerifyV1("usage","tool-jobs.expected-interactivity-mounted-dispatch");
            sayVerifyV1("tool-jobs.interactivity-mounted-dispatch-laws-clean",{laws:String(await this.lanes.toolJobLatestWinsSelfTests())});
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
      if (rustFixture !== toolJobFixedOperationRustFixtureSource(this.root)) throw refuseVerifyV1("failed","tool-jobs.interactivity-fixed-operation-fixture-generated");
      const source = `${policyReadModuleTreeSource(this.root, "🧰️framework/🔨️modules/🧵️job/🦀️.rs")}\n${policyReadFileSafe(this.root, "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️fixed-operation-registry/🦀️.rs")}`;
      const mutations = this.lanes.toolJobFixedOperationRegistrySelfTests(source);
      sayVerifyV1("tool-jobs.interactivity-fixed-operation-fixture-laws",{laws:fixture.results.length,mutations:String(mutations)});
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
      if (!toolJobLiveFixedReplayExact(actor, shard, executor, wgpu, actionBus, pluginApp, pluginHost, programBridge)) throw refuseVerifyV1("failed","tool-jobs.interactivity-p2c-live-fixed-replay");
      sayVerifyV1("tool-jobs.interactivity-p2c-live-source-clean",{mutations:String(mutations)});
      return;
    }
    if (args.includes("--p2a1-only")) {
      const store = policyReadOsStoreEvidence(this.root);
      const mutations = args.includes("--self-test") ? this.lanes.toolJobArtifactEnvelopeRejectionTransferSelfTests(store) : 0;
      if (!toolJobArtifactEnvelopeRejectionTransferExact(store)) throw refuseVerifyV1("failed","tool-jobs.interactivity-p2a1-store-rejection-transfer");
      sayVerifyV1("tool-jobs.interactivity-p2a1-live-source-clean",{mutations:String(mutations)});
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
      if (rustFixture !== toolJobFixedOperationRustFixtureSource(this.root)) throw refuseVerifyV1("failed","tool-jobs.interactivity-generated-fixed-operation-rust");
      const plugin = policyReadFileSafe(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs");
      const activation = toolJobFactoryProofActivationScan(this.root);
      if (activation.failures.length > 0) throw refuseVerifyV1("failed","tool-jobs.interactivity-app-activation-factory-proof",{activation:String(JSON.stringify(activation))});
      sayVerifyV1("tool-jobs.interactivity-exact-factory-proof-owners",{owners:String(activation.owners),customRows:String(activation.customRows),genericRows:String(activation.genericRows)});
      sayVerifyV1("tool-jobs.interactivity-self-tests-clean",{selfTests:String((await this.lanes.toolJobCoverageSelfTests()) + this.lanes.toolJobFixedOperationRegistrySelfTests(jobRuntime) + this.lanes.toolJobDrawingGestureOperationOwnerSelfTests() + this.lanes.toolJobArtifactRetainedCommandSelfTests(artifactRetainedCommand, plugin) + fixture.results.length + 1)});
      return;
    }
    const formatIndex = args.indexOf("--format");
    const format = formatIndex >= 0 ? args[formatIndex + 1] : "text";
    if (format !== "text" && format !== "json") throw refuseVerifyV1("failed","tool-jobs.interactivity-unsupported-format",{format:String(JSON.stringify(format))});
    const report = await toolJobCoverageRun(this.root, this.lanes);
    const outputIndex = args.indexOf("--output");
    const output = outputIndex >= 0 ? args[outputIndex + 1] : undefined;
    if (output) {
      const path = resolve(this.root, output);
      const relativePath = relative(this.root, path);
      if (relativePath.startsWith("..") || relativePath === "") throw refuseVerifyV1("invalid","tool-jobs.interactivity-output-must-name-a");
      writeFileSync(path, `${JSON.stringify(report, null, 2)}\n`, "utf8");
    }
    if (format === "json" && !output) console.log(JSON.stringify(report, null, 2));
    else if (output) sayVerifyV1("tool-jobs.interactivity-wrote",{output:String(output)});
    else sayVerifyV1("tool-jobs.interactivity-production-hosts-production-invocations",{macroHostFiles:String(report.macroHostFiles),macroInvocations:String(report.macroInvocations),commandRows:String(report.commandRows),literalRegistrations:String(report.literalRegistrations),fixtureMacroHostFiles:String(report.fixtureMacroHostFiles),fixtureMacroInvocations:String(report.fixtureMacroInvocations),fixtureCommandRows:String(report.fixtureCommandRows),boundedRows:String(report.boundedRows),batchOnlyRows:String(report.batchOnlyRows),forbiddenRows:String(report.forbiddenRows),deletedRows:String(report.deletedRows),remainingCommands:report.remainingCommands.length,uniqueCommandRows:String(report.uniqueCommandRows),productionFactories:String(report.productionFactories),productionRegistrations:String(report.productionRegistrations),productionDispatches:String(report.productionDispatches),aliases:String(report.aliases),selfTests:String(report.selfTests)});
    if (report.failures.length > 0) throw refuseVerifyV1("failed","tool-jobs.interactivity",{failures:report.failures.join("; ")});
  }
}
