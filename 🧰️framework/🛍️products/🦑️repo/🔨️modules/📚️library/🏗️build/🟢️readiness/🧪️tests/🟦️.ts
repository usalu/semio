import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {mkdtemp,rm} from "node:fs/promises";
import {tmpdir} from "node:os";
import {stripVTControlCharacters} from "node:util";
import {join} from "node:path";
import {ReadinessMatcherV1,admitReadinessDeclarationV1,probeReadinessV1,readinessMarkerV1,readReadinessDeclarationV1,resolveReadinessV1,watchReadinessV1,type ReadinessV1} from "../🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import projectSchema from "../../🧬️schema/📋️project/🔣️.json";

const read=(path:string)=>JSON.parse(readFileSync(new URL(path,import.meta.url),"utf8"));
const vectors=read("./🔣️.json"),table=read("../../../../🎛️dashboard/🧪️tests/🧫️fixtures/🟢️ready/🔣️.json") as {cases:{name:string;ready:ReadinessV1;chunks:string[];url:string|null}[]};
const sleep=(milliseconds:number)=>new Promise(accept=>setTimeout(accept,milliseconds));

function decide(ready:ReadinessV1,chunks:Iterable<string>):string|null{
 const matcher=new ReadinessMatcherV1(ready);let found:string|undefined;
 for(const chunk of chunks){matcher.push(chunk);found??=matcher.take();}
 matcher.end();found??=matcher.take();
 return found??null;
}

test("the matcher decides every case of the table shared with the dashboard daemon",()=>{
 expect(table.cases.length).toBeGreaterThan(20);
 for(const row of table.cases)expect(decide(row.ready,row.chunks),row.name).toBe(row.url);
});

test("the matcher decides the same way however the output is cut",()=>{
 for(const row of table.cases)expect(decide(row.ready,row.chunks.join("")),row.name).toBe(row.url);
 for(const row of table.cases)expect(decide(row.ready,[...row.chunks.join("")]),row.name).toBe(row.url);
});

test("Node stripVTControlCharacters and a regular expression independently decide the shared table",()=>{
 for(const row of table.cases){
  const pattern=new RegExp(String.raw`http://(?:127\.0\.0\.1|localhost|0\.0\.0\.0):${row.ready.port}(?!\d)`+(row.ready.printed?String.raw`\S*`:""));
  const found=stripVTControlCharacters(row.chunks.join("")).split(/[\r\n\t]/).map(line=>pattern.exec(line)?.[0]).find(match=>match!==undefined);
  expect(found===undefined?null:row.ready.printed?found:found+row.ready.path,row.name).toBe(row.url);
  expect(decide(row.ready,row.chunks),row.name).toBe(found===undefined?null:row.ready.printed?found:found+row.ready.path);
 }
});

test("a matcher announces one address once and ignores later output",()=>{
 const matcher=new ReadinessMatcherV1({port:6061,path:""});
 matcher.push("http://localhost:6061\r\n");
 expect(matcher.take()).toBe("http://localhost:6061");
 matcher.push("http://127.0.0.1:6061\r\n");
 expect(matcher.take()).toBeUndefined();
});

test("an address that reaches the end of the output waits for the next character",()=>{
 const matcher=new ReadinessMatcherV1({port:6061,path:""});
 matcher.push("http://localhost:6061");
 expect(matcher.take()).toBeUndefined();
 matcher.push("0");
 matcher.end();
 expect(matcher.take()).toBeUndefined();
});

test("the build schema validates the declaration vectors like Ajv validates the dashboard schema",async()=>{
 const {default:Ajv}=await import("ajv"),dashboard=read("../../../../🎛️dashboard/🧬️schema/🎮️registry/🔣️.json"),ajv=new Ajv({strict:false,allErrors:true}),accepts=ajv.compile({...dashboard.$defs.ReadyDeclaration,$defs:dashboard.$defs});
 for(const row of vectors.declarations){
  const errors=validateJsonSchemaSubset(schema.$defs.ReadyDeclaration,row.declaration,schema);
  expect(errors.length===0,`${row.name}: ${errors.join("; ")}`).toBe(row.valid);
  expect(accepts(row.declaration),row.name).toBe(row.valid);
 }
});

