/** 🕳️ Contract: a volume that cannot be measured is not a full volume. A degenerate answer of the host (a row of zeros, NaN, negative, `null`) lets the stage start with one bilingual warning line (once per path and reason), a real shortage still collects and then refuses in a typed way, a filesystem without an inode table never produces an inode shortage, and a running stage is never judged by the measurement. The provider is the one free-space question of the build system, Windows-shaped fakes stand for the host. */
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{READY_POLICY_V1 as P,createReadyLeaseV1,emptyReadyStateV1,readyDigestsV1,readyDiskShortV1,readyDiskUnknownV1,readyDiskUnmeasuredV1,readyFilesystemFactsV1,runReadyV1,type ReadyFilesystemFactsV1,type ReadyLocalStateV1,type ReadyPortsV1,type ReadyRunResultV1,type ReadyStatusV1}from"../../🟦️.ts";

const scratch:string[]=[];afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const MB=1048576,PATH="C:\\semio\\.cache",TMP="C:\\Users\\deck\\AppData\\Local\\Temp";
type Table=Record<string,ReadyFilesystemFactsV1|null>;

function world(table:Table,options:Readonly<{stage?:string;inflight?:string;stopAfterSleeps?:number;external?:(round:number)=>void}>={}){
 const shared={state:emptyReadyStateV1() as ReadyLocalStateV1,marker:false,now:1_000_000},controller=new AbortController(),sleeps:number[]=[],said:string[]=[],updates:Partial<ReadyStatusV1>[]=[],calls:string[]=[];
 const digest=readyDigestsV1({platform:"windows",hash:path=>path==="bun.lock"?"a".repeat(64):"b".repeat(64)}),directory=mkdtempSync(join(tmpdir(),"ready-unknown-"));scratch.push(directory);
 const facts=(path:string)=>table[path]??null,paths=[PATH,TMP],stage=options.stage??"tools";
 const ports:ReadyPortsV1={
  server:false,now:()=>shared.now,jitter:0,cores:4,
  sleep:async(milliseconds,signal)=>{sleeps.push(milliseconds);shared.now+=milliseconds;options.external?.(sleeps.length);await new Promise<void>(done=>setTimeout(done,1));if(sleeps.length>=(options.stopAfterSleeps??6)||signal.aborted)controller.abort();},
  digests:async()=>digest,
  state:{read:()=>structuredClone(shared.state),write:state=>{shared.state=structuredClone(state);}},
  marker:{exists:()=>shared.marker,write:()=>{shared.marker=true;}},
  repoPresent:()=>true,human:()=>false,
  lease:createReadyLeaseV1({directory,owner:"one",now:()=>Date.now()}),
  memory:()=>({availableMegabytes:8192,ok:true}),
  ...options.inflight===undefined?{}:{inflight:()=>options.inflight},
  disk:name=>name===stage?readyDiskShortV1({stage:name,paths,facts}):null,
  diskUnknown:name=>name===stage?readyDiskUnknownV1({paths,facts}):[],
  run:async row=>{calls.push(row.name);return{exit:0,timedOut:false,spawnFailed:false,cancelled:false,stalled:false,detached:false,startedMs:shared.now,peakMegabytes:512,tail:[]} as ReadyRunResultV1;},
  status:{update:patch=>{updates.push(patch);}},say:line=>{said.push(line);},
 };
 return{ports,controller,sleeps,said,updates,calls};
}
const ZERO_UNKNOWN:ReadyFilesystemFactsV1={freeBytes:null,freeInodes:null,unknown:{reason:"empty-row",detail:"bsize 4096, blocks 0, bavail 0"}};

test("the Windows wall of g1-m05: a volume measured as nothing starts the stage with one bilingual warning, not a wait for 4096 MB that cannot come",async()=>{
 const w=world({[PATH]:ZERO_UNKNOWN,[TMP]:{freeBytes:80_000*MB,freeInodes:null}},{stopAfterSleeps:3});
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.calls).toContain("tools");
 expect(w.updates.some(update=>update.refusal?.code==="disk-short")).toBe(false);
 const warnings=w.said.filter(line=>line.includes(PATH)&&line.includes("empty-row"));
 expect(warnings).toHaveLength(1);
 const [en,de]=warnings[0]!.split(" / ");
 expect(en).toContain("could not be measured");expect(de).toContain("nicht gemessen");
 expect(warnings[0]).not.toContain("{");
 expect(readyDiskUnmeasuredV1("tools",{path:PATH,reason:"empty-row",detail:"x"}).code).toBe("disk-unmeasured");
});

