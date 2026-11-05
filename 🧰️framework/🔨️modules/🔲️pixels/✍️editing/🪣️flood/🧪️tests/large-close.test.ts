import{test,expect}from"bun:test";
import Ajv from"ajv";
import{FloodRegionCursor}from"../🟦️.ts";
import fixture from"./large-close.json";
import schema from"./large-close.schema.json";

test("accepted1024square cancellation closes with actual mounted release authority",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const cursor=new FloodRegionCursor(fixture.width,fixture.height,fixture.seed[0]!,fixture.seed[1]!);
 const source=fixture.uniform as[number,number,number,number];let retained=0,released=0;
 let initialized=false;
 for(let index=0;index<fixture.maximumAllocationTurns;index++){
  const turn=cursor.advance(source,fixture.maximumWork,fixture.grant);expect(turn.result.ok).toBe(true);expect(turn.work).toBe(1);
  if(turn.result.ok){retained+=turn.result.value.progress.retainedCapacityBytes;if(turn.result.value.completed>0){initialized=true;break;}}
 }
 expect(initialized).toBe(true);
 expect(retained).toBe(fixture.expected.retainedBytes);
 const cancelled=cursor.advance(source,fixture.maximumWork,fixture.grant,true);expect(cancelled.result.ok).toBe(false);if(!cancelled.result.ok)expect(cancelled.result.error.kind).toBe("canceled");
 expect(()=>cursor.result()).toThrow();cursor.beginClose();
 const initialQuote=cursor.closeDemands(fixture.grant.maximumCopyBytes);const receipts=[];
 for(let index=0;index<fixture.maximumCloseTurns&&!cursor.closed;index++){
  const turn=cursor.close(fixture.grant);expect(turn.result.ok).toBe(true);
  if(turn.result.ok){expect(turn.work).toBe(1);expect(turn.result.value.progress.releasedBytes).toBeGreaterThan(0);expect(turn.result.value.progress.releasedBytes).toBeLessThanOrEqual(fixture.grant.maximumReleaseBytes);released+=turn.result.value.progress.releasedBytes;receipts.push({work:turn.work,...turn.result.value.progress});}
 }
 console.log("[DEBUG] large flood close probe "+JSON.stringify({retained,released,closed:cursor.closed,initialQuote,grant:fixture.grant,receiptCount:receipts.length}));
 expect(cursor.closed).toBe(fixture.expected.closed);expect(released).toBe(fixture.expected.releasedBytes);
});
