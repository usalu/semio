/** 🔁️ Contract: a failure caused by an edited implementation is classified once (stale code, authority change, captured exit 75, half-edited module, none) and answered by the outermost command with a bounded restart; a restart that follows a long run starts a new count; across real processes a peer that edits the loaded reader while the command runs ends the first process with exit 75 and the replacement finishes, unless the invocation was pinned, which finishes in place. */
import {afterAll,expect,test} from "bun:test";
import {copyFileSync,existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,relative} from "node:path";
import {fileURLToPath,pathToFileURL} from "node:url";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import churnSchema from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧬️schema/🔣️.json";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {exitIfStaleImplementationV1,formatStampProtocolV1,formatStampTextV1,isHalfEditedModuleLoadV1,staleImplementationOfV1} from "../../🟨️.mjs";
import {runWithStaleReexecV1} from "../../🔄️reexec/🟨️.mjs";
import {SCRIPT_PROCESS_INVOCATION_ENV,createScriptProcessEnvelope,readScriptPolicy,scriptProcessEnvironment} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {concurrentEditV1,staleImplementationV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";

type Wire={name?:string;code?:string;reason?:string;path?:string;rank?:number;message:string;outcome?:{status:number};cause?:Wire;errors?:Wire[]};
type Policy={id:string;origin:"error"|"exit"|"load"|"edit";attempt:number;ranMilliseconds:number;outermost:boolean};
const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),protocol=formatStampProtocolV1(),scratches:string[]=[];
const framework=fileURLToPath(new URL("../../../../../../../../",import.meta.url));
const scratch=()=>{const path=mkdtempSync(join(realpathSync(tmpdir()),"ie-"));scratches.push(path);return path;};
afterAll(()=>{for(const path of scratches)rmSync(path,{recursive:true,force:true});});
const oracle=JSON.parse(Bun.spawnSync([python!,"-I",join(here,"🐍️.py"),join(here,"🔣️.json"),join(here,"..","..","🧬️schema","🔣️.json"),join(here,"..","..","..","..","..","..","..","..","🔨️modules","📁️filesystem","🧾️observation","🔁️churn","🧬️schema","🔣️.json")]).stdout.toString()) as {classification:Record<string,string|null>;policy:Record<string,{status:string;code:number;delayMs?:number;nextAttempt?:number;announced?:string}>};
const errorOf=(wire:Wire):Error=>{
 const base=wire.errors?new AggregateError(wire.errors.map(errorOf),wire.message):new Error(wire.message,wire.cause?{cause:errorOf(wire.cause)}:undefined);
 if(wire.name)base.name=wire.name;
 return Object.assign(base,wire.code===undefined?{}:{code:wire.code},wire.reason===undefined?{}:{reason:wire.reason},wire.path===undefined?{}:{path:wire.path},wire.rank===undefined?{}:{rank:wire.rank},wire.outcome===undefined?{}:{outcome:wire.outcome});
};

test("the vectors are closed under their schema and the oracle answers every one",()=>{
 const validate=new Ajv({strict:false}).compile(fixtureSchema);
 expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
 expect(Object.keys(oracle.classification).sort()).toEqual(fixture.classification.map(row=>row.id).sort());
 expect(Object.keys(oracle.policy).sort()).toEqual(fixture.policy.map(row=>row.id).sort());
 expect(churnSchema.definitions.Protocol.const.staleImplementationCodes).toContain("BUILD_AUTHORITY_CHANGED");
});

test("every failure is classified exactly like the independent oracle and like the vector says",()=>{
 for(const row of fixture.classification){
  const found=staleImplementationOfV1(errorOf(row.error as Wire));
  expect(found?.origin??null,row.id).toBe(row.origin as "error"|"exit"|"load"|"edit"|null);
  expect(oracle.classification[row.id],row.id).toBe(row.origin);
 }
 expect(isHalfEditedModuleLoadV1(null)).toBe(false);
});

