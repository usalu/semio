/** 🛌️ Contract: a command that only waits for a peer's graph build yields: the waiting process writes a ticket and exits with the restart code, the front door waits alone and starts the same command again with the request time of the first start; the decision and the wake rule equal the independent Python oracle, the policy is closed under its schema in both languages, the supervisor restarts a yielded run without counting it as a failed restart, and a real child process ends with exit 75 and a readable ticket. */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import vectors from "./🔣️.json";
import {graphQueueRequestedAtV1,graphQueueYieldPolicyV1,graphQueueYieldRoundV1,graphQueueYieldsV1,waitGraphQueueYieldV1,yieldGraphQueueV1} from "../🟨️.mjs";
import {runWithStaleReexecV1} from "../../../../🧷️format-stamps/🔄️reexec/🟨️.mjs";
import {formatStampProtocolV1} from "../../../../🧷️format-stamps/🟨️.mjs";
import {loadGenerationGraphV1} from "../../🔌️load/🟨️.mjs";
import {applyOps,createMockHost,seed} from "../../🧪️tests/🏃️world/🟦️.ts";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";

const policy=graphQueueYieldPolicyV1(),here=dirname(fileURLToPath(import.meta.url)),names=policy.environment;
const oracle=JSON.parse(Bun.spawnSync([testPythonV1()!,"-I",join(here,"🐍️.py"),join(here,"..","🔣️.json"),join(here,"🔣️.json")]).stdout.toString()) as {decide:Record<string,boolean>;wake:Record<string,{elapsedMs:number;reason:string;progress:number}>};
let sandbox="";
beforeAll(()=>{sandbox=realpathSync(mkdtempSync(join(tmpdir(),"yield-")));});
afterAll(()=>{rmSync(sandbox,{recursive:true,force:true});});

test("the policy is closed under its schema, speaks both languages with the same placeholders and exits with the restart code of the format stamp protocol",()=>{
 const validate=new Ajv({strict:false}).compile(schema);expect(validate(policy),JSON.stringify(validate.errors)).toBe(true);
 for(const key of ["waiting","resumed"] as const){
  const placeholders=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort().join();
  expect(placeholders(policy.texts[key]!.en!)).toBe(placeholders(policy.texts[key]!.de!));
 }
 expect(policy.exitCode).toBe(formatStampProtocolV1().exitCode);
 expect(policy.yieldAfterMilliseconds).toBeGreaterThan(policy.pollMilliseconds);
});

test("the decision to yield equals the independent oracle on every vector",()=>{
 expect(Object.keys(oracle.decide).sort()).toEqual(vectors.decide.map(row=>row.id).sort());
 for(const row of vectors.decide){
  const environment:Record<string,string|undefined>={};if(row.ticket)environment[names.ticket]=join(sandbox,"t.json");if(row.round!==null)environment[names.round]=row.round;
  const answer=graphQueueYieldsV1({queuedMs:row.queuedMs,environment});
  expect(answer,row.id).toBe(row.expected);expect(answer,row.id).toBe(oracle.decide[row.id]!);
 }
 expect(graphQueueRequestedAtV1({[names.requestedAt]:"1700000000000"})).toBe(1700000000000);expect(graphQueueRequestedAtV1({[names.requestedAt]:"x"})).toBeUndefined();expect(graphQueueRequestedAtV1({})).toBeUndefined();
 expect(graphQueueYieldRoundV1({[names.round]:"2"})).toBe(2);
});

test("the wake rule of the waiting front door equals the oracle: a newer generation or the wake time, progress every thirty seconds, in both languages",async()=>{
 for(const row of vectors.wake){
  const ticket=join(sandbox,row.id+".json");writeFileSync(ticket,JSON.stringify({version:1,generation:row.generation,pointer:"/p/current.json",requestedAt:1234567,round:1,at:1}));
  let clock=1_000_000;const start=clock,lines:string[]=[];
  const series=row.series as [number,number][];
  const next=await waitGraphQueueYieldV1({environment:{[names.ticket]:ticket,SEMIO_LANGUAGE:""},now:()=>clock,sleep:async milliseconds=>{clock+=milliseconds;},write:text=>lines.push(text),read:()=>{let value=-1;for(const[moment,generation]of series)if(moment<=clock-start)value=generation;return value;}});
  const expected=oracle.wake[row.id]!;
  expect(clock-start,row.id).toBe(expected.elapsedMs);expect(clock-start,row.id).toBe(row.expected.elapsedMs);
  expect(lines.length-1,row.id).toBe(expected.progress);expect(lines.at(-1)!,row.id).toContain("graph: ");expect(lines.at(-1)!.split("\n").length).toBe(2);
  expect(next,row.id).toEqual({[names.requestedAt]:"1234567",[names.round]:"2"});expect(existsSync(ticket),row.id).toBe(false);
 }
 expect(await waitGraphQueueYieldV1({environment:{[names.ticket]:join(sandbox,"absent.json")}})).toBeUndefined();
 const broken=join(sandbox,"broken.json");writeFileSync(broken,"{ torn");expect(await waitGraphQueueYieldV1({environment:{[names.ticket]:broken}})).toBeUndefined();expect(existsSync(broken)).toBe(false);
 expect(await waitGraphQueueYieldV1({environment:{}})).toBeUndefined();
 const aborted=new AbortController(),held=join(sandbox,"held.json");writeFileSync(held,JSON.stringify({version:1,generation:1,pointer:"/p",requestedAt:1,round:0,at:1}));aborted.abort(new Error("stop"));
 await expect(waitGraphQueueYieldV1({environment:{[names.ticket]:held},signal:aborted.signal,read:()=>0,sleep:async()=>{}})).rejects.toThrow("stop");
});

