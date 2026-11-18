import{expect,test}from"bun:test";
import{chmodSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{wantedServerPolicyV1}from"../../../../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts";
import{wantedTextV1}from"../../../../../🪪️graph-state/🙋️wanted/🟦️.ts";
import{inferGraphReadV1}from"../../../🕸️graph/🟦️.ts";
import{describeGraphReadStallV1}from"../../../🕸️graph/🔬️stall/🟦️.ts";
import{measureReadTreeV1,memoryOverrunV1,parseFootprintsV1,parseProcessTableV1,readTreeRowsV1,summarizeReadTreeV1}from"../../../🕸️graph/🔬️stall/🌳️tree/🟦️.ts";

/** 🌳️ Contract: a read that prints nothing is explained by its process tree (running, idle and paging children, resident and physical memory against its grant), and a read whose tree holds far more memory than its grant while the host is under pressure is ended typed within minutes, not after the 25 minute wave limit. The table is the output of the stalled read of 2026-11-15 (pid 25177, grant 14336 MB, footprint 30.9 GB, children idle in a wait). */
const TABLE=[
 "25177 24695       45:36   0.1  69920 S    /w/.cache/b/t/m/bun/x/bun /w/script.ts repo show projects",
 "26138 25177       45:17  15.0 128032 Rs   /w/.cache/b/t/m/bun/x/bun --eval const fs=await import('node:fs')",
 "67777 26138       24:30   0.0  53536 S    /w/.cache/b/t/m/bun/x/bun --eval const fs=await import('node:fs')",
 "67779 26138       24:30   0.0 134128 S    /w/.cache/b/t/m/bun/x/bun --eval const fs=await import('node:fs')",
 "69694 26138    1-02:59:03   0.0  32144 Us   /w/.cache/b/t/m/bun/x/bun --eval const fs=await import('node:fs')",
 "  999     1 01-15:25:12   0.0  20608 Ss   /sbin/launchd",
 "bad line",
].join("\n");

test("the process table parses every row, the elapsed time in all its forms, and ignores lines that are not rows",()=>{
 const rows=parseProcessTableV1(TABLE);
 expect(rows.map(row=>row.pid)).toEqual([25177,26138,67777,67779,69694,999]);
 expect(rows[0]).toMatchObject({parentPid:24695,elapsedSeconds:45*60+36,cpu:0.1,rssMegabytes:Math.round(69920/1024),state:"S"});
 expect(rows[4]!.elapsedSeconds).toBe(86400+2*3600+59*60+3);
 expect(rows[5]!.elapsedSeconds).toBe(86400+15*3600+25*60+12);
 expect(rows[1]!.command).toContain("--eval");
});

test("the tree of a read is its root and every descendant, nothing else",()=>{
 expect(readTreeRowsV1(parseProcessTableV1(TABLE),25177).map(row=>row.pid).sort()).toEqual([25177,26138,67777,67779,69694]);
 expect(readTreeRowsV1(parseProcessTableV1(TABLE),26138).map(row=>row.pid).sort()).toEqual([26138,67777,67779,69694]);
 expect(readTreeRowsV1(parseProcessTableV1(TABLE),12345)).toEqual([]);
});

test("the summary counts running, idle (no CPU for a minute) and paging (uninterruptible) processes and takes the physical footprint over the resident size",()=>{
 const rows=readTreeRowsV1(parseProcessTableV1(TABLE),25177),plain=summarizeReadTreeV1(rows,undefined);
 expect(plain).toMatchObject({processes:5,running:1,idle:3,paging:1,footprintMegabytes:undefined});
 expect(plain.rssMegabytes).toBe([69920,128032,53536,134128,32144].reduce((sum,kb)=>sum+Math.round(kb/1024),0));
 const foot=summarizeReadTreeV1(rows,new Map([[26138,31641],[25177,50]]));
 expect(foot.footprintMegabytes).toBe(31691);expect(foot.largest).toEqual({pid:26138,megabytes:31641});
});

test("the footprint output of the system tool parses in kilobytes, megabytes and gigabytes per pid",()=>{
 const out="bun [26138]: 64-bit    Footprint: 30.9 GB (16384 bytes per page)\nbun [67779]: 64-bit    Footprint: 909 MB (16384 bytes per page)\nbun [1]: 64-bit    Footprint: 100 KB (16384 bytes per page)\nnoise";
 expect([...parseFootprintsV1(out)]).toEqual([[26138,31642],[67779,909],[1,0]]);
});

const policy=wantedServerPolicyV1;
test("memory overrun needs all three: far above the grant, a host under pressure, and a silent read; a read within its grant or a calm host is never ended",()=>{
 const base={footprintMegabytes:31000,grantMegabytes:14336,pressure:"warn",silentMilliseconds:policy.memoryOverrunMilliseconds,policy};
 expect(memoryOverrunV1(base)).toBe(true);
 expect(memoryOverrunV1({...base,footprintMegabytes:14336*policy.memoryOverrunFactor-1})).toBe(false);
 expect(memoryOverrunV1({...base,pressure:"normal"})).toBe(false);
 expect(memoryOverrunV1({...base,silentMilliseconds:policy.memoryOverrunMilliseconds-1})).toBe(false);
 expect(memoryOverrunV1({...base,grantMegabytes:undefined})).toBe(false);
 expect(memoryOverrunV1({...base,footprintMegabytes:undefined})).toBe(false);
 expect(memoryOverrunV1({...base,pressure:"critical"})).toBe(true);
});

const host={pressure:"warn",availableMegabytes:7953,totalMegabytes:32768};
test("the stall dump names the process tree and, when the read holds more than its grant, says it pages and waits for nothing",()=>{
 const tree=summarizeReadTreeV1(readTreeRowsV1(parseProcessTableV1(TABLE),25177),new Map([[26138,31641]]));
 const lines=describeGraphReadStallV1({rootPid:25177,entries:[],view:{host,grants:[{id:"Dxe",pid:25177,reservedMegabytes:14336}],waiters:[]},nowMs:1,silentMilliseconds:1332_000,lastOutput:"[repository:command] still running after 2561 s",language:"en",tree});
 expect(lines).toContain(wantedTextV1("stallTree","en",{processes:5,running:1,idle:3,paging:1,rss:tree.rssMegabytes,footprint:31641,pid:26138,largest:31641}));
 expect(lines).toContain(wantedTextV1("stallOverrun","en",{footprint:31641,grant:14336,times:"2.2"}));
 expect(lines.at(-1)).toBe(wantedTextV1("stallOutput","en",{last:"[repository:command] still running after 2561 s"}));
 const german=describeGraphReadStallV1({rootPid:25177,entries:[],view:{host,grants:[{id:"Dxe",pid:25177,reservedMegabytes:14336}],waiters:[]},nowMs:1,silentMilliseconds:1332_000,lastOutput:"",language:"de",tree});
 expect(german.some(line=>line.includes("31641")&&line!==lines.find(row=>row.includes("31641")))).toBe(true);
 const within=describeGraphReadStallV1({rootPid:25177,entries:[],view:{host,grants:[{id:"Dxe",pid:25177,reservedMegabytes:40000}],waiters:[]},nowMs:1,silentMilliseconds:1332_000,lastOutput:"",language:"en",tree});
 expect(within.some(line=>line===wantedTextV1("stallOverrun","en",{footprint:31641,grant:40000,times:"0.8"}))).toBe(false);
 expect(within.some(line=>line.includes("process tree"))).toBe(true);
});

const fakeRepo=(script:string)=>{const dir=mkdtempSync(join(tmpdir(),"graph-tree-")),path=join(dir,"repo");writeFileSync(path,"#!/bin/sh\n"+script+"\n");chmodSync(path,0o755);return{dir,path};};

test("a read that holds far more than its grant under pressure is stopped after the overrun limit with the typed reason, minutes before the wave limit; one within its grant runs on",async()=>{
 const{dir,path}=fakeRepo('echo "[repo] still working" >&2\nexec sleep 30');
 try{
  const small={...policy,inferStallMilliseconds:20_000,watchMilliseconds:30,memorySampleMilliseconds:30,memoryOverrunMilliseconds:200,stallDumpMilliseconds:600_000};
  const started=Date.now(),over=await inferGraphReadV1({command:path,cwd:tmpdir(),environment:{},reason:"request",signal:undefined,language:"en",policy:small,measure:async()=>({footprintMegabytes:31000,grantMegabytes:14336,pressure:"warn"})});
  expect(over.ok).toBe(false);expect(Date.now()-started).toBeLessThan(5000);
  expect(over.detail).toBe(wantedTextV1("memoryOverrun","en",{footprint:31000,grant:14336,minutes:"0.0"}));
  const calm=await inferGraphReadV1({command:path,cwd:tmpdir(),environment:{},reason:"request",signal:AbortSignal.timeout(900),language:"en",policy:small,measure:async()=>({footprintMegabytes:9000,grantMegabytes:14336,pressure:"warn"})});
  expect(calm.ok).toBe(false);expect(calm.detail,"a caller that stops waiting detaches from the read within its grant, it never stops it as an overrun").toBe("detached");
 }finally{rmSync(dir,{recursive:true,force:true});}
},30000);

test("the real process table sees this very process and a child it starts, with the footprint where the system says it",async()=>{
 const child=Bun.spawn(["sleep","5"]);
 try{
  const tree=await measureReadTreeV1(process.pid);
  expect(tree.processes).toBeGreaterThanOrEqual(2);expect(tree.rssMegabytes).toBeGreaterThan(0);expect(tree.largest!.megabytes).toBeGreaterThan(0);
  if(process.platform==="darwin")expect(tree.footprintMegabytes).toBeGreaterThan(0);
 }finally{child.kill();}
},30000);