test("the graph schema and the project schema carry the dashboard declaration of readiness",()=>{
 const dashboard=read("../../../../🎛️dashboard/🧬️schema/🎮️registry/🔣️.json").$defs.ReadyDeclaration,strip=(value:Record<string,unknown>)=>{const {description:_description,...rest}=value;return rest;};
 const fromDashboard=strip(dashboard),fromGraph=strip(schema.$defs.ReadyDeclaration),fromProject=strip(projectSchema.$defs.ReadyDeclaration);
 for(const key of ["port","portEnv","path"] as const){
  expect(fromGraph.properties[key],key).toEqual(fromDashboard.properties[key]);
  expect(fromProject.properties[key],key).toEqual(fromDashboard.properties[key]);
 }
 expect(fromGraph.properties.printed.type).toBe("boolean");
 expect(fromGraph.anyOf).toEqual(fromDashboard.anyOf);
 expect(fromProject).toEqual(fromGraph);
 expect(schema.$defs.ReadyPrinted).toEqual(projectSchema.$defs.ReadyPrinted);
});

test("a target of the graph and of a project declares readiness only through the dashboard metadata",()=>{
 const target=schema.properties.projects.additionalProperties.properties.targets.additionalProperties;
 const graph=(ready:unknown)=>({version:1,projects:{p:{name:"p",root:"p",targets:{serve:{execution:{kind:"command",command:"serve"},continuous:true,metadata:{semio:{dashboard:{ready}}}}}}},dependencies:[]});
 const project=(ready:unknown)=>({name:"p",targets:{serve:{executor:"repo:script",options:{command:"serve"},continuous:true,metadata:{semio:{dashboard:{ready}}}}}});
 for(const row of vectors.declarations){
  expect(validateJsonSchemaSubset(schema,graph(row.declaration)).length===0,`graph: ${row.name}`).toBe(row.valid);
  expect(validateJsonSchemaSubset(projectSchema,project(row.declaration)).length===0,`project: ${row.name}`).toBe(row.valid);
 }
 expect(Object.keys(target.properties.metadata.properties.semio.properties.dashboard.properties)).toEqual(["ready"]);
});

test("a declaration is admitted whole or refused with the identity of its task",()=>{
 expect(admitReadinessDeclarationV1({portEnv:"PORT"},"p:t")).toEqual({portEnv:"PORT"});
 expect(()=>admitReadinessDeclarationV1({port:"6061"},"p:t")).toThrow(/Invalid readiness declaration for p:t/u);
});

test("a target reads its declaration from the dashboard metadata and refuses a broken one",()=>{
 expect(readReadinessDeclarationV1({semio:{dashboard:{ready:{port:6033}}}},"p:t")).toEqual({port:6033});
 expect(readReadinessDeclarationV1({semio:{dashboard:{}}},"p:t")).toBeUndefined();
 expect(readReadinessDeclarationV1(undefined,"p:t")).toBeUndefined();
 expect(()=>readReadinessDeclarationV1({semio:{dashboard:{ready:{path:"/x"}}}},"p:t")).toThrow(/Invalid readiness declaration for p:t/u);
});

test("the port of one run is the variable, else the declared port",()=>{
 for(const row of vectors.resolutions){
  if(row.error)expect(()=>resolveReadinessV1(row.declaration,row.environment,"p:t"),row.name).toThrow(new RegExp(row.error,"u"));
  else expect(resolveReadinessV1(row.declaration,row.environment,"p:t"),row.name).toEqual(row.ready);
 }
});

