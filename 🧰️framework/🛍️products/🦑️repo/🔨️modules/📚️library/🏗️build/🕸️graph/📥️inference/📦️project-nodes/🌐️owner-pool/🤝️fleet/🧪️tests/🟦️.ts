import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{ownerPoolChildLimitsV1}from"../../🟦️.ts";
import{OWNER_POOL_FLEET_POLICY_V1 as F,ownerPoolMachineOfV1,ownerPoolNoProducerTextV1,ownerPoolProducersV1,ownerPoolReadFleetV1,ownerPoolTurnMillisecondsV1,type OwnerPoolMachineV1}from"../🟦️.ts";

/** 🤝️ The fleet acts as one computer for an owner one machine cannot hold: the machines that can hold it are known from their pool statuses, the largest free one claims first, the next ones in turn, and when none fits the refusal names the sizes of the fleet. */
const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const now=1_000_000_000;
const machine=(origin:string,gigabytes:number,rss=0,age=1000,state="pulling"):OwnerPoolMachineV1=>({origin,totalMegabytes:gigabytes*1024,grantMegabytes:ownerPoolChildLimitsV1(gigabytes*1024).hardMegabytes,state,rssMegabytes:rss,updatedMs:now-age});
const fleet=[machine("m8",8),machine("m16",16,1500),machine("m32",32,6000)];

test("only machines whose child grant holds the owner produce it, the largest free grant first",()=>{
 expect(fleet.map(row=>row.grantMegabytes)).toEqual([3072,4915,9830]);
 expect(ownerPoolProducersV1(fleet,4842,now).map(row=>row.origin)).toEqual(["m32","m16"]);
 expect(ownerPoolProducersV1(fleet,2000,now).map(row=>row.origin)).toEqual(["m32","m16","m8"]);
 expect(ownerPoolProducersV1(fleet,12000,now)).toEqual([]);
});

test("a machine that is busy with a big child ranks behind a free machine of the same size",()=>{
 const pair=[machine("b",16,4000),machine("a",16,100)];
 expect(ownerPoolProducersV1(pair,4000,now).map(row=>row.origin)).toEqual(["a","b"]);
});

test("a status older than the alive bound, an off pool and a too small pool are no producers",()=>{
 const rows=[machine("old",32,0,F.aliveMilliseconds+1),machine("off",32,0,1000,"off"),machine("small",32,0,1000,"too-small"),machine("ok",16)];
 expect(ownerPoolProducersV1(rows,4000,now).map(row=>row.origin)).toEqual(["ok"]);
});

test("the turn of a machine is its rank times the turn bound, and a machine that cannot hold the owner has none",()=>{
 expect(ownerPoolTurnMillisecondsV1("m32",fleet,4842,now)).toBe(0);
 expect(ownerPoolTurnMillisecondsV1("m16",fleet,4842,now)).toBe(F.turnMilliseconds);
 expect(ownerPoolTurnMillisecondsV1("m8",fleet,4842,now)).toBeUndefined();
 expect(ownerPoolTurnMillisecondsV1("m8",fleet,2000,now)).toBe(2*F.turnMilliseconds);
});

test("when no producer fits the refusal names the sizes of the fleet in both languages, and an empty fleet says so",()=>{
 const text=ownerPoolNoProducerTextV1(fleet,12000,now);
 expect(text).toContain("12000 MB");
 expect(text).toContain("1 x 32 GB (one owner up to 9830 MB)");
 expect(text).toContain("1 x 8 GB (one owner up to 3072 MB)");
 expect(text).toContain("keine Maschine der Flotte");
 expect(text).toContain("1 x 16 GB (ein Owner bis 4915 MB)");
 expect(ownerPoolNoProducerTextV1([],5000,now)).toContain("no machine reports a pool");
});

test("a status document is a machine only with its size; the store listing skips what is broken",()=>{
 const good={version:1,role:"puller",origin:"g",state:"pulling",totalMegabytes:16384,grantMegabytes:4915,rssMegabytes:10,updatedMs:5};
 expect(ownerPoolMachineOfV1(good)?.grantMegabytes).toBe(4915);
 expect(ownerPoolMachineOfV1({...good,totalMegabytes:undefined})).toBeUndefined();
 expect(ownerPoolMachineOfV1({...good,role:"publisher"})).toBeUndefined();
 expect(ownerPoolMachineOfV1(null)).toBeUndefined();
 const store=mkdtempSync(join(tmpdir(),"fleet-view-"));scratch.push(store);
 for(const [origin,text] of [["a",JSON.stringify({...good,origin:"a"})],["b","{broken"],["c",JSON.stringify({...good,origin:"c",totalMegabytes:32768,grantMegabytes:9830})]]){const directory=join(store,"v1","origins",origin);mkdirSync(directory,{recursive:true});writeFileSync(join(directory,"pool.json"),text);}
 mkdirSync(join(store,"v1","origins","d"),{recursive:true});
 expect(ownerPoolReadFleetV1(store).map(row=>row.origin).sort()).toEqual(["a","c"]);
 expect(ownerPoolReadFleetV1(undefined)).toEqual([]);
 expect(ownerPoolReadFleetV1(join(store,"missing"))).toEqual([]);
});
