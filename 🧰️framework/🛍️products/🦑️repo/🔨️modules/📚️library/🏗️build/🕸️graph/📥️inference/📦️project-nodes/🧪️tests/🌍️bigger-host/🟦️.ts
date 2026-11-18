/** 🌍️ Contract: what a bigger host measured never bars an owner on a smaller machine: a worker of 16 GB that reads a cost table shared through the fleet runs the owner alone and lets its own governor judge it, so one owner of 858 MB is not refused for a number a 64 GB machine measured with a lax heap. What this machine or a smaller one measured, and what it stopped itself, still bars.
 * @see ../../🟨️.mjs */
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{graphOwnerEnforcementV1,ownHostTableV1}from"../../🟨️.mjs";
import{recordOwnerCostFileV1}from"../../../../../🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts";

const MB=1048576,scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const withTotal=async<T>(megabytes:number,work:()=>Promise<T>)=>{const before=process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=String(megabytes);try{return await work();}finally{if(before===undefined)delete process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;else process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=before;}};
const enforce=(root:string,host:number)=>withTotal(host,()=>graphOwnerEnforcementV1(root,process.env,1024*MB,undefined,()=>host*MB*3/4,()=>host));
const record=(root:string,name:string,megabytes:number,hostMegabytes:number,stopped=false)=>recordOwnerCostFileV1(join(root,".cache/f/graph-costs/owner-costs.3.json"),{name,kind:"project",megabytes,seconds:90,rows:10,stopped,atMs:Date.now(),hostMegabytes});

test("a cost that only a bigger host measured does not bar an owner in process; the same cost measured on this host or a smaller one does",async()=>{
 const root=mkdtempSync(join(tmpdir(),"bigger-host-"));scratch.push(root);
 record(root,"@semio-tech/lax",5200,65536);
 record(root,"@semio-tech/own",5200,16384);
 record(root,"@semio-tech/smaller",5200,8192);
 const enforcement=await enforce(root,16384);
 expect(enforcement.neverHere!(["@semio-tech/lax","@semio-tech/own","@semio-tech/smaller"])).toEqual(["@semio-tech/own","@semio-tech/smaller"]);
});

test("an owner with samples of both kinds is judged by the samples of this host and smaller ones only",async()=>{
 const root=mkdtempSync(join(tmpdir(),"bigger-host-mixed-"));scratch.push(root);
 record(root,"@semio-tech/mixed",5200,65536);record(root,"@semio-tech/mixed",900,16384);
 const enforcement=await enforce(root,16384);
 expect(enforcement.neverHere!(["@semio-tech/mixed"])).toEqual([]);
});

test("the table law: every sample that stays is from a host no bigger than the machine, and the owners keep their other fields",()=>{
 const table={version:2,owners:{a:{kind:"project",rows:3,seconds:9,samples:[{megabytes:1,atMs:1,stopped:false,hostMegabytes:32768},{megabytes:2,atMs:2,stopped:true,hostMegabytes:16384},{megabytes:3,atMs:3,stopped:false,hostMegabytes:8192}]}}};
 const own=ownHostTableV1(table as never,16384) as typeof table;
 expect(own.owners.a.samples.map(sample=>sample.megabytes)).toEqual([2,3]);
 expect(own.owners.a.rows).toBe(3);expect(own.owners.a.seconds).toBe(9);expect(own.version).toBe(2);
});
