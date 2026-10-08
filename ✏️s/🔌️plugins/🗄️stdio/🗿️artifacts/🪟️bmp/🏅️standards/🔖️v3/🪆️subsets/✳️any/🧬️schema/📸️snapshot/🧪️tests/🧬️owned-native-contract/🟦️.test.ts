import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import schema from "../../🔣️.json";
import fixture from "../../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json";

const validate = new Ajv({ strict: false, allErrors: true }).compile(schema);

describe("Bmp owned native sample contract", () => {
  test("accepts every neutral precision and metadata fixture", () => {
    for (const row of fixture.cases) {
      expect(validate(row.snapshot), `${row.name}: ${JSON.stringify(validate.errors)}`).toBe(true);
    }
  });
  test("refuses encoded source authority and out-of-range samples", () => {
    expect(validate({ schema: fixture.cases[0]!.snapshot.schema, bytes: [0, 1, 2] })).toBe(false);
    const invalid = structuredClone(fixture.cases[0]!.snapshot);
    invalid.image.pixels.samples![0]!.reserved = 4294967296;
    expect(validate(invalid)).toBe(false);
  });
});

import replacementSchema from "../../../🧬️mutations/🔄️replace-image/🧬️schema/🔣️.json";
test("native BMP image replacement and inverse retain the exact ten-bit 513 component",async()=>{
 const validator=new Ajv({strict:false,allErrors:true}).addSchema(schema).compile(replacementSchema);
 const base=parseBmpSnapshot(fixture.cases[0]!.snapshot);
 for(const row of [...fixture.cases,...fixture.replacementCases]){
  const target=parseBmpSnapshot(row.snapshot),payload={image:target.image};
  expect(validator(payload),row.name).toBe(true);expect(validator({...payload,bytes:[0]}),row.name).toBe(false);
  const actual=await applyBmpMutation(base,{mutation:"replace-image",payload});expect(actual).toEqual(target);
  const diff=betweenBmpSnapshots(base,actual);expect(applyBmpDiff(actual,inverseBmpDiff(base,diff))).toEqual(base);
  const bytes=await encodeBmpSnapshot(actual);expect(await decodeBmpSnapshot(bytes)).toEqual(target);
  if(row.name==="bitfields32-ten-bit-513-exact-image-replacement"){
   const reference=Buffer.from(bytes),offset=reference.readUInt32LE(10),word=reference.readUInt32LE(offset);
   expect((word&1072693248)>>>20).toBe(513);expect((word&1047552)>>>10).toBe(777);expect(word>>>31).toBe(1);
  }
  console.log(`[DEBUG] BMP typed image replacement neutral=${row.name} nativeWordsExact=true inverseExact=true`);
 }
});

import {Database} from "bun:sqlite";
import {parseBmpSnapshot} from "../../🟦️.ts";
import {bmpRevision,paintBmpIndexedRegion,paintBmpDirectRegion,applyBmpMutation} from "../../../⚙️operations/🟦️.ts";
import {betweenBmpSnapshots,applyBmpDiff,inverseBmpDiff} from "../../../🔺️diff/🟦️.ts";
import {decodeBmpSnapshot,encodeBmpSnapshot} from "../../../../🚪️io/💾️binary/📸️snapshot/🟦️.ts";
import {bmpSnapshotToSqliteDatabase,bmpSnapshotFromSqliteDatabase} from "../../../../🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
import {exportSqliteDatabase} from "@semio-tech/framework";

test("owned BMP precision, native codec, addressed paint and inverse agree with neutral samples and independent SQLite",async()=>{
 for(const row of fixture.cases){const snapshot=parseBmpSnapshot(row.snapshot),p=row.paint;expect(snapshot).toEqual(parseBmpSnapshot(row.snapshot));const bytes=await encodeBmpSnapshot(snapshot);expect(await decodeBmpSnapshot(bytes)).toEqual(snapshot);const reference=Buffer.from(bytes);expect(reference.readInt32LE(18)).toBe(snapshot.image.width);expect(Math.abs(reference.readInt32LE(22))).toBe(snapshot.image.height);
 const revision=bmpRevision(snapshot),next=typeof p.paletteIndex==="number"?await paintBmpIndexedRegion(snapshot,revision,p,p.paletteIndex):await paintBmpDirectRegion(snapshot,revision,p,p);expect(next.image.pixels).toEqual(row.expectedIndices!==undefined?{storage:"indexed",indices:row.expectedIndices}:{storage:"direct",samples:row.expectedSamples});expect(validate(next)).toBe(true);const diff=betweenBmpSnapshots(snapshot,next);expect(applyBmpDiff(snapshot,diff)).toEqual(next);expect(applyBmpDiff(next,inverseBmpDiff(snapshot,diff))).toEqual(snapshot);const relational=await bmpSnapshotToSqliteDatabase(next);expect(await bmpSnapshotFromSqliteDatabase(relational)).toEqual(next);const independent=Database.deserialize(await exportSqliteDatabase(relational));try{expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);if(next.image.pixels.storage==="direct")expect(independent.query("SELECT red,green,blue,alpha,reserved FROM bmp_pixel_sample ORDER BY ordinal").all()).toEqual(next.image.pixels.samples);else expect(independent.query("SELECT palette_index FROM bmp_pixel_index ORDER BY ordinal").all().map((r:any)=>r.palette_index)).toEqual(next.image.pixels.indices);}finally{independent.close();}
 const paint=(rev:string,control={})=>typeof p.paletteIndex==="number"?paintBmpIndexedRegion(snapshot,rev,p,p.paletteIndex,control):paintBmpDirectRegion(snapshot,rev,p,p,control);await expect(paint("stale")).rejects.toThrow("stale");await expect(paint(revision,{progress:()=>false})).rejects.toThrow("cancelled");await expect(paint(revision,{maximumOwnedBytes:0})).rejects.toThrow("limit");expect(snapshot).toEqual(parseBmpSnapshot(row.snapshot));
 }
});

import {readFileSync as readPublicationFixture} from "node:fs";
import {applyPatch as patchPublicationGrant} from "fast-json-patch";
test("original image publication close grants preserve physical backing independently",()=>{
 const fixture=JSON.parse(readPublicationFixture(new URL("../../../../✏️editor/🧬️publication/🧫️fixtures/🔣️.json",import.meta.url),"utf8")),law=fixture.retirement;
 const backing=Buffer.alloc(law.physicalBytes),grant={items:1,copy:law.copyBytes,capacity:law.capacityBytes,release:backing.byteLength,depth:law.depth};
 for(const field of ["items","release","depth"]){const denied=patchPublicationGrant(structuredClone(grant),[{op:"replace",path:`/${field}`,value:0}],true).newDocument;expect(denied.items===0||denied.release<backing.byteLength||denied.depth===0).toBe(true);expect(backing.byteLength).toBe(law.physicalBytes);}
 expect(grant.copy).toBe(3);expect(grant.release).toBe(65536);expect(law.scratchEmptyRetainsCapacity).toBe(true);expect(law.terminalChildReceipt).toBe("Progress");expect(law.terminalOwnerReceipt).toBe("Complete");
});