test("a module that does not load because a peer is editing it is classified from the real loader errors",async()=>{
 const root=scratch();
 writeFileSync(join(root,"bad.mjs"),"export const a = ;\n");writeFileSync(join(root,"good.mjs"),"export const b = 1;\n");
 writeFileSync(join(root,"noexport.mjs"),'import {zzz} from "./good.mjs";export const c = zzz;\n');writeFileSync(join(root,"nofile.mjs"),'import {x} from "./missing.mjs";\n');
 for(const name of ["bad","noexport","nofile"]){
  const failure=await import(join(root,name+".mjs")).then(()=>undefined,error=>error);
  expect(failure,name).toBeDefined();expect(staleImplementationOfV1(failure)?.origin,name).toBe("load");
 }
 const failure=await import(join(root,"bad.mjs")+"?again").then(()=>undefined,error=>error);
 expect(staleImplementationOfV1(failure)?.message).toContain("bad.mjs");
 const bundled=new AggregateError([failure],"Bundle failed");expect(staleImplementationOfV1(bundled)).toBeUndefined();
});

test("the outermost command answers every policy vector like the independent oracle: restart, pause, yield, exhaust, progress",async()=>{
 for(const row of fixture.policy as Policy[]){
  const launches:{SEMIO_STALE_REEXEC?:string}[]=[],sleeps:number[]=[],written:string[]=[];
  const failure=row.origin==="error"?staleImplementationV1("Stale loaded Source revision reader; restart required"):row.origin==="exit"?errorOf({name:"OwnedCommandFailure",message:"repository:command failed (75)",outcome:{status:75}}):row.origin==="load"?errorOf({name:"BuildMessage",message:"Unexpected ;"}):concurrentEditV1("bytes","/w/a.tex");
  const outcome=await runWithStaleReexecV1(async()=>{throw failure;},{environment:{SEMIO_LANGUAGE:"en",SEMIO_STALE_REEXEC:String(row.attempt)},argv:["bun","/repo/script.ts","repo","run","a:b"],execPath:"bun",execArgv:[],outermost:row.outermost,ranMilliseconds:row.ranMilliseconds,sleep:async milliseconds=>{sleeps.push(milliseconds);},launch:async options=>{launches.push({SEMIO_STALE_REEXEC:options.environment.SEMIO_STALE_REEXEC});return 0;},write:text=>{written.push(text);}});
  const expected=oracle.policy[row.id]!;
  expect(outcome.status,row.id).toBe(expected.status as "completed"|"yielded"|"exhausted"|"reexecuted");
  if(expected.status==="reexecuted"){
   expect(sleeps,row.id).toEqual(expected.delayMs?[expected.delayMs]:[]);
   expect(launches.map(launch=>launch.SEMIO_STALE_REEXEC),row.id).toEqual([String(expected.nextAttempt)]);
   expect(written.join(""),row.id).toContain(expected.announced==="loadfailure"?"edited by another process while this command loaded it":expected.announced==="editfailure"?"kept editing /w/a.tex while this command read it (its bytes changed)":"Restarting with the current implementation ("+expected.nextAttempt+" of "+protocol.maximumReexecutions+")");
  }else expect(launches.length,row.id).toBe(0);
  if(expected.status==="exhausted")expect(written.join(""),row.id).toContain(formatStampTextV1("en","final"));
 }
});

