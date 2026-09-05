#!/usr/bin/env bun
/** @emoji ⚙️ Runs the `semio-framework-ui-contract` test suite and the guest-target compile gates.
 *
 * The wasm gates are the point of this crate: the contract is what `wasm32-wasip2` plugin components
 * and `wasm32-unknown-unknown` browser renderers both speak, so a dependency that fails either target
 * is a design error, not a build error. Native `cargo check` cannot see that — it never compiles
 * `#[cfg(target_arch = "wasm32")]` code — which is why these run on every acceptance. */
import { basename, dirname, join, relative } from "node:path";
import { tmpdir } from "node:os";
import { fileURLToPath } from "node:url";
import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { BundleScript, ScriptRouter, buildBudgetMs, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runExactCargoLaws, runCmd, runCmdStatus } from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { testBuiltTreeRetirementFixture } from "../../♻️retirement/🌲️built/📜️script.ts";

const packageRoot = import.meta.dir ?? dirname(fileURLToPath(import.meta.url));

//#region 📋️PagedListOracle
export function fixedListStorageSelfTests(): number {
  const fixture = JSON.parse(readFileSync(new URL("../../📋️list/🧫️fixture/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../📋️list/🧬️schema.json", import.meta.url), "utf8"));
  const Ajv = createRequire(import.meta.url)("ajv");
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  const ordered = Buffer.alloc(fixture.ordered.count * 8);
  for (let index = 0; index < fixture.ordered.count; index++) ordered.writeBigUInt64LE(BigInt(index), index * 8);
  let sum = 0n;
  for (let index = 0; index < fixture.ordered.count; index++) sum += ordered.readBigUInt64LE(index * 8);
  assert.equal(sum, BigInt(fixture.ordered.sum));
  assert.equal(ordered.readBigUInt64LE(0), BigInt(fixture.ordered.first));
  assert.equal(ordered.readBigUInt64LE(ordered.byteLength - 8), BigInt(fixture.ordered.last));
  assert(Buffer.alloc(fixture.binding.elementBytes).byteLength > fixture.binding.smallGrantBytes);
  assert(Buffer.alloc(fixture.oversized.elementBytes).byteLength > fixture.maximumGrantBytes);
  assert.equal(Buffer.alloc(fixture.edgeCases.zeroCapacity).byteLength, 0);
  assert.equal(Array.from({ length: fixture.edgeCases.zeroSizedCount }, () => null).length, 7);
  const retained = ordered.subarray(0, fixture.edgeCases.retainedPrefix * 8);
  assert.equal(retained.readBigUInt64LE(retained.byteLength - 8), 511n);
  assert.equal(ordered.byteLength - retained.byteLength, (fixture.edgeCases.tailCount - fixture.edgeCases.retainedPrefix) * 8);
  for (const bits of [32, 64]) {
    const maximum = (1n << BigInt(bits - 1)) - 1n;
    const counter = Buffer.alloc(8);
    counter.writeBigUInt64LE(maximum);
    assert(counter.readBigUInt64LE() + 384n > maximum);
  }
  assert.equal(Buffer.alloc(384 * fixture.counter.allocatorMultiplier).byteLength, 768);
  assert.equal(validate({ ...fixture, fanout: 32 }), false);
  assert.equal(validate({ ...fixture, retirement: { ...fixture.retirement, releasesLivePayload: true } }), false);
  const copy = JSON.parse(readFileSync(new URL("../../🔗️bindings/📋️copy/🧫️fixture/🔣️.json", import.meta.url), "utf8"));
  const copySchema = JSON.parse(readFileSync(new URL("../../🔗️bindings/📋️copy/🧬️schema.json", import.meta.url), "utf8"));
  const validateCopy = new Ajv({ strict: true, allErrors: true }).compile(copySchema);
  assert(validateCopy(copy), JSON.stringify(validateCopy.errors));
  const bindingBytes = Buffer.alloc(copy.elementBytes);
  assert(bindingBytes.byteLength <= copy.grantBytes && bindingBytes.byteLength > copy.smallGrantBytes);
  const expected = Array.from({ length: copy.count }, (_, index) => ({ trigger: "activate", action: { scope: copy.scope, name: `action-${index}`, version: 1 } }));
  assert.deepEqual(JSON.parse(JSON.stringify(expected)).slice(0, 3).map((binding: typeof expected[number]) => binding.action.name), copy.names);
  assert.equal(validateCopy({ ...copy, heldReaderSurvives: false }), false);
  const componentCopy = JSON.parse(readFileSync(new URL("../../🪞️copy/🧫️fixture/🔣️.json", import.meta.url), "utf8"));
  const componentCopySchema = JSON.parse(readFileSync(new URL("../../🪞️copy/🧬️schema.json", import.meta.url), "utf8"));
  const validateComponentCopy = new Ajv({ strict: true, allErrors: true }).compile(componentCopySchema);
  assert(validateComponentCopy(componentCopy), JSON.stringify(validateComponentCopy.errors));
  const components = JSON.parse(readFileSync(new URL("../../♻️retirement/🌳️typed/🧩️components.json", import.meta.url), "utf8"));
  assert.equal(new Set(components.cases.map((row: { component: { type: string } }) => row.component.type)).size, componentCopy.componentCount);
  const textBytes = Buffer.from(componentCopy.text.repeat(componentCopy.textRepeats));
  assert.equal(textBytes.byteLength, 512);
  assert.equal(Buffer.concat(Array(componentCopy.listItems * 2).fill(textBytes)).byteLength, 32768);
  assert.equal(Buffer.alloc(componentCopy.allocationGrant * componentCopy.allocatorMultiplier).byteLength, 65536);
  assert.equal(Buffer.alloc(componentCopy.allocationGrant).byteLength / componentCopy.runtimeWorkGrant, 8);
  assert.equal(validateComponentCopy({ ...componentCopy, partialCandidateReadable: true }), false);
  assert.equal(validateComponentCopy({ ...componentCopy, allocationGrant: 65536 }), false);
  const comparison = JSON.parse(readFileSync(new URL("../../⚖️compare/🧫️fixture/🔣️.json", import.meta.url), "utf8"));
  const comparisonSchema = JSON.parse(readFileSync(new URL("../../⚖️compare/🧬️schema.json", import.meta.url), "utf8"));
  const validateComparison = new Ajv({ strict: true, allErrors: true }).compile(comparisonSchema);
  assert(validateComparison(comparison), JSON.stringify(validateComparison.errors));
  for (const row of comparison.cases) assert.equal(Buffer.from(JSON.stringify(row.left)).equals(Buffer.from(JSON.stringify(row.right))), row.equal);
  assert.equal(validateComparison({ ...comparison, differentBackingMayBeEqual: false }), false);
  assert.equal(validateComparison({ ...comparison, contendedProgress: true }), false);
  const frameOracle = Buffer.alloc(comparison.frame.bytes);
  frameOracle.writeUInt16LE(comparison.frame.pageCount - 1, 0);
  frameOracle.writeUInt16LE(comparison.frame.maximumTextBytes * 2, 4);
  assert.equal(frameOracle.toString("hex"), comparison.frame.littleEndian);
  assert.equal(frameOracle.readUInt16LE(4), comparison.frame.maximumPosition);
  assert.equal(validateComparison({ ...comparison, frame: { ...comparison.frame, bytes: 32 } }), false);
  const documentComparison = JSON.parse(readFileSync(new URL("../../⚖️compare/📃️document/🧫️fixture/🔣️.json", import.meta.url), "utf8"));
  const documentComparisonSchema = JSON.parse(readFileSync(new URL("../../⚖️compare/📃️document/🧬️schema.json", import.meta.url), "utf8"));
  const validateDocumentComparison = new Ajv({ strict: true, allErrors: true }).compile(documentComparisonSchema);
  assert(validateDocumentComparison(documentComparison), JSON.stringify(validateDocumentComparison.errors));
  const nodeIds = Buffer.alloc(documentComparison.nodeIds.length * 8);
  documentComparison.nodeIds.forEach((id: number, index: number) => nodeIds.writeBigUInt64LE(BigInt(id), index * 8));
  assert.deepEqual(documentComparison.nodeIds.map((_: number, index: number) => Number(nodeIds.readBigUInt64LE(index * 8))), documentComparison.wireOrder);
  assert.equal(Buffer.from(documentComparison.text.repeat(documentComparison.textRepeats)).byteLength, 512);
  assert.equal(validateDocumentComparison({ ...documentComparison, copiesOldComponent: true }), false);
  assert.equal(validateDocumentComparison({ ...documentComparison, waitsForLiveDocumentOnCancel: true }), false);
  assert.equal(validateDocumentComparison({ ...documentComparison, frames: 1 }), false);
  const wholePatch = JSON.parse(readFileSync(new URL("../../♻️retirement/🩹️patch/📨️pending/📦️whole/🧫️fixture/🔣️.json", import.meta.url), "utf8"));
  const wholePatchSchema = JSON.parse(readFileSync(new URL("../../♻️retirement/🩹️patch/📨️pending/📦️whole/🧬️schema.json", import.meta.url), "utf8"));
  const validateWholePatch = new Ajv({ strict: true, allErrors: true }).compile(wholePatchSchema);
  assert(validateWholePatch(wholePatch), JSON.stringify(validateWholePatch.errors));
  assert.equal(Buffer.from(wholePatch.surface).byteLength, wholePatch.surfaceBytes);
  assert.equal(validateWholePatch({ ...wholePatch, includesEmptyBacking: false }), false);
  assert.equal(validateWholePatch({ ...wholePatch, readableAfterCloseStarts: true }), false);
  const assembly = JSON.parse(readFileSync(new URL("../../📃️document/🎟️assembly/🧫️fixture/🔣️.json", import.meta.url), "utf8"));
  const assemblySchema = JSON.parse(readFileSync(new URL("../../📃️document/🎟️assembly/🧬️schema.json", import.meta.url), "utf8"));
  const validateAssembly = new Ajv({ strict: true, allErrors: true }).compile(assemblySchema);
  assert(validateAssembly(assembly), JSON.stringify(validateAssembly.errors));
  const assemblyIds = Buffer.alloc(assembly.nodeIds.length * 8);
  assembly.nodeIds.forEach((id: number, index: number) => assemblyIds.writeBigUInt64LE(BigInt(id), index * 8));
  assert.equal(assemblyIds.toString("hex"), assembly.wireHex);
  assert.equal(Buffer.from(assembly.surface).byteLength, 6);
  assert.equal(Buffer.concat([assemblyIds.subarray(0, 8), assemblyIds.subarray(0, 8)]).byteLength, assembly.comparisonBytesPerIdentity);
  for (const field of ["zeroGrantAllocates", "copyOldRoot", "contentionWaits"]) assert.equal(validateAssembly({ ...assembly, [field]: true }), false);
  assert.equal(validateAssembly({ ...assembly, duplicateRetainsInput: false }), false);
  const resident = JSON.parse(readFileSync(new URL("../../🎟️resident/🧫️fixture/🔣️.json", import.meta.url), "utf8"));
  const residentSchema = JSON.parse(readFileSync(new URL("../../🎟️resident/🧬️schema.json", import.meta.url), "utf8"));
  const validateResident = new Ajv({ strict: true, allErrors: true }).compile(residentSchema);
  assert(validateResident(resident), JSON.stringify(validateResident.errors));
  const residentBytes = Buffer.alloc(8);
  residentBytes.writeBigUInt64LE(BigInt(resident.surfaceBytes) * 4n);
  assert.equal(Number(residentBytes.readBigUInt64LE()), resident.aggregateBytes);
  let mask = resident.rootOwner | resident.outputOwner;
  assert.deepEqual(resident.returnOrder.map((owner: number) => { mask &= ~owner; return mask === 0 ? resident.smallBytes : 0; }), resident.returnedBytes);
  assert(resident.smallReservations * resident.smallBytes < resident.aggregateBytes);
  for (const field of ["dropWaits", "reusesBeforeFinalOwner", "duplicateReturnAfterExplicitClose"]) assert.equal(validateResident({ ...resident, [field]: true }), false);
  assert.equal(validateResident({ ...resident, aggregateBytes: resident.aggregateBytes * 2 }), false);
  const residentFixed = JSON.parse(readFileSync(new URL("../../🎟️resident/🗃️fixed/🧪️fixture/🔣️.json", import.meta.url), "utf8"));
  const residentFixedSchema = JSON.parse(readFileSync(new URL("../../🎟️resident/🗃️fixed/🧬️schema.json", import.meta.url), "utf8"));
  const validateResidentFixed = new Ajv({ strict: true, allErrors: true }).compile(residentFixedSchema);
  assert(validateResidentFixed(residentFixed), JSON.stringify(validateResidentFixed.errors));
  const fixedArithmetic = residentFixed.arithmetic;
  residentBytes.writeBigUInt64LE(BigInt(fixedArithmetic.contract) + BigInt(fixedArithmetic.runtime) + BigInt(fixedArithmetic.payload));
  assert.equal(Number(residentBytes.readBigUInt64LE()), fixedArithmetic.admitted);
  assert.equal(Number(residentBytes.readBigUInt64LE() - BigInt(fixedArithmetic.payload)), fixedArithmetic.final);
  for (const field of ["finalOwnerReleasesStatic", "repeatRegistrationChargesAgain", "changedRegistrationAccepted", "zeroGrantMutates"]) assert.equal(validateResidentFixed({ ...residentFixed, [field]: true }), false);
  assert.equal(validateResidentFixed({ ...residentFixed, staticCountsAgainstAggregate: false }), false);
  const residentRoot = JSON.parse(readFileSync(new URL("../../🎟️resident/🌳️root/🧫️fixture/🔣️.json", import.meta.url), "utf8"));
  const residentRootSchema = JSON.parse(readFileSync(new URL("../../🎟️resident/🌳️root/🧬️schema.json", import.meta.url), "utf8"));
  const validateResidentRoot = new Ajv({ strict: true, allErrors: true }).compile(residentRootSchema);
  assert(validateResidentRoot(residentRoot), JSON.stringify(validateResidentRoot.errors));
  assert.equal(Buffer.from(residentRoot.surface).toString("hex"), residentRoot.surfaceUtf8);
  assert.equal(Buffer.byteLength(residentRoot.payload), residentRoot.payloadUtf8Bytes);
  const rootId = Buffer.alloc(8); rootId.writeBigUInt64LE(BigInt(residentRoot.rootId));
  assert.equal(rootId.toString("hex"), residentRoot.wireHex);
  for (const field of ["separateLedger", "reusesBeforeFinalReader", "dropWaits"]) assert.equal(validateResidentRoot({ ...residentRoot, [field]: true }), false);
  assert.equal(residentRoot.pressureRoots * residentRoot.pressureReservationBytes, residentRoot.pressureAggregateBytes);
  assert.equal(validateResidentRoot({ ...residentRoot, slotReuseRequiresTypedTerminal: false }), false);
  for (const order of residentRoot.outputOrders) {
    let pending = 3;
    assert.deepEqual(order.map((owner: number) => { pending &= ~owner; return pending === 0 ? residentRoot.sealedBytes : 0; }), [0, 32768]);
  }
  assert.equal(validateResidentRoot({ ...residentRoot, shrinkAfterSplit: true }), false);
  return 75;
}
//#endregion 📋️PagedListOracle

