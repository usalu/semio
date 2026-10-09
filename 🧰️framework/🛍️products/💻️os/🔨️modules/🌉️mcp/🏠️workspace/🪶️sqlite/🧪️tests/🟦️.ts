/** 🪶️ Compares handcrafted Probe node semantics with the independent SQLite engine. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {readFileSync} from "node:fs";

test("Probe semantic edge policies preserve exact numeric TEXT and shuffled source ordinals",()=>{
 expect(new Ajv2020({strict:true}).compile(schema)(fixture)).toBe(true);
 const database=new Database(":memory:");
 try{
  database.exec(readFileSync(new URL("../🗄️.sql",import.meta.url),"utf8"));const insert=database.query("INSERT INTO probe_node VALUES(?,?,?,?,?,?,?,?)");const rows=fixture.cases[0].rows;
  for(const identity of fixture.semanticEdges.physicalOrder)insert.run(...rows.find(row=>row[0]===identity)!);
  expect(database.query("SELECT * FROM probe_node ORDER BY id").values()).toEqual(rows);
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
import nativeSchema from "../🫴️native/🧬️schema/🔣️.json";
import {exportSqliteDatabase,importSqliteDatabase,parseSqliteDatabaseSchema,type SqliteValue} from "../../../../../../../🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

test("Probe original native policy is closed and preserves independent semantic witnesses",()=>{
 const validate=new Ajv2020({strict:true}).compile(nativeSchema);
 expect(validate(nativeFixture)).toBe(true);
 for(const key of Object.keys(nativeFixture)){const missing={...nativeFixture} as Record<string,unknown>;delete missing[key];expect(validate(missing)).toBe(false);}
 expect(validate({...nativeFixture,bodyGrant:{...nativeFixture.bodyGrant,maximumDepth:129}})).toBe(false);
 expect(nativeFixture.closeGrant).not.toEqual(nativeFixture.bodyGrant);
 expect(nativeFixture.localContinuation).toEqual({identity:"originalRecipient",rejectReplacement:true,conserveCumulative:true,retirementTurnsPerDrive:4096,maximumPendingPerDirection:1});
 expect(nativeFixture.refusals.map(row=>row.axis)).toEqual(["work","copy","capacity","release","depth"]);
 expect(Buffer.byteLength(JSON.parse(nativeFixture.cancellation.source),"utf8")).toBe(768);
 expect(JSON.parse(nativeFixture.duplicate.source)).toEqual({"long-λ🙂":2});
 for(const sample of fixture.cases)expect(JSON.stringify(JSON.parse(sample.wire))).toBeString();
 console.error("[DEBUG] Closed Probe original Native grant, cancellation text and duplicate witness validated with independent Ajv, Buffer and JSON oracles; no Native execution credit");
});

test("Probe node schema preserves explicit object ordinals and exact numeric words",async()=>{
 const validate=new Ajv2020({strict:true}).compile(schema);
 expect(validate(fixture)).toBe(true);
 for(const key of Object.keys(fixture)){const missing={...fixture} as Record<string,unknown>;delete missing[key];expect(validate(missing)).toBe(false);}
 const ddl=await Bun.file(new URL("../🗄️.sql",import.meta.url)).text();
 for(const sample of fixture.cases){
  const db=new Database(":memory:");
  try{
   db.exec("PRAGMA foreign_keys=ON");db.exec(ddl);
   const insert=db.query("INSERT INTO probe_node VALUES(?,?,?,?,?,?,?,?)");
   for(const row of sample.rows)insert.run(...row);
   expect(db.query("SELECT * FROM probe_node ORDER BY id").values()).toEqual(sample.rows);
   expect(db.query("SELECT position,member_name FROM probe_node WHERE parent_id=1 ORDER BY position").values()).toEqual(sample.rows.filter(row=>row[1]===1).map(row=>[row[2],row[3]]));
   for(const row of sample.rows)if(row[4]==="number"){expect(db.query("SELECT typeof(number_value),number_value FROM probe_node WHERE id=?").values(row[0])).toEqual([["text",row[6]]]);if(typeof row[6]==="string"&&/^-?\d+$/.test(row[6]))expect(BigInt(row[6]).toString()).toBe(row[6]);}
   expect(db.query("PRAGMA integrity_check").values()).toEqual([["ok"]]);
   const declared=parseSqliteDatabaseSchema(ddl);
   const rows=sample.rows.map(row=>({rowid:BigInt(row[0] as number),values:row.map((cell,index)=>typeof cell==="number"&&[0,1,2,5].includes(index)?BigInt(cell):cell) as SqliteValue[]}));
   const bytes=await exportSqliteDatabase({tables:declared.tables.map(table=>({...table,rows}))});
   const independent=Database.deserialize(bytes);
   try{expect(independent.query("SELECT * FROM probe_node ORDER BY id").values()).toEqual(sample.rows);expect(independent.query("PRAGMA integrity_check").values()).toEqual([["ok"]]);}finally{independent.close();}
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
