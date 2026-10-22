/** 📇️ Directory commands: descriptor digest, checkpoint pair, runtime source, session authority, event page contract, client and bootstrap, and the lease fixture.
 *
 * @see ../../📜️script.ts — the package script that registers the commands of this module
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { configuredExactCargoLawPolicyV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { runBudgetedTestCommand } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { runAdmittedCargoPackageTestsV1, runAdmittedExactCargoLawsV1 } from "@semio-tech/process/cargo-test";
import { runRepositoryTestCommand } from "@semio-tech/process/test-execution";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readModuleTreeSource, sliceModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { semioSchemaAjvV1 } from "@semio-tech/schema-ts/oracles/validator";
import { type SchemaCheck, exactCargoStageEnvironments } from "../🧱️script-bases/🟦️.ts";
import { runtimeTypeEnvironmentV1 } from "../../../../../../🔨️modules/⚙️runtime/🏷️identity/🟦️.ts";
import { runtimeResolveV1 } from "../../../../../../🔨️modules/⚙️runtime/🚀️process/🟦️.ts";

/** 🪢️ Verifies explicit binary checkpoint ownership and independent digest/framing witnesses. */
export class DirectoryDescriptorDigestSourceScript extends BundleScript {
  async run(segments:string[]):Promise<void>{
    if(segments.length)throw new Error("test-directory-descriptor-digest-source accepts no arguments");
    const source=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/🧱️binary/🔐️descriptor-digest/🧪️tests/🟦️.ts");
    await runBudgetedTestCommand(process.execPath,[runtimeResolveV1("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types",runtimeTypeEnvironmentV1(),source],{cwd:this.repoRoot,budgetMs:30000,throwOnFailure:true});
    await runRepositoryTestCommand(process.execPath,["test",source],{cwd:this.repoRoot});
  }
}

export class DirectoryDescriptorDigestNativeScript extends BundleScript {
  async run(segments:string[]):Promise<void>{const {rest}=resolveTestLevel(segments);await runAdmittedCargoPackageTestsV1(["semio-framework-os-kernel"], this.repoRoot, this.invocation, ["--lib","os_directory::io::binary::","--no-fail-fast",...rest,"--","--nocapture"]);}
}

export class DirectoryCheckpointPairSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-directory-checkpoint-pair-source accepts no arguments");
    const source = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🧪️tests/🪢️canonical-checkpoint-pair/🟦️.ts");
    await runBudgetedTestCommand(process.execPath, [runtimeResolveV1("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", runtimeTypeEnvironmentV1(), source], { cwd: this.repoRoot, budgetMs: 30000, throwOnFailure: true });
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot });
  }
}

export class DirectoryCheckpointPairNativeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runAdmittedCargoPackageTestsV1(["semio-framework-os-kernel"], this.repoRoot, this.invocation, ["--lib", "os_directory::io::binary::checkpoint_pair::tests::", "--no-fail-fast", ...rest, "--", "--nocapture"]);
  }
}

export class DirectoryRuntimeSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-directory-runtime-source accepts no arguments");
    const { testDirectoryRuntimeIdentityFixture } = await import("../../../../🔨️modules/📇️directory/🧪️tests/🪪️runtime-identity/🟦️.ts");
    testDirectoryRuntimeIdentityFixture();
  }
}