//#region 🔖️test
class BuiltTreeRetirementScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    testBuiltTreeRetirementFixture();
    if (segments.length === 1 && segments[0] === "--oracle-only") return;
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot, cargoArgs: segments, buildBudgetMs: 3_600_000,
      groups: [{ package: "semio-framework-ui-contract", target: { kind: "lib" }, laws: [
        "built_tree_retirement_closes_all_typed_fields_and_preserves_foreign_values",
        "built_tree_retirement_closes_full_page_chain_beyond_observer_depth",
        "built_child_retirement_contention_retains_exact_page",
        "built_tree_retirement_preserves_foreign_queued_page_at_full_capacity",
      ] }],
    });
    console.log(`[DEBUG] built-tree exact native laws: ${receipts.reduce((sum, receipt) => sum + receipt.assertions, 0)} executed`);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    console.log(`[DEBUG] fixed-list-page-oracle checks=${fixedListStorageSelfTests()}`);
    await runCargoTestBudgeted([], packageRoot, ["--all-features", ...rest]);
  }
}
//#endregion 🔖️test

//#region 🔖️conformance
/** 🔍️ Validates the language-neutral corpus catalog with Ajv and exact filesystem ownership. */
export function conformanceCorpusSelfTests(): number {
  const root = join(packageRoot, "../../📚️examples/🧪️conformance");
  const catalog = JSON.parse(readFileSync(join(root, "📇️catalog.json"), "utf8")) as { version: number; roles: { snapshot: string; expect: string; patch: string }; groups: Record<string, { patch: boolean; cases: Record<string, string> }> };
  const Ajv = createRequire(import.meta.url)("ajv");
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "🧬️catalog.schema.json"), "utf8")));
  assert(validate(catalog), JSON.stringify(validate.errors));
  assert(!validate({ ...catalog, roles: { ...catalog.roles, snapshot: "snapshot.json" } }));
  assert.deepEqual(readdirSync(root).sort(), [...Object.keys(catalog.groups), "📇️catalog.json", "🧬️catalog.schema.json"].sort());
  let count = 0;
  for (const [group, definition] of Object.entries(catalog.groups)) {
    assert.equal(new Set(Object.values(definition.cases)).size, Object.keys(definition.cases).length);
    assert.deepEqual(readdirSync(join(root, group)).sort(), Object.values(definition.cases).sort());
    for (const [id, directory] of Object.entries(definition.cases)) {
      const roles = [catalog.roles.snapshot, catalog.roles.expect, ...(definition.patch ? [catalog.roles.patch] : [])];
      assert.deepEqual(readdirSync(join(root, group, directory)).sort(), roles.sort());
      assert.equal(JSON.parse(readFileSync(join(root, group, directory, catalog.roles.expect), "utf8")).case, id);
      count++;
    }
  }
  assert.equal(count, 62);
  return count;
}

