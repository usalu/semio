import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/framework/process/compiler-capacity";
import { configuredExactCargoLawPolicyV1 } from "@semio-tech/framework/process/cargo-laws";
import { buildBudgetMs } from "@semio-tech/framework/process/budget";
import { runAdmittedExactCargoLawsV1 } from "@semio-tech/framework/process/cargo-test";
import { runBun } from "@semio-tech/framework/process/execution";

/** 🏃️ Runs the `GeometryContractScript` command under the caller's roots and invocation. */
export async function runGeometryContractScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments.length) throw new Error("geometry-contract accepts no arguments");
  runBun(["test", "./✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧪️tests/🔬️geometry-lifetime/🟦️.ts"], context.repoRoot);
  const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0),invocation: context.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd: context.repoRoot,env: { ...process.env, RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432" },nativeEnv: { RUST_MIN_STACK: "268435456" },groups: [{ package: "semio-s-plugin-playbook-procedural", target: { kind: "lib" }, laws: [
      "instance_geometry_admits_session_shell_before_terminal",
      "instance_geometry_replays_durable_sources_and_preserves_preview_authority",
      "preview_body_emits_world_scene",
      "export_solid_action_stashes_result_and_is_undoable",
      "import_solid_action_stashes_result_on_params",
      "import_solid_action_reports_error_when_no_data_given",
      "procedural_payload_vectors_match_the_json_oracle",
    ] }],progress(event) { console.log(`playbook-geometry-contract ${event.stage}: ${event.law ?? ""}`); }});
  console.log(`playbook-geometry-contract receipts=${receipts.length}`);
}
