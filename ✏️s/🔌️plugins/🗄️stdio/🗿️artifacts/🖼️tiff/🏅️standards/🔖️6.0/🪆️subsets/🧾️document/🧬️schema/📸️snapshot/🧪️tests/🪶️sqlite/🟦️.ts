/** 🧫️ TIFF canonical storage and exact typed values through an independent SQLite engine. */
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import f from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import {parseTiffSnapshot,type TiffSnapshot,type TiffValues} from "../../🟦️.ts";
import {TIFF_SQLITE_SCHEMA,tiffSnapshotToSqliteDatabase,tiffSnapshotFromSqliteDatabase,tiffSnapshotToSqliteFile,tiffSnapshotFromSqliteFile} from "../../🪶️sqlite/🟦️.ts";
import {exportSqliteDatabase} from "@semio-tech/framework";

const values:TiffValues[]=[
  {kind:"byte",value:[0,255]},
  {kind:"ascii",value:f.asciiBytes},
  {kind:"short",value:[0,65535]},
  {kind:"long",value:[0,4294967295]},
  {kind:"rational",value:[[4294967295,0],[1,4294967295]]},
  {kind:"sByte",value:[-128,127]},
  {kind:"undefined",value:[0,255]},
  {kind:"sShort",value:[-32768,32767]},
  {kind:"sLong",value:[-2147483648,2147483647]},
  {kind:"sRational",value:[[-2147483648,0],[2147483647,-1]]},
  {kind:"float",value:f.float32Bits.map(bits=>({bits:parseInt(bits,16)}))},
  {kind:"double",value:f.float64Bits.map(bits=>({bits:BigInt(`0x${bits}`)}))},
];
const input:TiffSnapshot={
  schema:f.schema,
  byteOrder:"bigEndian",
  ifds:[
    {entries:values.map(values=>({tag:65535,values})),storage:{kind:"strips",offsetsKind:"short",byteCountsKind:"long",chunks:f.primaryChunks}},
    {entries:[{tag:0,values:{kind:"byte",value:[]}}],storage:{kind:"tiles",offsetsKind:"long",byteCountsKind:"short",chunks:f.secondaryChunks}},
  ],
};

const table=(database:any,name:string)=>database.tables.find((candidate:any)=>candidate.name===name);

