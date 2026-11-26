/** 🪆️ Contract: a derived question that needs a TypeScript program or a derived walk is asked in a child process, one at a time, and never in the process that is already that child; the policy is closed under its schema; a child that cannot answer fails with a named code instead of hanging. */
import{expect,test}from"bun:test";
import{resolve}from"node:path";
import{isRepoErrorV1,renderRepoErrorV1}from"../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts";
import Ajv from"ajv";
import schema from"../🧬️schema/🔣️.json";
import policy from"../🔣️.json";
import{wantedPolicyV1}from"../../../../../🪪️graph-state/🙋️wanted/🟦️.ts";
import{soloGrantV1}from"../../../../../🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts";
import{HEAVY_QUESTION_POLICY_V1,askHeavyQuestionV1,heavyQuestionCapV1,heavyQuestionScaleV1,isHeavyQuestionV1,preflightToolClosuresInChildV1,primeSharedClosuresInChildV1,refusedToolClosuresV1}from"../🟨️.mjs";
import{mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{spawn}from"node:child_process";
import{fileURLToPath}from"node:url";
import{tmpdir}from"node:os";
import{join}from"node:path";

test("the policy follows its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(schema,policy),JSON.stringify(ajv.errors)).toBe(true);
 expect(HEAVY_QUESTION_POLICY_V1).toEqual(policy as never);
});

test("the cap of a child is the solo grant of the machine shared by the children, bound by free memory and reserve, between the declared floor and ceiling",()=>{
 const policy=HEAVY_QUESTION_POLICY_V1,cap=(hostMegabytes:number,availableMegabytes:number|null)=>heavyQuestionCapV1(policy,{hostMegabytes,availableMegabytes});
 const shared=(hostMegabytes:number,availableMegabytes:number)=>Math.floor(soloGrantV1({hostMegabytes,availableMegabytes},wantedPolicyV1).footprintMegabytes/policy.concurrentChildren);
 expect(policy.memoryFloorMegabytes).toBeLessThan(policy.memoryCeilingMegabytes);
 const middle=cap(16384,12000);
 expect(middle.megabytes).toBe(Math.min(policy.memoryCeilingMegabytes,Math.max(policy.memoryFloorMegabytes,shared(16384,12000))));
 expect(middle).toMatchObject({host:16384,available:12000,children:policy.concurrentChildren,floor:policy.memoryFloorMegabytes,ceiling:policy.memoryCeilingMegabytes});
 expect(cap(4096,500).megabytes,"a starved machine is admitted at the floor").toBe(policy.memoryFloorMegabytes);
 expect(cap(1048576,1000000).megabytes,"a huge machine is admitted at the ceiling").toBe(policy.memoryCeilingMegabytes);
 expect(cap(65536,40000).megabytes).toBeGreaterThan(cap(16384,12000).megabytes);
 const free=[3000,6000,9000,12000].map(available=>cap(32768,available).megabytes);
 expect(free,"less free memory never admits more").toEqual([...free].sort((a,b)=>a-b));expect(free[0]).toBeLessThan(free[3]!);
 const unreserved=cap(32768,12000);expect(unreserved.megabytes*policy.concurrentChildren,"the children together leave the reserve and the available floor").toBeLessThanOrEqual(12000);
 expect(cap(16384,null).megabytes).toBeGreaterThanOrEqual(policy.memoryFloorMegabytes);
});

test("programs and derived walks are heavy, command walks are not, and the child itself never routes again",()=>{
 const row=(question:unknown)=>({kind:"derived",path:JSON.stringify(question),fact:"x"});
 expect(isHeavyQuestionV1(row({program:1,workspaceRoot:"/w",projectRoot:"p",projects:[]}),{})).toBe(true);
 expect(isHeavyQuestionV1(row({derive:1,workspaceRoot:"/w",recipe:{}}),{})).toBe(true);
 expect(isHeavyQuestionV1(row({walk:1,roots:[],resources:[],selectors:[]}),{})).toBe(false);
 expect(isHeavyQuestionV1({path:"not json"},{})).toBe(false);
 expect(isHeavyQuestionV1(row({program:1}),{[HEAVY_QUESTION_POLICY_V1.environment]:String(heavyQuestionCapV1(HEAVY_QUESTION_POLICY_V1).megabytes)})).toBe(false);
});

