import { expect, test } from "bun:test";
import Ajv from "ajv";
import deepEqual from "fast-deep-equal";
import { blake3 } from "@noble/hashes/blake3.js";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { CurrentPhysicalOwnerV1, type CurrentPhysicalPortV1 } from "../../../../../📁️filesystem/🧾️observation/📁️current/🟦️.ts";
import { writeCompletedCargoInvocationProvenanceV1 } from "../../🟦️.ts";
import {
  CARGO_PROVENANCE_PHYSICAL_BUDGET_V1,
  CARGO_PROVENANCE_RECEIPT_MAX_BYTES_V1,
  CargoProvenanceStreamV1,
  cargoDepInfoChecksumsV1,
  cargoDepInfoResolvedChecksumsV1,
  cargoDepInfoResolvedSourcesV1,
  cargoDepInfoSourcesV1,
  cargoProvenancePhysicalControlV1,
  decodeCargoProvenanceV1,
  encodeCargoProvenanceV1,
  type CargoProvenanceV1,
} from "../🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", import.meta.url), "utf8"));
const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
if (!artifacts) throw Error("Caller-owned SEMIO_TEST_ARTIFACT_DIR is required");
const admit = new Ajv({ strict: true }).compile(schema);
const sha256 = (value: Uint8Array | string): string => createHash("sha256").update(value).digest("hex");

test("the language-neutral fixture encodes to its exact documents and decodes back, validated by Ajv", async () => {
  expect(fixture.cases.length).toBeGreaterThan(2);
  for (const row of fixture.cases) {
    const document = JSON.parse(await encodeCargoProvenanceV1(row.observation));
    expect(deepEqual(document, row.document), row.name).toBe(true);
    expect(admit(row.document), row.name + JSON.stringify(admit.errors)).toBe(true);
    const decoded = decodeCargoProvenanceV1(JSON.stringify(row.document));
    expect(deepEqual(JSON.parse(JSON.stringify(decoded)), row.observation), row.name).toBe(true);
    expect(deepEqual(JSON.parse(await encodeCargoProvenanceV1(decoded)), row.document), row.name + " re-encodes identically").toBe(true);
  }
});

test("every refused fixture document fails both the portable schema and the decoder", () => {
  expect(fixture.refusals.length).toBeGreaterThan(5);
  for (const row of fixture.refusals) {
    expect(admit(row.document), row.name).toBe(row.schemaAdmits);
    expect(() => decodeCargoProvenanceV1(JSON.stringify(row.document)), row.name).toThrow("Cargo provenance receipt");
  }
  expect(() => decodeCargoProvenanceV1("{")).toThrow("not valid JSON");
  expect(() => decodeCargoProvenanceV1("[]")).toThrow("must be an object");
});

test("dep-info is kept as its parsed sources and rustc checksums, never as text", () => {
  for (const row of fixture.depInfo) {
    expect(cargoDepInfoSourcesV1(row.text), JSON.stringify(row.text)).toEqual(row.sources);
    expect(cargoDepInfoChecksumsV1(row.text), JSON.stringify(row.text)).toEqual(row.checksums);
  }
  const bytes = Buffer.from("pub fn owned() {}\n");
  const text = `owner.rlib: /w/a.rs\n# checksum:blake3=${Buffer.from(blake3(bytes)).toString("hex")} file_len:${bytes.length} a.rs\n`;
  const parsed = { baseDirectory: "/w", sources: cargoDepInfoSourcesV1(text), checksums: cargoDepInfoChecksumsV1(text) };
  expect(cargoDepInfoResolvedChecksumsV1(parsed)).toEqual([{ path: "/w/a.rs", blake3: Buffer.from(blake3(bytes)).toString("hex"), length: bytes.length }]);
  expect(cargoDepInfoResolvedSourcesV1(parsed)).toEqual(["/w/a.rs"]);
  expect(() => cargoDepInfoResolvedSourcesV1({ ...parsed, baseDirectory: null })).toThrow("no base directory");
});

