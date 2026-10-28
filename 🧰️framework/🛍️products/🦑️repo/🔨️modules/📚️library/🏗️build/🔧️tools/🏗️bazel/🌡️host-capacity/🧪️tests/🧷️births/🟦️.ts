/** 🧷️ Contract on the real action control of the Bazel script (its `bazelCancellationControl` with the real width gate and the real birth handshake clock, over real sockets): only an action's birth takes a job of the grant (its own execution join belongs to it), so births that fill the grant's jobs never block their own children (audit10's reproducer, 1, 3 and 8 jobs); a birth first hears `waiting` and its handshake clock stops, so a quiet grant whose growth takes longer than the handshake still lets two births on a 1-job grant finish, one after the other.
 * @see ./🥒️.feature
 * @see ../../../📜️script.ts (a checkout whose executor has no handshake helper is driven with the clock it had: armed at the birth's start, stopped only by `started`)
 * @see ../../../🎬️executor/🟦️.ts
 * @see ../../../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/result-audit10.md */
import {expect,test} from "bun:test";
import {createConnection,createServer,type Socket} from "node:net";
import {randomUUID} from "node:crypto";
import {readFileSync} from "node:fs";
import {runInNewContext} from "node:vm";
import ts from "typescript";
import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import component from "../../../🧬️schema/🔣️.json";

const birthSchema=component.definitions.ActionBirth,policy=component.definitions.Policy.const,digest="a".repeat(64);
const extract=(file:string,names:readonly string[],optional:readonly string[]=[])=>{const ast=ts.createSourceFile("source.ts",readFileSync(new URL(file,import.meta.url),"utf8"),ts.ScriptTarget.Latest,true);return names.map(name=>{const node=ast.statements.find(value=>ts.isFunctionDeclaration(value)&&value.name?.text===name);if(!node&&!optional.includes(name))throw Error(`${name} required in ${file}`);return node?node.getText(ast).replace(/^export /u,""):"";}).join("\n");};
const legacyClock="function bazelBirthHandshakeV1(maximum,deadline,started,fail){let timer=setTimeout(()=>fail(Error('bazel.birth-handshake-deadline-exhausted')),maximum);return{waiting:()=>undefined,settle:()=>clearTimeout(timer)};}";
const executorText=extract("../../../🎬️executor/🟦️.ts",["initializeBazelControlTextV1","bazelBirthParametersAllowedV1","bazelBirthHandshakeV1"],["bazelBirthHandshakeV1"]);
const body=[extract("../../../📜️script.ts",["bazelCancellationControl"]),executorText,executorText.includes("function bazelBirthHandshakeV1")?"":legacyClock,extract("../../🟦️.ts",["createBazelActionWidthV1"])].join("\n");
const refuseBuildV1=(_kind:string,code:string)=>Object.assign(Error(code),{code});
const make=(jobs:()=>number|undefined,upgrade:(signal:AbortSignal)=>Promise<unknown>)=>runInNewContext(ts.transpile(body+"\n({control:bazelCancellationControl,handshake:bazelBirthHandshakeV1});",{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}),{advanceScriptInvocation:async()=>undefined,checkScriptInvocation:()=>undefined,readIssuedScriptProcessEnvelopeV1:()=>({}),upgradeBuildCapacityV1:upgrade,bazelLiveHostJobsV1:()=>jobs(),BUILD_HOST_CAPACITY_POLICY_V1:{wait:{pollMilliseconds:20}},refuseBuildV1,validateJsonSchemaSubset,randomUUID,createServer,AbortController,Buffer,JSON,Error,Map,Set,Promise,Object,Math,Date,performance,process,setTimeout,clearTimeout,setInterval,clearInterval,policy,birthSchema,componentSchema:component}) as {control:(invocation:unknown,signal:AbortSignal,births:Map<string,unknown>)=>Promise<{url:string;stop:()=>Promise<void>}>;handshake:(maximum:number,deadline:number|null,started:number,fail:(error:unknown)=>void)=>{waiting:()=>void;settle:()=>void}};