/** @emoji 🧪️ Runs only `🔬️conformance.rs`'s corpus harness — every fixture under
 * `📚️examples/🧪️conformance/` deserializes, validates/patches through this crate's own
 * `validate_snapshot`/`apply_patch`, and matches its declarative expectation. Same test binary as
 * `test`, filtered to the `conformance::` module path so iterating on the corpus does not pay for the
 * whole crate's suite. */
class ConformanceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    console.log(`[DEBUG] conformance-corpus-catalog cases=${conformanceCorpusSelfTests()}`);
    await runCargoTestBudgeted([], packageRoot, ["--all-features", ...rest, "--", "conformance::"]);
  }
}
//#endregion 🔖️conformance

//#region 🔖️check-wasm
/** @emoji 🌐️ Both guest flavours: wasip2 (plugin components) and unknown-unknown (browser renderers). */
class CheckWasmScript extends BundleScript {
  run(): void {
    const check = (args: string[]) => runCmd("cargo", ["check", "-p", "semio-framework-ui-contract", ...args], { cwd: packageRoot, budgetMs: buildBudgetMs() });
    check(["--target", "wasm32-wasip2"]);
    check(["--target", "wasm32-unknown-unknown"]);
    check(["--target", "wasm32-wasip2", "--features", "typegen"]);
  }
}
//#endregion 🔖️check-wasm

