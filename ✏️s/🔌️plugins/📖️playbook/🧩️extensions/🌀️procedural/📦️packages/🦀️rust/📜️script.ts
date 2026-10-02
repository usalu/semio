#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/⏱️budget/🟦️.ts";
/** 📦️ Extension package router: `bun ./📜️script.ts <test|package>`. */
import { runRepositoryCargoTests, runRepositoryExactCargoLaws, runExtensionComponentPackage, runBun } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests(["semio-s-plugin-playbook-procedural"], this.repoRoot, rest);
  }
}

class GeometryContractScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("geometry-contract accepts no arguments");
    runBun(["test", "./✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧪️tests/🔬️geometry-lifetime/🟦️.ts"], this.repoRoot);
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      env: { ...process.env, RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432" },
      nativeEnv: { RUST_MIN_STACK: "268435456" },
      groups: [{ package: "semio-s-plugin-playbook-procedural", target: { kind: "lib" }, laws: [
        "instance_geometry_admits_session_shell_before_terminal",
        "instance_geometry_replays_durable_sources_and_preserves_preview_authority",
        "preview_body_emits_world_scene",
        "export_solid_action_stashes_result_and_is_undoable",
        "import_solid_action_stashes_result_on_params",
        "import_solid_action_reports_error_when_no_data_given",
        "procedural_payload_vectors_match_the_json_oracle",
      ] }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: buildBudgetMs(),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) { console.log(`playbook-geometry-contract ${event.stage}: ${event.law ?? ""}`); },
    });
    console.log(`playbook-geometry-contract receipts=${receipts.length}`);
  }
}

class PackageScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("package writes the Nx-owned deliverable and accepts no output override");
    await runExtensionComponentPackage({ rsDir: import.meta.dir, repoRoot: this.repoRoot });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("geometry-contract", GeometryContractScript).register("canonical-architecture", GeometryContractScript).register("package", PackageScript);
await runScriptMain(router, { defaultCommand: "test" });
