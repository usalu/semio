/** 📑️ Compares original borrowed SQLite declarations and extents under genuine per-grant authority. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {existsSync,readFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔣️.json";
import grantSchema from "../../../../../🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json";

test("original SQLite receiving preserves plain declaration and extent oracles without trial admission",()=>{
 expect(existsSync(resolve(import.meta.dir,"../🧬️schema/🔣️.json"))).toBe(false);
 const validate=new Ajv({strict:true,allErrors:true}).compile(grantSchema);
 expect(validate(fixture.grant)).toBe(true);
 for(const axis of Object.keys(fixture.grant))expect(validate({...fixture.grant,[axis]:-1})).toBe(false);
 const reference=new Database(":memory:");
 try{
  reference.run(fixture.authoredSql);
  for(const row of fixture.cases){
   const actual=new Database(":memory:"),authored=new Database(":memory:");let accepted=false;
   try{actual.run(row.sql);authored.run("authoredSql" in row?row.authoredSql:fixture.authoredSql);accepted=JSON.stringify(actual.query("SELECT name,sql FROM sqlite_master WHERE type='table'").all())===JSON.stringify(authored.query("SELECT name,sql FROM sqlite_master WHERE type='table'").all());}catch{}finally{actual.close();authored.close();}
   expect(accepted,row.id).toBe(row.oracleAccepted);
  }
  reference.run("INSERT INTO retained_value VALUES(?,?)",...fixture.extentRow);
  const extent=reference.query("SELECT count(*) AS rows,sum(8+length(CAST(value AS BLOB))) AS bytes FROM retained_value").get() as {rows:number;bytes:number};
  expect(extent).toEqual({rows:1,bytes:8+Buffer.byteLength(fixture.extentRow[1] as string)});
  for(const row of fixture.extentCases)expect(extent.rows<=row.maximumRows&&fixture.extentRow.length<=row.maximumColumns&&extent.bytes<=row.maximumValueBytes,row.id).toBe(row.accepted);
 }finally{reference.close();}
 const owner=readFileSync(resolve(import.meta.dir,"../../../📦️packages/🦀️rust/📜️script.ts"),"utf8");
 const receiving=owner.slice(owner.indexOf("class ReceivingSchemaScript"),owner.indexOf("class OperationScript"));
 expect(receiving.includes("compile(schema)")).toBe(false);
 expect(receiving.includes("🧬️schema/🔣️.json")).toBe(false);
 expect(receiving.includes("sqlite_snapshot_receiving_borrowed_schema_uses_original_work_without_owned_tokens")).toBe(true);
 console.log("[DEBUG] Original SQLite receiving "+fixture.cases.length+" declaration/"+fixture.extentCases.length+" extent Bun SQLite oracles preserve original per-case declarations and fixed policy; Ajv validates actual Grant only; native allocator assertions remain separate");
});
