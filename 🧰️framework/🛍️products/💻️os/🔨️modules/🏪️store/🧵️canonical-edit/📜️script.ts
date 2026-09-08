/** 🧵️ Canonical edit contracts: schema-owned exports plus independent JSON/SHA-256/UTF-8 oracles. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

//#region 🧵️CanonicalEditOracle
const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
const sha256 = (value: string): string => createHash("sha256").update(value).digest("hex");

/** 🔏️ Proves the four canonical-edit fixtures against `os.store.canonical-edit` and independent encoders. */
export function testCanonicalEditFixtures(): void {
  const contract = read("./🧬️schema/🔣️.json");
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(contract);
  const exported = (name: string) => ajv.getSchema(`${contract.$id}#/$defs/${name}`)!;

  const reader = read("./🧪️fixtures/📖️canonical-reader.json");
  const sealer = read("./🧪️fixtures/🔏️canonical-edit-sealer.json");
  const borrowed = read("./🧪️fixtures/🗺️canonical-borrowed-map.json");
  const progress = read("./🧪️fixtures/🚧️canonical-error-progress.json");
  const validateReader = exported("CanonicalReader");
  const validateSealer = exported("CanonicalEditSealer");
  const validateBorrowed = exported("CanonicalBorrowedMap");
  const validateProgress = exported("CanonicalErrorProgress");
  assert(validateReader(reader), JSON.stringify(validateReader.errors));
  assert(validateSealer(sealer), JSON.stringify(validateSealer.errors));
  assert(validateBorrowed(borrowed), JSON.stringify(validateBorrowed.errors));
  assert(validateProgress(progress), JSON.stringify(validateProgress.errors));

  for (const [name, fixture] of [["canonical-edit-sealer", sealer], ["canonical-borrowed-map", borrowed]] as const) {
    assert.equal(JSON.stringify(fixture.edit), fixture.expectedJson, `${name} canonical JSON emits declaration order`);
    assert.match(fixture.expectedDigest, /^[a-f0-9]{64}$/);
    assert.notEqual(sha256(fixture.expectedJson), fixture.expectedDigest, `${name} seal digest is a prefixed preimage, not the bare document`);
  }

  assert.equal(reader.sourceFixture, "canonical-borrowed-map");
  assert.equal(reader.expectedByteLength, Buffer.byteLength(borrowed.expectedJson), "reader replays the borrowed-map document");
  assert.equal(reader.expectedJsonSha256, sha256(borrowed.expectedJson), "reader digest is the borrowed-map document digest");
  assert.deepEqual(reader.grants, [0, 1, 7, 4096]);
  assert.deepEqual(reader.lifecycle, ["zero-grant", "before-poll-cancel", "mid-key-cancel", "complete", "transfer-root", "close"]);
  assert.equal(reader.hostiles.length, 7);
  assert.equal(reader.expectedRootDrops, 1);

  assert.deepEqual(sealer.grants, [0, 1, 2, 7, 256, 4096]);
  assert.equal(sealer.hostile.length, 9);
  assert.deepEqual(sealer.largeTextBytes, [16384, 65536]);
  const sealedText: string = sealer.edit.forwards[0].Replace.text;
  const sealedFill = sealer.textPattern.repeat(sealer.repeat);
  assert(sealedText.startsWith(sealedFill), "the sealed text opens with the declared fill");
  assert(sealer.repeat > 4096, "the sealed fill crosses the one-page grant boundary");
  assert(/[\u0000-\u001f"\\]/.test(sealedText.slice(sealedFill.length)), "the sealed tail carries escape-forcing scalars");
  assert(sealer.expectedJson.includes(JSON.stringify(sealedText).slice(1, -1)), "escapes survive the canonical document");
  assert.deepEqual(sealer.origins.map((origin: { kind: string }) => origin.kind), ["owner", "contributed", "transaction"]);

  assert.deepEqual(borrowed.grants, [0, 1, 7, 256, 4096]);
  assert.equal(borrowed.hostile.length, 5);
  assert(borrowed.longKeyBytes > 4096, "one map key crosses the one-page grant boundary");
  assert.equal(
    Math.max(...Object.keys(borrowed.edit.forwards[0].ReplaceMap.map).map((key) => Buffer.byteLength(key))),
    borrowed.longKeyBytes,
    "longKeyBytes is the widest declared key",
  );
  assert.deepEqual(borrowed.lifetime, { cancelBeforeRootDrop: true, liveRootDrops: 1, iteratorDropsBeforeRoot: true });

  assert.equal(progress.version, 1);
  assert.deepEqual(progress.modes, ["indexed", "borrowed"]);
  assert.deepEqual(progress.grants, [0, 1, 7, 256, 4096]);
  assert.equal(progress.expectedPrefix, `[${JSON.stringify(progress.text)},`);
  assert.equal(progress.expectedBytes, Buffer.byteLength(progress.expectedPrefix), "the partial frame is byte-exact");
  assert.equal(progress.expectedSnapshotBytes, Buffer.byteLength(progress.text), "the retired snapshot carries the raw scalar");
  assert.equal(progress.expectedComplete, false);
  assert.equal(progress.expectedRootRetirements, 1);
  assert.equal(progress.sentinel, 165);

  const hostiles: [string, unknown][] = [
    ["CanonicalReader", { ...reader, unexpected: true }],
    ["CanonicalReader", { ...reader, lifecycle: [...reader.lifecycle, "unknown-stage"] }],
    ["CanonicalEditSealer", { ...sealer, hostile: [...sealer.hostile, "unknown-hostile"] }],
    ["CanonicalEditSealer", { ...sealer, expectedDigest: "not-a-digest" }],
    ["CanonicalBorrowedMap", { ...borrowed, lifetime: { ...borrowed.lifetime, unexpected: true } }],
    ["CanonicalBorrowedMap", { ...borrowed, edit: { ...borrowed.edit, forwards: [{ Replace: { text: "", nested: [], enabled: true, amount: 0 } }] } }],
    ["CanonicalErrorProgress", { ...progress, modes: ["indexed", "indexed"] }],
    ["CanonicalErrorProgress", { ...progress, sentinel: 256 }],
  ];
  for (const [name, hostile] of hostiles) assert.equal(exported(name)(hostile), false, `${name} hostile is refused`);

  console.log(`[DEBUG] canonical edit oracle: AJV=4 canonical-JSON=2 SHA-256=1 UTF-8-frames=2 hostiles=${hostiles.length}; native reader/sealer lifetimes remain a separate gate`);
}
//#endregion 🧵️CanonicalEditOracle