test("TIFF projects canonical chunk partitions and all exact scalar domains",async()=>{
  expect(TIFF_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql",import.meta.url)).text());
  expect(parseTiffSnapshot(input)).toEqual(input);
  const projected=await tiffSnapshotToSqliteDatabase(input);
  expect(projected.tables.length).toBe(16);
  expect(projected.tables.reduce((total,current)=>total+current.rows.length,0)).toBe(59);
  const independent=Database.deserialize(await exportSqliteDatabase(projected));
  try{
    expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
    expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(independent.query("SELECT storage_kind,offsets_type,byte_counts_type FROM tiff_ifd ORDER BY ordinal").all()).toEqual([
      {storage_kind:"strips",offsets_type:3,byte_counts_type:4},
      {storage_kind:"tiles",offsets_type:4,byte_counts_type:3},
    ]);
    expect(independent.query("SELECT ordinal,hex(payload) AS payload FROM tiff_chunk ORDER BY ifd_id,ordinal").all()).toEqual([
      {ordinal:0,payload:"00FF"},{ordinal:1,payload:"4080"},{ordinal:0,payload:"090800FF"},
    ]);
    expect(independent.query("SELECT value_bits,value FROM tiff_float_value WHERE ordinal=6").get()).toEqual({value_bits:2139095047,value:null});
    expect(await tiffSnapshotFromSqliteFile(independent.serialize())).toEqual(input);
  }finally{independent.close();}
});

test("TIFF independently edited ASCII octets, rationals, and one chunk reconstruct exactly",async()=>{
  const independent=Database.deserialize(await tiffSnapshotToSqliteFile(input));
  try{
    independent.query("DELETE FROM tiff_ascii_value").run();
    for(const [ordinal,value] of f.editedAsciiBytes.entries())independent.query("INSERT INTO tiff_ascii_value(tag_id,ordinal,value) VALUES(2,?,?)").run(ordinal,value);
    independent.query("UPDATE tiff_rational_value SET denominator=7 WHERE ordinal=0").run();
    independent.query("UPDATE tiff_chunk SET payload=x'030201' WHERE ifd_id=2 AND ordinal=0").run();
    const expected=structuredClone(input);
    expected.ifds[0]!.entries[1]!.values={kind:"ascii",value:f.editedAsciiBytes};
    expected.ifds[0]!.entries[4]!.values={kind:"rational",value:[[4294967295,7],[1,4294967295]]};
    expected.ifds[1]!.storage.chunks[0]=[3,2,1];
    expect(await tiffSnapshotFromSqliteFile(independent.serialize())).toEqual(expected);
  }finally{independent.close();}
});

test("TIFF refuses malformed relationships, native widths, storage invariants, and IEEE companions",async()=>{
  const original=await tiffSnapshotToSqliteDatabase(input);
  for(const alter of [
    (database:any)=>table(database,"tiff_tag").rows[0].values[1]=999n,
    (database:any)=>table(database,"tiff_short_value").rows[0].values[3]=65536n,
    (database:any)=>table(database,"tiff_chunk").rows[1].values[2]=0n,
    (database:any)=>table(database,"tiff_float_value").rows[5].values[5]="finite",
  ]){
    const malformed=structuredClone(original);alter(malformed);
    await expect(tiffSnapshotFromSqliteDatabase(malformed)).rejects.toThrow();
  }
  expect(()=>parseTiffSnapshot({...input,ifds:[{entries:[],storage:{kind:"none",offsetsKind:"long",byteCountsKind:"long",chunks:[[1]]}}]})).toThrow();
  expect(()=>parseTiffSnapshot({...input,ifds:[{entries:[],storage:{kind:"strips",offsetsKind:"ascii",byteCountsKind:"long",chunks:[[1]]}}]})).toThrow();
  expect(()=>parseTiffSnapshot({...input,ifds:[{entries:[{tag:1,values:{kind:"float",value:[{bits:-1}]}}],storage:{kind:"none",offsetsKind:"long",byteCountsKind:"long",chunks:[]}}]})).toThrow();
  await expect(tiffSnapshotToSqliteDatabase(input,{maxRows:58})).rejects.toThrow();
  await expect(tiffSnapshotFromSqliteDatabase(original,{maxValueBytes:1})).rejects.toThrow();
});

test("TIFF empty directories and empty typed sequences retain explicit presence",async()=>{
  const empty:TiffSnapshot={schema:"custom 世界",byteOrder:"littleEndian",ifds:[{entries:[{tag:0,values:{kind:"ascii",value:[]}},{tag:1,values:{kind:"double",value:[]}}],storage:{kind:"none",offsetsKind:"short",byteCountsKind:"long",chunks:[]}}]};
  expect(await tiffSnapshotFromSqliteDatabase(await tiffSnapshotToSqliteDatabase(empty))).toEqual(empty);
  expect(await tiffSnapshotFromSqliteDatabase(await tiffSnapshotToSqliteDatabase({...empty,ifds:[]}))).toEqual({...empty,ifds:[]});
});

test("TIFF chunk materialization is cancellable in both semantic directions",async()=>{
  const size=100_000,large:TiffSnapshot=structuredClone(input);
  large.ifds[0]!.storage.chunks=[Array.from({length:size},(_,index)=>index%256)];
  const projected=await tiffSnapshotToSqliteDatabase(large);
  for(const phase of ["projectSnapshot","reconstructSnapshot"]as const){
    const abort=new AbortController();let observed=false;
    const options={signal:abort.signal,onProgress:(progress:{phase:string,completed:number,total:number})=>{if(progress.phase===phase&&progress.total===size&&progress.completed>0&&progress.completed<size){observed=true;abort.abort();}}};
    await expect(phase==="projectSnapshot"?tiffSnapshotToSqliteDatabase(large,options):tiffSnapshotFromSqliteDatabase(projected,options)).rejects.toThrow();
    expect(observed).toBe(true);
  }
});


test("TIFF closed neutral custom schema and absent storage survive independent physical SQLite",async()=>{
  const neutral=await Bun.file(new URL("../../🧫️fixtures/🪶️sqlite/🧾️native-owner/🔣️.json",import.meta.url)).json();
  const closed=await Bun.file(new URL("../../🧫️fixtures/🪶️sqlite/🧾️native-owner/🧬️schema/🔣️.json",import.meta.url)).json();
  const {default:Ajv}=await import("ajv");
  const validate=new Ajv({strict:true}).compile(closed);
  expect(validate(neutral)).toBe(true);
  expect(validate({...neutral,unexpected:true})).toBe(false);
  const owner=parseTiffSnapshot(neutral);
  const independent=Database.deserialize(await tiffSnapshotToSqliteFile(owner));
  try{
    expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
    expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(independent.query("SELECT schema,byte_order FROM tiff_document").get()).toEqual({schema:neutral.schema,byte_order:neutral.byteOrder});
    expect(independent.query("SELECT storage_kind,offsets_type,byte_counts_type FROM tiff_ifd").get()).toEqual({storage_kind:"none",offsets_type:3,byte_counts_type:4});
    expect(independent.query("SELECT ordinal,tag_number,value_type FROM tiff_tag ORDER BY ordinal").all()).toEqual([{ordinal:0,tag_number:0,value_type:2},{ordinal:1,tag_number:1,value_type:12}]);
    expect(independent.query("SELECT COUNT(*) AS count FROM tiff_chunk").get()).toEqual({count:0});
    expect(await tiffSnapshotFromSqliteFile(independent.serialize())).toEqual(owner);
  }finally{independent.close();}
});
