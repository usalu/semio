import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔣️.json";
import floats from "../🧫️fixtures/🔢️float32.json";
import indices from "../🧫️fixtures/🧭️indices.json";
import integerQueries from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🧫️fixtures/🎯️integer-query.json";
import schema from "../../../../🧬️schema/📸️snapshot/🔣️.json";
import textSchema from "../../../📝️text/📸️snapshot/🔣️.json";
import {buildSchema,graphqlSync} from "graphql";
import type {WavSnapshot} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {parseWavSnapshot} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {binary32Value} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import {WAV_SQLITE_SCHEMA,wavSnapshotToSqliteDatabase,wavSnapshotFromSqliteDatabase} from "../🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase,SqliteOperation} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

function fixtureSnapshot():WavSnapshot{return parseWavSnapshot({...fixture,chunkOrder:fixture.chunkOrder.map(reference=>reference.kind==="other"?{kind:"other",value:BigInt(reference.value!)}:{kind:reference.kind})})}

test("WAV INTEGER query samples agree with independently measured binary32 words",async()=>{
 const oracle=new Database(":memory:",{safeIntegers:true}),view=new DataView(new ArrayBuffer(4));
 try{
  oracle.run("CREATE TABLE exact_query(sample BLOB)");
  for(const item of integerQueries.cases){
   const integer=BigInt(item.integer),bits=parseInt(item.binary32Bits,16);view.setUint32(0,bits);expect(BigInt(view.getFloat32(0))===integer).toBe(item.accept32);
   oracle.run("DELETE FROM exact_query");oracle.run("INSERT INTO exact_query VALUES(?)",[integer]);const query=(oracle.query("SELECT sample FROM exact_query").get()as{sample:bigint}).sample;expect(query).toBe(integer);
   const snapshot:WavSnapshot={...fixtureSnapshot(),data:{kind:"float32",value:[{bits}]}};const database=await wavSnapshotToSqliteDatabase(snapshot);
   const edited={...database,tables:database.tables.map(table=>table.name!=="wav_float32_sample"?table:{...table,rows:table.rows.map(row=>({...row,values:row.values.map((value,index)=>index===5?query:value)}))})};
   if(item.accept32)expect(await wavSnapshotFromSqliteDatabase(edited)).toEqual(snapshot);else await expect(wavSnapshotFromSqliteDatabase(edited)).rejects.toThrow();
  }
 }finally{oracle.close();}
});

