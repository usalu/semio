
const testSourceUrl = new URL("../../🧵️canonical-edit/📜️script.ts", import.meta.url);
/** 🧵️ Canonical edit contracts: schema-owned exports plus independent JSON/SHA-256/UTF-8 oracles. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

//#region 🧵️CanonicalEditOracle
const read = (path: string) => JSON.parse(readFileSync(new URL(path, testSourceUrl.href), "utf8"));
const sha256 = (value: string): string => createHash("sha256").update(value).digest("hex");
/** 🔢️ An edit's revision value: its members in declaration order without `sequenceNumber` (design §22.28) — where an edit sits in its history is not part of what it is. */
const revisionValue = ({ sequenceNumber: _sequenceNumber, ...revision }: Record<string, unknown>): Record<string, unknown> => revision;

/** 🔏️ Proves canonical edit payloads and example bytes with independent encoders. */
export function testCanonicalEditFixtures(): void {
  const contract = read("./🧬️schema/🔣️.json");
  const ajv = new Ajv({ strict: true, allErrors: true });
  const models = read("./🧪️testing/🧬️schema/🔣️.json");
  ajv.addSchema(contract).addSchema(models);
  const exported = (name: string) => ajv.getSchema(`${models.$id}#/$defs/${name}`)!;

  const reader = read("./🧫️fixtures/📖️canonical-reader.json");
  const sealer = read("./🧫️fixtures/🔏️canonical-edit-sealer.json");
  const borrowed = read("./🧫️fixtures/🗺️canonical-borrowed-map.json");
  const progress = read("./🧫️fixtures/🚧️canonical-error-progress.json");
  assert(exported("CanonicalEditSealerEdit")(sealer.edit), "sealer edit payload");
  assert(exported("CanonicalBorrowedMapEdit")(borrowed.edit), "borrowed map edit payload");

  for (const [name, fixture] of [["canonical-edit-sealer", sealer], ["canonical-borrowed-map", borrowed]] as const) {
    assert.equal(JSON.stringify(revisionValue(fixture.edit)), fixture.expectedJson, `${name} canonical JSON emits declaration order without the sequence number`);
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

  assert.equal(exported("CanonicalBorrowedMapEdit")({ ...borrowed.edit, forwards: [{ Replace: { text: "", nested: [], enabled: true, amount: 0 } }] }), false, "borrowed mutation vocabulary refuses a scalar replacement");

  const chains = read("./🧫️fixtures/🔗️edit-digest-chains.json");
  const u64 = (value: number) => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64BE(BigInt(value)); return bytes; };
  const record = (domain: string, parts: Buffer[]) => {
    const hash = createHash("sha256").update("semio.artifact.cursor.v2").update(u64(Buffer.byteLength(domain))).update(domain);
    for (const part of parts) hash.update(u64(part.length)).update(part);
    return hash.digest();
  };
  const chain = (domain: string, items: unknown[]) => items.reduce<Buffer>((state, item) => record(domain, [state, Buffer.from(JSON.stringify(item))]), Buffer.alloc(32));
  const text = (edit: Record<string, unknown>, key: string) => [Buffer.from([key in edit ? 1 : 0]), Buffer.from(String(edit[key] ?? ""))];
  const editDigest = (edit: Record<string, unknown>) => {
    const [forwards, inverse, meta] = [edit.forwards as unknown[], edit.inverse as unknown[], (edit.mutationMeta ?? []) as unknown[]];
    if (forwards.length <= 1) return record("edit", [Buffer.from(String(edit.id)), Buffer.from(JSON.stringify(revisionValue(edit)))]);
    const chained = record("edit-chained", [
      Buffer.from(String(edit.id)),
      ...text(edit, "actor"),
      Buffer.from(String(edit.startedAt)),
      ...text(edit, "finishedAt"),
      u64(forwards.length), chain("edit-forward", forwards),
      u64(inverse.length), chain("edit-inverse", inverse),
      u64(meta.length), chain("edit-meta", meta),
    ]);
    const authored = "verb" in edit ? record("edit-verb", [chained, Buffer.from(String(edit.verb))]) : chained;
    return record("edit-line", [authored, Buffer.from([edit.line === null ? 0 : 1]), Buffer.from(String(edit.line ?? ""))]);
  };
  for (const row of chains.cases) {
    const edit = row.edit ?? {
      ...row.header,
      forwards: Array.from({ length: row.generatedOperations }, (_, index) => ({ SetN: { n: index + 1 } })),
      inverse: Array.from({ length: row.generatedOperations }, (_, index) => ({ SetN: { n: index } })),
    };
    assert.equal(editDigest(edit).toString("hex"), row.expectedDigest, `edit digest vector ${row.name}`);
  }
}
//#endregion 🧵️CanonicalEditOracle
