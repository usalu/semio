import {binary64} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
/** 🛂️ Owned sparse-diff vectors retain native defaults, clearing and exact u64 ordinals. */
import { expect, test } from "bun:test";
import fixture from "../../🧫️fixtures/🛂️parsers/🔣️.json";
import { parseObjDiff, parseObjVerticesDiff, parseObjTexCoordsDiff, parseObjNormalsDiff, parseObjFacesDiff, parseObjGroupsDiff, parseObjObjectsDiff, parseObjVertexAdded, parseObjVertexModified, parseObjTexCoordAdded, parseObjNormalAdded, parseObjFaceAdded, parseObjGroupAdded, parseObjObjectAdded } from "../../🟦️.ts";
import {parseObjFaceVertex,parseObjSmoothingRange}from"../../🟦️.ts";

test("OBJ diff collection parsers match native omitted-vector defaults", () => {
  for (const parse of [parseObjVerticesDiff, parseObjTexCoordsDiff, parseObjNormalsDiff, parseObjFacesDiff, parseObjGroupsDiff, parseObjObjectsDiff]) {
    expect(parse(fixture.omittedCollections)).toEqual(fixture.expectedCollections);
  }
  expect(parseObjVertexModified(fixture.clearVertexWeight)).toMatchObject(fixture.clearVertexWeight);
  expect(parseObjDiff({ mtllib: null, unknownStatements: [{ lineIndex: 18446744073709551615n, raw: "source" }] })).toMatchObject({ mtllib: null, unknownStatements: [{ lineIndex: 18446744073709551615n, raw: "source" }] });
});

test("OBJ added-item parsers validate actual nested scalar and relationship domains", () => {
  for (const vertex of fixture.invalidVertices) expect(() => parseObjVertexAdded({ index: 0, vertex })).toThrow();
  for (const [parse, value] of [
    [parseObjTexCoordAdded, { index: 0, texcoord: { u: 0, v: "wrong" } }],
    [parseObjNormalAdded, { index: 0, normal: { x: 0, y: 0, z: NaN } }],
    [parseObjFaceAdded, { index: 0, face: { vertices: [{ vertex: "wrong" }] } }],
    [parseObjGroupAdded, { index: 0, group: { name: "group", faces: ["wrong"] } }],
    [parseObjObjectAdded, { index: 0, object: { name: "object", faces: ["wrong"] } }],
  ] as const) expect(() => parse(value)).toThrow();
  expect(parseObjVertexAdded({ index: 0, vertex: {x:binary64(1),y:binary64(2),z:binary64(3),w:binary64(4)} })).toEqual({ index: 0, vertex: {x:binary64(1),y:binary64(2),z:binary64(3),w:binary64(4)} });
});

test("OBJ diff face references retain unsigned32 without accepting wider or negative words",()=>{
 for(const vertex of[-1,4294967296]){
  expect(()=>parseObjFaceVertex({vertex})).toThrow();
  expect(()=>parseObjSmoothingRange({faceIndexFrom:0n,group:vertex})).toThrow();
  expect(()=>parseObjFaceAdded({index:0,face:{vertices:[{vertex}]}})).toThrow();
  expect(()=>parseObjFaceAdded({index:0,face:{vertices:[{vertex:0,texcoord:vertex}]}})).toThrow();
  expect(()=>parseObjFaceAdded({index:0,face:{vertices:[{vertex:0,normal:vertex}]}})).toThrow();
 }
 expect(parseObjFaceAdded({index:0,face:{vertices:[{vertex:4294967295,texcoord:4294967295,normal:4294967295}]}}).face.vertices[0]).toEqual({vertex:4294967295,texcoord:4294967295,normal:4294967295});
});