test("WAV unsigned64 auxiliary indices retain independently queryable optional relationships",async()=>{
 const snapshot:WavSnapshot={...fixtureSnapshot(),chunkOrder:indices.unsigned64ChunkIndices.map(value=>({kind:"other",value:BigInt(value)}))};const database=await wavSnapshotToSqliteDatabase(snapshot);expect(await wavSnapshotFromSqliteDatabase(database)).toEqual(snapshot);const db=Database.deserialize(await exportSqliteDatabase(database));try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(db.query("SELECT auxiliary_chunk_index,other_chunk_id FROM wav_chunk_order ORDER BY ordinal").all()).toEqual(indices.unsigned64ChunkIndices.map((value,index)=>({auxiliary_chunk_index:value,other_chunk_id:index<2?index+1:null})));db.run("UPDATE wav_chunk_order SET auxiliary_chunk_index='1',other_chunk_id=2 WHERE ordinal=4");const expected={...snapshot,chunkOrder:snapshot.chunkOrder.map((value,index)=>index===4?{kind:"other" as const,value:1n}:value)};expect(await wavSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(expected);db.run("UPDATE wav_chunk_order SET auxiliary_chunk_index='18446744073709551615',other_chunk_id=1 WHERE ordinal=4");await expect(wavSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow();}finally{db.close();}
});

test("WAV shared corpus exposes samples, format extensions and ordered chunk references",async()=>{
 expect(new Ajv({strict:false}).validate(schema,fixture)).toBe(true);
 expect(WAV_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql",import.meta.url)).text());
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
 for(const ext of [undefined,[]]){const {fmtPadByte,...document}=fixtureSnapshot();const {ext:original,...format}=fixture.fmt;const snapshot:WavSnapshot={...document,fmtPadByte:0,fmt:{...format,...(ext===undefined?{}:{ext})}};expect(await wavSnapshotFromSqliteDatabase(await wavSnapshotToSqliteDatabase(snapshot))).toEqual(snapshot);}
});

test("WAV rejects malformed relational ownership and honors bounded cancellation",async()=>{
 const database=await wavSnapshotToSqliteDatabase(fixtureSnapshot());
 for(const sql of ["UPDATE wav_pcm16_sample SET data_id=9 WHERE id=1","UPDATE wav_pcm16_sample SET ordinal=0 WHERE id=2","UPDATE wav_format SET channels=65536","UPDATE wav_chunk_order SET chunk_kind='samples' WHERE id=1","UPDATE wav_other_chunk_byte SET chunk_id=99","INSERT INTO wav_pcm8_sample VALUES(1,1,0,1)","DELETE FROM wav_format_extension"]){const db=Database.deserialize(await exportSqliteDatabase(database));try{db.run("PRAGMA ignore_check_constraints=ON");db.run(sql);await expect(wavSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow();}finally{db.close();}}
 await expect(wavSnapshotToSqliteDatabase(fixtureSnapshot(),{maxRows:1})).rejects.toThrow();await expect(wavSnapshotFromSqliteDatabase(database,{maxValueBytes:0})).rejects.toThrow();
 const controller=new AbortController();await expect(wavSnapshotToSqliteDatabase({...fixtureSnapshot(),data:{kind:"pcm16",value:new Array<number>(2000).fill(1)}},{signal:controller.signal,onProgress:event=>{if(event.completed>=256)controller.abort();}})).rejects.toHaveProperty("kind","canceled");
 const read=new AbortController();await expect(wavSnapshotFromSqliteDatabase(database,{signal:read.signal,onProgress:()=>read.abort()})).rejects.toHaveProperty("kind","canceled");
});

test("WAV canonical float words preserve every neutral NaN payload through independent SQL edits",async()=>{
 const snapshot:WavSnapshot={...fixtureSnapshot(),data:{kind:"float32",value:floats.ieee754Binary32Bits.map(bits=>({bits}))}};expect(parseWavSnapshot(snapshot)).toEqual(snapshot);const database=await wavSnapshotToSqliteDatabase(snapshot);expect(await wavSnapshotFromSqliteDatabase(database)).toEqual(snapshot);const db=Database.deserialize(await exportSqliteDatabase(database));try{expect(db.query("SELECT ieee754_binary32_bits AS bits FROM wav_float32_sample ORDER BY ordinal").all()).toEqual(floats.ieee754Binary32Bits.map(bits=>({bits})));db.run("UPDATE wav_float32_sample SET ieee754_binary32_bits=2143289429,numeric_class='nan',sample=NULL WHERE ordinal=0");const restored=await wavSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));expect(restored.data).toEqual({kind:"float32",value:[{bits:2143289429},...floats.ieee754Binary32Bits.slice(1).map(bits=>({bits}))]});}finally{db.close();}
});

test("WAV independent full JSON and GraphQL facets preserve unsigned64 and binary32 words",async()=>{const document={...fixture,schema:"owned 世界\0",fmt:{...fixture.fmt,ext:new Array<number>(65536).fill(255)},fmtPadByte:255,dataPadByte:255,otherChunks:[{fourcc:"自由 🎶",data:[],padByte:255}],data:{kind:"float32",value:floats.ieee754Binary32Bits.map(bits=>({bits}))},chunkOrder:indices.unsigned64ChunkIndices.map(value=>({kind:"other",value}))};const ajv=new Ajv({strict:false});expect(ajv.validate(schema,document)).toBe(true);expect(ajv.validate(textSchema,document)).toBe(true);for(const value of ["18446744073709551616","01","-1","1.0"]){expect(ajv.validate(schema,{...document,chunkOrder:[{kind:"other",value}]})).toBe(false);}const sdl=await Bun.file(new URL("../../../../🧬️schema/📸️snapshot/🔗️.graphql",import.meta.url)).text();const graph=buildSchema("enum StateClass { ARTIFACT SESSION } directive @state(class: StateClass!) on FIELD_DEFINITION\n"+sdl+"\ntype Query { snapshot: WavSnapshot! }");const result=graphqlSync({schema:graph,source:"{ snapshot { schema fmt { audioFormat channels sampleRate byteRate blockAlign bitsPerSample ext } data { ... on WavFloat32Data { kind value { bits } } } fmtPadByte dataPadByte otherChunks { fourcc data padByte } chunkOrder { kind value } } }",rootValue:{snapshot:document},typeResolver:value=>({pcm16:"WavPcm16Data",pcm8:"WavPcm8Data",float32:"WavFloat32Data",raw:"WavRawData"})[value.kind as "pcm16"|"pcm8"|"float32"|"raw"]});expect(result.errors).toBeUndefined();expect(JSON.parse(JSON.stringify(result.data?.snapshot))).toEqual(document);});

test("WAV full request contract retains independently queryable owned fields",async()=>{
 const plan=await Bun.file(new URL("../🧫️fixtures/💰️backing/🔬️requests/🔣️.json",import.meta.url)).json();
 
 
 expect(plan["phases"]).toEqual(["projectSnapshot","reconstructSnapshot"]);expect(plan["requestExtent"]).toEqual("fullConcreteRequests");expect(plan["reallocation"]).toEqual("completeReplacement");expect(plan["exactReplay"]).toEqual(true);expect(plan["denial"]).toEqual("oneByteBelowObserved");expect(plan["retirementRefund"]).toEqual(false);expect(plan["refusalKind"]).toEqual("ownershipLimit");expect(plan["tableCount"]).toEqual(12);
 
 
 
 const input=fixtureSnapshot();
 const physical=Database.deserialize(await exportSqliteDatabase(await wavSnapshotToSqliteDatabase(input)));
 try{
  expect(physical.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
  expect(physical.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(physical.query("SELECT count(*) AS count FROM sqlite_schema WHERE type='table' AND name LIKE 'wav_%'").get()).toEqual({count:plan.tableCount});
  expect(physical.query("SELECT o.ordinal,c.fourcc,c.ordinal AS chunk_ordinal FROM wav_chunk_order o JOIN wav_other_chunk c ON c.id=o.other_chunk_id ORDER BY o.ordinal").all()).toEqual([{ordinal:0,fourcc:"LIST",chunk_ordinal:1},{ordinal:2,fourcc:"fmt ",chunk_ordinal:0},{ordinal:4,fourcc:"LIST",chunk_ordinal:1}]);
  expect(await wavSnapshotFromSqliteDatabase(await importSqliteDatabase(physical.serialize()))).toEqual(input);
 }finally{physical.close();}
});

test("WAV independent row census closes the intrinsic bounded-work contract",async()=>{
 const laws=await Bun.file(new URL("../🧫️fixtures/🎛️controls.json",import.meta.url)).json();
 
 expect(laws["exactFixtureRows"]).toEqual(22);expect(laws["schemaLimitOffsets"]).toEqual([-1,0]);expect(laws["rowLimitOffsets"]).toEqual([-1,0]);expect(laws["rowRefusalKind"]).toEqual("workLimit");
 
 const input=fixtureSnapshot(),database=await wavSnapshotToSqliteDatabase(input),oracle=Database.deserialize(await exportSqliteDatabase(database));
 try{
  let rows=0;
  for(const{name}of oracle.query("SELECT name FROM sqlite_schema WHERE type='table' AND name LIKE 'wav_%'").all()as{name:string}[]){
   rows+=(oracle.query('SELECT count(*) AS n FROM "'+name.replaceAll('"','""')+'"').get()as{n:number}).n;
  }
  expect(rows).toBe(laws.exactFixtureRows);
  for(const offset of laws.rowLimitOffsets){
   const maximum=rows+offset;
   if(offset===0){expect(await wavSnapshotFromSqliteDatabase(await wavSnapshotToSqliteDatabase(input,{maxRows:maximum}),{maxRows:maximum})).toEqual(input);}
   else{await expect(wavSnapshotToSqliteDatabase(input,{maxRows:maximum})).rejects.toHaveProperty("kind",laws.rowRefusalKind);await expect(wavSnapshotFromSqliteDatabase(database,{maxRows:maximum})).rejects.toHaveProperty("kind",laws.rowRefusalKind);}
  }
 }finally{oracle.close();}
});

import frontiers from "../🧫️fixtures/🧵️frontiers/🔣️.json";

import {validateJsonSchemaSubset} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
function frontierSnapshot(caseKind:"samples"|"group"|"census"):WavSnapshot{const{ext,...fmt}=fixtureSnapshot().fmt;return {schema:frontiers.schema,fmt,fmtPadByte:0,dataPadByte:0,data:{kind:"pcm16",value:caseKind==="samples"?Array.from({length:frontiers.entities},(_,i)=>i+frontiers.sampleBase):[]},otherChunks:caseKind==="samples"?[]:caseKind==="group"?[{padByte:0,fourcc:frontiers.chunkGroup,data:Array.from({length:frontiers.entities},(_,i)=>i%frontiers.octetCycle)}]:Array.from({length:frontiers.entities},(_,i)=>({padByte:0,fourcc:String(i).padStart(frontiers.chunkWidth,"0"),data:[]})),chunkOrder:[]};}
async function frontierDatabase(caseKind:"samples"|"group"|"census"){const snapshot=frontierSnapshot(caseKind),database=await wavSnapshotToSqliteDatabase(snapshot),table=caseKind==="samples"?"wav_pcm16_sample":caseKind==="group"?"wav_other_chunk_byte":"wav_other_chunk";return {snapshot,table,database:{tables:database.tables.map(value=>value.name===table?{...value,rows:[...value.rows].reverse()}:value)}};}
test("WAV frontier corpus is closed and independently preserves reversed ordinal relationships",async()=>{
 expect(frontiers["entities"]).toEqual(769);expect(frontiers["frontier"]).toEqual(256);expect(frontiers["precedingPasses"]).toEqual(2);expect(frontiers["sampleBase"]).toEqual(-384);expect(frontiers["octetCycle"]).toEqual(256);expect(frontiers["chunkWidth"]).toEqual(4);expect(frontiers["chunkGroup"]).toEqual("JUNK");expect(frontiers["schema"]).toEqual("owned WAV frontiers 世界");expect(frontiers["order"]).toEqual("reverse");expect(frontiers["priorDebt"]).toEqual(7);expect(frontiers["capturedRows"]).toEqual(4096);expect(frontiers["refusalKind"]).toEqual("canceled");for(const hostile of [{...frontiers,extra:1},{...frontiers,frontier:1}]){}
 for(const kind of ["samples","group","census"]as const){const input=await frontierDatabase(kind),original=await wavSnapshotToSqliteDatabase(input.snapshot),db=Database.deserialize(await exportSqliteDatabase(original));try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);const reversed=db.query(`SELECT id,ordinal FROM "${input.table}" ORDER BY ordinal DESC`).all();expect(reversed).toEqual(input.database.tables.find(t=>t.name===input.table)!.rows.map(row=>({id:Number(row.rowid),ordinal:Number(row.values[2])})));if(kind==="samples")expect(db.query("SELECT sample FROM wav_pcm16_sample ORDER BY ordinal").all()).toEqual(input.snapshot.data.value.map(sample=>({sample})));else if(kind==="group")expect(db.query("SELECT b.octet FROM wav_other_chunk_byte b JOIN wav_other_chunk c ON c.id=b.chunk_id ORDER BY b.ordinal").all()).toEqual(input.snapshot.otherChunks[0]!.data.map(octet=>({octet})));else expect(db.query("SELECT fourcc FROM wav_other_chunk ORDER BY ordinal").all()).toEqual(input.snapshot.otherChunks.map(chunk=>({fourcc:chunk.fourcc})));expect(await wavSnapshotFromSqliteDatabase(input.database)).toEqual(input.snapshot);}finally{db.close();}}
});
for(const kind of ["samples","group"]as const)test(`WAV actual ${kind} ordinal placement cancels after its preceding owner passes`,async()=>{
 const input=await frontierDatabase(kind),abort=new AbortController();let preceding=0,placing=false,reads=0,reached=false;const rows=input.database.tables.find(t=>t.name===input.table)!.rows;for(const row of rows){const value=row.values[2];Object.defineProperty(row.values,"2",{get:()=>{if(placing)reads++;return value;}});}
 await expect(wavSnapshotFromSqliteDatabase(input.database,{signal:abort.signal,onProgress:event=>{if(event.phase!=="reconstructSnapshot"||event.total!==frontiers.entities)return;if(event.completed===frontiers.entities-1&&++preceding===frontiers.precedingPasses)placing=true;if(placing&&reads>0){reached=true;abort.abort();}}})).rejects.toHaveProperty("kind",frontiers.refusalKind);expect(preceding).toBeGreaterThanOrEqual(frontiers.precedingPasses);expect(reached).toBe(true);expect(reads).toBeGreaterThan(0);expect(reads).toBeLessThanOrEqual(frontiers.frontier);
});
test("WAV actual chunk identity census cancels after completed ordinal placement",async()=>{
 const input=await frontierDatabase("census"),abort=new AbortController();let preceding=0,placing=false,placed=0,census=false,reads=0,reached=false;for(const row of input.database.tables.find(t=>t.name===input.table)!.rows){const ordinal=row.values[2],id=row.rowid;Object.defineProperty(row.values,"2",{get:()=>{if(placing&&++placed===frontiers.entities)census=true;return ordinal;}});Object.defineProperty(row,"rowid",{get:()=>{if(census)reads++;return id;}});}
 await expect(wavSnapshotFromSqliteDatabase(input.database,{signal:abort.signal,onProgress:event=>{if(event.phase!=="reconstructSnapshot"||event.total!==frontiers.entities)return;if(event.completed===frontiers.entities-1&&++preceding===frontiers.precedingPasses)placing=true;if(census&&reads>0){reached=true;abort.abort();}}})).rejects.toHaveProperty("kind",frontiers.refusalKind);expect(placed).toBe(frontiers.entities);expect(reached).toBe(true);expect(reads).toBeGreaterThan(0);expect(reads).toBeLessThanOrEqual(frontiers.frontier);
});
test("WAV reconstruction captures one operation before caller option mutation",async()=>{
 const input=fixtureSnapshot(),database=await wavSnapshotToSqliteDatabase(input),valid=new AbortController(),stale=new AbortController();stale.abort();let captured=false;const options={maxRows:frontiers.capturedRows,signal:valid.signal,onProgress:()=>{if(!captured){captured=true;options.maxRows=0;options.signal=stale.signal;options.onProgress=()=>{throw new Error("Mutated options escaped WAV operation capture");};}}};expect(await wavSnapshotFromSqliteDatabase(database,options)).toEqual(input);expect(captured).toBe(true);
});
test("WAV semantic and physical routes retain one explicit cumulative byte owner",async()=>{
 const input=fixtureSnapshot(),measured=new SqliteOperation();const file=await exportSqliteDatabase(await wavSnapshotToSqliteDatabase(input,measured),measured);expect(await wavSnapshotFromSqliteDatabase(await importSqliteDatabase(file,measured),measured)).toEqual(input);expect(measured.ownedBytes).toBeGreaterThan(0);
 const exact=new SqliteOperation({maxAllocationBytes:measured.ownedBytes+frontiers.priorDebt});exact.allocateBytes(frontiers.priorDebt);const encoded=await exportSqliteDatabase(await wavSnapshotToSqliteDatabase(input,exact),exact);expect(await wavSnapshotFromSqliteDatabase(await importSqliteDatabase(encoded,exact),exact)).toEqual(input);expect(exact.remainingBytes()).toBe(0);
 const short=new SqliteOperation({maxAllocationBytes:measured.ownedBytes+frontiers.priorDebt-1});short.allocateBytes(frontiers.priorDebt);await expect((async()=>{const bytes=await exportSqliteDatabase(await wavSnapshotToSqliteDatabase(input,short),short);return wavSnapshotFromSqliteDatabase(await importSqliteDatabase(bytes,short),short);})()).rejects.toHaveProperty("kind",frontiers.allocationRefusalKind);expect(short.ownedBytes).toBeGreaterThanOrEqual(frontiers.priorDebt);
});

import paddingPresence from "../🧫️fixtures/🎚️presence/🔣️.json";

test("WAV complete padding fields retain canonical zero defaults and physical owner identity",async()=>{
 expect(paddingPresence["version"]).toEqual(1);expect(paddingPresence["canonicalPads"]).toEqual({"format":0,"samples":0,"auxiliary":0});
 for(const role of paddingPresence.cases){
  const {fmtPadByte,dataPadByte,...base}=fixtureSnapshot();const input={...base,otherChunks:base.otherChunks.map(({padByte,...chunk})=>({...chunk}))};
  const snapshot=parseWavSnapshot({...input,...(role.present?{fmtPadByte:role.value,dataPadByte:role.value}:{}),otherChunks:input.otherChunks.map(chunk=>({...chunk,...(role.present?{padByte:role.value}:{})}))});
  expect(Object.hasOwn(snapshot,"fmtPadByte")).toBe(true);expect(Object.hasOwn(snapshot,"dataPadByte")).toBe(true);for(const chunk of snapshot.otherChunks)expect(Object.hasOwn(chunk,"padByte")).toBe(true);expect([snapshot.fmtPadByte,snapshot.dataPadByte,...snapshot.otherChunks.map(chunk=>chunk.padByte)]).toEqual([paddingPresence.canonicalPads.format,paddingPresence.canonicalPads.samples,...snapshot.otherChunks.map(()=>paddingPresence.canonicalPads.auxiliary)]);
  const database=await wavSnapshotToSqliteDatabase(snapshot),oracle=Database.deserialize(await exportSqliteDatabase(database));try{
   expect(oracle.query("SELECT pad_byte FROM wav_format").all()).toEqual([{pad_byte:role.value}]);expect(oracle.query("SELECT pad_byte FROM wav_data").all()).toEqual([{pad_byte:role.value}]);expect(oracle.query("SELECT pad_byte FROM wav_other_chunk ORDER BY ordinal").all()).toEqual(snapshot.otherChunks.map(()=>({pad_byte:role.value})));
   expect(await wavSnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()))).toEqual(snapshot);
  }finally{oracle.close();}
 }
});

