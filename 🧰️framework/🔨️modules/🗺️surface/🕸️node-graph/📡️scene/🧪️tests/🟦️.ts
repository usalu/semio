import {expect, test} from "bun:test";
import Ajv from "ajv/dist/2020.js";
import {getNodeValue, parseTree, type ParseError} from "jsonc-parser";
import contract from "../🧬️schema/🔣️.json";
import corpus from "../🧫️fixtures/🔣️.json";
import wire from "../🧫️fixtures/📦️body/🔣️.json";
import viewport from "../../../../🖱️ui/🪟️viewport/◻️2d/🧬️schema/🔣️.json";
import {readIntrinsicBody} from "../../../../🎒️pack/🌱️value/🧪️tests/🫳️preflight/🔮️pack/🟦️.ts";

const ajv = new Ajv({strict: true, allErrors: true}).addSchema(viewport).addSchema(contract);
function physical(value:unknown):unknown{
  if(typeof value==="number"){const bytes=new Uint8Array(8);new DataView(bytes.buffer).setFloat64(0,value,false);return{bits:Buffer.from(bytes).toString("hex")};}
  if(Array.isArray(value))return value.map(physical);
  if(value!==null&&typeof value==="object")return Object.fromEntries(Object.entries(value).map(([key,value])=>[key,physical(value)]));
  return value;
}

test("scene ingress declares variable payloads and explicit physical framing", () => {
  const scene = ajv.getSchema(contract.$id)!;
  const request = ajv.getSchema(`${contract.$id}#/$defs/DecodeRequest`)!;
  const progress = ajv.getSchema(`${contract.$id}#/$defs/DecodeProgress`)!;
  for(const phase of [...corpus.cancelStages,"complete"])expect(progress({phase,units:0,admittedBytes:0,complete:false})).toBe(true);
  for(const phase of ["unknown",undefined])expect(progress({phase,units:0,admittedBytes:0,complete:false})).toBe(false);
  expect(scene(corpus.scene)).toBe(true);
  for (const row of corpus.embedded) expect(scene({...corpus.scene, [row.field]: JSON.stringify(row.value)})).toBe(true);
  for (const format of ["Body", "Document"]) {
    expect(request({format, inputBytes: 0, maximumOwnedBytes: 0, maximumDepth: 1, maximumItems: 1})).toBe(true);
    expect(request({format, inputBytes: 123, maximumOwnedBytes: 4096, maximumDepth: 17, maximumItems: 64})).toBe(true);
  }
  for (const format of [undefined, "Auto", "", "body"]) expect(request({format, inputBytes: 0, maximumOwnedBytes: 0, maximumDepth: 1, maximumItems: 1})).toBe(false);
  for (const row of corpus.structural.filter(row => row.refusal)) expect(scene(row.scene)).toBe(false);
  expect(scene({...corpus.scene, nodes: [{id: 7}]})).toBe(false);
  expect(scene({...corpus.scene, edges: [{id: "edge"}]})).toBe(false);
});

test("all embedded scene fields retain independent JSON syntax and Unicode values", () => {
  expect(new Set(corpus.embedded.map(row => row.field)).size).toBe(8);
  for (const row of corpus.embedded) {
    const source = JSON.stringify(row.value), errors: ParseError[] = [];
    const tree = parseTree(source, errors, {disallowComments: true, allowTrailingComma: false});
    expect(errors, row.field).toEqual([]);
    expect(getNodeValue(tree!), row.field).toEqual(row.value);
    expect(JSON.parse(source), row.field).toEqual(row.value);
    expect(JSON.parse(row.nativeJson), row.field).toEqual(row.value);
  }
});

test("fixed scene Body vectors agree with the independent General intrinsic wire reader", () => {
  expect(readIntrinsicBody(Uint8Array.from(Buffer.from(wire.bodyHex,"hex")))).toEqual(physical(corpus.scene));
  for(const row of corpus.embedded){const field=wire.embedded[row.field as keyof typeof wire.embedded];expect(field.startsWith("pk:")).toBe(true);expect(readIntrinsicBody(Uint8Array.from(Buffer.from(field.slice(3),"base64")))).toEqual(physical(row.value));}
  expect(readIntrinsicBody(Uint8Array.from(Buffer.from(wire.combinedHex,"hex")))).toEqual(physical({...corpus.scene,...wire.embedded}));
});
