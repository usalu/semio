import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {planSpillEvictionV1,pruneSpillV1,SPILL_POLICY_V1,spillBoundBytesV1,type SpillEntryV1,type SpillObjectV1} from "../🟦️.ts";
import {dropSpillObjectsV1,putSpillObjectV1,readSpillObjectV1} from "../../🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),schemaPath=join(here,"..","..","..","🧬️schema","🔣️.json"),minute=60_000,gib=2**30;
type Facts={totalBytes:number;availableBytes:number;spillBytes:number};type State={nowMs:number;boundBytes:number;objects:SpillObjectV1[];entries:SpillEntryV1[]};
const oracle=(bounds:readonly Facts[],plans:readonly State[]):{bounds:number[];plans:unknown[]}=>{const run=spawnSync("python3",["-I",join(here,"🐍️.py"),schemaPath],{input:JSON.stringify({bounds,plans}),encoding:"utf8",maxBuffer:1<<28});if(run.status!==0)throw Error(run.stderr);return JSON.parse(run.stdout);};
const name=(index:number)=>index.toString(16).padStart(64,"0");

test("the bound is the least of the maximum, the share and what keeps the volume free, and never negative",()=>{
 const total=30*gib,rule=SPILL_POLICY_V1;
 expect(spillBoundBytesV1({totalBytes:total,availableBytes:0,spillBytes:30*gib})).toBe(Math.min(rule.maximumBytes,Math.floor(total*rule.sharePercent/100),30*gib-Math.floor(total*rule.keepFreePercent/100)));
 expect(spillBoundBytesV1({totalBytes:total,availableBytes:0,spillBytes:gib})).toBe(0);
 expect(spillBoundBytesV1({totalBytes:total,availableBytes:total,spillBytes:0})).toBe(Math.min(rule.maximumBytes,Math.floor(total*rule.sharePercent/100)));
});

test("a plan drops unreferenced objects, empty entries and the least recently used entries, and keeps what was used lately",()=>{
 const now=10*60*minute,old=now-2*60*minute,objects:SpillObjectV1[]=[{sha256:name(1),bytes:100,usedMs:old},{sha256:name(2),bytes:100,usedMs:old+5},{sha256:name(3),bytes:100,usedMs:old+9},{sha256:name(4),bytes:100,usedMs:old},{sha256:name(5),bytes:100,usedMs:now-minute}];
 const entries:SpillEntryV1[]=[{key:"a",usedMs:old,objects:[name(1)]},{key:"b",usedMs:old+1,objects:[name(2),name(3)]},{key:"c",usedMs:old+2,objects:[name(3)]},{key:"d",usedMs:now-minute,objects:[name(5)]},{key:"e",usedMs:old,objects:[name(9)]}];
 const plan=planSpillEvictionV1({nowMs:now,boundBytes:200,objects,entries});
 expect(plan.objects).toEqual([name(1),name(2),name(4)]);expect(plan.entries).toEqual(["e","a","b"]);expect(plan.bytes).toBe(300);
 const all=planSpillEvictionV1({nowMs:now,boundBytes:0,objects,entries});
 expect(all.entries.sort()).toEqual(["a","b","c","e"]);expect(all.objects).not.toContain(name(5));expect(all.objects.sort()).toEqual([name(1),name(2),name(3),name(4)]);
});

test("500 seeded random states give the same bound and plan from the code and the independent Python oracle",()=>{
 let seed=20261127;const next=(limit:number)=>{seed=(seed*1103515245+12345)&0x7fffffff;return (seed>>>8)%limit;},bounds:Facts[]=[],states:State[]=[];
 for(let round=0;round<500;round++){
  bounds.push({totalBytes:next(64)*gib,availableBytes:next(40)*gib,spillBytes:next(30)*gib});
  const now=next(48)*minute+1_000*minute,objects=Array.from({length:next(12)},(_,index)=>({sha256:name(index),bytes:1+next(1000),usedMs:now-next(60)*minute})),entries=Array.from({length:next(8)},(_,index)=>({key:"k"+index,usedMs:now-next(60)*minute,objects:Array.from({length:next(4)},()=>name(next(14)))}));
  states.push({nowMs:now,boundBytes:next(4000),objects,entries});
 }
 const answer=oracle(bounds,states);let evicted=0;
 bounds.forEach((row,index)=>expect(spillBoundBytesV1(row),String(index)).toBe(answer.bounds[index]!));
 states.forEach((row,index)=>{const plan=planSpillEvictionV1(row);evicted+=plan.entries.length+plan.objects.length;expect<unknown>(plan,String(index)).toEqual(answer.plans[index]);});
 expect(evicted).toBeGreaterThan(500);
});

test("pruning a spill folder removes the least recently used entries beyond the bound, keeps the lately used ones and a read touches an object",()=>{
 const root=mkdtempSync(join(tmpdir(),"spill-bound-")),directory=join(root,"w","spill");
 try{
  const now=Date.now(),old=(now-3*3_600_000)/1000,entries=join(directory,"entries"),shas:string[]=[];mkdirSync(entries,{recursive:true});
  for(let index=0;index<5;index++){
   const sha=putSpillObjectV1(root,"x".repeat(1000)+index);shas.push(sha);
   writeFileSync(join(entries,"k"+index+".json"),JSON.stringify({outputs:[{sha256:sha}],stdout:null,stderr:null}));
   const when=index===4?now/1000:old+index;utimesSync(join(directory,sha),when,when);utimesSync(join(entries,"k"+index+".json"),when,when);
  }
  writeFileSync(join(directory,"left.tmp"),"stale");utimesSync(join(directory,"left.tmp"),old,old);
  const result=pruneSpillV1(directory,{boundBytes:2100,nowMs:now});
  expect(result.removedEntries).toBe(3);expect(result.keptBytes).toBe(2*1001);expect(existsSync(join(directory,"left.tmp"))).toBe(false);
  expect(readdirSync(entries).sort()).toEqual(["k3.json","k4.json"]);expect(existsSync(join(directory,shas[4]!))).toBe(true);expect(existsSync(join(directory,shas[0]!))).toBe(false);
  const before=Date.now()-3_000_000;utimesSync(join(directory,shas[3]!),before/1000,before/1000);
  expect(readSpillObjectV1(root,shas[3]!)).toBeDefined();
  expect(pruneSpillV1(directory,{boundBytes:0,nowMs:Date.now()}).removedEntries,"read and fresh entries are kept").toBe(0);
  expect(pruneSpillV1(join(root,"absent"),{}).removedEntries).toBe(0);
  dropSpillObjectsV1(root,{key:"k3",outputs:[{sha256:shas[3]!}]});expect(readdirSync(entries)).toEqual(["k4.json"]);
 }finally{rmSync(root,{recursive:true,force:true});}
});