test("a child that cannot answer fails with a bilingual refusal that names the exit status and the error line, and questions are asked one at a time",async()=>{
 const row={kind:"derived",path:JSON.stringify({derive:1,workspaceRoot:"/nonexistent-semio-root",recipe:{fn:"none"}}),fact:"x"};
 const outcomes=await Promise.allSettled([askHeavyQuestionV1(row,{workspaceRoot:"/nonexistent-semio-root"}),askHeavyQuestionV1(row,{workspaceRoot:"/nonexistent-semio-root"})]);
 for(const outcome of outcomes){
  expect(outcome.status).toBe("rejected");
  const reason=(outcome as PromiseRejectedResult).reason;
  expect(isRepoErrorV1(reason)).toBe(true);
  expect(reason.key).toBe("build.graph.heavy-question-failed");
  expect(reason.values.exit).toBe("1");expect(String(reason.values.detail).length).toBeGreaterThan(1);
  expect(renderRepoErrorV1(reason,"en")).toContain("exit 1");expect(renderRepoErrorV1(reason,"de")).toContain("Exit 1");
 }
},120000);

test("a refusal raised in the child reaches the parent with its code, key and values and renders in both languages (the lost message of the 17:11 failure)",async()=>{
 const root=resolve(import.meta.dir,"../".repeat(11)),row={kind:"derived",path:JSON.stringify({derive:1,workspaceRoot:root,recipe:{fn:"no-such-recipe"}}),fact:"x"};
 const reason=await askHeavyQuestionV1(row,{workspaceRoot:root}).then(()=>undefined,(error:unknown)=>error as any);
 expect(isRepoErrorV1(reason)).toBe(true);
 expect(reason.code).toBe("invalid");expect(reason.key).toBe("build.closure.derived-question-unknown");expect(String(reason.values.question)).toContain("no-such-recipe");
 const english=renderRepoErrorV1(reason,"en"),german=renderRepoErrorV1(reason,"de");
 expect(english).toContain("no-such-recipe");expect(german).toContain("no-such-recipe");expect(german).not.toBe(english);
},300000);

test("one refused tool closure is raised as itself and several as one refusal that names the count and every tool with its key and values, in both languages",()=>{
 const row=(name:string,targets:string)=>({name,version:1,code:"invalid",key:"build.closure.provider-generated-import",values:{port:name,targets}});
 const one=refusedToolClosuresV1([row("alpha","p:generate")]);
 expect(one.key).toBe("build.closure.provider-generated-import");expect(one.values).toEqual({port:"alpha",targets:"p:generate"});
 const many=refusedToolClosuresV1([row("alpha","p:generate"),row("beta","q:generate")]);
 expect(many.key).toBe("build.closure.tool-closures-refused");expect(many.values.count).toBe("2");
 for(const language of ["en","de"]){const text=renderRepoErrorV1(many,language);for(const part of ["alpha","beta","p:generate","q:generate","provider-generated-import"])expect(text).toContain(part);}
});

test("the preflight of the capped child accepts the real tree: every declared tool closure and provider port is derivable",async()=>{
 const root=resolve(import.meta.dir,"../".repeat(11)),answer=await preflightToolClosuresInChildV1({workspaceRoot:root});
 expect(answer.checked,"every declared tool closure and provider port was walked").toBeGreaterThan(0);
},600000);

const fakeChild=(directory:string,name:string,body:string)=>{const file=join(directory,name);writeFileSync(file,'const[,,root,output]=process.argv;const fs=require("node:fs");const beat=()=>process.stderr.write("\\u0001alive\\n");const gone=()=>fs.existsSync(root+"/go");const pause=ms=>Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,ms);'+body);return file;};
const FAST={...HEAVY_QUESTION_POLICY_V1,silenceMilliseconds:1500,livenessMilliseconds:1000,timeoutMilliseconds:60000},HOST={hostMegabytes:16384,availableMegabytes:10000},ONE={scale:()=>1,host:()=>HOST};
const QUESTION={kind:"derived",path:"{}",fact:"x"};
const until=async(condition:()=>boolean)=>{while(!condition())await new Promise(turn=>setTimeout(turn,5));};
const INERT={host:()=>HOST,now:()=>1e12,after:()=>()=>{},every:()=>()=>{}};

