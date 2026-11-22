import{test,expect}from"bun:test";
import Ajv from"ajv";
import ts from"typescript";
import{readFileSync}from"node:fs";
import schema from"./🧬️schema.json";
import fixture from"./🔣️.json";
test("optional ledger open is bounded, never holds the exclusive lease while waiting on a peer's SQLite lock, and degrades to a declining ledger",async()=>{
 const source=ts.createSourceFile("storage.mjs",readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true,ts.ScriptKind.JS),declaration=source.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="openOriginalGraphLedgerV1");if(!declaration)throw Error("Actual ledger open required");
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 for(const row of fixture.cases){
  let held=false,opens=0,heldDuringWait=false,busy=row.busyAttempts;const real={observe:()=>({usedBytes:0}),close:()=>{}},waits:boolean[]=[];
  const binding={
   quotaSchema:{definitions:{DefaultPolicy:{const:{maximumBytes:100,minimumAvailableBytes:1,lowWaterBytes:50}}}},
   ledgerBusyLimitMsV1:row.limitMs,
   repositoryLeaseDirectoryV1:()=>"/lease",resolve:(p:string)=>p,randomBytes:()=>({toString:()=>"owner"}),
   checkResourceLeaseControl:()=>{},
   acquireQueuedResourceLease:async(options:{resource:string;timeoutMs:number})=>{expect(options.timeoutMs).toBeGreaterThan(0);expect(held).toBe(false);if(row.leaseTimeout)throw Error("Resource lease timed out: "+options.resource);held=true;return{release:()=>{held=false;}};},
   openBuildGraphOptionalLedgerV1:async()=>{opens++;expect(held).toBe(true);if(row.error==="boom")throw Error("boom");if(busy-->0)throw Object.assign(Error("database is locked"),{code:"SQLITE_BUSY"});return real;}
  },run=new Function(...Object.keys(binding),"return ("+declaration.getText(source)+")")(...Object.values(binding));
  const control={publish:async()=>{waits.push(held);}},started=Date.now();let result:any,failure:unknown;
  try{result=await run("/workspace",{root:"/root",image:{}},control);}catch(error){failure=error;}
  expect(waits.every(h=>!h),row.id).toBe(true);expect(held,row.id).toBe(false);expect(Date.now()-started,row.id).toBeLessThan(row.limitMs+1500);
  if(row.outcome==="throws"){expect((failure as Error).message,row.id).toBe("boom");}
  else if(row.outcome==="ledger"){expect(failure,row.id).toBeUndefined();expect(result,row.id).toBe(real);}
  else{expect(failure,row.id).toBeUndefined();expect(result.unavailable,row.id).toBe(true);expect(result.observe().usedBytes,row.id).toBe(100);const reasons:string[]=[];expect(result.publish({},()=>{},(r:string)=>reasons.push(r))).toBe(false);expect(reasons).toEqual(["ledger-contention"]);expect(()=>result.retire(()=>{})).toThrow("Optional graph ledger busy");expect(()=>result.close()).not.toThrow();}
  if(row.opens!==null)expect(opens,row.id).toBe(row.opens);
 }
});
