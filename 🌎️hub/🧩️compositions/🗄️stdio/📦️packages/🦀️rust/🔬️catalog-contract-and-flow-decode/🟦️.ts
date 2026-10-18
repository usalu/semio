/** 🔬️ The catalog root contract tests and the flow retained decode script of the hub stdio package. */
import { requireIssuedResourceControlInvocationV1, BundleScript } from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { configuredExactCargoLawPolicyV1 } from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import assert from "node:assert/strict";
import {createRequire} from "node:module";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, truncateSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { decodePackValue, encodePackValue } from "../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️.ts";
import { devToolingEnv } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";
import { resolveWorkspaceBin } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts";
import { runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1 } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎯️exact/🟦️.ts";
import { cargoTargetDirectory, cargoBuildDirectory } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { ARTIFACT_MAX_BYTES, ARTIFACT_OWNER, CATALOG_DEADLINE_MS, COMPONENT_MANIFEST, DESCRIPTOR_JSON, DESCRIPTOR_PACK, IO_CHUNK_BYTES, PACKAGE_NAME } from "../🧮️component-constants/🟦️.ts";
import { assertComponentizableCore, assertRegularBounded, atomicDescriptorPair, catalogControlV1, type CatalogExecution, componentPackageId, copyCatalogArtifact, inspectWasmCoreStructure, prepareReleaseComponent, runControlled, webSha256 } from "../🧾️catalog-artifact-io/🟦️.ts";
import { catalogCargoEnvironment, requireEmptyFreshRoot } from "../🏗️catalog-root-script/🟦️.ts";

