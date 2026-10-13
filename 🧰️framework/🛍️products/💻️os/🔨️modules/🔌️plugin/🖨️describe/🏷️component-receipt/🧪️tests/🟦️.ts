import { describe, expect, it } from "bun:test";
import { createHash } from "node:crypto";
import { existsSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import Ajv from "ajv";
import { COMPONENT_PROFILE_POLICIES, ITERATE_COMPONENT_MAX_BYTES, SHIPPED_COMPONENT_MAX_BYTES, SHIPPED_COMPONENT_PROFILE, componentDeliverableV1, componentProfileV1, componentProvenanceDigestsV1, componentReceiptFindingsV1, encodeComponentReceiptV1, parseComponentReceiptV1, readComponentReceiptV1, rustToolchainIdentityFromVersionV1, writeComponentReceiptV1, type ComponentProfileV1, type ComponentProvenanceObservationV1, type ComponentReceiptV1 } from "../🟦️.ts";

const schema = JSON.parse(readFileSync(join(import.meta.dir, "../🧬️schema/🔣️.json"), "utf8"));
const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../🧫️fixtures/🔣️.json"), "utf8")) as {
  receipts: { id: string; valid: boolean; value: Record<string, unknown> }[];
  deliverables: { crateRoot: string; packageName: string; profile: ComponentProfileV1; directory: string; wasm: string; receipt: string }[];
  toolchain: { verbose: string; identity: string | null }[];
  provenance: { roots: { repository: string; build: string; cargoHome: string; home: string }; observed: ComponentProvenanceObservationV1; leaves: string[]; sourceTreeDigest: string; provenanceSha256: string };
};
const validate = new Ajv({ strict: false, allErrors: true }).compile(schema);
const text = (value: unknown): string => `${JSON.stringify(value, null, 2)}\n`;

