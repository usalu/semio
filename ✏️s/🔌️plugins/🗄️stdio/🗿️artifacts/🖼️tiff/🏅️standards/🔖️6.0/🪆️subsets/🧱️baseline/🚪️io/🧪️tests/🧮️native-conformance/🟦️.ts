import Ajv from "ajv";
import schema from "../../../../🧾️document/🚪️io/💾️binary/📸️snapshot/👁️observations/🔣️.json";
import {NativeDecodeControl} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import fixtures from "./🔣️.json";
import { classifyTiffNativeBaseline } from "../../🧮️native-conformance/🟦️.ts";
import { inspectTiffNative } from "../../../../🧾️document/🚪️io/💾️binary/📸️snapshot/🟦️.ts";
test("neutral native conformance profiles have exact codes", () => {
 for (const row of fixtures.cases) expect(classifyTiffNativeBaseline({...fixtures.base,...row.patch})).toEqual(row.codes.map(code=>"stdio.tiff.baseline."+code));
});
test("native observations retain metadata of the authentic independent TIFF fixture", async () => {
 const input=readFileSync(join(import.meta.dir,"../../../../🧾️document/🧫️fixtures/🎨️paint-region-applied/⬅️before.tiff"));
 const observed=await inspectTiffNative(input);
 expect(observed.ifdCount).toBe(1);expect(observed.raster).toBe(true);
 expect(observed.compression).toEqual([1]);expect(observed.photometric).toEqual([2]);expect(observed.bitsPerSample).toEqual([8,8,8]);expect(observed.tileWidth).toBeNull();expect(observed.tileLength).toBeNull();expect(observed.stripOffsets).toEqual([24]);
 expect(classifyTiffNativeBaseline(observed)).toEqual([]);
});
test("independent schema and paid observation admission refuse invalid data", async()=>{
 const validate=new Ajv({strict:false}).compile({...schema,$schema:undefined});
 for(const row of fixtures.cases)expect(validate({...fixtures.base,...row.patch})).toBe(true);
 expect(validate({...fixtures.base,headerTail:[]})).toBe(false);expect(validate({...fixtures.base,compression:[4294967296]})).toBe(false);
 const input=readFileSync(join(import.meta.dir,"../../../../🧾️document/🧫️fixtures/🎨️paint-region-applied/⬅️before.tiff"));
 await expect(inspectTiffNative(input,new NativeDecodeControl(1,()=>true))).rejects.toThrow();
 let checkpoints=0;await expect(inspectTiffNative(input,new NativeDecodeControl(1e7,()=>++checkpoints<3))).rejects.toThrow();expect(checkpoints).toBe(3);
});
