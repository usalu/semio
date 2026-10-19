/** 🧭️ Space artifact creation contract gate.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { configuredExactCargoLawPolicyV1 } from "@semio-tech/process/cargo-laws";
import { descriptorDigestEncodingV1 } from "@semio-tech/framework-os/modules/directory/io/binary/descriptor-digest";
import { parseSpaceArtifactCreateJsonV1, parseSpaceArtifactCreationCatalogJsonV1, parseSpaceArtifactCreationStatusJsonV1 } from "@semio-tech/framework-os/modules/directory/io/text/space-artifact-creation-v1";
import { type ExactCargoLawGroup } from "@semio-tech/process/cargo-laws";
import { buildBudgetMs } from "@semio-tech/process/budget";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import { canonicalJson } from "@semio-tech/repo-lib/authoring";
import { decodePackValue, encodePackValue, packValueToExactJson } from "@semio-tech/framework-os";
import type { PackValue } from "@semio-tech/framework-os";
import { DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, parseDocumentOpenPlanV1, parseDocumentExecutionTargetLeaseFieldsV1, leaseFieldsFromPlanV1 } from "@semio-tech/framework-os/modules/directory/schema/document-open";
import { DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES } from "@semio-tech/framework/directory/schema";
import { sealSpaceArtifactCreateV1 } from "@semio-tech/framework-os/modules/directory/schema/space-artifact-creation-v1";
import { foldAll as foldDirectoryIndexEvents } from "@semio-tech/framework/directory";
import { artifactFrontierIsGenesisForV1, artifactFrontierIsEditedForV1, validDocumentIndexEntryV1 } from "@semio-tech/framework/directory/schema";
import { runAdmittedExactCargoLawsV1 } from "@semio-tech/process/cargo-test";
import { BundleScript } from "@semio-tech/process/routing";
import { hubSchemaExport } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { exactCargoStageEnvironments } from "../../🏗️build/🛂staging-environment/🟦️.ts";
import { localRelayUpstreamPath } from "../../🚀️local-relay/🧭️routing/🟦️.ts";
import { executionTargetLeaseFieldsAdmissible } from "../🔗️execution-target-lease-checks/🟦️.ts";
import { documentOpenNeutralStructure } from "../🗺️open-plan-checks/🟦️.ts";
import { hubLawGroups, moduleRustSource } from "../../🧪️testing/📖️source-readers/🟦️.ts";

/** 📇️ The `document.indexed` directory event body as `schema://os.directory/DirectoryEventBody` declares it. */
type DocumentIndexedEventBodyV1 = {
  readonly kind: "document.indexed";
  readonly scope: { readonly spaceId: string; readonly documentId: string };
  readonly descriptorDigestV1: readonly number[];
  readonly entry: { readonly name: string; readonly dialect: { readonly artifactKind: string; readonly standard: string; readonly subset: string } };
};

