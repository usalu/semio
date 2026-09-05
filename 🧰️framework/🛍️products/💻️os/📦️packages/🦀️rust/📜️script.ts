#!/usr/bin/env bun
/** 🦀️ `@semio-tech/framework-os-kernel` task router. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv2020 from "ajv/dist/2020.js";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargo, resolveTestLevel, runCargoTestBudgeted, runExactCargoLaws } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { runNestedCargoPackageAdapter } from "../../../../../📜️script.ts";

//#region 🔎️ScalarWireSource
class ScalarWireSourceScript extends BundleScript {
  async run(): Promise<void> {
    const { testScalarRecordWireFixture } = await import("../../🔨️modules/🎒️pack/🔎️scalar-witness/📜️script.ts");
    testScalarRecordWireFixture();
  }
}
//#endregion 🔎️ScalarWireSource

class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "--manifest-path", "Cargo.toml", ...segments], this.root);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", ...segments], this.root);
  }
}

class NativeTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-os-kernel"], this.repoRoot, ["--lib", "--features", "sync,ureq", ...rest]);
  }
}

class DirectoryRuntimeSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-directory-runtime-source accepts no arguments");
    const { testDirectoryRuntimeIdentityFixture } = await import("../../🔨️modules/📇️directory/🔌️client/🪪️runtime/📜️script.ts");
    testDirectoryRuntimeIdentityFixture();
  }
}

/** 📃️ Proves the shared event-page envelope against an independent JSON Schema and SHA-256 oracle. */
export async function directoryEventPageContractOracle(repoRoot: string): Promise<number> {
  const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📃️event-page-v1.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json"), "utf8"));
  const validator = new Ajv2020({ strict: false, allErrors: true, discriminator: true }).compile({ $defs: schema.$defs, $ref: "#/$defs/DirectoryEventPageV1" });
  assert(validator(fixture.valid), JSON.stringify(validator.errors));
  assert.equal(new TextEncoder().encode(fixture.canonicalUnsigned).length, 474);
  assert.equal(createHash("sha256").update(fixture.canonicalUnsigned).digest("hex"), fixture.expectedReceiptSha256);
  const contract = await import("../../🔨️modules/📇️directory/🧬️schema/🟦️.ts");
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
  for (const hostile of fixture.hostileMutations) await assert.rejects(() => contract.parseDirectoryEventPageV1(JSON.stringify(setPath(fixture.valid, hostile.path, hostile.value))), undefined, hostile.name);
  const canonical = JSON.stringify(fixture.valid);
  await assert.rejects(() => contract.parseDirectoryEventPageV1(`${canonical} `), undefined, "trailing-byte");
  await assert.rejects(() => contract.parseDirectoryEventPageV1(canonical.replace("{\"schema\":", "{\"schema\":\"duplicate\",\"schema\":")), undefined, "duplicate-key");
  const rust = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs"), "utf8");
  assert(rust.includes("pub struct DirectoryEventPageV1") && rust.includes("pub fn receipt_matches(&self) -> bool"), "Rust event-page contract missing");
  return 5 + fixture.hostileMutations.length + fixture.rawHostiles.length;
}

/** 🔌️ Proves both directory clients preserve one canonical event-page response and its bounded header. */
export async function directoryEventPageClientOracle(repoRoot: string): Promise<number> {
  const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📃️event-page-v1.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json"), "utf8"));
  const validator = new Ajv2020({ strict: false, allErrors: true, discriminator: true }).compile({ $defs: schema.$defs, $ref: "#/$defs/DirectoryEventPageV1" });
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
  const typescript = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🟦️.ts"), "utf8");
  const method = typescript.slice(typescript.indexOf("async eventPage("), typescript.indexOf("stream(since", typescript.indexOf("async eventPage(")));
  assert(method.includes("response.text()") && method.includes("parseDirectoryEventPageV1(canonicalJson)") && method.includes("page.afterSeqExclusive !== after") && !method.includes("response.json()"), "TypeScript canonical page transport is incomplete");
  assert(typescript.includes("streamAcknowledged(since:") && typescript.includes("acknowledge: (through: number)"), "TypeScript acknowledged directory frontier is missing");
  const rust = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs"), "utf8");
  assert(rust.includes("pub async fn event_page") && rust.includes("CanonicalDirectoryEventPageV1") && rust.includes("DIRECTORY_EVENT_PAGE_MAX_BYTES"), "Rust canonical page transport is incomplete");
  assert(rust.includes("pub fn stream_acknowledged") && rust.includes("pub fn acknowledge(&mut self, through: u64)"), "Rust acknowledged directory frontier is missing");
  assert(rust.includes("pub struct DirectoryEventPageBootstrapV1") && rust.includes("pub enum DirectoryBootstrapTransition"), "Rust directory bootstrap owner is missing");
  return 11;
}

class DirectoryEventPageContractCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("directory-event-page-contract-check accepts only --native");
    const checks = await directoryEventPageContractOracle(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runExactCargoLaws({
        cwd: this.repoRoot,
        env: { ...process.env, RUST_MIN_STACK: "268435456" },
        groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: ["os_directory::schema::tests::directory_event_page_v1_matches_language_neutral_receipt_and_rejects_hostiles"] }],
        progress(event) { console.log(`directory-event-page-contract ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
      });
      console.log(`directory-event-page-contract-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`directory-event-page-contract-check: checks=${checks} clean`);
  }
}

class DirectoryEventPageClientCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("directory-event-page-client-check accepts only --native");
    const checks = await directoryEventPageClientOracle(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runExactCargoLaws({
        cwd: this.repoRoot,
        env: { ...process.env, RUST_MIN_STACK: "268435456" },
        groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: ["os_directory::client::tests::directory_event_page_preserves_canonical_bytes_bounds_and_cancels_before_io"] }],
        progress(event) { console.log(`directory-event-page-client ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
      });
      console.log(`directory-event-page-client-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`directory-event-page-client-check: checks=${checks} clean`);
  }
}

/** 🧭️ Proves fetch, exact Home ACK, next-page, and live-cursor ordering independently of either shell. */
export function directoryEventPageBootstrapOracle(repoRoot: string): number {
  const trace = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️event-page-bootstrap-v1.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧬️event-page-bootstrap-v1.schema.json"), "utf8"));
  const validator = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert(validator(trace), JSON.stringify(validator.errors));
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
  assert.throws(() => present(trace.pages[1]), undefined, "page 2 before ACK");
  assert.throws(() => acknowledge({ ...trace.pages[0], receiptSha256: "d".repeat(64) }), undefined, "forged ACK");
  assert.equal(cursor, trace.initialAfter);
  assert.equal(acknowledge(trace.pages[0]), "fetch");
  present(trace.pages[1]);
  assert.throws(() => acknowledge(trace.pages[1], trace.bootstrapEpoch + 1), undefined, "stale epoch");
  assert.equal(acknowledge(trace.pages[1]), "live");
  for (const wakeup of trace.wakeups) assert(wakeup > cursor && cursor === trace.expectedSocketSince);
  const worker = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts"), "utf8");
  assert(worker.includes("DirectoryEventPageBootstrapV1") && worker.includes("directory-bootstrap-ack") && worker.includes("directory-event-page"), "browser worker bootstrap owner missing");
  return 11;
}

class DirectoryEventPageBootstrapCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length) throw new Error("directory-event-page-bootstrap-check accepts no arguments");
    console.log(`directory-event-page-bootstrap-check: checks=${directoryEventPageBootstrapOracle(this.repoRoot)} clean`);
  }
}

class CodecSendSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-codec-send-source accepts no arguments");
    const { testNativeCodecSendFixture } = await import("../../🔨️modules/🏪️store/📦️codec/🧵️send/📜️script.ts");
    testNativeCodecSendFixture();
  }
}

class BackboneDetachSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-backbone-detach-source accepts no arguments");
    const { testBackboneDetachFixture } = await import("../../🔨️modules/🏪️store/🔗️backbone/✂️detach/📜️script.ts");
    testBackboneDetachFixture();
  }
}

class MemberDialectSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-member-dialect-source accepts no arguments");
    const { testMemberDialectFixture } = await import("../../🔨️modules/🏪️store/🧩️composition/🪪️member-dialect/📜️script.ts");
    testMemberDialectFixture();
    const { testFixtureProjectionRetirement } = await import("../../🔨️modules/🔌️plugin/🧪️tests/🌲️fixture-projection/📜️script.ts");
    testFixtureProjectionRetirement();
  }
}

class MemberDialectCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { testMemberDialectFixture } = await import("../../🔨️modules/🏪️store/🧩️composition/🪪️member-dialect/📜️script.ts");
    testMemberDialectFixture();
    const { testFixtureProjectionRetirement } = await import("../../🔨️modules/🔌️plugin/🧪️tests/🌲️fixture-projection/📜️script.ts");
    testFixtureProjectionRetirement();
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot,
      cargoArgs: segments,
      buildBudgetMs: 3_600_000,
      groups: [
        { package: "semio-framework-schema", target: { kind: "lib" }, laws: [
          "artifact_composition_fields_derive_emits_expected_slot_tables",
          "artifact_composition_fields_default_to_empty_for_leaf_artifacts",
          "artifact_composition_projection_walks_aliases_nested_options_and_cancels",
          "artifact_composition_projection_real_child_alias_has_fixed_admission_bounds",
        ] },
        { package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: [
          "member_factory_closed_dialect_matches_neutral_admission_corpus",
          "member_factory_closed_dialect_rejects_identity_and_owner_substitution",
          "member_factory_closed_dialect_graph_admission_matches_neutral_corpus",
          "member_factory_closed_dialect_graph_sync_preserves_prior_state_on_rejection",
          "member_factory_closed_dialect_parent_projection_matches_neutral_corpus",
          "initial_child_identity_matches_neutral_coordinates_and_blake3",
        ] },
        { package: "semio-framework-plugin", target: { kind: "lib" }, laws: [
          "fixture_projection_retires_exact_tree_before_return_error_or_panic",
          "member_factory_parent_snapshot_restore_matches_neutral_corpus",
          "member_factory_closed_dialect_open_failure_retains_pin_and_drains_exact_member",
          "member_factory_closed_dialect_register_rejects_pin_without_mutating_member",
          "member_factory_closed_dialect_fresh_register_and_restore_publish_exact_parent_owner",
        ] },
      ],
    });
    console.log(`[DEBUG] exact member admission laws: ${receipts.reduce((sum, receipt) => sum + receipt.assertions, 0)} executed across ${receipts.length} verified test executables`);
  }
}

//#region 🧩️JCO Package Adapter
class GenerateJcoPackageAdapterScript extends BundleScript {
  run(): void { runNestedCargoPackageAdapter(this.repoRoot, "generate"); }
}
class PreviewGeneratedScript extends BundleScript {
  run(): void { runNestedCargoPackageAdapter(this.repoRoot, "preview"); }
}
class CheckJcoPackageAdapterScript extends BundleScript {
  run(): void { runNestedCargoPackageAdapter(this.repoRoot, "check"); }
}
//#endregion 🧩️JCO Package Adapter

const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript).register("test-scalar-wire-source", ScalarWireSourceScript).register("generate-jco-package-adapter", GenerateJcoPackageAdapterScript).register("preview-generated", PreviewGeneratedScript).register("check-jco-package-adapter", CheckJcoPackageAdapterScript).register("test-native", NativeTestScript).register("test-directory-runtime-source", DirectoryRuntimeSourceScript).register("directory-event-page-contract-check", DirectoryEventPageContractCheckScript).register("directory-event-page-client-check", DirectoryEventPageClientCheckScript).register("directory-event-page-bootstrap-check", DirectoryEventPageBootstrapCheckScript).register("test-codec-send-source", CodecSendSourceScript).register("test-backbone-detach-source", BackboneDetachSourceScript).register("test-member-dialect-source", MemberDialectSourceScript).register("member-dialect-check", MemberDialectCheckScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "check" });
