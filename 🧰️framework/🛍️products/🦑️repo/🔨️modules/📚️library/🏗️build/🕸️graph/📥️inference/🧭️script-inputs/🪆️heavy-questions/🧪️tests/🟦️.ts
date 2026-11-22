/** 🪆️ Contract: a derived question that needs a TypeScript program or a derived walk is asked in a child process, one at a time, and never in the process that is already that child; the policy is closed under its schema; a child that cannot answer fails with a named code instead of hanging. */
import{expect,test}from"bun:test";
import{resolve}from"node:path";
import{isRepoErrorV1,renderRepoErrorV1}from"../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts";
import Ajv from"ajv";
import schema from"../🧬️schema/🔣️.json";
import policy from"../🔣️.json";
import{HEAVY_QUESTION_POLICY_V1,askHeavyQuestionV1,heavyQuestionScaleV1,isHeavyQuestionV1,preflightToolClosuresInChildV1,primeSharedClosuresInChildV1,refusedToolClosuresV1}from"../🟨️.mjs";
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

test("programs and derived walks are heavy, command walks are not, and the child itself never routes again",()=>{
 const row=(question:unknown)=>({kind:"derived",path:JSON.stringify(question),fact:"x"});
 expect(isHeavyQuestionV1(row({program:1,workspaceRoot:"/w",projectRoot:"p",projects:[]}),{})).toBe(true);
 expect(isHeavyQuestionV1(row({derive:1,workspaceRoot:"/w",recipe:{}}),{})).toBe(true);
 expect(isHeavyQuestionV1(row({walk:1,roots:[],resources:[],selectors:[]}),{})).toBe(false);
 expect(isHeavyQuestionV1({path:"not json"},{})).toBe(false);
 expect(isHeavyQuestionV1(row({program:1}),{[HEAVY_QUESTION_POLICY_V1.environment]:"2560"})).toBe(false);
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
 expect(answer.checked).toBeGreaterThan(30);
},600000);

const fakeChild=(directory:string,name:string,body:string)=>{const file=join(directory,name);writeFileSync(file,'const[,,root,output]=process.argv;const fs=require("node:fs");const beat=()=>process.stderr.write("\\u0001alive\\n");'+body);return file;};
const FAST={...HEAVY_QUESTION_POLICY_V1,silenceMilliseconds:1500,livenessMilliseconds:1000,timeoutMilliseconds:60000},ONE={scale:()=>1};
const QUESTION={kind:"derived",path:"{}",fact:"x"};

test("law: a child that writes nothing is stopped after the silence, with a bilingual refusal, not after the wall limit",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"heavy-silent-"));
 try{
  const script=fakeChild(directory,"silent.cjs","setInterval(()=>{},1000);");
  const began=Date.now(),reason=await askHeavyQuestionV1(QUESTION,{workspaceRoot:directory,policy:FAST,ports:ONE,script}).then(()=>undefined,(error:unknown)=>error as any);
  expect(Date.now()-began).toBeLessThan(10000);
  expect(isRepoErrorV1(reason)).toBe(true);expect(reason.key).toBe("build.graph.heavy-question-silent");
  expect(renderRepoErrorV1(reason,"en")).toContain("no sign of life for 2 s");expect(renderRepoErrorV1(reason,"de")).toContain("kein Lebenszeichen");
 }finally{rmSync(directory,{recursive:true,force:true});}
},30000);

test("law: a child that keeps its life line outlives the silence, and its answer is taken",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"heavy-alive-"));
 try{
  const script=fakeChild(directory,"alive.cjs","const timer=setInterval(beat,300);setTimeout(()=>{clearInterval(timer);fs.writeFileSync(output,JSON.stringify({fact:1,rows:[]}));process.exit(0);},4000);");
  expect(await askHeavyQuestionV1(QUESTION,{workspaceRoot:directory,policy:FAST,ports:ONE,script})).toEqual({fact:1,rows:[]});
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

test("law: a child that is truly dead is stopped within the bound derived from the machine, and the refusal names the question, the last phase and the silent seconds in both languages",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"heavy-dead-"));
 try{
  const script=fakeChild(directory,"dead.cjs","process.stderr.write('\\u0001alive lease:store 0\\n');setTimeout(()=>Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0),100);setInterval(()=>{},1000);");
  const began=Date.now(),reason=await askHeavyQuestionV1(QUESTION,{workspaceRoot:directory,policy:FAST,ports:{scale:()=>2},script}).then(()=>undefined,(error:unknown)=>error as any);
  const took=Date.now()-began;
  expect(took,"the window is the authored silence times the scale").toBeGreaterThanOrEqual(2900);expect(took).toBeLessThan(15000);
  expect(reason.key).toBe("build.graph.heavy-question-silent");
  const english=renderRepoErrorV1(reason,"en"),german=renderRepoErrorV1(reason,"de");
  for(const text of[english,german]){expect(text).toContain("derived");expect(text).toContain("lease:store");expect(text).toMatch(/[34] s/u);}
  expect(english).toContain("last phase");expect(german).toContain("letzte Phase");
 }finally{rmSync(directory,{recursive:true,force:true});}
},40000);

test("law: a child that waits on a named lease with its loop turning is waiting, not stalled, however long the wait outlasts the silence window",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"heavy-lease-"));
 try{
  const script=fakeChild(directory,"lease.cjs","process.stderr.write('Process 7 (repo show) holds the dependency store since now; waiting for it\\n');const timer=setInterval(()=>process.stderr.write('\\u0001alive lease:dependency-store 0\\n'),300);setTimeout(()=>{clearInterval(timer);fs.writeFileSync(output,JSON.stringify({fact:4,rows:[]}));process.exit(0);},4800);");
  expect(await askHeavyQuestionV1(QUESTION,{workspaceRoot:directory,policy:FAST,ports:ONE,script})).toEqual({fact:4,rows:[]});
 }finally{rmSync(directory,{recursive:true,force:true});}
},30000);