test("a local server decides readiness by its status without following redirects",async()=>{
 for(const row of vectors.answers){
  const seen:string[]=[],server=Bun.serve({hostname:"127.0.0.1",port:0,fetch(request){seen.push(new URL(request.url).pathname+new URL(request.url).search);return new Response("",{status:row.status,headers:row.status>=300&&row.status<400?{Location:"/elsewhere"}:{}});}});
  try{
   const address=`http://127.0.0.1:${server.port}${row.path==="/"?"":row.path}`;
   expect(await probeReadinessV1(address,new AbortController().signal),row.name).toBe(row.ready);
   expect(seen.length,row.name).toBe(1);
   if(row.probed)expect(seen[0],row.name).toBe(row.probed);
  }finally{await server.stop(true);}
 }
});

test("a closed port, a foreign host and a cancelled probe are not ready",async()=>{
 const server=Bun.serve({hostname:"127.0.0.1",port:0,fetch:()=>new Response("")}),port=server.port;
 const cancelled=new AbortController();cancelled.abort();
 expect(await probeReadinessV1(`http://127.0.0.1:${port}`,cancelled.signal)).toBe(false);
 await server.stop(true);
 expect(await probeReadinessV1(`http://127.0.0.1:${port}`,new AbortController().signal)).toBe(false);
 expect(await probeReadinessV1("http://example.com:80",new AbortController().signal)).toBe(false);
 expect(await probeReadinessV1("https://localhost:6061",new AbortController().signal)).toBe(false);
});

test("a watch becomes ready only after the announced server answers",async()=>{
 const probe=Bun.serve({hostname:"127.0.0.1",port:0,fetch:()=>new Response("")}),port=probe.port;
 await probe.stop(true);
 const watch=watchReadinessV1({port,path:"/health"},new AbortController().signal);
 let answered:string|undefined;void watch.ready.then(address=>{answered=address;});
 watch.observe("starting\n");
 watch.observe(`  Local: http://localhost:${port}/\n`);
 await sleep(300);
 expect(answered).toBeUndefined();
 const paths:string[]=[],server=Bun.serve({hostname:"127.0.0.1",port,fetch(request){paths.push(new URL(request.url).pathname);return new Response("");}});
 try{
  expect(await watch.ready).toBe(`http://localhost:${port}/health`);
  expect(paths.length).toBeGreaterThan(0);
  expect(new Set(paths)).toEqual(new Set(["/health"]));
 }finally{watch.stop();await server.stop(true);}
});

test("a cancelled watch never becomes ready and probes no more",async()=>{
 const controller=new AbortController(),calls:string[]=[];
 const watch=watchReadinessV1({port:6061,path:""},controller.signal,async address=>{calls.push(address);return false;},(milliseconds,signal)=>new Promise<void>(accept=>{const timer=setTimeout(accept,Math.min(milliseconds,5));signal.addEventListener("abort",()=>{clearTimeout(timer);accept();},{once:true});}));
 let answered=false;void watch.ready.then(()=>{answered=true;});
 watch.observe("http://localhost:6061\n");
 await sleep(60);
 expect(calls.length).toBeGreaterThan(1);
 controller.abort();
 await sleep(30);
 const settled=calls.length;
 await sleep(60);
 expect(calls.length).toBe(settled);
 expect(answered).toBe(false);
});

test("a watch without an announcement never probes",async()=>{
 const calls:string[]=[],watch=watchReadinessV1({port:6061,path:""},new AbortController().signal,async address=>{calls.push(address);return true;});
 watch.observe("http://localhost:3000\nlistening\n");
 watch.end();
 await sleep(50);
 expect(calls).toEqual([]);
 watch.stop();
});

test("the policy of readiness is closed and finite",()=>{
 const {marker,...limits}=schema.$defs.ReadinessPolicy.const;
 expect(limits.version).toBe(1);
 expect(Object.values(limits).every(value=>Number.isSafeInteger(value)&&value>0)).toBe(true);
 expect(readinessMarkerV1).toBe(marker);
 expect(marker.length).toBeGreaterThan(0);
});

