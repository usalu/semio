/** 🧪️ The reclaim commands collect the host store first when the partition is full, before any lease, admission or receipt write. */
import {mkdirSync,mkdtempSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {expect,test} from "bun:test";
import cases from "./🔣️.json" with {type:"json"};
import {CAS_FULL_DISK_RESERVE_BYTES_V1,casReclaimCommandV1,reclaimFullDiskFirstV1} from "../../🆘️full-disk/🟦️.ts";
import {runCasCommandV1} from "../../⌨️command/🟦️.ts";

const partitionOf=(freeBytes:number|null)=>()=>freeBytes===null?undefined:{partition:"/sim",totalBytes:1073741824,freeBytes};

test("the reserve of the module is the authored one",()=>{expect(CAS_FULL_DISK_RESERVE_BYTES_V1).toBe(cases.reserveBytes);});

for(const row of cases.cases)test("first collection: "+row.name,async()=>{
 const home=mkdtempSync(join(tmpdir(),"casfull-")),calls:string[][]=[],lines:string[]=[];
 try{
  const ran=await reclaimFullDiskFirstV1(row.args,{workspace:home,environment:{HOME:home},home,write:line=>lines.push(line),partition:partitionOf(row.freeBytes),collect:async args=>{calls.push([...args]);return 0;}});
  expect(ran).toBe(row.collects);expect(calls.length).toBe(row.collects?1:0);
  if(row.collects)expect(calls[0]![0]).toBe("gc");
  expect(readdirSync(home)).toEqual([]);
 }finally{rmSync(home,{recursive:true,force:true});}
});

test("only a real collection by the command is a reclaim command",()=>{expect(casReclaimCommandV1(["cache","gc"])).toBe(true);expect(casReclaimCommandV1(["cache","cas","gc"])).toBe(true);expect(casReclaimCommandV1(["cache","cas","gc","--dry-run"])).toBe(false);expect(casReclaimCommandV1(["cache","status"])).toBe(false);});

test("a full disk collects the real host store with zero free bytes and writes nothing but the collection",async()=>{
 const home=mkdtempSync(join(tmpdir(),"casfull-")),lines:string[]=[];
 try{
  mkdirSync(join(home,".cache","semio","cas"),{recursive:true});
  const ran=await reclaimFullDiskFirstV1(["cache","cas","gc","--lang=en"],{workspace:home,environment:{HOME:home},home,write:line=>lines.push(line),partition:partitionOf(0)});
  expect(ran).toBe(true);expect(lines.join("\n")).toMatch(/cache|Cache|collect|Collect/u);
  expect(readdirSync(home)).toEqual([".cache"]);
 }finally{rmSync(home,{recursive:true,force:true});}
});

test("a failing collection is reported and left to the command",async()=>{
 const lines:string[]=[];
 const ran=await reclaimFullDiskFirstV1(["cache","cas","gc"],{workspace:"/x",environment:{HOME:"/x"},home:"/x",write:line=>lines.push(line),partition:partitionOf(0),collect:async()=>{throw Object.assign(Error("ENOSPC"),{code:"ENOSPC"});}});
 expect(ran).toBe(false);expect(lines).toEqual(["[cas] ENOSPC"]);
});

test("the cas command runs without its own port on the simulated full partition",async()=>{
 const home=mkdtempSync(join(tmpdir(),"casfull-")),lines:string[]=[];
 try{expect(await runCasCommandV1(["gc","--lang=en"],{workspace:home,environment:{HOME:home},home,write:line=>lines.push(line),partition:partitionOf(0)},{signal:new AbortController().signal,progress:async()=>undefined})).toBe(0);}
 finally{rmSync(home,{recursive:true,force:true});}
});
