import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import corpus from"./🔣️.json"with{type:"json"};
import schema from"../🧬️schema/🔣️.json"with{type:"json"};

test("layered SDK metadata reads retain bounded width and completed parents",async()=>{
 const{forEachNxSdkSourceLayerV1}=await import("../🟦️.ts"),database=new Database(":memory:"),validate=new Ajv({strict:true}).compile(schema.definitions.Path),paths=[...corpus.parents,...Array.from({length:corpus.leafCount},(_,index)=>"node_modules/dep/leaf-"+index)],done=new Set<string>(),events:{path:string;depth:number}[]=[];let active=0,maximum=0;
 try{database.exec("create table paths(path text,depth integer)");for(const path of paths){expect(validate(path)).toBe(true);database.query("insert into paths values(?,?)").run(path,path===""?0:path.split("/").length);}const expected=database.query("select max(min(width,?1)) width from (select count(*) width from paths group by depth)").get(corpus.parallelReads)as{width:number};
  await forEachNxSdkSourceLayerV1(paths,path=>path,async path=>{const parent=path.includes("/")?path.slice(0,path.lastIndexOf("/")):"";if(path)expect(done.has(parent)).toBe(true);active++;maximum=Math.max(maximum,active);await new Promise<void>(accept=>setImmediate(accept));events.push({path,depth:path===""?0:path.split("/").length});done.add(path);active--;});
  expect(maximum).toBe(expected.width);expect(done.size).toBe(paths.length);expect(events.map(row=>row.depth)).toEqual([...events.map(row=>row.depth)].sort((a,b)=>a-b));expect(active).toBe(0);for(const path of corpus.refused){expect(validate(path)).toBe(false);let effects=0;await expect(forEachNxSdkSourceLayerV1([path],row=>row,async()=>{effects++;})).rejects.toThrow();expect(effects).toBe(0);}console.log("[DEBUG] Layered metadata width and completed-parent order agree with independent Ajv/SQLite reductions "+JSON.stringify({paths:paths.length,maximum}));
 }finally{database.close();}
});

test("layered SDK metadata failure drains accepted reads before rejecting",async()=>{
 const{forEachNxSdkSourceLayerV1}=await import("../🟦️.ts");for(const kind of corpus.failureKinds){const events:string[]=[],marker=kind==="undefined"?undefined:Error("fixture metadata failure");let release!:()=>void,entered!:()=>void,settled=false;const held=new Promise<void>(accept=>release=accept),ready=new Promise<void>(accept=>entered=accept),work=forEachNxSdkSourceLayerV1(corpus.failurePaths,path=>path,async path=>{events.push(path);if(path===corpus.failurePaths[0]){await held;events.push("held-read-settled");}else if(path===corpus.failurePaths[1]){entered();throw marker;}});void work.then(()=>{settled=true;},()=>{settled=true;});await ready;await new Promise<void>(accept=>setImmediate(accept));expect(settled).toBe(false);expect(events).toEqual(corpus.failurePaths.slice(0,2));release();let rejected=false;try{await work;}catch(error){rejected=true;expect(error).toBe(marker);}expect(rejected).toBe(true);expect(events).toEqual([...corpus.failurePaths.slice(0,2),"held-read-settled"]);expect(settled).toBe(true);}console.log("[DEBUG] Accepted metadata reads drained before both Error and undefined failures escaped; deeper layers never began");
});