/** 🕰️ The manual time of a law: the clock and both schedulers of the watch of a child run on it, `advance` moves it and runs every due task in order, `reads` counts the readings made outside a task (the start of the watch and every line of life the child wrote), and `kills` logs each kill the watch sent to the child with the manual time of it. */
const manual=()=>{
 let at=0,ticking=false,reads=0;const tasks:{task:()=>void;every:number;next:number;live:boolean}[]=[],kills:{at:number;signal:string|undefined}[]=[];let armed:()=>void;const ready=new Promise<void>(resolveReady=>{armed=resolveReady;});
 const register=(task:()=>void,every:number,next:number)=>{const row={task,every,next,live:true};tasks.push(row);if(every>0)armed();return()=>{row.live=false;};};
 return{
  kills,ready,reads:()=>reads,at:()=>at,
  ports:{
   now:()=>{if(!ticking)reads++;return at;},
   after:(task:()=>void,milliseconds:number)=>register(task,0,at+milliseconds),
   every:(task:()=>void,milliseconds:number)=>register(task,milliseconds,at+milliseconds),
   start:((command:string,args:string[],options:object)=>{const child=spawn(command,args,options as never),kill=child.kill.bind(child);child.kill=(signal?:NodeJS.Signals|number)=>{kills.push({at,signal:signal as string|undefined});return kill(signal);};return child;}) as typeof spawn
  },
  advance(milliseconds:number){
   const target=at+milliseconds;
   for(;;){
    const due=tasks.filter(row=>row.live&&row.next<=target).sort((a,b)=>a.next-b.next)[0];
    if(due===undefined)break;
    at=due.next;ticking=true;try{due.task();}finally{ticking=false;}
    if(due.every>0)due.next+=due.every;else due.live=false;
   }
   at=target;
  }
 };
};
const lifeLines=(clock:ReturnType<typeof manual>,count:number)=>until(()=>clock.reads()>=2+count);

test("law: a child that writes nothing is stopped exactly when the silence window has passed, with a bilingual refusal, not before and not after the wall limit",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"heavy-silent-")),clock=manual();
 try{
  const script=fakeChild(directory,"silent.cjs","setInterval(()=>{},1000);");
  const asked=askHeavyQuestionV1(QUESTION,{workspaceRoot:directory,policy:FAST,ports:{...ONE,...clock.ports,cpuOf:()=>undefined},script}).then(()=>undefined,(error:unknown)=>error as any);
  await clock.ready;clock.advance(FAST.silenceMilliseconds-1);
  expect(clock.kills,"nothing before the window").toEqual([]);
  clock.advance(1);
  const reason=await asked;
  expect(clock.kills).toEqual([{at:FAST.silenceMilliseconds,signal:"SIGKILL"}]);
  expect(isRepoErrorV1(reason)).toBe(true);expect(reason.key).toBe("build.graph.heavy-question-silent");
  expect(renderRepoErrorV1(reason,"en")).toContain("no sign of life for 2 s");expect(renderRepoErrorV1(reason,"de")).toContain("kein Lebenszeichen");
 }finally{rmSync(directory,{recursive:true,force:true});}
},30000);

const outlive=async(name:string,preface:string,beatCode:string,fact:number)=>{
 const directory=mkdtempSync(join(tmpdir(),"heavy-"+name+"-")),clock=manual();
 try{
  const script=fakeChild(directory,name+".cjs",preface+"setInterval(()=>{"+beatCode+";if(gone()){fs.writeFileSync(output,JSON.stringify({fact:"+fact+",rows:[]}));process.exit(0);}},50);");
  const asked=askHeavyQuestionV1(QUESTION,{workspaceRoot:directory,policy:FAST,ports:{...ONE,...clock.ports,cpuOf:()=>undefined},script});
  await clock.ready;
  while(clock.at()<FAST.silenceMilliseconds*3){const seen=clock.reads();await until(()=>clock.reads()>seen);clock.advance(FAST.silenceMilliseconds/4);}
  expect(clock.kills,"a living child is never killed").toEqual([]);
  writeFileSync(join(directory,"go"),"");
  expect(await asked).toEqual({fact,rows:[]});
  expect(clock.at(),"it outlived the silence window").toBeGreaterThan(FAST.silenceMilliseconds*2);
 }finally{rmSync(directory,{recursive:true,force:true});}
};
test("law: a child that keeps its life line outlives the silence, and its answer is taken",()=>outlive("alive","","beat()",1),30000);

