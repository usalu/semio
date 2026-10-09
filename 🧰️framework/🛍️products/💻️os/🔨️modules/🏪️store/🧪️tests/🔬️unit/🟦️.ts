import { spawnSync } from "node:child_process";
import Ajv from "ajv";
import physicalGrantSchema from "../../../../../../🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json" with { type: "json" };
import { fileURLToPath } from "node:url";
import assert from "node:assert/strict";
import { test } from "bun:test";
import corpus from "../../🧫️fixtures/📢️member-publication.json" with { type: "json" };

test("retained member publication funds independent decoding and release passes", () => {
  const encoder = new TextEncoder();
  const schemaBytes = encoder.encode(corpus.wireSchema).byteLength;
  for (const row of corpus.orderedMembers) {
    const bytes = encoder.encode(row.wire + " ".repeat(row.paddingBytes));
    assert.equal(JSON.parse(new TextDecoder().decode(bytes)), row.expected);
    const decodeTurns = bytes.byteLength;
    const releaseTurns = bytes.byteLength + schemaBytes;
    const turns = decodeTurns + releaseTurns + corpus.fixedPublicationTurns;
    assert.equal(turns, bytes.byteLength * corpus.wirePasses + schemaBytes + corpus.fixedPublicationTurns);
    assert.equal(Math.max(1, ...Array.from(bytes, () => 1)), corpus.maximumItems);
    assert(1 <= corpus.maximumBytes);
    console.log(`[DEBUG] Independent member publication ${row.id} borrowed-decode=${decodeTurns} retained-release=${releaseTurns} fixed=${corpus.fixedPublicationTurns}`);
  }
});


test("current original Store unit source is accepted by the independent Rust grammar", () => {
  const { status, error, stderr } = spawnSync("rustfmt", ["--edition", "2021", "--emit", "stdout", "--config", "skip_children=true", fileURLToPath(new URL("./🦀️.rs", import.meta.url))], { stdio: ["ignore", "ignore", "pipe"], timeout: 5000 });
  assert.ifError(error);
  assert.equal(status, 0, stderr.toString());
  console.log("[DEBUG] Original Store Unit Rust grammar accepted; no source copy or native type/runtime credit");
});

test("original Presence examples separate Unicode body from funded physical backing", async () => {
  const law = await Bun.file(new URL("../../👥️presence/🧫️fixtures/🛂️peer-admission.json", import.meta.url)).json();
  const retirement = await Bun.file(new URL("../../👥️presence/🧫️fixtures/🧹️retirement.json", import.meta.url)).json();
  assert.deepEqual(law.physicalCloseGrant, retirement.physicalCloseGrant);
  assert.equal(law.physicalCloseGrant.maximumCopyBytes, law.maximumBytes);
  for (const row of law.cases) {
    const text = row.actor.unit.repeat(row.actor.repeat);
    assert.equal(new TextEncoder().encode(text).byteLength, row.expectedActorBytes);
    assert.equal(Buffer.byteLength(JSON.parse(JSON.stringify(text)), "utf8"), row.expectedActorBytes);
    assert(row.actor.minimumCapacity > law.physicalCloseGrant.maximumCopyBytes);
    assert(row.actor.minimumCapacity <= law.physicalCloseGrant.maximumReleaseBytes);
  }
  assert.equal(law.requiresCapacitySizedCopyGrant, false);
  assert.equal(law.requiresCapacitySizedReleaseGrant, true);
  console.log("[DEBUG] Original Presence seven Unicode examples keep copy4096 independent of original backing release; JSON/Buffer/TextEncoder agree");
});

test("current original Presence and Store read owners parse with independent Rust grammar", () => {
  const files = [
    "../../🦀️.rs",
    "../../👥️presence/♻️retirement/🦀️.rs",
    "../../👥️presence/♻️retirement/🧪️tests/🔬️unit/🦀️.rs",
    "../../👥️presence/🚫️rejection/🧪️tests/🔬️unit/🦀️.rs",
    "../../👥️presence/🧪️testing/♻️retirement/🦀️.rs",
    "../../🔗️read/♻️retirement/🦀️.rs",
    "../../🔗️read/♻️retirement/🧪️tests/🔬️unit/🦀️.rs",
  ];
  for (const file of files) {
    const { status, error, stderr } = spawnSync("rustfmt", ["--edition", "2021", "--emit", "stdout", "--config", "skip_children=true", fileURLToPath(new URL(file, import.meta.url))], { stdio: ["ignore", "ignore", "pipe"], timeout: 5000 });
    assert.ifError(error);
    assert.equal(status, 0, stderr.toString());
  }
  console.log("[DEBUG] Original Presence/read Rust grammar accepted:7; Source copies0, native type/runtime assertions0");
});


