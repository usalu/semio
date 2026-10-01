import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import floats from "../../🧫️fixtures/🪶️sqlite/🔢️float32.json";
import indices from "../../🧫️fixtures/🪶️sqlite/🧭️indices.json";
import schema from "../../🔣️.json";
import textSchema from "../../📝️text/🔣️.json";
import {buildSchema,graphqlSync} from "graphql";
import type {WavSnapshot} from "../../🟦️.ts";
import {parseWavSnapshot} from "../../🟦️.ts";
import {binary32Value} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import {WAV_SQLITE_SCHEMA,wavSnapshotToSqliteDatabase,wavSnapshotFromSqliteDatabase} from "../../🪶️sqlite/🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

function fixtureSnapshot():WavSnapshot{return parseWavSnapshot({...fixture,chunkOrder:fixture.chunkOrder.map(reference=>reference.kind==="other"?{kind:"other",value:BigInt(reference.value!)}:{kind:reference.kind})})}

test("WAV unsigned64 auxiliary indices retain independently queryable optional relationships",async()=>{
 const snapshot:WavSnapshot={...fixtureSnapshot(),chunkOrder:indices.unsigned64ChunkIndices.map(value=>({kind:"other",value:BigInt(value)}))};const database=await wavSnapshotToSqliteDatabase(snapshot);expect(await wavSnapshotFromSqliteDatabase(database)).toEqual(snapshot);const db=Database.deserialize(await exportSqliteDatabase(database));try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(db.query("SELECT auxiliary_chunk_index,other_chunk_id FROM wav_chunk_order ORDER BY ordinal").all()).toEqual(indices.unsigned64ChunkIndices.map((value,index)=>({auxiliary_chunk_index:value,other_chunk_id:index<2?index+1:null})));db.run("UPDATE wav_chunk_order SET auxiliary_chunk_index='1',other_chunk_id=2 WHERE ordinal=4");const expected={...snapshot,chunkOrder:snapshot.chunkOrder.map((value,index)=>index===4?{kind:"other" as const,value:1n}:value)};expect(await wavSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(expected);db.run("UPDATE wav_chunk_order SET auxiliary_chunk_index='18446744073709551615',other_chunk_id=1 WHERE ordinal=4");await expect(wavSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow();}finally{db.close();}
});

test("WAV shared corpus exposes samples, format extensions and ordered chunk references",async()=>{
 expect(new Ajv({strict:false}).validate(schema,fixture)).toBe(true);
 expect(WAV_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql",import.meta.url)).text());
 const snapshot=fixtureSnapshot();const database=await wavSnapshotToSqliteDatabase(snapshot);expect(await wavSnapshotFromSqliteDatabase(database)).toEqual(snapshot);
 const db=Database.deserialize(await exportSqliteDatabase(database));try{
  expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(db.query("SELECT o.ordinal,c.fourcc,c.ordinal AS chunk_ordinal FROM wav_chunk_order o JOIN wav_other_chunk c ON c.id=o.other_chunk_id ORDER BY o.ordinal").all()).toEqual([{ordinal:0,fourcc:"LIST",chunk_ordinal:1},{ordinal:2,fourcc:"fmt ",chunk_ordinal:0},{ordinal:4,fourcc:"LIST",chunk_ordinal:1}]);
  db.run("UPDATE wav_pcm16_sample SET sample=42 WHERE ordinal=0");db.run("UPDATE wav_format SET channels=3");db.run("UPDATE wav_other_chunk_byte SET octet=99 WHERE chunk_id=1 AND ordinal=1");
  const expected={...snapshot,fmt:{...snapshot.fmt,channels:3},data:{kind:"pcm16" as const,value:[42,-1,0,1,32767]},otherChunks:[{...snapshot.otherChunks[0]!,data:[0,99,17]},snapshot.otherChunks[1]!]};
  expect(await wavSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(expected);
 }finally{db.close();}
});

test("WAV keeps all four sample domains, signed zero, optional empty tails and semantic snapshot fields",async()=>{
 const values=floats.ieee754Binary32Bits.map(bits=>({bits}));
 for(const data of [{kind:"pcm16" as const,value:[-32768,32767]},{kind:"pcm8" as const,value:[0,255]},{kind:"float32" as const,value:values},{kind:"raw" as const,value:[0,255,127]}]){
  const snapshot:WavSnapshot={...fixtureSnapshot(),schema:"custom 世界",fmt:{...fixture.fmt,ext:[]},fmtPadByte:255,data,dataPadByte:255,otherChunks:[{fourcc:"自由 🎶",data:[],padByte:255}],chunkOrder:[]};
  const database=await wavSnapshotToSqliteDatabase(snapshot);const restored=await wavSnapshotFromSqliteDatabase(await importSqliteDatabase(await exportSqliteDatabase(database)));expect(restored).toEqual(snapshot);if(restored.data.kind==="float32")expect(Object.is(binary32Value(restored.data.value[1]!),-0)).toBe(true);
  const db=Database.deserialize(await exportSqliteDatabase(database));try{expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);if(data.kind==="float32"){expect(db.query("SELECT numeric_class,COUNT(*) AS n FROM wav_float32_sample GROUP BY numeric_class ORDER BY numeric_class").all()).toEqual([{numeric_class:"finite",n:6},{numeric_class:"nan",n:3},{numeric_class:"negative_infinity",n:1},{numeric_class:"negative_zero",n:1},{numeric_class:"positive_infinity",n:1}]);}}finally{db.close();}
 }
 for(const ext of [undefined,[]]){const {fmtPadByte,...document}=fixtureSnapshot();const {ext:original,...format}=fixture.fmt;const snapshot:WavSnapshot={...document,fmt:{...format,...(ext===undefined?{}:{ext})}};expect(await wavSnapshotFromSqliteDatabase(await wavSnapshotToSqliteDatabase(snapshot))).toEqual(snapshot);}
});

test("WAV rejects malformed relational ownership and honors bounded cancellation",async()=>{
 const database=await wavSnapshotToSqliteDatabase(fixtureSnapshot());
 for(const sql of ["UPDATE wav_pcm16_sample SET data_id=9 WHERE id=1","UPDATE wav_pcm16_sample SET ordinal=0 WHERE id=2","UPDATE wav_format SET channels=65536","UPDATE wav_chunk_order SET chunk_kind='samples' WHERE id=1","UPDATE wav_other_chunk_byte SET chunk_id=99","INSERT INTO wav_pcm8_sample VALUES(1,1,0,1)","DELETE FROM wav_format_extension"]){const db=Database.deserialize(await exportSqliteDatabase(database));try{db.run("PRAGMA ignore_check_constraints=ON");db.run(sql);await expect(wavSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow();}finally{db.close();}}
 await expect(wavSnapshotToSqliteDatabase(fixtureSnapshot(),{maxRows:1})).rejects.toThrow();await expect(wavSnapshotFromSqliteDatabase(database,{maxValueBytes:0})).rejects.toThrow();
 const controller=new AbortController();await expect(wavSnapshotToSqliteDatabase({...fixtureSnapshot(),data:{kind:"pcm16",value:new Array<number>(2000).fill(1)}},{signal:controller.signal,onProgress:event=>{if(event.completed>=256)controller.abort();}})).rejects.toHaveProperty("name","AbortError");
 const read=new AbortController();await expect(wavSnapshotFromSqliteDatabase(database,{signal:read.signal,onProgress:()=>read.abort()})).rejects.toHaveProperty("name","AbortError");
});

test("WAV canonical float words preserve every neutral NaN payload through independent SQL edits",async()=>{
 const snapshot:WavSnapshot={...fixtureSnapshot(),data:{kind:"float32",value:floats.ieee754Binary32Bits.map(bits=>({bits}))}};expect(parseWavSnapshot(snapshot)).toEqual(snapshot);const database=await wavSnapshotToSqliteDatabase(snapshot);expect(await wavSnapshotFromSqliteDatabase(database)).toEqual(snapshot);const db=Database.deserialize(await exportSqliteDatabase(database));try{expect(db.query("SELECT ieee754_binary32_bits AS bits FROM wav_float32_sample ORDER BY ordinal").all()).toEqual(floats.ieee754Binary32Bits.map(bits=>({bits})));db.run("UPDATE wav_float32_sample SET ieee754_binary32_bits=2143289429,numeric_class='nan',sample=NULL WHERE ordinal=0");const restored=await wavSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));expect(restored.data).toEqual({kind:"float32",value:[{bits:2143289429},...floats.ieee754Binary32Bits.slice(1).map(bits=>({bits}))]});}finally{db.close();}
});

test("WAV independent full JSON and GraphQL facets preserve unsigned64 and binary32 words",async()=>{const document={...fixture,schema:"owned 世界\0",fmt:{...fixture.fmt,ext:new Array<number>(65536).fill(255)},fmtPadByte:255,dataPadByte:255,otherChunks:[{fourcc:"自由 🎶",data:[],padByte:255}],data:{kind:"float32",value:floats.ieee754Binary32Bits.map(bits=>({bits}))},chunkOrder:indices.unsigned64ChunkIndices.map(value=>({kind:"other",value}))};const ajv=new Ajv({strict:false});expect(ajv.validate(schema,document)).toBe(true);expect(ajv.validate(textSchema,document)).toBe(true);for(const value of ["18446744073709551616","01","-1","1.0"]){expect(ajv.validate(schema,{...document,chunkOrder:[{kind:"other",value}]})).toBe(false);}const sdl=await Bun.file(new URL("../../🔗️.graphql",import.meta.url)).text();const graph=buildSchema("enum StateClass { ARTIFACT SESSION } directive @state(class: StateClass!) on FIELD_DEFINITION\n"+sdl+"\ntype Query { snapshot: WavSnapshot! }");const result=graphqlSync({schema:graph,source:"{ snapshot { schema fmt { audioFormat channels sampleRate byteRate blockAlign bitsPerSample ext } data { ... on WavFloat32Data { kind value { bits } } } fmtPadByte dataPadByte otherChunks { fourcc data padByte } chunkOrder { kind value } } }",rootValue:{snapshot:document},typeResolver:value=>({pcm16:"WavPcm16Data",pcm8:"WavPcm8Data",float32:"WavFloat32Data",raw:"WavRawData"})[value.kind as "pcm16"|"pcm8"|"float32"|"raw"]});expect(result.errors).toBeUndefined();expect(JSON.parse(JSON.stringify(result.data?.snapshot))).toEqual(document);});