describe("ComponentReceiptV1", () => {
  it("uses the installed, locked Ajv as the independent schema oracle", () => {
    let root = import.meta.dir;
    while (!existsSync(join(root, "bun.lock"))) root = dirname(root);
    expect(readFileSync(join(root, "bun.lock"), "utf8")).toContain(`ajv@${require("ajv/package.json").version}`);
  });

  for (const row of fixture.receipts) it(`admits ${row.id} exactly as the schema does`, () => {
    expect(Boolean(validate(row.value)), JSON.stringify(validate.errors)).toBe(row.valid);
    if (row.valid) expect(parseComponentReceiptV1(text(row.value))).toEqual(row.value as ComponentReceiptV1);
    else expect(() => parseComponentReceiptV1(text(row.value))).toThrow();
  });

  it("refuses every non-canonical encoding of a schema-valid receipt", () => {
    const valid = fixture.receipts.find((row) => row.id === "shipped-release")!.value;
    expect(Boolean(validate(valid))).toBe(true);
    expect(() => parseComponentReceiptV1(JSON.stringify(valid))).toThrow(/canonical/);
    expect(() => parseComponentReceiptV1(text(Object.fromEntries(Object.entries(valid).reverse())))).toThrow(/canonical/);
    expect(() => parseComponentReceiptV1(text(valid).replace('"pluginId": "note"', '"pluginId": "note", "pluginId": "note"'))).toThrow();
    expect(encodeComponentReceiptV1(valid as ComponentReceiptV1)).toBe(text(valid));
  });

  it("owns the profile policy in the schema: one shipped profile at 256 MiB, a local-only iterate cap at 512 MiB", () => {
    expect(SHIPPED_COMPONENT_PROFILE).toBe("wasm-release");
    expect(SHIPPED_COMPONENT_MAX_BYTES).toBe(256 * 1024 * 1024);
    expect(ITERATE_COMPONENT_MAX_BYTES).toBe(512 * 1024 * 1024);
    expect(Object.entries(COMPONENT_PROFILE_POLICIES).filter(([, policy]) => policy.shipped).map(([profile]) => profile)).toEqual(["wasm-release"]);
    expect(schema.$defs.ComponentProfile.enum).toEqual(Object.keys(COMPONENT_PROFILE_POLICIES));
    expect(() => componentProfileV1("wasm-dev")).toThrow();
    expect(() => componentProfileV1("release")).toThrow();
  });

  for (const row of fixture.deliverables) it(`places the ${row.profile} deliverable of ${row.packageName}`, () => {
    expect(componentDeliverableV1(row.crateRoot, row.packageName, row.profile)).toEqual({ profile: row.profile, directory: row.directory, wasm: row.wasm, receipt: row.receipt });
  });

  for (const row of fixture.toolchain) it(`projects rustc -vV onto ${row.identity ?? "a refusal"}`, () => {
    if (row.identity === null) expect(() => rustToolchainIdentityFromVersionV1(row.verbose)).toThrow();
    else expect(rustToolchainIdentityFromVersionV1(row.verbose)).toBe(row.identity);
  });

  it("derives the provenance digests and Merkle root the independent Python oracle derived, independent of checkout location", () => {
    const { observed, roots } = fixture.provenance, expected = { sourceTreeDigest: fixture.provenance.sourceTreeDigest, provenanceSha256: fixture.provenance.provenanceSha256 };
    expect(componentProvenanceDigestsV1(observed, roots)).toEqual(expected);
    const moved = { repository: "/elsewhere/semio", build: "/elsewhere/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build", cargoHome: "/users/other/.cargo", home: "/users/other" };
    const relocate = (path: string): string => path.replace(roots.repository, moved.repository).replace(roots.home, moved.home);
    const relocated = { units: observed.units.map((unit) => ({ inputs: unit.inputs.map((row) => ({ ...row, path: relocate(row.path) })), artifacts: unit.artifacts.map((row) => ({ ...row, path: relocate(row.path) })) })), buildResources: observed.buildResources.map((run) => ({ ...run, path: relocate(run.path), resources: run.resources.map((witness) => ({ ...witness, input: { ...witness.input, path: relocate(String(witness.input.path)) } })) })) };
    expect(componentProvenanceDigestsV1(relocated, moved)).toEqual(expected);
    const changed = { ...observed, units: observed.units.map((unit, index) => index === 0 ? { ...unit, inputs: unit.inputs.map((row, position) => position === 1 ? { ...row, sha256: "9".repeat(64) } : row) } : unit) };
    expect(componentProvenanceDigestsV1(changed, roots).sourceTreeDigest).not.toBe(expected.sourceTreeDigest);
    expect(componentProvenanceDigestsV1({ ...observed, units: [...observed.units].reverse() }, roots)).toEqual(expected);
  });

  it("names every disagreement between a receipt and the bytes it is read beside", () => {
    const wasm = new TextEncoder().encode("\0asm\r\0\x01\0component"), pack = new TextEncoder().encode("semantic descriptor");
    const sha = (bytes: Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
    const receipt = { ...(fixture.receipts[0]!.value as ComponentReceiptV1), wasmSha256: sha(wasm), wasmBytes: wasm.byteLength, semanticDescriptorSha256: sha(pack) };
    const expected = { pluginId: "note", packageId: "semio:note", profile: "wasm-release" as const };
    expect(componentReceiptFindingsV1(receipt, expected, wasm, pack)).toEqual([]);
    expect(componentReceiptFindingsV1(receipt, { ...expected, profile: "wasm-prototype" }, wasm, pack)).toHaveLength(1);
    expect(componentReceiptFindingsV1(receipt, expected, wasm.subarray(1), pack)).toHaveLength(1);
    expect(componentReceiptFindingsV1(receipt, expected, wasm, wasm)).toHaveLength(1);
    expect(componentReceiptFindingsV1(receipt, { ...expected, pluginId: "gis" }, wasm, pack)).toHaveLength(1);
  });

  it("writes atomically and reads back only regular canonical files", () => {
    const directory = mkdtempSync(join(tmpdir(), "component-receipt-"));
    try {
      const path = join(directory, "🧾️receipt.json"), receipt = fixture.receipts[0]!.value as ComponentReceiptV1;
      writeComponentReceiptV1(path, receipt);
      expect(readComponentReceiptV1(path)).toEqual(receipt);
      expect(readFileSync(path, "utf8")).toBe(text(receipt));
      expect(() => writeComponentReceiptV1(path, { ...receipt, wasmBytes: 1 })).toThrow();
      expect(readComponentReceiptV1(path)).toEqual(receipt);
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});