/** 🪪️ Proves the broker-visible identity is canonically bound to one exact server session. */
export async function directorySessionAuthorityOracle(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🪪️session-authority-v1");
  const schema = JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🪪️session-authority-v1/🔣️.json"), "utf8"));
  const validate: SchemaCheck = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(schema);
  const contract = await import("@semio-tech/framework/directory/io/text/session-authority-v1");
  for (const row of fixture.rows) {
    assert.equal(validate(row.value), row.accepted, `${row.id}: ${JSON.stringify(validate.errors)}`);
    let accepted = true;
    try { contract.parseDirectorySessionAuthorityJsonV1(JSON.stringify(row.value)); } catch { accepted = false; }
    assert.equal(accepted, row.accepted, `${row.id}: TypeScript`);
  }
  for (const row of fixture.raw) {
    let accepted = true;
    try { contract.parseDirectorySessionAuthorityJsonV1(row.source); } catch { accepted = false; }
    assert.equal(accepted, row.accepted, `${row.id}: canonical`);
  }
  for (const row of fixture.bindingGoldens) {
    const hash = createHash("sha256");
    const length = (value: string) => { const bytes = Buffer.alloc(4); bytes.writeUInt32BE(Buffer.byteLength(value)); return bytes; };
    const generation = Buffer.alloc(8);
    const expiresAt = Buffer.alloc(8);
    generation.writeBigUInt64BE(BigInt(row.authorizationGeneration));
    expiresAt.writeBigInt64BE(BigInt(row.expiresAt));
    hash.update("semio/hub/directory-event-page/session-binding/v1\0");
    hash.update(length(row.sessionId));
    hash.update(row.sessionId);
    hash.update(length(row.userId));
    hash.update(row.userId);
    hash.update(generation);
    hash.update(expiresAt);
    assert.equal(hash.digest("hex"), row.sessionBindingSha256, row.id);
  }
  for (const row of fixture.authorityLifecycle) {
    assert(["installed", "retained", "replaced", "ignored", "retired"].includes(row.outcome), `${row.id}: outcome`);
    assert.equal(row.epochDelta === 1, row.retireMountedAuthorities, `${row.id}: retirement epoch`);
    if (row.outcome === "replaced" || row.outcome === "retired") assert.equal(row.retainIndeterminateJobs, true, `${row.id}: uncertain jobs`);
    if (!row.brokerAdmissionCurrent) assert.equal(row.outcome, "ignored", `${row.id}: stale admission`);
  }
  const rust = readFileSync(join(root, "🦀️.rs"), "utf8");
  const client = readModuleTreeSource(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs"), { crates: false });
  const sessionIo = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/📝️text/🪪️session-authority-v1/🦀️.rs"), "utf8");
  assert(rust.includes("pub struct DirectorySessionAuthorityV1") && sessionIo.includes("parse_canonical_json"), "Rust session authority contract missing");
  assert(client.includes("DirectorySessionAuthorityV1::parse_canonical_json") && client.includes("DIRECTORY_SESSION_AUTHORITY_MAX_BYTES"), "Rust client does not enforce canonical session authority");
  return fixture.rows.length + fixture.raw.length + fixture.bindingGoldens.length + fixture.authorityLifecycle.length + 2;
}

export class DirectorySessionAuthorityCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("directory-session-authority-check accepts only --native");
    const checks = await directorySessionAuthorityOracle(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, ...exactCargoStageEnvironments(), groups: [{
          package: "semio-framework-os-kernel",
          target: { kind: "lib" },
          laws: [
            "os_directory::schema::tests::directory_session_authority_v1_matches_neutral_corpus_and_binding_goldens",
            "os_directory::client::tests::session_authority_client_preserves_canonical_binding_and_rejects_reordered_body",
            "os_directory::schema::tests::inference_current_hub_wire_preserves_required_nullable_hash",
            "os_directory::schema::tests::inference_indeterminate_lifecycle_matches_neutral_corpus",
            "os_directory::client::tests::inference_client_refuses_substituted_hub_receipt_and_page_coordinates",
          ],
        }] });
      console.log(`directory-session-authority-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`directory-session-authority-check: checks=${checks} clean`);
  }
}

/** 📃️ Proves the shared event-page envelope against an independent JSON Schema and SHA-256 oracle. */
export async function directoryEventPageContractOracle(repoRoot: string): Promise<number> {
  const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory/📃️event-page-v1.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json"), "utf8"));
  const validator: SchemaCheck = semioSchemaAjvV1({ strict: false, allErrors: true }).compile({ $defs: schema.$defs, $ref: "#/$defs/DirectoryEventPageV1" });
  assert(validator(fixture.valid), JSON.stringify(validator.errors));
  assert.equal(new TextEncoder().encode(fixture.canonicalUnsigned).length, 474);
  assert.equal(createHash("sha256").update(fixture.canonicalUnsigned).digest("hex"), fixture.expectedReceiptSha256);
  const contract = await import("@semio-tech/framework/directory/io/text");
  const parsed = await contract.parseDirectoryEventPageV1(JSON.stringify(fixture.valid));
  assert.deepEqual(parsed, fixture.valid);
  const setPath = (value: any, path: string, replacement: unknown): any => {
    const copy = structuredClone(value);
    const parts = path.split(".");
    let parent = copy;
    for (const part of parts.slice(0, -1)) parent = parent[Number.isInteger(Number(part)) ? Number(part) : part];
    parent[parts.at(-1)!] = replacement;
    return copy;
  };
  for (const hostile of fixture.hostileMutations) await assert.rejects(() => contract.parseDirectoryEventPageV1(JSON.stringify(setPath(fixture.valid, hostile.path, hostile.value))), hostile.name);
  const canonical = JSON.stringify(fixture.valid);
  await assert.rejects(() => contract.parseDirectoryEventPageV1(`${canonical} `), "trailing-byte");
  await assert.rejects(() => contract.parseDirectoryEventPageV1(canonical.replace('{"schema":', '{"schema":"duplicate","schema":')), "duplicate-key");
  const rust = readModuleTreeSource(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs"));
  const pageIo = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/📝️text/🦀️.rs"), "utf8");
  assert(rust.includes("pub struct DirectoryEventPageV1") && pageIo.includes("pub fn receipt_matches(&self) -> bool"), "Rust event-page contract missing");
  return 5 + fixture.hostileMutations.length + fixture.rawHostiles.length;
}

/** 🔌️ Proves both directory clients preserve one canonical event-page response and its bounded header. */
export async function directoryEventPageClientOracle(repoRoot: string): Promise<number> {
  const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory/📃️event-page-v1.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json"), "utf8"));
  const validator: SchemaCheck = semioSchemaAjvV1({ strict: false, allErrors: true }).compile({ $defs: schema.$defs, $ref: "#/$defs/DirectoryEventPageV1" });
  const canonical = JSON.stringify(fixture.valid);
  const accept = (raw: string, after: number) => {
    if (!Number.isSafeInteger(after) || after < 0 || new TextEncoder().encode(raw).byteLength > 65_536) throw new Error("client admission");
    const parsed = JSON.parse(raw);
    if (JSON.stringify(parsed) !== raw || !validator(parsed)) throw new Error("client admission");
    if (parsed.afterSeqExclusive !== after) throw new Error("frontier substitution");
    const { receiptSha256, ...unsigned } = parsed;
    if (createHash("sha256").update(JSON.stringify(unsigned)).digest("hex") !== receiptSha256) throw new Error("receipt substitution");
    return { canonicalJson: raw, throughSeqInclusive: parsed.throughSeqInclusive, receiptSha256: parsed.receiptSha256 };
  };
  const page = accept(canonical, 3);
  assert.equal(page.canonicalJson, canonical);
  assert.equal(page.throughSeqInclusive, fixture.valid.throughSeqInclusive);
  assert.equal(page.receiptSha256, fixture.expectedReceiptSha256);
  assert.throws(() => accept("x".repeat(65_537), 0));
  assert.throws(() => accept(canonical, 4));
  assert.throws(() => accept(canonical, -1));
  assert.throws(() => accept(`${canonical} `, 3));
  const osRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🟦️.ts");
  const typescript = readModuleTreeSource(osRoot);
  const method = sliceModuleTreeSource(osRoot, "async eventPage(", "stream(since");
  assert(
    method.includes("response.text()") && method.includes("parseDirectoryEventPageV1(canonicalJson)") && method.includes("page.afterSeqExclusive !== after") && !method.includes("response.json()"),
    "TypeScript canonical page transport is incomplete",
  );
  assert(typescript.includes("streamAcknowledged(since:") && typescript.includes("acknowledge: (through: number)"), "TypeScript acknowledged directory frontier is missing");
  const rust = readModuleTreeSource(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs"), { crates: false });
  assert(rust.includes("pub async fn event_page") && rust.includes("CanonicalDirectoryEventPageV1") && rust.includes("DIRECTORY_EVENT_PAGE_MAX_BYTES"), "Rust canonical page transport is incomplete");
  assert(rust.includes("pub fn stream_acknowledged") && rust.includes("pub fn acknowledge(&mut self, through: u64)"), "Rust acknowledged directory frontier is missing");
  assert(rust.includes("pub struct DirectoryEventPageBootstrapV1") && rust.includes("pub enum DirectoryBootstrapTransition"), "Rust directory bootstrap owner is missing");
  return 11;
}

export class DirectoryEventPageContractCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("directory-event-page-contract-check accepts only --native");
    const checks = await directoryEventPageContractOracle(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, ...exactCargoStageEnvironments(), groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: ["os_directory::schema::tests::directory_event_page_v1_matches_language_neutral_receipt_and_rejects_hostiles"] }], progress(event) {
          console.log(`directory-event-page-contract ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        } });
      console.log(`directory-event-page-contract-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`directory-event-page-contract-check: checks=${checks} clean`);
  }
}

export class DirectoryEventPageClientCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("directory-event-page-client-check accepts only --native");
    const checks = await directoryEventPageClientOracle(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, ...exactCargoStageEnvironments(), groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: ["os_directory::client::tests::directory_event_page_preserves_canonical_bytes_bounds_and_cancels_before_io"] }], progress(event) {
          console.log(`directory-event-page-client ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        } });
      console.log(`directory-event-page-client-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`directory-event-page-client-check: checks=${checks} clean`);
  }
}

/** 🧭️ Proves fetch, exact Home ACK, next-page, and live-cursor ordering independently of either shell. */
export function directoryEventPageBootstrapOracle(repoRoot: string): number {
  const trace = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory/🚀️event-page-bootstrap-v1.json"), "utf8"));
    
  
  let cursor = trace.initialAfter;
  let pending: any = null;
  let live = false;
  const present = (page: any): void => {
    assert(!pending && !live && page.afterSeqExclusive === cursor && page.throughSeqInclusive >= cursor);
    pending = page;
  };
  const acknowledge = (page: any, epoch = trace.bootstrapEpoch): "fetch" | "live" => {
    assert(pending && epoch === trace.bootstrapEpoch);
    for (const key of ["receiptSha256", "sessionBindingSha256", "authorizationGeneration", "throughSeqInclusive"]) assert.equal(page[key], pending[key]);
    cursor = pending.throughSeqInclusive;
    const hasMore = pending.hasMore;
    pending = null;
    live = !hasMore;
    return hasMore ? "fetch" : "live";
  };
  present(trace.pages[0]);
  assert.throws(() => present(trace.pages[1]), "page 2 before ACK");
  assert.throws(() => acknowledge({ ...trace.pages[0], receiptSha256: "d".repeat(64) }), "forged ACK");
  assert.equal(cursor, trace.initialAfter);
  assert.equal(acknowledge(trace.pages[0]), "fetch");
  present(trace.pages[1]);
  assert.throws(() => acknowledge(trace.pages[1], trace.bootstrapEpoch + 1), "stale epoch");
  assert.equal(acknowledge(trace.pages[1]), "live");
  for (const wakeup of trace.wakeups) assert(wakeup > cursor && cursor === trace.expectedSocketSince);
  const worker = readModuleTreeSource(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"));
  assert(worker.includes("DirectoryEventPageBootstrapV1") && worker.includes("directory-bootstrap-ack") && worker.includes("directory-event-page"), "browser worker bootstrap owner missing");
  return 11;
}

export class DirectoryEventPageBootstrapCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length) throw new Error("directory-event-page-bootstrap-check accepts no arguments");
    console.log(`directory-event-page-bootstrap-check: checks=${directoryEventPageBootstrapOracle(this.repoRoot)} clean`);
  }
}

/** 🧱️ Runs the retained owned fixture law against its independent oracle. */
export class DirectoryLeaseFixtureScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-directory-lease-fixture accepts no arguments");
    await runBudgetedTestCommand(process.execPath, ["test", join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🔏️document-execution-target-lease-v1/🟦️.ts")], { cwd: this.repoRoot, env: process.env, budgetMs: 15_000, throwOnFailure: true });
  }
}
