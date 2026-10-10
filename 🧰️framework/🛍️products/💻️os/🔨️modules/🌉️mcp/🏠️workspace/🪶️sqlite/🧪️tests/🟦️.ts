/** 🪶️ Compares handcrafted Probe node semantics with the independent SQLite engine. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import retirementContract from "../../../../../../../🔨️modules/🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json";
const validateGrant=new Ajv({strict:true}).addSchema(retirementContract).compile({$ref:retirementContract.$id+"#/$defs/Grant"});
import fixture from "../🧫️fixtures/🔣️.json";
import {readFileSync} from "node:fs";
import semanticFixture from "../🫴️semantic/🧫️fixtures/🔣️.json";
import guestInputFixture from "../🧫️fixtures/guest-input-slot.json";
import guestInputSchema from "../🧬️schema/guest-input-slot.json";
import coordinateFixture from "../🧫️fixtures/coordinate.json";
import coordinateSchema from "../🧬️schema/coordinate.json";

test("Guest coordinate quotes exact original allocation before copying",()=>{
 expect(new Ajv({strict:true}).compile(coordinateSchema)(coordinateFixture)).toBe(true);
 expect(validateGrant(coordinateFixture.bodyGrant)).toBe(true);expect(validateGrant(coordinateFixture.closeGrant)).toBe(true);expect(validateGrant(coordinateFixture.deniedCloseGrant)).toBe(true);
 expect(Buffer.byteLength(coordinateFixture.coordinate,"utf8")).toBe(coordinateFixture.originalBytes);
 expect(Buffer.byteLength(coordinateFixture.canceledPrefix,"utf8")).toBe(coordinateFixture.canceledCopyBytes);
 const db=new Database(":memory:");try{for(const bytes of [coordinateFixture.oneShortBytes,coordinateFixture.originalBytes])expect(db.query("SELECT length(CAST(? AS BLOB))<=? AS admitted").get(coordinateFixture.coordinate,bytes)).toEqual({admitted:bytes===coordinateFixture.originalBytes?1:0});}finally{db.close();}
 const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8"),start=source.indexOf("fn guest_sqlite_import_coordinate("),end=source.indexOf("fn guest_sqlite_import(",start),body=source.slice(start,end);
 expect(body.indexOf("maximum_capacity_bytes<extent")).toBeLessThan(body.indexOf("std::alloc::alloc"));expect(body).toContain("String::from_raw_parts(pointer,0,extent)");expect(body).not.toContain("try_reserve");expect(body).toContain("retained_capacity_bytes:if coordinate.capacity()!=0{extent}else{0}");expect(body).toContain("native.advance(part.len())?;native.checkpoint()?");expect(body).toContain("native.record_progress(progress)");
 console.error("[DEBUG] Guest coordinate strict Ajv and independent SQLite declare exact30-byte birth/29-byte refusal; same original recipient preflight precedes exact allocator, Native heap proof separate");
});

test("Guest original input-slot witness agrees with independent SQL and serde-compatible grant",()=>{
 expect(new Ajv({strict:true}).compile(guestInputSchema)(guestInputFixture)).toBe(true);
 expect(validateGrant(guestInputFixture.grant)).toBe(true);
 expect(validateGrant(guestInputFixture.closeGrant)).toBe(true);expect(validateGrant(guestInputFixture.deniedCloseGrant)).toBe(true);expect(guestInputFixture.closeAuthority).toBe("independentCallerWallet");expect(guestInputFixture.closeGrant).not.toEqual(guestInputFixture.grant);
 const database=new Database(":memory:");try{database.exec(guestInputFixture.schema);database.query("INSERT INTO original_input VALUES(?,?)").run(1,guestInputFixture.value);expect(database.query("SELECT value FROM original_input WHERE id=1").get()).toEqual({value:guestInputFixture.value});}finally{database.close();}
 const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8"),start=source.indexOf("fn guest_sqlite_import("),end=source.indexOf("fn register_guest_document_codec",start),body=source.slice(start,end);
 expect(body).toContain("database: &mut Option<semio_framework::sqlite_snapshot::SqliteDatabase>");expect(body.indexOf("native.receive_nested")).toBeLessThan(body.indexOf("database.take()"));expect(body).toContain("Ok((||{");expect(body).toContain("frame.wire=Some(");expect(body).toContain("std::str::from_utf8(&payload.bytes)");expect(body).not.toContain("payload.clone()");
 console.error("[DEBUG] Guest input-slot neutral witness matches independent SQLite and canonical Grant Ajv; source admission precedes transfer and invalid UTF8 retains original bytes");
});

test("Probe original semantic custody policy matches independent exact numeric and SQLite ordinals",()=>{
 for(const grant of [semanticFixture.bodyGrant,semanticFixture.closeGrant,semanticFixture.deniedCloseGrant])expect(validateGrant(grant)).toBe(true);
 for(const key of Object.keys(semanticFixture.bodyGrant)){const missing={...semanticFixture.bodyGrant} as Record<string,unknown>;delete missing[key];expect(validateGrant(missing)).toBe(false);}
 expect(validateGrant({...semanticFixture.bodyGrant,maximumDepth:-1})).toBe(false);
 expect(semanticFixture.refusals.map(row=>row.axis)).toEqual(["work","copy","capacity","release","depth"]);
 expect(Buffer.byteLength(semanticFixture.stringUnit.repeat(semanticFixture.stringRepetitions),"utf8")).toBe(semanticFixture.stringBytes);
 for(const word of semanticFixture.invalidNumberWords){if(word==="1e9999")expect(Number.isFinite(JSON.parse(word))).toBe(false);else expect(()=>JSON.parse(word)).toThrow();}
 const database=new Database(":memory:");try{database.exec(readFileSync(new URL("../🗄️.sql",import.meta.url),"utf8"));const insert=database.query("INSERT INTO probe_node VALUES(?,?,?,?,?,?,?,?)");for(const id of fixture.semanticEdges.physicalOrder)insert.run(...fixture.cases[0].rows.find(row=>row[0]===id)!);expect(database.query("SELECT member_name FROM probe_node WHERE parent_id=1 ORDER BY position").values()).toEqual([["z"],["a"],["empty"],["text"]]);}finally{database.close();}
 console.error("[DEBUG] Canonical Grant Ajv, JSON.parse and SQLite validate plain semantic examples, numeric rejection witnesses and explicit source ordinals");
});

test("Probe semantic edge policies preserve exact numeric TEXT and shuffled source ordinals",()=>{
 expect(validateGrant(fixture.callerGrant)).toBe(true);
 const database=new Database(":memory:");
 try{
  database.exec(readFileSync(new URL("../🗄️.sql",import.meta.url),"utf8"));const insert=database.query("INSERT INTO probe_node VALUES(?,?,?,?,?,?,?,?)");const rows=fixture.cases[0].rows;
  for(const identity of fixture.semanticEdges.physicalOrder)insert.run(...rows.find(row=>row[0]===identity)!);
  expect<unknown>(database.query("SELECT * FROM probe_node ORDER BY id").values()).toEqual(rows);
  expect(database.query("SELECT member_name FROM probe_node WHERE parent_id=1 ORDER BY position").values()).toEqual([["z"],["a"],["empty"],["text"]]);
  for(const word of fixture.semanticEdges.numericWords){const bytes=database.query("SELECT length(CAST(? AS BLOB)) AS bytes").get(word.text) as {bytes:number};expect(bytes.bytes).toBe(word.bytes);expect(database.query("SELECT length(CAST(? AS BLOB))<=? AS admitted").get(word.text,word.bytes-1)).toEqual({admitted:0});expect(database.query("SELECT length(CAST(? AS BLOB))<=? AS admitted").get(word.text,word.bytes)).toEqual({admitted:1});if(word.exactKind!=="float")expect(BigInt(word.text).toString()).toBe(word.text);else if(word.text==="-0.0")expect(Object.is(JSON.parse(word.text),-0)).toBe(true);}
  const names=fixture.semanticEdges.longCommonNames;expect(database.query("SELECT CAST(? AS BLOB)=CAST(? AS BLOB) AS duplicate").get(names[0],names[1])).toEqual({duplicate:0});expect(database.query("SELECT CAST(? AS BLOB)=CAST(? AS BLOB) AS duplicate").get(names[0],names[0])).toEqual({duplicate:1});
 }finally{database.close();}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8"),start=source.indexOf("fn number_value"),end=source.indexOf("impl store::ArtifactSqliteSnapshot",start),number=source.slice(start,end);
 expect(number).toContain("control.check_value_bytes(text.len())?");expect(number).not.toContain("semio_framework_pack_json::parse(");
 expect(source).toContain("to_sqlite_database_receiving");expect(source).toContain("from_sqlite_database_receiving");
 console.error("[DEBUG] Probe semantic neutral policies match independent SQLite exact numeric bytes, shuffled source ordinals and long UTF8 name identity");
});
import nativeFixture from "../🫴️native/🧫️fixtures/🔣️.json";
import {exportSqliteDatabase,importSqliteDatabase,parseSqliteDatabaseSchema,type SqliteValue} from "../../../../../../../🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

test("Probe native examples preserve canonical grants and independent semantic witnesses",()=>{
 for(const grant of [nativeFixture.bodyGrant,nativeFixture.closeGrant,nativeFixture.deniedCloseGrant])expect(validateGrant(grant)).toBe(true);
 for(const key of Object.keys(nativeFixture.bodyGrant)){const missing={...nativeFixture.bodyGrant} as Record<string,unknown>;delete missing[key];expect(validateGrant(missing)).toBe(false);}
 expect(validateGrant({...nativeFixture.bodyGrant,maximumDepth:-1})).toBe(false);
 expect(nativeFixture.closeGrant).not.toEqual(nativeFixture.bodyGrant);
 expect(nativeFixture.localContinuation).toEqual({identity:"originalRecipient",rejectReplacement:true,conserveCumulative:true,retirementTurnsPerDrive:4096,maximumPendingPerDirection:1});
 expect(nativeFixture.refusals.map(row=>row.axis)).toEqual(["work","copy","capacity","release","depth"]);
 expect(Buffer.byteLength(JSON.parse(nativeFixture.cancellation.source),"utf8")).toBe(768);
 expect(JSON.parse(nativeFixture.duplicate.source)).toEqual({"long-λ🙂":2});
 for(const sample of fixture.cases)expect(JSON.stringify(JSON.parse(sample.wire))).toBeString();
 console.error("[DEBUG] Canonical Grant Ajv, Buffer and JSON validate plain native examples, cancellation text and duplicate witness; no native execution credit");
});

test("Probe node schema preserves explicit object ordinals and exact numeric words",async()=>{
 expect(validateGrant(fixture.callerGrant)).toBe(true);
 const ddl=await Bun.file(new URL("../🗄️.sql",import.meta.url)).text();
 for(const sample of fixture.cases){
  const db=new Database(":memory:");
  try{
   db.exec("PRAGMA foreign_keys=ON");db.exec(ddl);
   const insert=db.query("INSERT INTO probe_node VALUES(?,?,?,?,?,?,?,?)");
   for(const row of sample.rows)insert.run(...row);
   expect<unknown>(db.query("SELECT * FROM probe_node ORDER BY id").values()).toEqual(sample.rows);
   expect<unknown>(db.query("SELECT position,member_name FROM probe_node WHERE parent_id=1 ORDER BY position").values()).toEqual(sample.rows.filter(row=>row[1]===1).map(row=>[row[2],row[3]]));
   for(const row of sample.rows)if(row[4]==="number"){expect<unknown>(db.query("SELECT typeof(number_value),number_value FROM probe_node WHERE id=?").values(row[0])).toEqual([["text",row[6]]]);if(typeof row[6]==="string"&&/^-?\d+$/.test(row[6]))expect(BigInt(row[6]).toString()).toBe(row[6]);}
   expect(db.query("PRAGMA integrity_check").values()).toEqual([["ok"]]);
   const declared=parseSqliteDatabaseSchema(ddl);
   const rows=sample.rows.map(row=>({rowid:BigInt(row[0] as number),values:row.map((cell,index)=>typeof cell==="number"&&[0,1,2,5].includes(index)?BigInt(cell):cell) as SqliteValue[]}));
   const bytes=await exportSqliteDatabase({tables:declared.tables.map(table=>({...table,rows}))});
   const independent=Database.deserialize(bytes);
   try{expect<unknown>(independent.query("SELECT * FROM probe_node ORDER BY id").values()).toEqual(sample.rows);expect(independent.query("PRAGMA integrity_check").values()).toEqual([["ok"]]);}finally{independent.close();}
   const restored=await importSqliteDatabase(bytes);
   expect(restored.tables.find(table=>table.name==="probe_node")?.rows).toEqual(rows);
  }finally{db.close();}
 }
 for(const negative of fixture.schemaRefusals){
  const db=new Database(":memory:");
  try{db.exec(ddl);db.query("INSERT INTO probe_node VALUES(?,?,?,?,?,?,?,?)").run(...fixture.cases[0].rows[0]);expect(()=>db.query("INSERT INTO probe_node VALUES(?,?,?,?,?,?,?,?)").run(...negative.row)).toThrow();}finally{db.close();}
 }
 console.error("[DEBUG] Probe neutral node schema compared with independent SQLite; Native owner execution is qualified separately");
});
