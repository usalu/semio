/** 🎟️ Original retained work and currencies cross the defining actor wire without renewal. */
import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
test("original turn work and returned receipt preserve all independent currencies",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🔣️.json",import.meta.url)).json(),schema=await Bun.file(new URL("../🧬️schema/🔣️.json",import.meta.url)).json();const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);
 for(const key of ["operation","generation","epoch","work","grant"]){const input={...law.input};delete input[key];expect(validate({...law,input})).toBe(false)}
 for(const key of ["clockBinding","absoluteDeadlineUs","fuel"]){const work={...law.input.work};delete work[key];expect(validate({...law,input:{...law.input,work}})).toBe(false)}
 const db=new Database(":memory:");try{
  db.run("CREATE TABLE original_turn(axis TEXT PRIMARY KEY,granted INTEGER,spent INTEGER,remaining INTEGER)");for(const [axis,spent]of [["maximumItems","copiedItems"],["maximumCopyBytes","copiedBytes"],["maximumCapacityBytes","retainedCapacityBytes"],["maximumReleaseBytes","releasedBytes"]] as const)db.run("INSERT INTO original_turn VALUES(?,?,?,?)",axis,law.input.grant[axis],law.spent[spent],law.remaining[axis]);db.run("INSERT INTO original_turn VALUES(?,?,?,?)","fuel",law.input.work.fuel,law.workUnitsSpent,law.remainingFuel);expect(db.query("SELECT COUNT(*) AS n FROM original_turn WHERE granted=spent+remaining").get()).toEqual({n:5});expect(law.input.grant.maximumDepth).toBe(law.remaining.maximumDepth);
  db.run("CREATE TABLE work(clock INTEGER,deadline INTEGER,fuel INTEGER)");db.run("INSERT INTO work VALUES(?,?,?)",law.input.work.clockBinding,law.input.work.absoluteDeadlineUs,law.remainingFuel);expect(db.query("SELECT count(*) AS n FROM work WHERE clock=? AND deadline=? AND fuel=?").get(law.input.work.clockBinding,law.input.work.absoluteDeadlineUs,law.remainingFuel)).toEqual({n:1});expect(db.query("SELECT count(*) AS n FROM work WHERE clock=? AND deadline=?").get(law.input.work.clockBinding+1,law.input.work.absoluteDeadlineUs)).toEqual({n:0});
 }finally{db.close()}
 expect(await Bun.file(new URL("../🦀️.rs",import.meta.url)).exists()).toBe(true);console.log("[DEBUG] independent Ajv/SQLite original clock/deadline/work and four retained currencies conserved; native176-byte receipt and cross-ABI issuer clock are separate assertions");
});
test("issued original batch conserves work independently from physical metadata",async()=>{
 const law=await Bun.file(new URL("../📦️batch/🧪️tests/🧫️fixtures/🔣️.json",import.meta.url)).json(),schema=await Bun.file(new URL("../📦️batch/🧬️schema/📥️input.json",import.meta.url)).json();expect(new Ajv({strict:true}).compile(schema)({input:law.input,envelopes:law.envelopes})).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE work(seq INTEGER PRIMARY KEY,spent INTEGER,metadata INTEGER)");for(let seq=0;seq<law.envelopes;seq++){db.run("INSERT INTO work VALUES(?,?,?)",seq,law.workSpent[seq],law.spent[seq].copiedItems);const remaining=db.query("SELECT ?-sum(spent) AS fuel FROM work WHERE seq<=?").get(law.input.work.fuel,seq)as{fuel:number};expect(remaining.fuel).toBe(law.activeRemainingFuel[seq]);}expect(db.query("SELECT sum(spent) AS work,sum(metadata) AS metadata FROM work").get()).toEqual({work:3,metadata:1});expect(law.input.work.fuel-law.workSpent.reduce((sum:number,value:number)=>sum+value,0)).toBe(law.remainingFuel);}finally{db.close()}
 console.log("[DEBUG] independent Ajv/SQLite batch actual work3 differs from physical metadata1; original fuel64 becomes61 with same clock/deadline across suspension");
});