test("the announcements of a restart after a half edit exist in both languages with the same placeholders",()=>{
 const placeholders=(language:string)=>[...formatStampTextV1(language,"loadfailure",{reason:"r",delay:1,attempt:1,maximum:3}).matchAll(/./gu)].length>0&&[...String(protocol.texts[language]!.loadfailure).matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
 expect(placeholders("en")).toEqual(placeholders("de"));
 expect(formatStampTextV1("de","loadfailure",{reason:"r",delay:2000,attempt:1,maximum:3})).toContain("Warte 2000 ms");
});

const mirrored=(()=>{const base=join(framework,"🛍️products","🦑️repo","🔨️modules","📚️library","🏗️build","📥️inputs","🔏️revision");return[join(base,"🟨️.mjs"),join(base,"🧬️schema","🔣️.json"),join(framework,"🔨️modules","📁️filesystem","🧾️observation","🔁️churn","🟦️.ts"),join(framework,"🔨️modules","📁️filesystem","🧾️observation","🔁️churn","🧬️schema","🔣️.json"),join(framework,"🔨️modules","📁️filesystem","🧾️observation","🔁️churn","🌊️races","🟦️.ts"),join(framework,"🔨️modules","📁️filesystem","🧾️observation","🔁️churn","🧬️schema","🌊️races.json")];})();
const child=`import{existsSync,writeFileSync}from"node:fs";
import{exitIfStaleImplementationV1}from${JSON.stringify(pathToFileURL(join(framework,"🛍️products","🦑️repo","🔨️modules","📚️library","🏗️build","🧷️format-stamps","🟨️.mjs")).href)};
const[revision,target,directory,mode]=process.argv.slice(2),attempt=process.env.SEMIO_STALE_REEXEC??"0";
const loaded=await import(revision);
writeFileSync(directory+"/ready-"+attempt,"");
if(attempt==="0")while(!existsSync(directory+"/edited"))await new Promise(accept=>setTimeout(accept,20));
try{
 if(mode==="pinned")await loaded.withImplementationPinV1(async()=>loaded.captureBoundedSourceRevisionV1(target,1024));else loaded.captureBoundedSourceRevisionV1(target,1024);
 writeFileSync(directory+"/done-"+attempt,"completed");
}catch(error){exitIfStaleImplementationV1(error);throw error;}
`;

for(const[mode,editMode]of[["unpinned","append"],["unpinned","replace"],["pinned","append"]]as const)test(`across real processes: a peer edits the loaded reader while the command runs (${mode}, ${editMode}): ${mode==="pinned"?"the pinned invocation finishes in place":"exit 75, then the replacement finishes"}`,async()=>{
 const root=scratch(),directory=join(root,"signals"),target=join(root,"target.txt"),script=join(root,"child.mjs");mkdirSync(directory);writeFileSync(target,"data");writeFileSync(script,child);
 for(const path of mirrored){const copy=join(root,relative(framework,path));mkdirSync(dirname(copy),{recursive:true});copyFileSync(path,copy);}
 const revision=join(root,relative(framework,mirrored[0]!));
 const peer=Bun.spawn([python!,"-I",join(here,"🐍️peer/🐍️.py"),join(directory,"ready-0"),join(directory,"edited"),revision,editMode],{stdout:"pipe",stderr:"pipe"});
 const captured:string[]=[],written:string[]=[];
 const runChild=async()=>{const run=Bun.spawn([process.execPath,script,revision,target,directory,mode],{env:{...process.env,SEMIO_STALE_REEXEC:process.env.SEMIO_STALE_REEXEC??"0"},stdout:"pipe",stderr:"pipe"});const code=await run.exited;captured.push(await new Response(run.stderr).text());if(code!==0)throw Object.assign(new Error("repository:command failed ("+code+")"),{name:"OwnedCommandFailure",outcome:{version:1,status:code,signal:null}});};
 const launch=async(options:{execPath:string;args:readonly string[];environment:Readonly<Record<string,string|undefined>>})=>{const run=Bun.spawn([options.execPath,script,revision,target,directory,mode],{env:{...process.env,...options.environment as Record<string,string>},stdout:"pipe",stderr:"pipe"});const code=await run.exited;captured.push(await new Response(run.stderr).text());return code;};
 const outcome=await runWithStaleReexecV1(runChild,{environment:{...process.env,SEMIO_LANGUAGE:"en",SEMIO_STALE_REEXEC:"0"},outermost:true,argv:[process.execPath,script],execPath:process.execPath,execArgv:[],launch,write:text=>{written.push(text);},ranMilliseconds:1000});
 await peer.exited;
 expect(existsSync(join(directory,"edited"))).toBe(true);
 if(mode==="pinned"){
  expect(outcome).toEqual({status:"completed"});expect(existsSync(join(directory,"done-0"))).toBe(true);expect(existsSync(join(directory,"ready-1"))).toBe(false);
 }else{
  expect(outcome).toEqual({status:"reexecuted",code:0});
  expect(captured[0]).toContain("Stale loaded Source revision reader; restart required");
  expect(existsSync(join(directory,"done-0"))).toBe(false);expect(readFileSync(join(directory,"done-1"),"utf8")).toBe("completed");
  expect(written.join("")).toContain("Restarting with the current implementation (1 of "+protocol.maximumReexecutions+").");
 }
},60000);

const routingPath=fileURLToPath(new URL("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts",import.meta.url)),stampsPath=fileURLToPath(new URL("../../🟨️.mjs",import.meta.url)),churnPath=fileURLToPath(new URL("../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts",import.meta.url));
function scriptExit(failure:"edit"|"stale"|"ordinary",library:boolean):{status:number|null;stderr:string}{
 const root=scratch(),script=join(root,"script.ts");
 writeFileSync(script,`import{receiveScriptProcessInvocation}from${JSON.stringify(routingPath)};import{concurrentEditV1,staleImplementationV1}from${JSON.stringify(churnPath)};${library?`import${JSON.stringify(stampsPath)};`:""}
await receiveScriptProcessInvocation(process.env,async()=>{throw ${failure==="edit"?'concurrentEditV1("bytes","/w/a.tex","Consumed Print Source bytes changed","snapshot")':failure==="stale"?'staleImplementationV1("Stale loaded Source revision reader; restart required")':'new Error("ordinary failure")'};});`);
 const envelope=scriptProcessEnvironment(createScriptProcessEnvelope(readScriptPolicy({version:1,owner:"implementation-edit-law",maximumElapsedMilliseconds:60000}),{},Date.now()),{});
 const run=Bun.spawnSync([process.execPath,script],{cwd:root,env:{...process.env,SEMIO_LANGUAGE:"en",[SCRIPT_PROCESS_INVOCATION_ENV]:envelope[SCRIPT_PROCESS_INVOCATION_ENV]},stdout:"pipe",stderr:"pipe"});
 return{status:run.exitCode,stderr:run.stderr.toString()};
}

test("a script process that ends on a failure a fresh process cures exits with the declared code once the repository library is loaded; any other failure keeps its own exit code",()=>{
 const code=protocol.exitCode;
 expect(scriptExit("edit",true).status).toBe(code);
 expect(scriptExit("edit",true).stderr).toContain("kept editing /w/a.tex while this command read it");
 expect(scriptExit("stale",true).status).toBe(code);
 expect(scriptExit("ordinary",true).status).toBe(1);
 expect(scriptExit("edit",false).status).toBe(1);
},60000);

test("a command process that ends on an escaped concurrent edit waits the declared pause for its restart count before it yields exit 75; other restartable failures yield at once",()=>{
 const pauses=protocol.editFailure.backoffMs;
 for(const [attempt,expected] of [...pauses.keys(),pauses.length].map(index=>[index,pauses[Math.min(index,pauses.length-1)]] as const)){
  const waited:number[]=[],written:string[]=[];let code:number|undefined;
  exitIfStaleImplementationV1(concurrentEditV1("bytes","/w/a.tex","Consumed Print Source bytes changed","snapshot"),{environment:{SEMIO_STALE_REEXEC:String(attempt)},sleepSync:milliseconds=>{waited.push(milliseconds);},write:text=>{written.push(text);},exit:value=>{code=value;}});
  expect(waited,"attempt "+attempt).toEqual([expected]);expect(code).toBe(protocol.exitCode);expect(written.join("")).toContain("kept editing /w/a.tex while this command read it (its bytes changed). Waiting "+expected+" ms");expect(written.join("")).toContain("Ein anderer Prozess bearbeitete /w/a.tex laufend");
 }
 const waited:number[]=[];let code:number|undefined;
 exitIfStaleImplementationV1(staleImplementationV1("Stale loaded Source revision reader; restart required"),{sleepSync:milliseconds=>{waited.push(milliseconds);},write:()=>{},exit:value=>{code=value;}});
 expect(waited).toEqual([]);expect(code).toBe(protocol.exitCode);
});
