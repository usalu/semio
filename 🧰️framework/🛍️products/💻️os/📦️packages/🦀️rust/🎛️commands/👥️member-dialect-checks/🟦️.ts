/** 👥️ Member commands: codec send, group visibility, backbone detach, member dialect, input census and group publication checks.
 *
 * @see ../../📜️script.ts — the package script that registers the commands of this module
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { configuredExactCargoLawPolicyV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { resolveTestLevel, TEST_LEVEL_BUDGET_MS } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { resolve } from "node:path";
import { runRepositoryTestCommand, runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { exactCargoStageEnvironments } from "../🧱️script-bases/🟦️.ts";

export class CodecSendSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-codec-send-source accepts no arguments");
    const { testNativeCodecSendFixture } = await import("../../../../🔨️modules/🏪️store/🧪️tests/📦️native-codec-send/🟦️.ts");
    testNativeCodecSendFixture();
  }
}

export class GroupVisibilitySourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-group-visibility-source accepts no arguments");
    const { testGroupVisibilityFixtures } = await import("../../../../🔨️modules/🏪️store/🧪️tests/👁️group-visibility/🟦️.ts");
    testGroupVisibilityFixtures();
  }
}

export class BackboneDetachSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-backbone-detach-source accepts no arguments");
    const { testBackboneDetachFixture } = await import("../../../../🔨️modules/🏪️store/🧪️tests/🔗️backbone-detach/🟦️.ts");
    testBackboneDetachFixture();
  }
}

export class MemberDialectSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-member-dialect-source accepts no arguments");
    const { testMemberDialectFixture } = await import("../../../../🔨️modules/🏪️store/🧪️tests/🗣️member-dialect/🟦️.ts");
    testMemberDialectFixture();
    const { testFixtureProjectionRetirement } = await import("../../../../🔨️modules/🔌️plugin/🧪️tests/🌲️fixture-projection/🟦️.ts");
    testFixtureProjectionRetirement();
  }
}

/** 🪪️ Check retained input identity costs with independent UTF8 byte authorities. */
export class MemberInputCensusSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments, "quick");
    await runRepositoryTestCommand(process.execPath, ["test", resolve(this.root, "../../🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🧪️tests/🔬️unit/🟦️.ts"), ...rest], { cwd: this.repoRoot, budgetMs: TEST_LEVEL_BUDGET_MS[level] });
  }
}

/** 🤝️ Check common member publication against independent SQLite isolation. */
export class MemberGroupPublicationSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments, "quick");
    await runRepositoryTestCommand(process.execPath, ["test", resolve(this.root, "../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🧪️tests/🔬️unit/🟦️.ts"), ...rest], { cwd: this.repoRoot, budgetMs: TEST_LEVEL_BUDGET_MS[level] });
  }
}

export class MemberDialectCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { testMemberDialectFixture } = await import("../../../../🔨️modules/🏪️store/🧪️tests/🗣️member-dialect/🟦️.ts");
    testMemberDialectFixture();
    const { testFixtureProjectionRetirement } = await import("../../../../🔨️modules/🔌️plugin/🧪️tests/🌲️fixture-projection/🟦️.ts");
    testFixtureProjectionRetirement();
    const { testOwnedDocumentClosureOracle } = await import("../../../../🔨️modules/🏪️store/🧩️composition/🌳️closure/🧪️tests/🔬️unit/🟦️.ts");
    testOwnedDocumentClosureOracle();
    const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), preparation: configuredRepositoryCargoPreparationPolicyV1(), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 3_600_000 }, cwd: this.repoRoot, ...exactCargoStageEnvironments(), cargoArgs: segments, groups: [
        {
          package: "semio-framework-schema",
          target: { kind: "lib" },
          laws: [
            "artifact_composition_fields_derive_emits_expected_slot_tables",
            "artifact_composition_fields_default_to_empty_for_leaf_artifacts",
            "artifact_composition_projection_walks_aliases_nested_options_and_cancels",
            "artifact_composition_projection_real_child_alias_has_fixed_admission_bounds",
          ],
        },
        {
          package: "semio-framework-os-kernel",
          target: { kind: "lib" },
          laws: [
            "initial_child_identity_matches_neutral_coordinates_and_blake3",
          ],
        },
        {
          package: "semio-framework-os-store-member-open-tests",
          target: { kind: "lib" },
          laws: [
            "member_factory_closed_dialect_matches_neutral_admission_corpus",
            "member_factory_closed_dialect_rejects_identity_and_owner_substitution",
          ],
        },
        {
          package: "semio-framework-os-store-tests-05",
          target: { kind: "lib" },
          laws: [
            "member_factory_closed_dialect_graph_admission_matches_neutral_corpus",
            "member_factory_closed_dialect_graph_sync_preserves_prior_state_on_rejection",
            "member_factory_closed_dialect_parent_projection_matches_neutral_corpus",
          ],
        },
        {
          package: "semio-framework-plugin",
          target: { kind: "lib" },
          laws: [
            "fixture_projection_retires_exact_tree_before_return_error_or_panic",
            "member_factory_parent_snapshot_restore_matches_neutral_corpus",
            "member_factory_closed_dialect_open_failure_retains_pin_and_drains_exact_member",
            "member_factory_closed_dialect_register_rejects_pin_without_mutating_member",
            "member_factory_closed_dialect_fresh_register_and_restore_publish_exact_parent_owner",
          ],
        },
      ] });
    console.log(`exact member admission laws: ${receipts.reduce((sum, receipt) => sum + receipt.assertions, 0)} executed across ${receipts.length} verified test executables`);
  }
}