test("law: a child that computes without printing is not stopped, because liveness is progress and not output",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"heavy-busy-")),clock=manual();let cpu=0;
 try{
  const script=fakeChild(directory,"busy.cjs","while(!gone())pause(20);fs.writeFileSync(output,JSON.stringify({fact:3,rows:[]}));process.exit(0);");
  const asked=askHeavyQuestionV1(QUESTION,{workspaceRoot:directory,policy:FAST,ports:{...ONE,...clock.ports,cpuOf:()=>cpu+=1e6},script});
  await clock.ready;clock.advance(FAST.silenceMilliseconds*3);
  expect(clock.kills,"progress of the processor time is life").toEqual([]);
  writeFileSync(join(directory,"go"),"");
  expect(await asked).toEqual({fact:3,rows:[]});
 }finally{rmSync(directory,{recursive:true,force:true});}
},30000);
const REAL=fileURLToPath(new URL("../📜️script.ts",import.meta.url)),ROOT=resolve(import.meta.dir,"../".repeat(11));

test("law: a child that computes without printing is not stopped, because liveness is progress and not output",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"heavy-busy-"));
 try{
  const script=fakeChild(directory,"busy.cjs","const end=Date.now()+4500;while(Date.now()<end){}fs.writeFileSync(output,JSON.stringify({fact:3,rows:[]}));process.exit(0);");
  const began=Date.now();
  expect(await askHeavyQuestionV1(QUESTION,{workspaceRoot:directory,policy:FAST,ports:ONE,script})).toEqual({fact:3,rows:[]});
  expect(Date.now()-began,"it ran longer than the silence window").toBeGreaterThan(FAST.silenceMilliseconds*2);
 }finally{rmSync(directory,{recursive:true,force:true});}
},30000);

test("law: a child that is truly dead is stopped exactly at the bound derived from the machine, and the refusal names the question, the last phase and the silent seconds in both languages",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"heavy-dead-")),clock=manual(),window=FAST.silenceMilliseconds*2;
 try{
  const script=fakeChild(directory,"dead.cjs","process.stderr.write('\\u0001alive lease:store 0\\n');pause(600000);");
  const asked=askHeavyQuestionV1(QUESTION,{workspaceRoot:directory,policy:FAST,ports:{scale:()=>2,host:()=>HOST,...clock.ports,cpuOf:()=>1},script}).then(()=>undefined,(error:unknown)=>error as any);
  await clock.ready;await lifeLines(clock,1);
  clock.advance(window-1);
  expect(clock.kills,"the window is the authored silence times the scale: nothing before it").toEqual([]);
  clock.advance(1);
  const reason=await asked;
  expect(clock.kills).toEqual([{at:window,signal:"SIGKILL"}]);
  expect(reason.key).toBe("build.graph.heavy-question-silent");
  const english=renderRepoErrorV1(reason,"en"),german=renderRepoErrorV1(reason,"de");
  for(const text of[english,german]){expect(text).toContain("derived");expect(text).toContain("lease:store");expect(text).toContain("3 s");}
  expect(english).toContain("last phase");expect(german).toContain("letzte Phase");
 }finally{rmSync(directory,{recursive:true,force:true});}
},40000);

test("law: a child that waits on a named lease with its loop turning is waiting, not stalled, however long the wait outlasts the silence window",()=>outlive("lease","process.stderr.write('Process 7 (repo show) holds the dependency store since now; waiting for it\\n');","process.stderr.write('\\u0001alive lease:dependency-store 0\\n')",4),30000);

test("law: the window is the speed factor times the load per core, never below the authored time and never above the maximum, from an independent model",()=>{
 const small={SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"8192",SEMIO_HOST_CAPACITY_CORES:"4"},model=(load:number)=>Math.min(HEAVY_QUESTION_POLICY_V1.maximumScale,Math.ceil(Math.max(1,8/4)*Math.sqrt(16384/8192)*100-1e-9)/100*Math.max(1,load/4));
 for(const load of[0,2,4,8,12,100])expect(heavyQuestionScaleV1(HEAVY_QUESTION_POLICY_V1,small,load),"load "+load).toBeCloseTo(model(load),9);
 expect(heavyQuestionScaleV1(HEAVY_QUESTION_POLICY_V1,small,0)).toBeCloseTo(2.83,9);
 expect(heavyQuestionScaleV1(HEAVY_QUESTION_POLICY_V1,{},0)).toBeGreaterThanOrEqual(1);
 expect(heavyQuestionScaleV1(HEAVY_QUESTION_POLICY_V1,{},1e6)).toBe(HEAVY_QUESTION_POLICY_V1.maximumScale);
});

