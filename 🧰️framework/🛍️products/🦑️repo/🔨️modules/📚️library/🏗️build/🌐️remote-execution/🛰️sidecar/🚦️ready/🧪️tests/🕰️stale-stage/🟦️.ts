import{afterAll,expect,test}from"bun:test";
import{chmodSync,mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{READY_TREE_POLICY_V1 as TREE,readyScaledTreePolicyV1,readyStaleRestartV1,readyStagePhaseV1,readyStaleStopV1}from"../../🌲️process-tree/🟦️.ts";
import{READY_STAGES_V1 as STAGES,runReadyStageV1}from"../../🟦️.ts";
import treeSchema from"../../🌲️process-tree/🧬️schema/🔣️.json"with{type:"json"};

const scratch:string[]=[];afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`stale-${label}-`));scratch.push(path);return path;};
const posix=process.platform==="win32"?test.skip:test;
const fakeHome=(script:string)=>{const home=temp("home"),bin=join(home,".local","bin");mkdirSync(bin,{recursive:true});writeFileSync(join(bin,"repo"),script);chmodSync(join(bin,"repo"),0o755);return home;};
const stageOf=(name:string,timeoutMilliseconds:number)=>({...STAGES.find(stage=>stage.name===name)!,timeoutMilliseconds});
const working=(seconds:number)=>`#!/bin/sh\ni=0\nwhile [ $i -lt ${seconds} ]; do i=$((i+1)); echo "Reading the project graph slice ($i s): taken $i of 4455."; sleep 0.3; done\nexit 0\n`;
const executing=(reads:number,tasks:number)=>`#!/bin/sh\ni=0\nwhile [ $i -lt ${reads} ]; do i=$((i+1)); echo "Reading the project graph slice ($i s): taken $i of 590."; sleep 0.3; done\necho "NX   Running target build for project x and 2 tasks they depend on:"\nj=0\nwhile [ $j -lt ${tasks} ]; do j=$((j+1)); echo "built unit $j"; sleep 0.3; done\nexit 0\n`;
const silent=(seconds:number)=>`#!/bin/sh\ni=0\nwhile [ $i -lt ${seconds} ]; do i=$((i+1)); echo "working on unit $i"; sleep 0.3; done\nexit 0\n`;
const policy={...TREE,staleStageMilliseconds:1500,codeCheckMilliseconds:300,codeRoots:["code"],holderWaitRestartMilliseconds:600000,stallStopMilliseconds:600000};
const touch=(root:string,name:string,ahead:number)=>{mkdirSync(dirname(join(root,"code",name)),{recursive:true});const at=new Date(Date.now()+ahead);writeFileSync(join(root,"code",name),"y");utimesSync(join(root,"code",name),at,at);};
const run=(home:string,root:string)=>runReadyStageV1({home,root,environment:{},stage:stageOf("tools",600000),signal:new AbortController().signal,jobs:1,onLine:()=>undefined,tree:policy});

test("a stage that shows progress is stopped for new code only when it has run long enough",()=>{
 expect(TREE.staleStageMilliseconds).toBe(1800000);
 expect(readyStaleRestartV1(1799999,true)).toBe(false);expect(readyStaleRestartV1(1800000,true)).toBe(true);expect(readyStaleRestartV1(86400000,false)).toBe(false);
 expect(readyScaledTreePolicyV1(2.83).staleStageMilliseconds).toBe(Math.ceil(1800000*2.83));expect(readyScaledTreePolicyV1(1).staleStageMilliseconds).toBe(1800000);
});

test("the code a stage loaded includes the build library, whose landings the workers must follow",()=>{
 expect(TREE.codeRoots).toContain("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build");
 expect(TREE.codeSkipDirectories).toContain("🧪️tests");
 expect(treeSchema.definitions.Policy.const.codeRoots).toEqual([...TREE.codeRoots]);
});

