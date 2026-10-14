/** 👷️ Independent SQLite oracle for the original reserved job's payload/backing currencies. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
const law=JSON.parse(readFileSync(new URL("../../../🧪️tests/🧫️fixtures/👷️reserved/🔣️.json",import.meta.url),"utf8"));

test("reserved session admission borrows original control and keeps both refusal owners",()=>{
 const validate=new Ajv({strict:true}).compile({type:"object",additionalProperties:false,required:["control","sameCancellationNode","wholeFailure","recipient"],properties:{control:{const:"originalStepContext"},sameCancellationNode:{const:true},wholeFailure:{const:true},recipient:{const:"originalRecipient"}}});
 expect(validate(law.admission)).toBe(true);expect(validate({...law.admission,clock:0})).toBe(false);
 const db=new Database(":memory:");try{expect(db.query("select ? as deadline, ? as fuel, ? as capacity").get(law.caller.deadline,law.caller.fuel,law.caller.source.maximumCapacityBytes)).toEqual({deadline:1500,fuel:1,capacity:65536});}finally{db.close();}
 const source=readFileSync(new URL("../../🚪️session/🦀️.rs",import.meta.url),"utf8"),start=source.indexOf("macro_rules! admit_session"),admission=source.slice(start);
 expect(admission).toContain("$cx");expect(admission).toContain("try_admit_owned(&mut job, &mut params, $cx)");
 expect(admission).not.toContain("WorkerJobAdmissionContext::new");expect(admission).not.toContain("StepBudget::new");
 expect(source).toContain("ManuallyDrop<Option<OriginalTurnFailure>>");expect(source).toContain("ControlledRetirement<OriginalTurnFailure>");
 console.log("[DEBUG] Ajv/SQLite reserved admission requires actual original1500 deadline/fuel1/65536 capacity supplier and whole primary plus independent refusal custody");
});

test("reserved close preserves original typed depth refusal before the shared cursor gate",()=>{
 expect(law.zeroDepth).toBe("sameOwnerDepthLimit");const db=new Database(":memory:");try{expect(db.query("select ? < ? as denied, ? as owned").get(0,law.grant.depth,law.backingBytes)).toEqual({denied:1,owned:16384});}finally{db.close();}
 const source=readFileSync(new URL("../../🏛️reserved/🦀️.rs",import.meta.url),"utf8"),start=source.indexOf("fn close_step(&mut self,cx:"),body=source.slice(start,source.indexOf("fn next_close_copy_byte_demand",start));
 expect(start).toBeGreaterThanOrEqual(0);expect(body).toContain("remaining.maximum_depth<demand.depth");expect(body).toContain("ValueRefusalKind::DepthLimit");expect(body).toContain("OriginalTurnFailure::from");
 expect(body.indexOf("remaining.maximum_depth<demand.depth")).toBeLessThan(body.indexOf("close_original_retained_step"));
 console.log("[DEBUG] SQLite zero-depth reference keeps all original16384 backing bytes; native reserved close must retain complete typed depth failure before pending currency gate");
});
test("original reserved job retains spare capacity until exact physical release",async()=>{
 const implementation=await import("../🟦️.ts");
 const db=new Database(":memory:");db.run("CREATE TABLE owner(length INTEGER,capacity INTEGER)");db.run("INSERT INTO owner VALUES(?,?)",law.payloadBytes,law.backingBytes);
 let original={length:law.payloadBytes,capacity:law.backingBytes};
 for(const expected of law.turns){
  const quote=implementation.reservedJobByteDemand(original);
  const before=db.query("SELECT length,capacity FROM owner").get()as {length:number;capacity:number};
  const next=implementation.reservedJobByteTurn(original,law.grant);
  if(before.length>0)db.run("UPDATE owner SET length=MAX(0,length-?)",law.grant.copyBytes);else db.run("UPDATE owner SET capacity=0");
  const after=db.query("SELECT length,capacity FROM owner").get()as {length:number;capacity:number};
  expect(next.owner).toEqual(after);expect(next.progress).toEqual(expected);expect(quote.capacityBytes).toBe(0);
  const denied=implementation.reservedJobByteTurn(original,{...law.grant,items:0});expect(denied.owner).toBe(original);expect(denied.progress).toEqual({copiedItems:0,copiedBytes:0,capacityBytes:0,releaseBytes:0});
  if(before.length>0){const short=implementation.reservedJobByteTurn(original,{...law.grant,copyBytes:0,capacityBytes:law.backingBytes,releaseBytes:law.backingBytes});expect(short.owner).toBe(original);expect(short.progress).toEqual({copiedItems:0,copiedBytes:0,capacityBytes:0,releaseBytes:0});}
  const shallow=implementation.reservedJobByteTurn(original,{...law.grant,depth:0});expect(shallow.owner).toBe(original);expect(shallow.refusal).toBe("depthLimit");expect(shallow.progress).toEqual({copiedItems:0,copiedBytes:0,capacityBytes:0,releaseBytes:0});
  if(before.length===0){const short=implementation.reservedJobByteTurn(original,{...law.grant,releaseBytes:original.capacity-1});expect(short.owner).toBe(original);expect(short.progress.releaseBytes).toBe(0);}
  original=next.owner;
 }
 expect(original).toEqual({length:0,capacity:0});const terminal=implementation.reservedJobByteTurn(original,law.grant);expect(terminal.owner).toBe(original);expect(terminal.complete).toBe(true);expect(terminal.progress).toEqual({copiedItems:0,copiedBytes:0,capacityBytes:0,releaseBytes:0});db.close();
 console.log(`[DEBUG] independent SQLite original reserved job copied=${law.payloadBytes} physicalRelease=${law.backingBytes}`);
});