/** 🎚️ Independently measures complete sample variants and optional extension relationships. */
test("WAV complete semantic cells admit exact grants and refuse one-short limits",async()=>{
 const plan=await Bun.file(new URL("../🧫️fixtures/📏️cells/🔣️.json",import.meta.url)).json();expect(plan["tableWidths"]).toEqual([["wav_chunk_order",8],["wav_data",3],["wav_document",2],["wav_float32_sample",6],["wav_format",8],["wav_format_extension",1],["wav_format_extension_byte",4],["wav_other_chunk",5],["wav_other_chunk_byte",4],["wav_pcm16_sample",4],["wav_pcm8_sample",4],["wav_raw_data_byte",4]]);expect(plan["sampleKinds"]).toEqual(["pcm16","pcm8","float32","raw"]);expect(plan["extensionStates"]).toEqual(["absent","empty","present"]);expect(plan["formatReferences"]).toEqual(["format","samples"]);expect(plan["fullFixtureRows"]).toEqual(22);
 const variants:WavSnapshot["data"][]=[{kind:"pcm16",value:[-32768,-1,0,1,32767]},{kind:"pcm8",value:[0,255,127]},{kind:"float32",value:floats.ieee754Binary32Bits.map(bits=>({bits}))},{kind:"raw",value:[0,255,127]}];
 expect(variants.map(v=>v.kind)).toEqual(plan.sampleKinds);
 for(const extension of plan.extensionStates as ("absent"|"empty"|"present")[])for(const data of variants){
  const original=fixtureSnapshot(),{ext,...fmt}=original.fmt,input:WavSnapshot={...original,schema:"WAV 世界\0",fmt:{...fmt,...(extension==="absent"?{}:{ext:extension==="empty"?[]:ext!})},data,chunkOrder:[{kind:"format"},{kind:"samples"},...indices.unsigned64ChunkIndices.map(value=>({kind:"other" as const,value:BigInt(value)}))]};
  const database=await wavSnapshotToSqliteDatabase(input),db=Database.deserialize(await exportSqliteDatabase(database),{safeIntegers:true});let rows:number,cells:number;
  try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
   const tables=(db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all()as{name:string}[]).map(({name})=>{const columns=db.query('PRAGMA table_info('+name+')').all()as{name:string}[],expression=columns.map(({name})=>{const c='"'+name.replaceAll('"','""')+'"';return "CASE typeof("+c+") WHEN 'integer' THEN 8 WHEN 'real' THEN 8 WHEN 'text' THEN length(CAST("+c+" AS BLOB)) WHEN 'blob' THEN length("+c+") WHEN 'null' THEN 0 ELSE NULL END"}).join('+'),r=db.query('SELECT COUNT(*) AS rows,COALESCE(SUM('+expression+'),0) AS bytes FROM '+name).get()as{rows:bigint;bytes:bigint}|null;if(!r)throw Error("Independent cell aggregate absent");return{table:name,columns:columns.length,rows:Number(r.rows),semanticBytes:Number(r.bytes)}});
   expect(tables.map(t=>[t.table,t.columns])).toEqual(plan.tableWidths);rows=tables.reduce((n,t)=>n+t.rows,0);cells=tables.reduce((n,t)=>n+t.semanticBytes,0);
  }finally{db.close()}
  expect(await wavSnapshotToSqliteDatabase(input,{maxRows:rows,maxValueBytes:cells})).toEqual(database);
  for(const limit of[{maxRows:rows-1,maxValueBytes:cells},{maxRows:rows,maxValueBytes:cells-1}]){await expect(wavSnapshotToSqliteDatabase(input,limit)).rejects.toThrow();await expect(wavSnapshotFromSqliteDatabase(database,limit)).rejects.toThrow();}
 }
});
