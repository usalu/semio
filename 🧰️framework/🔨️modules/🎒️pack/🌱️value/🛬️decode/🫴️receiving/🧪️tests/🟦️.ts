import AjvDraft07 from "ajv";
import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv from "ajv/dist/2020.js";
import {Database} from "bun:sqlite";
const read=(path:string)=>readFileSync(resolve(import.meta.dir,path),"utf8");
test("original Record receiving schema requires successful one-item continuation",()=>{const fixture=JSON.parse(read("🧫️fixtures/🔣️.json"));expect(new AjvDraft07().compile(JSON.parse(read("../🧬️schema/🔣️.json")))(fixture)).toBe(true);const db=new Database(":memory:");try{expect(db.query("SELECT json_extract(?,'$.turn.items') AS items,json_extract(?,'$.text') AS text").get(JSON.stringify(fixture),JSON.stringify(fixture))).toEqual({items:1,text:fixture.text});}finally{db.close()}const source=read("../🦀️.rs");expect(source).toContain("cursor.source_step_demands()");expect(source).toContain("source.as_ptr()");expect(source).not.toContain("decode_record_body_exact_controlled");expect(source).not.toContain("NativeDecodeControl::new");});

test("original columnar receiving fixture independently preserves sparse row values",()=>{const fixture=JSON.parse(read("🧫️fixtures/📊️table.json"));expect(new Ajv().compile(JSON.parse(read("../🧬️schema/📊️table.json")))(fixture)).toBe(true);const db=new Database(":memory:");try{expect(db.query("SELECT json_array_length(?,'$.rows') AS rows,json_extract(?,'$.rows[0][0]') AS text,json_extract(?,'$.rows[1][1]') AS absent").get(JSON.stringify(fixture),JSON.stringify(fixture),JSON.stringify(fixture))).toEqual({rows:10,text:"漢",absent:null});}finally{db.close()}});

test("complete Pack record source preserves explicit framing and every denied receiving currency",()=>{
 const fixture=JSON.parse(read("🧫️fixtures/🔣️.json"));expect(new AjvDraft07().compile(JSON.parse(read("../🧬️schema/🔣️.json")))(fixture)).toBe(true);const plan=fixture.documentReceiving;expect(plan.sourceFormat).toBe("Document");expect(plan.copySource).toBe(false);expect(plan.guessBody).toBe(false);const db=new Database(":memory:");try{db.run("CREATE TABLE credit(axis TEXT,value INTEGER,need INTEGER)");for(const axis of plan.deniedAxes)db.query("INSERT INTO credit VALUES(?,?,?)").run(axis,0,1);expect(db.query("SELECT axis FROM credit WHERE value<need ORDER BY rowid").all()).toEqual(plan.deniedAxes.map((axis:string)=>({axis})));}finally{db.close();}const source=read("../🦀️.rs"),driver=read("../../📦️document/🦀️.rs");expect(source).toContain("pub fn admit_document(");expect(driver).toContain("fn normal_demands(");expect(driver).toContain("original_step_demands()");expect(driver).toContain("verified_payload_crc32c(segment)");expect(driver).toContain("entry.blake3");expect(source).not.toContain("decode_document_controlled");
});

test("generic record retirement retains the authored body and cancellation matrix",()=>{const f=JSON.parse(read("🧫️fixtures/🔣️.json"));expect(new AjvDraft07().compile(JSON.parse(read("../🧬️schema/🔣️.json")))(f)).toBe(true);const db=new Database(":memory:");try{const rows=db.query("SELECT CAST(b.value AS INTEGER) body, CAST(c.value AS INTEGER) cut FROM json_each(?1,'$.correlatedRetirement.bodies') b CROSS JOIN json_each(?1,'$.correlatedRetirement.cancelCuts') c ORDER BY b.key,c.key").all(JSON.stringify(f));expect(rows).toEqual(f.correlatedRetirement.bodies.flatMap((body:number)=>f.correlatedRetirement.cancelCuts.map((cut:number)=>({body,cut}))));}finally{db.close();}});

test("document driver range contract preserves indexed pages and explicit nonzero bounds",()=>{
 const fixture=JSON.parse(read("🧫️fixtures/🔣️.json"));expect(new AjvDraft07().compile(JSON.parse(read("../🧬️schema/🔣️.json")))(fixture)).toBe(true);
 const plan=fixture.documentReceiving.sourceRange,body=Buffer.from(fixture.text,"utf8"),original=Buffer.concat([Buffer.from(plan.prefix),body,Buffer.from(plan.suffix)]),range=[plan.prefix.length,plan.prefix.length+body.length],pages:Buffer[]=[];
 for(let at=0;at<original.length;at+=plan.pageBytes)pages.push(original.subarray(at,at+plan.pageBytes));
 const indexed=Buffer.from(Array.from({length:range[1]-range[0]},(_,i)=>pages[Math.floor((range[0]+i)/plan.pageBytes)][(range[0]+i)%plan.pageBytes]));
 expect(indexed.equals(original.subarray(...range))).toBe(true);expect(indexed.toString()).toBe(fixture.text);
 const db=new Database(":memory:");try{expect(db.query("select substr(?1,?2+1,?3-?2) as selected").get(original,...range)).toEqual({selected:indexed});}finally{db.close();}
 const driver=read("../../🎮️driver/🦀️.rs");expect(driver).toContain("fn one<S:RetainedWireByteSource+?Sized>(&mut self,input:&S,range:(usize,usize),maximum:usize)");
});