//#region 🔖️typegen
const TYPEGEN_TEST_NAME = "typegen_export";

function generatedUiContractPath(root: string): string {
  return join(root, "..", "..", "..", "..", "🛂️manifest", "🤖️generated", "📜️ui-contract.ts");
}

/** 🧬️ Runs the owned schema export test, optionally writing its deterministic projection. */
function runTypegenExportTest(root: string, outPath?: string): void {
  const env = outPath === undefined ? process.env : { ...process.env, SEMIO_TYPEGEN_OUT: outPath };
  const status = runCmdStatus("cargo", ["test", "-p", "semio-framework-ui-contract", "--features", "typegen", "--test", TYPEGEN_TEST_NAME], {
    cwd: root,
    env,
    budgetMs: buildBudgetMs(),
  });
  if (status !== 0) {
    console.error("ui-contract typegen: owned schema export failed — see output above.");
    process.exit(status);
  }
}

class GenerateScript extends BundleScript {
  run(_segments: string[]): void {
    const outPath = generatedUiContractPath(this.root);
    mkdirSync(join(this.root, "..", "..", "..", "..", "🛂️manifest", "🤖️generated"), { recursive: true });
    runTypegenExportTest(this.root, outPath);
    console.log(`ui-contract typescript mirror refreshed -> ${outPath}`);
  }
}

