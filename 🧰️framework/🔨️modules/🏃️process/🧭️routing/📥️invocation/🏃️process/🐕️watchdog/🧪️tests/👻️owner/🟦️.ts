import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import{spawn}from"node:child_process";
import{fileURLToPath}from"node:url";
import schema from"../../🧬️schema/🔣️.json";
import fixture from"./🔣️.json";
import{readScriptProcessOwnerV1,scriptProcessOwnerLostV1,startScriptProcessOwnerWatchV1,SCRIPT_PROCESS_OWNER_ENV_V1}from"../../🟦️.ts";
import{createScriptProcessEnvelope,scriptProcessEnvironment}from"../../../🟦️.ts";
const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch{return false;}},sleep=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
test("the owner rule agrees with the closed schema and an independent SQLite table",()=>{
 const database=new Database(":memory:"),validate=new Ajv({strict:true}).compile({...schema.definitions.OwnerFacts,definitions:{}});
 try{for(const row of fixture.vectors){expect(validate(row.facts),row.name).toBe(true);expect(scriptProcessOwnerLostV1(row.facts),row.name).toBe(row.lost);
  const expected=database.query("SELECT CASE WHEN :owner IS NULL THEN 0 WHEN :alive=0 THEN 1 WHEN :initial=:owner AND :current<>:owner THEN 1 ELSE 0 END AS lost").get({":owner":row.facts.owner,":alive":row.facts.ownerAlive?1:0,":initial":row.facts.initialParent,":current":row.facts.currentParent}) as{lost:number};
  expect(expected.lost===1,row.name).toBe(row.lost);}}finally{database.close();}
});
test("the owner is read from the handed over variable only, and a spawner hands over its own pid with the envelope",()=>{
 expect(readScriptProcessOwnerV1({})).toBeNull();expect(readScriptProcessOwnerV1({[SCRIPT_PROCESS_OWNER_ENV_V1]:"abc"})).toBeNull();expect(readScriptProcessOwnerV1({[SCRIPT_PROCESS_OWNER_ENV_V1]:String(process.pid)})).toBeNull();expect(readScriptProcessOwnerV1({[SCRIPT_PROCESS_OWNER_ENV_V1]:"4242"})).toBe(4242);
 const environment=scriptProcessEnvironment(createScriptProcessEnvelope({version:1,owner:"test:owner-watch",maximumElapsedMilliseconds:0},{},Date.now()),{});expect(environment[SCRIPT_PROCESS_OWNER_ENV_V1]).toBe(String(process.pid));
});
test("the watch reports a vanished owner once and never one that lives or was never handed over",async()=>{
 let living=true,lost:number[]=[];const options={alive:()=>living,parent:()=>7,checkMilliseconds:10,onLost:(owner:number)=>{lost.push(owner);}};
 const none=startScriptProcessOwnerWatchV1({...options,environment:{}}),stop=startScriptProcessOwnerWatchV1({...options,environment:{[SCRIPT_PROCESS_OWNER_ENV_V1]:"4242"}});
 try{await sleep(80);expect(lost).toEqual([]);living=false;await sleep(80);expect(lost).toEqual([4242]);await sleep(60);expect(lost).toEqual([4242]);}finally{none();stop();}
});
test("killing the owner ends its Original child, the child's own descendants and nothing is left running",async()=>{
 const middle=spawn(process.execPath,[fileURLToPath(new URL("./📜️middle.ts",import.meta.url))],{stdio:["ignore","pipe","inherit"]}),pids:Record<string,number>={};let text="";
 middle.stdout.on("data",(bytes:Buffer)=>{text+=bytes.toString();});
 try{
  for(let round=0;round<400&&!(pids.middle&&pids.child);round++){await sleep(50);for(const line of text.split("\n").filter(Boolean))Object.assign(pids,JSON.parse(line));}
  expect(pids.middle&&pids.child).toBeTruthy();
  for(let round=0;round<400&&!pids.grandchild;round++){await sleep(50);for(const line of text.split("\n").filter(Boolean))Object.assign(pids,JSON.parse(line));}
  expect(pids.grandchild).toBeTruthy();
  await sleep(300);expect(alive(pids.child!)&&alive(pids.grandchild!)).toBe(true);
  process.kill(pids.middle!,"SIGKILL");
  for(let round=0;round<300&&(alive(pids.child!)||alive(pids.grandchild!));round++)await sleep(100);
  expect(alive(pids.child!)).toBe(false);expect(alive(pids.grandchild!)).toBe(false);
 }finally{for(const pid of Object.values(pids))try{process.kill(pid,"SIGKILL");}catch{}}
},60000);
