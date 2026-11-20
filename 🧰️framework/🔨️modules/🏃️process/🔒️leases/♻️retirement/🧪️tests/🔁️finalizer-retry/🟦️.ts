/** 🔁️ Contract: a directory admission whose release failed (a full disk fails the close) is retried by the finalizer itself, so a process that never touches the lease store again does not hold the gate shared for ever and starve every exclusive retirement. */
import{test,expect}from"bun:test";
import{ResourceLeaseAdmissionFinalizerV1}from"../../🟦️.ts";

test("a failed gate release is retried in the background until it is released",async()=>{
 let attempts=0,released=false;
 const gate={release(){attempts++;if(attempts<2)throw Error("ENOSPC");released=true;}},finalizer=new ResourceLeaseAdmissionFinalizerV1<typeof gate>();
 expect(()=>finalizer.release(gate)).toThrow("ENOSPC");
 expect(released).toBe(false);
 const deadline=Date.now()+5000;
 while(!released&&Date.now()<deadline)await new Promise(resolve=>setTimeout(resolve,50));
 expect(released).toBe(true);
 expect(attempts).toBe(2);
});
