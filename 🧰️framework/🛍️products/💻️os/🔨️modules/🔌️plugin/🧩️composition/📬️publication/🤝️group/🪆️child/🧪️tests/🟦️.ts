import { test,expect } from "bun:test";
import { readFileSync } from "node:fs";
import { Database } from "bun:sqlite";
import Ajv from "ajv/dist/2020";
const root=new URL("../",import.meta.url);
const fixture=JSON.parse(readFileSync(new URL("🧫️fixtures/📍️declaration-query.json",root),"utf8"));
test("private child declaration capacity frontier agrees with independent SQLite ordered roster",()=>{
 const schema=JSON.parse(readFileSync(new URL("📐️declaration-query.schema.json",root),"utf8"));expect(new Ajv({strict:false}).compile(schema)(fixture)).toBe(true);
 const database=new Database(":memory:");database.run("CREATE TABLE declarations(ordinal INTEGER PRIMARY KEY,kind TEXT,standard TEXT,subset TEXT)");
 fixture.declarations.forEach((row:any,index:number)=>database.run("INSERT INTO declarations VALUES(?,?,?,?)",[index,row.kind,row.standard,row.subset]));
 for(const row of fixture.cases){expect((database.query("SELECT ordinal FROM declarations WHERE kind=? AND standard='1' AND subset='*'").get(row.kind) as any).ordinal).toBe(row.matchOrdinal);for(let advanced=0;advanced<=fixture.declarations.length;advanced++){const unread=(database.query("SELECT COUNT(*) AS count FROM declarations WHERE ordinal>=?").get(advanced) as any).count;expect(unread===0).toBe(row.capacityReadyAfterOrdinals[advanced]);}}
 database.close();
});
test("private child exact query stays structural until its sealed declaration roster is exhausted",()=>{
 const source=readFileSync(new URL("🦀️.rs",root),"utf8");const query=source.slice(source.indexOf("fn next_capacity_byte_demand"),source.indexOf("fn advance"));
 expect(query).toContain("declaration_count");expect(source).toContain("declarations.as_ptr()");expect(source).toContain("declarations.len()");
});
test("private genesis whole page admission retains the absent output frontier under one-below birth",()=>{
 const database=new Database(":memory:");database.run("CREATE TABLE grants(pages INTEGER,birth INTEGER,grantBytes INTEGER)");
 for(const row of fixture.pageAdmissionCases){database.run("INSERT INTO grants VALUES(?,?,?)",[row.pages,row.wholeBirthBytes,row.deniedCapacityBytes]);expect(row.wholeBirthBytes).toBe(row.pages*(4096+2));expect(row.deniedCapacityBytes+1).toBe(row.wholeBirthBytes);expect(row.outputOwnerReady).toBe(false);expect([row.copiedItems,row.copiedBytes,row.birthBytes,row.freeBytes]).toEqual([0,0,0,0]);}
 expect((database.query("SELECT COUNT(*) AS count FROM grants WHERE grantBytes>=birth").get() as any).count).toBe(0);database.close();
 const source=readFileSync(new URL("../../../📨️emission/🌱️genesis/📄️input/🦀️.rs",root),"utf8");expect(source).toContain("if self.pages.pages.is_none(){return Ok(step);}");
});
test("private input completion belongs to its whole original owner rather than one nested cursor",()=>{
 const database=new Database(":memory:");database.run("CREATE TABLE completion(phase TEXT,innerComplete INTEGER,outerReady INTEGER)");
 for(const row of fixture.completionCases){database.run("INSERT INTO completion VALUES(?,?,?)",[row.phase,Number(row.innerComplete),Number(row.outerReady)]);expect(Boolean((database.query("SELECT innerComplete AND outerReady AS complete FROM completion WHERE phase=?").get(row.phase) as any).complete)).toBe(row.complete);}
 database.close();const source=readFileSync(new URL("🦀️.rs",root),"utf8");expect(source).toContain("self.genesis.close_granted(grant).map(input_pending)");expect(source).toContain("RetainedCloneStep::Complete(progress) => RetainedCloneStep::Progress(progress)");
});
