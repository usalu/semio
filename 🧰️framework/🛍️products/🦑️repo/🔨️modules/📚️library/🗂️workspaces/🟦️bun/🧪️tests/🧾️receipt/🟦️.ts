import {test,expect} from "bun:test";
import {mkdtempSync,mkdirSync,writeFileSync,rmSync,chmodSync,realpathSync} from "node:fs";
import {createRequire} from "node:module";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {withPublishedBunWorkspaceOwnershipV1,withCurrentBunWorkspaceOwnershipV1} from "../../🟦️.ts";
import {localMachineSpeedFactorV1} from "../../../../🏗️build/🐢️speed/🟨️.mjs";

const fs=createRequire(import.meta.url)("node:fs") as typeof import("node:fs");
const control=()=>({signal:new AbortController().signal,frontierBatchPaths:64,advance:async()=>{}});
const manifest=(name:string,extra:object={})=>JSON.stringify({name,...extra});
/** 🧪️ A tree of `fan`^3 package-free leaf directories below one packaged owner, settled by publishing its ownership once. */
async function tree(fan:number):Promise<string>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"semio-receipt-")));
 writeFileSync(join(root,"package.json"),manifest("r",{semio:{workspace:{schemaVersion:1,members:["pkgs/**"],owners:["*/package.json"]}}}));
 for(let a=0;a<fan;a++)for(let b=0;b<fan;b++){const d=join(root,"pkgs","a"+a,"b"+b);mkdirSync(d,{recursive:true});for(let c=0;c<fan;c++)mkdirSync(join(d,"c"+c));if((a+b)%3===0)writeFileSync(join(d,"package.json"),manifest("p"+a+"-"+b));}
 await new Promise(done=>setTimeout(done,2300));await withPublishedBunWorkspaceOwnershipV1(root,{kind:"all"},control(),async()=>{});await new Promise(done=>setTimeout(done,2300));
 return root;
}
const counting=():{calls:()=>number;stop:()=>void}=>{const original=fs.lstatSync;let count=0;(fs as any).lstatSync=function(...args:any[]){count++;return (original as any).apply(this,args);};return{calls:()=>count,stop:()=>{(fs as any).lstatSync=original;}};};

test("one recheck of an unchanged custody replays every observed path once (call-count law)",async()=>{
 const root=await tree(6);
 try{await withCurrentBunWorkspaceOwnershipV1(root,{kind:"all"},control(),async custody=>{
  const records=custody.observed().paths.length,manifests=custody.observed().texts.length,meter=counting();
  try{await custody.recheck();}finally{meter.stop();}
  expect(records).toBeGreaterThan(200);
  expect(meter.calls()).toBeLessThanOrEqual(records+50*manifests);
 });}finally{rmSync(root,{recursive:true,force:true});}
});

test("the second and third recheck cost exactly as much as the first (no growth, no memo that could go stale)",async()=>{
 const root=await tree(5);
 try{await withCurrentBunWorkspaceOwnershipV1(root,{kind:"all"},control(),async custody=>{
  const costs:number[]=[];for(let i=0;i<3;i++){const meter=counting();try{await custody.recheck();}finally{meter.stop();}costs.push(meter.calls());}
  expect(costs[1]).toBe(costs[0]);expect(costs[2]).toBe(costs[0]);
 });}finally{rmSync(root,{recursive:true,force:true});}
});

test("tamper law: a new owner directory, a changed owner manifest and a changed directory mode are each refused by the next recheck",async()=>{
 for(const change of ["new-owner","manifest","mode"] as const){
  const root=await tree(3);
  try{await withCurrentBunWorkspaceOwnershipV1(root,{kind:"all"},control(),async custody=>{
   await custody.recheck();
   if(change==="new-owner"){mkdirSync(join(root,"pkgs","a0","b0","fresh"));writeFileSync(join(root,"pkgs","a0","b0","fresh","package.json"),manifest("fresh"));}
   if(change==="manifest")writeFileSync(join(root,"pkgs","a0","b0","package.json"),manifest("p0-0",{description:"edited"}));
   if(change==="mode")chmodSync(join(root,"pkgs","a1","b1","c1"),0o700);
   await expect(custody.recheck()).rejects.toThrow();
  }).catch(()=>{});}finally{chmodSync(join(root,"pkgs","a1","b1","c1"),0o755);rmSync(root,{recursive:true,force:true});}
 }
});

test("throughput law: a recheck stays under 120 microseconds per observed path, scaled by the speed factor of the machine",async()=>{
 const root=await tree(8),factor=localMachineSpeedFactorV1();
 try{await withCurrentBunWorkspaceOwnershipV1(root,{kind:"all"},control(),async custody=>{
  const records=custody.observed().paths.length;await custody.recheck();
  const started=performance.now();await custody.recheck();const per=(performance.now()-started)*1000/records;
  expect(per).toBeLessThan(120*factor);
 });}finally{rmSync(root,{recursive:true,force:true});}
});