test("law: the capped child turns its event loop during the preflight, names its phase and reports the lag of each turn (the 60 s of silence came from a stretch of 12 to 20 s that wrote nothing)",async()=>{
 const child=spawn(process.execPath,[REAL,ROOT,join(tmpdir(),"heavy-beat-"+process.pid+".json")],{stdio:["pipe","ignore","pipe"],env:{...process.env,[HEAVY_QUESTION_POLICY_V1.environment]:String(heavyQuestionCapV1(HEAVY_QUESTION_POLICY_V1).megabytes)}});
 const beats:string[][]=[];let tail="";child.stderr.on("data",chunk=>{tail+=chunk;const lines=tail.split("\n");tail=lines.pop()!;for(const line of lines){const match=/^\u0001alive (\S+) (\d+)$/u.exec(line);if(match)beats.push([match[1]!,match[2]!]);}});
 child.stdin.end(JSON.stringify({preflight:true}));
 const code=await new Promise<number|null>(resolveClose=>child.once("close",resolveClose));
 expect(code).toBe(0);
 expect(beats.filter(([phase])=>/^preflight:\d+\/\d+$/u.test(phase!)).length,"heartbeats named a preflight step").toBeGreaterThanOrEqual(2);
 const steps=beats.map(([phase])=>/^preflight:(\d+)\/(\d+)$/u.exec(phase!)).filter((match):match is RegExpExecArray=>match!==null).map(match=>[Number(match[1]),Number(match[2])] as const);
 expect(new Set(steps.map(([,total])=>total)).size,"every beat names the same total").toBe(1);
 expect(steps.every(([done,total],at)=>done<=total&&(at===0||done>=steps[at-1]![0])),"the steps only advance").toBe(true);
 expect(Math.max(...steps.map(([done])=>done)),"the preflight progressed between its beats").toBeGreaterThan(Math.min(...steps.map(([done])=>done)));
},600000);

test("law: a child whose parent is killed ends within seconds although it is still computing (two real processes)",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"heavy-orphan-"));
 try{
  const parent=join(directory,"parent.cjs");
  writeFileSync(parent,'const{spawn}=require("node:child_process");const c=spawn(process.execPath,[process.argv[2],process.argv[3],process.argv[4]],{stdio:["pipe","ignore","pipe"]});c.stdin.end(JSON.stringify({preflight:true}));let said=false;c.stderr.on("data",d=>{if(!said&&String(d).includes("alive preflight")){said=true;console.log("CHILD "+c.pid);}});setInterval(()=>{},1000);');
  const holder=spawn(process.execPath,[parent,REAL,ROOT,join(directory,"out.json")],{stdio:["ignore","pipe","ignore"],env:{...process.env,[HEAVY_QUESTION_POLICY_V1.environment]:String(heavyQuestionCapV1(HEAVY_QUESTION_POLICY_V1).megabytes)}});
  const pid=await new Promise<number>((resolvePid,rejectPid)=>{let text="";holder.stdout.on("data",chunk=>{text+=chunk;const match=/CHILD (\d+)/u.exec(text);if(match)resolvePid(Number(match[1]));});holder.once("close",()=>rejectPid(new Error("the parent ended before the child said preflight")));});
  const lives=()=>{try{process.kill(pid,0);return true;}catch{return false;}};
  expect(lives(),"the child computes while its parent is alive").toBe(true);
  holder.kill("SIGKILL");
  await until(()=>!lives());
  expect(lives(),"the orphan outlived its parent").toBe(false);
 }finally{rmSync(directory,{recursive:true,force:true});}
},600000);

test("law: a long prime does not hold a heavy question back (two lanes), and two primes still run one after the other",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"heavy-lanes-"));
 try{
  const slow=fakeChild(directory,"slow.cjs","const started=Date.now();while(!gone())pause(20);fs.writeFileSync(output,JSON.stringify({started,done:Date.now()}));process.exit(0);");
  const quick=fakeChild(directory,"quick.cjs","fs.writeFileSync(output,JSON.stringify({fact:2,rows:[]}));process.exit(0);");
  let firstEnded=false;
  const prime=primeSharedClosuresInChildV1({workspaceRoot:directory,storeDirectory:directory,policy:FAST,ports:{...ONE,...INERT},script:slow}),second=primeSharedClosuresInChildV1({workspaceRoot:directory,storeDirectory:directory,policy:FAST,ports:{...ONE,...INERT},script:slow});
  void prime.then(()=>{firstEnded=true;});
  const question=await askHeavyQuestionV1(QUESTION,{workspaceRoot:directory,policy:FAST,ports:{...ONE,...INERT},script:quick});
  expect(question).toEqual({fact:2,rows:[]});expect(firstEnded,"the question did not wait for the prime").toBe(false);
  writeFileSync(join(directory,"go"),"");
  const first=await prime,next=await second;expect(Reflect.get(next,"started"),"the second prime started after the first ended").toBeGreaterThanOrEqual(Reflect.get(first,"done"));
 }finally{rmSync(directory,{recursive:true,force:true});}
},40000);

