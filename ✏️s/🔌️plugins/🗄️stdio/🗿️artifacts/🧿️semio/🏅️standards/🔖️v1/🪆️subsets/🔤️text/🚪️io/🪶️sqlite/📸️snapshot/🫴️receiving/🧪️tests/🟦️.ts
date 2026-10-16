/** 🔤️ Language-neutral complete text receiving cases have independent SQLite field witnesses. */
import{Database}from"bun:sqlite";
import{expect,test}from"bun:test";
import Ajv from"ajv";
import contract from"./🧫️fixtures/🔣️.json";
import shape from"../🧬️schema/🔣️.json";
import { retainedCloneWireSchema as canonicalGrant } from "@semio-tech/framework/value/retained-clone/wire/schema";
import{semioTextSnapshotToSqliteDatabase as project,semioTextSnapshotFromSqliteDatabase as reconstruct,SEMIO_TEXT_SQLITE_SCHEMA}from"../../🟦️.ts";
import type{SemioTextSnapshot}from"../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import{exportSqliteDatabase,importSqliteDatabase}from"@semio-tech/framework";
const utf8=(value:string)=>Buffer.from(value,"hex").toString("utf8");
test("Semio Text neutral complete contract and canonical original receiving hooks",async()=>{
 const validate=new Ajv({strict:true}).compile(shape);expect(validate(contract),JSON.stringify(validate.errors)).toBe(true);
 expect(new Ajv({strict:true}).compile(canonicalGrant)(contract.originalGrant)).toBe(true);
 expect(contract.cases.map(item=>item.id)).toEqual(["emptyDocument","emptyRun","complete"]);expect(contract.signedOffsets).toEqual([0,-1,-1000]);expect(contract.edits).toHaveLength(14);
 const owner=await Bun.file(new URL("../../🦀️.rs",import.meta.url)).text();
 for(const method of["to_sqlite_database_receiving","from_sqlite_database_receiving","validate_sqlite_snapshot_subset_decoding","validate_sqlite_snapshot_subset_encoding"])expect(owner).toContain("fn "+method+"(");
 const rust=await Bun.file(new URL("./🦀️.rs",import.meta.url)).text();expect(rust).toContain("semio_text_original_receiving_");
});
test("Semio Text every original field, empty group, NUL UTF8 and signed relational identity",async()=>{
 expect(SEMIO_TEXT_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🗄️.sql",import.meta.url)).text());
 for(const item of contract.cases){const input=item.snapshot as SemioTextSnapshot,original=await project(input);for(const offset of contract.signedOffsets){const db=Database.deserialize(await exportSqliteDatabase(original));try{
  db.exec("PRAGMA foreign_keys=OFF");db.query("UPDATE semio_text_run SET id=id+?").run(offset);db.query("UPDATE semio_text_mark SET id=id+?,run_id=run_id+?").run(offset,offset);db.exec("PRAGMA foreign_keys=ON");
  expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect((db.query("SELECT hex(CAST(schema AS BLOB)) value FROM semio_text_document WHERE id=1").get()as{value:string}).value).toBe(Buffer.from(input.schema).toString("hex").toUpperCase());
  const runs=db.query("SELECT id,ordinal,hex(CAST(language AS BLOB)) language,hex(CAST(content AS BLOB)) content FROM semio_text_run ORDER BY ordinal").all()as{id:number,ordinal:number,language:string,content:string}[];
  expect(runs.map(row=>({language:utf8(row.language),content:utf8(row.content),marks:(db.query("SELECT kind,hex(CAST(href AS BLOB)) href FROM semio_text_mark WHERE run_id=? ORDER BY ordinal").all(row.id)as{kind:string,href:string}[]).map(mark=>({kind:mark.kind,href:utf8(mark.href)}))}))).toEqual(input.runs);
  expect(1+runs.length+Number((db.query("SELECT count(*) count FROM semio_text_mark").get()as{count:number}).count)).toBe(item.rows);
  expect(await reconstruct(await importSqliteDatabase(db.serialize()))).toEqual(input);
 }finally{db.close();}}}
 console.log("[DEBUG] Semio Text all complete fields/3tables, empty groups, all4tags, nonlink hrefs, NUL UTF8 and signed IDs independently witnessed");
});
test("Semio Text independent malformed relationships and changed complete fields",async()=>{
 const input=contract.cases[2]!.snapshot as SemioTextSnapshot,original=await project(input);
 for(const edit of contract.edits){const db=Database.deserialize(await exportSqliteDatabase(original));try{db.exec("PRAGMA foreign_keys=OFF;PRAGMA ignore_check_constraints=ON");db.exec(edit.sql);const database=await importSqliteDatabase(db.serialize());if(edit.reconstruct==="invalid")await expect(reconstruct(database)).rejects.toThrow();else expect(await reconstruct(database)).not.toEqual(input);}finally{db.close();}}
 console.log("[DEBUG] Semio Text fourteen independent SQLite malformed/changed field edits preserve complete semantic distinction");
});

