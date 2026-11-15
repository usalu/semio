/** 💰️ Contract: a wave of the real child that had exactly one project at its memory peak proves that project's cost and the server keeps it for the planning of every reader; several projects at the peak, a peak under the floor or no cost path prove nothing. */
import{expect,test}from"bun:test";
import{chmodSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{renderGraphWaveRowV1}from"../../../../../🕸️graph/📈️wave-row/🟨️.mjs";
import{estimateOwnerCostV1,ownerCostPathV1,readOwnerCostFileV1}from"../../../../../🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts";
import{wantedServerPolicyV1}from"../../../../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts";
import{inferGraphReadV1}from"../../../🕸️graph/🟦️.ts";

const row=(generation:number,peakMegabytes:number,managedMegabytes:number,peakProjects:string[])=>renderGraphWaveRowV1({version:1,generation,done:48,total:96,seconds:372,parentMegabytes:1987,managedMegabytes,peakMegabytes,peakProjects,collections:10,collectMilliseconds:100,farm:{workers:6,projects:87,adopted:7,failed:0,lost:0,declined:0}});
const fake=(script:string)=>{const dir=mkdtempSync(join(tmpdir(),"graph-costs-")),path=join(dir,"repo");writeFileSync(path,"#!/bin/sh\n"+script+"\n");chmodSync(path,0o755);return{dir,path};};
const read=(command:string,costPath:string|undefined)=>inferGraphReadV1({command,cwd:tmpdir(),environment:{},reason:"refresh",signal:undefined,language:"en",say:()=>{},...costPath===undefined?{}:{costPath},policy:{...wantedServerPolicyV1,inferStallMilliseconds:5000,watchMilliseconds:40}});

test("the measured waves of g1-server: the sole project at the peak is recorded with the peak above the surviving floor, the others are not",async()=>{
 const {dir,path}=fake(`echo '${row(154,18814,4838,["@semio-tech/s-dev-composition"])}' >&2\necho '${row(144,8365,4332,["workspace","@semio-tech/2d-booleans-rs","semio-repo-test-host"])}' >&2\necho '${row(142,7310,3969,[])}' >&2\necho '${row(158,5200,5062,["@semio-tech/tiny"])}' >&2\nexit 0`);
 try{
  const costPath=ownerCostPathV1(dir),result=await read(path,costPath);
  expect(result.ok).toBe(true);
  const table=readOwnerCostFileV1(costPath);
  expect(Object.keys(table.owners)).toEqual(["@semio-tech/s-dev-composition"]);
  expect(estimateOwnerCostV1(table,"@semio-tech/s-dev-composition")).toEqual({megabytes:13976,source:"recorded"});
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("without a cost path the read works as before and writes nothing",async()=>{
 const {dir,path}=fake(`echo '${row(154,18814,4838,["@semio-tech/s-dev-composition"])}' >&2\nexit 0`);
 try{expect((await read(path,undefined)).generation).toBe(154);expect(readOwnerCostFileV1(ownerCostPathV1(dir)).owners).toEqual({});}finally{rmSync(dir,{recursive:true,force:true});}
});
