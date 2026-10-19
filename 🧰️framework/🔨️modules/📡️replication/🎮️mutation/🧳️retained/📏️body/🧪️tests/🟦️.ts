import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
test("protocol logical body survives expanded enclosing physical headers",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{for(const row of fixture.rows){const oracle=db.query("SELECT ? AS child, CASE WHEN ?>=?+? THEN ? ELSE 0 END AS selected").get(row.body,row.physical,row.overhead,row.body,row.body) as {child:number;selected:number};expect(oracle.child).toBe(row.child);expect(oracle.selected).toBe(row.selected);expect(Buffer.alloc(row.physical,7).subarray(0,row.selected).byteLength).toBe(row.selected);expect(row.selected).toBeLessThan(row.physical);}}finally{db.close();}
});
test("real protocol clone consumers forward the same quoted logical body",()=>{
 for(const row of fixture.sourceGuards){const source=readFileSync(new URL("../../"+row.file,import.meta.url),"utf8").replace(/\s+/gu,"");expect(source,row.file).toContain(row.contains);}
});

test("bounded byte range outputs agree with SQLite selection and Buffer slices under every original body",()=>{
 const row=fixture.byteRange,db=new Database(":memory:");try{db.run("CREATE TABLE source(position INTEGER PRIMARY KEY,value INTEGER)");row.source.forEach((value,position)=>db.run("INSERT INTO source VALUES(?,?)",[position,value]));const selected=(db.query("SELECT value FROM source WHERE position>=? AND position<? ORDER BY position").all(row.start,row.start+row.length) as {value:number}[]).map(row=>row.value);expect(selected).toEqual(row.expected);expect([...Buffer.from(row.source).subarray(row.start,row.start+row.length)]).toEqual(row.expected);for(const policy of fixture.rows){const output:number[]=[];for(let offset=0;offset<row.length;offset+=policy.body)output.push(...row.source.slice(row.start+offset,row.start+Math.min(row.length,offset+policy.body)));expect(output).toEqual(selected);}}finally{db.close();}
});

test("central native fixture implementations preserve each original quoted body and receiving route",()=>{
 const read=(path:string)=>readFileSync(new URL("../../"+path,import.meta.url),"utf8");
 const preparation=read("🧪️tests/🦀️.rs"),apply=read("🔺️diff/🧪️tests/🦀️.rs"),sequence=read("🔺️diff/🪢️sequence/🧪️tests/🦀️.rs"),receiving=read("🔺️diff/🫴️receiving/🧪️tests/🦀️.rs");
 const headers=[preparation,apply,sequence,receiving].flatMap(source=>[...source.matchAll(/fn (?:decide|inverse|preflight)\(&mut self[^]*?->/g)].map(match=>match[0]));
 expect(headers).toHaveLength(6);
 for(const header of headers)expect(/body:usize,(?:grant|g):/.test(header)).toBe(true);
 const compact=(source:string)=>source.replace(/\s+/gu,"");
 expect(compact(preparation).includes("decide_retained_mutation(o,m.borrow(),p.borrow(),4096,g)")).toBe(true);
 expect(compact(preparation).includes("s.next_close_demand(4096)")).toBe(true);
 expect(compact(apply).includes("source.next_close_demand(4096)")).toBe(true);
 expect(compact(sequence).includes("source.next_close_demand(4096)")).toBe(true);
 expect(compact(apply).includes("preflight_retained_diff(&mutowner,diff.borrow(),before.borrow(),&post,128,quote,&mutreceiving)")).toBe(true);
 expect(compact(sequence).includes("protocol::preflight_retained_diff(&mutowner,diff.borrow(),original.borrow(),&post,body,grant(d),&mutreceiving)")).toBe(true);
 expect(compact(sequence).includes("self.0.preflight(d.project(0,|d|&d.edits),s,post,body,g,&mut*receiving)")).toBe(true);
 expect(compact(receiving).includes("self.field.advance(d.project(0,|d|&d.text),receiving,body,g)")).toBe(true);
 expect(compact(receiving).includes("preflight_retained_diff(&mutowner,d.borrow(),original.borrow(),&post,4096,g,receiving)")).toBe(true);
});
