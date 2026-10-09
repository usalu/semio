/** 👷️ Independent SQLite oracle for the original reserved job's payload/backing currencies. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
const law=JSON.parse(readFileSync(new URL("../../../🧫️fixtures/👷️reserved/🔣️.json",import.meta.url),"utf8"));
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