test("law: the child gets the cap admitted for this machine, and a child that outgrows it is refused in both languages with the numbers of the admission",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"heavy-cap-"));
 try{
  const expected=heavyQuestionCapV1(HEAVY_QUESTION_POLICY_V1,HOST);
  const echo=fakeChild(directory,"echo.cjs","fs.writeFileSync(output,JSON.stringify({fact:process.env."+HEAVY_QUESTION_POLICY_V1.environment+",rows:[]}));process.exit(0);");
  expect(await askHeavyQuestionV1(QUESTION,{workspaceRoot:directory,policy:FAST,ports:{...ONE,...INERT},script:echo})).toEqual({fact:String(expected.megabytes),rows:[]});
  const grown=fakeChild(directory,"grown.cjs","process.stderr.write('\\u0001alive question 0\\n');process.stderr.write('resident memory cap of "+expected.megabytes+" MB exceeded (resident "+(expected.megabytes+7)+" MB)\\n');process.exit(137);");
  const reason=await askHeavyQuestionV1(QUESTION,{workspaceRoot:directory,policy:FAST,ports:{...ONE,...INERT},script:grown}).then(()=>undefined,(error:unknown)=>error as any);
  expect(isRepoErrorV1(reason)).toBe(true);expect(reason.key).toBe("build.graph.heavy-question-memory");
  expect(reason.values).toMatchObject({limit:String(expected.megabytes),rss:String(expected.megabytes+7),host:"16384",available:"10000",children:String(expected.children),floor:String(expected.floor),ceiling:String(expected.ceiling),phase:"question"});
  for(const language of["en","de"]){const text=renderRepoErrorV1(reason,language);for(const part of[String(expected.megabytes),String(expected.megabytes+7),"16384","10000",String(expected.floor),String(expected.ceiling)])expect(text,language).toContain(part);}
  expect(renderRepoErrorV1(reason,"en")).toContain("resident memory cap");expect(renderRepoErrorV1(reason,"de")).toContain("Speicher");
 }finally{rmSync(directory,{recursive:true,force:true});}
},30000);

test("law: a child that ends by SIGKILL while the host warden holds a cancel record for its pid is refused as cancelled (bilingual, naming the kind and megabytes); without a record, with an old record or with a failing reader it stays the generic failure",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"heavy-cancelled-"));
 try{
  const script=fakeChild(directory,"killed.cjs","beat();process.kill(process.pid,'SIGKILL');setInterval(()=>{},1000);");
  const ask=(cancelOf:(pid:number)=>unknown)=>askHeavyQuestionV1(QUESTION,{workspaceRoot:directory,policy:FAST,ports:{...ONE,...INERT,cancelOf:cancelOf as never},script}).then(()=>undefined,(error:unknown)=>error as any);
  let seen=0;
  const cancelled=await ask(pid=>{seen=pid;return{version:1,pid,atMs:Date.now()+1000,kind:"heavy-question",megabytes:2300,reason:"danger"};});
  expect(seen).toBeGreaterThan(1);
  expect(isRepoErrorV1(cancelled)).toBe(true);expect(cancelled.code).toBe("failed");expect(cancelled.key).toBe("build.graph.heavy-question-cancelled");
  expect(cancelled.values).toMatchObject({pid:String(seen),kind:"heavy-question",megabytes:"2300",reason:"danger"});
  expect(renderRepoErrorV1(cancelled,"en")).toContain("host warden");expect(renderRepoErrorV1(cancelled,"de")).toContain("Host-Wächter");
  expect((await ask(()=>null)).key).toBe("build.graph.heavy-question-failed");
  expect((await ask(()=>({atMs:1,kind:"heavy-question",megabytes:1,reason:"danger"}))).key).toBe("build.graph.heavy-question-failed");
  expect((await ask(()=>{throw Error("unreadable");})).key).toBe("build.graph.heavy-question-failed");
 }finally{rmSync(directory,{recursive:true,force:true});}
},60000);