test("yielding writes the ticket atomically beside the path the front door named, tells the user in both languages and exits with the restart code",()=>{
 const ticket=join(sandbox,"written.json"),lines:string[]=[];let code:number|undefined;
 yieldGraphQueueV1({generation:41,pointer:"/cache/g/current.json",requestedAt:1700000000123,queuedMs:16000,environment:{[names.ticket]:ticket,[names.round]:"1",SEMIO_LANGUAGE:""},write:text=>lines.push(text),exit:value=>{code=value;},now:()=>99});
 expect(code).toBe(75);expect(JSON.parse(readFileSync(ticket,"utf8"))).toEqual({version:1,generation:41,pointer:"/cache/g/current.json",requestedAt:1700000000123,round:1,at:99});
 expect(lines.join("\n")).toContain("a peer builds the graph");expect(lines.join("\n")).toContain("ein Peer baut den Graphen");expect(lines.join("\n")).toContain("16 s");expect(lines.join("\n")).toContain("41");
 expect(existsSync(ticket+"."+process.pid+".tmp")).toBe(false);
});

test("the supervisor starts a yielded run again with the new environment and without counting a failed restart; a run that did not yield restarts as before; a nested command only yields",async()=>{
 const failure=Object.assign(new Error("repository:command failed (75)"),{name:"OwnedCommandFailure",outcome:{status:75}});
 const launches:{environment:Record<string,string|undefined>}[]=[],options={environment:{SEMIO_LANGUAGE:"en",SEMIO_STALE_REEXEC:"2"},argv:["bun","s.ts"],execPath:"bun",execArgv:[],write:()=>{},sleep:async()=>{},launch:async(row:{environment:Record<string,string|undefined>})=>{launches.push(row);return 0;}};
 expect(await runWithStaleReexecV1(async()=>{throw failure;},{...options,outermost:true,yielded:async()=>({[names.requestedAt]:"77",[names.round]:"1"})})).toEqual({status:"reexecuted",code:0});
 expect(launches[0]!.environment[names.requestedAt]).toBe("77");expect(launches[0]!.environment.SEMIO_STALE_REEXEC).toBe("2");
 expect(await runWithStaleReexecV1(async()=>{throw failure;},{...options,outermost:true,yielded:async()=>undefined})).toEqual({status:"reexecuted",code:0});
 expect(launches[1]!.environment.SEMIO_STALE_REEXEC).toBe("3");expect(launches[1]!.environment[names.requestedAt]).toBeUndefined();
 let asked=0;
 expect(await runWithStaleReexecV1(async()=>{throw failure;},{...options,outermost:false,yielded:async()=>{asked++;return{};}})).toEqual({status:"yielded",code:75});expect(asked).toBe(0);
 await expect(runWithStaleReexecV1(async()=>{throw new Error("ordinary");},{...options,outermost:true,yielded:async()=>({})})).rejects.toThrow("ordinary");
});

test("across real processes: the waiting child ends with exit 75 and a ticket, the supervisor of its parent waits for a newer generation and starts it again with the first request time",async()=>{
 const pointer=join(sandbox,"current.json"),ticket=join(sandbox,"real-ticket.json"),module=join(here,"..","🟨️.mjs"),reexec=join(here,"..","..","..","..","🧷️format-stamps","🔄️reexec","🟨️.mjs"),marker=join(sandbox,"second-start.txt");
 writeFileSync(pointer,JSON.stringify({version:1,generation:3}));
 const child=join(sandbox,"child.mjs");
 writeFileSync(child,`import{graphQueueRequestedAtV1,graphQueueYieldRoundV1,yieldGraphQueueV1}from${JSON.stringify(module)};import{writeFileSync}from"node:fs";
if(graphQueueYieldRoundV1()===0)yieldGraphQueueV1({generation:3,pointer:${JSON.stringify(pointer)},requestedAt:1700000000000,queuedMs:16000});
else writeFileSync(${JSON.stringify(marker)},String(graphQueueRequestedAtV1())+":"+graphQueueYieldRoundV1());`);
 const environment={...process.env,SEMIO_LANGUAGE:"en",[names.ticket]:ticket};delete environment[names.round];delete environment[names.requestedAt];
 const first=Bun.spawnSync([process.execPath,child],{env:environment,stderr:"pipe"});
 expect(first.exitCode).toBe(75);expect(JSON.parse(readFileSync(ticket,"utf8")).generation).toBe(3);expect(first.stderr.toString()).toContain("a peer builds the graph");
 setTimeout(()=>writeFileSync(pointer,JSON.stringify({version:1,generation:4})),400);
 const written:string[]=[];
 const outcome=await runWithStaleReexecV1(async()=>{throw Object.assign(new Error("repository:command failed (75)"),{name:"OwnedCommandFailure",outcome:{status:75}});},{environment,outermost:true,argv:[process.execPath,child],execPath:process.execPath,execArgv:[],write:text=>written.push(text),sleep:async()=>{},yielded:env=>waitGraphQueueYieldV1({environment:env,sleep:milliseconds=>new Promise(accept=>setTimeout(accept,Math.min(milliseconds,100)))})});
 expect(outcome).toEqual({status:"reexecuted",code:0});expect(readFileSync(marker,"utf8")).toBe("1700000000000:1");expect(existsSync(ticket)).toBe(false);
},30000);

