import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {spawn} from "node:child_process";
import {existsSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import cases from "./🔄️graph-yield.json";
import schema from "./🔄️graph-yield.schema.json";

const helper=fileURLToPath(new URL("./📜️child/🔄️graph-yield.ts",import.meta.url));
const read=(root:string,name:string)=>JSON.parse(readFileSync(join(root,name+".json"),"utf8"));
const entries=(root:string,kind:string)=>{try{return readdirSync(join(root,"capacity",kind));}catch{return [];}};
const grantRows=(root:string)=>entries(root,"grants").filter(name=>name.endsWith(".json"));
const until=async(check:()=>boolean,message:string,milliseconds=5000)=>{
 const end=performance.now()+milliseconds;
 while(!check()&&performance.now()<end)await Bun.sleep(20);
 expect(check(),message).toBe(true);
};

test("the private graph-yield corpus satisfies its neutral schema",()=>{
 expect(new Ajv({strict:true,allowUnionTypes:true}).validate(schema,cases)).toBe(true);
});

for(const fixture of cases)test.skipIf(fixture.signal!==null&&process.platform==="win32")("real graph yield capacity "+fixture.id,async()=>{
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"gy-")),ticket=join(root,"ticket.json");
 const environment={...process.env,SEMIO_BUILD_GRAPH_YIELD_TICKET:ticket,SEMIO_TEST_BAD_GRAPH_TICKET:fixture.id==="bad-ticket"?"1":"0"};
 for(const name of ["SEMIO_HOST_CAPACITY_GRANT","SEMIO_HOST_CAPACITY_HANDOVER","SEMIO_HOST_CAPACITY_PLAN","SEMIO_HOST_CAPACITY","SEMIO_SCRIPT_PROCESS_INVOCATION","SEMIO_STALE_REEXEC","SEMIO_BUILD_GRAPH_YIELD_ROUND","SEMIO_BUILD_GRAPH_REQUESTED_AT"])delete (environment as NodeJS.ProcessEnv)[name];
 const children:ReturnType<typeof start>[]=[];
 function start(mode:string,extra:Record<string,string>={}){
  const child=spawn(process.execPath,[helper,root,mode],{env:{...environment,...extra},stdio:["ignore","ignore","pipe"],windowsHide:true});
  let stderr="";child.stderr.on("data",chunk=>{stderr+=chunk;});
  const done=new Promise<{code:number|null;signal:NodeJS.Signals|null;stderr:string}>(resolve=>child.once("close",(code,signal)=>resolve({code,signal,stderr})));
  return {child,done};
 }
 const spawnFixture=(mode:string,extra?:Record<string,string>)=>{const started=start(mode,extra);children.push(started);return started;};
 const has=(name:string)=>existsSync(join(root,name+".json"));
 const publish=(generation:number)=>writeFileSync(join(root,"generation.json"),JSON.stringify({version:1,generation}));
 const stopCompetitor=()=>writeFileSync(join(root,"release-competitor"),"");
 try{
  publish(fixture.initialGeneration);
  using db=new Database(":memory:");
  const expected=db.query(`SELECT
   CASE WHEN $signal IS NOT NULL THEN 130 ELSE 0 END AS exit,
   CASE WHEN $signal IS NOT NULL THEN NULL WHEN $id='bad-ticket' THEN 0 ELSE 1 END AS round,
   CASE WHEN $signal IS NOT NULL THEN NULL ELSE $id='bad-ticket' END AS adopted,
   (20000-4096)>=12000 AND (20000-4096)<2*12000 AS singleGrant`).get({$signal:fixture.signal,$id:fixture.id}) as {exit:number;round:number|null;adopted:number|null;singleGrant:number};
  expect(expected).toEqual({exit:fixture.exit,round:fixture.round,adopted:fixture.adopted===null?null:Number(fixture.adopted),singleGrant:1});
  let competitor:ReturnType<typeof start>|undefined;
  if(fixture.id==="inherited"){
   competitor=spawnFixture("competitor");
   await until(()=>has("competitor"),"parent grant admitted");
  }
  const inherited=competitor===undefined?undefined:read(root,"competitor"),door=spawnFixture("door",inherited===undefined?{}:{SEMIO_HOST_CAPACITY_GRANT:inherited.id});
  await until(()=>has("first"),"original door was admitted");
  const first=read(root,"first");
  expect(first).toMatchObject({pid:door.child.pid,reservedMegabytes:12000,inherited:inherited!==undefined,grantPid:inherited?.pid??door.child.pid});
  if(fixture.competes||fixture.signal!==null){
   await until(()=>!grantRows(root).includes(first.id+".json"),"valid ticket returned the old grant before a new generation",1700);
   expect(entries(root,"grants")).toEqual([]);
   expect(existsSync(ticket)).toBe(false);
   expect(has("replacement")).toBe(false);
  }
  if(fixture.competes){
   competitor=spawnFixture("competitor");
   await until(()=>has("competitor"),"another real gate can take the freed capacity");
  }
  if(fixture.publishesGeneration!==null)publish(fixture.publishesGeneration);
  if(fixture.signal!==null)expect(door.child.kill(fixture.signal as NodeJS.Signals)).toBe(true);
  if(fixture.competes){
   await until(()=>entries(root,"queue").length===1,"replacement queues behind the competing grant",7000);
   expect(grantRows(root)).toEqual([read(root,"competitor").id+".json"]);
   expect(has("replacement")).toBe(false);
   stopCompetitor();
  }
  const result=await door.done;
  expect(result).toMatchObject({code:expected.exit,signal:null});
  if(expected.round===null){
   expect(has("replacement")).toBe(false);
   expect(read(root,"refusal")).toMatchObject({code:"cancelled",key:"capacity.graph-yield-wait-cancelled",values:{signal:fixture.signal},en:expect.stringContaining("next graph generation"),de:expect.stringContaining("nächste Graph-Generation")});
  }else{
   const replacement=read(root,"replacement");
   expect(replacement).toMatchObject({ppid:first.pid,adopted:Boolean(expected.adopted),inherited:inherited!==undefined,round:expected.round,requestedAt:expected.round===0?null:1700000000000});
   expect(replacement.pid).not.toBe(first.pid);
   expect(replacement.grantPid).toBe(inherited?.pid??replacement.pid);
   expect(replacement.id===first.id).toBe(inherited!==undefined||Boolean(expected.adopted));
  }
  if(inherited!==undefined){
   expect(entries(root,"grants")).toEqual([inherited.id+".json"]);
   stopCompetitor();
  }
  if(competitor!==undefined)expect((await competitor.done).code).toBe(0);
  expect(entries(root,"grants")).toEqual([]);
  expect(entries(root,"queue")).toEqual([]);
  expect(existsSync(ticket)).toBe(false);
 }finally{
  stopCompetitor();
  for(const {child} of children)if(child.exitCode===null&&child.signalCode===null)child.kill("SIGKILL");
  await Promise.all(children.map(({done})=>done));
  rmSync(root,{recursive:true,force:true});
 }
},22000);
