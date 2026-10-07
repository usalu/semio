/** 💾️ Editable metadata fixtures validate independently and preserve the complete layer tree. */
import {expect,test} from "bun:test";

import Ajv from "ajv";
import sharp from "sharp";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../../../🧬️schema/🔣️.json";
import valueSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🔣️.json";

import {rasterLayerNodeFromJson as parseRasterLayerNode} from "./../../../🚪️io/📝️text/📸️snapshot/🟦️.ts";
import {printRasterLayerNode} from "./../../../🚪️io/📝️text/📸️snapshot/🟦️.ts";
test("Editable archive fixture preserves nested masks, protection and adjustments",()=>{
  const validate=new Ajv({strict:false}).addSchema(valueSchema).addSchema({$id:schema.$id,$defs:schema.$defs}).compile({$ref:schema.$id+"#/$defs/RasterLayerNode"});
  for(const layer of fixture.layers){expect(validate(layer)).toBe(true);expect(JSON.parse(JSON.stringify(printRasterLayerNode(parseRasterLayerNode(layer))))).toEqual(layer);}
});
test("Editable archive source asset is independently decodable",async()=>{
  const image=await sharp(Buffer.from(fixture.pngBase64,"base64")).raw().toBuffer({resolveWithObject:true});
  expect([image.info.width,image.info.height]).toEqual(fixture.assetSize);expect(image.data.length).toBe(16);
});
