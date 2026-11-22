/** 📌️ Contract: a run commits to the implementation it started with: a format stamp edited while a pinned run holds a producer lease never makes it yield, it completes and publishes what it loaded, so the edit reaches the next run only; a refusal of a store that is newer than the loaded code stays, and an unpinned process still yields. */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import fixture from "./🔣️.json";
import {acquireQueuedResourceLease} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {withImplementationPinV1} from "../../../📥️inputs/🔏️revision/🟨️.mjs";
import {createFormatStampsV1,formatStampProtocolV1,registerFormatStampGuardV1,StaleImplementationErrorV1} from "../../🟨️.mjs";

const scratches:string[]=[],protocol=formatStampProtocolV1();
afterAll(()=>{for(const path of scratches)rmSync(path,{recursive:true,force:true});});
const write=(path:string,stamp:string)=>{writeFileSync(path+".tmp",JSON.stringify({f:{stamp}}));renameSync(path+".tmp",path);};

for(const row of fixture.cases)test(row.id,async()=>{
 const base=realpathSync(mkdtempSync(join(tmpdir(),"pin-")));scratches.push(base);const directory=join(base,"leases"),schema=join(base,"s.json");mkdirSync(directory);write(schema,"A");
 const table={...protocol,stamps:[{id:"p.format",schema:"s.json",pointer:"/f/stamp"}],resources:[{prefix:"producer:",stamps:["p.format"]}]},registry=createFormatStampsV1({base,table:table as never,environment:{SEMIO_LANGUAGE:"en"}});
 registry.declare("p.format",{f:{stamp:"A"}});
 const published:string[]=[],run=async()=>{for(const edit of row.edits)write(schema,edit);const held=await acquireQueuedResourceLease({directory,resource:"producer:graph",mode:"exclusive",owner:"pin-"+process.pid,signal:new AbortController().signal});try{if(row.kind==="refuse")registry.ensureCurrent(["p.format"],"store","refuse");published.push(registry.owned().get("p.format") as string);}finally{held.release();}};
 const unregister=registerFormatStampGuardV1(registry);
 try{
  const outcome=await (row.pinned?withImplementationPinV1(run):run()).then(()=>"completes",error=>error instanceof StaleImplementationErrorV1?"stale":"failed");
  expect(outcome).toBe(row.outcome);expect(published).toEqual(row.published);
 }finally{unregister();}
});
