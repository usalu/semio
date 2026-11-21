import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {acquireResourceLease,ResourceLeaseTimeoutErrorV1,type ResourceLease} from "@semio-tech/process/leases";
import {repositoryDependencyBindingsBusyV1,settleRepositoryDependencyBindingsResultV1,tryRepositoryDependencyBindingsV1} from "../🟦️.ts";
import fixture from "./🔣️.json";

const control=(signal=new AbortController().signal)=>({signal,remainingMilliseconds:()=>null,publish(){},yieldContinuation:async()=>{}});

test("protected bindings probe once and report contention after closing the dependency image",async()=>{
 const previous=process.env.SEMIO_LEASE_ORDER;
 process.env.SEMIO_LEASE_ORDER="enforce";
 try{for(const row of fixture.cases){
  const root=mkdtempSync(join(realpathSync(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"db-")),directory=join(root,"leases");
  mkdirSync(directory);
  const bindings="javascript-dependency-bindings:"+root,image="native-fixed-image:"+join(root,"image"),actual=control(),foreign=control();
  const held:ResourceLease[]=[];
  const acquire=async(resource:string,mode:"shared"|"exclusive",owner=foreign,timeoutMs=0)=>{
   const lease=await acquireResourceLease({directory,resource,mode,timeoutMs,control:owner,signal:owner.signal});
   held.push(lease);
   return lease;
  };
  const close=(lease:ResourceLease)=>{lease.release();held.splice(held.indexOf(lease),1);};
  try{
   const peer=row.peerBinding===null?undefined:await acquire(bindings,row.peerBinding as "shared"|"exclusive");
   const maintenance=await acquire(image,"exclusive",actual,1000);
   const binding=await tryRepositoryDependencyBindingsV1(directory,bindings,actual);
   if(binding)held.push(binding);
   expect(binding?"acquired":"busy").toBe(row.maintenanceBinding);
   const during=await acquire(image,"shared").then(()=>"acquired",error=>{expect(error).toBeInstanceOf(ResourceLeaseTimeoutErrorV1);return "busy";});
   expect(during).toBe(row.readerDuringMaintenance);
   const result=Object.freeze({acquired:true,value:binding?"maintained":repositoryDependencyBindingsBusyV1});
   if(binding)close(binding);
   close(maintenance);
   expect(settleRepositoryDependencyBindingsResultV1(result)).toEqual(binding?result:{acquired:false});
   const reader=await acquire(image,"shared");
   expect(reader?"acquired":"busy").toBe(row.readerAfterMaintenance);
   if(peer)close(peer);
  }finally{
   for(const lease of held.reverse())lease.release();
   rmSync(root,{recursive:true,force:true});
  }
 }}finally{if(previous===undefined)delete process.env.SEMIO_LEASE_ORDER;else process.env.SEMIO_LEASE_ORDER=previous;}
});

test("binding try preserves cancellation rather than returning contention",async()=>{
 const abort=new AbortController(),reason=Error("canceled binding probe");
 abort.abort(reason);
 const result=await tryRepositoryDependencyBindingsV1("/unused","javascript-dependency-bindings:/unused",control(abort.signal)).catch(error=>error);
 expect(result).toBe(reason);
});
