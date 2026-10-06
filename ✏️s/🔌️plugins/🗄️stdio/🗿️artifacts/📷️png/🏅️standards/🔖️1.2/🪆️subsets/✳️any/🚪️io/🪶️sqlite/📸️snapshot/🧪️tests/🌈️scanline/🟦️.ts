/** 🌈️ Independent complete scanline, packing and compression ownership laws. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {deflateSync,inflateSync} from "node:zlib";
import {PNG} from "pngjs";
import fixture from "../../🧫️fixtures/🌈️scanline/🔣️.json";

import {pngSnapshotToSqliteDatabase,pngSnapshotFromSqliteDatabase} from "../../🟦️.ts";
import {parsePngNative,encodePngNative} from "../../🧩️chunks/🟦️.ts";
import {parseCompression,encodeCompression} from "../../🗜️compression/🟦️.ts";
import dynamicFixture from "../../🧫️fixtures/🗜️compression/🔣️.json";

import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework";
function crc(bytes:Uint8Array):number{let v=0xffffffff;for(const byte of bytes){v^=byte;for(let i=0;i<8;i++)v=(v>>>1)^((v&1)?0xedb88320:0)}return(~v)>>>0}
function chunk(kind:string,data:Uint8Array):Buffer{const b=Buffer.alloc(data.length+12);b.writeUInt32BE(data.length);b.write(kind,4,"ascii");b.set(data,8);b.writeUInt32BE(crc(b.subarray(4,8+data.length)),8+data.length);return b}
function native(item:typeof fixture.cases[number],level:number):Buffer{const h=Buffer.alloc(13);h.writeUInt32BE(item.width);h.writeUInt32BE(item.height,4);h[8]=item.bitDepth;h[9]=item.colorType;h[12]=item.interlace;const compressed=deflateSync(Buffer.from(item.raw),{level});return Buffer.concat([Buffer.from([137,80,78,71,13,10,26,10]),chunk("IHDR",h),chunk("IDAT",compressed.subarray(0,3)),chunk("IDAT",compressed.subarray(3)),chunk("abCd",Buffer.from([0,255,0])),chunk("abCd",Buffer.from([17])),chunk("IEND",Buffer.alloc(0))])}
test("PNG native relational ownership interprets all filters, Adam7, packed remainder, decompressed tails and complete DEFLATE recipes",async()=>{
 expect(dynamicFixture["role"]).toEqual("exactDynamicHuffmanRecipe");expect(dynamicFixture["repeat"]).toEqual(1000);expect(dynamicFixture["length"]).toEqual(7000);expect(dynamicFixture["level"]).toEqual(6);expect(dynamicFixture["blockKind"]).toEqual(2);expect(dynamicFixture["cmf"]).toEqual(120);expect(dynamicFixture["flg"]).toEqual(156);expect(dynamicFixture["adler"]).toEqual(2651542025);
 const dynamicRaw=Buffer.from(Array.from({length:dynamicFixture.length},(_,index)=>dynamicFixture.pattern[index%dynamicFixture.pattern.length]));expect(dynamicRaw.length).toBe(dynamicFixture.pattern.length*dynamicFixture.repeat);const dynamicWire=deflateSync(dynamicRaw,{level:dynamicFixture.level});expect((dynamicWire[2]>>>1)&3).toBe(dynamicFixture.blockKind);expect([dynamicWire[0],dynamicWire[1],dynamicWire.readUInt32BE(dynamicWire.length-4)]).toEqual([dynamicFixture.cmf,dynamicFixture.flg,dynamicFixture.adler]);const dynamic=await parseCompression([...dynamicWire],async()=>{});expect(dynamic.stream.blocks[0].kind).toBe(2);expect(dynamic.raw).toEqual([...inflateSync(dynamicWire)]);expect((await encodeCompression(dynamic.stream,async()=>{})).bytes).toEqual([...dynamicWire]);
 expect(fixture["role"]).toEqual("interpretedNativeScanlines");expect(fixture["compressionAuthority"]).toEqual("completeBlockCodeTokenPadding");
 for(const item of fixture.cases)for(const level of [0,6]){const compressed=deflateSync(Buffer.from(item.raw),{level}),recipe=await parseCompression([...compressed],async()=>{});expect(recipe.raw).toEqual(item.raw);expect((await encodeCompression(recipe.stream,async()=>{})).bytes).toEqual([...compressed]);const visible={...item,raw:item.raw.slice(0,item.raw.length-item.tail.length)};expect([...PNG.sync.read(native(visible,level)).data]).toEqual(item.rgba);const source=native(item,level),model=await parsePngNative([...source],async()=>{});expect(model).not.toBeNull();expect(model!.scanlines.rows.flatMap(row=>row.samples)).toEqual(item.samples);expect(model!.scanlines.tail).toEqual(item.tail);expect(await encodePngNative(model!,async()=>{})).toEqual([...source]);}
 for(const item of fixture.cases)for(const level of [0,6]){
  const compressed=deflateSync(Buffer.from(item.raw),{level}),recipe=await parseCompression([...compressed],async()=>{});expect(recipe.raw).toEqual(item.raw);expect((await encodeCompression(recipe.stream,async()=>{})).bytes).toEqual([...compressed]);
  const source=native(item,level),owner={schema:"stdio.png",bytes:[...source]},db=Database.deserialize(await exportSqliteDatabase(await pngSnapshotToSqliteDatabase(owner)));
  try{
   expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
   expect(db.query("SELECT filter FROM png_scanline ORDER BY ordinal").all().map((r:any)=>r.filter)).toEqual(item.filters);
   expect(db.query("SELECT value FROM png_sample ORDER BY scanline_id,ordinal").all().map((r:any)=>r.value)).toEqual(item.samples);
   expect(db.query("SELECT value FROM png_packed_remainder ORDER BY id").all().map((r:any)=>r.value)).toEqual(item.unusedBits);
   expect(db.query("SELECT value FROM png_inflated_tail ORDER BY ordinal").all().map((r:any)=>r.value)).toEqual(item.tail);
   expect(db.query("SELECT COUNT(*) AS count FROM png_deflate_block").get()).toMatchObject({count:expect.any(Number)});
   expect(db.query("SELECT kind FROM png_chunk ORDER BY ordinal").all().map((r:any)=>r.kind)).toEqual(["IHDR","IDAT","IDAT","abCd","abCd","IEND"]);
   const restored=await pngSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()));expect(restored).toEqual(owner);
   const idat:Buffer[]=[];for(let at=8;at<source.length;){const length=source.readUInt32BE(at);if(source.toString("ascii",at+4,at+8)==="IDAT")idat.push(source.subarray(at+8,at+8+length));at+=length+12}expect([...inflateSync(Buffer.concat(idat))]).toEqual(item.raw);
   if(item.tail.length===0){const independent=PNG.sync.read(source);expect(independent.width).toBe(item.width);expect(independent.height).toBe(item.height)}
  }finally{db.close()}
 }
});
