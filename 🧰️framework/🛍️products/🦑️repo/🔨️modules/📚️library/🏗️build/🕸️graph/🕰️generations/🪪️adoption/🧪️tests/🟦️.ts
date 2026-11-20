import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import Ajv from "ajv";
import { createFleetRegistryV1, ensureFleetIdentityV1, fleetRegistrationPathV1, signFleetDocumentV1 } from "../../../../🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import { mirrorWholeGenerationAdoptionReceiptV1, publishWholeGenerationAdoptionReceiptV1, recoverWholeGenerationAdoptionReceiptV1, stageWholeGenerationAdoptionIntentV1, verifyWholeGenerationAdoptionReceiptV1, wholeGenerationAdoptionReceiptPathV1 } from "../🟦️.ts";
import type { RestoreExpectedWholeGenerationOutcomeV1 } from "../../🔌️load/🟨️.mjs";
import fixture from "./🔣️.json";
import schema from "../🧬️schema/🔣️.json";

const exact = { origin: "producer", generation: 15, manifest: "b".repeat(64), scope: "all", identity: "c".repeat(64), implementation: "d".repeat(64), platform: "linux-x64" };
const expected = { origin: "receiver", head: exact };
const label = schema.definitions.Policy.const.label;

async function privateFleet(run: (state: { folder: string; root: string; store: string; producer: string; registry: ReturnType<typeof createFleetRegistryV1> }) => Promise<void>) {
  const folder = mkdtempSync(join(process.env.SEMIO_GRAPH_ADOPTION_TEST_TMP ?? tmpdir(), "ga-"));
  try {
    const root = join(folder, "r"), producer = join(folder, "p"), store = join(folder, "s"), group = "g1";
    for (const [workspace, originId] of [[root, "receiver"], [producer, "producer"]] as const) {
      const machine = join(workspace, ".cache", "f", "machine.json");
      mkdirSync(dirname(machine), { recursive: true });
      writeFileSync(machine, JSON.stringify({ originId, groupId: group, storeRoot: store }));
      const { registration } = await ensureFleetIdentityV1({ root: workspace, originId, group });
      const path = fleetRegistrationPathV1(store, originId);
      mkdirSync(dirname(path), { recursive: true });
      writeFileSync(path, JSON.stringify(registration));
    }
    await run({ folder, root, store, producer, registry: createFleetRegistryV1({ storeRoot: store, group }) });
  } finally {
    rmSync(folder, { recursive: true, force: true });
  }
}

test("only a restored whole head signed by the receiving origin yields evidence", async () => {
  await privateFleet(async ({ folder, root, store, producer, registry }) => {
    using sqlite = new Database(":memory:");
    const judge = sqlite.query("SELECT ?1='exact' AND ?2='restored' AND ?3='receiver' AS accepted");
    const validator = new Ajv({ strict: false });
    validator.addSchema(schema, "receipt");
    const valid = validator.compile({ $ref: "receipt#/definitions/Envelope" });
    for (const [index, row] of fixture.entries()) {
      const localDirectory = join(folder, "d", String(index));
      const candidate = row.head === "self" ? { ...exact, origin: "receiver" } : row.head === "partial" ? { ...exact, scope: "a".repeat(64) } : exact;
      const head = row.head === "drift" ? { ...candidate, manifest: "a".repeat(64) } : candidate;
      const outcome = { status: row.result, head, generation: 4, localManifest: "e".repeat(64) } as RestoreExpectedWholeGenerationOutcomeV1;
      const receipt = await publishWholeGenerationAdoptionReceiptV1({ root: row.signer === "producer" ? producer : root, store, origin: expected.origin, localDirectory, candidate, outcome, now: () => 12500 });
      expect(judge.get(row.head, row.result, row.signer), row.name).toEqual({ accepted: Number(row.expected) });
      expect(receipt !== undefined, row.name).toBe(row.expected);
      if (!receipt) continue;
      const value = JSON.parse(readFileSync(wholeGenerationAdoptionReceiptPathV1(store, expected.origin), "utf8"));
      expect(valid(value)).toBe(true);
      expect(await verifyWholeGenerationAdoptionReceiptV1(registry, expected, value)).toEqual({ version: 1, destination: "receiver", head: exact, localGeneration: 4, localManifest: "e".repeat(64), completedMs: 12500 });
      expect(await verifyWholeGenerationAdoptionReceiptV1(registry, { ...expected, head: { ...exact, implementation: "a".repeat(64) } }, value)).toBeUndefined();
      expect(await verifyWholeGenerationAdoptionReceiptV1(registry, expected, { ...value, receipt: { ...value.receipt, localGeneration: 5 } })).toBeUndefined();
      const text = JSON.stringify([1, "receiver", "producer", 15, exact.manifest, "all", exact.identity, exact.implementation, exact.platform, 4, "e".repeat(64), 12500]);
      const other = signFleetDocumentV1(producer, "cache-read", label, text)!;
      const otherRole = signFleetDocumentV1(root, "cache-write", label, text)!;
      expect(await verifyWholeGenerationAdoptionReceiptV1(registry, expected, { ...value, signature: other.signature })).toBeUndefined();
      expect(await verifyWholeGenerationAdoptionReceiptV1(registry, expected, { ...value, signature: otherRole.signature })).toBeUndefined();
      const registration = JSON.parse(readFileSync(fleetRegistrationPathV1(store, expected.origin), "utf8"));
      const key = await crypto.subtle.importKey("spki", Buffer.from(registration.keys["cache-read"], "base64url"), "Ed25519", false, ["verify"]);
      expect(await crypto.subtle.verify("Ed25519", key, Buffer.from(value.signature, "base64url"), new TextEncoder().encode(`semio-fleet-v1-document\n${label}\n${text}`))).toBe(true);
    }
  });
});