test("every degenerate answer is unknown and never short: null, NaN, negative, infinite and a missing provider answer",()=>{
 const degenerate:ReadyFilesystemFactsV1[]=[ZERO_UNKNOWN,{freeBytes:null,freeInodes:null},{freeBytes:Number.NaN,freeInodes:null},{freeBytes:-5,freeInodes:null},{freeBytes:Number.NEGATIVE_INFINITY,freeInodes:null}];
 for(const fact of degenerate){
  expect(readyDiskShortV1({stage:"workspace",paths:[PATH],facts:()=>fact})).toBeNull();
  expect(readyDiskUnknownV1({paths:[PATH],facts:()=>fact})).toHaveLength(1);
 }
 expect(readyDiskShortV1({stage:"workspace",paths:[PATH],facts:()=>null})).toBeNull();
 expect(readyDiskUnknownV1({paths:[PATH],facts:()=>null})).toEqual([{path:PATH,reason:"unreadable",detail:"-"}]);
 expect(readyDiskUnknownV1({paths:[PATH],facts:()=>({freeBytes:80_000*MB,freeInodes:null})})).toEqual([]);
});

test("a real shortage is still a shortage: 500 MB free on a measured Windows volume waits with the typed refusal, and starts when the space returns",async()=>{
 const table:Table={[PATH]:{freeBytes:500*MB,freeInodes:null},[TMP]:{freeBytes:80_000*MB,freeInodes:null}};
 const w=world(table,{stopAfterSleeps:5,external:round=>{if(round===2)table[PATH]={freeBytes:9000*MB,freeInodes:null};}});
 await runReadyV1(w.ports,w.controller.signal);
 const refusal=w.updates.map(update=>update.refusal).find(row=>row?.code==="disk-short");
 expect(refusal).toBeDefined();expect(refusal!.en).toContain("only 500 MB");expect(refusal!.de).toContain("nur 500 MB");expect(refusal!.en).toContain(PATH);
 expect(w.said.some(line=>line.includes("could not be measured"))).toBe(false);
 expect(w.calls).toContain("tools");
});

test("a filesystem without an inode table never produces an inode shortage; a real exhausted inode table does",()=>{
 expect(readyDiskShortV1({stage:"tools",paths:[PATH],facts:()=>({freeBytes:80_000*MB,freeInodes:null})})).toBeNull();
 expect(readyDiskShortV1({stage:"tools",paths:[PATH],facts:()=>({freeBytes:80_000*MB,freeInodes:0})})).toMatchObject({resource:"inodes",available:0});
});

test("a stage that already runs is not warned about or judged by the measurement",async()=>{
 const w=world({[PATH]:ZERO_UNKNOWN,[TMP]:ZERO_UNKNOWN},{inflight:"tools",stopAfterSleeps:3});
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.said.some(line=>line.includes("could not be measured"))).toBe(false);
 expect(w.updates.some(update=>update.refusal?.code==="disk-short")).toBe(false);
});

test("the real provider on a Windows-shaped host: the volume call answers bytes without inodes, a zero row is unknown, a failing call is unknown with its reason",()=>{
 const directory=mkdtempSync(join(tmpdir(),"ready-real-"));scratch.push(directory);
 expect(readyFilesystemFactsV1(directory,{platform:"win32",windowsAvailableBytes:()=>64_000*MB})).toEqual({freeBytes:64_000*MB,freeInodes:null});
 expect(readyFilesystemFactsV1(directory,{read:()=>({bsize:4096,blocks:0,bavail:0,files:0,ffree:0})})).toMatchObject({freeBytes:null,freeInodes:null,unknown:{reason:"empty-row"}});
 expect(readyFilesystemFactsV1(directory,{platform:"win32",windowsAvailableBytes:()=>{throw Error("Host free space could not be read (Win32 error 5)");}})).toMatchObject({freeBytes:null,unknown:{reason:"host-call",detail:expect.stringContaining("Win32 error 5")}});
 const real=readyFilesystemFactsV1(directory);expect(real.freeBytes).not.toBeNull();expect(real.freeBytes!).toBeGreaterThan(0);
 expect(P.diskRequirements.tools.megabytes).toBeGreaterThan(0);
});