test("decoded records share their interned inputs and witnesses instead of copying them", () => {
  const row = fixture.cases.find((value: { name: string }) => value.name.startsWith("resource-witnesses"));
  const decoded = decodeCargoProvenanceV1(JSON.stringify(row.document));
  expect(decoded.compilerResources[1]!.resources[0]).toBe(decoded.compilerResources[0]!.resources[0]);
  expect(decoded.compilerResources[1]!.resources[2]).toBe(decoded.compilerResources[0]!.resources[0]);
  expect(decoded.compilerResources[0]!.producerUnit).toBe(decoded.units[1]!.message);
  expect(decoded.compilerResources[0]!.callerUnit).toBe(decoded.units[0]!.message);
  expect(decoded.buildResources[0]!.resources[0]).toBe(decoded.compilerResources[0]!.resources[0]);
  const shared = fixture.cases.find((value: { name: string }) => value.name.startsWith("units-share"));
  const units = decodeCargoProvenanceV1(JSON.stringify(shared.document)).units;
  expect(units[1]!.inputs[1]).toBe(units[0]!.inputs[0]);
});

test("the stream refuses out-of-order sections and conflicting input observations", async () => {
  const sink = { write: async () => undefined };
  const header = fixture.cases[0].observation;
  const early = new CargoProvenanceStreamV1(sink);
  await expect(early.unit({ message: {}, observedAtMs: 1, depInfo: [], inputs: [], artifacts: [] })).rejects.toThrow("out of order");
  const stream = new CargoProvenanceStreamV1(sink);
  await stream.begin(header);
  await expect(stream.begin(header)).rejects.toThrow("out of order");
  await expect(stream.finish(1)).rejects.toThrow("out of order");
  const unit = { message: { name: "a" }, observedAtMs: 1, depInfo: [], artifacts: [] };
  await stream.unit({ ...unit, inputs: [{ path: "/w/a.rs", kind: "file", sha256: "1".repeat(64) }] });
  await expect(stream.unit({ ...unit, message: { name: "b" }, inputs: [{ path: "/w/a.rs", kind: "file", sha256: "2".repeat(64) }] })).rejects.toThrow("different digests");
  const witnessed = new CargoProvenanceStreamV1(sink);
  await witnessed.begin(header);
  await witnessed.buildScripts([]);
  await expect(witnessed.buildResource({ package_id: "p", out_dir: "/o", path: "/o/x", sha256: "1".repeat(64), observedAtMs: 1, resources: [{ input: { kind: "read", path: 7 } }] })).rejects.toThrow("must be a string");
});

test("the physical control is finite, owner-validated and bounded by the caller's own deadline", () => {
  const rows: unknown[] = [];
  const control = cargoProvenancePhysicalControlV1({ cancelled: () => false, remainingMs: () => 5, onProgress: (row) => rows.push(row) });
  expect(control.maxBytes).toBe(CARGO_PROVENANCE_PHYSICAL_BUDGET_V1.maxBytes);
  expect(control.maxWork).toBe(CARGO_PROVENANCE_PHYSICAL_BUDGET_V1.maxWork);
  expect(control.remainingMs()).toBeLessThanOrEqual(5);
  expect(cargoProvenancePhysicalControlV1({ cancelled: () => false, onProgress: () => undefined }).remainingMs()).toBeGreaterThan(CARGO_PROVENANCE_PHYSICAL_BUDGET_V1.deadlineMs - 5000);
  expect(() => new CurrentPhysicalOwnerV1(artifacts, control)).not.toThrow();
  expect(CARGO_PROVENANCE_RECEIPT_MAX_BYTES_V1).toBeLessThanOrEqual(128 * 1024 * 1024);
});