test("law: the window is the speed factor times the load per core, never below the authored time and never above the maximum, from an independent model",()=>{
 const small={SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"8192",SEMIO_HOST_CAPACITY_CORES:"4"},model=(load:number)=>Math.min(HEAVY_QUESTION_POLICY_V1.maximumScale,Math.ceil(Math.max(1,8/4)*Math.sqrt(16384/8192)*100-1e-9)/100*Math.max(1,load/4));
 for(const load of[0,2,4,8,12,100])expect(heavyQuestionScaleV1(HEAVY_QUESTION_POLICY_V1,small,load),"load "+load).toBeCloseTo(model(load),9);
 expect(heavyQuestionScaleV1(HEAVY_QUESTION_POLICY_V1,small,0)).toBeCloseTo(2.83,9);
 expect(heavyQuestionScaleV1(HEAVY_QUESTION_POLICY_V1,{},0)).toBeGreaterThanOrEqual(1);
 expect(heavyQuestionScaleV1(HEAVY_QUESTION_POLICY_V1,{},1e6)).toBe(HEAVY_QUESTION_POLICY_V1.maximumScale);
});

test("law: the capped child turns its event loop during the preflight, names its phase and reports the lag of each turn (the 60 s of silence came from a stretch of 12 to 20 s that wrote nothing)",async()=>{
 const child=spawn(process.execPath,[REAL,ROOT,join(tmpdir(),"heavy-beat-"+process.pid+".json")],{stdio:["pipe","ignore","pipe"],env:{...process.env,[HEAVY_QUESTION_POLICY_V1.environment]:"2560"}});
 const beats:string[][]=[];let tail="";child.stderr.on("data",chunk=>{tail+=chunk;const lines=tail.split("\n");tail=lines.pop()!;for(const line of lines){const match=/^\u0001alive (\S+) (\d+)$/u.exec(line);if(match)beats.push([match[1]!,match[2]!]);}});
 child.stdin.end(JSON.stringify({preflight:true}));
 const code=await new Promise<number|null>(resolveClose=>child.once("close",resolveClose));
 expect(code).toBe(0);
 expect(beats.filter(([phase])=>/^preflight:\d+\/\d+$/u.test(phase!)).length,"heartbeats named a preflight step").toBeGreaterThanOrEqual(2);
 expect(beats.some(([phase])=>phase==="preflight:39/39"||/^preflight:3[0-9]\/39$/u.test(phase!))).toBe(true);
},600000);

test("law: a child whose parent is killed ends within seconds although it is still computing (two real processes)",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"heavy-orphan-"));
 try{
  const parent=join(directory,"parent.cjs");
  writeFileSync(parent,'const{spawn}=require("node:child_process");const c=spawn(process.execPath,[process.argv[2],process.argv[3],process.argv[4]],{stdio:["pipe","ignore","pipe"]});c.stdin.end(JSON.stringify({preflight:true}));let said=false;c.stderr.on("data",d=>{if(!said&&String(d).includes("alive preflight")){said=true;console.log("CHILD "+c.pid);}});setInterval(()=>{},1000);');
  const holder=spawn(process.execPath,[parent,REAL,ROOT,join(directory,"out.json")],{stdio:["ignore","pipe","ignore"],env:{...process.env,[HEAVY_QUESTION_POLICY_V1.environment]:"2560"}});
  const pid=await new Promise<number>((resolvePid,rejectPid)=>{let text="";holder.stdout.on("data",chunk=>{text+=chunk;const match=/CHILD (\d+)/u.exec(text);if(match)resolvePid(Number(match[1]));});holder.once("close",()=>rejectPid(new Error("the parent ended before the child said preflight")));});
  const lives=()=>{try{process.kill(pid,0);return true;}catch{return false;}};
  expect(lives(),"the child computes while its parent is alive").toBe(true);
  holder.kill("SIGKILL");
  const began=Date.now();while(lives()&&Date.now()-began<20000)await new Promise(resolveTimer=>setTimeout(resolveTimer,200));
  expect(lives(),"the orphan outlived its parent by "+(Date.now()-began)+" ms").toBe(false);expect(Date.now()-began).toBeLessThan(6000);
 }finally{rmSync(directory,{recursive:true,force:true});}
},600000);

test("law: a long prime does not hold a heavy question back (two lanes), and two primes still run one after the other",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"heavy-lanes-"));
 try{
  const slow=fakeChild(directory,"slow.cjs","const timer=setInterval(beat,300);setTimeout(()=>{clearInterval(timer);fs.writeFileSync(output,JSON.stringify({done:Date.now()}));process.exit(0);},3000);");
  const quick=fakeChild(directory,"quick.cjs","fs.writeFileSync(output,JSON.stringify({fact:2,rows:[]}));process.exit(0);");
  const began=Date.now(),prime=primeSharedClosuresInChildV1({workspaceRoot:directory,storeDirectory:directory,policy:FAST,ports:ONE,script:slow}),second=primeSharedClosuresInChildV1({workspaceRoot:directory,storeDirectory:directory,policy:FAST,ports:ONE,script:slow});
  const question=await askHeavyQuestionV1(QUESTION,{workspaceRoot:directory,policy:FAST,ports:ONE,script:quick});
  expect(question).toEqual({fact:2,rows:[]});expect(Date.now()-began,"the question did not wait for the prime").toBeLessThan(2500);
  const first=await prime,next=await second;expect(next.done-first.done,"the second prime started after the first ended").toBeGreaterThanOrEqual(2500);
 }finally{rmSync(directory,{recursive:true,force:true});}
},40000);
