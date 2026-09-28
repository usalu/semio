/** 📤️ Bounded base64 output agrees with the platform encoder and PNG fixture agrees with libvips. */
import {expect,test} from "bun:test";
import sharp from "sharp";
import fixture from "../🧫️fixtures/🔣️.json";
import {base64OutputPage} from "../🟦️.ts";
for(const row of fixture.base64Cases)test("Export base64 "+row.bytes.length,()=>{expect(new TextDecoder().decode(base64OutputPage(Uint8Array.from(row.bytes)))).toBe(row.expected);});
for(const length of fixture.boundaryLengths)test("Export chunk boundary "+length,()=>{
  const input=Uint8Array.from({length},(_,i)=>i%256),chunks:Uint8Array[]=[];
  for(let offset=0;offset<input.length;offset+=fixture.base64InputBytesPerGrant){const chunk=base64OutputPage(input.subarray(offset,offset+fixture.base64InputBytesPerGrant));expect(chunk.length).toBeLessThanOrEqual(fixture.outputChunkBytes);chunks.push(chunk);}
  expect(Buffer.concat(chunks).toString()).toBe(Buffer.from(input).toString("base64"));
});
test("Export refuses a base64 page above its grant",()=>{expect(()=>base64OutputPage(new Uint8Array(fixture.base64InputBytesPerGrant+1))).toThrow();});
test("Export PNG oracle preserves all RGBA channels",async()=>{
  const {width,height,pixels}=fixture.image;
  const png=await sharp(Uint8Array.from(pixels),{raw:{width,height,channels:4}}).png().toBuffer();
  const result=await sharp(png).raw().toBuffer({resolveWithObject:true});
  expect(result.info.width).toBe(width);expect(result.info.height).toBe(height);expect([...result.data]).toEqual(pixels);
});

test("Export progress schema admits every localized stage",async()=>{
  const Ajv=(await import("ajv")).default,schema=(await import("../🧬️schema/🔣️.json")).default,validate=new Ajv().compile(schema);
  for(const [stage,labels] of Object.entries(fixture.labels))expect(validate({stage,...labels,completedUnits:1})).toBe(true);
  expect(validate({stage:"prepare",en:"Preparing",completedUnits:1})).toBe(false);
});

test("Export download command accepts no document edits",async()=>{
  const Ajv=(await import("ajv")).default,schema=(await import("../../🎮️commands/📤️export-png/🧬️schema/🔣️.json")).default,validate=new Ajv().compile(schema);
  expect(validate(fixture.download.args)).toBe(true);
  expect(validate({pixels:[]})).toBe(false);
  expect(validate({filename:"../image.png"})).toBe(false);
});
