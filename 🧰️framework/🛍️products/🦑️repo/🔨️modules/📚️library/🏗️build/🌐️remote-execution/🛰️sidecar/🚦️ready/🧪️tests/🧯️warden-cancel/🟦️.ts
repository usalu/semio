import{expect,test}from"bun:test";
import{join}from"node:path";
import{spawnSync}from"node:child_process";
import{READY_POLICY_V1 as P,READY_STATUS_SCHEMA_V1,readyRefusalV1}from"../../🟦️.ts";
import{readyWardenCancelOfStageV1}from"../../🌲️process-tree/🟦️.ts";
import{decideHostWardenV1,HOST_WARDEN_POLICY_V1,HOST_WARDEN_TABLES_V1}from"../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🟦️.ts";

/** 🧯️ Contract: a stage whose own process the host warden cancelled (SIGTERM at a memory danger) is told apart from a crash by the cancel record of the stage pid, fails with the bilingual code `memory-cancelled` and retries after the short back-off instead of the doubling one; the warden itself takes the stage owner only after every other candidate.
 * @see ../../🌲️process-tree/🟦️.ts
 * @see ./🐍️.py */
const oracle=(payload:unknown)=>{const run=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py"),JSON.stringify(payload)],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const row=(pid:number,atMs:number,kind="stage")=>({pid,atMs,kind,megabytes:71,reason:"danger" as const,ancestors:[1]});
const cases:{cancels:ReturnType<typeof row>[];pid:number}[]=[
 {cancels:[],pid:7},
 {cancels:[row(8,5)],pid:7},
 {cancels:[row(7,5)],pid:7},
 {cancels:[row(7,5),row(7,9),row(8,12)],pid:7},
 {cancels:[{...row(9,5,"slice-worker"),ancestors:[7]}],pid:7},
];

test("the cancel record of the stage pid is found exactly like the independent oracle finds it; a cancel of a worker below the stage is not the stage's",()=>{
 const expected=oracle(cases);
 cases.forEach((one,index)=>expect(readyWardenCancelOfStageV1(one.cancels,one.pid)?.atMs??null,JSON.stringify(one)).toBe(expected[index]));
 expect(expected).toEqual([null,null,5,9,null]);
});

test("memory-cancelled has a bilingual refusal text, keeps a stale failure on refresh and the back-off after it is the initial one",()=>{
 const refusal=readyRefusalV1("memory-cancelled",{stage:"workspace",ending:{en:"ended by signal SIGTERM",de:"durch das Signal SIGTERM beendet"}});
 expect(refusal.en).toContain("workspace");
 expect(refusal.en).toContain("memory warden");
 expect(refusal.de).toContain("Speicherwächter");
 expect(refusal.de).toContain("SIGTERM");
 expect(P.refreshKeepingFailureCodes).toContain("memory-cancelled");
 expect(READY_STATUS_SCHEMA_V1).toBeDefined();
});

test("at a severe danger the warden cancels the worker within its grant before the stage that owns the tree",()=>{
 const stage=HOST_WARDEN_TABLES_V1.kinds.find(kind=>kind.name==="stage")!;
 expect(stage.lastResort).toBe(true);
 const host={totalMegabytes:16384,availableMegabytes:300,swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,compressedMegabytes:0,pressure:"normal" as const,swapFreeMegabytes:null,swapVolumeFreeMegabytes:null};
 const worker={id:"p401",pid:401,kind:"slice-worker",rank:60,elapsedSeconds:900,megabytes:3665,suspended:true,suspendedSeconds:30,canSuspend:true,canCancel:true,nonresidentMegabytes:0,capMegabytes:6144,capCritical:true,grantMegabytes:3829};
 const owner={id:"p402",pid:402,kind:"stage",rank:90,elapsedSeconds:200,megabytes:71,suspended:false,suspendedSeconds:0,canSuspend:false,canCancel:true,lastResort:true};
 const decision=decideHostWardenV1(HOST_WARDEN_POLICY_V1,host,[owner,worker],{level:"danger",calmTicks:0});
 expect(decision.actions).toEqual([{action:"cancel",id:"p401",reason:"danger"}]);
});
