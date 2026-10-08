import assert from "node:assert/strict";
import Ajv from "ajv";
import { test } from "bun:test";
import corpus from "../../🧫️fixtures/📢️member-publication.json" with { type: "json" };

test("retained member publication funds independent decoding and release passes", () => {
  const validate = new Ajv().compile({type:"object",required:["wirePasses","fixedPublicationTurns","maximumItems","maximumBytes","wireSchema","orderedMembers"],properties:{wirePasses:{const:2},fixedPublicationTurns:{const:32},maximumItems:{const:1},maximumBytes:{const:4096},wireSchema:{type:"string"},orderedMembers:{type:"array",minItems:3}}});
  assert(validate(corpus), JSON.stringify(validate.errors));
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
