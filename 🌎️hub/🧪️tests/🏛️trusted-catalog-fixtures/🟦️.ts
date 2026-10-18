/** 🧭️ Trusted catalog and browser actor identity fixtures and the opened-root and identity gates.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { configuredExactCargoLawPolicyV1 } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎯️exact/🟦️.ts";
import { BundleScript } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { hubSchemaExport } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { exactCargoStageEnvironments } from "../../🏗️build/🛂staging-environment/🟦️.ts";
import { proveDocumentBrowserActorIdentityFixture, proveTrustedCatalogOpenedRootFixture } from "./🪪️browser-actor-and-opened-root-fixtures/🟦️.ts";

export class DocumentBrowserActorIdentityCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments[0] !== undefined && !["--native", "--catalog-native"].includes(segments[0]))) throw new Error("document-browser-actor-identity-check accepts only --native or --catalog-native");
    await proveDocumentBrowserActorIdentityFixture(this.repoRoot);
    if (segments[0] === undefined) return;
    const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000), listMilliseconds: 60_000, lawMilliseconds: 180_000 },cwd: this.repoRoot,...exactCargoStageEnvironments(),groups:
        segments[0] === "--catalog-native"
          ? [
              {
                package: "semio-hub-trusted-catalog-contract-laws",
                target: { kind: "lib" },
                laws: ["browser_actor_tests::trusted_browser_actor_metadata_and_generation_match_neutral_corpus"],
              },
              {
                package: "semio-hub-trusted-catalog-laws",
                target: { kind: "lib" },
                laws: ["unit::trusted_browser_actor_loader_verifies_retains_and_cancels_before_publication", "unit::selected_execution_target_assets_are_generation_and_digest_bound", "unit::verified_trusted_catalog_document_open_generation_and_resolution_are_exact", "unit::trusted_profile_generation_binds_zero_target_package_and_every_codec_row"],
              },
              { package: "semio-hub-open-plan-laws", target: { kind: "lib" }, laws: ["tests::execution_target_asset_routes_revalidate_scope_role_descriptor_and_catalog_before_each_body"] },
            ]
          : [
              {
                package: "semio-framework-replication",
                target: { kind: "lib", name: "protocol" },
                laws: [
                  "integer_from_value_matches_serde_without_coercion",
                  "scalars_round_trip",
                  "option_collapses_nested_none_like_naive_serde",
                  "vec_round_trips_and_reports_index_on_error",
                  "btreemap_round_trips_in_key_order",
                  "tuple_round_trips_as_two_element_array_like_serde_json",
                  "fixed_size_array_round_trips_and_rejects_wrong_length",
                  "phantom_data_encodes_as_null_and_decodes_from_anything",
                  "u64_round_trips_as_uint_and_f64_round_trips_as_float",
                  "i64_min_and_max_round_trip_exactly",
                  "u64_max_round_trips_exactly_beyond_f64_2_pow_53",
                  "negative_zero_float_round_trips_and_stays_a_float",
                  "whole_float_and_same_valued_integer_are_distinct_dsl_values",
                ].map((law) => `value::codec::tests::${law}`),
              },
              {
                package: "semio-framework-os-kernel",
                target: { kind: "lib" },
                laws: [
                  "os_directory::schema::tests::document_authority_json_integer_tokens_never_coerce",
                  "os_directory::schema::browser_actor::tests::document_browser_actor_v1_matches_language_neutral_fixture",
                  "os_directory::schema::tests::document_open_plan_v1_matches_language_neutral_fixture",
                  "os_directory::client::tests::execution_target_lease_compares_every_plan_and_verified_byte_field",
                  "os_directory::client::tests::native_document_admission_issues_validates_and_exchanges_exactly_once",
                  "os_directory::client::tests::hostile_or_cancelled_plan_never_reaches_receipt_exchange",
                  "os_directory::client::tests::cancellation_after_receipt_exchange_never_reaches_a_document_socket",
                  "os_directory::client::tests::mismatched_local_plugin_selection_never_exchanges_a_plan_receipt",
                ],
              },
            ],progress(event) {
        console.log(`document-browser-actor-identity-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      }});
    console.log(`document-browser-actor-identity-native-receipts: ${JSON.stringify(receipts)}`);
  }
}

export class TrustedCatalogOpenedRootCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments[0] !== undefined && segments[0] !== "--native")) throw new Error("trusted-catalog-opened-root-check accepts only --native");
    await proveTrustedCatalogOpenedRootFixture(this.repoRoot);
    if (segments[0] === undefined) return;
    const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 86_400_000), listMilliseconds: 60_000, lawMilliseconds: 180_000 },cwd: this.repoRoot,...exactCargoStageEnvironments(),groups: [
        {
          package: "semio-hub-trusted-catalog-laws",
          target: { kind: "lib" },
          laws: ["unit::trusted_catalog_opened_root_rejects_linked_roots_leaves_intermediates_and_actors", "unit::trusted_catalog_opened_handle_is_swap_stable_bounded_and_cancel_safe", "unit::trusted_catalog_relative_paths_match_the_neutral_no_link_corpus"],
        },
        {
          package: "semio-hub-boot-laws",
          target: { kind: "lib" },
          laws: ["tests::trusted_catalog_startup_is_selected_only_by_the_server_owned_data_root", "tests::configured_catalog_without_a_native_provider_fails_closed"],
        },
        {
          package: "semio-hub-hostile-input-laws",
          target: { kind: "lib" },
          laws: ["tests::quick::native_openable_stdio_provider_is_the_only_atomic_readiness_transition"],
        },
      ],progress(event) {
        console.log(`trusted-catalog-opened-root-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      }});
    console.log(`trusted-catalog-opened-root-native: laws=${receipts.reduce((sum, receipt) => sum + receipt.assertions, 0)} receipts=${JSON.stringify(receipts)}`);
  }
}

/** 🪪️ Keeps descriptor SHA-256 authority distinct from component PackageRef BLAKE3. */
export async function proveTrustedCatalogIdentityRolesFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub", "🗿️artifact-authority", "🔏️trusted-catalog", "🧪️tests/🧫️fixtures", "🪪️identity-roles");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  const validateBundle = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedBundleV1");
  assert.deepEqual(Object.keys(fixture), ["schema", "source", "packageReferenceHash", "descriptorOwnerHash", "cases"]);
  assert.equal(fixture.schema, "semio.hub.trusted-catalog-identity-roles/v1");
  assert.equal(fixture.source, "../👥️two-package/🔣️.json");
  assert.equal(fixture.packageReferenceHash, "componentBlake3");
  assert.equal(fixture.descriptorOwnerHash, "componentSha256");
  assert.deepEqual(fixture.cases.map((row: any) => row.change), ["none", "blake3-owner", "descriptor-owner", "zero-owner", "bare-kind", "bare-surface"]);
  for (const row of fixture.cases) assert.deepEqual(Object.keys(row), ["change", "codec", "open"]);
  const source = JSON.parse(readFileSync(join(root, fixture.source), "utf8"));
  assert(validateBundle(source.bundle), "identity-role source bundle must satisfy the scope-owned contract");
  const bytes = Buffer.from(source.componentHex, "hex");
  const sha256 = createHash("sha256").update(bytes).digest("hex");
  if (sha256 !== source.componentSha256 || source.componentSha256 === source.componentBlake3) throw new Error("catalog hash roles lost their independent byte identities");
  const selected = source.bundle.packages[0],
    target = selected.openTargets[0];
  if (!target.artifactKind.startsWith("s.") || target.surfaceId !== `${target.artifactKind}@1/*#editor` || target.appId !== target.surfaceId) throw new Error("catalog fixture kind/surface is not exact canonical identity");
  for (const test of fixture.cases) {
    let ownerHash = source.componentSha256,
      kind = target.artifactKind,
      surface = target.surfaceId;
    switch (test.change) {
      case "blake3-owner":
        ownerHash = source.componentBlake3;
        break;
      case "descriptor-owner":
        ownerHash = selected.descriptor.sha256;
        break;
      case "zero-owner":
        ownerHash = "00".repeat(32);
        break;
      case "bare-kind":
        kind = kind.slice(2);
        break;
      case "bare-surface":
        surface = surface.slice(2);
        break;
    }
    const codec = ownerHash === sha256 && kind === selected.nativeCodecs[0].artifactKind;
    const open = codec && surface === target.appId;
    if (codec !== test.codec || open !== test.open) throw new Error(`catalog identity role mismatch ${test.change}`);
  }
  console.log(`trusted-catalog-identity-oracle: exact=${fixture.cases.length} canonical-kind=1 descriptor-sha256=1 package-ref-blake3=distinct; no GIS provider activation`);
}
