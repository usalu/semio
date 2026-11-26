/** 🧪️ Tests for compaction on idle: the plan against an independent Python model, the stamp rules with scripted ports, and a throwaway APFS image with two short-lived holders. */
import {expect,test} from "bun:test";
import {execFileSync,spawnSync} from "node:child_process";
import {lstatSync,mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync,unlinkSync} from "node:fs";
import {join} from "node:path";
import {testScratchRootV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🟦️.ts";
import {acquireResourceLease} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {createNativeImageDarwinReclaimPortV1,reclaimNativeImageV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/♻️reclaim/🟦️.ts";
import {compactOnIdleV1,planIdleCompactionV1,type IdleCompactionPortsV1} from "../🟦️.ts";

const HOUR=3600000;
test("the plan equals the independent Python model on every combination",()=>{
 const rows=[true,false].flatMap(due=>[undefined,0,HOUR-1,HOUR,5*HOUR].map(lastAt=>({partition:"p",due,lastAt}))),now=5*HOUR;
 const run=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py")],{input:JSON.stringify({rows,now,minimum:HOUR}),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 expect(planIdleCompactionV1(rows,now,HOUR).map(row=>row.verdict)).toEqual(JSON.parse(run.stdout));
});

const scripted=(over:Partial<IdleCompactionPortsV1>,log:string[]=[]):IdleCompactionPortsV1=>({now:()=>10*HOUR,due:()=>true,lastAt:()=>undefined,stamp:(partition,at)=>{log.push("stamp "+partition+" "+at);},reclaim:async partition=>{log.push("reclaim "+partition);return{outcome:"reclaimed",beforeBytes:10,afterBytes:4};},...over});

test("a compacted image is stamped, a busy or not due or too recent one is not touched and a failure never throws",async()=>{
 const log:string[]=[];
 const out=await compactOnIdleV1({partitions:["a","b","c","d","e"],minimumIntervalMilliseconds:HOUR,maximumMilliseconds:60000,ports:scripted({
  due:partition=>partition!=="b",lastAt:partition=>partition==="c"?10*HOUR-5:undefined,
  reclaim:async partition=>{log.push("reclaim "+partition);if(partition==="d")return{outcome:"skipped",reason:"lease-timeout"};if(partition==="e")throw Error("cycle failed");return{outcome:"reclaimed",beforeBytes:10,afterBytes:4};},
  stamp:(partition,at)=>{log.push("stamp "+partition+" "+at);}})});
 expect(out.map(row=>row.result)).toEqual(["compacted","not-due","too-soon","busy","failed"]);
 expect(log).toEqual(["reclaim a","stamp a "+10*HOUR,"reclaim d","reclaim e"]);
});

test("the pass stops at its bound and leaves the rest untouched",async()=>{
 const log:string[]=[];
 const out=await compactOnIdleV1({partitions:["a","b"],minimumIntervalMilliseconds:HOUR,maximumMilliseconds:50,ports:scripted({reclaim:async(partition,signal)=>{log.push("reclaim "+partition);await new Promise<void>(done=>signal.addEventListener("abort",()=>done(),{once:true}));return{outcome:"skipped",reason:"lease-timeout"};}},log)});
 expect(out.map(row=>row.result)).toEqual(["busy","busy"]);expect(log).toEqual(["reclaim a"]);
});

const tool=(program:string,args:string[])=>spawnSync(program,args,{encoding:"utf8"});
const detach=(root:string):void=>{tool("/usr/bin/hdiutil",["detach",root,"-force"]);};

test.skipIf(process.platform!=="darwin")("on a throwaway image two short-lived holders keep it busy, the last release compacts it, and an opener that arrives meanwhile waits and succeeds",async()=>{
 const directory=realpathSync(mkdtempSync(join(testScratchRootV1("idle"),"image-"))),image=join(directory,"fixed.dmg"),root=join(directory,"m"),leases=join(directory,"leases"),resource="native-fixed-image:"+directory;
 try{
  mkdirSync(root);mkdirSync(leases);
  expect(tool("/usr/bin/hdiutil",["create","-size","400m","-fs","APFS","-volname","idle",image]).status).toBe(0);
  const backing=image;
  expect(tool("/usr/sbin/diskutil",["image","attach","--nobrowse","--mountOptions","owners","--mountPoint",root,backing]).status).toBe(0);
  writeFileSync(join(root,"big"),Buffer.alloc(150*1048576,7));execFileSync("/bin/sync");unlinkSync(join(root,"big"));execFileSync("/bin/sync");
  const blocks=()=>lstatSync(backing).blocks*512,before=blocks();
  expect(before).toBeGreaterThan(100*1048576);
  const events:string[]=[];let opener:Promise<void>|undefined;
  const exec=(program:string,args:readonly string[])=>Promise.resolve(execFileSync(program,[...args],{encoding:"utf8",maxBuffer:67108864}));
  const ports:IdleCompactionPortsV1={now:Date.now,due:()=>true,lastAt:()=>undefined,stamp:()=>undefined,
   reclaim:async(_partition,signal)=>{
    let lease;
    try{lease=await acquireResourceLease({directory:leases,resource,mode:"exclusive",signal,timeoutMs:0});}catch{return{outcome:"skipped",reason:"lease-timeout"};}
    try{
     const port=createNativeImageDarwinReclaimPortV1({image:backing,cwd:directory,mountPoint:root,run:exec,cleanupRun:exec}),start=port.allocatedBytes();
     const result=await reclaimNativeImageV1(port,{check:()=>{},progress:done=>{if(done===1&&opener===undefined){events.push("opener asks");opener=acquireResourceLease({directory:leases,resource,mode:"shared",signal:new AbortController().signal}).then(held=>{events.push("opener holds");held.release();});}}});
     events.push("cycle ends");return{outcome:"reclaimed",beforeBytes:start,afterBytes:result.afterBytes};
    }finally{lease.release();events.push("exclusive released");}
   }};
  const first=await acquireResourceLease({directory:leases,resource,mode:"shared",signal:new AbortController().signal}),second=await acquireResourceLease({directory:leases,resource,mode:"shared",signal:new AbortController().signal});
  const busy=await compactOnIdleV1({partitions:["x"],ports,minimumIntervalMilliseconds:HOUR,maximumMilliseconds:120000});
  expect(busy[0]!.result).toBe("busy");expect(blocks()).toBe(before);expect(tool("/sbin/mount",[]).stdout).toContain(root);
  first.release();
  expect((await compactOnIdleV1({partitions:["x"],ports,minimumIntervalMilliseconds:HOUR,maximumMilliseconds:120000}))[0]!.result).toBe("busy");
  second.release();
  const done=await compactOnIdleV1({partitions:["x"],ports,minimumIntervalMilliseconds:HOUR,maximumMilliseconds:120000});
  expect(done[0]!.result,JSON.stringify(done)).toBe("compacted");
  await opener;
  expect(events.indexOf("opener asks")).toBeGreaterThanOrEqual(0);
  expect(events.indexOf("opener holds")).toBeGreaterThan(events.indexOf("exclusive released"));
  expect(blocks()).toBeLessThan(before-50*1048576);
 }finally{detach(root);rmSync(directory,{recursive:true,force:true});}
},240000);