const sleep=(ms:number)=>new Promise<void>(resolve=>setTimeout(resolve,ms));
test("a command that only waits for a peer's graph build yields at the next notice once its front door offered a ticket; without a ticket it queues in place; the first request time is the one it asks for",async()=>{
 const dir=join(sandbox,"queue-world"),files:Record<string,string>={};
 for(const[name,imports]of[["a",["@syn/b"]],["b",[]]] as [string,string[]][]){files[`pkg/${name}/project.json`]=JSON.stringify({name,package:`@syn/${name}`,uses:[]});files[`pkg/${name}/src/index.ts`]=imports.map(spec=>`import "${spec}";`).join("\n")+"\nexport const v=1;\n";}
 await seed(dir,files);const custody={tail:Promise.resolve()};
 await loadGenerationGraphV1(createMockHost(dir,{stamp:true,custody}).host);
 await applyOps(dir,[{op:"write",path:"pkg/b/src/index.ts",content:"export const v=2;\n"}]);
 const ticket=join(sandbox,"load-ticket.json"),saved={ticket:process.env[names.ticket],at:process.env[names.requestedAt],round:process.env[names.round]};
 let open!:()=>void;const gate=new Promise<void>(resolve=>{open=resolve;});
 try{
  delete process.env[names.requestedAt];delete process.env[names.round];
  const holder=createMockHost(dir,{stamp:true,custody,gate:()=>gate}),without=createMockHost(dir,{stamp:true,custody,waitNoticeMilliseconds:25});
  const exits:number[]=[];for(const row of[holder,without])Object.assign(row.host,{yieldAfterMilliseconds:30,exitProcess:(code:number)=>{exits.push(code);}});
  delete process.env[names.ticket];
  const one=loadGenerationGraphV1(holder.host,{closure:["a"]});await sleep(40);const two=loadGenerationGraphV1(without.host,{closure:["a"]});await sleep(200);
  expect(exits).toEqual([]);expect(existsSync(ticket)).toBe(false);
  open();await Promise.all([one,two]);
  await applyOps(dir,[{op:"write",path:"pkg/b/src/index.ts",content:"export const v=3;\n"}]);
  let again!:()=>void;const second=new Promise<void>(resolve=>{again=resolve;});
  const holder2=createMockHost(dir,{stamp:true,custody,gate:()=>second,waitNoticeMilliseconds:600000});Object.assign(holder2.host,{yieldAfterMilliseconds:30,exitProcess:(code:number)=>{exits.push(code);}});
  const four=loadGenerationGraphV1(holder2.host,{closure:["a"]});await sleep(40);
  const offered=createMockHost(dir,{stamp:true,custody,waitNoticeMilliseconds:25});Object.assign(offered.host,{yieldAfterMilliseconds:30,exitProcess:(code:number)=>{exits.push(code);}});
  const asked=Date.now()+50;process.env[names.ticket]=ticket;process.env[names.requestedAt]=String(asked);process.env[names.round]="1";
  const three=loadGenerationGraphV1(offered.host,{closure:["a"]});while(exits.length===0)await sleep(5);
  const written=JSON.parse(readFileSync(ticket,"utf8"));
  expect(exits[0]).toBe(75);expect(written).toMatchObject({version:1,pointer:join(dir,".gen","current.json"),requestedAt:asked,round:1});expect(Number.isInteger(written.generation)).toBe(true);
  expect(offered.said.some(row=>row.key==="waiting")).toBe(true);
  again();await Promise.all([four,three]);
 }finally{for(const[key,value]of[[names.ticket,saved.ticket],[names.requestedAt,saved.at],[names.round,saved.round]] as const){if(value===undefined)delete process.env[key];else process.env[key]=value;}open();}
},60000);
