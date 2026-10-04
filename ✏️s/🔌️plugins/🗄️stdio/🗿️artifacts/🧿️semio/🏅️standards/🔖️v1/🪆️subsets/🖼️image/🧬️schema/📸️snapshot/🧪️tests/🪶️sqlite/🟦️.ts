/** 🖼️ Native image frame bytes, nullable ICC and duplicate metadata laws. */
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import f from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import type {SemioImageSnapshot} from "../../🟦️.ts";
import {SEMIO_IMAGE_SQLITE_SCHEMA,semioImageSnapshotToSqliteDatabase,semioImageSnapshotFromSqliteDatabase} from "../../🪶️sqlite/🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework";
function bytes(hex:string):number[]{return Array.from(Uint8Array.fromHex(hex));}
const input:SemioImageSnapshot={schema:f.schema,width:f.width,height:f.height,colorspace:"indexed",bitDepth:f.bitDepth,frames:f.frameHex.map((hex,index)=>({delayMs:f.delays[index]!,rgba8:bytes(hex)})),icc:bytes(f.iccHex),metadata:f.metadata.map(([key,value])=>({key:key!,value:value!}))};
test("Semio image independent pixels/metadata queries and edited bytes",async()=>{expect(SEMIO_IMAGE_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql",import.meta.url)).text());const db=Database.deserialize(await exportSqliteDatabase(await semioImageSnapshotToSqliteDatabase(input)));try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(db.query(f.query).all()).toHaveLength(2);expect(await semioImageSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(input);db.query("UPDATE semio_image_document SET icc=NULL").run();db.query("UPDATE semio_image_frame SET delay_ms=17 WHERE ordinal=0").run();const edited=structuredClone(input);edited.icc=null;edited.frames[0]!.delayMs=17;expect(await semioImageSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(edited);}finally{db.close();}});
test("Semio image storage widths, pixel dimensions, empty ICC, ordering and cancellation",async()=>{const database=await semioImageSnapshotToSqliteDatabase(input);for(const mutate of [(d:any)=>d.tables[0].rows[0].values[2]=-1n,(d:any)=>d.tables[0].rows[0].values[6]="icc",(d:any)=>d.tables[1].rows[0].values[1]=2n,(d:any)=>d.tables[2].rows[0].values[3]=4n,(d:any)=>d.tables[3].rows[0].values[2]=8n]){const d=structuredClone(database);mutate(d);await expect(semioImageSnapshotFromSqliteDatabase(d)).rejects.toThrow();}const partial={...input,frames:[{delayMs:0,rgba8:[]}]};expect(await semioImageSnapshotFromSqliteDatabase(await semioImageSnapshotToSqliteDatabase(partial))).toEqual(partial);await expect(semioImageSnapshotToSqliteDatabase(input,{maxRows:1})).rejects.toThrow();await expect(semioImageSnapshotFromSqliteDatabase(database,{maxValueBytes:1})).rejects.toThrow();const c=new AbortController();c.abort();await expect(semioImageSnapshotFromSqliteDatabase(database,{signal:c.signal})).rejects.toThrow();const empty={...input,icc:[],frames:[],metadata:[]};expect(await semioImageSnapshotFromSqliteDatabase(await semioImageSnapshotToSqliteDatabase(empty))).toEqual(empty);});

test("Semio image controlled native corpus independently preserves octets and collection cardinality",async()=>{const {default:Ajv2020}=await import("ajv/dist/2020");const schema=await Bun.file(new URL("../../🛬️native/🧬️schema/🔣️.json",import.meta.url)).json();expect(new Ajv2020({strict:true}).compile(schema)(f.controlledAdmission)).toBe(true);const d=new Database(":memory:");try{expect(d.query("WITH RECURSIVE items(n) AS (SELECT 1 UNION ALL SELECT n+1 FROM items WHERE n<?) SELECT count(*) AS total,sum(n<=?) AS canceled FROM items").get(f.controlledAdmission.collectionItems,f.controlledAdmission.cancelAfter)).toEqual({total:1024,canceled:256});expect(d.query("SELECT length(?) AS bytes,hex(substr(?,1,1)) AS first").get(new Uint8Array(f.controlledAdmission.largeBytes).fill(255),new Uint8Array([255]))).toEqual({bytes:100000,first:"FF"});for(const hex of f.frameHex){expect(Buffer.from(hex,"hex").toString("hex")).toBe(hex);}}finally{d.close();}});


test("Semio image complete and intermediate RGBA8 owners have independently editable ordered channel entities",async()=>{
 const neutral=await Bun.file(new URL("../../🧫️fixtures/🪶️sqlite/🎨️samples/🔣️.json",import.meta.url)).json();
 const schema=await Bun.file(new URL("../../🧫️fixtures/🪶️sqlite/🎨️samples/🧬️schema/🔣️.json",import.meta.url)).json();
 const {default:Ajv}=await import("ajv");const valid=new Ajv({strict:true}).compile(schema);expect(valid(neutral)).toBe(true);expect(valid({...neutral,extra:true})).toBe(false);
 for(const item of neutral.cases){
  const input=item.snapshot;const physical=Database.deserialize(await exportSqliteDatabase(await semioImageSnapshotToSqliteDatabase(input)));
  try{
   expect(physical.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(physical.query("PRAGMA foreign_key_check").all()).toEqual([]);
   expect(physical.query(neutral.query).all()).toEqual(input.frames[0].rgba8.map((sample:number,ordinal:number)=>({ordinal,channel:ordinal%4,sample})));
   expect(await semioImageSnapshotFromSqliteDatabase(await importSqliteDatabase(physical.serialize()))).toEqual(input);
   if(input.frames[0].rgba8.length>neutral.edit.ordinal){
    physical.query("UPDATE semio_image_sample SET sample=? WHERE ordinal=?").run(neutral.edit.sample,neutral.edit.ordinal);
    const expected=structuredClone(input);expected.frames[0].rgba8[neutral.edit.ordinal]=neutral.edit.sample;
    expect(await semioImageSnapshotFromSqliteDatabase(await importSqliteDatabase(physical.serialize()))).toEqual(expected);
   }
  }finally{physical.close();}
 }
 console.log("[DEBUG] Semio image independently queried and edited ordered channel samples across all four complete and intermediate owners");

});