function proveSpaceArtifactCreationContractV1(repoRoot: string): number {
  const base = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🌱️space-artifact-creation-v1");
  const fixture = JSON.parse(readFileSync(join(base, "🔣️.json"), "utf8"));
  const validate = hubSchemaExport(repoRoot, "schema://os.directory/SpaceArtifactCreationV1");
  let checks = 0;
  /** 🪢 `kindId` (manifest taxonomy space: `2d.note`, `stdio.json`) and `dialect.artifactKind` /
   * `parentDialect.artifactKind` (plugin dialect space: `s.note.note`, `s.stdio.json`) are two id
   * spaces that coincide for `gis` alone, so this independent re-derivation bounds each field on its
   * own exactly as `SpaceArtifactCreationKindV1::validate` and `SpaceArtifactCreationReadyV1::validate`
   * now do; the binding between the spaces is declared by the trusted catalog, which derives both
   * from one `VerifiedDocumentOpenSelectionV1`. */
  const creationIdentity = (value: unknown): boolean => typeof value === "string" && value.length > 0 && value.length <= 256 && /^[A-Za-z0-9][A-Za-z0-9._:/-]*$/u.test(value);
  for (const row of [...fixture.requests, ...fixture.statuses, ...fixture.catalogs]) {
    const value = row.value;
    const independent = validate(value)
      && (value.progress === undefined || value.progress.completedUnits <= value.progress.totalUnits)
      && (value.phase !== "ready" || creationIdentity(value.ready.parentDialect.artifactKind))
      && (value.kinds === undefined || value.kinds.every((kind: any, index: number) => creationIdentity(kind.dialect.artifactKind) && (index === 0 || value.kinds[index - 1].kindId < kind.kindId)));
    if (Boolean(independent) !== row.accepted) throw new Error(`creation schema disagrees at ${row.id}`);
    let own = false;
    try {
      const actual = row.value.kinds !== undefined ? parseSpaceArtifactCreationCatalogJsonV1(JSON.stringify(value)) : row.value.phase === undefined ? sealSpaceArtifactCreateV1(value) : parseSpaceArtifactCreationStatusJsonV1(JSON.stringify(value));
      own = JSON.stringify(actual) === JSON.stringify(value);
    } catch {}
    if (own !== row.accepted) throw new Error(`creation first-party TypeScript disagrees at ${row.id}`);
    const wire = encodePackValue(value as PackValue);
    if (canonicalJson(packValueToExactJson(decodePackValue(wire))) !== canonicalJson(value)) throw new Error(`creation Pack roundtrip differs at ${row.id}`);
    checks++;
  }
  for (const row of fixture.rawJson) {
    let accepted = false;
    try { accepted = Boolean(row.type === "request" ? parseSpaceArtifactCreateJsonV1(row.source) : row.type === "catalog" ? parseSpaceArtifactCreationCatalogJsonV1(row.source) : parseSpaceArtifactCreationStatusJsonV1(row.source)); } catch {}
    if (accepted !== row.accepted) throw new Error(`creation raw JSON boundary differs: ${row.id}`);
  }
  const httpBase = join(repoRoot, "🌎️hub/🗿️artifact-authority/🌱️creation/🧪️tests/🧫️fixtures/🌐️http-owner-v1");
  const httpFixture = JSON.parse(readFileSync(join(httpBase, "🔣️.json"), "utf8"));
  const admitsHttpLimits = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.creation/ArtifactCreationHttpLimitsV1");
  const admitsHttpResponse = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.creation/ArtifactCreationHttpResponseV1");
  if (!admitsHttpLimits(httpFixture.limits) || httpFixture.limits.bodyBytes !== 4096 || httpFixture.limits.liveOperations !== 8 || httpFixture.limits.lifetimeMs !== 30000) throw new Error("creation HTTP owner limits differ from their owning scope contract");
  for (const row of httpFixture.responses) if (!admitsHttpResponse(row)) throw new Error(`creation HTTP response violates its owning scope contract: ${row.phase}`);
  for (const row of httpFixture.routes) {
    const actual = localRelayUpstreamPath(row.method, new URL(row.path, "http://relay.invalid"));
    if ((actual !== undefined) !== row.accepted) throw new Error(`creation relay boundary differs: ${row.id}`);
  }
  for (const row of httpFixture.authorities) {
    const expected = !row.authenticated || !row.sessionCurrent ? "unauthorized" : !row.backendAvailable ? "unavailable" : !row.member || row.role !== "author" ? "forbidden" : "admitted";
    if (expected !== row.expected) throw new Error(`creation independent HTTP authority differs: ${row.id}`);
  }
  for (const row of httpFixture.responses) {
    const expected = row.phase === "accepted" || row.phase === "preparing" ? 202 : 200;
    if (row.status !== expected) throw new Error(`creation HTTP response mapping differs: ${row.phase}`);
  }
  const spaceEditor = moduleRustSource(join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"));
  const createArtifact = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱create-artifact/🦀️.rs"), "utf8");
  if (!spaceEditor.includes('kind_choice: str_field("kindChoice").unwrap_or_default()')
    || spaceEditor.includes('kind_choice: str_field("kindId")')
    || !spaceEditor.includes("create_artifact_dialog_submission_preserves_the_exact_catalog_choice_in_the_host_relay")
    || !createArtifact.includes('action_id: "os.create-space-artifact".into()')
    || !createArtifact.includes('"kindChoice": payload.kind_choice')) throw new Error("ordinary Space creation dialog relay is not exact kindChoice-only");
  console.log(`space artifact creation contract: cases=${checks} raw-json=${fixture.rawJson.length} relay=${httpFixture.routes.length} authority=${httpFixture.authorities.length} responses=${httpFixture.responses.length} AJV=1 scope-exports=3 TypeScript=1 Pack=1 ready-only-coordinate=1 ordinary-bridge=3; runtime genesis remains a separate gate`);
  const indexed = JSON.parse(readFileSync(join(base, "../📇️document-index-v1/🔣️.json"), "utf8"));
  /** 🚧️ `schema://os.directory/DirectoryEventBody` is the owning export, but it carries the OpenAPI
   * `discriminator` annotation, which the shared draft-07 validator rejects as an unknown keyword. Until
   * `os.directory` drops it the `document.indexed` branch is compiled on its own, in the right dialect. */
  const directorySchema = JSON.parse(readFileSync(join(base, "../../🧬️schema/🔣️.json"), "utf8"));
  const indexedBodySchema = directorySchema.$defs.DirectoryEventBody.oneOf.find((row: any) => row.properties.kind.const === "document.indexed");
  const validateIndex = new Ajv({ strict: true, allErrors: true }).compile<DocumentIndexedEventBodyV1>({ ...indexedBodySchema, $defs: directorySchema.$defs });
  for (const row of indexed.cases) {
    const client = foldDirectoryIndexEvents({ spaces: new Map(), users: new Map(), cursor: 0 }, row.events);
    if (client.spaces.get("space-fixture")?.indexedDocuments.length !== row.clientRows) throw new Error(`document index client order differs: ${row.id}`);
    const descriptors = new Map<string, any>();
    let backendAccepted = true;
    for (const event of row.events) {
      const body = event.body;
      if (body.kind === "document.announced") descriptors.set(JSON.stringify([body.descriptor.spaceId, body.descriptor.documentId]), body.descriptor);
      if (body.kind !== "document.indexed") continue;
      const descriptor = descriptors.get(JSON.stringify([body.scope.spaceId, body.scope.documentId]));
      const expected = descriptor ? Array.from(createHash("sha256").update(descriptorDigestEncodingV1(descriptor)).digest()) : null;
      backendAccepted &&= Boolean(validateIndex(body) && descriptor && event.userId && event.spaceId === body.scope.spaceId && validDocumentIndexEntryV1(body.entry) && JSON.stringify(expected) === JSON.stringify(body.descriptorDigestV1));
    }
    if (backendAccepted !== row.backendAccepted) throw new Error(`document index independent SHA256 authority differs: ${row.id}`);
  }
  console.log(`document index fixture: ordered-client=${indexed.cases.length} AJV=1 independent-node-SHA256=${indexed.cases.length}; backend transactions not executed`);
  const genesis = JSON.parse(readFileSync(join(base, "../🌱️artifact-genesis-v1/🔣️.json"), "utf8"));
  const openFixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json"), "utf8"));
  const requiredPlan = hubSchemaExport(repoRoot, "schema://os.directory/DocumentOpenPlanV1");
  const requiredLease = hubSchemaExport(repoRoot, "schema://os.directory/DocumentExecutionTargetLeaseFieldsV1");
  for (const row of openFixture.checkpointPresenceCases) {
    const plan = structuredClone(openFixture.validPlan);
    const lease = leaseFieldsFromPlanV1(parseDocumentOpenPlanV1(plan, openFixture.nowMs), { component: 1, descriptor: 1 });
    for (const value of [plan, lease]) {
      if (row.kind === "missing") delete (value as any).checkpoint;
      else if (row.kind === "null") (value as any).checkpoint = null;
      else if (row.kind !== "present") throw new Error("unknown checkpoint presence case");
    }
    let planAccepted = false, leaseAccepted = false;
    try { parseDocumentOpenPlanV1(plan, openFixture.nowMs); planAccepted = true; } catch {}
    try { parseDocumentExecutionTargetLeaseFieldsV1(lease); leaseAccepted = true; } catch {}
    let neutralAccepted = false;
    try { documentOpenNeutralStructure(plan, openFixture.nowMs); neutralAccepted = true; } catch {}
    const neutralLeaseAccepted = executionTargetLeaseFieldsAdmissible(lease, { componentMaxBytes: DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, descriptorMaxBytes: DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES });
    if (planAccepted !== row.accepted || leaseAccepted !== row.accepted || neutralAccepted !== row.accepted || neutralLeaseAccepted !== row.accepted || Boolean(requiredPlan(plan)) !== row.accepted || Boolean(requiredLease(lease)) !== row.accepted) throw new Error(`required checkpoint presence differs: ${row.kind}`);
  }
  console.log(`required committed checkpoint: TypeScript/AJV plan+lease cases=${openFixture.checkpointPresenceCases.length * 2}; protected HTTP endpoints not executed`);
  const validateFrontier = hubSchemaExport(repoRoot, "schema://os.directory/ArtifactFrontier");
  for (const row of genesis.frontiers) {
    const independentFrontier = Boolean(validateFrontier(row.frontier) && row.scope.spaceId && row.scope.documentId === row.frontier.documentId);
    if (independentFrontier !== (row.genesis || row.edited)) throw new Error(`genesis AJV frontier differs: ${row.id}`);
    if (artifactFrontierIsGenesisForV1(row.scope, row.frontier) !== row.genesis || artifactFrontierIsEditedForV1(row.scope, row.frontier) !== row.edited) throw new Error(`genesis frontier differs: ${row.id}`);
    const plan = structuredClone(openFixture.validPlan);
    plan.scope = row.scope;
    plan.checkpoint.baselineFrontier = row.frontier;
    let opens = false;
    try { const parsed = parseDocumentOpenPlanV1(plan, openFixture.nowMs); leaseFieldsFromPlanV1(parsed, { component: 1, descriptor: 1 }); opens = true; } catch {}
    if (opens !== (row.genesis || row.edited)) throw new Error(`genesis open plan/lease differs: ${row.id}`);
  }
  const hash = (bytes: Uint8Array): number[] => Array.from(createHash("sha256").update(bytes).digest());
  const operationRoot = join(repoRoot, "🌎️hub/🗿️artifact-authority/🌱️creation/🧪️tests/🧫️fixtures/📚️operation-v1");
  const transactionRoot = join(operationRoot, "../🧪️transaction-v1");
  const transactions = JSON.parse(readFileSync(join(transactionRoot, "🔣️.json"), "utf8"));
  if (transactions.cases.length !== 32 || new Set(transactions.cases.map((row: any) => row.kind)).size !== 32) throw new Error("creation transaction outcome corpus is not the exact kind closure");
  for (const row of transactions.cases) {
    const committed = ["success", "ack-loss", "ack-receipt-unreadable", "ack-events-unreadable"].includes(row.kind);
    const terminal = committed ? "ready" : row.kind === "cancelled" ? "cancelled" : "preparing";
    const delivery = committed ? row.kind.includes("unreadable") ? "reconnect" : "broadcast" : "none";
    if (row.committed !== committed || row.publicEvents !== (committed ? 3 : 0) || row.terminal !== terminal || row.delivery !== delivery) throw new Error(`creation transaction oracle differs: ${row.kind}`);
  }
  const recovery = JSON.parse(readFileSync(join(transactionRoot, "🧯️accepted-recovery.json"), "utf8"));
  if (recovery.cases.length !== 3 || new Set(recovery.cases.map((row: any) => row.kind)).size !== 3) throw new Error("creation accepted recovery corpus is not the exact kind closure");
  for (const row of recovery.cases) {
    if (!["live", "revoked", "expired"].includes(row.kind) || row.recoveryCandidates !== (row.kind === "expired" ? 1 : 0) || row.terminal !== (row.kind === "live" ? "accepted" : "failed") || row.facts !== (row.kind === "live" ? 1 : 2)) throw new Error(`creation accepted recovery oracle differs: ${row.kind}`);
  }
  const operation = JSON.parse(readFileSync(join(operationRoot, "🔣️.json"), "utf8"));
  const cancellations = JSON.parse(readFileSync(join(operationRoot, "🛑️cancel.json"), "utf8"));
  if (cancellations.cases.length !== 9) throw new Error("creation cancellation race corpus differs");
  for (const row of cancellations.cases) {
    const revision = row.state === "accepted" ? 1 : row.state === "committed" ? 3 : 2;
    const accepted = Number.isSafeInteger(row.observedAtMs) && row.observedAtMs >= operation.intent.acceptedAtMs + revision - 1 && Number.isInteger(row.expectedRevision) && row.expectedRevision > 0 && row.expectedRevision <= revision;
    const phase = !accepted ? null : row.state === "committed" ? "ready" : row.state === "failed" ? "failed" : "cancelled";
    if (row.accepted !== accepted || row.phase !== phase) throw new Error(`creation cancellation oracle differs: ${row.state}/${row.expectedRevision}`);
  }
  if (operation.schema !== "semio.test.artifact-creation-operation/v1" || operation.transitions.length !== 30
    || new Set(operation.transitions.map((row: any) => `${row.state}/${row.fact}`)).size !== 30) throw new Error("creation operation transition corpus is not the exact 6x5 closure");
  for (const row of operation.transitions) {
    const open = row.state === "absent" && row.fact === "accepted" || row.state === "accepted" && row.fact === "prepared" || row.state === "prepared" && row.fact === "committed" || ["accepted", "prepared"].includes(row.state) && ["cancelled", "failed"].includes(row.fact);
    if (row.next !== (open ? row.fact : null)) throw new Error(`creation operation transition oracle differs: ${row.state}/${row.fact}`);
  }
  const intentDigest = createHash("sha256").update("semio.hub.artifact-creation-intent.v1\0");
  for (const value of [operation.intent.scope.spaceId, JSON.stringify(operation.intent.request)]) {
    const bytes = Buffer.from(value); const length = Buffer.alloc(8); length.writeBigUInt64BE(BigInt(bytes.length)); intentDigest.update(length).update(bytes);
  }
  if (intentDigest.digest("hex") !== operation.intent.commandSha256) throw new Error("creation operation independent intent commitment differs");
  const expected = genesis.expected;
  if (JSON.stringify(hash(Buffer.from(expected.checkpointEncodingHex, "hex"))) !== JSON.stringify(expected.checkpoint.checkpointId)
    || JSON.stringify(hash(descriptorDigestEncodingV1(expected.descriptor))) !== JSON.stringify(expected.checkpoint.descriptorDigestV1)
    || JSON.stringify(hash(Buffer.from(genesis.initialPair.pack))) !== JSON.stringify(expected.checkpoint.pack.sha256)
    || JSON.stringify(hash(Buffer.from(genesis.initialPair.spr))) !== JSON.stringify(expected.checkpoint.spr.sha256)
    || JSON.stringify(hash(Buffer.from([...genesis.initialPair.pack, ...genesis.initialPair.spr]))) !== JSON.stringify(expected.checkpoint.aggregateSha256)) throw new Error("genesis independent SHA256 fixture differs");
  console.log(`genesis frontier: TypeScript=${genesis.frontiers.length} AJV=1 open/lease=${genesis.frontiers.length} independent-node-SHA256=5; native factory/publication not executed`);
  return checks + httpFixture.routes.length + httpFixture.authorities.length + httpFixture.responses.length;
}

export class SpaceArtifactCreationCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = segments[0] ?? "source";
    if (segments.length > 1 || !["source", "sqlite", "postgres", "neo4j", "native"].includes(phase)) throw new Error("space-artifact-creation-check accepts source, sqlite, postgres, neo4j, or native");
    proveSpaceArtifactCreationContractV1(this.repoRoot);
    if (phase !== "source") {
      const sqliteLaws: [string, string][] = [["semio-hub-artifact-creation", "tests::creation_contract_matches_neutral_intents_and_ready_only_coordinates"], ["semio-hub-directory-laws", "unit::directory_document_index_is_ordered_idempotent_and_backend_descriptor_bound"], ["semio-hub-directory-backend-laws", "sqlite_tests::document_index_neutral_transactions_survive_projection_rebuild"], ["semio-hub-artifact-creation", "tests::genesis_materialization_binds_exact_zero_history_and_independent_sha256"], ["semio-hub-artifact-creation", "operation::tests::creation_facts_follow_neutral_terminal_and_exact_pair_transitions"], ["semio-hub-directory-laws", "sqlite_creation_tests::quick::genesis_neutral_transactions_are_atomic_replayable_and_authorized"], ["semio-hub-directory-laws", "sqlite_creation_tests::genesis_delivery_lease_serializes_invalidation_and_requires_replay"], ["semio-hub-directory-laws", "sqlite_creation_tests::long::genesis_accepted_only_recovery_has_no_prepared_or_public_side_effects"], ["semio-hub-directory-laws", "sqlite_creation_tests::genesis_physical_pair_and_sqlite_restart_preserve_exact_receipt"]];
      sqliteLaws.push(["semio-hub-directory-laws", "unit::memory_projection_is_atomic_and_fixed_caps_reject_max_plus_one"], ["semio-hub-directory-laws", "unit::artifact_public_scalars_and_private_locators_obey_exact_max_plus_one_laws"], ["semio-hub-directory-laws", "unit::artifact_checkpoint_publication_is_atomic_bounded_idempotent_and_replayable"], ["semio-hub-directory-laws", "unit::artifact_chunk_cas_retention_space_delete_and_dry_run_preserve_live_bytes"], ["semio-hub-directory-laws", "unit::artifact_chunk_cas_two_service_sweep_and_reservation_race_is_serialized_before_rewrite"]);
      sqliteLaws.push(["semio-hub-directory-laws", "unit::artifact_chunk_cas_sqlite_and_filesystem_restart_rebuild_restore_exact_authority"], ["semio-hub-directory-laws", "unit::artifact_chunk_cas_filesystem_process_sweep_and_publication_race_preserves_exact_bytes"]);
      const backend: { laws: [string, string][] } = phase === "postgres" ? { laws: [["semio-hub-directory-laws", "postgres_creation_tests::genesis_commit_ack_and_locked_expiry_are_atomic_postgres"]] }
        : phase === "neo4j" ? { laws: [["semio-hub-directory-laws", "neo4j_creation_tests::genesis_commit_ack_and_locked_expiry_are_atomic_neo4j"]] }
          : { laws: sqliteLaws };
      const groups: ExactCargoLawGroup[] = [
        ...hubLawGroups(backend.laws),
      ];
      if (phase === "native") {
        groups.push({
          package: "semio-hub-boot-laws",
          target: { kind: "lib" },
          cargoArgs: ["--features", "integration-fixtures"],
          laws: ["tests::artifact_creation_admission_cannot_activate_after_shutdown_deadline"],
        },
        {
          package: "semio-hub-hostile-input-laws",
          target: { kind: "lib" },
          cargoArgs: ["--features", "integration-fixtures"],
          laws: ["tests::long::space_artifact_creation_routes_are_author_owned_idempotent_and_genesis_backed"],
        },
        {
          package: "semio-hub-open-plan-laws",
          target: { kind: "lib" },
          cargoArgs: [],
          laws: ["tests::document_open_and_execution_target_refuse_descriptor_or_index_without_genesis", "tests::socket_grant_directory_route_uses_credential_free_hello_and_revokes_live"],
        });
      }
      const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 60_000, lawMilliseconds: 180_000 },cwd: this.repoRoot,...exactCargoStageEnvironments(),groups,progress(event) { console.log(`space-artifact-creation ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); }});
      for (const receipt of receipts) console.log(`space-artifact-creation-receipt: ${JSON.stringify(receipt)}`);
    }
  }
}