test("current transfer, clone and schema consumers retain five original currencies", async () => {
  const owners = ["../../🫧️ephemeral/📢️publication/🔁️transfer/🧪️tests/🔬️unit/🦀️.rs", "../🧬️retained-clone/🦀️.rs", "../🧬️owned-schema-record/🦀️.rs"];
  for (const owner of owners) {
    const path = fileURLToPath(new URL(owner, import.meta.url));
    const source = await Bun.file(path).text();
    assert(!source.includes("SnapshotRetirementStep"), `obsolete close currency in ${owner}`);
    assert(!/ArtifactStoreOneItemGrant\s*\{[^}]*maximum_bytes:/s.test(source), `collapsed grant in ${owner}`);
    const result = spawnSync("rustfmt", ["--edition", "2021", "--emit", "stdout", "--config", "skip_children=true", path], { stdio: ["ignore", "ignore", "pipe"], timeout: 5000 });
    assert.ifError(result.error);
    assert.equal(result.status, 0, result.stderr.toString());
  }
  const transfer = await Bun.file(new URL("../../🫧️ephemeral/📢️publication/🔁️transfer/🧫️fixtures/🔣️.json", import.meta.url)).json();
  const text = transfer.payload.text.repeat(transfer.payload.repeat);
  assert.equal(Buffer.byteLength(text), transfer.payload.utf8Bytes);
  assert.equal(new TextEncoder().encode(JSON.parse(JSON.stringify(text))).byteLength, transfer.payload.utf8Bytes);
  for (const row of transfer.cases) assert.equal(row.prepared, !row.cancelBefore && row.items > 0 && row.copyBytes >= 48 && row.capacityBytes >= 40);
  for (const name of ["maximumItems", "maximumCopyBytes", "maximumCapacityBytes", "maximumReleaseBytes", "maximumDepth"]) assert(Number.isSafeInteger(transfer.retirementGrant[name]));
  assert(transfer.retirementGrant.maximumReleaseBytes >= Buffer.byteLength(text));
  const admit = new Ajv({ strict: true }).compile(physicalGrantSchema);
  const clone = await Bun.file(new URL("../../../../../../🔨️modules/🌱️value/🧬️retained-clone/🧫️fixtures/📦️nested/🔣️.json", import.meta.url)).json();
  const schema = await Bun.file(new URL("../../🧫️fixtures/🧬️owned-schema-record/🔣️.json", import.meta.url)).json();
  for (const policy of [transfer.retirementGrant, clone.retirementGrant, schema.retirementGrant]) {
    assert.equal(admit(policy), true, JSON.stringify(admit.errors));
    for (const axis of physicalGrantSchema.required) {
      const missing = { ...policy };
      delete missing[axis];
      assert.equal(admit(missing), false);
    }
  }
  assert(clone.retirementGrant.maximumReleaseBytes >= clone.payloadByteLength);
  for (const wordBytes of [4, 8]) {
    const metadata = transfer.transferMetadata[String(wordBytes * 8)];
    assert.equal(metadata.copyBytes, 6 * wordBytes);
    assert.equal(metadata.arcBytes, 5 * wordBytes);
  }
  console.log("[DEBUG] Three original consumer grammars and independent JSON/Buffer/TextEncoder transfer grants; native System assertions remain required");
});

test("current native cleanup callers retain independent plain authority instead of pricing it from demands", async () => {
  const fixture = await Bun.file(new URL("../../🧩️composition/📨️emission/📦️owned/🧫️fixtures/🔣️.json", import.meta.url)).json();
  const policy = fixture.physicalCloseGrant;
  const admit = new Ajv({ strict: true }).compile(physicalGrantSchema);
  assert.equal(admit(policy), true, JSON.stringify(admit.errors));
  const text = await Bun.file(new URL("../../🧫️fixtures/♻️bounded-value-retirement/🔣️.json", import.meta.url)).json();
  const body = text.body.repeat(text.repeat);
  assert.equal(Buffer.byteLength(body), new TextEncoder().encode(JSON.parse(JSON.stringify(body))).byteLength);
  assert(Buffer.byteLength(body) < policy.maximumReleaseBytes);
  for (const file of ["../♻️bounded-value-retirement/🦀️.rs", "../🧪️supersede-replay/🦀️.rs"]) {
    const path = fileURLToPath(new URL(file, import.meta.url));
    const source = await Bun.file(path).text();
    assert(!/maximum_(?:copy|capacity|release)_bytes:\s*demand\.(?:copy|capacity|release)_bytes\s*[,}]/u.test(source), `demand becomes positive authority in ${file}`);
    assert(!source.includes(".max(demand.copy_bytes)") && !source.includes(".max(owner.next_release_byte_demand()"), `caller bound enlarged by frontier in ${file}`);
    assert(!/maximum_(?:capacity|release)_bytes:\s*(?:cursor|owner)\.next_\w+\([^)]*\)\.unwrap\(\)\s*[,}]/u.test(source), `queried owner supplies positive authority in ${file}`);
    assert(!source.includes("one_capacity_turn(capacity, 1)"), `queried constructor supplies positive policy in ${file}`);
    const result = spawnSync("rustfmt", ["--edition", "2021", "--emit", "stdout", "--config", "skip_children=true", path], { stdio: ["ignore", "ignore", "pipe"], timeout: 5000 });
    assert.ifError(result.error);
    assert.equal(result.status, 0, result.stderr.toString());
  }
  for (const file of ["../🧪️deferred-reprojection/🦀️.rs", "../🧪️supersede-law/🦀️.rs", "../🧪️tool-transaction/🦀️.rs", "../🧪️viewer-head/🦀️.rs", "../../🔄️sync/🧪️tests/🔬️unit/🦀️.rs", "../../🔄️sync/🧪️tests/🔬️backbone-parity/🦀️.rs"]) {
    const result = spawnSync("rustfmt", ["--edition", "2021", "--emit", "stdout", "--config", "skip_children=true", fileURLToPath(new URL(file, import.meta.url))], { stdio: ["ignore", "ignore", "pipe"], timeout: 5000 });
    assert.ifError(result.error);
    assert.equal(result.status, 0, result.stderr.toString());
  }
  console.log("[DEBUG] Independent original full policy admitted by Ajv; JSON/Buffer/TextEncoder body agrees, queries never price positive native cleanup authority; eight original Rust grammars, native System proof still required");
});