posix("a stage with progress is stopped with its tree when a source file changed after it started and it has run for the stale age",async()=>{
 const root=temp("root"),home=fakeHome(working(60)),began=Date.now(),pending=run(home,root);
 setTimeout(()=>touch(root,"b.ts",5000),700);
 const result=await pending;
 expect(result.stalled).toBe(true);expect(result.codeChanged).toBe(join("code","b.ts"));expect(result.codeStale).toBe(true);expect(result.timedOut).toBe(false);
 expect(Date.now()-began).toBeGreaterThan(1500);expect(Date.now()-began).toBeLessThan(25000);
},60000);

posix("a young stage is not interrupted by a landing, and an old stage on unchanged code goes on",async()=>{
 const youngRoot=temp("young"),young=fakeHome(working(4)),pendingYoung=run(young,youngRoot);
 setTimeout(()=>touch(youngRoot,"a.ts",5000),200);
 const early=await pendingYoung;
 expect(early.exit).toBe(0);expect(early.stalled).toBe(false);expect(early.codeStale).toBeUndefined();
 const quietRoot=temp("quiet"),quiet=fakeHome(working(9));mkdirSync(join(quietRoot,"code"),{recursive:true});writeFileSync(join(quietRoot,"code","a.ts"),"x");utimesSync(join(quietRoot,"code","a.ts"),new Date(Date.now()-600000),new Date(Date.now()-600000));
 const unchanged=await run(quiet,quietRoot);
 expect(unchanged.exit).toBe(0);expect(unchanged.stalled).toBe(false);expect(unchanged.codeChanged).toBeUndefined();
},60000);

test("the refusal of a stale stage is declared in both languages",async()=>{
 const schema=(await import("../../🧬️schema/🔣️.json",{with:{type:"json"}})).default as any;
 const messages=JSON.stringify(schema);
 expect(messages).toContain("stage-code-stale");
 const text=(language:string)=>JSON.stringify(Object.values(schema.definitions).flatMap((definition:any)=>definition?.const?.[language]?.["stage-code-stale"]??[]));
 for(const language of["en","de"]){const row=JSON.parse(text(language))[0] as string;expect(row).toContain("{stage}");expect(row).toContain("{minutes}");expect(row).toContain("{file}");}
 expect(JSON.parse(text("de"))[0]).toContain("neuen Code");expect(JSON.parse(text("en"))[0]).toContain("new code");
});

const LOG={reading:"Reading the project graph slice (4312 s): owners 111 of 703.",german:"Lese den Projektgraph-Ausschnitt (12 s).",nx:"NX   Running target build for project @semio-tech/x and 7 tasks they depend on:",task:"> nx run @semio-tech/x:build",answered:"graph: plan answered, managed 107 MB, resident 245 MB (released 0 MB)",noise:"warning: something"};

test("the phase of a stage is what its latest marker line says and stays when no line says anything",()=>{
 expect(readyStagePhaseV1([LOG.noise],null)).toBeNull();
 expect(readyStagePhaseV1([LOG.reading,LOG.noise],null)).toBe("reading");expect(readyStagePhaseV1([LOG.german],null)).toBe("reading");
 expect(readyStagePhaseV1([LOG.reading,LOG.nx,LOG.noise],"reading")).toBe("executing");expect(readyStagePhaseV1([LOG.answered],null)).toBe("executing");expect(readyStagePhaseV1([LOG.task],"reading")).toBe("executing");
 expect(readyStagePhaseV1([LOG.nx,LOG.reading],"executing")).toBe("reading");
 expect(readyStagePhaseV1([LOG.noise],"executing")).toBe("executing");
});