/** 🧾️ Runs the exact schema exporter outside the workspace and emits its canonical output bytes. */
class PreviewGeneratedScript extends BundleScript {
  run(_segments: string[]): void {
    const targetPath = generatedUiContractPath(this.root);
    const temp = mkdtempSync(join(tmpdir(), "semio-ui-contract-typegen-"));
    let content: Buffer;
    try {
      const outPath = join(temp, basename(targetPath));
      const result = Bun.spawnSync(["cargo", "test", "--locked", "-p", "semio-framework-ui-contract", "--features", "typegen", "--test", TYPEGEN_TEST_NAME], { cwd: this.root, env: { ...process.env, CARGO_TARGET_DIR: join(temp, "target"), SEMIO_TYPEGEN_OUT: outPath }, stderr: "pipe", stdout: "pipe" });
      if (result.exitCode !== 0) throw new Error(`ui-contract preview export failed: ${result.stderr.toString()}`);
      content = readFileSync(outPath);
    } finally {
      rmSync(temp, { recursive: true, force: true });
    }
    const nodes = [{ bytesBase64: content.toString("base64"), mode: 0o644, nodeKind: "file" as const, path: relative(this.repoRoot, targetPath).replaceAll("\\", "/").normalize("NFC") }];
    process.stdout.write(`${JSON.stringify({ contractId: "ui-contract", nodes, schemaVersion: 1, staleRemovals: [] })}\n`);
  }
}

/** 🔎️ Validates metadata and byte-compares the owned projection with the committed mirror. */
class CheckScript extends BundleScript {
  run(_segments: string[]): void {
    runTypegenExportTest(this.root);
    console.log("ui-contract typescript mirror is fresh.");
  }
}
//#endregion 🔖️typegen

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir)
    .register("test", TestScript)
    .register("built-tree-retirement-check", BuiltTreeRetirementScript)
    .register("conformance", ConformanceScript)
    .register("check-wasm", CheckWasmScript)
    .register("generate", GenerateScript)
    .register("preview-generated", PreviewGeneratedScript)
    .register("check", CheckScript);
  await runBundleScriptMain(router, import.meta.url);
}
