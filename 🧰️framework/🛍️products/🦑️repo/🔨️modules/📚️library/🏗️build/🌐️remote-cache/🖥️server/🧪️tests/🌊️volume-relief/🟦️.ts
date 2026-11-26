/** 🧪️ Contract: the cache host's maintenance tick hands its volume relief to the guard, so a volume that other writers fill below the relief level is defended by the derivable classes outside the store. */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {resetCasGuardsV1} from "../../../../🗄️cas/🚦️guard/🟦️.ts";
import {CAS_POLICY_V1} from "../../../../🗄️cas/🧬️schema/🟦️.ts";
import {createCasStoreV1} from "../../../../🗄️cas/🗃️store/🟦️.ts";
import {startRemoteCacheServerV1} from "../../🟦️.ts";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

test("the host-store tick calls the volume relief when the partition is below the relief level of the hard reserve",async()=>{
 resetCasGuardsV1();
 const base=mkdtempSync(join(tmpdir(),"rcache-relief-"));scratch.push(base);
 const policy={...CAS_POLICY_V1,enforceTickMilliseconds:200,sizing:{...CAS_POLICY_V1.sizing,roles:CAS_POLICY_V1.sizing.roles.map(role=>({...role,reserveFloorBytes:2**50}))}};
 const requests:Array<{root:string;targetFreeBytes:number;freeBytes:number}>=[];
 const server=await startRemoteCacheServerV1({directory:join(base,"dir"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"t"},validation:"off",hostStore:createCasStoreV1({root:join(base,"cas"),policy}),relieveVolume:async request=>{requests.push({root:request.root,targetFreeBytes:request.targetFreeBytes,freeBytes:request.freeBytes});},startupScan:"background"});
 try{
  const deadline=Date.now()+10000;
  while(requests.length===0&&Date.now()<deadline)await Bun.sleep(100);
  expect(requests.length).toBeGreaterThan(0);
  expect(requests[0]!.root).toBe(join(base,"cas"));
  expect(requests[0]!.targetFreeBytes).toBeGreaterThan(requests[0]!.freeBytes);
 }finally{await server.stop();resetCasGuardsV1();}
});

test("the fleet cache sidecar binds the volume relief of its machine to the server",()=>{
 const sidecar=readFileSync(join(import.meta.dir,"..","..","..","..","🌐️remote-execution","🛰️sidecar","🗄️cache","🟦️.ts"),"utf8");
 expect(sidecar).toContain("relieveVolume:async request=>");
 expect(sidecar).toContain("relieveMachineVolumeV1({workspace:context.root");
});