test("a local receipt survives loss of the mirror and can be replayed without changing its bytes", async () => {
  await privateFleet(async ({ folder, root, store, registry }) => {
    const localDirectory = join(folder, "local"), unreachable = join(folder, "file");
    writeFileSync(unreachable, "unchanged");
    const outcome = { status: "restored", head: exact, generation: 4, localManifest: "e".repeat(64) } as const;
    const receipt = await publishWholeGenerationAdoptionReceiptV1({ root, store: unreachable, origin: expected.origin, localDirectory, candidate: exact, outcome });
    expect(receipt).toBeDefined();
    expect(readFileSync(unreachable, "utf8")).toBe("unchanged");
    const path = join(localDirectory, schema.definitions.Policy.const.filename);
    const local = readFileSync(path, "utf8");
    expect(await mirrorWholeGenerationAdoptionReceiptV1({ root, store, origin: expected.origin, localDirectory })).toBe(true);
    const mirrored = wholeGenerationAdoptionReceiptPathV1(store, expected.origin);
    expect(readFileSync(mirrored, "utf8")).toBe(local);
    expect(await verifyWholeGenerationAdoptionReceiptV1(registry, expected, JSON.parse(local))).toEqual(receipt);
    writeFileSync(mirrored, "{");
    expect(await mirrorWholeGenerationAdoptionReceiptV1({ root, store, origin: expected.origin, localDirectory })).toBe(true);
    expect(readFileSync(mirrored, "utf8")).toBe(local);
    const value = JSON.parse(local);
    value.receipt.localGeneration = 19;
    writeFileSync(path, JSON.stringify(value));
    expect(await mirrorWholeGenerationAdoptionReceiptV1({ root, store, origin: expected.origin, localDirectory })).toBe(false);
    expect(readFileSync(mirrored, "utf8")).toBe(local);
  });
});

test("an interrupted commit recovers only from the exact published seal", async () => {
  await privateFleet(async ({ folder, root, store, registry }) => {
    const local = { generation: 4, localManifest: "e".repeat(64) };
    const localDirectory = join(folder, "local"), location = { root, store, origin: expected.origin, localDirectory };
    const receiptPath = join(localDirectory, schema.definitions.Policy.const.filename);
    await stageWholeGenerationAdoptionIntentV1({ ...location, candidate: exact, local });
    expect(() => readFileSync(receiptPath)).toThrow();
    expect(await mirrorWholeGenerationAdoptionReceiptV1(location)).toBe(false);
    for (const wrong of [{ ...local, generation: 3 }, { ...local, localManifest: exact.manifest }]) {
      expect(await recoverWholeGenerationAdoptionReceiptV1({ ...location, local: wrong })).toBeUndefined();
      expect(() => readFileSync(receiptPath)).toThrow();
    }
    const intentPath = join(localDirectory, schema.definitions.Policy.const.intentFilename), original = readFileSync(intentPath, "utf8");
    const changed = JSON.parse(original);
    changed.receipt.head.manifest = "a".repeat(64);
    writeFileSync(intentPath, JSON.stringify(changed));
    expect(await recoverWholeGenerationAdoptionReceiptV1({ ...location, local })).toBeUndefined();
    writeFileSync(intentPath, original);
    const recovered = await recoverWholeGenerationAdoptionReceiptV1({ ...location, local, now: () => 91300 });
    expect(recovered).toEqual({ version: 1, destination: "receiver", head: exact, localGeneration: 4, localManifest: local.localManifest, completedMs: 91300 });
    const receipt = readFileSync(receiptPath, "utf8");
    expect(await recoverWholeGenerationAdoptionReceiptV1({ ...location, local, now: () => 110000 })).toEqual(recovered);
    expect(readFileSync(receiptPath, "utf8")).toBe(receipt);
    expect(await mirrorWholeGenerationAdoptionReceiptV1(location)).toBe(true);
    const mirrored = JSON.parse(readFileSync(wholeGenerationAdoptionReceiptPathV1(store, expected.origin), "utf8"));
    expect(await verifyWholeGenerationAdoptionReceiptV1(registry, expected, mirrored)).toEqual(recovered);
  });
});