test("the Nx command bridge awaits the declared readiness of a continuous target through the shared marker",async()=>{
 const routing=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),{executeLiteralBuildArgumentsV1}=await import("../../../⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🟨️.mjs"),{runNxCommandSourceV1}=await import("../../../⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/📜️script.ts");
 const directory=await mkdtemp(join(tmpdir(),"semio-readiness-")),previous={invocation:process.env.SEMIO_SCRIPT_PROCESS_INVOCATION,artifacts:process.env.SEMIO_TEST_ARTIFACT_DIR},probe=Bun.serve({hostname:"127.0.0.1",port:0,fetch:()=>new Response("")}),port=probe.port;
 await probe.stop(true);
 const envelope=routing.createScriptProcessEnvelope({version:1,owner:"nx-readiness",maximumElapsedMilliseconds:30000},{exact:"original"},Date.now());
 process.env.SEMIO_SCRIPT_PROCESS_INVOCATION=JSON.stringify(envelope);process.env.SEMIO_TEST_ARTIFACT_DIR=directory;
 const target=(declared:Record<string,unknown>)=>({root:process.cwd(),projectName:"app",targetName:"serve",projectsConfigurations:{projects:{app:{targets:{serve:declared}}}}});
 const ready={port},metadata={semio:{dashboard:{ready}}};
 const program=`const delay=ms=>new Promise(accept=>setTimeout(accept,ms));await delay(150);const server=Bun.serve({hostname:"127.0.0.1",port:${port},fetch:()=>new Response("")});console.log("Local: http://localhost:${port}/");await delay(600);server.stop(true);`;
 const written:string[]=[],write=process.stdout.write.bind(process.stdout),log=console.log.bind(console);
 try{
  const idle=await executeLiteralBuildArgumentsV1({command:"bun -e "+JSON.stringify(program)},target({continuous:false,metadata}),async(plan:any)=>plan);
  expect(idle.readyWhen).toBeUndefined();
  const explicit=await executeLiteralBuildArgumentsV1({command:"true",readyWhen:"custom"},target({continuous:true,metadata}),async(plan:any)=>plan);
  expect(explicit.readyWhen).toBe("custom");
  const undeclared=await executeLiteralBuildArgumentsV1({command:"true"},target({continuous:true}),async(plan:any)=>plan);
  expect(undeclared.readyWhen).toBeUndefined();
  const plan=await executeLiteralBuildArgumentsV1({command:"bun -e "+JSON.stringify(program)},target({continuous:true,metadata}),async(plan:any)=>plan);
  expect(plan.readyWhen).toBe(readinessMarkerV1);
  const digest=plan.command.split(" ").at(-1),source=JSON.parse(readFileSync(join(plan.env.SEMIO_NX_COMMAND_DIRECTORY,digest+".json"),"utf8"));
  expect(source.ready).toEqual(ready);
  process.stdout.write=((chunk:any,...rest:any[])=>{written.push(String(chunk));return (write as any)(chunk,...rest);}) as typeof process.stdout.write;
  console.log=((...values:unknown[])=>{written.push(values.join(" ")+"\n");log(...values);}) as typeof console.log;
  await routing.withScriptProcessEnvelope(envelope,async()=>{await runNxCommandSourceV1([digest],plan.env);});
  process.stdout.write=write;console.log=log;
  const output=written.join("");
  expect(output).toContain(`Local: http://localhost:${port}/`);
  expect(output).toContain(`${readinessMarkerV1} http://localhost:${port}`);
  expect(output.indexOf(`Local: http://localhost:${port}/`)).toBeLessThan(output.indexOf(readinessMarkerV1));
 }finally{
  process.stdout.write=write;console.log=log;
  for(const [key,value] of [["SEMIO_SCRIPT_PROCESS_INVOCATION",previous.invocation],["SEMIO_TEST_ARTIFACT_DIR",previous.artifacts]] as const)if(value===undefined)delete process.env[key];else process.env[key]=value;
  await rm(directory,{recursive:true,force:true});
 }
},60000);