test("law: executing never stops for stale code, reading and unknown stop after the stale age",()=>{
 const age=TREE.staleStageMilliseconds;
 expect(readyStaleStopV1("executing",age*100)).toBe(false);
 expect(readyStaleStopV1("reading",age-1)).toBe(false);expect(readyStaleStopV1("reading",age)).toBe(true);
 expect(readyStaleStopV1(null,age-1)).toBe(false);expect(readyStaleStopV1(null,age)).toBe(true);
 expect(readyStaleStopV1("reading",readyScaledTreePolicyV1(2).staleStageMilliseconds-1,readyScaledTreePolicyV1(2))).toBe(false);
});

test("the loaded closure of a stage leaves out tests and testing files",()=>{expect(TREE.codeSkipDirectories).toContain("🧪️testing");for(const pattern of[...TREE.readingPatterns,...TREE.executingPatterns])expect(()=>new RegExp(pattern,"u")).not.toThrow();});

posix("a stage that executes planned tasks is not stopped when the code changes, it runs to its end",async()=>{
 const root=temp("exec"),home=fakeHome(executing(2,14)),pending=run(home,root);
 setTimeout(()=>touch(root,"b.ts",5000),700);
 const result=await pending;
 expect(result.exit).toBe(0);expect(result.stalled).toBe(false);expect(result.codeStale).toBeUndefined();expect(result.codeChanged).toBeUndefined();
},60000);

posix("a stage that reads and whose loaded closure changed is stopped as stale",async()=>{
 const root=temp("read"),home=fakeHome(working(60)),pending=run(home,root);
 setTimeout(()=>touch(root,"loaded.mjs",5000),700);
 const result=await pending;
 expect(result.codeStale).toBe(true);expect(result.codeChanged).toBe(join("code","loaded.mjs"));
},60000);

posix("a stage that reads is not stopped by a change of a test or a testing file",async()=>{
 const root=temp("tests"),home=fakeHome(working(9)),pending=run(home,root);
 setTimeout(()=>{touch(root,join("🧪️tests","a.ts"),5000);touch(root,join("🧪️testing","b.ts"),5000);},700);
 const result=await pending;
 expect(result.exit).toBe(0);expect(result.stalled).toBe(false);expect(result.codeChanged).toBeUndefined();
},60000);

posix("a stage that never reports a phase keeps the old rule",async()=>{
 const root=temp("silent"),home=fakeHome(silent(60)),pending=run(home,root);
 setTimeout(()=>touch(root,"c.ts",5000),700);
 const result=await pending;
 expect(result.codeStale).toBe(true);expect(result.stalled).toBe(true);
},60000);

posix("a second read after the tasks counts from its own start",async()=>{
 const root=temp("again"),script=`#!/bin/sh\necho "NX   Running target build for project x and 2 tasks they depend on:"\ni=0\nwhile [ $i -lt 11 ]; do i=$((i+1)); echo "built unit $i"; sleep 0.3; done\nj=0\nwhile [ $j -lt 80 ]; do j=$((j+1)); echo "Reading the project graph slice ($j s): taken $j of 590."; sleep 0.3; done\n`,home=fakeHome(script),pending=run(home,root);
 setTimeout(()=>touch(root,"early.ts",0),500);
 setTimeout(()=>touch(root,"late.ts",5000),4500);
 const result=await pending;
 expect(result.codeStale).toBe(true);expect(result.codeChanged).toBe(join("code","late.ts"));
},60000);

posix("the marker of the tasks is seen although the stage prints hundreds of lines before the next look",async()=>{
 const root=temp("flood"),script=`#!/bin/sh\necho "NX   Running target build for project x and 2 tasks they depend on:"\nseq 1 600 | sed 's/^/compiled crate /'\nj=0\nwhile [ $j -lt 14 ]; do j=$((j+1)); echo "built unit $j"; sleep 0.3; done\nexit 0\n`,home=fakeHome(script),pending=run(home,root);
 setTimeout(()=>touch(root,"b.ts",5000),700);
 const result=await pending;
 expect(result.exit).toBe(0);expect(result.stalled).toBe(false);expect(result.codeChanged).toBeUndefined();
},60000);