type Joined=Readonly<{socket:Socket;id:string;events:string[]}>;
/** 🔌️ One join on the real control: a birth (with the real handshake clock of `handshakeMilliseconds`) or an action's own execution join; resolves on `started`. */
function join(url:URL,birth:boolean,clock:ReturnType<typeof make>["handshake"],handshakeMilliseconds:number):Promise<Joined>{
 return new Promise((accept,refuse)=>{
  const socket=createConnection({host:url.hostname,port:Number(url.port)}),events:string[]=[],token=url.pathname.slice(1),started=performance.now();let id:string|undefined,pending="";
  const hs=birth?clock(handshakeMilliseconds,null,started,error=>{socket.destroy();refuse(error);}):undefined;
  socket.setEncoding("utf8");socket.on("error",refuse);
  socket.once("connect",()=>socket.write(JSON.stringify({version:1,event:"join",token,...birth?{birth:{kind:"launch",task:"fixture:service",manifestSha256:digest,parameters:[]}}:{}})+"\n"));
  socket.on("data",chunk=>{pending+=chunk;let end:number;while((end=pending.indexOf("\n"))>=0){const message=JSON.parse(pending.slice(0,end));pending=pending.slice(end+1);events.push(message.event);
   if(message.event===birthSchema.definitions.Protocol.const.waitingEvent&&!id)hs?.waiting();
   else if(message.event==="joined"&&!id){id=message.id;socket.write(JSON.stringify({version:1,event:"start",token,id})+"\n");}
   else if(message.event==="started"){hs?.settle();accept({socket,id:id!,events});}}});
 });
}
const close=(row:Joined,url:URL)=>new Promise<void>(done=>{row.socket.once("close",()=>done());row.socket.write(JSON.stringify({version:1,event:"closed",token:url.pathname.slice(1),id:row.id})+"\n");setTimeout(()=>row.socket.destroy(),200);});
const within=<T,>(promise:Promise<T>,ms:number)=>Promise.race([promise.then(value=>({value})),new Promise<{value?:T}>(done=>setTimeout(()=>done({}),ms))]);
const births=new Map([["fixture:service",{digest,parameters:{launch:[[]]}}]]);

test("audit10's reproducer on the real control: births that fill the grant's jobs never block their own children's execution joins (1, 3 and 8 jobs)",async()=>{
 for(const jobs of [1,3,8]){
  const api=make(()=>jobs,async()=>"held"),controller=new AbortController(),control=await api.control({},controller.signal,births),url=new URL(control.url);
  const born=await Promise.all([...Array(jobs)].map(()=>join(url,true,api.handshake,10_000)));
  const children=await Promise.all(born.map(()=>within(join(url,false,api.handshake,10_000),1500)));
  expect(children.filter(row=>row.value===undefined).length,`${jobs} job(s): blocked execution joins`).toBe(0);
  for(const row of [...children.map(item=>item.value!),...born])await close(row,url);
  controller.abort(Error("done"));await control.stop().catch(()=>undefined);
 }
},30000);

test("A quiet grant whose growth outlasts the birth handshake still lets two births on a 1-job grant finish, one after the other: each hears `waiting` first, the second starts only when the first closed",async()=>{
 let grown=false;const order:string[]=[];
 const api=make(()=>1,async()=>{if(!grown){await new Promise(done=>setTimeout(done,700));grown=true;}return "upgraded";}),controller=new AbortController(),control=await api.control({},controller.signal,births),url=new URL(control.url);
 const first=join(url,true,api.handshake,250).then(row=>{order.push("first started");return row;}),second=join(url,true,api.handshake,250).then(row=>{order.push("second started");return row;});
 const one=await first,child=await within(join(url,false,api.handshake,250),1500);
 expect(child.value,"the first action's own execution join is not blocked by the full grant").toBeDefined();
 await new Promise(done=>setTimeout(done,300));expect(order,"the second birth waits for the grant's only job").toEqual(["first started"]);
 order.push("first closed");await close(child.value!,url);await close(one,url);
 const two=await second;
 expect(order).toEqual(["first started","first closed","second started"]);
 expect(one.events.slice(0,2)).toEqual(["waiting","joined"]);expect(two.events[0]).toBe("waiting");
 await close(two,url);controller.abort(Error("done"));await control.stop().catch(()=>undefined);
},30000);

test("The script and the executor carry the rule: a birth says `waiting` before it waits for the growth and a job, an execution join takes no job, a closed connection frees its job, and the birth's handshake clock stops on `waiting`",()=>{
 const script=readFileSync(new URL("../../../📜️script.ts",import.meta.url),"utf8"),executor=readFileSync(new URL("../../../🎬️executor/🟦️.ts",import.meta.url),"utf8");
 const birth=script.indexOf('socket.write(JSON.stringify({version:1,event:birthSchema.definitions.Protocol.const.waitingEvent})+"\\n");await upgradeBuildCapacityV1(signal);await width.enter(socket,signal);}else await upgradeBuildCapacityV1(signal);const response=');
 expect(birth).toBeGreaterThan(0);expect(script.indexOf('event:"joined"',birth)).toBeGreaterThan(birth);expect(script.split("width.enter(").length-1).toBe(1);
 expect(script).toContain('socket.once("close",()=>{connections.delete(socket);width.leave(socket);});');
 expect(executor).toContain("handshake=bazelBirthHandshakeV1(protocol.maximumHandshakeMilliseconds,envelope.deadlineEpochMilliseconds,started,fail);");
 expect(executor).toContain("else if(message.event===protocol.waitingEvent&&!id&&!issued)handshake.waiting();");
});
