import { expect, test } from "bun:test";
import fixture from "../../../🧪️tests/🧫️fixtures/🔬️contract/🖼️decoded-image/🔣️.json";
import { parseRasterAssetsDelta } from "../../🔺️diff/🟦️.ts";
test("encoded carriers cannot enter the semantic image delta", () => {
  expect(() => parseRasterAssetsDelta({ entries: { image: { mime: "image/png", data: "" } } })).toThrow();
});

import { Buffer } from "node:buffer";
import { rasterTransformFromJson, rasterLayerNodeFromJson } from "./../../../🚪️io/📝️text/📸️snapshot/🟦️.ts";
import { parseRasterTransform } from "../../🟦️.ts";
/** 🔢️ Node Buffer is an independent native IEEE word oracle for physical numeric binding. */
test("physical numeric binding preserves the Node Buffer IEEE words", () => {
  for (const number of fixture.numericWords.binary64) {
    const expected=Buffer.alloc(8);expected.writeDoubleLE(number);
    const transform=rasterTransformFromJson({x:number,y:0,a:1,b:0,c:0,d:1});
    expect(transform.x.bits).toBe(expected.readBigUInt64LE());
    expect(parseRasterTransform(transform).x).toEqual(transform.x);
    expect(() => parseRasterTransform({x:number,y:0,a:1,b:0,c:0,d:1})).toThrow();
  }
  for (const number of fixture.numericWords.binary32) {
    const expected=Buffer.alloc(4);expected.writeFloatLE(number);
    const layer=rasterLayerNodeFromJson({kind:"pixel",id:"word",name:"Word",visible:true,locked:false,opacity:number,blendMode:"normal",transform:{x:0,y:0,a:1,b:0,c:0,d:1}});
    expect(layer.opacity.bits).toBe(expected.readUInt32LE());
  }
});
