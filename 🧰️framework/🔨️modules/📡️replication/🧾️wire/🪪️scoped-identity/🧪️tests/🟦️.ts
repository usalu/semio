import assert from "node:assert/strict";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import visitorFixture from "../🧫️fixtures/👁️visitor/🔣️.json" with {type:"json"};
import visitorSchema from "../🧬️schema/👁️visitor/🔣️.json" with {type:"json"};
import { visitScopedIdentitySegments, encodeScopedIdentity, decodeScopedIdentity, SCOPED_IDENTITY_MAX_BYTES, SCOPED_IDENTITY_MAX_SEGMENTS } from "../🟦️.ts";

/** 🔮️ Ajv and Node Buffer independently adjudicate canonical Unicode identities against shared vectors. */
export function testScopedIdentities(): void {
  const validate = new Ajv({strict:true}).compile(schema);
  for (const row of fixture.valid) {
    assert(validate(row.segments), JSON.stringify(validate.errors));
    const oracle = "scope:1:" + row.segments.map(segment => `${Buffer.byteLength(segment, "utf8")}:${segment}`).join("");
    assert.equal(oracle, row.identity);
    assert.equal(encodeScopedIdentity(row.segments), oracle);
    assert.deepEqual(decodeScopedIdentity(row.identity), row.segments);
  }
  for (const segments of fixture.invalidSegments) { assert(!validate(segments)); assert.throws(() => encodeScopedIdentity(segments)); }
  for (const identity of fixture.invalidIdentities) assert.throws(() => decodeScopedIdentity(identity), Error, JSON.stringify(identity));
  assert.throws(() => encodeScopedIdentity(Array(SCOPED_IDENTITY_MAX_SEGMENTS + 1).fill("x")));
  assert.throws(() => decodeScopedIdentity("scope:1:" + "1:x".repeat(SCOPED_IDENTITY_MAX_SEGMENTS + 1)));
  assert.throws(() => encodeScopedIdentity(["x".repeat(SCOPED_IDENTITY_MAX_BYTES)]));
  assert.throws(() => encodeScopedIdentity(["🧱️".repeat(SCOPED_IDENTITY_MAX_BYTES)]));
  assert.throws(() => decodeScopedIdentity("scope:1:" + "x".repeat(SCOPED_IDENTITY_MAX_BYTES * 2)));
  assert.throws(() => encodeScopedIdentity(["\ud800"]));
  assert.throws(() => decodeScopedIdentity("scope:1:3:\ud800"));
  assert(new Ajv({strict:true}).compile(visitorSchema)(visitorFixture));
  for (const [index, row] of fixture.valid.entries()) {
    const visited: string[] = [];
    assert.equal(visitScopedIdentitySegments(row.identity, segment => visited.push(segment)), visitorFixture.callbackCounts[index]);
    assert.deepEqual(visited, row.segments);
    assert.deepEqual(visited.map(segment => Buffer.from(segment, "utf8")), row.segments.map(segment => Buffer.from(segment, "utf8")));
  }
  for (const identity of fixture.invalidIdentities) {
    let callbacks = 0;
    assert.throws(() => visitScopedIdentitySegments(identity, () => callbacks++));
    assert.equal(callbacks, visitorFixture.invalidCallbacks);
  }
  let callbacks = 0;
  assert.equal(visitScopedIdentitySegments("scope:1:" + "1:x".repeat(SCOPED_IDENTITY_MAX_SEGMENTS), () => callbacks++), visitorFixture.maximumCallbacks);
  assert.equal(callbacks, visitorFixture.maximumCallbacks);
  callbacks = 0;
  assert.throws(() => visitScopedIdentitySegments("scope:1:" + "1:x".repeat(SCOPED_IDENTITY_MAX_SEGMENTS + 1), () => callbacks++));
  assert.equal(callbacks, visitorFixture.overflowCallbacks);
  console.log("[DEBUG] Scoped identity visitor validates whole input before callbacks; Buffer agrees with every ordered UTF8 segment and32segment boundary");
  console.log("[DEBUG] Scoped identity portable vectors agree with Ajv admission and independent Buffer UTF8 byte-length oracle");
}