export async function runCatalogRootContractTests(root: string, repoRoot: string, execution:CatalogExecution): Promise<void> {
  requireIssuedResourceControlInvocationV1(execution.phase,execution.invocation);
  const fixtureRoot = join(root, "../..", "🧪️tests/🧫️fixtures", "🌳️catalog-root");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as {
    packageId: string;
    vectors: { raw: string; core: string; distinct: boolean }[];
    wasmStructures: { name: string; core: string; definedFunctions: number; componentizable: boolean }[];
    compilerCaches: { id: string; ambient: NodeJS.ProcessEnv; target: string; build: string }[];
    releaseComponents: { name: string; wat: string; export: string; result: number; definedFunctions: number }[];
  };

  const { default: Ajv } = await import("ajv");
  const catalogSchema = JSON.parse(readFileSync(join(root, "../../📇️catalog/🧬️schema/🔣️.json"), "utf8"));
  const validateCommitment = new Ajv({ strict: false }).compile({ ...catalogSchema, $ref: "#/$defs/NativeCatalogSurfaceCommitment" });
  if (!validateCommitment({ schema: "semio.stdio.artifact-catalog/v1", pluginId: "stdio", packageId: "semio:stdio", packageVersion: "0.1.0", definitions: [], codecs: [] })) throw new Error("authored empty catalog commitment is invalid");
  if (componentPackageId(join(root, COMPONENT_MANIFEST)) !== fixture.packageId) throw new Error("Cargo and neutral fixture package identities disagree");
  for (const vector of fixture.vectors) {
    const raw = await webSha256(Buffer.from(vector.raw, "hex"));
    const core = await webSha256(Buffer.from(vector.core, "hex"));
    if ((raw !== core) !== vector.distinct) throw new Error("catalog-root identity vector failed");
  }
  const testBase = process.env.SEMIO_TEST_ARTIFACT_DIR ? resolve(repoRoot, process.env.SEMIO_TEST_ARTIFACT_DIR) : tmpdir();
  mkdirSync(testBase, { recursive: true });
  const scratch = mkdtempSync(join(testBase, "stdio-catalog-root-contract-"));
  const active=catalogControlV1(execution);
  try {
    mkdirSync(join(scratch, ".cargo"));
    const config = '[build]\ntarget-dir = "shared-target"\nbuild-dir = "shared-build"\n';
    writeFileSync(join(scratch, ".cargo/config.toml"), config);
    const { parse: independentToml } = await import("@iarna/toml");
    if (!isDeepStrictEqual(Bun.TOML.parse(config), independentToml(config))) throw new Error("compiler cache config oracle differs");
    for (const row of fixture.compilerCaches) {
      const env = catalogCargoEnvironment(join(scratch, row.target), row.ambient);
      if (cargoTargetDirectory(scratch, env) !== join(scratch, row.target) || cargoBuildDirectory(scratch, env) !== join(scratch, row.build)) throw new Error(`${row.id}: staged compiler cache escapes its private target`);
    }
    const wasmOpt = resolveWorkspaceBin("wasm-opt", root);
    if (!wasmOpt) throw new Error("missing Binaryen wasm-opt workspace binary");
    const jco = await import("@bytecodealliance/jco");
    for (const vector of fixture.releaseComponents) {
      const input = join(scratch, `${vector.name}.input.wasm`);
      const output = join(scratch, `${vector.name}.release.wasm`);
      const original = await jco.parse(vector.wat);
      writeFileSync(input, original);
      await prepareReleaseComponent(input, output, repoRoot, devToolingEnv({}), active,execution);
      const optimized = readFileSync(output);
      if (!readFileSync(input).equals(Buffer.from(original))) throw new Error(`${vector.name}: optimization mutated its compiler input`);
      if (await jco.componentWit(original) !== await jco.componentWit(optimized)) throw new Error(`${vector.name}: optimization changed the component interface`);
      const modules = (await jco.metadataShow(optimized)).filter((entry) => entry.metaType.tag === "module");
      if (modules.length !== 1) throw new Error(`${vector.name}: expected one optimized core module`);
      const module=modules[0];assert(module,"Actual optimized component module required");const core = optimized.subarray(module.range[0],module.range[1]);
      if (assertComponentizableCore(core).definedFunctions !== vector.definedFunctions) throw new Error(`${vector.name}: unused functions survived release optimization`);
      const instance = new WebAssembly.Instance(new WebAssembly.Module(core));
      if ((instance.exports[vector.export] as () => number)() !== vector.result) throw new Error(`${vector.name}: native WebAssembly execution disagrees with the neutral result`);
      await prepareReleaseComponent(input, output, repoRoot, devToolingEnv({}), active,execution);
      if (!readFileSync(output).equals(optimized)) throw new Error(`${vector.name}: repeated release optimization changed the output`);
      const cancelledOutput = join(scratch, `${vector.name}.cancelled.wasm`);
      let cancelled = false;
      try { await prepareReleaseComponent(input, cancelledOutput, repoRoot, devToolingEnv({}), { cancelled: () => true, remainingMs: () => CATALOG_DEADLINE_MS },execution); } catch { cancelled = true; }
      if (!cancelled || existsSync(cancelledOutput)) throw new Error(`${vector.name}: cancellation left a release artifact`);
    }
    for (const vector of fixture.wasmStructures) {
      const bytes = Buffer.from(vector.core, "hex");
      const structure = inspectWasmCoreStructure(bytes);
      if (structure.definedFunctions !== vector.definedFunctions) throw new Error(`${vector.name} function-section count disagrees`);
      let componentizable = true;
      try { assertComponentizableCore(bytes); } catch { componentizable = false; }
      if (componentizable !== vector.componentizable || WebAssembly.validate(bytes) !== vector.componentizable) throw new Error(`${vector.name} parser agreement failed`);
      if (vector.componentizable) {
        const input = join(scratch, `${vector.name}.wasm`);
        const output = join(scratch, `${vector.name}.optimized.wasm`);
        writeFileSync(input, bytes);
        await runControlled(execution,wasmOpt,[input,"-o",output],root,devToolingEnv(),active);
        if (!WebAssembly.validate(readFileSync(output))) throw new Error(`${vector.name} Binaryen oracle produced invalid wasm`);
      }
    }
    for (const [name, cargo] of [
      ["duplicate-section", '[package.metadata.component]\npackage = "semio:stdio"\n[package.metadata.component]\npackage = "semio:stdio"\n'],
      ["duplicate-key", '[package.metadata.component]\npackage = "semio:stdio"\npackage = "semio:stdio"\n'],
      ["noncanonical", '[package.metadata.component]\npackage = "semio:Stdio"\n'],
    ] as const) {
      const path = join(scratch, `${name}.toml`);
      writeFileSync(path, cargo);
      let invalid = false;
      try { componentPackageId(path); } catch { invalid = true; }
      if (!invalid) throw new Error(`${name} Cargo component identity was accepted`);
    }
    const oversized = join(scratch, "oversized.wasm");
    writeFileSync(oversized, "");
    truncateSync(oversized, ARTIFACT_MAX_BYTES + 1);
    let rejected = false;
    try { copyCatalogArtifact(oversized, join(scratch, "oversized-copy.wasm"), "fixture", active); } catch { rejected = true; }
    if (!rejected || existsSync(join(scratch, "oversized-copy.wasm"))) throw new Error("oversized artifact did not fail without publication");
    const pairRoot = join(scratch, "pair");
    atomicDescriptorPair(pairRoot, Buffer.from("old-pack"), Buffer.from("old-json"));
    rejected = false;
    try { atomicDescriptorPair(pairRoot, Buffer.from("new-pack"), Buffer.from("new-json"), true); } catch { rejected = true; }
    if (!rejected || readFileSync(join(pairRoot, DESCRIPTOR_PACK), "utf8") !== "old-pack" || readFileSync(join(pairRoot, DESCRIPTOR_JSON), "utf8") !== "old-json") throw new Error("descriptor pair rollback failed");
    const rawOnly = join(scratch, "raw-only.wasm");
    writeFileSync(rawOnly, "raw");
    rejected = false;
    try { copyCatalogArtifact(rawOnly, join(scratch, "cancelled.wasm"), "fixture", { cancelled: () => true, remainingMs: () => CATALOG_DEADLINE_MS }); } catch { rejected = true; }
    if (!rejected || existsSync(join(scratch, "cancelled.wasm"))) throw new Error("cancelled artifact copy left a publication");
    rejected = false;
    try { copyCatalogArtifact(rawOnly, join(scratch, "deadline.wasm"), "fixture", { cancelled: () => false, remainingMs: () => 0 }); } catch { rejected = true; }
    if (!rejected || existsSync(join(scratch, "deadline.wasm"))) throw new Error("expired artifact copy left a publication");
    rejected = false;
    try { assertRegularBounded(join(scratch, "missing-core.wasm"), "core module"); } catch { rejected = true; }
    if (!rejected) throw new Error("missing core input was accepted");
    const staleRoot = join(scratch, "stale-root");
    mkdirSync(staleRoot);
    writeFileSync(join(staleRoot, "stale-raw.wasm"), "stale");
    rejected = false;
    try { requireEmptyFreshRoot(root, staleRoot); } catch { rejected = true; }
    if (!rejected) throw new Error("stale raw component satisfied the fresh-root contract");
    const changing = join(scratch, "changing.wasm");
    writeFileSync(changing, Buffer.alloc(IO_CHUNK_BYTES * 2, 7));
    rejected = false;
    try {
      copyCatalogArtifact(changing, join(scratch, "changing-copy.wasm"), "fixture", {
        cancelled: () => false,
        remainingMs: () => CATALOG_DEADLINE_MS,
        afterChunk(copied) { if (copied === IO_CHUNK_BYTES) truncateSync(changing, IO_CHUNK_BYTES); },
      });
    } catch { rejected = true; }
    if (!rejected || existsSync(join(scratch, "changing-copy.wasm"))) throw new Error("changing artifact did not fail without publication");
    const packed = encodePackValue(fixture);
    if (!isDeepStrictEqual(decodePackValue(packed), fixture)) throw new Error("independent Pack fixture round-trip failed");
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
}

export class FlowRetainedDecodeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    
    const leb:{encodeU32(value:number):Uint8Array}=createRequire(import.meta.url)("@webassemblyjs/leb128");
    const base = join(this.repoRoot, ARTIFACT_OWNER, "🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/💾️binary/📸️snapshot");
    const fixture = JSON.parse(readFileSync(join(base, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
    assert.deepEqual(fixture.limits, { nodes: 256, edges: 512, paramsPerNode: 64, stringBytes: 4096, totalStringBytes: 1048576 });
    const header = Buffer.concat([Buffer.from([137,83,69,77,13,10,26,10,24,0,0,0]), Buffer.from("stdio.semio.flow.pack v1")]);
    const decode = (hex: string): unknown => {
      const bytes = Buffer.from(hex, "hex"); let offset = 0; let strings = 0;
      const fail = (reason: string): never => { throw new Error(reason); };
      const byte = (): number => offset < bytes.length ? bytes[offset++]! : fail("malformed");
      const uint = (): number => {
        const start = offset; let value = 0n;
        for (let index = 0; index < 10; index++) {
          const next = byte(); if (index === 9 && next > 1) fail("malformed");
          value |= BigInt(next & 127) << BigInt(index * 7);
          if (next < 128) {
            if (index && next === 0) fail("malformed");
            if (value <= 0xffffffffn) assert.deepEqual(bytes.subarray(start, offset), Buffer.from(leb.encodeU32(Number(value))));
            return value > BigInt(Number.MAX_SAFE_INTEGER) ? Number.MAX_SAFE_INTEGER : Number(value);
          }
        }
        return fail("malformed");
      };
      const text = (): string => {
        const size = uint(); if (size > fixture.limits.stringBytes || strings + size > fixture.limits.totalStringBytes) fail("capacity");
        strings += size; if (offset + size > bytes.length) fail("malformed");
        let value: string; try { value = new TextDecoder("utf-8", { fatal: true }).decode(bytes.subarray(offset, offset + size)); } catch { return fail("malformed"); }
        offset += size; return value;
      };
      const count = (maximum: number): number => { const value = uint(); return value <= maximum ? value : fail("capacity"); };
      const number = (): number => { if (offset + 8 > bytes.length) fail("malformed"); const value = bytes.readDoubleLE(offset); offset += 8; return value; };
      for (const expected of header) if (byte() !== expected) fail("malformed");
      if (byte() !== 1) fail("malformed");
      const identity = text(); if (identity !== "stdio.semio.flow") fail("identity");
      const nodes = []; const nodeCount = count(fixture.limits.nodes);
      for (let index = 0; index < nodeCount; index++) {
        const id = text(), kind = text(), label = text(), params = []; const parameterCount = count(fixture.limits.paramsPerNode);
        for (let parameter = 0; parameter < parameterCount; parameter++) params.push({ key: text(), value: text() });
        nodes.push({ id, kind, label, params, position: { x: number(), y: number() } });
      }
      const edges = []; const edgeCount = count(fixture.limits.edges);
      for (let index = 0; index < edgeCount; index++) edges.push({ id: text(), from: { node: text(), port: text() }, to: { node: text(), port: text() }, kind: text() });
      if (offset !== bytes.length) fail("malformed");
      return { schema: identity, nodes, edges };
    };
    const encode = (snapshot: any): Buffer => {
      const chunks = [header, Buffer.from([1])];
      const count = (value: number): void => { chunks.push(Buffer.from(leb.encodeU32(value))); };
      const text = (value: string): void => { const bytes = Buffer.from(value); count(bytes.length); chunks.push(bytes); };
      text(snapshot.schema); count(snapshot.nodes.length);
      for (const node of snapshot.nodes) {
        for (const value of [node.id, node.kind, node.label]) text(value);
        count(node.params.length); for (const parameter of node.params) { text(parameter.key); text(parameter.value); }
        const point = new ArrayBuffer(16), view = new DataView(point); view.setFloat64(0, node.position.x, true); view.setFloat64(8, node.position.y, true); chunks.push(Buffer.from(point));
      }
      count(snapshot.edges.length); for (const edge of snapshot.edges) for (const value of [edge.id, edge.from.node, edge.from.port, edge.to.node, edge.to.port, edge.kind]) text(value);
      return Buffer.concat(chunks);
    };
    for (const row of fixture.valid) { assert.deepEqual(decode(row.hex), row.snapshot, row.id); assert.equal(encode(row.snapshot).toString("hex"), row.hex, row.id); }
    for (const row of fixture.invalid) assert.throws(() => decode(row.hex), (error: Error) => error.message === row.reason, row.id);
    console.log(`Flow retained decoder independent oracle: ${fixture.valid.length} exact wire snapshots, ${fixture.invalid.length} hostile denials; third-party LEB128 agrees`);
    const lifecycle = JSON.parse(readFileSync(join(base, "🧪️tests/🧫️fixtures/♻️lifecycle/🔣️.json"), "utf8"));
    assert.equal(new Set(lifecycle.admission.map((row: any) => row.id)).size, 5);
    for (const row of lifecycle.admission) {
      const reason = row.state === "closing" || row.state === "retired" ? "stale" : row.state === "unadmitted" ? "unsealed" : row.subset !== "flow" ? "identity" : null;
      assert.equal(reason, row.reason, row.id);
    }
    const large = structuredClone(fixture.valid.find((row: any) => row.id === lifecycle.multiPage.source).snapshot);
    large.nodes[lifecycle.multiPage.nodeIndex].label = lifecycle.multiPage.labelScalar.repeat(lifecycle.multiPage.labelRepeats);
    const largeWire = encode(large);
    assert.deepEqual(decode(largeWire.toString("hex")), large);
    const stringBytes = (value: any): number => typeof value === "string" ? Buffer.byteLength(value) : value && typeof value === "object" ? Object.values(value).reduce<number>((sum, field) => sum + stringBytes(field), 0) : 0;
    const identityBytes = [lifecycle.request.artifactId, lifecycle.request.artifactKind, lifecycle.request.standard, lifecycle.request.subset].reduce((sum: number, value: string) => sum + Buffer.byteLength(value), 0);
    const inputBytes = Buffer.from(leb.encodeU32(largeWire.length)).length + largeWire.length + 1;
    const typedBytes = stringBytes(large) + large.nodes.length * 16;
    assert.equal(largeWire.length, lifecycle.multiPage.wireBytes);
    assert.equal(inputBytes, lifecycle.multiPage.inputBytes);
    assert.equal(Math.ceil(inputBytes / 4096), lifecycle.multiPage.inputPages);
    assert.equal(identityBytes, lifecycle.request.identityBytes);
    assert.equal(typedBytes, lifecycle.multiPage.snapshotRetiredBytes);
    assert.equal(inputBytes + identityBytes + typedBytes, lifecycle.multiPage.totalRetiredBytes);
    console.log(`Flow lifecycle independent oracle: ${lifecycle.admission.length} exact admission states, ${lifecycle.multiPage.inputPages} input pages, ${lifecycle.multiPage.totalRetiredBytes} retained bytes; third-party encoding agrees`);
    const source = readFileSync(join(base, "🧪️tests/💾️binary/🦀️.rs"), "utf8");
    assert(source.includes("semio_flow_retained_snapshot_matches_neutral_wire_and_retains_failures"));
    assert(source.includes("semio_flow_retained_snapshot_rejects_retired_requests_and_closes_exact_bytes"), "retained Flow lifecycle native law is absent");
    if (segments.includes("--oracle-only")) return;
    const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), preparation: configuredRepositoryCargoPreparationPolicyV1(), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, groups: [{ package: PACKAGE_NAME, target: { kind: "test", name: "flow_retained_decode" }, laws: ["semio_flow_retained_snapshot_matches_neutral_wire_and_retains_failures", "semio_flow_retained_snapshot_rejects_retired_requests_and_closes_exact_bytes"] }] });
    assert.equal(receipts[0]!.assertions, 2);
  }
}