function syntheticInvocation(root: string, fileCount: number, unitCount: number) {
  const source = join(root, "src");
  mkdirSync(source, { recursive: true });
  const files: { path: string; bytes: Buffer }[] = [];
  for (let index = 0; index < fileCount; index++) {
    const path = join(source, `module-${index}.rs`);
    const bytes = Buffer.from(`pub const VALUE_${index}: u32 = ${index};\n`);
    writeFileSync(path, bytes);
    files.push({ path, bytes });
  }
  const manifest = join(root, "Cargo.toml");
  writeFileSync(manifest, '[workspace]\n[package]\nname="synthetic"\nversion="0.0.0"\n');
  const build = join(root, "build");
  mkdirSync(build, { recursive: true });
  let naive = 0;
  const units = [];
  for (let unit = 0; unit < unitCount; unit++) {
    const artifact = join(build, `libunit${unit}.rmeta`);
    writeFileSync(artifact, `artifact ${unit}`);
    const text = `${artifact}: ${files.map((file) => file.path).join(" ")}\n${files.map((file) => `# checksum:blake3=${Buffer.from(blake3(file.bytes)).toString("hex")} file_len:${file.bytes.length} ${file.path}`).join("\n")}\n`;
    writeFileSync(join(build, `libunit${unit}.d`), text);
    naive += text.length * 2 + files.length * 170;
    units.push({ message: { reason: "compiler-artifact", package_id: `path+file:///synthetic#unit${unit}@0.0.0`, manifest_path: manifest, target: { name: `unit${unit}`, kind: ["lib"], src_path: files[0]!.path }, filenames: [artifact] }, evidence: { version: 1, kind: "discovery", paths: [], producer: null } });
  }
  return { files, manifest, build, units, naive, invocation: { manifest, cwd: root, command: "cargo", args: ["build"], buildDirectory: build, builtAtMs: Date.now(), status: 0, cancelled: false, units, buildScripts: [] } };
}

const physicalFor = (root: string) => new CurrentPhysicalOwnerV1(root, cargoProvenancePhysicalControlV1({ cancelled: () => false, onProgress: () => undefined }));
const wrap = (port: CurrentPhysicalOwnerV1, overrides: Partial<CurrentPhysicalPortV1> = {}): CurrentPhysicalPortV1 => ({ control: port.control, read: (path) => port.read(path), digest: (path) => port.digest(path), directoryEntries: (path) => port.directoryEntries(path), checkpoint: () => port.checkpoint(), recheck: () => port.recheck(), ...overrides });

test("a large invocation is observed against real bytes and published as a small interned receipt, atomically", async () => {
  const root = mkdtempSync(join(artifacts, "w-"));
  const { files, units, naive, invocation } = syntheticInvocation(root, 800, 12);
  const receipt = join(root, "receipts", "cargo-unit-provenance-synthetic.json");
  const ledger = join(root, "build", "semio-cargo-provenance", "cargo-unit-provenance-synthetic.json");
  let unpublished = 0;
  const owner = physicalFor(root);
  const physical = wrap(owner, {
    recheck: async () => {
      expect(existsSync(receipt)).toBe(false);
      expect(existsSync(ledger)).toBe(false);
      const pending = readdirSync(join(root, "receipts")).filter((name) => name.endsWith(".tmp"));
      expect(pending.length).toBe(1);
      expect(() => JSON.parse(readFileSync(join(root, "receipts", pending[0]!), "utf8"))).not.toThrow();
      unpublished++;
      await owner.recheck();
    },
  });
  await writeCompletedCargoInvocationProvenanceV1(receipt, invocation, new Map(), join(root, "cargo-home"), physical);
  expect(unpublished).toBe(1);
  expect(readdirSync(join(root, "receipts")).filter((name) => name.endsWith(".tmp"))).toEqual([]);
  expect(readdirSync(join(root, "build", "semio-cargo-provenance"))).toEqual(["cargo-unit-provenance-synthetic.json"]);
  const bytes = readFileSync(receipt);
  expect(bytes.equals(readFileSync(ledger))).toBe(true);
  expect(bytes.length * 3).toBeLessThan(naive);
  const document = JSON.parse(bytes.toString("utf8"));
  expect(admit(document), JSON.stringify(admit.errors?.slice(0, 3))).toBe(true);
  expect(document.strings.length).toBeLessThan(files.length + 150);
  expect(document.inputs.length).toBeGreaterThanOrEqual(files.length);
  expect(document.inputs.length).toBeLessThan(files.length + 20);
  const observed = decodeCargoProvenanceV1(bytes.toString("utf8"));
  expect(observed.units.length).toBe(units.length);
  const first = observed.units[0]!;
  expect(first.inputs.length).toBe(files.length);
  expect(first.inputs.find((row) => row.path === files[5]!.path)!.sha256).toBe(sha256(files[5]!.bytes));
  expect(first.depInfo[0]!.sha256).toBe(sha256(readFileSync(first.depInfo[0]!.path)));
  expect(first.depInfo[0]!.baseDirectory).toBe(root);
  expect(first.depInfo[0]!.sources.length).toBe(files.length);
  expect(first.depInfo[0]!.checksums[7]).toEqual({ path: files[7]!.path, blake3: Buffer.from(blake3(files[7]!.bytes)).toString("hex"), length: files[7]!.bytes.length });
  expect(first.artifacts[0]!.sha256).toBe(sha256(readFileSync(String(first.message.filenames ? (first.message.filenames as string[])[0] : ""))));
  expect(observed.units[11]!.inputs[3]).toBe(first.inputs[3]);
  expect(JSON.stringify(observed)).not.toContain('"text"');
  expect(statSync(receipt).size).toBe(bytes.length);
  console.log(`[DEBUG] synthetic receipt ${bytes.length} bytes against ${naive} naive bytes; ${document.inputs.length} interned inputs for ${units.length * files.length} references`);
});

