import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { configuredExactCargoLawPolicyV1 } from "@semio-tech/process/cargo-laws";
import { type ExactCargoLawGroup } from "@semio-tech/process/cargo-laws";
import { runAdmittedExactCargoLawsV1 } from "@semio-tech/process/cargo-test";
import { BundleScript } from "@semio-tech/process/routing";
import { exactCargoStageEnvironments } from "../../🏗️build/🛂staging-environment/🟦️.ts";
import { orderedDirectoryPublicationOracle } from "../../📇️directory/📣️publication/🧪️tests/🧾️ordered-append-broadcast/🟦️.ts";
import { checkRedemptionCallsAndHostiles, checkRedemptionFixtureContract, checkRedemptionOracleVectors } from "./🎟️redemption-fixture-and-oracle/🟦️.ts";
import { checkArchiveClosure, checkRedemptionSourceClosure, checkStateAuthorityClosure } from "./🔒️backend-source-closure/🟦️.ts";

/** 🎟️ Evaluates the neutral one-transaction invite state machine independently of every backend. */
export async function proveInviteRedemptionTransaction(repoRoot: string): Promise<number> {
  const { fixture, client, acceptanceMarker } = await checkRedemptionFixtureContract({ repoRoot });
  await checkRedemptionOracleVectors({ fixture, client });
  checkRedemptionCallsAndHostiles({ repoRoot, fixture, acceptanceMarker });
  const { body, ordered, directory, sqlite, postgres, neo4j, sourceHostiles } = checkRedemptionSourceClosure({ repoRoot });
  const { stateHostiles } = checkStateAuthorityClosure({ repoRoot, body, ordered, directory, sqlite, postgres, neo4j });
  const { archiveHostiles } = checkArchiveClosure({ body, ordered, directory, sqlite, postgres, neo4j });
  const checks =
    fixture.vectors.length + fixture.spaceStates.length + fixture.authorityRaces.length + fixture.backendOrders.length + fixture.receiptRelease.length + fixture.hostiles.length + sourceHostiles.length + stateHostiles.length + archiveHostiles.length;
  console.log(`invite-redemption-transaction-oracle: AJV=1 vectors=${fixture.vectors.length} hostiles=${fixture.hostiles.length} source-hostiles=${sourceHostiles.length}`);
  return checks;
}

export class InviteRedemptionTransactionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && !["--native", "--postgres", "--neo4j"].includes(segments[0]!))) throw new Error("invite-redemption-transaction-check accepts only --native, --postgres, or --neo4j");
    const checks = await proveInviteRedemptionTransaction(this.repoRoot);
    const lawGroups: ExactCargoLawGroup[] =
      segments[0] === "--postgres"
        ? [{ package: "semio-hub-directory-backend-laws", target: { kind: "lib" }, laws: ["postgres_tests::invite_redemption_claim_matches_neutral_contract"] }]
        : segments[0] === "--neo4j"
          ? [{ package: "semio-hub-directory-backend-laws", target: { kind: "lib" }, laws: ["neo4j_tests::invite_redemption_claim_matches_neutral_contract"] }]
          : segments[0] === "--native"
            ? [
                {
                  package: "semio-hub-directory-laws",
                  target: { kind: "lib" },
                  laws: ["unit::invite_redemption_sqlite_claim_is_exactly_once_across_concurrency_restart_and_rebuild", "unit::invite_redemption_projection_failure_rolls_back_claim_event_and_membership", "unit::invite_redemption_commit_and_publication_precede_the_next_directory_command", "unit::invite_archive_projection_serializes_independent_service_decisions"],
                },
              ]
            : [];
    if (lawGroups.length > 0) {
      const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, groups: lawGroups, progress(event) {
          console.log(`invite-redemption-transaction ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        } });
      console.log(`invite-redemption-transaction-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`invite-redemption-transaction-check: checks=${checks} mode=${segments[0] ?? "source"}`);
  }
}

export class DirectoryOrderedPublicationCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("directory-ordered-publication-check accepts only --native");
    const checks = orderedDirectoryPublicationOracle(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, ...exactCargoStageEnvironments(), groups: [
          {
            package: "semio-hub-directory-laws",
            target: { kind: "lib" },
            laws: ["unit::directory_append_and_live_broadcast_share_one_writer_guard_and_projection_order"],
          },
        ], progress(event) {
          console.log(`directory-ordered-publication ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        } });
      console.log(`directory-ordered-publication-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`directory-ordered-publication-check: checks=${checks} clean`);
  }
}
