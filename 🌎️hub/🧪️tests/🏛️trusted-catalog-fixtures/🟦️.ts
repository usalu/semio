/** 🧭️ Trusted catalog and browser actor identity fixtures and the opened-root and identity gates.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { type ScriptInvocation } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { configuredExactCargoLawPolicyV1 } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { createHash, webcrypto } from "node:crypto";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { canonicalJson } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧾️serialization/🔣️json/🟦️.ts";
import { blake3Hex } from "@semio-tech/framework/hash";
import { decodePackValue, encodePackValue, packValueToExactJson } from "../../../🧰️framework/🛍️products/💻️os/🟦️.ts";
import { DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔏️document-open/🟦️.ts";
import { DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 } from "@semio-tech/framework/directory/schema";
import { componentReceiptBindsV1, componentReceiptValueV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🟦️.ts";
import { verifySemanticDescriptorPairBytesV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts";
import { runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { hubSchemaExport } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { exactCargoStageEnvironments } from "../../🏗️build/🛂staging-environment/🟦️.ts";
import { assertObservationExpectation, type ObservationExpectation } from "../../🧪️testing/🛂️expectation/🟦️.ts";
import { readHubScriptModule } from "../../🧪️testing/📜️script-sources/🟦️.ts";
import { projectTrustedBootstrapCodecsV1 } from "../../🏗️bootstrap/🧾️provenance/🗃️bootstrap-bundle/🟦️.ts";
import { trustedBootstrapBrowserActorEncoding, trustedBootstrapBrowserActorV1, trustedBootstrapClosureEncoding, type TrustedBootstrapCodec, trustedBootstrapDependencies, trustedBootstrapDependencyEncoding, trustedBootstrapDescriptorClaims, trustedBootstrapIdentity, trustedBootstrapPlanGenerationOutcome, trustedBootstrapProfileEncoding, trustedBootstrapResolveClosure, trustedBootstrapResolveDependencies } from "../../🏗️bootstrap/🧾️provenance/🧮️bootstrap-claims/🟦️.ts";
import { proveTrustedRotationSourceFixture } from "../../🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🟦️.ts";
import { proveBrowserDocumentOpenFixture } from "../📄️browser-document-open/🟦️.ts";
import { proveExecutionTargetLeaseCorpus } from "../🔗️execution-target-lease-checks/🟦️.ts";
import { proveDocumentOpenPlanFixture } from "../🗺️open-plan-checks/🟦️.ts";
import { webSha256Hex } from "../../🚀️local-relay/🛰️servers/🟦️.ts";
import { moduleRustSource, readHubBinSource } from "../../🧪️testing/📖️source-readers/🟦️.ts";
import { proveTrustedBootstrapCodecCaptureFixture, proveTrustedGenerationStageFixture, proveTrustedPublicationFixture } from "../📮️trusted-publication-fixtures/🟦️.ts";

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

/** 🧬️ Independently validates the exact closed stdio+GIS profile and full-generation framing. */
async function proveDocumentBrowserActorIdentityFixture(repoRoot: string): Promise<void> {
  const integerRoot = join(repoRoot, "🧰️framework/🔨️modules/🌱️value/🔁️codec/🧫️fixtures");
  const integerFixture = JSON.parse(readFileSync(join(integerRoot, "🔣️.json"), "utf8"));
  let integerAccepted = 0;
  for (const target of integerFixture.targets) {
    if (target.name !== `${target.signed ? "i" : "u"}${target.bits}`) throw new Error("integer target policy mismatch");
    const limit = 1n << BigInt(target.bits - Number(target.signed));
    const minimum = target.signed ? -limit : 0n,
      maximum = limit - 1n;
    for (const raw of integerFixture.raw) {
      JSON.parse(raw);
      const integer = /^-?(?:0|[1-9][0-9]*)$/u.test(raw) ? BigInt(raw) : undefined;
      if (integer !== undefined && integer >= minimum && integer <= maximum) integerAccepted++;
    }
  }
  console.log(`exact-integer-value-oracle: targets=${integerFixture.targets.length} raw=${integerFixture.raw.length} admitted=${integerAccepted} arithmetic=BigInt; native production parity is a separate exact group`);
  const root = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌐️browser-actor");
  const fixture = JSON.parse(readFileSync(join(root, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
  const validators = {
    plan: hubSchemaExport(repoRoot, "schema://os.directory/DocumentBrowserActorPlan"),
    lease: hubSchemaExport(repoRoot, "schema://os.directory/DocumentBrowserActorLease"),
  };
  const contract = await import("@semio-tech/framework/directory/schema/browser-actor");
  const source = { componentSha256: fixture.componentSha256, descriptorByteSha256: fixture.descriptorByteSha256 };
  const parsers = { plan: contract.parseDocumentOpenBrowserActorV1, lease: contract.parseDocumentExecutionTargetBrowserActorV1 };
  for (const law of fixture.cases) {
    const candidate = law.kind === "none" ? { kind: "none" } : { ...structuredClone(fixture.closed), ...(law.view === "lease" ? { byteLength: fixture.byteLength } : {}) };
    Object.assign(candidate, law.set);
    if (law.remove) delete candidate[law.remove];
    const shape = Boolean(validators[law.view as "plan" | "lease"](candidate));
    const oracle =
      shape &&
      (candidate.kind === "none"
        ? ["react", "wgpu"].includes(law.renderer)
        : law.renderer === "wasm" &&
          candidate.sourceComponentSha256 === fixture.componentSha256 &&
          candidate.sourceDescriptorByteSha256 === fixture.descriptorByteSha256 &&
          JSON.stringify(candidate.importInterfaces) === JSON.stringify([...new Set(candidate.importInterfaces)].sort()));
    if (shape !== law.schemaAccepted || oracle !== law.accepted) throw new Error(`browser actor independent oracle mismatch: ${law.id}`);
    let parsed: ReturnType<typeof contract.parseDocumentOpenBrowserActorV1> | undefined;
    try {
      parsed = parsers[law.view as "plan" | "lease"](candidate, source, law.renderer);
    } catch {}
    if (Boolean(parsed) !== oracle) throw new Error(`browser actor production admission mismatch: ${law.id}`);
    if (parsed) {
      if (canonicalJson(parsed) !== canonicalJson(candidate) || !Object.isFrozen(parsed)) throw new Error(`browser actor projection mismatch: ${law.id}`);
      if (parsed.kind === "closed-browser-actor" && (!Object.isFrozen(parsed.importInterfaces) || parsed.importInterfaces === candidate.importInterfaces)) throw new Error(`browser actor borrowed mutable interfaces: ${law.id}`);
      if (!contract.sameDocumentBrowserActorV1(parsed, parsers[law.view as "plan" | "lease"](structuredClone(candidate), source, law.renderer))) throw new Error(`browser actor self equality mismatch: ${law.id}`);
    }
  }
  const plan = contract.parseDocumentOpenBrowserActorV1(fixture.closed, source, "wasm");
  const lease = contract.documentBrowserActorLeaseFromPlanV1(plan, source, "wasm", fixture.byteLength);
  if (canonicalJson(lease) !== canonicalJson({ ...fixture.closed, byteLength: fixture.byteLength })) throw new Error("browser actor lease projection mismatch");
  for (const field of Object.keys(lease)) {
    const changed = structuredClone(lease) as Record<string, any>;
    changed[field] = field === "importInterfaces" ? [] : field === "byteLength" ? fixture.byteLength + 1 : "different";
    if (contract.sameDocumentBrowserActorV1(lease, changed as any)) throw new Error(`browser actor equality omitted ${field}`);
  }
  if (contract.sameDocumentBrowserActorV1(plan, lease)) throw new Error("browser actor equality omitted lease length presence");
  const none = contract.parseDocumentOpenBrowserActorV1({ kind: "none" }, source, "react");
  if (!contract.sameDocumentBrowserActorV1(none, contract.documentBrowserActorLeaseFromPlanV1(none, source, "react", undefined))) throw new Error("browser actor none projection mismatch");
  let denied = 0;
  for (const [actor, length] of [
    [none, 1],
    [plan, undefined],
    [plan, 0],
    [plan, 67108865],
  ] as const) {
    try {
      contract.documentBrowserActorLeaseFromPlanV1(actor, source, actor.kind === "none" ? "react" : "wasm", length);
    } catch {
      denied++;
    }
  }
  let accessorReads = 0;
  for (const candidate of [
    undefined,
    null,
    [],
    Object.create(fixture.closed),
    {
      ...fixture.closed,
      get sha256() {
        accessorReads++;
        return fixture.closed.sha256;
      },
    },
  ]) {
    try {
      contract.parseDocumentOpenBrowserActorV1(candidate, source, "wasm");
    } catch {
      denied++;
    }
  }
  if (denied !== 9 || accessorReads !== 0) throw new Error("browser actor exact data ownership mismatch");
  for (const field of ["sourceComponentSha256", "sourceDescriptorByteSha256"]) {
    try {
      contract.documentBrowserActorLeaseFromPlanV1({ ...fixture.closed, [field]: "e".repeat(64) }, source, "wasm", fixture.byteLength);
    } catch {
      denied++;
    }
  }
  try {
    contract.documentBrowserActorLeaseFromPlanV1(plan, source, "react", fixture.byteLength);
  } catch {
    denied++;
  }
  try {
    contract.documentBrowserActorLeaseFromPlanV1(none, source, "wasm", undefined);
  } catch {
    denied++;
  }
  if (denied !== 13) throw new Error("browser actor projection bypassed package or renderer binding");
  console.log(`document-browser-actor-identity: AJV=2 cases=${fixture.cases.length} projection=2 equality=${Object.keys(lease).length + 1} ownership=9 projection-binding=4; metadata only, no catalog/activation claim`);
  await proveDocumentOpenPlanFixture(repoRoot);
  await proveBrowserDocumentOpenFixture(repoRoot);
  await proveExecutionTargetLeaseCorpus(repoRoot);
  await proveTrustedBrowserActorCatalogFixture(repoRoot);
}

/** 🧪️ Exercises the private catalog actor against the scope-owned closed actor contract. */
async function proveTrustedBrowserActorCatalogFixture(repoRoot: string): Promise<void> {
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  const root = join(repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog");
  const fixture = JSON.parse(readFileSync(join(root, "🧪️tests/🧫️fixtures/🌐️browser-actor/🔣️.json"), "utf8"));
  const validate = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedBundleBrowserActorV1");
  assert.deepEqual(Object.keys(fixture), ["schema", "bodyHex", "encodingSha256", "noneEncodingSha256", "closed", "rawLengths", "loadCases", "cases"]);
  assert.equal(fixture.schema, "semio.hub.trusted-browser-actor-fixture.v1");
  assert.match(fixture.bodyHex, /^(?:[0-9a-f]{2})+$/u);
  for (const key of ["encodingSha256", "noneEncodingSha256"] as const) assert.match(fixture[key], /^[0-9a-f]{64}$/u);
  assert(validate(fixture.closed), "closed catalog actor must satisfy its own scope-owned contract");
  assert.equal(fixture.cases.length, 26);
  assert.equal(fixture.loadCases.length, 8);
  assert.equal(fixture.rawLengths.length, 8);
  assert.equal(new Set(fixture.cases.map((row: any) => row.id)).size, fixture.cases.length);
  assert.equal(new Set(fixture.loadCases.map((row: any) => row.id)).size, fixture.loadCases.length);
  assert.equal(new Set(fixture.rawLengths.map((row: any) => row.token)).size, fixture.rawLengths.length);
  assert.deepEqual([...new Set(fixture.cases.map((row: any) => row.kind))].sort(), ["closed-browser-actor", "none"]);
  assert.deepEqual([...new Set(fixture.cases.map((row: any) => row.renderer))].sort(), ["react", "wasm", "wgpu"]);
  const source = { componentSha256: fixture.closed.sourceComponentSha256, descriptorByteSha256: fixture.closed.sourceDescriptorByteSha256 };
  for (const law of fixture.cases) {
    const actor = law.kind === "none" ? { kind: "none" } : structuredClone(fixture.closed);
    Object.assign(actor, law.set);
    if (law.remove) delete actor[law.remove];
    const shape = Boolean(validate(actor));
    assertObservationExpectation(law.id, law.expectation as ObservationExpectation, shape);
    const oracle =
      shape &&
      (actor.kind === "none"
        ? law.renderer !== "wasm"
        : law.renderer === "wasm" &&
          actor.sourceComponentSha256 === source.componentSha256 &&
          actor.sourceDescriptorByteSha256 === source.descriptorByteSha256 &&
          JSON.stringify(actor.importInterfaces) === JSON.stringify([...new Set(actor.importInterfaces)].sort()));
    assert.equal(oracle, law.accepted, `${law.id} oracle`);
    let encoded: Buffer | undefined;
    try {
      encoded = trustedBootstrapBrowserActorEncoding(actor, source, law.renderer);
    } catch {}
    assert.equal(Boolean(encoded), oracle, `${law.id} production`);
  }
  const bytes = Buffer.from(fixture.bodyHex, "hex");
  assert.equal(bytes.byteLength, fixture.closed.byteLength);
  assert.equal(await webSha256Hex(bytes), fixture.closed.sha256);
  for (const [actor, renderer, expected] of [
    [fixture.closed, "wasm", fixture.encodingSha256],
    [{ kind: "none" }, "react", fixture.noneEncodingSha256],
  ] as const) {
    const encoded = trustedBootstrapBrowserActorEncoding(actor, source, renderer);
    assert.equal(await webSha256Hex(encoded), expected);
    assert.equal(createHash("sha256").update(encoded).digest("hex"), expected);
  }
  for (const law of fixture.loadCases) {
    const body = law.bodyHex === null ? undefined : Buffer.from(law.bodyHex, "hex");
    const oracle = body !== undefined && body.byteLength === law.byteLength && body.byteLength > 0 && await webSha256Hex(body) === fixture.closed.sha256 && !law.cancelAfterDescriptor;
    assert.equal(oracle, law.accepted, `${law.id} body oracle`);
  }
  for (const law of fixture.rawLengths) {
    const actor = { ...fixture.closed, byteLength: JSON.parse(law.token) };
    assertObservationExpectation(`raw actor length ${law.token}`, law.expectation as ObservationExpectation, Boolean(validate(actor)));
    let accepted = false;
    try {
      trustedBootstrapBrowserActorV1(actor, source, "wasm");
      accepted = true;
    } catch {}
    assert.equal(accepted, law.accepted, `raw actor length ${law.token} production`);
  }
  console.log(`trusted-browser-actor-catalog: scope-export=TrustedBundleBrowserActorV1 cases=${fixture.cases.length} bodies=${fixture.loadCases.length} raw-lengths=${fixture.rawLengths.length} WebCrypto=1; metadata/framing oracle only, no native loader/activation claim`);
}

/** 🪢 Verifies independent kind coverage and the complete schema-qualified receipt roster. */
async function proveNativeCatalogReceiptPairs(repoRoot: string): Promise<void> {
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  const root = join(repoRoot, "🌎️hub/🧩️compositions/🗄️stdio/📇️catalog");
  const fixture = JSON.parse(readFileSync(join(root, "🧪️tests/🧫️fixtures/📇️native-catalog-surface/🧪️receipt-pairs.json"), "utf8"));
  const full = JSON.parse(readFileSync(join(root, "🧪️tests/🧫️fixtures/📇️native-catalog-surface/🧪️full-roster.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const ajv = new Ajv2020({ strict: true, allErrors: true });
  const triples = full.identities as string[][];
  const expectedTriples = new Set(triples.map((row) => JSON.stringify(row)));
  for (const row of full.cases) {
    const hostile = structuredClone(full);
    switch (row.mutation) {
      case "none": break;
      case "reverse": hostile.identities.reverse(); break;
      case "missing": hostile.identities.splice(17, 1); break;
      case "duplicate": hostile.identities[17] = [...hostile.identities[16]]; break;
      case "extra": hostile.identities.push(["s.stdio.pdf", "foreign.schema", "foreign.factory"]); break;
      case "schema": hostile.identities[17][1] = "foreign.schema"; break;
      case "factory": hostile.identities[17][2] = "foreign.factory"; break;
      default: throw new Error(`unknown full catalog mutation ${row.mutation}`);
    }
    const keys = new Set(hostile.identities.map((identity: string[]) => JSON.stringify(identity)));
    const accepted = keys.size === 30 && hostile.identities.length === 30 && [...keys].every((key) => expectedTriples.has(key as string));
    assert.equal(accepted, row.accepted, `${row.id} full owned identity roster`);

  }
  const FullDatabase = (await import("bun:sqlite")).Database;
  const fullCounts = new FullDatabase(":memory:");
  try {
    fullCounts.exec("CREATE TABLE codec_identity(kind TEXT NOT NULL, schema TEXT NOT NULL, factory TEXT NOT NULL)");
    const insert = fullCounts.prepare("INSERT INTO codec_identity VALUES (?, ?, ?)");
    for (const identity of triples) insert.run(identity[0]!, identity[1]!, identity[2]!);
    assert.deepEqual(fullCounts.query("SELECT count(*) AS rows, count(DISTINCT kind) AS kinds, count(DISTINCT factory) AS factories FROM codec_identity").get(), { rows: 30, kinds: 29, factories: 30 });
    assert.deepEqual(fullCounts.query("SELECT count(*) AS pairs FROM (SELECT DISTINCT kind, schema FROM codec_identity)").get(), { pairs: 30 });
  } finally {
    fullCounts.close();
  }
  const selection = readFileSync(join(root, "../🤖️generated/🧩️composition/🦀️.rs"), "utf8").split("\n").find((line) => line.startsWith("fn selected_contributions()"))!;
  const owners = new Set([...selection.matchAll(/semio_s_artifact_stdio_(\w+)::contribution/g)].map((match) => match[1]!));
  assert.equal(owners.size, full.artifactAssemblies);
  const artifacts = join(repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts");
  const actualTriples: string[][] = [];
  const ledger = { declared: { codecs: 0, mutations: 0, inferences: 0 }, registered: { codecs: 0, mutations: 0, inferences: 0 }, implemented: { codecs: 0, mutations: 0, inferences: 0 }, verified: { codecs: 0, mutations: 0, inferences: 0 } };
  let definitions = 0;
  for (const directory of readdirSync(artifacts)) {
    const path = join(artifacts, directory, "📜️artifact-definition.json");
    if (!existsSync(path)) continue;
    const definition = JSON.parse(readFileSync(path, "utf8"));
    if (!owners.has(definition.artifact)) continue;
    definitions++;
    for (const category of ["codecs", "mutations", "inferences"] as const) {
      for (const capability of definition[category]) {
        ledger.declared[category]++;
        if (capability.executable_registration) ledger.registered[category]++;
        if (capability.status === "implemented") ledger.implemented[category]++;
        if (capability.status === "verified") ledger.verified[category]++;
        if (category === "codecs" && capability.native_factory) {
          const binding = capability.native_factory;
          actualTriples.push([binding.artifact_kind, binding.artifact_schema, binding.factory_id]);
        }
      }
    }
  }
  assert.equal(definitions, full.artifactAssemblies);
  assert.deepEqual(ledger, full.ledger, "actual authored capability declarations retain exact independent ledger");
  assert.deepEqual([...actualTriples].sort(), [...triples].sort(), "actual selected definition factory identities equal the closed full roster");
  console.log(`native-catalog-full-roster: schemas=30 kinds=29 factories=30 declared=36 cases=${full.cases.length} independent-sqlite=30/29/30`);
  const nativeLaw = readFileSync(join(root, "../🔌️plugin/📇️catalog/🧪️tests/🔬️unit/🦀️.rs"), "utf8");
  assert(nativeLaw.includes('fixture["identities"]') && nativeLaw.includes('fixture["artifactKinds"]'), "original native count law must retain exact schema-qualified roster and unique kinds");
  const expected = new Set(fixture.pairs.map((pair: string[]) => JSON.stringify(pair)));
  const { Database } = await import("bun:sqlite");
  const counts = new Database(":memory:");
  try {
    counts.exec("CREATE TABLE codec_identity(kind TEXT NOT NULL, schema TEXT NOT NULL)");
    const insert = counts.prepare("INSERT INTO codec_identity VALUES (?, ?)");
    for (const pair of fixture.pairs) insert.run(pair[0], pair[1]);
    const independent = counts.query("SELECT (SELECT count(*) FROM (SELECT DISTINCT kind, schema FROM codec_identity)) AS codecIdentities, count(DISTINCT kind) AS artifactKinds FROM codec_identity").get();
    assert.deepEqual(independent, fixture.identityCounts, "independent SQLite identity projections");
    assert.equal(expected.size, fixture.identityCounts.codecIdentities, "owned schema-qualified identity count");
    assert.equal(new Set(fixture.pairs.map((pair: string[]) => pair[0])).size, fixture.identityCounts.artifactKinds, "owned artifact-kind count");
  } finally {
    counts.close();
  }
  for (const row of fixture.cases) {
    const pairs: string[][] = fixture.pairs.map((pair: string[]) => [...pair]);
    switch (row.mutation) {
      case "none": break;
      case "reverse": pairs.reverse(); break;
      case "missing-schema": pairs.splice(1, 1); break;
      case "missing-kind": pairs.pop(); break;
      case "duplicate": pairs[1] = [...pairs[0]!]; break;
      case "foreign-schema": pairs[1]![1] = "foreign.schema"; break;
      case "foreign-kind": pairs[1]![0] = "foreign.artifact"; break;
      case "extra": pairs.push(["s.stdio.pdf", "foreign.schema"]); break;
      default: throw new Error(`unknown receipt pair mutation ${row.mutation}`);
    }
    const keys = new Set(pairs.map((pair) => JSON.stringify(pair)));
    const kinds = [...new Set(pairs.map((pair) => pair[0]!))].sort();
    const accepted = keys.size === pairs.length && keys.size === expected.size && [...keys].every((key) => expected.has(key)) && JSON.stringify(kinds) === JSON.stringify([...fixture.kinds].sort());
    assert.equal(accepted, row.accepted, `${row.id} owned pair roster`);
  }
  console.log(`native-catalog-receipt-pairs: cases=${fixture.cases.length} schemas=${fixture.pairs.length} kinds=${fixture.kinds.length} independent-sqlite-identities=${fixture.identityCounts.codecIdentities}/${fixture.identityCounts.artifactKinds}`);
}

async function proveTrustedCatalogOpenedRootFixture(repoRoot: string): Promise<void> {
  await proveNativeCatalogReceiptPairs(repoRoot);
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  const root = join(repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog");
  const fixtureRoot = join(root, "🧪️tests/🧫️fixtures/🛡️opened-root");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
  const validatePointer = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedCatalogCurrentPointerV1");
  const validateRelativePath = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedCatalogRelativePathV1");
  assert.deepEqual(Object.keys(fixture), ["schema", "limits", "current", "relativePaths", "denials", "openedHandle"]);
  assert.equal(fixture.schema, "semio.hub.trusted-catalog-opened-root/v1");
  assert.deepEqual(fixture.limits, { pathBytes: 1024, pathSegments: 64, chunkBytes: 65536, pointerBytes: 65536 });
  assert(validatePointer(fixture.current), "opened-root current pointer must satisfy its scope-owned contract");
  assert.deepEqual(Object.keys(fixture.openedHandle), ["initialHex", "replacementHex", "maximum", "expectedHex", "providerCalls", "codecPublished"]);
  for (const key of ["initialHex", "replacementHex", "expectedHex"] as const) assert.match(fixture.openedHandle[key], /^(?:[0-9a-f]{2})+$/u);
  assert(Number.isInteger(fixture.openedHandle.maximum) && fixture.openedHandle.maximum >= 1 && fixture.openedHandle.maximum <= 67_108_864);
  assert.equal(fixture.openedHandle.providerCalls, 0);
  assert.equal(fixture.openedHandle.codecPublished, false);
  for (const row of fixture.relativePaths) assert.equal(validateRelativePath(row.value), row.accepted, `${row.id} contract`);
  const acceptedPath = (value: string): boolean => {
    const bytes = new TextEncoder().encode(value).byteLength;
    const segments = value.split("/");
    return (
      bytes > 0 &&
      bytes <= fixture.limits.pathBytes &&
      segments.length <= fixture.limits.pathSegments &&
      !value.includes("\\") &&
      !value.includes("\0") &&
      !value.startsWith("/") &&
      segments.every((segment: string) => segment.length > 0 && segment !== "." && segment !== "..")
    );
  };
  assert.equal(new Set(fixture.relativePaths.map((row: any) => row.id)).size, fixture.relativePaths.length);
  assert(fixture.relativePaths.length >= 12 && fixture.relativePaths.length <= 32);
  for (const row of fixture.relativePaths) assert.equal(acceptedPath(row.value), row.accepted, row.id);
  const pointer = Buffer.from(`${JSON.stringify(fixture.current)}\n`, "utf8");
  assert(pointer.byteLength > 0 && pointer.byteLength <= fixture.limits.pointerBytes);
  assert.deepEqual(JSON.parse(pointer.toString("utf8")), fixture.current);
  assert.equal(Buffer.from(fixture.openedHandle.initialHex, "hex").toString("hex"), fixture.openedHandle.expectedHex);
  assert.notEqual(fixture.openedHandle.initialHex, fixture.openedHandle.replacementHex);
  assert.deepEqual(
    fixture.denials.map((row: any) => row.kind),
    ["leaf-link", "intermediate-link", "actor-link", "data-root-link", "current-link", "generation-link", "reparse-point"],
  );
  assert(fixture.denials.every((row: any) => row.providerCalls === 0 && row.codecPublished === false));
  const source = moduleRustSource(root);
  const startup = readHubBinSource(repoRoot);
  const required = [
    "TrustedCatalogDataRoot",
    "TrustedCatalogGenerationRoot",
    "TrustedCatalogRelativePathV1",
    "TrustedCatalogOpenedFile",
    "open_server_owned",
    "open_current",
    "open_regular",
    "read_regular",
    "O_NOFOLLOW",
    "O_NONBLOCK",
    "fstat",
    "trusted_catalog_opened_root_rejects_linked_roots_leaves_intermediates_and_actors",
    "trusted_catalog_opened_handle_is_swap_stable_bounded_and_cancel_safe",
    "trusted_catalog_relative_paths_match_the_neutral_no_link_corpus",
  ];
  assert(
    required.every((needle) => source.includes(needle)),
    "trusted catalog lacks the opened-root/no-link owner or exact laws",
  );
  assert(startup.includes("configured_artifact_authority(&data_dir") && !startup.includes("OS_HUB_TRUSTED_CATALOG_BUNDLE") && !startup.includes("OS_HUB_TRUSTED_CATALOG_PROFILE"), "Hub startup still accepts ambient trusted bundle/profile paths");
  console.log(`trusted-catalog-opened-root: scope-exports=2 paths=${fixture.relativePaths.length} denials=${fixture.denials.length} same-handle=1 source=${required.length} startup=no-ambient-path; source contract only, native platform laws require --native`);
}

/** 🌐️ Binds a browser proof to one retained trusted generation rather than a second build. */
async function proveTrustedGisRetainedBrowserFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🧬️retained-gis-browser");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  assert.deepEqual(Object.keys(fixture), ["schema", "bootstrapFixture", "contract", "cases", "nonclaims"]);
  assert.equal(fixture.schema, "semio.hub.retained-gis-browser/v1");
  assert.equal(fixture.bootstrapFixture, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🧬️stdio-gis-bootstrap/🔣️.json");
  assert.deepEqual(fixture.contract, {
    generation: "published-current-exact",
    actorPath: "packages/gis/browser/closed-actor.mjs",
    descriptorPath: "packages/gis/descriptor.semio",
    componentPath: "packages/gis/component.wasm",
    actor: "retained-not-rebuilt",
    browser: "chromium-dedicated-child",
    result: "normalized-describe-equals-retained-descriptor",
    terminalActors: 0,
    terminalBytes: 0,
  });
  assert.deepEqual(fixture.nonclaims, ["authenticated-worker", "cold-pair-applied", "shell-renderer", "map-mutation", "two-peer-collaboration"]);
  assert.equal(new Set(fixture.nonclaims).size, fixture.nonclaims.length);
  const bootstrap = JSON.parse(readFileSync(join(repoRoot, fixture.bootstrapFixture), "utf8"));
  const gis = bootstrap.profile.packages.find((row: any) => row.pluginId === "gis");
  const expected = { generation: bootstrap.profile.generationId, component: gis.componentSha256, descriptor: gis.descriptorSha256, actor: gis.browserActor.sha256, parent: gis.browserActor.sourceComponentSha256 };
  const equal = (await import("fast-deep-equal")).default;
  assert.equal(createHash("sha256").update("abc").digest("hex"), expected.actor, "neutral actor identity must be independently reproducible");
  const digest = Buffer.from(await webcrypto.subtle.digest("SHA-256", new TextEncoder().encode("abc"))).toString("hex");
  assert.equal(digest, expected.actor);
  assert.deepEqual(fixture.cases.map((row: any) => row.name), ["exact", "replaced-component", "replaced-descriptor", "replaced-actor", "changed-generation", "changed-parent-digest"]);
  const mutations: Record<string, keyof typeof expected> = { "replaced-component": "component", "replaced-descriptor": "descriptor", "replaced-actor": "actor", "changed-generation": "generation", "changed-parent-digest": "parent" };
  for (const row of fixture.cases) {
    const value = { ...expected };
    const field = mutations[row.name];
    if (field) value[field] = "ff".repeat(32);
    assert.equal(equal(value, expected), row.accepted, row.name);
  }
  const source = readHubScriptModule(repoRoot, "🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication");
  const start = source.indexOf("\nasync function proveTrustedGisClosedActorV1(");
  const end = source.indexOf("\n/** 🧳️ Copies one already verified immutable selection", start);
  assert(start >= 0 && end > start, "retained GIS browser proof must exist before its native mode can qualify it");
  const selected = source.slice(start, end);
  const conforms = (value: string): boolean => value.includes("trustedBootstrapCurrent(dataRoot)")
    && value.includes("published.currentSha256 !== initialCurrent.currentSha256")
    && value.includes("trustedBootstrapVerifyGeneration(")
    && value.includes('"packages/gis/browser/closed-actor.mjs"')
    && value.includes("gis.browserActor.sourceComponentSha256 !== gis.component.sha256")
    && value.includes("gis.browserActor.sourceDescriptorByteSha256 !== gis.descriptor.sha256")
    && value.includes("await proveBrowserActorGisChildV1(")
    && value.split("verifyRetained();").length === 3
    && !value.includes("produceFreshComponentV1(")
    && !value.includes("buildClosedBrowserActorArtifactV1(");
  assert(conforms(selected), "retained browser proof must verify current and exact generation bytes on both sides of the genuine child");
  const hostiles = [
    selected.replaceAll("trustedBootstrapCurrent(dataRoot)", "undefined"),
    selected.replace("published.currentSha256 !== initialCurrent.currentSha256", "false"),
    selected.replaceAll("trustedBootstrapVerifyGeneration(", "omittedGenerationCheck("),
    selected.replaceAll('"packages/gis/browser/closed-actor.mjs"', '"caller-selected.mjs"'),
    selected.replace("gis.browserActor.sourceComponentSha256 !== gis.component.sha256", "false"),
    selected.replace("gis.browserActor.sourceDescriptorByteSha256 !== gis.descriptor.sha256", "false"),
    selected.replace("await proveBrowserActorGisChildV1(", "void proveBrowserActorGisChildV1("),
    selected.replace("verifyRetained();", ""),
  ];
  for (const hostile of hostiles) assert(hostile !== selected && !conforms(hostile), "retained browser hostile must fail");
  const gateStart = source.indexOf("\nclass TrustedStdioGisBundleCheckScript");
  const gate = source.slice(gateStart, source.indexOf("\nclass TrustedCatalogBootstrapScript", gateStart));
  const mode = gate.lastIndexOf('if (segments[0] === "--browser" || segments[0] === "--two-author-shell")');
  assert(mode >= 0 && gate.indexOf("const initialPlan = await validateAndPublishTrustedStdioGisCandidate") < mode);
  assert(gate.slice(mode).includes("await proveTrustedGisClosedActorV1(this.repoRoot, dataRoot, receipt, artifactPath)"));
  console.log("trusted-retained-gis-browser: pinned-tables=3 deep-equal=" + fixture.cases.length + " SHA256=node+webcrypto source-hostiles=" + hostiles.length + "; neutral bytes only, no browser execution claim");
}

/** 🤝️ Pins the genuine two-peer acceptance specification without claiming any observed runtime trace. */
async function proveTrustedGisMapCollaborationContractFixture(repoRoot: string): Promise<void> {
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  const equal = (await import("fast-deep-equal")).default;
  const root = join(repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🤝️gis-map-collaboration");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const pinned = {
    schema: "semio.hub.gis-map-collaboration/v1",
    prerequisites: {
      bootstrapFixture: "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🧬️stdio-gis-bootstrap/🔣️.json",
      browserFixture: "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🧬️retained-gis-browser/🔣️.json",
      approvalFixture: "🌎️hub/🧪️tests/🧫️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json",
    },
    selection: {
      profileId: "local-stdio-gis-open-v1",
      packageId: "semio:gis",
      artifactKind: "s.gis.gismap",
      artifactSchema: "gis.map",
      parentDialect: "s.gis.gismap@1/*",
      surfaceId: "s.gis.gismap@1/*#editor",
      grantedMode: "read-write-observe",
      serviceId: "s.gis.gismap.inference",
      generation: "same-published-current-through-teardown",
      actor: "retained-not-rebuilt",
    },
    peers: [
      { name: "author-a", role: "author", language: "en" },
      { name: "author-b", role: "author", language: "de" },
    ],
    authority: {
      sessions: "distinct-authenticated-react-relay-profiles",
      capabilities: "host-only-never-page-worker-or-report",
      createDocument: "production-directory-command",
      initialCheckpoint: "hub-materialized-check-in",
      open: "production-plan-asset-grant-session",
      pair: "actual-terminal-pack-spr-receipt",
      load: "genuine-plugin-load-document-pack",
      scene: "mounted-shell-tiled-map-host",
      sceneAck: "scope-client-surface-session-bound-ui-owner",
      concurrency: "both-sockets-live-before-mutation",
    },
    lifecycle: [
      "publish-qualified-generation", "create-shared-document", "publish-initial-checkpoint", "open-two-authenticated-shells",
      "observe-two-initial-gis-scenes", "cancel-separate-owner-job", "deny-peer-access-to-owner-private-job",
      "observe-server-stamped-create-region-proposal", "approve-owner-proposal", "observe-committed-wal-event",
      "observe-actor-frontier", "observe-public-checkpoint", "observe-ledger-applied", "observe-two-live-rebootstrap-frames",
      "observe-two-fresh-plan-grant-sessions", "observe-two-later-cold-pairs", "observe-two-equal-region-scenes",
      "undo-via-ordinary-document-command", "observe-two-equal-undone-scenes", "close-all-owners", "verify-unchanged-catalog-generation",
    ],
    equalBindings: [
      "profileId", "catalogGenerationId", "packageId", "componentSha256", "descriptorByteSha256", "browserActorSha256",
      "browserActorByteLength", "documentId", "surfaceId", "parentDialect", "coldPairManifestHash", "frontier", "visibleMapScene",
    ],
    refreshedBindings: ["sessionId", "socketGrant", "coldPairReceiptGeneration"],
    approval: {
      command: "CreateRegion",
      regionId: "inference-<owner-job-id>",
      kind: "inference-bounds",
      inverse: "DeleteRegion",
      effectCount: 1,
      peerMaySubmitOwnJob: true,
      peerMayReadOwnerJob: false,
      peerMayCancelOwnerJob: false,
      peerMayApproveOwnerJob: false,
    },
    teardown: { pages: 0, workers: 0, relays: 0, childActors: 0, childBytes: 0, retainedDocumentOwners: 0, hubProcesses: 0 },
    forbiddenSubstitutes: [
      "sequential-peers", "controlled-actor", "independently-rebuilt-actor", "direct-artifact-store-write",
      "direct-ui-document-store-patch", "injected-rebootstrap-frame", "patch-positions-as-ai-approval", "raw-capability-in-page",
    ],
    nonclaims: ["all-plugin-assets-qualified", "external-model-provider-qualified", "postgres-live-qualified", "neo4j-live-qualified"],
  };
  const validate = (candidate: unknown): boolean => equal(candidate, pinned) && JSON.stringify(Object.keys(candidate as object)) === JSON.stringify(Object.keys(pinned));
  assert(validate(fixture), "collaboration contract drifted from its pinned specification");
  const approval = JSON.parse(readFileSync(join(repoRoot, fixture.prerequisites.approvalFixture), "utf8"));
  const browser = JSON.parse(readFileSync(join(repoRoot, fixture.prerequisites.browserFixture), "utf8"));
  for (const key of ["packageId", "artifactKind", "artifactSchema", "surfaceId", "grantedMode", "serviceId"]) {
    assert.equal(fixture.selection[key], approval.binding[key]);
    assert(equal(fixture.selection[key], approval.binding[key]));
  }
  assert.equal(fixture.selection.actor, browser.contract.actor);
  assert(equal(fixture.lifecycle.filter((phase: string) => ["observe-committed-wal-event", "observe-actor-frontier", "observe-public-checkpoint", "observe-ledger-applied"].includes(phase)).map((phase: string) => phase.slice(8)), approval.committer.publicationOrder));
  const hostiles = [
    { ...fixture, authority: { ...fixture.authority, concurrency: "sequential" } },
    { ...fixture, selection: { ...fixture.selection, actor: "rebuilt" } },
    { ...fixture, approval: { ...fixture.approval, command: "patchPositions" } },
    { ...fixture, approval: { ...fixture.approval, peerMaySubmitOwnJob: false } },
    { ...fixture, approval: { ...fixture.approval, peerMayReadOwnerJob: true } },
    { ...fixture, lifecycle: fixture.lifecycle.filter((phase: string) => !phase.includes("undo")) },
    { ...fixture, peers: fixture.peers.slice(0, 1) },
    { ...fixture, teardown: { ...fixture.teardown, childBytes: 1 } },
  ];
  for (const hostile of hostiles) assert(!validate(hostile), "collaboration contract hostile must fail");
  console.log("trusted-gis-collaboration-contract: pinned-specification=1 hostile=" + hostiles.length + " cross-fixture-equality=8; specification only, no observed collaboration trace");
}

/** 🛂️ Checks one mandatory exact-generation proof before every new current publication. */
async function proveTrustedGisPublicationFixture(repoRoot: string, fixture: Record<string, any>): Promise<void> {
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  assert.deepEqual(fixture.entryPoints, ["development", "bootstrap", "native", "process-rotation"]);
  assert.deepEqual(fixture.steps, ["materialized", "exact-gis-cold-proof", "candidate-validated", "current-published"]);
  assert.deepEqual(fixture.cases.map((row: any) => row.name), ["exact", "omitted", "late", "wrong-generation", "proof-failed"]);
  const { Database } = await import("bun:sqlite");
  const oracle = new Database(":memory:");
  try {
    oracle.run("CREATE TABLE qualification (phase INTEGER, generation TEXT, current TEXT)");
    for (const entry of fixture.entryPoints) {
      for (const row of fixture.cases) {
        oracle.run("DELETE FROM qualification");
        oracle.run("INSERT INTO qualification VALUES (0, 'candidate', 'previous')");
        for (const step of row.steps) {
          const index = fixture.steps.indexOf(step);
          const allowed = step !== "exact-gis-cold-proof" || (row.proofSucceeds && row.proofGeneration === "same");
          const changed = allowed ? oracle.run("UPDATE qualification SET phase = phase + 1 WHERE phase = ?1", [index]).changes : 0;
          if (changed !== 1) break;
          if (step === "current-published") oracle.run("UPDATE qualification SET current = generation WHERE phase = 4");
        }
        const result = oracle.query("SELECT phase, current FROM qualification").get() as { phase: number; current: string };
        assert.equal(result.phase === 4, row.published, entry + ":" + row.name);
        assert.equal(result.current, row.retained, entry + ":" + row.name);
      }
    }
  } finally {
    oracle.close();
  }
  const source = readHubScriptModule(repoRoot, "🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication");
  const body = (start: string, end: string): string => {
    const first = source.indexOf(start), last = source.indexOf(end, first);
    assert(first >= 0 && last > first, start);
    return source.slice(first, last);
  };
  const candidate = body("\nasync function validateAndPublishTrustedStdioGisCandidate", "\n}\n");
  const proof="await proveTrustedGisColdMapComponentV1(invocation,control,repoRoot,receipt);";
  const stage="stageTrustedBootstrapCandidateCurrent(candidateContext.dataRoot,receipt,()=>{checkResourceLeaseControl(candidateContext.control);});";
  const conforms = (value: string): boolean => value.includes(proof)
    && value.indexOf(proof) < value.indexOf(stage)
    && value.indexOf(proof)<value.indexOf("await startLocalHub(")
    && value.includes("withLocalHubHostCandidateV1(host,control")
    && value.includes("startLocalHub(candidateHost,[profile],{capture:true})")
    && !value.slice(0, value.indexOf(stage)).includes("catch");
  assert(conforms(candidate), "every trusted candidate must prove its exact GIS generation before staging or startup");
  const hostiles = [
    candidate.replace(proof, ""),
    candidate.replace(proof, "").replace(stage, stage + "\n  " + proof),
    candidate.replace(proof, proof.replace("repoRoot,receipt", "repoRoot,{...receipt,generationId:'different'}")),
    candidate.replace(proof, proof.replace("await ", "void ")),
    candidate.replace(proof, proof.slice(0, -1) + ".catch(() => undefined);"),
  ];
  for (const hostile of hostiles) assert(hostile !== candidate && !conforms(hostile), "publication source admitted missing, late, substituted, detached or swallowed proof");
  const development = body("\nclass DevScript", "\nclass SecureLocalSmokeScript");
  const bootstrap = body("\nclass TrustedCatalogBootstrapScript", "\nclass AdminBackendCheckScript");
  const native = body("\nclass TrustedStdioGisBundleCheckScript", "\nclass TrustedCatalogBootstrapScript");
  for (const [name, caller] of [["development", development], ["bootstrap", bootstrap]] as const) {
    assert(caller.includes("await validateAndPublishTrustedStdioGisCandidate(host,actualPhase,receipt)"), name + " must join mandatory proof");
    assert(!caller.includes("publishTrustedBootstrapCurrent("), name + " cannot publish directly");
  }
  assert(native.includes("await validateAndPublishTrustedStdioGisCandidate(host,actualPhase,receipt)"), "native must join mandatory proof");
  assert(native.includes("await validateAndPublishTrustedStdioGisCandidate(host,actualPhase,rotated,initialPlan)"), "rotation must join mandatory proof");
  console.log("trusted-gis-publication: AJV=1 sqlite-traces=" + fixture.entryPoints.length * fixture.cases.length + " source-hostiles=" + hostiles.length + "; no native component or browser claim");
}

export async function proveTrustedStdioGisBootstrapFixture(invocation: ScriptInvocation, repoRoot: string): Promise<void> {
  await proveTrustedCompiledDependenciesFixture(repoRoot);
  await proveDocumentBrowserActorIdentityFixture(repoRoot);
  const { createFreshComponentTests } = await import("../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧪️tests/🆕️fresh-component/🟦️.ts");
  const { testFreshComponentStagingV1, testFreshComponentSourceEpochV1, testFreshComponentProcessV1 } = createFreshComponentTests(invocation);
  await testFreshComponentStagingV1(repoRoot);
  await testFreshComponentSourceEpochV1(repoRoot);
  await testFreshComponentProcessV1(repoRoot);
  await proveTrustedGenerationStageFixture(repoRoot);
  await proveTrustedPublicationFixture(repoRoot);
  await proveTrustedBootstrapCodecCaptureFixture(repoRoot);
  const root = join(repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🧬️stdio-gis-bootstrap");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const bootstrapAssert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  const validateIdentity = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedBundleIdentityV1");
  const validateActor = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedBundleBrowserActorV1");
  const validateProtocol = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedBundleExecutionProtocolV1");
  const validateDialect = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedBundleParentDialectV1");
  const validateGrant = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedBundleGrantV1");
  bootstrapAssert.deepEqual(Object.keys(fixture), ["schema", "profile", "sources", "limits", "framing", "cancellationStages", "hostile", "publication", "rotation"]);
  bootstrapAssert.equal(fixture.schema, "semio.hub.trusted-stdio-gis-bootstrap/v1");
  bootstrapAssert.equal(fixture.profile.id, "local-stdio-gis-open-v1");
  bootstrapAssert.deepEqual(fixture.limits, { descriptorBytes: 4_194_304, descriptorBytesPlusOne: 4_194_305, componentBytes: DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, ioChunkBytes: 65_536, packageCount: 2, codecCount: fixture.profile.packages.reduce((total: number, row: { codecCount: number }) => total + row.codecCount, 0), openTargetCount: 1 });
  bootstrapAssert.deepEqual(fixture.framing, { closureDomain: "semio/hub/trusted-profile-selected-closure/v1\u0000", generationDomain: "semio/hub/trusted-profile-generation/v2\u0000", integerEndian: "big", lengthBytes: 8 });
  bootstrapAssert.deepEqual(fixture.cancellationStages, ["build", "inspect-wit", "emit-descriptor", "stage-component", "stage-descriptor", "hash", "pre-publication"]);
  bootstrapAssert.deepEqual(fixture.hostile, [
    "stale-stdio-component", "stale-stdio-descriptor", "stale-stdio-codec", "wrong-stdio-version", "wrong-gis-version", "missing-map", "missing-terrain",
    "duplicate-terrain", "terrain-target", "non-wasm", "readonly-editor", "mixed-package", "closure-order", "path-escape", "path-collision",
    "descriptor-max-plus-one", "cancel-before-publication", "candidate-crash", "stale-plan-generation",
  ]);
  bootstrapAssert.deepEqual(Object.keys(fixture.sources), ["stdioReceipts", "gisReceipts"]);
  bootstrapAssert.deepEqual(fixture.rotation.sourceCases.map((row: any) => row.change), ["none", "replaced-bundle", "wrong-generation", "wrong-profile", "leaf-replaced-during-read", "cancel-before-read"]);
  bootstrapAssert.equal(fixture.rotation.writeCases.length, 7);
  bootstrapAssert.equal(new Set(fixture.rotation.writeCases.map((row: any) => row.id)).size, 7);
  for (const row of fixture.rotation.writeCases) bootstrapAssert(["new", "old", "absent"].includes(row.retained) && Number.isInteger(row.byteLength) && row.byteLength >= 1 && row.byteLength <= 67_108_864, row.id);
  for (const identity of fixture.profile.selectedClosure) bootstrapAssert(validateIdentity(identity), "bootstrap closure identity must satisfy the scope-owned contract");
  const validateReceipt = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedBundleReceiptV1");
  for (const record of fixture.profile.packages) {
    bootstrapAssert(validateReceipt(record.receipt) && componentReceiptBindsV1(componentReceiptValueV1(record.receipt), { profile: "release", pluginId: record.pluginId, packageId: record.packageId, wasmSha256: record.componentSha256, semanticDescriptorSha256: record.descriptorSha256 }), `${record.pluginId} receipt must be a shipped receipt binding its component to its descriptor`);
    bootstrapAssert(validateActor(record.browserActor), `${record.pluginId} browser actor must satisfy the scope-owned contract`);
    bootstrapAssert(validateProtocol(record.executionProtocol), `${record.pluginId} execution protocol must satisfy the scope-owned contract`);
    for (const dependency of record.dependencies) bootstrapAssert(validateIdentity(dependency), `${record.pluginId} dependency must satisfy the scope-owned contract`);
  }
  bootstrapAssert.equal(fixture.profile.openTargets.length, fixture.limits.openTargetCount);
  for (const declared of fixture.profile.openTargets) bootstrapAssert(validateDialect(declared.parentDialect) && validateGrant(declared.grant), "bootstrap open target dialect/grant must satisfy the scope-owned contract");
  await proveTrustedGisPublicationFixture(repoRoot, fixture.publication);
  await proveTrustedGisRetainedBrowserFixture(repoRoot);
  await proveTrustedGisMapCollaborationContractFixture(repoRoot);
  const stdio = JSON.parse(readFileSync(join(repoRoot, fixture.sources.stdioReceipts), "utf8"));
  const gis = JSON.parse(readFileSync(join(repoRoot, fixture.sources.gisReceipts), "utf8"));
  const codecs = projectTrustedBootstrapCodecsV1(stdio, gis).codecs;
  const profile = fixture.profile;
  const unique = (rows: readonly TrustedBootstrapCodec[]): boolean =>
    rows.length === new Set(rows.map((row) => JSON.stringify([row.artifactKind, row.artifactSchema, row.packSchemaHash]))).size && rows.every((row) => /^(?!0{64}$)[0-9a-f]{64}$/u.test(row.packSchemaHash));
  if (stdio.plugin_id !== "stdio" || stdio.package_id !== "semio:stdio" || codecs.stdio.length !== profile.packages[1]?.codecCount || !unique(codecs.stdio)) throw new Error(`stdio bootstrap closure is not the profile's exact ${profile.packages[1]?.codecCount} codecs`);
  if (
    gis.pluginId !== "gis" ||
    gis.packageId !== "semio:gis" ||
    gis.packageVersion !== profile.selectedClosure[0].version ||
    codecs.gis.length !== 2 ||
    !unique(codecs.gis) ||
    !codecs.gis.some((row) => row.artifactKind === "s.gis.gismap") ||
    !codecs.gis.some((row) => row.artifactKind === "s.gis.gisterrain")
  )
    throw new Error("GIS bootstrap closure is not exact Map plus Terrain");
  if (
    profile.packages.length !== 2 ||
    profile.packages[0].pluginId !== "gis" ||
    profile.packages[0].codecCount !== 2 ||
    profile.packages[0].targetCount !== 1 ||
    profile.packages[1].pluginId !== "stdio" ||
    profile.packages[1].codecCount !== codecs.stdio.length ||
    profile.packages[1].targetCount !== 0
  )
    throw new Error("trusted bootstrap package/target cardinality drifted");
  const closure = trustedBootstrapClosureEncoding(profile);
  const closureNode = createHash("sha256").update(closure).digest("hex");
  const closureWeb = await webSha256Hex(closure);
  if (closureNode !== profile.selectedClosureSha256 || closureWeb !== closureNode) throw new Error("trusted bootstrap selected closure digest mismatch");
  const generationBytes = trustedBootstrapProfileEncoding(profile, codecs);
  const generationNode = createHash("sha256").update(generationBytes).digest("hex");
  const generationWeb = await webSha256Hex(generationBytes);
  if (generationNode !== profile.generationId || generationWeb !== generationNode) throw new Error(`trusted bootstrap full generation mismatch: ${generationNode}`);
  const changed = structuredClone(profile);
  changed.packages[1].componentSha256 = "31".repeat(32);
  const changedComponent = createHash("sha256").update(trustedBootstrapProfileEncoding(changed, codecs)).digest("hex");
  changed.packages[1].componentSha256 = profile.packages[1].componentSha256;
  changed.packages[1].descriptorSha256 = "32".repeat(32);
  const changedDescriptor = createHash("sha256").update(trustedBootstrapProfileEncoding(changed, codecs)).digest("hex");
  const changedCodecs = { ...codecs, stdio: codecs.stdio.map((row, index) => (index === 0 ? { ...row, packSchemaHash: "33".repeat(32) } : row)) };
  const changedCodec = createHash("sha256").update(trustedBootstrapProfileEncoding(profile, changedCodecs)).digest("hex");
  changed.packages[1].descriptorSha256 = profile.packages[1].descriptorSha256;
  changed.packages[1].pluginModule = { ...profile.packages[1].pluginModule, blake3: "38".repeat(32) };
  const changedPluginModule = createHash("sha256").update(trustedBootstrapProfileEncoding(changed, codecs)).digest("hex");
  changed.packages[1].pluginModule = profile.packages[1].pluginModule;
  changed.packages[1].receipt = { ...profile.packages[1].receipt, provenanceSha256: "39".repeat(32) };
  const changedReceipt = createHash("sha256").update(trustedBootstrapProfileEncoding(changed, codecs)).digest("hex");
  if ([changedComponent, changedDescriptor, changedCodec, changedPluginModule, changedReceipt].some((digest) => digest === generationNode)) throw new Error("trusted bootstrap generation omits zero-target stdio authority");
  const target = profile.openTargets[0];
  if (
    target.pluginId !== "gis" ||
    target.artifactKind !== "s.gis.gismap" ||
    target.surfaceId !== "s.gis.gismap@1/*#editor" ||
    target.appId !== target.surfaceId ||
    target.windowKindId !== "gis2d-main" ||
    target.role !== "editor" ||
    target.rendererTarget !== "wasm" ||
    !target.grant.read ||
    !target.grant.write ||
    !target.grant.observe ||
    codecs.gis.some((row: TrustedBootstrapCodec) => row.artifactKind === "s.gis.gisterrain" && row.artifactKind === target.artifactKind)
  )
    throw new Error("trusted bootstrap target is not the sole writable GIS Map editor surface");
  const relative = (value: string): boolean => !/^[/\\]|^[A-Za-z]:[/\\]|(?:^|[/\\])\.\.(?:[/\\]|$)/u.test(value);
  if (!relative("packages/gis/component.wasm") || relative("../component.wasm") || relative("C:\\component.wasm")) throw new Error("trusted bootstrap path fence drifted");
  const rotation = fixture.rotation;
  if (
    fixture.limits.descriptorBytes + 1 !== fixture.limits.descriptorBytesPlusOne ||
    rotation.initialGenerationId !== generationNode ||
    rotation.currentAfterFailedCandidate !== rotation.initialGenerationId ||
    rotation.failedCandidateGenerationId === rotation.initialGenerationId ||
    rotation.nextGenerationId === rotation.initialGenerationId ||
    trustedBootstrapPlanGenerationOutcome(rotation.stalePlan.issuedGenerationId, rotation.stalePlan.observedGenerationId) !== rotation.stalePlan.expected ||
    trustedBootstrapPlanGenerationOutcome(rotation.freshPlan.issuedGenerationId, rotation.freshPlan.observedGenerationId) !== rotation.freshPlan.expected ||
    rotation.stalePlan.issuedGenerationId !== rotation.initialGenerationId ||
    rotation.stalePlan.observedGenerationId !== rotation.nextGenerationId ||
    rotation.freshPlan.issuedGenerationId !== rotation.nextGenerationId ||
    fixture.cancellationStages.length !== 7 ||
    fixture.hostile.length !== 19 ||
    !fixture.hostile.includes("stale-plan-generation")
  )
    throw new Error("trusted bootstrap bounds/cancellation/cross-generation rotation corpus drifted");
  if (blake3Hex(Buffer.from("abc")) !== "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85") throw new Error("trusted bootstrap first-party BLAKE3 known answer mismatch");
  const descriptor: Record<string, any> = {
    descriptorVersion: 1,
    packageId: "semio:gis",
    role: "plugin",
    manifest: { pluginId: "gis", label: "GIS", version: "0.1.0", apps: [], examples: [], capabilities: [], topicContributions: [], commands: [], artifactKinds: [], dependencies: [{ pluginId: "stdio", version: "=0.1.0" }], contributions: [] },
    activationEvents: [],
    capabilityRequests: [],
    extensionPoints: [],
    execution: "isolated",
    executionProtocol: { appChannelVersion: DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 },
    quotas: {},
    contributions: {},
    assets: [],
  };
  const jsonBytes = Buffer.from(JSON.stringify(descriptor));
  const packBytes = Buffer.from(encodePackValue(descriptor));
  const binding = { profile: "release" as const, pluginId: "gis", packageId: "semio:gis", wasmSha256: "41".repeat(32), semanticDescriptorSha256: createHash("sha256").update(packBytes).digest("hex") };
  const freshPair = (json: Uint8Array, pack: Uint8Array) => {
    const pair = verifySemanticDescriptorPairBytesV1(json, pack);
    if (pair.pluginId !== "gis" || pair.packageId !== "semio:gis" || pair.version !== "0.1.0" || pair.role !== "plugin" || pair.execution !== "isolated") throw new Error("fresh descriptor identity differs");
    return pair;
  };
  const receipt = componentReceiptValueV1({ schema: "semio.plugin.component-receipt/v1", ...binding, profile: "release", target: "wasm32-wasip2", wasmBytes: 4096, toolchain: "rustc 1.99.0-nightly (9f36de775bc636c8e88c31a173c2bcb6995956a0)", sourceTreeDigest: "43".repeat(32), provenanceSha256: "44".repeat(32) });
  if (freshPair(jsonBytes, packBytes).semanticSha256 !== binding.semanticDescriptorSha256 || !componentReceiptBindsV1(receipt, binding)) throw new Error("fresh semantic descriptor and its receipt do not bind");
  if (componentReceiptBindsV1({ ...receipt, profile: "prototype" }, binding) || componentReceiptBindsV1(receipt, { ...binding, semanticDescriptorSha256: "45".repeat(32) })) throw new Error("fresh receipt law admitted a non-shipped or foreign binding");
  const hostilePairs = [
    [Buffer.from(JSON.stringify({ ...descriptor, manifest: { ...descriptor.manifest, version: "9.9.9" } })), packBytes],
    [jsonBytes, Buffer.concat([packBytes, Buffer.from([0])])],
    [Buffer.from([0xff]), packBytes],
    [Buffer.from(JSON.stringify({ ...descriptor, hashes: { wasmSha256: "41".repeat(32) } })), Buffer.from(encodePackValue({ ...descriptor, hashes: { wasmSha256: "41".repeat(32) } }))],
  ] as const;
  for (const [json, pack] of hostilePairs) {
    let rejected = false;
    try {
      freshPair(json, pack);
    } catch {
      rejected = true;
    }
    if (!rejected) throw new Error("fresh catalog package verifier admitted a hostile JSON/pack pair");
  }
  await proveTrustedRotationSourceFixture(fixture);
  console.log(
    `trusted-stdio-gis-bootstrap-oracle: packages=2 codecs=${codecs.stdio.length + codecs.gis.length} targets=1 hostile=${fixture.hostile.length} cancellation=${fixture.cancellationStages.length} descriptor-pairs=5 receipt-laws=3 stale-plan=1 scope-exports=5+node+webcrypto+first-party-pack+blake3=1; no materialization or hub activation claim`,
  );
}

/** 🔗️ Exercises receipt-derived dependency claims with Pack, AJV and independent byte framing. */
export async function proveTrustedCompiledDependenciesFixture(repoRoot: string): Promise<void> {
  await (await import("../../../🧰️framework/🔨️modules/🎒️pack/🌱️value/📜️script.ts")).proveWireValueMaterializationFixture(repoRoot);
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  const root = join(repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🔗️compiled-dependencies");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validateIdentity = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.trusted-catalog/TrustedBundleIdentityV1");
  assert.deepEqual(Object.keys(fixture), ["schema", "publicationFiles", "publicationDirectories", "descriptorPreviewCases", "selectedClosure", "cases", "consumerCatalogCases", "atomicCases", "nativeCases", "rawCases", "encodingCases", "ordering"]);
  assert.equal(fixture.schema, "semio.hub.compiled-stdio-dependencies/v1");
  assert.deepEqual(fixture.publicationFiles, ["component.wasm", "descriptor.semio", "closed-actor.mjs", "plugin-module-gis.json", "stdio-component.wasm", "stdio-descriptor.semio", "plugin-module-stdio.json", "trusted-catalog.json"]);
  assert.deepEqual(fixture.publicationDirectories, ["plugin-modules"]);
  assert.deepEqual(fixture.descriptorPreviewCases, [
    { id: "first-selected-descriptor-invalid", packageIndex: 1, previews: [] },
    { id: "second-selected-descriptor-invalid", packageIndex: 0, previews: [] },
  ]);
  assert.deepEqual(fixture.consumerCatalogCases, [
    { change: "exact", accepted: true }, { change: "missing", accepted: false }, { change: "duplicate", accepted: false },
    { change: "wrong-version", accepted: false }, { change: "foreign-topic-version", accepted: false },
  ]);
  // 🩹️ `gis-dependency`, `gis-trailing-byte` and `gis-duplicate-field` lost their `stdio` preview in
  // commit 48a8c69cdb (2026-09-19): the FIXTURE was changed and this literal was not, so this law
  // has been red ever since — found by ticket 26/09/18 slice TC3c, whose own gate it blocked. The
  // literal is aligned to the fixture, which is the side that was edited deliberately.
  assert.deepEqual(fixture.atomicCases, [
    { change: "stdio-catalog", accepted: false, previews: [] }, { change: "gis-catalog", accepted: false, previews: ["stdio"] },
    { change: "gis-dependency", accepted: false, previews: [] }, { change: "gis-provider", accepted: false, previews: ["stdio"] },
    { change: "gis-trailing-byte", accepted: false, previews: [] }, { change: "gis-duplicate-field", accepted: false, previews: [] },
    { change: "exact", accepted: true, previews: ["stdio", "gis"] },
  ]);
  assert.equal(fixture.selectedClosure.length, 2);
  for (const identity of [...fixture.selectedClosure, ...fixture.ordering.input, ...fixture.encodingCases.flatMap((row: any) => row.identities)]) assert(validateIdentity(identity), "compiled-dependency identity must satisfy the scope-owned contract");
  assert.equal(fixture.cases.length, 25);
  assert.deepEqual([...new Set(fixture.cases.map((row: any) => row.change))], [
    "exact", "selected-order", "missing", "duplicate", "self", "unknown", "any", "caret", "wrong-version", "noncanonical-version", "overflow-version",
    "foreign-package", "duplicate-selected", "missing-selected", "extra-field", "missing-manifest", "oversized-claim", "stdio-dependent", "owner-plugin",
    "owner-package", "owner-version", "selected-version", "missing-protocol", "unsupported-protocol",
  ]);
  assert.equal(new Set(fixture.cases.map((row: any) => `${row.change}/${row.owner}`)).size, fixture.cases.length);
  for (const row of fixture.cases) assert.deepEqual(Object.keys(row).sort(), ["accepted", "change", "owner"]);
  for (const row of fixture.cases) assert(["gis", "stdio"].includes(row.owner) && typeof row.accepted === "boolean", row.change);
  assert.deepEqual(fixture.encodingCases.map((row: any) => row.name), ["gis-direct-dependency-stdio", "stdio-direct-dependencies-empty"]);
  assert.equal(fixture.rawCases.length, 4);
  for (const row of [...fixture.rawCases, ...fixture.encodingCases]) assert.match(row.hex, /^(?:[0-9a-f]{2}){4,1024}$/u);
  assert.equal(fixture.ordering.input.length, 3);
  assert.equal(fixture.ordering.expected.length, 3);
  const kindRoot = join(repoRoot, "🧰️framework/🔨️modules/🛂️manifest/🧫️fixtures");
  const kind = JSON.parse(readFileSync(join(kindRoot, "🗄️artifact-kind-formats.json"), "utf8"));
  assert.deepEqual(packValueToExactJson(decodePackValue(encodePackValue(kind))), JSON.parse(JSON.stringify(kind)));
  assert.deepEqual(fixture.nativeCases.map((row: any) => row.id), ["exact", "missing", "duplicate", "foreign", "any", "caret", "tilde", "at-least", "wrong-version"]);
  const hubSource = moduleRustSource(join(repoRoot, "🌎️hub", "🧪️tests"));
  const readinessFixture = hubSource.slice(hubSource.indexOf("fn native_openable_stdio_bundle("), hubSource.indexOf("async fn native_openable_stdio_provider_is_the_only_atomic_readiness_transition("));
  assert(readinessFixture.includes('"executionProtocol": { "appChannelVersion": descriptor.execution_protocol.app_channel_version }'), "readiness fixture must carry its actual descriptor protocol");
  const runnerSource = readHubScriptModule(repoRoot, "🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication");
  const fixtureCopy = hubSource.slice(hubSource.indexOf("async fn check_in_process_fixture_emits_verified_gis_ledger_and_catalog"));
  const cliCopy = runnerSource.slice(runnerSource.indexOf("\nasync function proveTrustedPublicationCli("));
  assert.deepEqual(JSON.parse(fixtureCopy.match(/for name in (\[[^\]\n]+\])/u)?.[1] ?? "null"), fixture.publicationFiles, "native process fixture must retain every dependency file");
  assert(cliCopy.includes("withCheckInProcessFixtureV1")&&cliCopy.includes('purpose:"publication"'),"publication CLI must retain the actual complete native fixture through its owned seed reader");
  assert.deepEqual(JSON.parse(fixtureCopy.match(/for directory in (\[[^\]\n]+\])/u)?.[1] ?? "null"), fixture.publicationDirectories, "native process fixture must retain every plugin module file");
  assert.deepEqual(JSON.parse(cliCopy.match(/for \(const directory of (\[[^\]\n]+\])/u)?.[1] ?? "null"), fixture.publicationDirectories, "publication CLI must retain every plugin module file");
  for (const row of fixture.nativeCases) assert.equal(row.dependencies.length === 1 && row.dependencies[0].pluginId === "stdio" && row.dependencies[0].version === "=0.1.0", row.accepted, row.id);
  assert.deepEqual(fixture.cases.map((row: any) => `${row.owner}:${row.change}`), ["gis:exact", "stdio:exact", "gis:selected-order", "gis:missing", "gis:duplicate", "gis:self", "gis:unknown", "gis:any", "gis:caret", "gis:wrong-version", "gis:noncanonical-version", "gis:overflow-version", "gis:foreign-package", "gis:duplicate-selected", "gis:missing-selected", "gis:extra-field", "gis:missing-manifest", "gis:oversized-claim", "stdio:stdio-dependent", "gis:owner-plugin", "gis:owner-package", "gis:owner-version", "gis:selected-version", "gis:missing-protocol", "gis:unsupported-protocol"]);
  assert.deepEqual(fixture.rawCases.map((row: any) => row.name), ["canonical-gis-descriptor", "duplicate-top-level-packageId-earlier-null-overwritten", "duplicate-manifest-earlier-null-overwritten", "canonical-descriptor-with-trailing-zero"]);
  const canonicalRaw = decodePackValue(Buffer.from(fixture.rawCases[0].hex, "hex"));
  for (const row of fixture.rawCases) {
    const bytes = Buffer.from(row.hex, "hex");
    assert.deepEqual(decodePackValue(bytes), canonicalRaw);
    if (row.accepted) assert.deepEqual(trustedBootstrapResolveDependencies(trustedBootstrapDescriptorClaims(bytes), fixture.selectedClosure), [fixture.selectedClosure[1]]);
    else assert.throws(() => trustedBootstrapDescriptorClaims(bytes), /Pack is not canonical/);
  }
  assert.deepEqual(fixture.encodingCases.map((row: any) => row.name), ["gis-direct-dependency-stdio", "stdio-direct-dependencies-empty"]);
  for (const row of fixture.encodingCases) assert.equal(trustedBootstrapDependencyEncoding(row.identities).toString("hex"), row.hex);
  for (const pluginId of [" stdio", "stdio ", String.fromCharCode(0xd800)]) assert.throws(() => trustedBootstrapIdentity({ pluginId, packageId: "semio:stdio", version: "0.1.0" }), /identity/);
  let accepted = 0;
  for (const row of fixture.cases) {
    const selected = structuredClone(fixture.selectedClosure);
    const owner = selected.find((identity: any) => identity.pluginId === row.owner);
    const dependencies: any[] = row.owner === "gis" ? [{ pluginId: "stdio", version: "=0.1.0" }] : [];
    const descriptor: any = { packageId: owner.packageId, manifest: { pluginId: row.owner, version: owner.version, dependencies }, executionProtocol: { appChannelVersion: DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 } };
    if (row.change === "selected-order") selected.reverse();
    if (row.change === "missing") dependencies.length = 0;
    if (row.change === "duplicate") dependencies.push({ ...dependencies[0] });
    if (row.change === "self") dependencies[0].pluginId = "gis";
    if (row.change === "unknown") dependencies[0].pluginId = "unknown";
    if (row.change === "any") dependencies[0].version = "*";
    if (row.change === "caret") dependencies[0].version = "^0.1.0";
    if (row.change === "wrong-version") dependencies[0].version = "=0.2.0";
    if (row.change === "noncanonical-version") dependencies[0].version = "=00.1.0";
    if (row.change === "overflow-version") dependencies[0].version = "=18446744073709551616.1.0";
    if (row.change === "foreign-package") selected[1].packageId = "semio:foreign";
    if (row.change === "duplicate-selected") selected.push({ ...selected[1] });
    if (row.change === "missing-selected") selected.pop();
    if (row.change === "extra-field") dependencies[0].extra = true;
    if (row.change === "missing-manifest") delete descriptor.manifest;
    if (row.change === "oversized-claim") dependencies[0].pluginId = "x".repeat(257);
    if (row.change === "stdio-dependent") dependencies.push({ pluginId: "gis", version: "=0.1.0" });
    if (row.change === "owner-plugin") descriptor.manifest.pluginId = "foreign";
    if (row.change === "owner-package") descriptor.packageId = "semio:foreign";
    if (row.change === "owner-version") descriptor.manifest.version = "0.2.0";
    if (row.change === "selected-version") selected[1].version = "0.2.0";
    if (row.change === "missing-protocol") delete descriptor.executionProtocol;
    if (row.change === "unsupported-protocol") descriptor.executionProtocol.appChannelVersion = DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 + 1;
    const counterpart = fixture.selectedClosure.find((identity: any) => identity.pluginId !== row.owner);
    const counterpartDescriptor = { packageId: counterpart.packageId, manifest: { pluginId: counterpart.pluginId, version: counterpart.version, dependencies: counterpart.pluginId === "gis" ? [{ pluginId: "stdio", version: `=${counterpart.version}` }] : [] }, executionProtocol: { appChannelVersion: DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 } };
    const operation = () => {
      const claims = new Map([[row.owner, trustedBootstrapDescriptorClaims(encodePackValue(descriptor))], [counterpart.pluginId, trustedBootstrapDescriptorClaims(encodePackValue(counterpartDescriptor))]]);
      return trustedBootstrapResolveClosure("local-stdio-gis-open-v1", claims, selected).get(row.owner)!;
    };
    if (!row.accepted) { assert.throws(operation, row.change === "extra-field" ? /keys/ : /dependenc|descriptor|identity|version|manifest|protocol|object/); continue; }
    const actual = operation();
    assert.deepEqual(actual, row.owner === "gis" ? [fixture.selectedClosure[1]] : []);
    assert(Object.isFrozen(actual) && actual.every(Object.isFrozen));
    const independent = new Uint8Array(4 + actual.reduce((size, identity) => size + [identity.pluginId, identity.packageId, identity.version].reduce((sum, field) => sum + 8 + new TextEncoder().encode(field).length, 0), 0));
    const view = new DataView(independent.buffer);
    view.setUint32(0, actual.length, false);
    let offset = 4;
    for (const identity of actual) for (const field of [identity.pluginId, identity.packageId, identity.version]) {
      const bytes = new TextEncoder().encode(field);
      view.setBigUint64(offset, BigInt(bytes.length), false); offset += 8;
      independent.set(bytes, offset); offset += bytes.length;
    }
    assert.deepEqual(trustedBootstrapDependencyEncoding(actual), Buffer.from(independent));
    accepted++;
  }
  assert.deepEqual(trustedBootstrapDependencies(fixture.ordering.input).map((identity) => identity.pluginId), fixture.ordering.expected);
  assert.equal(accepted, 3);
  console.log(`compiled dependency claims: AJV=1 Pack=1 DataView=1 cases=${fixture.cases.length} accepted=${accepted} native-vectors=${fixture.nativeCases.length} raw-Pack=${fixture.rawCases.length} source-derived-goldens=${fixture.encodingCases.length} publication-files=${fixture.publicationFiles.length} UTF8-order=1; native guest agreement remains separate`);
}
