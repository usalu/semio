/** 🧫️ Exact typed TIFF metadata and sample identities through an independent SQLite engine. */
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import f from "../🧫️fixtures/🔣️.json";
import native from "../../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json";
import semantic from "../🧫️fixtures/🎛️semantic.json";
import {independentSqliteExtent} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔮️semantic-extent/🟦️.ts";
import {parseTiffSnapshot,type TiffSnapshot,type TiffValues} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {TIFF_SQLITE_SCHEMA,tiffSnapshotToSqliteDatabase,tiffSnapshotFromSqliteDatabase,tiffSnapshotToSqliteFile,tiffSnapshotFromSqliteFile} from "../🟦️.ts";
import {exportSqliteDatabase} from "@semio-tech/framework";
const values:TiffValues[]=[
 {kind:"byte",value:[0,255]},{kind:"ascii",value:f.asciiTexts},{kind:"short",value:[0,65535]},{kind:"long",value:[0,4294967295]},{kind:"rational",value:[[4294967295,0],[1,4294967295]]},{kind:"sByte",value:[-128,127]},{kind:"undefined",value:[0,255]},{kind:"sShort",value:[-32768,32767]},{kind:"sLong",value:[-2147483648,2147483647]},{kind:"sRational",value:[[-2147483648,0],[2147483647,-1]]},{kind:"float",value:f.float32Bits.map(bits=>({bits:parseInt(bits,16)}))},{kind:"double",value:f.float64Words}
];
const input:TiffSnapshot={schema:"stdio.tiff",ifds:[{entries:values.map((values,i)=>({tag:500+i,values})),blocks:[]},parseTiffSnapshot(native.cases[1]!.snapshot).ifds[0]!]};
const table=(db:any,name:string)=>db.tables.find((candidate:any)=>candidate.name===name);
test("neutral complete relational extents match independently inspected SQLite",async()=>{const actual=[];for(const vector of semantic.cases){const source=parseTiffSnapshot(vector.input),extent=independentSqliteExtent(await tiffSnapshotToSqliteFile(source));actual.push(extent);}for(let i=0;i<semantic.cases.length;i++){const vector=semantic.cases[i]!;expect(actual[i]).toEqual({rows:vector.rows,valueBytes:vector.valueBytes,schemaBytes:semantic.schemaBytes,tableWidths:semantic.tableWidths});}});

test("TIFF owned sample words and all twelve scalar domains survive SQLite",async()=>{
 expect(TIFF_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql",import.meta.url)).text());expect(parseTiffSnapshot(input)).toEqual(input);
 const projected=await tiffSnapshotToSqliteDatabase(input);expect(projected.tables.length).toBe(17);
 const independent=Database.deserialize(await exportSqliteDatabase(projected));try{
  expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(independent.query("SELECT lo,hi FROM tiff_sample ORDER BY ordinal").all()).toEqual(native.cases[1]!.expectedSamples);
  expect(independent.query("SELECT lo,hi FROM tiff_double_value ORDER BY ordinal").all()).toEqual(f.float64Words);
  expect(independent.query("SELECT value FROM tiff_ascii_value ORDER BY ordinal").all()).toEqual(f.asciiTexts.map(value=>({value})));
  expect(independent.query("SELECT value_bits,value FROM tiff_float_value WHERE ordinal=6").get()).toEqual({value_bits:2139095047,value:null});
  expect(await tiffSnapshotFromSqliteFile(independent.serialize())).toEqual(input);
 }finally{independent.close();}
});
test("independent owned text, rational and NaN payload edits reconstruct exactly",async()=>{
 const independent=Database.deserialize(await tiffSnapshotToSqliteFile(input));try{
  independent.query("DELETE FROM tiff_ascii_value").run();independent.query("INSERT INTO tiff_ascii_value(tag_id,ordinal,value) VALUES(2,0,?)").run(f.editedAsciiTexts[0]!);
  independent.query("UPDATE tiff_rational_value SET denominator=7 WHERE ordinal=0").run();
  independent.query("UPDATE tiff_sample SET lo=23 WHERE ordinal=0").run();const expected=structuredClone(input);
  expected.ifds[0]!.entries[1]!.values={kind:"ascii",value:f.editedAsciiTexts};expected.ifds[0]!.entries[4]!.values={kind:"rational",value:[[4294967295,7],[1,4294967295]]};expected.ifds[1]!.blocks[0]!.samples[0]!.lo=23;
  expect(await tiffSnapshotFromSqliteFile(independent.serialize())).toEqual(expected);
 }finally{independent.close();}
});
test("foreign parents, ordinal holes, exact word bounds and IEEE contradictions refuse",async()=>{
 const original=await tiffSnapshotToSqliteDatabase(input);
 for(const alter of [
  (db:any)=>table(db,"tiff_tag").rows[0].values[1]=999n,
  (db:any)=>table(db,"tiff_short_value").rows[0].values[3]=65536n,
  (db:any)=>table(db,"tiff_sample").rows[1].values[2]=0n,
  (db:any)=>table(db,"tiff_sample").rows[0].values[3]=4294967296n,
  (db:any)=>table(db,"tiff_float_value").rows[5].values[5]="finite"
 ]){const malformed=structuredClone(original);alter(malformed);await expect(tiffSnapshotFromSqliteDatabase(malformed)).rejects.toThrow();}
 await expect(tiffSnapshotToSqliteDatabase(input,{maxRows:1})).rejects.toThrow();await expect(tiffSnapshotFromSqliteDatabase(original,{maxValueBytes:1})).rejects.toThrow();
});
test("empty directories and exact scalar sequence presence remain owned",async()=>{
 const empty:TiffSnapshot={schema:"stdio.tiff",ifds:[{entries:[{tag:0,values:{kind:"ascii",value:[]}},{tag:1,values:{kind:"double",value:[]}}],blocks:[]}]};
 expect(await tiffSnapshotFromSqliteDatabase(await tiffSnapshotToSqliteDatabase(empty))).toEqual(empty);
 expect(await tiffSnapshotFromSqliteDatabase(await tiffSnapshotToSqliteDatabase({...empty,ifds:[]}))).toEqual({...empty,ifds:[]});
});
test("sample row materialization receives cancellation in both directions",async()=>{
 const large=parseTiffSnapshot(native.cases[0]!.snapshot);large.ifds[0]!.entries[0]!.values={kind:"long",value:[10000]};large.ifds[0]!.blocks[0]!.width=10000;large.ifds[0]!.blocks[0]!.samples=Array.from({length:10000},(_,i)=>({lo:i,hi:0}));const projected=await tiffSnapshotToSqliteDatabase(large);
 for(const phase of ["projectSnapshot","reconstructSnapshot"] as const){const abort=new AbortController();let calls=0;const options={signal:abort.signal,onProgress:(event:{phase:string})=>{if(event.phase===phase&&++calls===4)abort.abort();}};await expect(phase==="projectSnapshot"?tiffSnapshotToSqliteDatabase(large,options):tiffSnapshotFromSqliteDatabase(projected,options)).rejects.toThrow();expect(calls).toBe(4);}
});