test("a refused or failed observation leaves neither a receipt nor a temporary file", async () => {
  const root = mkdtempSync(join(artifacts, "f-"));
  const { invocation } = syntheticInvocation(root, 20, 2);
  const receipt = join(root, "receipts", "cargo-unit-provenance-refused.json");
  const recheckFails = wrap(physicalFor(root), {
    recheck: async () => {
      throw Error("Current physical bytes changed before transfer");
    },
  });
  await expect(writeCompletedCargoInvocationProvenanceV1(receipt, invocation, new Map(), join(root, "cargo-home"), recheckFails)).rejects.toThrow("changed before transfer");
  expect(readdirSync(join(root, "receipts"))).toEqual([]);
  expect(existsSync(join(root, "build", "semio-cargo-provenance"))).toBe(false);
  const starved = new CurrentPhysicalOwnerV1(root, { ...cargoProvenancePhysicalControlV1({ cancelled: () => false, onProgress: () => undefined }), maxWork: 40 });
  await expect(writeCompletedCargoInvocationProvenanceV1(receipt, invocation, new Map(), join(root, "cargo-home"), starved)).rejects.toThrow();
  expect(readdirSync(join(root, "receipts"))).toEqual([]);
});

test("the receipt document is decoded from the file the writer published for a changed source", async () => {
  const root = mkdtempSync(join(artifacts, "c-"));
  const { files, invocation } = syntheticInvocation(root, 6, 1);
  const receipt = join(root, "receipt.json");
  await writeCompletedCargoInvocationProvenanceV1(receipt, { ...invocation, buildDirectory: null }, new Map(), join(root, "cargo-home"), physicalFor(root));
  const observed: CargoProvenanceV1 = decodeCargoProvenanceV1(readFileSync(receipt, "utf8"));
  const before = observed.units[0]!.inputs.find((row) => row.path === files[2]!.path)!.sha256;
  writeFileSync(files[2]!.path, "changed\n");
  expect(sha256(readFileSync(files[2]!.path))).not.toBe(before);
  expect(before).toBe(sha256(files[2]!.bytes));
  expect(observed.invocationInputs.some((row) => row.path === join(root, "Cargo.toml") && row.sha256 === sha256(readFileSync(join(root, "Cargo.toml"))))).toBe(true);
});
